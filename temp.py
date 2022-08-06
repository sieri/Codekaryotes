from sim.world import World
import pickle

world = World()
world.initiate(10, 10)

world.populate_randomly(count_creature=10, count_plant=30)

if __name__ == '__main__':
    world.loop_generation()
    # noinspection PyProtectedMember

    print(pickle.dumps(world._jar))
