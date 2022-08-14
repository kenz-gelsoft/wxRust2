use super::*;

// wxDataObject
pub trait DataObjectMethods: WxRustMethods {
    // DTOR: fn ~wxDataObject()
    // NOT_SUPPORTED: fn GetAllFormats()
    fn get_data_here(&self, format: *const c_void, buf: *mut c_void) -> bool {
        unsafe { ffi::wxDataObject_GetDataHere(self.as_ptr(), format, buf) }
    }
    fn get_data_size(&self, format: *const c_void) -> usize {
        unsafe { ffi::wxDataObject_GetDataSize(self.as_ptr(), format) }
    }
    // NOT_SUPPORTED: fn GetFormatCount()
    // NOT_SUPPORTED: fn GetPreferredFormat()
    fn set_data(&self, format: *const c_void, len: usize, buf: *const c_void) -> bool {
        unsafe { ffi::wxDataObject_SetData(self.as_ptr(), format, len, buf) }
    }
    // NOT_SUPPORTED: fn IsSupported()
}

// wxDataObjectSimple
pub trait DataObjectSimpleMethods: DataObjectMethods {
    fn get_data_here_void(&self, buf: *mut c_void) -> bool {
        unsafe { ffi::wxDataObjectSimple_GetDataHere(self.as_ptr(), buf) }
    }
    fn get_data_size(&self) -> usize {
        unsafe { ffi::wxDataObjectSimple_GetDataSize(self.as_ptr()) }
    }
    // BLOCKED: fn GetFormat()
    fn set_data_sz(&self, len: usize, buf: *const c_void) -> bool {
        unsafe { ffi::wxDataObjectSimple_SetData(self.as_ptr(), len, buf) }
    }
    fn set_format(&self, format: *const c_void) {
        unsafe { ffi::wxDataObjectSimple_SetFormat(self.as_ptr(), format) }
    }
}

// wxDateEvent
pub trait DateEventMethods: CommandEventMethods {
    fn get_date(&self) -> DateTimeIsOwned<false> {
        unsafe { DateTimeIsOwned::from_ptr(ffi::wxDateEvent_GetDate(self.as_ptr())) }
    }
    fn set_date<D: DateTimeMethods>(&self, date: &D) {
        unsafe {
            let date = date.as_ptr();
            ffi::wxDateEvent_SetDate(self.as_ptr(), date)
        }
    }
}

// wxDatePickerCtrl
pub trait DatePickerCtrlMethods: ControlMethods {
    fn create_datetime<
        W: WindowMethods,
        D: DateTimeMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        dt: &D,
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
            let dt = dt.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxDatePickerCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                dt,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_range<D: DateTimeMethods, D2: DateTimeMethods>(
        &self,
        dt1: Option<&D>,
        dt2: Option<&D2>,
    ) -> bool {
        unsafe {
            let dt1 = match dt1 {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt2 = match dt2 {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDatePickerCtrl_GetRange(self.as_ptr(), dt1, dt2)
        }
    }
    fn get_value(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDatePickerCtrl_GetValue(self.as_ptr())) }
    }
    fn set_null_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDatePickerCtrl_SetNullText(self.as_ptr(), text)
        }
    }
    fn set_range<D: DateTimeMethods, D2: DateTimeMethods>(&self, dt1: &D, dt2: &D2) {
        unsafe {
            let dt1 = dt1.as_ptr();
            let dt2 = dt2.as_ptr();
            ffi::wxDatePickerCtrl_SetRange(self.as_ptr(), dt1, dt2)
        }
    }
    fn set_value<D: DateTimeMethods>(&self, dt: &D) {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxDatePickerCtrl_SetValue(self.as_ptr(), dt)
        }
    }
}

