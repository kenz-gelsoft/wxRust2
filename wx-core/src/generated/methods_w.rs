use super::*;

// wxWindow
pub trait WindowMethods: EvtHandlerMethods {
    /// This method may be overridden in the derived classes to return false to indicate that this control doesn't accept input at all (i.e. behaves like e.g. wxStaticText) and so doesn't need focus.
    fn accepts_focus(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocus(self.as_ptr()) }
    }
    /// This method may be overridden in the derived classes to return false to indicate that while this control can, in principle, have focus if the user clicks it with the mouse, it shouldn't be included in the TAB traversal chain when using the keyboard.
    fn accepts_focus_from_keyboard(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocusFromKeyboard(self.as_ptr()) }
    }
    /// Overridden to indicate whether this window or one of its children accepts focus.
    fn accepts_focus_recursively(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocusRecursively(self.as_ptr()) }
    }
    /// Disable giving focus to this window using the keyboard navigation keys.
    fn disable_focus_from_keyboard(&self) {
        unsafe { ffi::wxWindow_DisableFocusFromKeyboard(self.as_ptr()) }
    }
    /// Can this window itself have focus?
    fn is_focusable(&self) -> bool {
        unsafe { ffi::wxWindow_IsFocusable(self.as_ptr()) }
    }
    /// Can this window have focus right now?
    fn can_accept_focus(&self) -> bool {
        unsafe { ffi::wxWindow_CanAcceptFocus(self.as_ptr()) }
    }
    /// Can this window be assigned focus from keyboard right now?
    fn can_accept_focus_from_keyboard(&self) -> bool {
        unsafe { ffi::wxWindow_CanAcceptFocusFromKeyboard(self.as_ptr()) }
    }
    /// Returns true if the window (or in case of composite controls, its main child window) has focus.
    fn has_focus(&self) -> bool {
        unsafe { ffi::wxWindow_HasFocus(self.as_ptr()) }
    }
    /// This method is only implemented by ports which have support for native TAB traversal (such as GTK+ 2.0).
    fn set_can_focus(&self, can_focus: bool) {
        unsafe { ffi::wxWindow_SetCanFocus(self.as_ptr(), can_focus) }
    }
    /// Enables or disables visible indication of keyboard focus.
    fn enable_visible_focus(&self, enable: bool) {
        unsafe { ffi::wxWindow_EnableVisibleFocus(self.as_ptr(), enable) }
    }
    /// This sets the window to receive keyboard input.
    fn set_focus(&self) {
        unsafe { ffi::wxWindow_SetFocus(self.as_ptr()) }
    }
    /// This function is called by wxWidgets keyboard navigation code when the user gives the focus to this window from keyboard (e.g. using TAB key).
    fn set_focus_from_kbd(&self) {
        unsafe { ffi::wxWindow_SetFocusFromKbd(self.as_ptr()) }
    }
    /// Adds a child window.
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
    fn destroy_children(&self) -> bool {
        unsafe { ffi::wxWindow_DestroyChildren(self.as_ptr()) }
    }
    /// Find a child of this window, by id.
    fn find_window_long(&self, id: c_long) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_FindWindow(self.as_ptr(), id)) }
    }
    /// Find a child of this window, by name.
    fn find_window_str(&self, name: &str) -> WeakRef<Window> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            WeakRef::<Window>::from(ffi::wxWindow_FindWindow1(self.as_ptr(), name))
        }
    }
    // BLOCKED: fn GetChildren()
    /// Returns a const reference to the list of the window's children.
    fn get_children(&self) -> WindowListIsOwned<false> {
        unsafe { WindowListIsOwned::from_ptr(ffi::wxWindow_GetChildren1(self.as_ptr())) }
    }
    /// Removes a child window.
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
    fn get_grand_parent(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetGrandParent(self.as_ptr())) }
    }
    /// Returns the next window after this one among the parent's children or NULL if this window is the last child.
    fn get_next_sibling(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetNextSibling(self.as_ptr())) }
    }
    /// Returns the parent of the window, or NULL if there is no parent.
    fn get_parent(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetParent(self.as_ptr())) }
    }
    /// Returns the previous window before this one among the parent's children or  NULL if this window is the first child.
    fn get_prev_sibling(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetPrevSibling(self.as_ptr())) }
    }
    /// Check if the specified window is a descendant of this one.
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
    fn always_show_scrollbars(&self, hflag: bool, vflag: bool) {
        unsafe { ffi::wxWindow_AlwaysShowScrollbars(self.as_ptr(), hflag, vflag) }
    }
    /// Returns the built-in scrollbar position.
    fn get_scroll_pos(&self, orientation: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetScrollPos(self.as_ptr(), orientation) }
    }
    /// Returns the built-in scrollbar range.
    fn get_scroll_range(&self, orientation: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetScrollRange(self.as_ptr(), orientation) }
    }
    /// Returns the built-in scrollbar thumb size.
    fn get_scroll_thumb(&self, orientation: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetScrollThumb(self.as_ptr(), orientation) }
    }
    /// Returns true if this window can have a scroll bar in this orientation.
    fn can_scroll(&self, orient: c_int) -> bool {
        unsafe { ffi::wxWindow_CanScroll(self.as_ptr(), orient) }
    }
    /// Returns true if this window currently has a scroll bar for this orientation.
    fn has_scrollbar(&self, orient: c_int) -> bool {
        unsafe { ffi::wxWindow_HasScrollbar(self.as_ptr(), orient) }
    }
    /// Return whether a scrollbar is always shown.
    fn is_scrollbar_always_shown(&self, orient: c_int) -> bool {
        unsafe { ffi::wxWindow_IsScrollbarAlwaysShown(self.as_ptr(), orient) }
    }
    /// Scrolls the window by the given number of lines down (if lines is positive) or up.
    fn scroll_lines(&self, lines: c_int) -> bool {
        unsafe { ffi::wxWindow_ScrollLines(self.as_ptr(), lines) }
    }
    /// Scrolls the window by the given number of pages down (if pages is positive) or up.
    fn scroll_pages(&self, pages: c_int) -> bool {
        unsafe { ffi::wxWindow_ScrollPages(self.as_ptr(), pages) }
    }
    /// Physically scrolls the pixels in the window and move child windows accordingly.
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
    fn line_up(&self) -> bool {
        unsafe { ffi::wxWindow_LineUp(self.as_ptr()) }
    }
    /// Same as ScrollLines (1).
    fn line_down(&self) -> bool {
        unsafe { ffi::wxWindow_LineDown(self.as_ptr()) }
    }
    /// Same as ScrollPages (-1).
    fn page_up(&self) -> bool {
        unsafe { ffi::wxWindow_PageUp(self.as_ptr()) }
    }
    /// Same as ScrollPages (1).
    fn page_down(&self) -> bool {
        unsafe { ffi::wxWindow_PageDown(self.as_ptr()) }
    }
    /// Sets the position of one of the built-in scrollbars.
    fn set_scroll_pos(&self, orientation: c_int, pos: c_int, refresh: bool) {
        unsafe { ffi::wxWindow_SetScrollPos(self.as_ptr(), orientation, pos, refresh) }
    }
    /// Sets the scrollbar properties of a built-in scrollbar.
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
    fn begin_repositioning_children(&self) -> bool {
        unsafe { ffi::wxWindow_BeginRepositioningChildren(self.as_ptr()) }
    }
    /// Fix child window positions after setting all of them at once.
    fn end_repositioning_children(&self) {
        unsafe { ffi::wxWindow_EndRepositioningChildren(self.as_ptr()) }
    }
    /// Sets the cached best size value.
    fn cache_best_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_CacheBestSize(self.as_ptr(), size)
        }
    }
    /// Converts client area size size to corresponding window size.
    fn client_to_window_size<S: SizeMethods>(&self, size: &S) -> Size {
        unsafe {
            let size = size.as_ptr();
            Size::from_ptr(ffi::wxWindow_ClientToWindowSize(self.as_ptr(), size))
        }
    }
    /// Converts window size size to corresponding client area size In other words, the returned value is what would GetClientSize() return if this window had given window size.
    fn window_to_client_size<S: SizeMethods>(&self, size: &S) -> Size {
        unsafe {
            let size = size.as_ptr();
            Size::from_ptr(ffi::wxWindow_WindowToClientSize(self.as_ptr(), size))
        }
    }
    /// Sizes the window to fit its best size.
    fn fit(&self) {
        unsafe { ffi::wxWindow_Fit(self.as_ptr()) }
    }
    /// Similar to Fit(), but sizes the interior (virtual) size of a window.
    fn fit_inside(&self) {
        unsafe { ffi::wxWindow_FitInside(self.as_ptr()) }
    }
    /// Convert DPI-independent pixel values to the value in pixels appropriate for the current toolkit.
    fn from_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_FromDIP(self.as_ptr(), sz))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn from_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_FromDIP1(self.as_ptr(), pt))
        }
    }
    /// Convert DPI-independent distance in pixels to the value in pixels appropriate for the current toolkit.
    fn from_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxWindow_FromDIP2(self.as_ptr(), d) }
    }
    /// Convert pixel values of the current toolkit to DPI-independent pixel values.
    fn to_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_ToDIP(self.as_ptr(), sz))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn to_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ToDIP1(self.as_ptr(), pt))
        }
    }
    /// Convert pixel values of the current toolkit to DPI-independent pixel values.
    fn to_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxWindow_ToDIP2(self.as_ptr(), d) }
    }
    /// Convert from physical pixels to logical pixels.
    fn from_phys_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_FromPhys(self.as_ptr(), sz))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn from_phys_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_FromPhys1(self.as_ptr(), pt))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn from_phys_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxWindow_FromPhys2(self.as_ptr(), d) }
    }
    /// Convert from logical pixels to physical pixels.
    fn to_phys_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_ToPhys(self.as_ptr(), sz))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn to_phys_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ToPhys1(self.as_ptr(), pt))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn to_phys_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxWindow_ToPhys2(self.as_ptr(), d) }
    }
    /// This functions returns the best acceptable minimal size for the window.
    fn get_best_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetBestSize(self.as_ptr())) }
    }
    /// Returns the best height needed by this window if it had the given width.
    fn get_best_height(&self, width: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetBestHeight(self.as_ptr(), width) }
    }
    /// Returns the best width needed by this window if it had the given height.
    fn get_best_width(&self, height: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetBestWidth(self.as_ptr(), height) }
    }
    /// Returns the size of the window 'client area' in pixels.
    fn get_client_size_int(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetClientSize(self.as_ptr(), width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn get_client_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetClientSize1(self.as_ptr())) }
    }
    /// Merges the window's best size into the min size and returns the result.
    fn get_effective_min_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetEffectiveMinSize(self.as_ptr())) }
    }
    /// Returns the maximum size of window's client area.
    fn get_max_client_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetMaxClientSize(self.as_ptr())) }
    }
    /// Returns the maximum size of the window.
    fn get_max_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetMaxSize(self.as_ptr())) }
    }
    /// Returns the minimum size of window's client area, an indication to the sizer layout mechanism that this is the minimum required size of its client area.
    fn get_min_client_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetMinClientSize(self.as_ptr())) }
    }
    /// Returns the minimum size of the window, an indication to the sizer layout mechanism that this is the minimum required size.
    fn get_min_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetMinSize(self.as_ptr())) }
    }
    /// Returns the horizontal component of window minimal size.
    fn get_min_width(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMinWidth(self.as_ptr()) }
    }
    /// Returns the vertical component of window minimal size.
    fn get_min_height(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMinHeight(self.as_ptr()) }
    }
    /// Returns the horizontal component of window maximal size.
    fn get_max_width(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMaxWidth(self.as_ptr()) }
    }
    /// Returns the vertical component of window maximal size.
    fn get_max_height(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMaxHeight(self.as_ptr()) }
    }
    /// Returns the size of the entire window in pixels, including title bar, border, scrollbars, etc.
    fn get_size_int(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetSize(self.as_ptr(), width, height) }
    }
    /// See the GetSize(int*,int*) overload for more info.
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetSize1(self.as_ptr())) }
    }
    /// This gets the virtual size of the window in pixels.
    fn get_virtual_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetVirtualSize(self.as_ptr())) }
    }
    /// Like the other GetVirtualSize() overload but uses pointers instead.
    fn get_virtual_size_int(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetVirtualSize1(self.as_ptr(), width, height) }
    }
    /// Return the largest of ClientSize and BestSize (as determined by a sizer, interior children, or other means)
    fn get_best_virtual_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetBestVirtualSize(self.as_ptr())) }
    }
    /// Returns the factor mapping logical pixels of this window to physical pixels.
    fn get_content_scale_factor(&self) -> c_double {
        unsafe { ffi::wxWindow_GetContentScaleFactor(self.as_ptr()) }
    }
    /// Returns the ratio of the DPI used by this window to the standard DPI.
    fn get_dpi_scale_factor(&self) -> c_double {
        unsafe { ffi::wxWindow_GetDPIScaleFactor(self.as_ptr()) }
    }
    /// Returns the size of the left/right and top/bottom borders of this window in x and y components of the result respectively.
    fn get_window_border_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetWindowBorderSize(self.as_ptr())) }
    }
    /// wxSizer and friends use this to give a chance to a component to recalc its min size once one of the final size components is known.
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
    fn invalidate_best_size(&self) {
        unsafe { ffi::wxWindow_InvalidateBestSize(self.as_ptr()) }
    }
    /// Posts a size event to the window.
    fn post_size_event(&self) {
        unsafe { ffi::wxWindow_PostSizeEvent(self.as_ptr()) }
    }
    /// Posts a size event to the parent of this window.
    fn post_size_event_to_parent(&self) {
        unsafe { ffi::wxWindow_PostSizeEventToParent(self.as_ptr()) }
    }
    /// This function sends a dummy size event to the window allowing it to re-layout its children positions.
    fn send_size_event(&self, flags: c_int) {
        unsafe { ffi::wxWindow_SendSizeEvent(self.as_ptr(), flags) }
    }
    /// Safe wrapper for GetParent()->SendSizeEvent().
    fn send_size_event_to_parent(&self, flags: c_int) {
        unsafe { ffi::wxWindow_SendSizeEventToParent(self.as_ptr(), flags) }
    }
    /// This sets the size of the window client area in pixels.
    fn set_client_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxWindow_SetClientSize(self.as_ptr(), width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn set_client_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetClientSize1(self.as_ptr(), size)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn set_client_size_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_SetClientSize2(self.as_ptr(), rect)
        }
    }
    /// Used by wxSizer internally to notify the window about being managed by the given sizer.
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
    fn set_initial_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetInitialSize(self.as_ptr(), size)
        }
    }
    /// Sets the maximum client size of the window, to indicate to the sizer layout mechanism that this is the maximum possible size of its client area.
    fn set_max_client_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMaxClientSize(self.as_ptr(), size)
        }
    }
    /// Sets the maximum size of the window, to indicate to the sizer layout mechanism that this is the maximum possible size.
    fn set_max_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMaxSize(self.as_ptr(), size)
        }
    }
    /// Sets the minimum client size of the window, to indicate to the sizer layout mechanism that this is the minimum required size of window's client area.
    fn set_min_client_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMinClientSize(self.as_ptr(), size)
        }
    }
    /// Sets the minimum size of the window, to indicate to the sizer layout mechanism that this is the minimum required size.
    fn set_min_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMinSize(self.as_ptr(), size)
        }
    }
    /// Sets the size of the window in pixels.
    fn set_size_int_int(&self, x: c_int, y: c_int, width: c_int, height: c_int, size_flags: c_int) {
        unsafe { ffi::wxWindow_SetSize(self.as_ptr(), x, y, width, height, size_flags) }
    }
    /// Sets the size of the window in pixels.
    fn set_size_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_SetSize1(self.as_ptr(), rect)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn set_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetSize2(self.as_ptr(), size)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn set_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxWindow_SetSize3(self.as_ptr(), width, height) }
    }
    /// Use of this function for windows which are not toplevel windows (such as wxDialog or wxFrame) is discouraged.
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
    fn set_virtual_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxWindow_SetVirtualSize(self.as_ptr(), width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn set_virtual_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetVirtualSize1(self.as_ptr(), size)
        }
    }
    /// Non window-specific DPI-independent pixels conversion functions.
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
    fn center(&self, dir: c_int) {
        unsafe { ffi::wxWindow_Center(self.as_ptr(), dir) }
    }
    /// A synonym for CentreOnParent().
    fn center_on_parent(&self, dir: c_int) {
        unsafe { ffi::wxWindow_CenterOnParent(self.as_ptr(), dir) }
    }
    /// Centres the window.
    fn centre(&self, direction: c_int) {
        unsafe { ffi::wxWindow_Centre(self.as_ptr(), direction) }
    }
    /// Centres the window on its parent.
    fn centre_on_parent(&self, direction: c_int) {
        unsafe { ffi::wxWindow_CentreOnParent(self.as_ptr(), direction) }
    }
    /// This gets the position of the window in pixels, relative to the parent window for the child windows or relative to the display origin for the top level windows.
    fn get_position_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_GetPosition(self.as_ptr(), x, y) }
    }
    /// This gets the position of the window in pixels, relative to the parent window for the child windows or relative to the display origin for the top level windows.
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxWindow_GetPosition1(self.as_ptr())) }
    }
    /// Returns the position and size of the window as a wxRect object.
    fn get_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxWindow_GetRect(self.as_ptr())) }
    }
    /// Returns the window position in screen coordinates, whether the window is a child window or a top level one.
    fn get_screen_position_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_GetScreenPosition(self.as_ptr(), x, y) }
    }
    /// Returns the window position in screen coordinates, whether the window is a child window or a top level one.
    fn get_screen_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxWindow_GetScreenPosition1(self.as_ptr())) }
    }
    /// Returns the position and size of the window on the screen as a wxRect object.
    fn get_screen_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxWindow_GetScreenRect(self.as_ptr())) }
    }
    /// Get the origin of the client area of the window relative to the window top left corner (the client area may be shifted because of the borders, scrollbars, other decorations...)
    fn get_client_area_origin(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxWindow_GetClientAreaOrigin(self.as_ptr())) }
    }
    /// Get the client rectangle in window (i.e. client) coordinates.
    fn get_client_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxWindow_GetClientRect(self.as_ptr())) }
    }
    /// Moves the window to the given position.
    fn move_int(&self, x: c_int, y: c_int, flags: c_int) {
        unsafe { ffi::wxWindow_Move(self.as_ptr(), x, y, flags) }
    }
    /// Moves the window to the given position.
    fn move_point<P: PointMethods>(&self, pt: &P, flags: c_int) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxWindow_Move1(self.as_ptr(), pt, flags)
        }
    }
    /// Moves the window to the specified position.
    fn set_position<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxWindow_SetPosition(self.as_ptr(), pt)
        }
    }
    /// Converts to screen coordinates from coordinates relative to this window.
    fn client_to_screen_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_ClientToScreen(self.as_ptr(), x, y) }
    }
    /// Converts to screen coordinates from coordinates relative to this window.
    fn client_to_screen_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ClientToScreen1(self.as_ptr(), pt))
        }
    }
    /// Converts a point or size from dialog units to pixels.
    fn convert_dialog_to_pixels_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ConvertDialogToPixels(self.as_ptr(), pt))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn convert_dialog_to_pixels_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_ConvertDialogToPixels1(self.as_ptr(), sz))
        }
    }
    /// Converts a point or size from pixels to dialog units.
    fn convert_pixels_to_dialog_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ConvertPixelsToDialog(self.as_ptr(), pt))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn convert_pixels_to_dialog_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_ConvertPixelsToDialog1(self.as_ptr(), sz))
        }
    }
    /// Converts from screen to client window coordinates.
    fn screen_to_client_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_ScreenToClient(self.as_ptr(), x, y) }
    }
    /// Converts from screen to client window coordinates.
    fn screen_to_client_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ScreenToClient1(self.as_ptr(), pt))
        }
    }
    /// Clears the window by filling it with the current background colour.
    fn clear_background(&self) {
        unsafe { ffi::wxWindow_ClearBackground(self.as_ptr()) }
    }
    /// Freezes the window or, in other words, prevents any updates from taking place on screen, the window is not redrawn at all.
    fn freeze(&self) {
        unsafe { ffi::wxWindow_Freeze(self.as_ptr()) }
    }
    /// Re-enables window updating after a previous call to Freeze().
    fn thaw(&self) {
        unsafe { ffi::wxWindow_Thaw(self.as_ptr()) }
    }
    /// Returns true if the window is currently frozen by a call to Freeze().
    fn is_frozen(&self) -> bool {
        unsafe { ffi::wxWindow_IsFrozen(self.as_ptr()) }
    }
    /// Returns the background colour of the window.
    fn get_background_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxWindow_GetBackgroundColour(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetBackgroundStyle()
    /// Returns the character height for this window.
    fn get_char_height(&self) -> c_int {
        unsafe { ffi::wxWindow_GetCharHeight(self.as_ptr()) }
    }
    /// Returns the average character width for this window.
    fn get_char_width(&self) -> c_int {
        unsafe { ffi::wxWindow_GetCharWidth(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetDefaultAttributes()
    /// Return the DPI of the display used by this window.
    fn get_dpi(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetDPI(self.as_ptr())) }
    }
    /// Returns the font for this window.
    fn get_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxWindow_GetFont(self.as_ptr())) }
    }
    /// Returns the foreground colour of the window.
    fn get_foreground_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxWindow_GetForegroundColour(self.as_ptr())) }
    }
    /// Gets the dimensions of the string as it would be drawn on the window with the currently selected font.
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
    fn get_text_extent(&self, string: &str) -> Size {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            Size::from_ptr(ffi::wxWindow_GetTextExtent1(self.as_ptr(), string))
        }
    }
    // BLOCKED: fn GetUpdateRegion()
    /// Get the update rectangle bounding box in client coords.
    fn get_update_client_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxWindow_GetUpdateClientRect(self.as_ptr())) }
    }
    /// Returns true if this window background is transparent (as, for example, for wxStaticText) and should show the parent window background.
    fn has_transparent_background(&self) -> bool {
        unsafe { ffi::wxWindow_HasTransparentBackground(self.as_ptr()) }
    }
    /// Causes this window, and all of its children recursively, to be repainted.
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
    fn refresh_rect<R: RectMethods>(&self, rect: &R, erase_background: bool) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_RefreshRect(self.as_ptr(), rect, erase_background)
        }
    }
    /// Calling this method immediately repaints the invalidated area of the window and all of its children recursively (this normally only happens when the flow of control returns to the event loop).
    fn update(&self) {
        unsafe { ffi::wxWindow_Update(self.as_ptr()) }
    }
    /// Sets the background colour of the window.
    fn set_background_colour<C: ColourMethods>(&self, colour: &C) -> bool {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWindow_SetBackgroundColour(self.as_ptr(), colour)
        }
    }
    // NOT_SUPPORTED: fn SetBackgroundStyle()
    /// Checks whether using transparent background might work.
    fn is_transparent_background_supported(&self, reason: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_IsTransparentBackgroundSupported(self.as_ptr(), reason) }
    }
    /// Sets the font for this window.
    fn set_font<F: FontMethods>(&self, font: &F) -> bool {
        unsafe {
            let font = font.as_ptr();
            ffi::wxWindow_SetFont(self.as_ptr(), font)
        }
    }
    /// Sets the foreground colour of the window.
    fn set_foreground_colour<C: ColourMethods>(&self, colour: &C) -> bool {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWindow_SetForegroundColour(self.as_ptr(), colour)
        }
    }
    /// Sets the background colour of the window but prevents it from being inherited by the children of this window.
    fn set_own_background_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWindow_SetOwnBackgroundColour(self.as_ptr(), colour)
        }
    }
    /// Return true if this window inherits the background colour from its parent.
    fn inherits_background_colour(&self) -> bool {
        unsafe { ffi::wxWindow_InheritsBackgroundColour(self.as_ptr()) }
    }
    /// Return true if a background colour has been set for this window.
    fn use_bg_col(&self) -> bool {
        unsafe { ffi::wxWindow_UseBgCol(self.as_ptr()) }
    }
    /// Return true if a background colour has been set for this window.
    fn use_background_colour(&self) -> bool {
        unsafe { ffi::wxWindow_UseBackgroundColour(self.as_ptr()) }
    }
    /// Sets the font of the window but prevents it from being inherited by the children of this window.
    fn set_own_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxWindow_SetOwnFont(self.as_ptr(), font)
        }
    }
    /// Sets the foreground colour of the window but prevents it from being inherited by the children of this window.
    fn set_own_foreground_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWindow_SetOwnForegroundColour(self.as_ptr(), colour)
        }
    }
    /// Return true if a foreground colour has been set for this window.
    fn use_foreground_colour(&self) -> bool {
        unsafe { ffi::wxWindow_UseForegroundColour(self.as_ptr()) }
    }
    /// Return true if this window inherits the foreground colour from its parent.
    fn inherits_foreground_colour(&self) -> bool {
        unsafe { ffi::wxWindow_InheritsForegroundColour(self.as_ptr()) }
    }
    fn set_palette<P: PaletteMethods>(&self, pal: &P) {
        unsafe {
            let pal = pal.as_ptr();
            ffi::wxWindow_SetPalette(self.as_ptr(), pal)
        }
    }
    /// Return true from here to allow the colours of this window to be changed by InheritAttributes().
    fn should_inherit_colours(&self) -> bool {
        unsafe { ffi::wxWindow_ShouldInheritColours(self.as_ptr()) }
    }
    /// This function tells a window if it should use the system's "theme" code to draw the windows' background instead of its own background drawing code.
    fn set_theme_enabled(&self, enable: bool) {
        unsafe { ffi::wxWindow_SetThemeEnabled(self.as_ptr(), enable) }
    }
    /// Returns true if the window uses the system theme for drawing its background.
    fn get_theme_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_GetThemeEnabled(self.as_ptr()) }
    }
    /// Returns true if the system supports transparent windows and calling SetTransparent() may succeed.
    fn can_set_transparent(&self) -> bool {
        unsafe { ffi::wxWindow_CanSetTransparent(self.as_ptr()) }
    }
    /// Set the transparency of the window.
    fn set_transparent(&self, alpha: c_uchar) -> bool {
        unsafe { ffi::wxWindow_SetTransparent(self.as_ptr(), alpha) }
    }
    /// Returns the event handler for this window.
    fn get_event_handler(&self) -> WeakRef<EvtHandler> {
        unsafe { WeakRef::<EvtHandler>::from(ffi::wxWindow_GetEventHandler(self.as_ptr())) }
    }
    /// This function will generate the appropriate call to Navigate() if the key event is one normally used for keyboard navigation and return true in this case.
    fn handle_as_navigation_key<K: KeyEventMethods>(&self, event: &K) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxWindow_HandleAsNavigationKey(self.as_ptr(), event)
        }
    }
    /// Shorthand for:
    fn handle_window_event<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxWindow_HandleWindowEvent(self.as_ptr(), event)
        }
    }
    /// Convenient wrapper for ProcessEvent().
    fn process_window_event<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxWindow_ProcessWindowEvent(self.as_ptr(), event)
        }
    }
    /// Wrapper for wxEvtHandler::ProcessEventLocally().
    fn process_window_event_locally<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxWindow_ProcessWindowEventLocally(self.as_ptr(), event)
        }
    }
    /// Removes and returns the top-most event handler on the event handler stack.
    fn pop_event_handler(&self, delete_handler: bool) -> WeakRef<EvtHandler> {
        unsafe {
            WeakRef::<EvtHandler>::from(ffi::wxWindow_PopEventHandler(
                self.as_ptr(),
                delete_handler,
            ))
        }
    }
    /// Pushes this event handler onto the event stack for the window.
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
    fn get_extra_style(&self) -> c_long {
        unsafe { ffi::wxWindow_GetExtraStyle(self.as_ptr()) }
    }
    /// Gets the window style that was passed to the constructor or Create() method.
    fn get_window_style_flag(&self) -> c_long {
        unsafe { ffi::wxWindow_GetWindowStyleFlag(self.as_ptr()) }
    }
    /// See GetWindowStyleFlag() for more info.
    fn get_window_style(&self) -> c_long {
        unsafe { ffi::wxWindow_GetWindowStyle(self.as_ptr()) }
    }
    /// Returns true if the window has the given exFlag bit set in its extra styles.
    fn has_extra_style(&self, ex_flag: c_int) -> bool {
        unsafe { ffi::wxWindow_HasExtraStyle(self.as_ptr(), ex_flag) }
    }
    /// Returns true if the window has the given flag bit set.
    fn has_flag(&self, flag: c_int) -> bool {
        unsafe { ffi::wxWindow_HasFlag(self.as_ptr(), flag) }
    }
    /// Sets the extra style bits for the window.
    fn set_extra_style(&self, ex_style: c_long) {
        unsafe { ffi::wxWindow_SetExtraStyle(self.as_ptr(), ex_style) }
    }
    /// Sets the style of the window.
    fn set_window_style_flag(&self, style: c_long) {
        unsafe { ffi::wxWindow_SetWindowStyleFlag(self.as_ptr(), style) }
    }
    /// See SetWindowStyleFlag() for more info.
    fn set_window_style(&self, style: c_long) {
        unsafe { ffi::wxWindow_SetWindowStyle(self.as_ptr(), style) }
    }
    /// Turns the given flag on if it's currently turned off and vice versa.
    fn toggle_window_style(&self, flag: c_int) -> bool {
        unsafe { ffi::wxWindow_ToggleWindowStyle(self.as_ptr(), flag) }
    }
    /// Moves this window in the tab navigation order after the specified win.
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
    fn navigate(&self, flags: c_int) -> bool {
        unsafe { ffi::wxWindow_Navigate(self.as_ptr(), flags) }
    }
    /// Performs a keyboard navigation action inside this window.
    fn navigate_in(&self, flags: c_int) -> bool {
        unsafe { ffi::wxWindow_NavigateIn(self.as_ptr(), flags) }
    }
    /// Lowers the window to the bottom of the window hierarchy (Z-order).
    fn lower(&self) {
        unsafe { ffi::wxWindow_Lower(self.as_ptr()) }
    }
    /// Raises the window to the top of the window hierarchy (Z-order).
    fn raise(&self) {
        unsafe { ffi::wxWindow_Raise(self.as_ptr()) }
    }
    /// Equivalent to calling wxWindow::Show(false).
    fn hide(&self) -> bool {
        unsafe { ffi::wxWindow_Hide(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn HideWithEffect()
    /// Returns true if the window is enabled, i.e. if it accepts user input, false otherwise.
    fn is_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_IsEnabled(self.as_ptr()) }
    }
    /// Returns true if the given point or rectangle area has been exposed since the last repaint.
    fn is_exposed_int(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxWindow_IsExposed(self.as_ptr(), x, y) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn is_exposed_point<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxWindow_IsExposed1(self.as_ptr(), pt)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn is_exposed_int_int(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> bool {
        unsafe { ffi::wxWindow_IsExposed2(self.as_ptr(), x, y, w, h) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn is_exposed_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_IsExposed3(self.as_ptr(), rect)
        }
    }
    /// Returns true if the window is shown, false if it has been hidden.
    fn is_shown(&self) -> bool {
        unsafe { ffi::wxWindow_IsShown(self.as_ptr()) }
    }
    /// Returns true if the window is physically visible on the screen, i.e. it is shown and all its parents up to the toplevel window are shown as well.
    fn is_shown_on_screen(&self) -> bool {
        unsafe { ffi::wxWindow_IsShownOnScreen(self.as_ptr()) }
    }
    /// Disables the window.
    fn disable(&self) -> bool {
        unsafe { ffi::wxWindow_Disable(self.as_ptr()) }
    }
    /// Enable or disable the window for user input.
    fn enable(&self, enable: bool) -> bool {
        unsafe { ffi::wxWindow_Enable(self.as_ptr(), enable) }
    }
    /// Shows or hides the window.
    fn show(&self, show: bool) -> bool {
        unsafe { ffi::wxWindow_Show(self.as_ptr(), show) }
    }
    // NOT_SUPPORTED: fn ShowWithEffect()
    /// Gets the help text to be used as context-sensitive help for this window.
    fn get_help_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxWindow_GetHelpText(self.as_ptr())).into() }
    }
    /// Sets the help text to be used as context-sensitive help for this window.
    fn set_help_text(&self, help_text: &str) {
        unsafe {
            let help_text = WxString::from(help_text);
            let help_text = help_text.as_ptr();
            ffi::wxWindow_SetHelpText(self.as_ptr(), help_text)
        }
    }
    // NOT_SUPPORTED: fn GetHelpTextAtPoint()
    /// Get the associated tooltip or NULL if none.
    fn get_tool_tip(&self) -> Option<ToolTipIsOwned<false>> {
        unsafe { ToolTip::option_from(ffi::wxWindow_GetToolTip(self.as_ptr())) }
    }
    /// Get the text of the associated tooltip or empty string if none.
    fn get_tool_tip_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxWindow_GetToolTipText(self.as_ptr())).into() }
    }
    /// Attach a tooltip to the window.
    fn set_tool_tip_str(&self, tip_string: &str) {
        unsafe {
            let tip_string = WxString::from(tip_string);
            let tip_string = tip_string.as_ptr();
            ffi::wxWindow_SetToolTip(self.as_ptr(), tip_string)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
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
    fn unset_tool_tip(&self) {
        unsafe { ffi::wxWindow_UnsetToolTip(self.as_ptr()) }
    }
    /// This function shows a popup menu at the given position in this window and returns the selected id.
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
    fn get_validator(&self) -> WeakRef<Validator> {
        unsafe { WeakRef::<Validator>::from(ffi::wxWindow_GetValidator(self.as_ptr())) }
    }
    /// Deletes the current validator (if any) and sets the window validator, having called wxValidator::Clone to create a new validator of this type.
    fn set_validator<V: ValidatorMethods>(&self, validator: &V) {
        unsafe {
            let validator = validator.as_ptr();
            ffi::wxWindow_SetValidator(self.as_ptr(), validator)
        }
    }
    /// Transfers values from child controls to data areas specified by their validators.
    fn transfer_data_from_window(&self) -> bool {
        unsafe { ffi::wxWindow_TransferDataFromWindow(self.as_ptr()) }
    }
    /// Transfers values to child controls from data areas specified by their validators.
    fn transfer_data_to_window(&self) -> bool {
        unsafe { ffi::wxWindow_TransferDataToWindow(self.as_ptr()) }
    }
    /// Validates the current values of the child controls using their validators.
    fn validate(&self) -> bool {
        unsafe { ffi::wxWindow_Validate(self.as_ptr()) }
    }
    /// Returns the identifier of the window.
    fn get_id(&self) -> c_int {
        unsafe { ffi::wxWindow_GetId(self.as_ptr()) }
    }
    /// Generic way of getting a label from any window, for identification purposes.
    fn get_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxWindow_GetLabel(self.as_ptr())).into() }
    }
    /// Returns the layout direction for this window, Note that wxLayout_Default is returned if layout direction is not supported.
    fn get_layout_direction(&self) -> c_int {
        unsafe { ffi::wxWindow_GetLayoutDirection(self.as_ptr()) }
    }
    /// Mirror coordinates for RTL layout if this window uses it and if the mirroring is not done automatically like Win32.
    fn adjust_for_layout_direction(&self, x: c_int, width: c_int, width_total: c_int) -> c_int {
        unsafe { ffi::wxWindow_AdjustForLayoutDirection(self.as_ptr(), x, width, width_total) }
    }
    /// Returns the window's name.
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxWindow_GetName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetWindowVariant()
    /// Sets the identifier of the window.
    fn set_id(&self, winid: c_int) {
        unsafe { ffi::wxWindow_SetId(self.as_ptr(), winid) }
    }
    /// Sets the window's label.
    fn set_label(&self, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxWindow_SetLabel(self.as_ptr(), label)
        }
    }
    /// Sets the layout direction for this window.
    fn set_layout_direction(&self, dir: c_int) {
        unsafe { ffi::wxWindow_SetLayoutDirection(self.as_ptr(), dir) }
    }
    /// Sets the window's name.
    fn set_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxWindow_SetName(self.as_ptr(), name)
        }
    }
    // NOT_SUPPORTED: fn SetWindowVariant()
    /// Gets the accelerator table for this window.
    fn get_accelerator_table(&self) -> Option<AcceleratorTableIsOwned<false>> {
        unsafe { AcceleratorTable::option_from(ffi::wxWindow_GetAcceleratorTable(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetAccessible()
    /// Sets the accelerator table for this window.
    fn set_accelerator_table<A: AcceleratorTableMethods>(&self, accel: &A) {
        unsafe {
            let accel = accel.as_ptr();
            ffi::wxWindow_SetAcceleratorTable(self.as_ptr(), accel)
        }
    }
    // NOT_SUPPORTED: fn SetAccessible()
    /// This function simply generates a wxCloseEvent whose handler usually tries to close the window.
    fn close(&self, force: bool) -> bool {
        unsafe { ffi::wxWindow_Close(self.as_ptr(), force) }
    }
    /// Destroys the window safely.
    fn destroy(&self) -> bool {
        unsafe { ffi::wxWindow_Destroy(self.as_ptr()) }
    }
    /// Returns true if this window is in process of being destroyed.
    fn is_being_deleted(&self) -> bool {
        unsafe { ffi::wxWindow_IsBeingDeleted(self.as_ptr()) }
    }
    /// Returns the associated drop target, which may be NULL.
    fn get_drop_target(&self) -> Option<DropTargetIsOwned<false>> {
        unsafe { DropTarget::option_from(ffi::wxWindow_GetDropTarget(self.as_ptr())) }
    }
    /// Associates a drop target with this window.
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
    fn drag_accept_files(&self, accept: bool) {
        unsafe { ffi::wxWindow_DragAcceptFiles(self.as_ptr(), accept) }
    }
    /// Returns the sizer of which this window is a member, if any, otherwise NULL.
    fn get_containing_sizer(&self) -> Option<SizerIsOwned<false>> {
        unsafe { Sizer::option_from(ffi::wxWindow_GetContainingSizer(self.as_ptr())) }
    }
    /// Returns the sizer associated with the window by a previous call to SetSizer(), or NULL.
    fn get_sizer(&self) -> Option<SizerIsOwned<false>> {
        unsafe { Sizer::option_from(ffi::wxWindow_GetSizer(self.as_ptr())) }
    }
    /// Sets the window to have the given layout sizer.
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
    fn get_constraints(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetConstraints(self.as_ptr()) }
    }
    /// Sets the window to have the given layout constraints.
    fn set_constraints(&self, constraints: *mut c_void) {
        unsafe { ffi::wxWindow_SetConstraints(self.as_ptr(), constraints) }
    }
    /// Lays out the children of this window using the associated sizer.
    fn layout(&self) -> bool {
        unsafe { ffi::wxWindow_Layout(self.as_ptr()) }
    }
    /// Determines whether the Layout() function will be called automatically when the window is resized.
    fn set_auto_layout(&self, auto_layout: bool) {
        unsafe { ffi::wxWindow_SetAutoLayout(self.as_ptr(), auto_layout) }
    }
    /// Returns true if Layout() is called automatically when the window is resized.
    fn get_auto_layout(&self) -> bool {
        unsafe { ffi::wxWindow_GetAutoLayout(self.as_ptr()) }
    }
    /// Directs all mouse input to this window.
    fn capture_mouse(&self) {
        unsafe { ffi::wxWindow_CaptureMouse(self.as_ptr()) }
    }
    /// Returns the caret() associated with the window.
    fn get_caret(&self) -> Option<CaretIsOwned<false>> {
        unsafe { Caret::option_from(ffi::wxWindow_GetCaret(self.as_ptr())) }
    }
    // BLOCKED: fn GetCursor()
    /// Returns true if this window has the current mouse capture.
    fn has_capture(&self) -> bool {
        unsafe { ffi::wxWindow_HasCapture(self.as_ptr()) }
    }
    /// Releases mouse input captured with CaptureMouse().
    fn release_mouse(&self) {
        unsafe { ffi::wxWindow_ReleaseMouse(self.as_ptr()) }
    }
    /// Sets the caret() associated with the window.
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
    fn set_cursor<C: CursorMethods>(&self, cursor: &C) -> bool {
        unsafe {
            let cursor = cursor.as_ptr();
            ffi::wxWindow_SetCursor(self.as_ptr(), cursor)
        }
    }
    /// Moves the pointer to the given position on the window.
    fn warp_pointer(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxWindow_WarpPointer(self.as_ptr(), x, y) }
    }
    /// Request generation of touch events for this window.
    fn enable_touch_events(&self, events_mask: c_int) -> bool {
        unsafe { ffi::wxWindow_EnableTouchEvents(self.as_ptr(), events_mask) }
    }
    // NOT_SUPPORTED: fn HitTest()
    // NOT_SUPPORTED: fn HitTest1()
    // NOT_SUPPORTED: fn GetBorder()
    // NOT_SUPPORTED: fn GetBorder1()
    /// Does the window-specific updating after processing the update event.
    fn do_update_window_ui<U: UpdateUIEventMethods>(&self, event: &U) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxWindow_DoUpdateWindowUI(self.as_ptr(), event)
        }
    }
    // NOT_SUPPORTED: fn GetHandle()
    /// This method should be overridden to return true if this window has multiple pages.
    fn has_multiple_pages(&self) -> bool {
        unsafe { ffi::wxWindow_HasMultiplePages(self.as_ptr()) }
    }
    /// This function is (or should be, in case of custom controls) called during window creation to intelligently set up the window visual attributes, that is the font and the foreground and background colours.
    fn inherit_attributes(&self) {
        unsafe { ffi::wxWindow_InheritAttributes(self.as_ptr()) }
    }
    /// Sends an wxEVT_INIT_DIALOG event, whose handler usually transfers data to the dialog via validators.
    fn init_dialog(&self) {
        unsafe { ffi::wxWindow_InitDialog(self.as_ptr()) }
    }
    /// Returns true if the window contents is double-buffered by the system, i.e. if any drawing done on the window is really done on a temporary backing surface and transferred to the screen all at once later.
    fn is_double_buffered(&self) -> bool {
        unsafe { ffi::wxWindow_IsDoubleBuffered(self.as_ptr()) }
    }
    /// Turn on or off double buffering of the window if the system supports it.
    fn set_double_buffered(&self, on: bool) {
        unsafe { ffi::wxWindow_SetDoubleBuffered(self.as_ptr(), on) }
    }
    /// Returns true if the window is retained, false otherwise.
    fn is_retained(&self) -> bool {
        unsafe { ffi::wxWindow_IsRetained(self.as_ptr()) }
    }
    /// Returns true if this window is intrinsically enabled, false otherwise, i.e. if Enable() Enable(false) had been called.
    fn is_this_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_IsThisEnabled(self.as_ptr()) }
    }
    /// Returns true if the given window is a top-level one.
    fn is_top_level(&self) -> bool {
        unsafe { ffi::wxWindow_IsTopLevel(self.as_ptr()) }
    }
    /// This virtual function is normally only used internally, but sometimes an application may need it to implement functionality that should not be disabled by an application defining an OnIdle handler in a derived class.
    fn on_internal_idle(&self) {
        unsafe { ffi::wxWindow_OnInternalIdle(self.as_ptr()) }
    }
    /// Send idle event to window and all subwindows.
    fn send_idle_events(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_SendIdleEvents(self.as_ptr(), event) }
    }
    /// Registers a system wide hotkey.
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
    fn unregister_hot_key(&self, hotkey_id: c_int) -> bool {
        unsafe { ffi::wxWindow_UnregisterHotKey(self.as_ptr(), hotkey_id) }
    }
    /// This function sends one or more wxUpdateUIEvent to the window.
    fn update_window_ui(&self, flags: c_long) {
        unsafe { ffi::wxWindow_UpdateWindowUI(self.as_ptr(), flags) }
    }
    // NOT_SUPPORTED: fn GetClassDefaultAttributes()
    /// Finds the window or control which currently has the keyboard focus.
    fn find_focus() -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_FindFocus()) }
    }
    /// Find the first window with the given id.
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
    fn get_capture() -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetCapture()) }
    }
    /// Create a new ID or range of IDs that are not currently in use.
    fn new_control_id(count: c_int) -> c_int {
        unsafe { ffi::wxWindow_NewControlId(count) }
    }
    /// Unreserve an ID or range of IDs that was reserved by NewControlId().
    fn unreserve_control_id(id: c_int, count: c_int) {
        unsafe { ffi::wxWindow_UnreserveControlId(id, count) }
    }
    // DTOR: fn ~wxWindow()
    /// Construct the actual window object after creating the C++ object.
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
pub trait WindowCreateEventMethods: CommandEventMethods {
    /// Return the window being created.
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindowCreateEvent_GetWindow(self.as_ptr())) }
    }
}

