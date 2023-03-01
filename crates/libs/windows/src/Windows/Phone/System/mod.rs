#[cfg(feature = "Phone_System_Power")]
pub mod Power;
#[cfg(feature = "Phone_System_Profile")]
pub mod Profile;
#[cfg(feature = "Phone_System_UserProfile")]
pub mod UserProfile;
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemProtectionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemProtectionStatics {
    type Vtable = ISystemProtectionStatics_Vtbl;
}
impl ::core::clone::Clone for ISystemProtectionStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISystemProtectionStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49c36560_97e1_4d99_8bfb_befeaa6ace6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemProtectionStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ScreenLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemProtectionUnlockStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemProtectionUnlockStatics {
    type Vtable = ISystemProtectionUnlockStatics_Vtbl;
}
impl ::core::clone::Clone for ISystemProtectionUnlockStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISystemProtectionUnlockStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0692fa3f_8f11_4c4b_aa0d_87d7af7b1779);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemProtectionUnlockStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RequestScreenUnlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Phone_System\"`*"]
pub struct SystemProtection;
impl SystemProtection {
    pub fn ScreenLocked() -> ::windows::core::Result<bool> {
        Self::ISystemProtectionStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ScreenLocked)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn RequestScreenUnlock() -> ::windows::core::Result<()> {
        Self::ISystemProtectionUnlockStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RequestScreenUnlock)(::windows::core::Interface::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    pub fn ISystemProtectionStatics<R, F: FnOnce(&ISystemProtectionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SystemProtection, ISystemProtectionStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISystemProtectionUnlockStatics<R, F: FnOnce(&ISystemProtectionUnlockStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SystemProtection, ISystemProtectionUnlockStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for SystemProtection {
    const NAME: &'static str = "Windows.Phone.System.SystemProtection";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
