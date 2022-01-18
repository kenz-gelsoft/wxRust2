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

type_mappings = {
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
            cls.print_constructor(f)
        print(H_EPILOGUE, file=f)

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
    
    def print_constructor(self, f):
        print('// CLASS: %s' % (self.name,),
                file=f)
        for method in self.methods:
            constructor = method.gen_constructor()
            if constructor is not None:
                print('%s' % (constructor,),
                        file=f)

class Method:
    def __init__(self, classname, e):
        self.classname = classname
        self.name = e.findtext('name')
        self.isconstructor = self.name == classname
        selftype = 'Pin<&mut %s>' % (classname,)
        self.params = ['self: %s' % (selftype,)]
        for param in e.findall('param'):
            ptype = ''.join(param.find('type').itertext())
            t = CxxType(ptype)
            pname = param.findtext('declname')
            self.params.append('%s: %s' % (pname, t.rusttype()))
    
    def __str__(self):
        body = 'unsafe fn %s(%s);' % (
            self.name,
            ', '.join(self.params),
        )
        if self.isconstructor:
            return '// %s' % (body,)
        return body

    def gen_constructor(self):
        body = '%s *%s(%s);' % (
            self.name,
            self.new_name(),
            ', '.join(self.params),
        )
        if self.isconstructor:
            return '%s' % (body,)
        return None

    def new_name(self):
        return 'New%s' % (self.name[2:],)

class CxxType:
    def __init__(self, s):
        self.origtype = s
        # print('parsing: |%s|' % (s,))
        matched = re.match(r'(const )?([^*&]*)([*&]+)?', s)
        self.basetype = None
        if matched:
            self.mut = matched.group(1) is None
            self.basetype = matched.group(2).strip()
            self.indirection = matched.group(3)
        if self.indirection is None:
            self.indirection = ''
    
    def rusttype(self):
        if self.basetype:
            t = self.basetype
            if t in type_mappings:
                t = type_mappings[t]
            mut = ''
            if self.indirection:
                mut = self.mut and 'mut ' or ''
            return '%s%s%s' % (self.indirection, mut, t)

        else:
            return None

if __name__ == '__main__':
    main()
