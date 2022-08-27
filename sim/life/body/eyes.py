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


def method_name(fov,range):
    return []

    #Todo Fix needing organism
    if range == 0:
        range = 0.1
    energy_rate = (fov / 180 * range) * param.ENERGY_EYES_RATE
    if fov <= 90:
        vision_cone_vertex = [(0.0, 0.0)] + \
                             [(range * math.cos(math.radians(i)), range * math.sin(math.radians(i)))
                              for i in np.linspace(-fov / 2, fov / 2, N_SEGMENT_CONE)]


        shape = (pymunk.Poly(organism.physical_body, vision_cone_vertex),)


    elif fov <= 180:
        f = fov / 2
        vision_cone_vertex = [(0.0, 0.0)] + \
                             [(range * math.cos(math.radians(i)), range * math.sin(math.radians(i)))
                              for i in np.linspace(-f / 2, f / 2, N_SEGMENT_CONE)]

        shape = (pymunk.Poly(organism.physical_body, vision_cone_vertex,
                                   transform=pymunk.Transform.rotation(math.radians(f / 2))),
                       pymunk.Poly(organism.physical_body, vision_cone_vertex,
                                   transform=pymunk.Transform.rotation(math.radians(-f / 2))))
    elif fov <= 270:
        f = fov / 3
        vision_cone_vertex = [(0.0, 0.0)] + \
                             [(range * math.cos(math.radians(i)), range * math.sin(math.radians(i)))
                              for i in np.linspace(-f / 2, f / 2, N_SEGMENT_CONE)]

        shape = (pymunk.Poly(organism.physical_body, vision_cone_vertex,
                                   transform=pymunk.Transform.rotation(math.radians(f))),
                       pymunk.Poly(organism.physical_body, vision_cone_vertex),
                       pymunk.Poly(organism.physical_body, vision_cone_vertex,
                                   transform=pymunk.Transform.rotation(math.radians(-f))))
    else:
        f = fov / 4
        vision_cone_vertex = [(0.0, 0.0)] + \
                             [(range * math.cos(math.radians(i)), range * math.sin(math.radians(i)))
                              for i in np.linspace(-f / 2, f / 2, N_SEGMENT_CONE)]

        shape = (pymunk.Poly(organism.physical_body, vision_cone_vertex,
                                   transform=pymunk.Transform.rotation(math.radians(1.5 * f))),
                       pymunk.Poly(organism.physical_body, vision_cone_vertex,
                                   transform=pymunk.Transform.rotation(math.radians(f / 2))),
                       pymunk.Poly(organism.physical_body, vision_cone_vertex,
                                   transform=pymunk.Transform.rotation(math.radians(-f / 2))),
                       pymunk.Poly(organism.physical_body, vision_cone_vertex,
                                   transform=pymunk.Transform.rotation(math.radians(-1.5 * f))))
    for s in shape:
        s.collision_type = 3
        s.sensor = True
        setattr(s, "color", (255, 255, 255, 128))

    return shape


