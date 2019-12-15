import collections
import functools
from PIL import Image, ImageQt
from io import BytesIO


from PySide2.QtCore import Qt
from PySide2.QtWidgets import QApplication
from PySide2.QtWidgets import QMessageBox
from PySide2.QtWidgets import QHBoxLayout
from PySide2.QtWidgets import QPushButton
from PySide2.QtWidgets import QVBoxLayout
from PySide2.QtWidgets import QLabel
from PySide2.QtWidgets import QComboBox
from PySide2.QtWidgets import QProgressBar
from PySide2.QtWidgets import QListView
from PySide2.QtGui import QStandardItemModel
from PySide2.QtGui import QStandardItem

from PySide2 import QtGui
from PySide2 import QtCore

from . import weapon
from . import assets
from . import async_request

Style = collections.namedtuple('Style', 'bg')

def player_style(items):
    weapon_type = weapon.get_weapon_type(items)

    if weapon_type == weapon.WeaponType.Arcane:
        return Style(bg = '#f032e6')
    elif weapon_type == weapon.WeaponType.Axe:
        return Style(bg = '#800000')
    elif weapon_type == weapon.WeaponType.Bow:
        return Style(bg = '#469990')
    elif weapon_type == weapon.WeaponType.Crossbow:
        return Style(bg = '#000075')
    elif weapon_type == weapon.WeaponType.Curse:
        return Style(bg = '#911eb4')
    elif weapon_type == weapon.WeaponType.Dagger:
        return Style(bg = '#4d5f20')
    elif weapon_type == weapon.WeaponType.Fire:
        return Style(bg = '#e6194B')
    elif weapon_type == weapon.WeaponType.Frost:
        return Style(bg = '#4363d8')
    elif weapon_type == weapon.WeaponType.Hammer:
        return Style(bg = '#9A6324')
    elif weapon_type == weapon.WeaponType.Holy:
        return Style(bg = '#42d4f4')
    elif weapon_type == weapon.WeaponType.Mace:
        return Style(bg = '#808000')
    elif weapon_type == weapon.WeaponType.Nature:
        return Style(bg = '#3cb44b')
    elif weapon_type == weapon.WeaponType.Quarterstaff:
        return Style(bg = '#f58231')
    elif weapon_type == weapon.WeaponType.Spear:
        return Style(bg = '#a9a9a9')
    elif weapon_type == weapon.WeaponType.Sword:
        return Style(bg = '#ffe119')
    else:
        return Style(bg = '#42413c')

# @functools.lru_cache(maxsize=None)
def player_icon(weapon):
    url = f'https://albiononline2d.ams3.cdn.digitaloceanspaces.com/thumbnails/orig/{weapon}'
    try:
        r = async_request.get(url)
        img = Image.open(BytesIO(r.content))

        img_qt = ImageQt.ImageQt(img)
        pix = QtGui.QPixmap.fromImage(img_qt)

        return QtGui.QIcon(pix)
    except Exception:
        return None

class DmgList(QListView):
    class DmgItem(QtGui.QStandardItem):
        def __init__(self, player, parent):
            QtGui.QStandardItem .__init__(self)
            self.parent = parent
            self.player = player
            self.player_style = player_style(player.items)
            self.update(player)
            
            self.setTextAlignment(QtCore.Qt.AlignmentFlag.AlignHCenter | QtCore.Qt.AlignmentFlag.AlignVCenter)

        def update(self, player):
            self.player = player
            self.setText("{} {} ({}, {}%)".format(
                self.player.name, self.player.damage, self.player.dps, self.player.percentage
            ))
            self.refresh()

        def refresh(self):
            value =  round(self.player.damage / self.player.best_damage, 2)
            QRectF = QtCore.QRectF(self.parent.rect())
            gradient = QtGui.QLinearGradient(QRectF.topLeft(), QRectF.topRight())
            gradient.setColorAt(value-0.001 if value > 0 else 0, QtGui.QColor(self.player_style.bg))
            gradient.setColorAt(value, QtGui.QColor('#000000'))
            gradient.setColorAt(value+0.001 if value < 1 else 1, QtGui.QColor('#000000'))

            brush = QtGui.QBrush(gradient)

            self.setBackground(brush)
            icon = player_icon(self.player.items['weapon'])
            if icon:
                self.setIcon(icon)

    class SortProxyModel(QtCore.QSortFilterProxyModel):
        def lessThan(self, left, right):
            l = self.sourceModel().item(left.row())
            r = self.sourceModel().item(right.row())
            return l.player.percentage < r.player.percentage

    class DmgItemModel(QStandardItemModel):
        def __init__(self, view):
            QStandardItemModel .__init__(self, view)

    def __init__(self):
        QListView .__init__(self)

        self.model = self.DmgItemModel(self)
        self.proxy = self.SortProxyModel()
        self.proxy.setSourceModel(self.model)

        self.setModel(self.proxy)

    def update(self, players):
        if not players:
            self.model.clear()
            return

        visible_names = []
        for player in players:
            items = (self.model.item(i) for i in range(self.model.rowCount()))
            found = next((i for i in items if i.player.name == player.name), None)
            if found:
                found.update(player)
            else:
                self.model.appendRow(self.DmgItem(player, self))
            visible_names.append(player.name)
        for i in range(self.model.rowCount()):
            item = self.model.item(i)
            if hasattr(item.player, 'name') and item.player.name not in visible_names:
                self.model.removeRow(i)
        
        self.proxy.sort(0, QtCore.Qt.DescendingOrder)
