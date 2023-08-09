#!/usr/bin/env python
# Display a runtext with double-buffering.
from rgbmatrix import graphics
import time


import argparse
import time
from functools import cache
import sys
import os

sys.path.append(os.path.abspath(os.path.dirname(__file__) + '/..'))
from rgbmatrix import RGBMatrix, RGBMatrixOptions


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
