use super::*;

// wxPCXHandler
pub trait PCXHandlerMethods: ImageHandlerMethods {}

// wxPNGHandler
pub trait PNGHandlerMethods: ImageHandlerMethods {}

// wxPNMHandler
pub trait PNMHandlerMethods: ImageHandlerMethods {}

// wxPaintDC
pub trait PaintDCMethods: ClientDCMethods {}

// wxPaintEvent
pub trait PaintEventMethods: EventMethods {}

// wxPalette
pub trait PaletteMethods: GDIObjectMethods {
    // DTOR: fn ~wxPalette()
    /// Creates a palette from arrays of size n, one for each red, blue or green component.
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
    fn get_colours_count(&self) -> c_int {
        unsafe { ffi::wxPalette_GetColoursCount(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetPixel()
    /// Returns RGB values for a given palette index.
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
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxPalette_IsOk(self.as_ptr()) }
    }
    // BLOCKED: fn operator=()
}

// wxPanel
pub trait PanelMethods: WindowMethods {
    // DTOR: fn ~wxPanel()
    /// The default handler for wxEVT_SYS_COLOUR_CHANGED.
    fn on_sys_colour_changed<S: SysColourChangedEventMethods>(&self, event: &S) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxPanel_OnSysColourChanged(self.as_ptr(), event)
        }
    }
    /// In contrast to SetFocus() (see above) this will set the focus to the panel even if there are child windows in the panel.
    fn set_focus_ignoring_children(&self) {
        unsafe { ffi::wxPanel_SetFocusIgnoringChildren(self.as_ptr()) }
    }
}

// wxPasswordEntryDialog
pub trait PasswordEntryDialogMethods: TextEntryDialogMethods {}

// wxPen
pub trait PenMethods: GDIObjectMethods {
    // DTOR: fn ~wxPen()
    // NOT_SUPPORTED: fn GetCap()
    // NOT_SUPPORTED: fn GetQuality()
    /// Returns a reference to the pen colour.
    fn get_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxPen_GetColour(self.as_ptr())) }
    }
    /// Gets an array of dashes (defined as char in X, DWORD under Windows).
    fn get_dashes(&self, dashes: *mut c_void) -> c_int {
        unsafe { ffi::wxPen_GetDashes(self.as_ptr(), dashes) }
    }
    // NOT_SUPPORTED: fn GetJoin()
    /// Gets a pointer to the stipple bitmap.
    fn get_stipple(&self) -> Option<BitmapIsOwned<false>> {
        unsafe { Bitmap::option_from(ffi::wxPen_GetStipple(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetStyle()
    /// Returns the pen width.
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxPen_GetWidth(self.as_ptr()) }
    }
    /// Returns true if the pen is initialised.
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxPen_IsOk(self.as_ptr()) }
    }
    /// Returns true if the pen is a valid non-transparent pen.
    fn is_non_transparent(&self) -> bool {
        unsafe { ffi::wxPen_IsNonTransparent(self.as_ptr()) }
    }
    /// Returns true if the pen is transparent.
    fn is_transparent(&self) -> bool {
        unsafe { ffi::wxPen_IsTransparent(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetCap()
    // NOT_SUPPORTED: fn SetQuality()
    /// The pen's colour is changed to the given colour.
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxPen_SetColour(self.as_ptr(), colour)
        }
    }
    // NOT_SUPPORTED: fn SetColour1()
    /// Associates an array of dash values (defined as char in X, DWORD under Windows) with the pen.
    fn set_dashes(&self, n: c_int, dash: *const c_void) {
        unsafe { ffi::wxPen_SetDashes(self.as_ptr(), n, dash) }
    }
    // NOT_SUPPORTED: fn SetJoin()
    /// Sets the bitmap for stippling.
    fn set_stipple<B: BitmapMethods>(&self, stipple: &B) {
        unsafe {
            let stipple = stipple.as_ptr();
            ffi::wxPen_SetStipple(self.as_ptr(), stipple)
        }
    }
    // NOT_SUPPORTED: fn SetStyle()
    /// Sets the pen width.
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxPen_SetWidth(self.as_ptr(), width) }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
}

