#!/usr/bin/env python
# Display a runtext with double-buffering.
# from rgbmatrix import graphics

import time
from functools import cache
import sys
import os
from asyncio import sleep

sys.path.append(os.path.abspath(os.path.dirname(__file__) + '/..'))
try:
    from rgbmatrix import RGBMatrix, RGBMatrixOptions, graphics
except ImportError:
    from RGBMatrixEmulator import RGBMatrix, RGBMatrixOptions, graphics

@cache
def get_matrix() -> RGBMatrix:
    options = RGBMatrixOptions()

    options.rows = 32
    options.cols = 64
    options.chain_length = 4
    options.pixel_mapper_config = 'U-mapper'
    options.drop_privileges = True
    options.limit_refresh_rate_hz = 60

    return RGBMatrix(options = options)

@cache
def get_frame_canvas():
    return get_matrix().CreateFrameCanvas()


def loading_generator(length=5, depth=4):
    matrix = get_matrix()
    offscreen_canvas = get_frame_canvas()
    # font = graphics.Font()
    # font.LoadFont("7x14.bdf")  # line height is 10
    headerColor = graphics.Color(120, 120, 120)
    index = LENGTH / 2  # start in the middle
    falloff = 50
    multiplier = 0.9
    while True:
        offscreen_canvas.Clear()
        primary = graphics.Color(255, 255, 255)

        for x in range(length):
            set_pixel_along_border(offscreen_canvas, index + x, depth, primary)
            set_pixel_along_border(offscreen_canvas, index + x + LENGTH, depth, primary)
            set_pixel_along_border(offscreen_canvas, index + x + LENGTH * 2, depth, primary)
            set_pixel_along_border(offscreen_canvas, index + x + LENGTH * 3, depth, primary)
        for x in range(falloff):
            color_adjust_brightness(primary, multiplier, True)
            set_pixel_along_border(offscreen_canvas, index - x, depth, primary)
            set_pixel_along_border(offscreen_canvas, index - x + LENGTH, depth, primary)
            set_pixel_along_border(offscreen_canvas, index - x + LENGTH * 2, depth, primary)
            set_pixel_along_border(offscreen_canvas, index - x + LENGTH * 3, depth, primary)

        # for y in range(depth):
        #     primary = graphics.Color(255, 255, 255)
        #     for x in range(length):
        #         offscreen_canvas.SetPixel(index + x, y, primary.red, primary.green, primary.blue)
        #     for x in range(falloff):
        #         color_adjust_brightness(primary, multiplier, True)
        #         offscreen_canvas.SetPixel(index - x, y, primary.red, primary.green, primary.blue)
        offscreen_canvas = matrix.SwapOnVSync(offscreen_canvas)
        index += 1
        yield


LENGTH = 64 * 2
WIDTH = 32 * 2

def set_pixel_along_border(canvas, x, depth, color):
    x = x % (LENGTH * 2 + WIDTH * 2)
    # Top section
    if 0 <= x < LENGTH:
        for y in range(depth):
            canvas.SetPixel(x, y, color.red, color.green, color.blue)
    # Bottom section
    elif LENGTH + WIDTH <= x < LENGTH * 2 + WIDTH:
        for y in range(depth):
            canvas.SetPixel(LENGTH * 2 + WIDTH - x, WIDTH - y - 1, color.red, color.green, color.blue)
    # Right side
    elif LENGTH <= x < LENGTH + WIDTH:
        for y in range(depth):
            canvas.SetPixel(LENGTH - y - 1, x - LENGTH, color.red, color.green, color.blue)
    # Left side
    # elif LENGTH * 2 + WIDTH <= x < LENGTH * 2 + WIDTH * 2:
    else:
        for y in range(depth):
            canvas.SetPixel(y, LENGTH * 2 + WIDTH * 2 - x, color.red, color.green, color.blue)
    # else:
    #     print(x)


def color_adjust_brightness(color, alpha, to_int = False):
    color.red   *= alpha
    color.green *= alpha
    color.blue  *= alpha

    if to_int:
        color.red   = int(color.red)
        color.green = int(color.green)
        color.blue  = int(color.blue)

# offscreen_canvas = self.matrix.CreateFrameCanvas()
# font = graphics.Font()
# font.LoadFont("../../../fonts/7x13.bdf")
# textColor = graphics.Color(255, 255, 0)
# pos = offscreen_canvas.width
# my_text = self.args.text

# while True:
#     offscreen_canvas.Clear()
#     len = graphics.DrawText(offscreen_canvas, font, pos, 10, textColor, my_text)
#     pos -= 1
#     if (pos + len < 0):
#         pos = offscreen_canvas.width

#     time.sleep(0.05)
#     offscreen_canvas = self.matrix.SwapOnVSync(offscreen_canvas)
