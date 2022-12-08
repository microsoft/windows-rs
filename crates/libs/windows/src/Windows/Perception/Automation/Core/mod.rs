#[doc(hidden)]
#[repr(transparent)]
pub struct ICorePerceptionAutomationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICorePerceptionAutomationStatics {
    type Vtable = ICorePerceptionAutomationStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICorePerceptionAutomationStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bb04541_4ce2_4923_9a76_8187ecc59112);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorePerceptionAutomationStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetActivationFactoryProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetActivationFactoryProvider: usize,
}
#[doc = "*Required features: `\"Perception_Automation_Core\"`*"]
pub struct CorePerceptionAutomation;
impl CorePerceptionAutomation {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetActivationFactoryProvider<P0, E0>(provider: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IGetActivationFactory>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICorePerceptionAutomationStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).SetActivationFactoryProvider)(::windows::core::Vtable::as_raw(this), provider.try_into().map_err(|e| e.into())?.abi()).ok() })
    }
    #[doc(hidden)]
    pub fn ICorePerceptionAutomationStatics<R, F: FnOnce(&ICorePerceptionAutomationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CorePerceptionAutomation, ICorePerceptionAutomationStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CorePerceptionAutomation {
    const NAME: &'static str = "Windows.Perception.Automation.Core.CorePerceptionAutomation";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
