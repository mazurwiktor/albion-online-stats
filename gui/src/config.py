import os
import toml

CFG_VERSION = '0.1'

default = """
[window]
width = 300
height = 200
font-size = '10px'
opacity = 0.5

[config]
# do not change
version = '%s'
""" % (CFG_VERSION)


def config():
    conf_file = os.path.join(os.path.dirname(
        os.path.realpath(__file__)), 'albion-online-stats.cfg')

    try:
        with open(conf_file, "r") as cfg_file:
            cfg = toml.load(cfg_file)
            if cfg['config']['version'] != CFG_VERSION:
                print('version changed')
                raise Exception('Version changed')
            return cfg
    except(Exception) as e:
        with open(conf_file, "w") as cfg_file:
            cfg_file.write(default)
            return toml.loads(default)

    return {}