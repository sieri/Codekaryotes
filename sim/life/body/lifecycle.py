from sim.life.codekaryote import BaseModule
from sim.world import World
from utils import clamp


class Eating(BaseModule):

    def __init__(self, organism, genome):
        super().__init__(organism=organism, genome=genome, name="eating")
        self._world = World()
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        pos = self._organism.position
        # noinspection PyUnresolvedReferences
        touch = self._organism.touch.touch_forward

        if touch:
            # noinspection PyUnresolvedReferences
            other = self._organism.touch.organism_touching
            if hasattr(other, "energy_source"):
                # noinspection PyUnresolvedReferences
                self._organism.energy_storage.current_energy += other.energy_source.energy
                other.die()

                self.organism.reproduce((pos.x, pos.y))
            # end if

    # -----------------Properties------------------
# end class Eating
