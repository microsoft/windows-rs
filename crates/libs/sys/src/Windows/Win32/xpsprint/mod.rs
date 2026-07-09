#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_winnt"))]
windows_link::link!("xpsprint.dll" "system" fn StartXpsPrintJob(printername : windows_sys::core::PCWSTR, jobname : windows_sys::core::PCWSTR, outputfilename : windows_sys::core::PCWSTR, progressevent : super::winnt::HANDLE, completionevent : super::winnt::HANDLE, printablepageson : *const u8, printablepagesoncount : u32, xpsprintjob : *mut *mut core::ffi::c_void, documentstream : *mut *mut core::ffi::c_void, printticketstream : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_xpsobjectmodel"))]
windows_link::link!("xpsprint.dll" "system" fn StartXpsPrintJob1(printername : windows_sys::core::PCWSTR, jobname : windows_sys::core::PCWSTR, outputfilename : windows_sys::core::PCWSTR, progressevent : super::winnt::HANDLE, completionevent : super::winnt::HANDLE, xpsprintjob : *mut *mut core::ffi::c_void, printcontentreceiver : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const XPS_JOB_CANCELLED: XPS_JOB_COMPLETION = 2;
pub const XPS_JOB_COMPLETED: XPS_JOB_COMPLETION = 1;
pub type XPS_JOB_COMPLETION = i32;
pub const XPS_JOB_FAILED: XPS_JOB_COMPLETION = 3;
pub const XPS_JOB_IN_PROGRESS: XPS_JOB_COMPLETION = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct XPS_JOB_STATUS {
    pub jobId: u32,
    pub currentDocument: i32,
    pub currentPage: i32,
    pub currentPageTotal: i32,
    pub completion: XPS_JOB_COMPLETION,
    pub jobStatus: windows_sys::core::HRESULT,
}
