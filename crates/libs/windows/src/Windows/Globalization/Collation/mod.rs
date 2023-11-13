#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICharacterGrouping(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICharacterGrouping {
    type Vtable = ICharacterGrouping_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICharacterGrouping {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfae761bb_805d_4bb0_95bb_c1f7c3e8eb8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterGrouping_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub First: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICharacterGroupings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICharacterGroupings {
    type Vtable = ICharacterGroupings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICharacterGroupings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8d20a75_d4cf_4055_80e5_ce169c226496);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterGroupings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Lookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICharacterGroupingsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICharacterGroupingsFactory {
    type Vtable = ICharacterGroupingsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICharacterGroupingsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99ea9fd9_886d_4401_9f98_69c82d4c2f78);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterGroupingsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CharacterGrouping(::windows_core::IUnknown);
impl CharacterGrouping {
    pub fn First(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CharacterGrouping {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.Collation.CharacterGrouping;{fae761bb-805d-4bb0-95bb-c1f7c3e8eb8e})");
}
unsafe impl ::windows_core::Interface for CharacterGrouping {
    type Vtable = ICharacterGrouping_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CharacterGrouping {
    const IID: ::windows_core::GUID = <ICharacterGrouping as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CharacterGrouping {
    const NAME: &'static str = "Windows.Globalization.Collation.CharacterGrouping";
}
::windows_core::imp::interface_hierarchy!(CharacterGrouping, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CharacterGrouping {}
unsafe impl ::core::marker::Sync for CharacterGrouping {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CharacterGroupings(::windows_core::IUnknown);
impl CharacterGroupings {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CharacterGroupings, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Lookup(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(language: &::windows_core::HSTRING) -> ::windows_core::Result<CharacterGroupings> {
        Self::ICharacterGroupingsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(language), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IIterator<CharacterGrouping>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<CharacterGrouping> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<CharacterGrouping>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<CharacterGrouping>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ICharacterGroupingsFactory<R, F: FnOnce(&ICharacterGroupingsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CharacterGroupings, ICharacterGroupingsFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for CharacterGroupings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.Collation.CharacterGroupings;{b8d20a75-d4cf-4055-80e5-ce169c226496})");
}
unsafe impl ::windows_core::Interface for CharacterGroupings {
    type Vtable = ICharacterGroupings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CharacterGroupings {
    const IID: ::windows_core::GUID = <ICharacterGroupings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CharacterGroupings {
    const NAME: &'static str = "Windows.Globalization.Collation.CharacterGroupings";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for CharacterGroupings {
    type Item = CharacterGrouping;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &CharacterGroupings {
    type Item = CharacterGrouping;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::windows_core::ComInterface::cast(self).ok())
    }
}
::windows_core::imp::interface_hierarchy!(CharacterGroupings, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IIterable<CharacterGrouping>> for CharacterGroupings {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IVectorView<CharacterGrouping>> for CharacterGroupings {}
unsafe impl ::core::marker::Send for CharacterGroupings {}
unsafe impl ::core::marker::Sync for CharacterGroupings {}
