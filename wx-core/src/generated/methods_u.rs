use super::*;

// wxUIActionSimulator
/// This trait represents [C++ `wxUIActionSimulator` class](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html)'s methods and inheritance.
///
/// See [`UIActionSimulatorFromCpp`] documentation for the class usage.
pub trait UIActionSimulatorMethods: WxRustMethods {
    /// Move the mouse to the specified coordinates.
    ///
    /// See [C++ `wxUIActionSimulator::MouseMove()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#a810d21924e14cb66205e4a921f711e5a).
    fn mouse_move_long(&self, x: c_long, y: c_long) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseMove(self.as_ptr(), x, y) }
    }
    /// Move the mouse to the specified coordinates.
    ///
    /// See [C++ `wxUIActionSimulator::MouseMove()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#aca7da1cd2fe227f2b1a85cfe8aef828b).
    fn mouse_move_point<P: PointMethods>(&self, point: &P) -> bool {
        unsafe {
            let point = point.as_ptr();
            ffi::wxUIActionSimulator_MouseMove1(self.as_ptr(), point)
        }
    }
    /// Press a mouse button.
    ///
    /// See [C++ `wxUIActionSimulator::MouseDown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#a9ddaf261490d1a71f723e713184424a1).
    fn mouse_down(&self, button: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseDown(self.as_ptr(), button) }
    }
    /// Release a mouse button.
    ///
    /// See [C++ `wxUIActionSimulator::MouseUp()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#a84bb2f01cfc044d682cbadd9efb9846f).
    fn mouse_up(&self, button: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseUp(self.as_ptr(), button) }
    }
    /// Click a mouse button.
    ///
    /// See [C++ `wxUIActionSimulator::MouseClick()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#a9b1c032466a935687c5e68d6eb77bcc5).
    fn mouse_click(&self, button: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseClick(self.as_ptr(), button) }
    }
    /// Double-click a mouse button.
    ///
    /// See [C++ `wxUIActionSimulator::MouseDblClick()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#aee0c2122c457fbd25bebb37e1f87e436).
    fn mouse_dbl_click(&self, button: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseDblClick(self.as_ptr(), button) }
    }
    /// Perform a drag and drop operation.
    ///
    /// See [C++ `wxUIActionSimulator::MouseDragDrop()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#a9365e46a8500daac15eadbbdc7c6fb9d).
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
    /// See [C++ `wxUIActionSimulator::KeyDown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#aabcdc7be926c32bd5b68b75f16c0fcb8).
    fn key_down(&self, keycode: c_int, modifiers: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_KeyDown(self.as_ptr(), keycode, modifiers) }
    }
    /// Release a key.
    ///
    /// See [C++ `wxUIActionSimulator::KeyUp()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#a7f08822c577ff59c7a04064285b12db1).
    fn key_up(&self, keycode: c_int, modifiers: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_KeyUp(self.as_ptr(), keycode, modifiers) }
    }
    /// Press and release a key.
    ///
    /// See [C++ `wxUIActionSimulator::Char()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#a573e464dc75d6168564a6372f6b9aa21).
    fn char(&self, keycode: c_int, modifiers: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_Char(self.as_ptr(), keycode, modifiers) }
    }
    /// Simulate selection of an item with the given text.
    ///
    /// See [C++ `wxUIActionSimulator::Select()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#a2a63445d3d08f9d2e062eeaa89b6a0ef).
    fn select(&self, text: &str) -> bool {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxUIActionSimulator_Select(self.as_ptr(), text)
        }
    }
    /// Emulate typing in the keys representing the given string.
    ///
    /// See [C++ `wxUIActionSimulator::Text()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#af7ecf0d4fe7c2565f327d83421df7a65).
    fn text(&self, text: *const c_void) -> bool {
        unsafe { ffi::wxUIActionSimulator_Text(self.as_ptr(), text) }
    }
}

// wxURLDataObject
/// This trait represents [C++ `wxURLDataObject` class](https://docs.wxwidgets.org/3.2/classwx_u_r_l_data_object.html)'s methods and inheritance.
///
/// See [`URLDataObjectFromCpp`] documentation for the class usage.
pub trait URLDataObjectMethods: DataObjectMethods {
    /// Returns the URL stored by this object, as a string.
    ///
    /// See [C++ `wxURLDataObject::GetURL()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_r_l_data_object.html#ae58c6e2a74b4d36fc7437448e7d80e01).
    fn get_url(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxURLDataObject_GetURL(self.as_ptr())).into() }
    }
    /// Sets the URL stored by this object.
    ///
    /// See [C++ `wxURLDataObject::SetURL()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_r_l_data_object.html#a6eb3ac7f0aea65eb54d005b91181bed2).
    fn set_url(&self, url: &str) {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            ffi::wxURLDataObject_SetURL(self.as_ptr(), url)
        }
    }
}

