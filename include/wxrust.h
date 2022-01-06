#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"
#include "wx/src/lib.rs.h"


namespace wxrust {
    using unsafe_any_ptr = const char *;

    // wxApp
    void AppSetOnInit(const Closure &closure);
    class App : public wxApp {
        virtual bool OnInit();
    };

    // wxEvtHandler
    template <typename T>
    class CxxClosure {
        typedef void (*TrampolineFunc)(unsafe_any_ptr);
        wxrust::Closure mClosure;

    public:
        CxxClosure() : mClosure() {}
        CxxClosure(const wxrust::Closure &closure) : mClosure(closure) {}

        void operator ()(T arg) const {
            if (mClosure.param) { // if set
                ((TrampolineFunc)mClosure.f)(mClosure.param);
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
    void Bind(wxEvtHandler &evtHandler, EventType eventType, const Closure &closure) {
        CxxClosure<wxCommandEvent &> functor(closure);
        evtHandler.Bind(fromRustEventType(eventType), functor);
    }
}

// wxFrame
wxFrame *wxFrame_new(rust::Str aTitle);

// wxString
wxString *wxString_from(rust::Str aString);

// wxButton
wxButton *wxButton_new(wxWindow &parent, rust::Str label);
