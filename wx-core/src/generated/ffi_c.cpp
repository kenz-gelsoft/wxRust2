#include "generated.h"

extern "C" {

// CLASS: wxCalculateLayoutEvent
wxClassInfo *wxCalculateLayoutEvent_CLASSINFO() {
    return wxCLASSINFO(wxCalculateLayoutEvent);
}
wxCalculateLayoutEvent *wxCalculateLayoutEvent_new(wxWindowID id) {
    return new wxCalculateLayoutEvent(id);
}
int wxCalculateLayoutEvent_GetFlags(const wxCalculateLayoutEvent * self) {
    return self->GetFlags();
}
wxRect *wxCalculateLayoutEvent_GetRect(const wxCalculateLayoutEvent * self) {
    return new wxRect(self->GetRect());
}
void wxCalculateLayoutEvent_SetFlags(wxCalculateLayoutEvent * self, int flags) {
    return self->SetFlags(flags);
}
void wxCalculateLayoutEvent_SetRect(wxCalculateLayoutEvent * self, const wxRect * rect) {
    return self->SetRect(*rect);
}

// CLASS: wxCalendarCtrl
wxClassInfo *wxCalendarCtrl_CLASSINFO() {
    return wxCLASSINFO(wxCalendarCtrl);
}
bool wxCalendarCtrl_SetDateRange(wxCalendarCtrl * self, const wxDateTime * lowerdate, const wxDateTime * upperdate) {
    return self->SetDateRange(*lowerdate, *upperdate);
}
bool wxCalendarCtrl_GetDateRange(const wxCalendarCtrl * self, wxDateTime * lowerdate, wxDateTime * upperdate) {
    return self->GetDateRange(lowerdate, upperdate);
}
wxCalendarCtrl *wxCalendarCtrl_new() {
    return new wxCalendarCtrl();
}
wxCalendarCtrl *wxCalendarCtrl_new1(wxWindow * parent, wxWindowID id, const wxDateTime * date, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxCalendarCtrl(parent, id, *date, *pos, *size, style, *name);
}
bool wxCalendarCtrl_Create(wxCalendarCtrl * self, wxWindow * parent, wxWindowID id, const wxDateTime * date, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *date, *pos, *size, style, *name);
}
void wxCalendarCtrl_EnableHolidayDisplay(wxCalendarCtrl * self, bool display) {
    return self->EnableHolidayDisplay(display);
}
bool wxCalendarCtrl_EnableMonthChange(wxCalendarCtrl * self, bool enable) {
    return self->EnableMonthChange(enable);
}
wxCalendarDateAttr * wxCalendarCtrl_GetAttr(const wxCalendarCtrl * self, size_t day) {
    return self->GetAttr(day);
}
wxDateTime *wxCalendarCtrl_GetDate(const wxCalendarCtrl * self) {
    return new wxDateTime(self->GetDate());
}
wxColour *wxCalendarCtrl_GetHeaderColourBg(const wxCalendarCtrl * self) {
    return new wxColour(self->GetHeaderColourBg());
}
wxColour *wxCalendarCtrl_GetHeaderColourFg(const wxCalendarCtrl * self) {
    return new wxColour(self->GetHeaderColourFg());
}
wxColour *wxCalendarCtrl_GetHighlightColourBg(const wxCalendarCtrl * self) {
    return new wxColour(self->GetHighlightColourBg());
}
wxColour *wxCalendarCtrl_GetHighlightColourFg(const wxCalendarCtrl * self) {
    return new wxColour(self->GetHighlightColourFg());
}
wxColour *wxCalendarCtrl_GetHolidayColourBg(const wxCalendarCtrl * self) {
    return new wxColour(self->GetHolidayColourBg());
}
wxColour *wxCalendarCtrl_GetHolidayColourFg(const wxCalendarCtrl * self) {
    return new wxColour(self->GetHolidayColourFg());
}
void wxCalendarCtrl_ResetAttr(wxCalendarCtrl * self, size_t day) {
    return self->ResetAttr(day);
}
void wxCalendarCtrl_SetAttr(wxCalendarCtrl * self, size_t day, wxCalendarDateAttr * attr) {
    return self->SetAttr(day, attr);
}
bool wxCalendarCtrl_SetDate(wxCalendarCtrl * self, const wxDateTime * date) {
    return self->SetDate(*date);
}
void wxCalendarCtrl_SetHeaderColours(wxCalendarCtrl * self, const wxColour * col_fg, const wxColour * col_bg) {
    return self->SetHeaderColours(*col_fg, *col_bg);
}
void wxCalendarCtrl_SetHighlightColours(wxCalendarCtrl * self, const wxColour * col_fg, const wxColour * col_bg) {
    return self->SetHighlightColours(*col_fg, *col_bg);
}
void wxCalendarCtrl_SetHoliday(wxCalendarCtrl * self, size_t day) {
    return self->SetHoliday(day);
}
void wxCalendarCtrl_SetHolidayColours(wxCalendarCtrl * self, const wxColour * col_fg, const wxColour * col_bg) {
    return self->SetHolidayColours(*col_fg, *col_bg);
}
void wxCalendarCtrl_Mark(wxCalendarCtrl * self, size_t day, bool mark) {
    return self->Mark(day, mark);
}

