#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

use std::mem;
use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};
use std::ptr;

use super::*;
use methods::*;

use wx_base::methods::*;
use wx_base::*;

mod ffi;
pub mod methods;

// wxAnyButton
wx_class! { AnyButton =
    AnyButtonIsOwned<true>(wxAnyButton) impl
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> AnyButtonIsOwned<OWNED> {
    pub fn new() -> AnyButtonIsOwned<OWNED> {
        unsafe { AnyButtonIsOwned(ffi::wxAnyButton_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxArtProvider
wx_class! { ArtProvider =
    ArtProviderIsOwned<true>(wxArtProvider) impl
        ArtProviderMethods,
        ObjectMethods
}
impl<const OWNED: bool> ArtProviderIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ArtProviderIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBitmap
wx_class! { Bitmap =
    BitmapIsOwned<true>(wxBitmap) impl
        BitmapMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> BitmapIsOwned<OWNED> {
    pub fn new() -> BitmapIsOwned<OWNED> {
        unsafe { BitmapIsOwned(ffi::wxBitmap_new()) }
    }
    pub fn new_with_bitmap<B: BitmapMethods>(bitmap: &B) -> BitmapIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            BitmapIsOwned(ffi::wxBitmap_new1(bitmap))
        }
    }
    // NOT_SUPPORTED: fn wxBitmap2()
    pub fn new_with_int_int(width: c_int, height: c_int, depth: c_int) -> BitmapIsOwned<OWNED> {
        unsafe { BitmapIsOwned(ffi::wxBitmap_new3(width, height, depth)) }
    }
    pub fn new_with_size<S: SizeMethods>(sz: &S, depth: c_int) -> BitmapIsOwned<OWNED> {
        unsafe {
            let sz = sz.as_ptr();
            BitmapIsOwned(ffi::wxBitmap_new4(sz, depth))
        }
    }
    pub fn new_with_int_dc(width: c_int, height: c_int, dc: *const c_void) -> BitmapIsOwned<OWNED> {
        unsafe { BitmapIsOwned(ffi::wxBitmap_new5(width, height, dc)) }
    }
    pub fn new_with_char(bits: *const c_void) -> BitmapIsOwned<OWNED> {
        unsafe { BitmapIsOwned(ffi::wxBitmap_new6(bits)) }
    }
    // NOT_SUPPORTED: fn wxBitmap7()
    pub fn new_with_image_int(img: *const c_void, depth: c_int) -> BitmapIsOwned<OWNED> {
        unsafe { BitmapIsOwned(ffi::wxBitmap_new8(img, depth)) }
    }
    pub fn new_with_image_dc(img: *const c_void, dc: *const c_void) -> BitmapIsOwned<OWNED> {
        unsafe { BitmapIsOwned(ffi::wxBitmap_new9(img, dc)) }
    }
    pub fn new_with_cursor(cursor: *const c_void) -> BitmapIsOwned<OWNED> {
        unsafe { BitmapIsOwned(ffi::wxBitmap_new10(cursor)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for BitmapIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBitmapBundle
wx_class! { BitmapBundle =
    BitmapBundleIsOwned<true>(wxBitmapBundle) impl
        BitmapBundleMethods
}
impl<const OWNED: bool> BitmapBundleIsOwned<OWNED> {
    pub fn new() -> BitmapBundleIsOwned<OWNED> {
        unsafe { BitmapBundleIsOwned(ffi::wxBitmapBundle_new()) }
    }
    pub fn new_with_bitmap<B: BitmapMethods>(bitmap: &B) -> BitmapBundleIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            BitmapBundleIsOwned(ffi::wxBitmapBundle_new1(bitmap))
        }
    }
    pub fn new_with_icon<I: IconMethods>(icon: &I) -> BitmapBundleIsOwned<OWNED> {
        unsafe {
            let icon = icon.as_ptr();
            BitmapBundleIsOwned(ffi::wxBitmapBundle_new2(icon))
        }
    }
    pub fn new_with_image(image: *const c_void) -> BitmapBundleIsOwned<OWNED> {
        unsafe { BitmapBundleIsOwned(ffi::wxBitmapBundle_new3(image)) }
    }
    pub fn new_with_char(xpm: *const c_void) -> BitmapBundleIsOwned<OWNED> {
        unsafe { BitmapBundleIsOwned(ffi::wxBitmapBundle_new4(xpm)) }
    }
    pub fn new_with_bitmapbundle<B: BitmapBundleMethods>(other: &B) -> BitmapBundleIsOwned<OWNED> {
        unsafe {
            let other = other.as_ptr();
            BitmapBundleIsOwned(ffi::wxBitmapBundle_new5(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for BitmapBundleIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxBitmapBundle_delete(self.0) }
        }
    }
}

// wxBitmapButton
wx_class! { BitmapButton =
    BitmapButtonIsOwned<true>(wxBitmapButton) impl
        BitmapButtonMethods,
        ButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BitmapButtonIsOwned<OWNED> {
    pub fn new_2step() -> BitmapButtonIsOwned<OWNED> {
        unsafe { BitmapButtonIsOwned(ffi::wxBitmapButton_new()) }
    }
    pub fn new<
        W: WindowMethods,
        B: BitmapBundleMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        bitmap: &B,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> BitmapButtonIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let bitmap = bitmap.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            BitmapButtonIsOwned(ffi::wxBitmapButton_new1(
                parent, id, bitmap, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxBookCtrlBase
wx_class! { BookCtrlBase =
    BookCtrlBaseIsOwned<true>(wxBookCtrlBase) impl
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BookCtrlBaseIsOwned<OWNED> {
    //  ENUM: @3
    pub const NO_IMAGE: c_int = -1;

    // BLOCKED: fn wxBookCtrlBase()
    // BLOCKED: fn wxBookCtrlBase1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> WindowMethods for BookCtrlBaseIsOwned<OWNED> {
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        winid: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxBookCtrlBase_Create(self.as_ptr(), parent, winid, pos, size, style, name)
        }
    }
}

// wxBookCtrlEvent
wx_class! { BookCtrlEvent =
    BookCtrlEventIsOwned<true>(wxBookCtrlEvent) impl
        BookCtrlEventMethods,
        NotifyEventMethods,
        // CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> BookCtrlEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxBookCtrlEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for BookCtrlEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> CommandEventMethods for BookCtrlEventIsOwned<OWNED> {
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxBookCtrlEvent_GetSelection(self.as_ptr()) }
    }
}

// wxBoxSizer
wx_class! { BoxSizer =
    BoxSizerIsOwned<true>(wxBoxSizer) impl
        BoxSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BoxSizerIsOwned<OWNED> {
    pub fn new(orient: c_int) -> BoxSizerIsOwned<OWNED> {
        unsafe { BoxSizerIsOwned(ffi::wxBoxSizer_new(orient)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxButton
wx_class! { Button =
    ButtonIsOwned<true>(wxButton) impl
        ButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ButtonIsOwned<OWNED> {
    pub fn new_2step() -> ButtonIsOwned<OWNED> {
        unsafe { ButtonIsOwned(ffi::wxButton_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ButtonIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ButtonIsOwned(ffi::wxButton_new1(
                parent, id, label, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxCheckBox
wx_class! { CheckBox =
    CheckBoxIsOwned<true>(wxCheckBox) impl
        CheckBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CheckBoxIsOwned<OWNED> {
    pub fn new_2step() -> CheckBoxIsOwned<OWNED> {
        unsafe { CheckBoxIsOwned(ffi::wxCheckBox_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> CheckBoxIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            CheckBoxIsOwned(ffi::wxCheckBox_new1(
                parent, id, label, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxCheckListBox
wx_class! { CheckListBox =
    CheckListBoxIsOwned<true>(wxCheckListBox) impl
        CheckListBoxMethods,
        // ListBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods,
        ItemContainerMethods,
        ItemContainerImmutableMethods
}
impl<const OWNED: bool> CheckListBoxIsOwned<OWNED> {
    pub fn new_2step() -> CheckListBoxIsOwned<OWNED> {
        unsafe { CheckListBoxIsOwned(ffi::wxCheckListBox_new()) }
    }
    // NOT_SUPPORTED: fn wxCheckListBox1()
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> CheckListBoxIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            CheckListBoxIsOwned(ffi::wxCheckListBox_new2(
                parent, id, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
// Mix-in(s) to wxCheckListBox
impl<const OWNED: bool> AsRef<ItemContainer> for CheckListBoxIsOwned<OWNED> {
    fn as_ref(&self) -> &ItemContainer {
        unsafe {
            let p = &ffi::wxCheckListBox_AsItemContainer(self.as_ptr()) as *const *mut c_void
                as *mut ItemContainer;
            p.as_ref().unwrap()
        }
    }
}
impl<const OWNED: bool> AsRef<ItemContainerImmutable> for CheckListBoxIsOwned<OWNED> {
    fn as_ref(&self) -> &ItemContainerImmutable {
        unsafe {
            let p = &ffi::wxCheckListBox_AsItemContainer(self.as_ptr()) as *const *mut c_void
                as *mut ItemContainerImmutable;
            p.as_ref().unwrap()
        }
    }
}
impl<const OWNED: bool> ListBoxMethods for CheckListBoxIsOwned<OWNED> {
    // NOT_SUPPORTED: fn Create()
    fn create_arraystring<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxCheckListBox_Create1(
                self.as_ptr(),
                parent,
                id,
                pos,
                size,
                choices,
                style,
                validator,
                name,
            )
        }
    }
}

// wxChoice
wx_class! { Choice =
    ChoiceIsOwned<true>(wxChoice) impl
        ChoiceMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods,
        ItemContainerMethods,
        ItemContainerImmutableMethods
}
impl<const OWNED: bool> ChoiceIsOwned<OWNED> {
    pub fn new_2step() -> ChoiceIsOwned<OWNED> {
        unsafe { ChoiceIsOwned(ffi::wxChoice_new()) }
    }
    // NOT_SUPPORTED: fn wxChoice1()
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ChoiceIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ChoiceIsOwned(ffi::wxChoice_new2(
                parent, id, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
// Mix-in(s) to wxChoice
impl<const OWNED: bool> AsRef<ItemContainer> for ChoiceIsOwned<OWNED> {
    fn as_ref(&self) -> &ItemContainer {
        unsafe {
            let p = &ffi::wxChoice_AsItemContainer(self.as_ptr()) as *const *mut c_void
                as *mut ItemContainer;
            p.as_ref().unwrap()
        }
    }
}
impl<const OWNED: bool> AsRef<ItemContainerImmutable> for ChoiceIsOwned<OWNED> {
    fn as_ref(&self) -> &ItemContainerImmutable {
        unsafe {
            let p = &ffi::wxChoice_AsItemContainer(self.as_ptr()) as *const *mut c_void
                as *mut ItemContainerImmutable;
            p.as_ref().unwrap()
        }
    }
}

// wxColour
wx_class! { Colour =
    ColourIsOwned<true>(wxColour) impl
        ColourMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourIsOwned<OWNED> {
    pub fn new() -> ColourIsOwned<OWNED> {
        unsafe { ColourIsOwned(ffi::wxColour_new()) }
    }
    // NOT_SUPPORTED: fn wxColour1()
    pub fn new_with_str(colour_name: &str) -> ColourIsOwned<OWNED> {
        unsafe {
            let colour_name = WxString::from(colour_name);
            let colour_name = colour_name.as_ptr();
            ColourIsOwned(ffi::wxColour_new2(colour_name))
        }
    }
    // NOT_SUPPORTED: fn wxColour3()
    pub fn new_with_colour<C: ColourMethods>(colour: &C) -> ColourIsOwned<OWNED> {
        unsafe {
            let colour = colour.as_ptr();
            ColourIsOwned(ffi::wxColour_new4(colour))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ColourIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxColourPickerCtrl
wx_class! { ColourPickerCtrl =
    ColourPickerCtrlIsOwned<true>(wxColourPickerCtrl) impl
        ColourPickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourPickerCtrlIsOwned<OWNED> {
    pub fn new_2step() -> ColourPickerCtrlIsOwned<OWNED> {
        unsafe { ColourPickerCtrlIsOwned(ffi::wxColourPickerCtrl_new()) }
    }
    pub fn new<
        W: WindowMethods,
        C: ColourMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        colour: &C,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ColourPickerCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let colour = colour.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ColourPickerCtrlIsOwned(ffi::wxColourPickerCtrl_new1(
                parent, id, colour, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxComboBox
wx_class! { ComboBox =
    ComboBoxIsOwned<true>(wxComboBox) impl
        ComboBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods,
        ItemContainerMethods,
        ItemContainerImmutableMethods,
        TextEntryMethods
}
impl<const OWNED: bool> ComboBoxIsOwned<OWNED> {
    pub fn new_2step() -> ComboBoxIsOwned<OWNED> {
        unsafe { ComboBoxIsOwned(ffi::wxComboBox_new()) }
    }
    // NOT_SUPPORTED: fn wxComboBox1()
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ComboBoxIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ComboBoxIsOwned(ffi::wxComboBox_new2(
                parent, id, value, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
// Mix-in(s) to wxComboBox
impl<const OWNED: bool> AsRef<ItemContainer> for ComboBoxIsOwned<OWNED> {
    fn as_ref(&self) -> &ItemContainer {
        unsafe {
            let p = &ffi::wxComboBox_AsItemContainer(self.as_ptr()) as *const *mut c_void
                as *mut ItemContainer;
            p.as_ref().unwrap()
        }
    }
}
impl<const OWNED: bool> AsRef<ItemContainerImmutable> for ComboBoxIsOwned<OWNED> {
    fn as_ref(&self) -> &ItemContainerImmutable {
        unsafe {
            let p = &ffi::wxComboBox_AsItemContainer(self.as_ptr()) as *const *mut c_void
                as *mut ItemContainerImmutable;
            p.as_ref().unwrap()
        }
    }
}
impl<const OWNED: bool> AsRef<TextEntry> for ComboBoxIsOwned<OWNED> {
    fn as_ref(&self) -> &TextEntry {
        unsafe {
            let p =
                &ffi::wxComboBox_AsTextEntry(self.as_ptr()) as *const *mut c_void as *mut TextEntry;
            p.as_ref().unwrap()
        }
    }
}

// wxCommandEvent
wx_class! { CommandEvent =
    CommandEventIsOwned<true>(wxCommandEvent) impl
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> CommandEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxCommandEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for CommandEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxControl
wx_class! { Control =
    ControlIsOwned<true>(wxControl) impl
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ControlIsOwned<OWNED> {
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ControlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ControlIsOwned(ffi::wxControl_new(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    pub fn new_2step() -> ControlIsOwned<OWNED> {
        unsafe { ControlIsOwned(ffi::wxControl_new1()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxDatePickerCtrl
wx_class! { DatePickerCtrl =
    DatePickerCtrlIsOwned<true>(wxDatePickerCtrl) impl
        DatePickerCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DatePickerCtrlIsOwned<OWNED> {
    pub fn new_2step() -> DatePickerCtrlIsOwned<OWNED> {
        unsafe { DatePickerCtrlIsOwned(ffi::wxDatePickerCtrl_new()) }
    }
    pub fn new<
        W: WindowMethods,
        D: DateTimeMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        dt: &D,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> DatePickerCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt = dt.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            DatePickerCtrlIsOwned(ffi::wxDatePickerCtrl_new1(
                parent, id, dt, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxDirPickerCtrl
wx_class! { DirPickerCtrl =
    DirPickerCtrlIsOwned<true>(wxDirPickerCtrl) impl
        DirPickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DirPickerCtrlIsOwned<OWNED> {
    pub fn new_2step() -> DirPickerCtrlIsOwned<OWNED> {
        unsafe { DirPickerCtrlIsOwned(ffi::wxDirPickerCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        path: &str,
        message: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> DirPickerCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let path = WxString::from(path);
            let path = path.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            DirPickerCtrlIsOwned(ffi::wxDirPickerCtrl_new1(
                parent, id, path, message, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxEditableListBox
wx_class! { EditableListBox =
    EditableListBoxIsOwned<true>(wxEditableListBox) impl
        EditableListBoxMethods,
        PanelMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> EditableListBoxIsOwned<OWNED> {
    pub fn new_2step() -> EditableListBoxIsOwned<OWNED> {
        unsafe { EditableListBoxIsOwned(ffi::wxEditableListBox_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> EditableListBoxIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            EditableListBoxIsOwned(ffi::wxEditableListBox_new1(
                parent, id, label, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxFileCtrl
wx_class! { FileCtrl =
    FileCtrlIsOwned<true>(wxFileCtrl) impl
        FileCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FileCtrlIsOwned<OWNED> {
    pub fn new_2step() -> FileCtrlIsOwned<OWNED> {
        unsafe { FileCtrlIsOwned(ffi::wxFileCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        default_directory: &str,
        default_filename: &str,
        wild_card: &str,
        style: c_long,
        pos: &P,
        size: &S,
        name: &str,
    ) -> FileCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let default_directory = WxString::from(default_directory);
            let default_directory = default_directory.as_ptr();
            let default_filename = WxString::from(default_filename);
            let default_filename = default_filename.as_ptr();
            let wild_card = WxString::from(wild_card);
            let wild_card = wild_card.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            FileCtrlIsOwned(ffi::wxFileCtrl_new1(
                parent,
                id,
                default_directory,
                default_filename,
                wild_card,
                style,
                pos,
                size,
                name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxFont
wx_class! { Font =
    FontIsOwned<true>(wxFont) impl
        FontMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> FontIsOwned<OWNED> {
    pub fn new() -> FontIsOwned<OWNED> {
        unsafe { FontIsOwned(ffi::wxFont_new()) }
    }
    pub fn new_with_font<F: FontMethods>(font: &F) -> FontIsOwned<OWNED> {
        unsafe {
            let font = font.as_ptr();
            FontIsOwned(ffi::wxFont_new1(font))
        }
    }
    pub fn new_with_fontinfo(font_info: *const c_void) -> FontIsOwned<OWNED> {
        unsafe { FontIsOwned(ffi::wxFont_new2(font_info)) }
    }
    // NOT_SUPPORTED: fn wxFont3()
    // NOT_SUPPORTED: fn wxFont4()
    pub fn new_with_str(native_info_string: &str) -> FontIsOwned<OWNED> {
        unsafe {
            let native_info_string = WxString::from(native_info_string);
            let native_info_string = native_info_string.as_ptr();
            FontIsOwned(ffi::wxFont_new5(native_info_string))
        }
    }
    pub fn new_with_nativefontinfo(native_info: *const c_void) -> FontIsOwned<OWNED> {
        unsafe { FontIsOwned(ffi::wxFont_new6(native_info)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for FontIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFontPickerCtrl
wx_class! { FontPickerCtrl =
    FontPickerCtrlIsOwned<true>(wxFontPickerCtrl) impl
        FontPickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FontPickerCtrlIsOwned<OWNED> {
    pub fn new_2step() -> FontPickerCtrlIsOwned<OWNED> {
        unsafe { FontPickerCtrlIsOwned(ffi::wxFontPickerCtrl_new()) }
    }
    pub fn new<
        W: WindowMethods,
        F: FontMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        font: &F,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> FontPickerCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let font = font.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            FontPickerCtrlIsOwned(ffi::wxFontPickerCtrl_new1(
                parent, id, font, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxFrame
wx_class! { Frame =
    FrameIsOwned<true>(wxFrame) impl
        FrameMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FrameIsOwned<OWNED> {
    pub fn new_2step() -> FrameIsOwned<OWNED> {
        unsafe { FrameIsOwned(ffi::wxFrame_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> FrameIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            FrameIsOwned(ffi::wxFrame_new1(parent, id, title, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> TopLevelWindowMethods for FrameIsOwned<OWNED> {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxFrame_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
}
impl<const OWNED: bool> WindowMethods for FrameIsOwned<OWNED> {
    fn centre(&self, direction: c_int) {
        unsafe { ffi::wxFrame_Centre(self.as_ptr(), direction) }
    }
}

// wxGDIObject
wx_class! { GDIObject =
    GDIObjectIsOwned<true>(wxGDIObject) impl
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GDIObjectIsOwned<OWNED> {
    // BLOCKED: fn wxGDIObject()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for GDIObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGauge
wx_class! { Gauge =
    GaugeIsOwned<true>(wxGauge) impl
        GaugeMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GaugeIsOwned<OWNED> {
    pub fn new_2step() -> GaugeIsOwned<OWNED> {
        unsafe { GaugeIsOwned(ffi::wxGauge_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        range: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> GaugeIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            GaugeIsOwned(ffi::wxGauge_new1(
                parent, id, range, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxGenericDirCtrl
wx_class! { GenericDirCtrl =
    GenericDirCtrlIsOwned<true>(wxGenericDirCtrl) impl
        GenericDirCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GenericDirCtrlIsOwned<OWNED> {
    pub fn new_2step() -> GenericDirCtrlIsOwned<OWNED> {
        unsafe { GenericDirCtrlIsOwned(ffi::wxGenericDirCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        dir: &str,
        pos: &P,
        size: &S,
        style: c_long,
        filter: &str,
        default_filter: c_int,
        name: &str,
    ) -> GenericDirCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let filter = WxString::from(filter);
            let filter = filter.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            GenericDirCtrlIsOwned(ffi::wxGenericDirCtrl_new1(
                parent,
                id,
                dir,
                pos,
                size,
                style,
                filter,
                default_filter,
                name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxHeaderColumn
wx_class! { HeaderColumn =
    HeaderColumnIsOwned<true>(wxHeaderColumn) impl
        HeaderColumnMethods
}
impl<const OWNED: bool> HeaderColumnIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for HeaderColumnIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxHeaderColumn_delete(self.0) }
        }
    }
}

// wxHeaderColumnSimple
wx_class! { HeaderColumnSimple =
    HeaderColumnSimpleIsOwned<true>(wxHeaderColumnSimple) impl
        HeaderColumnSimpleMethods,
        SettableHeaderColumnMethods,
        HeaderColumnMethods
}
impl<const OWNED: bool> HeaderColumnSimpleIsOwned<OWNED> {
    pub fn new_with_str(
        title: &str,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> HeaderColumnSimpleIsOwned<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            HeaderColumnSimpleIsOwned(ffi::wxHeaderColumnSimple_new(title, width, align, flags))
        }
    }
    pub fn new_with_bitmapbundle<B: BitmapBundleMethods>(
        bitmap: &B,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> HeaderColumnSimpleIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            HeaderColumnSimpleIsOwned(ffi::wxHeaderColumnSimple_new1(bitmap, width, align, flags))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for HeaderColumnSimpleIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxHeaderColumnSimple_delete(self.0) }
        }
    }
}

// wxHeaderCtrl
wx_class! { HeaderCtrl =
    HeaderCtrlIsOwned<true>(wxHeaderCtrl) impl
        HeaderCtrlMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> HeaderCtrlIsOwned<OWNED> {
    // BLOCKED: fn wxHeaderCtrl()
    // BLOCKED: fn wxHeaderCtrl1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> WindowMethods for HeaderCtrlIsOwned<OWNED> {
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        winid: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxHeaderCtrl_Create(self.as_ptr(), parent, winid, pos, size, style, name)
        }
    }
}

// wxHeaderCtrlSimple
wx_class! { HeaderCtrlSimple =
    HeaderCtrlSimpleIsOwned<true>(wxHeaderCtrlSimple) impl
        HeaderCtrlSimpleMethods,
        HeaderCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> HeaderCtrlSimpleIsOwned<OWNED> {
    pub fn new_2step() -> HeaderCtrlSimpleIsOwned<OWNED> {
        unsafe { HeaderCtrlSimpleIsOwned(ffi::wxHeaderCtrlSimple_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        winid: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> HeaderCtrlSimpleIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            HeaderCtrlSimpleIsOwned(ffi::wxHeaderCtrlSimple_new1(
                parent, winid, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxHyperlinkCtrl
wx_class! { HyperlinkCtrl =
    HyperlinkCtrlIsOwned<true>(wxHyperlinkCtrl) impl
        HyperlinkCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> HyperlinkCtrlIsOwned<OWNED> {
    pub fn new_2step() -> HyperlinkCtrlIsOwned<OWNED> {
        unsafe { HyperlinkCtrlIsOwned(ffi::wxHyperlinkCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        url: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> HyperlinkCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let url = WxString::from(url);
            let url = url.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            HyperlinkCtrlIsOwned(ffi::wxHyperlinkCtrl_new1(
                parent, id, label, url, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxIcon
wx_class! { Icon =
    IconIsOwned<true>(wxIcon) impl
        IconMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> IconIsOwned<OWNED> {
    pub fn new() -> IconIsOwned<OWNED> {
        unsafe { IconIsOwned(ffi::wxIcon_new()) }
    }
    pub fn new_with_icon<I: IconMethods>(icon: &I) -> IconIsOwned<OWNED> {
        unsafe {
            let icon = icon.as_ptr();
            IconIsOwned(ffi::wxIcon_new1(icon))
        }
    }
    // NOT_SUPPORTED: fn wxIcon2()
    pub fn new_with_char(bits: *const c_void) -> IconIsOwned<OWNED> {
        unsafe { IconIsOwned(ffi::wxIcon_new3(bits)) }
    }
    // NOT_SUPPORTED: fn wxIcon4()
    pub fn new_with_iconlocation(loc: *const c_void) -> IconIsOwned<OWNED> {
        unsafe { IconIsOwned(ffi::wxIcon_new5(loc)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for IconIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxItemContainer
wx_class! { ItemContainer =
    ItemContainerIsOwned<true>(wxItemContainer) impl
        ItemContainerMethods,
        ItemContainerImmutableMethods
}
impl<const OWNED: bool> AsRef<ItemContainer> for ItemContainerIsOwned<OWNED> {
    fn as_ref(&self) -> &ItemContainer {
        unsafe {
            let p = self.as_ptr() as *mut ItemContainer;
            p.as_ref().unwrap()
        }
    }
}
impl<const OWNED: bool> AsRef<ItemContainerImmutable> for ItemContainerIsOwned<OWNED> {
    fn as_ref(&self) -> &ItemContainerImmutable {
        unsafe {
            let p = self.as_ptr() as *mut ItemContainerImmutable;
            p.as_ref().unwrap()
        }
    }
}
impl<const OWNED: bool> ItemContainerIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ItemContainerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxItemContainer_delete(self.0) }
        }
    }
}

// wxItemContainerImmutable
wx_class! { ItemContainerImmutable =
    ItemContainerImmutableIsOwned<true>(wxItemContainerImmutable) impl
        ItemContainerImmutableMethods
}
impl<const OWNED: bool> AsRef<ItemContainerImmutable> for ItemContainerImmutableIsOwned<OWNED> {
    fn as_ref(&self) -> &ItemContainerImmutable {
        unsafe {
            let p = self.as_ptr() as *mut ItemContainerImmutable;
            p.as_ref().unwrap()
        }
    }
}
impl<const OWNED: bool> ItemContainerImmutableIsOwned<OWNED> {
    // BLOCKED: fn wxItemContainerImmutable()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ItemContainerImmutableIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxItemContainerImmutable_delete(self.0) }
        }
    }
}

// wxListBox
wx_class! { ListBox =
    ListBoxIsOwned<true>(wxListBox) impl
        ListBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods,
        ItemContainerMethods,
        ItemContainerImmutableMethods
}
impl<const OWNED: bool> ListBoxIsOwned<OWNED> {
    pub fn new_2step() -> ListBoxIsOwned<OWNED> {
        unsafe { ListBoxIsOwned(ffi::wxListBox_new()) }
    }
    // NOT_SUPPORTED: fn wxListBox1()
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ListBoxIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ListBoxIsOwned(ffi::wxListBox_new2(
                parent, id, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
// Mix-in(s) to wxListBox
impl<const OWNED: bool> AsRef<ItemContainer> for ListBoxIsOwned<OWNED> {
    fn as_ref(&self) -> &ItemContainer {
        unsafe {
            let p = &ffi::wxListBox_AsItemContainer(self.as_ptr()) as *const *mut c_void
                as *mut ItemContainer;
            p.as_ref().unwrap()
        }
    }
}
impl<const OWNED: bool> AsRef<ItemContainerImmutable> for ListBoxIsOwned<OWNED> {
    fn as_ref(&self) -> &ItemContainerImmutable {
        unsafe {
            let p = &ffi::wxListBox_AsItemContainer(self.as_ptr()) as *const *mut c_void
                as *mut ItemContainerImmutable;
            p.as_ref().unwrap()
        }
    }
}

// wxMenu
wx_class! { Menu =
    MenuIsOwned<true>(wxMenu) impl
        MenuMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuIsOwned<OWNED> {
    pub fn new() -> MenuIsOwned<OWNED> {
        unsafe { MenuIsOwned(ffi::wxMenu_new()) }
    }
    pub fn new_with_long(style: c_long) -> MenuIsOwned<OWNED> {
        unsafe { MenuIsOwned(ffi::wxMenu_new1(style)) }
    }
    pub fn new_with_str(title: &str, style: c_long) -> MenuIsOwned<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            MenuIsOwned(ffi::wxMenu_new2(title, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxMenuBar
wx_class! { MenuBar =
    MenuBarIsOwned<true>(wxMenuBar) impl
        MenuBarMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuBarIsOwned<OWNED> {
    pub fn new(style: c_long) -> MenuBarIsOwned<OWNED> {
        unsafe { MenuBarIsOwned(ffi::wxMenuBar_new(style)) }
    }
    // NOT_SUPPORTED: fn wxMenuBar1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxMenuItem
wx_class! { MenuItem =
    MenuItemIsOwned<true>(wxMenuItem) impl
        MenuItemMethods,
        ObjectMethods
}
impl<const OWNED: bool> MenuItemIsOwned<OWNED> {
    pub fn new<M: MenuMethods, M2: MenuMethods>(
        parent_menu: Option<&M>,
        id: c_int,
        text: &str,
        help_string: &str,
        kind: c_int,
        sub_menu: Option<&M2>,
    ) -> MenuItemIsOwned<OWNED> {
        unsafe {
            let parent_menu = match parent_menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let text = WxString::from(text);
            let text = text.as_ptr();
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            let sub_menu = match sub_menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItemIsOwned(ffi::wxMenuItem_new(
                parent_menu,
                id,
                text,
                help_string,
                kind,
                sub_menu,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for MenuItemIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxNonOwnedWindow
wx_class! { NonOwnedWindow =
    NonOwnedWindowIsOwned<true>(wxNonOwnedWindow) impl
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> NonOwnedWindowIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxNotebook
wx_class! { Notebook =
    NotebookIsOwned<true>(wxNotebook) impl
        NotebookMethods,
        // BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> NotebookIsOwned<OWNED> {
    pub fn new_2step() -> NotebookIsOwned<OWNED> {
        unsafe { NotebookIsOwned(ffi::wxNotebook_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> NotebookIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            NotebookIsOwned(ffi::wxNotebook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> BookCtrlBaseMethods for NotebookIsOwned<OWNED> {
    // BLOCKED: fn Create()
}
impl<const OWNED: bool> WindowMethods for NotebookIsOwned<OWNED> {
    // BLOCKED: fn Create()
}

// wxNotifyEvent
wx_class! { NotifyEvent =
    NotifyEventIsOwned<true>(wxNotifyEvent) impl
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> NotifyEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxNotifyEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for NotifyEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPanel
wx_class! { Panel =
    PanelIsOwned<true>(wxPanel) impl
        PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PanelIsOwned<OWNED> {
    pub fn new_2step() -> PanelIsOwned<OWNED> {
        unsafe { PanelIsOwned(ffi::wxPanel_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> PanelIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            PanelIsOwned(ffi::wxPanel_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> WindowMethods for PanelIsOwned<OWNED> {
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxPanel_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxPickerBase
wx_class! { PickerBase =
    PickerBaseIsOwned<true>(wxPickerBase) impl
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> PickerBaseIsOwned<OWNED> {
    // BLOCKED: fn wxPickerBase()
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxPoint
wx_class! { Point =
    PointIsOwned<true>(wxPoint) impl
        PointMethods
}
impl<const OWNED: bool> PointIsOwned<OWNED> {
    pub fn new() -> PointIsOwned<OWNED> {
        unsafe { PointIsOwned(ffi::wxPoint_new()) }
    }
    pub fn new_with_int(x: c_int, y: c_int) -> PointIsOwned<OWNED> {
        unsafe { PointIsOwned(ffi::wxPoint_new1(x, y)) }
    }
    pub fn new_with_realpoint(pt: *const c_void) -> PointIsOwned<OWNED> {
        unsafe { PointIsOwned(ffi::wxPoint_new2(pt)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for PointIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxPoint_delete(self.0) }
        }
    }
}

// wxRadioBox
wx_class! { RadioBox =
    RadioBoxIsOwned<true>(wxRadioBox) impl
        RadioBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods,
        ItemContainerImmutableMethods
}
impl<const OWNED: bool> RadioBoxIsOwned<OWNED> {
    pub fn new_2step() -> RadioBoxIsOwned<OWNED> {
        unsafe { RadioBoxIsOwned(ffi::wxRadioBox_new()) }
    }
    // NOT_SUPPORTED: fn wxRadioBox1()
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        choices: &A,
        major_dimension: c_int,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> RadioBoxIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            RadioBoxIsOwned(ffi::wxRadioBox_new2(
                parent,
                id,
                label,
                pos,
                size,
                choices,
                major_dimension,
                style,
                validator,
                name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
// Mix-in(s) to wxRadioBox
impl<const OWNED: bool> AsRef<ItemContainerImmutable> for RadioBoxIsOwned<OWNED> {
    fn as_ref(&self) -> &ItemContainerImmutable {
        unsafe {
            let p = &ffi::wxRadioBox_AsItemContainerImmutable(self.as_ptr()) as *const *mut c_void
                as *mut ItemContainerImmutable;
            p.as_ref().unwrap()
        }
    }
}

// wxRect
wx_class! { Rect =
    RectIsOwned<true>(wxRect) impl
        RectMethods
}
impl<const OWNED: bool> RectIsOwned<OWNED> {
    pub fn new() -> RectIsOwned<OWNED> {
        unsafe { RectIsOwned(ffi::wxRect_new()) }
    }
    pub fn new_with_int(x: c_int, y: c_int, width: c_int, height: c_int) -> RectIsOwned<OWNED> {
        unsafe { RectIsOwned(ffi::wxRect_new1(x, y, width, height)) }
    }
    pub fn new_with_point_point<P: PointMethods, P2: PointMethods>(
        top_left: &P,
        bottom_right: &P2,
    ) -> RectIsOwned<OWNED> {
        unsafe {
            let top_left = top_left.as_ptr();
            let bottom_right = bottom_right.as_ptr();
            RectIsOwned(ffi::wxRect_new2(top_left, bottom_right))
        }
    }
    pub fn new_with_point_size<P: PointMethods, S: SizeMethods>(
        pos: &P,
        size: &S,
    ) -> RectIsOwned<OWNED> {
        unsafe {
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            RectIsOwned(ffi::wxRect_new3(pos, size))
        }
    }
    pub fn new_with_size<S: SizeMethods>(size: &S) -> RectIsOwned<OWNED> {
        unsafe {
            let size = size.as_ptr();
            RectIsOwned(ffi::wxRect_new4(size))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for RectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRect_delete(self.0) }
        }
    }
}

// wxSettableHeaderColumn
wx_class! { SettableHeaderColumn =
    SettableHeaderColumnIsOwned<true>(wxSettableHeaderColumn) impl
        SettableHeaderColumnMethods,
        HeaderColumnMethods
}
impl<const OWNED: bool> SettableHeaderColumnIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for SettableHeaderColumnIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSettableHeaderColumn_delete(self.0) }
        }
    }
}

// wxSize
wx_class! { Size =
    SizeIsOwned<true>(wxSize) impl
        SizeMethods
}
impl<const OWNED: bool> SizeIsOwned<OWNED> {
    pub fn new() -> SizeIsOwned<OWNED> {
        unsafe { SizeIsOwned(ffi::wxSize_new()) }
    }
    pub fn new_with_int(width: c_int, height: c_int) -> SizeIsOwned<OWNED> {
        unsafe { SizeIsOwned(ffi::wxSize_new1(width, height)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for SizeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSize_delete(self.0) }
        }
    }
}

// wxSizer
wx_class! { Sizer =
    SizerIsOwned<true>(wxSizer) impl
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SizerIsOwned<OWNED> {
    // BLOCKED: fn wxSizer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxSizerFlags
wx_class! { SizerFlags =
    SizerFlagsIsOwned<true>(wxSizerFlags) impl
        SizerFlagsMethods
}
impl<const OWNED: bool> SizerFlagsIsOwned<OWNED> {
    pub fn new(proportion: c_int) -> SizerFlagsIsOwned<OWNED> {
        unsafe { SizerFlagsIsOwned(ffi::wxSizerFlags_new(proportion)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for SizerFlagsIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSizerFlags_delete(self.0) }
        }
    }
}

// wxStaticBitmap
wx_class! { StaticBitmap =
    StaticBitmapIsOwned<true>(wxStaticBitmap) impl
        StaticBitmapMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticBitmapIsOwned<OWNED> {
    //  ENUM: ScaleMode
    pub const Scale_None: c_int = 0;
    pub const Scale_Fill: c_int = 0 + 1;
    pub const Scale_AspectFit: c_int = 0 + 2;
    pub const Scale_AspectFill: c_int = 0 + 3;

    pub fn new_2step() -> StaticBitmapIsOwned<OWNED> {
        unsafe { StaticBitmapIsOwned(ffi::wxStaticBitmap_new()) }
    }
    pub fn new<W: WindowMethods, B: BitmapBundleMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &B,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> StaticBitmapIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            StaticBitmapIsOwned(ffi::wxStaticBitmap_new1(
                parent, id, label, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxStaticBox
wx_class! { StaticBox =
    StaticBoxIsOwned<true>(wxStaticBox) impl
        StaticBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticBoxIsOwned<OWNED> {
    pub fn new_2step() -> StaticBoxIsOwned<OWNED> {
        unsafe { StaticBoxIsOwned(ffi::wxStaticBox_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> StaticBoxIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            StaticBoxIsOwned(ffi::wxStaticBox_new1(
                parent, id, label, pos, size, style, name,
            ))
        }
    }
    // BLOCKED: fn wxStaticBox2()
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxStaticBoxSizer
wx_class! { StaticBoxSizer =
    StaticBoxSizerIsOwned<true>(wxStaticBoxSizer) impl
        StaticBoxSizerMethods,
        BoxSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticBoxSizerIsOwned<OWNED> {
    pub fn new_with_staticbox<S: StaticBoxMethods>(
        box_: Option<&S>,
        orient: c_int,
    ) -> StaticBoxSizerIsOwned<OWNED> {
        unsafe {
            let box_ = match box_ {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            StaticBoxSizerIsOwned(ffi::wxStaticBoxSizer_new(box_, orient))
        }
    }
    pub fn new_with_int<W: WindowMethods>(
        orient: c_int,
        parent: Option<&W>,
        label: &str,
    ) -> StaticBoxSizerIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            StaticBoxSizerIsOwned(ffi::wxStaticBoxSizer_new1(orient, parent, label))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxStaticText
wx_class! { StaticText =
    StaticTextIsOwned<true>(wxStaticText) impl
        StaticTextMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticTextIsOwned<OWNED> {
    pub fn new_2step() -> StaticTextIsOwned<OWNED> {
        unsafe { StaticTextIsOwned(ffi::wxStaticText_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> StaticTextIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            StaticTextIsOwned(ffi::wxStaticText_new1(
                parent, id, label, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxTextAttr
wx_class! { TextAttr =
    TextAttrIsOwned<true>(wxTextAttr) impl
        TextAttrMethods
}
impl<const OWNED: bool> TextAttrIsOwned<OWNED> {
    pub fn new() -> TextAttrIsOwned<OWNED> {
        unsafe { TextAttrIsOwned(ffi::wxTextAttr_new()) }
    }
    // NOT_SUPPORTED: fn wxTextAttr1()
    pub fn new_with_textattr<T: TextAttrMethods>(attr: &T) -> TextAttrIsOwned<OWNED> {
        unsafe {
            let attr = attr.as_ptr();
            TextAttrIsOwned(ffi::wxTextAttr_new2(attr))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for TextAttrIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTextAttr_delete(self.0) }
        }
    }
}

// wxTextCtrl
wx_class! { TextCtrl =
    TextCtrlIsOwned<true>(wxTextCtrl) impl
        TextCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods,
        TextEntryMethods
}
impl<const OWNED: bool> TextCtrlIsOwned<OWNED> {
    pub fn new_2step() -> TextCtrlIsOwned<OWNED> {
        unsafe { TextCtrlIsOwned(ffi::wxTextCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> TextCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            TextCtrlIsOwned(ffi::wxTextCtrl_new1(
                parent, id, value, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
// Mix-in(s) to wxTextCtrl
impl<const OWNED: bool> AsRef<TextEntry> for TextCtrlIsOwned<OWNED> {
    fn as_ref(&self) -> &TextEntry {
        unsafe {
            let p = &ffi::wxTextCtrl_AsTextEntry(self.as_ptr()) as *const *mut c_void
                as *const TextEntry;
            p.as_ref().unwrap()
        }
    }
}

// wxTextEntry
wx_class! { TextEntry =
    TextEntryIsOwned<true>(wxTextEntry) impl
        TextEntryMethods
}
impl<const OWNED: bool> AsRef<TextEntry> for TextEntryIsOwned<OWNED> {
    fn as_ref(&self) -> &TextEntry {
        unsafe { (self.as_ptr() as *const TextEntry).as_ref().unwrap() }
    }
}
impl<const OWNED: bool> TextEntryIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for TextEntryIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTextEntry_delete(self.0) }
        }
    }
}

// wxToolBar
wx_class! { ToolBar =
    ToolBarIsOwned<true>(wxToolBar) impl
        ToolBarMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ToolBarIsOwned<OWNED> {
    pub fn new_2step() -> ToolBarIsOwned<OWNED> {
        unsafe { ToolBarIsOwned(ffi::wxToolBar_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> ToolBarIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ToolBarIsOwned(ffi::wxToolBar_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxTopLevelWindow
wx_class! { TopLevelWindow =
    TopLevelWindowIsOwned<true>(wxTopLevelWindow) impl
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TopLevelWindowIsOwned<OWNED> {
    pub fn new_2step() -> TopLevelWindowIsOwned<OWNED> {
        unsafe { TopLevelWindowIsOwned(ffi::wxTopLevelWindow_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> TopLevelWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            TopLevelWindowIsOwned(ffi::wxTopLevelWindow_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxValidator
wx_class! { Validator =
    ValidatorIsOwned<true>(wxValidator) impl
        ValidatorMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ValidatorIsOwned<OWNED> {
    pub fn new() -> ValidatorIsOwned<OWNED> {
        unsafe { ValidatorIsOwned(ffi::wxValidator_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxWindow
wx_class! { Window =
    WindowIsOwned<true>(wxWindow) impl
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WindowIsOwned<OWNED> {
    pub fn new_2step() -> WindowIsOwned<OWNED> {
        unsafe { WindowIsOwned(ffi::wxWindow_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> WindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            WindowIsOwned(ffi::wxWindow_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxWrapSizer
wx_class! { WrapSizer =
    WrapSizerIsOwned<true>(wxWrapSizer) impl
        WrapSizerMethods,
        BoxSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WrapSizerIsOwned<OWNED> {
    pub fn new(orient: c_int, flags: c_int) -> WrapSizerIsOwned<OWNED> {
        unsafe { WrapSizerIsOwned(ffi::wxWrapSizer_new(orient, flags)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
