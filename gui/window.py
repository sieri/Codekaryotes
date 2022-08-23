import math
import sys

import pygame
import pymunk
from pymunk import pygame_util
from pygame import display, Color
from pygame.time import Clock
from pygame.locals import QUIT
from gui.elements.organisms import draw_organism
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

        self._screen = display.set_mode((math.ceil((x + 1) * factor), math.ceil((y + 1) * factor)))
        self._x = x
        self._y = y
        self._factor = factor
        self._surface = display.get_surface()
        self._clock = Clock()
        self._export_video = export_video
        self._fonts = self.create_fonts([32, 16, 14, 8])
        self.draw_options = pygame_util.DrawOptions(surface=self._surface)
        self.draw_options.transform = pymunk.Transform.scaling(factor)
        self.draw_options.shape_outline_color = (255,0,0,255)
        self._ticks= 0.0
        self._debug_display = False

    # def __init__

    @staticmethod
    def create_fonts(font_sizes_list):
        "Creates different fonts with one list"
        fonts = []
        for size in font_sizes_list:
            fonts.append(
                pygame.font.SysFont("Arial", size))
        return fonts

    def render(self, fnt, what, color, where):
        "Renders the fonts as passed from display_fps"
        text_to_show = fnt.render(what, 0, pygame.Color(color))
        self._screen.blit(text_to_show, where)

    def display_fps(self):
        "Data that will be rendered and blitted in _display"
        self.render(
            self._fonts[0],
            what=str(int(self._clock.get_fps())),
            color="white",
            where=(0, 0))

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
            if event.type == pygame.locals.KEYUP:
                if event.key == pygame.locals.K_F3:
                    self._debug_display = not self._debug_display

        self._surface.fill(Color(0, 0, 0))

        if self._debug_display:
            world._space.debug_draw(self.draw_options)

        for c in world.organisms:
            draw_organism(self._surface, c, self._factor)
        self.display_fps()
        display.flip()
        if self._export_video:
            take_capture()
        self._clock.tick(60)
        self._ticks += 1

    # end def redraw
    def end(self):
        print("Ending at clock tick:", self._ticks)
        pygame.quit()
        sys.exit()
# end class Window


win = type("",(),dict(redraw=lambda _, __: None, end=lambda _: None))()


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

def end():
    """
    draw the world.py
    :param world: the world.py to draw
    :type world: ``World``
    """
    # noinspection PyUnresolvedReferences
    win.end()
# end def redraw


