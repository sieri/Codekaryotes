use crate::codekaryotes::Codekaryote;
use crate::life;
use crate::life::genome::{Chromosome, Genome};

pub trait Module<T, G>
where
    T: Codekaryote<G>,
    G: Genome,
{
    fn update(organism: &mut T);
    fn reset(organism: &mut T);
    fn evolve(&self) -> Chromosome;
}

#[derive(Debug, Clone)]
pub struct Color {
    //For Module
    genome: Chromosome,
    mutation_rate: usize,
    //unique
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Clone)]
pub struct Ancestry {
    //For Module
    genome: Chromosome,
    mutation_rate: usize,
    //unique
    generation: usize,
    pub(crate) age: f64,
}
