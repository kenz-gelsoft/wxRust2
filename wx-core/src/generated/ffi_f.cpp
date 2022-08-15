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

// CLASS: wxFileCtrlEvent
wxClassInfo *wxFileCtrlEvent_CLASSINFO() {
    return wxCLASSINFO(wxFileCtrlEvent);
}
wxString *wxFileCtrlEvent_GetDirectory(const wxFileCtrlEvent * self) {
    return new wxString(self->GetDirectory());
}
wxString *wxFileCtrlEvent_GetFile(const wxFileCtrlEvent * self) {
    return new wxString(self->GetFile());
}
wxArrayString *wxFileCtrlEvent_GetFiles(const wxFileCtrlEvent * self) {
    return new wxArrayString(self->GetFiles());
}
int wxFileCtrlEvent_GetFilterIndex(const wxFileCtrlEvent * self) {
    return self->GetFilterIndex();
}
void wxFileCtrlEvent_SetFiles(wxFileCtrlEvent * self, const wxArrayString * files) {
    return self->SetFiles(*files);
}
void wxFileCtrlEvent_SetDirectory(wxFileCtrlEvent * self, const wxString * directory) {
    return self->SetDirectory(*directory);
}
void wxFileCtrlEvent_SetFilterIndex(wxFileCtrlEvent * self, int index) {
    return self->SetFilterIndex(index);
}

// CLASS: wxFileDataObject
void wxFileDataObject_delete(wxFileDataObject *self) {
    delete self;
}
wxFileDataObject *wxFileDataObject_new() {
    return new wxFileDataObject();
}
void wxFileDataObject_AddFile(wxFileDataObject * self, const wxString * file) {
    return self->AddFile(*file);
}
wxArrayString *wxFileDataObject_GetFilenames(const wxFileDataObject * self) {
    return new wxArrayString(self->GetFilenames());
}

// CLASS: wxFileDialog
wxClassInfo *wxFileDialog_CLASSINFO() {
    return wxCLASSINFO(wxFileDialog);
}
wxFileDialog *wxFileDialog_new(wxWindow * parent, const wxString * message, const wxString * default_dir, const wxString * default_file, const wxString * wildcard, long style, const wxPoint * pos, const wxSize * size, const wxString * name) {
    return new wxFileDialog(parent, *message, *default_dir, *default_file, *wildcard, style, *pos, *size, *name);
}
wxString *wxFileDialog_GetCurrentlySelectedFilename(const wxFileDialog * self) {
    return new wxString(self->GetCurrentlySelectedFilename());
}
#if wxCHECK_VERSION(3, 1, 0)
int wxFileDialog_GetCurrentlySelectedFilterIndex(const wxFileDialog * self) {
    return self->GetCurrentlySelectedFilterIndex();
}
#endif
wxString *wxFileDialog_GetDirectory(const wxFileDialog * self) {
    return new wxString(self->GetDirectory());
}
wxWindow * wxFileDialog_GetExtraControl(const wxFileDialog * self) {
    return self->GetExtraControl();
}
wxString *wxFileDialog_GetFilename(const wxFileDialog * self) {
    return new wxString(self->GetFilename());
}
void wxFileDialog_GetFilenames(const wxFileDialog * self, wxArrayString * filenames) {
    return self->GetFilenames(*filenames);
}
int wxFileDialog_GetFilterIndex(const wxFileDialog * self) {
    return self->GetFilterIndex();
}
wxString *wxFileDialog_GetMessage(const wxFileDialog * self) {
    return new wxString(self->GetMessage());
}
wxString *wxFileDialog_GetPath(const wxFileDialog * self) {
    return new wxString(self->GetPath());
}
void wxFileDialog_GetPaths(const wxFileDialog * self, wxArrayString * paths) {
    return self->GetPaths(*paths);
}
wxString *wxFileDialog_GetWildcard(const wxFileDialog * self) {
    return new wxString(self->GetWildcard());
}
#if wxCHECK_VERSION(3, 1, 7)
bool wxFileDialog_SetCustomizeHook(wxFileDialog * self, wxFileDialogCustomizeHook * customize_hook) {
    return self->SetCustomizeHook(*customize_hook);
}
#endif
void wxFileDialog_SetDirectory(wxFileDialog * self, const wxString * directory) {
    return self->SetDirectory(*directory);
}
void wxFileDialog_SetFilename(wxFileDialog * self, const wxString * setfilename) {
    return self->SetFilename(*setfilename);
}
void wxFileDialog_SetFilterIndex(wxFileDialog * self, int filter_index) {
    return self->SetFilterIndex(filter_index);
}
void wxFileDialog_SetMessage(wxFileDialog * self, const wxString * message) {
    return self->SetMessage(*message);
}
void wxFileDialog_SetPath(wxFileDialog * self, const wxString * path) {
    return self->SetPath(*path);
}
void wxFileDialog_SetWildcard(wxFileDialog * self, const wxString * wild_card) {
    return self->SetWildcard(*wild_card);
}

