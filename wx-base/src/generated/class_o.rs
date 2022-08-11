#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

use std::mem;
use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};
use std::ptr;

use super::methods::*;
use super::*;

use crate::wx_class;

// wxObject
wx_class! { Object =
    ObjectIsOwned<true>(wxObject) impl
        ObjectMethods
}
impl<const OWNED: bool> ObjectIsOwned<OWNED> {
    pub fn new() -> ObjectIsOwned<OWNED> {
        unsafe { ObjectIsOwned(ffi::wxObject_new()) }
    }
    pub fn new_with_object<O: ObjectMethods>(other: &O) -> ObjectIsOwned<OWNED> {
        unsafe {
            let other = other.as_ptr();
            ObjectIsOwned(ffi::wxObject_new1(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> DynamicCast for ObjectIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxObject_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
