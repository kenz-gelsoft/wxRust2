use super::*;

// wxGDIObject
pub trait GDIObjectMethods: ObjectMethods {}

// wxGauge
pub trait GaugeMethods: ControlMethods {
    // DTOR: fn ~wxGauge()
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
    fn get_range(&self) -> c_int {
        unsafe { ffi::wxGauge_GetRange(self.as_ptr()) }
    }
    fn get_value(&self) -> c_int {
        unsafe { ffi::wxGauge_GetValue(self.as_ptr()) }
    }
    fn is_vertical(&self) -> bool {
        unsafe { ffi::wxGauge_IsVertical(self.as_ptr()) }
    }
    fn pulse(&self) {
        unsafe { ffi::wxGauge_Pulse(self.as_ptr()) }
    }
    fn set_range(&self, range: c_int) {
        unsafe { ffi::wxGauge_SetRange(self.as_ptr(), range) }
    }
    fn set_value(&self, pos: c_int) {
        unsafe { ffi::wxGauge_SetValue(self.as_ptr(), pos) }
    }
}

// wxGenericDirCtrl
pub trait GenericDirCtrlMethods: ControlMethods {
    // DTOR: fn ~wxGenericDirCtrl()
    fn collapse_path(&self, path: &str) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_CollapsePath(self.as_ptr(), path)
        }
    }
    fn collapse_tree(&self) {
        unsafe { ffi::wxGenericDirCtrl_CollapseTree(self.as_ptr()) }
    }
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
    fn expand_path(&self, path: &str) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_ExpandPath(self.as_ptr(), path)
        }
    }
    fn get_default_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetDefaultPath(self.as_ptr())).into() }
    }
    fn get_file_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetFilePath(self.as_ptr())).into() }
    }
    fn get_file_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxGenericDirCtrl_GetFilePaths(self.as_ptr(), paths)
        }
    }
    fn get_filter(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetFilter(self.as_ptr())).into() }
    }
    fn get_filter_index(&self) -> c_int {
        unsafe { ffi::wxGenericDirCtrl_GetFilterIndex(self.as_ptr()) }
    }
    fn get_filter_list_ctrl(&self) -> *mut c_void {
        unsafe { ffi::wxGenericDirCtrl_GetFilterListCtrl(self.as_ptr()) }
    }
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetPath(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetPath1()
    fn get_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxGenericDirCtrl_GetPaths(self.as_ptr(), paths)
        }
    }
    // NOT_SUPPORTED: fn GetRootId()
    fn get_tree_ctrl(&self) -> *mut c_void {
        unsafe { ffi::wxGenericDirCtrl_GetTreeCtrl(self.as_ptr()) }
    }
    fn init(&self) {
        unsafe { ffi::wxGenericDirCtrl_Init(self.as_ptr()) }
    }
    fn re_create_tree(&self) {
        unsafe { ffi::wxGenericDirCtrl_ReCreateTree(self.as_ptr()) }
    }
    fn set_default_path(&self, path: &str) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_SetDefaultPath(self.as_ptr(), path)
        }
    }
    fn set_filter(&self, filter: &str) {
        unsafe {
            let filter = WxString::from(filter);
            let filter = filter.as_ptr();
            ffi::wxGenericDirCtrl_SetFilter(self.as_ptr(), filter)
        }
    }
    fn set_filter_index(&self, n: c_int) {
        unsafe { ffi::wxGenericDirCtrl_SetFilterIndex(self.as_ptr(), n) }
    }
    fn set_path(&self, path: &str) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_SetPath(self.as_ptr(), path)
        }
    }
    fn show_hidden(&self, show: bool) {
        unsafe { ffi::wxGenericDirCtrl_ShowHidden(self.as_ptr(), show) }
    }
    fn select_path(&self, path: &str, select: bool) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_SelectPath(self.as_ptr(), path, select)
        }
    }
    fn select_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxGenericDirCtrl_SelectPaths(self.as_ptr(), paths)
        }
    }
    fn unselect_all(&self) {
        unsafe { ffi::wxGenericDirCtrl_UnselectAll(self.as_ptr()) }
    }
}

// wxGridSizer
pub trait GridSizerMethods: SizerMethods {
    fn get_cols(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetCols(self.as_ptr()) }
    }
    fn get_rows(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetRows(self.as_ptr()) }
    }
    fn get_effective_cols_count(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetEffectiveColsCount(self.as_ptr()) }
    }
    fn get_effective_rows_count(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetEffectiveRowsCount(self.as_ptr()) }
    }
    fn get_h_gap(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetHGap(self.as_ptr()) }
    }
    fn get_v_gap(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetVGap(self.as_ptr()) }
    }
    fn set_cols(&self, cols: c_int) {
        unsafe { ffi::wxGridSizer_SetCols(self.as_ptr(), cols) }
    }
    fn set_h_gap(&self, gap: c_int) {
        unsafe { ffi::wxGridSizer_SetHGap(self.as_ptr(), gap) }
    }
    fn set_rows(&self, rows: c_int) {
        unsafe { ffi::wxGridSizer_SetRows(self.as_ptr(), rows) }
    }
    fn set_v_gap(&self, gap: c_int) {
        unsafe { ffi::wxGridSizer_SetVGap(self.as_ptr(), gap) }
    }
}
