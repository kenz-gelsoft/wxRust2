#include "wx/include/wxrust.h"

// wxApp
wxIMPLEMENT_APP_NO_MAIN(WxRustApp);

static WxRustClosure<int> globalOnInit;
void WxRustAppSetOnInit(
    rust::Fn<void(unsafe_any_ptr)> aFn,
    unsafe_any_ptr aParam
) {
    globalOnInit = { aFn, aParam };
}
bool WxRustApp::OnInit() {
    globalOnInit(/*unused*/0);
    return true;
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
