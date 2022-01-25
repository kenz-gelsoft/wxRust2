from __future__ import with_statement
import os
import re
import xml.etree.ElementTree as ET

PROLOGUE = '''\
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
'''

CXX_PROLOGUE = '''\
use std::os::raw::c_char;
use std::pin::Pin;

use crate::macros::wx_class;

// any pointer type used on ffi boundary.
// we chose this type as it's handy in cxx.
type UnsafeAnyPtr = *const c_char;

#[cxx::bridge(namespace = "wxrust")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("wx/include/wxrust.h");
        include!("wx/include/wxrust2.h");
'''
CXX_PROLOGUE2 = '''\
    }
    unsafe extern "C++" {'''
CXX_EPILOGUE = '''\
    }
}

pub trait ObjectMethods {
    unsafe fn as_ptr(&self) -> UnsafeAnyPtr;
    fn pinned<T>(&self) -> Pin<&mut T> {
        unsafe { Pin::new_unchecked(&mut *(self.as_ptr() as *mut _)) }
    }
}
'''

H_PROLOGUE = '''\
#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"
#include "wx/src/generated.rs.h"


namespace wxrust {
'''
H_EPILOGUE = '''\
} // namespace wxrust
'''

CC_PROLOGUE = '''\
#include "wx/include/wxrust.h"
#include "wx/include/wxrust2.h"

namespace wxrust {

// Constructors
'''

CC_EPILOGUE = '''\
} // namespace wxrust
'''

cxx_to_cxx = {
    'long': 'int64_t',
}

cxx_to_rust = {
    'double': 'f64',
    'int': 'i32',
    'long': 'i64',
    'unsigned int': 'u32',
    'wxByte': 'u8',
    'wxCoord': 'i32',
    'wxWindowID': 'i32',
}

types = [
    'wxPoint',
    'wxSize',
    'wxString',
    'wxValidator',
    # 'wxWindow',
    'wxWindowList',
    'wxRect',
    'wxSizer',
    'wxFont',
    'wxRegion',
    'wxColour',
    'wxPalette',
    # 'wxEvtHandler',
    'wxKeyEvent',
    'wxEvent',
    'wxToolTip',
    'wxMenu',
    'wxAcceleratorTable',
    # 'wxAccessible',
    'wxDropTarget',
    'wxLayoutConstraints',
    'wxCaret',
    'wxCursor',
    'wxUpdateUIEvent',
    'wxIdleEvent',
    'wxBitmap',
    'wxCommandEvent',
    'wxClientData',
    'wxEventFilter',
    'wxClassInfo',
    'wxObjectRefData',
]

# place wxWidgets doxygen xml files in wxml/ dir and run this.
def main():
    files = [
        'wxml/classwx_object.xml',
        'wxml/classwx_evt_handler.xml',
        'wxml/classwx_window.xml',
        'wxml/classwx_control.xml',
        'wxml/classwx_any_button.xml',
        'wxml/classwx_button.xml',
    ]
    classes = []
    for file in files:
        for cls in parse_classes_in(file):
            classes.append(cls)
    
    generate_bindings_for(classes)

def parse_classes_in(xmlfile):
    tree = ET.parse(xmlfile)
    root = tree.getroot()
    for cls in root.findall(".//compounddef[@kind='class']"):
        yield Class(cls)

def generate_bindings_for(classes):    
    with open('src/generated.rs', 'w') as f:
        indent = ' ' * 4 * 2
        print(PROLOGUE, file=f)
        print(CXX_PROLOGUE, file=f)
        for t in types:
            print('%stype %s;' % (indent,t), file=f)
        for cls in classes:
            cls.print(2, f)
        print(CXX_PROLOGUE2, file=f)
        for cls in classes:
            cls.print_ffi2(2, f)
        print(CXX_EPILOGUE, file=f)

        for cls in classes:
            cls.print_rs(f)

    with open('include/wxrust2.h', 'w') as f:
        print(H_PROLOGUE, file=f)
        for cls in classes:
            cls.print_ctors_to_h(f)
        print(H_EPILOGUE, file=f)

    with open('src/wxrust2.cc', 'w') as f:
        print(CC_PROLOGUE, file=f)
        for cls in classes:
            cls.print_ctors_to_cc(f)
        print(CC_EPILOGUE, file=f)

