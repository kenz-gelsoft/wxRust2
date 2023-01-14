use super::*;

// wxQuantize
wxwidgets! {
    /// Performs quantization, or colour reduction, on a wxImage.
    /// - [`Quantize`] represents a C++ `wxQuantize` class instance which your code has ownership, [`QuantizeFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Quantize`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxQuantize` class's documentation](https://docs.wxwidgets.org/3.2/classwx_quantize.html) for more details.
    #[doc(alias = "wxQuantize")]
    #[doc(alias = "Quantize")]
    class Quantize
        = QuantizeFromCpp<false>(wxQuantize) impl
        QuantizeMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> QuantizeFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxQuantize::wxQuantize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_quantize.html#a547099d33907c05795eaba1526e14a73).
    pub fn new() -> QuantizeFromCpp<FROM_CPP> {
        unsafe { QuantizeFromCpp(ffi::wxQuantize_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for QuantizeFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<QuantizeFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: QuantizeFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for QuantizeFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxQuantize_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for QuantizeFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxQueryLayoutInfoEvent
wxwidgets! {
    /// This event is sent when wxLayoutAlgorithm wishes to get the size, orientation and alignment of a window.
    /// - [`QueryLayoutInfoEvent`] represents a C++ `wxQueryLayoutInfoEvent` class instance which your code has ownership, [`QueryLayoutInfoEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`QueryLayoutInfoEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxQueryLayoutInfoEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_query_layout_info_event.html) for more details.
    #[doc(alias = "wxQueryLayoutInfoEvent")]
    #[doc(alias = "QueryLayoutInfoEvent")]
    class QueryLayoutInfoEvent
        = QueryLayoutInfoEventFromCpp<false>(wxQueryLayoutInfoEvent) impl
        QueryLayoutInfoEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> QueryLayoutInfoEventFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxQueryLayoutInfoEvent::wxQueryLayoutInfoEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_query_layout_info_event.html#ae49fe2e9f1e59fa1bcb24380b76c5f09).
    pub fn new(id: c_int) -> QueryLayoutInfoEventFromCpp<FROM_CPP> {
        unsafe { QueryLayoutInfoEventFromCpp(ffi::wxQueryLayoutInfoEvent_new(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for QueryLayoutInfoEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<QueryLayoutInfoEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: QueryLayoutInfoEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<QueryLayoutInfoEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: QueryLayoutInfoEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for QueryLayoutInfoEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxQueryLayoutInfoEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for QueryLayoutInfoEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
