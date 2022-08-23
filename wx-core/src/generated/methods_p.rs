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
    fn create(
        &self,
        n: c_int,
        red: *const c_void,
        green: *const c_void,
        blue: *const c_void,
    ) -> bool {
        unsafe { ffi::wxPalette_Create(self.as_ptr(), n, red, green, blue) }
    }
    fn get_colours_count(&self) -> c_int {
        unsafe { ffi::wxPalette_GetColoursCount(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetPixel()
    fn get_rgb(
        &self,
        pixel: c_int,
        red: *mut c_void,
        green: *mut c_void,
        blue: *mut c_void,
    ) -> bool {
        unsafe { ffi::wxPalette_GetRGB(self.as_ptr(), pixel, red, green, blue) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxPalette_IsOk(self.as_ptr()) }
    }
    // BLOCKED: fn operator=()
}

// wxPanel
pub trait PanelMethods: WindowMethods {
    // DTOR: fn ~wxPanel()
    fn on_sys_colour_changed<S: SysColourChangedEventMethods>(&self, event: &S) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxPanel_OnSysColourChanged(self.as_ptr(), event)
        }
    }
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
    fn get_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxPen_GetColour(self.as_ptr())) }
    }
    fn get_dashes(&self, dashes: *mut c_void) -> c_int {
        unsafe { ffi::wxPen_GetDashes(self.as_ptr(), dashes) }
    }
    // NOT_SUPPORTED: fn GetJoin()
    fn get_stipple(&self) -> Option<BitmapIsOwned<false>> {
        unsafe { Bitmap::option_from(ffi::wxPen_GetStipple(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetStyle()
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxPen_GetWidth(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxPen_IsOk(self.as_ptr()) }
    }
    fn is_non_transparent(&self) -> bool {
        unsafe { ffi::wxPen_IsNonTransparent(self.as_ptr()) }
    }
    fn is_transparent(&self) -> bool {
        unsafe { ffi::wxPen_IsTransparent(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetCap()
    // NOT_SUPPORTED: fn SetQuality()
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxPen_SetColour(self.as_ptr(), colour)
        }
    }
    // NOT_SUPPORTED: fn SetColour1()
    fn set_dashes(&self, n: c_int, dash: *const c_void) {
        unsafe { ffi::wxPen_SetDashes(self.as_ptr(), n, dash) }
    }
    // NOT_SUPPORTED: fn SetJoin()
    fn set_stipple<B: BitmapMethods>(&self, stipple: &B) {
        unsafe {
            let stipple = stipple.as_ptr();
            ffi::wxPen_SetStipple(self.as_ptr(), stipple)
        }
    }
    // NOT_SUPPORTED: fn SetStyle()
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
    fn set<P: PersistenceManagerMethods>(manager: &P) {
        unsafe {
            let manager = manager.as_ptr();
            ffi::wxPersistenceManager_Set(manager)
        }
    }
    fn get() -> PersistenceManagerIsOwned<false> {
        unsafe { PersistenceManagerIsOwned::from_ptr(ffi::wxPersistenceManager_Get()) }
    }
    fn disable_saving(&self) {
        unsafe { ffi::wxPersistenceManager_DisableSaving(self.as_ptr()) }
    }
    fn disable_restoring(&self) {
        unsafe { ffi::wxPersistenceManager_DisableRestoring(self.as_ptr()) }
    }
    // BLOCKED: fn Register()
    fn register(&self, obj: *mut c_void, po: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxPersistenceManager_Register1(self.as_ptr(), obj, po) }
    }
    fn find(&self, obj: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxPersistenceManager_Find(self.as_ptr(), obj) }
    }
    fn unregister(&self, obj: *mut c_void) {
        unsafe { ffi::wxPersistenceManager_Unregister(self.as_ptr(), obj) }
    }
    fn save(&self, obj: *mut c_void) {
        unsafe { ffi::wxPersistenceManager_Save(self.as_ptr(), obj) }
    }
    fn restore(&self, obj: *mut c_void) -> bool {
        unsafe { ffi::wxPersistenceManager_Restore(self.as_ptr(), obj) }
    }
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
    fn get_internal_margin(&self) -> c_int {
        unsafe { ffi::wxPickerBase_GetInternalMargin(self.as_ptr()) }
    }
    fn get_picker_ctrl_proportion(&self) -> c_int {
        unsafe { ffi::wxPickerBase_GetPickerCtrlProportion(self.as_ptr()) }
    }
    fn get_text_ctrl(&self) -> WeakRef<TextCtrl> {
        unsafe { WeakRef::<TextCtrl>::from_ptr(ffi::wxPickerBase_GetTextCtrl(self.as_ptr())) }
    }
    fn get_picker_ctrl(&self) -> WeakRef<Control> {
        unsafe { WeakRef::<Control>::from_ptr(ffi::wxPickerBase_GetPickerCtrl(self.as_ptr())) }
    }
    fn get_text_ctrl_proportion(&self) -> c_int {
        unsafe { ffi::wxPickerBase_GetTextCtrlProportion(self.as_ptr()) }
    }
    fn has_text_ctrl(&self) -> bool {
        unsafe { ffi::wxPickerBase_HasTextCtrl(self.as_ptr()) }
    }
    fn is_picker_ctrl_growable(&self) -> bool {
        unsafe { ffi::wxPickerBase_IsPickerCtrlGrowable(self.as_ptr()) }
    }
    fn is_text_ctrl_growable(&self) -> bool {
        unsafe { ffi::wxPickerBase_IsTextCtrlGrowable(self.as_ptr()) }
    }
    fn set_internal_margin(&self, margin: c_int) {
        unsafe { ffi::wxPickerBase_SetInternalMargin(self.as_ptr(), margin) }
    }
    fn set_picker_ctrl_growable(&self, grow: bool) {
        unsafe { ffi::wxPickerBase_SetPickerCtrlGrowable(self.as_ptr(), grow) }
    }
    fn set_picker_ctrl_proportion(&self, prop: c_int) {
        unsafe { ffi::wxPickerBase_SetPickerCtrlProportion(self.as_ptr(), prop) }
    }
    fn set_text_ctrl_growable(&self, grow: bool) {
        unsafe { ffi::wxPickerBase_SetTextCtrlGrowable(self.as_ptr(), grow) }
    }
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
    fn is_fully_specified(&self) -> bool {
        unsafe { ffi::wxPoint_IsFullySpecified(self.as_ptr()) }
    }
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
    fn popup<W: WindowMethods>(&self, focus: Option<&W>) {
        unsafe {
            let focus = match focus {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPopupTransientWindow_Popup(self.as_ptr(), focus)
        }
    }
    fn dismiss(&self) {
        unsafe { ffi::wxPopupTransientWindow_Dismiss(self.as_ptr()) }
    }
    fn process_left_down<M: MouseEventMethods>(&self, event: &M) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxPopupTransientWindow_ProcessLeftDown(self.as_ptr(), event)
        }
    }
}

