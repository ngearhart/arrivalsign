from abc import ABC, abstractmethod
from asyncio import sleep
import logging
from requests import exceptions
from led import get_matrix, get_frame_canvas
from db import firebase_get_widget_config
try:
    from rgbmatrix import graphics
except ImportError:
    from RGBMatrixEmulator import graphics

class Widget(ABC):

    sleep_seconds = 1
    should_display = True
    widget_list = []

    @abstractmethod
    async def update(self):
        pass

    async def _update(self):
        if self.should_display:
            logging.debug(f"Running update method on widget {type(self).__name__}")
            return await self.update()

    async def loop(self):
        while True:
            await self._update()
            await sleep(self.sleep_seconds)

    def __init__(self, widget_list):
        self.matrix = get_matrix()
        self.offscreen_canvas = get_frame_canvas()
        self.font = graphics.Font()
        self.font.LoadFont("7x14.bdf")  # line height is 10
        self.headerColor = graphics.Color(120, 120, 120)
        self.white = graphics.Color(255, 255, 255)
        self.widget_list = widget_list
        widget_list.append(self)

    async def get_fb_config(self):
        widgets = await firebase_get_widget_config()
        for widgetId in widgets:
            if widgets[widgetId]['name'] == self.WIDGET_NAME:
                logging.debug(widgets[widgetId])
                return widgets[widgetId]
        return None
