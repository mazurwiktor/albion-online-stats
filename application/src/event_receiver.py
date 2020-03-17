import abc

from typing import Dict, List

from .consts import events as consts


class EventReceiver(abc.ABC):
    def receive(self, event):
        event_name: str = event[consts.EvKeyName]
        value: dict = event[consts.EvKeyValue] if consts.EvKeyValue in event else None

        self.on_event(event_name, value)

    @abc.abstractmethod
    def on_event(self, name: str, value: dict):
        pass


class VisibilityEventReceiver(EventReceiver):
    def on_event(self, event_name: str, value: dict):
        if event_name == consts.EvNameMainPlayerAppeared:
            self.on_player_appeared(value[consts.EvKeyName])
        elif event_name == consts.EvNameUpdateParty:
            self.on_visible_players_changed(value[consts.EvKeyPlayerNames])
    
    @abc.abstractmethod
    def on_player_appeared(self, main_player_name: str):
        pass

    @abc.abstractmethod
    def on_visible_players_changed(self, visible_players : List[str]):
        pass

class FameEventReceiver(EventReceiver):

    def on_event(self, event_name: str, value: dict):
        if event_name == consts.EvNameUpdateFame:
            self.on_fame_update(value[consts.EvKeyValue])

    @abc.abstractmethod
    def on_fame_update(self, value: float):
        pass


class CombatEventReceiver(EventReceiver):

    def on_event(self, event_name: str, value: dict):
        if event_name == consts.EvNameMainPlayerAppeared:
            self.on_player_appeared(
                value[consts.EvKeyId], value[consts.EvKeyName])
        elif event_name == consts.EvNamePlayerAppeared:
            self.on_player_appeared(
                value[consts.EvKeyId], value[consts.EvKeyName])
        elif event_name == consts.EvNameDamageDone:
            self.on_damage_done(
                value[consts.EvKeySource], value[consts.EvKeyValue])
        elif event_name == consts.EvNameHealthReceived:
            self.on_health_received(
                value[consts.EvKeySource], value[consts.EvKeyTarget], value[consts.EvKeyValue])
        elif event_name == consts.EvNameEnterCombat:
            self.on_enter_combat(value[consts.EvKeyId])
        elif event_name == consts.EvNameLeaveCombat:
            self.on_leave_combat(value[consts.EvKeyId])
        elif event_name == consts.EvNameUpdateItems:
            self.on_items_update(
                value[consts.EvKeySource], value[consts.EvKeyValue])

    @abc.abstractmethod
    def on_player_appeared(self, id: int, name: str):
        pass

    @abc.abstractmethod
    def on_damage_done(self, id: int, damage: float):
        pass

    @abc.abstractmethod
    def on_health_received(self, id: int, target_id: int, health: float):
        pass

    @abc.abstractmethod
    def on_enter_combat(self, id: int):
        pass

    @abc.abstractmethod
    def on_leave_combat(self, id: int):
        pass

    @abc.abstractmethod
    def on_items_update(self, id: int, items: Dict[str, str]):
        pass
