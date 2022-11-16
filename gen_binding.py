from doxybindgen.model import Class, ClassManager
from doxybindgen.binding import CxxClassBinding, RustClassBinding

from itertools import chain

import string
import subprocess
import toml


# place wxWidgets doxygen xml files in wxml/ dir and run this.
def main():
    with open('Doxybindgen.toml', 'r') as f:
        config = toml.load(f)
    
    classes = ClassManager()
    parsed = []
    xmlfiles = config['wxml_files']
    progress('Parsing')
    for file in xmlfiles:
        progress('.')
        for cls in Class.in_xml(classes, file, config['types']):
            parsed.append(cls)
    # Register all classes once parsing finished.
    classes.register(parsed)

    progress('\nGenerating')
    with open('docs/OverloadTree.md', 'w') as overload_tree_md:
        print('''\
# Overload Method Name Decision Tree
''', file=overload_tree_md)
        generate_library(classes, config, 'base', overload_tree_md)
        generate_library(classes, config, 'core', overload_tree_md)
    
    generate_events(classes, config)


generated = []
def generate_library(classes, config, libname, overload_tree_md):
    generated.append(libname)
    files_per_initial = {
        'src/generated/ffi_%s.rs': ffi_i_rs,
        'src/generated/methods_%s.rs': methods_i_rs,
        'src/generated/class_%s.rs': class_i_rs,
        'include/generated/ffi_%s.h': ffi_i_h,
        'src/generated/ffi_%s.cpp': ffi_i_cpp,
    }
    rust_bindings = [RustClassBinding(cls, overload_tree_md) for cls in classes.in_lib(libname, generated)]
    cxx_bindings = [CxxClassBinding(cls, config) for cls in classes.in_lib(libname, generated)]
    initials = []
    for initial in string.ascii_lowercase:
        rust_bindings_i = [b for b in rust_bindings if b.has_initial(initial)]
        if len(rust_bindings_i) == 0:
            continue
        initials.append(initial)
        cxx_bindings_i = [c for c in cxx_bindings if c.has_initial(initial)]
        for path, generator in files_per_initial.items():
            progress('.')
            path = path % (initial,)
            is_rust = path.endswith('.rs')
            if libname:
                path = 'wx-%s/%s' % (libname, path)
            with open(path, 'w', newline='\n', encoding='utf-8') as f:
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
        'src/generated/class.rs': class_rs,
        'src/generated/ffi.rs': ffi_rs,
        'src/generated/methods.rs': methods_rs,
        'src/generated.rs': generated_rs,
        'include/generated.h': generated_h,
        'src/generated.cpp': generated_cpp,
    }
    for path, generator in to_be_generated.items():
        progress('.')
        is_rust = path.endswith('.rs')
        if libname:
            path = 'wx-%s/%s' % (libname, path)
        with open(path, 'w', newline='\n', encoding='utf-8') as f:
            for chunk in generator(
                initials,
                libname
            ):
                print(chunk, file=f)

def progress(s):
    print(s, end='', flush=True)

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
use super::*;
'''
    for cls in classes:
        for line in cls.lines():
            yield line


def ffi_i_h(classes, libname):
    yield '''\
#pragma once
'''
    uniq = set()
    for cls in classes:
        uniq.add(cls.include())
    for (include, condition) in sorted(uniq):
        if condition:
            yield condition
        yield include
        if condition:
            yield '#endif'
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

def class_rs(initials, libname):
    yield '''\
use super::*;
'''
    for i in initials:
        yield 'pub use class_%s::*;' % (i,)

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
#[doc(no_inline)]
pub use wx_base::methods::*;
'''
    for i in initials:
        yield 'pub use super::methods_%s::*;' % (i,)

def generated_rs(initials, libname):
    yield '''\
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};

use super::*;
use methods::*;
'''
    if libname == 'base':
        yield '''\
pub use events::*;
mod events;
'''
    yield 'mod ffi;'
    for i in initials:
        yield 'mod ffi_%s;' % (i,)
    yield ''
    yield 'pub mod methods;'
    for i in initials:
        yield 'mod methods_%s;' % (i,)
    yield ''
    yield 'pub mod class;'
    for i in initials:
        yield 'mod class_%s;' % (i,)


