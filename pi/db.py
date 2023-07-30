from firebase import firebase
from os import environ
import json
from functools import cache

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
