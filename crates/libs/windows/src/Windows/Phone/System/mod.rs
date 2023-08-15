#[cfg(feature = "Phone_System_Power")]
pub mod Power;
#[cfg(feature = "Phone_System_Profile")]
pub mod Profile;
#[cfg(feature = "Phone_System_UserProfile")]
pub mod UserProfile;
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemProtectionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemProtectionStatics {
    type Vtable = ISystemProtectionStatics_Vtbl;
}
impl ::core::clone::Clone for ISystemProtectionStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISystemProtectionStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49c36560_97e1_4d99_8bfb_befeaa6ace6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemProtectionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ScreenLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemProtectionUnlockStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemProtectionUnlockStatics {
    type Vtable = ISystemProtectionUnlockStatics_Vtbl;
}
impl ::core::clone::Clone for ISystemProtectionUnlockStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISystemProtectionUnlockStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0692fa3f_8f11_4c4b_aa0d_87d7af7b1779);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemProtectionUnlockStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequestScreenUnlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Phone_System\"`*"]
pub struct SystemProtection;
impl SystemProtection {
    pub fn ScreenLocked() -> ::windows_core::Result<bool> {
        Self::ISystemProtectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScreenLocked)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn RequestScreenUnlock() -> ::windows_core::Result<()> {
        Self::ISystemProtectionUnlockStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RequestScreenUnlock)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    pub fn ISystemProtectionStatics<R, F: FnOnce(&ISystemProtectionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SystemProtection, ISystemProtectionStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISystemProtectionUnlockStatics<R, F: FnOnce(&ISystemProtectionUnlockStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SystemProtection, ISystemProtectionUnlockStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for SystemProtection {
    const NAME: &'static str = "Windows.Phone.System.SystemProtection";
}
