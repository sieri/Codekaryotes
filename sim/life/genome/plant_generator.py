from random import randint


def generate_plant():
    energy = [randint(0, 4303355903) for _ in range(1)]

    return energy


def generate_plant_color():
    return [0, 255, 0]
