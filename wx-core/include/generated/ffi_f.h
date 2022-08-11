#pragma once
#include <wx/wx.h>
#include <wx/filectrl.h>
#include <wx/filepicker.h>
#include <wx/font.h>
#include <wx/fontpicker.h>
#include <wx/frame.h>

// wxBitmapBundle compatibility hack(for a while)
#if !wxCHECK_VERSION(3, 1, 6)
typedef wxBitmap wxBitmapBundle;
#endif

extern "C" {

// CLASS: wxFileCtrl
wxClassInfo *wxFileCtrl_CLASSINFO();
wxFileCtrl *wxFileCtrl_new();
wxFileCtrl *wxFileCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * default_directory, const wxString * default_filename, const wxString * wild_card, long style, const wxPoint * pos, const wxSize * size, const wxString * name);
bool wxFileCtrl_Create(wxFileCtrl * self, wxWindow * parent, wxWindowID id, const wxString * default_directory, const wxString * default_filename, const wxString * wild_card, long style, const wxPoint * pos, const wxSize * size, const wxString * name);
wxString *wxFileCtrl_GetDirectory(const wxFileCtrl * self);
wxString *wxFileCtrl_GetFilename(const wxFileCtrl * self);
void wxFileCtrl_GetFilenames(const wxFileCtrl * self, wxArrayString * filenames);
int wxFileCtrl_GetFilterIndex(const wxFileCtrl * self);
wxString *wxFileCtrl_GetPath(const wxFileCtrl * self);
void wxFileCtrl_GetPaths(const wxFileCtrl * self, wxArrayString * paths);
wxString *wxFileCtrl_GetWildcard(const wxFileCtrl * self);
bool wxFileCtrl_SetDirectory(wxFileCtrl * self, const wxString * directory);
bool wxFileCtrl_SetFilename(wxFileCtrl * self, const wxString * filename);
bool wxFileCtrl_SetPath(wxFileCtrl * self, const wxString * path);
void wxFileCtrl_SetFilterIndex(wxFileCtrl * self, int filter_index);
void wxFileCtrl_SetWildcard(wxFileCtrl * self, const wxString * wild_card);
void wxFileCtrl_ShowHidden(wxFileCtrl * self, bool show);

