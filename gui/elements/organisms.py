from sim.life.codekaryote import Codekaryote
from pygame import Color
from pygame.surface import Surface
from pygame.draw import rect


def draw_organism(surface, organism, factor):
    """
    Draw a organism on the surface

    :param surface: the surface to draw on it
    :type surface: ``Surface``
    :param organism: the position to draw it in
    :type organism: ``Codekaryote``
    :param factor: the factor to scale the organism to
    :param factor: ``float``
    """
    r = (organism.position.x*factor, organism.position.y*factor, factor, factor)
    # noinspection PyUnresolvedReferences
    rect(surface, color=Color(organism.color.color), rect=r)
# end def draw_organism
