use super::*;

// wxPCXHandler
/// This trait represents [C++ `wxPCXHandler` class](https://docs.wxwidgets.org/3.2/classwx_p_c_x_handler.html)'s methods and inheritance.
///
/// See [`PCXHandlerInRust`] documentation for the class usage.
pub trait PCXHandlerMethods: ImageHandlerMethods {}

// wxPNGHandler
/// This trait represents [C++ `wxPNGHandler` class](https://docs.wxwidgets.org/3.2/classwx_p_n_g_handler.html)'s methods and inheritance.
///
/// See [`PNGHandlerInRust`] documentation for the class usage.
pub trait PNGHandlerMethods: ImageHandlerMethods {}

// wxPNMHandler
/// This trait represents [C++ `wxPNMHandler` class](https://docs.wxwidgets.org/3.2/classwx_p_n_m_handler.html)'s methods and inheritance.
///
/// See [`PNMHandlerInRust`] documentation for the class usage.
pub trait PNMHandlerMethods: ImageHandlerMethods {}

// wxPaintDC
/// This trait represents [C++ `wxPaintDC` class](https://docs.wxwidgets.org/3.2/classwx_paint_d_c.html)'s methods and inheritance.
///
/// See [`PaintDCInRust`] documentation for the class usage.
pub trait PaintDCMethods: ClientDCMethods {}

// wxPaintEvent
/// This trait represents [C++ `wxPaintEvent` class](https://docs.wxwidgets.org/3.2/classwx_paint_event.html)'s methods and inheritance.
///
/// See [`PaintEventInRust`] documentation for the class usage.
pub trait PaintEventMethods: EventMethods {}

// wxPalette
/// This trait represents [C++ `wxPalette` class](https://docs.wxwidgets.org/3.2/classwx_palette.html)'s methods and inheritance.
///
/// See [`PaletteInRust`] documentation for the class usage.
pub trait PaletteMethods: GDIObjectMethods {
    // DTOR: fn ~wxPalette()
    /// Creates a palette from arrays of size n, one for each red, blue or green component.
    ///
    /// See [C++ `wxPalette::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_palette.html#aeb040e229b732bc46dac0dc204f005f4).
    fn create(
        &self,
        n: c_int,
        red: *const c_void,
        green: *const c_void,
        blue: *const c_void,
    ) -> bool {
        unsafe { ffi::wxPalette_Create(self.as_ptr(), n, red, green, blue) }
    }
    /// Returns number of entries in palette.
    ///
    /// See [C++ `wxPalette::GetColoursCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_palette.html#ae0b3f8ee520fcaffcbe4a35b4cf56c88).
    fn get_colours_count(&self) -> c_int {
        unsafe { ffi::wxPalette_GetColoursCount(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetPixel()
    /// Returns RGB values for a given palette index.
    ///
    /// See [C++ `wxPalette::GetRGB()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_palette.html#aaf1aee2c29a18bc3c2087cab76c09418).
    fn get_rgb(
        &self,
        pixel: c_int,
        red: *mut c_void,
        green: *mut c_void,
        blue: *mut c_void,
    ) -> bool {
        unsafe { ffi::wxPalette_GetRGB(self.as_ptr(), pixel, red, green, blue) }
    }
    /// Returns true if palette data is present.
    ///
    /// See [C++ `wxPalette::IsOk()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_palette.html#a82713e81e9006f455513895e12546429).
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxPalette_IsOk(self.as_ptr()) }
    }
    // BLOCKED: fn operator=()
}

// wxPanel
/// This trait represents [C++ `wxPanel` class](https://docs.wxwidgets.org/3.2/classwx_panel.html)'s methods and inheritance.
///
/// See [`PanelInRust`] documentation for the class usage.
pub trait PanelMethods: WindowMethods {
    // DTOR: fn ~wxPanel()
    /// The default handler for wxEVT_SYS_COLOUR_CHANGED.
    ///
    /// See [C++ `wxPanel::OnSysColourChanged()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_panel.html#a3953dd8ab6e04b15206b24a96c2636a4).
    fn on_sys_colour_changed<S: SysColourChangedEventMethods>(&self, event: &S) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxPanel_OnSysColourChanged(self.as_ptr(), event)
        }
    }
    /// In contrast to SetFocus() (see above) this will set the focus to the panel even if there are child windows in the panel.
    ///
    /// See [C++ `wxPanel::SetFocusIgnoringChildren()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_panel.html#ae1f608902d585383401423a8e4eefe13).
    fn set_focus_ignoring_children(&self) {
        unsafe { ffi::wxPanel_SetFocusIgnoringChildren(self.as_ptr()) }
    }
}

