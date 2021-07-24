import json

try:
    from pyaoaddons import localization_mapping
except:
    localization_mapping = {}


from . import assets


def map_name(name):
    return localization_mapping[name] if name in localization_mapping else name
