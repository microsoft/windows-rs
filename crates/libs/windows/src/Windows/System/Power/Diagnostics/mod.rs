#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'System_Power_Diagnostics', 'deprecated'*"]
#[cfg(feature = "deprecated")]
pub struct BackgroundEnergyDiagnostics {}
#[cfg(feature = "deprecated")]
impl BackgroundEnergyDiagnostics {
    #[doc = "*Required features: 'System_Power_Diagnostics', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceSpecificConversionFactor() -> ::windows::core::Result<f64> {
        Self::IBackgroundEnergyDiagnosticsStatics(|this| unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power_Diagnostics', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn ComputeTotalEnergyUsage() -> ::windows::core::Result<u64> {
        Self::IBackgroundEnergyDiagnosticsStatics(|this| unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power_Diagnostics', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn ResetTotalEnergyUsage() -> ::windows::core::Result<()> {
        Self::IBackgroundEnergyDiagnosticsStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IBackgroundEnergyDiagnosticsStatics<R, F: FnOnce(&IBackgroundEnergyDiagnosticsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundEnergyDiagnostics, IBackgroundEnergyDiagnosticsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for BackgroundEnergyDiagnostics {
    const NAME: &'static str = "Windows.System.Power.Diagnostics.BackgroundEnergyDiagnostics";
}
#[doc = "*Required features: 'System_Power_Diagnostics', 'deprecated'*"]
#[cfg(feature = "deprecated")]
pub struct ForegroundEnergyDiagnostics {}
#[cfg(feature = "deprecated")]
impl ForegroundEnergyDiagnostics {
    #[doc = "*Required features: 'System_Power_Diagnostics', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceSpecificConversionFactor() -> ::windows::core::Result<f64> {
        Self::IForegroundEnergyDiagnosticsStatics(|this| unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power_Diagnostics', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn ComputeTotalEnergyUsage() -> ::windows::core::Result<u64> {
        Self::IForegroundEnergyDiagnosticsStatics(|this| unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        })
    }
    #[doc = "*Required features: 'System_Power_Diagnostics', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn ResetTotalEnergyUsage() -> ::windows::core::Result<()> {
        Self::IForegroundEnergyDiagnosticsStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IForegroundEnergyDiagnosticsStatics<R, F: FnOnce(&IForegroundEnergyDiagnosticsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ForegroundEnergyDiagnostics, IForegroundEnergyDiagnosticsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for ForegroundEnergyDiagnostics {
    const NAME: &'static str = "Windows.System.Power.Diagnostics.ForegroundEnergyDiagnostics";
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IBackgroundEnergyDiagnosticsStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IBackgroundEnergyDiagnosticsStatics {
    type Vtable = IBackgroundEnergyDiagnosticsStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7663702_d3a6_46e0_8f9b_50b95bb4f9c5);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundEnergyDiagnosticsStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IForegroundEnergyDiagnosticsStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IForegroundEnergyDiagnosticsStatics {
    type Vtable = IForegroundEnergyDiagnosticsStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23ca0917_cd07_4609_be15_8fe894c5e41e);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IForegroundEnergyDiagnosticsStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
