#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const EVCCBF_LASTNOTIFICATION: u32 = 1u32;
pub const EVCF_DONTSHOWIFZERO: u32 = 16u32;
pub const EVCF_ENABLEBYDEFAULT: u32 = 2u32;
pub const EVCF_ENABLEBYDEFAULT_AUTO: u32 = 8u32;
pub const EVCF_HASSETTINGS: u32 = 1u32;
pub const EVCF_OUTOFDISKSPACE: u32 = 64u32;
pub const EVCF_REMOVEFROMLIST: u32 = 4u32;
pub const EVCF_SETTINGSMODE: u32 = 32u32;
pub const EVCF_SYSTEMAUTORUN: u32 = 256u32;
pub const EVCF_USERCONSENTOBTAINED: u32 = 128u32;
#[repr(transparent)]
pub struct IADesktopP2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IADesktopP2 {}
impl ::core::clone::Clone for IADesktopP2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IActiveDesktopP(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IActiveDesktopP {}
impl ::core::clone::Clone for IActiveDesktopP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBriefcaseInitiator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBriefcaseInitiator {}
impl ::core::clone::Clone for IBriefcaseInitiator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmptyVolumeCache(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmptyVolumeCache {}
impl ::core::clone::Clone for IEmptyVolumeCache {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmptyVolumeCache2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmptyVolumeCache2 {}
impl ::core::clone::Clone for IEmptyVolumeCache2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmptyVolumeCacheCallBack(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmptyVolumeCacheCallBack {}
impl ::core::clone::Clone for IEmptyVolumeCacheCallBack {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReconcilableObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReconcilableObject {}
impl ::core::clone::Clone for IReconcilableObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReconcileInitiator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReconcileInitiator {}
impl ::core::clone::Clone for IReconcileInitiator {
    fn clone(&self) -> Self {
        *self
    }
}
pub const REC_E_ABORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217408i32 as _);
pub const REC_E_INEEDTODOTHEUPDATES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217404i32 as _);
pub const REC_E_NOCALLBACK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217407i32 as _);
pub const REC_E_NORESIDUES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217406i32 as _);
pub const REC_E_TOODIFFERENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217405i32 as _);
pub const REC_S_IDIDTHEUPDATES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(266240i32 as _);
pub const REC_S_NOTCOMPLETE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(266241i32 as _);
pub const REC_S_NOTCOMPLETEBUTPROPAGATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(266242i32 as _);
pub const STATEBITS_FLAT: u32 = 1u32;
#[repr(transparent)]
pub struct _reconcilef(pub i32);
pub const RECONCILEF_MAYBOTHERUSER: _reconcilef = _reconcilef(1i32);
pub const RECONCILEF_FEEDBACKWINDOWVALID: _reconcilef = _reconcilef(2i32);
pub const RECONCILEF_NORESIDUESOK: _reconcilef = _reconcilef(4i32);
pub const RECONCILEF_OMITSELFRESIDUE: _reconcilef = _reconcilef(8i32);
pub const RECONCILEF_RESUMERECONCILIATION: _reconcilef = _reconcilef(16i32);
pub const RECONCILEF_YOUMAYDOTHEUPDATES: _reconcilef = _reconcilef(32i32);
pub const RECONCILEF_ONLYYOUWERECHANGED: _reconcilef = _reconcilef(64i32);
pub const ALL_RECONCILE_FLAGS: _reconcilef = _reconcilef(127i32);
impl ::core::marker::Copy for _reconcilef {}
impl ::core::clone::Clone for _reconcilef {
    fn clone(&self) -> Self {
        *self
    }
}
