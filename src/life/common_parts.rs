use crate::codekaryotes::Codekaryote;
use crate::life;
use crate::life::genome::{Chromonsone, Genome};

pub trait Module<T, G>
where
    T: Codekaryote<G>,
    G: Genome,
{
    fn by_box(self: Box<Self>);
    fn update(&self, organism: T);
    fn reset(&self, organism: T);
    fn evolve(&self) -> Chromonsone;
}

#[derive(Debug, Clone)]
pub struct Color {
    //For Module
    genome: Chromonsone,
    mutation_rate: usize,
    //unique
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Clone)]
pub struct Ancestry {
    //For Module
    genome: Chromonsone,
    mutation_rate: usize,
    //unique
    generation: usize,
    age: f64,
}
