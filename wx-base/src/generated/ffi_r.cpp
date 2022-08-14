#include "generated.h"

extern "C" {

// CLASS: wxRefCounter
wxRefCounter *wxRefCounter_new() {
    return new wxRefCounter();
}
void wxRefCounter_DecRef(wxRefCounter * self) {
    return self->DecRef();
}
int wxRefCounter_GetRefCount(const wxRefCounter * self) {
    return self->GetRefCount();
}
void wxRefCounter_IncRef(wxRefCounter * self) {
    return self->IncRef();
}

} // extern "C"

