from abc import ABC, abstractmethod
from asyncio import sleep
import logging

class Widget(ABC):

    sleep_seconds = 1

    @abstractmethod
    async def update(self):
        pass

    async def _update(self):
        logging.debug(f"Running update method on widget {type(self).__name__}")
        return await self.update()

    async def loop(self):
        while True:
            await self._update()
            await sleep(self.sleep_seconds)
