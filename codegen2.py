from doxybindgen.model import Class, TypeManager
from doxybindgen.binding import CxxClassBinding, RustClassBinding

types = [
    'wxString = crate::ffi::wxString',
    'wxWindowList',
    'wxRect',
    'wxSizer',
    'wxFont',
    'wxRegion',
    'wxColour',
    'wxPalette',
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
    # 'wxTaskBarButton',
    'wxMenuBar',
    'wxToolBar',
    'wxStatusBar',
    'wxIconBundle',
    'wxIcon',
    # 'GeometrySerializer',
    'wxGraphicsPath',
    'wxRealPoint',
]

BLOCKLIST = {
    'wxObject': [
        'operator delete',
    ],
    'wxEvtHandler': [
        'Bind1',
        'CallAfter1',
        'GetClientData',
        'SetClientData',
        'Unbind1',
    ],
    'wxWindow': [
        # TODO: treat long correctly
        'Create',
        # wxWindowBase's methods
        'AddChild',
        'FindWindow',
        'FromDIP',
        'GetExtraStyle',
        'GetWindowStyle',
        'GetWindowStyleFlag',
        'IsDescendant',
        'IsExposed1',
        'IsExposed3',
        'RemoveChild',
        'Reparent',
        'SetExtraStyle',
        'SetSize1',
        'SetWindowStyle',
        'SetWindowStyleFlag',
        'ToDIP2',
        'UpdateWindowUI',

        # TODO: dont return reference/temp value from self
        'GetChildren',
        'GetChildren1',
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
    'wxTopLevelWindow': [
        # TODO: treat long correctly
        'Create',
        # wxTopLevelWindowMac's methods (on Mac)
        'ShowFullScreen',
        # Other platform specific
        'IsUsingNativeDecorations',
        'MSWGetSystemMenu',
        'UseNativeDecorations',
        'UseNativeDecorationsByDefault',
        # Didn't support
        'GetIcons',
        'SaveGeometry',
        'RestoreToGeometry',
    ],
    'wxFrame': [
        # TODO: treat long correctly
        'Create',
        'CreateToolBar',
        'OnCreateStatusBar',
        # Other platform specific
        'MSWGetTaskBarButton',
        # wxFrameBase's methods
        'CreateStatusBar',
        'OnCreateToolBar',
    ],
    'wxPoint': [
        'operator+',
        'operator+1',
        'operator+2',
        'operator-',
        'operator-1',
        'operator-2',
        'operator/',
        'operator*',
        'operator*1',
        'operator=',
        'operator==',
        'operator!=',
        'operator+=',
        'operator+=1',
        'operator-=',
        'operator-=1',
        'operator/=',
        'operator*=',
    ],
    'wxSize': [
        'operator+',
        'operator-',
        'operator/',
        'operator*',
        'operator*1',
        'operator=',
        'operator==',
        'operator!=',
        'operator+=',
        'operator-=',
        'operator/=',
        'operator*=',
        # Didn't support
        'Scale',
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
        'wxml/classwx_non_owned_window.xml',
        'wxml/classwx_top_level_window.xml',
        'wxml/classwx_frame.xml',
        'wxml/classwx_point.xml',
        'wxml/classwx_size.xml',
        'wxml/classwx_validator.xml',
    ]
    type_manager = TypeManager()
    classes = []
    for file in xmlfiles:
        for cls in Class.in_xml(type_manager, file, BLOCKLIST):
            classes.append(cls)
    # Set known binding(name)s once all classes parsed.
    type_manager.known_bindings = [cls.name for cls in classes]
    
    to_be_generated = {
        'src/generated.rs': generated_rs,
        'include/wxrust2.h': wxrust2_h,
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
use std::ptr;

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
    bindings = [RustClassBinding(cls) for cls in classes]
    indent = ' ' * 4 * 2
    for t in types:
        yield '%stype %s;' % (indent,t)
    for cls in bindings:
        for line in cls.ffi_lines():
            yield '%s%s' % (indent, line)
    yield '''\
    }
    unsafe extern "C++" {'''
    for cls in bindings:
        for line in cls.ffi_lines(for_shim=True):
            yield '%s%s' % (indent, line)
    yield '''\
    }
}

pub trait WxRustMethods {
    unsafe fn as_ptr(&self) -> UnsafeAnyPtr;
    fn pinned<T>(&self) -> Pin<&mut T> {
        unsafe { Pin::new_unchecked(&mut *(self.as_ptr() as *mut _)) }
    }
}
'''
    for cls in bindings:
        for line in cls.binding_lines(classes):
            yield line

def wxrust2_h(classes):
    yield '''\
#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"
#include "wx/src/generated.rs.h"


namespace wxrust {
'''
    for cls in classes:
        binding = CxxClassBinding(cls)
        for line in binding.shims():
            yield line
    yield '''\
} // namespace wxrust
'''

if __name__ == '__main__':
    main()
