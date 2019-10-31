import sys
import os

sys.path.append(os.path.dirname(os.path.abspath('__file__')))

from PySide2.QtCore import Qt
from PySide2.QtWidgets import QApplication
from PySide2.QtWidgets import QMessageBox

from .config import config
from .main import MainWidget
from .engine import initialize
from .styling import style
from .version import current_version as get_current_version
from .version import latest_version as get_latest_version
from .version import latest_url

def run():
    initialize()
    conf = config()
    window_config = conf['window']

    WIDTH = window_config['width']
    HEIGHT = window_config['height']

    app = QApplication(sys.argv)

    app.setStyleSheet(style)

    geometry = app.screens()[0].size()
    clipboard = app.clipboard()

    widget = MainWidget(clipboard)
    widget.setWindowOpacity(window_config['opacity'])
    widget.resize(WIDTH, HEIGHT)
    widget.move(0, geometry.height() - HEIGHT - 280)

    widget.setWindowTitle('Damage Meter')

    widget.setWindowFlag(Qt.WindowStaysOnTopHint)
    widget.setWindowFlag(Qt.FramelessWindowHint)
    widget.show()

    current_version, latest_version = (get_current_version(), get_latest_version())

    if latest_version and current_version != latest_version:
        msg = QMessageBox()
        msg.setIcon(QMessageBox.Warning)
        msg.setWindowTitle("Update available!")
        msg.setText("Another version of app is avaliable.")
        msg.setInformativeText("You are using app in version {}, new version {} available <a href='{}'>here</a>".format(
            current_version, latest_version, latest_url))
        msg.setStandardButtons(QMessageBox.Ok)
        msg.show()

    sys.exit(app.exec_())
