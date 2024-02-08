import dotenv
dotenv.load_dotenv()

import logging
import argparse
from widgets.arrival import ArrivalWidget
import asyncio
from led import loading_generator

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


async def try_connect():
    while True:
        print("hi")
        await asyncio.sleep(5)


async def loading():
    async def print_loading():
        generator = loading_generator()
        while True:
            next(generator)
            await asyncio.sleep(0.05)
    await asyncio.gather(print_loading(), try_connect())


async def main():
    await loading()

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
