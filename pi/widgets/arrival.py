
## From https://github.com/metro-sign/dc-metro/blob/main/src/metro_api.py
from led import get_matrix, get_frame_canvas
from widgets import Widget
import logging
from db import get_firebase
from utils import MetroApi
from datetime import datetime

try:
    from rgbmatrix import graphics
except ImportError:
    from RGBMatrixEmulator import graphics

class ArrivalWidget(Widget):

    sleep_seconds = 10
    LINE_HEIGHT = 10
    LINE_HEIGHT_WITH_PADDING = 12
    WIDGET_NAME = 'DCMetroTrainArrivalWidget'
    DEFAULT_STATION = 'D02'
    MAX_DISPLAY = 4

    def __init__(self):
        self.matrix = get_matrix()
        self.offscreen_canvas = get_frame_canvas()
        self.font = graphics.Font()
        self.font.LoadFont("7x14.bdf")  # line height is 10
        self.headerColor = graphics.Color(120, 120, 120)
        self.white = graphics.Color(255, 255, 255)
        self.firebase = get_firebase()
        
    def get_fb_config(self):
        widgets = self.firebase.get("/widgets", None)
        for widgetId in widgets:
            if widgets[widgetId]['name'] == self.WIDGET_NAME:
                logging.debug(widgets[widgetId])
                return widgets[widgetId]

    def get_station(self):
        fb_obj = self.get_fb_config()
        if fb_obj is not None and 'station_id' in fb_obj:
            return fb_obj['station_id']
        logging.warn("Something is wrong with the firebase object. Falling back to default")
        return self.DEFAULT_STATION

    def get_custom_messages(self):
        fb_obj = self.get_fb_config()
        if fb_obj is not None and 'messages' in fb_obj:
            return fb_obj['messages']
        return []

    def get_lines_to_display(self):
        station = self.get_station()
        train_data_1 = MetroApi.fetch_train_predictions(station, '1')
        train_data_2 = MetroApi.fetch_train_predictions(station, '2')
        
        train_data = train_data_1 + train_data_2
        train_data = sorted(train_data, key=MetroApi._sort)[:self.MAX_DISPLAY]

        custom_messages = self.get_custom_messages()

        # Insert all non-sticky where they should be in the sorted list
        for message in custom_messages:
            arrival_time = datetime.fromtimestamp(message['time'] / 1000.0)
            if datetime.now() <= arrival_time:
                test = arrival_time - datetime.now()
                arrival_msg = str(test.seconds // 60)
                if arrival_msg == '0':
                    arrival_msg = 'ARR'
                train_data.append({
                    'line': '-',
                    'line_color': MetroApi._get_line_color('TS'),
                    'destination': message['message'],
                    'arrival': arrival_msg,
                    'arrival_timestamp': arrival_time,
                    'sticky': message['sticky'] == 'true'
                })
        train_data.sort(key=MetroApi._sort)

        # Pop extra items that aren't sticky
        while len(train_data) > self.MAX_DISPLAY:
            # If we only have sticky messages, break out
            if all([msg.get('sticky', False) for msg in train_data]):
                break

            # Otherwise we have real trains / non sticky messages. Remove them!
            # Find the first non-sticky at the end of the list
            for i in range(len(train_data)):
                if not train_data[i].get('sticky', False):
                    train_data.pop(i)
                    break

        return train_data

    async def update(self):
        data = self.get_lines_to_display()
        logging.debug(data)

        self.offscreen_canvas.Clear()

        # header
        graphics.DrawText(self.offscreen_canvas, self.font, 1,
                          self.LINE_HEIGHT, self.headerColor, "LN  DEST    LV MIN")

        for index, train in enumerate(data):
            graphics.DrawLine(self.offscreen_canvas, 1, self.LINE_HEIGHT_WITH_PADDING * (index + 2), 1, (self.LINE_HEIGHT_WITH_PADDING * (index + 1) + (self.LINE_HEIGHT_WITH_PADDING - self.LINE_HEIGHT)), train['line_color'])
            graphics.DrawLine(self.offscreen_canvas, 2, self.LINE_HEIGHT_WITH_PADDING * (index + 2), 2, (self.LINE_HEIGHT_WITH_PADDING * (index + 1) + (self.LINE_HEIGHT_WITH_PADDING - self.LINE_HEIGHT)), train['line_color'])
            graphics.DrawText(self.offscreen_canvas, self.font, 5, self.LINE_HEIGHT_WITH_PADDING * (index + 2), train['line_color'], train['line'])
            graphics.DrawText(self.offscreen_canvas, self.font, 29, self.LINE_HEIGHT_WITH_PADDING * (index + 2), self.white, train['destination'])
            arrival = train['arrival']
            if str(arrival).isnumeric():
                arrival_int = int(arrival)
                arrival = str(min(arrival_int, 999))
                if arrival_int > 15 and arrival_int < 99:
                    leave = str(arrival_int - 15)
                    graphics.DrawText(self.offscreen_canvas, self.font, 84, self.LINE_HEIGHT_WITH_PADDING * (index + 2), self.white, leave)
                else:
                    graphics.DrawText(self.offscreen_canvas, self.font, 84, self.LINE_HEIGHT_WITH_PADDING * (index + 2), self.white, '- ')
            graphics.DrawText(self.offscreen_canvas, self.font, 106, self.LINE_HEIGHT_WITH_PADDING * (index + 2), self.white, arrival)

        self.offscreen_canvas = self.matrix.SwapOnVSync(self.offscreen_canvas)
