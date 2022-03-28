mod generated;
pub use generated::*;

mod ffi {
    use std::os::raw::{c_void};
    extern "C" {
        pub fn wxObject_delete(self_: *mut c_void);
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
