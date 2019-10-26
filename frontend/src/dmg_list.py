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

class DmgList(QListView):
    class DmgItem(QtGui.QStandardItem):
        def __init__(self, player, parent):
            QtGui.QStandardItem .__init__(self)
            self.parent = parent
            self.update(player)
            
            self.setTextAlignment(QtCore.Qt.AlignmentFlag.AlignHCenter)

        def update(self, player):
            self.name = player.name
            self.damage = player.damage
            self.dps = player.dps
            self.percentage = player.percentage
            self.best_damage = player.best_damage
            self.setText("{} {} ({}, {}%)".format(
                self.name, self.damage, self.dps, self.percentage
            ))
            self.refresh()

        def refresh(self):
            value =  round(self.damage / self.best_damage, 2)
            QRectF = QtCore.QRectF(self.parent.rect())
            gradient = QtGui.QLinearGradient(QRectF.topLeft(), QRectF.topRight())
            gradient.setColorAt(value-0.001 if value > 0 else 0, QtGui.QColor('#42413c'))
            gradient.setColorAt(value, QtGui.QColor('#000000'))
            gradient.setColorAt(value+0.001 if value < 1 else 1, QtGui.QColor('#000000'))

            brush = QtGui.QBrush(gradient)

            self.setBackground(brush)

    class SortProxyModel(QtCore.QSortFilterProxyModel):
        def lessThan(self, left, right):
            l = self.sourceModel().item(left.row())
            r = self.sourceModel().item(right.row())
            return l.percentage < r.percentage

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
        visible_names = []
        for player in players:
            items = (self.model.item(i) for i in range(self.model.rowCount()))
            found = next((i for i in items if i.name == player.name), None)
            if found:
                found.update(player)
            else:
                self.model.appendRow(self.DmgItem(player, self))
            visible_names.append(player.name)
        for i in range(self.model.rowCount()):
            item = self.model.item(i)
            if hasattr(item, 'name') and item.name not in visible_names:
                self.model.removeRow(i)
        
        self.proxy.sort(0, QtCore.Qt.DescendingOrder)
