import PIL.Image
import pygame.image

import tempfile
images = []


def take_capture():
    img = tempfile.TemporaryFile()
    pygame.image.save(pygame.display.get_surface(), img)
    img.flush()
    images.append(img)
# def take_capture


def save():
    frames = []
    for img_file in images:
        frames.append(PIL.Image.open(img_file))
    # end for

    frames[0].save('moving_text.gif', format='GIF',
                   append_images=frames[1:], save_all=True, duration=30, loop=1)