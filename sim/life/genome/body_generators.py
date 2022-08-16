from random import randint

MAX_32 = 4303355903

def generate_eyes():
    fov = [randint(0, MAX_32), ]
    r = [randint(0, MAX_32), ]

    return fov+r


def generate_organism_color():
    return [randint(0, 255) for _ in range(3)]


def generate_energy_storage():
    return [randint(0, MAX_32), ]


def generate_size():
    return [randint(0, MAX_32), ]
