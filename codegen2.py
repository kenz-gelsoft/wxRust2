from doxybindgen.model import Class, ClassManager
from doxybindgen.binding import CxxClassBinding, RustClassBinding

import toml


# place wxWidgets doxygen xml files in wxml/ dir and run this.
def main():
    with open('doxybindgen.toml', 'r') as f:
        config = toml.load(f)
    
    classes = ClassManager()
    parsed = []
    xmlfiles = config['wxml_files']
    for file in xmlfiles:
        for cls in Class.in_xml(classes, file, config['types']):
            parsed.append(cls)
    # Register all classes once parsing finished.
    classes.register(parsed)
    
    generate_library(classes, config, 'base')
    generate_library(classes, config, None)


def generate_library(classes, config, libname):
    to_be_generated = {
        'src/generated.rs': generated_rs,
        'include/wxrust2.h': wxrust2_h,
        'src/wxrust2.cc': wxrust2_cc,
    }
    for path, generator in to_be_generated.items():
        if libname:
            path = '%s/%s' % (libname, path)
        with open(path, 'w') as f:
            for chunk in generator(classes, config, libname):
                print(chunk, file=f)


def generated_rs(classes, config, libname):
    yield '''\
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};
use std::ptr;

use crate::macros::wx_class;

mod ffi {
    use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};
    pub use crate::ffi::*;
    extern "C" {
'''
    bindings = [RustClassBinding(cls) for cls in classes.in_lib(libname)]
    indent = ' ' * 4 * 2
    for cls in bindings:
        for line in cls.lines(for_ffi=True):
            yield '%s%s' % (indent, line)
    yield '''\
    }
}

pub trait WxRustMethods {
    unsafe fn as_ptr(&self) -> *mut c_void;
}
'''
    for cls in bindings:
        for line in cls.lines():
            yield line


def wxrust2_h(classes, config, libname):
    yield '''\
#pragma once
#include <wx/wx.h>

extern "C" {
'''
    for cls in classes.in_lib(libname):
        binding = CxxClassBinding(cls)
        for line in binding.lines():
            yield line
    yield '''\
} // extern "C"
'''

def wxrust2_cc(classes, config, libname):
    yield '''\
#include <wx/wx.h>

extern "C" {
'''
    for cls in classes.in_lib(libname):
        binding = CxxClassBinding(cls)
        for line in binding.lines(is_cc=True):
            yield line
    yield '''\
} // extern "C"
'''

if __name__ == '__main__':
    main()
