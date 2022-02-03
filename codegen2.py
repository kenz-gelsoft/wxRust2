from doxybindgen import Class

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

        # TODO: dont return reference/temp value from self
        'GetChildren',
        'GetCursor', 
        'GetUpdateRegion',
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
    xmlfiles = [
        'wxml/classwx_object.xml',
        'wxml/classwx_evt_handler.xml',
        'wxml/classwx_window.xml',
        'wxml/classwx_control.xml',
        'wxml/classwx_any_button.xml',
        'wxml/classwx_button.xml',
    ]
    classes = []
    for file in xmlfiles:
        for cls in Class.in_xml(file, BLOCKLIST):
            classes.append(cls)
    
    to_be_generated = {
        'src/generated.rs': generated_rs,
        'include/wxrust2.h': wxrust2_h,
        'src/wxrust2.cc': wxrust2_cc,
    }
    for path, generator in to_be_generated.items():
        with open(path, 'w') as f:
            for chunk in generator(classes):
                print(chunk, file=f)

def generated_rs(classes):
    yield '''\
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

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
    indent = ' ' * 4 * 2
    for t in types:
        yield '%stype %s;' % (indent,t)
    for cls in classes:
        for chunk in cls.ffi_methods():
            yield chunk
    yield '''\
    }
    unsafe extern "C++" {'''
    for cls in classes:
        for chunk in cls.ffi_ctors():
            yield chunk
    yield '''\
    }
}

pub trait ObjectMethods {
    unsafe fn as_ptr(&self) -> UnsafeAnyPtr;
    fn pinned<T>(&self) -> Pin<&mut T> {
        unsafe { Pin::new_unchecked(&mut *(self.as_ptr() as *mut _)) }
    }
}
'''
    for cls in classes:
        for chunk in cls.safer_binding():
            yield chunk

def wxrust2_h(classes):
    yield '''\
#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"
#include "wx/src/generated.rs.h"


namespace wxrust {
'''
    for cls in classes:
        for chunk in cls.ctors_for_h():
            yield chunk
    yield '''\
} // namespace wxrust
'''

def wxrust2_cc(classes):
    yield '''\
#include "wx/include/wxrust.h"
#include "wx/include/wxrust2.h"

namespace wxrust {

// Constructors
'''
    for cls in classes:
        for chunk in cls.ctors_for_cc():
            yield chunk
    yield '''\
} // namespace wxrust
'''

if __name__ == '__main__':
    main()
