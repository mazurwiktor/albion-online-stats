from .utils.config import config
from .utils.number import Number
from .environment import TEST_ENV_ENABLED

from . import backend_proxy
from .backend_proxy import InitializationResult, INITIALIZATION_RESULT
from .stats import damage_stats, fame_stats, time_stats
from .consts import events as ev_consts


class StatType:
    Unknown = 0
    LastFight = 1
    Zone = 2
    Overall = 3


class GameStats():
    def __init__(self):
        self.zone = {
            'damage': damage_stats.DamageStats(),
            'fame': fame_stats.FameStats(),
            'time': time_stats.TimeStats(),
        }
        self.last_fight = {
            'damage': damage_stats.DamageStats(),
            'fame': fame_stats.FameStats(),
            'time': time_stats.TimeStats(),
        }
        self.history = {
            'damage': damage_stats.DamageStats(),
            'fame': fame_stats.FameStats(),
            'time': time_stats.TimeStats(),
        }

    def register_event(self, event):
        if event[ev_consts.EvKeyName] == ev_consts.EvNameEnterCombat:
            if self._are_everyone_in_session_out_of_combat():
                self.last_fight['damage'] = damage_stats.DamageStats.from_other(
                    self.zone['damage'])
        elif event[ev_consts.EvKeyName] == ev_consts.EvNameZoneChange:
            self.history['damage'].update(self.zone['damage'])
            self.history['fame'].update(self.zone['fame'])
            self.history['time'].update(self.zone['time'])

            self.zone['damage'] = damage_stats.DamageStats()
            self.zone['fame'] = fame_stats.FameStats()
            self.zone['time'] = time_stats.TimeStats()

        self.zone['damage'].receive(event)
        self.last_fight['damage'].receive(event)
        self.zone['fame'].receive(event)
        self.last_fight['fame'].receive(event)

    def reset(self, stat_type):
        if stat_type == StatType.Zone:
            self.zone['damage'] = damage_stats.DamageStats()
            self.last_fight['damage'] = damage_stats.DamageStats()
        elif stat_type == StatType.LastFight:
            self.last_fight['damage'] = damage_stats.DamageStats()
        elif stat_type == StatType.Overall:
            self.zone['damage'] = damage_stats.DamageStats()
            self.last_fight['damage'] = damage_stats.DamageStats()

    def damage_stats(self, stat_type):
        if stat_type == StatType.Zone:
            return self.zone['damage'].player_list()
        elif stat_type == StatType.LastFight:
            return self.last_fight['damage'].player_list()
        elif stat_type == StatType.Overall:
            return self.history['damage'].combined(self.zone['damage']).player_list()

    def fame_stats(self, stat_type):
        if stat_type == StatType.Zone:
            return self.zone['fame'].stats()
        elif stat_type == StatType.LastFight:
            return self.last_fight['fame'].stats()
        elif stat_type == StatType.Overall:
            return self.history['fame'].combined(self.zone['fame']).stats()

    def time_stats(self, stat_type):
        if stat_type == StatType.Zone:
            return self.zone['time'].stats()
        elif stat_type == StatType.LastFight:
            return self.last_fight['time'].stats()
        elif stat_type == StatType.Overall:
            return self.history['time'].combined(self.zone['time']).stats()

    def _are_everyone_in_session_out_of_combat(self):
        return all(player.combat_state == damage_stats.CombatState.OutOfCombat for player in self.zone['damage'].players.values())

    def _merged_stats(self, stats: dict):
        result = {}

        for stat in stats:
            result.update(stat.stats())

        return result


game_stats = GameStats()


class FameStat:
    def __init__(self, fame, fame_per_hour):
        self.fame = fame
        self.fame_per_hour = fame_per_hour


def zone_stats(with_damage=False):
    return get_stats(StatType.Zone, with_damage)


def overall_stats(with_damage=False):
    return get_stats(StatType.Overall, with_damage)


def last_fight_stats(with_damage=False):
    return get_stats(StatType.LastFight, with_damage)


def get_stats(stat_type: StatType, with_damage: bool):
    fame = game_stats.fame_stats(stat_type)
    time = game_stats.time_stats(stat_type)['seconds_in_game']
    fame = FameStat(Number(fame['fame']), Number(
        fame['fame'] / time if time > 0.0 else 0.0))
    players = list(filter(lambda p: p.value !=
                          0.0 if with_damage else True, game_stats.damage_stats(stat_type)))

    return players, fame, time


def reset_zone_stats():
    game_stats.reset(StatType.Zone)


def reset_last_fight_stats():
    game_stats.reset(StatType.LastFight)


def reset_stats():
    game_stats.reset(StatType.Overall)


def initialize():
    initialization_result = backend_proxy.initialize()
    backend_proxy.subscribe(game_stats.register_event)
    return initialization_result
