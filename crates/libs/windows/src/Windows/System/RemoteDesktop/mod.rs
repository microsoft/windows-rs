#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "System_RemoteDesktop_Input")]
pub mod Input;
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractiveSessionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractiveSessionStatics {
    type Vtable = IInteractiveSessionStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60884631_dd3a_4576_9c8d_e8027618bdce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractiveSessionStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsRemote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"System_RemoteDesktop\"`*"]
pub struct InteractiveSession {}
impl InteractiveSession {
    #[doc = "*Required features: `\"System_RemoteDesktop\"`*"]
    pub fn IsRemote() -> ::windows::core::Result<bool> {
        Self::IInteractiveSessionStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRemote)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInteractiveSessionStatics<R, F: FnOnce(&IInteractiveSessionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InteractiveSession, IInteractiveSessionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for InteractiveSession {
    const NAME: &'static str = "Windows.System.RemoteDesktop.InteractiveSession";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
