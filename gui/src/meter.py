import sys

from PySide2.QtCore import Qt
from PySide2.QtWidgets import QApplication

import libmeter

from main import MainWidget
from engine import initialize
from styling import style

if __name__ == "__main__":
    initialize()

    WIDTH = 256
    HEIGHT = 200

    app = QApplication(sys.argv)

    app.setStyleSheet(style)

    geometry = app.screens()[0].size()

    widget = MainWidget()
    widget.setWindowOpacity(0.5)
    widget.resize(WIDTH, HEIGHT)
    widget.move(0, geometry.height() - HEIGHT - 280)

    widget.setWindowTitle('Damage Meter')

    widget.setWindowFlag(Qt.WindowStaysOnTopHint)
    widget.setWindowFlag(Qt.FramelessWindowHint)
    widget.show()

    sys.exit(app.exec_())
