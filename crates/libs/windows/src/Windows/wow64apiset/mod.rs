#[inline]
pub unsafe fn GetSystemWow64Directory2A(lpbuffer: Option<&mut [u8]>, imagefilemachinetype: u16) -> u32 {
    windows_core::link!("api-ms-win-core-wow64-l1-1-1.dll" "system" fn GetSystemWow64Directory2A(lpbuffer : windows_core::PSTR, usize : u32, imagefilemachinetype : u16) -> u32);
    unsafe { GetSystemWow64Directory2A(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), imagefilemachinetype) }
}
#[inline]
pub unsafe fn GetSystemWow64Directory2W(lpbuffer: Option<&mut [u16]>, imagefilemachinetype: u16) -> u32 {
    windows_core::link!("api-ms-win-core-wow64-l1-1-1.dll" "system" fn GetSystemWow64Directory2W(lpbuffer : windows_core::PWSTR, usize : u32, imagefilemachinetype : u16) -> u32);
    unsafe { GetSystemWow64Directory2W(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), imagefilemachinetype) }
}
#[inline]
pub unsafe fn GetSystemWow64DirectoryA(lpbuffer: Option<&mut [u8]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemWow64DirectoryA(lpbuffer : windows_core::PSTR, usize : u32) -> u32);
    unsafe { GetSystemWow64DirectoryA(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetSystemWow64DirectoryW(lpbuffer: Option<&mut [u16]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemWow64DirectoryW(lpbuffer : windows_core::PWSTR, usize : u32) -> u32);
    unsafe { GetSystemWow64DirectoryW(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn IsWow64GuestMachineSupported(wowguestmachine: u16) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("kernel32.dll" "system" fn IsWow64GuestMachineSupported(wowguestmachine : u16, machineissupported : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        IsWow64GuestMachineSupported(wowguestmachine, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IsWow64Process(hprocess: super::winnt::HANDLE, wow64process: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsWow64Process(hprocess : super::winnt::HANDLE, wow64process : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { IsWow64Process(hprocess, wow64process as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IsWow64Process2(hprocess: super::winnt::HANDLE, pprocessmachine: *mut u16, pnativemachine: Option<*mut u16>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsWow64Process2(hprocess : super::winnt::HANDLE, pprocessmachine : *mut u16, pnativemachine : *mut u16) -> windows_core::BOOL);
    unsafe { IsWow64Process2(hprocess, pprocessmachine as _, pnativemachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn Wow64DisableWow64FsRedirection(oldvalue: *mut *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Wow64DisableWow64FsRedirection(oldvalue : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { Wow64DisableWow64FsRedirection(oldvalue as _) }
}
#[inline]
pub unsafe fn Wow64EnableWow64FsRedirection(wow64fsenableredirection: bool) -> bool {
    windows_core::link!("kernel32.dll" "system" fn Wow64EnableWow64FsRedirection(wow64fsenableredirection : bool) -> bool);
    unsafe { Wow64EnableWow64FsRedirection(wow64fsenableredirection) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn Wow64GetThreadContext(hthread: super::winnt::HANDLE, lpcontext: *mut super::winnt::WOW64_CONTEXT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Wow64GetThreadContext(hthread : super::winnt::HANDLE, lpcontext : *mut super::winnt::WOW64_CONTEXT) -> windows_core::BOOL);
    unsafe { Wow64GetThreadContext(hthread, lpcontext as _) }
}
#[inline]
pub unsafe fn Wow64RevertWow64FsRedirection(olvalue: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Wow64RevertWow64FsRedirection(olvalue : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { Wow64RevertWow64FsRedirection(olvalue) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn Wow64SetThreadContext(hthread: super::winnt::HANDLE, lpcontext: *const super::winnt::WOW64_CONTEXT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Wow64SetThreadContext(hthread : super::winnt::HANDLE, lpcontext : *const super::winnt::WOW64_CONTEXT) -> windows_core::BOOL);
    unsafe { Wow64SetThreadContext(hthread, lpcontext) }
}
#[inline]
pub unsafe fn Wow64SetThreadDefaultGuestMachine(machine: u16) -> u16 {
    windows_core::link!("api-ms-win-core-wow64-l1-1-1.dll" "system" fn Wow64SetThreadDefaultGuestMachine(machine : u16) -> u16);
    unsafe { Wow64SetThreadDefaultGuestMachine(machine) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn Wow64SuspendThread(hthread: super::winnt::HANDLE) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn Wow64SuspendThread(hthread : super::winnt::HANDLE) -> u32);
    unsafe { Wow64SuspendThread(hthread) }
}
