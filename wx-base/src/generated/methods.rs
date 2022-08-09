use std::os::raw::{c_int, c_long, c_uint, c_void};

use super::*;
use crate::WeakRef;

pub trait WxRustMethods {
    type Unowned;
    unsafe fn as_ptr(&self) -> *mut c_void;
    unsafe fn from_ptr(ptr: *mut c_void) -> Self;
    unsafe fn from_unowned_ptr(ptr: *mut c_void) -> Self::Unowned;
    unsafe fn with_ptr<F: Fn(&Self)>(ptr: *mut c_void, closure: F);
    unsafe fn option_from(ptr: *mut c_void) -> Option<Self::Unowned>
    where
        Self: Sized,
    {
        if ptr.is_null() {
            None
        } else {
            Some(Self::from_unowned_ptr(ptr))
        }
    }
}

// wxAny
pub trait AnyMethods: WxRustMethods {
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator=1()
    // BLOCKED: fn operator=2()
    // NOT_SUPPORTED: fn operator==()
    // NOT_SUPPORTED: fn operator==1()
    // NOT_SUPPORTED: fn operator==2()
    // NOT_SUPPORTED: fn operator==3()
    // NOT_SUPPORTED: fn operator==4()
    // NOT_SUPPORTED: fn operator==5()
    // NOT_SUPPORTED: fn operator==6()
    // BLOCKED: fn operator==7()
    // NOT_SUPPORTED: fn operator==8()
    // NOT_SUPPORTED: fn operator==9()
    // NOT_SUPPORTED: fn operator==10()
    // BLOCKED: fn operator==11()
    // BLOCKED: fn operator==12()
    // BLOCKED: fn operator==13()
    // BLOCKED: fn operator==14()
    // BLOCKED: fn operator==15()
    // NOT_SUPPORTED: fn operator!=()
    // NOT_SUPPORTED: fn operator!=1()
    // NOT_SUPPORTED: fn operator!=2()
    // NOT_SUPPORTED: fn operator!=3()
    // NOT_SUPPORTED: fn operator!=4()
    // NOT_SUPPORTED: fn operator!=5()
    // NOT_SUPPORTED: fn operator!=6()
    // BLOCKED: fn operator!=7()
    // NOT_SUPPORTED: fn operator!=8()
    // NOT_SUPPORTED: fn operator!=9()
    // NOT_SUPPORTED: fn operator!=10()
    // BLOCKED: fn operator!=11()
    // BLOCKED: fn operator!=12()
    // BLOCKED: fn operator!=13()
    // BLOCKED: fn operator!=14()
    // BLOCKED: fn operator!=15()
    // DTOR: fn ~wxAny()
    // NOT_SUPPORTED: fn As()
    fn check_type(&self) -> bool {
        unsafe { ffi::wxAny_CheckType(self.as_ptr()) }
    }
    fn get_as_t(&self, value: *mut c_void) -> bool {
        unsafe { ffi::wxAny_GetAs(self.as_ptr(), value) }
    }
    fn get_as_variant(&self, value: *mut c_void) -> bool {
        unsafe { ffi::wxAny_GetAs1(self.as_ptr(), value) }
    }
    fn get_type(&self) -> Option<AnyValueTypeIsOwned<false>> {
        unsafe { AnyValueType::option_from(ffi::wxAny_GetType(self.as_ptr())) }
    }
    fn has_same_type<A: AnyMethods>(&self, other: &A) -> bool {
        unsafe {
            let other = other.as_ptr();
            ffi::wxAny_HasSameType(self.as_ptr(), other)
        }
    }
    fn is_null(&self) -> bool {
        unsafe { ffi::wxAny_IsNull(self.as_ptr()) }
    }
    fn make_null(&self) {
        unsafe { ffi::wxAny_MakeNull(self.as_ptr()) }
    }
}

// wxAnyValueType
pub trait AnyValueTypeMethods: WxRustMethods {
    // DTOR: fn ~wxAnyValueType()
    fn check_type(&self) -> bool {
        unsafe { ffi::wxAnyValueType_CheckType(self.as_ptr()) }
    }
    fn convert_value<A: AnyValueTypeMethods>(
        &self,
        src: *const c_void,
        dst_type: Option<&A>,
        dst: *mut c_void,
    ) -> bool {
        unsafe {
            let dst_type = match dst_type {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxAnyValueType_ConvertValue(self.as_ptr(), src, dst_type, dst)
        }
    }
    fn copy_buffer(&self, src: *const c_void, dst: *mut c_void) {
        unsafe { ffi::wxAnyValueType_CopyBuffer(self.as_ptr(), src, dst) }
    }
    fn delete_value(&self, buf: *mut c_void) {
        unsafe { ffi::wxAnyValueType_DeleteValue(self.as_ptr(), buf) }
    }
    fn is_same_type<A: AnyValueTypeMethods>(&self, other_type: Option<&A>) -> bool {
        unsafe {
            let other_type = match other_type {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxAnyValueType_IsSameType(self.as_ptr(), other_type)
        }
    }
}

// wxAppTraits
pub trait AppTraitsMethods: WxRustMethods {
    fn create_config(&self) -> *mut c_void {
        unsafe { ffi::wxAppTraits_CreateConfig(self.as_ptr()) }
    }
    fn create_event_loop(&self) -> *mut c_void {
        unsafe { ffi::wxAppTraits_CreateEventLoop(self.as_ptr()) }
    }
    fn create_font_mapper(&self) -> Option<FontMapperIsOwned<false>> {
        unsafe { FontMapper::option_from(ffi::wxAppTraits_CreateFontMapper(self.as_ptr())) }
    }
    fn create_log_target(&self) -> Option<LogIsOwned<false>> {
        unsafe { Log::option_from(ffi::wxAppTraits_CreateLogTarget(self.as_ptr())) }
    }
    fn create_message_output(&self) -> Option<MessageOutputIsOwned<false>> {
        unsafe { MessageOutput::option_from(ffi::wxAppTraits_CreateMessageOutput(self.as_ptr())) }
    }
    fn create_renderer(&self) -> Option<RendererNativeIsOwned<false>> {
        unsafe { RendererNative::option_from(ffi::wxAppTraits_CreateRenderer(self.as_ptr())) }
    }
    fn get_desktop_environment(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAppTraits_GetDesktopEnvironment(self.as_ptr())).into() }
    }
    fn get_standard_paths(&self) -> StandardPathsIsOwned<false> {
        unsafe { StandardPathsIsOwned::from_ptr(ffi::wxAppTraits_GetStandardPaths(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetToolkitVersion()
    fn has_stderr(&self) -> bool {
        unsafe { ffi::wxAppTraits_HasStderr(self.as_ptr()) }
    }
    fn is_using_universal_widgets(&self) -> bool {
        unsafe { ffi::wxAppTraits_IsUsingUniversalWidgets(self.as_ptr()) }
    }
    fn show_assert_dialog(&self, msg: &str) -> bool {
        unsafe {
            let msg = WxString::from(msg);
            let msg = msg.as_ptr();
            ffi::wxAppTraits_ShowAssertDialog(self.as_ptr(), msg)
        }
    }
    fn safe_message_box(&self, text: &str, title: &str) -> bool {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxAppTraits_SafeMessageBox(self.as_ptr(), text, title)
        }
    }
    fn get_assert_stack_trace(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAppTraits_GetAssertStackTrace(self.as_ptr())).into() }
    }
}

// wxArchiveClassFactory
pub trait ArchiveClassFactoryMethods: ObjectMethods {
    // NOT_SUPPORTED: fn CanHandle()
    fn get_conv(&self) -> *mut c_void {
        unsafe { ffi::wxArchiveClassFactory_GetConv(self.as_ptr()) }
    }
    fn set_conv(&self, conv: *mut c_void) {
        unsafe { ffi::wxArchiveClassFactory_SetConv(self.as_ptr(), conv) }
    }
    fn get_next(&self) -> Option<ArchiveClassFactoryIsOwned<false>> {
        unsafe {
            ArchiveClassFactory::option_from(ffi::wxArchiveClassFactory_GetNext(self.as_ptr()))
        }
    }
    // NOT_SUPPORTED: fn GetInternalName()
    fn get_protocol(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxArchiveClassFactory_GetProtocol(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetProtocols()
    fn new_entry(&self) -> Option<ArchiveEntryIsOwned<false>> {
        unsafe { ArchiveEntry::option_from(ffi::wxArchiveClassFactory_NewEntry(self.as_ptr())) }
    }
    fn new_stream_inputstream(&self, stream: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxArchiveClassFactory_NewStream(self.as_ptr(), stream) }
    }
    fn new_stream_outputstream(&self, stream: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxArchiveClassFactory_NewStream1(self.as_ptr(), stream) }
    }
    fn new_stream_inputstream(&self, stream: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxArchiveClassFactory_NewStream2(self.as_ptr(), stream) }
    }
    fn new_stream_outputstream(&self, stream: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxArchiveClassFactory_NewStream3(self.as_ptr(), stream) }
    }
    fn push_front(&self) {
        unsafe { ffi::wxArchiveClassFactory_PushFront(self.as_ptr()) }
    }
    fn remove(&self) {
        unsafe { ffi::wxArchiveClassFactory_Remove(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Find()
    fn get_first() -> Option<ArchiveClassFactoryIsOwned<false>> {
        unsafe { ArchiveClassFactory::option_from(ffi::wxArchiveClassFactory_GetFirst()) }
    }
}

// wxArchiveEntry
pub trait ArchiveEntryMethods: ObjectMethods {
    fn clone(&self) -> Option<ArchiveEntryIsOwned<false>> {
        unsafe { ArchiveEntry::option_from(ffi::wxArchiveEntry_Clone(self.as_ptr())) }
    }
    fn get_date_time(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxArchiveEntry_GetDateTime(self.as_ptr())) }
    }
    fn set_date_time<D: DateTimeMethods>(&self, dt: &D) {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxArchiveEntry_SetDateTime(self.as_ptr(), dt)
        }
    }
    // NOT_SUPPORTED: fn GetName()
    // NOT_SUPPORTED: fn SetName()
    // NOT_SUPPORTED: fn GetSize()
    // NOT_SUPPORTED: fn SetSize()
    // NOT_SUPPORTED: fn GetInternalFormat()
    fn get_internal_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxArchiveEntry_GetInternalName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetOffset()
    fn is_dir(&self) -> bool {
        unsafe { ffi::wxArchiveEntry_IsDir(self.as_ptr()) }
    }
    fn set_is_dir(&self, is_dir: bool) {
        unsafe { ffi::wxArchiveEntry_SetIsDir(self.as_ptr(), is_dir) }
    }
    fn is_read_only(&self) -> bool {
        unsafe { ffi::wxArchiveEntry_IsReadOnly(self.as_ptr()) }
    }
    fn set_is_read_only(&self, is_read_only: bool) {
        unsafe { ffi::wxArchiveEntry_SetIsReadOnly(self.as_ptr(), is_read_only) }
    }
    fn set_notifier<A: ArchiveNotifierMethods>(&self, notifier: &A) {
        unsafe {
            let notifier = notifier.as_ptr();
            ffi::wxArchiveEntry_SetNotifier(self.as_ptr(), notifier)
        }
    }
    fn unset_notifier(&self) {
        unsafe { ffi::wxArchiveEntry_UnsetNotifier(self.as_ptr()) }
    }
}

// wxArchiveIterator
pub trait ArchiveIteratorMethods: WxRustMethods {
    // NOT_SUPPORTED: fn operator*()
    // BLOCKED: fn operator++()
    // BLOCKED: fn operator++1()
}

// wxArchiveNotifier
pub trait ArchiveNotifierMethods: WxRustMethods {
    fn on_entry_updated<A: ArchiveEntryMethods>(&self, entry: &A) {
        unsafe {
            let entry = entry.as_ptr();
            ffi::wxArchiveNotifier_OnEntryUpdated(self.as_ptr(), entry)
        }
    }
}

// wxClassInfo
pub trait ClassInfoMethods: WxRustMethods {
    fn create_object(&self) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxClassInfo_CreateObject(self.as_ptr())) }
    }
    fn get_base_class_name1(&self) -> *const c_void {
        unsafe { ffi::wxClassInfo_GetBaseClassName1(self.as_ptr()) }
    }
    fn get_base_class_name2(&self) -> *const c_void {
        unsafe { ffi::wxClassInfo_GetBaseClassName2(self.as_ptr()) }
    }
    fn get_class_name(&self) -> *const c_void {
        unsafe { ffi::wxClassInfo_GetClassName(self.as_ptr()) }
    }
    fn get_size(&self) -> c_int {
        unsafe { ffi::wxClassInfo_GetSize(self.as_ptr()) }
    }
    fn is_dynamic(&self) -> bool {
        unsafe { ffi::wxClassInfo_IsDynamic(self.as_ptr()) }
    }
    fn is_kind_of<C: ClassInfoMethods>(&self, info: Option<&C>) -> bool {
        unsafe {
            let info = match info {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxClassInfo_IsKindOf(self.as_ptr(), info)
        }
    }
    fn find_class(class_name: &str) -> Option<ClassInfoIsOwned<false>> {
        unsafe {
            let class_name = WxString::from(class_name);
            let class_name = class_name.as_ptr();
            ClassInfo::option_from(ffi::wxClassInfo_FindClass(class_name))
        }
    }
}

// wxClient
pub trait ClientMethods: ObjectMethods {
    fn make_connection(
        &self,
        host: &str,
        service: &str,
        topic: &str,
    ) -> Option<ConnectionBaseIsOwned<false>> {
        unsafe {
            let host = WxString::from(host);
            let host = host.as_ptr();
            let service = WxString::from(service);
            let service = service.as_ptr();
            let topic = WxString::from(topic);
            let topic = topic.as_ptr();
            ConnectionBase::option_from(ffi::wxClient_MakeConnection(
                self.as_ptr(),
                host,
                service,
                topic,
            ))
        }
    }
    fn on_make_connection(&self) -> Option<ConnectionBaseIsOwned<false>> {
        unsafe { ConnectionBase::option_from(ffi::wxClient_OnMakeConnection(self.as_ptr())) }
    }
    fn valid_host(&self, host: &str) -> bool {
        unsafe {
            let host = WxString::from(host);
            let host = host.as_ptr();
            ffi::wxClient_ValidHost(self.as_ptr(), host)
        }
    }
}

// wxClientData
pub trait ClientDataMethods: WxRustMethods {
    // DTOR: fn ~wxClientData()
}

// wxClientDataContainer
pub trait ClientDataContainerMethods: WxRustMethods {
    // DTOR: fn ~wxClientDataContainer()
    fn get_client_data(&self) -> *mut c_void {
        unsafe { ffi::wxClientDataContainer_GetClientData(self.as_ptr()) }
    }
    fn get_client_object(&self) -> Option<ClientDataIsOwned<false>> {
        unsafe {
            ClientData::option_from(ffi::wxClientDataContainer_GetClientObject(self.as_ptr()))
        }
    }
    fn set_client_data(&self, data: *mut c_void) {
        unsafe { ffi::wxClientDataContainer_SetClientData(self.as_ptr(), data) }
    }
    fn set_client_object<C: ClientDataMethods>(&self, data: Option<&C>) {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxClientDataContainer_SetClientObject(self.as_ptr(), data)
        }
    }
}

// wxCmdLineParser
pub trait CmdLineParserMethods: WxRustMethods {
    // DTOR: fn ~wxCmdLineParser()
    // NOT_SUPPORTED: fn AddLongOption()
    fn add_long_switch(&self, lng: &str, desc: &str, flags: c_int) {
        unsafe {
            let lng = WxString::from(lng);
            let lng = lng.as_ptr();
            let desc = WxString::from(desc);
            let desc = desc.as_ptr();
            ffi::wxCmdLineParser_AddLongSwitch(self.as_ptr(), lng, desc, flags)
        }
    }
    // NOT_SUPPORTED: fn AddOption()
    // NOT_SUPPORTED: fn AddParam()
    fn add_switch(&self, name: &str, lng: &str, desc: &str, flags: c_int) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let lng = WxString::from(lng);
            let lng = lng.as_ptr();
            let desc = WxString::from(desc);
            let desc = desc.as_ptr();
            ffi::wxCmdLineParser_AddSwitch(self.as_ptr(), name, lng, desc, flags)
        }
    }
    fn add_usage_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxCmdLineParser_AddUsageText(self.as_ptr(), text)
        }
    }
    fn are_long_options_enabled(&self) -> bool {
        unsafe { ffi::wxCmdLineParser_AreLongOptionsEnabled(self.as_ptr()) }
    }
    fn disable_long_options(&self) {
        unsafe { ffi::wxCmdLineParser_DisableLongOptions(self.as_ptr()) }
    }
    fn enable_long_options(&self, enable: bool) {
        unsafe { ffi::wxCmdLineParser_EnableLongOptions(self.as_ptr(), enable) }
    }
    fn found(&self, name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxCmdLineParser_Found(self.as_ptr(), name)
        }
    }
    // NOT_SUPPORTED: fn FoundSwitch()
    fn found_str(&self, name: &str, value: *mut c_void) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxCmdLineParser_Found1(self.as_ptr(), name, value)
        }
    }
    fn found_long(&self, name: &str, value: *mut c_void) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxCmdLineParser_Found2(self.as_ptr(), name, value)
        }
    }
    fn found_double(&self, name: &str, value: *mut c_void) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxCmdLineParser_Found3(self.as_ptr(), name, value)
        }
    }
    fn found_datetime<D: DateTimeMethods>(&self, name: &str, value: Option<&D>) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let value = match value {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxCmdLineParser_Found4(self.as_ptr(), name, value)
        }
    }
    fn get_param(&self, n: usize) -> String {
        unsafe { WxString::from_ptr(ffi::wxCmdLineParser_GetParam(self.as_ptr(), n)).into() }
    }
    fn get_param_count(&self) -> usize {
        unsafe { ffi::wxCmdLineParser_GetParamCount(self.as_ptr()) }
    }
    fn get_arguments(&self) -> CmdLineArgs {
        unsafe { CmdLineArgs::from_ptr(ffi::wxCmdLineParser_GetArguments(self.as_ptr())) }
    }
    fn parse(&self, give_usage: bool) -> c_int {
        unsafe { ffi::wxCmdLineParser_Parse(self.as_ptr(), give_usage) }
    }
    fn set_cmd_line_int_char(&self, argc: c_int, argv: *mut c_void) {
        unsafe { ffi::wxCmdLineParser_SetCmdLine(self.as_ptr(), argc, argv) }
    }
    fn set_cmd_line_int_wchar_t(&self, argc: c_int, argv: *mut c_void) {
        unsafe { ffi::wxCmdLineParser_SetCmdLine1(self.as_ptr(), argc, argv) }
    }
    fn set_cmd_line_str(&self, cmdline: &str) {
        unsafe {
            let cmdline = WxString::from(cmdline);
            let cmdline = cmdline.as_ptr();
            ffi::wxCmdLineParser_SetCmdLine2(self.as_ptr(), cmdline)
        }
    }
    fn set_desc(&self, desc: *const c_void) {
        unsafe { ffi::wxCmdLineParser_SetDesc(self.as_ptr(), desc) }
    }
    fn set_logo(&self, logo: &str) {
        unsafe {
            let logo = WxString::from(logo);
            let logo = logo.as_ptr();
            ffi::wxCmdLineParser_SetLogo(self.as_ptr(), logo)
        }
    }
    fn set_switch_chars(&self, switch_chars: &str) {
        unsafe {
            let switch_chars = WxString::from(switch_chars);
            let switch_chars = switch_chars.as_ptr();
            ffi::wxCmdLineParser_SetSwitchChars(self.as_ptr(), switch_chars)
        }
    }
    fn usage(&self) {
        unsafe { ffi::wxCmdLineParser_Usage(self.as_ptr()) }
    }
    fn get_usage_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxCmdLineParser_GetUsageString(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn ConvertStringToArgs()
}

// wxCondition
pub trait ConditionMethods: WxRustMethods {
    // DTOR: fn ~wxCondition()
    // NOT_SUPPORTED: fn Broadcast()
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxCondition_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Signal()
    // NOT_SUPPORTED: fn Wait()
    // NOT_SUPPORTED: fn Wait1()
    // NOT_SUPPORTED: fn WaitTimeout()
}

// wxConfigPathChanger
pub trait ConfigPathChangerMethods: WxRustMethods {
    // DTOR: fn ~wxConfigPathChanger()
    fn name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxConfigPathChanger_Name(self.as_ptr())).into() }
    }
    fn update_if_deleted(&self) {
        unsafe { ffi::wxConfigPathChanger_UpdateIfDeleted(self.as_ptr()) }
    }
}