// wxPasswordEntryDialog
/// This trait represents [C++ `wxPasswordEntryDialog` class](https://docs.wxwidgets.org/3.2/classwx_password_entry_dialog.html)'s methods and inheritance.
///
/// See [`PasswordEntryDialogInRust`] documentation for the class usage.
pub trait PasswordEntryDialogMethods: TextEntryDialogMethods {}

// wxPen
/// This trait represents [C++ `wxPen` class](https://docs.wxwidgets.org/3.2/classwx_pen.html)'s methods and inheritance.
///
/// See [`PenInRust`] documentation for the class usage.
pub trait PenMethods: GDIObjectMethods {
    // DTOR: fn ~wxPen()
    // NOT_SUPPORTED: fn GetCap()
    // NOT_SUPPORTED: fn GetQuality()
    /// Returns a reference to the pen colour.
    ///
    /// See [C++ `wxPen::GetColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_pen.html#a858c7585d538a9aefcea076324ce0fc6).
    fn get_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxPen_GetColour(self.as_ptr())) }
    }
    /// Gets an array of dashes (defined as char in X, DWORD under Windows).
    ///
    /// See [C++ `wxPen::GetDashes()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_pen.html#a1626800f8552ac647c0316894049a93e).
    fn get_dashes(&self, dashes: *mut c_void) -> c_int {
        unsafe { ffi::wxPen_GetDashes(self.as_ptr(), dashes) }
    }
    // NOT_SUPPORTED: fn GetJoin()
    /// Gets a pointer to the stipple bitmap.
    ///
    /// See [C++ `wxPen::GetStipple()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_pen.html#a7db669184e8e44c2db2d5c7d1549ca51).
    fn get_stipple(&self) -> Option<BitmapInRust<false>> {
        unsafe { Bitmap::option_from(ffi::wxPen_GetStipple(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetStyle()
    /// Returns the pen width.
    ///
    /// See [C++ `wxPen::GetWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_pen.html#ab053c4234c43e74d366f0b55dde36ba4).
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxPen_GetWidth(self.as_ptr()) }
    }
    /// Returns true if the pen is initialised.
    ///
    /// See [C++ `wxPen::IsOk()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_pen.html#a5fe34607626c5d841b4ce3ad5d03170f).
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxPen_IsOk(self.as_ptr()) }
    }
    /// Returns true if the pen is a valid non-transparent pen.
    ///
    /// See [C++ `wxPen::IsNonTransparent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_pen.html#a28229587b2a94f21966a336124dc7c92).
    fn is_non_transparent(&self) -> bool {
        unsafe { ffi::wxPen_IsNonTransparent(self.as_ptr()) }
    }
    /// Returns true if the pen is transparent.
    ///
    /// See [C++ `wxPen::IsTransparent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_pen.html#ad5461db9bcfa6370c8cf9a1ae30e9ae5).
    fn is_transparent(&self) -> bool {
        unsafe { ffi::wxPen_IsTransparent(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetCap()
    // NOT_SUPPORTED: fn SetQuality()
    /// The pen's colour is changed to the given colour.
    ///
    /// See [C++ `wxPen::SetColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_pen.html#a3c45922aa86019a0e2ffdeedd0182931).
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxPen_SetColour(self.as_ptr(), colour)
        }
    }
    // NOT_SUPPORTED: fn SetColour1()
    /// Associates an array of dash values (defined as char in X, DWORD under Windows) with the pen.
    ///
    /// See [C++ `wxPen::SetDashes()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_pen.html#a6f4471824d023b98e5d884b00535b573).
    fn set_dashes(&self, n: c_int, dash: *const c_void) {
        unsafe { ffi::wxPen_SetDashes(self.as_ptr(), n, dash) }
    }
    // NOT_SUPPORTED: fn SetJoin()
    /// Sets the bitmap for stippling.
    ///
    /// See [C++ `wxPen::SetStipple()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_pen.html#acb75ec80dcd9534181f8c17520aca2b0).
    fn set_stipple<B: BitmapMethods>(&self, stipple: &B) {
        unsafe {
            let stipple = stipple.as_ptr();
            ffi::wxPen_SetStipple(self.as_ptr(), stipple)
        }
    }
    // NOT_SUPPORTED: fn SetStyle()
    /// Sets the pen width.
    ///
    /// See [C++ `wxPen::SetWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_pen.html#ad390a7361cd737fcb176db622f5ad039).
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxPen_SetWidth(self.as_ptr(), width) }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
}

