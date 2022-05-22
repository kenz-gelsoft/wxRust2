use std::os::raw::{c_int, c_long, c_void};

use super::*;
use crate::WeakRef;

pub use wx_base::methods::*;

// wxAnyButton
pub trait AnyButtonMethods: ControlMethods {
    // DTOR: fn ~wxAnyButton()
    fn get_bitmap(&self) -> Bitmap {
        unsafe { BitmapIsOwned(ffi::wxAnyButton_GetBitmap(self.as_ptr())) }
    }
    fn get_bitmap_current(&self) -> Bitmap {
        unsafe { BitmapIsOwned(ffi::wxAnyButton_GetBitmapCurrent(self.as_ptr())) }
    }
    fn get_bitmap_disabled(&self) -> Bitmap {
        unsafe { BitmapIsOwned(ffi::wxAnyButton_GetBitmapDisabled(self.as_ptr())) }
    }
    fn get_bitmap_focus(&self) -> Bitmap {
        unsafe { BitmapIsOwned(ffi::wxAnyButton_GetBitmapFocus(self.as_ptr())) }
    }
    fn get_bitmap_label(&self) -> Bitmap {
        unsafe { BitmapIsOwned(ffi::wxAnyButton_GetBitmapLabel(self.as_ptr())) }
    }
    fn get_bitmap_pressed(&self) -> Bitmap {
        unsafe { BitmapIsOwned(ffi::wxAnyButton_GetBitmapPressed(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn SetBitmap()
    fn set_bitmap_current<B: BitmapMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapCurrent(self.as_ptr(), bitmap)
        }
    }
    fn set_bitmap_disabled<B: BitmapMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapDisabled(self.as_ptr(), bitmap)
        }
    }
    fn set_bitmap_focus<B: BitmapMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapFocus(self.as_ptr(), bitmap)
        }
    }
    fn set_bitmap_label<B: BitmapMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapLabel(self.as_ptr(), bitmap)
        }
    }
    fn set_bitmap_pressed<B: BitmapMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapPressed(self.as_ptr(), bitmap)
        }
    }
    fn get_bitmap_margins(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxAnyButton_GetBitmapMargins(self.as_ptr())) }
    }
    fn set_bitmap_margins_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxAnyButton_SetBitmapMargins(self.as_ptr(), x, y) }
    }
    fn set_bitmap_margins_size<S: SizeMethods>(&self, sz: &S) {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxAnyButton_SetBitmapMargins1(self.as_ptr(), sz)
        }
    }
    // NOT_SUPPORTED: fn SetBitmapPosition()
}

// wxArtProvider
pub trait ArtProviderMethods: ObjectMethods {
    // DTOR: fn ~wxArtProvider()
    fn delete<A: ArtProviderMethods>(provider: Option<&A>) -> bool {
        unsafe {
            let provider = match provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxArtProvider_Delete(provider)
        }
    }
    fn get_bitmap<S: SizeMethods>(id: &str, client: &str, size: &S) -> Bitmap {
        unsafe {
            let id = wx_base::wx_string_from(id);
            let client = wx_base::wx_string_from(client);
            let size = size.as_ptr();
            BitmapIsOwned(ffi::wxArtProvider_GetBitmap(id, client, size))
        }
    }
    // NOT_SUPPORTED: fn GetIcon()
    fn get_native_size_hint(client: &str) -> Size {
        unsafe {
            let client = wx_base::wx_string_from(client);
            SizeIsOwned(ffi::wxArtProvider_GetNativeSizeHint(client))
        }
    }
    fn get_size_hint(client: &str, platform_default: bool) -> Size {
        unsafe {
            let client = wx_base::wx_string_from(client);
            SizeIsOwned(ffi::wxArtProvider_GetSizeHint(client, platform_default))
        }
    }
    // NOT_SUPPORTED: fn GetIconBundle()
    fn has_native_provider() -> bool {
        unsafe { ffi::wxArtProvider_HasNativeProvider() }
    }
    // BLOCKED: fn Insert()
    fn pop() -> bool {
        unsafe { ffi::wxArtProvider_Pop() }
    }
    fn push<A: ArtProviderMethods>(provider: Option<&A>) {
        unsafe {
            let provider = match provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxArtProvider_Push(provider)
        }
    }
    fn push_back<A: ArtProviderMethods>(provider: Option<&A>) {
        unsafe {
            let provider = match provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxArtProvider_PushBack(provider)
        }
    }
    fn remove<A: ArtProviderMethods>(provider: Option<&A>) -> bool {
        unsafe {
            let provider = match provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxArtProvider_Remove(provider)
        }
    }
    fn get_message_box_icon_id(flags: c_int) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxArtProvider_GetMessageBoxIconId(flags)) }
    }
    // NOT_SUPPORTED: fn GetMessageBoxIcon()
}

// wxBitmap
pub trait BitmapMethods: GDIObjectMethods {
    // DTOR: fn ~wxBitmap()
    // NOT_SUPPORTED: fn ConvertToImage()
    fn copy_from_icon(&self, icon: *const c_void) -> bool {
        unsafe { ffi::wxBitmap_CopyFromIcon(self.as_ptr(), icon) }
    }
    fn create_int_int(&self, width: c_int, height: c_int, depth: c_int) -> bool {
        unsafe { ffi::wxBitmap_Create(self.as_ptr(), width, height, depth) }
    }
    fn create_size<S: SizeMethods>(&self, sz: &S, depth: c_int) -> bool {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxBitmap_Create1(self.as_ptr(), sz, depth)
        }
    }
    fn create_int_dc(&self, width: c_int, height: c_int, dc: *const c_void) -> bool {
        unsafe { ffi::wxBitmap_Create2(self.as_ptr(), width, height, dc) }
    }
    fn create_scaled(
        &self,
        width: c_int,
        height: c_int,
        depth: c_int,
        logical_scale: c_double,
    ) -> bool {
        unsafe { ffi::wxBitmap_CreateScaled(self.as_ptr(), width, height, depth, logical_scale) }
    }
    fn get_depth(&self) -> c_int {
        unsafe { ffi::wxBitmap_GetDepth(self.as_ptr()) }
    }
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxBitmap_GetHeight(self.as_ptr()) }
    }
    fn get_mask(&self) -> *mut c_void {
        unsafe { ffi::wxBitmap_GetMask(self.as_ptr()) }
    }
    fn get_palette(&self) -> *mut c_void {
        unsafe { ffi::wxBitmap_GetPalette(self.as_ptr()) }
    }
    fn get_sub_bitmap<R: RectMethods>(&self, rect: &R) -> Bitmap {
        unsafe {
            let rect = rect.as_ptr();
            BitmapIsOwned(ffi::wxBitmap_GetSubBitmap(self.as_ptr(), rect))
        }
    }
    fn get_size(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxBitmap_GetSize(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn ConvertToDisabled()
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxBitmap_GetWidth(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxBitmap_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn LoadFile()
    // NOT_SUPPORTED: fn SaveFile()
    fn set_depth(&self, depth: c_int) {
        unsafe { ffi::wxBitmap_SetDepth(self.as_ptr(), depth) }
    }
    fn set_height(&self, height: c_int) {
        unsafe { ffi::wxBitmap_SetHeight(self.as_ptr(), height) }
    }
    fn set_mask(&self, mask: *mut c_void) {
        unsafe { ffi::wxBitmap_SetMask(self.as_ptr(), mask) }
    }
    fn set_palette(&self, palette: *const c_void) {
        unsafe { ffi::wxBitmap_SetPalette(self.as_ptr(), palette) }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxBitmap_SetWidth(self.as_ptr(), width) }
    }
    fn add_handler(handler: *mut c_void) {
        unsafe { ffi::wxBitmap_AddHandler(handler) }
    }
    fn clean_up_handlers() {
        unsafe { ffi::wxBitmap_CleanUpHandlers() }
    }
    fn find_handler(name: &str) -> *mut c_void {
        unsafe {
            let name = wx_base::wx_string_from(name);
            ffi::wxBitmap_FindHandler(name)
        }
    }
    // NOT_SUPPORTED: fn FindHandler1()
    // NOT_SUPPORTED: fn FindHandler2()
    // BLOCKED: fn GetHandlers()
    fn init_standard_handlers() {
        unsafe { ffi::wxBitmap_InitStandardHandlers() }
    }
    fn insert_handler(handler: *mut c_void) {
        unsafe { ffi::wxBitmap_InsertHandler(handler) }
    }
    fn new_from_png_data(data: *const c_void, size: usize) -> Bitmap {
        unsafe { BitmapIsOwned(ffi::wxBitmap_NewFromPNGData(data, size)) }
    }
    fn remove_handler(name: &str) -> bool {
        unsafe {
            let name = wx_base::wx_string_from(name);
            ffi::wxBitmap_RemoveHandler(name)
        }
    }
}

// wxBookCtrlBase
pub trait BookCtrlBaseMethods: ControlMethods {
    fn get_page_image(&self, n_page: usize) -> c_int {
        unsafe { ffi::wxBookCtrlBase_GetPageImage(self.as_ptr(), n_page) }
    }
    fn set_page_image(&self, page: usize, image: c_int) -> bool {
        unsafe { ffi::wxBookCtrlBase_SetPageImage(self.as_ptr(), page, image) }
    }
    fn get_page_text(&self, n_page: usize) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxBookCtrlBase_GetPageText(self.as_ptr(), n_page)) }
    }
    fn set_page_text(&self, page: usize, text: &str) -> bool {
        unsafe {
            let text = wx_base::wx_string_from(text);
            ffi::wxBookCtrlBase_SetPageText(self.as_ptr(), page, text)
        }
    }
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxBookCtrlBase_GetSelection(self.as_ptr()) }
    }
    fn get_current_page(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxBookCtrlBase_GetCurrentPage(self.as_ptr())) }
    }
    fn set_selection(&self, page: usize) -> c_int {
        unsafe { ffi::wxBookCtrlBase_SetSelection(self.as_ptr(), page) }
    }
    fn advance_selection(&self, forward: bool) {
        unsafe { ffi::wxBookCtrlBase_AdvanceSelection(self.as_ptr(), forward) }
    }
    fn change_selection(&self, page: usize) -> c_int {
        unsafe { ffi::wxBookCtrlBase_ChangeSelection(self.as_ptr(), page) }
    }
    fn find_page<W: WindowMethods>(&self, page: Option<&W>) -> c_int {
        unsafe {
            let page = match page {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxBookCtrlBase_FindPage(self.as_ptr(), page)
        }
    }
    fn set_page_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxBookCtrlBase_SetPageSize(self.as_ptr(), size)
        }
    }
    fn hit_test<P: PointMethods>(&self, pt: &P, flags: *mut c_void) -> c_int {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxBookCtrlBase_HitTest(self.as_ptr(), pt, flags)
        }
    }
    fn add_page<W: WindowMethods>(
        &self,
        page: Option<&W>,
        text: &str,
        select: bool,
        image_id: c_int,
    ) -> bool {
        unsafe {
            let page = match page {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let text = wx_base::wx_string_from(text);
            ffi::wxBookCtrlBase_AddPage(self.as_ptr(), page, text, select, image_id)
        }
    }
    fn delete_all_pages(&self) -> bool {
        unsafe { ffi::wxBookCtrlBase_DeleteAllPages(self.as_ptr()) }
    }
    fn delete_page(&self, page: usize) -> bool {
        unsafe { ffi::wxBookCtrlBase_DeletePage(self.as_ptr(), page) }
    }
    fn insert_page<W: WindowMethods>(
        &self,
        index: usize,
        page: Option<&W>,
        text: &str,
        select: bool,
        image_id: c_int,
    ) -> bool {
        unsafe {
            let page = match page {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let text = wx_base::wx_string_from(text);
            ffi::wxBookCtrlBase_InsertPage(self.as_ptr(), index, page, text, select, image_id)
        }
    }
    fn remove_page(&self, page: usize) -> bool {
        unsafe { ffi::wxBookCtrlBase_RemovePage(self.as_ptr(), page) }
    }
    fn get_page_count(&self) -> usize {
        unsafe { ffi::wxBookCtrlBase_GetPageCount(self.as_ptr()) }
    }
    fn get_page(&self, page: usize) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxBookCtrlBase_GetPage(self.as_ptr(), page)) }
    }
}

// wxBookCtrlEvent
pub trait BookCtrlEventMethods: NotifyEventMethods {
    fn get_old_selection(&self) -> c_int {
        unsafe { ffi::wxBookCtrlEvent_GetOldSelection(self.as_ptr()) }
    }
    fn set_old_selection(&self, page: c_int) {
        unsafe { ffi::wxBookCtrlEvent_SetOldSelection(self.as_ptr(), page) }
    }
    fn set_selection(&self, page: c_int) {
        unsafe { ffi::wxBookCtrlEvent_SetSelection(self.as_ptr(), page) }
    }
}

// wxBoxSizer
pub trait BoxSizerMethods: SizerMethods {
    fn get_orientation(&self) -> c_int {
        unsafe { ffi::wxBoxSizer_GetOrientation(self.as_ptr()) }
    }
    fn set_orientation(&self, orient: c_int) {
        unsafe { ffi::wxBoxSizer_SetOrientation(self.as_ptr(), orient) }
    }
}

// wxButton
pub trait ButtonMethods: AnyButtonMethods {
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
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
            let label = wx_base::wx_string_from(label);
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = wx_base::wx_string_from(name);
            ffi::wxButton_Create(
                self.as_ptr(),
                parent,
                id,
                label,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_auth_needed(&self) -> bool {
        unsafe { ffi::wxButton_GetAuthNeeded(self.as_ptr()) }
    }
    fn set_auth_needed(&self, needed: bool) {
        unsafe { ffi::wxButton_SetAuthNeeded(self.as_ptr(), needed) }
    }
    fn set_default(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxButton_SetDefault(self.as_ptr())) }
    }
    fn get_default_size<W: WindowMethods>(win: Option<&W>) -> Size {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizeIsOwned(ffi::wxButton_GetDefaultSize(win))
        }
    }
}

// wxCheckBox
pub trait CheckBoxMethods: ControlMethods {
    // DTOR: fn ~wxCheckBox()
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
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
            let label = wx_base::wx_string_from(label);
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = wx_base::wx_string_from(name);
            ffi::wxCheckBox_Create(
                self.as_ptr(),
                parent,
                id,
                label,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_value(&self) -> bool {
        unsafe { ffi::wxCheckBox_GetValue(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Get3StateValue()
    fn is3_state(&self) -> bool {
        unsafe { ffi::wxCheckBox_Is3State(self.as_ptr()) }
    }
    fn is3rd_state_allowed_for_user(&self) -> bool {
        unsafe { ffi::wxCheckBox_Is3rdStateAllowedForUser(self.as_ptr()) }
    }
    fn is_checked(&self) -> bool {
        unsafe { ffi::wxCheckBox_IsChecked(self.as_ptr()) }
    }
    fn set_value(&self, state: bool) {
        unsafe { ffi::wxCheckBox_SetValue(self.as_ptr(), state) }
    }
    // NOT_SUPPORTED: fn Set3StateValue()
}

// wxColour
pub trait ColourMethods: ObjectMethods {
    // NOT_SUPPORTED: fn Alpha()
    // NOT_SUPPORTED: fn Blue()
    fn get_as_string(&self, flags: c_long) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxColour_GetAsString(self.as_ptr(), flags)) }
    }
    // NOT_SUPPORTED: fn SetRGB()
    // NOT_SUPPORTED: fn SetRGBA()
    // NOT_SUPPORTED: fn GetRGB()
    // NOT_SUPPORTED: fn GetRGBA()
    fn get_luminance(&self) -> c_double {
        unsafe { ffi::wxColour_GetLuminance(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetPixel()
    // NOT_SUPPORTED: fn Green()
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxColour_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Red()
    fn is_solid(&self) -> bool {
        unsafe { ffi::wxColour_IsSolid(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Set()
    // NOT_SUPPORTED: fn Set1()
    fn set(&self, str: &str) -> bool {
        unsafe {
            let str = wx_base::wx_string_from(str);
            ffi::wxColour_Set2(self.as_ptr(), str)
        }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // NOT_SUPPORTED: fn MakeDisabled()
    // BLOCKED: fn ChangeLightness()
    fn make_mono(r: *mut c_void, g: *mut c_void, b: *mut c_void, on: bool) {
        unsafe { ffi::wxColour_MakeMono(r, g, b, on) }
    }
    // NOT_SUPPORTED: fn MakeDisabled1()
    fn make_grey(r: *mut c_void, g: *mut c_void, b: *mut c_void) {
        unsafe { ffi::wxColour_MakeGrey(r, g, b) }
    }
    fn make_grey_double(
        r: *mut c_void,
        g: *mut c_void,
        b: *mut c_void,
        weight_r: c_double,
        weight_g: c_double,
        weight_b: c_double,
    ) {
        unsafe { ffi::wxColour_MakeGrey1(r, g, b, weight_r, weight_g, weight_b) }
    }
    // NOT_SUPPORTED: fn AlphaBlend()
    fn change_lightness(r: *mut c_void, g: *mut c_void, b: *mut c_void, ialpha: c_int) {
        unsafe { ffi::wxColour_ChangeLightness1(r, g, b, ialpha) }
    }
}

// wxCommandEvent
pub trait CommandEventMethods: EventMethods {
    fn get_client_data(&self) -> *mut c_void {
        unsafe { ffi::wxCommandEvent_GetClientData(self.as_ptr()) }
    }
    fn get_client_object(&self) -> *mut c_void {
        unsafe { ffi::wxCommandEvent_GetClientObject(self.as_ptr()) }
    }
    fn get_extra_long(&self) -> c_long {
        unsafe { ffi::wxCommandEvent_GetExtraLong(self.as_ptr()) }
    }
    fn get_int(&self) -> c_int {
        unsafe { ffi::wxCommandEvent_GetInt(self.as_ptr()) }
    }
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxCommandEvent_GetSelection(self.as_ptr()) }
    }
    fn get_string(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxCommandEvent_GetString(self.as_ptr())) }
    }
    fn is_checked(&self) -> bool {
        unsafe { ffi::wxCommandEvent_IsChecked(self.as_ptr()) }
    }
    fn is_selection(&self) -> bool {
        unsafe { ffi::wxCommandEvent_IsSelection(self.as_ptr()) }
    }
    fn set_client_data(&self, client_data: *mut c_void) {
        unsafe { ffi::wxCommandEvent_SetClientData(self.as_ptr(), client_data) }
    }
    fn set_client_object(&self, client_object: *mut c_void) {
        unsafe { ffi::wxCommandEvent_SetClientObject(self.as_ptr(), client_object) }
    }
    fn set_extra_long(&self, extra_long: c_long) {
        unsafe { ffi::wxCommandEvent_SetExtraLong(self.as_ptr(), extra_long) }
    }
    fn set_int(&self, int_command: c_int) {
        unsafe { ffi::wxCommandEvent_SetInt(self.as_ptr(), int_command) }
    }
    fn set_string(&self, string: &str) {
        unsafe {
            let string = wx_base::wx_string_from(string);
            ffi::wxCommandEvent_SetString(self.as_ptr(), string)
        }
    }
}

// wxControl
pub trait ControlMethods: WindowMethods {
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
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
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = wx_base::wx_string_from(name);
            ffi::wxControl_Create(self.as_ptr(), parent, id, pos, size, style, validator, name)
        }
    }
    fn command(&self, event: *mut c_void) {
        unsafe { ffi::wxControl_Command(self.as_ptr(), event) }
    }
    fn get_label_text(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxControl_GetLabelText(self.as_ptr())) }
    }
    fn get_size_from_text_size_int(&self, xlen: c_int, ylen: c_int) -> Size {
        unsafe {
            SizeIsOwned(ffi::wxControl_GetSizeFromTextSize(
                self.as_ptr(),
                xlen,
                ylen,
            ))
        }
    }
    fn get_size_from_text_size_size<S: SizeMethods>(&self, tsize: &S) -> Size {
        unsafe {
            let tsize = tsize.as_ptr();
            SizeIsOwned(ffi::wxControl_GetSizeFromTextSize1(self.as_ptr(), tsize))
        }
    }
    fn get_size_from_text(&self, text: &str) -> Size {
        unsafe {
            let text = wx_base::wx_string_from(text);
            SizeIsOwned(ffi::wxControl_GetSizeFromText(self.as_ptr(), text))
        }
    }
    fn set_label_text(&self, text: &str) {
        unsafe {
            let text = wx_base::wx_string_from(text);
            ffi::wxControl_SetLabelText(self.as_ptr(), text)
        }
    }
    fn set_label_markup(&self, markup: &str) -> bool {
        unsafe {
            let markup = wx_base::wx_string_from(markup);
            ffi::wxControl_SetLabelMarkup(self.as_ptr(), markup)
        }
    }
    fn get_label_text_str(label: &str) -> String {
        unsafe {
            let label = wx_base::wx_string_from(label);
            wx_base::from_wx_string(ffi::wxControl_GetLabelText1(label))
        }
    }
    fn remove_mnemonics(str: &str) -> String {
        unsafe {
            let str = wx_base::wx_string_from(str);
            wx_base::from_wx_string(ffi::wxControl_RemoveMnemonics(str))
        }
    }
    fn escape_mnemonics(text: &str) -> String {
        unsafe {
            let text = wx_base::wx_string_from(text);
            wx_base::from_wx_string(ffi::wxControl_EscapeMnemonics(text))
        }
    }
    fn ellipsize(
        label: &str,
        dc: *const c_void,
        mode: c_int,
        max_width: c_int,
        flags: c_int,
    ) -> String {
        unsafe {
            let label = wx_base::wx_string_from(label);
            wx_base::from_wx_string(ffi::wxControl_Ellipsize(label, dc, mode, max_width, flags))
        }
    }
}

