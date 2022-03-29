#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};
use std::ptr;

use crate::wx_class;

mod ffi {
    use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};
    pub use crate::ffi::*;
    extern "C" {

        // wxObject
        pub fn wxObject_new() -> *mut c_void;
        pub fn wxObject_new1(other: *const c_void) -> *mut c_void;
        // DTOR: pub fn wxObject_~wxObject(self_: *mut c_void);
        pub fn wxObject_GetClassInfo(self_: *const c_void) -> *mut c_void;
        pub fn wxObject_GetRefData(self_: *const c_void) -> *mut c_void;
        pub fn wxObject_IsKindOf(self_: *const c_void, info: *const c_void) -> bool;
        pub fn wxObject_IsSameAs(self_: *const c_void, obj: *const c_void) -> bool;
        pub fn wxObject_Ref(self_: *mut c_void, clone: *const c_void);
        pub fn wxObject_SetRefData(self_: *mut c_void, data: *mut c_void);
        pub fn wxObject_UnRef(self_: *mut c_void);
        pub fn wxObject_UnShare(self_: *mut c_void);
        // BLOCKED: pub fn wxObject_operator delete(self_: *mut c_void, buf: *mut c_void);
        // NOT_SUPPORTED: pub fn wxObject_operator new(self_: *mut c_void, size: size_t, filename: *const c_void, line_num: c_int) -> *mut c_void;
        
        // wxEvtHandler
        pub fn wxEvtHandler_QueueEvent(self_: *mut c_void, event: *mut c_void);
        pub fn wxEvtHandler_AddPendingEvent(self_: *mut c_void, event: *const c_void);
        // NOT_SUPPORTED: pub fn wxEvtHandler_CallAfter(self_: *mut c_void, method: *mut c_void, x1: T1, None: ...);
        // BLOCKED: pub fn wxEvtHandler_CallAfter1(self_: *mut c_void, functor: *const c_void);
        pub fn wxEvtHandler_ProcessEvent(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxEvtHandler_ProcessEventLocally(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxEvtHandler_SafelyProcessEvent(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxEvtHandler_ProcessPendingEvents(self_: *mut c_void);
        pub fn wxEvtHandler_DeletePendingEvents(self_: *mut c_void);
        pub fn wxEvtHandler_SearchEventTable(self_: *mut c_void, table: *mut c_void, event: *mut c_void) -> bool;
        // NOT_SUPPORTED: pub fn wxEvtHandler_Connect(self_: *mut c_void, id: c_int, last_id: c_int, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void);
        // NOT_SUPPORTED: pub fn wxEvtHandler_Connect1(self_: *mut c_void, id: c_int, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void);
        // NOT_SUPPORTED: pub fn wxEvtHandler_Connect2(self_: *mut c_void, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void);
        // NOT_SUPPORTED: pub fn wxEvtHandler_Disconnect(self_: *mut c_void, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void) -> bool;
        // NOT_SUPPORTED: pub fn wxEvtHandler_Disconnect1(self_: *mut c_void, id: c_int, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void) -> bool;
        // NOT_SUPPORTED: pub fn wxEvtHandler_Disconnect2(self_: *mut c_void, id: c_int, last_id: c_int, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void) -> bool;
        // NOT_SUPPORTED: pub fn wxEvtHandler_Bind(self_: *mut c_void, event_type: *const c_void, functor: Functor, id: c_int, last_id: c_int, user_data: *mut c_void);
        // BLOCKED: pub fn wxEvtHandler_Bind1(self_: *mut c_void, event_type: *const c_void, method: *mut c_void, handler: *mut c_void, id: c_int, last_id: c_int, user_data: *mut c_void);
        // NOT_SUPPORTED: pub fn wxEvtHandler_Unbind(self_: *mut c_void, event_type: *const c_void, functor: Functor, id: c_int, last_id: c_int, user_data: *mut c_void) -> bool;
        // BLOCKED: pub fn wxEvtHandler_Unbind1(self_: *mut c_void, event_type: *const c_void, method: *mut c_void, handler: *mut c_void, id: c_int, last_id: c_int, user_data: *mut c_void) -> bool;
        // BLOCKED: pub fn wxEvtHandler_GetClientData(self_: *const c_void) -> *mut c_void;
        pub fn wxEvtHandler_GetClientObject(self_: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxEvtHandler_SetClientData(self_: *mut c_void, data: *mut c_void);
        pub fn wxEvtHandler_SetClientObject(self_: *mut c_void, data: *mut c_void);
        pub fn wxEvtHandler_GetEvtHandlerEnabled(self_: *const c_void) -> bool;
        pub fn wxEvtHandler_GetNextHandler(self_: *const c_void) -> *mut c_void;
        pub fn wxEvtHandler_GetPreviousHandler(self_: *const c_void) -> *mut c_void;
        pub fn wxEvtHandler_SetEvtHandlerEnabled(self_: *mut c_void, enabled: bool);
        pub fn wxEvtHandler_SetNextHandler(self_: *mut c_void, handler: *mut c_void);
        pub fn wxEvtHandler_SetPreviousHandler(self_: *mut c_void, handler: *mut c_void);
        pub fn wxEvtHandler_Unlink(self_: *mut c_void);
        pub fn wxEvtHandler_IsUnlinked(self_: *const c_void) -> bool;
        pub fn wxEvtHandler_AddFilter(filter: *mut c_void);
        pub fn wxEvtHandler_RemoveFilter(filter: *mut c_void);
        pub fn wxEvtHandler_new() -> *mut c_void;
        // DTOR: pub fn wxEvtHandler_~wxEvtHandler(self_: *mut c_void);
        
    }
}

pub trait WxRustMethods {
    unsafe fn as_ptr(&self) -> *mut c_void;
}

// wxObject
wx_class! { Object(wxObject) impl
    ObjectMethods
}
impl Object {
    pub fn new() -> Object {
        unsafe { Object(ffi::wxObject_new()) }
    }
    pub fn new1(other: &Object) -> Object {
        unsafe {
            let other = other.as_ptr();
            Object(ffi::wxObject_new1(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Drop for Object {
    fn drop(&mut self) {
        unsafe { ffi::wxObject_delete(self.0) }
    }
}
pub trait ObjectMethods: WxRustMethods {
    // DTOR: fn ~wxObject()
    fn get_class_info(&self) -> *mut c_void {
        unsafe { ffi::wxObject_GetClassInfo(self.as_ptr()) }
    }
    fn get_ref_data(&self) -> *mut c_void {
        unsafe { ffi::wxObject_GetRefData(self.as_ptr()) }
    }
    fn is_kind_of(&self, info: *const c_void) -> bool {
        unsafe { ffi::wxObject_IsKindOf(self.as_ptr(), info) }
    }
    fn is_same_as(&self, obj: &Object) -> bool {
        unsafe {
            let obj = obj.as_ptr();
            ffi::wxObject_IsSameAs(self.as_ptr(), obj)
        }
    }
    fn ref_(&self, clone: &Object) {
        unsafe {
            let clone = clone.as_ptr();
            ffi::wxObject_Ref(self.as_ptr(), clone)
        }
    }
    fn set_ref_data(&self, data: *mut c_void) {
        unsafe { ffi::wxObject_SetRefData(self.as_ptr(), data) }
    }
    fn un_ref(&self) {
        unsafe { ffi::wxObject_UnRef(self.as_ptr()) }
    }
    fn un_share(&self) {
        unsafe { ffi::wxObject_UnShare(self.as_ptr()) }
    }
    // BLOCKED: fn operator delete()
    // NOT_SUPPORTED: fn operator new()
}

// wxEvtHandler
wx_class! { EvtHandler(wxEvtHandler) impl
    EvtHandlerMethods,
    ObjectMethods
}
impl EvtHandler {
    pub fn new() -> EvtHandler {
        unsafe { EvtHandler(ffi::wxEvtHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Drop for EvtHandler {
    fn drop(&mut self) {
        unsafe { ffi::wxObject_delete(self.0) }
    }
}
pub trait EvtHandlerMethods: ObjectMethods {
    fn queue_event(&self, event: *mut c_void) {
        unsafe { ffi::wxEvtHandler_QueueEvent(self.as_ptr(), event) }
    }
    fn add_pending_event(&self, event: *const c_void) {
        unsafe { ffi::wxEvtHandler_AddPendingEvent(self.as_ptr(), event) }
    }
    // NOT_SUPPORTED: fn CallAfter()
    // BLOCKED: fn CallAfter1()
    fn process_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxEvtHandler_ProcessEvent(self.as_ptr(), event) }
    }
    fn process_event_locally(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxEvtHandler_ProcessEventLocally(self.as_ptr(), event) }
    }
    fn safely_process_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxEvtHandler_SafelyProcessEvent(self.as_ptr(), event) }
    }
    fn process_pending_events(&self) {
        unsafe { ffi::wxEvtHandler_ProcessPendingEvents(self.as_ptr()) }
    }
    fn delete_pending_events(&self) {
        unsafe { ffi::wxEvtHandler_DeletePendingEvents(self.as_ptr()) }
    }
    fn search_event_table(&self, table: *mut c_void, event: *mut c_void) -> bool {
        unsafe { ffi::wxEvtHandler_SearchEventTable(self.as_ptr(), table, event) }
    }
    // NOT_SUPPORTED: fn Connect()
    // NOT_SUPPORTED: fn Connect1()
    // NOT_SUPPORTED: fn Connect2()
    // NOT_SUPPORTED: fn Disconnect()
    // NOT_SUPPORTED: fn Disconnect1()
    // NOT_SUPPORTED: fn Disconnect2()
    // NOT_SUPPORTED: fn Bind()
    // BLOCKED: fn Bind1()
    // NOT_SUPPORTED: fn Unbind()
    // BLOCKED: fn Unbind1()
    // BLOCKED: fn GetClientData()
    fn get_client_object(&self) -> *mut c_void {
        unsafe { ffi::wxEvtHandler_GetClientObject(self.as_ptr()) }
    }
    // BLOCKED: fn SetClientData()
    fn set_client_object(&self, data: *mut c_void) {
        unsafe { ffi::wxEvtHandler_SetClientObject(self.as_ptr(), data) }
    }
    fn get_evt_handler_enabled(&self) -> bool {
        unsafe { ffi::wxEvtHandler_GetEvtHandlerEnabled(self.as_ptr()) }
    }
    fn get_next_handler(&self) -> *mut c_void {
        unsafe { ffi::wxEvtHandler_GetNextHandler(self.as_ptr()) }
    }
    fn get_previous_handler(&self) -> *mut c_void {
        unsafe { ffi::wxEvtHandler_GetPreviousHandler(self.as_ptr()) }
    }
    fn set_evt_handler_enabled(&self, enabled: bool) {
        unsafe { ffi::wxEvtHandler_SetEvtHandlerEnabled(self.as_ptr(), enabled) }
    }
    fn set_next_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_SetNextHandler(self.as_ptr(), handler)
        }
    }
    fn set_previous_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_SetPreviousHandler(self.as_ptr(), handler)
        }
    }
    fn unlink(&self) {
        unsafe { ffi::wxEvtHandler_Unlink(self.as_ptr()) }
    }
    fn is_unlinked(&self) -> bool {
        unsafe { ffi::wxEvtHandler_IsUnlinked(self.as_ptr()) }
    }
    fn add_filter(filter: *mut c_void) {
        unsafe { ffi::wxEvtHandler_AddFilter(filter) }
    }
    fn remove_filter(filter: *mut c_void) {
        unsafe { ffi::wxEvtHandler_RemoveFilter(filter) }
    }
    // DTOR: fn ~wxEvtHandler()
}

