import pygame

from gui import window

from sim.world import World
from sim.parameters import world as param_world

if __name__ == '__main__':
    num = 256
    size = (num, num)
    count = num
    pygame.init()
    window.init(size[0], size[1], 2.5, False)
    world = World()
    world.initiate(size[0], size[1])

    if param_world.CONTINIOUS_GENERATIONS:
        world.populate_randomly(count_creature=count, count_plant=count)
        world.loop_infinite()
    else:
        world.populate_randomly(count_creature=count, count_plant=0)

        while True:
            world.loop_generation()
            world.kill_right_screen()
            world.populate_new_generation(count)

