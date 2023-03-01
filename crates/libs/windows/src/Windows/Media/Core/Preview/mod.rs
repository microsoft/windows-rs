#[doc(hidden)]
#[repr(transparent)]
pub struct ISoundLevelBrokerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISoundLevelBrokerStatics {
    type Vtable = ISoundLevelBrokerStatics_Vtbl;
}
impl ::core::clone::Clone for ISoundLevelBrokerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISoundLevelBrokerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a633961_dbed_464c_a09a_33412f5caa3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoundLevelBrokerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SoundLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::SoundLevel) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SoundLevelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSoundLevelChanged: usize,
}
#[doc = "*Required features: `\"Media_Core_Preview\"`*"]
pub struct SoundLevelBroker;
impl SoundLevelBroker {
    pub fn SoundLevel() -> ::windows::core::Result<super::super::SoundLevel> {
        Self::ISoundLevelBrokerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::SoundLevel>();
            (::windows::core::Interface::vtable(this).SoundLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SoundLevelChanged(handler: &super::super::super::Foundation::EventHandler<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::ISoundLevelBrokerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).SoundLevelChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSoundLevelChanged(token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::ISoundLevelBrokerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveSoundLevelChanged)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn ISoundLevelBrokerStatics<R, F: FnOnce(&ISoundLevelBrokerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SoundLevelBroker, ISoundLevelBrokerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for SoundLevelBroker {
    const NAME: &'static str = "Windows.Media.Core.Preview.SoundLevelBroker";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
