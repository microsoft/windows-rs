#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IRetailModeStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRetailModeStatics {
    type Vtable = IRetailModeStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7ded029_fdda_43e7_93fb_e53ab6e89ec3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRetailModeStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
pub struct RetailMode {}
impl RetailMode {
    #[cfg(feature = "deprecated")]
    pub fn RetailModeEnabled() -> ::windows::core::Result<bool> {
        Self::IRetailModeStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IRetailModeStatics<R, F: FnOnce(&IRetailModeStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RetailMode, IRetailModeStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for RetailMode {
    const NAME: &'static str = "Windows.Phone.System.Profile.RetailMode";
}
