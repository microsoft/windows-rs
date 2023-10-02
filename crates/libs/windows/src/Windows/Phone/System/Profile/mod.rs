#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRetailModeStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IRetailModeStatics {
    type Vtable = IRetailModeStatics_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IRetailModeStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7ded029_fdda_43e7_93fb_e53ab6e89ec3);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IRetailModeStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub RetailModeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RetailModeEnabled: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
pub struct RetailMode;
#[cfg(feature = "deprecated")]
impl RetailMode {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn RetailModeEnabled() -> ::windows_core::Result<bool> {
        Self::IRetailModeStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RetailModeEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IRetailModeStatics<R, F: FnOnce(&IRetailModeStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RetailMode, IRetailModeStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for RetailMode {
    const NAME: &'static str = "Windows.Phone.System.Profile.RetailMode";
}
