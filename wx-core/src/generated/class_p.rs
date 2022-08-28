use super::*;

// wxPCXHandler
wxwidgets! {
    /// This is the image handler for the PCX format.
    ///
    /// [See `wxPCXHandler`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_p_c_x_handler.html)
    #[doc(alias = "wxPCXHandler")]
    #[doc(alias = "PCXHandler")]
    class PCXHandler
        = PCXHandlerIsOwned<true>(wxPCXHandler) impl
        PCXHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PCXHandlerIsOwned<OWNED> {
    /// Default constructor for wxPCXHandler.
    pub fn new() -> PCXHandlerIsOwned<OWNED> {
        unsafe { PCXHandlerIsOwned(ffi::wxPCXHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PCXHandlerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// This is the image handler for the PNG format.
    ///
    /// [See `wxPNGHandler`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_p_n_g_handler.html)
    #[doc(alias = "wxPNGHandler")]
    #[doc(alias = "PNGHandler")]
    class PNGHandler
        = PNGHandlerIsOwned<true>(wxPNGHandler) impl
        PNGHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PNGHandlerIsOwned<OWNED> {
    /// Default constructor for wxPNGHandler.
    pub fn new() -> PNGHandlerIsOwned<OWNED> {
        unsafe { PNGHandlerIsOwned(ffi::wxPNGHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PNGHandlerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// This is the image handler for the PNM format.
    ///
    /// [See `wxPNMHandler`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_p_n_m_handler.html)
    #[doc(alias = "wxPNMHandler")]
    #[doc(alias = "PNMHandler")]
    class PNMHandler
        = PNMHandlerIsOwned<true>(wxPNMHandler) impl
        PNMHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PNMHandlerIsOwned<OWNED> {
    /// Default constructor for wxPNMHandler.
    pub fn new() -> PNMHandlerIsOwned<OWNED> {
        unsafe { PNMHandlerIsOwned(ffi::wxPNMHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PNMHandlerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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

// wxPaintDC
wxwidgets! {
    /// A wxPaintDC must be constructed if an application wishes to paint on the client area of a window from within an EVT_PAINT() event handler.
    ///
    /// [See `wxPaintDC`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_paint_d_c.html)
    #[doc(alias = "wxPaintDC")]
    #[doc(alias = "PaintDC")]
    class PaintDC
        = PaintDCIsOwned<true>(wxPaintDC) impl
        PaintDCMethods,
        ClientDCMethods,
        WindowDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> PaintDCIsOwned<OWNED> {
    /// Constructor.
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
impl Clone for PaintDCIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// A paint event is sent when a window's contents needs to be repainted.
    ///
    /// [See `wxPaintEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_paint_event.html)
    #[doc(alias = "wxPaintEvent")]
    #[doc(alias = "PaintEvent")]
    class PaintEvent
        = PaintEventIsOwned<true>(wxPaintEvent) impl
        PaintEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> PaintEventIsOwned<OWNED> {
    // BLOCKED: fn wxPaintEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PaintEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// A palette is a table that maps pixel values to RGB colours.
    ///
    /// [See `wxPalette`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_palette.html)
    #[doc(alias = "wxPalette")]
    #[doc(alias = "Palette")]
    class Palette
        = PaletteIsOwned<true>(wxPalette) impl
        PaletteMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> PaletteIsOwned<OWNED> {
    /// Default constructor.
    pub fn new() -> PaletteIsOwned<OWNED> {
        unsafe { PaletteIsOwned(ffi::wxPalette_new()) }
    }
    /// Copy constructor, uses Reference Counting.
    pub fn new_with_palette<P: PaletteMethods>(palette: &P) -> PaletteIsOwned<OWNED> {
        unsafe {
            let palette = palette.as_ptr();
            PaletteIsOwned(ffi::wxPalette_new1(palette))
        }
    }
    /// Creates a palette from arrays of size n, one for each red, blue or green component.
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
impl Clone for PaletteIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// A panel is a window on which controls are placed.
    ///
    /// [See `wxPanel`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_panel.html)
    #[doc(alias = "wxPanel")]
    #[doc(alias = "Panel")]
    class Panel
        = PanelIsOwned<true>(wxPanel) impl
        PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PanelIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> PanelIsOwned<OWNED> {
        unsafe { PanelIsOwned(ffi::wxPanel_new()) }
    }
    /// Constructor.
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
impl<const OWNED: bool> Clone for PanelIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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
    /// Used for two-step panel construction.
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
wxwidgets! {
    /// This class represents a dialog that requests a one-line password string from the user.
    ///
    /// [See `wxPasswordEntryDialog`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_password_entry_dialog.html)
    #[doc(alias = "wxPasswordEntryDialog")]
    #[doc(alias = "PasswordEntryDialog")]
    class PasswordEntryDialog
        = PasswordEntryDialogIsOwned<true>(wxPasswordEntryDialog) impl
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
    /// Constructor.
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
impl<const OWNED: bool> Clone for PasswordEntryDialogIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// A pen is a drawing tool for drawing outlines.
    ///
    /// [See `wxPen`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_pen.html)
    #[doc(alias = "wxPen")]
    #[doc(alias = "Pen")]
    class Pen
        = PenIsOwned<true>(wxPen) impl
        PenMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> PenIsOwned<OWNED> {
    /// Default constructor.
    pub fn new() -> PenIsOwned<OWNED> {
        unsafe { PenIsOwned(ffi::wxPen_new()) }
    }
    /// Creates a pen object using the specified pen description.
    pub fn new_with_peninfo(info: *const c_void) -> PenIsOwned<OWNED> {
        unsafe { PenIsOwned(ffi::wxPen_new1(info)) }
    }
    // NOT_SUPPORTED: fn wxPen2()
    /// Constructs a stippled pen from a stipple bitmap and a width.
    pub fn new_with_bitmap<B: BitmapMethods>(stipple: &B, width: c_int) -> PenIsOwned<OWNED> {
        unsafe {
            let stipple = stipple.as_ptr();
            PenIsOwned(ffi::wxPen_new3(stipple, width))
        }
    }
    /// Copy constructor, uses Reference Counting.
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
impl Clone for PenIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// There is only one instance of this class: wxThePenList.
    ///
    /// [See `wxPenList`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_pen_list.html)
    #[doc(alias = "wxPenList")]
    #[doc(alias = "PenList")]
    class PenList
        = PenListIsOwned<true>(wxPenList) impl
        PenListMethods
}
impl<const OWNED: bool> PenListIsOwned<OWNED> {
    /// Constructor.
    pub fn new() -> PenListIsOwned<OWNED> {
        unsafe { PenListIsOwned(ffi::wxPenList_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PenListIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// Provides support for automatically saving and restoring object properties to persistent storage.
    ///
    /// [See `wxPersistenceManager`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_persistence_manager.html)
    #[doc(alias = "wxPersistenceManager")]
    #[doc(alias = "PersistenceManager")]
    class PersistenceManager
        = PersistenceManagerIsOwned<true>(wxPersistenceManager) impl
        PersistenceManagerMethods
}
impl<const OWNED: bool> PersistenceManagerIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PersistenceManagerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// Base abstract class for all pickers which support an auxiliary text control.
    ///
    /// [See `wxPickerBase`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_picker_base.html)
    #[doc(alias = "wxPickerBase")]
    #[doc(alias = "PickerBase")]
    class PickerBase
        = PickerBaseIsOwned<true>(wxPickerBase) impl
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
impl<const OWNED: bool> Clone for PickerBaseIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// A wxPoint is a useful data structure for graphics operations.
    ///
    /// [See `wxPoint`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_point.html)
    #[doc(alias = "wxPoint")]
    #[doc(alias = "Point")]
    class Point
        = PointIsOwned<true>(wxPoint) impl
        PointMethods
}
impl<const OWNED: bool> PointIsOwned<OWNED> {
    /// Constructs a point.
    pub fn new() -> PointIsOwned<OWNED> {
        unsafe { PointIsOwned(ffi::wxPoint_new()) }
    }
    /// Initializes the point object with the given x and y coordinates.
    pub fn new_with_int(x: c_int, y: c_int) -> PointIsOwned<OWNED> {
        unsafe { PointIsOwned(ffi::wxPoint_new1(x, y)) }
    }
    /// Converts the given wxRealPoint (with floating point coordinates) to a wxPoint instance.
    pub fn new_with_realpoint<R: RealPointMethods>(pt: &R) -> PointIsOwned<OWNED> {
        unsafe {
            let pt = pt.as_ptr();
            PointIsOwned(ffi::wxPoint_new2(pt))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PointIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// A wxPopupWindow which disappears automatically when the user clicks mouse outside it or if it loses focus in any other way.
    ///
    /// [See `wxPopupTransientWindow`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_popup_transient_window.html)
    #[doc(alias = "wxPopupTransientWindow")]
    #[doc(alias = "PopupTransientWindow")]
    class PopupTransientWindow
        = PopupTransientWindowIsOwned<true>(wxPopupTransientWindow) impl
        PopupTransientWindowMethods,
        PopupWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PopupTransientWindowIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> PopupTransientWindowIsOwned<OWNED> {
        unsafe { PopupTransientWindowIsOwned(ffi::wxPopupTransientWindow_new()) }
    }
    /// Constructor.
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
impl<const OWNED: bool> Clone for PopupTransientWindowIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// A special kind of top level window used for popup menus, combobox popups and such.
    ///
    /// [See `wxPopupWindow`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_popup_window.html)
    #[doc(alias = "wxPopupWindow")]
    #[doc(alias = "PopupWindow")]
    class PopupWindow
        = PopupWindowIsOwned<true>(wxPopupWindow) impl
        PopupWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PopupWindowIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> PopupWindowIsOwned<OWNED> {
        unsafe { PopupWindowIsOwned(ffi::wxPopupWindow_new()) }
    }
    /// Constructor.
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
impl<const OWNED: bool> Clone for PopupWindowIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// Manage preferences dialog.
    ///
    /// [See `wxPreferencesEditor`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_preferences_editor.html)
    #[doc(alias = "wxPreferencesEditor")]
    #[doc(alias = "PreferencesEditor")]
    class PreferencesEditor
        = PreferencesEditorIsOwned<true>(wxPreferencesEditor) impl
        PreferencesEditorMethods
}
impl<const OWNED: bool> PreferencesEditorIsOwned<OWNED> {
    /// Constructor.
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
impl Clone for PreferencesEditorIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// One page of preferences dialog.
    ///
    /// [See `wxPreferencesPage`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_preferences_page.html)
    #[doc(alias = "wxPreferencesPage")]
    #[doc(alias = "PreferencesPage")]
    class PreferencesPage
        = PreferencesPageIsOwned<true>(wxPreferencesPage) impl
        PreferencesPageMethods
}
impl<const OWNED: bool> PreferencesPageIsOwned<OWNED> {
    // BLOCKED: fn wxPreferencesPage()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for PreferencesPageIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for PreferencesPageIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxPreferencesPage_delete(self.0) }
        }
    }
}

// wxPropertySheetDialog
wxwidgets! {
    /// This class represents a property sheet dialog: a tabbed dialog for showing settings.
    ///
    /// [See `wxPropertySheetDialog`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_property_sheet_dialog.html)
    #[doc(alias = "wxPropertySheetDialog")]
    #[doc(alias = "PropertySheetDialog")]
    class PropertySheetDialog
        = PropertySheetDialogIsOwned<true>(wxPropertySheetDialog) impl
        PropertySheetDialogMethods,
        DialogMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PropertySheetDialogIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> PropertySheetDialogIsOwned<OWNED> {
        unsafe { PropertySheetDialogIsOwned(ffi::wxPropertySheetDialog_new()) }
    }
    /// Constructor.
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
impl<const OWNED: bool> Clone for PropertySheetDialogIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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
    /// Call this from your own Create function, before adding buttons and pages.
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
