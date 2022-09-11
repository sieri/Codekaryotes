use crate::life;
use crate::life::genome::{Chromosome, Genome, Mutating};
use crate::utils::scale_between;
use bevy::prelude::*;

pub trait ChromosomalComponent {
    fn new(c: Chromosome) -> Self;
    fn get_mutated(&self) -> Chromosome;
}

#[derive(Component, Debug, Clone)]
pub struct CodekaryoteColor {
    //For Module
    pub(crate) chromosome: Chromosome,
    //unique
    pub(crate) r: f32,
    pub(crate) g: f32,
    pub(crate) b: f32,
}

impl ChromosomalComponent for CodekaryoteColor {
    fn new(c: Chromosome) -> Self {
        CodekaryoteColor {
            chromosome: c.to_vec(),
            r: (c[0] as f32) / (u32::MAX as f32),
            g: (c[1] as f32) / (u32::MAX as f32),
            b: (c[2] as f32) / (u32::MAX as f32),
        }
    }

    fn get_mutated(&self) -> Chromosome {
        self.chromosome.to_vec()
    }
}

#[derive(Component, Debug, Clone)]
pub struct Ancestry {
    //For Module
    pub(crate) genome: Chromosome,
    pub(crate) mutation_rate: usize,
    //unique
    pub(crate) generation: u32,
    pub(crate) age: f32,
}

#[derive(Component, Debug, Clone)]
pub struct CodekaryoteBody {
    //For Module
    chromosome: Chromosome,
    //Unique
    pub(crate) size: f32,
    pub(crate) mass: f32,
}

impl ChromosomalComponent for CodekaryoteBody {
    fn new(c: Chromosome) -> Self {
        const MIN: f32 = 10.0;
        const MAX: f32 = 40.0;
        const BODY_MASS_UNIT: f32 = 1f32;
        let size: f32 = scale_between(c[0] as f32, MIN, MAX, None, None);
        let mass: f32 = size.powi(2) * BODY_MASS_UNIT;

        CodekaryoteBody {
            chromosome: c.to_vec(),
            size,
            mass,
        }
    }

    fn get_mutated(&self) -> Chromosome {
        self.chromosome.mutate(1)
    }
}

#[derive(Component, Debug, Clone)]
pub struct Parent {
    pub(crate) entity: Entity
}
