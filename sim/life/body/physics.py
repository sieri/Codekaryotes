from sim.life.codekaryote import BaseModule
from sim.life.common.energy import AbstractEnergyConsumer
from sim.world import World
from utils import clamp
from sim.parameters import body as param
import pymunk as pm


class AbstractBody:
    """
    The body
    """
    _size = 0
    _mass = 0
    _inertia = 0
    _body = None

    # -----------------Properties------------------

    @property
    def mass(self):
        return self._mass
    # end def mass

    @property
    def size(self):
        return self._size
    # end def size
    
    @property
    def inertia(self):
        return self._inertia
    # end def inertia

    @property
    def body(self):
        return self._body
    # end def body
# end class AbstractBody


class BodyActiveCircle(AbstractBody, AbstractEnergyConsumer):

    def __init__(self, organism, genome):
        super().__init__(organism=organism, genome=genome, passive=True, name="body")

        self._size = genome[0]

        self._mass = (self._size**2) * param.MASS_UNIT_BODY
        self._inertia = pm.moment_for_circle(self._mass, 0, self._size, (0, 0))
        self._energy_rate = param.ENERGY_SIZE_SCALE * self._mass
        self._body = pm.Body(self._mass, self._inertia)

        setattr(self._organism, "physical_body", self._body)
        setattr(self._organism, "shape", pm.Circle(self._body, self._size, (0, 0)))
    # end def __init__
# end class BodyActiveCircle


class BodyPassiveCircle(AbstractBody, BaseModule):

    def __init__(self, organism, genome):
        super().__init__(organism=organism, genome=genome, name="body")

        self._size = genome[0]

        self._mass = (self._size**2) * param.MASS_UNIT_BODY
        self._inertia = pm.moment_for_circle(self._mass, 0, self._size, (0, 0))
        self._body = pm.Body(self._mass, self._inertia)

        setattr(self._organism, "physical_body", self._body)
        setattr(self._organism, "shape", pm.Circle(self._body, self._size, (0, 0)))

    # end def __init__

    def update(self):
        pass

# end class BodyPassiveCircle


class Movement(AbstractEnergyConsumer):

    def __init__(self, organism, genome):
        super().__init__(organism=organism, genome=genome,
                         passive=False, name="movement")

        self._forward = [0, 0]
        self._energy_rate = param.ENERGY_MOVEMENT_RATE
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        self._active = (self._forward == [0, 0])
        super().update()
        self.organism.physical_body.apply_force_at_local_point(self._forward)

    # end def update

    def move_up(self, ratio):
        self._forward[1] += ratio
    # end def move_up

    def move_down(self, ratio):
        self._forward[1] -= ratio
    # end def move_down

    def move_right(self, ratio):
        self._forward[0] += ratio
    # end def move_right

    def move_left(self, ratio):
        self._forward[0] -= ratio
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
        return False # TODO enable again
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
