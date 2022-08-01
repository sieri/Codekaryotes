import pygame

from gui import window

from sim.world import World

if __name__ == '__main__':
    num = 256
    size = (num, num)
    count = num
    pygame.init()
    window.init(size[0], size[1], 2.5, False)
    world = World()
    world.initiate(size[0], size[1])
    world.populate_randomly(count)
    window.redraw(world)

    while True:
        world.loop()
        world.kill_right_screen()
        world.populate_new_generation(count)

