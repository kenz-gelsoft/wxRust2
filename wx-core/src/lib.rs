use std::mem;
use std::os::raw::{c_int, c_long, c_void};
use std::ptr;

mod generated;
pub use generated::*;

// re-export wx_base
pub use wx_base::*;

#[doc(hidden)]
pub mod methods {
    use std::os::raw::c_int;

    pub use super::generated::methods::*;
    use super::*;

    // re-export wx_base::methods
    pub use wx_base::methods::*;

    pub trait Buildable<'a, P, B> {
        fn builder(parent: Option<&'a P>) -> B;
    }

    pub trait WindowListMethods: WxRustMethods {
        fn is_empty(&self) -> bool {
            unsafe { super::ffi::wxWindowList_IsEmpty(self.as_ptr()) }
        }
    }

    pub trait MenuItemBuilder {
        fn item<ID: Into<c_int>>(self, id: ID, s: &str) -> Self;
        fn item_h<ID: Into<c_int>>(self, id: ID, s: &str, h: &str) -> Self;
        fn check_item<ID: Into<c_int>>(self, id: ID, s: &str) -> Self;
        fn radio_item<ID: Into<c_int>>(self, id: ID, s: &str) -> Self;
        fn sub_menu<M: MenuMethods>(self, s: &str, submenu: &M) -> Self;
        fn separator(self) -> Self;
    }

