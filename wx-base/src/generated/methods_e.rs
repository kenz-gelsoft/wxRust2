use super::*;

// wxEvent
pub trait EventMethods: ObjectMethods {
    /// Returns a copy of the event.
    fn clone(&self) -> Event {
        unsafe { Event::from_ptr(ffi::wxEvent_Clone(self.as_ptr())) }
    }
    /// Returns the object (usually a window) associated with the event, if any.
    fn get_event_object(&self) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxEvent_GetEventObject(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetEventType()
    // NOT_SUPPORTED: fn GetEventCategory()
    /// Returns the identifier associated with this event, such as a button command id.
    fn get_id(&self) -> c_int {
        unsafe { ffi::wxEvent_GetId(self.as_ptr()) }
    }
    /// Return the user data associated with a dynamically connected event handler.
    fn get_event_user_data(&self) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxEvent_GetEventUserData(self.as_ptr())) }
    }
    /// Returns true if the event handler should be skipped, false otherwise.
    fn get_skipped(&self) -> bool {
        unsafe { ffi::wxEvent_GetSkipped(self.as_ptr()) }
    }
    /// Gets the timestamp for the event.
    fn get_timestamp(&self) -> c_long {
        unsafe { ffi::wxEvent_GetTimestamp(self.as_ptr()) }
    }
    /// Returns true if the event is or is derived from wxCommandEvent else it returns false.
    fn is_command_event(&self) -> bool {
        unsafe { ffi::wxEvent_IsCommandEvent(self.as_ptr()) }
    }
    /// Sets the propagation level to the given value (for example returned from an earlier call to wxEvent::StopPropagation).
    fn resume_propagation(&self, propagation_level: c_int) {
        unsafe { ffi::wxEvent_ResumePropagation(self.as_ptr(), propagation_level) }
    }
    /// Sets the originating object.
    fn set_event_object<O: ObjectMethods>(&self, object: Option<&O>) {
        unsafe {
            let object = match object {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvent_SetEventObject(self.as_ptr(), object)
        }
    }
    // NOT_SUPPORTED: fn SetEventType()
    /// Sets the identifier associated with this event, such as a button command id.
    fn set_id(&self, id: c_int) {
        unsafe { ffi::wxEvent_SetId(self.as_ptr(), id) }
    }
    /// Sets the timestamp for the event.
    fn set_timestamp(&self, time_stamp: c_long) {
        unsafe { ffi::wxEvent_SetTimestamp(self.as_ptr(), time_stamp) }
    }
    /// Test if this event should be propagated or not, i.e. if the propagation level is currently greater than 0.
    fn should_propagate(&self) -> bool {
        unsafe { ffi::wxEvent_ShouldPropagate(self.as_ptr()) }
    }
    /// This method can be used inside an event handler to control whether further event handlers bound to this event will be called after the current one returns.
    fn skip(&self, skip: bool) {
        unsafe { ffi::wxEvent_Skip(self.as_ptr(), skip) }
    }
    /// Stop the event from propagating to its parent window.
    fn stop_propagation(&self) -> c_int {
        unsafe { ffi::wxEvent_StopPropagation(self.as_ptr()) }
    }
}

