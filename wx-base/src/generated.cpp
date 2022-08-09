#include "generated.h"

extern "C" {

// CLASS: wxAny
void wxAny_delete(wxAny *self) {
    delete self;
}
wxAny *wxAny_new() {
    return new wxAny();
}
wxAny *wxAny_new1(const T * value) {
    return new wxAny(*value);
}
wxAny *wxAny_new2(const wxAny * any) {
    return new wxAny(*any);
}
wxAny *wxAny_new3(const wxVariant * variant) {
    return new wxAny(*variant);
}
bool wxAny_CheckType(const wxAny * self) {
    return self->CheckType();
}
bool wxAny_GetAs(const wxAny * self, T * value) {
    return self->GetAs(value);
}
bool wxAny_GetAs1(const wxAny * self, wxVariant * value) {
    return self->GetAs(value);
}
const wxAnyValueType * wxAny_GetType(const wxAny * self) {
    return self->GetType();
}
bool wxAny_HasSameType(const wxAny * self, const wxAny * other) {
    return self->HasSameType(*other);
}
bool wxAny_IsNull(const wxAny * self) {
    return self->IsNull();
}
void wxAny_MakeNull(wxAny * self) {
    return self->MakeNull();
}

// CLASS: wxAnyValueType
void wxAnyValueType_delete(wxAnyValueType *self) {
    delete self;
}
wxAnyValueType *wxAnyValueType_new() {
    return new wxAnyValueType();
}
bool wxAnyValueType_CheckType(const wxAnyValueType * self) {
    return self->CheckType();
}
bool wxAnyValueType_ConvertValue(const wxAnyValueType * self, const wxAnyValueBuffer * src, wxAnyValueType * dst_type, wxAnyValueBuffer * dst) {
    return self->ConvertValue(*src, dst_type, *dst);
}
void wxAnyValueType_CopyBuffer(const wxAnyValueType * self, const wxAnyValueBuffer * src, wxAnyValueBuffer * dst) {
    return self->CopyBuffer(*src, *dst);
}
void wxAnyValueType_DeleteValue(const wxAnyValueType * self, wxAnyValueBuffer * buf) {
    return self->DeleteValue(*buf);
}
bool wxAnyValueType_IsSameType(const wxAnyValueType * self, const wxAnyValueType * other_type) {
    return self->IsSameType(other_type);
}

// CLASS: wxAppTraits
void wxAppTraits_delete(wxAppTraits *self) {
    delete self;
}
wxConfigBase * wxAppTraits_CreateConfig(wxAppTraits * self) {
    return self->CreateConfig();
}
wxEventLoopBase * wxAppTraits_CreateEventLoop(wxAppTraits * self) {
    return self->CreateEventLoop();
}
wxFontMapper * wxAppTraits_CreateFontMapper(wxAppTraits * self) {
    return self->CreateFontMapper();
}
wxLog * wxAppTraits_CreateLogTarget(wxAppTraits * self) {
    return self->CreateLogTarget();
}
wxMessageOutput * wxAppTraits_CreateMessageOutput(wxAppTraits * self) {
    return self->CreateMessageOutput();
}
wxRendererNative * wxAppTraits_CreateRenderer(wxAppTraits * self) {
    return self->CreateRenderer();
}
wxString *wxAppTraits_GetDesktopEnvironment(const wxAppTraits * self) {
    return new wxString(self->GetDesktopEnvironment());
}
wxStandardPaths * wxAppTraits_GetStandardPaths(wxAppTraits * self) {
    return &(self->GetStandardPaths());
}
bool wxAppTraits_HasStderr(wxAppTraits * self) {
    return self->HasStderr();
}
bool wxAppTraits_IsUsingUniversalWidgets(const wxAppTraits * self) {
    return self->IsUsingUniversalWidgets();
}
bool wxAppTraits_ShowAssertDialog(wxAppTraits * self, const wxString * msg) {
    return self->ShowAssertDialog(*msg);
}
bool wxAppTraits_SafeMessageBox(wxAppTraits * self, const wxString * text, const wxString * title) {
    return self->SafeMessageBox(*text, *title);
}
wxString *wxAppTraits_GetAssertStackTrace(wxAppTraits * self) {
    return new wxString(self->GetAssertStackTrace());
}

// CLASS: wxArchiveClassFactory
wxClassInfo *wxArchiveClassFactory_CLASSINFO() {
    return wxCLASSINFO(wxArchiveClassFactory);
}
wxMBConv * wxArchiveClassFactory_GetConv(const wxArchiveClassFactory * self) {
    return self->GetConv();
}
void wxArchiveClassFactory_SetConv(wxArchiveClassFactory * self, wxMBConv * conv) {
    return self->SetConv(*conv);
}
const wxArchiveClassFactory * wxArchiveClassFactory_GetNext(const wxArchiveClassFactory * self) {
    return self->GetNext();
}
wxString *wxArchiveClassFactory_GetProtocol(const wxArchiveClassFactory * self) {
    return new wxString(self->GetProtocol());
}
wxArchiveEntry * wxArchiveClassFactory_NewEntry(const wxArchiveClassFactory * self) {
    return self->NewEntry();
}
wxArchiveInputStream * wxArchiveClassFactory_NewStream(const wxArchiveClassFactory * self, wxInputStream * stream) {
    return self->NewStream(*stream);
}
wxArchiveOutputStream * wxArchiveClassFactory_NewStream1(const wxArchiveClassFactory * self, wxOutputStream * stream) {
    return self->NewStream(*stream);
}
wxArchiveInputStream * wxArchiveClassFactory_NewStream2(const wxArchiveClassFactory * self, wxInputStream * stream) {
    return self->NewStream(stream);
}
wxArchiveOutputStream * wxArchiveClassFactory_NewStream3(const wxArchiveClassFactory * self, wxOutputStream * stream) {
    return self->NewStream(stream);
}
void wxArchiveClassFactory_PushFront(wxArchiveClassFactory * self) {
    return self->PushFront();
}
void wxArchiveClassFactory_Remove(wxArchiveClassFactory * self) {
    return self->Remove();
}
const wxArchiveClassFactory * wxArchiveClassFactory_GetFirst() {
    return wxArchiveClassFactory::GetFirst();
}

// CLASS: wxArchiveEntry
wxClassInfo *wxArchiveEntry_CLASSINFO() {
    return wxCLASSINFO(wxArchiveEntry);
}
wxArchiveEntry * wxArchiveEntry_Clone(const wxArchiveEntry * self) {
    return self->Clone();
}
wxDateTime *wxArchiveEntry_GetDateTime(const wxArchiveEntry * self) {
    return new wxDateTime(self->GetDateTime());
}
void wxArchiveEntry_SetDateTime(wxArchiveEntry * self, const wxDateTime * dt) {
    return self->SetDateTime(*dt);
}
wxString *wxArchiveEntry_GetInternalName(const wxArchiveEntry * self) {
    return new wxString(self->GetInternalName());
}
bool wxArchiveEntry_IsDir(const wxArchiveEntry * self) {
    return self->IsDir();
}
void wxArchiveEntry_SetIsDir(wxArchiveEntry * self, bool is_dir) {
    return self->SetIsDir(is_dir);
}
bool wxArchiveEntry_IsReadOnly(const wxArchiveEntry * self) {
    return self->IsReadOnly();
}
void wxArchiveEntry_SetIsReadOnly(wxArchiveEntry * self, bool is_read_only) {
    return self->SetIsReadOnly(is_read_only);
}
void wxArchiveEntry_SetNotifier(wxArchiveEntry * self, wxArchiveNotifier * notifier) {
    return self->SetNotifier(*notifier);
}
void wxArchiveEntry_UnsetNotifier(wxArchiveEntry * self) {
    return self->UnsetNotifier();
}

// CLASS: wxArchiveIterator
void wxArchiveIterator_delete(wxArchiveIterator *self) {
    delete self;
}
wxArchiveIterator *wxArchiveIterator_new() {
    return new wxArchiveIterator();
}
wxArchiveIterator *wxArchiveIterator_new1(Arc * arc) {
    return new wxArchiveIterator(*arc);
}

// CLASS: wxArchiveNotifier
void wxArchiveNotifier_delete(wxArchiveNotifier *self) {
    delete self;
}
void wxArchiveNotifier_OnEntryUpdated(wxArchiveNotifier * self, wxArchiveEntry * entry) {
    return self->OnEntryUpdated(*entry);
}

// CLASS: wxClassInfo
void wxClassInfo_delete(wxClassInfo *self) {
    delete self;
}
wxObject * wxClassInfo_CreateObject(const wxClassInfo * self) {
    return self->CreateObject();
}
const wxChar * wxClassInfo_GetBaseClassName1(const wxClassInfo * self) {
    return self->GetBaseClassName1();
}
const wxChar * wxClassInfo_GetBaseClassName2(const wxClassInfo * self) {
    return self->GetBaseClassName2();
}
const wxChar * wxClassInfo_GetClassName(const wxClassInfo * self) {
    return self->GetClassName();
}
int wxClassInfo_GetSize(const wxClassInfo * self) {
    return self->GetSize();
}
bool wxClassInfo_IsDynamic(const wxClassInfo * self) {
    return self->IsDynamic();
}
bool wxClassInfo_IsKindOf(const wxClassInfo * self, const wxClassInfo * info) {
    return self->IsKindOf(info);
}
wxClassInfo * wxClassInfo_FindClass(const wxString * class_name) {
    return wxClassInfo::FindClass(*class_name);
}

// CLASS: wxClient
wxClassInfo *wxClient_CLASSINFO() {
    return wxCLASSINFO(wxClient);
}
wxClient *wxClient_new() {
    return new wxClient();
}
wxConnectionBase * wxClient_MakeConnection(wxClient * self, const wxString * host, const wxString * service, const wxString * topic) {
    return self->MakeConnection(*host, *service, *topic);
}
wxConnectionBase * wxClient_OnMakeConnection(wxClient * self) {
    return self->OnMakeConnection();
}
bool wxClient_ValidHost(wxClient * self, const wxString * host) {
    return self->ValidHost(*host);
}

// CLASS: wxClientData
void wxClientData_delete(wxClientData *self) {
    delete self;
}
wxClientData *wxClientData_new() {
    return new wxClientData();
}

// CLASS: wxClientDataContainer
void wxClientDataContainer_delete(wxClientDataContainer *self) {
    delete self;
}
wxClientDataContainer *wxClientDataContainer_new() {
    return new wxClientDataContainer();
}
void * wxClientDataContainer_GetClientData(const wxClientDataContainer * self) {
    return self->GetClientData();
}
wxClientData * wxClientDataContainer_GetClientObject(const wxClientDataContainer * self) {
    return self->GetClientObject();
}
void wxClientDataContainer_SetClientData(wxClientDataContainer * self, void * data) {
    return self->SetClientData(data);
}
void wxClientDataContainer_SetClientObject(wxClientDataContainer * self, wxClientData * data) {
    return self->SetClientObject(data);
}

// CLASS: wxCmdLineParser
void wxCmdLineParser_delete(wxCmdLineParser *self) {
    delete self;
}
wxCmdLineParser *wxCmdLineParser_new() {
    return new wxCmdLineParser();
}
wxCmdLineParser *wxCmdLineParser_new1(int argc, char ** argv) {
    return new wxCmdLineParser(argc, argv);
}
wxCmdLineParser *wxCmdLineParser_new2(int argc, wchar_t ** argv) {
    return new wxCmdLineParser(argc, argv);
}
wxCmdLineParser *wxCmdLineParser_new3(const wxString * cmdline) {
    return new wxCmdLineParser(*cmdline);
}
wxCmdLineParser *wxCmdLineParser_new4(const wxCmdLineEntryDesc * desc) {
    return new wxCmdLineParser(desc);
}
wxCmdLineParser *wxCmdLineParser_new5(const wxCmdLineEntryDesc * desc, int argc, char ** argv) {
    return new wxCmdLineParser(desc, argc, argv);
}
wxCmdLineParser *wxCmdLineParser_new6(const wxCmdLineEntryDesc * desc, const wxString * cmdline) {
    return new wxCmdLineParser(desc, *cmdline);
}
void wxCmdLineParser_AddLongSwitch(wxCmdLineParser * self, const wxString * lng, const wxString * desc, int flags) {
    return self->AddLongSwitch(*lng, *desc, flags);
}
void wxCmdLineParser_AddSwitch(wxCmdLineParser * self, const wxString * name, const wxString * lng, const wxString * desc, int flags) {
    return self->AddSwitch(*name, *lng, *desc, flags);
}
void wxCmdLineParser_AddUsageText(wxCmdLineParser * self, const wxString * text) {
    return self->AddUsageText(*text);
}
bool wxCmdLineParser_AreLongOptionsEnabled(const wxCmdLineParser * self) {
    return self->AreLongOptionsEnabled();
}
void wxCmdLineParser_DisableLongOptions(wxCmdLineParser * self) {
    return self->DisableLongOptions();
}
void wxCmdLineParser_EnableLongOptions(wxCmdLineParser * self, bool enable) {
    return self->EnableLongOptions(enable);
}
bool wxCmdLineParser_Found(const wxCmdLineParser * self, const wxString * name) {
    return self->Found(*name);
}
bool wxCmdLineParser_Found1(const wxCmdLineParser * self, const wxString * name, wxString * value) {
    return self->Found(*name, value);
}
bool wxCmdLineParser_Found2(const wxCmdLineParser * self, const wxString * name, long * value) {
    return self->Found(*name, value);
}
bool wxCmdLineParser_Found3(const wxCmdLineParser * self, const wxString * name, double * value) {
    return self->Found(*name, value);
}
bool wxCmdLineParser_Found4(const wxCmdLineParser * self, const wxString * name, wxDateTime * value) {
    return self->Found(*name, value);
}
wxString *wxCmdLineParser_GetParam(const wxCmdLineParser * self, size_t n) {
    return new wxString(self->GetParam(n));
}
size_t wxCmdLineParser_GetParamCount(const wxCmdLineParser * self) {
    return self->GetParamCount();
}
wxCmdLineArgs *wxCmdLineParser_GetArguments(const wxCmdLineParser * self) {
    return new wxCmdLineArgs(self->GetArguments());
}
int wxCmdLineParser_Parse(wxCmdLineParser * self, bool give_usage) {
    return self->Parse(give_usage);
}
void wxCmdLineParser_SetCmdLine(wxCmdLineParser * self, int argc, char ** argv) {
    return self->SetCmdLine(argc, argv);
}
void wxCmdLineParser_SetCmdLine1(wxCmdLineParser * self, int argc, wchar_t ** argv) {
    return self->SetCmdLine(argc, argv);
}
void wxCmdLineParser_SetCmdLine2(wxCmdLineParser * self, const wxString * cmdline) {
    return self->SetCmdLine(*cmdline);
}
void wxCmdLineParser_SetDesc(wxCmdLineParser * self, const wxCmdLineEntryDesc * desc) {
    return self->SetDesc(desc);
}
void wxCmdLineParser_SetLogo(wxCmdLineParser * self, const wxString * logo) {
    return self->SetLogo(*logo);
}
void wxCmdLineParser_SetSwitchChars(wxCmdLineParser * self, const wxString * switch_chars) {
    return self->SetSwitchChars(*switch_chars);
}
void wxCmdLineParser_Usage(const wxCmdLineParser * self) {
    return self->Usage();
}
wxString *wxCmdLineParser_GetUsageString(const wxCmdLineParser * self) {
    return new wxString(self->GetUsageString());
}

// CLASS: wxCondition
void wxCondition_delete(wxCondition *self) {
    delete self;
}
wxCondition *wxCondition_new(wxMutex * mutex) {
    return new wxCondition(*mutex);
}
bool wxCondition_IsOk(const wxCondition * self) {
    return self->IsOk();
}

// CLASS: wxConfigPathChanger
void wxConfigPathChanger_delete(wxConfigPathChanger *self) {
    delete self;
}
wxConfigPathChanger *wxConfigPathChanger_new(const wxConfigBase * p_container, const wxString * str_entry) {
    return new wxConfigPathChanger(p_container, *str_entry);
}
wxString *wxConfigPathChanger_Name(const wxConfigPathChanger * self) {
    return new wxString(self->Name());
}
void wxConfigPathChanger_UpdateIfDeleted(wxConfigPathChanger * self) {
    return self->UpdateIfDeleted();
}

// CLASS: wxConnection
wxClassInfo *wxConnection_CLASSINFO() {
    return wxCLASSINFO(wxConnection);
}
wxConnection *wxConnection_new() {
    return new wxConnection();
}
wxConnection *wxConnection_new1(void * buffer, size_t size) {
    return new wxConnection(buffer, size);
}
bool wxConnection_Advise1(wxConnection * self, const wxString * item, const char * data) {
    return self->Advise(*item, data);
}
bool wxConnection_Advise2(wxConnection * self, const wxString * item, const wchar_t * data) {
    return self->Advise(*item, data);
}
bool wxConnection_Advise3(wxConnection * self, const wxString * item, const wxString data) {
    return self->Advise(*item, data);
}
bool wxConnection_Disconnect(wxConnection * self) {
    return self->Disconnect();
}
bool wxConnection_Execute1(wxConnection * self, const char * data) {
    return self->Execute(data);
}
bool wxConnection_Execute2(wxConnection * self, const wchar_t * data) {
    return self->Execute(data);
}
bool wxConnection_Execute3(wxConnection * self, const wxString data) {
    return self->Execute(data);
}
bool wxConnection_OnDisconnect(wxConnection * self) {
    return self->OnDisconnect();
}
bool wxConnection_OnExec(wxConnection * self, const wxString * topic, const wxString * data) {
    return self->OnExec(*topic, *data);
}
bool wxConnection_OnStartAdvise(wxConnection * self, const wxString * topic, const wxString * item) {
    return self->OnStartAdvise(*topic, *item);
}
bool wxConnection_OnStopAdvise(wxConnection * self, const wxString * topic, const wxString * item) {
    return self->OnStopAdvise(*topic, *item);
}
bool wxConnection_Poke1(wxConnection * self, const wxString * item, const char * data) {
    return self->Poke(*item, data);
}
bool wxConnection_Poke2(wxConnection * self, const wxString * item, const wchar_t * data) {
    return self->Poke(*item, data);
}
bool wxConnection_Poke3(wxConnection * self, const wxString * item, const wxString data) {
    return self->Poke(*item, data);
}
bool wxConnection_StartAdvise(wxConnection * self, const wxString * item) {
    return self->StartAdvise(*item);
}
bool wxConnection_StopAdvise(wxConnection * self, const wxString * item) {
    return self->StopAdvise(*item);
}

// CLASS: wxConnectionBase
wxClassInfo *wxConnectionBase_CLASSINFO() {
    return wxCLASSINFO(wxConnectionBase);
}

// CLASS: wxCriticalSection
void wxCriticalSection_delete(wxCriticalSection *self) {
    delete self;
}
void wxCriticalSection_Enter(wxCriticalSection * self) {
    return self->Enter();
}
bool wxCriticalSection_TryEnter(wxCriticalSection * self) {
    return self->TryEnter();
}
void wxCriticalSection_Leave(wxCriticalSection * self) {
    return self->Leave();
}

// CLASS: wxCriticalSectionLocker
void wxCriticalSectionLocker_delete(wxCriticalSectionLocker *self) {
    delete self;
}
wxCriticalSectionLocker *wxCriticalSectionLocker_new(wxCriticalSection * criticalsection) {
    return new wxCriticalSectionLocker(*criticalsection);
}

// CLASS: wxDDEClient
wxClassInfo *wxDDEClient_CLASSINFO() {
    return wxCLASSINFO(wxDDEClient);
}
wxDDEClient *wxDDEClient_new() {
    return new wxDDEClient();
}
wxConnectionBase * wxDDEClient_MakeConnection(wxDDEClient * self, const wxString * host, const wxString * service, const wxString * topic) {
    return self->MakeConnection(*host, *service, *topic);
}
wxConnectionBase * wxDDEClient_OnMakeConnection(wxDDEClient * self) {
    return self->OnMakeConnection();
}
bool wxDDEClient_ValidHost(wxDDEClient * self, const wxString * host) {
    return self->ValidHost(*host);
}

// CLASS: wxDDEConnection
wxClassInfo *wxDDEConnection_CLASSINFO() {
    return wxCLASSINFO(wxDDEConnection);
}
wxDDEConnection *wxDDEConnection_new() {
    return new wxDDEConnection();
}
wxDDEConnection *wxDDEConnection_new1(void * buffer, size_t size) {
    return new wxDDEConnection(buffer, size);
}
bool wxDDEConnection_Advise1(wxDDEConnection * self, const wxString * item, const char * data) {
    return self->Advise(*item, data);
}
bool wxDDEConnection_Advise2(wxDDEConnection * self, const wxString * item, const wchar_t * data) {
    return self->Advise(*item, data);
}
bool wxDDEConnection_Advise3(wxDDEConnection * self, const wxString * item, const wxString data) {
    return self->Advise(*item, data);
}
bool wxDDEConnection_Disconnect(wxDDEConnection * self) {
    return self->Disconnect();
}
bool wxDDEConnection_Execute1(wxDDEConnection * self, const char * data) {
    return self->Execute(data);
}
bool wxDDEConnection_Execute2(wxDDEConnection * self, const wchar_t * data) {
    return self->Execute(data);
}
bool wxDDEConnection_Execute3(wxDDEConnection * self, const wxString data) {
    return self->Execute(data);
}
bool wxDDEConnection_OnDisconnect(wxDDEConnection * self) {
    return self->OnDisconnect();
}
bool wxDDEConnection_OnStartAdvise(wxDDEConnection * self, const wxString * topic, const wxString * item) {
    return self->OnStartAdvise(*topic, *item);
}
bool wxDDEConnection_OnStopAdvise(wxDDEConnection * self, const wxString * topic, const wxString * item) {
    return self->OnStopAdvise(*topic, *item);
}
bool wxDDEConnection_Poke1(wxDDEConnection * self, const wxString * item, const char * data) {
    return self->Poke(*item, data);
}
bool wxDDEConnection_Poke2(wxDDEConnection * self, const wxString * item, const wchar_t * data) {
    return self->Poke(*item, data);
}
bool wxDDEConnection_Poke3(wxDDEConnection * self, const wxString * item, const wxString data) {
    return self->Poke(*item, data);
}
bool wxDDEConnection_StartAdvise(wxDDEConnection * self, const wxString * item) {
    return self->StartAdvise(*item);
}
bool wxDDEConnection_StopAdvise(wxDDEConnection * self, const wxString * item) {
    return self->StopAdvise(*item);
}

// CLASS: wxDDEServer
void wxDDEServer_delete(wxDDEServer *self) {
    delete self;
}
wxDDEServer *wxDDEServer_new() {
    return new wxDDEServer();
}
bool wxDDEServer_Create(wxDDEServer * self, const wxString * service) {
    return self->Create(*service);
}
wxConnectionBase * wxDDEServer_OnAcceptConnection(wxDDEServer * self, const wxString * topic) {
    return self->OnAcceptConnection(*topic);
}

// CLASS: wxDataInputStream
void wxDataInputStream_delete(wxDataInputStream *self) {
    delete self;
}
wxDataInputStream *wxDataInputStream_new(wxInputStream * stream, const wxMBConv * conv) {
    return new wxDataInputStream(*stream, *conv);
}
void wxDataInputStream_BigEndianOrdered(wxDataInputStream * self, bool be_order) {
    return self->BigEndianOrdered(be_order);
}
wxMBConv * wxDataInputStream_GetConv(const wxDataInputStream * self) {
    return self->GetConv();
}
void wxDataInputStream_Read81(wxDataInputStream * self, wxUint8 * buffer, size_t size) {
    return self->Read8(buffer, size);
}
void wxDataInputStream_Read161(wxDataInputStream * self, wxUint16 * buffer, size_t size) {
    return self->Read16(buffer, size);
}
void wxDataInputStream_Read321(wxDataInputStream * self, wxUint32 * buffer, size_t size) {
    return self->Read32(buffer, size);
}
void wxDataInputStream_Read641(wxDataInputStream * self, wxUint64 * buffer, size_t size) {
    return self->Read64(buffer, size);
}
void wxDataInputStream_ReadFloat1(wxDataInputStream * self, float * buffer, size_t size) {
    return self->ReadFloat(buffer, size);
}
double wxDataInputStream_ReadDouble(wxDataInputStream * self) {
    return self->ReadDouble();
}
void wxDataInputStream_ReadDouble1(wxDataInputStream * self, double * buffer, size_t size) {
    return self->ReadDouble(buffer, size);
}
wxString *wxDataInputStream_ReadString(wxDataInputStream * self) {
    return new wxString(self->ReadString());
}
void wxDataInputStream_SetConv(wxDataInputStream * self, const wxMBConv * conv) {
    return self->SetConv(*conv);
}
void wxDataInputStream_UseBasicPrecisions(wxDataInputStream * self) {
    return self->UseBasicPrecisions();
}
void wxDataInputStream_UseExtendedPrecision(wxDataInputStream * self) {
    return self->UseExtendedPrecision();
}

// CLASS: wxDataOutputStream
void wxDataOutputStream_delete(wxDataOutputStream *self) {
    delete self;
}
wxDataOutputStream *wxDataOutputStream_new(wxOutputStream * stream, const wxMBConv * conv) {
    return new wxDataOutputStream(*stream, *conv);
}
void wxDataOutputStream_BigEndianOrdered(wxDataOutputStream * self, bool be_order) {
    return self->BigEndianOrdered(be_order);
}
wxMBConv * wxDataOutputStream_GetConv(const wxDataOutputStream * self) {
    return self->GetConv();
}
void wxDataOutputStream_SetConv(wxDataOutputStream * self, const wxMBConv * conv) {
    return self->SetConv(*conv);
}
void wxDataOutputStream_UseBasicPrecisions(wxDataOutputStream * self) {
    return self->UseBasicPrecisions();
}
void wxDataOutputStream_UseExtendedPrecision(wxDataOutputStream * self) {
    return self->UseExtendedPrecision();
}
void wxDataOutputStream_Write81(wxDataOutputStream * self, const wxUint8 * buffer, size_t size) {
    return self->Write8(buffer, size);
}
void wxDataOutputStream_Write161(wxDataOutputStream * self, const wxUint16 * buffer, size_t size) {
    return self->Write16(buffer, size);
}
void wxDataOutputStream_Write321(wxDataOutputStream * self, const wxUint32 * buffer, size_t size) {
    return self->Write32(buffer, size);
}
void wxDataOutputStream_Write641(wxDataOutputStream * self, const wxUint64 * buffer, size_t size) {
    return self->Write64(buffer, size);
}
void wxDataOutputStream_WriteFloat1(wxDataOutputStream * self, const float * buffer, size_t size) {
    return self->WriteFloat(buffer, size);
}
void wxDataOutputStream_WriteDouble(wxDataOutputStream * self, double d) {
    return self->WriteDouble(d);
}
void wxDataOutputStream_WriteDouble1(wxDataOutputStream * self, const double * buffer, size_t size) {
    return self->WriteDouble(buffer, size);
}
void wxDataOutputStream_WriteString(wxDataOutputStream * self, const wxString * string) {
    return self->WriteString(*string);
}

// CLASS: wxDateSpan
void wxDateSpan_delete(wxDateSpan *self) {
    delete self;
}
wxDateSpan *wxDateSpan_new(int years, int months, int weeks, int days) {
    return new wxDateSpan(years, months, weeks, days);
}
wxDateSpan *wxDateSpan_Add(const wxDateSpan * self, const wxDateSpan * other) {
    return new wxDateSpan(self->Add(*other));
}
wxDateSpan * wxDateSpan_Add1(wxDateSpan * self, const wxDateSpan * other) {
    return &(self->Add(*other));
}
int wxDateSpan_GetDays(const wxDateSpan * self) {
    return self->GetDays();
}
int wxDateSpan_GetMonths(const wxDateSpan * self) {
    return self->GetMonths();
}
int wxDateSpan_GetTotalMonths(const wxDateSpan * self) {
    return self->GetTotalMonths();
}
int wxDateSpan_GetTotalDays(const wxDateSpan * self) {
    return self->GetTotalDays();
}
int wxDateSpan_GetWeeks(const wxDateSpan * self) {
    return self->GetWeeks();
}
int wxDateSpan_GetYears(const wxDateSpan * self) {
    return self->GetYears();
}
wxDateSpan *wxDateSpan_Multiply(const wxDateSpan * self, int factor) {
    return new wxDateSpan(self->Multiply(factor));
}
wxDateSpan * wxDateSpan_Multiply1(wxDateSpan * self, int factor) {
    return &(self->Multiply(factor));
}
wxDateSpan * wxDateSpan_Neg(wxDateSpan * self) {
    return &(self->Neg());
}
wxDateSpan *wxDateSpan_Negate(const wxDateSpan * self) {
    return new wxDateSpan(self->Negate());
}
wxDateSpan * wxDateSpan_SetDays(wxDateSpan * self, int n) {
    return &(self->SetDays(n));
}
wxDateSpan * wxDateSpan_SetMonths(wxDateSpan * self, int n) {
    return &(self->SetMonths(n));
}
wxDateSpan * wxDateSpan_SetWeeks(wxDateSpan * self, int n) {
    return &(self->SetWeeks(n));
}
wxDateSpan * wxDateSpan_SetYears(wxDateSpan * self, int n) {
    return &(self->SetYears(n));
}
wxDateSpan *wxDateSpan_Subtract(const wxDateSpan * self, const wxDateSpan * other) {
    return new wxDateSpan(self->Subtract(*other));
}
wxDateSpan * wxDateSpan_Subtract1(wxDateSpan * self, const wxDateSpan * other) {
    return &(self->Subtract(*other));
}
wxDateSpan *wxDateSpan_Day() {
    return new wxDateSpan(wxDateSpan::Day());
}
wxDateSpan *wxDateSpan_Days(int days) {
    return new wxDateSpan(wxDateSpan::Days(days));
}
wxDateSpan *wxDateSpan_Month() {
    return new wxDateSpan(wxDateSpan::Month());
}
wxDateSpan *wxDateSpan_Months(int mon) {
    return new wxDateSpan(wxDateSpan::Months(mon));
}
wxDateSpan *wxDateSpan_Week() {
    return new wxDateSpan(wxDateSpan::Week());
}
wxDateSpan *wxDateSpan_Weeks(int weeks) {
    return new wxDateSpan(wxDateSpan::Weeks(weeks));
}
wxDateSpan *wxDateSpan_Year() {
    return new wxDateSpan(wxDateSpan::Year());
}
wxDateSpan *wxDateSpan_Years(int years) {
    return new wxDateSpan(wxDateSpan::Years(years));
}

// CLASS: wxDateTime
void wxDateTime_delete(wxDateTime *self) {
    delete self;
}
wxDateTime *wxDateTime_new() {
    return new wxDateTime();
}
wxDateTime *wxDateTime_new1(const wxDateTime * date) {
    return new wxDateTime(*date);
}
wxDateTime *wxDateTime_new4(double jdn) {
    return new wxDateTime(jdn);
}
wxDateTime * wxDateTime_ResetTime(wxDateTime * self) {
    return &(self->ResetTime());
}
wxDateTime * wxDateTime_Set2(wxDateTime * self, const Tm * tm) {
    return &(self->Set(*tm));
}
wxDateTime * wxDateTime_Set3(wxDateTime * self, double jdn) {
    return &(self->Set(jdn));
}
wxDateTime * wxDateTime_SetToCurrent(wxDateTime * self) {
    return &(self->SetToCurrent());
}
wxDateTime * wxDateTime_SetYear(wxDateTime * self, int year) {
    return &(self->SetYear(year));
}
#ifdef __WXMSW__
wxDateTime * wxDateTime_SetFromMSWSysTime(wxDateTime * self, const struct _SYSTEMTIME * st) {
    return &(self->SetFromMSWSysTime(*st));
}
void wxDateTime_GetAsMSWSysTime(const wxDateTime * self, struct _SYSTEMTIME * st) {
    return self->GetAsMSWSysTime(st);
}
#endif
int wxDateTime_GetCentury(const wxDateTime * self, const TimeZone * tz) {
    return self->GetCentury(*tz);
}
wxDateTime *wxDateTime_GetDateOnly(const wxDateTime * self) {
    return new wxDateTime(self->GetDateOnly());
}
#if wxCHECK_VERSION(3, 1, 0)
int wxDateTime_GetWeekBasedYear(const wxDateTime * self, const TimeZone * tz) {
    return self->GetWeekBasedYear(*tz);
}
#endif
int wxDateTime_GetYear(const wxDateTime * self, const TimeZone * tz) {
    return self->GetYear(*tz);
}
bool wxDateTime_IsValid(const wxDateTime * self) {
    return self->IsValid();
}
bool wxDateTime_IsEarlierThan(const wxDateTime * self, const wxDateTime * datetime) {
    return self->IsEarlierThan(*datetime);
}
bool wxDateTime_IsEqualTo(const wxDateTime * self, const wxDateTime * datetime) {
    return self->IsEqualTo(*datetime);
}
bool wxDateTime_IsEqualUpTo(const wxDateTime * self, const wxDateTime * dt, const wxTimeSpan * ts) {
    return self->IsEqualUpTo(*dt, *ts);
}
bool wxDateTime_IsLaterThan(const wxDateTime * self, const wxDateTime * datetime) {
    return self->IsLaterThan(*datetime);
}
bool wxDateTime_IsSameDate(const wxDateTime * self, const wxDateTime * dt) {
    return self->IsSameDate(*dt);
}
bool wxDateTime_IsSameTime(const wxDateTime * self, const wxDateTime * dt) {
    return self->IsSameTime(*dt);
}
bool wxDateTime_IsStrictlyBetween(const wxDateTime * self, const wxDateTime * t1, const wxDateTime * t2) {
    return self->IsStrictlyBetween(*t1, *t2);
}
bool wxDateTime_IsBetween(const wxDateTime * self, const wxDateTime * t1, const wxDateTime * t2) {
    return self->IsBetween(*t1, *t2);
}
wxDateTime * wxDateTime_Add1(wxDateTime * self, const wxDateSpan * diff) {
    return &(self->Add(*diff));
}
wxDateTime * wxDateTime_Add3(wxDateTime * self, const wxTimeSpan * diff) {
    return &(self->Add(*diff));
}
wxDateTime * wxDateTime_Subtract1(wxDateTime * self, const wxTimeSpan * diff) {
    return &(self->Subtract(*diff));
}
wxDateTime * wxDateTime_Subtract3(wxDateTime * self, const wxDateSpan * diff) {
    return &(self->Subtract(*diff));
}
wxDateSpan *wxDateTime_DiffAsDateSpan(const wxDateTime * self, const wxDateTime * dt) {
    return new wxDateSpan(self->DiffAsDateSpan(*dt));
}
wxString *wxDateTime_Format(const wxDateTime * self, const wxString * format, const TimeZone * tz) {
    return new wxString(self->Format(*format, *tz));
}
wxString *wxDateTime_FormatDate(const wxDateTime * self) {
    return new wxString(self->FormatDate());
}
wxString *wxDateTime_FormatISODate(const wxDateTime * self) {
    return new wxString(self->FormatISODate());
}
wxString *wxDateTime_FormatISOTime(const wxDateTime * self) {
    return new wxString(self->FormatISOTime());
}
wxString *wxDateTime_FormatTime(const wxDateTime * self) {
    return new wxString(self->FormatTime());
}
bool wxDateTime_ParseDateTime(wxDateTime * self, const wxString * datetime, wxString::const_iterator * end) {
    return self->ParseDateTime(*datetime, end);
}
bool wxDateTime_ParseFormat(wxDateTime * self, const wxString * date, const wxString * format, const wxDateTime * date_def, wxString::const_iterator * end) {
    return self->ParseFormat(*date, *format, *date_def, end);
}
bool wxDateTime_ParseISODate(wxDateTime * self, const wxString * date) {
    return self->ParseISODate(*date);
}
bool wxDateTime_ParseISOTime(wxDateTime * self, const wxString * date) {
    return self->ParseISOTime(*date);
}
bool wxDateTime_ParseRfc822Date(wxDateTime * self, const wxString * date, wxString::const_iterator * end) {
    return self->ParseRfc822Date(*date, end);
}
bool wxDateTime_ParseTime(wxDateTime * self, const wxString * time, wxString::const_iterator * end) {
    return self->ParseTime(*time, end);
}
double wxDateTime_GetJDN(const wxDateTime * self) {
    return self->GetJDN();
}
double wxDateTime_GetJulianDayNumber(const wxDateTime * self) {
    return self->GetJulianDayNumber();
}
double wxDateTime_GetMJD(const wxDateTime * self) {
    return self->GetMJD();
}
double wxDateTime_GetModifiedJulianDayNumber(const wxDateTime * self) {
    return self->GetModifiedJulianDayNumber();
}
double wxDateTime_GetRataDie(const wxDateTime * self) {
    return self->GetRataDie();
}
wxDateTime *wxDateTime_FromTimezone(const wxDateTime * self, const TimeZone * tz, bool no_dst) {
    return new wxDateTime(self->FromTimezone(*tz, no_dst));
}
wxDateTime * wxDateTime_MakeFromTimezone(wxDateTime * self, const TimeZone * tz, bool no_dst) {
    return &(self->MakeFromTimezone(*tz, no_dst));
}
wxDateTime * wxDateTime_MakeTimezone(wxDateTime * self, const TimeZone * tz, bool no_dst) {
    return &(self->MakeTimezone(*tz, no_dst));
}
wxDateTime * wxDateTime_MakeUTC(wxDateTime * self, bool no_dst) {
    return &(self->MakeUTC(no_dst));
}
wxDateTime *wxDateTime_ToTimezone(const wxDateTime * self, const TimeZone * tz, bool no_dst) {
    return new wxDateTime(self->ToTimezone(*tz, no_dst));
}
wxDateTime *wxDateTime_ToUTC(const wxDateTime * self, bool no_dst) {
    return new wxDateTime(self->ToUTC(no_dst));
}
int wxDateTime_ConvertYearToBC(int year) {
    return wxDateTime::ConvertYearToBC(year);
}
void wxDateTime_GetAmPmStrings(wxString * am, wxString * pm) {
    return wxDateTime::GetAmPmStrings(am, pm);
}
int wxDateTime_GetCentury1(int year) {
    return wxDateTime::GetCentury(year);
}
tm * wxDateTime_GetTmNow1() {
    return wxDateTime::GetTmNow();
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxDateTime_GetFirstWeekDay(WeekDay * first_day) {
    return wxDateTime::GetFirstWeekDay(first_day);
}
#endif
wxDateTime *wxDateTime_Now() {
    return new wxDateTime(wxDateTime::Now());
}
wxDateTime *wxDateTime_Today() {
    return new wxDateTime(wxDateTime::Today());
}
wxDateTime *wxDateTime_UNow() {
    return new wxDateTime(wxDateTime::UNow());
}

// CLASS: wxDateTimeHolidayAuthority
void wxDateTimeHolidayAuthority_delete(wxDateTimeHolidayAuthority *self) {
    delete self;
}

// CLASS: wxDateTimeWorkDays
void wxDateTimeWorkDays_delete(wxDateTimeWorkDays *self) {
    delete self;
}

// CLASS: wxDebugContext
void wxDebugContext_delete(wxDebugContext *self) {
    delete self;
}
int wxDebugContext_Check(bool check_all) {
    return wxDebugContext::Check(check_all);
}
bool wxDebugContext_Dump() {
    return wxDebugContext::Dump();
}
bool wxDebugContext_GetCheckPrevious() {
    return wxDebugContext::GetCheckPrevious();
}
bool wxDebugContext_GetDebugMode() {
    return wxDebugContext::GetDebugMode();
}
int wxDebugContext_GetLevel() {
    return wxDebugContext::GetLevel();
}
bool wxDebugContext_PrintClasses() {
    return wxDebugContext::PrintClasses();
}
bool wxDebugContext_PrintStatistics(bool detailed) {
    return wxDebugContext::PrintStatistics(detailed);
}
void wxDebugContext_SetCheckPrevious(bool check) {
    return wxDebugContext::SetCheckPrevious(check);
}
void wxDebugContext_SetCheckpoint(bool all) {
    return wxDebugContext::SetCheckpoint(all);
}
void wxDebugContext_SetDebugMode(bool debug) {
    return wxDebugContext::SetDebugMode(debug);
}
void wxDebugContext_SetLevel(int level) {
    return wxDebugContext::SetLevel(level);
}

// CLASS: wxDir
void wxDir_delete(wxDir *self) {
    delete self;
}
wxDir *wxDir_new() {
    return new wxDir();
}
wxDir *wxDir_new1(const wxString * dir) {
    return new wxDir(*dir);
}
void wxDir_Close(wxDir * self) {
    return self->Close();
}
bool wxDir_GetFirst(const wxDir * self, wxString * filename, const wxString * filespec, int flags) {
    return self->GetFirst(filename, *filespec, flags);
}
wxString *wxDir_GetName(const wxDir * self) {
    return new wxString(self->GetName());
}
wxString *wxDir_GetNameWithSep(const wxDir * self) {
    return new wxString(self->GetNameWithSep());
}
bool wxDir_GetNext(const wxDir * self, wxString * filename) {
    return self->GetNext(filename);
}
bool wxDir_HasFiles(const wxDir * self, const wxString * filespec) {
    return self->HasFiles(*filespec);
}
bool wxDir_HasSubDirs(const wxDir * self, const wxString * dirspec) {
    return self->HasSubDirs(*dirspec);
}
bool wxDir_IsOpened(const wxDir * self) {
    return self->IsOpened();
}
bool wxDir_Open(wxDir * self, const wxString * dir) {
    return self->Open(*dir);
}
size_t wxDir_Traverse(const wxDir * self, wxDirTraverser * sink, const wxString * filespec, int flags) {
    return self->Traverse(*sink, *filespec, flags);
}
bool wxDir_Exists(const wxString * dir) {
    return wxDir::Exists(*dir);
}
wxString *wxDir_FindFirst(const wxString * dirname, const wxString * filespec, int flags) {
    return new wxString(wxDir::FindFirst(*dirname, *filespec, flags));
}
size_t wxDir_GetAllFiles(const wxString * dirname, wxArrayString * files, const wxString * filespec, int flags) {
    return wxDir::GetAllFiles(*dirname, files, *filespec, flags);
}
bool wxDir_Make(const wxString * dir, int perm, int flags) {
    return wxDir::Make(*dir, perm, flags);
}
bool wxDir_Remove(const wxString * dir, int flags) {
    return wxDir::Remove(*dir, flags);
}

// CLASS: wxDirTraverser
void wxDirTraverser_delete(wxDirTraverser *self) {
    delete self;
}

// CLASS: wxDynamicLibrary
void wxDynamicLibrary_delete(wxDynamicLibrary *self) {
    delete self;
}
wxDynamicLibrary *wxDynamicLibrary_new() {
    return new wxDynamicLibrary();
}
wxDynamicLibrary *wxDynamicLibrary_new1(const wxString * name, int flags) {
    return new wxDynamicLibrary(*name, flags);
}
void * wxDynamicLibrary_GetSymbol(const wxDynamicLibrary * self, const wxString * name, bool * success) {
    return self->GetSymbol(*name, success);
}
void * wxDynamicLibrary_GetSymbolAorW(const wxDynamicLibrary * self, const wxString * name) {
    return self->GetSymbolAorW(*name);
}
bool wxDynamicLibrary_HasSymbol(const wxDynamicLibrary * self, const wxString * name) {
    return self->HasSymbol(*name);
}
bool wxDynamicLibrary_IsLoaded(const wxDynamicLibrary * self) {
    return self->IsLoaded();
}
bool wxDynamicLibrary_Load(wxDynamicLibrary * self, const wxString * name, int flags) {
    return self->Load(*name, flags);
}
void wxDynamicLibrary_Unload(wxDynamicLibrary * self) {
    return self->Unload();
}
void * wxDynamicLibrary_GetModuleFromAddress(const void * addr, wxString * path) {
    return wxDynamicLibrary::GetModuleFromAddress(addr, path);
}

// CLASS: wxDynamicLibraryDetails
void wxDynamicLibraryDetails_delete(wxDynamicLibraryDetails *self) {
    delete self;
}
bool wxDynamicLibraryDetails_GetAddress(const wxDynamicLibraryDetails * self, void * addr, size_t * len) {
    return self->GetAddress(addr, len);
}
wxString *wxDynamicLibraryDetails_GetName(const wxDynamicLibraryDetails * self) {
    return new wxString(self->GetName());
}
wxString *wxDynamicLibraryDetails_GetPath(const wxDynamicLibraryDetails * self) {
    return new wxString(self->GetPath());
}
wxString *wxDynamicLibraryDetails_GetVersion(const wxDynamicLibraryDetails * self) {
    return new wxString(self->GetVersion());
}

// CLASS: wxEncodingConverter
wxClassInfo *wxEncodingConverter_CLASSINFO() {
    return wxCLASSINFO(wxEncodingConverter);
}
bool wxEncodingConverter_Convert(const wxEncodingConverter * self, const char * input, char * output) {
    return self->Convert(input, output);
}
bool wxEncodingConverter_Convert1(const wxEncodingConverter * self, const wchar_t * input, wchar_t * output) {
    return self->Convert(input, output);
}
bool wxEncodingConverter_Convert2(const wxEncodingConverter * self, const char * input, wchar_t * output) {
    return self->Convert(input, output);
}
bool wxEncodingConverter_Convert3(const wxEncodingConverter * self, const wchar_t * input, char * output) {
    return self->Convert(input, output);
}
bool wxEncodingConverter_Convert4(const wxEncodingConverter * self, char * str) {
    return self->Convert(str);
}
bool wxEncodingConverter_Convert5(const wxEncodingConverter * self, wchar_t * str) {
    return self->Convert(str);
}
wxString *wxEncodingConverter_Convert6(const wxEncodingConverter * self, const wxString * input) {
    return new wxString(self->Convert(*input));
}
wxEncodingConverter *wxEncodingConverter_new() {
    return new wxEncodingConverter();
}

// CLASS: wxEvent
wxClassInfo *wxEvent_CLASSINFO() {
    return wxCLASSINFO(wxEvent);
}
wxEvent * wxEvent_Clone(const wxEvent * self) {
    return self->Clone();
}
wxObject * wxEvent_GetEventObject(const wxEvent * self) {
    return self->GetEventObject();
}
int wxEvent_GetId(const wxEvent * self) {
    return self->GetId();
}
wxObject * wxEvent_GetEventUserData(const wxEvent * self) {
    return self->GetEventUserData();
}
bool wxEvent_GetSkipped(const wxEvent * self) {
    return self->GetSkipped();
}
long wxEvent_GetTimestamp(const wxEvent * self) {
    return self->GetTimestamp();
}
bool wxEvent_IsCommandEvent(const wxEvent * self) {
    return self->IsCommandEvent();
}
void wxEvent_ResumePropagation(wxEvent * self, int propagation_level) {
    return self->ResumePropagation(propagation_level);
}
void wxEvent_SetEventObject(wxEvent * self, wxObject * object) {
    return self->SetEventObject(object);
}
void wxEvent_SetId(wxEvent * self, int id) {
    return self->SetId(id);
}
void wxEvent_SetTimestamp(wxEvent * self, long time_stamp) {
    return self->SetTimestamp(time_stamp);
}
bool wxEvent_ShouldPropagate(const wxEvent * self) {
    return self->ShouldPropagate();
}
void wxEvent_Skip(wxEvent * self, bool skip) {
    return self->Skip(skip);
}
int wxEvent_StopPropagation(wxEvent * self) {
    return self->StopPropagation();
}

// CLASS: wxEvtHandler
wxClassInfo *wxEvtHandler_CLASSINFO() {
    return wxCLASSINFO(wxEvtHandler);
}
void wxEvtHandler_QueueEvent(wxEvtHandler * self, wxEvent * event) {
    return self->QueueEvent(event);
}
void wxEvtHandler_AddPendingEvent(wxEvtHandler * self, const wxEvent * event) {
    return self->AddPendingEvent(*event);
}
bool wxEvtHandler_ProcessEvent(wxEvtHandler * self, wxEvent * event) {
    return self->ProcessEvent(*event);
}
bool wxEvtHandler_ProcessEventLocally(wxEvtHandler * self, wxEvent * event) {
    return self->ProcessEventLocally(*event);
}
bool wxEvtHandler_SafelyProcessEvent(wxEvtHandler * self, wxEvent * event) {
    return self->SafelyProcessEvent(*event);
}
void wxEvtHandler_ProcessPendingEvents(wxEvtHandler * self) {
    return self->ProcessPendingEvents();
}
void wxEvtHandler_DeletePendingEvents(wxEvtHandler * self) {
    return self->DeletePendingEvents();
}
wxClientData * wxEvtHandler_GetClientObject(const wxEvtHandler * self) {
    return self->GetClientObject();
}
void wxEvtHandler_SetClientObject(wxEvtHandler * self, wxClientData * data) {
    return self->SetClientObject(data);
}
bool wxEvtHandler_GetEvtHandlerEnabled(const wxEvtHandler * self) {
    return self->GetEvtHandlerEnabled();
}
wxEvtHandler * wxEvtHandler_GetNextHandler(const wxEvtHandler * self) {
    return self->GetNextHandler();
}
wxEvtHandler * wxEvtHandler_GetPreviousHandler(const wxEvtHandler * self) {
    return self->GetPreviousHandler();
}
void wxEvtHandler_SetEvtHandlerEnabled(wxEvtHandler * self, bool enabled) {
    return self->SetEvtHandlerEnabled(enabled);
}
void wxEvtHandler_SetNextHandler(wxEvtHandler * self, wxEvtHandler * handler) {
    return self->SetNextHandler(handler);
}
void wxEvtHandler_SetPreviousHandler(wxEvtHandler * self, wxEvtHandler * handler) {
    return self->SetPreviousHandler(handler);
}
void wxEvtHandler_Unlink(wxEvtHandler * self) {
    return self->Unlink();
}
bool wxEvtHandler_IsUnlinked(const wxEvtHandler * self) {
    return self->IsUnlinked();
}
void wxEvtHandler_AddFilter(wxEventFilter * filter) {
    return wxEvtHandler::AddFilter(filter);
}
void wxEvtHandler_RemoveFilter(wxEventFilter * filter) {
    return wxEvtHandler::RemoveFilter(filter);
}
wxEvtHandler *wxEvtHandler_new() {
    return new wxEvtHandler();
}
// Mix-in(s) to wxEvtHandler
wxTrackable *wxEvtHandler_AsTrackable(wxEvtHandler* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxFFile
void wxFFile_delete(wxFFile *self) {
    delete self;
}
wxFFile *wxFFile_new() {
    return new wxFFile();
}
wxFFile *wxFFile_new1(FILE * fp) {
    return new wxFFile(fp);
}
wxFFile *wxFFile_new2(const wxString * filename, const wxString * mode) {
    return new wxFFile(*filename, *mode);
}
void wxFFile_Attach(wxFFile * self, FILE * fp, const wxString * name) {
    return self->Attach(fp, *name);
}
bool wxFFile_Close(wxFFile * self) {
    return self->Close();
}
FILE * wxFFile_Detach(wxFFile * self) {
    return self->Detach();
}
bool wxFFile_Eof(const wxFFile * self) {
    return self->Eof();
}
bool wxFFile_Error(const wxFFile * self) {
    return self->Error();
}
bool wxFFile_Flush(wxFFile * self) {
    return self->Flush();
}
wxString *wxFFile_GetName(const wxFFile * self) {
    return new wxString(self->GetName());
}
bool wxFFile_IsOpened(const wxFFile * self) {
    return self->IsOpened();
}
bool wxFFile_Open(wxFFile * self, const wxString * filename, const wxString * mode) {
    return self->Open(*filename, *mode);
}
size_t wxFFile_Read(wxFFile * self, void * buffer, size_t count) {
    return self->Read(buffer, count);
}
bool wxFFile_ReadAll(wxFFile * self, wxString * str, const wxMBConv * conv) {
    return self->ReadAll(str, *conv);
}
bool wxFFile_Write(wxFFile * self, const wxString * str, const wxMBConv * conv) {
    return self->Write(*str, *conv);
}
size_t wxFFile_Write1(wxFFile * self, const void * buffer, size_t count) {
    return self->Write(buffer, count);
}
FILE * wxFFile_fp(const wxFFile * self) {
    return self->fp();
}

// CLASS: wxFSFile
wxClassInfo *wxFSFile_CLASSINFO() {
    return wxCLASSINFO(wxFSFile);
}
wxFSFile *wxFSFile_new(wxInputStream * stream, const wxString * location, const wxString * mimetype, const wxString * anchor, wxDateTime modif) {
    return new wxFSFile(stream, *location, *mimetype, *anchor, modif);
}
wxInputStream * wxFSFile_DetachStream(wxFSFile * self) {
    return self->DetachStream();
}
wxString *wxFSFile_GetAnchor(const wxFSFile * self) {
    return new wxString(self->GetAnchor());
}
wxString *wxFSFile_GetLocation(const wxFSFile * self) {
    return new wxString(self->GetLocation());
}
wxString *wxFSFile_GetMimeType(const wxFSFile * self) {
    return new wxString(self->GetMimeType());
}
wxDateTime *wxFSFile_GetModificationTime(const wxFSFile * self) {
    return new wxDateTime(self->GetModificationTime());
}
wxInputStream * wxFSFile_GetStream(const wxFSFile * self) {
    return self->GetStream();
}

// CLASS: wxFSVolume
void wxFSVolume_delete(wxFSVolume *self) {
    delete self;
}
wxFSVolume *wxFSVolume_new() {
    return new wxFSVolume();
}
wxFSVolume *wxFSVolume_new1(const wxString * name) {
    return new wxFSVolume(*name);
}
bool wxFSVolume_Create(wxFSVolume * self, const wxString * name) {
    return self->Create(*name);
}
bool wxFSVolume_IsOk(const wxFSVolume * self) {
    return self->IsOk();
}
int wxFSVolume_GetFlags(const wxFSVolume * self) {
    return self->GetFlags();
}
bool wxFSVolume_IsWritable(const wxFSVolume * self) {
    return self->IsWritable();
}
wxString *wxFSVolume_GetName(const wxFSVolume * self) {
    return new wxString(self->GetName());
}
wxString *wxFSVolume_GetDisplayName(const wxFSVolume * self) {
    return new wxString(self->GetDisplayName());
}
wxArrayString *wxFSVolume_GetVolumes(int flags_set, int flags_unset) {
    return new wxArrayString(wxFSVolume::GetVolumes(flags_set, flags_unset));
}
void wxFSVolume_CancelSearch() {
    return wxFSVolume::CancelSearch();
}

// CLASS: wxFile
void wxFile_delete(wxFile *self) {
    delete self;
}
wxFile *wxFile_new() {
    return new wxFile();
}
wxFile *wxFile_new2(int fd) {
    return new wxFile(fd);
}
int wxFile_GetLastError(const wxFile * self) {
    return self->GetLastError();
}
void wxFile_ClearLastError(wxFile * self) {
    return self->ClearLastError();
}
void wxFile_Attach(wxFile * self, int fd) {
    return self->Attach(fd);
}
bool wxFile_Close(wxFile * self) {
    return self->Close();
}
bool wxFile_Create(wxFile * self, const wxString * filename, bool overwrite, int access) {
    return self->Create(*filename, overwrite, access);
}
int wxFile_Detach(wxFile * self) {
    return self->Detach();
}
bool wxFile_Eof(const wxFile * self) {
    return self->Eof();
}
bool wxFile_Flush(wxFile * self) {
    return self->Flush();
}
bool wxFile_IsOpened(const wxFile * self) {
    return self->IsOpened();
}
bool wxFile_ReadAll(wxFile * self, wxString * str, const wxMBConv * conv) {
    return self->ReadAll(str, *conv);
}
size_t wxFile_Write(wxFile * self, const void * buffer, size_t count) {
    return self->Write(buffer, count);
}
bool wxFile_Write1(wxFile * self, const wxString * s, const wxMBConv * conv) {
    return self->Write(*s, *conv);
}
int wxFile_fd(const wxFile * self) {
    return self->fd();
}
bool wxFile_Exists(const wxString * filename) {
    return wxFile::Exists(*filename);
}

// CLASS: wxFileName
void wxFileName_delete(wxFileName *self) {
    delete self;
}
wxFileName *wxFileName_new() {
    return new wxFileName();
}
wxFileName *wxFileName_new1(const wxFileName * filename) {
    return new wxFileName(*filename);
}
bool wxFileName_AppendDir(wxFileName * self, const wxString * dir) {
    return self->AppendDir(*dir);
}
void wxFileName_Assign(wxFileName * self, const wxFileName * filepath) {
    return self->Assign(*filepath);
}
void wxFileName_AssignCwd(wxFileName * self, const wxString * volume) {
    return self->AssignCwd(*volume);
}
void wxFileName_AssignHomeDir(wxFileName * self) {
    return self->AssignHomeDir();
}
void wxFileName_AssignTempFileName(wxFileName * self, const wxString * prefix) {
    return self->AssignTempFileName(*prefix);
}
void wxFileName_AssignTempFileName1(wxFileName * self, const wxString * prefix, wxFile * file_temp) {
    return self->AssignTempFileName(*prefix, file_temp);
}
void wxFileName_AssignTempFileName2(wxFileName * self, const wxString * prefix, wxFFile * file_temp) {
    return self->AssignTempFileName(*prefix, file_temp);
}
void wxFileName_Clear(wxFileName * self) {
    return self->Clear();
}
void wxFileName_ClearExt(wxFileName * self) {
    return self->ClearExt();
}
bool wxFileName_DirExists(const wxFileName * self) {
    return self->DirExists();
}
void wxFileName_DontFollowLink(wxFileName * self) {
    return self->DontFollowLink();
}
bool wxFileName_Exists(const wxFileName * self, int flags) {
    return self->Exists(flags);
}
bool wxFileName_FileExists(const wxFileName * self) {
    return self->FileExists();
}
size_t wxFileName_GetDirCount(const wxFileName * self) {
    return self->GetDirCount();
}
wxArrayString *wxFileName_GetDirs(const wxFileName * self) {
    return new wxArrayString(self->GetDirs());
}
wxString *wxFileName_GetExt(const wxFileName * self) {
    return new wxString(self->GetExt());
}
wxString *wxFileName_GetFullName(const wxFileName * self) {
    return new wxString(self->GetFullName());
}
wxString *wxFileName_GetLongPath(const wxFileName * self) {
    return new wxString(self->GetLongPath());
}
wxDateTime *wxFileName_GetModificationTime(const wxFileName * self) {
    return new wxDateTime(self->GetModificationTime());
}
wxString *wxFileName_GetName(const wxFileName * self) {
    return new wxString(self->GetName());
}
wxString *wxFileName_GetShortPath(const wxFileName * self) {
    return new wxString(self->GetShortPath());
}
bool wxFileName_GetTimes(const wxFileName * self, wxDateTime * dt_access, wxDateTime * dt_mod, wxDateTime * dt_create) {
    return self->GetTimes(dt_access, dt_mod, dt_create);
}
wxString *wxFileName_GetVolume(const wxFileName * self) {
    return new wxString(self->GetVolume());
}
bool wxFileName_HasExt(const wxFileName * self) {
    return self->HasExt();
}
bool wxFileName_HasName(const wxFileName * self) {
    return self->HasName();
}
bool wxFileName_HasVolume(const wxFileName * self) {
    return self->HasVolume();
}
bool wxFileName_InsertDir(wxFileName * self, size_t before, const wxString * dir) {
    return self->InsertDir(before, *dir);
}
bool wxFileName_IsDir(const wxFileName * self) {
    return self->IsDir();
}
bool wxFileName_IsDirReadable(const wxFileName * self) {
    return self->IsDirReadable();
}
bool wxFileName_IsDirWritable(const wxFileName * self) {
    return self->IsDirWritable();
}
bool wxFileName_IsFileExecutable(const wxFileName * self) {
    return self->IsFileExecutable();
}
bool wxFileName_IsFileReadable(const wxFileName * self) {
    return self->IsFileReadable();
}
bool wxFileName_IsFileWritable(const wxFileName * self) {
    return self->IsFileWritable();
}
bool wxFileName_IsOk(const wxFileName * self) {
    return self->IsOk();
}
bool wxFileName_Mkdir(const wxFileName * self, int perm, int flags) {
    return self->Mkdir(perm, flags);
}
void wxFileName_PrependDir(wxFileName * self, const wxString * dir) {
    return self->PrependDir(*dir);
}
void wxFileName_RemoveDir(wxFileName * self, size_t pos) {
    return self->RemoveDir(pos);
}
void wxFileName_RemoveLastDir(wxFileName * self) {
    return self->RemoveLastDir();
}
#if wxCHECK_VERSION(3, 1, 0)
wxFileName *wxFileName_ResolveLink(wxFileName * self) {
    return new wxFileName(self->ResolveLink());
}
#endif
bool wxFileName_Rmdir(const wxFileName * self, int flags) {
    return self->Rmdir(flags);
}
bool wxFileName_SetCwd(const wxFileName * self) {
    return self->SetCwd();
}
void wxFileName_SetEmptyExt(wxFileName * self) {
    return self->SetEmptyExt();
}
void wxFileName_SetExt(wxFileName * self, const wxString * ext) {
    return self->SetExt(*ext);
}
void wxFileName_SetFullName(wxFileName * self, const wxString * fullname) {
    return self->SetFullName(*fullname);
}
void wxFileName_SetName(wxFileName * self, const wxString * name) {
    return self->SetName(*name);
}
bool wxFileName_SetPermissions(wxFileName * self, int permissions) {
    return self->SetPermissions(permissions);
}
bool wxFileName_SetTimes(const wxFileName * self, const wxDateTime * dt_access, const wxDateTime * dt_mod, const wxDateTime * dt_create) {
    return self->SetTimes(dt_access, dt_mod, dt_create);
}
void wxFileName_SetVolume(wxFileName * self, const wxString * volume) {
    return self->SetVolume(*volume);
}
bool wxFileName_ShouldFollowLink(const wxFileName * self) {
    return self->ShouldFollowLink();
}
bool wxFileName_Touch(const wxFileName * self) {
    return self->Touch();
}
wxString *wxFileName_CreateTempFileName(const wxString * prefix, wxFile * file_temp) {
    return new wxString(wxFileName::CreateTempFileName(*prefix, file_temp));
}
wxString *wxFileName_CreateTempFileName1(const wxString * prefix, wxFFile * file_temp) {
    return new wxString(wxFileName::CreateTempFileName(*prefix, file_temp));
}
bool wxFileName_DirExists1(const wxString * dir) {
    return wxFileName::DirExists(*dir);
}
bool wxFileName_Exists1(const wxString * path, int flags) {
    return wxFileName::Exists(*path, flags);
}
bool wxFileName_FileExists1(const wxString * file) {
    return wxFileName::FileExists(*file);
}
wxString *wxFileName_GetCwd(const wxString * volume) {
    return new wxString(wxFileName::GetCwd(*volume));
}
wxString *wxFileName_GetHomeDir() {
    return new wxString(wxFileName::GetHomeDir());
}
wxString *wxFileName_GetTempDir() {
    return new wxString(wxFileName::GetTempDir());
}
bool wxFileName_IsDirReadable1(const wxString * dir) {
    return wxFileName::IsDirReadable(*dir);
}
bool wxFileName_IsDirWritable1(const wxString * dir) {
    return wxFileName::IsDirWritable(*dir);
}
bool wxFileName_IsFileExecutable1(const wxString * file) {
    return wxFileName::IsFileExecutable(*file);
}
bool wxFileName_IsFileReadable1(const wxString * file) {
    return wxFileName::IsFileReadable(*file);
}
bool wxFileName_IsFileWritable1(const wxString * file) {
    return wxFileName::IsFileWritable(*file);
}
bool wxFileName_Mkdir1(const wxString * dir, int perm, int flags) {
    return wxFileName::Mkdir(*dir, perm, flags);
}
bool wxFileName_Rmdir1(const wxString * dir, int flags) {
    return wxFileName::Rmdir(*dir, flags);
}
bool wxFileName_SetCwd1(const wxString * cwd) {
    return wxFileName::SetCwd(*cwd);
}
#if wxCHECK_VERSION(3, 1, 0)
wxFileName *wxFileName_URLToFileName(const wxString * url) {
    return new wxFileName(wxFileName::URLToFileName(*url));
}
wxString *wxFileName_FileNameToURL(const wxFileName * filename) {
    return new wxString(wxFileName::FileNameToURL(*filename));
}
#endif
wxString *wxFileName_StripExtension(const wxString * fullname) {
    return new wxString(wxFileName::StripExtension(*fullname));
}

// CLASS: wxFileSystem
wxClassInfo *wxFileSystem_CLASSINFO() {
    return wxCLASSINFO(wxFileSystem);
}
wxFileSystem *wxFileSystem_new() {
    return new wxFileSystem();
}
void wxFileSystem_ChangePathTo(wxFileSystem * self, const wxString * location, bool is_dir) {
    return self->ChangePathTo(*location, is_dir);
}
bool wxFileSystem_FindFileInPath(wxFileSystem * self, wxString * p_str, const wxString * path, const wxString * file) {
    return self->FindFileInPath(p_str, *path, *file);
}
wxString *wxFileSystem_FindFirst(wxFileSystem * self, const wxString * wildcard, int flags) {
    return new wxString(self->FindFirst(*wildcard, flags));
}
wxString *wxFileSystem_FindNext(wxFileSystem * self) {
    return new wxString(self->FindNext());
}
wxString *wxFileSystem_GetPath(const wxFileSystem * self) {
    return new wxString(self->GetPath());
}
wxFSFile * wxFileSystem_OpenFile(wxFileSystem * self, const wxString * location, int flags) {
    return self->OpenFile(*location, flags);
}
void wxFileSystem_AddHandler(wxFileSystemHandler * handler) {
    return wxFileSystem::AddHandler(handler);
}
wxFileSystemHandler * wxFileSystem_RemoveHandler(wxFileSystemHandler * handler) {
    return wxFileSystem::RemoveHandler(handler);
}
wxString *wxFileSystem_FileNameToURL(const wxFileName * filename) {
    return new wxString(wxFileSystem::FileNameToURL(*filename));
}
bool wxFileSystem_HasHandlerForPath(const wxString * location) {
    return wxFileSystem::HasHandlerForPath(*location);
}
wxFileName *wxFileSystem_URLToFileName(const wxString * url) {
    return new wxFileName(wxFileSystem::URLToFileName(*url));
}

// CLASS: wxFileSystemHandler
wxClassInfo *wxFileSystemHandler_CLASSINFO() {
    return wxCLASSINFO(wxFileSystemHandler);
}
wxFileSystemHandler *wxFileSystemHandler_new() {
    return new wxFileSystemHandler();
}
bool wxFileSystemHandler_CanOpen(wxFileSystemHandler * self, const wxString * location) {
    return self->CanOpen(*location);
}
wxString *wxFileSystemHandler_FindFirst(wxFileSystemHandler * self, const wxString * wildcard, int flags) {
    return new wxString(self->FindFirst(*wildcard, flags));
}
wxString *wxFileSystemHandler_FindNext(wxFileSystemHandler * self) {
    return new wxString(self->FindNext());
}
wxFSFile * wxFileSystemHandler_OpenFile(wxFileSystemHandler * self, wxFileSystem * fs, const wxString * location) {
    return self->OpenFile(*fs, *location);
}
wxString *wxFileSystemHandler_GetMimeTypeFromExt(const wxString * location) {
    return new wxString(wxFileSystemHandler::GetMimeTypeFromExt(*location));
}

// CLASS: wxFileSystemWatcher
wxClassInfo *wxFileSystemWatcher_CLASSINFO() {
    return wxCLASSINFO(wxFileSystemWatcher);
}
wxFileSystemWatcher *wxFileSystemWatcher_new() {
    return new wxFileSystemWatcher();
}
bool wxFileSystemWatcher_Add(wxFileSystemWatcher * self, const wxFileName * path, int events) {
    return self->Add(*path, events);
}
bool wxFileSystemWatcher_AddTree(wxFileSystemWatcher * self, const wxFileName * path, int events, const wxString * filter) {
    return self->AddTree(*path, events, *filter);
}
bool wxFileSystemWatcher_Remove(wxFileSystemWatcher * self, const wxFileName * path) {
    return self->Remove(*path);
}
bool wxFileSystemWatcher_RemoveTree(wxFileSystemWatcher * self, const wxFileName * path) {
    return self->RemoveTree(*path);
}
bool wxFileSystemWatcher_RemoveAll(wxFileSystemWatcher * self) {
    return self->RemoveAll();
}
int wxFileSystemWatcher_GetWatchedPathsCount(const wxFileSystemWatcher * self) {
    return self->GetWatchedPathsCount();
}
int wxFileSystemWatcher_GetWatchedPaths(const wxFileSystemWatcher * self, wxArrayString * paths) {
    return self->GetWatchedPaths(paths);
}
void wxFileSystemWatcher_SetOwner(wxFileSystemWatcher * self, wxEvtHandler * handler) {
    return self->SetOwner(handler);
}
// Mix-in(s) to wxFileSystemWatcher
wxTrackable *wxFileSystemWatcher_AsTrackable(wxFileSystemWatcher* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxFileSystemWatcherEvent
wxClassInfo *wxFileSystemWatcherEvent_CLASSINFO() {
    return wxCLASSINFO(wxFileSystemWatcherEvent);
}
wxFileSystemWatcherEvent *wxFileSystemWatcherEvent_new(int change_type, int watchid) {
    return new wxFileSystemWatcherEvent(change_type, watchid);
}
wxFileSystemWatcherEvent *wxFileSystemWatcherEvent_new2(int change_type, const wxFileName * path, const wxFileName * new_path, int watchid) {
    return new wxFileSystemWatcherEvent(change_type, *path, *new_path, watchid);
}
wxFileName *wxFileSystemWatcherEvent_GetPath(const wxFileSystemWatcherEvent * self) {
    return new wxFileName(self->GetPath());
}
wxFileName *wxFileSystemWatcherEvent_GetNewPath(const wxFileSystemWatcherEvent * self) {
    return new wxFileName(self->GetNewPath());
}
int wxFileSystemWatcherEvent_GetChangeType(const wxFileSystemWatcherEvent * self) {
    return self->GetChangeType();
}
bool wxFileSystemWatcherEvent_IsError(const wxFileSystemWatcherEvent * self) {
    return self->IsError();
}
wxString *wxFileSystemWatcherEvent_GetErrorDescription(const wxFileSystemWatcherEvent * self) {
    return new wxString(self->GetErrorDescription());
}
wxString *wxFileSystemWatcherEvent_ToString(const wxFileSystemWatcherEvent * self) {
    return new wxString(self->ToString());
}

// CLASS: wxFileType
void wxFileType_delete(wxFileType *self) {
    delete self;
}
wxFileType *wxFileType_new(const wxFileTypeInfo * ft_info) {
    return new wxFileType(*ft_info);
}
bool wxFileType_GetDescription(const wxFileType * self, wxString * desc) {
    return self->GetDescription(desc);
}
bool wxFileType_GetExtensions(wxFileType * self, wxArrayString * extensions) {
    return self->GetExtensions(*extensions);
}
bool wxFileType_GetIcon(const wxFileType * self, wxIconLocation * icon_loc) {
    return self->GetIcon(icon_loc);
}
bool wxFileType_GetMimeType(const wxFileType * self, wxString * mime_type) {
    return self->GetMimeType(mime_type);
}
bool wxFileType_GetMimeTypes(const wxFileType * self, wxArrayString * mime_types) {
    return self->GetMimeTypes(*mime_types);
}
bool wxFileType_GetOpenCommand(wxFileType * self, wxString * command, const MessageParameters * params) {
    return self->GetOpenCommand(command, *params);
}
wxString *wxFileType_GetOpenCommand1(const wxFileType * self, const wxString * filename) {
    return new wxString(self->GetOpenCommand(*filename));
}
bool wxFileType_GetPrintCommand(const wxFileType * self, wxString * command, const MessageParameters * params) {
    return self->GetPrintCommand(command, *params);
}
wxString *wxFileType_GetExpandedCommand(const wxFileType * self, const wxString * verb, const wxFileType::MessageParameters * params) {
    return new wxString(self->GetExpandedCommand(*verb, *params));
}
size_t wxFileType_GetAllCommands(const wxFileType * self, wxArrayString * verbs, wxArrayString * commands, const wxFileType::MessageParameters * params) {
    return self->GetAllCommands(verbs, commands, *params);
}
wxString *wxFileType_ExpandCommand(const wxString * command, const MessageParameters * params) {
    return new wxString(wxFileType::ExpandCommand(*command, *params));
}

// CLASS: wxFilterClassFactory
wxClassInfo *wxFilterClassFactory_CLASSINFO() {
    return wxCLASSINFO(wxFilterClassFactory);
}
const wxFilterClassFactory * wxFilterClassFactory_GetNext(const wxFilterClassFactory * self) {
    return self->GetNext();
}
wxString *wxFilterClassFactory_GetProtocol(const wxFilterClassFactory * self) {
    return new wxString(self->GetProtocol());
}
wxFilterInputStream * wxFilterClassFactory_NewStream(const wxFilterClassFactory * self, wxInputStream * stream) {
    return self->NewStream(*stream);
}
wxFilterOutputStream * wxFilterClassFactory_NewStream1(const wxFilterClassFactory * self, wxOutputStream * stream) {
    return self->NewStream(*stream);
}
wxFilterInputStream * wxFilterClassFactory_NewStream2(const wxFilterClassFactory * self, wxInputStream * stream) {
    return self->NewStream(stream);
}
wxFilterOutputStream * wxFilterClassFactory_NewStream3(const wxFilterClassFactory * self, wxOutputStream * stream) {
    return self->NewStream(stream);
}
wxString *wxFilterClassFactory_PopExtension(const wxFilterClassFactory * self, const wxString * location) {
    return new wxString(self->PopExtension(*location));
}
void wxFilterClassFactory_PushFront(wxFilterClassFactory * self) {
    return self->PushFront();
}
void wxFilterClassFactory_Remove(wxFilterClassFactory * self) {
    return self->Remove();
}
const wxFilterClassFactory * wxFilterClassFactory_GetFirst() {
    return wxFilterClassFactory::GetFirst();
}

// CLASS: wxHashMap
void wxHashMap_delete(wxHashMap *self) {
    delete self;
}
wxHashMap *wxHashMap_new1(const wxHashMap * map) {
    return new wxHashMap(*map);
}
void wxHashMap_clear(wxHashMap * self) {
    return self->clear();
}
bool wxHashMap_empty(const wxHashMap * self) {
    return self->empty();
}

// CLASS: wxHashSet
void wxHashSet_delete(wxHashSet *self) {
    delete self;
}
wxHashSet *wxHashSet_new1(const wxHashSet * set) {
    return new wxHashSet(*set);
}
void wxHashSet_clear(wxHashSet * self) {
    return self->clear();
}
bool wxHashSet_empty(const wxHashSet * self) {
    return self->empty();
}

// CLASS: wxHashTable
wxClassInfo *wxHashTable_CLASSINFO() {
    return wxCLASSINFO(wxHashTable);
}
void wxHashTable_BeginFind(wxHashTable * self) {
    return self->BeginFind();
}
void wxHashTable_Clear(wxHashTable * self) {
    return self->Clear();
}
wxObject * wxHashTable_Delete(wxHashTable * self, long key) {
    return self->Delete(key);
}
wxObject * wxHashTable_Delete1(wxHashTable * self, const wxString * key) {
    return self->Delete(*key);
}
void wxHashTable_DeleteContents(wxHashTable * self, bool flag) {
    return self->DeleteContents(flag);
}
wxObject * wxHashTable_Get(wxHashTable * self, long key) {
    return self->Get(key);
}
wxObject * wxHashTable_Get1(wxHashTable * self, const char * key) {
    return self->Get(key);
}
size_t wxHashTable_GetCount(const wxHashTable * self) {
    return self->GetCount();
}
wxHashTable::Node * wxHashTable_Next(wxHashTable * self) {
    return self->Next();
}
void wxHashTable_Put(wxHashTable * self, long key, wxObject * object) {
    return self->Put(key, object);
}
void wxHashTable_Put1(wxHashTable * self, const char * key, wxObject * object) {
    return self->Put(key, object);
}
long wxHashTable_MakeKey(const wxString * string) {
    return wxHashTable::MakeKey(*string);
}

// CLASS: wxIconLocation
void wxIconLocation_delete(wxIconLocation *self) {
    delete self;
}
bool wxIconLocation_IsOk(const wxIconLocation * self) {
    return self->IsOk();
}
void wxIconLocation_SetFileName(wxIconLocation * self, const wxString * filename) {
    return self->SetFileName(*filename);
}
wxString *wxIconLocation_GetFileName(const wxIconLocation * self) {
    return new wxString(self->GetFileName());
}

// CLASS: wxIdleEvent
wxClassInfo *wxIdleEvent_CLASSINFO() {
    return wxCLASSINFO(wxIdleEvent);
}
wxIdleEvent *wxIdleEvent_new() {
    return new wxIdleEvent();
}
bool wxIdleEvent_MoreRequested(const wxIdleEvent * self) {
    return self->MoreRequested();
}
void wxIdleEvent_RequestMore(wxIdleEvent * self, bool need_more) {
    return self->RequestMore(need_more);
}

// CLASS: wxInitializer
void wxInitializer_delete(wxInitializer *self) {
    delete self;
}
wxInitializer *wxInitializer_new(int argc, wxChar ** argv) {
    return new wxInitializer(argc, argv);
}
bool wxInitializer_IsOk(const wxInitializer * self) {
    return self->IsOk();
}

// CLASS: wxLocale
void wxLocale_delete(wxLocale *self) {
    delete self;
}
wxLocale *wxLocale_new() {
    return new wxLocale();
}
wxLocale *wxLocale_new1(int language, int flags) {
    return new wxLocale(language, flags);
}
wxLocale *wxLocale_new2(const wxString * name, const wxString * short_name, const wxString * locale, bool b_load_default) {
    return new wxLocale(*name, *short_name, *locale, b_load_default);
}
bool wxLocale_AddCatalog(wxLocale * self, const wxString * domain) {
    return self->AddCatalog(*domain);
}
wxString *wxLocale_GetCanonicalName(const wxLocale * self) {
    return new wxString(self->GetCanonicalName());
}
wxString *wxLocale_GetHeaderValue(const wxLocale * self, const wxString * header, const wxString * domain) {
    return new wxString(self->GetHeaderValue(*header, *domain));
}
int wxLocale_GetLanguage(const wxLocale * self) {
    return self->GetLanguage();
}
wxString *wxLocale_GetLocale(const wxLocale * self) {
    return new wxString(self->GetLocale());
}
wxString *wxLocale_GetName(const wxLocale * self) {
    return new wxString(self->GetName());
}
wxString *wxLocale_GetString(const wxLocale * self, const wxString * orig_string, const wxString * domain) {
    return new wxString(self->GetString(*orig_string, *domain));
}
wxString *wxLocale_GetSysName(const wxLocale * self) {
    return new wxString(self->GetSysName());
}
bool wxLocale_Init(wxLocale * self, int language, int flags) {
    return self->Init(language, flags);
}
bool wxLocale_Init1(wxLocale * self, const wxString * name, const wxString * short_name, const wxString * locale, bool b_load_default) {
    return self->Init(*name, *short_name, *locale, b_load_default);
}
bool wxLocale_IsLoaded(const wxLocale * self, const wxString * domain) {
    return self->IsLoaded(*domain);
}
bool wxLocale_IsOk(const wxLocale * self) {
    return self->IsOk();
}
void wxLocale_AddCatalogLookupPathPrefix(const wxString * prefix) {
    return wxLocale::AddCatalogLookupPathPrefix(*prefix);
}
void wxLocale_AddLanguage(const wxLanguageInfo * info) {
    return wxLocale::AddLanguage(*info);
}
const wxLanguageInfo * wxLocale_FindLanguageInfo(const wxString * locale) {
    return wxLocale::FindLanguageInfo(*locale);
}
const wxLanguageInfo * wxLocale_GetLanguageInfo(int lang) {
    return wxLocale::GetLanguageInfo(lang);
}
wxString *wxLocale_GetLanguageName(int lang) {
    return new wxString(wxLocale::GetLanguageName(lang));
}
wxString *wxLocale_GetLanguageCanonicalName(int lang) {
    return new wxString(wxLocale::GetLanguageCanonicalName(lang));
}
wxString *wxLocale_GetSystemEncodingName() {
    return new wxString(wxLocale::GetSystemEncodingName());
}
int wxLocale_GetSystemLanguage() {
    return wxLocale::GetSystemLanguage();
}
bool wxLocale_IsAvailable(int lang) {
    return wxLocale::IsAvailable(lang);
}

// CLASS: wxLog
void wxLog_delete(wxLog *self) {
    delete self;
}
void wxLog_AddTraceMask(const wxString * mask) {
    return wxLog::AddTraceMask(*mask);
}
void wxLog_ClearTraceMasks() {
    return wxLog::ClearTraceMasks();
}
wxArrayString *wxLog_GetTraceMasks() {
    return new wxArrayString(wxLog::GetTraceMasks());
}
bool wxLog_IsAllowedTraceMask(const wxString * mask) {
    return wxLog::IsAllowedTraceMask(*mask);
}
void wxLog_RemoveTraceMask(const wxString * mask) {
    return wxLog::RemoveTraceMask(*mask);
}
void wxLog_DontCreateOnDemand() {
    return wxLog::DontCreateOnDemand();
}
wxLog * wxLog_GetActiveTarget() {
    return wxLog::GetActiveTarget();
}
wxLog * wxLog_SetActiveTarget(wxLog * logtarget) {
    return wxLog::SetActiveTarget(logtarget);
}
wxLog * wxLog_SetThreadActiveTarget(wxLog * logger) {
    return wxLog::SetThreadActiveTarget(logger);
}
void wxLog_FlushActive() {
    return wxLog::FlushActive();
}
void wxLog_Resume() {
    return wxLog::Resume();
}
void wxLog_Suspend() {
    return wxLog::Suspend();
}
bool wxLog_EnableLogging(bool enable) {
    return wxLog::EnableLogging(enable);
}
bool wxLog_IsEnabled() {
    return wxLog::IsEnabled();
}
bool wxLog_GetRepetitionCounting() {
    return wxLog::GetRepetitionCounting();
}
void wxLog_SetRepetitionCounting(bool repet_counting) {
    return wxLog::SetRepetitionCounting(repet_counting);
}
wxString *wxLog_GetTimestamp() {
    return new wxString(wxLog::GetTimestamp());
}
void wxLog_SetTimestamp(const wxString * format) {
    return wxLog::SetTimestamp(*format);
}
void wxLog_DisableTimestamp() {
    return wxLog::DisableTimestamp();
}
bool wxLog_GetVerbose() {
    return wxLog::GetVerbose();
}
void wxLog_SetVerbose(bool verbose) {
    return wxLog::SetVerbose(verbose);
}
wxLogFormatter * wxLog_SetFormatter(wxLog * self, wxLogFormatter * formatter) {
    return self->SetFormatter(formatter);
}
void wxLog_Flush(wxLog * self) {
    return self->Flush();
}

// CLASS: wxLogBuffer
void wxLogBuffer_delete(wxLogBuffer *self) {
    delete self;
}
wxLogBuffer *wxLogBuffer_new() {
    return new wxLogBuffer();
}
wxString *wxLogBuffer_GetBuffer(const wxLogBuffer * self) {
    return new wxString(self->GetBuffer());
}

// CLASS: wxLogChain
void wxLogChain_delete(wxLogChain *self) {
    delete self;
}
wxLogChain *wxLogChain_new(wxLog * logger) {
    return new wxLogChain(logger);
}
void wxLogChain_DetachOldLog(wxLogChain * self) {
    return self->DetachOldLog();
}
wxLog * wxLogChain_GetOldLog(const wxLogChain * self) {
    return self->GetOldLog();
}
bool wxLogChain_IsPassingMessages(const wxLogChain * self) {
    return self->IsPassingMessages();
}
void wxLogChain_PassMessages(wxLogChain * self, bool pass_messages) {
    return self->PassMessages(pass_messages);
}
void wxLogChain_SetLog(wxLogChain * self, wxLog * logger) {
    return self->SetLog(logger);
}

// CLASS: wxLogFormatter
void wxLogFormatter_delete(wxLogFormatter *self) {
    delete self;
}
wxLogFormatter *wxLogFormatter_new() {
    return new wxLogFormatter();
}

// CLASS: wxLogInterposer
void wxLogInterposer_delete(wxLogInterposer *self) {
    delete self;
}
wxLogInterposer *wxLogInterposer_new() {
    return new wxLogInterposer();
}

// CLASS: wxLogInterposerTemp
void wxLogInterposerTemp_delete(wxLogInterposerTemp *self) {
    delete self;
}
wxLogInterposerTemp *wxLogInterposerTemp_new() {
    return new wxLogInterposerTemp();
}

// CLASS: wxLogNull
void wxLogNull_delete(wxLogNull *self) {
    delete self;
}
wxLogNull *wxLogNull_new() {
    return new wxLogNull();
}

// CLASS: wxLogStderr
void wxLogStderr_delete(wxLogStderr *self) {
    delete self;
}
wxLogStderr *wxLogStderr_new(FILE * fp, const wxMBConv * conv) {
    return new wxLogStderr(fp, *conv);
}

// CLASS: wxLogStream
void wxLogStream_delete(wxLogStream *self) {
    delete self;
}
wxLogStream *wxLogStream_new(std::ostream * ostr, const wxMBConv * conv) {
    return new wxLogStream(ostr, *conv);
}

// CLASS: wxMemoryBuffer
void wxMemoryBuffer_delete(wxMemoryBuffer *self) {
    delete self;
}
wxMemoryBuffer *wxMemoryBuffer_new(const wxMemoryBuffer * src) {
    return new wxMemoryBuffer(*src);
}
wxMemoryBuffer *wxMemoryBuffer_new1(size_t size) {
    return new wxMemoryBuffer(size);
}
void wxMemoryBuffer_AppendData(wxMemoryBuffer * self, const void * data, size_t len) {
    return self->AppendData(data, len);
}
void wxMemoryBuffer_Clear(wxMemoryBuffer * self) {
    return self->Clear();
}
void * wxMemoryBuffer_GetAppendBuf(wxMemoryBuffer * self, size_t size_needed) {
    return self->GetAppendBuf(size_needed);
}
size_t wxMemoryBuffer_GetBufSize(const wxMemoryBuffer * self) {
    return self->GetBufSize();
}
void * wxMemoryBuffer_GetData(const wxMemoryBuffer * self) {
    return self->GetData();
}
size_t wxMemoryBuffer_GetDataLen(const wxMemoryBuffer * self) {
    return self->GetDataLen();
}
void * wxMemoryBuffer_GetWriteBuf(wxMemoryBuffer * self, size_t size_needed) {
    return self->GetWriteBuf(size_needed);
}
bool wxMemoryBuffer_IsEmpty(const wxMemoryBuffer * self) {
    return self->IsEmpty();
}
void wxMemoryBuffer_SetBufSize(wxMemoryBuffer * self, size_t size) {
    return self->SetBufSize(size);
}
void wxMemoryBuffer_SetDataLen(wxMemoryBuffer * self, size_t size) {
    return self->SetDataLen(size);
}
void wxMemoryBuffer_UngetAppendBuf(wxMemoryBuffer * self, size_t size_used) {
    return self->UngetAppendBuf(size_used);
}
void wxMemoryBuffer_UngetWriteBuf(wxMemoryBuffer * self, size_t size_used) {
    return self->UngetWriteBuf(size_used);
}

// CLASS: wxMemoryFSHandler
wxClassInfo *wxMemoryFSHandler_CLASSINFO() {
    return wxCLASSINFO(wxMemoryFSHandler);
}
wxMemoryFSHandler *wxMemoryFSHandler_new() {
    return new wxMemoryFSHandler();
}
void wxMemoryFSHandler_AddFile2(const wxString * filename, const wxString * textdata) {
    return wxMemoryFSHandler::AddFile(*filename, *textdata);
}
void wxMemoryFSHandler_AddFile3(const wxString * filename, const void * binarydata, size_t size) {
    return wxMemoryFSHandler::AddFile(*filename, binarydata, size);
}
void wxMemoryFSHandler_AddFileWithMimeType(const wxString * filename, const wxString * textdata, const wxString * mimetype) {
    return wxMemoryFSHandler::AddFileWithMimeType(*filename, *textdata, *mimetype);
}
void wxMemoryFSHandler_AddFileWithMimeType1(const wxString * filename, const void * binarydata, size_t size, const wxString * mimetype) {
    return wxMemoryFSHandler::AddFileWithMimeType(*filename, binarydata, size, *mimetype);
}
void wxMemoryFSHandler_RemoveFile(const wxString * filename) {
    return wxMemoryFSHandler::RemoveFile(*filename);
}

// CLASS: wxMessageOutput
void wxMessageOutput_delete(wxMessageOutput *self) {
    delete self;
}
wxMessageOutput * wxMessageOutput_Get() {
    return wxMessageOutput::Get();
}
wxMessageOutput * wxMessageOutput_Set(wxMessageOutput * msgout) {
    return wxMessageOutput::Set(msgout);
}
void wxMessageOutput_Output(wxMessageOutput * self, const wxString * str) {
    return self->Output(*str);
}

// CLASS: wxMessageOutputBest
void wxMessageOutputBest_delete(wxMessageOutputBest *self) {
    delete self;
}

// CLASS: wxMessageOutputDebug
void wxMessageOutputDebug_delete(wxMessageOutputDebug *self) {
    delete self;
}
wxMessageOutputDebug *wxMessageOutputDebug_new() {
    return new wxMessageOutputDebug();
}

// CLASS: wxMessageOutputStderr
void wxMessageOutputStderr_delete(wxMessageOutputStderr *self) {
    delete self;
}
wxMessageOutputStderr *wxMessageOutputStderr_new(FILE * fp) {
    return new wxMessageOutputStderr(fp);
}

// CLASS: wxMimeTypesManager
void wxMimeTypesManager_delete(wxMimeTypesManager *self) {
    delete self;
}
wxMimeTypesManager *wxMimeTypesManager_new() {
    return new wxMimeTypesManager();
}
void wxMimeTypesManager_AddFallbacks(wxMimeTypesManager * self, const wxFileTypeInfo * fallbacks) {
    return self->AddFallbacks(fallbacks);
}
wxFileType * wxMimeTypesManager_GetFileTypeFromExtension(wxMimeTypesManager * self, const wxString * extension) {
    return self->GetFileTypeFromExtension(*extension);
}
wxFileType * wxMimeTypesManager_GetFileTypeFromMimeType(wxMimeTypesManager * self, const wxString * mime_type) {
    return self->GetFileTypeFromMimeType(*mime_type);
}
wxFileType * wxMimeTypesManager_Associate(wxMimeTypesManager * self, const wxFileTypeInfo * ft_info) {
    return self->Associate(*ft_info);
}
bool wxMimeTypesManager_Unassociate(wxMimeTypesManager * self, wxFileType * ft) {
    return self->Unassociate(ft);
}
size_t wxMimeTypesManager_EnumAllFileTypes(wxMimeTypesManager * self, wxArrayString * mimetypes) {
    return self->EnumAllFileTypes(*mimetypes);
}
bool wxMimeTypesManager_IsOfType(const wxString * mime_type, const wxString * wildcard) {
    return wxMimeTypesManager::IsOfType(*mime_type, *wildcard);
}

// CLASS: wxModule
wxClassInfo *wxModule_CLASSINFO() {
    return wxCLASSINFO(wxModule);
}
wxModule *wxModule_new() {
    return new wxModule();
}
void wxModule_OnExit(wxModule * self) {
    return self->OnExit();
}
bool wxModule_OnInit(wxModule * self) {
    return self->OnInit();
}

// CLASS: wxMutex
void wxMutex_delete(wxMutex *self) {
    delete self;
}

// CLASS: wxMutexLocker
void wxMutexLocker_delete(wxMutexLocker *self) {
    delete self;
}
wxMutexLocker *wxMutexLocker_new(wxMutex * mutex) {
    return new wxMutexLocker(*mutex);
}
bool wxMutexLocker_IsOk(const wxMutexLocker * self) {
    return self->IsOk();
}

// CLASS: wxObject
wxClassInfo *wxObject_CLASSINFO() {
    return wxCLASSINFO(wxObject);
}
wxObject *wxObject_new() {
    return new wxObject();
}
wxObject *wxObject_new1(const wxObject * other) {
    return new wxObject(*other);
}
wxClassInfo * wxObject_GetClassInfo(const wxObject * self) {
    return self->GetClassInfo();
}
wxObjectRefData * wxObject_GetRefData(const wxObject * self) {
    return self->GetRefData();
}
bool wxObject_IsKindOf(const wxObject * self, const wxClassInfo * info) {
    return self->IsKindOf(info);
}
bool wxObject_IsSameAs(const wxObject * self, const wxObject * obj) {
    return self->IsSameAs(*obj);
}
void wxObject_Ref(wxObject * self, const wxObject * clone) {
    return self->Ref(*clone);
}
void wxObject_SetRefData(wxObject * self, wxObjectRefData * data) {
    return self->SetRefData(data);
}
void wxObject_UnRef(wxObject * self) {
    return self->UnRef();
}
void wxObject_UnShare(wxObject * self) {
    return self->UnShare();
}

// CLASS: wxObjectRefData
void wxObjectRefData_delete(wxObjectRefData *self) {
    delete self;
}

// CLASS: wxPlatformInfo
void wxPlatformInfo_delete(wxPlatformInfo *self) {
    delete self;
}
wxString *wxPlatformInfo_GetCpuArchitectureName(const wxPlatformInfo * self) {
    return new wxString(self->GetCpuArchitectureName());
}
wxString *wxPlatformInfo_GetNativeCpuArchitectureName(const wxPlatformInfo * self) {
    return new wxString(self->GetNativeCpuArchitectureName());
}
int wxPlatformInfo_GetOSMajorVersion(const wxPlatformInfo * self) {
    return self->GetOSMajorVersion();
}
int wxPlatformInfo_GetOSMinorVersion(const wxPlatformInfo * self) {
    return self->GetOSMinorVersion();
}
int wxPlatformInfo_GetOSMicroVersion(const wxPlatformInfo * self) {
    return self->GetOSMicroVersion();
}
wxString *wxPlatformInfo_GetOperatingSystemDescription(const wxPlatformInfo * self) {
    return new wxString(self->GetOperatingSystemDescription());
}
wxString *wxPlatformInfo_GetDesktopEnvironment(const wxPlatformInfo * self) {
    return new wxString(self->GetDesktopEnvironment());
}
int wxPlatformInfo_GetToolkitMajorVersion(const wxPlatformInfo * self) {
    return self->GetToolkitMajorVersion();
}
int wxPlatformInfo_GetToolkitMinorVersion(const wxPlatformInfo * self) {
    return self->GetToolkitMinorVersion();
}
int wxPlatformInfo_GetToolkitMicroVersion(const wxPlatformInfo * self) {
    return self->GetToolkitMicroVersion();
}
wxString *wxPlatformInfo_GetArchName(const wxPlatformInfo * self) {
    return new wxString(self->GetArchName());
}
wxString *wxPlatformInfo_GetBitnessName(const wxPlatformInfo * self) {
    return new wxString(self->GetBitnessName());
}
wxString *wxPlatformInfo_GetEndiannessName(const wxPlatformInfo * self) {
    return new wxString(self->GetEndiannessName());
}
wxString *wxPlatformInfo_GetOperatingSystemFamilyName(const wxPlatformInfo * self) {
    return new wxString(self->GetOperatingSystemFamilyName());
}
wxString *wxPlatformInfo_GetOperatingSystemIdName(const wxPlatformInfo * self) {
    return new wxString(self->GetOperatingSystemIdName());
}
wxString *wxPlatformInfo_GetPortIdName(const wxPlatformInfo * self) {
    return new wxString(self->GetPortIdName());
}
wxString *wxPlatformInfo_GetPortIdShortName(const wxPlatformInfo * self) {
    return new wxString(self->GetPortIdShortName());
}
void wxPlatformInfo_SetOSVersion(wxPlatformInfo * self, int major, int minor, int micro) {
    return self->SetOSVersion(major, minor, micro);
}
void wxPlatformInfo_SetToolkitVersion(wxPlatformInfo * self, int major, int minor, int micro) {
    return self->SetToolkitVersion(major, minor, micro);
}
void wxPlatformInfo_SetOperatingSystemDescription(wxPlatformInfo * self, const wxString * desc) {
    return self->SetOperatingSystemDescription(*desc);
}
void wxPlatformInfo_SetDesktopEnvironment(wxPlatformInfo * self, const wxString * de) {
    return self->SetDesktopEnvironment(*de);
}
void wxPlatformInfo_SetLinuxDistributionInfo(wxPlatformInfo * self, const wxLinuxDistributionInfo * di) {
    return self->SetLinuxDistributionInfo(*di);
}
wxString *wxPlatformInfo_GetOperatingSystemDirectory() {
    return new wxString(wxPlatformInfo::GetOperatingSystemDirectory());
}
wxPlatformInfo *wxPlatformInfo_new() {
    return new wxPlatformInfo();
}
bool wxPlatformInfo_CheckOSVersion(const wxPlatformInfo * self, int major, int minor, int micro) {
    return self->CheckOSVersion(major, minor, micro);
}
bool wxPlatformInfo_CheckToolkitVersion(const wxPlatformInfo * self, int major, int minor, int micro) {
    return self->CheckToolkitVersion(major, minor, micro);
}
bool wxPlatformInfo_IsOk(const wxPlatformInfo * self) {
    return self->IsOk();
}
bool wxPlatformInfo_IsUsingUniversalWidgets(const wxPlatformInfo * self) {
    return self->IsUsingUniversalWidgets();
}
wxPlatformInfo *wxPlatformInfo_Get() {
    return new wxPlatformInfo(wxPlatformInfo::Get());
}

// CLASS: wxPosition
void wxPosition_delete(wxPosition *self) {
    delete self;
}
wxPosition *wxPosition_new() {
    return new wxPosition();
}
wxPosition *wxPosition_new1(int row, int col) {
    return new wxPosition(row, col);
}
int wxPosition_GetCol(const wxPosition * self) {
    return self->GetCol();
}
int wxPosition_GetColumn(const wxPosition * self) {
    return self->GetColumn();
}
int wxPosition_GetRow(const wxPosition * self) {
    return self->GetRow();
}
void wxPosition_SetCol(wxPosition * self, int column) {
    return self->SetCol(column);
}
void wxPosition_SetColumn(wxPosition * self, int column) {
    return self->SetColumn(column);
}
void wxPosition_SetRow(wxPosition * self, int row) {
    return self->SetRow(row);
}

// CLASS: wxPostScriptDC
wxClassInfo *wxPostScriptDC_CLASSINFO() {
    return wxCLASSINFO(wxPostScriptDC);
}
wxPostScriptDC *wxPostScriptDC_new() {
    return new wxPostScriptDC();
}
wxPostScriptDC *wxPostScriptDC_new1(const wxPrintData * print_data) {
    return new wxPostScriptDC(*print_data);
}

// CLASS: wxPowerEvent
wxClassInfo *wxPowerEvent_CLASSINFO() {
    return wxCLASSINFO(wxPowerEvent);
}
wxPowerEvent *wxPowerEvent_new() {
    return new wxPowerEvent();
}
void wxPowerEvent_Veto(wxPowerEvent * self) {
    return self->Veto();
}
bool wxPowerEvent_IsVetoed(const wxPowerEvent * self) {
    return self->IsVetoed();
}

// CLASS: wxPowerResource
void wxPowerResource_delete(wxPowerResource *self) {
    delete self;
}

// CLASS: wxPowerResourceBlocker
void wxPowerResourceBlocker_delete(wxPowerResourceBlocker *self) {
    delete self;
}
bool wxPowerResourceBlocker_IsInEffect(const wxPowerResourceBlocker * self) {
    return self->IsInEffect();
}

// CLASS: wxProcess
wxClassInfo *wxProcess_CLASSINFO() {
    return wxCLASSINFO(wxProcess);
}
wxProcess *wxProcess_new(wxEvtHandler * parent, int id) {
    return new wxProcess(parent, id);
}
wxProcess *wxProcess_new1(int flags) {
    return new wxProcess(flags);
}
bool wxProcess_Activate(const wxProcess * self) {
    return self->Activate();
}
void wxProcess_CloseOutput(wxProcess * self) {
    return self->CloseOutput();
}
void wxProcess_Detach(wxProcess * self) {
    return self->Detach();
}
wxInputStream * wxProcess_GetErrorStream(const wxProcess * self) {
    return self->GetErrorStream();
}
wxInputStream * wxProcess_GetInputStream(const wxProcess * self) {
    return self->GetInputStream();
}
wxOutputStream * wxProcess_GetOutputStream(const wxProcess * self) {
    return self->GetOutputStream();
}
long wxProcess_GetPid(const wxProcess * self) {
    return self->GetPid();
}
bool wxProcess_IsErrorAvailable(const wxProcess * self) {
    return self->IsErrorAvailable();
}
bool wxProcess_IsInputAvailable(const wxProcess * self) {
    return self->IsInputAvailable();
}
bool wxProcess_IsInputOpened(const wxProcess * self) {
    return self->IsInputOpened();
}
void wxProcess_OnTerminate(wxProcess * self, int pid, int status) {
    return self->OnTerminate(pid, status);
}
void wxProcess_Redirect(wxProcess * self) {
    return self->Redirect();
}
bool wxProcess_Exists(int pid) {
    return wxProcess::Exists(pid);
}
wxProcess * wxProcess_Open(const wxString * cmd, int flags) {
    return wxProcess::Open(*cmd, flags);
}
// Mix-in(s) to wxProcess
wxTrackable *wxProcess_AsTrackable(wxProcess* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxProcessEvent
wxClassInfo *wxProcessEvent_CLASSINFO() {
    return wxCLASSINFO(wxProcessEvent);
}
wxProcessEvent *wxProcessEvent_new(int id, int pid, int exitcode) {
    return new wxProcessEvent(id, pid, exitcode);
}
int wxProcessEvent_GetExitCode(const wxProcessEvent * self) {
    return self->GetExitCode();
}
int wxProcessEvent_GetPid(const wxProcessEvent * self) {
    return self->GetPid();
}

// CLASS: wxRecursionGuard
void wxRecursionGuard_delete(wxRecursionGuard *self) {
    delete self;
}
wxRecursionGuard *wxRecursionGuard_new(wxRecursionGuardFlag * flag) {
    return new wxRecursionGuard(*flag);
}
bool wxRecursionGuard_IsInside(const wxRecursionGuard * self) {
    return self->IsInside();
}

// CLASS: wxRecursionGuardFlag
void wxRecursionGuardFlag_delete(wxRecursionGuardFlag *self) {
    delete self;
}

// CLASS: wxRefCounter
void wxRefCounter_delete(wxRefCounter *self) {
    delete self;
}
wxRefCounter *wxRefCounter_new() {
    return new wxRefCounter();
}
void wxRefCounter_DecRef(wxRefCounter * self) {
    return self->DecRef();
}
int wxRefCounter_GetRefCount(const wxRefCounter * self) {
    return self->GetRefCount();
}
void wxRefCounter_IncRef(wxRefCounter * self) {
    return self->IncRef();
}

// CLASS: wxRegEx
void wxRegEx_delete(wxRegEx *self) {
    delete self;
}
wxRegEx *wxRegEx_new() {
    return new wxRegEx();
}
wxRegEx *wxRegEx_new1(const wxString * expr, int flags) {
    return new wxRegEx(*expr, flags);
}
bool wxRegEx_Compile(wxRegEx * self, const wxString * pattern, int flags) {
    return self->Compile(*pattern, flags);
}
bool wxRegEx_GetMatch(const wxRegEx * self, size_t * start, size_t * len, size_t index) {
    return self->GetMatch(start, len, index);
}
wxString *wxRegEx_GetMatch1(const wxRegEx * self, const wxString * text, size_t index) {
    return new wxString(self->GetMatch(*text, index));
}
size_t wxRegEx_GetMatchCount(const wxRegEx * self) {
    return self->GetMatchCount();
}
bool wxRegEx_IsValid(const wxRegEx * self) {
    return self->IsValid();
}
bool wxRegEx_Matches(const wxRegEx * self, const wxChar * text, int flags) {
    return self->Matches(text, flags);
}
bool wxRegEx_Matches1(const wxRegEx * self, const wxChar * text, int flags, size_t len) {
    return self->Matches(text, flags, len);
}
bool wxRegEx_Matches2(const wxRegEx * self, const wxString * text, int flags) {
    return self->Matches(*text, flags);
}
int wxRegEx_Replace(const wxRegEx * self, wxString * text, const wxString * replacement, size_t max_matches) {
    return self->Replace(text, *replacement, max_matches);
}
int wxRegEx_ReplaceAll(const wxRegEx * self, wxString * text, const wxString * replacement) {
    return self->ReplaceAll(text, *replacement);
}
int wxRegEx_ReplaceFirst(const wxRegEx * self, wxString * text, const wxString * replacement) {
    return self->ReplaceFirst(text, *replacement);
}
wxString *wxRegEx_QuoteMeta(const wxString * str) {
    return new wxString(wxRegEx::QuoteMeta(*str));
}
wxString *wxRegEx_ConvertFromBasic(const wxString * bre) {
    return new wxString(wxRegEx::ConvertFromBasic(*bre));
}
wxVersionInfo *wxRegEx_GetLibraryVersionInfo() {
    return new wxVersionInfo(wxRegEx::GetLibraryVersionInfo());
}

// CLASS: wxRegKey
void wxRegKey_delete(wxRegKey *self) {
    delete self;
}
wxRegKey *wxRegKey_new3(const wxRegKey * key_parent, const wxString * str_key) {
    return new wxRegKey(*key_parent, *str_key);
}
void wxRegKey_Close(wxRegKey * self) {
    return self->Close();
}
bool wxRegKey_Copy(wxRegKey * self, const wxString * sz_new_name) {
    return self->Copy(*sz_new_name);
}
bool wxRegKey_Copy1(wxRegKey * self, wxRegKey * key_dst) {
    return self->Copy(*key_dst);
}
bool wxRegKey_CopyValue(wxRegKey * self, const wxString * sz_value, wxRegKey * key_dst, const wxString * sz_new_name) {
    return self->CopyValue(*sz_value, *key_dst, *sz_new_name);
}
bool wxRegKey_Create(wxRegKey * self, bool b_ok_if_exists) {
    return self->Create(b_ok_if_exists);
}
void wxRegKey_DeleteKey(wxRegKey * self, const wxString * sz_key) {
    return self->DeleteKey(*sz_key);
}
void wxRegKey_DeleteSelf(wxRegKey * self) {
    return self->DeleteSelf();
}
void wxRegKey_DeleteValue(wxRegKey * self, const wxString * sz_key) {
    return self->DeleteValue(*sz_key);
}
bool wxRegKey_Exists(const wxRegKey * self) {
    return self->Exists();
}
bool wxRegKey_Export(const wxRegKey * self, const wxString * filename) {
    return self->Export(*filename);
}
bool wxRegKey_Export1(const wxRegKey * self, wxOutputStream * ostr) {
    return self->Export(*ostr);
}
bool wxRegKey_GetFirstKey(wxRegKey * self, wxString * str_key_name, long * l_index) {
    return self->GetFirstKey(*str_key_name, *l_index);
}
bool wxRegKey_GetFirstValue(wxRegKey * self, wxString * str_value_name, long * l_index) {
    return self->GetFirstValue(*str_value_name, *l_index);
}
bool wxRegKey_GetKeyInfo(const wxRegKey * self, size_t * pn_sub_keys, size_t * pn_max_key_len, size_t * pn_values, size_t * pn_max_value_len) {
    return self->GetKeyInfo(pn_sub_keys, pn_max_key_len, pn_values, pn_max_value_len);
}
wxString *wxRegKey_GetName(const wxRegKey * self, bool b_short_prefix) {
    return new wxString(self->GetName(b_short_prefix));
}
bool wxRegKey_GetNextKey(const wxRegKey * self, wxString * str_key_name, long * l_index) {
    return self->GetNextKey(*str_key_name, *l_index);
}
bool wxRegKey_GetNextValue(const wxRegKey * self, wxString * str_value_name, long * l_index) {
    return self->GetNextValue(*str_value_name, *l_index);
}
bool wxRegKey_HasSubKey(const wxRegKey * self, const wxString * sz_key) {
    return self->HasSubKey(*sz_key);
}
bool wxRegKey_HasSubkeys(const wxRegKey * self) {
    return self->HasSubkeys();
}
bool wxRegKey_HasValue(const wxRegKey * self, const wxString * sz_value) {
    return self->HasValue(*sz_value);
}
bool wxRegKey_HasValues(const wxRegKey * self) {
    return self->HasValues();
}
bool wxRegKey_IsEmpty(const wxRegKey * self) {
    return self->IsEmpty();
}
bool wxRegKey_IsNumericValue(const wxRegKey * self, const wxString * sz_value) {
    return self->IsNumericValue(*sz_value);
}
bool wxRegKey_IsOpened(const wxRegKey * self) {
    return self->IsOpened();
}
wxString *wxRegKey_QueryDefaultValue(const wxRegKey * self) {
    return new wxString(self->QueryDefaultValue());
}
bool wxRegKey_QueryRawValue(const wxRegKey * self, const wxString * sz_value, wxString * str_value) {
    return self->QueryRawValue(*sz_value, *str_value);
}
bool wxRegKey_QueryValue(const wxRegKey * self, const wxString * sz_value, wxString * str_value, bool raw) {
    return self->QueryValue(*sz_value, *str_value, raw);
}
bool wxRegKey_QueryValue1(const wxRegKey * self, const wxString * sz_value, long * pl_value) {
    return self->QueryValue(*sz_value, pl_value);
}
bool wxRegKey_QueryValue64(const wxRegKey * self, const wxString * sz_value, wxLongLong_t * pl_value) {
    return self->QueryValue64(*sz_value, pl_value);
}
bool wxRegKey_QueryValue2(const wxRegKey * self, const wxString * sz_value, wxMemoryBuffer * buf) {
    return self->QueryValue(*sz_value, *buf);
}
bool wxRegKey_Rename(wxRegKey * self, const wxString * sz_new_name) {
    return self->Rename(*sz_new_name);
}
bool wxRegKey_RenameValue(wxRegKey * self, const wxString * sz_value_old, const wxString * sz_value_new) {
    return self->RenameValue(*sz_value_old, *sz_value_new);
}
void wxRegKey_ReserveMemoryForName(wxRegKey * self, size_t bytes) {
    return self->ReserveMemoryForName(bytes);
}
void wxRegKey_SetName(wxRegKey * self, const wxString * str_key) {
    return self->SetName(*str_key);
}
void wxRegKey_SetName2(wxRegKey * self, const wxRegKey * key_parent, const wxString * str_key) {
    return self->SetName(*key_parent, *str_key);
}
bool wxRegKey_SetValue(wxRegKey * self, const wxString * sz_value, long l_value) {
    return self->SetValue(*sz_value, l_value);
}
bool wxRegKey_SetValue1(wxRegKey * self, const wxString * sz_value, const wxString * str_value) {
    return self->SetValue(*sz_value, *str_value);
}
bool wxRegKey_SetValue2(wxRegKey * self, const wxString * sz_value, const wxMemoryBuffer * buf) {
    return self->SetValue(*sz_value, *buf);
}

// CLASS: wxSecretStore
void wxSecretStore_delete(wxSecretStore *self) {
    delete self;
}
wxSecretStore *wxSecretStore_GetDefault() {
    return new wxSecretStore(wxSecretStore::GetDefault());
}
bool wxSecretStore_IsOk(const wxSecretStore * self, wxString * errmsg) {
    return self->IsOk(errmsg);
}
bool wxSecretStore_Save(wxSecretStore * self, const wxString * service, const wxString * username, const wxSecretValue * password) {
    return self->Save(*service, *username, *password);
}
bool wxSecretStore_Load(const wxSecretStore * self, const wxString * service, wxString * username, wxSecretValue * password) {
    return self->Load(*service, *username, *password);
}
bool wxSecretStore_Delete(wxSecretStore * self, const wxString * service) {
    return self->Delete(*service);
}

// CLASS: wxSecretValue
void wxSecretValue_delete(wxSecretValue *self) {
    delete self;
}
wxSecretValue *wxSecretValue_new() {
    return new wxSecretValue();
}
wxSecretValue *wxSecretValue_new1(size_t size, const void * data) {
    return new wxSecretValue(size, data);
}
wxSecretValue *wxSecretValue_new2(const wxString * secret) {
    return new wxSecretValue(*secret);
}
wxSecretValue *wxSecretValue_new3(const wxSecretValue * other) {
    return new wxSecretValue(*other);
}
bool wxSecretValue_IsOk(const wxSecretValue * self) {
    return self->IsOk();
}
size_t wxSecretValue_GetSize(const wxSecretValue * self) {
    return self->GetSize();
}
const void * wxSecretValue_GetData(const wxSecretValue * self) {
    return self->GetData();
}
wxString *wxSecretValue_GetAsString(const wxSecretValue * self, const wxMBConv * conv) {
    return new wxString(self->GetAsString(*conv));
}
void wxSecretValue_Wipe(size_t size, void * data) {
    return wxSecretValue::Wipe(size, data);
}
void wxSecretValue_WipeString(wxString * str) {
    return wxSecretValue::WipeString(*str);
}

// CLASS: wxSemaphore
void wxSemaphore_delete(wxSemaphore *self) {
    delete self;
}
wxSemaphore *wxSemaphore_new(int initialcount, int maxcount) {
    return new wxSemaphore(initialcount, maxcount);
}

// CLASS: wxServer
void wxServer_delete(wxServer *self) {
    delete self;
}
wxServer *wxServer_new() {
    return new wxServer();
}
bool wxServer_Create(wxServer * self, const wxString * service) {
    return self->Create(*service);
}
wxConnectionBase * wxServer_OnAcceptConnection(wxServer * self, const wxString * topic) {
    return self->OnAcceptConnection(*topic);
}

// CLASS: wxSharedClientDataContainer
void wxSharedClientDataContainer_delete(wxSharedClientDataContainer *self) {
    delete self;
}
void * wxSharedClientDataContainer_GetClientData(const wxSharedClientDataContainer * self) {
    return self->GetClientData();
}
wxClientData * wxSharedClientDataContainer_GetClientObject(const wxSharedClientDataContainer * self) {
    return self->GetClientObject();
}
void wxSharedClientDataContainer_SetClientData(wxSharedClientDataContainer * self, void * data) {
    return self->SetClientData(data);
}
void wxSharedClientDataContainer_SetClientObject(wxSharedClientDataContainer * self, wxClientData * data) {
    return self->SetClientObject(data);
}

// CLASS: wxSingleInstanceChecker
void wxSingleInstanceChecker_delete(wxSingleInstanceChecker *self) {
    delete self;
}
wxSingleInstanceChecker *wxSingleInstanceChecker_new() {
    return new wxSingleInstanceChecker();
}
wxSingleInstanceChecker *wxSingleInstanceChecker_new1(const wxString * name, const wxString * path) {
    return new wxSingleInstanceChecker(*name, *path);
}
bool wxSingleInstanceChecker_Create(wxSingleInstanceChecker * self, const wxString * name, const wxString * path) {
    return self->Create(*name, *path);
}
bool wxSingleInstanceChecker_CreateDefault(wxSingleInstanceChecker * self) {
    return self->CreateDefault();
}
bool wxSingleInstanceChecker_IsAnotherRunning(const wxSingleInstanceChecker * self) {
    return self->IsAnotherRunning();
}

// CLASS: wxStackFrame
void wxStackFrame_delete(wxStackFrame *self) {
    delete self;
}
void * wxStackFrame_GetAddress(const wxStackFrame * self) {
    return self->GetAddress();
}
wxString *wxStackFrame_GetFileName(const wxStackFrame * self) {
    return new wxString(self->GetFileName());
}
size_t wxStackFrame_GetLevel(const wxStackFrame * self) {
    return self->GetLevel();
}
size_t wxStackFrame_GetLine(const wxStackFrame * self) {
    return self->GetLine();
}
wxString *wxStackFrame_GetModule(const wxStackFrame * self) {
    return new wxString(self->GetModule());
}
wxString *wxStackFrame_GetName(const wxStackFrame * self) {
    return new wxString(self->GetName());
}
size_t wxStackFrame_GetOffset(const wxStackFrame * self) {
    return self->GetOffset();
}
bool wxStackFrame_GetParam(const wxStackFrame * self, size_t n, wxString * type_, wxString * name, wxString * value) {
    return self->GetParam(n, type_, name, value);
}
size_t wxStackFrame_GetParamCount(const wxStackFrame * self) {
    return self->GetParamCount();
}
bool wxStackFrame_HasSourceLocation(const wxStackFrame * self) {
    return self->HasSourceLocation();
}

// CLASS: wxStackWalker
void wxStackWalker_delete(wxStackWalker *self) {
    delete self;
}
wxStackWalker *wxStackWalker_new(const char * argv0) {
    return new wxStackWalker(argv0);
}
void wxStackWalker_Walk(wxStackWalker * self, size_t skip, size_t max_depth) {
    return self->Walk(skip, max_depth);
}
void wxStackWalker_WalkFromException(wxStackWalker * self, size_t max_depth) {
    return self->WalkFromException(max_depth);
}

// CLASS: wxStandardPaths
void wxStandardPaths_delete(wxStandardPaths *self) {
    delete self;
}
#ifdef __WXMSW__
void wxStandardPaths_DontIgnoreAppSubDir(wxStandardPaths * self) {
    return self->DontIgnoreAppSubDir();
}
#endif
wxString *wxStandardPaths_GetAppDocumentsDir(const wxStandardPaths * self) {
    return new wxString(self->GetAppDocumentsDir());
}
wxString *wxStandardPaths_GetConfigDir(const wxStandardPaths * self) {
    return new wxString(self->GetConfigDir());
}
wxString *wxStandardPaths_GetDataDir(const wxStandardPaths * self) {
    return new wxString(self->GetDataDir());
}
wxString *wxStandardPaths_GetDocumentsDir(const wxStandardPaths * self) {
    return new wxString(self->GetDocumentsDir());
}
wxString *wxStandardPaths_GetExecutablePath(const wxStandardPaths * self) {
    return new wxString(self->GetExecutablePath());
}
#ifdef __WXGTK__
wxString *wxStandardPaths_GetInstallPrefix(const wxStandardPaths * self) {
    return new wxString(self->GetInstallPrefix());
}
#endif
wxString *wxStandardPaths_GetLocalDataDir(const wxStandardPaths * self) {
    return new wxString(self->GetLocalDataDir());
}
wxString *wxStandardPaths_GetPluginsDir(const wxStandardPaths * self) {
    return new wxString(self->GetPluginsDir());
}
wxString *wxStandardPaths_GetResourcesDir(const wxStandardPaths * self) {
    return new wxString(self->GetResourcesDir());
}
wxString *wxStandardPaths_GetTempDir(const wxStandardPaths * self) {
    return new wxString(self->GetTempDir());
}
wxString *wxStandardPaths_GetUserConfigDir(const wxStandardPaths * self) {
    return new wxString(self->GetUserConfigDir());
}
wxString *wxStandardPaths_GetUserDataDir(const wxStandardPaths * self) {
    return new wxString(self->GetUserDataDir());
}
wxString *wxStandardPaths_GetUserLocalDataDir(const wxStandardPaths * self) {
    return new wxString(self->GetUserLocalDataDir());
}
#ifdef __WXMSW__
void wxStandardPaths_IgnoreAppSubDir(wxStandardPaths * self, const wxString * subdir_pattern) {
    return self->IgnoreAppSubDir(*subdir_pattern);
}
void wxStandardPaths_IgnoreAppBuildSubDirs(wxStandardPaths * self) {
    return self->IgnoreAppBuildSubDirs();
}
#endif
#ifdef __WXGTK__
void wxStandardPaths_SetInstallPrefix(wxStandardPaths * self, const wxString * prefix) {
    return self->SetInstallPrefix(*prefix);
}
#endif
void wxStandardPaths_UseAppInfo(wxStandardPaths * self, int info) {
    return self->UseAppInfo(info);
}
wxStandardPaths * wxStandardPaths_Get() {
    return &(wxStandardPaths::Get());
}
#ifdef __WXMSW__
wxString *wxStandardPaths_MSWGetShellDir(int csidl) {
    return new wxString(wxStandardPaths::MSWGetShellDir(csidl));
}
#endif

// CLASS: wxStopWatch
void wxStopWatch_delete(wxStopWatch *self) {
    delete self;
}
wxStopWatch *wxStopWatch_new() {
    return new wxStopWatch();
}
void wxStopWatch_Pause(wxStopWatch * self) {
    return self->Pause();
}
void wxStopWatch_Resume(wxStopWatch * self) {
    return self->Resume();
}
void wxStopWatch_Start(wxStopWatch * self, long milliseconds) {
    return self->Start(milliseconds);
}
long wxStopWatch_Time(const wxStopWatch * self) {
    return self->Time();
}

// CLASS: wxStreamBuffer
void wxStreamBuffer_delete(wxStreamBuffer *self) {
    delete self;
}
wxStreamBuffer *wxStreamBuffer_new1(size_t bufsize, wxInputStream * stream) {
    return new wxStreamBuffer(bufsize, *stream);
}
wxStreamBuffer *wxStreamBuffer_new2(size_t bufsize, wxOutputStream * stream) {
    return new wxStreamBuffer(bufsize, *stream);
}
wxStreamBuffer *wxStreamBuffer_new4(const wxStreamBuffer * buffer) {
    return new wxStreamBuffer(*buffer);
}
bool wxStreamBuffer_FillBuffer(wxStreamBuffer * self) {
    return self->FillBuffer();
}
void wxStreamBuffer_Fixed(wxStreamBuffer * self, bool fixed) {
    return self->Fixed(fixed);
}
bool wxStreamBuffer_FlushBuffer(wxStreamBuffer * self) {
    return self->FlushBuffer();
}
void wxStreamBuffer_Flushable(wxStreamBuffer * self, bool flushable) {
    return self->Flushable(flushable);
}
void * wxStreamBuffer_GetBufferEnd(const wxStreamBuffer * self) {
    return self->GetBufferEnd();
}
void * wxStreamBuffer_GetBufferPos(const wxStreamBuffer * self) {
    return self->GetBufferPos();
}
size_t wxStreamBuffer_GetBufferSize(const wxStreamBuffer * self) {
    return self->GetBufferSize();
}
void * wxStreamBuffer_GetBufferStart(const wxStreamBuffer * self) {
    return self->GetBufferStart();
}
size_t wxStreamBuffer_GetDataLeft(wxStreamBuffer * self) {
    return self->GetDataLeft();
}
size_t wxStreamBuffer_GetIntPosition(const wxStreamBuffer * self) {
    return self->GetIntPosition();
}
size_t wxStreamBuffer_GetLastAccess(const wxStreamBuffer * self) {
    return self->GetLastAccess();
}
size_t wxStreamBuffer_Read(wxStreamBuffer * self, void * buffer, size_t size) {
    return self->Read(buffer, size);
}
size_t wxStreamBuffer_Read1(wxStreamBuffer * self, wxStreamBuffer * buffer) {
    return self->Read(buffer);
}
void wxStreamBuffer_ResetBuffer(wxStreamBuffer * self) {
    return self->ResetBuffer();
}
void wxStreamBuffer_SetBufferIO(wxStreamBuffer * self, void * start, void * end, bool take_ownership) {
    return self->SetBufferIO(start, end, take_ownership);
}
void wxStreamBuffer_SetBufferIO1(wxStreamBuffer * self, size_t bufsize) {
    return self->SetBufferIO(bufsize);
}
void wxStreamBuffer_SetIntPosition(wxStreamBuffer * self, size_t pos) {
    return self->SetIntPosition(pos);
}
wxStreamBase * wxStreamBuffer_Stream(wxStreamBuffer * self) {
    return self->Stream();
}
void wxStreamBuffer_Truncate(wxStreamBuffer * self) {
    return self->Truncate();
}
size_t wxStreamBuffer_Write(wxStreamBuffer * self, const void * buffer, size_t size) {
    return self->Write(buffer, size);
}
size_t wxStreamBuffer_Write1(wxStreamBuffer * self, wxStreamBuffer * buffer) {
    return self->Write(buffer);
}

// CLASS: wxStringClientData
void wxStringClientData_delete(wxStringClientData *self) {
    delete self;
}
wxStringClientData *wxStringClientData_new() {
    return new wxStringClientData();
}
wxStringClientData *wxStringClientData_new1(const wxString * data) {
    return new wxStringClientData(*data);
}
wxString *wxStringClientData_GetData(const wxStringClientData * self) {
    return new wxString(self->GetData());
}
void wxStringClientData_SetData(wxStringClientData * self, const wxString * data) {
    return self->SetData(*data);
}

// CLASS: wxStringTokenizer
wxClassInfo *wxStringTokenizer_CLASSINFO() {
    return wxCLASSINFO(wxStringTokenizer);
}
wxStringTokenizer *wxStringTokenizer_new() {
    return new wxStringTokenizer();
}
size_t wxStringTokenizer_CountTokens(const wxStringTokenizer * self) {
    return self->CountTokens();
}
wxString *wxStringTokenizer_GetNextToken(wxStringTokenizer * self) {
    return new wxString(self->GetNextToken());
}
size_t wxStringTokenizer_GetPosition(const wxStringTokenizer * self) {
    return self->GetPosition();
}
wxString *wxStringTokenizer_GetString(const wxStringTokenizer * self) {
    return new wxString(self->GetString());
}
bool wxStringTokenizer_HasMoreTokens(const wxStringTokenizer * self) {
    return self->HasMoreTokens();
}

// CLASS: wxSystemOptions
wxClassInfo *wxSystemOptions_CLASSINFO() {
    return wxCLASSINFO(wxSystemOptions);
}
wxSystemOptions *wxSystemOptions_new() {
    return new wxSystemOptions();
}
wxString *wxSystemOptions_GetOption(const wxString * name) {
    return new wxString(wxSystemOptions::GetOption(*name));
}
int wxSystemOptions_GetOptionInt(const wxString * name) {
    return wxSystemOptions::GetOptionInt(*name);
}
bool wxSystemOptions_HasOption(const wxString * name) {
    return wxSystemOptions::HasOption(*name);
}
bool wxSystemOptions_IsFalse(const wxString * name) {
    return wxSystemOptions::IsFalse(*name);
}
void wxSystemOptions_SetOption(const wxString * name, const wxString * value) {
    return wxSystemOptions::SetOption(*name, *value);
}
void wxSystemOptions_SetOption1(const wxString * name, int value) {
    return wxSystemOptions::SetOption(*name, value);
}

// CLASS: wxTarClassFactory
wxClassInfo *wxTarClassFactory_CLASSINFO() {
    return wxCLASSINFO(wxTarClassFactory);
}

// CLASS: wxTempFFile
void wxTempFFile_delete(wxTempFFile *self) {
    delete self;
}
wxTempFFile *wxTempFFile_new() {
    return new wxTempFFile();
}
wxTempFFile *wxTempFFile_new1(const wxString * str_name) {
    return new wxTempFFile(*str_name);
}
bool wxTempFFile_Commit(wxTempFFile * self) {
    return self->Commit();
}
void wxTempFFile_Discard(wxTempFFile * self) {
    return self->Discard();
}
bool wxTempFFile_Flush(wxTempFFile * self) {
    return self->Flush();
}
bool wxTempFFile_IsOpened(const wxTempFFile * self) {
    return self->IsOpened();
}
bool wxTempFFile_Open(wxTempFFile * self, const wxString * str_name) {
    return self->Open(*str_name);
}
bool wxTempFFile_Write(wxTempFFile * self, const wxString * str, const wxMBConv * conv) {
    return self->Write(*str, *conv);
}

// CLASS: wxTempFile
void wxTempFile_delete(wxTempFile *self) {
    delete self;
}
wxTempFile *wxTempFile_new() {
    return new wxTempFile();
}
wxTempFile *wxTempFile_new1(const wxString * str_name) {
    return new wxTempFile(*str_name);
}
bool wxTempFile_Commit(wxTempFile * self) {
    return self->Commit();
}
void wxTempFile_Discard(wxTempFile * self) {
    return self->Discard();
}
bool wxTempFile_Flush(wxTempFile * self) {
    return self->Flush();
}
bool wxTempFile_IsOpened(const wxTempFile * self) {
    return self->IsOpened();
}
bool wxTempFile_Open(wxTempFile * self, const wxString * str_name) {
    return self->Open(*str_name);
}
bool wxTempFile_Write(wxTempFile * self, const wxString * str, const wxMBConv * conv) {
    return self->Write(*str, *conv);
}

// CLASS: wxThreadHelper
void wxThreadHelper_delete(wxThreadHelper *self) {
    delete self;
}
void wxThreadHelper_OnDelete(wxThreadHelper * self) {
    return self->OnDelete();
}
void wxThreadHelper_OnKill(wxThreadHelper * self) {
    return self->OnKill();
}
void wxThreadHelper_OnExit(wxThreadHelper * self) {
    return self->OnExit();
}
wxThread * wxThreadHelper_GetThread(const wxThreadHelper * self) {
    return self->GetThread();
}

// CLASS: wxTimer
wxClassInfo *wxTimer_CLASSINFO() {
    return wxCLASSINFO(wxTimer);
}
wxTimer *wxTimer_new() {
    return new wxTimer();
}
wxTimer *wxTimer_new1(wxEvtHandler * owner, int id) {
    return new wxTimer(owner, id);
}
int wxTimer_GetId(const wxTimer * self) {
    return self->GetId();
}
int wxTimer_GetInterval(const wxTimer * self) {
    return self->GetInterval();
}
wxEvtHandler * wxTimer_GetOwner(const wxTimer * self) {
    return self->GetOwner();
}
bool wxTimer_IsOneShot(const wxTimer * self) {
    return self->IsOneShot();
}
bool wxTimer_IsRunning(const wxTimer * self) {
    return self->IsRunning();
}
void wxTimer_Notify(wxTimer * self) {
    return self->Notify();
}
void wxTimer_SetOwner(wxTimer * self, wxEvtHandler * owner, int id) {
    return self->SetOwner(owner, id);
}
bool wxTimer_Start(wxTimer * self, int milliseconds, bool one_shot) {
    return self->Start(milliseconds, one_shot);
}
bool wxTimer_StartOnce(wxTimer * self, int milliseconds) {
    return self->StartOnce(milliseconds);
}
void wxTimer_Stop(wxTimer * self) {
    return self->Stop();
}
// Mix-in(s) to wxTimer
wxTrackable *wxTimer_AsTrackable(wxTimer* obj) {
    return static_cast<wxTrackable*>(obj);
}

// CLASS: wxTimerEvent
wxClassInfo *wxTimerEvent_CLASSINFO() {
    return wxCLASSINFO(wxTimerEvent);
}
wxTimerEvent *wxTimerEvent_new(wxTimer * timer) {
    return new wxTimerEvent(*timer);
}
int wxTimerEvent_GetInterval(const wxTimerEvent * self) {
    return self->GetInterval();
}
wxTimer * wxTimerEvent_GetTimer(const wxTimerEvent * self) {
    return &(self->GetTimer());
}

// CLASS: wxTrackable
void wxTrackable_delete(wxTrackable *self) {
    delete self;
}

// CLASS: wxUILocale
void wxUILocale_delete(wxUILocale *self) {
    delete self;
}
bool wxUILocale_UseDefault() {
    return wxUILocale::UseDefault();
}
wxUILocale *wxUILocale_GetCurrent() {
    return new wxUILocale(wxUILocale::GetCurrent());
}
wxUILocale *wxUILocale_FromTag(const wxString * tag) {
    return new wxUILocale(wxUILocale::FromTag(*tag));
}
void wxUILocale_AddLanguage(const wxLanguageInfo * info) {
    return wxUILocale::AddLanguage(*info);
}
const wxLanguageInfo * wxUILocale_FindLanguageInfo(const wxString * locale) {
    return wxUILocale::FindLanguageInfo(*locale);
}
const wxLanguageInfo * wxUILocale_FindLanguageInfo1(const wxLocaleIdent * locale_id) {
    return wxUILocale::FindLanguageInfo(*locale_id);
}
const wxLanguageInfo * wxUILocale_GetLanguageInfo(int lang) {
    return wxUILocale::GetLanguageInfo(lang);
}
wxString *wxUILocale_GetLanguageName(int lang) {
    return new wxString(wxUILocale::GetLanguageName(lang));
}
wxString *wxUILocale_GetLanguageCanonicalName(int lang) {
    return new wxString(wxUILocale::GetLanguageCanonicalName(lang));
}
int wxUILocale_GetSystemLanguage() {
    return wxUILocale::GetSystemLanguage();
}
int wxUILocale_GetSystemLocale() {
    return wxUILocale::GetSystemLocale();
}
wxUILocale *wxUILocale_new(const wxLocaleIdent * locale_id) {
    return new wxUILocale(*locale_id);
}
int wxUILocale_CompareStrings(const wxUILocale * self, const wxString * lhs, const wxString * rhs, int flags) {
    return self->CompareStrings(*lhs, *rhs, flags);
}
wxString *wxUILocale_GetName(const wxUILocale * self) {
    return new wxString(self->GetName());
}
wxLocaleIdent *wxUILocale_GetLocaleId(const wxUILocale * self) {
    return new wxLocaleIdent(self->GetLocaleId());
}
wxLayoutDirection wxUILocale_GetLayoutDirection(const wxUILocale * self) {
    return self->GetLayoutDirection();
}
bool wxUILocale_IsSupported(const wxUILocale * self) {
    return self->IsSupported();
}

// CLASS: wxURI
wxClassInfo *wxURI_CLASSINFO() {
    return wxCLASSINFO(wxURI);
}
wxURI *wxURI_new() {
    return new wxURI();
}
wxURI *wxURI_new1(const wxString * uri) {
    return new wxURI(*uri);
}
wxURI *wxURI_new2(const wxURI * uri) {
    return new wxURI(*uri);
}
wxString *wxURI_BuildURI(const wxURI * self) {
    return new wxString(self->BuildURI());
}
wxString *wxURI_BuildUnescapedURI(const wxURI * self) {
    return new wxString(self->BuildUnescapedURI());
}
bool wxURI_Create(wxURI * self, const wxString * uri) {
    return self->Create(*uri);
}
wxString *wxURI_GetFragment(const wxURI * self) {
    return new wxString(self->GetFragment());
}
wxString *wxURI_GetPassword(const wxURI * self) {
    return new wxString(self->GetPassword());
}
wxString *wxURI_GetPath(const wxURI * self) {
    return new wxString(self->GetPath());
}
wxString *wxURI_GetPort(const wxURI * self) {
    return new wxString(self->GetPort());
}
wxString *wxURI_GetQuery(const wxURI * self) {
    return new wxString(self->GetQuery());
}
wxString *wxURI_GetScheme(const wxURI * self) {
    return new wxString(self->GetScheme());
}
wxString *wxURI_GetServer(const wxURI * self) {
    return new wxString(self->GetServer());
}
wxString *wxURI_GetUser(const wxURI * self) {
    return new wxString(self->GetUser());
}
wxString *wxURI_GetUserInfo(const wxURI * self) {
    return new wxString(self->GetUserInfo());
}
bool wxURI_HasFragment(const wxURI * self) {
    return self->HasFragment();
}
bool wxURI_HasPath(const wxURI * self) {
    return self->HasPath();
}
bool wxURI_HasPort(const wxURI * self) {
    return self->HasPort();
}
bool wxURI_HasQuery(const wxURI * self) {
    return self->HasQuery();
}
bool wxURI_HasScheme(const wxURI * self) {
    return self->HasScheme();
}
bool wxURI_HasServer(const wxURI * self) {
    return self->HasServer();
}
bool wxURI_HasUserInfo(const wxURI * self) {
    return self->HasUserInfo();
}
bool wxURI_IsReference(const wxURI * self) {
    return self->IsReference();
}
void wxURI_Resolve(wxURI * self, const wxURI * base, int flags) {
    return self->Resolve(*base, flags);
}
wxString *wxURI_Unescape(const wxString * uri) {
    return new wxString(wxURI::Unescape(*uri));
}

// CLASS: wxUniCharRef
void wxUniCharRef_delete(wxUniCharRef *self) {
    delete self;
}

// CLASS: wxVariantData
void wxVariantData_delete(wxVariantData *self) {
    delete self;
}
wxVariantData *wxVariantData_new() {
    return new wxVariantData();
}
wxVariantData * wxVariantData_Clone(const wxVariantData * self) {
    return self->Clone();
}
void wxVariantData_DecRef(wxVariantData * self) {
    return self->DecRef();
}
bool wxVariantData_Eq(const wxVariantData * self, wxVariantData * data) {
    return self->Eq(*data);
}
bool wxVariantData_GetAny(const wxVariantData * self, wxAny * any) {
    return self->GetAny(any);
}
wxString *wxVariantData_GetType(const wxVariantData * self) {
    return new wxString(self->GetType());
}
wxClassInfo * wxVariantData_GetValueClassInfo(wxVariantData * self) {
    return self->GetValueClassInfo();
}
void wxVariantData_IncRef(wxVariantData * self) {
    return self->IncRef();
}
bool wxVariantData_Read(wxVariantData * self, istream * stream) {
    return self->Read(*stream);
}
bool wxVariantData_Read1(wxVariantData * self, wxString * string) {
    return self->Read(*string);
}
bool wxVariantData_Write(const wxVariantData * self, ostream * stream) {
    return self->Write(*stream);
}
bool wxVariantData_Write1(const wxVariantData * self, wxString * string) {
    return self->Write(*string);
}

// CLASS: wxVersionInfo
void wxVersionInfo_delete(wxVersionInfo *self) {
    delete self;
}
wxVersionInfo *wxVersionInfo_new(const wxString * name, int major, int minor, int micro, int revision, const wxString * description, const wxString * copyright) {
    return new wxVersionInfo(*name, major, minor, micro, revision, *description, *copyright);
}
wxString *wxVersionInfo_GetName(const wxVersionInfo * self) {
    return new wxString(self->GetName());
}
int wxVersionInfo_GetMajor(const wxVersionInfo * self) {
    return self->GetMajor();
}
int wxVersionInfo_GetMinor(const wxVersionInfo * self) {
    return self->GetMinor();
}
int wxVersionInfo_GetMicro(const wxVersionInfo * self) {
    return self->GetMicro();
}
int wxVersionInfo_GetRevision(const wxVersionInfo * self) {
    return self->GetRevision();
}
wxString *wxVersionInfo_ToString(const wxVersionInfo * self) {
    return new wxString(self->ToString());
}
wxString *wxVersionInfo_GetVersionString(const wxVersionInfo * self) {
    return new wxString(self->GetVersionString());
}
bool wxVersionInfo_HasDescription(const wxVersionInfo * self) {
    return self->HasDescription();
}
wxString *wxVersionInfo_GetDescription(wxVersionInfo * self) {
    return new wxString(self->GetDescription());
}
bool wxVersionInfo_HasCopyright(const wxVersionInfo * self) {
    return self->HasCopyright();
}
wxString *wxVersionInfo_GetCopyright(const wxVersionInfo * self) {
    return new wxString(self->GetCopyright());
}

// CLASS: wxWindowUpdateLocker
void wxWindowUpdateLocker_delete(wxWindowUpdateLocker *self) {
    delete self;
}
wxWindowUpdateLocker *wxWindowUpdateLocker_new() {
    return new wxWindowUpdateLocker();
}
wxWindowUpdateLocker *wxWindowUpdateLocker_new1(wxWindow * win) {
    return new wxWindowUpdateLocker(win);
}
void wxWindowUpdateLocker_Lock(wxWindowUpdateLocker * self, wxWindow * win) {
    return self->Lock(win);
}

// CLASS: wxZipClassFactory
wxClassInfo *wxZipClassFactory_CLASSINFO() {
    return wxCLASSINFO(wxZipClassFactory);
}

// CLASS: wxZipNotifier
void wxZipNotifier_delete(wxZipNotifier *self) {
    delete self;
}
void wxZipNotifier_OnEntryUpdated(wxZipNotifier * self, wxZipEntry * entry) {
    return self->OnEntryUpdated(*entry);
}

} // extern "C"

