#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PdhAddCounterA<P1>(hquery: PDH_HQUERY, szfullcounterpath: P1, dwuserdata: super::DWORD_PTR, phcounter: *mut PDH_HCOUNTER) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhAddCounterA(hquery : PDH_HQUERY, szfullcounterpath : windows_core::PCSTR, dwuserdata : super::DWORD_PTR, phcounter : *mut PDH_HCOUNTER) -> PDH_STATUS);
    unsafe { PdhAddCounterA(hquery, szfullcounterpath.param().abi(), dwuserdata, phcounter as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PdhAddCounterW<P1>(hquery: PDH_HQUERY, szfullcounterpath: P1, dwuserdata: super::DWORD_PTR, phcounter: *mut PDH_HCOUNTER) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhAddCounterW(hquery : PDH_HQUERY, szfullcounterpath : windows_core::PCWSTR, dwuserdata : super::DWORD_PTR, phcounter : *mut PDH_HCOUNTER) -> PDH_STATUS);
    unsafe { PdhAddCounterW(hquery, szfullcounterpath.param().abi(), dwuserdata, phcounter as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PdhAddEnglishCounterA<P1>(hquery: PDH_HQUERY, szfullcounterpath: P1, dwuserdata: super::DWORD_PTR, phcounter: *mut PDH_HCOUNTER) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhAddEnglishCounterA(hquery : PDH_HQUERY, szfullcounterpath : windows_core::PCSTR, dwuserdata : super::DWORD_PTR, phcounter : *mut PDH_HCOUNTER) -> PDH_STATUS);
    unsafe { PdhAddEnglishCounterA(hquery, szfullcounterpath.param().abi(), dwuserdata, phcounter as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PdhAddEnglishCounterW<P1>(hquery: PDH_HQUERY, szfullcounterpath: P1, dwuserdata: super::DWORD_PTR, phcounter: *mut PDH_HCOUNTER) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhAddEnglishCounterW(hquery : PDH_HQUERY, szfullcounterpath : windows_core::PCWSTR, dwuserdata : super::DWORD_PTR, phcounter : *mut PDH_HCOUNTER) -> PDH_STATUS);
    unsafe { PdhAddEnglishCounterW(hquery, szfullcounterpath.param().abi(), dwuserdata, phcounter as _) }
}
#[inline]
pub unsafe fn PdhBindInputDataSourceA<P1>(phdatasource: *mut PDH_HLOG, logfilenamelist: P1) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhBindInputDataSourceA(phdatasource : *mut PDH_HLOG, logfilenamelist : windows_core::PCSTR) -> PDH_STATUS);
    unsafe { PdhBindInputDataSourceA(phdatasource as _, logfilenamelist.param().abi()) }
}
#[inline]
pub unsafe fn PdhBindInputDataSourceW<P1>(phdatasource: *mut PDH_HLOG, logfilenamelist: P1) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhBindInputDataSourceW(phdatasource : *mut PDH_HLOG, logfilenamelist : windows_core::PCWSTR) -> PDH_STATUS);
    unsafe { PdhBindInputDataSourceW(phdatasource as _, logfilenamelist.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn PdhBrowseCountersA(pbrowsedlgdata: PPDH_BROWSE_DLG_CONFIG_A) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhBrowseCountersA(pbrowsedlgdata : PPDH_BROWSE_DLG_CONFIG_A) -> PDH_STATUS);
    unsafe { PdhBrowseCountersA(pbrowsedlgdata) }
}
#[cfg(all(feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn PdhBrowseCountersHA(pbrowsedlgdata: PPDH_BROWSE_DLG_CONFIG_HA) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhBrowseCountersHA(pbrowsedlgdata : PPDH_BROWSE_DLG_CONFIG_HA) -> PDH_STATUS);
    unsafe { PdhBrowseCountersHA(pbrowsedlgdata) }
}
#[cfg(all(feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn PdhBrowseCountersHW(pbrowsedlgdata: PPDH_BROWSE_DLG_CONFIG_HW) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhBrowseCountersHW(pbrowsedlgdata : PPDH_BROWSE_DLG_CONFIG_HW) -> PDH_STATUS);
    unsafe { PdhBrowseCountersHW(pbrowsedlgdata) }
}
#[cfg(all(feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn PdhBrowseCountersW(pbrowsedlgdata: PPDH_BROWSE_DLG_CONFIG_W) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhBrowseCountersW(pbrowsedlgdata : PPDH_BROWSE_DLG_CONFIG_W) -> PDH_STATUS);
    unsafe { PdhBrowseCountersW(pbrowsedlgdata) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhCalculateCounterFromRawValue(hcounter: PDH_HCOUNTER, dwformat: super::DWORD, rawvalue1: PPDH_RAW_COUNTER, rawvalue2: PPDH_RAW_COUNTER, fmtvalue: PPDH_FMT_COUNTERVALUE) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhCalculateCounterFromRawValue(hcounter : PDH_HCOUNTER, dwformat : super::DWORD, rawvalue1 : PPDH_RAW_COUNTER, rawvalue2 : PPDH_RAW_COUNTER, fmtvalue : PPDH_FMT_COUNTERVALUE) -> PDH_STATUS);
    unsafe { PdhCalculateCounterFromRawValue(hcounter, dwformat, rawvalue1, rawvalue2, fmtvalue as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PdhCloseLog(hlog: PDH_HLOG, dwflags: super::DWORD) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhCloseLog(hlog : PDH_HLOG, dwflags : super::DWORD) -> PDH_STATUS);
    unsafe { PdhCloseLog(hlog, dwflags) }
}
#[inline]
pub unsafe fn PdhCloseQuery(hquery: PDH_HQUERY) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhCloseQuery(hquery : PDH_HQUERY) -> PDH_STATUS);
    unsafe { PdhCloseQuery(hquery as _) }
}
#[inline]
pub unsafe fn PdhCollectQueryData(hquery: PDH_HQUERY) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhCollectQueryData(hquery : PDH_HQUERY) -> PDH_STATUS);
    unsafe { PdhCollectQueryData(hquery as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PdhCollectQueryDataEx(hquery: PDH_HQUERY, dwintervaltime: super::DWORD, hnewdataevent: super::HANDLE) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhCollectQueryDataEx(hquery : PDH_HQUERY, dwintervaltime : super::DWORD, hnewdataevent : super::HANDLE) -> PDH_STATUS);
    unsafe { PdhCollectQueryDataEx(hquery, dwintervaltime, hnewdataevent) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PdhCollectQueryDataWithTime(hquery: PDH_HQUERY, plltimestamp: *mut super::LONGLONG) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhCollectQueryDataWithTime(hquery : PDH_HQUERY, plltimestamp : *mut super::LONGLONG) -> PDH_STATUS);
    unsafe { PdhCollectQueryDataWithTime(hquery as _, plltimestamp as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhComputeCounterStatistics(hcounter: PDH_HCOUNTER, dwformat: super::DWORD, dwfirstentry: super::DWORD, dwnumentries: super::DWORD, lprawvaluearray: PPDH_RAW_COUNTER, data: PPDH_STATISTICS) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhComputeCounterStatistics(hcounter : PDH_HCOUNTER, dwformat : super::DWORD, dwfirstentry : super::DWORD, dwnumentries : super::DWORD, lprawvaluearray : PPDH_RAW_COUNTER, data : PPDH_STATISTICS) -> PDH_STATUS);
    unsafe { PdhComputeCounterStatistics(hcounter, dwformat, dwfirstentry, dwnumentries, lprawvaluearray, data as _) }
}
#[inline]
pub unsafe fn PdhConnectMachineA<P0>(szmachinename: P0) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhConnectMachineA(szmachinename : windows_core::PCSTR) -> PDH_STATUS);
    unsafe { PdhConnectMachineA(szmachinename.param().abi()) }
}
#[inline]
pub unsafe fn PdhConnectMachineW<P0>(szmachinename: P0) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhConnectMachineW(szmachinename : windows_core::PCWSTR) -> PDH_STATUS);
    unsafe { PdhConnectMachineW(szmachinename.param().abi()) }
}
#[inline]
pub unsafe fn PdhCreateSQLTablesA<P0>(szdatasource: P0) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhCreateSQLTablesA(szdatasource : windows_core::PCSTR) -> PDH_STATUS);
    unsafe { PdhCreateSQLTablesA(szdatasource.param().abi()) }
}
#[inline]
pub unsafe fn PdhCreateSQLTablesW<P0>(szdatasource: P0) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhCreateSQLTablesW(szdatasource : windows_core::PCWSTR) -> PDH_STATUS);
    unsafe { PdhCreateSQLTablesW(szdatasource.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn PdhEnumLogSetNamesA<P0>(szdatasource: P0, mszdatasetnamelist: Option<super::PZZSTR>, pcchbufferlength: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumLogSetNamesA(szdatasource : windows_core::PCSTR, mszdatasetnamelist : super::PZZSTR, pcchbufferlength : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhEnumLogSetNamesA(szdatasource.param().abi(), mszdatasetnamelist.unwrap_or(core::mem::zeroed()) as _, pcchbufferlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn PdhEnumLogSetNamesW<P0>(szdatasource: P0, mszdatasetnamelist: Option<super::PZZWSTR>, pcchbufferlength: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumLogSetNamesW(szdatasource : windows_core::PCWSTR, mszdatasetnamelist : super::PZZWSTR, pcchbufferlength : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhEnumLogSetNamesW(szdatasource.param().abi(), mszdatasetnamelist.unwrap_or(core::mem::zeroed()) as _, pcchbufferlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn PdhEnumMachinesA<P0>(szdatasource: P0, mszmachinelist: Option<super::PZZSTR>, pcchbuffersize: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumMachinesA(szdatasource : windows_core::PCSTR, mszmachinelist : super::PZZSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhEnumMachinesA(szdatasource.param().abi(), mszmachinelist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn PdhEnumMachinesHA(hdatasource: Option<PDH_HLOG>, mszmachinelist: Option<super::PZZSTR>, pcchbuffersize: super::LPDWORD) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhEnumMachinesHA(hdatasource : PDH_HLOG, mszmachinelist : super::PZZSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhEnumMachinesHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, mszmachinelist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn PdhEnumMachinesHW(hdatasource: Option<PDH_HLOG>, mszmachinelist: Option<super::PZZWSTR>, pcchbuffersize: super::LPDWORD) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhEnumMachinesHW(hdatasource : PDH_HLOG, mszmachinelist : super::PZZWSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhEnumMachinesHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, mszmachinelist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn PdhEnumMachinesW<P0>(szdatasource: P0, mszmachinelist: Option<super::PZZWSTR>, pcchbuffersize: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumMachinesW(szdatasource : windows_core::PCWSTR, mszmachinelist : super::PZZWSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhEnumMachinesW(szdatasource.param().abi(), mszmachinelist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PdhEnumObjectItemsA<P0, P1, P2>(szdatasource: P0, szmachinename: P1, szobjectname: P2, mszcounterlist: Option<super::PZZSTR>, pcchcounterlistlength: super::LPDWORD, mszinstancelist: Option<super::PZZSTR>, pcchinstancelistlength: super::LPDWORD, dwdetaillevel: super::DWORD, dwflags: super::DWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectItemsA(szdatasource : windows_core::PCSTR, szmachinename : windows_core::PCSTR, szobjectname : windows_core::PCSTR, mszcounterlist : super::PZZSTR, pcchcounterlistlength : super::LPDWORD, mszinstancelist : super::PZZSTR, pcchinstancelistlength : super::LPDWORD, dwdetaillevel : super::DWORD, dwflags : super::DWORD) -> PDH_STATUS);
    unsafe { PdhEnumObjectItemsA(szdatasource.param().abi(), szmachinename.param().abi(), szobjectname.param().abi(), mszcounterlist.unwrap_or(core::mem::zeroed()) as _, pcchcounterlistlength as _, mszinstancelist.unwrap_or(core::mem::zeroed()) as _, pcchinstancelistlength as _, dwdetaillevel, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PdhEnumObjectItemsHA<P1, P2>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szobjectname: P2, mszcounterlist: Option<super::PZZSTR>, pcchcounterlistlength: super::LPDWORD, mszinstancelist: Option<super::PZZSTR>, pcchinstancelistlength: super::LPDWORD, dwdetaillevel: super::DWORD, dwflags: super::DWORD) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectItemsHA(hdatasource : PDH_HLOG, szmachinename : windows_core::PCSTR, szobjectname : windows_core::PCSTR, mszcounterlist : super::PZZSTR, pcchcounterlistlength : super::LPDWORD, mszinstancelist : super::PZZSTR, pcchinstancelistlength : super::LPDWORD, dwdetaillevel : super::DWORD, dwflags : super::DWORD) -> PDH_STATUS);
    unsafe { PdhEnumObjectItemsHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szobjectname.param().abi(), mszcounterlist.unwrap_or(core::mem::zeroed()) as _, pcchcounterlistlength as _, mszinstancelist.unwrap_or(core::mem::zeroed()) as _, pcchinstancelistlength as _, dwdetaillevel, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PdhEnumObjectItemsHW<P1, P2>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szobjectname: P2, mszcounterlist: Option<super::PZZWSTR>, pcchcounterlistlength: super::LPDWORD, mszinstancelist: Option<super::PZZWSTR>, pcchinstancelistlength: super::LPDWORD, dwdetaillevel: super::DWORD, dwflags: super::DWORD) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectItemsHW(hdatasource : PDH_HLOG, szmachinename : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, mszcounterlist : super::PZZWSTR, pcchcounterlistlength : super::LPDWORD, mszinstancelist : super::PZZWSTR, pcchinstancelistlength : super::LPDWORD, dwdetaillevel : super::DWORD, dwflags : super::DWORD) -> PDH_STATUS);
    unsafe { PdhEnumObjectItemsHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szobjectname.param().abi(), mszcounterlist.unwrap_or(core::mem::zeroed()) as _, pcchcounterlistlength as _, mszinstancelist.unwrap_or(core::mem::zeroed()) as _, pcchinstancelistlength as _, dwdetaillevel, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PdhEnumObjectItemsW<P0, P1, P2>(szdatasource: P0, szmachinename: P1, szobjectname: P2, mszcounterlist: Option<super::PZZWSTR>, pcchcounterlistlength: super::LPDWORD, mszinstancelist: Option<super::PZZWSTR>, pcchinstancelistlength: super::LPDWORD, dwdetaillevel: super::DWORD, dwflags: super::DWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectItemsW(szdatasource : windows_core::PCWSTR, szmachinename : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, mszcounterlist : super::PZZWSTR, pcchcounterlistlength : super::LPDWORD, mszinstancelist : super::PZZWSTR, pcchinstancelistlength : super::LPDWORD, dwdetaillevel : super::DWORD, dwflags : super::DWORD) -> PDH_STATUS);
    unsafe { PdhEnumObjectItemsW(szdatasource.param().abi(), szmachinename.param().abi(), szobjectname.param().abi(), mszcounterlist.unwrap_or(core::mem::zeroed()) as _, pcchcounterlistlength as _, mszinstancelist.unwrap_or(core::mem::zeroed()) as _, pcchinstancelistlength as _, dwdetaillevel, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PdhEnumObjectsA<P0, P1>(szdatasource: P0, szmachinename: P1, mszobjectlist: Option<super::PZZSTR>, pcchbuffersize: super::LPDWORD, dwdetaillevel: super::DWORD, brefresh: bool) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectsA(szdatasource : windows_core::PCSTR, szmachinename : windows_core::PCSTR, mszobjectlist : super::PZZSTR, pcchbuffersize : super::LPDWORD, dwdetaillevel : super::DWORD, brefresh : windows_core::BOOL) -> PDH_STATUS);
    unsafe { PdhEnumObjectsA(szdatasource.param().abi(), szmachinename.param().abi(), mszobjectlist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwdetaillevel, brefresh.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PdhEnumObjectsHA<P1>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, mszobjectlist: Option<super::PZZSTR>, pcchbuffersize: super::LPDWORD, dwdetaillevel: super::DWORD, brefresh: bool) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectsHA(hdatasource : PDH_HLOG, szmachinename : windows_core::PCSTR, mszobjectlist : super::PZZSTR, pcchbuffersize : super::LPDWORD, dwdetaillevel : super::DWORD, brefresh : windows_core::BOOL) -> PDH_STATUS);
    unsafe { PdhEnumObjectsHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), mszobjectlist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwdetaillevel, brefresh.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PdhEnumObjectsHW<P1>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, mszobjectlist: Option<super::PZZWSTR>, pcchbuffersize: super::LPDWORD, dwdetaillevel: super::DWORD, brefresh: bool) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectsHW(hdatasource : PDH_HLOG, szmachinename : windows_core::PCWSTR, mszobjectlist : super::PZZWSTR, pcchbuffersize : super::LPDWORD, dwdetaillevel : super::DWORD, brefresh : windows_core::BOOL) -> PDH_STATUS);
    unsafe { PdhEnumObjectsHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), mszobjectlist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwdetaillevel, brefresh.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PdhEnumObjectsW<P0, P1>(szdatasource: P0, szmachinename: P1, mszobjectlist: Option<super::PZZWSTR>, pcchbuffersize: super::LPDWORD, dwdetaillevel: super::DWORD, brefresh: bool) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectsW(szdatasource : windows_core::PCWSTR, szmachinename : windows_core::PCWSTR, mszobjectlist : super::PZZWSTR, pcchbuffersize : super::LPDWORD, dwdetaillevel : super::DWORD, brefresh : windows_core::BOOL) -> PDH_STATUS);
    unsafe { PdhEnumObjectsW(szdatasource.param().abi(), szmachinename.param().abi(), mszobjectlist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwdetaillevel, brefresh.into()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn PdhExpandCounterPathA<P0>(szwildcardpath: P0, mszexpandedpathlist: Option<super::PZZSTR>, pcchpathlistlength: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandCounterPathA(szwildcardpath : windows_core::PCSTR, mszexpandedpathlist : super::PZZSTR, pcchpathlistlength : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhExpandCounterPathA(szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn PdhExpandCounterPathW<P0>(szwildcardpath: P0, mszexpandedpathlist: Option<super::PZZWSTR>, pcchpathlistlength: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandCounterPathW(szwildcardpath : windows_core::PCWSTR, mszexpandedpathlist : super::PZZWSTR, pcchpathlistlength : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhExpandCounterPathW(szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PdhExpandWildCardPathA<P0, P1>(szdatasource: P0, szwildcardpath: P1, mszexpandedpathlist: Option<super::PZZSTR>, pcchpathlistlength: super::LPDWORD, dwflags: super::DWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandWildCardPathA(szdatasource : windows_core::PCSTR, szwildcardpath : windows_core::PCSTR, mszexpandedpathlist : super::PZZSTR, pcchpathlistlength : super::LPDWORD, dwflags : super::DWORD) -> PDH_STATUS);
    unsafe { PdhExpandWildCardPathA(szdatasource.param().abi(), szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PdhExpandWildCardPathHA<P1>(hdatasource: Option<PDH_HLOG>, szwildcardpath: P1, mszexpandedpathlist: Option<super::PZZSTR>, pcchpathlistlength: super::LPDWORD, dwflags: super::DWORD) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandWildCardPathHA(hdatasource : PDH_HLOG, szwildcardpath : windows_core::PCSTR, mszexpandedpathlist : super::PZZSTR, pcchpathlistlength : super::LPDWORD, dwflags : super::DWORD) -> PDH_STATUS);
    unsafe { PdhExpandWildCardPathHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PdhExpandWildCardPathHW<P1>(hdatasource: Option<PDH_HLOG>, szwildcardpath: P1, mszexpandedpathlist: Option<super::PZZWSTR>, pcchpathlistlength: super::LPDWORD, dwflags: super::DWORD) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandWildCardPathHW(hdatasource : PDH_HLOG, szwildcardpath : windows_core::PCWSTR, mszexpandedpathlist : super::PZZWSTR, pcchpathlistlength : super::LPDWORD, dwflags : super::DWORD) -> PDH_STATUS);
    unsafe { PdhExpandWildCardPathHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PdhExpandWildCardPathW<P0, P1>(szdatasource: P0, szwildcardpath: P1, mszexpandedpathlist: Option<super::PZZWSTR>, pcchpathlistlength: super::LPDWORD, dwflags: super::DWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandWildCardPathW(szdatasource : windows_core::PCWSTR, szwildcardpath : windows_core::PCWSTR, mszexpandedpathlist : super::PZZWSTR, pcchpathlistlength : super::LPDWORD, dwflags : super::DWORD) -> PDH_STATUS);
    unsafe { PdhExpandWildCardPathW(szdatasource.param().abi(), szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhFormatFromRawValue(dwcountertype: super::DWORD, dwformat: super::DWORD, ptimebase: Option<*const super::LONGLONG>, prawvalue1: PPDH_RAW_COUNTER, prawvalue2: PPDH_RAW_COUNTER, pfmtvalue: PPDH_FMT_COUNTERVALUE) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhFormatFromRawValue(dwcountertype : super::DWORD, dwformat : super::DWORD, ptimebase : *const super::LONGLONG, prawvalue1 : PPDH_RAW_COUNTER, prawvalue2 : PPDH_RAW_COUNTER, pfmtvalue : PPDH_FMT_COUNTERVALUE) -> PDH_STATUS);
    unsafe { PdhFormatFromRawValue(dwcountertype, dwformat, ptimebase.unwrap_or(core::mem::zeroed()) as _, prawvalue1, prawvalue2, pfmtvalue as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhGetCounterInfoA(hcounter: PDH_HCOUNTER, bretrieveexplaintext: super::BOOLEAN, pdwbuffersize: super::LPDWORD, lpbuffer: Option<PPDH_COUNTER_INFO_A>) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetCounterInfoA(hcounter : PDH_HCOUNTER, bretrieveexplaintext : super::BOOLEAN, pdwbuffersize : super::LPDWORD, lpbuffer : PPDH_COUNTER_INFO_A) -> PDH_STATUS);
    unsafe { PdhGetCounterInfoA(hcounter, bretrieveexplaintext, pdwbuffersize as _, lpbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhGetCounterInfoW(hcounter: PDH_HCOUNTER, bretrieveexplaintext: super::BOOLEAN, pdwbuffersize: super::LPDWORD, lpbuffer: Option<PPDH_COUNTER_INFO_W>) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetCounterInfoW(hcounter : PDH_HCOUNTER, bretrieveexplaintext : super::BOOLEAN, pdwbuffersize : super::LPDWORD, lpbuffer : PPDH_COUNTER_INFO_W) -> PDH_STATUS);
    unsafe { PdhGetCounterInfoW(hcounter, bretrieveexplaintext, pdwbuffersize as _, lpbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PdhGetCounterTimeBase(hcounter: PDH_HCOUNTER, ptimebase: *mut super::LONGLONG) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetCounterTimeBase(hcounter : PDH_HCOUNTER, ptimebase : *mut super::LONGLONG) -> PDH_STATUS);
    unsafe { PdhGetCounterTimeBase(hcounter, ptimebase as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeA<P0>(szdatasource: P0, pdwnumentries: super::LPDWORD, pinfo: PPDH_TIME_INFO, pdwbuffersize: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeA(szdatasource : windows_core::PCSTR, pdwnumentries : super::LPDWORD, pinfo : PPDH_TIME_INFO, pdwbuffersize : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhGetDataSourceTimeRangeA(szdatasource.param().abi(), pdwnumentries as _, pinfo as _, pdwbuffersize as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeH(hdatasource: Option<PDH_HLOG>, pdwnumentries: super::LPDWORD, pinfo: PPDH_TIME_INFO, pdwbuffersize: super::LPDWORD) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeH(hdatasource : PDH_HLOG, pdwnumentries : super::LPDWORD, pinfo : PPDH_TIME_INFO, pdwbuffersize : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhGetDataSourceTimeRangeH(hdatasource.unwrap_or(core::mem::zeroed()) as _, pdwnumentries as _, pinfo as _, pdwbuffersize as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeW<P0>(szdatasource: P0, pdwnumentries: super::LPDWORD, pinfo: PPDH_TIME_INFO, pdwbuffersize: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeW(szdatasource : windows_core::PCWSTR, pdwnumentries : super::LPDWORD, pinfo : PPDH_TIME_INFO, pdwbuffersize : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhGetDataSourceTimeRangeW(szdatasource.param().abi(), pdwnumentries as _, pinfo as _, pdwbuffersize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterA<P0, P1, P2>(szdatasource: P0, szmachinename: P1, szobjectname: P2, szdefaultcountername: Option<windows_core::PSTR>, pcchbuffersize: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterA(szdatasource : windows_core::PCSTR, szmachinename : windows_core::PCSTR, szobjectname : windows_core::PCSTR, szdefaultcountername : windows_core::PSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhGetDefaultPerfCounterA(szdatasource.param().abi(), szmachinename.param().abi(), szobjectname.param().abi(), szdefaultcountername.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterHA<P1, P2>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szobjectname: P2, szdefaultcountername: Option<windows_core::PSTR>, pcchbuffersize: super::LPDWORD) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterHA(hdatasource : PDH_HLOG, szmachinename : windows_core::PCSTR, szobjectname : windows_core::PCSTR, szdefaultcountername : windows_core::PSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhGetDefaultPerfCounterHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szobjectname.param().abi(), szdefaultcountername.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterHW<P1, P2>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szobjectname: P2, szdefaultcountername: Option<windows_core::PWSTR>, pcchbuffersize: super::LPDWORD) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterHW(hdatasource : PDH_HLOG, szmachinename : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, szdefaultcountername : windows_core::PWSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhGetDefaultPerfCounterHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szobjectname.param().abi(), szdefaultcountername.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterW<P0, P1, P2>(szdatasource: P0, szmachinename: P1, szobjectname: P2, szdefaultcountername: Option<windows_core::PWSTR>, pcchbuffersize: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterW(szdatasource : windows_core::PCWSTR, szmachinename : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, szdefaultcountername : windows_core::PWSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhGetDefaultPerfCounterW(szdatasource.param().abi(), szmachinename.param().abi(), szobjectname.param().abi(), szdefaultcountername.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectA<P0, P1>(szdatasource: P0, szmachinename: P1, szdefaultobjectname: Option<windows_core::PSTR>, pcchbuffersize: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectA(szdatasource : windows_core::PCSTR, szmachinename : windows_core::PCSTR, szdefaultobjectname : windows_core::PSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhGetDefaultPerfObjectA(szdatasource.param().abi(), szmachinename.param().abi(), szdefaultobjectname.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectHA<P1>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szdefaultobjectname: Option<windows_core::PSTR>, pcchbuffersize: super::LPDWORD) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectHA(hdatasource : PDH_HLOG, szmachinename : windows_core::PCSTR, szdefaultobjectname : windows_core::PSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhGetDefaultPerfObjectHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szdefaultobjectname.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectHW<P1>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szdefaultobjectname: Option<windows_core::PWSTR>, pcchbuffersize: super::LPDWORD) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectHW(hdatasource : PDH_HLOG, szmachinename : windows_core::PCWSTR, szdefaultobjectname : windows_core::PWSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhGetDefaultPerfObjectHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szdefaultobjectname.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectW<P0, P1>(szdatasource: P0, szmachinename: P1, szdefaultobjectname: Option<windows_core::PWSTR>, pcchbuffersize: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectW(szdatasource : windows_core::PCWSTR, szmachinename : windows_core::PCWSTR, szdefaultobjectname : windows_core::PWSTR, pcchbuffersize : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhGetDefaultPerfObjectW(szdatasource.param().abi(), szmachinename.param().abi(), szdefaultobjectname.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PdhGetDllVersion(lpdwversion: Option<super::LPDWORD>) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetDllVersion(lpdwversion : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhGetDllVersion(lpdwversion.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhGetFormattedCounterArrayA(hcounter: PDH_HCOUNTER, dwformat: super::DWORD, lpdwbuffersize: super::LPDWORD, lpdwitemcount: super::LPDWORD, itembuffer: Option<PPDH_FMT_COUNTERVALUE_ITEM_A>) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetFormattedCounterArrayA(hcounter : PDH_HCOUNTER, dwformat : super::DWORD, lpdwbuffersize : super::LPDWORD, lpdwitemcount : super::LPDWORD, itembuffer : PPDH_FMT_COUNTERVALUE_ITEM_A) -> PDH_STATUS);
    unsafe { PdhGetFormattedCounterArrayA(hcounter, dwformat, lpdwbuffersize as _, lpdwitemcount as _, itembuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhGetFormattedCounterArrayW(hcounter: PDH_HCOUNTER, dwformat: super::DWORD, lpdwbuffersize: super::LPDWORD, lpdwitemcount: super::LPDWORD, itembuffer: Option<PPDH_FMT_COUNTERVALUE_ITEM_W>) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetFormattedCounterArrayW(hcounter : PDH_HCOUNTER, dwformat : super::DWORD, lpdwbuffersize : super::LPDWORD, lpdwitemcount : super::LPDWORD, itembuffer : PPDH_FMT_COUNTERVALUE_ITEM_W) -> PDH_STATUS);
    unsafe { PdhGetFormattedCounterArrayW(hcounter, dwformat, lpdwbuffersize as _, lpdwitemcount as _, itembuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhGetFormattedCounterValue(hcounter: PDH_HCOUNTER, dwformat: super::DWORD, lpdwtype: Option<super::LPDWORD>, pvalue: PPDH_FMT_COUNTERVALUE) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetFormattedCounterValue(hcounter : PDH_HCOUNTER, dwformat : super::DWORD, lpdwtype : super::LPDWORD, pvalue : PPDH_FMT_COUNTERVALUE) -> PDH_STATUS);
    unsafe { PdhGetFormattedCounterValue(hcounter, dwformat, lpdwtype.unwrap_or(core::mem::zeroed()) as _, pvalue as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PdhGetLogFileSize(hlog: PDH_HLOG, llsize: *mut super::LONGLONG) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetLogFileSize(hlog : PDH_HLOG, llsize : *mut super::LONGLONG) -> PDH_STATUS);
    unsafe { PdhGetLogFileSize(hlog, llsize as _) }
}
#[inline]
pub unsafe fn PdhGetLogSetGUID(hlog: PDH_HLOG, pguid: Option<*mut windows_core::GUID>, prunid: Option<*mut i32>) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetLogSetGUID(hlog : PDH_HLOG, pguid : *mut windows_core::GUID, prunid : *mut i32) -> PDH_STATUS);
    unsafe { PdhGetLogSetGUID(hlog, pguid.unwrap_or(core::mem::zeroed()) as _, prunid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhGetRawCounterArrayA(hcounter: PDH_HCOUNTER, lpdwbuffersize: super::LPDWORD, lpdwitemcount: super::LPDWORD, itembuffer: Option<PPDH_RAW_COUNTER_ITEM_A>) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetRawCounterArrayA(hcounter : PDH_HCOUNTER, lpdwbuffersize : super::LPDWORD, lpdwitemcount : super::LPDWORD, itembuffer : PPDH_RAW_COUNTER_ITEM_A) -> PDH_STATUS);
    unsafe { PdhGetRawCounterArrayA(hcounter, lpdwbuffersize as _, lpdwitemcount as _, itembuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhGetRawCounterArrayW(hcounter: PDH_HCOUNTER, lpdwbuffersize: super::LPDWORD, lpdwitemcount: super::LPDWORD, itembuffer: Option<PPDH_RAW_COUNTER_ITEM_W>) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetRawCounterArrayW(hcounter : PDH_HCOUNTER, lpdwbuffersize : super::LPDWORD, lpdwitemcount : super::LPDWORD, itembuffer : PPDH_RAW_COUNTER_ITEM_W) -> PDH_STATUS);
    unsafe { PdhGetRawCounterArrayW(hcounter, lpdwbuffersize as _, lpdwitemcount as _, itembuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhGetRawCounterValue(hcounter: PDH_HCOUNTER, lpdwtype: Option<super::LPDWORD>, pvalue: PPDH_RAW_COUNTER) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetRawCounterValue(hcounter : PDH_HCOUNTER, lpdwtype : super::LPDWORD, pvalue : PPDH_RAW_COUNTER) -> PDH_STATUS);
    unsafe { PdhGetRawCounterValue(hcounter, lpdwtype.unwrap_or(core::mem::zeroed()) as _, pvalue as _) }
}
#[inline]
pub unsafe fn PdhIsRealTimeQuery(hquery: PDH_HQUERY) -> windows_core::BOOL {
    windows_core::link!("pdh.dll" "system" fn PdhIsRealTimeQuery(hquery : PDH_HQUERY) -> windows_core::BOOL);
    unsafe { PdhIsRealTimeQuery(hquery) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PdhLookupPerfIndexByNameA<P0, P1>(szmachinename: P0, sznamebuffer: P1, pdwindex: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhLookupPerfIndexByNameA(szmachinename : windows_core::PCSTR, sznamebuffer : windows_core::PCSTR, pdwindex : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhLookupPerfIndexByNameA(szmachinename.param().abi(), sznamebuffer.param().abi(), pdwindex as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PdhLookupPerfIndexByNameW<P0, P1>(szmachinename: P0, sznamebuffer: P1, pdwindex: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhLookupPerfIndexByNameW(szmachinename : windows_core::PCWSTR, sznamebuffer : windows_core::PCWSTR, pdwindex : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhLookupPerfIndexByNameW(szmachinename.param().abi(), sznamebuffer.param().abi(), pdwindex as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhLookupPerfNameByIndexA<P0>(szmachinename: P0, dwnameindex: super::DWORD, sznamebuffer: Option<windows_core::PSTR>, pcchnamebuffersize: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhLookupPerfNameByIndexA(szmachinename : windows_core::PCSTR, dwnameindex : super::DWORD, sznamebuffer : windows_core::PSTR, pcchnamebuffersize : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhLookupPerfNameByIndexA(szmachinename.param().abi(), dwnameindex, sznamebuffer.unwrap_or(core::mem::zeroed()) as _, pcchnamebuffersize as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhLookupPerfNameByIndexW<P0>(szmachinename: P0, dwnameindex: super::DWORD, sznamebuffer: Option<windows_core::PWSTR>, pcchnamebuffersize: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhLookupPerfNameByIndexW(szmachinename : windows_core::PCWSTR, dwnameindex : super::DWORD, sznamebuffer : windows_core::PWSTR, pcchnamebuffersize : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhLookupPerfNameByIndexW(szmachinename.param().abi(), dwnameindex, sznamebuffer.unwrap_or(core::mem::zeroed()) as _, pcchnamebuffersize as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhMakeCounterPathA(pcounterpathelements: PPDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer: Option<windows_core::PSTR>, pcchbuffersize: super::LPDWORD, dwflags: super::DWORD) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhMakeCounterPathA(pcounterpathelements : PPDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer : windows_core::PSTR, pcchbuffersize : super::LPDWORD, dwflags : super::DWORD) -> PDH_STATUS);
    unsafe { PdhMakeCounterPathA(pcounterpathelements, szfullpathbuffer.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhMakeCounterPathW(pcounterpathelements: PPDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer: Option<windows_core::PWSTR>, pcchbuffersize: super::LPDWORD, dwflags: super::DWORD) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhMakeCounterPathW(pcounterpathelements : PPDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer : windows_core::PWSTR, pcchbuffersize : super::LPDWORD, dwflags : super::DWORD) -> PDH_STATUS);
    unsafe { PdhMakeCounterPathW(pcounterpathelements, szfullpathbuffer.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhOpenLogA<P0, P5>(szlogfilename: P0, dwaccessflags: super::DWORD, lpdwlogtype: super::LPDWORD, hquery: Option<PDH_HQUERY>, dwmaxsize: super::DWORD, szusercaption: P5, phlog: *mut PDH_HLOG) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhOpenLogA(szlogfilename : windows_core::PCSTR, dwaccessflags : super::DWORD, lpdwlogtype : super::LPDWORD, hquery : PDH_HQUERY, dwmaxsize : super::DWORD, szusercaption : windows_core::PCSTR, phlog : *mut PDH_HLOG) -> PDH_STATUS);
    unsafe { PdhOpenLogA(szlogfilename.param().abi(), dwaccessflags, lpdwlogtype as _, hquery.unwrap_or(core::mem::zeroed()) as _, dwmaxsize, szusercaption.param().abi(), phlog as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhOpenLogW<P0, P5>(szlogfilename: P0, dwaccessflags: super::DWORD, lpdwlogtype: super::LPDWORD, hquery: Option<PDH_HQUERY>, dwmaxsize: super::DWORD, szusercaption: P5, phlog: *mut PDH_HLOG) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhOpenLogW(szlogfilename : windows_core::PCWSTR, dwaccessflags : super::DWORD, lpdwlogtype : super::LPDWORD, hquery : PDH_HQUERY, dwmaxsize : super::DWORD, szusercaption : windows_core::PCWSTR, phlog : *mut PDH_HLOG) -> PDH_STATUS);
    unsafe { PdhOpenLogW(szlogfilename.param().abi(), dwaccessflags, lpdwlogtype as _, hquery.unwrap_or(core::mem::zeroed()) as _, dwmaxsize, szusercaption.param().abi(), phlog as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PdhOpenQueryA<P0>(szdatasource: P0, dwuserdata: super::DWORD_PTR, phquery: *mut PDH_HQUERY) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhOpenQueryA(szdatasource : windows_core::PCSTR, dwuserdata : super::DWORD_PTR, phquery : *mut PDH_HQUERY) -> PDH_STATUS);
    unsafe { PdhOpenQueryA(szdatasource.param().abi(), dwuserdata, phquery as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PdhOpenQueryH(hdatasource: Option<PDH_HLOG>, dwuserdata: super::DWORD_PTR, phquery: *mut PDH_HQUERY) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhOpenQueryH(hdatasource : PDH_HLOG, dwuserdata : super::DWORD_PTR, phquery : *mut PDH_HQUERY) -> PDH_STATUS);
    unsafe { PdhOpenQueryH(hdatasource.unwrap_or(core::mem::zeroed()) as _, dwuserdata, phquery as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PdhOpenQueryW<P0>(szdatasource: P0, dwuserdata: super::DWORD_PTR, phquery: *mut PDH_HQUERY) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhOpenQueryW(szdatasource : windows_core::PCWSTR, dwuserdata : super::DWORD_PTR, phquery : *mut PDH_HQUERY) -> PDH_STATUS);
    unsafe { PdhOpenQueryW(szdatasource.param().abi(), dwuserdata, phquery as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhParseCounterPathA<P0>(szfullpathbuffer: P0, pcounterpathelements: Option<PPDH_COUNTER_PATH_ELEMENTS_A>, pdwbuffersize: super::LPDWORD, dwflags: super::DWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhParseCounterPathA(szfullpathbuffer : windows_core::PCSTR, pcounterpathelements : PPDH_COUNTER_PATH_ELEMENTS_A, pdwbuffersize : super::LPDWORD, dwflags : super::DWORD) -> PDH_STATUS);
    unsafe { PdhParseCounterPathA(szfullpathbuffer.param().abi(), pcounterpathelements.unwrap_or(core::mem::zeroed()) as _, pdwbuffersize as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhParseCounterPathW<P0>(szfullpathbuffer: P0, pcounterpathelements: Option<PPDH_COUNTER_PATH_ELEMENTS_W>, pdwbuffersize: super::LPDWORD, dwflags: super::DWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhParseCounterPathW(szfullpathbuffer : windows_core::PCWSTR, pcounterpathelements : PPDH_COUNTER_PATH_ELEMENTS_W, pdwbuffersize : super::LPDWORD, dwflags : super::DWORD) -> PDH_STATUS);
    unsafe { PdhParseCounterPathW(szfullpathbuffer.param().abi(), pcounterpathelements.unwrap_or(core::mem::zeroed()) as _, pdwbuffersize as _, dwflags) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PdhParseInstanceNameA<P0>(szinstancestring: P0, szinstancename: Option<windows_core::PSTR>, pcchinstancenamelength: super::LPDWORD, szparentname: Option<windows_core::PSTR>, pcchparentnamelength: super::LPDWORD, lpindex: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhParseInstanceNameA(szinstancestring : windows_core::PCSTR, szinstancename : windows_core::PSTR, pcchinstancenamelength : super::LPDWORD, szparentname : windows_core::PSTR, pcchparentnamelength : super::LPDWORD, lpindex : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhParseInstanceNameA(szinstancestring.param().abi(), szinstancename.unwrap_or(core::mem::zeroed()) as _, pcchinstancenamelength as _, szparentname.unwrap_or(core::mem::zeroed()) as _, pcchparentnamelength as _, lpindex as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PdhParseInstanceNameW<P0>(szinstancestring: P0, szinstancename: Option<windows_core::PWSTR>, pcchinstancenamelength: super::LPDWORD, szparentname: Option<windows_core::PWSTR>, pcchparentnamelength: super::LPDWORD, lpindex: super::LPDWORD) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhParseInstanceNameW(szinstancestring : windows_core::PCWSTR, szinstancename : windows_core::PWSTR, pcchinstancenamelength : super::LPDWORD, szparentname : windows_core::PWSTR, pcchparentnamelength : super::LPDWORD, lpindex : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhParseInstanceNameW(szinstancestring.param().abi(), szinstancename.unwrap_or(core::mem::zeroed()) as _, pcchinstancenamelength as _, szparentname.unwrap_or(core::mem::zeroed()) as _, pcchparentnamelength as _, lpindex as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PdhReadRawLogRecord(hlog: PDH_HLOG, ftrecord: super::FILETIME, prawlogrecord: Option<PPDH_RAW_LOG_RECORD>, pdwbufferlength: super::LPDWORD) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhReadRawLogRecord(hlog : PDH_HLOG, ftrecord : super::FILETIME, prawlogrecord : PPDH_RAW_LOG_RECORD, pdwbufferlength : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhReadRawLogRecord(hlog, ftrecord, prawlogrecord.unwrap_or(core::mem::zeroed()) as _, pdwbufferlength as _) }
}
#[inline]
pub unsafe fn PdhRemoveCounter(hcounter: PDH_HCOUNTER) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhRemoveCounter(hcounter : PDH_HCOUNTER) -> PDH_STATUS);
    unsafe { PdhRemoveCounter(hcounter) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn PdhSelectDataSourceA(hwndowner: super::HWND, dwflags: super::DWORD, szdatasource: windows_core::PSTR, pcchbufferlength: super::LPDWORD) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhSelectDataSourceA(hwndowner : super::HWND, dwflags : super::DWORD, szdatasource : windows_core::PSTR, pcchbufferlength : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhSelectDataSourceA(hwndowner, dwflags, szdatasource, pcchbufferlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn PdhSelectDataSourceW(hwndowner: super::HWND, dwflags: super::DWORD, szdatasource: windows_core::PWSTR, pcchbufferlength: super::LPDWORD) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhSelectDataSourceW(hwndowner : super::HWND, dwflags : super::DWORD, szdatasource : windows_core::PWSTR, pcchbufferlength : super::LPDWORD) -> PDH_STATUS);
    unsafe { PdhSelectDataSourceW(hwndowner, dwflags, szdatasource, pcchbufferlength as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PdhSetCounterScaleFactor(hcounter: PDH_HCOUNTER, lfactor: super::LONG) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhSetCounterScaleFactor(hcounter : PDH_HCOUNTER, lfactor : super::LONG) -> PDH_STATUS);
    unsafe { PdhSetCounterScaleFactor(hcounter as _, lfactor) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PdhSetDefaultRealTimeDataSource(dwdatasourceid: super::DWORD) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhSetDefaultRealTimeDataSource(dwdatasourceid : super::DWORD) -> PDH_STATUS);
    unsafe { PdhSetDefaultRealTimeDataSource(dwdatasourceid) }
}
#[inline]
pub unsafe fn PdhSetLogSetRunID(hlog: PDH_HLOG, runid: i32) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhSetLogSetRunID(hlog : PDH_HLOG, runid : i32) -> PDH_STATUS);
    unsafe { PdhSetLogSetRunID(hlog as _, runid) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PdhSetQueryTimeRange(hquery: PDH_HQUERY, pinfo: PPDH_TIME_INFO) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhSetQueryTimeRange(hquery : PDH_HQUERY, pinfo : PPDH_TIME_INFO) -> PDH_STATUS);
    unsafe { PdhSetQueryTimeRange(hquery, pinfo) }
}
#[inline]
pub unsafe fn PdhUpdateLogA<P1>(hlog: PDH_HLOG, szuserstring: P1) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhUpdateLogA(hlog : PDH_HLOG, szuserstring : windows_core::PCSTR) -> PDH_STATUS);
    unsafe { PdhUpdateLogA(hlog, szuserstring.param().abi()) }
}
#[inline]
pub unsafe fn PdhUpdateLogFileCatalog(hlog: PDH_HLOG) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhUpdateLogFileCatalog(hlog : PDH_HLOG) -> PDH_STATUS);
    unsafe { PdhUpdateLogFileCatalog(hlog) }
}
#[inline]
pub unsafe fn PdhUpdateLogW<P1>(hlog: PDH_HLOG, szuserstring: P1) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhUpdateLogW(hlog : PDH_HLOG, szuserstring : windows_core::PCWSTR) -> PDH_STATUS);
    unsafe { PdhUpdateLogW(hlog, szuserstring.param().abi()) }
}
#[inline]
pub unsafe fn PdhValidatePathA<P0>(szfullpathbuffer: P0) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhValidatePathA(szfullpathbuffer : windows_core::PCSTR) -> PDH_STATUS);
    unsafe { PdhValidatePathA(szfullpathbuffer.param().abi()) }
}
#[inline]
pub unsafe fn PdhValidatePathExA<P1>(hdatasource: Option<PDH_HLOG>, szfullpathbuffer: P1) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhValidatePathExA(hdatasource : PDH_HLOG, szfullpathbuffer : windows_core::PCSTR) -> PDH_STATUS);
    unsafe { PdhValidatePathExA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szfullpathbuffer.param().abi()) }
}
#[inline]
pub unsafe fn PdhValidatePathExW<P1>(hdatasource: Option<PDH_HLOG>, szfullpathbuffer: P1) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhValidatePathExW(hdatasource : PDH_HLOG, szfullpathbuffer : windows_core::PCWSTR) -> PDH_STATUS);
    unsafe { PdhValidatePathExW(hdatasource.unwrap_or(core::mem::zeroed()) as _, szfullpathbuffer.param().abi()) }
}
#[inline]
pub unsafe fn PdhValidatePathW<P0>(szfullpathbuffer: P0) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhValidatePathW(szfullpathbuffer : windows_core::PCWSTR) -> PDH_STATUS);
    unsafe { PdhValidatePathW(szfullpathbuffer.param().abi()) }
}
#[inline]
pub unsafe fn PdhVerifySQLDBA<P0>(szdatasource: P0) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhVerifySQLDBA(szdatasource : windows_core::PCSTR) -> PDH_STATUS);
    unsafe { PdhVerifySQLDBA(szdatasource.param().abi()) }
}
#[inline]
pub unsafe fn PdhVerifySQLDBW<P0>(szdatasource: P0) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhVerifySQLDBW(szdatasource : windows_core::PCWSTR) -> PDH_STATUS);
    unsafe { PdhVerifySQLDBW(szdatasource.param().abi()) }
}
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
#[derive(Clone, Copy, Debug, Default)]
pub struct PDH_BROWSE_DLG_CONFIG_A {
    pub _bitfield: u32,
    pub hWndOwner: super::HWND,
    pub szDataSource: windows_core::PSTR,
    pub szReturnPathBuffer: windows_core::PSTR,
    pub cchReturnPathLength: super::DWORD,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: super::DWORD_PTR,
    pub CallBackStatus: PDH_STATUS,
    pub dwDefaultDetailLevel: super::DWORD,
    pub szDialogBoxCaption: windows_core::PSTR,
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl PDH_BROWSE_DLG_CONFIG_A {
    pub fn bIncludeInstanceIndex(&self) -> super::DWORD {
        (self._bitfield) & 1
    }
    pub fn set_bIncludeInstanceIndex(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn bSingleCounterPerAdd(&self) -> super::DWORD {
        (self._bitfield >> 1) & 1
    }
    pub fn set_bSingleCounterPerAdd(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn bSingleCounterPerDialog(&self) -> super::DWORD {
        (self._bitfield >> 2) & 1
    }
    pub fn set_bSingleCounterPerDialog(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn bLocalCountersOnly(&self) -> super::DWORD {
        (self._bitfield >> 3) & 1
    }
    pub fn set_bLocalCountersOnly(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn bWildCardInstances(&self) -> super::DWORD {
        (self._bitfield >> 4) & 1
    }
    pub fn set_bWildCardInstances(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn bHideDetailBox(&self) -> super::DWORD {
        (self._bitfield >> 5) & 1
    }
    pub fn set_bHideDetailBox(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn bInitializePath(&self) -> super::DWORD {
        (self._bitfield >> 6) & 1
    }
    pub fn set_bInitializePath(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn bDisableMachineSelection(&self) -> super::DWORD {
        (self._bitfield >> 7) & 1
    }
    pub fn set_bDisableMachineSelection(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
    pub fn bIncludeCostlyObjects(&self) -> super::DWORD {
        (self._bitfield >> 8) & 1
    }
    pub fn set_bIncludeCostlyObjects(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn bShowObjectBrowser(&self) -> super::DWORD {
        (self._bitfield >> 9) & 1
    }
    pub fn set_bShowObjectBrowser(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value & 1) << 9);
    }
    pub fn bReserved(&self) -> super::DWORD {
        self._bitfield >> 10
    }
    pub fn set_bReserved(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(4194303 << 10)) | ((value & 4194303) << 10);
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct PDH_BROWSE_DLG_CONFIG_HA {
    pub _bitfield: u32,
    pub hWndOwner: super::HWND,
    pub hDataSource: PDH_HLOG,
    pub szReturnPathBuffer: windows_core::PSTR,
    pub cchReturnPathLength: super::DWORD,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: super::DWORD_PTR,
    pub CallBackStatus: PDH_STATUS,
    pub dwDefaultDetailLevel: super::DWORD,
    pub szDialogBoxCaption: windows_core::PSTR,
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl PDH_BROWSE_DLG_CONFIG_HA {
    pub fn bIncludeInstanceIndex(&self) -> super::DWORD {
        (self._bitfield) & 1
    }
    pub fn set_bIncludeInstanceIndex(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn bSingleCounterPerAdd(&self) -> super::DWORD {
        (self._bitfield >> 1) & 1
    }
    pub fn set_bSingleCounterPerAdd(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn bSingleCounterPerDialog(&self) -> super::DWORD {
        (self._bitfield >> 2) & 1
    }
    pub fn set_bSingleCounterPerDialog(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn bLocalCountersOnly(&self) -> super::DWORD {
        (self._bitfield >> 3) & 1
    }
    pub fn set_bLocalCountersOnly(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn bWildCardInstances(&self) -> super::DWORD {
        (self._bitfield >> 4) & 1
    }
    pub fn set_bWildCardInstances(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn bHideDetailBox(&self) -> super::DWORD {
        (self._bitfield >> 5) & 1
    }
    pub fn set_bHideDetailBox(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn bInitializePath(&self) -> super::DWORD {
        (self._bitfield >> 6) & 1
    }
    pub fn set_bInitializePath(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn bDisableMachineSelection(&self) -> super::DWORD {
        (self._bitfield >> 7) & 1
    }
    pub fn set_bDisableMachineSelection(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
    pub fn bIncludeCostlyObjects(&self) -> super::DWORD {
        (self._bitfield >> 8) & 1
    }
    pub fn set_bIncludeCostlyObjects(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn bShowObjectBrowser(&self) -> super::DWORD {
        (self._bitfield >> 9) & 1
    }
    pub fn set_bShowObjectBrowser(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value & 1) << 9);
    }
    pub fn bReserved(&self) -> super::DWORD {
        self._bitfield >> 10
    }
    pub fn set_bReserved(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(4194303 << 10)) | ((value & 4194303) << 10);
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct PDH_BROWSE_DLG_CONFIG_HW {
    pub _bitfield: u32,
    pub hWndOwner: super::HWND,
    pub hDataSource: PDH_HLOG,
    pub szReturnPathBuffer: windows_core::PWSTR,
    pub cchReturnPathLength: super::DWORD,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: super::DWORD_PTR,
    pub CallBackStatus: PDH_STATUS,
    pub dwDefaultDetailLevel: super::DWORD,
    pub szDialogBoxCaption: windows_core::PWSTR,
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl PDH_BROWSE_DLG_CONFIG_HW {
    pub fn bIncludeInstanceIndex(&self) -> super::DWORD {
        (self._bitfield) & 1
    }
    pub fn set_bIncludeInstanceIndex(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn bSingleCounterPerAdd(&self) -> super::DWORD {
        (self._bitfield >> 1) & 1
    }
    pub fn set_bSingleCounterPerAdd(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn bSingleCounterPerDialog(&self) -> super::DWORD {
        (self._bitfield >> 2) & 1
    }
    pub fn set_bSingleCounterPerDialog(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn bLocalCountersOnly(&self) -> super::DWORD {
        (self._bitfield >> 3) & 1
    }
    pub fn set_bLocalCountersOnly(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn bWildCardInstances(&self) -> super::DWORD {
        (self._bitfield >> 4) & 1
    }
    pub fn set_bWildCardInstances(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn bHideDetailBox(&self) -> super::DWORD {
        (self._bitfield >> 5) & 1
    }
    pub fn set_bHideDetailBox(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn bInitializePath(&self) -> super::DWORD {
        (self._bitfield >> 6) & 1
    }
    pub fn set_bInitializePath(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn bDisableMachineSelection(&self) -> super::DWORD {
        (self._bitfield >> 7) & 1
    }
    pub fn set_bDisableMachineSelection(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
    pub fn bIncludeCostlyObjects(&self) -> super::DWORD {
        (self._bitfield >> 8) & 1
    }
    pub fn set_bIncludeCostlyObjects(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn bShowObjectBrowser(&self) -> super::DWORD {
        (self._bitfield >> 9) & 1
    }
    pub fn set_bShowObjectBrowser(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value & 1) << 9);
    }
    pub fn bReserved(&self) -> super::DWORD {
        self._bitfield >> 10
    }
    pub fn set_bReserved(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(4194303 << 10)) | ((value & 4194303) << 10);
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct PDH_BROWSE_DLG_CONFIG_W {
    pub _bitfield: u32,
    pub hWndOwner: super::HWND,
    pub szDataSource: windows_core::PWSTR,
    pub szReturnPathBuffer: windows_core::PWSTR,
    pub cchReturnPathLength: super::DWORD,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: super::DWORD_PTR,
    pub CallBackStatus: PDH_STATUS,
    pub dwDefaultDetailLevel: super::DWORD,
    pub szDialogBoxCaption: windows_core::PWSTR,
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl PDH_BROWSE_DLG_CONFIG_W {
    pub fn bIncludeInstanceIndex(&self) -> super::DWORD {
        (self._bitfield) & 1
    }
    pub fn set_bIncludeInstanceIndex(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn bSingleCounterPerAdd(&self) -> super::DWORD {
        (self._bitfield >> 1) & 1
    }
    pub fn set_bSingleCounterPerAdd(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn bSingleCounterPerDialog(&self) -> super::DWORD {
        (self._bitfield >> 2) & 1
    }
    pub fn set_bSingleCounterPerDialog(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn bLocalCountersOnly(&self) -> super::DWORD {
        (self._bitfield >> 3) & 1
    }
    pub fn set_bLocalCountersOnly(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn bWildCardInstances(&self) -> super::DWORD {
        (self._bitfield >> 4) & 1
    }
    pub fn set_bWildCardInstances(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn bHideDetailBox(&self) -> super::DWORD {
        (self._bitfield >> 5) & 1
    }
    pub fn set_bHideDetailBox(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn bInitializePath(&self) -> super::DWORD {
        (self._bitfield >> 6) & 1
    }
    pub fn set_bInitializePath(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn bDisableMachineSelection(&self) -> super::DWORD {
        (self._bitfield >> 7) & 1
    }
    pub fn set_bDisableMachineSelection(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
    pub fn bIncludeCostlyObjects(&self) -> super::DWORD {
        (self._bitfield >> 8) & 1
    }
    pub fn set_bIncludeCostlyObjects(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn bShowObjectBrowser(&self) -> super::DWORD {
        (self._bitfield >> 9) & 1
    }
    pub fn set_bShowObjectBrowser(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value & 1) << 9);
    }
    pub fn bReserved(&self) -> super::DWORD {
        self._bitfield >> 10
    }
    pub fn set_bReserved(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(4194303 << 10)) | ((value & 4194303) << 10);
    }
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
    pub szFullPath: windows_core::PSTR,
    pub Anonymous: PDH_COUNTER_INFO_A_0,
    pub szExplainText: windows_core::PSTR,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_COUNTER_INFO_A_0_0 {
    pub szMachineName: windows_core::PSTR,
    pub szObjectName: windows_core::PSTR,
    pub szInstanceName: windows_core::PSTR,
    pub szParentInstance: windows_core::PSTR,
    pub dwInstanceIndex: super::DWORD,
    pub szCounterName: windows_core::PSTR,
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
    pub szFullPath: windows_core::PWSTR,
    pub Anonymous: PDH_COUNTER_INFO_W_0,
    pub szExplainText: windows_core::PWSTR,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_COUNTER_INFO_W_0_0 {
    pub szMachineName: windows_core::PWSTR,
    pub szObjectName: windows_core::PWSTR,
    pub szInstanceName: windows_core::PWSTR,
    pub szParentInstance: windows_core::PWSTR,
    pub dwInstanceIndex: super::DWORD,
    pub szCounterName: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_COUNTER_PATH_ELEMENTS_A {
    pub szMachineName: windows_core::PSTR,
    pub szObjectName: windows_core::PSTR,
    pub szInstanceName: windows_core::PSTR,
    pub szParentInstance: windows_core::PSTR,
    pub dwInstanceIndex: super::DWORD,
    pub szCounterName: windows_core::PSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_COUNTER_PATH_ELEMENTS_W {
    pub szMachineName: windows_core::PWSTR,
    pub szObjectName: windows_core::PWSTR,
    pub szInstanceName: windows_core::PWSTR,
    pub szParentInstance: windows_core::PWSTR,
    pub dwInstanceIndex: super::DWORD,
    pub szCounterName: windows_core::PWSTR,
}
#[cfg(feature = "shared")]
pub const PDH_CVERSION_WIN40: super::DWORD = 1024;
#[cfg(feature = "shared")]
pub const PDH_CVERSION_WIN50: super::DWORD = 1280;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_A {
    pub szMachineName: windows_core::PSTR,
    pub ObjectGUID: windows_core::GUID,
    pub dwItemId: super::DWORD,
    pub szInstanceName: windows_core::PSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_W {
    pub szMachineName: windows_core::PWSTR,
    pub ObjectGUID: windows_core::GUID,
    pub dwItemId: super::DWORD,
    pub szInstanceName: windows_core::PWSTR,
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
    pub AnsiStringValue: windows_core::PCSTR,
    pub WideStringValue: windows_core::PCWSTR,
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
    pub szName: windows_core::PSTR,
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
    pub szName: windows_core::PWSTR,
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
    pub szLogFileCaption: windows_core::PSTR,
    pub szDefaultDir: windows_core::PSTR,
    pub szBaseFileName: windows_core::PSTR,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    pub PdlAutoNameInterval: super::DWORD,
    pub PdlAutoNameUnits: super::DWORD,
    pub PdlCommandFilename: windows_core::PSTR,
    pub PdlCounterList: windows_core::PSTR,
    pub PdlAutoNameFormat: super::DWORD,
    pub PdlSampleInterval: super::DWORD,
    pub PdlLogStartTime: super::FILETIME,
    pub PdlLogEndTime: super::FILETIME,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub TlLogFileName: windows_core::PSTR,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwLogQuota: super::DWORD,
    pub szLogFileCaption: windows_core::PWSTR,
    pub szDefaultDir: windows_core::PWSTR,
    pub szBaseFileName: windows_core::PWSTR,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    pub PdlAutoNameInterval: super::DWORD,
    pub PdlAutoNameUnits: super::DWORD,
    pub PdlCommandFilename: windows_core::PWSTR,
    pub PdlCounterList: windows_core::PWSTR,
    pub PdlAutoNameFormat: super::DWORD,
    pub PdlSampleInterval: super::DWORD,
    pub PdlLogStartTime: super::FILETIME,
    pub PdlLogEndTime: super::FILETIME,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub TlLogFileName: windows_core::PWSTR,
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
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_RAW_COUNTER {
    pub CStatus: super::DWORD,
    pub TimeStamp: super::FILETIME,
    pub FirstValue: super::LONGLONG,
    pub SecondValue: super::LONGLONG,
    pub MultiCount: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_RAW_COUNTER_ITEM_A {
    pub szName: windows_core::PSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_RAW_COUNTER_ITEM_W {
    pub szName: windows_core::PWSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PPDH_RAW_COUNTER = *mut PDH_RAW_COUNTER;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PPDH_RAW_COUNTER_ITEM_A = *mut PDH_RAW_COUNTER_ITEM_A;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PPDH_RAW_COUNTER_ITEM_W = *mut PDH_RAW_COUNTER_ITEM_W;
#[cfg(feature = "shared")]
pub type PPDH_RAW_LOG_RECORD = *mut PDH_RAW_LOG_RECORD;
#[cfg(feature = "shared")]
pub type PPDH_STATISTICS = *mut PDH_STATISTICS;
#[cfg(feature = "shared")]
pub type PPDH_TIME_INFO = *mut PDH_TIME_INFO;
