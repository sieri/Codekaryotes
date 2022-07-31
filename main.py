from gui import window

from sim.world import World

if __name__ == '__main__':
    size = (100, 100)
    count = 100

    window.init(size[0], size[1], 10, False)
    world = World()
    world.initiate(size[0], size[1])
    world.populate_randomly(count)

    world.loop()

