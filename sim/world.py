import math
import random
import sys
from concurrent import futures

import pymunk as pm
import numpy as np

from sim.life.codekaryote import Codekaryote
from gui.window import redraw, end
from sim.parameters import world as param


class World:
    """
    Contain all the elements that create the sim, for now is a simple 2D grid
    """

    _width = None
    _height = None
    _creature = []
    _plant = []
    _generation = 0

    _tick_gen = 0
    _executor = futures.ProcessPoolExecutor(12)
    _to_remove_creature = []
    _to_remove_plant = []
    _to_add_creature = []
    _to_add_plant = []
    _plant_shape = {}
    _creature_shape = {}
    _plant_cycle = 0
    _space = None
    _dt = 1.0 / 60.0
    _ch = None
    _constraints = None

    def __new__(cls, *args, **kwargs):
        if not hasattr(cls, 'instance'):
            cls.instance = super().__new__(cls)
        return cls.instance


    # -------------------Methods--------------------

    def initiate(self, width=10, height=10):
        """
        :param width: number of x squares
        :type width: ``int``
        :param height: number of y squares
        :type height: ``int``
        """
        self._width = width
        self._height = height
        self._tick_gen = 0
        self._space = pm.Space()
        self._space.gravity = (0, 0)
        self._space.damping = param.DAMPENING
        self._ch = self._space.add_collision_handler(0, 0)
        self._ch_eyes = self._space.add_collision_handler(3, 0)
        self._ch.post_solve = collision_post_resolve
        self._ch_eyes.begin = eyes_handler

        # wall
        static_lines = [
            pm.Segment(self._space.static_body, (0.0, 0.0), (self._width, 0.0), 0.0),
            pm.Segment(self._space.static_body, (self._width, 0.0), (self._width, self._height), 0.0),
            pm.Segment(self._space.static_body, (self._width, self._height), (0.0, self._height), 0.0),
            pm.Segment(self._space.static_body, (0.0, self._height), (0.0, 0.0), 0.0),

        ]

        for l in static_lines:
            l.collision_type = 1

        self._space.add(*static_lines)
    # end def initiate

    def populate_randomly(self, count_creature=10, count_plant=10):
        """
        populate the sim by placing organisms randomly

        :param count_creature: number of creature to place - OPTIONAL
        :type count_creature: ``int``
        :param count_plant: number of plant to place - OPTIONAL
        :type count_plant: ``int``
        """
        from sim.life.modules import generate_random_plant_genome

        sample = random.sample(range(self.width*self.height), count_creature+count_plant)
        for i in sample[:count_creature]:
            self.add_organism(Codekaryote(self.pos_from_index(i)))
        for i in sample[count_creature:]:
            self.add_organism(Codekaryote(self.pos_from_index(i), genome_generator=generate_random_plant_genome))
    # end def populate_randomly

    def populate_new_generation(self, count=10):
        """
        populate the sim by placing organisms randomly, bringing back the population to the count through mutation of
        the survivors

        :param count: number of organisms to place
        :type count: ``int``
        """
        to_evolve = count - len(self._creature)

        if len(self._creature) > 0:
            sample_to_evolve = [random.randint(0, len(self._creature)-1) for _ in range(to_evolve)]
        else:
            print("Extinction Event")
            end()
            sys.exit()

        new_genome = []
        for i in sample_to_evolve:
            parent = self._creature[i]
            new_genome.append(parent.reproduce_genome())

        old_genome = [c.genome for c in self._creature]

        sample_positions = random.sample(range(self.width*self.height), count)

        self._creature.clear()
        for (pos, genome) in zip(sample_positions, new_genome+old_genome):
            self.add_organism(Codekaryote(self.pos_from_index(pos), genome))

    # end def populate_new_generation

    @staticmethod
    def pos_from_index(index):
        """
        create a new position from an index in the current sim
        :param index: the index of the position
        :type index: ``int``
        :return: the new position
        :rtype: ``tuple(float, float)``
        """
        x = index % world.width
        y = math.floor(index / world.width)
        return x, y
    # def from_index

    def kill_right_screen(self):
        temp = []
        for c in self._creature:
            if c.position.x > self._width/2:
                temp.append(c)
        # end for

        self._creature = temp

    # end def kill_right_screen

    def loop_generation(self):
        print(f"generation: {self._generation}")
        for _ in range(param.GENERATION_TIME):
            self.loop_iteration()
        # end for
        self._generation += 1
    # end def loop_generation

    def loop_infinite(self):
        while True:
            self.loop_iteration()
        # end while
    # end def loop_infinite

    def loop_iteration(self):
        for c in self.creature:
            c.update()

        for remove in self._to_remove_creature:
            self._creature.remove(remove)
            self._space.remove(remove.physical_body, remove.shape, *remove.vision_cone)
            self._creature_shape.pop(remove.shape)
        self._to_remove_creature.clear()

        for remove in self._to_remove_plant:
            self._plant.remove(remove)
            self._space.remove(remove.physical_body, remove.shape)
            self._plant_shape.pop(remove.shape)
        self._to_remove_plant.clear()

        for add in self._to_add_creature:
            self._space.add(add.physical_body, add.shape, *add.vision_cone)
            self._creature_shape[add.shape] = add
        self._creature += self._to_add_creature
        self._to_add_creature.clear()

        for add in self._to_add_plant:
            self._space.add(add.physical_body, add.shape)
            self._plant_shape[add.shape] = add
        self._plant += self._to_add_plant
        self._to_add_plant.clear()

        if param.CHEAT_ANTI_EXTINCTION:
            if len(self._creature) < param.ANTI_EXTINCTION_THRESHOLD:
                if param.ANTI_EXTINCTION_BONCE_BACK_WITH_SURVIVOR:
                    self.populate_new_generation(param.ANTI_EXTINCTION_BONCE_BACK - len(self._creature))
                else:
                    self.populate_randomly(param.ANTI_EXTINCTION_BONCE_BACK - len(self._creature), count_plant=0)
        elif len(self._creature) == 0:
            print("Extinction Event")
            end()
            sys.exit()

        if param.PLANT_SPAWN:
            self._plant_cycle += 1
            if self._plant_cycle >= param.PLANT_CYCLE:
                self._plant_cycle = 0
                self.populate_randomly(count_creature=0, count_plant=param.PLANT_RATE)

        self._space.step(self._dt)

        redraw(self)
    # end def loop_iteration

    def remove_organism(self, organism):
        if hasattr(organism,  "movement"):
            self._to_remove_creature.append(organism)
        else:
            self._to_remove_plant.append(organism)
    # end def remove_organism

    def add_organism(self, organism):
        if hasattr(organism, "movement"):
            self._to_add_creature.append(organism)
        else:
            self._to_add_plant.append(organism)
    # end def add_organism

    # -----------------Properties------------------

    @property
    def width(self):
        """
        getter for the width of the sim
        :return: the width
        :rtype: ``int``
        """
        return self._width
    # end def width

    @property
    def height(self):
        """
        getter for the height of the sim
        :return: the height
        :rtype: ``int``
        """
        return self._height
    # end def height

    @property
    def organisms(self):
        return self._creature+self._plant
    # end def organisms

    @property
    def creature(self):
        return self._creature
    # end def creature

    @property
    def plant(self):
        return self._plant
    # end def plant
    
    @property
    def creature_shape(self):
        return self._creature_shape
    # end def creature_shape
    
    @property
    def plant_shape(self):
        return self._plant_shape
    # end def plant_shape
    
