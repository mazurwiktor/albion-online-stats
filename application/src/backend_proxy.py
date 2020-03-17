try:
    import aostats # type: ignore
except:
    class aostats:  # type: ignore
        @staticmethod
        def initialize(_):
            return InitializationResult.NetworkInterfaceListMissing

        @staticmethod
        def subscribe(_):
            pass

from .environment import TEST_ENV_ENABLED
from . import environment


class InitializationResult:
    Ok = 0
    UnknownFailure = 1
    NetworkInterfaceListMissing = 2


INITIALIZATION_RESULT = {
    0: InitializationResult.Ok,
    1: InitializationResult.UnknownFailure,
    2: InitializationResult.NetworkInterfaceListMissing
}


def initialize():
    if TEST_ENV_ENABLED:
        return InitializationResult.Ok

    try:
        result = aostats.initialize()

        return INITIALIZATION_RESULT[result]
    except:
        pass


def subscribe(callback):
    if TEST_ENV_ENABLED:
        return _testing_call_sequence(callback)

    aostats.subscribe(callback)


def _testing_call_sequence(callback):
    def add_player(id, name, weapon, event_name='PlayerAppeared'):
        return [
            {'name': event_name, 'value': {'id': id, 'name': name}},
            {'name': 'UpdateItems', 'value': {'source': id, 'value': {
                'weapon': weapon}}},
        ]

    def deal_damage(id, name, damage):
        return [
            {'name': 'EnterCombat', 'value': {'id': id, 'name': name}},
            {'name': 'DamageDone', 'value': {'source': id, 'value': damage}},
            {'sleep': 0.1},
            {'name': 'LeaveCombat', 'value': {'id': id, 'name': name}},
        ]

    import threading
    import time

    sequence = {}
    sequence[environment.TestType.Simple] = [
        {'name': 'ZoneChange'},
        *add_player(0, 'Arcane', 'T4_MAIN_ARCANESTAFF@3',
                    event_name='MainPlayerAppeared'),
        *add_player(1, 'Cursed', 'T5_MAIN_CURSEDSTAFF@2'),
        *add_player(2, 'Fire', 'T5_MAIN_FIRESTAFF@1'),
        *add_player(3, 'Frost', 'T5_MAIN_FROSTSTAFF@1'),
        *add_player(4, 'Holy', 'T6_MAIN_HOLYSTAFF'),
        *add_player(5, 'Nature', 'T8_MAIN_NATURESTAFF@3'),
        *add_player(6, 'Axe', 'T8_MAIN_AXE'),
        *add_player(7, 'Dagger', 'T8_MAIN_DAGGER@2'),
        *add_player(8, 'Hammer', 'T7_MAIN_HAMMER@2'),
        *add_player(9, 'Mace', 'T6_MAIN_MACE@2'),
        *add_player(10, 'Quarterstaff', 'T5_2H_IRONCLADEDSTAFF@2'),
        *add_player(11, 'Spear', 'T8_MAIN_SPEAR@2'),
        *add_player(12, 'Sword', 'T7_2H_CLAYMORE@2'),
        *add_player(13, 'Bow', 'T8_2H_BOW@2'),
        *add_player(14, 'Crossbow', 'T8_2H_CROSSBOWLARGE@3'),
        *deal_damage(2, 'Fire', 80.0),
        *deal_damage(3, 'Frost', 200.0),
        *deal_damage(6, 'Axe', 150.0),
        *deal_damage(7, 'Dagger', 175.0),
        *deal_damage(8, 'Hammer', 80.0),
        *deal_damage(9, 'Mace', 80.0),
        *deal_damage(10, 'Quarterstaff', 199.0),
        *deal_damage(11, 'Spear', 96.0),
        *deal_damage(12, 'Sword', 80.0),
        *deal_damage(13, 'Bow', 221.0),
        {'name': 'UpdateFame', 'value': {'value': 2000}},
    ]

    sequence[environment.TestType.ZoneChange] = [
        {'name': 'ZoneChange'},
        *add_player(0, 'Arcane', 'T4_MAIN_ARCANESTAFF@3',
                    event_name='MainPlayerAppeared'),
        *add_player(2, 'Fire', 'T5_MAIN_FIRESTAFF@1'),
        *add_player(3, 'Frost', 'T5_MAIN_FROSTSTAFF@1'),
        *add_player(6, 'Axe', 'T8_MAIN_AXE'),
        *add_player(7, 'Dagger', 'T8_MAIN_DAGGER@2'),
        *add_player(8, 'Hammer', 'T7_MAIN_HAMMER@2'),
        *deal_damage(2, 'Fire', 80.0),
        *deal_damage(3, 'Frost', 200.0),
        *deal_damage(6, 'Axe', 150.0),
        *deal_damage(7, 'Dagger', 175.0),
        *deal_damage(8, 'Hammer', 80.0),
        {'name': 'UpdateFame', 'value': {'value': 2000}},
        {'name': 'ZoneChange'},
        *add_player(2, 'Fire', 'T5_MAIN_FIRESTAFF@1'),
        *deal_damage(2, 'Fire', 80.0),
        {'name': 'UpdateFame', 'value': {'value': 1000}},
    ]

    def run_sequence():
        for v in sequence[environment.test_type]:
            if 'sleep' in v:
                time.sleep(v['sleep'])
            else:
                callback(v)
    thread = threading.Thread(target=run_sequence, daemon=True)
    thread.start()
