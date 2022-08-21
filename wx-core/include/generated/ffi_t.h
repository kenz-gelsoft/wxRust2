#pragma once

#include <wx/dataobj.h>
#include <wx/dnd.h>
#include <wx/event.h>
#include <wx/imagtga.h>
#include <wx/imagtiff.h>
#include <wx/taskbar.h>
#include <wx/textctrl.h>
#include <wx/textdlg.h>
#include <wx/textentry.h>
#include <wx/tglbtn.h>
#include <wx/timectrl.h>
#include <wx/tipdlg.h>
#include <wx/tipwin.h>
#include <wx/toolbar.h>
#include <wx/toolbook.h>
#include <wx/tooltip.h>
#include <wx/toplevel.h>
#include <wx/treebase.h>
#include <wx/treebook.h>
#include <wx/treectrl.h>
#include <wx/treelist.h>
#include <wx/valtext.h>

extern "C" {

// CLASS: wxTGAHandler
wxClassInfo *wxTGAHandler_CLASSINFO();
wxTGAHandler *wxTGAHandler_new();

// CLASS: wxTIFFHandler
wxClassInfo *wxTIFFHandler_CLASSINFO();
wxTIFFHandler *wxTIFFHandler_new();

// CLASS: wxTaskBarIcon
wxClassInfo *wxTaskBarIcon_CLASSINFO();
void wxTaskBarIcon_Destroy(wxTaskBarIcon * self);
bool wxTaskBarIcon_IsIconInstalled(const wxTaskBarIcon * self);
bool wxTaskBarIcon_IsOk(const wxTaskBarIcon * self);
bool wxTaskBarIcon_PopupMenu(wxTaskBarIcon * self, wxMenu * menu);
bool wxTaskBarIcon_RemoveIcon(wxTaskBarIcon * self);
bool wxTaskBarIcon_SetIcon(wxTaskBarIcon * self, const wxBitmapBundle * icon, const wxString * tooltip);
bool wxTaskBarIcon_IsAvailable();

// CLASS: wxTaskBarIconEvent
wxClassInfo *wxTaskBarIconEvent_CLASSINFO();

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
wxFont *wxTextAttr_GetFont(const wxTextAttr * self);
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
wxArrayInt *wxTextAttr_GetTabs(const wxTextAttr * self);
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
wxClassInfo *wxTextCtrl_CLASSINFO();
#ifdef __WXOSX__
void wxTextCtrl_OSXEnableNewLineReplacement(wxTextCtrl * self, bool enable);
#endif
wxTextCtrl *wxTextCtrl_new();
wxTextCtrl *wxTextCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxTextCtrl_Create(wxTextCtrl * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxTextCtrl_DiscardEdits(wxTextCtrl * self);
#if wxCHECK_VERSION(3, 1, 0)
void wxTextCtrl_EmptyUndoBuffer(wxTextCtrl * self);
#endif
bool wxTextCtrl_EmulateKeyPress(wxTextCtrl * self, const wxKeyEvent * event);
#ifndef __WXGTK__
bool wxTextCtrl_EnableProofCheck(wxTextCtrl * self, const wxTextProofOptions * options);
#endif
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
wxTextEntryBase *wxTextCtrl_AsTextEntry(wxTextCtrl* obj);

// CLASS: wxTextDataObject
void wxTextDataObject_delete(wxTextDataObject *self);
wxTextDataObject *wxTextDataObject_new(const wxString * text);
wxString *wxTextDataObject_GetText(const wxTextDataObject * self);
size_t wxTextDataObject_GetTextLength(const wxTextDataObject * self);
void wxTextDataObject_SetText(wxTextDataObject * self, const wxString * str_text);

// CLASS: wxTextDropTarget
void wxTextDropTarget_delete(wxTextDropTarget *self);
bool wxTextDropTarget_OnDropText(wxTextDropTarget * self, wxCoord x, wxCoord y, const wxString * data);

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

// CLASS: wxTextEntryDialog
wxClassInfo *wxTextEntryDialog_CLASSINFO();
wxTextEntryDialog *wxTextEntryDialog_new();
wxTextEntryDialog *wxTextEntryDialog_new1(wxWindow * parent, const wxString * message, const wxString * caption, const wxString * value, long style, const wxPoint * pos);
bool wxTextEntryDialog_Create(wxTextEntryDialog * self, wxWindow * parent, const wxString * message, const wxString * caption, const wxString * value, long style, const wxPoint * pos);
wxString *wxTextEntryDialog_GetValue(const wxTextEntryDialog * self);
void wxTextEntryDialog_SetTextValidator(wxTextEntryDialog * self, const wxTextValidator * validator);
void wxTextEntryDialog_SetValue(wxTextEntryDialog * self, const wxString * value);
#if wxCHECK_VERSION(3, 1, 0)
void wxTextEntryDialog_ForceUpper(wxTextEntryDialog * self);
#endif

// CLASS: wxTextValidator
wxClassInfo *wxTextValidator_CLASSINFO();
wxTextValidator *wxTextValidator_new(const wxTextValidator * validator);
wxTextValidator *wxTextValidator_new1(long style, wxString * val_ptr);
wxString *wxTextValidator_GetCharExcludes(const wxTextValidator * self);
wxString *wxTextValidator_GetCharIncludes(const wxTextValidator * self);
wxArrayString *wxTextValidator_GetExcludes(const wxTextValidator * self);
wxArrayString *wxTextValidator_GetIncludes(const wxTextValidator * self);
long wxTextValidator_GetStyle(const wxTextValidator * self);
void wxTextValidator_OnChar(wxTextValidator * self, wxKeyEvent * event);
void wxTextValidator_SetExcludes(wxTextValidator * self, const wxArrayString * string_list);
void wxTextValidator_SetCharExcludes(wxTextValidator * self, const wxString * chars);
void wxTextValidator_SetIncludes(wxTextValidator * self, const wxArrayString * string_list);
void wxTextValidator_SetCharIncludes(wxTextValidator * self, const wxString * chars);
void wxTextValidator_AddExclude(wxTextValidator * self, const wxString * exclude);
void wxTextValidator_AddInclude(wxTextValidator * self, const wxString * include);
void wxTextValidator_AddCharExcludes(wxTextValidator * self, const wxString * chars);
void wxTextValidator_AddCharIncludes(wxTextValidator * self, const wxString * chars);
void wxTextValidator_SetStyle(wxTextValidator * self, long style);
wxString *wxTextValidator_IsValid(const wxTextValidator * self, const wxString * val);

// CLASS: wxThreadEvent
wxClassInfo *wxThreadEvent_CLASSINFO();
long wxThreadEvent_GetExtraLong(const wxThreadEvent * self);
int wxThreadEvent_GetInt(const wxThreadEvent * self);
wxString *wxThreadEvent_GetString(const wxThreadEvent * self);
void wxThreadEvent_SetExtraLong(wxThreadEvent * self, long extra_long);
void wxThreadEvent_SetInt(wxThreadEvent * self, int int_command);
void wxThreadEvent_SetString(wxThreadEvent * self, const wxString * string);

// CLASS: wxTimePickerCtrl
wxClassInfo *wxTimePickerCtrl_CLASSINFO();
wxTimePickerCtrl *wxTimePickerCtrl_new();
wxTimePickerCtrl *wxTimePickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxTimePickerCtrl_Create(wxTimePickerCtrl * self, wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxTimePickerCtrl_GetTime(const wxTimePickerCtrl * self, int * hour, int * min, int * sec);
wxDateTime *wxTimePickerCtrl_GetValue(const wxTimePickerCtrl * self);
bool wxTimePickerCtrl_SetTime(wxTimePickerCtrl * self, int hour, int min, int sec);
void wxTimePickerCtrl_SetValue(wxTimePickerCtrl * self, const wxDateTime * dt);

// CLASS: wxTipProvider
void wxTipProvider_delete(wxTipProvider *self);
size_t wxTipProvider_GetCurrentTip(const wxTipProvider * self);
wxString *wxTipProvider_GetTip(wxTipProvider * self);

// CLASS: wxTipWindow
wxClassInfo *wxTipWindow_CLASSINFO();
wxTipWindow *wxTipWindow_new(wxWindow * parent, const wxString * text, wxCoord max_length, wxTipWindow ** window_ptr, wxRect * rect_bounds);
void wxTipWindow_SetBoundingRect(wxTipWindow * self, const wxRect * rect_bound);
void wxTipWindow_SetTipWindowPtr(wxTipWindow * self, wxTipWindow ** window_ptr);

// CLASS: wxToggleButton
wxClassInfo *wxToggleButton_CLASSINFO();
wxToggleButton *wxToggleButton_new();
wxToggleButton *wxToggleButton_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * val, const wxString * name);
bool wxToggleButton_Create(wxToggleButton * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * val, const wxString * name);
bool wxToggleButton_GetValue(const wxToggleButton * self);
void wxToggleButton_SetValue(wxToggleButton * self, bool state);

// CLASS: wxToolBar
wxClassInfo *wxToolBar_CLASSINFO();
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

// CLASS: wxToolTip
wxClassInfo *wxToolTip_CLASSINFO();
wxToolTip *wxToolTip_new(const wxString * tip);
wxString *wxToolTip_GetTip(const wxToolTip * self);
wxWindow * wxToolTip_GetWindow(const wxToolTip * self);
void wxToolTip_SetTip(wxToolTip * self, const wxString * tip);
void wxToolTip_Enable(bool flag);
void wxToolTip_SetAutoPop(long msecs);
void wxToolTip_SetDelay(long msecs);
#ifdef __WXMSW__
void wxToolTip_SetMaxWidth(int width);
#endif
void wxToolTip_SetReshow(long msecs);

// CLASS: wxToolbook
wxClassInfo *wxToolbook_CLASSINFO();
wxToolbook *wxToolbook_new();
wxToolbook *wxToolbook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxToolbook_Create(wxToolbook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxToolBarBase * wxToolbook_GetToolBar(const wxToolbook * self);
bool wxToolbook_EnablePage(wxToolbook * self, size_t page, bool enable);
bool wxToolbook_EnablePage1(wxToolbook * self, wxWindow * page, bool enable);

// CLASS: wxTopLevelWindow
wxClassInfo *wxTopLevelWindow_CLASSINFO();
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

// CLASS: wxTreeCtrl
wxClassInfo *wxTreeCtrl_CLASSINFO();
wxTreeCtrl *wxTreeCtrl_new();
wxTreeCtrl *wxTreeCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
wxTreeItemId *wxTreeCtrl_AddRoot(wxTreeCtrl * self, const wxString * text, int image, int sel_image, wxTreeItemData * data);
wxTreeItemId *wxTreeCtrl_AppendItem(wxTreeCtrl * self, const wxTreeItemId * parent, const wxString * text, int image, int sel_image, wxTreeItemData * data);
void wxTreeCtrl_AssignButtonsImageList(wxTreeCtrl * self, wxImageList * image_list);
void wxTreeCtrl_AssignStateImageList(wxTreeCtrl * self, wxImageList * image_list);
void wxTreeCtrl_Collapse(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_CollapseAll(wxTreeCtrl * self);
void wxTreeCtrl_CollapseAllChildren(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_CollapseAndReset(wxTreeCtrl * self, const wxTreeItemId * item);
bool wxTreeCtrl_Create(wxTreeCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxTreeCtrl_Delete(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_DeleteAllItems(wxTreeCtrl * self);
void wxTreeCtrl_DeleteChildren(wxTreeCtrl * self, const wxTreeItemId * item);
wxTextCtrl * wxTreeCtrl_EditLabel(wxTreeCtrl * self, const wxTreeItemId * item, wxClassInfo * text_ctrl_class);
void wxTreeCtrl_EnableBellOnNoMatch(wxTreeCtrl * self, bool on);
void wxTreeCtrl_EndEditLabel(wxTreeCtrl * self, const wxTreeItemId * item, bool discard_changes);
void wxTreeCtrl_EnsureVisible(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_Expand(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_ExpandAll(wxTreeCtrl * self);
void wxTreeCtrl_ExpandAllChildren(wxTreeCtrl * self, const wxTreeItemId * item);
bool wxTreeCtrl_GetBoundingRect(const wxTreeCtrl * self, const wxTreeItemId * item, wxRect * rect, bool text_only);
wxImageList * wxTreeCtrl_GetButtonsImageList(const wxTreeCtrl * self);
size_t wxTreeCtrl_GetChildrenCount(const wxTreeCtrl * self, const wxTreeItemId * item, bool recursively);
unsigned int wxTreeCtrl_GetCount(const wxTreeCtrl * self);
wxTextCtrl * wxTreeCtrl_GetEditControl(const wxTreeCtrl * self);
wxTreeItemId *wxTreeCtrl_GetFirstChild(const wxTreeCtrl * self, const wxTreeItemId * item, wxTreeItemIdValue * cookie);
wxTreeItemId *wxTreeCtrl_GetFirstVisibleItem(const wxTreeCtrl * self);
wxTreeItemId *wxTreeCtrl_GetFocusedItem(const wxTreeCtrl * self);
void wxTreeCtrl_ClearFocusedItem(wxTreeCtrl * self);
void wxTreeCtrl_SetFocusedItem(wxTreeCtrl * self, const wxTreeItemId * item);
unsigned int wxTreeCtrl_GetIndent(const wxTreeCtrl * self);
unsigned int wxTreeCtrl_GetSpacing(const wxTreeCtrl * self);
wxColour *wxTreeCtrl_GetItemBackgroundColour(const wxTreeCtrl * self, const wxTreeItemId * item);
wxTreeItemData * wxTreeCtrl_GetItemData(const wxTreeCtrl * self, const wxTreeItemId * item);
wxFont *wxTreeCtrl_GetItemFont(const wxTreeCtrl * self, const wxTreeItemId * item);
wxTreeItemId *wxTreeCtrl_GetItemParent(const wxTreeCtrl * self, const wxTreeItemId * item);
int wxTreeCtrl_GetItemState(const wxTreeCtrl * self, const wxTreeItemId * item);
wxString *wxTreeCtrl_GetItemText(const wxTreeCtrl * self, const wxTreeItemId * item);
wxColour *wxTreeCtrl_GetItemTextColour(const wxTreeCtrl * self, const wxTreeItemId * item);
wxTreeItemId *wxTreeCtrl_GetLastChild(const wxTreeCtrl * self, const wxTreeItemId * item);
wxTreeItemId *wxTreeCtrl_GetNextChild(const wxTreeCtrl * self, const wxTreeItemId * item, wxTreeItemIdValue * cookie);
wxTreeItemId *wxTreeCtrl_GetNextSibling(const wxTreeCtrl * self, const wxTreeItemId * item);
wxTreeItemId *wxTreeCtrl_GetNextVisible(const wxTreeCtrl * self, const wxTreeItemId * item);
wxTreeItemId *wxTreeCtrl_GetPrevSibling(const wxTreeCtrl * self, const wxTreeItemId * item);
wxTreeItemId *wxTreeCtrl_GetPrevVisible(const wxTreeCtrl * self, const wxTreeItemId * item);
bool wxTreeCtrl_GetQuickBestSize(const wxTreeCtrl * self);
wxTreeItemId *wxTreeCtrl_GetRootItem(const wxTreeCtrl * self);
wxTreeItemId *wxTreeCtrl_GetSelection(const wxTreeCtrl * self);
size_t wxTreeCtrl_GetSelections(const wxTreeCtrl * self, wxArrayTreeItemIds * selection);
wxImageList * wxTreeCtrl_GetStateImageList(const wxTreeCtrl * self);
wxTreeItemId *wxTreeCtrl_HitTest(const wxTreeCtrl * self, const wxPoint * point, int * flags);
wxTreeItemId *wxTreeCtrl_InsertItem(wxTreeCtrl * self, const wxTreeItemId * parent, const wxTreeItemId * previous, const wxString * text, int image, int sel_image, wxTreeItemData * data);
wxTreeItemId *wxTreeCtrl_InsertItem1(wxTreeCtrl * self, const wxTreeItemId * parent, size_t pos, const wxString * text, int image, int sel_image, wxTreeItemData * data);
bool wxTreeCtrl_IsBold(const wxTreeCtrl * self, const wxTreeItemId * item);
bool wxTreeCtrl_IsEmpty(const wxTreeCtrl * self);
bool wxTreeCtrl_IsExpanded(const wxTreeCtrl * self, const wxTreeItemId * item);
bool wxTreeCtrl_IsSelected(const wxTreeCtrl * self, const wxTreeItemId * item);
bool wxTreeCtrl_IsVisible(const wxTreeCtrl * self, const wxTreeItemId * item);
bool wxTreeCtrl_ItemHasChildren(const wxTreeCtrl * self, const wxTreeItemId * item);
int wxTreeCtrl_OnCompareItems(wxTreeCtrl * self, const wxTreeItemId * item1, const wxTreeItemId * item2);
wxTreeItemId *wxTreeCtrl_PrependItem(wxTreeCtrl * self, const wxTreeItemId * parent, const wxString * text, int image, int sel_image, wxTreeItemData * data);
void wxTreeCtrl_ScrollTo(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_SelectItem(wxTreeCtrl * self, const wxTreeItemId * item, bool select);
void wxTreeCtrl_SetButtonsImageList(wxTreeCtrl * self, wxImageList * image_list);
void wxTreeCtrl_SetIndent(wxTreeCtrl * self, unsigned int indent);
void wxTreeCtrl_SetSpacing(wxTreeCtrl * self, unsigned int spacing);
void wxTreeCtrl_SetItemBackgroundColour(wxTreeCtrl * self, const wxTreeItemId * item, const wxColour * col);
void wxTreeCtrl_SetItemBold(wxTreeCtrl * self, const wxTreeItemId * item, bool bold);
void wxTreeCtrl_SetItemData(wxTreeCtrl * self, const wxTreeItemId * item, wxTreeItemData * data);
void wxTreeCtrl_SetItemDropHighlight(wxTreeCtrl * self, const wxTreeItemId * item, bool highlight);
void wxTreeCtrl_SetItemFont(wxTreeCtrl * self, const wxTreeItemId * item, const wxFont * font);
void wxTreeCtrl_SetItemHasChildren(wxTreeCtrl * self, const wxTreeItemId * item, bool has_children);
void wxTreeCtrl_SetItemState(wxTreeCtrl * self, const wxTreeItemId * item, int state);
void wxTreeCtrl_SetItemText(wxTreeCtrl * self, const wxTreeItemId * item, const wxString * text);
void wxTreeCtrl_SetItemTextColour(wxTreeCtrl * self, const wxTreeItemId * item, const wxColour * col);
void wxTreeCtrl_SetQuickBestSize(wxTreeCtrl * self, bool quick_best_size);
void wxTreeCtrl_SetStateImageList(wxTreeCtrl * self, wxImageList * image_list);
void wxTreeCtrl_SetWindowStyle(wxTreeCtrl * self, long styles);
void wxTreeCtrl_SortChildren(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_Toggle(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_ToggleItemSelection(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_Unselect(wxTreeCtrl * self);
void wxTreeCtrl_UnselectAll(wxTreeCtrl * self);
void wxTreeCtrl_UnselectItem(wxTreeCtrl * self, const wxTreeItemId * item);
void wxTreeCtrl_SelectChildren(wxTreeCtrl * self, const wxTreeItemId * parent);

// CLASS: wxTreeEvent
wxClassInfo *wxTreeEvent_CLASSINFO();
wxTreeItemId *wxTreeEvent_GetItem(const wxTreeEvent * self);
int wxTreeEvent_GetKeyCode(const wxTreeEvent * self);
wxKeyEvent *wxTreeEvent_GetKeyEvent(const wxTreeEvent * self);
wxString *wxTreeEvent_GetLabel(const wxTreeEvent * self);
wxTreeItemId *wxTreeEvent_GetOldItem(const wxTreeEvent * self);
wxPoint *wxTreeEvent_GetPoint(const wxTreeEvent * self);
bool wxTreeEvent_IsEditCancelled(const wxTreeEvent * self);
void wxTreeEvent_SetToolTip(wxTreeEvent * self, const wxString * tooltip);

// CLASS: wxTreeItemData
void wxTreeItemData_delete(wxTreeItemData *self);
wxTreeItemData *wxTreeItemData_new();
wxTreeItemId *wxTreeItemData_GetId(const wxTreeItemData * self);
void wxTreeItemData_SetId(wxTreeItemData * self, const wxTreeItemId * id);

// CLASS: wxTreeItemId
void wxTreeItemId_delete(wxTreeItemId *self);
wxTreeItemId *wxTreeItemId_new();
bool wxTreeItemId_IsOk(const wxTreeItemId * self);
void * wxTreeItemId_GetID(const wxTreeItemId * self);
void wxTreeItemId_Unset(wxTreeItemId * self);

// CLASS: wxTreeListCtrl
wxClassInfo *wxTreeListCtrl_CLASSINFO();
void wxTreeListCtrl_AssignImageList(wxTreeListCtrl * self, wxImageList * image_list);
void wxTreeListCtrl_SetImageList(wxTreeListCtrl * self, wxImageList * image_list);
int wxTreeListCtrl_AppendColumn(wxTreeListCtrl * self, const wxString * title, int width, wxAlignment align, int flags);
void wxTreeListCtrl_ClearColumns(wxTreeListCtrl * self);
int wxTreeListCtrl_WidthFor(const wxTreeListCtrl * self, const wxString * text);
void wxTreeListCtrl_DeleteAllItems(wxTreeListCtrl * self);
wxTreeListItem *wxTreeListCtrl_GetRootItem(const wxTreeListCtrl * self);
wxTreeListItem *wxTreeListCtrl_GetFirstItem(const wxTreeListCtrl * self);
wxTreeListItem *wxTreeListCtrl_GetSelection(const wxTreeListCtrl * self);
void wxTreeListCtrl_SelectAll(wxTreeListCtrl * self);
void wxTreeListCtrl_UnselectAll(wxTreeListCtrl * self);
bool wxTreeListCtrl_GetSortColumn(wxTreeListCtrl * self, unsigned * col, bool * ascending_order);
void wxTreeListCtrl_SetItemComparator(wxTreeListCtrl * self, wxTreeListItemComparator * comparator);
wxWindow * wxTreeListCtrl_GetView(const wxTreeListCtrl * self);
wxDataViewCtrl * wxTreeListCtrl_GetDataView(const wxTreeListCtrl * self);
wxTreeListCtrl *wxTreeListCtrl_new();
wxTreeListCtrl *wxTreeListCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxTreeListCtrl_Create(wxTreeListCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);

// CLASS: wxTreeListItem
void wxTreeListItem_delete(wxTreeListItem *self);
wxTreeListItem *wxTreeListItem_new();
bool wxTreeListItem_IsOk(const wxTreeListItem * self);

// CLASS: wxTreeListItemComparator
void wxTreeListItemComparator_delete(wxTreeListItemComparator *self);

// CLASS: wxTreebook
wxClassInfo *wxTreebook_CLASSINFO();
wxTreebook *wxTreebook_new();
wxTreebook *wxTreebook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxTreebook_AddSubPage(wxTreebook * self, wxWindow * page, const wxString * text, bool b_select, int image_id);
bool wxTreebook_CollapseNode(wxTreebook * self, size_t page_id);
bool wxTreebook_Create(wxTreebook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxTreebook_ExpandNode(wxTreebook * self, size_t page_id, bool expand);
int wxTreebook_GetPageParent(const wxTreebook * self, size_t page);
bool wxTreebook_InsertSubPage(wxTreebook * self, size_t page_pos, wxWindow * page, const wxString * text, bool b_select, int image_id);
bool wxTreebook_IsNodeExpanded(const wxTreebook * self, size_t page_id);

} // extern "C"

