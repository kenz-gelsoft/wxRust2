use std::convert::TryInto;
use std::mem;
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

    unsafe extern "C++" {
        include!("wx/include/wxrust.h");

        unsafe fn WxRustAppSetOnInit(
            // type alias can't be used in cxx:bridge.
            f: unsafe fn(*const c_char),
            param: *const c_char
        );

        type wxEvtHandler;
        unsafe fn Bind(
            handler: Pin<&mut wxEvtHandler>,
            eventType: EventType,
            // type alias can't be used in cxx:bridge.
            f: unsafe fn(*const c_char),
            param: *const c_char
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

unsafe fn trampoline<F: Fn() + 'static>(closure: UnsafeAnyPtr) {
    let closure = &*(closure as *const F);
    closure();
}

// Rust closure to wx calablle function+param pair.
trait ToWxCallable {
    unsafe fn to_wx_callable(&self) -> (fn(UnsafeAnyPtr), UnsafeAnyPtr);
}
impl<F> ToWxCallable for F where F: Fn() + 'static {
    unsafe fn to_wx_callable(&self) -> (fn(UnsafeAnyPtr), UnsafeAnyPtr) {
        // pass the pointer in the heap to avoid move.
        let closure = Box::new(self);
        (
            mem::transmute::<_, fn(UnsafeAnyPtr)>(
                trampoline::<F> as UnsafeAnyPtr
            ),
            Box::into_raw(closure) as UnsafeAnyPtr
        )
    }
}

pub struct EvtHandler(*mut ffi::wxEvtHandler);
impl EvtHandlerMethods for EvtHandler {
    fn pinned(&self) -> Pin<&mut ffi::wxEvtHandler> {
        unsafe { Pin::new_unchecked(&mut *self.0) }
    }
}
pub trait EvtHandlerMethods {
    fn pinned(&self) -> Pin<&mut ffi::wxEvtHandler>;
    fn bind<F: Fn() + 'static>(&self, event_type: ffi::EventType, closure: F) {
        unsafe {
            let (f, param) = closure.to_wx_callable();
            ffi::Bind(self.pinned().as_mut(), event_type, f, param);
        }
    }
}

// wxApp
pub enum App {}
impl App {
    pub fn on_init<F: Fn() + 'static>(closure: F) {
        unsafe {
            let (f, param) = closure.to_wx_callable();
            ffi::WxRustAppSetOnInit(f, param);
        }
    }
}

// wxWindow
pub struct Window(*mut ffi::wxWindow);
impl WindowMethods for Window {
    fn pinned(&self) -> Pin<&mut ffi::wxWindow> {
        unsafe { Pin::new_unchecked(&mut *self.0) }
    }
}
pub trait WindowMethods {
    fn pinned(&self) -> Pin<&mut ffi::wxWindow>;
    fn centre(&self) {
        self.pinned().as_mut().Centre(0);
    }
    fn show(&self) {
        self.pinned().as_mut().Show(true);
    }
}

// wxFrame
pub struct Frame(*mut ffi::wxFrame);
impl WindowMethods for Frame {
    fn pinned(&self) -> Pin<&mut ffi::wxWindow> {
        unsafe { Pin::new_unchecked(&mut *(self.0 as *mut ffi::wxWindow)) }
    }
}
impl Frame {
    pub fn new(title: &str) -> Frame {
        Frame(ffi::wxFrame_new(title))
    }
}

// wxButton
pub struct Button(*mut ffi::wxButton);
impl EvtHandlerMethods for Button {
    fn pinned(&self) -> Pin<&mut ffi::wxEvtHandler> {
        unsafe { Pin::new_unchecked(&mut *(self.0 as *mut ffi::wxEvtHandler)) }
    }
}
impl WindowMethods for Button {
    fn pinned(&self) -> Pin<&mut ffi::wxWindow> {
        unsafe { Pin::new_unchecked(&mut *(self.0 as *mut ffi::wxWindow)) }
    }
}
impl Button {
    pub fn new(parent: &Frame, label: &str) -> Button {
        Button(ffi::wxButton_new(parent.pinned(), label))
    }
}
impl ButtonMethods for Button {
    fn pinned(&self) -> Pin<&mut ffi::wxButton> {
        unsafe { Pin::new_unchecked(&mut *self.0) }
    }
}
pub trait ButtonMethods {
    fn pinned(&self) -> Pin<&mut ffi::wxButton>;
    fn set_label(&self, s: &str) {
        unsafe {
            let label = ffi::wxString_from(s);
            self.pinned().as_mut().SetLabel(&*label);
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
