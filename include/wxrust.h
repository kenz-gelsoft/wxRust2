#pragma once
#include <wx/wx.h>


// wxEvtHandler
template <typename T>
class CxxClosure {
    typedef void (*TrampolineFunc)(void *);
    void *mFn;
    void *mParam;

public:
    CxxClosure() : mFn(), mParam()
    {}
    CxxClosure(void *f, void *param) : mFn(f), mParam(param)
    {}

    void operator ()(T arg) const {
        if (mParam) { // if set
            ((TrampolineFunc)mFn)(mParam);
        } else {
            // TODO: provide debug info
        }
    }
};

extern "C" {

// wxApp
void AppSetOnInit(void *aFn, void *aParam);
class App : public wxApp {
    virtual bool OnInit();
};

// TODO: auto generate
#define wxRUST_EVT_BUTTON 0

void wxEvtHandler_Bind(wxEvtHandler *evtHandler, int eventType, void *aFn, void *aParam);

// String
wxString *wxString_new(const unsigned char *psz, const size_t nLength);
const char *wxString_UTF8Data(wxString *self);
size_t wxString_Len(wxString *self);

int wxRustEntry(int *argc, char **argv);

} // extern "C"
