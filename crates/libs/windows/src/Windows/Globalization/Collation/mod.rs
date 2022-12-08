#[doc(hidden)]
#[repr(transparent)]
pub struct ICharacterGrouping(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICharacterGrouping {
    type Vtable = ICharacterGrouping_Vtbl;
}
unsafe impl ::windows::core::Interface for ICharacterGrouping {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfae761bb_805d_4bb0_95bb_c1f7c3e8eb8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterGrouping_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub First: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICharacterGroupings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICharacterGroupings {
    type Vtable = ICharacterGroupings_Vtbl;
}
unsafe impl ::windows::core::Interface for ICharacterGroupings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8d20a75_d4cf_4055_80e5_ce169c226496);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterGroupings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Lookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICharacterGroupingsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICharacterGroupingsFactory {
    type Vtable = ICharacterGroupingsFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICharacterGroupingsFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99ea9fd9_886d_4401_9f98_69c82d4c2f78);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterGroupingsFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Globalization_Collation\"`*"]
#[repr(transparent)]
pub struct CharacterGrouping(::windows::core::IUnknown);
impl CharacterGrouping {
    pub fn First(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Label)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CharacterGrouping {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CharacterGrouping {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CharacterGrouping {}
impl ::core::fmt::Debug for CharacterGrouping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CharacterGrouping").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CharacterGrouping {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.Collation.CharacterGrouping;{fae761bb-805d-4bb0-95bb-c1f7c3e8eb8e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CharacterGrouping {
    type Vtable = ICharacterGrouping_Vtbl;
}
unsafe impl ::windows::core::Interface for CharacterGrouping {
    const IID: ::windows::core::GUID = <ICharacterGrouping as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CharacterGrouping {
    const NAME: &'static str = "Windows.Globalization.Collation.CharacterGrouping";
}
::windows::core::interface_hierarchy!(CharacterGrouping, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CharacterGrouping {}
unsafe impl ::core::marker::Sync for CharacterGrouping {}
#[doc = "*Required features: `\"Globalization_Collation\"`*"]
#[repr(transparent)]
pub struct CharacterGroupings(::windows::core::IUnknown);
impl CharacterGroupings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CharacterGroupings, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Lookup(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Lookup)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(text), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create(language: &::windows::core::HSTRING) -> ::windows::core::Result<CharacterGroupings> {
        Self::ICharacterGroupingsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(language), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<CharacterGrouping>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<CharacterGrouping> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &CharacterGrouping, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<CharacterGrouping>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ICharacterGroupingsFactory<R, F: FnOnce(&ICharacterGroupingsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CharacterGroupings, ICharacterGroupingsFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CharacterGroupings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CharacterGroupings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CharacterGroupings {}
impl ::core::fmt::Debug for CharacterGroupings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CharacterGroupings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CharacterGroupings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.Collation.CharacterGroupings;{b8d20a75-d4cf-4055-80e5-ce169c226496})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CharacterGroupings {
    type Vtable = ICharacterGroupings_Vtbl;
}
unsafe impl ::windows::core::Interface for CharacterGroupings {
    const IID: ::windows::core::GUID = <ICharacterGroupings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CharacterGroupings {
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
        super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
::windows::core::interface_hierarchy!(CharacterGroupings, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&CharacterGroupings> for ::windows::core::InParam<super::super::Foundation::Collections::IIterable<CharacterGrouping>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CharacterGroupings) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
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
impl ::core::convert::TryFrom<&CharacterGroupings> for ::windows::core::InParam<super::super::Foundation::Collections::IVectorView<CharacterGrouping>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CharacterGroupings) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CharacterGroupings {}
unsafe impl ::core::marker::Sync for CharacterGroupings {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
