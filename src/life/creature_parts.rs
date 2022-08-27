use crate::codekaryotes::{Codekaryote, Creature, Plant, Pos, Seen};
use crate::life::common_parts::{Ancestry, Color, Module};
use crate::life::genome::{Chromosome, CreatureGenome, Mutating};
use crate::Brain;
use pyo3::number::or;
use pyo3::types::IntoPyDict;
use pyo3::{PyObject, Python};
use std::borrow::BorrowMut;

pub trait CreatureModule: Module<Creature, CreatureGenome> {}

pub trait ActiveModule {
    fn get_energy_rate(&self) -> f64;
}

#[derive(Debug, Clone)]
pub struct CreatureBody {
    //For Module
    genome: Chromosome,
    mutation_rate: usize,
    //For active
    energy_rate: f64,
    //Unique
    size: f64,
    mass: f64,
    circle: PyObject,
}

impl CreatureBody {
    pub(crate) fn push(&self, force: f64) {
        todo!()
    }
    pub(crate) fn rotate(&self, torque: f64) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Movement {
    //For Module
    genome: Chromosome,
    mutation_rate: usize,
    //For active
    energy_rate: f64,
    //Unique
    energy_rate_base: f64,
    forward: f64,
    torque: f64,
    multiplier_base: f64,
    multiplier_signal: f64,
    travelled: f64,
    last_pos: Pos,
}
#[derive(Debug, Clone)]
pub struct Touch {
    //For Module
    genome: Chromosome,
    mutation_rate: usize,
    //Unique
    touch: usize,
    touch_forward: usize,
}

pub struct Eyes {
    //For Module
    genome: Chromosome,
    mutation_rate: usize,
    //For active
    energy_rate: f64,
    //Unique
    fov: u32,
    range: u32,
    shape: PyObject,
    seen_creatures: Vec<Seen>,
    seen_plants: Vec<Seen>,
}
#[derive(Debug, Clone)]
pub struct Eating {
    //For Module
    genome: Chromosome,
    mutation_rate: usize,
    //Unique
    ticks: usize,
    can_eat: bool,
}
#[derive(Debug, Clone)]
pub struct Reproducer {
    //For Module
    genome: Chromosome,
    mutation_rate: usize,
}
#[derive(Debug, Clone)]
pub struct EnergyStorage {
    //For Module
    genome: Chromosome,
    mutation_rate: usize,
    //For active
    energy_max: f64,
    //Unique
    pub(crate) energy: f64,
}

impl Module<Creature, CreatureGenome> for CreatureBody {
    fn new(chromosome: Chromosome) -> CreatureBody {
        //TODO: Set Params
        const FACTOR: f64 = u32::MAX as f64 / ((1.2 - 0.8) * 10000.0);
        const BODY_MASS_UNIT: f64 = 1f64;
        let size: f64 = (((chromosome[0] as f64) / FACTOR) / 10000.0);
        let mass: f64 = size.powi(2) * BODY_MASS_UNIT;

        //get circle
        let gil = Python::acquire_gil();
        let &py = &gil.python();
        let locals = [("pm", py.import("pymunk").unwrap())].into_py_dict(py);
        println!("{:?}", locals);
        let code = format!(
            "pm.Body({}, pm.moment_for_circle({}, 0, {}, (0, 0)))",
            mass, mass, size
        );

        let circle = py.eval(code.as_str(), None, Some(&locals));
        println!("Circle, {:?}", circle);
        CreatureBody {
            genome: chromosome.to_vec(),
            mutation_rate: 1,
            energy_rate: 0.0,
            size: size,
            mass: mass,
            circle: PyObject::from(circle.unwrap()),
        }
    }

    fn update(organism: &mut Creature) {
        let s = organism.body();
    }

    fn reset(organism: &mut Creature) {}

