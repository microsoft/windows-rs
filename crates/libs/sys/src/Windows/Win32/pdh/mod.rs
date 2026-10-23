#[cfg(feature = "shared")]
windows_link::link!("pdh.dll" "system" fn PdhAddCounterA(hquery : PDH_HQUERY, szfullcounterpath : windows_sys::core::PCSTR, dwuserdata : super::DWORD_PTR, phcounter : *mut PDH_HCOUNTER) -> PDH_STATUS);
#[cfg(feature = "shared")]
windows_link::link!("pdh.dll" "system" fn PdhAddCounterW(hquery : PDH_HQUERY, szfullcounterpath : windows_sys::core::PCWSTR, dwuserdata : super::DWORD_PTR, phcounter : *mut PDH_HCOUNTER) -> PDH_STATUS);
#[cfg(feature = "shared")]
windows_link::link!("pdh.dll" "system" fn PdhAddEnglishCounterA(hquery : PDH_HQUERY, szfullcounterpath : windows_sys::core::PCSTR, dwuserdata : super::DWORD_PTR, phcounter : *mut PDH_HCOUNTER) -> PDH_STATUS);
#[cfg(feature = "shared")]
windows_link::link!("pdh.dll" "system" fn PdhAddEnglishCounterW(hquery : PDH_HQUERY, szfullcounterpath : windows_sys::core::PCWSTR, dwuserdata : super::DWORD_PTR, phcounter : *mut PDH_HCOUNTER) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhBindInputDataSourceA(phdatasource : *mut PDH_HLOG, logfilenamelist : windows_sys::core::PCSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhBindInputDataSourceW(phdatasource : *mut PDH_HLOG, logfilenamelist : windows_sys::core::PCWSTR) -> PDH_STATUS);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("pdh.dll" "system" fn PdhBrowseCountersA(pbrowsedlgdata : PPDH_BROWSE_DLG_CONFIG_A) -> PDH_STATUS);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("pdh.dll" "system" fn PdhBrowseCountersHA(pbrowsedlgdata : PPDH_BROWSE_DLG_CONFIG_HA) -> PDH_STATUS);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("pdh.dll" "system" fn PdhBrowseCountersHW(pbrowsedlgdata : PPDH_BROWSE_DLG_CONFIG_HW) -> PDH_STATUS);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("pdh.dll" "system" fn PdhBrowseCountersW(pbrowsedlgdata : PPDH_BROWSE_DLG_CONFIG_W) -> PDH_STATUS);
#[cfg(feature = "shared")]
windows_link::link!("pdh.dll" "system" fn PdhCloseLog(hlog : PDH_HLOG, dwflags : super::DWORD) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhCloseQuery(hquery : PDH_HQUERY) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhCollectQueryData(hquery : PDH_HQUERY) -> PDH_STATUS);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhCollectQueryDataEx(hquery : PDH_HQUERY, dwintervaltime : super::DWORD, hnewdataevent : super::HANDLE) -> PDH_STATUS);
#[cfg(feature = "shared")]
windows_link::link!("pdh.dll" "system" fn PdhCollectQueryDataWithTime(hquery : PDH_HQUERY, plltimestamp : *mut super::LONGLONG) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhConnectMachineA(szmachinename : windows_sys::core::PCSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhConnectMachineW(szmachinename : windows_sys::core::PCWSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhCreateSQLTablesA(szdatasource : windows_sys::core::PCSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhCreateSQLTablesW(szdatasource : windows_sys::core::PCWSTR) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhEnumLogSetNamesA(szdatasource : windows_sys::core::PCSTR, mszdatasetnamelist : super::PZZSTR, pcchbufferlength : super::LPDWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhEnumLogSetNamesW(szdatasource : windows_sys::core::PCWSTR, mszdatasetnamelist : super::PZZWSTR, pcchbufferlength : super::LPDWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhEnumMachinesA(szdatasource : windows_sys::core::PCSTR, mszmachinelist : super::PZZSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhEnumMachinesHA(hdatasource : PDH_HLOG, mszmachinelist : super::PZZSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhEnumMachinesHW(hdatasource : PDH_HLOG, mszmachinelist : super::PZZWSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhEnumMachinesW(szdatasource : windows_sys::core::PCWSTR, mszmachinelist : super::PZZWSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhEnumObjectItemsA(szdatasource : windows_sys::core::PCSTR, szmachinename : windows_sys::core::PCSTR, szobjectname : windows_sys::core::PCSTR, mszcounterlist : super::PZZSTR, pcchcounterlistlength : super::LPDWORD, mszinstancelist : super::PZZSTR, pcchinstancelistlength : super::LPDWORD, dwdetaillevel : super::DWORD, dwflags : super::DWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhEnumObjectItemsHA(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCSTR, szobjectname : windows_sys::core::PCSTR, mszcounterlist : super::PZZSTR, pcchcounterlistlength : super::LPDWORD, mszinstancelist : super::PZZSTR, pcchinstancelistlength : super::LPDWORD, dwdetaillevel : super::DWORD, dwflags : super::DWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhEnumObjectItemsHW(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCWSTR, szobjectname : windows_sys::core::PCWSTR, mszcounterlist : super::PZZWSTR, pcchcounterlistlength : super::LPDWORD, mszinstancelist : super::PZZWSTR, pcchinstancelistlength : super::LPDWORD, dwdetaillevel : super::DWORD, dwflags : super::DWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhEnumObjectItemsW(szdatasource : windows_sys::core::PCWSTR, szmachinename : windows_sys::core::PCWSTR, szobjectname : windows_sys::core::PCWSTR, mszcounterlist : super::PZZWSTR, pcchcounterlistlength : super::LPDWORD, mszinstancelist : super::PZZWSTR, pcchinstancelistlength : super::LPDWORD, dwdetaillevel : super::DWORD, dwflags : super::DWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhEnumObjectsA(szdatasource : windows_sys::core::PCSTR, szmachinename : windows_sys::core::PCSTR, mszobjectlist : super::PZZSTR, pcchbuffersize : super::LPDWORD, dwdetaillevel : super::DWORD, brefresh : windows_sys::core::BOOL) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhEnumObjectsHA(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCSTR, mszobjectlist : super::PZZSTR, pcchbuffersize : super::LPDWORD, dwdetaillevel : super::DWORD, brefresh : windows_sys::core::BOOL) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhEnumObjectsHW(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCWSTR, mszobjectlist : super::PZZWSTR, pcchbuffersize : super::LPDWORD, dwdetaillevel : super::DWORD, brefresh : windows_sys::core::BOOL) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhEnumObjectsW(szdatasource : windows_sys::core::PCWSTR, szmachinename : windows_sys::core::PCWSTR, mszobjectlist : super::PZZWSTR, pcchbuffersize : super::LPDWORD, dwdetaillevel : super::DWORD, brefresh : windows_sys::core::BOOL) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhExpandCounterPathA(szwildcardpath : windows_sys::core::PCSTR, mszexpandedpathlist : super::PZZSTR, pcchpathlistlength : super::LPDWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhExpandCounterPathW(szwildcardpath : windows_sys::core::PCWSTR, mszexpandedpathlist : super::PZZWSTR, pcchpathlistlength : super::LPDWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhExpandWildCardPathA(szdatasource : windows_sys::core::PCSTR, szwildcardpath : windows_sys::core::PCSTR, mszexpandedpathlist : super::PZZSTR, pcchpathlistlength : super::LPDWORD, dwflags : super::DWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhExpandWildCardPathHA(hdatasource : PDH_HLOG, szwildcardpath : windows_sys::core::PCSTR, mszexpandedpathlist : super::PZZSTR, pcchpathlistlength : super::LPDWORD, dwflags : super::DWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhExpandWildCardPathHW(hdatasource : PDH_HLOG, szwildcardpath : windows_sys::core::PCWSTR, mszexpandedpathlist : super::PZZWSTR, pcchpathlistlength : super::LPDWORD, dwflags : super::DWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("pdh.dll" "system" fn PdhExpandWildCardPathW(szdatasource : windows_sys::core::PCWSTR, szwildcardpath : windows_sys::core::PCWSTR, mszexpandedpathlist : super::PZZWSTR, pcchpathlistlength : super::LPDWORD, dwflags : super::DWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("pdh.dll" "system" fn PdhGetCounterInfoA(hcounter : PDH_HCOUNTER, bretrieveexplaintext : super::BOOLEAN, pdwbuffersize : super::LPDWORD, lpbuffer : PPDH_COUNTER_INFO_A) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("pdh.dll" "system" fn PdhGetCounterInfoW(hcounter : PDH_HCOUNTER, bretrieveexplaintext : super::BOOLEAN, pdwbuffersize : super::LPDWORD, lpbuffer : PPDH_COUNTER_INFO_W) -> PDH_STATUS);
#[cfg(feature = "shared")]
windows_link::link!("pdh.dll" "system" fn PdhGetCounterTimeBase(hcounter : PDH_HCOUNTER, ptimebase : *mut super::LONGLONG) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeA(szdatasource : windows_sys::core::PCSTR, pdwnumentries : super::LPDWORD, pinfo : PPDH_TIME_INFO, pdwbuffersize : super::LPDWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeH(hdatasource : PDH_HLOG, pdwnumentries : super::LPDWORD, pinfo : PPDH_TIME_INFO, pdwbuffersize : super::LPDWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeW(szdatasource : windows_sys::core::PCWSTR, pdwnumentries : super::LPDWORD, pinfo : PPDH_TIME_INFO, pdwbuffersize : super::LPDWORD) -> PDH_STATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterA(szdatasource : windows_sys::core::PCSTR, szmachinename : windows_sys::core::PCSTR, szobjectname : windows_sys::core::PCSTR, szdefaultcountername : windows_sys::core::PSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterHA(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCSTR, szobjectname : windows_sys::core::PCSTR, szdefaultcountername : windows_sys::core::PSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterHW(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCWSTR, szobjectname : windows_sys::core::PCWSTR, szdefaultcountername : windows_sys::core::PWSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterW(szdatasource : windows_sys::core::PCWSTR, szmachinename : windows_sys::core::PCWSTR, szobjectname : windows_sys::core::PCWSTR, szdefaultcountername : windows_sys::core::PWSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectA(szdatasource : windows_sys::core::PCSTR, szmachinename : windows_sys::core::PCSTR, szdefaultobjectname : windows_sys::core::PSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectHA(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCSTR, szdefaultobjectname : windows_sys::core::PSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectHW(hdatasource : PDH_HLOG, szmachinename : windows_sys::core::PCWSTR, szdefaultobjectname : windows_sys::core::PWSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectW(szdatasource : windows_sys::core::PCWSTR, szmachinename : windows_sys::core::PCWSTR, szdefaultobjectname : windows_sys::core::PWSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("pdh.dll" "system" fn PdhGetDllVersion(lpdwversion : super::LPDWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("pdh.dll" "system" fn PdhGetFormattedCounterArrayA(hcounter : PDH_HCOUNTER, dwformat : super::DWORD, lpdwbuffersize : super::LPDWORD, lpdwitemcount : super::LPDWORD, itembuffer : PPDH_FMT_COUNTERVALUE_ITEM_A) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("pdh.dll" "system" fn PdhGetFormattedCounterArrayW(hcounter : PDH_HCOUNTER, dwformat : super::DWORD, lpdwbuffersize : super::LPDWORD, lpdwitemcount : super::LPDWORD, itembuffer : PPDH_FMT_COUNTERVALUE_ITEM_W) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("pdh.dll" "system" fn PdhGetFormattedCounterValue(hcounter : PDH_HCOUNTER, dwformat : super::DWORD, lpdwtype : super::LPDWORD, pvalue : PPDH_FMT_COUNTERVALUE) -> PDH_STATUS);
#[cfg(feature = "shared")]
windows_link::link!("pdh.dll" "system" fn PdhGetLogFileSize(hlog : PDH_HLOG, llsize : *mut super::LONGLONG) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhGetLogSetGUID(hlog : PDH_HLOG, pguid : *mut windows_sys::core::GUID, prunid : *mut i32) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhIsRealTimeQuery(hquery : PDH_HQUERY) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("pdh.dll" "system" fn PdhLookupPerfIndexByNameA(szmachinename : windows_sys::core::PCSTR, sznamebuffer : windows_sys::core::PCSTR, pdwindex : super::LPDWORD) -> PDH_STATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("pdh.dll" "system" fn PdhLookupPerfIndexByNameW(szmachinename : windows_sys::core::PCWSTR, sznamebuffer : windows_sys::core::PCWSTR, pdwindex : super::LPDWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("pdh.dll" "system" fn PdhLookupPerfNameByIndexA(szmachinename : windows_sys::core::PCSTR, dwnameindex : super::DWORD, sznamebuffer : windows_sys::core::PSTR, pcchnamebuffersize : super::LPDWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("pdh.dll" "system" fn PdhLookupPerfNameByIndexW(szmachinename : windows_sys::core::PCWSTR, dwnameindex : super::DWORD, sznamebuffer : windows_sys::core::PWSTR, pcchnamebuffersize : super::LPDWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("pdh.dll" "system" fn PdhMakeCounterPathA(pcounterpathelements : PPDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer : windows_sys::core::PSTR, pcchbuffersize : super::LPDWORD, dwflags : super::DWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("pdh.dll" "system" fn PdhMakeCounterPathW(pcounterpathelements : PPDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer : windows_sys::core::PWSTR, pcchbuffersize : super::LPDWORD, dwflags : super::DWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("pdh.dll" "system" fn PdhOpenLogA(szlogfilename : windows_sys::core::PCSTR, dwaccessflags : super::DWORD, lpdwlogtype : super::LPDWORD, hquery : PDH_HQUERY, dwmaxsize : super::DWORD, szusercaption : windows_sys::core::PCSTR, phlog : *mut PDH_HLOG) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("pdh.dll" "system" fn PdhOpenLogW(szlogfilename : windows_sys::core::PCWSTR, dwaccessflags : super::DWORD, lpdwlogtype : super::LPDWORD, hquery : PDH_HQUERY, dwmaxsize : super::DWORD, szusercaption : windows_sys::core::PCWSTR, phlog : *mut PDH_HLOG) -> PDH_STATUS);
#[cfg(feature = "shared")]
windows_link::link!("pdh.dll" "system" fn PdhOpenQueryA(szdatasource : windows_sys::core::PCSTR, dwuserdata : super::DWORD_PTR, phquery : *mut PDH_HQUERY) -> PDH_STATUS);
#[cfg(feature = "shared")]
windows_link::link!("pdh.dll" "system" fn PdhOpenQueryH(hdatasource : PDH_HLOG, dwuserdata : super::DWORD_PTR, phquery : *mut PDH_HQUERY) -> PDH_STATUS);
#[cfg(feature = "shared")]
windows_link::link!("pdh.dll" "system" fn PdhOpenQueryW(szdatasource : windows_sys::core::PCWSTR, dwuserdata : super::DWORD_PTR, phquery : *mut PDH_HQUERY) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("pdh.dll" "system" fn PdhParseCounterPathA(szfullpathbuffer : windows_sys::core::PCSTR, pcounterpathelements : PPDH_COUNTER_PATH_ELEMENTS_A, pdwbuffersize : super::LPDWORD, dwflags : super::DWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("pdh.dll" "system" fn PdhParseCounterPathW(szfullpathbuffer : windows_sys::core::PCWSTR, pcounterpathelements : PPDH_COUNTER_PATH_ELEMENTS_W, pdwbuffersize : super::LPDWORD, dwflags : super::DWORD) -> PDH_STATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("pdh.dll" "system" fn PdhParseInstanceNameA(szinstancestring : windows_sys::core::PCSTR, szinstancename : windows_sys::core::PSTR, pcchinstancenamelength : super::LPDWORD, szparentname : windows_sys::core::PSTR, pcchparentnamelength : super::LPDWORD, lpindex : super::LPDWORD) -> PDH_STATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("pdh.dll" "system" fn PdhParseInstanceNameW(szinstancestring : windows_sys::core::PCWSTR, szinstancename : windows_sys::core::PWSTR, pcchinstancenamelength : super::LPDWORD, szparentname : windows_sys::core::PWSTR, pcchparentnamelength : super::LPDWORD, lpindex : super::LPDWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("pdh.dll" "system" fn PdhReadRawLogRecord(hlog : PDH_HLOG, ftrecord : super::FILETIME, prawlogrecord : PPDH_RAW_LOG_RECORD, pdwbufferlength : super::LPDWORD) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhRemoveCounter(hcounter : PDH_HCOUNTER) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("pdh.dll" "system" fn PdhSelectDataSourceA(hwndowner : super::HWND, dwflags : super::DWORD, szdatasource : windows_sys::core::PSTR, pcchbufferlength : super::LPDWORD) -> PDH_STATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("pdh.dll" "system" fn PdhSelectDataSourceW(hwndowner : super::HWND, dwflags : super::DWORD, szdatasource : windows_sys::core::PWSTR, pcchbufferlength : super::LPDWORD) -> PDH_STATUS);
#[cfg(feature = "shared")]
windows_link::link!("pdh.dll" "system" fn PdhSetCounterScaleFactor(hcounter : PDH_HCOUNTER, lfactor : super::LONG) -> PDH_STATUS);
#[cfg(feature = "shared")]
windows_link::link!("pdh.dll" "system" fn PdhSetDefaultRealTimeDataSource(dwdatasourceid : super::DWORD) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhSetLogSetRunID(hlog : PDH_HLOG, runid : i32) -> PDH_STATUS);
#[cfg(feature = "shared")]
windows_link::link!("pdh.dll" "system" fn PdhSetQueryTimeRange(hquery : PDH_HQUERY, pinfo : PPDH_TIME_INFO) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhUpdateLogA(hlog : PDH_HLOG, szuserstring : windows_sys::core::PCSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhUpdateLogFileCatalog(hlog : PDH_HLOG) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhUpdateLogW(hlog : PDH_HLOG, szuserstring : windows_sys::core::PCWSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhValidatePathA(szfullpathbuffer : windows_sys::core::PCSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhValidatePathExA(hdatasource : PDH_HLOG, szfullpathbuffer : windows_sys::core::PCSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhValidatePathExW(hdatasource : PDH_HLOG, szfullpathbuffer : windows_sys::core::PCWSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhValidatePathW(szfullpathbuffer : windows_sys::core::PCWSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhVerifySQLDBA(szdatasource : windows_sys::core::PCSTR) -> PDH_STATUS);
windows_link::link!("pdh.dll" "system" fn PdhVerifySQLDBW(szdatasource : windows_sys::core::PCWSTR) -> PDH_STATUS);
#[cfg(feature = "shared")]
pub type CounterPathCallBack = Option<unsafe extern "system" fn(param0: super::DWORD_PTR) -> PDH_STATUS>;
#[cfg(feature = "shared")]
pub const DATA_SOURCE_LOGFILE: super::DWORD = 2;
#[cfg(feature = "shared")]
pub const DATA_SOURCE_REGISTRY: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const DATA_SOURCE_WBEM: super::DWORD = 4;
pub type HCOUNTER = *mut core::ffi::c_void;
pub type HLOG = *mut core::ffi::c_void;
pub type HQUERY = *mut core::ffi::c_void;
pub const H_REALTIME_DATASOURCE: i32 = 0;
#[cfg(feature = "winnt")]
pub const H_WBEM_DATASOURCE: super::HANDLE = -1 as _;
pub const MAX_COUNTER_PATH: i32 = 256;
#[cfg(feature = "shared")]
pub const MAX_TIME_VALUE: super::LONGLONG = 9223372036854775807;
#[cfg(feature = "shared")]
pub const MIN_TIME_VALUE: super::LONGLONG = 0;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct PDH_BROWSE_DLG_CONFIG_A {
    pub _bitfield: u32,
    pub hWndOwner: super::HWND,
    pub szDataSource: windows_sys::core::PSTR,
    pub szReturnPathBuffer: windows_sys::core::PSTR,
    pub cchReturnPathLength: super::DWORD,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: super::DWORD_PTR,
    pub CallBackStatus: PDH_STATUS,
    pub dwDefaultDetailLevel: super::DWORD,
    pub szDialogBoxCaption: windows_sys::core::PSTR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct PDH_BROWSE_DLG_CONFIG_HA {
    pub _bitfield: u32,
    pub hWndOwner: super::HWND,
    pub hDataSource: PDH_HLOG,
    pub szReturnPathBuffer: windows_sys::core::PSTR,
    pub cchReturnPathLength: super::DWORD,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: super::DWORD_PTR,
    pub CallBackStatus: PDH_STATUS,
    pub dwDefaultDetailLevel: super::DWORD,
    pub szDialogBoxCaption: windows_sys::core::PSTR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct PDH_BROWSE_DLG_CONFIG_HW {
    pub _bitfield: u32,
    pub hWndOwner: super::HWND,
    pub hDataSource: PDH_HLOG,
    pub szReturnPathBuffer: windows_sys::core::PWSTR,
    pub cchReturnPathLength: super::DWORD,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: super::DWORD_PTR,
    pub CallBackStatus: PDH_STATUS,
    pub dwDefaultDetailLevel: super::DWORD,
    pub szDialogBoxCaption: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct PDH_BROWSE_DLG_CONFIG_W {
    pub _bitfield: u32,
    pub hWndOwner: super::HWND,
    pub szDataSource: windows_sys::core::PWSTR,
    pub szReturnPathBuffer: windows_sys::core::PWSTR,
    pub cchReturnPathLength: super::DWORD,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: super::DWORD_PTR,
    pub CallBackStatus: PDH_STATUS,
    pub dwDefaultDetailLevel: super::DWORD,
    pub szDialogBoxCaption: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PDH_COUNTER_INFO_A {
    pub dwLength: super::DWORD,
    pub dwType: super::DWORD,
    pub CVersion: super::DWORD,
    pub CStatus: super::DWORD,
    pub lScale: super::LONG,
    pub lDefaultScale: super::LONG,
    pub dwUserData: super::DWORD_PTR,
    pub dwQueryUserData: super::DWORD_PTR,
    pub szFullPath: windows_sys::core::PSTR,
    pub Anonymous: PDH_COUNTER_INFO_A_0,
    pub szExplainText: windows_sys::core::PSTR,
    pub DataBuffer: [super::DWORD; 1],
}
#[cfg(feature = "shared")]
impl Default for PDH_COUNTER_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PDH_COUNTER_INFO_A_0 {
    pub DataItemPath: PDH_DATA_ITEM_PATH_ELEMENTS_A,
    pub CounterPath: PDH_COUNTER_PATH_ELEMENTS_A,
    pub Anonymous: PDH_COUNTER_INFO_A_0_0,
}
#[cfg(feature = "shared")]
impl Default for PDH_COUNTER_INFO_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PDH_COUNTER_INFO_A_0_0 {
    pub szMachineName: windows_sys::core::PSTR,
    pub szObjectName: windows_sys::core::PSTR,
    pub szInstanceName: windows_sys::core::PSTR,
    pub szParentInstance: windows_sys::core::PSTR,
    pub dwInstanceIndex: super::DWORD,
    pub szCounterName: windows_sys::core::PSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PDH_COUNTER_INFO_W {
    pub dwLength: super::DWORD,
    pub dwType: super::DWORD,
    pub CVersion: super::DWORD,
    pub CStatus: super::DWORD,
    pub lScale: super::LONG,
    pub lDefaultScale: super::LONG,
    pub dwUserData: super::DWORD_PTR,
    pub dwQueryUserData: super::DWORD_PTR,
    pub szFullPath: windows_sys::core::PWSTR,
    pub Anonymous: PDH_COUNTER_INFO_W_0,
    pub szExplainText: windows_sys::core::PWSTR,
    pub DataBuffer: [super::DWORD; 1],
}
#[cfg(feature = "shared")]
impl Default for PDH_COUNTER_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PDH_COUNTER_INFO_W_0 {
    pub DataItemPath: PDH_DATA_ITEM_PATH_ELEMENTS_W,
    pub CounterPath: PDH_COUNTER_PATH_ELEMENTS_W,
    pub Anonymous: PDH_COUNTER_INFO_W_0_0,
}
#[cfg(feature = "shared")]
impl Default for PDH_COUNTER_INFO_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PDH_COUNTER_INFO_W_0_0 {
    pub szMachineName: windows_sys::core::PWSTR,
    pub szObjectName: windows_sys::core::PWSTR,
    pub szInstanceName: windows_sys::core::PWSTR,
    pub szParentInstance: windows_sys::core::PWSTR,
    pub dwInstanceIndex: super::DWORD,
    pub szCounterName: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PDH_COUNTER_PATH_ELEMENTS_A {
    pub szMachineName: windows_sys::core::PSTR,
    pub szObjectName: windows_sys::core::PSTR,
    pub szInstanceName: windows_sys::core::PSTR,
    pub szParentInstance: windows_sys::core::PSTR,
    pub dwInstanceIndex: super::DWORD,
    pub szCounterName: windows_sys::core::PSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PDH_COUNTER_PATH_ELEMENTS_W {
    pub szMachineName: windows_sys::core::PWSTR,
    pub szObjectName: windows_sys::core::PWSTR,
    pub szInstanceName: windows_sys::core::PWSTR,
    pub szParentInstance: windows_sys::core::PWSTR,
    pub dwInstanceIndex: super::DWORD,
    pub szCounterName: windows_sys::core::PWSTR,
}
#[cfg(feature = "shared")]
pub const PDH_CVERSION_WIN40: super::DWORD = 1024;
#[cfg(feature = "shared")]
pub const PDH_CVERSION_WIN50: super::DWORD = 1280;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_A {
    pub szMachineName: windows_sys::core::PSTR,
    pub ObjectGUID: windows_sys::core::GUID,
    pub dwItemId: super::DWORD,
    pub szInstanceName: windows_sys::core::PSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_W {
    pub szMachineName: windows_sys::core::PWSTR,
    pub ObjectGUID: windows_sys::core::GUID,
    pub dwItemId: super::DWORD,
    pub szInstanceName: windows_sys::core::PWSTR,
}
#[cfg(feature = "shared")]
pub const PDH_FLAGS_CLOSE_QUERY: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const PDH_FLAGS_FILE_BROWSER_ONLY: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const PDH_FMT_1000: super::DWORD = 8192;
#[cfg(feature = "shared")]
pub const PDH_FMT_ANSI: super::DWORD = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PDH_FMT_COUNTERVALUE {
    pub CStatus: super::DWORD,
    pub Anonymous: PDH_FMT_COUNTERVALUE_0,
}
#[cfg(feature = "shared")]
impl Default for PDH_FMT_COUNTERVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PDH_FMT_COUNTERVALUE_0 {
    pub longValue: super::LONG,
    pub doubleValue: f64,
    pub largeValue: super::LONGLONG,
    pub AnsiStringValue: windows_sys::core::PCSTR,
    pub WideStringValue: windows_sys::core::PCWSTR,
}
#[cfg(feature = "shared")]
impl Default for PDH_FMT_COUNTERVALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PDH_FMT_COUNTERVALUE_ITEM_A {
    pub szName: windows_sys::core::PSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
#[cfg(feature = "shared")]
impl Default for PDH_FMT_COUNTERVALUE_ITEM_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PDH_FMT_COUNTERVALUE_ITEM_W {
    pub szName: windows_sys::core::PWSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
#[cfg(feature = "shared")]
impl Default for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub const PDH_FMT_DOUBLE: super::DWORD = 512;
#[cfg(feature = "shared")]
pub const PDH_FMT_LARGE: super::DWORD = 1024;
#[cfg(feature = "shared")]
pub const PDH_FMT_LONG: super::DWORD = 256;
#[cfg(feature = "shared")]
pub const PDH_FMT_NOCAP100: super::DWORD = 32768;
#[cfg(feature = "shared")]
pub const PDH_FMT_NODATA: super::DWORD = 16384;
#[cfg(feature = "shared")]
pub const PDH_FMT_NOSCALE: super::DWORD = 4096;
#[cfg(feature = "shared")]
pub const PDH_FMT_RAW: super::DWORD = 16;
#[cfg(feature = "shared")]
pub const PDH_FMT_UNICODE: super::DWORD = 64;
pub type PDH_HCOUNTER = *mut core::ffi::c_void;
pub type PDH_HLOG = *mut core::ffi::c_void;
pub type PDH_HQUERY = *mut core::ffi::c_void;
#[cfg(feature = "shared")]
pub const PDH_LOG_ACCESS_MASK: super::DWORD = 983040;
#[cfg(feature = "shared")]
pub const PDH_LOG_CREATE_ALWAYS: super::DWORD = 2;
#[cfg(feature = "shared")]
pub const PDH_LOG_CREATE_MASK: super::DWORD = 15;
#[cfg(feature = "shared")]
pub const PDH_LOG_CREATE_NEW: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const PDH_LOG_OPEN_ALWAYS: super::DWORD = 3;
#[cfg(feature = "shared")]
pub const PDH_LOG_OPEN_EXISTING: super::DWORD = 4;
#[cfg(feature = "shared")]
pub const PDH_LOG_OPT_APPEND: super::DWORD = 134217728;
#[cfg(feature = "shared")]
pub const PDH_LOG_OPT_CIRCULAR: super::DWORD = 33554432;
#[cfg(feature = "shared")]
pub const PDH_LOG_OPT_MASK: super::DWORD = 251658240;
#[cfg(feature = "shared")]
pub const PDH_LOG_OPT_MAX_IS_BYTES: super::DWORD = 67108864;
#[cfg(feature = "shared")]
pub const PDH_LOG_OPT_USER_STRING: super::DWORD = 16777216;
#[cfg(feature = "shared")]
pub const PDH_LOG_READ_ACCESS: super::DWORD = 65536;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwLogQuota: super::DWORD,
    pub szLogFileCaption: windows_sys::core::PSTR,
    pub szDefaultDir: windows_sys::core::PSTR,
    pub szBaseFileName: windows_sys::core::PSTR,
    pub dwFileType: super::DWORD,
    pub dwReserved: super::DWORD,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_A_0,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for PDH_LOG_SERVICE_QUERY_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_A_0_0,
    pub Anonymous2: PDH_LOG_SERVICE_QUERY_INFO_A_0_1,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    pub PdlAutoNameInterval: super::DWORD,
    pub PdlAutoNameUnits: super::DWORD,
    pub PdlCommandFilename: windows_sys::core::PSTR,
    pub PdlCounterList: windows_sys::core::PSTR,
    pub PdlAutoNameFormat: super::DWORD,
    pub PdlSampleInterval: super::DWORD,
    pub PdlLogStartTime: super::FILETIME,
    pub PdlLogEndTime: super::FILETIME,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    pub TlNumberOfBuffers: super::DWORD,
    pub TlMinimumBuffers: super::DWORD,
    pub TlMaximumBuffers: super::DWORD,
    pub TlFreeBuffers: super::DWORD,
    pub TlBufferSize: super::DWORD,
    pub TlEventsLost: super::DWORD,
    pub TlLoggerThreadId: super::DWORD,
    pub TlBuffersWritten: super::DWORD,
    pub TlLogHandle: super::DWORD,
    pub TlLogFileName: windows_sys::core::PSTR,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwLogQuota: super::DWORD,
    pub szLogFileCaption: windows_sys::core::PWSTR,
    pub szDefaultDir: windows_sys::core::PWSTR,
    pub szBaseFileName: windows_sys::core::PWSTR,
    pub dwFileType: super::DWORD,
    pub dwReserved: super::DWORD,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_W_0,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for PDH_LOG_SERVICE_QUERY_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_W_0_0,
    pub Anonymous2: PDH_LOG_SERVICE_QUERY_INFO_W_0_1,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    pub PdlAutoNameInterval: super::DWORD,
    pub PdlAutoNameUnits: super::DWORD,
    pub PdlCommandFilename: windows_sys::core::PWSTR,
    pub PdlCounterList: windows_sys::core::PWSTR,
    pub PdlAutoNameFormat: super::DWORD,
    pub PdlSampleInterval: super::DWORD,
    pub PdlLogStartTime: super::FILETIME,
    pub PdlLogEndTime: super::FILETIME,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    pub TlNumberOfBuffers: super::DWORD,
    pub TlMinimumBuffers: super::DWORD,
    pub TlMaximumBuffers: super::DWORD,
    pub TlFreeBuffers: super::DWORD,
    pub TlBufferSize: super::DWORD,
    pub TlEventsLost: super::DWORD,
    pub TlLoggerThreadId: super::DWORD,
    pub TlBuffersWritten: super::DWORD,
    pub TlLogHandle: super::DWORD,
    pub TlLogFileName: windows_sys::core::PWSTR,
}
pub const PDH_LOG_TYPE_BINARY: i32 = 8;
pub const PDH_LOG_TYPE_CSV: i32 = 1;
pub const PDH_LOG_TYPE_PERFMON: i32 = 6;
pub const PDH_LOG_TYPE_RETIRED_BIN: i32 = 3;
pub const PDH_LOG_TYPE_SQL: i32 = 7;
pub const PDH_LOG_TYPE_TRACE_GENERIC: i32 = 5;
pub const PDH_LOG_TYPE_TRACE_KERNEL: i32 = 4;
pub const PDH_LOG_TYPE_TSV: i32 = 2;
pub const PDH_LOG_TYPE_UNDEFINED: i32 = 0;
#[cfg(feature = "shared")]
pub const PDH_LOG_UPDATE_ACCESS: super::DWORD = 262144;
#[cfg(feature = "shared")]
pub const PDH_LOG_WRITE_ACCESS: super::DWORD = 131072;
pub const PDH_MAX_COUNTER_NAME: i32 = 1024;
pub const PDH_MAX_COUNTER_PATH: i32 = 2048;
pub const PDH_MAX_DATASOURCE_PATH: i32 = 1024;
pub const PDH_MAX_INSTANCE_NAME: i32 = 1024;
pub const PDH_MAX_SCALE: i32 = 7;
pub const PDH_MIN_SCALE: i32 = -7;
pub const PDH_NOEXPANDCOUNTERS: i32 = 1;
pub const PDH_NOEXPANDINSTANCES: i32 = 2;
#[cfg(feature = "shared")]
pub const PDH_OBJECT_HAS_INSTANCES: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const PDH_PATH_WBEM_INPUT: super::DWORD = 2;
#[cfg(feature = "shared")]
pub const PDH_PATH_WBEM_RESULT: super::DWORD = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PDH_RAW_LOG_RECORD {
    pub dwStructureSize: super::DWORD,
    pub dwRecordType: super::DWORD,
    pub dwItems: super::DWORD,
    pub RawBytes: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for PDH_RAW_LOG_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PDH_REFRESHCOUNTERS: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PDH_STATISTICS {
    pub dwFormat: super::DWORD,
    pub count: super::DWORD,
    pub min: PDH_FMT_COUNTERVALUE,
    pub max: PDH_FMT_COUNTERVALUE,
    pub mean: PDH_FMT_COUNTERVALUE,
}
#[cfg(feature = "shared")]
impl Default for PDH_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PDH_STATUS = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PDH_TIME_INFO {
    pub StartTime: super::LONGLONG,
    pub EndTime: super::LONGLONG,
    pub SampleCount: super::DWORD,
}
#[cfg(feature = "shared")]
pub const PDH_VERSION: super::DWORD = 1283;
#[cfg(feature = "shared")]
pub const PERF_DETAIL_COSTLY: super::DWORD = 65536;
#[cfg(feature = "shared")]
pub const PERF_DETAIL_STANDARD: super::DWORD = 65535;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PPDH_BROWSE_DLG_CONFIG_A = *mut PDH_BROWSE_DLG_CONFIG_A;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PPDH_BROWSE_DLG_CONFIG_HA = *mut PDH_BROWSE_DLG_CONFIG_HA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PPDH_BROWSE_DLG_CONFIG_HW = *mut PDH_BROWSE_DLG_CONFIG_HW;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PPDH_BROWSE_DLG_CONFIG_W = *mut PDH_BROWSE_DLG_CONFIG_W;
#[cfg(feature = "shared")]
pub type PPDH_COUNTER_INFO_A = *mut PDH_COUNTER_INFO_A;
#[cfg(feature = "shared")]
pub type PPDH_COUNTER_INFO_W = *mut PDH_COUNTER_INFO_W;
#[cfg(feature = "shared")]
pub type PPDH_COUNTER_PATH_ELEMENTS_A = *mut PDH_COUNTER_PATH_ELEMENTS_A;
#[cfg(feature = "shared")]
pub type PPDH_COUNTER_PATH_ELEMENTS_W = *mut PDH_COUNTER_PATH_ELEMENTS_W;
#[cfg(feature = "shared")]
pub type PPDH_DATA_ITEM_PATH_ELEMENTS_A = *mut PDH_DATA_ITEM_PATH_ELEMENTS_A;
#[cfg(feature = "shared")]
pub type PPDH_DATA_ITEM_PATH_ELEMENTS_W = *mut PDH_DATA_ITEM_PATH_ELEMENTS_W;
#[cfg(feature = "shared")]
pub type PPDH_FMT_COUNTERVALUE = *mut PDH_FMT_COUNTERVALUE;
#[cfg(feature = "shared")]
pub type PPDH_FMT_COUNTERVALUE_ITEM_A = *mut PDH_FMT_COUNTERVALUE_ITEM_A;
#[cfg(feature = "shared")]
pub type PPDH_FMT_COUNTERVALUE_ITEM_W = *mut PDH_FMT_COUNTERVALUE_ITEM_W;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PPDH_LOG_SERVICE_QUERY_INFO_A = *mut PDH_LOG_SERVICE_QUERY_INFO_A;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PPDH_LOG_SERVICE_QUERY_INFO_W = *mut PDH_LOG_SERVICE_QUERY_INFO_W;
#[cfg(feature = "shared")]
pub type PPDH_RAW_LOG_RECORD = *mut PDH_RAW_LOG_RECORD;
#[cfg(feature = "shared")]
pub type PPDH_STATISTICS = *mut PDH_STATISTICS;
#[cfg(feature = "shared")]
pub type PPDH_TIME_INFO = *mut PDH_TIME_INFO;
