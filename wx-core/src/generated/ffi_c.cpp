#include "generated.h"

extern "C" {

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

} // extern "C"