// CLASS: wxCalendarDateAttr
void wxCalendarDateAttr_delete(wxCalendarDateAttr *self) {
    delete self;
}
wxColour *wxCalendarDateAttr_GetBackgroundColour(const wxCalendarDateAttr * self) {
    return new wxColour(self->GetBackgroundColour());
}
wxColour *wxCalendarDateAttr_GetBorderColour(const wxCalendarDateAttr * self) {
    return new wxColour(self->GetBorderColour());
}
wxFont *wxCalendarDateAttr_GetFont(const wxCalendarDateAttr * self) {
    return new wxFont(self->GetFont());
}
wxColour *wxCalendarDateAttr_GetTextColour(const wxCalendarDateAttr * self) {
    return new wxColour(self->GetTextColour());
}
bool wxCalendarDateAttr_HasBackgroundColour(const wxCalendarDateAttr * self) {
    return self->HasBackgroundColour();
}
bool wxCalendarDateAttr_HasBorder(const wxCalendarDateAttr * self) {
    return self->HasBorder();
}
bool wxCalendarDateAttr_HasBorderColour(const wxCalendarDateAttr * self) {
    return self->HasBorderColour();
}
bool wxCalendarDateAttr_HasFont(const wxCalendarDateAttr * self) {
    return self->HasFont();
}
bool wxCalendarDateAttr_HasTextColour(const wxCalendarDateAttr * self) {
    return self->HasTextColour();
}
bool wxCalendarDateAttr_IsHoliday(const wxCalendarDateAttr * self) {
    return self->IsHoliday();
}
void wxCalendarDateAttr_SetBackgroundColour(wxCalendarDateAttr * self, const wxColour * col_back) {
    return self->SetBackgroundColour(*col_back);
}
void wxCalendarDateAttr_SetBorderColour(wxCalendarDateAttr * self, const wxColour * col) {
    return self->SetBorderColour(*col);
}
void wxCalendarDateAttr_SetFont(wxCalendarDateAttr * self, const wxFont * font) {
    return self->SetFont(*font);
}
void wxCalendarDateAttr_SetHoliday(wxCalendarDateAttr * self, bool holiday) {
    return self->SetHoliday(holiday);
}
void wxCalendarDateAttr_SetTextColour(wxCalendarDateAttr * self, const wxColour * col_text) {
    return self->SetTextColour(*col_text);
}
wxCalendarDateAttr *wxCalendarDateAttr_GetMark() {
    return new wxCalendarDateAttr(wxCalendarDateAttr::GetMark());
}
void wxCalendarDateAttr_SetMark(const wxCalendarDateAttr * m) {
    return wxCalendarDateAttr::SetMark(*m);
}

// CLASS: wxCalendarEvent
wxClassInfo *wxCalendarEvent_CLASSINFO() {
    return wxCLASSINFO(wxCalendarEvent);
}
wxCalendarEvent *wxCalendarEvent_new() {
    return new wxCalendarEvent();
}

// CLASS: wxCaret
void wxCaret_delete(wxCaret *self) {
    delete self;
}
wxCaret *wxCaret_new() {
    return new wxCaret();
}
wxCaret *wxCaret_new1(wxWindow * window, int width, int height) {
    return new wxCaret(window, width, height);
}
wxCaret *wxCaret_new2(wxWindow * window, const wxSize * size) {
    return new wxCaret(window, *size);
}
bool wxCaret_Create(wxCaret * self, wxWindow * window, int width, int height) {
    return self->Create(window, width, height);
}
bool wxCaret_Create1(wxCaret * self, wxWindow * window, const wxSize * size) {
    return self->Create(window, *size);
}
void wxCaret_GetPosition(const wxCaret * self, int * x, int * y) {
    return self->GetPosition(x, y);
}
wxPoint *wxCaret_GetPosition1(const wxCaret * self) {
    return new wxPoint(self->GetPosition());
}
void wxCaret_GetSize(const wxCaret * self, int * width, int * height) {
    return self->GetSize(width, height);
}
wxSize *wxCaret_GetSize1(const wxCaret * self) {
    return new wxSize(self->GetSize());
}
wxWindow * wxCaret_GetWindow(const wxCaret * self) {
    return self->GetWindow();
}
void wxCaret_Hide(wxCaret * self) {
    return self->Hide();
}
bool wxCaret_IsOk(const wxCaret * self) {
    return self->IsOk();
}
bool wxCaret_IsVisible(const wxCaret * self) {
    return self->IsVisible();
}
void wxCaret_Move(wxCaret * self, int x, int y) {
    return self->Move(x, y);
}
void wxCaret_Move1(wxCaret * self, const wxPoint * pt) {
    return self->Move(*pt);
}
void wxCaret_SetSize(wxCaret * self, int width, int height) {
    return self->SetSize(width, height);
}
void wxCaret_SetSize1(wxCaret * self, const wxSize * size) {
    return self->SetSize(*size);
}
void wxCaret_Show(wxCaret * self, bool show) {
    return self->Show(show);
}
int wxCaret_GetBlinkTime() {
    return wxCaret::GetBlinkTime();
}
void wxCaret_SetBlinkTime(int milliseconds) {
    return wxCaret::SetBlinkTime(milliseconds);
}

// CLASS: wxCheckBox
wxClassInfo *wxCheckBox_CLASSINFO() {
    return wxCLASSINFO(wxCheckBox);
}
wxCheckBox *wxCheckBox_new() {
    return new wxCheckBox();
}
wxCheckBox *wxCheckBox_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxCheckBox(parent, id, *label, *pos, *size, style, *validator, *name);
}
bool wxCheckBox_Create(wxCheckBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *validator, *name);
}
bool wxCheckBox_GetValue(const wxCheckBox * self) {
    return self->GetValue();
}
wxCheckBoxState wxCheckBox_Get3StateValue(const wxCheckBox * self) {
    return self->Get3StateValue();
}
bool wxCheckBox_Is3State(const wxCheckBox * self) {
    return self->Is3State();
}
bool wxCheckBox_Is3rdStateAllowedForUser(const wxCheckBox * self) {
    return self->Is3rdStateAllowedForUser();
}
bool wxCheckBox_IsChecked(const wxCheckBox * self) {
    return self->IsChecked();
}
void wxCheckBox_SetValue(wxCheckBox * self, bool state) {
    return self->SetValue(state);
}
void wxCheckBox_Set3StateValue(wxCheckBox * self, wxCheckBoxState state) {
    return self->Set3StateValue(state);
}

// CLASS: wxCheckListBox
wxClassInfo *wxCheckListBox_CLASSINFO() {
    return wxCLASSINFO(wxCheckListBox);
}
wxCheckListBox *wxCheckListBox_new() {
    return new wxCheckListBox();
}
wxCheckListBox *wxCheckListBox_new2(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return new wxCheckListBox(parent, id, *pos, *size, *choices, style, *validator, *name);
}
bool wxCheckListBox_Create1(wxCheckListBox * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, *choices, style, *validator, *name);
}
void wxCheckListBox_Check(wxCheckListBox * self, unsigned int item, bool check) {
    return self->Check(item, check);
}
bool wxCheckListBox_IsChecked(const wxCheckListBox * self, unsigned int item) {
    return self->IsChecked(item);
}
unsigned int wxCheckListBox_GetCheckedItems(const wxCheckListBox * self, wxArrayInt * checked_items) {
    return self->GetCheckedItems(*checked_items);
}
// Mix-in(s) to wxCheckListBox
wxItemContainer *wxCheckListBox_AsItemContainer(wxCheckListBox* obj) {
    return static_cast<wxItemContainer*>(obj);
}

