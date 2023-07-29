
## From https://github.com/metro-sign/dc-metro/blob/main/src/metro_api.py

from led import get_matrix, get_frame_canvas

from widgets import Widget
from utils import MetroApi

from rgbmatrix import graphics
import logging
from db import get_firebase

class ArrivalWidget(Widget):

    sleep_seconds = 10
    LINE_HEIGHT = 10
    LINE_HEIGHT_WITH_PADDING = 12
    WIDGET_NAME = 'DCMetroTrainArrivalWidget'
    DEFAULT_STATION = 'D02'

    def __init__(self):
        self.matrix = get_matrix()
        self.offscreen_canvas = get_frame_canvas()
        self.font = graphics.Font()
        self.font.LoadFont("7x14.bdf")  # line height is 10
        self.headerColor = graphics.Color(120, 120, 120)
        self.white = graphics.Color(255, 255, 255)
        self.firebase = get_firebase()
        
    async def get_station(self):
        widgets = await self.firebase.get_async("/widgets", None)
        for widgetId in widgets:
            if widgets[widgetId]['name'] == self.WIDGET_NAME and 'station_id' in widgets[widgetId]:
                logging.debug(widgets[widgetId])
                return widgets[widgetId]['station_id']
        logging.warn("Something is wrong with the firebase object. Falling back to default")
        return self.DEFAULT_STATION

    async def update(self):
        station = await self.get_station()
        data = MetroApi.fetch_train_predictions(station, '2')
        logging.debug(data)

        self.offscreen_canvas.Clear()

        # header
        graphics.DrawText(self.offscreen_canvas, self.font, 1,
                          self.LINE_HEIGHT, self.headerColor, "LN  DEST       MIN")

        for index, train in enumerate(data[:4]):
            graphics.DrawLine(self.offscreen_canvas, 1, self.LINE_HEIGHT_WITH_PADDING * (index + 2), 1, (self.LINE_HEIGHT_WITH_PADDING * (index + 1) + (self.LINE_HEIGHT_WITH_PADDING - self.LINE_HEIGHT)), train['line_color'])
            graphics.DrawLine(self.offscreen_canvas, 2, self.LINE_HEIGHT_WITH_PADDING * (index + 2), 2, (self.LINE_HEIGHT_WITH_PADDING * (index + 1) + (self.LINE_HEIGHT_WITH_PADDING - self.LINE_HEIGHT)), train['line_color'])
            graphics.DrawText(self.offscreen_canvas, self.font, 5, self.LINE_HEIGHT_WITH_PADDING * (index + 2), train['line_color'], train['line'])
            graphics.DrawText(self.offscreen_canvas, self.font, 29, self.LINE_HEIGHT_WITH_PADDING * (index + 2), self.white, train['destination'])
            graphics.DrawText(self.offscreen_canvas, self.font, 106, self.LINE_HEIGHT_WITH_PADDING * (index + 2), self.white, train['arrival'])

        self.offscreen_canvas = self.matrix.SwapOnVSync(self.offscreen_canvas)
