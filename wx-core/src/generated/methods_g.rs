use super::*;

// wxGBPosition
/// This trait represents [C++ `wxGBPosition` class](https://docs.wxwidgets.org/3.2/classwx_g_b_position.html)'s methods and inheritance.
///
/// See [`GBPositionIsOwned`] documentation for the class usage.
pub trait GBPositionMethods: WxRustMethods {
    /// Get the current column value.
    ///
    /// See [C++ `wxGBPosition::GetCol()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_position.html#a74e010979c602a4ac6181c7ad6e75df9).
    fn get_col(&self) -> c_int {
        unsafe { ffi::wxGBPosition_GetCol(self.as_ptr()) }
    }
    /// Get the current row value.
    ///
    /// See [C++ `wxGBPosition::GetRow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_position.html#ae7804ad8687e6e0cb435d0a3868c9b90).
    fn get_row(&self) -> c_int {
        unsafe { ffi::wxGBPosition_GetRow(self.as_ptr()) }
    }
    /// Set a new column value.
    ///
    /// See [C++ `wxGBPosition::SetCol()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_position.html#a584e7ce01a9ea442bb61be26f2c3e756).
    fn set_col(&self, col: c_int) {
        unsafe { ffi::wxGBPosition_SetCol(self.as_ptr(), col) }
    }
    /// Set a new row value.
    ///
    /// See [C++ `wxGBPosition::SetRow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_position.html#a8ee420815d638827851492b588d990dc).
    fn set_row(&self, row: c_int) {
        unsafe { ffi::wxGBPosition_SetRow(self.as_ptr(), row) }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator==()
}

// wxGBSizerItem
/// This trait represents [C++ `wxGBSizerItem` class](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html)'s methods and inheritance.
///
/// See [`GBSizerItemIsOwned`] documentation for the class usage.
pub trait GBSizerItemMethods: SizerItemMethods {
    /// Get the row and column of the endpoint of this item.
    ///
    /// See [C++ `wxGBSizerItem::GetEndPos()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html#a6dfe180960c6d1eb5674de93bacefb66).
    fn get_end_pos(&self, row: *mut c_void, col: *mut c_void) {
        unsafe { ffi::wxGBSizerItem_GetEndPos(self.as_ptr(), row, col) }
    }
    /// Get the grid position of the item.
    ///
    /// See [C++ `wxGBSizerItem::GetPos()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html#af5be00a78fb32915745922bf5ec51d79).
    fn get_pos(&self) -> GBPosition {
        unsafe { GBPosition::from_ptr(ffi::wxGBSizerItem_GetPos(self.as_ptr())) }
    }
    ///
    /// See [C++ `wxGBSizerItem::GetPos()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html#a7666b9e9d898c276243a644e9d2b908c).
    fn get_pos_int(&self, row: *mut c_void, col: *mut c_void) {
        unsafe { ffi::wxGBSizerItem_GetPos1(self.as_ptr(), row, col) }
    }
    /// Get the row and column spanning of the item.
    ///
    /// See [C++ `wxGBSizerItem::GetSpan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html#ae649df760cf1575af568a80320941879).
    fn get_span(&self) -> GBSpan {
        unsafe { GBSpan::from_ptr(ffi::wxGBSizerItem_GetSpan(self.as_ptr())) }
    }
    ///
    /// See [C++ `wxGBSizerItem::GetSpan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html#a8cc525dab810c15ac3c62d969bfb2fe5).
    fn get_span_int(&self, rowspan: *mut c_void, colspan: *mut c_void) {
        unsafe { ffi::wxGBSizerItem_GetSpan1(self.as_ptr(), rowspan, colspan) }
    }
    /// Returns true if this item and the other item intersect.
    ///
    /// See [C++ `wxGBSizerItem::Intersects()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html#a53bb94f29fef93966cd960c644408cac).
    fn intersects_gbsizeritem<G: GBSizerItemMethods>(&self, other: &G) -> bool {
        unsafe {
            let other = other.as_ptr();
            ffi::wxGBSizerItem_Intersects(self.as_ptr(), other)
        }
    }
    /// Returns true if the given pos/span would intersect with this item.
    ///
    /// See [C++ `wxGBSizerItem::Intersects()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html#ada5e2355bc74010eed74e24fa21a9836).
    fn intersects_gbposition<G: GBPositionMethods, G2: GBSpanMethods>(
        &self,
        pos: &G,
        span: &G2,
    ) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            ffi::wxGBSizerItem_Intersects1(self.as_ptr(), pos, span)
        }
    }
    /// If the item is already a member of a sizer then first ensure that there is no other item that would intersect with this one at the new position, then set the new position.
    ///
    /// See [C++ `wxGBSizerItem::SetPos()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html#a16f4cfb3084129107896cdcb6ab6e0df).
    fn set_pos<G: GBPositionMethods>(&self, pos: &G) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxGBSizerItem_SetPos(self.as_ptr(), pos)
        }
    }
    /// If the item is already a member of a sizer then first ensure that there is no other item that would intersect with this one with its new spanning size, then set the new spanning.
    ///
    /// See [C++ `wxGBSizerItem::SetSpan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html#a62fe2aad8ca333ba7d99b688970ab23d).
    fn set_span<G: GBSpanMethods>(&self, span: &G) -> bool {
        unsafe {
            let span = span.as_ptr();
            ffi::wxGBSizerItem_SetSpan(self.as_ptr(), span)
        }
    }
    ///
    /// See [C++ `wxGBSizerItem::GetGBSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html#a50efa3c582e175880242aa00c87ec1d4).
    fn get_gb_sizer(&self) -> Option<GridBagSizerIsOwned<false>> {
        unsafe { GridBagSizer::option_from(ffi::wxGBSizerItem_GetGBSizer(self.as_ptr())) }
    }
    ///
    /// See [C++ `wxGBSizerItem::SetGBSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html#a38666e7ccd3eac99b170be25575c497f).
    fn set_gb_sizer<G: GridBagSizerMethods>(&self, sizer: Option<&G>) {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGBSizerItem_SetGBSizer(self.as_ptr(), sizer)
        }
    }
}

// wxGBSpan
/// This trait represents [C++ `wxGBSpan` class](https://docs.wxwidgets.org/3.2/classwx_g_b_span.html)'s methods and inheritance.
///
/// See [`GBSpanIsOwned`] documentation for the class usage.
pub trait GBSpanMethods: WxRustMethods {
    /// Get the current colspan value.
    ///
    /// See [C++ `wxGBSpan::GetColspan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_span.html#af98e9163ab73c7ea375b2e3a4c9e5d99).
    fn get_colspan(&self) -> c_int {
        unsafe { ffi::wxGBSpan_GetColspan(self.as_ptr()) }
    }
    /// Get the current rowspan value.
    ///
    /// See [C++ `wxGBSpan::GetRowspan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_span.html#a6ea613b4794bd9ce684e4d22a00d8974).
    fn get_rowspan(&self) -> c_int {
        unsafe { ffi::wxGBSpan_GetRowspan(self.as_ptr()) }
    }
    /// Set a new colspan value.
    ///
    /// See [C++ `wxGBSpan::SetColspan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_span.html#a58a5d23ca81c8053d0ddcec0e9ad79e9).
    fn set_colspan(&self, colspan: c_int) {
        unsafe { ffi::wxGBSpan_SetColspan(self.as_ptr(), colspan) }
    }
    /// Set a new rowspan value.
    ///
    /// See [C++ `wxGBSpan::SetRowspan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_span.html#a3d5755e3cb64405ebbbfc984b38aa287).
    fn set_rowspan(&self, rowspan: c_int) {
        unsafe { ffi::wxGBSpan_SetRowspan(self.as_ptr(), rowspan) }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator==()
}

// wxGCDC
/// This trait represents [C++ `wxGCDC` class](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html)'s methods and inheritance.
///
/// See [`GCDCIsOwned`] documentation for the class usage.
pub trait GCDCMethods: DCMethods {
    // DTOR: fn ~wxGCDC()
}

// wxGDIObject
/// This trait represents [C++ `wxGDIObject` class](https://docs.wxwidgets.org/3.2/classwx_g_d_i_object.html)'s methods and inheritance.
///
/// See [`GDIObjectIsOwned`] documentation for the class usage.
pub trait GDIObjectMethods: ObjectMethods {}

// wxGIFHandler
/// This trait represents [C++ `wxGIFHandler` class](https://docs.wxwidgets.org/3.2/classwx_g_i_f_handler.html)'s methods and inheritance.
///
/// See [`GIFHandlerIsOwned`] documentation for the class usage.
pub trait GIFHandlerMethods: ImageHandlerMethods {
    /// Save the animated gif.
    ///
    /// See [C++ `wxGIFHandler::SaveAnimation()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_i_f_handler.html#a9d3048c4ea2f719182781594c4c88b2b).
    fn save_animation(
        &self,
        images: *const c_void,
        stream: *mut c_void,
        verbose: bool,
        delay_milli_secs: c_int,
    ) -> bool {
        unsafe {
            ffi::wxGIFHandler_SaveAnimation(
                self.as_ptr(),
                images,
                stream,
                verbose,
                delay_milli_secs,
            )
        }
    }
}

// wxGauge
/// This trait represents [C++ `wxGauge` class](https://docs.wxwidgets.org/3.2/classwx_gauge.html)'s methods and inheritance.
///
/// See [`GaugeIsOwned`] documentation for the class usage.
pub trait GaugeMethods: ControlMethods {
    // DTOR: fn ~wxGauge()
    /// Creates the gauge for two-step construction.
    ///
    /// See [C++ `wxGauge::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_gauge.html#a5379a52a184c13c88a78a6e23d10982f).
    fn create_int<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        range: c_int,
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
            ffi::wxGauge_Create(
                self.as_ptr(),
                parent,
                id,
                range,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    /// Returns the maximum position of the gauge.
    ///
    /// See [C++ `wxGauge::GetRange()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_gauge.html#a0ad9c20ea6fff6c5083ecd7f77ab6044).
    fn get_range(&self) -> c_int {
        unsafe { ffi::wxGauge_GetRange(self.as_ptr()) }
    }
    /// Returns the current position of the gauge.
    ///
    /// See [C++ `wxGauge::GetValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_gauge.html#a777cd6782a01a9d395c12f70e958184b).
    fn get_value(&self) -> c_int {
        unsafe { ffi::wxGauge_GetValue(self.as_ptr()) }
    }
    /// Returns true if the gauge is vertical (has wxGA_VERTICAL style) and false otherwise.
    ///
    /// See [C++ `wxGauge::IsVertical()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_gauge.html#adf4a77e45e5680eeb2923adf2d0dc2a2).
    fn is_vertical(&self) -> bool {
        unsafe { ffi::wxGauge_IsVertical(self.as_ptr()) }
    }
    /// Switch the gauge to indeterminate mode (if required) and makes the gauge move a bit to indicate the user that some progress has been made.
    ///
    /// See [C++ `wxGauge::Pulse()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_gauge.html#a45c379e7a36861aa4cf4a381f8dc1e31).
    fn pulse(&self) {
        unsafe { ffi::wxGauge_Pulse(self.as_ptr()) }
    }
    /// Sets the range (maximum value) of the gauge.
    ///
    /// See [C++ `wxGauge::SetRange()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_gauge.html#ad371fb5ea9d29d482baea51f97976123).
    fn set_range(&self, range: c_int) {
        unsafe { ffi::wxGauge_SetRange(self.as_ptr(), range) }
    }
    /// Sets the position of the gauge.
    ///
    /// See [C++ `wxGauge::SetValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_gauge.html#a10d32bc2c4c4bb6c45041a0728f601dd).
    fn set_value(&self, pos: c_int) {
        unsafe { ffi::wxGauge_SetValue(self.as_ptr(), pos) }
    }
}

// wxGenericAboutDialog
/// This trait represents [C++ `wxGenericAboutDialog` class](https://docs.wxwidgets.org/3.2/classwx_generic_about_dialog.html)'s methods and inheritance.
///
/// See [`GenericAboutDialogIsOwned`] documentation for the class usage.
pub trait GenericAboutDialogMethods: WxRustMethods {
    /// Initializes the dialog created using the default constructor.
    ///
    /// See [C++ `wxGenericAboutDialog::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_about_dialog.html#a6a3ce237b9c32c4c3a9ba4f6011cabc0).
    fn create<A: AboutDialogInfoMethods, W: WindowMethods>(
        &self,
        info: &A,
        parent: Option<&W>,
    ) -> bool {
        unsafe {
            let info = info.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGenericAboutDialog_Create(self.as_ptr(), info, parent)
        }
    }
}

// wxGenericDirCtrl
/// This trait represents [C++ `wxGenericDirCtrl` class](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html)'s methods and inheritance.
///
/// See [`GenericDirCtrlIsOwned`] documentation for the class usage.
pub trait GenericDirCtrlMethods: ControlMethods {
    // DTOR: fn ~wxGenericDirCtrl()
    /// Collapse the given path.
    ///
    /// See [C++ `wxGenericDirCtrl::CollapsePath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#a114c6e42f0a75738ced711bc6395ac93).
    fn collapse_path(&self, path: &str) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_CollapsePath(self.as_ptr(), path)
        }
    }
    /// Collapses the entire tree.
    ///
    /// See [C++ `wxGenericDirCtrl::CollapseTree()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#aee6c6d4da828a3c159fc626d2fe85fb0).
    fn collapse_tree(&self) {
        unsafe { ffi::wxGenericDirCtrl_CollapseTree(self.as_ptr()) }
    }
    /// Create function for two-step construction.
    ///
    /// See [C++ `wxGenericDirCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#aefe8de00ec2421d2c3cc29c5825ac999).
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        dir: &str,
        pos: &P,
        size: &S,
        style: c_long,
        filter: &str,
        default_filter: c_int,
        name: &str,
    ) -> bool {
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
            ffi::wxGenericDirCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                dir,
                pos,
                size,
                style,
                filter,
                default_filter,
                name,
            )
        }
    }
    /// Tries to expand as much of the given path as possible, so that the filename or directory is visible in the tree control.
    ///
    /// See [C++ `wxGenericDirCtrl::ExpandPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#afbc4020de9d42bf1413320323b04e53e).
    fn expand_path(&self, path: &str) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_ExpandPath(self.as_ptr(), path)
        }
    }
    /// Gets the default path.
    ///
    /// See [C++ `wxGenericDirCtrl::GetDefaultPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#a5f2cb6a136eaec423f14c07f4849e7ce).
    fn get_default_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetDefaultPath(self.as_ptr())).into() }
    }
    /// Gets selected filename path only (else empty string).
    ///
    /// See [C++ `wxGenericDirCtrl::GetFilePath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#a1cd591a4c4778d018ee8914193773bf0).
    fn get_file_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetFilePath(self.as_ptr())).into() }
    }
    /// Fills the array paths with the currently selected filepaths.
    ///
    /// See [C++ `wxGenericDirCtrl::GetFilePaths()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#af576f168ca3039508af0aaf56bb94e76).
    fn get_file_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxGenericDirCtrl_GetFilePaths(self.as_ptr(), paths)
        }
    }
    /// Returns the filter string.
    ///
    /// See [C++ `wxGenericDirCtrl::GetFilter()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#af967c37eba095bb755e15174e464fb89).
    fn get_filter(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetFilter(self.as_ptr())).into() }
    }
    /// Returns the current filter index (zero-based).
    ///
    /// See [C++ `wxGenericDirCtrl::GetFilterIndex()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#a55a9c8e58d17926d7daa55ad74a65608).
    fn get_filter_index(&self) -> c_int {
        unsafe { ffi::wxGenericDirCtrl_GetFilterIndex(self.as_ptr()) }
    }
    /// Returns a pointer to the filter list control (if present).
    ///
    /// See [C++ `wxGenericDirCtrl::GetFilterListCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#aded34fc80676a272c0d559b89edb50bf).
    fn get_filter_list_ctrl(&self) -> *mut c_void {
        unsafe { ffi::wxGenericDirCtrl_GetFilterListCtrl(self.as_ptr()) }
    }
    /// Gets the currently-selected directory or filename.
    ///
    /// See [C++ `wxGenericDirCtrl::GetPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#aa5d0114bdcc93bd8955c8c074dea4904).
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetPath(self.as_ptr())).into() }
    }
    // BLOCKED: fn GetPath1()
    /// Fills the array paths with the selected directories and filenames.
    ///
    /// See [C++ `wxGenericDirCtrl::GetPaths()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#a4b183afd5be5421f064bef767361e6ec).
    fn get_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxGenericDirCtrl_GetPaths(self.as_ptr(), paths)
        }
    }
    /// Returns the root id for the tree control.
    ///
    /// See [C++ `wxGenericDirCtrl::GetRootId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#a52a3a39e47910a8cab4ed207e4b6a99f).
    fn get_root_id(&self) -> TreeItemId {
        unsafe { TreeItemId::from_ptr(ffi::wxGenericDirCtrl_GetRootId(self.as_ptr())) }
    }
    /// Returns a pointer to the tree control.
    ///
    /// See [C++ `wxGenericDirCtrl::GetTreeCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#aab7b5648efed1f6181127380d94b2e8a).
    fn get_tree_ctrl(&self) -> WeakRef<TreeCtrl> {
        unsafe { WeakRef::<TreeCtrl>::from(ffi::wxGenericDirCtrl_GetTreeCtrl(self.as_ptr())) }
    }
    /// Initializes variables.
    ///
    /// See [C++ `wxGenericDirCtrl::Init()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#aba23511d2deada35503cfa92b8b98283).
    fn init(&self) {
        unsafe { ffi::wxGenericDirCtrl_Init(self.as_ptr()) }
    }
    /// Collapse and expand the tree, thus re-creating it from scratch.
    ///
    /// See [C++ `wxGenericDirCtrl::ReCreateTree()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#a78a608e5aae6000be3f1b6225f67e613).
    fn re_create_tree(&self) {
        unsafe { ffi::wxGenericDirCtrl_ReCreateTree(self.as_ptr()) }
    }
    /// Sets the default path.
    ///
    /// See [C++ `wxGenericDirCtrl::SetDefaultPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#afaefbd20c8132ed2acbf0cee90e47723).
    fn set_default_path(&self, path: &str) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_SetDefaultPath(self.as_ptr(), path)
        }
    }
    /// Sets the filter string.
    ///
    /// See [C++ `wxGenericDirCtrl::SetFilter()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#a7cd100f7754fc1c533f7f53d6449d666).
    fn set_filter(&self, filter: &str) {
        unsafe {
            let filter = WxString::from(filter);
            let filter = filter.as_ptr();
            ffi::wxGenericDirCtrl_SetFilter(self.as_ptr(), filter)
        }
    }
    /// Sets the current filter index (zero-based).
    ///
    /// See [C++ `wxGenericDirCtrl::SetFilterIndex()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#abb2e1e12579519b7a0d8f85cb6d0b097).
    fn set_filter_index(&self, n: c_int) {
        unsafe { ffi::wxGenericDirCtrl_SetFilterIndex(self.as_ptr(), n) }
    }
    /// Sets the current path.
    ///
    /// See [C++ `wxGenericDirCtrl::SetPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#ac0e8bc480574ecf6fa0393bc38f20ecb).
    fn set_path(&self, path: &str) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_SetPath(self.as_ptr(), path)
        }
    }
    ///
    /// See [C++ `wxGenericDirCtrl::ShowHidden()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#a0a489055c42760e78e3a50e533a66484).
    fn show_hidden(&self, show: bool) {
        unsafe { ffi::wxGenericDirCtrl_ShowHidden(self.as_ptr(), show) }
    }
    /// Selects the given item.
    ///
    /// See [C++ `wxGenericDirCtrl::SelectPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#a5f175c4f1fef663c4ff6c322a6a175f5).
    fn select_path(&self, path: &str, select: bool) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_SelectPath(self.as_ptr(), path, select)
        }
    }
    /// Selects only the specified paths, clearing any previous selection.
    ///
    /// See [C++ `wxGenericDirCtrl::SelectPaths()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#a0f898f8a134b4351c9352c0278bcd866).
    fn select_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxGenericDirCtrl_SelectPaths(self.as_ptr(), paths)
        }
    }
    /// Removes the selection from all currently selected items.
    ///
    /// See [C++ `wxGenericDirCtrl::UnselectAll()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#a40f845ec747803acf66037a03499b9ef).
    fn unselect_all(&self) {
        unsafe { ffi::wxGenericDirCtrl_UnselectAll(self.as_ptr()) }
    }
}

