#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

use std::mem;
use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};
use std::ptr;

use super::*;
use methods::*;

use wx_base::methods::*;
use wx_base::*;

mod ffi;
pub mod methods;

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
    pub fn new_with_str(resource: &str) -> AcceleratorTableIsOwned<OWNED> {
        unsafe {
            let resource = WxString::from(resource);
            let resource = resource.as_ptr();
            AcceleratorTableIsOwned(ffi::wxAcceleratorTable_new2(resource))
        }
    }
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

// wxAccessible
wx_class! { Accessible =
    AccessibleIsOwned<true>(wxAccessible) impl
        AccessibleMethods,
        ObjectMethods
}
impl<const OWNED: bool> AccessibleIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(win: Option<&W>) -> AccessibleIsOwned<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            AccessibleIsOwned(ffi::wxAccessible_new(win))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<AccessibleIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: AccessibleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for AccessibleIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxAccessible_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for AccessibleIsOwned<OWNED> {
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

// wxActiveXContainer
wx_class! { ActiveXContainer =
    ActiveXContainerIsOwned<true>(wxActiveXContainer) impl
        ActiveXContainerMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ActiveXContainerIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxActiveXContainer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ActiveXContainerIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ActiveXContainerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ActiveXContainerIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ActiveXContainerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ActiveXContainerIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ActiveXContainerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ActiveXContainerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ActiveXContainerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ActiveXContainerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxActiveXContainer_CLASSINFO()) }
    }
}
// Mix-in(s) to wxActiveXContainer
impl<const OWNED: bool> TrackableMethods for ActiveXContainerIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxActiveXContainer_AsTrackable(self.as_ptr()) }
    }
}

// wxActiveXEvent
wx_class! { ActiveXEvent =
    ActiveXEventIsOwned<true>(wxActiveXEvent) impl
        ActiveXEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ActiveXEventIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ActiveXEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: ActiveXEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ActiveXEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ActiveXEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ActiveXEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ActiveXEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ActiveXEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxActiveXEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ActiveXEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxActivityIndicator
wx_class! { ActivityIndicator =
    ActivityIndicatorIsOwned<true>(wxActivityIndicator) impl
        ActivityIndicatorMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ActivityIndicatorIsOwned<OWNED> {
    pub fn new_2step() -> ActivityIndicatorIsOwned<OWNED> {
        unsafe { ActivityIndicatorIsOwned(ffi::wxActivityIndicator_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        winid: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> ActivityIndicatorIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ActivityIndicatorIsOwned(ffi::wxActivityIndicator_new1(
                parent, winid, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ActivityIndicatorIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ActivityIndicatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ActivityIndicatorIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ActivityIndicatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ActivityIndicatorIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ActivityIndicatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ActivityIndicatorIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ActivityIndicatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ActivityIndicatorIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxActivityIndicator_CLASSINFO()) }
    }
}
// Mix-in(s) to wxActivityIndicator
impl<const OWNED: bool> TrackableMethods for ActivityIndicatorIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxActivityIndicator_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> WindowMethods for ActivityIndicatorIsOwned<OWNED> {
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        winid: c_int,
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
            ffi::wxActivityIndicator_Create(self.as_ptr(), parent, winid, pos, size, style, name)
        }
    }
}

// wxAddRemoveAdaptor
wx_class! { AddRemoveAdaptor =
    AddRemoveAdaptorIsOwned<true>(wxAddRemoveAdaptor) impl
        AddRemoveAdaptorMethods
}
impl<const OWNED: bool> AddRemoveAdaptorIsOwned<OWNED> {
    pub fn new() -> AddRemoveAdaptorIsOwned<OWNED> {
        unsafe { AddRemoveAdaptorIsOwned(ffi::wxAddRemoveAdaptor_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for AddRemoveAdaptorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxAddRemoveAdaptor_delete(self.0) }
        }
    }
}

// wxAddRemoveCtrl
wx_class! { AddRemoveCtrl =
    AddRemoveCtrlIsOwned<true>(wxAddRemoveCtrl) impl
        AddRemoveCtrlMethods,
        // PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> AddRemoveCtrlIsOwned<OWNED> {
    pub fn new_2step() -> AddRemoveCtrlIsOwned<OWNED> {
        unsafe { AddRemoveCtrlIsOwned(ffi::wxAddRemoveCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        winid: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> AddRemoveCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            AddRemoveCtrlIsOwned(ffi::wxAddRemoveCtrl_new1(
                parent, winid, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<AddRemoveCtrlIsOwned<OWNED>> for PanelIsOwned<OWNED> {
    fn from(o: AddRemoveCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<AddRemoveCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: AddRemoveCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<AddRemoveCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: AddRemoveCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<AddRemoveCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: AddRemoveCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for AddRemoveCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxAddRemoveCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxAddRemoveCtrl
impl<const OWNED: bool> TrackableMethods for AddRemoveCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxAddRemoveCtrl_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> PanelMethods for AddRemoveCtrlIsOwned<OWNED> {
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        winid: c_int,
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
            ffi::wxAddRemoveCtrl_Create(self.as_ptr(), parent, winid, pos, size, style, name)
        }
    }
}
impl<const OWNED: bool> WindowMethods for AddRemoveCtrlIsOwned<OWNED> {
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        winid: c_int,
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
            ffi::wxAddRemoveCtrl_Create(self.as_ptr(), parent, winid, pos, size, style, name)
        }
    }
}

// wxAffineMatrix2D
wx_class! { AffineMatrix2D =
    AffineMatrix2DIsOwned<true>(wxAffineMatrix2D) impl
        AffineMatrix2DMethods,
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
    fn transform_point_point2ddouble<P: Point2DDoubleMethods>(&self, p: &P) -> Point2DDouble {
        unsafe {
            let p = p.as_ptr();
            Point2DDouble::from_ptr(ffi::wxAffineMatrix2D_TransformPoint(self.as_ptr(), p))
        }
    }
    fn transform_point_double(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxAffineMatrix2D_TransformPoint1(self.as_ptr(), x, y) }
    }
    fn transform_distance_point2ddouble<P: Point2DDoubleMethods>(&self, p: &P) -> Point2DDouble {
        unsafe {
            let p = p.as_ptr();
            Point2DDouble::from_ptr(ffi::wxAffineMatrix2D_TransformDistance(self.as_ptr(), p))
        }
    }
    fn transform_distance_double(&self, dx: *mut c_void, dy: *mut c_void) {
        unsafe { ffi::wxAffineMatrix2D_TransformDistance1(self.as_ptr(), dx, dy) }
    }
}

// wxAffineMatrix2DBase
wx_class! { AffineMatrix2DBase =
    AffineMatrix2DBaseIsOwned<true>(wxAffineMatrix2DBase) impl
        AffineMatrix2DBaseMethods
}
impl<const OWNED: bool> AffineMatrix2DBaseIsOwned<OWNED> {
    pub fn new() -> AffineMatrix2DBaseIsOwned<OWNED> {
        unsafe { AffineMatrix2DBaseIsOwned(ffi::wxAffineMatrix2DBase_new()) }
    }
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
    pub fn new<W: WindowMethods, A: AnimationMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        anim: &A,
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
            let anim = anim.as_ptr();
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
// Mix-in(s) to wxAnimationCtrl
impl<const OWNED: bool> TrackableMethods for AnimationCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxAnimationCtrl_AsTrackable(self.as_ptr()) }
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
// Mix-in(s) to wxAnyButton
impl<const OWNED: bool> TrackableMethods for AnyButtonIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxAnyButton_AsTrackable(self.as_ptr()) }
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

// wxAutoBufferedPaintDC
wx_class! { AutoBufferedPaintDC =
    AutoBufferedPaintDCIsOwned<true>(wxAutoBufferedPaintDC) impl
        AutoBufferedPaintDCMethods,
        BufferedPaintDCMethods,
        BufferedDCMethods,
        MemoryDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> AutoBufferedPaintDCIsOwned<OWNED> {
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

// wxAutomationObject
wx_class! { AutomationObject =
    AutomationObjectIsOwned<true>(wxAutomationObject) impl
        AutomationObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> AutomationObjectIsOwned<OWNED> {
    pub fn new(dispatch_ptr: *mut c_void) -> AutomationObjectIsOwned<OWNED> {
        unsafe { AutomationObjectIsOwned(ffi::wxAutomationObject_new(dispatch_ptr)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<AutomationObjectIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: AutomationObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for AutomationObjectIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxAutomationObject_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for AutomationObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBannerWindow
wx_class! { BannerWindow =
    BannerWindowIsOwned<true>(wxBannerWindow) impl
        BannerWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BannerWindowIsOwned<OWNED> {
    pub fn new_2step() -> BannerWindowIsOwned<OWNED> {
        unsafe { BannerWindowIsOwned(ffi::wxBannerWindow_new()) }
    }
    pub fn new<W: WindowMethods>(parent: Option<&W>, dir: c_int) -> BannerWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            BannerWindowIsOwned(ffi::wxBannerWindow_new1(parent, dir))
        }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        winid: c_int,
        dir: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> BannerWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            BannerWindowIsOwned(ffi::wxBannerWindow_new2(
                parent, winid, dir, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<BannerWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: BannerWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BannerWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: BannerWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BannerWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BannerWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BannerWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBannerWindow_CLASSINFO()) }
    }
}
// Mix-in(s) to wxBannerWindow
impl<const OWNED: bool> TrackableMethods for BannerWindowIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxBannerWindow_AsTrackable(self.as_ptr()) }
    }
}

// wxBitmap
wx_class! { Bitmap =
    BitmapIsOwned<true>(wxBitmap) impl
        BitmapMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> BitmapIsOwned<OWNED> {
    pub fn new() -> BitmapIsOwned<OWNED> {
        unsafe { BitmapIsOwned(ffi::wxBitmap_new()) }
    }
    pub fn new_with_bitmap<B: BitmapMethods>(bitmap: &B) -> BitmapIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            BitmapIsOwned(ffi::wxBitmap_new1(bitmap))
        }
    }
    // NOT_SUPPORTED: fn wxBitmap2()
    pub fn new_with_int_int(width: c_int, height: c_int, depth: c_int) -> BitmapIsOwned<OWNED> {
        unsafe { BitmapIsOwned(ffi::wxBitmap_new3(width, height, depth)) }
    }
    pub fn new_with_size<S: SizeMethods>(sz: &S, depth: c_int) -> BitmapIsOwned<OWNED> {
        unsafe {
            let sz = sz.as_ptr();
            BitmapIsOwned(ffi::wxBitmap_new4(sz, depth))
        }
    }
    pub fn new_with_int_dc<D: DCMethods>(
        width: c_int,
        height: c_int,
        dc: &D,
    ) -> BitmapIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            BitmapIsOwned(ffi::wxBitmap_new5(width, height, dc))
        }
    }
    pub fn new_with_char(bits: *const c_void) -> BitmapIsOwned<OWNED> {
        unsafe { BitmapIsOwned(ffi::wxBitmap_new6(bits)) }
    }
    // NOT_SUPPORTED: fn wxBitmap7()
    pub fn new_with_image_int<I: ImageMethods>(img: &I, depth: c_int) -> BitmapIsOwned<OWNED> {
        unsafe {
            let img = img.as_ptr();
            BitmapIsOwned(ffi::wxBitmap_new8(img, depth))
        }
    }
    pub fn new_with_image_dc<I: ImageMethods, D: DCMethods>(
        img: &I,
        dc: &D,
    ) -> BitmapIsOwned<OWNED> {
        unsafe {
            let img = img.as_ptr();
            let dc = dc.as_ptr();
            BitmapIsOwned(ffi::wxBitmap_new9(img, dc))
        }
    }
    pub fn new_with_cursor<C: CursorMethods>(cursor: &C) -> BitmapIsOwned<OWNED> {
        unsafe {
            let cursor = cursor.as_ptr();
            BitmapIsOwned(ffi::wxBitmap_new10(cursor))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<BitmapIsOwned<OWNED>> for GDIObjectIsOwned<OWNED> {
    fn from(o: BitmapIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BitmapIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BitmapIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBitmap_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BitmapIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBitmapBundle
wx_class! { BitmapBundle =
    BitmapBundleIsOwned<true>(wxBitmapBundle) impl
        BitmapBundleMethods
}
impl<const OWNED: bool> BitmapBundleIsOwned<OWNED> {
    pub fn new() -> BitmapBundleIsOwned<OWNED> {
        unsafe { BitmapBundleIsOwned(ffi::wxBitmapBundle_new()) }
    }
    pub fn new_with_bitmap<B: BitmapMethods>(bitmap: &B) -> BitmapBundleIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            BitmapBundleIsOwned(ffi::wxBitmapBundle_new1(bitmap))
        }
    }
    pub fn new_with_icon<I: IconMethods>(icon: &I) -> BitmapBundleIsOwned<OWNED> {
        unsafe {
            let icon = icon.as_ptr();
            BitmapBundleIsOwned(ffi::wxBitmapBundle_new2(icon))
        }
    }
    pub fn new_with_image<I: ImageMethods>(image: &I) -> BitmapBundleIsOwned<OWNED> {
        unsafe {
            let image = image.as_ptr();
            BitmapBundleIsOwned(ffi::wxBitmapBundle_new3(image))
        }
    }
    pub fn new_with_char(xpm: *const c_void) -> BitmapBundleIsOwned<OWNED> {
        unsafe { BitmapBundleIsOwned(ffi::wxBitmapBundle_new4(xpm)) }
    }
    pub fn new_with_bitmapbundle<B: BitmapBundleMethods>(other: &B) -> BitmapBundleIsOwned<OWNED> {
        unsafe {
            let other = other.as_ptr();
            BitmapBundleIsOwned(ffi::wxBitmapBundle_new5(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for BitmapBundleIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxBitmapBundle_delete(self.0) }
        }
    }
}

// wxBitmapBundleImpl
wx_class! { BitmapBundleImpl =
    BitmapBundleImplIsOwned<true>(wxBitmapBundleImpl) impl
        BitmapBundleImplMethods,
        RefCounterMethods
}
impl<const OWNED: bool> BitmapBundleImplIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<BitmapBundleImplIsOwned<OWNED>> for RefCounterIsOwned<OWNED> {
    fn from(o: BitmapBundleImplIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for BitmapBundleImplIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxBitmapBundleImpl_delete(self.0) }
        }
    }
}

// wxBitmapButton
wx_class! { BitmapButton =
    BitmapButtonIsOwned<true>(wxBitmapButton) impl
        BitmapButtonMethods,
        ButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BitmapButtonIsOwned<OWNED> {
    pub fn new_2step() -> BitmapButtonIsOwned<OWNED> {
        unsafe { BitmapButtonIsOwned(ffi::wxBitmapButton_new()) }
    }
    pub fn new<
        W: WindowMethods,
        B: BitmapBundleMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        bitmap: &B,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> BitmapButtonIsOwned<OWNED> {
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
            BitmapButtonIsOwned(ffi::wxBitmapButton_new1(
                parent, id, bitmap, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<BitmapButtonIsOwned<OWNED>> for ButtonIsOwned<OWNED> {
    fn from(o: BitmapButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapButtonIsOwned<OWNED>> for AnyButtonIsOwned<OWNED> {
    fn from(o: BitmapButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapButtonIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: BitmapButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapButtonIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: BitmapButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapButtonIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: BitmapButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapButtonIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BitmapButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BitmapButtonIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBitmapButton_CLASSINFO()) }
    }
}
// Mix-in(s) to wxBitmapButton
impl<const OWNED: bool> TrackableMethods for BitmapButtonIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxBitmapButton_AsTrackable(self.as_ptr()) }
    }
}

// wxBitmapComboBox
wx_class! { BitmapComboBox =
    BitmapComboBoxIsOwned<true>(wxBitmapComboBox) impl
        BitmapComboBoxMethods,
        // ComboBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BitmapComboBoxIsOwned<OWNED> {
    pub fn new_2step() -> BitmapComboBoxIsOwned<OWNED> {
        unsafe { BitmapComboBoxIsOwned(ffi::wxBitmapComboBox_new()) }
    }
    // NOT_SUPPORTED: fn wxBitmapComboBox1()
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> BitmapComboBoxIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            BitmapComboBoxIsOwned(ffi::wxBitmapComboBox_new2(
                parent, id, value, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<BitmapComboBoxIsOwned<OWNED>> for ComboBoxIsOwned<OWNED> {
    fn from(o: BitmapComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapComboBoxIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: BitmapComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapComboBoxIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: BitmapComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapComboBoxIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: BitmapComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapComboBoxIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BitmapComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BitmapComboBoxIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBitmapComboBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxBitmapComboBox
impl<const OWNED: bool> ItemContainerMethods for BitmapComboBoxIsOwned<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxBitmapComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for BitmapComboBoxIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxBitmapComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TextEntryMethods for BitmapComboBoxIsOwned<OWNED> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxBitmapComboBox_AsTextEntry(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for BitmapComboBoxIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxBitmapComboBox_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ComboBoxMethods for BitmapComboBoxIsOwned<OWNED> {
    // NOT_SUPPORTED: fn Create()
    fn create_str<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        value: &str,
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
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxBitmapComboBox_Create1(
                self.as_ptr(),
                parent,
                id,
                value,
                pos,
                size,
                choices,
                style,
                validator,
                name,
            )
        }
    }
}

// wxBitmapDataObject
wx_class! { BitmapDataObject =
    BitmapDataObjectIsOwned<true>(wxBitmapDataObject) impl
        BitmapDataObjectMethods,
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> BitmapDataObjectIsOwned<OWNED> {
    pub fn new<B: BitmapMethods>(bitmap: &B) -> BitmapDataObjectIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            BitmapDataObjectIsOwned(ffi::wxBitmapDataObject_new(bitmap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<BitmapDataObjectIsOwned<OWNED>> for DataObjectSimpleIsOwned<OWNED> {
    fn from(o: BitmapDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapDataObjectIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: BitmapDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for BitmapDataObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxBitmapDataObject_delete(self.0) }
        }
    }
}

// wxBitmapHandler
wx_class! { BitmapHandler =
    BitmapHandlerIsOwned<true>(wxBitmapHandler) impl
        BitmapHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BitmapHandlerIsOwned<OWNED> {
    pub fn new() -> BitmapHandlerIsOwned<OWNED> {
        unsafe { BitmapHandlerIsOwned(ffi::wxBitmapHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<BitmapHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BitmapHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BitmapHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBitmapHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BitmapHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBitmapToggleButton
wx_class! { BitmapToggleButton =
    BitmapToggleButtonIsOwned<true>(wxBitmapToggleButton) impl
        BitmapToggleButtonMethods,
        ToggleButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BitmapToggleButtonIsOwned<OWNED> {
    pub fn new_2step() -> BitmapToggleButtonIsOwned<OWNED> {
        unsafe { BitmapToggleButtonIsOwned(ffi::wxBitmapToggleButton_new()) }
    }
    pub fn new<
        W: WindowMethods,
        B: BitmapBundleMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        label: &B,
        pos: &P,
        size: &S,
        style: c_long,
        val: &V,
        name: &str,
    ) -> BitmapToggleButtonIsOwned<OWNED> {
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
            BitmapToggleButtonIsOwned(ffi::wxBitmapToggleButton_new1(
                parent, id, label, pos, size, style, val, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonIsOwned<OWNED>> for ToggleButtonIsOwned<OWNED> {
    fn from(o: BitmapToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonIsOwned<OWNED>> for AnyButtonIsOwned<OWNED> {
    fn from(o: BitmapToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: BitmapToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: BitmapToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: BitmapToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BitmapToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BitmapToggleButtonIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBitmapToggleButton_CLASSINFO()) }
    }
}
// Mix-in(s) to wxBitmapToggleButton
impl<const OWNED: bool> TrackableMethods for BitmapToggleButtonIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxBitmapToggleButton_AsTrackable(self.as_ptr()) }
    }
}

// wxBookCtrlBase
wx_class! { BookCtrlBase =
    BookCtrlBaseIsOwned<true>(wxBookCtrlBase) impl
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BookCtrlBaseIsOwned<OWNED> {
    //  ENUM: @3
    pub const NO_IMAGE: c_int = -1;

    // BLOCKED: fn wxBookCtrlBase()
    // BLOCKED: fn wxBookCtrlBase1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<BookCtrlBaseIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: BookCtrlBaseIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlBaseIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: BookCtrlBaseIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlBaseIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: BookCtrlBaseIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlBaseIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BookCtrlBaseIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BookCtrlBaseIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBookCtrlBase_CLASSINFO()) }
    }
}
// Mix-in(s) to wxBookCtrlBase
impl<const OWNED: bool> WithImagesMethods for BookCtrlBaseIsOwned<OWNED> {
    fn as_with_images(&self) -> *mut c_void {
        unsafe { ffi::wxBookCtrlBase_AsWithImages(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for BookCtrlBaseIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxBookCtrlBase_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> WindowMethods for BookCtrlBaseIsOwned<OWNED> {
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        winid: c_int,
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
            ffi::wxBookCtrlBase_Create(self.as_ptr(), parent, winid, pos, size, style, name)
        }
    }
}

// wxBookCtrlEvent
wx_class! { BookCtrlEvent =
    BookCtrlEventIsOwned<true>(wxBookCtrlEvent) impl
        BookCtrlEventMethods,
        NotifyEventMethods,
        // CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> BookCtrlEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxBookCtrlEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<BookCtrlEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: BookCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: BookCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: BookCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BookCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BookCtrlEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBookCtrlEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BookCtrlEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> CommandEventMethods for BookCtrlEventIsOwned<OWNED> {
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxBookCtrlEvent_GetSelection(self.as_ptr()) }
    }
}

// wxBoxSizer
wx_class! { BoxSizer =
    BoxSizerIsOwned<true>(wxBoxSizer) impl
        BoxSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BoxSizerIsOwned<OWNED> {
    pub fn new(orient: c_int) -> BoxSizerIsOwned<OWNED> {
        unsafe { BoxSizerIsOwned(ffi::wxBoxSizer_new(orient)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<BoxSizerIsOwned<OWNED>> for SizerIsOwned<OWNED> {
    fn from(o: BoxSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BoxSizerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BoxSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BoxSizerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBoxSizer_CLASSINFO()) }
    }
}

// wxBrush
wx_class! { Brush =
    BrushIsOwned<true>(wxBrush) impl
        BrushMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> BrushIsOwned<OWNED> {
    pub fn new() -> BrushIsOwned<OWNED> {
        unsafe { BrushIsOwned(ffi::wxBrush_new()) }
    }
    // NOT_SUPPORTED: fn wxBrush1()
    pub fn new_with_bitmap<B: BitmapMethods>(stipple_bitmap: &B) -> BrushIsOwned<OWNED> {
        unsafe {
            let stipple_bitmap = stipple_bitmap.as_ptr();
            BrushIsOwned(ffi::wxBrush_new2(stipple_bitmap))
        }
    }
    pub fn new_with_brush<B: BrushMethods>(brush: &B) -> BrushIsOwned<OWNED> {
        unsafe {
            let brush = brush.as_ptr();
            BrushIsOwned(ffi::wxBrush_new3(brush))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<BrushIsOwned<OWNED>> for GDIObjectIsOwned<OWNED> {
    fn from(o: BrushIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BrushIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BrushIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BrushIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBrush_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BrushIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBrushList
wx_class! { BrushList =
    BrushListIsOwned<true>(wxBrushList) impl
        BrushListMethods
}
impl<const OWNED: bool> BrushListIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for BrushListIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxBrushList_delete(self.0) }
        }
    }
}

// wxBufferedDC
wx_class! { BufferedDC =
    BufferedDCIsOwned<true>(wxBufferedDC) impl
        BufferedDCMethods,
        MemoryDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> BufferedDCIsOwned<OWNED> {
    pub fn new() -> BufferedDCIsOwned<OWNED> {
        unsafe { BufferedDCIsOwned(ffi::wxBufferedDC_new()) }
    }
    pub fn new_with_dc_size<D: DCMethods, S: SizeMethods>(
        dc: Option<&D>,
        area: &S,
        style: c_int,
    ) -> BufferedDCIsOwned<OWNED> {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let area = area.as_ptr();
            BufferedDCIsOwned(ffi::wxBufferedDC_new1(dc, area, style))
        }
    }
    pub fn new_with_dc_bitmap<D: DCMethods, B: BitmapMethods>(
        dc: Option<&D>,
        buffer: &B,
        style: c_int,
    ) -> BufferedDCIsOwned<OWNED> {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let buffer = buffer.as_ptr();
            BufferedDCIsOwned(ffi::wxBufferedDC_new2(dc, buffer, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<BufferedDCIsOwned<OWNED>> for MemoryDCIsOwned<OWNED> {
    fn from(o: BufferedDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BufferedDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: BufferedDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BufferedDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BufferedDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BufferedDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBufferedDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BufferedDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBufferedPaintDC
wx_class! { BufferedPaintDC =
    BufferedPaintDCIsOwned<true>(wxBufferedPaintDC) impl
        BufferedPaintDCMethods,
        BufferedDCMethods,
        MemoryDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> BufferedPaintDCIsOwned<OWNED> {
    pub fn new_with_bitmap<W: WindowMethods, B: BitmapMethods>(
        window: Option<&W>,
        buffer: &B,
        style: c_int,
    ) -> BufferedPaintDCIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let buffer = buffer.as_ptr();
            BufferedPaintDCIsOwned(ffi::wxBufferedPaintDC_new(window, buffer, style))
        }
    }
    pub fn new_with_int<W: WindowMethods>(
        window: Option<&W>,
        style: c_int,
    ) -> BufferedPaintDCIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            BufferedPaintDCIsOwned(ffi::wxBufferedPaintDC_new1(window, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<BufferedPaintDCIsOwned<OWNED>> for BufferedDCIsOwned<OWNED> {
    fn from(o: BufferedPaintDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BufferedPaintDCIsOwned<OWNED>> for MemoryDCIsOwned<OWNED> {
    fn from(o: BufferedPaintDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BufferedPaintDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: BufferedPaintDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BufferedPaintDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BufferedPaintDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BufferedPaintDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBufferedPaintDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BufferedPaintDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBusyCursor
wx_class! { BusyCursor =
    BusyCursorIsOwned<true>(wxBusyCursor) impl
        BusyCursorMethods
}
impl<const OWNED: bool> BusyCursorIsOwned<OWNED> {
    pub fn new<C: CursorMethods>(cursor: Option<&C>) -> BusyCursorIsOwned<OWNED> {
        unsafe {
            let cursor = match cursor {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            BusyCursorIsOwned(ffi::wxBusyCursor_new(cursor))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for BusyCursorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxBusyCursor_delete(self.0) }
        }
    }
}

// wxBusyInfo
wx_class! { BusyInfo =
    BusyInfoIsOwned<true>(wxBusyInfo) impl
        BusyInfoMethods
}
impl<const OWNED: bool> BusyInfoIsOwned<OWNED> {
    pub fn new_with_busyinfoflags<B: BusyInfoFlagsMethods>(flags: &B) -> BusyInfoIsOwned<OWNED> {
        unsafe {
            let flags = flags.as_ptr();
            BusyInfoIsOwned(ffi::wxBusyInfo_new(flags))
        }
    }
    pub fn new_with_str<W: WindowMethods>(msg: &str, parent: Option<&W>) -> BusyInfoIsOwned<OWNED> {
        unsafe {
            let msg = WxString::from(msg);
            let msg = msg.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            BusyInfoIsOwned(ffi::wxBusyInfo_new1(msg, parent))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for BusyInfoIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxBusyInfo_delete(self.0) }
        }
    }
}

// wxButton
wx_class! { Button =
    ButtonIsOwned<true>(wxButton) impl
        ButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ButtonIsOwned<OWNED> {
    pub fn new_2step() -> ButtonIsOwned<OWNED> {
        unsafe { ButtonIsOwned(ffi::wxButton_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ButtonIsOwned<OWNED> {
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
            ButtonIsOwned(ffi::wxButton_new1(
                parent, id, label, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ButtonIsOwned<OWNED>> for AnyButtonIsOwned<OWNED> {
    fn from(o: ButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ButtonIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ButtonIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ButtonIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ButtonIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ButtonIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxButton_CLASSINFO()) }
    }
}
// Mix-in(s) to wxButton
impl<const OWNED: bool> TrackableMethods for ButtonIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxButton_AsTrackable(self.as_ptr()) }
    }
}

// wxCalculateLayoutEvent
wx_class! { CalculateLayoutEvent =
    CalculateLayoutEventIsOwned<true>(wxCalculateLayoutEvent) impl
        CalculateLayoutEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> CalculateLayoutEventIsOwned<OWNED> {
    pub fn new(id: c_int) -> CalculateLayoutEventIsOwned<OWNED> {
        unsafe { CalculateLayoutEventIsOwned(ffi::wxCalculateLayoutEvent_new(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<CalculateLayoutEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: CalculateLayoutEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalculateLayoutEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CalculateLayoutEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CalculateLayoutEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCalculateLayoutEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CalculateLayoutEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCalendarCtrl
wx_class! { CalendarCtrl =
    CalendarCtrlIsOwned<true>(wxCalendarCtrl) impl
        CalendarCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CalendarCtrlIsOwned<OWNED> {
    pub fn new_2step() -> CalendarCtrlIsOwned<OWNED> {
        unsafe { CalendarCtrlIsOwned(ffi::wxCalendarCtrl_new()) }
    }
    pub fn new<W: WindowMethods, D: DateTimeMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        date: &D,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> CalendarCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let date = date.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            CalendarCtrlIsOwned(ffi::wxCalendarCtrl_new1(
                parent, id, date, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<CalendarCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: CalendarCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: CalendarCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: CalendarCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CalendarCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CalendarCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCalendarCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxCalendarCtrl
impl<const OWNED: bool> TrackableMethods for CalendarCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxCalendarCtrl_AsTrackable(self.as_ptr()) }
    }
}

// wxCalendarDateAttr
wx_class! { CalendarDateAttr =
    CalendarDateAttrIsOwned<true>(wxCalendarDateAttr) impl
        CalendarDateAttrMethods
}
impl<const OWNED: bool> CalendarDateAttrIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxCalendarDateAttr()
    // NOT_SUPPORTED: fn wxCalendarDateAttr1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for CalendarDateAttrIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxCalendarDateAttr_delete(self.0) }
        }
    }
}

// wxCalendarEvent
wx_class! { CalendarEvent =
    CalendarEventIsOwned<true>(wxCalendarEvent) impl
        CalendarEventMethods,
        DateEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> CalendarEventIsOwned<OWNED> {
    pub fn new() -> CalendarEventIsOwned<OWNED> {
        unsafe { CalendarEventIsOwned(ffi::wxCalendarEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxCalendarEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<CalendarEventIsOwned<OWNED>> for DateEventIsOwned<OWNED> {
    fn from(o: CalendarEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: CalendarEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: CalendarEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CalendarEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CalendarEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCalendarEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CalendarEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCaret
wx_class! { Caret =
    CaretIsOwned<true>(wxCaret) impl
        CaretMethods
}
impl<const OWNED: bool> CaretIsOwned<OWNED> {
    pub fn new() -> CaretIsOwned<OWNED> {
        unsafe { CaretIsOwned(ffi::wxCaret_new()) }
    }
    pub fn new_with_window_int<W: WindowMethods>(
        window: Option<&W>,
        width: c_int,
        height: c_int,
    ) -> CaretIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            CaretIsOwned(ffi::wxCaret_new1(window, width, height))
        }
    }
    pub fn new_with_window_size<W: WindowMethods, S: SizeMethods>(
        window: Option<&W>,
        size: &S,
    ) -> CaretIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let size = size.as_ptr();
            CaretIsOwned(ffi::wxCaret_new2(window, size))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for CaretIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxCaret_delete(self.0) }
        }
    }
}

// wxCheckBox
wx_class! { CheckBox =
    CheckBoxIsOwned<true>(wxCheckBox) impl
        CheckBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CheckBoxIsOwned<OWNED> {
    pub fn new_2step() -> CheckBoxIsOwned<OWNED> {
        unsafe { CheckBoxIsOwned(ffi::wxCheckBox_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> CheckBoxIsOwned<OWNED> {
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
            CheckBoxIsOwned(ffi::wxCheckBox_new1(
                parent, id, label, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<CheckBoxIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: CheckBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckBoxIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: CheckBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckBoxIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: CheckBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckBoxIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CheckBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CheckBoxIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCheckBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxCheckBox
impl<const OWNED: bool> TrackableMethods for CheckBoxIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxCheckBox_AsTrackable(self.as_ptr()) }
    }
}

// wxCheckListBox
wx_class! { CheckListBox =
    CheckListBoxIsOwned<true>(wxCheckListBox) impl
        CheckListBoxMethods,
        // ListBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CheckListBoxIsOwned<OWNED> {
    pub fn new_2step() -> CheckListBoxIsOwned<OWNED> {
        unsafe { CheckListBoxIsOwned(ffi::wxCheckListBox_new()) }
    }
    // NOT_SUPPORTED: fn wxCheckListBox1()
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> CheckListBoxIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            CheckListBoxIsOwned(ffi::wxCheckListBox_new2(
                parent, id, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<CheckListBoxIsOwned<OWNED>> for ListBoxIsOwned<OWNED> {
    fn from(o: CheckListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckListBoxIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: CheckListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckListBoxIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: CheckListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckListBoxIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: CheckListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckListBoxIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CheckListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CheckListBoxIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCheckListBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxCheckListBox
impl<const OWNED: bool> ItemContainerMethods for CheckListBoxIsOwned<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxCheckListBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for CheckListBoxIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxCheckListBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for CheckListBoxIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxCheckListBox_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ListBoxMethods for CheckListBoxIsOwned<OWNED> {
    // NOT_SUPPORTED: fn Create()
    fn create_arraystring<
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
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxCheckListBox_Create1(
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
}

// wxChildFocusEvent
wx_class! { ChildFocusEvent =
    ChildFocusEventIsOwned<true>(wxChildFocusEvent) impl
        ChildFocusEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ChildFocusEventIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(win: Option<&W>) -> ChildFocusEventIsOwned<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ChildFocusEventIsOwned(ffi::wxChildFocusEvent_new(win))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ChildFocusEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: ChildFocusEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChildFocusEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ChildFocusEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChildFocusEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ChildFocusEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ChildFocusEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxChildFocusEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ChildFocusEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxChoice
wx_class! { Choice =
    ChoiceIsOwned<true>(wxChoice) impl
        ChoiceMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ChoiceIsOwned<OWNED> {
    pub fn new_2step() -> ChoiceIsOwned<OWNED> {
        unsafe { ChoiceIsOwned(ffi::wxChoice_new()) }
    }
    // NOT_SUPPORTED: fn wxChoice1()
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ChoiceIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ChoiceIsOwned(ffi::wxChoice_new2(
                parent, id, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ChoiceIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ChoiceIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoiceIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ChoiceIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoiceIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ChoiceIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoiceIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ChoiceIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ChoiceIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxChoice_CLASSINFO()) }
    }
}
// Mix-in(s) to wxChoice
impl<const OWNED: bool> ItemContainerMethods for ChoiceIsOwned<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxChoice_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for ChoiceIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxChoice_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for ChoiceIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxChoice_AsTrackable(self.as_ptr()) }
    }
}

// wxChoicebook
wx_class! { Choicebook =
    ChoicebookIsOwned<true>(wxChoicebook) impl
        ChoicebookMethods,
        // BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ChoicebookIsOwned<OWNED> {
    pub fn new_2step() -> ChoicebookIsOwned<OWNED> {
        unsafe { ChoicebookIsOwned(ffi::wxChoicebook_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> ChoicebookIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ChoicebookIsOwned(ffi::wxChoicebook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ChoicebookIsOwned<OWNED>> for BookCtrlBaseIsOwned<OWNED> {
    fn from(o: ChoicebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoicebookIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ChoicebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoicebookIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ChoicebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoicebookIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ChoicebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoicebookIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ChoicebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ChoicebookIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxChoicebook_CLASSINFO()) }
    }
}
// Mix-in(s) to wxChoicebook
impl<const OWNED: bool> WithImagesMethods for ChoicebookIsOwned<OWNED> {
    fn as_with_images(&self) -> *mut c_void {
        unsafe { ffi::wxChoicebook_AsWithImages(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for ChoicebookIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxChoicebook_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> BookCtrlBaseMethods for ChoicebookIsOwned<OWNED> {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
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
            ffi::wxChoicebook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}
impl<const OWNED: bool> WindowMethods for ChoicebookIsOwned<OWNED> {
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
            ffi::wxChoicebook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxClientDC
wx_class! { ClientDC =
    ClientDCIsOwned<true>(wxClientDC) impl
        ClientDCMethods,
        WindowDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> ClientDCIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(window: Option<&W>) -> ClientDCIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ClientDCIsOwned(ffi::wxClientDC_new(window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ClientDCIsOwned<OWNED>> for WindowDCIsOwned<OWNED> {
    fn from(o: ClientDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ClientDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: ClientDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ClientDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ClientDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ClientDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxClientDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ClientDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxClipboard
wx_class! { Clipboard =
    ClipboardIsOwned<true>(wxClipboard) impl
        ClipboardMethods,
        ObjectMethods
}
impl<const OWNED: bool> ClipboardIsOwned<OWNED> {
    pub fn new() -> ClipboardIsOwned<OWNED> {
        unsafe { ClipboardIsOwned(ffi::wxClipboard_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ClipboardIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ClipboardIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ClipboardIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxClipboard_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ClipboardIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxClipboardTextEvent
wx_class! { ClipboardTextEvent =
    ClipboardTextEventIsOwned<true>(wxClipboardTextEvent) impl
        ClipboardTextEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ClipboardTextEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxClipboardTextEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ClipboardTextEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: ClipboardTextEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ClipboardTextEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ClipboardTextEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ClipboardTextEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ClipboardTextEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ClipboardTextEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxClipboardTextEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ClipboardTextEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCloseEvent
wx_class! { CloseEvent =
    CloseEventIsOwned<true>(wxCloseEvent) impl
        CloseEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> CloseEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxCloseEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<CloseEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: CloseEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CloseEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CloseEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CloseEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCloseEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CloseEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCollapsibleHeaderCtrl
wx_class! { CollapsibleHeaderCtrl =
    CollapsibleHeaderCtrlIsOwned<true>(wxCollapsibleHeaderCtrl) impl
        CollapsibleHeaderCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CollapsibleHeaderCtrlIsOwned<OWNED> {
    pub fn new_2step() -> CollapsibleHeaderCtrlIsOwned<OWNED> {
        unsafe { CollapsibleHeaderCtrlIsOwned(ffi::wxCollapsibleHeaderCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> CollapsibleHeaderCtrlIsOwned<OWNED> {
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
            CollapsibleHeaderCtrlIsOwned(ffi::wxCollapsibleHeaderCtrl_new1(
                parent, id, label, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<CollapsibleHeaderCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: CollapsibleHeaderCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsibleHeaderCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: CollapsibleHeaderCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsibleHeaderCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: CollapsibleHeaderCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsibleHeaderCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CollapsibleHeaderCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CollapsibleHeaderCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCollapsibleHeaderCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxCollapsibleHeaderCtrl
impl<const OWNED: bool> TrackableMethods for CollapsibleHeaderCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxCollapsibleHeaderCtrl_AsTrackable(self.as_ptr()) }
    }
}

// wxCollapsiblePane
wx_class! { CollapsiblePane =
    CollapsiblePaneIsOwned<true>(wxCollapsiblePane) impl
        CollapsiblePaneMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CollapsiblePaneIsOwned<OWNED> {
    pub fn new_2step() -> CollapsiblePaneIsOwned<OWNED> {
        unsafe { CollapsiblePaneIsOwned(ffi::wxCollapsiblePane_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> CollapsiblePaneIsOwned<OWNED> {
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
            CollapsiblePaneIsOwned(ffi::wxCollapsiblePane_new1(
                parent, id, label, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<CollapsiblePaneIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: CollapsiblePaneIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsiblePaneIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: CollapsiblePaneIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsiblePaneIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: CollapsiblePaneIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsiblePaneIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CollapsiblePaneIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CollapsiblePaneIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCollapsiblePane_CLASSINFO()) }
    }
}
// Mix-in(s) to wxCollapsiblePane
impl<const OWNED: bool> TrackableMethods for CollapsiblePaneIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxCollapsiblePane_AsTrackable(self.as_ptr()) }
    }
}

// wxCollapsiblePaneEvent
wx_class! { CollapsiblePaneEvent =
    CollapsiblePaneEventIsOwned<true>(wxCollapsiblePaneEvent) impl
        CollapsiblePaneEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> CollapsiblePaneEventIsOwned<OWNED> {
    pub fn new<O: ObjectMethods>(
        generator: Option<&O>,
        id: c_int,
        collapsed: bool,
    ) -> CollapsiblePaneEventIsOwned<OWNED> {
        unsafe {
            let generator = match generator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            CollapsiblePaneEventIsOwned(ffi::wxCollapsiblePaneEvent_new(generator, id, collapsed))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<CollapsiblePaneEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: CollapsiblePaneEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsiblePaneEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: CollapsiblePaneEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsiblePaneEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CollapsiblePaneEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CollapsiblePaneEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCollapsiblePaneEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CollapsiblePaneEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxColour
wx_class! { Colour =
    ColourIsOwned<true>(wxColour) impl
        ColourMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourIsOwned<OWNED> {
    pub fn new() -> ColourIsOwned<OWNED> {
        unsafe { ColourIsOwned(ffi::wxColour_new()) }
    }
    // NOT_SUPPORTED: fn wxColour1()
    pub fn new_with_str(colour_name: &str) -> ColourIsOwned<OWNED> {
        unsafe {
            let colour_name = WxString::from(colour_name);
            let colour_name = colour_name.as_ptr();
            ColourIsOwned(ffi::wxColour_new2(colour_name))
        }
    }
    // NOT_SUPPORTED: fn wxColour3()
    pub fn new_with_colour<C: ColourMethods>(colour: &C) -> ColourIsOwned<OWNED> {
        unsafe {
            let colour = colour.as_ptr();
            ColourIsOwned(ffi::wxColour_new4(colour))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ColourIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ColourIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ColourIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxColour_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ColourIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxColourData
wx_class! { ColourData =
    ColourDataIsOwned<true>(wxColourData) impl
        ColourDataMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourDataIsOwned<OWNED> {
    //  ENUM: @6
    pub const NUM_CUSTOM: c_int = 16;

    pub fn new() -> ColourDataIsOwned<OWNED> {
        unsafe { ColourDataIsOwned(ffi::wxColourData_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ColourDataIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ColourDataIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ColourDataIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxColourData_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ColourDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxColourDatabase
wx_class! { ColourDatabase =
    ColourDatabaseIsOwned<true>(wxColourDatabase) impl
        ColourDatabaseMethods
}
impl<const OWNED: bool> ColourDatabaseIsOwned<OWNED> {
    pub fn new() -> ColourDatabaseIsOwned<OWNED> {
        unsafe { ColourDatabaseIsOwned(ffi::wxColourDatabase_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ColourDatabaseIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxColourDatabase_delete(self.0) }
        }
    }
}

// wxColourDialog
wx_class! { ColourDialog =
    ColourDialogIsOwned<true>(wxColourDialog) impl
        ColourDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourDialogIsOwned<OWNED> {
    pub fn new<W: WindowMethods, C: ColourDataMethods>(
        parent: Option<&W>,
        data: Option<&C>,
    ) -> ColourDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ColourDialogIsOwned(ffi::wxColourDialog_new(parent, data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ColourDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: ColourDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: ColourDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: ColourDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ColourDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ColourDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ColourDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ColourDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxColourDialog_CLASSINFO()) }
    }
}
// Mix-in(s) to wxColourDialog
impl<const OWNED: bool> TrackableMethods for ColourDialogIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxColourDialog_AsTrackable(self.as_ptr()) }
    }
}

// wxColourDialogEvent
wx_class! { ColourDialogEvent =
    ColourDialogEventIsOwned<true>(wxColourDialogEvent) impl
        ColourDialogEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourDialogEventIsOwned<OWNED> {
    pub fn new() -> ColourDialogEventIsOwned<OWNED> {
        unsafe { ColourDialogEventIsOwned(ffi::wxColourDialogEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxColourDialogEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ColourDialogEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: ColourDialogEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourDialogEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ColourDialogEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourDialogEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ColourDialogEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ColourDialogEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxColourDialogEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ColourDialogEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxColourPickerCtrl
wx_class! { ColourPickerCtrl =
    ColourPickerCtrlIsOwned<true>(wxColourPickerCtrl) impl
        ColourPickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourPickerCtrlIsOwned<OWNED> {
    pub fn new_2step() -> ColourPickerCtrlIsOwned<OWNED> {
        unsafe { ColourPickerCtrlIsOwned(ffi::wxColourPickerCtrl_new()) }
    }
    pub fn new<
        W: WindowMethods,
        C: ColourMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        colour: &C,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ColourPickerCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let colour = colour.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ColourPickerCtrlIsOwned(ffi::wxColourPickerCtrl_new1(
                parent, id, colour, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ColourPickerCtrlIsOwned<OWNED>> for PickerBaseIsOwned<OWNED> {
    fn from(o: ColourPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ColourPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ColourPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ColourPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ColourPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ColourPickerCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxColourPickerCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxColourPickerCtrl
impl<const OWNED: bool> TrackableMethods for ColourPickerCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxColourPickerCtrl_AsTrackable(self.as_ptr()) }
    }
}

// wxColourPickerEvent
wx_class! { ColourPickerEvent =
    ColourPickerEventIsOwned<true>(wxColourPickerEvent) impl
        ColourPickerEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourPickerEventIsOwned<OWNED> {
    pub fn new() -> ColourPickerEventIsOwned<OWNED> {
        unsafe { ColourPickerEventIsOwned(ffi::wxColourPickerEvent_new()) }
    }
    pub fn new_with_object<O: ObjectMethods, C: ColourMethods>(
        generator: Option<&O>,
        id: c_int,
        colour: &C,
    ) -> ColourPickerEventIsOwned<OWNED> {
        unsafe {
            let generator = match generator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let colour = colour.as_ptr();
            ColourPickerEventIsOwned(ffi::wxColourPickerEvent_new1(generator, id, colour))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ColourPickerEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: ColourPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ColourPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ColourPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ColourPickerEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxColourPickerEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ColourPickerEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxComboBox
wx_class! { ComboBox =
    ComboBoxIsOwned<true>(wxComboBox) impl
        ComboBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ComboBoxIsOwned<OWNED> {
    pub fn new_2step() -> ComboBoxIsOwned<OWNED> {
        unsafe { ComboBoxIsOwned(ffi::wxComboBox_new()) }
    }
    // NOT_SUPPORTED: fn wxComboBox1()
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ComboBoxIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ComboBoxIsOwned(ffi::wxComboBox_new2(
                parent, id, value, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ComboBoxIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboBoxIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboBoxIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboBoxIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ComboBoxIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxComboBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxComboBox
impl<const OWNED: bool> ItemContainerMethods for ComboBoxIsOwned<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for ComboBoxIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TextEntryMethods for ComboBoxIsOwned<OWNED> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxComboBox_AsTextEntry(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for ComboBoxIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxComboBox_AsTrackable(self.as_ptr()) }
    }
}

// wxComboCtrl
wx_class! { ComboCtrl =
    ComboCtrlIsOwned<true>(wxComboCtrl) impl
        ComboCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ComboCtrlIsOwned<OWNED> {
    //  ENUM: @9
    pub const ShowBelow: c_int = 0x0000;
    pub const ShowAbove: c_int = 0x0001;
    pub const CanDeferShow: c_int = 0x0002;

    pub fn new_2step() -> ComboCtrlIsOwned<OWNED> {
        unsafe { ComboCtrlIsOwned(ffi::wxComboCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ComboCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ComboCtrlIsOwned(ffi::wxComboCtrl_new1(
                parent, id, value, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ComboCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ComboCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ComboCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ComboCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ComboCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ComboCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxComboCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxComboCtrl
impl<const OWNED: bool> TextEntryMethods for ComboCtrlIsOwned<OWNED> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxComboCtrl_AsTextEntry(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for ComboCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxComboCtrl_AsTrackable(self.as_ptr()) }
    }
}

// wxComboPopup
wx_class! { ComboPopup =
    ComboPopupIsOwned<true>(wxComboPopup) impl
        ComboPopupMethods
}
impl<const OWNED: bool> ComboPopupIsOwned<OWNED> {
    pub fn new() -> ComboPopupIsOwned<OWNED> {
        unsafe { ComboPopupIsOwned(ffi::wxComboPopup_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ComboPopupIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxComboPopup_delete(self.0) }
        }
    }
}

// wxCommand
wx_class! { Command =
    CommandIsOwned<true>(wxCommand) impl
        CommandMethods,
        ObjectMethods
}
impl<const OWNED: bool> CommandIsOwned<OWNED> {
    pub fn new(can_undo: bool, name: &str) -> CommandIsOwned<OWNED> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            CommandIsOwned(ffi::wxCommand_new(can_undo, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<CommandIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CommandIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CommandIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCommand_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CommandIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCommandEvent
wx_class! { CommandEvent =
    CommandEventIsOwned<true>(wxCommandEvent) impl
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> CommandEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxCommandEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<CommandEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: CommandEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CommandEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CommandEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCommandEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CommandEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCommandLinkButton
wx_class! { CommandLinkButton =
    CommandLinkButtonIsOwned<true>(wxCommandLinkButton) impl
        CommandLinkButtonMethods,
        ButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CommandLinkButtonIsOwned<OWNED> {
    pub fn new_2step() -> CommandLinkButtonIsOwned<OWNED> {
        unsafe { CommandLinkButtonIsOwned(ffi::wxCommandLinkButton_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        main_label: &str,
        note: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> CommandLinkButtonIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let main_label = WxString::from(main_label);
            let main_label = main_label.as_ptr();
            let note = WxString::from(note);
            let note = note.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            CommandLinkButtonIsOwned(ffi::wxCommandLinkButton_new1(
                parent, id, main_label, note, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<CommandLinkButtonIsOwned<OWNED>> for ButtonIsOwned<OWNED> {
    fn from(o: CommandLinkButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandLinkButtonIsOwned<OWNED>> for AnyButtonIsOwned<OWNED> {
    fn from(o: CommandLinkButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandLinkButtonIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: CommandLinkButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandLinkButtonIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: CommandLinkButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandLinkButtonIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: CommandLinkButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandLinkButtonIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CommandLinkButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CommandLinkButtonIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCommandLinkButton_CLASSINFO()) }
    }
}
// Mix-in(s) to wxCommandLinkButton
impl<const OWNED: bool> TrackableMethods for CommandLinkButtonIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxCommandLinkButton_AsTrackable(self.as_ptr()) }
    }
}

// wxCommandProcessor
wx_class! { CommandProcessor =
    CommandProcessorIsOwned<true>(wxCommandProcessor) impl
        CommandProcessorMethods,
        ObjectMethods
}
impl<const OWNED: bool> CommandProcessorIsOwned<OWNED> {
    pub fn new(max_commands: c_int) -> CommandProcessorIsOwned<OWNED> {
        unsafe { CommandProcessorIsOwned(ffi::wxCommandProcessor_new(max_commands)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<CommandProcessorIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CommandProcessorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CommandProcessorIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCommandProcessor_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CommandProcessorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxContextHelp
wx_class! { ContextHelp =
    ContextHelpIsOwned<true>(wxContextHelp) impl
        ContextHelpMethods,
        ObjectMethods
}
impl<const OWNED: bool> ContextHelpIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(window: Option<&W>, do_now: bool) -> ContextHelpIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ContextHelpIsOwned(ffi::wxContextHelp_new(window, do_now))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ContextHelpIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ContextHelpIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ContextHelpIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxContextHelp_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ContextHelpIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxContextHelpButton
wx_class! { ContextHelpButton =
    ContextHelpButtonIsOwned<true>(wxContextHelpButton) impl
        ContextHelpButtonMethods,
        BitmapButtonMethods,
        ButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ContextHelpButtonIsOwned<OWNED> {
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
    ) -> ContextHelpButtonIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            ContextHelpButtonIsOwned(ffi::wxContextHelpButton_new(parent, id, pos, size, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ContextHelpButtonIsOwned<OWNED>> for BitmapButtonIsOwned<OWNED> {
    fn from(o: ContextHelpButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ContextHelpButtonIsOwned<OWNED>> for ButtonIsOwned<OWNED> {
    fn from(o: ContextHelpButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ContextHelpButtonIsOwned<OWNED>> for AnyButtonIsOwned<OWNED> {
    fn from(o: ContextHelpButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ContextHelpButtonIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ContextHelpButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ContextHelpButtonIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ContextHelpButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ContextHelpButtonIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ContextHelpButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ContextHelpButtonIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ContextHelpButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ContextHelpButtonIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxContextHelpButton_CLASSINFO()) }
    }
}
// Mix-in(s) to wxContextHelpButton
impl<const OWNED: bool> TrackableMethods for ContextHelpButtonIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxContextHelpButton_AsTrackable(self.as_ptr()) }
    }
}

// wxContextMenuEvent
wx_class! { ContextMenuEvent =
    ContextMenuEventIsOwned<true>(wxContextMenuEvent) impl
        ContextMenuEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ContextMenuEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxContextMenuEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ContextMenuEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: ContextMenuEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ContextMenuEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ContextMenuEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ContextMenuEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ContextMenuEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ContextMenuEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxContextMenuEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ContextMenuEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxControl
wx_class! { Control =
    ControlIsOwned<true>(wxControl) impl
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ControlIsOwned<OWNED> {
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ControlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ControlIsOwned(ffi::wxControl_new(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    pub fn new_2step() -> ControlIsOwned<OWNED> {
        unsafe { ControlIsOwned(ffi::wxControl_new1()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ControlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ControlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ControlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ControlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ControlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ControlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ControlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxControl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxControl
impl<const OWNED: bool> TrackableMethods for ControlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxControl_AsTrackable(self.as_ptr()) }
    }
}

// wxControlWithItems
wx_class! { ControlWithItems =
    ControlWithItemsIsOwned<true>(wxControlWithItems) impl
        ControlWithItemsMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ControlWithItemsIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ControlWithItemsIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ControlWithItemsIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ControlWithItemsIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ControlWithItemsIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ControlWithItemsIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ControlWithItemsIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ControlWithItemsIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ControlWithItemsIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ControlWithItemsIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxControlWithItems_CLASSINFO()) }
    }
}
// Mix-in(s) to wxControlWithItems
impl<const OWNED: bool> ItemContainerMethods for ControlWithItemsIsOwned<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxControlWithItems_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for ControlWithItemsIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxControlWithItems_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for ControlWithItemsIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxControlWithItems_AsTrackable(self.as_ptr()) }
    }
}

// wxCredentialEntryDialog
wx_class! { CredentialEntryDialog =
    CredentialEntryDialogIsOwned<true>(wxCredentialEntryDialog) impl
        CredentialEntryDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CredentialEntryDialogIsOwned<OWNED> {
    pub fn new_2step() -> CredentialEntryDialogIsOwned<OWNED> {
        unsafe { CredentialEntryDialogIsOwned(ffi::wxCredentialEntryDialog_new()) }
    }
    pub fn new<W: WindowMethods, W2: WebCredentialsMethods>(
        parent: Option<&W>,
        message: &str,
        title: &str,
        cred: &W2,
    ) -> CredentialEntryDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let title = WxString::from(title);
            let title = title.as_ptr();
            let cred = cred.as_ptr();
            CredentialEntryDialogIsOwned(ffi::wxCredentialEntryDialog_new1(
                parent, message, title, cred,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<CredentialEntryDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: CredentialEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CredentialEntryDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: CredentialEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CredentialEntryDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: CredentialEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CredentialEntryDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: CredentialEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CredentialEntryDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: CredentialEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CredentialEntryDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CredentialEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CredentialEntryDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCredentialEntryDialog_CLASSINFO()) }
    }
}
// Mix-in(s) to wxCredentialEntryDialog
impl<const OWNED: bool> TrackableMethods for CredentialEntryDialogIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxCredentialEntryDialog_AsTrackable(self.as_ptr()) }
    }
}

// wxCursor
wx_class! { Cursor =
    CursorIsOwned<true>(wxCursor) impl
        CursorMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> CursorIsOwned<OWNED> {
    pub fn new() -> CursorIsOwned<OWNED> {
        unsafe { CursorIsOwned(ffi::wxCursor_new()) }
    }
    // NOT_SUPPORTED: fn wxCursor1()
    // NOT_SUPPORTED: fn wxCursor2()
    // NOT_SUPPORTED: fn wxCursor3()
    pub fn new_with_image<I: ImageMethods>(image: &I) -> CursorIsOwned<OWNED> {
        unsafe {
            let image = image.as_ptr();
            CursorIsOwned(ffi::wxCursor_new4(image))
        }
    }
    pub fn new_with_char(xpm_data: *const c_void) -> CursorIsOwned<OWNED> {
        unsafe { CursorIsOwned(ffi::wxCursor_new5(xpm_data)) }
    }
    pub fn new_with_cursor<C: CursorMethods>(cursor: &C) -> CursorIsOwned<OWNED> {
        unsafe {
            let cursor = cursor.as_ptr();
            CursorIsOwned(ffi::wxCursor_new6(cursor))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<CursorIsOwned<OWNED>> for GDIObjectIsOwned<OWNED> {
    fn from(o: CursorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CursorIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CursorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CursorIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCursor_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CursorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCustomDataObject
wx_class! { CustomDataObject =
    CustomDataObjectIsOwned<true>(wxCustomDataObject) impl
        CustomDataObjectMethods,
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> CustomDataObjectIsOwned<OWNED> {
    pub fn new(format: *const c_void) -> CustomDataObjectIsOwned<OWNED> {
        unsafe { CustomDataObjectIsOwned(ffi::wxCustomDataObject_new(format)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<CustomDataObjectIsOwned<OWNED>> for DataObjectSimpleIsOwned<OWNED> {
    fn from(o: CustomDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CustomDataObjectIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: CustomDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for CustomDataObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxCustomDataObject_delete(self.0) }
        }
    }
}

// wxDC
wx_class! { DC =
    DCIsOwned<true>(wxDC) impl
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> DCIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDCBrushChanger
wx_class! { DCBrushChanger =
    DCBrushChangerIsOwned<true>(wxDCBrushChanger) impl
        DCBrushChangerMethods
}
impl<const OWNED: bool> DCBrushChangerIsOwned<OWNED> {
    pub fn new<D: DCMethods, B: BrushMethods>(dc: &D, brush: &B) -> DCBrushChangerIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let brush = brush.as_ptr();
            DCBrushChangerIsOwned(ffi::wxDCBrushChanger_new(dc, brush))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DCBrushChangerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCBrushChanger_delete(self.0) }
        }
    }
}

// wxDCClipper
wx_class! { DCClipper =
    DCClipperIsOwned<true>(wxDCClipper) impl
        DCClipperMethods
}
impl<const OWNED: bool> DCClipperIsOwned<OWNED> {
    pub fn new_with_region<D: DCMethods, R: RegionMethods>(
        dc: &D,
        region: &R,
    ) -> DCClipperIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let region = region.as_ptr();
            DCClipperIsOwned(ffi::wxDCClipper_new(dc, region))
        }
    }
    pub fn new_with_rect<D: DCMethods, R: RectMethods>(
        dc: &D,
        rect: &R,
    ) -> DCClipperIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            DCClipperIsOwned(ffi::wxDCClipper_new1(dc, rect))
        }
    }
    pub fn new_with_coord<D: DCMethods>(
        dc: &D,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
    ) -> DCClipperIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            DCClipperIsOwned(ffi::wxDCClipper_new2(dc, x, y, w, h))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DCClipperIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCClipper_delete(self.0) }
        }
    }
}

// wxDCFontChanger
wx_class! { DCFontChanger =
    DCFontChangerIsOwned<true>(wxDCFontChanger) impl
        DCFontChangerMethods
}
impl<const OWNED: bool> DCFontChangerIsOwned<OWNED> {
    pub fn new<D: DCMethods>(dc: &D) -> DCFontChangerIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            DCFontChangerIsOwned(ffi::wxDCFontChanger_new(dc))
        }
    }
    pub fn new_with_font<D: DCMethods, F: FontMethods>(
        dc: &D,
        font: &F,
    ) -> DCFontChangerIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let font = font.as_ptr();
            DCFontChangerIsOwned(ffi::wxDCFontChanger_new1(dc, font))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DCFontChangerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCFontChanger_delete(self.0) }
        }
    }
}

// wxDCOverlay
wx_class! { DCOverlay =
    DCOverlayIsOwned<true>(wxDCOverlay) impl
        DCOverlayMethods
}
impl<const OWNED: bool> DCOverlayIsOwned<OWNED> {
    pub fn new_with_int<O: OverlayMethods, D: DCMethods>(
        overlay: &O,
        dc: Option<&D>,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    ) -> DCOverlayIsOwned<OWNED> {
        unsafe {
            let overlay = overlay.as_ptr();
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DCOverlayIsOwned(ffi::wxDCOverlay_new(overlay, dc, x, y, width, height))
        }
    }
    pub fn new<O: OverlayMethods, D: DCMethods>(
        overlay: &O,
        dc: Option<&D>,
    ) -> DCOverlayIsOwned<OWNED> {
        unsafe {
            let overlay = overlay.as_ptr();
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DCOverlayIsOwned(ffi::wxDCOverlay_new1(overlay, dc))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DCOverlayIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCOverlay_delete(self.0) }
        }
    }
}

// wxDCPenChanger
wx_class! { DCPenChanger =
    DCPenChangerIsOwned<true>(wxDCPenChanger) impl
        DCPenChangerMethods
}
impl<const OWNED: bool> DCPenChangerIsOwned<OWNED> {
    pub fn new<D: DCMethods, P: PenMethods>(dc: &D, pen: &P) -> DCPenChangerIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let pen = pen.as_ptr();
            DCPenChangerIsOwned(ffi::wxDCPenChanger_new(dc, pen))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DCPenChangerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCPenChanger_delete(self.0) }
        }
    }
}

// wxDCTextBgColourChanger
wx_class! { DCTextBgColourChanger =
    DCTextBgColourChangerIsOwned<true>(wxDCTextBgColourChanger) impl
        DCTextBgColourChangerMethods
}
impl<const OWNED: bool> DCTextBgColourChangerIsOwned<OWNED> {
    pub fn new<D: DCMethods>(dc: &D) -> DCTextBgColourChangerIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            DCTextBgColourChangerIsOwned(ffi::wxDCTextBgColourChanger_new(dc))
        }
    }
    pub fn new_with_colour<D: DCMethods, C: ColourMethods>(
        dc: &D,
        col: &C,
    ) -> DCTextBgColourChangerIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let col = col.as_ptr();
            DCTextBgColourChangerIsOwned(ffi::wxDCTextBgColourChanger_new1(dc, col))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DCTextBgColourChangerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCTextBgColourChanger_delete(self.0) }
        }
    }
}

// wxDCTextBgModeChanger
wx_class! { DCTextBgModeChanger =
    DCTextBgModeChangerIsOwned<true>(wxDCTextBgModeChanger) impl
        DCTextBgModeChangerMethods
}
impl<const OWNED: bool> DCTextBgModeChangerIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DCTextBgModeChangerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCTextBgModeChanger_delete(self.0) }
        }
    }
}

// wxDCTextColourChanger
wx_class! { DCTextColourChanger =
    DCTextColourChangerIsOwned<true>(wxDCTextColourChanger) impl
        DCTextColourChangerMethods
}
impl<const OWNED: bool> DCTextColourChangerIsOwned<OWNED> {
    pub fn new<D: DCMethods>(dc: &D) -> DCTextColourChangerIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            DCTextColourChangerIsOwned(ffi::wxDCTextColourChanger_new(dc))
        }
    }
    pub fn new_with_colour<D: DCMethods, C: ColourMethods>(
        dc: &D,
        col: &C,
    ) -> DCTextColourChangerIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let col = col.as_ptr();
            DCTextColourChangerIsOwned(ffi::wxDCTextColourChanger_new1(dc, col))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DCTextColourChangerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCTextColourChanger_delete(self.0) }
        }
    }
}

// wxDPIChangedEvent
wx_class! { DPIChangedEvent =
    DPIChangedEventIsOwned<true>(wxDPIChangedEvent) impl
        DPIChangedEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DPIChangedEventIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DPIChangedEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: DPIChangedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DPIChangedEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DPIChangedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DPIChangedEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDPIChangedEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DPIChangedEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataObject
wx_class! { DataObject =
    DataObjectIsOwned<true>(wxDataObject) impl
        DataObjectMethods
}
impl<const OWNED: bool> DataObjectIsOwned<OWNED> {
    //  ENUM: Direction
    pub const Get: c_int = 0x01;
    pub const Set: c_int = 0x02;
    pub const Both: c_int = 0x03;

    pub fn new() -> DataObjectIsOwned<OWNED> {
        unsafe { DataObjectIsOwned(ffi::wxDataObject_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DataObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataObject_delete(self.0) }
        }
    }
}

// wxDataObjectComposite
wx_class! { DataObjectComposite =
    DataObjectCompositeIsOwned<true>(wxDataObjectComposite) impl
        DataObjectCompositeMethods,
        DataObjectMethods
}
impl<const OWNED: bool> DataObjectCompositeIsOwned<OWNED> {
    pub fn new() -> DataObjectCompositeIsOwned<OWNED> {
        unsafe { DataObjectCompositeIsOwned(ffi::wxDataObjectComposite_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataObjectCompositeIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: DataObjectCompositeIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataObjectCompositeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataObjectComposite_delete(self.0) }
        }
    }
}

// wxDataObjectSimple
wx_class! { DataObjectSimple =
    DataObjectSimpleIsOwned<true>(wxDataObjectSimple) impl
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> DataObjectSimpleIsOwned<OWNED> {
    pub fn new(format: *const c_void) -> DataObjectSimpleIsOwned<OWNED> {
        unsafe { DataObjectSimpleIsOwned(ffi::wxDataObjectSimple_new(format)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataObjectSimpleIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: DataObjectSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataObjectSimpleIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataObjectSimple_delete(self.0) }
        }
    }
}

// wxDataViewBitmapRenderer
wx_class! { DataViewBitmapRenderer =
    DataViewBitmapRendererIsOwned<true>(wxDataViewBitmapRenderer) impl
        DataViewBitmapRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewBitmapRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewBitmapRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewBitmapRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewBitmapRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewBitmapRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewBitmapRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewBitmapRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewBitmapRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewBitmapRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewCheckIconTextRenderer
wx_class! { DataViewCheckIconTextRenderer =
    DataViewCheckIconTextRendererIsOwned<true>(wxDataViewCheckIconTextRenderer) impl
        DataViewCheckIconTextRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewCheckIconTextRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewCheckIconTextRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewCheckIconTextRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewCheckIconTextRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewCheckIconTextRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewCheckIconTextRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewCheckIconTextRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewCheckIconTextRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewCheckIconTextRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewChoiceByIndexRenderer
wx_class! { DataViewChoiceByIndexRenderer =
    DataViewChoiceByIndexRendererIsOwned<true>(wxDataViewChoiceByIndexRenderer) impl
        DataViewChoiceByIndexRendererMethods,
        DataViewChoiceRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewChoiceByIndexRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewChoiceByIndexRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewChoiceByIndexRendererIsOwned<OWNED>>
    for DataViewChoiceRendererIsOwned<OWNED>
{
    fn from(o: DataViewChoiceByIndexRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewChoiceByIndexRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewChoiceByIndexRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewChoiceByIndexRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewChoiceByIndexRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewChoiceByIndexRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewChoiceByIndexRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewChoiceByIndexRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewChoiceRenderer
wx_class! { DataViewChoiceRenderer =
    DataViewChoiceRendererIsOwned<true>(wxDataViewChoiceRenderer) impl
        DataViewChoiceRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewChoiceRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewChoiceRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewChoiceRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewChoiceRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewChoiceRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewChoiceRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewChoiceRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewChoiceRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewChoiceRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewColumn
wx_class! { DataViewColumn =
    DataViewColumnIsOwned<true>(wxDataViewColumn) impl
        DataViewColumnMethods,
        SettableHeaderColumnMethods,
        HeaderColumnMethods
}
impl<const OWNED: bool> DataViewColumnIsOwned<OWNED> {
    pub fn new_with_str<D: DataViewRendererMethods>(
        title: &str,
        renderer: Option<&D>,
        model_column: c_uint,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> DataViewColumnIsOwned<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            let renderer = match renderer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewColumnIsOwned(ffi::wxDataViewColumn_new(
                title,
                renderer,
                model_column,
                width,
                align,
                flags,
            ))
        }
    }
    pub fn new_with_bitmapbundle<B: BitmapBundleMethods, D: DataViewRendererMethods>(
        bitmap: &B,
        renderer: Option<&D>,
        model_column: c_uint,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> DataViewColumnIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            let renderer = match renderer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewColumnIsOwned(ffi::wxDataViewColumn_new1(
                bitmap,
                renderer,
                model_column,
                width,
                align,
                flags,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewColumnIsOwned<OWNED>> for SettableHeaderColumnIsOwned<OWNED> {
    fn from(o: DataViewColumnIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewColumnIsOwned<OWNED>> for HeaderColumnIsOwned<OWNED> {
    fn from(o: DataViewColumnIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewColumnIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewColumn_delete(self.0) }
        }
    }
}

// wxDataViewCtrl
wx_class! { DataViewCtrl =
    DataViewCtrlIsOwned<true>(wxDataViewCtrl) impl
        DataViewCtrlMethods,
        // ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewCtrlIsOwned<OWNED> {
    pub fn new_2step() -> DataViewCtrlIsOwned<OWNED> {
        unsafe { DataViewCtrlIsOwned(ffi::wxDataViewCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> DataViewCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            DataViewCtrlIsOwned(ffi::wxDataViewCtrl_new1(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: DataViewCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DataViewCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DataViewCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxDataViewCtrl
impl<const OWNED: bool> TrackableMethods for DataViewCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxDataViewCtrl_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ControlMethods for DataViewCtrlIsOwned<OWNED> {
    fn create_validator<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
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
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxDataViewCtrl_Create(self.as_ptr(), parent, id, pos, size, style, validator, name)
        }
    }
}

// wxDataViewCustomRenderer
wx_class! { DataViewCustomRenderer =
    DataViewCustomRendererIsOwned<true>(wxDataViewCustomRenderer) impl
        DataViewCustomRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewCustomRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewCustomRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewCustomRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewCustomRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewCustomRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewCustomRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewCustomRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewCustomRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewCustomRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewDateRenderer
wx_class! { DataViewDateRenderer =
    DataViewDateRendererIsOwned<true>(wxDataViewDateRenderer) impl
        DataViewDateRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewDateRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewDateRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewDateRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewDateRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewDateRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewDateRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewDateRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewDateRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewDateRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewEvent
wx_class! { DataViewEvent =
    DataViewEventIsOwned<true>(wxDataViewEvent) impl
        DataViewEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewEventIsOwned<OWNED> {
    pub fn new() -> DataViewEventIsOwned<OWNED> {
        unsafe { DataViewEventIsOwned(ffi::wxDataViewEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxDataViewEvent1()
    // NOT_SUPPORTED: fn wxDataViewEvent2()
    pub fn new_with_dataviewevent<D: DataViewEventMethods>(
        event: &D,
    ) -> DataViewEventIsOwned<OWNED> {
        unsafe {
            let event = event.as_ptr();
            DataViewEventIsOwned(ffi::wxDataViewEvent_new3(event))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: DataViewEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: DataViewEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: DataViewEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewIconText
wx_class! { DataViewIconText =
    DataViewIconTextIsOwned<true>(wxDataViewIconText) impl
        DataViewIconTextMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewIconTextIsOwned<OWNED> {
    pub fn new_with_str<B: BitmapBundleMethods>(
        text: &str,
        bitmap: &B,
    ) -> DataViewIconTextIsOwned<OWNED> {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let bitmap = bitmap.as_ptr();
            DataViewIconTextIsOwned(ffi::wxDataViewIconText_new(text, bitmap))
        }
    }
    pub fn new_with_dataviewicontext<D: DataViewIconTextMethods>(
        other: &D,
    ) -> DataViewIconTextIsOwned<OWNED> {
        unsafe {
            let other = other.as_ptr();
            DataViewIconTextIsOwned(ffi::wxDataViewIconText_new1(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewIconTextIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewIconTextIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewIconTextIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewIconText_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewIconTextIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewIconTextRenderer
wx_class! { DataViewIconTextRenderer =
    DataViewIconTextRendererIsOwned<true>(wxDataViewIconTextRenderer) impl
        DataViewIconTextRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewIconTextRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewIconTextRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewIconTextRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewIconTextRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewIconTextRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewIconTextRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewIconTextRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewIconTextRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewIconTextRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewIndexListModel
wx_class! { DataViewIndexListModel =
    DataViewIndexListModelIsOwned<true>(wxDataViewIndexListModel) impl
        DataViewIndexListModelMethods,
        DataViewListModelMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewIndexListModelIsOwned<OWNED> {
    pub fn new(initial_size: c_uint) -> DataViewIndexListModelIsOwned<OWNED> {
        unsafe { DataViewIndexListModelIsOwned(ffi::wxDataViewIndexListModel_new(initial_size)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewIndexListModelIsOwned<OWNED>>
    for DataViewListModelIsOwned<OWNED>
{
    fn from(o: DataViewIndexListModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewIndexListModelIsOwned<OWNED>> for DataViewModelIsOwned<OWNED> {
    fn from(o: DataViewIndexListModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewIndexListModelIsOwned<OWNED>> for RefCounterIsOwned<OWNED> {
    fn from(o: DataViewIndexListModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewIndexListModelIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewIndexListModel_delete(self.0) }
        }
    }
}

// wxDataViewItem
wx_class! { DataViewItem =
    DataViewItemIsOwned<true>(wxDataViewItem) impl
        DataViewItemMethods
}
impl<const OWNED: bool> DataViewItemIsOwned<OWNED> {
    pub fn new() -> DataViewItemIsOwned<OWNED> {
        unsafe { DataViewItemIsOwned(ffi::wxDataViewItem_new()) }
    }
    pub fn new_with_dataviewitem<D: DataViewItemMethods>(item: &D) -> DataViewItemIsOwned<OWNED> {
        unsafe {
            let item = item.as_ptr();
            DataViewItemIsOwned(ffi::wxDataViewItem_new1(item))
        }
    }
    pub fn new_with_void(id: *mut c_void) -> DataViewItemIsOwned<OWNED> {
        unsafe { DataViewItemIsOwned(ffi::wxDataViewItem_new2(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DataViewItemIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewItem_delete(self.0) }
        }
    }
}

// wxDataViewItemAttr
wx_class! { DataViewItemAttr =
    DataViewItemAttrIsOwned<true>(wxDataViewItemAttr) impl
        DataViewItemAttrMethods
}
impl<const OWNED: bool> DataViewItemAttrIsOwned<OWNED> {
    pub fn new() -> DataViewItemAttrIsOwned<OWNED> {
        unsafe { DataViewItemAttrIsOwned(ffi::wxDataViewItemAttr_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DataViewItemAttrIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewItemAttr_delete(self.0) }
        }
    }
}

// wxDataViewListCtrl
wx_class! { DataViewListCtrl =
    DataViewListCtrlIsOwned<true>(wxDataViewListCtrl) impl
        DataViewListCtrlMethods,
        DataViewCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewListCtrlIsOwned<OWNED> {
    pub fn new_2step() -> DataViewListCtrlIsOwned<OWNED> {
        unsafe { DataViewListCtrlIsOwned(ffi::wxDataViewListCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
    ) -> DataViewListCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            DataViewListCtrlIsOwned(ffi::wxDataViewListCtrl_new1(
                parent, id, pos, size, style, validator,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewListCtrlIsOwned<OWNED>> for DataViewCtrlIsOwned<OWNED> {
    fn from(o: DataViewListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: DataViewListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DataViewListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DataViewListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewListCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewListCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxDataViewListCtrl
impl<const OWNED: bool> TrackableMethods for DataViewListCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxDataViewListCtrl_AsTrackable(self.as_ptr()) }
    }
}

// wxDataViewListModel
wx_class! { DataViewListModel =
    DataViewListModelIsOwned<true>(wxDataViewListModel) impl
        DataViewListModelMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewListModelIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewListModelIsOwned<OWNED>> for DataViewModelIsOwned<OWNED> {
    fn from(o: DataViewListModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListModelIsOwned<OWNED>> for RefCounterIsOwned<OWNED> {
    fn from(o: DataViewListModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewListModelIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewListModel_delete(self.0) }
        }
    }
}

// wxDataViewListStore
wx_class! { DataViewListStore =
    DataViewListStoreIsOwned<true>(wxDataViewListStore) impl
        DataViewListStoreMethods,
        DataViewIndexListModelMethods,
        DataViewListModelMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewListStoreIsOwned<OWNED> {
    pub fn new() -> DataViewListStoreIsOwned<OWNED> {
        unsafe { DataViewListStoreIsOwned(ffi::wxDataViewListStore_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewListStoreIsOwned<OWNED>>
    for DataViewIndexListModelIsOwned<OWNED>
{
    fn from(o: DataViewListStoreIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListStoreIsOwned<OWNED>> for DataViewListModelIsOwned<OWNED> {
    fn from(o: DataViewListStoreIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListStoreIsOwned<OWNED>> for DataViewModelIsOwned<OWNED> {
    fn from(o: DataViewListStoreIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListStoreIsOwned<OWNED>> for RefCounterIsOwned<OWNED> {
    fn from(o: DataViewListStoreIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewListStoreIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewListStore_delete(self.0) }
        }
    }
}

// wxDataViewModel
wx_class! { DataViewModel =
    DataViewModelIsOwned<true>(wxDataViewModel) impl
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewModelIsOwned<OWNED> {
    pub fn new() -> DataViewModelIsOwned<OWNED> {
        unsafe { DataViewModelIsOwned(ffi::wxDataViewModel_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewModelIsOwned<OWNED>> for RefCounterIsOwned<OWNED> {
    fn from(o: DataViewModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewModelIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewModel_delete(self.0) }
        }
    }
}

// wxDataViewModelNotifier
wx_class! { DataViewModelNotifier =
    DataViewModelNotifierIsOwned<true>(wxDataViewModelNotifier) impl
        DataViewModelNotifierMethods
}
impl<const OWNED: bool> DataViewModelNotifierIsOwned<OWNED> {
    pub fn new() -> DataViewModelNotifierIsOwned<OWNED> {
        unsafe { DataViewModelNotifierIsOwned(ffi::wxDataViewModelNotifier_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DataViewModelNotifierIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewModelNotifier_delete(self.0) }
        }
    }
}

// wxDataViewProgressRenderer
wx_class! { DataViewProgressRenderer =
    DataViewProgressRendererIsOwned<true>(wxDataViewProgressRenderer) impl
        DataViewProgressRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewProgressRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewProgressRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewProgressRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewProgressRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewProgressRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewProgressRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewProgressRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewProgressRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewProgressRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewRenderer
wx_class! { DataViewRenderer =
    DataViewRendererIsOwned<true>(wxDataViewRenderer) impl
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewSpinRenderer
wx_class! { DataViewSpinRenderer =
    DataViewSpinRendererIsOwned<true>(wxDataViewSpinRenderer) impl
        DataViewSpinRendererMethods,
        DataViewCustomRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewSpinRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewSpinRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewSpinRendererIsOwned<OWNED>>
    for DataViewCustomRendererIsOwned<OWNED>
{
    fn from(o: DataViewSpinRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewSpinRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewSpinRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewSpinRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewSpinRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewSpinRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewSpinRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewSpinRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewTextRenderer
wx_class! { DataViewTextRenderer =
    DataViewTextRendererIsOwned<true>(wxDataViewTextRenderer) impl
        DataViewTextRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewTextRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewTextRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewTextRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewTextRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTextRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewTextRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewTextRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewTextRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewTextRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewToggleRenderer
wx_class! { DataViewToggleRenderer =
    DataViewToggleRendererIsOwned<true>(wxDataViewToggleRenderer) impl
        DataViewToggleRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewToggleRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewToggleRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewToggleRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewToggleRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewToggleRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewToggleRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewToggleRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewToggleRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewToggleRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewTreeCtrl
wx_class! { DataViewTreeCtrl =
    DataViewTreeCtrlIsOwned<true>(wxDataViewTreeCtrl) impl
        DataViewTreeCtrlMethods,
        DataViewCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewTreeCtrlIsOwned<OWNED> {
    pub fn new_2step() -> DataViewTreeCtrlIsOwned<OWNED> {
        unsafe { DataViewTreeCtrlIsOwned(ffi::wxDataViewTreeCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
    ) -> DataViewTreeCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            DataViewTreeCtrlIsOwned(ffi::wxDataViewTreeCtrl_new1(
                parent, id, pos, size, style, validator,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewTreeCtrlIsOwned<OWNED>> for DataViewCtrlIsOwned<OWNED> {
    fn from(o: DataViewTreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTreeCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: DataViewTreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTreeCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DataViewTreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTreeCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DataViewTreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTreeCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewTreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewTreeCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewTreeCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxDataViewTreeCtrl
impl<const OWNED: bool> TrackableMethods for DataViewTreeCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxDataViewTreeCtrl_AsTrackable(self.as_ptr()) }
    }
}

// wxDataViewTreeStore
wx_class! { DataViewTreeStore =
    DataViewTreeStoreIsOwned<true>(wxDataViewTreeStore) impl
        DataViewTreeStoreMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewTreeStoreIsOwned<OWNED> {
    pub fn new() -> DataViewTreeStoreIsOwned<OWNED> {
        unsafe { DataViewTreeStoreIsOwned(ffi::wxDataViewTreeStore_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewTreeStoreIsOwned<OWNED>> for DataViewModelIsOwned<OWNED> {
    fn from(o: DataViewTreeStoreIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTreeStoreIsOwned<OWNED>> for RefCounterIsOwned<OWNED> {
    fn from(o: DataViewTreeStoreIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewTreeStoreIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewTreeStore_delete(self.0) }
        }
    }
}

// wxDataViewValueAdjuster
wx_class! { DataViewValueAdjuster =
    DataViewValueAdjusterIsOwned<true>(wxDataViewValueAdjuster) impl
        DataViewValueAdjusterMethods
}
impl<const OWNED: bool> DataViewValueAdjusterIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DataViewValueAdjusterIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewValueAdjuster_delete(self.0) }
        }
    }
}

// wxDataViewVirtualListModel
wx_class! { DataViewVirtualListModel =
    DataViewVirtualListModelIsOwned<true>(wxDataViewVirtualListModel) impl
        DataViewVirtualListModelMethods,
        DataViewListModelMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewVirtualListModelIsOwned<OWNED> {
    pub fn new(initial_size: c_uint) -> DataViewVirtualListModelIsOwned<OWNED> {
        unsafe {
            DataViewVirtualListModelIsOwned(ffi::wxDataViewVirtualListModel_new(initial_size))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataViewVirtualListModelIsOwned<OWNED>>
    for DataViewListModelIsOwned<OWNED>
{
    fn from(o: DataViewVirtualListModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewVirtualListModelIsOwned<OWNED>>
    for DataViewModelIsOwned<OWNED>
{
    fn from(o: DataViewVirtualListModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewVirtualListModelIsOwned<OWNED>> for RefCounterIsOwned<OWNED> {
    fn from(o: DataViewVirtualListModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewVirtualListModelIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewVirtualListModel_delete(self.0) }
        }
    }
}

// wxDateEvent
wx_class! { DateEvent =
    DateEventIsOwned<true>(wxDateEvent) impl
        DateEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DateEventIsOwned<OWNED> {
    pub fn new() -> DateEventIsOwned<OWNED> {
        unsafe { DateEventIsOwned(ffi::wxDateEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxDateEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DateEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: DateEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DateEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: DateEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DateEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DateEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DateEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDateEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DateEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDatePickerCtrl
wx_class! { DatePickerCtrl =
    DatePickerCtrlIsOwned<true>(wxDatePickerCtrl) impl
        DatePickerCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DatePickerCtrlIsOwned<OWNED> {
    pub fn new_2step() -> DatePickerCtrlIsOwned<OWNED> {
        unsafe { DatePickerCtrlIsOwned(ffi::wxDatePickerCtrl_new()) }
    }
    pub fn new<
        W: WindowMethods,
        D: DateTimeMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        dt: &D,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> DatePickerCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt = dt.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            DatePickerCtrlIsOwned(ffi::wxDatePickerCtrl_new1(
                parent, id, dt, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DatePickerCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: DatePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DatePickerCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DatePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DatePickerCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DatePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DatePickerCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DatePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DatePickerCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDatePickerCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxDatePickerCtrl
impl<const OWNED: bool> TrackableMethods for DatePickerCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxDatePickerCtrl_AsTrackable(self.as_ptr()) }
    }
}

// wxDelegateRendererNative
wx_class! { DelegateRendererNative =
    DelegateRendererNativeIsOwned<true>(wxDelegateRendererNative) impl
        DelegateRendererNativeMethods,
        RendererNativeMethods
}
impl<const OWNED: bool> DelegateRendererNativeIsOwned<OWNED> {
    pub fn new() -> DelegateRendererNativeIsOwned<OWNED> {
        unsafe { DelegateRendererNativeIsOwned(ffi::wxDelegateRendererNative_new()) }
    }
    pub fn new_with_renderernative<R: RendererNativeMethods>(
        renderer_native: &R,
    ) -> DelegateRendererNativeIsOwned<OWNED> {
        unsafe {
            let renderer_native = renderer_native.as_ptr();
            DelegateRendererNativeIsOwned(ffi::wxDelegateRendererNative_new1(renderer_native))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DelegateRendererNativeIsOwned<OWNED>>
    for RendererNativeIsOwned<OWNED>
{
    fn from(o: DelegateRendererNativeIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DelegateRendererNativeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDelegateRendererNative_delete(self.0) }
        }
    }
}

// wxDialUpEvent
wx_class! { DialUpEvent =
    DialUpEventIsOwned<true>(wxDialUpEvent) impl
        DialUpEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DialUpEventIsOwned<OWNED> {
    pub fn new(is_connected: bool, is_own_event: bool) -> DialUpEventIsOwned<OWNED> {
        unsafe { DialUpEventIsOwned(ffi::wxDialUpEvent_new(is_connected, is_own_event)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DialUpEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: DialUpEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DialUpEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DialUpEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DialUpEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDialUpEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DialUpEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDialog
wx_class! { Dialog =
    DialogIsOwned<true>(wxDialog) impl
        DialogMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DialogIsOwned<OWNED> {
    pub fn new_2step() -> DialogIsOwned<OWNED> {
        unsafe { DialogIsOwned(ffi::wxDialog_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> DialogIsOwned<OWNED> {
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
            DialogIsOwned(ffi::wxDialog_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: DialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: DialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDialog_CLASSINFO()) }
    }
}
// Mix-in(s) to wxDialog
impl<const OWNED: bool> TrackableMethods for DialogIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxDialog_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TopLevelWindowMethods for DialogIsOwned<OWNED> {
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
            ffi::wxDialog_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
    fn set_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxDialog_SetIcon(self.as_ptr(), icon)
        }
    }
}
impl<const OWNED: bool> WindowMethods for DialogIsOwned<OWNED> {
    fn centre(&self, direction: c_int) {
        unsafe { ffi::wxDialog_Centre(self.as_ptr(), direction) }
    }
}

// wxDialogLayoutAdapter
wx_class! { DialogLayoutAdapter =
    DialogLayoutAdapterIsOwned<true>(wxDialogLayoutAdapter) impl
        DialogLayoutAdapterMethods
}
impl<const OWNED: bool> DialogLayoutAdapterIsOwned<OWNED> {
    pub fn new() -> DialogLayoutAdapterIsOwned<OWNED> {
        unsafe { DialogLayoutAdapterIsOwned(ffi::wxDialogLayoutAdapter_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DialogLayoutAdapterIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDialogLayoutAdapter_delete(self.0) }
        }
    }
}

// wxDirDialog
wx_class! { DirDialog =
    DirDialogIsOwned<true>(wxDirDialog) impl
        DirDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DirDialogIsOwned<OWNED> {
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        message: &str,
        default_path: &str,
        style: c_long,
        pos: &P,
        size: &S,
        name: &str,
    ) -> DirDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let default_path = WxString::from(default_path);
            let default_path = default_path.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            DirDialogIsOwned(ffi::wxDirDialog_new(
                parent,
                message,
                default_path,
                style,
                pos,
                size,
                name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DirDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: DirDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: DirDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: DirDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DirDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DirDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DirDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DirDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDirDialog_CLASSINFO()) }
    }
}
// Mix-in(s) to wxDirDialog
impl<const OWNED: bool> TrackableMethods for DirDialogIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxDirDialog_AsTrackable(self.as_ptr()) }
    }
}

// wxDirPickerCtrl
wx_class! { DirPickerCtrl =
    DirPickerCtrlIsOwned<true>(wxDirPickerCtrl) impl
        DirPickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DirPickerCtrlIsOwned<OWNED> {
    pub fn new_2step() -> DirPickerCtrlIsOwned<OWNED> {
        unsafe { DirPickerCtrlIsOwned(ffi::wxDirPickerCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        path: &str,
        message: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> DirPickerCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let path = WxString::from(path);
            let path = path.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            DirPickerCtrlIsOwned(ffi::wxDirPickerCtrl_new1(
                parent, id, path, message, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DirPickerCtrlIsOwned<OWNED>> for PickerBaseIsOwned<OWNED> {
    fn from(o: DirPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirPickerCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: DirPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirPickerCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DirPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirPickerCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DirPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirPickerCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DirPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DirPickerCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDirPickerCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxDirPickerCtrl
impl<const OWNED: bool> TrackableMethods for DirPickerCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxDirPickerCtrl_AsTrackable(self.as_ptr()) }
    }
}

// wxDisplay
wx_class! { Display =
    DisplayIsOwned<true>(wxDisplay) impl
        DisplayMethods
}
impl<const OWNED: bool> DisplayIsOwned<OWNED> {
    pub fn new() -> DisplayIsOwned<OWNED> {
        unsafe { DisplayIsOwned(ffi::wxDisplay_new()) }
    }
    pub fn new_with_uint(index: c_uint) -> DisplayIsOwned<OWNED> {
        unsafe { DisplayIsOwned(ffi::wxDisplay_new1(index)) }
    }
    pub fn new_with_window<W: WindowMethods>(window: Option<&W>) -> DisplayIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DisplayIsOwned(ffi::wxDisplay_new2(window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DisplayIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDisplay_delete(self.0) }
        }
    }
}

// wxDisplayChangedEvent
wx_class! { DisplayChangedEvent =
    DisplayChangedEventIsOwned<true>(wxDisplayChangedEvent) impl
        DisplayChangedEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DisplayChangedEventIsOwned<OWNED> {
    pub fn new() -> DisplayChangedEventIsOwned<OWNED> {
        unsafe { DisplayChangedEventIsOwned(ffi::wxDisplayChangedEvent_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DisplayChangedEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: DisplayChangedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DisplayChangedEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DisplayChangedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DisplayChangedEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDisplayChangedEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DisplayChangedEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDocChildFrame
wx_class! { DocChildFrame =
    DocChildFrameIsOwned<true>(wxDocChildFrame) impl
        DocChildFrameMethods,
        FrameMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DocChildFrameIsOwned<OWNED> {
    pub fn new<
        D: DocumentMethods,
        V: ViewMethods,
        F: FrameMethods,
        P: PointMethods,
        S: SizeMethods,
    >(
        doc: Option<&D>,
        view: Option<&V>,
        parent: Option<&F>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> DocChildFrameIsOwned<OWNED> {
        unsafe {
            let doc = match doc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let view = match view {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
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
            DocChildFrameIsOwned(ffi::wxDocChildFrame_new(
                doc, view, parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DocChildFrameIsOwned<OWNED>> for FrameIsOwned<OWNED> {
    fn from(o: DocChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocChildFrameIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: DocChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocChildFrameIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: DocChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocChildFrameIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DocChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocChildFrameIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DocChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocChildFrameIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DocChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DocChildFrameIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDocChildFrame_CLASSINFO()) }
    }
}
// Mix-in(s) to wxDocChildFrame
impl<const OWNED: bool> TrackableMethods for DocChildFrameIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxDocChildFrame_AsTrackable(self.as_ptr()) }
    }
}

// wxDocMDIChildFrame
wx_class! { DocMDIChildFrame =
    DocMDIChildFrameIsOwned<true>(wxDocMDIChildFrame) impl
        DocMDIChildFrameMethods,
        MDIChildFrameMethods,
        FrameMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DocMDIChildFrameIsOwned<OWNED> {
    pub fn new<
        D: DocumentMethods,
        V: ViewMethods,
        M: MDIParentFrameMethods,
        P: PointMethods,
        S: SizeMethods,
    >(
        doc: Option<&D>,
        view: Option<&V>,
        parent: Option<&M>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> DocMDIChildFrameIsOwned<OWNED> {
        unsafe {
            let doc = match doc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let view = match view {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
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
            DocMDIChildFrameIsOwned(ffi::wxDocMDIChildFrame_new(
                doc, view, parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DocMDIChildFrameIsOwned<OWNED>> for MDIChildFrameIsOwned<OWNED> {
    fn from(o: DocMDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocMDIChildFrameIsOwned<OWNED>> for FrameIsOwned<OWNED> {
    fn from(o: DocMDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocMDIChildFrameIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: DocMDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocMDIChildFrameIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: DocMDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocMDIChildFrameIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DocMDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocMDIChildFrameIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DocMDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocMDIChildFrameIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DocMDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DocMDIChildFrameIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDocMDIChildFrame_CLASSINFO()) }
    }
}
// Mix-in(s) to wxDocMDIChildFrame
impl<const OWNED: bool> TrackableMethods for DocMDIChildFrameIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxDocMDIChildFrame_AsTrackable(self.as_ptr()) }
    }
}

// wxDocMDIParentFrame
wx_class! { DocMDIParentFrame =
    DocMDIParentFrameIsOwned<true>(wxDocMDIParentFrame) impl
        DocMDIParentFrameMethods,
        MDIParentFrameMethods,
        FrameMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DocMDIParentFrameIsOwned<OWNED> {
    pub fn new_2step() -> DocMDIParentFrameIsOwned<OWNED> {
        unsafe { DocMDIParentFrameIsOwned(ffi::wxDocMDIParentFrame_new()) }
    }
    pub fn new<D: DocManagerMethods, F: FrameMethods, P: PointMethods, S: SizeMethods>(
        manager: Option<&D>,
        parent: Option<&F>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> DocMDIParentFrameIsOwned<OWNED> {
        unsafe {
            let manager = match manager {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
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
            DocMDIParentFrameIsOwned(ffi::wxDocMDIParentFrame_new1(
                manager, parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DocMDIParentFrameIsOwned<OWNED>> for MDIParentFrameIsOwned<OWNED> {
    fn from(o: DocMDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocMDIParentFrameIsOwned<OWNED>> for FrameIsOwned<OWNED> {
    fn from(o: DocMDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocMDIParentFrameIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: DocMDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocMDIParentFrameIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: DocMDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocMDIParentFrameIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DocMDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocMDIParentFrameIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DocMDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocMDIParentFrameIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DocMDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DocMDIParentFrameIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDocMDIParentFrame_CLASSINFO()) }
    }
}
// Mix-in(s) to wxDocMDIParentFrame
impl<const OWNED: bool> TrackableMethods for DocMDIParentFrameIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxDocMDIParentFrame_AsTrackable(self.as_ptr()) }
    }
}

// wxDocManager
wx_class! { DocManager =
    DocManagerIsOwned<true>(wxDocManager) impl
        DocManagerMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DocManagerIsOwned<OWNED> {
    pub fn new(flags: c_long, initialize: bool) -> DocManagerIsOwned<OWNED> {
        unsafe { DocManagerIsOwned(ffi::wxDocManager_new(flags, initialize)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DocManagerIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DocManagerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocManagerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DocManagerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DocManagerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDocManager_CLASSINFO()) }
    }
}
// Mix-in(s) to wxDocManager
impl<const OWNED: bool> TrackableMethods for DocManagerIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxDocManager_AsTrackable(self.as_ptr()) }
    }
}

// wxDocParentFrame
wx_class! { DocParentFrame =
    DocParentFrameIsOwned<true>(wxDocParentFrame) impl
        DocParentFrameMethods,
        FrameMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DocParentFrameIsOwned<OWNED> {
    pub fn new_2step() -> DocParentFrameIsOwned<OWNED> {
        unsafe { DocParentFrameIsOwned(ffi::wxDocParentFrame_new()) }
    }
    pub fn new<D: DocManagerMethods, F: FrameMethods, P: PointMethods, S: SizeMethods>(
        manager: Option<&D>,
        parent: Option<&F>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> DocParentFrameIsOwned<OWNED> {
        unsafe {
            let manager = match manager {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
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
            DocParentFrameIsOwned(ffi::wxDocParentFrame_new1(
                manager, parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DocParentFrameIsOwned<OWNED>> for FrameIsOwned<OWNED> {
    fn from(o: DocParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocParentFrameIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: DocParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocParentFrameIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: DocParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocParentFrameIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DocParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocParentFrameIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DocParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocParentFrameIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DocParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DocParentFrameIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDocParentFrame_CLASSINFO()) }
    }
}
// Mix-in(s) to wxDocParentFrame
impl<const OWNED: bool> TrackableMethods for DocParentFrameIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxDocParentFrame_AsTrackable(self.as_ptr()) }
    }
}

// wxDocTemplate
wx_class! { DocTemplate =
    DocTemplateIsOwned<true>(wxDocTemplate) impl
        DocTemplateMethods,
        ObjectMethods
}
impl<const OWNED: bool> DocTemplateIsOwned<OWNED> {
    pub fn new<D: DocManagerMethods, C: ClassInfoMethods, C2: ClassInfoMethods>(
        manager: Option<&D>,
        descr: &str,
        filter: &str,
        dir: &str,
        ext: &str,
        doc_type_name: &str,
        view_type_name: &str,
        doc_class_info: Option<&C>,
        view_class_info: Option<&C2>,
        flags: c_long,
    ) -> DocTemplateIsOwned<OWNED> {
        unsafe {
            let manager = match manager {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let descr = WxString::from(descr);
            let descr = descr.as_ptr();
            let filter = WxString::from(filter);
            let filter = filter.as_ptr();
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            let ext = WxString::from(ext);
            let ext = ext.as_ptr();
            let doc_type_name = WxString::from(doc_type_name);
            let doc_type_name = doc_type_name.as_ptr();
            let view_type_name = WxString::from(view_type_name);
            let view_type_name = view_type_name.as_ptr();
            let doc_class_info = match doc_class_info {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let view_class_info = match view_class_info {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DocTemplateIsOwned(ffi::wxDocTemplate_new(
                manager,
                descr,
                filter,
                dir,
                ext,
                doc_type_name,
                view_type_name,
                doc_class_info,
                view_class_info,
                flags,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DocTemplateIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DocTemplateIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DocTemplateIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDocTemplate_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DocTemplateIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDocument
wx_class! { Document =
    DocumentIsOwned<true>(wxDocument) impl
        DocumentMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DocumentIsOwned<OWNED> {
    pub fn new<D: DocumentMethods>(parent: Option<&D>) -> DocumentIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DocumentIsOwned(ffi::wxDocument_new(parent))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DocumentIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DocumentIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DocumentIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DocumentIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DocumentIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDocument_CLASSINFO()) }
    }
}
// Mix-in(s) to wxDocument
impl<const OWNED: bool> TrackableMethods for DocumentIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxDocument_AsTrackable(self.as_ptr()) }
    }
}

// wxDragImage
wx_class! { DragImage =
    DragImageIsOwned<true>(wxDragImage) impl
        DragImageMethods,
        ObjectMethods
}
impl<const OWNED: bool> DragImageIsOwned<OWNED> {
    pub fn new() -> DragImageIsOwned<OWNED> {
        unsafe { DragImageIsOwned(ffi::wxDragImage_new()) }
    }
    pub fn new_with_bitmap<B: BitmapMethods, C: CursorMethods>(
        image: &B,
        cursor: &C,
    ) -> DragImageIsOwned<OWNED> {
        unsafe {
            let image = image.as_ptr();
            let cursor = cursor.as_ptr();
            DragImageIsOwned(ffi::wxDragImage_new1(image, cursor))
        }
    }
    pub fn new_with_icon<I: IconMethods, C: CursorMethods>(
        image: &I,
        cursor: &C,
    ) -> DragImageIsOwned<OWNED> {
        unsafe {
            let image = image.as_ptr();
            let cursor = cursor.as_ptr();
            DragImageIsOwned(ffi::wxDragImage_new2(image, cursor))
        }
    }
    pub fn new_with_str<C: CursorMethods>(text: &str, cursor: &C) -> DragImageIsOwned<OWNED> {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let cursor = cursor.as_ptr();
            DragImageIsOwned(ffi::wxDragImage_new3(text, cursor))
        }
    }
    pub fn new_with_treectrl<T: TreeCtrlMethods, T2: TreeItemIdMethods>(
        tree_ctrl: &T,
        id: &T2,
    ) -> DragImageIsOwned<OWNED> {
        unsafe {
            let tree_ctrl = tree_ctrl.as_ptr();
            let id = id.as_ptr();
            DragImageIsOwned(ffi::wxDragImage_new4(tree_ctrl, id))
        }
    }
    pub fn new_with_listctrl<L: ListCtrlMethods>(
        list_ctrl: &L,
        id: c_long,
    ) -> DragImageIsOwned<OWNED> {
        unsafe {
            let list_ctrl = list_ctrl.as_ptr();
            DragImageIsOwned(ffi::wxDragImage_new5(list_ctrl, id))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DragImageIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DragImageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DragImageIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDragImage_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DragImageIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDropFilesEvent
wx_class! { DropFilesEvent =
    DropFilesEventIsOwned<true>(wxDropFilesEvent) impl
        DropFilesEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DropFilesEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDropFilesEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DropFilesEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: DropFilesEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DropFilesEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DropFilesEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DropFilesEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDropFilesEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DropFilesEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDropSource
wx_class! { DropSource =
    DropSourceIsOwned<true>(wxDropSource) impl
        DropSourceMethods
}
impl<const OWNED: bool> DropSourceIsOwned<OWNED> {
    pub fn new_with_window_cursor<
        W: WindowMethods,
        C: CursorMethods,
        C2: CursorMethods,
        C3: CursorMethods,
    >(
        win: Option<&W>,
        icon_copy: &C,
        icon_move: &C2,
        icon_none: &C3,
    ) -> DropSourceIsOwned<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let icon_copy = icon_copy.as_ptr();
            let icon_move = icon_move.as_ptr();
            let icon_none = icon_none.as_ptr();
            DropSourceIsOwned(ffi::wxDropSource_new(win, icon_copy, icon_move, icon_none))
        }
    }
    pub fn new_with_dataobject_cursor<
        D: DataObjectMethods,
        W: WindowMethods,
        C: CursorMethods,
        C2: CursorMethods,
        C3: CursorMethods,
    >(
        data: &D,
        win: Option<&W>,
        icon_copy: &C,
        icon_move: &C2,
        icon_none: &C3,
    ) -> DropSourceIsOwned<OWNED> {
        unsafe {
            let data = data.as_ptr();
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let icon_copy = icon_copy.as_ptr();
            let icon_move = icon_move.as_ptr();
            let icon_none = icon_none.as_ptr();
            DropSourceIsOwned(ffi::wxDropSource_new1(
                data, win, icon_copy, icon_move, icon_none,
            ))
        }
    }
    pub fn new_with_window_icon<
        W: WindowMethods,
        I: IconMethods,
        I2: IconMethods,
        I3: IconMethods,
    >(
        win: Option<&W>,
        icon_copy: &I,
        icon_move: &I2,
        icon_none: &I3,
    ) -> DropSourceIsOwned<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let icon_copy = icon_copy.as_ptr();
            let icon_move = icon_move.as_ptr();
            let icon_none = icon_none.as_ptr();
            DropSourceIsOwned(ffi::wxDropSource_new2(win, icon_copy, icon_move, icon_none))
        }
    }
    pub fn new_with_dataobject_icon<
        D: DataObjectMethods,
        W: WindowMethods,
        I: IconMethods,
        I2: IconMethods,
        I3: IconMethods,
    >(
        data: &D,
        win: Option<&W>,
        icon_copy: &I,
        icon_move: &I2,
        icon_none: &I3,
    ) -> DropSourceIsOwned<OWNED> {
        unsafe {
            let data = data.as_ptr();
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let icon_copy = icon_copy.as_ptr();
            let icon_move = icon_move.as_ptr();
            let icon_none = icon_none.as_ptr();
            DropSourceIsOwned(ffi::wxDropSource_new3(
                data, win, icon_copy, icon_move, icon_none,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DropSourceIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDropSource_delete(self.0) }
        }
    }
}

// wxDropTarget
wx_class! { DropTarget =
    DropTargetIsOwned<true>(wxDropTarget) impl
        DropTargetMethods
}
impl<const OWNED: bool> DropTargetIsOwned<OWNED> {
    pub fn new<D: DataObjectMethods>(data: Option<&D>) -> DropTargetIsOwned<OWNED> {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DropTargetIsOwned(ffi::wxDropTarget_new(data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DropTargetIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDropTarget_delete(self.0) }
        }
    }
}

// wxEditableListBox
wx_class! { EditableListBox =
    EditableListBoxIsOwned<true>(wxEditableListBox) impl
        EditableListBoxMethods,
        PanelMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> EditableListBoxIsOwned<OWNED> {
    pub fn new_2step() -> EditableListBoxIsOwned<OWNED> {
        unsafe { EditableListBoxIsOwned(ffi::wxEditableListBox_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> EditableListBoxIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            EditableListBoxIsOwned(ffi::wxEditableListBox_new1(
                parent, id, label, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<EditableListBoxIsOwned<OWNED>> for PanelIsOwned<OWNED> {
    fn from(o: EditableListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<EditableListBoxIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: EditableListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<EditableListBoxIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: EditableListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<EditableListBoxIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: EditableListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for EditableListBoxIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxEditableListBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxEditableListBox
impl<const OWNED: bool> TrackableMethods for EditableListBoxIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxEditableListBox_AsTrackable(self.as_ptr()) }
    }
}

// wxEraseEvent
wx_class! { EraseEvent =
    EraseEventIsOwned<true>(wxEraseEvent) impl
        EraseEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> EraseEventIsOwned<OWNED> {
    pub fn new<D: DCMethods>(id: c_int, dc: Option<&D>) -> EraseEventIsOwned<OWNED> {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            EraseEventIsOwned(ffi::wxEraseEvent_new(id, dc))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<EraseEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: EraseEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<EraseEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: EraseEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for EraseEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxEraseEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for EraseEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxEventBlocker
wx_class! { EventBlocker =
    EventBlockerIsOwned<true>(wxEventBlocker) impl
        EventBlockerMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> EventBlockerIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxEventBlocker()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<EventBlockerIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: EventBlockerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<EventBlockerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: EventBlockerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for EventBlockerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxEventBlocker_CLASSINFO()) }
    }
}
// Mix-in(s) to wxEventBlocker
impl<const OWNED: bool> TrackableMethods for EventBlockerIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxEventBlocker_AsTrackable(self.as_ptr()) }
    }
}

// wxExtHelpController
wx_class! { ExtHelpController =
    ExtHelpControllerIsOwned<true>(wxExtHelpController) impl
        ExtHelpControllerMethods,
        HelpControllerBaseMethods,
        ObjectMethods
}
impl<const OWNED: bool> ExtHelpControllerIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(parent_window: Option<&W>) -> ExtHelpControllerIsOwned<OWNED> {
        unsafe {
            let parent_window = match parent_window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ExtHelpControllerIsOwned(ffi::wxExtHelpController_new(parent_window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ExtHelpControllerIsOwned<OWNED>> for HelpControllerBaseIsOwned<OWNED> {
    fn from(o: ExtHelpControllerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ExtHelpControllerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ExtHelpControllerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ExtHelpControllerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxExtHelpController_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ExtHelpControllerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFileCtrl
wx_class! { FileCtrl =
    FileCtrlIsOwned<true>(wxFileCtrl) impl
        FileCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FileCtrlIsOwned<OWNED> {
    pub fn new_2step() -> FileCtrlIsOwned<OWNED> {
        unsafe { FileCtrlIsOwned(ffi::wxFileCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        default_directory: &str,
        default_filename: &str,
        wild_card: &str,
        style: c_long,
        pos: &P,
        size: &S,
        name: &str,
    ) -> FileCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let default_directory = WxString::from(default_directory);
            let default_directory = default_directory.as_ptr();
            let default_filename = WxString::from(default_filename);
            let default_filename = default_filename.as_ptr();
            let wild_card = WxString::from(wild_card);
            let wild_card = wild_card.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            FileCtrlIsOwned(ffi::wxFileCtrl_new1(
                parent,
                id,
                default_directory,
                default_filename,
                wild_card,
                style,
                pos,
                size,
                name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FileCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: FileCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: FileCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: FileCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FileCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FileCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFileCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxFileCtrl
impl<const OWNED: bool> TrackableMethods for FileCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxFileCtrl_AsTrackable(self.as_ptr()) }
    }
}

// wxFileCtrlEvent
wx_class! { FileCtrlEvent =
    FileCtrlEventIsOwned<true>(wxFileCtrlEvent) impl
        FileCtrlEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> FileCtrlEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxFileCtrlEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FileCtrlEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: FileCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileCtrlEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: FileCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileCtrlEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FileCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FileCtrlEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFileCtrlEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FileCtrlEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFileDataObject
wx_class! { FileDataObject =
    FileDataObjectIsOwned<true>(wxFileDataObject) impl
        FileDataObjectMethods,
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> FileDataObjectIsOwned<OWNED> {
    pub fn new() -> FileDataObjectIsOwned<OWNED> {
        unsafe { FileDataObjectIsOwned(ffi::wxFileDataObject_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FileDataObjectIsOwned<OWNED>> for DataObjectSimpleIsOwned<OWNED> {
    fn from(o: FileDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileDataObjectIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: FileDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for FileDataObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFileDataObject_delete(self.0) }
        }
    }
}

// wxFileDialog
wx_class! { FileDialog =
    FileDialogIsOwned<true>(wxFileDialog) impl
        FileDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FileDialogIsOwned<OWNED> {
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        message: &str,
        default_dir: &str,
        default_file: &str,
        wildcard: &str,
        style: c_long,
        pos: &P,
        size: &S,
        name: &str,
    ) -> FileDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let default_dir = WxString::from(default_dir);
            let default_dir = default_dir.as_ptr();
            let default_file = WxString::from(default_file);
            let default_file = default_file.as_ptr();
            let wildcard = WxString::from(wildcard);
            let wildcard = wildcard.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            FileDialogIsOwned(ffi::wxFileDialog_new(
                parent,
                message,
                default_dir,
                default_file,
                wildcard,
                style,
                pos,
                size,
                name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FileDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: FileDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: FileDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: FileDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: FileDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: FileDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FileDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FileDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFileDialog_CLASSINFO()) }
    }
}
// Mix-in(s) to wxFileDialog
impl<const OWNED: bool> TrackableMethods for FileDialogIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxFileDialog_AsTrackable(self.as_ptr()) }
    }
}

// wxFileDialogCustomize
wx_class! { FileDialogCustomize =
    FileDialogCustomizeIsOwned<true>(wxFileDialogCustomize) impl
        FileDialogCustomizeMethods
}
impl<const OWNED: bool> FileDialogCustomizeIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for FileDialogCustomizeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFileDialogCustomize_delete(self.0) }
        }
    }
}

// wxFileDialogCustomizeHook
wx_class! { FileDialogCustomizeHook =
    FileDialogCustomizeHookIsOwned<true>(wxFileDialogCustomizeHook) impl
        FileDialogCustomizeHookMethods
}
impl<const OWNED: bool> FileDialogCustomizeHookIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for FileDialogCustomizeHookIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFileDialogCustomizeHook_delete(self.0) }
        }
    }
}

// wxFileDirPickerEvent
wx_class! { FileDirPickerEvent =
    FileDirPickerEventIsOwned<true>(wxFileDirPickerEvent) impl
        FileDirPickerEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> FileDirPickerEventIsOwned<OWNED> {
    pub fn new() -> FileDirPickerEventIsOwned<OWNED> {
        unsafe { FileDirPickerEventIsOwned(ffi::wxFileDirPickerEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxFileDirPickerEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FileDirPickerEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: FileDirPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileDirPickerEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: FileDirPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileDirPickerEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FileDirPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FileDirPickerEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFileDirPickerEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FileDirPickerEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFileDropTarget
wx_class! { FileDropTarget =
    FileDropTargetIsOwned<true>(wxFileDropTarget) impl
        FileDropTargetMethods,
        DropTargetMethods
}
impl<const OWNED: bool> FileDropTargetIsOwned<OWNED> {
    pub fn new() -> FileDropTargetIsOwned<OWNED> {
        unsafe { FileDropTargetIsOwned(ffi::wxFileDropTarget_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FileDropTargetIsOwned<OWNED>> for DropTargetIsOwned<OWNED> {
    fn from(o: FileDropTargetIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for FileDropTargetIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFileDropTarget_delete(self.0) }
        }
    }
}

// wxFileHistory
wx_class! { FileHistory =
    FileHistoryIsOwned<true>(wxFileHistory) impl
        FileHistoryMethods,
        ObjectMethods
}
impl<const OWNED: bool> FileHistoryIsOwned<OWNED> {
    pub fn new(max_files: usize, id_base: c_int) -> FileHistoryIsOwned<OWNED> {
        unsafe { FileHistoryIsOwned(ffi::wxFileHistory_new(max_files, id_base)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FileHistoryIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FileHistoryIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FileHistoryIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFileHistory_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FileHistoryIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFilePickerCtrl
wx_class! { FilePickerCtrl =
    FilePickerCtrlIsOwned<true>(wxFilePickerCtrl) impl
        FilePickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FilePickerCtrlIsOwned<OWNED> {
    pub fn new_2step() -> FilePickerCtrlIsOwned<OWNED> {
        unsafe { FilePickerCtrlIsOwned(ffi::wxFilePickerCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        path: &str,
        message: &str,
        wildcard: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> FilePickerCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let path = WxString::from(path);
            let path = path.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            let wildcard = WxString::from(wildcard);
            let wildcard = wildcard.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            FilePickerCtrlIsOwned(ffi::wxFilePickerCtrl_new1(
                parent, id, path, message, wildcard, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FilePickerCtrlIsOwned<OWNED>> for PickerBaseIsOwned<OWNED> {
    fn from(o: FilePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FilePickerCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: FilePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FilePickerCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: FilePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FilePickerCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: FilePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FilePickerCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FilePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FilePickerCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFilePickerCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxFilePickerCtrl
impl<const OWNED: bool> TrackableMethods for FilePickerCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxFilePickerCtrl_AsTrackable(self.as_ptr()) }
    }
}

// wxFindDialogEvent
wx_class! { FindDialogEvent =
    FindDialogEventIsOwned<true>(wxFindDialogEvent) impl
        FindDialogEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> FindDialogEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxFindDialogEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FindDialogEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: FindDialogEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FindDialogEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: FindDialogEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FindDialogEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FindDialogEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FindDialogEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFindDialogEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FindDialogEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFindReplaceData
wx_class! { FindReplaceData =
    FindReplaceDataIsOwned<true>(wxFindReplaceData) impl
        FindReplaceDataMethods,
        ObjectMethods
}
impl<const OWNED: bool> FindReplaceDataIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxFindReplaceData()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FindReplaceDataIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FindReplaceDataIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FindReplaceDataIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFindReplaceData_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FindReplaceDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFindReplaceDialog
wx_class! { FindReplaceDialog =
    FindReplaceDialogIsOwned<true>(wxFindReplaceDialog) impl
        FindReplaceDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FindReplaceDialogIsOwned<OWNED> {
    pub fn new_2step() -> FindReplaceDialogIsOwned<OWNED> {
        unsafe { FindReplaceDialogIsOwned(ffi::wxFindReplaceDialog_new()) }
    }
    pub fn new<W: WindowMethods, F: FindReplaceDataMethods>(
        parent: Option<&W>,
        data: Option<&F>,
        title: &str,
        style: c_int,
    ) -> FindReplaceDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            FindReplaceDialogIsOwned(ffi::wxFindReplaceDialog_new1(parent, data, title, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FindReplaceDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: FindReplaceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FindReplaceDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: FindReplaceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FindReplaceDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: FindReplaceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FindReplaceDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: FindReplaceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FindReplaceDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: FindReplaceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FindReplaceDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FindReplaceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FindReplaceDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFindReplaceDialog_CLASSINFO()) }
    }
}
// Mix-in(s) to wxFindReplaceDialog
impl<const OWNED: bool> TrackableMethods for FindReplaceDialogIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxFindReplaceDialog_AsTrackable(self.as_ptr()) }
    }
}

// wxFlexGridSizer
wx_class! { FlexGridSizer =
    FlexGridSizerIsOwned<true>(wxFlexGridSizer) impl
        FlexGridSizerMethods,
        GridSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FlexGridSizerIsOwned<OWNED> {
    pub fn new_with_int_int(cols: c_int, vgap: c_int, hgap: c_int) -> FlexGridSizerIsOwned<OWNED> {
        unsafe { FlexGridSizerIsOwned(ffi::wxFlexGridSizer_new(cols, vgap, hgap)) }
    }
    pub fn new_with_size<S: SizeMethods>(cols: c_int, gap: &S) -> FlexGridSizerIsOwned<OWNED> {
        unsafe {
            let gap = gap.as_ptr();
            FlexGridSizerIsOwned(ffi::wxFlexGridSizer_new1(cols, gap))
        }
    }
    pub fn new_with_int_int_int(
        rows: c_int,
        cols: c_int,
        vgap: c_int,
        hgap: c_int,
    ) -> FlexGridSizerIsOwned<OWNED> {
        unsafe { FlexGridSizerIsOwned(ffi::wxFlexGridSizer_new2(rows, cols, vgap, hgap)) }
    }
    pub fn new_with_int_size<S: SizeMethods>(
        rows: c_int,
        cols: c_int,
        gap: &S,
    ) -> FlexGridSizerIsOwned<OWNED> {
        unsafe {
            let gap = gap.as_ptr();
            FlexGridSizerIsOwned(ffi::wxFlexGridSizer_new3(rows, cols, gap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FlexGridSizerIsOwned<OWNED>> for GridSizerIsOwned<OWNED> {
    fn from(o: FlexGridSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FlexGridSizerIsOwned<OWNED>> for SizerIsOwned<OWNED> {
    fn from(o: FlexGridSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FlexGridSizerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FlexGridSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FlexGridSizerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFlexGridSizer_CLASSINFO()) }
    }
}

// wxFocusEvent
wx_class! { FocusEvent =
    FocusEventIsOwned<true>(wxFocusEvent) impl
        FocusEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> FocusEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxFocusEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FocusEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: FocusEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FocusEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FocusEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FocusEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFocusEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FocusEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFont
wx_class! { Font =
    FontIsOwned<true>(wxFont) impl
        FontMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> FontIsOwned<OWNED> {
    pub fn new() -> FontIsOwned<OWNED> {
        unsafe { FontIsOwned(ffi::wxFont_new()) }
    }
    pub fn new_with_font<F: FontMethods>(font: &F) -> FontIsOwned<OWNED> {
        unsafe {
            let font = font.as_ptr();
            FontIsOwned(ffi::wxFont_new1(font))
        }
    }
    pub fn new_with_fontinfo<F: FontInfoMethods>(font_info: &F) -> FontIsOwned<OWNED> {
        unsafe {
            let font_info = font_info.as_ptr();
            FontIsOwned(ffi::wxFont_new2(font_info))
        }
    }
    // NOT_SUPPORTED: fn wxFont3()
    // NOT_SUPPORTED: fn wxFont4()
    pub fn new_with_str(native_info_string: &str) -> FontIsOwned<OWNED> {
        unsafe {
            let native_info_string = WxString::from(native_info_string);
            let native_info_string = native_info_string.as_ptr();
            FontIsOwned(ffi::wxFont_new5(native_info_string))
        }
    }
    pub fn new_with_nativefontinfo<N: NativeFontInfoMethods>(
        native_info: &N,
    ) -> FontIsOwned<OWNED> {
        unsafe {
            let native_info = native_info.as_ptr();
            FontIsOwned(ffi::wxFont_new6(native_info))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FontIsOwned<OWNED>> for GDIObjectIsOwned<OWNED> {
    fn from(o: FontIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FontIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FontIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFont_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FontIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFontData
wx_class! { FontData =
    FontDataIsOwned<true>(wxFontData) impl
        FontDataMethods,
        ObjectMethods
}
impl<const OWNED: bool> FontDataIsOwned<OWNED> {
    pub fn new() -> FontDataIsOwned<OWNED> {
        unsafe { FontDataIsOwned(ffi::wxFontData_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FontDataIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FontDataIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FontDataIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFontData_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FontDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFontDialog
wx_class! { FontDialog =
    FontDialogIsOwned<true>(wxFontDialog) impl
        FontDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FontDialogIsOwned<OWNED> {
    pub fn new_2step() -> FontDialogIsOwned<OWNED> {
        unsafe { FontDialogIsOwned(ffi::wxFontDialog_new()) }
    }
    pub fn new<W: WindowMethods>(parent: Option<&W>) -> FontDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            FontDialogIsOwned(ffi::wxFontDialog_new1(parent))
        }
    }
    pub fn new<W: WindowMethods, F: FontDataMethods>(
        parent: Option<&W>,
        data: &F,
    ) -> FontDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = data.as_ptr();
            FontDialogIsOwned(ffi::wxFontDialog_new2(parent, data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FontDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: FontDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: FontDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: FontDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: FontDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: FontDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FontDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FontDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFontDialog_CLASSINFO()) }
    }
}
// Mix-in(s) to wxFontDialog
impl<const OWNED: bool> TrackableMethods for FontDialogIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxFontDialog_AsTrackable(self.as_ptr()) }
    }
}

// wxFontEnumerator
wx_class! { FontEnumerator =
    FontEnumeratorIsOwned<true>(wxFontEnumerator) impl
        FontEnumeratorMethods
}
impl<const OWNED: bool> FontEnumeratorIsOwned<OWNED> {
    pub fn new() -> FontEnumeratorIsOwned<OWNED> {
        unsafe { FontEnumeratorIsOwned(ffi::wxFontEnumerator_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for FontEnumeratorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFontEnumerator_delete(self.0) }
        }
    }
}

// wxFontList
wx_class! { FontList =
    FontListIsOwned<true>(wxFontList) impl
        FontListMethods
}
impl<const OWNED: bool> FontListIsOwned<OWNED> {
    pub fn new() -> FontListIsOwned<OWNED> {
        unsafe { FontListIsOwned(ffi::wxFontList_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for FontListIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFontList_delete(self.0) }
        }
    }
}

// wxFontMapper
wx_class! { FontMapper =
    FontMapperIsOwned<true>(wxFontMapper) impl
        FontMapperMethods
}
impl<const OWNED: bool> FontMapperIsOwned<OWNED> {
    pub fn new() -> FontMapperIsOwned<OWNED> {
        unsafe { FontMapperIsOwned(ffi::wxFontMapper_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for FontMapperIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFontMapper_delete(self.0) }
        }
    }
}

// wxFontPickerCtrl
wx_class! { FontPickerCtrl =
    FontPickerCtrlIsOwned<true>(wxFontPickerCtrl) impl
        FontPickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FontPickerCtrlIsOwned<OWNED> {
    pub fn new_2step() -> FontPickerCtrlIsOwned<OWNED> {
        unsafe { FontPickerCtrlIsOwned(ffi::wxFontPickerCtrl_new()) }
    }
    pub fn new<
        W: WindowMethods,
        F: FontMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        font: &F,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> FontPickerCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let font = font.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            FontPickerCtrlIsOwned(ffi::wxFontPickerCtrl_new1(
                parent, id, font, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FontPickerCtrlIsOwned<OWNED>> for PickerBaseIsOwned<OWNED> {
    fn from(o: FontPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontPickerCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: FontPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontPickerCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: FontPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontPickerCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: FontPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontPickerCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FontPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FontPickerCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFontPickerCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxFontPickerCtrl
impl<const OWNED: bool> TrackableMethods for FontPickerCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxFontPickerCtrl_AsTrackable(self.as_ptr()) }
    }
}

// wxFontPickerEvent
wx_class! { FontPickerEvent =
    FontPickerEventIsOwned<true>(wxFontPickerEvent) impl
        FontPickerEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> FontPickerEventIsOwned<OWNED> {
    pub fn new<O: ObjectMethods, F: FontMethods>(
        generator: Option<&O>,
        id: c_int,
        font: &F,
    ) -> FontPickerEventIsOwned<OWNED> {
        unsafe {
            let generator = match generator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let font = font.as_ptr();
            FontPickerEventIsOwned(ffi::wxFontPickerEvent_new(generator, id, font))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FontPickerEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: FontPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontPickerEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: FontPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FontPickerEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FontPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FontPickerEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFontPickerEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FontPickerEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFrame
wx_class! { Frame =
    FrameIsOwned<true>(wxFrame) impl
        FrameMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FrameIsOwned<OWNED> {
    pub fn new_2step() -> FrameIsOwned<OWNED> {
        unsafe { FrameIsOwned(ffi::wxFrame_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> FrameIsOwned<OWNED> {
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
            FrameIsOwned(ffi::wxFrame_new1(parent, id, title, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FrameIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: FrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FrameIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: FrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FrameIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: FrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FrameIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: FrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FrameIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FrameIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFrame_CLASSINFO()) }
    }
}
// Mix-in(s) to wxFrame
impl<const OWNED: bool> TrackableMethods for FrameIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxFrame_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TopLevelWindowMethods for FrameIsOwned<OWNED> {
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
            ffi::wxFrame_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
}
impl<const OWNED: bool> WindowMethods for FrameIsOwned<OWNED> {
    fn centre(&self, direction: c_int) {
        unsafe { ffi::wxFrame_Centre(self.as_ptr(), direction) }
    }
}

// wxFullScreenEvent
wx_class! { FullScreenEvent =
    FullScreenEventIsOwned<true>(wxFullScreenEvent) impl
        FullScreenEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> FullScreenEventIsOwned<OWNED> {
    pub fn new(id: c_int, fullscreen: bool) -> FullScreenEventIsOwned<OWNED> {
        unsafe { FullScreenEventIsOwned(ffi::wxFullScreenEvent_new(id, fullscreen)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FullScreenEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: FullScreenEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FullScreenEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FullScreenEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FullScreenEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFullScreenEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FullScreenEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGBPosition
wx_class! { GBPosition =
    GBPositionIsOwned<true>(wxGBPosition) impl
        GBPositionMethods
}
impl<const OWNED: bool> GBPositionIsOwned<OWNED> {
    pub fn new() -> GBPositionIsOwned<OWNED> {
        unsafe { GBPositionIsOwned(ffi::wxGBPosition_new()) }
    }
    pub fn new_with_int(row: c_int, col: c_int) -> GBPositionIsOwned<OWNED> {
        unsafe { GBPositionIsOwned(ffi::wxGBPosition_new1(row, col)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for GBPositionIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGBPosition_delete(self.0) }
        }
    }
}

// wxGBSizerItem
wx_class! { GBSizerItem =
    GBSizerItemIsOwned<true>(wxGBSizerItem) impl
        GBSizerItemMethods,
        SizerItemMethods,
        ObjectMethods
}
impl<const OWNED: bool> GBSizerItemIsOwned<OWNED> {
    pub fn new_with_int<G: GBPositionMethods, G2: GBSpanMethods, O: ObjectMethods>(
        width: c_int,
        height: c_int,
        pos: &G,
        span: &G2,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> GBSizerItemIsOwned<OWNED> {
        unsafe {
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSizerItemIsOwned(ffi::wxGBSizerItem_new(
                width, height, pos, span, flag, border, user_data,
            ))
        }
    }
    pub fn new_with_window<
        W: WindowMethods,
        G: GBPositionMethods,
        G2: GBSpanMethods,
        O: ObjectMethods,
    >(
        window: Option<&W>,
        pos: &G,
        span: &G2,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> GBSizerItemIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSizerItemIsOwned(ffi::wxGBSizerItem_new1(
                window, pos, span, flag, border, user_data,
            ))
        }
    }
    pub fn new_with_sizer<
        S: SizerMethods,
        G: GBPositionMethods,
        G2: GBSpanMethods,
        O: ObjectMethods,
    >(
        sizer: Option<&S>,
        pos: &G,
        span: &G2,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> GBSizerItemIsOwned<OWNED> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSizerItemIsOwned(ffi::wxGBSizerItem_new2(
                sizer, pos, span, flag, border, user_data,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GBSizerItemIsOwned<OWNED>> for SizerItemIsOwned<OWNED> {
    fn from(o: GBSizerItemIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GBSizerItemIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GBSizerItemIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GBSizerItemIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGBSizerItem_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GBSizerItemIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGBSpan
wx_class! { GBSpan =
    GBSpanIsOwned<true>(wxGBSpan) impl
        GBSpanMethods
}
impl<const OWNED: bool> GBSpanIsOwned<OWNED> {
    pub fn new() -> GBSpanIsOwned<OWNED> {
        unsafe { GBSpanIsOwned(ffi::wxGBSpan_new()) }
    }
    pub fn new_with_int(rowspan: c_int, colspan: c_int) -> GBSpanIsOwned<OWNED> {
        unsafe { GBSpanIsOwned(ffi::wxGBSpan_new1(rowspan, colspan)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for GBSpanIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGBSpan_delete(self.0) }
        }
    }
}

// wxGCDC
wx_class! { GCDC =
    GCDCIsOwned<true>(wxGCDC) impl
        GCDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> GCDCIsOwned<OWNED> {
    pub fn new_with_windowdc<W: WindowDCMethods>(window_dc: &W) -> GCDCIsOwned<OWNED> {
        unsafe {
            let window_dc = window_dc.as_ptr();
            GCDCIsOwned(ffi::wxGCDC_new(window_dc))
        }
    }
    pub fn new_with_memorydc<M: MemoryDCMethods>(memory_dc: &M) -> GCDCIsOwned<OWNED> {
        unsafe {
            let memory_dc = memory_dc.as_ptr();
            GCDCIsOwned(ffi::wxGCDC_new1(memory_dc))
        }
    }
    pub fn new_with_printerdc<P: PrinterDCMethods>(printer_dc: &P) -> GCDCIsOwned<OWNED> {
        unsafe {
            let printer_dc = printer_dc.as_ptr();
            GCDCIsOwned(ffi::wxGCDC_new2(printer_dc))
        }
    }
    pub fn new_with_graphicscontext<G: GraphicsContextMethods>(
        context: Option<&G>,
    ) -> GCDCIsOwned<OWNED> {
        unsafe {
            let context = match context {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GCDCIsOwned(ffi::wxGCDC_new3(context))
        }
    }
    pub fn new_with_enhmetafiledc(emf_dc: *const c_void) -> GCDCIsOwned<OWNED> {
        unsafe { GCDCIsOwned(ffi::wxGCDC_new4(emf_dc)) }
    }
    pub fn new() -> GCDCIsOwned<OWNED> {
        unsafe { GCDCIsOwned(ffi::wxGCDC_new5()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GCDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: GCDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GCDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GCDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GCDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGCDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GCDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGDIObject
wx_class! { GDIObject =
    GDIObjectIsOwned<true>(wxGDIObject) impl
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GDIObjectIsOwned<OWNED> {
    // BLOCKED: fn wxGDIObject()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GDIObjectIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GDIObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GDIObjectIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGDIObject_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GDIObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGIFHandler
wx_class! { GIFHandler =
    GIFHandlerIsOwned<true>(wxGIFHandler) impl
        GIFHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GIFHandlerIsOwned<OWNED> {
    pub fn new() -> GIFHandlerIsOwned<OWNED> {
        unsafe { GIFHandlerIsOwned(ffi::wxGIFHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GIFHandlerIsOwned<OWNED>> for ImageHandlerIsOwned<OWNED> {
    fn from(o: GIFHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GIFHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GIFHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GIFHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGIFHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GIFHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGauge
wx_class! { Gauge =
    GaugeIsOwned<true>(wxGauge) impl
        GaugeMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GaugeIsOwned<OWNED> {
    pub fn new_2step() -> GaugeIsOwned<OWNED> {
        unsafe { GaugeIsOwned(ffi::wxGauge_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        range: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> GaugeIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            GaugeIsOwned(ffi::wxGauge_new1(
                parent, id, range, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GaugeIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: GaugeIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GaugeIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: GaugeIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GaugeIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: GaugeIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GaugeIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GaugeIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GaugeIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGauge_CLASSINFO()) }
    }
}
// Mix-in(s) to wxGauge
impl<const OWNED: bool> TrackableMethods for GaugeIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxGauge_AsTrackable(self.as_ptr()) }
    }
}

// wxGenericAboutDialog
wx_class! { GenericAboutDialog =
    GenericAboutDialogIsOwned<true>(wxGenericAboutDialog) impl
        GenericAboutDialogMethods
}
impl<const OWNED: bool> GenericAboutDialogIsOwned<OWNED> {
    pub fn new() -> GenericAboutDialogIsOwned<OWNED> {
        unsafe { GenericAboutDialogIsOwned(ffi::wxGenericAboutDialog_new()) }
    }
    pub fn new_with_aboutdialoginfo<A: AboutDialogInfoMethods, W: WindowMethods>(
        info: &A,
        parent: Option<&W>,
    ) -> GenericAboutDialogIsOwned<OWNED> {
        unsafe {
            let info = info.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericAboutDialogIsOwned(ffi::wxGenericAboutDialog_new1(info, parent))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for GenericAboutDialogIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGenericAboutDialog_delete(self.0) }
        }
    }
}

// wxGenericAnimationCtrl
wx_class! { GenericAnimationCtrl =
    GenericAnimationCtrlIsOwned<true>(wxGenericAnimationCtrl) impl
        GenericAnimationCtrlMethods,
        // AnimationCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GenericAnimationCtrlIsOwned<OWNED> {
    pub fn new<W: WindowMethods, A: AnimationMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        anim: &A,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> GenericAnimationCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let anim = anim.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            GenericAnimationCtrlIsOwned(ffi::wxGenericAnimationCtrl_new(
                parent, id, anim, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GenericAnimationCtrlIsOwned<OWNED>> for AnimationCtrlIsOwned<OWNED> {
    fn from(o: GenericAnimationCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericAnimationCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: GenericAnimationCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericAnimationCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: GenericAnimationCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericAnimationCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: GenericAnimationCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericAnimationCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GenericAnimationCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GenericAnimationCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGenericAnimationCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxGenericAnimationCtrl
impl<const OWNED: bool> TrackableMethods for GenericAnimationCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxGenericAnimationCtrl_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> AnimationCtrlMethods for GenericAnimationCtrlIsOwned<OWNED> {
    fn create_animation<W: WindowMethods, A: AnimationMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        anim: &A,
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
            let anim = anim.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxGenericAnimationCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                anim,
                pos,
                size,
                style,
                name,
            )
        }
    }
}

// wxGenericDirCtrl
wx_class! { GenericDirCtrl =
    GenericDirCtrlIsOwned<true>(wxGenericDirCtrl) impl
        GenericDirCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GenericDirCtrlIsOwned<OWNED> {
    pub fn new_2step() -> GenericDirCtrlIsOwned<OWNED> {
        unsafe { GenericDirCtrlIsOwned(ffi::wxGenericDirCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        dir: &str,
        pos: &P,
        size: &S,
        style: c_long,
        filter: &str,
        default_filter: c_int,
        name: &str,
    ) -> GenericDirCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let filter = WxString::from(filter);
            let filter = filter.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            GenericDirCtrlIsOwned(ffi::wxGenericDirCtrl_new1(
                parent,
                id,
                dir,
                pos,
                size,
                style,
                filter,
                default_filter,
                name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GenericDirCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: GenericDirCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericDirCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: GenericDirCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericDirCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: GenericDirCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericDirCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GenericDirCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GenericDirCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGenericDirCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxGenericDirCtrl
impl<const OWNED: bool> TrackableMethods for GenericDirCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxGenericDirCtrl_AsTrackable(self.as_ptr()) }
    }
}

// wxGenericProgressDialog
wx_class! { GenericProgressDialog =
    GenericProgressDialogIsOwned<true>(wxGenericProgressDialog) impl
        GenericProgressDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GenericProgressDialogIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(
        title: &str,
        message: &str,
        maximum: c_int,
        parent: Option<&W>,
        style: c_int,
    ) -> GenericProgressDialogIsOwned<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericProgressDialogIsOwned(ffi::wxGenericProgressDialog_new(
                title, message, maximum, parent, style,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GenericProgressDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: GenericProgressDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericProgressDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: GenericProgressDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericProgressDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: GenericProgressDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericProgressDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: GenericProgressDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericProgressDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: GenericProgressDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericProgressDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GenericProgressDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GenericProgressDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGenericProgressDialog_CLASSINFO()) }
    }
}
// Mix-in(s) to wxGenericProgressDialog
impl<const OWNED: bool> TrackableMethods for GenericProgressDialogIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxGenericProgressDialog_AsTrackable(self.as_ptr()) }
    }
}

// wxGenericValidator
wx_class! { GenericValidator =
    GenericValidatorIsOwned<true>(wxGenericValidator) impl
        GenericValidatorMethods,
        ValidatorMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GenericValidatorIsOwned<OWNED> {
    pub fn new_with_genericvalidator<G: GenericValidatorMethods>(
        validator: &G,
    ) -> GenericValidatorIsOwned<OWNED> {
        unsafe {
            let validator = validator.as_ptr();
            GenericValidatorIsOwned(ffi::wxGenericValidator_new(validator))
        }
    }
    pub fn new_with_bool(val_ptr: *mut c_void) -> GenericValidatorIsOwned<OWNED> {
        unsafe { GenericValidatorIsOwned(ffi::wxGenericValidator_new1(val_ptr)) }
    }
    pub fn new_with_str(val_ptr: *mut c_void) -> GenericValidatorIsOwned<OWNED> {
        unsafe { GenericValidatorIsOwned(ffi::wxGenericValidator_new2(val_ptr)) }
    }
    pub fn new_with_int(val_ptr: *mut c_void) -> GenericValidatorIsOwned<OWNED> {
        unsafe { GenericValidatorIsOwned(ffi::wxGenericValidator_new3(val_ptr)) }
    }
    pub fn new_with_arrayint<A: ArrayIntMethods>(
        val_ptr: Option<&A>,
    ) -> GenericValidatorIsOwned<OWNED> {
        unsafe {
            let val_ptr = match val_ptr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericValidatorIsOwned(ffi::wxGenericValidator_new4(val_ptr))
        }
    }
    pub fn new_with_datetime<D: DateTimeMethods>(
        val_ptr: Option<&D>,
    ) -> GenericValidatorIsOwned<OWNED> {
        unsafe {
            let val_ptr = match val_ptr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericValidatorIsOwned(ffi::wxGenericValidator_new5(val_ptr))
        }
    }
    pub fn new_with_filename<F: FileNameMethods>(
        val_ptr: Option<&F>,
    ) -> GenericValidatorIsOwned<OWNED> {
        unsafe {
            let val_ptr = match val_ptr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericValidatorIsOwned(ffi::wxGenericValidator_new6(val_ptr))
        }
    }
    pub fn new_with_float(val_ptr: *mut c_void) -> GenericValidatorIsOwned<OWNED> {
        unsafe { GenericValidatorIsOwned(ffi::wxGenericValidator_new7(val_ptr)) }
    }
    pub fn new_with_double(val_ptr: *mut c_void) -> GenericValidatorIsOwned<OWNED> {
        unsafe { GenericValidatorIsOwned(ffi::wxGenericValidator_new8(val_ptr)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GenericValidatorIsOwned<OWNED>> for ValidatorIsOwned<OWNED> {
    fn from(o: GenericValidatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericValidatorIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: GenericValidatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericValidatorIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GenericValidatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GenericValidatorIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGenericValidator_CLASSINFO()) }
    }
}
// Mix-in(s) to wxGenericValidator
impl<const OWNED: bool> TrackableMethods for GenericValidatorIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxGenericValidator_AsTrackable(self.as_ptr()) }
    }
}

// wxGestureEvent
wx_class! { GestureEvent =
    GestureEventIsOwned<true>(wxGestureEvent) impl
        GestureEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> GestureEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxGestureEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GestureEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: GestureEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GestureEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GestureEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GestureEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGestureEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GestureEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsBrush
wx_class! { GraphicsBrush =
    GraphicsBrushIsOwned<true>(wxGraphicsBrush) impl
        GraphicsBrushMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsBrushIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GraphicsBrushIsOwned<OWNED>> for GraphicsObjectIsOwned<OWNED> {
    fn from(o: GraphicsBrushIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsBrushIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GraphicsBrushIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsBrushIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGraphicsBrush_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsBrushIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsContext
wx_class! { GraphicsContext =
    GraphicsContextIsOwned<true>(wxGraphicsContext) impl
        GraphicsContextMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsContextIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GraphicsContextIsOwned<OWNED>> for GraphicsObjectIsOwned<OWNED> {
    fn from(o: GraphicsContextIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsContextIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GraphicsContextIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsContextIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGraphicsContext_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsContextIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsFont
wx_class! { GraphicsFont =
    GraphicsFontIsOwned<true>(wxGraphicsFont) impl
        GraphicsFontMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsFontIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GraphicsFontIsOwned<OWNED>> for GraphicsObjectIsOwned<OWNED> {
    fn from(o: GraphicsFontIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsFontIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GraphicsFontIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsFontIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGraphicsFont_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsFontIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsGradientStop
wx_class! { GraphicsGradientStop =
    GraphicsGradientStopIsOwned<true>(wxGraphicsGradientStop) impl
        GraphicsGradientStopMethods
}
impl<const OWNED: bool> GraphicsGradientStopIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxGraphicsGradientStop()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for GraphicsGradientStopIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGraphicsGradientStop_delete(self.0) }
        }
    }
}

// wxGraphicsGradientStops
wx_class! { GraphicsGradientStops =
    GraphicsGradientStopsIsOwned<true>(wxGraphicsGradientStops) impl
        GraphicsGradientStopsMethods
}
impl<const OWNED: bool> GraphicsGradientStopsIsOwned<OWNED> {
    pub fn new(
        start_col: ffi::wxColour,
        end_col: ffi::wxColour,
    ) -> GraphicsGradientStopsIsOwned<OWNED> {
        unsafe {
            GraphicsGradientStopsIsOwned(ffi::wxGraphicsGradientStops_new(start_col, end_col))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for GraphicsGradientStopsIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGraphicsGradientStops_delete(self.0) }
        }
    }
}

// wxGraphicsMatrix
wx_class! { GraphicsMatrix =
    GraphicsMatrixIsOwned<true>(wxGraphicsMatrix) impl
        GraphicsMatrixMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsMatrixIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GraphicsMatrixIsOwned<OWNED>> for GraphicsObjectIsOwned<OWNED> {
    fn from(o: GraphicsMatrixIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsMatrixIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GraphicsMatrixIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsMatrixIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGraphicsMatrix_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsMatrixIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsObject
wx_class! { GraphicsObject =
    GraphicsObjectIsOwned<true>(wxGraphicsObject) impl
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsObjectIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GraphicsObjectIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GraphicsObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsObjectIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGraphicsObject_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsPath
wx_class! { GraphicsPath =
    GraphicsPathIsOwned<true>(wxGraphicsPath) impl
        GraphicsPathMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsPathIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GraphicsPathIsOwned<OWNED>> for GraphicsObjectIsOwned<OWNED> {
    fn from(o: GraphicsPathIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsPathIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GraphicsPathIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsPathIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGraphicsPath_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsPathIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsPen
wx_class! { GraphicsPen =
    GraphicsPenIsOwned<true>(wxGraphicsPen) impl
        GraphicsPenMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsPenIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GraphicsPenIsOwned<OWNED>> for GraphicsObjectIsOwned<OWNED> {
    fn from(o: GraphicsPenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsPenIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GraphicsPenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsPenIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGraphicsPen_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsPenIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsRenderer
wx_class! { GraphicsRenderer =
    GraphicsRendererIsOwned<true>(wxGraphicsRenderer) impl
        GraphicsRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsRendererIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GraphicsRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GraphicsRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGraphicsRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridBagSizer
wx_class! { GridBagSizer =
    GridBagSizerIsOwned<true>(wxGridBagSizer) impl
        GridBagSizerMethods,
        FlexGridSizerMethods,
        GridSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridBagSizerIsOwned<OWNED> {
    pub fn new(vgap: c_int, hgap: c_int) -> GridBagSizerIsOwned<OWNED> {
        unsafe { GridBagSizerIsOwned(ffi::wxGridBagSizer_new(vgap, hgap)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridBagSizerIsOwned<OWNED>> for FlexGridSizerIsOwned<OWNED> {
    fn from(o: GridBagSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridBagSizerIsOwned<OWNED>> for GridSizerIsOwned<OWNED> {
    fn from(o: GridBagSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridBagSizerIsOwned<OWNED>> for SizerIsOwned<OWNED> {
    fn from(o: GridBagSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridBagSizerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GridBagSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridBagSizerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGridBagSizer_CLASSINFO()) }
    }
}

// wxGridCellAttr
wx_class! { GridCellAttr =
    GridCellAttrIsOwned<true>(wxGridCellAttr) impl
        GridCellAttrMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellAttrIsOwned<OWNED> {
    //  ENUM: wxAttrKind
    pub const Any: c_int = 0;
    pub const Cell: c_int = 0 + 1;
    pub const Row: c_int = 0 + 2;
    pub const Col: c_int = 0 + 3;
    pub const Default: c_int = 0 + 4;
    pub const Merged: c_int = 0 + 5;

    pub fn new_with_gridcellattr<G: GridCellAttrMethods>(
        attr_default: Option<&G>,
    ) -> GridCellAttrIsOwned<OWNED> {
        unsafe {
            let attr_default = match attr_default {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GridCellAttrIsOwned(ffi::wxGridCellAttr_new(attr_default))
        }
    }
    pub fn new_with_colour<C: ColourMethods, C2: ColourMethods, F: FontMethods>(
        col_text: &C,
        col_back: &C2,
        font: &F,
        h_align: c_int,
        v_align: c_int,
    ) -> GridCellAttrIsOwned<OWNED> {
        unsafe {
            let col_text = col_text.as_ptr();
            let col_back = col_back.as_ptr();
            let font = font.as_ptr();
            GridCellAttrIsOwned(ffi::wxGridCellAttr_new1(
                col_text, col_back, font, h_align, v_align,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellAttrIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellAttrIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellAttrIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellAttr_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellAttr
impl<const OWNED: bool> RefCounterMethods for GridCellAttrIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellAttr_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellAutoWrapStringEditor
wx_class! { GridCellAutoWrapStringEditor =
    GridCellAutoWrapStringEditorIsOwned<true>(wxGridCellAutoWrapStringEditor) impl
        GridCellAutoWrapStringEditorMethods,
        GridCellTextEditorMethods,
        GridCellEditorMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellAutoWrapStringEditorIsOwned<OWNED> {
    pub fn new() -> GridCellAutoWrapStringEditorIsOwned<OWNED> {
        unsafe { GridCellAutoWrapStringEditorIsOwned(ffi::wxGridCellAutoWrapStringEditor_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellAutoWrapStringEditorIsOwned<OWNED>>
    for GridCellTextEditorIsOwned<OWNED>
{
    fn from(o: GridCellAutoWrapStringEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellAutoWrapStringEditorIsOwned<OWNED>>
    for GridCellEditorIsOwned<OWNED>
{
    fn from(o: GridCellAutoWrapStringEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellAutoWrapStringEditorIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellAutoWrapStringEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellAutoWrapStringEditorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellAutoWrapStringEditor_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellAutoWrapStringEditor
impl<const OWNED: bool> RefCounterMethods for GridCellAutoWrapStringEditorIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellAutoWrapStringEditor_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellAutoWrapStringRenderer
wx_class! { GridCellAutoWrapStringRenderer =
    GridCellAutoWrapStringRendererIsOwned<true>(wxGridCellAutoWrapStringRenderer) impl
        GridCellAutoWrapStringRendererMethods,
        GridCellStringRendererMethods,
        GridCellRendererMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellAutoWrapStringRendererIsOwned<OWNED> {
    pub fn new() -> GridCellAutoWrapStringRendererIsOwned<OWNED> {
        unsafe {
            GridCellAutoWrapStringRendererIsOwned(ffi::wxGridCellAutoWrapStringRenderer_new())
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellAutoWrapStringRendererIsOwned<OWNED>>
    for GridCellStringRendererIsOwned<OWNED>
{
    fn from(o: GridCellAutoWrapStringRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellAutoWrapStringRendererIsOwned<OWNED>>
    for GridCellRendererIsOwned<OWNED>
{
    fn from(o: GridCellAutoWrapStringRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellAutoWrapStringRendererIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellAutoWrapStringRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellAutoWrapStringRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellAutoWrapStringRenderer_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellAutoWrapStringRenderer
impl<const OWNED: bool> RefCounterMethods for GridCellAutoWrapStringRendererIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellAutoWrapStringRenderer_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellBoolEditor
wx_class! { GridCellBoolEditor =
    GridCellBoolEditorIsOwned<true>(wxGridCellBoolEditor) impl
        GridCellBoolEditorMethods,
        GridCellEditorMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellBoolEditorIsOwned<OWNED> {
    pub fn new() -> GridCellBoolEditorIsOwned<OWNED> {
        unsafe { GridCellBoolEditorIsOwned(ffi::wxGridCellBoolEditor_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellBoolEditorIsOwned<OWNED>> for GridCellEditorIsOwned<OWNED> {
    fn from(o: GridCellBoolEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellBoolEditorIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellBoolEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellBoolEditorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellBoolEditor_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellBoolEditor
impl<const OWNED: bool> RefCounterMethods for GridCellBoolEditorIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellBoolEditor_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellBoolRenderer
wx_class! { GridCellBoolRenderer =
    GridCellBoolRendererIsOwned<true>(wxGridCellBoolRenderer) impl
        GridCellBoolRendererMethods,
        GridCellRendererMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellBoolRendererIsOwned<OWNED> {
    pub fn new() -> GridCellBoolRendererIsOwned<OWNED> {
        unsafe { GridCellBoolRendererIsOwned(ffi::wxGridCellBoolRenderer_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellBoolRendererIsOwned<OWNED>>
    for GridCellRendererIsOwned<OWNED>
{
    fn from(o: GridCellBoolRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellBoolRendererIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellBoolRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellBoolRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellBoolRenderer_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellBoolRenderer
impl<const OWNED: bool> RefCounterMethods for GridCellBoolRendererIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellBoolRenderer_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellChoiceEditor
wx_class! { GridCellChoiceEditor =
    GridCellChoiceEditorIsOwned<true>(wxGridCellChoiceEditor) impl
        GridCellChoiceEditorMethods,
        GridCellEditorMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellChoiceEditorIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxGridCellChoiceEditor()
    pub fn new<A: ArrayStringMethods>(
        choices: &A,
        allow_others: bool,
    ) -> GridCellChoiceEditorIsOwned<OWNED> {
        unsafe {
            let choices = choices.as_ptr();
            GridCellChoiceEditorIsOwned(ffi::wxGridCellChoiceEditor_new1(choices, allow_others))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellChoiceEditorIsOwned<OWNED>> for GridCellEditorIsOwned<OWNED> {
    fn from(o: GridCellChoiceEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellChoiceEditorIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellChoiceEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellChoiceEditorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellChoiceEditor_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellChoiceEditor
impl<const OWNED: bool> RefCounterMethods for GridCellChoiceEditorIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellChoiceEditor_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellDateEditor
wx_class! { GridCellDateEditor =
    GridCellDateEditorIsOwned<true>(wxGridCellDateEditor) impl
        GridCellDateEditorMethods,
        GridCellEditorMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellDateEditorIsOwned<OWNED> {
    pub fn new(format: &str) -> GridCellDateEditorIsOwned<OWNED> {
        unsafe {
            let format = WxString::from(format);
            let format = format.as_ptr();
            GridCellDateEditorIsOwned(ffi::wxGridCellDateEditor_new(format))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellDateEditorIsOwned<OWNED>> for GridCellEditorIsOwned<OWNED> {
    fn from(o: GridCellDateEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellDateEditorIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellDateEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellDateEditorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellDateEditor_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellDateEditor
impl<const OWNED: bool> RefCounterMethods for GridCellDateEditorIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellDateEditor_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellDateRenderer
wx_class! { GridCellDateRenderer =
    GridCellDateRendererIsOwned<true>(wxGridCellDateRenderer) impl
        GridCellDateRendererMethods,
        GridCellStringRendererMethods,
        GridCellRendererMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellDateRendererIsOwned<OWNED> {
    pub fn new(outformat: &str) -> GridCellDateRendererIsOwned<OWNED> {
        unsafe {
            let outformat = WxString::from(outformat);
            let outformat = outformat.as_ptr();
            GridCellDateRendererIsOwned(ffi::wxGridCellDateRenderer_new(outformat))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellDateRendererIsOwned<OWNED>>
    for GridCellStringRendererIsOwned<OWNED>
{
    fn from(o: GridCellDateRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellDateRendererIsOwned<OWNED>>
    for GridCellRendererIsOwned<OWNED>
{
    fn from(o: GridCellDateRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellDateRendererIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellDateRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellDateRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellDateRenderer_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellDateRenderer
impl<const OWNED: bool> RefCounterMethods for GridCellDateRendererIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellDateRenderer_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellDateTimeRenderer
wx_class! { GridCellDateTimeRenderer =
    GridCellDateTimeRendererIsOwned<true>(wxGridCellDateTimeRenderer) impl
        GridCellDateTimeRendererMethods,
        GridCellDateRendererMethods,
        GridCellStringRendererMethods,
        GridCellRendererMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellDateTimeRendererIsOwned<OWNED> {
    pub fn new(outformat: &str, informat: &str) -> GridCellDateTimeRendererIsOwned<OWNED> {
        unsafe {
            let outformat = WxString::from(outformat);
            let outformat = outformat.as_ptr();
            let informat = WxString::from(informat);
            let informat = informat.as_ptr();
            GridCellDateTimeRendererIsOwned(ffi::wxGridCellDateTimeRenderer_new(
                outformat, informat,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellDateTimeRendererIsOwned<OWNED>>
    for GridCellDateRendererIsOwned<OWNED>
{
    fn from(o: GridCellDateTimeRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellDateTimeRendererIsOwned<OWNED>>
    for GridCellStringRendererIsOwned<OWNED>
{
    fn from(o: GridCellDateTimeRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellDateTimeRendererIsOwned<OWNED>>
    for GridCellRendererIsOwned<OWNED>
{
    fn from(o: GridCellDateTimeRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellDateTimeRendererIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellDateTimeRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellDateTimeRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellDateTimeRenderer_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellDateTimeRenderer
impl<const OWNED: bool> RefCounterMethods for GridCellDateTimeRendererIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellDateTimeRenderer_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellEditor
wx_class! { GridCellEditor =
    GridCellEditorIsOwned<true>(wxGridCellEditor) impl
        GridCellEditorMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellEditorIsOwned<OWNED> {
    pub fn new() -> GridCellEditorIsOwned<OWNED> {
        unsafe { GridCellEditorIsOwned(ffi::wxGridCellEditor_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellEditorIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellEditorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellEditor_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellEditor
impl<const OWNED: bool> RefCounterMethods for GridCellEditorIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellEditor_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellEnumEditor
wx_class! { GridCellEnumEditor =
    GridCellEnumEditorIsOwned<true>(wxGridCellEnumEditor) impl
        GridCellEnumEditorMethods,
        GridCellChoiceEditorMethods,
        GridCellEditorMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellEnumEditorIsOwned<OWNED> {
    pub fn new(choices: &str) -> GridCellEnumEditorIsOwned<OWNED> {
        unsafe {
            let choices = WxString::from(choices);
            let choices = choices.as_ptr();
            GridCellEnumEditorIsOwned(ffi::wxGridCellEnumEditor_new(choices))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellEnumEditorIsOwned<OWNED>>
    for GridCellChoiceEditorIsOwned<OWNED>
{
    fn from(o: GridCellEnumEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellEnumEditorIsOwned<OWNED>> for GridCellEditorIsOwned<OWNED> {
    fn from(o: GridCellEnumEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellEnumEditorIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellEnumEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellEnumEditorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellEnumEditor_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellEnumEditor
impl<const OWNED: bool> RefCounterMethods for GridCellEnumEditorIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellEnumEditor_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellEnumRenderer
wx_class! { GridCellEnumRenderer =
    GridCellEnumRendererIsOwned<true>(wxGridCellEnumRenderer) impl
        GridCellEnumRendererMethods,
        GridCellStringRendererMethods,
        GridCellRendererMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellEnumRendererIsOwned<OWNED> {
    pub fn new(choices: &str) -> GridCellEnumRendererIsOwned<OWNED> {
        unsafe {
            let choices = WxString::from(choices);
            let choices = choices.as_ptr();
            GridCellEnumRendererIsOwned(ffi::wxGridCellEnumRenderer_new(choices))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellEnumRendererIsOwned<OWNED>>
    for GridCellStringRendererIsOwned<OWNED>
{
    fn from(o: GridCellEnumRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellEnumRendererIsOwned<OWNED>>
    for GridCellRendererIsOwned<OWNED>
{
    fn from(o: GridCellEnumRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellEnumRendererIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellEnumRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellEnumRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellEnumRenderer_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellEnumRenderer
impl<const OWNED: bool> RefCounterMethods for GridCellEnumRendererIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellEnumRenderer_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellFloatEditor
wx_class! { GridCellFloatEditor =
    GridCellFloatEditorIsOwned<true>(wxGridCellFloatEditor) impl
        GridCellFloatEditorMethods,
        GridCellTextEditorMethods,
        GridCellEditorMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellFloatEditorIsOwned<OWNED> {
    pub fn new(width: c_int, precision: c_int, format: c_int) -> GridCellFloatEditorIsOwned<OWNED> {
        unsafe {
            GridCellFloatEditorIsOwned(ffi::wxGridCellFloatEditor_new(width, precision, format))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellFloatEditorIsOwned<OWNED>>
    for GridCellTextEditorIsOwned<OWNED>
{
    fn from(o: GridCellFloatEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellFloatEditorIsOwned<OWNED>> for GridCellEditorIsOwned<OWNED> {
    fn from(o: GridCellFloatEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellFloatEditorIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellFloatEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellFloatEditorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellFloatEditor_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellFloatEditor
impl<const OWNED: bool> RefCounterMethods for GridCellFloatEditorIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellFloatEditor_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellFloatRenderer
wx_class! { GridCellFloatRenderer =
    GridCellFloatRendererIsOwned<true>(wxGridCellFloatRenderer) impl
        GridCellFloatRendererMethods,
        GridCellStringRendererMethods,
        GridCellRendererMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellFloatRendererIsOwned<OWNED> {
    pub fn new(
        width: c_int,
        precision: c_int,
        format: c_int,
    ) -> GridCellFloatRendererIsOwned<OWNED> {
        unsafe {
            GridCellFloatRendererIsOwned(ffi::wxGridCellFloatRenderer_new(width, precision, format))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellFloatRendererIsOwned<OWNED>>
    for GridCellStringRendererIsOwned<OWNED>
{
    fn from(o: GridCellFloatRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellFloatRendererIsOwned<OWNED>>
    for GridCellRendererIsOwned<OWNED>
{
    fn from(o: GridCellFloatRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellFloatRendererIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellFloatRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellFloatRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellFloatRenderer_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellFloatRenderer
impl<const OWNED: bool> RefCounterMethods for GridCellFloatRendererIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellFloatRenderer_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellNumberEditor
wx_class! { GridCellNumberEditor =
    GridCellNumberEditorIsOwned<true>(wxGridCellNumberEditor) impl
        GridCellNumberEditorMethods,
        GridCellTextEditorMethods,
        GridCellEditorMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellNumberEditorIsOwned<OWNED> {
    pub fn new(min: c_int, max: c_int) -> GridCellNumberEditorIsOwned<OWNED> {
        unsafe { GridCellNumberEditorIsOwned(ffi::wxGridCellNumberEditor_new(min, max)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellNumberEditorIsOwned<OWNED>>
    for GridCellTextEditorIsOwned<OWNED>
{
    fn from(o: GridCellNumberEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellNumberEditorIsOwned<OWNED>> for GridCellEditorIsOwned<OWNED> {
    fn from(o: GridCellNumberEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellNumberEditorIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellNumberEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellNumberEditorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellNumberEditor_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellNumberEditor
impl<const OWNED: bool> RefCounterMethods for GridCellNumberEditorIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellNumberEditor_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellNumberRenderer
wx_class! { GridCellNumberRenderer =
    GridCellNumberRendererIsOwned<true>(wxGridCellNumberRenderer) impl
        GridCellNumberRendererMethods,
        GridCellStringRendererMethods,
        GridCellRendererMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellNumberRendererIsOwned<OWNED> {
    pub fn new() -> GridCellNumberRendererIsOwned<OWNED> {
        unsafe { GridCellNumberRendererIsOwned(ffi::wxGridCellNumberRenderer_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellNumberRendererIsOwned<OWNED>>
    for GridCellStringRendererIsOwned<OWNED>
{
    fn from(o: GridCellNumberRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellNumberRendererIsOwned<OWNED>>
    for GridCellRendererIsOwned<OWNED>
{
    fn from(o: GridCellNumberRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellNumberRendererIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellNumberRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellNumberRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellNumberRenderer_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellNumberRenderer
impl<const OWNED: bool> RefCounterMethods for GridCellNumberRendererIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellNumberRenderer_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellRenderer
wx_class! { GridCellRenderer =
    GridCellRendererIsOwned<true>(wxGridCellRenderer) impl
        GridCellRendererMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellRendererIsOwned<OWNED> {
    pub fn new() -> GridCellRendererIsOwned<OWNED> {
        unsafe { GridCellRendererIsOwned(ffi::wxGridCellRenderer_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellRendererIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellRenderer_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellRenderer
impl<const OWNED: bool> RefCounterMethods for GridCellRendererIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellRenderer_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellStringRenderer
wx_class! { GridCellStringRenderer =
    GridCellStringRendererIsOwned<true>(wxGridCellStringRenderer) impl
        GridCellStringRendererMethods,
        GridCellRendererMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellStringRendererIsOwned<OWNED> {
    pub fn new() -> GridCellStringRendererIsOwned<OWNED> {
        unsafe { GridCellStringRendererIsOwned(ffi::wxGridCellStringRenderer_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellStringRendererIsOwned<OWNED>>
    for GridCellRendererIsOwned<OWNED>
{
    fn from(o: GridCellStringRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellStringRendererIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellStringRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellStringRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellStringRenderer_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellStringRenderer
impl<const OWNED: bool> RefCounterMethods for GridCellStringRendererIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellStringRenderer_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridCellTextEditor
wx_class! { GridCellTextEditor =
    GridCellTextEditorIsOwned<true>(wxGridCellTextEditor) impl
        GridCellTextEditorMethods,
        GridCellEditorMethods,
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> GridCellTextEditorIsOwned<OWNED> {
    pub fn new(max_chars: usize) -> GridCellTextEditorIsOwned<OWNED> {
        unsafe { GridCellTextEditorIsOwned(ffi::wxGridCellTextEditor_new(max_chars)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridCellTextEditorIsOwned<OWNED>> for GridCellEditorIsOwned<OWNED> {
    fn from(o: GridCellTextEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridCellTextEditorIsOwned<OWNED>>
    for SharedClientDataContainerIsOwned<OWNED>
{
    fn from(o: GridCellTextEditorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for GridCellTextEditorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridCellTextEditor_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxGridCellTextEditor
impl<const OWNED: bool> RefCounterMethods for GridCellTextEditorIsOwned<OWNED> {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { ffi::wxGridCellTextEditor_AsRefCounter(self.as_ptr()) }
    }
}

// wxGridEditorCreatedEvent
wx_class! { GridEditorCreatedEvent =
    GridEditorCreatedEventIsOwned<true>(wxGridEditorCreatedEvent) impl
        GridEditorCreatedEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridEditorCreatedEventIsOwned<OWNED> {
    pub fn new() -> GridEditorCreatedEventIsOwned<OWNED> {
        unsafe { GridEditorCreatedEventIsOwned(ffi::wxGridEditorCreatedEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxGridEditorCreatedEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridEditorCreatedEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: GridEditorCreatedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridEditorCreatedEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: GridEditorCreatedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridEditorCreatedEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GridEditorCreatedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridEditorCreatedEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGridEditorCreatedEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GridEditorCreatedEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridEvent
wx_class! { GridEvent =
    GridEventIsOwned<true>(wxGridEvent) impl
        GridEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridEventIsOwned<OWNED> {
    pub fn new() -> GridEventIsOwned<OWNED> {
        unsafe { GridEventIsOwned(ffi::wxGridEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxGridEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: GridEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: GridEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: GridEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GridEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGridEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GridEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridFitMode
wx_class! { GridFitMode =
    GridFitModeIsOwned<true>(wxGridFitMode) impl
        GridFitModeMethods
}
impl<const OWNED: bool> GridFitModeIsOwned<OWNED> {
    pub fn new() -> GridFitModeIsOwned<OWNED> {
        unsafe { GridFitModeIsOwned(ffi::wxGridFitMode_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for GridFitModeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridFitMode_delete(self.0) }
        }
    }
}

// wxGridRangeSelectEvent
wx_class! { GridRangeSelectEvent =
    GridRangeSelectEventIsOwned<true>(wxGridRangeSelectEvent) impl
        GridRangeSelectEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridRangeSelectEventIsOwned<OWNED> {
    pub fn new() -> GridRangeSelectEventIsOwned<OWNED> {
        unsafe { GridRangeSelectEventIsOwned(ffi::wxGridRangeSelectEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxGridRangeSelectEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridRangeSelectEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: GridRangeSelectEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridRangeSelectEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: GridRangeSelectEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridRangeSelectEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: GridRangeSelectEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridRangeSelectEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GridRangeSelectEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridRangeSelectEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGridRangeSelectEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GridRangeSelectEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridSizeEvent
wx_class! { GridSizeEvent =
    GridSizeEventIsOwned<true>(wxGridSizeEvent) impl
        GridSizeEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridSizeEventIsOwned<OWNED> {
    pub fn new() -> GridSizeEventIsOwned<OWNED> {
        unsafe { GridSizeEventIsOwned(ffi::wxGridSizeEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxGridSizeEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridSizeEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: GridSizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridSizeEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: GridSizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridSizeEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: GridSizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridSizeEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GridSizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridSizeEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGridSizeEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GridSizeEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridSizer
wx_class! { GridSizer =
    GridSizerIsOwned<true>(wxGridSizer) impl
        GridSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridSizerIsOwned<OWNED> {
    pub fn new_with_int_int(cols: c_int, vgap: c_int, hgap: c_int) -> GridSizerIsOwned<OWNED> {
        unsafe { GridSizerIsOwned(ffi::wxGridSizer_new(cols, vgap, hgap)) }
    }
    pub fn new_with_size<S: SizeMethods>(cols: c_int, gap: &S) -> GridSizerIsOwned<OWNED> {
        unsafe {
            let gap = gap.as_ptr();
            GridSizerIsOwned(ffi::wxGridSizer_new1(cols, gap))
        }
    }
    pub fn new_with_int_int_int(
        rows: c_int,
        cols: c_int,
        vgap: c_int,
        hgap: c_int,
    ) -> GridSizerIsOwned<OWNED> {
        unsafe { GridSizerIsOwned(ffi::wxGridSizer_new2(rows, cols, vgap, hgap)) }
    }
    pub fn new_with_int_size<S: SizeMethods>(
        rows: c_int,
        cols: c_int,
        gap: &S,
    ) -> GridSizerIsOwned<OWNED> {
        unsafe {
            let gap = gap.as_ptr();
            GridSizerIsOwned(ffi::wxGridSizer_new3(rows, cols, gap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridSizerIsOwned<OWNED>> for SizerIsOwned<OWNED> {
    fn from(o: GridSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridSizerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GridSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridSizerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGridSizer_CLASSINFO()) }
    }
}

// wxGridTableBase
wx_class! { GridTableBase =
    GridTableBaseIsOwned<true>(wxGridTableBase) impl
        GridTableBaseMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridTableBaseIsOwned<OWNED> {
    pub fn new() -> GridTableBaseIsOwned<OWNED> {
        unsafe { GridTableBaseIsOwned(ffi::wxGridTableBase_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<GridTableBaseIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GridTableBaseIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridTableBaseIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGridTableBase_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GridTableBaseIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridUpdateLocker
wx_class! { GridUpdateLocker =
    GridUpdateLockerIsOwned<true>(wxGridUpdateLocker) impl
        GridUpdateLockerMethods
}
impl<const OWNED: bool> GridUpdateLockerIsOwned<OWNED> {
    pub fn new(grid: *mut c_void) -> GridUpdateLockerIsOwned<OWNED> {
        unsafe { GridUpdateLockerIsOwned(ffi::wxGridUpdateLocker_new(grid)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for GridUpdateLockerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridUpdateLocker_delete(self.0) }
        }
    }
}

// wxHScrolledWindow
wx_class! { HScrolledWindow =
    HScrolledWindowIsOwned<true>(wxHScrolledWindow) impl
        HScrolledWindowMethods,
        // PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> HScrolledWindowIsOwned<OWNED> {
    pub fn new_2step() -> HScrolledWindowIsOwned<OWNED> {
        unsafe { HScrolledWindowIsOwned(ffi::wxHScrolledWindow_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> HScrolledWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            HScrolledWindowIsOwned(ffi::wxHScrolledWindow_new1(
                parent, id, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<HScrolledWindowIsOwned<OWNED>> for PanelIsOwned<OWNED> {
    fn from(o: HScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HScrolledWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: HScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HScrolledWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: HScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HScrolledWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HScrolledWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHScrolledWindow_CLASSINFO()) }
    }
}
// Mix-in(s) to wxHScrolledWindow
impl<const OWNED: bool> VarHScrollHelperMethods for HScrolledWindowIsOwned<OWNED> {
    fn as_var_h_scroll_helper(&self) -> *mut c_void {
        unsafe { ffi::wxHScrolledWindow_AsVarHScrollHelper(self.as_ptr()) }
    }
}
impl<const OWNED: bool> VarScrollHelperBaseMethods for HScrolledWindowIsOwned<OWNED> {
    fn as_var_scroll_helper_base(&self) -> *mut c_void {
        unsafe { ffi::wxHScrolledWindow_AsVarHScrollHelper(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for HScrolledWindowIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxHScrolledWindow_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> PanelMethods for HScrolledWindowIsOwned<OWNED> {
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
            ffi::wxHScrolledWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}
impl<const OWNED: bool> WindowMethods for HScrolledWindowIsOwned<OWNED> {
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
            ffi::wxHScrolledWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxHTMLDataObject
wx_class! { HTMLDataObject =
    HTMLDataObjectIsOwned<true>(wxHTMLDataObject) impl
        HTMLDataObjectMethods,
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> HTMLDataObjectIsOwned<OWNED> {
    pub fn new(html: &str) -> HTMLDataObjectIsOwned<OWNED> {
        unsafe {
            let html = WxString::from(html);
            let html = html.as_ptr();
            HTMLDataObjectIsOwned(ffi::wxHTMLDataObject_new(html))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<HTMLDataObjectIsOwned<OWNED>> for DataObjectSimpleIsOwned<OWNED> {
    fn from(o: HTMLDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HTMLDataObjectIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: HTMLDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for HTMLDataObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxHTMLDataObject_delete(self.0) }
        }
    }
}

// wxHVScrolledWindow
wx_class! { HVScrolledWindow =
    HVScrolledWindowIsOwned<true>(wxHVScrolledWindow) impl
        HVScrolledWindowMethods,
        // PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> HVScrolledWindowIsOwned<OWNED> {
    pub fn new_2step() -> HVScrolledWindowIsOwned<OWNED> {
        unsafe { HVScrolledWindowIsOwned(ffi::wxHVScrolledWindow_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> HVScrolledWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            HVScrolledWindowIsOwned(ffi::wxHVScrolledWindow_new1(
                parent, id, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<HVScrolledWindowIsOwned<OWNED>> for PanelIsOwned<OWNED> {
    fn from(o: HVScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HVScrolledWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: HVScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HVScrolledWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: HVScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HVScrolledWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HVScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HVScrolledWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHVScrolledWindow_CLASSINFO()) }
    }
}
// Mix-in(s) to wxHVScrolledWindow
impl<const OWNED: bool> VarHVScrollHelperMethods for HVScrolledWindowIsOwned<OWNED> {
    fn as_var_hv_scroll_helper(&self) -> *mut c_void {
        unsafe { ffi::wxHVScrolledWindow_AsVarHVScrollHelper(self.as_ptr()) }
    }
}
impl<const OWNED: bool> VarVScrollHelperMethods for HVScrolledWindowIsOwned<OWNED> {
    fn as_var_v_scroll_helper(&self) -> *mut c_void {
        unsafe { ffi::wxHVScrolledWindow_AsVarHVScrollHelper(self.as_ptr()) }
    }
}
impl<const OWNED: bool> VarScrollHelperBaseMethods for HVScrolledWindowIsOwned<OWNED> {
    fn as_var_scroll_helper_base(&self) -> *mut c_void {
        unsafe { ffi::wxHVScrolledWindow_AsVarHVScrollHelper(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for HVScrolledWindowIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxHVScrolledWindow_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> PanelMethods for HVScrolledWindowIsOwned<OWNED> {
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
            ffi::wxHVScrolledWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}
impl<const OWNED: bool> WindowMethods for HVScrolledWindowIsOwned<OWNED> {
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
            ffi::wxHVScrolledWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxHeaderColumn
wx_class! { HeaderColumn =
    HeaderColumnIsOwned<true>(wxHeaderColumn) impl
        HeaderColumnMethods
}
impl<const OWNED: bool> HeaderColumnIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for HeaderColumnIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxHeaderColumn_delete(self.0) }
        }
    }
}

// wxHeaderColumnSimple
wx_class! { HeaderColumnSimple =
    HeaderColumnSimpleIsOwned<true>(wxHeaderColumnSimple) impl
        HeaderColumnSimpleMethods,
        SettableHeaderColumnMethods,
        HeaderColumnMethods
}
impl<const OWNED: bool> HeaderColumnSimpleIsOwned<OWNED> {
    pub fn new_with_str(
        title: &str,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> HeaderColumnSimpleIsOwned<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            HeaderColumnSimpleIsOwned(ffi::wxHeaderColumnSimple_new(title, width, align, flags))
        }
    }
    pub fn new_with_bitmapbundle<B: BitmapBundleMethods>(
        bitmap: &B,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> HeaderColumnSimpleIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            HeaderColumnSimpleIsOwned(ffi::wxHeaderColumnSimple_new1(bitmap, width, align, flags))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<HeaderColumnSimpleIsOwned<OWNED>>
    for SettableHeaderColumnIsOwned<OWNED>
{
    fn from(o: HeaderColumnSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderColumnSimpleIsOwned<OWNED>> for HeaderColumnIsOwned<OWNED> {
    fn from(o: HeaderColumnSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for HeaderColumnSimpleIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxHeaderColumnSimple_delete(self.0) }
        }
    }
}

// wxHeaderCtrl
wx_class! { HeaderCtrl =
    HeaderCtrlIsOwned<true>(wxHeaderCtrl) impl
        HeaderCtrlMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> HeaderCtrlIsOwned<OWNED> {
    // BLOCKED: fn wxHeaderCtrl()
    // BLOCKED: fn wxHeaderCtrl1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<HeaderCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: HeaderCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: HeaderCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: HeaderCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HeaderCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HeaderCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHeaderCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxHeaderCtrl
impl<const OWNED: bool> TrackableMethods for HeaderCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxHeaderCtrl_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> WindowMethods for HeaderCtrlIsOwned<OWNED> {
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        winid: c_int,
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
            ffi::wxHeaderCtrl_Create(self.as_ptr(), parent, winid, pos, size, style, name)
        }
    }
}

// wxHeaderCtrlEvent
wx_class! { HeaderCtrlEvent =
    HeaderCtrlEventIsOwned<true>(wxHeaderCtrlEvent) impl
        HeaderCtrlEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> HeaderCtrlEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxHeaderCtrlEvent()
    pub fn new<H: HeaderCtrlEventMethods>(event: &H) -> HeaderCtrlEventIsOwned<OWNED> {
        unsafe {
            let event = event.as_ptr();
            HeaderCtrlEventIsOwned(ffi::wxHeaderCtrlEvent_new1(event))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<HeaderCtrlEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: HeaderCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: HeaderCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: HeaderCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HeaderCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HeaderCtrlEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHeaderCtrlEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for HeaderCtrlEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxHeaderCtrlSimple
wx_class! { HeaderCtrlSimple =
    HeaderCtrlSimpleIsOwned<true>(wxHeaderCtrlSimple) impl
        HeaderCtrlSimpleMethods,
        HeaderCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> HeaderCtrlSimpleIsOwned<OWNED> {
    pub fn new_2step() -> HeaderCtrlSimpleIsOwned<OWNED> {
        unsafe { HeaderCtrlSimpleIsOwned(ffi::wxHeaderCtrlSimple_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        winid: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> HeaderCtrlSimpleIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            HeaderCtrlSimpleIsOwned(ffi::wxHeaderCtrlSimple_new1(
                parent, winid, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<HeaderCtrlSimpleIsOwned<OWNED>> for HeaderCtrlIsOwned<OWNED> {
    fn from(o: HeaderCtrlSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlSimpleIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: HeaderCtrlSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlSimpleIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: HeaderCtrlSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlSimpleIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: HeaderCtrlSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlSimpleIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HeaderCtrlSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HeaderCtrlSimpleIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHeaderCtrlSimple_CLASSINFO()) }
    }
}
// Mix-in(s) to wxHeaderCtrlSimple
impl<const OWNED: bool> TrackableMethods for HeaderCtrlSimpleIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxHeaderCtrlSimple_AsTrackable(self.as_ptr()) }
    }
}

// wxHelpController
wx_class! { HelpController =
    HelpControllerIsOwned<true>(wxHelpController) impl
        HelpControllerMethods,
        HelpControllerBaseMethods,
        ObjectMethods
}
impl<const OWNED: bool> HelpControllerIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(parent_window: Option<&W>) -> HelpControllerIsOwned<OWNED> {
        unsafe {
            let parent_window = match parent_window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            HelpControllerIsOwned(ffi::wxHelpController_new(parent_window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<HelpControllerIsOwned<OWNED>> for HelpControllerBaseIsOwned<OWNED> {
    fn from(o: HelpControllerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HelpControllerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HelpControllerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HelpControllerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHelpController_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for HelpControllerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxHelpControllerBase
wx_class! { HelpControllerBase =
    HelpControllerBaseIsOwned<true>(wxHelpControllerBase) impl
        HelpControllerBaseMethods,
        ObjectMethods
}
impl<const OWNED: bool> HelpControllerBaseIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(parent_window: Option<&W>) -> HelpControllerBaseIsOwned<OWNED> {
        unsafe {
            let parent_window = match parent_window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            HelpControllerBaseIsOwned(ffi::wxHelpControllerBase_new(parent_window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<HelpControllerBaseIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HelpControllerBaseIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HelpControllerBaseIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHelpControllerBase_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for HelpControllerBaseIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxHelpControllerHelpProvider
wx_class! { HelpControllerHelpProvider =
    HelpControllerHelpProviderIsOwned<true>(wxHelpControllerHelpProvider) impl
        HelpControllerHelpProviderMethods,
        SimpleHelpProviderMethods,
        HelpProviderMethods
}
impl<const OWNED: bool> HelpControllerHelpProviderIsOwned<OWNED> {
    pub fn new<H: HelpControllerBaseMethods>(
        hc: Option<&H>,
    ) -> HelpControllerHelpProviderIsOwned<OWNED> {
        unsafe {
            let hc = match hc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            HelpControllerHelpProviderIsOwned(ffi::wxHelpControllerHelpProvider_new(hc))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<HelpControllerHelpProviderIsOwned<OWNED>>
    for SimpleHelpProviderIsOwned<OWNED>
{
    fn from(o: HelpControllerHelpProviderIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HelpControllerHelpProviderIsOwned<OWNED>>
    for HelpProviderIsOwned<OWNED>
{
    fn from(o: HelpControllerHelpProviderIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for HelpControllerHelpProviderIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxHelpControllerHelpProvider_delete(self.0) }
        }
    }
}

// wxHelpEvent
wx_class! { HelpEvent =
    HelpEventIsOwned<true>(wxHelpEvent) impl
        HelpEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> HelpEventIsOwned<OWNED> {
    //  ENUM: Origin
    pub const Origin_Unknown: c_int = 0;
    pub const Origin_Keyboard: c_int = 0 + 1;
    pub const Origin_HelpButton: c_int = 0 + 2;

    // NOT_SUPPORTED: fn wxHelpEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<HelpEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: HelpEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HelpEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: HelpEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HelpEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HelpEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HelpEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHelpEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for HelpEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxHelpProvider
wx_class! { HelpProvider =
    HelpProviderIsOwned<true>(wxHelpProvider) impl
        HelpProviderMethods
}
impl<const OWNED: bool> HelpProviderIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for HelpProviderIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxHelpProvider_delete(self.0) }
        }
    }
}

// wxHyperlinkCtrl
wx_class! { HyperlinkCtrl =
    HyperlinkCtrlIsOwned<true>(wxHyperlinkCtrl) impl
        HyperlinkCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> HyperlinkCtrlIsOwned<OWNED> {
    pub fn new_2step() -> HyperlinkCtrlIsOwned<OWNED> {
        unsafe { HyperlinkCtrlIsOwned(ffi::wxHyperlinkCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        url: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> HyperlinkCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let url = WxString::from(url);
            let url = url.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            HyperlinkCtrlIsOwned(ffi::wxHyperlinkCtrl_new1(
                parent, id, label, url, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<HyperlinkCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: HyperlinkCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HyperlinkCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: HyperlinkCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HyperlinkCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: HyperlinkCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HyperlinkCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HyperlinkCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HyperlinkCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHyperlinkCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxHyperlinkCtrl
impl<const OWNED: bool> TrackableMethods for HyperlinkCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxHyperlinkCtrl_AsTrackable(self.as_ptr()) }
    }
}

// wxHyperlinkEvent
wx_class! { HyperlinkEvent =
    HyperlinkEventIsOwned<true>(wxHyperlinkEvent) impl
        HyperlinkEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> HyperlinkEventIsOwned<OWNED> {
    pub fn new<O: ObjectMethods>(
        generator: Option<&O>,
        id: c_int,
        url: &str,
    ) -> HyperlinkEventIsOwned<OWNED> {
        unsafe {
            let generator = match generator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let url = WxString::from(url);
            let url = url.as_ptr();
            HyperlinkEventIsOwned(ffi::wxHyperlinkEvent_new(generator, id, url))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<HyperlinkEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: HyperlinkEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HyperlinkEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: HyperlinkEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HyperlinkEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HyperlinkEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HyperlinkEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHyperlinkEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for HyperlinkEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxIFFHandler
wx_class! { IFFHandler =
    IFFHandlerIsOwned<true>(wxIFFHandler) impl
        IFFHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> IFFHandlerIsOwned<OWNED> {
    pub fn new() -> IFFHandlerIsOwned<OWNED> {
        unsafe { IFFHandlerIsOwned(ffi::wxIFFHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<IFFHandlerIsOwned<OWNED>> for ImageHandlerIsOwned<OWNED> {
    fn from(o: IFFHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<IFFHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: IFFHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for IFFHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxIFFHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for IFFHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxIcon
wx_class! { Icon =
    IconIsOwned<true>(wxIcon) impl
        IconMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> IconIsOwned<OWNED> {
    pub fn new() -> IconIsOwned<OWNED> {
        unsafe { IconIsOwned(ffi::wxIcon_new()) }
    }
    pub fn new_with_icon<I: IconMethods>(icon: &I) -> IconIsOwned<OWNED> {
        unsafe {
            let icon = icon.as_ptr();
            IconIsOwned(ffi::wxIcon_new1(icon))
        }
    }
    // NOT_SUPPORTED: fn wxIcon2()
    pub fn new_with_char(bits: *const c_void) -> IconIsOwned<OWNED> {
        unsafe { IconIsOwned(ffi::wxIcon_new3(bits)) }
    }
    // NOT_SUPPORTED: fn wxIcon4()
    pub fn new_with_iconlocation<I: IconLocationMethods>(loc: &I) -> IconIsOwned<OWNED> {
        unsafe {
            let loc = loc.as_ptr();
            IconIsOwned(ffi::wxIcon_new5(loc))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<IconIsOwned<OWNED>> for GDIObjectIsOwned<OWNED> {
    fn from(o: IconIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<IconIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: IconIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for IconIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxIcon_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for IconIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxIconBundle
wx_class! { IconBundle =
    IconBundleIsOwned<true>(wxIconBundle) impl
        IconBundleMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> IconBundleIsOwned<OWNED> {
    //  ENUM: @29
    pub const FALLBACK_NONE: c_int = 0;
    pub const FALLBACK_SYSTEM: c_int = 1;
    pub const FALLBACK_NEAREST_LARGER: c_int = 2;

    pub fn new() -> IconBundleIsOwned<OWNED> {
        unsafe { IconBundleIsOwned(ffi::wxIconBundle_new()) }
    }
    // NOT_SUPPORTED: fn wxIconBundle1()
    // NOT_SUPPORTED: fn wxIconBundle2()
    pub fn new_with_icon<I: IconMethods>(icon: &I) -> IconBundleIsOwned<OWNED> {
        unsafe {
            let icon = icon.as_ptr();
            IconBundleIsOwned(ffi::wxIconBundle_new3(icon))
        }
    }
    // NOT_SUPPORTED: fn wxIconBundle4()
    pub fn new_with_iconbundle<I: IconBundleMethods>(ic: &I) -> IconBundleIsOwned<OWNED> {
        unsafe {
            let ic = ic.as_ptr();
            IconBundleIsOwned(ffi::wxIconBundle_new5(ic))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<IconBundleIsOwned<OWNED>> for GDIObjectIsOwned<OWNED> {
    fn from(o: IconBundleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<IconBundleIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: IconBundleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for IconBundleIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxIconBundle_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for IconBundleIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxIconizeEvent
wx_class! { IconizeEvent =
    IconizeEventIsOwned<true>(wxIconizeEvent) impl
        IconizeEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> IconizeEventIsOwned<OWNED> {
    pub fn new(id: c_int, iconized: bool) -> IconizeEventIsOwned<OWNED> {
        unsafe { IconizeEventIsOwned(ffi::wxIconizeEvent_new(id, iconized)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<IconizeEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: IconizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<IconizeEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: IconizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for IconizeEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxIconizeEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for IconizeEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxIdManager
wx_class! { IdManager =
    IdManagerIsOwned<true>(wxIdManager) impl
        IdManagerMethods
}
impl<const OWNED: bool> IdManagerIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for IdManagerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxIdManager_delete(self.0) }
        }
    }
}

// wxImage
wx_class! { Image =
    ImageIsOwned<true>(wxImage) impl
        ImageMethods,
        ObjectMethods
}
impl<const OWNED: bool> ImageIsOwned<OWNED> {
    pub fn new() -> ImageIsOwned<OWNED> {
        unsafe { ImageIsOwned(ffi::wxImage_new()) }
    }
    pub fn new_with_int_bool(width: c_int, height: c_int, clear: bool) -> ImageIsOwned<OWNED> {
        unsafe { ImageIsOwned(ffi::wxImage_new1(width, height, clear)) }
    }
    pub fn new_with_size_bool<S: SizeMethods>(sz: &S, clear: bool) -> ImageIsOwned<OWNED> {
        unsafe {
            let sz = sz.as_ptr();
            ImageIsOwned(ffi::wxImage_new2(sz, clear))
        }
    }
    pub fn new_with_int_uchar_bool(
        width: c_int,
        height: c_int,
        data: *mut c_void,
        static_data: bool,
    ) -> ImageIsOwned<OWNED> {
        unsafe { ImageIsOwned(ffi::wxImage_new3(width, height, data, static_data)) }
    }
    pub fn new_with_size_uchar_bool<S: SizeMethods>(
        sz: &S,
        data: *mut c_void,
        static_data: bool,
    ) -> ImageIsOwned<OWNED> {
        unsafe {
            let sz = sz.as_ptr();
            ImageIsOwned(ffi::wxImage_new4(sz, data, static_data))
        }
    }
    pub fn new_with_int_uchar_uchar(
        width: c_int,
        height: c_int,
        data: *mut c_void,
        alpha: *mut c_void,
        static_data: bool,
    ) -> ImageIsOwned<OWNED> {
        unsafe { ImageIsOwned(ffi::wxImage_new5(width, height, data, alpha, static_data)) }
    }
    pub fn new_with_size_uchar_uchar<S: SizeMethods>(
        sz: &S,
        data: *mut c_void,
        alpha: *mut c_void,
        static_data: bool,
    ) -> ImageIsOwned<OWNED> {
        unsafe {
            let sz = sz.as_ptr();
            ImageIsOwned(ffi::wxImage_new6(sz, data, alpha, static_data))
        }
    }
    pub fn new_with_char(xpm_data: *const c_void) -> ImageIsOwned<OWNED> {
        unsafe { ImageIsOwned(ffi::wxImage_new7(xpm_data)) }
    }
    // NOT_SUPPORTED: fn wxImage8()
    pub fn new_with_str(name: &str, mimetype: &str, index: c_int) -> ImageIsOwned<OWNED> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ImageIsOwned(ffi::wxImage_new9(name, mimetype, index))
        }
    }
    // NOT_SUPPORTED: fn wxImage10()
    pub fn new_with_inputstream(
        stream: *mut c_void,
        mimetype: &str,
        index: c_int,
    ) -> ImageIsOwned<OWNED> {
        unsafe {
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ImageIsOwned(ffi::wxImage_new11(stream, mimetype, index))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ImageIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ImageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ImageIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxImage_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ImageIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxImageDataObject
wx_class! { ImageDataObject =
    ImageDataObjectIsOwned<true>(wxImageDataObject) impl
        ImageDataObjectMethods,
        CustomDataObjectMethods,
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> ImageDataObjectIsOwned<OWNED> {
    pub fn new<I: ImageMethods>(image: &I) -> ImageDataObjectIsOwned<OWNED> {
        unsafe {
            let image = image.as_ptr();
            ImageDataObjectIsOwned(ffi::wxImageDataObject_new(image))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ImageDataObjectIsOwned<OWNED>> for CustomDataObjectIsOwned<OWNED> {
    fn from(o: ImageDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ImageDataObjectIsOwned<OWNED>> for DataObjectSimpleIsOwned<OWNED> {
    fn from(o: ImageDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ImageDataObjectIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: ImageDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for ImageDataObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxImageDataObject_delete(self.0) }
        }
    }
}

// wxImageHandler
wx_class! { ImageHandler =
    ImageHandlerIsOwned<true>(wxImageHandler) impl
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ImageHandlerIsOwned<OWNED> {
    pub fn new() -> ImageHandlerIsOwned<OWNED> {
        unsafe { ImageHandlerIsOwned(ffi::wxImageHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ImageHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ImageHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ImageHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxImageHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ImageHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxImageList
wx_class! { ImageList =
    ImageListIsOwned<true>(wxImageList) impl
        ImageListMethods,
        ObjectMethods
}
impl<const OWNED: bool> ImageListIsOwned<OWNED> {
    pub fn new() -> ImageListIsOwned<OWNED> {
        unsafe { ImageListIsOwned(ffi::wxImageList_new()) }
    }
    pub fn new_with_int(
        width: c_int,
        height: c_int,
        mask: bool,
        initial_count: c_int,
    ) -> ImageListIsOwned<OWNED> {
        unsafe { ImageListIsOwned(ffi::wxImageList_new1(width, height, mask, initial_count)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ImageListIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ImageListIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ImageListIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxImageList_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ImageListIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxInfoBar
wx_class! { InfoBar =
    InfoBarIsOwned<true>(wxInfoBar) impl
        InfoBarMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> InfoBarIsOwned<OWNED> {
    pub fn new_2step() -> InfoBarIsOwned<OWNED> {
        unsafe { InfoBarIsOwned(ffi::wxInfoBar_new()) }
    }
    pub fn new<W: WindowMethods>(parent: Option<&W>, winid: c_int) -> InfoBarIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            InfoBarIsOwned(ffi::wxInfoBar_new1(parent, winid))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<InfoBarIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: InfoBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<InfoBarIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: InfoBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<InfoBarIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: InfoBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<InfoBarIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: InfoBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for InfoBarIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxInfoBar_CLASSINFO()) }
    }
}
// Mix-in(s) to wxInfoBar
impl<const OWNED: bool> TrackableMethods for InfoBarIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxInfoBar_AsTrackable(self.as_ptr()) }
    }
}

// wxInitDialogEvent
wx_class! { InitDialogEvent =
    InitDialogEventIsOwned<true>(wxInitDialogEvent) impl
        InitDialogEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> InitDialogEventIsOwned<OWNED> {
    pub fn new(id: c_int) -> InitDialogEventIsOwned<OWNED> {
        unsafe { InitDialogEventIsOwned(ffi::wxInitDialogEvent_new(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<InitDialogEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: InitDialogEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<InitDialogEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: InitDialogEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for InitDialogEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxInitDialogEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for InitDialogEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxItemAttr
wx_class! { ItemAttr =
    ItemAttrIsOwned<true>(wxItemAttr) impl
        ItemAttrMethods
}
impl<const OWNED: bool> ItemAttrIsOwned<OWNED> {
    pub fn new() -> ItemAttrIsOwned<OWNED> {
        unsafe { ItemAttrIsOwned(ffi::wxItemAttr_new()) }
    }
    pub fn new_with_colour<C: ColourMethods, C2: ColourMethods, F: FontMethods>(
        col_text: &C,
        col_back: &C2,
        font: &F,
    ) -> ItemAttrIsOwned<OWNED> {
        unsafe {
            let col_text = col_text.as_ptr();
            let col_back = col_back.as_ptr();
            let font = font.as_ptr();
            ItemAttrIsOwned(ffi::wxItemAttr_new1(col_text, col_back, font))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ItemAttrIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxItemAttr_delete(self.0) }
        }
    }
}

// wxItemContainer

// wxItemContainerImmutable

// wxJPEGHandler
wx_class! { JPEGHandler =
    JPEGHandlerIsOwned<true>(wxJPEGHandler) impl
        JPEGHandlerMethods,
        // ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> JPEGHandlerIsOwned<OWNED> {
    pub fn new() -> JPEGHandlerIsOwned<OWNED> {
        unsafe { JPEGHandlerIsOwned(ffi::wxJPEGHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<JPEGHandlerIsOwned<OWNED>> for ImageHandlerIsOwned<OWNED> {
    fn from(o: JPEGHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<JPEGHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: JPEGHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for JPEGHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxJPEGHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for JPEGHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> ImageHandlerMethods for JPEGHandlerIsOwned<OWNED> {
    fn get_library_version_info() -> VersionInfo {
        unsafe { VersionInfo::from_ptr(ffi::wxJPEGHandler_GetLibraryVersionInfo()) }
    }
}

// wxJoystick
wx_class! { Joystick =
    JoystickIsOwned<true>(wxJoystick) impl
        JoystickMethods,
        ObjectMethods
}
impl<const OWNED: bool> JoystickIsOwned<OWNED> {
    pub fn new(joystick: c_int) -> JoystickIsOwned<OWNED> {
        unsafe { JoystickIsOwned(ffi::wxJoystick_new(joystick)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<JoystickIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: JoystickIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for JoystickIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxJoystick_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for JoystickIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxJoystickEvent
wx_class! { JoystickEvent =
    JoystickEventIsOwned<true>(wxJoystickEvent) impl
        JoystickEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> JoystickEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxJoystickEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<JoystickEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: JoystickEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<JoystickEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: JoystickEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for JoystickEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxJoystickEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for JoystickEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxKeyEvent
wx_class! { KeyEvent =
    KeyEventIsOwned<true>(wxKeyEvent) impl
        KeyEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> KeyEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxKeyEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<KeyEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: KeyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<KeyEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: KeyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for KeyEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxKeyEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for KeyEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxKeyEvent
impl<const OWNED: bool> KeyboardStateMethods for KeyEventIsOwned<OWNED> {
    fn as_keyboard_state(&self) -> *mut c_void {
        unsafe { ffi::wxKeyEvent_AsKeyboardState(self.as_ptr()) }
    }
}

// wxLayoutAlgorithm
wx_class! { LayoutAlgorithm =
    LayoutAlgorithmIsOwned<true>(wxLayoutAlgorithm) impl
        LayoutAlgorithmMethods,
        ObjectMethods
}
impl<const OWNED: bool> LayoutAlgorithmIsOwned<OWNED> {
    pub fn new() -> LayoutAlgorithmIsOwned<OWNED> {
        unsafe { LayoutAlgorithmIsOwned(ffi::wxLayoutAlgorithm_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<LayoutAlgorithmIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: LayoutAlgorithmIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for LayoutAlgorithmIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxLayoutAlgorithm_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for LayoutAlgorithmIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxListBox
wx_class! { ListBox =
    ListBoxIsOwned<true>(wxListBox) impl
        ListBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ListBoxIsOwned<OWNED> {
    pub fn new_2step() -> ListBoxIsOwned<OWNED> {
        unsafe { ListBoxIsOwned(ffi::wxListBox_new()) }
    }
    // NOT_SUPPORTED: fn wxListBox1()
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ListBoxIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ListBoxIsOwned(ffi::wxListBox_new2(
                parent, id, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ListBoxIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListBoxIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListBoxIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListBoxIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ListBoxIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxListBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxListBox
impl<const OWNED: bool> ItemContainerMethods for ListBoxIsOwned<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxListBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for ListBoxIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxListBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for ListBoxIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxListBox_AsTrackable(self.as_ptr()) }
    }
}

// wxListCtrl
wx_class! { ListCtrl =
    ListCtrlIsOwned<true>(wxListCtrl) impl
        ListCtrlMethods,
        // ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ListCtrlIsOwned<OWNED> {
    pub fn new_2step() -> ListCtrlIsOwned<OWNED> {
        unsafe { ListCtrlIsOwned(ffi::wxListCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ListCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ListCtrlIsOwned(ffi::wxListCtrl_new1(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ListCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ListCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxListCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxListCtrl
impl<const OWNED: bool> TrackableMethods for ListCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxListCtrl_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ControlMethods for ListCtrlIsOwned<OWNED> {
    fn create_validator<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
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
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxListCtrl_Create(self.as_ptr(), parent, id, pos, size, style, validator, name)
        }
    }
}

// wxListEvent
wx_class! { ListEvent =
    ListEventIsOwned<true>(wxListEvent) impl
        ListEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ListEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxListEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ListEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: ListEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: ListEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ListEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ListEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ListEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxListEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ListEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxListItem
wx_class! { ListItem =
    ListItemIsOwned<true>(wxListItem) impl
        ListItemMethods,
        ObjectMethods
}
impl<const OWNED: bool> ListItemIsOwned<OWNED> {
    pub fn new() -> ListItemIsOwned<OWNED> {
        unsafe { ListItemIsOwned(ffi::wxListItem_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ListItemIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ListItemIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ListItemIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxListItem_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ListItemIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxListView
wx_class! { ListView =
    ListViewIsOwned<true>(wxListView) impl
        ListViewMethods,
        ListCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ListViewIsOwned<OWNED> {
    pub fn new_2step() -> ListViewIsOwned<OWNED> {
        unsafe { ListViewIsOwned(ffi::wxListView_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        winid: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ListViewIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ListViewIsOwned(ffi::wxListView_new1(
                parent, winid, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ListViewIsOwned<OWNED>> for ListCtrlIsOwned<OWNED> {
    fn from(o: ListViewIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListViewIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ListViewIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListViewIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ListViewIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListViewIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ListViewIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListViewIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ListViewIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ListViewIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxListView_CLASSINFO()) }
    }
}
// Mix-in(s) to wxListView
impl<const OWNED: bool> TrackableMethods for ListViewIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxListView_AsTrackable(self.as_ptr()) }
    }
}

// wxListbook
wx_class! { Listbook =
    ListbookIsOwned<true>(wxListbook) impl
        ListbookMethods,
        // BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ListbookIsOwned<OWNED> {
    pub fn new_2step() -> ListbookIsOwned<OWNED> {
        unsafe { ListbookIsOwned(ffi::wxListbook_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> ListbookIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ListbookIsOwned(ffi::wxListbook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ListbookIsOwned<OWNED>> for BookCtrlBaseIsOwned<OWNED> {
    fn from(o: ListbookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListbookIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ListbookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListbookIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ListbookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListbookIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ListbookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListbookIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ListbookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ListbookIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxListbook_CLASSINFO()) }
    }
}
// Mix-in(s) to wxListbook
impl<const OWNED: bool> WithImagesMethods for ListbookIsOwned<OWNED> {
    fn as_with_images(&self) -> *mut c_void {
        unsafe { ffi::wxListbook_AsWithImages(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for ListbookIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxListbook_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> BookCtrlBaseMethods for ListbookIsOwned<OWNED> {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
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
            ffi::wxListbook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}
impl<const OWNED: bool> WindowMethods for ListbookIsOwned<OWNED> {
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
            ffi::wxListbook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxLogGui
wx_class! { LogGui =
    LogGuiIsOwned<true>(wxLogGui) impl
        LogGuiMethods,
        LogMethods
}
impl<const OWNED: bool> LogGuiIsOwned<OWNED> {
    pub fn new() -> LogGuiIsOwned<OWNED> {
        unsafe { LogGuiIsOwned(ffi::wxLogGui_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<LogGuiIsOwned<OWNED>> for LogIsOwned<OWNED> {
    fn from(o: LogGuiIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for LogGuiIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxLogGui_delete(self.0) }
        }
    }
}

// wxLogTextCtrl
wx_class! { LogTextCtrl =
    LogTextCtrlIsOwned<true>(wxLogTextCtrl) impl
        LogTextCtrlMethods,
        LogMethods
}
impl<const OWNED: bool> LogTextCtrlIsOwned<OWNED> {
    pub fn new<T: TextCtrlMethods>(p_text_ctrl: Option<&T>) -> LogTextCtrlIsOwned<OWNED> {
        unsafe {
            let p_text_ctrl = match p_text_ctrl {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            LogTextCtrlIsOwned(ffi::wxLogTextCtrl_new(p_text_ctrl))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<LogTextCtrlIsOwned<OWNED>> for LogIsOwned<OWNED> {
    fn from(o: LogTextCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for LogTextCtrlIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxLogTextCtrl_delete(self.0) }
        }
    }
}

// wxLogWindow
wx_class! { LogWindow =
    LogWindowIsOwned<true>(wxLogWindow) impl
        LogWindowMethods,
        LogInterposerMethods,
        LogChainMethods,
        LogMethods
}
impl<const OWNED: bool> LogWindowIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(
        p_parent: Option<&W>,
        sz_title: &str,
        show: bool,
        pass_to_old: bool,
    ) -> LogWindowIsOwned<OWNED> {
        unsafe {
            let p_parent = match p_parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let sz_title = WxString::from(sz_title);
            let sz_title = sz_title.as_ptr();
            LogWindowIsOwned(ffi::wxLogWindow_new(p_parent, sz_title, show, pass_to_old))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<LogWindowIsOwned<OWNED>> for LogInterposerIsOwned<OWNED> {
    fn from(o: LogWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<LogWindowIsOwned<OWNED>> for LogChainIsOwned<OWNED> {
    fn from(o: LogWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<LogWindowIsOwned<OWNED>> for LogIsOwned<OWNED> {
    fn from(o: LogWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for LogWindowIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxLogWindow_delete(self.0) }
        }
    }
}

// wxLongPressEvent
wx_class! { LongPressEvent =
    LongPressEventIsOwned<true>(wxLongPressEvent) impl
        LongPressEventMethods,
        GestureEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> LongPressEventIsOwned<OWNED> {
    pub fn new(windid: c_int) -> LongPressEventIsOwned<OWNED> {
        unsafe { LongPressEventIsOwned(ffi::wxLongPressEvent_new(windid)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<LongPressEventIsOwned<OWNED>> for GestureEventIsOwned<OWNED> {
    fn from(o: LongPressEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<LongPressEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: LongPressEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<LongPressEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: LongPressEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for LongPressEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxLongPressEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for LongPressEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMDIChildFrame
wx_class! { MDIChildFrame =
    MDIChildFrameIsOwned<true>(wxMDIChildFrame) impl
        MDIChildFrameMethods,
        FrameMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MDIChildFrameIsOwned<OWNED> {
    pub fn new_2step() -> MDIChildFrameIsOwned<OWNED> {
        unsafe { MDIChildFrameIsOwned(ffi::wxMDIChildFrame_new()) }
    }
    pub fn new<M: MDIParentFrameMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&M>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> MDIChildFrameIsOwned<OWNED> {
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
            MDIChildFrameIsOwned(ffi::wxMDIChildFrame_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MDIChildFrameIsOwned<OWNED>> for FrameIsOwned<OWNED> {
    fn from(o: MDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIChildFrameIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: MDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIChildFrameIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: MDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIChildFrameIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: MDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIChildFrameIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: MDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIChildFrameIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MDIChildFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MDIChildFrameIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMDIChildFrame_CLASSINFO()) }
    }
}
// Mix-in(s) to wxMDIChildFrame
impl<const OWNED: bool> TrackableMethods for MDIChildFrameIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxMDIChildFrame_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TopLevelWindowMethods for MDIChildFrameIsOwned<OWNED> {
    fn restore(&self) {
        unsafe { ffi::wxMDIChildFrame_Restore(self.as_ptr()) }
    }
}

// wxMDIClientWindow
wx_class! { MDIClientWindow =
    MDIClientWindowIsOwned<true>(wxMDIClientWindow) impl
        MDIClientWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MDIClientWindowIsOwned<OWNED> {
    pub fn new() -> MDIClientWindowIsOwned<OWNED> {
        unsafe { MDIClientWindowIsOwned(ffi::wxMDIClientWindow_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MDIClientWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: MDIClientWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIClientWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: MDIClientWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIClientWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MDIClientWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MDIClientWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMDIClientWindow_CLASSINFO()) }
    }
}
// Mix-in(s) to wxMDIClientWindow
impl<const OWNED: bool> TrackableMethods for MDIClientWindowIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxMDIClientWindow_AsTrackable(self.as_ptr()) }
    }
}

// wxMDIParentFrame
wx_class! { MDIParentFrame =
    MDIParentFrameIsOwned<true>(wxMDIParentFrame) impl
        MDIParentFrameMethods,
        // FrameMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MDIParentFrameIsOwned<OWNED> {
    pub fn new_2step() -> MDIParentFrameIsOwned<OWNED> {
        unsafe { MDIParentFrameIsOwned(ffi::wxMDIParentFrame_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> MDIParentFrameIsOwned<OWNED> {
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
            MDIParentFrameIsOwned(ffi::wxMDIParentFrame_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MDIParentFrameIsOwned<OWNED>> for FrameIsOwned<OWNED> {
    fn from(o: MDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIParentFrameIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: MDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIParentFrameIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: MDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIParentFrameIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: MDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIParentFrameIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: MDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MDIParentFrameIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MDIParentFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MDIParentFrameIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMDIParentFrame_CLASSINFO()) }
    }
}
// Mix-in(s) to wxMDIParentFrame
impl<const OWNED: bool> TrackableMethods for MDIParentFrameIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxMDIParentFrame_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> FrameMethods for MDIParentFrameIsOwned<OWNED> {
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
            ffi::wxMDIParentFrame_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
}
impl<const OWNED: bool> TopLevelWindowMethods for MDIParentFrameIsOwned<OWNED> {
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
            ffi::wxMDIParentFrame_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
}

// wxMask
wx_class! { Mask =
    MaskIsOwned<true>(wxMask) impl
        MaskMethods,
        ObjectMethods
}
impl<const OWNED: bool> MaskIsOwned<OWNED> {
    pub fn new() -> MaskIsOwned<OWNED> {
        unsafe { MaskIsOwned(ffi::wxMask_new()) }
    }
    pub fn new_with_bitmap_int<B: BitmapMethods>(bitmap: &B, index: c_int) -> MaskIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            MaskIsOwned(ffi::wxMask_new1(bitmap, index))
        }
    }
    pub fn new_with_bitmap<B: BitmapMethods>(bitmap: &B) -> MaskIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            MaskIsOwned(ffi::wxMask_new2(bitmap))
        }
    }
    pub fn new_with_bitmap_colour<B: BitmapMethods, C: ColourMethods>(
        bitmap: &B,
        colour: &C,
    ) -> MaskIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            let colour = colour.as_ptr();
            MaskIsOwned(ffi::wxMask_new3(bitmap, colour))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MaskIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MaskIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MaskIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMask_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MaskIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMaximizeEvent
wx_class! { MaximizeEvent =
    MaximizeEventIsOwned<true>(wxMaximizeEvent) impl
        MaximizeEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MaximizeEventIsOwned<OWNED> {
    pub fn new(id: c_int) -> MaximizeEventIsOwned<OWNED> {
        unsafe { MaximizeEventIsOwned(ffi::wxMaximizeEvent_new(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MaximizeEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: MaximizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MaximizeEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MaximizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MaximizeEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMaximizeEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MaximizeEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMemoryDC
wx_class! { MemoryDC =
    MemoryDCIsOwned<true>(wxMemoryDC) impl
        MemoryDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> MemoryDCIsOwned<OWNED> {
    pub fn new() -> MemoryDCIsOwned<OWNED> {
        unsafe { MemoryDCIsOwned(ffi::wxMemoryDC_new()) }
    }
    pub fn new_with_dc<D: DCMethods>(dc: Option<&D>) -> MemoryDCIsOwned<OWNED> {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MemoryDCIsOwned(ffi::wxMemoryDC_new1(dc))
        }
    }
    pub fn new_with_bitmap<B: BitmapMethods>(bitmap: &B) -> MemoryDCIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            MemoryDCIsOwned(ffi::wxMemoryDC_new2(bitmap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MemoryDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: MemoryDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MemoryDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MemoryDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MemoryDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMemoryDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MemoryDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMenu
wx_class! { Menu =
    MenuIsOwned<true>(wxMenu) impl
        MenuMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuIsOwned<OWNED> {
    pub fn new() -> MenuIsOwned<OWNED> {
        unsafe { MenuIsOwned(ffi::wxMenu_new()) }
    }
    pub fn new_with_long(style: c_long) -> MenuIsOwned<OWNED> {
        unsafe { MenuIsOwned(ffi::wxMenu_new1(style)) }
    }
    pub fn new_with_str(title: &str, style: c_long) -> MenuIsOwned<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            MenuIsOwned(ffi::wxMenu_new2(title, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MenuIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: MenuIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MenuIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MenuIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MenuIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMenu_CLASSINFO()) }
    }
}
// Mix-in(s) to wxMenu
impl<const OWNED: bool> TrackableMethods for MenuIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxMenu_AsTrackable(self.as_ptr()) }
    }
}

// wxMenuBar
wx_class! { MenuBar =
    MenuBarIsOwned<true>(wxMenuBar) impl
        MenuBarMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuBarIsOwned<OWNED> {
    pub fn new(style: c_long) -> MenuBarIsOwned<OWNED> {
        unsafe { MenuBarIsOwned(ffi::wxMenuBar_new(style)) }
    }
    // NOT_SUPPORTED: fn wxMenuBar1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MenuBarIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: MenuBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MenuBarIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: MenuBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MenuBarIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MenuBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MenuBarIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMenuBar_CLASSINFO()) }
    }
}
// Mix-in(s) to wxMenuBar
impl<const OWNED: bool> TrackableMethods for MenuBarIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxMenuBar_AsTrackable(self.as_ptr()) }
    }
}

// wxMenuEvent
wx_class! { MenuEvent =
    MenuEventIsOwned<true>(wxMenuEvent) impl
        MenuEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxMenuEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MenuEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: MenuEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MenuEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MenuEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MenuEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMenuEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MenuEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMenuItem
wx_class! { MenuItem =
    MenuItemIsOwned<true>(wxMenuItem) impl
        MenuItemMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuItemIsOwned<OWNED> {
    pub fn new<M: MenuMethods, M2: MenuMethods>(
        parent_menu: Option<&M>,
        id: c_int,
        text: &str,
        help_string: &str,
        kind: c_int,
        sub_menu: Option<&M2>,
    ) -> MenuItemIsOwned<OWNED> {
        unsafe {
            let parent_menu = match parent_menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let text = WxString::from(text);
            let text = text.as_ptr();
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            let sub_menu = match sub_menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItemIsOwned(ffi::wxMenuItem_new(
                parent_menu,
                id,
                text,
                help_string,
                kind,
                sub_menu,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MenuItemIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MenuItemIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MenuItemIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMenuItem_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MenuItemIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMessageDialog
wx_class! { MessageDialog =
    MessageDialogIsOwned<true>(wxMessageDialog) impl
        MessageDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MessageDialogIsOwned<OWNED> {
    pub fn new<W: WindowMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        caption: &str,
        style: c_long,
        pos: &P,
    ) -> MessageDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let caption = WxString::from(caption);
            let caption = caption.as_ptr();
            let pos = pos.as_ptr();
            MessageDialogIsOwned(ffi::wxMessageDialog_new(
                parent, message, caption, style, pos,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MessageDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: MessageDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MessageDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: MessageDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MessageDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: MessageDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MessageDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: MessageDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MessageDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: MessageDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MessageDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MessageDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MessageDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMessageDialog_CLASSINFO()) }
    }
}
// Mix-in(s) to wxMessageDialog
impl<const OWNED: bool> TrackableMethods for MessageDialogIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxMessageDialog_AsTrackable(self.as_ptr()) }
    }
}

// wxMessageOutputMessageBox
wx_class! { MessageOutputMessageBox =
    MessageOutputMessageBoxIsOwned<true>(wxMessageOutputMessageBox) impl
        MessageOutputMessageBoxMethods,
        MessageOutputMethods
}
impl<const OWNED: bool> MessageOutputMessageBoxIsOwned<OWNED> {
    pub fn new() -> MessageOutputMessageBoxIsOwned<OWNED> {
        unsafe { MessageOutputMessageBoxIsOwned(ffi::wxMessageOutputMessageBox_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MessageOutputMessageBoxIsOwned<OWNED>>
    for MessageOutputIsOwned<OWNED>
{
    fn from(o: MessageOutputMessageBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for MessageOutputMessageBoxIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxMessageOutputMessageBox_delete(self.0) }
        }
    }
}

// wxMetafile
wx_class! { Metafile =
    MetafileIsOwned<true>(wxMetafile) impl
        MetafileMethods,
        ObjectMethods
}
impl<const OWNED: bool> MetafileIsOwned<OWNED> {
    pub fn new(filename: &str) -> MetafileIsOwned<OWNED> {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            MetafileIsOwned(ffi::wxMetafile_new(filename))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MetafileIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MetafileIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MetafileIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMetafile_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MetafileIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMetafileDC
wx_class! { MetafileDC =
    MetafileDCIsOwned<true>(wxMetafileDC) impl
        MetafileDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> MetafileDCIsOwned<OWNED> {
    pub fn new(filename: &str) -> MetafileDCIsOwned<OWNED> {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            MetafileDCIsOwned(ffi::wxMetafileDC_new(filename))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MetafileDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: MetafileDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MetafileDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MetafileDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MetafileDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMetafileDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MetafileDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMiniFrame
wx_class! { MiniFrame =
    MiniFrameIsOwned<true>(wxMiniFrame) impl
        MiniFrameMethods,
        // FrameMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MiniFrameIsOwned<OWNED> {
    pub fn new_2step() -> MiniFrameIsOwned<OWNED> {
        unsafe { MiniFrameIsOwned(ffi::wxMiniFrame_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> MiniFrameIsOwned<OWNED> {
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
            MiniFrameIsOwned(ffi::wxMiniFrame_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MiniFrameIsOwned<OWNED>> for FrameIsOwned<OWNED> {
    fn from(o: MiniFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MiniFrameIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: MiniFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MiniFrameIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: MiniFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MiniFrameIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: MiniFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MiniFrameIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: MiniFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MiniFrameIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MiniFrameIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MiniFrameIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMiniFrame_CLASSINFO()) }
    }
}
// Mix-in(s) to wxMiniFrame
impl<const OWNED: bool> TrackableMethods for MiniFrameIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxMiniFrame_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> FrameMethods for MiniFrameIsOwned<OWNED> {
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
            ffi::wxMiniFrame_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
}
impl<const OWNED: bool> TopLevelWindowMethods for MiniFrameIsOwned<OWNED> {
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
            ffi::wxMiniFrame_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
}

// wxMirrorDC
wx_class! { MirrorDC =
    MirrorDCIsOwned<true>(wxMirrorDC) impl
        MirrorDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> MirrorDCIsOwned<OWNED> {
    pub fn new<D: DCMethods>(dc: &D, mirror: bool) -> MirrorDCIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            MirrorDCIsOwned(ffi::wxMirrorDC_new(dc, mirror))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MirrorDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: MirrorDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MirrorDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MirrorDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MirrorDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMirrorDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MirrorDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMouseCaptureChangedEvent
wx_class! { MouseCaptureChangedEvent =
    MouseCaptureChangedEventIsOwned<true>(wxMouseCaptureChangedEvent) impl
        MouseCaptureChangedEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MouseCaptureChangedEventIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(
        window_id: c_int,
        gained_capture: Option<&W>,
    ) -> MouseCaptureChangedEventIsOwned<OWNED> {
        unsafe {
            let gained_capture = match gained_capture {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MouseCaptureChangedEventIsOwned(ffi::wxMouseCaptureChangedEvent_new(
                window_id,
                gained_capture,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MouseCaptureChangedEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: MouseCaptureChangedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MouseCaptureChangedEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MouseCaptureChangedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MouseCaptureChangedEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMouseCaptureChangedEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MouseCaptureChangedEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMouseCaptureLostEvent
wx_class! { MouseCaptureLostEvent =
    MouseCaptureLostEventIsOwned<true>(wxMouseCaptureLostEvent) impl
        MouseCaptureLostEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MouseCaptureLostEventIsOwned<OWNED> {
    pub fn new(window_id: c_int) -> MouseCaptureLostEventIsOwned<OWNED> {
        unsafe { MouseCaptureLostEventIsOwned(ffi::wxMouseCaptureLostEvent_new(window_id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MouseCaptureLostEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: MouseCaptureLostEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MouseCaptureLostEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MouseCaptureLostEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MouseCaptureLostEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMouseCaptureLostEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MouseCaptureLostEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMouseEvent
wx_class! { MouseEvent =
    MouseEventIsOwned<true>(wxMouseEvent) impl
        MouseEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MouseEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxMouseEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MouseEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: MouseEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MouseEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MouseEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MouseEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMouseEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MouseEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
// Mix-in(s) to wxMouseEvent
impl<const OWNED: bool> MouseStateMethods for MouseEventIsOwned<OWNED> {
    fn as_mouse_state(&self) -> *mut c_void {
        unsafe { ffi::wxMouseEvent_AsMouseState(self.as_ptr()) }
    }
}
impl<const OWNED: bool> KeyboardStateMethods for MouseEventIsOwned<OWNED> {
    fn as_keyboard_state(&self) -> *mut c_void {
        unsafe { ffi::wxMouseEvent_AsMouseState(self.as_ptr()) }
    }
}

// wxMouseEventsManager
wx_class! { MouseEventsManager =
    MouseEventsManagerIsOwned<true>(wxMouseEventsManager) impl
        MouseEventsManagerMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MouseEventsManagerIsOwned<OWNED> {
    pub fn new() -> MouseEventsManagerIsOwned<OWNED> {
        unsafe { MouseEventsManagerIsOwned(ffi::wxMouseEventsManager_new()) }
    }
    pub fn new_with_window<W: WindowMethods>(win: Option<&W>) -> MouseEventsManagerIsOwned<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MouseEventsManagerIsOwned(ffi::wxMouseEventsManager_new1(win))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MouseEventsManagerIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: MouseEventsManagerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MouseEventsManagerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MouseEventsManagerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MouseEventsManagerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMouseEventsManager_CLASSINFO()) }
    }
}
// Mix-in(s) to wxMouseEventsManager
impl<const OWNED: bool> TrackableMethods for MouseEventsManagerIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxMouseEventsManager_AsTrackable(self.as_ptr()) }
    }
}

// wxMoveEvent
wx_class! { MoveEvent =
    MoveEventIsOwned<true>(wxMoveEvent) impl
        MoveEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> MoveEventIsOwned<OWNED> {
    pub fn new<P: PointMethods>(pt: &P, id: c_int) -> MoveEventIsOwned<OWNED> {
        unsafe {
            let pt = pt.as_ptr();
            MoveEventIsOwned(ffi::wxMoveEvent_new(pt, id))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MoveEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: MoveEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MoveEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MoveEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MoveEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMoveEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MoveEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxNativeFontInfo
wx_class! { NativeFontInfo =
    NativeFontInfoIsOwned<true>(wxNativeFontInfo) impl
        NativeFontInfoMethods
}
impl<const OWNED: bool> NativeFontInfoIsOwned<OWNED> {
    pub fn new() -> NativeFontInfoIsOwned<OWNED> {
        unsafe { NativeFontInfoIsOwned(ffi::wxNativeFontInfo_new()) }
    }
    pub fn new_with_nativefontinfo<N: NativeFontInfoMethods>(
        info: &N,
    ) -> NativeFontInfoIsOwned<OWNED> {
        unsafe {
            let info = info.as_ptr();
            NativeFontInfoIsOwned(ffi::wxNativeFontInfo_new1(info))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for NativeFontInfoIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxNativeFontInfo_delete(self.0) }
        }
    }
}

// wxNativeWindow
wx_class! { NativeWindow =
    NativeWindowIsOwned<true>(wxNativeWindow) impl
        NativeWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> NativeWindowIsOwned<OWNED> {
    pub fn new() -> NativeWindowIsOwned<OWNED> {
        unsafe { NativeWindowIsOwned(ffi::wxNativeWindow_new()) }
    }
    // NOT_SUPPORTED: fn wxNativeWindow1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<NativeWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: NativeWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NativeWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: NativeWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NativeWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: NativeWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NativeWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxNativeWindow_CLASSINFO()) }
    }
}
// Mix-in(s) to wxNativeWindow
impl<const OWNED: bool> TrackableMethods for NativeWindowIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxNativeWindow_AsTrackable(self.as_ptr()) }
    }
}

// wxNavigationKeyEvent
wx_class! { NavigationKeyEvent =
    NavigationKeyEventIsOwned<true>(wxNavigationKeyEvent) impl
        NavigationKeyEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> NavigationKeyEventIsOwned<OWNED> {
    //  ENUM: wxNavigationKeyEventFlags
    pub const IsBackward: c_int = 0x0000;
    pub const IsForward: c_int = 0x0001;
    pub const WinChange: c_int = 0x0002;
    pub const FromTab: c_int = 0x0004;

    pub fn new() -> NavigationKeyEventIsOwned<OWNED> {
        unsafe { NavigationKeyEventIsOwned(ffi::wxNavigationKeyEvent_new()) }
    }
    pub fn new_with_navigationkeyevent<N: NavigationKeyEventMethods>(
        event: &N,
    ) -> NavigationKeyEventIsOwned<OWNED> {
        unsafe {
            let event = event.as_ptr();
            NavigationKeyEventIsOwned(ffi::wxNavigationKeyEvent_new1(event))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<NavigationKeyEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: NavigationKeyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NavigationKeyEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: NavigationKeyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NavigationKeyEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxNavigationKeyEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for NavigationKeyEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxNonOwnedWindow
wx_class! { NonOwnedWindow =
    NonOwnedWindowIsOwned<true>(wxNonOwnedWindow) impl
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> NonOwnedWindowIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<NonOwnedWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: NonOwnedWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NonOwnedWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: NonOwnedWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NonOwnedWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: NonOwnedWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NonOwnedWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxNonOwnedWindow_CLASSINFO()) }
    }
}
// Mix-in(s) to wxNonOwnedWindow
impl<const OWNED: bool> TrackableMethods for NonOwnedWindowIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxNonOwnedWindow_AsTrackable(self.as_ptr()) }
    }
}

// wxNotebook
wx_class! { Notebook =
    NotebookIsOwned<true>(wxNotebook) impl
        NotebookMethods,
        // BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> NotebookIsOwned<OWNED> {
    pub fn new_2step() -> NotebookIsOwned<OWNED> {
        unsafe { NotebookIsOwned(ffi::wxNotebook_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> NotebookIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            NotebookIsOwned(ffi::wxNotebook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<NotebookIsOwned<OWNED>> for BookCtrlBaseIsOwned<OWNED> {
    fn from(o: NotebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotebookIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: NotebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotebookIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: NotebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotebookIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: NotebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotebookIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: NotebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NotebookIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxNotebook_CLASSINFO()) }
    }
}
// Mix-in(s) to wxNotebook
impl<const OWNED: bool> WithImagesMethods for NotebookIsOwned<OWNED> {
    fn as_with_images(&self) -> *mut c_void {
        unsafe { ffi::wxNotebook_AsWithImages(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for NotebookIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxNotebook_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> BookCtrlBaseMethods for NotebookIsOwned<OWNED> {
    // BLOCKED: fn Create()
}
impl<const OWNED: bool> WindowMethods for NotebookIsOwned<OWNED> {
    // BLOCKED: fn Create()
}

// wxNotificationMessage
wx_class! { NotificationMessage =
    NotificationMessageIsOwned<true>(wxNotificationMessage) impl
        NotificationMessageMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> NotificationMessageIsOwned<OWNED> {
    //  ENUM: @38
    pub const Timeout_Auto: c_int = -1;
    pub const Timeout_Never: c_int = 0;

    pub fn new() -> NotificationMessageIsOwned<OWNED> {
        unsafe { NotificationMessageIsOwned(ffi::wxNotificationMessage_new()) }
    }
    pub fn new_with_str<W: WindowMethods>(
        title: &str,
        message: &str,
        parent: Option<&W>,
        flags: c_int,
    ) -> NotificationMessageIsOwned<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            NotificationMessageIsOwned(ffi::wxNotificationMessage_new1(
                title, message, parent, flags,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<NotificationMessageIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: NotificationMessageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotificationMessageIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: NotificationMessageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NotificationMessageIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxNotificationMessage_CLASSINFO()) }
    }
}
// Mix-in(s) to wxNotificationMessage
impl<const OWNED: bool> TrackableMethods for NotificationMessageIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxNotificationMessage_AsTrackable(self.as_ptr()) }
    }
}

// wxNotifyEvent
wx_class! { NotifyEvent =
    NotifyEventIsOwned<true>(wxNotifyEvent) impl
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> NotifyEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxNotifyEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<NotifyEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: NotifyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotifyEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: NotifyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotifyEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: NotifyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NotifyEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxNotifyEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for NotifyEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxNumberEntryDialog
wx_class! { NumberEntryDialog =
    NumberEntryDialogIsOwned<true>(wxNumberEntryDialog) impl
        NumberEntryDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> NumberEntryDialogIsOwned<OWNED> {
    pub fn new_2step() -> NumberEntryDialogIsOwned<OWNED> {
        unsafe { NumberEntryDialogIsOwned(ffi::wxNumberEntryDialog_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        prompt: &str,
        caption: &str,
        value: c_long,
        min: c_long,
        max: c_long,
        pos: &P,
    ) -> NumberEntryDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let prompt = WxString::from(prompt);
            let prompt = prompt.as_ptr();
            let caption = WxString::from(caption);
            let caption = caption.as_ptr();
            let pos = pos.as_ptr();
            NumberEntryDialogIsOwned(ffi::wxNumberEntryDialog_new1(
                parent, message, prompt, caption, value, min, max, pos,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<NumberEntryDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: NumberEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NumberEntryDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: NumberEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NumberEntryDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: NumberEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NumberEntryDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: NumberEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NumberEntryDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: NumberEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NumberEntryDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: NumberEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NumberEntryDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxNumberEntryDialog_CLASSINFO()) }
    }
}
// Mix-in(s) to wxNumberEntryDialog
impl<const OWNED: bool> TrackableMethods for NumberEntryDialogIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxNumberEntryDialog_AsTrackable(self.as_ptr()) }
    }
}

// wxOverlay
wx_class! { Overlay =
    OverlayIsOwned<true>(wxOverlay) impl
        OverlayMethods
}
impl<const OWNED: bool> OverlayIsOwned<OWNED> {
    pub fn new() -> OverlayIsOwned<OWNED> {
        unsafe { OverlayIsOwned(ffi::wxOverlay_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for OverlayIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxOverlay_delete(self.0) }
        }
    }
}

// wxOwnerDrawnComboBox
wx_class! { OwnerDrawnComboBox =
    OwnerDrawnComboBoxIsOwned<true>(wxOwnerDrawnComboBox) impl
        OwnerDrawnComboBoxMethods,
        // ComboCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> OwnerDrawnComboBoxIsOwned<OWNED> {
    pub fn new_2step() -> OwnerDrawnComboBoxIsOwned<OWNED> {
        unsafe { OwnerDrawnComboBoxIsOwned(ffi::wxOwnerDrawnComboBox_new()) }
    }
    // NOT_SUPPORTED: fn wxOwnerDrawnComboBox1()
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> OwnerDrawnComboBoxIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            OwnerDrawnComboBoxIsOwned(ffi::wxOwnerDrawnComboBox_new2(
                parent, id, value, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<OwnerDrawnComboBoxIsOwned<OWNED>> for ComboCtrlIsOwned<OWNED> {
    fn from(o: OwnerDrawnComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<OwnerDrawnComboBoxIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: OwnerDrawnComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<OwnerDrawnComboBoxIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: OwnerDrawnComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<OwnerDrawnComboBoxIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: OwnerDrawnComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<OwnerDrawnComboBoxIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: OwnerDrawnComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for OwnerDrawnComboBoxIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxOwnerDrawnComboBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxOwnerDrawnComboBox
impl<const OWNED: bool> ItemContainerMethods for OwnerDrawnComboBoxIsOwned<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxOwnerDrawnComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for OwnerDrawnComboBoxIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxOwnerDrawnComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TextEntryMethods for OwnerDrawnComboBoxIsOwned<OWNED> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxOwnerDrawnComboBox_AsTextEntry(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for OwnerDrawnComboBoxIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxOwnerDrawnComboBox_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ComboCtrlMethods for OwnerDrawnComboBoxIsOwned<OWNED> {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
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
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxOwnerDrawnComboBox_Create(
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
}

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

// wxPanGestureEvent
wx_class! { PanGestureEvent =
    PanGestureEventIsOwned<true>(wxPanGestureEvent) impl
        PanGestureEventMethods,
        GestureEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> PanGestureEventIsOwned<OWNED> {
    pub fn new(winid: c_int) -> PanGestureEventIsOwned<OWNED> {
        unsafe { PanGestureEventIsOwned(ffi::wxPanGestureEvent_new(winid)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PanGestureEventIsOwned<OWNED>> for GestureEventIsOwned<OWNED> {
    fn from(o: PanGestureEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PanGestureEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: PanGestureEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PanGestureEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PanGestureEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PanGestureEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPanGestureEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PanGestureEventIsOwned<OWNED> {
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
// Mix-in(s) to wxPanel
impl<const OWNED: bool> TrackableMethods for PanelIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxPanel_AsTrackable(self.as_ptr()) }
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
// Mix-in(s) to wxPasswordEntryDialog
impl<const OWNED: bool> TrackableMethods for PasswordEntryDialogIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxPasswordEntryDialog_AsTrackable(self.as_ptr()) }
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
    pub fn new_with_peninfo<P: PenInfoMethods>(info: &P) -> PenIsOwned<OWNED> {
        unsafe {
            let info = info.as_ptr();
            PenIsOwned(ffi::wxPen_new1(info))
        }
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
// Mix-in(s) to wxPickerBase
impl<const OWNED: bool> TrackableMethods for PickerBaseIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxPickerBase_AsTrackable(self.as_ptr()) }
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
// Mix-in(s) to wxPopupTransientWindow
impl<const OWNED: bool> TrackableMethods for PopupTransientWindowIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxPopupTransientWindow_AsTrackable(self.as_ptr()) }
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
// Mix-in(s) to wxPopupWindow
impl<const OWNED: bool> TrackableMethods for PopupWindowIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxPopupWindow_AsTrackable(self.as_ptr()) }
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

// wxPressAndTapEvent
wx_class! { PressAndTapEvent =
    PressAndTapEventIsOwned<true>(wxPressAndTapEvent) impl
        PressAndTapEventMethods,
        GestureEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> PressAndTapEventIsOwned<OWNED> {
    pub fn new(windid: c_int) -> PressAndTapEventIsOwned<OWNED> {
        unsafe { PressAndTapEventIsOwned(ffi::wxPressAndTapEvent_new(windid)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PressAndTapEventIsOwned<OWNED>> for GestureEventIsOwned<OWNED> {
    fn from(o: PressAndTapEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PressAndTapEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: PressAndTapEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PressAndTapEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PressAndTapEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PressAndTapEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPressAndTapEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PressAndTapEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
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
// Mix-in(s) to wxPreviewControlBar
impl<const OWNED: bool> TrackableMethods for PreviewControlBarIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxPreviewControlBar_AsTrackable(self.as_ptr()) }
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
// Mix-in(s) to wxPreviewFrame
impl<const OWNED: bool> TrackableMethods for PreviewFrameIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxPreviewFrame_AsTrackable(self.as_ptr()) }
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
        // DialogMethods,
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
// Mix-in(s) to wxPropertySheetDialog
impl<const OWNED: bool> TrackableMethods for PropertySheetDialogIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxPropertySheetDialog_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> DialogMethods for PropertySheetDialogIsOwned<OWNED> {
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

// wxQuantize
wx_class! { Quantize =
    QuantizeIsOwned<true>(wxQuantize) impl
        QuantizeMethods,
        ObjectMethods
}
impl<const OWNED: bool> QuantizeIsOwned<OWNED> {
    pub fn new() -> QuantizeIsOwned<OWNED> {
        unsafe { QuantizeIsOwned(ffi::wxQuantize_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<QuantizeIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: QuantizeIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for QuantizeIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxQuantize_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for QuantizeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxQueryLayoutInfoEvent
wx_class! { QueryLayoutInfoEvent =
    QueryLayoutInfoEventIsOwned<true>(wxQueryLayoutInfoEvent) impl
        QueryLayoutInfoEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> QueryLayoutInfoEventIsOwned<OWNED> {
    pub fn new(id: c_int) -> QueryLayoutInfoEventIsOwned<OWNED> {
        unsafe { QueryLayoutInfoEventIsOwned(ffi::wxQueryLayoutInfoEvent_new(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<QueryLayoutInfoEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: QueryLayoutInfoEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<QueryLayoutInfoEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: QueryLayoutInfoEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for QueryLayoutInfoEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxQueryLayoutInfoEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for QueryLayoutInfoEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxRadioBox
wx_class! { RadioBox =
    RadioBoxIsOwned<true>(wxRadioBox) impl
        RadioBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> RadioBoxIsOwned<OWNED> {
    pub fn new_2step() -> RadioBoxIsOwned<OWNED> {
        unsafe { RadioBoxIsOwned(ffi::wxRadioBox_new()) }
    }
    // NOT_SUPPORTED: fn wxRadioBox1()
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
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
    ) -> RadioBoxIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            RadioBoxIsOwned(ffi::wxRadioBox_new2(
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
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<RadioBoxIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: RadioBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RadioBoxIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: RadioBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RadioBoxIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: RadioBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RadioBoxIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: RadioBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for RadioBoxIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxRadioBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxRadioBox
impl<const OWNED: bool> ItemContainerImmutableMethods for RadioBoxIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxRadioBox_AsItemContainerImmutable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for RadioBoxIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxRadioBox_AsTrackable(self.as_ptr()) }
    }
}

// wxRadioButton
wx_class! { RadioButton =
    RadioButtonIsOwned<true>(wxRadioButton) impl
        RadioButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> RadioButtonIsOwned<OWNED> {
    pub fn new_2step() -> RadioButtonIsOwned<OWNED> {
        unsafe { RadioButtonIsOwned(ffi::wxRadioButton_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> RadioButtonIsOwned<OWNED> {
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
            RadioButtonIsOwned(ffi::wxRadioButton_new1(
                parent, id, label, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<RadioButtonIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: RadioButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RadioButtonIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: RadioButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RadioButtonIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: RadioButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RadioButtonIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: RadioButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for RadioButtonIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxRadioButton_CLASSINFO()) }
    }
}
// Mix-in(s) to wxRadioButton
impl<const OWNED: bool> TrackableMethods for RadioButtonIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxRadioButton_AsTrackable(self.as_ptr()) }
    }
}

// wxRealPoint
wx_class! { RealPoint =
    RealPointIsOwned<true>(wxRealPoint) impl
        RealPointMethods
}
impl<const OWNED: bool> RealPointIsOwned<OWNED> {
    pub fn new() -> RealPointIsOwned<OWNED> {
        unsafe { RealPointIsOwned(ffi::wxRealPoint_new()) }
    }
    pub fn new_with_double(x: c_double, y: c_double) -> RealPointIsOwned<OWNED> {
        unsafe { RealPointIsOwned(ffi::wxRealPoint_new1(x, y)) }
    }
    pub fn new_with_point<P: PointMethods>(pt: &P) -> RealPointIsOwned<OWNED> {
        unsafe {
            let pt = pt.as_ptr();
            RealPointIsOwned(ffi::wxRealPoint_new2(pt))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for RealPointIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRealPoint_delete(self.0) }
        }
    }
}

// wxRearrangeCtrl
wx_class! { RearrangeCtrl =
    RearrangeCtrlIsOwned<true>(wxRearrangeCtrl) impl
        RearrangeCtrlMethods,
        PanelMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> RearrangeCtrlIsOwned<OWNED> {
    pub fn new_2step() -> RearrangeCtrlIsOwned<OWNED> {
        unsafe { RearrangeCtrlIsOwned(ffi::wxRearrangeCtrl_new()) }
    }
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayIntMethods,
        A2: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        order: &A,
        items: &A2,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> RearrangeCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let order = order.as_ptr();
            let items = items.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            RearrangeCtrlIsOwned(ffi::wxRearrangeCtrl_new1(
                parent, id, pos, size, order, items, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<RearrangeCtrlIsOwned<OWNED>> for PanelIsOwned<OWNED> {
    fn from(o: RearrangeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: RearrangeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: RearrangeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: RearrangeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for RearrangeCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxRearrangeCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxRearrangeCtrl
impl<const OWNED: bool> TrackableMethods for RearrangeCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxRearrangeCtrl_AsTrackable(self.as_ptr()) }
    }
}

// wxRearrangeDialog
wx_class! { RearrangeDialog =
    RearrangeDialogIsOwned<true>(wxRearrangeDialog) impl
        RearrangeDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> RearrangeDialogIsOwned<OWNED> {
    pub fn new_2step() -> RearrangeDialogIsOwned<OWNED> {
        unsafe { RearrangeDialogIsOwned(ffi::wxRearrangeDialog_new()) }
    }
    pub fn new<W: WindowMethods, A: ArrayIntMethods, A2: ArrayStringMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        title: &str,
        order: &A,
        items: &A2,
        pos: &P,
        name: &str,
    ) -> RearrangeDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let title = WxString::from(title);
            let title = title.as_ptr();
            let order = order.as_ptr();
            let items = items.as_ptr();
            let pos = pos.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            RearrangeDialogIsOwned(ffi::wxRearrangeDialog_new1(
                parent, message, title, order, items, pos, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<RearrangeDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: RearrangeDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: RearrangeDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: RearrangeDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: RearrangeDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: RearrangeDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: RearrangeDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for RearrangeDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxRearrangeDialog_CLASSINFO()) }
    }
}
// Mix-in(s) to wxRearrangeDialog
impl<const OWNED: bool> TrackableMethods for RearrangeDialogIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxRearrangeDialog_AsTrackable(self.as_ptr()) }
    }
}

// wxRearrangeList
wx_class! { RearrangeList =
    RearrangeListIsOwned<true>(wxRearrangeList) impl
        RearrangeListMethods,
        CheckListBoxMethods,
        ListBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> RearrangeListIsOwned<OWNED> {
    pub fn new_2step() -> RearrangeListIsOwned<OWNED> {
        unsafe { RearrangeListIsOwned(ffi::wxRearrangeList_new()) }
    }
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayIntMethods,
        A2: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        order: &A,
        items: &A2,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> RearrangeListIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let order = order.as_ptr();
            let items = items.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            RearrangeListIsOwned(ffi::wxRearrangeList_new1(
                parent, id, pos, size, order, items, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<RearrangeListIsOwned<OWNED>> for CheckListBoxIsOwned<OWNED> {
    fn from(o: RearrangeListIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeListIsOwned<OWNED>> for ListBoxIsOwned<OWNED> {
    fn from(o: RearrangeListIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeListIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: RearrangeListIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeListIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: RearrangeListIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeListIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: RearrangeListIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeListIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: RearrangeListIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for RearrangeListIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxRearrangeList_CLASSINFO()) }
    }
}
// Mix-in(s) to wxRearrangeList
impl<const OWNED: bool> ItemContainerMethods for RearrangeListIsOwned<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxRearrangeList_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for RearrangeListIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxRearrangeList_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for RearrangeListIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxRearrangeList_AsTrackable(self.as_ptr()) }
    }
}

// wxRect
wx_class! { Rect =
    RectIsOwned<true>(wxRect) impl
        RectMethods
}
impl<const OWNED: bool> RectIsOwned<OWNED> {
    pub fn new() -> RectIsOwned<OWNED> {
        unsafe { RectIsOwned(ffi::wxRect_new()) }
    }
    pub fn new_with_int(x: c_int, y: c_int, width: c_int, height: c_int) -> RectIsOwned<OWNED> {
        unsafe { RectIsOwned(ffi::wxRect_new1(x, y, width, height)) }
    }
    pub fn new_with_point_point<P: PointMethods, P2: PointMethods>(
        top_left: &P,
        bottom_right: &P2,
    ) -> RectIsOwned<OWNED> {
        unsafe {
            let top_left = top_left.as_ptr();
            let bottom_right = bottom_right.as_ptr();
            RectIsOwned(ffi::wxRect_new2(top_left, bottom_right))
        }
    }
    pub fn new_with_point_size<P: PointMethods, S: SizeMethods>(
        pos: &P,
        size: &S,
    ) -> RectIsOwned<OWNED> {
        unsafe {
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            RectIsOwned(ffi::wxRect_new3(pos, size))
        }
    }
    pub fn new_with_size<S: SizeMethods>(size: &S) -> RectIsOwned<OWNED> {
        unsafe {
            let size = size.as_ptr();
            RectIsOwned(ffi::wxRect_new4(size))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for RectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRect_delete(self.0) }
        }
    }
}

// wxRegion
wx_class! { Region =
    RegionIsOwned<true>(wxRegion) impl
        RegionMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> RegionIsOwned<OWNED> {
    pub fn new() -> RegionIsOwned<OWNED> {
        unsafe { RegionIsOwned(ffi::wxRegion_new()) }
    }
    pub fn new_with_coord(x: c_int, y: c_int, width: c_int, height: c_int) -> RegionIsOwned<OWNED> {
        unsafe { RegionIsOwned(ffi::wxRegion_new1(x, y, width, height)) }
    }
    pub fn new_with_point<P: PointMethods, P2: PointMethods>(
        top_left: &P,
        bottom_right: &P2,
    ) -> RegionIsOwned<OWNED> {
        unsafe {
            let top_left = top_left.as_ptr();
            let bottom_right = bottom_right.as_ptr();
            RegionIsOwned(ffi::wxRegion_new2(top_left, bottom_right))
        }
    }
    pub fn new_with_rect<R: RectMethods>(rect: &R) -> RegionIsOwned<OWNED> {
        unsafe {
            let rect = rect.as_ptr();
            RegionIsOwned(ffi::wxRegion_new3(rect))
        }
    }
    pub fn new_with_region<R: RegionMethods>(region: &R) -> RegionIsOwned<OWNED> {
        unsafe {
            let region = region.as_ptr();
            RegionIsOwned(ffi::wxRegion_new4(region))
        }
    }
    // NOT_SUPPORTED: fn wxRegion5()
    pub fn new_with_bitmap<B: BitmapMethods>(bmp: &B) -> RegionIsOwned<OWNED> {
        unsafe {
            let bmp = bmp.as_ptr();
            RegionIsOwned(ffi::wxRegion_new6(bmp))
        }
    }
    pub fn new_with_bitmap_colour<B: BitmapMethods, C: ColourMethods>(
        bmp: &B,
        trans_colour: &C,
        tolerance: c_int,
    ) -> RegionIsOwned<OWNED> {
        unsafe {
            let bmp = bmp.as_ptr();
            let trans_colour = trans_colour.as_ptr();
            RegionIsOwned(ffi::wxRegion_new7(bmp, trans_colour, tolerance))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<RegionIsOwned<OWNED>> for GDIObjectIsOwned<OWNED> {
    fn from(o: RegionIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RegionIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: RegionIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for RegionIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxRegion_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for RegionIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxRegionIterator
wx_class! { RegionIterator =
    RegionIteratorIsOwned<true>(wxRegionIterator) impl
        RegionIteratorMethods,
        ObjectMethods
}
impl<const OWNED: bool> RegionIteratorIsOwned<OWNED> {
    pub fn new() -> RegionIteratorIsOwned<OWNED> {
        unsafe { RegionIteratorIsOwned(ffi::wxRegionIterator_new()) }
    }
    pub fn new_with_region<R: RegionMethods>(region: &R) -> RegionIteratorIsOwned<OWNED> {
        unsafe {
            let region = region.as_ptr();
            RegionIteratorIsOwned(ffi::wxRegionIterator_new1(region))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<RegionIteratorIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: RegionIteratorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for RegionIteratorIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxRegionIterator_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for RegionIteratorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxRendererNative
wx_class! { RendererNative =
    RendererNativeIsOwned<true>(wxRendererNative) impl
        RendererNativeMethods
}
impl<const OWNED: bool> RendererNativeIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for RendererNativeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRendererNative_delete(self.0) }
        }
    }
}

// wxRichToolTip
wx_class! { RichToolTip =
    RichToolTipIsOwned<true>(wxRichToolTip) impl
        RichToolTipMethods
}
impl<const OWNED: bool> RichToolTipIsOwned<OWNED> {
    pub fn new(title: &str, message: &str) -> RichToolTipIsOwned<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            RichToolTipIsOwned(ffi::wxRichToolTip_new(title, message))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for RichToolTipIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRichToolTip_delete(self.0) }
        }
    }
}

// wxRotateGestureEvent
wx_class! { RotateGestureEvent =
    RotateGestureEventIsOwned<true>(wxRotateGestureEvent) impl
        RotateGestureEventMethods,
        GestureEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> RotateGestureEventIsOwned<OWNED> {
    pub fn new(windid: c_int) -> RotateGestureEventIsOwned<OWNED> {
        unsafe { RotateGestureEventIsOwned(ffi::wxRotateGestureEvent_new(windid)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<RotateGestureEventIsOwned<OWNED>> for GestureEventIsOwned<OWNED> {
    fn from(o: RotateGestureEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RotateGestureEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: RotateGestureEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RotateGestureEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: RotateGestureEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for RotateGestureEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxRotateGestureEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for RotateGestureEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSVGBitmapEmbedHandler
wx_class! { SVGBitmapEmbedHandler =
    SVGBitmapEmbedHandlerIsOwned<true>(wxSVGBitmapEmbedHandler) impl
        SVGBitmapEmbedHandlerMethods,
        SVGBitmapHandlerMethods
}
impl<const OWNED: bool> SVGBitmapEmbedHandlerIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SVGBitmapEmbedHandlerIsOwned<OWNED>>
    for SVGBitmapHandlerIsOwned<OWNED>
{
    fn from(o: SVGBitmapEmbedHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for SVGBitmapEmbedHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSVGBitmapEmbedHandler_delete(self.0) }
        }
    }
}

// wxSVGBitmapFileHandler
wx_class! { SVGBitmapFileHandler =
    SVGBitmapFileHandlerIsOwned<true>(wxSVGBitmapFileHandler) impl
        SVGBitmapFileHandlerMethods,
        SVGBitmapHandlerMethods
}
impl<const OWNED: bool> SVGBitmapFileHandlerIsOwned<OWNED> {
    pub fn new<F: FileNameMethods>(path: &F) -> SVGBitmapFileHandlerIsOwned<OWNED> {
        unsafe {
            let path = path.as_ptr();
            SVGBitmapFileHandlerIsOwned(ffi::wxSVGBitmapFileHandler_new(path))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SVGBitmapFileHandlerIsOwned<OWNED>>
    for SVGBitmapHandlerIsOwned<OWNED>
{
    fn from(o: SVGBitmapFileHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for SVGBitmapFileHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSVGBitmapFileHandler_delete(self.0) }
        }
    }
}

// wxSVGBitmapHandler
wx_class! { SVGBitmapHandler =
    SVGBitmapHandlerIsOwned<true>(wxSVGBitmapHandler) impl
        SVGBitmapHandlerMethods
}
impl<const OWNED: bool> SVGBitmapHandlerIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for SVGBitmapHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSVGBitmapHandler_delete(self.0) }
        }
    }
}

// wxSVGFileDC
wx_class! { SVGFileDC =
    SVGFileDCIsOwned<true>(wxSVGFileDC) impl
        SVGFileDCMethods,
        // DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> SVGFileDCIsOwned<OWNED> {
    pub fn new(
        filename: &str,
        width: c_int,
        height: c_int,
        dpi: c_double,
        title: &str,
    ) -> SVGFileDCIsOwned<OWNED> {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            let title = WxString::from(title);
            let title = title.as_ptr();
            SVGFileDCIsOwned(ffi::wxSVGFileDC_new(filename, width, height, dpi, title))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SVGFileDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: SVGFileDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SVGFileDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SVGFileDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SVGFileDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSVGFileDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SVGFileDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> DCMethods for SVGFileDCIsOwned<OWNED> {
    fn clear(&self) {
        unsafe { ffi::wxSVGFileDC_Clear(self.as_ptr()) }
    }
    fn destroy_clipping_region(&self) {
        unsafe { ffi::wxSVGFileDC_DestroyClippingRegion(self.as_ptr()) }
    }
    fn cross_hair_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxSVGFileDC_CrossHair(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn FloodFill()
    fn get_pixel<C: ColourMethods>(&self, x: c_int, y: c_int, colour: Option<&C>) -> bool {
        unsafe {
            let colour = match colour {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSVGFileDC_GetPixel(self.as_ptr(), x, y, colour)
        }
    }
    fn set_palette<P: PaletteMethods>(&self, palette: &P) {
        unsafe {
            let palette = palette.as_ptr();
            ffi::wxSVGFileDC_SetPalette(self.as_ptr(), palette)
        }
    }
    fn get_depth(&self) -> c_int {
        unsafe { ffi::wxSVGFileDC_GetDepth(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetLogicalFunction()
    // NOT_SUPPORTED: fn GetLogicalFunction()
    fn start_doc(&self, message: &str) -> bool {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxSVGFileDC_StartDoc(self.as_ptr(), message)
        }
    }
    fn end_doc(&self) {
        unsafe { ffi::wxSVGFileDC_EndDoc(self.as_ptr()) }
    }
    fn start_page(&self) {
        unsafe { ffi::wxSVGFileDC_StartPage(self.as_ptr()) }
    }
    fn end_page(&self) {
        unsafe { ffi::wxSVGFileDC_EndPage(self.as_ptr()) }
    }
}

// wxSashEvent
wx_class! { SashEvent =
    SashEventIsOwned<true>(wxSashEvent) impl
        SashEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SashEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxSashEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SashEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: SashEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: SashEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SashEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SashEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSashEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SashEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSashLayoutWindow
wx_class! { SashLayoutWindow =
    SashLayoutWindowIsOwned<true>(wxSashLayoutWindow) impl
        SashLayoutWindowMethods,
        SashWindowMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SashLayoutWindowIsOwned<OWNED> {
    pub fn new_2step() -> SashLayoutWindowIsOwned<OWNED> {
        unsafe { SashLayoutWindowIsOwned(ffi::wxSashLayoutWindow_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> SashLayoutWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SashLayoutWindowIsOwned(ffi::wxSashLayoutWindow_new1(
                parent, id, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SashLayoutWindowIsOwned<OWNED>> for SashWindowIsOwned<OWNED> {
    fn from(o: SashLayoutWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashLayoutWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SashLayoutWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashLayoutWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SashLayoutWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashLayoutWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SashLayoutWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SashLayoutWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSashLayoutWindow_CLASSINFO()) }
    }
}
// Mix-in(s) to wxSashLayoutWindow
impl<const OWNED: bool> TrackableMethods for SashLayoutWindowIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxSashLayoutWindow_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> WindowMethods for SashLayoutWindowIsOwned<OWNED> {
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
            ffi::wxSashLayoutWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxSashWindow
wx_class! { SashWindow =
    SashWindowIsOwned<true>(wxSashWindow) impl
        SashWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SashWindowIsOwned<OWNED> {
    pub fn new_2step() -> SashWindowIsOwned<OWNED> {
        unsafe { SashWindowIsOwned(ffi::wxSashWindow_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> SashWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SashWindowIsOwned(ffi::wxSashWindow_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SashWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SashWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SashWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SashWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SashWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSashWindow_CLASSINFO()) }
    }
}
// Mix-in(s) to wxSashWindow
impl<const OWNED: bool> TrackableMethods for SashWindowIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxSashWindow_AsTrackable(self.as_ptr()) }
    }
}

// wxScreenDC
wx_class! { ScreenDC =
    ScreenDCIsOwned<true>(wxScreenDC) impl
        ScreenDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> ScreenDCIsOwned<OWNED> {
    pub fn new() -> ScreenDCIsOwned<OWNED> {
        unsafe { ScreenDCIsOwned(ffi::wxScreenDC_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ScreenDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: ScreenDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScreenDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ScreenDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ScreenDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxScreenDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ScreenDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxScrollBar
wx_class! { ScrollBar =
    ScrollBarIsOwned<true>(wxScrollBar) impl
        ScrollBarMethods,
        // ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ScrollBarIsOwned<OWNED> {
    pub fn new_2step() -> ScrollBarIsOwned<OWNED> {
        unsafe { ScrollBarIsOwned(ffi::wxScrollBar_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ScrollBarIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ScrollBarIsOwned(ffi::wxScrollBar_new1(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ScrollBarIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ScrollBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollBarIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ScrollBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollBarIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ScrollBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollBarIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ScrollBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ScrollBarIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxScrollBar_CLASSINFO()) }
    }
}
// Mix-in(s) to wxScrollBar
impl<const OWNED: bool> TrackableMethods for ScrollBarIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxScrollBar_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ControlMethods for ScrollBarIsOwned<OWNED> {
    fn create_validator<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
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
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxScrollBar_Create(self.as_ptr(), parent, id, pos, size, style, validator, name)
        }
    }
}

// wxScrollEvent
wx_class! { ScrollEvent =
    ScrollEventIsOwned<true>(wxScrollEvent) impl
        ScrollEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ScrollEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxScrollEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ScrollEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: ScrollEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ScrollEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ScrollEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ScrollEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxScrollEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ScrollEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxScrollWinEvent
wx_class! { ScrollWinEvent =
    ScrollWinEventIsOwned<true>(wxScrollWinEvent) impl
        ScrollWinEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ScrollWinEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxScrollWinEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ScrollWinEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ScrollWinEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollWinEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ScrollWinEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ScrollWinEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxScrollWinEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ScrollWinEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSearchCtrl
wx_class! { SearchCtrl =
    SearchCtrlIsOwned<true>(wxSearchCtrl) impl
        SearchCtrlMethods,
        // TextCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SearchCtrlIsOwned<OWNED> {
    pub fn new_2step() -> SearchCtrlIsOwned<OWNED> {
        unsafe { SearchCtrlIsOwned(ffi::wxSearchCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> SearchCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SearchCtrlIsOwned(ffi::wxSearchCtrl_new1(
                parent, id, value, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SearchCtrlIsOwned<OWNED>> for TextCtrlIsOwned<OWNED> {
    fn from(o: SearchCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SearchCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: SearchCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SearchCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SearchCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SearchCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SearchCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SearchCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SearchCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SearchCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSearchCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxSearchCtrl
impl<const OWNED: bool> TextEntryMethods for SearchCtrlIsOwned<OWNED> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxSearchCtrl_AsTextEntry(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for SearchCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxSearchCtrl_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TextCtrlMethods for SearchCtrlIsOwned<OWNED> {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
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
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSearchCtrl_Create(
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
}

// wxSetCursorEvent
wx_class! { SetCursorEvent =
    SetCursorEventIsOwned<true>(wxSetCursorEvent) impl
        SetCursorEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SetCursorEventIsOwned<OWNED> {
    pub fn new(x: c_int, y: c_int) -> SetCursorEventIsOwned<OWNED> {
        unsafe { SetCursorEventIsOwned(ffi::wxSetCursorEvent_new(x, y)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SetCursorEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: SetCursorEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SetCursorEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SetCursorEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SetCursorEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSetCursorEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SetCursorEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSettableHeaderColumn
wx_class! { SettableHeaderColumn =
    SettableHeaderColumnIsOwned<true>(wxSettableHeaderColumn) impl
        SettableHeaderColumnMethods,
        HeaderColumnMethods
}
impl<const OWNED: bool> SettableHeaderColumnIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SettableHeaderColumnIsOwned<OWNED>> for HeaderColumnIsOwned<OWNED> {
    fn from(o: SettableHeaderColumnIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for SettableHeaderColumnIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSettableHeaderColumn_delete(self.0) }
        }
    }
}

// wxShowEvent
wx_class! { ShowEvent =
    ShowEventIsOwned<true>(wxShowEvent) impl
        ShowEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ShowEventIsOwned<OWNED> {
    pub fn new(winid: c_int, show: bool) -> ShowEventIsOwned<OWNED> {
        unsafe { ShowEventIsOwned(ffi::wxShowEvent_new(winid, show)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ShowEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ShowEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ShowEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ShowEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ShowEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxShowEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ShowEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSimpleHelpProvider
wx_class! { SimpleHelpProvider =
    SimpleHelpProviderIsOwned<true>(wxSimpleHelpProvider) impl
        SimpleHelpProviderMethods,
        HelpProviderMethods
}
impl<const OWNED: bool> SimpleHelpProviderIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SimpleHelpProviderIsOwned<OWNED>> for HelpProviderIsOwned<OWNED> {
    fn from(o: SimpleHelpProviderIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for SimpleHelpProviderIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSimpleHelpProvider_delete(self.0) }
        }
    }
}

// wxSimplebook
wx_class! { Simplebook =
    SimplebookIsOwned<true>(wxSimplebook) impl
        SimplebookMethods,
        // BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SimplebookIsOwned<OWNED> {
    pub fn new_2step() -> SimplebookIsOwned<OWNED> {
        unsafe { SimplebookIsOwned(ffi::wxSimplebook_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> SimplebookIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SimplebookIsOwned(ffi::wxSimplebook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SimplebookIsOwned<OWNED>> for BookCtrlBaseIsOwned<OWNED> {
    fn from(o: SimplebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SimplebookIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: SimplebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SimplebookIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SimplebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SimplebookIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SimplebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SimplebookIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SimplebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SimplebookIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSimplebook_CLASSINFO()) }
    }
}
// Mix-in(s) to wxSimplebook
impl<const OWNED: bool> WithImagesMethods for SimplebookIsOwned<OWNED> {
    fn as_with_images(&self) -> *mut c_void {
        unsafe { ffi::wxSimplebook_AsWithImages(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for SimplebookIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxSimplebook_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> BookCtrlBaseMethods for SimplebookIsOwned<OWNED> {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
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
            ffi::wxSimplebook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}
impl<const OWNED: bool> WindowMethods for SimplebookIsOwned<OWNED> {
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
            ffi::wxSimplebook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxSize
wx_class! { Size =
    SizeIsOwned<true>(wxSize) impl
        SizeMethods
}
impl<const OWNED: bool> SizeIsOwned<OWNED> {
    pub fn new() -> SizeIsOwned<OWNED> {
        unsafe { SizeIsOwned(ffi::wxSize_new()) }
    }
    pub fn new_with_int(width: c_int, height: c_int) -> SizeIsOwned<OWNED> {
        unsafe { SizeIsOwned(ffi::wxSize_new1(width, height)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for SizeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSize_delete(self.0) }
        }
    }
}

// wxSizeEvent
wx_class! { SizeEvent =
    SizeEventIsOwned<true>(wxSizeEvent) impl
        SizeEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SizeEventIsOwned<OWNED> {
    pub fn new<S: SizeMethods>(sz: &S, id: c_int) -> SizeEventIsOwned<OWNED> {
        unsafe {
            let sz = sz.as_ptr();
            SizeEventIsOwned(ffi::wxSizeEvent_new(sz, id))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SizeEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: SizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SizeEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SizeEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSizeEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SizeEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSizer
wx_class! { Sizer =
    SizerIsOwned<true>(wxSizer) impl
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SizerIsOwned<OWNED> {
    // BLOCKED: fn wxSizer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SizerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SizerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSizer_CLASSINFO()) }
    }
}

// wxSizerFlags
wx_class! { SizerFlags =
    SizerFlagsIsOwned<true>(wxSizerFlags) impl
        SizerFlagsMethods
}
impl<const OWNED: bool> SizerFlagsIsOwned<OWNED> {
    pub fn new(proportion: c_int) -> SizerFlagsIsOwned<OWNED> {
        unsafe { SizerFlagsIsOwned(ffi::wxSizerFlags_new(proportion)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for SizerFlagsIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSizerFlags_delete(self.0) }
        }
    }
}

// wxSizerItem
wx_class! { SizerItem =
    SizerItemIsOwned<true>(wxSizerItem) impl
        SizerItemMethods,
        ObjectMethods
}
impl<const OWNED: bool> SizerItemIsOwned<OWNED> {
    pub fn new_with_int<O: ObjectMethods>(
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> SizerItemIsOwned<OWNED> {
        unsafe {
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItemIsOwned(ffi::wxSizerItem_new(
                width, height, proportion, flag, border, user_data,
            ))
        }
    }
    pub fn new_with_window_sizerflags<W: WindowMethods, S: SizerFlagsMethods>(
        window: Option<&W>,
        flags: &S,
    ) -> SizerItemIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            SizerItemIsOwned(ffi::wxSizerItem_new1(window, flags))
        }
    }
    pub fn new_with_window_int<W: WindowMethods, O: ObjectMethods>(
        window: Option<&W>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> SizerItemIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItemIsOwned(ffi::wxSizerItem_new2(
                window, proportion, flag, border, user_data,
            ))
        }
    }
    pub fn new_with_sizer_sizerflags<S: SizerMethods, S2: SizerFlagsMethods>(
        sizer: Option<&S>,
        flags: &S2,
    ) -> SizerItemIsOwned<OWNED> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            SizerItemIsOwned(ffi::wxSizerItem_new3(sizer, flags))
        }
    }
    pub fn new_with_sizer_int<S: SizerMethods, O: ObjectMethods>(
        sizer: Option<&S>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> SizerItemIsOwned<OWNED> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItemIsOwned(ffi::wxSizerItem_new4(
                sizer, proportion, flag, border, user_data,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SizerItemIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SizerItemIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SizerItemIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSizerItem_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SizerItemIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSlider
wx_class! { Slider =
    SliderIsOwned<true>(wxSlider) impl
        SliderMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SliderIsOwned<OWNED> {
    pub fn new_2step() -> SliderIsOwned<OWNED> {
        unsafe { SliderIsOwned(ffi::wxSlider_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        value: c_int,
        min_value: c_int,
        max_value: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> SliderIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SliderIsOwned(ffi::wxSlider_new1(
                parent, id, value, min_value, max_value, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SliderIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: SliderIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SliderIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SliderIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SliderIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SliderIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SliderIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SliderIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SliderIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSlider_CLASSINFO()) }
    }
}
// Mix-in(s) to wxSlider
impl<const OWNED: bool> TrackableMethods for SliderIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxSlider_AsTrackable(self.as_ptr()) }
    }
}

// wxSound
wx_class! { Sound =
    SoundIsOwned<true>(wxSound) impl
        SoundMethods,
        ObjectMethods
}
impl<const OWNED: bool> SoundIsOwned<OWNED> {
    pub fn new() -> SoundIsOwned<OWNED> {
        unsafe { SoundIsOwned(ffi::wxSound_new()) }
    }
    pub fn new_with_str(file_name: &str, is_resource: bool) -> SoundIsOwned<OWNED> {
        unsafe {
            let file_name = WxString::from(file_name);
            let file_name = file_name.as_ptr();
            SoundIsOwned(ffi::wxSound_new1(file_name, is_resource))
        }
    }
    pub fn new_with_sz(size: usize, data: *const c_void) -> SoundIsOwned<OWNED> {
        unsafe { SoundIsOwned(ffi::wxSound_new2(size, data)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SoundIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SoundIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SoundIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSound_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SoundIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSpinButton
wx_class! { SpinButton =
    SpinButtonIsOwned<true>(wxSpinButton) impl
        SpinButtonMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SpinButtonIsOwned<OWNED> {
    pub fn new_2step() -> SpinButtonIsOwned<OWNED> {
        unsafe { SpinButtonIsOwned(ffi::wxSpinButton_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> SpinButtonIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SpinButtonIsOwned(ffi::wxSpinButton_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SpinButtonIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: SpinButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinButtonIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SpinButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinButtonIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SpinButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinButtonIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SpinButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SpinButtonIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSpinButton_CLASSINFO()) }
    }
}
// Mix-in(s) to wxSpinButton
impl<const OWNED: bool> TrackableMethods for SpinButtonIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxSpinButton_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> WindowMethods for SpinButtonIsOwned<OWNED> {
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
            ffi::wxSpinButton_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxSpinCtrl
wx_class! { SpinCtrl =
    SpinCtrlIsOwned<true>(wxSpinCtrl) impl
        SpinCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SpinCtrlIsOwned<OWNED> {
    pub fn new_2step() -> SpinCtrlIsOwned<OWNED> {
        unsafe { SpinCtrlIsOwned(ffi::wxSpinCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        min: c_int,
        max: c_int,
        initial: c_int,
        name: &str,
    ) -> SpinCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SpinCtrlIsOwned(ffi::wxSpinCtrl_new1(
                parent, id, value, pos, size, style, min, max, initial, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SpinCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: SpinCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SpinCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SpinCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SpinCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SpinCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSpinCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxSpinCtrl
impl<const OWNED: bool> TrackableMethods for SpinCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxSpinCtrl_AsTrackable(self.as_ptr()) }
    }
}

// wxSpinCtrlDouble
wx_class! { SpinCtrlDouble =
    SpinCtrlDoubleIsOwned<true>(wxSpinCtrlDouble) impl
        SpinCtrlDoubleMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SpinCtrlDoubleIsOwned<OWNED> {
    pub fn new_2step() -> SpinCtrlDoubleIsOwned<OWNED> {
        unsafe { SpinCtrlDoubleIsOwned(ffi::wxSpinCtrlDouble_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        min: c_double,
        max: c_double,
        initial: c_double,
        inc: c_double,
        name: &str,
    ) -> SpinCtrlDoubleIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SpinCtrlDoubleIsOwned(ffi::wxSpinCtrlDouble_new1(
                parent, id, value, pos, size, style, min, max, initial, inc, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SpinCtrlDoubleIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: SpinCtrlDoubleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlDoubleIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SpinCtrlDoubleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlDoubleIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SpinCtrlDoubleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlDoubleIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SpinCtrlDoubleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SpinCtrlDoubleIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSpinCtrlDouble_CLASSINFO()) }
    }
}
// Mix-in(s) to wxSpinCtrlDouble
impl<const OWNED: bool> TrackableMethods for SpinCtrlDoubleIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxSpinCtrlDouble_AsTrackable(self.as_ptr()) }
    }
}

// wxSpinDoubleEvent
wx_class! { SpinDoubleEvent =
    SpinDoubleEventIsOwned<true>(wxSpinDoubleEvent) impl
        SpinDoubleEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SpinDoubleEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxSpinDoubleEvent()
    pub fn new<S: SpinDoubleEventMethods>(event: &S) -> SpinDoubleEventIsOwned<OWNED> {
        unsafe {
            let event = event.as_ptr();
            SpinDoubleEventIsOwned(ffi::wxSpinDoubleEvent_new1(event))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SpinDoubleEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: SpinDoubleEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinDoubleEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: SpinDoubleEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinDoubleEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: SpinDoubleEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinDoubleEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SpinDoubleEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SpinDoubleEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSpinDoubleEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SpinDoubleEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSpinEvent
wx_class! { SpinEvent =
    SpinEventIsOwned<true>(wxSpinEvent) impl
        SpinEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SpinEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxSpinEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SpinEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: SpinEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: SpinEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: SpinEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SpinEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SpinEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSpinEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SpinEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSplashScreen
wx_class! { SplashScreen =
    SplashScreenIsOwned<true>(wxSplashScreen) impl
        SplashScreenMethods,
        FrameMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SplashScreenIsOwned<OWNED> {
    pub fn new<B: BitmapMethods, W: WindowMethods, P: PointMethods, S: SizeMethods>(
        bitmap: &B,
        splash_style: c_long,
        milliseconds: c_int,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
    ) -> SplashScreenIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            SplashScreenIsOwned(ffi::wxSplashScreen_new(
                bitmap,
                splash_style,
                milliseconds,
                parent,
                id,
                pos,
                size,
                style,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SplashScreenIsOwned<OWNED>> for FrameIsOwned<OWNED> {
    fn from(o: SplashScreenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplashScreenIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: SplashScreenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplashScreenIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: SplashScreenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplashScreenIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SplashScreenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplashScreenIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SplashScreenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplashScreenIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SplashScreenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SplashScreenIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSplashScreen_CLASSINFO()) }
    }
}
// Mix-in(s) to wxSplashScreen
impl<const OWNED: bool> TrackableMethods for SplashScreenIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxSplashScreen_AsTrackable(self.as_ptr()) }
    }
}

// wxSplitterEvent
wx_class! { SplitterEvent =
    SplitterEventIsOwned<true>(wxSplitterEvent) impl
        SplitterEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SplitterEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxSplitterEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SplitterEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: SplitterEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplitterEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: SplitterEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplitterEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: SplitterEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplitterEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SplitterEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SplitterEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSplitterEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SplitterEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSplitterWindow
wx_class! { SplitterWindow =
    SplitterWindowIsOwned<true>(wxSplitterWindow) impl
        SplitterWindowMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SplitterWindowIsOwned<OWNED> {
    pub fn new_2step() -> SplitterWindowIsOwned<OWNED> {
        unsafe { SplitterWindowIsOwned(ffi::wxSplitterWindow_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> SplitterWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SplitterWindowIsOwned(ffi::wxSplitterWindow_new1(
                parent, id, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SplitterWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SplitterWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplitterWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SplitterWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplitterWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SplitterWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SplitterWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSplitterWindow_CLASSINFO()) }
    }
}
// Mix-in(s) to wxSplitterWindow
impl<const OWNED: bool> TrackableMethods for SplitterWindowIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxSplitterWindow_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> WindowMethods for SplitterWindowIsOwned<OWNED> {
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        point: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let point = point.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSplitterWindow_Create(self.as_ptr(), parent, id, point, size, style, name)
        }
    }
}

// wxStaticBitmap
wx_class! { StaticBitmap =
    StaticBitmapIsOwned<true>(wxStaticBitmap) impl
        StaticBitmapMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticBitmapIsOwned<OWNED> {
    //  ENUM: ScaleMode
    pub const Scale_None: c_int = 0;
    pub const Scale_Fill: c_int = 0 + 1;
    pub const Scale_AspectFit: c_int = 0 + 2;
    pub const Scale_AspectFill: c_int = 0 + 3;

    pub fn new_2step() -> StaticBitmapIsOwned<OWNED> {
        unsafe { StaticBitmapIsOwned(ffi::wxStaticBitmap_new()) }
    }
    pub fn new<W: WindowMethods, B: BitmapBundleMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &B,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> StaticBitmapIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            StaticBitmapIsOwned(ffi::wxStaticBitmap_new1(
                parent, id, label, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<StaticBitmapIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: StaticBitmapIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBitmapIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: StaticBitmapIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBitmapIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: StaticBitmapIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBitmapIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: StaticBitmapIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StaticBitmapIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxStaticBitmap_CLASSINFO()) }
    }
}
// Mix-in(s) to wxStaticBitmap
impl<const OWNED: bool> TrackableMethods for StaticBitmapIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxStaticBitmap_AsTrackable(self.as_ptr()) }
    }
}

// wxStaticBox
wx_class! { StaticBox =
    StaticBoxIsOwned<true>(wxStaticBox) impl
        StaticBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticBoxIsOwned<OWNED> {
    pub fn new_2step() -> StaticBoxIsOwned<OWNED> {
        unsafe { StaticBoxIsOwned(ffi::wxStaticBox_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> StaticBoxIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            StaticBoxIsOwned(ffi::wxStaticBox_new1(
                parent, id, label, pos, size, style, name,
            ))
        }
    }
    // BLOCKED: fn wxStaticBox2()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<StaticBoxIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: StaticBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBoxIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: StaticBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBoxIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: StaticBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBoxIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: StaticBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StaticBoxIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxStaticBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxStaticBox
impl<const OWNED: bool> TrackableMethods for StaticBoxIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxStaticBox_AsTrackable(self.as_ptr()) }
    }
}

// wxStaticBoxSizer
wx_class! { StaticBoxSizer =
    StaticBoxSizerIsOwned<true>(wxStaticBoxSizer) impl
        StaticBoxSizerMethods,
        BoxSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticBoxSizerIsOwned<OWNED> {
    pub fn new_with_staticbox<S: StaticBoxMethods>(
        box_: Option<&S>,
        orient: c_int,
    ) -> StaticBoxSizerIsOwned<OWNED> {
        unsafe {
            let box_ = match box_ {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            StaticBoxSizerIsOwned(ffi::wxStaticBoxSizer_new(box_, orient))
        }
    }
    pub fn new_with_int<W: WindowMethods>(
        orient: c_int,
        parent: Option<&W>,
        label: &str,
    ) -> StaticBoxSizerIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            StaticBoxSizerIsOwned(ffi::wxStaticBoxSizer_new1(orient, parent, label))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<StaticBoxSizerIsOwned<OWNED>> for BoxSizerIsOwned<OWNED> {
    fn from(o: StaticBoxSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBoxSizerIsOwned<OWNED>> for SizerIsOwned<OWNED> {
    fn from(o: StaticBoxSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBoxSizerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: StaticBoxSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StaticBoxSizerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxStaticBoxSizer_CLASSINFO()) }
    }
}

// wxStaticLine
wx_class! { StaticLine =
    StaticLineIsOwned<true>(wxStaticLine) impl
        StaticLineMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticLineIsOwned<OWNED> {
    pub fn new_2step() -> StaticLineIsOwned<OWNED> {
        unsafe { StaticLineIsOwned(ffi::wxStaticLine_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> StaticLineIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            StaticLineIsOwned(ffi::wxStaticLine_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<StaticLineIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: StaticLineIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticLineIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: StaticLineIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticLineIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: StaticLineIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticLineIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: StaticLineIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StaticLineIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxStaticLine_CLASSINFO()) }
    }
}
// Mix-in(s) to wxStaticLine
impl<const OWNED: bool> TrackableMethods for StaticLineIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxStaticLine_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> WindowMethods for StaticLineIsOwned<OWNED> {
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
            ffi::wxStaticLine_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxStaticText
wx_class! { StaticText =
    StaticTextIsOwned<true>(wxStaticText) impl
        StaticTextMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticTextIsOwned<OWNED> {
    pub fn new_2step() -> StaticTextIsOwned<OWNED> {
        unsafe { StaticTextIsOwned(ffi::wxStaticText_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> StaticTextIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            StaticTextIsOwned(ffi::wxStaticText_new1(
                parent, id, label, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<StaticTextIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: StaticTextIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticTextIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: StaticTextIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticTextIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: StaticTextIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticTextIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: StaticTextIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StaticTextIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxStaticText_CLASSINFO()) }
    }
}
// Mix-in(s) to wxStaticText
impl<const OWNED: bool> TrackableMethods for StaticTextIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxStaticText_AsTrackable(self.as_ptr()) }
    }
}

// wxStatusBar
wx_class! { StatusBar =
    StatusBarIsOwned<true>(wxStatusBar) impl
        StatusBarMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StatusBarIsOwned<OWNED> {
    pub fn new_2step() -> StatusBarIsOwned<OWNED> {
        unsafe { StatusBarIsOwned(ffi::wxStatusBar_new()) }
    }
    pub fn new<W: WindowMethods>(
        parent: Option<&W>,
        id: c_int,
        style: c_long,
        name: &str,
    ) -> StatusBarIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let name = WxString::from(name);
            let name = name.as_ptr();
            StatusBarIsOwned(ffi::wxStatusBar_new1(parent, id, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<StatusBarIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: StatusBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StatusBarIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: StatusBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StatusBarIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: StatusBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StatusBarIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: StatusBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StatusBarIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxStatusBar_CLASSINFO()) }
    }
}
// Mix-in(s) to wxStatusBar
impl<const OWNED: bool> TrackableMethods for StatusBarIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxStatusBar_AsTrackable(self.as_ptr()) }
    }
}

// wxStatusBarPane
wx_class! { StatusBarPane =
    StatusBarPaneIsOwned<true>(wxStatusBarPane) impl
        StatusBarPaneMethods
}
impl<const OWNED: bool> StatusBarPaneIsOwned<OWNED> {
    pub fn new(style: c_int, width: c_int) -> StatusBarPaneIsOwned<OWNED> {
        unsafe { StatusBarPaneIsOwned(ffi::wxStatusBarPane_new(style, width)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for StatusBarPaneIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxStatusBarPane_delete(self.0) }
        }
    }
}

// wxStdDialogButtonSizer
wx_class! { StdDialogButtonSizer =
    StdDialogButtonSizerIsOwned<true>(wxStdDialogButtonSizer) impl
        StdDialogButtonSizerMethods,
        BoxSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StdDialogButtonSizerIsOwned<OWNED> {
    pub fn new() -> StdDialogButtonSizerIsOwned<OWNED> {
        unsafe { StdDialogButtonSizerIsOwned(ffi::wxStdDialogButtonSizer_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<StdDialogButtonSizerIsOwned<OWNED>> for BoxSizerIsOwned<OWNED> {
    fn from(o: StdDialogButtonSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StdDialogButtonSizerIsOwned<OWNED>> for SizerIsOwned<OWNED> {
    fn from(o: StdDialogButtonSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StdDialogButtonSizerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: StdDialogButtonSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StdDialogButtonSizerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxStdDialogButtonSizer_CLASSINFO()) }
    }
}

// wxStockPreferencesPage
wx_class! { StockPreferencesPage =
    StockPreferencesPageIsOwned<true>(wxStockPreferencesPage) impl
        StockPreferencesPageMethods,
        PreferencesPageMethods
}
impl<const OWNED: bool> StockPreferencesPageIsOwned<OWNED> {
    //  ENUM: Kind
    pub const Kind_General: c_int = 0;
    pub const Kind_Advanced: c_int = 0 + 1;

    // NOT_SUPPORTED: fn wxStockPreferencesPage()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<StockPreferencesPageIsOwned<OWNED>> for PreferencesPageIsOwned<OWNED> {
    fn from(o: StockPreferencesPageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for StockPreferencesPageIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxStockPreferencesPage_delete(self.0) }
        }
    }
}

// wxStreamToTextRedirector
wx_class! { StreamToTextRedirector =
    StreamToTextRedirectorIsOwned<true>(wxStreamToTextRedirector) impl
        StreamToTextRedirectorMethods
}
impl<const OWNED: bool> StreamToTextRedirectorIsOwned<OWNED> {
    pub fn new<T: TextCtrlMethods>(
        text: Option<&T>,
        ostr: *mut c_void,
    ) -> StreamToTextRedirectorIsOwned<OWNED> {
        unsafe {
            let text = match text {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            StreamToTextRedirectorIsOwned(ffi::wxStreamToTextRedirector_new(text, ostr))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for StreamToTextRedirectorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxStreamToTextRedirector_delete(self.0) }
        }
    }
}

// wxSysColourChangedEvent
wx_class! { SysColourChangedEvent =
    SysColourChangedEventIsOwned<true>(wxSysColourChangedEvent) impl
        SysColourChangedEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SysColourChangedEventIsOwned<OWNED> {
    pub fn new() -> SysColourChangedEventIsOwned<OWNED> {
        unsafe { SysColourChangedEventIsOwned(ffi::wxSysColourChangedEvent_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SysColourChangedEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: SysColourChangedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SysColourChangedEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SysColourChangedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SysColourChangedEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSysColourChangedEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SysColourChangedEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSystemSettings
wx_class! { SystemSettings =
    SystemSettingsIsOwned<true>(wxSystemSettings) impl
        SystemSettingsMethods
}
impl<const OWNED: bool> SystemSettingsIsOwned<OWNED> {
    pub fn new() -> SystemSettingsIsOwned<OWNED> {
        unsafe { SystemSettingsIsOwned(ffi::wxSystemSettings_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for SystemSettingsIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSystemSettings_delete(self.0) }
        }
    }
}

// wxTGAHandler
wx_class! { TGAHandler =
    TGAHandlerIsOwned<true>(wxTGAHandler) impl
        TGAHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TGAHandlerIsOwned<OWNED> {
    pub fn new() -> TGAHandlerIsOwned<OWNED> {
        unsafe { TGAHandlerIsOwned(ffi::wxTGAHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TGAHandlerIsOwned<OWNED>> for ImageHandlerIsOwned<OWNED> {
    fn from(o: TGAHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TGAHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TGAHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TGAHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTGAHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for TGAHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTIFFHandler
wx_class! { TIFFHandler =
    TIFFHandlerIsOwned<true>(wxTIFFHandler) impl
        TIFFHandlerMethods,
        // ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TIFFHandlerIsOwned<OWNED> {
    pub fn new() -> TIFFHandlerIsOwned<OWNED> {
        unsafe { TIFFHandlerIsOwned(ffi::wxTIFFHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TIFFHandlerIsOwned<OWNED>> for ImageHandlerIsOwned<OWNED> {
    fn from(o: TIFFHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TIFFHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TIFFHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TIFFHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTIFFHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for TIFFHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> ImageHandlerMethods for TIFFHandlerIsOwned<OWNED> {
    fn get_library_version_info() -> VersionInfo {
        unsafe { VersionInfo::from_ptr(ffi::wxTIFFHandler_GetLibraryVersionInfo()) }
    }
}

// wxTaskBarButton
wx_class! { TaskBarButton =
    TaskBarButtonIsOwned<true>(wxTaskBarButton) impl
        TaskBarButtonMethods
}
impl<const OWNED: bool> TaskBarButtonIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for TaskBarButtonIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTaskBarButton_delete(self.0) }
        }
    }
}

// wxTaskBarIcon
wx_class! { TaskBarIcon =
    TaskBarIconIsOwned<true>(wxTaskBarIcon) impl
        TaskBarIconMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TaskBarIconIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxTaskBarIcon()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TaskBarIconIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TaskBarIconIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TaskBarIconIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TaskBarIconIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TaskBarIconIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTaskBarIcon_CLASSINFO()) }
    }
}
// Mix-in(s) to wxTaskBarIcon
impl<const OWNED: bool> TrackableMethods for TaskBarIconIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxTaskBarIcon_AsTrackable(self.as_ptr()) }
    }
}

// wxTaskBarIconEvent
wx_class! { TaskBarIconEvent =
    TaskBarIconEventIsOwned<true>(wxTaskBarIconEvent) impl
        TaskBarIconEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> TaskBarIconEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxTaskBarIconEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TaskBarIconEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: TaskBarIconEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TaskBarIconEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TaskBarIconEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TaskBarIconEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTaskBarIconEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for TaskBarIconEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTaskBarJumpList
wx_class! { TaskBarJumpList =
    TaskBarJumpListIsOwned<true>(wxTaskBarJumpList) impl
        TaskBarJumpListMethods
}
impl<const OWNED: bool> TaskBarJumpListIsOwned<OWNED> {
    pub fn new(app_id: &str) -> TaskBarJumpListIsOwned<OWNED> {
        unsafe {
            let app_id = WxString::from(app_id);
            let app_id = app_id.as_ptr();
            TaskBarJumpListIsOwned(ffi::wxTaskBarJumpList_new(app_id))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for TaskBarJumpListIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTaskBarJumpList_delete(self.0) }
        }
    }
}

// wxTaskBarJumpListCategory
wx_class! { TaskBarJumpListCategory =
    TaskBarJumpListCategoryIsOwned<true>(wxTaskBarJumpListCategory) impl
        TaskBarJumpListCategoryMethods
}
impl<const OWNED: bool> TaskBarJumpListCategoryIsOwned<OWNED> {
    pub fn new<T: TaskBarJumpListMethods>(
        parent: Option<&T>,
        title: &str,
    ) -> TaskBarJumpListCategoryIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            TaskBarJumpListCategoryIsOwned(ffi::wxTaskBarJumpListCategory_new(parent, title))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for TaskBarJumpListCategoryIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTaskBarJumpListCategory_delete(self.0) }
        }
    }
}

// wxTaskBarJumpListItem
wx_class! { TaskBarJumpListItem =
    TaskBarJumpListItemIsOwned<true>(wxTaskBarJumpListItem) impl
        TaskBarJumpListItemMethods
}
impl<const OWNED: bool> TaskBarJumpListItemIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxTaskBarJumpListItem()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for TaskBarJumpListItemIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTaskBarJumpListItem_delete(self.0) }
        }
    }
}

// wxTextAttr
wx_class! { TextAttr =
    TextAttrIsOwned<true>(wxTextAttr) impl
        TextAttrMethods
}
impl<const OWNED: bool> TextAttrIsOwned<OWNED> {
    pub fn new() -> TextAttrIsOwned<OWNED> {
        unsafe { TextAttrIsOwned(ffi::wxTextAttr_new()) }
    }
    // NOT_SUPPORTED: fn wxTextAttr1()
    pub fn new_with_textattr<T: TextAttrMethods>(attr: &T) -> TextAttrIsOwned<OWNED> {
        unsafe {
            let attr = attr.as_ptr();
            TextAttrIsOwned(ffi::wxTextAttr_new2(attr))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for TextAttrIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTextAttr_delete(self.0) }
        }
    }
}

// wxTextCompleterSimple
wx_class! { TextCompleterSimple =
    TextCompleterSimpleIsOwned<true>(wxTextCompleterSimple) impl
        TextCompleterSimpleMethods,
        TextCompleterMethods
}
impl<const OWNED: bool> TextCompleterSimpleIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TextCompleterSimpleIsOwned<OWNED>> for TextCompleterIsOwned<OWNED> {
    fn from(o: TextCompleterSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for TextCompleterSimpleIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTextCompleterSimple_delete(self.0) }
        }
    }
}

// wxTextCtrl
wx_class! { TextCtrl =
    TextCtrlIsOwned<true>(wxTextCtrl) impl
        TextCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TextCtrlIsOwned<OWNED> {
    pub fn new_2step() -> TextCtrlIsOwned<OWNED> {
        unsafe { TextCtrlIsOwned(ffi::wxTextCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> TextCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            TextCtrlIsOwned(ffi::wxTextCtrl_new1(
                parent, id, value, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TextCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: TextCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: TextCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TextCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TextCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TextCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTextCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxTextCtrl
impl<const OWNED: bool> TextEntryMethods for TextCtrlIsOwned<OWNED> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxTextCtrl_AsTextEntry(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for TextCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxTextCtrl_AsTrackable(self.as_ptr()) }
    }
}

// wxTextDataObject
wx_class! { TextDataObject =
    TextDataObjectIsOwned<true>(wxTextDataObject) impl
        TextDataObjectMethods,
        // DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> TextDataObjectIsOwned<OWNED> {
    pub fn new(text: &str) -> TextDataObjectIsOwned<OWNED> {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            TextDataObjectIsOwned(ffi::wxTextDataObject_new(text))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TextDataObjectIsOwned<OWNED>> for DataObjectSimpleIsOwned<OWNED> {
    fn from(o: TextDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextDataObjectIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: TextDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for TextDataObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTextDataObject_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> DataObjectSimpleMethods for TextDataObjectIsOwned<OWNED> {
    fn get_format(&self) -> *const c_void {
        unsafe { ffi::wxTextDataObject_GetFormat(self.as_ptr()) }
    }
}

// wxTextDropTarget
wx_class! { TextDropTarget =
    TextDropTargetIsOwned<true>(wxTextDropTarget) impl
        TextDropTargetMethods,
        DropTargetMethods
}
impl<const OWNED: bool> TextDropTargetIsOwned<OWNED> {
    pub fn new() -> TextDropTargetIsOwned<OWNED> {
        unsafe { TextDropTargetIsOwned(ffi::wxTextDropTarget_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TextDropTargetIsOwned<OWNED>> for DropTargetIsOwned<OWNED> {
    fn from(o: TextDropTargetIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for TextDropTargetIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTextDropTarget_delete(self.0) }
        }
    }
}

// wxTextEntry

// wxTextEntryDialog
wx_class! { TextEntryDialog =
    TextEntryDialogIsOwned<true>(wxTextEntryDialog) impl
        TextEntryDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TextEntryDialogIsOwned<OWNED> {
    pub fn new_2step() -> TextEntryDialogIsOwned<OWNED> {
        unsafe { TextEntryDialogIsOwned(ffi::wxTextEntryDialog_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        caption: &str,
        value: &str,
        style: c_long,
        pos: &P,
    ) -> TextEntryDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let caption = WxString::from(caption);
            let caption = caption.as_ptr();
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            TextEntryDialogIsOwned(ffi::wxTextEntryDialog_new1(
                parent, message, caption, value, style, pos,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TextEntryDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: TextEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextEntryDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: TextEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextEntryDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: TextEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextEntryDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: TextEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextEntryDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TextEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextEntryDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TextEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TextEntryDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTextEntryDialog_CLASSINFO()) }
    }
}
// Mix-in(s) to wxTextEntryDialog
impl<const OWNED: bool> TrackableMethods for TextEntryDialogIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxTextEntryDialog_AsTrackable(self.as_ptr()) }
    }
}

// wxTextValidator
wx_class! { TextValidator =
    TextValidatorIsOwned<true>(wxTextValidator) impl
        TextValidatorMethods,
        ValidatorMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TextValidatorIsOwned<OWNED> {
    pub fn new_with_textvalidator<T: TextValidatorMethods>(
        validator: &T,
    ) -> TextValidatorIsOwned<OWNED> {
        unsafe {
            let validator = validator.as_ptr();
            TextValidatorIsOwned(ffi::wxTextValidator_new(validator))
        }
    }
    pub fn new_with_long(style: c_long, val_ptr: *mut c_void) -> TextValidatorIsOwned<OWNED> {
        unsafe { TextValidatorIsOwned(ffi::wxTextValidator_new1(style, val_ptr)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TextValidatorIsOwned<OWNED>> for ValidatorIsOwned<OWNED> {
    fn from(o: TextValidatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextValidatorIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TextValidatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextValidatorIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TextValidatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TextValidatorIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTextValidator_CLASSINFO()) }
    }
}
// Mix-in(s) to wxTextValidator
impl<const OWNED: bool> TrackableMethods for TextValidatorIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxTextValidator_AsTrackable(self.as_ptr()) }
    }
}

// wxThreadEvent
wx_class! { ThreadEvent =
    ThreadEventIsOwned<true>(wxThreadEvent) impl
        ThreadEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ThreadEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxThreadEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ThreadEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ThreadEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ThreadEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ThreadEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ThreadEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxThreadEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ThreadEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxThumbBarButton
wx_class! { ThumbBarButton =
    ThumbBarButtonIsOwned<true>(wxThumbBarButton) impl
        ThumbBarButtonMethods
}
impl<const OWNED: bool> ThumbBarButtonIsOwned<OWNED> {
    pub fn new() -> ThumbBarButtonIsOwned<OWNED> {
        unsafe { ThumbBarButtonIsOwned(ffi::wxThumbBarButton_new()) }
    }
    pub fn new_with_int<I: IconMethods>(
        id: c_int,
        icon: &I,
        tooltip: &str,
        enable: bool,
        dismiss_on_click: bool,
        has_background: bool,
        shown: bool,
        interactive: bool,
    ) -> ThumbBarButtonIsOwned<OWNED> {
        unsafe {
            let icon = icon.as_ptr();
            let tooltip = WxString::from(tooltip);
            let tooltip = tooltip.as_ptr();
            ThumbBarButtonIsOwned(ffi::wxThumbBarButton_new1(
                id,
                icon,
                tooltip,
                enable,
                dismiss_on_click,
                has_background,
                shown,
                interactive,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ThumbBarButtonIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxThumbBarButton_delete(self.0) }
        }
    }
}

// wxTimePickerCtrl
wx_class! { TimePickerCtrl =
    TimePickerCtrlIsOwned<true>(wxTimePickerCtrl) impl
        TimePickerCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TimePickerCtrlIsOwned<OWNED> {
    pub fn new_2step() -> TimePickerCtrlIsOwned<OWNED> {
        unsafe { TimePickerCtrlIsOwned(ffi::wxTimePickerCtrl_new()) }
    }
    pub fn new<
        W: WindowMethods,
        D: DateTimeMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        dt: &D,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> TimePickerCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt = dt.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            TimePickerCtrlIsOwned(ffi::wxTimePickerCtrl_new1(
                parent, id, dt, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TimePickerCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: TimePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TimePickerCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: TimePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TimePickerCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TimePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TimePickerCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TimePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TimePickerCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTimePickerCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxTimePickerCtrl
impl<const OWNED: bool> TrackableMethods for TimePickerCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxTimePickerCtrl_AsTrackable(self.as_ptr()) }
    }
}

// wxTipProvider
wx_class! { TipProvider =
    TipProviderIsOwned<true>(wxTipProvider) impl
        TipProviderMethods
}
impl<const OWNED: bool> TipProviderIsOwned<OWNED> {
    pub fn new(current_tip: usize) -> TipProviderIsOwned<OWNED> {
        unsafe { TipProviderIsOwned(ffi::wxTipProvider_new(current_tip)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for TipProviderIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTipProvider_delete(self.0) }
        }
    }
}

// wxTipWindow
wx_class! { TipWindow =
    TipWindowIsOwned<true>(wxTipWindow) impl
        TipWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TipWindowIsOwned<OWNED> {
    pub fn new<W: WindowMethods, T: TipWindowMethods, R: RectMethods>(
        parent: Option<&W>,
        text: &str,
        max_length: c_int,
        window_ptr: Option<&T>,
        rect_bounds: Option<&R>,
    ) -> TipWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let text = WxString::from(text);
            let text = text.as_ptr();
            let window_ptr = match window_ptr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let rect_bounds = match rect_bounds {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            TipWindowIsOwned(ffi::wxTipWindow_new(
                parent,
                text,
                max_length,
                window_ptr,
                rect_bounds,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TipWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: TipWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TipWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TipWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TipWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TipWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TipWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTipWindow_CLASSINFO()) }
    }
}
// Mix-in(s) to wxTipWindow
impl<const OWNED: bool> TrackableMethods for TipWindowIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxTipWindow_AsTrackable(self.as_ptr()) }
    }
}

// wxToggleButton
wx_class! { ToggleButton =
    ToggleButtonIsOwned<true>(wxToggleButton) impl
        ToggleButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ToggleButtonIsOwned<OWNED> {
    pub fn new_2step() -> ToggleButtonIsOwned<OWNED> {
        unsafe { ToggleButtonIsOwned(ffi::wxToggleButton_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        val: &V,
        name: &str,
    ) -> ToggleButtonIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let val = val.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ToggleButtonIsOwned(ffi::wxToggleButton_new1(
                parent, id, label, pos, size, style, val, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ToggleButtonIsOwned<OWNED>> for AnyButtonIsOwned<OWNED> {
    fn from(o: ToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToggleButtonIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToggleButtonIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToggleButtonIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToggleButtonIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ToggleButtonIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxToggleButton_CLASSINFO()) }
    }
}
// Mix-in(s) to wxToggleButton
impl<const OWNED: bool> TrackableMethods for ToggleButtonIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxToggleButton_AsTrackable(self.as_ptr()) }
    }
}

// wxToolBar
wx_class! { ToolBar =
    ToolBarIsOwned<true>(wxToolBar) impl
        ToolBarMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ToolBarIsOwned<OWNED> {
    pub fn new_2step() -> ToolBarIsOwned<OWNED> {
        unsafe { ToolBarIsOwned(ffi::wxToolBar_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> ToolBarIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ToolBarIsOwned(ffi::wxToolBar_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ToolBarIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ToolBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolBarIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ToolBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolBarIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ToolBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolBarIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ToolBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ToolBarIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxToolBar_CLASSINFO()) }
    }
}
// Mix-in(s) to wxToolBar
impl<const OWNED: bool> TrackableMethods for ToolBarIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxToolBar_AsTrackable(self.as_ptr()) }
    }
}

// wxToolTip
wx_class! { ToolTip =
    ToolTipIsOwned<true>(wxToolTip) impl
        ToolTipMethods,
        ObjectMethods
}
impl<const OWNED: bool> ToolTipIsOwned<OWNED> {
    pub fn new(tip: &str) -> ToolTipIsOwned<OWNED> {
        unsafe {
            let tip = WxString::from(tip);
            let tip = tip.as_ptr();
            ToolTipIsOwned(ffi::wxToolTip_new(tip))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ToolTipIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ToolTipIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ToolTipIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxToolTip_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ToolTipIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxToolbook
wx_class! { Toolbook =
    ToolbookIsOwned<true>(wxToolbook) impl
        ToolbookMethods,
        // BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ToolbookIsOwned<OWNED> {
    pub fn new_2step() -> ToolbookIsOwned<OWNED> {
        unsafe { ToolbookIsOwned(ffi::wxToolbook_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> ToolbookIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ToolbookIsOwned(ffi::wxToolbook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ToolbookIsOwned<OWNED>> for BookCtrlBaseIsOwned<OWNED> {
    fn from(o: ToolbookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolbookIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ToolbookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolbookIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ToolbookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolbookIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ToolbookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolbookIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ToolbookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ToolbookIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxToolbook_CLASSINFO()) }
    }
}
// Mix-in(s) to wxToolbook
impl<const OWNED: bool> WithImagesMethods for ToolbookIsOwned<OWNED> {
    fn as_with_images(&self) -> *mut c_void {
        unsafe { ffi::wxToolbook_AsWithImages(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for ToolbookIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxToolbook_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> BookCtrlBaseMethods for ToolbookIsOwned<OWNED> {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
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
            ffi::wxToolbook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}
impl<const OWNED: bool> WindowMethods for ToolbookIsOwned<OWNED> {
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
            ffi::wxToolbook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxTopLevelWindow
wx_class! { TopLevelWindow =
    TopLevelWindowIsOwned<true>(wxTopLevelWindow) impl
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TopLevelWindowIsOwned<OWNED> {
    pub fn new_2step() -> TopLevelWindowIsOwned<OWNED> {
        unsafe { TopLevelWindowIsOwned(ffi::wxTopLevelWindow_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> TopLevelWindowIsOwned<OWNED> {
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
            TopLevelWindowIsOwned(ffi::wxTopLevelWindow_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TopLevelWindowIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: TopLevelWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TopLevelWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: TopLevelWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TopLevelWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TopLevelWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TopLevelWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TopLevelWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TopLevelWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTopLevelWindow_CLASSINFO()) }
    }
}
// Mix-in(s) to wxTopLevelWindow
impl<const OWNED: bool> TrackableMethods for TopLevelWindowIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxTopLevelWindow_AsTrackable(self.as_ptr()) }
    }
}

// wxTreeCtrl
wx_class! { TreeCtrl =
    TreeCtrlIsOwned<true>(wxTreeCtrl) impl
        TreeCtrlMethods,
        // ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TreeCtrlIsOwned<OWNED> {
    pub fn new_2step() -> TreeCtrlIsOwned<OWNED> {
        unsafe { TreeCtrlIsOwned(ffi::wxTreeCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> TreeCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            TreeCtrlIsOwned(ffi::wxTreeCtrl_new1(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TreeCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: TreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: TreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TreeCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTreeCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxTreeCtrl
impl<const OWNED: bool> WithImagesMethods for TreeCtrlIsOwned<OWNED> {
    fn as_with_images(&self) -> *mut c_void {
        unsafe { ffi::wxTreeCtrl_AsWithImages(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for TreeCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxTreeCtrl_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ControlMethods for TreeCtrlIsOwned<OWNED> {
    fn create_validator<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
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
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxTreeCtrl_Create(self.as_ptr(), parent, id, pos, size, style, validator, name)
        }
    }
}
impl<const OWNED: bool> WindowMethods for TreeCtrlIsOwned<OWNED> {
    fn set_window_style(&self, styles: c_long) {
        unsafe { ffi::wxTreeCtrl_SetWindowStyle(self.as_ptr(), styles) }
    }
}

// wxTreeEvent
wx_class! { TreeEvent =
    TreeEventIsOwned<true>(wxTreeEvent) impl
        TreeEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> TreeEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxTreeEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TreeEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: TreeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: TreeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: TreeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TreeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TreeEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTreeEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for TreeEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTreeItemData
wx_class! { TreeItemData =
    TreeItemDataIsOwned<true>(wxTreeItemData) impl
        TreeItemDataMethods,
        ClientDataMethods
}
impl<const OWNED: bool> TreeItemDataIsOwned<OWNED> {
    pub fn new() -> TreeItemDataIsOwned<OWNED> {
        unsafe { TreeItemDataIsOwned(ffi::wxTreeItemData_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TreeItemDataIsOwned<OWNED>> for ClientDataIsOwned<OWNED> {
    fn from(o: TreeItemDataIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for TreeItemDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTreeItemData_delete(self.0) }
        }
    }
}

// wxTreeItemId
wx_class! { TreeItemId =
    TreeItemIdIsOwned<true>(wxTreeItemId) impl
        TreeItemIdMethods
}
impl<const OWNED: bool> TreeItemIdIsOwned<OWNED> {
    pub fn new() -> TreeItemIdIsOwned<OWNED> {
        unsafe { TreeItemIdIsOwned(ffi::wxTreeItemId_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for TreeItemIdIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTreeItemId_delete(self.0) }
        }
    }
}

// wxTreeListCtrl
wx_class! { TreeListCtrl =
    TreeListCtrlIsOwned<true>(wxTreeListCtrl) impl
        TreeListCtrlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TreeListCtrlIsOwned<OWNED> {
    pub fn new_2step() -> TreeListCtrlIsOwned<OWNED> {
        unsafe { TreeListCtrlIsOwned(ffi::wxTreeListCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> TreeListCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            TreeListCtrlIsOwned(ffi::wxTreeListCtrl_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TreeListCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: TreeListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeListCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TreeListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeListCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TreeListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TreeListCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTreeListCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxTreeListCtrl
impl<const OWNED: bool> TrackableMethods for TreeListCtrlIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxTreeListCtrl_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> WindowMethods for TreeListCtrlIsOwned<OWNED> {
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
            ffi::wxTreeListCtrl_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxTreeListItem
wx_class! { TreeListItem =
    TreeListItemIsOwned<true>(wxTreeListItem) impl
        TreeListItemMethods
}
impl<const OWNED: bool> TreeListItemIsOwned<OWNED> {
    pub fn new() -> TreeListItemIsOwned<OWNED> {
        unsafe { TreeListItemIsOwned(ffi::wxTreeListItem_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for TreeListItemIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTreeListItem_delete(self.0) }
        }
    }
}

// wxTreeListItemComparator
wx_class! { TreeListItemComparator =
    TreeListItemComparatorIsOwned<true>(wxTreeListItemComparator) impl
        TreeListItemComparatorMethods
}
impl<const OWNED: bool> TreeListItemComparatorIsOwned<OWNED> {
    pub fn new() -> TreeListItemComparatorIsOwned<OWNED> {
        unsafe { TreeListItemComparatorIsOwned(ffi::wxTreeListItemComparator_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for TreeListItemComparatorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTreeListItemComparator_delete(self.0) }
        }
    }
}

// wxTreebook
wx_class! { Treebook =
    TreebookIsOwned<true>(wxTreebook) impl
        TreebookMethods,
        // BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TreebookIsOwned<OWNED> {
    pub fn new_2step() -> TreebookIsOwned<OWNED> {
        unsafe { TreebookIsOwned(ffi::wxTreebook_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> TreebookIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            TreebookIsOwned(ffi::wxTreebook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TreebookIsOwned<OWNED>> for BookCtrlBaseIsOwned<OWNED> {
    fn from(o: TreebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreebookIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: TreebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreebookIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: TreebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreebookIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TreebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreebookIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TreebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TreebookIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTreebook_CLASSINFO()) }
    }
}
// Mix-in(s) to wxTreebook
impl<const OWNED: bool> WithImagesMethods for TreebookIsOwned<OWNED> {
    fn as_with_images(&self) -> *mut c_void {
        unsafe { ffi::wxTreebook_AsWithImages(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for TreebookIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxTreebook_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> BookCtrlBaseMethods for TreebookIsOwned<OWNED> {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
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
            ffi::wxTreebook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}
impl<const OWNED: bool> WindowMethods for TreebookIsOwned<OWNED> {
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
            ffi::wxTreebook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxTwoFingerTapEvent
wx_class! { TwoFingerTapEvent =
    TwoFingerTapEventIsOwned<true>(wxTwoFingerTapEvent) impl
        TwoFingerTapEventMethods,
        GestureEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> TwoFingerTapEventIsOwned<OWNED> {
    pub fn new(windid: c_int) -> TwoFingerTapEventIsOwned<OWNED> {
        unsafe { TwoFingerTapEventIsOwned(ffi::wxTwoFingerTapEvent_new(windid)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TwoFingerTapEventIsOwned<OWNED>> for GestureEventIsOwned<OWNED> {
    fn from(o: TwoFingerTapEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TwoFingerTapEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: TwoFingerTapEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TwoFingerTapEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TwoFingerTapEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TwoFingerTapEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTwoFingerTapEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for TwoFingerTapEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxUIActionSimulator
wx_class! { UIActionSimulator =
    UIActionSimulatorIsOwned<true>(wxUIActionSimulator) impl
        UIActionSimulatorMethods
}
impl<const OWNED: bool> UIActionSimulatorIsOwned<OWNED> {
    pub fn new() -> UIActionSimulatorIsOwned<OWNED> {
        unsafe { UIActionSimulatorIsOwned(ffi::wxUIActionSimulator_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for UIActionSimulatorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxUIActionSimulator_delete(self.0) }
        }
    }
}

// wxURLDataObject
wx_class! { URLDataObject =
    URLDataObjectIsOwned<true>(wxURLDataObject) impl
        URLDataObjectMethods,
        DataObjectMethods
}
impl<const OWNED: bool> URLDataObjectIsOwned<OWNED> {
    pub fn new(url: &str) -> URLDataObjectIsOwned<OWNED> {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            URLDataObjectIsOwned(ffi::wxURLDataObject_new(url))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<URLDataObjectIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: URLDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for URLDataObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxURLDataObject_delete(self.0) }
        }
    }
}

// wxUpdateUIEvent
wx_class! { UpdateUIEvent =
    UpdateUIEventIsOwned<true>(wxUpdateUIEvent) impl
        UpdateUIEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> UpdateUIEventIsOwned<OWNED> {
    pub fn new(command_id: c_int) -> UpdateUIEventIsOwned<OWNED> {
        unsafe { UpdateUIEventIsOwned(ffi::wxUpdateUIEvent_new(command_id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<UpdateUIEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: UpdateUIEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<UpdateUIEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: UpdateUIEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<UpdateUIEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: UpdateUIEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for UpdateUIEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxUpdateUIEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for UpdateUIEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxVListBox
wx_class! { VListBox =
    VListBoxIsOwned<true>(wxVListBox) impl
        VListBoxMethods,
        // VScrolledWindowMethods,
        // PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> VListBoxIsOwned<OWNED> {
    pub fn new_2step() -> VListBoxIsOwned<OWNED> {
        unsafe { VListBoxIsOwned(ffi::wxVListBox_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> VListBoxIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            VListBoxIsOwned(ffi::wxVListBox_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<VListBoxIsOwned<OWNED>> for VScrolledWindowIsOwned<OWNED> {
    fn from(o: VListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<VListBoxIsOwned<OWNED>> for PanelIsOwned<OWNED> {
    fn from(o: VListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<VListBoxIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: VListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<VListBoxIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: VListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<VListBoxIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: VListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for VListBoxIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxVListBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxVListBox
impl<const OWNED: bool> VarVScrollHelperMethods for VListBoxIsOwned<OWNED> {
    fn as_var_v_scroll_helper(&self) -> *mut c_void {
        unsafe { ffi::wxVListBox_AsVarVScrollHelper(self.as_ptr()) }
    }
}
impl<const OWNED: bool> VarScrollHelperBaseMethods for VListBoxIsOwned<OWNED> {
    fn as_var_scroll_helper_base(&self) -> *mut c_void {
        unsafe { ffi::wxVListBox_AsVarVScrollHelper(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for VListBoxIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxVListBox_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> VScrolledWindowMethods for VListBoxIsOwned<OWNED> {
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
            ffi::wxVListBox_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}
impl<const OWNED: bool> PanelMethods for VListBoxIsOwned<OWNED> {
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
            ffi::wxVListBox_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}
impl<const OWNED: bool> WindowMethods for VListBoxIsOwned<OWNED> {
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
            ffi::wxVListBox_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxVScrolledWindow
wx_class! { VScrolledWindow =
    VScrolledWindowIsOwned<true>(wxVScrolledWindow) impl
        VScrolledWindowMethods,
        // PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> VScrolledWindowIsOwned<OWNED> {
    pub fn new_2step() -> VScrolledWindowIsOwned<OWNED> {
        unsafe { VScrolledWindowIsOwned(ffi::wxVScrolledWindow_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> VScrolledWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            VScrolledWindowIsOwned(ffi::wxVScrolledWindow_new1(
                parent, id, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<VScrolledWindowIsOwned<OWNED>> for PanelIsOwned<OWNED> {
    fn from(o: VScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<VScrolledWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: VScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<VScrolledWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: VScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<VScrolledWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: VScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for VScrolledWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxVScrolledWindow_CLASSINFO()) }
    }
}
// Mix-in(s) to wxVScrolledWindow
impl<const OWNED: bool> VarVScrollHelperMethods for VScrolledWindowIsOwned<OWNED> {
    fn as_var_v_scroll_helper(&self) -> *mut c_void {
        unsafe { ffi::wxVScrolledWindow_AsVarVScrollHelper(self.as_ptr()) }
    }
}
impl<const OWNED: bool> VarScrollHelperBaseMethods for VScrolledWindowIsOwned<OWNED> {
    fn as_var_scroll_helper_base(&self) -> *mut c_void {
        unsafe { ffi::wxVScrolledWindow_AsVarVScrollHelper(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TrackableMethods for VScrolledWindowIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxVScrolledWindow_AsTrackable(self.as_ptr()) }
    }
}
impl<const OWNED: bool> PanelMethods for VScrolledWindowIsOwned<OWNED> {
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
            ffi::wxVScrolledWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}
impl<const OWNED: bool> WindowMethods for VScrolledWindowIsOwned<OWNED> {
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
            ffi::wxVScrolledWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxValidator
wx_class! { Validator =
    ValidatorIsOwned<true>(wxValidator) impl
        ValidatorMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ValidatorIsOwned<OWNED> {
    pub fn new() -> ValidatorIsOwned<OWNED> {
        unsafe { ValidatorIsOwned(ffi::wxValidator_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ValidatorIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ValidatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ValidatorIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ValidatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ValidatorIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxValidator_CLASSINFO()) }
    }
}
// Mix-in(s) to wxValidator
impl<const OWNED: bool> TrackableMethods for ValidatorIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxValidator_AsTrackable(self.as_ptr()) }
    }
}

// wxVarHScrollHelper

// wxVarHVScrollHelper

// wxVarScrollHelperBase
wx_class! { VarScrollHelperBase =
    VarScrollHelperBaseIsOwned<true>(wxVarScrollHelperBase) impl
        VarScrollHelperBaseMethods
}
impl<const OWNED: bool> VarScrollHelperBaseIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(win_to_scroll: Option<&W>) -> VarScrollHelperBaseIsOwned<OWNED> {
        unsafe {
            let win_to_scroll = match win_to_scroll {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            VarScrollHelperBaseIsOwned(ffi::wxVarScrollHelperBase_new(win_to_scroll))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for VarScrollHelperBaseIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxVarScrollHelperBase_delete(self.0) }
        }
    }
}

// wxVarVScrollHelper

// wxVariantDataCurrency
wx_class! { VariantDataCurrency =
    VariantDataCurrencyIsOwned<true>(wxVariantDataCurrency) impl
        VariantDataCurrencyMethods,
        VariantDataMethods,
        ObjectRefDataMethods
}
impl<const OWNED: bool> VariantDataCurrencyIsOwned<OWNED> {
    pub fn new() -> VariantDataCurrencyIsOwned<OWNED> {
        unsafe { VariantDataCurrencyIsOwned(ffi::wxVariantDataCurrency_new()) }
    }
    // NOT_SUPPORTED: fn wxVariantDataCurrency1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<VariantDataCurrencyIsOwned<OWNED>> for VariantDataIsOwned<OWNED> {
    fn from(o: VariantDataCurrencyIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<VariantDataCurrencyIsOwned<OWNED>> for ObjectRefDataIsOwned<OWNED> {
    fn from(o: VariantDataCurrencyIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for VariantDataCurrencyIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxVariantDataCurrency_delete(self.0) }
        }
    }
}

// wxVariantDataErrorCode
wx_class! { VariantDataErrorCode =
    VariantDataErrorCodeIsOwned<true>(wxVariantDataErrorCode) impl
        VariantDataErrorCodeMethods,
        VariantDataMethods,
        ObjectRefDataMethods
}
impl<const OWNED: bool> VariantDataErrorCodeIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxVariantDataErrorCode()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<VariantDataErrorCodeIsOwned<OWNED>> for VariantDataIsOwned<OWNED> {
    fn from(o: VariantDataErrorCodeIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<VariantDataErrorCodeIsOwned<OWNED>> for ObjectRefDataIsOwned<OWNED> {
    fn from(o: VariantDataErrorCodeIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for VariantDataErrorCodeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxVariantDataErrorCode_delete(self.0) }
        }
    }
}

// wxVariantDataSafeArray
wx_class! { VariantDataSafeArray =
    VariantDataSafeArrayIsOwned<true>(wxVariantDataSafeArray) impl
        VariantDataSafeArrayMethods,
        VariantDataMethods,
        ObjectRefDataMethods
}
impl<const OWNED: bool> VariantDataSafeArrayIsOwned<OWNED> {
    pub fn new(value: *mut c_void) -> VariantDataSafeArrayIsOwned<OWNED> {
        unsafe { VariantDataSafeArrayIsOwned(ffi::wxVariantDataSafeArray_new(value)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<VariantDataSafeArrayIsOwned<OWNED>> for VariantDataIsOwned<OWNED> {
    fn from(o: VariantDataSafeArrayIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<VariantDataSafeArrayIsOwned<OWNED>> for ObjectRefDataIsOwned<OWNED> {
    fn from(o: VariantDataSafeArrayIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for VariantDataSafeArrayIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxVariantDataSafeArray_delete(self.0) }
        }
    }
}

// wxView
wx_class! { View =
    ViewIsOwned<true>(wxView) impl
        ViewMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ViewIsOwned<OWNED> {
    pub fn new() -> ViewIsOwned<OWNED> {
        unsafe { ViewIsOwned(ffi::wxView_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ViewIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ViewIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ViewIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ViewIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ViewIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxView_CLASSINFO()) }
    }
}
// Mix-in(s) to wxView
impl<const OWNED: bool> TrackableMethods for ViewIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxView_AsTrackable(self.as_ptr()) }
    }
}

// wxWindow
wx_class! { Window =
    WindowIsOwned<true>(wxWindow) impl
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WindowIsOwned<OWNED> {
    pub fn new_2step() -> WindowIsOwned<OWNED> {
        unsafe { WindowIsOwned(ffi::wxWindow_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> WindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            WindowIsOwned(ffi::wxWindow_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<WindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: WindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWindow_CLASSINFO()) }
    }
}
// Mix-in(s) to wxWindow
impl<const OWNED: bool> TrackableMethods for WindowIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_AsTrackable(self.as_ptr()) }
    }
}

// wxWindowCreateEvent
wx_class! { WindowCreateEvent =
    WindowCreateEventIsOwned<true>(wxWindowCreateEvent) impl
        WindowCreateEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> WindowCreateEventIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(win: Option<&W>) -> WindowCreateEventIsOwned<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WindowCreateEventIsOwned(ffi::wxWindowCreateEvent_new(win))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<WindowCreateEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: WindowCreateEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowCreateEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: WindowCreateEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowCreateEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WindowCreateEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WindowCreateEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWindowCreateEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for WindowCreateEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxWindowDC
wx_class! { WindowDC =
    WindowDCIsOwned<true>(wxWindowDC) impl
        WindowDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> WindowDCIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(window: Option<&W>) -> WindowDCIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WindowDCIsOwned(ffi::wxWindowDC_new(window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<WindowDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: WindowDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WindowDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WindowDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWindowDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for WindowDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxWindowDestroyEvent
wx_class! { WindowDestroyEvent =
    WindowDestroyEventIsOwned<true>(wxWindowDestroyEvent) impl
        WindowDestroyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> WindowDestroyEventIsOwned<OWNED> {
    pub fn new<W: WindowMethods>(win: Option<&W>) -> WindowDestroyEventIsOwned<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WindowDestroyEventIsOwned(ffi::wxWindowDestroyEvent_new(win))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<WindowDestroyEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: WindowDestroyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowDestroyEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: WindowDestroyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowDestroyEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WindowDestroyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WindowDestroyEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWindowDestroyEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for WindowDestroyEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxWindowDisabler
wx_class! { WindowDisabler =
    WindowDisablerIsOwned<true>(wxWindowDisabler) impl
        WindowDisablerMethods
}
impl<const OWNED: bool> WindowDisablerIsOwned<OWNED> {
    pub fn new_with_bool(disable: bool) -> WindowDisablerIsOwned<OWNED> {
        unsafe { WindowDisablerIsOwned(ffi::wxWindowDisabler_new(disable)) }
    }
    pub fn new_with_window<W: WindowMethods, W2: WindowMethods>(
        win_to_skip: Option<&W>,
        win_to_skip2: Option<&W2>,
    ) -> WindowDisablerIsOwned<OWNED> {
        unsafe {
            let win_to_skip = match win_to_skip {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let win_to_skip2 = match win_to_skip2 {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WindowDisablerIsOwned(ffi::wxWindowDisabler_new1(win_to_skip, win_to_skip2))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for WindowDisablerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxWindowDisabler_delete(self.0) }
        }
    }
}

// wxWizard
wx_class! { Wizard =
    WizardIsOwned<true>(wxWizard) impl
        WizardMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WizardIsOwned<OWNED> {
    pub fn new_2step() -> WizardIsOwned<OWNED> {
        unsafe { WizardIsOwned(ffi::wxWizard_new()) }
    }
    pub fn new<W: WindowMethods, B: BitmapBundleMethods, P: PointMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        bitmap: &B,
        pos: &P,
        style: c_long,
    ) -> WizardIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            let bitmap = bitmap.as_ptr();
            let pos = pos.as_ptr();
            WizardIsOwned(ffi::wxWizard_new1(parent, id, title, bitmap, pos, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<WizardIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: WizardIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: WizardIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: WizardIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: WizardIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: WizardIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WizardIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WizardIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWizard_CLASSINFO()) }
    }
}
// Mix-in(s) to wxWizard
impl<const OWNED: bool> TrackableMethods for WizardIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxWizard_AsTrackable(self.as_ptr()) }
    }
}

// wxWizardEvent
wx_class! { WizardEvent =
    WizardEventIsOwned<true>(wxWizardEvent) impl
        WizardEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> WizardEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxWizardEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<WizardEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: WizardEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: WizardEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: WizardEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WizardEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WizardEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWizardEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for WizardEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxWizardPage
wx_class! { WizardPage =
    WizardPageIsOwned<true>(wxWizardPage) impl
        WizardPageMethods,
        PanelMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WizardPageIsOwned<OWNED> {
    pub fn new_2step() -> WizardPageIsOwned<OWNED> {
        unsafe { WizardPageIsOwned(ffi::wxWizardPage_new()) }
    }
    pub fn new<W: WizardMethods, B: BitmapBundleMethods>(
        parent: Option<&W>,
        bitmap: &B,
    ) -> WizardPageIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let bitmap = bitmap.as_ptr();
            WizardPageIsOwned(ffi::wxWizardPage_new1(parent, bitmap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<WizardPageIsOwned<OWNED>> for PanelIsOwned<OWNED> {
    fn from(o: WizardPageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: WizardPageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: WizardPageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WizardPageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WizardPageIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWizardPage_CLASSINFO()) }
    }
}
// Mix-in(s) to wxWizardPage
impl<const OWNED: bool> TrackableMethods for WizardPageIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxWizardPage_AsTrackable(self.as_ptr()) }
    }
}

// wxWizardPageSimple
wx_class! { WizardPageSimple =
    WizardPageSimpleIsOwned<true>(wxWizardPageSimple) impl
        WizardPageSimpleMethods,
        WizardPageMethods,
        PanelMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WizardPageSimpleIsOwned<OWNED> {
    pub fn new_2step() -> WizardPageSimpleIsOwned<OWNED> {
        unsafe { WizardPageSimpleIsOwned(ffi::wxWizardPageSimple_new()) }
    }
    pub fn new<
        W: WizardMethods,
        W2: WizardPageMethods,
        W3: WizardPageMethods,
        B: BitmapBundleMethods,
    >(
        parent: Option<&W>,
        prev: Option<&W2>,
        next: Option<&W3>,
        bitmap: &B,
    ) -> WizardPageSimpleIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let prev = match prev {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let next = match next {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let bitmap = bitmap.as_ptr();
            WizardPageSimpleIsOwned(ffi::wxWizardPageSimple_new1(parent, prev, next, bitmap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<WizardPageSimpleIsOwned<OWNED>> for WizardPageIsOwned<OWNED> {
    fn from(o: WizardPageSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageSimpleIsOwned<OWNED>> for PanelIsOwned<OWNED> {
    fn from(o: WizardPageSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageSimpleIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: WizardPageSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageSimpleIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: WizardPageSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageSimpleIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WizardPageSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WizardPageSimpleIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWizardPageSimple_CLASSINFO()) }
    }
}
// Mix-in(s) to wxWizardPageSimple
impl<const OWNED: bool> TrackableMethods for WizardPageSimpleIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxWizardPageSimple_AsTrackable(self.as_ptr()) }
    }
}

// wxWrapSizer
wx_class! { WrapSizer =
    WrapSizerIsOwned<true>(wxWrapSizer) impl
        WrapSizerMethods,
        BoxSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WrapSizerIsOwned<OWNED> {
    pub fn new(orient: c_int, flags: c_int) -> WrapSizerIsOwned<OWNED> {
        unsafe { WrapSizerIsOwned(ffi::wxWrapSizer_new(orient, flags)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<WrapSizerIsOwned<OWNED>> for BoxSizerIsOwned<OWNED> {
    fn from(o: WrapSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WrapSizerIsOwned<OWNED>> for SizerIsOwned<OWNED> {
    fn from(o: WrapSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WrapSizerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WrapSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WrapSizerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWrapSizer_CLASSINFO()) }
    }
}

// wxXPMHandler
wx_class! { XPMHandler =
    XPMHandlerIsOwned<true>(wxXPMHandler) impl
        XPMHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> XPMHandlerIsOwned<OWNED> {
    pub fn new() -> XPMHandlerIsOwned<OWNED> {
        unsafe { XPMHandlerIsOwned(ffi::wxXPMHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<XPMHandlerIsOwned<OWNED>> for ImageHandlerIsOwned<OWNED> {
    fn from(o: XPMHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<XPMHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: XPMHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for XPMHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxXPMHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for XPMHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxZoomGestureEvent
wx_class! { ZoomGestureEvent =
    ZoomGestureEventIsOwned<true>(wxZoomGestureEvent) impl
        ZoomGestureEventMethods,
        GestureEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ZoomGestureEventIsOwned<OWNED> {
    pub fn new(windid: c_int) -> ZoomGestureEventIsOwned<OWNED> {
        unsafe { ZoomGestureEventIsOwned(ffi::wxZoomGestureEvent_new(windid)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ZoomGestureEventIsOwned<OWNED>> for GestureEventIsOwned<OWNED> {
    fn from(o: ZoomGestureEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ZoomGestureEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ZoomGestureEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ZoomGestureEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ZoomGestureEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ZoomGestureEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxZoomGestureEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ZoomGestureEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
