#pragma once
#include <wx/wx.h>


// wxEvtHandler
template <typename T>
class CxxClosure {
    typedef void (*TrampolineFunc)(void *, T);
    void *mFn;
    void *mParam;

public:
    CxxClosure() : mFn(), mParam()
    {}
    CxxClosure(void *f, void *param) : mFn(f), mParam(param)
    {}

    void operator ()(T arg) const {
        if (mParam) { // if set
            ((TrampolineFunc)mFn)(mParam, arg);
        } else {
            // TODO: provide debug info
        }
    }
};

class CxxClosureVoid {
    CxxClosure<void*> mTyped;

public:
    CxxClosureVoid(void *f, void *param) : mTyped(f, param)
    {}

    void operator ()() const {
        mTyped(/*unused*/0);
    }
};

extern "C" {

// wxApp
void AppSetOnInit(void *aFn, void *aParam);
class App : public wxApp {
    virtual bool OnInit();
};

int wxApp_argc();
wxString *wxApp_argv(int i);

void wxObject_delete(wxObject *self);

void wxEvtHandler_Bind(wxEvtHandler *evtHandler, int eventType, void *aFn, void *aParam);
void wxEvtHandler_CallAfter(wxEvtHandler *evtHandler, void *aFn, void *aParam);

// String
struct UTF8Data {
    const char *data;
    size_t length;
};
wxString *wxString_new(const unsigned char *psz, const size_t nLength);
void wxString_delete(wxString *self);
UTF8Data wxString_UTF8Data(wxString *self);

// (wx)String::const_iterator
wxString::const_iterator *wxStringConstIterator_new();
void wxStringConstIterator_delete(wxString::const_iterator *self);
size_t wxStringConstIterator_IndexIn(wxString::const_iterator *self, const wxString *s);

// ArrayInt
wxArrayInt *wxArrayInt_new();
void wxArrayInt_delete(wxArrayInt *self);
void wxArrayInt_Add(wxArrayInt *self, int i);
int wxArrayInt_Item(wxArrayInt *self, size_t index);

// ArrayString
wxArrayString *wxArrayString_new();
void wxArrayString_delete(wxArrayString *self);
void wxArrayString_Add(wxArrayString *self, const wxString *s);

// DateTime
bool wxDateTime_ParseDate(wxDateTime * self, const wxString * date, wxString::const_iterator * end);

// WeakRef
void *OpaqueWeakRef_new(void *obj);
void *OpaqueWeakRef_copy(void *obj);
void OpaqueWeakRef_delete(void *self);
void *OpaqueWeakRef_Get(void *self);

#ifdef __WXMSW__
typedef wxChar ArgChar;
#else
typedef char ArgChar;
#endif // __WXMSW__
int wxRustEntry(int *argc, ArgChar **argv);

} // extern "C"
