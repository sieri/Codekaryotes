use crate::life::genome::Chromonsone;

pub struct EnergySource{
    //For Module
    genome: Chromonsone,
    mutation_rate: usize,
    //unique
    energy: f64
}