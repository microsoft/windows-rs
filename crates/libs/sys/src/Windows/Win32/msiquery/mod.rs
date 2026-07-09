#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiCreateRecord(cparams : u32) -> super::msi::MSIHANDLE);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiCreateTransformSummaryInfoA(hdatabase : super::msi::MSIHANDLE, hdatabasereference : super::msi::MSIHANDLE, sztransformfile : windows_sys::core::PCSTR, ierrorconditions : i32, ivalidation : i32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiCreateTransformSummaryInfoW(hdatabase : super::msi::MSIHANDLE, hdatabasereference : super::msi::MSIHANDLE, sztransformfile : windows_sys::core::PCWSTR, ierrorconditions : i32, ivalidation : i32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDatabaseApplyTransformA(hdatabase : super::msi::MSIHANDLE, sztransformfile : windows_sys::core::PCSTR, ierrorconditions : i32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDatabaseApplyTransformW(hdatabase : super::msi::MSIHANDLE, sztransformfile : windows_sys::core::PCWSTR, ierrorconditions : i32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDatabaseCommit(hdatabase : super::msi::MSIHANDLE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDatabaseExportA(hdatabase : super::msi::MSIHANDLE, sztablename : windows_sys::core::PCSTR, szfolderpath : windows_sys::core::PCSTR, szfilename : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDatabaseExportW(hdatabase : super::msi::MSIHANDLE, sztablename : windows_sys::core::PCWSTR, szfolderpath : windows_sys::core::PCWSTR, szfilename : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDatabaseGenerateTransformA(hdatabase : super::msi::MSIHANDLE, hdatabasereference : super::msi::MSIHANDLE, sztransformfile : windows_sys::core::PCSTR, ireserved1 : i32, ireserved2 : i32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDatabaseGenerateTransformW(hdatabase : super::msi::MSIHANDLE, hdatabasereference : super::msi::MSIHANDLE, sztransformfile : windows_sys::core::PCWSTR, ireserved1 : i32, ireserved2 : i32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDatabaseGetPrimaryKeysA(hdatabase : super::msi::MSIHANDLE, sztablename : windows_sys::core::PCSTR, phrecord : *mut super::msi::MSIHANDLE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDatabaseGetPrimaryKeysW(hdatabase : super::msi::MSIHANDLE, sztablename : windows_sys::core::PCWSTR, phrecord : *mut super::msi::MSIHANDLE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDatabaseImportA(hdatabase : super::msi::MSIHANDLE, szfolderpath : windows_sys::core::PCSTR, szfilename : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDatabaseImportW(hdatabase : super::msi::MSIHANDLE, szfolderpath : windows_sys::core::PCWSTR, szfilename : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDatabaseIsTablePersistentA(hdatabase : super::msi::MSIHANDLE, sztablename : windows_sys::core::PCSTR) -> MSICONDITION);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDatabaseIsTablePersistentW(hdatabase : super::msi::MSIHANDLE, sztablename : windows_sys::core::PCWSTR) -> MSICONDITION);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDatabaseMergeA(hdatabase : super::msi::MSIHANDLE, hdatabasemerge : super::msi::MSIHANDLE, sztablename : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDatabaseMergeW(hdatabase : super::msi::MSIHANDLE, hdatabasemerge : super::msi::MSIHANDLE, sztablename : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDatabaseOpenViewA(hdatabase : super::msi::MSIHANDLE, szquery : windows_sys::core::PCSTR, phview : *mut super::msi::MSIHANDLE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDatabaseOpenViewW(hdatabase : super::msi::MSIHANDLE, szquery : windows_sys::core::PCWSTR, phview : *mut super::msi::MSIHANDLE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDoActionA(hinstall : super::msi::MSIHANDLE, szaction : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiDoActionW(hinstall : super::msi::MSIHANDLE, szaction : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiEnableUIPreview(hdatabase : super::msi::MSIHANDLE, phpreview : *mut super::msi::MSIHANDLE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiEnumComponentCostsA(hinstall : super::msi::MSIHANDLE, szcomponent : windows_sys::core::PCSTR, dwindex : u32, istate : super::msi::INSTALLSTATE, szdrivebuf : windows_sys::core::PSTR, pcchdrivebuf : *mut u32, picost : *mut i32, pitempcost : *mut i32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiEnumComponentCostsW(hinstall : super::msi::MSIHANDLE, szcomponent : windows_sys::core::PCWSTR, dwindex : u32, istate : super::msi::INSTALLSTATE, szdrivebuf : windows_sys::core::PWSTR, pcchdrivebuf : *mut u32, picost : *mut i32, pitempcost : *mut i32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiEvaluateConditionA(hinstall : super::msi::MSIHANDLE, szcondition : windows_sys::core::PCSTR) -> MSICONDITION);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiEvaluateConditionW(hinstall : super::msi::MSIHANDLE, szcondition : windows_sys::core::PCWSTR) -> MSICONDITION);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiFormatRecordA(hinstall : super::msi::MSIHANDLE, hrecord : super::msi::MSIHANDLE, szresultbuf : windows_sys::core::PSTR, pcchresultbuf : *mut u32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiFormatRecordW(hinstall : super::msi::MSIHANDLE, hrecord : super::msi::MSIHANDLE, szresultbuf : windows_sys::core::PWSTR, pcchresultbuf : *mut u32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetActiveDatabase(hinstall : super::msi::MSIHANDLE) -> super::msi::MSIHANDLE);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetComponentStateA(hinstall : super::msi::MSIHANDLE, szcomponent : windows_sys::core::PCSTR, piinstalled : *mut super::msi::INSTALLSTATE, piaction : *mut super::msi::INSTALLSTATE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetComponentStateW(hinstall : super::msi::MSIHANDLE, szcomponent : windows_sys::core::PCWSTR, piinstalled : *mut super::msi::INSTALLSTATE, piaction : *mut super::msi::INSTALLSTATE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetDatabaseState(hdatabase : super::msi::MSIHANDLE) -> MSIDBSTATE);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetFeatureCostA(hinstall : super::msi::MSIHANDLE, szfeature : windows_sys::core::PCSTR, icosttree : MSICOSTTREE, istate : super::msi::INSTALLSTATE, picost : *mut i32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetFeatureCostW(hinstall : super::msi::MSIHANDLE, szfeature : windows_sys::core::PCWSTR, icosttree : MSICOSTTREE, istate : super::msi::INSTALLSTATE, picost : *mut i32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetFeatureStateA(hinstall : super::msi::MSIHANDLE, szfeature : windows_sys::core::PCSTR, piinstalled : *mut super::msi::INSTALLSTATE, piaction : *mut super::msi::INSTALLSTATE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetFeatureStateW(hinstall : super::msi::MSIHANDLE, szfeature : windows_sys::core::PCWSTR, piinstalled : *mut super::msi::INSTALLSTATE, piaction : *mut super::msi::INSTALLSTATE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetFeatureValidStatesA(hinstall : super::msi::MSIHANDLE, szfeature : windows_sys::core::PCSTR, lpinstallstates : *mut u32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetFeatureValidStatesW(hinstall : super::msi::MSIHANDLE, szfeature : windows_sys::core::PCWSTR, lpinstallstates : *mut u32) -> u32);
#[cfg(all(feature = "Win32_msi", feature = "Win32_winnt"))]
windows_link::link!("msi.dll" "system" fn MsiGetLanguage(hinstall : super::msi::MSIHANDLE) -> super::winnt::LANGID);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetLastErrorRecord() -> super::msi::MSIHANDLE);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetMode(hinstall : super::msi::MSIHANDLE, erunmode : MSIRUNMODE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetPropertyA(hinstall : super::msi::MSIHANDLE, szname : windows_sys::core::PCSTR, szvaluebuf : windows_sys::core::PSTR, pcchvaluebuf : *mut u32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetPropertyW(hinstall : super::msi::MSIHANDLE, szname : windows_sys::core::PCWSTR, szvaluebuf : windows_sys::core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetSourcePathA(hinstall : super::msi::MSIHANDLE, szfolder : windows_sys::core::PCSTR, szpathbuf : windows_sys::core::PSTR, pcchpathbuf : *mut u32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetSourcePathW(hinstall : super::msi::MSIHANDLE, szfolder : windows_sys::core::PCWSTR, szpathbuf : windows_sys::core::PWSTR, pcchpathbuf : *mut u32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetSummaryInformationA(hdatabase : super::msi::MSIHANDLE, szdatabasepath : windows_sys::core::PCSTR, uiupdatecount : u32, phsummaryinfo : *mut super::msi::MSIHANDLE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetSummaryInformationW(hdatabase : super::msi::MSIHANDLE, szdatabasepath : windows_sys::core::PCWSTR, uiupdatecount : u32, phsummaryinfo : *mut super::msi::MSIHANDLE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetTargetPathA(hinstall : super::msi::MSIHANDLE, szfolder : windows_sys::core::PCSTR, szpathbuf : windows_sys::core::PSTR, pcchpathbuf : *mut u32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiGetTargetPathW(hinstall : super::msi::MSIHANDLE, szfolder : windows_sys::core::PCWSTR, szpathbuf : windows_sys::core::PWSTR, pcchpathbuf : *mut u32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiOpenDatabaseA(szdatabasepath : windows_sys::core::PCSTR, szpersist : windows_sys::core::PCSTR, phdatabase : *mut super::msi::MSIHANDLE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiOpenDatabaseW(szdatabasepath : windows_sys::core::PCWSTR, szpersist : windows_sys::core::PCWSTR, phdatabase : *mut super::msi::MSIHANDLE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiPreviewBillboardA(hpreview : super::msi::MSIHANDLE, szcontrolname : windows_sys::core::PCSTR, szbillboard : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiPreviewBillboardW(hpreview : super::msi::MSIHANDLE, szcontrolname : windows_sys::core::PCWSTR, szbillboard : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiPreviewDialogA(hpreview : super::msi::MSIHANDLE, szdialogname : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiPreviewDialogW(hpreview : super::msi::MSIHANDLE, szdialogname : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiProcessMessage(hinstall : super::msi::MSIHANDLE, emessagetype : super::msi::INSTALLMESSAGE, hrecord : super::msi::MSIHANDLE) -> i32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiRecordClearData(hrecord : super::msi::MSIHANDLE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiRecordDataSize(hrecord : super::msi::MSIHANDLE, ifield : u32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiRecordGetFieldCount(hrecord : super::msi::MSIHANDLE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiRecordGetInteger(hrecord : super::msi::MSIHANDLE, ifield : u32) -> i32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiRecordGetStringA(hrecord : super::msi::MSIHANDLE, ifield : u32, szvaluebuf : windows_sys::core::PSTR, pcchvaluebuf : *mut u32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiRecordGetStringW(hrecord : super::msi::MSIHANDLE, ifield : u32, szvaluebuf : windows_sys::core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiRecordIsNull(hrecord : super::msi::MSIHANDLE, ifield : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiRecordReadStream(hrecord : super::msi::MSIHANDLE, ifield : u32, szdatabuf : *mut i8, pcbdatabuf : *mut u32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiRecordSetInteger(hrecord : super::msi::MSIHANDLE, ifield : u32, ivalue : i32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiRecordSetStreamA(hrecord : super::msi::MSIHANDLE, ifield : u32, szfilepath : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiRecordSetStreamW(hrecord : super::msi::MSIHANDLE, ifield : u32, szfilepath : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiRecordSetStringA(hrecord : super::msi::MSIHANDLE, ifield : u32, szvalue : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiRecordSetStringW(hrecord : super::msi::MSIHANDLE, ifield : u32, szvalue : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiSequenceA(hinstall : super::msi::MSIHANDLE, sztable : windows_sys::core::PCSTR, isequencemode : i32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiSequenceW(hinstall : super::msi::MSIHANDLE, sztable : windows_sys::core::PCWSTR, isequencemode : i32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiSetComponentStateA(hinstall : super::msi::MSIHANDLE, szcomponent : windows_sys::core::PCSTR, istate : super::msi::INSTALLSTATE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiSetComponentStateW(hinstall : super::msi::MSIHANDLE, szcomponent : windows_sys::core::PCWSTR, istate : super::msi::INSTALLSTATE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiSetFeatureAttributesA(hinstall : super::msi::MSIHANDLE, szfeature : windows_sys::core::PCSTR, dwattributes : u32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiSetFeatureAttributesW(hinstall : super::msi::MSIHANDLE, szfeature : windows_sys::core::PCWSTR, dwattributes : u32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiSetFeatureStateA(hinstall : super::msi::MSIHANDLE, szfeature : windows_sys::core::PCSTR, istate : super::msi::INSTALLSTATE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiSetFeatureStateW(hinstall : super::msi::MSIHANDLE, szfeature : windows_sys::core::PCWSTR, istate : super::msi::INSTALLSTATE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiSetInstallLevel(hinstall : super::msi::MSIHANDLE, iinstalllevel : i32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiSetMode(hinstall : super::msi::MSIHANDLE, erunmode : MSIRUNMODE, fstate : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiSetPropertyA(hinstall : super::msi::MSIHANDLE, szname : windows_sys::core::PCSTR, szvalue : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiSetPropertyW(hinstall : super::msi::MSIHANDLE, szname : windows_sys::core::PCWSTR, szvalue : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiSetTargetPathA(hinstall : super::msi::MSIHANDLE, szfolder : windows_sys::core::PCSTR, szfolderpath : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiSetTargetPathW(hinstall : super::msi::MSIHANDLE, szfolder : windows_sys::core::PCWSTR, szfolderpath : windows_sys::core::PCWSTR) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_msi"))]
windows_link::link!("msi.dll" "system" fn MsiSummaryInfoGetPropertyA(hsummaryinfo : super::msi::MSIHANDLE, uiproperty : u32, puidatatype : *mut u32, pivalue : *mut i32, pftvalue : *mut super::minwindef::FILETIME, szvaluebuf : windows_sys::core::PSTR, pcchvaluebuf : *mut u32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiSummaryInfoGetPropertyCount(hsummaryinfo : super::msi::MSIHANDLE, puipropertycount : *mut u32) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_msi"))]
windows_link::link!("msi.dll" "system" fn MsiSummaryInfoGetPropertyW(hsummaryinfo : super::msi::MSIHANDLE, uiproperty : u32, puidatatype : *mut u32, pivalue : *mut i32, pftvalue : *mut super::minwindef::FILETIME, szvaluebuf : windows_sys::core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiSummaryInfoPersist(hsummaryinfo : super::msi::MSIHANDLE) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_msi"))]
windows_link::link!("msi.dll" "system" fn MsiSummaryInfoSetPropertyA(hsummaryinfo : super::msi::MSIHANDLE, uiproperty : u32, uidatatype : u32, ivalue : i32, pftvalue : *mut super::minwindef::FILETIME, szvalue : windows_sys::core::PCSTR) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_msi"))]
windows_link::link!("msi.dll" "system" fn MsiSummaryInfoSetPropertyW(hsummaryinfo : super::msi::MSIHANDLE, uiproperty : u32, uidatatype : u32, ivalue : i32, pftvalue : *mut super::minwindef::FILETIME, szvalue : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiVerifyDiskSpace(hinstall : super::msi::MSIHANDLE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiViewClose(hview : super::msi::MSIHANDLE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiViewExecute(hview : super::msi::MSIHANDLE, hrecord : super::msi::MSIHANDLE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiViewFetch(hview : super::msi::MSIHANDLE, phrecord : *mut super::msi::MSIHANDLE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiViewGetColumnInfo(hview : super::msi::MSIHANDLE, ecolumninfo : MSICOLINFO, phrecord : *mut super::msi::MSIHANDLE) -> u32);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiViewGetErrorA(hview : super::msi::MSIHANDLE, szcolumnnamebuffer : windows_sys::core::PSTR, pcchbuf : *mut u32) -> MSIDBERROR);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiViewGetErrorW(hview : super::msi::MSIHANDLE, szcolumnnamebuffer : windows_sys::core::PWSTR, pcchbuf : *mut u32) -> MSIDBERROR);
#[cfg(feature = "Win32_msi")]
windows_link::link!("msi.dll" "system" fn MsiViewModify(hview : super::msi::MSIHANDLE, emodifymode : MSIMODIFY, hrecord : super::msi::MSIHANDLE) -> u32);
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
#[cfg(feature = "Win32_winnt")]
pub const MSIDBOPEN_CREATE: super::winnt::LPCTSTR = 3 as _;
#[cfg(feature = "Win32_winnt")]
pub const MSIDBOPEN_CREATEDIRECT: super::winnt::LPCTSTR = 4 as _;
#[cfg(feature = "Win32_winnt")]
pub const MSIDBOPEN_DIRECT: super::winnt::LPCTSTR = 2 as _;
#[cfg(feature = "Win32_winnt")]
pub const MSIDBOPEN_READONLY: super::winnt::LPCTSTR = 0 as _;
#[cfg(feature = "Win32_winnt")]
pub const MSIDBOPEN_TRANSACT: super::winnt::LPCTSTR = 1 as _;
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