// CLASS: wxChildFocusEvent
wxClassInfo *wxChildFocusEvent_CLASSINFO() {
    return wxCLASSINFO(wxChildFocusEvent);
}
wxChildFocusEvent *wxChildFocusEvent_new(wxWindow * win) {
    return new wxChildFocusEvent(win);
}
wxWindow * wxChildFocusEvent_GetWindow(const wxChildFocusEvent * self) {
    return self->GetWindow();
}

// CLASS: wxChoice
wxClassInfo *wxChoice_CLASSINFO() {
    return wxCLASSINFO(wxChoice);
}
wxChoice *wxChoice_new() {
    return new wxChoice();
}
wxChoice *wxChoice_new2(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return new wxChoice(parent, id, *pos, *size, *choices, style, *validator, *name);
}
bool wxChoice_Create1(wxChoice * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, *choices, style, *validator, *name);
}
int wxChoice_GetColumns(const wxChoice * self) {
    return self->GetColumns();
}
int wxChoice_GetCurrentSelection(const wxChoice * self) {
    return self->GetCurrentSelection();
}
void wxChoice_SetColumns(wxChoice * self, int n) {
    return self->SetColumns(n);
}
bool wxChoice_IsSorted(const wxChoice * self) {
    return self->IsSorted();
}
// Mix-in(s) to wxChoice
wxItemContainer *wxChoice_AsItemContainer(wxChoice* obj) {
    return static_cast<wxItemContainer*>(obj);
}

// CLASS: wxChoicebook
wxClassInfo *wxChoicebook_CLASSINFO() {
    return wxCLASSINFO(wxChoicebook);
}
wxChoicebook *wxChoicebook_new() {
    return new wxChoicebook();
}
wxChoicebook *wxChoicebook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxChoicebook(parent, id, *pos, *size, style, *name);
}
bool wxChoicebook_Create(wxChoicebook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
wxChoice * wxChoicebook_GetChoiceCtrl(const wxChoicebook * self) {
    return self->GetChoiceCtrl();
}

// CLASS: wxClipboard
wxClassInfo *wxClipboard_CLASSINFO() {
    return wxCLASSINFO(wxClipboard);
}
wxClipboard *wxClipboard_new() {
    return new wxClipboard();
}
bool wxClipboard_AddData(wxClipboard * self, wxDataObject * data) {
    return self->AddData(data);
}
void wxClipboard_Clear(wxClipboard * self) {
    return self->Clear();
}
void wxClipboard_Close(wxClipboard * self) {
    return self->Close();
}
bool wxClipboard_Flush(wxClipboard * self) {
    return self->Flush();
}
bool wxClipboard_GetData(wxClipboard * self, wxDataObject * data) {
    return self->GetData(*data);
}
bool wxClipboard_IsOpened(const wxClipboard * self) {
    return self->IsOpened();
}
bool wxClipboard_IsSupported(wxClipboard * self, const wxDataFormat * format) {
    return self->IsSupported(*format);
}
bool wxClipboard_IsUsingPrimarySelection(const wxClipboard * self) {
    return self->IsUsingPrimarySelection();
}
bool wxClipboard_Open(wxClipboard * self) {
    return self->Open();
}
bool wxClipboard_SetData(wxClipboard * self, wxDataObject * data) {
    return self->SetData(data);
}
void wxClipboard_UsePrimarySelection(wxClipboard * self, bool primary) {
    return self->UsePrimarySelection(primary);
}
wxClipboard * wxClipboard_Get() {
    return wxClipboard::Get();
}

// CLASS: wxClipboardTextEvent
wxClassInfo *wxClipboardTextEvent_CLASSINFO() {
    return wxCLASSINFO(wxClipboardTextEvent);
}

// CLASS: wxCloseEvent
wxClassInfo *wxCloseEvent_CLASSINFO() {
    return wxCLASSINFO(wxCloseEvent);
}
bool wxCloseEvent_CanVeto(const wxCloseEvent * self) {
    return self->CanVeto();
}
bool wxCloseEvent_GetLoggingOff(const wxCloseEvent * self) {
    return self->GetLoggingOff();
}
void wxCloseEvent_SetCanVeto(wxCloseEvent * self, bool can_veto) {
    return self->SetCanVeto(can_veto);
}
void wxCloseEvent_SetLoggingOff(wxCloseEvent * self, bool logging_off) {
    return self->SetLoggingOff(logging_off);
}
void wxCloseEvent_Veto(wxCloseEvent * self, bool veto) {
    return self->Veto(veto);
}
bool wxCloseEvent_GetVeto(const wxCloseEvent * self) {
    return self->GetVeto();
}

// CLASS: wxCollapsibleHeaderCtrl
wxClassInfo *wxCollapsibleHeaderCtrl_CLASSINFO() {
    return wxCLASSINFO(wxCollapsibleHeaderCtrl);
}
wxCollapsibleHeaderCtrl *wxCollapsibleHeaderCtrl_new() {
    return new wxCollapsibleHeaderCtrl();
}
wxCollapsibleHeaderCtrl *wxCollapsibleHeaderCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxCollapsibleHeaderCtrl(parent, id, *label, *pos, *size, style, *validator, *name);
}
bool wxCollapsibleHeaderCtrl_Create(wxCollapsibleHeaderCtrl * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *validator, *name);
}
void wxCollapsibleHeaderCtrl_SetCollapsed(wxCollapsibleHeaderCtrl * self, bool collapsed) {
    return self->SetCollapsed(collapsed);
}
bool wxCollapsibleHeaderCtrl_IsCollapsed(const wxCollapsibleHeaderCtrl * self) {
    return self->IsCollapsed();
}

