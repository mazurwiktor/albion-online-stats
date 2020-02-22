from .utils.config import config
from .utils.number import Number
from .environment import TEST_ENV_ENABLED

from . import backend_proxy
from .backend_proxy import InitializationResult, INITIALIZATION_RESULT
from .stats import damage_stats

class StatType:
    Unknown = 0
    LastFight = 1
    Zone = 2
    Overall = 3

class GameStats:
    def __init__(self):
        self.zone = damage_stats.DamageStats()
        self.last_fight = damage_stats.DamageStats()
        self.overall = []

    def register_event(self, event):
        if event['name'] == 'MainPlayerAppeared':
            self.zone.add_player(
                event['value']['id'], event['value']['name'])
        elif event['name'] == 'PlayerAppeared':
            self.zone.add_player(
                event['value']['id'], event['value']['name'])
        elif event['name'] == 'DamageDone':
            print("Damage done! {}".format(event))
            self.zone.register_damage_done(
                event['value']['id'], event['value']['value'])
        elif event['name'] == 'HealthReceived':
            pass
        elif event['name'] == 'ZoneChange':
            self.zone = damage_stats.DamageStats()
        elif event['name'] == 'EnterCombat':
            self.zone.enter_combat(
                event['value']['id'])
        elif event['name'] == 'LeaveCombat':
            self.zone.leave_conbat(
                event['value']['id'])
        elif event['name'] == 'UpdateFame':
            pass
        elif event['name'] == 'UpdateItems':
            self.zone.add_items(
                event['value']['source'], event['value']['value'])

    def reset(self, stat_type):
        if stat_type == StatType.Zone:
            self.zone = damage_stats.DamageStats()
        elif stat_type == StatType.LastFight:
            self.last_fight = damage_stats.DamageStats()
        elif stat_type == StatType.Overall:
            self.zone = damage_stats.DamageStats()
            self.last_fight = []

    def damage_stats(self, stat_type):
        if stat_type == StatType.Zone:
            return self.zone.stats()
        elif stat_type == StatType.LastFight:
            return self.last_fight.stats()
        elif stat_type == StatType.Overall:
            return damage_stats.combined_stats(self.overall + self.zone.stats())

game_stats = GameStats()

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
        return "Name {} Damage {} DPS {} percentage {} items {}".format(self.name, self.damage, self.dps, self.percentage, self.items)

    def __eq__(self, other):
        return self.name == other.name and self.damage == other.damage and self.time_in_combat == other.time_in_combat and self.dps == other.dps


class FameStat:
    def __init__(self, fame, fame_per_hour):
        self.fame = fame
        self.fame_per_hour = fame_per_hour


def stats(session, with_dmg=False):
    players = session['players']
    main_player = session['main']

    with_damage = [s for s in players if s['damage']
                   != 0.0] if with_dmg else players
    extended_session = with_percentage(with_damage)
    statistics = [DamageStat(
        s['player'],
        s['items'],
        s['damage'],
        s['time_in_combat'],
        s['dps'],
        s['dmg_percentage'],
        s['best_damage']) for s in extended_session]

    elapsed = 0
    fame = FameStat(Number(0.0), Number(0.0))
    if main_player:
        if 'fame' in main_player:
            fame = FameStat(Number(main_player['fame']), Number(
                main_player['fame_per_hour']))
        if 'seconds_in_game' in main_player:
            elapsed = main_player['seconds_in_game']

    return statistics, fame, elapsed


def with_percentage(session):
    best_damage = 0.0
    damage_done = 0.0
    for s in session:
        damage = s['damage']
        if damage > best_damage:
            best_damage = damage
        damage_done += damage

    for s in session:
        s['dmg_percentage'] = s['damage'] / \
            damage_done * 100 if s['damage'] else 0.0
        s['best_damage'] = best_damage

    return session


def zone_stats(with_damage=False):
    return stats({
        'players': game_stats.damage_stats(StatType.Zone), 
        'main': {}}, 
        with_damage)


def overall_stats(with_damage=False):
    return stats({
        'players': game_stats.damage_stats(StatType.Overall), 
        'main': {}}, 
        with_damage)


def last_fight_stats(with_damage=False):
    return stats({
        'players': game_stats.damage_stats(StatType.LastFight), 
        'main': {}}, 
        with_damage)


def reset_zone_stats():
    game_stats.reset(StatType.Zone)


def reset_last_fight_stats():
    game_stats.reset(StatType.LastFight)


def reset_stats():
    game_stats.reset(StatType.Overall)

def initialize():
    backend_proxy.initialize()
    backend_proxy.subscribe(game_stats.register_event)