// wxPenList
/// This trait represents [C++ `wxPenList` class](https://docs.wxwidgets.org/3.2/classwx_pen_list.html)'s methods and inheritance.
///
/// See [`PenListInRust`] documentation for the class usage.
pub trait PenListMethods: WxRustMethods {
    // NOT_SUPPORTED: fn FindOrCreatePen()
}

// wxPersistenceManager
/// This trait represents [C++ `wxPersistenceManager` class](https://docs.wxwidgets.org/3.2/classwx_persistence_manager.html)'s methods and inheritance.
///
/// See [`PersistenceManagerInRust`] documentation for the class usage.
pub trait PersistenceManagerMethods: WxRustMethods {
    /// Set the global persistence manager to use.
    ///
    /// See [C++ `wxPersistenceManager::Set()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_persistence_manager.html#a4cbc79238e7d46f5e201c67e86fd5343).
    fn set<P: PersistenceManagerMethods>(manager: &P) {
        unsafe {
            let manager = manager.as_ptr();
            ffi::wxPersistenceManager_Set(manager)
        }
    }
    /// Returns the unique persistence manager object.
    ///
    /// See [C++ `wxPersistenceManager::Get()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_persistence_manager.html#a4a6a946bdd6dc862fb66db774a4b9de7).
    fn get() -> PersistenceManagerInRust<false> {
        unsafe { PersistenceManagerInRust::from_ptr(ffi::wxPersistenceManager_Get()) }
    }
    /// Globally disable saving the persistence object properties.
    ///
    /// See [C++ `wxPersistenceManager::DisableSaving()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_persistence_manager.html#a4183a42f9dfbd223c363c2879b85d137).
    fn disable_saving(&self) {
        unsafe { ffi::wxPersistenceManager_DisableSaving(self.as_ptr()) }
    }
    /// Globally disable restoring the persistence object properties.
    ///
    /// See [C++ `wxPersistenceManager::DisableRestoring()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_persistence_manager.html#aea41f3b4c596f7bc90f43e888faac93b).
    fn disable_restoring(&self) {
        unsafe { ffi::wxPersistenceManager_DisableRestoring(self.as_ptr()) }
    }
    // BLOCKED: fn Register()
    /// Register an object with the manager.
    ///
    /// See [C++ `wxPersistenceManager::Register()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_persistence_manager.html#ac34c8e512fa7cef9d011d8df5809cfee).
    fn register(&self, obj: *mut c_void, po: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxPersistenceManager_Register1(self.as_ptr(), obj, po) }
    }
    /// Check if the object is registered and return the associated wxPersistentObject if it is or NULL otherwise.
    ///
    /// See [C++ `wxPersistenceManager::Find()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_persistence_manager.html#a832fcd53e5d4ddfd31bc3334ccb5e1d9).
    fn find(&self, obj: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxPersistenceManager_Find(self.as_ptr(), obj) }
    }
    /// Unregister the object and delete the associated wxPersistentObject.
    ///
    /// See [C++ `wxPersistenceManager::Unregister()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_persistence_manager.html#a453383431eb8f8127f0ac0d8a2cee1c5).
    fn unregister(&self, obj: *mut c_void) {
        unsafe { ffi::wxPersistenceManager_Unregister(self.as_ptr(), obj) }
    }
    /// Save the object properties to persistent storage.
    ///
    /// See [C++ `wxPersistenceManager::Save()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_persistence_manager.html#ae0b01606ded5323e923b1c9848bdb3bb).
    fn save(&self, obj: *mut c_void) {
        unsafe { ffi::wxPersistenceManager_Save(self.as_ptr(), obj) }
    }
    /// Restore the object properties previously saved by Save().
    ///
    /// See [C++ `wxPersistenceManager::Restore()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_persistence_manager.html#a932d8f5cf981aed723f79d1923f67aed).
    fn restore(&self, obj: *mut c_void) -> bool {
        unsafe { ffi::wxPersistenceManager_Restore(self.as_ptr(), obj) }
    }
    /// Combines both Save() and Unregister() calls.
    ///
    /// See [C++ `wxPersistenceManager::SaveAndUnregister()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_persistence_manager.html#ab8a9b090b5f84df7b6a765fed78d4820).
    fn save_and_unregister(&self, obj: *mut c_void) {
        unsafe { ffi::wxPersistenceManager_SaveAndUnregister(self.as_ptr(), obj) }
    }
    // BLOCKED: fn RegisterAndRestore()
    ///
    /// See [C++ `wxPersistenceManager::RegisterAndRestore()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_persistence_manager.html#ac0a4d5858a69cd6db22e5ab75e5e9298).
    fn register_and_restore(&self, obj: *mut c_void, po: *mut c_void) -> bool {
        unsafe { ffi::wxPersistenceManager_RegisterAndRestore1(self.as_ptr(), obj, po) }
    }
}

