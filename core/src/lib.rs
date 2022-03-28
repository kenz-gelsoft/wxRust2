use std::os::raw::c_void;

use base::entry;

mod generated;
pub use generated::*;

mod ffi {
    use std::os::raw::{c_uchar, c_void};
    extern "C" {
        pub fn wxObject_delete(self_: *mut c_void);

        pub fn AppSetOnInit(
            aFn: *mut c_void,
            aParam: *mut c_void
        );

        // String
        pub fn wxString_new(psz: *const c_uchar, nLength: usize) -> *mut c_void;
        pub fn wxString_UTF8Data(self_: *mut c_void) -> *mut c_uchar;
        pub fn wxString_Len(self_: *mut c_void) -> usize;
    }
}

fn from_wx_string(s: *mut c_void) -> String {
    unsafe {
        let utf8data = ffi::wxString_UTF8Data(s);
        let len = ffi::wxString_Len(s);
        return String::from_raw_parts(utf8data, len, len);
    }
}
pub unsafe fn wx_string_from(s: &str) -> *const c_void {
    return ffi::wxString_new(s.as_ptr(), s.len())
}

// Rust closure to wx calablle function+param pair.
unsafe fn to_wx_callable<F: Fn() + 'static>(closure: F) -> (*mut c_void, *mut c_void) {
    unsafe fn trampoline<F: Fn() + 'static>(closure: *mut c_void) {
        let closure = &*(closure as *const F);
        closure();
    }
    // pass the pointer in the heap to avoid move.
    let closure = Box::new(closure);
    (
        trampoline::<F> as _,
        Box::into_raw(closure) as _
    )
}

// wxApp
pub enum App {}
impl App {
    pub fn on_init<F: Fn() + 'static>(closure: F) {
        unsafe {
            let (f, param) = to_wx_callable(closure);
            ffi::AppSetOnInit(f, param);
        }
    }
    pub fn run<F: Fn() + 'static>(closure: F) {
        Self::on_init(closure);
        entry();
    }
}

// wxDefaultPosition
impl Default for Point {
    fn default() -> Self { Point::new1(-1, -1) }
}
// wxDefaultSize
impl Default for Size {
    fn default() -> Self { Size::new1(-1, -1) }
}
// wxDefaultValidator
impl Default for Validator {
    fn default() -> Self { Validator::new() }
}