def generated_h(initials, libname):
    yield '''\
#pragma once

#include <wx/wx.h>
'''
    if libname == 'core':
        yield '''\
// wxBitmapBundle compatibility hack(for a while)
#if !wxCHECK_VERSION(3, 1, 6)
typedef wxBitmap wxBitmapBundle;
#endif

typedef wxMessageDialog::ButtonLabel ButtonLabel;
'''
    else:
        yield '''\
typedef wxDateTime::TimeZone TimeZone;
typedef wxDateTime::Tm       Tm;
typedef wxDateTime::WeekDay  WeekDay;
'''
    for i in initials:
        yield '#include "generated/ffi_%s.h"' % (i,)

def generated_cpp(initials, libname):
    yield '''\
#include "generated.h"

// Including splitted source files into single source file to keep build.rs simple\
'''
    for i in initials:
        yield '#include "generated/ffi_%s.cpp"' % (i,)

def generate_events(classes, config):
    to_be_generated = {
        'wx-base/src/generated/events.rs': events_rs,
        'wx-base/src/generated/events.cpp': events_cpp,
    }
    event_classes = [c for c in classes.all() if classes.is_a(c, 'wxEvent')]
    for path, generator in to_be_generated.items():
        is_rust = path.endswith('.rs')
        with open(path, 'w', newline='\n', encoding='utf-8') as f:
            for chunk in generator(event_classes, config):
                print(chunk, file=f)
        if is_rust:
            error = subprocess.check_output(['rustfmt', path])
            if error:
                print(error)

def events_rs(event_classes, config):
    def snake_to_pascal(s):
        def capitalized(word):
            return word[0].upper() + word[1:].lower()
        words = s.split('_')
        return ''.join(capitalized(word) for word in words)
    yield '''\
pub enum RustEvent {\
'''
    event_types = sorted(chain.from_iterable(c.event_types for c in event_classes))
    for event_type in event_types:
        yield '    %s,' % (snake_to_pascal(event_type),)
    yield '''\
}
'''

def events_cpp(event_classes, config):
    yield '''\
#include <wx/bookctrl.h>

#include "manual.h"


enum WxRustEvent {\
'''
    event_types = sorted(chain.from_iterable(c.event_types for c in event_classes))
    for event_type in event_types:
        yield '    RUST_EVT_%s,' % (event_type,)
    yield '''\
};

#define MAP_RUST_EVT(name) case RUST_EVT_##name: return wxEVT_##name;
#define DEFINE_TYPE_TAG_OF_EVT(name, clazz) \\
    template<> wxEventTypeTag<clazz> TypeTagOf(int eventType) { \\
        switch (eventType) { \\
        MAP_RUST_EVT(name) \\
        } \\
        return wxEVT_NULL; \\
    }

template<typename T> wxEventTypeTag<T> TypeTagOf(int eventType) {
    return wxEVT_NULL;
}'''
    for cls in event_classes:
        if len(cls.event_types) < 1:
            continue
        if len(cls.event_types) == 1:
            yield 'DEFINE_TYPE_TAG_OF_EVT(%s, %s)' % (
                cls.event_types[0],
                cls.name,
            )
            continue
        yield '''\
template<> wxEventTypeTag<%s> TypeTagOf(int eventType) {
    switch (eventType) {\
''' % (cls.name,)
        for event_type in cls.event_types:
            yield '    MAP_RUST_EVT(%s)' % (event_type,)
        yield '''\
    }
    return wxEVT_NULL;
}'''
    yield '''
template<typename T> void BindIfEventIs(wxEvtHandler *self, int eventType, void *aFn, void *aParam) {
    wxEventTypeTag<T> typeTag = TypeTagOf<T>(eventType);
    if (typeTag != wxEVT_NULL) {
        CxxClosure<T &> functor(aFn, aParam);
        self->Bind(typeTag, functor);
    }
}
void wxEvtHandler_Bind(wxEvtHandler *self, int eventType, void *aFn, void *aParam) {\
'''
    for cls in event_classes:
        if len(cls.event_types) < 1:
            continue
        yield '    BindIfEventIs<%s>(self, eventType, aFn, aParam);' % (cls.name,)
    yield '''\
}'''

if __name__ == '__main__':
    main()
