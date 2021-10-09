# from PySide6.QtCore import QTimer
# from PySide6.QtCore import Qt

from typing import Callable

from PySide6.QtWidgets import QHBoxLayout  # type: ignore
from PySide6.QtWidgets import QVBoxLayout  # type: ignore
from PySide6.QtWidgets import QComboBox  # type: ignore
from PySide6.QtWidgets import QWidget  # type: ignore

from ....stats.list_item import PlayerListItem

from .list_view import ListItemView
from .stats_type import StatsType
from .player_stats import PlayerStats
from .copy_button import CopyButton
from .reset_button import ResetButton


class ModeWidget(QComboBox):
    def __init__(self):
        QComboBox.__init__(self)
        self.addItem(StatsType.OVERALL)
        self.addItem(StatsType.ZONE)
        self.addItem(StatsType.LAST_FIGHT)

    def get_mode(self):
        return self.currentText()


class SessionBar(QWidget):
    def __init__(self, clipboard):
        QWidget.__init__(self)
        self.layout = QHBoxLayout()

        self.reset_button = ResetButton()
        self.layout.addWidget(self.reset_button)

        self.copy_button = CopyButton(clipboard)
        self.layout.addWidget(self.copy_button)

        self.mode = ModeWidget()
        self.layout.addWidget(self.mode)

        self.player_stats = PlayerStats()
        self.layout.addWidget(self.player_stats)

        self.setLayout(self.layout)

    def update(self):
        self.reset_button.update(self.mode.get_mode())


class ModeBasedListView(QWidget):
    def __init__(self, clipboard):
        QWidget.__init__(self)
        self.layout = QVBoxLayout()
        self.layout.setSpacing(0)
        self.layout.setContentsMargins(0, 0, 0, 0)
        self.session_bar = SessionBar(clipboard)
        self.layout.addWidget(self.session_bar)

        self.list_view = ListItemView()
        self.layout.addWidget(self.list_view)

        self.setLayout(self.layout)

    def update(self, prefix, elapsed, fame, fame_per_hour, players: PlayerListItem):
        self.session_bar.player_stats.update(elapsed, fame, fame_per_hour)
        self.session_bar.copy_button.update(
            prefix, self.list_view.get_player_list_items(), self.session_bar.mode.get_mode(), fame)
        self.list_view.update(players)
        self.session_bar.update()

    def get_mode(self) -> str:
        return self.session_bar.mode.get_mode()
