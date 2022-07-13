#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IRetailModeStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IRetailModeStatics {
    type Vtable = IRetailModeStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7ded029_fdda_43e7_93fb_e53ab6e89ec3);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IRetailModeStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub RetailModeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RetailModeEnabled: usize,
}
#[doc = "*Required features: `\"Phone_System_Profile\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct RetailMode;
#[cfg(feature = "deprecated")]
impl RetailMode {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn RetailModeEnabled() -> ::windows::core::Result<bool> {
        Self::IRetailModeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RetailModeEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IRetailModeStatics<R, F: FnOnce(&IRetailModeStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RetailMode, IRetailModeStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for RetailMode {
    const NAME: &'static str = "Windows.Phone.System.Profile.RetailMode";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
