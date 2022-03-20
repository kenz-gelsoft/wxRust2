use std::convert::TryInto;
use std::mem;
use std::os::raw::c_char;
use std::ptr;

mod macros;

mod defs;
pub use defs::*;
mod manual;
pub use manual::*;

mod generated;
pub use generated::*;

// any pointer type used on ffi boundary.
// we chose this type as it's handy in cxx.
type UnsafeAnyPtr = *const c_char;

#[cxx::bridge(namespace = "wxrust")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("wx/include/wxrust.h");

        type wxEvtHandler;

        type wxString;

        unsafe fn wxEntry(argc: &mut i32, argv: *mut *mut c_char) -> i32;
    }

    unsafe extern "C++" {
        unsafe fn AppSetOnInit(
            aFn: *const c_char,
            aParam: *const c_char
        );
        unsafe fn Bind(
            handler: Pin<&mut wxEvtHandler>,
            eventType: i32,
            aFn: *const c_char,
            aParam: *const c_char
        );

        unsafe fn wxString_new(psz: *const u8, nLength: usize) -> *mut wxString;
    }
}

pub struct WxString(*mut ffi::wxString);
pub unsafe fn wx_string_from(s: &str) -> *mut ffi::wxString {
    return ffi::wxString_new(s.as_ptr(), s.len())
}

// Rust closure to wx calablle function+param pair.
unsafe fn to_wx_callable<F: Fn() + 'static>(closure: F) -> (UnsafeAnyPtr, UnsafeAnyPtr) {
    unsafe fn trampoline<F: Fn() + 'static>(closure: UnsafeAnyPtr) {
        let closure = &*(closure as *const F);
        closure();
    }
    // pass the pointer in the heap to avoid move.
    let closure = Box::new(closure);
    (
        mem::transmute(trampoline::<F> as UnsafeAnyPtr),
        Box::into_raw(closure) as UnsafeAnyPtr
    )
}

pub trait Bindable {
    fn bind<F: Fn() + 'static>(&self, event_type: i32, closure: F);
}
impl<T: EvtHandlerMethods> Bindable for T {
    fn bind<F: Fn() + 'static>(&self, event_type: i32, closure: F) {
        unsafe {
            let (f, param) = to_wx_callable(closure);
            ffi::Bind(self.pinned::<ffi::wxEvtHandler>().as_mut(), event_type, f, param);
        }
    }
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

// wxEntry
pub fn entry() {
    let args: Vec<String> = std::env::args().collect();
    let mut argv: Vec<*mut c_char> = Vec::with_capacity(args.len() + 1);
    for arg in &args {
        argv.push(arg.as_ptr() as *mut c_char);
    }
    argv.push(ptr::null_mut()); // Nul terminator.
    let mut argc: i32 = args.len().try_into().unwrap();
    unsafe {
        ffi::wxEntry(&mut argc, argv.as_mut_ptr());
    }
}
