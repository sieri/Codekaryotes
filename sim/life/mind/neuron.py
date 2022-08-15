import math
from enum import IntEnum

from sim.parameters.settings import Settings


class Activations(IntEnum):
    LINEAR = 0
    BINARY_STEP = 1
    LOGISTIC = 2
    TANH = 3
    GAUSSIAN = 4

    @classmethod
    def from_genome(cls, genome):
        """
        get an activation from a genome int
        :param genome: the genome
        :type genome: ``int``
        :return: the activation
        :rtype: ``Activation``
        """
        return genome % len(cls)
# end class Operation


class Neuron:
    """The base neuron of the network"""

    def __init__(self, activation, outputs=None):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param outputs: The list of other neurons the output of the activation is connected to - OPTIONAL
        :type outputs: ``list[Link]
        """

        self._interface_input = None
        self._interface_output = None
        self._interface_index = None
        if outputs is None:
            outputs = []
        self._activation = activation
        self._outputs = outputs
        self._inputs = []
    # end def __init__

    # -------------------Methods--------------------



    # -----------------Properties------------------

    @property
    def input(self):
        return self._interface_input[self._interface_index]
    # end def input

    @input.setter
    def input(self, value):
        self._interface_input[self._interface_index] += value
    # end def input

    @property
    def output(self):
        self._interface_input[self._interface_index] = 0
        return self._interface_output[self._interface_index]
    # end def outputs

    @property
    def activation(self):
        return self._activation
    # end def activation

    @property
    def interface(self):
        return self._interface_index, self._interface_input, self._interface_output
    # end def interface

    @interface.setter
    def interface(self, value):
        self._interface_index, self._interface_input, self._interface_output = value
    # end def interface

# end class neuron


class NeuronExit(Neuron):
    """Neuron that acts on a system of the body"""

    def __init__(self, activation, module):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param module: the module to interact with
        :type module: ``BaseModule``
        """
        super().__init__(activation)
        self._module = module

    # -------------------Methods--------------------

    def update(self):
        raise NotImplementedError
    # end def __init__

    # -----------------Properties------------------


# end class NeuronExit

class NeuronInput(Neuron):
    """
    Neuron that reads something from the environment/body
    """
    def __init__(self, activation, module, outputs=None):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param module: the module to interact with
        :type module: ``BaseModule``
        :param outputs: The list of other neurons the output of the activation is connected to - OPTIONAL
        :type outputs: ``list[Link]
        """
        super().__init__(activation, outputs)
        self._module = module

    # -------------------Methods--------------------

    def prepare(self):
        """
        Sums the inputs and setup for the activation function
        """
        s = 0
        for i in self._inputs:
            s += i
        # end for
        self._inputs.clear()
        print(s)
        if not Settings().brain_rust:
            self._interface_input[self._interface_index] = s
        else:
            return s


    # -----------------Properties------------------

    @property
    def input(self):
        return self.prepare()
    # end def input

# end class NeuronInput


class Link:
    """
    links two neurons, with a specific weights
    """
    def __init__(self, source, output, weight, id):
        """
        :param source: source neuron
        :type source: ``Neuron``
        :param output: output neuron
        :type output:  ``Neuron``
        :param weight: the weight
        :type weight: ``float``
        """
        self._weight = weight
        self._output = output
        self._input = source
        self.id = id

    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        self._output.input = self._input.output*self._weight
    # end def update

    # -----------------Properties------------------
# end class Link
