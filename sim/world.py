import math
import random
import sys
from concurrent import futures

import numpy as np
from more_itertools import grouper

from sim.life.codekaryote import Codekaryote
from gui.window import redraw
from sim.parameters import world as param


class World:
    """
    Contain all the elements that create the sim, for now is a simple 2D grid
    """

    _width = None
    _height = None
    _creature = []
    _plant = []
    _tick_gen = 0
    _grid = np.array((0, 0))
    _executor = futures.ProcessPoolExecutor(12)
    _generation = 0
    _to_remove_creature = []
    _to_remove_plant = []
    _to_add_creature = []
    _to_add_plant = []
    _plant_cycle = 0
    _oldest = 0

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
        self._grid = np.empty((self._width, self._height), dtype=np.int)
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
        self._creature += [Codekaryote(Position.from_index(i)) for i in sample[:count_creature]]
        self._plant += [Codekaryote(Position.from_index(i), genome_generator=generate_random_plant_genome) for i in sample[count_creature:]]
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
            sample_to_evolve = [random.randint(0, len(self._organisms)-1) for _ in range(to_evolve)]
        else:
            print("Extinction Event")
            sys.exit()

        new_genome = []
        for i in sample_to_evolve:
            parent = self._creature[i]
            new_genome.append(parent.reproduce_genome())

        old_genome = [c.genome for c in self._creature]

        sample_positions = random.sample(range(self.width*self.height), count)

        self._creature.clear()
        for (pos, genome) in zip(sample_positions, new_genome+old_genome):
            self._creature.append(Codekaryote(Position.from_index(pos), genome))

    # end def populate_new_generation

    def is_busy(self, position):
        """
        return true if this position is busy with an element at the moment
        :param position: the position to check
        :type position:
        :return: Flag if it's busy
        :rtype: ``bool``
        """
        return world.grid[position.x, position.y] != -1
    # end is_busy

    def get_local_organisms(self, pos, r):
        organisms_zone = self._grid[pos.x - r:pos.x + r, pos.y - r:pos.y + r]
        organisms = organisms_zone[np.where(organisms_zone >= 0)]
        return organisms
    # end def get_local_organisms

    def kill_right_screen(self):
        temp = []
        for c in self._creature:
            if c.position.x > self._width/2:
                temp.append(c)
        # end for

        self._creature = temp

    # end def kill_right_screen

    def build_grid(self):
        # build grid
        self._grid.fill(-1)
        for i, c in enumerate(self.organisms):
            self._grid[c.position.x, c.position.y] = i
        # end for
    # end def build_grid

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
        self.build_grid()
        for c in self.organisms:
            c.update()

        for remove in self._to_remove_creature:
            self._creature.remove(remove)
        self._to_remove_creature.clear()

        for remove in self._to_remove_plant:
            self._plant.remove(remove)
        self._to_remove_plant.clear()

        self._creature += self._to_add_creature
        self._to_add_creature.clear()
        self._plant += self._to_add_plant
        self._to_add_plant.clear()

        if param.CHEAT_ANTI_EXTINCTION:
            if len(self._creature) < param.ANTI_EXTINCTION_THRESHOLD:
                self.populate_randomly(param.ANTI_EXTINCTION_BONCE_BACK - len(self._creature), count_plant=0)

        if param.PLANT_SPAWN:
            self._plant_cycle += 1
            if self._plant_cycle >= param.PLANT_CYCLE:
                self._plant_cycle = 0
                self.populate_randomly(count_creature=0, count_plant=param.PLANT_SPAWN)

        print(f"end tick: creatures={len(self._creature)}, plants={len(self._plant)} || oldest generation {self._oldest}")
        redraw(self)
    # end def loop_iteration

    def remove_organism(self, organism):
        if hasattr(organism,  "movement"):
            self._to_remove_creature.append(organism)
            if organism.ancestry.generation > self._oldest:
                self._oldest = organism.ancestry.generation
            # end def
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
    def grid(self):
        return self._grid
    # end def grid

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
# end class World


