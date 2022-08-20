#include "generated.h"

extern "C" {

// CLASS: wxMessageOutput
void wxMessageOutput_delete(wxMessageOutput *self) {
    delete self;
}
wxMessageOutput * wxMessageOutput_Get() {
    return wxMessageOutput::Get();
}
wxMessageOutput * wxMessageOutput_Set(wxMessageOutput * msgout) {
    return wxMessageOutput::Set(msgout);
}
void wxMessageOutput_Output(wxMessageOutput * self, const wxString * str) {
    return self->Output(*str);
}

} // extern "C"

