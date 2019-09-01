import libmeter

import sys
import random
from PySide2.QtWidgets import (QApplication, QLabel, QPushButton,
                               QVBoxLayout, QWidget, QTableWidget, QHeaderView, QHBoxLayout,
                               QTableWidgetItem)
from PySide2.QtCore import Slot, Qt

from PySide2.QtCore import QTimer

import libmeter


class Stat:
    def __init__(self, name, damage, time_in_combat, dps):
        self.name = name
        self.damage = str(damage)
        self.time_in_combat = str(time_in_combat)
        self.dps = str(dps)

    def __eq__(self, other):
        return self.name == other.name and self.damage == other.damage and self.time_in_combat == other.time_in_combat and self.dps == other.dps


def get_instance_session():
    session = libmeter.get_instance_session()
    # session = [
        # {'player': 'A', 'damage': 1234.02, 'time_in_combat': 12.0, 'dps': 12.4234},
        # {'player': 'B', 'damage': 5435.02, 'time_in_combat': 12.0, 'dps': 12},
        # {'player': 'C', 'damage': 23.02, 'time_in_combat': 12.0, 'dps': 13},
        # {'player': 'D', 'damage': 23.02, 'time_in_combat': 12.0, 'dps': 0}
    # ]

    return [Stat(s['player'], s['damage'], s['time_in_combat'], s['dps']) for s in session]


class MyWidget(QWidget):
    def __init__(self):
        QWidget.__init__(self)

        self.session = None

        self.table = QTableWidget()
        self.table.setColumnCount(3)
        self.table.setHorizontalHeaderLabels(["Player", "Damage", "DPS"])
        self.table.horizontalHeader().setSectionResizeMode(QHeaderView.Stretch)

        self.layout = QHBoxLayout()
        self.layout.addWidget(self.table)
        self.setLayout(self.layout)

        self.fill_table()

        timer = QTimer(self)
        timer.timeout.connect(self.fill_table)
        timer.start(5000)

    def session_changed(self, session):
        return self.session != session

    def fill_table(self, data=None):
        new_session = get_instance_session()

        if not self.session_changed(new_session):
            return

        self.session = new_session

        session = sorted(new_session, key=lambda s: float(s.damage), reverse=True)

        for idx, stats in enumerate(session):
            self.table.removeRow(idx)
            self.table.insertRow(idx)
            self.table.setItem(idx, 0, QTableWidgetItem(stats.name))
            self.table.setItem(idx, 1, QTableWidgetItem(stats.damage))
            self.table.setItem(idx, 2, QTableWidgetItem(stats.dps))


if __name__ == "__main__":
    libmeter.initialize()

    WIDTH = 512
    HEIGHT = 180

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
