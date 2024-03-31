
## From https://github.com/metro-sign/dc-metro/blob/main/src/metro_api.py
from widgets import Widget
import logging
from utils import MetroApi
from datetime import datetime
from asyncio import sleep, gather
from random import choice
from led import alert_generator, AlertData
from textwrap import wrap

class AlertsWidget(Widget):

    # seconds_per_line = 4
    show_seconds = 10
    sleep_seconds = 600 # 10 minutes
    WIDGET_NAME = 'DCMetroAlertsWidget'

    async def get_custom_messages(self):
        fb_obj = await self.get_fb_config()
        if fb_obj is not None and 'alerts' in fb_obj:
            return fb_obj['alerts']
        return []

    async def sleep_then_terminate(self, data, time):
        await sleep(time)
        data.should_exit = True

    async def print_loading(self, data):
        generator = alert_generator(data)
        while not data.should_exit:
            next(generator)
            await sleep(2)

    async def update(self):
        alerts = await self.get_custom_messages()

        if len(alerts) == 0:
            return

        # Hide all other widgets
        for widget in self.widget_list:
            if widget != self:
                widget.should_display = False

        # Pick a random alert
        alert = choice(alerts)["message"]
        logging.debug(f"Showing alert {alert}")

        max_text_row_size = 25
        lines = wrap(alert, width=max_text_row_size)

        if len(lines) == 1:
            lines.insert('', 0)

        data = AlertData(line2='Metro Alert')
        await gather(self.print_loading(data), self.sleep_then_terminate(data, 3))

        data = AlertData(line1=lines[0], line2=lines[1] if len(lines) > 1 else '', line3=lines[2] if len(lines) > 2 else '')
        await gather(self.print_loading(data), self.sleep_then_terminate(data, self.show_seconds))

        # TODO: Scroll through lines
        # for index, line in enumerate(lines):

        #     await sleep(self.seconds_per_line)

        # Re-show all other widgets
        for widget in self.widget_list:
            if widget != self:
                widget.should_display = True
