#[cfg(feature = "gdiplustypes")]
#[inline]
pub unsafe fn GdipCreateEffect(guid: windows_core::GUID, effect: *mut *mut CGpEffect) -> super::Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateEffect(guid : windows_core::GUID, effect : *mut *mut CGpEffect) -> super::Status);
    unsafe { GdipCreateEffect(guid, effect as _) }
}
#[cfg(feature = "gdiplustypes")]
#[inline]
pub unsafe fn GdipDeleteEffect(effect: *mut CGpEffect) -> super::Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteEffect(effect : *mut CGpEffect) -> super::Status);
    unsafe { GdipDeleteEffect(effect as _) }
}
#[cfg(all(feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetEffectParameterSize(effect: *mut CGpEffect, size: *mut super::UINT) -> super::Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetEffectParameterSize(effect : *mut CGpEffect, size : *mut super::UINT) -> super::Status);
    unsafe { GdipGetEffectParameterSize(effect as _, size as _) }
}
#[cfg(all(feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipGetEffectParameters(effect: *mut CGpEffect, size: *mut super::UINT, params: *mut core::ffi::c_void) -> super::Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetEffectParameters(effect : *mut CGpEffect, size : *mut super::UINT, params : *mut core::ffi::c_void) -> super::Status);
    unsafe { GdipGetEffectParameters(effect as _, size as _, params as _) }
}
#[cfg(all(feature = "gdiplustypes", feature = "shared"))]
#[inline]
pub unsafe fn GdipSetEffectParameters(effect: *mut CGpEffect, params: *const core::ffi::c_void, size: super::UINT) -> super::Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetEffectParameters(effect : *mut CGpEffect, params : *const core::ffi::c_void, size : super::UINT) -> super::Status);
    unsafe { GdipSetEffectParameters(effect as _, params, size) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CGpEffect(pub u8);
