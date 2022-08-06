from sim.life.codekaryote import BaseModule
from sim.parameters import body as param


class Eating(BaseModule):

    def __init__(self, organism, genome):
        super().__init__(organism=organism, genome=genome, name="eating")
        self._ticks = 0
        self._can_eat = False
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        if self._can_eat:
            pos = self._organism.position
            # noinspection PyUnresolvedReferences
            touch = self._organism.touch.touch_forward

            if touch:
                # noinspection PyUnresolvedReferences
                other = self._organism.touch.organism_touching
                if hasattr(other, "energy_source"):
                    # noinspection PyUnresolvedReferences
                    self._organism.energy_storage.current_energy += other.energy_source.energy
                    other.die()

                    self._can_eat = False
                    self._ticks = 0
                # end if
        else:
            self._ticks += 1
            if self._ticks >= param.EAT_TICK_TIMEOUT:
                self._can_eat = True

    # -----------------Properties------------------
# end class Eating


class Reproducer(BaseModule):

    def __init__(self, organism, genome ):
        super().__init__(organism=organism, genome=genome, name="reproducer")
    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        # noinspection PyUnresolvedReferences
        if self._organism.energy_storage.percent > param.REPRODUCE_THRESHOLD:
            print(self._organism.energy_storage.percent)
            self._organism.reproduce((self._organism.position.x, self._organism.position.y))
            # noinspection PyUnresolvedReferences
            self._organism.energy_storage.current_energy -= self._organism.energy_storage.energy_storage_max * param.REPRODUCE_COST

    # -----------------Properties------------------


# end class Reproducer