// wxPickerBase
/// This trait represents [C++ `wxPickerBase` class](https://docs.wxwidgets.org/3.2/classwx_picker_base.html)'s methods and inheritance.
///
/// See [`PickerBaseInRust`] documentation for the class usage.
pub trait PickerBaseMethods: ControlMethods {
    // DTOR: fn ~wxPickerBase()
    ///
    /// See [C++ `wxPickerBase::CreateBase()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#a768d847f06c01b3e757656020e153c45).
    fn create_base<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        text: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let text = WxString::from(text);
            let text = text.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxPickerBase_CreateBase(
                self.as_ptr(),
                parent,
                id,
                text,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    /// Returns the margin (in pixel) between the picker and the text control.
    ///
    /// See [C++ `wxPickerBase::GetInternalMargin()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#a0a1bcdc7c9471b84a1a375c89d92b87f).
    fn get_internal_margin(&self) -> c_int {
        unsafe { ffi::wxPickerBase_GetInternalMargin(self.as_ptr()) }
    }
    /// Returns the proportion value of the picker.
    ///
    /// See [C++ `wxPickerBase::GetPickerCtrlProportion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#a28609a3b5850503ea4251906ebd0b4dc).
    fn get_picker_ctrl_proportion(&self) -> c_int {
        unsafe { ffi::wxPickerBase_GetPickerCtrlProportion(self.as_ptr()) }
    }
    /// Returns a pointer to the text control handled by this window or NULL if the wxPB_USE_TEXTCTRL style was not specified when this control was created.
    ///
    /// See [C++ `wxPickerBase::GetTextCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#ab03bc458da2c742c111dad65a83c972e).
    fn get_text_ctrl(&self) -> WeakRef<TextCtrl> {
        unsafe { WeakRef::<TextCtrl>::from(ffi::wxPickerBase_GetTextCtrl(self.as_ptr())) }
    }
    /// Returns the native implementation of the real picker control.
    ///
    /// See [C++ `wxPickerBase::GetPickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#aa66d96bd6e7925b5c44596cbbbad9580).
    fn get_picker_ctrl(&self) -> WeakRef<Control> {
        unsafe { WeakRef::<Control>::from(ffi::wxPickerBase_GetPickerCtrl(self.as_ptr())) }
    }
    /// Returns the proportion value of the text control.
    ///
    /// See [C++ `wxPickerBase::GetTextCtrlProportion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#ab167d0c703cd52f512c1d641bd234b32).
    fn get_text_ctrl_proportion(&self) -> c_int {
        unsafe { ffi::wxPickerBase_GetTextCtrlProportion(self.as_ptr()) }
    }
    /// Returns true if this window has a valid text control (i.e. if the wxPB_USE_TEXTCTRL style was given when creating this control).
    ///
    /// See [C++ `wxPickerBase::HasTextCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#ac34e92a720e9d34a2022b965984c591f).
    fn has_text_ctrl(&self) -> bool {
        unsafe { ffi::wxPickerBase_HasTextCtrl(self.as_ptr()) }
    }
    /// Returns true if the picker control is growable.
    ///
    /// See [C++ `wxPickerBase::IsPickerCtrlGrowable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#a1b472954a6879e26873b0ebb17429962).
    fn is_picker_ctrl_growable(&self) -> bool {
        unsafe { ffi::wxPickerBase_IsPickerCtrlGrowable(self.as_ptr()) }
    }
    /// Returns true if the text control is growable.
    ///
    /// See [C++ `wxPickerBase::IsTextCtrlGrowable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#a35b42f178d0ef26c590a71281af8b979).
    fn is_text_ctrl_growable(&self) -> bool {
        unsafe { ffi::wxPickerBase_IsTextCtrlGrowable(self.as_ptr()) }
    }
    /// Sets the margin (in pixel) between the picker and the text control.
    ///
    /// See [C++ `wxPickerBase::SetInternalMargin()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#a902b0b8db0a78820d8d985500f5f66c1).
    fn set_internal_margin(&self, margin: c_int) {
        unsafe { ffi::wxPickerBase_SetInternalMargin(self.as_ptr(), margin) }
    }
    /// Sets the picker control as growable when grow is true.
    ///
    /// See [C++ `wxPickerBase::SetPickerCtrlGrowable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#ab031bd45e323b43b6a9943e25ec6e191).
    fn set_picker_ctrl_growable(&self, grow: bool) {
        unsafe { ffi::wxPickerBase_SetPickerCtrlGrowable(self.as_ptr(), grow) }
    }
    /// Sets the proportion value of the picker.
    ///
    /// See [C++ `wxPickerBase::SetPickerCtrlProportion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#a5801852b32e1e8020bfa48b75bde67a5).
    fn set_picker_ctrl_proportion(&self, prop: c_int) {
        unsafe { ffi::wxPickerBase_SetPickerCtrlProportion(self.as_ptr(), prop) }
    }
    /// Sets the text control as growable when grow is true.
    ///
    /// See [C++ `wxPickerBase::SetTextCtrlGrowable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#a094adc3942377519ecee6ce1afeb2954).
    fn set_text_ctrl_growable(&self, grow: bool) {
        unsafe { ffi::wxPickerBase_SetTextCtrlGrowable(self.as_ptr(), grow) }
    }
    /// Sets the proportion value of the text control.
    ///
    /// See [C++ `wxPickerBase::SetTextCtrlProportion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#a34d76334dfea7b6f65bbbb3b41269a9e).
    fn set_text_ctrl_proportion(&self, prop: c_int) {
        unsafe { ffi::wxPickerBase_SetTextCtrlProportion(self.as_ptr(), prop) }
    }
    ///
    /// See [C++ `wxPickerBase::SetTextCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#a06f3f7837fa1ac0cc0a860e591e0b34a).
    fn set_text_ctrl<T: TextCtrlMethods>(&self, text: Option<&T>) {
        unsafe {
            let text = match text {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPickerBase_SetTextCtrl(self.as_ptr(), text)
        }
    }
    ///
    /// See [C++ `wxPickerBase::SetPickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#a87e04d99810b342d358d21d26b4c8915).
    fn set_picker_ctrl<C: ControlMethods>(&self, picker: Option<&C>) {
        unsafe {
            let picker = match picker {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPickerBase_SetPickerCtrl(self.as_ptr(), picker)
        }
    }
    ///
    /// See [C++ `wxPickerBase::UpdatePickerFromTextCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#a6dd0785ceb13d2c71cadecf7caf402bf).
    fn update_picker_from_text_ctrl(&self) {
        unsafe { ffi::wxPickerBase_UpdatePickerFromTextCtrl(self.as_ptr()) }
    }
    ///
    /// See [C++ `wxPickerBase::UpdateTextCtrlFromPicker()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_picker_base.html#a0be21e2897a509151242390054b86bb8).
    fn update_text_ctrl_from_picker(&self) {
        unsafe { ffi::wxPickerBase_UpdateTextCtrlFromPicker(self.as_ptr()) }
    }
}

// wxPoint
/// This trait represents [C++ `wxPoint` class](https://docs.wxwidgets.org/3.2/classwx_point.html)'s methods and inheritance.
///
/// See [`PointInRust`] documentation for the class usage.
pub trait PointMethods: WxRustMethods {
    /// Returns true if neither of the point components is equal to wxDefaultCoord.
    ///
    /// See [C++ `wxPoint::IsFullySpecified()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_point.html#a1ee077698a3bc36a4132af72a94f0012).
    fn is_fully_specified(&self) -> bool {
        unsafe { ffi::wxPoint_IsFullySpecified(self.as_ptr()) }
    }
    /// Combine this object with another one replacing the uninitialized values.
    ///
    /// See [C++ `wxPoint::SetDefaults()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_point.html#a3283b1248006f81984ac22a81d2d94f6).
    fn set_defaults<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxPoint_SetDefaults(self.as_ptr(), pt)
        }
    }
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator-()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // BLOCKED: fn operator+1()
    // BLOCKED: fn operator-1()
    // BLOCKED: fn operator+2()
    // BLOCKED: fn operator-2()
    // BLOCKED: fn operator+=1()
    // BLOCKED: fn operator-=1()
    // BLOCKED: fn operator/()
    // BLOCKED: fn operator*()
    // BLOCKED: fn operator*1()
    // BLOCKED: fn operator/=()
    // BLOCKED: fn operator*=()
}

// wxPopupTransientWindow
/// This trait represents [C++ `wxPopupTransientWindow` class](https://docs.wxwidgets.org/3.2/classwx_popup_transient_window.html)'s methods and inheritance.
///
/// See [`PopupTransientWindowInRust`] documentation for the class usage.
pub trait PopupTransientWindowMethods: PopupWindowMethods {
    /// Popup the window (this will show it too).
    ///
    /// See [C++ `wxPopupTransientWindow::Popup()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_popup_transient_window.html#ad9a80da9627d9412570f73fa4d047512).
    fn popup<W: WindowMethods>(&self, focus: Option<&W>) {
        unsafe {
            let focus = match focus {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPopupTransientWindow_Popup(self.as_ptr(), focus)
        }
    }
    /// Hide the window.
    ///
    /// See [C++ `wxPopupTransientWindow::Dismiss()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_popup_transient_window.html#a9bf20e3e3ca8640deaba41d96e851b59).
    fn dismiss(&self) {
        unsafe { ffi::wxPopupTransientWindow_Dismiss(self.as_ptr()) }
    }
    /// Called when a mouse is pressed while the popup is shown.
    ///
    /// See [C++ `wxPopupTransientWindow::ProcessLeftDown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_popup_transient_window.html#a393ede764481bcab94d39a2f6fdd31e8).
    fn process_left_down<M: MouseEventMethods>(&self, event: &M) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxPopupTransientWindow_ProcessLeftDown(self.as_ptr(), event)
        }
    }
}

