#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetAppContainerNamedObjectPath(token: Option<super::winnt::HANDLE>, appcontainersid: Option<super::winnt::PSID>, objectpath: Option<&mut [u16]>, returnlength: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetAppContainerNamedObjectPath(token : super::winnt::HANDLE, appcontainersid : super::winnt::PSID, objectpathlength : u32, objectpath : windows_core::PWSTR, returnlength : *mut u32) -> windows_core::BOOL);
    unsafe { GetAppContainerNamedObjectPath(token.unwrap_or(core::mem::zeroed()) as _, appcontainersid.unwrap_or(core::mem::zeroed()) as _, objectpath.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(objectpath.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), returnlength as _) }
}
