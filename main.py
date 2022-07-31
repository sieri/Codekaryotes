from gui import window

from sim.world import World

if __name__ == '__main__':
    # window.set_display()
    world = World()
    world.initiate(10, 10)
    world.populate_randomly(10)
    world.loop()