// wxUpdateUIEvent
/// This trait represents [C++ `wxUpdateUIEvent` class](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html)'s methods and inheritance.
///
/// See [`UpdateUIEventFromCpp`] documentation for the class usage.
pub trait UpdateUIEventMethods: CommandEventMethods {
    /// Check or uncheck the UI element.
    ///
    /// See [C++ `wxUpdateUIEvent::Check()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#abba131bbbd81478236298a606e3ed3cd).
    fn check(&self, check: bool) {
        unsafe { ffi::wxUpdateUIEvent_Check(self.as_ptr(), check) }
    }
    /// Enable or disable the UI element.
    ///
    /// See [C++ `wxUpdateUIEvent::Enable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a1105167b0e80d6ae19021656c5c404b8).
    fn enable(&self, enable: bool) {
        unsafe { ffi::wxUpdateUIEvent_Enable(self.as_ptr(), enable) }
    }
    /// Returns true if the UI element should be checked.
    ///
    /// See [C++ `wxUpdateUIEvent::GetChecked()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a10623f8b6ddeefad73e7c6421f7dad39).
    fn get_checked(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetChecked(self.as_ptr()) }
    }
    /// Returns true if the UI element should be enabled.
    ///
    /// See [C++ `wxUpdateUIEvent::GetEnabled()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#ac20d93b18a5201b4f9fae841867b0b02).
    fn get_enabled(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetEnabled(self.as_ptr()) }
    }
    /// Returns true if the UI element can be checked.
    ///
    /// See [C++ `wxUpdateUIEvent::IsCheckable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a22f2f3afa056d43d31736a2baa204d77).
    fn is_checkable(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_IsCheckable(self.as_ptr()) }
    }
    /// Returns true if the application has called Check().
    ///
    /// See [C++ `wxUpdateUIEvent::GetSetChecked()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a038221891a008537a0d942beb1a65ff2).
    fn get_set_checked(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetSetChecked(self.as_ptr()) }
    }
    /// Returns true if the application has called Enable().
    ///
    /// See [C++ `wxUpdateUIEvent::GetSetEnabled()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#ab61544747bc1e3c37004391fb7a27a15).
    fn get_set_enabled(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetSetEnabled(self.as_ptr()) }
    }
    /// Returns true if the application has called Show().
    ///
    /// See [C++ `wxUpdateUIEvent::GetSetShown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#aae527311483c11526b90a0887cb35563).
    fn get_set_shown(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetSetShown(self.as_ptr()) }
    }
    /// Returns true if the application has called SetText().
    ///
    /// See [C++ `wxUpdateUIEvent::GetSetText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#ae499b27877d0386dd38c86ad479c7801).
    fn get_set_text(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetSetText(self.as_ptr()) }
    }
    /// Returns true if the UI element should be shown.
    ///
    /// See [C++ `wxUpdateUIEvent::GetShown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a63c4462119dcb1a44e628c0554f619b2).
    fn get_shown(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetShown(self.as_ptr()) }
    }
    /// Returns the text that should be set for the UI element.
    ///
    /// See [C++ `wxUpdateUIEvent::GetText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a728f6f9203925e9040989fdf44099f30).
    fn get_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxUpdateUIEvent_GetText(self.as_ptr())).into() }
    }
    /// Sets the text for this UI element.
    ///
    /// See [C++ `wxUpdateUIEvent::SetText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a157efca52b0a423836610d115b4f69f7).
    fn set_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxUpdateUIEvent_SetText(self.as_ptr(), text)
        }
    }
    /// Show or hide the UI element.
    ///
    /// See [C++ `wxUpdateUIEvent::Show()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a9269df1c4a3345267dc3d7ef8ee8423c).
    fn show(&self, show: bool) {
        unsafe { ffi::wxUpdateUIEvent_Show(self.as_ptr(), show) }
    }
    /// Returns true if it is appropriate to update (send UI update events to) this window.
    ///
    /// See [C++ `wxUpdateUIEvent::CanUpdate()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a82d4a0dd8adea9d37d4b04cebe190d5f).
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
    /// See [C++ `wxUpdateUIEvent::GetUpdateInterval()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#af0218813302173f9eb33094f115ed565).
    fn get_update_interval() -> c_long {
        unsafe { ffi::wxUpdateUIEvent_GetUpdateInterval() }
    }
    /// Used internally to reset the last-updated time to the current time.
    ///
    /// See [C++ `wxUpdateUIEvent::ResetUpdateTime()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#acc9e27c48a04d2f798fd0836bd8c2988).
    fn reset_update_time() {
        unsafe { ffi::wxUpdateUIEvent_ResetUpdateTime() }
    }
    // NOT_SUPPORTED: fn SetMode()
    /// Sets the interval between updates in milliseconds.
    ///
    /// See [C++ `wxUpdateUIEvent::SetUpdateInterval()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#a24daac56f682b866baac592e761ccede).
    fn set_update_interval(update_interval: c_long) {
        unsafe { ffi::wxUpdateUIEvent_SetUpdateInterval(update_interval) }
    }
}
