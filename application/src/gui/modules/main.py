import sys
import functools

from PySide2 import QtGui
from PySide2.QtCore import Qt
from PySide2.QtCore import QTimer
from PySide2.QtWidgets import QComboBox
from PySide2.QtWidgets import QHBoxLayout
from PySide2.QtWidgets import QPushButton
from PySide2.QtWidgets import QVBoxLayout
from PySide2.QtWidgets import QWidget

from ... import engine

from ...utils import assets
from ...utils.config import config

from .view_type import ViewType
from .views import ModeBasedListView
from .views import StatsType

from . import about


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


class ViewTypeWidget(QComboBox):
    def __init__(self):
        QComboBox.__init__(self)
        self.addItem(ViewType.DMG)
        self.setEnabled(False)

    def get_view_type(self):
        return self.currentText()


class TopBar(QWidget):
    def __init__(self):
        QWidget.__init__(self)
        self.view_type = ViewTypeWidget()
        self.layout = QHBoxLayout()
        self.layout.setSpacing(0)
        self.layout.setMargin(0)
        self.layout.addWidget(self.view_type)

        self.about_button = AboutButton()
        self.layout.addWidget(self.about_button)

        self.close_button = CloseButton(config()['window']['frameless'])
        self.layout.addWidget(self.close_button)

        self.setLayout(self.layout)


class MainWidget(QWidget):
    def __init__(self, clipboard):
        QWidget.__init__(self)

        self.mouse_pos = None

        self.layout = QVBoxLayout()
        self.top_bar = TopBar()
        self.layout.addWidget(self.top_bar)

        self.view = ModeBasedListView(clipboard)
        self.layout.addWidget(self.view)

        self.setLayout(self.layout)
        self.refresh()

        timer = QTimer(self)
        timer.timeout.connect(self.refresh)
        timer.start(500)

    def refresh(self):
        player_list_items, fame_stat, elapsed = self.damage_stats()
        self.view.update(
            self.top_bar.view_type.get_view_type(),
            elapsed,
            fame_stat.fame,
            fame_stat.fame_per_hour,
            player_list_items
        )

    def mousePressEvent(self, event):
        self.mouse_pos = event.pos()

    def mouseMoveEvent(self, event):
        invalid_regions = (
            self.top_bar.geometry(),
            self.view.geometry(),
        )

        if any(region.contains(event.pos()) for region in invalid_regions):
            return
        if not self.mouse_pos:
            return
        if any(region.contains(self.mouse_pos) for region in invalid_regions):
            return


        if event.buttons() & Qt.LeftButton:
            diff = event.pos() - self.mouse_pos
            newpos = self.pos() + diff

            self.move(newpos)

    def damage_stats(self):
        stats = {
            ViewType.DMG: {
                StatsType.ZONE: functools.partial(engine.zone_stats),
                StatsType.LAST_FIGHT: functools.partial(engine.last_fight_stats),
                StatsType.OVERALL: functools.partial(engine.overall_stats),
            }
        }

        return stats[self.top_bar.view_type.get_view_type()][self.view.get_mode()]()