// CLASS: wxCollapsiblePane
wxClassInfo *wxCollapsiblePane_CLASSINFO() {
    return wxCLASSINFO(wxCollapsiblePane);
}
wxCollapsiblePane *wxCollapsiblePane_new() {
    return new wxCollapsiblePane();
}
wxCollapsiblePane *wxCollapsiblePane_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxCollapsiblePane(parent, id, *label, *pos, *size, style, *validator, *name);
}
bool wxCollapsiblePane_Create(wxCollapsiblePane * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *validator, *name);
}
void wxCollapsiblePane_Collapse(wxCollapsiblePane * self, bool collapse) {
    return self->Collapse(collapse);
}
void wxCollapsiblePane_Expand(wxCollapsiblePane * self) {
    return self->Expand();
}
wxWindow * wxCollapsiblePane_GetPane(const wxCollapsiblePane * self) {
    return self->GetPane();
}
bool wxCollapsiblePane_IsCollapsed(const wxCollapsiblePane * self) {
    return self->IsCollapsed();
}
bool wxCollapsiblePane_IsExpanded(const wxCollapsiblePane * self) {
    return self->IsExpanded();
}

// CLASS: wxCollapsiblePaneEvent
wxClassInfo *wxCollapsiblePaneEvent_CLASSINFO() {
    return wxCLASSINFO(wxCollapsiblePaneEvent);
}
wxCollapsiblePaneEvent *wxCollapsiblePaneEvent_new(wxObject * generator, int id, bool collapsed) {
    return new wxCollapsiblePaneEvent(generator, id, collapsed);
}
bool wxCollapsiblePaneEvent_GetCollapsed(const wxCollapsiblePaneEvent * self) {
    return self->GetCollapsed();
}
void wxCollapsiblePaneEvent_SetCollapsed(wxCollapsiblePaneEvent * self, bool collapsed) {
    return self->SetCollapsed(collapsed);
}

// CLASS: wxColour
wxClassInfo *wxColour_CLASSINFO() {
    return wxCLASSINFO(wxColour);
}
wxColour *wxColour_new() {
    return new wxColour();
}
wxColour *wxColour_new2(const wxString * colour_name) {
    return new wxColour(*colour_name);
}
wxColour *wxColour_new4(const wxColour * colour) {
    return new wxColour(*colour);
}
#if wxCHECK_VERSION(3, 1, 0)
unsigned int wxColour_GetAlpha(const wxColour * self) {
    return self->GetAlpha();
}
unsigned int wxColour_GetBlue(const wxColour * self) {
    return self->GetBlue();
}
unsigned int wxColour_GetGreen(const wxColour * self) {
    return self->GetGreen();
}
unsigned int wxColour_GetRed(const wxColour * self) {
    return self->GetRed();
}
#endif
wxString *wxColour_GetAsString(const wxColour * self, long flags) {
    return new wxString(self->GetAsString(flags));
}
#if wxCHECK_VERSION(3, 1, 0)
double wxColour_GetLuminance(const wxColour * self) {
    return self->GetLuminance();
}
#endif
bool wxColour_IsOk(const wxColour * self) {
    return self->IsOk();
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxColour_IsSolid(const wxColour * self) {
    return self->IsSolid();
}
#endif
bool wxColour_Set2(wxColour * self, const wxString * str) {
    return self->Set(*str);
}
void wxColour_MakeMono(unsigned char * r, unsigned char * g, unsigned char * b, bool on) {
    return wxColour::MakeMono(r, g, b, on);
}
void wxColour_MakeGrey(unsigned char * r, unsigned char * g, unsigned char * b) {
    return wxColour::MakeGrey(r, g, b);
}
void wxColour_MakeGrey1(unsigned char * r, unsigned char * g, unsigned char * b, double weight_r, double weight_g, double weight_b) {
    return wxColour::MakeGrey(r, g, b, weight_r, weight_g, weight_b);
}
void wxColour_ChangeLightness1(unsigned char * r, unsigned char * g, unsigned char * b, int ialpha) {
    return wxColour::ChangeLightness(r, g, b, ialpha);
}

// CLASS: wxColourData
wxClassInfo *wxColourData_CLASSINFO() {
    return wxCLASSINFO(wxColourData);
}
wxColourData *wxColourData_new() {
    return new wxColourData();
}
bool wxColourData_GetChooseFull(const wxColourData * self) {
    return self->GetChooseFull();
}
bool wxColourData_GetChooseAlpha(const wxColourData * self) {
    return self->GetChooseAlpha();
}
wxColour * wxColourData_GetColour(wxColourData * self) {
    return &(self->GetColour());
}
wxColour *wxColourData_GetCustomColour(const wxColourData * self, int i) {
    return new wxColour(self->GetCustomColour(i));
}
void wxColourData_SetChooseFull(wxColourData * self, bool flag) {
    return self->SetChooseFull(flag);
}
void wxColourData_SetChooseAlpha(wxColourData * self, bool flag) {
    return self->SetChooseAlpha(flag);
}
void wxColourData_SetColour(wxColourData * self, const wxColour * colour) {
    return self->SetColour(*colour);
}
void wxColourData_SetCustomColour(wxColourData * self, int i, const wxColour * colour) {
    return self->SetCustomColour(i, *colour);
}
wxString *wxColourData_ToString(const wxColourData * self) {
    return new wxString(self->ToString());
}
bool wxColourData_FromString(wxColourData * self, const wxString * str) {
    return self->FromString(*str);
}

// CLASS: wxColourDatabase
void wxColourDatabase_delete(wxColourDatabase *self) {
    delete self;
}
wxColourDatabase *wxColourDatabase_new() {
    return new wxColourDatabase();
}
void wxColourDatabase_AddColour(wxColourDatabase * self, const wxString * colour_name, const wxColour * colour) {
    return self->AddColour(*colour_name, *colour);
}
wxColour *wxColourDatabase_Find(const wxColourDatabase * self, const wxString * colour_name) {
    return new wxColour(self->Find(*colour_name));
}
wxString *wxColourDatabase_FindName(const wxColourDatabase * self, const wxColour * colour) {
    return new wxString(self->FindName(*colour));
}

// CLASS: wxColourDialog
wxClassInfo *wxColourDialog_CLASSINFO() {
    return wxCLASSINFO(wxColourDialog);
}
wxColourDialog *wxColourDialog_new(wxWindow * parent, const wxColourData * data) {
    return new wxColourDialog(parent, data);
}
bool wxColourDialog_Create(wxColourDialog * self, wxWindow * parent, const wxColourData * data) {
    return self->Create(parent, data);
}
wxColourData * wxColourDialog_GetColourData(wxColourDialog * self) {
    return &(self->GetColourData());
}

// CLASS: wxColourDialogEvent
wxClassInfo *wxColourDialogEvent_CLASSINFO() {
    return wxCLASSINFO(wxColourDialogEvent);
}
wxColourDialogEvent *wxColourDialogEvent_new() {
    return new wxColourDialogEvent();
}
wxColour *wxColourDialogEvent_GetColour(const wxColourDialogEvent * self) {
    return new wxColour(self->GetColour());
}
void wxColourDialogEvent_SetColour(wxColourDialogEvent * self, const wxColour * colour) {
    return self->SetColour(*colour);
}

// CLASS: wxColourPickerCtrl
wxClassInfo *wxColourPickerCtrl_CLASSINFO() {
    return wxCLASSINFO(wxColourPickerCtrl);
}
wxColourPickerCtrl *wxColourPickerCtrl_new() {
    return new wxColourPickerCtrl();
}
wxColourPickerCtrl *wxColourPickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxColour * colour, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxColourPickerCtrl(parent, id, *colour, *pos, *size, style, *validator, *name);
}
bool wxColourPickerCtrl_Create(wxColourPickerCtrl * self, wxWindow * parent, wxWindowID id, const wxColour * colour, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *colour, *pos, *size, style, *validator, *name);
}
wxColour *wxColourPickerCtrl_GetColour(const wxColourPickerCtrl * self) {
    return new wxColour(self->GetColour());
}
void wxColourPickerCtrl_SetColour(wxColourPickerCtrl * self, const wxColour * col) {
    return self->SetColour(*col);
}

