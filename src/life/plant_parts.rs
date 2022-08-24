use crate::life::genome::Chromonsone;
use pyo3::PyObject;

pub struct EnergySource {
    //For Module
    genome: Chromonsone,
    mutation_rate: usize,
    //unique
    energy: f64,
}

pub struct PlantBody {
    //For Module
    genome: Chromonsone,
    mutation_rate: usize,
    //unique
    mass: usize,
    circle: PyObject,
}
