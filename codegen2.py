from doxybindgen.model import Class, TypeManager
from doxybindgen.binding import CxxClassBinding, RustClassBinding

import toml


# place wxWidgets doxygen xml files in wxml/ dir and run this.
def main():
    with open('doxybindgen.toml', 'r') as f:
        config = toml.load(f)
    
    type_manager = TypeManager()
    classes = []
    xmlfiles = config['wxml_files']
    for file in xmlfiles:
        for cls in Class.in_xml(type_manager, file, config['types']):
            classes.append(cls)
    # Set known binding(name)s once all classes parsed.
    known_bindings = [cls.name for cls in classes]
    known_bindings.extend(cls.internal_base() for cls in classes)
    type_manager.known_bindings = known_bindings
    
    to_be_generated = {
        'src/generated.rs': generated_rs,
        'include/wxrust2.h': wxrust2_h,
    }
    for path, generator in to_be_generated.items():
        with open(path, 'w') as f:
            for chunk in generator(classes, config):
                print(chunk, file=f)


def generated_rs(classes, config):
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
    types = config['types']['decls']
    for t in types:
        yield '%stype %s;' % (indent, t)
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


def wxrust2_h(classes, config):
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
