from sim.creatures.genome.brain_generator import generate_brain


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
            genomes = self.__class__.generate_random_genome()

        from sim.creatures.modules import possible_modules
        for key, genome in genomes.items():
            m = possible_modules[key](self, genome)
            self._modules.append(m)
    # end def __init__

    # -------------------Methods--------------------

    @classmethod
    def generate_random_genome(cls):
        """
        Generate a random genome

        :return: the random genome
        :rtype: ```dict[str,list[int]]``
        """
        genomes = {
            "movement": [],
            "mind": generate_brain(),
        }
        return genomes
    # end def generate_random_genome

    def update(self):
        for m in self._modules:
            m.update()
        # end def for
    # end def update

    # -----------------Properties------------------

    @property
    def position(self):
        return self._position
    # end def position
# end class Codekaryotes


class BaseModule:
    """
    A base module for systems that can evolve independently
    """

    def __init__(self, creature, genome):
        """
        :param creature: the creature where this module exists
        :type creature: ``Codekaryote``
        """
        self._creature = creature
        self._genome = genome
    # def __init__

    # -------------------Methods--------------------

    def update(self):
        """
        update the module for the current tick
        """
        raise NotImplementedError
    # end def update

    # -----------------Properties------------------

    @property
    def creature(self):
        return self._creature
    # end def creature

    @property
    def genome(self):
        return self._genome
    # end def genome
# end class BaseModule
