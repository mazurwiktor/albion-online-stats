import os
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


def config():
    global _script_path
    global _config

    if _config:
        return _config

    conf_file = os.path.join(os.path.dirname(
        _script_path), 'albion-online-stats.cfg')

    try:
        with open(conf_file, "r") as cfg_file:
            cfg = toml.load(cfg_file)
            if cfg['config']['version'] != CFG_VERSION:
                raise Exception('Version changed')
            return cfg
    except(Exception) as e:
        with open(conf_file, "w") as cfg_file:
            cfg_file.write(default)
            return toml.loads(default)

    return {}
