from sim.life.mind.neuron import NeuronInput
from sim.world import World

world = World()
class VisionNeuron(NeuronInput):

    def __init__(self, activation, organism):
        """
        :param activation: the activation function applied in this specific neuron
        :type activation: ``Activations``
        :param organism: the organism to interact with
        :type organism: ``Codekaryote``
        """
        super().__init__(activation, organism.eyes)
    # end def __init__

    # -------------------Methods--------------------

# end class VisionNeuron


# noinspection DuplicatedCode
class DistLeft(VisionNeuron):

    def __init__(self, activation, organism):
        super().__init__(activation, organism)

    # end def __init__

    # -------------------Methods--------------------
    
    def prepare(self):
        self._inputs = [self._module.dist_left/world.width, ]
        return super().prepare()
    # end def prepare
# end class DistLeft


class DistUp(VisionNeuron):

    def __init__(self, activation, organism):
        super().__init__(activation, organism)

    # end def __init__

    # -------------------Methods--------------------

    def prepare(self):
        self._inputs = [self._module.dist_up/world.height, ]
        return super().prepare()
    # end def prepare
# end class DistUp


# noinspection DuplicatedCode
class DistDown(VisionNeuron):

    def __init__(self, activation, organism):
        super().__init__(activation, organism)

    # end def __init__

    # -------------------Methods--------------------

    def prepare(self):
        self._inputs = [self._module.dist_down/world.height, ]
        return super().prepare()
    # end def prepare
# end class DistDown


class DistRight(VisionNeuron):

    def __init__(self, activation, organism):
        super().__init__(activation, organism)

    # end def __init__

    # -------------------Methods--------------------

    def prepare(self):
        self._inputs = [self._module.dist_right/world.width, ]
        return super().prepare()
    # end def prepare
# end class DistRight


class NumForward(VisionNeuron):

    def __init__(self, activation, organism):
        super().__init__(activation, organism)
    # end def __init__

    # -------------------Methods--------------------

    def prepare(self):
        self._inputs = [self._module.num_forward, ]
        return super().prepare()
    # end def prepare
# end class DistRight
