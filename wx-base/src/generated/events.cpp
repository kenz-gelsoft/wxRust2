#include <wx/bookctrl.h>

#include "manual.h"


enum WxRustEvent {
    RUST_EVT_ACTIVATE,
    RUST_EVT_ACTIVATE_APP,
    RUST_EVT_AFTER_CHAR,
    RUST_EVT_AUX1_DCLICK,
    RUST_EVT_AUX1_DOWN,
    RUST_EVT_AUX1_UP,
    RUST_EVT_AUX2_DCLICK,
    RUST_EVT_AUX2_DOWN,
    RUST_EVT_AUX2_UP,
    RUST_EVT_BOOKCTRL_PAGE_CHANGED,
    RUST_EVT_BUTTON,
    RUST_EVT_CHAR,
    RUST_EVT_CHAR_HOOK,
    RUST_EVT_CHECKBOX,
    RUST_EVT_CHECKLISTBOX,
    RUST_EVT_CHILD_FOCUS,
    RUST_EVT_CHOICE,
    RUST_EVT_CLOSE_WINDOW,
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
    RUST_EVT_CONTEXT_MENU,
    RUST_EVT_CREATE,
    RUST_EVT_DESTROY,
    RUST_EVT_DETAILED_HELP,
    RUST_EVT_DISPLAY_CHANGED,
    RUST_EVT_DROP_FILES,
    RUST_EVT_END_SESSION,
    RUST_EVT_ENTER_WINDOW,
    RUST_EVT_ERASE_BACKGROUND,
    RUST_EVT_HELP,
    RUST_EVT_HIBERNATE,
    RUST_EVT_ICONIZE,
    RUST_EVT_INIT_DIALOG,
    RUST_EVT_JOY_BUTTON_DOWN,
    RUST_EVT_JOY_BUTTON_UP,
    RUST_EVT_JOY_MOVE,
    RUST_EVT_JOY_ZMOVE,
    RUST_EVT_KEY_DOWN,
    RUST_EVT_KEY_UP,
    RUST_EVT_KILL_FOCUS,
    RUST_EVT_LEAVE_WINDOW,
    RUST_EVT_LEFT_DCLICK,
    RUST_EVT_LEFT_DOWN,
    RUST_EVT_LEFT_UP,
    RUST_EVT_LISTBOX,
    RUST_EVT_LISTBOX_DCLICK,
    RUST_EVT_MAXIMIZE,
    RUST_EVT_MENU,
    RUST_EVT_MENU_CLOSE,
    RUST_EVT_MENU_HIGHLIGHT,
    RUST_EVT_MENU_OPEN,
    RUST_EVT_MIDDLE_DCLICK,
    RUST_EVT_MIDDLE_DOWN,
    RUST_EVT_MIDDLE_UP,
    RUST_EVT_MOTION,
    RUST_EVT_MOUSE_CAPTURE_CHANGED,
    RUST_EVT_MOUSE_CAPTURE_LOST,
    RUST_EVT_MOUSEWHEEL,
    RUST_EVT_MOVE,
    RUST_EVT_MOVE_END,
    RUST_EVT_MOVE_START,
    RUST_EVT_MOVING,
    RUST_EVT_NAVIGATION_KEY,
    RUST_EVT_PAINT,
    RUST_EVT_QUERY_END_SESSION,
    RUST_EVT_RADIOBOX,
    RUST_EVT_RADIOBUTTON,
    RUST_EVT_RIGHT_DCLICK,
    RUST_EVT_RIGHT_DOWN,
    RUST_EVT_RIGHT_UP,
    RUST_EVT_SCROLL_BOTTOM,
    RUST_EVT_SCROLL_CHANGED,
    RUST_EVT_SCROLL_LINEDOWN,
    RUST_EVT_SCROLL_LINEUP,
    RUST_EVT_SCROLL_PAGEDOWN,
    RUST_EVT_SCROLL_PAGEUP,
    RUST_EVT_SCROLL_THUMBRELEASE,
    RUST_EVT_SCROLL_THUMBTRACK,
    RUST_EVT_SCROLL_TOP,
    RUST_EVT_SCROLLBAR,
    RUST_EVT_SCROLLWIN_BOTTOM,
    RUST_EVT_SCROLLWIN_LINEDOWN,
    RUST_EVT_SCROLLWIN_LINEUP,
    RUST_EVT_SCROLLWIN_PAGEDOWN,
    RUST_EVT_SCROLLWIN_PAGEUP,
    RUST_EVT_SCROLLWIN_THUMBRELEASE,
    RUST_EVT_SCROLLWIN_THUMBTRACK,
    RUST_EVT_SCROLLWIN_TOP,
    RUST_EVT_SET_CURSOR,
    RUST_EVT_SET_FOCUS,
    RUST_EVT_SHOW,
    RUST_EVT_SIZE,
    RUST_EVT_SIZING,
    RUST_EVT_SLIDER,
    RUST_EVT_SYS_COLOUR_CHANGED,
    RUST_EVT_TEXT,
    RUST_EVT_TEXT_COPY,
    RUST_EVT_TEXT_CUT,
    RUST_EVT_TEXT_PASTE,
    RUST_EVT_THREAD,
    RUST_EVT_TIMER,
    RUST_EVT_TOOL_DROPDOWN,
    RUST_EVT_TOOL_ENTER,
    RUST_EVT_TOOL_RCLICKED,
    RUST_EVT_UPDATE_UI,
    RUST_EVT_VLBOX,
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
template<> wxEventTypeTag<wxActivateEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(ACTIVATE_APP)
    MAP_RUST_EVT(ACTIVATE)
    MAP_RUST_EVT(HIBERNATE)
    }
    return wxEVT_NULL;
}
DEFINE_TYPE_TAG_OF_EVT(BOOKCTRL_PAGE_CHANGED, wxBookCtrlEvent)
DEFINE_TYPE_TAG_OF_EVT(CHILD_FOCUS, wxChildFocusEvent)
template<> wxEventTypeTag<wxClipboardTextEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(TEXT_COPY)
    MAP_RUST_EVT(TEXT_CUT)
    MAP_RUST_EVT(TEXT_PASTE)
    }
    return wxEVT_NULL;
}
template<> wxEventTypeTag<wxCloseEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(CLOSE_WINDOW)
    MAP_RUST_EVT(END_SESSION)
    MAP_RUST_EVT(QUERY_END_SESSION)
    }
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
    MAP_RUST_EVT(TOOL_DROPDOWN)
    MAP_RUST_EVT(TOOL_ENTER)
    MAP_RUST_EVT(TOOL_RCLICKED)
    MAP_RUST_EVT(VLBOX)
    }
    return wxEVT_NULL;
}
DEFINE_TYPE_TAG_OF_EVT(CONTEXT_MENU, wxContextMenuEvent)
DEFINE_TYPE_TAG_OF_EVT(DISPLAY_CHANGED, wxDisplayChangedEvent)
DEFINE_TYPE_TAG_OF_EVT(DROP_FILES, wxDropFilesEvent)
DEFINE_TYPE_TAG_OF_EVT(ERASE_BACKGROUND, wxEraseEvent)
template<> wxEventTypeTag<wxFocusEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(SET_FOCUS)
    MAP_RUST_EVT(KILL_FOCUS)
    }
    return wxEVT_NULL;
}
template<> wxEventTypeTag<wxHelpEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(HELP)
    MAP_RUST_EVT(DETAILED_HELP)
    }
    return wxEVT_NULL;
}
DEFINE_TYPE_TAG_OF_EVT(ICONIZE, wxIconizeEvent)
DEFINE_TYPE_TAG_OF_EVT(INIT_DIALOG, wxInitDialogEvent)
template<> wxEventTypeTag<wxJoystickEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(JOY_BUTTON_DOWN)
    MAP_RUST_EVT(JOY_BUTTON_UP)
    MAP_RUST_EVT(JOY_MOVE)
    MAP_RUST_EVT(JOY_ZMOVE)
    }
    return wxEVT_NULL;
}
template<> wxEventTypeTag<wxKeyEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(AFTER_CHAR)
    MAP_RUST_EVT(CHAR)
    MAP_RUST_EVT(CHAR_HOOK)
    MAP_RUST_EVT(KEY_DOWN)
    MAP_RUST_EVT(KEY_UP)
    }
    return wxEVT_NULL;
}
DEFINE_TYPE_TAG_OF_EVT(MAXIMIZE, wxMaximizeEvent)
template<> wxEventTypeTag<wxMenuEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(MENU_OPEN)
    MAP_RUST_EVT(MENU_CLOSE)
    MAP_RUST_EVT(MENU_HIGHLIGHT)
    }
    return wxEVT_NULL;
}
DEFINE_TYPE_TAG_OF_EVT(MOUSE_CAPTURE_CHANGED, wxMouseCaptureChangedEvent)
DEFINE_TYPE_TAG_OF_EVT(MOUSE_CAPTURE_LOST, wxMouseCaptureLostEvent)
template<> wxEventTypeTag<wxMouseEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(AUX1_DCLICK)
    MAP_RUST_EVT(AUX1_DOWN)
    MAP_RUST_EVT(AUX1_UP)
    MAP_RUST_EVT(AUX2_DCLICK)
    MAP_RUST_EVT(AUX2_DOWN)
    MAP_RUST_EVT(AUX2_UP)
    MAP_RUST_EVT(ENTER_WINDOW)
    MAP_RUST_EVT(LEAVE_WINDOW)
    MAP_RUST_EVT(LEFT_DCLICK)
    MAP_RUST_EVT(LEFT_DOWN)
    MAP_RUST_EVT(LEFT_UP)
    MAP_RUST_EVT(MIDDLE_DCLICK)
    MAP_RUST_EVT(MIDDLE_DOWN)
    MAP_RUST_EVT(MIDDLE_UP)
    MAP_RUST_EVT(MOTION)
    MAP_RUST_EVT(MOUSEWHEEL)
    MAP_RUST_EVT(RIGHT_DCLICK)
    MAP_RUST_EVT(RIGHT_DOWN)
    MAP_RUST_EVT(RIGHT_UP)
    }
    return wxEVT_NULL;
}
template<> wxEventTypeTag<wxMoveEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(MOVE)
    MAP_RUST_EVT(MOVING)
    MAP_RUST_EVT(MOVE_START)
    MAP_RUST_EVT(MOVE_END)
    }
    return wxEVT_NULL;
}
DEFINE_TYPE_TAG_OF_EVT(NAVIGATION_KEY, wxNavigationKeyEvent)
DEFINE_TYPE_TAG_OF_EVT(PAINT, wxPaintEvent)
template<> wxEventTypeTag<wxScrollEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(SCROLL_TOP)
    MAP_RUST_EVT(SCROLL_BOTTOM)
    MAP_RUST_EVT(SCROLL_LINEUP)
    MAP_RUST_EVT(SCROLL_LINEDOWN)
    MAP_RUST_EVT(SCROLL_PAGEUP)
    MAP_RUST_EVT(SCROLL_PAGEDOWN)
    MAP_RUST_EVT(SCROLL_THUMBTRACK)
    MAP_RUST_EVT(SCROLL_THUMBRELEASE)
    MAP_RUST_EVT(SCROLL_CHANGED)
    }
    return wxEVT_NULL;
}
template<> wxEventTypeTag<wxScrollWinEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(SCROLLWIN_TOP)
    MAP_RUST_EVT(SCROLLWIN_BOTTOM)
    MAP_RUST_EVT(SCROLLWIN_LINEUP)
    MAP_RUST_EVT(SCROLLWIN_LINEDOWN)
    MAP_RUST_EVT(SCROLLWIN_PAGEUP)
    MAP_RUST_EVT(SCROLLWIN_PAGEDOWN)
    MAP_RUST_EVT(SCROLLWIN_THUMBTRACK)
    MAP_RUST_EVT(SCROLLWIN_THUMBRELEASE)
    }
    return wxEVT_NULL;
}
DEFINE_TYPE_TAG_OF_EVT(SET_CURSOR, wxSetCursorEvent)
DEFINE_TYPE_TAG_OF_EVT(SHOW, wxShowEvent)
template<> wxEventTypeTag<wxSizeEvent> TypeTagOf(int eventType) {
    switch (eventType) {
    MAP_RUST_EVT(SIZE)
    MAP_RUST_EVT(SIZING)
    }
    return wxEVT_NULL;
}
DEFINE_TYPE_TAG_OF_EVT(SYS_COLOUR_CHANGED, wxSysColourChangedEvent)
DEFINE_TYPE_TAG_OF_EVT(THREAD, wxThreadEvent)
DEFINE_TYPE_TAG_OF_EVT(TIMER, wxTimerEvent)
DEFINE_TYPE_TAG_OF_EVT(UPDATE_UI, wxUpdateUIEvent)
DEFINE_TYPE_TAG_OF_EVT(CREATE, wxWindowCreateEvent)
DEFINE_TYPE_TAG_OF_EVT(DESTROY, wxWindowDestroyEvent)

template<typename T> void BindIfEventIs(wxEvtHandler *self, int eventType, void *aFn, void *aParam) {
    wxEventTypeTag<T> typeTag = TypeTagOf<T>(eventType);
    if (typeTag != wxEVT_NULL) {
        CxxClosure<T &> functor(aFn, aParam);
        self->Bind(typeTag, functor);
    }
}
void wxEvtHandler_Bind(wxEvtHandler *self, int eventType, void *aFn, void *aParam) {
    BindIfEventIs<wxActivateEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxBookCtrlEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxChildFocusEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxClipboardTextEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxCloseEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxCommandEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxContextMenuEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxDisplayChangedEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxDropFilesEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxEraseEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxFocusEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxHelpEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxIconizeEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxInitDialogEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxJoystickEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxKeyEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxMaximizeEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxMenuEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxMouseCaptureChangedEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxMouseCaptureLostEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxMouseEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxMoveEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxNavigationKeyEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxPaintEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxScrollEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxScrollWinEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxSetCursorEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxShowEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxSizeEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxSysColourChangedEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxThreadEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxTimerEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxUpdateUIEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxWindowCreateEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxWindowDestroyEvent>(self, eventType, aFn, aParam);
}
