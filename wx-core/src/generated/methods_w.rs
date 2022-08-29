use super::*;

// wxWindow
/// This trait represents C++ [`wxWindow`](https://docs.wxwidgets.org/3.2/classwx_window.html) class's methods and inheritance.
///
/// See [`WindowIsOwned`] documentation for the class usage.
pub trait WindowMethods: EvtHandlerMethods {
    /// This method may be overridden in the derived classes to return false to indicate that this control doesn't accept input at all (i.e. behaves like e.g. wxStaticText) and so doesn't need focus.
    ///
    /// [See `wxWindow::AcceptsFocus()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a1da8baa734b9872d0c085b86a7b5d817)
    fn accepts_focus(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocus(self.as_ptr()) }
    }
    /// This method may be overridden in the derived classes to return false to indicate that while this control can, in principle, have focus if the user clicks it with the mouse, it shouldn't be included in the TAB traversal chain when using the keyboard.
    ///
    /// [See `wxWindow::AcceptsFocusFromKeyboard()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a3a0cb3bd5e82462b19df9f24ad6d9ac4)
    fn accepts_focus_from_keyboard(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocusFromKeyboard(self.as_ptr()) }
    }
    /// Overridden to indicate whether this window or one of its children accepts focus.
    ///
    /// [See `wxWindow::AcceptsFocusRecursively()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#af0c30b930ecf57f94b8cfc8871e8fd94)
    fn accepts_focus_recursively(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocusRecursively(self.as_ptr()) }
    }
    /// Disable giving focus to this window using the keyboard navigation keys.
    ///
    /// [See `wxWindow::DisableFocusFromKeyboard()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a6d721dc0df37d5c74e96474ddbc1e073)
    fn disable_focus_from_keyboard(&self) {
        unsafe { ffi::wxWindow_DisableFocusFromKeyboard(self.as_ptr()) }
    }
    /// Can this window itself have focus?
    ///
    /// [See `wxWindow::IsFocusable()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ad78eb073c4fe8271caa4c65301a24ee2)
    fn is_focusable(&self) -> bool {
        unsafe { ffi::wxWindow_IsFocusable(self.as_ptr()) }
    }
    /// Can this window have focus right now?
    ///
    /// [See `wxWindow::CanAcceptFocus()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#acc4e456ed056a27ec61cd296b96c1e3e)
    fn can_accept_focus(&self) -> bool {
        unsafe { ffi::wxWindow_CanAcceptFocus(self.as_ptr()) }
    }
    /// Can this window be assigned focus from keyboard right now?
    ///
    /// [See `wxWindow::CanAcceptFocusFromKeyboard()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ade15bb13dc6d51a8e6ecd49eb3f0f99b)
    fn can_accept_focus_from_keyboard(&self) -> bool {
        unsafe { ffi::wxWindow_CanAcceptFocusFromKeyboard(self.as_ptr()) }
    }
    /// Returns true if the window (or in case of composite controls, its main child window) has focus.
    ///
    /// [See `wxWindow::HasFocus()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#adaf042744bdbb8a5f9feccbe7749468a)
    fn has_focus(&self) -> bool {
        unsafe { ffi::wxWindow_HasFocus(self.as_ptr()) }
    }
    /// This method is only implemented by ports which have support for native TAB traversal (such as GTK+ 2.0).
    ///
    /// [See `wxWindow::SetCanFocus()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a2b8b2e99231a0ec1a05f5066f1b7f3d8)
    fn set_can_focus(&self, can_focus: bool) {
        unsafe { ffi::wxWindow_SetCanFocus(self.as_ptr(), can_focus) }
    }
    /// Enables or disables visible indication of keyboard focus.
    ///
    /// [See `wxWindow::EnableVisibleFocus()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ad0798adc95b19f956e7ac1e7fda6333d)
    fn enable_visible_focus(&self, enable: bool) {
        unsafe { ffi::wxWindow_EnableVisibleFocus(self.as_ptr(), enable) }
    }
    /// This sets the window to receive keyboard input.
    ///
    /// [See `wxWindow::SetFocus()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a697f9f8d3ff389790f1c74b59bcb1d75)
    fn set_focus(&self) {
        unsafe { ffi::wxWindow_SetFocus(self.as_ptr()) }
    }
    /// This function is called by wxWidgets keyboard navigation code when the user gives the focus to this window from keyboard (e.g. using TAB key).
    ///
    /// [See `wxWindow::SetFocusFromKbd()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a6fa03f82d7917dff482754d0d2e2b1c8)
    fn set_focus_from_kbd(&self) {
        unsafe { ffi::wxWindow_SetFocusFromKbd(self.as_ptr()) }
    }
    /// Adds a child window.
    ///
    /// [See `wxWindow::AddChild()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#abf60159278059a5bb397eb8647227bb3)
    fn add_child<W: WindowMethods>(&self, child: Option<&W>) {
        unsafe {
            let child = match child {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_AddChild(self.as_ptr(), child)
        }
    }
    /// Destroys all children of a window.
    ///
    /// [See `wxWindow::DestroyChildren()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aff47b32c8d42d515ea0bb6a6c2fea917)
    fn destroy_children(&self) -> bool {
        unsafe { ffi::wxWindow_DestroyChildren(self.as_ptr()) }
    }
    /// Find a child of this window, by id.
    ///
    /// [See `wxWindow::FindWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aceaaeff69f50577d5519e0f5c65e910f)
    fn find_window_long(&self, id: c_long) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_FindWindow(self.as_ptr(), id)) }
    }
    /// Find a child of this window, by name.
    ///
    /// [See `wxWindow::FindWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aa537b1b13413d5002e5f7a2a1047e0d7)
    fn find_window_str(&self, name: &str) -> WeakRef<Window> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            WeakRef::<Window>::from(ffi::wxWindow_FindWindow1(self.as_ptr(), name))
        }
    }
    // BLOCKED: fn GetChildren()
    /// Returns a const reference to the list of the window's children.
    ///
    /// [See `wxWindow::GetChildren()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aba31a46e73a980313da20b6277eb8c93)
    fn get_children(&self) -> WindowListIsOwned<false> {
        unsafe { WindowListIsOwned::from_ptr(ffi::wxWindow_GetChildren1(self.as_ptr())) }
    }
    /// Removes a child window.
    ///
    /// [See `wxWindow::RemoveChild()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#acee332ed4368d26e8bc3db5767c1240a)
    fn remove_child<W: WindowMethods>(&self, child: Option<&W>) {
        unsafe {
            let child = match child {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_RemoveChild(self.as_ptr(), child)
        }
    }
    /// Returns the grandparent of a window, or NULL if there isn't one.
    ///
    /// [See `wxWindow::GetGrandParent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a953d89197a6f325e2262ad20ef420585)
    fn get_grand_parent(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetGrandParent(self.as_ptr())) }
    }
    /// Returns the next window after this one among the parent's children or NULL if this window is the last child.
    ///
    /// [See `wxWindow::GetNextSibling()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#af9daac17df9bc32966e453487105d10c)
    fn get_next_sibling(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetNextSibling(self.as_ptr())) }
    }
    /// Returns the parent of the window, or NULL if there is no parent.
    ///
    /// [See `wxWindow::GetParent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a2040f41692f971e81663395ab3b59933)
    fn get_parent(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetParent(self.as_ptr())) }
    }
    /// Returns the previous window before this one among the parent's children or  NULL if this window is the first child.
    ///
    /// [See `wxWindow::GetPrevSibling()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a7dfab8b7173a055e35968a0b4fbd09b6)
    fn get_prev_sibling(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetPrevSibling(self.as_ptr())) }
    }
    /// Check if the specified window is a descendant of this one.
    ///
    /// [See `wxWindow::IsDescendant()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a71b301a7a0a9b5d5fdda417b8dadd13a)
    fn is_descendant<W: WindowMethods>(&self, win: Option<&W>) -> bool {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_IsDescendant(self.as_ptr(), win)
        }
    }
    /// Reparents the window, i.e. the window will be removed from its current parent window (e.g.
    ///
    /// [See `wxWindow::Reparent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a7977b749284e65aecfed2ce146799cb9)
    fn reparent<W: WindowMethods>(&self, new_parent: Option<&W>) -> bool {
        unsafe {
            let new_parent = match new_parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_Reparent(self.as_ptr(), new_parent)
        }
    }
    /// Call this function to force one or both scrollbars to be always shown, even if the window is big enough to show its entire contents without scrolling.
    ///
    /// [See `wxWindow::AlwaysShowScrollbars()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aae29552806a328c6a55ef8f07647f5ba)
    fn always_show_scrollbars(&self, hflag: bool, vflag: bool) {
        unsafe { ffi::wxWindow_AlwaysShowScrollbars(self.as_ptr(), hflag, vflag) }
    }
    /// Returns the built-in scrollbar position.
    ///
    /// [See `wxWindow::GetScrollPos()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#add5f4e225e16154f9148451a7d32d0b3)
    fn get_scroll_pos(&self, orientation: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetScrollPos(self.as_ptr(), orientation) }
    }
    /// Returns the built-in scrollbar range.
    ///
    /// [See `wxWindow::GetScrollRange()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a23b41cf939ab67a002d8f404ac4cf76b)
    fn get_scroll_range(&self, orientation: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetScrollRange(self.as_ptr(), orientation) }
    }
    /// Returns the built-in scrollbar thumb size.
    ///
    /// [See `wxWindow::GetScrollThumb()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ac34f47fef73e3a178e8c335aa38ce567)
    fn get_scroll_thumb(&self, orientation: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetScrollThumb(self.as_ptr(), orientation) }
    }
    /// Returns true if this window can have a scroll bar in this orientation.
    ///
    /// [See `wxWindow::CanScroll()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a6b5bbd3bf890f98897187879152ad97a)
    fn can_scroll(&self, orient: c_int) -> bool {
        unsafe { ffi::wxWindow_CanScroll(self.as_ptr(), orient) }
    }
    /// Returns true if this window currently has a scroll bar for this orientation.
    ///
    /// [See `wxWindow::HasScrollbar()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ae50c259b165d9366729734bc124cf184)
    fn has_scrollbar(&self, orient: c_int) -> bool {
        unsafe { ffi::wxWindow_HasScrollbar(self.as_ptr(), orient) }
    }
    /// Return whether a scrollbar is always shown.
    ///
    /// [See `wxWindow::IsScrollbarAlwaysShown()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#abf4f5068ebf648197ed9ca8ca58f38df)
    fn is_scrollbar_always_shown(&self, orient: c_int) -> bool {
        unsafe { ffi::wxWindow_IsScrollbarAlwaysShown(self.as_ptr(), orient) }
    }
    /// Scrolls the window by the given number of lines down (if lines is positive) or up.
    ///
    /// [See `wxWindow::ScrollLines()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aa5c5b683bd11a0d9771bd2fcdf643c64)
    fn scroll_lines(&self, lines: c_int) -> bool {
        unsafe { ffi::wxWindow_ScrollLines(self.as_ptr(), lines) }
    }
    /// Scrolls the window by the given number of pages down (if pages is positive) or up.
    ///
    /// [See `wxWindow::ScrollPages()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#adc0ed5e1c4925223cb901ced14b8343d)
    fn scroll_pages(&self, pages: c_int) -> bool {
        unsafe { ffi::wxWindow_ScrollPages(self.as_ptr(), pages) }
    }
    /// Physically scrolls the pixels in the window and move child windows accordingly.
    ///
    /// [See `wxWindow::ScrollWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ab7be4956ff22da37fff2b8aaa581045c)
    fn scroll_window<R: RectMethods>(&self, dx: c_int, dy: c_int, rect: Option<&R>) {
        unsafe {
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_ScrollWindow(self.as_ptr(), dx, dy, rect)
        }
    }
    /// Same as ScrollLines (-1).
    ///
    /// [See `wxWindow::LineUp()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#afc0816a174ceee6d36d1995c6824a273)
    fn line_up(&self) -> bool {
        unsafe { ffi::wxWindow_LineUp(self.as_ptr()) }
    }
    /// Same as ScrollLines (1).
    ///
    /// [See `wxWindow::LineDown()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ade808784f0e64d9985e2f279b5ca8c02)
    fn line_down(&self) -> bool {
        unsafe { ffi::wxWindow_LineDown(self.as_ptr()) }
    }
    /// Same as ScrollPages (-1).
    ///
    /// [See `wxWindow::PageUp()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aa7c2bbc480d8863d9f139c01d7abc1b1)
    fn page_up(&self) -> bool {
        unsafe { ffi::wxWindow_PageUp(self.as_ptr()) }
    }
    /// Same as ScrollPages (1).
    ///
    /// [See `wxWindow::PageDown()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a61e78cb48ece3e9e989e37b475ac1e35)
    fn page_down(&self) -> bool {
        unsafe { ffi::wxWindow_PageDown(self.as_ptr()) }
    }
    /// Sets the position of one of the built-in scrollbars.
    ///
    /// [See `wxWindow::SetScrollPos()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#afbf4dc9064cf70cfe6884554b97a27bf)
    fn set_scroll_pos(&self, orientation: c_int, pos: c_int, refresh: bool) {
        unsafe { ffi::wxWindow_SetScrollPos(self.as_ptr(), orientation, pos, refresh) }
    }
    /// Sets the scrollbar properties of a built-in scrollbar.
    ///
    /// [See `wxWindow::SetScrollbar()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aa842d59529f873683e55cd8392ec46e9)
    fn set_scrollbar(
        &self,
        orientation: c_int,
        position: c_int,
        thumb_size: c_int,
        range: c_int,
        refresh: bool,
    ) {
        unsafe {
            ffi::wxWindow_SetScrollbar(
                self.as_ptr(),
                orientation,
                position,
                thumb_size,
                range,
                refresh,
            )
        }
    }
    /// Prepare for changing positions of multiple child windows.
    ///
    /// [See `wxWindow::BeginRepositioningChildren()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ab4cea6ace96193b5c4282e097a6fbfee)
    fn begin_repositioning_children(&self) -> bool {
        unsafe { ffi::wxWindow_BeginRepositioningChildren(self.as_ptr()) }
    }
    /// Fix child window positions after setting all of them at once.
    ///
    /// [See `wxWindow::EndRepositioningChildren()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a1203fbd238d781253b44e0e459532301)
    fn end_repositioning_children(&self) {
        unsafe { ffi::wxWindow_EndRepositioningChildren(self.as_ptr()) }
    }
    /// Sets the cached best size value.
    ///
    /// [See `wxWindow::CacheBestSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a11b5d8d94efd0ab52eb2c95a6aa88cdf)
    fn cache_best_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_CacheBestSize(self.as_ptr(), size)
        }
    }
    /// Converts client area size size to corresponding window size.
    ///
    /// [See `wxWindow::ClientToWindowSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a5c72ca7de40d4e99aff55e79aad3962e)
    fn client_to_window_size<S: SizeMethods>(&self, size: &S) -> Size {
        unsafe {
            let size = size.as_ptr();
            Size::from_ptr(ffi::wxWindow_ClientToWindowSize(self.as_ptr(), size))
        }
    }
    /// Converts window size size to corresponding client area size In other words, the returned value is what would GetClientSize() return if this window had given window size.
    ///
    /// [See `wxWindow::WindowToClientSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a75e9c29023e441305574056cd5a725ff)
    fn window_to_client_size<S: SizeMethods>(&self, size: &S) -> Size {
        unsafe {
            let size = size.as_ptr();
            Size::from_ptr(ffi::wxWindow_WindowToClientSize(self.as_ptr(), size))
        }
    }
    /// Sizes the window to fit its best size.
    ///
    /// [See `wxWindow::Fit()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a2bf38a6cbd1f82fb46f274396f482994)
    fn fit(&self) {
        unsafe { ffi::wxWindow_Fit(self.as_ptr()) }
    }
    /// Similar to Fit(), but sizes the interior (virtual) size of a window.
    ///
    /// [See `wxWindow::FitInside()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a55aca401aab29d59f7cc53f89ba2e38d)
    fn fit_inside(&self) {
        unsafe { ffi::wxWindow_FitInside(self.as_ptr()) }
    }
    /// Convert DPI-independent pixel values to the value in pixels appropriate for the current toolkit.
    ///
    /// [See `wxWindow::FromDIP()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a4b0a9da72d28465bdf5c0bfe5661cdf9)
    fn from_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_FromDIP(self.as_ptr(), sz))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::FromDIP()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a560e5daa55869ce65855d29936ca99c5)
    fn from_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_FromDIP1(self.as_ptr(), pt))
        }
    }
    /// Convert DPI-independent distance in pixels to the value in pixels appropriate for the current toolkit.
    ///
    /// [See `wxWindow::FromDIP()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a7d7c6abc9b6fa48022f5e2d67a381bea)
    fn from_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxWindow_FromDIP2(self.as_ptr(), d) }
    }
    /// Convert pixel values of the current toolkit to DPI-independent pixel values.
    ///
    /// [See `wxWindow::ToDIP()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a0100742eee71c42ab33d98dc2422aa5b)
    fn to_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_ToDIP(self.as_ptr(), sz))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::ToDIP()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a896f95ec84dbbae82b03ed26c7e679e8)
    fn to_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ToDIP1(self.as_ptr(), pt))
        }
    }
    /// Convert pixel values of the current toolkit to DPI-independent pixel values.
    ///
    /// [See `wxWindow::ToDIP()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#af6fa0e628ec961c259b4fe40f640a9e5)
    fn to_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxWindow_ToDIP2(self.as_ptr(), d) }
    }
    /// Convert from physical pixels to logical pixels.
    ///
    /// [See `wxWindow::FromPhys()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a144678f0b197268aa9d8d81b76a150a7)
    fn from_phys_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_FromPhys(self.as_ptr(), sz))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::FromPhys()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a048d540d731ac83550c206df4af04270)
    fn from_phys_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_FromPhys1(self.as_ptr(), pt))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::FromPhys()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ac3f185cef9f6a645be63b9b62afae46a)
    fn from_phys_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxWindow_FromPhys2(self.as_ptr(), d) }
    }
    /// Convert from logical pixels to physical pixels.
    ///
    /// [See `wxWindow::ToPhys()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a5d3a9450e6b6f44c6b7d0d4a002fa40f)
    fn to_phys_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_ToPhys(self.as_ptr(), sz))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::ToPhys()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#afca50ba59976ad9913148fffd38ff0ef)
    fn to_phys_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ToPhys1(self.as_ptr(), pt))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::ToPhys()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ad0bea5e21d33d00d0cf19c011bad39bf)
    fn to_phys_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxWindow_ToPhys2(self.as_ptr(), d) }
    }
    /// This functions returns the best acceptable minimal size for the window.
    ///
    /// [See `wxWindow::GetBestSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ae507a81d682023383e465d8e913595b6)
    fn get_best_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetBestSize(self.as_ptr())) }
    }
    /// Returns the best height needed by this window if it had the given width.
    ///
    /// [See `wxWindow::GetBestHeight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a9ec151255cbb44935136d9bbff31a082)
    fn get_best_height(&self, width: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetBestHeight(self.as_ptr(), width) }
    }
    /// Returns the best width needed by this window if it had the given height.
    ///
    /// [See `wxWindow::GetBestWidth()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a5c797a38418a6f2fb79f800112ec73b0)
    fn get_best_width(&self, height: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetBestWidth(self.as_ptr(), height) }
    }
    /// Returns the size of the window 'client area' in pixels.
    ///
    /// [See `wxWindow::GetClientSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ad34d4bbd6d3064aa56e7a2cc3ee97be3)
    fn get_client_size_int(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetClientSize(self.as_ptr(), width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::GetClientSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a13d04f24ab78611f895bc40776118e90)
    fn get_client_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetClientSize1(self.as_ptr())) }
    }
    /// Merges the window's best size into the min size and returns the result.
    ///
    /// [See `wxWindow::GetEffectiveMinSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aa88f6c4946d1ae4821aca652efbf7c62)
    fn get_effective_min_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetEffectiveMinSize(self.as_ptr())) }
    }
    /// Returns the maximum size of window's client area.
    ///
    /// [See `wxWindow::GetMaxClientSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a6690ee82a5a246e070bf0a7dc4bcb5b3)
    fn get_max_client_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetMaxClientSize(self.as_ptr())) }
    }
    /// Returns the maximum size of the window.
    ///
    /// [See `wxWindow::GetMaxSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a107966aa1b2d50107b3b95cf0d7d9901)
    fn get_max_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetMaxSize(self.as_ptr())) }
    }
    /// Returns the minimum size of window's client area, an indication to the sizer layout mechanism that this is the minimum required size of its client area.
    ///
    /// [See `wxWindow::GetMinClientSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a24d22aacd834cbe6cd5b252fa91c3bd9)
    fn get_min_client_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetMinClientSize(self.as_ptr())) }
    }
    /// Returns the minimum size of the window, an indication to the sizer layout mechanism that this is the minimum required size.
    ///
    /// [See `wxWindow::GetMinSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a90ce6ffc8e0af7476157c9e1e6b616df)
    fn get_min_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetMinSize(self.as_ptr())) }
    }
    /// Returns the horizontal component of window minimal size.
    ///
    /// [See `wxWindow::GetMinWidth()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a327e1a8054d604a34cc5473a8c415e4d)
    fn get_min_width(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMinWidth(self.as_ptr()) }
    }
    /// Returns the vertical component of window minimal size.
    ///
    /// [See `wxWindow::GetMinHeight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a81bfd762272d33850c0511d65d896ca3)
    fn get_min_height(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMinHeight(self.as_ptr()) }
    }
    /// Returns the horizontal component of window maximal size.
    ///
    /// [See `wxWindow::GetMaxWidth()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#afde7288b24e48f2d0b5d2c7376f10426)
    fn get_max_width(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMaxWidth(self.as_ptr()) }
    }
    /// Returns the vertical component of window maximal size.
    ///
    /// [See `wxWindow::GetMaxHeight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a98bbe53eddb91b508b0082783402d954)
    fn get_max_height(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMaxHeight(self.as_ptr()) }
    }
    /// Returns the size of the entire window in pixels, including title bar, border, scrollbars, etc.
    ///
    /// [See `wxWindow::GetSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a36ea282508dd55d6f3981ec205ed0449)
    fn get_size_int(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetSize(self.as_ptr(), width, height) }
    }
    /// See the GetSize(int*,int*) overload for more info.
    ///
    /// [See `wxWindow::GetSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a36bf1ac809da1b0cd9c67fd806c4d9b5)
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetSize1(self.as_ptr())) }
    }
    /// This gets the virtual size of the window in pixels.
    ///
    /// [See `wxWindow::GetVirtualSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a557f92e6c66b6654c95315d4461d0e11)
    fn get_virtual_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetVirtualSize(self.as_ptr())) }
    }
    /// Like the other GetVirtualSize() overload but uses pointers instead.
    ///
    /// [See `wxWindow::GetVirtualSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a24cefbb69173899ec4d62460b2932ed9)
    fn get_virtual_size_int(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetVirtualSize1(self.as_ptr(), width, height) }
    }
    /// Return the largest of ClientSize and BestSize (as determined by a sizer, interior children, or other means)
    ///
    /// [See `wxWindow::GetBestVirtualSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a1e4b09c35c0e5495905da45c1580709d)
    fn get_best_virtual_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetBestVirtualSize(self.as_ptr())) }
    }
    /// Returns the factor mapping logical pixels of this window to physical pixels.
    ///
    /// [See `wxWindow::GetContentScaleFactor()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a89cbf94bc15d4e8cb5b1e7db85225577)
    fn get_content_scale_factor(&self) -> c_double {
        unsafe { ffi::wxWindow_GetContentScaleFactor(self.as_ptr()) }
    }
    /// Returns the ratio of the DPI used by this window to the standard DPI.
    ///
    /// [See `wxWindow::GetDPIScaleFactor()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a43bfd56d59b728157f95c838f981872c)
    fn get_dpi_scale_factor(&self) -> c_double {
        unsafe { ffi::wxWindow_GetDPIScaleFactor(self.as_ptr()) }
    }
    /// Returns the size of the left/right and top/bottom borders of this window in x and y components of the result respectively.
    ///
    /// [See `wxWindow::GetWindowBorderSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a21c97d5bcf61c4c664bd33066f5567b1)
    fn get_window_border_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetWindowBorderSize(self.as_ptr())) }
    }
    /// wxSizer and friends use this to give a chance to a component to recalc its min size once one of the final size components is known.
    ///
    /// [See `wxWindow::InformFirstDirection()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a9fd5b6520c1b30eb8e82bb5d56bc24c0)
    fn inform_first_direction(
        &self,
        direction: c_int,
        size: c_int,
        available_other_dir: c_int,
    ) -> bool {
        unsafe {
            ffi::wxWindow_InformFirstDirection(self.as_ptr(), direction, size, available_other_dir)
        }
    }
    /// Resets the cached best size value so it will be recalculated the next time it is needed.
    ///
    /// [See `wxWindow::InvalidateBestSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ae1b56ca87d8590ee5e576012229a380a)
    fn invalidate_best_size(&self) {
        unsafe { ffi::wxWindow_InvalidateBestSize(self.as_ptr()) }
    }
    /// Posts a size event to the window.
    ///
    /// [See `wxWindow::PostSizeEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ab1943463e6661f97e072b43337c6cc09)
    fn post_size_event(&self) {
        unsafe { ffi::wxWindow_PostSizeEvent(self.as_ptr()) }
    }
    /// Posts a size event to the parent of this window.
    ///
    /// [See `wxWindow::PostSizeEventToParent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aa080d8dcda58bcc6ea2abd8bea592e3e)
    fn post_size_event_to_parent(&self) {
        unsafe { ffi::wxWindow_PostSizeEventToParent(self.as_ptr()) }
    }
    /// This function sends a dummy size event to the window allowing it to re-layout its children positions.
    ///
    /// [See `wxWindow::SendSizeEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a237f739b21937d3e8f1bff5fa82ba4c2)
    fn send_size_event(&self, flags: c_int) {
        unsafe { ffi::wxWindow_SendSizeEvent(self.as_ptr(), flags) }
    }
    /// Safe wrapper for GetParent()->SendSizeEvent().
    ///
    /// [See `wxWindow::SendSizeEventToParent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#af7987987978fd8a93df88b29b19a6388)
    fn send_size_event_to_parent(&self, flags: c_int) {
        unsafe { ffi::wxWindow_SendSizeEventToParent(self.as_ptr(), flags) }
    }
    /// This sets the size of the window client area in pixels.
    ///
    /// [See `wxWindow::SetClientSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aa59f715217fffa5bcf14cd97f92e7840)
    fn set_client_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxWindow_SetClientSize(self.as_ptr(), width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::SetClientSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ab2aadc857ee7f55f47ab9a8669e3beb7)
    fn set_client_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetClientSize1(self.as_ptr(), size)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::SetClientSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a582d4d1f60a3f777627773b2da0bb2ef)
    fn set_client_size_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_SetClientSize2(self.as_ptr(), rect)
        }
    }
    /// Used by wxSizer internally to notify the window about being managed by the given sizer.
    ///
    /// [See `wxWindow::SetContainingSizer()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a0ccf78fe06722b500adfb7f36b8ce443)
    fn set_containing_sizer<S: SizerMethods>(&self, sizer: Option<&S>) {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetContainingSizer(self.as_ptr(), sizer)
        }
    }
    /// A smart SetSize that will fill in default size components with the window's best size values.
    ///
    /// [See `wxWindow::SetInitialSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a1b309ca50ba87e34f968c83b79af1397)
    fn set_initial_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetInitialSize(self.as_ptr(), size)
        }
    }
    /// Sets the maximum client size of the window, to indicate to the sizer layout mechanism that this is the maximum possible size of its client area.
    ///
    /// [See `wxWindow::SetMaxClientSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a664e5b2ddd817d9c58788269fe1d8479)
    fn set_max_client_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMaxClientSize(self.as_ptr(), size)
        }
    }
    /// Sets the maximum size of the window, to indicate to the sizer layout mechanism that this is the maximum possible size.
    ///
    /// [See `wxWindow::SetMaxSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a38b496214d728a3212afadee5ed51606)
    fn set_max_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMaxSize(self.as_ptr(), size)
        }
    }
    /// Sets the minimum client size of the window, to indicate to the sizer layout mechanism that this is the minimum required size of window's client area.
    ///
    /// [See `wxWindow::SetMinClientSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a6e35ba44b97e374dfffa460d41d94b31)
    fn set_min_client_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMinClientSize(self.as_ptr(), size)
        }
    }
    /// Sets the minimum size of the window, to indicate to the sizer layout mechanism that this is the minimum required size.
    ///
    /// [See `wxWindow::SetMinSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a3fc066f4d8083319f004ac43811d545d)
    fn set_min_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMinSize(self.as_ptr(), size)
        }
    }
    /// Sets the size of the window in pixels.
    ///
    /// [See `wxWindow::SetSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a180312d5ad4a4a5ad805b8d52d67a74e)
    fn set_size_int_int(&self, x: c_int, y: c_int, width: c_int, height: c_int, size_flags: c_int) {
        unsafe { ffi::wxWindow_SetSize(self.as_ptr(), x, y, width, height, size_flags) }
    }
    /// Sets the size of the window in pixels.
    ///
    /// [See `wxWindow::SetSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a8e383bc6d5ca008965922a36c676aea0)
    fn set_size_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_SetSize1(self.as_ptr(), rect)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::SetSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a030a928cd854de3def97c5720f14695b)
    fn set_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetSize2(self.as_ptr(), size)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::SetSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a2ea9b25296d591aea4470c8fd99ff7cb)
    fn set_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxWindow_SetSize3(self.as_ptr(), width, height) }
    }
    /// Use of this function for windows which are not toplevel windows (such as wxDialog or wxFrame) is discouraged.
    ///
    /// [See `wxWindow::SetSizeHints()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a307329dc3b10f5584aeb2cbce9293ffd)
    fn set_size_hints_size<S: SizeMethods, S2: SizeMethods, S3: SizeMethods>(
        &self,
        min_size: &S,
        max_size: &S2,
        inc_size: &S3,
    ) {
        unsafe {
            let min_size = min_size.as_ptr();
            let max_size = max_size.as_ptr();
            let inc_size = inc_size.as_ptr();
            ffi::wxWindow_SetSizeHints(self.as_ptr(), min_size, max_size, inc_size)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::SetSizeHints()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ae813c640e1e2bc6014423247050846cf)
    fn set_size_hints_int(
        &self,
        min_w: c_int,
        min_h: c_int,
        max_w: c_int,
        max_h: c_int,
        inc_w: c_int,
        inc_h: c_int,
    ) {
        unsafe {
            ffi::wxWindow_SetSizeHints1(self.as_ptr(), min_w, min_h, max_w, max_h, inc_w, inc_h)
        }
    }
    /// Sets the virtual size of the window in pixels.
    ///
    /// [See `wxWindow::SetVirtualSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a37f293b7904bc6668b86cccb0aea5669)
    fn set_virtual_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxWindow_SetVirtualSize(self.as_ptr(), width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::SetVirtualSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a8e95201edebe43b9623bd3bdc555af4d)
    fn set_virtual_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetVirtualSize1(self.as_ptr(), size)
        }
    }
    /// Non window-specific DPI-independent pixels conversion functions.
    ///
    /// [See `wxWindow::FromDIP()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a71f45373a9fc55e128d263391093c32a)
    fn from_dip_size_window<S: SizeMethods, W: WindowMethods>(sz: &S, w: Option<&W>) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxWindow_FromDIP3(sz, w))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::FromDIP()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ad122671fcef564f7e11658c3ee0d4d87)
    fn from_dip_point_window<P: PointMethods, W: WindowMethods>(pt: &P, w: Option<&W>) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Point::from_ptr(ffi::wxWindow_FromDIP4(pt, w))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::FromDIP()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aee63c8b1f8055fef76962578aa7f38c1)
    fn from_dip_int_window<W: WindowMethods>(d: c_int, w: Option<&W>) -> c_int {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FromDIP5(d, w)
        }
    }
    /// Non window-specific pixel to DPI-independent pixels conversion functions.
    ///
    /// [See `wxWindow::ToDIP()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a664dc49139d0f91fc3717994b0a5b4e6)
    fn to_dip_size_window<S: SizeMethods, W: WindowMethods>(sz: &S, w: Option<&W>) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxWindow_ToDIP3(sz, w))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::ToDIP()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aef413f1d0252a7a5a29512e8ab5e7c91)
    fn to_dip_point_window<P: PointMethods, W: WindowMethods>(pt: &P, w: Option<&W>) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Point::from_ptr(ffi::wxWindow_ToDIP4(pt, w))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::ToDIP()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a1771a46c000060619f6e1249ce362433)
    fn to_dip_int_window<W: WindowMethods>(d: c_int, w: Option<&W>) -> c_int {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_ToDIP5(d, w)
        }
    }
    /// Convert from physical pixels to logical pixels for any window.
    ///
    /// [See `wxWindow::FromPhys()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a2fe6412d36c2b81c0dd3eeb1dd8bd004)
    fn from_phys_size_window<S: SizeMethods, W: WindowMethods>(sz: &S, w: Option<&W>) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxWindow_FromPhys3(sz, w))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::FromPhys()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#af1ddb23778f531a56f2e1aa937a66d9c)
    fn from_phys_point_window<P: PointMethods, W: WindowMethods>(pt: &P, w: Option<&W>) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Point::from_ptr(ffi::wxWindow_FromPhys4(pt, w))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::FromPhys()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a27efab91147b0ffeee600f02059c23fc)
    fn from_phys_int_window<W: WindowMethods>(d: c_int, w: Option<&W>) -> c_int {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FromPhys5(d, w)
        }
    }
    /// Convert from logical pixels to physical pixels for any window.
    ///
    /// [See `wxWindow::ToPhys()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aaf0ac2191826f233f04bffa900dbc357)
    fn to_phys_size_window<S: SizeMethods, W: WindowMethods>(sz: &S, w: Option<&W>) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxWindow_ToPhys3(sz, w))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::ToPhys()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aef56d76b1cacdca83daebffcc385d6db)
    fn to_phys_point_window<P: PointMethods, W: WindowMethods>(pt: &P, w: Option<&W>) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Point::from_ptr(ffi::wxWindow_ToPhys4(pt, w))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::ToPhys()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a6797f4c9656276796031b9846ecc20b6)
    fn to_phys_int_window<W: WindowMethods>(d: c_int, w: Option<&W>) -> c_int {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_ToPhys5(d, w)
        }
    }
    /// A synonym for Centre().
    ///
    /// [See `wxWindow::Center()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a06c0ecb262995b40083bfb446a6cff99)
    fn center(&self, dir: c_int) {
        unsafe { ffi::wxWindow_Center(self.as_ptr(), dir) }
    }
    /// A synonym for CentreOnParent().
    ///
    /// [See `wxWindow::CenterOnParent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a2dc4e0a85d33fc55cc9650eaea1da0a4)
    fn center_on_parent(&self, dir: c_int) {
        unsafe { ffi::wxWindow_CenterOnParent(self.as_ptr(), dir) }
    }
    /// Centres the window.
    ///
    /// [See `wxWindow::Centre()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a4a1819eeee3f2143cdde4f329ffde787)
    fn centre(&self, direction: c_int) {
        unsafe { ffi::wxWindow_Centre(self.as_ptr(), direction) }
    }
    /// Centres the window on its parent.
    ///
    /// [See `wxWindow::CentreOnParent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ab8e9b91b0e2db344fd71259616dfd433)
    fn centre_on_parent(&self, direction: c_int) {
        unsafe { ffi::wxWindow_CentreOnParent(self.as_ptr(), direction) }
    }
    /// This gets the position of the window in pixels, relative to the parent window for the child windows or relative to the display origin for the top level windows.
    ///
    /// [See `wxWindow::GetPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ac2eece00a4b4b83e1433b59a5d31584f)
    fn get_position_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_GetPosition(self.as_ptr(), x, y) }
    }
    /// This gets the position of the window in pixels, relative to the parent window for the child windows or relative to the display origin for the top level windows.
    ///
    /// [See `wxWindow::GetPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#abfeeca6db664c24603f6371811397e0d)
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxWindow_GetPosition1(self.as_ptr())) }
    }
    /// Returns the position and size of the window as a wxRect object.
    ///
    /// [See `wxWindow::GetRect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a68e83a724887b18525fdbd947b2d8c88)
    fn get_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxWindow_GetRect(self.as_ptr())) }
    }
    /// Returns the window position in screen coordinates, whether the window is a child window or a top level one.
    ///
    /// [See `wxWindow::GetScreenPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a285a81f00ed59fa09938343708938566)
    fn get_screen_position_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_GetScreenPosition(self.as_ptr(), x, y) }
    }
    /// Returns the window position in screen coordinates, whether the window is a child window or a top level one.
    ///
    /// [See `wxWindow::GetScreenPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a5786c420fd2958bde726c923621700aa)
    fn get_screen_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxWindow_GetScreenPosition1(self.as_ptr())) }
    }
    /// Returns the position and size of the window on the screen as a wxRect object.
    ///
    /// [See `wxWindow::GetScreenRect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ae8f6c4a201650ad63bb5ebe4f35eb41e)
    fn get_screen_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxWindow_GetScreenRect(self.as_ptr())) }
    }
    /// Get the origin of the client area of the window relative to the window top left corner (the client area may be shifted because of the borders, scrollbars, other decorations...)
    ///
    /// [See `wxWindow::GetClientAreaOrigin()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a4a1b567002d1039bc630885fc09808ae)
    fn get_client_area_origin(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxWindow_GetClientAreaOrigin(self.as_ptr())) }
    }
    /// Get the client rectangle in window (i.e. client) coordinates.
    ///
    /// [See `wxWindow::GetClientRect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a5c6428c6aa8634b5c5963d1d0eaa75c7)
    fn get_client_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxWindow_GetClientRect(self.as_ptr())) }
    }
    /// Moves the window to the given position.
    ///
    /// [See `wxWindow::Move()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ab1cb98c8d25b9e6ff7b706b1446c3df7)
    fn move_int(&self, x: c_int, y: c_int, flags: c_int) {
        unsafe { ffi::wxWindow_Move(self.as_ptr(), x, y, flags) }
    }
    /// Moves the window to the given position.
    ///
    /// [See `wxWindow::Move()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a22a22c1e23ca05776707e7999d8047fe)
    fn move_point<P: PointMethods>(&self, pt: &P, flags: c_int) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxWindow_Move1(self.as_ptr(), pt, flags)
        }
    }
    /// Moves the window to the specified position.
    ///
    /// [See `wxWindow::SetPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a81f23590239934fa10fded0566a65d8c)
    fn set_position<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxWindow_SetPosition(self.as_ptr(), pt)
        }
    }
    /// Converts to screen coordinates from coordinates relative to this window.
    ///
    /// [See `wxWindow::ClientToScreen()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#af5e852c8695297c0328f1dfe3908605a)
    fn client_to_screen_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_ClientToScreen(self.as_ptr(), x, y) }
    }
    /// Converts to screen coordinates from coordinates relative to this window.
    ///
    /// [See `wxWindow::ClientToScreen()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#add6f5472d0d8fe0074145a886568721c)
    fn client_to_screen_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ClientToScreen1(self.as_ptr(), pt))
        }
    }
    /// Converts a point or size from dialog units to pixels.
    ///
    /// [See `wxWindow::ConvertDialogToPixels()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a3dbe034120d742c4f5f6d64dc5d69590)
    fn convert_dialog_to_pixels_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ConvertDialogToPixels(self.as_ptr(), pt))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::ConvertDialogToPixels()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ad70d63964459bb21f81bc0b1f09e84f4)
    fn convert_dialog_to_pixels_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_ConvertDialogToPixels1(self.as_ptr(), sz))
        }
    }
    /// Converts a point or size from pixels to dialog units.
    ///
    /// [See `wxWindow::ConvertPixelsToDialog()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a321de67666eff37b39556802c8029201)
    fn convert_pixels_to_dialog_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ConvertPixelsToDialog(self.as_ptr(), pt))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::ConvertPixelsToDialog()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a533552c4673c7a11f05bf9261b4dff5c)
    fn convert_pixels_to_dialog_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_ConvertPixelsToDialog1(self.as_ptr(), sz))
        }
    }
    /// Converts from screen to client window coordinates.
    ///
    /// [See `wxWindow::ScreenToClient()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a09aca554d41d6e771d3dc72f26b9cacc)
    fn screen_to_client_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_ScreenToClient(self.as_ptr(), x, y) }
    }
    /// Converts from screen to client window coordinates.
    ///
    /// [See `wxWindow::ScreenToClient()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a5f4dab599fda780823841bb2fe5f2f99)
    fn screen_to_client_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ScreenToClient1(self.as_ptr(), pt))
        }
    }
    /// Clears the window by filling it with the current background colour.
    ///
    /// [See `wxWindow::ClearBackground()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a61e833684ee5c89775e91e88be1a9a52)
    fn clear_background(&self) {
        unsafe { ffi::wxWindow_ClearBackground(self.as_ptr()) }
    }
    /// Freezes the window or, in other words, prevents any updates from taking place on screen, the window is not redrawn at all.
    ///
    /// [See `wxWindow::Freeze()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a15c678314cfc1d807196bc298b713ed3)
    fn freeze(&self) {
        unsafe { ffi::wxWindow_Freeze(self.as_ptr()) }
    }
    /// Re-enables window updating after a previous call to Freeze().
    ///
    /// [See `wxWindow::Thaw()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a279a532124073261b28131b6afb59a1e)
    fn thaw(&self) {
        unsafe { ffi::wxWindow_Thaw(self.as_ptr()) }
    }
    /// Returns true if the window is currently frozen by a call to Freeze().
    ///
    /// [See `wxWindow::IsFrozen()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#adf34f121a9a94fd7159a8818355f2b67)
    fn is_frozen(&self) -> bool {
        unsafe { ffi::wxWindow_IsFrozen(self.as_ptr()) }
    }
    /// Returns the background colour of the window.
    ///
    /// [See `wxWindow::GetBackgroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#af51668ec32e9e44db45574a15ec215b4)
    fn get_background_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxWindow_GetBackgroundColour(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetBackgroundStyle()
    /// Returns the character height for this window.
    ///
    /// [See `wxWindow::GetCharHeight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aa488520238094f858bea3c3a2c6b9809)
    fn get_char_height(&self) -> c_int {
        unsafe { ffi::wxWindow_GetCharHeight(self.as_ptr()) }
    }
    /// Returns the average character width for this window.
    ///
    /// [See `wxWindow::GetCharWidth()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a8f92c15635a44b85392d16f5906c31ef)
    fn get_char_width(&self) -> c_int {
        unsafe { ffi::wxWindow_GetCharWidth(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetDefaultAttributes()
    /// Return the DPI of the display used by this window.
    ///
    /// [See `wxWindow::GetDPI()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a391a91e5faa5b64d52e3461d5cf5666b)
    fn get_dpi(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetDPI(self.as_ptr())) }
    }
    /// Returns the font for this window.
    ///
    /// [See `wxWindow::GetFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#af25f8a799106d61b50d3ee796ba43728)
    fn get_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxWindow_GetFont(self.as_ptr())) }
    }
    /// Returns the foreground colour of the window.
    ///
    /// [See `wxWindow::GetForegroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ad9a607b86d9175d395f245ecbe00f38b)
    fn get_foreground_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxWindow_GetForegroundColour(self.as_ptr())) }
    }
    /// Gets the dimensions of the string as it would be drawn on the window with the currently selected font.
    ///
    /// [See `wxWindow::GetTextExtent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a4220668e42f3a173bf29d335da4db2c3)
    fn get_text_extent_int<F: FontMethods>(
        &self,
        string: &str,
        w: *mut c_void,
        h: *mut c_void,
        descent: *mut c_void,
        external_leading: *mut c_void,
        font: Option<&F>,
    ) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            let font = match font {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_GetTextExtent(
                self.as_ptr(),
                string,
                w,
                h,
                descent,
                external_leading,
                font,
            )
        }
    }
    /// Gets the dimensions of the string as it would be drawn on the window with the currently selected font.
    ///
    /// [See `wxWindow::GetTextExtent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a69fc79069821a633eba9c5d917fd8b7b)
    fn get_text_extent(&self, string: &str) -> Size {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            Size::from_ptr(ffi::wxWindow_GetTextExtent1(self.as_ptr(), string))
        }
    }
    // BLOCKED: fn GetUpdateRegion()
    /// Get the update rectangle bounding box in client coords.
    ///
    /// [See `wxWindow::GetUpdateClientRect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a23370688a951ed60ac25146af854d2ec)
    fn get_update_client_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxWindow_GetUpdateClientRect(self.as_ptr())) }
    }
    /// Returns true if this window background is transparent (as, for example, for wxStaticText) and should show the parent window background.
    ///
    /// [See `wxWindow::HasTransparentBackground()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ab9cbcb6d6f4a272c2f0342e69a13b59a)
    fn has_transparent_background(&self) -> bool {
        unsafe { ffi::wxWindow_HasTransparentBackground(self.as_ptr()) }
    }
    /// Causes this window, and all of its children recursively, to be repainted.
    ///
    /// [See `wxWindow::Refresh()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a29dc7251746154c821b17841b9877830)
    fn refresh<R: RectMethods>(&self, erase_background: bool, rect: Option<&R>) {
        unsafe {
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_Refresh(self.as_ptr(), erase_background, rect)
        }
    }
    /// Redraws the contents of the given rectangle: only the area inside it will be repainted.
    ///
    /// [See `wxWindow::RefreshRect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ab0ae6b9898cd261c39ebeb2891aa3d67)
    fn refresh_rect<R: RectMethods>(&self, rect: &R, erase_background: bool) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_RefreshRect(self.as_ptr(), rect, erase_background)
        }
    }
    /// Calling this method immediately repaints the invalidated area of the window and all of its children recursively (this normally only happens when the flow of control returns to the event loop).
    ///
    /// [See `wxWindow::Update()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#abaf28f1a075fd1b10f761a8febe597ec)
    fn update(&self) {
        unsafe { ffi::wxWindow_Update(self.as_ptr()) }
    }
    /// Sets the background colour of the window.
    ///
    /// [See `wxWindow::SetBackgroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a37219df52734626e23401fd83b25d8a0)
    fn set_background_colour<C: ColourMethods>(&self, colour: &C) -> bool {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWindow_SetBackgroundColour(self.as_ptr(), colour)
        }
    }
    // NOT_SUPPORTED: fn SetBackgroundStyle()
    /// Checks whether using transparent background might work.
    ///
    /// [See `wxWindow::IsTransparentBackgroundSupported()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a21c6e2be327062520d9d0aae55b1e8b0)
    fn is_transparent_background_supported(&self, reason: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_IsTransparentBackgroundSupported(self.as_ptr(), reason) }
    }
    /// Sets the font for this window.
    ///
    /// [See `wxWindow::SetFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a9ab11e7da57a1d08918aa75fc33f6ad3)
    fn set_font<F: FontMethods>(&self, font: &F) -> bool {
        unsafe {
            let font = font.as_ptr();
            ffi::wxWindow_SetFont(self.as_ptr(), font)
        }
    }
    /// Sets the foreground colour of the window.
    ///
    /// [See `wxWindow::SetForegroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a69f1e1c7ddd370d72e68c70f13ac8de9)
    fn set_foreground_colour<C: ColourMethods>(&self, colour: &C) -> bool {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWindow_SetForegroundColour(self.as_ptr(), colour)
        }
    }
    /// Sets the background colour of the window but prevents it from being inherited by the children of this window.
    ///
    /// [See `wxWindow::SetOwnBackgroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a9a3f9d8477aab1d9176cd66ee56e75d9)
    fn set_own_background_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWindow_SetOwnBackgroundColour(self.as_ptr(), colour)
        }
    }
    /// Return true if this window inherits the background colour from its parent.
    ///
    /// [See `wxWindow::InheritsBackgroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#af05b95cfeed0bee6f44797572367b26e)
    fn inherits_background_colour(&self) -> bool {
        unsafe { ffi::wxWindow_InheritsBackgroundColour(self.as_ptr()) }
    }
    /// Return true if a background colour has been set for this window.
    ///
    /// [See `wxWindow::UseBgCol()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a9131e424ddc3b332a08377e5ad713c60)
    fn use_bg_col(&self) -> bool {
        unsafe { ffi::wxWindow_UseBgCol(self.as_ptr()) }
    }
    /// Return true if a background colour has been set for this window.
    ///
    /// [See `wxWindow::UseBackgroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ab8f896f4af38b2a97717daa54f26cb6c)
    fn use_background_colour(&self) -> bool {
        unsafe { ffi::wxWindow_UseBackgroundColour(self.as_ptr()) }
    }
    /// Sets the font of the window but prevents it from being inherited by the children of this window.
    ///
    /// [See `wxWindow::SetOwnFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a89a4f62f23c1e7c845b8d07cecae4c43)
    fn set_own_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxWindow_SetOwnFont(self.as_ptr(), font)
        }
    }
    /// Sets the foreground colour of the window but prevents it from being inherited by the children of this window.
    ///
    /// [See `wxWindow::SetOwnForegroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a53f4a878e4e2d440bd00543f8014aaaa)
    fn set_own_foreground_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWindow_SetOwnForegroundColour(self.as_ptr(), colour)
        }
    }
    /// Return true if a foreground colour has been set for this window.
    ///
    /// [See `wxWindow::UseForegroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ad2403e81d23e4a76d2176a6e0570de3e)
    fn use_foreground_colour(&self) -> bool {
        unsafe { ffi::wxWindow_UseForegroundColour(self.as_ptr()) }
    }
    /// Return true if this window inherits the foreground colour from its parent.
    ///
    /// [See `wxWindow::InheritsForegroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a5db9556be7eccf8310f8d5653235a4e6)
    fn inherits_foreground_colour(&self) -> bool {
        unsafe { ffi::wxWindow_InheritsForegroundColour(self.as_ptr()) }
    }
    ///
    /// [See `wxWindow::SetPalette()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aee57358435d6bd33f598c81354b47425)
    fn set_palette<P: PaletteMethods>(&self, pal: &P) {
        unsafe {
            let pal = pal.as_ptr();
            ffi::wxWindow_SetPalette(self.as_ptr(), pal)
        }
    }
    /// Return true from here to allow the colours of this window to be changed by InheritAttributes().
    ///
    /// [See `wxWindow::ShouldInheritColours()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aa51b169745cda1746c1a45b4ffe3217a)
    fn should_inherit_colours(&self) -> bool {
        unsafe { ffi::wxWindow_ShouldInheritColours(self.as_ptr()) }
    }
    /// This function tells a window if it should use the system's "theme" code to draw the windows' background instead of its own background drawing code.
    ///
    /// [See `wxWindow::SetThemeEnabled()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a41dd19ed8809fd8ec662e2aa2a9579c3)
    fn set_theme_enabled(&self, enable: bool) {
        unsafe { ffi::wxWindow_SetThemeEnabled(self.as_ptr(), enable) }
    }
    /// Returns true if the window uses the system theme for drawing its background.
    ///
    /// [See `wxWindow::GetThemeEnabled()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a578145344f92cfee755a9c87f6703432)
    fn get_theme_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_GetThemeEnabled(self.as_ptr()) }
    }
    /// Returns true if the system supports transparent windows and calling SetTransparent() may succeed.
    ///
    /// [See `wxWindow::CanSetTransparent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a64f7f6fb75bf4b7281e1d33542d523c7)
    fn can_set_transparent(&self) -> bool {
        unsafe { ffi::wxWindow_CanSetTransparent(self.as_ptr()) }
    }
    /// Set the transparency of the window.
    ///
    /// [See `wxWindow::SetTransparent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ac8cf4398cec50ac36634760f45a0656f)
    fn set_transparent(&self, alpha: c_uchar) -> bool {
        unsafe { ffi::wxWindow_SetTransparent(self.as_ptr(), alpha) }
    }
    /// Returns the event handler for this window.
    ///
    /// [See `wxWindow::GetEventHandler()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a1e421cef85f7d0fb857b1a8317e185ab)
    fn get_event_handler(&self) -> WeakRef<EvtHandler> {
        unsafe { WeakRef::<EvtHandler>::from(ffi::wxWindow_GetEventHandler(self.as_ptr())) }
    }
    /// This function will generate the appropriate call to Navigate() if the key event is one normally used for keyboard navigation and return true in this case.
    ///
    /// [See `wxWindow::HandleAsNavigationKey()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a89d1b079de97aac170e999692095872c)
    fn handle_as_navigation_key<K: KeyEventMethods>(&self, event: &K) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxWindow_HandleAsNavigationKey(self.as_ptr(), event)
        }
    }
    /// Shorthand for:
    ///
    /// [See `wxWindow::HandleWindowEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a89816f1b78c125c59418463caea35c9a)
    fn handle_window_event<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxWindow_HandleWindowEvent(self.as_ptr(), event)
        }
    }
    /// Convenient wrapper for ProcessEvent().
    ///
    /// [See `wxWindow::ProcessWindowEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a0a76891d726ad5f9a729e27e4eab2b57)
    fn process_window_event<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxWindow_ProcessWindowEvent(self.as_ptr(), event)
        }
    }
    /// Wrapper for wxEvtHandler::ProcessEventLocally().
    ///
    /// [See `wxWindow::ProcessWindowEventLocally()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a0a11aa4a50491d051f947e2618e52178)
    fn process_window_event_locally<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxWindow_ProcessWindowEventLocally(self.as_ptr(), event)
        }
    }
    /// Removes and returns the top-most event handler on the event handler stack.
    ///
    /// [See `wxWindow::PopEventHandler()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a0b71a51a2f13179d0ca1b488d177ca4d)
    fn pop_event_handler(&self, delete_handler: bool) -> WeakRef<EvtHandler> {
        unsafe {
            WeakRef::<EvtHandler>::from(ffi::wxWindow_PopEventHandler(
                self.as_ptr(),
                delete_handler,
            ))
        }
    }
    /// Pushes this event handler onto the event stack for the window.
    ///
    /// [See `wxWindow::PushEventHandler()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a398c11ab9af7956067a964f560d1978c)
    fn push_event_handler<E: EvtHandlerMethods>(&self, handler: Option<&E>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_PushEventHandler(self.as_ptr(), handler)
        }
    }
    /// Find the given handler in the windows event handler stack and removes (but does not delete) it from the stack.
    ///
    /// [See `wxWindow::RemoveEventHandler()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aacbfe424fa527966b954229a1db96ab5)
    fn remove_event_handler<E: EvtHandlerMethods>(&self, handler: Option<&E>) -> bool {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_RemoveEventHandler(self.as_ptr(), handler)
        }
    }
    /// Sets the event handler for this window.
    ///
    /// [See `wxWindow::SetEventHandler()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#af6c84b7679183b377ba27a52a2f708b4)
    fn set_event_handler<E: EvtHandlerMethods>(&self, handler: Option<&E>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetEventHandler(self.as_ptr(), handler)
        }
    }
    /// Returns the extra style bits for the window.
    ///
    /// [See `wxWindow::GetExtraStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a0db5cd18ab5166b44da9d07f92bb5070)
    fn get_extra_style(&self) -> c_long {
        unsafe { ffi::wxWindow_GetExtraStyle(self.as_ptr()) }
    }
    /// Gets the window style that was passed to the constructor or Create() method.
    ///
    /// [See `wxWindow::GetWindowStyleFlag()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ad67a731cd937a21e561593d4a0c44979)
    fn get_window_style_flag(&self) -> c_long {
        unsafe { ffi::wxWindow_GetWindowStyleFlag(self.as_ptr()) }
    }
    /// See GetWindowStyleFlag() for more info.
    ///
    /// [See `wxWindow::GetWindowStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a418007123fec131b47d9841ac6d34891)
    fn get_window_style(&self) -> c_long {
        unsafe { ffi::wxWindow_GetWindowStyle(self.as_ptr()) }
    }
    /// Returns true if the window has the given exFlag bit set in its extra styles.
    ///
    /// [See `wxWindow::HasExtraStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aeee5333e672a3a510db48d2af37ddee9)
    fn has_extra_style(&self, ex_flag: c_int) -> bool {
        unsafe { ffi::wxWindow_HasExtraStyle(self.as_ptr(), ex_flag) }
    }
    /// Returns true if the window has the given flag bit set.
    ///
    /// [See `wxWindow::HasFlag()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a3d1d00b4cd51fa3c187e9d609d022aa4)
    fn has_flag(&self, flag: c_int) -> bool {
        unsafe { ffi::wxWindow_HasFlag(self.as_ptr(), flag) }
    }
    /// Sets the extra style bits for the window.
    ///
    /// [See `wxWindow::SetExtraStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ae9655f7c35ce7ac89cac2f6c0054b103)
    fn set_extra_style(&self, ex_style: c_long) {
        unsafe { ffi::wxWindow_SetExtraStyle(self.as_ptr(), ex_style) }
    }
    /// Sets the style of the window.
    ///
    /// [See `wxWindow::SetWindowStyleFlag()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aee2cf342f80523432e7f2299d299451b)
    fn set_window_style_flag(&self, style: c_long) {
        unsafe { ffi::wxWindow_SetWindowStyleFlag(self.as_ptr(), style) }
    }
    /// See SetWindowStyleFlag() for more info.
    ///
    /// [See `wxWindow::SetWindowStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a306af30adec68689f74ed537b4f9d5fd)
    fn set_window_style(&self, style: c_long) {
        unsafe { ffi::wxWindow_SetWindowStyle(self.as_ptr(), style) }
    }
    /// Turns the given flag on if it's currently turned off and vice versa.
    ///
    /// [See `wxWindow::ToggleWindowStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a8a4fa47129de552bfec37db8c69688a2)
    fn toggle_window_style(&self, flag: c_int) -> bool {
        unsafe { ffi::wxWindow_ToggleWindowStyle(self.as_ptr(), flag) }
    }
    /// Moves this window in the tab navigation order after the specified win.
    ///
    /// [See `wxWindow::MoveAfterInTabOrder()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a79e66079125e8420de269811bdb6f2b6)
    fn move_after_in_tab_order<W: WindowMethods>(&self, win: Option<&W>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_MoveAfterInTabOrder(self.as_ptr(), win)
        }
    }
    /// Same as MoveAfterInTabOrder() except that it inserts this window just before win instead of putting it right after it.
    ///
    /// [See `wxWindow::MoveBeforeInTabOrder()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#af2b92f61cd9f9e2e0efe4cce307e25b1)
    fn move_before_in_tab_order<W: WindowMethods>(&self, win: Option<&W>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_MoveBeforeInTabOrder(self.as_ptr(), win)
        }
    }
    /// Performs a keyboard navigation action starting from this window.
    ///
    /// [See `wxWindow::Navigate()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a86904f6785df4af6036b33383490a805)
    fn navigate(&self, flags: c_int) -> bool {
        unsafe { ffi::wxWindow_Navigate(self.as_ptr(), flags) }
    }
    /// Performs a keyboard navigation action inside this window.
    ///
    /// [See `wxWindow::NavigateIn()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aa58039c8fc65e19160becf510ee1d1d5)
    fn navigate_in(&self, flags: c_int) -> bool {
        unsafe { ffi::wxWindow_NavigateIn(self.as_ptr(), flags) }
    }
    /// Lowers the window to the bottom of the window hierarchy (Z-order).
    ///
    /// [See `wxWindow::Lower()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a124944524f36b71385dad8fddaad8ded)
    fn lower(&self) {
        unsafe { ffi::wxWindow_Lower(self.as_ptr()) }
    }
    /// Raises the window to the top of the window hierarchy (Z-order).
    ///
    /// [See `wxWindow::Raise()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a54808c933f22a891c5db646f6209fa4d)
    fn raise(&self) {
        unsafe { ffi::wxWindow_Raise(self.as_ptr()) }
    }
    /// Equivalent to calling wxWindow::Show(false).
    ///
    /// [See `wxWindow::Hide()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a7ed103df04014cb3c59c6a3fb4d95328)
    fn hide(&self) -> bool {
        unsafe { ffi::wxWindow_Hide(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn HideWithEffect()
    /// Returns true if the window is enabled, i.e. if it accepts user input, false otherwise.
    ///
    /// [See `wxWindow::IsEnabled()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a0c186513884fb2020c6af3c62f0913d2)
    fn is_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_IsEnabled(self.as_ptr()) }
    }
    /// Returns true if the given point or rectangle area has been exposed since the last repaint.
    ///
    /// [See `wxWindow::IsExposed()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a687120c4b7095481bbc6d483187d0c56)
    fn is_exposed_int(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxWindow_IsExposed(self.as_ptr(), x, y) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::IsExposed()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#afde1844b856787b95afbb686aaaa206d)
    fn is_exposed_point<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxWindow_IsExposed1(self.as_ptr(), pt)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::IsExposed()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a3017e9cdde376d567ee37b2a96913eca)
    fn is_exposed_int_int(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> bool {
        unsafe { ffi::wxWindow_IsExposed2(self.as_ptr(), x, y, w, h) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::IsExposed()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ac4653d89376b9e88afbec8c342d32530)
    fn is_exposed_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_IsExposed3(self.as_ptr(), rect)
        }
    }
    /// Returns true if the window is shown, false if it has been hidden.
    ///
    /// [See `wxWindow::IsShown()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ad3544f9c364b7952ac0676217e400061)
    fn is_shown(&self) -> bool {
        unsafe { ffi::wxWindow_IsShown(self.as_ptr()) }
    }
    /// Returns true if the window is physically visible on the screen, i.e. it is shown and all its parents up to the toplevel window are shown as well.
    ///
    /// [See `wxWindow::IsShownOnScreen()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ac2722709783e89e76b2eeb7f9f93236a)
    fn is_shown_on_screen(&self) -> bool {
        unsafe { ffi::wxWindow_IsShownOnScreen(self.as_ptr()) }
    }
    /// Disables the window.
    ///
    /// [See `wxWindow::Disable()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a26d7329a9a753fa0445501f01f66c41e)
    fn disable(&self) -> bool {
        unsafe { ffi::wxWindow_Disable(self.as_ptr()) }
    }
    /// Enable or disable the window for user input.
    ///
    /// [See `wxWindow::Enable()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a4e933aa891f42fbb3b87438057c573af)
    fn enable(&self, enable: bool) -> bool {
        unsafe { ffi::wxWindow_Enable(self.as_ptr(), enable) }
    }
    /// Shows or hides the window.
    ///
    /// [See `wxWindow::Show()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a7fbc92ce240a8d4f6956b6e0276ef07f)
    fn show(&self, show: bool) -> bool {
        unsafe { ffi::wxWindow_Show(self.as_ptr(), show) }
    }
    // NOT_SUPPORTED: fn ShowWithEffect()
    /// Gets the help text to be used as context-sensitive help for this window.
    ///
    /// [See `wxWindow::GetHelpText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#acb28971fe25abd1f5c6d768e203a042a)
    fn get_help_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxWindow_GetHelpText(self.as_ptr())).into() }
    }
    /// Sets the help text to be used as context-sensitive help for this window.
    ///
    /// [See `wxWindow::SetHelpText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a4c1a2cbc7363237b3a7c70af4e702c72)
    fn set_help_text(&self, help_text: &str) {
        unsafe {
            let help_text = WxString::from(help_text);
            let help_text = help_text.as_ptr();
            ffi::wxWindow_SetHelpText(self.as_ptr(), help_text)
        }
    }
    // NOT_SUPPORTED: fn GetHelpTextAtPoint()
    /// Get the associated tooltip or NULL if none.
    ///
    /// [See `wxWindow::GetToolTip()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a5192577a8c6d35a73a1cde9acfe03dd4)
    fn get_tool_tip(&self) -> Option<ToolTipIsOwned<false>> {
        unsafe { ToolTip::option_from(ffi::wxWindow_GetToolTip(self.as_ptr())) }
    }
    /// Get the text of the associated tooltip or empty string if none.
    ///
    /// [See `wxWindow::GetToolTipText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a7e7c9a92f9efbb47073704646db1344e)
    fn get_tool_tip_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxWindow_GetToolTipText(self.as_ptr())).into() }
    }
    /// Attach a tooltip to the window.
    ///
    /// [See `wxWindow::SetToolTip()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a81a9fb74e3c6c7f4416e882f5e589b8c)
    fn set_tool_tip_str(&self, tip_string: &str) {
        unsafe {
            let tip_string = WxString::from(tip_string);
            let tip_string = tip_string.as_ptr();
            ffi::wxWindow_SetToolTip(self.as_ptr(), tip_string)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::SetToolTip()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a494e0b7cfca9299caa40e847767e7357)
    fn set_tool_tip_tooltip<T: ToolTipMethods>(&self, tip: Option<&T>) {
        unsafe {
            let tip = match tip {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetToolTip1(self.as_ptr(), tip)
        }
    }
    /// Unset any existing tooltip.
    ///
    /// [See `wxWindow::UnsetToolTip()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ae75712451673658b0a533480bf5eeaa4)
    fn unset_tool_tip(&self) {
        unsafe { ffi::wxWindow_UnsetToolTip(self.as_ptr()) }
    }
    /// This function shows a popup menu at the given position in this window and returns the selected id.
    ///
    /// [See `wxWindow::GetPopupMenuSelectionFromUser()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a9b7de6ea85ca926b668ba0682a61a93e)
    fn get_popup_menu_selection_from_user_point<M: MenuMethods, P: PointMethods>(
        &self,
        menu: &M,
        pos: &P,
    ) -> c_int {
        unsafe {
            let menu = menu.as_ptr();
            let pos = pos.as_ptr();
            ffi::wxWindow_GetPopupMenuSelectionFromUser(self.as_ptr(), menu, pos)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::GetPopupMenuSelectionFromUser()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a00f5cb3cac003205317a565a33d4bb27)
    fn get_popup_menu_selection_from_user_int<M: MenuMethods>(
        &self,
        menu: &M,
        x: c_int,
        y: c_int,
    ) -> c_int {
        unsafe {
            let menu = menu.as_ptr();
            ffi::wxWindow_GetPopupMenuSelectionFromUser1(self.as_ptr(), menu, x, y)
        }
    }
    /// Pops up the given menu at the specified coordinates, relative to this window, and returns control when the user has dismissed the menu.
    ///
    /// [See `wxWindow::PopupMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a8f715d238cf74a845488b0e2645e98df)
    fn popup_menu_point<M: MenuMethods, P: PointMethods>(&self, menu: Option<&M>, pos: &P) -> bool {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            ffi::wxWindow_PopupMenu(self.as_ptr(), menu, pos)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// [See `wxWindow::PopupMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a834bcdfd41f5e5370ebd9ea401c92900)
    fn popup_menu_int<M: MenuMethods>(&self, menu: Option<&M>, x: c_int, y: c_int) -> bool {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_PopupMenu1(self.as_ptr(), menu, x, y)
        }
    }
    /// Validator functions.
    ///
    /// [See `wxWindow::GetValidator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aaa9b70aad1559212da2ce848f72ea894)
    fn get_validator(&self) -> WeakRef<Validator> {
        unsafe { WeakRef::<Validator>::from(ffi::wxWindow_GetValidator(self.as_ptr())) }
    }
    /// Deletes the current validator (if any) and sets the window validator, having called wxValidator::Clone to create a new validator of this type.
    ///
    /// [See `wxWindow::SetValidator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a00066c70049a7be3ce6b648d206e6432)
    fn set_validator<V: ValidatorMethods>(&self, validator: &V) {
        unsafe {
            let validator = validator.as_ptr();
            ffi::wxWindow_SetValidator(self.as_ptr(), validator)
        }
    }
    /// Transfers values from child controls to data areas specified by their validators.
    ///
    /// [See `wxWindow::TransferDataFromWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ab8e51f36e7d8790b361c8d8c6f37b1ad)
    fn transfer_data_from_window(&self) -> bool {
        unsafe { ffi::wxWindow_TransferDataFromWindow(self.as_ptr()) }
    }
    /// Transfers values to child controls from data areas specified by their validators.
    ///
    /// [See `wxWindow::TransferDataToWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a88cc65e424a129d9b0057756cdb67ec9)
    fn transfer_data_to_window(&self) -> bool {
        unsafe { ffi::wxWindow_TransferDataToWindow(self.as_ptr()) }
    }
    /// Validates the current values of the child controls using their validators.
    ///
    /// [See `wxWindow::Validate()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ac87f253253a0c5eb498871c83afa40fd)
    fn validate(&self) -> bool {
        unsafe { ffi::wxWindow_Validate(self.as_ptr()) }
    }
    /// Returns the identifier of the window.
    ///
    /// [See `wxWindow::GetId()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a79037f68b290bba5811628ec67bf3b24)
    fn get_id(&self) -> c_int {
        unsafe { ffi::wxWindow_GetId(self.as_ptr()) }
    }
    /// Generic way of getting a label from any window, for identification purposes.
    ///
    /// [See `wxWindow::GetLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a50db82dadf3cfaee41dd40a9b90ae339)
    fn get_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxWindow_GetLabel(self.as_ptr())).into() }
    }
    /// Returns the layout direction for this window, Note that wxLayout_Default is returned if layout direction is not supported.
    ///
    /// [See `wxWindow::GetLayoutDirection()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#afafd17cbb5dd6d899b25360255e0bdae)
    fn get_layout_direction(&self) -> c_int {
        unsafe { ffi::wxWindow_GetLayoutDirection(self.as_ptr()) }
    }
    /// Mirror coordinates for RTL layout if this window uses it and if the mirroring is not done automatically like Win32.
    ///
    /// [See `wxWindow::AdjustForLayoutDirection()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a71d1daa71f1ad1f6cbaac54db7e71af3)
    fn adjust_for_layout_direction(&self, x: c_int, width: c_int, width_total: c_int) -> c_int {
        unsafe { ffi::wxWindow_AdjustForLayoutDirection(self.as_ptr(), x, width, width_total) }
    }
    /// Returns the window's name.
    ///
    /// [See `wxWindow::GetName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a93b9ec7d9eaf152c17f3bf2698551ef4)
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxWindow_GetName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetWindowVariant()
    /// Sets the identifier of the window.
    ///
    /// [See `wxWindow::SetId()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a7f27d0faed14effa013381bdc40e1bcd)
    fn set_id(&self, winid: c_int) {
        unsafe { ffi::wxWindow_SetId(self.as_ptr(), winid) }
    }
    /// Sets the window's label.
    ///
    /// [See `wxWindow::SetLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aa00ffea9f53587f29ae343adde033b8e)
    fn set_label(&self, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxWindow_SetLabel(self.as_ptr(), label)
        }
    }
    /// Sets the layout direction for this window.
    ///
    /// [See `wxWindow::SetLayoutDirection()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a7d494549f7fcfed44af95f8ee364c1f9)
    fn set_layout_direction(&self, dir: c_int) {
        unsafe { ffi::wxWindow_SetLayoutDirection(self.as_ptr(), dir) }
    }
    /// Sets the window's name.
    ///
    /// [See `wxWindow::SetName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#af80875cda5e1af98dcd7c8e712e3c800)
    fn set_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxWindow_SetName(self.as_ptr(), name)
        }
    }
    // NOT_SUPPORTED: fn SetWindowVariant()
    /// Gets the accelerator table for this window.
    ///
    /// [See `wxWindow::GetAcceleratorTable()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a9cf3fd71ffab949cdf06f8f8e2646d56)
    fn get_accelerator_table(&self) -> Option<AcceleratorTableIsOwned<false>> {
        unsafe { AcceleratorTable::option_from(ffi::wxWindow_GetAcceleratorTable(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetAccessible()
    /// Sets the accelerator table for this window.
    ///
    /// [See `wxWindow::SetAcceleratorTable()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a0af5e9aa4dee6a4e92c0700f92605642)
    fn set_accelerator_table<A: AcceleratorTableMethods>(&self, accel: &A) {
        unsafe {
            let accel = accel.as_ptr();
            ffi::wxWindow_SetAcceleratorTable(self.as_ptr(), accel)
        }
    }
    // NOT_SUPPORTED: fn SetAccessible()
    /// This function simply generates a wxCloseEvent whose handler usually tries to close the window.
    ///
    /// [See `wxWindow::Close()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a3e44f4a494fc9ef4346c4fba70c8de0c)
    fn close(&self, force: bool) -> bool {
        unsafe { ffi::wxWindow_Close(self.as_ptr(), force) }
    }
    /// Destroys the window safely.
    ///
    /// [See `wxWindow::Destroy()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a6bf0c5be864544d9ce0560087667b7fc)
    fn destroy(&self) -> bool {
        unsafe { ffi::wxWindow_Destroy(self.as_ptr()) }
    }
    /// Returns true if this window is in process of being destroyed.
    ///
    /// [See `wxWindow::IsBeingDeleted()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a5fd3fba5d43efb67a834c0483a9c3d0e)
    fn is_being_deleted(&self) -> bool {
        unsafe { ffi::wxWindow_IsBeingDeleted(self.as_ptr()) }
    }
    /// Returns the associated drop target, which may be NULL.
    ///
    /// [See `wxWindow::GetDropTarget()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a9303a2de5ad692557b7db9fb3e651549)
    fn get_drop_target(&self) -> Option<DropTargetIsOwned<false>> {
        unsafe { DropTarget::option_from(ffi::wxWindow_GetDropTarget(self.as_ptr())) }
    }
    /// Associates a drop target with this window.
    ///
    /// [See `wxWindow::SetDropTarget()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ae34b4d45433ca8287df0e47d46411e58)
    fn set_drop_target<D: DropTargetMethods>(&self, target: Option<&D>) {
        unsafe {
            let target = match target {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetDropTarget(self.as_ptr(), target)
        }
    }
    /// Enables or disables eligibility for drop file events (OnDropFiles).
    ///
    /// [See `wxWindow::DragAcceptFiles()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a7e7015bc61bd79b6821d2dccaecf9eda)
    fn drag_accept_files(&self, accept: bool) {
        unsafe { ffi::wxWindow_DragAcceptFiles(self.as_ptr(), accept) }
    }
    /// Returns the sizer of which this window is a member, if any, otherwise NULL.
    ///
    /// [See `wxWindow::GetContainingSizer()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ad9e45e32ec75f3288f065d83263194a3)
    fn get_containing_sizer(&self) -> Option<SizerIsOwned<false>> {
        unsafe { Sizer::option_from(ffi::wxWindow_GetContainingSizer(self.as_ptr())) }
    }
    /// Returns the sizer associated with the window by a previous call to SetSizer(), or NULL.
    ///
    /// [See `wxWindow::GetSizer()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ae05f09350b273af1c47a82253538c5c4)
    fn get_sizer(&self) -> Option<SizerIsOwned<false>> {
        unsafe { Sizer::option_from(ffi::wxWindow_GetSizer(self.as_ptr())) }
    }
    /// Sets the window to have the given layout sizer.
    ///
    /// [See `wxWindow::SetSizer()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#abc95691b45e29a52c24aa9d33d46dec1)
    fn set_sizer<S: SizerMethods>(&self, sizer: Option<&S>, delete_old: bool) {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetSizer(self.as_ptr(), sizer, delete_old)
        }
    }
    /// Associate the sizer with the window and set the window size and minimal size accordingly.
    ///
    /// [See `wxWindow::SetSizerAndFit()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a29938af9828fd35da666536cdfb6b73c)
    fn set_sizer_and_fit<S: SizerMethods>(&self, sizer: Option<&S>, delete_old: bool) {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetSizerAndFit(self.as_ptr(), sizer, delete_old)
        }
    }
    /// Returns a pointer to the window's layout constraints, or NULL if there are none.
    ///
    /// [See `wxWindow::GetConstraints()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a768a91519aefb245da30c37782a716b7)
    fn get_constraints(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetConstraints(self.as_ptr()) }
    }
    /// Sets the window to have the given layout constraints.
    ///
    /// [See `wxWindow::SetConstraints()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#afa75d111bbd9a68f837101a5fbed60a7)
    fn set_constraints(&self, constraints: *mut c_void) {
        unsafe { ffi::wxWindow_SetConstraints(self.as_ptr(), constraints) }
    }
    /// Lays out the children of this window using the associated sizer.
    ///
    /// [See `wxWindow::Layout()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a1b143c3e72bd0af533b76db4830a6113)
    fn layout(&self) -> bool {
        unsafe { ffi::wxWindow_Layout(self.as_ptr()) }
    }
    /// Determines whether the Layout() function will be called automatically when the window is resized.
    ///
    /// [See `wxWindow::SetAutoLayout()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ad369fe1db5c20f9d9edff7b5eb1f7226)
    fn set_auto_layout(&self, auto_layout: bool) {
        unsafe { ffi::wxWindow_SetAutoLayout(self.as_ptr(), auto_layout) }
    }
    /// Returns true if Layout() is called automatically when the window is resized.
    ///
    /// [See `wxWindow::GetAutoLayout()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a64e3f936ab812d24966827ea4e0f15c0)
    fn get_auto_layout(&self) -> bool {
        unsafe { ffi::wxWindow_GetAutoLayout(self.as_ptr()) }
    }
    /// Directs all mouse input to this window.
    ///
    /// [See `wxWindow::CaptureMouse()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a5c72c6260a73ef77bb0b1f7ec85fcfef)
    fn capture_mouse(&self) {
        unsafe { ffi::wxWindow_CaptureMouse(self.as_ptr()) }
    }
    /// Returns the caret() associated with the window.
    ///
    /// [See `wxWindow::GetCaret()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a147ceedef6bd261d7a0ddf709f536233)
    fn get_caret(&self) -> Option<CaretIsOwned<false>> {
        unsafe { Caret::option_from(ffi::wxWindow_GetCaret(self.as_ptr())) }
    }
    // BLOCKED: fn GetCursor()
    /// Returns true if this window has the current mouse capture.
    ///
    /// [See `wxWindow::HasCapture()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a41e1b77ac82c7420d34b8030d5f26046)
    fn has_capture(&self) -> bool {
        unsafe { ffi::wxWindow_HasCapture(self.as_ptr()) }
    }
    /// Releases mouse input captured with CaptureMouse().
    ///
    /// [See `wxWindow::ReleaseMouse()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#adcc538819c11ecb3bd3e4e5d13c5ba7a)
    fn release_mouse(&self) {
        unsafe { ffi::wxWindow_ReleaseMouse(self.as_ptr()) }
    }
    /// Sets the caret() associated with the window.
    ///
    /// [See `wxWindow::SetCaret()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#acfef5e1cada92c73e2937b84ff57ff57)
    fn set_caret<C: CaretMethods>(&self, caret: Option<&C>) {
        unsafe {
            let caret = match caret {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetCaret(self.as_ptr(), caret)
        }
    }
    /// Sets the window's cursor.
    ///
    /// [See `wxWindow::SetCursor()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ad83f9c51c6f31e0e05f598b47a19ed98)
    fn set_cursor<C: CursorMethods>(&self, cursor: &C) -> bool {
        unsafe {
            let cursor = cursor.as_ptr();
            ffi::wxWindow_SetCursor(self.as_ptr(), cursor)
        }
    }
    /// Moves the pointer to the given position on the window.
    ///
    /// [See `wxWindow::WarpPointer()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ac500152d1eca3a2ee98a68e7ea7372b5)
    fn warp_pointer(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxWindow_WarpPointer(self.as_ptr(), x, y) }
    }
    /// Request generation of touch events for this window.
    ///
    /// [See `wxWindow::EnableTouchEvents()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ac1691c7a40e9245fe58430a1b8e3a998)
    fn enable_touch_events(&self, events_mask: c_int) -> bool {
        unsafe { ffi::wxWindow_EnableTouchEvents(self.as_ptr(), events_mask) }
    }
    // NOT_SUPPORTED: fn HitTest()
    // NOT_SUPPORTED: fn HitTest1()
    // NOT_SUPPORTED: fn GetBorder()
    // NOT_SUPPORTED: fn GetBorder1()
    /// Does the window-specific updating after processing the update event.
    ///
    /// [See `wxWindow::DoUpdateWindowUI()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ae8a84a80645e99f4d24a22e5c386f626)
    fn do_update_window_ui<U: UpdateUIEventMethods>(&self, event: &U) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxWindow_DoUpdateWindowUI(self.as_ptr(), event)
        }
    }
    // NOT_SUPPORTED: fn GetHandle()
    /// This method should be overridden to return true if this window has multiple pages.
    ///
    /// [See `wxWindow::HasMultiplePages()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a55c7b73596f2eec2694e8f512013ee81)
    fn has_multiple_pages(&self) -> bool {
        unsafe { ffi::wxWindow_HasMultiplePages(self.as_ptr()) }
    }
    /// This function is (or should be, in case of custom controls) called during window creation to intelligently set up the window visual attributes, that is the font and the foreground and background colours.
    ///
    /// [See `wxWindow::InheritAttributes()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a6b1bf9e099704e7a493b8c4666b1f7f7)
    fn inherit_attributes(&self) {
        unsafe { ffi::wxWindow_InheritAttributes(self.as_ptr()) }
    }
    /// Sends an wxEVT_INIT_DIALOG event, whose handler usually transfers data to the dialog via validators.
    ///
    /// [See `wxWindow::InitDialog()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#aa90a260c0a835a133043460b7d0024a8)
    fn init_dialog(&self) {
        unsafe { ffi::wxWindow_InitDialog(self.as_ptr()) }
    }
    /// Returns true if the window contents is double-buffered by the system, i.e. if any drawing done on the window is really done on a temporary backing surface and transferred to the screen all at once later.
    ///
    /// [See `wxWindow::IsDoubleBuffered()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#abe7a8c61796262b517f5e0765374cc2b)
    fn is_double_buffered(&self) -> bool {
        unsafe { ffi::wxWindow_IsDoubleBuffered(self.as_ptr()) }
    }
    /// Turn on or off double buffering of the window if the system supports it.
    ///
    /// [See `wxWindow::SetDoubleBuffered()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a5477a89c17fdcc3ec6c90274796eb1c3)
    fn set_double_buffered(&self, on: bool) {
        unsafe { ffi::wxWindow_SetDoubleBuffered(self.as_ptr(), on) }
    }
    /// Returns true if the window is retained, false otherwise.
    ///
    /// [See `wxWindow::IsRetained()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a77443cb35d15b5006b96a8c5ea0944f6)
    fn is_retained(&self) -> bool {
        unsafe { ffi::wxWindow_IsRetained(self.as_ptr()) }
    }
    /// Returns true if this window is intrinsically enabled, false otherwise, i.e. if Enable() Enable(false) had been called.
    ///
    /// [See `wxWindow::IsThisEnabled()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a1c6a83e5421bb7cfb8a9ad804d251b65)
    fn is_this_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_IsThisEnabled(self.as_ptr()) }
    }
    /// Returns true if the given window is a top-level one.
    ///
    /// [See `wxWindow::IsTopLevel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#afe04dba2155b58429d6c4e0a5a5e1664)
    fn is_top_level(&self) -> bool {
        unsafe { ffi::wxWindow_IsTopLevel(self.as_ptr()) }
    }
    /// This virtual function is normally only used internally, but sometimes an application may need it to implement functionality that should not be disabled by an application defining an OnIdle handler in a derived class.
    ///
    /// [See `wxWindow::OnInternalIdle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a6b5654c0c6c6245d592c279521f7099c)
    fn on_internal_idle(&self) {
        unsafe { ffi::wxWindow_OnInternalIdle(self.as_ptr()) }
    }
    /// Send idle event to window and all subwindows.
    ///
    /// [See `wxWindow::SendIdleEvents()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a1afc1653413957537073c074dcc3eade)
    fn send_idle_events(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_SendIdleEvents(self.as_ptr(), event) }
    }
    /// Registers a system wide hotkey.
    ///
    /// [See `wxWindow::RegisterHotKey()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a53ca57872dac5851ea6ba55a494b899b)
    fn register_hot_key(
        &self,
        hotkey_id: c_int,
        modifiers: c_int,
        virtual_key_code: c_int,
    ) -> bool {
        unsafe {
            ffi::wxWindow_RegisterHotKey(self.as_ptr(), hotkey_id, modifiers, virtual_key_code)
        }
    }
    /// Unregisters a system wide hotkey.
    ///
    /// [See `wxWindow::UnregisterHotKey()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a695f60d65f8f6c12e3c3645ad9c0c35b)
    fn unregister_hot_key(&self, hotkey_id: c_int) -> bool {
        unsafe { ffi::wxWindow_UnregisterHotKey(self.as_ptr(), hotkey_id) }
    }
    /// This function sends one or more wxUpdateUIEvent to the window.
    ///
    /// [See `wxWindow::UpdateWindowUI()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#adf0a4987728bd0bf69f922641b3efbfc)
    fn update_window_ui(&self, flags: c_long) {
        unsafe { ffi::wxWindow_UpdateWindowUI(self.as_ptr(), flags) }
    }
    // NOT_SUPPORTED: fn GetClassDefaultAttributes()
    /// Finds the window or control which currently has the keyboard focus.
    ///
    /// [See `wxWindow::FindFocus()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a84a2cb9c46c4829515f3c029f83495af)
    fn find_focus() -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_FindFocus()) }
    }
    /// Find the first window with the given id.
    ///
    /// [See `wxWindow::FindWindowById()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a87d7bf445768e9d90c30e2fe644062e6)
    fn find_window_by_id<W: WindowMethods>(id: c_long, parent: Option<&W>) -> WeakRef<Window> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<Window>::from(ffi::wxWindow_FindWindowById(id, parent))
        }
    }
    /// Find a window by its label.
    ///
    /// [See `wxWindow::FindWindowByLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a18f42acd37805eb7bad6b7a18c87e0f5)
    fn find_window_by_label<W: WindowMethods>(label: &str, parent: Option<&W>) -> WeakRef<Window> {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<Window>::from(ffi::wxWindow_FindWindowByLabel(label, parent))
        }
    }
    /// Find a window by its name (as given in a window constructor or Create() function call).
    ///
    /// [See `wxWindow::FindWindowByName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ad2abfb345618b1f3961721ecd6f41511)
    fn find_window_by_name<W: WindowMethods>(name: &str, parent: Option<&W>) -> WeakRef<Window> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<Window>::from(ffi::wxWindow_FindWindowByName(name, parent))
        }
    }
    /// Returns the currently captured window.
    ///
    /// [See `wxWindow::GetCapture()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a2a226e6e116858bdeb04f51f815eba03)
    fn get_capture() -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetCapture()) }
    }
    /// Create a new ID or range of IDs that are not currently in use.
    ///
    /// [See `wxWindow::NewControlId()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a8175da594e6045635a1d1cfe775cdddb)
    fn new_control_id(count: c_int) -> c_int {
        unsafe { ffi::wxWindow_NewControlId(count) }
    }
    /// Unreserve an ID or range of IDs that was reserved by NewControlId().
    ///
    /// [See `wxWindow::UnreserveControlId()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#ad9bc342d1e38d221e37f0f6396950c8c)
    fn unreserve_control_id(id: c_int, count: c_int) {
        unsafe { ffi::wxWindow_UnreserveControlId(id, count) }
    }
    // DTOR: fn ~wxWindow()
    /// Construct the actual window object after creating the C++ object.
    ///
    /// [See `wxWindow::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a95b7ca8faa033f5ab35458887c07bf38)
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
            ffi::wxWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxWindowCreateEvent
/// This trait represents C++ [`wxWindowCreateEvent`](https://docs.wxwidgets.org/3.2/classwx_window_create_event.html) class's methods and inheritance.
///
/// See [`WindowCreateEventIsOwned`] documentation for the class usage.
pub trait WindowCreateEventMethods: CommandEventMethods {
    /// Return the window being created.
    ///
    /// [See `wxWindowCreateEvent::GetWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window_create_event.html#a366d70275836a988a61bd6e3925007f9)
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindowCreateEvent_GetWindow(self.as_ptr())) }
    }
}

// wxWindowDC
/// This trait represents C++ [`wxWindowDC`](https://docs.wxwidgets.org/3.2/classwx_window_d_c.html) class's methods and inheritance.
///
/// See [`WindowDCIsOwned`] documentation for the class usage.
pub trait WindowDCMethods: DCMethods {}

// wxWindowDestroyEvent
/// This trait represents C++ [`wxWindowDestroyEvent`](https://docs.wxwidgets.org/3.2/classwx_window_destroy_event.html) class's methods and inheritance.
///
/// See [`WindowDestroyEventIsOwned`] documentation for the class usage.
pub trait WindowDestroyEventMethods: CommandEventMethods {
    /// Return the window being destroyed.
    ///
    /// [See `wxWindowDestroyEvent::GetWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window_destroy_event.html#a47d682152b33b3ebd559ae439ba37ce6)
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindowDestroyEvent_GetWindow(self.as_ptr())) }
    }
}

