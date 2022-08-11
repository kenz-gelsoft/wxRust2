from doxybindgen.model import Class, ClassManager
from doxybindgen.binding import CxxClassBinding, RustClassBinding

import string
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
    files_per_initial = {
        'src/generated/ffi_%s.rs': ffi_i_rs,
        'src/generated/methods_%s.rs': methods_i_rs,
        'src/generated/class_%s.rs': class_i_rs,
        'include/generated/ffi_%s.h': ffi_i_h,
        'src/generated/ffi_%s.cpp': ffi_i_cpp,
    }
    rust_bindings = [RustClassBinding(cls) for cls in classes.in_lib(libname, generated)]
    cxx_bindings = [CxxClassBinding(cls, config) for cls in classes.in_lib(libname, generated)]
    initials = []
    for initial in string.ascii_lowercase:
        rust_bindings_i = [b for b in rust_bindings if b.has_initial(initial)]
        if len(rust_bindings_i) == 0:
            continue
        initials.append(initial)
        cxx_bindings_i = [c for c in cxx_bindings if c.has_initial(initial)]
        for path, generator in files_per_initial.items():
            path = path % (initial,)
            is_rust = path.endswith('.rs')
            if libname:
                path = 'wx-%s/%s' % (libname, path)
            with open(path, 'w', newline='\n') as f:
                for chunk in generator(
                    rust_bindings_i if is_rust else cxx_bindings_i,
                    libname
                ):
                    print(chunk, file=f)
            if is_rust:
                error = subprocess.check_output(['rustfmt', path])
                if error:
                    print(error)
    to_be_generated = {
        'src/generated/ffi.rs': ffi_rs,
        'src/generated/methods.rs': methods_rs,
        'src/generated.rs': generated_rs,
        'include/generated.h': generated_h,
        'src/generated.cpp': generated_cpp,
    }
    for path, generator in to_be_generated.items():
        is_rust = path.endswith('.rs')
        if libname:
            path = 'wx-%s/%s' % (libname, path)
        with open(path, 'w', newline='\n') as f:
            for chunk in generator(
                initials,
                libname
            ):
                print(chunk, file=f)

def ffi_i_rs(classes, libname):
    yield '''\
use super::*;

extern "C" {'''
    indent = ' ' * 4 * 1
    for cls in classes:
        for line in cls.lines(for_ffi=True):
            if not line:
                yield ''
            else:
                yield '%s%s' % (indent, line)
    yield '''\

}\
'''

def methods_i_rs(classes, libname):
    yield '''\
use super::*;
'''
    for cls in classes:
        for line in cls.lines(for_methods=True):
            yield line

def class_i_rs(classes, libname):
    yield '''\
#![allow(non_upper_case_globals)]

use super::*;
'''
    for cls in classes:
        for line in cls.lines():
            yield line


def ffi_i_h(classes, libname):
    yield '''\
#pragma once
#include <wx/wx.h>\
'''
    uniq = set()
    for cls in classes:
        for include in cls.includes():
            uniq.add(include)
    for include in sorted(uniq):
        yield include
    if libname == 'core':
        yield '''\

// wxBitmapBundle compatibility hack(for a while)
#if !wxCHECK_VERSION(3, 1, 6)
typedef wxBitmap wxBitmapBundle;
#endif\
'''
    else:
        yield '''\

typedef wxDateTime::TimeZone TimeZone;
typedef wxDateTime::Tm       Tm;
typedef wxDateTime::WeekDay  WeekDay;\
'''
    yield '''\

extern "C" {
'''
    for cls in classes:
        for line in cls.lines():
            yield line
    yield '''\
} // extern "C"
'''

def ffi_i_cpp(classes, libname):
    yield '''\
#include "generated.h"

extern "C" {
'''
    for cls in classes:
        for line in cls.lines(is_cc=True):
            yield line
    yield '''\
} // extern "C"
'''

def ffi_rs(initials, libname):
    yield '''\
pub use crate::ffi::*;
'''
    for i in initials:
        yield 'pub use super::ffi_%s::*;' % (i,)

def methods_rs(initials, libname):
    if libname == 'base':
        yield '''\
use std::os::raw::c_void;

pub trait WxRustMethods {
    type Unowned;
    unsafe fn as_ptr(&self) -> *mut c_void;
    unsafe fn from_ptr(ptr: *mut c_void) -> Self;
    unsafe fn from_unowned_ptr(ptr: *mut c_void) -> Self::Unowned;
    unsafe fn with_ptr<F: Fn(&Self)>(ptr: *mut c_void, closure: F);
    unsafe fn option_from(ptr: *mut c_void) -> Option<Self::Unowned>
    where
        Self: Sized,
    {
        if ptr.is_null() {
            None
        } else {
            Some(Self::from_unowned_ptr(ptr))
        }
    }
}
'''
    else:
        yield '''\
pub use wx_base::methods::*;
'''
    for i in initials:
        yield 'pub use super::methods_%s::*;' % (i,)

def generated_rs(initials, libname):
    yield '''\
use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};

use super::*;
use methods::*;
'''
    yield 'mod ffi;'
    for i in initials:
        yield 'mod ffi_%s;' % (i,)
    yield ''
    yield 'pub mod methods;'
    for i in initials:
        yield 'mod methods_%s;' % (i,)
    yield ''
    for i in initials:
        yield 'mod class_%s;' % (i,)
    for i in initials:
        yield 'pub use class_%s::*;' % (i,)


def generated_h(initials, libname):
    yield '''\
#pragma once

'''
    for i in initials:
        yield '#include "generated/ffi_%s.h"' % (i,)

def generated_cpp(initials, libname):
    yield '''\
#include "generated.h"

// Including splitted source files into single source file to keep build.rs simple
'''
    for i in initials:
        yield '#include "generated/ffi_%s.cpp"' % (i,)

if __name__ == '__main__':
    main()