// wxConnection
pub trait ConnectionMethods: ObjectMethods {
    // NOT_SUPPORTED: fn Advise()
    fn advise_char(&self, item: &str, data: *const c_void) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxConnection_Advise1(self.as_ptr(), item, data)
        }
    }
    fn advise_wchar_t(&self, item: &str, data: *const c_void) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxConnection_Advise2(self.as_ptr(), item, data)
        }
    }
    fn advise_str(&self, item: &str, data: ffi::wxString) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxConnection_Advise3(self.as_ptr(), item, data)
        }
    }
    fn disconnect(&self) -> bool {
        unsafe { ffi::wxConnection_Disconnect(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Execute()
    fn execute_char(&self, data: *const c_void) -> bool {
        unsafe { ffi::wxConnection_Execute1(self.as_ptr(), data) }
    }
    fn execute_wchar_t(&self, data: *const c_void) -> bool {
        unsafe { ffi::wxConnection_Execute2(self.as_ptr(), data) }
    }
    fn execute_str(&self, data: ffi::wxString) -> bool {
        unsafe { ffi::wxConnection_Execute3(self.as_ptr(), data) }
    }
    // NOT_SUPPORTED: fn OnAdvise()
    fn on_disconnect(&self) -> bool {
        unsafe { ffi::wxConnection_OnDisconnect(self.as_ptr()) }
    }
    fn on_exec(&self, topic: &str, data: &str) -> bool {
        unsafe {
            let topic = WxString::from(topic);
            let topic = topic.as_ptr();
            let data = WxString::from(data);
            let data = data.as_ptr();
            ffi::wxConnection_OnExec(self.as_ptr(), topic, data)
        }
    }
    // NOT_SUPPORTED: fn OnPoke()
    // NOT_SUPPORTED: fn OnRequest()
    fn on_start_advise(&self, topic: &str, item: &str) -> bool {
        unsafe {
            let topic = WxString::from(topic);
            let topic = topic.as_ptr();
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxConnection_OnStartAdvise(self.as_ptr(), topic, item)
        }
    }
    fn on_stop_advise(&self, topic: &str, item: &str) -> bool {
        unsafe {
            let topic = WxString::from(topic);
            let topic = topic.as_ptr();
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxConnection_OnStopAdvise(self.as_ptr(), topic, item)
        }
    }
    // NOT_SUPPORTED: fn Poke()
    fn poke_char(&self, item: &str, data: *const c_void) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxConnection_Poke1(self.as_ptr(), item, data)
        }
    }
    fn poke_wchar_t(&self, item: &str, data: *const c_void) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxConnection_Poke2(self.as_ptr(), item, data)
        }
    }
    fn poke_str(&self, item: &str, data: ffi::wxString) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxConnection_Poke3(self.as_ptr(), item, data)
        }
    }
    // NOT_SUPPORTED: fn Request()
    fn start_advise(&self, item: &str) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxConnection_StartAdvise(self.as_ptr(), item)
        }
    }
    fn stop_advise(&self, item: &str) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxConnection_StopAdvise(self.as_ptr(), item)
        }
    }
    // NOT_SUPPORTED: fn IsTextFormat()
    // NOT_SUPPORTED: fn GetTextFromData()
}

// wxConnectionBase
pub trait ConnectionBaseMethods: ObjectMethods {}

// wxCriticalSection
pub trait CriticalSectionMethods: WxRustMethods {
    // DTOR: fn ~wxCriticalSection()
    fn enter(&self) {
        unsafe { ffi::wxCriticalSection_Enter(self.as_ptr()) }
    }
    fn try_enter(&self) -> bool {
        unsafe { ffi::wxCriticalSection_TryEnter(self.as_ptr()) }
    }
    fn leave(&self) {
        unsafe { ffi::wxCriticalSection_Leave(self.as_ptr()) }
    }
}

// wxCriticalSectionLocker
pub trait CriticalSectionLockerMethods: WxRustMethods {
    // DTOR: fn ~wxCriticalSectionLocker()
}

// wxDDEClient
pub trait DDEClientMethods: ObjectMethods {
    fn make_connection(
        &self,
        host: &str,
        service: &str,
        topic: &str,
    ) -> Option<ConnectionBaseIsOwned<false>> {
        unsafe {
            let host = WxString::from(host);
            let host = host.as_ptr();
            let service = WxString::from(service);
            let service = service.as_ptr();
            let topic = WxString::from(topic);
            let topic = topic.as_ptr();
            ConnectionBase::option_from(ffi::wxDDEClient_MakeConnection(
                self.as_ptr(),
                host,
                service,
                topic,
            ))
        }
    }
    fn on_make_connection(&self) -> Option<ConnectionBaseIsOwned<false>> {
        unsafe { ConnectionBase::option_from(ffi::wxDDEClient_OnMakeConnection(self.as_ptr())) }
    }
    fn valid_host(&self, host: &str) -> bool {
        unsafe {
            let host = WxString::from(host);
            let host = host.as_ptr();
            ffi::wxDDEClient_ValidHost(self.as_ptr(), host)
        }
    }
}

// wxDDEConnection
pub trait DDEConnectionMethods: ConnectionBaseMethods {
    // NOT_SUPPORTED: fn Advise()
    fn advise_char(&self, item: &str, data: *const c_void) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxDDEConnection_Advise1(self.as_ptr(), item, data)
        }
    }
    fn advise_wchar_t(&self, item: &str, data: *const c_void) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxDDEConnection_Advise2(self.as_ptr(), item, data)
        }
    }
    fn advise_str(&self, item: &str, data: ffi::wxString) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxDDEConnection_Advise3(self.as_ptr(), item, data)
        }
    }
    fn disconnect(&self) -> bool {
        unsafe { ffi::wxDDEConnection_Disconnect(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Execute()
    fn execute_char(&self, data: *const c_void) -> bool {
        unsafe { ffi::wxDDEConnection_Execute1(self.as_ptr(), data) }
    }
    fn execute_wchar_t(&self, data: *const c_void) -> bool {
        unsafe { ffi::wxDDEConnection_Execute2(self.as_ptr(), data) }
    }
    fn execute_str(&self, data: ffi::wxString) -> bool {
        unsafe { ffi::wxDDEConnection_Execute3(self.as_ptr(), data) }
    }
    // NOT_SUPPORTED: fn OnAdvise()
    fn on_disconnect(&self) -> bool {
        unsafe { ffi::wxDDEConnection_OnDisconnect(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn OnExecute()
    // NOT_SUPPORTED: fn OnPoke()
    // NOT_SUPPORTED: fn OnRequest()
    fn on_start_advise(&self, topic: &str, item: &str) -> bool {
        unsafe {
            let topic = WxString::from(topic);
            let topic = topic.as_ptr();
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxDDEConnection_OnStartAdvise(self.as_ptr(), topic, item)
        }
    }
    fn on_stop_advise(&self, topic: &str, item: &str) -> bool {
        unsafe {
            let topic = WxString::from(topic);
            let topic = topic.as_ptr();
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxDDEConnection_OnStopAdvise(self.as_ptr(), topic, item)
        }
    }
    // NOT_SUPPORTED: fn Poke()
    fn poke_char(&self, item: &str, data: *const c_void) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxDDEConnection_Poke1(self.as_ptr(), item, data)
        }
    }
    fn poke_wchar_t(&self, item: &str, data: *const c_void) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxDDEConnection_Poke2(self.as_ptr(), item, data)
        }
    }
    fn poke_str(&self, item: &str, data: ffi::wxString) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxDDEConnection_Poke3(self.as_ptr(), item, data)
        }
    }
    // NOT_SUPPORTED: fn Request()
    fn start_advise(&self, item: &str) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxDDEConnection_StartAdvise(self.as_ptr(), item)
        }
    }
    fn stop_advise(&self, item: &str) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxDDEConnection_StopAdvise(self.as_ptr(), item)
        }
    }
}

// wxDDEServer
pub trait DDEServerMethods: WxRustMethods {
    fn create(&self, service: &str) -> bool {
        unsafe {
            let service = WxString::from(service);
            let service = service.as_ptr();
            ffi::wxDDEServer_Create(self.as_ptr(), service)
        }
    }
    fn on_accept_connection(&self, topic: &str) -> Option<ConnectionBaseIsOwned<false>> {
        unsafe {
            let topic = WxString::from(topic);
            let topic = topic.as_ptr();
            ConnectionBase::option_from(ffi::wxDDEServer_OnAcceptConnection(self.as_ptr(), topic))
        }
    }
}

// wxDataInputStream
pub trait DataInputStreamMethods: WxRustMethods {
    // DTOR: fn ~wxDataInputStream()
    fn big_endian_ordered(&self, be_order: bool) {
        unsafe { ffi::wxDataInputStream_BigEndianOrdered(self.as_ptr(), be_order) }
    }
    fn get_conv(&self) -> *mut c_void {
        unsafe { ffi::wxDataInputStream_GetConv(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Read8()
    fn read8(&self, buffer: *mut c_void, size: usize) {
        unsafe { ffi::wxDataInputStream_Read81(self.as_ptr(), buffer, size) }
    }
    // NOT_SUPPORTED: fn Read16()
    fn read16(&self, buffer: *mut c_void, size: usize) {
        unsafe { ffi::wxDataInputStream_Read161(self.as_ptr(), buffer, size) }
    }
    // NOT_SUPPORTED: fn Read32()
    fn read32(&self, buffer: *mut c_void, size: usize) {
        unsafe { ffi::wxDataInputStream_Read321(self.as_ptr(), buffer, size) }
    }
    // NOT_SUPPORTED: fn Read64()
    fn read64(&self, buffer: *mut c_void, size: usize) {
        unsafe { ffi::wxDataInputStream_Read641(self.as_ptr(), buffer, size) }
    }
    // NOT_SUPPORTED: fn ReadFloat()
    fn read_float(&self, buffer: *mut c_void, size: usize) {
        unsafe { ffi::wxDataInputStream_ReadFloat1(self.as_ptr(), buffer, size) }
    }
    fn read_double(&self) -> c_double {
        unsafe { ffi::wxDataInputStream_ReadDouble(self.as_ptr()) }
    }
    fn read_double_double(&self, buffer: *mut c_void, size: usize) {
        unsafe { ffi::wxDataInputStream_ReadDouble1(self.as_ptr(), buffer, size) }
    }
    fn read_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDataInputStream_ReadString(self.as_ptr())).into() }
    }
    fn set_conv(&self, conv: *const c_void) {
        unsafe { ffi::wxDataInputStream_SetConv(self.as_ptr(), conv) }
    }
    fn use_basic_precisions(&self) {
        unsafe { ffi::wxDataInputStream_UseBasicPrecisions(self.as_ptr()) }
    }
    fn use_extended_precision(&self) {
        unsafe { ffi::wxDataInputStream_UseExtendedPrecision(self.as_ptr()) }
    }
}

// wxDataOutputStream
pub trait DataOutputStreamMethods: WxRustMethods {
    // DTOR: fn ~wxDataOutputStream()
    fn big_endian_ordered(&self, be_order: bool) {
        unsafe { ffi::wxDataOutputStream_BigEndianOrdered(self.as_ptr(), be_order) }
    }
    fn get_conv(&self) -> *mut c_void {
        unsafe { ffi::wxDataOutputStream_GetConv(self.as_ptr()) }
    }
    fn set_conv(&self, conv: *const c_void) {
        unsafe { ffi::wxDataOutputStream_SetConv(self.as_ptr(), conv) }
    }
    fn use_basic_precisions(&self) {
        unsafe { ffi::wxDataOutputStream_UseBasicPrecisions(self.as_ptr()) }
    }
    fn use_extended_precision(&self) {
        unsafe { ffi::wxDataOutputStream_UseExtendedPrecision(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Write8()
    fn write8(&self, buffer: *const c_void, size: usize) {
        unsafe { ffi::wxDataOutputStream_Write81(self.as_ptr(), buffer, size) }
    }
    // NOT_SUPPORTED: fn Write16()
    fn write16(&self, buffer: *const c_void, size: usize) {
        unsafe { ffi::wxDataOutputStream_Write161(self.as_ptr(), buffer, size) }
    }
    // NOT_SUPPORTED: fn Write32()
    fn write32(&self, buffer: *const c_void, size: usize) {
        unsafe { ffi::wxDataOutputStream_Write321(self.as_ptr(), buffer, size) }
    }
    // NOT_SUPPORTED: fn Write64()
    fn write64(&self, buffer: *const c_void, size: usize) {
        unsafe { ffi::wxDataOutputStream_Write641(self.as_ptr(), buffer, size) }
    }
    // NOT_SUPPORTED: fn WriteFloat()
    fn write_float(&self, buffer: *const c_void, size: usize) {
        unsafe { ffi::wxDataOutputStream_WriteFloat1(self.as_ptr(), buffer, size) }
    }
    fn write_double_double(&self, d: c_double) {
        unsafe { ffi::wxDataOutputStream_WriteDouble(self.as_ptr(), d) }
    }
    fn write_double_double(&self, buffer: *const c_void, size: usize) {
        unsafe { ffi::wxDataOutputStream_WriteDouble1(self.as_ptr(), buffer, size) }
    }
    fn write_string(&self, string: &str) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            ffi::wxDataOutputStream_WriteString(self.as_ptr(), string)
        }
    }
}

// wxDateSpan
pub trait DateSpanMethods: WxRustMethods {
    fn add<D: DateSpanMethods>(&self, other: &D) -> DateSpan {
        unsafe {
            let other = other.as_ptr();
            DateSpan::from_ptr(ffi::wxDateSpan_Add(self.as_ptr(), other))
        }
    }
    fn add<D: DateSpanMethods>(&self, other: &D) -> &Self {
        unsafe {
            let other = other.as_ptr();
            ffi::wxDateSpan_Add1(self.as_ptr(), other);
            &self
        }
    }
    fn get_days(&self) -> c_int {
        unsafe { ffi::wxDateSpan_GetDays(self.as_ptr()) }
    }
    fn get_months(&self) -> c_int {
        unsafe { ffi::wxDateSpan_GetMonths(self.as_ptr()) }
    }
    fn get_total_months(&self) -> c_int {
        unsafe { ffi::wxDateSpan_GetTotalMonths(self.as_ptr()) }
    }
    fn get_total_days(&self) -> c_int {
        unsafe { ffi::wxDateSpan_GetTotalDays(self.as_ptr()) }
    }
    fn get_weeks(&self) -> c_int {
        unsafe { ffi::wxDateSpan_GetWeeks(self.as_ptr()) }
    }
    fn get_years(&self) -> c_int {
        unsafe { ffi::wxDateSpan_GetYears(self.as_ptr()) }
    }
    fn multiply(&self, factor: c_int) -> DateSpan {
        unsafe { DateSpan::from_ptr(ffi::wxDateSpan_Multiply(self.as_ptr(), factor)) }
    }
    fn multiply(&self, factor: c_int) -> &Self {
        unsafe {
            ffi::wxDateSpan_Multiply1(self.as_ptr(), factor);
            &self
        }
    }
    fn neg(&self) -> &Self {
        unsafe {
            ffi::wxDateSpan_Neg(self.as_ptr());
            &self
        }
    }
    fn negate(&self) -> DateSpan {
        unsafe { DateSpan::from_ptr(ffi::wxDateSpan_Negate(self.as_ptr())) }
    }
    fn set_days(&self, n: c_int) -> &Self {
        unsafe {
            ffi::wxDateSpan_SetDays(self.as_ptr(), n);
            &self
        }
    }
    fn set_months(&self, n: c_int) -> &Self {
        unsafe {
            ffi::wxDateSpan_SetMonths(self.as_ptr(), n);
            &self
        }
    }
    fn set_weeks(&self, n: c_int) -> &Self {
        unsafe {
            ffi::wxDateSpan_SetWeeks(self.as_ptr(), n);
            &self
        }
    }
    fn set_years(&self, n: c_int) -> &Self {
        unsafe {
            ffi::wxDateSpan_SetYears(self.as_ptr(), n);
            &self
        }
    }
    fn subtract<D: DateSpanMethods>(&self, other: &D) -> DateSpan {
        unsafe {
            let other = other.as_ptr();
            DateSpan::from_ptr(ffi::wxDateSpan_Subtract(self.as_ptr(), other))
        }
    }
    fn subtract<D: DateSpanMethods>(&self, other: &D) -> &Self {
        unsafe {
            let other = other.as_ptr();
            ffi::wxDateSpan_Subtract1(self.as_ptr(), other);
            &self
        }
    }
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // BLOCKED: fn operator-()
    // BLOCKED: fn operator*=()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator==()
    fn day() -> DateSpan {
        unsafe { DateSpan::from_ptr(ffi::wxDateSpan_Day()) }
    }
    fn days(days: c_int) -> DateSpan {
        unsafe { DateSpan::from_ptr(ffi::wxDateSpan_Days(days)) }
    }
    fn month() -> DateSpan {
        unsafe { DateSpan::from_ptr(ffi::wxDateSpan_Month()) }
    }
    fn months(mon: c_int) -> DateSpan {
        unsafe { DateSpan::from_ptr(ffi::wxDateSpan_Months(mon)) }
    }
    fn week() -> DateSpan {
        unsafe { DateSpan::from_ptr(ffi::wxDateSpan_Week()) }
    }
    fn weeks(weeks: c_int) -> DateSpan {
        unsafe { DateSpan::from_ptr(ffi::wxDateSpan_Weeks(weeks)) }
    }
    fn year() -> DateSpan {
        unsafe { DateSpan::from_ptr(ffi::wxDateSpan_Year()) }
    }
    fn years(years: c_int) -> DateSpan {
        unsafe { DateSpan::from_ptr(ffi::wxDateSpan_Years(years)) }
    }
}

// wxDateTime
pub trait DateTimeMethods: WxRustMethods {
    fn reset_time(&self) -> &Self {
        unsafe {
            ffi::wxDateTime_ResetTime(self.as_ptr());
            &self
        }
    }
    // NOT_SUPPORTED: fn Set()
    // BLOCKED: fn Set1()
    fn set_tm(&self, tm: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Set2(self.as_ptr(), tm);
            &self
        }
    }
    fn set_double(&self, jdn: c_double) -> &Self {
        unsafe {
            ffi::wxDateTime_Set3(self.as_ptr(), jdn);
            &self
        }
    }
    // NOT_SUPPORTED: fn Set4()
    // NOT_SUPPORTED: fn Set5()
    // NOT_SUPPORTED: fn SetDay()
    // NOT_SUPPORTED: fn SetFromDOS()
    // NOT_SUPPORTED: fn SetHour()
    // NOT_SUPPORTED: fn SetMillisecond()
    // NOT_SUPPORTED: fn SetMinute()
    // NOT_SUPPORTED: fn SetMonth()
    // NOT_SUPPORTED: fn SetSecond()
    fn set_to_current(&self) -> &Self {
        unsafe {
            ffi::wxDateTime_SetToCurrent(self.as_ptr());
            &self
        }
    }
    fn set_year(&self, year: c_int) -> &Self {
        unsafe {
            ffi::wxDateTime_SetYear(self.as_ptr(), year);
            &self
        }
    }
    // NOT_SUPPORTED: fn operator=()
    // BLOCKED: fn operator=1()
    // NOT_SUPPORTED: fn GetAsDOS()
    fn set_from_msw_sys_time(&self, st: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_SetFromMSWSysTime(self.as_ptr(), st);
            &self
        }
    }
    fn get_as_msw_sys_time(&self, st: *mut c_void) {
        unsafe { ffi::wxDateTime_GetAsMSWSysTime(self.as_ptr(), st) }
    }
    fn get_century_timezone(&self, tz: *const c_void) -> c_int {
        unsafe { ffi::wxDateTime_GetCentury(self.as_ptr(), tz) }
    }
    fn get_date_only(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_GetDateOnly(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetDay()
    // NOT_SUPPORTED: fn GetDayOfYear()
    // NOT_SUPPORTED: fn GetHour()
    // NOT_SUPPORTED: fn GetMillisecond()
    // NOT_SUPPORTED: fn GetMinute()
    // NOT_SUPPORTED: fn GetMonth()
    // NOT_SUPPORTED: fn GetSecond()
    // NOT_SUPPORTED: fn GetTicks()
    // NOT_SUPPORTED: fn GetValue()
    // NOT_SUPPORTED: fn GetTm()
    // NOT_SUPPORTED: fn GetWeekDay()
    fn get_week_based_year(&self, tz: *const c_void) -> c_int {
        unsafe { ffi::wxDateTime_GetWeekBasedYear(self.as_ptr(), tz) }
    }
    // NOT_SUPPORTED: fn GetWeekOfMonth()
    // NOT_SUPPORTED: fn GetWeekOfYear()
    fn get_year(&self, tz: *const c_void) -> c_int {
        unsafe { ffi::wxDateTime_GetYear(self.as_ptr(), tz) }
    }
    fn is_valid(&self) -> bool {
        unsafe { ffi::wxDateTime_IsValid(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn IsWorkDay()
    fn is_earlier_than<D: DateTimeMethods>(&self, datetime: &D) -> bool {
        unsafe {
            let datetime = datetime.as_ptr();
            ffi::wxDateTime_IsEarlierThan(self.as_ptr(), datetime)
        }
    }
    fn is_equal_to<D: DateTimeMethods>(&self, datetime: &D) -> bool {
        unsafe {
            let datetime = datetime.as_ptr();
            ffi::wxDateTime_IsEqualTo(self.as_ptr(), datetime)
        }
    }
    fn is_equal_up_to<D: DateTimeMethods>(&self, dt: &D, ts: *const c_void) -> bool {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxDateTime_IsEqualUpTo(self.as_ptr(), dt, ts)
        }
    }
    fn is_later_than<D: DateTimeMethods>(&self, datetime: &D) -> bool {
        unsafe {
            let datetime = datetime.as_ptr();
            ffi::wxDateTime_IsLaterThan(self.as_ptr(), datetime)
        }
    }
    fn is_same_date<D: DateTimeMethods>(&self, dt: &D) -> bool {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxDateTime_IsSameDate(self.as_ptr(), dt)
        }
    }
    fn is_same_time<D: DateTimeMethods>(&self, dt: &D) -> bool {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxDateTime_IsSameTime(self.as_ptr(), dt)
        }
    }
    fn is_strictly_between<D: DateTimeMethods, D2: DateTimeMethods>(
        &self,
        t1: &D,
        t2: &D2,
    ) -> bool {
        unsafe {
            let t1 = t1.as_ptr();
            let t2 = t2.as_ptr();
            ffi::wxDateTime_IsStrictlyBetween(self.as_ptr(), t1, t2)
        }
    }
    fn is_between<D: DateTimeMethods, D2: DateTimeMethods>(&self, t1: &D, t2: &D2) -> bool {
        unsafe {
            let t1 = t1.as_ptr();
            let t2 = t2.as_ptr();
            ffi::wxDateTime_IsBetween(self.as_ptr(), t1, t2)
        }
    }
    // BLOCKED: fn Add()
    fn add_datespan<D: DateSpanMethods>(&self, diff: &D) -> &Self {
        unsafe {
            let diff = diff.as_ptr();
            ffi::wxDateTime_Add1(self.as_ptr(), diff);
            &self
        }
    }
    // BLOCKED: fn Add2()
    fn add_timespan(&self, diff: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Add3(self.as_ptr(), diff);
            &self
        }
    }
    // BLOCKED: fn Subtract()
    fn subtract_timespan(&self, diff: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Subtract1(self.as_ptr(), diff);
            &self
        }
    }
    // BLOCKED: fn Subtract2()
    fn subtract_datespan<D: DateSpanMethods>(&self, diff: &D) -> &Self {
        unsafe {
            let diff = diff.as_ptr();
            ffi::wxDateTime_Subtract3(self.as_ptr(), diff);
            &self
        }
    }
    // NOT_SUPPORTED: fn Subtract4()
    fn diff_as_date_span<D: DateTimeMethods>(&self, dt: &D) -> DateSpan {
        unsafe {
            let dt = dt.as_ptr();
            DateSpan::from_ptr(ffi::wxDateTime_DiffAsDateSpan(self.as_ptr(), dt))
        }
    }
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator-=()
    // BLOCKED: fn operator-()
    // BLOCKED: fn operator+=1()
    // BLOCKED: fn operator+1()
    // BLOCKED: fn operator-=1()
    // BLOCKED: fn operator-1()
    // BLOCKED: fn operator-2()
    fn format(&self, format: &str, tz: *const c_void) -> String {
        unsafe {
            let format = WxString::from(format);
            let format = format.as_ptr();
            WxString::from_ptr(ffi::wxDateTime_Format(self.as_ptr(), format, tz)).into()
        }
    }
    fn format_date(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDateTime_FormatDate(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn FormatISOCombined()
    fn format_iso_date(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDateTime_FormatISODate(self.as_ptr())).into() }
    }
    fn format_iso_time(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDateTime_FormatISOTime(self.as_ptr())).into() }
    }
    fn format_time(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDateTime_FormatTime(self.as_ptr())).into() }
    }
    // BLOCKED: fn ParseDate()
    fn parse_date_time(&self, datetime: &str, end: *mut c_void) -> bool {
        unsafe {
            let datetime = WxString::from(datetime);
            let datetime = datetime.as_ptr();
            ffi::wxDateTime_ParseDateTime(self.as_ptr(), datetime, end)
        }
    }
    fn parse_format<D: DateTimeMethods>(
        &self,
        date: &str,
        format: &str,
        date_def: &D,
        end: *mut c_void,
    ) -> bool {
        unsafe {
            let date = WxString::from(date);
            let date = date.as_ptr();
            let format = WxString::from(format);
            let format = format.as_ptr();
            let date_def = date_def.as_ptr();
            ffi::wxDateTime_ParseFormat(self.as_ptr(), date, format, date_def, end)
        }
    }
    // BLOCKED: fn ParseFormat1()
    // BLOCKED: fn ParseFormat2()
    // NOT_SUPPORTED: fn ParseISOCombined()
    fn parse_iso_date(&self, date: &str) -> bool {
        unsafe {
            let date = WxString::from(date);
            let date = date.as_ptr();
            ffi::wxDateTime_ParseISODate(self.as_ptr(), date)
        }
    }
    fn parse_iso_time(&self, date: &str) -> bool {
        unsafe {
            let date = WxString::from(date);
            let date = date.as_ptr();
            ffi::wxDateTime_ParseISOTime(self.as_ptr(), date)
        }
    }
    fn parse_rfc822_date(&self, date: &str, end: *mut c_void) -> bool {
        unsafe {
            let date = WxString::from(date);
            let date = date.as_ptr();
            ffi::wxDateTime_ParseRfc822Date(self.as_ptr(), date, end)
        }
    }
    fn parse_time(&self, time: &str, end: *mut c_void) -> bool {
        unsafe {
            let time = WxString::from(time);
            let time = time.as_ptr();
            ffi::wxDateTime_ParseTime(self.as_ptr(), time, end)
        }
    }
    // NOT_SUPPORTED: fn GetLastMonthDay()
    // NOT_SUPPORTED: fn GetLastWeekDay()
    // NOT_SUPPORTED: fn GetNextWeekDay()
    // NOT_SUPPORTED: fn GetPrevWeekDay()
    // NOT_SUPPORTED: fn GetWeekDay1()
    // NOT_SUPPORTED: fn GetWeekDayInSameWeek()
    // NOT_SUPPORTED: fn GetYearDay()
    // NOT_SUPPORTED: fn SetToLastMonthDay()
    // NOT_SUPPORTED: fn SetToLastWeekDay()
    // NOT_SUPPORTED: fn SetToNextWeekDay()
    // NOT_SUPPORTED: fn SetToPrevWeekDay()
    // NOT_SUPPORTED: fn SetToWeekDay()
    // NOT_SUPPORTED: fn SetToWeekDayInSameWeek()
    // NOT_SUPPORTED: fn SetToYearDay()
    fn get_jdn(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetJDN(self.as_ptr()) }
    }
    fn get_julian_day_number(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetJulianDayNumber(self.as_ptr()) }
    }
    fn get_mjd(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetMJD(self.as_ptr()) }
    }
    fn get_modified_julian_day_number(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetModifiedJulianDayNumber(self.as_ptr()) }
    }
    fn get_rata_die(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetRataDie(self.as_ptr()) }
    }
    fn from_timezone(&self, tz: *const c_void, no_dst: bool) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_FromTimezone(self.as_ptr(), tz, no_dst)) }
    }
    // NOT_SUPPORTED: fn IsDST()
    fn make_from_timezone(&self, tz: *const c_void, no_dst: bool) -> &Self {
        unsafe {
            ffi::wxDateTime_MakeFromTimezone(self.as_ptr(), tz, no_dst);
            &self
        }
    }
    fn make_timezone(&self, tz: *const c_void, no_dst: bool) -> &Self {
        unsafe {
            ffi::wxDateTime_MakeTimezone(self.as_ptr(), tz, no_dst);
            &self
        }
    }
    fn make_utc(&self, no_dst: bool) -> &Self {
        unsafe {
            ffi::wxDateTime_MakeUTC(self.as_ptr(), no_dst);
            &self
        }
    }
    fn to_timezone(&self, tz: *const c_void, no_dst: bool) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_ToTimezone(self.as_ptr(), tz, no_dst)) }
    }
    fn to_utc(&self, no_dst: bool) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_ToUTC(self.as_ptr(), no_dst)) }
    }
    fn convert_year_to_bc(year: c_int) -> c_int {
        unsafe { ffi::wxDateTime_ConvertYearToBC(year) }
    }
    fn get_am_pm_strings(am: *mut c_void, pm: *mut c_void) {
        unsafe { ffi::wxDateTime_GetAmPmStrings(am, pm) }
    }
    // NOT_SUPPORTED: fn GetBeginDST()
    // NOT_SUPPORTED: fn GetEndDST()
    fn get_century_int(year: c_int) -> c_int {
        unsafe { ffi::wxDateTime_GetCentury1(year) }
    }
    // NOT_SUPPORTED: fn GetCountry()
    // NOT_SUPPORTED: fn GetCurrentMonth()
    // NOT_SUPPORTED: fn GetCurrentYear()
    // NOT_SUPPORTED: fn GetEnglishMonthName()
    // NOT_SUPPORTED: fn GetEnglishWeekDayName()
    // NOT_SUPPORTED: fn GetMonthName()
    // NOT_SUPPORTED: fn GetNumberOfDays()
    // NOT_SUPPORTED: fn GetNumberOfDays1()
    // NOT_SUPPORTED: fn GetTimeNow()
    // BLOCKED: fn GetTmNow()
    fn get_tm_now() -> *mut c_void {
        unsafe { ffi::wxDateTime_GetTmNow1() }
    }
    // NOT_SUPPORTED: fn GetWeekDayName()
    // NOT_SUPPORTED: fn IsDSTApplicable()
    fn get_first_week_day(first_day: *mut c_void) -> bool {
        unsafe { ffi::wxDateTime_GetFirstWeekDay(first_day) }
    }
    // NOT_SUPPORTED: fn IsLeapYear()
    // NOT_SUPPORTED: fn IsWestEuropeanCountry()
    fn now() -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_Now()) }
    }
    // NOT_SUPPORTED: fn SetCountry()
    // NOT_SUPPORTED: fn SetToWeekOfYear()
    fn today() -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_Today()) }
    }
    fn u_now() -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_UNow()) }
    }
}

