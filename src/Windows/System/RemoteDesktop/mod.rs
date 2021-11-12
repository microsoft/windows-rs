#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "System_RemoteDesktop_Input")]
pub mod Input;
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractiveSessionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractiveSessionStatics {
    type Vtable = IInteractiveSessionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60884631_dd3a_4576_9c8d_e8027618bdce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractiveSessionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
pub struct InteractiveSession {}
impl InteractiveSession {
    pub fn IsRemote() -> ::windows::core::Result<bool> {
        Self::IInteractiveSessionStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IInteractiveSessionStatics<R, F: FnOnce(&IInteractiveSessionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InteractiveSession, IInteractiveSessionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for InteractiveSession {
    const NAME: &'static str = "Windows.System.RemoteDesktop.InteractiveSession";
}
