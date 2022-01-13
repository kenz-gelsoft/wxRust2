import os
import re
import xml.etree.ElementTree as ET

# placde wxWidgets doxygen xml files in wxml/ dir and run this.
generated = set()
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

    # wxRichTextRange
    'wxRICHTEXT_ALL',
    'wxRICHTEXT_NONE',
    'wxRICHTEXT_NO_SELECTION',

    # wxSize
    'wxRICHTEXT_DEFAULT_OVERALL_SIZE',
    'wxRICHTEXT_DEFAULT_IMAGE_SIZE',

    # wxColour
    'wxRICHTEXT_DEFAULT_UNFOCUSSED_BACKGROUND',
    'wxRICHTEXT_DEFAULT_FOCUSSED_BACKGROUND',
    'wxRICHTEXT_DEFAULT_UNSELECTED_BACKGROUND',
    'wxRICHTEXT_DEFAULT_TYPE_COLOUR',
    'wxRICHTEXT_DEFAULT_FOCUS_RECT_COLOUR',

    # wxDefaultSize
    'wxPG_DEFAULT_IMAGE_SIZE',
]
def parse_define(e):
    name = e.findtext('name')
    if name in generated:
        return
    generated.add(name)
    if name in blocklist or name in typedefs:
        print('//  SKIP: %s' % (name,))
        return
    initializer = e.find('initializer')
    if initializer is not None:
        v = ''.join(initializer.itertext())
        t = 'i32'
        if v == 'true' or v == 'false':
            t = 'bool'
        elif '.' in v:
            t = 'f32'
        elif '"' in v:
            t = '&str'
        v = re.sub(r'wxString\((".+")\)', r'\1', v)
        v = re.sub(r'wxS\((".+")\)', r'\1', v)
        v = re.sub(r'wxT\((".+")\)', r'\1', v)
        print('const %s: %s = %s;' % (name, t, v))
    else:
        print('// NODEF: %s' % (name,))

if __name__ == '__main__':
    main()