// wxDialog
pub trait DialogMethods: TopLevelWindowMethods {
    // DTOR: fn ~wxDialog()
    fn add_main_button_id(&self, id: c_int) {
        unsafe { ffi::wxDialog_AddMainButtonId(self.as_ptr(), id) }
    }
    fn can_do_layout_adaptation(&self) -> bool {
        unsafe { ffi::wxDialog_CanDoLayoutAdaptation(self.as_ptr()) }
    }
    fn create_button_sizer(&self, flags: c_long) -> Option<SizerIsOwned<false>> {
        unsafe { Sizer::option_from(ffi::wxDialog_CreateButtonSizer(self.as_ptr(), flags)) }
    }
    fn create_separated_button_sizer(&self, flags: c_long) -> Option<SizerIsOwned<false>> {
        unsafe {
            Sizer::option_from(ffi::wxDialog_CreateSeparatedButtonSizer(
                self.as_ptr(),
                flags,
            ))
        }
    }
    fn create_separated_sizer<S: SizerMethods>(
        &self,
        sizer: Option<&S>,
    ) -> Option<SizerIsOwned<false>> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Sizer::option_from(ffi::wxDialog_CreateSeparatedSizer(self.as_ptr(), sizer))
        }
    }
    fn create_std_dialog_button_sizer(&self, flags: c_long) -> *mut c_void {
        unsafe { ffi::wxDialog_CreateStdDialogButtonSizer(self.as_ptr(), flags) }
    }
    fn create_text_sizer(&self, message: &str, width_max: c_int) -> Option<SizerIsOwned<false>> {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            Sizer::option_from(ffi::wxDialog_CreateTextSizer(
                self.as_ptr(),
                message,
                width_max,
            ))
        }
    }
    fn do_layout_adaptation(&self) -> bool {
        unsafe { ffi::wxDialog_DoLayoutAdaptation(self.as_ptr()) }
    }
    fn end_modal(&self, ret_code: c_int) {
        unsafe { ffi::wxDialog_EndModal(self.as_ptr(), ret_code) }
    }
    fn get_affirmative_id(&self) -> c_int {
        unsafe { ffi::wxDialog_GetAffirmativeId(self.as_ptr()) }
    }
    fn get_content_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxDialog_GetContentWindow(self.as_ptr())) }
    }
    fn get_escape_id(&self) -> c_int {
        unsafe { ffi::wxDialog_GetEscapeId(self.as_ptr()) }
    }
    fn get_layout_adaptation_done(&self) -> bool {
        unsafe { ffi::wxDialog_GetLayoutAdaptationDone(self.as_ptr()) }
    }
    fn get_layout_adaptation_level(&self) -> c_int {
        unsafe { ffi::wxDialog_GetLayoutAdaptationLevel(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetLayoutAdaptationMode()
    fn get_main_button_ids(&self) -> ArrayIntIsOwned<false> {
        unsafe { ArrayIntIsOwned::from_ptr(ffi::wxDialog_GetMainButtonIds(self.as_ptr())) }
    }
    fn get_return_code(&self) -> c_int {
        unsafe { ffi::wxDialog_GetReturnCode(self.as_ptr()) }
    }
    fn get_tool_bar(&self) -> WeakRef<ToolBar> {
        unsafe { WeakRef::<ToolBar>::from(ffi::wxDialog_GetToolBar(self.as_ptr())) }
    }
    fn is_main_button_id(&self, id: c_int) -> bool {
        unsafe { ffi::wxDialog_IsMainButtonId(self.as_ptr(), id) }
    }
    fn is_modal(&self) -> bool {
        unsafe { ffi::wxDialog_IsModal(self.as_ptr()) }
    }
    fn set_affirmative_id(&self, id: c_int) {
        unsafe { ffi::wxDialog_SetAffirmativeId(self.as_ptr(), id) }
    }
    fn set_escape_id(&self, id: c_int) {
        unsafe { ffi::wxDialog_SetEscapeId(self.as_ptr(), id) }
    }
    fn set_layout_adaptation_done(&self, done: bool) {
        unsafe { ffi::wxDialog_SetLayoutAdaptationDone(self.as_ptr(), done) }
    }
    fn set_layout_adaptation_level(&self, level: c_int) {
        unsafe { ffi::wxDialog_SetLayoutAdaptationLevel(self.as_ptr(), level) }
    }
    // NOT_SUPPORTED: fn SetLayoutAdaptationMode()
    fn set_return_code(&self, ret_code: c_int) {
        unsafe { ffi::wxDialog_SetReturnCode(self.as_ptr(), ret_code) }
    }
    fn show_modal(&self) -> c_int {
        unsafe { ffi::wxDialog_ShowModal(self.as_ptr()) }
    }
    fn show_window_modal(&self) {
        unsafe { ffi::wxDialog_ShowWindowModal(self.as_ptr()) }
    }
    // BLOCKED: fn ShowWindowModalThenDo()
    fn enable_layout_adaptation(enable: bool) {
        unsafe { ffi::wxDialog_EnableLayoutAdaptation(enable) }
    }
    fn get_layout_adapter() -> *mut c_void {
        unsafe { ffi::wxDialog_GetLayoutAdapter() }
    }
    fn is_layout_adaptation_enabled() -> bool {
        unsafe { ffi::wxDialog_IsLayoutAdaptationEnabled() }
    }
    fn set_layout_adapter(adapter: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxDialog_SetLayoutAdapter(adapter) }
    }
}

// wxDirPickerCtrl
pub trait DirPickerCtrlMethods: PickerBaseMethods {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        path: &str,
        message: &str,
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
            let path = WxString::from(path);
            let path = path.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxDirPickerCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                path,
                message,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_dir_name(&self) -> FileName {
        unsafe { FileName::from_ptr(ffi::wxDirPickerCtrl_GetDirName(self.as_ptr())) }
    }
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDirPickerCtrl_GetPath(self.as_ptr())).into() }
    }
    fn set_dir_name<F: FileNameMethods>(&self, dirname: &F) {
        unsafe {
            let dirname = dirname.as_ptr();
            ffi::wxDirPickerCtrl_SetDirName(self.as_ptr(), dirname)
        }
    }
    fn set_initial_directory(&self, dir: &str) {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxDirPickerCtrl_SetInitialDirectory(self.as_ptr(), dir)
        }
    }
    fn set_path(&self, dirname: &str) {
        unsafe {
            let dirname = WxString::from(dirname);
            let dirname = dirname.as_ptr();
            ffi::wxDirPickerCtrl_SetPath(self.as_ptr(), dirname)
        }
    }
}
