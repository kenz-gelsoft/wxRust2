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
		// fn wxButton(self: Pin<&mut wxButton>);
		// fn wxButton(self: Pin<&mut wxButton>, parent: Pin<&mut wxWindow>, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString);
		fn Create(self: Pin<&mut wxButton>, parent: Pin<&mut wxWindow>, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString);
		fn GetAuthNeeded(self: Pin<&mut wxButton>);
		fn GetLabel(self: Pin<&mut wxButton>);
		fn SetAuthNeeded(self: Pin<&mut wxButton>, needed: bool);
		fn SetDefault(self: Pin<&mut wxButton>);
		fn SetLabel(self: Pin<&mut wxButton>, label: &wxString);
		fn GetDefaultSize(self: Pin<&mut wxButton>, win: Pin<&mut wxWindow>);
	}
}

