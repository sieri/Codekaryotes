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
pub struct Neuron {
    pub id: usize,
    pub out_val: f64,
}

#[derive(Debug)]
pub struct NeuronInput {
    pub id: usize,
    pub out_val: f64,
    pub object: PyObject,
}

#[derive(Debug)]
pub struct NeuronOutput {
    pub id: usize,
    pub out_val: f64,
    pub object: PyObject,
}

#[derive(Debug)]
#[pyclass(module = "codekaryotes.codekaryotes")]
#[derive(FromPyObject)]
pub struct Link {
    #[pyo3(get, set)]
    pub weight: f64,
    #[pyo3(get, set)]
    pub input: usize,
    #[pyo3(get, set)]
    pub output: usize,
}

#[derive(Debug)]
#[pyclass(module = "codekaryotes.codekaryotes")]
pub struct Brain {
    inputs: Vec<NeuronDefinition>,
    internals: Vec<NeuronDefinition>,
    outputs: Vec<NeuronDefinition>,
    links: Vec<Link>,

    linear_internal: Vec<Neuron>,
    binary_step_internal: Vec<Neuron>,
    logistic_internal: Vec<Neuron>,
    tanh_internal: Vec<Neuron>,
    gaussian_internal: Vec<Neuron>,

    linear_input: Vec<NeuronInput>,
    binary_step_input: Vec<NeuronInput>,
    logistic_input: Vec<NeuronInput>,
    tanh_input: Vec<NeuronInput>,
    gaussian_input: Vec<NeuronInput>,

    linear_output: Vec<NeuronOutput>,
    binary_step_output: Vec<NeuronOutput>,
    logistic_output: Vec<NeuronOutput>,
    tanh_output: Vec<NeuronOutput>,
    gaussian_output: Vec<NeuronOutput>,
}

#[pymethods]
impl NeuronDefinition {
    #[new]
    fn __new__(activation: Activation, pos: Position, id: usize, linked_obj: PyObject) -> Self {
        NeuronDefinition {
            activation,
            id,
            position: pos,
            object: linked_obj,
        }
    }
}

impl Neuron {
    fn new(id: usize) -> Neuron {
        Neuron { id, out_val: 0.0 }
    }
}

impl NeuronInput {
    fn new(id: usize, object: PyObject) -> NeuronInput {
        NeuronInput {
            id,
            out_val: 0.0,
            object: object,
        }
    }

    fn update(&mut self, py: Python<'_>) {
        let name = self.object.getattr(py, "__type__");
        println!("module {:?}", name);
        let a = self.object.getattr(py, "input");
        println!("a {:?}", a);
        let b = a.unwrap();
        println!("b {:?}", b);
        let c = b.extract(py);
        println!("c {:?}", c);

        self.out_val = c.unwrap();
    }
}

impl NeuronOutput {
    fn new(id: usize, object: PyObject) -> NeuronOutput {
        NeuronOutput {
            id,
            out_val: 0.0,
            object,
        }
    }
}

#[pymethods]
impl Link {
    #[new]
    fn __new__(input: usize, output: usize, weight: f64) -> Self {
        Link {
            weight,
            output,
            input,
        }
    }
}

#[pymethods]
impl Brain {
    #[staticmethod]
    pub fn new() -> Brain {
        Brain {
            inputs: vec![],
            internals: vec![],
            outputs: vec![],
            links: vec![],
            linear_internal: vec![],
            binary_step_internal: vec![],
            logistic_internal: vec![],
            tanh_internal: vec![],
            gaussian_internal: vec![],
            linear_input: vec![],
            binary_step_input: vec![],
            logistic_input: vec![],
            tanh_input: vec![],
            gaussian_input: vec![],
            linear_output: vec![],
            binary_step_output: vec![],
            logistic_output: vec![],
            tanh_output: vec![],
            gaussian_output: vec![],
        }
    }

    pub fn add_input(&mut self, a: NeuronDefinition) -> PyResult<()> {
        self.inputs.push(a);
        Ok(())
    }

    pub fn add_internal(&mut self, a: NeuronDefinition) -> PyResult<()> {
        self.internals.push(a);
        Ok(())
    }

    pub fn add_output(&mut self, a: NeuronDefinition) -> PyResult<()> {
        self.outputs.push(a);
        Ok(())
    }

    pub fn add_link(&mut self, l: Link) -> PyResult<()> {
        self.links.push(l);
        Ok(())
    }

    pub fn initiate(&mut self) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();
        let &py = &gil.python();

        for input in self.inputs.iter() {
            match input.activation {
                Activation::Linear => self
                    .linear_input
                    .push(NeuronInput::new(input.id, input.object.clone_ref(py))),
                Activation::BinaryStep => self
                    .binary_step_input
                    .push(NeuronInput::new(input.id, input.object.clone_ref(py))),
                Activation::Logistic => self
                    .logistic_input
                    .push(NeuronInput::new(input.id, input.object.clone_ref(py))),
                Activation::Tanh => self
                    .tanh_input
                    .push(NeuronInput::new(input.id, input.object.clone_ref(py))),
                Activation::Gaussian => self
                    .gaussian_input
                    .push(NeuronInput::new(input.id, input.object.clone_ref(py))),
            }
        }

        for internal in self.internals.iter() {
            match internal.activation {
                Activation::Linear => self.linear_internal.push(Neuron::new(internal.id)),
                Activation::BinaryStep => self.binary_step_internal.push(Neuron::new(internal.id)),
                Activation::Logistic => self.logistic_internal.push(Neuron::new(internal.id)),
                Activation::Tanh => self.tanh_internal.push(Neuron::new(internal.id)),
                Activation::Gaussian => self.gaussian_internal.push(Neuron::new(internal.id)),
            }
        }

        for output in self.outputs.iter() {
            match output.activation {
                Activation::Linear => self
                    .linear_output
                    .push(NeuronOutput::new(output.id, output.object.clone_ref(py))),
                Activation::BinaryStep => self
                    .binary_step_output
                    .push(NeuronOutput::new(output.id, output.object.clone_ref(py))),
                Activation::Logistic => self
                    .logistic_output
                    .push(NeuronOutput::new(output.id, output.object.clone_ref(py))),
                Activation::Tanh => self
                    .tanh_output
                    .push(NeuronOutput::new(output.id, output.object.clone_ref(py))),
                Activation::Gaussian => self.gaussian_output.push(NeuronOutput::new(
                    output.id,
                    output.object.clone_ref(py).clone_ref(py),
                )),
            }
        }

        Ok(())
    }

    pub fn update(&mut self) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();
        let &py = &gil.python();

        // acquire inputs

        for x in self.linear_input.iter_mut() {
            x.update(py);
        }
        for x in self.binary_step_input.iter_mut() {
            x.update(py);
        }
        for x in self.logistic_input.iter_mut() {
            x.update(py);
        }
        for x in self.tanh_input.iter_mut() {
            x.update(py);
        }
        for x in self.gaussian_input.iter_mut() {
            x.update(py);
        }

        Ok(())
    }

    pub fn output(&self) -> PyResult<()> {
        Ok(())
    }
}
