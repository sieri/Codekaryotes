from sim.creatures.codekaryote import BaseModule
from sim.world import Vector, World
from utils import clamp


class Movement(BaseModule):

    def __init__(self, creature, genome):
        super().__init__(creature, genome, "movement")
        creature.__setattr__("movement_module", self)
        self._forward = Vector(x=0, y=0)
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        self._forward.clear()
    # end def update

    def move_up(self):
        self._creature.position.y += 1
        self._forward.y += 1
    # end def move_up

    def move_down(self):
        self._creature.position.y -= 1
        self._forward.y -= 1
    # end def move_down

    def move_right(self):
        self._creature.position.x += 1
        self._forward.x += 1
    # end def move_right

    def move_left(self):
        self._creature.position.x -= 1
        self._forward.x -= 1
    # end def move_left

    # -----------------Properties------------------

    @property
    def forward(self):
        return self._forward
    # end def forward
# end class Movement


class Touch(BaseModule):

    def __init__(self, creature, genome):
        super().__init__(creature, genome, "touch")
        creature.__setattr__("touch", self)
        self._world = World()
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        pass
    # end def update

    # -----------------Properties------------------

    @property
    def touch(self):
        creatures = self._world.get_local_creatures(self._creature.position, 1)
        return len(creatures)
    # end def touch

    @property
    def touch_forward(self):
        pos = self._creature.position
        # noinspection PyUnresolvedReferences
        fwd = self._creature.movement_module.forward
        x = pos.x+round(clamp(fwd.x, -1, 1))
        y = pos.y+round(clamp(fwd.y, -1, 1))

        if x >= self._world.width or y >= self._world.width:
            return 0

        return self._world.grid[x, y] >= 0
    # end def touch_forward
# end class Movement