// wxPopupWindow
pub trait PopupWindowMethods: NonOwnedWindowMethods {
    fn create_int<W: WindowMethods>(&self, parent: Option<&W>, flags: c_int) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPopupWindow_Create(self.as_ptr(), parent, flags)
        }
    }
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
    fn add_page<P: PreferencesPageMethods>(&self, page: Option<&P>) {
        unsafe {
            let page = match page {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPreferencesEditor_AddPage(self.as_ptr(), page)
        }
    }
    fn show<W: WindowMethods>(&self, parent: Option<&W>) {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPreferencesEditor_Show(self.as_ptr(), parent)
        }
    }
    fn dismiss(&self) {
        unsafe { ffi::wxPreferencesEditor_Dismiss(self.as_ptr()) }
    }
    fn should_apply_changes_immediately() -> bool {
        unsafe { ffi::wxPreferencesEditor_ShouldApplyChangesImmediately() }
    }
    fn shown_modally() -> bool {
        unsafe { ffi::wxPreferencesEditor_ShownModally() }
    }
}

// wxPreferencesPage
pub trait PreferencesPageMethods: WxRustMethods {
    // DTOR: fn ~wxPreferencesPage()
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxPreferencesPage_GetName(self.as_ptr())).into() }
    }
    fn get_icon(&self) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxPreferencesPage_GetIcon(self.as_ptr())) }
    }
    fn get_large_icon(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxPreferencesPage_GetLargeIcon(self.as_ptr())) }
    }
    fn create_window<W: WindowMethods>(&self, parent: Option<&W>) -> WeakRef<Window> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<Window>::from_ptr(ffi::wxPreferencesPage_CreateWindow(self.as_ptr(), parent))
        }
    }
}

