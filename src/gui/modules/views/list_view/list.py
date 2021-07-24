import collections
import functools
import re
from typing import List

from PySide2 import QtCore  # type: ignore
from PySide2 import QtGui  # type: ignore
from PySide2.QtCore import Qt  # type: ignore
from PySide2.QtGui import QStandardItem  # type: ignore
from PySide2.QtGui import QStandardItemModel  # type: ignore
from PySide2.QtWidgets import QApplication  # type: ignore
from PySide2.QtWidgets import QComboBox  # type: ignore
from PySide2.QtWidgets import QHBoxLayout  # type: ignore
from PySide2.QtWidgets import QLabel  # type: ignore
from PySide2.QtWidgets import QListView  # type: ignore
from PySide2.QtWidgets import QMessageBox  # type: ignore
from PySide2.QtWidgets import QProgressBar  # type: ignore
from PySide2.QtWidgets import QPushButton  # type: ignore
from PySide2.QtWidgets import QVBoxLayout  # type: ignore

from .....stats.list_item import PlayerListItem
from .....utils import assets
from .....utils import names
from .....utils import weapon

from .icon import get_weapon_icon as player_icon

Style = collections.namedtuple('Style', ['bg', 'fg'])

enchant_re = re.compile(r"(.*)@(\d+)")


def map_name(name):

    match = enchant_re.match(name)
    if match:
        return names.map_name(match[1])

    return names.map_name(name)


def player_style(items):
    weapon_type = weapon.get_weapon_type(items)

    if weapon_type == weapon.WeaponType.Arcane:
        return Style(bg='#f032e6', fg='#ffffff')
    elif weapon_type == weapon.WeaponType.Axe:
        return Style(bg='#800000', fg='#7fffff')
    elif weapon_type == weapon.WeaponType.Bow:
        return Style(bg='#469990', fg='#ED9247')
    elif weapon_type == weapon.WeaponType.Crossbow:
        return Style(bg='#000075', fg='#ffff8a')
    elif weapon_type == weapon.WeaponType.Curse:
        return Style(bg='#911eb4', fg='#6ee14b')
    elif weapon_type == weapon.WeaponType.Dagger:
        return Style(bg='#4d5f20', fg='#ffffff')
    elif weapon_type == weapon.WeaponType.Fire:
        return Style(bg='#e6194B', fg='#4763ED')
    elif weapon_type == weapon.WeaponType.Frost:
        return Style(bg='#4363d8', fg='#eeeeee')
    elif weapon_type == weapon.WeaponType.Hammer:
        return Style(bg='#9A6324', fg='#2FEAED')
    elif weapon_type == weapon.WeaponType.Holy:
        return Style(bg='#42d4f4', fg='#bd2b0b')
    elif weapon_type == weapon.WeaponType.Mace:
        return Style(bg='#808000', fg='#2FEAED')
    elif weapon_type == weapon.WeaponType.Nature:
        return Style(bg='#3cb44b', fg='#EDC647')
    elif weapon_type == weapon.WeaponType.Quarterstaff:
        return Style(bg='#f58231', fg='#0a7dce')
    elif weapon_type == weapon.WeaponType.Spear:
        return Style(bg='#a9a9a9', fg='#18ED54')
    elif weapon_type == weapon.WeaponType.Sword:
        return Style(bg='#b59d00', fg='#4a62ff')
    else:
        return Style(bg='#42413c', fg='#bdbec3')


class ListItemView(QListView):
    class Item(QtGui.QStandardItem):
        def __init__(self, player, parent):
            QtGui.QStandardItem .__init__(self)
            self.parent = parent
            self.player = player
            self.update(player)
            self.setFlags(self.flags() & ~Qt.ItemIsSelectable)

            self.setTextAlignment(
                QtCore.Qt.AlignmentFlag.AlignHCenter | QtCore.Qt.AlignmentFlag.AlignVCenter)

        def update(self, player):
            self.player = player
            self.setText("{} {} ({}, {}%)".format(
                self.player.name, self.player.value, self.player.value_per_second, self.player.percentage
            ))
            self.refresh()

        def refresh(self):
            value = round(self.player.value / self.player.best_value, 2)
            QRectF = QtCore.QRectF(self.parent.rect())
            gradient = QtGui.QLinearGradient(
                QRectF.topLeft(), QRectF.topRight())
            gradient.setColorAt(value-0.001 if value > 0 else 0,
                                QtGui.QColor(player_style(self.player.items).bg))
            gradient.setColorAt(value, QtGui.QColor('#000000'))
            gradient.setColorAt(value+0.001 if value <
                                1 else 1, QtGui.QColor('#000000'))

            brush = QtGui.QBrush(gradient)

            self.setBackground(brush)

            brush_fg = QtGui.QBrush(QtGui.QColor(
                player_style(self.player.items).fg))
            self.setForeground(brush_fg)

            icon = player_icon(self.player.items['weapon'])
            self.setToolTip('\n'.join(
                [f'{k}: {map_name(v)}' for (k, v) in self.player.items.items() if v]))
            if icon:
                self.setIcon(icon)

    class SortProxyModel(QtCore.QSortFilterProxyModel):
        def lessThan(self, left, right):
            l = self.sourceModel().item(left.row())
            r = self.sourceModel().item(right.row())
            return l.player.percentage < r.player.percentage

    class ItemModel(QStandardItemModel):
        def __init__(self, view):
            QStandardItemModel .__init__(self, view)

    def __init__(self):
        QListView .__init__(self)
        self.setIconSize(QtCore.QSize(20, 20))
        self.model = self.ItemModel(self)
        self.proxy = self.SortProxyModel()
        self.proxy.setSourceModel(self.model)

        self.setModel(self.proxy)

    def update(self, players: List[PlayerListItem]):
        if not players:
            self.model.clear()
            return

        visible_names = []
        for player in players:
            items = (self.model.item(i) for i in range(self.model.rowCount()))
            found = next(
                (i for i in items if i.player.name == player.name), None)
            if found:
                found.update(player)
            else:
                self.model.appendRow(self.Item(player, self))
            visible_names.append(player.name)
        for i in range(self.model.rowCount()):
            item = self.model.item(i)
            if hasattr(item, 'player') and hasattr(item.player, 'name') and item.player.name not in visible_names:
                self.model.removeRow(i)

        self.proxy.sort(0, QtCore.Qt.DescendingOrder)

    def get_player_list_items(self) -> List[PlayerListItem]:
        return sorted(
            [self.model.item(i).player for i in range(self.model.rowCount())],
            key=lambda i: i.value,
            reverse=True)
