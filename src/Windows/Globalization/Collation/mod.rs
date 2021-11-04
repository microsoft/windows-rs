#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Globalization_Collation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CharacterGrouping(pub ::windows::runtime::IInspectable);
impl CharacterGrouping {
    #[doc = "*Required features: `Globalization_Collation`*"]
    pub fn First(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Collation`*"]
    pub fn Label(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CharacterGrouping {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Globalization.Collation.CharacterGrouping;{fae761bb-805d-4bb0-95bb-c1f7c3e8eb8e})");
}
unsafe impl ::windows::runtime::Interface for CharacterGrouping {
    type Vtable = ICharacterGrouping_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4209467835, 32861, 19376, [149, 187, 193, 247, 195, 232, 235, 142]);
}
impl ::windows::runtime::RuntimeName for CharacterGrouping {
    const NAME: &'static str = "Windows.Globalization.Collation.CharacterGrouping";
}
impl ::std::convert::From<CharacterGrouping> for ::windows::runtime::IUnknown {
    fn from(value: CharacterGrouping) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CharacterGrouping> for ::windows::runtime::IUnknown {
    fn from(value: &CharacterGrouping) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CharacterGrouping {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CharacterGrouping {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CharacterGrouping> for ::windows::runtime::IInspectable {
    fn from(value: CharacterGrouping) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CharacterGrouping> for ::windows::runtime::IInspectable {
    fn from(value: &CharacterGrouping) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CharacterGrouping {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CharacterGrouping {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CharacterGrouping {}
unsafe impl ::std::marker::Sync for CharacterGrouping {}
#[doc = "*Required features: `Globalization_Collation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CharacterGroupings(pub ::windows::runtime::IInspectable);
impl CharacterGroupings {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CharacterGroupings, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Globalization_Collation`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_Collation`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterator<CharacterGrouping>> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IIterable<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<CharacterGrouping>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_Collation`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<CharacterGrouping> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<CharacterGrouping>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_Collation`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_Collation`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, CharacterGrouping>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_Collation`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<CharacterGrouping as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Collation`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(language: Param0) -> ::windows::runtime::Result<CharacterGroupings> {
        Self::ICharacterGroupingsFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), language.into_param().abi(), &mut result__).from_abi::<CharacterGroupings>(result__)
        })
    }
    pub fn ICharacterGroupingsFactory<R, F: FnOnce(&ICharacterGroupingsFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CharacterGroupings, ICharacterGroupingsFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CharacterGroupings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Globalization.Collation.CharacterGroupings;{b8d20a75-d4cf-4055-80e5-ce169c226496})");
}
unsafe impl ::windows::runtime::Interface for CharacterGroupings {
    type Vtable = ICharacterGroupings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3100772981, 54479, 16469, [128, 229, 206, 22, 156, 34, 100, 150]);
}
impl ::windows::runtime::RuntimeName for CharacterGroupings {
    const NAME: &'static str = "Windows.Globalization.Collation.CharacterGroupings";
}
impl ::std::convert::From<CharacterGroupings> for ::windows::runtime::IUnknown {
    fn from(value: CharacterGroupings) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CharacterGroupings> for ::windows::runtime::IUnknown {
    fn from(value: &CharacterGroupings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CharacterGroupings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CharacterGroupings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CharacterGroupings> for ::windows::runtime::IInspectable {
    fn from(value: CharacterGroupings) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CharacterGroupings> for ::windows::runtime::IInspectable {
    fn from(value: &CharacterGroupings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CharacterGroupings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CharacterGroupings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<CharacterGroupings> for super::super::Foundation::Collections::IIterable<CharacterGrouping> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CharacterGroupings) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&CharacterGroupings> for super::super::Foundation::Collections::IIterable<CharacterGrouping> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CharacterGroupings) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<CharacterGrouping>> for CharacterGroupings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IIterable<CharacterGrouping>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<CharacterGrouping>> for &CharacterGroupings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IIterable<CharacterGrouping>> {
        ::std::convert::TryInto::<super::super::Foundation::Collections::IIterable<CharacterGrouping>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<CharacterGroupings> for super::super::Foundation::Collections::IVectorView<CharacterGrouping> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CharacterGroupings) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&CharacterGroupings> for super::super::Foundation::Collections::IVectorView<CharacterGrouping> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CharacterGroupings) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IVectorView<CharacterGrouping>> for CharacterGroupings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IVectorView<CharacterGrouping>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IVectorView<CharacterGrouping>> for &CharacterGroupings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IVectorView<CharacterGrouping>> {
        ::std::convert::TryInto::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for CharacterGroupings {}
unsafe impl ::std::marker::Sync for CharacterGroupings {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for CharacterGroupings {
    type Item = CharacterGrouping;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &CharacterGroupings {
    type Item = CharacterGrouping;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICharacterGrouping(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICharacterGrouping {
    type Vtable = ICharacterGrouping_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4209467835, 32861, 19376, [149, 187, 193, 247, 195, 232, 235, 142]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterGrouping_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICharacterGroupings(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICharacterGroupings {
    type Vtable = ICharacterGroupings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3100772981, 54479, 16469, [128, 229, 206, 22, 156, 34, 100, 150]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterGroupings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICharacterGroupingsFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICharacterGroupingsFactory {
    type Vtable = ICharacterGroupingsFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2582290393, 34925, 17409, [159, 152, 105, 200, 45, 76, 47, 120]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterGroupingsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
