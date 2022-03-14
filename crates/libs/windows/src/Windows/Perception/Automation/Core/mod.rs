#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Perception_Automation_Core\"`*"]
pub struct CorePerceptionAutomation {}
impl CorePerceptionAutomation {
    #[doc = "*Required features: `\"Perception_Automation_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetActivationFactoryProvider<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IGetActivationFactory>>(provider: Param0) -> ::windows::core::Result<()> {
        Self::ICorePerceptionAutomationStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetActivationFactoryProvider)(::core::mem::transmute_copy(this), provider.into_param().abi()).ok() })
    }
    #[doc(hidden)]
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
    type Vtable = ICorePerceptionAutomationStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bb04541_4ce2_4923_9a76_8187ecc59112);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorePerceptionAutomationStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SetActivationFactoryProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetActivationFactoryProvider: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
