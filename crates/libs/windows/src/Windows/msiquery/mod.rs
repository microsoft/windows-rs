#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiCreateRecord(cparams: u32) -> super::msi::MSIHANDLE {
    windows_core::link!("msi.dll" "system" fn MsiCreateRecord(cparams : u32) -> super::msi::MSIHANDLE);
    unsafe { MsiCreateRecord(cparams) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiCreateTransformSummaryInfoA<P2>(hdatabase: super::msi::MSIHANDLE, hdatabasereference: super::msi::MSIHANDLE, sztransformfile: P2, ierrorconditions: i32, ivalidation: i32) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiCreateTransformSummaryInfoA(hdatabase : super::msi::MSIHANDLE, hdatabasereference : super::msi::MSIHANDLE, sztransformfile : windows_core::PCSTR, ierrorconditions : i32, ivalidation : i32) -> u32);
    unsafe { MsiCreateTransformSummaryInfoA(hdatabase, hdatabasereference, sztransformfile.param().abi(), ierrorconditions, ivalidation) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiCreateTransformSummaryInfoW<P2>(hdatabase: super::msi::MSIHANDLE, hdatabasereference: super::msi::MSIHANDLE, sztransformfile: P2, ierrorconditions: i32, ivalidation: i32) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiCreateTransformSummaryInfoW(hdatabase : super::msi::MSIHANDLE, hdatabasereference : super::msi::MSIHANDLE, sztransformfile : windows_core::PCWSTR, ierrorconditions : i32, ivalidation : i32) -> u32);
    unsafe { MsiCreateTransformSummaryInfoW(hdatabase, hdatabasereference, sztransformfile.param().abi(), ierrorconditions, ivalidation) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseApplyTransformA<P1>(hdatabase: super::msi::MSIHANDLE, sztransformfile: P1, ierrorconditions: i32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseApplyTransformA(hdatabase : super::msi::MSIHANDLE, sztransformfile : windows_core::PCSTR, ierrorconditions : i32) -> u32);
    unsafe { MsiDatabaseApplyTransformA(hdatabase, sztransformfile.param().abi(), ierrorconditions) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseApplyTransformW<P1>(hdatabase: super::msi::MSIHANDLE, sztransformfile: P1, ierrorconditions: i32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseApplyTransformW(hdatabase : super::msi::MSIHANDLE, sztransformfile : windows_core::PCWSTR, ierrorconditions : i32) -> u32);
    unsafe { MsiDatabaseApplyTransformW(hdatabase, sztransformfile.param().abi(), ierrorconditions) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseCommit(hdatabase: super::msi::MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseCommit(hdatabase : super::msi::MSIHANDLE) -> u32);
    unsafe { MsiDatabaseCommit(hdatabase) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseExportA<P1, P2, P3>(hdatabase: super::msi::MSIHANDLE, sztablename: P1, szfolderpath: P2, szfilename: P3) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseExportA(hdatabase : super::msi::MSIHANDLE, sztablename : windows_core::PCSTR, szfolderpath : windows_core::PCSTR, szfilename : windows_core::PCSTR) -> u32);
    unsafe { MsiDatabaseExportA(hdatabase, sztablename.param().abi(), szfolderpath.param().abi(), szfilename.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseExportW<P1, P2, P3>(hdatabase: super::msi::MSIHANDLE, sztablename: P1, szfolderpath: P2, szfilename: P3) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseExportW(hdatabase : super::msi::MSIHANDLE, sztablename : windows_core::PCWSTR, szfolderpath : windows_core::PCWSTR, szfilename : windows_core::PCWSTR) -> u32);
    unsafe { MsiDatabaseExportW(hdatabase, sztablename.param().abi(), szfolderpath.param().abi(), szfilename.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseGenerateTransformA<P2>(hdatabase: super::msi::MSIHANDLE, hdatabasereference: super::msi::MSIHANDLE, sztransformfile: P2, ireserved1: i32, ireserved2: i32) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseGenerateTransformA(hdatabase : super::msi::MSIHANDLE, hdatabasereference : super::msi::MSIHANDLE, sztransformfile : windows_core::PCSTR, ireserved1 : i32, ireserved2 : i32) -> u32);
    unsafe { MsiDatabaseGenerateTransformA(hdatabase, hdatabasereference, sztransformfile.param().abi(), ireserved1, ireserved2) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseGenerateTransformW<P2>(hdatabase: super::msi::MSIHANDLE, hdatabasereference: super::msi::MSIHANDLE, sztransformfile: P2, ireserved1: i32, ireserved2: i32) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseGenerateTransformW(hdatabase : super::msi::MSIHANDLE, hdatabasereference : super::msi::MSIHANDLE, sztransformfile : windows_core::PCWSTR, ireserved1 : i32, ireserved2 : i32) -> u32);
    unsafe { MsiDatabaseGenerateTransformW(hdatabase, hdatabasereference, sztransformfile.param().abi(), ireserved1, ireserved2) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseGetPrimaryKeysA<P1>(hdatabase: super::msi::MSIHANDLE, sztablename: P1, phrecord: *mut super::msi::MSIHANDLE) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseGetPrimaryKeysA(hdatabase : super::msi::MSIHANDLE, sztablename : windows_core::PCSTR, phrecord : *mut super::msi::MSIHANDLE) -> u32);
    unsafe { MsiDatabaseGetPrimaryKeysA(hdatabase, sztablename.param().abi(), phrecord as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseGetPrimaryKeysW<P1>(hdatabase: super::msi::MSIHANDLE, sztablename: P1, phrecord: *mut super::msi::MSIHANDLE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseGetPrimaryKeysW(hdatabase : super::msi::MSIHANDLE, sztablename : windows_core::PCWSTR, phrecord : *mut super::msi::MSIHANDLE) -> u32);
    unsafe { MsiDatabaseGetPrimaryKeysW(hdatabase, sztablename.param().abi(), phrecord as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseImportA<P1, P2>(hdatabase: super::msi::MSIHANDLE, szfolderpath: P1, szfilename: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseImportA(hdatabase : super::msi::MSIHANDLE, szfolderpath : windows_core::PCSTR, szfilename : windows_core::PCSTR) -> u32);
    unsafe { MsiDatabaseImportA(hdatabase, szfolderpath.param().abi(), szfilename.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseImportW<P1, P2>(hdatabase: super::msi::MSIHANDLE, szfolderpath: P1, szfilename: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseImportW(hdatabase : super::msi::MSIHANDLE, szfolderpath : windows_core::PCWSTR, szfilename : windows_core::PCWSTR) -> u32);
    unsafe { MsiDatabaseImportW(hdatabase, szfolderpath.param().abi(), szfilename.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseIsTablePersistentA<P1>(hdatabase: super::msi::MSIHANDLE, sztablename: P1) -> MSICONDITION
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseIsTablePersistentA(hdatabase : super::msi::MSIHANDLE, sztablename : windows_core::PCSTR) -> MSICONDITION);
    unsafe { MsiDatabaseIsTablePersistentA(hdatabase, sztablename.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseIsTablePersistentW<P1>(hdatabase: super::msi::MSIHANDLE, sztablename: P1) -> MSICONDITION
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseIsTablePersistentW(hdatabase : super::msi::MSIHANDLE, sztablename : windows_core::PCWSTR) -> MSICONDITION);
    unsafe { MsiDatabaseIsTablePersistentW(hdatabase, sztablename.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseMergeA<P2>(hdatabase: super::msi::MSIHANDLE, hdatabasemerge: super::msi::MSIHANDLE, sztablename: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseMergeA(hdatabase : super::msi::MSIHANDLE, hdatabasemerge : super::msi::MSIHANDLE, sztablename : windows_core::PCSTR) -> u32);
    unsafe { MsiDatabaseMergeA(hdatabase, hdatabasemerge, sztablename.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseMergeW<P2>(hdatabase: super::msi::MSIHANDLE, hdatabasemerge: super::msi::MSIHANDLE, sztablename: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseMergeW(hdatabase : super::msi::MSIHANDLE, hdatabasemerge : super::msi::MSIHANDLE, sztablename : windows_core::PCWSTR) -> u32);
    unsafe { MsiDatabaseMergeW(hdatabase, hdatabasemerge, sztablename.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseOpenViewA<P1>(hdatabase: super::msi::MSIHANDLE, szquery: P1, phview: *mut super::msi::MSIHANDLE) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseOpenViewA(hdatabase : super::msi::MSIHANDLE, szquery : windows_core::PCSTR, phview : *mut super::msi::MSIHANDLE) -> u32);
    unsafe { MsiDatabaseOpenViewA(hdatabase, szquery.param().abi(), phview as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseOpenViewW<P1>(hdatabase: super::msi::MSIHANDLE, szquery: P1, phview: *mut super::msi::MSIHANDLE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseOpenViewW(hdatabase : super::msi::MSIHANDLE, szquery : windows_core::PCWSTR, phview : *mut super::msi::MSIHANDLE) -> u32);
    unsafe { MsiDatabaseOpenViewW(hdatabase, szquery.param().abi(), phview as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDoActionA<P1>(hinstall: super::msi::MSIHANDLE, szaction: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDoActionA(hinstall : super::msi::MSIHANDLE, szaction : windows_core::PCSTR) -> u32);
    unsafe { MsiDoActionA(hinstall, szaction.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDoActionW<P1>(hinstall: super::msi::MSIHANDLE, szaction: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDoActionW(hinstall : super::msi::MSIHANDLE, szaction : windows_core::PCWSTR) -> u32);
    unsafe { MsiDoActionW(hinstall, szaction.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiEnableUIPreview(hdatabase: super::msi::MSIHANDLE, phpreview: *mut super::msi::MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiEnableUIPreview(hdatabase : super::msi::MSIHANDLE, phpreview : *mut super::msi::MSIHANDLE) -> u32);
    unsafe { MsiEnableUIPreview(hdatabase, phpreview as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiEnumComponentCostsA<P1>(hinstall: super::msi::MSIHANDLE, szcomponent: P1, dwindex: u32, istate: super::msi::INSTALLSTATE, szdrivebuf: windows_core::PSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentCostsA(hinstall : super::msi::MSIHANDLE, szcomponent : windows_core::PCSTR, dwindex : u32, istate : super::msi::INSTALLSTATE, szdrivebuf : windows_core::PSTR, pcchdrivebuf : *mut u32, picost : *mut i32, pitempcost : *mut i32) -> u32);
    unsafe { MsiEnumComponentCostsA(hinstall, szcomponent.param().abi(), dwindex, istate, szdrivebuf, pcchdrivebuf as _, picost as _, pitempcost as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiEnumComponentCostsW<P1>(hinstall: super::msi::MSIHANDLE, szcomponent: P1, dwindex: u32, istate: super::msi::INSTALLSTATE, szdrivebuf: windows_core::PWSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentCostsW(hinstall : super::msi::MSIHANDLE, szcomponent : windows_core::PCWSTR, dwindex : u32, istate : super::msi::INSTALLSTATE, szdrivebuf : windows_core::PWSTR, pcchdrivebuf : *mut u32, picost : *mut i32, pitempcost : *mut i32) -> u32);
    unsafe { MsiEnumComponentCostsW(hinstall, szcomponent.param().abi(), dwindex, istate, szdrivebuf, pcchdrivebuf as _, picost as _, pitempcost as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiEvaluateConditionA<P1>(hinstall: super::msi::MSIHANDLE, szcondition: P1) -> MSICONDITION
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEvaluateConditionA(hinstall : super::msi::MSIHANDLE, szcondition : windows_core::PCSTR) -> MSICONDITION);
    unsafe { MsiEvaluateConditionA(hinstall, szcondition.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiEvaluateConditionW<P1>(hinstall: super::msi::MSIHANDLE, szcondition: P1) -> MSICONDITION
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEvaluateConditionW(hinstall : super::msi::MSIHANDLE, szcondition : windows_core::PCWSTR) -> MSICONDITION);
    unsafe { MsiEvaluateConditionW(hinstall, szcondition.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiFormatRecordA(hinstall: super::msi::MSIHANDLE, hrecord: super::msi::MSIHANDLE, szresultbuf: Option<windows_core::PSTR>, pcchresultbuf: Option<*mut u32>) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiFormatRecordA(hinstall : super::msi::MSIHANDLE, hrecord : super::msi::MSIHANDLE, szresultbuf : windows_core::PSTR, pcchresultbuf : *mut u32) -> u32);
    unsafe { MsiFormatRecordA(hinstall, hrecord, szresultbuf.unwrap_or(core::mem::zeroed()) as _, pcchresultbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiFormatRecordW(hinstall: super::msi::MSIHANDLE, hrecord: super::msi::MSIHANDLE, szresultbuf: Option<windows_core::PWSTR>, pcchresultbuf: Option<*mut u32>) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiFormatRecordW(hinstall : super::msi::MSIHANDLE, hrecord : super::msi::MSIHANDLE, szresultbuf : windows_core::PWSTR, pcchresultbuf : *mut u32) -> u32);
    unsafe { MsiFormatRecordW(hinstall, hrecord, szresultbuf.unwrap_or(core::mem::zeroed()) as _, pcchresultbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetActiveDatabase(hinstall: super::msi::MSIHANDLE) -> super::msi::MSIHANDLE {
    windows_core::link!("msi.dll" "system" fn MsiGetActiveDatabase(hinstall : super::msi::MSIHANDLE) -> super::msi::MSIHANDLE);
    unsafe { MsiGetActiveDatabase(hinstall) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetComponentStateA<P1>(hinstall: super::msi::MSIHANDLE, szcomponent: P1, piinstalled: *mut super::msi::INSTALLSTATE, piaction: *mut super::msi::INSTALLSTATE) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetComponentStateA(hinstall : super::msi::MSIHANDLE, szcomponent : windows_core::PCSTR, piinstalled : *mut super::msi::INSTALLSTATE, piaction : *mut super::msi::INSTALLSTATE) -> u32);
    unsafe { MsiGetComponentStateA(hinstall, szcomponent.param().abi(), piinstalled as _, piaction as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetComponentStateW<P1>(hinstall: super::msi::MSIHANDLE, szcomponent: P1, piinstalled: *mut super::msi::INSTALLSTATE, piaction: *mut super::msi::INSTALLSTATE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetComponentStateW(hinstall : super::msi::MSIHANDLE, szcomponent : windows_core::PCWSTR, piinstalled : *mut super::msi::INSTALLSTATE, piaction : *mut super::msi::INSTALLSTATE) -> u32);
    unsafe { MsiGetComponentStateW(hinstall, szcomponent.param().abi(), piinstalled as _, piaction as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetDatabaseState(hdatabase: super::msi::MSIHANDLE) -> MSIDBSTATE {
    windows_core::link!("msi.dll" "system" fn MsiGetDatabaseState(hdatabase : super::msi::MSIHANDLE) -> MSIDBSTATE);
    unsafe { MsiGetDatabaseState(hdatabase) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetFeatureCostA<P1>(hinstall: super::msi::MSIHANDLE, szfeature: P1, icosttree: MSICOSTTREE, istate: super::msi::INSTALLSTATE, picost: *mut i32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureCostA(hinstall : super::msi::MSIHANDLE, szfeature : windows_core::PCSTR, icosttree : MSICOSTTREE, istate : super::msi::INSTALLSTATE, picost : *mut i32) -> u32);
    unsafe { MsiGetFeatureCostA(hinstall, szfeature.param().abi(), icosttree, istate, picost as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetFeatureCostW<P1>(hinstall: super::msi::MSIHANDLE, szfeature: P1, icosttree: MSICOSTTREE, istate: super::msi::INSTALLSTATE, picost: *mut i32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureCostW(hinstall : super::msi::MSIHANDLE, szfeature : windows_core::PCWSTR, icosttree : MSICOSTTREE, istate : super::msi::INSTALLSTATE, picost : *mut i32) -> u32);
    unsafe { MsiGetFeatureCostW(hinstall, szfeature.param().abi(), icosttree, istate, picost as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetFeatureStateA<P1>(hinstall: super::msi::MSIHANDLE, szfeature: P1, piinstalled: *mut super::msi::INSTALLSTATE, piaction: *mut super::msi::INSTALLSTATE) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureStateA(hinstall : super::msi::MSIHANDLE, szfeature : windows_core::PCSTR, piinstalled : *mut super::msi::INSTALLSTATE, piaction : *mut super::msi::INSTALLSTATE) -> u32);
    unsafe { MsiGetFeatureStateA(hinstall, szfeature.param().abi(), piinstalled as _, piaction as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetFeatureStateW<P1>(hinstall: super::msi::MSIHANDLE, szfeature: P1, piinstalled: *mut super::msi::INSTALLSTATE, piaction: *mut super::msi::INSTALLSTATE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureStateW(hinstall : super::msi::MSIHANDLE, szfeature : windows_core::PCWSTR, piinstalled : *mut super::msi::INSTALLSTATE, piaction : *mut super::msi::INSTALLSTATE) -> u32);
    unsafe { MsiGetFeatureStateW(hinstall, szfeature.param().abi(), piinstalled as _, piaction as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetFeatureValidStatesA<P1>(hinstall: super::msi::MSIHANDLE, szfeature: P1, lpinstallstates: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureValidStatesA(hinstall : super::msi::MSIHANDLE, szfeature : windows_core::PCSTR, lpinstallstates : *mut u32) -> u32);
    unsafe { MsiGetFeatureValidStatesA(hinstall, szfeature.param().abi(), lpinstallstates as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetFeatureValidStatesW<P1>(hinstall: super::msi::MSIHANDLE, szfeature: P1, lpinstallstates: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureValidStatesW(hinstall : super::msi::MSIHANDLE, szfeature : windows_core::PCWSTR, lpinstallstates : *mut u32) -> u32);
    unsafe { MsiGetFeatureValidStatesW(hinstall, szfeature.param().abi(), lpinstallstates as _) }
}
#[cfg(all(feature = "msi", feature = "winnt"))]
#[inline]
pub unsafe fn MsiGetLanguage(hinstall: super::msi::MSIHANDLE) -> super::winnt::LANGID {
    windows_core::link!("msi.dll" "system" fn MsiGetLanguage(hinstall : super::msi::MSIHANDLE) -> super::winnt::LANGID);
    unsafe { MsiGetLanguage(hinstall) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetLastErrorRecord() -> super::msi::MSIHANDLE {
    windows_core::link!("msi.dll" "system" fn MsiGetLastErrorRecord() -> super::msi::MSIHANDLE);
    unsafe { MsiGetLastErrorRecord() }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetMode(hinstall: super::msi::MSIHANDLE, erunmode: MSIRUNMODE) -> windows_core::BOOL {
    windows_core::link!("msi.dll" "system" fn MsiGetMode(hinstall : super::msi::MSIHANDLE, erunmode : MSIRUNMODE) -> windows_core::BOOL);
    unsafe { MsiGetMode(hinstall, erunmode) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetPropertyA<P1>(hinstall: super::msi::MSIHANDLE, szname: P1, szvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPropertyA(hinstall : super::msi::MSIHANDLE, szname : windows_core::PCSTR, szvaluebuf : windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiGetPropertyA(hinstall, szname.param().abi(), szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetPropertyW<P1>(hinstall: super::msi::MSIHANDLE, szname: P1, szvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPropertyW(hinstall : super::msi::MSIHANDLE, szname : windows_core::PCWSTR, szvaluebuf : windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiGetPropertyW(hinstall, szname.param().abi(), szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetSourcePathA<P1>(hinstall: super::msi::MSIHANDLE, szfolder: P1, szpathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetSourcePathA(hinstall : super::msi::MSIHANDLE, szfolder : windows_core::PCSTR, szpathbuf : windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiGetSourcePathA(hinstall, szfolder.param().abi(), szpathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetSourcePathW<P1>(hinstall: super::msi::MSIHANDLE, szfolder: P1, szpathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetSourcePathW(hinstall : super::msi::MSIHANDLE, szfolder : windows_core::PCWSTR, szpathbuf : windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiGetSourcePathW(hinstall, szfolder.param().abi(), szpathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetSummaryInformationA<P1>(hdatabase: super::msi::MSIHANDLE, szdatabasepath: P1, uiupdatecount: u32, phsummaryinfo: *mut super::msi::MSIHANDLE) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetSummaryInformationA(hdatabase : super::msi::MSIHANDLE, szdatabasepath : windows_core::PCSTR, uiupdatecount : u32, phsummaryinfo : *mut super::msi::MSIHANDLE) -> u32);
    unsafe { MsiGetSummaryInformationA(hdatabase, szdatabasepath.param().abi(), uiupdatecount, phsummaryinfo as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetSummaryInformationW<P1>(hdatabase: super::msi::MSIHANDLE, szdatabasepath: P1, uiupdatecount: u32, phsummaryinfo: *mut super::msi::MSIHANDLE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetSummaryInformationW(hdatabase : super::msi::MSIHANDLE, szdatabasepath : windows_core::PCWSTR, uiupdatecount : u32, phsummaryinfo : *mut super::msi::MSIHANDLE) -> u32);
    unsafe { MsiGetSummaryInformationW(hdatabase, szdatabasepath.param().abi(), uiupdatecount, phsummaryinfo as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetTargetPathA<P1>(hinstall: super::msi::MSIHANDLE, szfolder: P1, szpathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetTargetPathA(hinstall : super::msi::MSIHANDLE, szfolder : windows_core::PCSTR, szpathbuf : windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiGetTargetPathA(hinstall, szfolder.param().abi(), szpathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetTargetPathW<P1>(hinstall: super::msi::MSIHANDLE, szfolder: P1, szpathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetTargetPathW(hinstall : super::msi::MSIHANDLE, szfolder : windows_core::PCWSTR, szpathbuf : windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiGetTargetPathW(hinstall, szfolder.param().abi(), szpathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiOpenDatabaseA<P0, P1>(szdatabasepath: P0, szpersist: P1, phdatabase: *mut super::msi::MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenDatabaseA(szdatabasepath : windows_core::PCSTR, szpersist : windows_core::PCSTR, phdatabase : *mut super::msi::MSIHANDLE) -> u32);
    unsafe { MsiOpenDatabaseA(szdatabasepath.param().abi(), szpersist.param().abi(), phdatabase as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiOpenDatabaseW<P0, P1>(szdatabasepath: P0, szpersist: P1, phdatabase: *mut super::msi::MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenDatabaseW(szdatabasepath : windows_core::PCWSTR, szpersist : windows_core::PCWSTR, phdatabase : *mut super::msi::MSIHANDLE) -> u32);
    unsafe { MsiOpenDatabaseW(szdatabasepath.param().abi(), szpersist.param().abi(), phdatabase as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiPreviewBillboardA<P1, P2>(hpreview: super::msi::MSIHANDLE, szcontrolname: P1, szbillboard: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiPreviewBillboardA(hpreview : super::msi::MSIHANDLE, szcontrolname : windows_core::PCSTR, szbillboard : windows_core::PCSTR) -> u32);
    unsafe { MsiPreviewBillboardA(hpreview, szcontrolname.param().abi(), szbillboard.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiPreviewBillboardW<P1, P2>(hpreview: super::msi::MSIHANDLE, szcontrolname: P1, szbillboard: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiPreviewBillboardW(hpreview : super::msi::MSIHANDLE, szcontrolname : windows_core::PCWSTR, szbillboard : windows_core::PCWSTR) -> u32);
    unsafe { MsiPreviewBillboardW(hpreview, szcontrolname.param().abi(), szbillboard.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiPreviewDialogA<P1>(hpreview: super::msi::MSIHANDLE, szdialogname: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiPreviewDialogA(hpreview : super::msi::MSIHANDLE, szdialogname : windows_core::PCSTR) -> u32);
    unsafe { MsiPreviewDialogA(hpreview, szdialogname.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiPreviewDialogW<P1>(hpreview: super::msi::MSIHANDLE, szdialogname: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiPreviewDialogW(hpreview : super::msi::MSIHANDLE, szdialogname : windows_core::PCWSTR) -> u32);
    unsafe { MsiPreviewDialogW(hpreview, szdialogname.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiProcessMessage(hinstall: super::msi::MSIHANDLE, emessagetype: super::msi::INSTALLMESSAGE, hrecord: super::msi::MSIHANDLE) -> i32 {
    windows_core::link!("msi.dll" "system" fn MsiProcessMessage(hinstall : super::msi::MSIHANDLE, emessagetype : super::msi::INSTALLMESSAGE, hrecord : super::msi::MSIHANDLE) -> i32);
    unsafe { MsiProcessMessage(hinstall, emessagetype, hrecord) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiRecordClearData(hrecord: super::msi::MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiRecordClearData(hrecord : super::msi::MSIHANDLE) -> u32);
    unsafe { MsiRecordClearData(hrecord) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiRecordDataSize(hrecord: super::msi::MSIHANDLE, ifield: u32) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiRecordDataSize(hrecord : super::msi::MSIHANDLE, ifield : u32) -> u32);
    unsafe { MsiRecordDataSize(hrecord, ifield) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiRecordGetFieldCount(hrecord: super::msi::MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiRecordGetFieldCount(hrecord : super::msi::MSIHANDLE) -> u32);
    unsafe { MsiRecordGetFieldCount(hrecord) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiRecordGetInteger(hrecord: super::msi::MSIHANDLE, ifield: u32) -> i32 {
    windows_core::link!("msi.dll" "system" fn MsiRecordGetInteger(hrecord : super::msi::MSIHANDLE, ifield : u32) -> i32);
    unsafe { MsiRecordGetInteger(hrecord, ifield) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiRecordGetStringA(hrecord: super::msi::MSIHANDLE, ifield: u32, szvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<*mut u32>) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiRecordGetStringA(hrecord : super::msi::MSIHANDLE, ifield : u32, szvaluebuf : windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiRecordGetStringA(hrecord, ifield, szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiRecordGetStringW(hrecord: super::msi::MSIHANDLE, ifield: u32, szvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<*mut u32>) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiRecordGetStringW(hrecord : super::msi::MSIHANDLE, ifield : u32, szvaluebuf : windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiRecordGetStringW(hrecord, ifield, szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiRecordIsNull(hrecord: super::msi::MSIHANDLE, ifield: u32) -> windows_core::BOOL {
    windows_core::link!("msi.dll" "system" fn MsiRecordIsNull(hrecord : super::msi::MSIHANDLE, ifield : u32) -> windows_core::BOOL);
    unsafe { MsiRecordIsNull(hrecord, ifield) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiRecordReadStream(hrecord: super::msi::MSIHANDLE, ifield: u32, szdatabuf: Option<*mut i8>, pcbdatabuf: *mut u32) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiRecordReadStream(hrecord : super::msi::MSIHANDLE, ifield : u32, szdatabuf : *mut i8, pcbdatabuf : *mut u32) -> u32);
    unsafe { MsiRecordReadStream(hrecord, ifield, szdatabuf.unwrap_or(core::mem::zeroed()) as _, pcbdatabuf as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiRecordSetInteger(hrecord: super::msi::MSIHANDLE, ifield: u32, ivalue: i32) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiRecordSetInteger(hrecord : super::msi::MSIHANDLE, ifield : u32, ivalue : i32) -> u32);
    unsafe { MsiRecordSetInteger(hrecord, ifield, ivalue) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiRecordSetStreamA<P2>(hrecord: super::msi::MSIHANDLE, ifield: u32, szfilepath: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiRecordSetStreamA(hrecord : super::msi::MSIHANDLE, ifield : u32, szfilepath : windows_core::PCSTR) -> u32);
    unsafe { MsiRecordSetStreamA(hrecord, ifield, szfilepath.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiRecordSetStreamW<P2>(hrecord: super::msi::MSIHANDLE, ifield: u32, szfilepath: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiRecordSetStreamW(hrecord : super::msi::MSIHANDLE, ifield : u32, szfilepath : windows_core::PCWSTR) -> u32);
    unsafe { MsiRecordSetStreamW(hrecord, ifield, szfilepath.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiRecordSetStringA<P2>(hrecord: super::msi::MSIHANDLE, ifield: u32, szvalue: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiRecordSetStringA(hrecord : super::msi::MSIHANDLE, ifield : u32, szvalue : windows_core::PCSTR) -> u32);
    unsafe { MsiRecordSetStringA(hrecord, ifield, szvalue.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiRecordSetStringW<P2>(hrecord: super::msi::MSIHANDLE, ifield: u32, szvalue: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiRecordSetStringW(hrecord : super::msi::MSIHANDLE, ifield : u32, szvalue : windows_core::PCWSTR) -> u32);
    unsafe { MsiRecordSetStringW(hrecord, ifield, szvalue.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiSequenceA<P1>(hinstall: super::msi::MSIHANDLE, sztable: P1, isequencemode: i32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSequenceA(hinstall : super::msi::MSIHANDLE, sztable : windows_core::PCSTR, isequencemode : i32) -> u32);
    unsafe { MsiSequenceA(hinstall, sztable.param().abi(), isequencemode) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiSequenceW<P1>(hinstall: super::msi::MSIHANDLE, sztable: P1, isequencemode: i32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSequenceW(hinstall : super::msi::MSIHANDLE, sztable : windows_core::PCWSTR, isequencemode : i32) -> u32);
    unsafe { MsiSequenceW(hinstall, sztable.param().abi(), isequencemode) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiSetComponentStateA<P1>(hinstall: super::msi::MSIHANDLE, szcomponent: P1, istate: super::msi::INSTALLSTATE) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetComponentStateA(hinstall : super::msi::MSIHANDLE, szcomponent : windows_core::PCSTR, istate : super::msi::INSTALLSTATE) -> u32);
    unsafe { MsiSetComponentStateA(hinstall, szcomponent.param().abi(), istate) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiSetComponentStateW<P1>(hinstall: super::msi::MSIHANDLE, szcomponent: P1, istate: super::msi::INSTALLSTATE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetComponentStateW(hinstall : super::msi::MSIHANDLE, szcomponent : windows_core::PCWSTR, istate : super::msi::INSTALLSTATE) -> u32);
    unsafe { MsiSetComponentStateW(hinstall, szcomponent.param().abi(), istate) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiSetFeatureAttributesA<P1>(hinstall: super::msi::MSIHANDLE, szfeature: P1, dwattributes: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetFeatureAttributesA(hinstall : super::msi::MSIHANDLE, szfeature : windows_core::PCSTR, dwattributes : u32) -> u32);
    unsafe { MsiSetFeatureAttributesA(hinstall, szfeature.param().abi(), dwattributes) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiSetFeatureAttributesW<P1>(hinstall: super::msi::MSIHANDLE, szfeature: P1, dwattributes: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetFeatureAttributesW(hinstall : super::msi::MSIHANDLE, szfeature : windows_core::PCWSTR, dwattributes : u32) -> u32);
    unsafe { MsiSetFeatureAttributesW(hinstall, szfeature.param().abi(), dwattributes) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiSetFeatureStateA<P1>(hinstall: super::msi::MSIHANDLE, szfeature: P1, istate: super::msi::INSTALLSTATE) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetFeatureStateA(hinstall : super::msi::MSIHANDLE, szfeature : windows_core::PCSTR, istate : super::msi::INSTALLSTATE) -> u32);
    unsafe { MsiSetFeatureStateA(hinstall, szfeature.param().abi(), istate) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiSetFeatureStateW<P1>(hinstall: super::msi::MSIHANDLE, szfeature: P1, istate: super::msi::INSTALLSTATE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetFeatureStateW(hinstall : super::msi::MSIHANDLE, szfeature : windows_core::PCWSTR, istate : super::msi::INSTALLSTATE) -> u32);
    unsafe { MsiSetFeatureStateW(hinstall, szfeature.param().abi(), istate) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiSetInstallLevel(hinstall: super::msi::MSIHANDLE, iinstalllevel: i32) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiSetInstallLevel(hinstall : super::msi::MSIHANDLE, iinstalllevel : i32) -> u32);
    unsafe { MsiSetInstallLevel(hinstall, iinstalllevel) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiSetMode(hinstall: super::msi::MSIHANDLE, erunmode: MSIRUNMODE, fstate: bool) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiSetMode(hinstall : super::msi::MSIHANDLE, erunmode : MSIRUNMODE, fstate : windows_core::BOOL) -> u32);
    unsafe { MsiSetMode(hinstall, erunmode, fstate.into()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiSetPropertyA<P1, P2>(hinstall: super::msi::MSIHANDLE, szname: P1, szvalue: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetPropertyA(hinstall : super::msi::MSIHANDLE, szname : windows_core::PCSTR, szvalue : windows_core::PCSTR) -> u32);
    unsafe { MsiSetPropertyA(hinstall, szname.param().abi(), szvalue.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiSetPropertyW<P1, P2>(hinstall: super::msi::MSIHANDLE, szname: P1, szvalue: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetPropertyW(hinstall : super::msi::MSIHANDLE, szname : windows_core::PCWSTR, szvalue : windows_core::PCWSTR) -> u32);
    unsafe { MsiSetPropertyW(hinstall, szname.param().abi(), szvalue.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiSetTargetPathA<P1, P2>(hinstall: super::msi::MSIHANDLE, szfolder: P1, szfolderpath: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetTargetPathA(hinstall : super::msi::MSIHANDLE, szfolder : windows_core::PCSTR, szfolderpath : windows_core::PCSTR) -> u32);
    unsafe { MsiSetTargetPathA(hinstall, szfolder.param().abi(), szfolderpath.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiSetTargetPathW<P1, P2>(hinstall: super::msi::MSIHANDLE, szfolder: P1, szfolderpath: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetTargetPathW(hinstall : super::msi::MSIHANDLE, szfolder : windows_core::PCWSTR, szfolderpath : windows_core::PCWSTR) -> u32);
    unsafe { MsiSetTargetPathW(hinstall, szfolder.param().abi(), szfolderpath.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "msi"))]
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyA(hsummaryinfo: super::msi::MSIHANDLE, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: Option<*mut super::minwindef::FILETIME>, szvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<*mut u32>) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoGetPropertyA(hsummaryinfo : super::msi::MSIHANDLE, uiproperty : u32, puidatatype : *mut u32, pivalue : *mut i32, pftvalue : *mut super::minwindef::FILETIME, szvaluebuf : windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiSummaryInfoGetPropertyA(hsummaryinfo, uiproperty, puidatatype as _, pivalue as _, pftvalue.unwrap_or(core::mem::zeroed()) as _, szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyCount(hsummaryinfo: super::msi::MSIHANDLE, puipropertycount: *mut u32) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoGetPropertyCount(hsummaryinfo : super::msi::MSIHANDLE, puipropertycount : *mut u32) -> u32);
    unsafe { MsiSummaryInfoGetPropertyCount(hsummaryinfo, puipropertycount as _) }
}
#[cfg(all(feature = "minwindef", feature = "msi"))]
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyW(hsummaryinfo: super::msi::MSIHANDLE, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: Option<*mut super::minwindef::FILETIME>, szvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<*mut u32>) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoGetPropertyW(hsummaryinfo : super::msi::MSIHANDLE, uiproperty : u32, puidatatype : *mut u32, pivalue : *mut i32, pftvalue : *mut super::minwindef::FILETIME, szvaluebuf : windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiSummaryInfoGetPropertyW(hsummaryinfo, uiproperty, puidatatype as _, pivalue as _, pftvalue.unwrap_or(core::mem::zeroed()) as _, szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiSummaryInfoPersist(hsummaryinfo: super::msi::MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoPersist(hsummaryinfo : super::msi::MSIHANDLE) -> u32);
    unsafe { MsiSummaryInfoPersist(hsummaryinfo) }
}
#[cfg(all(feature = "minwindef", feature = "msi"))]
#[inline]
pub unsafe fn MsiSummaryInfoSetPropertyA<P5>(hsummaryinfo: super::msi::MSIHANDLE, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut super::minwindef::FILETIME, szvalue: P5) -> u32
where
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoSetPropertyA(hsummaryinfo : super::msi::MSIHANDLE, uiproperty : u32, uidatatype : u32, ivalue : i32, pftvalue : *mut super::minwindef::FILETIME, szvalue : windows_core::PCSTR) -> u32);
    unsafe { MsiSummaryInfoSetPropertyA(hsummaryinfo, uiproperty, uidatatype, ivalue, pftvalue as _, szvalue.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "msi"))]
#[inline]
pub unsafe fn MsiSummaryInfoSetPropertyW<P5>(hsummaryinfo: super::msi::MSIHANDLE, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut super::minwindef::FILETIME, szvalue: P5) -> u32
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoSetPropertyW(hsummaryinfo : super::msi::MSIHANDLE, uiproperty : u32, uidatatype : u32, ivalue : i32, pftvalue : *mut super::minwindef::FILETIME, szvalue : windows_core::PCWSTR) -> u32);
    unsafe { MsiSummaryInfoSetPropertyW(hsummaryinfo, uiproperty, uidatatype, ivalue, pftvalue as _, szvalue.param().abi()) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiVerifyDiskSpace(hinstall: super::msi::MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiVerifyDiskSpace(hinstall : super::msi::MSIHANDLE) -> u32);
    unsafe { MsiVerifyDiskSpace(hinstall) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiViewClose(hview: super::msi::MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiViewClose(hview : super::msi::MSIHANDLE) -> u32);
    unsafe { MsiViewClose(hview) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiViewExecute(hview: super::msi::MSIHANDLE, hrecord: super::msi::MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiViewExecute(hview : super::msi::MSIHANDLE, hrecord : super::msi::MSIHANDLE) -> u32);
    unsafe { MsiViewExecute(hview, hrecord) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiViewFetch(hview: super::msi::MSIHANDLE, phrecord: *mut super::msi::MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiViewFetch(hview : super::msi::MSIHANDLE, phrecord : *mut super::msi::MSIHANDLE) -> u32);
    unsafe { MsiViewFetch(hview, phrecord as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiViewGetColumnInfo(hview: super::msi::MSIHANDLE, ecolumninfo: MSICOLINFO, phrecord: *mut super::msi::MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiViewGetColumnInfo(hview : super::msi::MSIHANDLE, ecolumninfo : MSICOLINFO, phrecord : *mut super::msi::MSIHANDLE) -> u32);
    unsafe { MsiViewGetColumnInfo(hview, ecolumninfo, phrecord as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiViewGetErrorA(hview: super::msi::MSIHANDLE, szcolumnnamebuffer: Option<windows_core::PSTR>, pcchbuf: Option<*mut u32>) -> MSIDBERROR {
    windows_core::link!("msi.dll" "system" fn MsiViewGetErrorA(hview : super::msi::MSIHANDLE, szcolumnnamebuffer : windows_core::PSTR, pcchbuf : *mut u32) -> MSIDBERROR);
    unsafe { MsiViewGetErrorA(hview, szcolumnnamebuffer.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiViewGetErrorW(hview: super::msi::MSIHANDLE, szcolumnnamebuffer: Option<windows_core::PWSTR>, pcchbuf: Option<*mut u32>) -> MSIDBERROR {
    windows_core::link!("msi.dll" "system" fn MsiViewGetErrorW(hview : super::msi::MSIHANDLE, szcolumnnamebuffer : windows_core::PWSTR, pcchbuf : *mut u32) -> MSIDBERROR);
    unsafe { MsiViewGetErrorW(hview, szcolumnnamebuffer.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiViewModify(hview: super::msi::MSIHANDLE, emodifymode: MSIMODIFY, hrecord: super::msi::MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiViewModify(hview : super::msi::MSIHANDLE, emodifymode : MSIMODIFY, hrecord : super::msi::MSIHANDLE) -> u32);
    unsafe { MsiViewModify(hview, emodifymode, hrecord) }
}
pub const INSTALLMESSAGE_TYPEMASK: u32 = 4278190080;
pub type MSICOLINFO = i32;
pub const MSICOLINFO_NAMES: MSICOLINFO = 0;
pub const MSICOLINFO_TYPES: MSICOLINFO = 1;
pub type MSICONDITION = i32;
pub const MSICONDITION_ERROR: MSICONDITION = 3;
pub const MSICONDITION_FALSE: MSICONDITION = 0;
pub const MSICONDITION_NONE: MSICONDITION = 2;
pub const MSICONDITION_TRUE: MSICONDITION = 1;
pub type MSICOSTTREE = i32;
pub const MSICOSTTREE_CHILDREN: MSICOSTTREE = 1;
pub const MSICOSTTREE_PARENTS: MSICOSTTREE = 2;
pub const MSICOSTTREE_RESERVED: MSICOSTTREE = 3;
pub const MSICOSTTREE_SELFONLY: MSICOSTTREE = 0;
pub type MSIDBERROR = i32;
pub const MSIDBERROR_BADCABINET: MSIDBERROR = 26;
pub const MSIDBERROR_BADCASE: MSIDBERROR = 8;
pub const MSIDBERROR_BADCATEGORY: MSIDBERROR = 23;
pub const MSIDBERROR_BADCONDITION: MSIDBERROR = 15;
pub const MSIDBERROR_BADCUSTOMSOURCE: MSIDBERROR = 20;
pub const MSIDBERROR_BADDEFAULTDIR: MSIDBERROR = 18;
pub const MSIDBERROR_BADFILENAME: MSIDBERROR = 13;
pub const MSIDBERROR_BADFORMATTED: MSIDBERROR = 16;
pub const MSIDBERROR_BADGUID: MSIDBERROR = 9;
pub const MSIDBERROR_BADIDENTIFIER: MSIDBERROR = 11;
pub const MSIDBERROR_BADKEYTABLE: MSIDBERROR = 24;
pub const MSIDBERROR_BADLANGUAGE: MSIDBERROR = 12;
pub const MSIDBERROR_BADLINK: MSIDBERROR = 3;
pub const MSIDBERROR_BADLOCALIZEATTRIB: MSIDBERROR = 29;
pub const MSIDBERROR_BADMAXMINVALUES: MSIDBERROR = 25;
pub const MSIDBERROR_BADPATH: MSIDBERROR = 14;
pub const MSIDBERROR_BADPROPERTY: MSIDBERROR = 21;
pub const MSIDBERROR_BADREGPATH: MSIDBERROR = 19;
pub const MSIDBERROR_BADSHORTCUT: MSIDBERROR = 27;
pub const MSIDBERROR_BADTEMPLATE: MSIDBERROR = 17;
pub const MSIDBERROR_BADVERSION: MSIDBERROR = 7;
pub const MSIDBERROR_BADWILDCARD: MSIDBERROR = 10;
pub const MSIDBERROR_DUPLICATEKEY: MSIDBERROR = 1;
pub const MSIDBERROR_FUNCTIONERROR: MSIDBERROR = -1;
pub const MSIDBERROR_INVALIDARG: MSIDBERROR = -3;
pub const MSIDBERROR_MISSINGDATA: MSIDBERROR = 22;
pub const MSIDBERROR_MOREDATA: MSIDBERROR = -2;
pub const MSIDBERROR_NOERROR: MSIDBERROR = 0;
pub const MSIDBERROR_NOTINSET: MSIDBERROR = 6;
pub const MSIDBERROR_OVERFLOW: MSIDBERROR = 4;
pub const MSIDBERROR_REQUIRED: MSIDBERROR = 2;
pub const MSIDBERROR_STRINGOVERFLOW: MSIDBERROR = 28;
pub const MSIDBERROR_UNDERFLOW: MSIDBERROR = 5;
#[cfg(feature = "winnt")]
pub const MSIDBOPEN_CREATE: super::winnt::LPCTSTR = windows_core::PCSTR(3 as _);
#[cfg(feature = "winnt")]
pub const MSIDBOPEN_CREATEDIRECT: super::winnt::LPCTSTR = windows_core::PCSTR(4 as _);
#[cfg(feature = "winnt")]
pub const MSIDBOPEN_DIRECT: super::winnt::LPCTSTR = windows_core::PCSTR(2 as _);
#[cfg(feature = "winnt")]
pub const MSIDBOPEN_READONLY: super::winnt::LPCTSTR = windows_core::PCSTR(0 as _);
#[cfg(feature = "winnt")]
pub const MSIDBOPEN_TRANSACT: super::winnt::LPCTSTR = windows_core::PCSTR(1 as _);
pub type MSIDBSTATE = i32;
pub const MSIDBSTATE_ERROR: MSIDBSTATE = -1;
pub const MSIDBSTATE_READ: MSIDBSTATE = 0;
pub const MSIDBSTATE_WRITE: MSIDBSTATE = 1;
pub type MSIMODIFY = i32;
pub const MSIMODIFY_ASSIGN: MSIMODIFY = 3;
pub const MSIMODIFY_DELETE: MSIMODIFY = 6;
pub const MSIMODIFY_INSERT: MSIMODIFY = 1;
pub const MSIMODIFY_INSERT_TEMPORARY: MSIMODIFY = 7;
pub const MSIMODIFY_MERGE: MSIMODIFY = 5;
pub const MSIMODIFY_REFRESH: MSIMODIFY = 0;
pub const MSIMODIFY_REPLACE: MSIMODIFY = 4;
pub const MSIMODIFY_SEEK: MSIMODIFY = -1;
pub const MSIMODIFY_UPDATE: MSIMODIFY = 2;
pub const MSIMODIFY_VALIDATE: MSIMODIFY = 8;
pub const MSIMODIFY_VALIDATE_DELETE: MSIMODIFY = 11;
pub const MSIMODIFY_VALIDATE_FIELD: MSIMODIFY = 10;
pub const MSIMODIFY_VALIDATE_NEW: MSIMODIFY = 9;
pub type MSIRUNMODE = i32;
pub const MSIRUNMODE_ADMIN: MSIRUNMODE = 0;
pub const MSIRUNMODE_ADVERTISE: MSIRUNMODE = 1;
pub const MSIRUNMODE_CABINET: MSIRUNMODE = 8;
pub const MSIRUNMODE_COMMIT: MSIRUNMODE = 18;
pub const MSIRUNMODE_LOGENABLED: MSIRUNMODE = 4;
pub const MSIRUNMODE_MAINTENANCE: MSIRUNMODE = 2;
pub const MSIRUNMODE_OPERATIONS: MSIRUNMODE = 5;
pub const MSIRUNMODE_REBOOTATEND: MSIRUNMODE = 6;
pub const MSIRUNMODE_REBOOTNOW: MSIRUNMODE = 7;
pub const MSIRUNMODE_RESERVED11: MSIRUNMODE = 11;
pub const MSIRUNMODE_RESERVED14: MSIRUNMODE = 14;
pub const MSIRUNMODE_RESERVED15: MSIRUNMODE = 15;
pub const MSIRUNMODE_ROLLBACK: MSIRUNMODE = 17;
pub const MSIRUNMODE_ROLLBACKENABLED: MSIRUNMODE = 3;
pub const MSIRUNMODE_SCHEDULED: MSIRUNMODE = 16;
pub const MSIRUNMODE_SOURCESHORTNAMES: MSIRUNMODE = 9;
pub const MSIRUNMODE_TARGETSHORTNAMES: MSIRUNMODE = 10;
pub const MSIRUNMODE_WINDOWS9X: MSIRUNMODE = 12;
pub const MSIRUNMODE_ZAWENABLED: MSIRUNMODE = 13;
pub type MSITRANSFORM_ERROR = i32;
pub const MSITRANSFORM_ERROR_ADDEXISTINGROW: MSITRANSFORM_ERROR = 1;
pub const MSITRANSFORM_ERROR_ADDEXISTINGTABLE: MSITRANSFORM_ERROR = 4;
pub const MSITRANSFORM_ERROR_CHANGECODEPAGE: MSITRANSFORM_ERROR = 32;
pub const MSITRANSFORM_ERROR_DELMISSINGROW: MSITRANSFORM_ERROR = 2;
pub const MSITRANSFORM_ERROR_DELMISSINGTABLE: MSITRANSFORM_ERROR = 8;
pub const MSITRANSFORM_ERROR_UPDATEMISSINGROW: MSITRANSFORM_ERROR = 16;
pub const MSITRANSFORM_ERROR_VIEWTRANSFORM: MSITRANSFORM_ERROR = 256;
pub type MSITRANSFORM_VALIDATE = i32;
pub const MSITRANSFORM_VALIDATE_LANGUAGE: MSITRANSFORM_VALIDATE = 1;
pub const MSITRANSFORM_VALIDATE_MAJORVERSION: MSITRANSFORM_VALIDATE = 8;
pub const MSITRANSFORM_VALIDATE_MINORVERSION: MSITRANSFORM_VALIDATE = 16;
pub const MSITRANSFORM_VALIDATE_NEWEQUALBASEVERSION: MSITRANSFORM_VALIDATE = 256;
pub const MSITRANSFORM_VALIDATE_NEWGREATERBASEVERSION: MSITRANSFORM_VALIDATE = 1024;
pub const MSITRANSFORM_VALIDATE_NEWGREATEREQUALBASEVERSION: MSITRANSFORM_VALIDATE = 512;
pub const MSITRANSFORM_VALIDATE_NEWLESSBASEVERSION: MSITRANSFORM_VALIDATE = 64;
pub const MSITRANSFORM_VALIDATE_NEWLESSEQUALBASEVERSION: MSITRANSFORM_VALIDATE = 128;
pub const MSITRANSFORM_VALIDATE_PLATFORM: MSITRANSFORM_VALIDATE = 4;
pub const MSITRANSFORM_VALIDATE_PRODUCT: MSITRANSFORM_VALIDATE = 2;
pub const MSITRANSFORM_VALIDATE_UPDATEVERSION: MSITRANSFORM_VALIDATE = 32;
pub const MSITRANSFORM_VALIDATE_UPGRADECODE: MSITRANSFORM_VALIDATE = 2048;
pub const MSI_NULL_INTEGER: u32 = 2147483648;