    // wxTextEntry
    pub trait TextEntryMethods: WxRustMethods {
        fn ptr_from(obj: &Self) -> *mut c_void {
            unsafe { obj.as_ptr() }
        }
        fn append_text(&self, text: &str) {
            unsafe {
                let text = wx_base::wx_string_from(text);
                ffi::wxTextEntry_AppendText(Self::ptr_from(&self), text)
            }
        }
        fn auto_complete_arraystring<A: ArrayStringMethods>(&self, choices: &A) -> bool {
            unsafe {
                let choices = choices.as_ptr();
                ffi::wxTextEntry_AutoComplete(Self::ptr_from(&self), choices)
            }
        }
        fn auto_complete_textcompleter(&self, completer: *mut c_void) -> bool {
            unsafe { ffi::wxTextEntry_AutoComplete1(Self::ptr_from(&self), completer) }
        }
        fn auto_complete_file_names(&self) -> bool {
            unsafe { ffi::wxTextEntry_AutoCompleteFileNames(Self::ptr_from(&self)) }
        }
        fn auto_complete_directories(&self) -> bool {
            unsafe { ffi::wxTextEntry_AutoCompleteDirectories(Self::ptr_from(&self)) }
        }
        fn can_copy(&self) -> bool {
            unsafe { ffi::wxTextEntry_CanCopy(Self::ptr_from(&self)) }
        }
        fn can_cut(&self) -> bool {
            unsafe { ffi::wxTextEntry_CanCut(Self::ptr_from(&self)) }
        }
        fn can_paste(&self) -> bool {
            unsafe { ffi::wxTextEntry_CanPaste(Self::ptr_from(&self)) }
        }
        fn can_redo(&self) -> bool {
            unsafe { ffi::wxTextEntry_CanRedo(Self::ptr_from(&self)) }
        }
        fn can_undo(&self) -> bool {
            unsafe { ffi::wxTextEntry_CanUndo(Self::ptr_from(&self)) }
        }
        fn change_value(&self, value: &str) {
            unsafe {
                let value = wx_base::wx_string_from(value);
                ffi::wxTextEntry_ChangeValue(Self::ptr_from(&self), value)
            }
        }
        fn clear(&self) {
            unsafe { ffi::wxTextEntry_Clear(Self::ptr_from(&self)) }
        }
        fn copy(&self) {
            unsafe { ffi::wxTextEntry_Copy(Self::ptr_from(&self)) }
        }
        fn cut(&self) {
            unsafe { ffi::wxTextEntry_Cut(Self::ptr_from(&self)) }
        }
        fn force_upper(&self) {
            unsafe { ffi::wxTextEntry_ForceUpper(Self::ptr_from(&self)) }
        }
        fn get_insertion_point(&self) -> c_long {
            unsafe { ffi::wxTextEntry_GetInsertionPoint(Self::ptr_from(&self)) }
        }
        // NOT_SUPPORTED: fn GetLastPosition()
        fn get_range(&self, from: c_long, to: c_long) -> String {
            unsafe {
                wx_base::from_wx_string(ffi::wxTextEntry_GetRange(Self::ptr_from(&self), from, to))
            }
        }
        fn get_selection(&self, from: *mut c_void, to: *mut c_void) {
            unsafe { ffi::wxTextEntry_GetSelection(Self::ptr_from(&self), from, to) }
        }
        fn get_string_selection(&self) -> String {
            unsafe {
                wx_base::from_wx_string(ffi::wxTextEntry_GetStringSelection(Self::ptr_from(&self)))
            }
        }
        fn get_value(&self) -> String {
            unsafe { wx_base::from_wx_string(ffi::wxTextEntry_GetValue(Self::ptr_from(&self))) }
        }
        fn is_editable(&self) -> bool {
            unsafe { ffi::wxTextEntry_IsEditable(Self::ptr_from(&self)) }
        }
        fn is_empty(&self) -> bool {
            unsafe { ffi::wxTextEntry_IsEmpty(Self::ptr_from(&self)) }
        }
        fn paste(&self) {
            unsafe { ffi::wxTextEntry_Paste(Self::ptr_from(&self)) }
        }
        fn redo(&self) {
            unsafe { ffi::wxTextEntry_Redo(Self::ptr_from(&self)) }
        }
        fn remove(&self, from: c_long, to: c_long) {
            unsafe { ffi::wxTextEntry_Remove(Self::ptr_from(&self), from, to) }
        }
        fn replace(&self, from: c_long, to: c_long, value: &str) {
            unsafe {
                let value = wx_base::wx_string_from(value);
                ffi::wxTextEntry_Replace(Self::ptr_from(&self), from, to, value)
            }
        }
        fn set_editable(&self, editable: bool) {
            unsafe { ffi::wxTextEntry_SetEditable(Self::ptr_from(&self), editable) }
        }
        fn set_insertion_point(&self, pos: c_long) {
            unsafe { ffi::wxTextEntry_SetInsertionPoint(Self::ptr_from(&self), pos) }
        }
        fn set_insertion_point_end(&self) {
            unsafe { ffi::wxTextEntry_SetInsertionPointEnd(Self::ptr_from(&self)) }
        }
        // NOT_SUPPORTED: fn SetMaxLength()
        fn set_selection(&self, from: c_long, to: c_long) {
            unsafe { ffi::wxTextEntry_SetSelection(Self::ptr_from(&self), from, to) }
        }
        fn select_all(&self) {
            unsafe { ffi::wxTextEntry_SelectAll(Self::ptr_from(&self)) }
        }
        fn select_none(&self) {
            unsafe { ffi::wxTextEntry_SelectNone(Self::ptr_from(&self)) }
        }
        fn set_hint(&self, hint: &str) -> bool {
            unsafe {
                let hint = wx_base::wx_string_from(hint);
                ffi::wxTextEntry_SetHint(Self::ptr_from(&self), hint)
            }
        }
        fn get_hint(&self) -> String {
            unsafe { wx_base::from_wx_string(ffi::wxTextEntry_GetHint(Self::ptr_from(&self))) }
        }
        fn set_margins_point<P: PointMethods>(&self, pt: &P) -> bool {
            unsafe {
                let pt = pt.as_ptr();
                ffi::wxTextEntry_SetMargins(Self::ptr_from(&self), pt)
            }
        }
        fn set_margins_coord(&self, left: c_int, top: c_int) -> bool {
            unsafe { ffi::wxTextEntry_SetMargins1(Self::ptr_from(&self), left, top) }
        }
        fn get_margins(&self) -> Point {
            unsafe { Point::from_ptr(ffi::wxTextEntry_GetMargins(Self::ptr_from(&self))) }
        }
        fn set_value(&self, value: &str) {
            unsafe {
                let value = wx_base::wx_string_from(value);
                ffi::wxTextEntry_SetValue(Self::ptr_from(&self), value)
            }
        }
        fn undo(&self) {
            unsafe { ffi::wxTextEntry_Undo(Self::ptr_from(&self)) }
        }
        fn write_text(&self, text: &str) {
            unsafe {
                let text = wx_base::wx_string_from(text);
                ffi::wxTextEntry_WriteText(Self::ptr_from(&self), text)
            }
        }
    }
}
use methods::*;

