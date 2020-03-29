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
from typing import Optional, List, Callable, Iterable

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


class Party:
    def __init__(self, get_visible_players):
        self.get_visible_players = get_visible_players

    def combat_state(self) -> int:
        return CombatState.OutOfCombat if all(
            player.combat_state == CombatState.InCombat for player in self.get_visible_players()) else CombatState.InCombat

    def time_in_combat(self) -> float:
        longest = 0.0

        for player in self.get_visible_players():
            if player.time_in_combat > longest:
                longest = player.time_in_combat

        return longest


@dataclass
class Player(Stats):
    name: str
    party: Party
    items: dict = field(default_factory=lambda: {'weapon': None})
    damage_done: float = 0.0
    healing_done: float = 0.0
    combat_time: CombatTime = field(default_factory=lambda: CombatTime())
    combat_state: int = CombatState.OutOfCombat
    active: bool  = False

    def activate(self):
        self.active = True
    
    def is_active(self) -> bool:
        return self.active

    def has_stats(self) -> bool:
        return bool(self.damage_done or self.healing_done)

    @staticmethod
    def new(self):
        raise Exception("Should never happen")

    @staticmethod
    def from_other(other):
        return Player(other.name, other.party, items=other.items)

    def update(self, other):
        self.name = other.name
        self.damage_done += other.damage_done
        self.healing_done += other.healing_done
        self.combat_state = other.combat_state
        self.combat_time.entered_combat = other.combat_time.entered_combat
        self.combat_time.time_in_combat += other.combat_time.time_in_combat
        self.items = other.items
        self.active = other.active or self.active

    def register_items(self, value):
        self.items = value

    def register_damage_done(self, value):
        if self.combat_state == CombatState.OutOfCombat:
            return

        self.damage_done += value
        self.activate()

    def register_healing_done(self, value):
        if self.party.combat_state() == CombatState.OutOfCombat:
            return

        self.healing_done += value
        self.activate()

    def enter_combat(self):
        self.combat_time.entered_combat = time_utils.now()
        self.combat_state = CombatState.InCombat

    def leave_combat(self):
        if self.combat_time.entered_combat:
            self.combat_time.time_in_combat += time_utils.delta(
                self.combat_time.entered_combat)
        self.combat_state = CombatState.OutOfCombat

    def into_damage_list_item(self) -> StandalonePlayerListItem:
        return StandalonePlayerListItem(
            self.name, self.items, self.damage_done, self.dps, self.combat_state)

    def into_healing_list_item(self) -> StandalonePlayerListItem:
        return StandalonePlayerListItem(
            self.name, self.items, self.healing_done, self.hps, self.combat_state)

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

    @property
    def hps(self):
        time_in_combat = self.party.time_in_combat()

        if time_in_combat == 0.0:
            return 0.0

        return time_utils.as_milliseconds(self.healing_done / time_in_combat)


class CombatStats(CombatEventReceiver, Stats):
    def __init__(self, visibility, players=None):
        self.visibility = visibility
        self.party = Party(self.visible_players)
        if not players:
            players = {}
        self.players = players

    @staticmethod
    def new(self):
        raise Exception("Should never happen")

    @staticmethod
    def from_other(other):
        return CombatStats(other.visibility, {k: Player.from_other(v) for (k, v) in other.players.items()})

    def update(self, other):
        for (id, player) in other.players.items():
            if id in self.players:
                self.players[id].update(player)
            else:
                self.players[id] = Player.from_other(player)
                self.players[id].update(player)

    def update_non_idle(self, other):
        self.update(other)
        self.players = dict(filter(lambda elem: elem[1].has_stats(), self.players.items()))

        for player in self.players.values():
            player.activate()

    def combined(self, other):
        stats = CombatStats(self.visibility)
        stats.update(self)
        stats.update(other)

        return stats

    def players_damage(self) -> List[PlayerListItem]:
        return to_player_list_items([player.into_damage_list_item()
                                     for player in self.visible_players()
                                     ])

    def players_healing(self) -> List[PlayerListItem]:
        return to_player_list_items([player.into_healing_list_item()
                                     for player in self.visible_players()
                                     ])

    def party_combat_state(self):
        return self.party.combat_state()

    def visible_players(self) -> Iterable[Player]:
        return (player for player in self.players.values() if player.is_active() and self.visibility.test(player.name))

    def on_player_appeared(self, id: int, name: str):
        if id not in self.players:
            self.players[id] = Player(name, self.party)

        self.players[id].activate()

    def on_damage_done(self, id: int, damage: float):
        self.players[id].register_damage_done(damage)

    def on_health_received(self, id: int, target_id: int, health: float):
        self.players[id].register_healing_done(health)

    def on_enter_combat(self, id: int):
        self.players[id].enter_combat()

    def on_leave_combat(self, id: int):
        self.players[id].leave_combat()

    def on_items_update(self, id: int, items: dict):
        self.players[id].register_items(items)
