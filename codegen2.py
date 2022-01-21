import os
import re
import xml.etree.ElementTree as ET

PROLOGUE = '''\
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
'''

CXX_PROLOGUE = '''\
#[cxx::bridge(namespace = "wxrust")]
mod ffi {
\t#[namespace = ""]
\tunsafe extern "C++" {
\t\tinclude!("wx/include/wxrust.h");
'''
CXX_EPILOGUE = '''\
\t}
}
'''

H_PROLOGUE = '''\
#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"
#include "wx/src/generated.rs.h"


namespace wxrust {
'''
H_EPILOGUE = '''\

} // namespace wxrust
'''

CC_PROLOGUE = '''\
#include "wx/include/wxrust.h"

namespace wxrust {

// Constructors
'''

CC_EPILOGUE = '''\
} // namespace wxrust
'''

cxx_to_rust = {
    'long': 'i32',
    'wxWindowID': 'i32',
}

types = [
    'wxPoint',
    'wxSize',
    'wxString',
    'wxValidator',
    'wxWindow',
]

# place wxWidgets doxygen xml files in wxml/ dir and run this.
def main():
    tree = ET.parse('wxml/classwx_button.xml')
    root = tree.getroot()
    
    classes = []
    for cls in classes_in(root):
        classes.append(Class(cls))
    
    with open('src/generated.rs', 'w') as f:
        print(PROLOGUE, file=f)
        print(CXX_PROLOGUE, file=f)
        for t in types:
            print('\t\ttype %s;' % (t,), file=f)
        for cls in classes:
            cls.print(2, f)
        print(CXX_EPILOGUE, file=f)

    with open('include/wxrust2.h', 'w') as f:
        print(H_PROLOGUE, file=f)
        for cls in classes:
            cls.print_ctors_to_h(f)
        print(H_EPILOGUE, file=f)

    with open('src/wxrust2.cc', 'w') as f:
        print(CC_PROLOGUE, file=f)
        for cls in classes:
            cls.print_ctors_to_cc(f)
        print(CC_EPILOGUE, file=f)

def classes_in(root):
    return root.findall(".//compounddef[@kind='class']")

class Class:
    def __init__(self, e):
        self.name = e.findtext('compoundname')
        self.methods = []
        for method in e.findall(".//memberdef[@kind='function']"):
            self.methods.append(Method(self.name, method))
    
    def print(self, level, f):
        indent = '\t' * level
        print('%s// CLASS: %s' % (indent, self.name),
                file=f)
        print('%stype %s;' % (indent, self.name),
                file=f)
        for method in self.methods:
            print('%s%s' % (indent, method),
                    file=f)
    
    def print_ctors_to_h(self, f):
        print('// CLASS: %s' % (self.name,),
                file=f)
        for ctor in self.ctors():
            print(ctor.for_h(), file=f)
    
    def print_ctors_to_cc(self, f):
        print('// CLASS: %s' % (self.name,),
                file=f)
        for ctor in self.ctors():
            print(ctor.for_cc(), file=f)

    def ctors(self):
        for method in self.methods:
            if method.is_ctor:
                yield method


class Method:
    def __init__(self, classname, e):
        self.classname = classname
        self.name = e.findtext('name')
        self.is_ctor = self.name == classname
        self.this = Param(SelfType(classname), 'self')
        self.params = []
        for param in e.findall('param'):
            ptype = ''.join(param.find('type').itertext())
            pname = param.findtext('declname')
            self.params.append(Param(CxxType(ptype), pname))
    
    def rust_params(self):
        clone = self.params.copy()
        clone.insert(0, self.this)
        return ', '.join((str(p) for p in clone))
    
    def params_decl(self):
        return ', '.join((p.cxx_decl() for p in self.params))
    
    def call_params(self):
        return ', '.join((p.cxx_call() for p in self.params))
    
    def __str__(self):
        body = 'fn %s(%s);' % (
            self.name,
            self.rust_params(),
        )
        if self.is_ctor:
            return '// %s' % (body,)
        return body

    def for_h(self):
        body = '%s *%s(%s);' % (
            self.name,
            self.new_name(),
            self.params_decl(),
        )
        return body
    
    def for_cc(self):
        cc_template = '''\
%s *%s(%s) {
    return new %s(%s);
}
'''
        return cc_template % (
            self.classname,
            self.new_name(),
            self.params_decl(),
            self.classname,
            self.call_params(),
        )

    def new_name(self):
        return 'New%s' % (self.name[2:],)

class Param:
    def __init__(self, type, name):
        self.type = type
        self.name = name
    
    def __str__(self):
        return '%s: %s' % (self.name, self.type.in_rust())
    
    def cxx_decl(self):
        return '%s %s' % (self.type.in_cxx, self.name)

    def cxx_call(self):
        return self.name

class SelfType:
    def __init__(self, s):
        self.type = s

    def in_rust(self):
        return 'Pin<&mut %s>' % (self.type,)

class CxxType:
    def __init__(self, s):
        self.in_cxx = s
        # print('parsing: |%s|' % (s,))
        matched = re.match(r'(const )?([^*&]*)([*&]+)?', s)
        self.basetype = None
        if matched:
            self.mut = matched.group(1) is None
            self.basetype = matched.group(2).strip()
            self.indirection = matched.group(3)
        if self.indirection is None:
            self.indirection = ''
    
    def in_rust(self):
        t = self.basetype
        if t in cxx_to_rust:
            t = cxx_to_rust[t]
        ref = self.indirection
        if ref.startswith('*'):
            ref = '&' + ref[1:]
        if ref.startswith('&') and self.mut:
            return 'Pin<%smut %s>' % (ref, t)
        return '%s%s' % (ref, t)

if __name__ == '__main__':
    main()
