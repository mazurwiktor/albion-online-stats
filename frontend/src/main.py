import sys
import os

from PySide2.QtCore import QTimer
from PySide2.QtCore import Qt

from PySide2.QtWidgets import QHBoxLayout
from PySide2.QtWidgets import QPushButton
from PySide2.QtWidgets import QVBoxLayout
from PySide2.QtWidgets import QWidget
from PySide2.QtWidgets import QLabel
from PySide2.QtWidgets import QComboBox

from PySide2 import QtGui
from PySide2 import QtCore

from .dmg_list import DmgList
from . import about
from . import engine

assets_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'assets')

class Mode:
    CURRENT_ZONE = 'Statistics: Current zone'
    OVERALL = 'Statistics: Overall'
    LAST_FIGHT = 'Statistics: Last fight'


class InteractiveBar(QWidget):
    def __init__(self, table, clipboard):
        QWidget.__init__(self)
        self.mode = ModeWidget()
        self.table = table
        self.layout = QHBoxLayout()
        self.about = about.About()
        self.fame_per_minute = 0.0
        self.clipboard = clipboard

        self.copy_button = QPushButton(self)
        self.copy_button.setIcon(QtGui.QIcon(os.path.join(assets_path, 'copy.png')))

        self.reset_button = QPushButton()
        self.reset_button.setIcon(QtGui.QIcon(os.path.join(assets_path, 'reset.png')))

        self.close_button = QPushButton(self)
        self.close_button.setIcon(QtGui.QIcon(os.path.join(assets_path, 'close.png')))

        self.about_button = QPushButton(self)
        self.about_button.setIcon(QtGui.QIcon(os.path.join(assets_path, 'about.png')))

        self.layout.addWidget(self.mode)
        self.layout.addWidget(self.copy_button)
        self.layout.addWidget(self.reset_button)
        self.layout.addWidget(self.about_button)
        self.layout.addWidget(self.close_button)
        self.setLayout(self.layout)

        self.copy_button.clicked.connect(self.copy)
        self.reset_button.clicked.connect(self.reset)
        self.close_button.clicked.connect(self.close)
        self.about_button.clicked.connect(self.about.show)

    def copy(self):
        model = self.table.model
        items = sorted(
            [model.item(i) for i in range(model.rowCount())], 
            key=lambda i: i.damage, 
            reverse=True)
        clip = "{}, Fame/min: {}\nDMG: \n".format(self.mode.currentText(), self.fame_per_minute)
        for index, i in enumerate(items[:3]):
            clip += '{}. {}-{}-{}%'.format(index+1, i.name, i.damage, i.percentage)
            clip += "\n"
        clip += "(AOStats https://git.io/JeBD1)"

        self.clipboard.clear(mode=self.clipboard.Clipboard )
        self.clipboard.setText(clip, mode=self.clipboard.Clipboard)

    def set_fame_per_minute(self, fpm):
        self.fame_per_minute = fpm

    def reset(self):
        reset = {
            Mode.CURRENT_ZONE: engine.reset_zone_stats,
            Mode.LAST_FIGHT: engine.reset_last_fight_stats,
            Mode.OVERALL: engine.reset_stats
        }

        reset[self.mode.currentText()]()

    def close(self):
        sys.exit(0)

class ModeWidget(QComboBox):
    def __init__(self):
        QComboBox.__init__(self)

        self.addItem(Mode.CURRENT_ZONE)
        self.addItem(Mode.OVERALL)
        self.addItem(Mode.LAST_FIGHT)


class MainWidget(QWidget):
    def __init__(self, clipboard):
        QWidget.__init__(self)

        self.mouse_pos = None
        self.table = DmgList()
        self.fame_label = QLabel()
        self.fame_label.setAlignment(QtCore.Qt.AlignmentFlag.AlignHCenter)
        self.bar = InteractiveBar(self.table, clipboard)

        self.layout = QVBoxLayout()
        self.layout.addWidget(self.bar)
        self.layout.addWidget(self.fame_label)
        self.layout.addWidget(self.table)
        self.setLayout(self.layout)

        self.refresh()

        timer = QTimer(self)
        timer.timeout.connect(self.refresh)
        timer.start(500)

    def refresh(self):
        damage_session, fame_stat = self.session()
        self.table.update(damage_session)
        self.bar.set_fame_per_minute(fame_stat.fame_per_minute)
        self.fame_label.setText("Fame <b>{}</b> | Fame per minute <b>{}</b> | Party members <b>{}</b>".format(
            fame_stat.fame, fame_stat.fame_per_minute, len(engine.get_party_members())))
    
    def mousePressEvent(self, event):
        self.mouse_pos = event.pos()

    def mouseMoveEvent(self, event):
        if self.bar.geometry().contains(event.pos()):
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

        return sessions[self.bar.mode.currentText()]()
