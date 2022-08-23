use crate::life;
use crate::life::creature_parts::EnergyStorage;
use crate::life::genome::{CreatureGenome, PlantGenome};
use life::common_parts::Module;
use life::genome;

pub struct Pos {
    x: f64,
    y: f64,
}

pub trait Codekaryote<G: genome::Genome> {
    fn update(&self) -> ();
    fn reproduce_genome(&self) -> G;
    fn die(&self) -> ();
    fn reproduce(&self, pos: Pos) -> ();
}

const CREATURE_BODY_INDEX: usize = 0;
const CREATURE_EYES_INDEX: usize = 1;
const CREATURE_TOUCH_INDEX: usize = 2;
const CREATURE_MOVEMENT_INDEX: usize = 3;
const CREATURE_COLOR_INDEX: usize = 4;
const CREATURE_ENERGY_STORAGE_INDEX: usize = 5;
const CREATURE_EATING_INDEX: usize = 6;
const CREATURE_REPRODUCER_INDEX: usize = 7;
const CREATURE_ANCESTRY_INDEX: usize = 8;
const CREATURE_BRAIN_INDEX: usize = 9;

pub struct Creature {
    modules: [Box<dyn Module<Creature, CreatureGenome>>; 9],
}

const PLANT_BODY_INDEX: usize = 0;
const PLANT_ENERGY_SOURCE_INDEX: usize = 1;
const PLANT_COLOR_INDEX: usize = 2;
const PLANT_ANCESTRY_INDEX: usize = 3;

pub struct Plant {
    modules: [Box<dyn Module<Creature, PlantGenome>>; 4],
}

impl Codekaryote<CreatureGenome> for Creature {
    fn update(&self) -> () {
        todo!()
    }

    fn reproduce_genome(&self) -> CreatureGenome {
        todo!()
    }

    fn die(&self) -> () {
        todo!()
    }

    fn reproduce(&self, pos: Pos) -> () {
        todo!()
    }
}

impl Codekaryote<PlantGenome> for Plant {
    fn update(&self) -> () {
        todo!()
    }

    fn reproduce_genome(&self) -> PlantGenome {
        todo!()
    }

    fn die(&self) -> () {
        todo!()
    }

    fn reproduce(&self, pos: Pos) -> () {
        todo!()
    }
}

impl Plant {
    pub fn new() -> Self {
        todo!()
    }
}

impl Creature {
    pub fn new() -> Self {
        todo!()
    }

    pub fn energy_storage(&self) -> &mut EnergyStorage {
        self.modules[CREATURE_ENERGY_STORAGE_INDEX].into()
    }
}