// wxFrame
pub trait FrameMethods: TopLevelWindowMethods {
    // DTOR: fn ~wxFrame()
    fn create_status_bar(
        &self,
        number: c_int,
        style: c_long,
        id: c_int,
        name: &str,
    ) -> *mut c_void {
        unsafe {
            let name = wx_base::wx_string_from(name);
            ffi::wxFrame_CreateStatusBar(self.as_ptr(), number, style, id, name)
        }
    }
    fn create_tool_bar(&self, style: c_long, id: c_int, name: &str) -> WeakRef<ToolBar> {
        unsafe {
            let name = wx_base::wx_string_from(name);
            WeakRef::<ToolBar>::from(ffi::wxFrame_CreateToolBar(self.as_ptr(), style, id, name))
        }
    }
    fn do_give_help(&self, text: &str, show: bool) {
        unsafe {
            let text = wx_base::wx_string_from(text);
            ffi::wxFrame_DoGiveHelp(self.as_ptr(), text, show)
        }
    }
    fn get_menu_bar(&self) -> WeakRef<MenuBar> {
        unsafe { WeakRef::<MenuBar>::from(ffi::wxFrame_GetMenuBar(self.as_ptr())) }
    }
    fn get_status_bar(&self) -> *mut c_void {
        unsafe { ffi::wxFrame_GetStatusBar(self.as_ptr()) }
    }
    fn get_status_bar_pane(&self) -> c_int {
        unsafe { ffi::wxFrame_GetStatusBarPane(self.as_ptr()) }
    }
    fn get_tool_bar(&self) -> WeakRef<ToolBar> {
        unsafe { WeakRef::<ToolBar>::from(ffi::wxFrame_GetToolBar(self.as_ptr())) }
    }
    fn on_create_status_bar(
        &self,
        number: c_int,
        style: c_long,
        id: c_int,
        name: &str,
    ) -> *mut c_void {
        unsafe {
            let name = wx_base::wx_string_from(name);
            ffi::wxFrame_OnCreateStatusBar(self.as_ptr(), number, style, id, name)
        }
    }
    fn on_create_tool_bar(&self, style: c_long, id: c_int, name: &str) -> WeakRef<ToolBar> {
        unsafe {
            let name = wx_base::wx_string_from(name);
            WeakRef::<ToolBar>::from(ffi::wxFrame_OnCreateToolBar(self.as_ptr(), style, id, name))
        }
    }
    fn process_command(&self, id: c_int) -> bool {
        unsafe { ffi::wxFrame_ProcessCommand(self.as_ptr(), id) }
    }
    fn set_menu_bar<M: MenuBarMethods>(&self, menu_bar: Option<&M>) {
        unsafe {
            let menu_bar = match menu_bar {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFrame_SetMenuBar(self.as_ptr(), menu_bar)
        }
    }
    fn set_status_bar(&self, status_bar: *mut c_void) {
        unsafe { ffi::wxFrame_SetStatusBar(self.as_ptr(), status_bar) }
    }
    fn set_status_bar_pane(&self, n: c_int) {
        unsafe { ffi::wxFrame_SetStatusBarPane(self.as_ptr(), n) }
    }
    fn set_status_text(&self, text: &str, number: c_int) {
        unsafe {
            let text = wx_base::wx_string_from(text);
            ffi::wxFrame_SetStatusText(self.as_ptr(), text, number)
        }
    }
    fn set_status_widths(&self, n: c_int, widths_field: *const c_void) {
        unsafe { ffi::wxFrame_SetStatusWidths(self.as_ptr(), n, widths_field) }
    }
    fn set_tool_bar<T: ToolBarMethods>(&self, tool_bar: Option<&T>) {
        unsafe {
            let tool_bar = match tool_bar {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFrame_SetToolBar(self.as_ptr(), tool_bar)
        }
    }
    fn msw_get_task_bar_button(&self) -> *mut c_void {
        unsafe { ffi::wxFrame_MSWGetTaskBarButton(self.as_ptr()) }
    }
    fn push_status_text(&self, text: &str, number: c_int) {
        unsafe {
            let text = wx_base::wx_string_from(text);
            ffi::wxFrame_PushStatusText(self.as_ptr(), text, number)
        }
    }
    fn pop_status_text(&self, number: c_int) {
        unsafe { ffi::wxFrame_PopStatusText(self.as_ptr(), number) }
    }
}

// wxGDIObject
pub trait GDIObjectMethods: ObjectMethods {}

// wxListBox
pub trait ListBoxMethods: ControlMethods {
    // DTOR: fn ~wxListBox()
    // NOT_SUPPORTED: fn Create()
    fn create<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = wx_base::wx_string_from(name);
            ffi::wxListBox_Create1(
                self.as_ptr(),
                parent,
                id,
                pos,
                size,
                choices,
                style,
                validator,
                name,
            )
        }
    }
    fn deselect(&self, n: c_int) {
        unsafe { ffi::wxListBox_Deselect(self.as_ptr(), n) }
    }
    fn set_string_selection_bool(&self, s: &str, select: bool) -> bool {
        unsafe {
            let s = wx_base::wx_string_from(s);
            ffi::wxListBox_SetStringSelection(self.as_ptr(), s, select)
        }
    }
    fn set_string_selection(&self, s: &str) -> bool {
        unsafe {
            let s = wx_base::wx_string_from(s);
            ffi::wxListBox_SetStringSelection1(self.as_ptr(), s)
        }
    }
    fn get_selections(&self, selections: *mut c_void) -> c_int {
        unsafe { ffi::wxListBox_GetSelections(self.as_ptr(), selections) }
    }
    fn hit_test_point<P: PointMethods>(&self, point: &P) -> c_int {
        unsafe {
            let point = point.as_ptr();
            ffi::wxListBox_HitTest(self.as_ptr(), point)
        }
    }
    fn hit_test_int(&self, x: c_int, y: c_int) -> c_int {
        unsafe { ffi::wxListBox_HitTest1(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn InsertItems()
    // NOT_SUPPORTED: fn InsertItems1()
    fn is_selected(&self, n: c_int) -> bool {
        unsafe { ffi::wxListBox_IsSelected(self.as_ptr(), n) }
    }
    fn set_first_item_int(&self, n: c_int) {
        unsafe { ffi::wxListBox_SetFirstItem(self.as_ptr(), n) }
    }
    fn set_first_item_str(&self, string: &str) {
        unsafe {
            let string = wx_base::wx_string_from(string);
            ffi::wxListBox_SetFirstItem1(self.as_ptr(), string)
        }
    }
    fn ensure_visible(&self, n: c_int) {
        unsafe { ffi::wxListBox_EnsureVisible(self.as_ptr(), n) }
    }
    fn is_sorted(&self) -> bool {
        unsafe { ffi::wxListBox_IsSorted(self.as_ptr()) }
    }
    fn get_count_per_page(&self) -> c_int {
        unsafe { ffi::wxListBox_GetCountPerPage(self.as_ptr()) }
    }
    fn get_top_item(&self) -> c_int {
        unsafe { ffi::wxListBox_GetTopItem(self.as_ptr()) }
    }
    // BLOCKED: fn MSWSetTabStops()
}

// wxMenu
pub trait MenuMethods: EvtHandlerMethods {
    // DTOR: fn ~wxMenu()
    fn append_int_str(
        &self,
        id: c_int,
        item: &str,
        help_string: &str,
        kind: c_int,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = wx_base::wx_string_from(item);
            let help_string = wx_base::wx_string_from(help_string);
            MenuItem::option_from(ffi::wxMenu_Append(
                self.as_ptr(),
                id,
                item,
                help_string,
                kind,
            ))
        }
    }
    fn append_int_menu<M: MenuMethods>(
        &self,
        id: c_int,
        item: &str,
        sub_menu: Option<&M>,
        help_string: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = wx_base::wx_string_from(item);
            let sub_menu = match sub_menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let help_string = wx_base::wx_string_from(help_string);
            MenuItem::option_from(ffi::wxMenu_Append1(
                self.as_ptr(),
                id,
                item,
                sub_menu,
                help_string,
            ))
        }
    }
    fn append_menuitem<M: MenuItemMethods>(
        &self,
        menu_item: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let menu_item = match menu_item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenu_Append2(self.as_ptr(), menu_item))
        }
    }
    fn append_check_item(
        &self,
        id: c_int,
        item: &str,
        help: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = wx_base::wx_string_from(item);
            let help = wx_base::wx_string_from(help);
            MenuItem::option_from(ffi::wxMenu_AppendCheckItem(self.as_ptr(), id, item, help))
        }
    }
    fn append_radio_item(
        &self,
        id: c_int,
        item: &str,
        help: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = wx_base::wx_string_from(item);
            let help = wx_base::wx_string_from(help);
            MenuItem::option_from(ffi::wxMenu_AppendRadioItem(self.as_ptr(), id, item, help))
        }
    }
    fn append_separator(&self) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_AppendSeparator(self.as_ptr())) }
    }
    fn append_sub_menu<M: MenuMethods>(
        &self,
        submenu: Option<&M>,
        text: &str,
        help: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let submenu = match submenu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let text = wx_base::wx_string_from(text);
            let help = wx_base::wx_string_from(help);
            MenuItem::option_from(ffi::wxMenu_AppendSubMenu(
                self.as_ptr(),
                submenu,
                text,
                help,
            ))
        }
    }
    fn break_(&self) {
        unsafe { ffi::wxMenu_Break(self.as_ptr()) }
    }
    fn check(&self, id: c_int, check: bool) {
        unsafe { ffi::wxMenu_Check(self.as_ptr(), id, check) }
    }
    fn delete_int(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenu_Delete(self.as_ptr(), id) }
    }
    fn delete_menuitem<M: MenuItemMethods>(&self, item: Option<&M>) -> bool {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_Delete1(self.as_ptr(), item)
        }
    }
    fn destroy_int(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenu_Destroy(self.as_ptr(), id) }
    }
    fn destroy_menuitem<M: MenuItemMethods>(&self, item: Option<&M>) -> bool {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_Destroy1(self.as_ptr(), item)
        }
    }
    fn enable(&self, id: c_int, enable: bool) {
        unsafe { ffi::wxMenu_Enable(self.as_ptr(), id, enable) }
    }
    fn find_child_item(&self, id: c_int, pos: *mut c_void) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_FindChildItem(self.as_ptr(), id, pos)) }
    }
    fn find_item_str(&self, item_string: &str) -> c_int {
        unsafe {
            let item_string = wx_base::wx_string_from(item_string);
            ffi::wxMenu_FindItem(self.as_ptr(), item_string)
        }
    }
    fn find_item_int<M: MenuMethods>(
        &self,
        id: c_int,
        menu: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenu_FindItem1(self.as_ptr(), id, menu))
        }
    }
    fn find_item_by_position(&self, position: usize) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_FindItemByPosition(self.as_ptr(), position)) }
    }
    fn get_help_string(&self, id: c_int) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxMenu_GetHelpString(self.as_ptr(), id)) }
    }
    fn get_label(&self, id: c_int) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxMenu_GetLabel(self.as_ptr(), id)) }
    }
    fn get_label_text(&self, id: c_int) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxMenu_GetLabelText(self.as_ptr(), id)) }
    }
    fn get_menu_item_count(&self) -> usize {
        unsafe { ffi::wxMenu_GetMenuItemCount(self.as_ptr()) }
    }
    // BLOCKED: fn GetMenuItems()
    // BLOCKED: fn GetMenuItems1()
    fn get_title(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxMenu_GetTitle(self.as_ptr())) }
    }
    fn insert_menuitem<M: MenuItemMethods>(
        &self,
        pos: usize,
        menu_item: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let menu_item = match menu_item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenu_Insert(self.as_ptr(), pos, menu_item))
        }
    }
    fn insert_int_str(
        &self,
        pos: usize,
        id: c_int,
        item: &str,
        help_string: &str,
        kind: c_int,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = wx_base::wx_string_from(item);
            let help_string = wx_base::wx_string_from(help_string);
            MenuItem::option_from(ffi::wxMenu_Insert1(
                self.as_ptr(),
                pos,
                id,
                item,
                help_string,
                kind,
            ))
        }
    }
    fn insert_int_menu<M: MenuMethods>(
        &self,
        pos: usize,
        id: c_int,
        text: &str,
        submenu: Option<&M>,
        help: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let text = wx_base::wx_string_from(text);
            let submenu = match submenu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let help = wx_base::wx_string_from(help);
            MenuItem::option_from(ffi::wxMenu_Insert2(
                self.as_ptr(),
                pos,
                id,
                text,
                submenu,
                help,
            ))
        }
    }
    fn insert_check_item(
        &self,
        pos: usize,
        id: c_int,
        item: &str,
        help_string: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = wx_base::wx_string_from(item);
            let help_string = wx_base::wx_string_from(help_string);
            MenuItem::option_from(ffi::wxMenu_InsertCheckItem(
                self.as_ptr(),
                pos,
                id,
                item,
                help_string,
            ))
        }
    }
    fn insert_radio_item(
        &self,
        pos: usize,
        id: c_int,
        item: &str,
        help_string: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = wx_base::wx_string_from(item);
            let help_string = wx_base::wx_string_from(help_string);
            MenuItem::option_from(ffi::wxMenu_InsertRadioItem(
                self.as_ptr(),
                pos,
                id,
                item,
                help_string,
            ))
        }
    }
    fn insert_separator(&self, pos: usize) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_InsertSeparator(self.as_ptr(), pos)) }
    }
    fn is_checked(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenu_IsChecked(self.as_ptr(), id) }
    }
    fn is_enabled(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenu_IsEnabled(self.as_ptr(), id) }
    }
    // NOT_SUPPORTED: fn MSWCommand()
    fn prepend_menuitem<M: MenuItemMethods>(
        &self,
        item: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenu_Prepend(self.as_ptr(), item))
        }
    }
    fn prepend_int_str(
        &self,
        id: c_int,
        item: &str,
        help_string: &str,
        kind: c_int,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = wx_base::wx_string_from(item);
            let help_string = wx_base::wx_string_from(help_string);
            MenuItem::option_from(ffi::wxMenu_Prepend1(
                self.as_ptr(),
                id,
                item,
                help_string,
                kind,
            ))
        }
    }
    fn prepend_int_menu<M: MenuMethods>(
        &self,
        id: c_int,
        text: &str,
        submenu: Option<&M>,
        help: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let text = wx_base::wx_string_from(text);
            let submenu = match submenu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let help = wx_base::wx_string_from(help);
            MenuItem::option_from(ffi::wxMenu_Prepend2(self.as_ptr(), id, text, submenu, help))
        }
    }
    fn prepend_check_item(
        &self,
        id: c_int,
        item: &str,
        help_string: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = wx_base::wx_string_from(item);
            let help_string = wx_base::wx_string_from(help_string);
            MenuItem::option_from(ffi::wxMenu_PrependCheckItem(
                self.as_ptr(),
                id,
                item,
                help_string,
            ))
        }
    }
    fn prepend_radio_item(
        &self,
        id: c_int,
        item: &str,
        help_string: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = wx_base::wx_string_from(item);
            let help_string = wx_base::wx_string_from(help_string);
            MenuItem::option_from(ffi::wxMenu_PrependRadioItem(
                self.as_ptr(),
                id,
                item,
                help_string,
            ))
        }
    }
    fn prepend_separator(&self) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_PrependSeparator(self.as_ptr())) }
    }
    fn remove_int(&self, id: c_int) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_Remove(self.as_ptr(), id)) }
    }
    fn remove_menuitem<M: MenuItemMethods>(
        &self,
        item: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenu_Remove1(self.as_ptr(), item))
        }
    }
    fn set_help_string(&self, id: c_int, help_string: &str) {
        unsafe {
            let help_string = wx_base::wx_string_from(help_string);
            ffi::wxMenu_SetHelpString(self.as_ptr(), id, help_string)
        }
    }
    fn set_label(&self, id: c_int, label: &str) {
        unsafe {
            let label = wx_base::wx_string_from(label);
            ffi::wxMenu_SetLabel(self.as_ptr(), id, label)
        }
    }
    fn set_title(&self, title: &str) {
        unsafe {
            let title = wx_base::wx_string_from(title);
            ffi::wxMenu_SetTitle(self.as_ptr(), title)
        }
    }
    fn update_ui<E: EvtHandlerMethods>(&self, source: Option<&E>) {
        unsafe {
            let source = match source {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_UpdateUI(self.as_ptr(), source)
        }
    }
    fn set_invoking_window<W: WindowMethods>(&self, win: Option<&W>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_SetInvokingWindow(self.as_ptr(), win)
        }
    }
    fn get_invoking_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxMenu_GetInvokingWindow(self.as_ptr())) }
    }
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxMenu_GetWindow(self.as_ptr())) }
    }
    fn get_style(&self) -> c_long {
        unsafe { ffi::wxMenu_GetStyle(self.as_ptr()) }
    }
    fn set_parent<M: MenuMethods>(&self, parent: Option<&M>) {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_SetParent(self.as_ptr(), parent)
        }
    }
    fn get_parent(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenu_GetParent(self.as_ptr())) }
    }
    fn attach<M: MenuBarMethods>(&self, menubar: Option<&M>) {
        unsafe {
            let menubar = match menubar {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_Attach(self.as_ptr(), menubar)
        }
    }
    fn detach(&self) {
        unsafe { ffi::wxMenu_Detach(self.as_ptr()) }
    }
    fn is_attached(&self) -> bool {
        unsafe { ffi::wxMenu_IsAttached(self.as_ptr()) }
    }
}

