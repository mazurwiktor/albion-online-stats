import sys

from PySide2.QtCore import Qt
from PySide2.QtWidgets import QApplication

import libmeter

from config import set_script_path
set_script_path(sys.argv[0])  # pyinstaller creates tmpdir for python files, thus this is the way to get executable path

from config import config
from main import MainWidget
from engine import initialize
from styling import style


if __name__ == "__main__":
    initialize()
    conf = config()
    window_config = conf['window']

    WIDTH = window_config['width']
    HEIGHT = window_config['height']

    app = QApplication(sys.argv)

    app.setStyleSheet(style)

    geometry = app.screens()[0].size()

    widget = MainWidget()
    widget.setWindowOpacity(window_config['opacity'])
    widget.resize(WIDTH, HEIGHT)
    widget.move(0, geometry.height() - HEIGHT - 280)

    widget.setWindowTitle('Damage Meter')

    widget.setWindowFlag(Qt.WindowStaysOnTopHint)
    widget.setWindowFlag(Qt.FramelessWindowHint)
    widget.show()

    sys.exit(app.exec_())
