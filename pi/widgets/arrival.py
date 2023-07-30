
## From https://github.com/metro-sign/dc-metro/blob/main/src/metro_api.py
import requests
import os
import time
from led import get_matrix, get_frame_canvas

try:
    from rgbmatrix import graphics
except ImportError:
    from RGBMatrixEmulator import graphics

class MetroApiOnFireException(Exception):
    pass

API_URL = "https://api.wmata.com/StationPrediction.svc/json/GetPrediction/"
API_KEY = os.environ["WMATA_API_KEY"]
RETRIES = 5

class MetroApi:
    def fetch_train_predictions(station_code: str, group: str) -> list[dict]:
        return MetroApi._fetch_train_predictions(station_code, group, retry_attempt=0)

    def _fetch_train_predictions(station_code: str, group: str, retry_attempt: int) -> list[dict]:
        try:
            api_url = API_URL + station_code
            train_data = requests.get(api_url, headers={
                'api_key': API_KEY
            }).json()

            print('Received response from WMATA api...')

            trains = filter(lambda t: t['Group'] == group, train_data['Trains'])

            normalized_results = list(map(MetroApi._normalize_train_response, trains))

            return normalized_results
        except RuntimeError:
            if retry_attempt < RETRIES:
                print('Failed to connect to WMATA API. Reattempting...')
                # Recursion for retry logic because I don't care about your stack
                return MetroApi._fetch_train_predictions(station_code, group, retry_attempt + 1)
            else:
                raise MetroApiOnFireException()
    
    def _normalize_train_response(train: dict) -> dict:
        line = train['Line']
        destination = train['Destination']
        arrival = train['Min']

        if destination == 'No Passenger' or destination == 'NoPssenger' or destination == 'ssenger':
            destination = 'No Psngr'

        return {
            'line_color': MetroApi._get_line_color(line),
            'destination': destination,
            'arrival': arrival
        }
    
    def _get_line_color(line: str) -> int:
        if line == 'RD':
            return 0xFF0000
        elif line == 'OR':
            return 0xFF5500
        elif line == 'YL':
            return 0xFFFF00
        elif line == 'GR':
            return 0x00FF00
        elif line == 'BL':
            return 0x0000FF
        else:
            return 0xAAAAAA

def test():
    print(MetroApi.fetch_train_predictions('D02', '2'))

def update_screen():
    matrix = get_matrix()
    offscreen_canvas = get_frame_canvas()
    font = graphics.Font()
    font.LoadFont("../5x7.bdf")
    textColor = graphics.Color(255, 255, 0)
    pos = offscreen_canvas.width
    my_text = "test"

    while True:
        offscreen_canvas.Clear()
        len = graphics.DrawText(offscreen_canvas, font, pos, 10, textColor, my_text)
        pos -= 1
        if (pos + len < 0):
            pos = offscreen_canvas.width

        time.sleep(0.05)
        offscreen_canvas = matrix.SwapOnVSync(offscreen_canvas)
