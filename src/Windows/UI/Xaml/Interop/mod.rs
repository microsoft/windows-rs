#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BindableVectorChangedEventHandler(::windows::core::IUnknown);
impl BindableVectorChangedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<IBindableObservableVector>, &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = BindableVectorChangedEventHandler_box::<F> {
            vtable: &BindableVectorChangedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, IBindableObservableVector>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, vector: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), vector.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for BindableVectorChangedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({624cd4e1-d007-43b1-9c03-af4d3e6258c4})");
}
unsafe impl ::windows::core::Interface for BindableVectorChangedEventHandler {
    type Vtable = BindableVectorChangedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x624cd4e1_d007_43b1_9c03_af4d3e6258c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct BindableVectorChangedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vector: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct BindableVectorChangedEventHandler_box<F: FnMut(&::core::option::Option<IBindableObservableVector>, &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const BindableVectorChangedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<IBindableObservableVector>, &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> + 'static> BindableVectorChangedEventHandler_box<F> {
    const VTABLE: BindableVectorChangedEventHandler_abi = BindableVectorChangedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<BindableVectorChangedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, vector: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&vector as *const <IBindableObservableVector as ::windows::core::Abi>::Abi as *const <IBindableObservableVector as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBindableIterable(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBindableIterable {
    type Vtable = IBindableIterable_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x036d2c08_df29_41af_8aa2_d774be62ba6f);
}
impl IBindableIterable {
    pub fn First(&self) -> ::windows::core::Result<IBindableIterator> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBindableIterator>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IBindableIterable {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{036d2c08-df29-41af-8aa2-d774be62ba6f}");
}
impl ::core::convert::From<IBindableIterable> for ::windows::core::IUnknown {
    fn from(value: IBindableIterable) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IBindableIterable> for ::windows::core::IUnknown {
    fn from(value: &IBindableIterable) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindableIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindableIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IBindableIterable> for ::windows::core::IInspectable {
    fn from(value: IBindableIterable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBindableIterable> for ::windows::core::IInspectable {
    fn from(value: &IBindableIterable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBindableIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBindableIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableIterable_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBindableIterator(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBindableIterator {
    type Vtable = IBindableIterator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a1d6c07_076d_49f2_8314_f52c9c9a8331);
}
impl IBindableIterator {
    pub fn Current(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn HasCurrent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn MoveNext(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IBindableIterator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6a1d6c07-076d-49f2-8314-f52c9c9a8331}");
}
impl ::core::convert::From<IBindableIterator> for ::windows::core::IUnknown {
    fn from(value: IBindableIterator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IBindableIterator> for ::windows::core::IUnknown {
    fn from(value: &IBindableIterator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindableIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindableIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IBindableIterator> for ::windows::core::IInspectable {
    fn from(value: IBindableIterator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBindableIterator> for ::windows::core::IInspectable {
    fn from(value: &IBindableIterator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBindableIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBindableIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableIterator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBindableObservableVector(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBindableObservableVector {
    type Vtable = IBindableObservableVector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe1eb536_7e7f_4f90_ac9a_474984aae512);
}
impl IBindableObservableVector {
    #[cfg(feature = "Foundation")]
    pub fn VectorChanged<'a, Param0: ::windows::core::IntoParam<'a, BindableVectorChangedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVectorChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn First(&self) -> ::windows::core::Result<IBindableIterator> {
        let this = &::windows::core::Interface::cast::<IBindableIterable>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBindableIterator>(result__)
        }
    }
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn GetView(&self) -> ::windows::core::Result<IBindableVectorView> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBindableVectorView>(result__)
        }
    }
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IBindableObservableVector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fe1eb536-7e7f-4f90-ac9a-474984aae512}");
}
impl ::core::convert::From<IBindableObservableVector> for ::windows::core::IUnknown {
    fn from(value: IBindableObservableVector) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IBindableObservableVector> for ::windows::core::IUnknown {
    fn from(value: &IBindableObservableVector) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindableObservableVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindableObservableVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IBindableObservableVector> for ::windows::core::IInspectable {
    fn from(value: IBindableObservableVector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBindableObservableVector> for ::windows::core::IInspectable {
    fn from(value: &IBindableObservableVector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBindableObservableVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBindableObservableVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(C)]
#[doc(hidden)]
pub struct IBindableObservableVector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBindableVector(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBindableVector {
    type Vtable = IBindableVector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x393de7de_6fd0_4c0d_bb71_47244a113e93);
}
impl IBindableVector {
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn GetView(&self) -> ::windows::core::Result<IBindableVectorView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBindableVectorView>(result__)
        }
    }
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn First(&self) -> ::windows::core::Result<IBindableIterator> {
        let this = &::windows::core::Interface::cast::<IBindableIterable>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBindableIterator>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IBindableVector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{393de7de-6fd0-4c0d-bb71-47244a113e93}");
}
impl ::core::convert::From<IBindableVector> for ::windows::core::IUnknown {
    fn from(value: IBindableVector) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IBindableVector> for ::windows::core::IUnknown {
    fn from(value: &IBindableVector) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindableVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindableVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IBindableVector> for ::windows::core::IInspectable {
    fn from(value: IBindableVector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBindableVector> for ::windows::core::IInspectable {
    fn from(value: &IBindableVector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBindableVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBindableVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(C)]
#[doc(hidden)]
pub struct IBindableVector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, index: *mut u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBindableVectorView(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBindableVectorView {
    type Vtable = IBindableVectorView_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x346dd6e7_976e_4bc3_815d_ece243bc0f33);
}
impl IBindableVectorView {
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn First(&self) -> ::windows::core::Result<IBindableIterator> {
        let this = &::windows::core::Interface::cast::<IBindableIterable>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBindableIterator>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IBindableVectorView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{346dd6e7-976e-4bc3-815d-ece243bc0f33}");
}
impl ::core::convert::From<IBindableVectorView> for ::windows::core::IUnknown {
    fn from(value: IBindableVectorView) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IBindableVectorView> for ::windows::core::IUnknown {
    fn from(value: &IBindableVectorView) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindableVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindableVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IBindableVectorView> for ::windows::core::IInspectable {
    fn from(value: IBindableVectorView) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBindableVectorView> for ::windows::core::IInspectable {
    fn from(value: &IBindableVectorView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBindableVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBindableVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(C)]
#[doc(hidden)]
pub struct IBindableVectorView_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, index: *mut u32, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INotifyCollectionChanged(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INotifyCollectionChanged {
    type Vtable = INotifyCollectionChanged_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28b167d5_1a31_465b_9b25_d5c3ae686c40);
}
impl INotifyCollectionChanged {
    #[cfg(feature = "Foundation")]
    pub fn CollectionChanged<'a, Param0: ::windows::core::IntoParam<'a, NotifyCollectionChangedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCollectionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for INotifyCollectionChanged {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{28b167d5-1a31-465b-9b25-d5c3ae686c40}");
}
impl ::core::convert::From<INotifyCollectionChanged> for ::windows::core::IUnknown {
    fn from(value: INotifyCollectionChanged) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INotifyCollectionChanged> for ::windows::core::IUnknown {
    fn from(value: &INotifyCollectionChanged) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INotifyCollectionChanged {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INotifyCollectionChanged {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INotifyCollectionChanged> for ::windows::core::IInspectable {
    fn from(value: INotifyCollectionChanged) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INotifyCollectionChanged> for ::windows::core::IInspectable {
    fn from(value: &INotifyCollectionChanged) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INotifyCollectionChanged {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INotifyCollectionChanged {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChanged_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INotifyCollectionChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INotifyCollectionChangedEventArgs {
    type Vtable = INotifyCollectionChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cf68d33_e3f2_4964_b85e_945b4f7e2f21);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut NotifyCollectionChangedAction) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INotifyCollectionChangedEventArgsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INotifyCollectionChangedEventArgsFactory {
    type Vtable = INotifyCollectionChangedEventArgsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb30c3e3a_df8d_44a5_9a38_7ac0d08ce63d);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChangedEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, action: NotifyCollectionChangedAction, newitems: ::windows::core::RawPtr, olditems: ::windows::core::RawPtr, newindex: i32, oldindex: i32, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NotifyCollectionChangedAction(pub i32);
impl NotifyCollectionChangedAction {
    pub const Add: NotifyCollectionChangedAction = NotifyCollectionChangedAction(0i32);
    pub const Remove: NotifyCollectionChangedAction = NotifyCollectionChangedAction(1i32);
    pub const Replace: NotifyCollectionChangedAction = NotifyCollectionChangedAction(2i32);
    pub const Move: NotifyCollectionChangedAction = NotifyCollectionChangedAction(3i32);
    pub const Reset: NotifyCollectionChangedAction = NotifyCollectionChangedAction(4i32);
}
impl ::core::convert::From<i32> for NotifyCollectionChangedAction {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NotifyCollectionChangedAction {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NotifyCollectionChangedAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Interop.NotifyCollectionChangedAction;i4)");
}
impl ::windows::core::DefaultType for NotifyCollectionChangedAction {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NotifyCollectionChangedEventArgs(pub ::windows::core::IInspectable);
impl NotifyCollectionChangedEventArgs {
    pub fn Action(&self) -> ::windows::core::Result<NotifyCollectionChangedAction> {
        let this = self;
        unsafe {
            let mut result__: NotifyCollectionChangedAction = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NotifyCollectionChangedAction>(result__)
        }
    }
    pub fn NewItems(&self) -> ::windows::core::Result<IBindableVector> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBindableVector>(result__)
        }
    }
    pub fn OldItems(&self) -> ::windows::core::Result<IBindableVector> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IBindableVector>(result__)
        }
    }
    pub fn NewStartingIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn OldStartingIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn CreateInstanceWithAllParameters<'a, Param1: ::windows::core::IntoParam<'a, IBindableVector>, Param2: ::windows::core::IntoParam<'a, IBindableVector>>(action: NotifyCollectionChangedAction, newitems: Param1, olditems: Param2, newindex: i32, oldindex: i32) -> ::windows::core::Result<NotifyCollectionChangedEventArgs> {
        Self::INotifyCollectionChangedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), action, newitems.into_param().abi(), olditems.into_param().abi(), newindex, oldindex, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<NotifyCollectionChangedEventArgs>(result__)
        })
    }
    pub fn INotifyCollectionChangedEventArgsFactory<R, F: FnOnce(&INotifyCollectionChangedEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NotifyCollectionChangedEventArgs, INotifyCollectionChangedEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for NotifyCollectionChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Interop.NotifyCollectionChangedEventArgs;{4cf68d33-e3f2-4964-b85e-945b4f7e2f21})");
}
unsafe impl ::windows::core::Interface for NotifyCollectionChangedEventArgs {
    type Vtable = INotifyCollectionChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cf68d33_e3f2_4964_b85e_945b4f7e2f21);
}
impl ::windows::core::RuntimeName for NotifyCollectionChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Interop.NotifyCollectionChangedEventArgs";
}
impl ::core::convert::From<NotifyCollectionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: NotifyCollectionChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NotifyCollectionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &NotifyCollectionChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NotifyCollectionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NotifyCollectionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NotifyCollectionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: NotifyCollectionChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NotifyCollectionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &NotifyCollectionChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NotifyCollectionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NotifyCollectionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for NotifyCollectionChangedEventArgs {}
unsafe impl ::core::marker::Sync for NotifyCollectionChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NotifyCollectionChangedEventHandler(::windows::core::IUnknown);
impl NotifyCollectionChangedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NotifyCollectionChangedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = NotifyCollectionChangedEventHandler_box::<F> {
            vtable: &NotifyCollectionChangedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, NotifyCollectionChangedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for NotifyCollectionChangedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({ca10b37c-f382-4591-8557-5e24965279b0})");
}
unsafe impl ::windows::core::Interface for NotifyCollectionChangedEventHandler {
    type Vtable = NotifyCollectionChangedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca10b37c_f382_4591_8557_5e24965279b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct NotifyCollectionChangedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct NotifyCollectionChangedEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NotifyCollectionChangedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const NotifyCollectionChangedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NotifyCollectionChangedEventArgs>) -> ::windows::core::Result<()> + 'static> NotifyCollectionChangedEventHandler_box<F> {
    const VTABLE: NotifyCollectionChangedEventHandler_abi = NotifyCollectionChangedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<NotifyCollectionChangedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <NotifyCollectionChangedEventArgs as ::windows::core::Abi>::Abi as *const <NotifyCollectionChangedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TypeKind(pub i32);
impl TypeKind {
    pub const Primitive: TypeKind = TypeKind(0i32);
    pub const Metadata: TypeKind = TypeKind(1i32);
    pub const Custom: TypeKind = TypeKind(2i32);
}
impl ::core::convert::From<i32> for TypeKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TypeKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TypeKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Interop.TypeKind;i4)");
}
impl ::windows::core::DefaultType for TypeKind {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
pub struct TypeName {
    pub Name: ::windows::core::HSTRING,
    pub Kind: TypeKind,
}
impl TypeName {}
impl ::core::default::Default for TypeName {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TypeName {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TypeName").field("Name", &self.Name).field("Kind", &self.Kind).finish()
    }
}
impl ::core::cmp::PartialEq for TypeName {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Kind == other.Kind
    }
}
impl ::core::cmp::Eq for TypeName {}
unsafe impl ::windows::core::Abi for TypeName {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows::core::RuntimeType for TypeName {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Interop.TypeName;string;enum(Windows.UI.Xaml.Interop.TypeKind;i4))");
}
impl ::windows::core::DefaultType for TypeName {
    type DefaultType = Self;
}
