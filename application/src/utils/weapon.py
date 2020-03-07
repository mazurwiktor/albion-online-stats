import json

from . import assets


class WeaponType:
    Arcane = 'arcanestaff'
    Axe = 'axe'
    Bow = 'bow'
    Crossbow = 'crossbow'
    Curse = 'cursestaff'
    Dagger = 'dagger'
    Fire = 'firestaff'
    Frost = 'froststaff'
    Hammer = 'hammer'
    Holy = 'holystaff'
    Mace = 'mace'
    Nature = 'naturestaff'
    Quarterstaff = 'quarterstaff'
    Spear = 'spear'
    Sword = 'sword'


_mappings = None


def map_weapon(weapon):
    global _mappings

    if not _mappings:
        with open(assets.path('item_category_map.json')) as m:
            _mappings = json.loads(m.read())

    return _mappings[weapon] if weapon in _mappings else None


def get_weapon_type(items):
    weapon = map_weapon(items['weapon'])

    if weapon == WeaponType.Arcane:
        return WeaponType.Arcane
    elif weapon == WeaponType.Axe:
        return WeaponType.Axe
    elif weapon == WeaponType.Bow:
        return WeaponType.Bow
    elif weapon == WeaponType.Crossbow:
        return WeaponType.Crossbow
    elif weapon == WeaponType.Curse:
        return WeaponType.Curse
    elif weapon == WeaponType.Dagger:
        return WeaponType.Dagger
    elif weapon == WeaponType.Fire:
        return WeaponType.Fire
    elif weapon == WeaponType.Frost:
        return WeaponType.Frost
    elif weapon == WeaponType.Hammer:
        return WeaponType.Hammer
    elif weapon == WeaponType.Holy:
        return WeaponType.Holy
    elif weapon == WeaponType.Mace:
        return WeaponType.Mace
    elif weapon == WeaponType.Nature:
        return WeaponType.Nature
    elif weapon == WeaponType.Quarterstaff:
        return WeaponType.Quarterstaff
    elif weapon == WeaponType.Spear:
        return WeaponType.Spear
    elif weapon == WeaponType.Sword:
        return WeaponType.Sword
    else:
        return None
