from sim.creatures.mind.neuron import NeuronInput


class VisionNeuron(NeuronInput):

    def __init__(self, activation, creature):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param creature: the creature to interact with
        :type creature: ``Codekaryote``
        """
        super().__init__(activation, creature.eyes)
    # end def __init__

    # -------------------Methods--------------------

# end class VisionNeuron


# noinspection DuplicatedCode
class DistLeft(VisionNeuron):

    def __init__(self, activation, creature):
        super().__init__(activation, creature)

    # end def __init__

    # -------------------Methods--------------------
    
    def prepare(self):
        self._inputs = [self._module.dist_left, ]
        super().prepare()
    # end def prepare
# end class DistLeft


class DistUp(VisionNeuron):

    def __init__(self, activation, creature):
        super().__init__(activation, creature)

    # end def __init__

    # -------------------Methods--------------------

    def prepare(self):
        self._inputs = [self._module.dist_up, ]
        super().prepare()
    # end def prepare
# end class DistUp


# noinspection DuplicatedCode
class DistDown(VisionNeuron):

    def __init__(self, activation, creature):
        super().__init__(activation, creature)

    # end def __init__

    # -------------------Methods--------------------

    def prepare(self):
        self._inputs = [self._module.dist_down, ]
        super().prepare()
    # end def prepare
# end class DistDown


class DistRight(VisionNeuron):

    def __init__(self, activation, creature):
        super().__init__(activation, creature)

    # end def __init__

    # -------------------Methods--------------------

    def prepare(self):
        self._inputs = [self._module.dist_right, ]
        super().prepare()
    # end def prepare
# end class DistRight


class NumForward(VisionNeuron):

    def __init__(self, activation, creature):
        super().__init__(activation, creature)
    # end def __init__

    # -------------------Methods--------------------

    def prepare(self):
        self._inputs = [self._module.num_forward, ]
        super().prepare()
    # end def prepare
# end class DistRight