// wxWindowDC
pub trait WindowDCMethods: DCMethods {}

// wxWindowDestroyEvent
pub trait WindowDestroyEventMethods: CommandEventMethods {
    /// Return the window being destroyed.
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindowDestroyEvent_GetWindow(self.as_ptr())) }
    }
}

// wxWindowDisabler
pub trait WindowDisablerMethods: WxRustMethods {
    // DTOR: fn ~wxWindowDisabler()
}

// wxWizard
pub trait WizardMethods: DialogMethods {
    /// Creates the wizard dialog.
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
    fn get_bitmap(&self) -> BitmapIsOwned<false> {
        unsafe { BitmapIsOwned::from_ptr(ffi::wxWizard_GetBitmap(self.as_ptr())) }
    }
    /// Returns the colour that should be used to fill the area not taken up by the wizard or page bitmap, if a non-zero bitmap placement flag has been set.
    fn get_bitmap_background_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxWizard_GetBitmapBackgroundColour(self.as_ptr())) }
    }
    /// Returns the flags indicating how the wizard or page bitmap should be expanded and positioned to fit the page height.
    fn get_bitmap_placement(&self) -> c_int {
        unsafe { ffi::wxWizard_GetBitmapPlacement(self.as_ptr()) }
    }
    /// Get the current page while the wizard is running.
    fn get_current_page(&self) -> WeakRef<WizardPage> {
        unsafe { WeakRef::<WizardPage>::from(ffi::wxWizard_GetCurrentPage(self.as_ptr())) }
    }
    /// Returns the minimum width for the bitmap that will be constructed to contain the actual wizard or page bitmap if a non-zero bitmap placement flag has been set.
    fn get_minimum_bitmap_width(&self) -> c_int {
        unsafe { ffi::wxWizard_GetMinimumBitmapWidth(self.as_ptr()) }
    }
    /// Returns pointer to page area sizer.
    fn get_page_area_sizer(&self) -> Option<SizerIsOwned<false>> {
        unsafe { Sizer::option_from(ffi::wxWizard_GetPageAreaSizer(self.as_ptr())) }
    }
    /// Returns the size available for the pages.
    fn get_page_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWizard_GetPageSize(self.as_ptr())) }
    }
    /// Return true if this page is not the last one in the wizard.
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
    fn set_bitmap<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxWizard_SetBitmap(self.as_ptr(), bitmap)
        }
    }
    /// Sets the colour that should be used to fill the area not taken up by the wizard or page bitmap, if a non-zero bitmap placement flag has been set.
    fn set_bitmap_background_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWizard_SetBitmapBackgroundColour(self.as_ptr(), colour)
        }
    }
    /// Sets the flags indicating how the wizard or page bitmap should be expanded and positioned to fit the page height.
    fn set_bitmap_placement(&self, placement: c_int) {
        unsafe { ffi::wxWizard_SetBitmapPlacement(self.as_ptr(), placement) }
    }
    /// Sets width of border around page area.
    fn set_border(&self, border: c_int) {
        unsafe { ffi::wxWizard_SetBorder(self.as_ptr(), border) }
    }
    /// Sets the minimum width for the bitmap that will be constructed to contain the actual wizard or page bitmap if a non-zero bitmap placement flag has been set.
    fn set_minimum_bitmap_width(&self, width: c_int) {
        unsafe { ffi::wxWizard_SetMinimumBitmapWidth(self.as_ptr(), width) }
    }
    /// Sets the minimal size to be made available for the wizard pages.
    fn set_page_size<S: SizeMethods>(&self, size_page: &S) {
        unsafe {
            let size_page = size_page.as_ptr();
            ffi::wxWizard_SetPageSize(self.as_ptr(), size_page)
        }
    }
}

