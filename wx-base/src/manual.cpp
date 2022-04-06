#include "manual.h"

// wxApp
wxIMPLEMENT_APP_NO_MAIN(App);

static CxxClosure<int> globalOnInit;
void AppSetOnInit(void *f, void *params) {
    globalOnInit = CxxClosure<int>(f, params);
}

bool App::OnInit() {
    globalOnInit(/*unused*/0);
    return true;
}

void wxObject_delete(wxObject *self) {
    delete self;
}

// wxEvtHandler
wxEventTypeTag<wxCommandEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    case wxRUST_EVT_BUTTON:
        return wxEVT_BUTTON;
    case wxRUST_EVT_MENU:
        return wxEVT_MENU;
    }
    return wxEVT_NULL;
}
void wxEvtHandler_Bind(wxEvtHandler *self, int eventType, void *aFn, void *aParam) {
    CxxClosure<wxCommandEvent &> functor(aFn, aParam);
    self->Bind(TypeTagOf(eventType), functor);
}

// String
wxString *wxString_new(const unsigned char *psz, const size_t nLength) {
    return new wxString(psz, wxConvUTF8, nLength);
}
const char *wxString_UTF8Data(wxString *self) {
    return self->ToUTF8().data();
}
size_t wxString_Len(wxString *self) {
    return self->Len();
}

int wxRustEntry(int *argc, char **argv) {
    return wxEntry(*argc, argv);
}
