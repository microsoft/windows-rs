#[inline]
pub unsafe fn PerfCloseQueryHandle(hquery: HANDLE) -> ULONG {
    windows_core::link!("advapi32.dll" "system" fn PerfCloseQueryHandle(hquery : HANDLE) -> ULONG);
    unsafe { PerfCloseQueryHandle(hquery) }
}
#[inline]
pub unsafe fn PerfOpenQueryHandle<P0>(szmachine: P0, phquery: *mut HANDLE) -> ULONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn PerfOpenQueryHandle(szmachine : windows_core::PCWSTR, phquery : *mut HANDLE) -> ULONG);
    unsafe { PerfOpenQueryHandle(szmachine.param().abi(), phquery as _) }
}
#[inline]
pub unsafe fn PerfQueryCounterInfo(
    hquery: HANDLE,
    pcounters: Option<PPERF_COUNTER_IDENTIFIER>,
    cbcounters: DWORD,
    pcbcountersactual: LPDWORD,
) -> ULONG {
    windows_core::link!("advapi32.dll" "system" fn PerfQueryCounterInfo(hquery : HANDLE, pcounters : PPERF_COUNTER_IDENTIFIER, cbcounters : DWORD, pcbcountersactual : LPDWORD) -> ULONG);
    unsafe {
        PerfQueryCounterInfo(
            hquery,
            pcounters.unwrap_or(core::mem::zeroed()) as _,
            cbcounters,
            pcbcountersactual as _,
        )
    }
}
pub type DWORD = u32;
pub type HANDLE = *mut core::ffi::c_void;
pub type LPDWORD = *mut u32;
pub type LPVOID = *mut core::ffi::c_void;
pub type PERFLIBREQUEST = Option<
    unsafe extern "system" fn(requestcode: ULONG, buffer: PVOID, buffersize: ULONG) -> ULONG,
>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PERF_COUNTER_IDENTIFIER {
    pub CounterSetGuid: windows_core::GUID,
    pub Status: ULONG,
    pub Size: ULONG,
    pub CounterId: ULONG,
    pub InstanceId: ULONG,
    pub Index: ULONG,
    pub Reserved: ULONG,
}
pub type PERF_MEM_ALLOC =
    Option<unsafe extern "system" fn(allocsize: SIZE_T, pcontext: LPVOID) -> LPVOID>;
pub type PERF_MEM_FREE = Option<unsafe extern "system" fn(pbuffer: LPVOID, pcontext: LPVOID)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PERF_PROVIDER_CONTEXT {
    pub ContextSize: DWORD,
    pub Reserved: DWORD,
    pub ControlCallback: PERFLIBREQUEST,
    pub MemAllocRoutine: PERF_MEM_ALLOC,
    pub MemFreeRoutine: PERF_MEM_FREE,
    pub pMemContext: LPVOID,
}
pub type PPERF_COUNTER_IDENTIFIER = *mut PERF_COUNTER_IDENTIFIER;
pub type PVOID = *mut core::ffi::c_void;
pub type SIZE_T = usize;
pub type ULONG = u32;
