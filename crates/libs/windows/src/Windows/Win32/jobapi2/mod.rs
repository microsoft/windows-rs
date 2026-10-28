#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AssignProcessToJobObject(hjob: super::HANDLE, hprocess: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AssignProcessToJobObject(hjob : super::HANDLE, hprocess : super::HANDLE) -> windows_core::BOOL);
    unsafe { AssignProcessToJobObject(hjob, hprocess) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateJobObjectW<P1>(lpjobattributes: Option<super::LPSECURITY_ATTRIBUTES>, lpname: P1) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateJobObjectW(lpjobattributes : super::LPSECURITY_ATTRIBUTES, lpname : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { CreateJobObjectW(lpjobattributes.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi()) }
}
#[inline]
pub unsafe fn FreeMemoryJobObject(buffer: *const core::ffi::c_void) {
    windows_core::link!("kernel32.dll" "system" fn FreeMemoryJobObject(buffer : *const core::ffi::c_void));
    unsafe { FreeMemoryJobObject(buffer) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenJobObjectW<P2>(dwdesiredaccess: super::DWORD, binherithandle: bool, lpname: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenJobObjectW(dwdesiredaccess : super::DWORD, binherithandle : windows_core::BOOL, lpname : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { OpenJobObjectW(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryInformationJobObject(hjob: Option<super::HANDLE>, jobobjectinformationclass: super::JOBOBJECTINFOCLASS, lpjobobjectinformation: super::LPVOID, cbjobobjectinformationlength: super::DWORD, lpreturnlength: Option<super::LPDWORD>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryInformationJobObject(hjob : super::HANDLE, jobobjectinformationclass : super::JOBOBJECTINFOCLASS, lpjobobjectinformation : super::LPVOID, cbjobobjectinformationlength : super::DWORD, lpreturnlength : super::LPDWORD) -> windows_core::BOOL);
    unsafe { QueryInformationJobObject(hjob.unwrap_or(core::mem::zeroed()) as _, jobobjectinformationclass, lpjobobjectinformation as _, cbjobobjectinformationlength, lpreturnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryIoRateControlInformationJobObject<P1>(hjob: Option<super::HANDLE>, volumename: P1, infoblocks: *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION, infoblockcount: *mut super::ULONG) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn QueryIoRateControlInformationJobObject(hjob : super::HANDLE, volumename : windows_core::PCWSTR, infoblocks : *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION, infoblockcount : *mut super::ULONG) -> super::DWORD);
    unsafe { QueryIoRateControlInformationJobObject(hjob.unwrap_or(core::mem::zeroed()) as _, volumename.param().abi(), infoblocks as _, infoblockcount as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetInformationJobObject(hjob: super::HANDLE, jobobjectinformationclass: super::JOBOBJECTINFOCLASS, lpjobobjectinformation: super::LPVOID, cbjobobjectinformationlength: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetInformationJobObject(hjob : super::HANDLE, jobobjectinformationclass : super::JOBOBJECTINFOCLASS, lpjobobjectinformation : super::LPVOID, cbjobobjectinformationlength : super::DWORD) -> windows_core::BOOL);
    unsafe { SetInformationJobObject(hjob, jobobjectinformationclass, lpjobobjectinformation, cbjobobjectinformationlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetIoRateControlInformationJobObject(hjob: super::HANDLE, ioratecontrolinfo: *const JOBOBJECT_IO_RATE_CONTROL_INFORMATION) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn SetIoRateControlInformationJobObject(hjob : super::HANDLE, ioratecontrolinfo : *const JOBOBJECT_IO_RATE_CONTROL_INFORMATION) -> super::DWORD);
    unsafe { SetIoRateControlInformationJobObject(hjob, ioratecontrolinfo) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TerminateJobObject(hjob: super::HANDLE, uexitcode: super::UINT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TerminateJobObject(hjob : super::HANDLE, uexitcode : super::UINT) -> windows_core::BOOL);
    unsafe { TerminateJobObject(hjob, uexitcode) }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    pub MaxIops: super::LONG64,
    pub MaxBandwidth: super::LONG64,
    pub ReservationIops: super::LONG64,
    pub VolumeName: windows_core::PCWSTR,
    pub BaseIoSize: super::ULONG,
    pub ControlFlags: super::ULONG,
}