// wxGenericProgressDialog
/// This trait represents [C++ `wxGenericProgressDialog` class](https://docs.wxwidgets.org/3.2/classwx_generic_progress_dialog.html)'s methods and inheritance.
///
/// See [`GenericProgressDialogIsOwned`] documentation for the class usage.
pub trait GenericProgressDialogMethods: DialogMethods {
    // DTOR: fn ~wxGenericProgressDialog()
    /// Returns the last value passed to the Update() function or wxNOT_FOUND if the dialog has no progress bar.
    ///
    /// See [C++ `wxGenericProgressDialog::GetValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_progress_dialog.html#a08804d1e26708d36b0d4a3b1904ec752).
    fn get_value(&self) -> c_int {
        unsafe { ffi::wxGenericProgressDialog_GetValue(self.as_ptr()) }
    }
    /// Returns the maximum value of the progress meter, as passed to the constructor or wxNOT_FOUND if the dialog has no progress bar.
    ///
    /// See [C++ `wxGenericProgressDialog::GetRange()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_progress_dialog.html#a11ef590c7ee98d3a85b84b72a77f6e7d).
    fn get_range(&self) -> c_int {
        unsafe { ffi::wxGenericProgressDialog_GetRange(self.as_ptr()) }
    }
    /// Returns the last message passed to the Update() function; if you always passed wxEmptyString to Update() then the message set through the constructor is returned.
    ///
    /// See [C++ `wxGenericProgressDialog::GetMessage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_progress_dialog.html#a80f29ad9a70396e5411caaff3dd76413).
    fn get_message(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericProgressDialog_GetMessage(self.as_ptr())).into() }
    }
    /// Like Update() but makes the gauge control run in indeterminate mode.
    ///
    /// See [C++ `wxGenericProgressDialog::Pulse()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_progress_dialog.html#ab5ecf227eebaa1aadb5f5c553e4a4ee5).
    fn pulse(&self, newmsg: &str, skip: *mut c_void) -> bool {
        unsafe {
            let newmsg = WxString::from(newmsg);
            let newmsg = newmsg.as_ptr();
            ffi::wxGenericProgressDialog_Pulse(self.as_ptr(), newmsg, skip)
        }
    }
    /// Can be used to continue with the dialog, after the user had clicked the "Abort" button.
    ///
    /// See [C++ `wxGenericProgressDialog::Resume()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_progress_dialog.html#a076a10d2d6cb1e2561b6f1820663cd70).
    fn resume(&self) {
        unsafe { ffi::wxGenericProgressDialog_Resume(self.as_ptr()) }
    }
    /// Changes the maximum value of the progress meter given in the constructor.
    ///
    /// See [C++ `wxGenericProgressDialog::SetRange()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_progress_dialog.html#af5f24bd05ef8b31dd1991bbc7533cb17).
    fn set_range(&self, maximum: c_int) {
        unsafe { ffi::wxGenericProgressDialog_SetRange(self.as_ptr(), maximum) }
    }
    /// Returns true if the "Cancel" button was pressed.
    ///
    /// See [C++ `wxGenericProgressDialog::WasCancelled()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_progress_dialog.html#a84b2b2c2ff19e416199442d5adbf113e).
    fn was_cancelled(&self) -> bool {
        unsafe { ffi::wxGenericProgressDialog_WasCancelled(self.as_ptr()) }
    }
    /// Returns true if the "Skip" button was pressed.
    ///
    /// See [C++ `wxGenericProgressDialog::WasSkipped()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_progress_dialog.html#a471fe2a16820fd4382189685b9a3ce38).
    fn was_skipped(&self) -> bool {
        unsafe { ffi::wxGenericProgressDialog_WasSkipped(self.as_ptr()) }
    }
    /// Updates the dialog, setting the progress bar to the new value and updating the message if new one is specified.
    ///
    /// See [C++ `wxGenericProgressDialog::Update()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_progress_dialog.html#a1d542cea96c531d85c32d6612bd55a02).
    fn update_int(&self, value: c_int, newmsg: &str, skip: *mut c_void) -> bool {
        unsafe {
            let newmsg = WxString::from(newmsg);
            let newmsg = newmsg.as_ptr();
            ffi::wxGenericProgressDialog_Update(self.as_ptr(), value, newmsg, skip)
        }
    }
}

// wxGenericValidator
/// This trait represents [C++ `wxGenericValidator` class](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html)'s methods and inheritance.
///
/// See [`GenericValidatorIsOwned`] documentation for the class usage.
pub trait GenericValidatorMethods: ValidatorMethods {
    // DTOR: fn ~wxGenericValidator()
}

// wxGraphicsBrush
/// This trait represents [C++ `wxGraphicsBrush` class](https://docs.wxwidgets.org/3.2/classwx_graphics_brush.html)'s methods and inheritance.
///
/// See [`GraphicsBrushIsOwned`] documentation for the class usage.
pub trait GraphicsBrushMethods: GraphicsObjectMethods {}

