#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn AssignProcessToJobObject(hjob : super::HANDLE, hprocess : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateJobObjectW(lpjobattributes : super::LPSECURITY_ATTRIBUTES, lpname : windows_sys::core::PCWSTR) -> super::HANDLE);
windows_link::link!("kernel32.dll" "system" fn FreeMemoryJobObject(buffer : *const core::ffi::c_void));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn OpenJobObjectW(dwdesiredaccess : super::DWORD, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn QueryInformationJobObject(hjob : super::HANDLE, jobobjectinformationclass : super::JOBOBJECTINFOCLASS, lpjobobjectinformation : super::LPVOID, cbjobobjectinformationlength : super::DWORD, lpreturnlength : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn QueryIoRateControlInformationJobObject(hjob : super::HANDLE, volumename : windows_sys::core::PCWSTR, infoblocks : *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION, infoblockcount : *mut super::ULONG) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetInformationJobObject(hjob : super::HANDLE, jobobjectinformationclass : super::JOBOBJECTINFOCLASS, lpjobobjectinformation : super::LPVOID, cbjobobjectinformationlength : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetIoRateControlInformationJobObject(hjob : super::HANDLE, ioratecontrolinfo : *const JOBOBJECT_IO_RATE_CONTROL_INFORMATION) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn TerminateJobObject(hjob : super::HANDLE, uexitcode : super::UINT) -> windows_sys::core::BOOL);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    pub MaxIops: super::LONG64,
    pub MaxBandwidth: super::LONG64,
    pub ReservationIops: super::LONG64,
    pub VolumeName: windows_sys::core::PCWSTR,
    pub BaseIoSize: super::ULONG,
    pub ControlFlags: super::ULONG,
}
