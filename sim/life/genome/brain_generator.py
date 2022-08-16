from random import randint
from sim.parameters import brain as param
from sim.life.mind.neuron import Activations


def generate_neuron():
    return randint(0, len(Activations))


def generate_brain():

    inputs = [generate_neuron() for _ in range(param.NUM_INPUT)]

    outputs = [generate_neuron() for _ in range(param.NUM_OUTPUT)]

    internal = [generate_neuron() for _ in range(param.INTERNAL_NEURON)]

    links = [randint(0, 4303355903) for _ in range(param.LINKS)]

    return inputs+outputs+internal+links
