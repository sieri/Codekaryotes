from sim.life.codekaryote import Codekaryote
from pygame import Color
from pygame.surface import Surface
from pygame.draw import circle


def draw_organism(surface, organism, factor):
    """
    Draw an organism on the surface

    :param surface: the surface to draw on it
    :type surface: ``Surface``
    :param organism: the position to draw it in
    :type organism: ``Codekaryote``
    :param factor: the factor to scale the organism to
    :param factor: ``float``
    """
    pos = organism.position * factor
    # noinspection PyUnresolvedReferences
    circle(surface, Color(organism.color.color), pos, int(organism.body.size*factor), 2)
# end def draw_organism
