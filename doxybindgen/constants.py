import re
import xml.etree.ElementTree as ET


# MEMO: don't replace `wx` prefix of `wx_GL_COMPAT_PROFILE`
RE_IDENT = re.compile(r'wx([^_]\w)')

def generate_constants_in(element):
    empty = True
    for define in defines_in(element):
        empty = False
        for line in generate_define(define):
            yield line

    for enum in enums_in(element):
        empty = False
        for line in generate_enum(enum):
            yield line
    
    if not empty:
        yield ''

def has_constants(compound):
    kind = compound.get('kind')
    if kind in ['class', 'struct']:
        return False
    for member in compound.findall("./member"):
        kind = member.get('kind')
        if kind in ['define', 'enum']:
            return True
    return False

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

    # Mutual reference in the inner class in wx_base:: not supported
    'Inv_Year',
    'WET',
    'WEST',
    'CET',
    'CEST',
    'EET',
    'EEST',
    'MSK',
    'MSD',
    'AST',
    'ADT',
    'EST',
    'EDT',
    'CST',
    'CDT',
    'MST',
    'MDT',
    'PST',
    'PDT',
    'HST',
    'AKST',
    'AKDT',
    'A_WST',
    'A_CST',
    'A_EST',
    'A_ESST',
    'NZST',
    'NZDT',
    'UTC',
    'Country_EEC',
    'France',
    'Germany',
    'UK',
    'Country_WesternEurope_End',
    'Russia',
    'USA',

    # broken initializer
    'wxAUI_TBART_OVERFLOW_SIZE',
    'wxFILE_EXISTS_NO_FOLLOW',
    'wxPG_PROP_BEING_DELETED',

    # special macro
    'wxDEPRECATED_ATTR',
]
generated = set()
class Define:
    def __init__(self, e):
        self.name = e.findtext('name')
        initializer = e.find('initializer')
        self.__initializer = initializer.itertext() if initializer is not None else None

    def blocked_reason(self):
        if self.__initializer is None:
            return 'NODEF'

        if self.name in generated:
            return '  DUP'
        
        if (self.name in blocklist or
            self.name in typedefs):
            return ' SKIP'
        
        return None

    def __str__(self):
        name = self.name
        v = ''.join(self.__initializer)
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
        # Don't strip `wx` prefix of string literal (c.f. IMAGE_OPTION_BMP_FORMAT)
        if '"' not in v:
            v = RE_IDENT.sub(r'\1', v)
        name = RE_IDENT.sub(r'\1', name)
        return 'pub const %s: %s = %s;' % (name, t, v)

def generate_define(e):
    d = Define(e)
    name = d.name
    blocked = d.blocked_reason()
    if blocked is not None:
        yield '// %s: %s' % (blocked, name)
        return
    generated.add(name)
    yield d

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

class Enum:
    def __init__(self, e):
        self.name = e.findtext('name')
        self.__current_initializer = '= 0'
        self.__count = 0
        self.__values = []
        for v in e.findall('enumvalue'):
            v = EnumValue(self, v)
            self._add_value(v)
    
    def _add_value(self, v):
        if v.initializer is None:
            if self.__count:
                v.initializer = '%s + %s' % (
                    self.__current_initializer,
                    self.__count,
                )
            else:
                v.initializer = self.__current_initializer
            self.__count += 1
        else:
            self.__current_initializer = v.initializer
            self.__count = 1
        self.__values.append(v)
    
    def generate(self):
        yield '//  ENUM: %s' % (self.name,)
        for v in self.__values:
            blocked = v.blocked_reason()
            if blocked is not None:
                yield '// %s: %s' % (
                    blocked,
                    v.name,
                )
                continue
            generated.add(v.name)
            yield v

class EnumValue:
    def __init__(self, enum, e):
        self.__enum = enum
        self.name = e.findtext('name')
        self.initializer = e.findtext('initializer')
    
    def blocked_reason(self):
        if self.name in generated:
            return '  DUP'
        
        if (self.name in blocklist or
            self.name in typedefs):
            return ' SKIP'
        
        return None
    
    def __str__(self):
        initializer = self.initializer.replace('~', '!') # special replacement for wxPATH_NORM_ALL
        t = 'c_int'
        if self.__enum.name in long_types:
            t = 'c_long'
        if "'" in initializer:
            t = 'char'
        self.name = RE_IDENT.sub(r'\1', self.name)
        initializer = RE_IDENT.sub(r'\1', initializer)
        return 'pub const %s: %s %s;' % (
            self.name,
            t,
            initializer,
        )

def generate_enum(e):
    enum = Enum(e)
    for line in enum.generate():
        yield line