world = World()


class Coordinate:
    """
    Base class for anything dealing with coordinate
    """
    def __init__(self, **kwargs):
        self._coord = np.array([0, 0])

        if "coord" in kwargs:
            self._coord[0] = kwargs["coord"][0]
            self._coord[1] = kwargs["coord"][1]
        else:
            self._coord[0] = kwargs["x"]
            self._coord[1] = kwargs["y"]
        # end if
    # end def __init__

    def __eq__(self, other):
        return (self._coord == other.coord).all()
    # end def __eq__

    def __sub__(self, other):
        return self._coord - other.coord
    # end def __sub__

    def __add__(self, other):
        return self._coord + other.coord
    # end def __add__

    def __mul__(self, other):
        if isinstance(other, Coordinate):
            return self.__class__(coord=self._coord * other.coord)
        else:
            return self.__class__(coord=self._coord * other)
    # end def __mul__

    def __repr__(self):
        return f"{self.__class__.__name__}({self._coord})"
    # end def __repr__

    @property
    def coord(self):
        return self._coord
    # end def coord

    @property
    def x(self):
        """
        Getter for x coordinate

        :return: the value of x
        :rtype: ``int``
        """
        return self._coord[0]

    @x.setter
    def x(self, val):
        """
        Setter for x coordinate

        :param val: the value to set
        :type val: ``int``
        """
        self._coord[0] = val

    @property
    def y(self):
        """
        Getter for y coordinate

        :return: the value of y
        :rtype: ``int``
        """
        return self._coord[1]
    # end def y

    @y.setter
    def y(self, val):
        """
        Setter for y coordinate

        :param val: the value to set
        :type val: ``int``
        """
        self._coord[1] = val
    # end def y
# end class Coordinate


class Position(Coordinate):
    """
    A position on the sim for an item
    """

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
    
    # -------------------Methods--------------------

    @classmethod
    def from_index(cls, index):
        """
        create a new position from an index in the current sim
        :param index: the index of the position
        :type index: ``int``
        :return: the new position
        :rtype: ``Position``
        """
        x = index % world.width
        y = math.floor(index / world.width)
        return Position(x=x, y=y)
    # def from_index

    def dist(self, other):
        """
        return the distance between two positions
        :param other: the other positions
        :type other: ``Position``
        :return: the distance
        :rtype: ``int``
        """
        square_x = (self.x - other.x)**2
        square_y = (self.y - other.y)**2
        
        return math.sqrt(square_x+square_y)
    # end def dist

    # -----------------Properties------------------

    @Coordinate.x.setter
    def x(self, val):
        """
        Setter for x coordinate

        :param val: the value to set
        :type val: ``int``
        """
        if val < 0:
            val = 0
        elif val >= world.width:
            val = world.width-1
        elif world.is_busy(Position(x=val, y=self.y)):
            return
        self._coord[0] = val

    @Coordinate.y.setter
    def y(self, val):
        """
        Setter for y coordinate

        :param val: the value to set
        :type val: ``int``
        """
        if val < 0:
            val = 0
        elif val >= world.height:
            val = world.height-1
        elif world.is_busy(Position(x=self.x, y=val)):
            return
        self._coord[1] = val
    # end def y
# end class Position


class Vector(Coordinate):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
    # end def __init__

    # -------------------Methods--------------------

    def clear(self):
        self._coord = np.empty(2)

    def angle_with(self, origin, other):
        """
        calculate the angle

        :param origin: origin of the current vector
        :type origin: ``Position``
        :param other: other position
        :type other: ``Position``
        :return: the angle
        :rtype: ``float``
        """
        v0 = self - origin
        v1 = self - other

        angle = np.math.atan2(np.linalg.det([v0, v1]), np.dot(v0, v1))
        return np.degrees(angle)
    # -----------------Properties------------------
# end class Vector
