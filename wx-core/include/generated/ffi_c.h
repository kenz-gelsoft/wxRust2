#pragma once

#include <wx/calctrl.h>
#include <wx/caret.h>
#include <wx/checkbox.h>
#include <wx/checklst.h>
#include <wx/choice.h>
#include <wx/choicebk.h>
#include <wx/clipbrd.h>
#include <wx/clrpicker.h>
#include <wx/cmdproc.h>
#include <wx/collpane.h>
#include <wx/colordlg.h>
#include <wx/colour.h>
#include <wx/colourdata.h>
#include <wx/combo.h>
#include <wx/combobox.h>
#include <wx/commandlinkbutton.h>
#include <wx/control.h>
#include <wx/ctrlsub.h>
#include <wx/cursor.h>
#include <wx/dataobj.h>
#include <wx/dcclient.h>
#include <wx/event.h>
#include <wx/gdicmn.h>
#include <wx/laywin.h>

extern "C" {

// CLASS: wxCalculateLayoutEvent
wxClassInfo *wxCalculateLayoutEvent_CLASSINFO();
wxCalculateLayoutEvent *wxCalculateLayoutEvent_new(wxWindowID id);
int wxCalculateLayoutEvent_GetFlags(const wxCalculateLayoutEvent * self);
wxRect *wxCalculateLayoutEvent_GetRect(const wxCalculateLayoutEvent * self);
void wxCalculateLayoutEvent_SetFlags(wxCalculateLayoutEvent * self, int flags);
void wxCalculateLayoutEvent_SetRect(wxCalculateLayoutEvent * self, const wxRect * rect);

// CLASS: wxCalendarCtrl
wxClassInfo *wxCalendarCtrl_CLASSINFO();
bool wxCalendarCtrl_SetDateRange(wxCalendarCtrl * self, const wxDateTime * lowerdate, const wxDateTime * upperdate);
bool wxCalendarCtrl_GetDateRange(const wxCalendarCtrl * self, wxDateTime * lowerdate, wxDateTime * upperdate);
wxCalendarCtrl *wxCalendarCtrl_new();
wxCalendarCtrl *wxCalendarCtrl_new1(wxWindow * parent, wxWindowID id, const wxDateTime * date, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxCalendarCtrl_Create(wxCalendarCtrl * self, wxWindow * parent, wxWindowID id, const wxDateTime * date, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxCalendarCtrl_EnableHolidayDisplay(wxCalendarCtrl * self, bool display);
bool wxCalendarCtrl_EnableMonthChange(wxCalendarCtrl * self, bool enable);
wxCalendarDateAttr * wxCalendarCtrl_GetAttr(const wxCalendarCtrl * self, size_t day);
wxDateTime *wxCalendarCtrl_GetDate(const wxCalendarCtrl * self);
wxColour *wxCalendarCtrl_GetHeaderColourBg(const wxCalendarCtrl * self);
wxColour *wxCalendarCtrl_GetHeaderColourFg(const wxCalendarCtrl * self);
wxColour *wxCalendarCtrl_GetHighlightColourBg(const wxCalendarCtrl * self);
wxColour *wxCalendarCtrl_GetHighlightColourFg(const wxCalendarCtrl * self);
wxColour *wxCalendarCtrl_GetHolidayColourBg(const wxCalendarCtrl * self);
wxColour *wxCalendarCtrl_GetHolidayColourFg(const wxCalendarCtrl * self);
void wxCalendarCtrl_ResetAttr(wxCalendarCtrl * self, size_t day);
void wxCalendarCtrl_SetAttr(wxCalendarCtrl * self, size_t day, wxCalendarDateAttr * attr);
bool wxCalendarCtrl_SetDate(wxCalendarCtrl * self, const wxDateTime * date);
void wxCalendarCtrl_SetHeaderColours(wxCalendarCtrl * self, const wxColour * col_fg, const wxColour * col_bg);
void wxCalendarCtrl_SetHighlightColours(wxCalendarCtrl * self, const wxColour * col_fg, const wxColour * col_bg);
void wxCalendarCtrl_SetHoliday(wxCalendarCtrl * self, size_t day);
void wxCalendarCtrl_SetHolidayColours(wxCalendarCtrl * self, const wxColour * col_fg, const wxColour * col_bg);
void wxCalendarCtrl_Mark(wxCalendarCtrl * self, size_t day, bool mark);

// CLASS: wxCalendarDateAttr
void wxCalendarDateAttr_delete(wxCalendarDateAttr *self);
wxColour *wxCalendarDateAttr_GetBackgroundColour(const wxCalendarDateAttr * self);
wxColour *wxCalendarDateAttr_GetBorderColour(const wxCalendarDateAttr * self);
wxFont *wxCalendarDateAttr_GetFont(const wxCalendarDateAttr * self);
wxColour *wxCalendarDateAttr_GetTextColour(const wxCalendarDateAttr * self);
bool wxCalendarDateAttr_HasBackgroundColour(const wxCalendarDateAttr * self);
bool wxCalendarDateAttr_HasBorder(const wxCalendarDateAttr * self);
bool wxCalendarDateAttr_HasBorderColour(const wxCalendarDateAttr * self);
bool wxCalendarDateAttr_HasFont(const wxCalendarDateAttr * self);
bool wxCalendarDateAttr_HasTextColour(const wxCalendarDateAttr * self);
bool wxCalendarDateAttr_IsHoliday(const wxCalendarDateAttr * self);
void wxCalendarDateAttr_SetBackgroundColour(wxCalendarDateAttr * self, const wxColour * col_back);
void wxCalendarDateAttr_SetBorderColour(wxCalendarDateAttr * self, const wxColour * col);
void wxCalendarDateAttr_SetFont(wxCalendarDateAttr * self, const wxFont * font);
void wxCalendarDateAttr_SetHoliday(wxCalendarDateAttr * self, bool holiday);
void wxCalendarDateAttr_SetTextColour(wxCalendarDateAttr * self, const wxColour * col_text);
wxCalendarDateAttr *wxCalendarDateAttr_GetMark();
void wxCalendarDateAttr_SetMark(const wxCalendarDateAttr * m);

// CLASS: wxCalendarEvent
wxClassInfo *wxCalendarEvent_CLASSINFO();
wxCalendarEvent *wxCalendarEvent_new();

// CLASS: wxCaret
void wxCaret_delete(wxCaret *self);
wxCaret *wxCaret_new();
wxCaret *wxCaret_new1(wxWindow * window, int width, int height);
wxCaret *wxCaret_new2(wxWindow * window, const wxSize * size);
bool wxCaret_Create(wxCaret * self, wxWindow * window, int width, int height);
bool wxCaret_Create1(wxCaret * self, wxWindow * window, const wxSize * size);
void wxCaret_GetPosition(const wxCaret * self, int * x, int * y);
wxPoint *wxCaret_GetPosition1(const wxCaret * self);
void wxCaret_GetSize(const wxCaret * self, int * width, int * height);
wxSize *wxCaret_GetSize1(const wxCaret * self);
wxWindow * wxCaret_GetWindow(const wxCaret * self);
void wxCaret_Hide(wxCaret * self);
bool wxCaret_IsOk(const wxCaret * self);
bool wxCaret_IsVisible(const wxCaret * self);
void wxCaret_Move(wxCaret * self, int x, int y);
void wxCaret_Move1(wxCaret * self, const wxPoint * pt);
void wxCaret_SetSize(wxCaret * self, int width, int height);
void wxCaret_SetSize1(wxCaret * self, const wxSize * size);
void wxCaret_Show(wxCaret * self, bool show);
int wxCaret_GetBlinkTime();
void wxCaret_SetBlinkTime(int milliseconds);

// CLASS: wxCheckBox
wxClassInfo *wxCheckBox_CLASSINFO();
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

// CLASS: wxCheckListBox
wxClassInfo *wxCheckListBox_CLASSINFO();
wxCheckListBox *wxCheckListBox_new();
wxCheckListBox *wxCheckListBox_new2(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
bool wxCheckListBox_Create1(wxCheckListBox * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
void wxCheckListBox_Check(wxCheckListBox * self, unsigned int item, bool check);
bool wxCheckListBox_IsChecked(const wxCheckListBox * self, unsigned int item);
unsigned int wxCheckListBox_GetCheckedItems(const wxCheckListBox * self, wxArrayInt * checked_items);
// Mix-in(s) to wxCheckListBox
wxItemContainer *wxCheckListBox_AsItemContainer(wxCheckListBox* obj);

// CLASS: wxChildFocusEvent
wxClassInfo *wxChildFocusEvent_CLASSINFO();
wxChildFocusEvent *wxChildFocusEvent_new(wxWindow * win);
wxWindow * wxChildFocusEvent_GetWindow(const wxChildFocusEvent * self);

// CLASS: wxChoice
wxClassInfo *wxChoice_CLASSINFO();
wxChoice *wxChoice_new();
wxChoice *wxChoice_new2(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
bool wxChoice_Create1(wxChoice * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
int wxChoice_GetColumns(const wxChoice * self);
int wxChoice_GetCurrentSelection(const wxChoice * self);
void wxChoice_SetColumns(wxChoice * self, int n);
bool wxChoice_IsSorted(const wxChoice * self);
// Mix-in(s) to wxChoice
wxItemContainer *wxChoice_AsItemContainer(wxChoice* obj);

// CLASS: wxChoicebook
wxClassInfo *wxChoicebook_CLASSINFO();
wxChoicebook *wxChoicebook_new();
wxChoicebook *wxChoicebook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxChoicebook_Create(wxChoicebook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxChoice * wxChoicebook_GetChoiceCtrl(const wxChoicebook * self);

// CLASS: wxClientDC
wxClassInfo *wxClientDC_CLASSINFO();
wxClientDC *wxClientDC_new(wxWindow * window);

// CLASS: wxClipboard
wxClassInfo *wxClipboard_CLASSINFO();
wxClipboard *wxClipboard_new();
bool wxClipboard_AddData(wxClipboard * self, wxDataObject * data);
void wxClipboard_Clear(wxClipboard * self);
void wxClipboard_Close(wxClipboard * self);
bool wxClipboard_Flush(wxClipboard * self);
bool wxClipboard_GetData(wxClipboard * self, wxDataObject * data);
bool wxClipboard_IsOpened(const wxClipboard * self);
bool wxClipboard_IsSupported(wxClipboard * self, const wxDataFormat * format);
bool wxClipboard_IsUsingPrimarySelection(const wxClipboard * self);
bool wxClipboard_Open(wxClipboard * self);
bool wxClipboard_SetData(wxClipboard * self, wxDataObject * data);
void wxClipboard_UsePrimarySelection(wxClipboard * self, bool primary);
wxClipboard * wxClipboard_Get();

// CLASS: wxClipboardTextEvent
wxClassInfo *wxClipboardTextEvent_CLASSINFO();

// CLASS: wxCloseEvent
wxClassInfo *wxCloseEvent_CLASSINFO();
bool wxCloseEvent_CanVeto(const wxCloseEvent * self);
bool wxCloseEvent_GetLoggingOff(const wxCloseEvent * self);
void wxCloseEvent_SetCanVeto(wxCloseEvent * self, bool can_veto);
void wxCloseEvent_SetLoggingOff(wxCloseEvent * self, bool logging_off);
void wxCloseEvent_Veto(wxCloseEvent * self, bool veto);
bool wxCloseEvent_GetVeto(const wxCloseEvent * self);

// CLASS: wxCollapsiblePane
wxClassInfo *wxCollapsiblePane_CLASSINFO();
wxCollapsiblePane *wxCollapsiblePane_new();
wxCollapsiblePane *wxCollapsiblePane_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxCollapsiblePane_Create(wxCollapsiblePane * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxCollapsiblePane_Collapse(wxCollapsiblePane * self, bool collapse);
void wxCollapsiblePane_Expand(wxCollapsiblePane * self);
wxWindow * wxCollapsiblePane_GetPane(const wxCollapsiblePane * self);
bool wxCollapsiblePane_IsCollapsed(const wxCollapsiblePane * self);
bool wxCollapsiblePane_IsExpanded(const wxCollapsiblePane * self);

// CLASS: wxCollapsiblePaneEvent
wxClassInfo *wxCollapsiblePaneEvent_CLASSINFO();
wxCollapsiblePaneEvent *wxCollapsiblePaneEvent_new(wxObject * generator, int id, bool collapsed);
bool wxCollapsiblePaneEvent_GetCollapsed(const wxCollapsiblePaneEvent * self);
void wxCollapsiblePaneEvent_SetCollapsed(wxCollapsiblePaneEvent * self, bool collapsed);

// CLASS: wxColour
wxClassInfo *wxColour_CLASSINFO();
wxColour *wxColour_new();
wxColour *wxColour_new2(const wxString * colour_name);
wxColour *wxColour_new4(const wxColour * colour);
#if wxCHECK_VERSION(3, 1, 0)
unsigned int wxColour_GetAlpha(const wxColour * self);
unsigned int wxColour_GetBlue(const wxColour * self);
unsigned int wxColour_GetGreen(const wxColour * self);
unsigned int wxColour_GetRed(const wxColour * self);
#endif
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

// CLASS: wxColourData
wxClassInfo *wxColourData_CLASSINFO();
wxColourData *wxColourData_new();
bool wxColourData_GetChooseFull(const wxColourData * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxColourData_GetChooseAlpha(const wxColourData * self);
#endif
wxColour * wxColourData_GetColour(wxColourData * self);
wxColour *wxColourData_GetCustomColour(const wxColourData * self, int i);
void wxColourData_SetChooseFull(wxColourData * self, bool flag);
#if wxCHECK_VERSION(3, 1, 0)
void wxColourData_SetChooseAlpha(wxColourData * self, bool flag);
#endif
void wxColourData_SetColour(wxColourData * self, const wxColour * colour);
void wxColourData_SetCustomColour(wxColourData * self, int i, const wxColour * colour);
wxString *wxColourData_ToString(const wxColourData * self);
bool wxColourData_FromString(wxColourData * self, const wxString * str);

// CLASS: wxColourDatabase
void wxColourDatabase_delete(wxColourDatabase *self);
wxColourDatabase *wxColourDatabase_new();
void wxColourDatabase_AddColour(wxColourDatabase * self, const wxString * colour_name, const wxColour * colour);
wxColour *wxColourDatabase_Find(const wxColourDatabase * self, const wxString * colour_name);
wxString *wxColourDatabase_FindName(const wxColourDatabase * self, const wxColour * colour);

// CLASS: wxColourDialog
wxClassInfo *wxColourDialog_CLASSINFO();
#if wxCHECK_VERSION(3, 1, 0)
wxColourDialog *wxColourDialog_new(wxWindow * parent, const wxColourData * data);
bool wxColourDialog_Create(wxColourDialog * self, wxWindow * parent, const wxColourData * data);
#endif
wxColourData * wxColourDialog_GetColourData(wxColourDialog * self);

// CLASS: wxColourPickerCtrl
wxClassInfo *wxColourPickerCtrl_CLASSINFO();
wxColourPickerCtrl *wxColourPickerCtrl_new();
wxColourPickerCtrl *wxColourPickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxColour * colour, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxColourPickerCtrl_Create(wxColourPickerCtrl * self, wxWindow * parent, wxWindowID id, const wxColour * colour, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
wxColour *wxColourPickerCtrl_GetColour(const wxColourPickerCtrl * self);
void wxColourPickerCtrl_SetColour(wxColourPickerCtrl * self, const wxColour * col);

// CLASS: wxColourPickerEvent
wxClassInfo *wxColourPickerEvent_CLASSINFO();
wxColourPickerEvent *wxColourPickerEvent_new();
wxColourPickerEvent *wxColourPickerEvent_new1(wxObject * generator, int id, const wxColour * colour);
wxColour *wxColourPickerEvent_GetColour(const wxColourPickerEvent * self);
void wxColourPickerEvent_SetColour(wxColourPickerEvent * self, const wxColour * pos);

// CLASS: wxComboBox
wxClassInfo *wxComboBox_CLASSINFO();
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
wxTextEntryBase *wxComboBox_AsTextEntry(wxComboBox* obj);

// CLASS: wxComboCtrl
wxClassInfo *wxComboCtrl_CLASSINFO();
wxComboCtrl *wxComboCtrl_new();
wxComboCtrl *wxComboCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxComboCtrl_Create(wxComboCtrl * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxComboCtrl_Dismiss(wxComboCtrl * self);
void wxComboCtrl_EnablePopupAnimation(wxComboCtrl * self, bool enable);
bool wxComboCtrl_IsKeyPopupToggle(const wxComboCtrl * self, const wxKeyEvent * event);
void wxComboCtrl_PrepareBackground(const wxComboCtrl * self, wxDC * dc, const wxRect * rect, int flags);
bool wxComboCtrl_ShouldDrawFocus(const wxComboCtrl * self);
wxBitmap *wxComboCtrl_GetBitmapDisabled(const wxComboCtrl * self);
wxBitmap *wxComboCtrl_GetBitmapHover(const wxComboCtrl * self);
wxBitmap *wxComboCtrl_GetBitmapNormal(const wxComboCtrl * self);
wxBitmap *wxComboCtrl_GetBitmapPressed(const wxComboCtrl * self);
wxSize *wxComboCtrl_GetButtonSize(wxComboCtrl * self);
int wxComboCtrl_GetCustomPaintWidth(const wxComboCtrl * self);
wxPoint *wxComboCtrl_GetMargins(const wxComboCtrl * self);
wxComboPopup * wxComboCtrl_GetPopupControl(wxComboCtrl * self);
wxWindow * wxComboCtrl_GetPopupWindow(const wxComboCtrl * self);
wxTextCtrl * wxComboCtrl_GetTextCtrl(const wxComboCtrl * self);
wxRect *wxComboCtrl_GetTextRect(const wxComboCtrl * self);
void wxComboCtrl_HidePopup(wxComboCtrl * self, bool generate_event);
bool wxComboCtrl_IsPopupShown(const wxComboCtrl * self);
bool wxComboCtrl_IsPopupWindowState(const wxComboCtrl * self, int state);
void wxComboCtrl_OnButtonClick(wxComboCtrl * self);
void wxComboCtrl_Popup(wxComboCtrl * self);
void wxComboCtrl_SetButtonBitmaps(wxComboCtrl * self, const wxBitmapBundle * bmp_normal, bool push_button_bg, const wxBitmapBundle * bmp_pressed, const wxBitmapBundle * bmp_hover, const wxBitmapBundle * bmp_disabled);
void wxComboCtrl_SetButtonPosition(wxComboCtrl * self, int width, int height, int side, int spacing_x);
void wxComboCtrl_SetCustomPaintWidth(wxComboCtrl * self, int width);
#if wxCHECK_VERSION(3, 1, 0)
void wxComboCtrl_SetMainControl(wxComboCtrl * self, wxWindow * win);
#endif
bool wxComboCtrl_SetMargins(wxComboCtrl * self, const wxPoint * pt);
bool wxComboCtrl_SetMargins1(wxComboCtrl * self, wxCoord left, wxCoord top);
void wxComboCtrl_SetPopupAnchor(wxComboCtrl * self, int anchor_side);
void wxComboCtrl_SetPopupControl(wxComboCtrl * self, wxComboPopup * popup);
void wxComboCtrl_SetPopupExtents(wxComboCtrl * self, int ext_left, int ext_right);
void wxComboCtrl_SetPopupMaxHeight(wxComboCtrl * self, int height);
void wxComboCtrl_SetPopupMinWidth(wxComboCtrl * self, int width);
void wxComboCtrl_SetText(wxComboCtrl * self, const wxString * value);
void wxComboCtrl_SetTextCtrlStyle(wxComboCtrl * self, int style);
void wxComboCtrl_SetValueByUser(wxComboCtrl * self, const wxString * value);
void wxComboCtrl_ShowPopup(wxComboCtrl * self);
void wxComboCtrl_UseAltPopupWindow(wxComboCtrl * self, bool enable);
int wxComboCtrl_GetFeatures();
// Mix-in(s) to wxComboCtrl
wxTextEntryBase *wxComboCtrl_AsTextEntry(wxComboCtrl* obj);

// CLASS: wxComboPopup
void wxComboPopup_delete(wxComboPopup *self);
bool wxComboPopup_Create(wxComboPopup * self, wxWindow * parent);
void wxComboPopup_DestroyPopup(wxComboPopup * self);
void wxComboPopup_Dismiss(wxComboPopup * self);
bool wxComboPopup_FindItem(wxComboPopup * self, const wxString * item, wxString * true_item);
wxSize *wxComboPopup_GetAdjustedSize(wxComboPopup * self, int min_width, int pref_height, int max_height);
wxComboCtrl * wxComboPopup_GetComboCtrl(const wxComboPopup * self);
wxWindow * wxComboPopup_GetControl(wxComboPopup * self);
wxString *wxComboPopup_GetStringValue(const wxComboPopup * self);
void wxComboPopup_Init(wxComboPopup * self);
bool wxComboPopup_IsCreated(const wxComboPopup * self);
bool wxComboPopup_LazyCreate(wxComboPopup * self);
void wxComboPopup_OnComboDoubleClick(wxComboPopup * self);
void wxComboPopup_OnComboKeyEvent(wxComboPopup * self, wxKeyEvent * event);
void wxComboPopup_OnDismiss(wxComboPopup * self);
void wxComboPopup_OnPopup(wxComboPopup * self);
void wxComboPopup_PaintComboControl(wxComboPopup * self, wxDC * dc, const wxRect * rect);
void wxComboPopup_SetStringValue(wxComboPopup * self, const wxString * value);

// CLASS: wxCommand
wxClassInfo *wxCommand_CLASSINFO();
bool wxCommand_CanUndo(const wxCommand * self);
bool wxCommand_Do(wxCommand * self);
wxString *wxCommand_GetName(const wxCommand * self);
bool wxCommand_Undo(wxCommand * self);

// CLASS: wxCommandEvent
wxClassInfo *wxCommandEvent_CLASSINFO();
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

// CLASS: wxCommandLinkButton
wxClassInfo *wxCommandLinkButton_CLASSINFO();
wxCommandLinkButton *wxCommandLinkButton_new();
wxCommandLinkButton *wxCommandLinkButton_new1(wxWindow * parent, wxWindowID id, const wxString * main_label, const wxString * note, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxCommandLinkButton_Create(wxCommandLinkButton * self, wxWindow * parent, wxWindowID id, const wxString * main_label, const wxString * note, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxCommandLinkButton_SetMainLabelAndNote(wxCommandLinkButton * self, const wxString * main_label, const wxString * note);
void wxCommandLinkButton_SetMainLabel(wxCommandLinkButton * self, const wxString * main_label);
void wxCommandLinkButton_SetNote(wxCommandLinkButton * self, const wxString * note);
wxString *wxCommandLinkButton_GetMainLabel(const wxCommandLinkButton * self);
wxString *wxCommandLinkButton_GetNote(const wxCommandLinkButton * self);

// CLASS: wxCommandProcessor
wxClassInfo *wxCommandProcessor_CLASSINFO();
wxCommandProcessor *wxCommandProcessor_new(int max_commands);
bool wxCommandProcessor_CanUndo(const wxCommandProcessor * self);
bool wxCommandProcessor_CanRedo(const wxCommandProcessor * self);
void wxCommandProcessor_ClearCommands(wxCommandProcessor * self);
wxCommand * wxCommandProcessor_GetCurrentCommand(const wxCommandProcessor * self);
wxMenu * wxCommandProcessor_GetEditMenu(const wxCommandProcessor * self);
int wxCommandProcessor_GetMaxCommands(const wxCommandProcessor * self);
wxString *wxCommandProcessor_GetRedoAccelerator(const wxCommandProcessor * self);
wxString *wxCommandProcessor_GetRedoMenuLabel(const wxCommandProcessor * self);
wxString *wxCommandProcessor_GetUndoAccelerator(const wxCommandProcessor * self);
wxString *wxCommandProcessor_GetUndoMenuLabel(const wxCommandProcessor * self);
void wxCommandProcessor_Initialize(wxCommandProcessor * self);
bool wxCommandProcessor_IsDirty(const wxCommandProcessor * self);
void wxCommandProcessor_MarkAsSaved(wxCommandProcessor * self);
bool wxCommandProcessor_Redo(wxCommandProcessor * self);
void wxCommandProcessor_SetEditMenu(wxCommandProcessor * self, wxMenu * menu);
void wxCommandProcessor_SetMenuStrings(wxCommandProcessor * self);
void wxCommandProcessor_SetRedoAccelerator(wxCommandProcessor * self, const wxString * accel);
void wxCommandProcessor_SetUndoAccelerator(wxCommandProcessor * self, const wxString * accel);
bool wxCommandProcessor_Submit(wxCommandProcessor * self, wxCommand * command, bool store_it);
void wxCommandProcessor_Store(wxCommandProcessor * self, wxCommand * command);
bool wxCommandProcessor_Undo(wxCommandProcessor * self);

// CLASS: wxContextMenuEvent
wxClassInfo *wxContextMenuEvent_CLASSINFO();
wxPoint *wxContextMenuEvent_GetPosition(const wxContextMenuEvent * self);
void wxContextMenuEvent_SetPosition(wxContextMenuEvent * self, const wxPoint * point);

// CLASS: wxControl
wxClassInfo *wxControl_CLASSINFO();
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

// CLASS: wxControlWithItems
wxClassInfo *wxControlWithItems_CLASSINFO();
// Mix-in(s) to wxControlWithItems
wxItemContainer *wxControlWithItems_AsItemContainer(wxControlWithItems* obj);

// CLASS: wxCursor
wxClassInfo *wxCursor_CLASSINFO();
wxCursor *wxCursor_new();
wxCursor *wxCursor_new4(const wxImage * image);
wxCursor *wxCursor_new5(const char *const * xpm_data);
wxCursor *wxCursor_new6(const wxCursor * cursor);
bool wxCursor_IsOk(const wxCursor * self);
#if wxCHECK_VERSION(3, 1, 0)
wxPoint *wxCursor_GetHotSpot(const wxCursor * self);
#endif

// CLASS: wxCustomDataObject
void wxCustomDataObject_delete(wxCustomDataObject *self);
wxCustomDataObject *wxCustomDataObject_new(const wxDataFormat * format);
void * wxCustomDataObject_Alloc(wxCustomDataObject * self, size_t size);
void wxCustomDataObject_Free(wxCustomDataObject * self);
void * wxCustomDataObject_GetData(const wxCustomDataObject * self);
size_t wxCustomDataObject_GetSize(const wxCustomDataObject * self);
void wxCustomDataObject_TakeData(wxCustomDataObject * self, size_t size, void * data);

} // extern "C"

