import math
from enum import IntEnum


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

        if outputs is None:
            outputs = []
        self._activation = activation
        self._outputs = outputs
        self._inputs = []
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        """
        Update the neuron
        """
        y = self._activate()

        self._output_to_links(y)

    def _output_to_links(self, y):
        for o in self._outputs:
            o.value = y

    def _activate(self):
        """
        Sums the inputs and run the activation function
        :return: Output value
        :rtype: ``float``
        """
        s = 0
        for i in self._inputs:
            s += i
        # end for
        self._inputs.clear()
        if self._activation == Activations.LINEAR:
            y = s
        elif self._activation == Activations.BINARY_STEP:
            y = 0 if s < 0 else 1
        elif self._activation == Activations.LOGISTIC:
            if s >= 0:
                z = math.exp(-s)
                y = 1 / (1 + z)
            else:
                z = math.exp(s)
                y = z / (1 + z)
        elif self._activation == Activations.TANH:
            y = math.tanh(s)
        elif self._activation == Activations.GAUSSIAN:
            y = math.exp(-(s * s))
        else:
            y = 0
        return y

    # -----------------Properties------------------

    @property
    def input(self):
        return self._inputs
    # end def input

    @input.setter
    def input(self, value):
        self._inputs.append(value)
    # end def input

    @property
    def outputs(self):
        return self._outputs
    # end def outputs

    @outputs.setter
    def outputs(self, value):
        self._outputs.append(value)
    # end def outputs
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

    def update(self):
        raise NotImplementedError
    # end def __init__

    # -----------------Properties------------------



# end class NeuronInput


class Link:
    """
    links two neurons, with a specific weights
    """
    def __init__(self, source, output, weight):
        """
        :param source: source neuron
        :type source: ``Neuron``
        :param output: output neuron
        :type output:  ``Neuron``
        :param weight: the weight
        :type weight: ``float``
        """
        self._value = 0
        self._weight = weight
        self._output = output
        self._input = source

        self._input.outputs = self
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        self._output.input = self._value
    # end def update

    # -----------------Properties------------------

    @property
    def value(self):
        return self._value
    # end def value

    @value.setter
    def value(self, value):
        self._value = value*self._weight
    # end def value


# end class Link