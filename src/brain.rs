#![allow(dead_code)]

extern crate pyo3;

use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass(module = "codekaryotes.codekaryotes")]
pub enum Activation {
    Linear = 0,
    BinaryStep = 1,
    Logistic = 2,
    Tanh = 3,
    Gaussian = 4,
}

#[derive(Debug, Clone, Copy)]
#[pyclass(module = "codekaryotes.codekaryotes")]
pub enum Position {
    Input,
    Output,
    Internal,
}

#[derive(Debug)]
#[pyclass(module = "codekaryotes.codekaryotes")]
#[derive(FromPyObject)]
pub struct NeuronDefinition {
    #[pyo3(get, set)]
    pub activation: Activation,
    #[pyo3(get, set)]
    pub position: Position,
    #[pyo3(get, set)]
    pub id: usize,
    #[pyo3(get, set)]
    pub object: PyObject,
}

#[derive(Debug)]
#[pyclass(module = "codekaryotes.codekaryotes")]
pub struct Brain {
    pub inputs: Vec<NeuronDefinition>,
    pub internals: Vec<NeuronDefinition>,
    pub outputs: Vec<NeuronDefinition>,
}

#[pymethods]
impl NeuronDefinition {
    #[new]
    fn __new__(activation: Activation, pos: Position, id: usize, linked_obj: PyObject) -> Self {
        println!("Activation {:?}", activation);
        println!("pos: {:?}", pos);
        println!("id: {:?}", id);
        println!("linked_obj: {:?}", linked_obj);
        /*        NeuronDefinition {
            activation,
            id,
            position: pos,
            object: linked_obj,
        }*/
        NeuronDefinition {
            activation: Activation::Linear,
            id: 0,
            position: pos,
            object: linked_obj,
        }
    }
}

#[pymethods]
impl Brain {
    pub fn add_input(&mut self, a: NeuronDefinition) -> PyResult<()> {
        println!("Hello");
        self.inputs.push(a);
        Ok(())
    }

    pub fn add_internal(&mut self, a: NeuronDefinition) -> PyResult<()> {
        println!("Hello");
        self.inputs.push(a);
        Ok(())
    }

    pub fn add_output(&mut self, a: NeuronDefinition) -> PyResult<()> {
        println!("Hello");
        self.inputs.push(a);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        println!("Hello");
        Python::with_gil(|py| {
            let node = NeuronDefinition {
                activation: Activation::Linear,
                position: Position::Input,
                id: 0,
                object: py.None(),
            };
            println!("Node {:?}", node);
            let mut brain = Brain {
                inputs: vec![],
                internals: vec![],
                outputs: vec![],
            };
            println!("brain 1 {:?}", brain);
            let res = brain.add_input(node);
            println!("brain 2 {:?}", brain);
            res
        });
    }
}
