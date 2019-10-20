from .config import config

cfg = config()
font_size = cfg['window']['font-size']

style = """
    QWidget {
        font-size: %s;
    }
    QListView { 
        min-height: 100px;
    }
    QPushButton {
        max-width: 60px;
    }
""" % (font_size)
