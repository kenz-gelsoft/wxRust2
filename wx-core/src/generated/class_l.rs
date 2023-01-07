use super::*;

// wxLayoutAlgorithm
wxwidgets! {
    /// wxLayoutAlgorithm implements layout of subwindows in MDI or SDI frames.
    /// - [`LayoutAlgorithm`] represents a C++ `wxLayoutAlgorithm` class instance which your code has ownership, [`LayoutAlgorithmFromCpp`]`<true>` represents one which don't own.
    /// - Use [`LayoutAlgorithm`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxLayoutAlgorithm` class's documentation](https://docs.wxwidgets.org/3.2/classwx_layout_algorithm.html) for more details.
    #[doc(alias = "wxLayoutAlgorithm")]
    #[doc(alias = "LayoutAlgorithm")]
    class LayoutAlgorithm
        = LayoutAlgorithmFromCpp<false>(wxLayoutAlgorithm) impl
        LayoutAlgorithmMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> LayoutAlgorithmFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxLayoutAlgorithm::wxLayoutAlgorithm()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_layout_algorithm.html#af8e2301f3e97d27ee3c2e0cbb2587488).
    pub fn new() -> LayoutAlgorithmFromCpp<FROM_CPP> {
        unsafe { LayoutAlgorithmFromCpp(ffi::wxLayoutAlgorithm_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for LayoutAlgorithmFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<LayoutAlgorithmFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: LayoutAlgorithmFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for LayoutAlgorithmFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxLayoutAlgorithm_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for LayoutAlgorithmFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxListBox
wxwidgets! {
    /// A listbox is used to select one or more of a list of strings.
    /// - [`ListBox`] represents a C++ `wxListBox` class instance which your code has ownership, [`ListBoxFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ListBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxListBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html) for more details.
    #[doc(alias = "wxListBox")]
    #[doc(alias = "ListBox")]
    class ListBox
        = ListBoxFromCpp<false>(wxListBox) impl
        ListBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ListBoxFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxListBox::wxListBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#a63347db33d1ae5a10c8eb54b3d556b56).
    pub fn new_2step() -> ListBoxFromCpp<FROM_CPP> {
        unsafe { ListBoxFromCpp(ffi::wxListBox_new()) }
    }
    // NOT_SUPPORTED: fn wxListBox1()
    /// Constructor, creating and showing a list box.
    ///
    /// See [C++ `wxListBox::wxListBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#a1dd35666935f63e013a64216f45865eb).
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
    ) -> ListBoxFromCpp<FROM_CPP> {
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
            ListBoxFromCpp(ffi::wxListBox_new2(
                parent, id, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ListBoxFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ListBoxFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: ListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ListBoxFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: ListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ListBoxFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: ListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ListBoxFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ListBoxFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxListBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxListBox
impl<const FROM_CPP: bool> ItemContainerMethods for ListBoxFromCpp<FROM_CPP> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxListBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> ItemContainerImmutableMethods for ListBoxFromCpp<FROM_CPP> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxListBox_AsItemContainer(self.as_ptr()) }
    }
}

// wxListCtrl
wxwidgets! {
    /// A list control presents lists in a number of formats: list view, report view, icon view and small icon view.
    /// - [`ListCtrl`] represents a C++ `wxListCtrl` class instance which your code has ownership, [`ListCtrlFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ListCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxListCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html) for more details.
    #[doc(alias = "wxListCtrl")]
    #[doc(alias = "ListCtrl")]
    class ListCtrl
        = ListCtrlFromCpp<false>(wxListCtrl) impl
        ListCtrlMethods,
        // ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ListCtrlFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxListCtrl::wxListCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a6941e35348b00a288e833871fd0138e8).
    pub fn new_2step() -> ListCtrlFromCpp<FROM_CPP> {
        unsafe { ListCtrlFromCpp(ffi::wxListCtrl_new()) }
    }
    /// Constructor, creating and showing a list control.
    ///
    /// See [C++ `wxListCtrl::wxListCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#aaf98aeeba06ad86fa471f9218ae91097).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ListCtrlFromCpp<FROM_CPP> {
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
            ListCtrlFromCpp(ffi::wxListCtrl_new1(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ListCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ListCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: ListCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ListCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: ListCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ListCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: ListCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ListCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ListCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ListCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxListCtrl_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> ControlMethods for ListCtrlFromCpp<FROM_CPP> {
    /// Creates the list control.
    ///
    /// See [C++ `wxListCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a5b13e700b9957677468d63558d73d5df).
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
wxwidgets! {
    /// A list event holds information about events associated with wxListCtrl objects.
    /// - [`ListEvent`] represents a C++ `wxListEvent` class instance which your code has ownership, [`ListEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ListEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxListEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html) for more details.
    #[doc(alias = "wxListEvent")]
    #[doc(alias = "ListEvent")]
    class ListEvent
        = ListEventFromCpp<false>(wxListEvent) impl
        ListEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ListEventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxListEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ListEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ListEventFromCpp<FROM_CPP>> for NotifyEventFromCpp<FROM_CPP> {
    fn from(o: ListEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ListEventFromCpp<FROM_CPP>> for CommandEventFromCpp<FROM_CPP> {
    fn from(o: ListEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ListEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: ListEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ListEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ListEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ListEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxListEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ListEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxListItem
wxwidgets! {
    /// This class stores information about a wxListCtrl item or column.
    /// - [`ListItem`] represents a C++ `wxListItem` class instance which your code has ownership, [`ListItemFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ListItem`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxListItem` class's documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html) for more details.
    #[doc(alias = "wxListItem")]
    #[doc(alias = "ListItem")]
    class ListItem
        = ListItemFromCpp<false>(wxListItem) impl
        ListItemMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ListItemFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxListItem::wxListItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#ab133f45cd8964e254cdaa1512713c153).
    pub fn new() -> ListItemFromCpp<FROM_CPP> {
        unsafe { ListItemFromCpp(ffi::wxListItem_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ListItemFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ListItemFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ListItemFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ListItemFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxListItem_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ListItemFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxListView
wxwidgets! {
    /// This class currently simply presents a simpler to use interface for the wxListCtrl  it can be thought of as a façade for that complicated class.
    /// - [`ListView`] represents a C++ `wxListView` class instance which your code has ownership, [`ListViewFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ListView`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxListView` class's documentation](https://docs.wxwidgets.org/3.2/classwx_list_view.html) for more details.
    #[doc(alias = "wxListView")]
    #[doc(alias = "ListView")]
    class ListView
        = ListViewFromCpp<false>(wxListView) impl
        ListViewMethods,
        ListCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ListViewFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxListView::wxListView()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_view.html#a103cf973c8be268b78a0c08c419f606f).
    pub fn new_2step() -> ListViewFromCpp<FROM_CPP> {
        unsafe { ListViewFromCpp(ffi::wxListView_new()) }
    }
    /// Constructor, creating and showing a listview control.
    ///
    /// See [C++ `wxListView::wxListView()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_view.html#a6984e2ef325a4702ac7871d6a1c65119).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        winid: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ListViewFromCpp<FROM_CPP> {
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
            ListViewFromCpp(ffi::wxListView_new1(
                parent, winid, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ListViewFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ListViewFromCpp<FROM_CPP>> for ListCtrlFromCpp<FROM_CPP> {
    fn from(o: ListViewFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ListViewFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: ListViewFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ListViewFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: ListViewFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ListViewFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: ListViewFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ListViewFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ListViewFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ListViewFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxListView_CLASSINFO()) }
    }
}

// wxListbook
wxwidgets! {
    /// wxListbook is a class similar to wxNotebook but which uses a wxListCtrl to show the labels instead of the tabs.
    /// - [`Listbook`] represents a C++ `wxListbook` class instance which your code has ownership, [`ListbookFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Listbook`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxListbook` class's documentation](https://docs.wxwidgets.org/3.2/classwx_listbook.html) for more details.
    #[doc(alias = "wxListbook")]
    #[doc(alias = "Listbook")]
    class Listbook
        = ListbookFromCpp<false>(wxListbook) impl
        ListbookMethods,
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ListbookFromCpp<FROM_CPP> {
    /// Default ctor.
    ///
    /// See [C++ `wxListbook::wxListbook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_listbook.html#abed6de6659aaca045b6503cbf8baf622).
    pub fn new_2step() -> ListbookFromCpp<FROM_CPP> {
        unsafe { ListbookFromCpp(ffi::wxListbook_new()) }
    }
    /// Constructs a listbook control.
    ///
    /// See [C++ `wxListbook::wxListbook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_listbook.html#a2e5a198617615b4fa83eafa5b577afb3).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> ListbookFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ListbookFromCpp(ffi::wxListbook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ListbookFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ListbookFromCpp<FROM_CPP>> for BookCtrlBaseFromCpp<FROM_CPP> {
    fn from(o: ListbookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ListbookFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: ListbookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ListbookFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: ListbookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ListbookFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: ListbookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ListbookFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ListbookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ListbookFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxListbook_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> WindowMethods for ListbookFromCpp<FROM_CPP> {
    /// Create the list book control that has already been constructed with the default constructor.
    ///
    /// See [C++ `wxListbook::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_listbook.html#a38a6aad6c1fb31fa24acb471a8a37fd2).
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
