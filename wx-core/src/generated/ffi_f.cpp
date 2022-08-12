#include "generated.h"

extern "C" {

// CLASS: wxFileCtrl
wxClassInfo *wxFileCtrl_CLASSINFO() {
    return wxCLASSINFO(wxFileCtrl);
}
wxFileCtrl *wxFileCtrl_new() {
    return new wxFileCtrl();
}
wxFileCtrl *wxFileCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * default_directory, const wxString * default_filename, const wxString * wild_card, long style, const wxPoint * pos, const wxSize * size, const wxString * name) {
    return new wxFileCtrl(parent, id, *default_directory, *default_filename, *wild_card, style, *pos, *size, *name);
}
bool wxFileCtrl_Create(wxFileCtrl * self, wxWindow * parent, wxWindowID id, const wxString * default_directory, const wxString * default_filename, const wxString * wild_card, long style, const wxPoint * pos, const wxSize * size, const wxString * name) {
    return self->Create(parent, id, *default_directory, *default_filename, *wild_card, style, *pos, *size, *name);
}
wxString *wxFileCtrl_GetDirectory(const wxFileCtrl * self) {
    return new wxString(self->GetDirectory());
}
wxString *wxFileCtrl_GetFilename(const wxFileCtrl * self) {
    return new wxString(self->GetFilename());
}
void wxFileCtrl_GetFilenames(const wxFileCtrl * self, wxArrayString * filenames) {
    return self->GetFilenames(*filenames);
}
int wxFileCtrl_GetFilterIndex(const wxFileCtrl * self) {
    return self->GetFilterIndex();
}
wxString *wxFileCtrl_GetPath(const wxFileCtrl * self) {
    return new wxString(self->GetPath());
}
void wxFileCtrl_GetPaths(const wxFileCtrl * self, wxArrayString * paths) {
    return self->GetPaths(*paths);
}
wxString *wxFileCtrl_GetWildcard(const wxFileCtrl * self) {
    return new wxString(self->GetWildcard());
}
bool wxFileCtrl_SetDirectory(wxFileCtrl * self, const wxString * directory) {
    return self->SetDirectory(*directory);
}
bool wxFileCtrl_SetFilename(wxFileCtrl * self, const wxString * filename) {
    return self->SetFilename(*filename);
}
bool wxFileCtrl_SetPath(wxFileCtrl * self, const wxString * path) {
    return self->SetPath(*path);
}
void wxFileCtrl_SetFilterIndex(wxFileCtrl * self, int filter_index) {
    return self->SetFilterIndex(filter_index);
}
void wxFileCtrl_SetWildcard(wxFileCtrl * self, const wxString * wild_card) {
    return self->SetWildcard(*wild_card);
}
void wxFileCtrl_ShowHidden(wxFileCtrl * self, bool show) {
    return self->ShowHidden(show);
}

// CLASS: wxFilePickerCtrl
wxClassInfo *wxFilePickerCtrl_CLASSINFO() {
    return wxCLASSINFO(wxFilePickerCtrl);
}
wxFilePickerCtrl *wxFilePickerCtrl_new() {
    return new wxFilePickerCtrl();
}
wxFilePickerCtrl *wxFilePickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxString * wildcard, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxFilePickerCtrl(parent, id, *path, *message, *wildcard, *pos, *size, style, *validator, *name);
}
bool wxFilePickerCtrl_Create(wxFilePickerCtrl * self, wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxString * wildcard, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *path, *message, *wildcard, *pos, *size, style, *validator, *name);
}
wxFileName *wxFilePickerCtrl_GetFileName(const wxFilePickerCtrl * self) {
    return new wxFileName(self->GetFileName());
}
wxString *wxFilePickerCtrl_GetPath(const wxFilePickerCtrl * self) {
    return new wxString(self->GetPath());
}
void wxFilePickerCtrl_SetFileName(wxFilePickerCtrl * self, const wxFileName * filename) {
    return self->SetFileName(*filename);
}
void wxFilePickerCtrl_SetInitialDirectory(wxFilePickerCtrl * self, const wxString * dir) {
    return self->SetInitialDirectory(*dir);
}
void wxFilePickerCtrl_SetPath(wxFilePickerCtrl * self, const wxString * filename) {
    return self->SetPath(*filename);
}

