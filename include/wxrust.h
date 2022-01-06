#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"
#include "wx/src/lib.rs.h"


using unsafe_any_ptr = const char *;

// wxApp
void WxRustAppSetOnInit(const wxrust::Closure &closure);
class WxRustApp : public wxApp {
    virtual bool OnInit();
};

// wxEvtHandler
template <typename T>
class WxRustClosure {
    typedef void (*TrampolineFunc)(unsafe_any_ptr);
    wxrust::Closure mClosure;

public:
    WxRustClosure() : mClosure() {}
    WxRustClosure(const wxrust::Closure &closure) : mClosure(closure) {}

    void operator ()(T arg) const {
        if (mClosure.param) { // if set
            ((TrampolineFunc)mClosure.f)(mClosure.param);
        } else {
            // TODO: provide debug info
        }
    }
};

inline wxEventTypeTag<wxCommandEvent> fromRustEventType(wxrust::EventType eventType) {
    switch (eventType) {
    case wxrust::EventType::Button:
        return wxEVT_BUTTON;
    }
    return wxEVT_BUTTON;
}

void Bind(wxEvtHandler &evtHandler, wxrust::EventType eventType, const wxrust::Closure &closure) {
    WxRustClosure<wxCommandEvent &> functor(closure);
    evtHandler.Bind(fromRustEventType(eventType), functor);
}

// wxFrame
wxFrame *wxFrame_new(rust::Str aTitle);

// wxString
wxString *wxString_from(rust::Str aString);

// wxButton
wxButton *wxButton_new(wxWindow &parent, rust::Str label);