mod ffi {
    use std::os::raw::{c_int, c_long, c_void};
    extern "C" {
        pub fn wxObject_delete(self_: *mut c_void);

        // WindowList
        pub fn wxWindowList_new() -> *mut c_void;
        pub fn wxWindowList_delete(self_: *mut c_void);
        pub fn wxWindowList_IsEmpty(self_: *mut c_void) -> bool;

        pub fn wxRustMessageBox(
            message: *const c_void,
            caption: *const c_void,
            style: c_int,
            parent: *mut c_void,
            x: c_int,
            y: c_int,
        );

        pub fn wxTextCtrl_ToTextEntry(obj: *mut c_void) -> *mut c_void;
        // wxTextEntry
        pub fn wxTextEntry_delete(self_: *mut c_void);
        pub fn wxTextEntry_AppendText(self_: *mut c_void, text: *const c_void);
        pub fn wxTextEntry_AutoComplete(self_: *mut c_void, choices: *const c_void) -> bool;
        pub fn wxTextEntry_AutoComplete1(self_: *mut c_void, completer: *mut c_void) -> bool;
        pub fn wxTextEntry_AutoCompleteFileNames(self_: *mut c_void) -> bool;
        pub fn wxTextEntry_AutoCompleteDirectories(self_: *mut c_void) -> bool;
        pub fn wxTextEntry_CanCopy(self_: *const c_void) -> bool;
        pub fn wxTextEntry_CanCut(self_: *const c_void) -> bool;
        pub fn wxTextEntry_CanPaste(self_: *const c_void) -> bool;
        pub fn wxTextEntry_CanRedo(self_: *const c_void) -> bool;
        pub fn wxTextEntry_CanUndo(self_: *const c_void) -> bool;
        pub fn wxTextEntry_ChangeValue(self_: *mut c_void, value: *const c_void);
        pub fn wxTextEntry_Clear(self_: *mut c_void);
        pub fn wxTextEntry_Copy(self_: *mut c_void);
        pub fn wxTextEntry_Cut(self_: *mut c_void);
        pub fn wxTextEntry_ForceUpper(self_: *mut c_void);
        pub fn wxTextEntry_GetInsertionPoint(self_: *const c_void) -> c_long;
        // NOT_SUPPORTED: pub fn wxTextEntry_GetLastPosition(self_: *const c_void) -> wxTextPos;
        pub fn wxTextEntry_GetRange(self_: *const c_void, from: c_long, to: c_long) -> *mut c_void;
        pub fn wxTextEntry_GetSelection(self_: *const c_void, from: *mut c_void, to: *mut c_void);
        pub fn wxTextEntry_GetStringSelection(self_: *const c_void) -> *mut c_void;
        pub fn wxTextEntry_GetValue(self_: *const c_void) -> *mut c_void;
        pub fn wxTextEntry_IsEditable(self_: *const c_void) -> bool;
        pub fn wxTextEntry_IsEmpty(self_: *const c_void) -> bool;
        pub fn wxTextEntry_Paste(self_: *mut c_void);
        pub fn wxTextEntry_Redo(self_: *mut c_void);
        pub fn wxTextEntry_Remove(self_: *mut c_void, from: c_long, to: c_long);
        pub fn wxTextEntry_Replace(
            self_: *mut c_void,
            from: c_long,
            to: c_long,
            value: *const c_void,
        );
        pub fn wxTextEntry_SetEditable(self_: *mut c_void, editable: bool);
        pub fn wxTextEntry_SetInsertionPoint(self_: *mut c_void, pos: c_long);
        pub fn wxTextEntry_SetInsertionPointEnd(self_: *mut c_void);
        // NOT_SUPPORTED: pub fn wxTextEntry_SetMaxLength(self_: *mut c_void, len: unsigned long);
        pub fn wxTextEntry_SetSelection(self_: *mut c_void, from: c_long, to: c_long);
        pub fn wxTextEntry_SelectAll(self_: *mut c_void);
        pub fn wxTextEntry_SelectNone(self_: *mut c_void);
        pub fn wxTextEntry_SetHint(self_: *mut c_void, hint: *const c_void) -> bool;
        pub fn wxTextEntry_GetHint(self_: *const c_void) -> *mut c_void;
        pub fn wxTextEntry_SetMargins(self_: *mut c_void, pt: *const c_void) -> bool;
        pub fn wxTextEntry_SetMargins1(self_: *mut c_void, left: c_int, top: c_int) -> bool;
        pub fn wxTextEntry_GetMargins(self_: *const c_void) -> *mut c_void;
        pub fn wxTextEntry_SetValue(self_: *mut c_void, value: *const c_void);
        pub fn wxTextEntry_Undo(self_: *mut c_void);
        pub fn wxTextEntry_WriteText(self_: *mut c_void, text: *const c_void);

    }
}