// CLASS: wxColourPickerEvent
wxClassInfo *wxColourPickerEvent_CLASSINFO() {
    return wxCLASSINFO(wxColourPickerEvent);
}
wxColourPickerEvent *wxColourPickerEvent_new() {
    return new wxColourPickerEvent();
}
wxColourPickerEvent *wxColourPickerEvent_new1(wxObject * generator, int id, const wxColour * colour) {
    return new wxColourPickerEvent(generator, id, *colour);
}
wxColour *wxColourPickerEvent_GetColour(const wxColourPickerEvent * self) {
    return new wxColour(self->GetColour());
}
void wxColourPickerEvent_SetColour(wxColourPickerEvent * self, const wxColour * pos) {
    return self->SetColour(*pos);
}

// CLASS: wxComboBox
wxClassInfo *wxComboBox_CLASSINFO() {
    return wxCLASSINFO(wxComboBox);
}
wxComboBox *wxComboBox_new() {
    return new wxComboBox();
}
wxComboBox *wxComboBox_new2(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return new wxComboBox(parent, id, *value, *pos, *size, *choices, style, *validator, *name);
}
bool wxComboBox_Create1(wxComboBox * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *value, *pos, *size, *choices, style, *validator, *name);
}
int wxComboBox_GetCurrentSelection(const wxComboBox * self) {
    return self->GetCurrentSelection();
}
bool wxComboBox_IsListEmpty(const wxComboBox * self) {
    return self->IsListEmpty();
}
bool wxComboBox_IsTextEmpty(const wxComboBox * self) {
    return self->IsTextEmpty();
}
void wxComboBox_Popup(wxComboBox * self) {
    return self->Popup();
}
void wxComboBox_Dismiss(wxComboBox * self) {
    return self->Dismiss();
}
// Mix-in(s) to wxComboBox
wxItemContainer *wxComboBox_AsItemContainer(wxComboBox* obj) {
    return static_cast<wxItemContainer*>(obj);
}
wxTextEntryBase *wxComboBox_AsTextEntry(wxComboBox* obj) {
    return static_cast<wxTextEntryBase*>(obj);
}