// wxPopupWindow
/// This trait represents [C++ `wxPopupWindow` class](https://docs.wxwidgets.org/3.2/classwx_popup_window.html)'s methods and inheritance.
///
/// See [`PopupWindowInRust`] documentation for the class usage.
pub trait PopupWindowMethods: NonOwnedWindowMethods {
    /// Create method for two-step creation.
    ///
    /// See [C++ `wxPopupWindow::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_popup_window.html#a2b82f62c61b126fe4831a8039fe72f09).
    fn create_int<W: WindowMethods>(&self, parent: Option<&W>, flags: c_int) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPopupWindow_Create(self.as_ptr(), parent, flags)
        }
    }
    /// Move the popup window to the right position, i.e. such that it is entirely visible.
    ///
    /// See [C++ `wxPopupWindow::Position()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_popup_window.html#af39e2a99cb6323a429833b9550819130).
    fn position<P: PointMethods, S: SizeMethods>(&self, pt_origin: &P, size_popup: &S) {
        unsafe {
            let pt_origin = pt_origin.as_ptr();
            let size_popup = size_popup.as_ptr();
            ffi::wxPopupWindow_Position(self.as_ptr(), pt_origin, size_popup)
        }
    }
}

// wxPreferencesEditor
/// This trait represents [C++ `wxPreferencesEditor` class](https://docs.wxwidgets.org/3.2/classwx_preferences_editor.html)'s methods and inheritance.
///
/// See [`PreferencesEditorInRust`] documentation for the class usage.
pub trait PreferencesEditorMethods: WxRustMethods {
    // DTOR: fn ~wxPreferencesEditor()
    /// Add a new page to the editor.
    ///
    /// See [C++ `wxPreferencesEditor::AddPage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_preferences_editor.html#a3f5102fccc200964eb2ea4890b334c29).
    fn add_page<P: PreferencesPageMethods>(&self, page: Option<&P>) {
        unsafe {
            let page = match page {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPreferencesEditor_AddPage(self.as_ptr(), page)
        }
    }
    /// Show the preferences dialog or bring it to the top if it's already shown.
    ///
    /// See [C++ `wxPreferencesEditor::Show()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_preferences_editor.html#a626cdcd775e6e1150901d64b63c34819).
    fn show<W: WindowMethods>(&self, parent: Option<&W>) {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPreferencesEditor_Show(self.as_ptr(), parent)
        }
    }
    /// Hide the currently shown dialog, if any.
    ///
    /// See [C++ `wxPreferencesEditor::Dismiss()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_preferences_editor.html#acdecd2b18a9d3171e04cdc561848635b).
    fn dismiss(&self) {
        unsafe { ffi::wxPreferencesEditor_Dismiss(self.as_ptr()) }
    }
    /// Returns whether changes to values in preferences pages should be applied immediately or only when the user clicks the OK button.
    ///
    /// See [C++ `wxPreferencesEditor::ShouldApplyChangesImmediately()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_preferences_editor.html#a7345475a70fab9c99ff3ea6984c8c574).
    fn should_apply_changes_immediately() -> bool {
        unsafe { ffi::wxPreferencesEditor_ShouldApplyChangesImmediately() }
    }
    /// Returns whether the preferences dialog is shown modally.
    ///
    /// See [C++ `wxPreferencesEditor::ShownModally()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_preferences_editor.html#a744201ef6aa0962c06d4d733cc3cada4).
    fn shown_modally() -> bool {
        unsafe { ffi::wxPreferencesEditor_ShownModally() }
    }
}

