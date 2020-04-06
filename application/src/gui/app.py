import os
import sys

from PyQt5.QtCore import Qt  # type: ignore
from PyQt5 import QtGui  # type: ignore
from PyQt5.QtWidgets import QApplication  # type: ignore
from PyQt5.QtWidgets import QMessageBox  # type: ignore

from ..engine import InitializationResult
from ..engine import initialize
from ..utils.config import config

from .modules.main import MainWidget
from .styling import style
from ..utils.version import current_version as get_current_version
from ..utils.version import latest_url
from ..utils.version import latest_version as get_latest_version
from ..utils.assets import path

sys.path.append(os.path.dirname(os.path.abspath('__file__')))


def run():
    initialization_result = initialize()

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

    widget.setWindowTitle('Albion Online Stats')
    widget.setWindowIcon(QtGui.QIcon(path('albion-stats-icon.png')))

    if window_config['always_on_top']:
        widget.setWindowFlag(Qt.WindowStaysOnTopHint)
    if window_config['frameless']:
        widget.setWindowFlag(Qt.FramelessWindowHint)

    widget.show()

    current_version, latest_version = (
        get_current_version(), get_latest_version())

    if latest_version and current_version != latest_version:
        msg = QMessageBox()
        msg.setIcon(QMessageBox.Warning)
        msg.setWindowTitle("Update available!")
        msg.setText("Another version of app is avaliable.")
        msg.setInformativeText("You are using app in version {}, new version {} available <a href='{}'>here</a>".format(
            current_version, latest_version, latest_url))
        msg.setStandardButtons(QMessageBox.Ok)
        msg.show()

    if initialization_result == InitializationResult.NetworkInterfaceListMissing:
        msg = QMessageBox()
        msg.setIcon(QMessageBox.Critical)
        msg.setWindowTitle("Unable to track network traffic data!")
        msg.setText(
            "On windows make sure that WinPcap is installed in your system.")
        msg.setInformativeText("WinPcap can be installed from <a href='{}'>here</a> <br> <b>Make sure to install with the \"Install Npcap in WinPcap API-compatible Mode\"<b> option"
                               .format('https://nmap.org/npcap/dist/npcap-0.9986.exe'))
        msg.setStandardButtons(QMessageBox.Ok)
        msg.show()

    sys.exit(app.exec_())
