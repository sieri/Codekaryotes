pub mod systems;

use crate::life::common_parts::ChromosomalComponent;
use crate::life::creature::Creature;
use crate::life::genome::{Chromosome, Mutating};
use crate::parameters::CodekaryoteParameters;
use arr_macro::arr;
use bevy::prelude::*;
use rand::distributions::Slice;
use std::collections::{HashMap, HashSet};
use std::fmt::{format, Write};
use std::fmt::{Display, Formatter, Result};
use std::ops::Range;
use Position::{Input, Internal, Output};

//TODO: set parameters
const LINKS: usize = 70;
const NUM_INPUT: usize = 14;
const NUM_OUTPUT: usize = 4;
const INTERNAL_NEURON: usize = 42;

const INTERNAL_PREFIX: usize = NUM_INPUT;
const OUTPUT_PREFIX: usize = INTERNAL_PREFIX + INTERNAL_NEURON;

#[derive(Debug, Clone, Copy)]
pub enum Activation {
    Linear = 0,
    BinaryStep = 1,
    Logistic = 2,
    Tanh = 3,
    Gaussian = 4,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Position {
    Input,
    Output,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Inputs {
    Constant,
    Touch,
    TouchForward,
    Angle,
    Speed,
    RotationSpeed,
    Energy,
    NumSeen,
    NumSeenCreature,
    NumSeenPlant,
    ClosestCreatureAngle,
    ClosestCreatureDist,
    ClosestCreatureSizeRatio,
    ClosestPlantAngle,
    ClosestPlantDist,
    ClosestPlantSizeRatio,
}

impl From<usize> for Inputs {
    fn from(i: usize) -> Self {
        match i {
            00 => Inputs::Constant,
            01 => Inputs::Touch,
            02 => Inputs::TouchForward,
            03 => Inputs::Angle,
            04 => Inputs::Speed,
            05 => Inputs::RotationSpeed,
            06 => Inputs::Energy,
            07 => Inputs::NumSeen,
            08 => Inputs::NumSeenCreature,
            09 => Inputs::NumSeenPlant,
            10 => Inputs::ClosestCreatureAngle,
            11 => Inputs::ClosestCreatureDist,
            12 => Inputs::ClosestPlantAngle,
            13 => Inputs::ClosestPlantDist,
            _ => {
                panic!("Unknown input")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Outputs {
    Multiplier,
    Forward,
    Backward,
    TurnLeft,
    TurnRight,
}

impl From<usize> for Outputs {
    fn from(i: usize) -> Self {
        match i {
            0 => Outputs::Multiplier,
            1 => Outputs::Forward,
            2 => Outputs::Backward,
            3 => Outputs::TurnLeft,
            4 => Outputs::TurnRight,
            _ => {
                panic!("Unknown input")
            }
        }
    }
}

#[derive(Clone)]
pub struct Neuron {
    pub id: usize,
    pub out_val: f32,
    pub in_val: f32,
    pub input: Option<Inputs>,
    pub output: Option<Outputs>,
    pub act: Activation,
}

struct NeuronDefinition {
    pub id_initial: usize,
    pub pos: Position,
    pub act: Activation,
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub struct LinkDefinition {
    pub weight: f32,
    pub input: usize,
    pub output: usize,
    pub input_type: Position,
    pub output_type: Position,
}

#[derive(Copy, Clone)]
struct Link {
    input: usize,
    output: usize,
    weight: f32,
}

#[derive(Component, Clone)]
pub struct Brain {
    links: Vec<Link>,
    pub neurons: Vec<Neuron>,
    neurons_input_count: usize,
    neurons_internal_count: usize,
    neurons_output_count: usize,
    pub energy_rate: f32,
    //For Module
    chromosome: Chromosome,
}

impl Neuron {
    fn new(id: usize, act: Activation) -> Neuron {
        Neuron {
            id,
            out_val: 0.0,
            in_val: 0.0,
            input: None,
            output: None,
            act,
        }
    }

    fn new_input(id: usize, act: Activation, input: Inputs) -> Neuron {
        Neuron {
            id,
            out_val: 0.0,
            in_val: 0.0,
            input: Some(input),
            output: None,
            act,
        }
    }

    fn new_output(id: usize, act: Activation, output: Outputs) -> Neuron {
        Neuron {
            id,
            out_val: 0.0,
            in_val: 0.0,
            input: None,
            output: Some(output),
            act,
        }
    }

    fn write_in(&mut self, val: f32) {
        self.in_val = val;
    }
}

impl Display for Neuron {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let prefix = {
            match (self.input.is_none(), self.output.is_none()) {
                (true, false) => "❤",
                (false, true) => "♦",
                (_, _) => "✅",
            }
        };
        write!(
            f,
            "{}Neuron: [id{}, in:{}, out:{}, act:{}]",
            prefix, self.id, self.in_val, self.out_val, self.act,
        )
    }
}

impl Activation {
    fn from_chromosome(i: u32) -> Activation {
        match i {
            0 => Activation::Linear,
            1 => Activation::BinaryStep,
            2 => Activation::Gaussian,
            3 => Activation::Logistic,
            4 => Activation::Tanh,
            _ => panic!("Wrong activation"),
        }
    }
}

impl Display for Activation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Link {
    fn new(input: usize, output: usize, weight: f32) -> Self {
        Link {
            input,
            output,
            weight,
        }
    }
}

fn test_bit(v: u32, index: u32) -> bool {
    let mask: u32 = 1 << index;
    (v & mask) > 0
}

fn bit_range(v: u32, index: u32, length: u32) -> u32 {
    let mask: u32 = (2u32.pow(length) - 1) << index;
    (v & mask) >> index
}

impl LinkDefinition {
    fn new(c: u32) -> Self {
        let input_type = match test_bit(c, 31) {
            true => Input,
            false => Internal,
        };

        let output_type = match test_bit(c, 23) {
            true => Output,
            false => Internal,
        };

        let input = match input_type {
            Input => bit_range(c, 24, 7) % NUM_INPUT as u32,
            Output => panic!("This shouldn't happen"),
            Internal => INTERNAL_PREFIX as u32 + (bit_range(c, 24, 7) % INTERNAL_NEURON as u32),
        } as usize;

        let output = match output_type {
            Input => panic!("This shouldn't happen"),
            Output => OUTPUT_PREFIX as u32 + (bit_range(c, 16, 7) % NUM_OUTPUT as u32),
            Internal => INTERNAL_PREFIX as u32 + (bit_range(c, 16, 7) % INTERNAL_NEURON as u32),
        } as usize;

        let weight = to_signed(bit_range(c, 0, 16), 16) / 8191.75;

        LinkDefinition {
            weight,
            output,
            input_type,
            input,
            output_type,
        }
    }
}

fn to_signed(number: u32, length_of_range: u32) -> f32 {
    if test_bit(number, length_of_range - 1) {
        return -(bit_range(number, 0, length_of_range - 1) as f32);
    } else {
        return number as f32;
    }
}

impl Brain {
    pub fn out_range(&self) -> Range<usize> {
        (self.neurons_input_count + self.neurons_internal_count)
            ..(self.neurons_input_count + self.neurons_internal_count + self.neurons_output_count)
    }

    pub fn in_range(&self) -> Range<usize> {
        0..self.neurons_input_count
    }
    pub fn offset(&self) -> usize {
        NUM_INPUT + INTERNAL_NEURON
    }
}

impl NeuronDefinition {
    fn new(id: usize, pos: Position, activation: Activation) -> NeuronDefinition {
        NeuronDefinition {
            id_initial: id,
            pos,
            act: activation,
        }
    }
}

struct LinkHelper {
    links: Vec<LinkDefinition>,
    checked: Vec<(LinkDefinition, bool)>,
    finals: Vec<LinkDefinition>,
}

fn connected_to_input(l: &LinkDefinition, help: &mut LinkHelper) -> bool {
    if l.input_type == Input {
        help.finals.push(*l);
        return true;
    } else {
        let links = {
            let l: Vec<&LinkDefinition> =
                help.links.iter().filter(|x| x.output == l.input).collect();
            let mut li: Vec<LinkDefinition> = vec![];
            for i in l {
                let n = i.clone();
                li.push(n)
            }
            li
        };

        for lo in links {
            let exist = help.checked.iter_mut().find(|x| (x.0) == lo);
            match exist {
                None => {
                    let i = help.checked.len();
                    help.checked.push((lo, false));
                    let connected = connected_to_input(&lo, help);
                    if connected {
                        help.finals.push(lo);
                        help.checked[i] = (lo, true)
                    }
                    return connected;
                }
                Some(x) => return x.1,
            }
        }
        return false;
    }
}

impl ChromosomalComponent for Brain {
    fn new(chromosome: Chromosome, param: CodekaryoteParameters) -> Self {
        let mut brain = Brain {
            links: vec![],
            neurons: vec![],
            neurons_input_count: 0,
            neurons_internal_count: 0,
            neurons_output_count: 0,
            energy_rate: 0.0,
            chromosome: chromosome.to_vec(),
        };
        //initialize brain
        //Get definitions
        let mut index = 0;

        let inputs: [NeuronDefinition; NUM_INPUT] = arr![NeuronDefinition::new({index+=1; index-1}, Input, Activation::from_chromosome(chromosome[index-1] % 5)); 14];
        let internals: [NeuronDefinition; INTERNAL_NEURON] = arr![NeuronDefinition::new({index+=1; INTERNAL_PREFIX+index-1}, Position::Internal, Activation::from_chromosome(chromosome[index-1] % 5)); 42];
        let outputs: [NeuronDefinition; NUM_OUTPUT] = arr![NeuronDefinition::new({index+=1; OUTPUT_PREFIX+index-1}, Position::Output, Activation::from_chromosome(chromosome[index-1] % 5)); 4];

        let links: [LinkDefinition; LINKS] =
            arr![LinkDefinition::new(chromosome[{index+=1; index-1}]); 70];

        //clean links
        let list_of_output_link: Vec<&LinkDefinition> =
            links.iter().filter(|x| x.output_type == Output).collect();

        let mut help = LinkHelper {
            links: links.to_vec(),
            checked: vec![],
            finals: vec![],
        };

        //Check all the neurons for connections between output and inputs
        for l in list_of_output_link {
            if connected_to_input(l, &mut help) {
                help.finals.push(*l)
            }
        }

        //create maps of each active inputs, to count them only once
        let mut input_map = HashMap::new();
        let mut internal_map = HashMap::new();
        let mut output_map = HashMap::new();
        for link_def in help.finals.iter() {
            if link_def.output_type == Output {
                output_map.entry(link_def.output).or_insert(link_def.output);
            } else {
                internal_map
                    .entry(link_def.output)
                    .or_insert(link_def.output);
            }
            if link_def.input_type == Input {
                input_map.entry(link_def.input).or_insert(link_def.input);
            } else {
                internal_map.entry(link_def.input).or_insert(link_def.input);
            }
        }

        //Put the neurons in the brain, and give them a continuous id
        let mut id_counter = 0;
        let mut inputs_ids = HashMap::new();
        for (i, v) in input_map.iter().enumerate() {
            brain.neurons.push(Neuron::new_input(
                id_counter,
                inputs[*v.1].act,
                (*v.1).into(),
            ));
            inputs_ids.insert(*v.1, id_counter);
            id_counter += 1;
        }
        let mut internals_ids = HashMap::new();
        for (i, v) in internal_map.iter().enumerate() {
            brain.neurons.push(Neuron::new(
                id_counter,
                internals[*v.1 - INTERNAL_PREFIX].act,
            ));
            internals_ids.insert(*v.1, id_counter);
            id_counter += 1;
        }
        let mut outputs_ids = HashMap::new();
        for (i, v) in output_map.iter().enumerate() {
            brain.neurons.push(Neuron::new_output(
                id_counter,
                outputs[*v.1 - OUTPUT_PREFIX].act,
                (*v.1 - OUTPUT_PREFIX).into(),
            ));
            outputs_ids.insert(*v.1, id_counter);
            id_counter += 1;
        }
        brain.neurons_input_count = inputs_ids.len();
        brain.neurons_internal_count = internals_ids.len();
        brain.neurons_output_count = outputs_ids.len();
        //Links the neurons
        for l in help.finals.iter() {
            let i = *match l.input_type {
                Input => inputs_ids.entry(l.input),
                Output => panic!("This shouldn't happen"),
                Internal => internals_ids.entry(l.input),
            }
            .or_default();
            let o = *match l.output_type {
                Input => panic!("This shouldn't happen"),
                Output => outputs_ids.entry(l.output),
                Internal => internals_ids.entry(l.output),
            }
            .or_default();
            brain.links.push(Link::new(i, o, l.weight))
        }

        brain.energy_rate =
            0.00000001 * brain.links.len().pow(2) as f32 * (brain.neurons.len() as f32).powi(3);

        brain
    }
    fn get_mutated(&self) -> Chromosome {
        self.chromosome.mutate(5)
    }
}

fn graph_def(defs: &[LinkDefinition]) {
    let mut b = String::new();
    b += "===============================================\ndigraph G {";
    let mut i = HashSet::new();
    let mut o = HashSet::new();
    for def in defs {
        b += &*format!("    {}->{};", def.input, def.output);
        if def.input_type == Input {
            i.insert(def.input);
        }
        if def.output_type == Output {
            o.insert(def.output);
        }
    }

    b += "\tsubgraph cluster_0{";
    for v in i {
        b += &*format!("\t{};", v);
    }

    b += "\tstyle=filled;\tcolor=lightgrey;label=\"inputs\"}";
    b += "\tsubgraph cluster_1{";
    for v in o {
        b += &*format!("\t{};", v);
    }
    b += "\tstyle=filled;\tcolor=lightgrey;label=\"outputs\"}";
    b += "}\n===============================================\n";

    println!("{}", b);
}
