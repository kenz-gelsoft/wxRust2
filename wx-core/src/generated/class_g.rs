#![allow(non_upper_case_globals)]

use super::*;

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
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        anim: *const c_void,
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
impl<const OWNED: bool> AnimationCtrlMethods for GenericAnimationCtrlIsOwned<OWNED> {
    fn create_animation<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        anim: *const c_void,
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
    // BLOCKED: fn wxGraphicsGradientStops()
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
    // BLOCKED: fn wxGridCellEditor()
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
    // BLOCKED: fn wxGridCellRenderer()
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
    // BLOCKED: fn wxGridTableBase()
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
