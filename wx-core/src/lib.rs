use std::os::raw::c_int;
use std::ptr;

mod generated;
pub use generated::*;

// re-export wx_base
pub use wx_base::*;

#[doc(hidden)]
pub mod methods {
    pub use super::generated::methods::*;

    // re-export wx_base::methods
    pub use wx_base::methods::*;
}
use methods::*;

mod ffi {
    use std::os::raw::{c_int, c_void};
    extern "C" {
        pub fn wxObject_delete(self_: *mut c_void);
        pub fn wxRustMessageBox(
            message: *const c_void,
            caption: *const c_void,
            style: c_int,
            parent: *mut c_void,
            x: c_int,
            y: c_int,
        );
    }
}

// wxDefaultPosition
impl<const Owned: bool> Default for Point<Owned> {
    fn default() -> Self {
        Point::new_with_int(-1, -1)
    }
}
// wxDefaultSize
impl<const Owned: bool> Default for Size<Owned> {
    fn default() -> Self {
        Size::new_with_int(-1, -1)
    }
}
// wxDefaultValidator
impl<const Owned: bool> Default for Validator<Owned> {
    fn default() -> Self {
        Validator::new()
    }
}

pub fn message_box<T: WindowMethods>(
    message: &str,
    caption: &str,
    style: c_int,
    parent: Option<&T>,
) {
    unsafe {
        let message = wx_base::wx_string_from(message);
        let caption = wx_base::wx_string_from(caption);
        let parent = match parent {
            Some(r) => r.as_ptr(),
            None => ptr::null_mut(),
        };
        ffi::wxRustMessageBox(message, caption, style, parent, -1, -1)
    }
}
