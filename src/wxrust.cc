#include "wx/include/wxrust.h"

// wxApp
wxIMPLEMENT_APP_NO_MAIN(WxRustApp);

static rust::Fn<void()> globalOnInit;
static bool globalOnInitSet = false;
void WxRustAppSetOnInit(rust::Fn<void()> aOnInit) {
    globalOnInit = aOnInit;
    globalOnInitSet = true;
}
bool WxRustApp::OnInit() {
    if (globalOnInitSet) {
        (*globalOnInit)();
    } else {
        // TODO: provide debug info
    }
    return true;
}

// wxFrame
wxFrame *wxFrame_new(rust::Str title) {
    return new wxFrame(NULL, -1, std::string(title));
}
