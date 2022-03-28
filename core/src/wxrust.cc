#include "wxrust.h"

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
    }
    return wxEVT_NULL;
}
void wxEvtHandler_Bind(wxEvtHandler *self, int eventType, void *aFn, void *aParam) {
    CxxClosure<wxCommandEvent &> functor(aFn, aParam);
    self->Bind(TypeTagOf(eventType), functor);
}
