#[cfg(feature = "gdiplustypes")]
windows_link::link!("gdiplus.dll" "system" fn GdipCreateEffect(guid : windows_sys::core::GUID, effect : *mut *mut CGpEffect) -> super::Status);
#[cfg(feature = "gdiplustypes")]
windows_link::link!("gdiplus.dll" "system" fn GdipDeleteEffect(effect : *mut CGpEffect) -> super::Status);
#[cfg(all(feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetEffectParameterSize(effect : *mut CGpEffect, size : *mut super::UINT) -> super::Status);
#[cfg(all(feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipGetEffectParameters(effect : *mut CGpEffect, size : *mut super::UINT, params : *mut core::ffi::c_void) -> super::Status);
#[cfg(all(feature = "gdiplustypes", feature = "shared"))]
windows_link::link!("gdiplus.dll" "system" fn GdipSetEffectParameters(effect : *mut CGpEffect, params : *const core::ffi::c_void, size : super::UINT) -> super::Status);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CGpEffect(pub u8);
