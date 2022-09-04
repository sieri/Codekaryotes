use crate::life::genome::Chromosome;

pub struct EnergySource {
    //For Module
    genome: Chromosome,
    mutation_rate: usize,
    //unique
    energy: f64,
}

pub struct PlantBody {
    //For Module
    genome: Chromosome,
    mutation_rate: usize,
    //unique
    mass: usize,
    circle: Option<()>,
}
