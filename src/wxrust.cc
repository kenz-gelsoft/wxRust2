#include "wx/include/wxrust.h"

namespace wxrust {

// wxApp
wxIMPLEMENT_APP_NO_MAIN(App);

static CxxClosure<int> globalOnInit;
void AppSetOnInit(const Closure &closure) {
    globalOnInit = closure;
}

bool App::OnInit() {
    globalOnInit(/*unused*/0);
    return true;
}

// wxEvtHandler
wxEventTypeTag<wxCommandEvent> TypeTagOf(EventType eventType) {
    switch (eventType) {
    case EventType::Button:
        return wxEVT_BUTTON;
    }
    return wxEVT_BUTTON;
}
void Bind(wxEvtHandler &evtHandler, EventType eventType, const Closure &closure) {
    CxxClosure<wxCommandEvent &> functor(closure);
    evtHandler.Bind(TypeTagOf(eventType), functor);
}

// Constructors
wxString *wxString_new(const unsigned char *psz, const size_t nLength) {
    return new wxString(psz, wxConvUTF8, nLength);
}

} // namespace wxrust
