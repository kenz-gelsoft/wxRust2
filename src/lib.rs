use std::convert::TryInto;
use std::os::raw::c_char;
use std::pin::Pin;
use std::ptr;

// any pointer type used on ffi boundary.
// we chose this type as it's handy in cxx.
type UnsafeAnyPtr = *const c_char;

#[cxx::bridge]
mod ffi {
    enum EventType {
        Button,
    }

    struct Closure {
        // type alias can't be used in cxx:bridge.
        f: *const c_char,
        param: *const c_char,
    }

    unsafe extern "C++" {
        include!("wx/include/wxrust.h");

        fn WxRustAppSetOnInit(closure: &Closure);

        type wxEvtHandler;
        fn Bind(
            handler: Pin<&mut wxEvtHandler>,
            eventType: EventType,
            closure: &Closure,
        );

        type wxWindow;
        fn Centre(self: Pin<&mut wxWindow>, direction: i32);
        fn Show(self: Pin<&mut wxWindow>, b: bool) -> bool;

        type wxFrame;
        fn wxFrame_new(title: &str) -> *mut wxFrame;

        type wxString;
        fn wxString_from(s: &str) -> *mut wxString;

        type wxButton;
        fn wxButton_new(parent: Pin<&mut wxWindow>, label: &str) -> *mut wxButton;
        fn SetLabel(self: Pin<&mut wxButton>, label: &wxString);

        unsafe fn wxEntry(argc: &mut i32, argv: *mut *mut c_char) -> i32;
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

pub trait ObjectMethods {
    unsafe fn as_ptr(&self) -> UnsafeAnyPtr;
}

pub struct EvtHandler(*mut ffi::wxEvtHandler);
impl EvtHandlerMethods for EvtHandler {}
impl ObjectMethods for EvtHandler {
    unsafe fn as_ptr(&self) -> UnsafeAnyPtr { self.0 as _ }
}
pub trait EvtHandlerMethods: ObjectMethods {
    fn pinned(&self) -> Pin<&mut ffi::wxEvtHandler> {
        unsafe { Pin::new_unchecked(&mut *(self.as_ptr() as *mut _)) }
    }
    fn bind<F: Fn() + 'static>(&self, event_type: ffi::EventType, closure: F) {
        ffi::Bind(self.pinned().as_mut(), event_type, &ffi::Closure::new(closure));
    }
}

// wxApp
pub enum App {}
impl App {
    pub fn on_init<F: Fn() + 'static>(closure: F) {
        ffi::WxRustAppSetOnInit(&ffi::Closure::new(closure));
    }
}

// wxWindow
pub struct Window(*mut ffi::wxWindow);
impl EvtHandlerMethods for Window {}
impl WindowMethods for Window {}
impl ObjectMethods for Window {
    unsafe fn as_ptr(&self) -> UnsafeAnyPtr { self.0 as _ }
}
pub trait WindowMethods: EvtHandlerMethods {
    fn pinned(&self) -> Pin<&mut ffi::wxWindow> {
        unsafe { Pin::new_unchecked(&mut *(self.as_ptr() as *mut _)) }
    }
    fn centre(&self) {
        WindowMethods::pinned(self).as_mut().Centre(0);
    }
    fn show(&self) {
        WindowMethods::pinned(self).as_mut().Show(true);
    }
}

// wxFrame
pub struct Frame(*mut ffi::wxFrame);
impl WindowMethods for Frame {}
impl EvtHandlerMethods for Frame {}
impl ObjectMethods for Frame {
    unsafe fn as_ptr(&self) -> UnsafeAnyPtr { self.0 as _ }
}
impl Frame {
    pub fn new(title: &str) -> Frame {
        Frame(ffi::wxFrame_new(title))
    }
}

// wxButton
#[derive(Clone)]
pub struct Button(*mut ffi::wxButton);
impl ButtonMethods for Button {}
impl WindowMethods for Button {}
impl EvtHandlerMethods for Button {}
impl ObjectMethods for Button {
    unsafe fn as_ptr(&self) -> UnsafeAnyPtr { self.0 as _ }
}
impl Button {
    pub fn new(parent: &Frame, label: &str) -> Button {
        Button(ffi::wxButton_new(WindowMethods::pinned(parent), label))
    }
}
pub trait ButtonMethods: WindowMethods {
    fn pinned(&self) -> Pin<&mut ffi::wxButton> {
        unsafe { Pin::new_unchecked(&mut *(self.as_ptr() as *mut _)) }
    }
    fn set_label(&self, s: &str) {
        unsafe {
            let label = ffi::wxString_from(s);
            ButtonMethods::pinned(self).as_mut().SetLabel(&*label);
        }
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
    let mut argc: i32 = args.len().try_into().unwrap();
    unsafe {
        ffi::wxEntry(&mut argc, argv.as_mut_ptr());
    }
}