// CLASS: wxFilePickerCtrl
wxClassInfo *wxFilePickerCtrl_CLASSINFO();
wxFilePickerCtrl *wxFilePickerCtrl_new();
wxFilePickerCtrl *wxFilePickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxString * wildcard, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxFilePickerCtrl_Create(wxFilePickerCtrl * self, wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxString * wildcard, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
wxFileName *wxFilePickerCtrl_GetFileName(const wxFilePickerCtrl * self);
wxString *wxFilePickerCtrl_GetPath(const wxFilePickerCtrl * self);
void wxFilePickerCtrl_SetFileName(wxFilePickerCtrl * self, const wxFileName * filename);
void wxFilePickerCtrl_SetInitialDirectory(wxFilePickerCtrl * self, const wxString * dir);
void wxFilePickerCtrl_SetPath(wxFilePickerCtrl * self, const wxString * filename);

// CLASS: wxFont
wxClassInfo *wxFont_CLASSINFO();
#if wxCHECK_VERSION(3, 1, 0)
wxFont *wxFont_GetBaseFont(const wxFont * self);
#endif
wxString *wxFont_GetFaceName(const wxFont * self);
wxString *wxFont_GetNativeFontInfoDesc(const wxFont * self);
wxString *wxFont_GetNativeFontInfoUserDesc(const wxFont * self);
const wxNativeFontInfo * wxFont_GetNativeFontInfo(const wxFont * self);
int wxFont_GetPointSize(const wxFont * self);
#if wxCHECK_VERSION(3, 1, 0)
double wxFont_GetFractionalPointSize(const wxFont * self);
#endif
wxSize *wxFont_GetPixelSize(const wxFont * self);
bool wxFont_GetUnderlined(const wxFont * self);
bool wxFont_GetStrikethrough(const wxFont * self);
#if wxCHECK_VERSION(3, 1, 0)
int wxFont_GetNumericWeight(const wxFont * self);
#endif
bool wxFont_IsFixedWidth(const wxFont * self);
bool wxFont_IsOk(const wxFont * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxFont_AddPrivateFont(const wxString * filename);
#endif
wxFont *wxFont_Bold(const wxFont * self);
wxFont *wxFont_Italic(const wxFont * self);
wxFont *wxFont_Larger(const wxFont * self);
wxFont *wxFont_Smaller(const wxFont * self);
wxFont *wxFont_Underlined(const wxFont * self);
wxFont *wxFont_Strikethrough(const wxFont * self);
wxFont * wxFont_MakeBold(wxFont * self);
wxFont * wxFont_MakeItalic(wxFont * self);
wxFont * wxFont_MakeLarger(wxFont * self);
wxFont * wxFont_MakeSmaller(wxFont * self);
wxFont * wxFont_MakeUnderlined(wxFont * self);
wxFont * wxFont_MakeStrikethrough(wxFont * self);
bool wxFont_SetFaceName(wxFont * self, const wxString * face_name);
bool wxFont_SetNativeFontInfo(wxFont * self, const wxString * info);
bool wxFont_SetNativeFontInfoUserDesc(wxFont * self, const wxString * info);
void wxFont_SetNativeFontInfo1(wxFont * self, const wxNativeFontInfo * info);
void wxFont_SetPointSize(wxFont * self, int point_size);
#if wxCHECK_VERSION(3, 1, 0)
void wxFont_SetFractionalPointSize(wxFont * self, double point_size);
#endif
void wxFont_SetPixelSize(wxFont * self, const wxSize * pixel_size);
void wxFont_SetUnderlined(wxFont * self, bool underlined);
void wxFont_SetStrikethrough(wxFont * self, bool strikethrough);
#if wxCHECK_VERSION(3, 1, 0)
void wxFont_SetNumericWeight(wxFont * self, int weight);
#endif
wxFont * wxFont_New4(const wxNativeFontInfo * native_info);
wxFont * wxFont_New5(const wxString * native_info_string);
wxFont *wxFont_new();
wxFont *wxFont_new1(const wxFont * font);
wxFont *wxFont_new2(const wxFontInfo * font_info);
wxFont *wxFont_new5(const wxString * native_info_string);
wxFont *wxFont_new6(const wxNativeFontInfo * native_info);

// CLASS: wxFontPickerCtrl
wxClassInfo *wxFontPickerCtrl_CLASSINFO();
wxFontPickerCtrl *wxFontPickerCtrl_new();
wxFontPickerCtrl *wxFontPickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxFont * font, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxFontPickerCtrl_Create(wxFontPickerCtrl * self, wxWindow * parent, wxWindowID id, const wxFont * font, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
unsigned int wxFontPickerCtrl_GetMaxPointSize(const wxFontPickerCtrl * self);
#if wxCHECK_VERSION(3, 1, 0)
unsigned int wxFontPickerCtrl_GetMinPointSize(const wxFontPickerCtrl * self);
wxColour *wxFontPickerCtrl_GetSelectedColour(const wxFontPickerCtrl * self);
#endif
wxFont *wxFontPickerCtrl_GetSelectedFont(const wxFontPickerCtrl * self);
void wxFontPickerCtrl_SetMaxPointSize(wxFontPickerCtrl * self, unsigned int max);
#if wxCHECK_VERSION(3, 1, 0)
void wxFontPickerCtrl_SetMinPointSize(wxFontPickerCtrl * self, unsigned int min);
void wxFontPickerCtrl_SetSelectedColour(wxFontPickerCtrl * self, const wxColour * colour);
#endif
void wxFontPickerCtrl_SetSelectedFont(wxFontPickerCtrl * self, const wxFont * font);

// CLASS: wxFrame
wxClassInfo *wxFrame_CLASSINFO();
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

} // extern "C"

