from sim.life.body.eyes import Eyes
from sim.life.body.lifecycle import Eating, Reproducer
from sim.life.body.physics import Movement, Touch, BodyPassiveCircle, BodyActiveCircle
from sim.life.common.aesthetic import Color
from sim.life.common.ancestry import Ancestry
from sim.life.common.energy import EnergySource, EnergyStorage
from sim.life.genome.body_generators import generate_eyes, generate_organism_color, generate_energy_storage, \
    generate_size
from sim.life.genome.brain_generator import generate_brain
from sim.life.genome.plant_generator import generate_plant, generate_plant_color
from sim.life.mind.brain import Brain

possible_modules = {
            "body_active_circle": BodyActiveCircle,
            "body_passive_circle": BodyPassiveCircle,
            "movement": Movement,
            "eyes": Eyes,
            "touch": Touch,
            "brain": Brain,
            "color": Color,
            "energy_source": EnergySource,
            "energy_storage": EnergyStorage,
            "eating": Eating,
            "reproducer": Reproducer,
            "ancestry": Ancestry,


}


def generate_random_creature_full_genome():
    """
    Generate a random genome

    :return: the random genome
    :rtype: ```dict[str,list[int]]``
    """
    genomes = {
        "eyes": generate_eyes(),
        "touch": [],
        "body_active_circle": generate_size(),
        "movement": [],
        "color": generate_organism_color(),
        "energy_storage": generate_energy_storage(),
        "eating": [],
        "reproducer": [],
        "ancestry": [0, ],
        "brain": generate_brain(),
    }
    return genomes
# end def generate_random_creature_full_genome


def generate_random_plant_genome():
    genomes = {
        "body_passive_circle": generate_size(),
        "energy_source": generate_plant(),
        "color": generate_plant_color(),
        "ancestry": [0, ],
    }
    return genomes
# end def generate_random_plant_genome
