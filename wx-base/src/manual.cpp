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
    // MEMO: wx32 introduced event types are commented out
    RUST_EVT_ASYNC_METHOD_CALL,
    RUST_EVT_AUX1_DCLICK,
    RUST_EVT_AUX1_DOWN,
    RUST_EVT_AUX1_UP,
    RUST_EVT_AUX2_DCLICK,
    RUST_EVT_AUX2_DOWN,
    RUST_EVT_AUX2_UP,
    RUST_EVT_BOOKCTRL_PAGE_CHANGED,
    RUST_EVT_BUTTON,
    RUST_EVT_CHECKBOX,
    RUST_EVT_CHECKLISTBOX,
    RUST_EVT_CHILD_FOCUS,
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
    RUST_EVT_CONTEXT_MENU,
    RUST_EVT_CREATE,
    RUST_EVT_DESTROY,
    RUST_EVT_DISPLAY_CHANGED,
    // RUST_EVT_DPI_CHANGED,
    RUST_EVT_DROP_FILES,
    RUST_EVT_ENTER_WINDOW,
    RUST_EVT_ERASE_BACKGROUND,
    // RUST_EVT_FULLSCREEN,
    // RUST_EVT_GESTURE_PAN,
    // RUST_EVT_GESTURE_ROTATE,
    // RUST_EVT_GESTURE_ZOOM,
    RUST_EVT_ICONIZE,
    RUST_EVT_IDLE,
    RUST_EVT_INIT_DIALOG,
    RUST_EVT_LEAVE_WINDOW,
    RUST_EVT_LEFT_DCLICK,
    RUST_EVT_LEFT_DOWN,
    RUST_EVT_LEFT_UP,
    RUST_EVT_LISTBOX,
    RUST_EVT_LISTBOX_DCLICK,
    // RUST_EVT_LONG_PRESS,
    RUST_EVT_MAXIMIZE,
    RUST_EVT_MAGNIFY,
    RUST_EVT_MENU,
    RUST_EVT_MIDDLE_DCLICK,
    RUST_EVT_MIDDLE_DOWN,
    RUST_EVT_MIDDLE_UP,
    RUST_EVT_MOTION,
    RUST_EVT_MOUSEWHEEL,
    RUST_EVT_MOUSE_CAPTURE_CHANGED,
    RUST_EVT_MOUSE_CAPTURE_LOST,
    RUST_EVT_NAVIGATION_KEY,
    RUST_EVT_NC_PAINT,
    RUST_EVT_PAINT,
    RUST_EVT_PALETTE_CHANGED,
    // RUST_EVT_PRESS_AND_TAP,
    RUST_EVT_QUERY_NEW_PALETTE,
    RUST_EVT_RADIOBOX,
    RUST_EVT_RADIOBUTTON,
    RUST_EVT_RIGHT_DCLICK,
    RUST_EVT_RIGHT_DOWN,
    RUST_EVT_RIGHT_UP,
    RUST_EVT_SCROLLBAR,
    RUST_EVT_SET_CURSOR,
    RUST_EVT_SHOW,
    RUST_EVT_SLIDER,
    RUST_EVT_SYS_COLOUR_CHANGED,
    RUST_EVT_TEXT,
    RUST_EVT_THREAD,
    RUST_EVT_TIMER,
    RUST_EVT_TOOL_DROPDOWN,
    RUST_EVT_TOOL_ENTER,
    RUST_EVT_TOOL_RCLICKED,
    // RUST_EVT_TWO_FINGER_TAP,
    RUST_EVT_UPDATE_UI,
    RUST_EVT_VLBOX,
};
template<typename T> wxEventTypeTag<T> TypeTagOf(int eventType) {
    return wxEVT_NULL;
}
#define MAP_RUST_EVT(name) case RUST_EVT_##name: return wxEVT_##name;
#define DEFINE_TYPE_TAG_OF_EVT(name, clazz) \
    template<> wxEventTypeTag<clazz> TypeTagOf(int eventType) { \
        switch (eventType) { \
        MAP_RUST_EVT(name) \
        } \
        return wxEVT_NULL; \
    }
