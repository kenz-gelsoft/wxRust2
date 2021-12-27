#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"


// wxApp
using unsafe_any_ptr = const char *;
void WxRustAppSetOnInit(
    rust::Fn<void(unsafe_any_ptr)> aFn,
    unsafe_any_ptr aParam
);
class WxRustApp : public wxApp {
    virtual bool OnInit();
};

// wxFrame
wxFrame *wxFrame_new(rust::Str aTitle);

// wxButton
wxButton *wxButton_new(wxWindow &parent, rust::Str label);
