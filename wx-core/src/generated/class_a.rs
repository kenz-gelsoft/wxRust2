use super::*;

// wxAboutDialogInfo
wxwidgets! {
    /// wxAboutDialogInfo contains information shown in the standard About dialog displayed by the wxAboutBox() function.
    /// - [`AboutDialogInfo`] represents a C++ `wxAboutDialogInfo` class instance which your code has ownership, [`AboutDialogInfoInRust`]`<false>` represents one which don't own.
    /// - Use [`AboutDialogInfo`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxAboutDialogInfo` class's documentation](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html) for more details.
    #[doc(alias = "wxAboutDialogInfo")]
    #[doc(alias = "AboutDialogInfo")]
    class AboutDialogInfo
        = AboutDialogInfoInRust<true>(wxAboutDialogInfo) impl
        AboutDialogInfoMethods
}
impl<const IN_RUST: bool> AboutDialogInfoInRust<IN_RUST> {
    /// Default constructor leaves all fields are initially uninitialized, in general you should call at least SetVersion(), SetCopyright() and SetDescription().
    ///
    /// See [C++ `wxAboutDialogInfo::wxAboutDialogInfo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#ab58b7d0d7bb3aa40eca4531c4f8c1e6f).
    pub fn new() -> AboutDialogInfoInRust<IN_RUST> {
        unsafe { AboutDialogInfoInRust(ffi::wxAboutDialogInfo_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AboutDialogInfoInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> Drop for AboutDialogInfoInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxAboutDialogInfo_delete(self.0) }
        }
    }
}

// wxAcceleratorEntry
wxwidgets! {
    /// An object used by an application wishing to create an accelerator table (see wxAcceleratorTable).
    /// - [`AcceleratorEntry`] represents a C++ `wxAcceleratorEntry` class instance which your code has ownership, [`AcceleratorEntryInRust`]`<false>` represents one which don't own.
    /// - Use [`AcceleratorEntry`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxAcceleratorEntry` class's documentation](https://docs.wxwidgets.org/3.2/classwx_accelerator_entry.html) for more details.
    #[doc(alias = "wxAcceleratorEntry")]
    #[doc(alias = "AcceleratorEntry")]
    class AcceleratorEntry
        = AcceleratorEntryInRust<true>(wxAcceleratorEntry) impl
        AcceleratorEntryMethods
}
impl<const IN_RUST: bool> AcceleratorEntryInRust<IN_RUST> {
    /// Constructor.
    ///
    /// See [C++ `wxAcceleratorEntry::wxAcceleratorEntry()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_accelerator_entry.html#a9387fbbaed1cbfb8673308ec2b10da3e).
    pub fn new_with_int<M: MenuItemMethods>(
        flags: c_int,
        key_code: c_int,
        cmd: c_int,
        item: Option<&M>,
    ) -> AcceleratorEntryInRust<IN_RUST> {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            AcceleratorEntryInRust(ffi::wxAcceleratorEntry_new(flags, key_code, cmd, item))
        }
    }
    /// Copy ctor.
    ///
    /// See [C++ `wxAcceleratorEntry::wxAcceleratorEntry()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_accelerator_entry.html#a8853051e5706c505d15ff70e1e44c7df).
    pub fn new_with_acceleratorentry<A: AcceleratorEntryMethods>(
        entry: &A,
    ) -> AcceleratorEntryInRust<IN_RUST> {
        unsafe {
            let entry = entry.as_ptr();
            AcceleratorEntryInRust(ffi::wxAcceleratorEntry_new1(entry))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AcceleratorEntryInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> Drop for AcceleratorEntryInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxAcceleratorEntry_delete(self.0) }
        }
    }
}

