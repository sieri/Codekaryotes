from sim.life.codekaryote import BaseModule


class Color(BaseModule):

    def __init__(self, organism, genome):
        super().__init__(organism, genome, "color")
        self._mutation_rate = 0

        self._r = genome[0]
        self._g = genome[1]
        self._b = genome[2]
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        pass
    # end def update

    # -----------------Properties------------------

    @property
    def color(self):
        return self._r, self._g, self._b
    # end def color
# end class Color
