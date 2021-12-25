use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("wxcxx/include/wxrust.h");

        fn WxRustAppSetOnInit(on_init: fn());

        type wxFrame;
        fn wxFrame_new(title: &str) -> *mut wxFrame;
        fn Centre(self: Pin<&mut wxFrame>, direction: i32);
        fn Show(self: Pin<&mut wxFrame>, b: bool) -> bool;

        unsafe fn wxEntry(argc: &mut i32, argv: *mut *mut c_char) -> i32;
    }
}

mod wx {
    use std::convert::TryInto;
    use std::os::raw::c_char;
    use std::ptr;

    // wxApp
    pub enum App {}
    impl App {
        pub fn on_init(f: fn()) {
            super::ffi::WxRustAppSetOnInit(f);
        }
    }

    // wxFrame
    pub struct Frame(*mut super::ffi::wxFrame);
    impl Frame {
        pub fn new(title: &str) -> Frame {
            Frame(super::ffi::wxFrame_new(title))
        }
        pub fn centre(&mut self) {
            self.pinned().as_mut().Centre(0);
        }
        pub fn show(&mut self) {
            self.pinned().as_mut().Show(true);
        }
        // private
        fn pinned(&mut self) -> super::Pin<&mut super::ffi::wxFrame> {
            unsafe { super::Pin::new_unchecked(&mut *self.0) }
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
            super::ffi::wxEntry(&mut argc, argv.as_mut_ptr());
        }
    }
}

fn main() {
    wx::App::on_init(|| {
        let mut frame = wx::Frame::new("Hello, 世界");
        frame.centre();
        frame.show();
    });

    wx::entry();
}
