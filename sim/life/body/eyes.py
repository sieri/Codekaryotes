from sim.life.common.energy import AbstractEnergyConsumer
from sim.world import World
from sim.parameters import body as param

world = World()

class Eyes(AbstractEnergyConsumer):

    def __init__(self, organism, genome):
        super().__init__(organism=organism, genome=genome,
                         passive=True, name="eyes")

        # initialize from the genome
        self._fov = genome[0] % 360
        self._range = genome[0] % param.EYE_RANGE_LIMIT

        self._energy_rate = (self._fov/180*self._range) * param.ENERGY_EYES_RATE
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        super().update()

    # -----------------Properties------------------

    @property
    def dist_left(self):
        return self._organism.position.x
    # end def dist_left

    @property
    def dist_right(self):
        return world.width - self._organism.position.x
    # end def dist_right

    @property
    def dist_down(self):
        return self._organism.position.y
    # end def dist_bottom

    @property
    def dist_up(self):
        return world.height - self._organism.position.y
    # end def dist_up

    @property
    def num_forward(self):
        return 0 # TODO renenable
        count = 0
        pos = self._organism.position

        # get from the distance
        organisms = world.get_local_organisms(pos, self._range)
        for c_index in organisms:
            c = world.organisms[c_index]
            # noinspection PyUnresolvedReferences
            angle = self._organism.movement.forward.angle_with(self.organism.position, c.position)
            if abs(angle) < self._fov/2:
                count += 1
        return count
    # end def num_forward
# end class Eyes
