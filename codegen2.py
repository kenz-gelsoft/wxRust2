from doxybindgen.model import Class, ClassManager
from doxybindgen.binding import CxxClassBinding, RustClassBinding

import os
import subprocess
import toml

excludes = [
    'wxApp',
    'wxAppConsole',
    'wxArchiveInputStream',
    'wxArchiveOutputStream',
    'wxArray< T >',
    'wxArrayString',
    'wxBufferedInputStream',
    'wxBufferedOutputStream',
    'wxCharBuffer',
    'wxCharTypeBuffer',
    'wxConfigBase',
    'wxConvAuto',
    'wxCountingOutputStream',
    'wxCSConv',
    'wxCustomBackgroundWindow',
    'wxDataFormat',
    'wxDialUpManager',
    'wxEventLoopActivator',
    'wxEventLoopBase',
    'wxFFileInputStream',
    'wxFFileOutputStream',
    'wxFFileStream',
    'wxFileConfig',
    'wxFileInputStream',
    'wxFileOutputStream',
    'wxFileStream',
    'wxFilterInputStream',
    'wxFilterOutputStream',
    'wxFloatingPointValidator',
    'wxFSInputStream',
    'wxGrid',
    'wxGUIEventLoop',
    'wxHtmlWindow',
    'wxImageHistogram',
    'wxInputStream',
    'wxIntegerValidator',
    'wxList< T >',
    'wxLZMAInputStream',
    'wxLZMAOutputStream',
    'wxLongLong',
    'wxMBConv',
    'wxMBConvUTF16',
    'wxMBConvUTF32',
    'wxMBConvUTF7',
    'wxMBConvUTF8',
    'wxMemoryInputStream',
    'wxMemoryOutputStream',
    'wxNavigationEnabled',
    'wxNode< T >',
    'wxNumberFormatter',
    'wxObjectDataPtr< T >',
    'wxOutputStream',
    'wxPathList',
    'wxPersistentBookCtrl',
    'wxPersistentComboBox',
    'wxPersistentDataViewCtrl',
    'wxPersistentTLW',
    'wxPersistentTreeBookCtrl',
    'wxPixelData',
    'wxPreviewCanvas',
    'wxPropertyGrid',
    'wxRegConfig',
    'wxRichMessageDialog',
    'wxSafeArray< varType >',
    'wxScopedArray',
    'wxScopedPtr',
    'wxScopedPtr< T >',
    'wxScopedTiedPtr',
    'wxScrolled',
    'wxSecretString',
    'wxSharedPtr< T >',
    'wxSocketInputStream',
    'wxSocketOutputStream',
    'wxSortedArrayString',
    'wxStdInputStream',
    'wxStdInputStreamBuffer',
    'wxStdOutputStream',
    'wxStdOutputStreamBuffer',
    'wxStreamBase',
    'wxString',
    'wxStringBuffer',
    'wxStringBufferLength',
    'wxStringInputStream',
    'wxStringOutputStream',
    'wxSystemThemedControl',
    'wxTarEntry',
    'wxTarInputStream',
    'wxTarOutputStream',
    'wxTempFFileOutputStream',
    'wxTempFileOutputStream',
    'wxThread',
    'wxTimeSpan',
    'wxTextFile',
    'wxTextInputStream',
    'wxTextOutputStream',
    'wxULongLong',
    'wxUString',
    'wxUniChar',
    'wxVariant',
    'wxWCharBuffer',
    'wxWeakRef< T >',
    'wxWindowPtr< T >',
    'wxWrapperInputStream',
    'wxXLocale',
    'wxZipEntry',
    'wxZipInputStream',
    'wxZipOutputStream',
    'wxZlibInputStream',
    'wxZlibOutputStream',
]

# place wxWidgets doxygen xml files in wxml/ dir and run this.
def main():
    with open('doxybindgen.toml', 'r') as f:
        config = toml.load(f)
    
    classes = ClassManager()
    parsed = []
    includes = config['includes']
    for wxml in sorted(wxml_files()):
        for cls in Class.in_xml(classes, wxml, config['types']):
            if cls.name in excludes:
                continue
            parsed.append(cls)
    # Register all classes once parsing finished.
    classes.register(parsed)
    
    generate_library(classes, config, 'base')
    generate_library(classes, config, 'core')


def wxml_files():
    for root, dirs, files in os.walk('wxml'):
        for file in files:
            if (file.startswith('classwx_') and
                file.endswith('.xml')):
                yield os.path.join(root, file)


generated = []
def generate_library(classes, config, libname):
    generated.append(libname)
    to_be_generated = {
        'src/generated/ffi.rs': generated_ffi_rs,
        'src/generated/methods.rs': generated_methods_rs,
        'src/generated.rs': generated_rs,
        'include/generated.h': generated_h,
        'src/generated.cpp': generated_cpp,
    }
    rust_bindings = [RustClassBinding(cls) for cls in classes.in_lib(libname, generated)]
    cxx_bindings = [CxxClassBinding(cls, config) for cls in classes.in_lib(libname, generated)]
    for path, generator in to_be_generated.items():
        is_rust = path.endswith('.rs')
        if libname:
            path = 'wx-%s/%s' % (libname, path)
        with open(path, 'w', newline='\n') as f:
            for chunk in generator(
                rust_bindings if is_rust else cxx_bindings,
                libname
            ):
                print(chunk, file=f)
        if is_rust:
            error = subprocess.check_output(['rustfmt', path])
            if error:
                print(error)

def generated_ffi_rs(classes, libname):
    yield '''\
use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};

pub use crate::ffi::*;

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
use std::os::raw::{c_int, c_long, c_uint, c_void};

use super::*;
use crate::WeakRef;
'''
    if libname == 'base':
        yield '''\
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
}\
'''
    else:
        yield 'pub use wx_base::methods::*;'
    for cls in classes:
        for line in cls.lines(for_methods=True):
            yield line

def generated_rs(classes, libname):
    yield '''\
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

use std::mem;
use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};
use std::ptr;

use super::*;
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

mod ffi;
pub mod methods;

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
