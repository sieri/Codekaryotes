from codekaryotes.codekaryotes import brain_update, get_brain, Brain, Activation, Position, \
    acc_from_int
from codekaryotes.codekaryotes import NeuronDefinition as nd
from codekaryotes.codekaryotes import LinkDefinition
from sim.parameters.settings import Settings

import numpy as np

from sim.life.codekaryote import BaseModule
from sim.life.common.energy import AbstractEnergyConsumer
from sim.parameters import brain as param
from sim.life.mind.neuron import NeuronInput, NeuronExit, Neuron, Activations, Link
from sim.life.mind.output_neurons import moveneuron
from sim.life.mind.input_neurons import basic, vision
from utils import test_bit, bit_range, to_signed

if not Settings().brain_rust:
    class Brain(AbstractEnergyConsumer):

        def __init__(self, organism, genome):
            super().__init__(organism=organism, genome=genome,
                             passive=True, name="brain")
            setattr(organism, "brain", self)
            self._input_neurons = []
            self._output_neurons = []
            self._internal_neurons = []
            self._links = []

            # initialize from the genome
            i = 0

            # input neurons
            self._input_neurons.append(basic.ConstantNeuron(Activations.from_genome(genome[i]), organism))
            i += 1
            self._input_neurons.append(vision.DistLeft(Activations.from_genome(genome[i]), organism))
            i += 1
            self._input_neurons.append(vision.DistRight(Activations.from_genome(genome[i]), organism))
            i += 1
            self._input_neurons.append(vision.DistUp(Activations.from_genome(genome[i]), organism))
            i += 1
            self._input_neurons.append(vision.DistDown(Activations.from_genome(genome[i]), organism))
            i += 1
            self._input_neurons.append(vision.NumForward(Activations.from_genome(genome[i]), organism))
            i += 1
            self._input_neurons.append(basic.TouchNeuron(Activations.from_genome(genome[i]), organism))
            i += 1
            self._input_neurons.append(basic.TouchForwardNeuron(Activations.from_genome(genome[i]), organism))
            i += 1

            # output neurons
            self._output_neurons.append(moveneuron.MoveRightNeuron(Activations.from_genome(genome[i]), organism))
            i += 1
            self._output_neurons.append(moveneuron.MoveLeftNeuron(Activations.from_genome(genome[i]), organism))
            i += 1
            self._output_neurons.append(moveneuron.MoveUpNeuron(Activations.from_genome(genome[i]), organism))
            i += 1
            self._output_neurons.append(moveneuron.MoveDownNeuron(Activations.from_genome(genome[i]), organism))
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

            self._clean_links()

            self._create_interface()

            self._energy_rate = len(self._links)*param.ENERGY_PER_LINK
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
            self._links.append(Link(source=source, output=output, weight=weight, id=0))
        # end def create_link

        # noinspection DuplicatedCode
        def _create_interface(self):
            # noinspection PyTypeChecker
            self._all_neurons = self._output_neurons + self._input_neurons + self._internal_neurons

            self._linear_neuron = [n for n in self._all_neurons if n.activation == Activations.LINEAR]
            self._binary_neuron = [n for n in self._all_neurons if n.activation == Activations.BINARY_STEP]
            self._logistic_neuron = [n for n in self._all_neurons if n.activation == Activations.LOGISTIC]
            self._tanh_neuron = [n for n in self._all_neurons if n.activation == Activations.TANH]
            self._gaussian_neuron = [n for n in self._all_neurons if n.activation == Activations.GAUSSIAN]

            # self._linear_outputs = np.empty(len(self._linear_neuron))
            self._binary_outputs = np.empty(len(self._binary_neuron), dtype=np.float)
            self._logistic_outputs = np.empty(len(self._logistic_neuron), dtype=np.float)
            self._tanh_outputs = np.empty(len(self._tanh_neuron), dtype=np.float)
            self._gaussian_outputs = np.empty(len(self._gaussian_neuron), dtype=np.float)

            self._linear_outputs = self._linear_inputs = np.empty(len(self._linear_neuron), dtype=np.float)
            self._binary_inputs = np.empty(len(self._binary_neuron), dtype=np.float)
            self._logistic_inputs = np.empty(len(self._logistic_neuron), dtype=np.float)
            self._tanh_inputs = np.empty(len(self._tanh_neuron), dtype=np.float)
            self._gaussian_inputs = np.empty(len(self._gaussian_neuron), dtype=np.float)

            for i, n in enumerate(self._linear_neuron):
                n.interface = (i, self._linear_inputs, self._linear_outputs)
            for i, n in enumerate(self._binary_neuron):
                n.interface = (i, self._binary_inputs, self._binary_outputs)
            for i, n in enumerate(self._logistic_neuron):
                n.interface = (i, self._logistic_inputs, self._logistic_outputs)
            for i, n in enumerate(self._tanh_neuron):
                n.interface = (i, self._tanh_inputs, self._tanh_outputs)
            for i, n in enumerate(self._gaussian_neuron):
                n.interface = (i, self._gaussian_inputs, self._gaussian_outputs)
        # end def _create_interface

        def _clean_links(self):
            list_of_output = [(link, link._output) for link in self._links if isinstance(link._output, NeuronExit)]

            final_output = [i[1] for i in list_of_output]
            link_to_check = [i[0] for i in list_of_output]
            final_links = []
            final_internal_neurons = []
            final_inputs = []
            while len(link_to_check) > 0:
                neuron_to_check = [link._input for link in link_to_check]
                final_links += link_to_check
                link_to_check = [link for link in self._links if link not in final_links and link._output in neuron_to_check]
                final_internal_neurons += [neuron for neuron in neuron_to_check if not isinstance(neuron, NeuronInput)]
                final_inputs += [neuron for neuron in neuron_to_check if isinstance(neuron, NeuronInput)]

            self._input_neurons = final_inputs
            self._output_neurons = final_output
            self._internal_neurons = final_internal_neurons
            self._links = final_links
        # def _clean_links

        def update(self):
            super().update()
            list(map(Link.update, self._links))

            # end for
            for i in self._input_neurons:
                i.prepare()

            np.choose(self._binary_inputs < 0, [1, 0], out=self._binary_outputs)
            self._logistic_outputs = 1 / (1 + np.exp(-self._logistic_inputs))
            np.tanh(self._tanh_inputs, out=self._tanh_outputs)
            np.exp(-np.power(self._gaussian_inputs, 2), out=self._gaussian_outputs)

        # end def __init__

        def output(self):
            """
            Send the signals of the last round to the body
            """
            for o in self._output_neurons:
                o.update()

        # -----------------Properties------------------

    # end class Brain