DEFINE_TYPE_TAG_OF_EVT(ASYNC_METHOD_CALL, wxAsyncMethodCallEvent)
DEFINE_TYPE_TAG_OF_EVT(BOOKCTRL_PAGE_CHANGED, wxBookCtrlEvent)
DEFINE_TYPE_TAG_OF_EVT(CHILD_FOCUS, wxChildFocusEvent)
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
    MAP_RUST_EVT(MAGNIFY)
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
DEFINE_TYPE_TAG_OF_EVT(CONTEXT_MENU, wxContextMenuEvent)
DEFINE_TYPE_TAG_OF_EVT(CREATE, wxWindowCreateEvent)
DEFINE_TYPE_TAG_OF_EVT(DESTROY, wxWindowDestroyEvent)
DEFINE_TYPE_TAG_OF_EVT(DISPLAY_CHANGED, wxDisplayChangedEvent)
// DEFINE_TYPE_TAG_OF_EVT(DPI_CHANGED, wxDPIChangedEvent)
DEFINE_TYPE_TAG_OF_EVT(DROP_FILES, wxDropFilesEvent)
DEFINE_TYPE_TAG_OF_EVT(ERASE_BACKGROUND, wxEraseEvent)
// DEFINE_TYPE_TAG_OF_EVT(FULLSCREEN, wxFullScreenEvent)
// DEFINE_TYPE_TAG_OF_EVT(GESTURE_PAN, wxPanGestureEvent)
// DEFINE_TYPE_TAG_OF_EVT(GESTURE_ROTATE, wxRotateGestureEvent)
// DEFINE_TYPE_TAG_OF_EVT(GESTURE_ZOOM, wxZoomGestureEvent)
DEFINE_TYPE_TAG_OF_EVT(ICONIZE, wxIconizeEvent)
DEFINE_TYPE_TAG_OF_EVT(IDLE, wxIdleEvent)
DEFINE_TYPE_TAG_OF_EVT(INIT_DIALOG, wxInitDialogEvent)
// DEFINE_TYPE_TAG_OF_EVT(LONG_PRESS, wxLongPressEvent)
DEFINE_TYPE_TAG_OF_EVT(MAXIMIZE, wxMaximizeEvent)
DEFINE_TYPE_TAG_OF_EVT(MOUSE_CAPTURE_CHANGED, wxMouseCaptureChangedEvent)
DEFINE_TYPE_TAG_OF_EVT(MOUSE_CAPTURE_LOST, wxMouseCaptureLostEvent)
DEFINE_TYPE_TAG_OF_EVT(NAVIGATION_KEY, wxNavigationKeyEvent)
DEFINE_TYPE_TAG_OF_EVT(NC_PAINT, wxNcPaintEvent)
DEFINE_TYPE_TAG_OF_EVT(PAINT, wxPaintEvent)
DEFINE_TYPE_TAG_OF_EVT(PALETTE_CHANGED, wxPaletteChangedEvent)
// DEFINE_TYPE_TAG_OF_EVT(PRESS_AND_TAP, wxPressAndTapEvent)
DEFINE_TYPE_TAG_OF_EVT(QUERY_NEW_PALETTE, wxQueryNewPaletteEvent)
DEFINE_TYPE_TAG_OF_EVT(SET_CURSOR, wxSetCursorEvent)
DEFINE_TYPE_TAG_OF_EVT(SHOW, wxShowEvent)
DEFINE_TYPE_TAG_OF_EVT(SYS_COLOUR_CHANGED, wxSysColourChangedEvent)
DEFINE_TYPE_TAG_OF_EVT(THREAD, wxThreadEvent)
DEFINE_TYPE_TAG_OF_EVT(TIMER, wxTimerEvent)
// DEFINE_TYPE_TAG_OF_EVT(TWO_FINGER_TAP, wxTwoFingerTapEvent)
DEFINE_TYPE_TAG_OF_EVT(UPDATE_UI, wxUpdateUIEvent)
template<typename T> void BindIfEventIs(wxEvtHandler *self, int eventType, void *aFn, void *aParam) {
    wxEventTypeTag<T> typeTag = TypeTagOf<T>(eventType);
    if (typeTag != wxEVT_NULL) {
        CxxClosure<T &> functor(aFn, aParam);
        self->Bind(typeTag, functor);
    }
}
void wxEvtHandler_Bind(wxEvtHandler *self, int eventType, void *aFn, void *aParam) {
    BindIfEventIs<wxAsyncMethodCallEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxBookCtrlEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxChildFocusEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxCommandEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxContextMenuEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxWindowCreateEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxWindowDestroyEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxDisplayChangedEvent>(self, eventType, aFn, aParam);
    // BindIfEventIs<wxDPIChangedEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxDropFilesEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxEraseEvent>(self, eventType, aFn, aParam);
    // BindIfEventIs<wxFullScreenEvent>(self, eventType, aFn, aParam);
    // BindIfEventIs<wxPanGestureEvent>(self, eventType, aFn, aParam);
    // BindIfEventIs<wxRotateGestureEvent>(self, eventType, aFn, aParam);
    // BindIfEventIs<wxZoomGestureEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxIconizeEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxIdleEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxInitDialogEvent>(self, eventType, aFn, aParam);
    // BindIfEventIs<wxLongPressEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxMaximizeEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxMenuEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxMouseCaptureChangedEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxMouseCaptureLostEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxNavigationKeyEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxNcPaintEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxPaintEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxPaletteChangedEvent>(self, eventType, aFn, aParam);
    // BindIfEventIs<wxPressAndTapEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxQueryNewPaletteEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxSetCursorEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxShowEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxSysColourChangedEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxThreadEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxTimerEvent>(self, eventType, aFn, aParam);
    // BindIfEventIs<wxTwoFingerTapEvent>(self, eventType, aFn, aParam);
    BindIfEventIs<wxUpdateUIEvent>(self, eventType, aFn, aParam);
}

void wxEvtHandler_CallAfter(wxEvtHandler *self, void *aFn, void *aParam) {
    CxxClosureVoid functor(aFn, aParam);
    self->CallAfter(functor);
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
