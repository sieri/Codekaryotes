from random import randint


def generate_eyes():
    fov = [randint(0, 360), ]
    r = [randint(0, 100), ]

    return fov+r
