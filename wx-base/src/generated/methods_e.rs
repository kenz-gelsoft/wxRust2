use super::*;

// wxEvent
/// This trait represents [C++ `wxEvent` class](https://docs.wxwidgets.org/3.2/classwx_event.html)'s methods and inheritance.
///
/// See [`EventIsOwned`] documentation for the class usage.
pub trait EventMethods: ObjectMethods {
    /// Returns a copy of the event.
    ///
    /// See [C++ `wxEvent::Clone()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_event.html#a1458e0e59752bd8753ec20cb719e088b).
    fn clone(&self) -> Event {
        unsafe { Event::from_ptr(ffi::wxEvent_Clone(self.as_ptr())) }
    }
    /// Returns the object (usually a window) associated with the event, if any.
    ///
    /// See [C++ `wxEvent::GetEventObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_event.html#a1ed12f8a9b61af6a76c6746cb8acfeae).
    fn get_event_object(&self) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxEvent_GetEventObject(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetEventType()
    // NOT_SUPPORTED: fn GetEventCategory()
    /// Returns the identifier associated with this event, such as a button command id.
    ///
    /// See [C++ `wxEvent::GetId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_event.html#ac5fa5c10d4845d903e58026a42b403c7).
    fn get_id(&self) -> c_int {
        unsafe { ffi::wxEvent_GetId(self.as_ptr()) }
    }
    /// Return the user data associated with a dynamically connected event handler.
    ///
    /// See [C++ `wxEvent::GetEventUserData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_event.html#aa46a14bdca4d0ebcd4f42e5805db1df9).
    fn get_event_user_data(&self) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxEvent_GetEventUserData(self.as_ptr())) }
    }
    /// Returns true if the event handler should be skipped, false otherwise.
    ///
    /// See [C++ `wxEvent::GetSkipped()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_event.html#aa770dbcd0f1714ba097836af6534b4c5).
    fn get_skipped(&self) -> bool {
        unsafe { ffi::wxEvent_GetSkipped(self.as_ptr()) }
    }
    /// Gets the timestamp for the event.
    ///
    /// See [C++ `wxEvent::GetTimestamp()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_event.html#a48662230971127737d2500cef7be549d).
    fn get_timestamp(&self) -> c_long {
        unsafe { ffi::wxEvent_GetTimestamp(self.as_ptr()) }
    }
    /// Returns true if the event is or is derived from wxCommandEvent else it returns false.
    ///
    /// See [C++ `wxEvent::IsCommandEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_event.html#a31f6a4377d6d36964b8eae4e56ec43e0).
    fn is_command_event(&self) -> bool {
        unsafe { ffi::wxEvent_IsCommandEvent(self.as_ptr()) }
    }
    /// Sets the propagation level to the given value (for example returned from an earlier call to wxEvent::StopPropagation).
    ///
    /// See [C++ `wxEvent::ResumePropagation()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_event.html#a0acb5c75f6e67b8822ad8ba3c5bdc4fe).
    fn resume_propagation(&self, propagation_level: c_int) {
        unsafe { ffi::wxEvent_ResumePropagation(self.as_ptr(), propagation_level) }
    }
    /// Sets the originating object.
    ///
    /// See [C++ `wxEvent::SetEventObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_event.html#a3460217d04c36393ab868ba453fde13d).
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
    ///
    /// See [C++ `wxEvent::SetId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_event.html#ab9973f687bfa8a60318d8d9bd629d0d4).
    fn set_id(&self, id: c_int) {
        unsafe { ffi::wxEvent_SetId(self.as_ptr(), id) }
    }
    /// Sets the timestamp for the event.
    ///
    /// See [C++ `wxEvent::SetTimestamp()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_event.html#ad4380dff3144a986cb960473051a1d8d).
    fn set_timestamp(&self, time_stamp: c_long) {
        unsafe { ffi::wxEvent_SetTimestamp(self.as_ptr(), time_stamp) }
    }
    /// Test if this event should be propagated or not, i.e. if the propagation level is currently greater than 0.
    ///
    /// See [C++ `wxEvent::ShouldPropagate()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_event.html#ac7a178c3c781c80f1308945042f76e7f).
    fn should_propagate(&self) -> bool {
        unsafe { ffi::wxEvent_ShouldPropagate(self.as_ptr()) }
    }
    /// This method can be used inside an event handler to control whether further event handlers bound to this event will be called after the current one returns.
    ///
    /// See [C++ `wxEvent::Skip()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_event.html#a98eb20b76106f9a933c2eb3ee119f66c).
    fn skip(&self, skip: bool) {
        unsafe { ffi::wxEvent_Skip(self.as_ptr(), skip) }
    }
    /// Stop the event from propagating to its parent window.
    ///
    /// See [C++ `wxEvent::StopPropagation()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_event.html#a060a7d222404daff4d3cef30cddeaae3).
    fn stop_propagation(&self) -> c_int {
        unsafe { ffi::wxEvent_StopPropagation(self.as_ptr()) }
    }
}

