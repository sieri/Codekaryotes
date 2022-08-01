import math
import random

from sim.creatures.codekaryote import Codekaryote
from gui.window import redraw
from sim.Parameters import world as param

class World:
    """
    Contain all the elements that create the sim, for now is a simple 2D grid
    """

    _width = None
    _height = None
    _creatures = []
    _tick_gen = 0

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
    # end def initiate

    def populate_randomly(self, count=10):
        """
        populate the sim by placing creatures randomly

        :param count: number of creatures to place
        :type count: ``int``
        """
        sample = random.sample(range(self.width*self.height), count)
        self._creatures = [Codekaryote(Position.from_index(i)) for i in sample]
    # end def populate_randomly

    def populate_new_generation(self, count=10):
        """
        populate the sim by placing creatures randomly, bringing back the population to the count through mutation of
        the survivors

        :param count: number of creatures to place
        :type count: ``int``
        """
        to_evolve = count - len(self._creatures)
        sample_to_evolve = [random.randint(0, len(self._creatures)-1) for _ in range(to_evolve)]

        new_genome = []
        for i in sample_to_evolve:
            parent = self._creatures[i]
            new_genome.append(parent.reproduce_genome())

        old_genome = [c.genome for c in self._creatures]

        sample_positions = random.sample(range(self.width*self.height), count)

        self._creatures.clear()
        for (pos, genome)  in zip(sample_positions, new_genome+old_genome):
            self._creatures.append(Codekaryote(Position.from_index(pos), genome))

    # end def populate_new_generation

    def is_busy(self, position):
        """
        return true if this position is busy with an element at the moment
        :param position: the position to check
        :type position:
        :return: Flag if it's busy
        :rtype: ``bool``
        """
        for c in self._creatures:
            if c.position == position:
                return True
            # end if
        # end for
        return False
    # end is_busy

    def kill_right_screen(self):
        temp = []
        for c in self._creatures:
            if c.position.x > self._width/2:
                temp.append(c)
        # end for

        self._creatures = temp
    # end def kill_right_screen

    def loop(self):
        for _ in range(param.GENERATION_TIME):
            for c in self._creatures:
                c.update()
            # end for
            redraw(self)
    # end def loop

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
    def creatures(self):
        return self._creatures
    # end def creatures
# end class World


world = World()


class Position:
    """
    A position on the sim for an item
    """

    def __init__(self, x, y):
        """
        :param x: The x coordinate
        :type x: ``ìnt``
        :param y: The x coordinate
        :type y: ``ìnt``
        """
        self._x = x
        self._y = y

    def __eq__(self, other):
        return self._y == other.y and self._x == other.x
    # end def __eq__

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
        return Position(x, y)
    # def from_index

    # -----------------Properties------------------

    @property
    def x(self):
        """
        Getter for x coordinate

        :return: the value of x
        :rtype: ``int``
        """
        return self._x

    # end def x

    @x.setter
    def x(self, val):
        """
        Setter for x coordinate

        :param val: the value to set
        :type val: ``int``
        """
        if val < 0:
            val = 0
        elif val > world.width:
            val = world.width
        elif world.is_busy(Position(self.x+1, self.y)):
            return
        self._x = val

    # end def x

    @property
    def y(self):
        """
        Getter for y coordinate

        :return: the value of y
        :rtype: ``int``
        """
        return self._y

    # end def y

    @y.setter
    def y(self, val):
        """
        Setter for y coordinate

        :param val: the value to set
        :type val: ``int``
        """
        if val < 0:
            val = 0
        elif val > world.height:
            val = world.height
        elif world.is_busy(Position(self.x, self.y+1)):
            return
        self._y = val
    # end def y
# end class Position
