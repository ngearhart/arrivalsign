import dotenv
dotenv.load_dotenv()

import logging
import argparse
from widgets.arrival import ArrivalWidget
import asyncio

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

async def main():
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
