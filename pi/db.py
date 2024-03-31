from firebase import firebase
from dotenv import load_dotenv
from os import environ
import json
from datetime import datetime, timedelta
from functools import cache

load_dotenv()

## Verify environment
ENV_VAR_NAMES = {
    'config': 'FIREBASE_URL',
    'username': 'FIREBASE_USERNAME',
    'password': 'FIREBASE_PASSWORD'
}

@cache
def get_firebase():
    for key in ENV_VAR_NAMES:
        if ENV_VAR_NAMES[key] not in environ:
            raise Exception(f'{ENV_VAR_NAMES[key]} missing from environment variables')

    auth = firebase.FirebaseAuthentication(environ.get(ENV_VAR_NAMES['password']), email=environ.get(ENV_VAR_NAMES['username']))
    url = environ.get(ENV_VAR_NAMES['config'])

    return firebase.FirebaseApplication(url, auth)


async def firebase_get_with_retries(url, name):
    RETRIES = 5
    SLEEP_SECONDS = 5
    for _ in range(RETRIES):
        try:
            return get_firebase().get(url, name)
        except exceptions.ConnectionError:
            logging.warn("Firebase connection error, retrying...")
            await sleep(SLEEP_SECONDS)
    return None


last_widget_config_update = datetime.now()
last_widget_config = None
TTL_SECONDS = 60
async def firebase_get_widget_config():
    # Poor man's TTL cache
    global last_widget_config
    global last_widget_config_update
    if last_widget_config is None or (datetime.now() - last_widget_config_update) > timedelta(seconds=TTL_SECONDS):
        last_widget_config = await firebase_get_with_retries("/widgets", None)
        last_widget_config_update = datetime.now()
    return last_widget_config
