#![allow(non_upper_case_globals)]

use super::*;

// wxAboutDialogInfo
wx_class! { AboutDialogInfo =
    AboutDialogInfoIsOwned<true>(wxAboutDialogInfo) impl
        AboutDialogInfoMethods
}
impl<const OWNED: bool> AboutDialogInfoIsOwned<OWNED> {
    pub fn new() -> AboutDialogInfoIsOwned<OWNED> {
        unsafe { AboutDialogInfoIsOwned(ffi::wxAboutDialogInfo_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
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
wx_class! { AcceleratorEntry =
    AcceleratorEntryIsOwned<true>(wxAcceleratorEntry) impl
        AcceleratorEntryMethods
}
impl<const OWNED: bool> AcceleratorEntryIsOwned<OWNED> {
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
impl<const OWNED: bool> Drop for AcceleratorEntryIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxAcceleratorEntry_delete(self.0) }
        }
    }
}

// wxAcceleratorTable
wx_class! { AcceleratorTable =
    AcceleratorTableIsOwned<true>(wxAcceleratorTable) impl
        AcceleratorTableMethods,
        ObjectMethods
}
impl<const OWNED: bool> AcceleratorTableIsOwned<OWNED> {
    pub fn new() -> AcceleratorTableIsOwned<OWNED> {
        unsafe { AcceleratorTableIsOwned(ffi::wxAcceleratorTable_new()) }
    }
    // NOT_SUPPORTED: fn wxAcceleratorTable1()
    // BLOCKED: fn wxAcceleratorTable2()
    pub fn none() -> Option<&'static Self> {
        None
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
wx_class! { ActivateEvent =
    ActivateEventIsOwned<true>(wxActivateEvent) impl
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
wx_class! { AffineMatrix2D =
    AffineMatrix2DIsOwned<true>(wxAffineMatrix2D) impl
        AffineMatrix2DMethods
        // AffineMatrix2DBaseMethods
}
impl<const OWNED: bool> AffineMatrix2DIsOwned<OWNED> {
    pub fn new() -> AffineMatrix2DIsOwned<OWNED> {
        unsafe { AffineMatrix2DIsOwned(ffi::wxAffineMatrix2D_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
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
wx_class! { AffineMatrix2DBase =
    AffineMatrix2DBaseIsOwned<true>(wxAffineMatrix2DBase) impl
        AffineMatrix2DBaseMethods
}
impl<const OWNED: bool> AffineMatrix2DBaseIsOwned<OWNED> {
    // BLOCKED: fn wxAffineMatrix2DBase()
    pub fn none() -> Option<&'static Self> {
        None
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
wx_class! { AnimationCtrl =
    AnimationCtrlIsOwned<true>(wxAnimationCtrl) impl
        AnimationCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> AnimationCtrlIsOwned<OWNED> {
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
wx_class! { AnyButton =
    AnyButtonIsOwned<true>(wxAnyButton) impl
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

// wxAppProgressIndicator
wx_class! { AppProgressIndicator =
    AppProgressIndicatorIsOwned<true>(wxAppProgressIndicator) impl
        AppProgressIndicatorMethods
}
impl<const OWNED: bool> AppProgressIndicatorIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(
        parent: Option<&W>,
        max_value: c_int,
    ) -> AppProgressIndicatorIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            AppProgressIndicatorIsOwned(ffi::wxAppProgressIndicator_new(parent, max_value))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for AppProgressIndicatorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxAppProgressIndicator_delete(self.0) }
        }
    }
}

// wxArtProvider
wx_class! { ArtProvider =
    ArtProviderIsOwned<true>(wxArtProvider) impl
        ArtProviderMethods,
        ObjectMethods
}
impl<const OWNED: bool> ArtProviderIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
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
