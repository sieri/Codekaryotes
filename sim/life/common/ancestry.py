from sim.life.codekaryote import BaseModule


class Ancestry(BaseModule):

    def __init__(self, organism, genome):
        super().__init__(organism=organism, genome=genome,
                         name="ancestry")
        self._mutation_rate = 0
        self._generation = genome[0]
    # end def __init__

    # -------------------Methods--------------------

    def evolve(self):
        return [self._generation+1, ]

    def update(self):
        pass

    # -----------------Properties------------------

    @property
    def generation(self):
        return self._generation
    # end def generation

# end class Ancestry(BaseModule)