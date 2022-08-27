#![allow(dead_code)]

mod codekaryotes;
mod life;

use crate::codekaryotes::{Codekaryote, Creature, Pos};
use crate::life::genome::CreatureGenome;
use life::brain::{Activation, Brain, LinkDefinition, NeuronDefinition, Position};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn get_brain() -> Brain {
    Brain::new_py()
}

#[pyfunction]
fn test() {
    println!("Rust test");
    let creature = Creature::new(CreatureGenome::new(), Pos { x: 0.0, y: 0.0 });
}

#[pyfunction]
fn acc_from_int(val: usize) -> PyResult<Activation> {
    match val {
        0 => Ok(Activation::Linear),
        1 => Ok(Activation::BinaryStep),
        2 => Ok(Activation::Logistic),
        3 => Ok(Activation::Tanh),
        4 => Ok(Activation::Gaussian),
        _ => Err(PyValueError::new_err("Unknown Activation")),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn codekaryotes(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_brain, m)?)?;
    m.add_function(wrap_pyfunction!(acc_from_int, m)?)?;
    m.add_function(wrap_pyfunction!(test, m)?)?;
    m.add_class::<Brain>()?;
    m.add_class::<NeuronDefinition>()?;
    m.add_class::<Activation>()?;
    m.add_class::<Position>()?;
    m.add_class::<LinkDefinition>()?;
    Ok(())
}
