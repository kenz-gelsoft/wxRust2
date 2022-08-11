use std::os::raw::{c_int, c_long, c_uint, c_void};

use super::*;
use crate::WeakRef;

pub use wx_base::methods::*;

// wxAnyButton
pub trait AnyButtonMethods: ControlMethods {
    // DTOR: fn ~wxAnyButton()
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmap(self.as_ptr())) }
    }
    fn get_bitmap_current(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapCurrent(self.as_ptr())) }
    }
    fn get_bitmap_disabled(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapDisabled(self.as_ptr())) }
    }
    fn get_bitmap_focus(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapFocus(self.as_ptr())) }
    }
    fn get_bitmap_label(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapLabel(self.as_ptr())) }
    }
    fn get_bitmap_pressed(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapPressed(self.as_ptr())) }
    }
    fn set_bitmap<B: BitmapBundleMethods>(&self, bitmap: &B, dir: c_int) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmap(self.as_ptr(), bitmap, dir)
        }
    }
    fn set_bitmap_current<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapCurrent(self.as_ptr(), bitmap)
        }
    }
    fn set_bitmap_disabled<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapDisabled(self.as_ptr(), bitmap)
        }
    }
    fn set_bitmap_focus<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapFocus(self.as_ptr(), bitmap)
        }
    }
    fn set_bitmap_label<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapLabel(self.as_ptr(), bitmap)
        }
    }
    fn set_bitmap_pressed<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapPressed(self.as_ptr(), bitmap)
        }
    }
    fn get_bitmap_margins(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxAnyButton_GetBitmapMargins(self.as_ptr())) }
    }
    fn set_bitmap_margins_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxAnyButton_SetBitmapMargins(self.as_ptr(), x, y) }
    }
    fn set_bitmap_margins_size<S: SizeMethods>(&self, sz: &S) {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxAnyButton_SetBitmapMargins1(self.as_ptr(), sz)
        }
    }
    fn set_bitmap_position(&self, dir: c_int) {
        unsafe { ffi::wxAnyButton_SetBitmapPosition(self.as_ptr(), dir) }
    }
}

// wxArtProvider
pub trait ArtProviderMethods: ObjectMethods {
    // DTOR: fn ~wxArtProvider()
    fn delete<A: ArtProviderMethods>(provider: Option<&A>) -> bool {
        unsafe {
            let provider = match provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxArtProvider_Delete(provider)
        }
    }
    fn get_bitmap<S: SizeMethods>(id: &str, client: &str, size: &S) -> Bitmap {
        unsafe {
            let id = WxString::from(id);
            let id = id.as_ptr();
            let client = WxString::from(client);
            let client = client.as_ptr();
            let size = size.as_ptr();
            Bitmap::from_ptr(ffi::wxArtProvider_GetBitmap(id, client, size))
        }
    }
    fn get_bitmap_bundle<S: SizeMethods>(id: &str, client: &str, size: &S) -> BitmapBundle {
        unsafe {
            let id = WxString::from(id);
            let id = id.as_ptr();
            let client = WxString::from(client);
            let client = client.as_ptr();
            let size = size.as_ptr();
            BitmapBundle::from_ptr(ffi::wxArtProvider_GetBitmapBundle(id, client, size))
        }
    }
    fn get_icon<S: SizeMethods>(id: &str, client: &str, size: &S) -> Icon {
        unsafe {
            let id = WxString::from(id);
            let id = id.as_ptr();
            let client = WxString::from(client);
            let client = client.as_ptr();
            let size = size.as_ptr();
            Icon::from_ptr(ffi::wxArtProvider_GetIcon(id, client, size))
        }
    }
    fn get_native_dip_size_hint(client: &str) -> Size {
        unsafe {
            let client = WxString::from(client);
            let client = client.as_ptr();
            Size::from_ptr(ffi::wxArtProvider_GetNativeDIPSizeHint(client))
        }
    }
    fn get_native_size_hint<W: WindowMethods>(client: &str, win: Option<&W>) -> Size {
        unsafe {
            let client = WxString::from(client);
            let client = client.as_ptr();
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxArtProvider_GetNativeSizeHint(client, win))
        }
    }
    fn get_dip_size_hint(client: &str) -> Size {
        unsafe {
            let client = WxString::from(client);
            let client = client.as_ptr();
            Size::from_ptr(ffi::wxArtProvider_GetDIPSizeHint(client))
        }
    }
    fn get_size_hint<W: WindowMethods>(client: &str, win: Option<&W>) -> Size {
        unsafe {
            let client = WxString::from(client);
            let client = client.as_ptr();
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxArtProvider_GetSizeHint(client, win))
        }
    }
    // NOT_SUPPORTED: fn GetIconBundle()
    fn has_native_provider() -> bool {
        unsafe { ffi::wxArtProvider_HasNativeProvider() }
    }
    // BLOCKED: fn Insert()
    fn pop() -> bool {
        unsafe { ffi::wxArtProvider_Pop() }
    }
    fn push<A: ArtProviderMethods>(provider: Option<&A>) {
        unsafe {
            let provider = match provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxArtProvider_Push(provider)
        }
    }
    fn push_back<A: ArtProviderMethods>(provider: Option<&A>) {
        unsafe {
            let provider = match provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxArtProvider_PushBack(provider)
        }
    }
    fn remove<A: ArtProviderMethods>(provider: Option<&A>) -> bool {
        unsafe {
            let provider = match provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxArtProvider_Remove(provider)
        }
    }
    fn get_message_box_icon_id(flags: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxArtProvider_GetMessageBoxIconId(flags)).into() }
    }
    fn get_message_box_icon(flags: c_int) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxArtProvider_GetMessageBoxIcon(flags)) }
    }
}
