#include "wx/include/wxrust.h"

namespace wxrust {

// wxApp
wxIMPLEMENT_APP_NO_MAIN(App);

static CxxClosure<int> globalOnInit;
void AppSetOnInit(UnsafeAnyPtr f, UnsafeAnyPtr params) {
    globalOnInit = CxxClosure<int>(f, params);
}

bool App::OnInit() {
    globalOnInit(/*unused*/0);
    return true;
}

// wxEvtHandler
wxEventTypeTag<wxCommandEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    case wxRUST_EVT_BUTTON:
        return wxEVT_BUTTON;
    }
    return wxEVT_NULL;
}
void wxEvtHandler_Bind(wxEvtHandler *self, int eventType, UnsafeAnyPtr aFn, UnsafeAnyPtr aParam) {
    CxxClosure<wxCommandEvent &> functor(aFn, aParam);
    self->Bind(TypeTagOf(eventType), functor);
}

// Constructors
wxString *wxString_new(const unsigned char *psz, const size_t nLength) {
    return new wxString(psz, wxConvUTF8, nLength);
}

int wxRustEntry(int *argc, char **argv) {
    return wxEntry(*argc, argv);
}

} // namespace wxrust