// wxAcceleratorTable
wxwidgets! {
    /// An accelerator table allows the application to specify a table of keyboard shortcuts for menu or button commands.
    /// - [`AcceleratorTable`] represents a C++ `wxAcceleratorTable` class instance which your code has ownership, [`AcceleratorTableInRust`]`<false>` represents one which don't own.
    /// - Use [`AcceleratorTable`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxAcceleratorTable` class's documentation](https://docs.wxwidgets.org/3.2/classwx_accelerator_table.html) for more details.
    #[doc(alias = "wxAcceleratorTable")]
    #[doc(alias = "AcceleratorTable")]
    class AcceleratorTable
        = AcceleratorTableInRust<true>(wxAcceleratorTable) impl
        AcceleratorTableMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> AcceleratorTableInRust<IN_RUST> {
    /// Default ctor.
    ///
    /// See [C++ `wxAcceleratorTable::wxAcceleratorTable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_accelerator_table.html#af172242a8a1487aa326f7965857df7f7).
    pub fn new() -> AcceleratorTableInRust<IN_RUST> {
        unsafe { AcceleratorTableInRust(ffi::wxAcceleratorTable_new()) }
    }
    // NOT_SUPPORTED: fn wxAcceleratorTable1()
    // BLOCKED: fn wxAcceleratorTable2()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AcceleratorTableInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<AcceleratorTableInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: AcceleratorTableInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for AcceleratorTableInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxAcceleratorTable_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for AcceleratorTableInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxActivateEvent
wxwidgets! {
    /// An activate event is sent when a window or application is being activated or deactivated.
    /// - [`ActivateEvent`] represents a C++ `wxActivateEvent` class instance which your code has ownership, [`ActivateEventInRust`]`<false>` represents one which don't own.
    /// - Use [`ActivateEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxActivateEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_activate_event.html) for more details.
    #[doc(alias = "wxActivateEvent")]
    #[doc(alias = "ActivateEvent")]
    class ActivateEvent
        = ActivateEventInRust<true>(wxActivateEvent) impl
        ActivateEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> ActivateEventInRust<IN_RUST> {
    //  ENUM: Reason
    pub const Reason_Mouse: c_int = 0;
    pub const Reason_Unknown: c_int = 0 + 1;

    // NOT_SUPPORTED: fn wxActivateEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ActivateEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<ActivateEventInRust<IN_RUST>> for EventInRust<IN_RUST> {
    fn from(o: ActivateEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<ActivateEventInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: ActivateEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for ActivateEventInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxActivateEvent_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for ActivateEventInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxAffineMatrix2D
wxwidgets! {
    /// A 3x2 matrix representing an affine 2D transformation.
    /// - [`AffineMatrix2D`] represents a C++ `wxAffineMatrix2D` class instance which your code has ownership, [`AffineMatrix2DInRust`]`<false>` represents one which don't own.
    /// - Use [`AffineMatrix2D`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxAffineMatrix2D` class's documentation](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d.html) for more details.
    #[doc(alias = "wxAffineMatrix2D")]
    #[doc(alias = "AffineMatrix2D")]
    class AffineMatrix2D
        = AffineMatrix2DInRust<true>(wxAffineMatrix2D) impl
        AffineMatrix2DMethods
        // AffineMatrix2DBaseMethods
}
impl<const IN_RUST: bool> AffineMatrix2DInRust<IN_RUST> {
    /// Default constructor.
    ///
    /// See [C++ `wxAffineMatrix2D::wxAffineMatrix2D()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d.html#a00fa188b54418b476d122e4de408dc27).
    pub fn new() -> AffineMatrix2DInRust<IN_RUST> {
        unsafe { AffineMatrix2DInRust(ffi::wxAffineMatrix2D_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AffineMatrix2DInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<AffineMatrix2DInRust<IN_RUST>>
    for AffineMatrix2DBaseInRust<IN_RUST>
{
    fn from(o: AffineMatrix2DInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> Drop for AffineMatrix2DInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxAffineMatrix2D_delete(self.0) }
        }
    }
}
impl<const IN_RUST: bool> AffineMatrix2DBaseMethods for AffineMatrix2DInRust<IN_RUST> {
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    /// Add mirroring to this matrix.
    ///
    /// See [C++ `wxAffineMatrix2D::Mirror()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d.html#a5bec8e0a74908b53c46dd67d8013837c).
    fn mirror(&self, direction: c_int) {
        unsafe { ffi::wxAffineMatrix2D_Mirror(self.as_ptr(), direction) }
    }
    // NOT_SUPPORTED: fn TransformPoint()
    ///
    /// See [C++ `wxAffineMatrix2D::TransformPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d.html#ac7a910efff7136da8e2843db77b3f94c).
    fn transform_point(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxAffineMatrix2D_TransformPoint1(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn TransformDistance()
    ///
    /// See [C++ `wxAffineMatrix2D::TransformDistance()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d.html#a65a659678c537c6f68042af78ef5d7a9).
    fn transform_distance(&self, dx: *mut c_void, dy: *mut c_void) {
        unsafe { ffi::wxAffineMatrix2D_TransformDistance1(self.as_ptr(), dx, dy) }
    }
}

// wxAffineMatrix2DBase
wxwidgets! {
    /// A 2x3 matrix representing an affine 2D transformation.
    /// - [`AffineMatrix2DBase`] represents a C++ `wxAffineMatrix2DBase` class instance which your code has ownership, [`AffineMatrix2DBaseInRust`]`<false>` represents one which don't own.
    /// - Use [`AffineMatrix2DBase`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxAffineMatrix2DBase` class's documentation](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d_base.html) for more details.
    #[doc(alias = "wxAffineMatrix2DBase")]
    #[doc(alias = "AffineMatrix2DBase")]
    class AffineMatrix2DBase
        = AffineMatrix2DBaseInRust<true>(wxAffineMatrix2DBase) impl
        AffineMatrix2DBaseMethods
}
impl<const IN_RUST: bool> AffineMatrix2DBaseInRust<IN_RUST> {
    // BLOCKED: fn wxAffineMatrix2DBase()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AffineMatrix2DBaseInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> Drop for AffineMatrix2DBaseInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxAffineMatrix2DBase_delete(self.0) }
        }
    }
}

// wxAnimationCtrl
wxwidgets! {
    /// This is a static control which displays an animation.
    /// - [`AnimationCtrl`] represents a C++ `wxAnimationCtrl` class instance which your code has ownership, [`AnimationCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`AnimationCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxAnimationCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_animation_ctrl.html) for more details.
    #[doc(alias = "wxAnimationCtrl")]
    #[doc(alias = "AnimationCtrl")]
    class AnimationCtrl
        = AnimationCtrlInRust<true>(wxAnimationCtrl) impl
        AnimationCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> AnimationCtrlInRust<IN_RUST> {
    /// Initializes the object and calls Create() with all the parameters.
    ///
    /// See [C++ `wxAnimationCtrl::wxAnimationCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_animation_ctrl.html#ae661663278f7e2650ba70c08817e0511).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        anim: *const c_void,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> AnimationCtrlInRust<IN_RUST> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            AnimationCtrlInRust(ffi::wxAnimationCtrl_new(
                parent, id, anim, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for AnimationCtrlInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<AnimationCtrlInRust<IN_RUST>> for ControlInRust<IN_RUST> {
    fn from(o: AnimationCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<AnimationCtrlInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: AnimationCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<AnimationCtrlInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: AnimationCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<AnimationCtrlInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: AnimationCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for AnimationCtrlInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxAnimationCtrl_CLASSINFO()) }
    }
}

// wxAnyButton
wxwidgets! {
    /// A class for common button functionality used as the base for the various button classes.
    /// - [`AnyButton`] represents a C++ `wxAnyButton` class instance which your code has ownership, [`AnyButtonInRust`]`<false>` represents one which don't own.
    /// - Use [`AnyButton`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxAnyButton` class's documentation](https://docs.wxwidgets.org/3.2/classwx_any_button.html) for more details.
    #[doc(alias = "wxAnyButton")]
    #[doc(alias = "AnyButton")]
    class AnyButton
        = AnyButtonInRust<true>(wxAnyButton) impl
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> AnyButtonInRust<IN_RUST> {
    ///
    /// See [C++ `wxAnyButton::wxAnyButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_any_button.html#a89b895988e816974fa5f5971e2f3e2a4).
    pub fn new() -> AnyButtonInRust<IN_RUST> {
        unsafe { AnyButtonInRust(ffi::wxAnyButton_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for AnyButtonInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<AnyButtonInRust<IN_RUST>> for ControlInRust<IN_RUST> {
    fn from(o: AnyButtonInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<AnyButtonInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: AnyButtonInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<AnyButtonInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: AnyButtonInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<AnyButtonInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: AnyButtonInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for AnyButtonInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxAnyButton_CLASSINFO()) }
    }
}

// wxArtProvider
wxwidgets! {
    /// wxArtProvider class is used to customize the look of wxWidgets application.
    /// - [`ArtProvider`] represents a C++ `wxArtProvider` class instance which your code has ownership, [`ArtProviderInRust`]`<false>` represents one which don't own.
    /// - Use [`ArtProvider`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxArtProvider` class's documentation](https://docs.wxwidgets.org/3.2/classwx_art_provider.html) for more details.
    #[doc(alias = "wxArtProvider")]
    #[doc(alias = "ArtProvider")]
    class ArtProvider
        = ArtProviderInRust<true>(wxArtProvider) impl
        ArtProviderMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> ArtProviderInRust<IN_RUST> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ArtProviderInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<ArtProviderInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: ArtProviderInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for ArtProviderInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxArtProvider_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for ArtProviderInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxAutoBufferedPaintDC
wxwidgets! {
    /// This wxDC derivative can be used inside of an EVT_PAINT() event handler to achieve double-buffered drawing.
    /// - [`AutoBufferedPaintDC`] represents a C++ `wxAutoBufferedPaintDC` class instance which your code has ownership, [`AutoBufferedPaintDCInRust`]`<false>` represents one which don't own.
    /// - Use [`AutoBufferedPaintDC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxAutoBufferedPaintDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_auto_buffered_paint_d_c.html) for more details.
    #[doc(alias = "wxAutoBufferedPaintDC")]
    #[doc(alias = "AutoBufferedPaintDC")]
    class AutoBufferedPaintDC
        = AutoBufferedPaintDCInRust<true>(wxAutoBufferedPaintDC) impl
        AutoBufferedPaintDCMethods,
        BufferedPaintDCMethods,
        BufferedDCMethods,
        MemoryDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> AutoBufferedPaintDCInRust<IN_RUST> {
    /// Constructor.
    ///
    /// See [C++ `wxAutoBufferedPaintDC::wxAutoBufferedPaintDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_auto_buffered_paint_d_c.html#a80468adfa451fbec5345ba8c32ae01b1).
    pub fn new<W: WindowMethods>(window: Option<&W>) -> AutoBufferedPaintDCInRust<IN_RUST> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            AutoBufferedPaintDCInRust(ffi::wxAutoBufferedPaintDC_new(window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AutoBufferedPaintDCInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<AutoBufferedPaintDCInRust<IN_RUST>>
    for BufferedPaintDCInRust<IN_RUST>
{
    fn from(o: AutoBufferedPaintDCInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<AutoBufferedPaintDCInRust<IN_RUST>> for BufferedDCInRust<IN_RUST> {
    fn from(o: AutoBufferedPaintDCInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<AutoBufferedPaintDCInRust<IN_RUST>> for MemoryDCInRust<IN_RUST> {
    fn from(o: AutoBufferedPaintDCInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<AutoBufferedPaintDCInRust<IN_RUST>> for DCInRust<IN_RUST> {
    fn from(o: AutoBufferedPaintDCInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<AutoBufferedPaintDCInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: AutoBufferedPaintDCInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for AutoBufferedPaintDCInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxAutoBufferedPaintDC_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for AutoBufferedPaintDCInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
