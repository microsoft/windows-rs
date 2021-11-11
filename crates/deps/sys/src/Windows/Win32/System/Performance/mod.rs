#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_System_Performance_HardwareCounterProfiling")]
pub mod HardwareCounterProfiling;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupPerfRegistryToFileW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallPerfDllA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallPerfDllW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadPerfCounterTextStringsA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadPerfCounterTextStringsW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhAddCounterA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhAddCounterW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhAddEnglishCounterA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhAddEnglishCounterW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBindInputDataSourceA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBindInputDataSourceW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBrowseCountersA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBrowseCountersHA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBrowseCountersHW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBrowseCountersW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhCalculateCounterFromRawValue();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhCloseLog();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhCloseQuery();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhCollectQueryData();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhCollectQueryDataEx();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhCollectQueryDataWithTime();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhComputeCounterStatistics();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhConnectMachineA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhConnectMachineW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhCreateSQLTablesA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhCreateSQLTablesW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumLogSetNamesA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumLogSetNamesW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumMachinesA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumMachinesHA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumMachinesHW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumMachinesW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectItemsA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectItemsHA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectItemsHW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectItemsW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectsA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectsHA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectsHW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectsW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandCounterPathA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandCounterPathW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandWildCardPathA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandWildCardPathHA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandWildCardPathHW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandWildCardPathW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhFormatFromRawValue();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetCounterInfoA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetCounterInfoW();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhGetCounterTimeBase();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDataSourceTimeRangeA();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhGetDataSourceTimeRangeH();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDataSourceTimeRangeW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfCounterA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfCounterHA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfCounterHW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfCounterW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfObjectA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfObjectHA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfObjectHW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfObjectW();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhGetDllVersion();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetFormattedCounterArrayA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetFormattedCounterArrayW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetFormattedCounterValue();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhGetLogFileSize();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhGetLogSetGUID();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetRawCounterArrayA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetRawCounterArrayW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetRawCounterValue();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhIsRealTimeQuery();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhLookupPerfIndexByNameA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhLookupPerfIndexByNameW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhLookupPerfNameByIndexA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhLookupPerfNameByIndexW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhMakeCounterPathA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhMakeCounterPathW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhOpenLogA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhOpenLogW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhOpenQueryA();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhOpenQueryH();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhOpenQueryW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhParseCounterPathA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhParseCounterPathW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhParseInstanceNameA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhParseInstanceNameW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhReadRawLogRecord();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhRemoveCounter();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhSelectDataSourceA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhSelectDataSourceW();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhSetCounterScaleFactor();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhSetDefaultRealTimeDataSource();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhSetLogSetRunID();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhSetQueryTimeRange();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhUpdateLogA();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhUpdateLogFileCatalog();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhUpdateLogW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhValidatePathA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhValidatePathExA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhValidatePathExW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhValidatePathW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhVerifySQLDBA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhVerifySQLDBW();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PerfAddCounters();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfCloseQueryHandle();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfCreateInstance();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfDecrementULongCounterValue();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfDecrementULongLongCounterValue();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PerfDeleteCounters();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PerfDeleteInstance();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfEnumerateCounterSet();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfEnumerateCounterSetInstances();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfIncrementULongCounterValue();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfIncrementULongLongCounterValue();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfOpenQueryHandle();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfQueryCounterData();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PerfQueryCounterInfo();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfQueryCounterSetRegistrationInfo();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfQueryInstance();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfSetCounterRefValue();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfSetCounterSetInfo();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfSetULongCounterValue();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfSetULongLongCounterValue();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PerfStartProvider();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PerfStartProviderEx();
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PerfStopProvider();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryPerformanceCounter();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryPerformanceFrequency();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestorePerfRegistryFromFileW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetServiceAsTrustedA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetServiceAsTrustedW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnloadPerfCounterTextStringsA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnloadPerfCounterTextStringsW();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdatePerfNameFilesA();
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdatePerfNameFilesW();
}
