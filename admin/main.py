import firebase_admin
from firebase_admin import auth, _auth_utils

import os

default_app = firebase_admin.initialize_app()

def set_admin_by_email_or_uid(admin: bool, **kwargs):
    if 'email' in kwargs:
        try: 
            user: auth.UserRecord = auth.get_user_by_email(kwargs['email'], default_app)
        except _auth_utils.UserNotFoundError:
            print('User not found.')
            for user in auth.get_users(default_app):
                print(f' - {user.email}: {user.uid}')
            raise
        set_admin(user.uid, admin)
    elif 'uid' in kwargs:
        set_admin(kwargs['uid'], admin)
    else:
        raise Exception('Invalid arguments: need email or uid')

def set_admin(uid: str, admin: bool):
    auth.set_custom_user_claims(uid, {'admin': admin})
    print('done')

def main():
    print('Running user admin task')
    print(f'Using app {default_app.name} ({default_app.project_id})')
    set_admin_by_email_or_uid(os.environ.get('ADMIN') == 'True', email=os.environ.get('EMAIL'), uid=os.environ.get('UID'))

if __name__ == '__main__':
    main()
