#include "generated.h"

extern "C" {

// CLASS: wxTextAttr
void wxTextAttr_delete(wxTextAttr *self) {
    delete self;
}
wxColour *wxTextAttr_GetBackgroundColour(const wxTextAttr * self) {
    return new wxColour(self->GetBackgroundColour());
}
wxString *wxTextAttr_GetBulletFont(const wxTextAttr * self) {
    return new wxString(self->GetBulletFont());
}
wxString *wxTextAttr_GetBulletName(const wxTextAttr * self) {
    return new wxString(self->GetBulletName());
}
int wxTextAttr_GetBulletNumber(const wxTextAttr * self) {
    return self->GetBulletNumber();
}
int wxTextAttr_GetBulletStyle(const wxTextAttr * self) {
    return self->GetBulletStyle();
}
wxString *wxTextAttr_GetBulletText(const wxTextAttr * self) {
    return new wxString(self->GetBulletText());
}
wxString *wxTextAttr_GetCharacterStyleName(const wxTextAttr * self) {
    return new wxString(self->GetCharacterStyleName());
}
long wxTextAttr_GetFlags(const wxTextAttr * self) {
    return self->GetFlags();
}
wxFont *wxTextAttr_GetFont(const wxTextAttr * self) {
    return new wxFont(self->GetFont());
}
bool wxTextAttr_GetFontAttributes(wxTextAttr * self, const wxFont * font, int flags) {
    return self->GetFontAttributes(*font, flags);
}
wxString *wxTextAttr_GetFontFaceName(const wxTextAttr * self) {
    return new wxString(self->GetFontFaceName());
}
int wxTextAttr_GetFontSize(const wxTextAttr * self) {
    return self->GetFontSize();
}
bool wxTextAttr_GetFontUnderlined(const wxTextAttr * self) {
    return self->GetFontUnderlined();
}
#if wxCHECK_VERSION(3, 1, 0)
wxColour *wxTextAttr_GetUnderlineColour(const wxTextAttr * self) {
    return new wxColour(self->GetUnderlineColour());
}
#endif
long wxTextAttr_GetLeftIndent(const wxTextAttr * self) {
    return self->GetLeftIndent();
}
long wxTextAttr_GetLeftSubIndent(const wxTextAttr * self) {
    return self->GetLeftSubIndent();
}
int wxTextAttr_GetLineSpacing(const wxTextAttr * self) {
    return self->GetLineSpacing();
}
wxString *wxTextAttr_GetListStyleName(const wxTextAttr * self) {
    return new wxString(self->GetListStyleName());
}
int wxTextAttr_GetOutlineLevel(const wxTextAttr * self) {
    return self->GetOutlineLevel();
}
int wxTextAttr_GetParagraphSpacingAfter(const wxTextAttr * self) {
    return self->GetParagraphSpacingAfter();
}
int wxTextAttr_GetParagraphSpacingBefore(const wxTextAttr * self) {
    return self->GetParagraphSpacingBefore();
}
wxString *wxTextAttr_GetParagraphStyleName(const wxTextAttr * self) {
    return new wxString(self->GetParagraphStyleName());
}
long wxTextAttr_GetRightIndent(const wxTextAttr * self) {
    return self->GetRightIndent();
}
wxArrayInt *wxTextAttr_GetTabs(const wxTextAttr * self) {
    return new wxArrayInt(self->GetTabs());
}
wxColour *wxTextAttr_GetTextColour(const wxTextAttr * self) {
    return new wxColour(self->GetTextColour());
}
int wxTextAttr_GetTextEffectFlags(const wxTextAttr * self) {
    return self->GetTextEffectFlags();
}
int wxTextAttr_GetTextEffects(const wxTextAttr * self) {
    return self->GetTextEffects();
}
wxString *wxTextAttr_GetURL(const wxTextAttr * self) {
    return new wxString(self->GetURL());
}
bool wxTextAttr_HasAlignment(const wxTextAttr * self) {
    return self->HasAlignment();
}
bool wxTextAttr_HasBackgroundColour(const wxTextAttr * self) {
    return self->HasBackgroundColour();
}
bool wxTextAttr_HasBulletName(const wxTextAttr * self) {
    return self->HasBulletName();
}
bool wxTextAttr_HasBulletNumber(const wxTextAttr * self) {
    return self->HasBulletNumber();
}
bool wxTextAttr_HasBulletStyle(const wxTextAttr * self) {
    return self->HasBulletStyle();
}
bool wxTextAttr_HasBulletText(const wxTextAttr * self) {
    return self->HasBulletText();
}
bool wxTextAttr_HasCharacterStyleName(const wxTextAttr * self) {
    return self->HasCharacterStyleName();
}
bool wxTextAttr_HasFlag(const wxTextAttr * self, long flag) {
    return self->HasFlag(flag);
}
bool wxTextAttr_HasFont(const wxTextAttr * self) {
    return self->HasFont();
}
bool wxTextAttr_HasFontEncoding(const wxTextAttr * self) {
    return self->HasFontEncoding();
}
bool wxTextAttr_HasFontFaceName(const wxTextAttr * self) {
    return self->HasFontFaceName();
}
bool wxTextAttr_HasFontFamily(const wxTextAttr * self) {
    return self->HasFontFamily();
}
bool wxTextAttr_HasFontItalic(const wxTextAttr * self) {
    return self->HasFontItalic();
}
bool wxTextAttr_HasFontSize(const wxTextAttr * self) {
    return self->HasFontSize();
}
bool wxTextAttr_HasFontPointSize(const wxTextAttr * self) {
    return self->HasFontPointSize();
}
bool wxTextAttr_HasFontPixelSize(const wxTextAttr * self) {
    return self->HasFontPixelSize();
}
bool wxTextAttr_HasFontUnderlined(const wxTextAttr * self) {
    return self->HasFontUnderlined();
}
bool wxTextAttr_HasFontWeight(const wxTextAttr * self) {
    return self->HasFontWeight();
}
bool wxTextAttr_HasLeftIndent(const wxTextAttr * self) {
    return self->HasLeftIndent();
}
bool wxTextAttr_HasLineSpacing(const wxTextAttr * self) {
    return self->HasLineSpacing();
}
bool wxTextAttr_HasListStyleName(const wxTextAttr * self) {
    return self->HasListStyleName();
}
bool wxTextAttr_HasOutlineLevel(const wxTextAttr * self) {
    return self->HasOutlineLevel();
}
bool wxTextAttr_HasPageBreak(const wxTextAttr * self) {
    return self->HasPageBreak();
}
bool wxTextAttr_HasParagraphSpacingAfter(const wxTextAttr * self) {
    return self->HasParagraphSpacingAfter();
}
bool wxTextAttr_HasParagraphSpacingBefore(const wxTextAttr * self) {
    return self->HasParagraphSpacingBefore();
}
bool wxTextAttr_HasParagraphStyleName(const wxTextAttr * self) {
    return self->HasParagraphStyleName();
}
bool wxTextAttr_HasRightIndent(const wxTextAttr * self) {
    return self->HasRightIndent();
}
bool wxTextAttr_HasTabs(const wxTextAttr * self) {
    return self->HasTabs();
}
bool wxTextAttr_HasTextColour(const wxTextAttr * self) {
    return self->HasTextColour();
}
bool wxTextAttr_HasTextEffects(const wxTextAttr * self) {
    return self->HasTextEffects();
}
bool wxTextAttr_HasURL(const wxTextAttr * self) {
    return self->HasURL();
}
bool wxTextAttr_IsCharacterStyle(const wxTextAttr * self) {
    return self->IsCharacterStyle();
}
bool wxTextAttr_IsDefault(const wxTextAttr * self) {
    return self->IsDefault();
}
bool wxTextAttr_IsParagraphStyle(const wxTextAttr * self) {
    return self->IsParagraphStyle();
}
void wxTextAttr_SetBackgroundColour(wxTextAttr * self, const wxColour * col_back) {
    return self->SetBackgroundColour(*col_back);
}
void wxTextAttr_SetBulletFont(wxTextAttr * self, const wxString * font) {
    return self->SetBulletFont(*font);
}
void wxTextAttr_SetBulletName(wxTextAttr * self, const wxString * name) {
    return self->SetBulletName(*name);
}
void wxTextAttr_SetBulletNumber(wxTextAttr * self, int n) {
    return self->SetBulletNumber(n);
}
void wxTextAttr_SetBulletStyle(wxTextAttr * self, int style) {
    return self->SetBulletStyle(style);
}
void wxTextAttr_SetBulletText(wxTextAttr * self, const wxString * text) {
    return self->SetBulletText(*text);
}
void wxTextAttr_SetCharacterStyleName(wxTextAttr * self, const wxString * name) {
    return self->SetCharacterStyleName(*name);
}
void wxTextAttr_SetFlags(wxTextAttr * self, long flags) {
    return self->SetFlags(flags);
}
void wxTextAttr_SetFont(wxTextAttr * self, const wxFont * font, int flags) {
    return self->SetFont(*font, flags);
}
void wxTextAttr_SetFontFaceName(wxTextAttr * self, const wxString * face_name) {
    return self->SetFontFaceName(*face_name);
}
void wxTextAttr_SetFontSize(wxTextAttr * self, int point_size) {
    return self->SetFontSize(point_size);
}
void wxTextAttr_SetFontPointSize(wxTextAttr * self, int point_size) {
    return self->SetFontPointSize(point_size);
}
void wxTextAttr_SetFontPixelSize(wxTextAttr * self, int pixel_size) {
    return self->SetFontPixelSize(pixel_size);
}
void wxTextAttr_SetFontUnderlined(wxTextAttr * self, bool underlined) {
    return self->SetFontUnderlined(underlined);
}
void wxTextAttr_SetLeftIndent(wxTextAttr * self, int indent, int sub_indent) {
    return self->SetLeftIndent(indent, sub_indent);
}
void wxTextAttr_SetLineSpacing(wxTextAttr * self, int spacing) {
    return self->SetLineSpacing(spacing);
}
void wxTextAttr_SetListStyleName(wxTextAttr * self, const wxString * name) {
    return self->SetListStyleName(*name);
}
void wxTextAttr_SetOutlineLevel(wxTextAttr * self, int level) {
    return self->SetOutlineLevel(level);
}
void wxTextAttr_SetPageBreak(wxTextAttr * self, bool page_break) {
    return self->SetPageBreak(page_break);
}
void wxTextAttr_SetParagraphSpacingAfter(wxTextAttr * self, int spacing) {
    return self->SetParagraphSpacingAfter(spacing);
}
void wxTextAttr_SetParagraphSpacingBefore(wxTextAttr * self, int spacing) {
    return self->SetParagraphSpacingBefore(spacing);
}
void wxTextAttr_SetParagraphStyleName(wxTextAttr * self, const wxString * name) {
    return self->SetParagraphStyleName(*name);
}
void wxTextAttr_SetRightIndent(wxTextAttr * self, int indent) {
    return self->SetRightIndent(indent);
}
void wxTextAttr_SetTabs(wxTextAttr * self, const wxArrayInt * tabs) {
    return self->SetTabs(*tabs);
}
void wxTextAttr_SetTextColour(wxTextAttr * self, const wxColour * col_text) {
    return self->SetTextColour(*col_text);
}
void wxTextAttr_SetTextEffectFlags(wxTextAttr * self, int flags) {
    return self->SetTextEffectFlags(flags);
}
void wxTextAttr_SetTextEffects(wxTextAttr * self, int effects) {
    return self->SetTextEffects(effects);
}
void wxTextAttr_SetURL(wxTextAttr * self, const wxString * url) {
    return self->SetURL(*url);
}
wxTextAttr *wxTextAttr_new() {
    return new wxTextAttr();
}
wxTextAttr *wxTextAttr_new2(const wxTextAttr * attr) {
    return new wxTextAttr(*attr);
}
bool wxTextAttr_Apply(wxTextAttr * self, const wxTextAttr * style, const wxTextAttr * compare_with) {
    return self->Apply(*style, compare_with);
}
void wxTextAttr_Merge(wxTextAttr * self, const wxTextAttr * overlay) {
    return self->Merge(*overlay);
}
bool wxTextAttr_EqPartial(const wxTextAttr * self, const wxTextAttr * attr, bool weak_test) {
    return self->EqPartial(*attr, weak_test);
}
wxTextAttr *wxTextAttr_Merge1(const wxTextAttr * base, const wxTextAttr * overlay) {
    return new wxTextAttr(wxTextAttr::Merge(*base, *overlay));
}

// CLASS: wxTextCtrl
wxClassInfo *wxTextCtrl_CLASSINFO() {
    return wxCLASSINFO(wxTextCtrl);
}
#ifdef __WXOSX__
void wxTextCtrl_OSXEnableNewLineReplacement(wxTextCtrl * self, bool enable) {
    return self->OSXEnableNewLineReplacement(enable);
}
#endif
wxTextCtrl *wxTextCtrl_new() {
    return new wxTextCtrl();
}
wxTextCtrl *wxTextCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxTextCtrl(parent, id, *value, *pos, *size, style, *validator, *name);
}
bool wxTextCtrl_Create(wxTextCtrl * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *value, *pos, *size, style, *validator, *name);
}
void wxTextCtrl_DiscardEdits(wxTextCtrl * self) {
    return self->DiscardEdits();
}
#if wxCHECK_VERSION(3, 1, 0)
void wxTextCtrl_EmptyUndoBuffer(wxTextCtrl * self) {
    return self->EmptyUndoBuffer();
}
#endif
bool wxTextCtrl_EmulateKeyPress(wxTextCtrl * self, const wxKeyEvent * event) {
    return self->EmulateKeyPress(*event);
}
#ifndef __WXGTK__
bool wxTextCtrl_EnableProofCheck(wxTextCtrl * self, const wxTextProofOptions * options) {
    return self->EnableProofCheck(*options);
}
#endif
wxTextAttr *wxTextCtrl_GetDefaultStyle(const wxTextCtrl * self) {
    return new wxTextAttr(self->GetDefaultStyle());
}
int wxTextCtrl_GetLineLength(const wxTextCtrl * self, long line_no) {
    return self->GetLineLength(line_no);
}
wxString *wxTextCtrl_GetLineText(const wxTextCtrl * self, long line_no) {
    return new wxString(self->GetLineText(line_no));
}
int wxTextCtrl_GetNumberOfLines(const wxTextCtrl * self) {
    return self->GetNumberOfLines();
}
bool wxTextCtrl_GetStyle(wxTextCtrl * self, long position, wxTextAttr * style) {
    return self->GetStyle(position, *style);
}
bool wxTextCtrl_IsModified(const wxTextCtrl * self) {
    return self->IsModified();
}
bool wxTextCtrl_IsMultiLine(const wxTextCtrl * self) {
    return self->IsMultiLine();
}
bool wxTextCtrl_IsSingleLine(const wxTextCtrl * self) {
    return self->IsSingleLine();
}
bool wxTextCtrl_LoadFile(wxTextCtrl * self, const wxString * filename, int file_type) {
    return self->LoadFile(*filename, file_type);
}
void wxTextCtrl_MarkDirty(wxTextCtrl * self) {
    return self->MarkDirty();
}
void wxTextCtrl_OnDropFiles(wxTextCtrl * self, wxDropFilesEvent * event) {
    return self->OnDropFiles(*event);
}
bool wxTextCtrl_PositionToXY(const wxTextCtrl * self, long pos, long * x, long * y) {
    return self->PositionToXY(pos, x, y);
}
wxPoint *wxTextCtrl_PositionToCoords(const wxTextCtrl * self, long pos) {
    return new wxPoint(self->PositionToCoords(pos));
}
bool wxTextCtrl_SaveFile(wxTextCtrl * self, const wxString * filename, int file_type) {
    return self->SaveFile(*filename, file_type);
}
bool wxTextCtrl_SetDefaultStyle(wxTextCtrl * self, const wxTextAttr * style) {
    return self->SetDefaultStyle(*style);
}
void wxTextCtrl_SetModified(wxTextCtrl * self, bool modified) {
    return self->SetModified(modified);
}
bool wxTextCtrl_SetStyle(wxTextCtrl * self, long start, long end, const wxTextAttr * style) {
    return self->SetStyle(start, end, *style);
}
void wxTextCtrl_ShowPosition(wxTextCtrl * self, long pos) {
    return self->ShowPosition(pos);
}
long wxTextCtrl_XYToPosition(const wxTextCtrl * self, long x, long y) {
    return self->XYToPosition(x, y);
}
// Mix-in(s) to wxTextCtrl
wxTextEntryBase *wxTextCtrl_AsTextEntry(wxTextCtrl* obj) {
    return static_cast<wxTextEntryBase*>(obj);
}

