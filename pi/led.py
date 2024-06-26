#!/usr/bin/env python
# Display a runtext with double-buffering.
# from rgbmatrix import graphics
from math import sqrt,cos,sin,radians

from dataclasses import dataclass
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

    options.hardware_mapping = 'adafruit-hat-pwm'
    options.rows = 32
    options.cols = 64
    options.chain_length = 4
    options.pixel_mapper_config = 'U-mapper'
    options.drop_privileges = True
    # options.show_refresh_rate = True
    options.limit_refresh_rate_hz = 60
    options.pwm_dither_bits = 1
    options.pwm_lsb_nanoseconds = 50
    options.pwm_bits = 7

    return RGBMatrix(options = options)

@cache
def get_frame_canvas():
    return get_matrix().CreateFrameCanvas()


LENGTH = 64 * 2
WIDTH = 32 * 2

@dataclass
class LoadingData:
    # I know this could be an array - deal with it
    line1: str = ''
    line2: str = ''
    line3: str = ''
    should_exit: bool = False
    loading_index: int = LENGTH / 2  # start in the middle


def loading_generator(data: LoadingData, length=5, depth=3):
    matrix = get_matrix()
    offscreen_canvas = get_frame_canvas()
    font = graphics.Font()
    font.LoadFont("6x10.bdf")
    falloff = 50
    multiplier = 0.9
    rotation = RGBRotate()
    rotation.set_hue_rotation(5)
    original = graphics.Color(0, 255, 0)
    while not data.should_exit:
        offscreen_canvas.Clear()
        new_r, new_g, new_b = rotation.apply(original.red, original.green, original.blue)
        original = graphics.Color(new_r, new_g, new_b)
        for x in range(length):
            set_pixel_along_border(offscreen_canvas, data.loading_index + x, depth, original)
            set_pixel_along_border(offscreen_canvas, data.loading_index + x + LENGTH, depth, original)
            set_pixel_along_border(offscreen_canvas, data.loading_index + x + LENGTH * 2, depth, original)
            set_pixel_along_border(offscreen_canvas, data.loading_index + x + LENGTH * 3, depth, original)
        
        r, g, b = color_adjust_brightness(original, multiplier, True)
        trail = graphics.Color(r, g, b)
        for x in range(falloff):
            r, g, b = color_adjust_brightness(trail, multiplier, True)
            r, g, b = rotation.apply(r, g, b)
            trail = graphics.Color(r, g, b)
            set_pixel_along_border(offscreen_canvas, data.loading_index - x, depth, trail)
            set_pixel_along_border(offscreen_canvas, data.loading_index - x + LENGTH, depth, trail)
            set_pixel_along_border(offscreen_canvas, data.loading_index - x + LENGTH * 2, depth, trail)
            set_pixel_along_border(offscreen_canvas, data.loading_index - x + LENGTH * 3, depth, trail)

        graphics.DrawText(offscreen_canvas, font, 7,
                          24, graphics.Color(210, 210, 210), data.line1.center(20))
        graphics.DrawText(offscreen_canvas, font, 7,
                          35, graphics.Color(210, 210, 210), data.line2.center(20))
        graphics.DrawText(offscreen_canvas, font, 7,
                          46, graphics.Color(210, 210, 210), data.line3.center(20))
        # for y in range(depth):
        #     primary = graphics.Color(255, 255, 255)
        #     for x in range(length):
        #         offscreen_canvas.SetPixel(index + x, y, primary.red, primary.green, primary.blue)
        #     for x in range(falloff):
        #         color_adjust_brightness(primary, multiplier, True)
        #         offscreen_canvas.SetPixel(index - x, y, primary.red, primary.green, primary.blue)
        offscreen_canvas = matrix.SwapOnVSync(offscreen_canvas)
        data.loading_index += 1
        yield


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
    r = color.red   * alpha
    g = color.green * alpha
    b = color.blue  * alpha

    if to_int:
        r   = int(r)
        g = int(g)
        b  = int(b)
    return r, g, b