// wxDateTimeHolidayAuthority
pub trait DateTimeHolidayAuthorityMethods: WxRustMethods {}

// wxDateTimeWorkDays
pub trait DateTimeWorkDaysMethods: WxRustMethods {}

// wxDebugContext
pub trait DebugContextMethods: WxRustMethods {
    fn check(check_all: bool) -> c_int {
        unsafe { ffi::wxDebugContext_Check(check_all) }
    }
    fn dump() -> bool {
        unsafe { ffi::wxDebugContext_Dump() }
    }
    fn get_check_previous() -> bool {
        unsafe { ffi::wxDebugContext_GetCheckPrevious() }
    }
    fn get_debug_mode() -> bool {
        unsafe { ffi::wxDebugContext_GetDebugMode() }
    }
    fn get_level() -> c_int {
        unsafe { ffi::wxDebugContext_GetLevel() }
    }
    fn print_classes() -> bool {
        unsafe { ffi::wxDebugContext_PrintClasses() }
    }
    fn print_statistics(detailed: bool) -> bool {
        unsafe { ffi::wxDebugContext_PrintStatistics(detailed) }
    }
    fn set_check_previous(check: bool) {
        unsafe { ffi::wxDebugContext_SetCheckPrevious(check) }
    }
    fn set_checkpoint(all: bool) {
        unsafe { ffi::wxDebugContext_SetCheckpoint(all) }
    }
    fn set_debug_mode(debug: bool) {
        unsafe { ffi::wxDebugContext_SetDebugMode(debug) }
    }
    fn set_level(level: c_int) {
        unsafe { ffi::wxDebugContext_SetLevel(level) }
    }
    // NOT_SUPPORTED: fn SetShutdownNotifyFunction()
}

// wxDir
pub trait DirMethods: WxRustMethods {
    // DTOR: fn ~wxDir()
    fn close(&self) {
        unsafe { ffi::wxDir_Close(self.as_ptr()) }
    }
    fn get_first(&self, filename: *mut c_void, filespec: &str, flags: c_int) -> bool {
        unsafe {
            let filespec = WxString::from(filespec);
            let filespec = filespec.as_ptr();
            ffi::wxDir_GetFirst(self.as_ptr(), filename, filespec, flags)
        }
    }
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDir_GetName(self.as_ptr())).into() }
    }
    fn get_name_with_sep(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDir_GetNameWithSep(self.as_ptr())).into() }
    }
    fn get_next(&self, filename: *mut c_void) -> bool {
        unsafe { ffi::wxDir_GetNext(self.as_ptr(), filename) }
    }
    fn has_files(&self, filespec: &str) -> bool {
        unsafe {
            let filespec = WxString::from(filespec);
            let filespec = filespec.as_ptr();
            ffi::wxDir_HasFiles(self.as_ptr(), filespec)
        }
    }
    fn has_sub_dirs(&self, dirspec: &str) -> bool {
        unsafe {
            let dirspec = WxString::from(dirspec);
            let dirspec = dirspec.as_ptr();
            ffi::wxDir_HasSubDirs(self.as_ptr(), dirspec)
        }
    }
    fn is_opened(&self) -> bool {
        unsafe { ffi::wxDir_IsOpened(self.as_ptr()) }
    }
    fn open(&self, dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxDir_Open(self.as_ptr(), dir)
        }
    }
    fn traverse<D: DirTraverserMethods>(&self, sink: &D, filespec: &str, flags: c_int) -> usize {
        unsafe {
            let sink = sink.as_ptr();
            let filespec = WxString::from(filespec);
            let filespec = filespec.as_ptr();
            ffi::wxDir_Traverse(self.as_ptr(), sink, filespec, flags)
        }
    }
    fn exists(dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxDir_Exists(dir)
        }
    }
    fn find_first(dirname: &str, filespec: &str, flags: c_int) -> String {
        unsafe {
            let dirname = WxString::from(dirname);
            let dirname = dirname.as_ptr();
            let filespec = WxString::from(filespec);
            let filespec = filespec.as_ptr();
            WxString::from_ptr(ffi::wxDir_FindFirst(dirname, filespec, flags)).into()
        }
    }
    fn get_all_files<A: ArrayStringMethods>(
        dirname: &str,
        files: Option<&A>,
        filespec: &str,
        flags: c_int,
    ) -> usize {
        unsafe {
            let dirname = WxString::from(dirname);
            let dirname = dirname.as_ptr();
            let files = match files {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let filespec = WxString::from(filespec);
            let filespec = filespec.as_ptr();
            ffi::wxDir_GetAllFiles(dirname, files, filespec, flags)
        }
    }
    // NOT_SUPPORTED: fn GetTotalSize()
    fn make(dir: &str, perm: c_int, flags: c_int) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxDir_Make(dir, perm, flags)
        }
    }
    fn remove(dir: &str, flags: c_int) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxDir_Remove(dir, flags)
        }
    }
}

// wxDirTraverser
pub trait DirTraverserMethods: WxRustMethods {
    // NOT_SUPPORTED: fn OnDir()
    // NOT_SUPPORTED: fn OnFile()
    // NOT_SUPPORTED: fn OnOpenError()
}

// wxDynamicLibrary
pub trait DynamicLibraryMethods: WxRustMethods {
    // NOT_SUPPORTED: fn Attach()
    // NOT_SUPPORTED: fn Detach()
    fn get_symbol(&self, name: &str, success: *mut c_void) -> *mut c_void {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxDynamicLibrary_GetSymbol(self.as_ptr(), name, success)
        }
    }
    fn get_symbol_aor_w(&self, name: &str) -> *mut c_void {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxDynamicLibrary_GetSymbolAorW(self.as_ptr(), name)
        }
    }
    fn has_symbol(&self, name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxDynamicLibrary_HasSymbol(self.as_ptr(), name)
        }
    }
    fn is_loaded(&self) -> bool {
        unsafe { ffi::wxDynamicLibrary_IsLoaded(self.as_ptr()) }
    }
    fn load(&self, name: &str, flags: c_int) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxDynamicLibrary_Load(self.as_ptr(), name, flags)
        }
    }
    fn unload(&self) {
        unsafe { ffi::wxDynamicLibrary_Unload(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetDllExt()
    // NOT_SUPPORTED: fn CanonicalizeName()
    // NOT_SUPPORTED: fn CanonicalizePluginName()
    // NOT_SUPPORTED: fn GetProgramHandle()
    // NOT_SUPPORTED: fn ListLoaded()
    fn get_module_from_address(addr: *const c_void, path: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxDynamicLibrary_GetModuleFromAddress(addr, path) }
    }
    // NOT_SUPPORTED: fn Unload1()
}

// wxDynamicLibraryDetails
pub trait DynamicLibraryDetailsMethods: WxRustMethods {
    fn get_address(&self, addr: *mut c_void, len: *mut c_void) -> bool {
        unsafe { ffi::wxDynamicLibraryDetails_GetAddress(self.as_ptr(), addr, len) }
    }
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDynamicLibraryDetails_GetName(self.as_ptr())).into() }
    }
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDynamicLibraryDetails_GetPath(self.as_ptr())).into() }
    }
    fn get_version(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDynamicLibraryDetails_GetVersion(self.as_ptr())).into() }
    }
}

// wxEncodingConverter
pub trait EncodingConverterMethods: ObjectMethods {
    fn convert_char_char(&self, input: *const c_void, output: *mut c_void) -> bool {
        unsafe { ffi::wxEncodingConverter_Convert(self.as_ptr(), input, output) }
    }
    fn convert_wchar_t_wchar_t(&self, input: *const c_void, output: *mut c_void) -> bool {
        unsafe { ffi::wxEncodingConverter_Convert1(self.as_ptr(), input, output) }
    }
    fn convert_char_wchar_t(&self, input: *const c_void, output: *mut c_void) -> bool {
        unsafe { ffi::wxEncodingConverter_Convert2(self.as_ptr(), input, output) }
    }
    fn convert_wchar_t_char(&self, input: *const c_void, output: *mut c_void) -> bool {
        unsafe { ffi::wxEncodingConverter_Convert3(self.as_ptr(), input, output) }
    }
    fn convert_char(&self, str: *mut c_void) -> bool {
        unsafe { ffi::wxEncodingConverter_Convert4(self.as_ptr(), str) }
    }
    fn convert_wchar_t(&self, str: *mut c_void) -> bool {
        unsafe { ffi::wxEncodingConverter_Convert5(self.as_ptr(), str) }
    }
    fn convert_str(&self, input: &str) -> String {
        unsafe {
            let input = WxString::from(input);
            let input = input.as_ptr();
            WxString::from_ptr(ffi::wxEncodingConverter_Convert6(self.as_ptr(), input)).into()
        }
    }
    // NOT_SUPPORTED: fn Init()
    // NOT_SUPPORTED: fn GetAllEquivalents()
    // NOT_SUPPORTED: fn GetPlatformEquivalents()
    // NOT_SUPPORTED: fn CanConvert()
}

// wxEvent
pub trait EventMethods: ObjectMethods {
    fn clone(&self) -> Event {
        unsafe { Event::from_ptr(ffi::wxEvent_Clone(self.as_ptr())) }
    }
    fn get_event_object(&self) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxEvent_GetEventObject(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetEventType()
    // NOT_SUPPORTED: fn GetEventCategory()
    fn get_id(&self) -> c_int {
        unsafe { ffi::wxEvent_GetId(self.as_ptr()) }
    }
    fn get_event_user_data(&self) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxEvent_GetEventUserData(self.as_ptr())) }
    }
    fn get_skipped(&self) -> bool {
        unsafe { ffi::wxEvent_GetSkipped(self.as_ptr()) }
    }
    fn get_timestamp(&self) -> c_long {
        unsafe { ffi::wxEvent_GetTimestamp(self.as_ptr()) }
    }
    fn is_command_event(&self) -> bool {
        unsafe { ffi::wxEvent_IsCommandEvent(self.as_ptr()) }
    }
    fn resume_propagation(&self, propagation_level: c_int) {
        unsafe { ffi::wxEvent_ResumePropagation(self.as_ptr(), propagation_level) }
    }
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
    fn set_id(&self, id: c_int) {
        unsafe { ffi::wxEvent_SetId(self.as_ptr(), id) }
    }
    fn set_timestamp(&self, time_stamp: c_long) {
        unsafe { ffi::wxEvent_SetTimestamp(self.as_ptr(), time_stamp) }
    }
    fn should_propagate(&self) -> bool {
        unsafe { ffi::wxEvent_ShouldPropagate(self.as_ptr()) }
    }
    fn skip(&self, skip: bool) {
        unsafe { ffi::wxEvent_Skip(self.as_ptr(), skip) }
    }
    fn stop_propagation(&self) -> c_int {
        unsafe { ffi::wxEvent_StopPropagation(self.as_ptr()) }
    }
}

