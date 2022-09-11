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
    RUST_EVT_CHECKBOX,
    RUST_EVT_CHECKLISTBOX,
    RUST_EVT_CHOICE,
    RUST_EVT_LISTBOX,
    RUST_EVT_LISTBOX_DCLICK,
    RUST_EVT_MENU,
    RUST_EVT_SLIDER,
    RUST_EVT_RADIOBOX,
    RUST_EVT_RADIOBUTTON,
    RUST_EVT_TIMER,
};
template<typename T> wxEventTypeTag<T> TypeTagOf(int eventType) {
    return wxEVT_NULL;
}
#define MAP_RUST_EVT(name) case RUST_EVT_##name: return wxEVT_##name;
#define DEFINE_TYPE_TAG_OF_EVT(name, clazz) \
    template<> wxEventTypeTag<clazz> TypeTagOf(int eventType) { \
        return eventType == RUST_EVT_##name ? wxEVT_##name : wxEVT_NULL; \
    }
DEFINE_TYPE_TAG_OF_EVT(BOOKCTRL_PAGE_CHANGED, wxBookCtrlEvent)
template<> wxEventTypeTag<wxCommandEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(BUTTON)
    MAP_RUST_EVT(CHECKBOX)
    MAP_RUST_EVT(CHECKLISTBOX)
    MAP_RUST_EVT(CHOICE)
    MAP_RUST_EVT(LISTBOX)
    MAP_RUST_EVT(LISTBOX_DCLICK)
    MAP_RUST_EVT(MENU)
    MAP_RUST_EVT(SLIDER)
    MAP_RUST_EVT(RADIOBOX)
    MAP_RUST_EVT(RADIOBUTTON)
    }
    return wxEVT_NULL;
}
DEFINE_TYPE_TAG_OF_EVT(TIMER, wxTimerEvent)
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
    BindIfEventIs<wxTimerEvent>(self, eventType, aFn, aParam);
}

// String
wxString *wxString_new(const unsigned char *psz, const size_t nLength) {
    return new wxString(psz, wxConvUTF8, nLength);
}
void wxString_delete(wxString *self) {
    delete self;
}
UTF8Data wxString_UTF8Data(wxString *self) {
    auto utf8 = self->ToUTF8();
    return {
        utf8.data(),
        utf8.length()
    };
}

// (wx)String::const_iterator
wxString::const_iterator *wxStringConstIterator_new() {
    return new wxString::const_iterator;
}
void wxStringConstIterator_delete(wxString::const_iterator *self) {
    delete self;
}
size_t wxStringConstIterator_IndexIn(wxString::const_iterator *self, const wxString *s) {
    return std::distance(s->begin(), *self);
}

// ArrayInt
wxArrayInt *wxArrayInt_new() {
    return new wxArrayInt();
}
void wxArrayInt_delete(wxArrayInt *self) {
    delete self;
}
void wxArrayInt_Add(wxArrayInt *self, int i) {
    self->Add(i);
}
int wxArrayInt_Item(wxArrayInt *self, size_t index) {
    return self->Item(index);
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

// DateTime
bool wxDateTime_ParseDate(wxDateTime * self, const wxString * date, wxString::const_iterator * end) {
    return self->ParseDate(*date, end);
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
void *OpaqueWeakRef_copy(void *obj) {
    return new OpaqueWeakRef(OpaqueWeakRef_Get(obj));
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
