#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49c36560_97e1_4d99_8bfb_befeaa6ace6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemProtectionStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ScreenLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemProtectionUnlockStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemProtectionUnlockStatics {
    type Vtable = ISystemProtectionUnlockStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0692fa3f_8f11_4c4b_aa0d_87d7af7b1779);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemProtectionUnlockStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RequestScreenUnlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Phone_System\"`*"]
pub struct SystemProtection {}
impl SystemProtection {
    #[doc = "*Required features: `\"Phone_System\"`*"]
    pub fn ScreenLocked() -> ::windows::core::Result<bool> {
        Self::ISystemProtectionStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScreenLocked)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_System\"`*"]
    pub fn RequestScreenUnlock() -> ::windows::core::Result<()> {
        Self::ISystemProtectionUnlockStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RequestScreenUnlock)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc(hidden)]
    pub fn ISystemProtectionStatics<R, F: FnOnce(&ISystemProtectionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemProtection, ISystemProtectionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ISystemProtectionUnlockStatics<R, F: FnOnce(&ISystemProtectionUnlockStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemProtection, ISystemProtectionUnlockStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for SystemProtection {
    const NAME: &'static str = "Windows.Phone.System.SystemProtection";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
