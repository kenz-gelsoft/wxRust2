#pragma once
#include <wx/wx.h>
//#include <wx/activityindicator.h>
#include <wx/artprov.h>
#include <wx/bookctrl.h>
#include <wx/clrpicker.h>
#include <wx/datectrl.h>
#include <wx/filepicker.h>
#include <wx/wrapsizer.h>

extern "C" {

// CLASS: wxAnyButton
wxAnyButton *wxAnyButton_new();
wxBitmap *wxAnyButton_GetBitmap(const wxAnyButton * self);
wxBitmap *wxAnyButton_GetBitmapCurrent(const wxAnyButton * self);
wxBitmap *wxAnyButton_GetBitmapDisabled(const wxAnyButton * self);
wxBitmap *wxAnyButton_GetBitmapFocus(const wxAnyButton * self);
wxBitmap *wxAnyButton_GetBitmapLabel(const wxAnyButton * self);
wxBitmap *wxAnyButton_GetBitmapPressed(const wxAnyButton * self);
void wxAnyButton_SetBitmap(wxAnyButton * self, const wxBitmapBundle * bitmap, wxDirection dir);
void wxAnyButton_SetBitmapCurrent(wxAnyButton * self, const wxBitmapBundle * bitmap);
void wxAnyButton_SetBitmapDisabled(wxAnyButton * self, const wxBitmapBundle * bitmap);
void wxAnyButton_SetBitmapFocus(wxAnyButton * self, const wxBitmapBundle * bitmap);
void wxAnyButton_SetBitmapLabel(wxAnyButton * self, const wxBitmapBundle * bitmap);
void wxAnyButton_SetBitmapPressed(wxAnyButton * self, const wxBitmapBundle * bitmap);
wxSize *wxAnyButton_GetBitmapMargins(wxAnyButton * self);
void wxAnyButton_SetBitmapMargins(wxAnyButton * self, wxCoord x, wxCoord y);
void wxAnyButton_SetBitmapMargins1(wxAnyButton * self, const wxSize * sz);
void wxAnyButton_SetBitmapPosition(wxAnyButton * self, wxDirection dir);

// CLASS: wxArtProvider
bool wxArtProvider_Delete(wxArtProvider * provider);
wxBitmap *wxArtProvider_GetBitmap(const wxArtID * id, const wxArtClient * client, const wxSize * size);
wxBitmapBundle *wxArtProvider_GetBitmapBundle(const wxArtID * id, const wxArtClient * client, const wxSize * size);
wxIcon *wxArtProvider_GetIcon(const wxArtID * id, const wxArtClient * client, const wxSize * size);
wxSize *wxArtProvider_GetNativeDIPSizeHint(const wxArtClient * client);
wxSize *wxArtProvider_GetNativeSizeHint(const wxArtClient * client, wxWindow * win);
wxSize *wxArtProvider_GetDIPSizeHint(const wxArtClient * client);
#if wxCHECK_VERSION(3, 1, 7)
wxSize *wxArtProvider_GetSizeHint(const wxArtClient * client, wxWindow * win);
#endif
bool wxArtProvider_HasNativeProvider();
bool wxArtProvider_Pop();
void wxArtProvider_Push(wxArtProvider * provider);
void wxArtProvider_PushBack(wxArtProvider * provider);
bool wxArtProvider_Remove(wxArtProvider * provider);
wxArtID *wxArtProvider_GetMessageBoxIconId(int flags);
wxIcon *wxArtProvider_GetMessageBoxIcon(int flags);

// CLASS: wxBitmap
wxBitmap *wxBitmap_new();
wxBitmap *wxBitmap_new1(const wxBitmap * bitmap);
wxBitmap *wxBitmap_new3(int width, int height, int depth);
wxBitmap *wxBitmap_new4(const wxSize * sz, int depth);
wxBitmap *wxBitmap_new5(int width, int height, const wxDC * dc);
wxBitmap *wxBitmap_new6(const char *const * bits);
#if wxCHECK_VERSION(3, 1, 0)
wxBitmap *wxBitmap_new8(const wxImage * img, int depth);
#endif
wxBitmap *wxBitmap_new9(const wxImage * img, const wxDC * dc);
wxBitmap *wxBitmap_new10(const wxCursor * cursor);
bool wxBitmap_CopyFromIcon(wxBitmap * self, const wxIcon * icon);
bool wxBitmap_Create(wxBitmap * self, int width, int height, int depth);
bool wxBitmap_Create1(wxBitmap * self, const wxSize * sz, int depth);
bool wxBitmap_Create2(wxBitmap * self, int width, int height, const wxDC * dc);
bool wxBitmap_CreateWithDIPSize(wxBitmap * self, const wxSize * size, double scale, int depth);
bool wxBitmap_CreateWithDIPSize1(wxBitmap * self, int width, int height, double scale, int depth);
bool wxBitmap_CreateScaled(wxBitmap * self, int width, int height, int depth, double logical_scale);
int wxBitmap_GetDepth(const wxBitmap * self);
wxSize *wxBitmap_GetDIPSize(const wxBitmap * self);
int wxBitmap_GetHeight(const wxBitmap * self);
double wxBitmap_GetLogicalHeight(const wxBitmap * self);
wxSize *wxBitmap_GetLogicalSize(const wxBitmap * self);
double wxBitmap_GetLogicalWidth(const wxBitmap * self);
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
void wxBitmap_ResetAlpha(wxBitmap * self);
#if wxCHECK_VERSION(3, 1, 7)
void wxBitmap_SetDepth(wxBitmap * self, int depth);
void wxBitmap_SetHeight(wxBitmap * self, int height);
#endif
void wxBitmap_SetScaleFactor(wxBitmap * self, double scale);
void wxBitmap_SetMask(wxBitmap * self, wxMask * mask);
void wxBitmap_SetPalette(wxBitmap * self, const wxPalette * palette);
#if wxCHECK_VERSION(3, 1, 7)
void wxBitmap_SetWidth(wxBitmap * self, int width);
#endif
#ifndef __WXMSW__
bool wxBitmap_UseAlpha(wxBitmap * self, bool use_);
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
void wxBitmap_Rescale(wxBitmap * bmp, const wxSize * size_needed);

// CLASS: wxBitmapBundle
void wxBitmapBundle_delete(wxBitmapBundle *self);
wxBitmapBundle *wxBitmapBundle_new();
wxBitmapBundle *wxBitmapBundle_new1(const wxBitmap * bitmap);
wxBitmapBundle *wxBitmapBundle_new2(const wxIcon * icon);
wxBitmapBundle *wxBitmapBundle_new3(const wxImage * image);
wxBitmapBundle *wxBitmapBundle_new4(const char *const * xpm);
wxBitmapBundle *wxBitmapBundle_new5(const wxBitmapBundle * other);
void wxBitmapBundle_Clear(wxBitmapBundle * self);
bool wxBitmapBundle_IsOk(const wxBitmapBundle * self);
wxSize *wxBitmapBundle_GetDefaultSize(const wxBitmapBundle * self);
wxSize *wxBitmapBundle_GetPreferredBitmapSizeAtScale(const wxBitmapBundle * self, double scale);
wxSize *wxBitmapBundle_GetPreferredBitmapSizeFor(const wxBitmapBundle * self, const wxWindow * window);
wxSize *wxBitmapBundle_GetPreferredLogicalSizeFor(const wxBitmapBundle * self, const wxWindow * window);
wxBitmap *wxBitmapBundle_GetBitmap(const wxBitmapBundle * self, const wxSize * size);
wxBitmap *wxBitmapBundle_GetBitmapFor(const wxBitmapBundle * self, const wxWindow * window);
wxIcon *wxBitmapBundle_GetIcon(const wxBitmapBundle * self, const wxSize * size);
wxIcon *wxBitmapBundle_GetIconFor(const wxBitmapBundle * self, const wxWindow * window);
bool wxBitmapBundle_IsSameAs(const wxBitmapBundle * self, const wxBitmapBundle * other);
wxBitmapBundle *wxBitmapBundle_FromBitmaps1(const wxBitmap * bitmap1, const wxBitmap * bitmap2);
wxBitmapBundle *wxBitmapBundle_FromBitmap(const wxBitmap * bitmap);
wxBitmapBundle *wxBitmapBundle_FromIconBundle(const wxIconBundle * icon_bundle);
wxBitmapBundle *wxBitmapBundle_FromImage(const wxImage * image);
wxBitmapBundle *wxBitmapBundle_FromImpl(wxBitmapBundleImpl * impl_);
wxBitmapBundle *wxBitmapBundle_FromResources(const wxString * name);
wxBitmapBundle *wxBitmapBundle_FromFiles(const wxString * path, const wxString * filename, const wxString * extension);
wxBitmapBundle *wxBitmapBundle_FromFiles1(const wxString * fullpathname);
wxBitmapBundle *wxBitmapBundle_FromSVG1(const char * data, const wxSize * size_def);
wxBitmapBundle *wxBitmapBundle_FromSVGFile(const wxString * path, const wxSize * size_def);
wxBitmapBundle *wxBitmapBundle_FromSVGResource(const wxString * name, const wxSize * size_def);

// CLASS: wxBitmapButton
wxBitmapButton *wxBitmapButton_new();
wxBitmapButton *wxBitmapButton_new1(wxWindow * parent, wxWindowID id, const wxBitmapBundle * bitmap, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxBitmapButton_Create(wxBitmapButton * self, wxWindow * parent, wxWindowID id, const wxBitmapBundle * bitmap, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
#if wxCHECK_VERSION(3, 1, 0)
bool wxBitmapButton_CreateCloseButton(wxBitmapButton * self, wxWindow * parent, wxWindowID winid, const wxString * name);
wxBitmapButton * wxBitmapButton_NewCloseButton(wxWindow * parent, wxWindowID winid, const wxString * name);
#endif

// CLASS: wxBookCtrlBase
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
int wxBookCtrlEvent_GetOldSelection(const wxBookCtrlEvent * self);
int wxBookCtrlEvent_GetSelection(const wxBookCtrlEvent * self);
void wxBookCtrlEvent_SetOldSelection(wxBookCtrlEvent * self, int page);
void wxBookCtrlEvent_SetSelection(wxBookCtrlEvent * self, int page);

// CLASS: wxBoxSizer
wxBoxSizer *wxBoxSizer_new(int orient);
int wxBoxSizer_GetOrientation(const wxBoxSizer * self);
void wxBoxSizer_SetOrientation(wxBoxSizer * self, int orient);

// CLASS: wxButton
wxButton *wxButton_new();
wxButton *wxButton_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxButton_Create(wxButton * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxButton_GetAuthNeeded(const wxButton * self);
void wxButton_SetAuthNeeded(wxButton * self, bool needed);
wxWindow * wxButton_SetDefault(wxButton * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxButton_GetDefaultSize(wxWindow * win);
#endif

// CLASS: wxCheckBox
wxCheckBox *wxCheckBox_new();
wxCheckBox *wxCheckBox_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxCheckBox_Create(wxCheckBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxCheckBox_GetValue(const wxCheckBox * self);
wxCheckBoxState wxCheckBox_Get3StateValue(const wxCheckBox * self);
bool wxCheckBox_Is3State(const wxCheckBox * self);
bool wxCheckBox_Is3rdStateAllowedForUser(const wxCheckBox * self);
bool wxCheckBox_IsChecked(const wxCheckBox * self);
void wxCheckBox_SetValue(wxCheckBox * self, bool state);
void wxCheckBox_Set3StateValue(wxCheckBox * self, wxCheckBoxState state);

// CLASS: wxChoice
wxChoice *wxChoice_new();
wxChoice *wxChoice_new2(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
bool wxChoice_Create1(wxChoice * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
int wxChoice_GetColumns(const wxChoice * self);
int wxChoice_GetCurrentSelection(const wxChoice * self);
void wxChoice_SetColumns(wxChoice * self, int n);
bool wxChoice_IsSorted(const wxChoice * self);
// Mix-in(s) to wxChoice
wxItemContainer *wxChoice_AsItemContainer(wxChoice* obj);

// CLASS: wxColour
wxColour *wxColour_new();
wxColour *wxColour_new2(const wxString * colour_name);
wxColour *wxColour_new4(const wxColour * colour);
unsigned int wxColour_GetAlpha(const wxColour * self);
unsigned int wxColour_GetBlue(const wxColour * self);
unsigned int wxColour_GetGreen(const wxColour * self);
unsigned int wxColour_GetRed(const wxColour * self);
wxString *wxColour_GetAsString(const wxColour * self, long flags);
#if wxCHECK_VERSION(3, 1, 0)
double wxColour_GetLuminance(const wxColour * self);
#endif
bool wxColour_IsOk(const wxColour * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxColour_IsSolid(const wxColour * self);
#endif
bool wxColour_Set2(wxColour * self, const wxString * str);
void wxColour_MakeMono(unsigned char * r, unsigned char * g, unsigned char * b, bool on);
void wxColour_MakeGrey(unsigned char * r, unsigned char * g, unsigned char * b);
void wxColour_MakeGrey1(unsigned char * r, unsigned char * g, unsigned char * b, double weight_r, double weight_g, double weight_b);
void wxColour_ChangeLightness1(unsigned char * r, unsigned char * g, unsigned char * b, int ialpha);

// CLASS: wxColourPickerCtrl
wxColourPickerCtrl *wxColourPickerCtrl_new();
wxColourPickerCtrl *wxColourPickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxColour * colour, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxColourPickerCtrl_Create(wxColourPickerCtrl * self, wxWindow * parent, wxWindowID id, const wxColour * colour, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
wxColour *wxColourPickerCtrl_GetColour(const wxColourPickerCtrl * self);
void wxColourPickerCtrl_SetColour(wxColourPickerCtrl * self, const wxColour * col);

// CLASS: wxComboBox
wxComboBox *wxComboBox_new();
wxComboBox *wxComboBox_new2(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
bool wxComboBox_Create1(wxComboBox * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
int wxComboBox_GetCurrentSelection(const wxComboBox * self);
bool wxComboBox_IsListEmpty(const wxComboBox * self);
bool wxComboBox_IsTextEmpty(const wxComboBox * self);
void wxComboBox_Popup(wxComboBox * self);
void wxComboBox_Dismiss(wxComboBox * self);
// Mix-in(s) to wxComboBox
wxItemContainer *wxComboBox_AsItemContainer(wxComboBox* obj);
wxTextEntry *wxComboBox_AsTextEntry(wxComboBox* obj);

// CLASS: wxCommandEvent
void * wxCommandEvent_GetClientData(const wxCommandEvent * self);
wxClientData * wxCommandEvent_GetClientObject(const wxCommandEvent * self);
long wxCommandEvent_GetExtraLong(const wxCommandEvent * self);
int wxCommandEvent_GetInt(const wxCommandEvent * self);
int wxCommandEvent_GetSelection(const wxCommandEvent * self);
wxString *wxCommandEvent_GetString(const wxCommandEvent * self);
bool wxCommandEvent_IsChecked(const wxCommandEvent * self);
bool wxCommandEvent_IsSelection(const wxCommandEvent * self);
void wxCommandEvent_SetClientData(wxCommandEvent * self, void * client_data);
void wxCommandEvent_SetClientObject(wxCommandEvent * self, wxClientData * client_object);
void wxCommandEvent_SetExtraLong(wxCommandEvent * self, long extra_long);
void wxCommandEvent_SetInt(wxCommandEvent * self, int int_command);
void wxCommandEvent_SetString(wxCommandEvent * self, const wxString * string);

// CLASS: wxControl
wxControl *wxControl_new(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
wxControl *wxControl_new1();
bool wxControl_Create(wxControl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxControl_Command(wxControl * self, wxCommandEvent * event);
wxString *wxControl_GetLabelText(const wxControl * self);
wxSize *wxControl_GetSizeFromTextSize(const wxControl * self, int xlen, int ylen);
wxSize *wxControl_GetSizeFromTextSize1(const wxControl * self, const wxSize * tsize);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxControl_GetSizeFromText(const wxControl * self, const wxString * text);
#endif
void wxControl_SetLabelText(wxControl * self, const wxString * text);
bool wxControl_SetLabelMarkup(wxControl * self, const wxString * markup);
wxString *wxControl_GetLabelText1(const wxString * label);
wxString *wxControl_RemoveMnemonics(const wxString * str);
wxString *wxControl_EscapeMnemonics(const wxString * text);
wxString *wxControl_Ellipsize(const wxString * label, const wxDC * dc, wxEllipsizeMode mode, int max_width, int flags);

// CLASS: wxDatePickerCtrl
wxDatePickerCtrl *wxDatePickerCtrl_new();
wxDatePickerCtrl *wxDatePickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxDatePickerCtrl_Create(wxDatePickerCtrl * self, wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxDatePickerCtrl_GetRange(const wxDatePickerCtrl * self, wxDateTime * dt1, wxDateTime * dt2);
wxDateTime *wxDatePickerCtrl_GetValue(const wxDatePickerCtrl * self);
#if wxCHECK_VERSION(3, 1, 0)
void wxDatePickerCtrl_SetNullText(wxDatePickerCtrl * self, const wxString * text);
#endif
void wxDatePickerCtrl_SetRange(wxDatePickerCtrl * self, const wxDateTime * dt1, const wxDateTime * dt2);
void wxDatePickerCtrl_SetValue(wxDatePickerCtrl * self, const wxDateTime * dt);

// CLASS: wxDirPickerCtrl
wxDirPickerCtrl *wxDirPickerCtrl_new();
wxDirPickerCtrl *wxDirPickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxDirPickerCtrl_Create(wxDirPickerCtrl * self, wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
wxFileName *wxDirPickerCtrl_GetDirName(const wxDirPickerCtrl * self);
wxString *wxDirPickerCtrl_GetPath(const wxDirPickerCtrl * self);
void wxDirPickerCtrl_SetDirName(wxDirPickerCtrl * self, const wxFileName * dirname);
void wxDirPickerCtrl_SetInitialDirectory(wxDirPickerCtrl * self, const wxString * dir);
void wxDirPickerCtrl_SetPath(wxDirPickerCtrl * self, const wxString * dirname);

// CLASS: wxFrame
wxFrame *wxFrame_new();
wxFrame *wxFrame_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxFrame_Centre(wxFrame * self, int direction);
bool wxFrame_Create(wxFrame * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxStatusBar * wxFrame_CreateStatusBar(wxFrame * self, int number, long style, wxWindowID id, const wxString * name);
wxToolBar * wxFrame_CreateToolBar(wxFrame * self, long style, wxWindowID id, const wxString * name);
void wxFrame_DoGiveHelp(wxFrame * self, const wxString * text, bool show);
wxMenuBar * wxFrame_GetMenuBar(const wxFrame * self);
wxStatusBar * wxFrame_GetStatusBar(const wxFrame * self);
int wxFrame_GetStatusBarPane(const wxFrame * self);
wxToolBar * wxFrame_GetToolBar(const wxFrame * self);
wxStatusBar * wxFrame_OnCreateStatusBar(wxFrame * self, int number, long style, wxWindowID id, const wxString * name);
wxToolBar * wxFrame_OnCreateToolBar(wxFrame * self, long style, wxWindowID id, const wxString * name);
bool wxFrame_ProcessCommand(wxFrame * self, int id);
void wxFrame_SetMenuBar(wxFrame * self, wxMenuBar * menu_bar);
void wxFrame_SetStatusBar(wxFrame * self, wxStatusBar * status_bar);
void wxFrame_SetStatusBarPane(wxFrame * self, int n);
void wxFrame_SetStatusText(wxFrame * self, const wxString * text, int number);
void wxFrame_SetStatusWidths(wxFrame * self, int n, const int * widths_field);
void wxFrame_SetToolBar(wxFrame * self, wxToolBar * tool_bar);
#ifdef __WXMSW__
wxTaskBarButton * wxFrame_MSWGetTaskBarButton(wxFrame * self);
#endif
void wxFrame_PushStatusText(wxFrame * self, const wxString * text, int number);
void wxFrame_PopStatusText(wxFrame * self, int number);

// CLASS: wxGDIObject

// CLASS: wxIcon
wxIcon *wxIcon_new();
wxIcon *wxIcon_new1(const wxIcon * icon);
wxIcon *wxIcon_new3(const char *const * bits);
wxIcon *wxIcon_new5(const wxIconLocation * loc);
void wxIcon_CopyFromBitmap(wxIcon * self, const wxBitmap * bmp);
int wxIcon_GetDepth(const wxIcon * self);
int wxIcon_GetHeight(const wxIcon * self);
double wxIcon_GetLogicalHeight(const wxIcon * self);
wxSize *wxIcon_GetLogicalSize(const wxIcon * self);
double wxIcon_GetLogicalWidth(const wxIcon * self);
double wxIcon_GetScaleFactor(const wxIcon * self);
wxSize *wxIcon_GetSize(const wxIcon * self);
int wxIcon_GetWidth(const wxIcon * self);
bool wxIcon_IsOk(const wxIcon * self);
#if wxCHECK_VERSION(3, 1, 7)
void wxIcon_SetDepth(wxIcon * self, int depth);
void wxIcon_SetHeight(wxIcon * self, int height);
void wxIcon_SetWidth(wxIcon * self, int width);
#endif

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
void wxItemContainerImmutable_SetString(wxItemContainerImmutable * self, unsigned int n, const wxString * string);
int wxItemContainerImmutable_FindString(const wxItemContainerImmutable * self, const wxString * string, bool case_sensitive);

// CLASS: wxListBox
wxListBox *wxListBox_new();
wxListBox *wxListBox_new2(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
bool wxListBox_Create1(wxListBox * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
void wxListBox_Deselect(wxListBox * self, int n);
bool wxListBox_SetStringSelection(wxListBox * self, const wxString * s, bool select);
bool wxListBox_SetStringSelection1(wxListBox * self, const wxString * s);
int wxListBox_GetSelections(const wxListBox * self, wxArrayInt * selections);
int wxListBox_HitTest(const wxListBox * self, const wxPoint * point);
int wxListBox_HitTest1(const wxListBox * self, int x, int y);
void wxListBox_InsertItems1(wxListBox * self, const wxArrayString * items, unsigned int pos);
bool wxListBox_IsSelected(const wxListBox * self, int n);
void wxListBox_SetFirstItem(wxListBox * self, int n);
void wxListBox_SetFirstItem1(wxListBox * self, const wxString * string);
void wxListBox_EnsureVisible(wxListBox * self, int n);
bool wxListBox_IsSorted(const wxListBox * self);
#if wxCHECK_VERSION(3, 1, 0)
int wxListBox_GetCountPerPage(const wxListBox * self);
int wxListBox_GetTopItem(const wxListBox * self);
#endif
// Mix-in(s) to wxListBox
wxItemContainer *wxListBox_AsItemContainer(wxListBox* obj);

// CLASS: wxMenu
wxMenu *wxMenu_new();
wxMenu *wxMenu_new1(long style);
wxMenu *wxMenu_new2(const wxString * title, long style);
wxMenuItem * wxMenu_Append(wxMenu * self, int id, const wxString * item, const wxString * help_string, wxItemKind kind);
wxMenuItem * wxMenu_Append1(wxMenu * self, int id, const wxString * item, wxMenu * sub_menu, const wxString * help_string);
wxMenuItem * wxMenu_Append2(wxMenu * self, wxMenuItem * menu_item);
wxMenuItem * wxMenu_AppendCheckItem(wxMenu * self, int id, const wxString * item, const wxString * help);
wxMenuItem * wxMenu_AppendRadioItem(wxMenu * self, int id, const wxString * item, const wxString * help);
wxMenuItem * wxMenu_AppendSeparator(wxMenu * self);
wxMenuItem * wxMenu_AppendSubMenu(wxMenu * self, wxMenu * submenu, const wxString * text, const wxString * help);
void wxMenu_Break(wxMenu * self);
void wxMenu_Check(wxMenu * self, int id, bool check);
bool wxMenu_Delete(wxMenu * self, int id);
bool wxMenu_Delete1(wxMenu * self, wxMenuItem * item);
bool wxMenu_Destroy(wxMenu * self, int id);
bool wxMenu_Destroy1(wxMenu * self, wxMenuItem * item);
void wxMenu_Enable(wxMenu * self, int id, bool enable);
wxMenuItem * wxMenu_FindChildItem(const wxMenu * self, int id, size_t * pos);
int wxMenu_FindItem(const wxMenu * self, const wxString * item_string);
wxMenuItem * wxMenu_FindItem1(const wxMenu * self, int id, wxMenu ** menu);
wxMenuItem * wxMenu_FindItemByPosition(const wxMenu * self, size_t position);
wxString *wxMenu_GetHelpString(const wxMenu * self, int id);
wxString *wxMenu_GetLabel(const wxMenu * self, int id);
wxString *wxMenu_GetLabelText(const wxMenu * self, int id);
size_t wxMenu_GetMenuItemCount(const wxMenu * self);
wxString *wxMenu_GetTitle(const wxMenu * self);
wxMenuItem * wxMenu_Insert(wxMenu * self, size_t pos, wxMenuItem * menu_item);
wxMenuItem * wxMenu_Insert1(wxMenu * self, size_t pos, int id, const wxString * item, const wxString * help_string, wxItemKind kind);
wxMenuItem * wxMenu_Insert2(wxMenu * self, size_t pos, int id, const wxString * text, wxMenu * submenu, const wxString * help);
wxMenuItem * wxMenu_InsertCheckItem(wxMenu * self, size_t pos, int id, const wxString * item, const wxString * help_string);
wxMenuItem * wxMenu_InsertRadioItem(wxMenu * self, size_t pos, int id, const wxString * item, const wxString * help_string);
wxMenuItem * wxMenu_InsertSeparator(wxMenu * self, size_t pos);
bool wxMenu_IsChecked(const wxMenu * self, int id);
bool wxMenu_IsEnabled(const wxMenu * self, int id);
wxMenuItem * wxMenu_Prepend(wxMenu * self, wxMenuItem * item);
wxMenuItem * wxMenu_Prepend1(wxMenu * self, int id, const wxString * item, const wxString * help_string, wxItemKind kind);
wxMenuItem * wxMenu_Prepend2(wxMenu * self, int id, const wxString * text, wxMenu * submenu, const wxString * help);
wxMenuItem * wxMenu_PrependCheckItem(wxMenu * self, int id, const wxString * item, const wxString * help_string);
wxMenuItem * wxMenu_PrependRadioItem(wxMenu * self, int id, const wxString * item, const wxString * help_string);
wxMenuItem * wxMenu_PrependSeparator(wxMenu * self);
wxMenuItem * wxMenu_Remove(wxMenu * self, int id);
wxMenuItem * wxMenu_Remove1(wxMenu * self, wxMenuItem * item);
void wxMenu_SetHelpString(wxMenu * self, int id, const wxString * help_string);
void wxMenu_SetLabel(wxMenu * self, int id, const wxString * label);
void wxMenu_SetTitle(wxMenu * self, const wxString * title);
void wxMenu_UpdateUI(wxMenu * self, wxEvtHandler * source);
void wxMenu_SetInvokingWindow(wxMenu * self, wxWindow * win);
wxWindow * wxMenu_GetInvokingWindow(const wxMenu * self);
wxWindow * wxMenu_GetWindow(const wxMenu * self);
long wxMenu_GetStyle(const wxMenu * self);
void wxMenu_SetParent(wxMenu * self, wxMenu * parent);
wxMenu * wxMenu_GetParent(const wxMenu * self);
void wxMenu_Attach(wxMenu * self, wxMenuBar * menubar);
void wxMenu_Detach(wxMenu * self);
bool wxMenu_IsAttached(const wxMenu * self);

// CLASS: wxMenuBar
wxMenuBar *wxMenuBar_new(long style);
bool wxMenuBar_Append(wxMenuBar * self, wxMenu * menu, const wxString * title);
void wxMenuBar_Check(wxMenuBar * self, int id, bool check);
void wxMenuBar_Enable(wxMenuBar * self, int id, bool enable);
bool wxMenuBar_IsEnabledTop(const wxMenuBar * self, size_t pos);
void wxMenuBar_EnableTop(wxMenuBar * self, size_t pos, bool enable);
wxMenuItem * wxMenuBar_FindItem(const wxMenuBar * self, int id, wxMenu ** menu);
int wxMenuBar_FindMenu(const wxMenuBar * self, const wxString * title);
int wxMenuBar_FindMenuItem(const wxMenuBar * self, const wxString * menu_string, const wxString * item_string);
wxString *wxMenuBar_GetHelpString(const wxMenuBar * self, int id);
wxString *wxMenuBar_GetLabel(const wxMenuBar * self, int id);
wxMenu * wxMenuBar_GetMenu(const wxMenuBar * self, size_t menu_index);
size_t wxMenuBar_GetMenuCount(const wxMenuBar * self);
wxString *wxMenuBar_GetMenuLabel(const wxMenuBar * self, size_t pos);
wxString *wxMenuBar_GetMenuLabelText(const wxMenuBar * self, size_t pos);
bool wxMenuBar_Insert(wxMenuBar * self, size_t pos, wxMenu * menu, const wxString * title);
bool wxMenuBar_IsChecked(const wxMenuBar * self, int id);
bool wxMenuBar_IsEnabled(const wxMenuBar * self, int id);
wxMenu * wxMenuBar_Remove(wxMenuBar * self, size_t pos);
wxMenu * wxMenuBar_Replace(wxMenuBar * self, size_t pos, wxMenu * menu, const wxString * title);
void wxMenuBar_SetHelpString(wxMenuBar * self, int id, const wxString * help_string);
void wxMenuBar_SetLabel(wxMenuBar * self, int id, const wxString * label);
void wxMenuBar_SetMenuLabel(wxMenuBar * self, size_t pos, const wxString * label);
#ifdef __WXOSX__
wxMenu * wxMenuBar_OSXGetAppleMenu(const wxMenuBar * self);
#endif
wxFrame * wxMenuBar_GetFrame(const wxMenuBar * self);
bool wxMenuBar_IsAttached(const wxMenuBar * self);
void wxMenuBar_Attach(wxMenuBar * self, wxFrame * frame);
void wxMenuBar_Detach(wxMenuBar * self);
#ifdef __WXOSX__
void wxMenuBar_MacSetCommonMenuBar(wxMenuBar * menubar);
wxMenuBar * wxMenuBar_MacGetCommonMenuBar();
#endif

// CLASS: wxMenuItem
wxBitmap *wxMenuItem_GetBitmap(const wxMenuItem * self);
#ifdef __WXMSW__
wxBitmap *wxMenuItem_GetBitmap1(const wxMenuItem * self, bool checked);
#endif
wxBitmapBundle *wxMenuItem_GetBitmapBundle(const wxMenuItem * self);
#ifdef __WXMSW__
wxBitmap *wxMenuItem_GetDisabledBitmap(const wxMenuItem * self);
#endif
wxString *wxMenuItem_GetHelp(const wxMenuItem * self);
int wxMenuItem_GetId(const wxMenuItem * self);
wxString *wxMenuItem_GetItemLabel(const wxMenuItem * self);
wxString *wxMenuItem_GetItemLabelText(const wxMenuItem * self);
wxItemKind wxMenuItem_GetKind(const wxMenuItem * self);
#ifdef __WXMSW__
int wxMenuItem_GetMarginWidth(const wxMenuItem * self);
#endif
wxMenu * wxMenuItem_GetMenu(const wxMenuItem * self);
wxMenu * wxMenuItem_GetSubMenu(const wxMenuItem * self);
wxAcceleratorEntry * wxMenuItem_GetAccel(const wxMenuItem * self);
bool wxMenuItem_IsCheck(const wxMenuItem * self);
bool wxMenuItem_IsCheckable(const wxMenuItem * self);
bool wxMenuItem_IsChecked(const wxMenuItem * self);
bool wxMenuItem_IsEnabled(const wxMenuItem * self);
bool wxMenuItem_IsRadio(const wxMenuItem * self);
bool wxMenuItem_IsSeparator(const wxMenuItem * self);
bool wxMenuItem_IsSubMenu(const wxMenuItem * self);
#ifdef __WXMSW__
void wxMenuItem_SetBackgroundColour(wxMenuItem * self, const wxColour * colour);
#endif
void wxMenuItem_SetBitmap(wxMenuItem * self, const wxBitmapBundle * bmp);
#ifdef __WXMSW__
void wxMenuItem_SetBitmap1(wxMenuItem * self, const wxBitmapBundle * bmp, bool checked);
void wxMenuItem_SetBitmaps(wxMenuItem * self, const wxBitmapBundle * checked, const wxBitmapBundle * unchecked);
void wxMenuItem_SetDisabledBitmap(wxMenuItem * self, const wxBitmapBundle * disabled);
void wxMenuItem_SetFont(wxMenuItem * self, const wxFont * font);
#endif
void wxMenuItem_SetHelp(wxMenuItem * self, const wxString * help_string);
void wxMenuItem_SetItemLabel(wxMenuItem * self, const wxString * label);
#ifdef __WXMSW__
void wxMenuItem_SetMarginWidth(wxMenuItem * self, int width);
#endif
void wxMenuItem_SetMenu(wxMenuItem * self, wxMenu * menu);
void wxMenuItem_SetSubMenu(wxMenuItem * self, wxMenu * menu);
#ifdef __WXMSW__
void wxMenuItem_SetTextColour(wxMenuItem * self, const wxColour * colour);
#endif
void wxMenuItem_SetAccel(wxMenuItem * self, wxAcceleratorEntry * accel);
void wxMenuItem_AddExtraAccel(wxMenuItem * self, const wxAcceleratorEntry * accel);
void wxMenuItem_ClearExtraAccels(wxMenuItem * self);
wxMenuItem *wxMenuItem_new(wxMenu * parent_menu, int id, const wxString * text, const wxString * help_string, wxItemKind kind, wxMenu * sub_menu);
void wxMenuItem_Check(wxMenuItem * self, bool check);
void wxMenuItem_Enable(wxMenuItem * self, bool enable);
wxString *wxMenuItem_GetLabelText(const wxString * text);

// CLASS: wxNonOwnedWindow
bool wxNonOwnedWindow_SetShape(wxNonOwnedWindow * self, const wxRegion * region);
bool wxNonOwnedWindow_SetShape1(wxNonOwnedWindow * self, const wxGraphicsPath * path);

// CLASS: wxNotebook
wxNotebook *wxNotebook_new();
wxNotebook *wxNotebook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
int wxNotebook_GetRowCount(const wxNotebook * self);
wxColour *wxNotebook_GetThemeBackgroundColour(const wxNotebook * self);
void wxNotebook_SetPadding(wxNotebook * self, const wxSize * padding);

// CLASS: wxNotifyEvent
void wxNotifyEvent_Allow(wxNotifyEvent * self);
bool wxNotifyEvent_IsAllowed(const wxNotifyEvent * self);
void wxNotifyEvent_Veto(wxNotifyEvent * self);

// CLASS: wxPanel
wxPanel *wxPanel_new();
wxPanel *wxPanel_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxPanel_Create(wxPanel * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxPanel_OnSysColourChanged(wxPanel * self, wxSysColourChangedEvent * event);
void wxPanel_SetFocusIgnoringChildren(wxPanel * self);

// CLASS: wxPickerBase
bool wxPickerBase_CreateBase(wxPickerBase * self, wxWindow * parent, wxWindowID id, const wxString * text, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
int wxPickerBase_GetInternalMargin(const wxPickerBase * self);
int wxPickerBase_GetPickerCtrlProportion(const wxPickerBase * self);
wxTextCtrl * wxPickerBase_GetTextCtrl(wxPickerBase * self);
wxControl * wxPickerBase_GetPickerCtrl(wxPickerBase * self);
int wxPickerBase_GetTextCtrlProportion(const wxPickerBase * self);
bool wxPickerBase_HasTextCtrl(const wxPickerBase * self);
bool wxPickerBase_IsPickerCtrlGrowable(const wxPickerBase * self);
bool wxPickerBase_IsTextCtrlGrowable(const wxPickerBase * self);
void wxPickerBase_SetInternalMargin(wxPickerBase * self, int margin);
void wxPickerBase_SetPickerCtrlGrowable(wxPickerBase * self, bool grow);
void wxPickerBase_SetPickerCtrlProportion(wxPickerBase * self, int prop);
void wxPickerBase_SetTextCtrlGrowable(wxPickerBase * self, bool grow);
void wxPickerBase_SetTextCtrlProportion(wxPickerBase * self, int prop);
void wxPickerBase_SetTextCtrl(wxPickerBase * self, wxTextCtrl * text);
void wxPickerBase_SetPickerCtrl(wxPickerBase * self, wxControl * picker);
void wxPickerBase_UpdatePickerFromTextCtrl(wxPickerBase * self);
void wxPickerBase_UpdateTextCtrlFromPicker(wxPickerBase * self);

// CLASS: wxPoint
void wxPoint_delete(wxPoint *self);
bool wxPoint_IsFullySpecified(const wxPoint * self);
void wxPoint_SetDefaults(wxPoint * self, const wxPoint * pt);
wxPoint *wxPoint_new();
wxPoint *wxPoint_new1(int x, int y);
wxPoint *wxPoint_new2(const wxRealPoint * pt);

// CLASS: wxRadioBox
wxRadioBox *wxRadioBox_new();
wxRadioBox *wxRadioBox_new2(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, int major_dimension, long style, const wxValidator * validator, const wxString * name);
bool wxRadioBox_Create1(wxRadioBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, int major_dimension, long style, const wxValidator * validator, const wxString * name);
bool wxRadioBox_Enable(wxRadioBox * self, unsigned int n, bool enable);
unsigned int wxRadioBox_GetColumnCount(const wxRadioBox * self);
int wxRadioBox_GetItemFromPoint(const wxRadioBox * self, const wxPoint * pt);
#if wxCHECK_VERSION(3, 1, 7)
wxString *wxRadioBox_GetItemHelpText(const wxRadioBox * self, unsigned int item);
#endif
wxToolTip * wxRadioBox_GetItemToolTip(const wxRadioBox * self, unsigned int item);
unsigned int wxRadioBox_GetRowCount(const wxRadioBox * self);
bool wxRadioBox_IsItemEnabled(const wxRadioBox * self, unsigned int n);
bool wxRadioBox_IsItemShown(const wxRadioBox * self, unsigned int n);
void wxRadioBox_SetItemHelpText(wxRadioBox * self, unsigned int item, const wxString * helptext);
void wxRadioBox_SetItemToolTip(wxRadioBox * self, unsigned int item, const wxString * text);
bool wxRadioBox_Show(wxRadioBox * self, unsigned int item, bool show);
// Mix-in(s) to wxRadioBox
wxItemContainerImmutable *wxRadioBox_AsItemContainerImmutable(wxRadioBox* obj);

// CLASS: wxRect
void wxRect_delete(wxRect *self);
wxRect *wxRect_new();
wxRect *wxRect_new1(int x, int y, int width, int height);
wxRect *wxRect_new2(const wxPoint * top_left, const wxPoint * bottom_right);
wxRect *wxRect_new3(const wxPoint * pos, const wxSize * size);
wxRect *wxRect_new4(const wxSize * size);
wxRect *wxRect_CentreIn(const wxRect * self, const wxRect * r, int dir);
wxRect *wxRect_CenterIn(const wxRect * self, const wxRect * r, int dir);
bool wxRect_Contains(const wxRect * self, int x, int y);
bool wxRect_Contains1(const wxRect * self, const wxPoint * pt);
bool wxRect_Contains2(const wxRect * self, const wxRect * rect);
wxRect *wxRect_Deflate3(const wxRect * self, wxCoord dx, wxCoord dy);
int wxRect_GetBottom(const wxRect * self);
wxPoint *wxRect_GetBottomLeft(const wxRect * self);
wxPoint *wxRect_GetBottomRight(const wxRect * self);
int wxRect_GetHeight(const wxRect * self);
int wxRect_GetLeft(const wxRect * self);
wxPoint *wxRect_GetPosition(const wxRect * self);
int wxRect_GetRight(const wxRect * self);
wxSize *wxRect_GetSize(const wxRect * self);
int wxRect_GetTop(const wxRect * self);
wxPoint *wxRect_GetTopLeft(const wxRect * self);
wxPoint *wxRect_GetTopRight(const wxRect * self);
int wxRect_GetWidth(const wxRect * self);
int wxRect_GetX(const wxRect * self);
int wxRect_GetY(const wxRect * self);
wxRect *wxRect_Inflate3(const wxRect * self, wxCoord dx, wxCoord dy);
wxRect *wxRect_Intersect1(const wxRect * self, const wxRect * rect);
bool wxRect_Intersects(const wxRect * self, const wxRect * rect);
bool wxRect_IsEmpty(const wxRect * self);
void wxRect_Offset(wxRect * self, wxCoord dx, wxCoord dy);
void wxRect_Offset1(wxRect * self, const wxPoint * pt);
void wxRect_SetHeight(wxRect * self, int height);
void wxRect_SetPosition(wxRect * self, const wxPoint * pos);
void wxRect_SetSize(wxRect * self, const wxSize * s);
void wxRect_SetWidth(wxRect * self, int width);
void wxRect_SetX(wxRect * self, int x);
void wxRect_SetY(wxRect * self, int y);
void wxRect_SetLeft(wxRect * self, int left);
void wxRect_SetRight(wxRect * self, int right);
void wxRect_SetTop(wxRect * self, int top);
void wxRect_SetBottom(wxRect * self, int bottom);
void wxRect_SetTopLeft(wxRect * self, const wxPoint * p);
void wxRect_SetBottomRight(wxRect * self, const wxPoint * p);
void wxRect_SetTopRight(wxRect * self, const wxPoint * p);
void wxRect_SetBottomLeft(wxRect * self, const wxPoint * p);
wxRect *wxRect_Union(const wxRect * self, const wxRect * rect);

// CLASS: wxSize
void wxSize_delete(wxSize *self);
wxSize *wxSize_new();
wxSize *wxSize_new1(int width, int height);
void wxSize_DecBy(wxSize * self, const wxPoint * pt);
void wxSize_DecBy1(wxSize * self, const wxSize * size);
void wxSize_DecBy2(wxSize * self, int dx, int dy);
void wxSize_DecBy3(wxSize * self, int d);
void wxSize_DecTo(wxSize * self, const wxSize * size);
void wxSize_DecToIfSpecified(wxSize * self, const wxSize * size);
int wxSize_GetHeight(const wxSize * self);
int wxSize_GetWidth(const wxSize * self);
void wxSize_IncBy(wxSize * self, const wxPoint * pt);
void wxSize_IncBy1(wxSize * self, const wxSize * size);
void wxSize_IncBy2(wxSize * self, int dx, int dy);
void wxSize_IncBy3(wxSize * self, int d);
void wxSize_IncTo(wxSize * self, const wxSize * size);
bool wxSize_IsFullySpecified(const wxSize * self);
void wxSize_Set(wxSize * self, int width, int height);
void wxSize_SetDefaults(wxSize * self, const wxSize * size_default);
void wxSize_SetHeight(wxSize * self, int height);
void wxSize_SetWidth(wxSize * self, int width);

// CLASS: wxSizer
wxSizerItem * wxSizer_Add(wxSizer * self, wxWindow * window, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Add1(wxSizer * self, wxWindow * window, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Add2(wxSizer * self, wxSizer * sizer, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Add3(wxSizer * self, wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Add4(wxSizer * self, int width, int height, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Add5(wxSizer * self, int width, int height, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Add6(wxSizer * self, wxSizerItem * item);
wxSizerItem * wxSizer_AddSpacer(wxSizer * self, int size);
wxSizerItem * wxSizer_AddStretchSpacer(wxSizer * self, int prop);
wxSize *wxSizer_CalcMin(wxSizer * self);
void wxSizer_Clear(wxSizer * self, bool delete_windows);
wxSize *wxSizer_ComputeFittingClientSize(wxSizer * self, wxWindow * window);
wxSize *wxSizer_ComputeFittingWindowSize(wxSizer * self, wxWindow * window);
bool wxSizer_Detach(wxSizer * self, wxWindow * window);
bool wxSizer_Detach1(wxSizer * self, wxSizer * sizer);
bool wxSizer_Detach2(wxSizer * self, int index);
wxSize *wxSizer_Fit(wxSizer * self, wxWindow * window);
void wxSizer_FitInside(wxSizer * self, wxWindow * window);
bool wxSizer_InformFirstDirection(wxSizer * self, int direction, int size, int available_other_dir);
wxSizerItemList * wxSizer_GetChildren(wxSizer * self);
wxWindow * wxSizer_GetContainingWindow(const wxSizer * self);
void wxSizer_SetContainingWindow(wxSizer * self, wxWindow * window);
size_t wxSizer_GetItemCount(const wxSizer * self);
wxSizerItem * wxSizer_GetItem(wxSizer * self, wxWindow * window, bool recursive);
wxSizerItem * wxSizer_GetItem1(wxSizer * self, wxSizer * sizer, bool recursive);
wxSizerItem * wxSizer_GetItem2(wxSizer * self, size_t index);
wxSizerItem * wxSizer_GetItemById(wxSizer * self, int id, bool recursive);
wxSize *wxSizer_GetMinSize(wxSizer * self);
wxPoint *wxSizer_GetPosition(const wxSizer * self);
wxSize *wxSizer_GetSize(const wxSizer * self);
bool wxSizer_Hide(wxSizer * self, wxWindow * window, bool recursive);
bool wxSizer_Hide1(wxSizer * self, wxSizer * sizer, bool recursive);
bool wxSizer_Hide2(wxSizer * self, size_t index);
wxSizerItem * wxSizer_Insert(wxSizer * self, size_t index, wxWindow * window, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Insert1(wxSizer * self, size_t index, wxWindow * window, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Insert2(wxSizer * self, size_t index, wxSizer * sizer, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Insert3(wxSizer * self, size_t index, wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Insert4(wxSizer * self, size_t index, int width, int height, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Insert5(wxSizer * self, size_t index, int width, int height, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Insert6(wxSizer * self, size_t index, wxSizerItem * item);
wxSizerItem * wxSizer_InsertSpacer(wxSizer * self, size_t index, int size);
wxSizerItem * wxSizer_InsertStretchSpacer(wxSizer * self, size_t index, int prop);
bool wxSizer_IsEmpty(const wxSizer * self);
bool wxSizer_IsShown(const wxSizer * self, wxWindow * window);
bool wxSizer_IsShown1(const wxSizer * self, wxSizer * sizer);
bool wxSizer_IsShown2(const wxSizer * self, size_t index);
void wxSizer_Layout(wxSizer * self);
wxSizerItem * wxSizer_Prepend(wxSizer * self, wxWindow * window, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Prepend1(wxSizer * self, wxWindow * window, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Prepend2(wxSizer * self, wxSizer * sizer, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Prepend3(wxSizer * self, wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Prepend4(wxSizer * self, int width, int height, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Prepend5(wxSizer * self, int width, int height, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Prepend6(wxSizer * self, wxSizerItem * item);
wxSizerItem * wxSizer_PrependSpacer(wxSizer * self, int size);
wxSizerItem * wxSizer_PrependStretchSpacer(wxSizer * self, int prop);
#if wxCHECK_VERSION(3, 1, 0)
void wxSizer_RepositionChildren(wxSizer * self, const wxSize * min_size);
#endif
bool wxSizer_Remove1(wxSizer * self, wxSizer * sizer);
bool wxSizer_Remove2(wxSizer * self, int index);
bool wxSizer_Replace(wxSizer * self, wxWindow * oldwin, wxWindow * newwin, bool recursive);
bool wxSizer_Replace1(wxSizer * self, wxSizer * oldsz, wxSizer * newsz, bool recursive);
bool wxSizer_Replace2(wxSizer * self, size_t index, wxSizerItem * newitem);
void wxSizer_SetDimension(wxSizer * self, int x, int y, int width, int height);
void wxSizer_SetDimension1(wxSizer * self, const wxPoint * pos, const wxSize * size);
bool wxSizer_SetItemMinSize(wxSizer * self, wxWindow * window, int width, int height);
bool wxSizer_SetItemMinSize1(wxSizer * self, wxWindow * window, const wxSize * size);
bool wxSizer_SetItemMinSize2(wxSizer * self, wxSizer * sizer, int width, int height);
bool wxSizer_SetItemMinSize3(wxSizer * self, wxSizer * sizer, const wxSize * size);
bool wxSizer_SetItemMinSize4(wxSizer * self, size_t index, int width, int height);
bool wxSizer_SetItemMinSize5(wxSizer * self, size_t index, const wxSize * size);
void wxSizer_SetMinSize(wxSizer * self, const wxSize * size);
void wxSizer_SetMinSize1(wxSizer * self, int width, int height);
void wxSizer_SetSizeHints(wxSizer * self, wxWindow * window);
bool wxSizer_Show(wxSizer * self, wxWindow * window, bool show, bool recursive);
bool wxSizer_Show1(wxSizer * self, wxSizer * sizer, bool show, bool recursive);
bool wxSizer_Show2(wxSizer * self, size_t index, bool show);
void wxSizer_ShowItems(wxSizer * self, bool show);

// CLASS: wxSizerFlags
void wxSizerFlags_delete(wxSizerFlags *self);
wxSizerFlags *wxSizerFlags_new(int proportion);
wxSizerFlags * wxSizerFlags_Align(wxSizerFlags * self, int alignment);
wxSizerFlags * wxSizerFlags_Border(wxSizerFlags * self, int direction, int borderinpixels);
wxSizerFlags * wxSizerFlags_Border1(wxSizerFlags * self, int direction);
wxSizerFlags * wxSizerFlags_Bottom(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Center(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Centre(wxSizerFlags * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSizerFlags * wxSizerFlags_CenterHorizontal(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_CenterVertical(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_CentreHorizontal(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_CentreVertical(wxSizerFlags * self);
#endif
wxSizerFlags * wxSizerFlags_DoubleBorder(wxSizerFlags * self, int direction);
wxSizerFlags * wxSizerFlags_DoubleHorzBorder(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Expand(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_FixedMinSize(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_ReserveSpaceEvenIfHidden(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Left(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Proportion(wxSizerFlags * self, int proportion);
wxSizerFlags * wxSizerFlags_Right(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Shaped(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Top(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_TripleBorder(wxSizerFlags * self, int direction);
void wxSizerFlags_DisableConsistencyChecks();
int wxSizerFlags_GetDefaultBorder();

// CLASS: wxStaticBitmap
wxStaticBitmap *wxStaticBitmap_new();
wxStaticBitmap *wxStaticBitmap_new1(wxWindow * parent, wxWindowID id, const wxBitmapBundle * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxStaticBitmap_Create(wxStaticBitmap * self, wxWindow * parent, wxWindowID id, const wxBitmapBundle * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxBitmap *wxStaticBitmap_GetBitmap(const wxStaticBitmap * self);
wxIcon *wxStaticBitmap_GetIcon(const wxStaticBitmap * self);
void wxStaticBitmap_SetBitmap(wxStaticBitmap * self, const wxBitmapBundle * label);
void wxStaticBitmap_SetIcon(wxStaticBitmap * self, const wxIcon * label);

// CLASS: wxStaticBox
wxStaticBox *wxStaticBox_new();
wxStaticBox *wxStaticBox_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxStaticBox_Create(wxStaticBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);

// CLASS: wxStaticBoxSizer
wxStaticBoxSizer *wxStaticBoxSizer_new(wxStaticBox * box_, int orient);
wxStaticBoxSizer *wxStaticBoxSizer_new1(int orient, wxWindow * parent, const wxString * label);
wxStaticBox * wxStaticBoxSizer_GetStaticBox(const wxStaticBoxSizer * self);

// CLASS: wxStaticText
wxStaticText *wxStaticText_new();
wxStaticText *wxStaticText_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxStaticText_Create(wxStaticText * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxStaticText_IsEllipsized(const wxStaticText * self);
void wxStaticText_Wrap(wxStaticText * self, int width);

// CLASS: wxTextAttr
void wxTextAttr_delete(wxTextAttr *self);
wxColour *wxTextAttr_GetBackgroundColour(const wxTextAttr * self);
wxString *wxTextAttr_GetBulletFont(const wxTextAttr * self);
wxString *wxTextAttr_GetBulletName(const wxTextAttr * self);
int wxTextAttr_GetBulletNumber(const wxTextAttr * self);
int wxTextAttr_GetBulletStyle(const wxTextAttr * self);
wxString *wxTextAttr_GetBulletText(const wxTextAttr * self);
wxString *wxTextAttr_GetCharacterStyleName(const wxTextAttr * self);
long wxTextAttr_GetFlags(const wxTextAttr * self);
bool wxTextAttr_GetFontAttributes(wxTextAttr * self, const wxFont * font, int flags);
wxString *wxTextAttr_GetFontFaceName(const wxTextAttr * self);
int wxTextAttr_GetFontSize(const wxTextAttr * self);
bool wxTextAttr_GetFontUnderlined(const wxTextAttr * self);
#if wxCHECK_VERSION(3, 1, 0)
wxColour *wxTextAttr_GetUnderlineColour(const wxTextAttr * self);
#endif
long wxTextAttr_GetLeftIndent(const wxTextAttr * self);
long wxTextAttr_GetLeftSubIndent(const wxTextAttr * self);
int wxTextAttr_GetLineSpacing(const wxTextAttr * self);
wxString *wxTextAttr_GetListStyleName(const wxTextAttr * self);
int wxTextAttr_GetOutlineLevel(const wxTextAttr * self);
int wxTextAttr_GetParagraphSpacingAfter(const wxTextAttr * self);
int wxTextAttr_GetParagraphSpacingBefore(const wxTextAttr * self);
wxString *wxTextAttr_GetParagraphStyleName(const wxTextAttr * self);
long wxTextAttr_GetRightIndent(const wxTextAttr * self);
wxColour *wxTextAttr_GetTextColour(const wxTextAttr * self);
int wxTextAttr_GetTextEffectFlags(const wxTextAttr * self);
int wxTextAttr_GetTextEffects(const wxTextAttr * self);
wxString *wxTextAttr_GetURL(const wxTextAttr * self);
bool wxTextAttr_HasAlignment(const wxTextAttr * self);
bool wxTextAttr_HasBackgroundColour(const wxTextAttr * self);
bool wxTextAttr_HasBulletName(const wxTextAttr * self);
bool wxTextAttr_HasBulletNumber(const wxTextAttr * self);
bool wxTextAttr_HasBulletStyle(const wxTextAttr * self);
bool wxTextAttr_HasBulletText(const wxTextAttr * self);
bool wxTextAttr_HasCharacterStyleName(const wxTextAttr * self);
bool wxTextAttr_HasFlag(const wxTextAttr * self, long flag);
bool wxTextAttr_HasFont(const wxTextAttr * self);
bool wxTextAttr_HasFontEncoding(const wxTextAttr * self);
bool wxTextAttr_HasFontFaceName(const wxTextAttr * self);
bool wxTextAttr_HasFontFamily(const wxTextAttr * self);
bool wxTextAttr_HasFontItalic(const wxTextAttr * self);
bool wxTextAttr_HasFontSize(const wxTextAttr * self);
bool wxTextAttr_HasFontPointSize(const wxTextAttr * self);
bool wxTextAttr_HasFontPixelSize(const wxTextAttr * self);
bool wxTextAttr_HasFontUnderlined(const wxTextAttr * self);
bool wxTextAttr_HasFontWeight(const wxTextAttr * self);
bool wxTextAttr_HasLeftIndent(const wxTextAttr * self);
bool wxTextAttr_HasLineSpacing(const wxTextAttr * self);
bool wxTextAttr_HasListStyleName(const wxTextAttr * self);
bool wxTextAttr_HasOutlineLevel(const wxTextAttr * self);
bool wxTextAttr_HasPageBreak(const wxTextAttr * self);
bool wxTextAttr_HasParagraphSpacingAfter(const wxTextAttr * self);
bool wxTextAttr_HasParagraphSpacingBefore(const wxTextAttr * self);
bool wxTextAttr_HasParagraphStyleName(const wxTextAttr * self);
bool wxTextAttr_HasRightIndent(const wxTextAttr * self);
bool wxTextAttr_HasTabs(const wxTextAttr * self);
bool wxTextAttr_HasTextColour(const wxTextAttr * self);
bool wxTextAttr_HasTextEffects(const wxTextAttr * self);
bool wxTextAttr_HasURL(const wxTextAttr * self);
bool wxTextAttr_IsCharacterStyle(const wxTextAttr * self);
bool wxTextAttr_IsDefault(const wxTextAttr * self);
bool wxTextAttr_IsParagraphStyle(const wxTextAttr * self);
void wxTextAttr_SetBackgroundColour(wxTextAttr * self, const wxColour * col_back);
void wxTextAttr_SetBulletFont(wxTextAttr * self, const wxString * font);
void wxTextAttr_SetBulletName(wxTextAttr * self, const wxString * name);
void wxTextAttr_SetBulletNumber(wxTextAttr * self, int n);
void wxTextAttr_SetBulletStyle(wxTextAttr * self, int style);
void wxTextAttr_SetBulletText(wxTextAttr * self, const wxString * text);
void wxTextAttr_SetCharacterStyleName(wxTextAttr * self, const wxString * name);
void wxTextAttr_SetFlags(wxTextAttr * self, long flags);
void wxTextAttr_SetFont(wxTextAttr * self, const wxFont * font, int flags);
void wxTextAttr_SetFontFaceName(wxTextAttr * self, const wxString * face_name);
void wxTextAttr_SetFontSize(wxTextAttr * self, int point_size);
void wxTextAttr_SetFontPointSize(wxTextAttr * self, int point_size);
void wxTextAttr_SetFontPixelSize(wxTextAttr * self, int pixel_size);
void wxTextAttr_SetFontUnderlined(wxTextAttr * self, bool underlined);
void wxTextAttr_SetLeftIndent(wxTextAttr * self, int indent, int sub_indent);
void wxTextAttr_SetLineSpacing(wxTextAttr * self, int spacing);
void wxTextAttr_SetListStyleName(wxTextAttr * self, const wxString * name);
void wxTextAttr_SetOutlineLevel(wxTextAttr * self, int level);
void wxTextAttr_SetPageBreak(wxTextAttr * self, bool page_break);
void wxTextAttr_SetParagraphSpacingAfter(wxTextAttr * self, int spacing);
void wxTextAttr_SetParagraphSpacingBefore(wxTextAttr * self, int spacing);
void wxTextAttr_SetParagraphStyleName(wxTextAttr * self, const wxString * name);
void wxTextAttr_SetRightIndent(wxTextAttr * self, int indent);
void wxTextAttr_SetTabs(wxTextAttr * self, const wxArrayInt * tabs);
void wxTextAttr_SetTextColour(wxTextAttr * self, const wxColour * col_text);
void wxTextAttr_SetTextEffectFlags(wxTextAttr * self, int flags);
void wxTextAttr_SetTextEffects(wxTextAttr * self, int effects);
void wxTextAttr_SetURL(wxTextAttr * self, const wxString * url);
wxTextAttr *wxTextAttr_new();
wxTextAttr *wxTextAttr_new2(const wxTextAttr * attr);
bool wxTextAttr_Apply(wxTextAttr * self, const wxTextAttr * style, const wxTextAttr * compare_with);
void wxTextAttr_Merge(wxTextAttr * self, const wxTextAttr * overlay);
bool wxTextAttr_EqPartial(const wxTextAttr * self, const wxTextAttr * attr, bool weak_test);
wxTextAttr *wxTextAttr_Merge1(const wxTextAttr * base, const wxTextAttr * overlay);

// CLASS: wxTextCtrl
#ifdef __WXOSX__
void wxTextCtrl_OSXEnableNewLineReplacement(wxTextCtrl * self, bool enable);
#endif
wxTextCtrl *wxTextCtrl_new();
wxTextCtrl *wxTextCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxTextCtrl_Create(wxTextCtrl * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxTextCtrl_DiscardEdits(wxTextCtrl * self);
void wxTextCtrl_EmptyUndoBuffer(wxTextCtrl * self);
bool wxTextCtrl_EmulateKeyPress(wxTextCtrl * self, const wxKeyEvent * event);
bool wxTextCtrl_EnableProofCheck(wxTextCtrl * self, const wxTextProofOptions * options);
wxTextAttr *wxTextCtrl_GetDefaultStyle(const wxTextCtrl * self);
int wxTextCtrl_GetLineLength(const wxTextCtrl * self, long line_no);
wxString *wxTextCtrl_GetLineText(const wxTextCtrl * self, long line_no);
int wxTextCtrl_GetNumberOfLines(const wxTextCtrl * self);
bool wxTextCtrl_GetStyle(wxTextCtrl * self, long position, wxTextAttr * style);
bool wxTextCtrl_IsModified(const wxTextCtrl * self);
bool wxTextCtrl_IsMultiLine(const wxTextCtrl * self);
bool wxTextCtrl_IsSingleLine(const wxTextCtrl * self);
bool wxTextCtrl_LoadFile(wxTextCtrl * self, const wxString * filename, int file_type);
void wxTextCtrl_MarkDirty(wxTextCtrl * self);
void wxTextCtrl_OnDropFiles(wxTextCtrl * self, wxDropFilesEvent * event);
bool wxTextCtrl_PositionToXY(const wxTextCtrl * self, long pos, long * x, long * y);
wxPoint *wxTextCtrl_PositionToCoords(const wxTextCtrl * self, long pos);
bool wxTextCtrl_SaveFile(wxTextCtrl * self, const wxString * filename, int file_type);
bool wxTextCtrl_SetDefaultStyle(wxTextCtrl * self, const wxTextAttr * style);
void wxTextCtrl_SetModified(wxTextCtrl * self, bool modified);
bool wxTextCtrl_SetStyle(wxTextCtrl * self, long start, long end, const wxTextAttr * style);
void wxTextCtrl_ShowPosition(wxTextCtrl * self, long pos);
long wxTextCtrl_XYToPosition(const wxTextCtrl * self, long x, long y);
// Mix-in(s) to wxTextCtrl
wxTextEntry *wxTextCtrl_AsTextEntry(wxTextCtrl* obj);

// CLASS: wxTextEntry
void wxTextEntry_delete(wxTextEntry *self);
void wxTextEntry_AppendText(wxTextEntry * self, const wxString * text);
bool wxTextEntry_AutoComplete(wxTextEntry * self, const wxArrayString * choices);
bool wxTextEntry_AutoComplete1(wxTextEntry * self, wxTextCompleter * completer);
bool wxTextEntry_AutoCompleteFileNames(wxTextEntry * self);
bool wxTextEntry_AutoCompleteDirectories(wxTextEntry * self);
bool wxTextEntry_CanCopy(const wxTextEntry * self);
bool wxTextEntry_CanCut(const wxTextEntry * self);
bool wxTextEntry_CanPaste(const wxTextEntry * self);
bool wxTextEntry_CanRedo(const wxTextEntry * self);
bool wxTextEntry_CanUndo(const wxTextEntry * self);
void wxTextEntry_ChangeValue(wxTextEntry * self, const wxString * value);
void wxTextEntry_Clear(wxTextEntry * self);
void wxTextEntry_Copy(wxTextEntry * self);
void wxTextEntry_Cut(wxTextEntry * self);
#if wxCHECK_VERSION(3, 1, 0)
void wxTextEntry_ForceUpper(wxTextEntry * self);
#endif
long wxTextEntry_GetInsertionPoint(const wxTextEntry * self);
wxString *wxTextEntry_GetRange(const wxTextEntry * self, long from, long to);
void wxTextEntry_GetSelection(const wxTextEntry * self, long * from, long * to);
wxString *wxTextEntry_GetStringSelection(const wxTextEntry * self);
wxString *wxTextEntry_GetValue(const wxTextEntry * self);
bool wxTextEntry_IsEditable(const wxTextEntry * self);
bool wxTextEntry_IsEmpty(const wxTextEntry * self);
void wxTextEntry_Paste(wxTextEntry * self);
void wxTextEntry_Redo(wxTextEntry * self);
void wxTextEntry_Remove(wxTextEntry * self, long from, long to);
void wxTextEntry_Replace(wxTextEntry * self, long from, long to, const wxString * value);
void wxTextEntry_SetEditable(wxTextEntry * self, bool editable);
void wxTextEntry_SetInsertionPoint(wxTextEntry * self, long pos);
void wxTextEntry_SetInsertionPointEnd(wxTextEntry * self);
void wxTextEntry_SetSelection(wxTextEntry * self, long from, long to);
void wxTextEntry_SelectAll(wxTextEntry * self);
void wxTextEntry_SelectNone(wxTextEntry * self);
bool wxTextEntry_SetHint(wxTextEntry * self, const wxString * hint);
wxString *wxTextEntry_GetHint(const wxTextEntry * self);
bool wxTextEntry_SetMargins(wxTextEntry * self, const wxPoint * pt);
bool wxTextEntry_SetMargins1(wxTextEntry * self, wxCoord left, wxCoord top);
wxPoint *wxTextEntry_GetMargins(const wxTextEntry * self);
void wxTextEntry_SetValue(wxTextEntry * self, const wxString * value);
void wxTextEntry_Undo(wxTextEntry * self);
void wxTextEntry_WriteText(wxTextEntry * self, const wxString * text);

// CLASS: wxToolBar
wxToolBar *wxToolBar_new();
wxToolBar *wxToolBar_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxToolBarToolBase * wxToolBar_AddCheckTool(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bitmap1, const wxBitmapBundle * bmp_disabled, const wxString * short_help, const wxString * long_help, wxObject * client_data);
wxToolBarToolBase * wxToolBar_AddControl(wxToolBar * self, wxControl * control, const wxString * label);
wxToolBarToolBase * wxToolBar_AddRadioTool(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bitmap1, const wxBitmapBundle * bmp_disabled, const wxString * short_help, const wxString * long_help, wxObject * client_data);
wxToolBarToolBase * wxToolBar_AddSeparator(wxToolBar * self);
wxToolBarToolBase * wxToolBar_AddStretchableSpace(wxToolBar * self);
wxToolBarToolBase * wxToolBar_AddTool(wxToolBar * self, wxToolBarToolBase * tool);
wxToolBarToolBase * wxToolBar_AddTool1(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bitmap, const wxString * short_help, wxItemKind kind);
wxToolBarToolBase * wxToolBar_AddTool2(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bitmap, const wxBitmapBundle * bmp_disabled, wxItemKind kind, const wxString * short_help, const wxString * long_help, wxObject * client_data);
void wxToolBar_ClearTools(wxToolBar * self);
bool wxToolBar_DeleteTool(wxToolBar * self, int tool_id);
bool wxToolBar_DeleteToolByPos(wxToolBar * self, size_t pos);
void wxToolBar_EnableTool(wxToolBar * self, int tool_id, bool enable);
wxToolBarToolBase * wxToolBar_FindById(const wxToolBar * self, int id);
wxControl * wxToolBar_FindControl(wxToolBar * self, int id);
wxToolBarToolBase * wxToolBar_FindToolForPosition(const wxToolBar * self, wxCoord x, wxCoord y);
wxSize *wxToolBar_GetMargins(const wxToolBar * self);
wxSize *wxToolBar_GetToolBitmapSize(const wxToolBar * self);
const wxToolBarToolBase * wxToolBar_GetToolByPos1(const wxToolBar * self, int pos);
wxObject * wxToolBar_GetToolClientData(const wxToolBar * self, int tool_id);
bool wxToolBar_GetToolEnabled(const wxToolBar * self, int tool_id);
wxString *wxToolBar_GetToolLongHelp(const wxToolBar * self, int tool_id);
int wxToolBar_GetToolPacking(const wxToolBar * self);
int wxToolBar_GetToolPos(const wxToolBar * self, int tool_id);
int wxToolBar_GetToolSeparation(const wxToolBar * self);
wxString *wxToolBar_GetToolShortHelp(const wxToolBar * self, int tool_id);
wxSize *wxToolBar_GetToolSize(const wxToolBar * self);
bool wxToolBar_GetToolState(const wxToolBar * self, int tool_id);
size_t wxToolBar_GetToolsCount(const wxToolBar * self);
wxToolBarToolBase * wxToolBar_InsertControl(wxToolBar * self, size_t pos, wxControl * control, const wxString * label);
wxToolBarToolBase * wxToolBar_InsertSeparator(wxToolBar * self, size_t pos);
wxToolBarToolBase * wxToolBar_InsertStretchableSpace(wxToolBar * self, size_t pos);
wxToolBarToolBase * wxToolBar_InsertTool(wxToolBar * self, size_t pos, int tool_id, const wxString * label, const wxBitmapBundle * bitmap, const wxBitmapBundle * bmp_disabled, wxItemKind kind, const wxString * short_help, const wxString * long_help, wxObject * client_data);
wxToolBarToolBase * wxToolBar_InsertTool1(wxToolBar * self, size_t pos, wxToolBarToolBase * tool);
bool wxToolBar_OnLeftClick(wxToolBar * self, int tool_id, bool toggle_down);
void wxToolBar_OnMouseEnter(wxToolBar * self, int tool_id);
void wxToolBar_OnRightClick(wxToolBar * self, int tool_id, long x, long y);
bool wxToolBar_Realize(wxToolBar * self);
wxToolBarToolBase * wxToolBar_RemoveTool(wxToolBar * self, int id);
bool wxToolBar_SetDropdownMenu(wxToolBar * self, int id, wxMenu * menu);
void wxToolBar_SetMargins(wxToolBar * self, int x, int y);
void wxToolBar_SetMargins1(wxToolBar * self, const wxSize * size);
void wxToolBar_SetToolBitmapSize(wxToolBar * self, const wxSize * size);
void wxToolBar_SetToolClientData(wxToolBar * self, int id, wxObject * client_data);
void wxToolBar_SetToolDisabledBitmap(wxToolBar * self, int id, const wxBitmapBundle * bitmap);
void wxToolBar_SetToolLongHelp(wxToolBar * self, int tool_id, const wxString * help_string);
void wxToolBar_SetToolNormalBitmap(wxToolBar * self, int id, const wxBitmapBundle * bitmap);
void wxToolBar_SetToolPacking(wxToolBar * self, int packing);
void wxToolBar_SetToolSeparation(wxToolBar * self, int separation);
void wxToolBar_SetToolShortHelp(wxToolBar * self, int tool_id, const wxString * help_string);
void wxToolBar_ToggleTool(wxToolBar * self, int tool_id, bool toggle);
wxToolBarToolBase * wxToolBar_CreateTool(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bmp_normal, const wxBitmapBundle * bmp_disabled, wxItemKind kind, wxObject * client_data, const wxString * short_help, const wxString * long_help);
wxToolBarToolBase * wxToolBar_CreateTool1(wxToolBar * self, wxControl * control, const wxString * label);
wxToolBarToolBase * wxToolBar_CreateSeparator(wxToolBar * self);

// CLASS: wxTopLevelWindow
wxTopLevelWindow *wxTopLevelWindow_new();
wxTopLevelWindow *wxTopLevelWindow_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxTopLevelWindow_Create(wxTopLevelWindow * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxTopLevelWindow_CenterOnScreen(wxTopLevelWindow * self, int direction);
void wxTopLevelWindow_CentreOnScreen(wxTopLevelWindow * self, int direction);
bool wxTopLevelWindow_EnableCloseButton(wxTopLevelWindow * self, bool enable);
#if wxCHECK_VERSION(3, 1, 0)
bool wxTopLevelWindow_EnableMaximizeButton(wxTopLevelWindow * self, bool enable);
bool wxTopLevelWindow_EnableMinimizeButton(wxTopLevelWindow * self, bool enable);
#endif
wxWindow * wxTopLevelWindow_GetDefaultItem(const wxTopLevelWindow * self);
wxIcon *wxTopLevelWindow_GetIcon(const wxTopLevelWindow * self);
wxString *wxTopLevelWindow_GetTitle(const wxTopLevelWindow * self);
void wxTopLevelWindow_Iconize(wxTopLevelWindow * self, bool iconize);
bool wxTopLevelWindow_IsActive(wxTopLevelWindow * self);
bool wxTopLevelWindow_IsAlwaysMaximized(const wxTopLevelWindow * self);
bool wxTopLevelWindow_IsFullScreen(const wxTopLevelWindow * self);
bool wxTopLevelWindow_IsIconized(const wxTopLevelWindow * self);
bool wxTopLevelWindow_IsMaximized(const wxTopLevelWindow * self);
void wxTopLevelWindow_Maximize(wxTopLevelWindow * self, bool maximize);
#ifdef __WXMSW__
wxMenu * wxTopLevelWindow_MSWGetSystemMenu(const wxTopLevelWindow * self);
#endif
void wxTopLevelWindow_RequestUserAttention(wxTopLevelWindow * self, int flags);
void wxTopLevelWindow_Restore(wxTopLevelWindow * self);
wxWindow * wxTopLevelWindow_SetDefaultItem(wxTopLevelWindow * self, wxWindow * win);
wxWindow * wxTopLevelWindow_SetTmpDefaultItem(wxTopLevelWindow * self, wxWindow * win);
wxWindow * wxTopLevelWindow_GetTmpDefaultItem(const wxTopLevelWindow * self);
void wxTopLevelWindow_SetIcon(wxTopLevelWindow * self, const wxIcon * icon);
void wxTopLevelWindow_SetIcons(wxTopLevelWindow * self, const wxIconBundle * icons);
void wxTopLevelWindow_SetTitle(wxTopLevelWindow * self, const wxString * title);
bool wxTopLevelWindow_ShouldPreventAppExit(const wxTopLevelWindow * self);
void wxTopLevelWindow_OSXSetModified(wxTopLevelWindow * self, bool modified);
bool wxTopLevelWindow_OSXIsModified(const wxTopLevelWindow * self);
void wxTopLevelWindow_SetRepresentedFilename(wxTopLevelWindow * self, const wxString * filename);
void wxTopLevelWindow_ShowWithoutActivating(wxTopLevelWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxTopLevelWindow_EnableFullScreenView(wxTopLevelWindow * self, bool enable, long style);
#endif
bool wxTopLevelWindow_ShowFullScreen(wxTopLevelWindow * self, bool show, long style);
wxSize *wxTopLevelWindow_GetDefaultSize();

// CLASS: wxValidator
wxValidator *wxValidator_new();
wxObject * wxValidator_Clone(const wxValidator * self);
wxWindow * wxValidator_GetWindow(const wxValidator * self);
void wxValidator_SetWindow(wxValidator * self, wxWindow * window);
bool wxValidator_TransferFromWindow(wxValidator * self);
bool wxValidator_TransferToWindow(wxValidator * self);
bool wxValidator_Validate(wxValidator * self, wxWindow * parent);
void wxValidator_SuppressBellOnError(bool suppress);
bool wxValidator_IsSilent();

// CLASS: wxWindow
bool wxWindow_AcceptsFocus(const wxWindow * self);
bool wxWindow_AcceptsFocusFromKeyboard(const wxWindow * self);
bool wxWindow_AcceptsFocusRecursively(const wxWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
void wxWindow_DisableFocusFromKeyboard(wxWindow * self);
#endif
bool wxWindow_IsFocusable(const wxWindow * self);
bool wxWindow_CanAcceptFocus(const wxWindow * self);
bool wxWindow_CanAcceptFocusFromKeyboard(const wxWindow * self);
bool wxWindow_HasFocus(const wxWindow * self);
void wxWindow_SetCanFocus(wxWindow * self, bool can_focus);
#if wxCHECK_VERSION(3, 1, 0)
void wxWindow_EnableVisibleFocus(wxWindow * self, bool enable);
#endif
void wxWindow_SetFocus(wxWindow * self);
void wxWindow_SetFocusFromKbd(wxWindow * self);
void wxWindow_AddChild(wxWindow * self, wxWindow * child);
bool wxWindow_DestroyChildren(wxWindow * self);
wxWindow * wxWindow_FindWindow(const wxWindow * self, long id);
wxWindow * wxWindow_FindWindow1(const wxWindow * self, const wxString * name);
wxWindowList *wxWindow_GetChildren1(const wxWindow * self);
void wxWindow_RemoveChild(wxWindow * self, wxWindow * child);
wxWindow * wxWindow_GetGrandParent(const wxWindow * self);
wxWindow * wxWindow_GetNextSibling(const wxWindow * self);
wxWindow * wxWindow_GetParent(const wxWindow * self);
wxWindow * wxWindow_GetPrevSibling(const wxWindow * self);
bool wxWindow_IsDescendant(const wxWindow * self, wxWindow * win);
bool wxWindow_Reparent(wxWindow * self, wxWindow * new_parent);
void wxWindow_AlwaysShowScrollbars(wxWindow * self, bool hflag, bool vflag);
int wxWindow_GetScrollPos(const wxWindow * self, int orientation);
int wxWindow_GetScrollRange(const wxWindow * self, int orientation);
int wxWindow_GetScrollThumb(const wxWindow * self, int orientation);
bool wxWindow_CanScroll(const wxWindow * self, int orient);
bool wxWindow_HasScrollbar(const wxWindow * self, int orient);
bool wxWindow_IsScrollbarAlwaysShown(const wxWindow * self, int orient);
bool wxWindow_ScrollLines(wxWindow * self, int lines);
bool wxWindow_ScrollPages(wxWindow * self, int pages);
void wxWindow_ScrollWindow(wxWindow * self, int dx, int dy, const wxRect * rect);
bool wxWindow_LineUp(wxWindow * self);
bool wxWindow_LineDown(wxWindow * self);
bool wxWindow_PageUp(wxWindow * self);
bool wxWindow_PageDown(wxWindow * self);
void wxWindow_SetScrollPos(wxWindow * self, int orientation, int pos, bool refresh);
void wxWindow_SetScrollbar(wxWindow * self, int orientation, int position, int thumb_size, int range, bool refresh);
bool wxWindow_BeginRepositioningChildren(wxWindow * self);
void wxWindow_EndRepositioningChildren(wxWindow * self);
void wxWindow_CacheBestSize(const wxWindow * self, const wxSize * size);
wxSize *wxWindow_ClientToWindowSize(const wxWindow * self, const wxSize * size);
wxSize *wxWindow_WindowToClientSize(const wxWindow * self, const wxSize * size);
void wxWindow_Fit(wxWindow * self);
void wxWindow_FitInside(wxWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxWindow_FromDIP(const wxWindow * self, const wxSize * sz);
wxPoint *wxWindow_FromDIP1(const wxWindow * self, const wxPoint * pt);
int wxWindow_FromDIP2(const wxWindow * self, int d);
wxSize *wxWindow_ToDIP(const wxWindow * self, const wxSize * sz);
wxPoint *wxWindow_ToDIP1(const wxWindow * self, const wxPoint * pt);
int wxWindow_ToDIP2(const wxWindow * self, int d);
#endif
wxSize *wxWindow_FromPhys(const wxWindow * self, const wxSize * sz);
wxPoint *wxWindow_FromPhys1(const wxWindow * self, const wxPoint * pt);
int wxWindow_FromPhys2(const wxWindow * self, int d);
wxSize *wxWindow_ToPhys(const wxWindow * self, const wxSize * sz);
wxPoint *wxWindow_ToPhys1(const wxWindow * self, const wxPoint * pt);
int wxWindow_ToPhys2(const wxWindow * self, int d);
wxSize *wxWindow_GetBestSize(const wxWindow * self);
int wxWindow_GetBestHeight(const wxWindow * self, int width);
int wxWindow_GetBestWidth(const wxWindow * self, int height);
void wxWindow_GetClientSize(const wxWindow * self, int * width, int * height);
wxSize *wxWindow_GetClientSize1(const wxWindow * self);
wxSize *wxWindow_GetEffectiveMinSize(const wxWindow * self);
wxSize *wxWindow_GetMaxClientSize(const wxWindow * self);
wxSize *wxWindow_GetMaxSize(const wxWindow * self);
wxSize *wxWindow_GetMinClientSize(const wxWindow * self);
wxSize *wxWindow_GetMinSize(const wxWindow * self);
int wxWindow_GetMinWidth(const wxWindow * self);
int wxWindow_GetMinHeight(const wxWindow * self);
int wxWindow_GetMaxWidth(const wxWindow * self);
int wxWindow_GetMaxHeight(const wxWindow * self);
void wxWindow_GetSize(const wxWindow * self, int * width, int * height);
wxSize *wxWindow_GetSize1(const wxWindow * self);
wxSize *wxWindow_GetVirtualSize(const wxWindow * self);
void wxWindow_GetVirtualSize1(const wxWindow * self, int * width, int * height);
wxSize *wxWindow_GetBestVirtualSize(const wxWindow * self);
double wxWindow_GetContentScaleFactor(const wxWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
double wxWindow_GetDPIScaleFactor(const wxWindow * self);
#endif
wxSize *wxWindow_GetWindowBorderSize(const wxWindow * self);
bool wxWindow_InformFirstDirection(wxWindow * self, int direction, int size, int available_other_dir);
void wxWindow_InvalidateBestSize(wxWindow * self);
void wxWindow_PostSizeEvent(wxWindow * self);
void wxWindow_PostSizeEventToParent(wxWindow * self);
void wxWindow_SendSizeEvent(wxWindow * self, int flags);
void wxWindow_SendSizeEventToParent(wxWindow * self, int flags);
void wxWindow_SetClientSize(wxWindow * self, int width, int height);
void wxWindow_SetClientSize1(wxWindow * self, const wxSize * size);
void wxWindow_SetClientSize2(wxWindow * self, const wxRect * rect);
void wxWindow_SetContainingSizer(wxWindow * self, wxSizer * sizer);
void wxWindow_SetInitialSize(wxWindow * self, const wxSize * size);
void wxWindow_SetMaxClientSize(wxWindow * self, const wxSize * size);
void wxWindow_SetMaxSize(wxWindow * self, const wxSize * size);
void wxWindow_SetMinClientSize(wxWindow * self, const wxSize * size);
void wxWindow_SetMinSize(wxWindow * self, const wxSize * size);
void wxWindow_SetSize(wxWindow * self, int x, int y, int width, int height, int size_flags);
void wxWindow_SetSize1(wxWindow * self, const wxRect * rect);
void wxWindow_SetSize2(wxWindow * self, const wxSize * size);
void wxWindow_SetSize3(wxWindow * self, int width, int height);
void wxWindow_SetSizeHints(wxWindow * self, const wxSize * min_size, const wxSize * max_size, const wxSize * inc_size);
void wxWindow_SetSizeHints1(wxWindow * self, int min_w, int min_h, int max_w, int max_h, int inc_w, int inc_h);
void wxWindow_SetVirtualSize(wxWindow * self, int width, int height);
void wxWindow_SetVirtualSize1(wxWindow * self, const wxSize * size);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxWindow_FromDIP3(const wxSize * sz, const wxWindow * w);
wxPoint *wxWindow_FromDIP4(const wxPoint * pt, const wxWindow * w);
int wxWindow_FromDIP5(int d, const wxWindow * w);
wxSize *wxWindow_ToDIP3(const wxSize * sz, const wxWindow * w);
wxPoint *wxWindow_ToDIP4(const wxPoint * pt, const wxWindow * w);
int wxWindow_ToDIP5(int d, const wxWindow * w);
#endif
wxSize *wxWindow_FromPhys3(const wxSize * sz, const wxWindow * w);
wxPoint *wxWindow_FromPhys4(const wxPoint * pt, const wxWindow * w);
int wxWindow_FromPhys5(int d, const wxWindow * w);
wxSize *wxWindow_ToPhys3(const wxSize * sz, const wxWindow * w);
wxPoint *wxWindow_ToPhys4(const wxPoint * pt, const wxWindow * w);
int wxWindow_ToPhys5(int d, const wxWindow * w);
void wxWindow_Center(wxWindow * self, int dir);
void wxWindow_CenterOnParent(wxWindow * self, int dir);
void wxWindow_Centre(wxWindow * self, int direction);
void wxWindow_CentreOnParent(wxWindow * self, int direction);
void wxWindow_GetPosition(const wxWindow * self, int * x, int * y);
wxPoint *wxWindow_GetPosition1(const wxWindow * self);
wxRect *wxWindow_GetRect(const wxWindow * self);
void wxWindow_GetScreenPosition(const wxWindow * self, int * x, int * y);
wxPoint *wxWindow_GetScreenPosition1(const wxWindow * self);
wxRect *wxWindow_GetScreenRect(const wxWindow * self);
wxPoint *wxWindow_GetClientAreaOrigin(const wxWindow * self);
wxRect *wxWindow_GetClientRect(const wxWindow * self);
void wxWindow_Move(wxWindow * self, int x, int y, int flags);
void wxWindow_Move1(wxWindow * self, const wxPoint * pt, int flags);
void wxWindow_SetPosition(wxWindow * self, const wxPoint * pt);
void wxWindow_ClientToScreen(const wxWindow * self, int * x, int * y);
wxPoint *wxWindow_ClientToScreen1(const wxWindow * self, const wxPoint * pt);
wxPoint *wxWindow_ConvertDialogToPixels(const wxWindow * self, const wxPoint * pt);
wxSize *wxWindow_ConvertDialogToPixels1(const wxWindow * self, const wxSize * sz);
wxPoint *wxWindow_ConvertPixelsToDialog(const wxWindow * self, const wxPoint * pt);
wxSize *wxWindow_ConvertPixelsToDialog1(const wxWindow * self, const wxSize * sz);
void wxWindow_ScreenToClient(const wxWindow * self, int * x, int * y);
wxPoint *wxWindow_ScreenToClient1(const wxWindow * self, const wxPoint * pt);
void wxWindow_ClearBackground(wxWindow * self);
void wxWindow_Freeze(wxWindow * self);
void wxWindow_Thaw(wxWindow * self);
bool wxWindow_IsFrozen(const wxWindow * self);
wxColour *wxWindow_GetBackgroundColour(const wxWindow * self);
int wxWindow_GetCharHeight(const wxWindow * self);
int wxWindow_GetCharWidth(const wxWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxWindow_GetDPI(const wxWindow * self);
#endif
wxColour *wxWindow_GetForegroundColour(const wxWindow * self);
void wxWindow_GetTextExtent(const wxWindow * self, const wxString * string, int * w, int * h, int * descent, int * external_leading, const wxFont * font);
wxSize *wxWindow_GetTextExtent1(const wxWindow * self, const wxString * string);
wxRect *wxWindow_GetUpdateClientRect(const wxWindow * self);
bool wxWindow_HasTransparentBackground(wxWindow * self);
void wxWindow_Refresh(wxWindow * self, bool erase_background, const wxRect * rect);
void wxWindow_RefreshRect(wxWindow * self, const wxRect * rect, bool erase_background);
void wxWindow_Update(wxWindow * self);
bool wxWindow_SetBackgroundColour(wxWindow * self, const wxColour * colour);
bool wxWindow_IsTransparentBackgroundSupported(const wxWindow * self, wxString * reason);
bool wxWindow_SetFont(wxWindow * self, const wxFont * font);
bool wxWindow_SetForegroundColour(wxWindow * self, const wxColour * colour);
void wxWindow_SetOwnBackgroundColour(wxWindow * self, const wxColour * colour);
bool wxWindow_InheritsBackgroundColour(const wxWindow * self);
bool wxWindow_UseBgCol(const wxWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxWindow_UseBackgroundColour(const wxWindow * self);
#endif
void wxWindow_SetOwnFont(wxWindow * self, const wxFont * font);
void wxWindow_SetOwnForegroundColour(wxWindow * self, const wxColour * colour);
#if wxCHECK_VERSION(3, 1, 0)
bool wxWindow_UseForegroundColour(const wxWindow * self);
bool wxWindow_InheritsForegroundColour(const wxWindow * self);
#endif
void wxWindow_SetPalette(wxWindow * self, const wxPalette * pal);
bool wxWindow_ShouldInheritColours(const wxWindow * self);
void wxWindow_SetThemeEnabled(wxWindow * self, bool enable);
bool wxWindow_GetThemeEnabled(const wxWindow * self);
bool wxWindow_CanSetTransparent(wxWindow * self);
bool wxWindow_SetTransparent(wxWindow * self, wxByte alpha);
wxEvtHandler * wxWindow_GetEventHandler(const wxWindow * self);
bool wxWindow_HandleAsNavigationKey(wxWindow * self, const wxKeyEvent * event);
bool wxWindow_HandleWindowEvent(const wxWindow * self, wxEvent * event);
bool wxWindow_ProcessWindowEvent(wxWindow * self, wxEvent * event);
bool wxWindow_ProcessWindowEventLocally(wxWindow * self, wxEvent * event);
wxEvtHandler * wxWindow_PopEventHandler(wxWindow * self, bool delete_handler);
void wxWindow_PushEventHandler(wxWindow * self, wxEvtHandler * handler);
bool wxWindow_RemoveEventHandler(wxWindow * self, wxEvtHandler * handler);
void wxWindow_SetEventHandler(wxWindow * self, wxEvtHandler * handler);
long wxWindow_GetExtraStyle(const wxWindow * self);
long wxWindow_GetWindowStyleFlag(const wxWindow * self);
long wxWindow_GetWindowStyle(const wxWindow * self);
bool wxWindow_HasExtraStyle(const wxWindow * self, int ex_flag);
bool wxWindow_HasFlag(const wxWindow * self, int flag);
void wxWindow_SetExtraStyle(wxWindow * self, long ex_style);
void wxWindow_SetWindowStyleFlag(wxWindow * self, long style);
void wxWindow_SetWindowStyle(wxWindow * self, long style);
bool wxWindow_ToggleWindowStyle(wxWindow * self, int flag);
void wxWindow_MoveAfterInTabOrder(wxWindow * self, wxWindow * win);
void wxWindow_MoveBeforeInTabOrder(wxWindow * self, wxWindow * win);
bool wxWindow_Navigate(wxWindow * self, int flags);
bool wxWindow_NavigateIn(wxWindow * self, int flags);
void wxWindow_Lower(wxWindow * self);
void wxWindow_Raise(wxWindow * self);
bool wxWindow_Hide(wxWindow * self);
bool wxWindow_IsEnabled(const wxWindow * self);
bool wxWindow_IsExposed(const wxWindow * self, int x, int y);
bool wxWindow_IsExposed1(const wxWindow * self, wxPoint * pt);
bool wxWindow_IsExposed2(const wxWindow * self, int x, int y, int w, int h);
bool wxWindow_IsExposed3(const wxWindow * self, wxRect * rect);
bool wxWindow_IsShown(const wxWindow * self);
bool wxWindow_IsShownOnScreen(const wxWindow * self);
bool wxWindow_Disable(wxWindow * self);
bool wxWindow_Enable(wxWindow * self, bool enable);
bool wxWindow_Show(wxWindow * self, bool show);
#if wxCHECK_VERSION(3, 1, 7)
wxString *wxWindow_GetHelpText(const wxWindow * self);
#endif
void wxWindow_SetHelpText(wxWindow * self, const wxString * help_text);
wxToolTip * wxWindow_GetToolTip(const wxWindow * self);
wxString *wxWindow_GetToolTipText(const wxWindow * self);
void wxWindow_SetToolTip(wxWindow * self, const wxString * tip_string);
void wxWindow_SetToolTip1(wxWindow * self, wxToolTip * tip);
void wxWindow_UnsetToolTip(wxWindow * self);
int wxWindow_GetPopupMenuSelectionFromUser(wxWindow * self, wxMenu * menu, const wxPoint * pos);
int wxWindow_GetPopupMenuSelectionFromUser1(wxWindow * self, wxMenu * menu, int x, int y);
bool wxWindow_PopupMenu(wxWindow * self, wxMenu * menu, const wxPoint * pos);
bool wxWindow_PopupMenu1(wxWindow * self, wxMenu * menu, int x, int y);
wxValidator * wxWindow_GetValidator(wxWindow * self);
void wxWindow_SetValidator(wxWindow * self, const wxValidator * validator);
bool wxWindow_TransferDataFromWindow(wxWindow * self);
bool wxWindow_TransferDataToWindow(wxWindow * self);
bool wxWindow_Validate(wxWindow * self);
wxWindowID wxWindow_GetId(const wxWindow * self);
wxString *wxWindow_GetLabel(const wxWindow * self);
wxLayoutDirection wxWindow_GetLayoutDirection(const wxWindow * self);
wxCoord wxWindow_AdjustForLayoutDirection(const wxWindow * self, wxCoord x, wxCoord width, wxCoord width_total);
wxString *wxWindow_GetName(const wxWindow * self);
void wxWindow_SetId(wxWindow * self, wxWindowID winid);
void wxWindow_SetLabel(wxWindow * self, const wxString * label);
void wxWindow_SetLayoutDirection(wxWindow * self, wxLayoutDirection dir);
void wxWindow_SetName(wxWindow * self, const wxString * name);
wxAcceleratorTable * wxWindow_GetAcceleratorTable(wxWindow * self);
void wxWindow_SetAcceleratorTable(wxWindow * self, const wxAcceleratorTable * accel);
bool wxWindow_Close(wxWindow * self, bool force);
bool wxWindow_Destroy(wxWindow * self);
bool wxWindow_IsBeingDeleted(const wxWindow * self);
wxDropTarget * wxWindow_GetDropTarget(const wxWindow * self);
void wxWindow_SetDropTarget(wxWindow * self, wxDropTarget * target);
void wxWindow_DragAcceptFiles(wxWindow * self, bool accept);
wxSizer * wxWindow_GetContainingSizer(const wxWindow * self);
wxSizer * wxWindow_GetSizer(const wxWindow * self);
void wxWindow_SetSizer(wxWindow * self, wxSizer * sizer, bool delete_old);
void wxWindow_SetSizerAndFit(wxWindow * self, wxSizer * sizer, bool delete_old);
wxLayoutConstraints * wxWindow_GetConstraints(const wxWindow * self);
void wxWindow_SetConstraints(wxWindow * self, wxLayoutConstraints * constraints);
bool wxWindow_Layout(wxWindow * self);
void wxWindow_SetAutoLayout(wxWindow * self, bool auto_layout);
bool wxWindow_GetAutoLayout(const wxWindow * self);
void wxWindow_CaptureMouse(wxWindow * self);
wxCaret * wxWindow_GetCaret(const wxWindow * self);
bool wxWindow_HasCapture(const wxWindow * self);
void wxWindow_ReleaseMouse(wxWindow * self);
void wxWindow_SetCaret(wxWindow * self, wxCaret * caret);
bool wxWindow_SetCursor(wxWindow * self, const wxCursor * cursor);
void wxWindow_WarpPointer(wxWindow * self, int x, int y);
#if wxCHECK_VERSION(3, 1, 0)
bool wxWindow_EnableTouchEvents(wxWindow * self, int events_mask);
#endif
void wxWindow_DoUpdateWindowUI(wxWindow * self, wxUpdateUIEvent * event);
bool wxWindow_HasMultiplePages(const wxWindow * self);
void wxWindow_InheritAttributes(wxWindow * self);
void wxWindow_InitDialog(wxWindow * self);
bool wxWindow_IsDoubleBuffered(const wxWindow * self);
void wxWindow_SetDoubleBuffered(wxWindow * self, bool on);
bool wxWindow_IsRetained(const wxWindow * self);
bool wxWindow_IsThisEnabled(const wxWindow * self);
bool wxWindow_IsTopLevel(const wxWindow * self);
void wxWindow_OnInternalIdle(wxWindow * self);
bool wxWindow_SendIdleEvents(wxWindow * self, wxIdleEvent * event);
#ifndef __WXGTK__
bool wxWindow_RegisterHotKey(wxWindow * self, int hotkey_id, int modifiers, int virtual_key_code);
bool wxWindow_UnregisterHotKey(wxWindow * self, int hotkey_id);
#endif
void wxWindow_UpdateWindowUI(wxWindow * self, long flags);
wxWindow * wxWindow_FindFocus();
wxWindow * wxWindow_FindWindowById(long id, const wxWindow * parent);
wxWindow * wxWindow_FindWindowByLabel(const wxString * label, const wxWindow * parent);
wxWindow * wxWindow_FindWindowByName(const wxString * name, const wxWindow * parent);
wxWindow * wxWindow_GetCapture();
wxWindowID wxWindow_NewControlId(int count);
void wxWindow_UnreserveControlId(wxWindowID id, int count);
wxWindow *wxWindow_new();
wxWindow *wxWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxWindow_Create(wxWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);

// CLASS: wxWrapSizer
wxWrapSizer *wxWrapSizer_new(int orient, int flags);

} // extern "C"