// CLASS: wxFont
wxClassInfo *wxFont_CLASSINFO() {
    return wxCLASSINFO(wxFont);
}
#if wxCHECK_VERSION(3, 1, 0)
wxFont *wxFont_GetBaseFont(const wxFont * self) {
    return new wxFont(self->GetBaseFont());
}
#endif
wxString *wxFont_GetFaceName(const wxFont * self) {
    return new wxString(self->GetFaceName());
}
wxString *wxFont_GetNativeFontInfoDesc(const wxFont * self) {
    return new wxString(self->GetNativeFontInfoDesc());
}
wxString *wxFont_GetNativeFontInfoUserDesc(const wxFont * self) {
    return new wxString(self->GetNativeFontInfoUserDesc());
}
const wxNativeFontInfo * wxFont_GetNativeFontInfo(const wxFont * self) {
    return self->GetNativeFontInfo();
}
int wxFont_GetPointSize(const wxFont * self) {
    return self->GetPointSize();
}
#if wxCHECK_VERSION(3, 1, 0)
double wxFont_GetFractionalPointSize(const wxFont * self) {
    return self->GetFractionalPointSize();
}
#endif
wxSize *wxFont_GetPixelSize(const wxFont * self) {
    return new wxSize(self->GetPixelSize());
}
bool wxFont_GetUnderlined(const wxFont * self) {
    return self->GetUnderlined();
}
bool wxFont_GetStrikethrough(const wxFont * self) {
    return self->GetStrikethrough();
}
#if wxCHECK_VERSION(3, 1, 0)
int wxFont_GetNumericWeight(const wxFont * self) {
    return self->GetNumericWeight();
}
#endif
bool wxFont_IsFixedWidth(const wxFont * self) {
    return self->IsFixedWidth();
}
bool wxFont_IsOk(const wxFont * self) {
    return self->IsOk();
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxFont_AddPrivateFont(const wxString * filename) {
    return wxFont::AddPrivateFont(*filename);
}
#endif
wxFont *wxFont_Bold(const wxFont * self) {
    return new wxFont(self->Bold());
}
wxFont *wxFont_Italic(const wxFont * self) {
    return new wxFont(self->Italic());
}
wxFont *wxFont_Larger(const wxFont * self) {
    return new wxFont(self->Larger());
}
wxFont *wxFont_Smaller(const wxFont * self) {
    return new wxFont(self->Smaller());
}
wxFont *wxFont_Underlined(const wxFont * self) {
    return new wxFont(self->Underlined());
}
wxFont *wxFont_Strikethrough(const wxFont * self) {
    return new wxFont(self->Strikethrough());
}
wxFont * wxFont_MakeBold(wxFont * self) {
    return &(self->MakeBold());
}
wxFont * wxFont_MakeItalic(wxFont * self) {
    return &(self->MakeItalic());
}
wxFont * wxFont_MakeLarger(wxFont * self) {
    return &(self->MakeLarger());
}
wxFont * wxFont_MakeSmaller(wxFont * self) {
    return &(self->MakeSmaller());
}
wxFont * wxFont_MakeUnderlined(wxFont * self) {
    return &(self->MakeUnderlined());
}
wxFont * wxFont_MakeStrikethrough(wxFont * self) {
    return &(self->MakeStrikethrough());
}
bool wxFont_SetFaceName(wxFont * self, const wxString * face_name) {
    return self->SetFaceName(*face_name);
}
bool wxFont_SetNativeFontInfo(wxFont * self, const wxString * info) {
    return self->SetNativeFontInfo(*info);
}
bool wxFont_SetNativeFontInfoUserDesc(wxFont * self, const wxString * info) {
    return self->SetNativeFontInfoUserDesc(*info);
}
void wxFont_SetNativeFontInfo1(wxFont * self, const wxNativeFontInfo * info) {
    return self->SetNativeFontInfo(*info);
}
void wxFont_SetPointSize(wxFont * self, int point_size) {
    return self->SetPointSize(point_size);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxFont_SetFractionalPointSize(wxFont * self, double point_size) {
    return self->SetFractionalPointSize(point_size);
}
#endif
void wxFont_SetPixelSize(wxFont * self, const wxSize * pixel_size) {
    return self->SetPixelSize(*pixel_size);
}
void wxFont_SetUnderlined(wxFont * self, bool underlined) {
    return self->SetUnderlined(underlined);
}
void wxFont_SetStrikethrough(wxFont * self, bool strikethrough) {
    return self->SetStrikethrough(strikethrough);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxFont_SetNumericWeight(wxFont * self, int weight) {
    return self->SetNumericWeight(weight);
}
#endif
wxFont * wxFont_New4(const wxNativeFontInfo * native_info) {
    return wxFont::New(*native_info);
}
wxFont * wxFont_New5(const wxString * native_info_string) {
    return wxFont::New(*native_info_string);
}
wxFont *wxFont_new() {
    return new wxFont();
}
wxFont *wxFont_new1(const wxFont * font) {
    return new wxFont(*font);
}
wxFont *wxFont_new2(const wxFontInfo * font_info) {
    return new wxFont(*font_info);
}
wxFont *wxFont_new5(const wxString * native_info_string) {
    return new wxFont(*native_info_string);
}
wxFont *wxFont_new6(const wxNativeFontInfo * native_info) {
    return new wxFont(*native_info);
}

// CLASS: wxFontPickerCtrl
wxClassInfo *wxFontPickerCtrl_CLASSINFO() {
    return wxCLASSINFO(wxFontPickerCtrl);
}
wxFontPickerCtrl *wxFontPickerCtrl_new() {
    return new wxFontPickerCtrl();
}
wxFontPickerCtrl *wxFontPickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxFont * font, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxFontPickerCtrl(parent, id, *font, *pos, *size, style, *validator, *name);
}
bool wxFontPickerCtrl_Create(wxFontPickerCtrl * self, wxWindow * parent, wxWindowID id, const wxFont * font, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *font, *pos, *size, style, *validator, *name);
}
unsigned int wxFontPickerCtrl_GetMaxPointSize(const wxFontPickerCtrl * self) {
    return self->GetMaxPointSize();
}
#if wxCHECK_VERSION(3, 1, 0)
unsigned int wxFontPickerCtrl_GetMinPointSize(const wxFontPickerCtrl * self) {
    return self->GetMinPointSize();
}
wxColour *wxFontPickerCtrl_GetSelectedColour(const wxFontPickerCtrl * self) {
    return new wxColour(self->GetSelectedColour());
}
#endif
wxFont *wxFontPickerCtrl_GetSelectedFont(const wxFontPickerCtrl * self) {
    return new wxFont(self->GetSelectedFont());
}
void wxFontPickerCtrl_SetMaxPointSize(wxFontPickerCtrl * self, unsigned int max) {
    return self->SetMaxPointSize(max);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxFontPickerCtrl_SetMinPointSize(wxFontPickerCtrl * self, unsigned int min) {
    return self->SetMinPointSize(min);
}
void wxFontPickerCtrl_SetSelectedColour(wxFontPickerCtrl * self, const wxColour * colour) {
    return self->SetSelectedColour(*colour);
}
#endif
void wxFontPickerCtrl_SetSelectedFont(wxFontPickerCtrl * self, const wxFont * font) {
    return self->SetSelectedFont(*font);
}

// CLASS: wxFrame
wxClassInfo *wxFrame_CLASSINFO() {
    return wxCLASSINFO(wxFrame);
}
wxFrame *wxFrame_new() {
    return new wxFrame();
}
wxFrame *wxFrame_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxFrame(parent, id, *title, *pos, *size, style, *name);
}
void wxFrame_Centre(wxFrame * self, int direction) {
    return self->Centre(direction);
}
bool wxFrame_Create(wxFrame * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *title, *pos, *size, style, *name);
}
wxStatusBar * wxFrame_CreateStatusBar(wxFrame * self, int number, long style, wxWindowID id, const wxString * name) {
    return self->CreateStatusBar(number, style, id, *name);
}
wxToolBar * wxFrame_CreateToolBar(wxFrame * self, long style, wxWindowID id, const wxString * name) {
    return self->CreateToolBar(style, id, *name);
}
void wxFrame_DoGiveHelp(wxFrame * self, const wxString * text, bool show) {
    return self->DoGiveHelp(*text, show);
}
wxMenuBar * wxFrame_GetMenuBar(const wxFrame * self) {
    return self->GetMenuBar();
}
wxStatusBar * wxFrame_GetStatusBar(const wxFrame * self) {
    return self->GetStatusBar();
}
int wxFrame_GetStatusBarPane(const wxFrame * self) {
    return self->GetStatusBarPane();
}
wxToolBar * wxFrame_GetToolBar(const wxFrame * self) {
    return self->GetToolBar();
}
wxStatusBar * wxFrame_OnCreateStatusBar(wxFrame * self, int number, long style, wxWindowID id, const wxString * name) {
    return self->OnCreateStatusBar(number, style, id, *name);
}
wxToolBar * wxFrame_OnCreateToolBar(wxFrame * self, long style, wxWindowID id, const wxString * name) {
    return self->OnCreateToolBar(style, id, *name);
}
bool wxFrame_ProcessCommand(wxFrame * self, int id) {
    return self->ProcessCommand(id);
}
void wxFrame_SetMenuBar(wxFrame * self, wxMenuBar * menu_bar) {
    return self->SetMenuBar(menu_bar);
}
void wxFrame_SetStatusBar(wxFrame * self, wxStatusBar * status_bar) {
    return self->SetStatusBar(status_bar);
}
void wxFrame_SetStatusBarPane(wxFrame * self, int n) {
    return self->SetStatusBarPane(n);
}
void wxFrame_SetStatusText(wxFrame * self, const wxString * text, int number) {
    return self->SetStatusText(*text, number);
}
void wxFrame_SetStatusWidths(wxFrame * self, int n, const int * widths_field) {
    return self->SetStatusWidths(n, widths_field);
}
void wxFrame_SetToolBar(wxFrame * self, wxToolBar * tool_bar) {
    return self->SetToolBar(tool_bar);
}
#ifdef __WXMSW__
wxTaskBarButton * wxFrame_MSWGetTaskBarButton(wxFrame * self) {
    return self->MSWGetTaskBarButton();
}
#endif
void wxFrame_PushStatusText(wxFrame * self, const wxString * text, int number) {
    return self->PushStatusText(*text, number);
}
void wxFrame_PopStatusText(wxFrame * self, int number) {
    return self->PopStatusText(number);
}

} // extern "C"

