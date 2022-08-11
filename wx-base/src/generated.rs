use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};

use super::*;
use methods::*;

// TODO: auto-generate
mod ffi_c;
mod ffi_d;
mod ffi_e;
mod ffi_f;
mod ffi_o;
mod ffi_s;
mod ffi_t;
mod ffi;

// TODO: auto-generate
mod methods_c;
mod methods_d;
mod methods_e;
mod methods_f;
mod methods_o;
mod methods_s;
mod methods_t;
pub mod methods;

mod class_c;
mod class_d;
mod class_e;
mod class_f;
mod class_o;
mod class_s;
mod class_t;
// TODO: auto-generate
pub use class_c::*;
pub use class_d::*;
pub use class_e::*;
pub use class_f::*;
pub use class_o::*;
pub use class_s::*;
pub use class_t::*;
