import os

import pygame

from gui import window

from sim.world import World
from sim.parameters import world as param_world
import stats

os.environ["RUST_BACKTRACE"] = "full"

if __name__ == '__main__':
    num = 256
    size = (num, num)
    pygame.init()
    window.init(size[0], size[1], 2.5, False)
    world = World()
    world.initiate(size[0], size[1])
    stats.start_thread()

    if param_world.KICKSTART_MODE:
        world.kickstart_world()
    elif param_world.CONTINUOUS_GENERATIONS:
            world.populate_randomly(count_creature=param_world.START_CREATURE_COUNT,
                                    count_plant=param_world.START_PLANT_COUNT)
            world.loop_infinite()
    else:
        world.populate_randomly(count_creature=param_world.START_CREATURE_COUNT,
                                count_plant=param_world.START_PLANT_COUNT)

        while True:
            world.loop_generation()
            world.kill_right_screen()
            world.populate_new_generation(count)
