from PySide2.QtWidgets import QMessageBox # type: ignore
import os
import sys
import toml
import functools
from .view_type import ViewType
from PySide2 import QtGui  # type: ignore
from PySide2.QtCore import Qt  # type: ignore
from PySide2.QtWidgets import QHBoxLayout  # type: ignore
from PySide2.QtWidgets import QPushButton  # type: ignore
from PySide2.QtWidgets import QCheckBox  # type: ignore
from PySide2.QtWidgets import QSpinBox  # type: ignore
from PySide2.QtWidgets import QSlider  # type: ignore
from PySide2.QtWidgets import QVBoxLayout  # type: ignore
from PySide2.QtWidgets import QWidget  # type: ignore
from PySide2.QtWidgets import QLabel  # type: ignore

width = 300
height = 220
font = 10
opaci_percent = 100
opaci = opaci_percent/100
frame = False
top = False

class FontSpin(QSpinBox):
    def __init__(self):
        QSpinBox.__init__(self)
        self.setMinimum(5)
        self.setMaximum(25)
        self.setValue(font)
        self.setSingleStep(1)


class OpaciSpin(QSpinBox):
    def __init__(self):
        QSpinBox.__init__(self)
        self.setMinimum(1)
        self.setMaximum(100)
        self.setValue(opaci_percent)
        self.setSingleStep(10)
        self.setSuffix("%")
class FrameCheck(QCheckBox):
    def __init__(self):
        QCheckBox.__init__(self)
        self.setChecked(frame)


class TopCheck(QCheckBox):
    def __init__(self):
        QCheckBox.__init__(self)
        self.setChecked(top)

class ApplyButton(QPushButton):
    def __init__(self):
        QPushButton.__init__(self)
        self.setText("Apply")


        def Apply():
            font = FontSpin.value
            opaci_percent = OpaciSpin.value
            frame = FrameCheck.checkState
            top = TopCheck.checkState
            width = TopBar.width_text
            height = TopBar.height_text
        self.clicked.connect(Apply)


class TopBar(QWidget):
    def __init__(self):
        QWidget.__init__(self)
        self.layout = QHBoxLayout()
        self.layout.setSpacing(0)
        self.layout.setMargin(0)

        width_name = QLabel()
        width_name.setText("Window width:    ")
        self.layout.addWidget(width_name)

        WidthSlider = QSlider(Qt.Horizontal)
        WidthSlider.setMinimum(300)
        WidthSlider.setMaximum(1920)
        WidthSlider.setValue(width)
        WidthSlider.setSingleStep(10)
        self.layout.addWidget(WidthSlider)

        WidthSlider.valueChanged[int].connect(self.valuechange)
        width_text = QLabel()
        width_text.setNum(width)        
        self.layout.addWidget(width_text)

        height_name = QLabel()
        height_name.setText("\n Window height:    ")
        self.layout.addWidget(height_name)

        HeightSlider = QSlider(Qt.Horizontal)
        HeightSlider.setMinimum(220)
        HeightSlider.setMaximum(1080)
        HeightSlider.setValue(height)
        HeightSlider.setSingleStep(10)
        self.layout.addWidget(HeightSlider)

        HeightSlider.valueChanged[int].connect(self.valuechange)
        height_text = QLabel()
        height_text.setNum(height)        
        self.layout.addWidget(height_text)

        self.setLayout(self.layout)

    def valuechange(self, value):
        self.__init__(value)   # FCK INIT CALLED TWICE, i was looking for solution but can't find anything ;_;

class Settings(QWidget):
    def __init__(self):
        QWidget.__init__(self)
        self.setWindowTitle("Settings")
        self.layout = QVBoxLayout()
        self.top_bar = TopBar()
        self.layout.addWidget(self.top_bar)

        font_name = QLabel()
        font_name.setText("Font size:")
        self.layout.addWidget(font_name)
        self.font_spin = FontSpin()
        self.layout.addWidget(self.font_spin) 

        opaci_name = QLabel()
        opaci_name.setText("Opacity:")
        self.layout.addWidget(opaci_name)
        self.opaci_spin = OpaciSpin()
        self.layout.addWidget(self.opaci_spin)

        frame_name = QLabel()
        frame_name.setText("Frameless:")
        self.layout.addWidget(frame_name)
        self.frame_check = FrameCheck()
        self.layout.addWidget(self.frame_check) 

        top_name = QLabel()
        top_name.setText("Show always on top:")
        self.layout.addWidget(top_name)
        self.top_check = TopCheck()
        self.layout.addWidget(self.top_check) 

        self.setLayout(self.layout)

        self.apply_button = ApplyButton()
        self.layout.addWidget(self.apply_button) 
