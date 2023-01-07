use super::*;

// wxAboutDialogInfo
wxwidgets! {
    /// wxAboutDialogInfo contains information shown in the standard About dialog displayed by the wxAboutBox() function.
    /// - [`AboutDialogInfo`] represents a C++ `wxAboutDialogInfo` class instance which your code has ownership, [`AboutDialogInfoFromCpp`]`<false>` represents one which don't own.
    /// - Use [`AboutDialogInfo`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxAboutDialogInfo` class's documentation](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html) for more details.
    #[doc(alias = "wxAboutDialogInfo")]
    #[doc(alias = "AboutDialogInfo")]
    class AboutDialogInfo
        = AboutDialogInfoFromCpp<true>(wxAboutDialogInfo) impl
        AboutDialogInfoMethods
}
impl<const FROM_CPP: bool> AboutDialogInfoFromCpp<FROM_CPP> {
    /// Default constructor leaves all fields are initially uninitialized, in general you should call at least SetVersion(), SetCopyright() and SetDescription().
    ///
    /// See [C++ `wxAboutDialogInfo::wxAboutDialogInfo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#ab58b7d0d7bb3aa40eca4531c4f8c1e6f).
    pub fn new() -> AboutDialogInfoFromCpp<FROM_CPP> {
        unsafe { AboutDialogInfoFromCpp(ffi::wxAboutDialogInfo_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AboutDialogInfoFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for AboutDialogInfoFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxAboutDialogInfo_delete(self.0) }
        }
    }
}

// wxAcceleratorEntry
wxwidgets! {
    /// An object used by an application wishing to create an accelerator table (see wxAcceleratorTable).
    /// - [`AcceleratorEntry`] represents a C++ `wxAcceleratorEntry` class instance which your code has ownership, [`AcceleratorEntryFromCpp`]`<false>` represents one which don't own.
    /// - Use [`AcceleratorEntry`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxAcceleratorEntry` class's documentation](https://docs.wxwidgets.org/3.2/classwx_accelerator_entry.html) for more details.
    #[doc(alias = "wxAcceleratorEntry")]
    #[doc(alias = "AcceleratorEntry")]
    class AcceleratorEntry
        = AcceleratorEntryFromCpp<true>(wxAcceleratorEntry) impl
        AcceleratorEntryMethods
}
impl<const FROM_CPP: bool> AcceleratorEntryFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxAcceleratorEntry::wxAcceleratorEntry()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_accelerator_entry.html#a9387fbbaed1cbfb8673308ec2b10da3e).
    pub fn new_with_int<M: MenuItemMethods>(
        flags: c_int,
        key_code: c_int,
        cmd: c_int,
        item: Option<&M>,
    ) -> AcceleratorEntryFromCpp<FROM_CPP> {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            AcceleratorEntryFromCpp(ffi::wxAcceleratorEntry_new(flags, key_code, cmd, item))
        }
    }
    /// Copy ctor.
    ///
    /// See [C++ `wxAcceleratorEntry::wxAcceleratorEntry()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_accelerator_entry.html#a8853051e5706c505d15ff70e1e44c7df).
    pub fn new_with_acceleratorentry<A: AcceleratorEntryMethods>(
        entry: &A,
    ) -> AcceleratorEntryFromCpp<FROM_CPP> {
        unsafe {
            let entry = entry.as_ptr();
            AcceleratorEntryFromCpp(ffi::wxAcceleratorEntry_new1(entry))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AcceleratorEntryFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for AcceleratorEntryFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxAcceleratorEntry_delete(self.0) }
        }
    }
}

