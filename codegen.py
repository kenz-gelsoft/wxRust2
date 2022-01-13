import os
import re
import xml.etree.ElementTree as ET

# placde wxWidgets doxygen xml files in wxml/ dir and run this.
generated = set()
def main():
    print('#![allow(unused_parens)]')
    print()
    print('use crate::manual::*;')
    print()
    for file in xml_files_in('wxml/'):
        tree = ET.parse(file)
        root = tree.getroot()
        
        empty = True
        for define in defines_in(root):
            empty = False
            parse_define(define)

        for enum in enums_in(root):
            empty = False
            parse_enum(enum)
        
        if not empty:
            print()

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

def enums_in(root):
    memberdefs = root.findall(".//memberdef[@kind='enum']")
    for memberdef in memberdefs:
        yield memberdef

typedefs = [
    'wxPGVFBFlags',
]
blocklist = [
    # complex defs
    'Inv_Year',
    'wxBookCtrl',
    'wxDISABLE_DEBUG_SUPPORT',
    'wxDISABLE_ASSERTS_IN_RELEASE_BUILD',
    'wxEVENT_PROPAGATE_MAX',
    'wxInvalidDateTime',
    'wxNullProperty',
    'wxPG_COLOUR',
    'wxPG_COLOUR_BLACK',
    'wxPG_DEFAULT_IMAGE_SIZE',
    'wxPG_INVALID_VALUE',
    'wxPG_IT_CHILDREN',
    'wxPG_LABEL',
    'wxPG_NULL_BITMAP',
    'wxPG_PROP_PARENTAL_FLAGS',
    'wxPGChoicesEmptyData',
    'wxTLS_TYPE',
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

    # min max defs
    'wxINT8_MIN',
    'wxINT8_MAX',
    'wxUINT8_MAX',
    'wxINT16_MIN',
    'wxINT16_MAX',
    'wxUINT16_MAX',
    'wxINT32_MIN',
    'wxINT32_MAX',
    'wxUINT32_MAX',
    'wxINT64_MIN',
    'wxINT64_MAX',
    'wxUINT64_MAX',

    # broken initializer
    'wxAUI_TBART_OVERFLOW_SIZE',
    'wxFILE_EXISTS_NO_FOLLOW',
    'wxPG_PROP_BEING_DELETED',
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
        v = ''.join(map(lambda s: s.lstrip(), v.split('\\\n')))
        t = 'i32'
        if v == 'true' or v == 'false':
            t = 'bool'
        elif '.' in v:
            t = 'f32'
        elif '"' in v:
            t = '&str'
        elif "'" in v:
            t = 'char'
        v = re.sub(r'(\d+)[Ll]', r'\1', v)
        v = re.sub(r'wxString\((".+")\)', r'\1', v)
        v = re.sub(r'wxS\((".+")\)', r'\1', v)
        v = re.sub(r'wxT\((".+")\)', r'\1', v)
        print('const %s: %s = %s;' % (name, t, v))
    else:
        print('// NODEF: %s' % (name,))

def parse_enum(e):
    name = e.findtext('name')
    print('//  ENUM: %s' % (name,))
    current_initializer = '= 0'
    count = 0
    for v in e.findall('enumvalue'):
        vname = v.findtext('name')
        if vname in generated:
            continue
        generated.add(vname)
        if vname in blocklist:
            print('//  SKIP: %s' % (vname,))
            continue
        initializer = v.findtext('initializer')
        if initializer is None:
            if count:
                initializer = '%s + %s' % (current_initializer, count)
            else:
                initializer = current_initializer
            count += 1
        else:
            current_initializer = initializer
            count = 1
        t = 'i32'
        if "'" in initializer:
            t = 'char'
        print('const %s: %s %s;' % (vname, t, initializer))

if __name__ == '__main__':
    main()