// wxEvtHandler
/// This trait represents [C++ `wxEvtHandler` class](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html)'s methods and inheritance.
///
/// See [`EvtHandlerIsOwned`] documentation for the class usage.
pub trait EvtHandlerMethods: ObjectMethods {
    /// Queue event for a later processing.
    ///
    /// See [C++ `wxEvtHandler::QueueEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#acffd03bf407a856166ea71ef0318b59a).
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
    ///
    /// See [C++ `wxEvtHandler::AddPendingEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#a0737c6d2cbcd5ded4b1ecdd53ed0def3).
    fn add_pending_event<E: EventMethods>(&self, event: &E) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxEvtHandler_AddPendingEvent(self.as_ptr(), event)
        }
    }
    // NOT_SUPPORTED: fn CallAfter()
    // BLOCKED: fn CallAfter1()
    /// Processes an event, searching event tables and calling zero or more suitable event handler function(s).
    ///
    /// See [C++ `wxEvtHandler::ProcessEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#a65968dd27f3aac7718f2dd6b2ddd5a08).
    fn process_event<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxEvtHandler_ProcessEvent(self.as_ptr(), event)
        }
    }
    /// Try to process the event in this handler and all those chained to it.
    ///
    /// See [C++ `wxEvtHandler::ProcessEventLocally()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#ac0f5d2cb29a04c1f7f82eb6b351f79fb).
    fn process_event_locally<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxEvtHandler_ProcessEventLocally(self.as_ptr(), event)
        }
    }
    /// Processes an event by calling ProcessEvent() and handles any exceptions that occur in the process.
    ///
    /// See [C++ `wxEvtHandler::SafelyProcessEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#a8205cb1a5a00d8b550b3ead22266b16b).
    fn safely_process_event<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxEvtHandler_SafelyProcessEvent(self.as_ptr(), event)
        }
    }
    /// Processes the pending events previously queued using QueueEvent() or AddPendingEvent(); you must call this function only if you are sure there are pending events for this handler, otherwise a wxCHECK will fail.
    ///
    /// See [C++ `wxEvtHandler::ProcessPendingEvents()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#a6f643dbdcf8e914ae1c8b70dd305e6f2).
    fn process_pending_events(&self) {
        unsafe { ffi::wxEvtHandler_ProcessPendingEvents(self.as_ptr()) }
    }
    /// Deletes all events queued on this event handler using QueueEvent() or AddPendingEvent().
    ///
    /// See [C++ `wxEvtHandler::DeletePendingEvents()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#a6e7f9cf4ebd0623c1d94979855d096f8).
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
    ///
    /// See [C++ `wxEvtHandler::GetClientObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#a537d17d644e48bc1735c4dd28b8b8c04).
    fn get_client_object(&self) -> Option<ClientDataIsOwned<false>> {
        unsafe { ClientData::option_from(ffi::wxEvtHandler_GetClientObject(self.as_ptr())) }
    }
    // BLOCKED: fn SetClientData()
    /// Set the client data object.
    ///
    /// See [C++ `wxEvtHandler::SetClientObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#af1e33a06087b8b2ddc43c7d15a91b326).
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
    ///
    /// See [C++ `wxEvtHandler::GetEvtHandlerEnabled()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#a7742d81c5eb7849a0ad75b9de8575153).
    fn get_evt_handler_enabled(&self) -> bool {
        unsafe { ffi::wxEvtHandler_GetEvtHandlerEnabled(self.as_ptr()) }
    }
    /// Returns the pointer to the next handler in the chain.
    ///
    /// See [C++ `wxEvtHandler::GetNextHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#a6de721ad9f331826a5c925d6008116e5).
    fn get_next_handler(&self) -> WeakRef<EvtHandler> {
        unsafe { WeakRef::<EvtHandler>::from(ffi::wxEvtHandler_GetNextHandler(self.as_ptr())) }
    }
    /// Returns the pointer to the previous handler in the chain.
    ///
    /// See [C++ `wxEvtHandler::GetPreviousHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#abbf9904ea5108b816f5f4faab1a33db9).
    fn get_previous_handler(&self) -> WeakRef<EvtHandler> {
        unsafe { WeakRef::<EvtHandler>::from(ffi::wxEvtHandler_GetPreviousHandler(self.as_ptr())) }
    }
    /// Enables or disables the event handler.
    ///
    /// See [C++ `wxEvtHandler::SetEvtHandlerEnabled()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#a7388ae19c8657e5656471b658c320036).
    fn set_evt_handler_enabled(&self, enabled: bool) {
        unsafe { ffi::wxEvtHandler_SetEvtHandlerEnabled(self.as_ptr(), enabled) }
    }
    /// Sets the pointer to the next handler.
    ///
    /// See [C++ `wxEvtHandler::SetNextHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#a68e2ef2d2b7d68c4c9c18ca92933031b).
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
    ///
    /// See [C++ `wxEvtHandler::SetPreviousHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#aff0d1836464be82e2ad723ad3a58eccc).
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
    ///
    /// See [C++ `wxEvtHandler::Unlink()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#a22e5db2ec1d19c8252c056fd116975d7).
    fn unlink(&self) {
        unsafe { ffi::wxEvtHandler_Unlink(self.as_ptr()) }
    }
    /// Returns true if the next and the previous handler pointers of this event handler instance are NULL.
    ///
    /// See [C++ `wxEvtHandler::IsUnlinked()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#ac0734578a8d929b8b0be440ce0b53ad6).
    fn is_unlinked(&self) -> bool {
        unsafe { ffi::wxEvtHandler_IsUnlinked(self.as_ptr()) }
    }
    /// Add an event filter whose FilterEvent() method will be called for each and every event processed by wxWidgets.
    ///
    /// See [C++ `wxEvtHandler::AddFilter()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#a7dc3c701781f4044372049de5004137e).
    fn add_filter(filter: *mut c_void) {
        unsafe { ffi::wxEvtHandler_AddFilter(filter) }
    }
    /// Remove a filter previously installed with AddFilter().
    ///
    /// See [C++ `wxEvtHandler::RemoveFilter()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#a67a57b759c447b121bf70a7c9804c8f2).
    fn remove_filter(filter: *mut c_void) {
        unsafe { ffi::wxEvtHandler_RemoveFilter(filter) }
    }
    // DTOR: fn ~wxEvtHandler()
}
