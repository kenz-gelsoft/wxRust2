#pragma once
#include <wx/wx.h>
#include <wx/bitmap.h>
#include <wx/bmpbndl.h>
#include <wx/bmpbuttn.h>
#include <wx/bookctrl.h>
#include <wx/button.h>
#include <wx/sizer.h>
#include <wx/tglbtn.h>

// wxBitmapBundle compatibility hack(for a while)
#if !wxCHECK_VERSION(3, 1, 6)
typedef wxBitmap wxBitmapBundle;
#endif

extern "C" {

// CLASS: wxBitmap
wxClassInfo *wxBitmap_CLASSINFO();
wxBitmap *wxBitmap_new();
wxBitmap *wxBitmap_new1(const wxBitmap * bitmap);
wxBitmap *wxBitmap_new3(int width, int height, int depth);
wxBitmap *wxBitmap_new4(const wxSize * sz, int depth);
#if wxCHECK_VERSION(3, 1, 0)
wxBitmap *wxBitmap_new5(int width, int height, const wxDC * dc);
#endif
wxBitmap *wxBitmap_new6(const char *const * bits);
#if wxCHECK_VERSION(3, 1, 0)
wxBitmap *wxBitmap_new8(const wxImage * img, int depth);
#endif
#if wxCHECK_VERSION(3, 1, 7)
wxBitmap *wxBitmap_new9(const wxImage * img, const wxDC * dc);
#endif
#if wxCHECK_VERSION(3, 1, 0)
wxBitmap *wxBitmap_new10(const wxCursor * cursor);
#endif
bool wxBitmap_CopyFromIcon(wxBitmap * self, const wxIcon * icon);
bool wxBitmap_Create(wxBitmap * self, int width, int height, int depth);
bool wxBitmap_Create1(wxBitmap * self, const wxSize * sz, int depth);
bool wxBitmap_Create2(wxBitmap * self, int width, int height, const wxDC * dc);
#if wxCHECK_VERSION(3, 1, 0)
bool wxBitmap_CreateWithDIPSize(wxBitmap * self, const wxSize * size, double scale, int depth);
bool wxBitmap_CreateWithDIPSize1(wxBitmap * self, int width, int height, double scale, int depth);
#endif
bool wxBitmap_CreateScaled(wxBitmap * self, int width, int height, int depth, double logical_scale);
int wxBitmap_GetDepth(const wxBitmap * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxBitmap_GetDIPSize(const wxBitmap * self);
#endif
int wxBitmap_GetHeight(const wxBitmap * self);
#if wxCHECK_VERSION(3, 1, 0)
double wxBitmap_GetLogicalHeight(const wxBitmap * self);
wxSize *wxBitmap_GetLogicalSize(const wxBitmap * self);
double wxBitmap_GetLogicalWidth(const wxBitmap * self);
#endif
wxMask * wxBitmap_GetMask(const wxBitmap * self);
wxPalette * wxBitmap_GetPalette(const wxBitmap * self);
wxBitmap *wxBitmap_GetSubBitmap(const wxBitmap * self, const wxRect * rect);
double wxBitmap_GetScaleFactor(const wxBitmap * self);
double wxBitmap_GetScaledHeight(const wxBitmap * self);
wxSize *wxBitmap_GetScaledSize(const wxBitmap * self);
double wxBitmap_GetScaledWidth(const wxBitmap * self);
wxSize *wxBitmap_GetSize(const wxBitmap * self);
int wxBitmap_GetWidth(const wxBitmap * self);
bool wxBitmap_HasAlpha(const wxBitmap * self);
bool wxBitmap_IsOk(const wxBitmap * self);
#if wxCHECK_VERSION(3, 1, 7)
void wxBitmap_SetDepth(wxBitmap * self, int depth);
void wxBitmap_SetHeight(wxBitmap * self, int height);
#endif
#if wxCHECK_VERSION(3, 1, 0)
void wxBitmap_SetScaleFactor(wxBitmap * self, double scale);
#endif
void wxBitmap_SetMask(wxBitmap * self, wxMask * mask);
void wxBitmap_SetPalette(wxBitmap * self, const wxPalette * palette);
#if wxCHECK_VERSION(3, 1, 7)
void wxBitmap_SetWidth(wxBitmap * self, int width);
#endif
void wxBitmap_AddHandler(wxBitmapHandler * handler);
void wxBitmap_CleanUpHandlers();
#ifndef __WXMSW__
wxBitmapHandler * wxBitmap_FindHandler(const wxString * name);
#endif
void wxBitmap_InitStandardHandlers();
void wxBitmap_InsertHandler(wxBitmapHandler * handler);
wxBitmap *wxBitmap_NewFromPNGData(const void * data, size_t size);
bool wxBitmap_RemoveHandler(const wxString * name);
#if wxCHECK_VERSION(3, 1, 0)
void wxBitmap_Rescale(wxBitmap * bmp, const wxSize * size_needed);
#endif

// CLASS: wxBitmapBundle
void wxBitmapBundle_delete(wxBitmapBundle *self);
wxBitmapBundle *wxBitmapBundle_new();
wxBitmapBundle *wxBitmapBundle_new1(const wxBitmap * bitmap);
wxBitmapBundle *wxBitmapBundle_new2(const wxIcon * icon);
wxBitmapBundle *wxBitmapBundle_new3(const wxImage * image);
#if wxCHECK_VERSION(3, 2, 0)
wxBitmapBundle *wxBitmapBundle_new4(const char *const * xpm);
#endif
wxBitmapBundle *wxBitmapBundle_new5(const wxBitmapBundle * other);
#if wxCHECK_VERSION(3, 1, 7)
void wxBitmapBundle_Clear(wxBitmapBundle * self);
#endif
bool wxBitmapBundle_IsOk(const wxBitmapBundle * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxBitmapBundle_GetDefaultSize(const wxBitmapBundle * self);
wxSize *wxBitmapBundle_GetPreferredBitmapSizeAtScale(const wxBitmapBundle * self, double scale);
wxSize *wxBitmapBundle_GetPreferredBitmapSizeFor(const wxBitmapBundle * self, const wxWindow * window);
wxSize *wxBitmapBundle_GetPreferredLogicalSizeFor(const wxBitmapBundle * self, const wxWindow * window);
wxBitmap *wxBitmapBundle_GetBitmap(const wxBitmapBundle * self, const wxSize * size);
wxBitmap *wxBitmapBundle_GetBitmapFor(const wxBitmapBundle * self, const wxWindow * window);
wxIcon *wxBitmapBundle_GetIcon(const wxBitmapBundle * self, const wxSize * size);
wxIcon *wxBitmapBundle_GetIconFor(const wxBitmapBundle * self, const wxWindow * window);
#endif
bool wxBitmapBundle_IsSameAs(const wxBitmapBundle * self, const wxBitmapBundle * other);
#if wxCHECK_VERSION(3, 1, 0)
wxBitmapBundle *wxBitmapBundle_FromBitmaps1(const wxBitmap * bitmap1, const wxBitmap * bitmap2);
wxBitmapBundle *wxBitmapBundle_FromBitmap(const wxBitmap * bitmap);
#endif
#if wxCHECK_VERSION(3, 1, 7)
wxBitmapBundle *wxBitmapBundle_FromIconBundle(const wxIconBundle * icon_bundle);
#endif
#if wxCHECK_VERSION(3, 1, 0)
wxBitmapBundle *wxBitmapBundle_FromImage(const wxImage * image);
wxBitmapBundle *wxBitmapBundle_FromImpl(wxBitmapBundleImpl * impl_);
wxBitmapBundle *wxBitmapBundle_FromResources(const wxString * name);
wxBitmapBundle *wxBitmapBundle_FromFiles(const wxString * path, const wxString * filename, const wxString * extension);
wxBitmapBundle *wxBitmapBundle_FromFiles1(const wxString * fullpathname);
wxBitmapBundle *wxBitmapBundle_FromSVG1(const char * data, const wxSize * size_def);
wxBitmapBundle *wxBitmapBundle_FromSVGFile(const wxString * path, const wxSize * size_def);
wxBitmapBundle *wxBitmapBundle_FromSVGResource(const wxString * name, const wxSize * size_def);
#endif

// CLASS: wxBitmapButton
wxClassInfo *wxBitmapButton_CLASSINFO();
wxBitmapButton *wxBitmapButton_new();
wxBitmapButton *wxBitmapButton_new1(wxWindow * parent, wxWindowID id, const wxBitmapBundle * bitmap, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxBitmapButton_Create(wxBitmapButton * self, wxWindow * parent, wxWindowID id, const wxBitmapBundle * bitmap, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
#if wxCHECK_VERSION(3, 1, 0)
bool wxBitmapButton_CreateCloseButton(wxBitmapButton * self, wxWindow * parent, wxWindowID winid, const wxString * name);
wxBitmapButton * wxBitmapButton_NewCloseButton(wxWindow * parent, wxWindowID winid, const wxString * name);
#endif

// CLASS: wxBitmapToggleButton
wxClassInfo *wxBitmapToggleButton_CLASSINFO();
wxBitmapToggleButton *wxBitmapToggleButton_new();
wxBitmapToggleButton *wxBitmapToggleButton_new1(wxWindow * parent, wxWindowID id, const wxBitmapBundle * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * val, const wxString * name);
bool wxBitmapToggleButton_Create(wxBitmapToggleButton * self, wxWindow * parent, wxWindowID id, const wxBitmapBundle * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * val, const wxString * name);

// CLASS: wxBookCtrlBase
wxClassInfo *wxBookCtrlBase_CLASSINFO();
int wxBookCtrlBase_GetPageImage(const wxBookCtrlBase * self, size_t n_page);
bool wxBookCtrlBase_SetPageImage(wxBookCtrlBase * self, size_t page, int image);
wxString *wxBookCtrlBase_GetPageText(const wxBookCtrlBase * self, size_t n_page);
bool wxBookCtrlBase_SetPageText(wxBookCtrlBase * self, size_t page, const wxString * text);
int wxBookCtrlBase_GetSelection(const wxBookCtrlBase * self);
wxWindow * wxBookCtrlBase_GetCurrentPage(const wxBookCtrlBase * self);
int wxBookCtrlBase_SetSelection(wxBookCtrlBase * self, size_t page);
void wxBookCtrlBase_AdvanceSelection(wxBookCtrlBase * self, bool forward);
int wxBookCtrlBase_ChangeSelection(wxBookCtrlBase * self, size_t page);
int wxBookCtrlBase_FindPage(const wxBookCtrlBase * self, const wxWindow * page);
void wxBookCtrlBase_SetPageSize(wxBookCtrlBase * self, const wxSize * size);
int wxBookCtrlBase_HitTest(const wxBookCtrlBase * self, const wxPoint * pt, long * flags);
bool wxBookCtrlBase_AddPage(wxBookCtrlBase * self, wxWindow * page, const wxString * text, bool select, int image_id);
bool wxBookCtrlBase_DeleteAllPages(wxBookCtrlBase * self);
bool wxBookCtrlBase_DeletePage(wxBookCtrlBase * self, size_t page);
bool wxBookCtrlBase_InsertPage(wxBookCtrlBase * self, size_t index, wxWindow * page, const wxString * text, bool select, int image_id);
bool wxBookCtrlBase_RemovePage(wxBookCtrlBase * self, size_t page);
size_t wxBookCtrlBase_GetPageCount(const wxBookCtrlBase * self);
wxWindow * wxBookCtrlBase_GetPage(const wxBookCtrlBase * self, size_t page);
bool wxBookCtrlBase_Create(wxBookCtrlBase * self, wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name);

// CLASS: wxBookCtrlEvent
wxClassInfo *wxBookCtrlEvent_CLASSINFO();
int wxBookCtrlEvent_GetOldSelection(const wxBookCtrlEvent * self);
int wxBookCtrlEvent_GetSelection(const wxBookCtrlEvent * self);
void wxBookCtrlEvent_SetOldSelection(wxBookCtrlEvent * self, int page);
void wxBookCtrlEvent_SetSelection(wxBookCtrlEvent * self, int page);

// CLASS: wxBoxSizer
wxClassInfo *wxBoxSizer_CLASSINFO();
wxBoxSizer *wxBoxSizer_new(int orient);
int wxBoxSizer_GetOrientation(const wxBoxSizer * self);
void wxBoxSizer_SetOrientation(wxBoxSizer * self, int orient);

// CLASS: wxButton
wxClassInfo *wxButton_CLASSINFO();
wxButton *wxButton_new();
wxButton *wxButton_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxButton_Create(wxButton * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxButton_GetAuthNeeded(const wxButton * self);
void wxButton_SetAuthNeeded(wxButton * self, bool needed);
wxWindow * wxButton_SetDefault(wxButton * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxButton_GetDefaultSize(wxWindow * win);
#endif

} // extern "C"
