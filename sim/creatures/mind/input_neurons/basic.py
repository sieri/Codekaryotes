from sim.creatures.mind.neuron import NeuronInput

THRESHOlD = 0.5


class ConstantNeuron(NeuronInput):

    def __init__(self, activation, creature):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param creature: the creature to interact with
        :type creature: ``Codekaryote``
        """
        super().__init__(activation, None)  # Todo make a body module for constant

    # end def __init__

    # -------------------Methods--------------------

    def prepare(self):
        self._inputs = [1, ]
        super().prepare()
    # end def update
# end class ConstantNeuron

class TouchNeuron(NeuronInput):

    def __init__(self, activation, creature):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param creature: the creature to interact with
        :type creature: ``Codekaryote``
        """
        super().__init__(activation, creature.touch)
    # end def __init__

    # -------------------Methods--------------------

    def prepare(self):
        self._inputs = [self._module.touch, ]
        super().prepare()
    # end def update


class TouchForwardNeuron(NeuronInput):
    def __init__(self, activation, creature):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param creature: the creature to interact with
        :type creature: ``Codekaryote``
        """
        super().__init__(activation, creature.touch)
    # end def __init__

    # -------------------Methods--------------------

    def prepare(self):
        self._inputs = [self._module.touch_forward, ]
        super().prepare()
    # end def prepare


# end class MoveUpNeuron