// wxMenuBar
pub trait MenuBarMethods: WindowMethods {
    // DTOR: fn ~wxMenuBar()
    fn append<M: MenuMethods>(&self, menu: Option<&M>, title: &str) -> bool {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = wx_base::wx_string_from(title);
            ffi::wxMenuBar_Append(self.as_ptr(), menu, title)
        }
    }
    fn check(&self, id: c_int, check: bool) {
        unsafe { ffi::wxMenuBar_Check(self.as_ptr(), id, check) }
    }
    fn enable(&self, id: c_int, enable: bool) {
        unsafe { ffi::wxMenuBar_Enable(self.as_ptr(), id, enable) }
    }
    fn is_enabled_top(&self, pos: usize) -> bool {
        unsafe { ffi::wxMenuBar_IsEnabledTop(self.as_ptr(), pos) }
    }
    fn enable_top(&self, pos: usize, enable: bool) {
        unsafe { ffi::wxMenuBar_EnableTop(self.as_ptr(), pos, enable) }
    }
    fn find_item<M: MenuMethods>(
        &self,
        id: c_int,
        menu: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenuBar_FindItem(self.as_ptr(), id, menu))
        }
    }
    fn find_menu(&self, title: &str) -> c_int {
        unsafe {
            let title = wx_base::wx_string_from(title);
            ffi::wxMenuBar_FindMenu(self.as_ptr(), title)
        }
    }
    fn find_menu_item(&self, menu_string: &str, item_string: &str) -> c_int {
        unsafe {
            let menu_string = wx_base::wx_string_from(menu_string);
            let item_string = wx_base::wx_string_from(item_string);
            ffi::wxMenuBar_FindMenuItem(self.as_ptr(), menu_string, item_string)
        }
    }
    fn get_help_string(&self, id: c_int) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxMenuBar_GetHelpString(self.as_ptr(), id)) }
    }
    fn get_label(&self, id: c_int) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxMenuBar_GetLabel(self.as_ptr(), id)) }
    }
    // BLOCKED: fn GetLabelTop()
    fn get_menu(&self, menu_index: usize) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuBar_GetMenu(self.as_ptr(), menu_index)) }
    }
    fn get_menu_count(&self) -> usize {
        unsafe { ffi::wxMenuBar_GetMenuCount(self.as_ptr()) }
    }
    fn get_menu_label(&self, pos: usize) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxMenuBar_GetMenuLabel(self.as_ptr(), pos)) }
    }
    fn get_menu_label_text(&self, pos: usize) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxMenuBar_GetMenuLabelText(self.as_ptr(), pos)) }
    }
    fn insert<M: MenuMethods>(&self, pos: usize, menu: Option<&M>, title: &str) -> bool {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = wx_base::wx_string_from(title);
            ffi::wxMenuBar_Insert(self.as_ptr(), pos, menu, title)
        }
    }
    fn is_checked(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenuBar_IsChecked(self.as_ptr(), id) }
    }
    fn is_enabled(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenuBar_IsEnabled(self.as_ptr(), id) }
    }
    fn remove(&self, pos: usize) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuBar_Remove(self.as_ptr(), pos)) }
    }
    fn replace<M: MenuMethods>(&self, pos: usize, menu: Option<&M>, title: &str) -> WeakRef<Menu> {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = wx_base::wx_string_from(title);
            WeakRef::<Menu>::from(ffi::wxMenuBar_Replace(self.as_ptr(), pos, menu, title))
        }
    }
    fn set_help_string(&self, id: c_int, help_string: &str) {
        unsafe {
            let help_string = wx_base::wx_string_from(help_string);
            ffi::wxMenuBar_SetHelpString(self.as_ptr(), id, help_string)
        }
    }
    fn set_label(&self, id: c_int, label: &str) {
        unsafe {
            let label = wx_base::wx_string_from(label);
            ffi::wxMenuBar_SetLabel(self.as_ptr(), id, label)
        }
    }
    // BLOCKED: fn SetLabelTop()
    fn set_menu_label(&self, pos: usize, label: &str) {
        unsafe {
            let label = wx_base::wx_string_from(label);
            ffi::wxMenuBar_SetMenuLabel(self.as_ptr(), pos, label)
        }
    }
    fn osx_get_apple_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuBar_OSXGetAppleMenu(self.as_ptr())) }
    }
    fn get_frame(&self) -> WeakRef<Frame> {
        unsafe { WeakRef::<Frame>::from(ffi::wxMenuBar_GetFrame(self.as_ptr())) }
    }
    fn is_attached(&self) -> bool {
        unsafe { ffi::wxMenuBar_IsAttached(self.as_ptr()) }
    }
    fn attach<F: FrameMethods>(&self, frame: Option<&F>) {
        unsafe {
            let frame = match frame {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuBar_Attach(self.as_ptr(), frame)
        }
    }
    fn detach(&self) {
        unsafe { ffi::wxMenuBar_Detach(self.as_ptr()) }
    }
    fn mac_set_common_menu_bar<M: MenuBarMethods>(menubar: Option<&M>) {
        unsafe {
            let menubar = match menubar {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuBar_MacSetCommonMenuBar(menubar)
        }
    }
    fn mac_get_common_menu_bar() -> WeakRef<MenuBar> {
        unsafe { WeakRef::<MenuBar>::from(ffi::wxMenuBar_MacGetCommonMenuBar()) }
    }
}

// wxMenuItem
pub trait MenuItemMethods: ObjectMethods {
    fn get_background_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxMenuItem_GetBackgroundColour(self.as_ptr())) }
    }
    // BLOCKED: fn GetBitmap()
    fn get_disabled_bitmap(&self) -> BitmapIsOwned<false> {
        unsafe { BitmapIsOwned::from_ptr(ffi::wxMenuItem_GetDisabledBitmap(self.as_ptr())) }
    }
    fn get_font(&self) -> *mut c_void {
        unsafe { ffi::wxMenuItem_GetFont(self.as_ptr()) }
    }
    fn get_help(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxMenuItem_GetHelp(self.as_ptr())) }
    }
    fn get_id(&self) -> c_int {
        unsafe { ffi::wxMenuItem_GetId(self.as_ptr()) }
    }
    fn get_item_label(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxMenuItem_GetItemLabel(self.as_ptr())) }
    }
    fn get_item_label_text(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxMenuItem_GetItemLabelText(self.as_ptr())) }
    }
    fn get_kind(&self) -> c_int {
        unsafe { ffi::wxMenuItem_GetKind(self.as_ptr()) }
    }
    // BLOCKED: fn GetLabel()
    fn get_margin_width(&self) -> c_int {
        unsafe { ffi::wxMenuItem_GetMarginWidth(self.as_ptr()) }
    }
    fn get_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuItem_GetMenu(self.as_ptr())) }
    }
    // BLOCKED: fn GetName()
    fn get_sub_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuItem_GetSubMenu(self.as_ptr())) }
    }
    // BLOCKED: fn GetText()
    fn get_text_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxMenuItem_GetTextColour(self.as_ptr())) }
    }
    fn get_accel(&self) -> *mut c_void {
        unsafe { ffi::wxMenuItem_GetAccel(self.as_ptr()) }
    }
    fn get_accel_from_string(label: &str) -> *mut c_void {
        unsafe {
            let label = wx_base::wx_string_from(label);
            ffi::wxMenuItem_GetAccelFromString(label)
        }
    }
    fn is_check(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsCheck(self.as_ptr()) }
    }
    fn is_checkable(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsCheckable(self.as_ptr()) }
    }
    fn is_checked(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsChecked(self.as_ptr()) }
    }
    fn is_enabled(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsEnabled(self.as_ptr()) }
    }
    fn is_radio(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsRadio(self.as_ptr()) }
    }
    fn is_separator(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsSeparator(self.as_ptr()) }
    }
    fn is_sub_menu(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsSubMenu(self.as_ptr()) }
    }
    fn set_background_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxMenuItem_SetBackgroundColour(self.as_ptr(), colour)
        }
    }
    // BLOCKED: fn SetBitmap()
    fn set_bitmaps<B: BitmapMethods, B2: BitmapMethods>(&self, checked: &B, unchecked: &B2) {
        unsafe {
            let checked = checked.as_ptr();
            let unchecked = unchecked.as_ptr();
            ffi::wxMenuItem_SetBitmaps(self.as_ptr(), checked, unchecked)
        }
    }
    fn set_disabled_bitmap<B: BitmapMethods>(&self, disabled: &B) {
        unsafe {
            let disabled = disabled.as_ptr();
            ffi::wxMenuItem_SetDisabledBitmap(self.as_ptr(), disabled)
        }
    }
    fn set_font(&self, font: *const c_void) {
        unsafe { ffi::wxMenuItem_SetFont(self.as_ptr(), font) }
    }
    fn set_help(&self, help_string: &str) {
        unsafe {
            let help_string = wx_base::wx_string_from(help_string);
            ffi::wxMenuItem_SetHelp(self.as_ptr(), help_string)
        }
    }
    fn set_item_label(&self, label: &str) {
        unsafe {
            let label = wx_base::wx_string_from(label);
            ffi::wxMenuItem_SetItemLabel(self.as_ptr(), label)
        }
    }
    fn set_margin_width(&self, width: c_int) {
        unsafe { ffi::wxMenuItem_SetMarginWidth(self.as_ptr(), width) }
    }
    fn set_menu<M: MenuMethods>(&self, menu: Option<&M>) {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuItem_SetMenu(self.as_ptr(), menu)
        }
    }
    fn set_sub_menu<M: MenuMethods>(&self, menu: Option<&M>) {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuItem_SetSubMenu(self.as_ptr(), menu)
        }
    }
    // BLOCKED: fn SetText()
    fn set_text_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxMenuItem_SetTextColour(self.as_ptr(), colour)
        }
    }
    fn set_accel(&self, accel: *mut c_void) {
        unsafe { ffi::wxMenuItem_SetAccel(self.as_ptr(), accel) }
    }
    // DTOR: fn ~wxMenuItem()
    fn check(&self, check: bool) {
        unsafe { ffi::wxMenuItem_Check(self.as_ptr(), check) }
    }
    fn enable(&self, enable: bool) {
        unsafe { ffi::wxMenuItem_Enable(self.as_ptr(), enable) }
    }
    // BLOCKED: fn GetLabelFromText()
    fn get_label_text(text: &str) -> String {
        unsafe {
            let text = wx_base::wx_string_from(text);
            wx_base::from_wx_string(ffi::wxMenuItem_GetLabelText(text))
        }
    }
}

// wxNonOwnedWindow
pub trait NonOwnedWindowMethods: WindowMethods {
    fn set_shape_region(&self, region: *const c_void) -> bool {
        unsafe { ffi::wxNonOwnedWindow_SetShape(self.as_ptr(), region) }
    }
    fn set_shape_graphicspath(&self, path: *const c_void) -> bool {
        unsafe { ffi::wxNonOwnedWindow_SetShape1(self.as_ptr(), path) }
    }
}

// wxNotebook
pub trait NotebookMethods: BookCtrlBaseMethods {
    // DTOR: fn ~wxNotebook()
    fn get_row_count(&self) -> c_int {
        unsafe { ffi::wxNotebook_GetRowCount(self.as_ptr()) }
    }
    fn get_theme_background_colour(&self) -> Colour {
        unsafe { ColourIsOwned(ffi::wxNotebook_GetThemeBackgroundColour(self.as_ptr())) }
    }
    // BLOCKED: fn OnSelChange()
    fn set_padding<S: SizeMethods>(&self, padding: &S) {
        unsafe {
            let padding = padding.as_ptr();
            ffi::wxNotebook_SetPadding(self.as_ptr(), padding)
        }
    }
}

// wxNotifyEvent
pub trait NotifyEventMethods: CommandEventMethods {
    fn allow(&self) {
        unsafe { ffi::wxNotifyEvent_Allow(self.as_ptr()) }
    }
    fn is_allowed(&self) -> bool {
        unsafe { ffi::wxNotifyEvent_IsAllowed(self.as_ptr()) }
    }
    fn veto(&self) {
        unsafe { ffi::wxNotifyEvent_Veto(self.as_ptr()) }
    }
}

// wxPanel
pub trait PanelMethods: WindowMethods {
    // DTOR: fn ~wxPanel()
    fn on_sys_colour_changed(&self, event: *mut c_void) {
        unsafe { ffi::wxPanel_OnSysColourChanged(self.as_ptr(), event) }
    }
    fn set_focus_ignoring_children(&self) {
        unsafe { ffi::wxPanel_SetFocusIgnoringChildren(self.as_ptr()) }
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

// wxRadioBox
pub trait RadioBoxMethods: ControlMethods {
    // DTOR: fn ~wxRadioBox()
    // NOT_SUPPORTED: fn Create()
    fn create<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        choices: &A,
        major_dimension: c_int,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = wx_base::wx_string_from(label);
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = wx_base::wx_string_from(name);
            ffi::wxRadioBox_Create1(
                self.as_ptr(),
                parent,
                id,
                label,
                pos,
                size,
                choices,
                major_dimension,
                style,
                validator,
                name,
            )
        }
    }
    // NOT_SUPPORTED: fn Enable()
    // NOT_SUPPORTED: fn GetColumnCount()
    fn get_item_from_point<P: PointMethods>(&self, pt: &P) -> c_int {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRadioBox_GetItemFromPoint(self.as_ptr(), pt)
        }
    }
    // NOT_SUPPORTED: fn GetItemHelpText()
    // NOT_SUPPORTED: fn GetItemToolTip()
    // NOT_SUPPORTED: fn GetRowCount()
    // NOT_SUPPORTED: fn IsItemEnabled()
    // NOT_SUPPORTED: fn IsItemShown()
    // NOT_SUPPORTED: fn SetItemHelpText()
    // NOT_SUPPORTED: fn SetItemToolTip()
    // NOT_SUPPORTED: fn Show()
}

// wxRect
pub trait RectMethods: WxRustMethods {
    fn centre_in<R: RectMethods>(&self, r: &R, dir: c_int) -> Rect {
        unsafe {
            let r = r.as_ptr();
            RectIsOwned(ffi::wxRect_CentreIn(self.as_ptr(), r, dir))
        }
    }
    fn center_in<R: RectMethods>(&self, r: &R, dir: c_int) -> Rect {
        unsafe {
            let r = r.as_ptr();
            RectIsOwned(ffi::wxRect_CenterIn(self.as_ptr(), r, dir))
        }
    }
    fn contains_int(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxRect_Contains(self.as_ptr(), x, y) }
    }
    fn contains_point<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRect_Contains1(self.as_ptr(), pt)
        }
    }
    fn contains_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRect_Contains2(self.as_ptr(), rect)
        }
    }
    // BLOCKED: fn Deflate()
    // BLOCKED: fn Deflate1()
    // BLOCKED: fn Deflate2()
    fn deflate(&self, dx: c_int, dy: c_int) -> Rect {
        unsafe { RectIsOwned(ffi::wxRect_Deflate3(self.as_ptr(), dx, dy)) }
    }
    fn get_bottom(&self) -> c_int {
        unsafe { ffi::wxRect_GetBottom(self.as_ptr()) }
    }
    fn get_bottom_left(&self) -> Point {
        unsafe { PointIsOwned(ffi::wxRect_GetBottomLeft(self.as_ptr())) }
    }
    fn get_bottom_right(&self) -> Point {
        unsafe { PointIsOwned(ffi::wxRect_GetBottomRight(self.as_ptr())) }
    }
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxRect_GetHeight(self.as_ptr()) }
    }
    fn get_left(&self) -> c_int {
        unsafe { ffi::wxRect_GetLeft(self.as_ptr()) }
    }
    fn get_position(&self) -> Point {
        unsafe { PointIsOwned(ffi::wxRect_GetPosition(self.as_ptr())) }
    }
    fn get_right(&self) -> c_int {
        unsafe { ffi::wxRect_GetRight(self.as_ptr()) }
    }
    fn get_size(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxRect_GetSize(self.as_ptr())) }
    }
    fn get_top(&self) -> c_int {
        unsafe { ffi::wxRect_GetTop(self.as_ptr()) }
    }
    fn get_top_left(&self) -> Point {
        unsafe { PointIsOwned(ffi::wxRect_GetTopLeft(self.as_ptr())) }
    }
    fn get_top_right(&self) -> Point {
        unsafe { PointIsOwned(ffi::wxRect_GetTopRight(self.as_ptr())) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxRect_GetWidth(self.as_ptr()) }
    }
    fn get_x(&self) -> c_int {
        unsafe { ffi::wxRect_GetX(self.as_ptr()) }
    }
    fn get_y(&self) -> c_int {
        unsafe { ffi::wxRect_GetY(self.as_ptr()) }
    }
    // BLOCKED: fn Inflate()
    // BLOCKED: fn Inflate1()
    // BLOCKED: fn Inflate2()
    fn inflate(&self, dx: c_int, dy: c_int) -> Rect {
        unsafe { RectIsOwned(ffi::wxRect_Inflate3(self.as_ptr(), dx, dy)) }
    }
    // BLOCKED: fn Intersect()
    fn intersect<R: RectMethods>(&self, rect: &R) -> Rect {
        unsafe {
            let rect = rect.as_ptr();
            RectIsOwned(ffi::wxRect_Intersect1(self.as_ptr(), rect))
        }
    }
    fn intersects<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRect_Intersects(self.as_ptr(), rect)
        }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxRect_IsEmpty(self.as_ptr()) }
    }
    fn offset_coord(&self, dx: c_int, dy: c_int) {
        unsafe { ffi::wxRect_Offset(self.as_ptr(), dx, dy) }
    }
    fn offset_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRect_Offset1(self.as_ptr(), pt)
        }
    }
    fn set_height(&self, height: c_int) {
        unsafe { ffi::wxRect_SetHeight(self.as_ptr(), height) }
    }
    fn set_position<P: PointMethods>(&self, pos: &P) {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxRect_SetPosition(self.as_ptr(), pos)
        }
    }
    fn set_size<S: SizeMethods>(&self, s: &S) {
        unsafe {
            let s = s.as_ptr();
            ffi::wxRect_SetSize(self.as_ptr(), s)
        }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxRect_SetWidth(self.as_ptr(), width) }
    }
    fn set_x(&self, x: c_int) {
        unsafe { ffi::wxRect_SetX(self.as_ptr(), x) }
    }
    fn set_y(&self, y: c_int) {
        unsafe { ffi::wxRect_SetY(self.as_ptr(), y) }
    }
    fn set_left(&self, left: c_int) {
        unsafe { ffi::wxRect_SetLeft(self.as_ptr(), left) }
    }
    fn set_right(&self, right: c_int) {
        unsafe { ffi::wxRect_SetRight(self.as_ptr(), right) }
    }
    fn set_top(&self, top: c_int) {
        unsafe { ffi::wxRect_SetTop(self.as_ptr(), top) }
    }
    fn set_bottom(&self, bottom: c_int) {
        unsafe { ffi::wxRect_SetBottom(self.as_ptr(), bottom) }
    }
    fn set_top_left<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetTopLeft(self.as_ptr(), p)
        }
    }
    fn set_bottom_right<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetBottomRight(self.as_ptr(), p)
        }
    }
    fn set_top_right<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetTopRight(self.as_ptr(), p)
        }
    }
    fn set_bottom_left<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetBottomLeft(self.as_ptr(), p)
        }
    }
    fn union<R: RectMethods>(&self, rect: &R) -> Rect {
        unsafe {
            let rect = rect.as_ptr();
            RectIsOwned(ffi::wxRect_Union(self.as_ptr(), rect))
        }
    }
    // BLOCKED: fn Union1()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator*()
    // BLOCKED: fn operator*=()
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
}

// wxSize
pub trait SizeMethods: WxRustMethods {
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator-()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // BLOCKED: fn operator/()
    // BLOCKED: fn operator*()
    // BLOCKED: fn operator*1()
    // BLOCKED: fn operator/=()
    // BLOCKED: fn operator*=()
    fn dec_by_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxSize_DecBy(self.as_ptr(), pt)
        }
    }
    fn dec_by_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_DecBy1(self.as_ptr(), size)
        }
    }
    fn dec_by_int_int(&self, dx: c_int, dy: c_int) {
        unsafe { ffi::wxSize_DecBy2(self.as_ptr(), dx, dy) }
    }
    fn dec_by_int(&self, d: c_int) {
        unsafe { ffi::wxSize_DecBy3(self.as_ptr(), d) }
    }
    fn dec_to<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_DecTo(self.as_ptr(), size)
        }
    }
    fn dec_to_if_specified<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_DecToIfSpecified(self.as_ptr(), size)
        }
    }
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxSize_GetHeight(self.as_ptr()) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxSize_GetWidth(self.as_ptr()) }
    }
    fn inc_by_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxSize_IncBy(self.as_ptr(), pt)
        }
    }
    fn inc_by_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_IncBy1(self.as_ptr(), size)
        }
    }
    fn inc_by_int_int(&self, dx: c_int, dy: c_int) {
        unsafe { ffi::wxSize_IncBy2(self.as_ptr(), dx, dy) }
    }
    fn inc_by_int(&self, d: c_int) {
        unsafe { ffi::wxSize_IncBy3(self.as_ptr(), d) }
    }
    fn inc_to<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_IncTo(self.as_ptr(), size)
        }
    }
    fn is_fully_specified(&self) -> bool {
        unsafe { ffi::wxSize_IsFullySpecified(self.as_ptr()) }
    }
    // BLOCKED: fn Scale()
    fn set(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxSize_Set(self.as_ptr(), width, height) }
    }
    fn set_defaults<S: SizeMethods>(&self, size_default: &S) {
        unsafe {
            let size_default = size_default.as_ptr();
            ffi::wxSize_SetDefaults(self.as_ptr(), size_default)
        }
    }
    fn set_height(&self, height: c_int) {
        unsafe { ffi::wxSize_SetHeight(self.as_ptr(), height) }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxSize_SetWidth(self.as_ptr(), width) }
    }
}

