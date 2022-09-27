#![doc = include_str!("../README.md")]

use std::marker::PhantomData;
use std::mem;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::slice;
use std::str;

mod macros;

mod constants;
#[doc(hidden)]
pub use constants::*;
mod manual;
#[doc(hidden)]
pub use manual::*;

mod generated;
pub use generated::class::*;

use methods::*;

mod ffi {
    use std::os::raw::{c_char, c_int, c_uchar, c_void};

    #[repr(C)]
    pub struct UTF8Data {
        pub data: *mut c_uchar,
        pub length: usize,
    }
    extern "C" {
        pub fn wxObject_delete(self_: *mut c_void);

        pub fn AppSetOnInit(aFn: *mut c_void, aParam: *mut c_void);
        pub fn wxEvtHandler_Bind(
            self_: *mut c_void,
            eventType: c_int,
            aFn: *mut c_void,
            aParam: *mut c_void,
        );
        
        pub fn wxEvtHandler_CallAfter(
            self_: *mut c_void,
            aFn: *mut c_void,
            aParam: *mut c_void,
        );

        // String
        pub fn wxString_new(psz: *const c_uchar, nLength: usize) -> *mut c_void;
        pub fn wxString_delete(self_: *mut c_void);
        pub fn wxString_UTF8Data(self_: *mut c_void) -> UTF8Data;

        // (wx)String::const_iterator
        pub fn wxStringConstIterator_new() -> *mut c_void;
        pub fn wxStringConstIterator_delete(self_: *mut c_void);
        pub fn wxStringConstIterator_IndexIn(self_: *mut c_void, s: *const c_void) -> usize;

        // ArrayInt
        pub fn wxArrayInt_new() -> *mut c_void;
        pub fn wxArrayInt_delete(self_: *mut c_void);
        pub fn wxArrayInt_Add(self_: *mut c_void, i: c_int);
        pub fn wxArrayInt_Item(self_: *mut c_void, index: usize) -> c_int;

        // ArrayString
        pub fn wxArrayString_new() -> *mut c_void;
        pub fn wxArrayString_delete(self_: *mut c_void);
        pub fn wxArrayString_Add(self_: *mut c_void, s: *const c_void);

        pub fn wxRustEntry(argc: *mut c_int, argv: *mut *mut c_char) -> c_int;

        // WeakRef
        pub fn OpaqueWeakRef_new(obj: *mut c_void) -> *mut c_void;
        pub fn OpaqueWeakRef_copy(obj: *mut c_void) -> *mut c_void;
        pub fn OpaqueWeakRef_delete(self_: *mut c_void);
        pub fn OpaqueWeakRef_Get(self_: *mut c_void) -> *mut c_void;

        // DateTime
        pub fn wxDateTime_ParseDate(
            self_: *mut c_void,
            date: *const c_void,
            end: *mut c_void,
        ) -> bool;
    }
}

pub mod methods {
    pub use super::generated::methods::*;
    use super::*;

    pub trait Bindable {
        fn bind<E: EventMethods, F: Fn(&E) + 'static>(&self, event_type: RustEvent, closure: F);
        fn call_after<F: Fn() + 'static>(&self, closure: F);
    }

    pub trait ArrayIntMethods: WxRustMethods {
        fn add(&self, i: c_int) {
            unsafe { ffi::wxArrayInt_Add(self.as_ptr(), i) }
        }
        fn item(&self, index: usize) -> c_int {
            unsafe { ffi::wxArrayInt_Item(self.as_ptr(), index) }
        }
    }

    pub trait ArrayStringMethods: WxRustMethods {
        fn add(&self, s: &str) {
            unsafe {
                let s = WxString::from(s);
                ffi::wxArrayString_Add(self.as_ptr(), s.as_ptr())
            }
        }
    }

    pub trait StringConstIteratorMethods: WxRustMethods {
        fn index_in(&self, s: *const c_void) -> usize {
            unsafe { ffi::wxStringConstIterator_IndexIn(self.as_ptr(), s) }
        }
    }

