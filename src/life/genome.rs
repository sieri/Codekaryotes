use arr_macro::arr;
use rand::Rng;

pub trait Genome {}

pub trait Mutating {
    fn mutate(&self, mutation_rate: usize) -> Self;
}

pub type Chromosome = Vec<u32>;

pub struct CreatureGenome {
    pub(crate) body: Chromosome,
    pub(crate) eyes: Chromosome,
    pub(crate) movement: Chromosome,
    pub(crate) color: Chromosome,
    pub(crate) energy_storage: Chromosome,
    pub(crate) ancestry: Chromosome,
    pub(crate) brain: Chromosome,
}

pub struct PlantGenome {}

impl Genome for CreatureGenome {}

impl CreatureGenome {
    pub(crate) fn new() -> CreatureGenome {
        let m = u32::MAX;
        let mut mutator = rand::thread_rng();

        const INPUT_COUNT: usize = 18usize;
        const INTERNAL_COUNT: usize = 42usize;
        const OUTPUT_COUNT: usize = 4usize;
        const LINKS_COUNT: usize = 70usize;

        let brain: [u32; INTERNAL_COUNT + OUTPUT_COUNT + INPUT_COUNT + LINKS_COUNT] =
            arr![mutator.gen_range(0..m); 134];

        CreatureGenome {
            body: vec![mutator.gen_range(0..m)],
            eyes: vec![mutator.gen_range(0..m), mutator.gen_range(0..m)],
            movement: vec![mutator.gen_range(0..m)],
            color: vec![
                mutator.gen_range(0..m),
                mutator.gen_range(0..m),
                mutator.gen_range(0..m),
            ],
            energy_storage: vec![mutator.gen_range(0..m)],
            ancestry: vec![0, 0],
            brain: brain.to_vec(),
        }
    }
}

impl Genome for PlantGenome {}

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