    fn evolve(&self) -> Chromosome {
        self.genome.mutate(self.mutation_rate)
    }
}

impl ActiveModule for CreatureBody {
    fn get_energy_rate(&self) -> f64 {
        self.energy_rate
    }
}

impl CreatureModule for CreatureBody {}

impl CreatureBody {
    pub fn get_position(&self) -> Pos {
        todo!()
    }
}

impl Module<Creature, CreatureGenome> for Color {
    fn new(chromosome: Chromosome) -> Self {
        Color {
            genome: chromosome.to_vec(),
            mutation_rate: 0,
            r: chromosome[0] as u8,
            g: chromosome[1] as u8,
            b: chromosome[2] as u8,
        }
    }

    fn update(organism: &mut Creature) {}

    fn reset(organism: &mut Creature) {}

    fn evolve(&self) -> Chromosome {
        self.genome.to_vec()
    }
}

impl Module<Creature, CreatureGenome> for Ancestry {
    fn new(chromosome: Chromosome) -> Self {
        Ancestry {
            genome: chromosome.to_vec(),
            mutation_rate: 0,
            generation: chromosome[0],
            age: 0.0,
        }
    }

    fn update(organism: &mut Creature) {
        let s = organism.ancestry();
        s.age += 1f64;
    }

    fn reset(organism: &mut Creature) {}

    fn evolve(&self) -> Chromosome {
        vec![self.generation, 0]
    }
}

impl CreatureModule for Ancestry {}

impl CreatureModule for Color {}

impl Module<Creature, CreatureGenome> for Movement {
    fn new(chromosome: Chromosome) -> Self {
        const ENERGY_MOVEMENT_RATE: f64 = 0.0005;

        Movement {
            genome: chromosome.to_vec(),
            mutation_rate: 2,
            energy_rate: 0.0,
            energy_rate_base: ENERGY_MOVEMENT_RATE,
            forward: 0.0,
            torque: 0.0,
            multiplier_base: 0.0,
            multiplier_signal: 1.0,
            travelled: 0.0,
            last_pos: Pos { x: 0.0, y: 0.0 },
        }
    }

    fn update(organism: &mut Creature) {
        let current_post = organism.get_position();
        let s = organism.movement();
        s.travelled += s.last_pos.dist(current_post);
        s.last_pos = current_post;
        let actual_forward = s.forward * s.multiplier_base * s.multiplier_signal;
        let actual_torque = s.torque * s.multiplier_base * s.multiplier_signal;
        s.energy_rate = s.energy_rate_base * (actual_forward.abs() + actual_torque.abs());
        let body = organism.body();
        body.push(actual_forward);
        body.rotate(actual_torque);
    }

    fn reset(organism: &mut Creature) {
        let s = organism.movement();
        s.forward = 0f64;
        s.torque = 0f64;
        s.multiplier_signal = 0f64;
    }

    fn evolve(&self) -> Chromosome {
        self.genome.mutate(self.mutation_rate)
    }
}

impl CreatureModule for Movement {}

impl ActiveModule for Movement {
    fn get_energy_rate(&self) -> f64 {
        self.energy_rate
    }
}

impl Module<Creature, CreatureGenome> for Touch {
    fn new(chromosome: Chromosome) -> Self {
        Touch {
            genome: chromosome.to_vec(),
            mutation_rate: 0,
            touch: 0,
            touch_forward: 0,
        }
    }

    fn update(organism: &mut Creature) {}

    fn reset(organism: &mut Creature) {
        let s = organism.touch();
        s.touch = 0;
        s.touch_forward = 0;
    }