// wxSizer
pub trait SizerMethods: ObjectMethods {
    // DTOR: fn ~wxSizer()
    fn add_window_sizerflags<W: WindowMethods, S: SizerFlagsMethods>(
        &self,
        window: Option<&W>,
        flags: &S,
    ) -> *mut c_void {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            ffi::wxSizer_Add(self.as_ptr(), window, flags)
        }
    }
    fn add_window_int<W: WindowMethods, O: ObjectMethods>(
        &self,
        window: Option<&W>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Add1(self.as_ptr(), window, proportion, flag, border, user_data)
        }
    }
    fn add_sizer_sizerflags<S: SizerMethods, S2: SizerFlagsMethods>(
        &self,
        sizer: Option<&S>,
        flags: &S2,
    ) -> *mut c_void {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            ffi::wxSizer_Add2(self.as_ptr(), sizer, flags)
        }
    }
    fn add_sizer_int<S: SizerMethods, O: ObjectMethods>(
        &self,
        sizer: Option<&S>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Add3(self.as_ptr(), sizer, proportion, flag, border, user_data)
        }
    }
    fn add_int_int<O: ObjectMethods>(
        &self,
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Add4(
                self.as_ptr(),
                width,
                height,
                proportion,
                flag,
                border,
                user_data,
            )
        }
    }
    fn add_int_sizerflags<S: SizerFlagsMethods>(
        &self,
        width: c_int,
        height: c_int,
        flags: &S,
    ) -> *mut c_void {
        unsafe {
            let flags = flags.as_ptr();
            ffi::wxSizer_Add5(self.as_ptr(), width, height, flags)
        }
    }
    fn add_sizeritem(&self, item: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxSizer_Add6(self.as_ptr(), item) }
    }
    fn add_spacer(&self, size: c_int) -> *mut c_void {
        unsafe { ffi::wxSizer_AddSpacer(self.as_ptr(), size) }
    }
    fn add_stretch_spacer(&self, prop: c_int) -> *mut c_void {
        unsafe { ffi::wxSizer_AddStretchSpacer(self.as_ptr(), prop) }
    }
    fn calc_min(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxSizer_CalcMin(self.as_ptr())) }
    }
    fn clear(&self, delete_windows: bool) {
        unsafe { ffi::wxSizer_Clear(self.as_ptr(), delete_windows) }
    }
    fn compute_fitting_client_size<W: WindowMethods>(&self, window: Option<&W>) -> Size {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizeIsOwned(ffi::wxSizer_ComputeFittingClientSize(self.as_ptr(), window))
        }
    }
    fn compute_fitting_window_size<W: WindowMethods>(&self, window: Option<&W>) -> Size {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizeIsOwned(ffi::wxSizer_ComputeFittingWindowSize(self.as_ptr(), window))
        }
    }
    fn detach_window<W: WindowMethods>(&self, window: Option<&W>) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Detach(self.as_ptr(), window)
        }
    }
    fn detach_sizer<S: SizerMethods>(&self, sizer: Option<&S>) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Detach1(self.as_ptr(), sizer)
        }
    }
    fn detach_int(&self, index: c_int) -> bool {
        unsafe { ffi::wxSizer_Detach2(self.as_ptr(), index) }
    }
    fn fit<W: WindowMethods>(&self, window: Option<&W>) -> Size {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizeIsOwned(ffi::wxSizer_Fit(self.as_ptr(), window))
        }
    }
    fn fit_inside<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_FitInside(self.as_ptr(), window)
        }
    }
    fn inform_first_direction(
        &self,
        direction: c_int,
        size: c_int,
        available_other_dir: c_int,
    ) -> bool {
        unsafe {
            ffi::wxSizer_InformFirstDirection(self.as_ptr(), direction, size, available_other_dir)
        }
    }
    // BLOCKED: fn GetChildren()
    // BLOCKED: fn GetChildren1()
    fn get_containing_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxSizer_GetContainingWindow(self.as_ptr())) }
    }
    fn set_containing_window<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_SetContainingWindow(self.as_ptr(), window)
        }
    }
    fn get_item_count(&self) -> usize {
        unsafe { ffi::wxSizer_GetItemCount(self.as_ptr()) }
    }
    fn get_item_window<W: WindowMethods>(
        &self,
        window: Option<&W>,
        recursive: bool,
    ) -> *mut c_void {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_GetItem(self.as_ptr(), window, recursive)
        }
    }
    fn get_item_sizer<S: SizerMethods>(&self, sizer: Option<&S>, recursive: bool) -> *mut c_void {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_GetItem1(self.as_ptr(), sizer, recursive)
        }
    }
    fn get_item_sz(&self, index: usize) -> *mut c_void {
        unsafe { ffi::wxSizer_GetItem2(self.as_ptr(), index) }
    }
    fn get_item_by_id(&self, id: c_int, recursive: bool) -> *mut c_void {
        unsafe { ffi::wxSizer_GetItemById(self.as_ptr(), id, recursive) }
    }
    fn get_min_size(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxSizer_GetMinSize(self.as_ptr())) }
    }
    fn get_position(&self) -> Point {
        unsafe { PointIsOwned(ffi::wxSizer_GetPosition(self.as_ptr())) }
    }
    fn get_size(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxSizer_GetSize(self.as_ptr())) }
    }
    fn hide_window<W: WindowMethods>(&self, window: Option<&W>, recursive: bool) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Hide(self.as_ptr(), window, recursive)
        }
    }
    fn hide_sizer<S: SizerMethods>(&self, sizer: Option<&S>, recursive: bool) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Hide1(self.as_ptr(), sizer, recursive)
        }
    }
    fn hide_sz(&self, index: usize) -> bool {
        unsafe { ffi::wxSizer_Hide2(self.as_ptr(), index) }
    }
    fn insert_window_sizerflags<W: WindowMethods, S: SizerFlagsMethods>(
        &self,
        index: usize,
        window: Option<&W>,
        flags: &S,
    ) -> *mut c_void {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            ffi::wxSizer_Insert(self.as_ptr(), index, window, flags)
        }
    }
    fn insert_window_int<W: WindowMethods, O: ObjectMethods>(
        &self,
        index: usize,
        window: Option<&W>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Insert1(
                self.as_ptr(),
                index,
                window,
                proportion,
                flag,
                border,
                user_data,
            )
        }
    }
    fn insert_sizer_sizerflags<S: SizerMethods, S2: SizerFlagsMethods>(
        &self,
        index: usize,
        sizer: Option<&S>,
        flags: &S2,
    ) -> *mut c_void {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            ffi::wxSizer_Insert2(self.as_ptr(), index, sizer, flags)
        }
    }
    fn insert_sizer_int<S: SizerMethods, O: ObjectMethods>(
        &self,
        index: usize,
        sizer: Option<&S>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Insert3(
                self.as_ptr(),
                index,
                sizer,
                proportion,
                flag,
                border,
                user_data,
            )
        }
    }
    fn insert_int_int<O: ObjectMethods>(
        &self,
        index: usize,
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Insert4(
                self.as_ptr(),
                index,
                width,
                height,
                proportion,
                flag,
                border,
                user_data,
            )
        }
    }
    fn insert_int_sizerflags<S: SizerFlagsMethods>(
        &self,
        index: usize,
        width: c_int,
        height: c_int,
        flags: &S,
    ) -> *mut c_void {
        unsafe {
            let flags = flags.as_ptr();
            ffi::wxSizer_Insert5(self.as_ptr(), index, width, height, flags)
        }
    }
    fn insert_sizeritem(&self, index: usize, item: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxSizer_Insert6(self.as_ptr(), index, item) }
    }
    fn insert_spacer(&self, index: usize, size: c_int) -> *mut c_void {
        unsafe { ffi::wxSizer_InsertSpacer(self.as_ptr(), index, size) }
    }
    fn insert_stretch_spacer(&self, index: usize, prop: c_int) -> *mut c_void {
        unsafe { ffi::wxSizer_InsertStretchSpacer(self.as_ptr(), index, prop) }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxSizer_IsEmpty(self.as_ptr()) }
    }
    fn is_shown_window<W: WindowMethods>(&self, window: Option<&W>) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_IsShown(self.as_ptr(), window)
        }
    }
    fn is_shown_sizer<S: SizerMethods>(&self, sizer: Option<&S>) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_IsShown1(self.as_ptr(), sizer)
        }
    }
    fn is_shown_sz(&self, index: usize) -> bool {
        unsafe { ffi::wxSizer_IsShown2(self.as_ptr(), index) }
    }
    fn layout(&self) {
        unsafe { ffi::wxSizer_Layout(self.as_ptr()) }
    }
    fn prepend_window_sizerflags<W: WindowMethods, S: SizerFlagsMethods>(
        &self,
        window: Option<&W>,
        flags: &S,
    ) -> *mut c_void {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            ffi::wxSizer_Prepend(self.as_ptr(), window, flags)
        }
    }
    fn prepend_window_int<W: WindowMethods, O: ObjectMethods>(
        &self,
        window: Option<&W>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Prepend1(self.as_ptr(), window, proportion, flag, border, user_data)
        }
    }
    fn prepend_sizer_sizerflags<S: SizerMethods, S2: SizerFlagsMethods>(
        &self,
        sizer: Option<&S>,
        flags: &S2,
    ) -> *mut c_void {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            ffi::wxSizer_Prepend2(self.as_ptr(), sizer, flags)
        }
    }
    fn prepend_sizer_int<S: SizerMethods, O: ObjectMethods>(
        &self,
        sizer: Option<&S>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Prepend3(self.as_ptr(), sizer, proportion, flag, border, user_data)
        }
    }
    fn prepend_int_int<O: ObjectMethods>(
        &self,
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Prepend4(
                self.as_ptr(),
                width,
                height,
                proportion,
                flag,
                border,
                user_data,
            )
        }
    }
    fn prepend_int_sizerflags<S: SizerFlagsMethods>(
        &self,
        width: c_int,
        height: c_int,
        flags: &S,
    ) -> *mut c_void {
        unsafe {
            let flags = flags.as_ptr();
            ffi::wxSizer_Prepend5(self.as_ptr(), width, height, flags)
        }
    }
    fn prepend_sizeritem(&self, item: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxSizer_Prepend6(self.as_ptr(), item) }
    }
    fn prepend_spacer(&self, size: c_int) -> *mut c_void {
        unsafe { ffi::wxSizer_PrependSpacer(self.as_ptr(), size) }
    }
    fn prepend_stretch_spacer(&self, prop: c_int) -> *mut c_void {
        unsafe { ffi::wxSizer_PrependStretchSpacer(self.as_ptr(), prop) }
    }
    fn reposition_children<S: SizeMethods>(&self, min_size: &S) {
        unsafe {
            let min_size = min_size.as_ptr();
            ffi::wxSizer_RepositionChildren(self.as_ptr(), min_size)
        }
    }
    // BLOCKED: fn Remove()
    fn remove_sizer<S: SizerMethods>(&self, sizer: Option<&S>) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Remove1(self.as_ptr(), sizer)
        }
    }
    fn remove_int(&self, index: c_int) -> bool {
        unsafe { ffi::wxSizer_Remove2(self.as_ptr(), index) }
    }
    fn replace_window<W: WindowMethods, W2: WindowMethods>(
        &self,
        oldwin: Option<&W>,
        newwin: Option<&W2>,
        recursive: bool,
    ) -> bool {
        unsafe {
            let oldwin = match oldwin {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let newwin = match newwin {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Replace(self.as_ptr(), oldwin, newwin, recursive)
        }
    }
    fn replace_sizer<S: SizerMethods, S2: SizerMethods>(
        &self,
        oldsz: Option<&S>,
        newsz: Option<&S2>,
        recursive: bool,
    ) -> bool {
        unsafe {
            let oldsz = match oldsz {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let newsz = match newsz {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Replace1(self.as_ptr(), oldsz, newsz, recursive)
        }
    }
    fn replace_sz(&self, index: usize, newitem: *mut c_void) -> bool {
        unsafe { ffi::wxSizer_Replace2(self.as_ptr(), index, newitem) }
    }
    fn set_dimension_int(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxSizer_SetDimension(self.as_ptr(), x, y, width, height) }
    }
    fn set_dimension_point<P: PointMethods, S: SizeMethods>(&self, pos: &P, size: &S) {
        unsafe {
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            ffi::wxSizer_SetDimension1(self.as_ptr(), pos, size)
        }
    }
    fn set_item_min_size_window_int<W: WindowMethods>(
        &self,
        window: Option<&W>,
        width: c_int,
        height: c_int,
    ) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_SetItemMinSize(self.as_ptr(), window, width, height)
        }
    }
    fn set_item_min_size_window_size<W: WindowMethods, S: SizeMethods>(
        &self,
        window: Option<&W>,
        size: &S,
    ) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let size = size.as_ptr();
            ffi::wxSizer_SetItemMinSize1(self.as_ptr(), window, size)
        }
    }
    fn set_item_min_size_sizer_int<S: SizerMethods>(
        &self,
        sizer: Option<&S>,
        width: c_int,
        height: c_int,
    ) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_SetItemMinSize2(self.as_ptr(), sizer, width, height)
        }
    }
    fn set_item_min_size_sizer_size<S: SizerMethods, S2: SizeMethods>(
        &self,
        sizer: Option<&S>,
        size: &S2,
    ) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let size = size.as_ptr();
            ffi::wxSizer_SetItemMinSize3(self.as_ptr(), sizer, size)
        }
    }
    fn set_item_min_size_sz_int(&self, index: usize, width: c_int, height: c_int) -> bool {
        unsafe { ffi::wxSizer_SetItemMinSize4(self.as_ptr(), index, width, height) }
    }
    fn set_item_min_size_sz_size<S: SizeMethods>(&self, index: usize, size: &S) -> bool {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSizer_SetItemMinSize5(self.as_ptr(), index, size)
        }
    }
    fn set_min_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSizer_SetMinSize(self.as_ptr(), size)
        }
    }
    fn set_min_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxSizer_SetMinSize1(self.as_ptr(), width, height) }
    }
    fn set_size_hints<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_SetSizeHints(self.as_ptr(), window)
        }
    }
    // BLOCKED: fn SetVirtualSizeHints()
    fn show_window<W: WindowMethods>(
        &self,
        window: Option<&W>,
        show: bool,
        recursive: bool,
    ) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Show(self.as_ptr(), window, show, recursive)
        }
    }
    fn show_sizer<S: SizerMethods>(&self, sizer: Option<&S>, show: bool, recursive: bool) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Show1(self.as_ptr(), sizer, show, recursive)
        }
    }
    fn show_sz(&self, index: usize, show: bool) -> bool {
        unsafe { ffi::wxSizer_Show2(self.as_ptr(), index, show) }
    }
    fn show_items(&self, show: bool) {
        unsafe { ffi::wxSizer_ShowItems(self.as_ptr(), show) }
    }
}

// wxSizerFlags
pub trait SizerFlagsMethods: WxRustMethods {
    fn align(&self, alignment: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Align(self.as_ptr(), alignment);
            &self
        }
    }
    fn border_int(&self, direction: c_int, borderinpixels: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Border(self.as_ptr(), direction, borderinpixels);
            &self
        }
    }
    fn border(&self, direction: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Border1(self.as_ptr(), direction);
            &self
        }
    }
    fn bottom(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Bottom(self.as_ptr());
            &self
        }
    }
    fn center(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Center(self.as_ptr());
            &self
        }
    }
    fn centre(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Centre(self.as_ptr());
            &self
        }
    }
    fn center_horizontal(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_CenterHorizontal(self.as_ptr());
            &self
        }
    }
    fn center_vertical(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_CenterVertical(self.as_ptr());
            &self
        }
    }
    fn centre_horizontal(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_CentreHorizontal(self.as_ptr());
            &self
        }
    }
    fn centre_vertical(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_CentreVertical(self.as_ptr());
            &self
        }
    }
    fn double_border(&self, direction: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_DoubleBorder(self.as_ptr(), direction);
            &self
        }
    }
    fn double_horz_border(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_DoubleHorzBorder(self.as_ptr());
            &self
        }
    }
    fn expand(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Expand(self.as_ptr());
            &self
        }
    }
    fn fixed_min_size(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_FixedMinSize(self.as_ptr());
            &self
        }
    }
    fn reserve_space_even_if_hidden(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_ReserveSpaceEvenIfHidden(self.as_ptr());
            &self
        }
    }
    fn left(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Left(self.as_ptr());
            &self
        }
    }
    fn proportion(&self, proportion: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Proportion(self.as_ptr(), proportion);
            &self
        }
    }
    fn right(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Right(self.as_ptr());
            &self
        }
    }
    fn shaped(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Shaped(self.as_ptr());
            &self
        }
    }
    fn top(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Top(self.as_ptr());
            &self
        }
    }
    fn triple_border(&self, direction: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_TripleBorder(self.as_ptr(), direction);
            &self
        }
    }
    fn get_default_border() -> c_int {
        unsafe { ffi::wxSizerFlags_GetDefaultBorder() }
    }
    // NOT_SUPPORTED: fn GetDefaultBorderFractional()
}

// wxStaticBitmap
pub trait StaticBitmapMethods: ControlMethods {
    fn create<W: WindowMethods, B: BitmapMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &B,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = wx_base::wx_string_from(name);
            ffi::wxStaticBitmap_Create(self.as_ptr(), parent, id, label, pos, size, style, name)
        }
    }
    fn get_bitmap(&self) -> Bitmap {
        unsafe { BitmapIsOwned(ffi::wxStaticBitmap_GetBitmap(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetIcon()
    fn set_bitmap<B: BitmapMethods>(&self, label: &B) {
        unsafe {
            let label = label.as_ptr();
            ffi::wxStaticBitmap_SetBitmap(self.as_ptr(), label)
        }
    }
    fn set_icon(&self, label: *const c_void) {
        unsafe { ffi::wxStaticBitmap_SetIcon(self.as_ptr(), label) }
    }
    // NOT_SUPPORTED: fn SetScaleMode()
    // NOT_SUPPORTED: fn GetScaleMode()
}

// wxStaticBox
pub trait StaticBoxMethods: ControlMethods {
    // DTOR: fn ~wxStaticBox()
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = wx_base::wx_string_from(label);
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = wx_base::wx_string_from(name);
            ffi::wxStaticBox_Create(self.as_ptr(), parent, id, label, pos, size, style, name)
        }
    }
    // BLOCKED: fn Create1()
}

// wxStaticBoxSizer
pub trait StaticBoxSizerMethods: BoxSizerMethods {
    fn get_static_box(&self) -> WeakRef<StaticBox> {
        unsafe { WeakRef::<StaticBox>::from(ffi::wxStaticBoxSizer_GetStaticBox(self.as_ptr())) }
    }
}

// wxStaticText
pub trait StaticTextMethods: ControlMethods {
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = wx_base::wx_string_from(label);
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = wx_base::wx_string_from(name);
            ffi::wxStaticText_Create(self.as_ptr(), parent, id, label, pos, size, style, name)
        }
    }
    fn is_ellipsized(&self) -> bool {
        unsafe { ffi::wxStaticText_IsEllipsized(self.as_ptr()) }
    }
    fn wrap(&self, width: c_int) {
        unsafe { ffi::wxStaticText_Wrap(self.as_ptr(), width) }
    }
}

