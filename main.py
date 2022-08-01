import pygame

from gui import window
from sim.multiprocess import start_processes

from sim.world import World

from os import environ
environ['PYGAME_HIDE_SUPPORT_PROMPT'] = '1'


if __name__ == '__main__':
    num = 128
    size = (num, num)
    count = num
    pygame.init()
    start_processes()
    window.init(size[0], size[1], 2.5, False)
    world = World()
    world.initiate(size[0], size[1])
    world.populate_randomly(count)
    window.redraw(world)

    while True:
        world.loop()
        world.kill_right_screen()
        world.populate_new_generation(count)

