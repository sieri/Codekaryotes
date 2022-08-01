import random
from sim.Parameters import evolution as para_ev
from utils import toggle_bit


class Codekaryote:

    def __init__(self, starting_position, genomes=None):
        """
        :param starting_position: the position the Codekaryotes spawn in
        :type starting_position: ``Position``
        :param genomes: the genome of the Codekaryote if None generate randomly - OPTIONAL
        :type genomes: ```dict(list[int])`` or ``None``
        """

        self._position = starting_position
        self._modules = []

        if genomes is None:
            from sim.creatures.modules import generate_random_genome
            genomes = generate_random_genome()
        # end if

        self._genome = genomes

        from sim.creatures.modules import possible_modules
        for key, genome in genomes.items():
            m = possible_modules[key](self, genome)
            self._modules.append(m)
        # end for
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        for m in self._modules:
            m.update()
        # end def for
    # end def update

    def reproduce_genome(self):
        new_genome = dict()
        for m in self._modules:
            new_genome[m.name] = m.evolve()
        # end def for

        return new_genome
    # end def reproduce_genome

    # -----------------Properties------------------

    @property
    def position(self):
        return self._position
    # end def position
    
    @property
    def genome(self):
        return self._genome
    # end def genome
# end class Codekaryotes


class BaseModule:
    """
    A base module for systems that can evolve independently
    """

    def __init__(self, creature, genome, name):
        """
        :param creature: the creature where this module exists
        :type creature: ``Codekaryote``
        """
        self._creature = creature
        self._genome = genome
        self._name = name
    # def __init__

    # -------------------Methods--------------------

    def update(self):
        """
        update the module for the current tick
        """
        raise NotImplementedError
    # end def update

    def evolve(self):
        """
        return a new genome evolved
        :return: the new genome
        :rtype: ``dict[int]``
        """
        sample = random.sample(range(len(self._genome)), min(para_ev.BASE_RATE, len(self._genome)))
        gen = self._genome.copy()
        for i in sample:
            gen[i] = toggle_bit(self._genome[i], random.randint(0, 31))
        # end for
        return gen

    # end def evolve
    # -----------------Properties------------------

    @property
    def creature(self):
        return self._creature
    # end def creature

    @property
    def genome(self):
        return self._genome
    # end def genome

    @property
    def name(self):
        return self._name
    # end def name
# end class BaseModule
