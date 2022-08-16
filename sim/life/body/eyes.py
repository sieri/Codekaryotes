import math

import numpy as np
import pymunk

from sim.life.common.energy import AbstractEnergyConsumer
from sim.world import World
from sim.parameters import body as param

world = World()

N_SEGMENT_CONE = 5

class Eyes(AbstractEnergyConsumer):

    def __init__(self, organism, genome):
        super().__init__(organism=organism, genome=genome,
                         passive=True, name="eyes")

        # initialize from the genome
        self._fov = genome[0] % 360
        self._range = genome[1] % param.EYE_RANGE_LIMIT
        if self._range == 0:
            self._range = 0.1

        self._energy_rate = (self._fov/180*self._range) * param.ENERGY_EYES_RATE

        if self._fov <= 90:
            vision_cone_vertex = [(0.0, 0.0)]+\
                                 [(self._range * math.cos(math.radians(i)), self._range * math.sin(math.radians(i)))
                                  for i in np.linspace(-self._fov/2,self._fov/2, N_SEGMENT_CONE)]

            self._shape = (pymunk.Poly(organism.physical_body, vision_cone_vertex),)


        elif self._fov <= 180:
            f = self._fov/2
            vision_cone_vertex = [(0.0, 0.0)] + \
                                 [(self._range * math.cos(math.radians(i)), self._range * math.sin(math.radians(i)))
                                  for i in np.linspace(-f / 2, f / 2, N_SEGMENT_CONE)]

            self._shape = (pymunk.Poly(organism.physical_body, vision_cone_vertex, transform=pymunk.Transform.rotation(math.radians(f/2))),
                          pymunk.Poly(organism.physical_body, vision_cone_vertex, transform=pymunk.Transform.rotation(math.radians(-f/2))))
        elif self._fov <= 270:
            f = self._fov/3
            vision_cone_vertex = [(0.0, 0.0)] + \
                                 [(self._range * math.cos(math.radians(i)), self._range * math.sin(math.radians(i)))
                                  for i in np.linspace(-f / 2, f / 2, N_SEGMENT_CONE)]

            self._shape = (pymunk.Poly(organism.physical_body, vision_cone_vertex, transform=pymunk.Transform.rotation(math.radians(f))),
                           pymunk.Poly(organism.physical_body, vision_cone_vertex),
                          pymunk.Poly(organism.physical_body, vision_cone_vertex, transform=pymunk.Transform.rotation(math.radians(-f))))
        else:
            f = self._fov/4
            vision_cone_vertex = [(0.0, 0.0)] + \
                                 [(self._range * math.cos(math.radians(i)), self._range * math.sin(math.radians(i)))
                                  for i in np.linspace(-f / 2, f / 2, N_SEGMENT_CONE)]

            self._shape = (pymunk.Poly(organism.physical_body, vision_cone_vertex, transform=pymunk.Transform.rotation(math.radians(1.5*f))),
                           pymunk.Poly(organism.physical_body, vision_cone_vertex, transform=pymunk.Transform.rotation(math.radians(f/2))),
                           pymunk.Poly(organism.physical_body, vision_cone_vertex, transform=pymunk.Transform.rotation(math.radians(-f/2))),
                           pymunk.Poly(organism.physical_body, vision_cone_vertex, transform=pymunk.Transform.rotation(math.radians(-1.5*f))))



        for s in self._shape:
            s.collision_type = 3
            s.sensor = True
            s.__setattr__("color", (255, 255, 255, 128))

        setattr(self._organism, "vision_cone", self._shape)




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
