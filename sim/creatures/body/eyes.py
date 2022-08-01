from sim.creatures.codekaryote import BaseModule
from sim.world import World


class Eyes(BaseModule):

    def __init__(self, creature, genome):
        super().__init__(creature, genome, "eyes")
        creature.__setattr__("eyes", self)
        self._world = World()
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
# end class Eyes
