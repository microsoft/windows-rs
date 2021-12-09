#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub struct CorePerceptionAutomation {}
impl CorePerceptionAutomation {
    #[cfg(feature = "Foundation")]
    pub fn SetActivationFactoryProvider<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IGetActivationFactory>>(provider: Param0) -> ::windows::core::Result<()> {
        Self::ICorePerceptionAutomationStatics(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), provider.into_param().abi()).ok() })
    }
    pub fn ICorePerceptionAutomationStatics<R, F: FnOnce(&ICorePerceptionAutomationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CorePerceptionAutomation, ICorePerceptionAutomationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CorePerceptionAutomation {
    const NAME: &'static str = "Windows.Perception.Automation.Core.CorePerceptionAutomation";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICorePerceptionAutomationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICorePerceptionAutomationStatics {
    type Vtable = ICorePerceptionAutomationStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bb04541_4ce2_4923_9a76_8187ecc59112);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorePerceptionAutomationStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