// wxPenList
pub trait PenListMethods: WxRustMethods {
    // NOT_SUPPORTED: fn FindOrCreatePen()
}

// wxPersistenceManager
pub trait PersistenceManagerMethods: WxRustMethods {
    /// Set the global persistence manager to use.
    fn set<P: PersistenceManagerMethods>(manager: &P) {
        unsafe {
            let manager = manager.as_ptr();
            ffi::wxPersistenceManager_Set(manager)
        }
    }
    /// Returns the unique persistence manager object.
    fn get() -> PersistenceManagerIsOwned<false> {
        unsafe { PersistenceManagerIsOwned::from_ptr(ffi::wxPersistenceManager_Get()) }
    }
    /// Globally disable saving the persistence object properties.
    fn disable_saving(&self) {
        unsafe { ffi::wxPersistenceManager_DisableSaving(self.as_ptr()) }
    }
    /// Globally disable restoring the persistence object properties.
    fn disable_restoring(&self) {
        unsafe { ffi::wxPersistenceManager_DisableRestoring(self.as_ptr()) }
    }
    // BLOCKED: fn Register()
    /// Register an object with the manager.
    fn register(&self, obj: *mut c_void, po: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxPersistenceManager_Register1(self.as_ptr(), obj, po) }
    }
    /// Check if the object is registered and return the associated wxPersistentObject if it is or NULL otherwise.
    fn find(&self, obj: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxPersistenceManager_Find(self.as_ptr(), obj) }
    }
    /// Unregister the object and delete the associated wxPersistentObject.
    fn unregister(&self, obj: *mut c_void) {
        unsafe { ffi::wxPersistenceManager_Unregister(self.as_ptr(), obj) }
    }
    /// Save the object properties to persistent storage.
    fn save(&self, obj: *mut c_void) {
        unsafe { ffi::wxPersistenceManager_Save(self.as_ptr(), obj) }
    }
    /// Restore the object properties previously saved by Save().
    fn restore(&self, obj: *mut c_void) -> bool {
        unsafe { ffi::wxPersistenceManager_Restore(self.as_ptr(), obj) }
    }
    /// Combines both Save() and Unregister() calls.
    fn save_and_unregister(&self, obj: *mut c_void) {
        unsafe { ffi::wxPersistenceManager_SaveAndUnregister(self.as_ptr(), obj) }
    }
    // BLOCKED: fn RegisterAndRestore()
    fn register_and_restore(&self, obj: *mut c_void, po: *mut c_void) -> bool {
        unsafe { ffi::wxPersistenceManager_RegisterAndRestore1(self.as_ptr(), obj, po) }
    }
}

