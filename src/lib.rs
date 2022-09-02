#![allow(dead_code)]

extern crate core;

mod codekaryotes;
mod life;

use crate::codekaryotes::{Codekaryote, Creature, Pos};
use crate::life::genome::CreatureGenome;
use life::brain::{Activation, Brain, LinkDefinition, Position};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn test() {
    println!("Rust test");
    let creature = Creature::new(CreatureGenome::new(), Pos { x: 0.0, y: 0.0 });
}

/// A Python module implemented in Rust.
#[pymodule]
fn codekaryotes(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test, m)?)?;
    m.add_class::<Creature>()?;

    Ok(())
}
