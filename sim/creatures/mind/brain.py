from sim.creatures.codekaryote import BaseModule
from sim.parameters import brain as param
from sim.creatures.mind.neuron import NeuronInput, NeuronExit, Neuron, Activations, Link
from sim.creatures.mind.output_neurons import moveneuron
from sim.creatures.mind.input_neurons import basic, vision
from utils import test_bit, bit_range, to_signed


class Brain(BaseModule):

    def __init__(self, creature, genome):
        super().__init__(creature, genome, "brain")
        self._input_neurons = []
        self._output_neurons = []
        self._internal_neurons = []
        self._links = []

        # initialize from the genome
        i = 0

        # input neurons
        self._input_neurons.append(basic.ConstantNeuron(Activations.from_genome(genome[i]), creature))
        i += 1
        self._input_neurons.append(vision.DistLeft(Activations.from_genome(genome[i]), creature))
        i += 1
        self._input_neurons.append(vision.DistRight(Activations.from_genome(genome[i]), creature))
        i += 1
        self._input_neurons.append(vision.DistUp(Activations.from_genome(genome[i]), creature))
        i += 1
        self._input_neurons.append(vision.DistDown(Activations.from_genome(genome[i]), creature))
        i += 1
        self._input_neurons.append(vision.NumForward(Activations.from_genome(genome[i]), creature))
        i += 1
        self._input_neurons.append(basic.TouchNeuron(Activations.from_genome(genome[i]), creature))
        i += 1
        self._input_neurons.append(basic.TouchForwardNeuron(Activations.from_genome(genome[i]), creature))
        i += 1

        # output neurons
        self._output_neurons.append(moveneuron.MoveRightNeuron(Activations.from_genome(genome[i]), creature))
        i += 1
        self._output_neurons.append(moveneuron.MoveLeftNeuron(Activations.from_genome(genome[i]), creature))
        i += 1
        self._output_neurons.append(moveneuron.MoveUpNeuron(Activations.from_genome(genome[i]), creature))
        i += 1
        self._output_neurons.append(moveneuron.MoveDownNeuron(Activations.from_genome(genome[i]), creature))
        i += 1

        # internal neurones
        for j in range(param.INTERNAL_NEURON):
            self._internal_neurons.append(Neuron(Activations.from_genome(genome[i])))
            i += 1
        # end for

        # links
        for j in range(param.INTERNAL_NEURON):
            self._create_link(genome[i])
            i += 1
        # end for
    # end def __init__

    # -------------------Methods--------------------

    def _create_link(self, gene):
        """
        create a link from a specific gene int
        :param gene: the gene
        :param gene: ``int``
        """

        if test_bit(gene, 31):  # if bit 31 is 1 the source is an input neuron
            index = bit_range(gene, 24, 7) % len(self._input_neurons)
            source = self._input_neurons[index]
        else:
            index = bit_range(gene, 24, 7) % len(self._internal_neurons)
            source = self._internal_neurons[index]
        # end if

        if test_bit(gene, 23):  # if bit 31 is 1 the output is an input neuron
            index = bit_range(gene, 16, 7) % len(self._output_neurons)
            output = self._output_neurons[index]
        else:
            index = bit_range(gene, 16, 7) % len(self._internal_neurons)
            output = self._internal_neurons[index]
        # end if
        weight = to_signed(bit_range(gene, 0, 16), 16) / 8191.75
        self._links.append(Link(source=source, output=output, weight=weight))
    # end def create_link

    def update(self):
        for link in self._links:
            link.update()
        # end for
        for n in self._input_neurons:
            n.update()
        # end for
        for n in self._output_neurons:
            n.update()
        # end for
        for n in self._internal_neurons:
            n.update()
        # end for
    # end def __init__

    # -----------------Properties------------------


# end class Brain