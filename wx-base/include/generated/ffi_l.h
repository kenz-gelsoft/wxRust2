#pragma once

#include <wx/log.h>

extern "C" {

// CLASS: wxLog
void wxLog_delete(wxLog *self);
void wxLog_AddTraceMask(const wxString * mask);
void wxLog_ClearTraceMasks();
wxArrayString *wxLog_GetTraceMasks();
bool wxLog_IsAllowedTraceMask(const wxString * mask);
void wxLog_RemoveTraceMask(const wxString * mask);
void wxLog_DontCreateOnDemand();
wxLog * wxLog_GetActiveTarget();
wxLog * wxLog_SetActiveTarget(wxLog * logtarget);
wxLog * wxLog_SetThreadActiveTarget(wxLog * logger);
void wxLog_FlushActive();
void wxLog_Resume();
void wxLog_Suspend();
bool wxLog_EnableLogging(bool enable);
bool wxLog_IsEnabled();
bool wxLog_GetRepetitionCounting();
void wxLog_SetRepetitionCounting(bool repet_counting);
wxString *wxLog_GetTimestamp();
void wxLog_SetTimestamp(const wxString * format);
void wxLog_DisableTimestamp();
bool wxLog_GetVerbose();
void wxLog_SetVerbose(bool verbose);
wxLogFormatter * wxLog_SetFormatter(wxLog * self, wxLogFormatter * formatter);
void wxLog_Flush(wxLog * self);

// CLASS: wxLogChain
void wxLogChain_delete(wxLogChain *self);
wxLogChain *wxLogChain_new(wxLog * logger);
void wxLogChain_DetachOldLog(wxLogChain * self);
wxLog * wxLogChain_GetOldLog(const wxLogChain * self);
bool wxLogChain_IsPassingMessages(const wxLogChain * self);
void wxLogChain_PassMessages(wxLogChain * self, bool pass_messages);
void wxLogChain_SetLog(wxLogChain * self, wxLog * logger);

// CLASS: wxLogInterposer
void wxLogInterposer_delete(wxLogInterposer *self);
wxLogInterposer *wxLogInterposer_new();

} // extern "C"

