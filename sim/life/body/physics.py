from sim.life.codekaryote import BaseModule
from sim.life.common.energy import AbstractEnergyConsumer
from sim.world import World
from sim.parameters import body as param
import pymunk as pm

world = World()


class AbstractBody:
    """
    The body
    """
    _size = 0
    _mass = 0
    _inertia = 0
    _body = None
    FACTOR = 4303355903 / ((param.BODY_SIZE_MAX-param.BODY_SIZE_MIN)*10000)

    def _gen(self, genome):
        self._size = ((genome[0] / self.FACTOR) / 10000) + param.BODY_SIZE_MIN
        self._mass = (self._size**2) * param.BODY_MASS_UNIT
        self._inertia = pm.moment_for_circle(self._mass, 0, self._size, (0, 0))
        self._body = pm.Body(self._mass, self._inertia)
    # end def genome

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

        self._gen(genome)
        self._energy_rate = param.ENERGY_SIZE_SCALE * self._mass

        setattr(self._organism, "physical_body", self._body)
        circle = pm.Circle(self._body, self._size, (0, 0))
        setattr(circle, "owner", self)
        setattr(circle, "organism", self._organism)
        setattr(self._organism, "shape", circle)
    # end def __init__

    @property
    def name(self):
        return "body_active_circle"
# end class BodyActiveCircle


class BodyPassiveCircle(AbstractBody, BaseModule):

    def __init__(self, organism, genome):
        super().__init__(organism=organism, genome=genome, name="body")

        self._gen(genome)

        setattr(self._organism, "physical_body", self._body)
        circle = pm.Circle(self._body, self._size, (0, 0))
        setattr(circle, "owner", self)
        setattr(circle, "organism", self._organism)
        setattr(self._organism, "shape", circle)

    # end def __init__

    def update(self):
        pass

    @property
    def name(self):
        return "body_passive_circle"
# end class BodyPassiveCircle


class Movement(AbstractEnergyConsumer):

    def __init__(self, organism, genome):
        super().__init__(organism=organism, genome=genome,
                         passive=False, name="movement")

        self.need_reset = True

        self._forward = 0
        self._torque = 0
        self._energy_rate = param.ENERGY_MOVEMENT_RATE
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        self._active = not (self._forward == 0)
        super().update()
        self.organism.physical_body.apply_force_at_local_point((self._forward,0))
        self.organism.physical_body.torque = self._torque
    # end def update

    def reset(self):
        self._forward = 0
        self._torque = 0

    def move_forward(self, ratio):

        self._forward += ratio
    # end def move_up

    def move_backward(self, ratio):
        self._forward -= ratio
    # end def move_down

    def turn_right(self, ratio):
        #self._forward[0] += ratio
        self._torque += ratio
    # end def move_right

    def turn_left(self, ratio):
        #self._forward[0] -= ratio
        self._torque -= ratio
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

        self.need_reset = True

        self._touch = 0
        self._touch_forward = 0
        self._organism_touching = None
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        pass

    def reset(self):
        self._touch = 0
        self._touch_forward = 0
    # end def reset

    def touch_update(self, other, points):
        self._touch += 1

        for point in points.points:
            # noinspection PyUnresolvedReferences
            local = self._organism.physical_body.world_to_local(point.point_a)
            # TODO manage with rotation
        self._touch_forward = 1
        self._organism_touching = other

    # -----------------Properties------------------

    @property
    def touch(self):
        return self._touch
    # end def touch

    @property
    def touch_forward(self):
        return self._touch_forward
    # end def touch_forward

    @property
    def organism_touching(self):
        return self._organism_touching
    # end def organism_touching

# end class Touch
