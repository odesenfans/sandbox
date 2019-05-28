# To generate a new secret key:
# >>> import random, string
# >>> "".join([random.choice(string.printable) for _ in range(24)])

import os

SECRET_KEY = '}U5\rJlL9S{%D.u\\"6bPwaq=!'
FB_APP_ID = 553484131847572

basedir = os.path.abspath(os.path.dirname(__file__))
SQLALCHEMY_DATABASE_URI = 'sqlite:///' + os.path.join(basedir, 'app.db')
