use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};

pub use crate::ffi::*;

extern "C" {

    // wxEvent
    // NOT_SUPPORTED: pub fn wxEvent_new(id: c_int, event_type: wxEventType) -> *mut c_void;
    pub fn wxEvent_Clone(self_: *const c_void) -> *mut c_void;
    pub fn wxEvent_GetEventObject(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxEvent_GetEventType(self_: *const c_void) -> wxEventType;
    // NOT_SUPPORTED: pub fn wxEvent_GetEventCategory(self_: *const c_void) -> wxEventCategory;
    pub fn wxEvent_GetId(self_: *const c_void) -> c_int;
    pub fn wxEvent_GetEventUserData(self_: *const c_void) -> *mut c_void;
    pub fn wxEvent_GetSkipped(self_: *const c_void) -> bool;
    pub fn wxEvent_GetTimestamp(self_: *const c_void) -> c_long;
    pub fn wxEvent_IsCommandEvent(self_: *const c_void) -> bool;
    pub fn wxEvent_ResumePropagation(self_: *mut c_void, propagation_level: c_int);
    pub fn wxEvent_SetEventObject(self_: *mut c_void, object: *mut c_void);
    // NOT_SUPPORTED: pub fn wxEvent_SetEventType(self_: *mut c_void, type_: wxEventType);
    pub fn wxEvent_SetId(self_: *mut c_void, id: c_int);
    pub fn wxEvent_SetTimestamp(self_: *mut c_void, time_stamp: c_long);
    pub fn wxEvent_ShouldPropagate(self_: *const c_void) -> bool;
    pub fn wxEvent_Skip(self_: *mut c_void, skip: bool);
    pub fn wxEvent_StopPropagation(self_: *mut c_void) -> c_int;

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
    // BLOCKED: pub fn wxObject_operator new(self_: *mut c_void, size: usize, filename: *const c_void, line_num: c_int) -> *mut c_void;

}