// wxWindowDisabler
/// This trait represents C++ [`wxWindowDisabler`](https://docs.wxwidgets.org/3.2/classwx_window_disabler.html) class's methods and inheritance.
///
/// See [`WindowDisablerIsOwned`] documentation for the class usage.
pub trait WindowDisablerMethods: WxRustMethods {
    // DTOR: fn ~wxWindowDisabler()
}

// wxWizard
/// This trait represents C++ [`wxWizard`](https://docs.wxwidgets.org/3.2/classwx_wizard.html) class's methods and inheritance.
///
/// See [`WizardIsOwned`] documentation for the class usage.
pub trait WizardMethods: DialogMethods {
    /// Creates the wizard dialog.
    ///
    /// [See `wxWizard::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#a3f6ec3ff744228c15aebfacfd127143f)
    fn create_int<W: WindowMethods, B: BitmapBundleMethods, P: PointMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        title: &str,
        bitmap: &B,
        pos: &P,
        style: c_long,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            let bitmap = bitmap.as_ptr();
            let pos = pos.as_ptr();
            ffi::wxWizard_Create(self.as_ptr(), parent, id, title, bitmap, pos, style)
        }
    }
    /// This method is obsolete, use GetPageAreaSizer() instead.
    ///
    /// [See `wxWizard::FitToPage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#a2348b05acbe3e9ec5a54e5a6a7c9a042)
    fn fit_to_page<W: WizardPageMethods>(&self, first_page: Option<&W>) {
        unsafe {
            let first_page = match first_page {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWizard_FitToPage(self.as_ptr(), first_page)
        }
    }
    /// Returns the bitmap used for the wizard.
    ///
    /// [See `wxWizard::GetBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#a7183729eb9e8992298b944da87d7bdc5)
    fn get_bitmap(&self) -> BitmapIsOwned<false> {
        unsafe { BitmapIsOwned::from_ptr(ffi::wxWizard_GetBitmap(self.as_ptr())) }
    }
    /// Returns the colour that should be used to fill the area not taken up by the wizard or page bitmap, if a non-zero bitmap placement flag has been set.
    ///
    /// [See `wxWizard::GetBitmapBackgroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#ac75615eb0b2808b3cb22e2ffdd661eef)
    fn get_bitmap_background_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxWizard_GetBitmapBackgroundColour(self.as_ptr())) }
    }
    /// Returns the flags indicating how the wizard or page bitmap should be expanded and positioned to fit the page height.
    ///
    /// [See `wxWizard::GetBitmapPlacement()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#aad5f212e215e0e1a6c298ccb6b7cc9a7)
    fn get_bitmap_placement(&self) -> c_int {
        unsafe { ffi::wxWizard_GetBitmapPlacement(self.as_ptr()) }
    }
    /// Get the current page while the wizard is running.
    ///
    /// [See `wxWizard::GetCurrentPage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#addbdd626e927fd9143fbe817f75385aa)
    fn get_current_page(&self) -> WeakRef<WizardPage> {
        unsafe { WeakRef::<WizardPage>::from(ffi::wxWizard_GetCurrentPage(self.as_ptr())) }
    }
    /// Returns the minimum width for the bitmap that will be constructed to contain the actual wizard or page bitmap if a non-zero bitmap placement flag has been set.
    ///
    /// [See `wxWizard::GetMinimumBitmapWidth()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#aeeaee33a159cb30b55edfcd0b04a59df)
    fn get_minimum_bitmap_width(&self) -> c_int {
        unsafe { ffi::wxWizard_GetMinimumBitmapWidth(self.as_ptr()) }
    }
    /// Returns pointer to page area sizer.
    ///
    /// [See `wxWizard::GetPageAreaSizer()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#a0af838517d05fc40293edfff6bfe93ce)
    fn get_page_area_sizer(&self) -> Option<SizerIsOwned<false>> {
        unsafe { Sizer::option_from(ffi::wxWizard_GetPageAreaSizer(self.as_ptr())) }
    }
    /// Returns the size available for the pages.
    ///
    /// [See `wxWizard::GetPageSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#ace8db44010ce8976c7d571e88ec0783e)
    fn get_page_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWizard_GetPageSize(self.as_ptr())) }
    }
    /// Return true if this page is not the last one in the wizard.
    ///
    /// [See `wxWizard::HasNextPage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#a8166f3467d70856c2cb08ba61bc3b46d)
    fn has_next_page<W: WizardPageMethods>(&self, page: Option<&W>) -> bool {
        unsafe {
            let page = match page {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWizard_HasNextPage(self.as_ptr(), page)
        }
    }
    /// Returns true if this page is not the first one in the wizard.
    ///
    /// [See `wxWizard::HasPrevPage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#a9fe21e8524c76db5b8d0db752bc10d81)
    fn has_prev_page<W: WizardPageMethods>(&self, page: Option<&W>) -> bool {
        unsafe {
            let page = match page {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWizard_HasPrevPage(self.as_ptr(), page)
        }
    }
    /// Executes the wizard starting from the given page, returning true if it was successfully finished or false if user cancelled it.
    ///
    /// [See `wxWizard::RunWizard()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#ae4ed7afb38e10d782a66a1daba8fa83d)
    fn run_wizard<W: WizardPageMethods>(&self, first_page: Option<&W>) -> bool {
        unsafe {
            let first_page = match first_page {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWizard_RunWizard(self.as_ptr(), first_page)
        }
    }
    /// Sets the bitmap used for the wizard.
    ///
    /// [See `wxWizard::SetBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#abe92496bbc84875fcb787d944fb163e0)
    fn set_bitmap<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxWizard_SetBitmap(self.as_ptr(), bitmap)
        }
    }
    /// Sets the colour that should be used to fill the area not taken up by the wizard or page bitmap, if a non-zero bitmap placement flag has been set.
    ///
    /// [See `wxWizard::SetBitmapBackgroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#a7ec9f8483c4c79a76899f61d3719d51d)
    fn set_bitmap_background_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWizard_SetBitmapBackgroundColour(self.as_ptr(), colour)
        }
    }
    /// Sets the flags indicating how the wizard or page bitmap should be expanded and positioned to fit the page height.
    ///
    /// [See `wxWizard::SetBitmapPlacement()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#a862344eb45f192916e6b6ca83b588eb6)
    fn set_bitmap_placement(&self, placement: c_int) {
        unsafe { ffi::wxWizard_SetBitmapPlacement(self.as_ptr(), placement) }
    }
    /// Sets width of border around page area.
    ///
    /// [See `wxWizard::SetBorder()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#aaa18c0f177ddae29bb096f3e8c2f5d72)
    fn set_border(&self, border: c_int) {
        unsafe { ffi::wxWizard_SetBorder(self.as_ptr(), border) }
    }
    /// Sets the minimum width for the bitmap that will be constructed to contain the actual wizard or page bitmap if a non-zero bitmap placement flag has been set.
    ///
    /// [See `wxWizard::SetMinimumBitmapWidth()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#aacea5d1dc302604fc757bd1eb89b50f3)
    fn set_minimum_bitmap_width(&self, width: c_int) {
        unsafe { ffi::wxWizard_SetMinimumBitmapWidth(self.as_ptr(), width) }
    }
    /// Sets the minimal size to be made available for the wizard pages.
    ///
    /// [See `wxWizard::SetPageSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#af54b79bba58c1bdd6e4d389953131344)
    fn set_page_size<S: SizeMethods>(&self, size_page: &S) {
        unsafe {
            let size_page = size_page.as_ptr();
            ffi::wxWizard_SetPageSize(self.as_ptr(), size_page)
        }
    }
}

