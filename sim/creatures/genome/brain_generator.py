from random import randint
from sim.Parameters import brain as param


def generate_brain():
    from sim.creatures.mind.neuron import Activations
    inputs = [randint(0, len(Activations)),]

    outputs = [randint(0, len(Activations)), randint(0, len(Activations)),
               randint(0, len(Activations)), randint(0, len(Activations)),]

    internal = [randint(0, len(Activations)) for i in range(param.INTERNAL_NEURON)]

    links = [randint(0, 4303355903) for i in range(param.LINKS)]

    return inputs+outputs+internal+links