    // TODO: Support manual(semi-auto) binding in codegen
    //
    // This trait should be `DateTimeMethods` and, the base trait
    // should be `DateTimeMethodsAuto` for API consistencey.
    pub trait DateTimeMethodsManual: DateTimeMethods {
        fn parse_date(&self, date: &str) -> Option<usize> {
            unsafe {
                let end = StringConstIterator::new();
                let date = WxString::from(date);
                let date = date.as_ptr();
                if ffi::wxDateTime_ParseDate(self.as_ptr(), date, end.as_ptr()) {
                    Some(end.index_in(date))
                } else {
                    None
                }
            }
        }
    }

    pub trait DynamicCast: ObjectMethods {
        fn class_info() -> ClassInfoIsOwned<false>;
        fn as_unowned<T: DynamicCast>(&self) -> Option<T::Unowned> {
            if self.is_kind_of(Some(&T::class_info())) {
                unsafe { Some(T::from_unowned_ptr(self.as_ptr())) }
            } else {
                None
            }
        }
    }

    pub trait Trackable<T>: EvtHandlerMethods {
        fn to_weak_ref(&self) -> WeakRef<T>;
    }
}

// wxString
pub struct WxString(*mut c_void);
impl WxString {
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Self {
        WxString(ptr)
    }
    pub unsafe fn as_ptr(&self) -> *mut c_void {
        return self.0;
    }
    pub fn to_str<'a>(&'a self) -> &'a str {
        unsafe {
            let utf8 = ffi::wxString_UTF8Data(self.as_ptr());
            let len = utf8.length;
            let slice = slice::from_raw_parts(utf8.data, len);
            str::from_utf8_unchecked(slice)
        }
    }
}
impl From<WxString> for String {
    fn from(s: WxString) -> Self {
        s.to_str().to_owned()
    }
}
impl From<&str> for WxString {
    fn from(s: &str) -> Self {
        unsafe { WxString(ffi::wxString_new(s.as_ptr(), s.len())) }
    }
}
impl Drop for WxString {
    fn drop(&mut self) {
        unsafe { ffi::wxString_delete(self.0) }
    }
}

// Rust closure to wx calablle function+param pair.
unsafe fn to_wx_callable<F: Fn(*mut c_void) + 'static>(closure: F) -> (*mut c_void, *mut c_void) {
    unsafe fn trampoline<F: Fn(*mut c_void) + 'static>(closure: *mut c_void, arg: *mut c_void) {
        let closure = &*(closure as *const F);
        closure(arg);
    }
    // pass the pointer in the heap to avoid move.
    let closure = Box::new(closure);
    (trampoline::<F> as _, Box::into_raw(closure) as _)
}

// TODO auto generate
pub enum RustEvent {
    // MEMO: wx32 introduced event types are commented out
    AsyncMethodCall,
    BookctrlPageChanged,
    Button,
    CheckBox,
    CheckListBox,
    ChildFocus,
    Choice,
    ComboBox,
    ComboBoxCloseUp,
    ComboBoxDropDown,
    CommandEnter,
    CommandKillFocus,
    CommandLeftClick,
    CommandLeftDClick,
    CommandRightClick,
    CommandRightDClick,
    CommandSetFocus,
    ContextMenu,
    Create,
    Destroy,
    DisplayChanged,
    // DPIChanged,
    DropFiles,
    EraseBackground,
    // FullScreen,
    // GesturePan,
    // GestureRotate,
    // GestureZoom,
    Iconize,
    Idle,
    InitDialog,
    ListBox,
    ListBoxDClick,
    // LongPress,
    Maximize,
    Menu,
    MouseCaptureChanged,
    MouseCaptureLost,
    NavigationKey,
    NCPaint,
    Paint,
    PaletteChanged,
    // PressAndTap,
    QueryNewPalette,
    RadioBox,
    RadioButton,
    ScrollBar,
    SetCursor,
    Show,
    Slider,
    SysColourChanged,
    Text,
    Thread,
    Timer,
    ToolDropDown,
    ToolEnter,
    ToolRClicked,
    // TwoFingerTap,
    UpdateUI,
    VLBox,
}
impl<T: EvtHandlerMethods> Bindable for T {
    fn bind<E: EventMethods, F: Fn(&E) + 'static>(&self, event_type: RustEvent, closure: F) {
        unsafe {
            let (f, param) = to_wx_callable(move |arg: *mut c_void| {
                E::with_ptr(arg, |event| {
                    closure(event);
                });
            });
            ffi::wxEvtHandler_Bind(self.as_ptr(), event_type as c_int, f, param);
        }
    }
    fn call_after<F: Fn() + 'static>(&self, closure: F) {
        unsafe {
            let (f, param) = to_wx_callable(move |_: *mut c_void| {
                closure();
            });
            ffi::wxEvtHandler_CallAfter(self.as_ptr(), f, param);
        }
    }
}