// wxGraphicsContext
/// This trait represents [C++ `wxGraphicsContext` class](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html)'s methods and inheritance.
///
/// See [`GraphicsContextIsOwned`] documentation for the class usage.
pub trait GraphicsContextMethods: GraphicsObjectMethods {
    /// Creates a wxGraphicsContext from a wxWindow.
    ///
    /// See [C++ `wxGraphicsContext::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#ae8720ba0ce3401fe236449858b7cf950).
    fn create_window<W: WindowMethods>(
        window: Option<&W>,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GraphicsContext::option_from(ffi::wxGraphicsContext_Create(window))
        }
    }
    /// Creates a wxGraphicsContext from a wxWindowDC.
    ///
    /// See [C++ `wxGraphicsContext::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a25dca87b498a8fe9fa23a702fa3a384e).
    fn create_windowdc<W: WindowDCMethods>(window_dc: &W) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let window_dc = window_dc.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsContext_Create1(window_dc))
        }
    }
    /// Creates a wxGraphicsContext from a wxMemoryDC.
    ///
    /// See [C++ `wxGraphicsContext::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#af544047a78a2cb0f1bb216e27ace1d0c).
    fn create_memorydc<M: MemoryDCMethods>(memory_dc: &M) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let memory_dc = memory_dc.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsContext_Create2(memory_dc))
        }
    }
    // BLOCKED: fn Create3()
    /// Creates a wxGraphicsContext from a wxEnhMetaFileDC.
    ///
    /// See [C++ `wxGraphicsContext::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a9a5091dadfe615f0b26825d3389ec734).
    fn create_enhmetafiledc(meta_file_dc: *const c_void) -> Option<GraphicsContextIsOwned<false>> {
        unsafe { GraphicsContext::option_from(ffi::wxGraphicsContext_Create4(meta_file_dc)) }
    }
    /// Creates a wxGraphicsContext from a DC of unknown specific type.
    ///
    /// See [C++ `wxGraphicsContext::CreateFromUnknownDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#aa8000f79cb2123da46b31c15246c7383).
    fn create_from_unknown_dc<D: DCMethods>(dc: &D) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let dc = dc.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsContext_CreateFromUnknownDC(dc))
        }
    }
    /// Creates a wxGraphicsContext associated with a wxImage.
    ///
    /// See [C++ `wxGraphicsContext::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a91dbb89974c189fa4b743ee7dc9ae7cf).
    fn create_image<I: ImageMethods>(image: &I) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let image = image.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsContext_Create5(image))
        }
    }
    /// Creates a wxGraphicsContext from a native context.
    ///
    /// See [C++ `wxGraphicsContext::CreateFromNative()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#ad74196005aaf6c1e346a52eb042a623f).
    fn create_from_native(context: *mut c_void) -> Option<GraphicsContextIsOwned<false>> {
        unsafe { GraphicsContext::option_from(ffi::wxGraphicsContext_CreateFromNative(context)) }
    }
    /// Creates a wxGraphicsContext from a native window.
    ///
    /// See [C++ `wxGraphicsContext::CreateFromNativeWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a538b8d96804b12f6f32d159eef03d919).
    fn create_from_native_window(window: *mut c_void) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            GraphicsContext::option_from(ffi::wxGraphicsContext_CreateFromNativeWindow(window))
        }
    }
    // NOT_SUPPORTED: fn CreateFromNativeHDC()
    /// Create a lightweight context that can be used only for measuring text.
    ///
    /// See [C++ `wxGraphicsContext::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a5be9099dbe7c3ca825aa55bdcc541cf5).
    fn create() -> Option<GraphicsContextIsOwned<false>> {
        unsafe { GraphicsContext::option_from(ffi::wxGraphicsContext_Create6()) }
    }
    /// Resets the clipping to original shape.
    ///
    /// See [C++ `wxGraphicsContext::ResetClip()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a41edbc852f1f3a1393059e44e29d500d).
    fn reset_clip(&self) {
        unsafe { ffi::wxGraphicsContext_ResetClip(self.as_ptr()) }
    }
    /// Sets the clipping region to the intersection of the given region and the previously set clipping region.
    ///
    /// See [C++ `wxGraphicsContext::Clip()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#aacdbd6d9f656007791cc57192d5a3d7a).
    fn clip<R: RegionMethods>(&self, region: &R) {
        unsafe {
            let region = region.as_ptr();
            ffi::wxGraphicsContext_Clip(self.as_ptr(), region)
        }
    }
    // NOT_SUPPORTED: fn Clip1()
    /// Returns bounding box of the current clipping region.
    ///
    /// See [C++ `wxGraphicsContext::GetClipBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a25eac8ed5203b7b696a42982a6c31aae).
    fn get_clip_box(&self, x: *mut c_void, y: *mut c_void, w: *mut c_void, h: *mut c_void) {
        unsafe { ffi::wxGraphicsContext_GetClipBox(self.as_ptr(), x, y, w, h) }
    }
    // NOT_SUPPORTED: fn CreateMatrix()
    /// Creates a native affine transformation matrix from the passed generic one.
    ///
    /// See [C++ `wxGraphicsContext::CreateMatrix()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a299bf382fd5f22e11d9955db5574381a).
    fn create_matrix<A: AffineMatrix2DBaseMethods>(&self, mat: &A) -> GraphicsMatrix {
        unsafe {
            let mat = mat.as_ptr();
            GraphicsMatrix::from_ptr(ffi::wxGraphicsContext_CreateMatrix1(self.as_ptr(), mat))
        }
    }
    /// Concatenates the passed in transform with the current transform of this context.
    ///
    /// See [C++ `wxGraphicsContext::ConcatTransform()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a27a0d32bd48956ffac1acc4b44f268e9).
    fn concat_transform<G: GraphicsMatrixMethods>(&self, matrix: &G) {
        unsafe {
            let matrix = matrix.as_ptr();
            ffi::wxGraphicsContext_ConcatTransform(self.as_ptr(), matrix)
        }
    }
    /// Gets the current transformation matrix of this context.
    ///
    /// See [C++ `wxGraphicsContext::GetTransform()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#af9352912c014c93414f99d39761f5682).
    fn get_transform(&self) -> GraphicsMatrix {
        unsafe { GraphicsMatrix::from_ptr(ffi::wxGraphicsContext_GetTransform(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn Rotate()
    // NOT_SUPPORTED: fn Scale()
    /// Sets the current transformation matrix of this context.
    ///
    /// See [C++ `wxGraphicsContext::SetTransform()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a1d3eb969a3973525523b2e7fadb59f51).
    fn set_transform<G: GraphicsMatrixMethods>(&self, matrix: &G) {
        unsafe {
            let matrix = matrix.as_ptr();
            ffi::wxGraphicsContext_SetTransform(self.as_ptr(), matrix)
        }
    }
    // NOT_SUPPORTED: fn Translate()
    /// Creates a native brush from a wxBrush.
    ///
    /// See [C++ `wxGraphicsContext::CreateBrush()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a4516ae09c39b5cf1e1f4e65761fb3a19).
    fn create_brush<B: BrushMethods>(&self, brush: &B) -> GraphicsBrush {
        unsafe {
            let brush = brush.as_ptr();
            GraphicsBrush::from_ptr(ffi::wxGraphicsContext_CreateBrush(self.as_ptr(), brush))
        }
    }
    // NOT_SUPPORTED: fn CreateLinearGradientBrush()
    // NOT_SUPPORTED: fn CreateLinearGradientBrush1()
    // NOT_SUPPORTED: fn CreateRadialGradientBrush()
    // NOT_SUPPORTED: fn CreateRadialGradientBrush1()
    /// Sets the brush for filling paths.
    ///
    /// See [C++ `wxGraphicsContext::SetBrush()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a622c7f2349d0df34482f47983e2de2fd).
    fn set_brush_brush<B: BrushMethods>(&self, brush: &B) {
        unsafe {
            let brush = brush.as_ptr();
            ffi::wxGraphicsContext_SetBrush(self.as_ptr(), brush)
        }
    }
    /// Sets the brush for filling paths.
    ///
    /// See [C++ `wxGraphicsContext::SetBrush()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#ae7f43bd73c7212db2763c1a0535adb84).
    fn set_brush_graphicsbrush<G: GraphicsBrushMethods>(&self, brush: &G) {
        unsafe {
            let brush = brush.as_ptr();
            ffi::wxGraphicsContext_SetBrush1(self.as_ptr(), brush)
        }
    }
    /// Creates a native pen from a wxPen.
    ///
    /// See [C++ `wxGraphicsContext::CreatePen()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#ad0fcb2f41fd517e417e9bb7ac74587a8).
    fn create_pen_pen<P: PenMethods>(&self, pen: &P) -> GraphicsPen {
        unsafe {
            let pen = pen.as_ptr();
            GraphicsPen::from_ptr(ffi::wxGraphicsContext_CreatePen(self.as_ptr(), pen))
        }
    }
    /// Creates a native pen from a wxGraphicsPenInfo.
    ///
    /// See [C++ `wxGraphicsContext::CreatePen()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a03a0886ccc898a79e70aa55c22e6bf81).
    fn create_pen_graphicspeninfo(&self, info: *const c_void) -> GraphicsPen {
        unsafe { GraphicsPen::from_ptr(ffi::wxGraphicsContext_CreatePen1(self.as_ptr(), info)) }
    }
    /// Sets the pen used for stroking.
    ///
    /// See [C++ `wxGraphicsContext::SetPen()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#ac34d6320c2777fe2afcf8777868e37d3).
    fn set_pen_pen<P: PenMethods>(&self, pen: &P) {
        unsafe {
            let pen = pen.as_ptr();
            ffi::wxGraphicsContext_SetPen(self.as_ptr(), pen)
        }
    }
    /// Sets the pen used for stroking.
    ///
    /// See [C++ `wxGraphicsContext::SetPen()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#acc15194eea4192f622d560bd33233cee).
    fn set_pen_graphicspen<G: GraphicsPenMethods>(&self, pen: &G) {
        unsafe {
            let pen = pen.as_ptr();
            ffi::wxGraphicsContext_SetPen1(self.as_ptr(), pen)
        }
    }
    // NOT_SUPPORTED: fn DrawBitmap()
    // NOT_SUPPORTED: fn DrawBitmap1()
    // NOT_SUPPORTED: fn DrawEllipse()
    // NOT_SUPPORTED: fn DrawIcon()
    // NOT_SUPPORTED: fn DrawLines()
    // NOT_SUPPORTED: fn DrawPath()
    // NOT_SUPPORTED: fn DrawRectangle()
    // NOT_SUPPORTED: fn DrawRoundedRectangle()
    // NOT_SUPPORTED: fn DrawText()
    // NOT_SUPPORTED: fn DrawText1()
    // NOT_SUPPORTED: fn DrawText2()
    // NOT_SUPPORTED: fn DrawText3()
    /// Creates a native graphics path which is initially empty.
    ///
    /// See [C++ `wxGraphicsContext::CreatePath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#acf166e67b5a0b02e418e46d39123e0ca).
    fn create_path(&self) -> GraphicsPath {
        unsafe { GraphicsPath::from_ptr(ffi::wxGraphicsContext_CreatePath(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn FillPath()
    // NOT_SUPPORTED: fn StrokeLine()
    /// Stroke disconnected lines from begin to end points, fastest method available for this purpose.
    ///
    /// See [C++ `wxGraphicsContext::StrokeLines()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#acfb6d98806247a5d06b5b2f5f57972ac).
    fn stroke_lines_point2ddouble(
        &self,
        n: usize,
        begin_points: *const c_void,
        end_points: *const c_void,
    ) {
        unsafe { ffi::wxGraphicsContext_StrokeLines(self.as_ptr(), n, begin_points, end_points) }
    }
    /// Stroke lines connecting all the points.
    ///
    /// See [C++ `wxGraphicsContext::StrokeLines()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a08055b131515a6fb39868ad7c2bf25fd).
    fn stroke_lines(&self, n: usize, points: *const c_void) {
        unsafe { ffi::wxGraphicsContext_StrokeLines1(self.as_ptr(), n, points) }
    }
    /// Strokes along a path with the current pen.
    ///
    /// See [C++ `wxGraphicsContext::StrokePath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a4f7f8f768e84dcdf50493f0e7ef0c00a).
    fn stroke_path<G: GraphicsPathMethods>(&self, path: &G) {
        unsafe {
            let path = path.as_ptr();
            ffi::wxGraphicsContext_StrokePath(self.as_ptr(), path)
        }
    }
    /// Creates a native graphics font from a wxFont and a text colour.
    ///
    /// See [C++ `wxGraphicsContext::CreateFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a27be36e04ed80b0ac7aac5b657d496cd).
    fn create_font_font<F: FontMethods, C: ColourMethods>(
        &self,
        font: &F,
        col: &C,
    ) -> GraphicsFont {
        unsafe {
            let font = font.as_ptr();
            let col = col.as_ptr();
            GraphicsFont::from_ptr(ffi::wxGraphicsContext_CreateFont(self.as_ptr(), font, col))
        }
    }
    /// Creates a font object with the specified attributes.
    ///
    /// See [C++ `wxGraphicsContext::CreateFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#ac1d467c23ea43d1a402d63b674bcd18e).
    fn create_font_double<C: ColourMethods>(
        &self,
        size_in_pixels: c_double,
        facename: &str,
        flags: c_int,
        col: &C,
    ) -> GraphicsFont {
        unsafe {
            let facename = WxString::from(facename);
            let facename = facename.as_ptr();
            let col = col.as_ptr();
            GraphicsFont::from_ptr(ffi::wxGraphicsContext_CreateFont1(
                self.as_ptr(),
                size_in_pixels,
                facename,
                flags,
                col,
            ))
        }
    }
    /// Sets the font for drawing text.
    ///
    /// See [C++ `wxGraphicsContext::SetFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a8a1756278f0a3f7125280ac1aeedd135).
    fn set_font_font<F: FontMethods, C: ColourMethods>(&self, font: &F, colour: &C) {
        unsafe {
            let font = font.as_ptr();
            let colour = colour.as_ptr();
            ffi::wxGraphicsContext_SetFont(self.as_ptr(), font, colour)
        }
    }
    /// Sets the font for drawing text.
    ///
    /// See [C++ `wxGraphicsContext::SetFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#adcd4539f8db8200d9ddc47385a142328).
    fn set_font_graphicsfont<G: GraphicsFontMethods>(&self, font: &G) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxGraphicsContext_SetFont1(self.as_ptr(), font)
        }
    }
    /// Fills the widths array with the widths from the beginning of text to the corresponding character of text.
    ///
    /// See [C++ `wxGraphicsContext::GetPartialTextExtents()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a833d7b036804e46cd2f1fd7bacb1db62).
    fn get_partial_text_extents(&self, text: &str, widths: *mut c_void) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxGraphicsContext_GetPartialTextExtents(self.as_ptr(), text, widths)
        }
    }
    /// Gets the dimensions of the string using the currently selected font.
    ///
    /// See [C++ `wxGraphicsContext::GetTextExtent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a32afcb5c7ed4d75d473c47c5b567198a).
    fn get_text_extent(
        &self,
        text: &str,
        width: *mut c_void,
        height: *mut c_void,
        descent: *mut c_void,
        external_leading: *mut c_void,
    ) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxGraphicsContext_GetTextExtent(
                self.as_ptr(),
                text,
                width,
                height,
                descent,
                external_leading,
            )
        }
    }
    /// Begin a new document (relevant only for printing / pdf etc.) If there is a progress dialog, message will be shown.
    ///
    /// See [C++ `wxGraphicsContext::StartDoc()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#aafa1fe3a6e692dde741505e0bf1c90c8).
    fn start_doc(&self, message: &str) -> bool {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxGraphicsContext_StartDoc(self.as_ptr(), message)
        }
    }
    /// Done with that document (relevant only for printing / pdf etc.)
    ///
    /// See [C++ `wxGraphicsContext::EndDoc()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a21d6014bb8529002eff2be6bcd2572f1).
    fn end_doc(&self) {
        unsafe { ffi::wxGraphicsContext_EndDoc(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn StartPage()
    /// Ends the current page (relevant only for printing / pdf etc.)
    ///
    /// See [C++ `wxGraphicsContext::EndPage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a33b54026827258d12aa9a7c64a69464e).
    fn end_page(&self) {
        unsafe { ffi::wxGraphicsContext_EndPage(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn CreateBitmap()
    // NOT_SUPPORTED: fn CreateBitmapFromImage()
    // NOT_SUPPORTED: fn CreateSubBitmap()
    // NOT_SUPPORTED: fn BeginLayer()
    /// Composites back the drawings into the context with the opacity given at the BeginLayer() call.
    ///
    /// See [C++ `wxGraphicsContext::EndLayer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a5135c2f7c70eab04fe68532ccf3d6f95).
    fn end_layer(&self) {
        unsafe { ffi::wxGraphicsContext_EndLayer(self.as_ptr()) }
    }
    /// Push the current state (like transformations, clipping region and quality settings) of the context on a stack.
    ///
    /// See [C++ `wxGraphicsContext::PushState()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#af2548831583e900dd74462c7d58fec47).
    fn push_state(&self) {
        unsafe { ffi::wxGraphicsContext_PushState(self.as_ptr()) }
    }
    /// Sets current state of the context to the state saved by a preceding call to PushState() and removes that state from the stack of saved states.
    ///
    /// See [C++ `wxGraphicsContext::PopState()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a396b928674376f503aa3b2959e39c473).
    fn pop_state(&self) {
        unsafe { ffi::wxGraphicsContext_PopState(self.as_ptr()) }
    }
    /// Make sure that the current content of this context is immediately visible.
    ///
    /// See [C++ `wxGraphicsContext::Flush()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#aebfbc516794756202ebfc2d4e153c9e9).
    fn flush(&self) {
        unsafe { ffi::wxGraphicsContext_Flush(self.as_ptr()) }
    }
    /// Returns the native context (CGContextRef for Core Graphics, Graphics pointer for GDIPlus and cairo_t pointer for cairo).
    ///
    /// See [C++ `wxGraphicsContext::GetNativeContext()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a530df82b84aa33456bf5d24990a2e031).
    fn get_native_context(&self) -> *mut c_void {
        unsafe { ffi::wxGraphicsContext_GetNativeContext(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetAntialiasMode()
    // NOT_SUPPORTED: fn GetAntialiasMode()
    // NOT_SUPPORTED: fn SetInterpolationQuality()
    // NOT_SUPPORTED: fn GetInterpolationQuality()
    // NOT_SUPPORTED: fn SetCompositionMode()
    // NOT_SUPPORTED: fn GetCompositionMode()
    /// Returns the size of the graphics context in device coordinates.
    ///
    /// See [C++ `wxGraphicsContext::GetSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a018be4882b4a89f5a3dc33debb4337af).
    fn get_size(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxGraphicsContext_GetSize(self.as_ptr(), width, height) }
    }
    /// Returns the resolution of the graphics context in device points per inch.
    ///
    /// See [C++ `wxGraphicsContext::GetDPI()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#afbc39e0316871a429748b0567ed92112).
    fn get_dpi(&self, dpi_x: *mut c_void, dpi_y: *mut c_void) {
        unsafe { ffi::wxGraphicsContext_GetDPI(self.as_ptr(), dpi_x, dpi_y) }
    }
    /// Returns the associated window if any.
    ///
    /// See [C++ `wxGraphicsContext::GetWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a1cad810a6544ed6c624a145c61061c10).
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxGraphicsContext_GetWindow(self.as_ptr())) }
    }
    /// Helper to determine if a 0.5 offset should be applied for the drawing operation.
    ///
    /// See [C++ `wxGraphicsContext::ShouldOffset()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#ae63c8f7649b2b3df71b5e4bc867f4186).
    fn should_offset(&self) -> bool {
        unsafe { ffi::wxGraphicsContext_ShouldOffset(self.as_ptr()) }
    }
    /// Indicates whether the context should try to offset for pixel boundaries.
    ///
    /// See [C++ `wxGraphicsContext::EnableOffset()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a80d7a1215921597ef049b224708fb321).
    fn enable_offset(&self, enable: bool) {
        unsafe { ffi::wxGraphicsContext_EnableOffset(self.as_ptr(), enable) }
    }
    /// Helper to determine if a 0.5 offset should be applied for the drawing operation.
    ///
    /// See [C++ `wxGraphicsContext::DisableOffset()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#aab2095c1335c9b9476f73220aacd1f0e).
    fn disable_offset(&self) {
        unsafe { ffi::wxGraphicsContext_DisableOffset(self.as_ptr()) }
    }
    /// Helper to determine if a 0.5 offset should be applied for the drawing operation.
    ///
    /// See [C++ `wxGraphicsContext::OffsetEnabled()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#ae6acf61cd09fd305c99692d340c93e76).
    fn offset_enabled(&self) -> bool {
        unsafe { ffi::wxGraphicsContext_OffsetEnabled(self.as_ptr()) }
    }
    /// Convert DPI-independent pixel values to the value in pixels appropriate for the graphics context.
    ///
    /// See [C++ `wxGraphicsContext::FromDIP()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a01ed2c8aaf6a06588ca068920a9b7fd1).
    fn from_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxGraphicsContext_FromDIP(self.as_ptr(), sz))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxGraphicsContext::FromDIP()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#ac26c87efb8517022df9ad2c81103418c).
    fn from_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxGraphicsContext_FromDIP1(self.as_ptr(), pt))
        }
    }
    /// Convert DPI-independent value in pixels to the value in pixels appropriate for the graphics context.
    ///
    /// See [C++ `wxGraphicsContext::FromDIP()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a4319b176fa7408926bee7acd475f4976).
    fn from_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxGraphicsContext_FromDIP2(self.as_ptr(), d) }
    }
    /// Convert pixel values of the current graphics context to DPI-independent pixel values.
    ///
    /// See [C++ `wxGraphicsContext::ToDIP()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a049abe7a59f6e6ce874b8b9dbd00a72b).
    fn to_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxGraphicsContext_ToDIP(self.as_ptr(), sz))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxGraphicsContext::ToDIP()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#a98131a2c87e43f474f47541a76ec4258).
    fn to_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxGraphicsContext_ToDIP1(self.as_ptr(), pt))
        }
    }
    /// Convert pixel values of the current graphics context to DPI-independent pixel values.
    ///
    /// See [C++ `wxGraphicsContext::ToDIP()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html#ac8acf884ba9158b91bc935b8cc79736a).
    fn to_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxGraphicsContext_ToDIP2(self.as_ptr(), d) }
    }
}