// wxTextAttr
pub trait TextAttrMethods: WxRustMethods {
    // NOT_SUPPORTED: fn GetAlignment()
    fn get_background_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxTextAttr_GetBackgroundColour(self.as_ptr())) }
    }
    fn get_bullet_font(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxTextAttr_GetBulletFont(self.as_ptr())) }
    }
    fn get_bullet_name(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxTextAttr_GetBulletName(self.as_ptr())) }
    }
    fn get_bullet_number(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetBulletNumber(self.as_ptr()) }
    }
    fn get_bullet_style(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetBulletStyle(self.as_ptr()) }
    }
    fn get_bullet_text(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxTextAttr_GetBulletText(self.as_ptr())) }
    }
    fn get_character_style_name(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxTextAttr_GetCharacterStyleName(self.as_ptr())) }
    }
    fn get_flags(&self) -> c_long {
        unsafe { ffi::wxTextAttr_GetFlags(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetFont()
    fn get_font_attributes(&self, font: *const c_void, flags: c_int) -> bool {
        unsafe { ffi::wxTextAttr_GetFontAttributes(self.as_ptr(), font, flags) }
    }
    // NOT_SUPPORTED: fn GetFontEncoding()
    fn get_font_face_name(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxTextAttr_GetFontFaceName(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetFontFamily()
    fn get_font_size(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetFontSize(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetFontStyle()
    fn get_font_underlined(&self) -> bool {
        unsafe { ffi::wxTextAttr_GetFontUnderlined(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetUnderlineType()
    fn get_underline_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxTextAttr_GetUnderlineColour(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetFontWeight()
    fn get_left_indent(&self) -> c_long {
        unsafe { ffi::wxTextAttr_GetLeftIndent(self.as_ptr()) }
    }
    fn get_left_sub_indent(&self) -> c_long {
        unsafe { ffi::wxTextAttr_GetLeftSubIndent(self.as_ptr()) }
    }
    fn get_line_spacing(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetLineSpacing(self.as_ptr()) }
    }
    fn get_list_style_name(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxTextAttr_GetListStyleName(self.as_ptr())) }
    }
    fn get_outline_level(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetOutlineLevel(self.as_ptr()) }
    }
    fn get_paragraph_spacing_after(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetParagraphSpacingAfter(self.as_ptr()) }
    }
    fn get_paragraph_spacing_before(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetParagraphSpacingBefore(self.as_ptr()) }
    }
    fn get_paragraph_style_name(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxTextAttr_GetParagraphStyleName(self.as_ptr())) }
    }
    fn get_right_indent(&self) -> c_long {
        unsafe { ffi::wxTextAttr_GetRightIndent(self.as_ptr()) }
    }
    // BLOCKED: fn GetTabs()
    fn get_text_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxTextAttr_GetTextColour(self.as_ptr())) }
    }
    fn get_text_effect_flags(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetTextEffectFlags(self.as_ptr()) }
    }
    fn get_text_effects(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetTextEffects(self.as_ptr()) }
    }
    fn get_url(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxTextAttr_GetURL(self.as_ptr())) }
    }
    fn has_alignment(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasAlignment(self.as_ptr()) }
    }
    fn has_background_colour(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasBackgroundColour(self.as_ptr()) }
    }
    fn has_bullet_name(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasBulletName(self.as_ptr()) }
    }
    fn has_bullet_number(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasBulletNumber(self.as_ptr()) }
    }
    fn has_bullet_style(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasBulletStyle(self.as_ptr()) }
    }
    fn has_bullet_text(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasBulletText(self.as_ptr()) }
    }
    fn has_character_style_name(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasCharacterStyleName(self.as_ptr()) }
    }
    fn has_flag(&self, flag: c_long) -> bool {
        unsafe { ffi::wxTextAttr_HasFlag(self.as_ptr(), flag) }
    }
    fn has_font(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFont(self.as_ptr()) }
    }
    fn has_font_encoding(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontEncoding(self.as_ptr()) }
    }
    fn has_font_face_name(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontFaceName(self.as_ptr()) }
    }
    fn has_font_family(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontFamily(self.as_ptr()) }
    }
    fn has_font_italic(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontItalic(self.as_ptr()) }
    }
    fn has_font_size(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontSize(self.as_ptr()) }
    }
    fn has_font_point_size(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontPointSize(self.as_ptr()) }
    }
    fn has_font_pixel_size(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontPixelSize(self.as_ptr()) }
    }
    fn has_font_underlined(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontUnderlined(self.as_ptr()) }
    }
    fn has_font_weight(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontWeight(self.as_ptr()) }
    }
    fn has_left_indent(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasLeftIndent(self.as_ptr()) }
    }
    fn has_line_spacing(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasLineSpacing(self.as_ptr()) }
    }
    fn has_list_style_name(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasListStyleName(self.as_ptr()) }
    }
    fn has_outline_level(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasOutlineLevel(self.as_ptr()) }
    }
    fn has_page_break(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasPageBreak(self.as_ptr()) }
    }
    fn has_paragraph_spacing_after(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasParagraphSpacingAfter(self.as_ptr()) }
    }
    fn has_paragraph_spacing_before(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasParagraphSpacingBefore(self.as_ptr()) }
    }
    fn has_paragraph_style_name(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasParagraphStyleName(self.as_ptr()) }
    }
    fn has_right_indent(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasRightIndent(self.as_ptr()) }
    }
    fn has_tabs(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasTabs(self.as_ptr()) }
    }
    fn has_text_colour(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasTextColour(self.as_ptr()) }
    }
    fn has_text_effects(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasTextEffects(self.as_ptr()) }
    }
    fn has_url(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasURL(self.as_ptr()) }
    }
    fn is_character_style(&self) -> bool {
        unsafe { ffi::wxTextAttr_IsCharacterStyle(self.as_ptr()) }
    }
    fn is_default(&self) -> bool {
        unsafe { ffi::wxTextAttr_IsDefault(self.as_ptr()) }
    }
    fn is_paragraph_style(&self) -> bool {
        unsafe { ffi::wxTextAttr_IsParagraphStyle(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetAlignment()
    fn set_background_colour<C: ColourMethods>(&self, col_back: &C) {
        unsafe {
            let col_back = col_back.as_ptr();
            ffi::wxTextAttr_SetBackgroundColour(self.as_ptr(), col_back)
        }
    }
    fn set_bullet_font(&self, font: &str) {
        unsafe {
            let font = wx_base::wx_string_from(font);
            ffi::wxTextAttr_SetBulletFont(self.as_ptr(), font)
        }
    }
    fn set_bullet_name(&self, name: &str) {
        unsafe {
            let name = wx_base::wx_string_from(name);
            ffi::wxTextAttr_SetBulletName(self.as_ptr(), name)
        }
    }
    fn set_bullet_number(&self, n: c_int) {
        unsafe { ffi::wxTextAttr_SetBulletNumber(self.as_ptr(), n) }
    }
    fn set_bullet_style(&self, style: c_int) {
        unsafe { ffi::wxTextAttr_SetBulletStyle(self.as_ptr(), style) }
    }
    fn set_bullet_text(&self, text: &str) {
        unsafe {
            let text = wx_base::wx_string_from(text);
            ffi::wxTextAttr_SetBulletText(self.as_ptr(), text)
        }
    }
    fn set_character_style_name(&self, name: &str) {
        unsafe {
            let name = wx_base::wx_string_from(name);
            ffi::wxTextAttr_SetCharacterStyleName(self.as_ptr(), name)
        }
    }
    fn set_flags(&self, flags: c_long) {
        unsafe { ffi::wxTextAttr_SetFlags(self.as_ptr(), flags) }
    }
    fn set_font(&self, font: *const c_void, flags: c_int) {
        unsafe { ffi::wxTextAttr_SetFont(self.as_ptr(), font, flags) }
    }
    // NOT_SUPPORTED: fn SetFontEncoding()
    fn set_font_face_name(&self, face_name: &str) {
        unsafe {
            let face_name = wx_base::wx_string_from(face_name);
            ffi::wxTextAttr_SetFontFaceName(self.as_ptr(), face_name)
        }
    }
    // NOT_SUPPORTED: fn SetFontFamily()
    fn set_font_size(&self, point_size: c_int) {
        unsafe { ffi::wxTextAttr_SetFontSize(self.as_ptr(), point_size) }
    }
    fn set_font_point_size(&self, point_size: c_int) {
        unsafe { ffi::wxTextAttr_SetFontPointSize(self.as_ptr(), point_size) }
    }
    fn set_font_pixel_size(&self, pixel_size: c_int) {
        unsafe { ffi::wxTextAttr_SetFontPixelSize(self.as_ptr(), pixel_size) }
    }
    // NOT_SUPPORTED: fn SetFontStyle()
    fn set_font_underlined(&self, underlined: bool) {
        unsafe { ffi::wxTextAttr_SetFontUnderlined(self.as_ptr(), underlined) }
    }
    // NOT_SUPPORTED: fn SetFontUnderlined1()
    // NOT_SUPPORTED: fn SetFontWeight()
    fn set_left_indent(&self, indent: c_int, sub_indent: c_int) {
        unsafe { ffi::wxTextAttr_SetLeftIndent(self.as_ptr(), indent, sub_indent) }
    }
    fn set_line_spacing(&self, spacing: c_int) {
        unsafe { ffi::wxTextAttr_SetLineSpacing(self.as_ptr(), spacing) }
    }
    fn set_list_style_name(&self, name: &str) {
        unsafe {
            let name = wx_base::wx_string_from(name);
            ffi::wxTextAttr_SetListStyleName(self.as_ptr(), name)
        }
    }
    fn set_outline_level(&self, level: c_int) {
        unsafe { ffi::wxTextAttr_SetOutlineLevel(self.as_ptr(), level) }
    }
    fn set_page_break(&self, page_break: bool) {
        unsafe { ffi::wxTextAttr_SetPageBreak(self.as_ptr(), page_break) }
    }
    fn set_paragraph_spacing_after(&self, spacing: c_int) {
        unsafe { ffi::wxTextAttr_SetParagraphSpacingAfter(self.as_ptr(), spacing) }
    }
    fn set_paragraph_spacing_before(&self, spacing: c_int) {
        unsafe { ffi::wxTextAttr_SetParagraphSpacingBefore(self.as_ptr(), spacing) }
    }
    fn set_paragraph_style_name(&self, name: &str) {
        unsafe {
            let name = wx_base::wx_string_from(name);
            ffi::wxTextAttr_SetParagraphStyleName(self.as_ptr(), name)
        }
    }
    fn set_right_indent(&self, indent: c_int) {
        unsafe { ffi::wxTextAttr_SetRightIndent(self.as_ptr(), indent) }
    }
    fn set_tabs(&self, tabs: *const c_void) {
        unsafe { ffi::wxTextAttr_SetTabs(self.as_ptr(), tabs) }
    }
    fn set_text_colour<C: ColourMethods>(&self, col_text: &C) {
        unsafe {
            let col_text = col_text.as_ptr();
            ffi::wxTextAttr_SetTextColour(self.as_ptr(), col_text)
        }
    }
    fn set_text_effect_flags(&self, flags: c_int) {
        unsafe { ffi::wxTextAttr_SetTextEffectFlags(self.as_ptr(), flags) }
    }
    fn set_text_effects(&self, effects: c_int) {
        unsafe { ffi::wxTextAttr_SetTextEffects(self.as_ptr(), effects) }
    }
    fn set_url(&self, url: &str) {
        unsafe {
            let url = wx_base::wx_string_from(url);
            ffi::wxTextAttr_SetURL(self.as_ptr(), url)
        }
    }
    // BLOCKED: fn operator=()
    fn apply<T: TextAttrMethods, T2: TextAttrMethods>(
        &self,
        style: &T,
        compare_with: Option<&T2>,
    ) -> bool {
        unsafe {
            let style = style.as_ptr();
            let compare_with = match compare_with {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTextAttr_Apply(self.as_ptr(), style, compare_with)
        }
    }
    fn merge<T: TextAttrMethods>(&self, overlay: &T) {
        unsafe {
            let overlay = overlay.as_ptr();
            ffi::wxTextAttr_Merge(self.as_ptr(), overlay)
        }
    }
    fn eq_partial<T: TextAttrMethods>(&self, attr: &T, weak_test: bool) -> bool {
        unsafe {
            let attr = attr.as_ptr();
            ffi::wxTextAttr_EqPartial(self.as_ptr(), attr, weak_test)
        }
    }
    fn merge_textattr<T: TextAttrMethods, T2: TextAttrMethods>(base: &T, overlay: &T2) -> TextAttr {
        unsafe {
            let base = base.as_ptr();
            let overlay = overlay.as_ptr();
            TextAttrIsOwned(ffi::wxTextAttr_Merge1(base, overlay))
        }
    }
}

// wxTextCtrl
pub trait TextCtrlMethods: ControlMethods {
    // DTOR: fn ~wxTextCtrl()
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        value: &str,
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
            let value = wx_base::wx_string_from(value);
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = wx_base::wx_string_from(name);
            ffi::wxTextCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                value,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn discard_edits(&self) {
        unsafe { ffi::wxTextCtrl_DiscardEdits(self.as_ptr()) }
    }
    fn emulate_key_press(&self, event: *const c_void) -> bool {
        unsafe { ffi::wxTextCtrl_EmulateKeyPress(self.as_ptr(), event) }
    }
    fn get_default_style(&self) -> TextAttrIsOwned<false> {
        unsafe { TextAttrIsOwned::from_ptr(ffi::wxTextCtrl_GetDefaultStyle(self.as_ptr())) }
    }
    fn get_line_length(&self, line_no: c_long) -> c_int {
        unsafe { ffi::wxTextCtrl_GetLineLength(self.as_ptr(), line_no) }
    }
    fn get_line_text(&self, line_no: c_long) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxTextCtrl_GetLineText(self.as_ptr(), line_no)) }
    }
    fn get_number_of_lines(&self) -> c_int {
        unsafe { ffi::wxTextCtrl_GetNumberOfLines(self.as_ptr()) }
    }
    fn get_style(&self, position: c_long, style: *mut c_void) -> bool {
        unsafe { ffi::wxTextCtrl_GetStyle(self.as_ptr(), position, style) }
    }
    // NOT_SUPPORTED: fn HitTest()
    // NOT_SUPPORTED: fn HitTest1()
    fn is_modified(&self) -> bool {
        unsafe { ffi::wxTextCtrl_IsModified(self.as_ptr()) }
    }
    fn is_multi_line(&self) -> bool {
        unsafe { ffi::wxTextCtrl_IsMultiLine(self.as_ptr()) }
    }
    fn is_single_line(&self) -> bool {
        unsafe { ffi::wxTextCtrl_IsSingleLine(self.as_ptr()) }
    }
    fn load_file(&self, filename: &str, file_type: c_int) -> bool {
        unsafe {
            let filename = wx_base::wx_string_from(filename);
            ffi::wxTextCtrl_LoadFile(self.as_ptr(), filename, file_type)
        }
    }
    fn mark_dirty(&self) {
        unsafe { ffi::wxTextCtrl_MarkDirty(self.as_ptr()) }
    }
    fn on_drop_files(&self, event: *mut c_void) {
        unsafe { ffi::wxTextCtrl_OnDropFiles(self.as_ptr(), event) }
    }
    fn position_to_xy(&self, pos: c_long, x: *mut c_void, y: *mut c_void) -> bool {
        unsafe { ffi::wxTextCtrl_PositionToXY(self.as_ptr(), pos, x, y) }
    }
    fn position_to_coords(&self, pos: c_long) -> Point {
        unsafe { PointIsOwned(ffi::wxTextCtrl_PositionToCoords(self.as_ptr(), pos)) }
    }
    fn save_file(&self, filename: &str, file_type: c_int) -> bool {
        unsafe {
            let filename = wx_base::wx_string_from(filename);
            ffi::wxTextCtrl_SaveFile(self.as_ptr(), filename, file_type)
        }
    }
    fn set_default_style<T: TextAttrMethods>(&self, style: &T) -> bool {
        unsafe {
            let style = style.as_ptr();
            ffi::wxTextCtrl_SetDefaultStyle(self.as_ptr(), style)
        }
    }
    fn set_modified(&self, modified: bool) {
        unsafe { ffi::wxTextCtrl_SetModified(self.as_ptr(), modified) }
    }
    fn set_style<T: TextAttrMethods>(&self, start: c_long, end: c_long, style: &T) -> bool {
        unsafe {
            let style = style.as_ptr();
            ffi::wxTextCtrl_SetStyle(self.as_ptr(), start, end, style)
        }
    }
    fn show_position(&self, pos: c_long) {
        unsafe { ffi::wxTextCtrl_ShowPosition(self.as_ptr(), pos) }
    }
    fn xy_to_position(&self, x: c_long, y: c_long) -> c_long {
        unsafe { ffi::wxTextCtrl_XYToPosition(self.as_ptr(), x, y) }
    }
    // BLOCKED: fn operator<<()
    // BLOCKED: fn operator<<1()
    // BLOCKED: fn operator<<2()
    // NOT_SUPPORTED: fn operator<<3()
    // BLOCKED: fn operator<<4()
    // NOT_SUPPORTED: fn operator<<5()
    // NOT_SUPPORTED: fn operator<<6()
}

// wxTextEntry
pub trait TextEntryMethods: WxRustMethods {
    fn append_text(&self, text: &str) {
        unsafe {
            let text = wx_base::wx_string_from(text);
            ffi::wxTextEntry_AppendText(self.as_ptr(), text)
        }
    }
    fn auto_complete_arraystring<A: ArrayStringMethods>(&self, choices: &A) -> bool {
        unsafe {
            let choices = choices.as_ptr();
            ffi::wxTextEntry_AutoComplete(self.as_ptr(), choices)
        }
    }
    fn auto_complete_textcompleter(&self, completer: *mut c_void) -> bool {
        unsafe { ffi::wxTextEntry_AutoComplete1(self.as_ptr(), completer) }
    }
    fn auto_complete_file_names(&self) -> bool {
        unsafe { ffi::wxTextEntry_AutoCompleteFileNames(self.as_ptr()) }
    }
    fn auto_complete_directories(&self) -> bool {
        unsafe { ffi::wxTextEntry_AutoCompleteDirectories(self.as_ptr()) }
    }
    fn can_copy(&self) -> bool {
        unsafe { ffi::wxTextEntry_CanCopy(self.as_ptr()) }
    }
    fn can_cut(&self) -> bool {
        unsafe { ffi::wxTextEntry_CanCut(self.as_ptr()) }
    }
    fn can_paste(&self) -> bool {
        unsafe { ffi::wxTextEntry_CanPaste(self.as_ptr()) }
    }
    fn can_redo(&self) -> bool {
        unsafe { ffi::wxTextEntry_CanRedo(self.as_ptr()) }
    }
    fn can_undo(&self) -> bool {
        unsafe { ffi::wxTextEntry_CanUndo(self.as_ptr()) }
    }
    fn change_value(&self, value: &str) {
        unsafe {
            let value = wx_base::wx_string_from(value);
            ffi::wxTextEntry_ChangeValue(self.as_ptr(), value)
        }
    }
    fn clear(&self) {
        unsafe { ffi::wxTextEntry_Clear(self.as_ptr()) }
    }
    fn copy(&self) {
        unsafe { ffi::wxTextEntry_Copy(self.as_ptr()) }
    }
    fn cut(&self) {
        unsafe { ffi::wxTextEntry_Cut(self.as_ptr()) }
    }
    fn force_upper(&self) {
        unsafe { ffi::wxTextEntry_ForceUpper(self.as_ptr()) }
    }
    fn get_insertion_point(&self) -> c_long {
        unsafe { ffi::wxTextEntry_GetInsertionPoint(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetLastPosition()
    fn get_range(&self, from: c_long, to: c_long) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxTextEntry_GetRange(self.as_ptr(), from, to)) }
    }
    fn get_selection(&self, from: *mut c_void, to: *mut c_void) {
        unsafe { ffi::wxTextEntry_GetSelection(self.as_ptr(), from, to) }
    }
    fn get_string_selection(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxTextEntry_GetStringSelection(self.as_ptr())) }
    }
    fn get_value(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxTextEntry_GetValue(self.as_ptr())) }
    }
    fn is_editable(&self) -> bool {
        unsafe { ffi::wxTextEntry_IsEditable(self.as_ptr()) }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxTextEntry_IsEmpty(self.as_ptr()) }
    }
    fn paste(&self) {
        unsafe { ffi::wxTextEntry_Paste(self.as_ptr()) }
    }
    fn redo(&self) {
        unsafe { ffi::wxTextEntry_Redo(self.as_ptr()) }
    }
    fn remove(&self, from: c_long, to: c_long) {
        unsafe { ffi::wxTextEntry_Remove(self.as_ptr(), from, to) }
    }
    fn replace(&self, from: c_long, to: c_long, value: &str) {
        unsafe {
            let value = wx_base::wx_string_from(value);
            ffi::wxTextEntry_Replace(self.as_ptr(), from, to, value)
        }
    }
    fn set_editable(&self, editable: bool) {
        unsafe { ffi::wxTextEntry_SetEditable(self.as_ptr(), editable) }
    }
    fn set_insertion_point(&self, pos: c_long) {
        unsafe { ffi::wxTextEntry_SetInsertionPoint(self.as_ptr(), pos) }
    }
    fn set_insertion_point_end(&self) {
        unsafe { ffi::wxTextEntry_SetInsertionPointEnd(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetMaxLength()
    fn set_selection(&self, from: c_long, to: c_long) {
        unsafe { ffi::wxTextEntry_SetSelection(self.as_ptr(), from, to) }
    }
    fn select_all(&self) {
        unsafe { ffi::wxTextEntry_SelectAll(self.as_ptr()) }
    }
    fn select_none(&self) {
        unsafe { ffi::wxTextEntry_SelectNone(self.as_ptr()) }
    }
    fn set_hint(&self, hint: &str) -> bool {
        unsafe {
            let hint = wx_base::wx_string_from(hint);
            ffi::wxTextEntry_SetHint(self.as_ptr(), hint)
        }
    }
    fn get_hint(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxTextEntry_GetHint(self.as_ptr())) }
    }
    fn set_margins_point<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxTextEntry_SetMargins(self.as_ptr(), pt)
        }
    }
    fn set_margins_coord(&self, left: c_int, top: c_int) -> bool {
        unsafe { ffi::wxTextEntry_SetMargins1(self.as_ptr(), left, top) }
    }
    fn get_margins(&self) -> Point {
        unsafe { PointIsOwned(ffi::wxTextEntry_GetMargins(self.as_ptr())) }
    }
    fn set_value(&self, value: &str) {
        unsafe {
            let value = wx_base::wx_string_from(value);
            ffi::wxTextEntry_SetValue(self.as_ptr(), value)
        }
    }
    fn undo(&self) {
        unsafe { ffi::wxTextEntry_Undo(self.as_ptr()) }
    }
    fn write_text(&self, text: &str) {
        unsafe {
            let text = wx_base::wx_string_from(text);
            ffi::wxTextEntry_WriteText(self.as_ptr(), text)
        }
    }
}

