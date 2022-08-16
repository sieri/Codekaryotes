import os
import pygame

from gui import window
from sim.parameters.settings import Settings
settings = Settings()
settings.set_brain_rust(True)
from sim.world import World

os.environ["RUST_BACKTRACE"] = "full"

if __name__ == '__main__':
    world = World()
    world.initiate(10, 10)
    pygame.init()
    window.init(10, 10, 50, False)
    world.populate_randomly(count_creature=1, count_plant=0)
    world.loop_generation()
