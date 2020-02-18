import json

from . import assets

_mappings = None

def map_name(name):
    global _mappings

    if not _mappings:
        with open(assets.path('localization.json')) as m:
            _mappings = json.loads(m.read())

    return _mappings[name] if name in _mappings else name

