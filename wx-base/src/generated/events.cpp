#include "generated.h"

#define MAP_RUST_EVT(name) case RUST_EVT_##name: return wxEVT_##name;

enum WxRustEvent {
    RUST_EVT_DROP_FILES,
    RUST_EVT_ICONIZE,
    RUST_EVT_PAINT,
    RUST_EVT_SHOW,
    RUST_EVT_THREAD,
    RUST_EVT_UPDATE_UI,
};
template<typename T> wxEventTypeTag<T> TypeTagOf(int eventType) {
    return wxEVT_NULL;
}
template<> wxEventTypeTag<wxDropFilesEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(DROP_FILES)
    }
    return wxEVT_NULL;
}
template<> wxEventTypeTag<wxIconizeEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(ICONIZE)
    }
    return wxEVT_NULL;
}
template<> wxEventTypeTag<wxPaintEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(PAINT)
    }
    return wxEVT_NULL;
}
template<> wxEventTypeTag<wxShowEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(SHOW)
    }
    return wxEVT_NULL;
}
template<> wxEventTypeTag<wxThreadEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(THREAD)
    }
    return wxEVT_NULL;
}
template<> wxEventTypeTag<wxUpdateUIEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(UPDATE_UI)
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
    // BindIfEventIs<wxFooEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxDropFilesEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxIconizeEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxPaintEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxShowEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxThreadEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxUpdateUIEvent>(self, eventType, aFn, aParam);
}
