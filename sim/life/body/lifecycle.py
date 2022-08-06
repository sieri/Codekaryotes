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
        fwd = self._organism.movement.forward

        return # Todo Reactivate

        x = pos.x+round(clamp(fwd.x, -1, 1))
        y = pos.y+round(clamp(fwd.y, -1, 1))

        if x >= self._world.width or y >= self._world.width:
            return
        # end if
        if self._world.grid[x, y] >= 0:
            other = self._world.organisms[self._world.grid[x, y]]
            if hasattr(other, "energy_source"):
                # noinspection PyUnresolvedReferences
                self._organism.energy_storage.current_energy += other.energy_source.energy
                other.die()

                x = pos.x - round(clamp(fwd.x, -1, 1))
                y = pos.y - round(clamp(fwd.y, -1, 1))
                self.organism.reproduce((x, y))
            # end if
        # end if
    # -----------------Properties------------------
# end class Eating
