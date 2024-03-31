import requests
import os
import time
from led import get_matrix, get_frame_canvas
from datetime import datetime, timedelta
import asyncio
from functools import wraps, partial

from functools import cache
import logging

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

    def fetch_all_for_station(station_code: str, up_to=2) -> list[dict]:
        data = []
        for i in range(up_to):
            data.extend(MetroApi._fetch_train_predictions(station_code, str(i + 1), retry_attempt=0))
        data.sort(key=MetroApi._sort)
        return data

    def _sort(data):
        if data['arrival'].isnumeric():
            return int(data['arrival'])
        if data['arrival'] == 'ARR':
            return -1
        if data['arrival'] == 'BRD':
            return -2
        return -1000

    def _fetch_train_predictions(station_code: str, group: str, retry_attempt: int) -> list[dict]:
        try:
            api_url = API_URL + station_code
            train_data = requests.get(api_url, headers={
                'api_key': API_KEY
            }).json()

            logging.debug('Received response from WMATA api.')

            trains = filter(lambda t: t['Group'] == group, train_data['Trains'])

            normalized_results = list(map(MetroApi._normalize_train_response, trains))

            return normalized_results
        except RuntimeError:
            if retry_attempt < RETRIES:
                logging.warn('Failed to connect to WMATA API. Reattempting...')
                # Recursion for retry logic because I don't care about your stack
                return MetroApi._fetch_train_predictions(station_code, group, retry_attempt + 1)
            else:
                raise MetroApiOnFireException()
    
    def _normalize_train_response(train: dict) -> dict:
        line = train['Line']
        destination = train['Destination']
        arrival = train['Min']

        arrival_is_now = not arrival.isnumeric()

        if destination == 'No Passenger' or destination == 'NoPssenger' or destination == 'ssenger':
            destination = 'No Psngr'

        return {
            'line': line,
            'line_color': MetroApi._get_line_color(line),
            'destination': destination[:8],  # only 8 characters can fit on my display
            'arrival': arrival,
            'arrival_timestamp': datetime.now() + timedelta(minutes=0 if arrival_is_now else int(arrival))
        }
    
    @cache
    def _get_line_color(line: str) -> int:
        if line == 'RD':
            return graphics.Color(255, 0, 0)
            # return 0xFF0000
        elif line == 'OR':
            return graphics.Color(255, 85, 0)
            # return 0xFF5500
        elif line == 'YL':
            return graphics.Color(255, 255, 0)
            # return 0xFFFF00
        elif line == 'GR':
            return graphics.Color(0, 255, 0)
            # return 0x00FF00
        elif line == 'BL':
            return graphics.Color(0, 0, 255)
            # return 0x0000FF
        else:
            return graphics.Color(170, 170, 170)
            # return 0xAAAAAA


def async_wrap(func):
    @wraps(func)
    async def run(*args, loop=None, executor=None, **kwargs):
        if loop is None:
            loop = asyncio.get_event_loop()
        pfunc = partial(func, *args, **kwargs)
        return await loop.run_in_executor(executor, pfunc)
    return run 
