#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};

use super::*;
use methods::*;

pub use events::*;
mod events;

mod ffi;
mod ffi_c;
mod ffi_d;
mod ffi_e;
mod ffi_f;
mod ffi_m;
mod ffi_o;
mod ffi_r;
mod ffi_s;
mod ffi_t;
mod ffi_v;

pub mod methods;
mod methods_c;
mod methods_d;
mod methods_e;
mod methods_f;
mod methods_m;
mod methods_o;
mod methods_r;
mod methods_s;
mod methods_t;
mod methods_v;

pub mod class;
mod class_c;
mod class_d;
mod class_e;
mod class_f;
mod class_m;
mod class_o;
mod class_r;
mod class_s;
mod class_t;
mod class_v;