// wxEvtHandler
pub trait EvtHandlerMethods: ObjectMethods {
    /// Queue event for a later processing.
    fn queue_event<E: EventMethods>(&self, event: Option<&E>) {
        unsafe {
            let event = match event {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_QueueEvent(self.as_ptr(), event)
        }
    }
    /// Post an event to be processed later.
    fn add_pending_event<E: EventMethods>(&self, event: &E) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxEvtHandler_AddPendingEvent(self.as_ptr(), event)
        }
    }
    // NOT_SUPPORTED: fn CallAfter()
    // BLOCKED: fn CallAfter1()
    /// Processes an event, searching event tables and calling zero or more suitable event handler function(s).
    fn process_event<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxEvtHandler_ProcessEvent(self.as_ptr(), event)
        }
    }
    /// Try to process the event in this handler and all those chained to it.
    fn process_event_locally<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxEvtHandler_ProcessEventLocally(self.as_ptr(), event)
        }
    }
    /// Processes an event by calling ProcessEvent() and handles any exceptions that occur in the process.
    fn safely_process_event<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxEvtHandler_SafelyProcessEvent(self.as_ptr(), event)
        }
    }
    /// Processes the pending events previously queued using QueueEvent() or AddPendingEvent(); you must call this function only if you are sure there are pending events for this handler, otherwise a wxCHECK will fail.
    fn process_pending_events(&self) {
        unsafe { ffi::wxEvtHandler_ProcessPendingEvents(self.as_ptr()) }
    }
    /// Deletes all events queued on this event handler using QueueEvent() or AddPendingEvent().
    fn delete_pending_events(&self) {
        unsafe { ffi::wxEvtHandler_DeletePendingEvents(self.as_ptr()) }
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
    /// Returns a pointer to the user-supplied client data object.
    fn get_client_object(&self) -> Option<ClientDataIsOwned<false>> {
        unsafe { ClientData::option_from(ffi::wxEvtHandler_GetClientObject(self.as_ptr())) }
    }
    // BLOCKED: fn SetClientData()
    /// Set the client data object.
    fn set_client_object<C: ClientDataMethods>(&self, data: Option<&C>) {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_SetClientObject(self.as_ptr(), data)
        }
    }
    /// Returns true if the event handler is enabled, false otherwise.
    fn get_evt_handler_enabled(&self) -> bool {
        unsafe { ffi::wxEvtHandler_GetEvtHandlerEnabled(self.as_ptr()) }
    }
    /// Returns the pointer to the next handler in the chain.
    fn get_next_handler(&self) -> WeakRef<EvtHandler> {
        unsafe { WeakRef::<EvtHandler>::from(ffi::wxEvtHandler_GetNextHandler(self.as_ptr())) }
    }
    /// Returns the pointer to the previous handler in the chain.
    fn get_previous_handler(&self) -> WeakRef<EvtHandler> {
        unsafe { WeakRef::<EvtHandler>::from(ffi::wxEvtHandler_GetPreviousHandler(self.as_ptr())) }
    }
    /// Enables or disables the event handler.
    fn set_evt_handler_enabled(&self, enabled: bool) {
        unsafe { ffi::wxEvtHandler_SetEvtHandlerEnabled(self.as_ptr(), enabled) }
    }
    /// Sets the pointer to the next handler.
    fn set_next_handler<E: EvtHandlerMethods>(&self, handler: Option<&E>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_SetNextHandler(self.as_ptr(), handler)
        }
    }
    /// Sets the pointer to the previous handler.
    fn set_previous_handler<E: EvtHandlerMethods>(&self, handler: Option<&E>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_SetPreviousHandler(self.as_ptr(), handler)
        }
    }
    /// Unlinks this event handler from the chain it's part of (if any); then links the "previous" event handler to the "next" one (so that the chain won't be interrupted).
    fn unlink(&self) {
        unsafe { ffi::wxEvtHandler_Unlink(self.as_ptr()) }
    }
    /// Returns true if the next and the previous handler pointers of this event handler instance are NULL.
    fn is_unlinked(&self) -> bool {
        unsafe { ffi::wxEvtHandler_IsUnlinked(self.as_ptr()) }
    }
    /// Add an event filter whose FilterEvent() method will be called for each and every event processed by wxWidgets.
    fn add_filter(filter: *mut c_void) {
        unsafe { ffi::wxEvtHandler_AddFilter(filter) }
    }
    /// Remove a filter previously installed with AddFilter().
    fn remove_filter(filter: *mut c_void) {
        unsafe { ffi::wxEvtHandler_RemoveFilter(filter) }
    }
    // DTOR: fn ~wxEvtHandler()
}
