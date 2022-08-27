use crate::life::common_parts::{Ancestry, Color};
use crate::life::creature_parts::{
    ActiveModule, CreatureBody, Eating, EnergyStorage, Eyes, Movement, Reproducer, Touch,
};
use crate::life::genome::{CreatureGenome, PlantGenome};
use crate::life::plant_parts::{EnergySource, PlantBody};
use crate::{life, Brain};
use life::common_parts::Module;
use life::genome;
use pyo3::intern;
use std::borrow::{Borrow, BorrowMut};
use std::marker::PhantomData;
use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub struct Pos {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

pub trait Codekaryote<G: genome::Genome> {
    fn update(&mut self) -> ();
    fn reproduce_genome(&self) -> G;
    fn die(&self) -> ();
    fn reproduce(&self, pos: Pos) -> ();

    fn get_position(&self) -> Pos;
}

pub struct Creature(
    CreatureBody,
    Eyes,
    Touch,
    Movement,
    Color,
    EnergyStorage,
    Eating,
    Reproducer,
    Ancestry,
    Brain,
);

pub struct Plant(PlantBody, EnergySource, Color, Ancestry);

impl Codekaryote<CreatureGenome> for Creature {
    fn update(&mut self) -> () {
        CreatureBody::update(self);
        Eyes::update(self);
        Touch::update(self);
        Movement::update(self);
        Color::update(self);
        EnergyStorage::update(self);
        Eating::update(self);
        Reproducer::update(self);
        Ancestry::update(self);
        Brain::update(self);

        //Tally the energy consumption
        let energy_consumption = self.0.get_energy_rate()
            + self.1.get_energy_rate()
            + self.3.get_energy_rate()
            + self.5.get_energy_rate()
            + self.9.get_energy_rate();

        let still_alive = self.5.consume(energy_consumption);

        if !still_alive {
            self.die();
            return;
        }

        //reset the modules
        CreatureBody::reset(self);
        Eyes::reset(self);
        Touch::reset(self);
        Movement::reset(self);
        Color::reset(self);
        EnergyStorage::reset(self);
        Eating::reset(self);
        Reproducer::reset(self);
        Ancestry::reset(self);
        Brain::reset(self);
    }

    fn reproduce_genome(&self) -> CreatureGenome {
        CreatureGenome {
            body: self.0.evolve(),
            eyes: self.1.evolve(),
            movement: self.3.evolve(),
            color: self.4.evolve(),
            energy_storage: self.5.evolve(),
            ancestry: self.8.evolve(),
            brain: self.9.evolve(),
        }
    }

    fn die(&self) -> () {
        todo!()
    }

    fn reproduce(&self, pos: Pos) -> () {
        todo!()
    }

    fn get_position(&self) -> Pos {
        let body = &self.0;
        body.get_position()
    }
}

impl Codekaryote<PlantGenome> for Plant {
    fn update(self: &mut Plant) -> () {
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

    fn get_position(&self) -> Pos {
        todo!()
    }
}

enum Kind {
    Creature,
    Plant,
}

pub struct Seen {
    pos: Pos,
    kind: Kind,
}

impl Plant {
    pub fn new() -> Self {
        todo!()
    }
    pub fn body(&mut self) -> &mut PlantBody {
        &mut self.0
    }
    pub fn energy_source(&mut self) -> &mut EnergySource {
        &mut self.1
    }
    pub fn color(&mut self) -> &mut Color {
        &mut self.2
    }
    pub fn ancestry(&mut self) -> &mut Ancestry {
        &mut self.3
    }
}

impl Creature {
    pub fn new(genome: CreatureGenome, pos: Pos) -> Self {
        println!("New!");
        Creature {
            0: CreatureBody::new(genome.body),
            1: Eyes::new(genome.eyes),
            2: Touch::new(vec![]),
            3: Movement::new(genome.movement),
            4: Color::new(genome.color),
            5: EnergyStorage::new(genome.energy_storage),
            6: Eating::new(vec![]),
            7: Reproducer::new(vec![]),
            8: Ancestry::new(genome.ancestry),
            9: Brain::new(genome.brain),
        }
    }

    pub fn new_rand(pos: Pos) -> Self {
        Self::new(CreatureGenome::new(), pos)
    }

    pub fn body(&mut self) -> &mut CreatureBody {
        &mut self.0
    }
    pub fn eyes(&mut self) -> &mut Eyes {
        &mut self.1
    }
    pub fn touch(&mut self) -> &mut Touch {
        &mut self.2
    }
    pub fn movement(&mut self) -> &mut Movement {
        &mut self.3
    }
    pub fn color(&mut self) -> &mut Color {
        &mut self.4
    }
    pub fn energy_storage(&mut self) -> &mut EnergyStorage {
        &mut self.5
    }
    pub fn eating(&mut self) -> &mut Eating {
        &mut self.6
    }
    pub fn reproducer(&mut self) -> &mut Reproducer {
        &mut self.7
    }
    pub fn ancestry(&mut self) -> &mut Ancestry {
        &mut self.8
    }
    pub fn brain(&mut self) -> &mut Brain {
        &mut self.9
    }
}

impl Pos {
    pub(crate) fn dist(&self, other: Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.x - other.x).powi(2)).sqrt()
    }
}
