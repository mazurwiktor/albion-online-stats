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

from .dmg_list import DmgList
from . import about
from . import engine

class Mode:
    CURRENT_ZONE = 'Statistics: Current zone'
    OVERALL = 'Statistics: Overall'
    LAST_FIGHT = 'Statistics: Last fight'


class BottomButtons(QWidget):
    def __init__(self, table, mode):
        QWidget.__init__(self)
        self.mode = mode
        self.table = table
        self.layout = QHBoxLayout()
        self.about = about.About()

        self.copy_button = QPushButton("&Copy", self)
        self.reset_button = QPushButton("&Reset", self)
        self.close_button = QPushButton("&Close", self)
        self.about_button = QPushButton("&About", self)
        self.layout.addWidget(self.copy_button)
        self.layout.addWidget(self.reset_button)
        self.layout.addWidget(self.about_button)
        self.layout.addWidget(self.close_button)
        self.setLayout(self.layout)

        self.copy_button.clicked.connect(self.copy)
        self.reset_button.clicked.connect(self.reset)
        self.close_button.clicked.connect(self.close)
        self.about_button.clicked.connect(self.about.show)

        self.copy_button.setObjectName('BottomButtons')
        self.reset_button.setObjectName('BottomButtons')
        self.about_button.setObjectName('BottomButtons')
        self.close_button.setObjectName('BottomButtons')

    def copy(self):
        clip = "{}\n".format(self.mode())
        model = self.table.model
        items = sorted(
            [model.item(i) for i in range(model.rowCount())], 
            key=lambda i: i.damage, 
            reverse=True)
        for index, i in enumerate(items):
            clip += '{}. {}-{}%'.format(index+1, i.name, i.percentage)
            clip += "\n"
        clip += "(AOStats https://git.io/JeBD1)"
        print(clip)
        clipboard.copy(clip)

    def reset(self):
        reset = {
            Mode.CURRENT_ZONE: engine.reset_zone_stats,
            Mode.LAST_FIGHT: engine.reset_last_fight_stats,
            Mode.OVERALL: engine.reset_stats
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
        self.table = DmgList()
        self.fame_label = QLabel()
        self.bottom_buttons = BottomButtons(
            self.table, lambda: self.mode.currentText())

        self.layout = QVBoxLayout()
        self.layout.addWidget(self.mode)
        self.layout.addWidget(self.fame_label)
        self.layout.addWidget(self.table)
        self.layout.addWidget(self.bottom_buttons)
        self.setLayout(self.layout)

        self.refresh()

        timer = QTimer(self)
        timer.timeout.connect(self.refresh)
        timer.start(500)

    def refresh(self):
        damage_session, fame_stat = self.session()
        self.table.update(damage_session)
        self.fame_label.setText("Fame <b>{}</b> | Fame per minute <b>{}</b> | Party members <b>{}</b>".format(
            fame_stat.fame, fame_stat.fame_per_minute, len(engine.get_party_members())))

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
            Mode.CURRENT_ZONE: engine.zone_stats,
            Mode.LAST_FIGHT: engine.last_fight_stats,
            Mode.OVERALL: engine.overall_stats
        }

        return sessions[self.mode.currentText()]()
