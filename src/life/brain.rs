extern crate pyo3;
use crate::codekaryotes::Creature;
use crate::life::common_parts::Module;
use crate::life::creature_parts::CreatureModule;
use crate::life::genome::{Chromonsone, CreatureGenome};
use pyo3::prelude::*;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy)]
#[pyclass(module = "codekaryotes.codekaryotes")]
pub enum Activation {
    Linear = 0,
    BinaryStep = 1,
    Logistic = 2,
    Tanh = 3,
    Gaussian = 4,
}

#[derive(Debug, Clone, Copy)]
#[pyclass(module = "codekaryotes.codekaryotes")]
pub enum Position {
    Input,
    Output,
    Internal,
}

#[derive(Debug)]
#[pyclass(module = "codekaryotes.codekaryotes")]
#[derive(FromPyObject)]
pub struct NeuronDefinition {
    #[pyo3(get, set)]
    pub activation: Activation,
    #[pyo3(get, set)]
    pub position: Position,
    #[pyo3(get, set)]
    pub id: usize,
    #[pyo3(get, set)]
    pub object: PyObject,
}

#[derive(Debug)]
pub struct Neuron {
    pub id: usize,
    pub out_val: f64,
    pub in_val: f64,
    pub object: Option<PyObject>,
    pub act: Activation,
}

#[derive(Debug)]
#[pyclass(module = "codekaryotes.codekaryotes")]
#[derive(FromPyObject)]
pub struct LinkDefinition {
    #[pyo3(get, set)]
    pub weight: f64,
    #[pyo3(get, set)]
    pub input: usize,
    #[pyo3(get, set)]
    pub output: usize,
    #[pyo3(get, set)]
    pub input_type: Position,
    #[pyo3(get, set)]
    pub output_type: Position,
}

#[derive(Debug)]
struct Link {
    input: usize,
    output: usize,
    weight: f64,
}

#[derive(Debug)]
#[pyclass(module = "codekaryotes.codekaryotes")]
pub struct Brain {
    inputs: Vec<NeuronDefinition>,
    internals: Vec<NeuronDefinition>,
    outputs: Vec<NeuronDefinition>,
    links_def: Vec<LinkDefinition>,

    links: Vec<Link>,

    neurons: Vec<Neuron>,

    neurons_id_input: Vec<usize>,
    neurons_id_output: Vec<usize>,
}

#[pymethods]
impl NeuronDefinition {
    #[new]
    fn __new__(activation: Activation, pos: Position, id: usize, linked_obj: PyObject) -> Self {
        NeuronDefinition {
            activation,
            id,
            position: pos,
            object: linked_obj,
        }
    }
}

impl Neuron {
    fn new(id: usize, act: Activation) -> Neuron {
        Neuron {
            id,
            out_val: 0.0,
            in_val: 0.0,
            object: None,
            act,
        }
    }

    fn new_obj(id: usize, act: Activation, object: PyObject) -> Neuron {
        Neuron {
            id,
            out_val: 0.0,
            in_val: 0.0,
            act,
            object: Some(object),
        }
    }

    fn update_neural_input(&mut self, py: Python<'_>) {
        self.in_val = (self.object)
            .as_ref()
            .unwrap()
            .getattr(py, "input")
            .unwrap()
            .extract(py)
            .unwrap();
    }

    fn update_neural_output(&mut self, py: Python<'_>) {
        let res = self
            .object
            .as_ref()
            .unwrap()
            .call_method(py, "output", (self.out_val,), None);

        if res.is_err() {
            println!("Error on output update {:?}", res);
        }
    }

    fn write_in(&mut self, val: f64) {
        self.in_val = val;
    }
}

impl Display for Neuron {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let prefix = {
            match self.object.is_none() {
                true => "❤",
                false => "✅",
            }
        };
        write!(
            f,
            "{}Neuron: [id{}, in:{}, out:{}, act:{}]",
            prefix, self.id, self.in_val, self.out_val, self.act,
        )
    }
}

impl Display for Activation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Link {
    fn new(input: usize, output: usize, weight: f64) -> Self {
        Link {
            input,
            output,
            weight,
        }
    }
}

#[pymethods]
impl LinkDefinition {
    #[new]
    fn __new__(input: usize, output: usize, weight: f64, input_type: Position) -> Self {
        LinkDefinition {
            weight,
            output,
            input_type,
            input,
            output_type: Position::Output,
        }
    }
}

