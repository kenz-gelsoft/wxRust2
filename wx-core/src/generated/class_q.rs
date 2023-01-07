use super::*;

// wxQuantize
wxwidgets! {
    /// Performs quantization, or colour reduction, on a wxImage.
    /// - [`Quantize`] represents a C++ `wxQuantize` class instance which your code has ownership, [`QuantizeInRust`]`<false>` represents one which don't own.
    /// - Use [`Quantize`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxQuantize` class's documentation](https://docs.wxwidgets.org/3.2/classwx_quantize.html) for more details.
    #[doc(alias = "wxQuantize")]
    #[doc(alias = "Quantize")]
    class Quantize
        = QuantizeInRust<true>(wxQuantize) impl
        QuantizeMethods,
        ObjectMethods
}
impl<const OWNED: bool> QuantizeInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxQuantize::wxQuantize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_quantize.html#a547099d33907c05795eaba1526e14a73).
    pub fn new() -> QuantizeInRust<OWNED> {
        unsafe { QuantizeInRust(ffi::wxQuantize_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for QuantizeInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<QuantizeInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: QuantizeInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for QuantizeInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxQuantize_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for QuantizeInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxQueryLayoutInfoEvent
wxwidgets! {
    /// This event is sent when wxLayoutAlgorithm wishes to get the size, orientation and alignment of a window.
    /// - [`QueryLayoutInfoEvent`] represents a C++ `wxQueryLayoutInfoEvent` class instance which your code has ownership, [`QueryLayoutInfoEventInRust`]`<false>` represents one which don't own.
    /// - Use [`QueryLayoutInfoEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxQueryLayoutInfoEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_query_layout_info_event.html) for more details.
    #[doc(alias = "wxQueryLayoutInfoEvent")]
    #[doc(alias = "QueryLayoutInfoEvent")]
    class QueryLayoutInfoEvent
        = QueryLayoutInfoEventInRust<true>(wxQueryLayoutInfoEvent) impl
        QueryLayoutInfoEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> QueryLayoutInfoEventInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxQueryLayoutInfoEvent::wxQueryLayoutInfoEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_query_layout_info_event.html#ae49fe2e9f1e59fa1bcb24380b76c5f09).
    pub fn new(id: c_int) -> QueryLayoutInfoEventInRust<OWNED> {
        unsafe { QueryLayoutInfoEventInRust(ffi::wxQueryLayoutInfoEvent_new(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for QueryLayoutInfoEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<QueryLayoutInfoEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: QueryLayoutInfoEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<QueryLayoutInfoEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: QueryLayoutInfoEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for QueryLayoutInfoEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxQueryLayoutInfoEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for QueryLayoutInfoEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