// wxWizardEvent
pub trait WizardEventMethods: NotifyEventMethods {
    /// Return the direction in which the page is changing: for EVT_WIZARD_PAGE_CHANGING, return true if we're going forward or false otherwise and for EVT_WIZARD_PAGE_CHANGED return true if we came from the previous page and false if we returned from the next one.
    fn get_direction(&self) -> bool {
        unsafe { ffi::wxWizardEvent_GetDirection(self.as_ptr()) }
    }
    /// Returns the wxWizardPage which was active when this event was generated.
    fn get_page(&self) -> WeakRef<WizardPage> {
        unsafe { WeakRef::<WizardPage>::from(ffi::wxWizardEvent_GetPage(self.as_ptr())) }
    }
}

// wxWizardPage
pub trait WizardPageMethods: PanelMethods {
    /// Creates the wizard page.
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
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxWizardPage_GetBitmap(self.as_ptr())) }
    }
    /// Get the page which should be shown when the user chooses the "Next" button: if NULL is returned, this button will be disabled.
    fn get_next(&self) -> WeakRef<WizardPage> {
        unsafe { WeakRef::<WizardPage>::from(ffi::wxWizardPage_GetNext(self.as_ptr())) }
    }
    /// Get the page which should be shown when the user chooses the "Back" button: if NULL is returned, this button will be disabled.
    fn get_prev(&self) -> WeakRef<WizardPage> {
        unsafe { WeakRef::<WizardPage>::from(ffi::wxWizardPage_GetPrev(self.as_ptr())) }
    }
}

// wxWizardPageSimple
pub trait WizardPageSimpleMethods: WizardPageMethods {
    /// Creates the wizard page.
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
pub trait WrapSizerMethods: BoxSizerMethods {}