#[pymethods]
impl Brain {
    #[staticmethod]
    pub fn new() -> Brain {
        Brain {
            inputs: vec![],
            internals: vec![],
            outputs: vec![],
            links_def: vec![],
            links: vec![],

            neurons: vec![],

            neurons_id_input: vec![],
            neurons_id_output: vec![],
        }
    }

    pub fn add_input(&mut self, a: NeuronDefinition) -> PyResult<()> {
        self.inputs.push(a);
        Ok(())
    }

    pub fn add_internal(&mut self, a: NeuronDefinition) -> PyResult<()> {
        self.internals.push(a);
        Ok(())
    }

    pub fn add_output(&mut self, a: NeuronDefinition) -> PyResult<()> {
        self.outputs.push(a);
        Ok(())
    }

    pub fn add_link(&mut self, l: LinkDefinition) -> PyResult<()> {
        self.links_def.push(l);
        Ok(())
    }

    pub fn initiate(&mut self) -> PyResult<()> {
        let gil = Python::acquire_gil();
        let &py = &gil.python();

        for input in self.inputs.iter() {
            self.neurons_id_input.push(input.id);
            self.neurons.push(Neuron::new_obj(
                input.id,
                input.activation,
                input.object.clone_ref(py),
            ))
        }
        let internal_prefix: usize = self.neurons.len();

        for internal in self.internals.iter() {
            self.neurons.push(Neuron::new(
                internal_prefix + internal.id,
                internal.activation,
            ))
        }

        let output_prefix: usize = self.neurons.len();

        for output in self.outputs.iter() {
            self.neurons_id_output.push(output_prefix + output.id);
            self.neurons.push(Neuron::new_obj(
                output_prefix + output.id,
                output.activation,
                output.object.clone_ref(py),
            ))
        }

        // create links
        //println!("Hello!{:?}", self.links_def);
        for l in self.links_def.iter() {
            let input = match l.input_type {
                Position::Input => l.input,
                Position::Internal => internal_prefix + l.input,
                _ => 0,
            };
            let output = match l.output_type {
                Position::Output => output_prefix + l.output,
                Position::Internal => internal_prefix + l.output,
                _ => 0,
            };

            let nl = Link::new(input, output, l.weight);
            self.links.push(nl);
        }

        Ok(())
    }

    pub fn update(&mut self) -> PyResult<()> {
        let gil = Python::acquire_gil();
        let &py = &gil.python();

        //feed links
        //println!("----------Feed Link----------");
        self.links.iter().for_each(|x| -> () {
            let new_val = self.neurons[x.input].out_val * x.weight;
            self.neurons[x.output].write_in(new_val);
            //println!("new_val={}, written in {}", new_val, self.neurons[x.output]);
        });

        // acquire inputs
        for inp in self.neurons_id_input.iter() {
            self.neurons[*inp].update_neural_input(py);
        }

        //activate neurons
        //println!("----------Activation----------");
        self.neurons.iter_mut().for_each(|n| -> () {
            match n.act {
                Activation::Linear => n.out_val = n.in_val,
                Activation::BinaryStep => {
                    if n.in_val > 0.0 {
                        n.out_val = 1.0;
                    } else {
                        n.out_val = 0.0;
                    }
                }
                Activation::Logistic => n.out_val = 1.0 / (1.0 + (-n.in_val).exp()),
                Activation::Tanh => n.out_val = n.in_val.tanh(),
                Activation::Gaussian => n.out_val = (-(n.in_val.powi(2))).exp(),
            };
            //println!("{}", n);
        });

        Ok(())
    }

    pub fn output(&mut self) -> PyResult<()> {
        let gil = Python::acquire_gil();
        let &py = &gil.python();
        for x in self.neurons_id_output.iter() {
            self.neurons[*x].update_neural_output(py);
        }

        Ok(())
    }
}

impl Module<Creature, CreatureGenome> for Brain {
    fn by_box(self: Box<Self>) {
        todo!()
    }

    fn update(&self, organism: Creature) {
        todo!()
    }

    fn reset(&self, organism: Creature) {
        todo!()
    }

    fn evolve(&self) -> Chromonsone {
        todo!()
    }
}

impl CreatureModule for Brain {}
