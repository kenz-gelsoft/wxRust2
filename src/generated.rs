#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

#[cxx::bridge(namespace = "wxrust")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("wx/include/wxrust.h");
        include!("wx/include/wxrust2.h");

        type wxPoint;
        type wxSize;
        type wxString;
        type wxValidator;
        type wxWindow;
        // CLASS: wxButton
        type wxButton;
        // CTOR: fn wxButton(self: Pin<&mut wxButton>);
        // CTOR: unsafe fn wxButton(self: Pin<&mut wxButton>, parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString);
        // CXX_UNSUPPORTED: unsafe fn Create(self: Pin<&mut wxButton>, parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> bool;
        // BLOCKED: fn GetAuthNeeded(self: Pin<&mut wxButton>) -> bool;
        // CXX_UNSUPPORTED: fn GetLabel(self: Pin<&mut wxButton>) -> wxString;
        fn SetAuthNeeded(self: Pin<&mut wxButton>, needed: bool);
        unsafe fn SetDefault(self: Pin<&mut wxButton>) -> *mut wxWindow;
        fn SetLabel(self: Pin<&mut wxButton>, label: &wxString);
        // CXX_UNSUPPORTED: unsafe fn GetDefaultSize(self: Pin<&mut wxButton>, win: *mut wxWindow) -> wxSize;
    }
}

