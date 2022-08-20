#pragma once

#include <wx/ctrlsub.h>
#include <wx/dataobj.h>
#include <wx/event.h>
#include <wx/icon.h>
#include <wx/iconbndl.h>
#include <wx/image.h>
#include <wx/imagiff.h>
#include <wx/imaglist.h>
#include <wx/infobar.h>
#include <wx/itemattr.h>
#include <wx/windowid.h>

extern "C" {

// CLASS: wxIFFHandler
wxClassInfo *wxIFFHandler_CLASSINFO();
wxIFFHandler *wxIFFHandler_new();

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

// CLASS: wxIconBundle
wxClassInfo *wxIconBundle_CLASSINFO();
wxIconBundle *wxIconBundle_new();
wxIconBundle *wxIconBundle_new3(const wxIcon * icon);
wxIconBundle *wxIconBundle_new5(const wxIconBundle * ic);
void wxIconBundle_AddIcon3(wxIconBundle * self, const wxIcon * icon);
wxIcon *wxIconBundle_GetIcon(const wxIconBundle * self, const wxSize * size, int flags);
wxIcon *wxIconBundle_GetIcon1(const wxIconBundle * self, wxCoord size, int flags);
wxIcon *wxIconBundle_GetIconOfExactSize(const wxIconBundle * self, const wxSize * size);
size_t wxIconBundle_GetIconCount(const wxIconBundle * self);
wxIcon *wxIconBundle_GetIconByIndex(const wxIconBundle * self, size_t n);
bool wxIconBundle_IsEmpty(const wxIconBundle * self);

// CLASS: wxIconizeEvent
wxClassInfo *wxIconizeEvent_CLASSINFO();
wxIconizeEvent *wxIconizeEvent_new(int id, bool iconized);
bool wxIconizeEvent_IsIconized(const wxIconizeEvent * self);

// CLASS: wxIdManager
void wxIdManager_delete(wxIdManager *self);
wxWindowID wxIdManager_ReserveId(int count);
void wxIdManager_UnreserveId(wxWindowID id, int count);

// CLASS: wxImage
wxClassInfo *wxImage_CLASSINFO();
wxImage *wxImage_Copy(const wxImage * self);
bool wxImage_Create(wxImage * self, int width, int height, bool clear);
bool wxImage_Create1(wxImage * self, const wxSize * sz, bool clear);
bool wxImage_Create2(wxImage * self, int width, int height, unsigned char * data, bool static_data);
bool wxImage_Create3(wxImage * self, const wxSize * sz, unsigned char * data, bool static_data);
bool wxImage_Create4(wxImage * self, int width, int height, unsigned char * data, unsigned char * alpha, bool static_data);
bool wxImage_Create5(wxImage * self, const wxSize * sz, unsigned char * data, unsigned char * alpha, bool static_data);
void wxImage_Destroy(wxImage * self);
void wxImage_InitAlpha(wxImage * self);
wxImage *wxImage_Blur(const wxImage * self, int blur_radius);
wxImage *wxImage_BlurHorizontal(const wxImage * self, int blur_radius);
wxImage *wxImage_BlurVertical(const wxImage * self, int blur_radius);
wxImage *wxImage_Mirror(const wxImage * self, bool horizontally);
wxImage * wxImage_Resize(wxImage * self, const wxSize * size, const wxPoint * pos, int red, int green, int blue);
wxImage *wxImage_Rotate(const wxImage * self, double angle, const wxPoint * rotation_centre, bool interpolating, wxPoint * offset_after_rotation);
wxImage *wxImage_Rotate90(const wxImage * self, bool clockwise);
wxImage *wxImage_Rotate180(const wxImage * self);
void wxImage_RotateHue(wxImage * self, double angle);
void wxImage_ChangeSaturation(wxImage * self, double factor);
void wxImage_ChangeBrightness(wxImage * self, double factor);
void wxImage_ChangeHSV(wxImage * self, double angle_h, double factor_s, double factor_v);
wxImage *wxImage_Size(const wxImage * self, const wxSize * size, const wxPoint * pos, int red, int green, int blue);
wxImage *wxImage_ConvertToGreyscale(const wxImage * self, double weight_r, double weight_g, double weight_b);
wxImage *wxImage_ConvertToGreyscale1(const wxImage * self);
wxImage *wxImage_ChangeLightness(const wxImage * self, int alpha);
unsigned char * wxImage_GetAlpha(const wxImage * self);
unsigned char * wxImage_GetData(const wxImage * self);
int wxImage_GetWidth(const wxImage * self);
int wxImage_GetHeight(const wxImage * self);
wxSize *wxImage_GetSize(const wxImage * self);
wxString *wxImage_GetOption(const wxImage * self, const wxString * name);
int wxImage_GetOptionInt(const wxImage * self, const wxString * name);
bool wxImage_GetOrFindMaskColour(const wxImage * self, unsigned char * r, unsigned char * g, unsigned char * b);
wxImage *wxImage_GetSubImage(const wxImage * self, const wxRect * rect);
bool wxImage_HasAlpha(const wxImage * self);
bool wxImage_HasMask(const wxImage * self);
bool wxImage_HasOption(const wxImage * self, const wxString * name);
bool wxImage_IsOk(const wxImage * self);
bool wxImage_LoadFile2(wxImage * self, const wxString * name, const wxString * mimetype, int index);
bool wxImage_LoadFile3(wxImage * self, wxInputStream * stream, const wxString * mimetype, int index);
bool wxImage_SaveFile(const wxImage * self, wxOutputStream * stream, const wxString * mimetype);
bool wxImage_SaveFile2(const wxImage * self, const wxString * name, const wxString * mimetype);
bool wxImage_SaveFile3(const wxImage * self, const wxString * name);
void wxImage_SetAlpha(wxImage * self, unsigned char * alpha, bool static_data);
void wxImage_ClearAlpha(wxImage * self);
void wxImage_SetData(wxImage * self, unsigned char * data, bool static_data);
void wxImage_SetData1(wxImage * self, unsigned char * data, int new_width, int new_height, bool static_data);
void wxImage_SetLoadFlags(wxImage * self, int flags);
void wxImage_SetMask(wxImage * self, bool has_mask);
void wxImage_SetOption(wxImage * self, const wxString * name, const wxString * value);
void wxImage_SetOption1(wxImage * self, const wxString * name, int value);
void wxImage_SetPalette(wxImage * self, const wxPalette * palette);
void wxImage_SetDefaultLoadFlags(int flags);
int wxImage_GetLoadFlags(const wxImage * self);
void wxImage_AddHandler(wxImageHandler * handler);
void wxImage_CleanUpHandlers();
wxImageHandler * wxImage_FindHandler(const wxString * name);
wxImageHandler * wxImage_FindHandlerMime(const wxString * mimetype);
void wxImage_InitStandardHandlers();
void wxImage_InsertHandler(wxImageHandler * handler);
bool wxImage_RemoveHandler(const wxString * name);
bool wxImage_CanRead(const wxString * filename);
bool wxImage_CanRead1(wxInputStream * stream);
int wxImage_GetDefaultLoadFlags();
wxString *wxImage_GetImageExtWildcard();
wxImage *wxImage_new();
wxImage *wxImage_new1(int width, int height, bool clear);
wxImage *wxImage_new2(const wxSize * sz, bool clear);
wxImage *wxImage_new3(int width, int height, unsigned char * data, bool static_data);
wxImage *wxImage_new4(const wxSize * sz, unsigned char * data, bool static_data);
wxImage *wxImage_new5(int width, int height, unsigned char * data, unsigned char * alpha, bool static_data);
wxImage *wxImage_new6(const wxSize * sz, unsigned char * data, unsigned char * alpha, bool static_data);
wxImage *wxImage_new7(const char *const * xpm_data);
wxImage *wxImage_new9(const wxString * name, const wxString * mimetype, int index);
wxImage *wxImage_new11(wxInputStream * stream, const wxString * mimetype, int index);

// CLASS: wxImageDataObject
void wxImageDataObject_delete(wxImageDataObject *self);
wxImageDataObject *wxImageDataObject_new(const wxImage * image);
wxImage *wxImageDataObject_GetImage(const wxImageDataObject * self);
void wxImageDataObject_SetImage(wxImageDataObject * self, const wxImage * image);

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

// CLASS: wxImageList
wxClassInfo *wxImageList_CLASSINFO();
wxImageList *wxImageList_new();
wxImageList *wxImageList_new1(int width, int height, bool mask, int initial_count);
int wxImageList_Add(wxImageList * self, const wxBitmap * bitmap, const wxBitmap * mask);
int wxImageList_Add1(wxImageList * self, const wxBitmap * bitmap, const wxColour * mask_colour);
int wxImageList_Add2(wxImageList * self, const wxIcon * icon);
bool wxImageList_Create(wxImageList * self, int width, int height, bool mask, int initial_count);
void wxImageList_Destroy(wxImageList * self);
bool wxImageList_Draw(wxImageList * self, int index, wxDC * dc, int x, int y, int flags, bool solid_background);
wxBitmap *wxImageList_GetBitmap(const wxImageList * self, int index);
wxIcon *wxImageList_GetIcon(const wxImageList * self, int index);
int wxImageList_GetImageCount(const wxImageList * self);
bool wxImageList_GetSize(const wxImageList * self, int index, int * width, int * height);
wxSize *wxImageList_GetSize1(const wxImageList * self);
bool wxImageList_Remove(wxImageList * self, int index);
bool wxImageList_RemoveAll(wxImageList * self);
bool wxImageList_Replace(wxImageList * self, int index, const wxBitmap * bitmap, const wxBitmap * mask);
bool wxImageList_Replace1(wxImageList * self, int index, const wxIcon * icon);

// CLASS: wxInfoBar
wxClassInfo *wxInfoBar_CLASSINFO();
void wxInfoBar_SetEffectDuration(wxInfoBar * self, int duration);
int wxInfoBar_GetEffectDuration(const wxInfoBar * self);
wxInfoBar *wxInfoBar_new();
wxInfoBar *wxInfoBar_new1(wxWindow * parent, wxWindowID winid);
bool wxInfoBar_Create(wxInfoBar * self, wxWindow * parent, wxWindowID winid);
void wxInfoBar_AddButton(wxInfoBar * self, wxWindowID btnid, const wxString * label);
void wxInfoBar_Dismiss(wxInfoBar * self);
void wxInfoBar_RemoveButton(wxInfoBar * self, wxWindowID btnid);
void wxInfoBar_ShowMessage(wxInfoBar * self, const wxString * msg, int flags);
size_t wxInfoBar_GetButtonCount(const wxInfoBar * self);
wxWindowID wxInfoBar_GetButtonId(const wxInfoBar * self, size_t idx);
bool wxInfoBar_HasButtonId(const wxInfoBar * self, wxWindowID btnid);

// CLASS: wxInitDialogEvent
wxClassInfo *wxInitDialogEvent_CLASSINFO();
wxInitDialogEvent *wxInitDialogEvent_new(int id);

// CLASS: wxItemAttr
void wxItemAttr_delete(wxItemAttr *self);
wxItemAttr *wxItemAttr_new();
wxItemAttr *wxItemAttr_new1(const wxColour * col_text, const wxColour * col_back, const wxFont * font);
wxColour *wxItemAttr_GetBackgroundColour(const wxItemAttr * self);
wxFont *wxItemAttr_GetFont(const wxItemAttr * self);
wxColour *wxItemAttr_GetTextColour(const wxItemAttr * self);
bool wxItemAttr_HasBackgroundColour(const wxItemAttr * self);
bool wxItemAttr_HasColours(const wxItemAttr * self);
bool wxItemAttr_HasFont(const wxItemAttr * self);
bool wxItemAttr_HasTextColour(const wxItemAttr * self);
bool wxItemAttr_IsDefault(const wxItemAttr * self);
void wxItemAttr_SetBackgroundColour(wxItemAttr * self, const wxColour * colour);
void wxItemAttr_SetFont(wxItemAttr * self, const wxFont * font);
void wxItemAttr_SetTextColour(wxItemAttr * self, const wxColour * colour);

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