class Class:
    def __init__(self, e):
        self.name = e.findtext('compoundname')
        self.methods = []
        for method in e.findall(".//memberdef[@kind='function']"):
            m = Method(self, method)
            if not m.is_public:
                continue
            self.methods.append(m)
    
    def print(self, level, f):
        indent = ' ' * 4 * level
        print(file=f)
        print('%s// CLASS: %s' % (indent, self.name),
                file=f)
        print('%stype %s;' % (indent, self.name),
                file=f)
        for method in self.methods:
            for line in method.in_rust():
                print('%s%s' % (indent, line),
                        file=f)

    def print_ffi2(self, level, f):
        indent = ' ' * 4 * level
        for ctor in self._ctors():
            for line in ctor.for_ffi():
                print('%s%s' % (indent, line),
                        file=f)

    def print_rs(self, f):
        rs_template = '''\
// %s
wx_class! { %s(%s) impl
}'''
        without_wx = self.name[2:]
        print(rs_template % (
            self.name,
            without_wx,
            self.name,
        ), file=f)
        self.print_ctors_to_rs(f)

    def print_ctors_to_rs(self, f):
        without_wx = self.name[2:]
        print('impl %s {' % (without_wx,),
                file=f)
        for ctor in self._ctors():
            print(ctor.for_rs(), file=f)
        print('}', file=f)
        print(file=f)

    def print_ctors_to_h(self, f):
        print('// CLASS: %s' % (self.name,),
                file=f)
        for ctor in self._ctors():
            print(ctor.for_h(), file=f)
        print(file=f)
    
    def print_ctors_to_cc(self, f):
        print('// CLASS: %s' % (self.name,),
                file=f)
        for ctor in self._ctors():
            print(ctor.for_cc(), file=f)

    def _ctors(self):
        for method in self.methods:
            if method.is_ctor:
                yield method

blocklist = {
    'wxObject': [
        'operator delete',
    ],
    'wxEvtHandler': [
        'AddFilter',
        'Bind',
        'CallAfter',
        'GetClientData',
        'RemoveFilter',
        'SetClientData',
        'Unbind',
    ],
    'wxWindow': [
        # TODO: treat long correctly
        'Create',
        # wxWindowBase's methods
        'AddChild',
        'FindFocus',
        'FindWindow',
        'FindWindowById',
        'FindWindowByLabel',
        'FindWindowByName',
        'FromDIP',
        'GetCapture',
        'GetExtraStyle',
        'GetWindowStyle',
        'GetWindowStyleFlag',
        'IsDescendant',
        'IsExposed',
        'NewControlId',
        'RemoveChild',
        'Reparent',
        'SetExtraStyle',
        'SetSize',
        'SetWindowStyle',
        'SetWindowStyleFlag',
        'ToDIP',
        'UnreserveControlId',
        'UpdateWindowUI',
    ],
    'wxControl': [
        # TODO: treat long correctly
        'Create',
    ],
    'wxButton': [
        # TODO: treat long correctly
        'Create',
    ],
}
class Method:
    def __init__(self, cls, e):
        self.is_static = e.get('static') == 'yes'
        self.is_public = e.get('prot') == 'public'
        self.__class = cls
        self.__returns = CxxType(''.join(e.find('type').itertext()))
        self.__name = e.findtext('name')
        self.__index = self._overload_index()
        self.is_ctor = self.__name == cls.name
        const = e.get('const') == 'yes'
        self.__self_param = Param(SelfType(cls.name, const), 'self')
        self.__params = []
        for param in e.findall('param'):
            ptype = ''.join(param.find('type').itertext())
            pname = param.findtext('declname')
            self.__params.append(Param(CxxType(ptype), pname))

    def _overload_index(self):
        return sum(m.__name == self.__name for m in self.__class.methods)

    def _is_blocked(self):
        if self.__class.name not in blocklist:
            return False
        blocked_methods = blocklist[self.__class.name]
        if blocked_methods:
            return self.__name in blocked_methods
        return False

    def _rust_params(self, with_ffi=False):
        params = self.__params.copy()
        if not (self.is_static or self.is_ctor):
            params.insert(0, self.__self_param)
        return ', '.join((p.rs_decl(with_ffi) for p in params))
    
    def _params_decl(self):
        return ', '.join((p.cxx_decl() for p in self.__params))
    
    def _call_params(self):
        return ', '.join((p.cxx_call() for p in self.__params))

    def in_rust(self):
        body = '%sfn %s(%s)%s;' % (
            self._maybe_unsafe(),
            self.__name,
            self._rust_params(),
            self._returns_or_not(),
        )
        suppressed = self._suppressed_reason()
        if suppressed:
            return ['// %s: %s' % (suppressed, body)]
        lines = [body]
        overload = self._rename()
        if overload:
            lines.insert(0, overload)
        # print(lines)
        return lines

    def _rename(self):
        if self.__index == 0:
            return ''
        return '#[rust_name = "%s"]' % (self._overload_name(),)

    def _overload_name(self):
        name = self.__name
        if self.is_ctor:
            without_wx = self.__class.name[2:]
            name = 'New%s' % (without_wx,)
        index = self.__index
        if self.__index == 0:
            index = ''
        return '%s%s' % (name, index)
    
    def _maybe_unsafe(self):
        return self._uses_ptr_type() and 'unsafe ' or ''
    
    def _uses_ptr_type(self):
        if self.__returns.is_ptr():
            return True
        return any(p.type.is_ptr() for p in self.__params)

    def _returns_or_not(self):
        returns = self.__returns.in_rust()
        if returns in ['void', '']:
            returns = ''
        else:
            returns = ' -> %s' % (returns,)
        return returns
    
    def _suppressed_reason(self):
        if self.is_ctor:
            return 'CTOR'
        if self._uses_unsupported_type():
            return 'CXX_UNSUPPORTED'
        if self._is_blocked():
            return 'BLOCKED'
        return None
    
    def _uses_unsupported_type(self):
        if self.__returns.not_supported():
            return True
        return any(p.type.not_supported() for p in self.__params)

    def for_ffi(self):
        rs_template = 'unsafe fn %s(%s) -> *mut %s;'
        lines = [rs_template % (
            self._overload_name(),
            self._rust_params(),
            self.__class.name,
        )]
        overload = self._rename()
        if overload:
            lines.insert(0, overload)
        return lines

    def for_rs(self):
        rs_template = '''\
    pub fn %s(%s) -> %s {
        unsafe { %s(ffi::%s(%s)) }
    }'''
        new_name = 'new'
        if self.__index > 0:
            new_name += str(self.__index)
        without_wx = self.__class.name[2:]
        return rs_template % (
            new_name,
            self._rust_params(with_ffi=True),
            without_wx,
            without_wx,
            self._overload_name(),
            self._call_params(),
        )

    def for_h(self):
        body = '%s *%s(%s);' % (
            self.__name,
            self._overload_name(),
            self._params_decl(),
        )
        return body
    
    def for_cc(self):
        cc_template = '''\
%s *%s(%s) {
    return new %s(%s);
}
'''
        return cc_template % (
            self.__class.name,
            self._overload_name(),
            self._params_decl(),
            self.__class.name,
            self._call_params(),
        )