// wxGraphicsFont
/// This trait represents [C++ `wxGraphicsFont` class](https://docs.wxwidgets.org/3.2/classwx_graphics_font.html)'s methods and inheritance.
///
/// See [`GraphicsFontIsOwned`] documentation for the class usage.
pub trait GraphicsFontMethods: GraphicsObjectMethods {}

// wxGraphicsGradientStop
/// This trait represents [C++ `wxGraphicsGradientStop` class](https://docs.wxwidgets.org/3.2/classwx_graphics_gradient_stop.html)'s methods and inheritance.
///
/// See [`GraphicsGradientStopIsOwned`] documentation for the class usage.
pub trait GraphicsGradientStopMethods: WxRustMethods {
    /// Return the stop colour.
    ///
    /// See [C++ `wxGraphicsGradientStop::GetColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_gradient_stop.html#a3d18a62d5d55701d3b9df7bfad7d72d4).
    fn get_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxGraphicsGradientStop_GetColour(self.as_ptr())) }
    }
    /// Change the stop colour.
    ///
    /// See [C++ `wxGraphicsGradientStop::SetColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_gradient_stop.html#ad50f38b4a022989dad00d4186c0b930e).
    fn set_colour<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxGraphicsGradientStop_SetColour(self.as_ptr(), col)
        }
    }
    // NOT_SUPPORTED: fn GetPosition()
    // NOT_SUPPORTED: fn SetPosition()
}

// wxGraphicsGradientStops
/// This trait represents [C++ `wxGraphicsGradientStops` class](https://docs.wxwidgets.org/3.2/classwx_graphics_gradient_stops.html)'s methods and inheritance.
///
/// See [`GraphicsGradientStopsIsOwned`] documentation for the class usage.
pub trait GraphicsGradientStopsMethods: WxRustMethods {
    /// Add a new stop.
    ///
    /// See [C++ `wxGraphicsGradientStops::Add()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_gradient_stops.html#ad983c9678da51616ef1107522b9d5f51).
    fn add<G: GraphicsGradientStopMethods>(&self, stop: &G) {
        unsafe {
            let stop = stop.as_ptr();
            ffi::wxGraphicsGradientStops_Add(self.as_ptr(), stop)
        }
    }
    // NOT_SUPPORTED: fn Add1()
    // NOT_SUPPORTED: fn Item()
    /// Returns the number of stops.
    ///
    /// See [C++ `wxGraphicsGradientStops::GetCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_gradient_stops.html#a9279f8d41db201dee4527d143986ed33).
    fn get_count(&self) -> usize {
        unsafe { ffi::wxGraphicsGradientStops_GetCount(self.as_ptr()) }
    }
    // BLOCKED: fn SetStartColour()
    /// Returns the start colour.
    ///
    /// See [C++ `wxGraphicsGradientStops::GetStartColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_gradient_stops.html#ab1791b1ff9786eae82d0aa5f075cd86b).
    fn get_start_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxGraphicsGradientStops_GetStartColour(self.as_ptr())) }
    }
    // BLOCKED: fn SetEndColour()
    /// Returns the end colour.
    ///
    /// See [C++ `wxGraphicsGradientStops::GetEndColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_gradient_stops.html#a44f9e60bdfb06e84ed052b1c1b26ebb4).
    fn get_end_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxGraphicsGradientStops_GetEndColour(self.as_ptr())) }
    }
}

