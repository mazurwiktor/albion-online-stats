from io import BytesIO
from PIL import Image, ImageQt  # type: ignore
from PySide2 import QtGui  # type: ignore

from .....utils import async_request


icon_cache = {}


def get_weapon_icon(weapon):
    global icon_cache

    url = f'https://albiononline2d.ams3.cdn.digitaloceanspaces.com/thumbnails/orig/{weapon}'

    if weapon in icon_cache:
        return icon_cache[weapon]

    try:
        r = async_request.get(url)
        img = Image.open(BytesIO(r.content))

        img_qt = ImageQt.ImageQt(img)
        pix = QtGui.QPixmap.fromImage(img_qt)

        icon_cache[weapon] = QtGui.QIcon(pix)
        return icon_cache[weapon]
    except Exception:
        return None