// Mix-in wxTextEntry manually
impl<const OWNED: bool> TextEntryMethods for TextCtrlIsOwned<OWNED> {
    fn ptr_from(obj: &Self) -> *mut c_void {
        unsafe { ffi::wxTextCtrl_ToTextEntry(obj.as_ptr()) }
    }
}

pub struct FrameBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    title: String,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
}
impl<'a, P: WindowMethods> Buildable<'a, P, FrameBuilder<'a, P>> for Frame {
    fn builder(parent: Option<&'a P>) -> FrameBuilder<'a, P> {
        FrameBuilder {
            parent: parent,
            id: ID_ANY,
            title: "".to_string(),
            pos: None,
            size: None,
            style: DEFAULT_FRAME_STYLE,
        }
    }
}
impl<'a, P: WindowMethods> FrameBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_string();
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn build(&mut self) -> Frame {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        Frame::new(
            self.parent,
            self.id,
            &self.title,
            &pos,
            &size,
            self.style,
            "",
        )
    }
}

pub struct PanelBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
}
impl<'a, P: WindowMethods> Buildable<'a, P, PanelBuilder<'a, P>> for Panel {
    fn builder(parent: Option<&'a P>) -> PanelBuilder<'a, P> {
        PanelBuilder {
            parent: parent,
            pos: None,
            size: None,
            style: TAB_TRAVERSAL,
        }
    }
}
impl<'a, P: WindowMethods> PanelBuilder<'a, P> {
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn build(&mut self) -> Panel {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        Panel::new(self.parent, ID_ANY, &pos, &size, self.style, "")
    }
}

// pub struct ActivityIndicatorBuilder<'a, P: WindowMethods> {
//     parent: Option<&'a P>,
//     id: c_int,
//     pos: Option<Point>,
//     size: Option<Size>,
//     style: c_long,
// }
// impl<'a, P: WindowMethods> Buildable<'a, P, ActivityIndicatorBuilder<'a, P>> for ActivityIndicator {
//     fn builder(parent: Option<&'a P>) -> ActivityIndicatorBuilder<'a, P> {
//         ActivityIndicatorBuilder {
//             parent: parent,
//             id: ID_ANY,
//             pos: None,
//             size: None,
//             style: 0,
//         }
//     }
// }
// impl<'a, P: WindowMethods> ActivityIndicatorBuilder<'a, P> {
//     pub fn id(&mut self, id: c_int) -> &mut Self {
//         self.id = id;
//         self
//     }
//     pub fn pos(&mut self, pos: Point) -> &mut Self {
//         self.pos = Some(pos);
//         self
//     }
//     pub fn size(&mut self, size: Size) -> &mut Self {
//         self.size = Some(size);
//         self
//     }
//     pub fn style(&mut self, style: c_long) -> &mut Self {
//         self.style = style;
//         self
//     }
//     pub fn build(&mut self) -> ActivityIndicator {
//         let pos = self.pos.take().unwrap_or_else(|| Point::default());
//         let size = self.size.take().unwrap_or_else(|| Size::default());
//         ActivityIndicator::new(self.parent, self.id, &pos, &size, self.style, "")
//     }
// }

