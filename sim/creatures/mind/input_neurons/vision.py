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

    def update(self):
        raise NotImplementedError
    # end def update
# end class VisionNeuron


# noinspection DuplicatedCode
class DistLeft(VisionNeuron):

    def __init__(self, activation, creature):
        super().__init__(activation, creature)

    # end def __init__

    # -------------------Methods--------------------
    
    def update(self):
        self._inputs = [self._module.dist_left, ]
        y = self._activate()
        self._output_to_links(y)
    # end def update
# end class DistLeft


class DistUp(VisionNeuron):

    def __init__(self, activation, creature):
        super().__init__(activation, creature)

    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        self._inputs = [self._module.dist_up, ]
        y = self._activate()
        self._output_to_links(y)
    # end def update
# end class DistUp


# noinspection DuplicatedCode
class DistDown(VisionNeuron):

    def __init__(self, activation, creature):
        super().__init__(activation, creature)

    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        self._inputs = [self._module.dist_down, ]
        y = self._activate()
        self._output_to_links(y)
    # end def update
# end class DistDown


class DistRight(VisionNeuron):

    def __init__(self, activation, creature):
        super().__init__(activation, creature)

    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        self._inputs = [self._module.dist_right, ]
        y = self._activate()
        self._output_to_links(y)
    # end def update
# end class DistRight


class NumForward(VisionNeuron):

    def __init__(self, activation, creature):
        super().__init__(activation, creature)
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        self._inputs = [self._module.num_forward, ]
        y = self._activate()
        self._output_to_links(y)
    # end def update
# end class DistRight
