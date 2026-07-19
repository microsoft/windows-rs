#[inline]
pub unsafe fn ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue: u64, lpperformancecountervalue: *mut u64, lpconversionerror: Option<*mut u64>) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue : u64, lpperformancecountervalue : *mut u64, lpconversionerror : *mut u64) -> windows_core::HRESULT);
    unsafe { ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue, lpperformancecountervalue as _, lpconversionerror.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue: u64, lpauxiliarycountervalue: *mut u64, lpconversionerror: Option<*mut u64>) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue : u64, lpauxiliarycountervalue : *mut u64, lpconversionerror : *mut u64) -> windows_core::HRESULT);
    unsafe { ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue, lpauxiliarycountervalue as _, lpconversionerror.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn QueryAuxiliaryCounterFrequency() -> windows_core::Result<u64> {
    windows_core::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn QueryAuxiliaryCounterFrequency(lpauxiliarycounterfrequency : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        QueryAuxiliaryCounterFrequency(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn QueryIdleProcessorCycleTime(bufferlength: *mut u32, processoridlecycletime: Option<*mut u64>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryIdleProcessorCycleTime(bufferlength : *mut u32, processoridlecycletime : *mut u64) -> windows_core::BOOL);
    unsafe { QueryIdleProcessorCycleTime(bufferlength as _, processoridlecycletime.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn QueryIdleProcessorCycleTimeEx(group: u16, bufferlength: *mut u32, processoridlecycletime: Option<*mut u64>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryIdleProcessorCycleTimeEx(group : u16, bufferlength : *mut u32, processoridlecycletime : *mut u64) -> windows_core::BOOL);
    unsafe { QueryIdleProcessorCycleTimeEx(group, bufferlength as _, processoridlecycletime.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn QueryInterruptTime() -> u64 {
    windows_core::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryInterruptTime(lpinterrupttime : *mut u64));
    unsafe {
        let mut result__ = core::mem::zeroed();
        QueryInterruptTime(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn QueryInterruptTimePrecise() -> u64 {
    windows_core::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryInterruptTimePrecise(lpinterrupttimeprecise : *mut u64));
    unsafe {
        let mut result__ = core::mem::zeroed();
        QueryInterruptTimePrecise(&mut result__);
        result__
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryProcessCycleTime(processhandle: super::HANDLE, cycletime: *mut u64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryProcessCycleTime(processhandle : super::HANDLE, cycletime : *mut u64) -> windows_core::BOOL);
    unsafe { QueryProcessCycleTime(processhandle, cycletime as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryThreadCycleTime(threadhandle: super::HANDLE, cycletime: *mut u64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryThreadCycleTime(threadhandle : super::HANDLE, cycletime : *mut u64) -> windows_core::BOOL);
    unsafe { QueryThreadCycleTime(threadhandle, cycletime as _) }
}
#[inline]
pub unsafe fn QueryUnbiasedInterruptTime(unbiasedtime: *mut u64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryUnbiasedInterruptTime(unbiasedtime : *mut u64) -> windows_core::BOOL);
    unsafe { QueryUnbiasedInterruptTime(unbiasedtime as _) }
}
#[inline]
pub unsafe fn QueryUnbiasedInterruptTimePrecise() -> u64 {
    windows_core::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryUnbiasedInterruptTimePrecise(lpunbiasedinterrupttimeprecise : *mut u64));
    unsafe {
        let mut result__ = core::mem::zeroed();
        QueryUnbiasedInterruptTimePrecise(&mut result__);
        result__
    }
}
