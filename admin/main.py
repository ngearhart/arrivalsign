import firebase_admin
from firebase_admin import auth

import os

default_app = firebase_admin.initialize_app()

def set_admin(email: str, admin: bool):
    user: auth.UserRecord = auth.get_user_by_email(email, default_app)
    auth.set_custom_user_claims(user.uid, {'admin': admin})
    print('done')

def main():
    print('Running user admin task')
    print(f'Using app {default_app.name}')
    set_admin(os.environ['EMAIL'], os.environ['ADMIN'] == 'True')

if __name__ == '__main__':
    main()
