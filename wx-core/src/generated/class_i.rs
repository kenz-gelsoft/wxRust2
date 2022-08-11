#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

use std::mem;
use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};
use std::ptr;

use super::methods::*;
use super::*;

use wx_base::methods::*;
use wx_base::*;

// wxIcon
wx_class! { Icon =
    IconIsOwned<true>(wxIcon) impl
        IconMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> IconIsOwned<OWNED> {
    pub fn new() -> IconIsOwned<OWNED> {
        unsafe { IconIsOwned(ffi::wxIcon_new()) }
    }
    pub fn new_with_icon<I: IconMethods>(icon: &I) -> IconIsOwned<OWNED> {
        unsafe {
            let icon = icon.as_ptr();
            IconIsOwned(ffi::wxIcon_new1(icon))
        }
    }
    // NOT_SUPPORTED: fn wxIcon2()
    pub fn new_with_char(bits: *const c_void) -> IconIsOwned<OWNED> {
        unsafe { IconIsOwned(ffi::wxIcon_new3(bits)) }
    }
    // NOT_SUPPORTED: fn wxIcon4()
    pub fn new_with_iconlocation(loc: *const c_void) -> IconIsOwned<OWNED> {
        unsafe { IconIsOwned(ffi::wxIcon_new5(loc)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<IconIsOwned<OWNED>> for GDIObjectIsOwned<OWNED> {
    fn from(o: IconIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<IconIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: IconIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for IconIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxIcon_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for IconIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxItemContainer

// wxItemContainerImmutable