pub struct ButtonBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    label: String,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
    validator: Option<Validator>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, ButtonBuilder<'a, P>> for Button {
    fn builder(parent: Option<&'a P>) -> ButtonBuilder<'a, P> {
        ButtonBuilder {
            parent: parent,
            id: ID_ANY,
            label: "".to_string(),
            pos: None,
            size: None,
            style: 0,
            validator: None,
        }
    }
}
impl<'a, P: WindowMethods> ButtonBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn label(&mut self, label: &str) -> &mut Self {
        self.label = label.to_string();
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn validator(&mut self, validator: Validator) -> &mut Self {
        self.validator = Some(validator);
        self
    }
    pub fn build(&mut self) -> Button {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        let validator = self
            .validator
            .take()
            .unwrap_or_else(|| Validator::default());
        Button::new(
            self.parent,
            self.id,
            &self.label,
            &pos,
            &size,
            self.style,
            &validator,
            "",
        )
    }
}

pub struct CheckBoxBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    label: String,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
    validator: Option<Validator>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, CheckBoxBuilder<'a, P>> for CheckBox {
    fn builder(parent: Option<&'a P>) -> CheckBoxBuilder<'a, P> {
        CheckBoxBuilder {
            parent: parent,
            id: ID_ANY,
            label: "".to_string(),
            pos: None,
            size: None,
            style: 0,
            validator: None,
        }
    }
}
impl<'a, P: WindowMethods> CheckBoxBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn label(&mut self, label: &str) -> &mut Self {
        self.label = label.to_string();
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn validator(&mut self, validator: Validator) -> &mut Self {
        self.validator = Some(validator);
        self
    }
    pub fn build(&mut self) -> CheckBox {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        let validator = self
            .validator
            .take()
            .unwrap_or_else(|| Validator::default());
        CheckBox::new(
            self.parent,
            self.id,
            &self.label,
            &pos,
            &size,
            self.style,
            &validator,
            "",
        )
    }
}

pub struct ListBoxBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    pos: Option<Point>,
    size: Option<Size>,
    choices: Option<ArrayString>,
    style: c_long,
    validator: Option<Validator>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, ListBoxBuilder<'a, P>> for ListBox {
    fn builder(parent: Option<&'a P>) -> ListBoxBuilder<'a, P> {
        ListBoxBuilder {
            parent: parent,
            id: ID_ANY,
            pos: None,
            size: None,
            choices: None,
            style: 0,
            validator: None,
        }
    }
}
impl<'a, P: WindowMethods> ListBoxBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn choices(&mut self, choices: ArrayString) -> &mut Self {
        self.choices = Some(choices);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn validator(&mut self, validator: Validator) -> &mut Self {
        self.validator = Some(validator);
        self
    }
    pub fn build(&mut self) -> ListBox {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        let choices = self.choices.take().unwrap_or_else(|| ArrayString::new());
        let validator = self
            .validator
            .take()
            .unwrap_or_else(|| Validator::default());
        ListBox::new(
            self.parent,
            self.id,
            &pos,
            &size,
            &choices,
            self.style,
            &validator,
            "",
        )
    }
}

pub struct NotebookBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
}
impl<'a, P: WindowMethods> Buildable<'a, P, NotebookBuilder<'a, P>> for Notebook {
    fn builder(parent: Option<&'a P>) -> NotebookBuilder<'a, P> {
        NotebookBuilder {
            parent: parent,
            id: ID_ANY,
            pos: None,
            size: None,
            style: 0,
        }
    }
}
impl<'a, P: WindowMethods> NotebookBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn build(&mut self) -> Notebook {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        Notebook::new(self.parent, self.id, &pos, &size, self.style, "")
    }
}

