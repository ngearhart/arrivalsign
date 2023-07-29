from widgets.arrival import ArrivalWidget
import asyncio

import dotenv

async def main():
    widgets = [
        ArrivalWidget()
    ]

    tasks = [widget.loop() for widget in widgets]

    await asyncio.gather(tasks)

if __name__ == '__main__':
    dotenv.load_dotenv()
    asyncio.run(main())
