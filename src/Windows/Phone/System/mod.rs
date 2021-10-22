#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Phone_System_Power")]
pub mod Power;
#[cfg(feature = "Phone_System_Profile")]
pub mod Profile;
#[cfg(feature = "Phone_System_UserProfile")]
pub mod UserProfile;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISystemProtectionStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemProtectionStatics {
    type Vtable = ISystemProtectionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1237542240,
        38881,
        19865,
        [139, 251, 190, 254, 170, 106, 206, 109],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemProtectionStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISystemProtectionUnlockStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemProtectionUnlockStatics {
    type Vtable = ISystemProtectionUnlockStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        110295615,
        36625,
        19531,
        [170, 13, 135, 215, 175, 123, 23, 121],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemProtectionUnlockStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
pub struct SystemProtection {}
impl SystemProtection {
    pub fn ScreenLocked() -> ::windows::runtime::Result<bool> {
        Self::ISystemProtectionStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn RequestScreenUnlock() -> ::windows::runtime::Result<()> {
        Self::ISystemProtectionUnlockStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        })
    }
    pub fn ISystemProtectionStatics<
        R,
        F: FnOnce(&ISystemProtectionStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SystemProtection,
            ISystemProtectionStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISystemProtectionUnlockStatics<
        R,
        F: FnOnce(&ISystemProtectionUnlockStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SystemProtection,
            ISystemProtectionUnlockStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for SystemProtection {
    const NAME: &'static str = "Windows.Phone.System.SystemProtection";
}