pub struct RadioBoxBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    label: String,
    pos: Option<Point>,
    size: Option<Size>,
    choices: Option<ArrayString>,
    major_dimension: c_int,
    style: c_long,
    validator: Option<Validator>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, RadioBoxBuilder<'a, P>> for RadioBox {
    fn builder(parent: Option<&'a P>) -> RadioBoxBuilder<'a, P> {
        RadioBoxBuilder {
            parent: parent,
            id: ID_ANY,
            label: "".to_string(),
            pos: None,
            size: None,
            choices: None,
            major_dimension: 0,
            style: RA_SPECIFY_COLS.into(),
            validator: None,
        }
    }
}
impl<'a, P: WindowMethods> RadioBoxBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn label(&mut self, label: &str) -> &mut Self {
        self.label = label.to_string();
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn choices(&mut self, choices: ArrayString) -> &mut Self {
        self.choices = Some(choices);
        self
    }
    pub fn major_dimension(&mut self, major_dimension: c_int) -> &mut Self {
        self.major_dimension = major_dimension;
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn validator(&mut self, validator: Validator) -> &mut Self {
        self.validator = Some(validator);
        self
    }
    pub fn build(&mut self) -> RadioBox {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        let choices = self.choices.take().unwrap_or_else(|| ArrayString::new());
        let validator = self
            .validator
            .take()
            .unwrap_or_else(|| Validator::default());
        RadioBox::new(
            self.parent,
            self.id,
            &self.label,
            &pos,
            &size,
            &choices,
            self.major_dimension,
            self.style,
            &validator,
            "",
        )
    }
}

pub struct StaticBoxBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    label: String,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
}
impl<'a, P: WindowMethods> Buildable<'a, P, StaticBoxBuilder<'a, P>> for StaticBox {
    fn builder(parent: Option<&'a P>) -> StaticBoxBuilder<'a, P> {
        StaticBoxBuilder {
            parent: parent,
            id: ID_ANY,
            label: "".to_string(),
            pos: None,
            size: None,
            style: 0,
        }
    }
}
impl<'a, P: WindowMethods> StaticBoxBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn label(&mut self, label: &str) -> &mut Self {
        self.label = label.to_string();
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn build(&mut self) -> StaticBox {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        StaticBox::new(
            self.parent,
            self.id,
            &self.label,
            &pos,
            &size,
            self.style,
            "",
        )
    }
}

pub struct StaticTextBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    label: String,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
}
impl<'a, P: WindowMethods> Buildable<'a, P, StaticTextBuilder<'a, P>> for StaticText {
    fn builder(parent: Option<&'a P>) -> StaticTextBuilder<'a, P> {
        StaticTextBuilder {
            parent: parent,
            id: ID_ANY,
            label: "".to_string(),
            pos: None,
            size: None,
            style: 0,
        }
    }
}
impl<'a, P: WindowMethods> StaticTextBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn label(&mut self, label: &str) -> &mut Self {
        self.label = label.to_string();
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn build(&mut self) -> StaticText {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        StaticText::new(
            self.parent,
            self.id,
            &self.label,
            &pos,
            &size,
            self.style,
            "",
        )
    }
}

pub struct TextCtrlBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    value: String,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
    validator: Option<Validator>,
}
impl<'a, P: WindowMethods> Buildable<'a, P, TextCtrlBuilder<'a, P>> for TextCtrl {
    fn builder(parent: Option<&'a P>) -> TextCtrlBuilder<'a, P> {
        TextCtrlBuilder {
            parent: parent,
            id: ID_ANY,
            value: "".to_string(),
            pos: None,
            size: None,
            style: 0,
            validator: None,
        }
    }
}
impl<'a, P: WindowMethods> TextCtrlBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn value(&mut self, value: &str) -> &mut Self {
        self.value = value.to_string();
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn validator(&mut self, validator: Validator) -> &mut Self {
        self.validator = Some(validator);
        self
    }
    pub fn build(&mut self) -> TextCtrl {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        let validator = self
            .validator
            .take()
            .unwrap_or_else(|| Validator::default());
        TextCtrl::new(
            self.parent,
            self.id,
            &self.value,
            &pos,
            &size,
            self.style,
            &validator,
            "",
        )
    }
}

