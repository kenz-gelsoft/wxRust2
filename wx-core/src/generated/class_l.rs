use super::*;

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
impl<const OWNED: bool> Trackable<ListBoxIsOwned<false>> for ListBoxIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<ListBoxIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
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
impl<const OWNED: bool> Trackable<ListCtrlIsOwned<false>> for ListCtrlIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<ListCtrlIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
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
impl<const OWNED: bool> Trackable<ListViewIsOwned<false>> for ListViewIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<ListViewIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
    }
}

// wxListbook
wx_class! { Listbook =
    ListbookIsOwned<true>(wxListbook) impl
        ListbookMethods,
        BookCtrlBaseMethods,
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
impl<const OWNED: bool> Trackable<ListbookIsOwned<false>> for ListbookIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<ListbookIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
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
