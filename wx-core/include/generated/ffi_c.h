#pragma once
#include <wx/wx.h>
#include <wx/checkbox.h>
#include <wx/checklst.h>
#include <wx/choice.h>
#include <wx/clrpicker.h>
#include <wx/colour.h>
#include <wx/combobox.h>
#include <wx/control.h>
#include <wx/event.h>

// wxBitmapBundle compatibility hack(for a while)
#if !wxCHECK_VERSION(3, 1, 6)
typedef wxBitmap wxBitmapBundle;
#endif

extern "C" {

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

// CLASS: wxColourPickerCtrl
wxClassInfo *wxColourPickerCtrl_CLASSINFO();
wxColourPickerCtrl *wxColourPickerCtrl_new();
wxColourPickerCtrl *wxColourPickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxColour * colour, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxColourPickerCtrl_Create(wxColourPickerCtrl * self, wxWindow * parent, wxWindowID id, const wxColour * colour, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
wxColour *wxColourPickerCtrl_GetColour(const wxColourPickerCtrl * self);
void wxColourPickerCtrl_SetColour(wxColourPickerCtrl * self, const wxColour * col);

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

} // extern "C"

