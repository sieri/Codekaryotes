import numpy as np

from sim.creatures.codekaryote import BaseModule
from sim.world import World
from sim.Parameters import body as param

class Eyes(BaseModule):

    def __init__(self, creature, genome):
        super().__init__(creature, genome, "eyes")
        creature.__setattr__("eyes", self)
        self._world = World()

        # initialize from the genome
        self._fov = genome[0] % 360
        self._range = genome[0] % param.EYE_RANGE_LIMIT
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        pass

    # -----------------Properties------------------

    @property
    def dist_left(self):
        return self._creature.position.x
    # end def dist_left

    @property
    def dist_right(self):
        return self._world.width - self._creature.position.x
    # end def dist_right

    @property
    def dist_down(self):
        return self._creature.position.y
    # end def dist_bottom

    @property
    def dist_up(self):
        return self._world.height - self._creature.position.y
    # end def dist_up

    @property
    def num_forward(self):
        count = 0
        pos = self._creature.position

        # get from the distance
        creatures = self._world.get_local_creatures(pos, self._range)
        for c_index in creatures:
            c = self._world.creatures[c_index]
            # noinspection PyUnresolvedReferences
            angle = self._creature.movement_module.forward.angle_with(self.creature.position, c.position)
            if abs(angle) < self._fov/2:
                count += 1
        return count
    # end def num_forward
# end class Eyes