# end class World


# noinspection PyUnusedLocal
def collision_post_resolve(arbiter, space, data):
    shapes = arbiter.shapes
    organism = [None, None]
    creature = [bool, bool]

    for i, shape in enumerate(shapes):
        if shape in world.plant_shape:
            organism[i] = world.plant_shape[shape]
            creature[i] = False
        elif shape in world.creature_shape:
            organism[i] = world.creature_shape[shape]
            creature[i] = True

    if None not in organism:
        if (not creature[0]) and creature[1]:
            organism[1].touch.touch_update(organism[0], points=arbiter.contact_point_set)
        elif (creature[0]) and (not creature[1]):
            organism[0].touch.touch_update(organism[1], points=arbiter.contact_point_set)
        elif creature[0] and creature[1]:
            organism[0].touch.touch_update(organism[1], points=arbiter.contact_point_set)
            organism[1].touch.touch_update(organism[0], points=arbiter.contact_point_set)

# noinspection PyUnusedLocal
def eyes_handler(arbiter, space, data):

    shapes = arbiter.shapes
    organism = None
    creature = False

    if shapes[1] in world.plant_shape:
        organism = world.plant_shape[shapes[1]]
        creature = False
    elif shapes[1] in world.creature_shape:
        organism = world.creature_shape[shapes[1]]
        creature = True

    if organism is not None:
        if creature:
            shapes[0].owner.new_seen_creature(shapes[1].organism)
        else:
            shapes[0].owner.new_seen_plant(shapes[1].organism)

    return True


world = World()
