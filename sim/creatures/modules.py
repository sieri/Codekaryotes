from sim.creatures.body.eyes import Eyes
from sim.creatures.body.modules import Movement
from sim.creatures.genome.brain_generator import generate_brain
from sim.creatures.mind.brain import Brain

possible_modules = {
            "movement": Movement,
            "eyes": Eyes,
            "brain": Brain,
        }


def generate_random_genome():
    """
    Generate a random genome

    :return: the random genome
    :rtype: ```dict[str,list[int]]``
    """
    genomes = {
        "movement": [],
        "eyes": [],
        "brain": generate_brain(),
    }
    return genomes
# end def generate_random_genome