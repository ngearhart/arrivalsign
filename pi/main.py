import dotenv
dotenv.load_dotenv()

from widgets.arrival import ArrivalWidget
import asyncio


async def main():
    widgets = [
        ArrivalWidget()
    ]

    tasks = [widget.loop() for widget in widgets]

    await asyncio.gather(*tasks)

if __name__ == '__main__':
    asyncio.run(main())
