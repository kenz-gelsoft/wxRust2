use super::*;

// wxVListBox
wx_class! { VListBox =
    VListBoxIsOwned<true>(wxVListBox) impl
        VListBoxMethods,
        VScrolledWindowMethods,
        PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> VListBoxIsOwned<OWNED> {
    // BLOCKED: fn wxVListBox()
    // BLOCKED: fn wxVListBox1()
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
impl<const OWNED: bool> Trackable<VListBoxIsOwned<false>> for VListBoxIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<VListBoxIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
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
        PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> VScrolledWindowIsOwned<OWNED> {
    // BLOCKED: fn wxVScrolledWindow()
    // BLOCKED: fn wxVScrolledWindow1()
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
impl<const OWNED: bool> Trackable<VScrolledWindowIsOwned<false>> for VScrolledWindowIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<VScrolledWindowIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
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
impl<const OWNED: bool> Trackable<ValidatorIsOwned<false>> for ValidatorIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<ValidatorIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
    }
}
