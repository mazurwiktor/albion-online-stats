import sys

from PySide2.QtCore import QTimer
from PySide2.QtCore import Qt

from PySide2.QtWidgets import QHBoxLayout
from PySide2.QtWidgets import QPushButton
from PySide2.QtWidgets import QVBoxLayout
from PySide2.QtWidgets import QWidget
from PySide2.QtWidgets import QLabel

import clipboard

from table import Table
from engine import get_instance_session
from engine import reset_instance_session

class BottomButtons(QWidget):
    def __init__(self, table):
        QWidget.__init__(self)

        self.table = table
        self.layout = QHBoxLayout()

        self.copy_button = QPushButton("&Copy", self)
        self.reset_button = QPushButton("&Reset", self)
        self.close_button = QPushButton("&Close", self)

        self.layout.addWidget(self.copy_button)
        self.layout.addWidget(self.reset_button)
        self.layout.addWidget(self.close_button)
        self.setLayout(self.layout)

        self.copy_button.clicked.connect(self.copy)
        self.reset_button.clicked.connect(self.reset)
        self.close_button.clicked.connect(self.close)

        self.copy_button.setObjectName('BottomButtons')
        self.reset_button.setObjectName('BottomButtons')
        self.close_button.setObjectName('BottomButtons')

    def copy(self):
        clip = "Damage: Current instance\n"
        for i in range(self.table.rowCount()):
            clip += '{}. {} {}-{}'.format(i+1, self.table.item(i, 0).text(
            ), self.table.item(i, 1).text(), self.table.item(i, 2).text())
            clip += "\n"
        clipboard.copy(clip)

    def reset(self):
        reset_instance_session()

    def close(self):
        sys.exit(0)

class MainWidget(QWidget):
    def __init__(self):
        QWidget.__init__(self)

        self.mouse_pos = None

        self.phoneLabel = QLabel("Damage: Current instance", self)

        self.table = Table()
        self.bottom_buttons = BottomButtons(self.table)

        self.layout = QVBoxLayout()
        self.layout.addWidget(self.phoneLabel)
        self.layout.addWidget(self.table)
        self.layout.addWidget(self.bottom_buttons)
        self.setLayout(self.layout)

        self.table.fill(get_instance_session())

        timer = QTimer(self)
        timer.timeout.connect(lambda: self.table.fill(get_instance_session()))
        timer.start(500)

    def mousePressEvent(self, event):
        self.mouse_pos = event.pos()

    def mouseMoveEvent(self, event):
        if event.buttons() & Qt.LeftButton:
            diff = event.pos() - self.mouse_pos
            newpos = self.pos() + diff

            self.move(newpos)
