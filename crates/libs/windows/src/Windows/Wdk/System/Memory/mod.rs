#[inline]
pub unsafe fn NtMapViewOfSection(sectionhandle: super::super::super::Win32::Foundation::HANDLE, processhandle: super::super::super::Win32::Foundation::HANDLE, baseaddress: *mut *mut core::ffi::c_void, zerobits: usize, commitsize: usize, sectionoffset: Option<*mut i64>, viewsize: *mut usize, inheritdisposition: SECTION_INHERIT, allocationtype: u32, win32protect: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtMapViewOfSection(sectionhandle : super::super::super::Win32::Foundation:: HANDLE, processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut core::ffi::c_void, zerobits : usize, commitsize : usize, sectionoffset : *mut i64, viewsize : *mut usize, inheritdisposition : SECTION_INHERIT, allocationtype : u32, win32protect : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtMapViewOfSection(sectionhandle, processhandle, core::mem::transmute(baseaddress), zerobits, commitsize, core::mem::transmute(sectionoffset.unwrap_or(core::mem::zeroed())), core::mem::transmute(viewsize), inheritdisposition, allocationtype, win32protect) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NtOpenSection(sectionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtOpenSection(sectionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtOpenSection(core::mem::transmute(sectionhandle), desiredaccess, objectattributes) }
}
#[inline]
pub unsafe fn NtUnmapViewOfSection(processhandle: super::super::super::Win32::Foundation::HANDLE, baseaddress: Option<*const core::ffi::c_void>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtUnmapViewOfSection(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtUnmapViewOfSection(processhandle, core::mem::transmute(baseaddress.unwrap_or(core::mem::zeroed()))) }
}
#[inline]
pub unsafe fn ZwMapViewOfSection(sectionhandle: super::super::super::Win32::Foundation::HANDLE, processhandle: super::super::super::Win32::Foundation::HANDLE, baseaddress: *mut *mut core::ffi::c_void, zerobits: usize, commitsize: usize, sectionoffset: Option<*mut i64>, viewsize: *mut usize, inheritdisposition: SECTION_INHERIT, allocationtype: u32, win32protect: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwMapViewOfSection(sectionhandle : super::super::super::Win32::Foundation:: HANDLE, processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut core::ffi::c_void, zerobits : usize, commitsize : usize, sectionoffset : *mut i64, viewsize : *mut usize, inheritdisposition : SECTION_INHERIT, allocationtype : u32, win32protect : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwMapViewOfSection(sectionhandle, processhandle, core::mem::transmute(baseaddress), zerobits, commitsize, core::mem::transmute(sectionoffset.unwrap_or(core::mem::zeroed())), core::mem::transmute(viewsize), inheritdisposition, allocationtype, win32protect) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ZwOpenSection(sectionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwOpenSection(sectionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwOpenSection(core::mem::transmute(sectionhandle), desiredaccess, objectattributes) }
}
#[inline]
pub unsafe fn ZwUnmapViewOfSection(processhandle: super::super::super::Win32::Foundation::HANDLE, baseaddress: Option<*const core::ffi::c_void>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwUnmapViewOfSection(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwUnmapViewOfSection(processhandle, core::mem::transmute(baseaddress.unwrap_or(core::mem::zeroed()))) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECTION_INHERIT(pub i32);
pub const ViewShare: SECTION_INHERIT = SECTION_INHERIT(1i32);
pub const ViewUnmap: SECTION_INHERIT = SECTION_INHERIT(2i32);
