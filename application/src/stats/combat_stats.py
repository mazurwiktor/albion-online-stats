# Example output
# 'player': 'Cursed',
# 'damage': 1100.0,
# 'time_in_combat': 12.0,
# 'dps': 222,
# 'items': {
#   'weapon': 'T5_MAIN_CURSEDSTAFF@2'
#  }

from dataclasses import field
from dataclasses import dataclass
from typing import Optional, List

from . import time_utils
from ..event_receiver import CombatEventReceiver, VisibilityEventReceiver
from .statistics import Stats
from .list_item import PlayerListItem, StandalonePlayerListItem, to_player_list_items
from .visibility import Visibility
from .combat_state import CombatState


@dataclass
class CombatTime:
    entered_combat: Optional[float] = None
    time_in_combat: float = 0.0


@dataclass
class Player(Stats):
    name: str
    items: dict = field(default_factory=lambda: {'weapon': None})
    damage_done: float = 0.0
    combat_time: CombatTime = field(default_factory=lambda: CombatTime())
    combat_state: int = CombatState.OutOfCombat

    @staticmethod
    def new(self):
        return Player()

    @staticmethod
    def from_other(other):
        return Player(other.name, other.items)

    def update(self, other):
        self.name = other.name
        self.damage_done += other.damage_done
        self.combat_state = other.combat_state
        self.combat_time.entered_combat = other.combat_time.entered_combat
        self.combat_time.time_in_combat += other.combat_time.time_in_combat
        self.items = other.items

    def register_items(self, value):
        self.items = value

    def register_damage_done(self, value):
        if self.combat_state == CombatState.OutOfCombat:
            return

        self.damage_done += value

    def enter_combat(self):
        self.combat_time.entered_combat = time_utils.now()
        self.combat_state = CombatState.InCombat

    def leave_combat(self):
        if self.combat_time.entered_combat:
            self.combat_time.time_in_combat += time_utils.delta(
                self.combat_time.entered_combat)
        self.combat_state = CombatState.OutOfCombat

    @property
    def time_in_combat(self):
        if self.combat_state == CombatState.InCombat:
            if self.combat_time.entered_combat:
                return self.combat_time.time_in_combat + time_utils.delta(self.combat_time.entered_combat)

        return self.combat_time.time_in_combat

    @property
    def dps(self):
        if self.time_in_combat == 0.0:
            return 0.0

        return time_utils.as_milliseconds(self.damage_done / self.time_in_combat)


class CombatStats(CombatEventReceiver, Stats):
    def __init__(self, players=None):
        if not players:
            players = {}
        self.players = players

    @staticmethod
    def new(self):
        return CombatStats()

    @staticmethod
    def from_other(other):
        return CombatStats({k: Player.from_other(v) for (k, v) in other.players.items()})

    def update(self, other):
        for (id, player) in other.players.items():
            if id in self.players:
                self.players[id].update(player)
            else:
                self.players[id] = Player.from_other(player)
                self.players[id].update(player)

    def combined(self, other):
        stats = CombatStats()
        stats.update(self)
        stats.update(other)

        return stats

    def player_list(self, visibility: Visibility) -> List[PlayerListItem]:
        return to_player_list_items([
            StandalonePlayerListItem(
                player.name, player.items, player.damage_done, player.dps, player.combat_state)
            for player in self.players.values() if visibility.test(player.name)
        ])

    def on_player_appeared(self, id: int, name: str):
        if id not in self.players:
            self.players[id] = Player(name)

    def on_damage_done(self, id: int, damage: float):
        self.players[id].register_damage_done(damage)

    def on_health_received(self, id: int, damage: float):
        pass

    def on_enter_combat(self, id: int):
        self.players[id].enter_combat()

    def on_leave_combat(self, id: int):
        self.players[id].leave_combat()

    def on_items_update(self, id: int, items: dict):
        self.players[id].register_items(items)
