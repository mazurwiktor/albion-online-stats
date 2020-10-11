import os
import platform
import sys
import toml

CFG_VERSION = '0.7'

default = """
[window]
width = 300
height = 220
font-size = '10px'

# Note: do not change!
# options bellow are here only for testing purposes
# SBI politics doesn't allow opacity frameless and always on top applications!
opacity = 1.0
frameless = false
always_on_top = false

[app]

[config]
# do not change
version = '%s'
""" % (CFG_VERSION)


# pyinstaller creates tmpdir for python files, thus this is the way to get executable path
_script_path = sys.argv[0]
_config = None


def get_config_path():
    cfg_file_name = 'albion-online-stats.cfg'
    script_dir = os.path.dirname(_script_path)
    if platform.system() == 'Windows':
        return os.path.join(os.getenv('APPDATA', script_dir), cfg_file_name)
    else:
        return os.path.join(script_dir, cfg_file_name)


def config():
    global _config

    if _config:
        return _config

    conf_file = get_config_path()

    try:
        with open(conf_file, "r") as cfg_file:
            cfg = toml.load(cfg_file)
            if cfg['config']['version'] != CFG_VERSION:
                raise Exception('Version changed')
            _config = cfg
    except(Exception) as _:
        with open(conf_file, "w") as cfg_file:
            cfg_file.write(default)
            _config = toml.loads(default)

    return _config