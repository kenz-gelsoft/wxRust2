#![allow(non_upper_case_globals)]

use super::*;

// wxPCXHandler
wx_class! { PCXHandler =
    PCXHandlerIsOwned<true>(wxPCXHandler) impl
        PCXHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PCXHandlerIsOwned<OWNED> {
    pub fn new() -> PCXHandlerIsOwned<OWNED> {
        unsafe { PCXHandlerIsOwned(ffi::wxPCXHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PCXHandlerIsOwned<OWNED>> for ImageHandlerIsOwned<OWNED> {
    fn from(o: PCXHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PCXHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PCXHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PCXHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPCXHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PCXHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPNGHandler
wx_class! { PNGHandler =
    PNGHandlerIsOwned<true>(wxPNGHandler) impl
        PNGHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PNGHandlerIsOwned<OWNED> {
    pub fn new() -> PNGHandlerIsOwned<OWNED> {
        unsafe { PNGHandlerIsOwned(ffi::wxPNGHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PNGHandlerIsOwned<OWNED>> for ImageHandlerIsOwned<OWNED> {
    fn from(o: PNGHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PNGHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PNGHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PNGHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPNGHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PNGHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPNMHandler
wx_class! { PNMHandler =
    PNMHandlerIsOwned<true>(wxPNMHandler) impl
        PNMHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PNMHandlerIsOwned<OWNED> {
    pub fn new() -> PNMHandlerIsOwned<OWNED> {
        unsafe { PNMHandlerIsOwned(ffi::wxPNMHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PNMHandlerIsOwned<OWNED>> for ImageHandlerIsOwned<OWNED> {
    fn from(o: PNMHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PNMHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PNMHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PNMHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPNMHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PNMHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPageSetupDialog
wx_class! { PageSetupDialog =
    PageSetupDialogIsOwned<true>(wxPageSetupDialog) impl
        PageSetupDialogMethods,
        ObjectMethods
}
impl<const OWNED: bool> PageSetupDialogIsOwned<OWNED> {
    pub fn new<W: WindowMethods, P: PageSetupDialogDataMethods>(
        parent: Option<&W>,
        data: Option<&P>,
    ) -> PageSetupDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            PageSetupDialogIsOwned(ffi::wxPageSetupDialog_new(parent, data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PageSetupDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PageSetupDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PageSetupDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPageSetupDialog_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PageSetupDialogIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPageSetupDialogData
wx_class! { PageSetupDialogData =
    PageSetupDialogDataIsOwned<true>(wxPageSetupDialogData) impl
        PageSetupDialogDataMethods,
        ObjectMethods
}
impl<const OWNED: bool> PageSetupDialogDataIsOwned<OWNED> {
    pub fn new() -> PageSetupDialogDataIsOwned<OWNED> {
        unsafe { PageSetupDialogDataIsOwned(ffi::wxPageSetupDialogData_new()) }
    }
    pub fn new_with_pagesetupdialogdata<P: PageSetupDialogDataMethods>(
        data: &P,
    ) -> PageSetupDialogDataIsOwned<OWNED> {
        unsafe {
            let data = data.as_ptr();
            PageSetupDialogDataIsOwned(ffi::wxPageSetupDialogData_new1(data))
        }
    }
    pub fn new_with_printdata<P: PrintDataMethods>(
        print_data: &P,
    ) -> PageSetupDialogDataIsOwned<OWNED> {
        unsafe {
            let print_data = print_data.as_ptr();
            PageSetupDialogDataIsOwned(ffi::wxPageSetupDialogData_new2(print_data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PageSetupDialogDataIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PageSetupDialogDataIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PageSetupDialogDataIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPageSetupDialogData_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PageSetupDialogDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPaintDC
wx_class! { PaintDC =
    PaintDCIsOwned<true>(wxPaintDC) impl
        PaintDCMethods,
        ClientDCMethods,
        WindowDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> PaintDCIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(window: Option<&W>) -> PaintDCIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            PaintDCIsOwned(ffi::wxPaintDC_new(window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PaintDCIsOwned<OWNED>> for ClientDCIsOwned<OWNED> {
    fn from(o: PaintDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PaintDCIsOwned<OWNED>> for WindowDCIsOwned<OWNED> {
    fn from(o: PaintDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PaintDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: PaintDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PaintDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PaintDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PaintDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPaintDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PaintDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPaintEvent
wx_class! { PaintEvent =
    PaintEventIsOwned<true>(wxPaintEvent) impl
        PaintEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> PaintEventIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(window: Option<&W>) -> PaintEventIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            PaintEventIsOwned(ffi::wxPaintEvent_new(window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PaintEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: PaintEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PaintEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PaintEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PaintEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPaintEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PaintEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPalette
wx_class! { Palette =
    PaletteIsOwned<true>(wxPalette) impl
        PaletteMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> PaletteIsOwned<OWNED> {
    pub fn new() -> PaletteIsOwned<OWNED> {
        unsafe { PaletteIsOwned(ffi::wxPalette_new()) }
    }
    pub fn new_with_palette<P: PaletteMethods>(palette: &P) -> PaletteIsOwned<OWNED> {
        unsafe {
            let palette = palette.as_ptr();
            PaletteIsOwned(ffi::wxPalette_new1(palette))
        }
    }
    pub fn new_with_int(
        n: c_int,
        red: *const c_void,
        green: *const c_void,
        blue: *const c_void,
    ) -> PaletteIsOwned<OWNED> {
        unsafe { PaletteIsOwned(ffi::wxPalette_new2(n, red, green, blue)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PaletteIsOwned<OWNED>> for GDIObjectIsOwned<OWNED> {
    fn from(o: PaletteIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PaletteIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PaletteIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PaletteIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPalette_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PaletteIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPanel
wx_class! { Panel =
    PanelIsOwned<true>(wxPanel) impl
        PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PanelIsOwned<OWNED> {
    pub fn new_2step() -> PanelIsOwned<OWNED> {
        unsafe { PanelIsOwned(ffi::wxPanel_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> PanelIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            PanelIsOwned(ffi::wxPanel_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PanelIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: PanelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PanelIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: PanelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PanelIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PanelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PanelIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPanel_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for PanelIsOwned<OWNED> {
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
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxPanel_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxPasswordEntryDialog
wx_class! { PasswordEntryDialog =
    PasswordEntryDialogIsOwned<true>(wxPasswordEntryDialog) impl
        PasswordEntryDialogMethods,
        TextEntryDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PasswordEntryDialogIsOwned<OWNED> {
    pub fn new<W: WindowMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        caption: &str,
        default_value: &str,
        style: c_long,
        pos: &P,
    ) -> PasswordEntryDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let caption = WxString::from(caption);
            let caption = caption.as_ptr();
            let default_value = WxString::from(default_value);
            let default_value = default_value.as_ptr();
            let pos = pos.as_ptr();
            PasswordEntryDialogIsOwned(ffi::wxPasswordEntryDialog_new(
                parent,
                message,
                caption,
                default_value,
                style,
                pos,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PasswordEntryDialogIsOwned<OWNED>> for TextEntryDialogIsOwned<OWNED> {
    fn from(o: PasswordEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PasswordEntryDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: PasswordEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PasswordEntryDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: PasswordEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PasswordEntryDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: PasswordEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PasswordEntryDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: PasswordEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PasswordEntryDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: PasswordEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PasswordEntryDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PasswordEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PasswordEntryDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPasswordEntryDialog_CLASSINFO()) }
    }
}

// wxPen
wx_class! { Pen =
    PenIsOwned<true>(wxPen) impl
        PenMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> PenIsOwned<OWNED> {
    pub fn new() -> PenIsOwned<OWNED> {
        unsafe { PenIsOwned(ffi::wxPen_new()) }
    }
    pub fn new_with_peninfo(info: *const c_void) -> PenIsOwned<OWNED> {
        unsafe { PenIsOwned(ffi::wxPen_new1(info)) }
    }
    // NOT_SUPPORTED: fn wxPen2()
    pub fn new_with_bitmap<B: BitmapMethods>(stipple: &B, width: c_int) -> PenIsOwned<OWNED> {
        unsafe {
            let stipple = stipple.as_ptr();
            PenIsOwned(ffi::wxPen_new3(stipple, width))
        }
    }
    pub fn new_with_pen<P: PenMethods>(pen: &P) -> PenIsOwned<OWNED> {
        unsafe {
            let pen = pen.as_ptr();
            PenIsOwned(ffi::wxPen_new4(pen))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PenIsOwned<OWNED>> for GDIObjectIsOwned<OWNED> {
    fn from(o: PenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PenIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PenIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPen_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PenIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPenList
wx_class! { PenList =
    PenListIsOwned<true>(wxPenList) impl
        PenListMethods
}
impl<const OWNED: bool> PenListIsOwned<OWNED> {
    pub fn new() -> PenListIsOwned<OWNED> {
        unsafe { PenListIsOwned(ffi::wxPenList_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for PenListIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxPenList_delete(self.0) }
        }
    }
}

// wxPersistenceManager
wx_class! { PersistenceManager =
    PersistenceManagerIsOwned<true>(wxPersistenceManager) impl
        PersistenceManagerMethods
}
impl<const OWNED: bool> PersistenceManagerIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for PersistenceManagerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxPersistenceManager_delete(self.0) }
        }
    }
}

// wxPickerBase
wx_class! { PickerBase =
    PickerBaseIsOwned<true>(wxPickerBase) impl
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PickerBaseIsOwned<OWNED> {
    // BLOCKED: fn wxPickerBase()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PickerBaseIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: PickerBaseIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PickerBaseIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: PickerBaseIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PickerBaseIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: PickerBaseIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PickerBaseIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PickerBaseIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PickerBaseIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPickerBase_CLASSINFO()) }
    }
}

// wxPoint
wx_class! { Point =
    PointIsOwned<true>(wxPoint) impl
        PointMethods
}
impl<const OWNED: bool> PointIsOwned<OWNED> {
    pub fn new() -> PointIsOwned<OWNED> {
        unsafe { PointIsOwned(ffi::wxPoint_new()) }
    }
    pub fn new_with_int(x: c_int, y: c_int) -> PointIsOwned<OWNED> {
        unsafe { PointIsOwned(ffi::wxPoint_new1(x, y)) }
    }
    pub fn new_with_realpoint(pt: *const c_void) -> PointIsOwned<OWNED> {
        unsafe { PointIsOwned(ffi::wxPoint_new2(pt)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for PointIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxPoint_delete(self.0) }
        }
    }
}

// wxPopupTransientWindow
wx_class! { PopupTransientWindow =
    PopupTransientWindowIsOwned<true>(wxPopupTransientWindow) impl
        PopupTransientWindowMethods,
        PopupWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PopupTransientWindowIsOwned<OWNED> {
    pub fn new_2step() -> PopupTransientWindowIsOwned<OWNED> {
        unsafe { PopupTransientWindowIsOwned(ffi::wxPopupTransientWindow_new()) }
    }
    pub fn new<W: WindowMethods>(
        parent: Option<&W>,
        flags: c_int,
    ) -> PopupTransientWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            PopupTransientWindowIsOwned(ffi::wxPopupTransientWindow_new1(parent, flags))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PopupTransientWindowIsOwned<OWNED>> for PopupWindowIsOwned<OWNED> {
    fn from(o: PopupTransientWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PopupTransientWindowIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: PopupTransientWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PopupTransientWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: PopupTransientWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PopupTransientWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: PopupTransientWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PopupTransientWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PopupTransientWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PopupTransientWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPopupTransientWindow_CLASSINFO()) }
    }
}

// wxPopupWindow
wx_class! { PopupWindow =
    PopupWindowIsOwned<true>(wxPopupWindow) impl
        PopupWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PopupWindowIsOwned<OWNED> {
    pub fn new_2step() -> PopupWindowIsOwned<OWNED> {
        unsafe { PopupWindowIsOwned(ffi::wxPopupWindow_new()) }
    }
    pub fn new<W: WindowMethods>(parent: Option<&W>, flags: c_int) -> PopupWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            PopupWindowIsOwned(ffi::wxPopupWindow_new1(parent, flags))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PopupWindowIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: PopupWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PopupWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: PopupWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PopupWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: PopupWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PopupWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PopupWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PopupWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPopupWindow_CLASSINFO()) }
    }
}

// wxPreferencesEditor
wx_class! { PreferencesEditor =
    PreferencesEditorIsOwned<true>(wxPreferencesEditor) impl
        PreferencesEditorMethods
}
impl<const OWNED: bool> PreferencesEditorIsOwned<OWNED> {
    pub fn new(title: &str) -> PreferencesEditorIsOwned<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            PreferencesEditorIsOwned(ffi::wxPreferencesEditor_new(title))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for PreferencesEditorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxPreferencesEditor_delete(self.0) }
        }
    }
}

// wxPreferencesPage
wx_class! { PreferencesPage =
    PreferencesPageIsOwned<true>(wxPreferencesPage) impl
        PreferencesPageMethods
}
impl<const OWNED: bool> PreferencesPageIsOwned<OWNED> {
    pub fn new() -> PreferencesPageIsOwned<OWNED> {
        unsafe { PreferencesPageIsOwned(ffi::wxPreferencesPage_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for PreferencesPageIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxPreferencesPage_delete(self.0) }
        }
    }
}

// wxPreviewControlBar
wx_class! { PreviewControlBar =
    PreviewControlBarIsOwned<true>(wxPreviewControlBar) impl
        PreviewControlBarMethods,
        PanelMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PreviewControlBarIsOwned<OWNED> {
    pub fn new<P: PrintPreviewMethods, W: WindowMethods, P2: PointMethods, S: SizeMethods>(
        preview: Option<&P>,
        buttons: c_long,
        parent: Option<&W>,
        pos: &P2,
        size: &S,
        style: c_long,
        name: &str,
    ) -> PreviewControlBarIsOwned<OWNED> {
        unsafe {
            let preview = match preview {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            PreviewControlBarIsOwned(ffi::wxPreviewControlBar_new(
                preview, buttons, parent, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PreviewControlBarIsOwned<OWNED>> for PanelIsOwned<OWNED> {
    fn from(o: PreviewControlBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PreviewControlBarIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: PreviewControlBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PreviewControlBarIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: PreviewControlBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PreviewControlBarIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PreviewControlBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PreviewControlBarIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPreviewControlBar_CLASSINFO()) }
    }
}

// wxPreviewFrame
wx_class! { PreviewFrame =
    PreviewFrameIsOwned<true>(wxPreviewFrame) impl
        PreviewFrameMethods,
        FrameMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PreviewFrameIsOwned<OWNED> {
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        preview: *mut c_void,
        parent: Option<&W>,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> PreviewFrameIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            PreviewFrameIsOwned(ffi::wxPreviewFrame_new(
                preview, parent, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PreviewFrameIsOwned<OWNED>> for FrameIsOwned<OWNED> {
    fn from(o: PreviewFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PreviewFrameIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: PreviewFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PreviewFrameIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: PreviewFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PreviewFrameIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: PreviewFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PreviewFrameIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: PreviewFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PreviewFrameIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PreviewFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PreviewFrameIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPreviewFrame_CLASSINFO()) }
    }
}

// wxPrintData
wx_class! { PrintData =
    PrintDataIsOwned<true>(wxPrintData) impl
        PrintDataMethods,
        ObjectMethods
}
impl<const OWNED: bool> PrintDataIsOwned<OWNED> {
    pub fn new() -> PrintDataIsOwned<OWNED> {
        unsafe { PrintDataIsOwned(ffi::wxPrintData_new()) }
    }
    pub fn new_with_printdata<P: PrintDataMethods>(data: &P) -> PrintDataIsOwned<OWNED> {
        unsafe {
            let data = data.as_ptr();
            PrintDataIsOwned(ffi::wxPrintData_new1(data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PrintDataIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PrintDataIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PrintDataIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPrintData_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PrintDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPrintDialog
wx_class! { PrintDialog =
    PrintDialogIsOwned<true>(wxPrintDialog) impl
        PrintDialogMethods,
        ObjectMethods
}
impl<const OWNED: bool> PrintDialogIsOwned<OWNED> {
    pub fn new_with_printdialogdata<W: WindowMethods, P: PrintDialogDataMethods>(
        parent: Option<&W>,
        data: Option<&P>,
    ) -> PrintDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            PrintDialogIsOwned(ffi::wxPrintDialog_new(parent, data))
        }
    }
    pub fn new_with_printdata<W: WindowMethods, P: PrintDataMethods>(
        parent: Option<&W>,
        data: Option<&P>,
    ) -> PrintDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            PrintDialogIsOwned(ffi::wxPrintDialog_new1(parent, data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PrintDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PrintDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PrintDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPrintDialog_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PrintDialogIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPrintDialogData
wx_class! { PrintDialogData =
    PrintDialogDataIsOwned<true>(wxPrintDialogData) impl
        PrintDialogDataMethods,
        ObjectMethods
}
impl<const OWNED: bool> PrintDialogDataIsOwned<OWNED> {
    pub fn new() -> PrintDialogDataIsOwned<OWNED> {
        unsafe { PrintDialogDataIsOwned(ffi::wxPrintDialogData_new()) }
    }
    pub fn new_with_printdialogdata<P: PrintDialogDataMethods>(
        dialog_data: &P,
    ) -> PrintDialogDataIsOwned<OWNED> {
        unsafe {
            let dialog_data = dialog_data.as_ptr();
            PrintDialogDataIsOwned(ffi::wxPrintDialogData_new1(dialog_data))
        }
    }
    pub fn new_with_printdata<P: PrintDataMethods>(
        print_data: &P,
    ) -> PrintDialogDataIsOwned<OWNED> {
        unsafe {
            let print_data = print_data.as_ptr();
            PrintDialogDataIsOwned(ffi::wxPrintDialogData_new2(print_data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PrintDialogDataIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PrintDialogDataIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PrintDialogDataIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPrintDialogData_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PrintDialogDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPrintPreview
wx_class! { PrintPreview =
    PrintPreviewIsOwned<true>(wxPrintPreview) impl
        PrintPreviewMethods,
        ObjectMethods
}
impl<const OWNED: bool> PrintPreviewIsOwned<OWNED> {
    pub fn new_with_printdialogdata<
        P: PrintoutMethods,
        P2: PrintoutMethods,
        P3: PrintDialogDataMethods,
    >(
        printout: Option<&P>,
        printout_for_printing: Option<&P2>,
        data: Option<&P3>,
    ) -> PrintPreviewIsOwned<OWNED> {
        unsafe {
            let printout = match printout {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let printout_for_printing = match printout_for_printing {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            PrintPreviewIsOwned(ffi::wxPrintPreview_new(
                printout,
                printout_for_printing,
                data,
            ))
        }
    }
    pub fn new_with_printdata<P: PrintoutMethods, P2: PrintoutMethods, P3: PrintDataMethods>(
        printout: Option<&P>,
        printout_for_printing: Option<&P2>,
        data: Option<&P3>,
    ) -> PrintPreviewIsOwned<OWNED> {
        unsafe {
            let printout = match printout {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let printout_for_printing = match printout_for_printing {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            PrintPreviewIsOwned(ffi::wxPrintPreview_new1(
                printout,
                printout_for_printing,
                data,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PrintPreviewIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PrintPreviewIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PrintPreviewIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPrintPreview_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PrintPreviewIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPrinter
wx_class! { Printer =
    PrinterIsOwned<true>(wxPrinter) impl
        PrinterMethods,
        ObjectMethods
}
impl<const OWNED: bool> PrinterIsOwned<OWNED> {
    pub fn new<P: PrintDialogDataMethods>(data: Option<&P>) -> PrinterIsOwned<OWNED> {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            PrinterIsOwned(ffi::wxPrinter_new(data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PrinterIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PrinterIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PrinterIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPrinter_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PrinterIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPrinterDC
wx_class! { PrinterDC =
    PrinterDCIsOwned<true>(wxPrinterDC) impl
        PrinterDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> PrinterDCIsOwned<OWNED> {
    pub fn new<P: PrintDataMethods>(print_data: &P) -> PrinterDCIsOwned<OWNED> {
        unsafe {
            let print_data = print_data.as_ptr();
            PrinterDCIsOwned(ffi::wxPrinterDC_new(print_data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PrinterDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: PrinterDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PrinterDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PrinterDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PrinterDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPrinterDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PrinterDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPrintout
wx_class! { Printout =
    PrintoutIsOwned<true>(wxPrintout) impl
        PrintoutMethods,
        ObjectMethods
}
impl<const OWNED: bool> PrintoutIsOwned<OWNED> {
    pub fn new(title: &str) -> PrintoutIsOwned<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            PrintoutIsOwned(ffi::wxPrintout_new(title))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PrintoutIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PrintoutIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PrintoutIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPrintout_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PrintoutIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPropertySheetDialog
wx_class! { PropertySheetDialog =
    PropertySheetDialogIsOwned<true>(wxPropertySheetDialog) impl
        PropertySheetDialogMethods,
        DialogMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PropertySheetDialogIsOwned<OWNED> {
    pub fn new_2step() -> PropertySheetDialogIsOwned<OWNED> {
        unsafe { PropertySheetDialogIsOwned(ffi::wxPropertySheetDialog_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> PropertySheetDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            PropertySheetDialogIsOwned(ffi::wxPropertySheetDialog_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PropertySheetDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: PropertySheetDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PropertySheetDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: PropertySheetDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PropertySheetDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: PropertySheetDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PropertySheetDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: PropertySheetDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PropertySheetDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: PropertySheetDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PropertySheetDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PropertySheetDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PropertySheetDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPropertySheetDialog_CLASSINFO()) }
    }
}
impl<const OWNED: bool> TopLevelWindowMethods for PropertySheetDialogIsOwned<OWNED> {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
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
            let title = WxString::from(title);
            let title = title.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxPropertySheetDialog_Create(
                self.as_ptr(),
                parent,
                id,
                title,
                pos,
                size,
                style,
                name,
            )
        }
    }
}
