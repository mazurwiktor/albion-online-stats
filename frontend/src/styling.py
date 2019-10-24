from .config import config

cfg = config()
font_size = cfg['window']['font-size']

style = """
    QWidget {
        font-size: %s;
        background-color: black;
        color: white;
    }

    QListView { 
        min-height: 100px;
    }

    QListView::item:selected:active {
        background-color: black;
    }
""" % (font_size)
