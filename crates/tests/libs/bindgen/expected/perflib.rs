#[inline]
pub unsafe fn PerfCloseQueryHandle(hquery: HANDLE) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfCloseQueryHandle(hquery : HANDLE) -> u32);
    unsafe { PerfCloseQueryHandle(hquery) }
}
#[inline]
pub unsafe fn PerfOpenQueryHandle<P0>(szmachine: P0, phquery: *mut HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn PerfOpenQueryHandle(szmachine : windows_core::PCWSTR, phquery : *mut HANDLE) -> u32);
    unsafe { PerfOpenQueryHandle(szmachine.param().abi(), phquery as _) }
}
#[inline]
pub unsafe fn PerfQueryCounterInfo(
    hquery: HANDLE,
    pcounters: PPERF_COUNTER_IDENTIFIER,
    cbcounters: u32,
    pcbcountersactual: LPDWORD,
) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfQueryCounterInfo(hquery : HANDLE, pcounters : PPERF_COUNTER_IDENTIFIER, cbcounters : u32, pcbcountersactual : LPDWORD) -> u32);
    unsafe { PerfQueryCounterInfo(hquery, pcounters, cbcounters, pcbcountersactual as _) }
}
pub type HANDLE = *mut core::ffi::c_void;
pub type LPDWORD = *mut u32;
pub type PERFLIBREQUEST = Option<
    unsafe extern "system" fn(
        requestcode: u32,
        buffer: *mut core::ffi::c_void,
        buffersize: u32,
    ) -> u32,
>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PERF_COUNTER_IDENTIFIER {
    pub CounterSetGuid: windows_core::GUID,
    pub Status: u32,
    pub Size: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub Index: u32,
    pub Reserved: u32,
}
pub type PERF_MEM_ALLOC = Option<
    unsafe extern "system" fn(
        allocsize: usize,
        pcontext: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void,
>;
pub type PERF_MEM_FREE = Option<
    unsafe extern "system" fn(pbuffer: *mut core::ffi::c_void, pcontext: *mut core::ffi::c_void),
>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PERF_PROVIDER_CONTEXT {
    pub ContextSize: u32,
    pub Reserved: u32,
    pub ControlCallback: PERFLIBREQUEST,
    pub MemAllocRoutine: PERF_MEM_ALLOC,
    pub MemFreeRoutine: PERF_MEM_FREE,
    pub pMemContext: *mut core::ffi::c_void,
}
pub type PPERF_COUNTER_IDENTIFIER = *mut PERF_COUNTER_IDENTIFIER;
