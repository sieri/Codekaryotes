use crate::life;
use crate::life::genome::{Chromosome, Genome};
use bevy::prelude::*;

pub trait ChromosomalComponent {
    fn new(c: Chromosome) -> Self;
    fn get_mutated(&self) -> Chromosome;
}

#[derive(Component, Debug, Clone)]
pub struct Color {
    //For Module
    pub(crate) chromosome: Chromosome,
    //unique
    pub(crate) r: f32,
    pub(crate) g: f32,
    pub(crate) b: f32,
}

impl ChromosomalComponent for Color {
    fn new(c: Chromosome) -> Self {
        Color {
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
    pub(crate) age: f64,
}
