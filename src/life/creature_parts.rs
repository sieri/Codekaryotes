use crate::codekaryotes::{Codekaryote, Creature, Plant, Pos};
use crate::life::common_parts::Module;
use crate::life::genome::{Chromonsone, CreatureGenome};
use pyo3::ffi::PyObject;
use std::borrow::BorrowMut;

pub trait CreatureModule: Module<Creature, CreatureGenome> {}

pub trait ActiveModule {
    fn consume_energy(&self, organism: Creature) {
        let mut es = organism.energy_storage();
        es.energy += self.get_energy_rate()
    }

    fn get_energy_rate(&self) -> f64;
}

pub struct CreatureBody {
    //For Module
    genome: Chromonsone,
    mutation_rate: usize,
    //For active
    energy_rate: f64,
    //Unique
    mass: usize,
    circle: PyObject,
}

pub struct Movement {
    //For Module
    genome: Chromonsone,
    mutation_rate: usize,
    //For active
    energy_rate: f64,
    //Unique
    forward: f64,
    torque: f64,
    multiplier_signal: f64,
    travelled: f64,
    last_pos: Pos,
}

pub struct Touch {
    //For Module
    genome: Chromonsone,
    mutation_rate: usize,
    //For active
    energy_rate: f64,
    //Unique
    touch: usize,
    touch_forward: usize,
}

pub struct Eyes {
    //For Module
    genome: Chromonsone,
    mutation_rate: usize,
    //For active
    energy_rate: f64,
    //Unique
    fov: usize,
    range: usize,
    shape: PyObject,
    seen_creatures: Box<Creature>,
    seen_plants: Box<Plant>,
}

pub struct Eating {
    //For Module
    genome: Chromonsone,
    mutation_rate: usize,
    //Unique
    ticks: usize,
    can_eat: bool,
}

pub struct Reproducer {
    //For Module
    genome: Chromonsone,
    mutation_rate: usize,
}

pub struct EnergyStorage {
    //For Module
    genome: Chromonsone,
    mutation_rate: usize,
    //For active
    energy_rate: f64,
    //Unique
    energy: f64,
}

impl Module<Creature, CreatureGenome> for CreatureBody {
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

impl CreatureModule for CreatureBody {}

impl ActiveModule for CreatureBody {
    fn get_energy_rate(&self) -> f64 {
        self.energy_rate
    }
}

impl Module<Creature, CreatureGenome> for Movement {
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

impl CreatureModule for Movement {}

impl ActiveModule for Movement {
    fn get_energy_rate(&self) -> f64 {
        self.energy_rate
    }
}

impl Module<Creature, CreatureGenome> for Touch {
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

impl CreatureModule for Touch {}

impl ActiveModule for Touch {
    fn get_energy_rate(&self) -> f64 {
        self.energy_rate
    }
}

impl Module<Creature, CreatureGenome> for Eyes {
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

impl CreatureModule for Eyes {}

impl ActiveModule for Eyes {
    fn get_energy_rate(&self) -> f64 {
        return self.energy_rate;
    }
}

impl Module<Creature, CreatureGenome> for Eating {
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

impl CreatureModule for Eating {}

impl Module<Creature, CreatureGenome> for Reproducer {
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

impl CreatureModule for Reproducer {}

impl Module<Creature, CreatureGenome> for EnergyStorage {
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

impl CreatureModule for EnergyStorage {}

impl ActiveModule for EnergyStorage {
    fn get_energy_rate(&self) -> f64 {
        self.energy_rate
    }
}