// wxEvtHandler
pub trait EvtHandlerMethods: ObjectMethods {
    fn queue_event<E: EventMethods>(&self, event: Option<&E>) {
        unsafe {
            let event = match event {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_QueueEvent(self.as_ptr(), event)
        }
    }
    fn add_pending_event<E: EventMethods>(&self, event: &E) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxEvtHandler_AddPendingEvent(self.as_ptr(), event)
        }
    }
    // NOT_SUPPORTED: fn CallAfter()
    // BLOCKED: fn CallAfter1()
    fn process_event<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxEvtHandler_ProcessEvent(self.as_ptr(), event)
        }
    }
    fn process_event_locally<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxEvtHandler_ProcessEventLocally(self.as_ptr(), event)
        }
    }
    fn safely_process_event<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxEvtHandler_SafelyProcessEvent(self.as_ptr(), event)
        }
    }
    fn process_pending_events(&self) {
        unsafe { ffi::wxEvtHandler_ProcessPendingEvents(self.as_ptr()) }
    }
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
    fn get_client_object(&self) -> Option<ClientDataIsOwned<false>> {
        unsafe { ClientData::option_from(ffi::wxEvtHandler_GetClientObject(self.as_ptr())) }
    }
    // BLOCKED: fn SetClientData()
    fn set_client_object<C: ClientDataMethods>(&self, data: Option<&C>) {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_SetClientObject(self.as_ptr(), data)
        }
    }
    fn get_evt_handler_enabled(&self) -> bool {
        unsafe { ffi::wxEvtHandler_GetEvtHandlerEnabled(self.as_ptr()) }
    }
    fn get_next_handler(&self) -> WeakRef<EvtHandler> {
        unsafe { WeakRef::<EvtHandler>::from(ffi::wxEvtHandler_GetNextHandler(self.as_ptr())) }
    }
    fn get_previous_handler(&self) -> WeakRef<EvtHandler> {
        unsafe { WeakRef::<EvtHandler>::from(ffi::wxEvtHandler_GetPreviousHandler(self.as_ptr())) }
    }
    fn set_evt_handler_enabled(&self, enabled: bool) {
        unsafe { ffi::wxEvtHandler_SetEvtHandlerEnabled(self.as_ptr(), enabled) }
    }
    fn set_next_handler<E: EvtHandlerMethods>(&self, handler: Option<&E>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_SetNextHandler(self.as_ptr(), handler)
        }
    }
    fn set_previous_handler<E: EvtHandlerMethods>(&self, handler: Option<&E>) {
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
    fn add_filter<E: EventFilterMethods>(filter: Option<&E>) {
        unsafe {
            let filter = match filter {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_AddFilter(filter)
        }
    }
    fn remove_filter<E: EventFilterMethods>(filter: Option<&E>) {
        unsafe {
            let filter = match filter {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_RemoveFilter(filter)
        }
    }
    // DTOR: fn ~wxEvtHandler()
}

// wxFFile
pub trait FFileMethods: WxRustMethods {
    // DTOR: fn ~wxFFile()
    fn attach(&self, fp: *mut c_void, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxFFile_Attach(self.as_ptr(), fp, name)
        }
    }
    fn close(&self) -> bool {
        unsafe { ffi::wxFFile_Close(self.as_ptr()) }
    }
    fn detach(&self) -> *mut c_void {
        unsafe { ffi::wxFFile_Detach(self.as_ptr()) }
    }
    fn eof(&self) -> bool {
        unsafe { ffi::wxFFile_Eof(self.as_ptr()) }
    }
    fn error(&self) -> bool {
        unsafe { ffi::wxFFile_Error(self.as_ptr()) }
    }
    fn flush(&self) -> bool {
        unsafe { ffi::wxFFile_Flush(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetKind()
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFFile_GetName(self.as_ptr())).into() }
    }
    fn is_opened(&self) -> bool {
        unsafe { ffi::wxFFile_IsOpened(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Length()
    fn open(&self, filename: &str, mode: &str) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            let mode = WxString::from(mode);
            let mode = mode.as_ptr();
            ffi::wxFFile_Open(self.as_ptr(), filename, mode)
        }
    }
    fn read(&self, buffer: *mut c_void, count: usize) -> usize {
        unsafe { ffi::wxFFile_Read(self.as_ptr(), buffer, count) }
    }
    fn read_all(&self, str: *mut c_void, conv: *const c_void) -> bool {
        unsafe { ffi::wxFFile_ReadAll(self.as_ptr(), str, conv) }
    }
    // NOT_SUPPORTED: fn Seek()
    // NOT_SUPPORTED: fn SeekEnd()
    // NOT_SUPPORTED: fn Tell()
    fn write_str(&self, str: &str, conv: *const c_void) -> bool {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxFFile_Write(self.as_ptr(), str, conv)
        }
    }
    fn write_void(&self, buffer: *const c_void, count: usize) -> usize {
        unsafe { ffi::wxFFile_Write1(self.as_ptr(), buffer, count) }
    }
    fn fp(&self) -> *mut c_void {
        unsafe { ffi::wxFFile_fp(self.as_ptr()) }
    }
}

// wxFSFile
pub trait FSFileMethods: ObjectMethods {
    fn detach_stream(&self) -> *mut c_void {
        unsafe { ffi::wxFSFile_DetachStream(self.as_ptr()) }
    }
    fn get_anchor(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFSFile_GetAnchor(self.as_ptr())).into() }
    }
    fn get_location(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFSFile_GetLocation(self.as_ptr())).into() }
    }
    fn get_mime_type(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFSFile_GetMimeType(self.as_ptr())).into() }
    }
    fn get_modification_time(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxFSFile_GetModificationTime(self.as_ptr())) }
    }
    fn get_stream(&self) -> *mut c_void {
        unsafe { ffi::wxFSFile_GetStream(self.as_ptr()) }
    }
}

// wxFSVolume
pub trait FSVolumeMethods: WxRustMethods {
    fn create(&self, name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxFSVolume_Create(self.as_ptr(), name)
        }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxFSVolume_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetKind()
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxFSVolume_GetFlags(self.as_ptr()) }
    }
    fn is_writable(&self) -> bool {
        unsafe { ffi::wxFSVolume_IsWritable(self.as_ptr()) }
    }
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFSVolume_GetName(self.as_ptr())).into() }
    }
    fn get_display_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFSVolume_GetDisplayName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetIcon()
    fn get_volumes(flags_set: c_int, flags_unset: c_int) -> ArrayString {
        unsafe { ArrayString::from_ptr(ffi::wxFSVolume_GetVolumes(flags_set, flags_unset)) }
    }
    fn cancel_search() {
        unsafe { ffi::wxFSVolume_CancelSearch() }
    }
}

// wxFile
pub trait FileMethods: WxRustMethods {
    // DTOR: fn ~wxFile()
    fn get_last_error(&self) -> c_int {
        unsafe { ffi::wxFile_GetLastError(self.as_ptr()) }
    }
    fn clear_last_error(&self) {
        unsafe { ffi::wxFile_ClearLastError(self.as_ptr()) }
    }
    fn attach(&self, fd: c_int) {
        unsafe { ffi::wxFile_Attach(self.as_ptr(), fd) }
    }
    fn close(&self) -> bool {
        unsafe { ffi::wxFile_Close(self.as_ptr()) }
    }
    fn create(&self, filename: &str, overwrite: bool, access: c_int) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxFile_Create(self.as_ptr(), filename, overwrite, access)
        }
    }
    fn detach(&self) -> c_int {
        unsafe { ffi::wxFile_Detach(self.as_ptr()) }
    }
    fn eof(&self) -> bool {
        unsafe { ffi::wxFile_Eof(self.as_ptr()) }
    }
    fn flush(&self) -> bool {
        unsafe { ffi::wxFile_Flush(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetKind()
    fn is_opened(&self) -> bool {
        unsafe { ffi::wxFile_IsOpened(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Length()
    // NOT_SUPPORTED: fn Open()
    // NOT_SUPPORTED: fn Read()
    fn read_all(&self, str: *mut c_void, conv: *const c_void) -> bool {
        unsafe { ffi::wxFile_ReadAll(self.as_ptr(), str, conv) }
    }
    // NOT_SUPPORTED: fn Seek()
    // NOT_SUPPORTED: fn SeekEnd()
    // NOT_SUPPORTED: fn Tell()
    fn write_void(&self, buffer: *const c_void, count: usize) -> usize {
        unsafe { ffi::wxFile_Write(self.as_ptr(), buffer, count) }
    }
    fn write_str(&self, s: &str, conv: *const c_void) -> bool {
        unsafe {
            let s = WxString::from(s);
            let s = s.as_ptr();
            ffi::wxFile_Write1(self.as_ptr(), s, conv)
        }
    }
    fn fd(&self) -> c_int {
        unsafe { ffi::wxFile_fd(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Access()
    fn exists(filename: &str) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxFile_Exists(filename)
        }
    }
}

// wxFileName
pub trait FileNameMethods: WxRustMethods {
    fn append_dir(&self, dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_AppendDir(self.as_ptr(), dir)
        }
    }
    fn assign<F: FileNameMethods>(&self, filepath: &F) {
        unsafe {
            let filepath = filepath.as_ptr();
            ffi::wxFileName_Assign(self.as_ptr(), filepath)
        }
    }
    // NOT_SUPPORTED: fn Assign1()
    // NOT_SUPPORTED: fn Assign2()
    // NOT_SUPPORTED: fn Assign3()
    // NOT_SUPPORTED: fn Assign4()
    // NOT_SUPPORTED: fn Assign5()
    fn assign_cwd(&self, volume: &str) {
        unsafe {
            let volume = WxString::from(volume);
            let volume = volume.as_ptr();
            ffi::wxFileName_AssignCwd(self.as_ptr(), volume)
        }
    }
    // NOT_SUPPORTED: fn AssignDir()
    fn assign_home_dir(&self) {
        unsafe { ffi::wxFileName_AssignHomeDir(self.as_ptr()) }
    }
    fn assign_temp_file_name(&self, prefix: &str) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxFileName_AssignTempFileName(self.as_ptr(), prefix)
        }
    }
    fn assign_temp_file_name_file<F: FileMethods>(&self, prefix: &str, file_temp: Option<&F>) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            let file_temp = match file_temp {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFileName_AssignTempFileName1(self.as_ptr(), prefix, file_temp)
        }
    }
    fn assign_temp_file_name_ffile<F: FFileMethods>(&self, prefix: &str, file_temp: Option<&F>) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            let file_temp = match file_temp {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFileName_AssignTempFileName2(self.as_ptr(), prefix, file_temp)
        }
    }
    fn clear(&self) {
        unsafe { ffi::wxFileName_Clear(self.as_ptr()) }
    }
    fn clear_ext(&self) {
        unsafe { ffi::wxFileName_ClearExt(self.as_ptr()) }
    }
    fn dir_exists(&self) -> bool {
        unsafe { ffi::wxFileName_DirExists(self.as_ptr()) }
    }
    fn dont_follow_link(&self) {
        unsafe { ffi::wxFileName_DontFollowLink(self.as_ptr()) }
    }
    fn exists_int(&self, flags: c_int) -> bool {
        unsafe { ffi::wxFileName_Exists(self.as_ptr(), flags) }
    }
    fn file_exists(&self) -> bool {
        unsafe { ffi::wxFileName_FileExists(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetAbsolutePath()
    fn get_dir_count(&self) -> usize {
        unsafe { ffi::wxFileName_GetDirCount(self.as_ptr()) }
    }
    fn get_dirs(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxFileName_GetDirs(self.as_ptr())) }
    }
    fn get_ext(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetExt(self.as_ptr())).into() }
    }
    fn get_full_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetFullName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetFullPath()
    // NOT_SUPPORTED: fn GetHumanReadableSize()
    fn get_long_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetLongPath(self.as_ptr())).into() }
    }
    fn get_modification_time(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxFileName_GetModificationTime(self.as_ptr())) }
    }
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetPath()
    // NOT_SUPPORTED: fn GetPathWithSep()
    fn get_short_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetShortPath(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetSize()
    fn get_times<D: DateTimeMethods, D2: DateTimeMethods, D3: DateTimeMethods>(
        &self,
        dt_access: Option<&D>,
        dt_mod: Option<&D2>,
        dt_create: Option<&D3>,
    ) -> bool {
        unsafe {
            let dt_access = match dt_access {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt_mod = match dt_mod {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt_create = match dt_create {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFileName_GetTimes(self.as_ptr(), dt_access, dt_mod, dt_create)
        }
    }
    fn get_volume(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetVolume(self.as_ptr())).into() }
    }
    fn has_ext(&self) -> bool {
        unsafe { ffi::wxFileName_HasExt(self.as_ptr()) }
    }
    fn has_name(&self) -> bool {
        unsafe { ffi::wxFileName_HasName(self.as_ptr()) }
    }
    fn has_volume(&self) -> bool {
        unsafe { ffi::wxFileName_HasVolume(self.as_ptr()) }
    }
    fn insert_dir(&self, before: usize, dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_InsertDir(self.as_ptr(), before, dir)
        }
    }
    // NOT_SUPPORTED: fn IsAbsolute()
    fn is_dir(&self) -> bool {
        unsafe { ffi::wxFileName_IsDir(self.as_ptr()) }
    }
    fn is_dir_readable(&self) -> bool {
        unsafe { ffi::wxFileName_IsDirReadable(self.as_ptr()) }
    }
    fn is_dir_writable(&self) -> bool {
        unsafe { ffi::wxFileName_IsDirWritable(self.as_ptr()) }
    }
    fn is_file_executable(&self) -> bool {
        unsafe { ffi::wxFileName_IsFileExecutable(self.as_ptr()) }
    }
    fn is_file_readable(&self) -> bool {
        unsafe { ffi::wxFileName_IsFileReadable(self.as_ptr()) }
    }
    fn is_file_writable(&self) -> bool {
        unsafe { ffi::wxFileName_IsFileWritable(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxFileName_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn IsRelative()
    // NOT_SUPPORTED: fn MakeAbsolute()
    // NOT_SUPPORTED: fn MakeRelativeTo()
    fn mkdir_int(&self, perm: c_int, flags: c_int) -> bool {
        unsafe { ffi::wxFileName_Mkdir(self.as_ptr(), perm, flags) }
    }
    // NOT_SUPPORTED: fn Normalize()
    fn prepend_dir(&self, dir: &str) {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_PrependDir(self.as_ptr(), dir)
        }
    }
    fn remove_dir(&self, pos: usize) {
        unsafe { ffi::wxFileName_RemoveDir(self.as_ptr(), pos) }
    }
    fn remove_last_dir(&self) {
        unsafe { ffi::wxFileName_RemoveLastDir(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn ReplaceEnvVariable()
    // NOT_SUPPORTED: fn ReplaceHomeDir()
    fn resolve_link(&self) -> FileName {
        unsafe { FileName::from_ptr(ffi::wxFileName_ResolveLink(self.as_ptr())) }
    }
    fn rmdir_int(&self, flags: c_int) -> bool {
        unsafe { ffi::wxFileName_Rmdir(self.as_ptr(), flags) }
    }
    // NOT_SUPPORTED: fn SameAs()
    fn set_cwd(&self) -> bool {
        unsafe { ffi::wxFileName_SetCwd(self.as_ptr()) }
    }
    fn set_empty_ext(&self) {
        unsafe { ffi::wxFileName_SetEmptyExt(self.as_ptr()) }
    }
    fn set_ext(&self, ext: &str) {
        unsafe {
            let ext = WxString::from(ext);
            let ext = ext.as_ptr();
            ffi::wxFileName_SetExt(self.as_ptr(), ext)
        }
    }
    fn set_full_name(&self, fullname: &str) {
        unsafe {
            let fullname = WxString::from(fullname);
            let fullname = fullname.as_ptr();
            ffi::wxFileName_SetFullName(self.as_ptr(), fullname)
        }
    }
    fn set_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxFileName_SetName(self.as_ptr(), name)
        }
    }
    // NOT_SUPPORTED: fn SetPath()
    fn set_permissions(&self, permissions: c_int) -> bool {
        unsafe { ffi::wxFileName_SetPermissions(self.as_ptr(), permissions) }
    }
    fn set_times<D: DateTimeMethods, D2: DateTimeMethods, D3: DateTimeMethods>(
        &self,
        dt_access: Option<&D>,
        dt_mod: Option<&D2>,
        dt_create: Option<&D3>,
    ) -> bool {
        unsafe {
            let dt_access = match dt_access {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt_mod = match dt_mod {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt_create = match dt_create {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFileName_SetTimes(self.as_ptr(), dt_access, dt_mod, dt_create)
        }
    }
    fn set_volume(&self, volume: &str) {
        unsafe {
            let volume = WxString::from(volume);
            let volume = volume.as_ptr();
            ffi::wxFileName_SetVolume(self.as_ptr(), volume)
        }
    }
    fn should_follow_link(&self) -> bool {
        unsafe { ffi::wxFileName_ShouldFollowLink(self.as_ptr()) }
    }
    fn touch(&self) -> bool {
        unsafe { ffi::wxFileName_Touch(self.as_ptr()) }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator!=1()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator==1()
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator=1()
    fn create_temp_file_name_file<F: FileMethods>(prefix: &str, file_temp: Option<&F>) -> String {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            let file_temp = match file_temp {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WxString::from_ptr(ffi::wxFileName_CreateTempFileName(prefix, file_temp)).into()
        }
    }
    fn create_temp_file_name_ffile<F: FFileMethods>(prefix: &str, file_temp: Option<&F>) -> String {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            let file_temp = match file_temp {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WxString::from_ptr(ffi::wxFileName_CreateTempFileName1(prefix, file_temp)).into()
        }
    }
    fn dir_exists_str(dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_DirExists1(dir)
        }
    }
    // NOT_SUPPORTED: fn DirName()
    fn exists_str(path: &str, flags: c_int) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxFileName_Exists1(path, flags)
        }
    }
    fn file_exists_str(file: &str) -> bool {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileName_FileExists1(file)
        }
    }
    // NOT_SUPPORTED: fn FileName()
    fn get_cwd(volume: &str) -> String {
        unsafe {
            let volume = WxString::from(volume);
            let volume = volume.as_ptr();
            WxString::from_ptr(ffi::wxFileName_GetCwd(volume)).into()
        }
    }
    // NOT_SUPPORTED: fn GetForbiddenChars()
    // NOT_SUPPORTED: fn GetFormat()
    fn get_home_dir() -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetHomeDir()).into() }
    }
    // NOT_SUPPORTED: fn GetHumanReadableSize1()
    // NOT_SUPPORTED: fn GetPathSeparator()
    // NOT_SUPPORTED: fn GetPathSeparators()
    // NOT_SUPPORTED: fn GetPathTerminators()
    // NOT_SUPPORTED: fn GetSize1()
    fn get_temp_dir() -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetTempDir()).into() }
    }
    // NOT_SUPPORTED: fn GetVolumeSeparator()
    // NOT_SUPPORTED: fn GetVolumeString()
    // NOT_SUPPORTED: fn IsCaseSensitive()
    fn is_dir_readable_str(dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_IsDirReadable1(dir)
        }
    }
    fn is_dir_writable_str(dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_IsDirWritable1(dir)
        }
    }
    fn is_file_executable_str(file: &str) -> bool {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileName_IsFileExecutable1(file)
        }
    }
    fn is_file_readable_str(file: &str) -> bool {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileName_IsFileReadable1(file)
        }
    }
    fn is_file_writable_str(file: &str) -> bool {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileName_IsFileWritable1(file)
        }
    }
    // NOT_SUPPORTED: fn IsPathSeparator()
    // NOT_SUPPORTED: fn IsMSWUniqueVolumeNamePath()
    fn mkdir_str(dir: &str, perm: c_int, flags: c_int) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_Mkdir1(dir, perm, flags)
        }
    }
    fn rmdir_str(dir: &str, flags: c_int) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_Rmdir1(dir, flags)
        }
    }
    fn set_cwd_str(cwd: &str) -> bool {
        unsafe {
            let cwd = WxString::from(cwd);
            let cwd = cwd.as_ptr();
            ffi::wxFileName_SetCwd1(cwd)
        }
    }
    fn url_to_file_name(url: &str) -> FileName {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            FileName::from_ptr(ffi::wxFileName_URLToFileName(url))
        }
    }
    fn file_name_to_url<F: FileNameMethods>(filename: &F) -> String {
        unsafe {
            let filename = filename.as_ptr();
            WxString::from_ptr(ffi::wxFileName_FileNameToURL(filename)).into()
        }
    }
    // NOT_SUPPORTED: fn SplitPath()
    // NOT_SUPPORTED: fn SplitPath1()
    // NOT_SUPPORTED: fn SplitPath2()
    // NOT_SUPPORTED: fn SplitVolume()
    fn strip_extension(fullname: &str) -> String {
        unsafe {
            let fullname = WxString::from(fullname);
            let fullname = fullname.as_ptr();
            WxString::from_ptr(ffi::wxFileName_StripExtension(fullname)).into()
        }
    }
}

// wxFileSystem
pub trait FileSystemMethods: ObjectMethods {
    fn change_path_to(&self, location: &str, is_dir: bool) {
        unsafe {
            let location = WxString::from(location);
            let location = location.as_ptr();
            ffi::wxFileSystem_ChangePathTo(self.as_ptr(), location, is_dir)
        }
    }
    fn find_file_in_path(&self, p_str: *mut c_void, path: &str, file: &str) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileSystem_FindFileInPath(self.as_ptr(), p_str, path, file)
        }
    }
    fn find_first(&self, wildcard: &str, flags: c_int) -> String {
        unsafe {
            let wildcard = WxString::from(wildcard);
            let wildcard = wildcard.as_ptr();
            WxString::from_ptr(ffi::wxFileSystem_FindFirst(self.as_ptr(), wildcard, flags)).into()
        }
    }
    fn find_next(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileSystem_FindNext(self.as_ptr())).into() }
    }
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileSystem_GetPath(self.as_ptr())).into() }
    }
    fn open_file(&self, location: &str, flags: c_int) -> Option<FSFileIsOwned<false>> {
        unsafe {
            let location = WxString::from(location);
            let location = location.as_ptr();
            FSFile::option_from(ffi::wxFileSystem_OpenFile(self.as_ptr(), location, flags))
        }
    }
    fn add_handler<F: FileSystemHandlerMethods>(handler: Option<&F>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFileSystem_AddHandler(handler)
        }
    }
    fn remove_handler<F: FileSystemHandlerMethods>(
        handler: Option<&F>,
    ) -> Option<FileSystemHandlerIsOwned<false>> {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            FileSystemHandler::option_from(ffi::wxFileSystem_RemoveHandler(handler))
        }
    }
    fn file_name_to_url<F: FileNameMethods>(filename: &F) -> String {
        unsafe {
            let filename = filename.as_ptr();
            WxString::from_ptr(ffi::wxFileSystem_FileNameToURL(filename)).into()
        }
    }
    fn has_handler_for_path(location: &str) -> bool {
        unsafe {
            let location = WxString::from(location);
            let location = location.as_ptr();
            ffi::wxFileSystem_HasHandlerForPath(location)
        }
    }
    fn url_to_file_name(url: &str) -> FileName {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            FileName::from_ptr(ffi::wxFileSystem_URLToFileName(url))
        }
    }
}