pub struct ToolBarBuilder<'a, P: WindowMethods> {
    parent: Option<&'a P>,
    id: c_int,
    pos: Option<Point>,
    size: Option<Size>,
    style: c_long,
}
impl<'a, P: WindowMethods> Buildable<'a, P, ToolBarBuilder<'a, P>> for ToolBar {
    fn builder(parent: Option<&'a P>) -> ToolBarBuilder<'a, P> {
        ToolBarBuilder {
            parent: parent,
            id: ID_ANY,
            pos: None,
            size: None,
            style: TB_HORIZONTAL.into(),
        }
    }
}
impl<'a, P: WindowMethods> ToolBarBuilder<'a, P> {
    pub fn id(&mut self, id: c_int) -> &mut Self {
        self.id = id;
        self
    }
    pub fn pos(&mut self, pos: Point) -> &mut Self {
        self.pos = Some(pos);
        self
    }
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn style(&mut self, style: c_long) -> &mut Self {
        self.style = style;
        self
    }
    pub fn build(&mut self) -> ToolBar {
        let pos = self.pos.take().unwrap_or_else(|| Point::default());
        let size = self.size.take().unwrap_or_else(|| Size::default());
        ToolBar::new(self.parent, self.id, &pos, &size, self.style, "")
    }
}

impl MenuItemBuilder for Menu {
    fn item<ID: Into<c_int>>(self, id: ID, item: &str) -> Self {
        self.item_h(id, item, "")
    }
    fn item_h<ID: Into<c_int>>(self, id: ID, item: &str, help: &str) -> Self {
        self.append_int_str(id.into(), item, help, ITEM_NORMAL);
        self
    }
    fn check_item<ID: Into<c_int>>(self, id: ID, item: &str) -> Self {
        self.append_check_item(id.into(), item, "");
        self
    }
    fn radio_item<ID: Into<c_int>>(self, id: ID, item: &str) -> Self {
        self.append_radio_item(id.into(), item, "");
        self
    }
    fn sub_menu<M: MenuMethods>(self, text: &str, submenu: &M) -> Self {
        self.append_sub_menu(Some(submenu), text, "");
        self
    }
    fn separator(self) -> Self {
        self.append_separator();
        self
    }
}

// wxDefaultPosition
impl<const OWNED: bool> Default for PointIsOwned<OWNED> {
    fn default() -> Self {
        PointIsOwned::new_with_int(-1, -1)
    }
}
// wxDefaultSize
impl<const OWNED: bool> Default for SizeIsOwned<OWNED> {
    fn default() -> Self {
        SizeIsOwned::new_with_int(-1, -1)
    }
}
// wxDefaultValidator
impl<const OWNED: bool> Default for ValidatorIsOwned<OWNED> {
    fn default() -> Self {
        ValidatorIsOwned::new()
    }
}

wx_class! { WindowList =
    WindowListIsOwned<true>(wxWindowList) impl
        WindowListMethods
}
impl<const OWNED: bool> WindowListIsOwned<OWNED> {
    pub fn new() -> Self {
        unsafe { WindowListIsOwned(ffi::wxWindowList_new()) }
    }
}
impl<const OWNED: bool> Drop for WindowListIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxWindowList_delete(self.0) }
        }
    }
}

pub fn message_box<T: WindowMethods>(
    message: &str,
    caption: &str,
    style: c_int,
    parent: Option<&T>,
) {
    unsafe {
        let message = wx_base::wx_string_from(message);
        let caption = wx_base::wx_string_from(caption);
        let parent = match parent {
            Some(r) => r.as_ptr(),
            None => ptr::null_mut(),
        };
        ffi::wxRustMessageBox(message, caption, style, parent, -1, -1)
    }
}
