use crate::life::codekaryotes::Codekaryote;
use crate::life;
use crate::life::genome::{Chromosome, Genome};

pub trait Module<T, G>
where
    T: Codekaryote<G>,
    G: Genome,
{
    fn new(chromosome: Chromosome) -> Self;
    fn update(organism: &mut T);
    fn reset(organism: &mut T);
    fn evolve(&self) -> Chromosome;
}

#[derive(Debug, Clone)]
pub struct Color {
    //For Module
    pub(crate) genome: Chromosome,
    pub(crate) mutation_rate: usize,
    //unique
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

#[derive(Debug, Clone)]
pub struct Ancestry {
    //For Module
    pub(crate) genome: Chromosome,
    pub(crate) mutation_rate: usize,
    //unique
    pub(crate) generation: u32,
    pub(crate) age: f64,
}
