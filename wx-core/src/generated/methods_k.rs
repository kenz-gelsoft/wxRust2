use super::*;

// wxKeyEvent
/// This trait represents C++ [`wxKeyEvent`](https://docs.wxwidgets.org/3.2/classwx_key_event.html) class's methods and inheritance.
///
/// See [`KeyEventIsOwned`] documentation for the class usage.
pub trait KeyEventMethods: EventMethods {
    /// Returns the key code of the key that generated this event.
    ///
    /// [See `wxKeyEvent::GetKeyCode()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_key_event.html#a460e06e32b36973a63a02958ce4528b9)
    fn get_key_code(&self) -> c_int {
        unsafe { ffi::wxKeyEvent_GetKeyCode(self.as_ptr()) }
    }
    /// Returns true if the key is in the given key category.
    ///
    /// [See `wxKeyEvent::IsKeyInCategory()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_key_event.html#a1fdf62ae59cc6e39aaea7bfe75f57706)
    fn is_key_in_category(&self, category: c_int) -> bool {
        unsafe { ffi::wxKeyEvent_IsKeyInCategory(self.as_ptr(), category) }
    }
    /// Returns true if this event is an auto-repeat of the key, false if this is the initial key press.
    ///
    /// [See `wxKeyEvent::IsAutoRepeat()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_key_event.html#a3d71d5e2e0f0045310974a7460b5976e)
    fn is_auto_repeat(&self) -> bool {
        unsafe { ffi::wxKeyEvent_IsAutoRepeat(self.as_ptr()) }
    }
    /// Obtains the position (in client coordinates) at which the key was pressed.
    ///
    /// [See `wxKeyEvent::GetPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_key_event.html#ad96e42664e12bc376660890d2e585ccc)
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxKeyEvent_GetPosition(self.as_ptr())) }
    }
    ///
    /// [See `wxKeyEvent::GetPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_key_event.html#a5b307d2ab78138d67b3dc93fe85a3d74)
    fn get_position_coord(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxKeyEvent_GetPosition1(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn GetRawKeyCode()
    // NOT_SUPPORTED: fn GetRawKeyFlags()
    // NOT_SUPPORTED: fn GetUnicodeKey()
    /// Returns the X position (in client coordinates) of the event.
    ///
    /// [See `wxKeyEvent::GetX()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_key_event.html#a0bf492d5a1bcf0b071e72d4c9099afc9)
    fn get_x(&self) -> c_int {
        unsafe { ffi::wxKeyEvent_GetX(self.as_ptr()) }
    }
    /// Returns the Y position (in client coordinates) of the event.
    ///
    /// [See `wxKeyEvent::GetY()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_key_event.html#a79e7bde4b39389e1198a1e9a3fb77159)
    fn get_y(&self) -> c_int {
        unsafe { ffi::wxKeyEvent_GetY(self.as_ptr()) }
    }
    /// Allow normal key events generation.
    ///
    /// [See `wxKeyEvent::DoAllowNextEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_key_event.html#a4a7060ef0054d681cf8685e0467a663e)
    fn do_allow_next_event(&self) {
        unsafe { ffi::wxKeyEvent_DoAllowNextEvent(self.as_ptr()) }
    }
    /// Returns true if DoAllowNextEvent() had been called, false by default.
    ///
    /// [See `wxKeyEvent::IsNextEventAllowed()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_key_event.html#ae4406a48bc998d66cf636fb7bc856c52)
    fn is_next_event_allowed(&self) -> bool {
        unsafe { ffi::wxKeyEvent_IsNextEventAllowed(self.as_ptr()) }
    }
}
