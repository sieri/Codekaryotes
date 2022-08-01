from sim.creatures.body.eyes import Eyes
from sim.creatures.body.modules import Movement, Touch
from sim.creatures.genome.body_generators import generate_eyes
from sim.creatures.genome.brain_generator import generate_brain
from sim.creatures.mind.brain import Brain

possible_modules = {
            "movement": Movement,
            "eyes": Eyes,
            "touch": Touch,
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
        "eyes": generate_eyes(),
        "touch": [],
        "brain": generate_brain(),
    }
    return genomes
# end def generate_random_genome