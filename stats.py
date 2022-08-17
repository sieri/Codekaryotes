import threading
import time
import multiprocessing as mult

import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
import matplotlib.animation as animation
from matplotlib import style

import numpy as np

from sim.world import World
from collections import deque, Counter

plt.style.use('ggplot')

NUMBER_TICK = 100

conn = None


class Displayer():
    """Subprocess displaying the graphs"""

    _age_ax = None
    _pop_ax = None
    _gen_ax = None
    _fig = None
    _horizontal_ticks = np.arange(NUMBER_TICK)
    _legends = {}
    # -------------------Methods--------------------

    @classmethod
    def run(cls, pipe):
        Displayer.pipe = pipe
        style.use('fivethirtyeight')
        cls._fig = plt.figure(figsize=(10, 5))
        # population count graph
        cls._pop_ax = plt.subplot(131)
        pop = mpatches.Patch(color='b', label='Population')
        plant = mpatches.Patch(color='g', label='Plant')
        cls._legends["pop"] = [pop, plant]
        cls._pop_ax.legend(handles=cls._legends["pop"])
        cls._pop_ax.set(xlim=(0, NUMBER_TICK), xticks=np.linspace(0, NUMBER_TICK, 9),
                        yticks=np.linspace(0, 100, 10))


        # generation plot
        cls._gen_ax = plt.subplot(132)
        cls._gen_ax.xaxis.set_ticks_position('none')
        cls._gen_ax.yaxis.set_ticks_position('none')

        # population count graph
        cls._age_ax = plt.subplot(133)
        cls._age_ax.set(xlim=(0, NUMBER_TICK), xticks=np.linspace(0, NUMBER_TICK, 9),
                        yticks=np.linspace(0, 100, 10))

        ani = animation.FuncAnimation(cls._fig, cls.update_graph, interval=45)

        plt.show()


    @classmethod
    def update_graph(cls, _=None):
        """
        create a graph and update it
        :param aggr: the aggregator
        :type aggr: ``Aggregator``
        :param _: ignored
        :type _: ``object``
        """
        try:
            aggr = cls.pipe.recv()
        except EOFError:
            print("Stop")
            mult.current_process().kill()
            return


        # population
        cls._pop_ax.clear()
        cls._pop_ax.plot(cls._horizontal_ticks[:len(aggr.count_stat_pop)], aggr.count_stat_pop, 'b')
        cls._pop_ax.plot(cls._horizontal_ticks[:len(aggr.count_stat_plant)], aggr.count_stat_plant, 'g')

        m = max(max(aggr.count_stat_pop), max(aggr.count_stat_plant))
        high_point = m - m % 10 + 10
        cls._pop_ax.legend(handles=cls._legends["pop"])
        cls._pop_ax.set(xlim=(0, NUMBER_TICK), xticks=np.linspace(0, NUMBER_TICK, 9),
                        yticks=np.linspace(0, high_point, 10))
        cls._pop_ax.set_title('Population',
                              loc='center', )

        # generation
        cls._gen_ax.clear()
        count = aggr.count_gen_stat_pop.most_common()
        keys = [f"Gen:{k[0]}" for k in count[:10]]
        value = [k[1] for k in count]
        keys.append("others")
        value = value[:10] + [sum(value[10:]),]
        cls._gen_ax.barh(keys, value)
        cls._gen_ax.invert_yaxis()

        cls._gen_ax.xaxis.set_ticks_position('none')
        cls._gen_ax.yaxis.set_ticks_position('none')
        cls._gen_ax.set_title('Generations',
                              loc='center', )
        for i in cls._gen_ax.patches:
            plt.text(i.get_width() + 0.2, i.get_y() + 0.5,
                     str(round((i.get_width()), 2)),
                     fontsize=10, fontweight='bold',
                     color='grey')

        # age of oldest
        cls._age_ax.clear()
        cls._age_ax.plot(cls._horizontal_ticks[:len(aggr.max_age_stat_pop)], aggr.max_age_stat_pop, 'y')

        m = max(aggr.max_age_stat_pop)
        high_point = m - m % 10 + 10
        cls._age_ax.set(xlim=(0, NUMBER_TICK), xticks=np.linspace(0, NUMBER_TICK, 9),
                        yticks=np.linspace(0, high_point, 10))
        cls._age_ax.set_title('Age of oldest codekaryote',
                              loc='center', )




# end class Displayer

world = None

class Aggregator(object):
    """
    A class for aggregating data from the simulation data
    """

    def __init__(self):
        global conn
        self.data = []
        self._time = 0.5
        self.count_stat_pop = Buffer(maxlen=NUMBER_TICK)
        self.count_stat_plant = Buffer(maxlen=NUMBER_TICK)
        self.max_age_stat_pop = Buffer(maxlen=NUMBER_TICK)
        self.count_gen_stat_pop = Counter()
        self.count_gen_stat_plant = Counter()

    # noinspection PyUnresolvedReferences
    def run(self):
        while True:
            time.sleep(self._time)
            snapshot_pop = world.creature.copy()
            snapshot_plant = world.plant.copy()

            if len(snapshot_pop) == 0:
                continue

            self.count_stat_pop.put(len(snapshot_pop))
            self.count_stat_plant.put(len(snapshot_plant))

            self.count_gen_stat_pop = Counter([o.ancestry.generation for o in snapshot_pop])
            #self.count_gen_stat_plant = Counter([o.ancestry.generation for o in snapshot_plant])  #reactivate with plant evolution

            self.max_age_stat_pop.put(max([o.ancestry.age for o in snapshot_pop]))

            conn.send(self)
    # end while
# end def run


class Buffer(deque):
    """Buffer containing a fix number of elements"""

    def __init__(self, iterable=(), maxlen=10):
        super().__init__(maxlen=maxlen, iterable=iterable)
    # end def __init__

    # -------------------Methods--------------------

    def put(self, val):
        """
                Add a new value at the end
                :param val: the value to add
                :type val:
                :return:
                :rtype:
                """
        if len(self) == self.maxlen:
            self.rotate(-1)
            self.pop()
        self.append(val)

    # -----------------Properties------------------


def start_thread():
    global conn, world
    world = World()
    thread_con, conn = mult.Pipe()

    process = mult.Process(target=Displayer.run, args=(thread_con,), name="statistical analysis")

    agg = Aggregator()
    thread = threading.Thread(target=Aggregator.run, args=(agg,))
    thread.start()
    process.start()

# end def start_thread