// CLASS: wxComboCtrl
wxClassInfo *wxComboCtrl_CLASSINFO() {
    return wxCLASSINFO(wxComboCtrl);
}
wxComboCtrl *wxComboCtrl_new() {
    return new wxComboCtrl();
}
wxComboCtrl *wxComboCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxComboCtrl(parent, id, *value, *pos, *size, style, *validator, *name);
}
bool wxComboCtrl_Create(wxComboCtrl * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *value, *pos, *size, style, *validator, *name);
}
void wxComboCtrl_Dismiss(wxComboCtrl * self) {
    return self->Dismiss();
}
void wxComboCtrl_EnablePopupAnimation(wxComboCtrl * self, bool enable) {
    return self->EnablePopupAnimation(enable);
}
bool wxComboCtrl_IsKeyPopupToggle(const wxComboCtrl * self, const wxKeyEvent * event) {
    return self->IsKeyPopupToggle(*event);
}
void wxComboCtrl_PrepareBackground(const wxComboCtrl * self, wxDC * dc, const wxRect * rect, int flags) {
    return self->PrepareBackground(*dc, *rect, flags);
}
bool wxComboCtrl_ShouldDrawFocus(const wxComboCtrl * self) {
    return self->ShouldDrawFocus();
}
wxBitmap *wxComboCtrl_GetBitmapDisabled(const wxComboCtrl * self) {
    return new wxBitmap(self->GetBitmapDisabled());
}
wxBitmap *wxComboCtrl_GetBitmapHover(const wxComboCtrl * self) {
    return new wxBitmap(self->GetBitmapHover());
}
wxBitmap *wxComboCtrl_GetBitmapNormal(const wxComboCtrl * self) {
    return new wxBitmap(self->GetBitmapNormal());
}
wxBitmap *wxComboCtrl_GetBitmapPressed(const wxComboCtrl * self) {
    return new wxBitmap(self->GetBitmapPressed());
}
wxSize *wxComboCtrl_GetButtonSize(wxComboCtrl * self) {
    return new wxSize(self->GetButtonSize());
}
int wxComboCtrl_GetCustomPaintWidth(const wxComboCtrl * self) {
    return self->GetCustomPaintWidth();
}
wxPoint *wxComboCtrl_GetMargins(const wxComboCtrl * self) {
    return new wxPoint(self->GetMargins());
}
wxComboPopup * wxComboCtrl_GetPopupControl(wxComboCtrl * self) {
    return self->GetPopupControl();
}
wxWindow * wxComboCtrl_GetPopupWindow(const wxComboCtrl * self) {
    return self->GetPopupWindow();
}
wxTextCtrl * wxComboCtrl_GetTextCtrl(const wxComboCtrl * self) {
    return self->GetTextCtrl();
}
wxRect *wxComboCtrl_GetTextRect(const wxComboCtrl * self) {
    return new wxRect(self->GetTextRect());
}
void wxComboCtrl_HidePopup(wxComboCtrl * self, bool generate_event) {
    return self->HidePopup(generate_event);
}
bool wxComboCtrl_IsPopupShown(const wxComboCtrl * self) {
    return self->IsPopupShown();
}
bool wxComboCtrl_IsPopupWindowState(const wxComboCtrl * self, int state) {
    return self->IsPopupWindowState(state);
}
void wxComboCtrl_OnButtonClick(wxComboCtrl * self) {
    return self->OnButtonClick();
}
void wxComboCtrl_Popup(wxComboCtrl * self) {
    return self->Popup();
}
void wxComboCtrl_SetButtonBitmaps(wxComboCtrl * self, const wxBitmapBundle * bmp_normal, bool push_button_bg, const wxBitmapBundle * bmp_pressed, const wxBitmapBundle * bmp_hover, const wxBitmapBundle * bmp_disabled) {
    return self->SetButtonBitmaps(*bmp_normal, push_button_bg, *bmp_pressed, *bmp_hover, *bmp_disabled);
}
void wxComboCtrl_SetButtonPosition(wxComboCtrl * self, int width, int height, int side, int spacing_x) {
    return self->SetButtonPosition(width, height, side, spacing_x);
}
void wxComboCtrl_SetCustomPaintWidth(wxComboCtrl * self, int width) {
    return self->SetCustomPaintWidth(width);
}
void wxComboCtrl_SetMainControl(wxComboCtrl * self, wxWindow * win) {
    return self->SetMainControl(win);
}
bool wxComboCtrl_SetMargins(wxComboCtrl * self, const wxPoint * pt) {
    return self->SetMargins(*pt);
}
bool wxComboCtrl_SetMargins1(wxComboCtrl * self, wxCoord left, wxCoord top) {
    return self->SetMargins(left, top);
}
void wxComboCtrl_SetPopupAnchor(wxComboCtrl * self, int anchor_side) {
    return self->SetPopupAnchor(anchor_side);
}
void wxComboCtrl_SetPopupControl(wxComboCtrl * self, wxComboPopup * popup) {
    return self->SetPopupControl(popup);
}
void wxComboCtrl_SetPopupExtents(wxComboCtrl * self, int ext_left, int ext_right) {
    return self->SetPopupExtents(ext_left, ext_right);
}
void wxComboCtrl_SetPopupMaxHeight(wxComboCtrl * self, int height) {
    return self->SetPopupMaxHeight(height);
}
void wxComboCtrl_SetPopupMinWidth(wxComboCtrl * self, int width) {
    return self->SetPopupMinWidth(width);
}
void wxComboCtrl_SetText(wxComboCtrl * self, const wxString * value) {
    return self->SetText(*value);
}
void wxComboCtrl_SetTextCtrlStyle(wxComboCtrl * self, int style) {
    return self->SetTextCtrlStyle(style);
}
void wxComboCtrl_SetValueByUser(wxComboCtrl * self, const wxString * value) {
    return self->SetValueByUser(*value);
}
void wxComboCtrl_ShowPopup(wxComboCtrl * self) {
    return self->ShowPopup();
}
void wxComboCtrl_UseAltPopupWindow(wxComboCtrl * self, bool enable) {
    return self->UseAltPopupWindow(enable);
}
int wxComboCtrl_GetFeatures() {
    return wxComboCtrl::GetFeatures();
}
// Mix-in(s) to wxComboCtrl
wxTextEntryBase *wxComboCtrl_AsTextEntry(wxComboCtrl* obj) {
    return static_cast<wxTextEntryBase*>(obj);
}

// CLASS: wxComboPopup
void wxComboPopup_delete(wxComboPopup *self) {
    delete self;
}
bool wxComboPopup_Create(wxComboPopup * self, wxWindow * parent) {
    return self->Create(parent);
}
void wxComboPopup_DestroyPopup(wxComboPopup * self) {
    return self->DestroyPopup();
}
void wxComboPopup_Dismiss(wxComboPopup * self) {
    return self->Dismiss();
}
bool wxComboPopup_FindItem(wxComboPopup * self, const wxString * item, wxString * true_item) {
    return self->FindItem(*item, true_item);
}
wxSize *wxComboPopup_GetAdjustedSize(wxComboPopup * self, int min_width, int pref_height, int max_height) {
    return new wxSize(self->GetAdjustedSize(min_width, pref_height, max_height));
}
wxComboCtrl * wxComboPopup_GetComboCtrl(const wxComboPopup * self) {
    return self->GetComboCtrl();
}
wxWindow * wxComboPopup_GetControl(wxComboPopup * self) {
    return self->GetControl();
}
wxString *wxComboPopup_GetStringValue(const wxComboPopup * self) {
    return new wxString(self->GetStringValue());
}
void wxComboPopup_Init(wxComboPopup * self) {
    return self->Init();
}
bool wxComboPopup_IsCreated(const wxComboPopup * self) {
    return self->IsCreated();
}
bool wxComboPopup_LazyCreate(wxComboPopup * self) {
    return self->LazyCreate();
}
void wxComboPopup_OnComboDoubleClick(wxComboPopup * self) {
    return self->OnComboDoubleClick();
}
void wxComboPopup_OnComboKeyEvent(wxComboPopup * self, wxKeyEvent * event) {
    return self->OnComboKeyEvent(*event);
}
void wxComboPopup_OnDismiss(wxComboPopup * self) {
    return self->OnDismiss();
}
void wxComboPopup_OnPopup(wxComboPopup * self) {
    return self->OnPopup();
}
void wxComboPopup_PaintComboControl(wxComboPopup * self, wxDC * dc, const wxRect * rect) {
    return self->PaintComboControl(*dc, *rect);
}
void wxComboPopup_SetStringValue(wxComboPopup * self, const wxString * value) {
    return self->SetStringValue(*value);
}

