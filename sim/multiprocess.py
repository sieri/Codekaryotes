import itertools
import multiprocessing as mp

import numpy as np

processes = []


class SimProcess:

    def __init__(self):
        self._parent_conn, self._child_conn = mp.Pipe()
        self._process = mp.Process(target=SimProcess.run, args=(self, ))
        self._process.start()

        # Create tuples of (multiprocessing.Array, numpy.ndarray) referencing the same underlying buffers

    # end def __init__

    # -------------------Methods--------------------

    def run(self):
        print(f"starting process {mp.process.current_process()}")
        while mp.parent_process().is_alive():
            creatures = self._child_conn.recv()
            new_pos = []
            for c in creatures:
                if c is not None:
                    c.update()
                    new_pos.append(c.position)
            self._child_conn.send(new_pos)
    # end def run

    def get_values(self, batch):
        new_pos = self._parent_conn.recv()
        for pos, c in zip(new_pos, batch):
            c.position = pos

    # -----------------Properties------------------

    @property
    def creatures(self):
        return None
    # end def creatures

    @creatures.setter
    def creatures(self, value):
        self._parent_conn.send(value)
    # end def creatures

# end class SimProcess


def start_processes():
    global processes
    count = mp.cpu_count()
    print(f"number of cpu: {count}")

    processes = [SimProcess() for _ in range(count)]
# end def start_processes


def dispatch(batches):
    i = 0
    for batch, process in itertools.zip_longest(batches, processes):
        if process is None:
            process = processes[i]
            i = (i+1) % len(processes)
        # end if

        if batch is not None:
            process.creatures = batch
        # end if

    for batch, process in itertools.zip_longest(batches, processes):
        if process is None:
            process = processes[i]
            i = (i+1) % len(processes)
        # end if

        if batch is not None:
            process.get_values(batch)
        # end if


# end def dispatch
