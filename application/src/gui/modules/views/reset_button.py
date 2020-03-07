
from typing import Callable

from PySide2.QtWidgets import QPushButton
from PySide2 import QtGui

from ....utils import assets
from ....stats.list_item import PlayerListItem
from .... import engine

from .stats_type import StatsType

class ResetButton(QPushButton):
    def __init__(self):
        QPushButton.__init__(self)

        self.stat_type: str = StatsType.ZONE

        self.setIcon(QtGui.QIcon(assets.path('reset.png')))
        self.setToolTip("Reset")
        self.clicked.connect(self.reset)

    def reset(self):
        reset = {
            StatsType.ZONE: engine.reset_zone_stats,
            StatsType.LAST_FIGHT: engine.reset_last_fight_stats,
            StatsType.OVERALL: engine.reset_stats,
        }

        reset[self.stat_type]()
    
    def update(self, stat_type: str):
        self.stat_type = stat_type
