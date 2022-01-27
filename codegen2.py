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

if __name__ == '__main__':
    main()
