from typing import Iterable
from dataclasses import dataclass
from queue import Queue

from .utils.config import config
from .utils.number import Number
from .environment import TEST_ENV_ENABLED

from . import backend_proxy
from .backend_proxy import InitializationResult, INITIALIZATION_RESULT
from .stats import combat_stats, fame_stats, time_stats, combat_state
from .consts import events as ev_consts
from .event_receiver import VisibilityEventReceiver
from .stats.visibility import Visibility


@dataclass(frozen=True)
class VisibilityType:
    LastFight: str = 'last_fight'
    Zone: str = 'zone'
    Overall: str = 'history'


@dataclass(frozen=True)
class StatType:
    Combat: str = 'Combat'
    Fame: str = 'Fame'
    Time: str = 'Time'


@dataclass(frozen=True)
class CombatStatType:
    Damage: str = 'Damage'
    Healing: str = 'Healing'


class GameStats():
    def __init__(self):
        self.visibility = Visibility()

        self.zone = {
            StatType.Combat: combat_stats.CombatStats(self.visibility),
            StatType.Fame: fame_stats.FameStats(),
            StatType.Time: time_stats.TimeStats(),
        }
        self.last_fight = {
            StatType.Combat: combat_stats.CombatStats(self.visibility),
            StatType.Fame: fame_stats.FameStats(),
            StatType.Time: time_stats.TimeStats(),
        }
        self.history = {
            StatType.Combat: combat_stats.CombatStats(self.visibility),
            StatType.Fame: fame_stats.FameStats(),
            StatType.Time: time_stats.TimeStats(),
        }

    def register_event(self, event):
        if event[ev_consts.EvKeyName] == ev_consts.EvNameEnterCombat:
            if self.zone[StatType.Combat].party_combat_state() == combat_state.CombatState.OutOfCombat:
                self._construct_new_stats([VisibilityType.LastFight])

        elif event[ev_consts.EvKeyName] == ev_consts.EvNameZoneChange:
            self.history[StatType.Combat].update(self.zone[StatType.Combat])
            self.history[StatType.Fame].update(self.zone[StatType.Fame])
            self.history[StatType.Time].update(self.zone[StatType.Time])

            self._construct_new_stats([VisibilityType.Zone])

        self.zone[StatType.Combat].receive(event)
        self.last_fight[StatType.Combat].receive(event)
        self.zone[StatType.Fame].receive(event)
        self.last_fight[StatType.Fame].receive(event)
        self.visibility.receive(event)

    def reset(self, stat_type):
        if stat_type == VisibilityType.Zone:
            for t in (StatType.Combat, StatType.Time, StatType.Fame):
                self.history[t].update(self.zone[t])
            self._construct_new_stats(
                (VisibilityType.Zone, VisibilityType.LastFight))
        elif stat_type == VisibilityType.LastFight:
            self._construct_new_stats([VisibilityType.LastFight])
        elif stat_type == VisibilityType.Overall:
            self._construct_new_stats(
                (VisibilityType.Zone, VisibilityType.Overall, VisibilityType.LastFight))

    def get_damage_stats(self, stat_type):
        if stat_type == VisibilityType.Zone:
            return self.zone[StatType.Combat].players_damage()
        elif stat_type == VisibilityType.LastFight:
            return self.last_fight[StatType.Combat].players_damage()
        elif stat_type == VisibilityType.Overall:
            return self.history[StatType.Combat].combined(self.zone[StatType.Combat]).players_damage()

    def get_healing_stats(self, stat_type):
        if stat_type == VisibilityType.Zone:
            return self.zone[StatType.Combat].players_healing()
        elif stat_type == VisibilityType.LastFight:
            return self.last_fight[StatType.Combat].players_healing()
        elif stat_type == VisibilityType.Overall:
            return self.history[StatType.Combat].combined(self.zone[StatType.Combat]).players_healing()

    def fame_stats(self, stat_type):
        if stat_type == VisibilityType.Zone:
            return self.zone[StatType.Fame].stats()
        elif stat_type == VisibilityType.LastFight:
            return self.last_fight[StatType.Fame].stats()
        elif stat_type == VisibilityType.Overall:
            return self.history[StatType.Fame].combined(self.zone[StatType.Fame]).stats()

    def time_stats(self, stat_type):
        if stat_type == VisibilityType.Zone:
            return self.zone[StatType.Time].stats()
        elif stat_type == VisibilityType.LastFight:
            return self.last_fight[StatType.Time].stats()
        elif stat_type == VisibilityType.Overall:
            return self.history[StatType.Time].combined(self.zone[StatType.Time]).stats()

    def _construct_new_stats(self, scope: Iterable[str]):
        types = (StatType.Combat, StatType.Time, StatType.Fame)
        for s in scope:
            for t in types:
                if t == StatType.Time:
                    getattr(self, s)[t] = time_stats.TimeStats()
                elif t == StatType.Combat:
                    getattr(self, s)[t] = combat_stats.CombatStats.from_other(
                        getattr(self, s)[t])
                elif t == StatType.Fame:
                    getattr(self, s)[t] = fame_stats.FameStats()

    def _merged_stats(self, stats: dict):
        result = {}

        for stat in stats:
            result.update(stat.stats())

        return result


game_stats = GameStats()
event_queue: Queue = Queue()


class FameStat:
    def __init__(self, fame, fame_per_hour):
        self.fame = fame
        self.fame_per_hour = fame_per_hour


def zone_stats(combat_stat_type: str):
    return get_stats(VisibilityType.Zone, combat_stat_type)


def overall_stats(combat_stat_type: str):
    return get_stats(VisibilityType.Overall, combat_stat_type)


def last_fight_stats(combat_stat_type: str):
    return get_stats(VisibilityType.LastFight, combat_stat_type)


cached_players = ()  # do not compute values if there is nothing in queue
cached_players_type = None

def get_stats(stat_type: str, combat_stat_type: str):
    global cached_players, cached_players_type
    new_events = False

    while not event_queue.empty():
        new_events = True
        game_stats.register_event(event_queue.get_nowait())

    fame = game_stats.fame_stats(stat_type)
    time = game_stats.time_stats(stat_type)['seconds_in_game']
    fame = FameStat(Number(fame['fame']), Number(
        (fame['fame'] / time) * 60 * 60 if time > 0.0 else 0.0))

    if not new_events and cached_players and cached_players_type != stat_type:
        return (cached_players, fame, time)

    cached_players = {
        CombatStatType.Damage: lambda stat_type : game_stats.get_damage_stats(stat_type),
        CombatStatType.Healing: lambda stat_type : game_stats.get_healing_stats(stat_type)
    }[combat_stat_type](stat_type)

    cached_players_type = stat_type

    return (cached_players, fame, time)


def queue_an_event(event):
    event_queue.put(event)


def reset_zone_stats():
    game_stats.reset(VisibilityType.Zone)


def reset_last_fight_stats():
    game_stats.reset(VisibilityType.LastFight)


def reset_stats():
    game_stats.reset(VisibilityType.Overall)


def is_ready():
    return game_stats.visibility.is_main_player_visible


def initialize():
    initialization_result = backend_proxy.initialize()
    backend_proxy.subscribe(queue_an_event)
    return initialization_result
