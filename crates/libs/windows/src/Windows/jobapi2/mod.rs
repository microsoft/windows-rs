#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AssignProcessToJobObject(hjob: super::winnt::HANDLE, hprocess: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AssignProcessToJobObject(hjob : super::winnt::HANDLE, hprocess : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { AssignProcessToJobObject(hjob, hprocess) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateJobObjectW<P1>(lpjobattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, lpname: P1) -> super::winnt::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateJobObjectW(lpjobattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, lpname : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { CreateJobObjectW(lpjobattributes.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi()) }
}
#[inline]
pub unsafe fn FreeMemoryJobObject(buffer: *const core::ffi::c_void) {
    windows_core::link!("kernel32.dll" "system" fn FreeMemoryJobObject(buffer : *const core::ffi::c_void));
    unsafe { FreeMemoryJobObject(buffer) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn OpenJobObjectW<P2>(dwdesiredaccess: u32, binherithandle: bool, lpname: P2) -> super::winnt::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenJobObjectW(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, lpname : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { OpenJobObjectW(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryInformationJobObject(hjob: Option<super::winnt::HANDLE>, jobobjectinformationclass: super::winnt::JOBOBJECTINFOCLASS, lpjobobjectinformation: *mut core::ffi::c_void, cbjobobjectinformationlength: u32, lpreturnlength: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryInformationJobObject(hjob : super::winnt::HANDLE, jobobjectinformationclass : super::winnt::JOBOBJECTINFOCLASS, lpjobobjectinformation : *mut core::ffi::c_void, cbjobobjectinformationlength : u32, lpreturnlength : *mut u32) -> windows_core::BOOL);
    unsafe { QueryInformationJobObject(hjob.unwrap_or(core::mem::zeroed()) as _, jobobjectinformationclass, lpjobobjectinformation as _, cbjobobjectinformationlength, lpreturnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryIoRateControlInformationJobObject<P1>(hjob: Option<super::winnt::HANDLE>, volumename: P1, infoblocks: *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION, infoblockcount: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn QueryIoRateControlInformationJobObject(hjob : super::winnt::HANDLE, volumename : windows_core::PCWSTR, infoblocks : *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION, infoblockcount : *mut u32) -> u32);
    unsafe { QueryIoRateControlInformationJobObject(hjob.unwrap_or(core::mem::zeroed()) as _, volumename.param().abi(), infoblocks as _, infoblockcount as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetInformationJobObject(hjob: super::winnt::HANDLE, jobobjectinformationclass: super::winnt::JOBOBJECTINFOCLASS, lpjobobjectinformation: *const core::ffi::c_void, cbjobobjectinformationlength: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetInformationJobObject(hjob : super::winnt::HANDLE, jobobjectinformationclass : super::winnt::JOBOBJECTINFOCLASS, lpjobobjectinformation : *const core::ffi::c_void, cbjobobjectinformationlength : u32) -> windows_core::BOOL);
    unsafe { SetInformationJobObject(hjob, jobobjectinformationclass, lpjobobjectinformation, cbjobobjectinformationlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetIoRateControlInformationJobObject(hjob: super::winnt::HANDLE, ioratecontrolinfo: *const JOBOBJECT_IO_RATE_CONTROL_INFORMATION) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn SetIoRateControlInformationJobObject(hjob : super::winnt::HANDLE, ioratecontrolinfo : *const JOBOBJECT_IO_RATE_CONTROL_INFORMATION) -> u32);
    unsafe { SetIoRateControlInformationJobObject(hjob, ioratecontrolinfo) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TerminateJobObject(hjob: super::winnt::HANDLE, uexitcode: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TerminateJobObject(hjob : super::winnt::HANDLE, uexitcode : u32) -> windows_core::BOOL);
    unsafe { TerminateJobObject(hjob, uexitcode) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: windows_core::PCWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: u32,
}
