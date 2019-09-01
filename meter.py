import sys
import random

import clipboard

from PySide2.QtWidgets import (QApplication, QLabel, QPushButton,
                               QVBoxLayout, QWidget, QTableWidget, QHeaderView, QHBoxLayout,
                               QTableWidgetItem, QMenu, QAction)
from PySide2.QtCore import Slot, Qt

from PySide2 import QtGui

from PySide2.QtCore import QTimer

import libmeter


class Stat:
    def __init__(self, name, damage, time_in_combat, dps):
        self.name = name
        self.damage = '{0:.2f}'.format(damage)
        self.time_in_combat = '{0:.2f}'.format(time_in_combat)
        self.dps = '{0:.2f}'.format(dps)

    def __eq__(self, other):
        return self.name == other.name and self.damage == other.damage and self.time_in_combat == other.time_in_combat and self.dps == other.dps


def get_instance_session():
    session = libmeter.get_instance_session()
    # session = [
    #     {'player': 'A', 'damage': 1234.02, 'time_in_combat': 12.0, 'dps': 12.4234},
    #     {'player': 'B', 'damage': 5435.02, 'time_in_combat': 12.0, 'dps': 12},
    #     {'player': 'C', 'damage': 23.02, 'time_in_combat': 12.0, 'dps': 13},
    #     {'player': 'D', 'damage': 0, 'time_in_combat': 12.0, 'dps': 0}
    # ]

    return [Stat(s['player'], s['damage'], s['time_in_combat'], s['dps']) for s in session]


class Table(QTableWidget):
    def __init__(self):
        QTableWidget.__init__(self)

        self.session = None

        self.setColumnCount(3)
        self.setHorizontalHeaderLabels(["Player", "Damage", "DPS"])
        self.horizontalHeader().setSectionResizeMode(QHeaderView.Stretch)

    def fill(self, new_session):
        if not self._session_changed(new_session):
            return

        self.session = new_session

        session = sorted(new_session, key=lambda s: float(
            s.damage), reverse=True)
        session = [s for s in session if float(s.damage) > 0]

        self._align_rows(len(session))

        for idx, stats in enumerate(session):
            self.setItem(idx, 0, QTableWidgetItem(stats.name))
            self.setItem(idx, 1, QTableWidgetItem(stats.damage))
            self.setItem(idx, 2, QTableWidgetItem(stats.dps))

    def _align_rows(self, new_row_count):
        row_count = self.rowCount()
        diff = row_count - new_row_count

        if diff < 0:
            for i in range(row_count, row_count - diff):
                self.insertRow(i)
        else:
            for i in range(0, diff):
                self.removeRow(0)

    def _session_changed(self, session):
        return self.session != session


class MyWidget(QWidget):
    def __init__(self):
        QWidget.__init__(self)

        self.copy_button = QPushButton("&Copy stats", self)
        self.refresh_button = QPushButton("&Refresh stats", self)
        self.table = Table()

        self.layout = QVBoxLayout()
        self.layout.addWidget(self.table)
        self.layout.addWidget(self.copy_button)
        self.layout.addWidget(self.refresh_button)
        self.setLayout(self.layout)

        self.table.fill(get_instance_session())
        self.copy_button.clicked.connect(self.copy)
        self.refresh_button.clicked.connect(self.clear)

        timer = QTimer(self)
        timer.timeout.connect(lambda: self.table.fill(get_instance_session()))
        timer.start(5000)

    def copy(self):
        clip = "AO Damage Meter: \n"
        for i in range(self.table.rowCount()):
            clip += '{}. \t Player: {} \t Damage: {} \t DPS: {}'.format(i+1, self.table.item(i, 0).text(
            ), self.table.item(i, 1).text(), self.table.item(i, 2).text())
            clip += "\n"
        clipboard.copy(clip)

    def clear(self):
        self.table.fill([])


if __name__ == "__main__":
    libmeter.initialize()

    WIDTH = 256
    HEIGHT = 200

    app = QApplication(sys.argv)
    geometry = app.screens()[0].size()

    widget = MyWidget()
    widget.setWindowOpacity(0.5)
    widget.resize(WIDTH, HEIGHT)
    widget.move(0, geometry.height() - HEIGHT - 280)

    widget.setWindowTitle('Damage Meter')

    widget.setWindowFlag(Qt.WindowStaysOnTopHint)
    widget.show()

    sys.exit(app.exec_())