// wxGraphicsMatrix
/// This trait represents [C++ `wxGraphicsMatrix` class](https://docs.wxwidgets.org/3.2/classwx_graphics_matrix.html)'s methods and inheritance.
///
/// See [`GraphicsMatrixIsOwned`] documentation for the class usage.
pub trait GraphicsMatrixMethods: GraphicsObjectMethods {
    /// Concatenates the matrix passed with the current matrix.
    ///
    /// See [C++ `wxGraphicsMatrix::Concat()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_matrix.html#a382d94e2ed9445408e4ac158af667196).
    fn concat<G: GraphicsMatrixMethods>(&self, t: Option<&G>) {
        unsafe {
            let t = match t {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGraphicsMatrix_Concat(self.as_ptr(), t)
        }
    }
    // BLOCKED: fn Concat1()
    /// Returns the component values of the matrix via the argument pointers.
    ///
    /// See [C++ `wxGraphicsMatrix::Get()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_matrix.html#ae5d9afdab4e1b9b7cf4b9560c5ac11dc).
    fn get(
        &self,
        a: *mut c_void,
        b: *mut c_void,
        c: *mut c_void,
        d: *mut c_void,
        tx: *mut c_void,
        ty: *mut c_void,
    ) {
        unsafe { ffi::wxGraphicsMatrix_Get(self.as_ptr(), a, b, c, d, tx, ty) }
    }
    /// Returns the native representation of the matrix.
    ///
    /// See [C++ `wxGraphicsMatrix::GetNativeMatrix()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_matrix.html#a196f90f908d9c42aa898c164a7ab3900).
    fn get_native_matrix(&self) -> *mut c_void {
        unsafe { ffi::wxGraphicsMatrix_GetNativeMatrix(self.as_ptr()) }
    }
    /// Inverts the matrix.
    ///
    /// See [C++ `wxGraphicsMatrix::Invert()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_matrix.html#a4e83f56734919d83d8cc0d5c56b0f11c).
    fn invert(&self) {
        unsafe { ffi::wxGraphicsMatrix_Invert(self.as_ptr()) }
    }
    // BLOCKED: fn IsEqual()
    /// Returns true if the elements of the transformation matrix are equal.
    ///
    /// See [C++ `wxGraphicsMatrix::IsEqual()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_matrix.html#a35e88e44e98b0355c0133aa5dce0ad23).
    fn is_equal<G: GraphicsMatrixMethods>(&self, t: &G) -> bool {
        unsafe {
            let t = t.as_ptr();
            ffi::wxGraphicsMatrix_IsEqual1(self.as_ptr(), t)
        }
    }
    /// Return true if this is the identity matrix.
    ///
    /// See [C++ `wxGraphicsMatrix::IsIdentity()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_matrix.html#acba6effdb77b23e32dbc5ccb99602a6f).
    fn is_identity(&self) -> bool {
        unsafe { ffi::wxGraphicsMatrix_IsIdentity(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Rotate()
    // NOT_SUPPORTED: fn Scale()
    // NOT_SUPPORTED: fn Set()
    /// Applies this matrix to a distance (ie.
    ///
    /// See [C++ `wxGraphicsMatrix::TransformDistance()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_matrix.html#a3c091a79ad14e942aa256ed37b6080e9).
    fn transform_distance(&self, dx: *mut c_void, dy: *mut c_void) {
        unsafe { ffi::wxGraphicsMatrix_TransformDistance(self.as_ptr(), dx, dy) }
    }
    /// Applies this matrix to a point.
    ///
    /// See [C++ `wxGraphicsMatrix::TransformPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_matrix.html#a10968c615ae3a91fa09aedf0f3607be5).
    fn transform_point(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxGraphicsMatrix_TransformPoint(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn Translate()
}

// wxGraphicsObject
/// This trait represents [C++ `wxGraphicsObject` class](https://docs.wxwidgets.org/3.2/classwx_graphics_object.html)'s methods and inheritance.
///
/// See [`GraphicsObjectIsOwned`] documentation for the class usage.
pub trait GraphicsObjectMethods: ObjectMethods {
    /// Returns the renderer that was used to create this instance, or NULL if it has not been initialized yet.
    ///
    /// See [C++ `wxGraphicsObject::GetRenderer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_object.html#a0c39cbf592f20ac90b3718d399210e1e).
    fn get_renderer(&self) -> Option<GraphicsRendererIsOwned<false>> {
        unsafe { GraphicsRenderer::option_from(ffi::wxGraphicsObject_GetRenderer(self.as_ptr())) }
    }
    ///
    /// See [C++ `wxGraphicsObject::IsNull()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_object.html#a217fe7a511a2a20f4a39089329e5b6e5).
    fn is_null(&self) -> bool {
        unsafe { ffi::wxGraphicsObject_IsNull(self.as_ptr()) }
    }
}

// wxGraphicsPath
/// This trait represents [C++ `wxGraphicsPath` class](https://docs.wxwidgets.org/3.2/classwx_graphics_path.html)'s methods and inheritance.
///
/// See [`GraphicsPathIsOwned`] documentation for the class usage.
pub trait GraphicsPathMethods: GraphicsObjectMethods {
    // NOT_SUPPORTED: fn AddArc()
    // NOT_SUPPORTED: fn AddArc1()
    // NOT_SUPPORTED: fn AddArcToPoint()
    // NOT_SUPPORTED: fn AddCircle()
    // NOT_SUPPORTED: fn AddCurveToPoint()
    /// Adds a cubic bezier curve from the current point, using two control points and an end point.
    ///
    /// See [C++ `wxGraphicsPath::AddCurveToPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_path.html#a513c48b6ef2cee88b57b09d19af641ee).
    fn add_curve_to_point(&self, c1: *const c_void, c2: *const c_void, e: *const c_void) {
        unsafe { ffi::wxGraphicsPath_AddCurveToPoint1(self.as_ptr(), c1, c2, e) }
    }
    // NOT_SUPPORTED: fn AddEllipse()
    // NOT_SUPPORTED: fn AddLineToPoint()
    /// Adds a straight line from the current point to p.
    ///
    /// See [C++ `wxGraphicsPath::AddLineToPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_path.html#a3df39365090cffcd4b86ec2987833a29).
    fn add_line_to_point(&self, p: *const c_void) {
        unsafe { ffi::wxGraphicsPath_AddLineToPoint1(self.as_ptr(), p) }
    }
    /// Adds another path onto the current path.
    ///
    /// See [C++ `wxGraphicsPath::AddPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_path.html#ab1d0507f094a4f65df8d1a8e0a40dba9).
    fn add_path<G: GraphicsPathMethods>(&self, path: &G) {
        unsafe {
            let path = path.as_ptr();
            ffi::wxGraphicsPath_AddPath(self.as_ptr(), path)
        }
    }
    // NOT_SUPPORTED: fn AddQuadCurveToPoint()
    // NOT_SUPPORTED: fn AddRectangle()
    // NOT_SUPPORTED: fn AddRoundedRectangle()
    /// Closes the current sub-path.
    ///
    /// See [C++ `wxGraphicsPath::CloseSubpath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_path.html#a93ee08b5acd2a2aa82dceb4bbd9cc819).
    fn close_subpath(&self) {
        unsafe { ffi::wxGraphicsPath_CloseSubpath(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Contains()
    // NOT_SUPPORTED: fn Contains1()
    // NOT_SUPPORTED: fn GetBox()
    /// Gets the bounding box enclosing all points (possibly including control points).
    ///
    /// See [C++ `wxGraphicsPath::GetBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_path.html#aada0b73af79007e1d018c05d2dc75538).
    fn get_box(&self, x: *mut c_void, y: *mut c_void, w: *mut c_void, h: *mut c_void) {
        unsafe { ffi::wxGraphicsPath_GetBox1(self.as_ptr(), x, y, w, h) }
    }
    /// Gets the last point of the current path, (0,0) if not yet set.
    ///
    /// See [C++ `wxGraphicsPath::GetCurrentPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_path.html#a5544ac944d9d6b1f094ecba8c512508f).
    fn get_current_point(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxGraphicsPath_GetCurrentPoint(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn GetCurrentPoint1()
    /// Returns the native path (CGPathRef for Core Graphics, Path pointer for GDIPlus and a cairo_path_t pointer for cairo).
    ///
    /// See [C++ `wxGraphicsPath::GetNativePath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_path.html#a2368804660d6324f2a8d56324b4aaf90).
    fn get_native_path(&self) -> *mut c_void {
        unsafe { ffi::wxGraphicsPath_GetNativePath(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn MoveToPoint()
    /// Begins a new subpath at p.
    ///
    /// See [C++ `wxGraphicsPath::MoveToPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_path.html#a42a800a76412ad4ae2eb276ba4016f37).
    fn move_to_point(&self, p: *const c_void) {
        unsafe { ffi::wxGraphicsPath_MoveToPoint1(self.as_ptr(), p) }
    }
    /// Transforms each point of this path by the matrix.
    ///
    /// See [C++ `wxGraphicsPath::Transform()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_path.html#a6a6a766853d1e57678c2d54e2eea490d).
    fn transform<G: GraphicsMatrixMethods>(&self, matrix: &G) {
        unsafe {
            let matrix = matrix.as_ptr();
            ffi::wxGraphicsPath_Transform(self.as_ptr(), matrix)
        }
    }
    /// Gives back the native path returned by GetNativePath() because there might be some deallocations necessary (e.g.
    ///
    /// See [C++ `wxGraphicsPath::UnGetNativePath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_path.html#a060e3ef3670852aa8a4b80da06ed5a6c).
    fn un_get_native_path(&self, p: *mut c_void) {
        unsafe { ffi::wxGraphicsPath_UnGetNativePath(self.as_ptr(), p) }
    }
}

// wxGraphicsPen
/// This trait represents [C++ `wxGraphicsPen` class](https://docs.wxwidgets.org/3.2/classwx_graphics_pen.html)'s methods and inheritance.
///
/// See [`GraphicsPenIsOwned`] documentation for the class usage.
pub trait GraphicsPenMethods: GraphicsObjectMethods {}

// wxGraphicsRenderer
/// This trait represents [C++ `wxGraphicsRenderer` class](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html)'s methods and inheritance.
///
/// See [`GraphicsRendererIsOwned`] documentation for the class usage.
pub trait GraphicsRendererMethods: ObjectMethods {
    // NOT_SUPPORTED: fn CreateBitmap()
    // NOT_SUPPORTED: fn CreateBitmapFromImage()
    /// Creates a wxImage from a wxGraphicsBitmap.
    ///
    /// See [C++ `wxGraphicsRenderer::CreateImageFromBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#ae2297b12e7f4d1c4f1d836d887318317).
    fn create_image_from_bitmap(&self, bmp: *const c_void) -> Image {
        unsafe {
            Image::from_ptr(ffi::wxGraphicsRenderer_CreateImageFromBitmap(
                self.as_ptr(),
                bmp,
            ))
        }
    }
    // NOT_SUPPORTED: fn CreateBitmapFromNativeBitmap()
    /// Creates a wxGraphicsContext from a wxWindow.
    ///
    /// See [C++ `wxGraphicsRenderer::CreateContext()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#a8e9d88060d2f305050007fa0e0f1b5f5).
    fn create_context_window<W: WindowMethods>(
        &self,
        window: Option<&W>,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateContext(
                self.as_ptr(),
                window,
            ))
        }
    }
    /// Creates a wxGraphicsContext from a wxWindowDC.
    ///
    /// See [C++ `wxGraphicsRenderer::CreateContext()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#a917966669506b5086005833a927734f2).
    fn create_context_windowdc<W: WindowDCMethods>(
        &self,
        window_dc: &W,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let window_dc = window_dc.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateContext1(
                self.as_ptr(),
                window_dc,
            ))
        }
    }
    /// Creates a wxGraphicsContext from a wxMemoryDC.
    ///
    /// See [C++ `wxGraphicsRenderer::CreateContext()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#a4db02063f2c1b603c4b017cc3c0140cf).
    fn create_context_memorydc<M: MemoryDCMethods>(
        &self,
        memory_dc: &M,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let memory_dc = memory_dc.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateContext2(
                self.as_ptr(),
                memory_dc,
            ))
        }
    }
    // BLOCKED: fn CreateContext3()
    /// Creates a wxGraphicsContext from a wxEnhMetaFileDC.
    ///
    /// See [C++ `wxGraphicsRenderer::CreateContext()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#a9a84ffc7ec1b606352d26e74221d8be9).
    fn create_context_enhmetafiledc(
        &self,
        meta_file_dc: *const c_void,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateContext4(
                self.as_ptr(),
                meta_file_dc,
            ))
        }
    }
    /// Creates a wxGraphicsContext from a DC of unknown specific type.
    ///
    /// See [C++ `wxGraphicsRenderer::CreateContextFromUnknownDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#a013783b6a162aa69b2fa3bee00df447a).
    fn create_context_from_unknown_dc<D: DCMethods>(
        &self,
        dc: &D,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let dc = dc.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateContextFromUnknownDC(
                self.as_ptr(),
                dc,
            ))
        }
    }
    /// Creates a wxGraphicsContext associated with a wxImage.
    ///
    /// See [C++ `wxGraphicsRenderer::CreateContextFromImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#a784688a2b1020351837790b48ee4cdca).
    fn create_context_from_image<I: ImageMethods>(
        &self,
        image: &I,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let image = image.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateContextFromImage(
                self.as_ptr(),
                image,
            ))
        }
    }
    /// Creates a native brush from a wxBrush.
    ///
    /// See [C++ `wxGraphicsRenderer::CreateBrush()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#af4925b45520135fd41788fc37e244029).
    fn create_brush<B: BrushMethods>(&self, brush: &B) -> GraphicsBrush {
        unsafe {
            let brush = brush.as_ptr();
            GraphicsBrush::from_ptr(ffi::wxGraphicsRenderer_CreateBrush(self.as_ptr(), brush))
        }
    }
    /// Creates a wxGraphicsContext from a native context.
    ///
    /// See [C++ `wxGraphicsRenderer::CreateContextFromNativeContext()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#a876136ecca42eec1221262423dd480cb).
    fn create_context_from_native_context(
        &self,
        context: *mut c_void,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateContextFromNativeContext(
                self.as_ptr(),
                context,
            ))
        }
    }
    /// Creates a wxGraphicsContext from a native window.
    ///
    /// See [C++ `wxGraphicsRenderer::CreateContextFromNativeWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#a1fc536e8ad3ae87cd5979cbb50edc02a).
    fn create_context_from_native_window(
        &self,
        window: *mut c_void,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateContextFromNativeWindow(
                self.as_ptr(),
                window,
            ))
        }
    }
    /// Creates a wxGraphicsContext that can be used for measuring texts only.
    ///
    /// See [C++ `wxGraphicsRenderer::CreateMeasuringContext()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#a5b0e0b419191ca672f3a043ac20f4228).
    fn create_measuring_context(&self) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateMeasuringContext(
                self.as_ptr(),
            ))
        }
    }
    /// Creates a native graphics font from a wxFont and a text colour.
    ///
    /// See [C++ `wxGraphicsRenderer::CreateFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#afebce1c20aa5d61fffb91b968e2d630d).
    fn create_font_font<F: FontMethods, C: ColourMethods>(
        &self,
        font: &F,
        col: &C,
    ) -> GraphicsFont {
        unsafe {
            let font = font.as_ptr();
            let col = col.as_ptr();
            GraphicsFont::from_ptr(ffi::wxGraphicsRenderer_CreateFont(self.as_ptr(), font, col))
        }
    }
    /// Creates a graphics font with the given characteristics.
    ///
    /// See [C++ `wxGraphicsRenderer::CreateFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#a6969476be51ad558d09687f727918ed1).
    fn create_font_double<C: ColourMethods>(
        &self,
        size_in_pixels: c_double,
        facename: &str,
        flags: c_int,
        col: &C,
    ) -> GraphicsFont {
        unsafe {
            let facename = WxString::from(facename);
            let facename = facename.as_ptr();
            let col = col.as_ptr();
            GraphicsFont::from_ptr(ffi::wxGraphicsRenderer_CreateFont1(
                self.as_ptr(),
                size_in_pixels,
                facename,
                flags,
                col,
            ))
        }
    }
    /// Creates a native graphics font from a wxFont and a text colour.
    ///
    /// See [C++ `wxGraphicsRenderer::CreateFontAtDPI()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#a3b2de8e1a3b5f2d9cb6b20be22c8892b).
    fn create_font_at_dpi<F: FontMethods, R: RealPointMethods, C: ColourMethods>(
        &self,
        font: &F,
        dpi: &R,
        col: &C,
    ) -> GraphicsFont {
        unsafe {
            let font = font.as_ptr();
            let dpi = dpi.as_ptr();
            let col = col.as_ptr();
            GraphicsFont::from_ptr(ffi::wxGraphicsRenderer_CreateFontAtDPI(
                self.as_ptr(),
                font,
                dpi,
                col,
            ))
        }
    }
    // NOT_SUPPORTED: fn CreateLinearGradientBrush()
    // NOT_SUPPORTED: fn CreateMatrix()
    /// Creates a native graphics path which is initially empty.
    ///
    /// See [C++ `wxGraphicsRenderer::CreatePath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#acdaf5505ed3833c4cb76112d001b8717).
    fn create_path(&self) -> GraphicsPath {
        unsafe { GraphicsPath::from_ptr(ffi::wxGraphicsRenderer_CreatePath(self.as_ptr())) }
    }
    /// Creates a native pen from its description.
    ///
    /// See [C++ `wxGraphicsRenderer::CreatePen()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#addc5c8708c8cf97e51eab9f6bb2a43c3).
    fn create_pen(&self, info: *const c_void) -> GraphicsPen {
        unsafe { GraphicsPen::from_ptr(ffi::wxGraphicsRenderer_CreatePen(self.as_ptr(), info)) }
    }
    // NOT_SUPPORTED: fn CreateRadialGradientBrush()
    // NOT_SUPPORTED: fn CreateSubBitmap()
    /// Returns the name of the technology used by the renderer.
    ///
    /// See [C++ `wxGraphicsRenderer::GetName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#afba6de5b8d2415167cfe5a1d6c179003).
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGraphicsRenderer_GetName(self.as_ptr())).into() }
    }
    /// Returns the version major, minor and micro/build of the technology used by the renderer.
    ///
    /// See [C++ `wxGraphicsRenderer::GetVersion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#a4b2a82b70638120b6d0b811d453cae0e).
    fn get_version(&self, major: *mut c_void, minor: *mut c_void, micro: *mut c_void) {
        unsafe { ffi::wxGraphicsRenderer_GetVersion(self.as_ptr(), major, minor, micro) }
    }
    // NOT_SUPPORTED: fn CreateContextFromNativeHDC()
    /// Returns the default renderer on this platform.
    ///
    /// See [C++ `wxGraphicsRenderer::GetDefaultRenderer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#a45ad50c976863fa5bba7a63d69599b40).
    fn get_default_renderer() -> Option<GraphicsRendererIsOwned<false>> {
        unsafe { GraphicsRenderer::option_from(ffi::wxGraphicsRenderer_GetDefaultRenderer()) }
    }
    /// Returns Cairo renderer.
    ///
    /// See [C++ `wxGraphicsRenderer::GetCairoRenderer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#a82933112c5ee82ca5bdee0b8cac4a1e2).
    fn get_cairo_renderer() -> Option<GraphicsRendererIsOwned<false>> {
        unsafe { GraphicsRenderer::option_from(ffi::wxGraphicsRenderer_GetCairoRenderer()) }
    }
    /// Returns GDI+ renderer (MSW only).
    ///
    /// See [C++ `wxGraphicsRenderer::GetGDIPlusRenderer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#a29c172723b5354c64ac0d80b0dfb0037).
    fn get_gdi_plus_renderer() -> Option<GraphicsRendererIsOwned<false>> {
        unsafe { GraphicsRenderer::option_from(ffi::wxGraphicsRenderer_GetGDIPlusRenderer()) }
    }
    /// Returns Direct2D renderer (MSW only).
    ///
    /// See [C++ `wxGraphicsRenderer::GetDirect2DRenderer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html#afb1995eef02b02c7724dcd90443c1c2d).
    fn get_direct2_d_renderer() -> Option<GraphicsRendererIsOwned<false>> {
        unsafe { GraphicsRenderer::option_from(ffi::wxGraphicsRenderer_GetDirect2DRenderer()) }
    }
}

