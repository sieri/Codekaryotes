from sim.life.codekaryote import BaseModule
from sim.parameters import plant as plant_param
from sim.parameters import body as body_param

FACTOR = 4303355903 / plant_param.ENERGY_MAX


class AbstractEnergyConsumer(BaseModule):

    def __init__(self, organism, genome, passive, name):
        super().__init__(organism, genome, name)
        # generate from genome
        self._energy_rate = 0
        self._passive = passive
        self._active = False

    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        if self._active or self._passive:
            # noinspection PyUnresolvedReferences
            self._organism.energy_storage.current_energy -= self._energy_rate
    # end def update

    # -----------------Properties------------------


# end class EnergyConsumer


class EnergySource(BaseModule):

    def __init__(self, organism, genome):
        super().__init__(organism, genome, "energy_source")

        # generate from genome
        self._energy_source = genome[0]/FACTOR

    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        pass
    # end def update

    # -----------------Properties------------------

    @property
    def energy(self):
        return self._energy_source
    # end def energy

# end class EnergySource


class EnergyStorage(AbstractEnergyConsumer):

    def __init__(self, organism, genome):
        super().__init__(organism=organism, genome=genome,
                         passive=True, name="energy_storage")

        # generate from genome
        self._energy_storage_max = genome[0]/FACTOR
        self.current_energy = self._energy_storage_max

        self._energy_rate = self._energy_storage_max*body_param.ENERGY_STORAGE_LOSS_RATE

    # end def __init__

    # -------------------Methods--------------------

    def update(self):
        super().update()
    # end def update

    # -----------------Properties------------------

    @property
    def current_energy(self):
        return self._current_energy
    # end def current_energy

    @current_energy.setter
    def current_energy(self, value):
        self._current_energy = value
        if self._current_energy <= 0:
            self.organism.die()
    # end def current_energy

# end class EnergyStorage
