try:
    from pyaoaddons import item_category_mapping
except:
    item_category_mapping = {}


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


def get_weapon_type(items):
    weapon = item_category_mapping.get(items.get('weapon', None), None)

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
