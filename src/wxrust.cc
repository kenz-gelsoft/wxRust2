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

    // wxFrame
    wxFrame *NewFrame(rust::Str title) {
        return new wxFrame(NULL, -1, std::string(title));
    }

    // wxString
    wxString *NewString(rust::Str aString) {
        return new wxString(std::string(aString).c_str(), wxConvUTF8);
    }

    // wxButton
    wxButton *NewButton(wxWindow &parent, rust::Str label) {
        return new wxButton(&parent, wxID_ANY, std::string(label));
    }
}
