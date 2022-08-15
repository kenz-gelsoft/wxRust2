#pragma once

#include <wx/dataobj.h>
#include <wx/dnd.h>
#include <wx/event.h>
#include <wx/fdrepdlg.h>
#include <wx/filectrl.h>
#include <wx/filedlg.h>
#include <wx/filehistory.h>
#include <wx/filepicker.h>
#include <wx/font.h>
#include <wx/fontdata.h>
#include <wx/fontdlg.h>
#include <wx/fontenum.h>
#include <wx/fontmap.h>
#include <wx/fontpicker.h>
#include <wx/frame.h>
#include <wx/sizer.h>

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

// CLASS: wxFileCtrlEvent
wxClassInfo *wxFileCtrlEvent_CLASSINFO();
wxString *wxFileCtrlEvent_GetDirectory(const wxFileCtrlEvent * self);
wxString *wxFileCtrlEvent_GetFile(const wxFileCtrlEvent * self);
wxArrayString *wxFileCtrlEvent_GetFiles(const wxFileCtrlEvent * self);
int wxFileCtrlEvent_GetFilterIndex(const wxFileCtrlEvent * self);
void wxFileCtrlEvent_SetFiles(wxFileCtrlEvent * self, const wxArrayString * files);
void wxFileCtrlEvent_SetDirectory(wxFileCtrlEvent * self, const wxString * directory);
void wxFileCtrlEvent_SetFilterIndex(wxFileCtrlEvent * self, int index);

// CLASS: wxFileDataObject
void wxFileDataObject_delete(wxFileDataObject *self);
wxFileDataObject *wxFileDataObject_new();
void wxFileDataObject_AddFile(wxFileDataObject * self, const wxString * file);
wxArrayString *wxFileDataObject_GetFilenames(const wxFileDataObject * self);

// CLASS: wxFileDialog
wxClassInfo *wxFileDialog_CLASSINFO();
wxFileDialog *wxFileDialog_new(wxWindow * parent, const wxString * message, const wxString * default_dir, const wxString * default_file, const wxString * wildcard, long style, const wxPoint * pos, const wxSize * size, const wxString * name);
wxString *wxFileDialog_GetCurrentlySelectedFilename(const wxFileDialog * self);
int wxFileDialog_GetCurrentlySelectedFilterIndex(const wxFileDialog * self);
wxString *wxFileDialog_GetDirectory(const wxFileDialog * self);
wxWindow * wxFileDialog_GetExtraControl(const wxFileDialog * self);
wxString *wxFileDialog_GetFilename(const wxFileDialog * self);
void wxFileDialog_GetFilenames(const wxFileDialog * self, wxArrayString * filenames);
int wxFileDialog_GetFilterIndex(const wxFileDialog * self);
wxString *wxFileDialog_GetMessage(const wxFileDialog * self);
wxString *wxFileDialog_GetPath(const wxFileDialog * self);
void wxFileDialog_GetPaths(const wxFileDialog * self, wxArrayString * paths);
wxString *wxFileDialog_GetWildcard(const wxFileDialog * self);
#if wxCHECK_VERSION(3, 1, 7)
bool wxFileDialog_SetCustomizeHook(wxFileDialog * self, wxFileDialogCustomizeHook * customize_hook);
#endif
void wxFileDialog_SetDirectory(wxFileDialog * self, const wxString * directory);
void wxFileDialog_SetFilename(wxFileDialog * self, const wxString * setfilename);
void wxFileDialog_SetFilterIndex(wxFileDialog * self, int filter_index);
void wxFileDialog_SetMessage(wxFileDialog * self, const wxString * message);
void wxFileDialog_SetPath(wxFileDialog * self, const wxString * path);
void wxFileDialog_SetWildcard(wxFileDialog * self, const wxString * wild_card);

// CLASS: wxFileDirPickerEvent
wxClassInfo *wxFileDirPickerEvent_CLASSINFO();
wxFileDirPickerEvent *wxFileDirPickerEvent_new();
wxString *wxFileDirPickerEvent_GetPath(const wxFileDirPickerEvent * self);
void wxFileDirPickerEvent_SetPath(wxFileDirPickerEvent * self, const wxString * path);

// CLASS: wxFileDropTarget
void wxFileDropTarget_delete(wxFileDropTarget *self);
bool wxFileDropTarget_OnDropFiles(wxFileDropTarget * self, wxCoord x, wxCoord y, const wxArrayString * filenames);

