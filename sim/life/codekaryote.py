import math
import random
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

        self._alive = True
        self._modules = []
        self._modules_reset = []

        if genome_generator is None:
            from sim.life.modules import generate_random_creature_full_genome
            genome_generator = generate_random_creature_full_genome

        if genomes is None:
            genomes = genome_generator()
        # end if

        self._genome = genomes

        from sim.life.modules import possible_modules
        for key, genome in genomes.items():
            m = possible_modules[key](self, genome)
            self._modules.append(m)
            if m.need_reset:
                self._modules_reset.append(m)
        # end for

        # noinspection PyUnresolvedReferences
        self.physical_body.position = starting_position
        # noinspection PyUnresolvedReferences
        self.physical_body.angle = random.random()*2*math.pi
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        self.brain.output() # if plants need to be updated, split the class

        for m in self._modules:
            m.update()
        # end def for

        for m in self._modules_reset:
            m.reset()
        # end def for
    # end def update

    def reproduce_genome(self):
        new_genome = dict()
        for m in self._modules:
            new_genome[m.name] = m.evolve()
        # end def for

        return new_genome
    # end def reproduce_genome

    def die(self):
        """
        remove the creature
        """
        from sim.world import World
        if self._alive:
            World().remove_organism(self)
            self._alive = False
    # end def die

    def reproduce(self, position):
        from sim.world import World
        genome = self.reproduce_genome()
        World().add_organism(organism=Codekaryote(starting_position=position, genomes=genome))
    # end def reproduce

    # -----------------Properties------------------

    @property
    def position(self):
        # noinspection PyUnresolvedReferences
        return self.physical_body.position
    # end def position

    @property
    def angle(self):
        # noinspection PyUnresolvedReferences
        return self.physical_body.angle
    # end def angle

    @property
    def genome(self):
        return self._genome
    # end def genome
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
