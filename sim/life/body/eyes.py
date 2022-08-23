import itertools
import math

import numpy as np
import pymunk

from sim.life.common.energy import AbstractEnergyConsumer
from sim.world import World
from sim.parameters import body as param
from utils import dist, angle

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
            setattr(s, "color", (255, 255, 255, 128))
            setattr(s,"owner", self)

        setattr(self._organism, "vision_cone", self._shape)


        self._seen_creatures = []
        self._seen_plants = []
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        super().update()

    def reset(self):
        self._seen_creatures.clear()
        self._seen_plants.clear()

    def new_seen_creature(self, creature):
        self._seen_creatures.append(creature)

    def new_seen_plant(self, plant):
        self._seen_plants.append(plant)

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
    def num_seen(self):
        return len(self._seen_creatures)+len(self._seen_plants)
    # end def num_seen

    @property
    def num_seen_creatures(self):
        return len(self._seen_creatures)
    # end def num_seen_creatures

    @property
    def num_seen_plants(self):
        return len(self._seen_plants)
    # end def num_seen_plants


    @property
    def closest_creature_dist(self):
        if self.num_seen_creatures == 0:
            return -1
        elif self.num_seen_creatures == 1:
            return dist(self._seen_creatures[0], self._organism.position)

        dists = map(dist, self._seen_creatures, itertools.repeat(self._organism.position))
        return min(list(dists))
    # end def closest_creature_dist

    
    @property
    def closest_plant_dist(self):
        if self.num_seen_plants == 0:
            return -1
        elif self.num_seen_plants == 1:
            return dist(self._seen_plants[0], self._organism.position)

        dists = map(dist, self._seen_plants, itertools.repeat(self._organism.position))
        return min(list(dists))
    # end def closest_plant_dist


    @property
    def closest_creature_angle(self):
        if self.num_seen_creatures == 0:
            return 0
        elif self.num_seen_creatures == 1:
            return angle(self._seen_creatures[0], self._organism.position) - self._organism.angle

        dists = list(map(dist, self._seen_creatures, itertools.repeat(self._organism.position)))
        min_val = min(dists)
        index = np.where(dists == np.amin(dists))[0][0]
        return angle(self._seen_creatures[index], self._organism.position) - self._organism.angle

    # end def closest_creature_angle

    @property
    def closest_plant_angle(self):
        if self.num_seen_plants == 0:
            return 0
        elif self.num_seen_plants == 1:
            return angle(self._seen_plants[0], self._organism.position) - self._organism.angle

        dists = list(map(dist, self._seen_plants, itertools.repeat(self._organism.position)))
        min_val = min(dists)
        index = np.where(dists == np.amin(dists))[0][0]
        return angle(self._seen_plants[index], self._organism.position) - self._organism.angle

    # end def closest_plant_angle

# end class Eyes
