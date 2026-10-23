#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiCreateRecord(cparams: super::UINT) -> super::MSIHANDLE {
    windows_core::link!("msi.dll" "system" fn MsiCreateRecord(cparams : super::UINT) -> super::MSIHANDLE);
    unsafe { MsiCreateRecord(cparams) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiCreateTransformSummaryInfoA(hdatabase: super::MSIHANDLE, hdatabasereference: super::MSIHANDLE, sztransformfile: &windows_core::PCSTR, ierrorconditions: i32, ivalidation: i32) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiCreateTransformSummaryInfoA(hdatabase : super::MSIHANDLE, hdatabasereference : super::MSIHANDLE, sztransformfile : windows_core::PCSTR, ierrorconditions : i32, ivalidation : i32) -> super::UINT);
    unsafe { MsiCreateTransformSummaryInfoA(hdatabase, hdatabasereference, core::mem::transmute_copy(sztransformfile), ierrorconditions, ivalidation) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiCreateTransformSummaryInfoW(hdatabase: super::MSIHANDLE, hdatabasereference: super::MSIHANDLE, sztransformfile: &windows_core::PCWSTR, ierrorconditions: i32, ivalidation: i32) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiCreateTransformSummaryInfoW(hdatabase : super::MSIHANDLE, hdatabasereference : super::MSIHANDLE, sztransformfile : windows_core::PCWSTR, ierrorconditions : i32, ivalidation : i32) -> super::UINT);
    unsafe { MsiCreateTransformSummaryInfoW(hdatabase, hdatabasereference, core::mem::transmute_copy(sztransformfile), ierrorconditions, ivalidation) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiDatabaseApplyTransformA(hdatabase: super::MSIHANDLE, sztransformfile: &windows_core::PCSTR, ierrorconditions: i32) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseApplyTransformA(hdatabase : super::MSIHANDLE, sztransformfile : windows_core::PCSTR, ierrorconditions : i32) -> super::UINT);
    unsafe { MsiDatabaseApplyTransformA(hdatabase, core::mem::transmute_copy(sztransformfile), ierrorconditions) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiDatabaseApplyTransformW(hdatabase: super::MSIHANDLE, sztransformfile: &windows_core::PCWSTR, ierrorconditions: i32) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseApplyTransformW(hdatabase : super::MSIHANDLE, sztransformfile : windows_core::PCWSTR, ierrorconditions : i32) -> super::UINT);
    unsafe { MsiDatabaseApplyTransformW(hdatabase, core::mem::transmute_copy(sztransformfile), ierrorconditions) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiDatabaseCommit(hdatabase: super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseCommit(hdatabase : super::MSIHANDLE) -> super::UINT);
    unsafe { MsiDatabaseCommit(hdatabase) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiDatabaseExportA(hdatabase: super::MSIHANDLE, sztablename: &windows_core::PCSTR, szfolderpath: &windows_core::PCSTR, szfilename: &windows_core::PCSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseExportA(hdatabase : super::MSIHANDLE, sztablename : windows_core::PCSTR, szfolderpath : windows_core::PCSTR, szfilename : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiDatabaseExportA(hdatabase, core::mem::transmute_copy(sztablename), core::mem::transmute_copy(szfolderpath), core::mem::transmute_copy(szfilename)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiDatabaseExportW(hdatabase: super::MSIHANDLE, sztablename: &windows_core::PCWSTR, szfolderpath: &windows_core::PCWSTR, szfilename: &windows_core::PCWSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseExportW(hdatabase : super::MSIHANDLE, sztablename : windows_core::PCWSTR, szfolderpath : windows_core::PCWSTR, szfilename : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiDatabaseExportW(hdatabase, core::mem::transmute_copy(sztablename), core::mem::transmute_copy(szfolderpath), core::mem::transmute_copy(szfilename)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiDatabaseGenerateTransformA(hdatabase: super::MSIHANDLE, hdatabasereference: super::MSIHANDLE, sztransformfile: &windows_core::PCSTR, ireserved1: i32, ireserved2: i32) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseGenerateTransformA(hdatabase : super::MSIHANDLE, hdatabasereference : super::MSIHANDLE, sztransformfile : windows_core::PCSTR, ireserved1 : i32, ireserved2 : i32) -> super::UINT);
    unsafe { MsiDatabaseGenerateTransformA(hdatabase, hdatabasereference, core::mem::transmute_copy(sztransformfile), ireserved1, ireserved2) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiDatabaseGenerateTransformW(hdatabase: super::MSIHANDLE, hdatabasereference: super::MSIHANDLE, sztransformfile: &windows_core::PCWSTR, ireserved1: i32, ireserved2: i32) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseGenerateTransformW(hdatabase : super::MSIHANDLE, hdatabasereference : super::MSIHANDLE, sztransformfile : windows_core::PCWSTR, ireserved1 : i32, ireserved2 : i32) -> super::UINT);
    unsafe { MsiDatabaseGenerateTransformW(hdatabase, hdatabasereference, core::mem::transmute_copy(sztransformfile), ireserved1, ireserved2) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiDatabaseGetPrimaryKeysA(hdatabase: super::MSIHANDLE, sztablename: &windows_core::PCSTR, phrecord: *mut super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseGetPrimaryKeysA(hdatabase : super::MSIHANDLE, sztablename : windows_core::PCSTR, phrecord : *mut super::MSIHANDLE) -> super::UINT);
    unsafe { MsiDatabaseGetPrimaryKeysA(hdatabase, core::mem::transmute_copy(sztablename), phrecord as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiDatabaseGetPrimaryKeysW(hdatabase: super::MSIHANDLE, sztablename: &windows_core::PCWSTR, phrecord: *mut super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseGetPrimaryKeysW(hdatabase : super::MSIHANDLE, sztablename : windows_core::PCWSTR, phrecord : *mut super::MSIHANDLE) -> super::UINT);
    unsafe { MsiDatabaseGetPrimaryKeysW(hdatabase, core::mem::transmute_copy(sztablename), phrecord as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiDatabaseImportA(hdatabase: super::MSIHANDLE, szfolderpath: &windows_core::PCSTR, szfilename: &windows_core::PCSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseImportA(hdatabase : super::MSIHANDLE, szfolderpath : windows_core::PCSTR, szfilename : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiDatabaseImportA(hdatabase, core::mem::transmute_copy(szfolderpath), core::mem::transmute_copy(szfilename)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiDatabaseImportW(hdatabase: super::MSIHANDLE, szfolderpath: &windows_core::PCWSTR, szfilename: &windows_core::PCWSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseImportW(hdatabase : super::MSIHANDLE, szfolderpath : windows_core::PCWSTR, szfilename : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiDatabaseImportW(hdatabase, core::mem::transmute_copy(szfolderpath), core::mem::transmute_copy(szfilename)) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseIsTablePersistentA(hdatabase: super::MSIHANDLE, sztablename: &windows_core::PCSTR) -> MSICONDITION {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseIsTablePersistentA(hdatabase : super::MSIHANDLE, sztablename : windows_core::PCSTR) -> MSICONDITION);
    unsafe { MsiDatabaseIsTablePersistentA(hdatabase, core::mem::transmute_copy(sztablename)) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiDatabaseIsTablePersistentW(hdatabase: super::MSIHANDLE, sztablename: &windows_core::PCWSTR) -> MSICONDITION {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseIsTablePersistentW(hdatabase : super::MSIHANDLE, sztablename : windows_core::PCWSTR) -> MSICONDITION);
    unsafe { MsiDatabaseIsTablePersistentW(hdatabase, core::mem::transmute_copy(sztablename)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiDatabaseMergeA(hdatabase: super::MSIHANDLE, hdatabasemerge: super::MSIHANDLE, sztablename: &windows_core::PCSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseMergeA(hdatabase : super::MSIHANDLE, hdatabasemerge : super::MSIHANDLE, sztablename : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiDatabaseMergeA(hdatabase, hdatabasemerge, core::mem::transmute_copy(sztablename)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiDatabaseMergeW(hdatabase: super::MSIHANDLE, hdatabasemerge: super::MSIHANDLE, sztablename: &windows_core::PCWSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseMergeW(hdatabase : super::MSIHANDLE, hdatabasemerge : super::MSIHANDLE, sztablename : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiDatabaseMergeW(hdatabase, hdatabasemerge, core::mem::transmute_copy(sztablename)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiDatabaseOpenViewA(hdatabase: super::MSIHANDLE, szquery: &windows_core::PCSTR, phview: *mut super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseOpenViewA(hdatabase : super::MSIHANDLE, szquery : windows_core::PCSTR, phview : *mut super::MSIHANDLE) -> super::UINT);
    unsafe { MsiDatabaseOpenViewA(hdatabase, core::mem::transmute_copy(szquery), phview as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiDatabaseOpenViewW(hdatabase: super::MSIHANDLE, szquery: &windows_core::PCWSTR, phview: *mut super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseOpenViewW(hdatabase : super::MSIHANDLE, szquery : windows_core::PCWSTR, phview : *mut super::MSIHANDLE) -> super::UINT);
    unsafe { MsiDatabaseOpenViewW(hdatabase, core::mem::transmute_copy(szquery), phview as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiDoActionA(hinstall: super::MSIHANDLE, szaction: &windows_core::PCSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiDoActionA(hinstall : super::MSIHANDLE, szaction : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiDoActionA(hinstall, core::mem::transmute_copy(szaction)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiDoActionW(hinstall: super::MSIHANDLE, szaction: &windows_core::PCWSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiDoActionW(hinstall : super::MSIHANDLE, szaction : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiDoActionW(hinstall, core::mem::transmute_copy(szaction)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiEnableUIPreview(hdatabase: super::MSIHANDLE, phpreview: *mut super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiEnableUIPreview(hdatabase : super::MSIHANDLE, phpreview : *mut super::MSIHANDLE) -> super::UINT);
    unsafe { MsiEnableUIPreview(hdatabase, phpreview as _) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiEnumComponentCostsA(hinstall: super::MSIHANDLE, szcomponent: &windows_core::PCSTR, dwindex: super::DWORD, istate: super::INSTALLSTATE, szdrivebuf: windows_core::PSTR, pcchdrivebuf: super::LPDWORD, picost: super::LPINT, pitempcost: super::LPINT) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentCostsA(hinstall : super::MSIHANDLE, szcomponent : windows_core::PCSTR, dwindex : super::DWORD, istate : super::INSTALLSTATE, szdrivebuf : windows_core::PSTR, pcchdrivebuf : super::LPDWORD, picost : super::LPINT, pitempcost : super::LPINT) -> super::UINT);
    unsafe { MsiEnumComponentCostsA(hinstall, core::mem::transmute_copy(szcomponent), dwindex, istate, szdrivebuf, pcchdrivebuf as _, picost as _, pitempcost as _) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiEnumComponentCostsW(hinstall: super::MSIHANDLE, szcomponent: &windows_core::PCWSTR, dwindex: super::DWORD, istate: super::INSTALLSTATE, szdrivebuf: windows_core::PWSTR, pcchdrivebuf: super::LPDWORD, picost: super::LPINT, pitempcost: super::LPINT) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentCostsW(hinstall : super::MSIHANDLE, szcomponent : windows_core::PCWSTR, dwindex : super::DWORD, istate : super::INSTALLSTATE, szdrivebuf : windows_core::PWSTR, pcchdrivebuf : super::LPDWORD, picost : super::LPINT, pitempcost : super::LPINT) -> super::UINT);
    unsafe { MsiEnumComponentCostsW(hinstall, core::mem::transmute_copy(szcomponent), dwindex, istate, szdrivebuf, pcchdrivebuf as _, picost as _, pitempcost as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiEvaluateConditionA(hinstall: super::MSIHANDLE, szcondition: &windows_core::PCSTR) -> MSICONDITION {
    windows_core::link!("msi.dll" "system" fn MsiEvaluateConditionA(hinstall : super::MSIHANDLE, szcondition : windows_core::PCSTR) -> MSICONDITION);
    unsafe { MsiEvaluateConditionA(hinstall, core::mem::transmute_copy(szcondition)) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiEvaluateConditionW(hinstall: super::MSIHANDLE, szcondition: &windows_core::PCWSTR) -> MSICONDITION {
    windows_core::link!("msi.dll" "system" fn MsiEvaluateConditionW(hinstall : super::MSIHANDLE, szcondition : windows_core::PCWSTR) -> MSICONDITION);
    unsafe { MsiEvaluateConditionW(hinstall, core::mem::transmute_copy(szcondition)) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiFormatRecordA(hinstall: super::MSIHANDLE, hrecord: super::MSIHANDLE, szresultbuf: Option<windows_core::PSTR>, pcchresultbuf: Option<super::LPDWORD>) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiFormatRecordA(hinstall : super::MSIHANDLE, hrecord : super::MSIHANDLE, szresultbuf : windows_core::PSTR, pcchresultbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiFormatRecordA(hinstall, hrecord, szresultbuf.unwrap_or(core::mem::zeroed()) as _, pcchresultbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiFormatRecordW(hinstall: super::MSIHANDLE, hrecord: super::MSIHANDLE, szresultbuf: Option<windows_core::PWSTR>, pcchresultbuf: Option<super::LPDWORD>) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiFormatRecordW(hinstall : super::MSIHANDLE, hrecord : super::MSIHANDLE, szresultbuf : windows_core::PWSTR, pcchresultbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiFormatRecordW(hinstall, hrecord, szresultbuf.unwrap_or(core::mem::zeroed()) as _, pcchresultbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetActiveDatabase(hinstall: super::MSIHANDLE) -> super::MSIHANDLE {
    windows_core::link!("msi.dll" "system" fn MsiGetActiveDatabase(hinstall : super::MSIHANDLE) -> super::MSIHANDLE);
    unsafe { MsiGetActiveDatabase(hinstall) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetComponentStateA(hinstall: super::MSIHANDLE, szcomponent: &windows_core::PCSTR, piinstalled: *mut super::INSTALLSTATE, piaction: *mut super::INSTALLSTATE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiGetComponentStateA(hinstall : super::MSIHANDLE, szcomponent : windows_core::PCSTR, piinstalled : *mut super::INSTALLSTATE, piaction : *mut super::INSTALLSTATE) -> super::UINT);
    unsafe { MsiGetComponentStateA(hinstall, core::mem::transmute_copy(szcomponent), piinstalled as _, piaction as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetComponentStateW(hinstall: super::MSIHANDLE, szcomponent: &windows_core::PCWSTR, piinstalled: *mut super::INSTALLSTATE, piaction: *mut super::INSTALLSTATE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiGetComponentStateW(hinstall : super::MSIHANDLE, szcomponent : windows_core::PCWSTR, piinstalled : *mut super::INSTALLSTATE, piaction : *mut super::INSTALLSTATE) -> super::UINT);
    unsafe { MsiGetComponentStateW(hinstall, core::mem::transmute_copy(szcomponent), piinstalled as _, piaction as _) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetDatabaseState(hdatabase: super::MSIHANDLE) -> MSIDBSTATE {
    windows_core::link!("msi.dll" "system" fn MsiGetDatabaseState(hdatabase : super::MSIHANDLE) -> MSIDBSTATE);
    unsafe { MsiGetDatabaseState(hdatabase) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetFeatureCostA(hinstall: super::MSIHANDLE, szfeature: &windows_core::PCSTR, icosttree: MSICOSTTREE, istate: super::INSTALLSTATE, picost: super::LPINT) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureCostA(hinstall : super::MSIHANDLE, szfeature : windows_core::PCSTR, icosttree : MSICOSTTREE, istate : super::INSTALLSTATE, picost : super::LPINT) -> super::UINT);
    unsafe { MsiGetFeatureCostA(hinstall, core::mem::transmute_copy(szfeature), icosttree, istate, picost as _) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetFeatureCostW(hinstall: super::MSIHANDLE, szfeature: &windows_core::PCWSTR, icosttree: MSICOSTTREE, istate: super::INSTALLSTATE, picost: super::LPINT) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureCostW(hinstall : super::MSIHANDLE, szfeature : windows_core::PCWSTR, icosttree : MSICOSTTREE, istate : super::INSTALLSTATE, picost : super::LPINT) -> super::UINT);
    unsafe { MsiGetFeatureCostW(hinstall, core::mem::transmute_copy(szfeature), icosttree, istate, picost as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetFeatureStateA(hinstall: super::MSIHANDLE, szfeature: &windows_core::PCSTR, piinstalled: *mut super::INSTALLSTATE, piaction: *mut super::INSTALLSTATE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureStateA(hinstall : super::MSIHANDLE, szfeature : windows_core::PCSTR, piinstalled : *mut super::INSTALLSTATE, piaction : *mut super::INSTALLSTATE) -> super::UINT);
    unsafe { MsiGetFeatureStateA(hinstall, core::mem::transmute_copy(szfeature), piinstalled as _, piaction as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetFeatureStateW(hinstall: super::MSIHANDLE, szfeature: &windows_core::PCWSTR, piinstalled: *mut super::INSTALLSTATE, piaction: *mut super::INSTALLSTATE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureStateW(hinstall : super::MSIHANDLE, szfeature : windows_core::PCWSTR, piinstalled : *mut super::INSTALLSTATE, piaction : *mut super::INSTALLSTATE) -> super::UINT);
    unsafe { MsiGetFeatureStateW(hinstall, core::mem::transmute_copy(szfeature), piinstalled as _, piaction as _) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetFeatureValidStatesA(hinstall: super::MSIHANDLE, szfeature: &windows_core::PCSTR, lpinstallstates: super::LPDWORD) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureValidStatesA(hinstall : super::MSIHANDLE, szfeature : windows_core::PCSTR, lpinstallstates : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetFeatureValidStatesA(hinstall, core::mem::transmute_copy(szfeature), lpinstallstates as _) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetFeatureValidStatesW(hinstall: super::MSIHANDLE, szfeature: &windows_core::PCWSTR, lpinstallstates: super::LPDWORD) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureValidStatesW(hinstall : super::MSIHANDLE, szfeature : windows_core::PCWSTR, lpinstallstates : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetFeatureValidStatesW(hinstall, core::mem::transmute_copy(szfeature), lpinstallstates as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetLanguage(hinstall: super::MSIHANDLE) -> super::LANGID {
    windows_core::link!("msi.dll" "system" fn MsiGetLanguage(hinstall : super::MSIHANDLE) -> super::LANGID);
    unsafe { MsiGetLanguage(hinstall) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetLastErrorRecord() -> super::MSIHANDLE {
    windows_core::link!("msi.dll" "system" fn MsiGetLastErrorRecord() -> super::MSIHANDLE);
    unsafe { MsiGetLastErrorRecord() }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiGetMode(hinstall: super::MSIHANDLE, erunmode: MSIRUNMODE) -> windows_core::BOOL {
    windows_core::link!("msi.dll" "system" fn MsiGetMode(hinstall : super::MSIHANDLE, erunmode : MSIRUNMODE) -> windows_core::BOOL);
    unsafe { MsiGetMode(hinstall, erunmode) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetPropertyA(hinstall: super::MSIHANDLE, szname: &windows_core::PCSTR, szvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<super::LPDWORD>) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiGetPropertyA(hinstall : super::MSIHANDLE, szname : windows_core::PCSTR, szvaluebuf : windows_core::PSTR, pcchvaluebuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetPropertyA(hinstall, core::mem::transmute_copy(szname), szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetPropertyW(hinstall: super::MSIHANDLE, szname: &windows_core::PCWSTR, szvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<super::LPDWORD>) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiGetPropertyW(hinstall : super::MSIHANDLE, szname : windows_core::PCWSTR, szvaluebuf : windows_core::PWSTR, pcchvaluebuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetPropertyW(hinstall, core::mem::transmute_copy(szname), szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetSourcePathA(hinstall: super::MSIHANDLE, szfolder: &windows_core::PCSTR, szpathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<super::LPDWORD>) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiGetSourcePathA(hinstall : super::MSIHANDLE, szfolder : windows_core::PCSTR, szpathbuf : windows_core::PSTR, pcchpathbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetSourcePathA(hinstall, core::mem::transmute_copy(szfolder), szpathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetSourcePathW(hinstall: super::MSIHANDLE, szfolder: &windows_core::PCWSTR, szpathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<super::LPDWORD>) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiGetSourcePathW(hinstall : super::MSIHANDLE, szfolder : windows_core::PCWSTR, szpathbuf : windows_core::PWSTR, pcchpathbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetSourcePathW(hinstall, core::mem::transmute_copy(szfolder), szpathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetSummaryInformationA(hdatabase: super::MSIHANDLE, szdatabasepath: &windows_core::PCSTR, uiupdatecount: super::UINT, phsummaryinfo: *mut super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiGetSummaryInformationA(hdatabase : super::MSIHANDLE, szdatabasepath : windows_core::PCSTR, uiupdatecount : super::UINT, phsummaryinfo : *mut super::MSIHANDLE) -> super::UINT);
    unsafe { MsiGetSummaryInformationA(hdatabase, core::mem::transmute_copy(szdatabasepath), uiupdatecount, phsummaryinfo as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetSummaryInformationW(hdatabase: super::MSIHANDLE, szdatabasepath: &windows_core::PCWSTR, uiupdatecount: super::UINT, phsummaryinfo: *mut super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiGetSummaryInformationW(hdatabase : super::MSIHANDLE, szdatabasepath : windows_core::PCWSTR, uiupdatecount : super::UINT, phsummaryinfo : *mut super::MSIHANDLE) -> super::UINT);
    unsafe { MsiGetSummaryInformationW(hdatabase, core::mem::transmute_copy(szdatabasepath), uiupdatecount, phsummaryinfo as _) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetTargetPathA(hinstall: super::MSIHANDLE, szfolder: &windows_core::PCSTR, szpathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<super::LPDWORD>) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiGetTargetPathA(hinstall : super::MSIHANDLE, szfolder : windows_core::PCSTR, szpathbuf : windows_core::PSTR, pcchpathbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetTargetPathA(hinstall, core::mem::transmute_copy(szfolder), szpathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetTargetPathW(hinstall: super::MSIHANDLE, szfolder: &windows_core::PCWSTR, szpathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<super::LPDWORD>) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiGetTargetPathW(hinstall : super::MSIHANDLE, szfolder : windows_core::PCWSTR, szpathbuf : windows_core::PWSTR, pcchpathbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetTargetPathW(hinstall, core::mem::transmute_copy(szfolder), szpathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiOpenDatabaseA(szdatabasepath: &windows_core::PCSTR, szpersist: &windows_core::PCSTR, phdatabase: *mut super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiOpenDatabaseA(szdatabasepath : windows_core::PCSTR, szpersist : windows_core::PCSTR, phdatabase : *mut super::MSIHANDLE) -> super::UINT);
    unsafe { MsiOpenDatabaseA(core::mem::transmute_copy(szdatabasepath), core::mem::transmute_copy(szpersist), phdatabase as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiOpenDatabaseW(szdatabasepath: &windows_core::PCWSTR, szpersist: &windows_core::PCWSTR, phdatabase: *mut super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiOpenDatabaseW(szdatabasepath : windows_core::PCWSTR, szpersist : windows_core::PCWSTR, phdatabase : *mut super::MSIHANDLE) -> super::UINT);
    unsafe { MsiOpenDatabaseW(core::mem::transmute_copy(szdatabasepath), core::mem::transmute_copy(szpersist), phdatabase as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiPreviewBillboardA(hpreview: super::MSIHANDLE, szcontrolname: &windows_core::PCSTR, szbillboard: &windows_core::PCSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiPreviewBillboardA(hpreview : super::MSIHANDLE, szcontrolname : windows_core::PCSTR, szbillboard : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiPreviewBillboardA(hpreview, core::mem::transmute_copy(szcontrolname), core::mem::transmute_copy(szbillboard)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiPreviewBillboardW(hpreview: super::MSIHANDLE, szcontrolname: &windows_core::PCWSTR, szbillboard: &windows_core::PCWSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiPreviewBillboardW(hpreview : super::MSIHANDLE, szcontrolname : windows_core::PCWSTR, szbillboard : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiPreviewBillboardW(hpreview, core::mem::transmute_copy(szcontrolname), core::mem::transmute_copy(szbillboard)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiPreviewDialogA(hpreview: super::MSIHANDLE, szdialogname: &windows_core::PCSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiPreviewDialogA(hpreview : super::MSIHANDLE, szdialogname : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiPreviewDialogA(hpreview, core::mem::transmute_copy(szdialogname)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiPreviewDialogW(hpreview: super::MSIHANDLE, szdialogname: &windows_core::PCWSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiPreviewDialogW(hpreview : super::MSIHANDLE, szdialogname : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiPreviewDialogW(hpreview, core::mem::transmute_copy(szdialogname)) }
}
#[cfg(feature = "msi")]
#[inline]
pub unsafe fn MsiProcessMessage(hinstall: super::MSIHANDLE, emessagetype: super::INSTALLMESSAGE, hrecord: super::MSIHANDLE) -> i32 {
    windows_core::link!("msi.dll" "system" fn MsiProcessMessage(hinstall : super::MSIHANDLE, emessagetype : super::INSTALLMESSAGE, hrecord : super::MSIHANDLE) -> i32);
    unsafe { MsiProcessMessage(hinstall, emessagetype, hrecord) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiRecordClearData(hrecord: super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiRecordClearData(hrecord : super::MSIHANDLE) -> super::UINT);
    unsafe { MsiRecordClearData(hrecord) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiRecordDataSize(hrecord: super::MSIHANDLE, ifield: super::UINT) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiRecordDataSize(hrecord : super::MSIHANDLE, ifield : super::UINT) -> super::UINT);
    unsafe { MsiRecordDataSize(hrecord, ifield) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiRecordGetFieldCount(hrecord: super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiRecordGetFieldCount(hrecord : super::MSIHANDLE) -> super::UINT);
    unsafe { MsiRecordGetFieldCount(hrecord) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiRecordGetInteger(hrecord: super::MSIHANDLE, ifield: super::UINT) -> i32 {
    windows_core::link!("msi.dll" "system" fn MsiRecordGetInteger(hrecord : super::MSIHANDLE, ifield : super::UINT) -> i32);
    unsafe { MsiRecordGetInteger(hrecord, ifield) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiRecordGetStringA(hrecord: super::MSIHANDLE, ifield: super::UINT, szvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<super::LPDWORD>) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiRecordGetStringA(hrecord : super::MSIHANDLE, ifield : super::UINT, szvaluebuf : windows_core::PSTR, pcchvaluebuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiRecordGetStringA(hrecord, ifield, szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiRecordGetStringW(hrecord: super::MSIHANDLE, ifield: super::UINT, szvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<super::LPDWORD>) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiRecordGetStringW(hrecord : super::MSIHANDLE, ifield : super::UINT, szvaluebuf : windows_core::PWSTR, pcchvaluebuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiRecordGetStringW(hrecord, ifield, szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiRecordIsNull(hrecord: super::MSIHANDLE, ifield: super::UINT) -> windows_core::BOOL {
    windows_core::link!("msi.dll" "system" fn MsiRecordIsNull(hrecord : super::MSIHANDLE, ifield : super::UINT) -> windows_core::BOOL);
    unsafe { MsiRecordIsNull(hrecord, ifield) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiRecordReadStream(hrecord: super::MSIHANDLE, ifield: super::UINT, szdatabuf: Option<*mut i8>, pcbdatabuf: super::LPDWORD) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiRecordReadStream(hrecord : super::MSIHANDLE, ifield : super::UINT, szdatabuf : *mut i8, pcbdatabuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiRecordReadStream(hrecord, ifield, szdatabuf.unwrap_or(core::mem::zeroed()) as _, pcbdatabuf as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiRecordSetInteger(hrecord: super::MSIHANDLE, ifield: super::UINT, ivalue: i32) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiRecordSetInteger(hrecord : super::MSIHANDLE, ifield : super::UINT, ivalue : i32) -> super::UINT);
    unsafe { MsiRecordSetInteger(hrecord, ifield, ivalue) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiRecordSetStreamA(hrecord: super::MSIHANDLE, ifield: super::UINT, szfilepath: &windows_core::PCSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiRecordSetStreamA(hrecord : super::MSIHANDLE, ifield : super::UINT, szfilepath : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiRecordSetStreamA(hrecord, ifield, core::mem::transmute_copy(szfilepath)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiRecordSetStreamW(hrecord: super::MSIHANDLE, ifield: super::UINT, szfilepath: &windows_core::PCWSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiRecordSetStreamW(hrecord : super::MSIHANDLE, ifield : super::UINT, szfilepath : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiRecordSetStreamW(hrecord, ifield, core::mem::transmute_copy(szfilepath)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiRecordSetStringA(hrecord: super::MSIHANDLE, ifield: super::UINT, szvalue: &windows_core::PCSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiRecordSetStringA(hrecord : super::MSIHANDLE, ifield : super::UINT, szvalue : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiRecordSetStringA(hrecord, ifield, core::mem::transmute_copy(szvalue)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiRecordSetStringW(hrecord: super::MSIHANDLE, ifield: super::UINT, szvalue: &windows_core::PCWSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiRecordSetStringW(hrecord : super::MSIHANDLE, ifield : super::UINT, szvalue : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiRecordSetStringW(hrecord, ifield, core::mem::transmute_copy(szvalue)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSequenceA(hinstall: super::MSIHANDLE, sztable: &windows_core::PCSTR, isequencemode: super::INT) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSequenceA(hinstall : super::MSIHANDLE, sztable : windows_core::PCSTR, isequencemode : super::INT) -> super::UINT);
    unsafe { MsiSequenceA(hinstall, core::mem::transmute_copy(sztable), isequencemode) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSequenceW(hinstall: super::MSIHANDLE, sztable: &windows_core::PCWSTR, isequencemode: super::INT) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSequenceW(hinstall : super::MSIHANDLE, sztable : windows_core::PCWSTR, isequencemode : super::INT) -> super::UINT);
    unsafe { MsiSequenceW(hinstall, core::mem::transmute_copy(sztable), isequencemode) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSetComponentStateA(hinstall: super::MSIHANDLE, szcomponent: &windows_core::PCSTR, istate: super::INSTALLSTATE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSetComponentStateA(hinstall : super::MSIHANDLE, szcomponent : windows_core::PCSTR, istate : super::INSTALLSTATE) -> super::UINT);
    unsafe { MsiSetComponentStateA(hinstall, core::mem::transmute_copy(szcomponent), istate) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSetComponentStateW(hinstall: super::MSIHANDLE, szcomponent: &windows_core::PCWSTR, istate: super::INSTALLSTATE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSetComponentStateW(hinstall : super::MSIHANDLE, szcomponent : windows_core::PCWSTR, istate : super::INSTALLSTATE) -> super::UINT);
    unsafe { MsiSetComponentStateW(hinstall, core::mem::transmute_copy(szcomponent), istate) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSetFeatureAttributesA(hinstall: super::MSIHANDLE, szfeature: &windows_core::PCSTR, dwattributes: super::DWORD) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSetFeatureAttributesA(hinstall : super::MSIHANDLE, szfeature : windows_core::PCSTR, dwattributes : super::DWORD) -> super::UINT);
    unsafe { MsiSetFeatureAttributesA(hinstall, core::mem::transmute_copy(szfeature), dwattributes) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSetFeatureAttributesW(hinstall: super::MSIHANDLE, szfeature: &windows_core::PCWSTR, dwattributes: super::DWORD) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSetFeatureAttributesW(hinstall : super::MSIHANDLE, szfeature : windows_core::PCWSTR, dwattributes : super::DWORD) -> super::UINT);
    unsafe { MsiSetFeatureAttributesW(hinstall, core::mem::transmute_copy(szfeature), dwattributes) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSetFeatureStateA(hinstall: super::MSIHANDLE, szfeature: &windows_core::PCSTR, istate: super::INSTALLSTATE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSetFeatureStateA(hinstall : super::MSIHANDLE, szfeature : windows_core::PCSTR, istate : super::INSTALLSTATE) -> super::UINT);
    unsafe { MsiSetFeatureStateA(hinstall, core::mem::transmute_copy(szfeature), istate) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSetFeatureStateW(hinstall: super::MSIHANDLE, szfeature: &windows_core::PCWSTR, istate: super::INSTALLSTATE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSetFeatureStateW(hinstall : super::MSIHANDLE, szfeature : windows_core::PCWSTR, istate : super::INSTALLSTATE) -> super::UINT);
    unsafe { MsiSetFeatureStateW(hinstall, core::mem::transmute_copy(szfeature), istate) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSetInstallLevel(hinstall: super::MSIHANDLE, iinstalllevel: i32) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSetInstallLevel(hinstall : super::MSIHANDLE, iinstalllevel : i32) -> super::UINT);
    unsafe { MsiSetInstallLevel(hinstall, iinstalllevel) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSetMode(hinstall: super::MSIHANDLE, erunmode: MSIRUNMODE, fstate: windows_core::BOOL) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSetMode(hinstall : super::MSIHANDLE, erunmode : MSIRUNMODE, fstate : windows_core::BOOL) -> super::UINT);
    unsafe { MsiSetMode(hinstall, erunmode, fstate) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSetPropertyA(hinstall: super::MSIHANDLE, szname: &windows_core::PCSTR, szvalue: &windows_core::PCSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSetPropertyA(hinstall : super::MSIHANDLE, szname : windows_core::PCSTR, szvalue : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiSetPropertyA(hinstall, core::mem::transmute_copy(szname), core::mem::transmute_copy(szvalue)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSetPropertyW(hinstall: super::MSIHANDLE, szname: &windows_core::PCWSTR, szvalue: &windows_core::PCWSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSetPropertyW(hinstall : super::MSIHANDLE, szname : windows_core::PCWSTR, szvalue : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiSetPropertyW(hinstall, core::mem::transmute_copy(szname), core::mem::transmute_copy(szvalue)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSetTargetPathA(hinstall: super::MSIHANDLE, szfolder: &windows_core::PCSTR, szfolderpath: &windows_core::PCSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSetTargetPathA(hinstall : super::MSIHANDLE, szfolder : windows_core::PCSTR, szfolderpath : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiSetTargetPathA(hinstall, core::mem::transmute_copy(szfolder), core::mem::transmute_copy(szfolderpath)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSetTargetPathW(hinstall: super::MSIHANDLE, szfolder: &windows_core::PCWSTR, szfolderpath: &windows_core::PCWSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSetTargetPathW(hinstall : super::MSIHANDLE, szfolder : windows_core::PCWSTR, szfolderpath : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiSetTargetPathW(hinstall, core::mem::transmute_copy(szfolder), core::mem::transmute_copy(szfolderpath)) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyA(hsummaryinfo: super::MSIHANDLE, uiproperty: super::UINT, puidatatype: super::PUINT, pivalue: super::LPINT, pftvalue: Option<*mut super::FILETIME>, szvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<super::LPDWORD>) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoGetPropertyA(hsummaryinfo : super::MSIHANDLE, uiproperty : super::UINT, puidatatype : super::PUINT, pivalue : super::LPINT, pftvalue : *mut super::FILETIME, szvaluebuf : windows_core::PSTR, pcchvaluebuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiSummaryInfoGetPropertyA(hsummaryinfo, uiproperty, puidatatype as _, pivalue as _, pftvalue.unwrap_or(core::mem::zeroed()) as _, szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyCount(hsummaryinfo: super::MSIHANDLE, puipropertycount: super::PUINT) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoGetPropertyCount(hsummaryinfo : super::MSIHANDLE, puipropertycount : super::PUINT) -> super::UINT);
    unsafe { MsiSummaryInfoGetPropertyCount(hsummaryinfo, puipropertycount as _) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyW(hsummaryinfo: super::MSIHANDLE, uiproperty: super::UINT, puidatatype: super::PUINT, pivalue: super::LPINT, pftvalue: Option<*mut super::FILETIME>, szvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<super::LPDWORD>) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoGetPropertyW(hsummaryinfo : super::MSIHANDLE, uiproperty : super::UINT, puidatatype : super::PUINT, pivalue : super::LPINT, pftvalue : *mut super::FILETIME, szvaluebuf : windows_core::PWSTR, pcchvaluebuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiSummaryInfoGetPropertyW(hsummaryinfo, uiproperty, puidatatype as _, pivalue as _, pftvalue.unwrap_or(core::mem::zeroed()) as _, szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSummaryInfoPersist(hsummaryinfo: super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoPersist(hsummaryinfo : super::MSIHANDLE) -> super::UINT);
    unsafe { MsiSummaryInfoPersist(hsummaryinfo) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSummaryInfoSetPropertyA(hsummaryinfo: super::MSIHANDLE, uiproperty: super::UINT, uidatatype: super::UINT, ivalue: super::INT, pftvalue: *mut super::FILETIME, szvalue: &windows_core::PCSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoSetPropertyA(hsummaryinfo : super::MSIHANDLE, uiproperty : super::UINT, uidatatype : super::UINT, ivalue : super::INT, pftvalue : *mut super::FILETIME, szvalue : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiSummaryInfoSetPropertyA(hsummaryinfo, uiproperty, uidatatype, ivalue, pftvalue as _, core::mem::transmute_copy(szvalue)) }
}
#[cfg(all(feature = "minwindef", feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiSummaryInfoSetPropertyW(hsummaryinfo: super::MSIHANDLE, uiproperty: super::UINT, uidatatype: super::UINT, ivalue: super::INT, pftvalue: *mut super::FILETIME, szvalue: &windows_core::PCWSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoSetPropertyW(hsummaryinfo : super::MSIHANDLE, uiproperty : super::UINT, uidatatype : super::UINT, ivalue : super::INT, pftvalue : *mut super::FILETIME, szvalue : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiSummaryInfoSetPropertyW(hsummaryinfo, uiproperty, uidatatype, ivalue, pftvalue as _, core::mem::transmute_copy(szvalue)) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiVerifyDiskSpace(hinstall: super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiVerifyDiskSpace(hinstall : super::MSIHANDLE) -> super::UINT);
    unsafe { MsiVerifyDiskSpace(hinstall) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiViewClose(hview: super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiViewClose(hview : super::MSIHANDLE) -> super::UINT);
    unsafe { MsiViewClose(hview) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiViewExecute(hview: super::MSIHANDLE, hrecord: super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiViewExecute(hview : super::MSIHANDLE, hrecord : super::MSIHANDLE) -> super::UINT);
    unsafe { MsiViewExecute(hview, hrecord) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiViewFetch(hview: super::MSIHANDLE, phrecord: *mut super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiViewFetch(hview : super::MSIHANDLE, phrecord : *mut super::MSIHANDLE) -> super::UINT);
    unsafe { MsiViewFetch(hview, phrecord as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiViewGetColumnInfo(hview: super::MSIHANDLE, ecolumninfo: MSICOLINFO, phrecord: *mut super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiViewGetColumnInfo(hview : super::MSIHANDLE, ecolumninfo : MSICOLINFO, phrecord : *mut super::MSIHANDLE) -> super::UINT);
    unsafe { MsiViewGetColumnInfo(hview, ecolumninfo, phrecord as _) }
}
#[cfg(all(feature = "minwindef", feature = "msi"))]
#[inline]
pub unsafe fn MsiViewGetErrorA(hview: super::MSIHANDLE, szcolumnnamebuffer: Option<windows_core::PSTR>, pcchbuf: Option<super::LPDWORD>) -> MSIDBERROR {
    windows_core::link!("msi.dll" "system" fn MsiViewGetErrorA(hview : super::MSIHANDLE, szcolumnnamebuffer : windows_core::PSTR, pcchbuf : super::LPDWORD) -> MSIDBERROR);
    unsafe { MsiViewGetErrorA(hview, szcolumnnamebuffer.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "msi"))]
#[inline]
pub unsafe fn MsiViewGetErrorW(hview: super::MSIHANDLE, szcolumnnamebuffer: Option<windows_core::PWSTR>, pcchbuf: Option<super::LPDWORD>) -> MSIDBERROR {
    windows_core::link!("msi.dll" "system" fn MsiViewGetErrorW(hview : super::MSIHANDLE, szcolumnnamebuffer : windows_core::PWSTR, pcchbuf : super::LPDWORD) -> MSIDBERROR);
    unsafe { MsiViewGetErrorW(hview, szcolumnnamebuffer.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "msi", feature = "shared"))]
#[inline]
pub unsafe fn MsiViewModify(hview: super::MSIHANDLE, emodifymode: MSIMODIFY, hrecord: super::MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiViewModify(hview : super::MSIHANDLE, emodifymode : MSIMODIFY, hrecord : super::MSIHANDLE) -> super::UINT);
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
pub const MSIDBOPEN_CREATE: super::LPCTSTR = 3 as _;
#[cfg(feature = "winnt")]
pub const MSIDBOPEN_CREATEDIRECT: super::LPCTSTR = 4 as _;
#[cfg(feature = "winnt")]
pub const MSIDBOPEN_DIRECT: super::LPCTSTR = 2 as _;
#[cfg(target_arch = "x86")]
pub const MSIDBOPEN_PATCHFILE: u32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MSIDBOPEN_PATCHFILE: u64 = 32;
#[cfg(feature = "winnt")]
pub const MSIDBOPEN_READONLY: super::LPCTSTR = 0 as _;
#[cfg(feature = "winnt")]
pub const MSIDBOPEN_TRANSACT: super::LPCTSTR = 1 as _;
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
