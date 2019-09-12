import os

import libmeter

TESTING_ENABLED = bool(os.getenv('TESTING'))

class Stat:
    def __init__(self, name, damage, time_in_combat, dps):
        self.name = name
        self.damage = '{0:.2f}'.format(damage)
        self.time_in_combat = '{0:.2f}'.format(time_in_combat)
        self.dps = '{0:.2f}'.format(dps)

    def __eq__(self, other):
        return self.name == other.name and self.damage == other.damage and self.time_in_combat == other.time_in_combat and self.dps == other.dps


def get_instance_session():
    if TESTING_ENABLED:
        session = [
            {'player': 'A', 'damage': 1234.02, 'time_in_combat': 12.0, 'dps': 12.4234},
            {'player': 'B', 'damage': 5435.02, 'time_in_combat': 12.0, 'dps': 12},
            {'player': 'C', 'damage': 23.02, 'time_in_combat': 12.0, 'dps': 13},
            {'player': 'D', 'damage': 0, 'time_in_combat': 12.0, 'dps': 0}
        ]
    else:
        session = libmeter.get_instance_session()

    return [Stat(s['player'], s['damage'], s['time_in_combat'], s['dps']) for s in session]

def reset_instance_session():
    libmeter.reset_instance_session()

def initialize():
    if TESTING_ENABLED:
        return

    try:
        libmeter.initialize()
    except:
        pass