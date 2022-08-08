from doxybindgen.model import Class, ClassManager
from doxybindgen.binding import CxxClassBinding, RustClassBinding

import os
import subprocess
import toml


# place wxWidgets doxygen xml files in wxml/ dir and run this.
def main():
    with open('doxybindgen.toml', 'r') as f:
        config = toml.load(f)
    
    classes = ClassManager()
    parsed = []
    for wxml in wxml_files():
        for cls in Class.in_xml(classes, wxml, config['types']):
            parsed.append(cls)
    classes.register(parsed)
    
    print_classes_in_lib(classes, config, 'base')
    print_classes_in_lib(classes, config, 'core')

def wxml_files():
    for root, dirs, files in os.walk('wxml'):
        for file in files:
            if (file.startswith('classwx_') and
                file.endswith('.xml')):
                yield os.path.join(root, file)

generated = []
def print_classes_in_lib(classes, config, libname):
    print('%s:' % (libname,))
    n = 0
    for cls in classes.in_lib(libname, generated):
        print('\t%s' % (cls.name,))
        n += 1
    print('\t%s classes.' % (n))

if __name__ == '__main__':
    main()