// wxWizardEvent
/// This trait represents C++ [`wxWizardEvent`](https://docs.wxwidgets.org/3.2/classwx_wizard_event.html) class's methods and inheritance.
///
/// See [`WizardEventIsOwned`] documentation for the class usage.
pub trait WizardEventMethods: NotifyEventMethods {
    /// Return the direction in which the page is changing: for EVT_WIZARD_PAGE_CHANGING, return true if we're going forward or false otherwise and for EVT_WIZARD_PAGE_CHANGED return true if we came from the previous page and false if we returned from the next one.
    ///
    /// [See `wxWizardEvent::GetDirection()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard_event.html#af398fffca9cf822fbf3707cc69abc5e4)
    fn get_direction(&self) -> bool {
        unsafe { ffi::wxWizardEvent_GetDirection(self.as_ptr()) }
    }
    /// Returns the wxWizardPage which was active when this event was generated.
    ///
    /// [See `wxWizardEvent::GetPage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard_event.html#aeb23d10773734416097281d097c6673e)
    fn get_page(&self) -> WeakRef<WizardPage> {
        unsafe { WeakRef::<WizardPage>::from(ffi::wxWizardEvent_GetPage(self.as_ptr())) }
    }
}

// wxWizardPage
/// This trait represents C++ [`wxWizardPage`](https://docs.wxwidgets.org/3.2/classwx_wizard_page.html) class's methods and inheritance.
///
/// See [`WizardPageIsOwned`] documentation for the class usage.
pub trait WizardPageMethods: PanelMethods {
    /// Creates the wizard page.
    ///
    /// [See `wxWizardPage::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard_page.html#a8d8433183ab5eda79a883eec9ff28562)
    fn create_wizard<W: WizardMethods, B: BitmapBundleMethods>(
        &self,
        parent: Option<&W>,
        bitmap: &B,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let bitmap = bitmap.as_ptr();
            ffi::wxWizardPage_Create(self.as_ptr(), parent, bitmap)
        }
    }
    /// This method is called by wxWizard to get the bitmap to display alongside the page.
    ///
    /// [See `wxWizardPage::GetBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard_page.html#a4acbde71fe16e2d9f1ef1d41066fc434)
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxWizardPage_GetBitmap(self.as_ptr())) }
    }
    /// Get the page which should be shown when the user chooses the "Next" button: if NULL is returned, this button will be disabled.
    ///
    /// [See `wxWizardPage::GetNext()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard_page.html#a1046455b7c16d356f265c429a7da2270)
    fn get_next(&self) -> WeakRef<WizardPage> {
        unsafe { WeakRef::<WizardPage>::from(ffi::wxWizardPage_GetNext(self.as_ptr())) }
    }
    /// Get the page which should be shown when the user chooses the "Back" button: if NULL is returned, this button will be disabled.
    ///
    /// [See `wxWizardPage::GetPrev()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard_page.html#a044b9309f55069341c17cd4748a86e92)
    fn get_prev(&self) -> WeakRef<WizardPage> {
        unsafe { WeakRef::<WizardPage>::from(ffi::wxWizardPage_GetPrev(self.as_ptr())) }
    }
}