class Param:
    def __init__(self, type, name):
        self.type = type
        self.name = name
    
    def rs_decl(self, with_ffi=False):
        return '%s: %s' % (
            self.name,
            self.type.in_rust(with_ffi)
        )
    
    def cxx_decl(self):
        return '%s %s' % (self.type.in_cxx(), self.name)

    def cxx_call(self):
        return self.name

class SelfType:
    def __init__(self, s, const):
        self.type = s
        self.const = const

    def in_rust(self, with_ffi=False):
        t = self.type
        t = prefixed(t, with_ffi)
        if self.const:
            return '&%s' % (t)
        return 'Pin<&mut %s>' % (t,)

os_unsupported_types = [
    'wxAccessible',
]
cxx_unsupported_types = [
]
cxx_supported_value_types = [
    'bool',
    'void',
]
class CxxType:
    def __init__(self, s):
        self.origtype = s
        # print('parsing: |%s|' % (s,))
        matched = re.match(r'(const )?([^*&]*)([*&]+)?', s)
        self.basetype = None
        if matched:
            self.mut = matched.group(1) is None
            self.basetype = matched.group(2).strip()
            self.indirection = matched.group(3)
        if self.indirection is None:
            self.indirection = ''
    
    def in_cxx(self):
        if self.origtype in cxx_to_cxx:
            return cxx_to_cxx[self.origtype]
        return self.origtype

    def in_rust(self, with_ffi=False):
        t = self.basetype
        if t in cxx_to_rust:
            t = cxx_to_rust[t]
        t = prefixed(t, with_ffi)
        ref = self.indirection
        mut = ''
        if ref:
            mut = self.mut and 'mut ' or ''
            if ref.startswith('*') and not self.mut:
                mut = 'const '
        if ref.startswith('&') and self.mut:
            return 'Pin<%smut %s>' % (ref, t)
        return '%s%s%s' % (ref, mut, t)
    
    def not_supported(self):
        if self.basetype in os_unsupported_types:
            return True
        if self.basetype in cxx_unsupported_types:
            return True
        if not self._is_cxx_supported_value_type():
            return not self.indirection
        return False
    
    def _is_cxx_supported_value_type(self):
        if self.basetype in cxx_supported_value_types:
            return True
        if self.basetype in cxx_to_rust:
            return True
        return False
    
    def is_ptr(self):
        return self.indirection.startswith('*')

rust_primitives = [
    'i32',
    'i64',
]
def prefixed(t, with_ffi):
    if t in rust_primitives:
        return t
    if with_ffi:
        t = 'ffi::%s' % (t,)
    return t


if __name__ == '__main__':
    main()
