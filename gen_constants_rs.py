from doxybindgen.constants import generate_constants_in, xml_files_in

import subprocess
import xml.etree.ElementTree as ET

PROLOGUE = '''\
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

// FIXME: workaround for windows (LLP64)
#![allow(overflowing_literals)]

use std::os::raw::{c_int, c_long};

use crate::manual::*;
'''

# place wxWidgets doxygen xml files in wxml/ dir and run this.
def main():
    outpath = 'wx-base/src/constants.rs'
    with open(outpath, 'w') as f:
        print(PROLOGUE, file=f)
        for file in xml_files_in('wxml/'):
            # print(file)
            tree = ET.parse(file)
            root = tree.getroot()

            for line in generate_constants_in(root):
                print(line, file=f)
            
    # print(subprocess.check_output(['rustfmt', outpath]))

if __name__ == '__main__':
    main()

