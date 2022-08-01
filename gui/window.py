import math
import sys

import pygame
from pygame import display, Color
from pygame.time import Clock
from pygame.locals import QUIT
from gui.elements.creatures import draw_creature
from gui.export_video import take_capture, save


class Window:
    def __init__(self, x, y, factor, export_video=False):
        """
        :param x: the width of the grid of the world.py
        :type x: ``int``
        :param y: the height of the gird of the wold
        :type y: ``int``
        :param factor: a factor to scale the world.py
        :type factor: ``float``
        :param export_video: flag if a video need to be exported - OPTIONAL
        :type export_video: ``bool``
        """
        display.set_mode((math.ceil((x+1)*factor), math.ceil((y+1)*factor)))
        self._x = x
        self._y = y
        self._factor = factor
        self._surface = display.get_surface()
        self._clock = Clock()
        self._export_video = export_video
    # def __init__

    def redraw(self, world):
        """
        draw the world.py
        :param world: the world.py to draw
        :type world: ``World``
        """
        for event in pygame.event.get():
            if event.type == QUIT:
                if self._export_video:
                    save()
                pygame.quit()
                sys.exit()

        self._surface.fill(Color(0, 0, 0))

        for c in world.creatures:
            draw_creature(self._surface, c, self._factor)

        display.flip()
        if self._export_video:
            take_capture()
        self._clock.tick(60)
    # end def redraw
# end class Window


win = None


def init(x, y, factor, export_video=False):
    """
    initialize and create a window
    :param x: the width of the grid of the world.py
    :type x: ``int``
    :param y: the height of the gird of the wold
    :type y: ``int``
    :param factor: a factor to scale the world.py
    :type factor: ``float``
    :param export_video: flag if a video need to be exported - OPTIONAL
    :type export_video: ``bool``
    """
    global win
    win = Window(x, y, factor, export_video)
# end def init


def redraw(world):
    """
    draw the world.py
    :param world: the world.py to draw
    :type world: ``World``
    """
    # noinspection PyUnresolvedReferences
    win.redraw(world)
# end def redraw
