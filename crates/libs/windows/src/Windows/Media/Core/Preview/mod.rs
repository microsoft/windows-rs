#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ISoundLevelBrokerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISoundLevelBrokerStatics {
    type Vtable = ISoundLevelBrokerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a633961_dbed_464c_a09a_33412f5caa3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoundLevelBrokerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SoundLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::SoundLevel) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SoundLevelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSoundLevelChanged: usize,
}
#[doc = "*Required features: `\"Media_Core_Preview\"`*"]
pub struct SoundLevelBroker {}
impl SoundLevelBroker {
    #[doc = "*Required features: `\"Media_Core_Preview\"`*"]
    pub fn SoundLevel() -> ::windows::core::Result<super::super::SoundLevel> {
        Self::ISoundLevelBrokerStatics(|this| unsafe {
            let mut result__: super::super::SoundLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SoundLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::SoundLevel>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Core_Preview\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SoundLevelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::ISoundLevelBrokerStatics(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SoundLevelChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Core_Preview\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSoundLevelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ISoundLevelBrokerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveSoundLevelChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn ISoundLevelBrokerStatics<R, F: FnOnce(&ISoundLevelBrokerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SoundLevelBroker, ISoundLevelBrokerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for SoundLevelBroker {
    const NAME: &'static str = "Windows.Media.Core.Preview.SoundLevelBroker";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
