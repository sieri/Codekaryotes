from sim.life.body.eyes import Eyes
from sim.life.body.modules import Movement, Touch, Eating
from sim.life.common.aesthetic import Color
from sim.life.common.ancestry import Ancestry
from sim.life.common.energy import EnergySource, EnergyStorage
from sim.life.genome.body_generators import generate_eyes, generate_organism_color, generate_energy_storage
from sim.life.genome.brain_generator import generate_brain
from sim.life.genome.plant_generator import generate_plant, generate_plant_color
from sim.life.mind.brain import Brain

possible_modules = {
            "movement": Movement,
            "eyes": Eyes,
            "touch": Touch,
            "brain": Brain,
            "color": Color,
            "energy_source": EnergySource,
            "energy_storage": EnergyStorage,
            "eating": Eating,
            "ancestry": Ancestry

}


def generate_random_creature_full_genome():
    """
    Generate a random genome

    :return: the random genome
    :rtype: ```dict[str,list[int]]``
    """
    genomes = {
        "movement": [],
        "eyes": generate_eyes(),
        "touch": [],
        "brain": generate_brain(),
        "color": generate_organism_color(),
        "energy_storage": generate_energy_storage(),
        "eating": [],
        "ancestry": [0, ]
    }
    return genomes
# end def generate_random_creature_full_genome


def generate_random_plant_genome():
    genomes = {
        "energy_source": generate_plant(),
        "color": generate_plant_color(),
        "ancestry": [0, ],
    }
    return genomes
# end def generate_random_plant_genome
