#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"
#include "wx/src/lib.rs.h"


namespace wxrust {

using UnsafeAnyPtr = const char *;

// wxApp
void AppSetOnInit(
    UnsafeAnyPtr aFn,
    UnsafeAnyPtr aParam
);
class App : public wxApp {
    virtual bool OnInit();
};

// wxEvtHandler
template <typename T>
class CxxClosure {
    typedef void (*TrampolineFunc)(UnsafeAnyPtr);
    UnsafeAnyPtr mFn;
    UnsafeAnyPtr mParam;

public:
    CxxClosure() : mFn(), mParam()
    {}
    CxxClosure(UnsafeAnyPtr f, UnsafeAnyPtr param) :
        mFn(f),
        mParam(param)
    {}

    void operator ()(T arg) const {
        if (mParam) { // if set
            ((TrampolineFunc)mFn)(mParam);
        } else {
            // TODO: provide debug info
        }
    }
};

// TODO: auto generate
#define wxRUST_EVT_BUTTON 0

void wxEvtHandler_Bind(wxEvtHandler *evtHandler, int eventType, UnsafeAnyPtr aFn, UnsafeAnyPtr aParam);

// Constructors
wxString *wxString_new(const unsigned char *aString, size_t aLen);

int wxRustEntry(int *argc, char **argv);

} // namespace wxrust
