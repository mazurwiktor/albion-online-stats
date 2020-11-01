from ..utils.config import config

cfg = config()
font_size = cfg['window']['font-size']

style = """
    QWidget {
        font-size: %s;
        background-color: black;
        color: white;
    }

    #Motd, #Motd > * {
        background-color: black;
        color: white;
        font-size: auto;
    }

    QListView { 
        min-height: 100px;
    }

    QListView::item:selected:active {
        background-color: black;
    }
    
    QListView::item:selected {
        background-color: black;
    }

    QListView::item:selected:!active {
        background-color: black;
        color: white;
    }

""" % (font_size)
