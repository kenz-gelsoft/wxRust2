#pragma once

#include <wx/variant.h>

extern "C" {

// CLASS: wxVariantData
wxVariantData * wxVariantData_Clone(const wxVariantData * self);
void wxVariantData_DecRef(wxVariantData * self);
bool wxVariantData_Eq(const wxVariantData * self, wxVariantData * data);
wxString *wxVariantData_GetType(const wxVariantData * self);
wxClassInfo * wxVariantData_GetValueClassInfo(wxVariantData * self);
void wxVariantData_IncRef(wxVariantData * self);
bool wxVariantData_Read1(wxVariantData * self, wxString * string);
bool wxVariantData_Write1(const wxVariantData * self, wxString * string);

} // extern "C"

