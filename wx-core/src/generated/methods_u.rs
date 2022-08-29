use super::*;

// wxUIActionSimulator
/// wxUIActionSimulator is a class used to simulate user interface actions such as a mouse click or a key press.
///
/// [See `wxUIActionSimulator`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html)
pub trait UIActionSimulatorMethods: WxRustMethods {
    /// Move the mouse to the specified coordinates.
    ///
    /// [See `wxUIActionSimulator::MouseMove()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#a810d21924e14cb66205e4a921f711e5a)
    fn mouse_move_long(&self, x: c_long, y: c_long) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseMove(self.as_ptr(), x, y) }
    }
    /// Move the mouse to the specified coordinates.
    ///
    /// [See `wxUIActionSimulator::MouseMove()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#aca7da1cd2fe227f2b1a85cfe8aef828b)
    fn mouse_move_point<P: PointMethods>(&self, point: &P) -> bool {
        unsafe {
            let point = point.as_ptr();
            ffi::wxUIActionSimulator_MouseMove1(self.as_ptr(), point)
        }
    }
    /// Press a mouse button.
    ///
    /// [See `wxUIActionSimulator::MouseDown()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#a9ddaf261490d1a71f723e713184424a1)
    fn mouse_down(&self, button: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseDown(self.as_ptr(), button) }
    }
    /// Release a mouse button.
    ///
    /// [See `wxUIActionSimulator::MouseUp()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#a84bb2f01cfc044d682cbadd9efb9846f)
    fn mouse_up(&self, button: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseUp(self.as_ptr(), button) }
    }
    /// Click a mouse button.
    ///
    /// [See `wxUIActionSimulator::MouseClick()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#a9b1c032466a935687c5e68d6eb77bcc5)
    fn mouse_click(&self, button: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseClick(self.as_ptr(), button) }
    }
    /// Double-click a mouse button.
    ///
    /// [See `wxUIActionSimulator::MouseDblClick()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#aee0c2122c457fbd25bebb37e1f87e436)
    fn mouse_dbl_click(&self, button: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseDblClick(self.as_ptr(), button) }
    }
    /// Perform a drag and drop operation.
    ///
    /// [See `wxUIActionSimulator::MouseDragDrop()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#a9365e46a8500daac15eadbbdc7c6fb9d)
    fn mouse_drag_drop(
        &self,
        x1: c_long,
        y1: c_long,
        x2: c_long,
        y2: c_long,
        button: c_int,
    ) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseDragDrop(self.as_ptr(), x1, y1, x2, y2, button) }
    }
    /// Press a key.
    ///
    /// [See `wxUIActionSimulator::KeyDown()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#aabcdc7be926c32bd5b68b75f16c0fcb8)
    fn key_down(&self, keycode: c_int, modifiers: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_KeyDown(self.as_ptr(), keycode, modifiers) }
    }
    /// Release a key.
    ///
    /// [See `wxUIActionSimulator::KeyUp()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#a7f08822c577ff59c7a04064285b12db1)
    fn key_up(&self, keycode: c_int, modifiers: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_KeyUp(self.as_ptr(), keycode, modifiers) }
    }
    /// Press and release a key.
    ///
    /// [See `wxUIActionSimulator::Char()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#a573e464dc75d6168564a6372f6b9aa21)
    fn char(&self, keycode: c_int, modifiers: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_Char(self.as_ptr(), keycode, modifiers) }
    }
    /// Simulate selection of an item with the given text.
    ///
    /// [See `wxUIActionSimulator::Select()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#a2a63445d3d08f9d2e062eeaa89b6a0ef)
    fn select(&self, text: &str) -> bool {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxUIActionSimulator_Select(self.as_ptr(), text)
        }
    }
    /// Emulate typing in the keys representing the given string.
    ///
    /// [See `wxUIActionSimulator::Text()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#af7ecf0d4fe7c2565f327d83421df7a65)
    fn text(&self, text: *const c_void) -> bool {
        unsafe { ffi::wxUIActionSimulator_Text(self.as_ptr(), text) }
    }
}

// wxURLDataObject
/// wxURLDataObject is a wxDataObject containing an URL and can be used e.g.
///
/// [See `wxURLDataObject`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_u_r_l_data_object.html)
pub trait URLDataObjectMethods: DataObjectMethods {
    /// Returns the URL stored by this object, as a string.
    ///
    /// [See `wxURLDataObject::GetURL()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_u_r_l_data_object.html#ae58c6e2a74b4d36fc7437448e7d80e01)
    fn get_url(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxURLDataObject_GetURL(self.as_ptr())).into() }
    }
    /// Sets the URL stored by this object.
    ///
    /// [See `wxURLDataObject::SetURL()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_u_r_l_data_object.html#a6eb3ac7f0aea65eb54d005b91181bed2)
    fn set_url(&self, url: &str) {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            ffi::wxURLDataObject_SetURL(self.as_ptr(), url)
        }
    }
}

