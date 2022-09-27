use arr_macro::arr;
use bevy::prelude::*;
use rand::Rng;

pub trait Genome {
    fn mutate(&self) -> Self;
}

pub trait Mutating {
    fn mutate(&self, mutation_rate: usize) -> Self;
}

pub type Chromosome = Vec<u32>;

#[derive(Component, Clone)]
pub struct CreatureGenome {
    pub(crate) body: Chromosome,
    pub(crate) eyes: Chromosome,
    pub(crate) movement: Chromosome,
    pub(crate) color: Chromosome,
    pub(crate) energy_storage: Chromosome,
    pub(crate) ancestry: Chromosome,
    pub(crate) brain: Chromosome,
}

pub struct PlantGenome {
    pub(crate) color: Chromosome,
    pub(crate) body: Chromosome,
}

impl Genome for CreatureGenome {
    fn mutate(&self) -> Self {
        CreatureGenome {
            body: self.body.mutate(1),
            eyes: self.eyes.mutate(1),
            movement: self.movement.mutate(1),
            color: self.color.mutate(1),
            energy_storage: self.energy_storage.mutate(1),
            ancestry: self.ancestry.to_vec(),
            brain: self.brain.mutate(5),
        }
    }
}

impl CreatureGenome {
    pub(crate) fn new() -> CreatureGenome {
        const M: u32 = u32::MAX;
        let mut mutator = rand::thread_rng();

        const INPUT_COUNT: usize = 18usize;
        const INTERNAL_COUNT: usize = 42usize;
        const OUTPUT_COUNT: usize = 4usize;
        const LINKS_COUNT: usize = 70usize;

        let brain: [u32; INTERNAL_COUNT + OUTPUT_COUNT + INPUT_COUNT + LINKS_COUNT] =
            arr![mutator.gen_range(0..M); 134];

        CreatureGenome {
            body: vec![mutator.gen_range(0..M)],
            eyes: vec![mutator.gen_range(0..M), mutator.gen_range(0..M)],
            movement: vec![mutator.gen_range(0..M), mutator.gen_range(0..M)],
            color: vec![
                mutator.gen_range(0..M),
                mutator.gen_range(0..M),
                mutator.gen_range(0..M),
            ],
            energy_storage: vec![mutator.gen_range(0..M)],
            ancestry: vec![0, 0],
            brain: brain.to_vec(),
        }
    }
}

impl PlantGenome {
    pub(crate) fn new() -> PlantGenome {
        const M: u32 = u32::MAX;
        let mut mutator = rand::thread_rng();

        PlantGenome {
            body: vec![mutator.gen_range(0..M)],
            color: vec![0, M, 0],
        }
    }
}

impl Genome for PlantGenome {
    fn mutate(&self) -> Self {
        todo!()
    }
}

fn toggle_bit(val: u32, index: u8) -> u32 {
    let mask: u32 = 1 << index;
    val ^ mask
}

impl Mutating for Chromosome {
    fn mutate(&self, mutation_rate: usize) -> Self {
        let mut new = self.to_vec();
        let mut mutator = rand::thread_rng();
        let lim = new.len();

        for _ in 0..mutation_rate {
            let i: usize = mutator.gen_range(0..lim);
            let b: u8 = mutator.gen_range(0..32);
            new[i] = toggle_bit(new[i], b);
        }
        new
    }
}
