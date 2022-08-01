from sim.life.codekaryote import BaseModule
from sim.life.common.energy import AbstractEnergyConsumer
from sim.world import Vector, World, Coordinate, Position
from utils import clamp
from sim.parameters import body as param


class Movement(AbstractEnergyConsumer):

    def __init__(self, organism, genome):
        super().__init__(organism=organism, genome=genome,
                         passive=False, name="movement")

        self._forward = Vector(x=0, y=0)
        self._energy_rate = param.ENERGY_MOVEMENT_RATE
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        self._active = (self._forward == Coordinate(coord=[0, 0]))
        super().update()
        self._forward.clear()
    # end def update

    def move_up(self):
        self._organism.position.y += 1
        self._forward.y += 1
    # end def move_up

    def move_down(self):
        self._organism.position.y -= 1
        self._forward.y -= 1
    # end def move_down

    def move_right(self):
        self._organism.position.x += 1
        self._forward.x += 1
    # end def move_right

    def move_left(self):
        self._organism.position.x -= 1
        self._forward.x -= 1
    # end def move_left

    # -----------------Properties------------------

    @property
    def forward(self):
        return self._forward
    # end def forward
# end class Movement


class Touch(BaseModule):

    def __init__(self, organism, genome):
        super().__init__(organism, genome, "touch")
        self._world = World()
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        pass
    # end def update

    # -----------------Properties------------------

    @property
    def touch(self):
        organisms = self._world.get_local_organisms(self._organism.position, 1)
        return len(organisms)
    # end def touch

    @property
    def touch_forward(self):
        pos = self._organism.position
        # noinspection PyUnresolvedReferences
        fwd = self._organism.movement.forward
        x = pos.x+round(clamp(fwd.x, -1, 1))
        y = pos.y+round(clamp(fwd.y, -1, 1))

        if x >= self._world.width or y >= self._world.width:
            return 0

        return self._world.grid[x, y] >= 0
    # end def touch_forward
# end class Touch


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
                self.organism.reproduce(Position(x=x, y=y))
            # end if
        # end if
    # -----------------Properties------------------


# end class Eating