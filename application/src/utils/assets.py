import os

assets_path = os.path.join(os.path.dirname(
    os.path.abspath(__file__)), '..', 'assets')


def path(file):
    return os.path.join(assets_path, file)