// CLASS: wxFileDirPickerEvent
wxClassInfo *wxFileDirPickerEvent_CLASSINFO() {
    return wxCLASSINFO(wxFileDirPickerEvent);
}
wxFileDirPickerEvent *wxFileDirPickerEvent_new() {
    return new wxFileDirPickerEvent();
}
wxString *wxFileDirPickerEvent_GetPath(const wxFileDirPickerEvent * self) {
    return new wxString(self->GetPath());
}
void wxFileDirPickerEvent_SetPath(wxFileDirPickerEvent * self, const wxString * path) {
    return self->SetPath(*path);
}

// CLASS: wxFileDropTarget
void wxFileDropTarget_delete(wxFileDropTarget *self) {
    delete self;
}
bool wxFileDropTarget_OnDropFiles(wxFileDropTarget * self, wxCoord x, wxCoord y, const wxArrayString * filenames) {
    return self->OnDropFiles(x, y, *filenames);
}

// CLASS: wxFileHistory
wxClassInfo *wxFileHistory_CLASSINFO() {
    return wxCLASSINFO(wxFileHistory);
}
wxFileHistory *wxFileHistory_new(size_t max_files, wxWindowID id_base) {
    return new wxFileHistory(max_files, id_base);
}
void wxFileHistory_AddFileToHistory(wxFileHistory * self, const wxString * filename) {
    return self->AddFileToHistory(*filename);
}
void wxFileHistory_AddFilesToMenu(wxFileHistory * self) {
    return self->AddFilesToMenu();
}
void wxFileHistory_AddFilesToMenu1(wxFileHistory * self, wxMenu * menu) {
    return self->AddFilesToMenu(menu);
}
wxWindowID wxFileHistory_GetBaseId(const wxFileHistory * self) {
    return self->GetBaseId();
}
size_t wxFileHistory_GetCount(const wxFileHistory * self) {
    return self->GetCount();
}
wxString *wxFileHistory_GetHistoryFile(const wxFileHistory * self, size_t index) {
    return new wxString(self->GetHistoryFile(index));
}
int wxFileHistory_GetMaxFiles(const wxFileHistory * self) {
    return self->GetMaxFiles();
}
void wxFileHistory_Load(wxFileHistory * self, const wxConfigBase * config) {
    return self->Load(*config);
}
void wxFileHistory_RemoveFileFromHistory(wxFileHistory * self, size_t i) {
    return self->RemoveFileFromHistory(i);
}
void wxFileHistory_RemoveMenu(wxFileHistory * self, wxMenu * menu) {
    return self->RemoveMenu(menu);
}
void wxFileHistory_Save(wxFileHistory * self, wxConfigBase * config) {
    return self->Save(*config);
}
void wxFileHistory_SetBaseId(wxFileHistory * self, wxWindowID base_id) {
    return self->SetBaseId(base_id);
}
void wxFileHistory_UseMenu(wxFileHistory * self, wxMenu * menu) {
    return self->UseMenu(menu);
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

// CLASS: wxFindDialogEvent
wxClassInfo *wxFindDialogEvent_CLASSINFO() {
    return wxCLASSINFO(wxFindDialogEvent);
}
wxFindReplaceDialog * wxFindDialogEvent_GetDialog(const wxFindDialogEvent * self) {
    return self->GetDialog();
}
wxString *wxFindDialogEvent_GetFindString(const wxFindDialogEvent * self) {
    return new wxString(self->GetFindString());
}
int wxFindDialogEvent_GetFlags(const wxFindDialogEvent * self) {
    return self->GetFlags();
}
wxString *wxFindDialogEvent_GetReplaceString(const wxFindDialogEvent * self) {
    return new wxString(self->GetReplaceString());
}

// CLASS: wxFindReplaceData
wxClassInfo *wxFindReplaceData_CLASSINFO() {
    return wxCLASSINFO(wxFindReplaceData);
}
wxString *wxFindReplaceData_GetFindString(const wxFindReplaceData * self) {
    return new wxString(self->GetFindString());
}
int wxFindReplaceData_GetFlags(const wxFindReplaceData * self) {
    return self->GetFlags();
}
wxString *wxFindReplaceData_GetReplaceString(const wxFindReplaceData * self) {
    return new wxString(self->GetReplaceString());
}
void wxFindReplaceData_SetFindString(wxFindReplaceData * self, const wxString * str) {
    return self->SetFindString(*str);
}
void wxFindReplaceData_SetReplaceString(wxFindReplaceData * self, const wxString * str) {
    return self->SetReplaceString(*str);
}

// CLASS: wxFindReplaceDialog
wxClassInfo *wxFindReplaceDialog_CLASSINFO() {
    return wxCLASSINFO(wxFindReplaceDialog);
}
wxFindReplaceDialog *wxFindReplaceDialog_new() {
    return new wxFindReplaceDialog();
}
wxFindReplaceDialog *wxFindReplaceDialog_new1(wxWindow * parent, wxFindReplaceData * data, const wxString * title, int style) {
    return new wxFindReplaceDialog(parent, data, *title, style);
}
bool wxFindReplaceDialog_Create(wxFindReplaceDialog * self, wxWindow * parent, wxFindReplaceData * data, const wxString * title, int style) {
    return self->Create(parent, data, *title, style);
}
const wxFindReplaceData * wxFindReplaceDialog_GetData(const wxFindReplaceDialog * self) {
    return self->GetData();
}

// CLASS: wxFlexGridSizer
wxClassInfo *wxFlexGridSizer_CLASSINFO() {
    return wxCLASSINFO(wxFlexGridSizer);
}
wxFlexGridSizer *wxFlexGridSizer_new(int cols, int vgap, int hgap) {
    return new wxFlexGridSizer(cols, vgap, hgap);
}
wxFlexGridSizer *wxFlexGridSizer_new1(int cols, const wxSize * gap) {
    return new wxFlexGridSizer(cols, *gap);
}
wxFlexGridSizer *wxFlexGridSizer_new2(int rows, int cols, int vgap, int hgap) {
    return new wxFlexGridSizer(rows, cols, vgap, hgap);
}
wxFlexGridSizer *wxFlexGridSizer_new3(int rows, int cols, const wxSize * gap) {
    return new wxFlexGridSizer(rows, cols, *gap);
}
void wxFlexGridSizer_AddGrowableCol(wxFlexGridSizer * self, size_t idx, int proportion) {
    return self->AddGrowableCol(idx, proportion);
}
void wxFlexGridSizer_AddGrowableRow(wxFlexGridSizer * self, size_t idx, int proportion) {
    return self->AddGrowableRow(idx, proportion);
}
int wxFlexGridSizer_GetFlexibleDirection(const wxFlexGridSizer * self) {
    return self->GetFlexibleDirection();
}
bool wxFlexGridSizer_IsColGrowable(wxFlexGridSizer * self, size_t idx) {
    return self->IsColGrowable(idx);
}
bool wxFlexGridSizer_IsRowGrowable(wxFlexGridSizer * self, size_t idx) {
    return self->IsRowGrowable(idx);
}
void wxFlexGridSizer_RemoveGrowableCol(wxFlexGridSizer * self, size_t idx) {
    return self->RemoveGrowableCol(idx);
}
void wxFlexGridSizer_RemoveGrowableRow(wxFlexGridSizer * self, size_t idx) {
    return self->RemoveGrowableRow(idx);
}
void wxFlexGridSizer_SetFlexibleDirection(wxFlexGridSizer * self, int direction) {
    return self->SetFlexibleDirection(direction);
}
wxArrayInt *wxFlexGridSizer_GetRowHeights(const wxFlexGridSizer * self) {
    return new wxArrayInt(self->GetRowHeights());
}
wxArrayInt *wxFlexGridSizer_GetColWidths(const wxFlexGridSizer * self) {
    return new wxArrayInt(self->GetColWidths());
}

// CLASS: wxFocusEvent
wxClassInfo *wxFocusEvent_CLASSINFO() {
    return wxCLASSINFO(wxFocusEvent);
}
wxWindow * wxFocusEvent_GetWindow(const wxFocusEvent * self) {
    return self->GetWindow();
}
void wxFocusEvent_SetWindow(wxFocusEvent * self, wxWindow * win) {
    return self->SetWindow(win);
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

// CLASS: wxFontData
wxClassInfo *wxFontData_CLASSINFO() {
    return wxCLASSINFO(wxFontData);
}
wxFontData *wxFontData_new() {
    return new wxFontData();
}
void wxFontData_EnableEffects(wxFontData * self, bool enable) {
    return self->EnableEffects(enable);
}
bool wxFontData_GetAllowSymbols(const wxFontData * self) {
    return self->GetAllowSymbols();
}
wxFont *wxFontData_GetChosenFont(const wxFontData * self) {
    return new wxFont(self->GetChosenFont());
}
wxColour *wxFontData_GetColour(const wxFontData * self) {
    return new wxColour(self->GetColour());
}
bool wxFontData_GetEnableEffects(const wxFontData * self) {
    return self->GetEnableEffects();
}
#if wxCHECK_VERSION(3, 1, 0)
int wxFontData_GetRestrictSelection(const wxFontData * self) {
    return self->GetRestrictSelection();
}
#endif
wxFont *wxFontData_GetInitialFont(const wxFontData * self) {
    return new wxFont(self->GetInitialFont());
}
bool wxFontData_GetShowHelp(const wxFontData * self) {
    return self->GetShowHelp();
}
#if wxCHECK_VERSION(3, 1, 0)
void wxFontData_RestrictSelection(wxFontData * self, int flags) {
    return self->RestrictSelection(flags);
}
#endif
void wxFontData_SetAllowSymbols(wxFontData * self, bool allow_symbols) {
    return self->SetAllowSymbols(allow_symbols);
}
void wxFontData_SetChosenFont(wxFontData * self, const wxFont * font) {
    return self->SetChosenFont(*font);
}
void wxFontData_SetColour(wxFontData * self, const wxColour * colour) {
    return self->SetColour(*colour);
}
void wxFontData_SetInitialFont(wxFontData * self, const wxFont * font) {
    return self->SetInitialFont(*font);
}
void wxFontData_SetRange(wxFontData * self, int min, int max) {
    return self->SetRange(min, max);
}
void wxFontData_SetShowHelp(wxFontData * self, bool show_help) {
    return self->SetShowHelp(show_help);
}

// CLASS: wxFontDialog
wxClassInfo *wxFontDialog_CLASSINFO() {
    return wxCLASSINFO(wxFontDialog);
}
wxFontDialog *wxFontDialog_new() {
    return new wxFontDialog();
}
wxFontDialog *wxFontDialog_new2(wxWindow * parent, const wxFontData * data) {
    return new wxFontDialog(parent, *data);
}
bool wxFontDialog_Create(wxFontDialog * self, wxWindow * parent) {
    return self->Create(parent);
}
bool wxFontDialog_Create1(wxFontDialog * self, wxWindow * parent, const wxFontData * data) {
    return self->Create(parent, *data);
}
wxFontData * wxFontDialog_GetFontData1(wxFontDialog * self) {
    return &(self->GetFontData());
}

// CLASS: wxFontEnumerator
void wxFontEnumerator_delete(wxFontEnumerator *self) {
    delete self;
}
wxFontEnumerator *wxFontEnumerator_new() {
    return new wxFontEnumerator();
}
bool wxFontEnumerator_EnumerateEncodings(wxFontEnumerator * self, const wxString * font) {
    return self->EnumerateEncodings(*font);
}
bool wxFontEnumerator_OnFacename(wxFontEnumerator * self, const wxString * font) {
    return self->OnFacename(*font);
}
bool wxFontEnumerator_OnFontEncoding(wxFontEnumerator * self, const wxString * font, const wxString * encoding) {
    return self->OnFontEncoding(*font, *encoding);
}
wxArrayString *wxFontEnumerator_GetEncodings(const wxString * facename) {
    return new wxArrayString(wxFontEnumerator::GetEncodings(*facename));
}
bool wxFontEnumerator_IsValidFacename(const wxString * facename) {
    return wxFontEnumerator::IsValidFacename(*facename);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxFontEnumerator_InvalidateCache() {
    return wxFontEnumerator::InvalidateCache();
}
#endif

// CLASS: wxFontList
void wxFontList_delete(wxFontList *self) {
    delete self;
}
wxFontList *wxFontList_new() {
    return new wxFontList();
}
#if wxCHECK_VERSION(3, 1, 0)
wxFont * wxFontList_FindOrCreateFont1(wxFontList * self, const wxFontInfo * font_info) {
    return self->FindOrCreateFont(*font_info);
}
#endif

// CLASS: wxFontMapper
void wxFontMapper_delete(wxFontMapper *self) {
    delete self;
}
wxFontMapper *wxFontMapper_new() {
    return new wxFontMapper();
}
void wxFontMapper_SetConfigPath(wxFontMapper * self, const wxString * prefix) {
    return self->SetConfigPath(*prefix);
}
void wxFontMapper_SetDialogParent(wxFontMapper * self, wxWindow * parent) {
    return self->SetDialogParent(parent);
}
void wxFontMapper_SetDialogTitle(wxFontMapper * self, const wxString * title) {
    return self->SetDialogTitle(*title);
}
wxFontMapper * wxFontMapper_Get() {
    return wxFontMapper::Get();
}
size_t wxFontMapper_GetSupportedEncodingsCount() {
    return wxFontMapper::GetSupportedEncodingsCount();
}
wxFontMapper * wxFontMapper_Set(wxFontMapper * mapper) {
    return wxFontMapper::Set(mapper);
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

// CLASS: wxFontPickerEvent
wxClassInfo *wxFontPickerEvent_CLASSINFO() {
    return wxCLASSINFO(wxFontPickerEvent);
}
wxFontPickerEvent *wxFontPickerEvent_new(wxObject * generator, int id, const wxFont * font) {
    return new wxFontPickerEvent(generator, id, *font);
}
wxFont *wxFontPickerEvent_GetFont(const wxFontPickerEvent * self) {
    return new wxFont(self->GetFont());
}
void wxFontPickerEvent_SetFont(wxFontPickerEvent * self, const wxFont * f) {
    return self->SetFont(*f);
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

