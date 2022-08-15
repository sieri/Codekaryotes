from sim.life.mind.neuron import NeuronExit
from utils import clamp

class MoveUpNeuron(NeuronExit):

    def __init__(self, activation, organism):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param organism: the organism to interact with
        :type organism: ``Codekaryote``
        """
        super().__init__(activation, organism.movement)

    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        y = self._interface_output[self._interface_index]

        self._module.move_up(clamp(y, 0, 1))

    # end def update
    def output(self, value):
        y = clamp(value, 0, 1)

        self._module.move_up(clamp(y, 0, 1))
# end class MoveUpNeuron


class MoveDownNeuron(NeuronExit):

    def __init__(self, activation, organism):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param organism: the organism to interact with
        :type organism: ``Codekaryote``
        """
        super().__init__(activation, organism.movement)

    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        y = self._interface_output[self._interface_index]
        
        self._module.move_down(clamp(y, 0, 1))

    def output(self, value):
        y = clamp(value, 0, 1)

        self._module.move_down(clamp(y, 0, 1))

# end class MoveDownNeuron


class MoveRightNeuron(NeuronExit):

    def __init__(self, activation, organism):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param organism: the organism to interact with
        :type organism: ``Codekaryote``
        """
        super().__init__(activation, organism.movement)

    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        y = self._interface_output[self._interface_index]

        self._module.move_right(clamp(y, 0, 1))
    # end def update

    def output(self, value):
        y = clamp(value, 0, 1)

        self._module.move_right(clamp(y, 0, 1))
# end class MoveRightNeuron


class MoveLeftNeuron(NeuronExit):

    def __init__(self, activation, organism):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param organism: the organism to interact with
        :type organism: ``Codekaryote``
        """
        super().__init__(activation, organism.movement)

    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        y = self._interface_output[self._interface_index]

        self._module.move_left(clamp(y, 0, 1))

    def output(self, value):
        y = clamp(value, 0, 1)

        self._module.move_left(clamp(y, 0, 1))
    # end def update
# end class MoveLeftNeuron
