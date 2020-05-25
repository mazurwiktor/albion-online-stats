import functools

from dataclasses import dataclass, field
from typing import List
import re
from src.utils.config import config

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
        if _is_visible_by_config():  # Note: only for testing purposes
            return True

        pattern = '|'.join(self.visible_players) + '|{}'.format(
            self.main_player_name) if self.visible_players else self.main_player_name
        return self.main_player_name and bool(_compile_pattern(pattern).match(name))

    @property
    def is_main_player_visible(self) -> bool:
        return self.main_player_name is not ""


@functools.lru_cache(maxsize=128)
def _is_visible_by_config():
    if 'visibility' in config()['app']:
        return True

    return False


@functools.lru_cache(maxsize=128)
def _compile_pattern(pattern: str):
    return re.compile(pattern)
