#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn AssignProcessToJobObject(hjob : super::winnt::HANDLE, hprocess : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateJobObjectW(lpjobattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, lpname : windows_sys::core::PCWSTR) -> super::winnt::HANDLE);
windows_link::link!("kernel32.dll" "system" fn FreeMemoryJobObject(buffer : *const core::ffi::c_void));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenJobObjectW(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::winnt::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryInformationJobObject(hjob : super::winnt::HANDLE, jobobjectinformationclass : super::winnt::JOBOBJECTINFOCLASS, lpjobobjectinformation : *mut core::ffi::c_void, cbjobobjectinformationlength : u32, lpreturnlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryIoRateControlInformationJobObject(hjob : super::winnt::HANDLE, volumename : windows_sys::core::PCWSTR, infoblocks : *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION, infoblockcount : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetInformationJobObject(hjob : super::winnt::HANDLE, jobobjectinformationclass : super::winnt::JOBOBJECTINFOCLASS, lpjobobjectinformation : *const core::ffi::c_void, cbjobobjectinformationlength : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetIoRateControlInformationJobObject(hjob : super::winnt::HANDLE, ioratecontrolinfo : *const JOBOBJECT_IO_RATE_CONTROL_INFORMATION) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn TerminateJobObject(hjob : super::winnt::HANDLE, uexitcode : u32) -> windows_sys::core::BOOL);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: windows_sys::core::PCWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: u32,
}
impl Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