def clamp(v):
    if v < 0:
        return 0
    if v > 255:
        return 255
    return int(v + 0.5)

class RGBRotate(object):
    def __init__(self):
        self.matrix = [[1,0,0],[0,1,0],[0,0,1]]

    def set_hue_rotation(self, degrees):
        cosA = cos(radians(degrees))
        sinA = sin(radians(degrees))
        self.matrix[0][0] = cosA + (1.0 - cosA) / 3.0
        self.matrix[0][1] = 1./3. * (1.0 - cosA) - sqrt(1./3.) * sinA
        self.matrix[0][2] = 1./3. * (1.0 - cosA) + sqrt(1./3.) * sinA
        self.matrix[1][0] = 1./3. * (1.0 - cosA) + sqrt(1./3.) * sinA
        self.matrix[1][1] = cosA + 1./3.*(1.0 - cosA)
        self.matrix[1][2] = 1./3. * (1.0 - cosA) - sqrt(1./3.) * sinA
        self.matrix[2][0] = 1./3. * (1.0 - cosA) - sqrt(1./3.) * sinA
        self.matrix[2][1] = 1./3. * (1.0 - cosA) + sqrt(1./3.) * sinA
        self.matrix[2][2] = cosA + 1./3. * (1.0 - cosA)

    def apply(self, r, g, b):
        rx = r * self.matrix[0][0] + g * self.matrix[0][1] + b * self.matrix[0][2]
        gx = r * self.matrix[1][0] + g * self.matrix[1][1] + b * self.matrix[1][2]
        bx = r * self.matrix[2][0] + g * self.matrix[2][1] + b * self.matrix[2][2]
        return clamp(rx), clamp(gx), clamp(bx)


def plain_text(line1: str, line2: str, line3: str, r, g, b):
    matrix = get_matrix()
    offscreen_canvas = get_frame_canvas()
    font = graphics.Font()
    font.LoadFont("7x14.bdf")  # line height is 10
    rotation = RGBRotate()
    rotation.set_hue_rotation(5)
    offscreen_canvas.Clear()
    graphics.DrawText(offscreen_canvas, font, 10,
                        20, graphics.Color(r, g, b), line1.center(15))
    graphics.DrawText(offscreen_canvas, font, 10,
                        35, graphics.Color(r, g, b), line2.center(15))
    graphics.DrawText(offscreen_canvas, font, 10,
                        50, graphics.Color(r, g, b), line3.center(15))
    matrix.SwapOnVSync(offscreen_canvas)

@dataclass
class AlertData:
    line1: str = ''
    line2: str = ''
    line3: str = ''
    should_exit: bool = False

def alert_generator(data: AlertData, box_size: int=8):
    matrix = get_matrix()
    offscreen_canvas = get_frame_canvas()
    font = graphics.Font()
    font.LoadFont("5x7.bdf")
    color = graphics.Color(255, 255, 0)

    flip = False

    while True:
        offscreen_canvas.Clear()
        offset = 0 if flip else 1
        for i in range(LENGTH // (box_size * 2)):
            for x in range(box_size):
                set_pixel_along_border(offscreen_canvas, (((i * 2) + offset) * box_size) + x, box_size, color)
                set_pixel_along_border(offscreen_canvas, (((i * 2) + offset) * box_size) + x + (LENGTH + WIDTH), box_size, color)

        # Max width 25 characters with this font.
        # Short by like 3 pixels to be 26.
        graphics.DrawText(offscreen_canvas, font, 2,
                          24, graphics.Color(210, 210, 210), data.line1.center(25))
        graphics.DrawText(offscreen_canvas, font, 2,
                          35, graphics.Color(210, 210, 210), data.line2.center(25))
        graphics.DrawText(offscreen_canvas, font, 2,
                          46, graphics.Color(210, 210, 210), data.line3.center(25))
        offscreen_canvas = matrix.SwapOnVSync(offscreen_canvas)
        flip = not flip
        yield


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
