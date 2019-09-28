from PySide2.QtWidgets import QTableWidget
from PySide2.QtWidgets import QHeaderView
from PySide2.QtWidgets import QTableWidgetItem

class Table(QTableWidget):
    def __init__(self):
        QTableWidget.__init__(self)

        self.session = None

        self.setColumnCount(4)
        self.setHorizontalHeaderLabels(["Player", "Damage", "DPS", "%"])
        self.horizontalHeader().setSectionResizeMode(QHeaderView.Stretch)

    def fill(self, new_session):
        if not self._session_changed(new_session):
            return

        self.session = new_session

        session = sorted(new_session, key=lambda s: float(
            s.damage), reverse=True)

        self._align_rows(len(session))

        for idx, stats in enumerate(session):
            self.setItem(idx, 0, QTableWidgetItem(stats.name))
            self.setItem(idx, 1, QTableWidgetItem(stats.damage))
            self.setItem(idx, 2, QTableWidgetItem(stats.dps))
            self.setItem(idx, 3, QTableWidgetItem(stats.precentage))

    def _align_rows(self, new_row_count):
        row_count = self.rowCount()
        diff = row_count - new_row_count

        if diff < 0:
            for i in range(row_count, row_count - diff):
                self.insertRow(i)
        else:
            for i in range(0, diff):
                self.removeRow(0)

    def _session_changed(self, session):
        return self.session != session