// CLASS: wxFileHistory
wxClassInfo *wxFileHistory_CLASSINFO();
wxFileHistory *wxFileHistory_new(size_t max_files, wxWindowID id_base);
void wxFileHistory_AddFileToHistory(wxFileHistory * self, const wxString * filename);
void wxFileHistory_AddFilesToMenu(wxFileHistory * self);
void wxFileHistory_AddFilesToMenu1(wxFileHistory * self, wxMenu * menu);
wxWindowID wxFileHistory_GetBaseId(const wxFileHistory * self);
size_t wxFileHistory_GetCount(const wxFileHistory * self);
wxString *wxFileHistory_GetHistoryFile(const wxFileHistory * self, size_t index);
int wxFileHistory_GetMaxFiles(const wxFileHistory * self);
void wxFileHistory_Load(wxFileHistory * self, const wxConfigBase * config);
void wxFileHistory_RemoveFileFromHistory(wxFileHistory * self, size_t i);
void wxFileHistory_RemoveMenu(wxFileHistory * self, wxMenu * menu);
void wxFileHistory_Save(wxFileHistory * self, wxConfigBase * config);
void wxFileHistory_SetBaseId(wxFileHistory * self, wxWindowID base_id);
void wxFileHistory_UseMenu(wxFileHistory * self, wxMenu * menu);

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

// CLASS: wxFindDialogEvent
wxClassInfo *wxFindDialogEvent_CLASSINFO();
wxFindReplaceDialog * wxFindDialogEvent_GetDialog(const wxFindDialogEvent * self);
wxString *wxFindDialogEvent_GetFindString(const wxFindDialogEvent * self);
int wxFindDialogEvent_GetFlags(const wxFindDialogEvent * self);
wxString *wxFindDialogEvent_GetReplaceString(const wxFindDialogEvent * self);

// CLASS: wxFindReplaceData
wxClassInfo *wxFindReplaceData_CLASSINFO();
wxString *wxFindReplaceData_GetFindString(const wxFindReplaceData * self);
int wxFindReplaceData_GetFlags(const wxFindReplaceData * self);
wxString *wxFindReplaceData_GetReplaceString(const wxFindReplaceData * self);
void wxFindReplaceData_SetFindString(wxFindReplaceData * self, const wxString * str);
void wxFindReplaceData_SetReplaceString(wxFindReplaceData * self, const wxString * str);

// CLASS: wxFindReplaceDialog
wxClassInfo *wxFindReplaceDialog_CLASSINFO();
wxFindReplaceDialog *wxFindReplaceDialog_new();
wxFindReplaceDialog *wxFindReplaceDialog_new1(wxWindow * parent, wxFindReplaceData * data, const wxString * title, int style);
bool wxFindReplaceDialog_Create(wxFindReplaceDialog * self, wxWindow * parent, wxFindReplaceData * data, const wxString * title, int style);
const wxFindReplaceData * wxFindReplaceDialog_GetData(const wxFindReplaceDialog * self);

// CLASS: wxFlexGridSizer
wxClassInfo *wxFlexGridSizer_CLASSINFO();
wxFlexGridSizer *wxFlexGridSizer_new(int cols, int vgap, int hgap);
wxFlexGridSizer *wxFlexGridSizer_new1(int cols, const wxSize * gap);
wxFlexGridSizer *wxFlexGridSizer_new2(int rows, int cols, int vgap, int hgap);
wxFlexGridSizer *wxFlexGridSizer_new3(int rows, int cols, const wxSize * gap);
void wxFlexGridSizer_AddGrowableCol(wxFlexGridSizer * self, size_t idx, int proportion);
void wxFlexGridSizer_AddGrowableRow(wxFlexGridSizer * self, size_t idx, int proportion);
int wxFlexGridSizer_GetFlexibleDirection(const wxFlexGridSizer * self);
bool wxFlexGridSizer_IsColGrowable(wxFlexGridSizer * self, size_t idx);
bool wxFlexGridSizer_IsRowGrowable(wxFlexGridSizer * self, size_t idx);
void wxFlexGridSizer_RemoveGrowableCol(wxFlexGridSizer * self, size_t idx);
void wxFlexGridSizer_RemoveGrowableRow(wxFlexGridSizer * self, size_t idx);
void wxFlexGridSizer_SetFlexibleDirection(wxFlexGridSizer * self, int direction);
wxArrayInt *wxFlexGridSizer_GetRowHeights(const wxFlexGridSizer * self);
wxArrayInt *wxFlexGridSizer_GetColWidths(const wxFlexGridSizer * self);

