import sys

from PySide2.QtCore import QTimer
from PySide2.QtCore import Qt

from PySide2.QtWidgets import QHBoxLayout
from PySide2.QtWidgets import QPushButton
from PySide2.QtWidgets import QVBoxLayout
from PySide2.QtWidgets import QWidget
from PySide2.QtWidgets import QLabel
from PySide2.QtWidgets import QComboBox

import clipboard

from table import Table
import engine


class Mode:
    CURRENT_ZONE = 'Damage: Current zone'
    OVERALL = 'Damage: Overall'
    LAST_FIGHT = 'Damage: Last fight'


class BottomButtons(QWidget):
    def __init__(self, table, mode):
        QWidget.__init__(self)
        self.mode = mode
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
        clip = "{}\n".format(self.mode())
        for i in range(self.table.rowCount()):
            clip += '{}. {} {}-{}'.format(i+1, self.table.item(i, 0).text(
            ), self.table.item(i, 1).text(), self.table.item(i, 2).text())
            clip += "\n"
        clipboard.copy(clip)

    def reset(self):
        reset = {
            Mode.CURRENT_ZONE: engine.new_zone_session,
            Mode.LAST_FIGHT: engine.new_last_fight_session,
            Mode.OVERALL: engine.reset_sessions
        }

        reset[self.mode()]()

    def close(self):
        sys.exit(0)


class ModeWidget(QComboBox):
    def __init__(self):
        QComboBox.__init__(self)

        self.addItem(Mode.CURRENT_ZONE)
        self.addItem(Mode.OVERALL)
        self.addItem(Mode.LAST_FIGHT)


class MainWidget(QWidget):
    def __init__(self):
        QWidget.__init__(self)

        self.mouse_pos = None
        self.mode = ModeWidget()
        self.table = Table()
        self.bottom_buttons = BottomButtons(
            self.table, lambda: self.mode.currentText())

        self.layout = QVBoxLayout()
        self.layout.addWidget(self.mode)
        self.layout.addWidget(self.table)
        self.layout.addWidget(self.bottom_buttons)
        self.setLayout(self.layout)

        self.table.fill(self.session())

        timer = QTimer(self)
        timer.timeout.connect(lambda: self.table.fill(self.session()))
        timer.start(500)

    def mousePressEvent(self, event):
        self.mouse_pos = event.pos()

    def mouseMoveEvent(self, event):
        if self.mode.geometry().contains(event.pos()):
            return
        if not self.mouse_pos:
            return

        if event.buttons() & Qt.LeftButton:
            diff = event.pos() - self.mouse_pos
            newpos = self.pos() + diff

            self.move(newpos)

    def session(self):
        sessions = {
            Mode.CURRENT_ZONE: engine.get_zone_session,
            Mode.LAST_FIGHT: engine.get_last_fight_session,
            Mode.OVERALL: engine.get_overall_session
        }

        return sessions[self.mode.currentText()]()
