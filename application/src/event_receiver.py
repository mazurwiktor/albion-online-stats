import abc

from typing import Dict

from .consts import events as consts


class EventReceiver(abc.ABC):
    def receive(self, event):
        event_name: Dict[str, str] = event[consts.EvKeyName]
        value: dict = event[consts.EvKeyValue] if consts.EvKeyValue in event else None

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
                value[consts.EvKeySource], value[consts.EvKeyValue])
        elif event_name == consts.EvNameZoneChange:
            self.on_zone_change()
        elif event_name == consts.EvNameEnterCombat:
            self.on_enter_combat(value[consts.EvKeyId])
        elif event_name == consts.EvNameLeaveCombat:
            self.on_leave_combat(value[consts.EvKeyId])
        elif event_name == consts.EvNameUpdateFame:
            self.on_fame_update(value[consts.EvKeyValue])
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
    def on_health_received(self, id: int, health: float):
        pass

    @abc.abstractmethod
    def on_zone_change(self):
        pass

    @abc.abstractmethod
    def on_enter_combat(self, id: int):
        pass

    @abc.abstractmethod
    def on_leave_combat(self, id: int):
        pass

    @abc.abstractmethod
    def on_fame_update(self, value: float):
        pass

    @abc.abstractmethod
    def on_items_update(self, id: int, items: Dict[str, str]):
        pass


class PassiveEventReceiver(EventReceiver):
    def on_player_appeared(self, id: int, name: str):
        pass

    def on_damage_done(self, id: int, damage: float):
        pass

    def on_health_received(self, id: int, health: float):
        pass

    def on_zone_change(self):
        pass

    def on_enter_combat(self, id: int):
        pass

    def on_leave_combat(self, id: int):
        pass

    def on_fame_update(self, value: float):
        pass

    def on_items_update(self, id: int, items: Dict[str, str]):
        pass


class FameEventReceiver(PassiveEventReceiver):
    @abc.abstractmethod
    def on_fame_update(self, value: float):
        pass


class CombatEventReceiver(PassiveEventReceiver):
    @abc.abstractmethod
    def on_player_appeared(self, id: int, name: str):
        pass

    @abc.abstractmethod
    def on_damage_done(self, id: int, damage: float):
        pass

    @abc.abstractmethod
    def on_health_received(self, id: int, health: float):
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
