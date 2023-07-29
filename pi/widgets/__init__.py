from abc import ABC, abstractmethod
from asyncio import sleep

class Widget(ABC):

    sleep_seconds = 1

    @abstractmethod
    async def update(self):
        pass

    async def loop(self):
        while True:
            await self.update()
            await sleep(self.sleep_seconds)