// wxUpdateUIEvent
/// This class is used for pseudo-events which are called by wxWidgets to give an application the chance to update various user interface elements.
///
/// [See `wxUpdateUIEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html)
pub trait UpdateUIEventMethods: CommandEventMethods {
    /// Check or uncheck the UI element.
    ///
    /// [See `wxUpdateUIEvent::Check()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#abba131bbbd81478236298a606e3ed3cd)
    fn check(&self, check: bool) {
        unsafe { ffi::wxUpdateUIEvent_Check(self.as_ptr(), check) }
    }
    /// Enable or disable the UI element.
    ///
    /// [See `wxUpdateUIEvent::Enable()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a1105167b0e80d6ae19021656c5c404b8)
    fn enable(&self, enable: bool) {
        unsafe { ffi::wxUpdateUIEvent_Enable(self.as_ptr(), enable) }
    }
    /// Returns true if the UI element should be checked.
    ///
    /// [See `wxUpdateUIEvent::GetChecked()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a10623f8b6ddeefad73e7c6421f7dad39)
    fn get_checked(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetChecked(self.as_ptr()) }
    }
    /// Returns true if the UI element should be enabled.
    ///
    /// [See `wxUpdateUIEvent::GetEnabled()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#ac20d93b18a5201b4f9fae841867b0b02)
    fn get_enabled(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetEnabled(self.as_ptr()) }
    }
    /// Returns true if the UI element can be checked.
    ///
    /// [See `wxUpdateUIEvent::IsCheckable()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a22f2f3afa056d43d31736a2baa204d77)
    fn is_checkable(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_IsCheckable(self.as_ptr()) }
    }
    /// Returns true if the application has called Check().
    ///
    /// [See `wxUpdateUIEvent::GetSetChecked()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a038221891a008537a0d942beb1a65ff2)
    fn get_set_checked(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetSetChecked(self.as_ptr()) }
    }
    /// Returns true if the application has called Enable().
    ///
    /// [See `wxUpdateUIEvent::GetSetEnabled()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#ab61544747bc1e3c37004391fb7a27a15)
    fn get_set_enabled(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetSetEnabled(self.as_ptr()) }
    }
    /// Returns true if the application has called Show().
    ///
    /// [See `wxUpdateUIEvent::GetSetShown()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#aae527311483c11526b90a0887cb35563)
    fn get_set_shown(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetSetShown(self.as_ptr()) }
    }
    /// Returns true if the application has called SetText().
    ///
    /// [See `wxUpdateUIEvent::GetSetText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#ae499b27877d0386dd38c86ad479c7801)
    fn get_set_text(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetSetText(self.as_ptr()) }
    }
    /// Returns true if the UI element should be shown.
    ///
    /// [See `wxUpdateUIEvent::GetShown()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a63c4462119dcb1a44e628c0554f619b2)
    fn get_shown(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetShown(self.as_ptr()) }
    }
    /// Returns the text that should be set for the UI element.
    ///
    /// [See `wxUpdateUIEvent::GetText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a728f6f9203925e9040989fdf44099f30)
    fn get_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxUpdateUIEvent_GetText(self.as_ptr())).into() }
    }
    /// Sets the text for this UI element.
    ///
    /// [See `wxUpdateUIEvent::SetText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a157efca52b0a423836610d115b4f69f7)
    fn set_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxUpdateUIEvent_SetText(self.as_ptr(), text)
        }
    }
    /// Show or hide the UI element.
    ///
    /// [See `wxUpdateUIEvent::Show()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a9269df1c4a3345267dc3d7ef8ee8423c)
    fn show(&self, show: bool) {
        unsafe { ffi::wxUpdateUIEvent_Show(self.as_ptr(), show) }
    }
    /// Returns true if it is appropriate to update (send UI update events to) this window.
    ///
    /// [See `wxUpdateUIEvent::CanUpdate()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a82d4a0dd8adea9d37d4b04cebe190d5f)
    fn can_update<W: WindowMethods>(window: Option<&W>) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxUpdateUIEvent_CanUpdate(window)
        }
    }
    // NOT_SUPPORTED: fn GetMode()
    /// Returns the current interval between updates in milliseconds.
    ///
    /// [See `wxUpdateUIEvent::GetUpdateInterval()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#af0218813302173f9eb33094f115ed565)
    fn get_update_interval() -> c_long {
        unsafe { ffi::wxUpdateUIEvent_GetUpdateInterval() }
    }
    /// Used internally to reset the last-updated time to the current time.
    ///
    /// [See `wxUpdateUIEvent::ResetUpdateTime()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#acc9e27c48a04d2f798fd0836bd8c2988)
    fn reset_update_time() {
        unsafe { ffi::wxUpdateUIEvent_ResetUpdateTime() }
    }
    // NOT_SUPPORTED: fn SetMode()
    /// Sets the interval between updates in milliseconds.
    ///
    /// [See `wxUpdateUIEvent::SetUpdateInterval()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a24daac56f682b866baac592e761ccede)
    fn set_update_interval(update_interval: c_long) {
        unsafe { ffi::wxUpdateUIEvent_SetUpdateInterval(update_interval) }
    }
}