// CLASS: wxTextEntry
void wxTextEntry_delete(wxTextEntry *self) {
    delete self;
}
void wxTextEntry_AppendText(wxTextEntry * self, const wxString * text) {
    return self->AppendText(*text);
}
bool wxTextEntry_AutoComplete(wxTextEntry * self, const wxArrayString * choices) {
    return self->AutoComplete(*choices);
}
bool wxTextEntry_AutoComplete1(wxTextEntry * self, wxTextCompleter * completer) {
    return self->AutoComplete(completer);
}
bool wxTextEntry_AutoCompleteFileNames(wxTextEntry * self) {
    return self->AutoCompleteFileNames();
}
bool wxTextEntry_AutoCompleteDirectories(wxTextEntry * self) {
    return self->AutoCompleteDirectories();
}
bool wxTextEntry_CanCopy(const wxTextEntry * self) {
    return self->CanCopy();
}
bool wxTextEntry_CanCut(const wxTextEntry * self) {
    return self->CanCut();
}
bool wxTextEntry_CanPaste(const wxTextEntry * self) {
    return self->CanPaste();
}
bool wxTextEntry_CanRedo(const wxTextEntry * self) {
    return self->CanRedo();
}
bool wxTextEntry_CanUndo(const wxTextEntry * self) {
    return self->CanUndo();
}
void wxTextEntry_ChangeValue(wxTextEntry * self, const wxString * value) {
    return self->ChangeValue(*value);
}
void wxTextEntry_Clear(wxTextEntry * self) {
    return self->Clear();
}
void wxTextEntry_Copy(wxTextEntry * self) {
    return self->Copy();
}
void wxTextEntry_Cut(wxTextEntry * self) {
    return self->Cut();
}
#if wxCHECK_VERSION(3, 1, 0)
void wxTextEntry_ForceUpper(wxTextEntry * self) {
    return self->ForceUpper();
}
#endif
long wxTextEntry_GetInsertionPoint(const wxTextEntry * self) {
    return self->GetInsertionPoint();
}
wxString *wxTextEntry_GetRange(const wxTextEntry * self, long from, long to) {
    return new wxString(self->GetRange(from, to));
}
void wxTextEntry_GetSelection(const wxTextEntry * self, long * from, long * to) {
    return self->GetSelection(from, to);
}
wxString *wxTextEntry_GetStringSelection(const wxTextEntry * self) {
    return new wxString(self->GetStringSelection());
}
wxString *wxTextEntry_GetValue(const wxTextEntry * self) {
    return new wxString(self->GetValue());
}
bool wxTextEntry_IsEditable(const wxTextEntry * self) {
    return self->IsEditable();
}
bool wxTextEntry_IsEmpty(const wxTextEntry * self) {
    return self->IsEmpty();
}
void wxTextEntry_Paste(wxTextEntry * self) {
    return self->Paste();
}
void wxTextEntry_Redo(wxTextEntry * self) {
    return self->Redo();
}
void wxTextEntry_Remove(wxTextEntry * self, long from, long to) {
    return self->Remove(from, to);
}
void wxTextEntry_Replace(wxTextEntry * self, long from, long to, const wxString * value) {
    return self->Replace(from, to, *value);
}
void wxTextEntry_SetEditable(wxTextEntry * self, bool editable) {
    return self->SetEditable(editable);
}
void wxTextEntry_SetInsertionPoint(wxTextEntry * self, long pos) {
    return self->SetInsertionPoint(pos);
}
void wxTextEntry_SetInsertionPointEnd(wxTextEntry * self) {
    return self->SetInsertionPointEnd();
}
void wxTextEntry_SetSelection(wxTextEntry * self, long from, long to) {
    return self->SetSelection(from, to);
}
void wxTextEntry_SelectAll(wxTextEntry * self) {
    return self->SelectAll();
}
void wxTextEntry_SelectNone(wxTextEntry * self) {
    return self->SelectNone();
}
bool wxTextEntry_SetHint(wxTextEntry * self, const wxString * hint) {
    return self->SetHint(*hint);
}
wxString *wxTextEntry_GetHint(const wxTextEntry * self) {
    return new wxString(self->GetHint());
}
bool wxTextEntry_SetMargins(wxTextEntry * self, const wxPoint * pt) {
    return self->SetMargins(*pt);
}
bool wxTextEntry_SetMargins1(wxTextEntry * self, wxCoord left, wxCoord top) {
    return self->SetMargins(left, top);
}
wxPoint *wxTextEntry_GetMargins(const wxTextEntry * self) {
    return new wxPoint(self->GetMargins());
}
void wxTextEntry_SetValue(wxTextEntry * self, const wxString * value) {
    return self->SetValue(*value);
}
void wxTextEntry_Undo(wxTextEntry * self) {
    return self->Undo();
}
void wxTextEntry_WriteText(wxTextEntry * self, const wxString * text) {
    return self->WriteText(*text);
}

