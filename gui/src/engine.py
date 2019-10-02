import os

import libmeter
from config import config

TESTING_ENABLED = bool(os.getenv('TESTING'))


class DamageStat:
    def __init__(self, name, damage, time_in_combat, dps, precentage):
        self.name = name
        self.damage = '{0:.2f}'.format(damage)
        self.time_in_combat = '{0:.2f}'.format(time_in_combat)
        self.dps = '{0:.2f}'.format(dps)
        self.precentage = '{0:.2f}'.format(precentage)

    def __eq__(self, other):
        return self.name == other.name and self.damage == other.damage and self.time_in_combat == other.time_in_combat and self.dps == other.dps


class FameStat:
    def __init__(self, fame, fame_per_minute):
        self.fame = '{0:.2f}'.format(fame)
        self.fame_per_minute = fame_per_minute


def stats(session):
    with_damage = [s for s in session if s['damage'] != 0.0]
    extended_session = with_precentage(with_damage)
    statistics = [DamageStat(s['player'], s['damage'], s['time_in_combat'], s['dps'], s['dmg_precentage']) for s in extended_session]
    stats_with_fame = [p for p in session if 'fame' in p and p['fame'] != 0.0]

    if len(stats_with_fame) > 0:
        stat_with_fame = stats_with_fame[0]
        fame = FameStat(stat_with_fame['fame'], stat_with_fame['fame_per_minute'])
    else:
        fame = FameStat(0.0, 0.0)

    return statistics, fame


def with_precentage(session):

    damage_done = 0.0
    for s in session:
        damage_done += s['damage']

    for s in session:
        s['dmg_precentage'] = s['damage'] / damage_done * 100

    return session

def get_zone_session():
    if TESTING_ENABLED:
        session = [
            {'player': 'A', 'damage': 1234.02,
                'time_in_combat': 12.0, 'dps': 12.4234, 'fame': 20.0, 'fame_per_minute': 30},
            {'player': 'B', 'damage': 5435.02, 'time_in_combat': 12.0, 'dps': 12},
            {'player': 'C', 'damage': 23.02, 'time_in_combat': 12.0, 'dps': 13},
            {'player': 'D', 'damage': 0, 'time_in_combat': 12.0, 'dps': 0}
        ]
    else:
        session = libmeter.get_zone_session()

    return stats(session)


def get_overall_session():
    if TESTING_ENABLED:
        session = [
            {'player': 'overall', 'damage': 1234.02,
                'time_in_combat': 12.0, 'dps': 12.4234},
        ]
    else:
        session = libmeter.get_overall_session()

    return stats(session)


def get_last_fight_session():
    if TESTING_ENABLED:
        session = [
            {'player': 'last fight', 'damage': 1234.02,
                'time_in_combat': 12.0, 'dps': 12.4234},
        ]
    else:
        session = libmeter.get_last_fight_session()

    return stats(session)

def get_party_members():
    if TESTING_ENABLED:
        return ['a', 'b', 'c']
    else:
        return libmeter.get_players_in_party()
   

def new_zone_session():
    libmeter.new_zone_session()

def new_last_fight_session():
    libmeter.new_last_fight_session()

def reset_sessions():
    libmeter.reset_sessions()

def initialize():
    if TESTING_ENABLED:
        return
    cfg = config()
    try:
        libmeter.initialize(cfg['app']['skip_non_party_players'])
    except:
        pass
