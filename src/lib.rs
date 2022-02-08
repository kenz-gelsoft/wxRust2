use std::convert::TryInto;
use std::os::raw::c_char;
use std::pin::Pin;
use std::ptr;

mod macros;
use macros::wx_class;

mod defs;
pub use defs::*;
mod manual;

mod generated;
pub use generated::*;

// any pointer type used on ffi_manual boundary.
// we chose this type as it's handy in cxx.
type UnsafeAnyPtr = *const c_char;

#[cxx::bridge(namespace = "wxrust")]
pub mod ffi_manual {
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

        type wxWindow;
        fn Centre(self: Pin<&mut wxWindow>, direction: i32);
        fn Show(self: Pin<&mut wxWindow>, b: bool) -> bool;

        type wxFrame;

        type wxString;

        type wxButton;
        fn SetLabel(self: Pin<&mut wxButton>, label: &wxString);

        unsafe fn wxEntry(argc: &mut i32, argv: *mut *mut c_char) -> i32;
    }

    unsafe extern "C++" {
        fn AppSetOnInit(closure: &Closure);
        fn Bind(handler: Pin<&mut wxEvtHandler>, eventType: EventType, closure: &Closure);

        fn NewString(s: &str) -> UniquePtr<wxString>;
        fn NewFrame(title: &str) -> *mut wxFrame;
        fn NewButton(parent: Pin<&mut wxWindow>, label: &str) -> *mut wxButton;
    }
}

pub use ffi_manual::EventType;

// Rust closure to wx calablle function+param pair.
impl ffi_manual::Closure {
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

pub trait WxRustMethods {
    unsafe fn as_ptr(&self) -> UnsafeAnyPtr;
    fn pinned<T>(&self) -> Pin<&mut T> {
        unsafe { Pin::new_unchecked(&mut *(self.as_ptr() as *mut _)) }
    }
}

// wx_class! { EvtHandler(wxEvtHandler) impl
//     EvtHandlerMethods
// }
// pub trait EvtHandlerMethods: WxRustMethods {
//     fn bind<F: Fn() + 'static>(&self, event_type: ffi_manual::EventType, closure: F) {
//         ffi_manual::Bind(
//             self.pinned::<ffi_manual::wxEvtHandler>().as_mut(),
//             event_type,
//             &ffi_manual::Closure::new(closure),
//         );
//     }
// }

// wxApp
pub enum App {}
impl App {
    pub fn on_init<F: Fn() + 'static>(closure: F) {
        ffi_manual::AppSetOnInit(&ffi_manual::Closure::new(closure));
    }
    pub fn run<F: Fn() + 'static>(closure: F) {
        Self::on_init(closure);
        entry();
    }
}

// // wxWindow
// wx_class! { Window(wxWindow) impl
//     WindowMethods,
//     EvtHandlerMethods
// }
// pub trait WindowMethods: EvtHandlerMethods {
//     fn centre(&self) {
//         self.pinned::<ffi_manual::wxWindow>().as_mut().Centre(0);
//     }
//     fn show(&self) {
//         self.pinned::<ffi_manual::wxWindow>().as_mut().Show(true);
//     }
// }

// // wxFrame
// wx_class! { Frame(wxFrame) impl
//     WindowMethods,
//     EvtHandlerMethods
// }
// impl Frame {
//     pub fn new(title: &str) -> Frame {
//         Frame(ffi_manual::NewFrame(title))
//     }
// }

// // wxButton
// wx_class! { Button(wxButton) impl
//     ButtonMethods,
//     WindowMethods,
//     EvtHandlerMethods
// }
// impl Button {
//     pub fn new(parent: &Frame, label: &str) -> Button {
//         Button(ffi_manual::NewButton(parent.pinned(), label))
//     }
// }
// pub trait ButtonMethods: WindowMethods {
//     fn set_label(&self, s: &str) {
//         let label = ffi_manual::NewString(s);
//         self.pinned::<ffi_manual::wxButton>().as_mut().SetLabel(&label);
//     }
// }

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
        ffi_manual::wxEntry(&mut argc, argv.as_mut_ptr());
    }
}
