#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Globalization_Collation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CharacterGrouping(pub ::windows::core::IInspectable);
impl CharacterGrouping {
    #[doc = "*Required features: `Globalization_Collation`*"]
    pub fn First(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Collation`*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CharacterGrouping {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.Collation.CharacterGrouping;{fae761bb-805d-4bb0-95bb-c1f7c3e8eb8e})");
}
unsafe impl ::windows::core::Interface for CharacterGrouping {
    type Vtable = ICharacterGrouping_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfae761bb_805d_4bb0_95bb_c1f7c3e8eb8e);
}
impl ::windows::core::RuntimeName for CharacterGrouping {
    const NAME: &'static str = "Windows.Globalization.Collation.CharacterGrouping";
}
impl ::core::convert::From<CharacterGrouping> for ::windows::core::IUnknown {
    fn from(value: CharacterGrouping) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CharacterGrouping> for ::windows::core::IUnknown {
    fn from(value: &CharacterGrouping) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CharacterGrouping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CharacterGrouping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CharacterGrouping> for ::windows::core::IInspectable {
    fn from(value: CharacterGrouping) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CharacterGrouping> for ::windows::core::IInspectable {
    fn from(value: &CharacterGrouping) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CharacterGrouping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CharacterGrouping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CharacterGrouping {}
unsafe impl ::core::marker::Sync for CharacterGrouping {}
#[doc = "*Required features: `Globalization_Collation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CharacterGroupings(pub ::windows::core::IInspectable);
impl CharacterGroupings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CharacterGroupings, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Globalization_Collation`*"]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_Collation`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<CharacterGrouping>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<CharacterGrouping>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_Collation`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<CharacterGrouping> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<CharacterGrouping>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_Collation`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_Collation`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, CharacterGrouping>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_Collation`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<CharacterGrouping as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Collation`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(language: Param0) -> ::windows::core::Result<CharacterGroupings> {
        Self::ICharacterGroupingsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), language.into_param().abi(), &mut result__).from_abi::<CharacterGroupings>(result__)
        })
    }
    pub fn ICharacterGroupingsFactory<R, F: FnOnce(&ICharacterGroupingsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CharacterGroupings, ICharacterGroupingsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CharacterGroupings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.Collation.CharacterGroupings;{b8d20a75-d4cf-4055-80e5-ce169c226496})");
}
unsafe impl ::windows::core::Interface for CharacterGroupings {
    type Vtable = ICharacterGroupings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8d20a75_d4cf_4055_80e5_ce169c226496);
}
impl ::windows::core::RuntimeName for CharacterGroupings {
    const NAME: &'static str = "Windows.Globalization.Collation.CharacterGroupings";
}
impl ::core::convert::From<CharacterGroupings> for ::windows::core::IUnknown {
    fn from(value: CharacterGroupings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CharacterGroupings> for ::windows::core::IUnknown {
    fn from(value: &CharacterGroupings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CharacterGroupings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CharacterGroupings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CharacterGroupings> for ::windows::core::IInspectable {
    fn from(value: CharacterGroupings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CharacterGroupings> for ::windows::core::IInspectable {
    fn from(value: &CharacterGroupings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CharacterGroupings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CharacterGroupings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<CharacterGroupings> for super::super::Foundation::Collections::IIterable<CharacterGrouping> {
    type Error = ::windows::core::Error;
    fn try_from(value: CharacterGroupings) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&CharacterGroupings> for super::super::Foundation::Collections::IIterable<CharacterGrouping> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CharacterGroupings) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<CharacterGrouping>> for CharacterGroupings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<CharacterGrouping>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<CharacterGrouping>> for &CharacterGroupings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<CharacterGrouping>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<CharacterGrouping>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<CharacterGroupings> for super::super::Foundation::Collections::IVectorView<CharacterGrouping> {
    type Error = ::windows::core::Error;
    fn try_from(value: CharacterGroupings) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&CharacterGroupings> for super::super::Foundation::Collections::IVectorView<CharacterGrouping> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CharacterGroupings) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<CharacterGrouping>> for CharacterGroupings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVectorView<CharacterGrouping>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<CharacterGrouping>> for &CharacterGroupings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVectorView<CharacterGrouping>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CharacterGroupings {}
unsafe impl ::core::marker::Sync for CharacterGroupings {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for CharacterGroupings {
    type Item = CharacterGrouping;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &CharacterGroupings {
    type Item = CharacterGrouping;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICharacterGrouping(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICharacterGrouping {
    type Vtable = ICharacterGrouping_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfae761bb_805d_4bb0_95bb_c1f7c3e8eb8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterGrouping_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICharacterGroupings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICharacterGroupings {
    type Vtable = ICharacterGroupings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8d20a75_d4cf_4055_80e5_ce169c226496);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterGroupings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICharacterGroupingsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICharacterGroupingsFactory {
    type Vtable = ICharacterGroupingsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99ea9fd9_886d_4401_9f98_69c82d4c2f78);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterGroupingsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
