#include "generated.h"

extern "C" {

// CLASS: wxVariantData
wxVariantData * wxVariantData_Clone(const wxVariantData * self) {
    return self->Clone();
}
void wxVariantData_DecRef(wxVariantData * self) {
    return self->DecRef();
}
bool wxVariantData_Eq(const wxVariantData * self, wxVariantData * data) {
    return self->Eq(*data);
}
wxString *wxVariantData_GetType(const wxVariantData * self) {
    return new wxString(self->GetType());
}
wxClassInfo * wxVariantData_GetValueClassInfo(wxVariantData * self) {
    return self->GetValueClassInfo();
}
void wxVariantData_IncRef(wxVariantData * self) {
    return self->IncRef();
}
bool wxVariantData_Read1(wxVariantData * self, wxString * string) {
    return self->Read(*string);
}
bool wxVariantData_Write1(const wxVariantData * self, wxString * string) {
    return self->Write(*string);
}

} // extern "C"

