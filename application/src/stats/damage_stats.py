# Example output
# 'player': 'Cursed',
# 'damage': 1100.0,
# 'time_in_combat': 12.0,
# 'dps': 222,
# 'items': {
#   'weapon': 'T5_MAIN_CURSEDSTAFF@2'
#  }

from datetime import datetime


class Player:
    class CombatTime:
        def __init__(self):
            self.entered_combat = None
            self.time_in_combat = 0.0

    class CombatState:
        InCombat = 1
        OutOfCombat = 2

    def __init__(self, name):
        self.name = name
        self.damage_done = 0.0
        self.items = {'weapon': None}
        self.combat_time = self.CombatTime()
        self.combat_state = self.CombatState.OutOfCombat

    @staticmethod
    def from_other(other):
        Player(other.name)

    def register_items(self, value):
        self.items = value

    def register_damage_done(self, value):
        print("{} dealing damage {}".format(self.name, value))
        if self.combat_state == self.CombatState.OutOfCombat:
            return

        self.damage_done += value

    def enter_combat(self):
        print("{} enters combat".format(self.name))
        self.combat_time.entered_combat = datetime.now()
        self.combat_state = self.CombatState.InCombat

    def leave_combat(self):
        print("{} leaves combat".format(self.name))
        if self.combat_time.entered_combat:
            self.combat_time.time_in_combat += (
                datetime.now() - self.combat_time.entered_combat).microseconds / 100.0
        self.combat_state = self.CombatState.OutOfCombat

    @property
    def time_in_combat(self):
        if self.combat_state == self.CombatState.InCombat:
            if self.combat_time.entered_combat:
                return self.combat_time.time_in_combat + (datetime.now() - self.combat_time.entered_combat).microseconds / 100.0

        return self.combat_time.time_in_combat

    @property
    def dps(self):
        if self.time_in_combat == 0.0:
            return 0.0
        
        return (self.damage_done / self.time_in_combat) * 1000.0


    def stats(self):
        return {
            'player': self.name,
            'damage': self.damage_done,
            'time_in_combat': self.time_in_combat,
            'dps': self.dps,
            'items': self.items}


class DamageStats:
    def __init__(self, players=None):
        if not players:
            players = {}
        self.players = players

    @staticmethod
    def from_other(other):
        DamageStats({k: Player.from_other(v)} for k, v in other.players)

    def add_player(self, player_id, name):
        self.players[player_id] = Player(name)

    def add_items(self, player_id, items):
        self.players[player_id].register_items(items)

    def enter_combat(self, player_id):
        self.players[player_id].enter_combat()

    def leave_conbat(self, player_id):
        self.players[player_id].leave_combat()

    def register_damage_done(self, player_id, value):
        self.players[player_id].register_damage_done(value)

    def stats(self):
        return [player.stats() for player in self.players.values()]


def combined_stats(stats_list):
    combined = {}

    for stats in stats_list:
        if stats['player'] in combined:
            current = combined[stats['player']]
            current['damage'] += stats['damage']
            current['time_in_combat'] += stats['time_in_combat']
            current['dps'] += stats['dps']
            current['items'] = stats['items']
        else:
            combined[stats['player']] = stats

    return [s for s in combined.values()]
