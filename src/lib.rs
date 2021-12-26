use std::convert::TryInto;
use std::os::raw::c_char;
use std::pin::Pin;
use std::ptr;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("wx/include/wxrust.h");

        fn WxRustAppSetOnInit(on_init: fn());

        type wxWindow;

        type wxFrame;
        fn wxFrame_new(title: &str) -> *mut wxFrame;
        fn Centre(self: Pin<&mut wxFrame>, direction: i32);
        fn Show(self: Pin<&mut wxFrame>, b: bool) -> bool;

        type wxButton;
        fn wxButton_new(parent: Pin<&mut wxFrame>, label: &str) -> *mut wxButton;

        unsafe fn wxEntry(argc: &mut i32, argv: *mut *mut c_char) -> i32;
    }
}

// wxApp
pub enum App {}
impl App {
    pub fn on_init(f: fn()) {
        ffi::WxRustAppSetOnInit(f);
    }
}

// wxFrame
pub struct Frame(*mut ffi::wxFrame);
impl Frame {
    pub fn new(title: &str) -> Frame {
        Frame(ffi::wxFrame_new(title))
    }
    pub fn centre(&mut self) {
        self.pinned().as_mut().Centre(0);
    }
    pub fn show(&mut self) {
        self.pinned().as_mut().Show(true);
    }
    // private
    fn pinned(&mut self) -> Pin<&mut ffi::wxFrame> {
        unsafe { Pin::new_unchecked(&mut *self.0) }
    }
}

// wxButton
pub struct Button(*mut ffi::wxButton);
impl Button {
    pub fn new(parent: &mut Frame, label: &str) -> Button {
        Button(ffi::wxButton_new(parent.pinned(), label))
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
