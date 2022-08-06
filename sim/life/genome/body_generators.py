from random import randint


def generate_eyes():
    fov = [randint(0, 360), ]
    r = [randint(0, 100), ]

    return fov+r


def generate_organism_color():
    return [randint(0, 255) for _ in range(3)]


def generate_energy_storage():
    return [randint(0, 4303355903), ]


def generate_size():
    return [randint(0, 4303355903), ]
