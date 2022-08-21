#include "generated.h"

extern "C" {

// CLASS: wxTGAHandler
wxClassInfo *wxTGAHandler_CLASSINFO() {
    return wxCLASSINFO(wxTGAHandler);
}
wxTGAHandler *wxTGAHandler_new() {
    return new wxTGAHandler();
}

// CLASS: wxTIFFHandler
wxClassInfo *wxTIFFHandler_CLASSINFO() {
    return wxCLASSINFO(wxTIFFHandler);
}
wxTIFFHandler *wxTIFFHandler_new() {
    return new wxTIFFHandler();
}

// CLASS: wxTaskBarIcon
wxClassInfo *wxTaskBarIcon_CLASSINFO() {
    return wxCLASSINFO(wxTaskBarIcon);
}
void wxTaskBarIcon_Destroy(wxTaskBarIcon * self) {
    return self->Destroy();
}
bool wxTaskBarIcon_IsIconInstalled(const wxTaskBarIcon * self) {
    return self->IsIconInstalled();
}
bool wxTaskBarIcon_IsOk(const wxTaskBarIcon * self) {
    return self->IsOk();
}
bool wxTaskBarIcon_PopupMenu(wxTaskBarIcon * self, wxMenu * menu) {
    return self->PopupMenu(menu);
}
bool wxTaskBarIcon_RemoveIcon(wxTaskBarIcon * self) {
    return self->RemoveIcon();
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxTaskBarIcon_SetIcon(wxTaskBarIcon * self, const wxBitmapBundle * icon, const wxString * tooltip) {
    return self->SetIcon(*icon, *tooltip);
}
#endif
bool wxTaskBarIcon_IsAvailable() {
    return wxTaskBarIcon::IsAvailable();
}

// CLASS: wxTaskBarIconEvent
wxClassInfo *wxTaskBarIconEvent_CLASSINFO() {
    return wxCLASSINFO(wxTaskBarIconEvent);
}

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

// CLASS: wxTextDataObject
void wxTextDataObject_delete(wxTextDataObject *self) {
    delete self;
}
wxTextDataObject *wxTextDataObject_new(const wxString * text) {
    return new wxTextDataObject(*text);
}
wxString *wxTextDataObject_GetText(const wxTextDataObject * self) {
    return new wxString(self->GetText());
}
size_t wxTextDataObject_GetTextLength(const wxTextDataObject * self) {
    return self->GetTextLength();
}
void wxTextDataObject_SetText(wxTextDataObject * self, const wxString * str_text) {
    return self->SetText(*str_text);
}

// CLASS: wxTextDropTarget
void wxTextDropTarget_delete(wxTextDropTarget *self) {
    delete self;
}
bool wxTextDropTarget_OnDropText(wxTextDropTarget * self, wxCoord x, wxCoord y, const wxString * data) {
    return self->OnDropText(x, y, *data);
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

// CLASS: wxTextEntryDialog
wxClassInfo *wxTextEntryDialog_CLASSINFO() {
    return wxCLASSINFO(wxTextEntryDialog);
}
wxTextEntryDialog *wxTextEntryDialog_new() {
    return new wxTextEntryDialog();
}
wxTextEntryDialog *wxTextEntryDialog_new1(wxWindow * parent, const wxString * message, const wxString * caption, const wxString * value, long style, const wxPoint * pos) {
    return new wxTextEntryDialog(parent, *message, *caption, *value, style, *pos);
}
bool wxTextEntryDialog_Create(wxTextEntryDialog * self, wxWindow * parent, const wxString * message, const wxString * caption, const wxString * value, long style, const wxPoint * pos) {
    return self->Create(parent, *message, *caption, *value, style, *pos);
}
wxString *wxTextEntryDialog_GetValue(const wxTextEntryDialog * self) {
    return new wxString(self->GetValue());
}
void wxTextEntryDialog_SetTextValidator(wxTextEntryDialog * self, const wxTextValidator * validator) {
    return self->SetTextValidator(*validator);
}
void wxTextEntryDialog_SetValue(wxTextEntryDialog * self, const wxString * value) {
    return self->SetValue(*value);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxTextEntryDialog_ForceUpper(wxTextEntryDialog * self) {
    return self->ForceUpper();
}
#endif

// CLASS: wxTextValidator
wxClassInfo *wxTextValidator_CLASSINFO() {
    return wxCLASSINFO(wxTextValidator);
}
wxTextValidator *wxTextValidator_new(const wxTextValidator * validator) {
    return new wxTextValidator(*validator);
}
wxTextValidator *wxTextValidator_new1(long style, wxString * val_ptr) {
    return new wxTextValidator(style, val_ptr);
}
#if wxCHECK_VERSION(3, 1, 0)
wxString *wxTextValidator_GetCharExcludes(const wxTextValidator * self) {
    return new wxString(self->GetCharExcludes());
}
wxString *wxTextValidator_GetCharIncludes(const wxTextValidator * self) {
    return new wxString(self->GetCharIncludes());
}
wxArrayString *wxTextValidator_GetExcludes(const wxTextValidator * self) {
    return new wxArrayString(self->GetExcludes());
}
wxArrayString *wxTextValidator_GetIncludes(const wxTextValidator * self) {
    return new wxArrayString(self->GetIncludes());
}
#endif
long wxTextValidator_GetStyle(const wxTextValidator * self) {
    return self->GetStyle();
}
void wxTextValidator_OnChar(wxTextValidator * self, wxKeyEvent * event) {
    return self->OnChar(*event);
}
void wxTextValidator_SetExcludes(wxTextValidator * self, const wxArrayString * string_list) {
    return self->SetExcludes(*string_list);
}
void wxTextValidator_SetCharExcludes(wxTextValidator * self, const wxString * chars) {
    return self->SetCharExcludes(*chars);
}
void wxTextValidator_SetIncludes(wxTextValidator * self, const wxArrayString * string_list) {
    return self->SetIncludes(*string_list);
}
void wxTextValidator_SetCharIncludes(wxTextValidator * self, const wxString * chars) {
    return self->SetCharIncludes(*chars);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxTextValidator_AddExclude(wxTextValidator * self, const wxString * exclude) {
    return self->AddExclude(*exclude);
}
void wxTextValidator_AddInclude(wxTextValidator * self, const wxString * include) {
    return self->AddInclude(*include);
}
void wxTextValidator_AddCharExcludes(wxTextValidator * self, const wxString * chars) {
    return self->AddCharExcludes(*chars);
}
void wxTextValidator_AddCharIncludes(wxTextValidator * self, const wxString * chars) {
    return self->AddCharIncludes(*chars);
}
#endif
void wxTextValidator_SetStyle(wxTextValidator * self, long style) {
    return self->SetStyle(style);
}
#if wxCHECK_VERSION(3, 1, 0)
wxString *wxTextValidator_IsValid(const wxTextValidator * self, const wxString * val) {
    return new wxString(self->IsValid(*val));
}
#endif

// CLASS: wxThreadEvent
wxClassInfo *wxThreadEvent_CLASSINFO() {
    return wxCLASSINFO(wxThreadEvent);
}
long wxThreadEvent_GetExtraLong(const wxThreadEvent * self) {
    return self->GetExtraLong();
}
int wxThreadEvent_GetInt(const wxThreadEvent * self) {
    return self->GetInt();
}
wxString *wxThreadEvent_GetString(const wxThreadEvent * self) {
    return new wxString(self->GetString());
}
void wxThreadEvent_SetExtraLong(wxThreadEvent * self, long extra_long) {
    return self->SetExtraLong(extra_long);
}
void wxThreadEvent_SetInt(wxThreadEvent * self, int int_command) {
    return self->SetInt(int_command);
}
void wxThreadEvent_SetString(wxThreadEvent * self, const wxString * string) {
    return self->SetString(*string);
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

// CLASS: wxTipProvider
void wxTipProvider_delete(wxTipProvider *self) {
    delete self;
}
size_t wxTipProvider_GetCurrentTip(const wxTipProvider * self) {
    return self->GetCurrentTip();
}
wxString *wxTipProvider_GetTip(wxTipProvider * self) {
    return new wxString(self->GetTip());
}

// CLASS: wxTipWindow
wxClassInfo *wxTipWindow_CLASSINFO() {
    return wxCLASSINFO(wxTipWindow);
}
wxTipWindow *wxTipWindow_new(wxWindow * parent, const wxString * text, wxCoord max_length, wxTipWindow ** window_ptr, wxRect * rect_bounds) {
    return new wxTipWindow(parent, *text, max_length, window_ptr, rect_bounds);
}
void wxTipWindow_SetBoundingRect(wxTipWindow * self, const wxRect * rect_bound) {
    return self->SetBoundingRect(*rect_bound);
}
void wxTipWindow_SetTipWindowPtr(wxTipWindow * self, wxTipWindow ** window_ptr) {
    return self->SetTipWindowPtr(window_ptr);
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

// CLASS: wxToolTip
wxClassInfo *wxToolTip_CLASSINFO() {
    return wxCLASSINFO(wxToolTip);
}
wxToolTip *wxToolTip_new(const wxString * tip) {
    return new wxToolTip(*tip);
}
wxString *wxToolTip_GetTip(const wxToolTip * self) {
    return new wxString(self->GetTip());
}
wxWindow * wxToolTip_GetWindow(const wxToolTip * self) {
    return self->GetWindow();
}
void wxToolTip_SetTip(wxToolTip * self, const wxString * tip) {
    return self->SetTip(*tip);
}
void wxToolTip_Enable(bool flag) {
    return wxToolTip::Enable(flag);
}
void wxToolTip_SetAutoPop(long msecs) {
    return wxToolTip::SetAutoPop(msecs);
}
void wxToolTip_SetDelay(long msecs) {
    return wxToolTip::SetDelay(msecs);
}
#ifdef __WXMSW__
void wxToolTip_SetMaxWidth(int width) {
    return wxToolTip::SetMaxWidth(width);
}
#endif
void wxToolTip_SetReshow(long msecs) {
    return wxToolTip::SetReshow(msecs);
}

// CLASS: wxToolbook
wxClassInfo *wxToolbook_CLASSINFO() {
    return wxCLASSINFO(wxToolbook);
}
wxToolbook *wxToolbook_new() {
    return new wxToolbook();
}
wxToolbook *wxToolbook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxToolbook(parent, id, *pos, *size, style, *name);
}
bool wxToolbook_Create(wxToolbook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
wxToolBarBase * wxToolbook_GetToolBar(const wxToolbook * self) {
    return self->GetToolBar();
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxToolbook_EnablePage(wxToolbook * self, size_t page, bool enable) {
    return self->EnablePage(page, enable);
}
bool wxToolbook_EnablePage1(wxToolbook * self, wxWindow * page, bool enable) {
    return self->EnablePage(page, enable);
}
#endif

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

// CLASS: wxTreeCtrl
wxClassInfo *wxTreeCtrl_CLASSINFO() {
    return wxCLASSINFO(wxTreeCtrl);
}
wxTreeCtrl *wxTreeCtrl_new() {
    return new wxTreeCtrl();
}
wxTreeCtrl *wxTreeCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxTreeCtrl(parent, id, *pos, *size, style, *validator, *name);
}
wxTreeItemId *wxTreeCtrl_AddRoot(wxTreeCtrl * self, const wxString * text, int image, int sel_image, wxTreeItemData * data) {
    return new wxTreeItemId(self->AddRoot(*text, image, sel_image, data));
}
wxTreeItemId *wxTreeCtrl_AppendItem(wxTreeCtrl * self, const wxTreeItemId * parent, const wxString * text, int image, int sel_image, wxTreeItemData * data) {
    return new wxTreeItemId(self->AppendItem(*parent, *text, image, sel_image, data));
}
#ifndef __WXMSW__
void wxTreeCtrl_AssignButtonsImageList(wxTreeCtrl * self, wxImageList * image_list) {
    return self->AssignButtonsImageList(image_list);
}
#endif
void wxTreeCtrl_AssignStateImageList(wxTreeCtrl * self, wxImageList * image_list) {
    return self->AssignStateImageList(image_list);
}
void wxTreeCtrl_Collapse(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->Collapse(*item);
}
void wxTreeCtrl_CollapseAll(wxTreeCtrl * self) {
    return self->CollapseAll();
}
void wxTreeCtrl_CollapseAllChildren(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->CollapseAllChildren(*item);
}
void wxTreeCtrl_CollapseAndReset(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->CollapseAndReset(*item);
}
bool wxTreeCtrl_Create(wxTreeCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *validator, *name);
}
void wxTreeCtrl_Delete(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->Delete(*item);
}
void wxTreeCtrl_DeleteAllItems(wxTreeCtrl * self) {
    return self->DeleteAllItems();
}
void wxTreeCtrl_DeleteChildren(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->DeleteChildren(*item);
}
wxTextCtrl * wxTreeCtrl_EditLabel(wxTreeCtrl * self, const wxTreeItemId * item, wxClassInfo * text_ctrl_class) {
    return self->EditLabel(*item, text_ctrl_class);
}
void wxTreeCtrl_EnableBellOnNoMatch(wxTreeCtrl * self, bool on) {
    return self->EnableBellOnNoMatch(on);
}
void wxTreeCtrl_EndEditLabel(wxTreeCtrl * self, const wxTreeItemId * item, bool discard_changes) {
    return self->EndEditLabel(*item, discard_changes);
}
void wxTreeCtrl_EnsureVisible(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->EnsureVisible(*item);
}
void wxTreeCtrl_Expand(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->Expand(*item);
}
void wxTreeCtrl_ExpandAll(wxTreeCtrl * self) {
    return self->ExpandAll();
}
void wxTreeCtrl_ExpandAllChildren(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->ExpandAllChildren(*item);
}
bool wxTreeCtrl_GetBoundingRect(const wxTreeCtrl * self, const wxTreeItemId * item, wxRect * rect, bool text_only) {
    return self->GetBoundingRect(*item, *rect, text_only);
}
#ifndef __WXMSW__
wxImageList * wxTreeCtrl_GetButtonsImageList(const wxTreeCtrl * self) {
    return self->GetButtonsImageList();
}
#endif
size_t wxTreeCtrl_GetChildrenCount(const wxTreeCtrl * self, const wxTreeItemId * item, bool recursively) {
    return self->GetChildrenCount(*item, recursively);
}
unsigned int wxTreeCtrl_GetCount(const wxTreeCtrl * self) {
    return self->GetCount();
}
wxTextCtrl * wxTreeCtrl_GetEditControl(const wxTreeCtrl * self) {
    return self->GetEditControl();
}
wxTreeItemId *wxTreeCtrl_GetFirstChild(const wxTreeCtrl * self, const wxTreeItemId * item, wxTreeItemIdValue * cookie) {
    return new wxTreeItemId(self->GetFirstChild(*item, *cookie));
}
wxTreeItemId *wxTreeCtrl_GetFirstVisibleItem(const wxTreeCtrl * self) {
    return new wxTreeItemId(self->GetFirstVisibleItem());
}
wxTreeItemId *wxTreeCtrl_GetFocusedItem(const wxTreeCtrl * self) {
    return new wxTreeItemId(self->GetFocusedItem());
}
void wxTreeCtrl_ClearFocusedItem(wxTreeCtrl * self) {
    return self->ClearFocusedItem();
}
void wxTreeCtrl_SetFocusedItem(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->SetFocusedItem(*item);
}
unsigned int wxTreeCtrl_GetIndent(const wxTreeCtrl * self) {
    return self->GetIndent();
}
unsigned int wxTreeCtrl_GetSpacing(const wxTreeCtrl * self) {
    return self->GetSpacing();
}
wxColour *wxTreeCtrl_GetItemBackgroundColour(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxColour(self->GetItemBackgroundColour(*item));
}
wxTreeItemData * wxTreeCtrl_GetItemData(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->GetItemData(*item);
}
wxFont *wxTreeCtrl_GetItemFont(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxFont(self->GetItemFont(*item));
}
wxTreeItemId *wxTreeCtrl_GetItemParent(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxTreeItemId(self->GetItemParent(*item));
}
int wxTreeCtrl_GetItemState(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->GetItemState(*item);
}
wxString *wxTreeCtrl_GetItemText(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxString(self->GetItemText(*item));
}
wxColour *wxTreeCtrl_GetItemTextColour(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxColour(self->GetItemTextColour(*item));
}
wxTreeItemId *wxTreeCtrl_GetLastChild(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxTreeItemId(self->GetLastChild(*item));
}
wxTreeItemId *wxTreeCtrl_GetNextChild(const wxTreeCtrl * self, const wxTreeItemId * item, wxTreeItemIdValue * cookie) {
    return new wxTreeItemId(self->GetNextChild(*item, *cookie));
}
wxTreeItemId *wxTreeCtrl_GetNextSibling(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxTreeItemId(self->GetNextSibling(*item));
}
wxTreeItemId *wxTreeCtrl_GetNextVisible(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxTreeItemId(self->GetNextVisible(*item));
}
wxTreeItemId *wxTreeCtrl_GetPrevSibling(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxTreeItemId(self->GetPrevSibling(*item));
}
wxTreeItemId *wxTreeCtrl_GetPrevVisible(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return new wxTreeItemId(self->GetPrevVisible(*item));
}
bool wxTreeCtrl_GetQuickBestSize(const wxTreeCtrl * self) {
    return self->GetQuickBestSize();
}
wxTreeItemId *wxTreeCtrl_GetRootItem(const wxTreeCtrl * self) {
    return new wxTreeItemId(self->GetRootItem());
}
wxTreeItemId *wxTreeCtrl_GetSelection(const wxTreeCtrl * self) {
    return new wxTreeItemId(self->GetSelection());
}
size_t wxTreeCtrl_GetSelections(const wxTreeCtrl * self, wxArrayTreeItemIds * selection) {
    return self->GetSelections(*selection);
}
wxImageList * wxTreeCtrl_GetStateImageList(const wxTreeCtrl * self) {
    return self->GetStateImageList();
}
wxTreeItemId *wxTreeCtrl_HitTest(const wxTreeCtrl * self, const wxPoint * point, int * flags) {
    return new wxTreeItemId(self->HitTest(*point, *flags));
}
wxTreeItemId *wxTreeCtrl_InsertItem(wxTreeCtrl * self, const wxTreeItemId * parent, const wxTreeItemId * previous, const wxString * text, int image, int sel_image, wxTreeItemData * data) {
    return new wxTreeItemId(self->InsertItem(*parent, *previous, *text, image, sel_image, data));
}
wxTreeItemId *wxTreeCtrl_InsertItem1(wxTreeCtrl * self, const wxTreeItemId * parent, size_t pos, const wxString * text, int image, int sel_image, wxTreeItemData * data) {
    return new wxTreeItemId(self->InsertItem(*parent, pos, *text, image, sel_image, data));
}
bool wxTreeCtrl_IsBold(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->IsBold(*item);
}
bool wxTreeCtrl_IsEmpty(const wxTreeCtrl * self) {
    return self->IsEmpty();
}
bool wxTreeCtrl_IsExpanded(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->IsExpanded(*item);
}
bool wxTreeCtrl_IsSelected(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->IsSelected(*item);
}
bool wxTreeCtrl_IsVisible(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->IsVisible(*item);
}
bool wxTreeCtrl_ItemHasChildren(const wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->ItemHasChildren(*item);
}
int wxTreeCtrl_OnCompareItems(wxTreeCtrl * self, const wxTreeItemId * item1, const wxTreeItemId * item2) {
    return self->OnCompareItems(*item1, *item2);
}
wxTreeItemId *wxTreeCtrl_PrependItem(wxTreeCtrl * self, const wxTreeItemId * parent, const wxString * text, int image, int sel_image, wxTreeItemData * data) {
    return new wxTreeItemId(self->PrependItem(*parent, *text, image, sel_image, data));
}
void wxTreeCtrl_ScrollTo(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->ScrollTo(*item);
}
void wxTreeCtrl_SelectItem(wxTreeCtrl * self, const wxTreeItemId * item, bool select) {
    return self->SelectItem(*item, select);
}
#ifndef __WXMSW__
void wxTreeCtrl_SetButtonsImageList(wxTreeCtrl * self, wxImageList * image_list) {
    return self->SetButtonsImageList(image_list);
}
#endif
void wxTreeCtrl_SetIndent(wxTreeCtrl * self, unsigned int indent) {
    return self->SetIndent(indent);
}
void wxTreeCtrl_SetSpacing(wxTreeCtrl * self, unsigned int spacing) {
    return self->SetSpacing(spacing);
}
void wxTreeCtrl_SetItemBackgroundColour(wxTreeCtrl * self, const wxTreeItemId * item, const wxColour * col) {
    return self->SetItemBackgroundColour(*item, *col);
}
void wxTreeCtrl_SetItemBold(wxTreeCtrl * self, const wxTreeItemId * item, bool bold) {
    return self->SetItemBold(*item, bold);
}
void wxTreeCtrl_SetItemData(wxTreeCtrl * self, const wxTreeItemId * item, wxTreeItemData * data) {
    return self->SetItemData(*item, data);
}
void wxTreeCtrl_SetItemDropHighlight(wxTreeCtrl * self, const wxTreeItemId * item, bool highlight) {
    return self->SetItemDropHighlight(*item, highlight);
}
void wxTreeCtrl_SetItemFont(wxTreeCtrl * self, const wxTreeItemId * item, const wxFont * font) {
    return self->SetItemFont(*item, *font);
}
void wxTreeCtrl_SetItemHasChildren(wxTreeCtrl * self, const wxTreeItemId * item, bool has_children) {
    return self->SetItemHasChildren(*item, has_children);
}
void wxTreeCtrl_SetItemState(wxTreeCtrl * self, const wxTreeItemId * item, int state) {
    return self->SetItemState(*item, state);
}
void wxTreeCtrl_SetItemText(wxTreeCtrl * self, const wxTreeItemId * item, const wxString * text) {
    return self->SetItemText(*item, *text);
}
void wxTreeCtrl_SetItemTextColour(wxTreeCtrl * self, const wxTreeItemId * item, const wxColour * col) {
    return self->SetItemTextColour(*item, *col);
}
void wxTreeCtrl_SetQuickBestSize(wxTreeCtrl * self, bool quick_best_size) {
    return self->SetQuickBestSize(quick_best_size);
}
void wxTreeCtrl_SetStateImageList(wxTreeCtrl * self, wxImageList * image_list) {
    return self->SetStateImageList(image_list);
}
void wxTreeCtrl_SetWindowStyle(wxTreeCtrl * self, long styles) {
    return self->SetWindowStyle(styles);
}
void wxTreeCtrl_SortChildren(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->SortChildren(*item);
}
void wxTreeCtrl_Toggle(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->Toggle(*item);
}
void wxTreeCtrl_ToggleItemSelection(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->ToggleItemSelection(*item);
}
void wxTreeCtrl_Unselect(wxTreeCtrl * self) {
    return self->Unselect();
}
void wxTreeCtrl_UnselectAll(wxTreeCtrl * self) {
    return self->UnselectAll();
}
void wxTreeCtrl_UnselectItem(wxTreeCtrl * self, const wxTreeItemId * item) {
    return self->UnselectItem(*item);
}
void wxTreeCtrl_SelectChildren(wxTreeCtrl * self, const wxTreeItemId * parent) {
    return self->SelectChildren(*parent);
}

// CLASS: wxTreeEvent
wxClassInfo *wxTreeEvent_CLASSINFO() {
    return wxCLASSINFO(wxTreeEvent);
}
wxTreeItemId *wxTreeEvent_GetItem(const wxTreeEvent * self) {
    return new wxTreeItemId(self->GetItem());
}
int wxTreeEvent_GetKeyCode(const wxTreeEvent * self) {
    return self->GetKeyCode();
}
wxKeyEvent *wxTreeEvent_GetKeyEvent(const wxTreeEvent * self) {
    return new wxKeyEvent(self->GetKeyEvent());
}
wxString *wxTreeEvent_GetLabel(const wxTreeEvent * self) {
    return new wxString(self->GetLabel());
}
wxTreeItemId *wxTreeEvent_GetOldItem(const wxTreeEvent * self) {
    return new wxTreeItemId(self->GetOldItem());
}
wxPoint *wxTreeEvent_GetPoint(const wxTreeEvent * self) {
    return new wxPoint(self->GetPoint());
}
bool wxTreeEvent_IsEditCancelled(const wxTreeEvent * self) {
    return self->IsEditCancelled();
}
void wxTreeEvent_SetToolTip(wxTreeEvent * self, const wxString * tooltip) {
    return self->SetToolTip(*tooltip);
}

// CLASS: wxTreeItemData
void wxTreeItemData_delete(wxTreeItemData *self) {
    delete self;
}
wxTreeItemData *wxTreeItemData_new() {
    return new wxTreeItemData();
}
wxTreeItemId *wxTreeItemData_GetId(const wxTreeItemData * self) {
    return new wxTreeItemId(self->GetId());
}
void wxTreeItemData_SetId(wxTreeItemData * self, const wxTreeItemId * id) {
    return self->SetId(*id);
}

// CLASS: wxTreeItemId
void wxTreeItemId_delete(wxTreeItemId *self) {
    delete self;
}
wxTreeItemId *wxTreeItemId_new() {
    return new wxTreeItemId();
}
bool wxTreeItemId_IsOk(const wxTreeItemId * self) {
    return self->IsOk();
}
void * wxTreeItemId_GetID(const wxTreeItemId * self) {
    return self->GetID();
}
void wxTreeItemId_Unset(wxTreeItemId * self) {
    return self->Unset();
}

// CLASS: wxTreeListCtrl
wxClassInfo *wxTreeListCtrl_CLASSINFO() {
    return wxCLASSINFO(wxTreeListCtrl);
}
void wxTreeListCtrl_AssignImageList(wxTreeListCtrl * self, wxImageList * image_list) {
    return self->AssignImageList(image_list);
}
void wxTreeListCtrl_SetImageList(wxTreeListCtrl * self, wxImageList * image_list) {
    return self->SetImageList(image_list);
}
int wxTreeListCtrl_AppendColumn(wxTreeListCtrl * self, const wxString * title, int width, wxAlignment align, int flags) {
    return self->AppendColumn(*title, width, align, flags);
}
void wxTreeListCtrl_ClearColumns(wxTreeListCtrl * self) {
    return self->ClearColumns();
}
int wxTreeListCtrl_WidthFor(const wxTreeListCtrl * self, const wxString * text) {
    return self->WidthFor(*text);
}
void wxTreeListCtrl_DeleteAllItems(wxTreeListCtrl * self) {
    return self->DeleteAllItems();
}
wxTreeListItem *wxTreeListCtrl_GetRootItem(const wxTreeListCtrl * self) {
    return new wxTreeListItem(self->GetRootItem());
}
wxTreeListItem *wxTreeListCtrl_GetFirstItem(const wxTreeListCtrl * self) {
    return new wxTreeListItem(self->GetFirstItem());
}
wxTreeListItem *wxTreeListCtrl_GetSelection(const wxTreeListCtrl * self) {
    return new wxTreeListItem(self->GetSelection());
}
void wxTreeListCtrl_SelectAll(wxTreeListCtrl * self) {
    return self->SelectAll();
}
void wxTreeListCtrl_UnselectAll(wxTreeListCtrl * self) {
    return self->UnselectAll();
}
bool wxTreeListCtrl_GetSortColumn(wxTreeListCtrl * self, unsigned * col, bool * ascending_order) {
    return self->GetSortColumn(col, ascending_order);
}
void wxTreeListCtrl_SetItemComparator(wxTreeListCtrl * self, wxTreeListItemComparator * comparator) {
    return self->SetItemComparator(comparator);
}
wxWindow * wxTreeListCtrl_GetView(const wxTreeListCtrl * self) {
    return self->GetView();
}
wxDataViewCtrl * wxTreeListCtrl_GetDataView(const wxTreeListCtrl * self) {
    return self->GetDataView();
}
wxTreeListCtrl *wxTreeListCtrl_new() {
    return new wxTreeListCtrl();
}
wxTreeListCtrl *wxTreeListCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxTreeListCtrl(parent, id, *pos, *size, style, *name);
}
bool wxTreeListCtrl_Create(wxTreeListCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}

// CLASS: wxTreeListItem
void wxTreeListItem_delete(wxTreeListItem *self) {
    delete self;
}
wxTreeListItem *wxTreeListItem_new() {
    return new wxTreeListItem();
}
bool wxTreeListItem_IsOk(const wxTreeListItem * self) {
    return self->IsOk();
}

// CLASS: wxTreeListItemComparator
void wxTreeListItemComparator_delete(wxTreeListItemComparator *self) {
    delete self;
}

// CLASS: wxTreebook
wxClassInfo *wxTreebook_CLASSINFO() {
    return wxCLASSINFO(wxTreebook);
}
wxTreebook *wxTreebook_new() {
    return new wxTreebook();
}
wxTreebook *wxTreebook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxTreebook(parent, id, *pos, *size, style, *name);
}
bool wxTreebook_AddSubPage(wxTreebook * self, wxWindow * page, const wxString * text, bool b_select, int image_id) {
    return self->AddSubPage(page, *text, b_select, image_id);
}
bool wxTreebook_CollapseNode(wxTreebook * self, size_t page_id) {
    return self->CollapseNode(page_id);
}
bool wxTreebook_Create(wxTreebook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
bool wxTreebook_ExpandNode(wxTreebook * self, size_t page_id, bool expand) {
    return self->ExpandNode(page_id, expand);
}
int wxTreebook_GetPageParent(const wxTreebook * self, size_t page) {
    return self->GetPageParent(page);
}
bool wxTreebook_InsertSubPage(wxTreebook * self, size_t page_pos, wxWindow * page, const wxString * text, bool b_select, int image_id) {
    return self->InsertSubPage(page_pos, page, *text, b_select, image_id);
}
bool wxTreebook_IsNodeExpanded(const wxTreebook * self, size_t page_id) {
    return self->IsNodeExpanded(page_id);
}

} // extern "C"

