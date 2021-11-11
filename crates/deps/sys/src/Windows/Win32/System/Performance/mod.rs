#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_System_Performance_HardwareCounterProfiling")]
pub mod HardwareCounterProfiling;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupPerfRegistryToFileW(szfilename: super::super::Foundation::PWSTR, szcommentstring: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallPerfDllA(szcomputername: super::super::Foundation::PSTR, lpinifile: super::super::Foundation::PSTR, dwflags: usize) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallPerfDllW(szcomputername: super::super::Foundation::PWSTR, lpinifile: super::super::Foundation::PWSTR, dwflags: usize) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadPerfCounterTextStringsA(lpcommandline: super::super::Foundation::PSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadPerfCounterTextStringsW(lpcommandline: super::super::Foundation::PWSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhAddCounterA(hquery: isize, szfullcounterpath: super::super::Foundation::PSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhAddCounterW(hquery: isize, szfullcounterpath: super::super::Foundation::PWSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhAddEnglishCounterA(hquery: isize, szfullcounterpath: super::super::Foundation::PSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhAddEnglishCounterW(hquery: isize, szfullcounterpath: super::super::Foundation::PWSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBindInputDataSourceA(phdatasource: *mut isize, logfilenamelist: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBindInputDataSourceW(phdatasource: *mut isize, logfilenamelist: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBrowseCountersA(pbrowsedlgdata: *const ::core::mem::ManuallyDrop<PDH_BROWSE_DLG_CONFIG_A>) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBrowseCountersHA(pbrowsedlgdata: *const ::core::mem::ManuallyDrop<PDH_BROWSE_DLG_CONFIG_HA>) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBrowseCountersHW(pbrowsedlgdata: *const ::core::mem::ManuallyDrop<PDH_BROWSE_DLG_CONFIG_HW>) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhBrowseCountersW(pbrowsedlgdata: *const ::core::mem::ManuallyDrop<PDH_BROWSE_DLG_CONFIG_W>) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhCalculateCounterFromRawValue(hcounter: isize, dwformat: PDH_FMT, rawvalue1: *const PDH_RAW_COUNTER, rawvalue2: *const PDH_RAW_COUNTER, fmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhCloseLog(hlog: isize, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhCloseQuery(hquery: isize) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhCollectQueryData(hquery: isize) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhCollectQueryDataEx(hquery: isize, dwintervaltime: u32, hnewdataevent: super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhCollectQueryDataWithTime(hquery: isize, plltimestamp: *mut i64) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhComputeCounterStatistics(hcounter: isize, dwformat: PDH_FMT, dwfirstentry: u32, dwnumentries: u32, lprawvaluearray: *const PDH_RAW_COUNTER, data: *mut PDH_STATISTICS) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhConnectMachineA(szmachinename: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhConnectMachineW(szmachinename: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhCreateSQLTablesA(szdatasource: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhCreateSQLTablesW(szdatasource: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumLogSetNamesA(szdatasource: super::super::Foundation::PSTR, mszdatasetnamelist: super::super::Foundation::PSTR, pcchbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumLogSetNamesW(szdatasource: super::super::Foundation::PWSTR, mszdatasetnamelist: super::super::Foundation::PWSTR, pcchbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumMachinesA(szdatasource: super::super::Foundation::PSTR, mszmachinelist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumMachinesHA(hdatasource: isize, mszmachinelist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumMachinesHW(hdatasource: isize, mszmachinelist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumMachinesW(szdatasource: super::super::Foundation::PWSTR, mszmachinelist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectItemsA(szdatasource: super::super::Foundation::PSTR, szmachinename: super::super::Foundation::PSTR, szobjectname: super::super::Foundation::PSTR, mszcounterlist: super::super::Foundation::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectItemsHA(hdatasource: isize, szmachinename: super::super::Foundation::PSTR, szobjectname: super::super::Foundation::PSTR, mszcounterlist: super::super::Foundation::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectItemsHW(hdatasource: isize, szmachinename: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, mszcounterlist: super::super::Foundation::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectItemsW(szdatasource: super::super::Foundation::PWSTR, szmachinename: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, mszcounterlist: super::super::Foundation::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectsA(szdatasource: super::super::Foundation::PSTR, szmachinename: super::super::Foundation::PSTR, mszobjectlist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectsHA(hdatasource: isize, szmachinename: super::super::Foundation::PSTR, mszobjectlist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectsHW(hdatasource: isize, szmachinename: super::super::Foundation::PWSTR, mszobjectlist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhEnumObjectsW(szdatasource: super::super::Foundation::PWSTR, szmachinename: super::super::Foundation::PWSTR, mszobjectlist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandCounterPathA(szwildcardpath: super::super::Foundation::PSTR, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandCounterPathW(szwildcardpath: super::super::Foundation::PWSTR, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandWildCardPathA(szdatasource: super::super::Foundation::PSTR, szwildcardpath: super::super::Foundation::PSTR, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandWildCardPathHA(hdatasource: isize, szwildcardpath: super::super::Foundation::PSTR, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandWildCardPathHW(hdatasource: isize, szwildcardpath: super::super::Foundation::PWSTR, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhExpandWildCardPathW(szdatasource: super::super::Foundation::PWSTR, szwildcardpath: super::super::Foundation::PWSTR, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhFormatFromRawValue(dwcountertype: u32, dwformat: PDH_FMT, ptimebase: *const i64, prawvalue1: *const PDH_RAW_COUNTER, prawvalue2: *const PDH_RAW_COUNTER, pfmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetCounterInfoA(hcounter: isize, bretrieveexplaintext: super::super::Foundation::BOOLEAN, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_A) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetCounterInfoW(hcounter: isize, bretrieveexplaintext: super::super::Foundation::BOOLEAN, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_W) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhGetCounterTimeBase(hcounter: isize, ptimebase: *mut i64) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDataSourceTimeRangeA(szdatasource: super::super::Foundation::PSTR, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhGetDataSourceTimeRangeH(hdatasource: isize, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDataSourceTimeRangeW(szdatasource: super::super::Foundation::PWSTR, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfCounterA(szdatasource: super::super::Foundation::PSTR, szmachinename: super::super::Foundation::PSTR, szobjectname: super::super::Foundation::PSTR, szdefaultcountername: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfCounterHA(hdatasource: isize, szmachinename: super::super::Foundation::PSTR, szobjectname: super::super::Foundation::PSTR, szdefaultcountername: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfCounterHW(hdatasource: isize, szmachinename: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, szdefaultcountername: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfCounterW(szdatasource: super::super::Foundation::PWSTR, szmachinename: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, szdefaultcountername: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfObjectA(szdatasource: super::super::Foundation::PSTR, szmachinename: super::super::Foundation::PSTR, szdefaultobjectname: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfObjectHA(hdatasource: isize, szmachinename: super::super::Foundation::PSTR, szdefaultobjectname: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfObjectHW(hdatasource: isize, szmachinename: super::super::Foundation::PWSTR, szdefaultobjectname: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetDefaultPerfObjectW(szdatasource: super::super::Foundation::PWSTR, szmachinename: super::super::Foundation::PWSTR, szdefaultobjectname: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhGetDllVersion(lpdwversion: *mut PDH_DLL_VERSION) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetFormattedCounterArrayA(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_A) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetFormattedCounterArrayW(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_W) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetFormattedCounterValue(hcounter: isize, dwformat: PDH_FMT, lpdwtype: *mut u32, pvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhGetLogFileSize(hlog: isize, llsize: *mut i64) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhGetLogSetGUID(hlog: isize, pguid: *mut ::windows::runtime::GUID, prunid: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetRawCounterArrayA(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_A) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetRawCounterArrayW(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_W) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhGetRawCounterValue(hcounter: isize, lpdwtype: *mut u32, pvalue: *mut PDH_RAW_COUNTER) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhIsRealTimeQuery(hquery: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhLookupPerfIndexByNameA(szmachinename: super::super::Foundation::PSTR, sznamebuffer: super::super::Foundation::PSTR, pdwindex: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhLookupPerfIndexByNameW(szmachinename: super::super::Foundation::PWSTR, sznamebuffer: super::super::Foundation::PWSTR, pdwindex: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhLookupPerfNameByIndexA(szmachinename: super::super::Foundation::PSTR, dwnameindex: u32, sznamebuffer: super::super::Foundation::PSTR, pcchnamebuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhLookupPerfNameByIndexW(szmachinename: super::super::Foundation::PWSTR, dwnameindex: u32, sznamebuffer: super::super::Foundation::PWSTR, pcchnamebuffersize: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhMakeCounterPathA(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhMakeCounterPathW(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhOpenLogA(szlogfilename: super::super::Foundation::PSTR, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: super::super::Foundation::PSTR, phlog: *mut isize) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhOpenLogW(szlogfilename: super::super::Foundation::PWSTR, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: super::super::Foundation::PWSTR, phlog: *mut isize) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhOpenQueryA(szdatasource: super::super::Foundation::PSTR, dwuserdata: usize, phquery: *mut isize) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhOpenQueryH(hdatasource: isize, dwuserdata: usize, phquery: *mut isize) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhOpenQueryW(szdatasource: super::super::Foundation::PWSTR, dwuserdata: usize, phquery: *mut isize) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhParseCounterPathA(szfullpathbuffer: super::super::Foundation::PSTR, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_A, pdwbuffersize: *mut u32, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhParseCounterPathW(szfullpathbuffer: super::super::Foundation::PWSTR, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_W, pdwbuffersize: *mut u32, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhParseInstanceNameA(szinstancestring: super::super::Foundation::PSTR, szinstancename: super::super::Foundation::PSTR, pcchinstancenamelength: *mut u32, szparentname: super::super::Foundation::PSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhParseInstanceNameW(szinstancestring: super::super::Foundation::PWSTR, szinstancename: super::super::Foundation::PWSTR, pcchinstancenamelength: *mut u32, szparentname: super::super::Foundation::PWSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhReadRawLogRecord(hlog: isize, ftrecord: super::super::Foundation::FILETIME, prawlogrecord: *mut PDH_RAW_LOG_RECORD, pdwbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhRemoveCounter(hcounter: isize) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhSelectDataSourceA(hwndowner: super::super::Foundation::HWND, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: super::super::Foundation::PSTR, pcchbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhSelectDataSourceW(hwndowner: super::super::Foundation::HWND, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: super::super::Foundation::PWSTR, pcchbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhSetCounterScaleFactor(hcounter: isize, lfactor: i32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhSetDefaultRealTimeDataSource(dwdatasourceid: REAL_TIME_DATA_SOURCE_ID_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhSetLogSetRunID(hlog: isize, runid: i32) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhSetQueryTimeRange(hquery: isize, pinfo: *const PDH_TIME_INFO) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhUpdateLogA(hlog: isize, szuserstring: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PdhUpdateLogFileCatalog(hlog: isize) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhUpdateLogW(hlog: isize, szuserstring: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhValidatePathA(szfullpathbuffer: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhValidatePathExA(hdatasource: isize, szfullpathbuffer: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhValidatePathExW(hdatasource: isize, szfullpathbuffer: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhValidatePathW(szfullpathbuffer: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhVerifySQLDBA(szdatasource: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PdhVerifySQLDBW(szdatasource: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PerfAddCounters(hquery: PerfQueryHandle, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfCloseQueryHandle(hquery: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfCreateInstance(providerhandle: PerfProviderHandle, countersetguid: *const ::windows::runtime::GUID, name: super::super::Foundation::PWSTR, id: u32) -> *mut PERF_COUNTERSET_INSTANCE;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfDecrementULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfDecrementULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PerfDeleteCounters(hquery: PerfQueryHandle, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PerfDeleteInstance(provider: PerfProviderHandle, instanceblock: *const PERF_COUNTERSET_INSTANCE) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfEnumerateCounterSet(szmachine: super::super::Foundation::PWSTR, pcountersetids: *mut ::windows::runtime::GUID, ccountersetids: u32, pccountersetidsactual: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfEnumerateCounterSetInstances(szmachine: super::super::Foundation::PWSTR, pcountersetid: *const ::windows::runtime::GUID, pinstances: *mut PERF_INSTANCE_HEADER, cbinstances: u32, pcbinstancesactual: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfIncrementULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfIncrementULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfOpenQueryHandle(szmachine: super::super::Foundation::PWSTR, phquery: *mut PerfQueryHandle) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfQueryCounterData(hquery: PerfQueryHandle, pcounterblock: *mut PERF_DATA_HEADER, cbcounterblock: u32, pcbcounterblockactual: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PerfQueryCounterInfo(hquery: PerfQueryHandle, pcounters: *mut PERF_COUNTER_IDENTIFIER, cbcounters: u32, pcbcountersactual: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfQueryCounterSetRegistrationInfo(szmachine: super::super::Foundation::PWSTR, pcountersetid: *const ::windows::runtime::GUID, requestcode: PerfRegInfoType, requestlangid: u32, pbreginfo: *mut u8, cbreginfo: u32, pcbreginfoactual: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfQueryInstance(providerhandle: super::super::Foundation::HANDLE, countersetguid: *const ::windows::runtime::GUID, name: super::super::Foundation::PWSTR, id: u32) -> *mut PERF_COUNTERSET_INSTANCE;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfSetCounterRefValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, address: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfSetCounterSetInfo(providerhandle: super::super::Foundation::HANDLE, template: *mut PERF_COUNTERSET_INFO, templatesize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfSetULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PerfSetULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PerfStartProvider(providerguid: *const ::windows::runtime::GUID, controlcallback: ::windows::runtime::RawPtr, phprovider: *mut PerfProviderHandle) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PerfStartProviderEx(providerguid: *const ::windows::runtime::GUID, providercontext: *const ::core::mem::ManuallyDrop<PERF_PROVIDER_CONTEXT>, provider: *mut PerfProviderHandle) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`*"]
    pub fn PerfStopProvider(providerhandle: PerfProviderHandle) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryPerformanceCounter(lpperformancecount: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryPerformanceFrequency(lpfrequency: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestorePerfRegistryFromFileW(szfilename: super::super::Foundation::PWSTR, szlangid: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetServiceAsTrustedA(szreserved: super::super::Foundation::PSTR, szservicename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetServiceAsTrustedW(szreserved: super::super::Foundation::PWSTR, szservicename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnloadPerfCounterTextStringsA(lpcommandline: super::super::Foundation::PSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnloadPerfCounterTextStringsW(lpcommandline: super::super::Foundation::PWSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdatePerfNameFilesA(sznewctrfilepath: super::super::Foundation::PSTR, sznewhlpfilepath: super::super::Foundation::PSTR, szlanguageid: super::super::Foundation::PSTR, dwflags: usize) -> u32;
    #[doc = "*Required features: `Win32_System_Performance`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdatePerfNameFilesW(sznewctrfilepath: super::super::Foundation::PWSTR, sznewhlpfilepath: super::super::Foundation::PWSTR, szlanguageid: super::super::Foundation::PWSTR, dwflags: usize) -> u32;
}
