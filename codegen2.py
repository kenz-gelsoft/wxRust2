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
    to_be_generated = {
        'src/generated/ffi_%s.rs': generated_ffi_rs,
        'src/generated/methods_%s.rs': generated_methods_rs,
        'src/generated/class_%s.rs': generated_rs,
        'include/generated/ffi_%s.h': generated_h,
        'src/generated/ffi_%s.cpp': generated_cpp,
    }
    rust_bindings = [RustClassBinding(cls) for cls in classes.in_lib(libname, generated)]
    classes_in_lib = classes.in_lib(libname, generated)
    cxx_bindings = [CxxClassBinding(cls, config) for cls in classes_in_lib]
    for initial in string.ascii_lowercase:
        rust_bindings_i = [b for b in rust_bindings if b.has_initial(initial)]
        if len(rust_bindings_i) == 0:
            continue
        cxx_bindings_i = [c for c in cxx_bindings if c.has_initial(initial)]
        for path, generator in to_be_generated.items():
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

def generated_ffi_rs(classes, libname):
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

def generated_methods_rs(classes, libname):
    yield '''\
use super::*;
'''
    for cls in classes:
        for line in cls.lines(for_methods=True):
            yield line

def generated_rs(classes, libname):
    yield '''\
use super::*;
'''
    for cls in classes:
        for line in cls.lines():
            yield line


def generated_h(classes, libname):
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

def generated_cpp(classes, libname):
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

if __name__ == '__main__':
    main()
