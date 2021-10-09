
from PySide6.QtWidgets import QPushButton  # type: ignore
from PySide6 import QtGui  # type: ignore

from ....utils import assets
from ....stats.list_item import PlayerListItem

from .stats_type import StatsType


class CopyButton(QPushButton):
    def __init__(self, clipboard):
        QPushButton.__init__(self)
        self.clipboard = clipboard
        self.prefix = '<< INITIAL PREFIX >>'
        self.players: PlayerListItem = []
        self.mode = StatsType.ZONE
        self.fame = 0.0

        self.clicked.connect(self.copy)
        self.setIcon(QtGui.QIcon(assets.path('copy.png')))
        self.setToolTip("Copy to clipboard")

    def copy(self):
        clip = "{}: {}, FPH: {}\n".format(
            self.prefix, self.mode, self.fame)
        for index, i in enumerate(self.players[:3]):
            clip += '{}. {}-{}/{}-{}%'.format(index+1,
                                              i.name, i.value, i.value_per_second, i.percentage)
            clip += "\n"
        clip += "(AOStats https://git.io/JeBD1)"

        self.clipboard.clear(mode=self.clipboard.Clipboard)
        self.clipboard.setText(clip, mode=self.clipboard.Clipboard)

    def update(self, prefix: str, players: PlayerListItem, mode: str, fame: float):
        self.prefix = prefix
        self.players = players
        self.mode = mode
        self.fame = fame
