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

void Bind(wxEvtHandler &evtHandler, EventType eventType, const Closure &closure);

// Constructors
std::unique_ptr<wxString> NewString(rust::Str aString);
wxFrame *NewFrame(rust::Str aTitle);
wxButton *NewButton(wxWindow &parent, rust::Str label);

} // namespace wxrust
