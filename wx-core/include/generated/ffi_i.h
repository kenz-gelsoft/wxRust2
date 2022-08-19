#pragma once

#include <wx/ctrlsub.h>
#include <wx/icon.h>
#include <wx/image.h>

extern "C" {

// CLASS: wxIcon
wxClassInfo *wxIcon_CLASSINFO();
wxIcon *wxIcon_new();
wxIcon *wxIcon_new1(const wxIcon * icon);
wxIcon *wxIcon_new3(const char *const * bits);
wxIcon *wxIcon_new5(const wxIconLocation * loc);
void wxIcon_CopyFromBitmap(wxIcon * self, const wxBitmap * bmp);
int wxIcon_GetDepth(const wxIcon * self);
int wxIcon_GetHeight(const wxIcon * self);
#if wxCHECK_VERSION(3, 1, 0)
double wxIcon_GetLogicalHeight(const wxIcon * self);
wxSize *wxIcon_GetLogicalSize(const wxIcon * self);
double wxIcon_GetLogicalWidth(const wxIcon * self);
#endif
double wxIcon_GetScaleFactor(const wxIcon * self);
wxSize *wxIcon_GetSize(const wxIcon * self);
int wxIcon_GetWidth(const wxIcon * self);
bool wxIcon_IsOk(const wxIcon * self);
#if wxCHECK_VERSION(3, 1, 7)
void wxIcon_SetDepth(wxIcon * self, int depth);
void wxIcon_SetHeight(wxIcon * self, int height);
void wxIcon_SetWidth(wxIcon * self, int width);
#endif

// CLASS: wxImageHandler
wxClassInfo *wxImageHandler_CLASSINFO();
bool wxImageHandler_CanRead(wxImageHandler * self, wxInputStream * stream);
bool wxImageHandler_CanRead1(wxImageHandler * self, const wxString * filename);
wxString *wxImageHandler_GetExtension(const wxImageHandler * self);
wxArrayString *wxImageHandler_GetAltExtensions(const wxImageHandler * self);
int wxImageHandler_GetImageCount(wxImageHandler * self, wxInputStream * stream);
wxString *wxImageHandler_GetMimeType(const wxImageHandler * self);
wxString *wxImageHandler_GetName(const wxImageHandler * self);
bool wxImageHandler_LoadFile(wxImageHandler * self, wxImage * image, wxInputStream * stream, bool verbose, int index);
bool wxImageHandler_SaveFile(wxImageHandler * self, wxImage * image, wxOutputStream * stream, bool verbose);
void wxImageHandler_SetExtension(wxImageHandler * self, const wxString * extension);
void wxImageHandler_SetAltExtensions(wxImageHandler * self, const wxArrayString * extensions);
void wxImageHandler_SetMimeType(wxImageHandler * self, const wxString * mimetype);
void wxImageHandler_SetName(wxImageHandler * self, const wxString * name);

// CLASS: wxItemContainer
void wxItemContainer_delete(wxItemContainer *self);
int wxItemContainer_Append(wxItemContainer * self, const wxString * item);
int wxItemContainer_Append1(wxItemContainer * self, const wxString * item, void * client_data);
int wxItemContainer_Append2(wxItemContainer * self, const wxString * item, wxClientData * client_data);
int wxItemContainer_Append3(wxItemContainer * self, const wxArrayString * items);
int wxItemContainer_Append5(wxItemContainer * self, const wxArrayString * items, void ** client_data);
int wxItemContainer_Append6(wxItemContainer * self, const wxArrayString * items, wxClientData ** client_data);
int wxItemContainer_Append7(wxItemContainer * self, unsigned int n, const wxString * items);
int wxItemContainer_Append8(wxItemContainer * self, unsigned int n, const wxString * items, void ** client_data);
int wxItemContainer_Append9(wxItemContainer * self, unsigned int n, const wxString * items, wxClientData ** client_data);
void wxItemContainer_Clear(wxItemContainer * self);
void wxItemContainer_Delete(wxItemContainer * self, unsigned int n);
wxClientData * wxItemContainer_DetachClientObject(wxItemContainer * self, unsigned int n);
bool wxItemContainer_HasClientData(const wxItemContainer * self);
bool wxItemContainer_HasClientObjectData(const wxItemContainer * self);
bool wxItemContainer_HasClientUntypedData(const wxItemContainer * self);
void * wxItemContainer_GetClientData(const wxItemContainer * self, unsigned int n);
wxClientData * wxItemContainer_GetClientObject(const wxItemContainer * self, unsigned int n);
void wxItemContainer_SetClientData(wxItemContainer * self, unsigned int n, void * data);
void wxItemContainer_SetClientObject(wxItemContainer * self, unsigned int n, wxClientData * data);
int wxItemContainer_Insert(wxItemContainer * self, const wxString * item, unsigned int pos);
int wxItemContainer_Insert1(wxItemContainer * self, const wxString * item, unsigned int pos, void * client_data);
int wxItemContainer_Insert2(wxItemContainer * self, const wxString * item, unsigned int pos, wxClientData * client_data);
int wxItemContainer_Insert3(wxItemContainer * self, const wxArrayString * items, unsigned int pos);
int wxItemContainer_Insert5(wxItemContainer * self, const wxArrayString * items, unsigned int pos, void ** client_data);
int wxItemContainer_Insert6(wxItemContainer * self, const wxArrayString * items, unsigned int pos, wxClientData ** client_data);
int wxItemContainer_Insert7(wxItemContainer * self, unsigned int n, const wxString * items, unsigned int pos);
int wxItemContainer_Insert8(wxItemContainer * self, unsigned int n, const wxString * items, unsigned int pos, void ** client_data);
int wxItemContainer_Insert9(wxItemContainer * self, unsigned int n, const wxString * items, unsigned int pos, wxClientData ** client_data);
void wxItemContainer_Set(wxItemContainer * self, const wxArrayString * items);
void wxItemContainer_Set2(wxItemContainer * self, const wxArrayString * items, void ** client_data);
void wxItemContainer_Set3(wxItemContainer * self, const wxArrayString * items, wxClientData ** client_data);
void wxItemContainer_Set4(wxItemContainer * self, unsigned int n, const wxString * items);
void wxItemContainer_Set5(wxItemContainer * self, unsigned int n, const wxString * items, void ** client_data);
void wxItemContainer_Set6(wxItemContainer * self, unsigned int n, const wxString * items, wxClientData ** client_data);

// CLASS: wxItemContainerImmutable
void wxItemContainerImmutable_delete(wxItemContainerImmutable *self);
void wxItemContainerImmutable_SetSelection(wxItemContainerImmutable * self, int n);
int wxItemContainerImmutable_GetSelection(const wxItemContainerImmutable * self);
bool wxItemContainerImmutable_SetStringSelection(wxItemContainerImmutable * self, const wxString * string);
wxString *wxItemContainerImmutable_GetStringSelection(const wxItemContainerImmutable * self);
void wxItemContainerImmutable_Select(wxItemContainerImmutable * self, int n);
unsigned int wxItemContainerImmutable_GetCount(const wxItemContainerImmutable * self);
bool wxItemContainerImmutable_IsEmpty(const wxItemContainerImmutable * self);
wxString *wxItemContainerImmutable_GetString(const wxItemContainerImmutable * self, unsigned int n);
wxArrayString *wxItemContainerImmutable_GetStrings(const wxItemContainerImmutable * self);
void wxItemContainerImmutable_SetString(wxItemContainerImmutable * self, unsigned int n, const wxString * string);
int wxItemContainerImmutable_FindString(const wxItemContainerImmutable * self, const wxString * string, bool case_sensitive);

} // extern "C"

