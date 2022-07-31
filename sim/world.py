import math
import random

from sim.creatures.codekaryote import Codekaryote
from gui.window import redraw


class World:
    """
    Contain all the elements that create the sim, for now is a simple 2D grid
    """

    def __init__(self):
        self._width = None
        self._height = None
        self._creatures = []

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

    def loop(self):
        while True:
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
        self.x = x
        self.y = y

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
        self._y = val
    # end def y
# end class Position
