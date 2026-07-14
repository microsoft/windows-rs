#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhAddCounterA<P1>(hquery: PDH_HQUERY, szfullcounterpath: P1, dwuserdata: usize, phcounter: *mut PDH_HCOUNTER) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhAddCounterA(hquery : PDH_HQUERY, szfullcounterpath : windows_core::PCSTR, dwuserdata : usize, phcounter : *mut PDH_HCOUNTER) -> PDH_STATUS);
    unsafe { PdhAddCounterA(hquery, szfullcounterpath.param().abi(), dwuserdata, phcounter as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhAddCounterW<P1>(hquery: PDH_HQUERY, szfullcounterpath: P1, dwuserdata: usize, phcounter: *mut PDH_HCOUNTER) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhAddCounterW(hquery : PDH_HQUERY, szfullcounterpath : windows_core::PCWSTR, dwuserdata : usize, phcounter : *mut PDH_HCOUNTER) -> PDH_STATUS);
    unsafe { PdhAddCounterW(hquery, szfullcounterpath.param().abi(), dwuserdata, phcounter as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhAddEnglishCounterA<P1>(hquery: PDH_HQUERY, szfullcounterpath: P1, dwuserdata: usize, phcounter: *mut PDH_HCOUNTER) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhAddEnglishCounterA(hquery : PDH_HQUERY, szfullcounterpath : windows_core::PCSTR, dwuserdata : usize, phcounter : *mut PDH_HCOUNTER) -> PDH_STATUS);
    unsafe { PdhAddEnglishCounterA(hquery, szfullcounterpath.param().abi(), dwuserdata, phcounter as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhAddEnglishCounterW<P1>(hquery: PDH_HQUERY, szfullcounterpath: P1, dwuserdata: usize, phcounter: *mut PDH_HCOUNTER) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhAddEnglishCounterW(hquery : PDH_HQUERY, szfullcounterpath : windows_core::PCWSTR, dwuserdata : usize, phcounter : *mut PDH_HCOUNTER) -> PDH_STATUS);
    unsafe { PdhAddEnglishCounterW(hquery, szfullcounterpath.param().abi(), dwuserdata, phcounter as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhBindInputDataSourceA<P1>(phdatasource: *mut PDH_HLOG, logfilenamelist: P1) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhBindInputDataSourceA(phdatasource : *mut PDH_HLOG, logfilenamelist : windows_core::PCSTR) -> PDH_STATUS);
    unsafe { PdhBindInputDataSourceA(phdatasource as _, logfilenamelist.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhBindInputDataSourceW<P1>(phdatasource: *mut PDH_HLOG, logfilenamelist: P1) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhBindInputDataSourceW(phdatasource : *mut PDH_HLOG, logfilenamelist : windows_core::PCWSTR) -> PDH_STATUS);
    unsafe { PdhBindInputDataSourceW(phdatasource as _, logfilenamelist.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn PdhBrowseCountersA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_A) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhBrowseCountersA(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_A) -> PDH_STATUS);
    unsafe { PdhBrowseCountersA(pbrowsedlgdata) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn PdhBrowseCountersHA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HA) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhBrowseCountersHA(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_HA) -> PDH_STATUS);
    unsafe { PdhBrowseCountersHA(pbrowsedlgdata) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn PdhBrowseCountersHW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HW) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhBrowseCountersHW(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_HW) -> PDH_STATUS);
    unsafe { PdhBrowseCountersHW(pbrowsedlgdata) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn PdhBrowseCountersW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_W) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhBrowseCountersW(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_W) -> PDH_STATUS);
    unsafe { PdhBrowseCountersW(pbrowsedlgdata) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn PdhCalculateCounterFromRawValue(hcounter: PDH_HCOUNTER, dwformat: u32, rawvalue1: *const PDH_RAW_COUNTER, rawvalue2: *const PDH_RAW_COUNTER, fmtvalue: *mut PDH_FMT_COUNTERVALUE) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhCalculateCounterFromRawValue(hcounter : PDH_HCOUNTER, dwformat : u32, rawvalue1 : *const PDH_RAW_COUNTER, rawvalue2 : *const PDH_RAW_COUNTER, fmtvalue : *mut PDH_FMT_COUNTERVALUE) -> PDH_STATUS);
    unsafe { PdhCalculateCounterFromRawValue(hcounter, dwformat, rawvalue1, rawvalue2, fmtvalue as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhCloseLog(hlog: PDH_HLOG, dwflags: u32) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhCloseLog(hlog : PDH_HLOG, dwflags : u32) -> PDH_STATUS);
    unsafe { PdhCloseLog(hlog, dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhCloseQuery(hquery: PDH_HQUERY) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhCloseQuery(hquery : PDH_HQUERY) -> PDH_STATUS);
    unsafe { PdhCloseQuery(hquery as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhCollectQueryData(hquery: PDH_HQUERY) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhCollectQueryData(hquery : PDH_HQUERY) -> PDH_STATUS);
    unsafe { PdhCollectQueryData(hquery as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhCollectQueryDataEx(hquery: PDH_HQUERY, dwintervaltime: u32, hnewdataevent: super::winnt::HANDLE) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhCollectQueryDataEx(hquery : PDH_HQUERY, dwintervaltime : u32, hnewdataevent : super::winnt::HANDLE) -> PDH_STATUS);
    unsafe { PdhCollectQueryDataEx(hquery, dwintervaltime, hnewdataevent) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhCollectQueryDataWithTime(hquery: PDH_HQUERY, plltimestamp: *mut i64) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhCollectQueryDataWithTime(hquery : PDH_HQUERY, plltimestamp : *mut i64) -> PDH_STATUS);
    unsafe { PdhCollectQueryDataWithTime(hquery as _, plltimestamp as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn PdhComputeCounterStatistics(hcounter: PDH_HCOUNTER, dwformat: u32, dwfirstentry: u32, dwnumentries: u32, lprawvaluearray: *const PDH_RAW_COUNTER, data: *mut PDH_STATISTICS) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhComputeCounterStatistics(hcounter : PDH_HCOUNTER, dwformat : u32, dwfirstentry : u32, dwnumentries : u32, lprawvaluearray : *const PDH_RAW_COUNTER, data : *mut PDH_STATISTICS) -> PDH_STATUS);
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
#[inline]
pub unsafe fn PdhEnumLogSetNamesA<P0>(szdatasource: P0, mszdatasetnamelist: Option<*mut i8>, pcchbufferlength: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumLogSetNamesA(szdatasource : windows_core::PCSTR, mszdatasetnamelist : *mut i8, pcchbufferlength : *mut u32) -> PDH_STATUS);
    unsafe { PdhEnumLogSetNamesA(szdatasource.param().abi(), mszdatasetnamelist.unwrap_or(core::mem::zeroed()) as _, pcchbufferlength as _) }
}
#[inline]
pub unsafe fn PdhEnumLogSetNamesW<P0>(szdatasource: P0, mszdatasetnamelist: Option<*mut u16>, pcchbufferlength: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumLogSetNamesW(szdatasource : windows_core::PCWSTR, mszdatasetnamelist : *mut u16, pcchbufferlength : *mut u32) -> PDH_STATUS);
    unsafe { PdhEnumLogSetNamesW(szdatasource.param().abi(), mszdatasetnamelist.unwrap_or(core::mem::zeroed()) as _, pcchbufferlength as _) }
}
#[inline]
pub unsafe fn PdhEnumMachinesA<P0>(szdatasource: P0, mszmachinelist: Option<*mut i8>, pcchbuffersize: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumMachinesA(szdatasource : windows_core::PCSTR, mszmachinelist : *mut i8, pcchbuffersize : *mut u32) -> PDH_STATUS);
    unsafe { PdhEnumMachinesA(szdatasource.param().abi(), mszmachinelist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhEnumMachinesHA(hdatasource: Option<PDH_HLOG>, mszmachinelist: Option<*mut i8>, pcchbuffersize: *mut u32) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhEnumMachinesHA(hdatasource : PDH_HLOG, mszmachinelist : *mut i8, pcchbuffersize : *mut u32) -> PDH_STATUS);
    unsafe { PdhEnumMachinesHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, mszmachinelist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhEnumMachinesHW(hdatasource: Option<PDH_HLOG>, mszmachinelist: Option<*mut u16>, pcchbuffersize: *mut u32) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhEnumMachinesHW(hdatasource : PDH_HLOG, mszmachinelist : *mut u16, pcchbuffersize : *mut u32) -> PDH_STATUS);
    unsafe { PdhEnumMachinesHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, mszmachinelist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhEnumMachinesW<P0>(szdatasource: P0, mszmachinelist: Option<*mut u16>, pcchbuffersize: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumMachinesW(szdatasource : windows_core::PCWSTR, mszmachinelist : *mut u16, pcchbuffersize : *mut u32) -> PDH_STATUS);
    unsafe { PdhEnumMachinesW(szdatasource.param().abi(), mszmachinelist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhEnumObjectItemsA<P0, P1, P2>(szdatasource: P0, szmachinename: P1, szobjectname: P2, mszcounterlist: Option<*mut i8>, pcchcounterlistlength: *mut u32, mszinstancelist: Option<*mut i8>, pcchinstancelistlength: *mut u32, dwdetaillevel: u32, dwflags: u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectItemsA(szdatasource : windows_core::PCSTR, szmachinename : windows_core::PCSTR, szobjectname : windows_core::PCSTR, mszcounterlist : *mut i8, pcchcounterlistlength : *mut u32, mszinstancelist : *mut i8, pcchinstancelistlength : *mut u32, dwdetaillevel : u32, dwflags : u32) -> PDH_STATUS);
    unsafe { PdhEnumObjectItemsA(szdatasource.param().abi(), szmachinename.param().abi(), szobjectname.param().abi(), mszcounterlist.unwrap_or(core::mem::zeroed()) as _, pcchcounterlistlength as _, mszinstancelist.unwrap_or(core::mem::zeroed()) as _, pcchinstancelistlength as _, dwdetaillevel, dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhEnumObjectItemsHA<P1, P2>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szobjectname: P2, mszcounterlist: Option<*mut i8>, pcchcounterlistlength: *mut u32, mszinstancelist: Option<*mut i8>, pcchinstancelistlength: *mut u32, dwdetaillevel: u32, dwflags: u32) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectItemsHA(hdatasource : PDH_HLOG, szmachinename : windows_core::PCSTR, szobjectname : windows_core::PCSTR, mszcounterlist : *mut i8, pcchcounterlistlength : *mut u32, mszinstancelist : *mut i8, pcchinstancelistlength : *mut u32, dwdetaillevel : u32, dwflags : u32) -> PDH_STATUS);
    unsafe { PdhEnumObjectItemsHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szobjectname.param().abi(), mszcounterlist.unwrap_or(core::mem::zeroed()) as _, pcchcounterlistlength as _, mszinstancelist.unwrap_or(core::mem::zeroed()) as _, pcchinstancelistlength as _, dwdetaillevel, dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhEnumObjectItemsHW<P1, P2>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szobjectname: P2, mszcounterlist: Option<*mut u16>, pcchcounterlistlength: *mut u32, mszinstancelist: Option<*mut u16>, pcchinstancelistlength: *mut u32, dwdetaillevel: u32, dwflags: u32) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectItemsHW(hdatasource : PDH_HLOG, szmachinename : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, mszcounterlist : *mut u16, pcchcounterlistlength : *mut u32, mszinstancelist : *mut u16, pcchinstancelistlength : *mut u32, dwdetaillevel : u32, dwflags : u32) -> PDH_STATUS);
    unsafe { PdhEnumObjectItemsHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szobjectname.param().abi(), mszcounterlist.unwrap_or(core::mem::zeroed()) as _, pcchcounterlistlength as _, mszinstancelist.unwrap_or(core::mem::zeroed()) as _, pcchinstancelistlength as _, dwdetaillevel, dwflags) }
}
#[inline]
pub unsafe fn PdhEnumObjectItemsW<P0, P1, P2>(szdatasource: P0, szmachinename: P1, szobjectname: P2, mszcounterlist: Option<*mut u16>, pcchcounterlistlength: *mut u32, mszinstancelist: Option<*mut u16>, pcchinstancelistlength: *mut u32, dwdetaillevel: u32, dwflags: u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectItemsW(szdatasource : windows_core::PCWSTR, szmachinename : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, mszcounterlist : *mut u16, pcchcounterlistlength : *mut u32, mszinstancelist : *mut u16, pcchinstancelistlength : *mut u32, dwdetaillevel : u32, dwflags : u32) -> PDH_STATUS);
    unsafe { PdhEnumObjectItemsW(szdatasource.param().abi(), szmachinename.param().abi(), szobjectname.param().abi(), mszcounterlist.unwrap_or(core::mem::zeroed()) as _, pcchcounterlistlength as _, mszinstancelist.unwrap_or(core::mem::zeroed()) as _, pcchinstancelistlength as _, dwdetaillevel, dwflags) }
}
#[inline]
pub unsafe fn PdhEnumObjectsA<P0, P1>(szdatasource: P0, szmachinename: P1, mszobjectlist: Option<*mut i8>, pcchbuffersize: *mut u32, dwdetaillevel: u32, brefresh: bool) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectsA(szdatasource : windows_core::PCSTR, szmachinename : windows_core::PCSTR, mszobjectlist : *mut i8, pcchbuffersize : *mut u32, dwdetaillevel : u32, brefresh : windows_core::BOOL) -> PDH_STATUS);
    unsafe { PdhEnumObjectsA(szdatasource.param().abi(), szmachinename.param().abi(), mszobjectlist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwdetaillevel, brefresh.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhEnumObjectsHA<P1>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, mszobjectlist: Option<*mut i8>, pcchbuffersize: *mut u32, dwdetaillevel: u32, brefresh: bool) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectsHA(hdatasource : PDH_HLOG, szmachinename : windows_core::PCSTR, mszobjectlist : *mut i8, pcchbuffersize : *mut u32, dwdetaillevel : u32, brefresh : windows_core::BOOL) -> PDH_STATUS);
    unsafe { PdhEnumObjectsHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), mszobjectlist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwdetaillevel, brefresh.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhEnumObjectsHW<P1>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, mszobjectlist: Option<*mut u16>, pcchbuffersize: *mut u32, dwdetaillevel: u32, brefresh: bool) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectsHW(hdatasource : PDH_HLOG, szmachinename : windows_core::PCWSTR, mszobjectlist : *mut u16, pcchbuffersize : *mut u32, dwdetaillevel : u32, brefresh : windows_core::BOOL) -> PDH_STATUS);
    unsafe { PdhEnumObjectsHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), mszobjectlist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwdetaillevel, brefresh.into()) }
}
#[inline]
pub unsafe fn PdhEnumObjectsW<P0, P1>(szdatasource: P0, szmachinename: P1, mszobjectlist: Option<*mut u16>, pcchbuffersize: *mut u32, dwdetaillevel: u32, brefresh: bool) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectsW(szdatasource : windows_core::PCWSTR, szmachinename : windows_core::PCWSTR, mszobjectlist : *mut u16, pcchbuffersize : *mut u32, dwdetaillevel : u32, brefresh : windows_core::BOOL) -> PDH_STATUS);
    unsafe { PdhEnumObjectsW(szdatasource.param().abi(), szmachinename.param().abi(), mszobjectlist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwdetaillevel, brefresh.into()) }
}
#[inline]
pub unsafe fn PdhExpandCounterPathA<P0>(szwildcardpath: P0, mszexpandedpathlist: Option<*mut i8>, pcchpathlistlength: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandCounterPathA(szwildcardpath : windows_core::PCSTR, mszexpandedpathlist : *mut i8, pcchpathlistlength : *mut u32) -> PDH_STATUS);
    unsafe { PdhExpandCounterPathA(szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _) }
}
#[inline]
pub unsafe fn PdhExpandCounterPathW<P0>(szwildcardpath: P0, mszexpandedpathlist: Option<*mut u16>, pcchpathlistlength: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandCounterPathW(szwildcardpath : windows_core::PCWSTR, mszexpandedpathlist : *mut u16, pcchpathlistlength : *mut u32) -> PDH_STATUS);
    unsafe { PdhExpandCounterPathW(szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _) }
}
#[inline]
pub unsafe fn PdhExpandWildCardPathA<P0, P1>(szdatasource: P0, szwildcardpath: P1, mszexpandedpathlist: Option<*mut i8>, pcchpathlistlength: *mut u32, dwflags: u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandWildCardPathA(szdatasource : windows_core::PCSTR, szwildcardpath : windows_core::PCSTR, mszexpandedpathlist : *mut i8, pcchpathlistlength : *mut u32, dwflags : u32) -> PDH_STATUS);
    unsafe { PdhExpandWildCardPathA(szdatasource.param().abi(), szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _, dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhExpandWildCardPathHA<P1>(hdatasource: Option<PDH_HLOG>, szwildcardpath: P1, mszexpandedpathlist: Option<*mut i8>, pcchpathlistlength: *mut u32, dwflags: u32) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandWildCardPathHA(hdatasource : PDH_HLOG, szwildcardpath : windows_core::PCSTR, mszexpandedpathlist : *mut i8, pcchpathlistlength : *mut u32, dwflags : u32) -> PDH_STATUS);
    unsafe { PdhExpandWildCardPathHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _, dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhExpandWildCardPathHW<P1>(hdatasource: Option<PDH_HLOG>, szwildcardpath: P1, mszexpandedpathlist: Option<*mut u16>, pcchpathlistlength: *mut u32, dwflags: u32) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandWildCardPathHW(hdatasource : PDH_HLOG, szwildcardpath : windows_core::PCWSTR, mszexpandedpathlist : *mut u16, pcchpathlistlength : *mut u32, dwflags : u32) -> PDH_STATUS);
    unsafe { PdhExpandWildCardPathHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _, dwflags) }
}
#[inline]
pub unsafe fn PdhExpandWildCardPathW<P0, P1>(szdatasource: P0, szwildcardpath: P1, mszexpandedpathlist: Option<*mut u16>, pcchpathlistlength: *mut u32, dwflags: u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandWildCardPathW(szdatasource : windows_core::PCWSTR, szwildcardpath : windows_core::PCWSTR, mszexpandedpathlist : *mut u16, pcchpathlistlength : *mut u32, dwflags : u32) -> PDH_STATUS);
    unsafe { PdhExpandWildCardPathW(szdatasource.param().abi(), szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _, dwflags) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PdhFormatFromRawValue(dwcountertype: u32, dwformat: u32, ptimebase: Option<*const i64>, prawvalue1: *const PDH_RAW_COUNTER, prawvalue2: *const PDH_RAW_COUNTER, pfmtvalue: *mut PDH_FMT_COUNTERVALUE) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhFormatFromRawValue(dwcountertype : u32, dwformat : u32, ptimebase : *const i64, prawvalue1 : *const PDH_RAW_COUNTER, prawvalue2 : *const PDH_RAW_COUNTER, pfmtvalue : *mut PDH_FMT_COUNTERVALUE) -> PDH_STATUS);
    unsafe { PdhFormatFromRawValue(dwcountertype, dwformat, ptimebase.unwrap_or(core::mem::zeroed()) as _, prawvalue1, prawvalue2, pfmtvalue as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhGetCounterInfoA(hcounter: PDH_HCOUNTER, bretrieveexplaintext: bool, pdwbuffersize: *mut u32, lpbuffer: Option<*mut PDH_COUNTER_INFO_A>) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetCounterInfoA(hcounter : PDH_HCOUNTER, bretrieveexplaintext : bool, pdwbuffersize : *mut u32, lpbuffer : *mut PDH_COUNTER_INFO_A) -> PDH_STATUS);
    unsafe { PdhGetCounterInfoA(hcounter, bretrieveexplaintext, pdwbuffersize as _, lpbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhGetCounterInfoW(hcounter: PDH_HCOUNTER, bretrieveexplaintext: bool, pdwbuffersize: *mut u32, lpbuffer: Option<*mut PDH_COUNTER_INFO_W>) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetCounterInfoW(hcounter : PDH_HCOUNTER, bretrieveexplaintext : bool, pdwbuffersize : *mut u32, lpbuffer : *mut PDH_COUNTER_INFO_W) -> PDH_STATUS);
    unsafe { PdhGetCounterInfoW(hcounter, bretrieveexplaintext, pdwbuffersize as _, lpbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhGetCounterTimeBase(hcounter: PDH_HCOUNTER, ptimebase: *mut i64) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetCounterTimeBase(hcounter : PDH_HCOUNTER, ptimebase : *mut i64) -> PDH_STATUS);
    unsafe { PdhGetCounterTimeBase(hcounter, ptimebase as _) }
}
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeA<P0>(szdatasource: P0, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeA(szdatasource : windows_core::PCSTR, pdwnumentries : *mut u32, pinfo : *mut PDH_TIME_INFO, pdwbuffersize : *mut u32) -> PDH_STATUS);
    unsafe { PdhGetDataSourceTimeRangeA(szdatasource.param().abi(), pdwnumentries as _, pinfo as _, pdwbuffersize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeH(hdatasource: Option<PDH_HLOG>, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeH(hdatasource : PDH_HLOG, pdwnumentries : *mut u32, pinfo : *mut PDH_TIME_INFO, pdwbuffersize : *mut u32) -> PDH_STATUS);
    unsafe { PdhGetDataSourceTimeRangeH(hdatasource.unwrap_or(core::mem::zeroed()) as _, pdwnumentries as _, pinfo as _, pdwbuffersize as _) }
}
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeW<P0>(szdatasource: P0, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeW(szdatasource : windows_core::PCWSTR, pdwnumentries : *mut u32, pinfo : *mut PDH_TIME_INFO, pdwbuffersize : *mut u32) -> PDH_STATUS);
    unsafe { PdhGetDataSourceTimeRangeW(szdatasource.param().abi(), pdwnumentries as _, pinfo as _, pdwbuffersize as _) }
}
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterA<P0, P1, P2>(szdatasource: P0, szmachinename: P1, szobjectname: P2, szdefaultcountername: Option<windows_core::PSTR>, pcchbuffersize: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterA(szdatasource : windows_core::PCSTR, szmachinename : windows_core::PCSTR, szobjectname : windows_core::PCSTR, szdefaultcountername : windows_core::PSTR, pcchbuffersize : *mut u32) -> PDH_STATUS);
    unsafe { PdhGetDefaultPerfCounterA(szdatasource.param().abi(), szmachinename.param().abi(), szobjectname.param().abi(), szdefaultcountername.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterHA<P1, P2>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szobjectname: P2, szdefaultcountername: Option<windows_core::PSTR>, pcchbuffersize: *mut u32) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterHA(hdatasource : PDH_HLOG, szmachinename : windows_core::PCSTR, szobjectname : windows_core::PCSTR, szdefaultcountername : windows_core::PSTR, pcchbuffersize : *mut u32) -> PDH_STATUS);
    unsafe { PdhGetDefaultPerfCounterHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szobjectname.param().abi(), szdefaultcountername.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterHW<P1, P2>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szobjectname: P2, szdefaultcountername: Option<windows_core::PWSTR>, pcchbuffersize: *mut u32) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterHW(hdatasource : PDH_HLOG, szmachinename : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, szdefaultcountername : windows_core::PWSTR, pcchbuffersize : *mut u32) -> PDH_STATUS);
    unsafe { PdhGetDefaultPerfCounterHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szobjectname.param().abi(), szdefaultcountername.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterW<P0, P1, P2>(szdatasource: P0, szmachinename: P1, szobjectname: P2, szdefaultcountername: Option<windows_core::PWSTR>, pcchbuffersize: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterW(szdatasource : windows_core::PCWSTR, szmachinename : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, szdefaultcountername : windows_core::PWSTR, pcchbuffersize : *mut u32) -> PDH_STATUS);
    unsafe { PdhGetDefaultPerfCounterW(szdatasource.param().abi(), szmachinename.param().abi(), szobjectname.param().abi(), szdefaultcountername.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectA<P0, P1>(szdatasource: P0, szmachinename: P1, szdefaultobjectname: Option<windows_core::PSTR>, pcchbuffersize: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectA(szdatasource : windows_core::PCSTR, szmachinename : windows_core::PCSTR, szdefaultobjectname : windows_core::PSTR, pcchbuffersize : *mut u32) -> PDH_STATUS);
    unsafe { PdhGetDefaultPerfObjectA(szdatasource.param().abi(), szmachinename.param().abi(), szdefaultobjectname.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectHA<P1>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szdefaultobjectname: Option<windows_core::PSTR>, pcchbuffersize: *mut u32) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectHA(hdatasource : PDH_HLOG, szmachinename : windows_core::PCSTR, szdefaultobjectname : windows_core::PSTR, pcchbuffersize : *mut u32) -> PDH_STATUS);
    unsafe { PdhGetDefaultPerfObjectHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szdefaultobjectname.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectHW<P1>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szdefaultobjectname: Option<windows_core::PWSTR>, pcchbuffersize: *mut u32) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectHW(hdatasource : PDH_HLOG, szmachinename : windows_core::PCWSTR, szdefaultobjectname : windows_core::PWSTR, pcchbuffersize : *mut u32) -> PDH_STATUS);
    unsafe { PdhGetDefaultPerfObjectHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szdefaultobjectname.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectW<P0, P1>(szdatasource: P0, szmachinename: P1, szdefaultobjectname: Option<windows_core::PWSTR>, pcchbuffersize: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectW(szdatasource : windows_core::PCWSTR, szmachinename : windows_core::PCWSTR, szdefaultobjectname : windows_core::PWSTR, pcchbuffersize : *mut u32) -> PDH_STATUS);
    unsafe { PdhGetDefaultPerfObjectW(szdatasource.param().abi(), szmachinename.param().abi(), szdefaultobjectname.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhGetDllVersion(lpdwversion: Option<*mut u32>) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetDllVersion(lpdwversion : *mut u32) -> PDH_STATUS);
    unsafe { PdhGetDllVersion(lpdwversion.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhGetFormattedCounterArrayA(hcounter: PDH_HCOUNTER, dwformat: u32, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: Option<*mut PDH_FMT_COUNTERVALUE_ITEM_A>) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetFormattedCounterArrayA(hcounter : PDH_HCOUNTER, dwformat : u32, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_FMT_COUNTERVALUE_ITEM_A) -> PDH_STATUS);
    unsafe { PdhGetFormattedCounterArrayA(hcounter, dwformat, lpdwbuffersize as _, lpdwitemcount as _, itembuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhGetFormattedCounterArrayW(hcounter: PDH_HCOUNTER, dwformat: u32, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: Option<*mut PDH_FMT_COUNTERVALUE_ITEM_W>) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetFormattedCounterArrayW(hcounter : PDH_HCOUNTER, dwformat : u32, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_FMT_COUNTERVALUE_ITEM_W) -> PDH_STATUS);
    unsafe { PdhGetFormattedCounterArrayW(hcounter, dwformat, lpdwbuffersize as _, lpdwitemcount as _, itembuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhGetFormattedCounterValue(hcounter: PDH_HCOUNTER, dwformat: u32, lpdwtype: Option<*mut u32>, pvalue: *mut PDH_FMT_COUNTERVALUE) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetFormattedCounterValue(hcounter : PDH_HCOUNTER, dwformat : u32, lpdwtype : *mut u32, pvalue : *mut PDH_FMT_COUNTERVALUE) -> PDH_STATUS);
    unsafe { PdhGetFormattedCounterValue(hcounter, dwformat, lpdwtype.unwrap_or(core::mem::zeroed()) as _, pvalue as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhGetLogFileSize(hlog: PDH_HLOG, llsize: *mut i64) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetLogFileSize(hlog : PDH_HLOG, llsize : *mut i64) -> PDH_STATUS);
    unsafe { PdhGetLogFileSize(hlog, llsize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhGetLogSetGUID(hlog: PDH_HLOG, pguid: Option<*mut windows_core::GUID>, prunid: Option<*mut i32>) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetLogSetGUID(hlog : PDH_HLOG, pguid : *mut windows_core::GUID, prunid : *mut i32) -> PDH_STATUS);
    unsafe { PdhGetLogSetGUID(hlog, pguid.unwrap_or(core::mem::zeroed()) as _, prunid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn PdhGetRawCounterArrayA(hcounter: PDH_HCOUNTER, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: Option<*mut PDH_RAW_COUNTER_ITEM_A>) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetRawCounterArrayA(hcounter : PDH_HCOUNTER, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_RAW_COUNTER_ITEM_A) -> PDH_STATUS);
    unsafe { PdhGetRawCounterArrayA(hcounter, lpdwbuffersize as _, lpdwitemcount as _, itembuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn PdhGetRawCounterArrayW(hcounter: PDH_HCOUNTER, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: Option<*mut PDH_RAW_COUNTER_ITEM_W>) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetRawCounterArrayW(hcounter : PDH_HCOUNTER, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_RAW_COUNTER_ITEM_W) -> PDH_STATUS);
    unsafe { PdhGetRawCounterArrayW(hcounter, lpdwbuffersize as _, lpdwitemcount as _, itembuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn PdhGetRawCounterValue(hcounter: PDH_HCOUNTER, lpdwtype: Option<*mut u32>, pvalue: *mut PDH_RAW_COUNTER) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhGetRawCounterValue(hcounter : PDH_HCOUNTER, lpdwtype : *mut u32, pvalue : *mut PDH_RAW_COUNTER) -> PDH_STATUS);
    unsafe { PdhGetRawCounterValue(hcounter, lpdwtype.unwrap_or(core::mem::zeroed()) as _, pvalue as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhIsRealTimeQuery(hquery: PDH_HQUERY) -> windows_core::BOOL {
    windows_core::link!("pdh.dll" "system" fn PdhIsRealTimeQuery(hquery : PDH_HQUERY) -> windows_core::BOOL);
    unsafe { PdhIsRealTimeQuery(hquery) }
}
#[inline]
pub unsafe fn PdhLookupPerfIndexByNameA<P0, P1>(szmachinename: P0, sznamebuffer: P1, pdwindex: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhLookupPerfIndexByNameA(szmachinename : windows_core::PCSTR, sznamebuffer : windows_core::PCSTR, pdwindex : *mut u32) -> PDH_STATUS);
    unsafe { PdhLookupPerfIndexByNameA(szmachinename.param().abi(), sznamebuffer.param().abi(), pdwindex as _) }
}
#[inline]
pub unsafe fn PdhLookupPerfIndexByNameW<P0, P1>(szmachinename: P0, sznamebuffer: P1, pdwindex: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhLookupPerfIndexByNameW(szmachinename : windows_core::PCWSTR, sznamebuffer : windows_core::PCWSTR, pdwindex : *mut u32) -> PDH_STATUS);
    unsafe { PdhLookupPerfIndexByNameW(szmachinename.param().abi(), sznamebuffer.param().abi(), pdwindex as _) }
}
#[inline]
pub unsafe fn PdhLookupPerfNameByIndexA<P0>(szmachinename: P0, dwnameindex: u32, sznamebuffer: Option<windows_core::PSTR>, pcchnamebuffersize: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhLookupPerfNameByIndexA(szmachinename : windows_core::PCSTR, dwnameindex : u32, sznamebuffer : windows_core::PSTR, pcchnamebuffersize : *mut u32) -> PDH_STATUS);
    unsafe { PdhLookupPerfNameByIndexA(szmachinename.param().abi(), dwnameindex, sznamebuffer.unwrap_or(core::mem::zeroed()) as _, pcchnamebuffersize as _) }
}
#[inline]
pub unsafe fn PdhLookupPerfNameByIndexW<P0>(szmachinename: P0, dwnameindex: u32, sznamebuffer: Option<windows_core::PWSTR>, pcchnamebuffersize: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhLookupPerfNameByIndexW(szmachinename : windows_core::PCWSTR, dwnameindex : u32, sznamebuffer : windows_core::PWSTR, pcchnamebuffersize : *mut u32) -> PDH_STATUS);
    unsafe { PdhLookupPerfNameByIndexW(szmachinename.param().abi(), dwnameindex, sznamebuffer.unwrap_or(core::mem::zeroed()) as _, pcchnamebuffersize as _) }
}
#[inline]
pub unsafe fn PdhMakeCounterPathA(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer: Option<windows_core::PSTR>, pcchbuffersize: *mut u32, dwflags: u32) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhMakeCounterPathA(pcounterpathelements : *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer : windows_core::PSTR, pcchbuffersize : *mut u32, dwflags : u32) -> PDH_STATUS);
    unsafe { PdhMakeCounterPathA(pcounterpathelements, szfullpathbuffer.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwflags) }
}
#[inline]
pub unsafe fn PdhMakeCounterPathW(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer: Option<windows_core::PWSTR>, pcchbuffersize: *mut u32, dwflags: u32) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhMakeCounterPathW(pcounterpathelements : *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer : windows_core::PWSTR, pcchbuffersize : *mut u32, dwflags : u32) -> PDH_STATUS);
    unsafe { PdhMakeCounterPathW(pcounterpathelements, szfullpathbuffer.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhOpenLogA<P0, P5>(szlogfilename: P0, dwaccessflags: u32, lpdwlogtype: *mut u32, hquery: Option<PDH_HQUERY>, dwmaxsize: u32, szusercaption: P5, phlog: *mut PDH_HLOG) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhOpenLogA(szlogfilename : windows_core::PCSTR, dwaccessflags : u32, lpdwlogtype : *mut u32, hquery : PDH_HQUERY, dwmaxsize : u32, szusercaption : windows_core::PCSTR, phlog : *mut PDH_HLOG) -> PDH_STATUS);
    unsafe { PdhOpenLogA(szlogfilename.param().abi(), dwaccessflags, lpdwlogtype as _, hquery.unwrap_or(core::mem::zeroed()) as _, dwmaxsize, szusercaption.param().abi(), phlog as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhOpenLogW<P0, P5>(szlogfilename: P0, dwaccessflags: u32, lpdwlogtype: *mut u32, hquery: Option<PDH_HQUERY>, dwmaxsize: u32, szusercaption: P5, phlog: *mut PDH_HLOG) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhOpenLogW(szlogfilename : windows_core::PCWSTR, dwaccessflags : u32, lpdwlogtype : *mut u32, hquery : PDH_HQUERY, dwmaxsize : u32, szusercaption : windows_core::PCWSTR, phlog : *mut PDH_HLOG) -> PDH_STATUS);
    unsafe { PdhOpenLogW(szlogfilename.param().abi(), dwaccessflags, lpdwlogtype as _, hquery.unwrap_or(core::mem::zeroed()) as _, dwmaxsize, szusercaption.param().abi(), phlog as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhOpenQueryA<P0>(szdatasource: P0, dwuserdata: usize, phquery: *mut PDH_HQUERY) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhOpenQueryA(szdatasource : windows_core::PCSTR, dwuserdata : usize, phquery : *mut PDH_HQUERY) -> PDH_STATUS);
    unsafe { PdhOpenQueryA(szdatasource.param().abi(), dwuserdata, phquery as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhOpenQueryH(hdatasource: Option<PDH_HLOG>, dwuserdata: usize, phquery: *mut PDH_HQUERY) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhOpenQueryH(hdatasource : PDH_HLOG, dwuserdata : usize, phquery : *mut PDH_HQUERY) -> PDH_STATUS);
    unsafe { PdhOpenQueryH(hdatasource.unwrap_or(core::mem::zeroed()) as _, dwuserdata, phquery as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhOpenQueryW<P0>(szdatasource: P0, dwuserdata: usize, phquery: *mut PDH_HQUERY) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhOpenQueryW(szdatasource : windows_core::PCWSTR, dwuserdata : usize, phquery : *mut PDH_HQUERY) -> PDH_STATUS);
    unsafe { PdhOpenQueryW(szdatasource.param().abi(), dwuserdata, phquery as _) }
}
#[inline]
pub unsafe fn PdhParseCounterPathA<P0>(szfullpathbuffer: P0, pcounterpathelements: Option<*mut PDH_COUNTER_PATH_ELEMENTS_A>, pdwbuffersize: *mut u32, dwflags: u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhParseCounterPathA(szfullpathbuffer : windows_core::PCSTR, pcounterpathelements : *mut PDH_COUNTER_PATH_ELEMENTS_A, pdwbuffersize : *mut u32, dwflags : u32) -> PDH_STATUS);
    unsafe { PdhParseCounterPathA(szfullpathbuffer.param().abi(), pcounterpathelements.unwrap_or(core::mem::zeroed()) as _, pdwbuffersize as _, dwflags) }
}
#[inline]
pub unsafe fn PdhParseCounterPathW<P0>(szfullpathbuffer: P0, pcounterpathelements: Option<*mut PDH_COUNTER_PATH_ELEMENTS_W>, pdwbuffersize: *mut u32, dwflags: u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhParseCounterPathW(szfullpathbuffer : windows_core::PCWSTR, pcounterpathelements : *mut PDH_COUNTER_PATH_ELEMENTS_W, pdwbuffersize : *mut u32, dwflags : u32) -> PDH_STATUS);
    unsafe { PdhParseCounterPathW(szfullpathbuffer.param().abi(), pcounterpathelements.unwrap_or(core::mem::zeroed()) as _, pdwbuffersize as _, dwflags) }
}
#[inline]
pub unsafe fn PdhParseInstanceNameA<P0>(szinstancestring: P0, szinstancename: Option<windows_core::PSTR>, pcchinstancenamelength: *mut u32, szparentname: Option<windows_core::PSTR>, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhParseInstanceNameA(szinstancestring : windows_core::PCSTR, szinstancename : windows_core::PSTR, pcchinstancenamelength : *mut u32, szparentname : windows_core::PSTR, pcchparentnamelength : *mut u32, lpindex : *mut u32) -> PDH_STATUS);
    unsafe { PdhParseInstanceNameA(szinstancestring.param().abi(), szinstancename.unwrap_or(core::mem::zeroed()) as _, pcchinstancenamelength as _, szparentname.unwrap_or(core::mem::zeroed()) as _, pcchparentnamelength as _, lpindex as _) }
}
#[inline]
pub unsafe fn PdhParseInstanceNameW<P0>(szinstancestring: P0, szinstancename: Option<windows_core::PWSTR>, pcchinstancenamelength: *mut u32, szparentname: Option<windows_core::PWSTR>, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> PDH_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhParseInstanceNameW(szinstancestring : windows_core::PCWSTR, szinstancename : windows_core::PWSTR, pcchinstancenamelength : *mut u32, szparentname : windows_core::PWSTR, pcchparentnamelength : *mut u32, lpindex : *mut u32) -> PDH_STATUS);
    unsafe { PdhParseInstanceNameW(szinstancestring.param().abi(), szinstancename.unwrap_or(core::mem::zeroed()) as _, pcchinstancenamelength as _, szparentname.unwrap_or(core::mem::zeroed()) as _, pcchparentnamelength as _, lpindex as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn PdhReadRawLogRecord(hlog: PDH_HLOG, ftrecord: super::minwindef::FILETIME, prawlogrecord: Option<*mut PDH_RAW_LOG_RECORD>, pdwbufferlength: *mut u32) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhReadRawLogRecord(hlog : PDH_HLOG, ftrecord : super::minwindef::FILETIME, prawlogrecord : *mut PDH_RAW_LOG_RECORD, pdwbufferlength : *mut u32) -> PDH_STATUS);
    unsafe { PdhReadRawLogRecord(hlog, ftrecord, prawlogrecord.unwrap_or(core::mem::zeroed()) as _, pdwbufferlength as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhRemoveCounter(hcounter: PDH_HCOUNTER) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhRemoveCounter(hcounter : PDH_HCOUNTER) -> PDH_STATUS);
    unsafe { PdhRemoveCounter(hcounter) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn PdhSelectDataSourceA(hwndowner: super::windef::HWND, dwflags: u32, szdatasource: windows_core::PSTR, pcchbufferlength: *mut u32) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhSelectDataSourceA(hwndowner : super::windef::HWND, dwflags : u32, szdatasource : windows_core::PSTR, pcchbufferlength : *mut u32) -> PDH_STATUS);
    unsafe { PdhSelectDataSourceA(hwndowner, dwflags, szdatasource, pcchbufferlength as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn PdhSelectDataSourceW(hwndowner: super::windef::HWND, dwflags: u32, szdatasource: windows_core::PWSTR, pcchbufferlength: *mut u32) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhSelectDataSourceW(hwndowner : super::windef::HWND, dwflags : u32, szdatasource : windows_core::PWSTR, pcchbufferlength : *mut u32) -> PDH_STATUS);
    unsafe { PdhSelectDataSourceW(hwndowner, dwflags, szdatasource, pcchbufferlength as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhSetCounterScaleFactor(hcounter: PDH_HCOUNTER, lfactor: i32) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhSetCounterScaleFactor(hcounter : PDH_HCOUNTER, lfactor : i32) -> PDH_STATUS);
    unsafe { PdhSetCounterScaleFactor(hcounter as _, lfactor) }
}
#[inline]
pub unsafe fn PdhSetDefaultRealTimeDataSource(dwdatasourceid: u32) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhSetDefaultRealTimeDataSource(dwdatasourceid : u32) -> PDH_STATUS);
    unsafe { PdhSetDefaultRealTimeDataSource(dwdatasourceid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhSetLogSetRunID(hlog: PDH_HLOG, runid: i32) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhSetLogSetRunID(hlog : PDH_HLOG, runid : i32) -> PDH_STATUS);
    unsafe { PdhSetLogSetRunID(hlog as _, runid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhSetQueryTimeRange(hquery: PDH_HQUERY, pinfo: *const PDH_TIME_INFO) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhSetQueryTimeRange(hquery : PDH_HQUERY, pinfo : *const PDH_TIME_INFO) -> PDH_STATUS);
    unsafe { PdhSetQueryTimeRange(hquery, pinfo) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhUpdateLogA<P1>(hlog: PDH_HLOG, szuserstring: P1) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhUpdateLogA(hlog : PDH_HLOG, szuserstring : windows_core::PCSTR) -> PDH_STATUS);
    unsafe { PdhUpdateLogA(hlog, szuserstring.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhUpdateLogFileCatalog(hlog: PDH_HLOG) -> PDH_STATUS {
    windows_core::link!("pdh.dll" "system" fn PdhUpdateLogFileCatalog(hlog : PDH_HLOG) -> PDH_STATUS);
    unsafe { PdhUpdateLogFileCatalog(hlog) }
}
#[cfg(feature = "winnt")]
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
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PdhValidatePathExA<P1>(hdatasource: Option<PDH_HLOG>, szfullpathbuffer: P1) -> PDH_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhValidatePathExA(hdatasource : PDH_HLOG, szfullpathbuffer : windows_core::PCSTR) -> PDH_STATUS);
    unsafe { PdhValidatePathExA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szfullpathbuffer.param().abi()) }
}
#[cfg(feature = "winnt")]
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
#[derive(Clone, Copy, Debug, Default)]
pub struct PDH_BROWSE_DLG_CONFIG_A {
    pub _bitfield: u32,
    pub hWndOwner: super::windef::HWND,
    pub szDataSource: windows_core::PSTR,
    pub szReturnPathBuffer: windows_core::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: PDH_STATUS,
    pub dwDefaultDetailLevel: u32,
    pub szDialogBoxCaption: windows_core::PSTR,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct PDH_BROWSE_DLG_CONFIG_HA {
    pub _bitfield: u32,
    pub hWndOwner: super::windef::HWND,
    pub hDataSource: PDH_HLOG,
    pub szReturnPathBuffer: windows_core::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: PDH_STATUS,
    pub dwDefaultDetailLevel: u32,
    pub szDialogBoxCaption: windows_core::PSTR,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct PDH_BROWSE_DLG_CONFIG_HW {
    pub _bitfield: u32,
    pub hWndOwner: super::windef::HWND,
    pub hDataSource: PDH_HLOG,
    pub szReturnPathBuffer: windows_core::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: PDH_STATUS,
    pub dwDefaultDetailLevel: u32,
    pub szDialogBoxCaption: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default)]
pub struct PDH_BROWSE_DLG_CONFIG_W {
    pub _bitfield: u32,
    pub hWndOwner: super::windef::HWND,
    pub szDataSource: windows_core::PWSTR,
    pub szReturnPathBuffer: windows_core::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: PDH_STATUS,
    pub dwDefaultDetailLevel: u32,
    pub szDialogBoxCaption: windows_core::PWSTR,
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
    pub szFullPath: windows_core::PSTR,
    pub Anonymous: PDH_COUNTER_INFO_A_0,
    pub szExplainText: windows_core::PSTR,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_COUNTER_INFO_A_0_0 {
    pub szMachineName: windows_core::PSTR,
    pub szObjectName: windows_core::PSTR,
    pub szInstanceName: windows_core::PSTR,
    pub szParentInstance: windows_core::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_core::PSTR,
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
    pub szFullPath: windows_core::PWSTR,
    pub Anonymous: PDH_COUNTER_INFO_W_0,
    pub szExplainText: windows_core::PWSTR,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_COUNTER_INFO_W_0_0 {
    pub szMachineName: windows_core::PWSTR,
    pub szObjectName: windows_core::PWSTR,
    pub szInstanceName: windows_core::PWSTR,
    pub szParentInstance: windows_core::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_COUNTER_PATH_ELEMENTS_A {
    pub szMachineName: windows_core::PSTR,
    pub szObjectName: windows_core::PSTR,
    pub szInstanceName: windows_core::PSTR,
    pub szParentInstance: windows_core::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_COUNTER_PATH_ELEMENTS_W {
    pub szMachineName: windows_core::PWSTR,
    pub szObjectName: windows_core::PWSTR,
    pub szInstanceName: windows_core::PWSTR,
    pub szParentInstance: windows_core::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_core::PWSTR,
}
pub const PDH_CVERSION_WIN40: u32 = 1024;
pub const PDH_CVERSION_WIN50: u32 = 1280;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_A {
    pub szMachineName: windows_core::PSTR,
    pub ObjectGUID: windows_core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_W {
    pub szMachineName: windows_core::PWSTR,
    pub ObjectGUID: windows_core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: windows_core::PWSTR,
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
    pub AnsiStringValue: windows_core::PCSTR,
    pub WideStringValue: windows_core::PCWSTR,
}
impl Default for PDH_FMT_COUNTERVALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_FMT_COUNTERVALUE_ITEM_A {
    pub szName: windows_core::PSTR,
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
    pub szName: windows_core::PWSTR,
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
    pub szLogFileCaption: windows_core::PSTR,
    pub szDefaultDir: windows_core::PSTR,
    pub szBaseFileName: windows_core::PSTR,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: windows_core::PSTR,
    pub PdlCounterList: windows_core::PSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::minwindef::FILETIME,
    pub PdlLogEndTime: super::minwindef::FILETIME,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub TlLogFileName: windows_core::PSTR,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: windows_core::PWSTR,
    pub szDefaultDir: windows_core::PWSTR,
    pub szBaseFileName: windows_core::PWSTR,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: windows_core::PWSTR,
    pub PdlCounterList: windows_core::PWSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::minwindef::FILETIME,
    pub PdlLogEndTime: super::minwindef::FILETIME,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub TlLogFileName: windows_core::PWSTR,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_RAW_COUNTER {
    pub CStatus: u32,
    pub TimeStamp: super::minwindef::FILETIME,
    pub FirstValue: i64,
    pub SecondValue: i64,
    pub MultiCount: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_RAW_COUNTER_ITEM_A {
    pub szName: windows_core::PSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_RAW_COUNTER_ITEM_W {
    pub szName: windows_core::PWSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PDH_STATUS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