else:

    class Brain(AbstractEnergyConsumer):

        def __init__(self, organism, genome):
            super().__init__(organism=organism, genome=genome,
                             passive=True, name="brain")
            setattr(organism, "brain", self)
            self._input_neurons = []
            self._output_neurons = []
            self._internal_neurons = []
            self._links = []

            # initialize from the genome
            i = 0

            # input neurons
            self._input_neurons.append(basic.ConstantNeuron(Activations.from_genome(genome[i]), organism))
            i += 1
            self._input_neurons.append(vision.DistLeft(Activations.from_genome(genome[i]), organism))
            i += 1
            self._input_neurons.append(vision.DistRight(Activations.from_genome(genome[i]), organism))
            i += 1
            self._input_neurons.append(vision.DistUp(Activations.from_genome(genome[i]), organism))
            i += 1
            self._input_neurons.append(vision.DistDown(Activations.from_genome(genome[i]), organism))
            i += 1
            self._input_neurons.append(vision.NumForward(Activations.from_genome(genome[i]), organism))
            i += 1
            self._input_neurons.append(basic.TouchNeuron(Activations.from_genome(genome[i]), organism))
            i += 1
            self._input_neurons.append(basic.TouchForwardNeuron(Activations.from_genome(genome[i]), organism))
            i += 1

            # output neurons
            self._output_neurons.append(moveneuron.MoveRightNeuron(Activations.from_genome(genome[i]), organism))
            i += 1
            self._output_neurons.append(moveneuron.MoveLeftNeuron(Activations.from_genome(genome[i]), organism))
            i += 1
            self._output_neurons.append(moveneuron.MoveUpNeuron(Activations.from_genome(genome[i]), organism))
            i += 1
            self._output_neurons.append(moveneuron.MoveDownNeuron(Activations.from_genome(genome[i]), organism))
            i += 1

            # internal neurones
            for j in range(param.INTERNAL_NEURON):
                self._internal_neurons.append(Neuron(Activations.from_genome(genome[i])))
                i += 1
            # end for

            # links
            for j in range(param.INTERNAL_NEURON):
                self._create_link(genome[i], j)
                i += 1
            # end for

            self._clean_links()

            self._rust_brain = get_brain()

            rust_link_partial = {}
            rust_link = []

            for (i, n) in enumerate(self._input_neurons):
                self._rust_brain.add_input(nd(acc_from_int(n.activation), Position.Internal, i, n))
                links = [lk for lk in self._links if lk._input in self._input_neurons]
                for lk in links:
                    rust_link_partial[lk.id] =LinkDefinition(input=i, output=0, weight=lk._weight, input_type=Position.Input)


            for (i, n) in enumerate(self._internal_neurons):
                self._rust_brain.add_internal(nd(acc_from_int(n.activation), Position.Internal, i, n))
                links = [lk for lk in self._links if lk._input in self._internal_neurons ]
                for lk in links:
                    rust_link_partial[lk.id] =LinkDefinition(input=i, output=0, weight=lk._weight,input_type=Position.Internal)

                links = [lk for lk in self._links if lk._output in self._internal_neurons ]
                for lk in links:
                    rlk = rust_link_partial[lk.id]
                    rlk.output = i
                    rlk.output_type = Position.Internal
                    rust_link.append(rlk)

            for (i, n) in enumerate(self._output_neurons):
                self._rust_brain.add_output(nd(acc_from_int(n.activation), Position.Output, i, n))

                links = [lk for lk in self._links if lk._output in self._output_neurons]
                for lk in links:
                    rlk = rust_link_partial[lk.id]
                    rlk.output = i
                    rust_link.append(rlk)

            for l in rust_link:
                self._rust_brain.add_link(l)

            self._rust_brain.initiate()
            self._energy_rate = len(self._links)*param.ENERGY_PER_LINK
        # end def __init__

        # -------------------Methods--------------------

        def _create_link(self, gene, id):
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
            self._links.append(Link(source=source, output=output, weight=weight, id=id))
        # end def create_link

        # noinspection DuplicatedCode
        def _create_interface(self):
            # noinspection PyTypeChecker
            self._all_neurons = self._output_neurons + self._input_neurons + self._internal_neurons

            self._linear_neuron = [n for n in self._all_neurons if n.activation == Activations.LINEAR]
            self._binary_neuron = [n for n in self._all_neurons if n.activation == Activations.BINARY_STEP]
            self._logistic_neuron = [n for n in self._all_neurons if n.activation == Activations.LOGISTIC]
            self._tanh_neuron = [n for n in self._all_neurons if n.activation == Activations.TANH]
            self._gaussian_neuron = [n for n in self._all_neurons if n.activation == Activations.GAUSSIAN]

            # self._linear_outputs = np.empty(len(self._linear_neuron))
            self._binary_outputs = np.empty(len(self._binary_neuron), dtype=np.float)
            self._logistic_outputs = np.empty(len(self._logistic_neuron), dtype=np.float)
            self._tanh_outputs = np.empty(len(self._tanh_neuron), dtype=np.float)
            self._gaussian_outputs = np.empty(len(self._gaussian_neuron), dtype=np.float)

            self._linear_outputs = self._linear_inputs = np.empty(len(self._linear_neuron), dtype=np.float)
            self._binary_inputs = np.empty(len(self._binary_neuron), dtype=np.float)
            self._logistic_inputs = np.empty(len(self._logistic_neuron), dtype=np.float)
            self._tanh_inputs = np.empty(len(self._tanh_neuron), dtype=np.float)
            self._gaussian_inputs = np.empty(len(self._gaussian_neuron), dtype=np.float)

            for i, n in enumerate(self._linear_neuron):
                n.interface = (i, self._linear_inputs, self._linear_outputs)
            for i, n in enumerate(self._binary_neuron):
                n.interface = (i, self._binary_inputs, self._binary_outputs)
            for i, n in enumerate(self._logistic_neuron):
                n.interface = (i, self._logistic_inputs, self._logistic_outputs)
            for i, n in enumerate(self._tanh_neuron):
                n.interface = (i, self._tanh_inputs, self._tanh_outputs)
            for i, n in enumerate(self._gaussian_neuron):
                n.interface = (i, self._gaussian_inputs, self._gaussian_outputs)
        # end def _create_interface

        def _clean_links(self):
            list_of_output = [(link, link._output) for link in self._links if isinstance(link._output, NeuronExit)]

            final_output = [i[1] for i in list_of_output]
            link_to_check = [i[0] for i in list_of_output]
            final_links = []
            final_internal_neurons = []
            final_inputs = []
            while len(link_to_check) > 0:
                neuron_to_check = [link._input for link in link_to_check]
                final_links += link_to_check
                link_to_check = [link for link in self._links if link not in final_links and link._output in neuron_to_check]
                final_internal_neurons += [neuron for neuron in neuron_to_check if not isinstance(neuron, NeuronInput)]
                final_inputs += [neuron for neuron in neuron_to_check if isinstance(neuron, NeuronInput)]

            self._input_neurons = final_inputs
            self._output_neurons = final_output
            self._internal_neurons = final_internal_neurons
            self._links = final_links
        # def _clean_links

        def update(self):
            super().update()
            self._rust_brain.update()
        # end def __init__

        def output(self):
            """
            Send the signals of the last round to the body
            """

            self._rust_brain.output()

        # -----------------Properties------------------