// wxPreferencesPage
/// This trait represents [C++ `wxPreferencesPage` class](https://docs.wxwidgets.org/3.2/classwx_preferences_page.html)'s methods and inheritance.
///
/// See [`PreferencesPageInRust`] documentation for the class usage.
pub trait PreferencesPageMethods: WxRustMethods {
    // DTOR: fn ~wxPreferencesPage()
    /// Return name of the page.
    ///
    /// See [C++ `wxPreferencesPage::GetName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_preferences_page.html#aaf3db462208f700fff0fd99c66afd8d7).
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxPreferencesPage_GetName(self.as_ptr())).into() }
    }
    /// Return the icon to be used for the page on some platforms.
    ///
    /// See [C++ `wxPreferencesPage::GetIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_preferences_page.html#a90e5b4b5c30b32be39b470f573792f52).
    fn get_icon(&self) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxPreferencesPage_GetIcon(self.as_ptr())) }
    }
    ///
    /// See [C++ `wxPreferencesPage::GetLargeIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_preferences_page.html#a05880595e8507b60eaea11d5addc71e6).
    fn get_large_icon(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxPreferencesPage_GetLargeIcon(self.as_ptr())) }
    }
    /// Create a window for this page.
    ///
    /// See [C++ `wxPreferencesPage::CreateWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_preferences_page.html#a5d401e0e131e088e43cf378248ea494d).
    fn create_window<W: WindowMethods>(&self, parent: Option<&W>) -> WeakRef<Window> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<Window>::from(ffi::wxPreferencesPage_CreateWindow(self.as_ptr(), parent))
        }
    }
}

