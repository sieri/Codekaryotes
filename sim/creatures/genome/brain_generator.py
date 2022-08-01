from random import randint
from sim.Parameters import brain as param


def generate_brain():
    from sim.creatures.mind.neuron import Activations
    inputs = [randint(0, len(Activations)) for _ in range(8)]

    outputs = [randint(0, len(Activations)) for _ in range(4)]

    internal = [randint(0, len(Activations)) for _ in range(param.INTERNAL_NEURON)]

    links = [randint(0, 4303355903) for _ in range(param.LINKS)]

    return inputs+outputs+internal+links
