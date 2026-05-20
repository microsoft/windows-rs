pub type GlobalMemoryStatus = unsafe extern "system" fn(lpbuffer: *mut MEMORYSTATUS);
windows_link::link!("kernel32.dll" "system" fn GlobalMemoryStatus(lpbuffer : *mut MEMORYSTATUS));
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MEMORYSTATUS {
    pub dwLength: u32,
    pub dwMemoryLoad: u32,
    pub dwTotalPhys: usize,
    pub dwAvailPhys: usize,
    pub dwTotalPageFile: usize,
    pub dwAvailPageFile: usize,
    pub dwTotalVirtual: usize,
    pub dwAvailVirtual: usize,
}
