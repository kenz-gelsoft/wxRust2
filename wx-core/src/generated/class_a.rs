use super::*;

// wxAboutDialogInfo
wxwidgets! {
    /// wxAboutDialogInfo contains information shown in the standard About dialog displayed by the wxAboutBox() function.
    ///
    /// [See `wxAboutDialogInfo`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html)
    #[doc(alias = "wxAboutDialogInfo")]
    #[doc(alias = "AboutDialogInfo")]
    class AboutDialogInfo
        = AboutDialogInfoIsOwned<true>(wxAboutDialogInfo) impl
        AboutDialogInfoMethods
}
impl<const OWNED: bool> AboutDialogInfoIsOwned<OWNED> {
    /// Default constructor leaves all fields are initially uninitialized, in general you should call at least SetVersion(), SetCopyright() and SetDescription().
    pub fn new() -> AboutDialogInfoIsOwned<OWNED> {
        unsafe { AboutDialogInfoIsOwned(ffi::wxAboutDialogInfo_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AboutDialogInfoIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for AboutDialogInfoIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxAboutDialogInfo_delete(self.0) }
        }
    }
}

// wxAcceleratorEntry
wxwidgets! {
    /// An object used by an application wishing to create an accelerator table (see wxAcceleratorTable).
    ///
    /// [See `wxAcceleratorEntry`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_accelerator_entry.html)
    #[doc(alias = "wxAcceleratorEntry")]
    #[doc(alias = "AcceleratorEntry")]
    class AcceleratorEntry
        = AcceleratorEntryIsOwned<true>(wxAcceleratorEntry) impl
        AcceleratorEntryMethods
}
impl<const OWNED: bool> AcceleratorEntryIsOwned<OWNED> {
    /// Constructor.
    pub fn new_with_int<M: MenuItemMethods>(
        flags: c_int,
        key_code: c_int,
        cmd: c_int,
        item: Option<&M>,
    ) -> AcceleratorEntryIsOwned<OWNED> {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            AcceleratorEntryIsOwned(ffi::wxAcceleratorEntry_new(flags, key_code, cmd, item))
        }
    }
    /// Copy ctor.
    pub fn new_with_acceleratorentry<A: AcceleratorEntryMethods>(
        entry: &A,
    ) -> AcceleratorEntryIsOwned<OWNED> {
        unsafe {
            let entry = entry.as_ptr();
            AcceleratorEntryIsOwned(ffi::wxAcceleratorEntry_new1(entry))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AcceleratorEntryIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for AcceleratorEntryIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxAcceleratorEntry_delete(self.0) }
        }
    }
}

