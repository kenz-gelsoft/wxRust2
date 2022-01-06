#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"
#include "wx/src/lib.rs.h"


namespace wxrust {
    using UnsafeAnyPtr = const char *;

    // wxApp
    void AppSetOnInit(const Closure &closure);
    class App : public wxApp {
        virtual bool OnInit();
    };

    // wxEvtHandler
    template <typename T>
    class CxxClosure {
        typedef void (*TrampolineFunc)(UnsafeAnyPtr);
        Closure mClosure;

    public:
        CxxClosure() : mClosure() {}
        CxxClosure(const Closure &closure) : mClosure(closure) {}

        void operator ()(T arg) const {
            if (mClosure.param) { // if set
                ((TrampolineFunc)mClosure.f)(mClosure.param);
            } else {
                // TODO: provide debug info
            }
        }
    };

    inline wxEventTypeTag<wxCommandEvent> FromRustEventType(EventType eventType) {
        switch (eventType) {
        case EventType::Button:
            return wxEVT_BUTTON;
        }
        return wxEVT_BUTTON;
    }
    void Bind(wxEvtHandler &evtHandler, EventType eventType, const Closure &closure) {
        CxxClosure<wxCommandEvent &> functor(closure);
        evtHandler.Bind(FromRustEventType(eventType), functor);
    }

    // wxFrame
    wxFrame *NewFrame(rust::Str aTitle);

    // wxString
    wxString *NewString(rust::Str aString);

    // wxButton
    wxButton *NewButton(wxWindow &parent, rust::Str label);
}

