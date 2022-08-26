use rand::distributions::{Distribution, Uniform};

pub trait Genome {}

pub trait Mutating {
    fn mutate(&self, mutation_rate: usize) -> Self;
}

pub type Chromosome = Vec<i32>;

pub struct CreatureGenome {}

pub struct PlantGenome {}

impl Genome for CreatureGenome {}

impl Genome for PlantGenome {}

impl Mutating for Chromosome {
    fn mutate(&self, mutation_rate: usize) -> Self {
        let mut new = vec![00];
        let lim = new.len();
        let mut rng = rand::thread_rng();
        for n in 0..mutation_rate {
            //let i: usize = rng.gen_range(0..lim);
            // new[i]
        }
        new
    }
}

struct Mutator {
    //rng: rand::ThreadRng,
}
