import xml.etree.ElementTree as ET
from doxybindgen import Class

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

BLOCKLIST = {
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
        for chunk in generated_rs(classes):
            print(chunk, file=f)

    with open('include/wxrust2.h', 'w') as f:
        for chunk in wxrust2_h(classes):
            print(chunk, file=f)

    with open('src/wxrust2.cc', 'w') as f:
        for chunk in wxrust2_cc(classes):
            print(chunk, file=f)

def generated_rs(classes):
    yield PROLOGUE
    yield CXX_PROLOGUE
    indent = ' ' * 4 * 2
    for t in types:
        yield '%stype %s;' % (indent,t)
    for cls in classes:
        for chunk in cls.ffi_methods(BLOCKLIST):
            yield chunk
    yield CXX_PROLOGUE2
    for cls in classes:
        for chunk in cls.ffi_ctors():
            yield chunk
    yield CXX_EPILOGUE

    for cls in classes:
        for chunk in cls.safer_binding():
            yield chunk

def wxrust2_h(classes):
    yield H_PROLOGUE
    for cls in classes:
        for chunk in cls.ctors_for_h():
            yield chunk
    yield H_EPILOGUE

def wxrust2_cc(classes):
    yield CC_PROLOGUE
    for cls in classes:
        for chunk in cls.ctors_for_cc():
            yield chunk
    yield CC_EPILOGUE

if __name__ == '__main__':
    main()
