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
void Bind(wxEvtHandler &evtHandler, int eventType, UnsafeAnyPtr aFn, UnsafeAnyPtr aParam) {
    CxxClosure<wxCommandEvent &> functor(aFn, aParam);
    evtHandler.Bind(TypeTagOf(eventType), functor);
}

// Constructors
wxString *wxString_new(const unsigned char *psz, const size_t nLength) {
    return new wxString(psz, wxConvUTF8, nLength);
}

} // namespace wxrust
