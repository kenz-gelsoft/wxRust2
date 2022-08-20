#include "generated.h"

extern "C" {

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

// CLASS: wxLogInterposer
void wxLogInterposer_delete(wxLogInterposer *self) {
    delete self;
}
wxLogInterposer *wxLogInterposer_new() {
    return new wxLogInterposer();
}

} // extern "C"

