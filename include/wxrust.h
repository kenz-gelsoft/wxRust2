#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"


// wxApp
void WxRustAppSetOnInit(rust::Fn<void()> aOnInit);
class WxRustApp : public wxApp {
    virtual bool OnInit();
};

// wxFrame
wxFrame *wxFrame_new(rust::Str aTitle);
