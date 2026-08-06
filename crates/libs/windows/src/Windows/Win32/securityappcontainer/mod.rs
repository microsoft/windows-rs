#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetAppContainerNamedObjectPath(token: Option<super::HANDLE>, appcontainersid: Option<super::PSID>, objectpathlength: u32, objectpath: Option<windows_core::PWSTR>, returnlength: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetAppContainerNamedObjectPath(token : super::HANDLE, appcontainersid : super::PSID, objectpathlength : u32, objectpath : windows_core::PWSTR, returnlength : *mut u32) -> windows_core::BOOL);
    unsafe { GetAppContainerNamedObjectPath(token.unwrap_or(core::mem::zeroed()) as _, appcontainersid.unwrap_or(core::mem::zeroed()) as _, objectpathlength, objectpath.unwrap_or(core::mem::zeroed()) as _, returnlength as _) }
}
