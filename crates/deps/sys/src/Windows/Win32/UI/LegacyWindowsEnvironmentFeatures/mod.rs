#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCCBF_LASTNOTIFICATION: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_DONTSHOWIFZERO: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_ENABLEBYDEFAULT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_ENABLEBYDEFAULT_AUTO: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_HASSETTINGS: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_OUTOFDISKSPACE: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_REMOVEFROMLIST: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_SETTINGSMODE: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_SYSTEMAUTORUN: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const EVCF_USERCONSENTOBTAINED: u32 = 128u32;
pub struct IADesktopP2(pub *mut ::core::ffi::c_void);
pub struct IActiveDesktopP(pub *mut ::core::ffi::c_void);
pub struct IBriefcaseInitiator(pub *mut ::core::ffi::c_void);
pub struct IEmptyVolumeCache(pub *mut ::core::ffi::c_void);
pub struct IEmptyVolumeCache2(pub *mut ::core::ffi::c_void);
pub struct IEmptyVolumeCacheCallBack(pub *mut ::core::ffi::c_void);
pub struct IReconcilableObject(pub *mut ::core::ffi::c_void);
pub struct IReconcileInitiator(pub *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const REC_E_ABORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217408i32 as _);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const REC_E_INEEDTODOTHEUPDATES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217404i32 as _);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const REC_E_NOCALLBACK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217407i32 as _);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const REC_E_NORESIDUES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217406i32 as _);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const REC_E_TOODIFFERENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217405i32 as _);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const REC_S_IDIDTHEUPDATES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(266240i32 as _);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const REC_S_NOTCOMPLETE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(266241i32 as _);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const REC_S_NOTCOMPLETEBUTPROPAGATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(266242i32 as _);
#[doc = "*Required features: `Win32_UI_LegacyWindowsEnvironmentFeatures`*"]
pub const STATEBITS_FLAT: u32 = 1u32;
pub struct _reconcilef(i32);
