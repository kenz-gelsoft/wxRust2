#include "wx/include/wxrust.h"

// wxApp
wxIMPLEMENT_APP_NO_MAIN(WxRustApp);

struct WxRustClosure {
    rust::Fn<void(unsafe_any_ptr)> f;
    unsafe_any_ptr param;

    void operator ()() const {
        if (param) { // if set
            f(param);
        } else {
            // TODO: provide debug info
        }
    }
};

static WxRustClosure globalOnInit;
void WxRustAppSetOnInit(
    rust::Fn<void(unsafe_any_ptr)> aFn,
    unsafe_any_ptr aParam
) {
    globalOnInit = { aFn, aParam };
}
bool WxRustApp::OnInit() {
    globalOnInit();
    return true;
}

// wxFrame
wxFrame *wxFrame_new(rust::Str title) {
    return new wxFrame(NULL, -1, std::string(title));
}

// wxButton
wxButton *wxButton_new(wxWindow &parent, rust::Str label) {
    return new wxButton(&parent, wxID_ANY, std::string(label));
}
