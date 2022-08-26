use crate::codekaryotes::{Codekaryote, Creature, Plant, Pos, Seen};
use crate::life::common_parts::{Ancestry, Color, Module};
use crate::life::genome::{Chromosome, CreatureGenome};
use crate::Brain;
use pyo3::number::or;
use pyo3::PyObject;
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
    mass: usize,
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
    //For active
    energy_rate: f64,
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
    fov: usize,
    range: usize,
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
    fn update(organism: &mut Creature) {
        let s = organism.body();
    }

    fn reset(organism: &mut Creature) {}

    fn evolve(&self) -> Chromosome {
        todo!()
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
    fn update(organism: &mut Creature) {}

    fn reset(organism: &mut Creature) {}

    fn evolve(&self) -> Chromosome {
        todo!()
    }
}

impl Module<Creature, CreatureGenome> for Ancestry {
    fn update(organism: &mut Creature) {
        let s = organism.ancestry();
        s.age += 1f64;
    }

    fn reset(organism: &mut Creature) {}

    fn evolve(&self) -> Chromosome {
        todo!()
    }
}

impl CreatureModule for Ancestry {}

impl CreatureModule for Color {}

impl Module<Creature, CreatureGenome> for Movement {
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
        todo!()
    }
}

impl CreatureModule for Movement {}

impl ActiveModule for Movement {
    fn get_energy_rate(&self) -> f64 {
        self.energy_rate
    }
}

impl Module<Creature, CreatureGenome> for Touch {
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

impl ActiveModule for Touch {
    fn get_energy_rate(&self) -> f64 {
        self.energy_rate
    }
}

impl Module<Creature, CreatureGenome> for Eyes {
    fn update(organism: &mut Creature) {
        let s = organism.eyes();
    }

    fn reset(organism: &mut Creature) {
        let s = organism.eyes();
        s.seen_creatures.clear();
        s.seen_plants.clear();
    }

    fn evolve(&self) -> Chromosome {
        todo!()
    }
}

impl CreatureModule for Eyes {}

impl ActiveModule for Eyes {
    fn get_energy_rate(&self) -> f64 {
        return self.energy_rate;
    }
}

impl Module<Creature, CreatureGenome> for Eating {
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
        todo!()
    }
}

impl CreatureModule for Eating {}

impl Module<Creature, CreatureGenome> for Reproducer {
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
        todo!()
    }
}

impl CreatureModule for Reproducer {}

impl Module<Creature, CreatureGenome> for EnergyStorage {
    fn update(organism: &mut Creature) {
        let s = organism.energy_storage();
        s.energy -= s.get_energy_rate();
    }

    fn reset(organism: &mut Creature) {}

    fn evolve(&self) -> Chromosome {
        todo!()
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
    
    pub fn consume(&mut self, energy: f64) -> bool
    {
        self.energy -= energy;
        
        energy > 0f64
    }
}
