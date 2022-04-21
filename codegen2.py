from doxybindgen.model import Class, ClassManager
from doxybindgen.binding import CxxClassBinding, RustClassBinding

import subprocess
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
    generate_library(classes, config, 'core')


generated = []
def generate_library(classes, config, libname):
    generated.append(libname)
    to_be_generated = {
        'src/generated.rs': generated_rs,
        'include/generated.h': generated_h,
        'src/generated.cpp': generated_cpp,
    }
    for path, generator in to_be_generated.items():
        if libname:
            path = 'wx-%s/%s' % (libname, path)
        with open(path, 'w') as f:
            for chunk in generator(classes, config, libname):
                print(chunk, file=f)
        if path.endswith('.rs'):
            print(subprocess.check_output(['rustfmt', path]))


def generated_rs(classes, config, libname):
    yield '''\
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

use std::mem;
use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};
use std::ptr;

use methods::*;
'''
    if libname == 'base':
        yield 'use crate::wx_class;'
    else:
        yield '''\
use wx_base::methods::*;
use wx_base::*;\
'''
    yield '''\

mod ffi {
    use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};

    pub use crate::ffi::*;

    extern "C" {'''
    bindings = [RustClassBinding(cls) for cls in classes.in_lib(libname, generated)]
    indent = ' ' * 4 * 2
    for cls in bindings:
        for line in cls.lines(for_ffi=True):
            if not line:
                yield ''
            else:
                yield '%s%s' % (indent, line)
    yield '''\
    }
}

pub mod methods {
    use std::os::raw::{c_int, c_long, c_void};

    use super::*;
'''
    if libname == 'base':
        yield '''\
    pub trait WxRustMethods {
        unsafe fn as_ptr(&self) -> *mut c_void;
        unsafe fn with_ptr<F: Fn(&Self)>(ptr: *mut c_void, closure: F);
    }\
'''
    else:
        yield '    pub use wx_base::methods::*;'
    for cls in bindings:
        for line in cls.lines(for_methods=True):
            if line:
                yield '    %s' % (line,)
            else:
                yield ''
    yield '''\
}\
'''
    for cls in bindings:
        for line in cls.lines():
            yield line


def generated_h(classes, config, libname):
    yield '''\
#pragma once
#include <wx/wx.h>

extern "C" {
'''
    for cls in classes.in_lib(libname, generated):
        binding = CxxClassBinding(cls, config)
        for line in binding.lines():
            yield line
    yield '''\
} // extern "C"
'''

def generated_cpp(classes, config, libname):
    yield '''\
#include "generated.h"

extern "C" {
'''
    for cls in classes.in_lib(libname, generated):
        binding = CxxClassBinding(cls, config)
        for line in binding.lines(is_cc=True):
            yield line
    yield '''\
} // extern "C"
'''

if __name__ == '__main__':
    main()