// wxGridBagSizer
/// This trait represents [C++ `wxGridBagSizer` class](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html)'s methods and inheritance.
///
/// See [`GridBagSizerIsOwned`] documentation for the class usage.
pub trait GridBagSizerMethods: FlexGridSizerMethods {
    /// Adds the given item to the given position.
    ///
    /// See [C++ `wxGridBagSizer::Add()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#aea9b458fea699140a04bcda724266ea1).
    fn add_window_gbposition<
        W: WindowMethods,
        G: GBPositionMethods,
        G2: GBSpanMethods,
        O: ObjectMethods,
    >(
        &self,
        window: Option<&W>,
        pos: &G,
        span: &G2,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> Option<SizerItemIsOwned<false>> {
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
            SizerItem::option_from(ffi::wxGridBagSizer_Add(
                self.as_ptr(),
                window,
                pos,
                span,
                flag,
                border,
                user_data,
            ))
        }
    }
    ///
    /// See [C++ `wxGridBagSizer::Add()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#a50a68bdbe4249b2068f9a5685398e8f8).
    fn add_sizer_gbposition<
        S: SizerMethods,
        G: GBPositionMethods,
        G2: GBSpanMethods,
        O: ObjectMethods,
    >(
        &self,
        sizer: Option<&S>,
        pos: &G,
        span: &G2,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> Option<SizerItemIsOwned<false>> {
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
            SizerItem::option_from(ffi::wxGridBagSizer_Add1(
                self.as_ptr(),
                sizer,
                pos,
                span,
                flag,
                border,
                user_data,
            ))
        }
    }
    ///
    /// See [C++ `wxGridBagSizer::Add()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#a5addd89bad9b240fefeb19fd65383031).
    fn add_gbsizeritem<G: GBSizerItemMethods>(
        &self,
        item: Option<&G>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxGridBagSizer_Add2(self.as_ptr(), item))
        }
    }
    /// Adds a spacer to the given position.
    ///
    /// See [C++ `wxGridBagSizer::Add()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#a7a4689838ea956d82fb00398f3e05b88).
    fn add_int_gbposition<G: GBPositionMethods, G2: GBSpanMethods, O: ObjectMethods>(
        &self,
        width: c_int,
        height: c_int,
        pos: &G,
        span: &G2,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxGridBagSizer_Add3(
                self.as_ptr(),
                width,
                height,
                pos,
                span,
                flag,
                border,
                user_data,
            ))
        }
    }
    /// Look at all items and see if any intersect (or would overlap) the given item.
    ///
    /// See [C++ `wxGridBagSizer::CheckForIntersection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#a6eeacb196cb017f5772c5efbb041af89).
    fn check_for_intersection_gbsizeritem<G: GBSizerItemMethods, G2: GBSizerItemMethods>(
        &self,
        item: Option<&G>,
        exclude_item: Option<&G2>,
    ) -> bool {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let exclude_item = match exclude_item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGridBagSizer_CheckForIntersection(self.as_ptr(), item, exclude_item)
        }
    }
    ///
    /// See [C++ `wxGridBagSizer::CheckForIntersection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#a8744578298852be7556d94f0bf8064e9).
    fn check_for_intersection_gbposition<
        G: GBPositionMethods,
        G2: GBSpanMethods,
        G3: GBSizerItemMethods,
    >(
        &self,
        pos: &G,
        span: &G2,
        exclude_item: Option<&G3>,
    ) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            let exclude_item = match exclude_item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGridBagSizer_CheckForIntersection1(self.as_ptr(), pos, span, exclude_item)
        }
    }
    /// Find the sizer item for the given window or subsizer, returns NULL if not found.
    ///
    /// See [C++ `wxGridBagSizer::FindItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#a0c79d6067cf7f24dc93528ff9718bcd3).
    fn find_item_window<W: WindowMethods>(
        &self,
        window: Option<&W>,
    ) -> Option<GBSizerItemIsOwned<false>> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSizerItem::option_from(ffi::wxGridBagSizer_FindItem(self.as_ptr(), window))
        }
    }
    ///
    /// See [C++ `wxGridBagSizer::FindItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#af5615280d778ddb55b7882d96be65d5e).
    fn find_item_sizer<S: SizerMethods>(
        &self,
        sizer: Option<&S>,
    ) -> Option<GBSizerItemIsOwned<false>> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSizerItem::option_from(ffi::wxGridBagSizer_FindItem1(self.as_ptr(), sizer))
        }
    }
    /// Return the sizer item located at the point given in pt, or NULL if there is no item at that point.
    ///
    /// See [C++ `wxGridBagSizer::FindItemAtPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#acc8d171752ad3953802fbd7f586587f3).
    fn find_item_at_point<P: PointMethods>(&self, pt: &P) -> Option<GBSizerItemIsOwned<false>> {
        unsafe {
            let pt = pt.as_ptr();
            GBSizerItem::option_from(ffi::wxGridBagSizer_FindItemAtPoint(self.as_ptr(), pt))
        }
    }
    /// Return the sizer item for the given grid cell, or NULL if there is no item at that position.
    ///
    /// See [C++ `wxGridBagSizer::FindItemAtPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#ab4662e411df351f1ff49252d2b49627c).
    fn find_item_at_position<G: GBPositionMethods>(
        &self,
        pos: &G,
    ) -> Option<GBSizerItemIsOwned<false>> {
        unsafe {
            let pos = pos.as_ptr();
            GBSizerItem::option_from(ffi::wxGridBagSizer_FindItemAtPosition(self.as_ptr(), pos))
        }
    }
    /// Return the sizer item that has a matching user data (it only compares pointer values) or NULL if not found.
    ///
    /// See [C++ `wxGridBagSizer::FindItemWithData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#ae6ced5c6db6560fc23a53e504929d8c1).
    fn find_item_with_data<O: ObjectMethods>(
        &self,
        user_data: Option<&O>,
    ) -> Option<GBSizerItemIsOwned<false>> {
        unsafe {
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSizerItem::option_from(ffi::wxGridBagSizer_FindItemWithData(
                self.as_ptr(),
                user_data,
            ))
        }
    }
    /// Get the size of the specified cell, including hgap and vgap.
    ///
    /// See [C++ `wxGridBagSizer::GetCellSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#a1a58df28d030ad138cd55bd0d033aba2).
    fn get_cell_size(&self, row: c_int, col: c_int) -> Size {
        unsafe { Size::from_ptr(ffi::wxGridBagSizer_GetCellSize(self.as_ptr(), row, col)) }
    }
    /// Get the size used for cells in the grid with no item.
    ///
    /// See [C++ `wxGridBagSizer::GetEmptyCellSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#abf3483f05f18c19f9befc42dc0741632).
    fn get_empty_cell_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxGridBagSizer_GetEmptyCellSize(self.as_ptr())) }
    }
    /// Get the grid position of the specified item.
    ///
    /// See [C++ `wxGridBagSizer::GetItemPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#a9aab78eb0dc76273d06c42999d998123).
    fn get_item_position_window<W: WindowMethods>(&self, window: Option<&W>) -> GBPosition {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBPosition::from_ptr(ffi::wxGridBagSizer_GetItemPosition(self.as_ptr(), window))
        }
    }
    ///
    /// See [C++ `wxGridBagSizer::GetItemPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#af5768adfe480bc01ab4489a5dbbc5832).
    fn get_item_position_sizer<S: SizerMethods>(&self, sizer: Option<&S>) -> GBPosition {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBPosition::from_ptr(ffi::wxGridBagSizer_GetItemPosition1(self.as_ptr(), sizer))
        }
    }
    ///
    /// See [C++ `wxGridBagSizer::GetItemPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#a17167c32523eeb872bf33d47a2c08687).
    fn get_item_position_sz(&self, index: usize) -> GBPosition {
        unsafe { GBPosition::from_ptr(ffi::wxGridBagSizer_GetItemPosition2(self.as_ptr(), index)) }
    }
    /// Get the row/col spanning of the specified item.
    ///
    /// See [C++ `wxGridBagSizer::GetItemSpan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#aa071287e1f6ba7bedd229b59c17d760b).
    fn get_item_span_window<W: WindowMethods>(&self, window: Option<&W>) -> GBSpan {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSpan::from_ptr(ffi::wxGridBagSizer_GetItemSpan(self.as_ptr(), window))
        }
    }
    ///
    /// See [C++ `wxGridBagSizer::GetItemSpan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#a03907c66e3deca160f93ca5d7eaf2e2b).
    fn get_item_span_sizer<S: SizerMethods>(&self, sizer: Option<&S>) -> GBSpan {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSpan::from_ptr(ffi::wxGridBagSizer_GetItemSpan1(self.as_ptr(), sizer))
        }
    }
    ///
    /// See [C++ `wxGridBagSizer::GetItemSpan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#ab123f8caed225b5d8ccbf612662c0623).
    fn get_item_span_sz(&self, index: usize) -> GBSpan {
        unsafe { GBSpan::from_ptr(ffi::wxGridBagSizer_GetItemSpan2(self.as_ptr(), index)) }
    }
    /// Set the size used for cells in the grid with no item.
    ///
    /// See [C++ `wxGridBagSizer::SetEmptyCellSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#a018984becbaa662df8e2b9ceff5387e0).
    fn set_empty_cell_size<S: SizeMethods>(&self, sz: &S) {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxGridBagSizer_SetEmptyCellSize(self.as_ptr(), sz)
        }
    }
    /// Set the grid position of the specified item.
    ///
    /// See [C++ `wxGridBagSizer::SetItemPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#ae201d64c488895851e2b1d5499150433).
    fn set_item_position_window<W: WindowMethods, G: GBPositionMethods>(
        &self,
        window: Option<&W>,
        pos: &G,
    ) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            ffi::wxGridBagSizer_SetItemPosition(self.as_ptr(), window, pos)
        }
    }
    ///
    /// See [C++ `wxGridBagSizer::SetItemPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#af0a3f5de3ff8e2a3ed0d71390af359c8).
    fn set_item_position_sizer<S: SizerMethods, G: GBPositionMethods>(
        &self,
        sizer: Option<&S>,
        pos: &G,
    ) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            ffi::wxGridBagSizer_SetItemPosition1(self.as_ptr(), sizer, pos)
        }
    }
    ///
    /// See [C++ `wxGridBagSizer::SetItemPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#a7b54ad2fcbda614085ac803bc89006b6).
    fn set_item_position_sz<G: GBPositionMethods>(&self, index: usize, pos: &G) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxGridBagSizer_SetItemPosition2(self.as_ptr(), index, pos)
        }
    }
    /// Set the row/col spanning of the specified item.
    ///
    /// See [C++ `wxGridBagSizer::SetItemSpan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#a83d09fb358698a56b69ed2219c4f600a).
    fn set_item_span_window<W: WindowMethods, G: GBSpanMethods>(
        &self,
        window: Option<&W>,
        span: &G,
    ) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let span = span.as_ptr();
            ffi::wxGridBagSizer_SetItemSpan(self.as_ptr(), window, span)
        }
    }
    ///
    /// See [C++ `wxGridBagSizer::SetItemSpan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#af5336e6b0b863f1ac0c42d893b710500).
    fn set_item_span_sizer<S: SizerMethods, G: GBSpanMethods>(
        &self,
        sizer: Option<&S>,
        span: &G,
    ) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let span = span.as_ptr();
            ffi::wxGridBagSizer_SetItemSpan1(self.as_ptr(), sizer, span)
        }
    }
    ///
    /// See [C++ `wxGridBagSizer::SetItemSpan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#a18df9884db5df835a5d53c8aa11440b8).
    fn set_item_span_sz<G: GBSpanMethods>(&self, index: usize, span: &G) -> bool {
        unsafe {
            let span = span.as_ptr();
            ffi::wxGridBagSizer_SetItemSpan2(self.as_ptr(), index, span)
        }
    }
}

// wxGridEditorCreatedEvent
/// This trait represents [C++ `wxGridEditorCreatedEvent` class](https://docs.wxwidgets.org/3.2/classwx_grid_editor_created_event.html)'s methods and inheritance.
///
/// See [`GridEditorCreatedEventIsOwned`] documentation for the class usage.
pub trait GridEditorCreatedEventMethods: CommandEventMethods {
    /// Returns the column at which the event occurred.
    ///
    /// See [C++ `wxGridEditorCreatedEvent::GetCol()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_editor_created_event.html#a139a033ef545825fca8d8e1394499b1d).
    fn get_col(&self) -> c_int {
        unsafe { ffi::wxGridEditorCreatedEvent_GetCol(self.as_ptr()) }
    }
    /// Returns the edit control.
    ///
    /// See [C++ `wxGridEditorCreatedEvent::GetControl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_editor_created_event.html#a000a9681c0f81ae06456a89bec95a467).
    fn get_control(&self) -> WeakRef<Control> {
        unsafe { WeakRef::<Control>::from(ffi::wxGridEditorCreatedEvent_GetControl(self.as_ptr())) }
    }
    /// Returns the row at which the event occurred.
    ///
    /// See [C++ `wxGridEditorCreatedEvent::GetRow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_editor_created_event.html#a5366c31b8a1d0b491a7decaaeaa4f987).
    fn get_row(&self) -> c_int {
        unsafe { ffi::wxGridEditorCreatedEvent_GetRow(self.as_ptr()) }
    }
    /// Returns the edit window.
    ///
    /// See [C++ `wxGridEditorCreatedEvent::GetWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_editor_created_event.html#a013b866bcc5931b7b7ff3bfac8381fca).
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxGridEditorCreatedEvent_GetWindow(self.as_ptr())) }
    }
    /// Sets the column at which the event occurred.
    ///
    /// See [C++ `wxGridEditorCreatedEvent::SetCol()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_editor_created_event.html#a5611b865d446357717e07067aa0ae62d).
    fn set_col(&self, col: c_int) {
        unsafe { ffi::wxGridEditorCreatedEvent_SetCol(self.as_ptr(), col) }
    }
    /// Sets the edit control.
    ///
    /// See [C++ `wxGridEditorCreatedEvent::SetControl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_editor_created_event.html#ad74a99f2cd9d7ace62cc6554a2c90eaf).
    fn set_control<C: ControlMethods>(&self, ctrl: Option<&C>) {
        unsafe {
            let ctrl = match ctrl {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGridEditorCreatedEvent_SetControl(self.as_ptr(), ctrl)
        }
    }
    /// Sets the row at which the event occurred.
    ///
    /// See [C++ `wxGridEditorCreatedEvent::SetRow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_editor_created_event.html#a7c9590a4d5edacd68071c324d426f67c).
    fn set_row(&self, row: c_int) {
        unsafe { ffi::wxGridEditorCreatedEvent_SetRow(self.as_ptr(), row) }
    }
    /// Sets the edit window.
    ///
    /// See [C++ `wxGridEditorCreatedEvent::SetWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_editor_created_event.html#ad9ead83bdc6f711805fe692aad904c80).
    fn set_window<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGridEditorCreatedEvent_SetWindow(self.as_ptr(), window)
        }
    }
}