// wxToolBar
pub trait ToolBarMethods: ControlMethods {
    // DTOR: fn ~wxToolBar()
    fn add_check_tool<B: BitmapMethods, B2: BitmapMethods, O: ObjectMethods>(
        &self,
        tool_id: c_int,
        label: &str,
        bitmap1: &B,
        bmp_disabled: &B2,
        short_help: &str,
        long_help: &str,
        client_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let label = wx_base::wx_string_from(label);
            let bitmap1 = bitmap1.as_ptr();
            let bmp_disabled = bmp_disabled.as_ptr();
            let short_help = wx_base::wx_string_from(short_help);
            let long_help = wx_base::wx_string_from(long_help);
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxToolBar_AddCheckTool(
                self.as_ptr(),
                tool_id,
                label,
                bitmap1,
                bmp_disabled,
                short_help,
                long_help,
                client_data,
            )
        }
    }
    fn add_control<C: ControlMethods>(&self, control: Option<&C>, label: &str) -> *mut c_void {
        unsafe {
            let control = match control {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = wx_base::wx_string_from(label);
            ffi::wxToolBar_AddControl(self.as_ptr(), control, label)
        }
    }
    fn add_radio_tool<B: BitmapMethods, B2: BitmapMethods, O: ObjectMethods>(
        &self,
        tool_id: c_int,
        label: &str,
        bitmap1: &B,
        bmp_disabled: &B2,
        short_help: &str,
        long_help: &str,
        client_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let label = wx_base::wx_string_from(label);
            let bitmap1 = bitmap1.as_ptr();
            let bmp_disabled = bmp_disabled.as_ptr();
            let short_help = wx_base::wx_string_from(short_help);
            let long_help = wx_base::wx_string_from(long_help);
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxToolBar_AddRadioTool(
                self.as_ptr(),
                tool_id,
                label,
                bitmap1,
                bmp_disabled,
                short_help,
                long_help,
                client_data,
            )
        }
    }
    fn add_separator(&self) -> *mut c_void {
        unsafe { ffi::wxToolBar_AddSeparator(self.as_ptr()) }
    }
    fn add_stretchable_space(&self) -> *mut c_void {
        unsafe { ffi::wxToolBar_AddStretchableSpace(self.as_ptr()) }
    }
    fn add_tool_toolbartoolbase(&self, tool: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxToolBar_AddTool(self.as_ptr(), tool) }
    }
    fn add_tool_int_str<B: BitmapMethods>(
        &self,
        tool_id: c_int,
        label: &str,
        bitmap: &B,
        short_help: &str,
        kind: c_int,
    ) -> *mut c_void {
        unsafe {
            let label = wx_base::wx_string_from(label);
            let bitmap = bitmap.as_ptr();
            let short_help = wx_base::wx_string_from(short_help);
            ffi::wxToolBar_AddTool1(self.as_ptr(), tool_id, label, bitmap, short_help, kind)
        }
    }
    fn add_tool_int_bitmap<B: BitmapMethods, B2: BitmapMethods, O: ObjectMethods>(
        &self,
        tool_id: c_int,
        label: &str,
        bitmap: &B,
        bmp_disabled: &B2,
        kind: c_int,
        short_help: &str,
        long_help: &str,
        client_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let label = wx_base::wx_string_from(label);
            let bitmap = bitmap.as_ptr();
            let bmp_disabled = bmp_disabled.as_ptr();
            let short_help = wx_base::wx_string_from(short_help);
            let long_help = wx_base::wx_string_from(long_help);
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxToolBar_AddTool2(
                self.as_ptr(),
                tool_id,
                label,
                bitmap,
                bmp_disabled,
                kind,
                short_help,
                long_help,
                client_data,
            )
        }
    }
    fn clear_tools(&self) {
        unsafe { ffi::wxToolBar_ClearTools(self.as_ptr()) }
    }
    fn delete_tool(&self, tool_id: c_int) -> bool {
        unsafe { ffi::wxToolBar_DeleteTool(self.as_ptr(), tool_id) }
    }
    fn delete_tool_by_pos(&self, pos: usize) -> bool {
        unsafe { ffi::wxToolBar_DeleteToolByPos(self.as_ptr(), pos) }
    }
    fn enable_tool(&self, tool_id: c_int, enable: bool) {
        unsafe { ffi::wxToolBar_EnableTool(self.as_ptr(), tool_id, enable) }
    }
    fn find_by_id(&self, id: c_int) -> *mut c_void {
        unsafe { ffi::wxToolBar_FindById(self.as_ptr(), id) }
    }
    fn find_control(&self, id: c_int) -> WeakRef<Control> {
        unsafe { WeakRef::<Control>::from(ffi::wxToolBar_FindControl(self.as_ptr(), id)) }
    }
    fn find_tool_for_position(&self, x: c_int, y: c_int) -> *mut c_void {
        unsafe { ffi::wxToolBar_FindToolForPosition(self.as_ptr(), x, y) }
    }
    fn get_margins(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxToolBar_GetMargins(self.as_ptr())) }
    }
    fn get_tool_bitmap_size(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxToolBar_GetToolBitmapSize(self.as_ptr())) }
    }
    // BLOCKED: fn GetToolByPos()
    fn get_tool_by_pos(&self, pos: c_int) -> *const c_void {
        unsafe { ffi::wxToolBar_GetToolByPos1(self.as_ptr(), pos) }
    }
    fn get_tool_client_data(&self, tool_id: c_int) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxToolBar_GetToolClientData(self.as_ptr(), tool_id)) }
    }
    fn get_tool_enabled(&self, tool_id: c_int) -> bool {
        unsafe { ffi::wxToolBar_GetToolEnabled(self.as_ptr(), tool_id) }
    }
    fn get_tool_long_help(&self, tool_id: c_int) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxToolBar_GetToolLongHelp(self.as_ptr(), tool_id)) }
    }
    fn get_tool_packing(&self) -> c_int {
        unsafe { ffi::wxToolBar_GetToolPacking(self.as_ptr()) }
    }
    fn get_tool_pos(&self, tool_id: c_int) -> c_int {
        unsafe { ffi::wxToolBar_GetToolPos(self.as_ptr(), tool_id) }
    }
    fn get_tool_separation(&self) -> c_int {
        unsafe { ffi::wxToolBar_GetToolSeparation(self.as_ptr()) }
    }
    fn get_tool_short_help(&self, tool_id: c_int) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxToolBar_GetToolShortHelp(self.as_ptr(), tool_id)) }
    }
    fn get_tool_size(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxToolBar_GetToolSize(self.as_ptr())) }
    }
    fn get_tool_state(&self, tool_id: c_int) -> bool {
        unsafe { ffi::wxToolBar_GetToolState(self.as_ptr(), tool_id) }
    }
    fn get_tools_count(&self) -> usize {
        unsafe { ffi::wxToolBar_GetToolsCount(self.as_ptr()) }
    }
    fn insert_control<C: ControlMethods>(
        &self,
        pos: usize,
        control: Option<&C>,
        label: &str,
    ) -> *mut c_void {
        unsafe {
            let control = match control {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = wx_base::wx_string_from(label);
            ffi::wxToolBar_InsertControl(self.as_ptr(), pos, control, label)
        }
    }
    fn insert_separator(&self, pos: usize) -> *mut c_void {
        unsafe { ffi::wxToolBar_InsertSeparator(self.as_ptr(), pos) }
    }
    fn insert_stretchable_space(&self, pos: usize) -> *mut c_void {
        unsafe { ffi::wxToolBar_InsertStretchableSpace(self.as_ptr(), pos) }
    }
    fn insert_tool_int<B: BitmapMethods, B2: BitmapMethods, O: ObjectMethods>(
        &self,
        pos: usize,
        tool_id: c_int,
        label: &str,
        bitmap: &B,
        bmp_disabled: &B2,
        kind: c_int,
        short_help: &str,
        long_help: &str,
        client_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let label = wx_base::wx_string_from(label);
            let bitmap = bitmap.as_ptr();
            let bmp_disabled = bmp_disabled.as_ptr();
            let short_help = wx_base::wx_string_from(short_help);
            let long_help = wx_base::wx_string_from(long_help);
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxToolBar_InsertTool(
                self.as_ptr(),
                pos,
                tool_id,
                label,
                bitmap,
                bmp_disabled,
                kind,
                short_help,
                long_help,
                client_data,
            )
        }
    }
    fn insert_tool_toolbartoolbase(&self, pos: usize, tool: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxToolBar_InsertTool1(self.as_ptr(), pos, tool) }
    }
    fn on_left_click(&self, tool_id: c_int, toggle_down: bool) -> bool {
        unsafe { ffi::wxToolBar_OnLeftClick(self.as_ptr(), tool_id, toggle_down) }
    }
    fn on_mouse_enter(&self, tool_id: c_int) {
        unsafe { ffi::wxToolBar_OnMouseEnter(self.as_ptr(), tool_id) }
    }
    fn on_right_click(&self, tool_id: c_int, x: c_long, y: c_long) {
        unsafe { ffi::wxToolBar_OnRightClick(self.as_ptr(), tool_id, x, y) }
    }
    fn realize(&self) -> bool {
        unsafe { ffi::wxToolBar_Realize(self.as_ptr()) }
    }
    fn remove_tool(&self, id: c_int) -> *mut c_void {
        unsafe { ffi::wxToolBar_RemoveTool(self.as_ptr(), id) }
    }
    fn set_dropdown_menu<M: MenuMethods>(&self, id: c_int, menu: Option<&M>) -> bool {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxToolBar_SetDropdownMenu(self.as_ptr(), id, menu)
        }
    }
    fn set_margins_int(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxToolBar_SetMargins(self.as_ptr(), x, y) }
    }
    fn set_margins_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxToolBar_SetMargins1(self.as_ptr(), size)
        }
    }
    fn set_tool_bitmap_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxToolBar_SetToolBitmapSize(self.as_ptr(), size)
        }
    }
    fn set_tool_client_data<O: ObjectMethods>(&self, id: c_int, client_data: Option<&O>) {
        unsafe {
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxToolBar_SetToolClientData(self.as_ptr(), id, client_data)
        }
    }
    fn set_tool_disabled_bitmap<B: BitmapMethods>(&self, id: c_int, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxToolBar_SetToolDisabledBitmap(self.as_ptr(), id, bitmap)
        }
    }
    fn set_tool_long_help(&self, tool_id: c_int, help_string: &str) {
        unsafe {
            let help_string = wx_base::wx_string_from(help_string);
            ffi::wxToolBar_SetToolLongHelp(self.as_ptr(), tool_id, help_string)
        }
    }
    fn set_tool_normal_bitmap<B: BitmapMethods>(&self, id: c_int, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxToolBar_SetToolNormalBitmap(self.as_ptr(), id, bitmap)
        }
    }
    fn set_tool_packing(&self, packing: c_int) {
        unsafe { ffi::wxToolBar_SetToolPacking(self.as_ptr(), packing) }
    }
    fn set_tool_separation(&self, separation: c_int) {
        unsafe { ffi::wxToolBar_SetToolSeparation(self.as_ptr(), separation) }
    }
    fn set_tool_short_help(&self, tool_id: c_int, help_string: &str) {
        unsafe {
            let help_string = wx_base::wx_string_from(help_string);
            ffi::wxToolBar_SetToolShortHelp(self.as_ptr(), tool_id, help_string)
        }
    }
    fn toggle_tool(&self, tool_id: c_int, toggle: bool) {
        unsafe { ffi::wxToolBar_ToggleTool(self.as_ptr(), tool_id, toggle) }
    }
    fn create_tool_int<B: BitmapMethods, B2: BitmapMethods, O: ObjectMethods>(
        &self,
        tool_id: c_int,
        label: &str,
        bmp_normal: &B,
        bmp_disabled: &B2,
        kind: c_int,
        client_data: Option<&O>,
        short_help: &str,
        long_help: &str,
    ) -> *mut c_void {
        unsafe {
            let label = wx_base::wx_string_from(label);
            let bmp_normal = bmp_normal.as_ptr();
            let bmp_disabled = bmp_disabled.as_ptr();
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let short_help = wx_base::wx_string_from(short_help);
            let long_help = wx_base::wx_string_from(long_help);
            ffi::wxToolBar_CreateTool(
                self.as_ptr(),
                tool_id,
                label,
                bmp_normal,
                bmp_disabled,
                kind,
                client_data,
                short_help,
                long_help,
            )
        }
    }
    fn create_tool_control<C: ControlMethods>(
        &self,
        control: Option<&C>,
        label: &str,
    ) -> *mut c_void {
        unsafe {
            let control = match control {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = wx_base::wx_string_from(label);
            ffi::wxToolBar_CreateTool1(self.as_ptr(), control, label)
        }
    }
    fn create_separator(&self) -> *mut c_void {
        unsafe { ffi::wxToolBar_CreateSeparator(self.as_ptr()) }
    }
}

// wxTopLevelWindow
pub trait TopLevelWindowMethods: NonOwnedWindowMethods {
    // DTOR: fn ~wxTopLevelWindow()
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = wx_base::wx_string_from(title);
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = wx_base::wx_string_from(name);
            ffi::wxTopLevelWindow_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
    fn center_on_screen(&self, direction: c_int) {
        unsafe { ffi::wxTopLevelWindow_CenterOnScreen(self.as_ptr(), direction) }
    }
    fn centre_on_screen(&self, direction: c_int) {
        unsafe { ffi::wxTopLevelWindow_CentreOnScreen(self.as_ptr(), direction) }
    }
    fn enable_close_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableCloseButton(self.as_ptr(), enable) }
    }
    fn enable_maximize_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableMaximizeButton(self.as_ptr(), enable) }
    }
    fn enable_minimize_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableMinimizeButton(self.as_ptr(), enable) }
    }
    fn get_default_item(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxTopLevelWindow_GetDefaultItem(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetIcon()
    // BLOCKED: fn GetIcons()
    fn get_title(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxTopLevelWindow_GetTitle(self.as_ptr())) }
    }
    fn iconize(&self, iconize: bool) {
        unsafe { ffi::wxTopLevelWindow_Iconize(self.as_ptr(), iconize) }
    }
    fn is_active(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsActive(self.as_ptr()) }
    }
    fn is_always_maximized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsAlwaysMaximized(self.as_ptr()) }
    }
    fn is_full_screen(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsFullScreen(self.as_ptr()) }
    }
    fn is_iconized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsIconized(self.as_ptr()) }
    }
    fn is_maximized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsMaximized(self.as_ptr()) }
    }
    // BLOCKED: fn IsUsingNativeDecorations()
    fn maximize(&self, maximize: bool) {
        unsafe { ffi::wxTopLevelWindow_Maximize(self.as_ptr(), maximize) }
    }
    fn msw_get_system_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxTopLevelWindow_MSWGetSystemMenu(self.as_ptr())) }
    }
    fn request_user_attention(&self, flags: c_int) {
        unsafe { ffi::wxTopLevelWindow_RequestUserAttention(self.as_ptr(), flags) }
    }
    fn restore(&self) {
        unsafe { ffi::wxTopLevelWindow_Restore(self.as_ptr()) }
    }
    // BLOCKED: fn RestoreToGeometry()
    // BLOCKED: fn SaveGeometry()
    fn set_default_item<W: WindowMethods>(&self, win: Option<&W>) -> WeakRef<Window> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<Window>::from(ffi::wxTopLevelWindow_SetDefaultItem(self.as_ptr(), win))
        }
    }
    fn set_tmp_default_item<W: WindowMethods>(&self, win: Option<&W>) -> WeakRef<Window> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<Window>::from(ffi::wxTopLevelWindow_SetTmpDefaultItem(self.as_ptr(), win))
        }
    }
    fn get_tmp_default_item(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxTopLevelWindow_GetTmpDefaultItem(self.as_ptr())) }
    }
    fn set_icon(&self, icon: *const c_void) {
        unsafe { ffi::wxTopLevelWindow_SetIcon(self.as_ptr(), icon) }
    }
    fn set_icons(&self, icons: *const c_void) {
        unsafe { ffi::wxTopLevelWindow_SetIcons(self.as_ptr(), icons) }
    }
    fn set_title(&self, title: &str) {
        unsafe {
            let title = wx_base::wx_string_from(title);
            ffi::wxTopLevelWindow_SetTitle(self.as_ptr(), title)
        }
    }
    fn should_prevent_app_exit(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_ShouldPreventAppExit(self.as_ptr()) }
    }
    fn osx_set_modified(&self, modified: bool) {
        unsafe { ffi::wxTopLevelWindow_OSXSetModified(self.as_ptr(), modified) }
    }
    fn osx_is_modified(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_OSXIsModified(self.as_ptr()) }
    }
    fn set_represented_filename(&self, filename: &str) {
        unsafe {
            let filename = wx_base::wx_string_from(filename);
            ffi::wxTopLevelWindow_SetRepresentedFilename(self.as_ptr(), filename)
        }
    }
    fn show_without_activating(&self) {
        unsafe { ffi::wxTopLevelWindow_ShowWithoutActivating(self.as_ptr()) }
    }
    fn enable_full_screen_view(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableFullScreenView(self.as_ptr(), enable) }
    }
    fn show_full_screen(&self, show: bool, style: c_long) -> bool {
        unsafe { ffi::wxTopLevelWindow_ShowFullScreen(self.as_ptr(), show, style) }
    }
    // BLOCKED: fn UseNativeDecorations()
    // BLOCKED: fn UseNativeDecorationsByDefault()
    fn get_default_size() -> Size {
        unsafe { SizeIsOwned(ffi::wxTopLevelWindow_GetDefaultSize()) }
    }
}

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

