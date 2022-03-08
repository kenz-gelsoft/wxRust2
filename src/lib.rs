use std::convert::TryInto;
use std::os::raw::{c_char, c_int};
use std::ptr;

use cxx::{type_id, ExternType};

mod macros;

mod defs;
pub use defs::*;
mod manual;

mod generated;
pub use generated::*;

// any pointer type used on ffi boundary.
// we chose this type as it's handy in cxx.
type UnsafeAnyPtr = *const c_char;

// wxPoint
#[repr(C)]
pub struct wxPoint {
    pub x: c_int,
    pub y: c_int,
}
unsafe impl ExternType for wxPoint {
    type Id = type_id!("wxPoint");
    type Kind = cxx::kind::Trivial;
}

// wxSize
#[repr(C)]
pub struct wxSize {
    pub x: c_int,
    pub y: c_int,
}
unsafe impl ExternType for wxSize {
    type Id = type_id!("wxSize");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = "wxrust")]
mod ffi {
    enum EventType {
        Button,
    }

    struct Closure {
        // type alias can't be used in cxx:bridge.
        f: *const c_char,
        param: *const c_char,
    }

    #[namespace = ""]
    unsafe extern "C++" {
        include!("wx/include/wxrust.h");

        type wxEvtHandler;

        type wxString;

        unsafe fn wxEntry(argc: &mut i32, argv: *mut *mut c_char) -> i32;
    }

    unsafe extern "C++" {
        fn AppSetOnInit(closure: &Closure);
        fn Bind(handler: Pin<&mut wxEvtHandler>, eventType: EventType, closure: &Closure);

        fn NewString(s: &str) -> UniquePtr<wxString>;
    }
}

pub use ffi::EventType;

// Rust closure to wx calablle function+param pair.
impl ffi::Closure {
    fn new<F: Fn() + 'static>(closure: F) -> Self {
        unsafe fn trampoline<F: Fn() + 'static>(closure: UnsafeAnyPtr) {
            let closure = &*(closure as *const F);
            closure();
        }
        // pass the pointer in the heap to avoid move.
        let closure = Box::new(closure);
        Self {
            f: trampoline::<F> as UnsafeAnyPtr,
            param: Box::into_raw(closure) as UnsafeAnyPtr,
        }
    }
}

pub trait Bindable {
    fn bind<F: Fn() + 'static>(&self, event_type: ffi::EventType, closure: F);
}
impl<T: EvtHandlerMethods> Bindable for T {
    fn bind<F: Fn() + 'static>(&self, event_type: ffi::EventType, closure: F) {
        ffi::Bind(
            self.pinned::<ffi::wxEvtHandler>().as_mut(),
            event_type,
            &ffi::Closure::new(closure),
        );
    }
}

// wxApp
pub enum App {}
impl App {
    pub fn on_init<F: Fn() + 'static>(closure: F) {
        ffi::AppSetOnInit(&ffi::Closure::new(closure));
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
