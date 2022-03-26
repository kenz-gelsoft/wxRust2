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
    type_manager.known_bindings = known_bindings
    
    to_be_generated = {
        'src/generated.rs': generated_rs,
        'include/wxrust2.h': wxrust2_h,
        'src/wxrust2.cc': wxrust2_cc,
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

use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};
use std::ptr;

use crate::macros::wx_class;

mod ffi {
    use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};
    extern "C" {
'''
    bindings = [RustClassBinding(cls) for cls in classes]
    indent = ' ' * 4 * 2
    for cls in bindings:
        for line in cls.lines(classes=classes, for_ffi=True):
            yield '%s%s' % (indent, line)
    yield '''\
    }
}

pub trait WxRustMethods {
    unsafe fn as_ptr(&self) -> *mut c_void;
}
'''
    for cls in bindings:
        for line in cls.lines(classes=classes):
            yield line


def wxrust2_h(classes, config):
    yield '''\
#pragma once
#include <wx/wx.h>

extern "C" {
'''
    for cls in classes:
        binding = CxxClassBinding(cls)
        for line in binding.lines(classes=classes):
            yield line
    yield '''\
} // extern "C"
'''

def wxrust2_cc(classes, config):
    yield '''\
#include <wx/wx.h>

extern "C" {
'''
    for cls in classes:
        binding = CxxClassBinding(cls)
        for line in binding.lines(classes=classes, is_cc=True):
            yield line
    yield '''\
} // extern "C"
'''

if __name__ == '__main__':
    main()