// wxPropertySheetDialog
/// This trait represents [C++ `wxPropertySheetDialog` class](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html)'s methods and inheritance.
///
/// See [`PropertySheetDialogInRust`] documentation for the class usage.
pub trait PropertySheetDialogMethods: DialogMethods {
    /// Override this if you wish to add the book control in a way different from the standard way (for example, using different spacing).
    ///
    /// See [C++ `wxPropertySheetDialog::AddBookCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html#a7827bb55364f6daa75fca9775cc0e6a8).
    fn add_book_ctrl<S: SizerMethods>(&self, sizer: Option<&S>) {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPropertySheetDialog_AddBookCtrl(self.as_ptr(), sizer)
        }
    }
    /// Override this if you wish to create a different kind of book control; by default, the value passed to SetSheetStyle() is used to determine the control.
    ///
    /// See [C++ `wxPropertySheetDialog::CreateBookCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html#adf8a676f05979629dfa6c8ea1a171057).
    fn create_book_ctrl(&self) -> WeakRef<BookCtrlBase> {
        unsafe {
            WeakRef::<BookCtrlBase>::from(ffi::wxPropertySheetDialog_CreateBookCtrl(self.as_ptr()))
        }
    }
    /// Call this to create the buttons for the dialog.
    ///
    /// See [C++ `wxPropertySheetDialog::CreateButtons()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html#a74a83482c1a6aa508bcd8240e0461ac9).
    fn create_buttons(&self, flags: c_int) {
        unsafe { ffi::wxPropertySheetDialog_CreateButtons(self.as_ptr(), flags) }
    }
    /// Returns the book control that will contain your settings pages.
    ///
    /// See [C++ `wxPropertySheetDialog::GetBookCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html#afe009dc2b88cd2125e8f089f46735f6d).
    fn get_book_ctrl(&self) -> WeakRef<BookCtrlBase> {
        unsafe {
            WeakRef::<BookCtrlBase>::from(ffi::wxPropertySheetDialog_GetBookCtrl(self.as_ptr()))
        }
    }
    /// Returns the inner sizer that contains the book control and button sizer.
    ///
    /// See [C++ `wxPropertySheetDialog::GetInnerSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html#a4868b8dcd1786c5bc00ce26a47c19573).
    fn get_inner_sizer(&self) -> Option<SizerInRust<false>> {
        unsafe { Sizer::option_from(ffi::wxPropertySheetDialog_GetInnerSizer(self.as_ptr())) }
    }
    /// Set the inner sizer that contains the book control and button sizer.
    ///
    /// See [C++ `wxPropertySheetDialog::SetInnerSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html#a2f200c9958b36f24009299a014ac5c8d).
    fn set_inner_sizer<S: SizerMethods>(&self, sizer: Option<&S>) {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPropertySheetDialog_SetInnerSizer(self.as_ptr(), sizer)
        }
    }
    /// Returns the sheet style.
    ///
    /// See [C++ `wxPropertySheetDialog::GetSheetStyle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html#ad341f19b19cb821420eb1e5a2c5a17e5).
    fn get_sheet_style(&self) -> c_long {
        unsafe { ffi::wxPropertySheetDialog_GetSheetStyle(self.as_ptr()) }
    }
    /// Call this to lay out the dialog.
    ///
    /// See [C++ `wxPropertySheetDialog::LayoutDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html#a9a15ba6b238d9ef84eb1882108ae72c6).
    fn layout_dialog(&self, centre_flags: c_int) {
        unsafe { ffi::wxPropertySheetDialog_LayoutDialog(self.as_ptr(), centre_flags) }
    }
    /// Sets the book control used for the dialog.
    ///
    /// See [C++ `wxPropertySheetDialog::SetBookCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html#afe6349786b9de62856fcfc555b318c83).
    fn set_book_ctrl<B: BookCtrlBaseMethods>(&self, book_ctrl: Option<&B>) {
        unsafe {
            let book_ctrl = match book_ctrl {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPropertySheetDialog_SetBookCtrl(self.as_ptr(), book_ctrl)
        }
    }
    /// You can customize the look and feel of the dialog by setting the sheet style.
    ///
    /// See [C++ `wxPropertySheetDialog::SetSheetStyle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html#acdfd10705dfdc4517150e5e9e70b9f9a).
    fn set_sheet_style(&self, style: c_long) {
        unsafe { ffi::wxPropertySheetDialog_SetSheetStyle(self.as_ptr(), style) }
    }
    /// Set the border around the whole dialog.
    ///
    /// See [C++ `wxPropertySheetDialog::SetSheetOuterBorder()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html#a84449186299052ea54f360ed6a4851ac).
    fn set_sheet_outer_border(&self, border: c_int) {
        unsafe { ffi::wxPropertySheetDialog_SetSheetOuterBorder(self.as_ptr(), border) }
    }
    /// Returns the border around the whole dialog.
    ///
    /// See [C++ `wxPropertySheetDialog::GetSheetOuterBorder()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html#ab2ff7b4e9dd9e34bbce58da70dbf1be3).
    fn get_sheet_outer_border(&self) -> c_int {
        unsafe { ffi::wxPropertySheetDialog_GetSheetOuterBorder(self.as_ptr()) }
    }
    /// Set the border around the book control only.
    ///
    /// See [C++ `wxPropertySheetDialog::SetSheetInnerBorder()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html#a6d02a5ca161438831c2c194d43b9ea52).
    fn set_sheet_inner_border(&self, border: c_int) {
        unsafe { ffi::wxPropertySheetDialog_SetSheetInnerBorder(self.as_ptr(), border) }
    }
    /// Returns the border around the book control only.
    ///
    /// See [C++ `wxPropertySheetDialog::GetSheetInnerBorder()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html#a09e07e03fe3bc8f58e1e67356856cbd8).
    fn get_sheet_inner_border(&self) -> c_int {
        unsafe { ffi::wxPropertySheetDialog_GetSheetInnerBorder(self.as_ptr()) }
    }
}