// wxGridEvent
/// This trait represents [C++ `wxGridEvent` class](https://docs.wxwidgets.org/3.2/classwx_grid_event.html)'s methods and inheritance.
///
/// See [`GridEventIsOwned`] documentation for the class usage.
pub trait GridEventMethods: NotifyEventMethods {
    /// Returns true if the Alt key was down at the time of the event.
    ///
    /// See [C++ `wxGridEvent::AltDown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_event.html#a9b624927507dde4d919bec2fffd25aef).
    fn alt_down(&self) -> bool {
        unsafe { ffi::wxGridEvent_AltDown(self.as_ptr()) }
    }
    /// Returns true if the Control key was down at the time of the event.
    ///
    /// See [C++ `wxGridEvent::ControlDown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_event.html#a0240c3d5ed36a42f096a0898dffb57d9).
    fn control_down(&self) -> bool {
        unsafe { ffi::wxGridEvent_ControlDown(self.as_ptr()) }
    }
    /// Column at which the event occurred.
    ///
    /// See [C++ `wxGridEvent::GetCol()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_event.html#a0dad3524ee9f6486501c2aee9217b96f).
    fn get_col(&self) -> c_int {
        unsafe { ffi::wxGridEvent_GetCol(self.as_ptr()) }
    }
    /// Position in pixels at which the event occurred.
    ///
    /// See [C++ `wxGridEvent::GetPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_event.html#a210ea046e8539c44b3a9fc1fe4fa6de8).
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxGridEvent_GetPosition(self.as_ptr())) }
    }
    /// Row at which the event occurred.
    ///
    /// See [C++ `wxGridEvent::GetRow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_event.html#aa5b5903ab422edd6043100f0a225b518).
    fn get_row(&self) -> c_int {
        unsafe { ffi::wxGridEvent_GetRow(self.as_ptr()) }
    }
    /// Returns true if the Meta key was down at the time of the event.
    ///
    /// See [C++ `wxGridEvent::MetaDown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_event.html#a5598476e2f561957bc592a4f92696d66).
    fn meta_down(&self) -> bool {
        unsafe { ffi::wxGridEvent_MetaDown(self.as_ptr()) }
    }
    /// Returns true if the user is selecting grid cells, or false if deselecting.
    ///
    /// See [C++ `wxGridEvent::Selecting()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_event.html#aefd02209448d812a517136881a3592ad).
    fn selecting(&self) -> bool {
        unsafe { ffi::wxGridEvent_Selecting(self.as_ptr()) }
    }
    /// Returns true if the Shift key was down at the time of the event.
    ///
    /// See [C++ `wxGridEvent::ShiftDown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_event.html#a52cfc397443ad22af46aaf084d1a340d).
    fn shift_down(&self) -> bool {
        unsafe { ffi::wxGridEvent_ShiftDown(self.as_ptr()) }
    }
}

// wxGridRangeSelectEvent
/// This trait represents [C++ `wxGridRangeSelectEvent` class](https://docs.wxwidgets.org/3.2/classwx_grid_range_select_event.html)'s methods and inheritance.
///
/// See [`GridRangeSelectEventIsOwned`] documentation for the class usage.
pub trait GridRangeSelectEventMethods: NotifyEventMethods {
    /// Returns true if the Alt key was down at the time of the event.
    ///
    /// See [C++ `wxGridRangeSelectEvent::AltDown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_range_select_event.html#a239c8b792e3f1b499d3749ac913dd5c5).
    fn alt_down(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_AltDown(self.as_ptr()) }
    }
    /// Returns true if the Control key was down at the time of the event.
    ///
    /// See [C++ `wxGridRangeSelectEvent::ControlDown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_range_select_event.html#a068e35e7147f4247df24d28cf1828303).
    fn control_down(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_ControlDown(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetBottomRightCoords()
    /// Bottom row of the rectangular area that was (de)selected.
    ///
    /// See [C++ `wxGridRangeSelectEvent::GetBottomRow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_range_select_event.html#abb38fb9d58aa7f6f19b7ee8d6d33f35e).
    fn get_bottom_row(&self) -> c_int {
        unsafe { ffi::wxGridRangeSelectEvent_GetBottomRow(self.as_ptr()) }
    }
    /// Left column of the rectangular area that was (de)selected.
    ///
    /// See [C++ `wxGridRangeSelectEvent::GetLeftCol()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_range_select_event.html#a955c063e7f4e24608157d7aed24011ce).
    fn get_left_col(&self) -> c_int {
        unsafe { ffi::wxGridRangeSelectEvent_GetLeftCol(self.as_ptr()) }
    }
    /// Right column of the rectangular area that was (de)selected.
    ///
    /// See [C++ `wxGridRangeSelectEvent::GetRightCol()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_range_select_event.html#a0f65aa3792ed3b0acf843071464454a6).
    fn get_right_col(&self) -> c_int {
        unsafe { ffi::wxGridRangeSelectEvent_GetRightCol(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetTopLeftCoords()
    /// Top row of the rectangular area that was (de)selected.
    ///
    /// See [C++ `wxGridRangeSelectEvent::GetTopRow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_range_select_event.html#ab0ea5b8cff7ba2a3d3a3146e33d879b0).
    fn get_top_row(&self) -> c_int {
        unsafe { ffi::wxGridRangeSelectEvent_GetTopRow(self.as_ptr()) }
    }
    /// Returns true if the Meta key was down at the time of the event.
    ///
    /// See [C++ `wxGridRangeSelectEvent::MetaDown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_range_select_event.html#a6a19580dd8ee46765dbcfae0be74f116).
    fn meta_down(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_MetaDown(self.as_ptr()) }
    }
    /// Returns true if the area was selected, false otherwise.
    ///
    /// See [C++ `wxGridRangeSelectEvent::Selecting()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_range_select_event.html#aaf6c6e6a9bf8b99f7cd649047c2b67c1).
    fn selecting(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_Selecting(self.as_ptr()) }
    }
    /// Returns true if the Shift key was down at the time of the event.
    ///
    /// See [C++ `wxGridRangeSelectEvent::ShiftDown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_range_select_event.html#ad0d5ebb89a88f950ce98c23a1b9ecc9f).
    fn shift_down(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_ShiftDown(self.as_ptr()) }
    }
}

// wxGridSizeEvent
/// This trait represents [C++ `wxGridSizeEvent` class](https://docs.wxwidgets.org/3.2/classwx_grid_size_event.html)'s methods and inheritance.
///
/// See [`GridSizeEventIsOwned`] documentation for the class usage.
pub trait GridSizeEventMethods: NotifyEventMethods {
    /// Returns true if the Alt key was down at the time of the event.
    ///
    /// See [C++ `wxGridSizeEvent::AltDown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_size_event.html#ad30cb54ba2dd9af68d6467447a3ded08).
    fn alt_down(&self) -> bool {
        unsafe { ffi::wxGridSizeEvent_AltDown(self.as_ptr()) }
    }
    /// Returns true if the Control key was down at the time of the event.
    ///
    /// See [C++ `wxGridSizeEvent::ControlDown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_size_event.html#ab25f0bf25c5ac498e53d56fcf5f1b46d).
    fn control_down(&self) -> bool {
        unsafe { ffi::wxGridSizeEvent_ControlDown(self.as_ptr()) }
    }
    /// Position in pixels at which the event occurred.
    ///
    /// See [C++ `wxGridSizeEvent::GetPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_size_event.html#ad853d8f733a260397a016982cd11b175).
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxGridSizeEvent_GetPosition(self.as_ptr())) }
    }
    /// Row or column at that was resized.
    ///
    /// See [C++ `wxGridSizeEvent::GetRowOrCol()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_size_event.html#a6a7434599611228c22c4a6136c503427).
    fn get_row_or_col(&self) -> c_int {
        unsafe { ffi::wxGridSizeEvent_GetRowOrCol(self.as_ptr()) }
    }
    /// Returns true if the Meta key was down at the time of the event.
    ///
    /// See [C++ `wxGridSizeEvent::MetaDown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_size_event.html#a658a6fbfd7e16af4f65c70588d9e3ec7).
    fn meta_down(&self) -> bool {
        unsafe { ffi::wxGridSizeEvent_MetaDown(self.as_ptr()) }
    }
    /// Returns true if the Shift key was down at the time of the event.
    ///
    /// See [C++ `wxGridSizeEvent::ShiftDown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_size_event.html#a249b50a1ce0f3f415b716251a034d309).
    fn shift_down(&self) -> bool {
        unsafe { ffi::wxGridSizeEvent_ShiftDown(self.as_ptr()) }
    }
}

// wxGridSizer
/// This trait represents [C++ `wxGridSizer` class](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html)'s methods and inheritance.
///
/// See [`GridSizerIsOwned`] documentation for the class usage.
pub trait GridSizerMethods: SizerMethods {
    /// Returns the number of columns that has been specified for the sizer.
    ///
    /// See [C++ `wxGridSizer::GetCols()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#aaf65c2655dcdc43ec0e7d10954927371).
    fn get_cols(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetCols(self.as_ptr()) }
    }
    /// Returns the number of rows that has been specified for the sizer.
    ///
    /// See [C++ `wxGridSizer::GetRows()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#a3198c12867f5e8c0382eef888f6b88be).
    fn get_rows(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetRows(self.as_ptr()) }
    }
    /// Returns the number of columns currently used by the sizer.
    ///
    /// See [C++ `wxGridSizer::GetEffectiveColsCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#a3ac849f1d1056999b5bfc6dfea3df5b0).
    fn get_effective_cols_count(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetEffectiveColsCount(self.as_ptr()) }
    }
    /// Returns the number of rows currently used by the sizer.
    ///
    /// See [C++ `wxGridSizer::GetEffectiveRowsCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#a619c568c293bf5bd54182f968531b9ab).
    fn get_effective_rows_count(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetEffectiveRowsCount(self.as_ptr()) }
    }
    /// Returns the horizontal gap (in pixels) between cells in the sizer.
    ///
    /// See [C++ `wxGridSizer::GetHGap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#aa857169111c3ac1769c9e0f972ec8a69).
    fn get_h_gap(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetHGap(self.as_ptr()) }
    }
    /// Returns the vertical gap (in pixels) between the cells in the sizer.
    ///
    /// See [C++ `wxGridSizer::GetVGap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#aa3f266b32b716286fd36ec2ac8644c1b).
    fn get_v_gap(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetVGap(self.as_ptr()) }
    }
    /// Sets the number of columns in the sizer.
    ///
    /// See [C++ `wxGridSizer::SetCols()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#aaa0e5a8e167de18b0fd3066febed5cad).
    fn set_cols(&self, cols: c_int) {
        unsafe { ffi::wxGridSizer_SetCols(self.as_ptr(), cols) }
    }
    /// Sets the horizontal gap (in pixels) between cells in the sizer.
    ///
    /// See [C++ `wxGridSizer::SetHGap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#acf9b3399f7b6044f1d75446015f28747).
    fn set_h_gap(&self, gap: c_int) {
        unsafe { ffi::wxGridSizer_SetHGap(self.as_ptr(), gap) }
    }
    /// Sets the number of rows in the sizer.
    ///
    /// See [C++ `wxGridSizer::SetRows()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#a66b7cccb173e76a06a70256ba2cbaf33).
    fn set_rows(&self, rows: c_int) {
        unsafe { ffi::wxGridSizer_SetRows(self.as_ptr(), rows) }
    }
    /// Sets the vertical gap (in pixels) between the cells in the sizer.
    ///
    /// See [C++ `wxGridSizer::SetVGap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#a89c28d0796cead886b0e2218178125f0).
    fn set_v_gap(&self, gap: c_int) {
        unsafe { ffi::wxGridSizer_SetVGap(self.as_ptr(), gap) }
    }
}