// wxWindow
pub trait WindowMethods: EvtHandlerMethods {
    fn accepts_focus(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocus(self.as_ptr()) }
    }
    fn accepts_focus_from_keyboard(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocusFromKeyboard(self.as_ptr()) }
    }
    fn accepts_focus_recursively(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocusRecursively(self.as_ptr()) }
    }
    fn disable_focus_from_keyboard(&self) {
        unsafe { ffi::wxWindow_DisableFocusFromKeyboard(self.as_ptr()) }
    }
    fn is_focusable(&self) -> bool {
        unsafe { ffi::wxWindow_IsFocusable(self.as_ptr()) }
    }
    fn can_accept_focus(&self) -> bool {
        unsafe { ffi::wxWindow_CanAcceptFocus(self.as_ptr()) }
    }
    fn can_accept_focus_from_keyboard(&self) -> bool {
        unsafe { ffi::wxWindow_CanAcceptFocusFromKeyboard(self.as_ptr()) }
    }
    fn has_focus(&self) -> bool {
        unsafe { ffi::wxWindow_HasFocus(self.as_ptr()) }
    }
    fn set_can_focus(&self, can_focus: bool) {
        unsafe { ffi::wxWindow_SetCanFocus(self.as_ptr(), can_focus) }
    }
    fn enable_visible_focus(&self, enable: bool) {
        unsafe { ffi::wxWindow_EnableVisibleFocus(self.as_ptr(), enable) }
    }
    fn set_focus(&self) {
        unsafe { ffi::wxWindow_SetFocus(self.as_ptr()) }
    }
    fn set_focus_from_kbd(&self) {
        unsafe { ffi::wxWindow_SetFocusFromKbd(self.as_ptr()) }
    }
    fn add_child<W: WindowMethods>(&self, child: Option<&W>) {
        unsafe {
            let child = match child {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_AddChild(self.as_ptr(), child)
        }
    }
    fn destroy_children(&self) -> bool {
        unsafe { ffi::wxWindow_DestroyChildren(self.as_ptr()) }
    }
    fn find_window_long(&self, id: c_long) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_FindWindow(self.as_ptr(), id)) }
    }
    fn find_window_str(&self, name: &str) -> WeakRef<Window> {
        unsafe {
            let name = wx_base::wx_string_from(name);
            WeakRef::<Window>::from(ffi::wxWindow_FindWindow1(self.as_ptr(), name))
        }
    }
    // BLOCKED: fn GetChildren()
    fn get_children(&self) -> WindowListIsOwned<false> {
        unsafe { WindowListIsOwned::from_ptr(ffi::wxWindow_GetChildren1(self.as_ptr())) }
    }
    fn remove_child<W: WindowMethods>(&self, child: Option<&W>) {
        unsafe {
            let child = match child {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_RemoveChild(self.as_ptr(), child)
        }
    }
    fn get_grand_parent(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetGrandParent(self.as_ptr())) }
    }
    fn get_next_sibling(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetNextSibling(self.as_ptr())) }
    }
    fn get_parent(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetParent(self.as_ptr())) }
    }
    fn get_prev_sibling(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetPrevSibling(self.as_ptr())) }
    }
    fn is_descendant<W: WindowMethods>(&self, win: Option<&W>) -> bool {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_IsDescendant(self.as_ptr(), win)
        }
    }
    fn reparent<W: WindowMethods>(&self, new_parent: Option<&W>) -> bool {
        unsafe {
            let new_parent = match new_parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_Reparent(self.as_ptr(), new_parent)
        }
    }
    fn always_show_scrollbars(&self, hflag: bool, vflag: bool) {
        unsafe { ffi::wxWindow_AlwaysShowScrollbars(self.as_ptr(), hflag, vflag) }
    }
    fn get_scroll_pos(&self, orientation: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetScrollPos(self.as_ptr(), orientation) }
    }
    fn get_scroll_range(&self, orientation: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetScrollRange(self.as_ptr(), orientation) }
    }
    fn get_scroll_thumb(&self, orientation: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetScrollThumb(self.as_ptr(), orientation) }
    }
    fn can_scroll(&self, orient: c_int) -> bool {
        unsafe { ffi::wxWindow_CanScroll(self.as_ptr(), orient) }
    }
    fn has_scrollbar(&self, orient: c_int) -> bool {
        unsafe { ffi::wxWindow_HasScrollbar(self.as_ptr(), orient) }
    }
    fn is_scrollbar_always_shown(&self, orient: c_int) -> bool {
        unsafe { ffi::wxWindow_IsScrollbarAlwaysShown(self.as_ptr(), orient) }
    }
    fn scroll_lines(&self, lines: c_int) -> bool {
        unsafe { ffi::wxWindow_ScrollLines(self.as_ptr(), lines) }
    }
    fn scroll_pages(&self, pages: c_int) -> bool {
        unsafe { ffi::wxWindow_ScrollPages(self.as_ptr(), pages) }
    }
    fn scroll_window<R: RectMethods>(&self, dx: c_int, dy: c_int, rect: Option<&R>) {
        unsafe {
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_ScrollWindow(self.as_ptr(), dx, dy, rect)
        }
    }
    fn line_up(&self) -> bool {
        unsafe { ffi::wxWindow_LineUp(self.as_ptr()) }
    }
    fn line_down(&self) -> bool {
        unsafe { ffi::wxWindow_LineDown(self.as_ptr()) }
    }
    fn page_up(&self) -> bool {
        unsafe { ffi::wxWindow_PageUp(self.as_ptr()) }
    }
    fn page_down(&self) -> bool {
        unsafe { ffi::wxWindow_PageDown(self.as_ptr()) }
    }
    fn set_scroll_pos(&self, orientation: c_int, pos: c_int, refresh: bool) {
        unsafe { ffi::wxWindow_SetScrollPos(self.as_ptr(), orientation, pos, refresh) }
    }
    fn set_scrollbar(
        &self,
        orientation: c_int,
        position: c_int,
        thumb_size: c_int,
        range: c_int,
        refresh: bool,
    ) {
        unsafe {
            ffi::wxWindow_SetScrollbar(
                self.as_ptr(),
                orientation,
                position,
                thumb_size,
                range,
                refresh,
            )
        }
    }
    fn begin_repositioning_children(&self) -> bool {
        unsafe { ffi::wxWindow_BeginRepositioningChildren(self.as_ptr()) }
    }
    fn end_repositioning_children(&self) {
        unsafe { ffi::wxWindow_EndRepositioningChildren(self.as_ptr()) }
    }
    fn cache_best_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_CacheBestSize(self.as_ptr(), size)
        }
    }
    fn client_to_window_size<S: SizeMethods>(&self, size: &S) -> Size {
        unsafe {
            let size = size.as_ptr();
            SizeIsOwned(ffi::wxWindow_ClientToWindowSize(self.as_ptr(), size))
        }
    }
    fn window_to_client_size<S: SizeMethods>(&self, size: &S) -> Size {
        unsafe {
            let size = size.as_ptr();
            SizeIsOwned(ffi::wxWindow_WindowToClientSize(self.as_ptr(), size))
        }
    }
    fn fit(&self) {
        unsafe { ffi::wxWindow_Fit(self.as_ptr()) }
    }
    fn fit_inside(&self) {
        unsafe { ffi::wxWindow_FitInside(self.as_ptr()) }
    }
    fn from_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            SizeIsOwned(ffi::wxWindow_FromDIP(self.as_ptr(), sz))
        }
    }
    fn from_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            PointIsOwned(ffi::wxWindow_FromDIP1(self.as_ptr(), pt))
        }
    }
    fn from_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxWindow_FromDIP2(self.as_ptr(), d) }
    }
    fn to_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            SizeIsOwned(ffi::wxWindow_ToDIP(self.as_ptr(), sz))
        }
    }
    fn to_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            PointIsOwned(ffi::wxWindow_ToDIP1(self.as_ptr(), pt))
        }
    }
    fn to_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxWindow_ToDIP2(self.as_ptr(), d) }
    }
    fn get_best_size(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxWindow_GetBestSize(self.as_ptr())) }
    }
    fn get_best_height(&self, width: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetBestHeight(self.as_ptr(), width) }
    }
    fn get_best_width(&self, height: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetBestWidth(self.as_ptr(), height) }
    }
    fn get_client_size_int(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetClientSize(self.as_ptr(), width, height) }
    }
    fn get_client_size(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxWindow_GetClientSize1(self.as_ptr())) }
    }
    fn get_effective_min_size(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxWindow_GetEffectiveMinSize(self.as_ptr())) }
    }
    fn get_max_client_size(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxWindow_GetMaxClientSize(self.as_ptr())) }
    }
    fn get_max_size(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxWindow_GetMaxSize(self.as_ptr())) }
    }
    fn get_min_client_size(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxWindow_GetMinClientSize(self.as_ptr())) }
    }
    fn get_min_size(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxWindow_GetMinSize(self.as_ptr())) }
    }
    fn get_min_width(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMinWidth(self.as_ptr()) }
    }
    fn get_min_height(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMinHeight(self.as_ptr()) }
    }
    fn get_max_width(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMaxWidth(self.as_ptr()) }
    }
    fn get_max_height(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMaxHeight(self.as_ptr()) }
    }
    fn get_size_int(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetSize(self.as_ptr(), width, height) }
    }
    fn get_size(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxWindow_GetSize1(self.as_ptr())) }
    }
    fn get_virtual_size(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxWindow_GetVirtualSize(self.as_ptr())) }
    }
    fn get_virtual_size_int(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetVirtualSize1(self.as_ptr(), width, height) }
    }
    fn get_best_virtual_size(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxWindow_GetBestVirtualSize(self.as_ptr())) }
    }
    fn get_content_scale_factor(&self) -> c_double {
        unsafe { ffi::wxWindow_GetContentScaleFactor(self.as_ptr()) }
    }
    fn get_dpi_scale_factor(&self) -> c_double {
        unsafe { ffi::wxWindow_GetDPIScaleFactor(self.as_ptr()) }
    }
    fn get_window_border_size(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxWindow_GetWindowBorderSize(self.as_ptr())) }
    }
    fn inform_first_direction(
        &self,
        direction: c_int,
        size: c_int,
        available_other_dir: c_int,
    ) -> bool {
        unsafe {
            ffi::wxWindow_InformFirstDirection(self.as_ptr(), direction, size, available_other_dir)
        }
    }
    fn invalidate_best_size(&self) {
        unsafe { ffi::wxWindow_InvalidateBestSize(self.as_ptr()) }
    }
    fn post_size_event(&self) {
        unsafe { ffi::wxWindow_PostSizeEvent(self.as_ptr()) }
    }
    fn post_size_event_to_parent(&self) {
        unsafe { ffi::wxWindow_PostSizeEventToParent(self.as_ptr()) }
    }
    fn send_size_event(&self, flags: c_int) {
        unsafe { ffi::wxWindow_SendSizeEvent(self.as_ptr(), flags) }
    }
    fn send_size_event_to_parent(&self, flags: c_int) {
        unsafe { ffi::wxWindow_SendSizeEventToParent(self.as_ptr(), flags) }
    }
    fn set_client_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxWindow_SetClientSize(self.as_ptr(), width, height) }
    }
    fn set_client_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetClientSize1(self.as_ptr(), size)
        }
    }
    fn set_client_size_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_SetClientSize2(self.as_ptr(), rect)
        }
    }
    fn set_containing_sizer<S: SizerMethods>(&self, sizer: Option<&S>) {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetContainingSizer(self.as_ptr(), sizer)
        }
    }
    fn set_initial_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetInitialSize(self.as_ptr(), size)
        }
    }
    fn set_max_client_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMaxClientSize(self.as_ptr(), size)
        }
    }
    fn set_max_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMaxSize(self.as_ptr(), size)
        }
    }
    fn set_min_client_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMinClientSize(self.as_ptr(), size)
        }
    }
    fn set_min_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMinSize(self.as_ptr(), size)
        }
    }
    fn set_size_int_int(&self, x: c_int, y: c_int, width: c_int, height: c_int, size_flags: c_int) {
        unsafe { ffi::wxWindow_SetSize(self.as_ptr(), x, y, width, height, size_flags) }
    }
    fn set_size_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_SetSize1(self.as_ptr(), rect)
        }
    }
    fn set_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetSize2(self.as_ptr(), size)
        }
    }
    fn set_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxWindow_SetSize3(self.as_ptr(), width, height) }
    }
    fn set_size_hints_size<S: SizeMethods, S2: SizeMethods, S3: SizeMethods>(
        &self,
        min_size: &S,
        max_size: &S2,
        inc_size: &S3,
    ) {
        unsafe {
            let min_size = min_size.as_ptr();
            let max_size = max_size.as_ptr();
            let inc_size = inc_size.as_ptr();
            ffi::wxWindow_SetSizeHints(self.as_ptr(), min_size, max_size, inc_size)
        }
    }
    fn set_size_hints_int(
        &self,
        min_w: c_int,
        min_h: c_int,
        max_w: c_int,
        max_h: c_int,
        inc_w: c_int,
        inc_h: c_int,
    ) {
        unsafe {
            ffi::wxWindow_SetSizeHints1(self.as_ptr(), min_w, min_h, max_w, max_h, inc_w, inc_h)
        }
    }
    fn set_virtual_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxWindow_SetVirtualSize(self.as_ptr(), width, height) }
    }
    fn set_virtual_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetVirtualSize1(self.as_ptr(), size)
        }
    }
    fn from_dip_size_window<S: SizeMethods, W: WindowMethods>(sz: &S, w: Option<&W>) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizeIsOwned(ffi::wxWindow_FromDIP3(sz, w))
        }
    }
    fn from_dip_point_window<P: PointMethods, W: WindowMethods>(pt: &P, w: Option<&W>) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            PointIsOwned(ffi::wxWindow_FromDIP4(pt, w))
        }
    }
    fn from_dip_int_window<W: WindowMethods>(d: c_int, w: Option<&W>) -> c_int {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FromDIP5(d, w)
        }
    }
    fn to_dip_size_window<S: SizeMethods, W: WindowMethods>(sz: &S, w: Option<&W>) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizeIsOwned(ffi::wxWindow_ToDIP3(sz, w))
        }
    }
    fn to_dip_point_window<P: PointMethods, W: WindowMethods>(pt: &P, w: Option<&W>) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            PointIsOwned(ffi::wxWindow_ToDIP4(pt, w))
        }
    }
    fn to_dip_int_window<W: WindowMethods>(d: c_int, w: Option<&W>) -> c_int {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_ToDIP5(d, w)
        }
    }
    fn center(&self, dir: c_int) {
        unsafe { ffi::wxWindow_Center(self.as_ptr(), dir) }
    }
    fn center_on_parent(&self, dir: c_int) {
        unsafe { ffi::wxWindow_CenterOnParent(self.as_ptr(), dir) }
    }
    fn centre(&self, direction: c_int) {
        unsafe { ffi::wxWindow_Centre(self.as_ptr(), direction) }
    }
    fn centre_on_parent(&self, direction: c_int) {
        unsafe { ffi::wxWindow_CentreOnParent(self.as_ptr(), direction) }
    }
    fn get_position_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_GetPosition(self.as_ptr(), x, y) }
    }
    fn get_position(&self) -> Point {
        unsafe { PointIsOwned(ffi::wxWindow_GetPosition1(self.as_ptr())) }
    }
    fn get_rect(&self) -> Rect {
        unsafe { RectIsOwned(ffi::wxWindow_GetRect(self.as_ptr())) }
    }
    fn get_screen_position_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_GetScreenPosition(self.as_ptr(), x, y) }
    }
    fn get_screen_position(&self) -> Point {
        unsafe { PointIsOwned(ffi::wxWindow_GetScreenPosition1(self.as_ptr())) }
    }
    fn get_screen_rect(&self) -> Rect {
        unsafe { RectIsOwned(ffi::wxWindow_GetScreenRect(self.as_ptr())) }
    }
    fn get_client_area_origin(&self) -> Point {
        unsafe { PointIsOwned(ffi::wxWindow_GetClientAreaOrigin(self.as_ptr())) }
    }
    fn get_client_rect(&self) -> Rect {
        unsafe { RectIsOwned(ffi::wxWindow_GetClientRect(self.as_ptr())) }
    }
    fn move_int(&self, x: c_int, y: c_int, flags: c_int) {
        unsafe { ffi::wxWindow_Move(self.as_ptr(), x, y, flags) }
    }
    fn move_point<P: PointMethods>(&self, pt: &P, flags: c_int) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxWindow_Move1(self.as_ptr(), pt, flags)
        }
    }
    fn set_position<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxWindow_SetPosition(self.as_ptr(), pt)
        }
    }
    fn client_to_screen_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_ClientToScreen(self.as_ptr(), x, y) }
    }
    fn client_to_screen_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            PointIsOwned(ffi::wxWindow_ClientToScreen1(self.as_ptr(), pt))
        }
    }
    fn convert_dialog_to_pixels_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            PointIsOwned(ffi::wxWindow_ConvertDialogToPixels(self.as_ptr(), pt))
        }
    }
    fn convert_dialog_to_pixels_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            SizeIsOwned(ffi::wxWindow_ConvertDialogToPixels1(self.as_ptr(), sz))
        }
    }
    fn convert_pixels_to_dialog_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            PointIsOwned(ffi::wxWindow_ConvertPixelsToDialog(self.as_ptr(), pt))
        }
    }
    fn convert_pixels_to_dialog_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            SizeIsOwned(ffi::wxWindow_ConvertPixelsToDialog1(self.as_ptr(), sz))
        }
    }
    fn screen_to_client_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_ScreenToClient(self.as_ptr(), x, y) }
    }
    fn screen_to_client_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            PointIsOwned(ffi::wxWindow_ScreenToClient1(self.as_ptr(), pt))
        }
    }
    fn clear_background(&self) {
        unsafe { ffi::wxWindow_ClearBackground(self.as_ptr()) }
    }
    fn freeze(&self) {
        unsafe { ffi::wxWindow_Freeze(self.as_ptr()) }
    }
    fn thaw(&self) {
        unsafe { ffi::wxWindow_Thaw(self.as_ptr()) }
    }
    fn is_frozen(&self) -> bool {
        unsafe { ffi::wxWindow_IsFrozen(self.as_ptr()) }
    }
    fn get_background_colour(&self) -> Colour {
        unsafe { ColourIsOwned(ffi::wxWindow_GetBackgroundColour(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetBackgroundStyle()
    fn get_char_height(&self) -> c_int {
        unsafe { ffi::wxWindow_GetCharHeight(self.as_ptr()) }
    }
    fn get_char_width(&self) -> c_int {
        unsafe { ffi::wxWindow_GetCharWidth(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetDefaultAttributes()
    fn get_dpi(&self) -> Size {
        unsafe { SizeIsOwned(ffi::wxWindow_GetDPI(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetFont()
    fn get_foreground_colour(&self) -> Colour {
        unsafe { ColourIsOwned(ffi::wxWindow_GetForegroundColour(self.as_ptr())) }
    }
    fn get_text_extent_int(
        &self,
        string: &str,
        w: *mut c_void,
        h: *mut c_void,
        descent: *mut c_void,
        external_leading: *mut c_void,
        font: *const c_void,
    ) {
        unsafe {
            let string = wx_base::wx_string_from(string);
            ffi::wxWindow_GetTextExtent(
                self.as_ptr(),
                string,
                w,
                h,
                descent,
                external_leading,
                font,
            )
        }
    }
    fn get_text_extent(&self, string: &str) -> Size {
        unsafe {
            let string = wx_base::wx_string_from(string);
            SizeIsOwned(ffi::wxWindow_GetTextExtent1(self.as_ptr(), string))
        }
    }
    // BLOCKED: fn GetUpdateRegion()
    fn get_update_client_rect(&self) -> Rect {
        unsafe { RectIsOwned(ffi::wxWindow_GetUpdateClientRect(self.as_ptr())) }
    }
    fn has_transparent_background(&self) -> bool {
        unsafe { ffi::wxWindow_HasTransparentBackground(self.as_ptr()) }
    }
    fn refresh<R: RectMethods>(&self, erase_background: bool, rect: Option<&R>) {
        unsafe {
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_Refresh(self.as_ptr(), erase_background, rect)
        }
    }
    fn refresh_rect<R: RectMethods>(&self, rect: &R, erase_background: bool) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_RefreshRect(self.as_ptr(), rect, erase_background)
        }
    }
    fn update(&self) {
        unsafe { ffi::wxWindow_Update(self.as_ptr()) }
    }
    fn set_background_colour<C: ColourMethods>(&self, colour: &C) -> bool {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWindow_SetBackgroundColour(self.as_ptr(), colour)
        }
    }
    // NOT_SUPPORTED: fn SetBackgroundStyle()
    fn is_transparent_background_supported(&self, reason: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_IsTransparentBackgroundSupported(self.as_ptr(), reason) }
    }
    fn set_font(&self, font: *const c_void) -> bool {
        unsafe { ffi::wxWindow_SetFont(self.as_ptr(), font) }
    }
    fn set_foreground_colour<C: ColourMethods>(&self, colour: &C) -> bool {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWindow_SetForegroundColour(self.as_ptr(), colour)
        }
    }
    fn set_own_background_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWindow_SetOwnBackgroundColour(self.as_ptr(), colour)
        }
    }
    fn inherits_background_colour(&self) -> bool {
        unsafe { ffi::wxWindow_InheritsBackgroundColour(self.as_ptr()) }
    }
    fn use_bg_col(&self) -> bool {
        unsafe { ffi::wxWindow_UseBgCol(self.as_ptr()) }
    }
    fn use_background_colour(&self) -> bool {
        unsafe { ffi::wxWindow_UseBackgroundColour(self.as_ptr()) }
    }
    fn set_own_font(&self, font: *const c_void) {
        unsafe { ffi::wxWindow_SetOwnFont(self.as_ptr(), font) }
    }
    fn set_own_foreground_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWindow_SetOwnForegroundColour(self.as_ptr(), colour)
        }
    }
    fn use_foreground_colour(&self) -> bool {
        unsafe { ffi::wxWindow_UseForegroundColour(self.as_ptr()) }
    }
    fn inherits_foreground_colour(&self) -> bool {
        unsafe { ffi::wxWindow_InheritsForegroundColour(self.as_ptr()) }
    }
    fn set_palette(&self, pal: *const c_void) {
        unsafe { ffi::wxWindow_SetPalette(self.as_ptr(), pal) }
    }
    fn should_inherit_colours(&self) -> bool {
        unsafe { ffi::wxWindow_ShouldInheritColours(self.as_ptr()) }
    }
    fn set_theme_enabled(&self, enable: bool) {
        unsafe { ffi::wxWindow_SetThemeEnabled(self.as_ptr(), enable) }
    }
    fn get_theme_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_GetThemeEnabled(self.as_ptr()) }
    }
    fn can_set_transparent(&self) -> bool {
        unsafe { ffi::wxWindow_CanSetTransparent(self.as_ptr()) }
    }
    fn set_transparent(&self, alpha: c_uchar) -> bool {
        unsafe { ffi::wxWindow_SetTransparent(self.as_ptr(), alpha) }
    }
    fn get_event_handler(&self) -> WeakRef<EvtHandler> {
        unsafe { WeakRef::<EvtHandler>::from(ffi::wxWindow_GetEventHandler(self.as_ptr())) }
    }
    fn handle_as_navigation_key(&self, event: *const c_void) -> bool {
        unsafe { ffi::wxWindow_HandleAsNavigationKey(self.as_ptr(), event) }
    }
    fn handle_window_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_HandleWindowEvent(self.as_ptr(), event) }
    }
    fn process_window_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_ProcessWindowEvent(self.as_ptr(), event) }
    }
    fn process_window_event_locally(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_ProcessWindowEventLocally(self.as_ptr(), event) }
    }
    fn pop_event_handler(&self, delete_handler: bool) -> WeakRef<EvtHandler> {
        unsafe {
            WeakRef::<EvtHandler>::from(ffi::wxWindow_PopEventHandler(
                self.as_ptr(),
                delete_handler,
            ))
        }
    }
    fn push_event_handler<E: EvtHandlerMethods>(&self, handler: Option<&E>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_PushEventHandler(self.as_ptr(), handler)
        }
    }
    fn remove_event_handler<E: EvtHandlerMethods>(&self, handler: Option<&E>) -> bool {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_RemoveEventHandler(self.as_ptr(), handler)
        }
    }
    fn set_event_handler<E: EvtHandlerMethods>(&self, handler: Option<&E>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetEventHandler(self.as_ptr(), handler)
        }
    }
    fn get_extra_style(&self) -> c_long {
        unsafe { ffi::wxWindow_GetExtraStyle(self.as_ptr()) }
    }
    fn get_window_style_flag(&self) -> c_long {
        unsafe { ffi::wxWindow_GetWindowStyleFlag(self.as_ptr()) }
    }
    fn get_window_style(&self) -> c_long {
        unsafe { ffi::wxWindow_GetWindowStyle(self.as_ptr()) }
    }
    fn has_extra_style(&self, ex_flag: c_int) -> bool {
        unsafe { ffi::wxWindow_HasExtraStyle(self.as_ptr(), ex_flag) }
    }
    fn has_flag(&self, flag: c_int) -> bool {
        unsafe { ffi::wxWindow_HasFlag(self.as_ptr(), flag) }
    }
    fn set_extra_style(&self, ex_style: c_long) {
        unsafe { ffi::wxWindow_SetExtraStyle(self.as_ptr(), ex_style) }
    }
    fn set_window_style_flag(&self, style: c_long) {
        unsafe { ffi::wxWindow_SetWindowStyleFlag(self.as_ptr(), style) }
    }
    fn set_window_style(&self, style: c_long) {
        unsafe { ffi::wxWindow_SetWindowStyle(self.as_ptr(), style) }
    }
    fn toggle_window_style(&self, flag: c_int) -> bool {
        unsafe { ffi::wxWindow_ToggleWindowStyle(self.as_ptr(), flag) }
    }
    fn move_after_in_tab_order<W: WindowMethods>(&self, win: Option<&W>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_MoveAfterInTabOrder(self.as_ptr(), win)
        }
    }
    fn move_before_in_tab_order<W: WindowMethods>(&self, win: Option<&W>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_MoveBeforeInTabOrder(self.as_ptr(), win)
        }
    }
    fn navigate(&self, flags: c_int) -> bool {
        unsafe { ffi::wxWindow_Navigate(self.as_ptr(), flags) }
    }
    fn navigate_in(&self, flags: c_int) -> bool {
        unsafe { ffi::wxWindow_NavigateIn(self.as_ptr(), flags) }
    }
    fn lower(&self) {
        unsafe { ffi::wxWindow_Lower(self.as_ptr()) }
    }
    fn raise(&self) {
        unsafe { ffi::wxWindow_Raise(self.as_ptr()) }
    }
    fn hide(&self) -> bool {
        unsafe { ffi::wxWindow_Hide(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn HideWithEffect()
    fn is_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_IsEnabled(self.as_ptr()) }
    }
    fn is_exposed_int(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxWindow_IsExposed(self.as_ptr(), x, y) }
    }
    fn is_exposed_point(&self, pt: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_IsExposed1(self.as_ptr(), pt) }
    }
    fn is_exposed_int_int(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> bool {
        unsafe { ffi::wxWindow_IsExposed2(self.as_ptr(), x, y, w, h) }
    }
    fn is_exposed_rect(&self, rect: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_IsExposed3(self.as_ptr(), rect) }
    }
    fn is_shown(&self) -> bool {
        unsafe { ffi::wxWindow_IsShown(self.as_ptr()) }
    }
    fn is_shown_on_screen(&self) -> bool {
        unsafe { ffi::wxWindow_IsShownOnScreen(self.as_ptr()) }
    }
    fn disable(&self) -> bool {
        unsafe { ffi::wxWindow_Disable(self.as_ptr()) }
    }
    fn enable(&self, enable: bool) -> bool {
        unsafe { ffi::wxWindow_Enable(self.as_ptr(), enable) }
    }
    fn show(&self, show: bool) -> bool {
        unsafe { ffi::wxWindow_Show(self.as_ptr(), show) }
    }
    // NOT_SUPPORTED: fn ShowWithEffect()
    fn get_help_text(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxWindow_GetHelpText(self.as_ptr())) }
    }
    fn set_help_text(&self, help_text: &str) {
        unsafe {
            let help_text = wx_base::wx_string_from(help_text);
            ffi::wxWindow_SetHelpText(self.as_ptr(), help_text)
        }
    }
    // NOT_SUPPORTED: fn GetHelpTextAtPoint()
    fn get_tool_tip(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetToolTip(self.as_ptr()) }
    }
    fn get_tool_tip_text(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxWindow_GetToolTipText(self.as_ptr())) }
    }
    fn set_tool_tip_str(&self, tip_string: &str) {
        unsafe {
            let tip_string = wx_base::wx_string_from(tip_string);
            ffi::wxWindow_SetToolTip(self.as_ptr(), tip_string)
        }
    }
    fn set_tool_tip_tooltip(&self, tip: *mut c_void) {
        unsafe { ffi::wxWindow_SetToolTip1(self.as_ptr(), tip) }
    }
    fn unset_tool_tip(&self) {
        unsafe { ffi::wxWindow_UnsetToolTip(self.as_ptr()) }
    }
    fn get_popup_menu_selection_from_user_point<P: PointMethods>(
        &self,
        menu: *mut c_void,
        pos: &P,
    ) -> c_int {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxWindow_GetPopupMenuSelectionFromUser(self.as_ptr(), menu, pos)
        }
    }
    fn get_popup_menu_selection_from_user_int(
        &self,
        menu: *mut c_void,
        x: c_int,
        y: c_int,
    ) -> c_int {
        unsafe { ffi::wxWindow_GetPopupMenuSelectionFromUser1(self.as_ptr(), menu, x, y) }
    }
    fn popup_menu_point<M: MenuMethods, P: PointMethods>(&self, menu: Option<&M>, pos: &P) -> bool {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            ffi::wxWindow_PopupMenu(self.as_ptr(), menu, pos)
        }
    }
    fn popup_menu_int<M: MenuMethods>(&self, menu: Option<&M>, x: c_int, y: c_int) -> bool {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_PopupMenu1(self.as_ptr(), menu, x, y)
        }
    }
    fn get_validator(&self) -> WeakRef<Validator> {
        unsafe { WeakRef::<Validator>::from(ffi::wxWindow_GetValidator(self.as_ptr())) }
    }
    fn set_validator<V: ValidatorMethods>(&self, validator: &V) {
        unsafe {
            let validator = validator.as_ptr();
            ffi::wxWindow_SetValidator(self.as_ptr(), validator)
        }
    }
    fn transfer_data_from_window(&self) -> bool {
        unsafe { ffi::wxWindow_TransferDataFromWindow(self.as_ptr()) }
    }
    fn transfer_data_to_window(&self) -> bool {
        unsafe { ffi::wxWindow_TransferDataToWindow(self.as_ptr()) }
    }
    fn validate(&self) -> bool {
        unsafe { ffi::wxWindow_Validate(self.as_ptr()) }
    }
    fn get_id(&self) -> c_int {
        unsafe { ffi::wxWindow_GetId(self.as_ptr()) }
    }
    fn get_label(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxWindow_GetLabel(self.as_ptr())) }
    }
    fn get_layout_direction(&self) -> c_int {
        unsafe { ffi::wxWindow_GetLayoutDirection(self.as_ptr()) }
    }
    fn adjust_for_layout_direction(&self, x: c_int, width: c_int, width_total: c_int) -> c_int {
        unsafe { ffi::wxWindow_AdjustForLayoutDirection(self.as_ptr(), x, width, width_total) }
    }
    fn get_name(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxWindow_GetName(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetWindowVariant()
    fn set_id(&self, winid: c_int) {
        unsafe { ffi::wxWindow_SetId(self.as_ptr(), winid) }
    }
    fn set_label(&self, label: &str) {
        unsafe {
            let label = wx_base::wx_string_from(label);
            ffi::wxWindow_SetLabel(self.as_ptr(), label)
        }
    }
    fn set_layout_direction(&self, dir: c_int) {
        unsafe { ffi::wxWindow_SetLayoutDirection(self.as_ptr(), dir) }
    }
    fn set_name(&self, name: &str) {
        unsafe {
            let name = wx_base::wx_string_from(name);
            ffi::wxWindow_SetName(self.as_ptr(), name)
        }
    }
    // NOT_SUPPORTED: fn SetWindowVariant()
    fn get_accelerator_table(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetAcceleratorTable(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetAccessible()
    fn set_accelerator_table(&self, accel: *const c_void) {
        unsafe { ffi::wxWindow_SetAcceleratorTable(self.as_ptr(), accel) }
    }
    // NOT_SUPPORTED: fn SetAccessible()
    fn close(&self, force: bool) -> bool {
        unsafe { ffi::wxWindow_Close(self.as_ptr(), force) }
    }
    fn destroy(&self) -> bool {
        unsafe { ffi::wxWindow_Destroy(self.as_ptr()) }
    }
    fn is_being_deleted(&self) -> bool {
        unsafe { ffi::wxWindow_IsBeingDeleted(self.as_ptr()) }
    }
    fn get_drop_target(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetDropTarget(self.as_ptr()) }
    }
    fn set_drop_target(&self, target: *mut c_void) {
        unsafe { ffi::wxWindow_SetDropTarget(self.as_ptr(), target) }
    }
    fn drag_accept_files(&self, accept: bool) {
        unsafe { ffi::wxWindow_DragAcceptFiles(self.as_ptr(), accept) }
    }
    fn get_containing_sizer(&self) -> Option<SizerIsOwned<false>> {
        unsafe { Sizer::option_from(ffi::wxWindow_GetContainingSizer(self.as_ptr())) }
    }
    fn get_sizer(&self) -> Option<SizerIsOwned<false>> {
        unsafe { Sizer::option_from(ffi::wxWindow_GetSizer(self.as_ptr())) }
    }
    fn set_sizer<S: SizerMethods>(&self, sizer: Option<&S>, delete_old: bool) {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetSizer(self.as_ptr(), sizer, delete_old)
        }
    }
    fn set_sizer_and_fit<S: SizerMethods>(&self, sizer: Option<&S>, delete_old: bool) {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetSizerAndFit(self.as_ptr(), sizer, delete_old)
        }
    }
    fn get_constraints(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetConstraints(self.as_ptr()) }
    }
    fn set_constraints(&self, constraints: *mut c_void) {
        unsafe { ffi::wxWindow_SetConstraints(self.as_ptr(), constraints) }
    }
    fn layout(&self) -> bool {
        unsafe { ffi::wxWindow_Layout(self.as_ptr()) }
    }
    fn set_auto_layout(&self, auto_layout: bool) {
        unsafe { ffi::wxWindow_SetAutoLayout(self.as_ptr(), auto_layout) }
    }
    fn get_auto_layout(&self) -> bool {
        unsafe { ffi::wxWindow_GetAutoLayout(self.as_ptr()) }
    }
    fn capture_mouse(&self) {
        unsafe { ffi::wxWindow_CaptureMouse(self.as_ptr()) }
    }
    fn get_caret(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetCaret(self.as_ptr()) }
    }
    // BLOCKED: fn GetCursor()
    fn has_capture(&self) -> bool {
        unsafe { ffi::wxWindow_HasCapture(self.as_ptr()) }
    }
    fn release_mouse(&self) {
        unsafe { ffi::wxWindow_ReleaseMouse(self.as_ptr()) }
    }
    fn set_caret(&self, caret: *mut c_void) {
        unsafe { ffi::wxWindow_SetCaret(self.as_ptr(), caret) }
    }
    fn set_cursor(&self, cursor: *const c_void) -> bool {
        unsafe { ffi::wxWindow_SetCursor(self.as_ptr(), cursor) }
    }
    fn warp_pointer(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxWindow_WarpPointer(self.as_ptr(), x, y) }
    }
    fn enable_touch_events(&self, events_mask: c_int) -> bool {
        unsafe { ffi::wxWindow_EnableTouchEvents(self.as_ptr(), events_mask) }
    }
    // NOT_SUPPORTED: fn HitTest()
    // NOT_SUPPORTED: fn HitTest1()
    // NOT_SUPPORTED: fn GetBorder()
    // NOT_SUPPORTED: fn GetBorder1()
    fn do_update_window_ui(&self, event: *mut c_void) {
        unsafe { ffi::wxWindow_DoUpdateWindowUI(self.as_ptr(), event) }
    }
    // NOT_SUPPORTED: fn GetHandle()
    fn has_multiple_pages(&self) -> bool {
        unsafe { ffi::wxWindow_HasMultiplePages(self.as_ptr()) }
    }
    fn inherit_attributes(&self) {
        unsafe { ffi::wxWindow_InheritAttributes(self.as_ptr()) }
    }
    fn init_dialog(&self) {
        unsafe { ffi::wxWindow_InitDialog(self.as_ptr()) }
    }
    fn is_double_buffered(&self) -> bool {
        unsafe { ffi::wxWindow_IsDoubleBuffered(self.as_ptr()) }
    }
    fn set_double_buffered(&self, on: bool) {
        unsafe { ffi::wxWindow_SetDoubleBuffered(self.as_ptr(), on) }
    }
    fn is_retained(&self) -> bool {
        unsafe { ffi::wxWindow_IsRetained(self.as_ptr()) }
    }
    fn is_this_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_IsThisEnabled(self.as_ptr()) }
    }
    fn is_top_level(&self) -> bool {
        unsafe { ffi::wxWindow_IsTopLevel(self.as_ptr()) }
    }
    fn on_internal_idle(&self) {
        unsafe { ffi::wxWindow_OnInternalIdle(self.as_ptr()) }
    }
    fn send_idle_events(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_SendIdleEvents(self.as_ptr(), event) }
    }
    fn register_hot_key(
        &self,
        hotkey_id: c_int,
        modifiers: c_int,
        virtual_key_code: c_int,
    ) -> bool {
        unsafe {
            ffi::wxWindow_RegisterHotKey(self.as_ptr(), hotkey_id, modifiers, virtual_key_code)
        }
    }
    fn unregister_hot_key(&self, hotkey_id: c_int) -> bool {
        unsafe { ffi::wxWindow_UnregisterHotKey(self.as_ptr(), hotkey_id) }
    }
    fn update_window_ui(&self, flags: c_long) {
        unsafe { ffi::wxWindow_UpdateWindowUI(self.as_ptr(), flags) }
    }
    // NOT_SUPPORTED: fn GetClassDefaultAttributes()
    fn find_focus() -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_FindFocus()) }
    }
    fn find_window_by_id<W: WindowMethods>(id: c_long, parent: Option<&W>) -> WeakRef<Window> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<Window>::from(ffi::wxWindow_FindWindowById(id, parent))
        }
    }
    fn find_window_by_label<W: WindowMethods>(label: &str, parent: Option<&W>) -> WeakRef<Window> {
        unsafe {
            let label = wx_base::wx_string_from(label);
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<Window>::from(ffi::wxWindow_FindWindowByLabel(label, parent))
        }
    }
    fn find_window_by_name<W: WindowMethods>(name: &str, parent: Option<&W>) -> WeakRef<Window> {
        unsafe {
            let name = wx_base::wx_string_from(name);
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<Window>::from(ffi::wxWindow_FindWindowByName(name, parent))
        }
    }
    fn get_capture() -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetCapture()) }
    }
    fn new_control_id(count: c_int) -> c_int {
        unsafe { ffi::wxWindow_NewControlId(count) }
    }
    fn unreserve_control_id(id: c_int, count: c_int) {
        unsafe { ffi::wxWindow_UnreserveControlId(id, count) }
    }
    // DTOR: fn ~wxWindow()
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = wx_base::wx_string_from(name);
            ffi::wxWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxWrapSizer
pub trait WrapSizerMethods: BoxSizerMethods {}
