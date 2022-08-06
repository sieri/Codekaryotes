import pygame

from gui import window

from sim.world import World
from sim.parameters import world as param_world
import stats

if __name__ == '__main__':
    num = 100
    size = (num, num)
    count = num
    pygame.init()
    window.init(size[0], size[1], 5, False)
    world = World()
    world.initiate(size[0], size[1])
    stats.start_thread()
    
    if param_world.CONTINUOUS_GENERATIONS:
        world.populate_randomly(count_creature=count, count_plant=count*5)
        world.loop_infinite()
    else:
        world.populate_randomly(count_creature=count, count_plant=0)

        while True:
            world.loop_generation()
            world.kill_right_screen()
            world.populate_new_generation(count)
