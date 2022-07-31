import pygame
from pygame import display, Color
from pygame.time import Clock
from gui.elements.creatures import draw_creature
surface = None

if surface is None:
    pygame.init()
    display.init()
    display.set_mode((640, 480))
    surface = display.get_surface()
    clock = Clock()


def redraw(world):
    """
    draw the sim
    :param world: the sim to draw
    :type world: ``World``
    """
    surface.fill(Color(0, 0, 0))
    for c in world.creatures:
        draw_creature(surface, c)
    display.flip()
    clock.tick(60)

# end def redraw
