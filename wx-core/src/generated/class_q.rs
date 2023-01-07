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
impl<const IN_RUST: bool> QuantizeInRust<IN_RUST> {
    /// Constructor.
    ///
    /// See [C++ `wxQuantize::wxQuantize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_quantize.html#a547099d33907c05795eaba1526e14a73).
    pub fn new() -> QuantizeInRust<IN_RUST> {
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
impl<const IN_RUST: bool> From<QuantizeInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: QuantizeInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for QuantizeInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxQuantize_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for QuantizeInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
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
impl<const IN_RUST: bool> QueryLayoutInfoEventInRust<IN_RUST> {
    /// Constructor.
    ///
    /// See [C++ `wxQueryLayoutInfoEvent::wxQueryLayoutInfoEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_query_layout_info_event.html#ae49fe2e9f1e59fa1bcb24380b76c5f09).
    pub fn new(id: c_int) -> QueryLayoutInfoEventInRust<IN_RUST> {
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
impl<const IN_RUST: bool> From<QueryLayoutInfoEventInRust<IN_RUST>> for EventInRust<IN_RUST> {
    fn from(o: QueryLayoutInfoEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<QueryLayoutInfoEventInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: QueryLayoutInfoEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for QueryLayoutInfoEventInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxQueryLayoutInfoEvent_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for QueryLayoutInfoEventInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
