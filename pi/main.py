import dotenv
dotenv.load_dotenv()

import logging
import argparse
from widgets.arrival import ArrivalWidget
import asyncio
from led import loading_generator, LoadingData, plain_text
from network import try_connect

try:
    import rgbmatrix
except ImportError:
    print("Hey! Looks like you are emulating the RGBMatrix. Enabling nest_asyncio.")
    import nest_asyncio
    nest_asyncio.apply()

def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("-v", "--verbose", action="store_true", default=False)
    return parser.parse_args()


async def wait_for_network(data):
    while True:
        ssid = await try_connect(data)
        if ssid is not None:
            data.should_exit = True
            return
        await asyncio.sleep(1)

async def print_loading(data):
    generator = loading_generator(data)
    while not data.should_exit:
        next(generator)
        await asyncio.sleep(0.02)


async def sleep_then_terminate(data, time):
    await asyncio.sleep(time)
    data.should_exit = True

async def startup():
    data = LoadingData(line1="WMATA Metro Tracker", line2="by Noah Gearhart", line3="for Dark Wolf")

    await asyncio.gather(print_loading(data), sleep_then_terminate(data, 5))
    data.should_exit = False
    await asyncio.gather(print_loading(data), wait_for_network(data))
    data.should_exit = False
    await asyncio.gather(print_loading(data), sleep_then_terminate(data, 5))
    plain_text("", "Starting", "", 100, 100, 100)
    await asyncio.sleep(1)


async def main():
    await startup()

    widgets = [
        ArrivalWidget()
    ]

    tasks = [widget.loop() for widget in widgets]

    await asyncio.gather(*tasks)

if __name__ == '__main__':
    args = parse_args()
    logging.basicConfig(level=logging.DEBUG if args.verbose else logging.INFO)

    logging.info("Starting arrival sign manager! Press Ctrl+C to exit.")
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        logging.info("Exiting due to Ctrl+C")
