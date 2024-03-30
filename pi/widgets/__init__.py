from abc import ABC, abstractmethod
from asyncio import sleep
import logging
from requests import exceptions
from led import get_matrix, get_frame_canvas
from db import get_firebase
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
        self.firebase = get_firebase()
        self.widget_list = widget_list
        widget_list.append(self)

    async def _firebase_get_with_retries(self, url, name):
        RETRIES = 5
        SLEEP_SECONDS = 5
        for _ in range(RETRIES):
            try:
                return self.firebase.get(url, name)
            except exceptions.ConnectionError:
                logging.warn("Firebase connection error, retrying...")
                await sleep(SLEEP_SECONDS)
        return None

    async def get_fb_config(self):
        # The following is fraught for error
        widgets = await self._firebase_get_with_retries("/widgets", None)
        for widgetId in widgets:
            if widgets[widgetId]['name'] == self.WIDGET_NAME:
                logging.debug(widgets[widgetId])
                return widgets[widgetId]
        return None
