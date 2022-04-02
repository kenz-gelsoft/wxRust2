use std::convert::TryInto;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

mod macros;

mod defs;
pub use defs::*;
mod manual;
pub use manual::*;

mod generated;
pub use generated::*;

mod ffi {
    use std::os::raw::{c_char, c_int, c_uchar, c_void};
    extern "C" {
        pub fn wxObject_delete(self_: *mut c_void);

        pub fn AppSetOnInit(
            aFn: *mut c_void,
            aParam: *mut c_void
        );
        pub fn wxEvtHandler_Bind(
            self_: *mut c_void,
            eventType: c_int,
            aFn: *mut c_void,
            aParam: *mut c_void
        );

        // String
        pub fn wxString_new(psz: *const c_uchar, nLength: usize) -> *mut c_void;
        pub fn wxString_UTF8Data(self_: *mut c_void) -> *mut c_uchar;
        pub fn wxString_Len(self_: *mut c_void) -> usize;
        
        pub fn wxRustEntry(argc: *mut c_int, argv: *mut *mut c_char) -> c_int;
    }
}

pub fn from_wx_string(s: *mut c_void) -> String {
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
unsafe fn to_wx_callable<F: Fn(*mut c_void) + 'static>(closure: F) -> (*mut c_void, *mut c_void) {
    unsafe fn trampoline<F: Fn(*mut c_void) + 'static>(closure: *mut c_void, arg: *mut c_void) {
        let closure = &*(closure as *const F);
        closure(arg);
    }
    // pass the pointer in the heap to avoid move.
    let closure = Box::new(closure);
    (
        trampoline::<F> as _,
        Box::into_raw(closure) as _
    )
}

pub trait Bindable {
    fn bind<F: Fn(*mut c_void) + 'static>(&self, event_type: c_int, closure: F);
}
impl<T: EvtHandlerMethods> Bindable for T {
    fn bind<F: Fn(*mut c_void) + 'static>(&self, event_type: c_int, closure: F) {
        unsafe {
            let (f, param) = to_wx_callable(closure);
            ffi::wxEvtHandler_Bind(self.as_ptr(), event_type, f, param);
        }
    }
}

// wxApp
pub enum App {}
impl App {
    pub fn on_init<F: Fn(*mut c_void) + 'static>(closure: F) {
        unsafe {
            let (f, param) = to_wx_callable(closure);
            ffi::AppSetOnInit(f, param);
        }
    }
    pub fn run<F: Fn(*mut c_void) + 'static>(closure: F) {
        Self::on_init(closure);
        entry();
    }
}

// wxEntry
pub fn entry() {
    let args: Vec<String> = std::env::args().collect();
    let mut argv: Vec<*mut c_char> = Vec::with_capacity(args.len() + 1);
    for arg in &args {
        argv.push(arg.as_ptr() as *mut c_char);
    }
    argv.push(ptr::null_mut()); // Nul terminator.
    let mut argc: c_int = args.len().try_into().unwrap();
    unsafe {
        ffi::wxRustEntry(&mut argc, argv.as_mut_ptr());
    }
}