// wxAcceleratorTable
wxwidgets! {
    /// An accelerator table allows the application to specify a table of keyboard shortcuts for menu or button commands.
    ///
    /// [See `wxAcceleratorTable`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_accelerator_table.html)
    #[doc(alias = "wxAcceleratorTable")]
    #[doc(alias = "AcceleratorTable")]
    class AcceleratorTable
        = AcceleratorTableIsOwned<true>(wxAcceleratorTable) impl
        AcceleratorTableMethods,
        ObjectMethods
}
impl<const OWNED: bool> AcceleratorTableIsOwned<OWNED> {
    /// Default ctor.
    pub fn new() -> AcceleratorTableIsOwned<OWNED> {
        unsafe { AcceleratorTableIsOwned(ffi::wxAcceleratorTable_new()) }
    }
    // NOT_SUPPORTED: fn wxAcceleratorTable1()
    // BLOCKED: fn wxAcceleratorTable2()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AcceleratorTableIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<AcceleratorTableIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: AcceleratorTableIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for AcceleratorTableIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxAcceleratorTable_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for AcceleratorTableIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxActivateEvent
wxwidgets! {
    /// An activate event is sent when a window or application is being activated or deactivated.
    ///
    /// [See `wxActivateEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_activate_event.html)
    #[doc(alias = "wxActivateEvent")]
    #[doc(alias = "ActivateEvent")]
    class ActivateEvent
        = ActivateEventIsOwned<true>(wxActivateEvent) impl
        ActivateEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ActivateEventIsOwned<OWNED> {
    //  ENUM: Reason
    pub const Reason_Mouse: c_int = 0;
    pub const Reason_Unknown: c_int = 0 + 1;

    // NOT_SUPPORTED: fn wxActivateEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ActivateEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ActivateEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ActivateEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ActivateEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ActivateEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ActivateEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxActivateEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ActivateEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxAffineMatrix2D
wxwidgets! {
    /// A 3x2 matrix representing an affine 2D transformation.
    ///
    /// [See `wxAffineMatrix2D`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d.html)
    #[doc(alias = "wxAffineMatrix2D")]
    #[doc(alias = "AffineMatrix2D")]
    class AffineMatrix2D
        = AffineMatrix2DIsOwned<true>(wxAffineMatrix2D) impl
        AffineMatrix2DMethods
        // AffineMatrix2DBaseMethods
}
impl<const OWNED: bool> AffineMatrix2DIsOwned<OWNED> {
    /// Default constructor.
    pub fn new() -> AffineMatrix2DIsOwned<OWNED> {
        unsafe { AffineMatrix2DIsOwned(ffi::wxAffineMatrix2D_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AffineMatrix2DIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<AffineMatrix2DIsOwned<OWNED>> for AffineMatrix2DBaseIsOwned<OWNED> {
    fn from(o: AffineMatrix2DIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for AffineMatrix2DIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxAffineMatrix2D_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> AffineMatrix2DBaseMethods for AffineMatrix2DIsOwned<OWNED> {
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    /// Add mirroring to this matrix.
    fn mirror(&self, direction: c_int) {
        unsafe { ffi::wxAffineMatrix2D_Mirror(self.as_ptr(), direction) }
    }
    // NOT_SUPPORTED: fn TransformPoint()
    fn transform_point(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxAffineMatrix2D_TransformPoint1(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn TransformDistance()
    fn transform_distance(&self, dx: *mut c_void, dy: *mut c_void) {
        unsafe { ffi::wxAffineMatrix2D_TransformDistance1(self.as_ptr(), dx, dy) }
    }
}

// wxAffineMatrix2DBase
wxwidgets! {
    /// A 2x3 matrix representing an affine 2D transformation.
    ///
    /// [See `wxAffineMatrix2DBase`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d_base.html)
    #[doc(alias = "wxAffineMatrix2DBase")]
    #[doc(alias = "AffineMatrix2DBase")]
    class AffineMatrix2DBase
        = AffineMatrix2DBaseIsOwned<true>(wxAffineMatrix2DBase) impl
        AffineMatrix2DBaseMethods
}
impl<const OWNED: bool> AffineMatrix2DBaseIsOwned<OWNED> {
    // BLOCKED: fn wxAffineMatrix2DBase()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AffineMatrix2DBaseIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for AffineMatrix2DBaseIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxAffineMatrix2DBase_delete(self.0) }
        }
    }
}

// wxAnimationCtrl
wxwidgets! {
    /// This is a static control which displays an animation.
    ///
    /// [See `wxAnimationCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_animation_ctrl.html)
    #[doc(alias = "wxAnimationCtrl")]
    #[doc(alias = "AnimationCtrl")]
    class AnimationCtrl
        = AnimationCtrlIsOwned<true>(wxAnimationCtrl) impl
        AnimationCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> AnimationCtrlIsOwned<OWNED> {
    /// Initializes the object and calls Create() with all the parameters.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        anim: *const c_void,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> AnimationCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            AnimationCtrlIsOwned(ffi::wxAnimationCtrl_new(
                parent, id, anim, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for AnimationCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<AnimationCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: AnimationCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<AnimationCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: AnimationCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<AnimationCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: AnimationCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<AnimationCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: AnimationCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for AnimationCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxAnimationCtrl_CLASSINFO()) }
    }
}

// wxAnyButton
wxwidgets! {
    /// A class for common button functionality used as the base for the various button classes.
    ///
    /// [See `wxAnyButton`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_any_button.html)
    #[doc(alias = "wxAnyButton")]
    #[doc(alias = "AnyButton")]
    class AnyButton
        = AnyButtonIsOwned<true>(wxAnyButton) impl
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> AnyButtonIsOwned<OWNED> {
    pub fn new() -> AnyButtonIsOwned<OWNED> {
        unsafe { AnyButtonIsOwned(ffi::wxAnyButton_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for AnyButtonIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<AnyButtonIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: AnyButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<AnyButtonIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: AnyButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<AnyButtonIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: AnyButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<AnyButtonIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: AnyButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for AnyButtonIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxAnyButton_CLASSINFO()) }
    }
}

// wxArtProvider
wxwidgets! {
    /// wxArtProvider class is used to customize the look of wxWidgets application.
    ///
    /// [See `wxArtProvider`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_art_provider.html)
    #[doc(alias = "wxArtProvider")]
    #[doc(alias = "ArtProvider")]
    class ArtProvider
        = ArtProviderIsOwned<true>(wxArtProvider) impl
        ArtProviderMethods,
        ObjectMethods
}
impl<const OWNED: bool> ArtProviderIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ArtProviderIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ArtProviderIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ArtProviderIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ArtProviderIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxArtProvider_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ArtProviderIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxAutoBufferedPaintDC
wxwidgets! {
    /// This wxDC derivative can be used inside of an EVT_PAINT() event handler to achieve double-buffered drawing.
    ///
    /// [See `wxAutoBufferedPaintDC`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_auto_buffered_paint_d_c.html)
    #[doc(alias = "wxAutoBufferedPaintDC")]
    #[doc(alias = "AutoBufferedPaintDC")]
    class AutoBufferedPaintDC
        = AutoBufferedPaintDCIsOwned<true>(wxAutoBufferedPaintDC) impl
        AutoBufferedPaintDCMethods,
        BufferedPaintDCMethods,
        BufferedDCMethods,
        MemoryDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> AutoBufferedPaintDCIsOwned<OWNED> {
    /// Constructor.
    pub fn new<W: WindowMethods>(window: Option<&W>) -> AutoBufferedPaintDCIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            AutoBufferedPaintDCIsOwned(ffi::wxAutoBufferedPaintDC_new(window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AutoBufferedPaintDCIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<AutoBufferedPaintDCIsOwned<OWNED>> for BufferedPaintDCIsOwned<OWNED> {
    fn from(o: AutoBufferedPaintDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<AutoBufferedPaintDCIsOwned<OWNED>> for BufferedDCIsOwned<OWNED> {
    fn from(o: AutoBufferedPaintDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<AutoBufferedPaintDCIsOwned<OWNED>> for MemoryDCIsOwned<OWNED> {
    fn from(o: AutoBufferedPaintDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<AutoBufferedPaintDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: AutoBufferedPaintDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<AutoBufferedPaintDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: AutoBufferedPaintDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for AutoBufferedPaintDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxAutoBufferedPaintDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for AutoBufferedPaintDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