// CLASS: wxFocusEvent
wxClassInfo *wxFocusEvent_CLASSINFO();
wxWindow * wxFocusEvent_GetWindow(const wxFocusEvent * self);
void wxFocusEvent_SetWindow(wxFocusEvent * self, wxWindow * win);

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

// CLASS: wxFontData
wxClassInfo *wxFontData_CLASSINFO();
wxFontData *wxFontData_new();
void wxFontData_EnableEffects(wxFontData * self, bool enable);
bool wxFontData_GetAllowSymbols(const wxFontData * self);
wxFont *wxFontData_GetChosenFont(const wxFontData * self);
wxColour *wxFontData_GetColour(const wxFontData * self);
bool wxFontData_GetEnableEffects(const wxFontData * self);
int wxFontData_GetRestrictSelection(const wxFontData * self);
wxFont *wxFontData_GetInitialFont(const wxFontData * self);
bool wxFontData_GetShowHelp(const wxFontData * self);
void wxFontData_RestrictSelection(wxFontData * self, int flags);
void wxFontData_SetAllowSymbols(wxFontData * self, bool allow_symbols);
void wxFontData_SetChosenFont(wxFontData * self, const wxFont * font);
void wxFontData_SetColour(wxFontData * self, const wxColour * colour);
void wxFontData_SetInitialFont(wxFontData * self, const wxFont * font);
void wxFontData_SetRange(wxFontData * self, int min, int max);
void wxFontData_SetShowHelp(wxFontData * self, bool show_help);

// CLASS: wxFontDialog
wxClassInfo *wxFontDialog_CLASSINFO();
wxFontDialog *wxFontDialog_new();
wxFontDialog *wxFontDialog_new2(wxWindow * parent, const wxFontData * data);
bool wxFontDialog_Create(wxFontDialog * self, wxWindow * parent);
bool wxFontDialog_Create1(wxFontDialog * self, wxWindow * parent, const wxFontData * data);
wxFontData * wxFontDialog_GetFontData1(wxFontDialog * self);

// CLASS: wxFontEnumerator
void wxFontEnumerator_delete(wxFontEnumerator *self);
wxFontEnumerator *wxFontEnumerator_new();
bool wxFontEnumerator_EnumerateEncodings(wxFontEnumerator * self, const wxString * font);
bool wxFontEnumerator_OnFacename(wxFontEnumerator * self, const wxString * font);
bool wxFontEnumerator_OnFontEncoding(wxFontEnumerator * self, const wxString * font, const wxString * encoding);
wxArrayString *wxFontEnumerator_GetEncodings(const wxString * facename);
bool wxFontEnumerator_IsValidFacename(const wxString * facename);
void wxFontEnumerator_InvalidateCache();

// CLASS: wxFontList
void wxFontList_delete(wxFontList *self);
wxFontList *wxFontList_new();
wxFont * wxFontList_FindOrCreateFont1(wxFontList * self, const wxFontInfo * font_info);

// CLASS: wxFontMapper
void wxFontMapper_delete(wxFontMapper *self);
wxFontMapper *wxFontMapper_new();
void wxFontMapper_SetConfigPath(wxFontMapper * self, const wxString * prefix);
void wxFontMapper_SetDialogParent(wxFontMapper * self, wxWindow * parent);
void wxFontMapper_SetDialogTitle(wxFontMapper * self, const wxString * title);
wxFontMapper * wxFontMapper_Get();
size_t wxFontMapper_GetSupportedEncodingsCount();
wxFontMapper * wxFontMapper_Set(wxFontMapper * mapper);

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

// CLASS: wxFontPickerEvent
wxClassInfo *wxFontPickerEvent_CLASSINFO();
wxFontPickerEvent *wxFontPickerEvent_new(wxObject * generator, int id, const wxFont * font);
wxFont *wxFontPickerEvent_GetFont(const wxFontPickerEvent * self);
void wxFontPickerEvent_SetFont(wxFontPickerEvent * self, const wxFont * f);

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

// CLASS: wxFullScreenEvent
wxClassInfo *wxFullScreenEvent_CLASSINFO();
wxFullScreenEvent *wxFullScreenEvent_new(int id, bool fullscreen);
bool wxFullScreenEvent_IsFullScreen(const wxFullScreenEvent * self);

} // extern "C"

