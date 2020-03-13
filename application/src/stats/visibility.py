from dataclasses import dataclass, field
from typing import List
import re


from ..event_receiver import VisibilityEventReceiver


@dataclass
class Visibility(VisibilityEventReceiver):
    main_player_name: str = ""
    visible_players: List[str] = field(default_factory=lambda: [])

    def on_player_appeared(self, main_player_name: str):
        self.main_player_name = main_player_name

    def on_visible_players_changed(self, visible_players: List[str]):
        self.visible_players = visible_players

    def test(self, name):
        pattern = re.compile('|'.join(self.visible_players) + '|{}'.format(
            self.main_player_name) if self.visible_players else self.main_player_name)
        return self.main_player_name and bool(pattern.match(name))