// wxFileSystemHandler
pub trait FileSystemHandlerMethods: ObjectMethods {
    fn can_open(&self, location: &str) -> bool {
        unsafe {
            let location = WxString::from(location);
            let location = location.as_ptr();
            ffi::wxFileSystemHandler_CanOpen(self.as_ptr(), location)
        }
    }
    fn find_first(&self, wildcard: &str, flags: c_int) -> String {
        unsafe {
            let wildcard = WxString::from(wildcard);
            let wildcard = wildcard.as_ptr();
            WxString::from_ptr(ffi::wxFileSystemHandler_FindFirst(
                self.as_ptr(),
                wildcard,
                flags,
            ))
            .into()
        }
    }
    fn find_next(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileSystemHandler_FindNext(self.as_ptr())).into() }
    }
    fn open_file<F: FileSystemMethods>(
        &self,
        fs: &F,
        location: &str,
    ) -> Option<FSFileIsOwned<false>> {
        unsafe {
            let fs = fs.as_ptr();
            let location = WxString::from(location);
            let location = location.as_ptr();
            FSFile::option_from(ffi::wxFileSystemHandler_OpenFile(
                self.as_ptr(),
                fs,
                location,
            ))
        }
    }
    fn get_mime_type_from_ext(location: &str) -> String {
        unsafe {
            let location = WxString::from(location);
            let location = location.as_ptr();
            WxString::from_ptr(ffi::wxFileSystemHandler_GetMimeTypeFromExt(location)).into()
        }
    }
}

// wxFileSystemWatcher
pub trait FileSystemWatcherMethods: EvtHandlerMethods {
    // DTOR: fn ~wxFileSystemWatcher()
    fn add<F: FileNameMethods>(&self, path: &F, events: c_int) -> bool {
        unsafe {
            let path = path.as_ptr();
            ffi::wxFileSystemWatcher_Add(self.as_ptr(), path, events)
        }
    }
    fn add_tree<F: FileNameMethods>(&self, path: &F, events: c_int, filter: &str) -> bool {
        unsafe {
            let path = path.as_ptr();
            let filter = WxString::from(filter);
            let filter = filter.as_ptr();
            ffi::wxFileSystemWatcher_AddTree(self.as_ptr(), path, events, filter)
        }
    }
    fn remove<F: FileNameMethods>(&self, path: &F) -> bool {
        unsafe {
            let path = path.as_ptr();
            ffi::wxFileSystemWatcher_Remove(self.as_ptr(), path)
        }
    }
    fn remove_tree<F: FileNameMethods>(&self, path: &F) -> bool {
        unsafe {
            let path = path.as_ptr();
            ffi::wxFileSystemWatcher_RemoveTree(self.as_ptr(), path)
        }
    }
    fn remove_all(&self) -> bool {
        unsafe { ffi::wxFileSystemWatcher_RemoveAll(self.as_ptr()) }
    }
    fn get_watched_paths_count(&self) -> c_int {
        unsafe { ffi::wxFileSystemWatcher_GetWatchedPathsCount(self.as_ptr()) }
    }
    fn get_watched_paths<A: ArrayStringMethods>(&self, paths: Option<&A>) -> c_int {
        unsafe {
            let paths = match paths {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFileSystemWatcher_GetWatchedPaths(self.as_ptr(), paths)
        }
    }
    fn set_owner<E: EvtHandlerMethods>(&self, handler: Option<&E>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFileSystemWatcher_SetOwner(self.as_ptr(), handler)
        }
    }
}

// wxFileSystemWatcherEvent
pub trait FileSystemWatcherEventMethods: EventMethods {
    fn get_path(&self) -> FileNameIsOwned<false> {
        unsafe { FileNameIsOwned::from_ptr(ffi::wxFileSystemWatcherEvent_GetPath(self.as_ptr())) }
    }
    fn get_new_path(&self) -> FileNameIsOwned<false> {
        unsafe {
            FileNameIsOwned::from_ptr(ffi::wxFileSystemWatcherEvent_GetNewPath(self.as_ptr()))
        }
    }
    fn get_change_type(&self) -> c_int {
        unsafe { ffi::wxFileSystemWatcherEvent_GetChangeType(self.as_ptr()) }
    }
    fn is_error(&self) -> bool {
        unsafe { ffi::wxFileSystemWatcherEvent_IsError(self.as_ptr()) }
    }
    fn get_error_description(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxFileSystemWatcherEvent_GetErrorDescription(
                self.as_ptr(),
            ))
            .into()
        }
    }
    // NOT_SUPPORTED: fn GetWarningType()
    fn to_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileSystemWatcherEvent_ToString(self.as_ptr())).into() }
    }
}

// wxFileType
pub trait FileTypeMethods: WxRustMethods {
    // DTOR: fn ~wxFileType()
    fn get_description(&self, desc: *mut c_void) -> bool {
        unsafe { ffi::wxFileType_GetDescription(self.as_ptr(), desc) }
    }
    fn get_extensions<A: ArrayStringMethods>(&self, extensions: &A) -> bool {
        unsafe {
            let extensions = extensions.as_ptr();
            ffi::wxFileType_GetExtensions(self.as_ptr(), extensions)
        }
    }
    fn get_icon<I: IconLocationMethods>(&self, icon_loc: Option<&I>) -> bool {
        unsafe {
            let icon_loc = match icon_loc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFileType_GetIcon(self.as_ptr(), icon_loc)
        }
    }
    fn get_mime_type(&self, mime_type: *mut c_void) -> bool {
        unsafe { ffi::wxFileType_GetMimeType(self.as_ptr(), mime_type) }
    }
    fn get_mime_types<A: ArrayStringMethods>(&self, mime_types: &A) -> bool {
        unsafe {
            let mime_types = mime_types.as_ptr();
            ffi::wxFileType_GetMimeTypes(self.as_ptr(), mime_types)
        }
    }
    fn get_open_command_str(&self, command: *mut c_void, params: *const c_void) -> bool {
        unsafe { ffi::wxFileType_GetOpenCommand(self.as_ptr(), command, params) }
    }
    fn get_open_command_str(&self, filename: &str) -> String {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            WxString::from_ptr(ffi::wxFileType_GetOpenCommand1(self.as_ptr(), filename)).into()
        }
    }
    fn get_print_command(&self, command: *mut c_void, params: *const c_void) -> bool {
        unsafe { ffi::wxFileType_GetPrintCommand(self.as_ptr(), command, params) }
    }
    fn get_expanded_command<F: FileType::MessageParametersMethods>(
        &self,
        verb: &str,
        params: &F,
    ) -> String {
        unsafe {
            let verb = WxString::from(verb);
            let verb = verb.as_ptr();
            let params = params.as_ptr();
            WxString::from_ptr(ffi::wxFileType_GetExpandedCommand(
                self.as_ptr(),
                verb,
                params,
            ))
            .into()
        }
    }
    fn get_all_commands<
        A: ArrayStringMethods,
        A2: ArrayStringMethods,
        F: FileType::MessageParametersMethods,
    >(
        &self,
        verbs: Option<&A>,
        commands: Option<&A2>,
        params: &F,
    ) -> usize {
        unsafe {
            let verbs = match verbs {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let commands = match commands {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let params = params.as_ptr();
            ffi::wxFileType_GetAllCommands(self.as_ptr(), verbs, commands, params)
        }
    }
    fn expand_command(command: &str, params: *const c_void) -> String {
        unsafe {
            let command = WxString::from(command);
            let command = command.as_ptr();
            WxString::from_ptr(ffi::wxFileType_ExpandCommand(command, params)).into()
        }
    }
}

// wxFilterClassFactory
pub trait FilterClassFactoryMethods: ObjectMethods {
    // NOT_SUPPORTED: fn CanHandle()
    fn get_next(&self) -> Option<FilterClassFactoryIsOwned<false>> {
        unsafe { FilterClassFactory::option_from(ffi::wxFilterClassFactory_GetNext(self.as_ptr())) }
    }
    fn get_protocol(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFilterClassFactory_GetProtocol(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetProtocols()
    fn new_stream_inputstream(&self, stream: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxFilterClassFactory_NewStream(self.as_ptr(), stream) }
    }
    fn new_stream_outputstream(&self, stream: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxFilterClassFactory_NewStream1(self.as_ptr(), stream) }
    }
    fn new_stream_inputstream(&self, stream: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxFilterClassFactory_NewStream2(self.as_ptr(), stream) }
    }
    fn new_stream_outputstream(&self, stream: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxFilterClassFactory_NewStream3(self.as_ptr(), stream) }
    }
    fn pop_extension(&self, location: &str) -> String {
        unsafe {
            let location = WxString::from(location);
            let location = location.as_ptr();
            WxString::from_ptr(ffi::wxFilterClassFactory_PopExtension(
                self.as_ptr(),
                location,
            ))
            .into()
        }
    }
    fn push_front(&self) {
        unsafe { ffi::wxFilterClassFactory_PushFront(self.as_ptr()) }
    }
    fn remove(&self) {
        unsafe { ffi::wxFilterClassFactory_Remove(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Find()
    fn get_first() -> Option<FilterClassFactoryIsOwned<false>> {
        unsafe { FilterClassFactory::option_from(ffi::wxFilterClassFactory_GetFirst()) }
    }
}

// wxHashMap
pub trait HashMapMethods: WxRustMethods {
    // NOT_SUPPORTED: fn begin()
    // NOT_SUPPORTED: fn begin1()
    fn clear(&self) {
        unsafe { ffi::wxHashMap_clear(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn count()
    fn empty(&self) -> bool {
        unsafe { ffi::wxHashMap_empty(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn end()
    // NOT_SUPPORTED: fn end1()
    // NOT_SUPPORTED: fn erase()
    // NOT_SUPPORTED: fn erase1()
    // NOT_SUPPORTED: fn erase2()
    // NOT_SUPPORTED: fn find()
    // NOT_SUPPORTED: fn find1()
    // NOT_SUPPORTED: fn insert()
    // NOT_SUPPORTED: fn operator[]()
    // NOT_SUPPORTED: fn size()
}

// wxHashSet
pub trait HashSetMethods: WxRustMethods {
    // NOT_SUPPORTED: fn begin()
    // NOT_SUPPORTED: fn begin1()
    fn clear(&self) {
        unsafe { ffi::wxHashSet_clear(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn count()
    fn empty(&self) -> bool {
        unsafe { ffi::wxHashSet_empty(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn end()
    // NOT_SUPPORTED: fn end1()
    // NOT_SUPPORTED: fn erase()
    // NOT_SUPPORTED: fn erase1()
    // NOT_SUPPORTED: fn erase2()
    // NOT_SUPPORTED: fn find()
    // NOT_SUPPORTED: fn find1()
    // NOT_SUPPORTED: fn insert()
    // NOT_SUPPORTED: fn size()
}

// wxHashTable
pub trait HashTableMethods: ObjectMethods {
    // DTOR: fn ~wxHashTable()
    fn begin_find(&self) {
        unsafe { ffi::wxHashTable_BeginFind(self.as_ptr()) }
    }
    fn clear(&self) {
        unsafe { ffi::wxHashTable_Clear(self.as_ptr()) }
    }
    fn delete_long(&self, key: c_long) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxHashTable_Delete(self.as_ptr(), key)) }
    }
    fn delete_str(&self, key: &str) -> Option<ObjectIsOwned<false>> {
        unsafe {
            let key = WxString::from(key);
            let key = key.as_ptr();
            Object::option_from(ffi::wxHashTable_Delete1(self.as_ptr(), key))
        }
    }
    fn delete_contents(&self, flag: bool) {
        unsafe { ffi::wxHashTable_DeleteContents(self.as_ptr(), flag) }
    }
    fn get_long(&self, key: c_long) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxHashTable_Get(self.as_ptr(), key)) }
    }
    fn get_char(&self, key: *const c_void) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxHashTable_Get1(self.as_ptr(), key)) }
    }
    fn get_count(&self) -> usize {
        unsafe { ffi::wxHashTable_GetCount(self.as_ptr()) }
    }
    fn next(&self) -> *mut c_void {
        unsafe { ffi::wxHashTable_Next(self.as_ptr()) }
    }
    fn put_long<O: ObjectMethods>(&self, key: c_long, object: Option<&O>) {
        unsafe {
            let object = match object {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxHashTable_Put(self.as_ptr(), key, object)
        }
    }
    fn put_char<O: ObjectMethods>(&self, key: *const c_void, object: Option<&O>) {
        unsafe {
            let object = match object {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxHashTable_Put1(self.as_ptr(), key, object)
        }
    }
    fn make_key(string: &str) -> c_long {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            ffi::wxHashTable_MakeKey(string)
        }
    }
}

// wxIconLocation
pub trait IconLocationMethods: WxRustMethods {
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxIconLocation_IsOk(self.as_ptr()) }
    }
    fn set_file_name(&self, filename: &str) {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxIconLocation_SetFileName(self.as_ptr(), filename)
        }
    }
    fn get_file_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxIconLocation_GetFileName(self.as_ptr())).into() }
    }
}

// wxIdleEvent
pub trait IdleEventMethods: EventMethods {
    fn more_requested(&self) -> bool {
        unsafe { ffi::wxIdleEvent_MoreRequested(self.as_ptr()) }
    }
    fn request_more(&self, need_more: bool) {
        unsafe { ffi::wxIdleEvent_RequestMore(self.as_ptr(), need_more) }
    }
    // NOT_SUPPORTED: fn GetMode()
    // NOT_SUPPORTED: fn SetMode()
}

// wxInitializer
pub trait InitializerMethods: WxRustMethods {
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxInitializer_IsOk(self.as_ptr()) }
    }
    // DTOR: fn ~wxInitializer()
}

// wxLocale
pub trait LocaleMethods: WxRustMethods {
    // DTOR: fn ~wxLocale()
    fn add_catalog(&self, domain: &str) -> bool {
        unsafe {
            let domain = WxString::from(domain);
            let domain = domain.as_ptr();
            ffi::wxLocale_AddCatalog(self.as_ptr(), domain)
        }
    }
    // NOT_SUPPORTED: fn AddCatalog1()
    // NOT_SUPPORTED: fn AddCatalog2()
    fn get_canonical_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxLocale_GetCanonicalName(self.as_ptr())).into() }
    }
    fn get_header_value(&self, header: &str, domain: &str) -> String {
        unsafe {
            let header = WxString::from(header);
            let header = header.as_ptr();
            let domain = WxString::from(domain);
            let domain = domain.as_ptr();
            WxString::from_ptr(ffi::wxLocale_GetHeaderValue(self.as_ptr(), header, domain)).into()
        }
    }
    fn get_language(&self) -> c_int {
        unsafe { ffi::wxLocale_GetLanguage(self.as_ptr()) }
    }
    fn get_locale(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxLocale_GetLocale(self.as_ptr())).into() }
    }
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxLocale_GetName(self.as_ptr())).into() }
    }
    fn get_string(&self, orig_string: &str, domain: &str) -> String {
        unsafe {
            let orig_string = WxString::from(orig_string);
            let orig_string = orig_string.as_ptr();
            let domain = WxString::from(domain);
            let domain = domain.as_ptr();
            WxString::from_ptr(ffi::wxLocale_GetString(self.as_ptr(), orig_string, domain)).into()
        }
    }
    // NOT_SUPPORTED: fn GetString1()
    fn get_sys_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxLocale_GetSysName(self.as_ptr())).into() }
    }
    fn init_int(&self, language: c_int, flags: c_int) -> bool {
        unsafe { ffi::wxLocale_Init(self.as_ptr(), language, flags) }
    }
    fn init_str(&self, name: &str, short_name: &str, locale: &str, b_load_default: bool) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let short_name = WxString::from(short_name);
            let short_name = short_name.as_ptr();
            let locale = WxString::from(locale);
            let locale = locale.as_ptr();
            ffi::wxLocale_Init1(self.as_ptr(), name, short_name, locale, b_load_default)
        }
    }
    fn is_loaded(&self, domain: &str) -> bool {
        unsafe {
            let domain = WxString::from(domain);
            let domain = domain.as_ptr();
            ffi::wxLocale_IsLoaded(self.as_ptr(), domain)
        }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxLocale_IsOk(self.as_ptr()) }
    }
    fn add_catalog_lookup_path_prefix(prefix: &str) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxLocale_AddCatalogLookupPathPrefix(prefix)
        }
    }
    fn add_language(info: *const c_void) {
        unsafe { ffi::wxLocale_AddLanguage(info) }
    }
    fn find_language_info(locale: &str) -> *const c_void {
        unsafe {
            let locale = WxString::from(locale);
            let locale = locale.as_ptr();
            ffi::wxLocale_FindLanguageInfo(locale)
        }
    }
    fn get_language_info(lang: c_int) -> *const c_void {
        unsafe { ffi::wxLocale_GetLanguageInfo(lang) }
    }
    fn get_language_name(lang: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxLocale_GetLanguageName(lang)).into() }
    }
    fn get_language_canonical_name(lang: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxLocale_GetLanguageCanonicalName(lang)).into() }
    }
    // NOT_SUPPORTED: fn GetSystemEncoding()
    fn get_system_encoding_name() -> String {
        unsafe { WxString::from_ptr(ffi::wxLocale_GetSystemEncodingName()).into() }
    }
    fn get_system_language() -> c_int {
        unsafe { ffi::wxLocale_GetSystemLanguage() }
    }
    // NOT_SUPPORTED: fn GetInfo()
    // NOT_SUPPORTED: fn GetOSInfo()
    fn is_available(lang: c_int) -> bool {
        unsafe { ffi::wxLocale_IsAvailable(lang) }
    }
}