// wxWizardPageSimple
/// This trait represents C++ [`wxWizardPageSimple`](https://docs.wxwidgets.org/3.2/classwx_wizard_page_simple.html) class's methods and inheritance.
///
/// See [`WizardPageSimpleIsOwned`] documentation for the class usage.
pub trait WizardPageSimpleMethods: WizardPageMethods {
    /// Creates the wizard page.
    ///
    /// [See `wxWizardPageSimple::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard_page_simple.html#afec87beaffdc6a86e479604db9a3c56a)
    fn create_wizard_wizardpage<
        W: WizardMethods,
        W2: WizardPageMethods,
        W3: WizardPageMethods,
        B: BitmapBundleMethods,
    >(
        &self,
        parent: Option<&W>,
        prev: Option<&W2>,
        next: Option<&W3>,
        bitmap: &B,
    ) -> bool {
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
            ffi::wxWizardPageSimple_Create(self.as_ptr(), parent, prev, next, bitmap)
        }
    }
    /// A helper chaining this page with the next one.
    ///
    /// [See `wxWizardPageSimple::Chain()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard_page_simple.html#a2388f3625df916ee16a7ed90decbe1e0)
    fn chain<W: WizardPageSimpleMethods>(&self, next: Option<&W>) -> &Self {
        unsafe {
            let next = match next {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWizardPageSimple_Chain(self.as_ptr(), next);
            &self
        }
    }
    /// Sets the next page.
    ///
    /// [See `wxWizardPageSimple::SetNext()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard_page_simple.html#a2a6d73e88712a812ebda74860d37cd34)
    fn set_next<W: WizardPageMethods>(&self, next: Option<&W>) {
        unsafe {
            let next = match next {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWizardPageSimple_SetNext(self.as_ptr(), next)
        }
    }
    /// Sets the previous page.
    ///
    /// [See `wxWizardPageSimple::SetPrev()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard_page_simple.html#ab7ae2a2d58ad8a2ab12d9e4a0cd2fa52)
    fn set_prev<W: WizardPageMethods>(&self, prev: Option<&W>) {
        unsafe {
            let prev = match prev {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWizardPageSimple_SetPrev(self.as_ptr(), prev)
        }
    }
    /// A convenience function to make the pages follow each other.
    ///
    /// [See `wxWizardPageSimple::Chain()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard_page_simple.html#a85e8772012bd5c628d04465b0b84e036)
    fn chain_wizardpagesimple<W: WizardPageSimpleMethods, W2: WizardPageSimpleMethods>(
        first: Option<&W>,
        second: Option<&W2>,
    ) {
        unsafe {
            let first = match first {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let second = match second {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWizardPageSimple_Chain1(first, second)
        }
    }
}

// wxWrapSizer
/// This trait represents C++ [`wxWrapSizer`](https://docs.wxwidgets.org/3.2/classwx_wrap_sizer.html) class's methods and inheritance.
///
/// See [`WrapSizerIsOwned`] documentation for the class usage.
pub trait WrapSizerMethods: BoxSizerMethods {}
