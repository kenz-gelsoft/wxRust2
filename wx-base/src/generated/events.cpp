#include "generated.h"


enum WxRustEvent {
    RUST_EVT_BUTTON,
    RUST_EVT_CHECKBOX,
    RUST_EVT_CHECKLISTBOX,
    RUST_EVT_CHOICE,
    RUST_EVT_COMBOBOX,
    RUST_EVT_COMBOBOX_CLOSEUP,
    RUST_EVT_COMBOBOX_DROPDOWN,
    RUST_EVT_COMMAND_ENTER,
    RUST_EVT_COMMAND_KILL_FOCUS,
    RUST_EVT_COMMAND_LEFT_CLICK,
    RUST_EVT_COMMAND_LEFT_DCLICK,
    RUST_EVT_COMMAND_RIGHT_CLICK,
    RUST_EVT_COMMAND_RIGHT_DCLICK,
    RUST_EVT_COMMAND_SET_FOCUS,
    RUST_EVT_LISTBOX,
    RUST_EVT_LISTBOX_DCLICK,
    RUST_EVT_MENU,
    RUST_EVT_RADIOBOX,
    RUST_EVT_RADIOBUTTON,
    RUST_EVT_SCROLLBAR,
    RUST_EVT_SLIDER,
    RUST_EVT_TEXT,
    RUST_EVT_TOOL_ENTER,
    RUST_EVT_TOOL_DROPDOWN,
    RUST_EVT_TOOL_RCLICKED,
    RUST_EVT_VLBOX,
    RUST_EVT_DROP_FILES,
    RUST_EVT_ICONIZE,
    RUST_EVT_PAINT,
    RUST_EVT_SHOW,
    RUST_EVT_THREAD,
    RUST_EVT_UPDATE_UI,
};

#define MAP_RUST_EVT(name) case RUST_EVT_##name: return wxEVT_##name;
#define DEFINE_TYPE_TAG_OF_EVT(name, clazz) \
    template<> wxEventTypeTag<clazz> TypeTagOf(int eventType) { \
        switch (eventType) { \
        MAP_RUST_EVT(name) \
        } \
        return wxEVT_NULL; \
    }

template<typename T> wxEventTypeTag<T> TypeTagOf(int eventType) {
    return wxEVT_NULL;
}
template<> wxEventTypeTag<wxCommandEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(BUTTON)
    MAP_RUST_EVT(CHECKBOX)
    MAP_RUST_EVT(CHECKLISTBOX)
    MAP_RUST_EVT(CHOICE)
    MAP_RUST_EVT(COMBOBOX)
    MAP_RUST_EVT(COMBOBOX_CLOSEUP)
    MAP_RUST_EVT(COMBOBOX_DROPDOWN)
    MAP_RUST_EVT(COMMAND_ENTER)
    MAP_RUST_EVT(COMMAND_KILL_FOCUS)
    MAP_RUST_EVT(COMMAND_LEFT_CLICK)
    MAP_RUST_EVT(COMMAND_LEFT_DCLICK)
    MAP_RUST_EVT(COMMAND_RIGHT_CLICK)
    MAP_RUST_EVT(COMMAND_RIGHT_DCLICK)
    MAP_RUST_EVT(COMMAND_SET_FOCUS)
    MAP_RUST_EVT(LISTBOX)
    MAP_RUST_EVT(LISTBOX_DCLICK)
    MAP_RUST_EVT(MENU)
    MAP_RUST_EVT(RADIOBOX)
    MAP_RUST_EVT(RADIOBUTTON)
    MAP_RUST_EVT(SCROLLBAR)
    MAP_RUST_EVT(SLIDER)
    MAP_RUST_EVT(TEXT)
    MAP_RUST_EVT(TOOL_ENTER)
    MAP_RUST_EVT(TOOL_DROPDOWN)
    MAP_RUST_EVT(TOOL_RCLICKED)
    MAP_RUST_EVT(VLBOX)
    }
    return wxEVT_NULL;
}
DEFINE_TYPE_TAG_OF_EVT(DROP_FILES, wxDropFilesEvent)
DEFINE_TYPE_TAG_OF_EVT(ICONIZE, wxIconizeEvent)
DEFINE_TYPE_TAG_OF_EVT(PAINT, wxPaintEvent)
DEFINE_TYPE_TAG_OF_EVT(SHOW, wxShowEvent)
DEFINE_TYPE_TAG_OF_EVT(THREAD, wxThreadEvent)
DEFINE_TYPE_TAG_OF_EVT(UPDATE_UI, wxUpdateUIEvent)
template<typename T> void BindIfEventIs(wxEvtHandler *self, int eventType, void *aFn, void *aParam) {
    wxEventTypeTag<T> typeTag = TypeTagOf<T>(eventType);
    if (typeTag != wxEVT_NULL) {
        CxxClosure<T &> functor(aFn, aParam);
        self->Bind(typeTag, functor);
    }
}
void wxEvtHandler_Bind(wxEvtHandler *self, int eventType, void *aFn, void *aParam) {
    // BindIfEventIs<wxFooEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxCommandEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxDropFilesEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxIconizeEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxPaintEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxShowEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxThreadEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxUpdateUIEvent>(self, eventType, aFn, aParam);
}
