import os
import re
import xml.etree.ElementTree as ET

PROLOGUE = '''\
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

// FIXME: workaround for windows (LLP64)
#![allow(overflowing_literals)]

use std::os::raw::{c_int, c_long};

use crate::manual::*;
'''

# place wxWidgets doxygen xml files in wxml/ dir and run this.
generated = set()
def main():
    with open('wx-base/src/defs.rs', 'w') as f:
        print(PROLOGUE, file=f)
        for file in xml_files_in('wxml/'):
            # print(file)
            tree = ET.parse(file)
            root = tree.getroot()
            
            empty = True
            for define in defines_in(root):
                empty = False
                generate_define(define, f)

            for enum in enums_in(root):
                empty = False
                generate_enum(enum, f)
            
            if not empty:
                print(file=f)

def xml_files_in(dir):
    index = os.path.join(dir, 'index.xml')
    with open(index, 'r') as f:
        tree = ET.parse(f)
        root = tree.getroot()
        for compound in root.findall("./compound"):
            xml = compound.get('refid') + '.xml'
            yield os.path.join(dir, xml)

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
def generate_define(e, f):
    name = e.findtext('name')
    if name in generated:
        return
    generated.add(name)
    if name in blocklist or name in typedefs:
        print('//  SKIP: %s' % (name,),
                file=f)
        return
    initializer = e.find('initializer')
    if initializer is not None:
        v = ''.join(initializer.itertext())
        v = ''.join(map(lambda s: s.lstrip(), v.split('\\\n')))
        t = 'c_int'
        if name in long_types:
            t = 'c_long'
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
        print('pub const %s: %s = %s;' % (name, t, v),
                file=f)
    else:
        print('// NODEF: %s' % (name,),
                file=f)

long_types = [
    'wxAC_DEFAULT_STYLE',
    'wxAEDIALOG_STYLE',
    'wxALWAYS_SHOW_SB',
    'wxAccObject',
    'wxBACKINGSTORE',
    'wxBORDER',
    'wxBorder',
    'wxCANCEL',
    'wxCANCEL_DEFAULT',
    'wxCAPTION',
    'wxCHOICEDLG_STYLE',
    'wxCLIP_CHILDREN',
    'wxCLIP_SIBLINGS',
    'wxCLOSE_BOX',
    'wxCP_DEFAULT_STYLE',
    'wxDD_DEFAULT_STYLE',
    'wxDEFAULT_DIALOG_STYLE',
    'wxDEFAULT_FRAME_STYLE',
    'wxDOUBLE_BORDER',
    'wxFULL_REPAINT_ON_RESIZE',
    'wxGeometryCentre',
    'wxHLB_DEFAULT_STYLE',
    'wxHL_ALIGN_CENTRE',
    'wxHL_CONTEXTMENU',
    'wxHL_DEFAULT_STYLE',
    'wxHSCROLL',
    'wxLB_HSCROLL',
    'wxMAXIMIZE_BOX',
    'wxMINIMIZE_BOX',
    'wxNO_BORDER',
    'wxOK',
    'wxPGPropertyFlags',
    'wxPG_ITERATOR_FLAGS',
    'wxPG_PROP_ACTIVE_BTN',
    'wxPG_PROP_MAX',
    'wxPG_PROP_PASSWORD',
    'wxPG_PROP_SHOW_FULL_FILENAME',
    'wxPG_PROP_STATIC_CHOICES',
    'wxPG_PROP_TRANSLATE_CUSTOM',
    'wxPG_PROP_USE_CHECKBOX',
    'wxPG_PROP_USE_DCC',
    'wxPG_STRING_STORED_FLAGS',
    'wxPOPUP_WINDOW',
    'wxRAISED_BORDER',
    'wxRESIZE_BORDER',
    'wxRETAINED',
    'wxSIMPLE_BORDER',
    'wxSTATIC_BORDER',
    'wxSTB_DEFAULT_STYLE',
    'wxSTB_ELLIPSIZE_END',
    'wxSTB_SHOW_TIPS',
    'wxSTB_SIZEGRIP',
    'wxSTC_MASK_FOLDERS',
    'wxSUNKEN_BORDER',
    'wxSYSTEM_MENU',
    'wxTAB_TRAVERSAL',
    'wxTE_DONTWRAP',
    'wxTRANSPARENT_WINDOW',
    'wxTextEntryDialogStyle',
    'wxVSCROLL',
    'wxWANTS_CHARS',
    'wxWINDOW_STYLE_MASK',
]

def generate_enum(e, f):
    name = e.findtext('name')
    print('//  ENUM: %s' % (name,),
            file=f)
    current_initializer = '= 0'
    count = 0
    for v in e.findall('enumvalue'):
        vname = v.findtext('name')
        if vname in generated:
            continue
        generated.add(vname)
        if vname in blocklist:
            print('//  SKIP: %s' % (vname,),
                    file=f)
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
        initializer = initializer.replace('~', '!') # special replacement for wxPATH_NORM_ALL
        t = 'c_int'
        if name in long_types:
            t = 'c_long'
        if "'" in initializer:
            t = 'char'
        print('pub const %s: %s %s;' % (vname, t, initializer),
                file=f)

if __name__ == '__main__':
    main()
