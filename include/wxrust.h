#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"
#include "wx/src/lib.rs.h"


using unsafe_any_ptr = const char *;

// wxApp
void WxRustAppSetOnInit(
    rust::Fn<void(unsafe_any_ptr)> aFn,
    unsafe_any_ptr aParam
);
class WxRustApp : public wxApp {
    virtual bool OnInit();
};

// wxEvtHandler
template <typename T>
struct WxRustClosure {
    rust::Fn<void(unsafe_any_ptr)> f;
    unsafe_any_ptr param;

    void operator ()(T arg) const {
        if (param) { // if set
            f(param);
        } else {
            // TODO: provide debug info
        }
    }
};

inline wxEventTypeTag<wxCommandEvent> fromRustEventType(EventType eventType) {
    switch (eventType) {
    case EventType::Button:
        return wxEVT_BUTTON;
    }
    return wxEVT_BUTTON;
}

void Bind(
    wxEvtHandler &evtHandler,
    EventType eventType,
    rust::Fn<void(unsafe_any_ptr)> aFn,
    unsafe_any_ptr aParam
) {
    WxRustClosure<wxCommandEvent &> functor = { aFn, aParam };
    evtHandler.Bind(fromRustEventType(eventType), functor);
}

// wxFrame
wxFrame *wxFrame_new(rust::Str aTitle);

// wxString
wxString *wxString_from(rust::Str aString);

// wxButton
wxButton *wxButton_new(wxWindow &parent, rust::Str label);
