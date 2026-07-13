#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhAddCounterA(hquery : PDH_HQUERY, szfullcounterpath : windows_sys::core::PCSTR, dwuserdata : usize, phcounter : *mut PDH_HCOUNTER) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhAddCounterW(hquery : PDH_HQUERY, szfullcounterpath : windows_sys::core::PCWSTR, dwuserdata : usize, phcounter : *mut PDH_HCOUNTER) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhAddEnglishCounterA(hquery : PDH_HQUERY, szfullcounterpath : windows_sys::core::PCSTR, dwuserdata : usize, phcounter : *mut PDH_HCOUNTER) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhAddEnglishCounterW(hquery : PDH_HQUERY, szfullcounterpath : windows_sys::core::PCWSTR, dwuserdata : usize, phcounter : *mut PDH_HCOUNTER) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhBindInputDataSourceA(phdatasource : *mut PDH_HLOG, logfilenamelist : windows_sys::core::PCSTR) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhBindInputDataSourceW(phdatasource : *mut PDH_HLOG, logfilenamelist : windows_sys::core::PCWSTR) -> PDH_STATUS);
#[cfg(feature = "windef")]
windows_link::link!("pdh.dll" "system" fn PdhBrowseCountersA(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_A) -> PDH_STATUS);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhBrowseCountersHA(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_HA) -> PDH_STATUS);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhBrowseCountersHW(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_HW) -> PDH_STATUS);
#[cfg(feature = "windef")]
windows_link::link!("pdh.dll" "system" fn PdhBrowseCountersW(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_W) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhCalculateCounterFromRawValue(hcounter : PDH_HCOUNTER, dwformat : u32, rawvalue1 : *const PDH_RAW_COUNTER, rawvalue2 : *const PDH_RAW_COUNTER, fmtvalue : *mut PDH_FMT_COUNTERVALUE) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhCloseLog(hlog : PDH_HLOG, dwflags : u32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhCloseQuery(hquery : PDH_HQUERY) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhCollectQueryData(hquery : PDH_HQUERY) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhCollectQueryDataEx(hquery : PDH_HQUERY, dwintervaltime : u32, hnewdataevent : super::winnt::HANDLE) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhCollectQueryDataWithTime(hquery : PDH_HQUERY, plltimestamp : *mut i64) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhComputeCounterStatistics(hcounter : PDH_HCOUNTER, dwformat : u32, dwfirstentry : u32, dwnumentries : u32, lprawvaluearray : *const PDH_RAW_COUNTER, data : *mut PDH_STATISTICS) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhConnectMachineA(szmachinename : windows_sys::core::PCSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhConnectMachineW(szmachinename : windows_sys::core::PCWSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhCreateSQLTablesA(szdatasource : windows_sys::core::PCSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhCreateSQLTablesW(szdatasource : windows_sys::core::PCWSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhEnumLogSetNamesA(szdatasource : windows_sys::core::PCSTR, mszdatasetnamelist : *mut i8, pcchbufferlength : *mut u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhEnumLogSetNamesW(szdatasource : windows_sys::core::PCWSTR, mszdatasetnamelist : *mut u16, pcchbufferlength : *mut u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhEnumMachinesA(szdatasource : windows_sys::core::PCSTR, mszmachinelist : *mut i8, pcchbuffersize : *mut u32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhEnumMachinesHA(hdatasource : PDH_HLOG, mszmachinelist : *mut i8, pcchbuffersize : *mut u32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhEnumMachinesHW(hdatasource : PDH_HLOG, mszmachinelist : *mut u16, pcchbuffersize : *mut u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhEnumMachinesW(szdatasource : windows_sys::core::PCWSTR, mszmachinelist : *mut u16, pcchbuffersize : *mut u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhEnumObjectItemsA(szdatasource : windows_sys::core::PCSTR, szmachinename : windows_sys::core::PCSTR, szobjectname : windows_sys::core::PCSTR, mszcounterlist : *mut i8, pcchcounterlistlength : *mut u32, mszinstancelist : *mut i8, pcchinstancelistlength : *mut u32, dwdetaillevel : u32, dwflags : u32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhEnumObjectItemsHA(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCSTR, szobjectname : windows_sys::core::PCSTR, mszcounterlist : *mut i8, pcchcounterlistlength : *mut u32, mszinstancelist : *mut i8, pcchinstancelistlength : *mut u32, dwdetaillevel : u32, dwflags : u32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhEnumObjectItemsHW(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCWSTR, szobjectname : windows_sys::core::PCWSTR, mszcounterlist : *mut u16, pcchcounterlistlength : *mut u32, mszinstancelist : *mut u16, pcchinstancelistlength : *mut u32, dwdetaillevel : u32, dwflags : u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhEnumObjectItemsW(szdatasource : windows_sys::core::PCWSTR, szmachinename : windows_sys::core::PCWSTR, szobjectname : windows_sys::core::PCWSTR, mszcounterlist : *mut u16, pcchcounterlistlength : *mut u32, mszinstancelist : *mut u16, pcchinstancelistlength : *mut u32, dwdetaillevel : u32, dwflags : u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhEnumObjectsA(szdatasource : windows_sys::core::PCSTR, szmachinename : windows_sys::core::PCSTR, mszobjectlist : *mut i8, pcchbuffersize : *mut u32, dwdetaillevel : u32, brefresh : windows_sys::core::BOOL) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhEnumObjectsHA(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCSTR, mszobjectlist : *mut i8, pcchbuffersize : *mut u32, dwdetaillevel : u32, brefresh : windows_sys::core::BOOL) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhEnumObjectsHW(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCWSTR, mszobjectlist : *mut u16, pcchbuffersize : *mut u32, dwdetaillevel : u32, brefresh : windows_sys::core::BOOL) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhEnumObjectsW(szdatasource : windows_sys::core::PCWSTR, szmachinename : windows_sys::core::PCWSTR, mszobjectlist : *mut u16, pcchbuffersize : *mut u32, dwdetaillevel : u32, brefresh : windows_sys::core::BOOL) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhExpandCounterPathA(szwildcardpath : windows_sys::core::PCSTR, mszexpandedpathlist : *mut i8, pcchpathlistlength : *mut u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhExpandCounterPathW(szwildcardpath : windows_sys::core::PCWSTR, mszexpandedpathlist : *mut u16, pcchpathlistlength : *mut u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhExpandWildCardPathA(szdatasource : windows_sys::core::PCSTR, szwildcardpath : windows_sys::core::PCSTR, mszexpandedpathlist : *mut i8, pcchpathlistlength : *mut u32, dwflags : u32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhExpandWildCardPathHA(hdatasource : PDH_HLOG, szwildcardpath : windows_sys::core::PCSTR, mszexpandedpathlist : *mut i8, pcchpathlistlength : *mut u32, dwflags : u32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhExpandWildCardPathHW(hdatasource : PDH_HLOG, szwildcardpath : windows_sys::core::PCWSTR, mszexpandedpathlist : *mut u16, pcchpathlistlength : *mut u32, dwflags : u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhExpandWildCardPathW(szdatasource : windows_sys::core::PCWSTR, szwildcardpath : windows_sys::core::PCWSTR, mszexpandedpathlist : *mut u16, pcchpathlistlength : *mut u32, dwflags : u32) -> PDH_STATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("pdh.dll" "system" fn PdhFormatFromRawValue(dwcountertype : u32, dwformat : u32, ptimebase : *const i64, prawvalue1 : *const PDH_RAW_COUNTER, prawvalue2 : *const PDH_RAW_COUNTER, pfmtvalue : *mut PDH_FMT_COUNTERVALUE) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhGetCounterInfoA(hcounter : PDH_HCOUNTER, bretrieveexplaintext : bool, pdwbuffersize : *mut u32, lpbuffer : *mut PDH_COUNTER_INFO_A) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhGetCounterInfoW(hcounter : PDH_HCOUNTER, bretrieveexplaintext : bool, pdwbuffersize : *mut u32, lpbuffer : *mut PDH_COUNTER_INFO_W) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhGetCounterTimeBase(hcounter : PDH_HCOUNTER, ptimebase : *mut i64) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeA(szdatasource : windows_sys::core::PCSTR, pdwnumentries : *mut u32, pinfo : *mut PDH_TIME_INFO, pdwbuffersize : *mut u32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeH(hdatasource : PDH_HLOG, pdwnumentries : *mut u32, pinfo : *mut PDH_TIME_INFO, pdwbuffersize : *mut u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeW(szdatasource : windows_sys::core::PCWSTR, pdwnumentries : *mut u32, pinfo : *mut PDH_TIME_INFO, pdwbuffersize : *mut u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterA(szdatasource : windows_sys::core::PCSTR, szmachinename : windows_sys::core::PCSTR, szobjectname : windows_sys::core::PCSTR, szdefaultcountername : windows_sys::core::PSTR, pcchbuffersize : *mut u32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterHA(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCSTR, szobjectname : windows_sys::core::PCSTR, szdefaultcountername : windows_sys::core::PSTR, pcchbuffersize : *mut u32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterHW(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCWSTR, szobjectname : windows_sys::core::PCWSTR, szdefaultcountername : windows_sys::core::PWSTR, pcchbuffersize : *mut u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterW(szdatasource : windows_sys::core::PCWSTR, szmachinename : windows_sys::core::PCWSTR, szobjectname : windows_sys::core::PCWSTR, szdefaultcountername : windows_sys::core::PWSTR, pcchbuffersize : *mut u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectA(szdatasource : windows_sys::core::PCSTR, szmachinename : windows_sys::core::PCSTR, szdefaultobjectname : windows_sys::core::PSTR, pcchbuffersize : *mut u32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectHA(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCSTR, szdefaultobjectname : windows_sys::core::PSTR, pcchbuffersize : *mut u32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectHW(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCWSTR, szdefaultobjectname : windows_sys::core::PWSTR, pcchbuffersize : *mut u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectW(szdatasource : windows_sys::core::PCWSTR, szmachinename : windows_sys::core::PCWSTR, szdefaultobjectname : windows_sys::core::PWSTR, pcchbuffersize : *mut u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhGetDllVersion(lpdwversion : *mut u32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhGetFormattedCounterArrayA(hcounter : PDH_HCOUNTER, dwformat : u32, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_FMT_COUNTERVALUE_ITEM_A) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhGetFormattedCounterArrayW(hcounter : PDH_HCOUNTER, dwformat : u32, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_FMT_COUNTERVALUE_ITEM_W) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhGetFormattedCounterValue(hcounter : PDH_HCOUNTER, dwformat : u32, lpdwtype : *mut u32, pvalue : *mut PDH_FMT_COUNTERVALUE) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhGetLogFileSize(hlog : PDH_HLOG, llsize : *mut i64) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhGetLogSetGUID(hlog : PDH_HLOG, pguid : *mut windows_sys::core::GUID, prunid : *mut i32) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhGetRawCounterArrayA(hcounter : PDH_HCOUNTER, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_RAW_COUNTER_ITEM_A) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhGetRawCounterArrayW(hcounter : PDH_HCOUNTER, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_RAW_COUNTER_ITEM_W) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhGetRawCounterValue(hcounter : PDH_HCOUNTER, lpdwtype : *mut u32, pvalue : *mut PDH_RAW_COUNTER) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhIsRealTimeQuery(hquery : PDH_HQUERY) -> windows_sys::core::BOOL);
windows_link::link!("pdh.dll" "system" fn PdhLookupPerfIndexByNameA(szmachinename : windows_sys::core::PCSTR, sznamebuffer : windows_sys::core::PCSTR, pdwindex : *mut u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhLookupPerfIndexByNameW(szmachinename : windows_sys::core::PCWSTR, sznamebuffer : windows_sys::core::PCWSTR, pdwindex : *mut u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhLookupPerfNameByIndexA(szmachinename : windows_sys::core::PCSTR, dwnameindex : u32, sznamebuffer : windows_sys::core::PSTR, pcchnamebuffersize : *mut u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhLookupPerfNameByIndexW(szmachinename : windows_sys::core::PCWSTR, dwnameindex : u32, sznamebuffer : windows_sys::core::PWSTR, pcchnamebuffersize : *mut u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhMakeCounterPathA(pcounterpathelements : *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer : windows_sys::core::PSTR, pcchbuffersize : *mut u32, dwflags : u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhMakeCounterPathW(pcounterpathelements : *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer : windows_sys::core::PWSTR, pcchbuffersize : *mut u32, dwflags : u32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhOpenLogA(szlogfilename : windows_sys::core::PCSTR, dwaccessflags : u32, lpdwlogtype : *mut u32, hquery : PDH_HQUERY, dwmaxsize : u32, szusercaption : windows_sys::core::PCSTR, phlog : *mut PDH_HLOG) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhOpenLogW(szlogfilename : windows_sys::core::PCWSTR, dwaccessflags : u32, lpdwlogtype : *mut u32, hquery : PDH_HQUERY, dwmaxsize : u32, szusercaption : windows_sys::core::PCWSTR, phlog : *mut PDH_HLOG) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhOpenQueryA(szdatasource : windows_sys::core::PCSTR, dwuserdata : usize, phquery : *mut PDH_HQUERY) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhOpenQueryH(hdatasource : PDH_HLOG, dwuserdata : usize, phquery : *mut PDH_HQUERY) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhOpenQueryW(szdatasource : windows_sys::core::PCWSTR, dwuserdata : usize, phquery : *mut PDH_HQUERY) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhParseCounterPathA(szfullpathbuffer : windows_sys::core::PCSTR, pcounterpathelements : *mut PDH_COUNTER_PATH_ELEMENTS_A, pdwbuffersize : *mut u32, dwflags : u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhParseCounterPathW(szfullpathbuffer : windows_sys::core::PCWSTR, pcounterpathelements : *mut PDH_COUNTER_PATH_ELEMENTS_W, pdwbuffersize : *mut u32, dwflags : u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhParseInstanceNameA(szinstancestring : windows_sys::core::PCSTR, szinstancename : windows_sys::core::PSTR, pcchinstancenamelength : *mut u32, szparentname : windows_sys::core::PSTR, pcchparentnamelength : *mut u32, lpindex : *mut u32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhParseInstanceNameW(szinstancestring : windows_sys::core::PCWSTR, szinstancename : windows_sys::core::PWSTR, pcchinstancenamelength : *mut u32, szparentname : windows_sys::core::PWSTR, pcchparentnamelength : *mut u32, lpindex : *mut u32) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhReadRawLogRecord(hlog : PDH_HLOG, ftrecord : super::minwindef::FILETIME, prawlogrecord : *mut PDH_RAW_LOG_RECORD, pdwbufferlength : *mut u32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhRemoveCounter(hcounter : PDH_HCOUNTER) -> PDH_STATUS);
#[cfg(feature = "windef")]
windows_link::link!("pdh.dll" "system" fn PdhSelectDataSourceA(hwndowner : super::windef::HWND, dwflags : u32, szdatasource : windows_sys::core::PSTR, pcchbufferlength : *mut u32) -> PDH_STATUS);
#[cfg(feature = "windef")]
windows_link::link!("pdh.dll" "system" fn PdhSelectDataSourceW(hwndowner : super::windef::HWND, dwflags : u32, szdatasource : windows_sys::core::PWSTR, pcchbufferlength : *mut u32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhSetCounterScaleFactor(hcounter : PDH_HCOUNTER, lfactor : i32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhSetDefaultRealTimeDataSource(dwdatasourceid : u32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhSetLogSetRunID(hlog : PDH_HLOG, runid : i32) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhSetQueryTimeRange(hquery : PDH_HQUERY, pinfo : *const PDH_TIME_INFO) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhUpdateLogA(hlog : PDH_HLOG, szuserstring : windows_sys::core::PCSTR) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhUpdateLogFileCatalog(hlog : PDH_HLOG) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhUpdateLogW(hlog : PDH_HLOG, szuserstring : windows_sys::core::PCWSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhValidatePathA(szfullpathbuffer : windows_sys::core::PCSTR) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhValidatePathExA(hdatasource : PDH_HLOG, szfullpathbuffer : windows_sys::core::PCSTR) -> PDH_STATUS);
#[cfg(feature = "winnt")]
windows_link::link!("pdh.dll" "system" fn PdhValidatePathExW(hdatasource : PDH_HLOG, szfullpathbuffer : windows_sys::core::PCWSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhValidatePathW(szfullpathbuffer : windows_sys::core::PCWSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhVerifySQLDBA(szdatasource : windows_sys::core::PCSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhVerifySQLDBW(szdatasource : windows_sys::core::PCWSTR) -> PDH_STATUS);
pub type CounterPathCallBack = Option<unsafe extern "system" fn(param0: usize) -> PDH_STATUS>;
pub const DATA_SOURCE_LOGFILE: u32 = 2;
pub const DATA_SOURCE_REGISTRY: u32 = 1;
pub const DATA_SOURCE_WBEM: u32 = 4;
#[cfg(feature = "winnt")]
pub type HCOUNTER = PDH_HCOUNTER;
#[cfg(feature = "winnt")]
pub type HLOG = PDH_HLOG;
#[cfg(feature = "winnt")]
pub type HQUERY = PDH_HQUERY;
pub const H_REALTIME_DATASOURCE: u32 = 0;
pub const MAX_COUNTER_PATH: u32 = 256;
pub const MAX_TIME_VALUE: i64 = 9223372036854775807;
pub const MIN_TIME_VALUE: i64 = 0;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct PDH_BROWSE_DLG_CONFIG_A {
    pub _bitfield: u32,
    pub hWndOwner: super::windef::HWND,
    pub szDataSource: windows_sys::core::PSTR,
    pub szReturnPathBuffer: windows_sys::core::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: PDH_STATUS,
    pub dwDefaultDetailLevel: u32,
    pub szDialogBoxCaption: windows_sys::core::PSTR,
}
#[cfg(feature = "windef")]
impl Default for PDH_BROWSE_DLG_CONFIG_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PDH_BROWSE_DLG_CONFIG_HA {
    pub _bitfield: u32,
    pub hWndOwner: super::windef::HWND,
    pub hDataSource: PDH_HLOG,
    pub szReturnPathBuffer: windows_sys::core::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: PDH_STATUS,
    pub dwDefaultDetailLevel: u32,
    pub szDialogBoxCaption: windows_sys::core::PSTR,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for PDH_BROWSE_DLG_CONFIG_HA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PDH_BROWSE_DLG_CONFIG_HW {
    pub _bitfield: u32,
    pub hWndOwner: super::windef::HWND,
    pub hDataSource: PDH_HLOG,
    pub szReturnPathBuffer: windows_sys::core::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: PDH_STATUS,
    pub dwDefaultDetailLevel: u32,
    pub szDialogBoxCaption: windows_sys::core::PWSTR,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for PDH_BROWSE_DLG_CONFIG_HW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct PDH_BROWSE_DLG_CONFIG_W {
    pub _bitfield: u32,
    pub hWndOwner: super::windef::HWND,
    pub szDataSource: windows_sys::core::PWSTR,
    pub szReturnPathBuffer: windows_sys::core::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: PDH_STATUS,
    pub dwDefaultDetailLevel: u32,
    pub szDialogBoxCaption: windows_sys::core::PWSTR,
}
#[cfg(feature = "windef")]
impl Default for PDH_BROWSE_DLG_CONFIG_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_COUNTER_INFO_A {
    pub dwLength: u32,
    pub dwType: u32,
    pub CVersion: u32,
    pub CStatus: u32,
    pub lScale: i32,
    pub lDefaultScale: i32,
    pub dwUserData: usize,
    pub dwQueryUserData: usize,
    pub szFullPath: windows_sys::core::PSTR,
    pub Anonymous: PDH_COUNTER_INFO_A_0,
    pub szExplainText: windows_sys::core::PSTR,
    pub DataBuffer: [u32; 1],
}
impl Default for PDH_COUNTER_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PDH_COUNTER_INFO_A_0 {
    pub DataItemPath: PDH_DATA_ITEM_PATH_ELEMENTS_A,
    pub CounterPath: PDH_COUNTER_PATH_ELEMENTS_A,
    pub Anonymous: PDH_COUNTER_INFO_A_0_0,
}
impl Default for PDH_COUNTER_INFO_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_COUNTER_INFO_A_0_0 {
    pub szMachineName: windows_sys::core::PSTR,
    pub szObjectName: windows_sys::core::PSTR,
    pub szInstanceName: windows_sys::core::PSTR,
    pub szParentInstance: windows_sys::core::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_sys::core::PSTR,
}
impl Default for PDH_COUNTER_INFO_A_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_COUNTER_INFO_W {
    pub dwLength: u32,
    pub dwType: u32,
    pub CVersion: u32,
    pub CStatus: u32,
    pub lScale: i32,
    pub lDefaultScale: i32,
    pub dwUserData: usize,
    pub dwQueryUserData: usize,
    pub szFullPath: windows_sys::core::PWSTR,
    pub Anonymous: PDH_COUNTER_INFO_W_0,
    pub szExplainText: windows_sys::core::PWSTR,
    pub DataBuffer: [u32; 1],
}
impl Default for PDH_COUNTER_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PDH_COUNTER_INFO_W_0 {
    pub DataItemPath: PDH_DATA_ITEM_PATH_ELEMENTS_W,
    pub CounterPath: PDH_COUNTER_PATH_ELEMENTS_W,
    pub Anonymous: PDH_COUNTER_INFO_W_0_0,
}
impl Default for PDH_COUNTER_INFO_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_COUNTER_INFO_W_0_0 {
    pub szMachineName: windows_sys::core::PWSTR,
    pub szObjectName: windows_sys::core::PWSTR,
    pub szInstanceName: windows_sys::core::PWSTR,
    pub szParentInstance: windows_sys::core::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_sys::core::PWSTR,
}
impl Default for PDH_COUNTER_INFO_W_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_COUNTER_PATH_ELEMENTS_A {
    pub szMachineName: windows_sys::core::PSTR,
    pub szObjectName: windows_sys::core::PSTR,
    pub szInstanceName: windows_sys::core::PSTR,
    pub szParentInstance: windows_sys::core::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_sys::core::PSTR,
}
impl Default for PDH_COUNTER_PATH_ELEMENTS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_COUNTER_PATH_ELEMENTS_W {
    pub szMachineName: windows_sys::core::PWSTR,
    pub szObjectName: windows_sys::core::PWSTR,
    pub szInstanceName: windows_sys::core::PWSTR,
    pub szParentInstance: windows_sys::core::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_sys::core::PWSTR,
}
impl Default for PDH_COUNTER_PATH_ELEMENTS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PDH_CVERSION_WIN40: u32 = 1024;
pub const PDH_CVERSION_WIN50: u32 = 1280;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_A {
    pub szMachineName: windows_sys::core::PSTR,
    pub ObjectGUID: windows_sys::core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: windows_sys::core::PSTR,
}
impl Default for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_W {
    pub szMachineName: windows_sys::core::PWSTR,
    pub ObjectGUID: windows_sys::core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: windows_sys::core::PWSTR,
}
impl Default for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PDH_FLAGS_CLOSE_QUERY: u32 = 1;
pub const PDH_FLAGS_FILE_BROWSER_ONLY: u32 = 1;
pub const PDH_FMT_1000: u32 = 8192;
pub const PDH_FMT_ANSI: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_FMT_COUNTERVALUE {
    pub CStatus: u32,
    pub Anonymous: PDH_FMT_COUNTERVALUE_0,
}
impl Default for PDH_FMT_COUNTERVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PDH_FMT_COUNTERVALUE_0 {
    pub longValue: i32,
    pub doubleValue: f64,
    pub largeValue: i64,
    pub AnsiStringValue: windows_sys::core::PCSTR,
    pub WideStringValue: windows_sys::core::PCWSTR,
}
impl Default for PDH_FMT_COUNTERVALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_FMT_COUNTERVALUE_ITEM_A {
    pub szName: windows_sys::core::PSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
impl Default for PDH_FMT_COUNTERVALUE_ITEM_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_FMT_COUNTERVALUE_ITEM_W {
    pub szName: windows_sys::core::PWSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
impl Default for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PDH_FMT_DOUBLE: u32 = 512;
pub const PDH_FMT_LARGE: u32 = 1024;
pub const PDH_FMT_LONG: u32 = 256;
pub const PDH_FMT_NOCAP100: u32 = 32768;
pub const PDH_FMT_NODATA: u32 = 16384;
pub const PDH_FMT_NOSCALE: u32 = 4096;
pub const PDH_FMT_RAW: u32 = 16;
pub const PDH_FMT_UNICODE: u32 = 64;
#[cfg(feature = "winnt")]
pub type PDH_HCOUNTER = super::winnt::HANDLE;
#[cfg(feature = "winnt")]
pub type PDH_HLOG = super::winnt::HANDLE;
#[cfg(feature = "winnt")]
pub type PDH_HQUERY = super::winnt::HANDLE;
pub const PDH_LOG_ACCESS_MASK: u32 = 983040;
pub const PDH_LOG_CREATE_ALWAYS: u32 = 2;
pub const PDH_LOG_CREATE_MASK: u32 = 15;
pub const PDH_LOG_CREATE_NEW: u32 = 1;
pub const PDH_LOG_OPEN_ALWAYS: u32 = 3;
pub const PDH_LOG_OPEN_EXISTING: u32 = 4;
pub const PDH_LOG_OPT_APPEND: u32 = 134217728;
pub const PDH_LOG_OPT_CIRCULAR: u32 = 33554432;
pub const PDH_LOG_OPT_MASK: u32 = 251658240;
pub const PDH_LOG_OPT_MAX_IS_BYTES: u32 = 67108864;
pub const PDH_LOG_OPT_USER_STRING: u32 = 16777216;
pub const PDH_LOG_READ_ACCESS: u32 = 65536;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: windows_sys::core::PSTR,
    pub szDefaultDir: windows_sys::core::PSTR,
    pub szBaseFileName: windows_sys::core::PSTR,
    pub dwFileType: u32,
    pub dwReserved: u32,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_A_0,
}
#[cfg(feature = "minwindef")]
impl Default for PDH_LOG_SERVICE_QUERY_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_A_0_0,
    pub Anonymous2: PDH_LOG_SERVICE_QUERY_INFO_A_0_1,
}
#[cfg(feature = "minwindef")]
impl Default for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: windows_sys::core::PSTR,
    pub PdlCounterList: windows_sys::core::PSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::minwindef::FILETIME,
    pub PdlLogEndTime: super::minwindef::FILETIME,
}
#[cfg(feature = "minwindef")]
impl Default for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    pub TlNumberOfBuffers: u32,
    pub TlMinimumBuffers: u32,
    pub TlMaximumBuffers: u32,
    pub TlFreeBuffers: u32,
    pub TlBufferSize: u32,
    pub TlEventsLost: u32,
    pub TlLoggerThreadId: u32,
    pub TlBuffersWritten: u32,
    pub TlLogHandle: u32,
    pub TlLogFileName: windows_sys::core::PSTR,
}
#[cfg(feature = "minwindef")]
impl Default for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: windows_sys::core::PWSTR,
    pub szDefaultDir: windows_sys::core::PWSTR,
    pub szBaseFileName: windows_sys::core::PWSTR,
    pub dwFileType: u32,
    pub dwReserved: u32,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_W_0,
}
#[cfg(feature = "minwindef")]
impl Default for PDH_LOG_SERVICE_QUERY_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_W_0_0,
    pub Anonymous2: PDH_LOG_SERVICE_QUERY_INFO_W_0_1,
}
#[cfg(feature = "minwindef")]
impl Default for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: windows_sys::core::PWSTR,
    pub PdlCounterList: windows_sys::core::PWSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::minwindef::FILETIME,
    pub PdlLogEndTime: super::minwindef::FILETIME,
}
#[cfg(feature = "minwindef")]
impl Default for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    pub TlNumberOfBuffers: u32,
    pub TlMinimumBuffers: u32,
    pub TlMaximumBuffers: u32,
    pub TlFreeBuffers: u32,
    pub TlBufferSize: u32,
    pub TlEventsLost: u32,
    pub TlLoggerThreadId: u32,
    pub TlBuffersWritten: u32,
    pub TlLogHandle: u32,
    pub TlLogFileName: windows_sys::core::PWSTR,
}
#[cfg(feature = "minwindef")]
impl Default for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PDH_LOG_TYPE_BINARY: u32 = 8;
pub const PDH_LOG_TYPE_CSV: u32 = 1;
pub const PDH_LOG_TYPE_PERFMON: u32 = 6;
pub const PDH_LOG_TYPE_RETIRED_BIN: u32 = 3;
pub const PDH_LOG_TYPE_SQL: u32 = 7;
pub const PDH_LOG_TYPE_TRACE_GENERIC: u32 = 5;
pub const PDH_LOG_TYPE_TRACE_KERNEL: u32 = 4;
pub const PDH_LOG_TYPE_TSV: u32 = 2;
pub const PDH_LOG_TYPE_UNDEFINED: u32 = 0;
pub const PDH_LOG_UPDATE_ACCESS: u32 = 262144;
pub const PDH_LOG_WRITE_ACCESS: u32 = 131072;
pub const PDH_MAX_COUNTER_NAME: u32 = 1024;
pub const PDH_MAX_COUNTER_PATH: u32 = 2048;
pub const PDH_MAX_DATASOURCE_PATH: u32 = 1024;
pub const PDH_MAX_INSTANCE_NAME: u32 = 1024;
pub const PDH_MAX_SCALE: u32 = 7;
pub const PDH_MIN_SCALE: i32 = -7;
pub const PDH_NOEXPANDCOUNTERS: u32 = 1;
pub const PDH_NOEXPANDINSTANCES: u32 = 2;
pub const PDH_OBJECT_HAS_INSTANCES: u32 = 1;
pub const PDH_PATH_WBEM_INPUT: u32 = 2;
pub const PDH_PATH_WBEM_RESULT: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct PDH_RAW_COUNTER {
    pub CStatus: u32,
    pub TimeStamp: super::minwindef::FILETIME,
    pub FirstValue: i64,
    pub SecondValue: i64,
    pub MultiCount: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct PDH_RAW_COUNTER_ITEM_A {
    pub szName: windows_sys::core::PSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
#[cfg(feature = "minwindef")]
impl Default for PDH_RAW_COUNTER_ITEM_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct PDH_RAW_COUNTER_ITEM_W {
    pub szName: windows_sys::core::PWSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
#[cfg(feature = "minwindef")]
impl Default for PDH_RAW_COUNTER_ITEM_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_RAW_LOG_RECORD {
    pub dwStructureSize: u32,
    pub dwRecordType: u32,
    pub dwItems: u32,
    pub RawBytes: [u8; 1],
}
impl Default for PDH_RAW_LOG_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PDH_REFRESHCOUNTERS: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_STATISTICS {
    pub dwFormat: u32,
    pub count: u32,
    pub min: PDH_FMT_COUNTERVALUE,
    pub max: PDH_FMT_COUNTERVALUE,
    pub mean: PDH_FMT_COUNTERVALUE,
}
impl Default for PDH_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PDH_STATUS = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PDH_TIME_INFO {
    pub StartTime: i64,
    pub EndTime: i64,
    pub SampleCount: u32,
}
pub const PDH_VERSION: u32 = 1283;
pub const PERF_DETAIL_COSTLY: u32 = 65536;
pub const PERF_DETAIL_STANDARD: u32 = 65535;
#[cfg(feature = "windef")]
pub type PPDH_BROWSE_DLG_CONFIG_A = *mut PDH_BROWSE_DLG_CONFIG_A;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PPDH_BROWSE_DLG_CONFIG_HA = *mut PDH_BROWSE_DLG_CONFIG_HA;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PPDH_BROWSE_DLG_CONFIG_HW = *mut PDH_BROWSE_DLG_CONFIG_HW;
#[cfg(feature = "windef")]
pub type PPDH_BROWSE_DLG_CONFIG_W = *mut PDH_BROWSE_DLG_CONFIG_W;
pub type PPDH_COUNTER_INFO_A = *mut PDH_COUNTER_INFO_A;
pub type PPDH_COUNTER_INFO_W = *mut PDH_COUNTER_INFO_W;
pub type PPDH_COUNTER_PATH_ELEMENTS_A = *mut PDH_COUNTER_PATH_ELEMENTS_A;
pub type PPDH_COUNTER_PATH_ELEMENTS_W = *mut PDH_COUNTER_PATH_ELEMENTS_W;
pub type PPDH_DATA_ITEM_PATH_ELEMENTS_A = *mut PDH_DATA_ITEM_PATH_ELEMENTS_A;
pub type PPDH_DATA_ITEM_PATH_ELEMENTS_W = *mut PDH_DATA_ITEM_PATH_ELEMENTS_W;
pub type PPDH_FMT_COUNTERVALUE = *mut PDH_FMT_COUNTERVALUE;
pub type PPDH_FMT_COUNTERVALUE_ITEM_A = *mut PDH_FMT_COUNTERVALUE_ITEM_A;
pub type PPDH_FMT_COUNTERVALUE_ITEM_W = *mut PDH_FMT_COUNTERVALUE_ITEM_W;
#[cfg(feature = "minwindef")]
pub type PPDH_LOG_SERVICE_QUERY_INFO_A = *mut PDH_LOG_SERVICE_QUERY_INFO_A;
#[cfg(feature = "minwindef")]
pub type PPDH_LOG_SERVICE_QUERY_INFO_W = *mut PDH_LOG_SERVICE_QUERY_INFO_W;
#[cfg(feature = "minwindef")]
pub type PPDH_RAW_COUNTER = *mut PDH_RAW_COUNTER;
#[cfg(feature = "minwindef")]
pub type PPDH_RAW_COUNTER_ITEM_A = *mut PDH_RAW_COUNTER_ITEM_A;
#[cfg(feature = "minwindef")]
pub type PPDH_RAW_COUNTER_ITEM_W = *mut PDH_RAW_COUNTER_ITEM_W;
pub type PPDH_RAW_LOG_RECORD = *mut PDH_RAW_LOG_RECORD;
pub type PPDH_STATISTICS = *mut PDH_STATISTICS;
pub type PPDH_TIME_INFO = *mut PDH_TIME_INFO;
