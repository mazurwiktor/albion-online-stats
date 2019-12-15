import os

try:
    import aostats
except:
    class aostats:
        @staticmethod
        def initialize(_):
            return InitializationResult.NetworkInterfaceListMissing
        @staticmethod
        def stats(_):
            return []
        @staticmethod
        def get_players_in_party():
            return []

from .config import config
from .number import Number

TESTING_ENABLED = bool(os.getenv('TESTING'))


class StatType:
    Unknown = 0
    LastFight = 1
    Zone = 2
    Overall = 3

class InitializationResult:
    Ok = 0
    UnknownFailure = 1
    NetworkInterfaceListMissing = 2

INITIALIZATION_RESULT = {
    0: InitializationResult.Ok,
    1: InitializationResult.UnknownFailure,
    2: InitializationResult.NetworkInterfaceListMissing
}

class DamageStat:
    def __init__(self, name, items, damage, time_in_combat, dps, percentage, best_damage):
        self.name = name
        self.items = items
        self.damage = Number(damage)
        self.time_in_combat = Number(time_in_combat)
        self.dps = Number(dps)
        self.percentage = Number(percentage)
        self.best_damage = Number(best_damage)

    def __str__(self):
        return "Name {} Damage {} DPS {} percentage {}".format(self.name, self.damage, self.dps, self.percentage)

    def __eq__(self, other):
        return self.name == other.name and self.damage == other.damage and self.time_in_combat == other.time_in_combat and self.dps == other.dps


class FameStat:
    def __init__(self, fame, fame_per_minute):
        self.fame = fame
        self.fame_per_minute = fame_per_minute


def stats(session, with_dmg=False):
    with_damage = [s for s in session if s['damage'] != 0.0] if with_dmg else session
    extended_session = with_percentage(with_damage)
    statistics = [DamageStat(
        s['player'],
        s['items'],
        s['damage'], 
        s['time_in_combat'], 
        s['dps'], 
        s['dmg_percentage'], 
        s['best_damage']) for s in extended_session]
    stats_with_fame = [p for p in session if 'fame' in p and p['fame'] != 0.0]

    if len(stats_with_fame) > 0:
        stat_with_fame = stats_with_fame[0]
        fame = FameStat(
            Number(stat_with_fame['fame']), 
            Number(stat_with_fame['fame_per_minute'])
        )
    else:
        fame = FameStat(Number(0.0), Number(0.0))

    return statistics, fame


def with_percentage(session):
    best_damage = 0.0
    damage_done = 0.0
    for s in session:
        damage = s['damage']
        if damage > best_damage:
            best_damage = damage
        damage_done += damage

    for s in session:
        s['dmg_percentage'] = s['damage'] / damage_done * 100 if s['damage']  else 0.0
        s['best_damage'] = best_damage

    return session

def zone_stats(with_damage=False):
    if TESTING_ENABLED:
        session = [
            {'player': 'Arcane', 'damage': 1000.0, 'time_in_combat': 12.0, 'dps': 12.4234, 'fame': 20.0, 'fame_per_minute': 30, 'items': {
                'weapon': 'T3_MAIN_ARCANESTAFF'
            }},
            {'player': 'Cursed', 'damage': 1000.0, 'time_in_combat': 12.0, 'dps': 12, 'items': {
                'weapon': 'T5_MAIN_CURSEDSTAFF'
            }},
            {'player': 'Fire', 'damage': 1000.0, 'time_in_combat': 12.0, 'dps': 13, 'items': {
                'weapon': 'T5_MAIN_FIRESTAFF'
            }},
            {'player': 'Frost', 'damage': 1000.0, 'time_in_combat': 12.0, 'dps': 13, 'items': {
                'weapon': 'T5_MAIN_FROSTSTAFF@1'
            }},
            {'player': 'Holy', 'damage': 500.0, 'time_in_combat': 12.0, 'dps': 13, 'items': {
                'weapon': 'T6_MAIN_HOLYSTAFF'
            }},
            {'player': 'Nature', 'damage': 500.0, 'time_in_combat': 12.0, 'dps': 13, 'items': {
                'weapon': 'T8_MAIN_NATURESTAFF@3'
            }},
            {'player': 'Axe', 'damage': 500.0, 'time_in_combat': 12.0, 'dps': 13, 'items': {
                'weapon': 'T8_MAIN_AXE'
            }},
            {'player': 'Dagger', 'damage': 500.0, 'time_in_combat': 12.0, 'dps': 13, 'items': {
                'weapon': 'T8_MAIN_DAGGER@2'
            }},
            {'player': 'Hammer', 'damage': 500.0, 'time_in_combat': 12.0, 'dps': 13, 'items': {
                'weapon': 'T7_MAIN_HAMMER@2'
            }},
            {'player': 'Mace', 'damage': 500.0, 'time_in_combat': 12.0, 'dps': 13, 'items': {
                'weapon': 'T6_MAIN_MACE@2'
            }},
            {'player': 'Quarterstaff', 'damage': 250.0, 'time_in_combat': 12.0, 'dps': 13, 'items': {
                'weapon': 'T5_2H_IRONCLADEDSTAFF'
            }},
            {'player': 'Spear', 'damage': 250.0, 'time_in_combat': 12.0, 'dps': 13, 'items': {
                'weapon': 'T8_MAIN_SPEAR@2'
            }},
            {'player': 'Sword', 'damage': 250.0, 'time_in_combat': 12.0, 'dps': 13, 'items': {
                'weapon': 'T7_2H_CLAYMORE@1'
            }},
            {'player': 'Bow', 'damage': 250.0, 'time_in_combat': 12.0, 'dps': 13, 'items': {
                'weapon': 'T8_2H_BOW'
            }},
            {'player': 'Crossbow', 'damage': 250.0, 'time_in_combat': 12.0, 'dps': 13, 'items': {
                'weapon': 'T8_2H_CROSSBOWLARGE@3'
            }},
        ]
    else:
        session = aostats.stats(StatType.Zone)

    return stats(session, with_damage)


def overall_stats(with_damage=False):
    if TESTING_ENABLED:
        session = [
            {'player': 'overall', 'damage': 1000.0,
                'time_in_combat': 12.0, 'dps': 12.4234},
        ]
    else:
        session = aostats.stats(StatType.Overall)

    return stats(session, with_damage)


def last_fight_stats(with_damage=False):
    if TESTING_ENABLED:
        session = [
            {'player': 'last fight', 'damage': 1000.0,
                'time_in_combat': 12.0, 'dps': 12.4234},
        ]
    else:
        session = aostats.stats(StatType.LastFight)

    return stats(session, with_damage)

def get_party_members():
    if TESTING_ENABLED:
        return ['a', 'b', 'c']
    else:
        return aostats.get_players_in_party()
   

def reset_zone_stats():
    aostats.reset(StatType.Zone)

def reset_last_fight_stats():
    aostats.reset(StatType.LastFight)

def reset_stats():
    aostats.reset(StatType.Overall)

def initialize():
    if TESTING_ENABLED:
        return InitializationResult.Ok
    cfg = config()
    try:
        return INITIALIZATION_RESULT[aostats.initialize(cfg['app']['skip_non_party_players'])]
    except:
        pass
