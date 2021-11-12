#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_System_Performance_HardwareCounterProfiling")]
pub mod HardwareCounterProfiling;
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupPerfRegistryToFileW(szfilename: super::super::Foundation::PWSTR, szcommentstring: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallPerfDllA(szcomputername: super::super::Foundation::PSTR, lpinifile: super::super::Foundation::PSTR, dwflags: usize) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallPerfDllW(szcomputername: super::super::Foundation::PWSTR, lpinifile: super::super::Foundation::PWSTR, dwflags: usize) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadPerfCounterTextStringsA(lpcommandline: super::super::Foundation::PSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadPerfCounterTextStringsW(lpcommandline: super::super::Foundation::PWSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhAddCounterA(hquery: isize, szfullcounterpath: super::super::Foundation::PSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhAddCounterW(hquery: isize, szfullcounterpath: super::super::Foundation::PWSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhAddEnglishCounterA(hquery: isize, szfullcounterpath: super::super::Foundation::PSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhAddEnglishCounterW(hquery: isize, szfullcounterpath: super::super::Foundation::PWSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBindInputDataSourceA(phdatasource: *mut isize, logfilenamelist: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBindInputDataSourceW(phdatasource: *mut isize, logfilenamelist: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBrowseCountersA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_A) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBrowseCountersHA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HA) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBrowseCountersHW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HW) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBrowseCountersW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_W) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhCalculateCounterFromRawValue(hcounter: isize, dwformat: PDH_FMT, rawvalue1: *const PDH_RAW_COUNTER, rawvalue2: *const PDH_RAW_COUNTER, fmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
    pub fn PdhCloseLog(hlog: isize, dwflags: u32) -> i32;
    pub fn PdhCloseQuery(hquery: isize) -> i32;
    pub fn PdhCollectQueryData(hquery: isize) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhCollectQueryDataEx(hquery: isize, dwintervaltime: u32, hnewdataevent: super::super::Foundation::HANDLE) -> i32;
    pub fn PdhCollectQueryDataWithTime(hquery: isize, plltimestamp: *mut i64) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhComputeCounterStatistics(hcounter: isize, dwformat: PDH_FMT, dwfirstentry: u32, dwnumentries: u32, lprawvaluearray: *const PDH_RAW_COUNTER, data: *mut PDH_STATISTICS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhConnectMachineA(szmachinename: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhConnectMachineW(szmachinename: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhCreateSQLTablesA(szdatasource: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhCreateSQLTablesW(szdatasource: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumLogSetNamesA(szdatasource: super::super::Foundation::PSTR, mszdatasetnamelist: super::super::Foundation::PSTR, pcchbufferlength: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumLogSetNamesW(szdatasource: super::super::Foundation::PWSTR, mszdatasetnamelist: super::super::Foundation::PWSTR, pcchbufferlength: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumMachinesA(szdatasource: super::super::Foundation::PSTR, mszmachinelist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumMachinesHA(hdatasource: isize, mszmachinelist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumMachinesHW(hdatasource: isize, mszmachinelist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumMachinesW(szdatasource: super::super::Foundation::PWSTR, mszmachinelist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectItemsA(szdatasource: super::super::Foundation::PSTR, szmachinename: super::super::Foundation::PSTR, szobjectname: super::super::Foundation::PSTR, mszcounterlist: super::super::Foundation::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectItemsHA(hdatasource: isize, szmachinename: super::super::Foundation::PSTR, szobjectname: super::super::Foundation::PSTR, mszcounterlist: super::super::Foundation::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectItemsHW(hdatasource: isize, szmachinename: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, mszcounterlist: super::super::Foundation::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectItemsW(szdatasource: super::super::Foundation::PWSTR, szmachinename: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, mszcounterlist: super::super::Foundation::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectsA(szdatasource: super::super::Foundation::PSTR, szmachinename: super::super::Foundation::PSTR, mszobjectlist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectsHA(hdatasource: isize, szmachinename: super::super::Foundation::PSTR, mszobjectlist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectsHW(hdatasource: isize, szmachinename: super::super::Foundation::PWSTR, mszobjectlist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectsW(szdatasource: super::super::Foundation::PWSTR, szmachinename: super::super::Foundation::PWSTR, mszobjectlist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandCounterPathA(szwildcardpath: super::super::Foundation::PSTR, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandCounterPathW(szwildcardpath: super::super::Foundation::PWSTR, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandWildCardPathA(szdatasource: super::super::Foundation::PSTR, szwildcardpath: super::super::Foundation::PSTR, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandWildCardPathHA(hdatasource: isize, szwildcardpath: super::super::Foundation::PSTR, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandWildCardPathHW(hdatasource: isize, szwildcardpath: super::super::Foundation::PWSTR, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandWildCardPathW(szdatasource: super::super::Foundation::PWSTR, szwildcardpath: super::super::Foundation::PWSTR, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhFormatFromRawValue(dwcountertype: u32, dwformat: PDH_FMT, ptimebase: *const i64, prawvalue1: *const PDH_RAW_COUNTER, prawvalue2: *const PDH_RAW_COUNTER, pfmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetCounterInfoA(hcounter: isize, bretrieveexplaintext: super::super::Foundation::BOOLEAN, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_A) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetCounterInfoW(hcounter: isize, bretrieveexplaintext: super::super::Foundation::BOOLEAN, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_W) -> i32;
    pub fn PdhGetCounterTimeBase(hcounter: isize, ptimebase: *mut i64) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDataSourceTimeRangeA(szdatasource: super::super::Foundation::PSTR, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
    pub fn PdhGetDataSourceTimeRangeH(hdatasource: isize, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDataSourceTimeRangeW(szdatasource: super::super::Foundation::PWSTR, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfCounterA(szdatasource: super::super::Foundation::PSTR, szmachinename: super::super::Foundation::PSTR, szobjectname: super::super::Foundation::PSTR, szdefaultcountername: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfCounterHA(hdatasource: isize, szmachinename: super::super::Foundation::PSTR, szobjectname: super::super::Foundation::PSTR, szdefaultcountername: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfCounterHW(hdatasource: isize, szmachinename: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, szdefaultcountername: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfCounterW(szdatasource: super::super::Foundation::PWSTR, szmachinename: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, szdefaultcountername: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfObjectA(szdatasource: super::super::Foundation::PSTR, szmachinename: super::super::Foundation::PSTR, szdefaultobjectname: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfObjectHA(hdatasource: isize, szmachinename: super::super::Foundation::PSTR, szdefaultobjectname: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfObjectHW(hdatasource: isize, szmachinename: super::super::Foundation::PWSTR, szdefaultobjectname: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfObjectW(szdatasource: super::super::Foundation::PWSTR, szmachinename: super::super::Foundation::PWSTR, szdefaultobjectname: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
    pub fn PdhGetDllVersion(lpdwversion: *mut PDH_DLL_VERSION) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetFormattedCounterArrayA(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_A) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetFormattedCounterArrayW(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_W) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetFormattedCounterValue(hcounter: isize, dwformat: PDH_FMT, lpdwtype: *mut u32, pvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
    pub fn PdhGetLogFileSize(hlog: isize, llsize: *mut i64) -> i32;
    pub fn PdhGetLogSetGUID(hlog: isize, pguid: *mut ::windows_sys::core::GUID, prunid: *mut i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetRawCounterArrayA(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_A) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetRawCounterArrayW(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_W) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetRawCounterValue(hcounter: isize, lpdwtype: *mut u32, pvalue: *mut PDH_RAW_COUNTER) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhIsRealTimeQuery(hquery: isize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhLookupPerfIndexByNameA(szmachinename: super::super::Foundation::PSTR, sznamebuffer: super::super::Foundation::PSTR, pdwindex: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhLookupPerfIndexByNameW(szmachinename: super::super::Foundation::PWSTR, sznamebuffer: super::super::Foundation::PWSTR, pdwindex: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhLookupPerfNameByIndexA(szmachinename: super::super::Foundation::PSTR, dwnameindex: u32, sznamebuffer: super::super::Foundation::PSTR, pcchnamebuffersize: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhLookupPerfNameByIndexW(szmachinename: super::super::Foundation::PWSTR, dwnameindex: u32, sznamebuffer: super::super::Foundation::PWSTR, pcchnamebuffersize: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhMakeCounterPathA(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhMakeCounterPathW(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhOpenLogA(szlogfilename: super::super::Foundation::PSTR, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: super::super::Foundation::PSTR, phlog: *mut isize) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhOpenLogW(szlogfilename: super::super::Foundation::PWSTR, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: super::super::Foundation::PWSTR, phlog: *mut isize) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhOpenQueryA(szdatasource: super::super::Foundation::PSTR, dwuserdata: usize, phquery: *mut isize) -> i32;
    pub fn PdhOpenQueryH(hdatasource: isize, dwuserdata: usize, phquery: *mut isize) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhOpenQueryW(szdatasource: super::super::Foundation::PWSTR, dwuserdata: usize, phquery: *mut isize) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhParseCounterPathA(szfullpathbuffer: super::super::Foundation::PSTR, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_A, pdwbuffersize: *mut u32, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhParseCounterPathW(szfullpathbuffer: super::super::Foundation::PWSTR, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_W, pdwbuffersize: *mut u32, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhParseInstanceNameA(szinstancestring: super::super::Foundation::PSTR, szinstancename: super::super::Foundation::PSTR, pcchinstancenamelength: *mut u32, szparentname: super::super::Foundation::PSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhParseInstanceNameW(szinstancestring: super::super::Foundation::PWSTR, szinstancename: super::super::Foundation::PWSTR, pcchinstancenamelength: *mut u32, szparentname: super::super::Foundation::PWSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhReadRawLogRecord(hlog: isize, ftrecord: super::super::Foundation::FILETIME, prawlogrecord: *mut PDH_RAW_LOG_RECORD, pdwbufferlength: *mut u32) -> i32;
    pub fn PdhRemoveCounter(hcounter: isize) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhSelectDataSourceA(hwndowner: super::super::Foundation::HWND, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: super::super::Foundation::PSTR, pcchbufferlength: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhSelectDataSourceW(hwndowner: super::super::Foundation::HWND, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: super::super::Foundation::PWSTR, pcchbufferlength: *mut u32) -> i32;
    pub fn PdhSetCounterScaleFactor(hcounter: isize, lfactor: i32) -> i32;
    pub fn PdhSetDefaultRealTimeDataSource(dwdatasourceid: REAL_TIME_DATA_SOURCE_ID_FLAGS) -> i32;
    pub fn PdhSetLogSetRunID(hlog: isize, runid: i32) -> i32;
    pub fn PdhSetQueryTimeRange(hquery: isize, pinfo: *const PDH_TIME_INFO) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhUpdateLogA(hlog: isize, szuserstring: super::super::Foundation::PSTR) -> i32;
    pub fn PdhUpdateLogFileCatalog(hlog: isize) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhUpdateLogW(hlog: isize, szuserstring: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhValidatePathA(szfullpathbuffer: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhValidatePathExA(hdatasource: isize, szfullpathbuffer: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhValidatePathExW(hdatasource: isize, szfullpathbuffer: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhValidatePathW(szfullpathbuffer: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhVerifySQLDBA(szdatasource: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhVerifySQLDBW(szdatasource: super::super::Foundation::PWSTR) -> i32;
    pub fn PerfAddCounters(hquery: PerfQueryHandle, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfCloseQueryHandle(hquery: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfCreateInstance(providerhandle: PerfProviderHandle, countersetguid: *const ::windows_sys::core::GUID, name: super::super::Foundation::PWSTR, id: u32) -> *mut PERF_COUNTERSET_INSTANCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfDecrementULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfDecrementULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
    pub fn PerfDeleteCounters(hquery: PerfQueryHandle, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32;
    pub fn PerfDeleteInstance(provider: PerfProviderHandle, instanceblock: *const PERF_COUNTERSET_INSTANCE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfEnumerateCounterSet(szmachine: super::super::Foundation::PWSTR, pcountersetids: *mut ::windows_sys::core::GUID, ccountersetids: u32, pccountersetidsactual: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfEnumerateCounterSetInstances(szmachine: super::super::Foundation::PWSTR, pcountersetid: *const ::windows_sys::core::GUID, pinstances: *mut PERF_INSTANCE_HEADER, cbinstances: u32, pcbinstancesactual: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfIncrementULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfIncrementULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfOpenQueryHandle(szmachine: super::super::Foundation::PWSTR, phquery: *mut PerfQueryHandle) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfQueryCounterData(hquery: PerfQueryHandle, pcounterblock: *mut PERF_DATA_HEADER, cbcounterblock: u32, pcbcounterblockactual: *mut u32) -> u32;
    pub fn PerfQueryCounterInfo(hquery: PerfQueryHandle, pcounters: *mut PERF_COUNTER_IDENTIFIER, cbcounters: u32, pcbcountersactual: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfQueryCounterSetRegistrationInfo(szmachine: super::super::Foundation::PWSTR, pcountersetid: *const ::windows_sys::core::GUID, requestcode: PerfRegInfoType, requestlangid: u32, pbreginfo: *mut u8, cbreginfo: u32, pcbreginfoactual: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfQueryInstance(providerhandle: super::super::Foundation::HANDLE, countersetguid: *const ::windows_sys::core::GUID, name: super::super::Foundation::PWSTR, id: u32) -> *mut PERF_COUNTERSET_INSTANCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfSetCounterRefValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, address: *const ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfSetCounterSetInfo(providerhandle: super::super::Foundation::HANDLE, template: *mut PERF_COUNTERSET_INFO, templatesize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfSetULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfSetULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
    pub fn PerfStartProvider(providerguid: *const ::windows_sys::core::GUID, controlcallback: PERFLIBREQUEST, phprovider: *mut PerfProviderHandle) -> u32;
    pub fn PerfStartProviderEx(providerguid: *const ::windows_sys::core::GUID, providercontext: *const PERF_PROVIDER_CONTEXT, provider: *mut PerfProviderHandle) -> u32;
    pub fn PerfStopProvider(providerhandle: PerfProviderHandle) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryPerformanceCounter(lpperformancecount: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryPerformanceFrequency(lpfrequency: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestorePerfRegistryFromFileW(szfilename: super::super::Foundation::PWSTR, szlangid: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetServiceAsTrustedA(szreserved: super::super::Foundation::PSTR, szservicename: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetServiceAsTrustedW(szreserved: super::super::Foundation::PWSTR, szservicename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnloadPerfCounterTextStringsA(lpcommandline: super::super::Foundation::PSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnloadPerfCounterTextStringsW(lpcommandline: super::super::Foundation::PWSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdatePerfNameFilesA(sznewctrfilepath: super::super::Foundation::PSTR, sznewhlpfilepath: super::super::Foundation::PSTR, szlanguageid: super::super::Foundation::PSTR, dwflags: usize) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdatePerfNameFilesW(sznewctrfilepath: super::super::Foundation::PWSTR, sznewhlpfilepath: super::super::Foundation::PWSTR, szlanguageid: super::super::Foundation::PWSTR, dwflags: usize) -> u32;
}
pub struct AppearPropPage(i32);
pub struct AutoPathFormat(i32);
pub struct BootTraceSession(i32);
pub struct BootTraceSessionCollection(i32);
pub struct ClockType(i32);
pub struct CommitMode(i32);
pub struct CounterItem(i32);
pub struct CounterItem2(i32);
pub struct CounterPathCallBack(i32);
pub struct CounterPropPage(i32);
pub struct Counters(i32);
#[repr(transparent)]
pub struct DICounterItem(pub *mut ::core::ffi::c_void);
pub const DIID_DICounterItem: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3230420978, data2: 3630, data3: 4559, data4: [148, 44, 0, 128, 41, 0, 67, 71] };
pub const DIID_DILogFileItem: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2366193660,
    data2: 63351,
    data3: 18711,
    data4: [130, 209, 131, 63, 188, 84, 197, 143],
};
pub const DIID_DISystemMonitor: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 332873089, data2: 49966, data3: 4559, data4: [147, 152, 0, 170, 0, 163, 221, 234] };
pub const DIID_DISystemMonitorEvents: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2224527664, data2: 19123, data3: 4559, data4: [148, 58, 0, 128, 41, 0, 67, 71] };
pub const DIID_DISystemMonitorInternal: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 424587842, data2: 49964, data3: 4559, data4: [147, 152, 0, 170, 0, 163, 221, 234] };
#[repr(transparent)]
pub struct DILogFileItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DISystemMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DISystemMonitorEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DISystemMonitorInternal(pub *mut ::core::ffi::c_void);
pub struct DataCollectorSet(i32);
pub struct DataCollectorSetCollection(i32);
pub struct DataCollectorSetStatus(i32);
pub struct DataCollectorType(i32);
pub struct DataManagerSteps(i32);
pub struct DataSourceTypeConstants(i32);
pub struct DisplayTypeConstants(i32);
pub struct FileFormat(i32);
pub struct FolderActionSteps(i32);
pub struct GeneralPropPage(i32);
pub struct GraphPropPage(i32);
pub const H_WBEM_DATASOURCE: i32 = -1i32;
#[repr(transparent)]
pub struct IAlertDataCollector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApiTracingDataCollector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConfigurationDataCollector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICounterItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICounterItem2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICounters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataCollector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataCollectorCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataCollectorSet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataCollectorSetCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFolderAction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFolderActionCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILogFileItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILogFiles(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerformanceCounterDataCollector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISchedule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduleCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemMonitor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemMonitorEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITraceDataCollector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITraceDataProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITraceDataProviderCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IValueMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IValueMapItem(pub *mut ::core::ffi::c_void);
pub const LIBID_SystemMonitor: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 460799554, data2: 9481, data3: 4559, data4: [148, 47, 0, 128, 41, 0, 67, 71] };
pub struct LegacyDataCollectorSet(i32);
pub struct LegacyDataCollectorSetCollection(i32);
pub struct LegacyTraceSession(i32);
pub struct LegacyTraceSessionCollection(i32);
pub struct LogFileItem(i32);
pub struct LogFiles(i32);
pub const MAX_COUNTER_PATH: u32 = 256u32;
pub const MAX_PERF_OBJECTS_IN_QUERY_FUNCTION: i32 = 64i32;
pub const PDH_ACCESS_DENIED: i32 = -1073738789i32;
pub const PDH_ASYNC_QUERY_TIMEOUT: i32 = -2147481637i32;
pub const PDH_BINARY_LOG_CORRUPT: i32 = -1073738761i32;
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_HA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_HW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_W(i32);
pub const PDH_CALC_NEGATIVE_DENOMINATOR: i32 = -2147481642i32;
pub const PDH_CALC_NEGATIVE_TIMEBASE: i32 = -2147481641i32;
pub const PDH_CALC_NEGATIVE_VALUE: i32 = -2147481640i32;
pub const PDH_CANNOT_CONNECT_MACHINE: i32 = -1073738813i32;
pub const PDH_CANNOT_CONNECT_WMI_SERVER: i32 = -1073738776i32;
pub const PDH_CANNOT_READ_NAME_STRINGS: i32 = -1073738808i32;
pub const PDH_CANNOT_SET_DEFAULT_REALTIME_DATASOURCE: i32 = -2147481636i32;
pub const PDH_COUNTER_ALREADY_IN_QUERY: i32 = -1073738762i32;
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_COUNTER_INFO_A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_COUNTER_INFO_W(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_COUNTER_PATH_ELEMENTS_A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_COUNTER_PATH_ELEMENTS_W(i32);
pub const PDH_CSTATUS_BAD_COUNTERNAME: i32 = -1073738816i32;
pub const PDH_CSTATUS_INVALID_DATA: i32 = -1073738822i32;
pub const PDH_CSTATUS_ITEM_NOT_VALIDATED: i32 = -2147481645i32;
pub const PDH_CSTATUS_NEW_DATA: i32 = 1i32;
pub const PDH_CSTATUS_NO_COUNTER: i32 = -1073738823i32;
pub const PDH_CSTATUS_NO_COUNTERNAME: i32 = -1073738817i32;
pub const PDH_CSTATUS_NO_INSTANCE: i32 = -2147481647i32;
pub const PDH_CSTATUS_NO_MACHINE: i32 = -2147481648i32;
pub const PDH_CSTATUS_NO_OBJECT: i32 = -1073738824i32;
pub const PDH_CSTATUS_VALID_DATA: i32 = 0i32;
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_W(i32);
pub const PDH_DATA_SOURCE_IS_LOG_FILE: i32 = -1073738802i32;
pub const PDH_DATA_SOURCE_IS_REAL_TIME: i32 = -1073738801i32;
pub const PDH_DIALOG_CANCELLED: i32 = -2147481639i32;
pub struct PDH_DLL_VERSION(i32);
pub const PDH_END_OF_LOG_FILE: i32 = -2147481638i32;
pub const PDH_ENTRY_NOT_IN_LOG_FILE: i32 = -1073738803i32;
pub const PDH_FILE_ALREADY_EXISTS: i32 = -1073738798i32;
pub const PDH_FILE_NOT_FOUND: i32 = -1073738799i32;
pub struct PDH_FMT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_FMT_COUNTERVALUE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_FMT_COUNTERVALUE_ITEM_A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_FMT_COUNTERVALUE_ITEM_W(i32);
pub const PDH_FUNCTION_NOT_FOUND: i32 = -1073738818i32;
pub const PDH_INCORRECT_APPEND_TIME: i32 = -1073738757i32;
pub const PDH_INSUFFICIENT_BUFFER: i32 = -1073738814i32;
pub const PDH_INVALID_ARGUMENT: i32 = -1073738819i32;
pub const PDH_INVALID_BUFFER: i32 = -1073738815i32;
pub const PDH_INVALID_DATA: i32 = -1073738810i32;
pub const PDH_INVALID_DATASOURCE: i32 = -1073738787i32;
pub const PDH_INVALID_HANDLE: i32 = -1073738820i32;
pub const PDH_INVALID_INSTANCE: i32 = -1073738811i32;
pub const PDH_INVALID_PATH: i32 = -1073738812i32;
pub const PDH_INVALID_SQLDB: i32 = -1073738786i32;
pub const PDH_INVALID_SQL_LOG_FORMAT: i32 = -1073738763i32;
pub struct PDH_LOG(i32);
pub const PDH_LOGSVC_NOT_OPENED: i32 = -1073738791i32;
pub const PDH_LOGSVC_QUERY_NOT_FOUND: i32 = -1073738792i32;
pub const PDH_LOG_FILE_CREATE_ERROR: i32 = -1073738807i32;
pub const PDH_LOG_FILE_OPEN_ERROR: i32 = -1073738806i32;
pub const PDH_LOG_FILE_TOO_SMALL: i32 = -1073738788i32;
pub const PDH_LOG_SAMPLE_TOO_SMALL: i32 = -1073738760i32;
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W(i32);
pub struct PDH_LOG_TYPE(i32);
pub const PDH_LOG_TYPE_NOT_FOUND: i32 = -1073738805i32;
pub const PDH_LOG_TYPE_RETIRED_BIN: u32 = 3u32;
pub const PDH_LOG_TYPE_TRACE_GENERIC: u32 = 5u32;
pub const PDH_LOG_TYPE_TRACE_KERNEL: u32 = 4u32;
pub const PDH_MAX_COUNTER_NAME: u32 = 1024u32;
pub const PDH_MAX_COUNTER_PATH: u32 = 2048u32;
pub const PDH_MAX_DATASOURCE_PATH: u32 = 1024u32;
pub const PDH_MAX_INSTANCE_NAME: u32 = 1024u32;
pub const PDH_MAX_SCALE: i32 = 7i32;
pub const PDH_MEMORY_ALLOCATION_FAILURE: i32 = -1073738821i32;
pub const PDH_MIN_SCALE: i32 = -7i32;
pub const PDH_MORE_DATA: i32 = -2147481646i32;
pub const PDH_NOEXPANDCOUNTERS: u32 = 1u32;
pub const PDH_NOEXPANDINSTANCES: u32 = 2u32;
pub const PDH_NOT_IMPLEMENTED: i32 = -1073738797i32;
pub const PDH_NO_COUNTERS: i32 = -1073738785i32;
pub const PDH_NO_DATA: i32 = -2147481643i32;
pub const PDH_NO_DIALOG_DATA: i32 = -1073738809i32;
pub const PDH_NO_MORE_DATA: i32 = -1073738804i32;
pub const PDH_OS_EARLIER_VERSION: i32 = -1073738758i32;
pub const PDH_OS_LATER_VERSION: i32 = -1073738759i32;
pub struct PDH_PATH_FLAGS(i32);
pub const PDH_PLA_COLLECTION_ALREADY_RUNNING: i32 = -1073738775i32;
pub const PDH_PLA_COLLECTION_NOT_FOUND: i32 = -1073738773i32;
pub const PDH_PLA_ERROR_ALREADY_EXISTS: i32 = -1073738770i32;
pub const PDH_PLA_ERROR_FILEPATH: i32 = -1073738768i32;
pub const PDH_PLA_ERROR_NAME_TOO_LONG: i32 = -1073738764i32;
pub const PDH_PLA_ERROR_NOSTART: i32 = -1073738771i32;
pub const PDH_PLA_ERROR_SCHEDULE_ELAPSED: i32 = -1073738772i32;
pub const PDH_PLA_ERROR_SCHEDULE_OVERLAP: i32 = -1073738774i32;
pub const PDH_PLA_ERROR_TYPE_MISMATCH: i32 = -1073738769i32;
pub const PDH_PLA_SERVICE_ERROR: i32 = -1073738767i32;
pub const PDH_PLA_VALIDATION_ERROR: i32 = -1073738766i32;
pub const PDH_PLA_VALIDATION_WARNING: i32 = -2147480589i32;
pub const PDH_QUERY_PERF_DATA_TIMEOUT: i32 = -1073738754i32;
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_RAW_COUNTER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_RAW_COUNTER_ITEM_A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_RAW_COUNTER_ITEM_W(i32);
pub struct PDH_RAW_LOG_RECORD(i32);
pub const PDH_REFRESHCOUNTERS: u32 = 4u32;
pub const PDH_RETRY: i32 = -2147481644i32;
pub struct PDH_SELECT_DATA_SOURCE_FLAGS(i32);
pub const PDH_SQL_ALLOCCON_FAILED: i32 = -1073738783i32;
pub const PDH_SQL_ALLOC_FAILED: i32 = -1073738784i32;
pub const PDH_SQL_ALTER_DETAIL_FAILED: i32 = -1073738755i32;
pub const PDH_SQL_BIND_FAILED: i32 = -1073738777i32;
pub const PDH_SQL_CONNECT_FAILED: i32 = -1073738778i32;
pub const PDH_SQL_EXEC_DIRECT_FAILED: i32 = -1073738782i32;
pub const PDH_SQL_FETCH_FAILED: i32 = -1073738781i32;
pub const PDH_SQL_MORE_RESULTS_FAILED: i32 = -1073738779i32;
pub const PDH_SQL_ROWCOUNT_FAILED: i32 = -1073738780i32;
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_STATISTICS(i32);
pub const PDH_STRING_NOT_FOUND: i32 = -1073738796i32;
pub struct PDH_TIME_INFO(i32);
pub const PDH_UNABLE_MAP_NAME_FILES: i32 = -2147480619i32;
pub const PDH_UNABLE_READ_LOG_HEADER: i32 = -1073738800i32;
pub const PDH_UNKNOWN_LOGSVC_COMMAND: i32 = -1073738793i32;
pub const PDH_UNKNOWN_LOG_FORMAT: i32 = -1073738794i32;
pub const PDH_UNMATCHED_APPEND_COUNTER: i32 = -1073738756i32;
pub const PDH_WBEM_ERROR: i32 = -1073738790i32;
pub struct PERFLIBREQUEST(i32);
pub const PERF_ADD_COUNTER: u32 = 1u32;
pub const PERF_AGGREGATE_MAX: u32 = 4u32;
pub const PERF_ATTRIB_BY_REFERENCE: u64 = 1u64;
pub const PERF_ATTRIB_DISPLAY_AS_HEX: u64 = 16u64;
pub const PERF_ATTRIB_DISPLAY_AS_REAL: u64 = 8u64;
pub const PERF_ATTRIB_NO_DISPLAYABLE: u64 = 2u64;
pub const PERF_ATTRIB_NO_GROUP_SEPARATOR: u64 = 4u64;
pub const PERF_COLLECT_END: u32 = 6u32;
pub const PERF_COLLECT_START: u32 = 5u32;
pub const PERF_COUNTERSET_FLAG_AGGREGATE: u32 = 4u32;
pub const PERF_COUNTERSET_FLAG_HISTORY: u32 = 8u32;
pub const PERF_COUNTERSET_FLAG_INSTANCE: u32 = 16u32;
pub const PERF_COUNTERSET_FLAG_MULTIPLE: u32 = 2u32;
pub struct PERF_COUNTERSET_INFO(i32);
pub struct PERF_COUNTERSET_INSTANCE(i32);
pub const PERF_COUNTERSET_MULTI_INSTANCES: u32 = 2u32;
pub struct PERF_COUNTERSET_REG_INFO(i32);
pub const PERF_COUNTERSET_SINGLE_AGGREGATE: u32 = 4u32;
pub const PERF_COUNTERSET_SINGLE_INSTANCE: u32 = 0u32;
pub struct PERF_COUNTER_AGGREGATE_FUNC(i32);
pub const PERF_COUNTER_BASE: u32 = 196608u32;
pub struct PERF_COUNTER_BLOCK(i32);
pub struct PERF_COUNTER_DATA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_COUNTER_DEFINITION(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_COUNTER_DEFINITION(i32);
pub const PERF_COUNTER_ELAPSED: u32 = 262144u32;
pub const PERF_COUNTER_FRACTION: u32 = 131072u32;
pub struct PERF_COUNTER_HEADER(i32);
pub const PERF_COUNTER_HISTOGRAM: u32 = 393216u32;
pub const PERF_COUNTER_HISTOGRAM_TYPE: u32 = 2147483648u32;
pub struct PERF_COUNTER_IDENTIFIER(i32);
pub struct PERF_COUNTER_IDENTITY(i32);
pub struct PERF_COUNTER_INFO(i32);
pub const PERF_COUNTER_PRECISION: u32 = 458752u32;
pub const PERF_COUNTER_QUEUELEN: u32 = 327680u32;
pub const PERF_COUNTER_RATE: u32 = 65536u32;
pub struct PERF_COUNTER_REG_INFO(i32);
pub const PERF_COUNTER_VALUE: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_DATA_BLOCK(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_DATA_HEADER(i32);
pub const PERF_DATA_REVISION: u32 = 1u32;
pub const PERF_DATA_VERSION: u32 = 1u32;
pub const PERF_DELTA_BASE: u32 = 8388608u32;
pub const PERF_DELTA_COUNTER: u32 = 4194304u32;
pub struct PERF_DETAIL(i32);
pub const PERF_DISPLAY_NOSHOW: u32 = 1073741824u32;
pub const PERF_DISPLAY_NO_SUFFIX: u32 = 0u32;
pub const PERF_DISPLAY_PERCENT: u32 = 536870912u32;
pub const PERF_DISPLAY_PER_SEC: u32 = 268435456u32;
pub const PERF_DISPLAY_SECONDS: u32 = 805306368u32;
pub const PERF_ENUM_INSTANCES: u32 = 3u32;
pub const PERF_FILTER: u32 = 9u32;
pub struct PERF_INSTANCE_DEFINITION(i32);
pub struct PERF_INSTANCE_HEADER(i32);
pub const PERF_INVERSE_COUNTER: u32 = 16777216u32;
pub const PERF_MAX_INSTANCE_NAME: u32 = 1024u32;
pub struct PERF_MEM_ALLOC(i32);
pub struct PERF_MEM_FREE(i32);
pub const PERF_METADATA_MULTIPLE_INSTANCES: i32 = -2i32;
pub const PERF_METADATA_NO_INSTANCES: i32 = -3i32;
pub const PERF_MULTI_COUNTER: u32 = 33554432u32;
pub struct PERF_MULTI_COUNTERS(i32);
pub struct PERF_MULTI_INSTANCES(i32);
pub const PERF_NO_INSTANCES: i32 = -1i32;
pub const PERF_NO_UNIQUE_ID: i32 = -1i32;
pub const PERF_NUMBER_DECIMAL: u32 = 65536u32;
pub const PERF_NUMBER_DEC_1000: u32 = 131072u32;
pub const PERF_NUMBER_HEX: u32 = 0u32;
pub const PERF_OBJECT_TIMER: u32 = 2097152u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_OBJECT_TYPE(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_OBJECT_TYPE(i32);
pub struct PERF_PROVIDER_CONTEXT(i32);
pub const PERF_PROVIDER_DRIVER: u32 = 2u32;
pub const PERF_PROVIDER_KERNEL_MODE: u32 = 1u32;
pub const PERF_PROVIDER_USER_MODE: u32 = 0u32;
pub const PERF_REMOVE_COUNTER: u32 = 2u32;
pub const PERF_SIZE_DWORD: u32 = 0u32;
pub const PERF_SIZE_LARGE: u32 = 256u32;
pub const PERF_SIZE_VARIABLE_LEN: u32 = 768u32;
pub const PERF_SIZE_ZERO: u32 = 512u32;
pub struct PERF_STRING_BUFFER_HEADER(i32);
pub struct PERF_STRING_COUNTER_HEADER(i32);
pub const PERF_TEXT_ASCII: u32 = 65536u32;
pub const PERF_TEXT_UNICODE: u32 = 0u32;
pub const PERF_TIMER_100NS: u32 = 1048576u32;
pub const PERF_TIMER_TICK: u32 = 0u32;
pub const PERF_TYPE_COUNTER: u32 = 1024u32;
pub const PERF_TYPE_NUMBER: u32 = 0u32;
pub const PERF_TYPE_TEXT: u32 = 2048u32;
pub const PERF_TYPE_ZERO: u32 = 3072u32;
pub const PERF_WILDCARD_COUNTER: u32 = 4294967295u32;
pub struct PLA_CABEXTRACT_CALLBACK(i32);
pub const PLA_CAPABILITY_AUTOLOGGER: u32 = 32u32;
pub const PLA_CAPABILITY_LEGACY_SESSION: u32 = 8u32;
pub const PLA_CAPABILITY_LEGACY_SVC: u32 = 16u32;
pub const PLA_CAPABILITY_LOCAL: u32 = 268435456u32;
pub const PLA_CAPABILITY_V1_SESSION: u32 = 2u32;
pub const PLA_CAPABILITY_V1_SVC: u32 = 1u32;
pub const PLA_CAPABILITY_V1_SYSTEM: u32 = 4u32;
pub struct PM_CLOSE_PROC(i32);
pub struct PM_COLLECT_PROC(i32);
pub struct PM_OPEN_PROC(i32);
pub struct PerfCounterDataType(i32);
pub struct PerfProviderHandle(i32);
pub struct PerfQueryHandle(i32);
pub struct PerfRegInfoType(i32);
pub struct REAL_TIME_DATA_SOURCE_ID_FLAGS(i32);
pub struct ReportValueTypeConstants(i32);
pub struct ResourcePolicy(i32);
pub const S_PDH: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 81159000, data2: 50337, data3: 16795, data4: [128, 35, 35, 183, 57, 2, 222, 44] };
pub struct ServerDataCollectorSet(i32);
pub struct ServerDataCollectorSetCollection(i32);
pub struct SourcePropPage(i32);
pub struct StreamMode(i32);
pub struct SysmonBatchReason(i32);
pub struct SysmonDataType(i32);
pub struct SysmonFileType(i32);
pub struct SystemDataCollectorSet(i32);
pub struct SystemDataCollectorSetCollection(i32);
pub struct SystemMonitor(i32);
pub struct SystemMonitor2(i32);
pub struct TraceDataProvider(i32);
pub struct TraceDataProviderCollection(i32);
pub struct TraceSession(i32);
pub struct TraceSessionCollection(i32);
pub struct ValueMapType(i32);
pub const WINPERF_LOG_DEBUG: u32 = 2u32;
pub const WINPERF_LOG_NONE: u32 = 0u32;
pub const WINPERF_LOG_USER: u32 = 1u32;
pub const WINPERF_LOG_VERBOSE: u32 = 3u32;
pub struct WeekDays(i32);
#[repr(transparent)]
pub struct _ICounterItemUnion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _ISystemMonitorUnion(pub *mut ::core::ffi::c_void);
