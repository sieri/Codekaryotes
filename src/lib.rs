#![allow(dead_code)]
mod brain;

use crate::brain::{Activation, Brain, NeuronDefinition, Position};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn brain_update() {
    print!("Calling rust");
}

#[pyfunction]
fn get_brain() -> Brain {
    Brain {
        inputs: vec![],
        internals: vec![],
        outputs: vec![],
    }
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
    m.add_function(wrap_pyfunction!(brain_update, m)?)?;
    m.add_function(wrap_pyfunction!(get_brain, m)?)?;
    m.add_function(wrap_pyfunction!(acc_from_int, m)?)?;
    m.add_class::<Brain>()?;
    m.add_class::<NeuronDefinition>()?;
    m.add_class::<Activation>()?;
    m.add_class::<Position>()?;
    Ok(())
}
