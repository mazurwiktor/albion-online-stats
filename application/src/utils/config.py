import os
import sys
import toml

width = 800
height = 220
font = 15
frame = False
frame_str = str(frame).lower()
top = False
top_str = str(top).lower()
opaci_percent = 100
opaci = opaci_percent/100
opaci_formatted = "{:.1f}".format(opaci)

CFG_VERSION = '0.7'

default = """
[window]
width = %d
height = %d
font-size = '%dpx'

# Note: do not change!
# options bellow are here only for testing purposes
# SBI politics doesn't allow opacity frameless and always on top applications!
opacity = %s
frameless = %s
always_on_top = %s

[app]

[config]
# do not change
version = '%s'
""" % (width, height, font, opaci_formatted, frame_str, top_str, CFG_VERSION)


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
            _config = cfg
    except(Exception) as _:
        with open(conf_file, "w") as cfg_file:
            cfg_file.write(default)
            _config = toml.loads(default)

    return _config