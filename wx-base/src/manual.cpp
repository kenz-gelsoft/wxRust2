#include <wx/bookctrl.h>

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
// TODO: auto generate
enum WxRustEvent {
    RUST_EVT_BOOKCTRL_PAGE_CHANGED,
    RUST_EVT_BUTTON,
    RUST_EVT_MENU,
};
template<typename T> wxEventTypeTag<T> TypeTagOf(int eventType) {
    return wxEVT_NULL;
}
template<> wxEventTypeTag<wxBookCtrlEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    case RUST_EVT_BOOKCTRL_PAGE_CHANGED:
        return wxEVT_BOOKCTRL_PAGE_CHANGED;
    }
    return wxEVT_NULL;
}
template<> wxEventTypeTag<wxCommandEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    case RUST_EVT_BUTTON:
        return wxEVT_BUTTON;
    case RUST_EVT_MENU:
        return wxEVT_MENU;
    }
    return wxEVT_NULL;
}
template<typename T> void BindIfEventIs(wxEvtHandler *self, int eventType, void *aFn, void *aParam) {
    wxEventTypeTag<T> typeTag = TypeTagOf<T>(eventType);
    if (typeTag != wxEVT_NULL) {
        CxxClosure<T &> functor(aFn, aParam);
        self->Bind(typeTag, functor);
    }
}
void wxEvtHandler_Bind(wxEvtHandler *self, int eventType, void *aFn, void *aParam) {
    BindIfEventIs<wxBookCtrlEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxCommandEvent>(self, eventType, aFn, aParam);
}

// String
wxString *wxString_new(const unsigned char *psz, const size_t nLength) {
    return new wxString(psz, wxConvUTF8, nLength);
}
UTF8Data wxString_UTF8Data(wxString *self) {
    auto utf8 = self->ToUTF8();
    return {
        utf8.data(),
        utf8.length()
    };
}

// ArrayString
wxArrayString *wxArrayString_new() {
    return new wxArrayString();
}
void wxArrayString_delete(wxArrayString *self) {
    delete self;
}
void wxArrayString_Add(wxArrayString *self, const wxString *s) {
    self->Add(*s);
}


class OpaqueWeakRef : public wxTrackerNode
{
public:
    OpaqueWeakRef(void *ptr) :
        mPtr(ptr)
    {
        AsTrackable()->AddNode(this);
    }
    virtual ~OpaqueWeakRef()
    {
        AsTrackable()->RemoveNode(this);
        mPtr = nullptr;
    }
    void *Get() const
    {
        return mPtr;
    }
    virtual void OnObjectDestroy()
    {
        mPtr = nullptr;
    }
private:
    wxTrackable *AsTrackable() const
    {
        // Casting to any(not true) class ptr to use dynamic_cast,
        wxObject *obj = reinterpret_cast<wxObject *>(mPtr);
        // we need to dynamic_cast to get correct wxTrackable's vtable.
        wxTrackable *trackable = dynamic_cast<wxTrackable *>(obj);
        wxASSERT(trackable);
        return trackable;
    }
    void *mPtr;
}; 

void *OpaqueWeakRef_new(void *obj) {
    return new OpaqueWeakRef(obj);
}
void OpaqueWeakRef_delete(void *self) {
    OpaqueWeakRef *weakRef = reinterpret_cast<OpaqueWeakRef *>(self);
    delete weakRef;
}
void *OpaqueWeakRef_Get(void *self) {
    OpaqueWeakRef *weakRef = reinterpret_cast<OpaqueWeakRef *>(self);
    return weakRef->Get();
}

int wxRustEntry(int *argc, char **argv) {
    return wxEntry(*argc, argv);
}
