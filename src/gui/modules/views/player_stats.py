import datetime

from PySide6.QtWidgets import QLabel  # type: ignore
from PySide6 import QtGui  # type: ignore
from PySide6 import QtCore  # type: ignore

from src.utils import assets, number
from src.stats.list_item import PlayerListItem
from src import engine


class PlayerStats(QLabel):
    def __init__(self):
        QLabel.__init__(self)

        self.update(0, number.Number(0.0), number.Number(0.0))

    def update(self, elapsed, fame, fame_per_hour):
        if engine.is_ready():
            self.setText("<b>{}</b> | Fame <b>{}</b> | FPH <b>{}</b>".format(
                datetime.timedelta(seconds=elapsed), fame, fame_per_hour))
        else:
            self.setText("Not ready: waiting for zone change")
