windows_link::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue : u64, lpperformancecountervalue : *mut u64, lpconversionerror : *mut u64) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue : u64, lpauxiliarycountervalue : *mut u64, lpconversionerror : *mut u64) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-realtime-l1-1-2.dll" "system" fn QueryAuxiliaryCounterFrequency(lpauxiliarycounterfrequency : *mut u64) -> windows_sys::core::HRESULT);
windows_link::link!("kernel32.dll" "system" fn QueryIdleProcessorCycleTime(bufferlength : *mut u32, processoridlecycletime : *mut u64) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn QueryIdleProcessorCycleTimeEx(group : u16, bufferlength : *mut u32, processoridlecycletime : *mut u64) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryInterruptTime(lpinterrupttime : *mut u64));
windows_link::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryInterruptTimePrecise(lpinterrupttimeprecise : *mut u64));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryProcessCycleTime(processhandle : super::winnt::HANDLE, cycletime : *mut u64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryThreadCycleTime(threadhandle : super::winnt::HANDLE, cycletime : *mut u64) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn QueryUnbiasedInterruptTime(unbiasedtime : *mut u64) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-realtime-l1-1-1.dll" "system" fn QueryUnbiasedInterruptTimePrecise(lpunbiasedinterrupttimeprecise : *mut u64));