// CLASS: wxTimePickerCtrl
wxClassInfo *wxTimePickerCtrl_CLASSINFO() {
    return wxCLASSINFO(wxTimePickerCtrl);
}
wxTimePickerCtrl *wxTimePickerCtrl_new() {
    return new wxTimePickerCtrl();
}
wxTimePickerCtrl *wxTimePickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxTimePickerCtrl(parent, id, *dt, *pos, *size, style, *validator, *name);
}
bool wxTimePickerCtrl_Create(wxTimePickerCtrl * self, wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *dt, *pos, *size, style, *validator, *name);
}
bool wxTimePickerCtrl_GetTime(const wxTimePickerCtrl * self, int * hour, int * min, int * sec) {
    return self->GetTime(hour, min, sec);
}
wxDateTime *wxTimePickerCtrl_GetValue(const wxTimePickerCtrl * self) {
    return new wxDateTime(self->GetValue());
}
bool wxTimePickerCtrl_SetTime(wxTimePickerCtrl * self, int hour, int min, int sec) {
    return self->SetTime(hour, min, sec);
}
void wxTimePickerCtrl_SetValue(wxTimePickerCtrl * self, const wxDateTime * dt) {
    return self->SetValue(*dt);
}

// CLASS: wxToggleButton
wxClassInfo *wxToggleButton_CLASSINFO() {
    return wxCLASSINFO(wxToggleButton);
}
wxToggleButton *wxToggleButton_new() {
    return new wxToggleButton();
}
wxToggleButton *wxToggleButton_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * val, const wxString * name) {
    return new wxToggleButton(parent, id, *label, *pos, *size, style, *val, *name);
}
bool wxToggleButton_Create(wxToggleButton * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * val, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *val, *name);
}
bool wxToggleButton_GetValue(const wxToggleButton * self) {
    return self->GetValue();
}
void wxToggleButton_SetValue(wxToggleButton * self, bool state) {
    return self->SetValue(state);
}

