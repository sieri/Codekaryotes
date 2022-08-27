import math
import random
from codekaryotes.codekaryotes import Creature
import utils
from sim.parameters import evolution as para_ev
from utils import toggle_bit
import pymunk as pm


class Codekaryote:

    def __init__(self, starting_position, genomes=None, genome_generator=None):
        """
        :param starting_position: the position the Codekaryotes spawn in
        :type starting_position: ``tuple(int, int)``
        :param genomes: the genome of the Codekaryote if None generate randomly - OPTIONAL
        :type genomes: ``dict(list[int])`` or ``None``
        :param genome_generator: function generating a genome, to create another style of Codekaryotic life - OPTIONAL
        :type genome_generator: ``funct``
        """
        self._rust = Creature(starting_position[0], starting_position[1])
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        self._rust.update_py()
    # end def update

    def reproduce_genome(self):
        self._rust.reproduce_py()
    # end def reproduce_genome

    def die(self):
        """
        remove the creature
        """
        self._rust.die_py()
    # end def die

    def reproduce(self, position):
        self._rust.reproduce_py()
    # end def reproduce



    # -----------------Properties------------------

    @property
    def color(self):
        return self._rust.get_color()
    # end def Color

    @property
    def size(self):
        return self._rust.get_size()
    # end def size

    @property
    def physical_body(self):
        return self._rust.get_physical_body()
    # end def physical_body

    @property
    def shape(self):
        return self._rust.get_shape()
    # end def shape

    @property
    def vision_cone(self):
        return []
    # end def vision_cone

    @property
    def position(self):
        # noinspection PyUnresolvedReferences
        return self.physical_body.position
    # end def position

    @property
    def angle(self):
        # noinspection PyUnresolvedReferences
        return self.physical_body.angle
    #end def angle

# end class Codekaryotes


class BaseModule:
    """
    A base module for systems that can evolve independently
    """

    def __init__(self, organism, genome, name):
        """
        :param organism: the organism where this module exists
        :type organism: ``Codekaryote``
        """
        self._organism = organism
        self._genome = genome
        self._name = name
        self._mutation_rate = para_ev.BASE_RATE
        self.need_reset = True
        organism.__setattr__(name, self)
    # def __init__

    def __str__(self):
        return f"{self._name}({self._genome})"

    # -------------------Methods--------------------

    def update(self):
        """
        update the module for the current tick
        """
        raise NotImplementedError
    # end def update

    def reset(self):
        """
        Reset a module after a step
        """
        pass

    def evolve(self):
        """
        return a new genome evolved
        :return: the new genome
        :rtype: ``list[int]``
        """
        sample = random.sample(range(len(self._genome)), min(self._mutation_rate, len(self._genome)))
        gen = self._genome.copy()
        for i in sample:
            gen[i] = toggle_bit(gen[i], random.randint(0, 31))
        # end for
        return gen

    # end def evolve
    # -----------------Properties------------------

    @property
    def organism(self):
        return self._organism
    # end def organism

    @property
    def genome(self):
        return self._genome
    # end def genome

    @property
    def name(self):
        return self._name
    # end def name
# end class BaseModule
