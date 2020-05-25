import os

assets_path = os.path.join(os.path.dirname(
    os.path.abspath(__file__)), '..', 'assets')

scripts_path = os.path.join(os.path.dirname(
    os.path.abspath(__file__)),  '..', 'scripts')


def path(file):
    return os.path.join(assets_path, file)


def scripts(script_name):
    return os.path.join(scripts_path, script_name)
