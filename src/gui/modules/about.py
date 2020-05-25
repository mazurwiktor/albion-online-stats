from PySide2.QtWidgets import QMessageBox # type: ignore


text = """
This software is free to use and delivered to you with a lot on fun :)

If somebody ask you how to get it, just tell him/her to google 'albion online stats github'

Feel free to donate me with some in-game silver or support me on patronite.

Quick contact - catch me on discord JaWiktor#1717

See you in game, WedrowyczJakub
"""

class About(QMessageBox):
    def __init__(self):
        QMessageBox.__init__(self)
        self.setIcon(QMessageBox.Information)
        self.setWindowTitle("Albion online stats")
        self.setText(text)
        self.setInformativeText('<a href="{}"> Report Bug/Feature request </a> | <a href="{}"> Contribute </a> | <a href="{}"> Download </a> | <a href="{}"> Donate </a>'.format(
            "https://github.com/mazurwiktor/albion-online-stats/issues/new/choose",
            "https://github.com/mazurwiktor/albion-online-stats",
            "https://github.com/mazurwiktor/albion-online-stats/releases/latest",
            "https://www.patreon.com/wiktormazur",
            ))
        self.setStandardButtons(QMessageBox.Ok)