// wxGridTableBase
/// This trait represents [C++ `wxGridTableBase` class](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html)'s methods and inheritance.
///
/// See [`GridTableBaseIsOwned`] documentation for the class usage.
pub trait GridTableBaseMethods: ObjectMethods {
    /// May be overridden to implement testing for empty cells.
    ///
    /// See [C++ `wxGridTableBase::IsEmptyCell()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a53b7f70b9eff26b9d699993e3a71968b).
    fn is_empty_cell(&self, row: c_int, col: c_int) -> bool {
        unsafe { ffi::wxGridTableBase_IsEmptyCell(self.as_ptr(), row, col) }
    }
    /// Same as IsEmptyCell() but taking wxGridCellCoords.
    ///
    /// See [C++ `wxGridTableBase::IsEmpty()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a819caeb23236eb3f70832cacab2a9836).
    fn is_empty(&self, coords: *const c_void) -> bool {
        unsafe { ffi::wxGridTableBase_IsEmpty(self.as_ptr(), coords) }
    }
    /// Must be overridden to implement accessing the table values as text.
    ///
    /// See [C++ `wxGridTableBase::GetValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a4233348a081a46dabadb6b2dd2cfb972).
    fn get_value(&self, row: c_int, col: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxGridTableBase_GetValue(self.as_ptr(), row, col)).into() }
    }
    /// Must be overridden to implement setting the table values as text.
    ///
    /// See [C++ `wxGridTableBase::SetValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a81959e0a329f006de970c5cc82d99ba2).
    fn set_value(&self, row: c_int, col: c_int, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxGridTableBase_SetValue(self.as_ptr(), row, col, value)
        }
    }
    /// Returns the type of the value in the given cell.
    ///
    /// See [C++ `wxGridTableBase::GetTypeName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a2192fb69ab1daf9684f23562d1fea727).
    fn get_type_name(&self, row: c_int, col: c_int) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxGridTableBase_GetTypeName(self.as_ptr(), row, col)).into()
        }
    }
    /// Returns true if the value of the given cell can be accessed as if it were of the specified type.
    ///
    /// See [C++ `wxGridTableBase::CanGetValueAs()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#ab35a34c87742cf546751d93b5f6cdbdb).
    fn can_get_value_as(&self, row: c_int, col: c_int, type_name: &str) -> bool {
        unsafe {
            let type_name = WxString::from(type_name);
            let type_name = type_name.as_ptr();
            ffi::wxGridTableBase_CanGetValueAs(self.as_ptr(), row, col, type_name)
        }
    }
    /// Returns true if the value of the given cell can be set as if it were of the specified type.
    ///
    /// See [C++ `wxGridTableBase::CanSetValueAs()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#ab17971b84ec8c875cf758310a975d96e).
    fn can_set_value_as(&self, row: c_int, col: c_int, type_name: &str) -> bool {
        unsafe {
            let type_name = WxString::from(type_name);
            let type_name = type_name.as_ptr();
            ffi::wxGridTableBase_CanSetValueAs(self.as_ptr(), row, col, type_name)
        }
    }
    /// Returns the value of the given cell as a long.
    ///
    /// See [C++ `wxGridTableBase::GetValueAsLong()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#ac4abb42709ba4ad5fb3e65747b5525f2).
    fn get_value_as_long(&self, row: c_int, col: c_int) -> c_long {
        unsafe { ffi::wxGridTableBase_GetValueAsLong(self.as_ptr(), row, col) }
    }
    /// Returns the value of the given cell as a double.
    ///
    /// See [C++ `wxGridTableBase::GetValueAsDouble()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#ab761072281ec4e7f800f44c9577040bc).
    fn get_value_as_double(&self, row: c_int, col: c_int) -> c_double {
        unsafe { ffi::wxGridTableBase_GetValueAsDouble(self.as_ptr(), row, col) }
    }
    /// Returns the value of the given cell as a boolean.
    ///
    /// See [C++ `wxGridTableBase::GetValueAsBool()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a133772ec139879eef58d0440f22e4476).
    fn get_value_as_bool(&self, row: c_int, col: c_int) -> bool {
        unsafe { ffi::wxGridTableBase_GetValueAsBool(self.as_ptr(), row, col) }
    }
    /// Returns the value of the given cell as a user-defined type.
    ///
    /// See [C++ `wxGridTableBase::GetValueAsCustom()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a96ae77c99a53b967d365d5c6902d70ff).
    fn get_value_as_custom(&self, row: c_int, col: c_int, type_name: &str) -> *mut c_void {
        unsafe {
            let type_name = WxString::from(type_name);
            let type_name = type_name.as_ptr();
            ffi::wxGridTableBase_GetValueAsCustom(self.as_ptr(), row, col, type_name)
        }
    }
    /// Sets the value of the given cell as a long.
    ///
    /// See [C++ `wxGridTableBase::SetValueAsLong()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#ae035a347dd02d16182e37e866b5c2632).
    fn set_value_as_long(&self, row: c_int, col: c_int, value: c_long) {
        unsafe { ffi::wxGridTableBase_SetValueAsLong(self.as_ptr(), row, col, value) }
    }
    /// Sets the value of the given cell as a double.
    ///
    /// See [C++ `wxGridTableBase::SetValueAsDouble()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a4cf3f5ba2c6338cf2cb944960c15bdb1).
    fn set_value_as_double(&self, row: c_int, col: c_int, value: c_double) {
        unsafe { ffi::wxGridTableBase_SetValueAsDouble(self.as_ptr(), row, col, value) }
    }
    /// Sets the value of the given cell as a boolean.
    ///
    /// See [C++ `wxGridTableBase::SetValueAsBool()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a1948b5e152aac985892e568655561feb).
    fn set_value_as_bool(&self, row: c_int, col: c_int, value: bool) {
        unsafe { ffi::wxGridTableBase_SetValueAsBool(self.as_ptr(), row, col, value) }
    }
    /// Sets the value of the given cell as a user-defined type.
    ///
    /// See [C++ `wxGridTableBase::SetValueAsCustom()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#ad9be80bb5bf75a8f80e26e40208be583).
    fn set_value_as_custom(&self, row: c_int, col: c_int, type_name: &str, value: *mut c_void) {
        unsafe {
            let type_name = WxString::from(type_name);
            let type_name = type_name.as_ptr();
            ffi::wxGridTableBase_SetValueAsCustom(self.as_ptr(), row, col, type_name, value)
        }
    }
    /// Called by the grid when the table is associated with it.
    ///
    /// See [C++ `wxGridTableBase::SetView()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a7dfac01d7b0655d3b41d350e2f322f60).
    fn set_view(&self, grid: *mut c_void) {
        unsafe { ffi::wxGridTableBase_SetView(self.as_ptr(), grid) }
    }
    /// Returns the last grid passed to SetView().
    ///
    /// See [C++ `wxGridTableBase::GetView()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#ad0a03a4fbcbe745e2bd8b88904a74605).
    fn get_view(&self) -> *mut c_void {
        unsafe { ffi::wxGridTableBase_GetView(self.as_ptr()) }
    }
    /// Clear the table contents.
    ///
    /// See [C++ `wxGridTableBase::Clear()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#acc1ed3ef5d026594cb09d957f34761f5).
    fn clear(&self) {
        unsafe { ffi::wxGridTableBase_Clear(self.as_ptr()) }
    }
    /// Insert additional rows into the table.
    ///
    /// See [C++ `wxGridTableBase::InsertRows()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a649fad50aeeb8dea442ae1f9122e4e6a).
    fn insert_rows(&self, pos: usize, num_rows: usize) -> bool {
        unsafe { ffi::wxGridTableBase_InsertRows(self.as_ptr(), pos, num_rows) }
    }
    /// Append additional rows at the end of the table.
    ///
    /// See [C++ `wxGridTableBase::AppendRows()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a4b85abbed0e689820aa1928eb3a303cd).
    fn append_rows(&self, num_rows: usize) -> bool {
        unsafe { ffi::wxGridTableBase_AppendRows(self.as_ptr(), num_rows) }
    }
    /// Delete rows from the table.
    ///
    /// See [C++ `wxGridTableBase::DeleteRows()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a038567ce937e90cc3f06c5d7c4df7c16).
    fn delete_rows(&self, pos: usize, num_rows: usize) -> bool {
        unsafe { ffi::wxGridTableBase_DeleteRows(self.as_ptr(), pos, num_rows) }
    }
    /// Exactly the same as InsertRows() but for columns.
    ///
    /// See [C++ `wxGridTableBase::InsertCols()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a361c7064a0199afc01678318ce8b712d).
    fn insert_cols(&self, pos: usize, num_cols: usize) -> bool {
        unsafe { ffi::wxGridTableBase_InsertCols(self.as_ptr(), pos, num_cols) }
    }
    /// Exactly the same as AppendRows() but for columns.
    ///
    /// See [C++ `wxGridTableBase::AppendCols()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a2f70e66794796670b6e5367e200d5168).
    fn append_cols(&self, num_cols: usize) -> bool {
        unsafe { ffi::wxGridTableBase_AppendCols(self.as_ptr(), num_cols) }
    }
    /// Exactly the same as DeleteRows() but for columns.
    ///
    /// See [C++ `wxGridTableBase::DeleteCols()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a4f96bd344c49875495c4f20ef707ca4b).
    fn delete_cols(&self, pos: usize, num_cols: usize) -> bool {
        unsafe { ffi::wxGridTableBase_DeleteCols(self.as_ptr(), pos, num_cols) }
    }
    /// Return the label of the specified row.
    ///
    /// See [C++ `wxGridTableBase::GetRowLabelValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#ad2a20c7f8ac0cbca8665639897b0aa33).
    fn get_row_label_value(&self, row: c_int) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxGridTableBase_GetRowLabelValue(self.as_ptr(), row)).into()
        }
    }
    /// Return the label of the specified column.
    ///
    /// See [C++ `wxGridTableBase::GetColLabelValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#ac7db44c525e0791b29005ee4fa48e729).
    fn get_col_label_value(&self, col: c_int) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxGridTableBase_GetColLabelValue(self.as_ptr(), col)).into()
        }
    }
    /// Return the label of the grid's corner.
    ///
    /// See [C++ `wxGridTableBase::GetCornerLabelValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#aff972cc0dc54cc9033bd085cb7654e5c).
    fn get_corner_label_value(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxGridTableBase_GetCornerLabelValue(self.as_ptr())).into()
        }
    }
    /// Set the given label for the specified row.
    ///
    /// See [C++ `wxGridTableBase::SetRowLabelValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a8205840a49da8ab2147fa8081afb2de6).
    fn set_row_label_value(&self, row: c_int, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxGridTableBase_SetRowLabelValue(self.as_ptr(), row, label)
        }
    }
    /// Exactly the same as SetRowLabelValue() but for columns.
    ///
    /// See [C++ `wxGridTableBase::SetColLabelValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a2c228818518de127b34c13655153021e).
    fn set_col_label_value(&self, col: c_int, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxGridTableBase_SetColLabelValue(self.as_ptr(), col, label)
        }
    }
    // BLOCKED: fn SetCornerLabelValue()
    /// Associate this attributes provider with the table.
    ///
    /// See [C++ `wxGridTableBase::SetAttrProvider()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#ad0248937170cf1f0280694525ee5f994).
    fn set_attr_provider(&self, attr_provider: *mut c_void) {
        unsafe { ffi::wxGridTableBase_SetAttrProvider(self.as_ptr(), attr_provider) }
    }
    /// Returns the attribute provider currently being used.
    ///
    /// See [C++ `wxGridTableBase::GetAttrProvider()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a991adb9bb3bbd40e53e88a6a8ef4feda).
    fn get_attr_provider(&self) -> *mut c_void {
        unsafe { ffi::wxGridTableBase_GetAttrProvider(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetAttr()
    // NOT_SUPPORTED: fn GetAttrPtr()
    /// Set attribute of the specified cell.
    ///
    /// See [C++ `wxGridTableBase::SetAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a12277f75a635fd35fcaefd0d56cad0ab).
    fn set_attr(&self, attr: *mut c_void, row: c_int, col: c_int) {
        unsafe { ffi::wxGridTableBase_SetAttr(self.as_ptr(), attr, row, col) }
    }
    /// Set attribute of the specified row.
    ///
    /// See [C++ `wxGridTableBase::SetRowAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a8f1dc4a35497eeec27104943f0d7be11).
    fn set_row_attr(&self, attr: *mut c_void, row: c_int) {
        unsafe { ffi::wxGridTableBase_SetRowAttr(self.as_ptr(), attr, row) }
    }
    /// Set attribute of the specified column.
    ///
    /// See [C++ `wxGridTableBase::SetColAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#ac08a10473e538c1f3859ad59936eb870).
    fn set_col_attr(&self, attr: *mut c_void, col: c_int) {
        unsafe { ffi::wxGridTableBase_SetColAttr(self.as_ptr(), attr, col) }
    }
    /// Returns true if this table supports attributes or false otherwise.
    ///
    /// See [C++ `wxGridTableBase::CanHaveAttributes()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#aa5d507ed6d82b7a7878ad65f5e75d4f0).
    fn can_have_attributes(&self) -> bool {
        unsafe { ffi::wxGridTableBase_CanHaveAttributes(self.as_ptr()) }
    }
    /// Override to return true if the same attribute can be used for measuring all cells in the given column.
    ///
    /// See [C++ `wxGridTableBase::CanMeasureColUsingSameAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a1028486fa8f15afcb014de89aaff82b9).
    fn can_measure_col_using_same_attr(&self, col: c_int) -> bool {
        unsafe { ffi::wxGridTableBase_CanMeasureColUsingSameAttr(self.as_ptr(), col) }
    }
    // DTOR: fn ~wxGridTableBase()
    /// Must be overridden to return the number of rows in the table.
    ///
    /// See [C++ `wxGridTableBase::GetNumberRows()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#aefe57964fd86e7a9ab2253d070908b48).
    fn get_number_rows(&self) -> c_int {
        unsafe { ffi::wxGridTableBase_GetNumberRows(self.as_ptr()) }
    }
    /// Must be overridden to return the number of columns in the table.
    ///
    /// See [C++ `wxGridTableBase::GetNumberCols()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#ad290f56b43056a66fa08531410c658d7).
    fn get_number_cols(&self) -> c_int {
        unsafe { ffi::wxGridTableBase_GetNumberCols(self.as_ptr()) }
    }
    /// Return the number of rows in the table.
    ///
    /// See [C++ `wxGridTableBase::GetRowsCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#a06c0945d3f271c6954b7f92c3bac54b4).
    fn get_rows_count(&self) -> c_int {
        unsafe { ffi::wxGridTableBase_GetRowsCount(self.as_ptr()) }
    }
    /// Return the number of columns in the table.
    ///
    /// See [C++ `wxGridTableBase::GetColsCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html#ab07c51b35ebce4e11fb5b93b0323e13c).
    fn get_cols_count(&self) -> c_int {
        unsafe { ffi::wxGridTableBase_GetColsCount(self.as_ptr()) }
    }
}

// wxGridUpdateLocker
/// This trait represents [C++ `wxGridUpdateLocker` class](https://docs.wxwidgets.org/3.2/classwx_grid_update_locker.html)'s methods and inheritance.
///
/// See [`GridUpdateLockerIsOwned`] documentation for the class usage.
pub trait GridUpdateLockerMethods: WxRustMethods {
    // DTOR: fn ~wxGridUpdateLocker()
    /// This method can be called if the object had been constructed using the default constructor.
    ///
    /// See [C++ `wxGridUpdateLocker::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_update_locker.html#a43089db17a5ef1a315c6700e20cf71ac).
    fn create(&self, grid: *mut c_void) {
        unsafe { ffi::wxGridUpdateLocker_Create(self.as_ptr(), grid) }
    }
}
