import dotenv
dotenv.load_dotenv()

import logging
import argparse
from widgets.arrival import ArrivalWidget
import asyncio

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
    logging.basicConfig(level=args.verbose)

    logging.info("Starting arrival sign manager! Press Ctrl+C to exit.")
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        logging.info("Exiting due to Ctrl+C")