// CLASS: wxCommand
wxClassInfo *wxCommand_CLASSINFO() {
    return wxCLASSINFO(wxCommand);
}
bool wxCommand_CanUndo(const wxCommand * self) {
    return self->CanUndo();
}
bool wxCommand_Do(wxCommand * self) {
    return self->Do();
}
wxString *wxCommand_GetName(const wxCommand * self) {
    return new wxString(self->GetName());
}
bool wxCommand_Undo(wxCommand * self) {
    return self->Undo();
}

// CLASS: wxCommandEvent
wxClassInfo *wxCommandEvent_CLASSINFO() {
    return wxCLASSINFO(wxCommandEvent);
}
void * wxCommandEvent_GetClientData(const wxCommandEvent * self) {
    return self->GetClientData();
}
wxClientData * wxCommandEvent_GetClientObject(const wxCommandEvent * self) {
    return self->GetClientObject();
}
long wxCommandEvent_GetExtraLong(const wxCommandEvent * self) {
    return self->GetExtraLong();
}
int wxCommandEvent_GetInt(const wxCommandEvent * self) {
    return self->GetInt();
}
int wxCommandEvent_GetSelection(const wxCommandEvent * self) {
    return self->GetSelection();
}
wxString *wxCommandEvent_GetString(const wxCommandEvent * self) {
    return new wxString(self->GetString());
}
bool wxCommandEvent_IsChecked(const wxCommandEvent * self) {
    return self->IsChecked();
}
bool wxCommandEvent_IsSelection(const wxCommandEvent * self) {
    return self->IsSelection();
}
void wxCommandEvent_SetClientData(wxCommandEvent * self, void * client_data) {
    return self->SetClientData(client_data);
}
void wxCommandEvent_SetClientObject(wxCommandEvent * self, wxClientData * client_object) {
    return self->SetClientObject(client_object);
}
void wxCommandEvent_SetExtraLong(wxCommandEvent * self, long extra_long) {
    return self->SetExtraLong(extra_long);
}
void wxCommandEvent_SetInt(wxCommandEvent * self, int int_command) {
    return self->SetInt(int_command);
}
void wxCommandEvent_SetString(wxCommandEvent * self, const wxString * string) {
    return self->SetString(*string);
}

// CLASS: wxCommandLinkButton
wxClassInfo *wxCommandLinkButton_CLASSINFO() {
    return wxCLASSINFO(wxCommandLinkButton);
}
wxCommandLinkButton *wxCommandLinkButton_new() {
    return new wxCommandLinkButton();
}
wxCommandLinkButton *wxCommandLinkButton_new1(wxWindow * parent, wxWindowID id, const wxString * main_label, const wxString * note, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxCommandLinkButton(parent, id, *main_label, *note, *pos, *size, style, *validator, *name);
}
bool wxCommandLinkButton_Create(wxCommandLinkButton * self, wxWindow * parent, wxWindowID id, const wxString * main_label, const wxString * note, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *main_label, *note, *pos, *size, style, *validator, *name);
}
void wxCommandLinkButton_SetMainLabelAndNote(wxCommandLinkButton * self, const wxString * main_label, const wxString * note) {
    return self->SetMainLabelAndNote(*main_label, *note);
}
void wxCommandLinkButton_SetMainLabel(wxCommandLinkButton * self, const wxString * main_label) {
    return self->SetMainLabel(*main_label);
}
void wxCommandLinkButton_SetNote(wxCommandLinkButton * self, const wxString * note) {
    return self->SetNote(*note);
}
wxString *wxCommandLinkButton_GetMainLabel(const wxCommandLinkButton * self) {
    return new wxString(self->GetMainLabel());
}
wxString *wxCommandLinkButton_GetNote(const wxCommandLinkButton * self) {
    return new wxString(self->GetNote());
}

// CLASS: wxCommandProcessor
wxClassInfo *wxCommandProcessor_CLASSINFO() {
    return wxCLASSINFO(wxCommandProcessor);
}
wxCommandProcessor *wxCommandProcessor_new(int max_commands) {
    return new wxCommandProcessor(max_commands);
}
bool wxCommandProcessor_CanUndo(const wxCommandProcessor * self) {
    return self->CanUndo();
}
bool wxCommandProcessor_CanRedo(const wxCommandProcessor * self) {
    return self->CanRedo();
}
void wxCommandProcessor_ClearCommands(wxCommandProcessor * self) {
    return self->ClearCommands();
}
wxCommand * wxCommandProcessor_GetCurrentCommand(const wxCommandProcessor * self) {
    return self->GetCurrentCommand();
}
wxMenu * wxCommandProcessor_GetEditMenu(const wxCommandProcessor * self) {
    return self->GetEditMenu();
}
int wxCommandProcessor_GetMaxCommands(const wxCommandProcessor * self) {
    return self->GetMaxCommands();
}
wxString *wxCommandProcessor_GetRedoAccelerator(const wxCommandProcessor * self) {
    return new wxString(self->GetRedoAccelerator());
}
wxString *wxCommandProcessor_GetRedoMenuLabel(const wxCommandProcessor * self) {
    return new wxString(self->GetRedoMenuLabel());
}
wxString *wxCommandProcessor_GetUndoAccelerator(const wxCommandProcessor * self) {
    return new wxString(self->GetUndoAccelerator());
}
wxString *wxCommandProcessor_GetUndoMenuLabel(const wxCommandProcessor * self) {
    return new wxString(self->GetUndoMenuLabel());
}
void wxCommandProcessor_Initialize(wxCommandProcessor * self) {
    return self->Initialize();
}
bool wxCommandProcessor_IsDirty(const wxCommandProcessor * self) {
    return self->IsDirty();
}
void wxCommandProcessor_MarkAsSaved(wxCommandProcessor * self) {
    return self->MarkAsSaved();
}
bool wxCommandProcessor_Redo(wxCommandProcessor * self) {
    return self->Redo();
}
void wxCommandProcessor_SetEditMenu(wxCommandProcessor * self, wxMenu * menu) {
    return self->SetEditMenu(menu);
}
void wxCommandProcessor_SetMenuStrings(wxCommandProcessor * self) {
    return self->SetMenuStrings();
}
void wxCommandProcessor_SetRedoAccelerator(wxCommandProcessor * self, const wxString * accel) {
    return self->SetRedoAccelerator(*accel);
}
void wxCommandProcessor_SetUndoAccelerator(wxCommandProcessor * self, const wxString * accel) {
    return self->SetUndoAccelerator(*accel);
}
bool wxCommandProcessor_Submit(wxCommandProcessor * self, wxCommand * command, bool store_it) {
    return self->Submit(command, store_it);
}
void wxCommandProcessor_Store(wxCommandProcessor * self, wxCommand * command) {
    return self->Store(command);
}
bool wxCommandProcessor_Undo(wxCommandProcessor * self) {
    return self->Undo();
}

