#include "generated.h"

extern "C" {

// CLASS: wxClassInfo
void wxClassInfo_delete(wxClassInfo *self) {
    delete self;
}
wxObject * wxClassInfo_CreateObject(const wxClassInfo * self) {
    return self->CreateObject();
}
const wxChar * wxClassInfo_GetBaseClassName1(const wxClassInfo * self) {
    return self->GetBaseClassName1();
}
const wxChar * wxClassInfo_GetBaseClassName2(const wxClassInfo * self) {
    return self->GetBaseClassName2();
}
const wxChar * wxClassInfo_GetClassName(const wxClassInfo * self) {
    return self->GetClassName();
}
int wxClassInfo_GetSize(const wxClassInfo * self) {
    return self->GetSize();
}
bool wxClassInfo_IsDynamic(const wxClassInfo * self) {
    return self->IsDynamic();
}
bool wxClassInfo_IsKindOf(const wxClassInfo * self, const wxClassInfo * info) {
    return self->IsKindOf(info);
}
wxClassInfo * wxClassInfo_FindClass(const wxString * class_name) {
    return wxClassInfo::FindClass(*class_name);
}

} // extern "C"

