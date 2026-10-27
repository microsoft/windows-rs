#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue : super::ULONGLONG, lpperformancecountervalue : super::PULONGLONG, lpconversionerror : super::PULONGLONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue : super::ULONGLONG, lpauxiliarycountervalue : super::PULONGLONG, lpconversionerror : super::PULONGLONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn QueryAuxiliaryCounterFrequency(lpauxiliarycounterfrequency : super::PULONGLONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "basetsd", feature = "minwindef"))]
windows_link::link!("kernel32.dll" "system" fn QueryIdleProcessorCycleTime(bufferlength : super::PULONG, processoridlecycletime : super::PULONG64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn QueryIdleProcessorCycleTimeEx(group : super::USHORT, bufferlength : super::PULONG, processoridlecycletime : super::PULONG64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryInterruptTime(lpinterrupttime : super::PULONGLONG));
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryInterruptTimePrecise(lpinterrupttimeprecise : super::PULONGLONG));
#[cfg(all(feature = "basetsd", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn QueryProcessCycleTime(processhandle : super::HANDLE, cycletime : super::PULONG64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn QueryThreadCycleTime(threadhandle : super::HANDLE, cycletime : super::PULONG64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryUnbiasedInterruptTime(unbiasedtime : super::PULONGLONG) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryUnbiasedInterruptTimePrecise(lpunbiasedinterrupttimeprecise : super::PULONGLONG));