// wxLog
pub trait LogMethods: WxRustMethods {
    fn add_trace_mask(mask: &str) {
        unsafe {
            let mask = WxString::from(mask);
            let mask = mask.as_ptr();
            ffi::wxLog_AddTraceMask(mask)
        }
    }
    fn clear_trace_masks() {
        unsafe { ffi::wxLog_ClearTraceMasks() }
    }
    fn get_trace_masks() -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxLog_GetTraceMasks()) }
    }
    fn is_allowed_trace_mask(mask: &str) -> bool {
        unsafe {
            let mask = WxString::from(mask);
            let mask = mask.as_ptr();
            ffi::wxLog_IsAllowedTraceMask(mask)
        }
    }
    fn remove_trace_mask(mask: &str) {
        unsafe {
            let mask = WxString::from(mask);
            let mask = mask.as_ptr();
            ffi::wxLog_RemoveTraceMask(mask)
        }
    }
    fn dont_create_on_demand() {
        unsafe { ffi::wxLog_DontCreateOnDemand() }
    }
    fn get_active_target() -> Option<LogIsOwned<false>> {
        unsafe { Log::option_from(ffi::wxLog_GetActiveTarget()) }
    }
    fn set_active_target<L: LogMethods>(logtarget: Option<&L>) -> Option<LogIsOwned<false>> {
        unsafe {
            let logtarget = match logtarget {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Log::option_from(ffi::wxLog_SetActiveTarget(logtarget))
        }
    }
    fn set_thread_active_target<L: LogMethods>(logger: Option<&L>) -> Option<LogIsOwned<false>> {
        unsafe {
            let logger = match logger {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Log::option_from(ffi::wxLog_SetThreadActiveTarget(logger))
        }
    }
    fn flush_active() {
        unsafe { ffi::wxLog_FlushActive() }
    }
    fn resume() {
        unsafe { ffi::wxLog_Resume() }
    }
    fn suspend() {
        unsafe { ffi::wxLog_Suspend() }
    }
    // NOT_SUPPORTED: fn GetLogLevel()
    // NOT_SUPPORTED: fn IsLevelEnabled()
    // NOT_SUPPORTED: fn SetComponentLevel()
    // NOT_SUPPORTED: fn SetLogLevel()
    fn enable_logging(enable: bool) -> bool {
        unsafe { ffi::wxLog_EnableLogging(enable) }
    }
    fn is_enabled() -> bool {
        unsafe { ffi::wxLog_IsEnabled() }
    }
    fn get_repetition_counting() -> bool {
        unsafe { ffi::wxLog_GetRepetitionCounting() }
    }
    fn set_repetition_counting(repet_counting: bool) {
        unsafe { ffi::wxLog_SetRepetitionCounting(repet_counting) }
    }
    fn get_timestamp() -> String {
        unsafe { WxString::from_ptr(ffi::wxLog_GetTimestamp()).into() }
    }
    fn set_timestamp(format: &str) {
        unsafe {
            let format = WxString::from(format);
            let format = format.as_ptr();
            ffi::wxLog_SetTimestamp(format)
        }
    }
    fn disable_timestamp() {
        unsafe { ffi::wxLog_DisableTimestamp() }
    }
    fn get_verbose() -> bool {
        unsafe { ffi::wxLog_GetVerbose() }
    }
    fn set_verbose(verbose: bool) {
        unsafe { ffi::wxLog_SetVerbose(verbose) }
    }
    fn set_formatter<L: LogFormatterMethods>(
        &self,
        formatter: Option<&L>,
    ) -> Option<LogFormatterIsOwned<false>> {
        unsafe {
            let formatter = match formatter {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            LogFormatter::option_from(ffi::wxLog_SetFormatter(self.as_ptr(), formatter))
        }
    }
    fn flush(&self) {
        unsafe { ffi::wxLog_Flush(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn LogRecord()
}

// wxLogBuffer
pub trait LogBufferMethods: LogMethods {
    fn get_buffer(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxLogBuffer_GetBuffer(self.as_ptr())).into() }
    }
}

// wxLogChain
pub trait LogChainMethods: LogMethods {
    // DTOR: fn ~wxLogChain()
    fn detach_old_log(&self) {
        unsafe { ffi::wxLogChain_DetachOldLog(self.as_ptr()) }
    }
    fn get_old_log(&self) -> Option<LogIsOwned<false>> {
        unsafe { Log::option_from(ffi::wxLogChain_GetOldLog(self.as_ptr())) }
    }
    fn is_passing_messages(&self) -> bool {
        unsafe { ffi::wxLogChain_IsPassingMessages(self.as_ptr()) }
    }
    fn pass_messages(&self, pass_messages: bool) {
        unsafe { ffi::wxLogChain_PassMessages(self.as_ptr(), pass_messages) }
    }
    fn set_log<L: LogMethods>(&self, logger: Option<&L>) {
        unsafe {
            let logger = match logger {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxLogChain_SetLog(self.as_ptr(), logger)
        }
    }
}

// wxLogFormatter
pub trait LogFormatterMethods: WxRustMethods {
    // NOT_SUPPORTED: fn Format()
}

// wxLogInterposer
pub trait LogInterposerMethods: LogChainMethods {}

// wxLogInterposerTemp
pub trait LogInterposerTempMethods: LogChainMethods {}

// wxLogNull
pub trait LogNullMethods: WxRustMethods {
    // DTOR: fn ~wxLogNull()
}

// wxLogStderr
pub trait LogStderrMethods: LogMethods {}

// wxLogStream
pub trait LogStreamMethods: LogMethods {}

// wxMemoryBuffer
pub trait MemoryBufferMethods: WxRustMethods {
    // NOT_SUPPORTED: fn AppendByte()
    fn append_data(&self, data: *const c_void, len: usize) {
        unsafe { ffi::wxMemoryBuffer_AppendData(self.as_ptr(), data, len) }
    }
    fn clear(&self) {
        unsafe { ffi::wxMemoryBuffer_Clear(self.as_ptr()) }
    }
    fn get_append_buf(&self, size_needed: usize) -> *mut c_void {
        unsafe { ffi::wxMemoryBuffer_GetAppendBuf(self.as_ptr(), size_needed) }
    }
    fn get_buf_size(&self) -> usize {
        unsafe { ffi::wxMemoryBuffer_GetBufSize(self.as_ptr()) }
    }
    fn get_data(&self) -> *mut c_void {
        unsafe { ffi::wxMemoryBuffer_GetData(self.as_ptr()) }
    }
    fn get_data_len(&self) -> usize {
        unsafe { ffi::wxMemoryBuffer_GetDataLen(self.as_ptr()) }
    }
    fn get_write_buf(&self, size_needed: usize) -> *mut c_void {
        unsafe { ffi::wxMemoryBuffer_GetWriteBuf(self.as_ptr(), size_needed) }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxMemoryBuffer_IsEmpty(self.as_ptr()) }
    }
    fn set_buf_size(&self, size: usize) {
        unsafe { ffi::wxMemoryBuffer_SetBufSize(self.as_ptr(), size) }
    }
    fn set_data_len(&self, size: usize) {
        unsafe { ffi::wxMemoryBuffer_SetDataLen(self.as_ptr(), size) }
    }
    fn unget_append_buf(&self, size_used: usize) {
        unsafe { ffi::wxMemoryBuffer_UngetAppendBuf(self.as_ptr(), size_used) }
    }
    fn unget_write_buf(&self, size_used: usize) {
        unsafe { ffi::wxMemoryBuffer_UngetWriteBuf(self.as_ptr(), size_used) }
    }
}

// wxMemoryFSHandler
pub trait MemoryFSHandlerMethods: FileSystemHandlerMethods {
    // NOT_SUPPORTED: fn AddFile()
    // NOT_SUPPORTED: fn AddFile1()
    fn add_file_str(filename: &str, textdata: &str) {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            let textdata = WxString::from(textdata);
            let textdata = textdata.as_ptr();
            ffi::wxMemoryFSHandler_AddFile2(filename, textdata)
        }
    }
    fn add_file_void(filename: &str, binarydata: *const c_void, size: usize) {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxMemoryFSHandler_AddFile3(filename, binarydata, size)
        }
    }
    fn add_file_with_mime_type_str(filename: &str, textdata: &str, mimetype: &str) {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            let textdata = WxString::from(textdata);
            let textdata = textdata.as_ptr();
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ffi::wxMemoryFSHandler_AddFileWithMimeType(filename, textdata, mimetype)
        }
    }
    fn add_file_with_mime_type_void(
        filename: &str,
        binarydata: *const c_void,
        size: usize,
        mimetype: &str,
    ) {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ffi::wxMemoryFSHandler_AddFileWithMimeType1(filename, binarydata, size, mimetype)
        }
    }
    fn remove_file(filename: &str) {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxMemoryFSHandler_RemoveFile(filename)
        }
    }
}

// wxMessageOutput
pub trait MessageOutputMethods: WxRustMethods {
    fn get() -> Option<MessageOutputIsOwned<false>> {
        unsafe { MessageOutput::option_from(ffi::wxMessageOutput_Get()) }
    }
    fn set<M: MessageOutputMethods>(msgout: Option<&M>) -> Option<MessageOutputIsOwned<false>> {
        unsafe {
            let msgout = match msgout {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MessageOutput::option_from(ffi::wxMessageOutput_Set(msgout))
        }
    }
    // NOT_SUPPORTED: fn Printf()
    fn output(&self, str: &str) {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxMessageOutput_Output(self.as_ptr(), str)
        }
    }
}

// wxMessageOutputBest
pub trait MessageOutputBestMethods: MessageOutputStderrMethods {}

// wxMessageOutputDebug
pub trait MessageOutputDebugMethods: MessageOutputStderrMethods {}

// wxMessageOutputStderr
pub trait MessageOutputStderrMethods: MessageOutputMethods {}

// wxMimeTypesManager
pub trait MimeTypesManagerMethods: WxRustMethods {
    // DTOR: fn ~wxMimeTypesManager()
    fn add_fallbacks<F: FileTypeInfoMethods>(&self, fallbacks: Option<&F>) {
        unsafe {
            let fallbacks = match fallbacks {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMimeTypesManager_AddFallbacks(self.as_ptr(), fallbacks)
        }
    }
    fn get_file_type_from_extension(&self, extension: &str) -> Option<FileTypeIsOwned<false>> {
        unsafe {
            let extension = WxString::from(extension);
            let extension = extension.as_ptr();
            FileType::option_from(ffi::wxMimeTypesManager_GetFileTypeFromExtension(
                self.as_ptr(),
                extension,
            ))
        }
    }
    fn get_file_type_from_mime_type(&self, mime_type: &str) -> Option<FileTypeIsOwned<false>> {
        unsafe {
            let mime_type = WxString::from(mime_type);
            let mime_type = mime_type.as_ptr();
            FileType::option_from(ffi::wxMimeTypesManager_GetFileTypeFromMimeType(
                self.as_ptr(),
                mime_type,
            ))
        }
    }
    fn associate<F: FileTypeInfoMethods>(&self, ft_info: &F) -> Option<FileTypeIsOwned<false>> {
        unsafe {
            let ft_info = ft_info.as_ptr();
            FileType::option_from(ffi::wxMimeTypesManager_Associate(self.as_ptr(), ft_info))
        }
    }
    fn unassociate<F: FileTypeMethods>(&self, ft: Option<&F>) -> bool {
        unsafe {
            let ft = match ft {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMimeTypesManager_Unassociate(self.as_ptr(), ft)
        }
    }
    fn enum_all_file_types<A: ArrayStringMethods>(&self, mimetypes: &A) -> usize {
        unsafe {
            let mimetypes = mimetypes.as_ptr();
            ffi::wxMimeTypesManager_EnumAllFileTypes(self.as_ptr(), mimetypes)
        }
    }
    fn is_of_type(mime_type: &str, wildcard: &str) -> bool {
        unsafe {
            let mime_type = WxString::from(mime_type);
            let mime_type = mime_type.as_ptr();
            let wildcard = WxString::from(wildcard);
            let wildcard = wildcard.as_ptr();
            ffi::wxMimeTypesManager_IsOfType(mime_type, wildcard)
        }
    }
}

// wxModule
pub trait ModuleMethods: ObjectMethods {
    // DTOR: fn ~wxModule()
    fn on_exit(&self) {
        unsafe { ffi::wxModule_OnExit(self.as_ptr()) }
    }
    fn on_init(&self) -> bool {
        unsafe { ffi::wxModule_OnInit(self.as_ptr()) }
    }
}

// wxMutex
pub trait MutexMethods: WxRustMethods {
    // DTOR: fn ~wxMutex()
    // NOT_SUPPORTED: fn Lock()
    // NOT_SUPPORTED: fn LockTimeout()
    // NOT_SUPPORTED: fn TryLock()
    // NOT_SUPPORTED: fn Unlock()
}

// wxMutexLocker
pub trait MutexLockerMethods: WxRustMethods {
    // DTOR: fn ~wxMutexLocker()
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxMutexLocker_IsOk(self.as_ptr()) }
    }
}

// wxObject
pub trait ObjectMethods: WxRustMethods {
    // DTOR: fn ~wxObject()
    fn get_class_info(&self) -> Option<ClassInfoIsOwned<false>> {
        unsafe { ClassInfo::option_from(ffi::wxObject_GetClassInfo(self.as_ptr())) }
    }
    fn get_ref_data(&self) -> Option<ObjectRefDataIsOwned<false>> {
        unsafe { ObjectRefData::option_from(ffi::wxObject_GetRefData(self.as_ptr())) }
    }
    fn is_kind_of<C: ClassInfoMethods>(&self, info: Option<&C>) -> bool {
        unsafe {
            let info = match info {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxObject_IsKindOf(self.as_ptr(), info)
        }
    }
    fn is_same_as<O: ObjectMethods>(&self, obj: &O) -> bool {
        unsafe {
            let obj = obj.as_ptr();
            ffi::wxObject_IsSameAs(self.as_ptr(), obj)
        }
    }
    fn ref_<O: ObjectMethods>(&self, clone: &O) {
        unsafe {
            let clone = clone.as_ptr();
            ffi::wxObject_Ref(self.as_ptr(), clone)
        }
    }
    fn set_ref_data<O: ObjectRefDataMethods>(&self, data: Option<&O>) {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxObject_SetRefData(self.as_ptr(), data)
        }
    }
    fn un_ref(&self) {
        unsafe { ffi::wxObject_UnRef(self.as_ptr()) }
    }
    fn un_share(&self) {
        unsafe { ffi::wxObject_UnShare(self.as_ptr()) }
    }
    // BLOCKED: fn operator delete()
    // BLOCKED: fn operator new()
}

// wxObjectRefData
pub trait ObjectRefDataMethods: WxRustMethods {}

// wxPlatformInfo
pub trait PlatformInfoMethods: WxRustMethods {
    // NOT_SUPPORTED: fn GetArchitecture()
    // NOT_SUPPORTED: fn GetBitness()
    // NOT_SUPPORTED: fn GetEndianness()
    fn get_cpu_architecture_name(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxPlatformInfo_GetCpuArchitectureName(self.as_ptr())).into()
        }
    }
    fn get_native_cpu_architecture_name(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxPlatformInfo_GetNativeCpuArchitectureName(
                self.as_ptr(),
            ))
            .into()
        }
    }
    fn get_os_major_version(&self) -> c_int {
        unsafe { ffi::wxPlatformInfo_GetOSMajorVersion(self.as_ptr()) }
    }
    fn get_os_minor_version(&self) -> c_int {
        unsafe { ffi::wxPlatformInfo_GetOSMinorVersion(self.as_ptr()) }
    }
    fn get_os_micro_version(&self) -> c_int {
        unsafe { ffi::wxPlatformInfo_GetOSMicroVersion(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetOperatingSystemId()
    fn get_operating_system_description(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxPlatformInfo_GetOperatingSystemDescription(
                self.as_ptr(),
            ))
            .into()
        }
    }
    // NOT_SUPPORTED: fn GetPortId()
    // NOT_SUPPORTED: fn GetLinuxDistributionInfo()
    fn get_desktop_environment(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxPlatformInfo_GetDesktopEnvironment(self.as_ptr())).into()
        }
    }
    fn get_toolkit_major_version(&self) -> c_int {
        unsafe { ffi::wxPlatformInfo_GetToolkitMajorVersion(self.as_ptr()) }
    }
    fn get_toolkit_minor_version(&self) -> c_int {
        unsafe { ffi::wxPlatformInfo_GetToolkitMinorVersion(self.as_ptr()) }
    }
    fn get_toolkit_micro_version(&self) -> c_int {
        unsafe { ffi::wxPlatformInfo_GetToolkitMicroVersion(self.as_ptr()) }
    }
    fn get_arch_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxPlatformInfo_GetArchName(self.as_ptr())).into() }
    }
    fn get_bitness_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxPlatformInfo_GetBitnessName(self.as_ptr())).into() }
    }
    fn get_endianness_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxPlatformInfo_GetEndiannessName(self.as_ptr())).into() }
    }
    fn get_operating_system_family_name(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxPlatformInfo_GetOperatingSystemFamilyName(
                self.as_ptr(),
            ))
            .into()
        }
    }
    fn get_operating_system_id_name(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxPlatformInfo_GetOperatingSystemIdName(self.as_ptr())).into()
        }
    }
    fn get_port_id_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxPlatformInfo_GetPortIdName(self.as_ptr())).into() }
    }
    fn get_port_id_short_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxPlatformInfo_GetPortIdShortName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn SetArchitecture()
    // NOT_SUPPORTED: fn SetBitness()
    // NOT_SUPPORTED: fn SetEndianness()
    fn set_os_version(&self, major: c_int, minor: c_int, micro: c_int) {
        unsafe { ffi::wxPlatformInfo_SetOSVersion(self.as_ptr(), major, minor, micro) }
    }
    // NOT_SUPPORTED: fn SetOperatingSystemId()
    // NOT_SUPPORTED: fn SetPortId()
    fn set_toolkit_version(&self, major: c_int, minor: c_int, micro: c_int) {
        unsafe { ffi::wxPlatformInfo_SetToolkitVersion(self.as_ptr(), major, minor, micro) }
    }
    fn set_operating_system_description(&self, desc: &str) {
        unsafe {
            let desc = WxString::from(desc);
            let desc = desc.as_ptr();
            ffi::wxPlatformInfo_SetOperatingSystemDescription(self.as_ptr(), desc)
        }
    }
    fn set_desktop_environment(&self, de: &str) {
        unsafe {
            let de = WxString::from(de);
            let de = de.as_ptr();
            ffi::wxPlatformInfo_SetDesktopEnvironment(self.as_ptr(), de)
        }
    }
    fn set_linux_distribution_info(&self, di: *const c_void) {
        unsafe { ffi::wxPlatformInfo_SetLinuxDistributionInfo(self.as_ptr(), di) }
    }
    // NOT_SUPPORTED: fn GetBitness1()
    // NOT_SUPPORTED: fn GetArch()
    // NOT_SUPPORTED: fn GetEndianness1()
    // NOT_SUPPORTED: fn GetOperatingSystemId1()
    // NOT_SUPPORTED: fn GetPortId1()
    // NOT_SUPPORTED: fn GetArchName1()
    // NOT_SUPPORTED: fn GetBitnessName1()
    // NOT_SUPPORTED: fn GetEndiannessName1()
    // NOT_SUPPORTED: fn GetOperatingSystemFamilyName1()
    // NOT_SUPPORTED: fn GetOperatingSystemIdName1()
    // NOT_SUPPORTED: fn GetPortIdName1()
    // NOT_SUPPORTED: fn GetPortIdShortName1()
    fn get_operating_system_directory() -> String {
        unsafe { WxString::from_ptr(ffi::wxPlatformInfo_GetOperatingSystemDirectory()).into() }
    }
    fn check_os_version(&self, major: c_int, minor: c_int, micro: c_int) -> bool {
        unsafe { ffi::wxPlatformInfo_CheckOSVersion(self.as_ptr(), major, minor, micro) }
    }
    fn check_toolkit_version(&self, major: c_int, minor: c_int, micro: c_int) -> bool {
        unsafe { ffi::wxPlatformInfo_CheckToolkitVersion(self.as_ptr(), major, minor, micro) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxPlatformInfo_IsOk(self.as_ptr()) }
    }
    fn is_using_universal_widgets(&self) -> bool {
        unsafe { ffi::wxPlatformInfo_IsUsingUniversalWidgets(self.as_ptr()) }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator==()
    fn get() -> PlatformInfoIsOwned<false> {
        unsafe { PlatformInfoIsOwned::from_ptr(ffi::wxPlatformInfo_Get()) }
    }
}

// wxPosition
pub trait PositionMethods: WxRustMethods {
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // BLOCKED: fn operator+=1()
    // BLOCKED: fn operator-=1()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator-()
    // BLOCKED: fn operator+1()
    // BLOCKED: fn operator-1()
    fn get_col(&self) -> c_int {
        unsafe { ffi::wxPosition_GetCol(self.as_ptr()) }
    }
    fn get_column(&self) -> c_int {
        unsafe { ffi::wxPosition_GetColumn(self.as_ptr()) }
    }
    fn get_row(&self) -> c_int {
        unsafe { ffi::wxPosition_GetRow(self.as_ptr()) }
    }
    fn set_col(&self, column: c_int) {
        unsafe { ffi::wxPosition_SetCol(self.as_ptr(), column) }
    }
    fn set_column(&self, column: c_int) {
        unsafe { ffi::wxPosition_SetColumn(self.as_ptr(), column) }
    }
    fn set_row(&self, row: c_int) {
        unsafe { ffi::wxPosition_SetRow(self.as_ptr(), row) }
    }
}

// wxPostScriptDC
pub trait PostScriptDCMethods: DCMethods {}

// wxPowerEvent
pub trait PowerEventMethods: EventMethods {
    fn veto(&self) {
        unsafe { ffi::wxPowerEvent_Veto(self.as_ptr()) }
    }
    fn is_vetoed(&self) -> bool {
        unsafe { ffi::wxPowerEvent_IsVetoed(self.as_ptr()) }
    }
}

// wxPowerResource
pub trait PowerResourceMethods: WxRustMethods {
    // NOT_SUPPORTED: fn Acquire()
    // NOT_SUPPORTED: fn Release()
}

// wxPowerResourceBlocker
pub trait PowerResourceBlockerMethods: WxRustMethods {
    fn is_in_effect(&self) -> bool {
        unsafe { ffi::wxPowerResourceBlocker_IsInEffect(self.as_ptr()) }
    }
    // DTOR: fn ~wxPowerResourceBlocker()
}

