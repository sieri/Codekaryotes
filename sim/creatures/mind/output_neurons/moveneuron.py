from sim.creatures.mind.neuron import NeuronExit

THRESHOlD = 0.5


class MoveUpNeuron(NeuronExit):

    def __init__(self, activation, creature):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param creature: the creature to interact with
        :type creature: ``Codekaryote``
        """
        super().__init__(activation, creature.movement_module)

    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        y = self._interface_output[self._interface_index]

        if y > THRESHOlD:
            self._module.move_up()
    # end def update
# end class MoveUpNeuron


class MoveDownNeuron(NeuronExit):

    def __init__(self, activation, creature):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param creature: the creature to interact with
        :type creature: ``Codekaryote``
        """
        super().__init__(activation, creature.movement_module)

    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        y = self._interface_output[self._interface_index]

        if y > THRESHOlD:
            self._module.move_down()
    # end def update
# end class MoveDownNeuron


class MoveRightNeuron(NeuronExit):

    def __init__(self, activation, creature):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param creature: the creature to interact with
        :type creature: ``Codekaryote``
        """
        super().__init__(activation, creature.movement_module)

    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        y = self._interface_output[self._interface_index]

        if y > THRESHOlD:
            self._module.move_right()
    # end def update
# end class MoveRightNeuron


class MoveLeftNeuron(NeuronExit):

    def __init__(self, activation, creature):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param creature: the creature to interact with
        :type creature: ``Codekaryote``
        """
        super().__init__(activation, creature.movement_module)

    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        y = self._interface_output[self._interface_index]

        if y > THRESHOlD:
            self._module.move_left()
    # end def update
# end class MoveLeftNeuron
