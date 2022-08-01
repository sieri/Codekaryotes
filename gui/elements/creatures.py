from sim.creatures.codekaryote import Codekaryote
from pygame import Color
from pygame.surface import Surface
from pygame.draw import rect


def draw_creature(surface, creature, factor):
    """
    Draw a creature on the surface

    :param surface: the surface to draw on it
    :type surface: ``Surface``
    :param creature: the position to draw it in
    :type creature: ``Codekaryote``
    :param factor: the factor to scale the creature to
    :param factor: ``float``
    """
    r = (creature.position.x*factor, creature.position.y*factor, factor, factor)
    rect(surface, color=Color(255, 255, 255), rect=r)
# end def draw_creature
