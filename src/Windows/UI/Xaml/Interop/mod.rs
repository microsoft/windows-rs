#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_Xaml_Interop`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BindableVectorChangedEventHandler(::windows::runtime::IUnknown);
impl BindableVectorChangedEventHandler {
    pub fn new<F: FnMut(&::std::option::Option<IBindableObservableVector>, &::std::option::Option<::windows::runtime::IInspectable>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = BindableVectorChangedEventHandler_box::<F> {
            vtable: &BindableVectorChangedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, IBindableObservableVector>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, vector: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), vector.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BindableVectorChangedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({624cd4e1-d007-43b1-9c03-af4d3e6258c4})");
}
unsafe impl ::windows::runtime::Interface for BindableVectorChangedEventHandler {
    type Vtable = BindableVectorChangedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1649202401, 53255, 17329, [156, 3, 175, 77, 62, 98, 88, 196]);
}
#[repr(C)]
#[doc(hidden)]
pub struct BindableVectorChangedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vector: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct BindableVectorChangedEventHandler_box<F: FnMut(&::std::option::Option<IBindableObservableVector>, &::std::option::Option<::windows::runtime::IInspectable>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const BindableVectorChangedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<IBindableObservableVector>, &::std::option::Option<::windows::runtime::IInspectable>) -> ::windows::runtime::Result<()> + 'static> BindableVectorChangedEventHandler_box<F> {
    const VTABLE: BindableVectorChangedEventHandler_abi = BindableVectorChangedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<BindableVectorChangedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, vector: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&vector as *const <IBindableObservableVector as ::windows::runtime::Abi>::Abi as *const <IBindableObservableVector as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Interop`*"]
pub struct IBindableIterable(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBindableIterable {
    type Vtable = IBindableIterable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(57486344, 57129, 16815, [138, 162, 215, 116, 190, 98, 186, 111]);
}
impl IBindableIterable {
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn First(&self) -> ::windows::runtime::Result<IBindableIterator> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IBindableIterator>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBindableIterable {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{036d2c08-df29-41af-8aa2-d774be62ba6f}");
}
impl ::std::convert::From<IBindableIterable> for ::windows::runtime::IUnknown {
    fn from(value: IBindableIterable) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBindableIterable> for ::windows::runtime::IUnknown {
    fn from(value: &IBindableIterable) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBindableIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBindableIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IBindableIterable> for ::windows::runtime::IInspectable {
    fn from(value: IBindableIterable) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBindableIterable> for ::windows::runtime::IInspectable {
    fn from(value: &IBindableIterable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IBindableIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IBindableIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableIterable_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Interop`*"]
