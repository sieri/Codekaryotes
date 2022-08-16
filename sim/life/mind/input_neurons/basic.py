from sim.life.mind.neuron import NeuronInput

THRESHOlD = 0.5


class ConstantNeuron(NeuronInput):

    def __init__(self, activation, organism):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param organism: the organism to interact with
        :type organism: ``Codekaryote``
        """
        super().__init__(activation, None)  # Todo make a body module for constant

    # end def __init__

    # -------------------Methods--------------------

    def prepare(self):
        self._inputs = [1, ]
        return super().prepare()
    # end def update
# end class ConstantNeuron

class TouchNeuron(NeuronInput):

    def __init__(self, activation, organism):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param organism: the organism to interact with
        :type organism: ``Codekaryote``
        """
        super().__init__(activation, organism.touch)
    # end def __init__

    # -------------------Methods--------------------

    def prepare(self):
        self._inputs = [self._module.touch, ]
        return super().prepare()
    # end def update


class TouchForwardNeuron(NeuronInput):
    def __init__(self, activation, organism):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param organism: the organism to interact with
        :type organism: ``Codekaryote``
        """
        super().__init__(activation, organism.touch)
    # end def __init__

    # -------------------Methods--------------------

    def prepare(self):
        self._inputs = [self._module.touch_forward, ]
        return super().prepare()
    # end def prepare
# end class MoveUpNeuron


class AngleNeuron(NeuronInput):
    def __init__(self, activation, organism):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param organism: the organism to interact with
        :type organism: ``Codekaryote``
        """
        super().__init__(activation, organism.physical_body)
    # end def __init__

    # -------------------Methods--------------------

    def prepare(self):
        self._inputs = [self._module.angle, ]
        return super().prepare()
    # end def prepare
# end class AngleNeuron


class SpeedNeuron(NeuronInput):
    def __init__(self, activation, organism):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param organism: the organism to interact with
        :type organism: ``Codekaryote``
        """
        super().__init__(activation, organism.physical_body)
    # end def __init__

    # -------------------Methods--------------------

    def prepare(self):
        self._inputs = [self._module.velocity.length]
        return super().prepare()
    # end def prepare
# end class SpeedNeuron

class SpeedRotationNeuron(NeuronInput):
    def __init__(self, activation, organism):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param organism: the organism to interact with
        :type organism: ``Codekaryote``
        """
        super().__init__(activation, organism.physical_body)
    # end def __init__

    # -------------------Methods--------------------

    def prepare(self):
        self._inputs = [self._module.angular_velocity]
        return super().prepare()
    # end def prepare
# end class SpeedRotationNeuron