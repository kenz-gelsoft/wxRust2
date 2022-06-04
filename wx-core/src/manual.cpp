#include <wx/ctrlsub.h>
#include "manual.h"

// wxList<T>
#define IMPL_WX_LIST_BINDING(wxclass)                       \
    wxclass##List *wxclass##List_new() {                    \
        return new wxclass##List();                         \
    }                                                       \
    void wxclass##List_delete(wxclass##List *self) {        \
        delete self;                                        \
    }                                                       \
    size_t wxclass##List_GetCount(wxclass##List *self) {    \
        return self->GetCount();                            \
    }                                                       \
    bool wxclass##List_IsEmpty(wxclass##List *self) {       \
        return self->IsEmpty();                             \
    }

IMPL_WX_LIST_BINDING(wxSizerItem);
IMPL_WX_LIST_BINDING(wxWindow);

int wxRustMessageBox(const wxString *message, const wxString *caption, int style, wxWindow *parent, int x, int y) {
    return wxMessageBox(*message, *caption, style, parent, x, y);
}

// CLASS: wxItemContainer
void wxItemContainer_delete(wxItemContainer *self) {
    delete self;
}
int wxItemContainer_Append(wxItemContainer * self, const wxString * item) {
    return self->Append(*item);
}
int wxItemContainer_Append1(wxItemContainer * self, const wxString * item, void * client_data) {
    return self->Append(*item, client_data);
}
int wxItemContainer_Append2(wxItemContainer * self, const wxString * item, wxClientData * client_data) {
    return self->Append(*item, client_data);
}
int wxItemContainer_Append3(wxItemContainer * self, const wxArrayString * items) {
    return self->Append(*items);
}
int wxItemContainer_Append5(wxItemContainer * self, const wxArrayString * items, void ** client_data) {
    return self->Append(*items, client_data);
}
int wxItemContainer_Append6(wxItemContainer * self, const wxArrayString * items, wxClientData ** client_data) {
    return self->Append(*items, client_data);
}
void wxItemContainer_Clear(wxItemContainer * self) {
    return self->Clear();
}
void wxItemContainer_Delete(wxItemContainer * self, unsigned int n) {
    return self->Delete(n);
}
wxClientData * wxItemContainer_DetachClientObject(wxItemContainer * self, unsigned int n) {
    return self->DetachClientObject(n);
}
bool wxItemContainer_HasClientData(const wxItemContainer * self) {
    return self->HasClientData();
}
bool wxItemContainer_HasClientObjectData(const wxItemContainer * self) {
    return self->HasClientObjectData();
}
bool wxItemContainer_HasClientUntypedData(const wxItemContainer * self) {
    return self->HasClientUntypedData();
}
void * wxItemContainer_GetClientData(const wxItemContainer * self, unsigned int n) {
    return self->GetClientData(n);
}
wxClientData * wxItemContainer_GetClientObject(const wxItemContainer * self, unsigned int n) {
    return self->GetClientObject(n);
}
void wxItemContainer_SetClientData(wxItemContainer * self, unsigned int n, void * data) {
    return self->SetClientData(n, data);
}
void wxItemContainer_SetClientObject(wxItemContainer * self, unsigned int n, wxClientData * data) {
    return self->SetClientObject(n, data);
}
int wxItemContainer_Insert(wxItemContainer * self, const wxString * item, unsigned int pos) {
    return self->Insert(*item, pos);
}
int wxItemContainer_Insert1(wxItemContainer * self, const wxString * item, unsigned int pos, void * client_data) {
    return self->Insert(*item, pos, client_data);
}
int wxItemContainer_Insert2(wxItemContainer * self, const wxString * item, unsigned int pos, wxClientData * client_data) {
    return self->Insert(*item, pos, client_data);
}
int wxItemContainer_Insert3(wxItemContainer * self, const wxArrayString * items, unsigned int pos) {
    return self->Insert(*items, pos);
}
int wxItemContainer_Insert5(wxItemContainer * self, const wxArrayString * items, unsigned int pos, void ** client_data) {
    return self->Insert(*items, pos, client_data);
}
int wxItemContainer_Insert6(wxItemContainer * self, const wxArrayString * items, unsigned int pos, wxClientData ** client_data) {
    return self->Insert(*items, pos, client_data);
}
void wxItemContainer_Set(wxItemContainer * self, const wxArrayString * items) {
    return self->Set(*items);
}
void wxItemContainer_Set2(wxItemContainer * self, const wxArrayString * items, void ** client_data) {
    return self->Set(*items, client_data);
}
void wxItemContainer_Set3(wxItemContainer * self, const wxArrayString * items, wxClientData ** client_data) {
    return self->Set(*items, client_data);
}

wxItemContainer *wxChoice_AsItemContainer(wxChoice * obj) {
    return static_cast<wxItemContainer *>(obj);
}

// CLASS: wxItemContainerImmutable
void wxItemContainerImmutable_delete(wxItemContainerImmutable *self) {
    delete self;
}
void wxItemContainerImmutable_SetSelection(wxItemContainerImmutable * self, int n) {
    return self->SetSelection(n);
}
int wxItemContainerImmutable_GetSelection(const wxItemContainerImmutable * self) {
    return self->GetSelection();
}
bool wxItemContainerImmutable_SetStringSelection(wxItemContainerImmutable * self, const wxString * string) {
    return self->SetStringSelection(*string);
}
wxString *wxItemContainerImmutable_GetStringSelection(const wxItemContainerImmutable * self) {
    return new wxString(self->GetStringSelection());
}
void wxItemContainerImmutable_Select(wxItemContainerImmutable * self, int n) {
    return self->Select(n);
}
// wxItemContainerImmutable *wxItemContainerImmutable_new() {
//     return new wxItemContainerImmutable();
// }
unsigned int wxItemContainerImmutable_GetCount(const wxItemContainerImmutable * self) {
    return self->GetCount();
}
bool wxItemContainerImmutable_IsEmpty(const wxItemContainerImmutable * self) {
    return self->IsEmpty();
}
wxString *wxItemContainerImmutable_GetString(const wxItemContainerImmutable * self, unsigned int n) {
    return new wxString(self->GetString(n));
}
wxArrayString *wxItemContainerImmutable_GetStrings(const wxItemContainerImmutable * self) {
    return new wxArrayString(self->GetStrings());
}
void wxItemContainerImmutable_SetString(wxItemContainerImmutable * self, unsigned int n, const wxString * string) {
    return self->SetString(n, *string);
}
int wxItemContainerImmutable_FindString(const wxItemContainerImmutable * self, const wxString * string, bool case_sensitive) {
    return self->FindString(*string, case_sensitive);
}

wxItemContainerImmutable *wxRadioBox_AsItemContainerImmutable(wxRadioBox * obj) {
    return static_cast<wxItemContainerImmutable *>(obj);
}
