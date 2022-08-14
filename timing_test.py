from timeit import timeit

from sim.parameters.world import GENERATION_TIME
from sim.world import World

rust = True
repeats = 1

class FalseWorld(World):
    def __init__(self, *args, **kwargs):
        pass

world = FalseWorld()



if rust:
    print("Test the rust run" ,timeit("world.loop_generation()", number=repeats, setup=
"""

from sim.parameters.settings import Settings
settings = Settings()
settings.set_brain_rust(True)
from timing_test import world


world.initiate(100, 100)
world.populate_randomly(count_creature=200, count_plant=50)
"""))
else:
    print("Test the python run", timeit("world.loop_generation()", number=repeats, setup=
"""
from sim.parameters.settings import Settings
from timing_test import world

settings = Settings()
settings.set_brain_rust(False)


world.initiate(100, 100)
world.populate_randomly(count_creature=200, count_plant=50)
"""))