pub struct IBindableIterator(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBindableIterator {
    type Vtable = IBindableIterator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1780313095, 1901, 18930, [131, 20, 245, 44, 156, 154, 131, 49]);
}
impl IBindableIterator {
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn Current(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn HasCurrent(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn MoveNext(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBindableIterator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{6a1d6c07-076d-49f2-8314-f52c9c9a8331}");
}
impl ::std::convert::From<IBindableIterator> for ::windows::runtime::IUnknown {
    fn from(value: IBindableIterator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBindableIterator> for ::windows::runtime::IUnknown {
    fn from(value: &IBindableIterator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBindableIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBindableIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IBindableIterator> for ::windows::runtime::IInspectable {
    fn from(value: IBindableIterator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBindableIterator> for ::windows::runtime::IInspectable {
    fn from(value: &IBindableIterator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IBindableIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IBindableIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableIterator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Interop`*"]
pub struct IBindableObservableVector(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBindableObservableVector {
    type Vtable = IBindableObservableVector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4263425334, 32383, 20368, [172, 154, 71, 73, 132, 170, 229, 18]);
}
impl IBindableObservableVector {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Interop`, `Foundation`*"]
    pub fn VectorChanged<'a, Param0: ::windows::runtime::IntoParam<'a, BindableVectorChangedEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Interop`, `Foundation`*"]
    pub fn RemoveVectorChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn First(&self) -> ::windows::runtime::Result<IBindableIterator> {
        let this = &::windows::runtime::Interface::cast::<IBindableIterable>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IBindableIterator>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<IBindableVectorView> {
        let this = &::windows::runtime::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IBindableVectorView>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBindableVector>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBindableObservableVector {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{fe1eb536-7e7f-4f90-ac9a-474984aae512}");
}
impl ::std::convert::From<IBindableObservableVector> for ::windows::runtime::IUnknown {
    fn from(value: IBindableObservableVector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBindableObservableVector> for ::windows::runtime::IUnknown {
    fn from(value: &IBindableObservableVector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBindableObservableVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBindableObservableVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IBindableObservableVector> for ::windows::runtime::IInspectable {
    fn from(value: IBindableObservableVector) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBindableObservableVector> for ::windows::runtime::IInspectable {
    fn from(value: &IBindableObservableVector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IBindableObservableVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IBindableObservableVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<IBindableObservableVector> for IBindableIterable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IBindableObservableVector) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IBindableObservableVector> for IBindableIterable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IBindableObservableVector) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBindableIterable> for IBindableObservableVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBindableIterable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBindableIterable> for &IBindableObservableVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBindableIterable> {
        ::std::convert::TryInto::<IBindableIterable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IBindableObservableVector> for IBindableVector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IBindableObservableVector) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IBindableObservableVector> for IBindableVector {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IBindableObservableVector) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBindableVector> for IBindableObservableVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBindableVector> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBindableVector> for &IBindableObservableVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBindableVector> {
        ::std::convert::TryInto::<IBindableVector>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableObservableVector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Interop`*"]
pub struct IBindableVector(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBindableVector {
    type Vtable = IBindableVector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(960358366, 28624, 19469, [187, 113, 71, 36, 74, 17, 62, 147]);
}
impl IBindableVector {
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<IBindableVectorView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IBindableVectorView>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn First(&self) -> ::windows::runtime::Result<IBindableIterator> {
        let this = &::windows::runtime::Interface::cast::<IBindableIterable>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IBindableIterator>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBindableVector {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{393de7de-6fd0-4c0d-bb71-47244a113e93}");
}
impl ::std::convert::From<IBindableVector> for ::windows::runtime::IUnknown {
    fn from(value: IBindableVector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBindableVector> for ::windows::runtime::IUnknown {
    fn from(value: &IBindableVector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBindableVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBindableVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IBindableVector> for ::windows::runtime::IInspectable {
    fn from(value: IBindableVector) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBindableVector> for ::windows::runtime::IInspectable {
    fn from(value: &IBindableVector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IBindableVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IBindableVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<IBindableVector> for IBindableIterable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IBindableVector) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IBindableVector> for IBindableIterable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IBindableVector) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBindableIterable> for IBindableVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBindableIterable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBindableIterable> for &IBindableVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBindableIterable> {
        ::std::convert::TryInto::<IBindableIterable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableVector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, index: *mut u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Interop`*"]
pub struct IBindableVectorView(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBindableVectorView {
    type Vtable = IBindableVectorView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(879613671, 38766, 19395, [129, 93, 236, 226, 67, 188, 15, 51]);
}
impl IBindableVectorView {
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn First(&self) -> ::windows::runtime::Result<IBindableIterator> {
        let this = &::windows::runtime::Interface::cast::<IBindableIterable>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IBindableIterator>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBindableVectorView {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{346dd6e7-976e-4bc3-815d-ece243bc0f33}");
}
impl ::std::convert::From<IBindableVectorView> for ::windows::runtime::IUnknown {
    fn from(value: IBindableVectorView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBindableVectorView> for ::windows::runtime::IUnknown {
    fn from(value: &IBindableVectorView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBindableVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBindableVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IBindableVectorView> for ::windows::runtime::IInspectable {
    fn from(value: IBindableVectorView) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBindableVectorView> for ::windows::runtime::IInspectable {
    fn from(value: &IBindableVectorView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IBindableVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IBindableVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<IBindableVectorView> for IBindableIterable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IBindableVectorView) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IBindableVectorView> for IBindableIterable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IBindableVectorView) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBindableIterable> for IBindableVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBindableIterable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBindableIterable> for &IBindableVectorView {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBindableIterable> {
        ::std::convert::TryInto::<IBindableIterable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableVectorView_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, index: *mut u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Interop`*"]
pub struct INotifyCollectionChanged(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INotifyCollectionChanged {
    type Vtable = INotifyCollectionChanged_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(682715093, 6705, 18011, [155, 37, 213, 195, 174, 104, 108, 64]);
}
impl INotifyCollectionChanged {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Interop`, `Foundation`*"]
    pub fn CollectionChanged<'a, Param0: ::windows::runtime::IntoParam<'a, NotifyCollectionChangedEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Interop`, `Foundation`*"]
    pub fn RemoveCollectionChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INotifyCollectionChanged {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{28b167d5-1a31-465b-9b25-d5c3ae686c40}");
}
impl ::std::convert::From<INotifyCollectionChanged> for ::windows::runtime::IUnknown {
    fn from(value: INotifyCollectionChanged) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INotifyCollectionChanged> for ::windows::runtime::IUnknown {
    fn from(value: &INotifyCollectionChanged) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for INotifyCollectionChanged {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &INotifyCollectionChanged {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<INotifyCollectionChanged> for ::windows::runtime::IInspectable {
    fn from(value: INotifyCollectionChanged) -> Self {
        value.0
    }
}
impl ::std::convert::From<&INotifyCollectionChanged> for ::windows::runtime::IInspectable {
    fn from(value: &INotifyCollectionChanged) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for INotifyCollectionChanged {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a INotifyCollectionChanged {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChanged_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INotifyCollectionChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INotifyCollectionChangedEventArgs {
    type Vtable = INotifyCollectionChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1291226419, 58354, 18788, [184, 94, 148, 91, 79, 126, 47, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NotifyCollectionChangedAction) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INotifyCollectionChangedEventArgsFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INotifyCollectionChangedEventArgsFactory {
    type Vtable = INotifyCollectionChangedEventArgsFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3003924026, 57229, 17573, [154, 56, 122, 192, 208, 140, 230, 61]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChangedEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, action: NotifyCollectionChangedAction, newitems: ::windows::runtime::RawPtr, olditems: ::windows::runtime::RawPtr, newindex: i32, oldindex: i32, baseinterface: ::windows::runtime::RawPtr, innerinterface: *mut ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Xaml_Interop`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NotifyCollectionChangedAction(pub i32);
impl NotifyCollectionChangedAction {
    pub const Add: NotifyCollectionChangedAction = NotifyCollectionChangedAction(0i32);
    pub const Remove: NotifyCollectionChangedAction = NotifyCollectionChangedAction(1i32);
    pub const Replace: NotifyCollectionChangedAction = NotifyCollectionChangedAction(2i32);
    pub const Move: NotifyCollectionChangedAction = NotifyCollectionChangedAction(3i32);
    pub const Reset: NotifyCollectionChangedAction = NotifyCollectionChangedAction(4i32);
}
impl ::std::convert::From<i32> for NotifyCollectionChangedAction {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NotifyCollectionChangedAction {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NotifyCollectionChangedAction {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Interop.NotifyCollectionChangedAction;i4)");
}
impl ::windows::runtime::DefaultType for NotifyCollectionChangedAction {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Interop`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NotifyCollectionChangedEventArgs(::windows::runtime::IInspectable);
impl NotifyCollectionChangedEventArgs {
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn Action(&self) -> ::windows::runtime::Result<NotifyCollectionChangedAction> {
        let this = self;
        unsafe {
            let mut result__: NotifyCollectionChangedAction = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NotifyCollectionChangedAction>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn NewItems(&self) -> ::windows::runtime::Result<IBindableVector> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IBindableVector>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn OldItems(&self) -> ::windows::runtime::Result<IBindableVector> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IBindableVector>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn NewStartingIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn OldStartingIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn CreateInstanceWithAllParameters<'a, Param1: ::windows::runtime::IntoParam<'a, IBindableVector>, Param2: ::windows::runtime::IntoParam<'a, IBindableVector>>(action: NotifyCollectionChangedAction, newitems: Param1, olditems: Param2, newindex: i32, oldindex: i32) -> ::windows::runtime::Result<NotifyCollectionChangedEventArgs> {
        Self::INotifyCollectionChangedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), action, newitems.into_param().abi(), olditems.into_param().abi(), newindex, oldindex, ::std::ptr::null_mut(), &mut ::std::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<NotifyCollectionChangedEventArgs>(result__)
        })
    }
    pub fn INotifyCollectionChangedEventArgsFactory<R, F: FnOnce(&INotifyCollectionChangedEventArgsFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NotifyCollectionChangedEventArgs, INotifyCollectionChangedEventArgsFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NotifyCollectionChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Interop.NotifyCollectionChangedEventArgs;{4cf68d33-e3f2-4964-b85e-945b4f7e2f21})");
}
unsafe impl ::windows::runtime::Interface for NotifyCollectionChangedEventArgs {
    type Vtable = INotifyCollectionChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1291226419, 58354, 18788, [184, 94, 148, 91, 79, 126, 47, 33]);
}
impl ::windows::runtime::RuntimeName for NotifyCollectionChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Interop.NotifyCollectionChangedEventArgs";
}
impl ::std::convert::From<NotifyCollectionChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: NotifyCollectionChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NotifyCollectionChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &NotifyCollectionChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NotifyCollectionChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NotifyCollectionChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NotifyCollectionChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: NotifyCollectionChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NotifyCollectionChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &NotifyCollectionChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NotifyCollectionChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NotifyCollectionChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for NotifyCollectionChangedEventArgs {}
unsafe impl ::std::marker::Sync for NotifyCollectionChangedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Interop`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NotifyCollectionChangedEventHandler(::windows::runtime::IUnknown);
impl NotifyCollectionChangedEventHandler {
    pub fn new<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NotifyCollectionChangedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = NotifyCollectionChangedEventHandler_box::<F> {
            vtable: &NotifyCollectionChangedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Interop`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, NotifyCollectionChangedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NotifyCollectionChangedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({ca10b37c-f382-4591-8557-5e24965279b0})");
}
unsafe impl ::windows::runtime::Interface for NotifyCollectionChangedEventHandler {
    type Vtable = NotifyCollectionChangedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3390092156, 62338, 17809, [133, 87, 94, 36, 150, 82, 121, 176]);
}
#[repr(C)]
#[doc(hidden)]
pub struct NotifyCollectionChangedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct NotifyCollectionChangedEventHandler_box<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NotifyCollectionChangedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const NotifyCollectionChangedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NotifyCollectionChangedEventArgs>) -> ::windows::runtime::Result<()> + 'static> NotifyCollectionChangedEventHandler_box<F> {
    const VTABLE: NotifyCollectionChangedEventHandler_abi = NotifyCollectionChangedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<NotifyCollectionChangedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <NotifyCollectionChangedEventArgs as ::windows::runtime::Abi>::Abi as *const <NotifyCollectionChangedEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `UI_Xaml_Interop`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TypeKind(pub i32);
impl TypeKind {
    pub const Primitive: TypeKind = TypeKind(0i32);
    pub const Metadata: TypeKind = TypeKind(1i32);
    pub const Custom: TypeKind = TypeKind(2i32);
}
impl ::std::convert::From<i32> for TypeKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TypeKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TypeKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Interop.TypeKind;i4)");
}
impl ::windows::runtime::DefaultType for TypeKind {
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `UI_Xaml_Interop`*"]
pub struct TypeName {
    pub Name: ::windows::runtime::HSTRING,
    pub Kind: TypeKind,
}
impl TypeName {}
impl ::std::default::Default for TypeName {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TypeName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TypeName").field("Name", &self.Name).field("Kind", &self.Kind).finish()
    }
}
impl ::std::cmp::PartialEq for TypeName {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Kind == other.Kind
    }
}
impl ::std::cmp::Eq for TypeName {}
unsafe impl ::windows::runtime::Abi for TypeName {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows::runtime::RuntimeType for TypeName {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Interop.TypeName;string;enum(Windows.UI.Xaml.Interop.TypeKind;i4))");
}
impl ::windows::runtime::DefaultType for TypeName {
    type DefaultType = Self;
}
