use std::os::raw::{c_int, c_long, c_uint, c_void};

use super::*;
use crate::WeakRef;

pub use wx_base::methods::*;

// wxBitmap
pub trait BitmapMethods: GDIObjectMethods {
    // DTOR: fn ~wxBitmap()
    // NOT_SUPPORTED: fn ConvertToDisabled()
    // NOT_SUPPORTED: fn ConvertToImage()
    fn copy_from_icon<I: IconMethods>(&self, icon: &I) -> bool {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxBitmap_CopyFromIcon(self.as_ptr(), icon)
        }
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
    fn create_with_dip_size_size<S: SizeMethods>(
        &self,
        size: &S,
        scale: c_double,
        depth: c_int,
    ) -> bool {
        unsafe {
            let size = size.as_ptr();
            ffi::wxBitmap_CreateWithDIPSize(self.as_ptr(), size, scale, depth)
        }
    }
    fn create_with_dip_size_int(
        &self,
        width: c_int,
        height: c_int,
        scale: c_double,
        depth: c_int,
    ) -> bool {
        unsafe { ffi::wxBitmap_CreateWithDIPSize1(self.as_ptr(), width, height, scale, depth) }
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
    fn get_dip_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmap_GetDIPSize(self.as_ptr())) }
    }
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxBitmap_GetHeight(self.as_ptr()) }
    }
    fn get_logical_height(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetLogicalHeight(self.as_ptr()) }
    }
    fn get_logical_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmap_GetLogicalSize(self.as_ptr())) }
    }
    fn get_logical_width(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetLogicalWidth(self.as_ptr()) }
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
            Bitmap::from_ptr(ffi::wxBitmap_GetSubBitmap(self.as_ptr(), rect))
        }
    }
    fn get_scale_factor(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetScaleFactor(self.as_ptr()) }
    }
    fn get_scaled_height(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetScaledHeight(self.as_ptr()) }
    }
    fn get_scaled_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmap_GetScaledSize(self.as_ptr())) }
    }
    fn get_scaled_width(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetScaledWidth(self.as_ptr()) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmap_GetSize(self.as_ptr())) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxBitmap_GetWidth(self.as_ptr()) }
    }
    fn has_alpha(&self) -> bool {
        unsafe { ffi::wxBitmap_HasAlpha(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxBitmap_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn LoadFile()
    // BLOCKED: fn ResetAlpha()
    // NOT_SUPPORTED: fn SaveFile()
    fn set_depth(&self, depth: c_int) {
        unsafe { ffi::wxBitmap_SetDepth(self.as_ptr(), depth) }
    }
    fn set_height(&self, height: c_int) {
        unsafe { ffi::wxBitmap_SetHeight(self.as_ptr(), height) }
    }
    fn set_scale_factor(&self, scale: c_double) {
        unsafe { ffi::wxBitmap_SetScaleFactor(self.as_ptr(), scale) }
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
    // BLOCKED: fn UseAlpha()
    fn add_handler(handler: *mut c_void) {
        unsafe { ffi::wxBitmap_AddHandler(handler) }
    }
    fn clean_up_handlers() {
        unsafe { ffi::wxBitmap_CleanUpHandlers() }
    }
    fn find_handler(name: &str) -> *mut c_void {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
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
        unsafe { Bitmap::from_ptr(ffi::wxBitmap_NewFromPNGData(data, size)) }
    }
    fn remove_handler(name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxBitmap_RemoveHandler(name)
        }
    }
    fn rescale<B: BitmapMethods, S: SizeMethods>(bmp: &B, size_needed: &S) {
        unsafe {
            let bmp = bmp.as_ptr();
            let size_needed = size_needed.as_ptr();
            ffi::wxBitmap_Rescale(bmp, size_needed)
        }
    }
}

// wxBitmapBundle
pub trait BitmapBundleMethods: WxRustMethods {
    // BLOCKED: fn operator=()
    fn clear(&self) {
        unsafe { ffi::wxBitmapBundle_Clear(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxBitmapBundle_IsOk(self.as_ptr()) }
    }
    fn get_default_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmapBundle_GetDefaultSize(self.as_ptr())) }
    }
    fn get_preferred_bitmap_size_at_scale(&self, scale: c_double) -> Size {
        unsafe {
            Size::from_ptr(ffi::wxBitmapBundle_GetPreferredBitmapSizeAtScale(
                self.as_ptr(),
                scale,
            ))
        }
    }
    fn get_preferred_bitmap_size_for<W: WindowMethods>(&self, window: Option<&W>) -> Size {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxBitmapBundle_GetPreferredBitmapSizeFor(
                self.as_ptr(),
                window,
            ))
        }
    }
    fn get_preferred_logical_size_for<W: WindowMethods>(&self, window: Option<&W>) -> Size {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxBitmapBundle_GetPreferredLogicalSizeFor(
                self.as_ptr(),
                window,
            ))
        }
    }
    fn get_bitmap<S: SizeMethods>(&self, size: &S) -> Bitmap {
        unsafe {
            let size = size.as_ptr();
            Bitmap::from_ptr(ffi::wxBitmapBundle_GetBitmap(self.as_ptr(), size))
        }
    }
    fn get_bitmap_for<W: WindowMethods>(&self, window: Option<&W>) -> Bitmap {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Bitmap::from_ptr(ffi::wxBitmapBundle_GetBitmapFor(self.as_ptr(), window))
        }
    }
    fn get_icon<S: SizeMethods>(&self, size: &S) -> Icon {
        unsafe {
            let size = size.as_ptr();
            Icon::from_ptr(ffi::wxBitmapBundle_GetIcon(self.as_ptr(), size))
        }
    }
    fn get_icon_for<W: WindowMethods>(&self, window: Option<&W>) -> Icon {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Icon::from_ptr(ffi::wxBitmapBundle_GetIconFor(self.as_ptr(), window))
        }
    }
    fn is_same_as<B: BitmapBundleMethods>(&self, other: &B) -> bool {
        unsafe {
            let other = other.as_ptr();
            ffi::wxBitmapBundle_IsSameAs(self.as_ptr(), other)
        }
    }
    // BLOCKED: fn FromBitmaps()
    fn from_bitmaps<B: BitmapMethods, B2: BitmapMethods>(
        bitmap1: &B,
        bitmap2: &B2,
    ) -> BitmapBundle {
        unsafe {
            let bitmap1 = bitmap1.as_ptr();
            let bitmap2 = bitmap2.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromBitmaps1(bitmap1, bitmap2))
        }
    }
    fn from_bitmap<B: BitmapMethods>(bitmap: &B) -> BitmapBundle {
        unsafe {
            let bitmap = bitmap.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromBitmap(bitmap))
        }
    }
    fn from_icon_bundle(icon_bundle: *const c_void) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromIconBundle(icon_bundle)) }
    }
    fn from_image(image: *const c_void) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromImage(image)) }
    }
    fn from_impl(impl_: *mut c_void) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromImpl(impl_)) }
    }
    fn from_resources(name: &str) -> BitmapBundle {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromResources(name))
        }
    }
    fn from_files_str(path: &str, filename: &str, extension: &str) -> BitmapBundle {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            let extension = WxString::from(extension);
            let extension = extension.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromFiles(path, filename, extension))
        }
    }
    fn from_files(fullpathname: &str) -> BitmapBundle {
        unsafe {
            let fullpathname = WxString::from(fullpathname);
            let fullpathname = fullpathname.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromFiles1(fullpathname))
        }
    }
    // BLOCKED: fn FromSVG()
    fn from_svg<S: SizeMethods>(data: *const c_void, size_def: &S) -> BitmapBundle {
        unsafe {
            let size_def = size_def.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromSVG1(data, size_def))
        }
    }
    fn from_svg_file<S: SizeMethods>(path: &str, size_def: &S) -> BitmapBundle {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            let size_def = size_def.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromSVGFile(path, size_def))
        }
    }
    fn from_svg_resource<S: SizeMethods>(name: &str, size_def: &S) -> BitmapBundle {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let size_def = size_def.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromSVGResource(name, size_def))
        }
    }
}

