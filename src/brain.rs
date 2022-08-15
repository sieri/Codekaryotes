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
    pub in_val: f64,
    pub object: Option<PyObject>,
}

#[derive(Debug)]
#[pyclass(module = "codekaryotes.codekaryotes")]
#[derive(FromPyObject)]
pub struct LinkDefinition {
    #[pyo3(get, set)]
    pub weight: f64,
    #[pyo3(get, set)]
    pub input: usize,
    #[pyo3(get, set)]
    pub output: usize,
    #[pyo3(get, set)]
    pub input_type: Position,
    #[pyo3(get, set)]
    pub output_type: Position,
}

#[derive(Debug)]
struct Link {
    input: usize,
    output: usize,
    weight: f64,
}

#[derive(Debug)]
pub struct NeuronLists {
    linear: Vec<usize>,
    binary: Vec<usize>,
    logistic: Vec<usize>,
    tanh: Vec<usize>,
    gaussian: Vec<usize>,
}

#[derive(Debug)]
#[pyclass(module = "codekaryotes.codekaryotes")]
pub struct Brain {
    inputs: Vec<NeuronDefinition>,
    internals: Vec<NeuronDefinition>,
    outputs: Vec<NeuronDefinition>,
    links_def: Vec<LinkDefinition>,

    links: Vec<Link>,

    neurons: Vec<Neuron>,

    neurons_id_input: NeuronLists,
    neurons_id_internal: NeuronLists,
    neurons_id_output: NeuronLists,
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

trait NeuronalInterface {
    fn o(&self) -> f64;
    fn i(&mut self, val: f64);
}

trait LinkTrait {
    fn update_neuron(&mut self);
}

impl Neuron {
    fn new(id: usize) -> Neuron {
        Neuron {
            id,
            out_val: 0.0,
            in_val: 0.0,
            object: None,
        }
    }

    fn new_obj(id: usize, object: PyObject) -> Neuron {
        Neuron {
            id,
            out_val: 0.0,
            in_val: 0.0,
            object: Some(object),
        }
    }

    fn update_neural_input(&mut self, py: Python<'_>) {
        self.in_val = (self.object)
            .as_ref()
            .unwrap()
            .getattr(py, "input")
            .unwrap()
            .extract(py)
            .unwrap();
    }

    fn write_in(&mut self, val: f64) {
        self.in_val = val;
    }
}

impl Link {
    fn new(input: usize, output: usize, weight: f64) -> Self {
        Link {
            input,
            output,
            weight,
        }
    }
}

impl NeuronLists {
    fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self,
            index: 0,
        }
    }

    fn find(&self, id: usize) -> usize {
        for l in self.iter() {
            let res = l.iter().find(|&&x| x == id);
            if res.is_some() {
                return *res.unwrap();
            }
        }
        0
    }
}

struct Iter<'a> {
    inner: &'a NeuronLists,
    index: u8,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => &self.inner.linear,
            1 => &self.inner.binary,
            2 => &self.inner.gaussian,
            3 => &self.inner.tanh,
            4 => &self.inner.logistic,
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}

#[pymethods]
impl LinkDefinition {
    #[new]
    fn __new__(input: usize, output: usize, weight: f64, input_type: Position) -> Self {
        LinkDefinition {
            weight,
            output,
            input_type,
            input,
            output_type: Position::Output,
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
            links_def: vec![],
            links: vec![],

            neurons: vec![],

            neurons_id_input: NeuronLists {
                linear: vec![],
                binary: vec![],
                logistic: vec![],
                tanh: vec![],
                gaussian: vec![],
            },
            neurons_id_internal: NeuronLists {
                linear: vec![],
                binary: vec![],
                logistic: vec![],
                tanh: vec![],
                gaussian: vec![],
            },
            neurons_id_output: NeuronLists {
                linear: vec![],
                binary: vec![],
                logistic: vec![],
                tanh: vec![],
                gaussian: vec![],
            },
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

    pub fn add_link(&mut self, l: LinkDefinition) -> PyResult<()> {
        self.links_def.push(l);
        Ok(())
    }

    pub fn initiate(&mut self) -> PyResult<()> {
        let gil = Python::acquire_gil();
        let &py = &gil.python();

        let internal_prefix: usize = 2usize.pow(31);
        let output_prefix: usize = 2usize.pow(32);

        for input in self.inputs.iter() {
            match input.activation {
                Activation::Linear => self.neurons_id_input.linear.push(input.id),
                Activation::BinaryStep => self.neurons_id_input.binary.push(input.id),
                Activation::Logistic => self.neurons_id_input.logistic.push(input.id),
                Activation::Tanh => self.neurons_id_input.tanh.push(input.id),
                Activation::Gaussian => self.neurons_id_input.gaussian.push(input.id),
            }
            self.neurons
                .push(Neuron::new_obj(input.id, input.object.clone_ref(py)))
        }

        for internal in self.internals.iter() {
            match internal.activation {
                Activation::Linear => self
                    .neurons_id_internal
                    .linear
                    .push(internal_prefix + internal.id),
                Activation::BinaryStep => self
                    .neurons_id_internal
                    .binary
                    .push(internal_prefix + internal.id),
                Activation::Logistic => self
                    .neurons_id_internal
                    .logistic
                    .push(internal_prefix + internal.id),
                Activation::Tanh => self
                    .neurons_id_internal
                    .tanh
                    .push(internal_prefix + internal.id),
                Activation::Gaussian => self
                    .neurons_id_internal
                    .gaussian
                    .push(internal_prefix + internal.id),
            }
            self.neurons
                .push(Neuron::new(internal_prefix + internal.id))
        }

        for output in self.outputs.iter() {
            match output.activation {
                Activation::Linear => self
                    .neurons_id_output
                    .linear
                    .push(output_prefix + output.id),
                Activation::BinaryStep => self
                    .neurons_id_output
                    .binary
                    .push(output_prefix + output.id),
                Activation::Logistic => self
                    .neurons_id_output
                    .logistic
                    .push(output_prefix + output.id),
                Activation::Tanh => self.neurons_id_output.tanh.push(output_prefix + output.id),
                Activation::Gaussian => self
                    .neurons_id_output
                    .gaussian
                    .push(output_prefix + output.id),
            }
            self.neurons.push(Neuron::new_obj(
                output_prefix + output.id,
                output.object.clone_ref(py),
            ))
        }

        // create links
        for l in self.links_def.iter() {
            let input = match l.input_type {
                Position::Input => self.neurons_id_input.find(l.input),
                Position::Internal => self.neurons_id_internal.find(l.input),
                _ => 0,
            };
            let output = match l.output_type {
                Position::Output => self.neurons_id_output.find(l.output),
                Position::Internal => self.neurons_id_internal.find(l.output),
                _ => 0,
            };

            let nl = Link::new(input, output, l.weight);
            self.links.push(nl);
        }

        Ok(())
    }

    pub fn update(&mut self) -> PyResult<()> {
        let gil = Python::acquire_gil();
        let &py = &gil.python();

        //feed links
        self.links.iter_mut().for_each(|x| -> () {
            let new_val = self.neurons[x.input].out_val * x.weight;
            self.neurons[x.output].write_in(new_val);
        });

        // acquire inputs
        for inp in self.neurons_id_input.iter() {
            for x in inp.iter() {
                self.neurons[*x].update_neural_input(py);
            }
        }

        //activate neurons

        Ok(())
    }

    pub fn output(&self) -> PyResult<()> {
        Ok(())
    }
}
