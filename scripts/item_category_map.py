import json
import re
import sys
import xml.etree.ElementTree as ET

file = sys.argv[1]

def add_item(db, name, item):
    if 'craftingcategory' in item:
        db[name] = item['craftingcategory']

nd = {}
tree = ET.parse(file)
root = tree.getroot()

for child in root:
    add_item(nd, child.attrib['uniquename'], child.attrib)


    ench = child.find("enchantments")
    
    if ench:
        for e in ench:
            add_item(nd, f"{child.attrib['uniquename']}@{e.attrib['enchantmentlevel']}", child.attrib)

print(json.dumps(nd))