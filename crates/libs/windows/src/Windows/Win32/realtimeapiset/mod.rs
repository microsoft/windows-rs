#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue: super::ULONGLONG, lpperformancecountervalue: super::PULONGLONG, lpconversionerror: Option<super::PULONGLONG>) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue : super::ULONGLONG, lpperformancecountervalue : super::PULONGLONG, lpconversionerror : super::PULONGLONG) -> windows_core::HRESULT);
    unsafe { ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue, lpperformancecountervalue as _, lpconversionerror.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue: super::ULONGLONG, lpauxiliarycountervalue: super::PULONGLONG, lpconversionerror: Option<super::PULONGLONG>) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue : super::ULONGLONG, lpauxiliarycountervalue : super::PULONGLONG, lpconversionerror : super::PULONGLONG) -> windows_core::HRESULT);
    unsafe { ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue, lpauxiliarycountervalue as _, lpconversionerror.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryAuxiliaryCounterFrequency(lpauxiliarycounterfrequency: super::PULONGLONG) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn QueryAuxiliaryCounterFrequency(lpauxiliarycounterfrequency : super::PULONGLONG) -> windows_core::HRESULT);
    unsafe { QueryAuxiliaryCounterFrequency(lpauxiliarycounterfrequency as _) }
}
#[cfg(all(feature = "basetsd", feature = "minwindef"))]
#[inline]
pub unsafe fn QueryIdleProcessorCycleTime(bufferlength: super::PULONG, processoridlecycletime: Option<super::PULONG64>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryIdleProcessorCycleTime(bufferlength : super::PULONG, processoridlecycletime : super::PULONG64) -> windows_core::BOOL);
    unsafe { QueryIdleProcessorCycleTime(bufferlength as _, processoridlecycletime.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn QueryIdleProcessorCycleTimeEx(group: super::USHORT, bufferlength: super::PULONG, processoridlecycletime: Option<super::PULONG64>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryIdleProcessorCycleTimeEx(group : super::USHORT, bufferlength : super::PULONG, processoridlecycletime : super::PULONG64) -> windows_core::BOOL);
    unsafe { QueryIdleProcessorCycleTimeEx(group, bufferlength as _, processoridlecycletime.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryInterruptTime(lpinterrupttime: super::PULONGLONG) {
    windows_core::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryInterruptTime(lpinterrupttime : super::PULONGLONG));
    unsafe { QueryInterruptTime(lpinterrupttime as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryInterruptTimePrecise(lpinterrupttimeprecise: super::PULONGLONG) {
    windows_core::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryInterruptTimePrecise(lpinterrupttimeprecise : super::PULONGLONG));
    unsafe { QueryInterruptTimePrecise(lpinterrupttimeprecise as _) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[inline]
pub unsafe fn QueryProcessCycleTime(processhandle: super::HANDLE, cycletime: super::PULONG64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryProcessCycleTime(processhandle : super::HANDLE, cycletime : super::PULONG64) -> windows_core::BOOL);
    unsafe { QueryProcessCycleTime(processhandle, cycletime as _) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[inline]
pub unsafe fn QueryThreadCycleTime(threadhandle: super::HANDLE, cycletime: super::PULONG64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryThreadCycleTime(threadhandle : super::HANDLE, cycletime : super::PULONG64) -> windows_core::BOOL);
    unsafe { QueryThreadCycleTime(threadhandle, cycletime as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryUnbiasedInterruptTime(unbiasedtime: super::PULONGLONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryUnbiasedInterruptTime(unbiasedtime : super::PULONGLONG) -> windows_core::BOOL);
    unsafe { QueryUnbiasedInterruptTime(unbiasedtime as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryUnbiasedInterruptTimePrecise(lpunbiasedinterrupttimeprecise: super::PULONGLONG) {
    windows_core::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryUnbiasedInterruptTimePrecise(lpunbiasedinterrupttimeprecise : super::PULONGLONG));
    unsafe { QueryUnbiasedInterruptTimePrecise(lpunbiasedinterrupttimeprecise as _) }
}
