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

    def update(self):
        self._output_to_links(1)
    # end def update


# end class MoveUpNeuron