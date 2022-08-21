use super::*;

// wxVListBox
pub trait VListBoxMethods: VScrolledWindowMethods {
    // DTOR: fn ~wxVListBox()
    fn clear(&self) {
        unsafe { ffi::wxVListBox_Clear(self.as_ptr()) }
    }
    fn deselect_all(&self) -> bool {
        unsafe { ffi::wxVListBox_DeselectAll(self.as_ptr()) }
    }
    fn get_first_selected(&self, cookie: *mut c_void) -> c_int {
        unsafe { ffi::wxVListBox_GetFirstSelected(self.as_ptr(), cookie) }
    }
    fn get_item_count(&self) -> usize {
        unsafe { ffi::wxVListBox_GetItemCount(self.as_ptr()) }
    }
    fn get_margins(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxVListBox_GetMargins(self.as_ptr())) }
    }
    fn get_item_rect(&self, item: usize) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxVListBox_GetItemRect(self.as_ptr(), item)) }
    }
    fn get_next_selected(&self, cookie: *mut c_void) -> c_int {
        unsafe { ffi::wxVListBox_GetNextSelected(self.as_ptr(), cookie) }
    }
    fn get_selected_count(&self) -> usize {
        unsafe { ffi::wxVListBox_GetSelectedCount(self.as_ptr()) }
    }
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxVListBox_GetSelection(self.as_ptr()) }
    }
    fn get_selection_background(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxVListBox_GetSelectionBackground(self.as_ptr())) }
    }
    fn has_multiple_selection(&self) -> bool {
        unsafe { ffi::wxVListBox_HasMultipleSelection(self.as_ptr()) }
    }
    fn is_current(&self, item: usize) -> bool {
        unsafe { ffi::wxVListBox_IsCurrent(self.as_ptr(), item) }
    }
    fn is_selected(&self, item: usize) -> bool {
        unsafe { ffi::wxVListBox_IsSelected(self.as_ptr(), item) }
    }
    fn select(&self, item: usize, select: bool) -> bool {
        unsafe { ffi::wxVListBox_Select(self.as_ptr(), item, select) }
    }
    fn select_all(&self) -> bool {
        unsafe { ffi::wxVListBox_SelectAll(self.as_ptr()) }
    }
    fn select_range(&self, from: usize, to: usize) -> bool {
        unsafe { ffi::wxVListBox_SelectRange(self.as_ptr(), from, to) }
    }
    fn set_item_count(&self, count: usize) {
        unsafe { ffi::wxVListBox_SetItemCount(self.as_ptr(), count) }
    }
    fn set_margins_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxVListBox_SetMargins(self.as_ptr(), pt)
        }
    }
    fn set_margins_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxVListBox_SetMargins1(self.as_ptr(), x, y) }
    }
    fn set_selection(&self, selection: c_int) {
        unsafe { ffi::wxVListBox_SetSelection(self.as_ptr(), selection) }
    }
    fn set_selection_background<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxVListBox_SetSelectionBackground(self.as_ptr(), col)
        }
    }
    fn toggle(&self, item: usize) {
        unsafe { ffi::wxVListBox_Toggle(self.as_ptr(), item) }
    }
}

// wxVScrolledWindow
pub trait VScrolledWindowMethods: PanelMethods {}

// wxValidator
pub trait ValidatorMethods: EvtHandlerMethods {
    // DTOR: fn ~wxValidator()
    fn clone(&self) -> Object {
        unsafe { Object::from_ptr(ffi::wxValidator_Clone(self.as_ptr())) }
    }
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxValidator_GetWindow(self.as_ptr())) }
    }
    fn set_window<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxValidator_SetWindow(self.as_ptr(), window)
        }
    }
    fn transfer_from_window(&self) -> bool {
        unsafe { ffi::wxValidator_TransferFromWindow(self.as_ptr()) }
    }
    fn transfer_to_window(&self) -> bool {
        unsafe { ffi::wxValidator_TransferToWindow(self.as_ptr()) }
    }
    fn validate<W: WindowMethods>(&self, parent: Option<&W>) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxValidator_Validate(self.as_ptr(), parent)
        }
    }
    fn suppress_bell_on_error(suppress: bool) {
        unsafe { ffi::wxValidator_SuppressBellOnError(suppress) }
    }
    fn is_silent() -> bool {
        unsafe { ffi::wxValidator_IsSilent() }
    }
}

// wxView
pub trait ViewMethods: EvtHandlerMethods {
    // DTOR: fn ~wxView()
    fn activate(&self, activate: bool) {
        unsafe { ffi::wxView_Activate(self.as_ptr(), activate) }
    }
    fn close(&self, delete_window: bool) -> bool {
        unsafe { ffi::wxView_Close(self.as_ptr(), delete_window) }
    }
    fn get_document(&self) -> *mut c_void {
        unsafe { ffi::wxView_GetDocument(self.as_ptr()) }
    }
    fn get_document_manager(&self) -> *mut c_void {
        unsafe { ffi::wxView_GetDocumentManager(self.as_ptr()) }
    }
    fn get_frame(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxView_GetFrame(self.as_ptr())) }
    }
    fn get_view_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxView_GetViewName(self.as_ptr())).into() }
    }
    fn on_activate_view<V: ViewMethods, V2: ViewMethods>(
        &self,
        activate: bool,
        active_view: Option<&V>,
        deactive_view: Option<&V2>,
    ) {
        unsafe {
            let active_view = match active_view {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let deactive_view = match deactive_view {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxView_OnActivateView(self.as_ptr(), activate, active_view, deactive_view)
        }
    }
    fn on_change_filename(&self) {
        unsafe { ffi::wxView_OnChangeFilename(self.as_ptr()) }
    }
    fn on_close(&self, delete_window: bool) -> bool {
        unsafe { ffi::wxView_OnClose(self.as_ptr(), delete_window) }
    }
    fn on_closing_document(&self) {
        unsafe { ffi::wxView_OnClosingDocument(self.as_ptr()) }
    }
    fn on_create(&self, doc: *mut c_void, flags: c_long) -> bool {
        unsafe { ffi::wxView_OnCreate(self.as_ptr(), doc, flags) }
    }
    fn on_create_printout(&self) -> *mut c_void {
        unsafe { ffi::wxView_OnCreatePrintout(self.as_ptr()) }
    }
    fn on_draw<D: DCMethods>(&self, dc: Option<&D>) {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxView_OnDraw(self.as_ptr(), dc)
        }
    }
    fn on_update<V: ViewMethods, O: ObjectMethods>(&self, sender: Option<&V>, hint: Option<&O>) {
        unsafe {
            let sender = match sender {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let hint = match hint {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxView_OnUpdate(self.as_ptr(), sender, hint)
        }
    }
    fn set_document(&self, doc: *mut c_void) {
        unsafe { ffi::wxView_SetDocument(self.as_ptr(), doc) }
    }
    fn set_frame<W: WindowMethods>(&self, frame: Option<&W>) {
        unsafe {
            let frame = match frame {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxView_SetFrame(self.as_ptr(), frame)
        }
    }
    fn set_view_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxView_SetViewName(self.as_ptr(), name)
        }
    }
}
