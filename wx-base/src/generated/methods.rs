use std::os::raw::c_void;

pub trait WxRustMethods {
    type CppManaged;
    unsafe fn as_ptr(&self) -> *mut c_void;
    unsafe fn from_ptr(ptr: *mut c_void) -> Self;
    unsafe fn from_cpp_managed_ptr(ptr: *mut c_void) -> Self::CppManaged;
    unsafe fn with_ptr<F: Fn(&Self)>(ptr: *mut c_void, closure: F);
    unsafe fn option_from(ptr: *mut c_void) -> Option<Self::CppManaged>
    where
        Self: Sized,
    {
        if ptr.is_null() {
            None
        } else {
            Some(Self::from_cpp_managed_ptr(ptr))
        }
    }
}

pub use super::methods_c::*;
pub use super::methods_d::*;
pub use super::methods_e::*;
pub use super::methods_f::*;
pub use super::methods_m::*;
pub use super::methods_o::*;
pub use super::methods_r::*;
pub use super::methods_s::*;
pub use super::methods_t::*;
pub use super::methods_v::*;
