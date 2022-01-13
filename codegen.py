import os
import xml.etree.ElementTree as ET

# placde wxWidgets doxygen xml files in wxml/ dir and run this.
def main():
    for file in xml_files_in('wxml/'):
        tree = ET.parse(file)
        root = tree.getroot()
        for define in defines_in(root):
            parse_define(define)

def xml_files_in(dir):
    for path, _, files in os.walk(dir):
        for file in files:
            if not file.endswith('.xml'):
                continue
            yield os.path.join(path, file)

def defines_in(root):
    memberdefs = root.findall(".//memberdef[@kind='define']")
    for memberdef in memberdefs:
        yield memberdef

typedefs = [
    'wxPGVFBFlags',
]
blocklist = [
    # complex defs
    'wxNullProperty',
    'wxPGChoicesEmptyData',
    'wxDISABLE_DEBUG_SUPPORT',
    'wxTreeListEventHandler',
]
def parse_define(e):
    name = e.findtext('name')
    if name in blocklist or name in typedefs:
        return
    init = e.find('initializer')
    if init is not None:
        init = ''.join(init.itertext())
    else:
        init = '(none)'
    print('const %s: i32 = %s;' % (name, init))

if __name__ == '__main__':
    main()