// CLASS: wxContextHelpButton
wxClassInfo *wxContextHelpButton_CLASSINFO() {
    return wxCLASSINFO(wxContextHelpButton);
}
wxContextHelpButton *wxContextHelpButton_new(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style) {
    return new wxContextHelpButton(parent, id, *pos, *size, style);
}

// CLASS: wxContextMenuEvent
wxClassInfo *wxContextMenuEvent_CLASSINFO() {
    return wxCLASSINFO(wxContextMenuEvent);
}
wxPoint *wxContextMenuEvent_GetPosition(const wxContextMenuEvent * self) {
    return new wxPoint(self->GetPosition());
}
void wxContextMenuEvent_SetPosition(wxContextMenuEvent * self, const wxPoint * point) {
    return self->SetPosition(*point);
}

// CLASS: wxControl
wxClassInfo *wxControl_CLASSINFO() {
    return wxCLASSINFO(wxControl);
}
wxControl *wxControl_new(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxControl(parent, id, *pos, *size, style, *validator, *name);
}
wxControl *wxControl_new1() {
    return new wxControl();
}
bool wxControl_Create(wxControl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *validator, *name);
}
void wxControl_Command(wxControl * self, wxCommandEvent * event) {
    return self->Command(*event);
}
wxString *wxControl_GetLabelText(const wxControl * self) {
    return new wxString(self->GetLabelText());
}
wxSize *wxControl_GetSizeFromTextSize(const wxControl * self, int xlen, int ylen) {
    return new wxSize(self->GetSizeFromTextSize(xlen, ylen));
}
wxSize *wxControl_GetSizeFromTextSize1(const wxControl * self, const wxSize * tsize) {
    return new wxSize(self->GetSizeFromTextSize(*tsize));
}
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxControl_GetSizeFromText(const wxControl * self, const wxString * text) {
    return new wxSize(self->GetSizeFromText(*text));
}
#endif
void wxControl_SetLabelText(wxControl * self, const wxString * text) {
    return self->SetLabelText(*text);
}
bool wxControl_SetLabelMarkup(wxControl * self, const wxString * markup) {
    return self->SetLabelMarkup(*markup);
}
wxString *wxControl_GetLabelText1(const wxString * label) {
    return new wxString(wxControl::GetLabelText(*label));
}
wxString *wxControl_RemoveMnemonics(const wxString * str) {
    return new wxString(wxControl::RemoveMnemonics(*str));
}
wxString *wxControl_EscapeMnemonics(const wxString * text) {
    return new wxString(wxControl::EscapeMnemonics(*text));
}
wxString *wxControl_Ellipsize(const wxString * label, const wxDC * dc, wxEllipsizeMode mode, int max_width, int flags) {
    return new wxString(wxControl::Ellipsize(*label, *dc, mode, max_width, flags));
}

// CLASS: wxControlWithItems
wxClassInfo *wxControlWithItems_CLASSINFO() {
    return wxCLASSINFO(wxControlWithItems);
}
// Mix-in(s) to wxControlWithItems
wxItemContainer *wxControlWithItems_AsItemContainer(wxControlWithItems* obj) {
    return static_cast<wxItemContainer*>(obj);
}

// CLASS: wxCredentialEntryDialog
wxClassInfo *wxCredentialEntryDialog_CLASSINFO() {
    return wxCLASSINFO(wxCredentialEntryDialog);
}
wxCredentialEntryDialog *wxCredentialEntryDialog_new() {
    return new wxCredentialEntryDialog();
}
wxCredentialEntryDialog *wxCredentialEntryDialog_new1(wxWindow * parent, const wxString * message, const wxString * title, const wxWebCredentials * cred) {
    return new wxCredentialEntryDialog(parent, *message, *title, *cred);
}
bool wxCredentialEntryDialog_Create(wxCredentialEntryDialog * self, wxWindow * parent, const wxString * message, const wxString * title, const wxWebCredentials * cred) {
    return self->Create(parent, *message, *title, *cred);
}
void wxCredentialEntryDialog_SetUser(wxCredentialEntryDialog * self, const wxString * user) {
    return self->SetUser(*user);
}
void wxCredentialEntryDialog_SetPassword(wxCredentialEntryDialog * self, const wxString * password) {
    return self->SetPassword(*password);
}

// CLASS: wxCursor
wxClassInfo *wxCursor_CLASSINFO() {
    return wxCLASSINFO(wxCursor);
}
wxCursor *wxCursor_new() {
    return new wxCursor();
}
wxCursor *wxCursor_new4(const wxImage * image) {
    return new wxCursor(*image);
}
wxCursor *wxCursor_new5(const char *const * xpm_data) {
    return new wxCursor(xpm_data);
}
wxCursor *wxCursor_new6(const wxCursor * cursor) {
    return new wxCursor(*cursor);
}
bool wxCursor_IsOk(const wxCursor * self) {
    return self->IsOk();
}
wxPoint *wxCursor_GetHotSpot(const wxCursor * self) {
    return new wxPoint(self->GetHotSpot());
}

// CLASS: wxCustomDataObject
void wxCustomDataObject_delete(wxCustomDataObject *self) {
    delete self;
}
wxCustomDataObject *wxCustomDataObject_new(const wxDataFormat * format) {
    return new wxCustomDataObject(*format);
}
void * wxCustomDataObject_Alloc(wxCustomDataObject * self, size_t size) {
    return self->Alloc(size);
}
void wxCustomDataObject_Free(wxCustomDataObject * self) {
    return self->Free();
}
void * wxCustomDataObject_GetData(const wxCustomDataObject * self) {
    return self->GetData();
}
size_t wxCustomDataObject_GetSize(const wxCustomDataObject * self) {
    return self->GetSize();
}
void wxCustomDataObject_TakeData(wxCustomDataObject * self, size_t size, void * data) {
    return self->TakeData(size, data);
}

} // extern "C"