// wxProcess
pub trait ProcessMethods: EvtHandlerMethods {
    // DTOR: fn ~wxProcess()
    fn activate(&self) -> bool {
        unsafe { ffi::wxProcess_Activate(self.as_ptr()) }
    }
    fn close_output(&self) {
        unsafe { ffi::wxProcess_CloseOutput(self.as_ptr()) }
    }
    fn detach(&self) {
        unsafe { ffi::wxProcess_Detach(self.as_ptr()) }
    }
    fn get_error_stream(&self) -> *mut c_void {
        unsafe { ffi::wxProcess_GetErrorStream(self.as_ptr()) }
    }
    fn get_input_stream(&self) -> *mut c_void {
        unsafe { ffi::wxProcess_GetInputStream(self.as_ptr()) }
    }
    fn get_output_stream(&self) -> *mut c_void {
        unsafe { ffi::wxProcess_GetOutputStream(self.as_ptr()) }
    }
    fn get_pid(&self) -> c_long {
        unsafe { ffi::wxProcess_GetPid(self.as_ptr()) }
    }
    fn is_error_available(&self) -> bool {
        unsafe { ffi::wxProcess_IsErrorAvailable(self.as_ptr()) }
    }
    fn is_input_available(&self) -> bool {
        unsafe { ffi::wxProcess_IsInputAvailable(self.as_ptr()) }
    }
    fn is_input_opened(&self) -> bool {
        unsafe { ffi::wxProcess_IsInputOpened(self.as_ptr()) }
    }
    fn on_terminate(&self, pid: c_int, status: c_int) {
        unsafe { ffi::wxProcess_OnTerminate(self.as_ptr(), pid, status) }
    }
    fn redirect(&self) {
        unsafe { ffi::wxProcess_Redirect(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetPriority()
    fn exists(pid: c_int) -> bool {
        unsafe { ffi::wxProcess_Exists(pid) }
    }
    // NOT_SUPPORTED: fn Kill()
    fn open(cmd: &str, flags: c_int) -> WeakRef<Process> {
        unsafe {
            let cmd = WxString::from(cmd);
            let cmd = cmd.as_ptr();
            WeakRef::<Process>::from(ffi::wxProcess_Open(cmd, flags))
        }
    }
}

// wxProcessEvent
pub trait ProcessEventMethods: EventMethods {
    fn get_exit_code(&self) -> c_int {
        unsafe { ffi::wxProcessEvent_GetExitCode(self.as_ptr()) }
    }
    fn get_pid(&self) -> c_int {
        unsafe { ffi::wxProcessEvent_GetPid(self.as_ptr()) }
    }
}

// wxRecursionGuard
pub trait RecursionGuardMethods: WxRustMethods {
    // DTOR: fn ~wxRecursionGuard()
    fn is_inside(&self) -> bool {
        unsafe { ffi::wxRecursionGuard_IsInside(self.as_ptr()) }
    }
}

// wxRecursionGuardFlag
pub trait RecursionGuardFlagMethods: WxRustMethods {}

// wxRefCounter
pub trait RefCounterMethods: WxRustMethods {
    fn dec_ref(&self) {
        unsafe { ffi::wxRefCounter_DecRef(self.as_ptr()) }
    }
    fn get_ref_count(&self) -> c_int {
        unsafe { ffi::wxRefCounter_GetRefCount(self.as_ptr()) }
    }
    fn inc_ref(&self) {
        unsafe { ffi::wxRefCounter_IncRef(self.as_ptr()) }
    }
}

// wxRegEx
pub trait RegExMethods: WxRustMethods {
    // DTOR: fn ~wxRegEx()
    fn compile(&self, pattern: &str, flags: c_int) -> bool {
        unsafe {
            let pattern = WxString::from(pattern);
            let pattern = pattern.as_ptr();
            ffi::wxRegEx_Compile(self.as_ptr(), pattern, flags)
        }
    }
    fn get_match_sz(&self, start: *mut c_void, len: *mut c_void, index: usize) -> bool {
        unsafe { ffi::wxRegEx_GetMatch(self.as_ptr(), start, len, index) }
    }
    fn get_match_str(&self, text: &str, index: usize) -> String {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            WxString::from_ptr(ffi::wxRegEx_GetMatch1(self.as_ptr(), text, index)).into()
        }
    }
    fn get_match_count(&self) -> usize {
        unsafe { ffi::wxRegEx_GetMatchCount(self.as_ptr()) }
    }
    fn is_valid(&self) -> bool {
        unsafe { ffi::wxRegEx_IsValid(self.as_ptr()) }
    }
    fn matches_char(&self, text: *const c_void, flags: c_int) -> bool {
        unsafe { ffi::wxRegEx_Matches(self.as_ptr(), text, flags) }
    }
    fn matches_char_sz(&self, text: *const c_void, flags: c_int, len: usize) -> bool {
        unsafe { ffi::wxRegEx_Matches1(self.as_ptr(), text, flags, len) }
    }
    fn matches_str(&self, text: &str, flags: c_int) -> bool {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxRegEx_Matches2(self.as_ptr(), text, flags)
        }
    }
    fn replace(&self, text: *mut c_void, replacement: &str, max_matches: usize) -> c_int {
        unsafe {
            let replacement = WxString::from(replacement);
            let replacement = replacement.as_ptr();
            ffi::wxRegEx_Replace(self.as_ptr(), text, replacement, max_matches)
        }
    }
    fn replace_all(&self, text: *mut c_void, replacement: &str) -> c_int {
        unsafe {
            let replacement = WxString::from(replacement);
            let replacement = replacement.as_ptr();
            ffi::wxRegEx_ReplaceAll(self.as_ptr(), text, replacement)
        }
    }
    fn replace_first(&self, text: *mut c_void, replacement: &str) -> c_int {
        unsafe {
            let replacement = WxString::from(replacement);
            let replacement = replacement.as_ptr();
            ffi::wxRegEx_ReplaceFirst(self.as_ptr(), text, replacement)
        }
    }
    fn quote_meta(str: &str) -> String {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            WxString::from_ptr(ffi::wxRegEx_QuoteMeta(str)).into()
        }
    }
    fn convert_from_basic(bre: &str) -> String {
        unsafe {
            let bre = WxString::from(bre);
            let bre = bre.as_ptr();
            WxString::from_ptr(ffi::wxRegEx_ConvertFromBasic(bre)).into()
        }
    }
    fn get_library_version_info() -> VersionInfo {
        unsafe { VersionInfo::from_ptr(ffi::wxRegEx_GetLibraryVersionInfo()) }
    }
}

// wxRegKey
pub trait RegKeyMethods: WxRustMethods {
    fn close(&self) {
        unsafe { ffi::wxRegKey_Close(self.as_ptr()) }
    }
    fn copy_str(&self, sz_new_name: &str) -> bool {
        unsafe {
            let sz_new_name = WxString::from(sz_new_name);
            let sz_new_name = sz_new_name.as_ptr();
            ffi::wxRegKey_Copy(self.as_ptr(), sz_new_name)
        }
    }
    fn copy_regkey<R: RegKeyMethods>(&self, key_dst: &R) -> bool {
        unsafe {
            let key_dst = key_dst.as_ptr();
            ffi::wxRegKey_Copy1(self.as_ptr(), key_dst)
        }
    }
    fn copy_value<R: RegKeyMethods>(&self, sz_value: &str, key_dst: &R, sz_new_name: &str) -> bool {
        unsafe {
            let sz_value = WxString::from(sz_value);
            let sz_value = sz_value.as_ptr();
            let key_dst = key_dst.as_ptr();
            let sz_new_name = WxString::from(sz_new_name);
            let sz_new_name = sz_new_name.as_ptr();
            ffi::wxRegKey_CopyValue(self.as_ptr(), sz_value, key_dst, sz_new_name)
        }
    }
    fn create(&self, b_ok_if_exists: bool) -> bool {
        unsafe { ffi::wxRegKey_Create(self.as_ptr(), b_ok_if_exists) }
    }
    fn delete_key(&self, sz_key: &str) {
        unsafe {
            let sz_key = WxString::from(sz_key);
            let sz_key = sz_key.as_ptr();
            ffi::wxRegKey_DeleteKey(self.as_ptr(), sz_key)
        }
    }
    fn delete_self(&self) {
        unsafe { ffi::wxRegKey_DeleteSelf(self.as_ptr()) }
    }
    fn delete_value(&self, sz_key: &str) {
        unsafe {
            let sz_key = WxString::from(sz_key);
            let sz_key = sz_key.as_ptr();
            ffi::wxRegKey_DeleteValue(self.as_ptr(), sz_key)
        }
    }
    fn exists(&self) -> bool {
        unsafe { ffi::wxRegKey_Exists(self.as_ptr()) }
    }
    fn export_str(&self, filename: &str) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxRegKey_Export(self.as_ptr(), filename)
        }
    }
    fn export_outputstream(&self, ostr: *mut c_void) -> bool {
        unsafe { ffi::wxRegKey_Export1(self.as_ptr(), ostr) }
    }
    fn get_first_key(&self, str_key_name: *mut c_void, l_index: *mut c_void) -> bool {
        unsafe { ffi::wxRegKey_GetFirstKey(self.as_ptr(), str_key_name, l_index) }
    }
    fn get_first_value(&self, str_value_name: *mut c_void, l_index: *mut c_void) -> bool {
        unsafe { ffi::wxRegKey_GetFirstValue(self.as_ptr(), str_value_name, l_index) }
    }
    fn get_key_info(
        &self,
        pn_sub_keys: *mut c_void,
        pn_max_key_len: *mut c_void,
        pn_values: *mut c_void,
        pn_max_value_len: *mut c_void,
    ) -> bool {
        unsafe {
            ffi::wxRegKey_GetKeyInfo(
                self.as_ptr(),
                pn_sub_keys,
                pn_max_key_len,
                pn_values,
                pn_max_value_len,
            )
        }
    }
    fn get_name(&self, b_short_prefix: bool) -> String {
        unsafe { WxString::from_ptr(ffi::wxRegKey_GetName(self.as_ptr(), b_short_prefix)).into() }
    }
    // NOT_SUPPORTED: fn GetView()
    fn get_next_key(&self, str_key_name: *mut c_void, l_index: *mut c_void) -> bool {
        unsafe { ffi::wxRegKey_GetNextKey(self.as_ptr(), str_key_name, l_index) }
    }
    fn get_next_value(&self, str_value_name: *mut c_void, l_index: *mut c_void) -> bool {
        unsafe { ffi::wxRegKey_GetNextValue(self.as_ptr(), str_value_name, l_index) }
    }
    // NOT_SUPPORTED: fn GetValueType()
    fn has_sub_key(&self, sz_key: &str) -> bool {
        unsafe {
            let sz_key = WxString::from(sz_key);
            let sz_key = sz_key.as_ptr();
            ffi::wxRegKey_HasSubKey(self.as_ptr(), sz_key)
        }
    }
    fn has_subkeys(&self) -> bool {
        unsafe { ffi::wxRegKey_HasSubkeys(self.as_ptr()) }
    }
    fn has_value(&self, sz_value: &str) -> bool {
        unsafe {
            let sz_value = WxString::from(sz_value);
            let sz_value = sz_value.as_ptr();
            ffi::wxRegKey_HasValue(self.as_ptr(), sz_value)
        }
    }
    fn has_values(&self) -> bool {
        unsafe { ffi::wxRegKey_HasValues(self.as_ptr()) }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxRegKey_IsEmpty(self.as_ptr()) }
    }
    fn is_numeric_value(&self, sz_value: &str) -> bool {
        unsafe {
            let sz_value = WxString::from(sz_value);
            let sz_value = sz_value.as_ptr();
            ffi::wxRegKey_IsNumericValue(self.as_ptr(), sz_value)
        }
    }
    fn is_opened(&self) -> bool {
        unsafe { ffi::wxRegKey_IsOpened(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Open()
    // BLOCKED: fn operator=()
    fn query_default_value(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxRegKey_QueryDefaultValue(self.as_ptr())).into() }
    }
    fn query_raw_value(&self, sz_value: &str, str_value: *mut c_void) -> bool {
        unsafe {
            let sz_value = WxString::from(sz_value);
            let sz_value = sz_value.as_ptr();
            ffi::wxRegKey_QueryRawValue(self.as_ptr(), sz_value, str_value)
        }
    }
    fn query_value_str(&self, sz_value: &str, str_value: *mut c_void, raw: bool) -> bool {
        unsafe {
            let sz_value = WxString::from(sz_value);
            let sz_value = sz_value.as_ptr();
            ffi::wxRegKey_QueryValue(self.as_ptr(), sz_value, str_value, raw)
        }
    }
    fn query_value_long(&self, sz_value: &str, pl_value: *mut c_void) -> bool {
        unsafe {
            let sz_value = WxString::from(sz_value);
            let sz_value = sz_value.as_ptr();
            ffi::wxRegKey_QueryValue1(self.as_ptr(), sz_value, pl_value)
        }
    }
    fn query_value64(&self, sz_value: &str, pl_value: *mut c_void) -> bool {
        unsafe {
            let sz_value = WxString::from(sz_value);
            let sz_value = sz_value.as_ptr();
            ffi::wxRegKey_QueryValue64(self.as_ptr(), sz_value, pl_value)
        }
    }
    fn query_value_memorybuffer<M: MemoryBufferMethods>(&self, sz_value: &str, buf: &M) -> bool {
        unsafe {
            let sz_value = WxString::from(sz_value);
            let sz_value = sz_value.as_ptr();
            let buf = buf.as_ptr();
            ffi::wxRegKey_QueryValue2(self.as_ptr(), sz_value, buf)
        }
    }
    fn rename(&self, sz_new_name: &str) -> bool {
        unsafe {
            let sz_new_name = WxString::from(sz_new_name);
            let sz_new_name = sz_new_name.as_ptr();
            ffi::wxRegKey_Rename(self.as_ptr(), sz_new_name)
        }
    }
    fn rename_value(&self, sz_value_old: &str, sz_value_new: &str) -> bool {
        unsafe {
            let sz_value_old = WxString::from(sz_value_old);
            let sz_value_old = sz_value_old.as_ptr();
            let sz_value_new = WxString::from(sz_value_new);
            let sz_value_new = sz_value_new.as_ptr();
            ffi::wxRegKey_RenameValue(self.as_ptr(), sz_value_old, sz_value_new)
        }
    }
    fn reserve_memory_for_name(&self, bytes: usize) {
        unsafe { ffi::wxRegKey_ReserveMemoryForName(self.as_ptr(), bytes) }
    }
    // NOT_SUPPORTED: fn SetHkey()
    fn set_name_str(&self, str_key: &str) {
        unsafe {
            let str_key = WxString::from(str_key);
            let str_key = str_key.as_ptr();
            ffi::wxRegKey_SetName(self.as_ptr(), str_key)
        }
    }
    // NOT_SUPPORTED: fn SetName1()
    fn set_name_regkey<R: RegKeyMethods>(&self, key_parent: &R, str_key: &str) {
        unsafe {
            let key_parent = key_parent.as_ptr();
            let str_key = WxString::from(str_key);
            let str_key = str_key.as_ptr();
            ffi::wxRegKey_SetName2(self.as_ptr(), key_parent, str_key)
        }
    }
    fn set_value_long(&self, sz_value: &str, l_value: c_long) -> bool {
        unsafe {
            let sz_value = WxString::from(sz_value);
            let sz_value = sz_value.as_ptr();
            ffi::wxRegKey_SetValue(self.as_ptr(), sz_value, l_value)
        }
    }
    // NOT_SUPPORTED: fn SetValue64()
    fn set_value_str(&self, sz_value: &str, str_value: &str) -> bool {
        unsafe {
            let sz_value = WxString::from(sz_value);
            let sz_value = sz_value.as_ptr();
            let str_value = WxString::from(str_value);
            let str_value = str_value.as_ptr();
            ffi::wxRegKey_SetValue1(self.as_ptr(), sz_value, str_value)
        }
    }
    fn set_value_memorybuffer<M: MemoryBufferMethods>(&self, sz_value: &str, buf: &M) -> bool {
        unsafe {
            let sz_value = WxString::from(sz_value);
            let sz_value = sz_value.as_ptr();
            let buf = buf.as_ptr();
            ffi::wxRegKey_SetValue2(self.as_ptr(), sz_value, buf)
        }
    }
}

// wxSecretStore
pub trait SecretStoreMethods: WxRustMethods {
    fn get_default() -> SecretStore {
        unsafe { SecretStore::from_ptr(ffi::wxSecretStore_GetDefault()) }
    }
    fn is_ok(&self, errmsg: *mut c_void) -> bool {
        unsafe { ffi::wxSecretStore_IsOk(self.as_ptr(), errmsg) }
    }
    fn save<S: SecretValueMethods>(&self, service: &str, username: &str, password: &S) -> bool {
        unsafe {
            let service = WxString::from(service);
            let service = service.as_ptr();
            let username = WxString::from(username);
            let username = username.as_ptr();
            let password = password.as_ptr();
            ffi::wxSecretStore_Save(self.as_ptr(), service, username, password)
        }
    }
    fn load<S: SecretValueMethods>(
        &self,
        service: &str,
        username: *mut c_void,
        password: &S,
    ) -> bool {
        unsafe {
            let service = WxString::from(service);
            let service = service.as_ptr();
            let password = password.as_ptr();
            ffi::wxSecretStore_Load(self.as_ptr(), service, username, password)
        }
    }
    fn delete(&self, service: &str) -> bool {
        unsafe {
            let service = WxString::from(service);
            let service = service.as_ptr();
            ffi::wxSecretStore_Delete(self.as_ptr(), service)
        }
    }
}

// wxSecretValue
pub trait SecretValueMethods: WxRustMethods {
    // BLOCKED: fn operator=()
    // DTOR: fn ~wxSecretValue()
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxSecretValue_IsOk(self.as_ptr()) }
    }
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    fn get_size(&self) -> usize {
        unsafe { ffi::wxSecretValue_GetSize(self.as_ptr()) }
    }
    fn get_data(&self) -> *const c_void {
        unsafe { ffi::wxSecretValue_GetData(self.as_ptr()) }
    }
    fn get_as_string(&self, conv: *const c_void) -> String {
        unsafe { WxString::from_ptr(ffi::wxSecretValue_GetAsString(self.as_ptr(), conv)).into() }
    }
    fn wipe(size: usize, data: *mut c_void) {
        unsafe { ffi::wxSecretValue_Wipe(size, data) }
    }
    fn wipe_string(str: *mut c_void) {
        unsafe { ffi::wxSecretValue_WipeString(str) }
    }
}

// wxSemaphore
pub trait SemaphoreMethods: WxRustMethods {
    // DTOR: fn ~wxSemaphore()
    // NOT_SUPPORTED: fn Post()
    // NOT_SUPPORTED: fn TryWait()
    // NOT_SUPPORTED: fn Wait()
    // NOT_SUPPORTED: fn WaitTimeout()
}

// wxServer
pub trait ServerMethods: WxRustMethods {
    fn create(&self, service: &str) -> bool {
        unsafe {
            let service = WxString::from(service);
            let service = service.as_ptr();
            ffi::wxServer_Create(self.as_ptr(), service)
        }
    }
    fn on_accept_connection(&self, topic: &str) -> Option<ConnectionBaseIsOwned<false>> {
        unsafe {
            let topic = WxString::from(topic);
            let topic = topic.as_ptr();
            ConnectionBase::option_from(ffi::wxServer_OnAcceptConnection(self.as_ptr(), topic))
        }
    }
}

// wxSharedClientDataContainer
pub trait SharedClientDataContainerMethods: WxRustMethods {
    fn get_client_data(&self) -> *mut c_void {
        unsafe { ffi::wxSharedClientDataContainer_GetClientData(self.as_ptr()) }
    }
    fn get_client_object(&self) -> Option<ClientDataIsOwned<false>> {
        unsafe {
            ClientData::option_from(ffi::wxSharedClientDataContainer_GetClientObject(
                self.as_ptr(),
            ))
        }
    }
    fn set_client_data(&self, data: *mut c_void) {
        unsafe { ffi::wxSharedClientDataContainer_SetClientData(self.as_ptr(), data) }
    }
    fn set_client_object<C: ClientDataMethods>(&self, data: Option<&C>) {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSharedClientDataContainer_SetClientObject(self.as_ptr(), data)
        }
    }
}

// wxSingleInstanceChecker
pub trait SingleInstanceCheckerMethods: WxRustMethods {
    // DTOR: fn ~wxSingleInstanceChecker()
    fn create(&self, name: &str, path: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxSingleInstanceChecker_Create(self.as_ptr(), name, path)
        }
    }
    fn create_default(&self) -> bool {
        unsafe { ffi::wxSingleInstanceChecker_CreateDefault(self.as_ptr()) }
    }
    fn is_another_running(&self) -> bool {
        unsafe { ffi::wxSingleInstanceChecker_IsAnotherRunning(self.as_ptr()) }
    }
}

// wxStackFrame
pub trait StackFrameMethods: WxRustMethods {
    fn get_address(&self) -> *mut c_void {
        unsafe { ffi::wxStackFrame_GetAddress(self.as_ptr()) }
    }
    fn get_file_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStackFrame_GetFileName(self.as_ptr())).into() }
    }
    fn get_level(&self) -> usize {
        unsafe { ffi::wxStackFrame_GetLevel(self.as_ptr()) }
    }
    fn get_line(&self) -> usize {
        unsafe { ffi::wxStackFrame_GetLine(self.as_ptr()) }
    }
    fn get_module(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStackFrame_GetModule(self.as_ptr())).into() }
    }
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStackFrame_GetName(self.as_ptr())).into() }
    }
    fn get_offset(&self) -> usize {
        unsafe { ffi::wxStackFrame_GetOffset(self.as_ptr()) }
    }
    fn get_param(
        &self,
        n: usize,
        type_: *mut c_void,
        name: *mut c_void,
        value: *mut c_void,
    ) -> bool {
        unsafe { ffi::wxStackFrame_GetParam(self.as_ptr(), n, type_, name, value) }
    }
    fn get_param_count(&self) -> usize {
        unsafe { ffi::wxStackFrame_GetParamCount(self.as_ptr()) }
    }
    fn has_source_location(&self) -> bool {
        unsafe { ffi::wxStackFrame_HasSourceLocation(self.as_ptr()) }
    }
}

// wxStackWalker
pub trait StackWalkerMethods: WxRustMethods {
    // DTOR: fn ~wxStackWalker()
    fn walk(&self, skip: usize, max_depth: usize) {
        unsafe { ffi::wxStackWalker_Walk(self.as_ptr(), skip, max_depth) }
    }
    fn walk_from_exception(&self, max_depth: usize) {
        unsafe { ffi::wxStackWalker_WalkFromException(self.as_ptr(), max_depth) }
    }
}

// wxStandardPaths
pub trait StandardPathsMethods: WxRustMethods {
    fn dont_ignore_app_sub_dir(&self) {
        unsafe { ffi::wxStandardPaths_DontIgnoreAppSubDir(self.as_ptr()) }
    }
    fn get_app_documents_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetAppDocumentsDir(self.as_ptr())).into() }
    }
    fn get_config_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetConfigDir(self.as_ptr())).into() }
    }
    fn get_data_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetDataDir(self.as_ptr())).into() }
    }
    fn get_documents_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetDocumentsDir(self.as_ptr())).into() }
    }
    fn get_executable_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetExecutablePath(self.as_ptr())).into() }
    }
    fn get_install_prefix(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetInstallPrefix(self.as_ptr())).into() }
    }
    fn get_local_data_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetLocalDataDir(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetLocalizedResourcesDir()
    fn get_plugins_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetPluginsDir(self.as_ptr())).into() }
    }
    fn get_resources_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetResourcesDir(self.as_ptr())).into() }
    }
    fn get_temp_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetTempDir(self.as_ptr())).into() }
    }
    fn get_user_config_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetUserConfigDir(self.as_ptr())).into() }
    }
    fn get_user_data_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetUserDataDir(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetUserDir()
    fn get_user_local_data_dir(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxStandardPaths_GetUserLocalDataDir(self.as_ptr())).into()
        }
    }
    fn ignore_app_sub_dir(&self, subdir_pattern: &str) {
        unsafe {
            let subdir_pattern = WxString::from(subdir_pattern);
            let subdir_pattern = subdir_pattern.as_ptr();
            ffi::wxStandardPaths_IgnoreAppSubDir(self.as_ptr(), subdir_pattern)
        }
    }
    fn ignore_app_build_sub_dirs(&self) {
        unsafe { ffi::wxStandardPaths_IgnoreAppBuildSubDirs(self.as_ptr()) }
    }
    fn set_install_prefix(&self, prefix: &str) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxStandardPaths_SetInstallPrefix(self.as_ptr(), prefix)
        }
    }
    fn use_app_info(&self, info: c_int) {
        unsafe { ffi::wxStandardPaths_UseAppInfo(self.as_ptr(), info) }
    }
    // NOT_SUPPORTED: fn SetFileLayout()
    // NOT_SUPPORTED: fn GetFileLayout()
    // NOT_SUPPORTED: fn MakeConfigFileName()
    fn get() -> StandardPathsIsOwned<false> {
        unsafe { StandardPathsIsOwned::from_ptr(ffi::wxStandardPaths_Get()) }
    }
    fn msw_get_shell_dir(csidl: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_MSWGetShellDir(csidl)).into() }
    }
}