// wxPropertySheetDialog
pub trait PropertySheetDialogMethods: DialogMethods {
    fn add_book_ctrl<S: SizerMethods>(&self, sizer: Option<&S>) {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPropertySheetDialog_AddBookCtrl(self.as_ptr(), sizer)
        }
    }
    fn create_book_ctrl(&self) -> WeakRef<BookCtrlBase> {
        unsafe {
            WeakRef::<BookCtrlBase>::from_ptr(ffi::wxPropertySheetDialog_CreateBookCtrl(
                self.as_ptr(),
            ))
        }
    }
    fn create_buttons(&self, flags: c_int) {
        unsafe { ffi::wxPropertySheetDialog_CreateButtons(self.as_ptr(), flags) }
    }
    fn get_book_ctrl(&self) -> WeakRef<BookCtrlBase> {
        unsafe {
            WeakRef::<BookCtrlBase>::from_ptr(ffi::wxPropertySheetDialog_GetBookCtrl(self.as_ptr()))
        }
    }
    fn get_inner_sizer(&self) -> Option<SizerIsOwned<false>> {
        unsafe { Sizer::option_from(ffi::wxPropertySheetDialog_GetInnerSizer(self.as_ptr())) }
    }
    fn set_inner_sizer<S: SizerMethods>(&self, sizer: Option<&S>) {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPropertySheetDialog_SetInnerSizer(self.as_ptr(), sizer)
        }
    }
    fn get_sheet_style(&self) -> c_long {
        unsafe { ffi::wxPropertySheetDialog_GetSheetStyle(self.as_ptr()) }
    }
    fn layout_dialog(&self, centre_flags: c_int) {
        unsafe { ffi::wxPropertySheetDialog_LayoutDialog(self.as_ptr(), centre_flags) }
    }
    fn set_book_ctrl<B: BookCtrlBaseMethods>(&self, book_ctrl: Option<&B>) {
        unsafe {
            let book_ctrl = match book_ctrl {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPropertySheetDialog_SetBookCtrl(self.as_ptr(), book_ctrl)
        }
    }
    fn set_sheet_style(&self, style: c_long) {
        unsafe { ffi::wxPropertySheetDialog_SetSheetStyle(self.as_ptr(), style) }
    }
    fn set_sheet_outer_border(&self, border: c_int) {
        unsafe { ffi::wxPropertySheetDialog_SetSheetOuterBorder(self.as_ptr(), border) }
    }
    fn get_sheet_outer_border(&self) -> c_int {
        unsafe { ffi::wxPropertySheetDialog_GetSheetOuterBorder(self.as_ptr()) }
    }
    fn set_sheet_inner_border(&self, border: c_int) {
        unsafe { ffi::wxPropertySheetDialog_SetSheetInnerBorder(self.as_ptr(), border) }
    }
    fn get_sheet_inner_border(&self) -> c_int {
        unsafe { ffi::wxPropertySheetDialog_GetSheetInnerBorder(self.as_ptr()) }
    }
}
