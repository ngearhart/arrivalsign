
## From https://github.com/metro-sign/dc-metro/blob/main/src/metro_api.py

from led import get_matrix, get_frame_canvas

from widgets import Widget
from utils import MetroApi

from rgbmatrix import graphics
import logging


class ArrivalWidget(Widget):

    sleep_seconds = 10
    LINE_HEIGHT = 10
    LINE_HEIGHT_WITH_PADDING = 12

    def __init__(self):
        self.matrix = get_matrix()
        self.offscreen_canvas = get_frame_canvas()
        self.font = graphics.Font()
        self.font.LoadFont("7x14.bdf")  # line height is 10
        self.headerColor = graphics.Color(120, 120, 120)
        self.white = graphics.Color(255, 255, 255)

    async def update(self):
        data = MetroApi.fetch_train_predictions('D02', '2')
        logging.debug(data)

        self.offscreen_canvas.Clear()

        # header
        self.graphics.DrawText(self.offscreen_canvas, self.font, 1,
                                self.LINE_HEIGHT, self.headerColor, "LN  DEST       MIN")

        for index, train in enumerate(data[:4]):
            graphics.DrawLine(self.offscreen_canvas, 1, self.LINE_HEIGHT_WITH_PADDING * (index + 2), 1, (self.LINE_HEIGHT_WITH_PADDING * (index + 1) + (self.LINE_HEIGHT_WITH_PADDING - self.LINE_HEIGHT)), train['line_color'])
            graphics.DrawLine(self.offscreen_canvas, 2, self.LINE_HEIGHT_WITH_PADDING * (index + 2), 2, (self.LINE_HEIGHT_WITH_PADDING * (index + 1) + (self.LINE_HEIGHT_WITH_PADDING - self.LINE_HEIGHT)), train['line_color'])
            graphics.DrawText(self.offscreen_canvas, self.font, 5, self.LINE_HEIGHT_WITH_PADDING * (index + 2), train['line_color'], train['line'])
            graphics.DrawText(self.offscreen_canvas, self.font, 29, self.LINE_HEIGHT_WITH_PADDING * (index + 2), self.white, train['destination'])
            graphics.DrawText(self.offscreen_canvas, self.font, 106, self.LINE_HEIGHT_WITH_PADDING * (index + 2), self.white, train['arrival'])

        self.offscreen_canvas = self.matrix.SwapOnVSync(self.offscreen_canvas)
