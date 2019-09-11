from PySide2.QtCore import QTimer
from PySide2.QtCore import Qt

from PySide2.QtWidgets import QHBoxLayout
from PySide2.QtWidgets import QPushButton
from PySide2.QtWidgets import QVBoxLayout
from PySide2.QtWidgets import QWidget

import clipboard

from table import Table
from engine import get_instance_session

class MainWidget(QWidget):
    def __init__(self):
        QWidget.__init__(self)

        self.mouse_pos = None

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

    def mousePressEvent(self, event):
        self.mouse_pos = event.pos()

    def mouseMoveEvent(self, event):
        if event.buttons() & Qt.LeftButton:
            diff = event.pos() - self.mouse_pos
            newpos = self.pos() + diff

            self.move(newpos)

    def copy(self):
        clip = "AO Damage Meter: \n"
        for i in range(self.table.rowCount()):
            clip += '{}. \t Player: {} \t Damage: {} \t DPS: {}'.format(i+1, self.table.item(i, 0).text(
            ), self.table.item(i, 1).text(), self.table.item(i, 2).text())
            clip += "\n"
        clipboard.copy(clip)

    def clear(self):
        self.table.fill([])
