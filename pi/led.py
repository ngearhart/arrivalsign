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


def loading_generator(length=5, depth=3):
    matrix = get_matrix()
    offscreen_canvas = get_frame_canvas()
    # font = graphics.Font()
    # font.LoadFont("7x14.bdf")  # line height is 10
    headerColor = graphics.Color(120, 120, 120)
    primary = graphics.Color(255, 255, 255)
    index = 0
    while True:
        for x in range(length):
            for y in range(depth):
                offscreen_canvas.SetPixel(index + x, y, primary.red, primary.green, primary.blue)
        offscreen_canvas = matrix.SwapOnVSync(offscreen_canvas)
        index += 1
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