    fn evolve(&self) -> Chromosome {
        todo!()
    }
}

impl CreatureModule for Touch {}

impl Module<Creature, CreatureGenome> for Eyes {
    fn new(chromosome: Chromosome) -> Self {
        //TODO: Set parameters
        const EYE_RANGE_LIMIT: u32 = 75;
        const ENERGY_EYES_RATE: f64 = 0.005;

        let fov = (chromosome[0] % 360) + 1;
        let range = chromosome[1] % EYE_RANGE_LIMIT;
        let energy_rate: f64 = ((fov as f64) / 180.0 * (range as f64)) * ENERGY_EYES_RATE;

        //get shape
        let gil = Python::acquire_gil();
        let &py = &gil.python();
        let locals = [("eyes", py.import("sim.life.body.eyes").unwrap())].into_py_dict(py);
        let code = format!("eyes.method_name({},{})", fov, range);

        let shape = py.eval(code.as_str(), None, Some(&locals));
        println!("Shape, {:?}", shape);
        Eyes {
            genome: chromosome.to_vec(),
            mutation_rate: 1,
            energy_rate,
            fov,
            range,
            shape: PyObject::from(shape.unwrap()),
            seen_creatures: vec![],
            seen_plants: vec![],
        }
    }

    fn update(organism: &mut Creature) {
        let s = organism.eyes();
    }

    fn reset(organism: &mut Creature) {
        let s = organism.eyes();
        s.seen_creatures.clear();
        s.seen_plants.clear();
    }

    fn evolve(&self) -> Chromosome {
        self.genome.mutate(self.mutation_rate)
    }
}

impl CreatureModule for Eyes {}

impl ActiveModule for Eyes {
    fn get_energy_rate(&self) -> f64 {
        return self.energy_rate;
    }
}

impl Module<Creature, CreatureGenome> for Eating {
    fn new(chromosome: Chromosome) -> Self {
        Eating {
            genome: chromosome.to_vec(),
            mutation_rate: 0,
            ticks: 0,
            can_eat: true,
        }
    }

    fn update(organism: &mut Creature) {
        let s = organism.eating();
        if !s.can_eat {
            s.ticks += 1;
            if s.ticks >= 50
            //TODO:introduce parameter
            {
                s.can_eat = true;
                s.ticks = 0;
            }
        }
    }

    fn reset(organism: &mut Creature) {}

    fn evolve(&self) -> Chromosome {
        vec![]
    }
}

impl CreatureModule for Eating {}

impl Module<Creature, CreatureGenome> for Reproducer {
    fn new(chromosome: Chromosome) -> Self {
        Reproducer {
            genome: chromosome.to_vec(),
            mutation_rate: 0,
        }
    }

    fn update(organism: &mut Creature) {
        let s = organism.reproducer();
        let energy_storage = organism.energy_storage();
        if energy_storage.get_level() > 0.8
        //TODO: Make parameters
        {
            organism.reproduce(organism.get_position())
        }
    }

    fn reset(organism: &mut Creature) {}

    fn evolve(&self) -> Chromosome {
        vec![]
    }
}

impl CreatureModule for Reproducer {}

impl Module<Creature, CreatureGenome> for EnergyStorage {
    fn new(chromosome: Chromosome) -> Self {
        const ENERGY_STORAGE_MAX: f64 = 2048.0;
        const FACTOR: f64 = u32::MAX as f64 / ENERGY_STORAGE_MAX;
        let energy_max = chromosome[0] as f64 / FACTOR;
        let energy = energy_max / 2.0;
        EnergyStorage {
            genome: chromosome.to_vec(),
            mutation_rate: 1,
            energy_max,
            energy,
        }
    }

    fn update(organism: &mut Creature) {
        let s = organism.energy_storage();
        s.energy -= s.get_energy_rate();
    }

    fn reset(organism: &mut Creature) {}

    fn evolve(&self) -> Chromosome {
        self.genome.mutate(self.mutation_rate)
    }
}

impl CreatureModule for EnergyStorage {}

impl ActiveModule for EnergyStorage {
    fn get_energy_rate(&self) -> f64 {
        self.energy_max
    }
}

impl EnergyStorage {
    fn get_level(&self) -> f64 {
        self.energy / self.energy_max
    }

    pub fn consume(&mut self, energy: f64) -> bool {
        self.energy -= energy;

        energy > 0f64
    }
}
