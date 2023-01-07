use super::*;

// wxLayoutAlgorithm
wxwidgets! {
    /// wxLayoutAlgorithm implements layout of subwindows in MDI or SDI frames.
    /// - [`LayoutAlgorithm`] represents a C++ `wxLayoutAlgorithm` class instance which your code has ownership, [`LayoutAlgorithmInRust`]`<false>` represents one which don't own.
    /// - Use [`LayoutAlgorithm`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxLayoutAlgorithm` class's documentation](https://docs.wxwidgets.org/3.2/classwx_layout_algorithm.html) for more details.
    #[doc(alias = "wxLayoutAlgorithm")]
    #[doc(alias = "LayoutAlgorithm")]
    class LayoutAlgorithm
        = LayoutAlgorithmInRust<true>(wxLayoutAlgorithm) impl
        LayoutAlgorithmMethods,
        ObjectMethods
}
impl<const OWNED: bool> LayoutAlgorithmInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxLayoutAlgorithm::wxLayoutAlgorithm()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_layout_algorithm.html#af8e2301f3e97d27ee3c2e0cbb2587488).
    pub fn new() -> LayoutAlgorithmInRust<OWNED> {
        unsafe { LayoutAlgorithmInRust(ffi::wxLayoutAlgorithm_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for LayoutAlgorithmInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<LayoutAlgorithmInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: LayoutAlgorithmInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for LayoutAlgorithmInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxLayoutAlgorithm_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for LayoutAlgorithmInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxListBox
wxwidgets! {
    /// A listbox is used to select one or more of a list of strings.
    /// - [`ListBox`] represents a C++ `wxListBox` class instance which your code has ownership, [`ListBoxInRust`]`<false>` represents one which don't own.
    /// - Use [`ListBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxListBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html) for more details.
    #[doc(alias = "wxListBox")]
    #[doc(alias = "ListBox")]
    class ListBox
        = ListBoxInRust<true>(wxListBox) impl
        ListBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ListBoxInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxListBox::wxListBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#a63347db33d1ae5a10c8eb54b3d556b56).
    pub fn new_2step() -> ListBoxInRust<OWNED> {
        unsafe { ListBoxInRust(ffi::wxListBox_new()) }
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
    ) -> ListBoxInRust<OWNED> {
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
            ListBoxInRust(ffi::wxListBox_new2(
                parent, id, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ListBoxInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ListBoxInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: ListBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListBoxInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: ListBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListBoxInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ListBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListBoxInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ListBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ListBoxInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxListBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxListBox
impl<const OWNED: bool> ItemContainerMethods for ListBoxInRust<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxListBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for ListBoxInRust<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxListBox_AsItemContainer(self.as_ptr()) }
    }
}

// wxListCtrl
wxwidgets! {
    /// A list control presents lists in a number of formats: list view, report view, icon view and small icon view.
    /// - [`ListCtrl`] represents a C++ `wxListCtrl` class instance which your code has ownership, [`ListCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`ListCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxListCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html) for more details.
    #[doc(alias = "wxListCtrl")]
    #[doc(alias = "ListCtrl")]
    class ListCtrl
        = ListCtrlInRust<true>(wxListCtrl) impl
        ListCtrlMethods,
        // ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ListCtrlInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxListCtrl::wxListCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a6941e35348b00a288e833871fd0138e8).
    pub fn new_2step() -> ListCtrlInRust<OWNED> {
        unsafe { ListCtrlInRust(ffi::wxListCtrl_new()) }
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
    ) -> ListCtrlInRust<OWNED> {
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
            ListCtrlInRust(ffi::wxListCtrl_new1(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ListCtrlInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ListCtrlInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: ListCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListCtrlInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: ListCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListCtrlInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ListCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListCtrlInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ListCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ListCtrlInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxListCtrl_CLASSINFO()) }
    }
}
impl<const OWNED: bool> ControlMethods for ListCtrlInRust<OWNED> {
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
    /// - [`ListEvent`] represents a C++ `wxListEvent` class instance which your code has ownership, [`ListEventInRust`]`<false>` represents one which don't own.
    /// - Use [`ListEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxListEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html) for more details.
    #[doc(alias = "wxListEvent")]
    #[doc(alias = "ListEvent")]
    class ListEvent
        = ListEventInRust<true>(wxListEvent) impl
        ListEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ListEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxListEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ListEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ListEventInRust<OWNED>> for NotifyEventInRust<OWNED> {
    fn from(o: ListEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: ListEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: ListEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ListEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ListEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxListEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ListEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxListItem
wxwidgets! {
    /// This class stores information about a wxListCtrl item or column.
    /// - [`ListItem`] represents a C++ `wxListItem` class instance which your code has ownership, [`ListItemInRust`]`<false>` represents one which don't own.
    /// - Use [`ListItem`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxListItem` class's documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html) for more details.
    #[doc(alias = "wxListItem")]
    #[doc(alias = "ListItem")]
    class ListItem
        = ListItemInRust<true>(wxListItem) impl
        ListItemMethods,
        ObjectMethods
}
impl<const OWNED: bool> ListItemInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxListItem::wxListItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#ab133f45cd8964e254cdaa1512713c153).
    pub fn new() -> ListItemInRust<OWNED> {
        unsafe { ListItemInRust(ffi::wxListItem_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ListItemInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ListItemInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ListItemInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ListItemInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxListItem_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ListItemInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxListView
wxwidgets! {
    /// This class currently simply presents a simpler to use interface for the wxListCtrl  it can be thought of as a façade for that complicated class.
    /// - [`ListView`] represents a C++ `wxListView` class instance which your code has ownership, [`ListViewInRust`]`<false>` represents one which don't own.
    /// - Use [`ListView`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxListView` class's documentation](https://docs.wxwidgets.org/3.2/classwx_list_view.html) for more details.
    #[doc(alias = "wxListView")]
    #[doc(alias = "ListView")]
    class ListView
        = ListViewInRust<true>(wxListView) impl
        ListViewMethods,
        ListCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ListViewInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxListView::wxListView()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_view.html#a103cf973c8be268b78a0c08c419f606f).
    pub fn new_2step() -> ListViewInRust<OWNED> {
        unsafe { ListViewInRust(ffi::wxListView_new()) }
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
    ) -> ListViewInRust<OWNED> {
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
            ListViewInRust(ffi::wxListView_new1(
                parent, winid, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ListViewInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ListViewInRust<OWNED>> for ListCtrlInRust<OWNED> {
    fn from(o: ListViewInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListViewInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: ListViewInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListViewInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: ListViewInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListViewInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ListViewInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListViewInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ListViewInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ListViewInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxListView_CLASSINFO()) }
    }
}

// wxListbook
wxwidgets! {
    /// wxListbook is a class similar to wxNotebook but which uses a wxListCtrl to show the labels instead of the tabs.
    /// - [`Listbook`] represents a C++ `wxListbook` class instance which your code has ownership, [`ListbookInRust`]`<false>` represents one which don't own.
    /// - Use [`Listbook`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxListbook` class's documentation](https://docs.wxwidgets.org/3.2/classwx_listbook.html) for more details.
    #[doc(alias = "wxListbook")]
    #[doc(alias = "Listbook")]
    class Listbook
        = ListbookInRust<true>(wxListbook) impl
        ListbookMethods,
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ListbookInRust<OWNED> {
    /// Default ctor.
    ///
    /// See [C++ `wxListbook::wxListbook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_listbook.html#abed6de6659aaca045b6503cbf8baf622).
    pub fn new_2step() -> ListbookInRust<OWNED> {
        unsafe { ListbookInRust(ffi::wxListbook_new()) }
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
    ) -> ListbookInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ListbookInRust(ffi::wxListbook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ListbookInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ListbookInRust<OWNED>> for BookCtrlBaseInRust<OWNED> {
    fn from(o: ListbookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListbookInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: ListbookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListbookInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: ListbookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListbookInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ListbookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ListbookInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ListbookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ListbookInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxListbook_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for ListbookInRust<OWNED> {
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
