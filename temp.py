from sim.life.codekaryote import Codekaryote
from sim.life.mind.neuron import Neuron, Activations
from sim.life.mind.brain import Brain
from timeit import timeit

NUMBER = 100000

print("Full brain", timeit(
'''
brain.update()
''',
number=NUMBER,
setup=
"""
from sim.organisms.codekaryote import Codekaryote
from sim.world import Position, World
world = World()
world.initiate(100, 100)
code = Codekaryote(starting_position=Position(coord=[0, 0]), genomes={'movement': [], 'eyes': [119, 50], 'touch': [], 'brain': [2, 0, 4, 2, 1, 1, 5, 1, 2, 3, 0, 0, 1, 4, 0, 4, 3, 3, 2, 0, 0, 4, 2456978449, 3760448392, 1538909483, 2389701972, 2289435698, 3095255089, 864925571, 2379947949, 3681564904, 2421123410]})
brain = code.brain

"""))