// CLASS: wxToolBar
wxClassInfo *wxToolBar_CLASSINFO() {
    return wxCLASSINFO(wxToolBar);
}
wxToolBar *wxToolBar_new() {
    return new wxToolBar();
}
wxToolBar *wxToolBar_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxToolBar(parent, id, *pos, *size, style, *name);
}
wxToolBarToolBase * wxToolBar_AddCheckTool(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bitmap1, const wxBitmapBundle * bmp_disabled, const wxString * short_help, const wxString * long_help, wxObject * client_data) {
    return self->AddCheckTool(tool_id, *label, *bitmap1, *bmp_disabled, *short_help, *long_help, client_data);
}
wxToolBarToolBase * wxToolBar_AddControl(wxToolBar * self, wxControl * control, const wxString * label) {
    return self->AddControl(control, *label);
}
wxToolBarToolBase * wxToolBar_AddRadioTool(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bitmap1, const wxBitmapBundle * bmp_disabled, const wxString * short_help, const wxString * long_help, wxObject * client_data) {
    return self->AddRadioTool(tool_id, *label, *bitmap1, *bmp_disabled, *short_help, *long_help, client_data);
}
wxToolBarToolBase * wxToolBar_AddSeparator(wxToolBar * self) {
    return self->AddSeparator();
}
wxToolBarToolBase * wxToolBar_AddStretchableSpace(wxToolBar * self) {
    return self->AddStretchableSpace();
}
wxToolBarToolBase * wxToolBar_AddTool(wxToolBar * self, wxToolBarToolBase * tool) {
    return self->AddTool(tool);
}
wxToolBarToolBase * wxToolBar_AddTool1(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bitmap, const wxString * short_help, wxItemKind kind) {
    return self->AddTool(tool_id, *label, *bitmap, *short_help, kind);
}
wxToolBarToolBase * wxToolBar_AddTool2(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bitmap, const wxBitmapBundle * bmp_disabled, wxItemKind kind, const wxString * short_help, const wxString * long_help, wxObject * client_data) {
    return self->AddTool(tool_id, *label, *bitmap, *bmp_disabled, kind, *short_help, *long_help, client_data);
}
void wxToolBar_ClearTools(wxToolBar * self) {
    return self->ClearTools();
}
bool wxToolBar_DeleteTool(wxToolBar * self, int tool_id) {
    return self->DeleteTool(tool_id);
}
bool wxToolBar_DeleteToolByPos(wxToolBar * self, size_t pos) {
    return self->DeleteToolByPos(pos);
}
void wxToolBar_EnableTool(wxToolBar * self, int tool_id, bool enable) {
    return self->EnableTool(tool_id, enable);
}
wxToolBarToolBase * wxToolBar_FindById(const wxToolBar * self, int id) {
    return self->FindById(id);
}
wxControl * wxToolBar_FindControl(wxToolBar * self, int id) {
    return self->FindControl(id);
}
wxToolBarToolBase * wxToolBar_FindToolForPosition(const wxToolBar * self, wxCoord x, wxCoord y) {
    return self->FindToolForPosition(x, y);
}
wxSize *wxToolBar_GetMargins(const wxToolBar * self) {
    return new wxSize(self->GetMargins());
}
wxSize *wxToolBar_GetToolBitmapSize(const wxToolBar * self) {
    return new wxSize(self->GetToolBitmapSize());
}
const wxToolBarToolBase * wxToolBar_GetToolByPos1(const wxToolBar * self, int pos) {
    return self->GetToolByPos(pos);
}
wxObject * wxToolBar_GetToolClientData(const wxToolBar * self, int tool_id) {
    return self->GetToolClientData(tool_id);
}
bool wxToolBar_GetToolEnabled(const wxToolBar * self, int tool_id) {
    return self->GetToolEnabled(tool_id);
}
wxString *wxToolBar_GetToolLongHelp(const wxToolBar * self, int tool_id) {
    return new wxString(self->GetToolLongHelp(tool_id));
}
int wxToolBar_GetToolPacking(const wxToolBar * self) {
    return self->GetToolPacking();
}
int wxToolBar_GetToolPos(const wxToolBar * self, int tool_id) {
    return self->GetToolPos(tool_id);
}
int wxToolBar_GetToolSeparation(const wxToolBar * self) {
    return self->GetToolSeparation();
}
wxString *wxToolBar_GetToolShortHelp(const wxToolBar * self, int tool_id) {
    return new wxString(self->GetToolShortHelp(tool_id));
}
wxSize *wxToolBar_GetToolSize(const wxToolBar * self) {
    return new wxSize(self->GetToolSize());
}
bool wxToolBar_GetToolState(const wxToolBar * self, int tool_id) {
    return self->GetToolState(tool_id);
}
size_t wxToolBar_GetToolsCount(const wxToolBar * self) {
    return self->GetToolsCount();
}
wxToolBarToolBase * wxToolBar_InsertControl(wxToolBar * self, size_t pos, wxControl * control, const wxString * label) {
    return self->InsertControl(pos, control, *label);
}
wxToolBarToolBase * wxToolBar_InsertSeparator(wxToolBar * self, size_t pos) {
    return self->InsertSeparator(pos);
}
wxToolBarToolBase * wxToolBar_InsertStretchableSpace(wxToolBar * self, size_t pos) {
    return self->InsertStretchableSpace(pos);
}
wxToolBarToolBase * wxToolBar_InsertTool(wxToolBar * self, size_t pos, int tool_id, const wxString * label, const wxBitmapBundle * bitmap, const wxBitmapBundle * bmp_disabled, wxItemKind kind, const wxString * short_help, const wxString * long_help, wxObject * client_data) {
    return self->InsertTool(pos, tool_id, *label, *bitmap, *bmp_disabled, kind, *short_help, *long_help, client_data);
}
wxToolBarToolBase * wxToolBar_InsertTool1(wxToolBar * self, size_t pos, wxToolBarToolBase * tool) {
    return self->InsertTool(pos, tool);
}
bool wxToolBar_OnLeftClick(wxToolBar * self, int tool_id, bool toggle_down) {
    return self->OnLeftClick(tool_id, toggle_down);
}
void wxToolBar_OnMouseEnter(wxToolBar * self, int tool_id) {
    return self->OnMouseEnter(tool_id);
}
void wxToolBar_OnRightClick(wxToolBar * self, int tool_id, long x, long y) {
    return self->OnRightClick(tool_id, x, y);
}
bool wxToolBar_Realize(wxToolBar * self) {
    return self->Realize();
}
wxToolBarToolBase * wxToolBar_RemoveTool(wxToolBar * self, int id) {
    return self->RemoveTool(id);
}
bool wxToolBar_SetDropdownMenu(wxToolBar * self, int id, wxMenu * menu) {
    return self->SetDropdownMenu(id, menu);
}
void wxToolBar_SetMargins(wxToolBar * self, int x, int y) {
    return self->SetMargins(x, y);
}
void wxToolBar_SetMargins1(wxToolBar * self, const wxSize * size) {
    return self->SetMargins(*size);
}
void wxToolBar_SetToolBitmapSize(wxToolBar * self, const wxSize * size) {
    return self->SetToolBitmapSize(*size);
}
void wxToolBar_SetToolClientData(wxToolBar * self, int id, wxObject * client_data) {
    return self->SetToolClientData(id, client_data);
}
void wxToolBar_SetToolDisabledBitmap(wxToolBar * self, int id, const wxBitmapBundle * bitmap) {
    return self->SetToolDisabledBitmap(id, *bitmap);
}
void wxToolBar_SetToolLongHelp(wxToolBar * self, int tool_id, const wxString * help_string) {
    return self->SetToolLongHelp(tool_id, *help_string);
}
void wxToolBar_SetToolNormalBitmap(wxToolBar * self, int id, const wxBitmapBundle * bitmap) {
    return self->SetToolNormalBitmap(id, *bitmap);
}
void wxToolBar_SetToolPacking(wxToolBar * self, int packing) {
    return self->SetToolPacking(packing);
}
void wxToolBar_SetToolSeparation(wxToolBar * self, int separation) {
    return self->SetToolSeparation(separation);
}
void wxToolBar_SetToolShortHelp(wxToolBar * self, int tool_id, const wxString * help_string) {
    return self->SetToolShortHelp(tool_id, *help_string);
}
void wxToolBar_ToggleTool(wxToolBar * self, int tool_id, bool toggle) {
    return self->ToggleTool(tool_id, toggle);
}
wxToolBarToolBase * wxToolBar_CreateTool(wxToolBar * self, int tool_id, const wxString * label, const wxBitmapBundle * bmp_normal, const wxBitmapBundle * bmp_disabled, wxItemKind kind, wxObject * client_data, const wxString * short_help, const wxString * long_help) {
    return self->CreateTool(tool_id, *label, *bmp_normal, *bmp_disabled, kind, client_data, *short_help, *long_help);
}
wxToolBarToolBase * wxToolBar_CreateTool1(wxToolBar * self, wxControl * control, const wxString * label) {
    return self->CreateTool(control, *label);
}
wxToolBarToolBase * wxToolBar_CreateSeparator(wxToolBar * self) {
    return self->CreateSeparator();
}

