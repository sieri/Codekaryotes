from PIL import Image, ImageDraw, ImageFont

fnt = ImageFont.truetype("arial", 36)
def create_image_with_text(wh, text):
    width, height = wh
    img = Image.new('RGB', (300, 200), "yellow")
    draw = ImageDraw.Draw(img)
    # draw.ellipse takes a 4-tuple (x0, y0, x1, y1) where (x0, y0) is the top-left bound of the box
    # and (x1, y1) is the lower-right bound of the box.
    draw.text((width, height), text, font = fnt, fill="black")
    return img
# Create the frames
frames = []
x, y = 0, 0
for i in range(100):
    new_frame = create_image_with_text((x-100,y), "HELLO")
    frames.append(new_frame)
    x += 4
    y += 1

# Save into a GIF file that loops forever
frames[0].save('moving_text.gif', format='GIF',
               append_images=frames[1:], save_all=True, duration=30, loop=0)