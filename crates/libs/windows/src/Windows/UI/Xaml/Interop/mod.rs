#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct BindableVectorChangedEventHandler(pub ::windows::core::IUnknown);
impl BindableVectorChangedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<IBindableObservableVector>, &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = BindableVectorChangedEventHandlerBox::<F> { vtable: &BindableVectorChangedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, IBindableObservableVector>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, vector: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), vector.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct BindableVectorChangedEventHandlerBox<F: FnMut(&::core::option::Option<IBindableObservableVector>, &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const BindableVectorChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<IBindableObservableVector>, &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> BindableVectorChangedEventHandlerBox<F> {
    const VTABLE: BindableVectorChangedEventHandler_Vtbl = BindableVectorChangedEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<BindableVectorChangedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, vector: ::windows::core::RawPtr, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&vector), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for BindableVectorChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BindableVectorChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BindableVectorChangedEventHandler {}
impl ::core::fmt::Debug for BindableVectorChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BindableVectorChangedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for BindableVectorChangedEventHandler {
    type Vtable = BindableVectorChangedEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x624cd4e1_d007_43b1_9c03_af4d3e6258c4);
}
unsafe impl ::windows::core::RuntimeType for BindableVectorChangedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{624cd4e1-d007-43b1-9c03-af4d3e6258c4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct BindableVectorChangedEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vector: ::windows::core::RawPtr, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct IBindableIterable(::windows::core::IUnknown);
impl IBindableIterable {
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn First(&self) -> ::windows::core::Result<IBindableIterator> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBindableIterator>(result__)
        }
    }
}
impl ::core::convert::From<IBindableIterable> for ::windows::core::IUnknown {
    fn from(value: IBindableIterable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindableIterable> for ::windows::core::IUnknown {
    fn from(value: &IBindableIterable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindableIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindableIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBindableIterable> for ::windows::core::IInspectable {
    fn from(value: IBindableIterable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindableIterable> for ::windows::core::IInspectable {
    fn from(value: &IBindableIterable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBindableIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBindableIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBindableIterable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindableIterable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindableIterable {}
impl ::core::fmt::Debug for IBindableIterable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindableIterable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IBindableIterable {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{036d2c08-df29-41af-8aa2-d774be62ba6f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IBindableIterable {
    type Vtable = IBindableIterable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x036d2c08_df29_41af_8aa2_d774be62ba6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableIterable_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub First: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct IBindableIterator(::windows::core::IUnknown);
impl IBindableIterator {
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn Current(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Current)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn HasCurrent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasCurrent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn MoveNext(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveNext)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<IBindableIterator> for ::windows::core::IUnknown {
    fn from(value: IBindableIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindableIterator> for ::windows::core::IUnknown {
    fn from(value: &IBindableIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindableIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindableIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBindableIterator> for ::windows::core::IInspectable {
    fn from(value: IBindableIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindableIterator> for ::windows::core::IInspectable {
    fn from(value: &IBindableIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBindableIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBindableIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBindableIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindableIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindableIterator {}
impl ::core::fmt::Debug for IBindableIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindableIterator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IBindableIterator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6a1d6c07-076d-49f2-8314-f52c9c9a8331}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IBindableIterator {
    type Vtable = IBindableIterator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a1d6c07_076d_49f2_8314_f52c9c9a8331);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableIterator_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct IBindableObservableVector(::windows::core::IUnknown);
impl IBindableObservableVector {
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VectorChanged<'a, Param0: ::windows::core::IntoParam<'a, BindableVectorChangedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VectorChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVectorChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveVectorChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn First(&self) -> ::windows::core::Result<IBindableIterator> {
        let this = &::windows::core::Interface::cast::<IBindableIterable>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBindableIterator>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAt)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn GetView(&self) -> ::windows::core::Result<IBindableVectorView> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBindableVectorView>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndexOf)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAt)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).InsertAt)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAt)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Append)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAtEnd)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IBindableObservableVector> for ::windows::core::IUnknown {
    fn from(value: IBindableObservableVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindableObservableVector> for ::windows::core::IUnknown {
    fn from(value: &IBindableObservableVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindableObservableVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindableObservableVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBindableObservableVector> for ::windows::core::IInspectable {
    fn from(value: IBindableObservableVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindableObservableVector> for ::windows::core::IInspectable {
    fn from(value: &IBindableObservableVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBindableObservableVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBindableObservableVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IBindableObservableVector> for IBindableIterable {
    type Error = ::windows::core::Error;
    fn try_from(value: IBindableObservableVector) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBindableObservableVector> for IBindableIterable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBindableObservableVector) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBindableIterable> for IBindableObservableVector {
    fn into_param(self) -> ::windows::core::Param<'a, IBindableIterable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBindableIterable> for &IBindableObservableVector {
    fn into_param(self) -> ::windows::core::Param<'a, IBindableIterable> {
        ::core::convert::TryInto::<IBindableIterable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IBindableObservableVector> for IBindableVector {
    type Error = ::windows::core::Error;
    fn try_from(value: IBindableObservableVector) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBindableObservableVector> for IBindableVector {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBindableObservableVector) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBindableVector> for IBindableObservableVector {
    fn into_param(self) -> ::windows::core::Param<'a, IBindableVector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBindableVector> for &IBindableObservableVector {
    fn into_param(self) -> ::windows::core::Param<'a, IBindableVector> {
        ::core::convert::TryInto::<IBindableVector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IBindableObservableVector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindableObservableVector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindableObservableVector {}
impl ::core::fmt::Debug for IBindableObservableVector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindableObservableVector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IBindableObservableVector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fe1eb536-7e7f-4f90-ac9a-474984aae512}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IBindableObservableVector {
    type Vtable = IBindableObservableVector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe1eb536_7e7f_4f90_ac9a_474984aae512);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableObservableVector_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub VectorChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VectorChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVectorChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVectorChanged: usize,
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct IBindableVector(::windows::core::IUnknown);
impl IBindableVector {
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAt)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn GetView(&self) -> ::windows::core::Result<IBindableVectorView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBindableVectorView>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndexOf)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAt)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).InsertAt)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAt)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Append)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAtEnd)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn First(&self) -> ::windows::core::Result<IBindableIterator> {
        let this = &::windows::core::Interface::cast::<IBindableIterable>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBindableIterator>(result__)
        }
    }
}
impl ::core::convert::From<IBindableVector> for ::windows::core::IUnknown {
    fn from(value: IBindableVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindableVector> for ::windows::core::IUnknown {
    fn from(value: &IBindableVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindableVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindableVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBindableVector> for ::windows::core::IInspectable {
    fn from(value: IBindableVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindableVector> for ::windows::core::IInspectable {
    fn from(value: &IBindableVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBindableVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBindableVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IBindableVector> for IBindableIterable {
    type Error = ::windows::core::Error;
    fn try_from(value: IBindableVector) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBindableVector> for IBindableIterable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBindableVector) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBindableIterable> for IBindableVector {
    fn into_param(self) -> ::windows::core::Param<'a, IBindableIterable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBindableIterable> for &IBindableVector {
    fn into_param(self) -> ::windows::core::Param<'a, IBindableIterable> {
        ::core::convert::TryInto::<IBindableIterable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IBindableVector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindableVector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindableVector {}
impl ::core::fmt::Debug for IBindableVector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindableVector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IBindableVector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{393de7de-6fd0-4c0d-bb71-47244a113e93}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IBindableVector {
    type Vtable = IBindableVector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x393de7de_6fd0_4c0d_bb71_47244a113e93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableVector_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub GetView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, index: *mut u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAtEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct IBindableVectorView(::windows::core::IUnknown);
impl IBindableVectorView {
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAt)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndexOf)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn First(&self) -> ::windows::core::Result<IBindableIterator> {
        let this = &::windows::core::Interface::cast::<IBindableIterable>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBindableIterator>(result__)
        }
    }
}
impl ::core::convert::From<IBindableVectorView> for ::windows::core::IUnknown {
    fn from(value: IBindableVectorView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindableVectorView> for ::windows::core::IUnknown {
    fn from(value: &IBindableVectorView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindableVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindableVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBindableVectorView> for ::windows::core::IInspectable {
    fn from(value: IBindableVectorView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindableVectorView> for ::windows::core::IInspectable {
    fn from(value: &IBindableVectorView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBindableVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBindableVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IBindableVectorView> for IBindableIterable {
    type Error = ::windows::core::Error;
    fn try_from(value: IBindableVectorView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBindableVectorView> for IBindableIterable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBindableVectorView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBindableIterable> for IBindableVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, IBindableIterable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBindableIterable> for &IBindableVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, IBindableIterable> {
        ::core::convert::TryInto::<IBindableIterable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IBindableVectorView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindableVectorView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindableVectorView {}
impl ::core::fmt::Debug for IBindableVectorView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindableVectorView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IBindableVectorView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{346dd6e7-976e-4bc3-815d-ece243bc0f33}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IBindableVectorView {
    type Vtable = IBindableVectorView_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x346dd6e7_976e_4bc3_815d_ece243bc0f33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableVectorView_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, index: *mut u32, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct INotifyCollectionChanged(::windows::core::IUnknown);
impl INotifyCollectionChanged {
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CollectionChanged<'a, Param0: ::windows::core::IntoParam<'a, NotifyCollectionChangedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CollectionChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCollectionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCollectionChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<INotifyCollectionChanged> for ::windows::core::IUnknown {
    fn from(value: INotifyCollectionChanged) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INotifyCollectionChanged> for ::windows::core::IUnknown {
    fn from(value: &INotifyCollectionChanged) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INotifyCollectionChanged {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INotifyCollectionChanged {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INotifyCollectionChanged> for ::windows::core::IInspectable {
    fn from(value: INotifyCollectionChanged) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INotifyCollectionChanged> for ::windows::core::IInspectable {
    fn from(value: &INotifyCollectionChanged) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INotifyCollectionChanged {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INotifyCollectionChanged {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INotifyCollectionChanged {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INotifyCollectionChanged {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INotifyCollectionChanged {}
impl ::core::fmt::Debug for INotifyCollectionChanged {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INotifyCollectionChanged").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for INotifyCollectionChanged {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{28b167d5-1a31-465b-9b25-d5c3ae686c40}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for INotifyCollectionChanged {
    type Vtable = INotifyCollectionChanged_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28b167d5_1a31_465b_9b25_d5c3ae686c40);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChanged_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CollectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CollectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCollectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCollectionChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotifyCollectionChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INotifyCollectionChangedEventArgs {
    type Vtable = INotifyCollectionChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cf68d33_e3f2_4964_b85e_945b4f7e2f21);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NotifyCollectionChangedAction) -> ::windows::core::HRESULT,
    pub NewItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OldItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NewStartingIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub OldStartingIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotifyCollectionChangedEventArgsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INotifyCollectionChangedEventArgsFactory {
    type Vtable = INotifyCollectionChangedEventArgsFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb30c3e3a_df8d_44a5_9a38_7ac0d08ce63d);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChangedEventArgsFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstanceWithAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: NotifyCollectionChangedAction, newitems: ::windows::core::RawPtr, olditems: ::windows::core::RawPtr, newindex: i32, oldindex: i32, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NotifyCollectionChangedAction(pub i32);
impl NotifyCollectionChangedAction {
    pub const Add: Self = Self(0i32);
    pub const Remove: Self = Self(1i32);
    pub const Replace: Self = Self(2i32);
    pub const Move: Self = Self(3i32);
    pub const Reset: Self = Self(4i32);
}
impl ::core::marker::Copy for NotifyCollectionChangedAction {}
impl ::core::clone::Clone for NotifyCollectionChangedAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NotifyCollectionChangedAction {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NotifyCollectionChangedAction {
    type Abi = Self;
}
impl ::core::fmt::Debug for NotifyCollectionChangedAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotifyCollectionChangedAction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NotifyCollectionChangedAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Interop.NotifyCollectionChangedAction;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct NotifyCollectionChangedEventArgs(::windows::core::IUnknown);
impl NotifyCollectionChangedEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn Action(&self) -> ::windows::core::Result<NotifyCollectionChangedAction> {
        let this = self;
        unsafe {
            let mut result__: NotifyCollectionChangedAction = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Action)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NotifyCollectionChangedAction>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn NewItems(&self) -> ::windows::core::Result<IBindableVector> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NewItems)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBindableVector>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn OldItems(&self) -> ::windows::core::Result<IBindableVector> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OldItems)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBindableVector>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn NewStartingIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NewStartingIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn OldStartingIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OldStartingIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn CreateInstanceWithAllParameters<'a, Param1: ::windows::core::IntoParam<'a, IBindableVector>, Param2: ::windows::core::IntoParam<'a, IBindableVector>>(action: NotifyCollectionChangedAction, newitems: Param1, olditems: Param2, newindex: i32, oldindex: i32) -> ::windows::core::Result<NotifyCollectionChangedEventArgs> {
        Self::INotifyCollectionChangedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithAllParameters)(::core::mem::transmute_copy(this), action, newitems.into_param().abi(), olditems.into_param().abi(), newindex, oldindex, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<NotifyCollectionChangedEventArgs>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn CreateInstanceWithAllParameters_compose<'a, Param1: ::windows::core::IntoParam<'a, IBindableVector>, Param2: ::windows::core::IntoParam<'a, IBindableVector>, T: ::windows::core::Compose>(action: NotifyCollectionChangedAction, newitems: Param1, olditems: Param2, newindex: i32, oldindex: i32, compose: T) -> ::windows::core::Result<NotifyCollectionChangedEventArgs> {
        Self::INotifyCollectionChangedEventArgsFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithAllParameters)(::core::mem::transmute_copy(this), action, newitems.into_param().abi(), olditems.into_param().abi(), newindex, oldindex, ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<NotifyCollectionChangedEventArgs>(result__)
        })
    }
    #[doc(hidden)]
    pub fn INotifyCollectionChangedEventArgsFactory<R, F: FnOnce(&INotifyCollectionChangedEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NotifyCollectionChangedEventArgs, INotifyCollectionChangedEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for NotifyCollectionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NotifyCollectionChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NotifyCollectionChangedEventArgs {}
impl ::core::fmt::Debug for NotifyCollectionChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotifyCollectionChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NotifyCollectionChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Interop.NotifyCollectionChangedEventArgs;{4cf68d33-e3f2-4964-b85e-945b4f7e2f21})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for NotifyCollectionChangedEventArgs {
    type Vtable = INotifyCollectionChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <INotifyCollectionChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NotifyCollectionChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Interop.NotifyCollectionChangedEventArgs";
}
impl ::core::convert::From<NotifyCollectionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: NotifyCollectionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NotifyCollectionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &NotifyCollectionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NotifyCollectionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NotifyCollectionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NotifyCollectionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: NotifyCollectionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NotifyCollectionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &NotifyCollectionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NotifyCollectionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NotifyCollectionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NotifyCollectionChangedEventArgs {}
unsafe impl ::core::marker::Sync for NotifyCollectionChangedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct NotifyCollectionChangedEventHandler(pub ::windows::core::IUnknown);
impl NotifyCollectionChangedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NotifyCollectionChangedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = NotifyCollectionChangedEventHandlerBox::<F> { vtable: &NotifyCollectionChangedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, NotifyCollectionChangedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct NotifyCollectionChangedEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NotifyCollectionChangedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const NotifyCollectionChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NotifyCollectionChangedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> NotifyCollectionChangedEventHandlerBox<F> {
    const VTABLE: NotifyCollectionChangedEventHandler_Vtbl = NotifyCollectionChangedEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<NotifyCollectionChangedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for NotifyCollectionChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NotifyCollectionChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NotifyCollectionChangedEventHandler {}
impl ::core::fmt::Debug for NotifyCollectionChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotifyCollectionChangedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for NotifyCollectionChangedEventHandler {
    type Vtable = NotifyCollectionChangedEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca10b37c_f382_4591_8557_5e24965279b0);
}
unsafe impl ::windows::core::RuntimeType for NotifyCollectionChangedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ca10b37c-f382-4591-8557-5e24965279b0}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct NotifyCollectionChangedEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TypeKind(pub i32);
impl TypeKind {
    pub const Primitive: Self = Self(0i32);
    pub const Metadata: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
}
impl ::core::marker::Copy for TypeKind {}
impl ::core::clone::Clone for TypeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TypeKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TypeKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for TypeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TypeKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TypeKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Interop.TypeKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
pub struct TypeName {
    pub Name: ::windows::core::HSTRING,
    pub Kind: TypeKind,
}
impl ::core::clone::Clone for TypeName {
    fn clone(&self) -> Self {
        Self { Name: self.Name.clone(), Kind: self.Kind }
    }
}
impl ::core::fmt::Debug for TypeName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TypeName").field("Name", &self.Name).field("Kind", &self.Kind).finish()
    }
}
unsafe impl ::windows::core::Abi for TypeName {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows::core::RuntimeType for TypeName {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Interop.TypeName;string;enum(Windows.UI.Xaml.Interop.TypeKind;i4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(from.clone())
    }
}
impl ::core::cmp::PartialEq for TypeName {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Kind == other.Kind
    }
}
impl ::core::cmp::Eq for TypeName {}
impl ::core::default::Default for TypeName {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