// wxBitmapButton
pub trait BitmapButtonMethods: ButtonMethods {
    fn create_bitmapbundle<
        W: WindowMethods,
        B: BitmapBundleMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        bitmap: &B,
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
            let bitmap = bitmap.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxBitmapButton_Create(
                self.as_ptr(),
                parent,
                id,
                bitmap,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn create_close_button<W: WindowMethods>(
        &self,
        parent: Option<&W>,
        winid: c_int,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxBitmapButton_CreateCloseButton(self.as_ptr(), parent, winid, name)
        }
    }
    fn new_close_button<W: WindowMethods>(
        parent: Option<&W>,
        winid: c_int,
        name: &str,
    ) -> WeakRef<BitmapButton> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let name = WxString::from(name);
            let name = name.as_ptr();
            WeakRef::<BitmapButton>::from(ffi::wxBitmapButton_NewCloseButton(parent, winid, name))
        }
    }
}

// wxBitmapToggleButton
pub trait BitmapToggleButtonMethods: ToggleButtonMethods {
    fn create_bitmapbundle<
        W: WindowMethods,
        B: BitmapBundleMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &B,
        pos: &P,
        size: &S,
        style: c_long,
        val: &V,
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
            let val = val.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxBitmapToggleButton_Create(
                self.as_ptr(),
                parent,
                id,
                label,
                pos,
                size,
                style,
                val,
                name,
            )
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
        unsafe { WxString::from_ptr(ffi::wxBookCtrlBase_GetPageText(self.as_ptr(), n_page)).into() }
    }
    fn set_page_text(&self, page: usize, text: &str) -> bool {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
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
            let text = WxString::from(text);
            let text = text.as_ptr();
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
            let text = WxString::from(text);
            let text = text.as_ptr();
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
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
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
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
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
            Size::from_ptr(ffi::wxButton_GetDefaultSize(win))
        }
    }
}
