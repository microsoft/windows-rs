#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "System_RemoteDesktop_Input")]
pub mod Input;
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractiveSessionStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractiveSessionStatics {
    type Vtable = IInteractiveSessionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x60884631_dd3a_4576_9c8d_e8027618bdce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractiveSessionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `System_RemoteDesktop`*"]
pub struct InteractiveSession {}
impl InteractiveSession {
    #[doc = "*Required features: `System_RemoteDesktop`*"]
    pub fn IsRemote() -> ::windows::runtime::Result<bool> {
        Self::IInteractiveSessionStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IInteractiveSessionStatics<R, F: FnOnce(&IInteractiveSessionStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InteractiveSession, IInteractiveSessionStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for InteractiveSession {
    const NAME: &'static str = "Windows.System.RemoteDesktop.InteractiveSession";
}
