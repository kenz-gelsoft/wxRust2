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
wxString *wxString_new(rust::Str aString) {
    return new wxString(std::string(aString).c_str(), wxConvUTF8);
}
wxFrame *NewFrame(rust::Str title) {
    return new wxFrame(NULL, -1, std::string(title));
}
wxButton *NewButton(wxWindow &parent, rust::Str label) {
    return new wxButton(&parent, wxID_ANY, std::string(label));
}

} // namespace wxrust