// wxAcceleratorTable
wxwidgets! {
    /// An accelerator table allows the application to specify a table of keyboard shortcuts for menu or button commands.
    /// - [`AcceleratorTable`] represents a C++ `wxAcceleratorTable` class instance which your code has ownership, [`AcceleratorTableFromCpp`]`<false>` represents one which don't own.
    /// - Use [`AcceleratorTable`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxAcceleratorTable` class's documentation](https://docs.wxwidgets.org/3.2/classwx_accelerator_table.html) for more details.
    #[doc(alias = "wxAcceleratorTable")]
    #[doc(alias = "AcceleratorTable")]
    class AcceleratorTable
        = AcceleratorTableFromCpp<true>(wxAcceleratorTable) impl
        AcceleratorTableMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> AcceleratorTableFromCpp<FROM_CPP> {
    /// Default ctor.
    ///
    /// See [C++ `wxAcceleratorTable::wxAcceleratorTable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_accelerator_table.html#af172242a8a1487aa326f7965857df7f7).
    pub fn new() -> AcceleratorTableFromCpp<FROM_CPP> {
        unsafe { AcceleratorTableFromCpp(ffi::wxAcceleratorTable_new()) }
    }
    // NOT_SUPPORTED: fn wxAcceleratorTable1()
    // BLOCKED: fn wxAcceleratorTable2()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AcceleratorTableFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<AcceleratorTableFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: AcceleratorTableFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for AcceleratorTableFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxAcceleratorTable_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for AcceleratorTableFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxActivateEvent
wxwidgets! {
    /// An activate event is sent when a window or application is being activated or deactivated.
    /// - [`ActivateEvent`] represents a C++ `wxActivateEvent` class instance which your code has ownership, [`ActivateEventFromCpp`]`<false>` represents one which don't own.
    /// - Use [`ActivateEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxActivateEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_activate_event.html) for more details.
    #[doc(alias = "wxActivateEvent")]
    #[doc(alias = "ActivateEvent")]
    class ActivateEvent
        = ActivateEventFromCpp<true>(wxActivateEvent) impl
        ActivateEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ActivateEventFromCpp<FROM_CPP> {
    //  ENUM: Reason
    pub const Reason_Mouse: c_int = 0;
    pub const Reason_Unknown: c_int = 0 + 1;

    // NOT_SUPPORTED: fn wxActivateEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ActivateEventFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ActivateEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: ActivateEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ActivateEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ActivateEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ActivateEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxActivateEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ActivateEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxAffineMatrix2D
wxwidgets! {
    /// A 3x2 matrix representing an affine 2D transformation.
    /// - [`AffineMatrix2D`] represents a C++ `wxAffineMatrix2D` class instance which your code has ownership, [`AffineMatrix2DFromCpp`]`<false>` represents one which don't own.
    /// - Use [`AffineMatrix2D`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxAffineMatrix2D` class's documentation](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d.html) for more details.
    #[doc(alias = "wxAffineMatrix2D")]
    #[doc(alias = "AffineMatrix2D")]
    class AffineMatrix2D
        = AffineMatrix2DFromCpp<true>(wxAffineMatrix2D) impl
        AffineMatrix2DMethods
        // AffineMatrix2DBaseMethods
}
impl<const FROM_CPP: bool> AffineMatrix2DFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxAffineMatrix2D::wxAffineMatrix2D()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d.html#a00fa188b54418b476d122e4de408dc27).
    pub fn new() -> AffineMatrix2DFromCpp<FROM_CPP> {
        unsafe { AffineMatrix2DFromCpp(ffi::wxAffineMatrix2D_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AffineMatrix2DFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<AffineMatrix2DFromCpp<FROM_CPP>>
    for AffineMatrix2DBaseFromCpp<FROM_CPP>
{
    fn from(o: AffineMatrix2DFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for AffineMatrix2DFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxAffineMatrix2D_delete(self.0) }
        }
    }
}
impl<const FROM_CPP: bool> AffineMatrix2DBaseMethods for AffineMatrix2DFromCpp<FROM_CPP> {
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
    /// - [`AffineMatrix2DBase`] represents a C++ `wxAffineMatrix2DBase` class instance which your code has ownership, [`AffineMatrix2DBaseFromCpp`]`<false>` represents one which don't own.
    /// - Use [`AffineMatrix2DBase`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxAffineMatrix2DBase` class's documentation](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d_base.html) for more details.
    #[doc(alias = "wxAffineMatrix2DBase")]
    #[doc(alias = "AffineMatrix2DBase")]
    class AffineMatrix2DBase
        = AffineMatrix2DBaseFromCpp<true>(wxAffineMatrix2DBase) impl
        AffineMatrix2DBaseMethods
}
impl<const FROM_CPP: bool> AffineMatrix2DBaseFromCpp<FROM_CPP> {
    // BLOCKED: fn wxAffineMatrix2DBase()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AffineMatrix2DBaseFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for AffineMatrix2DBaseFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxAffineMatrix2DBase_delete(self.0) }
        }
    }
}

// wxAnimationCtrl
wxwidgets! {
    /// This is a static control which displays an animation.
    /// - [`AnimationCtrl`] represents a C++ `wxAnimationCtrl` class instance which your code has ownership, [`AnimationCtrlFromCpp`]`<false>` represents one which don't own.
    /// - Use [`AnimationCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxAnimationCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_animation_ctrl.html) for more details.
    #[doc(alias = "wxAnimationCtrl")]
    #[doc(alias = "AnimationCtrl")]
    class AnimationCtrl
        = AnimationCtrlFromCpp<true>(wxAnimationCtrl) impl
        AnimationCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> AnimationCtrlFromCpp<FROM_CPP> {
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
    ) -> AnimationCtrlFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            AnimationCtrlFromCpp(ffi::wxAnimationCtrl_new(
                parent, id, anim, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for AnimationCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<AnimationCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: AnimationCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<AnimationCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: AnimationCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<AnimationCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: AnimationCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<AnimationCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: AnimationCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for AnimationCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxAnimationCtrl_CLASSINFO()) }
    }
}

// wxAnyButton
wxwidgets! {
    /// A class for common button functionality used as the base for the various button classes.
    /// - [`AnyButton`] represents a C++ `wxAnyButton` class instance which your code has ownership, [`AnyButtonFromCpp`]`<false>` represents one which don't own.
    /// - Use [`AnyButton`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxAnyButton` class's documentation](https://docs.wxwidgets.org/3.2/classwx_any_button.html) for more details.
    #[doc(alias = "wxAnyButton")]
    #[doc(alias = "AnyButton")]
    class AnyButton
        = AnyButtonFromCpp<true>(wxAnyButton) impl
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> AnyButtonFromCpp<FROM_CPP> {
    ///
    /// See [C++ `wxAnyButton::wxAnyButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_any_button.html#a89b895988e816974fa5f5971e2f3e2a4).
    pub fn new() -> AnyButtonFromCpp<FROM_CPP> {
        unsafe { AnyButtonFromCpp(ffi::wxAnyButton_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for AnyButtonFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<AnyButtonFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: AnyButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<AnyButtonFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: AnyButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<AnyButtonFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: AnyButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<AnyButtonFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: AnyButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for AnyButtonFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxAnyButton_CLASSINFO()) }
    }
}

// wxArtProvider
wxwidgets! {
    /// wxArtProvider class is used to customize the look of wxWidgets application.
    /// - [`ArtProvider`] represents a C++ `wxArtProvider` class instance which your code has ownership, [`ArtProviderFromCpp`]`<false>` represents one which don't own.
    /// - Use [`ArtProvider`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxArtProvider` class's documentation](https://docs.wxwidgets.org/3.2/classwx_art_provider.html) for more details.
    #[doc(alias = "wxArtProvider")]
    #[doc(alias = "ArtProvider")]
    class ArtProvider
        = ArtProviderFromCpp<true>(wxArtProvider) impl
        ArtProviderMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ArtProviderFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ArtProviderFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ArtProviderFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ArtProviderFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ArtProviderFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxArtProvider_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ArtProviderFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxAutoBufferedPaintDC
wxwidgets! {
    /// This wxDC derivative can be used inside of an EVT_PAINT() event handler to achieve double-buffered drawing.
    /// - [`AutoBufferedPaintDC`] represents a C++ `wxAutoBufferedPaintDC` class instance which your code has ownership, [`AutoBufferedPaintDCFromCpp`]`<false>` represents one which don't own.
    /// - Use [`AutoBufferedPaintDC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxAutoBufferedPaintDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_auto_buffered_paint_d_c.html) for more details.
    #[doc(alias = "wxAutoBufferedPaintDC")]
    #[doc(alias = "AutoBufferedPaintDC")]
    class AutoBufferedPaintDC
        = AutoBufferedPaintDCFromCpp<true>(wxAutoBufferedPaintDC) impl
        AutoBufferedPaintDCMethods,
        BufferedPaintDCMethods,
        BufferedDCMethods,
        MemoryDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> AutoBufferedPaintDCFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxAutoBufferedPaintDC::wxAutoBufferedPaintDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_auto_buffered_paint_d_c.html#a80468adfa451fbec5345ba8c32ae01b1).
    pub fn new<W: WindowMethods>(window: Option<&W>) -> AutoBufferedPaintDCFromCpp<FROM_CPP> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            AutoBufferedPaintDCFromCpp(ffi::wxAutoBufferedPaintDC_new(window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for AutoBufferedPaintDCFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<AutoBufferedPaintDCFromCpp<FROM_CPP>>
    for BufferedPaintDCFromCpp<FROM_CPP>
{
    fn from(o: AutoBufferedPaintDCFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<AutoBufferedPaintDCFromCpp<FROM_CPP>>
    for BufferedDCFromCpp<FROM_CPP>
{
    fn from(o: AutoBufferedPaintDCFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<AutoBufferedPaintDCFromCpp<FROM_CPP>>
    for MemoryDCFromCpp<FROM_CPP>
{
    fn from(o: AutoBufferedPaintDCFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<AutoBufferedPaintDCFromCpp<FROM_CPP>> for DCFromCpp<FROM_CPP> {
    fn from(o: AutoBufferedPaintDCFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<AutoBufferedPaintDCFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: AutoBufferedPaintDCFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for AutoBufferedPaintDCFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxAutoBufferedPaintDC_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for AutoBufferedPaintDCFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
