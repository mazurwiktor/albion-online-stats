import sys
import functools
import datetime

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

from .list_view import List
from . import about
from ... import engine
from ...utils import assets
from ...utils.config import config
from ...stats.list_item import PlayerListItem

class Mode:
    DMG_CURRENT_ZONE = 'Stats (dmg): Current zone'
    DMG_OVERALL = 'Stats (dmg): Overall'
    DMG_LAST_FIGHT = 'Stats (dmg): Last fight'
    CURRENT_ZONE = 'Stats (all): Current zone'
    OVERALL = 'Stats (all): Overall'
    LAST_FIGHT = 'Stats (all): Last fight'


class CopyButton(QPushButton):
    def __init__(self, clipboard, players : PlayerListItem, mode, fame):
        QPushButton.__init__(self)
        self.clipboard = clipboard
        self.players : PlayerListItem = players
        self.mode = mode
        self.fame = fame

        self.clicked.connect(self.copy)
        self.setIcon(QtGui.QIcon(assets.path('copy.png')))
        self.setToolTip("Copy to clipboard")

    def copy(self):
        clip = "{}, FPH: {}\nDMG: \n".format(
            self.mode(), self.fame())
        for index, i in enumerate(self.players()[:3]):
            clip += '{}. {}-{}/{}-{}%'.format(index+1,
                                              i.name, i.value, i.value_per_second, i.percentage)
            clip += "\n"
        clip += "(AOStats https://git.io/JeBD1)"

        self.clipboard.clear(mode=self.clipboard.Clipboard)
        self.clipboard.setText(clip, mode=self.clipboard.Clipboard)


class ResetButton(QPushButton):
    def __init__(self, mode):
        QPushButton.__init__(self)

        self.mode = mode

        self.setIcon(QtGui.QIcon(assets.path('reset.png')))
        self.setToolTip("Reset")
        self.clicked.connect(self.reset)

    def reset(self):
        reset = {
            Mode.DMG_CURRENT_ZONE: engine.reset_zone_stats,
            Mode.DMG_LAST_FIGHT: engine.reset_last_fight_stats,
            Mode.DMG_OVERALL: engine.reset_stats,
            Mode.CURRENT_ZONE: engine.reset_zone_stats,
            Mode.LAST_FIGHT: engine.reset_last_fight_stats,
            Mode.OVERALL: engine.reset_stats,
        }

        reset[self.mode()]()


class AboutButton(QPushButton):
    def __init__(self):
        QPushButton.__init__(self)

        self.about = about.About()
        self.setIcon(QtGui.QIcon(assets.path('about.png')))
        self.setToolTip("About")
        self.clicked.connect(self.about.show)


class CloseButton(QPushButton):
    def __init__(self, is_visible):
        QPushButton.__init__(self)

        self.setIcon(QtGui.QIcon(assets.path('close.png')))
        self.setToolTip("Close")
        self.clicked.connect(self.close)
        if not is_visible:
            self.hide()

    def close(self):
        sys.exit(0)


class InteractiveBar(QWidget):
    def __init__(self, table, clipboard):
        QWidget.__init__(self)
        self.mode = ModeWidget()
        self.table = table
        self.layout = QHBoxLayout()
        self.fame_per_hour = 0.0

        self.layout.addWidget(self.mode)

        self.copy_button = CopyButton(
            clipboard, self.table.get_player_list_items, self.mode.get_mode, self.get_fame_per_hour)
        self.layout.addWidget(self.copy_button)

        self.reset_button = ResetButton(self.mode.get_mode)
        self.layout.addWidget(self.reset_button)

        self.about_button = AboutButton()
        self.layout.addWidget(self.about_button)

        self.close_button = CloseButton(config()['window']['frameless'])
        self.layout.addWidget(self.close_button)
        
        self.setLayout(self.layout)

    def get_fame_per_hour(self):
        return self.fame_per_hour

    def set_fame_per_hour(self, fpm):
        self.fame_per_hour = fpm

    def close(self):
        sys.exit(0)


class ModeWidget(QComboBox):
    def __init__(self):
        QComboBox.__init__(self)
        self.addItem(Mode.DMG_CURRENT_ZONE)
        self.addItem(Mode.DMG_OVERALL)
        self.addItem(Mode.DMG_LAST_FIGHT)
        self.addItem(Mode.CURRENT_ZONE)
        self.addItem(Mode.OVERALL)
        self.addItem(Mode.LAST_FIGHT)

    def get_mode(self):
        return self.currentText()

class MainWidget(QWidget):
    def __init__(self, clipboard):
        QWidget.__init__(self)

        self.mouse_pos = None
        self.table = List()
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
        player_list_items, fame_stat, elapsed = self.session()
        self.table.update(player_list_items)
        self.bar.set_fame_per_hour(fame_stat.fame_per_hour)
        self.fame_label.setText("<b>{}</b> | Fame <b>{}</b> | FPH <b>{}</b>".format(
            datetime.timedelta(seconds=elapsed), fame_stat.fame, fame_stat.fame_per_hour))

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
            Mode.DMG_CURRENT_ZONE: functools.partial(engine.zone_stats, with_damage=True),
            Mode.DMG_LAST_FIGHT: functools.partial(engine.last_fight_stats, with_damage=True),
            Mode.DMG_OVERALL: functools.partial(engine.overall_stats, with_damage=True),
            Mode.CURRENT_ZONE: engine.zone_stats,
            Mode.LAST_FIGHT: engine.last_fight_stats,
            Mode.OVERALL: engine.overall_stats
        }

        return sessions[self.bar.mode.currentText()]()
