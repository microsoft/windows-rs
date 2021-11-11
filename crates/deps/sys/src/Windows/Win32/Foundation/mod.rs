#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CloseHandle();
    fn CompareObjectHandles();
    fn DuplicateHandle();
    fn GetHandleInformation();
    fn GetLastError();
    fn RtlNtStatusToDosError();
    fn SetHandleInformation();
    fn SetLastError();
    fn SetLastErrorEx();
    fn SysAddRefString();
    fn SysAllocString();
    fn SysAllocStringByteLen();
    fn SysAllocStringLen();
    fn SysFreeString();
    fn SysReAllocString();
    fn SysReAllocStringLen();
    fn SysReleaseString();
    fn SysStringByteLen();
    fn SysStringLen();
}
