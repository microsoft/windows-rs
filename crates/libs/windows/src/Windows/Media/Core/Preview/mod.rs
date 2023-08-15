#[doc(hidden)]
#[repr(transparent)]
pub struct ISoundLevelBrokerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISoundLevelBrokerStatics {
    type Vtable = ISoundLevelBrokerStatics_Vtbl;
}
impl ::core::clone::Clone for ISoundLevelBrokerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISoundLevelBrokerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a633961_dbed_464c_a09a_33412f5caa3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoundLevelBrokerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SoundLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::SoundLevel) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SoundLevelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSoundLevelChanged: usize,
}
#[doc = "*Required features: `\"Media_Core_Preview\"`*"]
pub struct SoundLevelBroker;
impl SoundLevelBroker {
    pub fn SoundLevel() -> ::windows_core::Result<super::super::SoundLevel> {
        Self::ISoundLevelBrokerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SoundLevel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SoundLevelChanged<P0>(handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::ISoundLevelBrokerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SoundLevelChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSoundLevelChanged(token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::ISoundLevelBrokerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveSoundLevelChanged)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn ISoundLevelBrokerStatics<R, F: FnOnce(&ISoundLevelBrokerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SoundLevelBroker, ISoundLevelBrokerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for SoundLevelBroker {
    const NAME: &'static str = "Windows.Media.Core.Preview.SoundLevelBroker";
}
