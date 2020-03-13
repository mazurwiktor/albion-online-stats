import datetime

from PySide2.QtWidgets import QLabel
from PySide2 import QtGui
from PySide2 import QtCore

from ....utils import assets
from ....stats.list_item import PlayerListItem
from .... import engine


class PlayerStats(QLabel):
    def __init__(self):
        QLabel.__init__(self)

        self.update(0, 0.0, 0.0)

    def update(self, elapsed, fame, fame_per_hour):
        if engine.is_ready():
            self.setText("<b>{}</b> | Fame <b>{}</b> | FPH <b>{}</b>".format(
                datetime.timedelta(seconds=elapsed), fame, fame_per_hour))
        else:
            self.setText("Not ready: waiting for zone change")