// Effectively all wxEvtHandlers are wxTrackable.
impl<T: EvtHandlerMethods> Trackable<T> for T {
    fn to_weak_ref(&self) -> WeakRef<T> {
        unsafe { WeakRef::from(self.as_ptr()) }
    }
}

// wxApp
pub enum App {}
impl App {
    pub fn on_init<F: Fn(*mut c_void) + 'static>(closure: F) {
        unsafe {
            let (f, param) = to_wx_callable(closure);
            ffi::AppSetOnInit(f, param);
        }
    }
    pub fn run<F: Fn(*mut c_void) + 'static>(closure: F) {
        Self::on_init(closure);
        entry();
    }
}

wxwidgets! {
    class ArrayInt
        = ArrayIntIsOwned<true>(wxArrayInt) impl
        ArrayIntMethods
}
impl<const OWNED: bool> ArrayIntIsOwned<OWNED> {
    pub fn new() -> Self {
        unsafe { ArrayIntIsOwned(ffi::wxArrayInt_new()) }
    }
}
impl<const OWNED: bool> Drop for ArrayIntIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxArrayInt_delete(self.0) }
        }
    }
}

wxwidgets! {
    class ArrayString
        = ArrayStringIsOwned<true>(wxArrayString) impl
        ArrayStringMethods
}
impl<const OWNED: bool> ArrayStringIsOwned<OWNED> {
    pub fn new() -> Self {
        unsafe { ArrayStringIsOwned(ffi::wxArrayString_new()) }
    }
}
impl<const OWNED: bool> Drop for ArrayStringIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxArrayString_delete(self.0) }
        }
    }
}

// (wx)String::const_iterator
wxwidgets! {
    class StringConstIterator
        = StringConstIteratorIsOwned<true>(wxStringConstIterator) impl
        StringConstIteratorMethods
}
impl<const OWNED: bool> StringConstIteratorIsOwned<OWNED> {
    pub fn new() -> Self {
        unsafe { StringConstIteratorIsOwned(ffi::wxStringConstIterator_new()) }
    }
}
impl<const OWNED: bool> Drop for StringConstIteratorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxStringConstIterator_delete(self.0) }
        }
    }
}

// wxEntry
pub fn entry() {
    let args: Vec<String> = std::env::args().collect();
    let mut argv: Vec<*mut c_char> = Vec::with_capacity(args.len() + 1);
    for arg in &args {
        argv.push(arg.as_ptr() as *mut c_char);
    }
    argv.push(ptr::null_mut()); // Nul terminator.
    let mut argc: c_int = args.len().try_into().unwrap();
    unsafe {
        ffi::wxRustEntry(&mut argc, argv.as_mut_ptr());
    }
}

// wxWeakRef
pub struct WeakRef<T>(*mut c_void, PhantomData<T>);
impl<T: WxRustMethods> WeakRef<T> {
    pub unsafe fn from(ptr: *mut c_void) -> Self {
        let ptr = if ptr.is_null() {
            ptr
        } else {
            ffi::OpaqueWeakRef_new(ptr)
        };
        WeakRef(ptr, PhantomData)
    }
    pub fn get(&self) -> Option<T::Unowned> {
        unsafe {
            let ptr = self.0;
            let ptr = if ptr.is_null() {
                ptr
            } else {
                ffi::OpaqueWeakRef_Get(ptr)
            };
            if ptr.is_null() {
                None
            } else {
                Some(T::from_unowned_ptr(ptr))
            }
        }
    }
}
impl<T: WxRustMethods> Clone for WeakRef<T> {
    fn clone(&self) -> Self {
        unsafe {
            let ptr = ffi::OpaqueWeakRef_copy(self.0);
            WeakRef(ptr, PhantomData)
        }
    }
}
impl<T> Drop for WeakRef<T> {
    fn drop(&mut self) {
        unsafe { ffi::OpaqueWeakRef_delete(self.0) }
    }
}

impl<const OWNED: bool> DateTimeMethodsManual for DateTimeIsOwned<OWNED> {}