// wxStopWatch
pub trait StopWatchMethods: WxRustMethods {
    fn pause(&self) {
        unsafe { ffi::wxStopWatch_Pause(self.as_ptr()) }
    }
    fn resume(&self) {
        unsafe { ffi::wxStopWatch_Resume(self.as_ptr()) }
    }
    fn start(&self, milliseconds: c_long) {
        unsafe { ffi::wxStopWatch_Start(self.as_ptr(), milliseconds) }
    }
    fn time(&self) -> c_long {
        unsafe { ffi::wxStopWatch_Time(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn TimeInMicro()
}

// wxStreamBuffer
pub trait StreamBufferMethods: WxRustMethods {
    // DTOR: fn ~wxStreamBuffer()
    fn fill_buffer(&self) -> bool {
        unsafe { ffi::wxStreamBuffer_FillBuffer(self.as_ptr()) }
    }
    fn fixed(&self, fixed: bool) {
        unsafe { ffi::wxStreamBuffer_Fixed(self.as_ptr(), fixed) }
    }
    fn flush_buffer(&self) -> bool {
        unsafe { ffi::wxStreamBuffer_FlushBuffer(self.as_ptr()) }
    }
    fn flushable(&self, flushable: bool) {
        unsafe { ffi::wxStreamBuffer_Flushable(self.as_ptr(), flushable) }
    }
    fn get_buffer_end(&self) -> *mut c_void {
        unsafe { ffi::wxStreamBuffer_GetBufferEnd(self.as_ptr()) }
    }
    fn get_buffer_pos(&self) -> *mut c_void {
        unsafe { ffi::wxStreamBuffer_GetBufferPos(self.as_ptr()) }
    }
    fn get_buffer_size(&self) -> usize {
        unsafe { ffi::wxStreamBuffer_GetBufferSize(self.as_ptr()) }
    }
    fn get_buffer_start(&self) -> *mut c_void {
        unsafe { ffi::wxStreamBuffer_GetBufferStart(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetChar()
    fn get_data_left(&self) -> usize {
        unsafe { ffi::wxStreamBuffer_GetDataLeft(self.as_ptr()) }
    }
    fn get_int_position(&self) -> usize {
        unsafe { ffi::wxStreamBuffer_GetIntPosition(self.as_ptr()) }
    }
    fn get_last_access(&self) -> usize {
        unsafe { ffi::wxStreamBuffer_GetLastAccess(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn PutChar()
    fn read_void(&self, buffer: *mut c_void, size: usize) -> usize {
        unsafe { ffi::wxStreamBuffer_Read(self.as_ptr(), buffer, size) }
    }
    fn read_streambuffer<S: StreamBufferMethods>(&self, buffer: Option<&S>) -> usize {
        unsafe {
            let buffer = match buffer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxStreamBuffer_Read1(self.as_ptr(), buffer)
        }
    }
    fn reset_buffer(&self) {
        unsafe { ffi::wxStreamBuffer_ResetBuffer(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Seek()
    fn set_buffer_io_void(&self, start: *mut c_void, end: *mut c_void, take_ownership: bool) {
        unsafe { ffi::wxStreamBuffer_SetBufferIO(self.as_ptr(), start, end, take_ownership) }
    }
    fn set_buffer_io_sz(&self, bufsize: usize) {
        unsafe { ffi::wxStreamBuffer_SetBufferIO1(self.as_ptr(), bufsize) }
    }
    fn set_int_position(&self, pos: usize) {
        unsafe { ffi::wxStreamBuffer_SetIntPosition(self.as_ptr(), pos) }
    }
    fn stream(&self) -> *mut c_void {
        unsafe { ffi::wxStreamBuffer_Stream(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Tell()
    fn truncate(&self) {
        unsafe { ffi::wxStreamBuffer_Truncate(self.as_ptr()) }
    }
    fn write_void(&self, buffer: *const c_void, size: usize) -> usize {
        unsafe { ffi::wxStreamBuffer_Write(self.as_ptr(), buffer, size) }
    }
    fn write_streambuffer<S: StreamBufferMethods>(&self, buffer: Option<&S>) -> usize {
        unsafe {
            let buffer = match buffer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxStreamBuffer_Write1(self.as_ptr(), buffer)
        }
    }
}

// wxStringClientData
pub trait StringClientDataMethods: ClientDataMethods {
    fn get_data(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStringClientData_GetData(self.as_ptr())).into() }
    }
    fn set_data(&self, data: &str) {
        unsafe {
            let data = WxString::from(data);
            let data = data.as_ptr();
            ffi::wxStringClientData_SetData(self.as_ptr(), data)
        }
    }
}

// wxStringTokenizer
pub trait StringTokenizerMethods: ObjectMethods {
    fn count_tokens(&self) -> usize {
        unsafe { ffi::wxStringTokenizer_CountTokens(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetLastDelimiter()
    fn get_next_token(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStringTokenizer_GetNextToken(self.as_ptr())).into() }
    }
    fn get_position(&self) -> usize {
        unsafe { ffi::wxStringTokenizer_GetPosition(self.as_ptr()) }
    }
    fn get_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStringTokenizer_GetString(self.as_ptr())).into() }
    }
    fn has_more_tokens(&self) -> bool {
        unsafe { ffi::wxStringTokenizer_HasMoreTokens(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetString()
}

// wxSystemOptions
pub trait SystemOptionsMethods: ObjectMethods {
    fn get_option(name: &str) -> String {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            WxString::from_ptr(ffi::wxSystemOptions_GetOption(name)).into()
        }
    }
    fn get_option_int(name: &str) -> c_int {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSystemOptions_GetOptionInt(name)
        }
    }
    fn has_option(name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSystemOptions_HasOption(name)
        }
    }
    fn is_false(name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSystemOptions_IsFalse(name)
        }
    }
    fn set_option_str(name: &str, value: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxSystemOptions_SetOption(name, value)
        }
    }
    fn set_option_int(name: &str, value: c_int) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSystemOptions_SetOption1(name, value)
        }
    }
}

// wxTarClassFactory
pub trait TarClassFactoryMethods: ArchiveClassFactoryMethods {}

// wxTempFFile
pub trait TempFFileMethods: WxRustMethods {
    // DTOR: fn ~wxTempFFile()
    fn commit(&self) -> bool {
        unsafe { ffi::wxTempFFile_Commit(self.as_ptr()) }
    }
    fn discard(&self) {
        unsafe { ffi::wxTempFFile_Discard(self.as_ptr()) }
    }
    fn flush(&self) -> bool {
        unsafe { ffi::wxTempFFile_Flush(self.as_ptr()) }
    }
    fn is_opened(&self) -> bool {
        unsafe { ffi::wxTempFFile_IsOpened(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Length()
    fn open(&self, str_name: &str) -> bool {
        unsafe {
            let str_name = WxString::from(str_name);
            let str_name = str_name.as_ptr();
            ffi::wxTempFFile_Open(self.as_ptr(), str_name)
        }
    }
    // NOT_SUPPORTED: fn Seek()
    // NOT_SUPPORTED: fn Tell()
    fn write(&self, str: &str, conv: *const c_void) -> bool {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxTempFFile_Write(self.as_ptr(), str, conv)
        }
    }
}

// wxTempFile
pub trait TempFileMethods: WxRustMethods {
    // DTOR: fn ~wxTempFile()
    fn commit(&self) -> bool {
        unsafe { ffi::wxTempFile_Commit(self.as_ptr()) }
    }
    fn discard(&self) {
        unsafe { ffi::wxTempFile_Discard(self.as_ptr()) }
    }
    fn flush(&self) -> bool {
        unsafe { ffi::wxTempFile_Flush(self.as_ptr()) }
    }
    fn is_opened(&self) -> bool {
        unsafe { ffi::wxTempFile_IsOpened(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Length()
    fn open(&self, str_name: &str) -> bool {
        unsafe {
            let str_name = WxString::from(str_name);
            let str_name = str_name.as_ptr();
            ffi::wxTempFile_Open(self.as_ptr(), str_name)
        }
    }
    // NOT_SUPPORTED: fn Seek()
    // NOT_SUPPORTED: fn Tell()
    fn write(&self, str: &str, conv: *const c_void) -> bool {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxTempFile_Write(self.as_ptr(), str, conv)
        }
    }
}

// wxThreadHelper
pub trait ThreadHelperMethods: WxRustMethods {
    // DTOR: fn ~wxThreadHelper()
    // NOT_SUPPORTED: fn Entry()
    fn on_delete(&self) {
        unsafe { ffi::wxThreadHelper_OnDelete(self.as_ptr()) }
    }
    fn on_kill(&self) {
        unsafe { ffi::wxThreadHelper_OnKill(self.as_ptr()) }
    }
    fn on_exit(&self) {
        unsafe { ffi::wxThreadHelper_OnExit(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Create()
    // NOT_SUPPORTED: fn CreateThread()
    fn get_thread(&self) -> *mut c_void {
        unsafe { ffi::wxThreadHelper_GetThread(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetThreadKind()
}

// wxTimer
pub trait TimerMethods: EvtHandlerMethods {
    // DTOR: fn ~wxTimer()
    fn get_id(&self) -> c_int {
        unsafe { ffi::wxTimer_GetId(self.as_ptr()) }
    }
    fn get_interval(&self) -> c_int {
        unsafe { ffi::wxTimer_GetInterval(self.as_ptr()) }
    }
    fn get_owner(&self) -> WeakRef<EvtHandler> {
        unsafe { WeakRef::<EvtHandler>::from(ffi::wxTimer_GetOwner(self.as_ptr())) }
    }
    fn is_one_shot(&self) -> bool {
        unsafe { ffi::wxTimer_IsOneShot(self.as_ptr()) }
    }
    fn is_running(&self) -> bool {
        unsafe { ffi::wxTimer_IsRunning(self.as_ptr()) }
    }
    fn notify(&self) {
        unsafe { ffi::wxTimer_Notify(self.as_ptr()) }
    }
    fn set_owner<E: EvtHandlerMethods>(&self, owner: Option<&E>, id: c_int) {
        unsafe {
            let owner = match owner {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTimer_SetOwner(self.as_ptr(), owner, id)
        }
    }
    fn start(&self, milliseconds: c_int, one_shot: bool) -> bool {
        unsafe { ffi::wxTimer_Start(self.as_ptr(), milliseconds, one_shot) }
    }
    fn start_once(&self, milliseconds: c_int) -> bool {
        unsafe { ffi::wxTimer_StartOnce(self.as_ptr(), milliseconds) }
    }
    fn stop(&self) {
        unsafe { ffi::wxTimer_Stop(self.as_ptr()) }
    }
}

// wxTimerEvent
pub trait TimerEventMethods: EventMethods {
    fn get_interval(&self) -> c_int {
        unsafe { ffi::wxTimerEvent_GetInterval(self.as_ptr()) }
    }
    fn get_timer(&self) -> TimerIsOwned<false> {
        unsafe { TimerIsOwned::from_ptr(ffi::wxTimerEvent_GetTimer(self.as_ptr())) }
    }
}

// wxTrackable
pub trait TrackableMethods: WxRustMethods {
    fn as_trackable(&self) -> *mut c_void;
}

// wxUILocale
pub trait UILocaleMethods: WxRustMethods {
    fn use_default() -> bool {
        unsafe { ffi::wxUILocale_UseDefault() }
    }
    fn get_current() -> UILocaleIsOwned<false> {
        unsafe { UILocaleIsOwned::from_ptr(ffi::wxUILocale_GetCurrent()) }
    }
    fn from_tag(tag: &str) -> UILocale {
        unsafe {
            let tag = WxString::from(tag);
            let tag = tag.as_ptr();
            UILocale::from_ptr(ffi::wxUILocale_FromTag(tag))
        }
    }
    fn add_language(info: *const c_void) {
        unsafe { ffi::wxUILocale_AddLanguage(info) }
    }
    fn find_language_info_str(locale: &str) -> *const c_void {
        unsafe {
            let locale = WxString::from(locale);
            let locale = locale.as_ptr();
            ffi::wxUILocale_FindLanguageInfo(locale)
        }
    }
    fn find_language_info_localeident<L: LocaleIdentMethods>(locale_id: &L) -> *const c_void {
        unsafe {
            let locale_id = locale_id.as_ptr();
            ffi::wxUILocale_FindLanguageInfo1(locale_id)
        }
    }
    fn get_language_info(lang: c_int) -> *const c_void {
        unsafe { ffi::wxUILocale_GetLanguageInfo(lang) }
    }
    fn get_language_name(lang: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxUILocale_GetLanguageName(lang)).into() }
    }
    fn get_language_canonical_name(lang: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxUILocale_GetLanguageCanonicalName(lang)).into() }
    }
    fn get_system_language() -> c_int {
        unsafe { ffi::wxUILocale_GetSystemLanguage() }
    }
    fn get_system_locale() -> c_int {
        unsafe { ffi::wxUILocale_GetSystemLocale() }
    }
    fn compare_strings(&self, lhs: &str, rhs: &str, flags: c_int) -> c_int {
        unsafe {
            let lhs = WxString::from(lhs);
            let lhs = lhs.as_ptr();
            let rhs = WxString::from(rhs);
            let rhs = rhs.as_ptr();
            ffi::wxUILocale_CompareStrings(self.as_ptr(), lhs, rhs, flags)
        }
    }
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxUILocale_GetName(self.as_ptr())).into() }
    }
    fn get_locale_id(&self) -> LocaleIdent {
        unsafe { LocaleIdent::from_ptr(ffi::wxUILocale_GetLocaleId(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetInfo()
    // NOT_SUPPORTED: fn GetLocalizedName()
    fn get_layout_direction(&self) -> c_int {
        unsafe { ffi::wxUILocale_GetLayoutDirection(self.as_ptr()) }
    }
    fn is_supported(&self) -> bool {
        unsafe { ffi::wxUILocale_IsSupported(self.as_ptr()) }
    }
}

// wxURI
pub trait URIMethods: ObjectMethods {
    fn build_uri(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxURI_BuildURI(self.as_ptr())).into() }
    }
    fn build_unescaped_uri(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxURI_BuildUnescapedURI(self.as_ptr())).into() }
    }
    fn create(&self, uri: &str) -> bool {
        unsafe {
            let uri = WxString::from(uri);
            let uri = uri.as_ptr();
            ffi::wxURI_Create(self.as_ptr(), uri)
        }
    }
    fn get_fragment(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxURI_GetFragment(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetHostType()
    fn get_password(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxURI_GetPassword(self.as_ptr())).into() }
    }
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxURI_GetPath(self.as_ptr())).into() }
    }
    fn get_port(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxURI_GetPort(self.as_ptr())).into() }
    }
    fn get_query(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxURI_GetQuery(self.as_ptr())).into() }
    }
    fn get_scheme(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxURI_GetScheme(self.as_ptr())).into() }
    }
    fn get_server(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxURI_GetServer(self.as_ptr())).into() }
    }
    fn get_user(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxURI_GetUser(self.as_ptr())).into() }
    }
    fn get_user_info(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxURI_GetUserInfo(self.as_ptr())).into() }
    }
    fn has_fragment(&self) -> bool {
        unsafe { ffi::wxURI_HasFragment(self.as_ptr()) }
    }
    fn has_path(&self) -> bool {
        unsafe { ffi::wxURI_HasPath(self.as_ptr()) }
    }
    fn has_port(&self) -> bool {
        unsafe { ffi::wxURI_HasPort(self.as_ptr()) }
    }
    fn has_query(&self) -> bool {
        unsafe { ffi::wxURI_HasQuery(self.as_ptr()) }
    }
    fn has_scheme(&self) -> bool {
        unsafe { ffi::wxURI_HasScheme(self.as_ptr()) }
    }
    fn has_server(&self) -> bool {
        unsafe { ffi::wxURI_HasServer(self.as_ptr()) }
    }
    fn has_user_info(&self) -> bool {
        unsafe { ffi::wxURI_HasUserInfo(self.as_ptr()) }
    }
    fn is_reference(&self) -> bool {
        unsafe { ffi::wxURI_IsReference(self.as_ptr()) }
    }
    fn resolve<U: URIMethods>(&self, base: &U, flags: c_int) {
        unsafe {
            let base = base.as_ptr();
            ffi::wxURI_Resolve(self.as_ptr(), base, flags)
        }
    }
    // BLOCKED: fn operator==()
    fn unescape(uri: &str) -> String {
        unsafe {
            let uri = WxString::from(uri);
            let uri = uri.as_ptr();
            WxString::from_ptr(ffi::wxURI_Unescape(uri)).into()
        }
    }
}

// wxUniCharRef
pub trait UniCharRefMethods: WxRustMethods {}

// wxVariantData
pub trait VariantDataMethods: ObjectRefDataMethods {
    fn clone(&self) -> Option<VariantDataIsOwned<false>> {
        unsafe { VariantData::option_from(ffi::wxVariantData_Clone(self.as_ptr())) }
    }
    fn dec_ref(&self) {
        unsafe { ffi::wxVariantData_DecRef(self.as_ptr()) }
    }
    fn eq<V: VariantDataMethods>(&self, data: &V) -> bool {
        unsafe {
            let data = data.as_ptr();
            ffi::wxVariantData_Eq(self.as_ptr(), data)
        }
    }
    fn get_any<A: AnyMethods>(&self, any: Option<&A>) -> bool {
        unsafe {
            let any = match any {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxVariantData_GetAny(self.as_ptr(), any)
        }
    }
    fn get_type(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxVariantData_GetType(self.as_ptr())).into() }
    }
    fn get_value_class_info(&self) -> Option<ClassInfoIsOwned<false>> {
        unsafe { ClassInfo::option_from(ffi::wxVariantData_GetValueClassInfo(self.as_ptr())) }
    }
    fn inc_ref(&self) {
        unsafe { ffi::wxVariantData_IncRef(self.as_ptr()) }
    }
    fn read_istream(&self, stream: *mut c_void) -> bool {
        unsafe { ffi::wxVariantData_Read(self.as_ptr(), stream) }
    }
    fn read_str(&self, string: *mut c_void) -> bool {
        unsafe { ffi::wxVariantData_Read1(self.as_ptr(), string) }
    }
    fn write_ostream(&self, stream: *mut c_void) -> bool {
        unsafe { ffi::wxVariantData_Write(self.as_ptr(), stream) }
    }
    fn write_str(&self, string: *mut c_void) -> bool {
        unsafe { ffi::wxVariantData_Write1(self.as_ptr(), string) }
    }
}

// wxVersionInfo
pub trait VersionInfoMethods: WxRustMethods {
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxVersionInfo_GetName(self.as_ptr())).into() }
    }
    fn get_major(&self) -> c_int {
        unsafe { ffi::wxVersionInfo_GetMajor(self.as_ptr()) }
    }
    fn get_minor(&self) -> c_int {
        unsafe { ffi::wxVersionInfo_GetMinor(self.as_ptr()) }
    }
    fn get_micro(&self) -> c_int {
        unsafe { ffi::wxVersionInfo_GetMicro(self.as_ptr()) }
    }
    fn get_revision(&self) -> c_int {
        unsafe { ffi::wxVersionInfo_GetRevision(self.as_ptr()) }
    }
    fn to_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxVersionInfo_ToString(self.as_ptr())).into() }
    }
    fn get_version_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxVersionInfo_GetVersionString(self.as_ptr())).into() }
    }
    fn has_description(&self) -> bool {
        unsafe { ffi::wxVersionInfo_HasDescription(self.as_ptr()) }
    }
    fn get_description(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxVersionInfo_GetDescription(self.as_ptr())).into() }
    }
    fn has_copyright(&self) -> bool {
        unsafe { ffi::wxVersionInfo_HasCopyright(self.as_ptr()) }
    }
    fn get_copyright(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxVersionInfo_GetCopyright(self.as_ptr())).into() }
    }
}

// wxWindowUpdateLocker
pub trait WindowUpdateLockerMethods: WxRustMethods {
    fn lock<W: WindowMethods>(&self, win: Option<&W>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindowUpdateLocker_Lock(self.as_ptr(), win)
        }
    }
    // DTOR: fn ~wxWindowUpdateLocker()
}

// wxZipClassFactory
pub trait ZipClassFactoryMethods: ArchiveClassFactoryMethods {}

// wxZipNotifier
pub trait ZipNotifierMethods: WxRustMethods {
    fn on_entry_updated(&self, entry: *mut c_void) {
        unsafe { ffi::wxZipNotifier_OnEntryUpdated(self.as_ptr(), entry) }
    }
}