// wxPickerBase
pub trait PickerBaseMethods: ControlMethods {
    // DTOR: fn ~wxPickerBase()
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
    fn get_internal_margin(&self) -> c_int {
        unsafe { ffi::wxPickerBase_GetInternalMargin(self.as_ptr()) }
    }
    /// Returns the proportion value of the picker.
    fn get_picker_ctrl_proportion(&self) -> c_int {
        unsafe { ffi::wxPickerBase_GetPickerCtrlProportion(self.as_ptr()) }
    }
    /// Returns a pointer to the text control handled by this window or NULL if the wxPB_USE_TEXTCTRL style was not specified when this control was created.
    fn get_text_ctrl(&self) -> WeakRef<TextCtrl> {
        unsafe { WeakRef::<TextCtrl>::from(ffi::wxPickerBase_GetTextCtrl(self.as_ptr())) }
    }
    /// Returns the native implementation of the real picker control.
    fn get_picker_ctrl(&self) -> WeakRef<Control> {
        unsafe { WeakRef::<Control>::from(ffi::wxPickerBase_GetPickerCtrl(self.as_ptr())) }
    }
    /// Returns the proportion value of the text control.
    fn get_text_ctrl_proportion(&self) -> c_int {
        unsafe { ffi::wxPickerBase_GetTextCtrlProportion(self.as_ptr()) }
    }
    /// Returns true if this window has a valid text control (i.e. if the wxPB_USE_TEXTCTRL style was given when creating this control).
    fn has_text_ctrl(&self) -> bool {
        unsafe { ffi::wxPickerBase_HasTextCtrl(self.as_ptr()) }
    }
    /// Returns true if the picker control is growable.
    fn is_picker_ctrl_growable(&self) -> bool {
        unsafe { ffi::wxPickerBase_IsPickerCtrlGrowable(self.as_ptr()) }
    }
    /// Returns true if the text control is growable.
    fn is_text_ctrl_growable(&self) -> bool {
        unsafe { ffi::wxPickerBase_IsTextCtrlGrowable(self.as_ptr()) }
    }
    /// Sets the margin (in pixel) between the picker and the text control.
    fn set_internal_margin(&self, margin: c_int) {
        unsafe { ffi::wxPickerBase_SetInternalMargin(self.as_ptr(), margin) }
    }
    /// Sets the picker control as growable when grow is true.
    fn set_picker_ctrl_growable(&self, grow: bool) {
        unsafe { ffi::wxPickerBase_SetPickerCtrlGrowable(self.as_ptr(), grow) }
    }
    /// Sets the proportion value of the picker.
    fn set_picker_ctrl_proportion(&self, prop: c_int) {
        unsafe { ffi::wxPickerBase_SetPickerCtrlProportion(self.as_ptr(), prop) }
    }
    /// Sets the text control as growable when grow is true.
    fn set_text_ctrl_growable(&self, grow: bool) {
        unsafe { ffi::wxPickerBase_SetTextCtrlGrowable(self.as_ptr(), grow) }
    }
    /// Sets the proportion value of the text control.
    fn set_text_ctrl_proportion(&self, prop: c_int) {
        unsafe { ffi::wxPickerBase_SetTextCtrlProportion(self.as_ptr(), prop) }
    }
    fn set_text_ctrl<T: TextCtrlMethods>(&self, text: Option<&T>) {
        unsafe {
            let text = match text {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPickerBase_SetTextCtrl(self.as_ptr(), text)
        }
    }
    fn set_picker_ctrl<C: ControlMethods>(&self, picker: Option<&C>) {
        unsafe {
            let picker = match picker {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPickerBase_SetPickerCtrl(self.as_ptr(), picker)
        }
    }
    fn update_picker_from_text_ctrl(&self) {
        unsafe { ffi::wxPickerBase_UpdatePickerFromTextCtrl(self.as_ptr()) }
    }
    fn update_text_ctrl_from_picker(&self) {
        unsafe { ffi::wxPickerBase_UpdateTextCtrlFromPicker(self.as_ptr()) }
    }
}

// wxPoint
pub trait PointMethods: WxRustMethods {
    /// Returns true if neither of the point components is equal to wxDefaultCoord.
    fn is_fully_specified(&self) -> bool {
        unsafe { ffi::wxPoint_IsFullySpecified(self.as_ptr()) }
    }
    /// Combine this object with another one replacing the uninitialized values.
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
pub trait PopupTransientWindowMethods: PopupWindowMethods {
    /// Popup the window (this will show it too).
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
    fn dismiss(&self) {
        unsafe { ffi::wxPopupTransientWindow_Dismiss(self.as_ptr()) }
    }
    /// Called when a mouse is pressed while the popup is shown.
    fn process_left_down<M: MouseEventMethods>(&self, event: &M) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxPopupTransientWindow_ProcessLeftDown(self.as_ptr(), event)
        }
    }
}

// wxPopupWindow
pub trait PopupWindowMethods: NonOwnedWindowMethods {
    /// Create method for two-step creation.
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
    fn position<P: PointMethods, S: SizeMethods>(&self, pt_origin: &P, size_popup: &S) {
        unsafe {
            let pt_origin = pt_origin.as_ptr();
            let size_popup = size_popup.as_ptr();
            ffi::wxPopupWindow_Position(self.as_ptr(), pt_origin, size_popup)
        }
    }
}

// wxPreferencesEditor
pub trait PreferencesEditorMethods: WxRustMethods {
    // DTOR: fn ~wxPreferencesEditor()
    /// Add a new page to the editor.
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
    fn dismiss(&self) {
        unsafe { ffi::wxPreferencesEditor_Dismiss(self.as_ptr()) }
    }
    /// Returns whether changes to values in preferences pages should be applied immediately or only when the user clicks the OK button.
    fn should_apply_changes_immediately() -> bool {
        unsafe { ffi::wxPreferencesEditor_ShouldApplyChangesImmediately() }
    }
    /// Returns whether the preferences dialog is shown modally.
    fn shown_modally() -> bool {
        unsafe { ffi::wxPreferencesEditor_ShownModally() }
    }
}

// wxPreferencesPage
pub trait PreferencesPageMethods: WxRustMethods {
    // DTOR: fn ~wxPreferencesPage()
    /// Return name of the page.
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxPreferencesPage_GetName(self.as_ptr())).into() }
    }
    /// Return the icon to be used for the page on some platforms.
    fn get_icon(&self) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxPreferencesPage_GetIcon(self.as_ptr())) }
    }
    fn get_large_icon(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxPreferencesPage_GetLargeIcon(self.as_ptr())) }
    }
    /// Create a window for this page.
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
pub trait PropertySheetDialogMethods: DialogMethods {
    /// Override this if you wish to add the book control in a way different from the standard way (for example, using different spacing).
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
    fn create_book_ctrl(&self) -> WeakRef<BookCtrlBase> {
        unsafe {
            WeakRef::<BookCtrlBase>::from(ffi::wxPropertySheetDialog_CreateBookCtrl(self.as_ptr()))
        }
    }
    /// Call this to create the buttons for the dialog.
    fn create_buttons(&self, flags: c_int) {
        unsafe { ffi::wxPropertySheetDialog_CreateButtons(self.as_ptr(), flags) }
    }
    /// Returns the book control that will contain your settings pages.
    fn get_book_ctrl(&self) -> WeakRef<BookCtrlBase> {
        unsafe {
            WeakRef::<BookCtrlBase>::from(ffi::wxPropertySheetDialog_GetBookCtrl(self.as_ptr()))
        }
    }
    /// Returns the inner sizer that contains the book control and button sizer.
    fn get_inner_sizer(&self) -> Option<SizerIsOwned<false>> {
        unsafe { Sizer::option_from(ffi::wxPropertySheetDialog_GetInnerSizer(self.as_ptr())) }
    }
    /// Set the inner sizer that contains the book control and button sizer.
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
    fn get_sheet_style(&self) -> c_long {
        unsafe { ffi::wxPropertySheetDialog_GetSheetStyle(self.as_ptr()) }
    }
    /// Call this to lay out the dialog.
    fn layout_dialog(&self, centre_flags: c_int) {
        unsafe { ffi::wxPropertySheetDialog_LayoutDialog(self.as_ptr(), centre_flags) }
    }
    /// Sets the book control used for the dialog.
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
    fn set_sheet_style(&self, style: c_long) {
        unsafe { ffi::wxPropertySheetDialog_SetSheetStyle(self.as_ptr(), style) }
    }
    /// Set the border around the whole dialog.
    fn set_sheet_outer_border(&self, border: c_int) {
        unsafe { ffi::wxPropertySheetDialog_SetSheetOuterBorder(self.as_ptr(), border) }
    }
    /// Returns the border around the whole dialog.
    fn get_sheet_outer_border(&self) -> c_int {
        unsafe { ffi::wxPropertySheetDialog_GetSheetOuterBorder(self.as_ptr()) }
    }
    /// Set the border around the book control only.
    fn set_sheet_inner_border(&self, border: c_int) {
        unsafe { ffi::wxPropertySheetDialog_SetSheetInnerBorder(self.as_ptr(), border) }
    }
    /// Returns the border around the book control only.
    fn get_sheet_inner_border(&self) -> c_int {
        unsafe { ffi::wxPropertySheetDialog_GetSheetInnerBorder(self.as_ptr()) }
    }
}
