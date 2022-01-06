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
}

// wxFrame
wxFrame *wxFrame_new(rust::Str title) {
    return new wxFrame(NULL, -1, std::string(title));
}

// wxString
wxString *wxString_from(rust::Str aString) {
    return new wxString(std::string(aString).c_str(), wxConvUTF8);
}

// wxButton
wxButton *wxButton_new(wxWindow &parent, rust::Str label) {
    return new wxButton(&parent, wxID_ANY, std::string(label));
}
