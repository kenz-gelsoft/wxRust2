#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

#[cxx::bridge(namespace = "wxrust")]
mod ffi {
	#[namespace = ""]
	unsafe extern "C++" {
		include!("wx/include/wxrust.h");

		type wxPoint;
		type wxSize;
		type wxString;
		type wxValidator;
		type wxWindow;
		// CLASS: wxButton
		type wxButton;
		// unsafe fn wxButton(self: Pin<&mut wxButton>);
		// unsafe fn wxButton(self: Pin<&mut wxButton>, parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString);
		unsafe fn Create(self: Pin<&mut wxButton>, parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString);
		unsafe fn GetAuthNeeded(self: Pin<&mut wxButton>);
		unsafe fn GetLabel(self: Pin<&mut wxButton>);
		unsafe fn SetAuthNeeded(self: Pin<&mut wxButton>, needed: bool);
		unsafe fn SetDefault(self: Pin<&mut wxButton>);
		unsafe fn SetLabel(self: Pin<&mut wxButton>, label: &wxString);
		unsafe fn GetDefaultSize(self: Pin<&mut wxButton>, win: *mut wxWindow);
	}
}

