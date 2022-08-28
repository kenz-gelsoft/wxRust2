use super::*;

// wxQuantize
wxwidgets! {
    /// Performs quantization, or colour reduction, on a wxImage.
    ///
    /// [See `wxQuantize`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_quantize.html)
    #[doc(alias = "wxQuantize")]
    #[doc(alias = "Quantize")]
    class Quantize
        = QuantizeIsOwned<true>(wxQuantize) impl
        QuantizeMethods,
        ObjectMethods
}
impl<const OWNED: bool> QuantizeIsOwned<OWNED> {
    /// Constructor.
    pub fn new() -> QuantizeIsOwned<OWNED> {
        unsafe { QuantizeIsOwned(ffi::wxQuantize_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for QuantizeIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<QuantizeIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: QuantizeIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for QuantizeIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxQuantize_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for QuantizeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxQueryLayoutInfoEvent
wxwidgets! {
    /// This event is sent when wxLayoutAlgorithm wishes to get the size, orientation and alignment of a window.
    ///
    /// [See `wxQueryLayoutInfoEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_query_layout_info_event.html)
    #[doc(alias = "wxQueryLayoutInfoEvent")]
    #[doc(alias = "QueryLayoutInfoEvent")]
    class QueryLayoutInfoEvent
        = QueryLayoutInfoEventIsOwned<true>(wxQueryLayoutInfoEvent) impl
        QueryLayoutInfoEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> QueryLayoutInfoEventIsOwned<OWNED> {
    /// Constructor.
    pub fn new(id: c_int) -> QueryLayoutInfoEventIsOwned<OWNED> {
        unsafe { QueryLayoutInfoEventIsOwned(ffi::wxQueryLayoutInfoEvent_new(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for QueryLayoutInfoEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<QueryLayoutInfoEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: QueryLayoutInfoEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<QueryLayoutInfoEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: QueryLayoutInfoEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for QueryLayoutInfoEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxQueryLayoutInfoEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for QueryLayoutInfoEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