// CLASS: wxTopLevelWindow
wxClassInfo *wxTopLevelWindow_CLASSINFO() {
    return wxCLASSINFO(wxTopLevelWindow);
}
wxTopLevelWindow *wxTopLevelWindow_new() {
    return new wxTopLevelWindow();
}
wxTopLevelWindow *wxTopLevelWindow_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxTopLevelWindow(parent, id, *title, *pos, *size, style, *name);
}
bool wxTopLevelWindow_Create(wxTopLevelWindow * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *title, *pos, *size, style, *name);
}
void wxTopLevelWindow_CenterOnScreen(wxTopLevelWindow * self, int direction) {
    return self->CenterOnScreen(direction);
}
void wxTopLevelWindow_CentreOnScreen(wxTopLevelWindow * self, int direction) {
    return self->CentreOnScreen(direction);
}
bool wxTopLevelWindow_EnableCloseButton(wxTopLevelWindow * self, bool enable) {
    return self->EnableCloseButton(enable);
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxTopLevelWindow_EnableMaximizeButton(wxTopLevelWindow * self, bool enable) {
    return self->EnableMaximizeButton(enable);
}
bool wxTopLevelWindow_EnableMinimizeButton(wxTopLevelWindow * self, bool enable) {
    return self->EnableMinimizeButton(enable);
}
#endif
wxWindow * wxTopLevelWindow_GetDefaultItem(const wxTopLevelWindow * self) {
    return self->GetDefaultItem();
}
wxIcon *wxTopLevelWindow_GetIcon(const wxTopLevelWindow * self) {
    return new wxIcon(self->GetIcon());
}
wxString *wxTopLevelWindow_GetTitle(const wxTopLevelWindow * self) {
    return new wxString(self->GetTitle());
}
void wxTopLevelWindow_Iconize(wxTopLevelWindow * self, bool iconize) {
    return self->Iconize(iconize);
}
bool wxTopLevelWindow_IsActive(wxTopLevelWindow * self) {
    return self->IsActive();
}
bool wxTopLevelWindow_IsAlwaysMaximized(const wxTopLevelWindow * self) {
    return self->IsAlwaysMaximized();
}
bool wxTopLevelWindow_IsFullScreen(const wxTopLevelWindow * self) {
    return self->IsFullScreen();
}
bool wxTopLevelWindow_IsIconized(const wxTopLevelWindow * self) {
    return self->IsIconized();
}
bool wxTopLevelWindow_IsMaximized(const wxTopLevelWindow * self) {
    return self->IsMaximized();
}
void wxTopLevelWindow_Maximize(wxTopLevelWindow * self, bool maximize) {
    return self->Maximize(maximize);
}
#ifdef __WXMSW__
wxMenu * wxTopLevelWindow_MSWGetSystemMenu(const wxTopLevelWindow * self) {
    return self->MSWGetSystemMenu();
}
#endif
void wxTopLevelWindow_RequestUserAttention(wxTopLevelWindow * self, int flags) {
    return self->RequestUserAttention(flags);
}
void wxTopLevelWindow_Restore(wxTopLevelWindow * self) {
    return self->Restore();
}
wxWindow * wxTopLevelWindow_SetDefaultItem(wxTopLevelWindow * self, wxWindow * win) {
    return self->SetDefaultItem(win);
}
wxWindow * wxTopLevelWindow_SetTmpDefaultItem(wxTopLevelWindow * self, wxWindow * win) {
    return self->SetTmpDefaultItem(win);
}
wxWindow * wxTopLevelWindow_GetTmpDefaultItem(const wxTopLevelWindow * self) {
    return self->GetTmpDefaultItem();
}
void wxTopLevelWindow_SetIcon(wxTopLevelWindow * self, const wxIcon * icon) {
    return self->SetIcon(*icon);
}
void wxTopLevelWindow_SetIcons(wxTopLevelWindow * self, const wxIconBundle * icons) {
    return self->SetIcons(*icons);
}
void wxTopLevelWindow_SetTitle(wxTopLevelWindow * self, const wxString * title) {
    return self->SetTitle(*title);
}
bool wxTopLevelWindow_ShouldPreventAppExit(const wxTopLevelWindow * self) {
    return self->ShouldPreventAppExit();
}
void wxTopLevelWindow_OSXSetModified(wxTopLevelWindow * self, bool modified) {
    return self->OSXSetModified(modified);
}
bool wxTopLevelWindow_OSXIsModified(const wxTopLevelWindow * self) {
    return self->OSXIsModified();
}
void wxTopLevelWindow_SetRepresentedFilename(wxTopLevelWindow * self, const wxString * filename) {
    return self->SetRepresentedFilename(*filename);
}
void wxTopLevelWindow_ShowWithoutActivating(wxTopLevelWindow * self) {
    return self->ShowWithoutActivating();
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxTopLevelWindow_EnableFullScreenView(wxTopLevelWindow * self, bool enable, long style) {
    return self->EnableFullScreenView(enable, style);
}
#endif
bool wxTopLevelWindow_ShowFullScreen(wxTopLevelWindow * self, bool show, long style) {
    return self->ShowFullScreen(show, style);
}
wxSize *wxTopLevelWindow_GetDefaultSize() {
    return new wxSize(wxTopLevelWindow::GetDefaultSize());
}

} // extern "C"

