windows_link::link!("esent.dll" "system" fn JetAddColumnA(sesid : JET_SESID, tableid : JET_TABLEID, szcolumnname : *const JET_CHAR, pcolumndef : *const JET_COLUMNDEF, pvdefault : JET_PCVOID, cbdefault : JET_UINT32, pcolumnid : *mut JET_COLUMNID) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetAddColumnW(sesid : JET_SESID, tableid : JET_TABLEID, szcolumnname : *const JET_WCHAR, pcolumndef : *const JET_COLUMNDEF, pvdefault : JET_PCVOID, cbdefault : JET_UINT32, pcolumnid : *mut JET_COLUMNID) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetAttachDatabase2A(sesid : JET_SESID, szfilename : *const JET_CHAR, cpgdatabasesizemax : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetAttachDatabase2W(sesid : JET_SESID, szfilename : *const JET_WCHAR, cpgdatabasesizemax : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetAttachDatabaseA(sesid : JET_SESID, szfilename : *const JET_CHAR, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetAttachDatabaseW(sesid : JET_SESID, szfilename : *const JET_WCHAR, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetBackupA(szbackuppath : *const JET_CHAR, grbit : JET_GRBIT, pfnstatus : JET_PFNSTATUS) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetBackupInstanceA(instance : JET_INSTANCE, szbackuppath : *const JET_CHAR, grbit : JET_GRBIT, pfnstatus : JET_PFNSTATUS) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetBackupInstanceW(instance : JET_INSTANCE, szbackuppath : *const JET_WCHAR, grbit : JET_GRBIT, pfnstatus : JET_PFNSTATUS) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetBackupW(szbackuppath : *const JET_WCHAR, grbit : JET_GRBIT, pfnstatus : JET_PFNSTATUS) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetBeginExternalBackup(grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetBeginExternalBackupInstance(instance : JET_INSTANCE, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetBeginSessionA(instance : JET_INSTANCE, psesid : *mut JET_SESID, szusername : *const JET_CHAR, szpassword : *const JET_CHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetBeginSessionW(instance : JET_INSTANCE, psesid : *mut JET_SESID, szusername : *const JET_WCHAR, szpassword : *const JET_WCHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetBeginTransaction(sesid : JET_SESID) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetBeginTransaction2(sesid : JET_SESID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetBeginTransaction3(sesid : JET_SESID, trxid : JET_INT64, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCloseDatabase(sesid : JET_SESID, dbid : JET_DBID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCloseFile(hffile : JET_HANDLE) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCloseFileInstance(instance : JET_INSTANCE, hffile : JET_HANDLE) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCloseTable(sesid : JET_SESID, tableid : JET_TABLEID) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCommitTransaction(sesid : JET_SESID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCommitTransaction2(sesid : JET_SESID, grbit : JET_GRBIT, cmsecdurablecommit : JET_UINT32, pcommitid : *mut JET_COMMIT_ID) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCompactA(sesid : JET_SESID, szdatabasesrc : *const JET_CHAR, szdatabasedest : *const JET_CHAR, pfnstatus : JET_PFNSTATUS, pconvert : *const JET_CONVERT_A, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCompactW(sesid : JET_SESID, szdatabasesrc : *const JET_WCHAR, szdatabasedest : *const JET_WCHAR, pfnstatus : JET_PFNSTATUS, pconvert : *const JET_CONVERT_W, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetComputeStats(sesid : JET_SESID, tableid : JET_TABLEID) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetConfigureProcessForCrashDump(grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateDatabase2A(sesid : JET_SESID, szfilename : *const JET_CHAR, cpgdatabasesizemax : JET_UINT32, pdbid : *mut JET_DBID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateDatabase2W(sesid : JET_SESID, szfilename : *const JET_WCHAR, cpgdatabasesizemax : JET_UINT32, pdbid : *mut JET_DBID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateDatabaseA(sesid : JET_SESID, szfilename : *const JET_CHAR, szconnect : *const JET_CHAR, pdbid : *mut JET_DBID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateDatabaseW(sesid : JET_SESID, szfilename : *const JET_WCHAR, szconnect : *const JET_WCHAR, pdbid : *mut JET_DBID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateIndex2A(sesid : JET_SESID, tableid : JET_TABLEID, pindexcreate : *const JET_INDEXCREATE_A, cindexcreate : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateIndex2W(sesid : JET_SESID, tableid : JET_TABLEID, pindexcreate : *const JET_INDEXCREATE_W, cindexcreate : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateIndex3A(sesid : JET_SESID, tableid : JET_TABLEID, pindexcreate : *const JET_INDEXCREATE2_A, cindexcreate : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateIndex3W(sesid : JET_SESID, tableid : JET_TABLEID, pindexcreate : *const JET_INDEXCREATE2_W, cindexcreate : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateIndex4A(sesid : JET_SESID, tableid : JET_TABLEID, pindexcreate : *const JET_INDEXCREATE3_A, cindexcreate : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateIndex4W(sesid : JET_SESID, tableid : JET_TABLEID, pindexcreate : *const JET_INDEXCREATE3_W, cindexcreate : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateIndexA(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_CHAR, grbit : JET_GRBIT, szkey : *const JET_CHAR, cbkey : JET_UINT32, ldensity : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateIndexW(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_WCHAR, grbit : JET_GRBIT, szkey : *const JET_WCHAR, cbkey : JET_UINT32, ldensity : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateInstance2A(pinstance : *mut JET_INSTANCE, szinstancename : *const JET_CHAR, szdisplayname : *const JET_CHAR, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateInstance2W(pinstance : *mut JET_INSTANCE, szinstancename : *const JET_WCHAR, szdisplayname : *const JET_WCHAR, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateInstanceA(pinstance : *mut JET_INSTANCE, szinstancename : *const JET_CHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateInstanceW(pinstance : *mut JET_INSTANCE, szinstancename : *const JET_WCHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateTableA(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_CHAR, lpages : JET_UINT32, ldensity : JET_UINT32, ptableid : *mut JET_TABLEID) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateTableColumnIndex2A(sesid : JET_SESID, dbid : JET_DBID, ptablecreate : *mut JET_TABLECREATE2_A) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateTableColumnIndex2W(sesid : JET_SESID, dbid : JET_DBID, ptablecreate : *mut JET_TABLECREATE2_W) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateTableColumnIndex3A(sesid : JET_SESID, dbid : JET_DBID, ptablecreate : *mut JET_TABLECREATE3_A) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateTableColumnIndex3W(sesid : JET_SESID, dbid : JET_DBID, ptablecreate : *mut JET_TABLECREATE3_W) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateTableColumnIndex4A(sesid : JET_SESID, dbid : JET_DBID, ptablecreate : *mut JET_TABLECREATE4_A) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateTableColumnIndex4W(sesid : JET_SESID, dbid : JET_DBID, ptablecreate : *mut JET_TABLECREATE4_W) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateTableColumnIndexA(sesid : JET_SESID, dbid : JET_DBID, ptablecreate : *mut JET_TABLECREATE_A) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateTableColumnIndexW(sesid : JET_SESID, dbid : JET_DBID, ptablecreate : *mut JET_TABLECREATE_W) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetCreateTableW(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_WCHAR, lpages : JET_UINT32, ldensity : JET_UINT32, ptableid : *mut JET_TABLEID) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDefragment2A(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_CHAR, pcpasses : *mut JET_UINT32, pcseconds : *mut JET_UINT32, callback : JET_CALLBACK, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDefragment2W(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_WCHAR, pcpasses : *mut JET_UINT32, pcseconds : *mut JET_UINT32, callback : JET_CALLBACK, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDefragment3A(sesid : JET_SESID, szdatabasename : *const JET_CHAR, sztablename : *const JET_CHAR, pcpasses : *mut JET_UINT32, pcseconds : *mut JET_UINT32, callback : JET_CALLBACK, pvcontext : JET_PVOID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDefragment3W(sesid : JET_SESID, szdatabasename : *const JET_WCHAR, sztablename : *const JET_WCHAR, pcpasses : *mut JET_UINT32, pcseconds : *mut JET_UINT32, callback : JET_CALLBACK, pvcontext : JET_PVOID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDefragmentA(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_CHAR, pcpasses : *mut JET_UINT32, pcseconds : *mut JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDefragmentW(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_WCHAR, pcpasses : *mut JET_UINT32, pcseconds : *mut JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDelete(sesid : JET_SESID, tableid : JET_TABLEID) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDeleteColumn2A(sesid : JET_SESID, tableid : JET_TABLEID, szcolumnname : *const JET_CHAR, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDeleteColumn2W(sesid : JET_SESID, tableid : JET_TABLEID, szcolumnname : *const JET_WCHAR, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDeleteColumnA(sesid : JET_SESID, tableid : JET_TABLEID, szcolumnname : *const JET_CHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDeleteColumnW(sesid : JET_SESID, tableid : JET_TABLEID, szcolumnname : *const JET_WCHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDeleteIndexA(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_CHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDeleteIndexW(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_WCHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDeleteTableA(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_CHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDeleteTableW(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_WCHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDetachDatabase2A(sesid : JET_SESID, szfilename : *const JET_CHAR, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDetachDatabase2W(sesid : JET_SESID, szfilename : *const JET_WCHAR, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDetachDatabaseA(sesid : JET_SESID, szfilename : *const JET_CHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDetachDatabaseW(sesid : JET_SESID, szfilename : *const JET_WCHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDupCursor(sesid : JET_SESID, tableid : JET_TABLEID, ptableid : *mut JET_TABLEID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetDupSession(sesid : JET_SESID, psesid : *mut JET_SESID) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetEnableMultiInstanceA(psetsysparam : *const JET_SETSYSPARAM_A, csetsysparam : JET_UINT32, pcsetsucceed : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetEnableMultiInstanceW(psetsysparam : *const JET_SETSYSPARAM_W, csetsysparam : JET_UINT32, pcsetsucceed : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetEndExternalBackup() -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetEndExternalBackupInstance(instance : JET_INSTANCE) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetEndExternalBackupInstance2(instance : JET_INSTANCE, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetEndSession(sesid : JET_SESID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetEnumerateColumns(sesid : JET_SESID, tableid : JET_TABLEID, cenumcolumnid : JET_UINT32, rgenumcolumnid : *const JET_ENUMCOLUMNID, pcenumcolumn : *mut JET_UINT32, prgenumcolumn : *mut *mut JET_ENUMCOLUMN, pfnrealloc : JET_PFNREALLOC, pvrealloccontext : JET_PVOID, cbdatamost : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetEscrowUpdate(sesid : JET_SESID, tableid : JET_TABLEID, columnid : JET_COLUMNID, pv : JET_PVOID, cbmax : JET_UINT32, pvold : JET_PVOID, cboldmax : JET_UINT32, pcboldactual : *mut JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetExternalRestore2A(szcheckpointfilepath : *const JET_CHAR, szlogpath : *const JET_CHAR, rgrstmap : *const JET_RSTMAP_A, crstfilemap : JET_INT32, szbackuplogpath : *const JET_CHAR, ploginfo : *mut JET_LOGINFO_A, sztargetinstancename : *const JET_CHAR, sztargetinstancelogpath : *const JET_CHAR, sztargetinstancecheckpointpath : *const JET_CHAR, pfn : JET_PFNSTATUS) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetExternalRestore2W(szcheckpointfilepath : *const JET_WCHAR, szlogpath : *const JET_WCHAR, rgrstmap : *const JET_RSTMAP_W, crstfilemap : JET_INT32, szbackuplogpath : *const JET_WCHAR, ploginfo : *mut JET_LOGINFO_W, sztargetinstancename : *const JET_WCHAR, sztargetinstancelogpath : *const JET_WCHAR, sztargetinstancecheckpointpath : *const JET_WCHAR, pfn : JET_PFNSTATUS) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetExternalRestoreA(szcheckpointfilepath : *const JET_CHAR, szlogpath : *const JET_CHAR, rgrstmap : *const JET_RSTMAP_A, crstfilemap : JET_INT32, szbackuplogpath : *const JET_CHAR, genlow : JET_INT32, genhigh : JET_INT32, pfn : JET_PFNSTATUS) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetExternalRestoreW(szcheckpointfilepath : *const JET_WCHAR, szlogpath : *const JET_WCHAR, rgrstmap : *const JET_RSTMAP_W, crstfilemap : JET_INT32, szbackuplogpath : *const JET_WCHAR, genlow : JET_INT32, genhigh : JET_INT32, pfn : JET_PFNSTATUS) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetFreeBuffer(pbbuf : *mut JET_CHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetAttachInfoA(szzdatabases : *mut JET_CHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetAttachInfoInstanceA(instance : JET_INSTANCE, szzdatabases : *mut JET_CHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetAttachInfoInstanceW(instance : JET_INSTANCE, szzdatabases : *mut JET_WCHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetAttachInfoW(wszzdatabases : *mut JET_WCHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetBookmark(sesid : JET_SESID, tableid : JET_TABLEID, pvbookmark : JET_PVOID, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetColumnInfoA(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_CHAR, pcolumnnameorid : *const JET_CHAR, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetColumnInfoW(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_WCHAR, pwcolumnnameorid : *const JET_WCHAR, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetCurrentIndexA(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *mut JET_CHAR, cbindexname : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetCurrentIndexW(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *mut JET_WCHAR, cbindexname : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetCursorInfo(sesid : JET_SESID, tableid : JET_TABLEID, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetDatabaseFileInfoA(szdatabasename : *const JET_CHAR, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetDatabaseFileInfoW(szdatabasename : *const JET_WCHAR, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetDatabaseInfoA(sesid : JET_SESID, dbid : JET_DBID, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetDatabaseInfoW(sesid : JET_SESID, dbid : JET_DBID, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetErrorInfoW(pvcontext : JET_PVOID, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetIndexInfoA(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_CHAR, szindexname : *const JET_CHAR, pvresult : JET_PVOID, cbresult : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetIndexInfoW(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_WCHAR, szindexname : *const JET_WCHAR, pvresult : JET_PVOID, cbresult : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetInstanceInfoA(pcinstanceinfo : *mut JET_UINT32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_A) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetInstanceInfoW(pcinstanceinfo : *mut JET_UINT32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_W) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetInstanceMiscInfo(instance : JET_INSTANCE, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetLS(sesid : JET_SESID, tableid : JET_TABLEID, pls : *mut JET_LS, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetLock(sesid : JET_SESID, tableid : JET_TABLEID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetLogInfoA(szzlogs : *mut JET_CHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetLogInfoInstance2A(instance : JET_INSTANCE, szzlogs : *mut JET_CHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32, ploginfo : *mut JET_LOGINFO_A) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetLogInfoInstance2W(instance : JET_INSTANCE, wszzlogs : *mut JET_WCHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32, ploginfo : *mut JET_LOGINFO_W) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetLogInfoInstanceA(instance : JET_INSTANCE, szzlogs : *mut JET_CHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetLogInfoInstanceW(instance : JET_INSTANCE, wszzlogs : *mut JET_WCHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetLogInfoW(szzlogs : *mut JET_WCHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetObjectInfoA(sesid : JET_SESID, dbid : JET_DBID, objtyp : JET_OBJTYP, szcontainername : *const JET_CHAR, szobjectname : *const JET_CHAR, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetObjectInfoW(sesid : JET_SESID, dbid : JET_DBID, objtyp : JET_OBJTYP, szcontainername : *const JET_WCHAR, szobjectname : *const JET_WCHAR, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetRecordPosition(sesid : JET_SESID, tableid : JET_TABLEID, precpos : *mut JET_RECPOS, cbrecpos : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetRecordSize(sesid : JET_SESID, tableid : JET_TABLEID, precsize : *mut JET_RECSIZE, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetRecordSize2(sesid : JET_SESID, tableid : JET_TABLEID, precsize : *mut JET_RECSIZE2, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetSecondaryIndexBookmark(sesid : JET_SESID, tableid : JET_TABLEID, pvsecondarykey : JET_PVOID, cbsecondarykeymax : JET_UINT32, pcbsecondarykeyactual : *mut JET_UINT32, pvprimarybookmark : JET_PVOID, cbprimarybookmarkmax : JET_UINT32, pcbprimarybookmarkactual : *mut JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetSessionParameter(sesid : JET_SESID, sesparamid : JET_UINT32, pvparam : JET_PVOID, cbparammax : JET_UINT32, pcbparamactual : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetSystemParameterA(instance : JET_INSTANCE, sesid : JET_SESID, paramid : JET_UINT32, plparam : *mut JET_API_PTR, szparam : *mut JET_CHAR, cbmax : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetSystemParameterW(instance : JET_INSTANCE, sesid : JET_SESID, paramid : JET_UINT32, plparam : *mut JET_API_PTR, szparam : *mut JET_WCHAR, cbmax : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetTableColumnInfoA(sesid : JET_SESID, tableid : JET_TABLEID, szcolumnname : *const JET_CHAR, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetTableColumnInfoW(sesid : JET_SESID, tableid : JET_TABLEID, szcolumnname : *const JET_WCHAR, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetTableIndexInfoA(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_CHAR, pvresult : JET_PVOID, cbresult : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetTableIndexInfoW(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_WCHAR, pvresult : JET_PVOID, cbresult : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetTableInfoA(sesid : JET_SESID, tableid : JET_TABLEID, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetTableInfoW(sesid : JET_SESID, tableid : JET_TABLEID, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetThreadStats(pvresult : JET_PVOID, cbmax : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetTruncateLogInfoInstanceA(instance : JET_INSTANCE, szzlogs : *mut JET_CHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetTruncateLogInfoInstanceW(instance : JET_INSTANCE, wszzlogs : *mut JET_WCHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGetVersion(sesid : JET_SESID, pwversion : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGotoBookmark(sesid : JET_SESID, tableid : JET_TABLEID, pvbookmark : JET_PVOID, cbbookmark : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGotoPosition(sesid : JET_SESID, tableid : JET_TABLEID, precpos : *const JET_RECPOS) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGotoSecondaryIndexBookmark(sesid : JET_SESID, tableid : JET_TABLEID, pvsecondarykey : JET_PVOID, cbsecondarykey : JET_UINT32, pvprimarybookmark : JET_PVOID, cbprimarybookmark : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetGrowDatabase(sesid : JET_SESID, dbid : JET_DBID, cpg : JET_UINT32, pcpgreal : *const JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetIdle(sesid : JET_SESID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetIndexRecordCount(sesid : JET_SESID, tableid : JET_TABLEID, pcrec : *mut JET_UINT32, crecmax : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetInit(pinstance : *mut JET_INSTANCE) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetInit2(pinstance : *mut JET_INSTANCE, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetInit3A(pinstance : *mut JET_INSTANCE, prstinfo : *const JET_RSTINFO_A, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetInit3W(pinstance : *mut JET_INSTANCE, prstinfo : *const JET_RSTINFO_W, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetIntersectIndexes(sesid : JET_SESID, rgindexrange : *const JET_INDEXRANGE, cindexrange : JET_UINT32, precordlist : *mut JET_RECORDLIST, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetMakeKey(sesid : JET_SESID, tableid : JET_TABLEID, pvdata : JET_PCVOID, cbdata : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetMove(sesid : JET_SESID, tableid : JET_TABLEID, crow : JET_INT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOSSnapshotAbort(snapid : JET_OSSNAPID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOSSnapshotEnd(snapid : JET_OSSNAPID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOSSnapshotFreezeA(snapid : JET_OSSNAPID, pcinstanceinfo : *mut JET_UINT32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_A, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOSSnapshotFreezeW(snapid : JET_OSSNAPID, pcinstanceinfo : *mut JET_UINT32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_W, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOSSnapshotGetFreezeInfoA(snapid : JET_OSSNAPID, pcinstanceinfo : *mut JET_UINT32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_A, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOSSnapshotGetFreezeInfoW(snapid : JET_OSSNAPID, pcinstanceinfo : *mut JET_UINT32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_W, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOSSnapshotPrepare(psnapid : *mut JET_OSSNAPID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOSSnapshotPrepareInstance(snapid : JET_OSSNAPID, instance : JET_INSTANCE, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOSSnapshotThaw(snapid : JET_OSSNAPID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOSSnapshotTruncateLog(snapid : JET_OSSNAPID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOSSnapshotTruncateLogInstance(snapid : JET_OSSNAPID, instance : JET_INSTANCE, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOpenDatabaseA(sesid : JET_SESID, szfilename : *const JET_CHAR, szconnect : *const JET_CHAR, pdbid : *mut JET_DBID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOpenDatabaseW(sesid : JET_SESID, szfilename : *const JET_WCHAR, szconnect : *const JET_WCHAR, pdbid : *mut JET_DBID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOpenFileA(szfilename : *const JET_CHAR, phffile : *mut JET_HANDLE, pulfilesizelow : *mut JET_UINT32, pulfilesizehigh : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOpenFileInstanceA(instance : JET_INSTANCE, szfilename : *const JET_CHAR, phffile : *mut JET_HANDLE, pulfilesizelow : *mut JET_UINT32, pulfilesizehigh : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOpenFileInstanceW(instance : JET_INSTANCE, szfilename : *const JET_WCHAR, phffile : *mut JET_HANDLE, pulfilesizelow : *mut JET_UINT32, pulfilesizehigh : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOpenFileW(szfilename : *const JET_WCHAR, phffile : *mut JET_HANDLE, pulfilesizelow : *mut JET_UINT32, pulfilesizehigh : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOpenTableA(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_CHAR, pvparameters : JET_PCVOID, cbparameters : JET_UINT32, grbit : JET_GRBIT, ptableid : *mut JET_TABLEID) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOpenTableW(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_WCHAR, pvparameters : JET_PCVOID, cbparameters : JET_UINT32, grbit : JET_GRBIT, ptableid : *mut JET_TABLEID) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOpenTempTable(sesid : JET_SESID, prgcolumndef : *const JET_COLUMNDEF, ccolumn : JET_UINT32, grbit : JET_GRBIT, ptableid : *mut JET_TABLEID, prgcolumnid : *mut JET_COLUMNID) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOpenTempTable2(sesid : JET_SESID, prgcolumndef : *const JET_COLUMNDEF, ccolumn : JET_UINT32, lcid : JET_LCID, grbit : JET_GRBIT, ptableid : *mut JET_TABLEID, prgcolumnid : *mut JET_COLUMNID) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOpenTempTable3(sesid : JET_SESID, prgcolumndef : *const JET_COLUMNDEF, ccolumn : JET_UINT32, pidxunicode : *const JET_UNICODEINDEX, grbit : JET_GRBIT, ptableid : *mut JET_TABLEID, prgcolumnid : *mut JET_COLUMNID) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOpenTemporaryTable(sesid : JET_SESID, popentemporarytable : *const JET_OPENTEMPORARYTABLE) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetOpenTemporaryTable2(sesid : JET_SESID, popentemporarytable : *const JET_OPENTEMPORARYTABLE2) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetPrepareUpdate(sesid : JET_SESID, tableid : JET_TABLEID, prep : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetPrereadIndexRanges(sesid : JET_SESID, tableid : JET_TABLEID, rgindexranges : *const JET_INDEX_RANGE, cindexranges : JET_UINT32, pcrangespreread : *mut JET_UINT32, rgcolumnidpreread : *const JET_COLUMNID, ccolumnidpreread : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetPrereadKeys(sesid : JET_SESID, tableid : JET_TABLEID, rgpvkeys : *const JET_PCVOID, rgcbkeys : *const JET_UINT32, ckeys : JET_INT32, pckeyspreread : *mut JET_INT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetReadFile(hffile : JET_HANDLE, pv : JET_PVOID, cb : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetReadFileInstance(instance : JET_INSTANCE, hffile : JET_HANDLE, pv : JET_PVOID, cb : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetRegisterCallback(sesid : JET_SESID, tableid : JET_TABLEID, cbtyp : JET_CBTYP, pcallback : JET_CALLBACK, pvcontext : JET_PVOID, phcallbackid : *const JET_HANDLE) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetRenameColumnA(sesid : JET_SESID, tableid : JET_TABLEID, szname : *const JET_CHAR, sznamenew : *const JET_CHAR, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetRenameColumnW(sesid : JET_SESID, tableid : JET_TABLEID, szname : *const JET_WCHAR, sznamenew : *const JET_WCHAR, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetRenameTableA(sesid : JET_SESID, dbid : JET_DBID, szname : *const JET_CHAR, sznamenew : *const JET_CHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetRenameTableW(sesid : JET_SESID, dbid : JET_DBID, szname : *const JET_WCHAR, sznamenew : *const JET_WCHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetResetSessionContext(sesid : JET_SESID) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetResetTableSequential(sesid : JET_SESID, tableid : JET_TABLEID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetResizeDatabase(sesid : JET_SESID, dbid : JET_DBID, cpgtarget : JET_UINT32, pcpgactual : *mut JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetRestore2A(sz : *const JET_CHAR, szdest : *const JET_CHAR, pfn : JET_PFNSTATUS) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetRestore2W(sz : *const JET_WCHAR, szdest : *const JET_WCHAR, pfn : JET_PFNSTATUS) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetRestoreA(szsource : *const JET_CHAR, pfn : JET_PFNSTATUS) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetRestoreInstanceA(instance : JET_INSTANCE, sz : *const JET_CHAR, szdest : *const JET_CHAR, pfn : JET_PFNSTATUS) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetRestoreInstanceW(instance : JET_INSTANCE, sz : *const JET_WCHAR, szdest : *const JET_WCHAR, pfn : JET_PFNSTATUS) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetRestoreW(szsource : *const JET_WCHAR, pfn : JET_PFNSTATUS) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetRetrieveColumn(sesid : JET_SESID, tableid : JET_TABLEID, columnid : JET_COLUMNID, pvdata : JET_PVOID, cbdata : JET_UINT32, pcbactual : *mut JET_UINT32, grbit : JET_GRBIT, pretinfo : *mut JET_RETINFO) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetRetrieveColumns(sesid : JET_SESID, tableid : JET_TABLEID, pretrievecolumn : *mut JET_RETRIEVECOLUMN, cretrievecolumn : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetRetrieveKey(sesid : JET_SESID, tableid : JET_TABLEID, pvkey : JET_PVOID, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetRollback(sesid : JET_SESID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSeek(sesid : JET_SESID, tableid : JET_TABLEID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetColumn(sesid : JET_SESID, tableid : JET_TABLEID, columnid : JET_COLUMNID, pvdata : JET_PCVOID, cbdata : JET_UINT32, grbit : JET_GRBIT, psetinfo : *const JET_SETINFO) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetColumnDefaultValueA(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_CHAR, szcolumnname : *const JET_CHAR, pvdata : JET_PCVOID, cbdata : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetColumnDefaultValueW(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_WCHAR, szcolumnname : *const JET_WCHAR, pvdata : JET_PCVOID, cbdata : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetColumns(sesid : JET_SESID, tableid : JET_TABLEID, psetcolumn : *const JET_SETCOLUMN, csetcolumn : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetCurrentIndex2A(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_CHAR, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetCurrentIndex2W(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_WCHAR, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetCurrentIndex3A(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_CHAR, grbit : JET_GRBIT, itagsequence : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetCurrentIndex3W(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_WCHAR, grbit : JET_GRBIT, itagsequence : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetCurrentIndex4A(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_CHAR, pindexid : *const JET_INDEXID, grbit : JET_GRBIT, itagsequence : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetCurrentIndex4W(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_WCHAR, pindexid : *const JET_INDEXID, grbit : JET_GRBIT, itagsequence : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetCurrentIndexA(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_CHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetCurrentIndexW(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_WCHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetCursorFilter(sesid : JET_SESID, tableid : JET_TABLEID, rgcolumnfilters : *const JET_INDEX_COLUMN, ccolumnfilters : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetDatabaseSizeA(sesid : JET_SESID, szdatabasename : *const JET_CHAR, cpg : JET_UINT32, pcpgreal : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetDatabaseSizeW(sesid : JET_SESID, szdatabasename : *const JET_WCHAR, cpg : JET_UINT32, pcpgreal : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetIndexRange(sesid : JET_SESID, tableidsrc : JET_TABLEID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetLS(sesid : JET_SESID, tableid : JET_TABLEID, ls : JET_LS, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetSessionContext(sesid : JET_SESID, ulcontext : JET_API_PTR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetSessionParameter(sesid : JET_SESID, sesparamid : JET_UINT32, pvparam : JET_PVOID, cbparam : JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetSystemParameterA(pinstance : *mut JET_INSTANCE, sesid : JET_SESID, paramid : JET_UINT32, lparam : JET_API_PTR, szparam : *const JET_CHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetSystemParameterW(pinstance : *mut JET_INSTANCE, sesid : JET_SESID, paramid : JET_UINT32, lparam : JET_API_PTR, szparam : *const JET_WCHAR) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetSetTableSequential(sesid : JET_SESID, tableid : JET_TABLEID, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetStopBackup() -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetStopBackupInstance(instance : JET_INSTANCE) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetStopService() -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetStopServiceInstance(instance : JET_INSTANCE) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetStopServiceInstance2(instance : JET_INSTANCE, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetTerm(instance : JET_INSTANCE) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetTerm2(instance : JET_INSTANCE, grbit : JET_GRBIT) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetTruncateLog() -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetTruncateLogInstance(instance : JET_INSTANCE) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetUnregisterCallback(sesid : JET_SESID, tableid : JET_TABLEID, cbtyp : JET_CBTYP, hcallbackid : JET_HANDLE) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetUpdate(sesid : JET_SESID, tableid : JET_TABLEID, pvbookmark : JET_PVOID, cbbookmark : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
windows_link::link!("esent.dll" "system" fn JetUpdate2(sesid : JET_SESID, tableid : JET_TABLEID, pvbookmark : JET_PVOID, cbbookmark : JET_UINT32, pcbactual : *mut JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
#[cfg(target_arch = "x86")]
pub type JET_API_PTR = JET_UINT32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type JET_API_PTR = JET_UINT64;
pub const JET_BASE_NAME_LENGTH: u32 = 3;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct JET_BKINFO {
    pub lgposMark: JET_LGPOS,
    pub Anonymous: JET_BKINFO_0,
    pub genLow: JET_UINT32,
    pub genHigh: JET_UINT32,
}
impl Default for JET_BKINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_BKINFO_0 {
    pub logtimeMark: JET_LOGTIME,
    pub bklogtimeMark: JET_BKLOGTIME,
}
impl Default for JET_BKINFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_BKLOGTIME {
    pub bSeconds: JET_INT8,
    pub bMinutes: JET_INT8,
    pub bHours: JET_INT8,
    pub bDay: JET_INT8,
    pub bMonth: JET_INT8,
    pub bYear: JET_INT8,
    pub Anonymous: JET_BKLOGTIME_0,
    pub Anonymous2: JET_BKLOGTIME_1,
}
impl Default for JET_BKLOGTIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_BKLOGTIME_0 {
    pub bFiller1: JET_BYTE,
    pub Anonymous: JET_BKLOGTIME_0_0,
}
impl Default for JET_BKLOGTIME_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_BKLOGTIME_0_0 {
    pub _bitfield: JET_BYTE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_BKLOGTIME_1 {
    pub bFiller2: JET_BYTE,
    pub Anonymous: JET_BKLOGTIME_1_0,
}
impl Default for JET_BKLOGTIME_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_BKLOGTIME_1_0 {
    pub _bitfield: JET_BYTE,
}
pub type JET_BYTE = u8;
pub type JET_CALLBACK = Option<unsafe extern "system" fn(sesid: JET_SESID, dbid: JET_DBID, tableid: JET_TABLEID, cbtyp: JET_CBTYP, pvarg1: JET_PVOID, pvarg2: JET_PVOID, pvcontext: JET_PVOID, ulunused: JET_API_PTR) -> JET_ERR>;
pub type JET_CBTYP = JET_UINT32;
pub type JET_CHAR = i8;
pub type JET_COLTYP = JET_UINT32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_COLUMNBASE_A {
    pub cbStruct: JET_UINT32,
    pub columnid: JET_COLUMNID,
    pub coltyp: JET_COLTYP,
    pub wCountry: JET_UINT16,
    pub langid: JET_LANGID,
    pub cp: JET_CP,
    pub wFiller: JET_UINT16,
    pub cbMax: JET_UINT32,
    pub grbit: JET_GRBIT,
    pub szBaseTableName: [JET_CHAR; 256],
    pub szBaseColumnName: [JET_CHAR; 256],
}
impl Default for JET_COLUMNBASE_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_COLUMNBASE_W {
    pub cbStruct: JET_UINT32,
    pub columnid: JET_COLUMNID,
    pub coltyp: JET_COLTYP,
    pub wCountry: JET_UINT16,
    pub langid: JET_LANGID,
    pub cp: JET_CP,
    pub wFiller: JET_UINT16,
    pub cbMax: JET_UINT32,
    pub grbit: JET_GRBIT,
    pub szBaseTableName: [JET_WCHAR; 256],
    pub szBaseColumnName: [JET_WCHAR; 256],
}
impl Default for JET_COLUMNBASE_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_COLUMNCREATE_A {
    pub cbStruct: JET_UINT32,
    pub szColumnName: JET_PSTR,
    pub coltyp: JET_COLTYP,
    pub cbMax: JET_UINT32,
    pub grbit: JET_GRBIT,
    pub pvDefault: JET_PVOID,
    pub cbDefault: JET_UINT32,
    pub cp: JET_UINT32,
    pub columnid: JET_COLUMNID,
    pub err: JET_ERR,
}
impl Default for JET_COLUMNCREATE_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_COLUMNCREATE_W {
    pub cbStruct: JET_UINT32,
    pub szColumnName: JET_PWSTR,
    pub coltyp: JET_COLTYP,
    pub cbMax: JET_UINT32,
    pub grbit: JET_GRBIT,
    pub pvDefault: JET_PVOID,
    pub cbDefault: JET_UINT32,
    pub cp: JET_UINT32,
    pub columnid: JET_COLUMNID,
    pub err: JET_ERR,
}
impl Default for JET_COLUMNCREATE_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_COLUMNDEF {
    pub cbStruct: JET_UINT32,
    pub columnid: JET_COLUMNID,
    pub coltyp: JET_COLTYP,
    pub wCountry: JET_UINT16,
    pub langid: JET_LANGID,
    pub cp: JET_CP,
    pub wCollate: JET_UINT16,
    pub cbMax: JET_UINT32,
    pub grbit: JET_GRBIT,
}
pub type JET_COLUMNID = JET_UINT32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_COLUMNLIST {
    pub cbStruct: JET_UINT32,
    pub tableid: JET_TABLEID,
    pub cRecord: JET_UINT32,
    pub columnidPresentationOrder: JET_COLUMNID,
    pub columnidcolumnname: JET_COLUMNID,
    pub columnidcolumnid: JET_COLUMNID,
    pub columnidcoltyp: JET_COLUMNID,
    pub columnidCountry: JET_COLUMNID,
    pub columnidLangid: JET_COLUMNID,
    pub columnidCp: JET_COLUMNID,
    pub columnidCollate: JET_COLUMNID,
    pub columnidcbMax: JET_COLUMNID,
    pub columnidgrbit: JET_COLUMNID,
    pub columnidDefault: JET_COLUMNID,
    pub columnidBaseTableName: JET_COLUMNID,
    pub columnidBaseColumnName: JET_COLUMNID,
    pub columnidDefinitionName: JET_COLUMNID,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct JET_COMMIT_ID {
    pub signLog: JET_SIGNATURE,
    pub reserved: JET_INT32,
    pub commitId: JET_INT64,
}
#[cfg(target_arch = "x86")]
impl Default for JET_COMMIT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct JET_COMMIT_ID {
    pub signLog: JET_SIGNATURE,
    pub reserved: JET_INT32,
    pub commitId: JET_INT64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for JET_COMMIT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_CONDITIONALCOLUMN_A {
    pub cbStruct: JET_UINT32,
    pub szColumnName: JET_PSTR,
    pub grbit: JET_GRBIT,
}
impl Default for JET_CONDITIONALCOLUMN_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_CONDITIONALCOLUMN_W {
    pub cbStruct: JET_UINT32,
    pub szColumnName: JET_PWSTR,
    pub grbit: JET_GRBIT,
}
impl Default for JET_CONDITIONALCOLUMN_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_CONVERT_A {
    pub szOldDll: JET_PSTR,
    pub Anonymous: JET_CONVERT_A_0,
}
impl Default for JET_CONVERT_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_CONVERT_A_0 {
    pub fFlags: JET_UINT32,
    pub Anonymous: JET_CONVERT_A_0_0,
}
impl Default for JET_CONVERT_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_CONVERT_A_0_0 {
    pub _bitfield: JET_UINT32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_CONVERT_W {
    pub szOldDll: JET_PWSTR,
    pub Anonymous: JET_CONVERT_W_0,
}
impl Default for JET_CONVERT_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_CONVERT_W_0 {
    pub fFlags: JET_UINT32,
    pub Anonymous: JET_CONVERT_W_0_0,
}
impl Default for JET_CONVERT_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_CONVERT_W_0_0 {
    pub _bitfield: JET_UINT32,
}
pub type JET_CP = JET_UINT16;
pub const JET_ColInfo: u32 = 0;
pub const JET_ColInfoBase: u32 = 4;
pub const JET_ColInfoBaseByColid: u32 = 8;
pub const JET_ColInfoByColid: u32 = 6;
pub const JET_ColInfoGrbitMinimalInfo: u32 = 1073741824;
pub const JET_ColInfoGrbitNonDerivedColumnsOnly: u32 = 2147483648;
pub const JET_ColInfoGrbitSortByColumnid: u32 = 536870912;
pub const JET_ColInfoList: u32 = 1;
pub const JET_ColInfoListCompact: u32 = 5;
pub const JET_ColInfoListSortColumnid: u32 = 7;
pub const JET_ColInfoSysTabCursor: u32 = 3;
pub type JET_DBID = JET_UINT32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_DBINFOMISC {
    pub ulVersion: JET_UINT32,
    pub ulUpdate: JET_UINT32,
    pub signDb: JET_SIGNATURE,
    pub dbstate: JET_UINT32,
    pub lgposConsistent: JET_LGPOS,
    pub logtimeConsistent: JET_LOGTIME,
    pub logtimeAttach: JET_LOGTIME,
    pub lgposAttach: JET_LGPOS,
    pub logtimeDetach: JET_LOGTIME,
    pub lgposDetach: JET_LGPOS,
    pub signLog: JET_SIGNATURE,
    pub bkinfoFullPrev: JET_BKINFO,
    pub bkinfoIncPrev: JET_BKINFO,
    pub bkinfoFullCur: JET_BKINFO,
    pub fShadowingDisabled: JET_UINT32,
    pub fUpgradeDb: JET_UINT32,
    pub dwMajorVersion: JET_UINT32,
    pub dwMinorVersion: JET_UINT32,
    pub dwBuildNumber: JET_UINT32,
    pub lSPNumber: JET_INT32,
    pub cbPageSize: JET_UINT32,
}
impl Default for JET_DBINFOMISC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_DBINFOMISC2 {
    pub ulVersion: JET_UINT32,
    pub ulUpdate: JET_UINT32,
    pub signDb: JET_SIGNATURE,
    pub dbstate: JET_UINT32,
    pub lgposConsistent: JET_LGPOS,
    pub logtimeConsistent: JET_LOGTIME,
    pub logtimeAttach: JET_LOGTIME,
    pub lgposAttach: JET_LGPOS,
    pub logtimeDetach: JET_LOGTIME,
    pub lgposDetach: JET_LGPOS,
    pub signLog: JET_SIGNATURE,
    pub bkinfoFullPrev: JET_BKINFO,
    pub bkinfoIncPrev: JET_BKINFO,
    pub bkinfoFullCur: JET_BKINFO,
    pub fShadowingDisabled: JET_UINT32,
    pub fUpgradeDb: JET_UINT32,
    pub dwMajorVersion: JET_UINT32,
    pub dwMinorVersion: JET_UINT32,
    pub dwBuildNumber: JET_UINT32,
    pub lSPNumber: JET_INT32,
    pub cbPageSize: JET_UINT32,
    pub genMinRequired: JET_UINT32,
    pub genMaxRequired: JET_UINT32,
    pub logtimeGenMaxCreate: JET_LOGTIME,
    pub ulRepairCount: JET_UINT32,
    pub logtimeRepair: JET_LOGTIME,
    pub ulRepairCountOld: JET_UINT32,
    pub ulECCFixSuccess: JET_UINT32,
    pub logtimeECCFixSuccess: JET_LOGTIME,
    pub ulECCFixSuccessOld: JET_UINT32,
    pub ulECCFixFail: JET_UINT32,
    pub logtimeECCFixFail: JET_LOGTIME,
    pub ulECCFixFailOld: JET_UINT32,
    pub ulBadChecksum: JET_UINT32,
    pub logtimeBadChecksum: JET_LOGTIME,
    pub ulBadChecksumOld: JET_UINT32,
}
impl Default for JET_DBINFOMISC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_DBINFOMISC3 {
    pub ulVersion: JET_UINT32,
    pub ulUpdate: JET_UINT32,
    pub signDb: JET_SIGNATURE,
    pub dbstate: JET_UINT32,
    pub lgposConsistent: JET_LGPOS,
    pub logtimeConsistent: JET_LOGTIME,
    pub logtimeAttach: JET_LOGTIME,
    pub lgposAttach: JET_LGPOS,
    pub logtimeDetach: JET_LOGTIME,
    pub lgposDetach: JET_LGPOS,
    pub signLog: JET_SIGNATURE,
    pub bkinfoFullPrev: JET_BKINFO,
    pub bkinfoIncPrev: JET_BKINFO,
    pub bkinfoFullCur: JET_BKINFO,
    pub fShadowingDisabled: JET_UINT32,
    pub fUpgradeDb: JET_UINT32,
    pub dwMajorVersion: JET_UINT32,
    pub dwMinorVersion: JET_UINT32,
    pub dwBuildNumber: JET_UINT32,
    pub lSPNumber: JET_INT32,
    pub cbPageSize: JET_UINT32,
    pub genMinRequired: JET_UINT32,
    pub genMaxRequired: JET_UINT32,
    pub logtimeGenMaxCreate: JET_LOGTIME,
    pub ulRepairCount: JET_UINT32,
    pub logtimeRepair: JET_LOGTIME,
    pub ulRepairCountOld: JET_UINT32,
    pub ulECCFixSuccess: JET_UINT32,
    pub logtimeECCFixSuccess: JET_LOGTIME,
    pub ulECCFixSuccessOld: JET_UINT32,
    pub ulECCFixFail: JET_UINT32,
    pub logtimeECCFixFail: JET_LOGTIME,
    pub ulECCFixFailOld: JET_UINT32,
    pub ulBadChecksum: JET_UINT32,
    pub logtimeBadChecksum: JET_LOGTIME,
    pub ulBadChecksumOld: JET_UINT32,
    pub genCommitted: JET_UINT32,
}
impl Default for JET_DBINFOMISC3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_DBINFOMISC4 {
    pub ulVersion: JET_UINT32,
    pub ulUpdate: JET_UINT32,
    pub signDb: JET_SIGNATURE,
    pub dbstate: JET_UINT32,
    pub lgposConsistent: JET_LGPOS,
    pub logtimeConsistent: JET_LOGTIME,
    pub logtimeAttach: JET_LOGTIME,
    pub lgposAttach: JET_LGPOS,
    pub logtimeDetach: JET_LOGTIME,
    pub lgposDetach: JET_LGPOS,
    pub signLog: JET_SIGNATURE,
    pub bkinfoFullPrev: JET_BKINFO,
    pub bkinfoIncPrev: JET_BKINFO,
    pub bkinfoFullCur: JET_BKINFO,
    pub fShadowingDisabled: JET_UINT32,
    pub fUpgradeDb: JET_UINT32,
    pub dwMajorVersion: JET_UINT32,
    pub dwMinorVersion: JET_UINT32,
    pub dwBuildNumber: JET_UINT32,
    pub lSPNumber: JET_INT32,
    pub cbPageSize: JET_UINT32,
    pub genMinRequired: JET_UINT32,
    pub genMaxRequired: JET_UINT32,
    pub logtimeGenMaxCreate: JET_LOGTIME,
    pub ulRepairCount: JET_UINT32,
    pub logtimeRepair: JET_LOGTIME,
    pub ulRepairCountOld: JET_UINT32,
    pub ulECCFixSuccess: JET_UINT32,
    pub logtimeECCFixSuccess: JET_LOGTIME,
    pub ulECCFixSuccessOld: JET_UINT32,
    pub ulECCFixFail: JET_UINT32,
    pub logtimeECCFixFail: JET_LOGTIME,
    pub ulECCFixFailOld: JET_UINT32,
    pub ulBadChecksum: JET_UINT32,
    pub logtimeBadChecksum: JET_LOGTIME,
    pub ulBadChecksumOld: JET_UINT32,
    pub genCommitted: JET_UINT32,
    pub bkinfoCopyPrev: JET_BKINFO,
    pub bkinfoDiffPrev: JET_BKINFO,
}
impl Default for JET_DBINFOMISC4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_DBINFOUPGRADE {
    pub cbStruct: JET_UINT32,
    pub cbFilesizeLow: JET_UINT32,
    pub cbFilesizeHigh: JET_UINT32,
    pub cbFreeSpaceRequiredLow: JET_UINT32,
    pub cbFreeSpaceRequiredHigh: JET_UINT32,
    pub csecToUpgrade: JET_UINT32,
    pub Anonymous: JET_DBINFOUPGRADE_0,
}
impl Default for JET_DBINFOUPGRADE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_DBINFOUPGRADE_0 {
    pub ulFlags: JET_UINT32,
    pub Anonymous: JET_DBINFOUPGRADE_0_0,
}
impl Default for JET_DBINFOUPGRADE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_DBINFOUPGRADE_0_0 {
    pub _bitfield: JET_UINT32,
}
pub const JET_DbInfoCollate: u32 = 5;
pub const JET_DbInfoConnect: u32 = 1;
pub const JET_DbInfoCountry: u32 = 2;
pub const JET_DbInfoCp: u32 = 4;
pub const JET_DbInfoDBInUse: u32 = 15;
pub const JET_DbInfoFileType: u32 = 19;
pub const JET_DbInfoFilename: u32 = 0;
pub const JET_DbInfoFilesize: u32 = 10;
pub const JET_DbInfoFilesizeOnDisk: u32 = 21;
pub const JET_DbInfoIsam: u32 = 9;
pub const JET_DbInfoLCID: u32 = 3;
pub const JET_DbInfoLangid: u32 = 3;
pub const JET_DbInfoMisc: u32 = 14;
pub const JET_DbInfoOptions: u32 = 6;
pub const JET_DbInfoPageSize: u32 = 17;
pub const JET_DbInfoSpaceAvailable: u32 = 12;
pub const JET_DbInfoSpaceOwned: u32 = 11;
pub const JET_DbInfoTransactions: u32 = 7;
pub const JET_DbInfoUpgrade: u32 = 13;
pub const JET_DbInfoVersion: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_ENUMCOLUMN {
    pub columnid: JET_COLUMNID,
    pub err: JET_ERR,
    pub Anonymous: JET_ENUMCOLUMN_0,
}
impl Default for JET_ENUMCOLUMN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_ENUMCOLUMN_0 {
    pub Anonymous: JET_ENUMCOLUMN_0_0,
    pub Anonymous2: JET_ENUMCOLUMN_0_1,
}
impl Default for JET_ENUMCOLUMN_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_ENUMCOLUMN_0_0 {
    pub cEnumColumnValue: JET_UINT32,
    pub rgEnumColumnValue: *mut JET_ENUMCOLUMNVALUE,
}
impl Default for JET_ENUMCOLUMN_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_ENUMCOLUMN_0_1 {
    pub cbData: JET_UINT32,
    pub pvData: JET_PVOID,
}
impl Default for JET_ENUMCOLUMN_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_ENUMCOLUMNID {
    pub columnid: JET_COLUMNID,
    pub ctagSequence: JET_UINT32,
    pub rgtagSequence: *mut JET_UINT32,
}
impl Default for JET_ENUMCOLUMNID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_ENUMCOLUMNVALUE {
    pub itagSequence: JET_UINT32,
    pub err: JET_ERR,
    pub cbData: JET_UINT32,
    pub pvData: JET_PVOID,
}
impl Default for JET_ENUMCOLUMNVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type JET_ERR = JET_INT32;
pub type JET_ERRCAT = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_ERRINFOBASIC_W {
    pub cbStruct: JET_UINT32,
    pub errValue: JET_ERR,
    pub errcatMostSpecific: JET_ERRCAT,
    pub rgCategoricalHierarchy: [JET_BYTE; 8],
    pub lSourceLine: JET_UINT32,
    pub rgszSourceFile: [JET_WCHAR; 64],
}
impl Default for JET_ERRINFOBASIC_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const JET_ErrorInfoSpecificErr: u32 = 1;
pub const JET_EventLoggingDisable: u32 = 0;
pub const JET_EventLoggingLevelHigh: u32 = 75;
pub const JET_EventLoggingLevelLow: u32 = 25;
pub const JET_EventLoggingLevelMax: u32 = 100;
pub const JET_EventLoggingLevelMedium: u32 = 50;
pub const JET_EventLoggingLevelMin: u32 = 1;
pub const JET_ExceptionFailFast: u32 = 4;
pub const JET_ExceptionMsgBox: u32 = 1;
pub const JET_ExceptionNone: u32 = 2;
pub type JET_GRBIT = JET_UINT32;
pub type JET_HANDLE = JET_API_PTR;
pub type JET_INDEXCHECKING = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_INDEXCREATE2_A {
    pub cbStruct: JET_UINT32,
    pub szIndexName: JET_PSTR,
    pub szKey: JET_PSTR,
    pub cbKey: JET_UINT32,
    pub grbit: JET_GRBIT,
    pub ulDensity: JET_UINT32,
    pub Anonymous: JET_INDEXCREATE2_A_0,
    pub Anonymous2: JET_INDEXCREATE2_A_1,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_A,
    pub cConditionalColumn: JET_UINT32,
    pub err: JET_ERR,
    pub cbKeyMost: JET_UINT32,
    pub pSpacehints: *mut JET_SPACEHINTS,
}
impl Default for JET_INDEXCREATE2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE2_A_0 {
    pub lcid: JET_LCID,
    pub pidxunicode: *mut JET_UNICODEINDEX,
}
impl Default for JET_INDEXCREATE2_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE2_A_1 {
    pub cbVarSegMac: JET_UINT32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl Default for JET_INDEXCREATE2_A_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_INDEXCREATE2_W {
    pub cbStruct: JET_UINT32,
    pub szIndexName: JET_PWSTR,
    pub szKey: JET_PWSTR,
    pub cbKey: JET_UINT32,
    pub grbit: JET_GRBIT,
    pub ulDensity: JET_UINT32,
    pub Anonymous: JET_INDEXCREATE2_W_0,
    pub Anonymous2: JET_INDEXCREATE2_W_1,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    pub cConditionalColumn: JET_UINT32,
    pub err: JET_ERR,
    pub cbKeyMost: JET_UINT32,
    pub pSpacehints: *mut JET_SPACEHINTS,
}
impl Default for JET_INDEXCREATE2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE2_W_0 {
    pub lcid: JET_LCID,
    pub pidxunicode: *mut JET_UNICODEINDEX,
}
impl Default for JET_INDEXCREATE2_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE2_W_1 {
    pub cbVarSegMac: JET_UINT32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl Default for JET_INDEXCREATE2_W_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_INDEXCREATE3_A {
    pub cbStruct: JET_UINT32,
    pub szIndexName: JET_PSTR,
    pub szKey: JET_PSTR,
    pub cbKey: JET_UINT32,
    pub grbit: JET_GRBIT,
    pub ulDensity: JET_UINT32,
    pub pidxunicode: *mut JET_UNICODEINDEX2,
    pub Anonymous: JET_INDEXCREATE3_A_0,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_A,
    pub cConditionalColumn: JET_UINT32,
    pub err: JET_ERR,
    pub cbKeyMost: JET_UINT32,
    pub pSpacehints: *mut JET_SPACEHINTS,
}
impl Default for JET_INDEXCREATE3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE3_A_0 {
    pub cbVarSegMac: JET_UINT32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl Default for JET_INDEXCREATE3_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_INDEXCREATE3_W {
    pub cbStruct: JET_UINT32,
    pub szIndexName: JET_PWSTR,
    pub szKey: JET_PWSTR,
    pub cbKey: JET_UINT32,
    pub grbit: JET_GRBIT,
    pub ulDensity: JET_UINT32,
    pub pidxunicode: *mut JET_UNICODEINDEX2,
    pub Anonymous: JET_INDEXCREATE3_W_0,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    pub cConditionalColumn: JET_UINT32,
    pub err: JET_ERR,
    pub cbKeyMost: JET_UINT32,
    pub pSpacehints: *mut JET_SPACEHINTS,
}
impl Default for JET_INDEXCREATE3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE3_W_0 {
    pub cbVarSegMac: JET_UINT32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl Default for JET_INDEXCREATE3_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_INDEXCREATE_A {
    pub cbStruct: JET_UINT32,
    pub szIndexName: JET_PSTR,
    pub szKey: JET_PSTR,
    pub cbKey: JET_UINT32,
    pub grbit: JET_GRBIT,
    pub ulDensity: JET_UINT32,
    pub Anonymous: JET_INDEXCREATE_A_0,
    pub Anonymous2: JET_INDEXCREATE_A_1,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_A,
    pub cConditionalColumn: JET_UINT32,
    pub err: JET_ERR,
    pub cbKeyMost: JET_UINT32,
}
impl Default for JET_INDEXCREATE_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE_A_0 {
    pub lcid: JET_LCID,
    pub pidxunicode: *mut JET_UNICODEINDEX,
}
impl Default for JET_INDEXCREATE_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE_A_1 {
    pub cbVarSegMac: JET_UINT32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl Default for JET_INDEXCREATE_A_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_INDEXCREATE_W {
    pub cbStruct: JET_UINT32,
    pub szIndexName: JET_PWSTR,
    pub szKey: JET_PWSTR,
    pub cbKey: JET_UINT32,
    pub grbit: JET_GRBIT,
    pub ulDensity: JET_UINT32,
    pub Anonymous: JET_INDEXCREATE_W_0,
    pub Anonymous2: JET_INDEXCREATE_W_1,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    pub cConditionalColumn: JET_UINT32,
    pub err: JET_ERR,
    pub cbKeyMost: JET_UINT32,
}
impl Default for JET_INDEXCREATE_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE_W_0 {
    pub lcid: JET_LCID,
    pub pidxunicode: *mut JET_UNICODEINDEX,
}
impl Default for JET_INDEXCREATE_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE_W_1 {
    pub cbVarSegMac: JET_UINT32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl Default for JET_INDEXCREATE_W_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct JET_INDEXID {
    pub cbStruct: JET_UINT32,
    pub rgbIndexId: [JET_BYTE; 12],
}
#[cfg(target_arch = "x86")]
impl Default for JET_INDEXID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct JET_INDEXID {
    pub cbStruct: JET_UINT32,
    pub rgbIndexId: [JET_BYTE; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for JET_INDEXID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_INDEXLIST {
    pub cbStruct: JET_UINT32,
    pub tableid: JET_TABLEID,
    pub cRecord: JET_UINT32,
    pub columnidindexname: JET_COLUMNID,
    pub columnidgrbitIndex: JET_COLUMNID,
    pub columnidcKey: JET_COLUMNID,
    pub columnidcEntry: JET_COLUMNID,
    pub columnidcPage: JET_COLUMNID,
    pub columnidcColumn: JET_COLUMNID,
    pub columnidiColumn: JET_COLUMNID,
    pub columnidcolumnid: JET_COLUMNID,
    pub columnidcoltyp: JET_COLUMNID,
    pub columnidCountry: JET_COLUMNID,
    pub columnidLangid: JET_COLUMNID,
    pub columnidCp: JET_COLUMNID,
    pub columnidCollate: JET_COLUMNID,
    pub columnidgrbitColumn: JET_COLUMNID,
    pub columnidcolumnname: JET_COLUMNID,
    pub columnidLCMapFlags: JET_COLUMNID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_INDEXRANGE {
    pub cbStruct: JET_UINT32,
    pub tableid: JET_TABLEID,
    pub grbit: JET_GRBIT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_INDEX_COLUMN {
    pub columnid: JET_COLUMNID,
    pub relop: JET_RELOP,
    pub pv: JET_PVOID,
    pub cb: JET_UINT32,
    pub grbit: JET_GRBIT,
}
impl Default for JET_INDEX_COLUMN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_INDEX_RANGE {
    pub rgStartColumns: *mut JET_INDEX_COLUMN,
    pub cStartColumns: JET_UINT32,
    pub rgEndColumns: *mut JET_INDEX_COLUMN,
    pub cEndColumns: JET_UINT32,
}
impl Default for JET_INDEX_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type JET_INSTANCE = JET_API_PTR;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_INSTANCE_INFO_A {
    pub hInstanceId: JET_INSTANCE,
    pub szInstanceName: JET_PSTR,
    pub cDatabases: JET_API_PTR,
    pub szDatabaseFileName: *mut JET_PSTR,
    pub szDatabaseDisplayName: *mut JET_PSTR,
    pub szDatabaseSLVFileName_Obsolete: *mut JET_PSTR,
}
impl Default for JET_INSTANCE_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_INSTANCE_INFO_W {
    pub hInstanceId: JET_INSTANCE,
    pub szInstanceName: JET_PWSTR,
    pub cDatabases: JET_API_PTR,
    pub szDatabaseFileName: *mut JET_PWSTR,
    pub szDatabaseDisplayName: *mut JET_PWSTR,
    pub szDatabaseSLVFileName_Obsolete: *mut JET_PWSTR,
}
impl Default for JET_INSTANCE_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type JET_INT16 = i16;
pub type JET_INT32 = i32;
pub type JET_INT64 = i64;
pub type JET_INT8 = i8;
pub const JET_IOPriorityLow: u32 = 1;
pub const JET_IOPriorityNormal: u32 = 0;
pub const JET_IdxInfo: u32 = 0;
pub const JET_IdxInfoCount: u32 = 7;
pub const JET_IdxInfoCreateIndex: u32 = 11;
pub const JET_IdxInfoCreateIndex2: u32 = 12;
pub const JET_IdxInfoCreateIndex3: u32 = 13;
pub const JET_IdxInfoIndexId: u32 = 9;
pub const JET_IdxInfoKeyMost: u32 = 10;
pub const JET_IdxInfoLCID: u32 = 6;
pub const JET_IdxInfoLangid: u32 = 6;
pub const JET_IdxInfoList: u32 = 1;
pub const JET_IdxInfoLocaleName: u32 = 14;
pub const JET_IdxInfoOLC: u32 = 3;
pub const JET_IdxInfoResetOLC: u32 = 4;
pub const JET_IdxInfoSpaceAlloc: u32 = 5;
pub const JET_IdxInfoSysTabCursor: u32 = 2;
pub const JET_IdxInfoVarSegMac: u32 = 8;
pub const JET_IndexCheckingDeferToOpenTable: JET_INDEXCHECKING = 2;
pub const JET_IndexCheckingMax: JET_INDEXCHECKING = 3;
pub const JET_IndexCheckingOff: JET_INDEXCHECKING = 0;
pub const JET_IndexCheckingOn: JET_INDEXCHECKING = 1;
pub const JET_InstanceMiscInfoLogSignature: u32 = 0;
pub type JET_LANGID = JET_UINT16;
pub type JET_LCID = JET_UINT32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct JET_LGPOS {
    pub ib: JET_UINT16,
    pub isec: JET_UINT16,
    pub lGeneration: JET_INT32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_LOGINFO_A {
    pub cbSize: JET_UINT32,
    pub ulGenLow: JET_UINT32,
    pub ulGenHigh: JET_UINT32,
    pub szBaseName: [JET_CHAR; 4],
}
impl Default for JET_LOGINFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_LOGINFO_W {
    pub cbSize: JET_UINT32,
    pub ulGenLow: JET_UINT32,
    pub ulGenHigh: JET_UINT32,
    pub szBaseName: [JET_WCHAR; 4],
}
impl Default for JET_LOGINFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_LOGTIME {
    pub bSeconds: JET_INT8,
    pub bMinutes: JET_INT8,
    pub bHours: JET_INT8,
    pub bDay: JET_INT8,
    pub bMonth: JET_INT8,
    pub bYear: JET_INT8,
    pub Anonymous: JET_LOGTIME_0,
    pub Anonymous2: JET_LOGTIME_1,
}
impl Default for JET_LOGTIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_LOGTIME_0 {
    pub bFiller1: JET_BYTE,
    pub Anonymous: JET_LOGTIME_0_0,
}
impl Default for JET_LOGTIME_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_LOGTIME_0_0 {
    pub _bitfield: JET_BYTE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_LOGTIME_1 {
    pub bFiller2: JET_BYTE,
    pub Anonymous: JET_LOGTIME_1_0,
}
impl Default for JET_LOGTIME_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_LOGTIME_1_0 {
    pub _bitfield: JET_BYTE,
}
pub type JET_LS = JET_API_PTR;
pub const JET_LSNil: i32 = -1;
pub const JET_MAX_COMPUTERNAME_LENGTH: u32 = 15;
pub const JET_MoveFirst: u32 = 2147483648;
pub const JET_MoveLast: u32 = 2147483647;
pub const JET_MoveNext: u32 = 1;
pub const JET_MovePrevious: i32 = -1;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct JET_OBJECTINFO {
    pub cbStruct: JET_UINT32,
    pub objtyp: JET_OBJTYP,
    pub dtCreate: f64,
    pub dtUpdate: f64,
    pub grbit: JET_GRBIT,
    pub flags: JET_UINT32,
    pub cRecord: JET_UINT32,
    pub cPage: JET_UINT32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct JET_OBJECTINFO {
    pub cbStruct: JET_UINT32,
    pub objtyp: JET_OBJTYP,
    pub dtCreate: f64,
    pub dtUpdate: f64,
    pub grbit: JET_GRBIT,
    pub flags: JET_UINT32,
    pub cRecord: JET_UINT32,
    pub cPage: JET_UINT32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_OBJECTLIST {
    pub cbStruct: JET_UINT32,
    pub tableid: JET_TABLEID,
    pub cRecord: JET_UINT32,
    pub columnidcontainername: JET_COLUMNID,
    pub columnidobjectname: JET_COLUMNID,
    pub columnidobjtyp: JET_COLUMNID,
    pub columniddtCreate: JET_COLUMNID,
    pub columniddtUpdate: JET_COLUMNID,
    pub columnidgrbit: JET_COLUMNID,
    pub columnidflags: JET_COLUMNID,
    pub columnidcRecord: JET_COLUMNID,
    pub columnidcPage: JET_COLUMNID,
}
pub type JET_OBJTYP = JET_UINT32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_OPENTEMPORARYTABLE {
    pub cbStruct: JET_UINT32,
    pub prgcolumndef: *const JET_COLUMNDEF,
    pub ccolumn: JET_UINT32,
    pub pidxunicode: *mut JET_UNICODEINDEX,
    pub grbit: JET_GRBIT,
    pub prgcolumnid: *mut JET_COLUMNID,
    pub cbKeyMost: JET_UINT32,
    pub cbVarSegMac: JET_UINT32,
    pub tableid: JET_TABLEID,
}
impl Default for JET_OPENTEMPORARYTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_OPENTEMPORARYTABLE2 {
    pub cbStruct: JET_UINT32,
    pub prgcolumndef: *const JET_COLUMNDEF,
    pub ccolumn: JET_UINT32,
    pub pidxunicode: *mut JET_UNICODEINDEX2,
    pub grbit: JET_GRBIT,
    pub prgcolumnid: *mut JET_COLUMNID,
    pub cbKeyMost: JET_UINT32,
    pub cbVarSegMac: JET_UINT32,
    pub tableid: JET_TABLEID,
}
impl Default for JET_OPENTEMPORARYTABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_OPERATIONCONTEXT {
    pub ulUserID: JET_UINT32,
    pub nOperationID: JET_BYTE,
    pub nOperationType: JET_BYTE,
    pub nClientType: JET_BYTE,
    pub fFlags: JET_BYTE,
}
pub type JET_OSSNAPID = JET_API_PTR;
pub const JET_ObjInfo: u32 = 0;
pub const JET_ObjInfoList: u32 = 2;
pub const JET_ObjInfoListACM: u32 = 4;
pub const JET_ObjInfoListNoStats: u32 = 1;
pub const JET_ObjInfoMax: u32 = 8;
pub const JET_ObjInfoNoStats: u32 = 5;
pub const JET_ObjInfoRulesLoaded: u32 = 7;
pub const JET_ObjInfoSysTabCursor: u32 = 3;
pub const JET_ObjInfoSysTabReadOnly: u32 = 6;
pub const JET_OnlineDefragAll: u32 = 65535;
pub const JET_OnlineDefragAllOBSOLETE: u32 = 1;
pub const JET_OnlineDefragDatabases: u32 = 2;
pub const JET_OnlineDefragDisable: u32 = 0;
pub const JET_OnlineDefragSpaceTrees: u32 = 4;
pub type JET_PCSTR = *const JET_CHAR;
pub type JET_PCVOID = *const core::ffi::c_void;
pub type JET_PCWSTR = *const JET_WCHAR;
pub type JET_PFNDURABLECOMMITCALLBACK = Option<unsafe extern "system" fn(instance: JET_INSTANCE, pcommitidseen: *const JET_COMMIT_ID, grbit: JET_GRBIT) -> JET_ERR>;
pub type JET_PFNREALLOC = Option<unsafe extern "system" fn(pvcontext: JET_PVOID, pv: JET_PVOID, cb: JET_UINT32) -> JET_PVOID>;
pub type JET_PFNSTATUS = Option<unsafe extern "system" fn(sesid: JET_SESID, snp: JET_SNP, snt: JET_SNT, pv: JET_PVOID) -> JET_ERR>;
pub type JET_PSTR = *mut JET_CHAR;
pub type JET_PVOID = *mut core::ffi::c_void;
pub type JET_PWSTR = *mut JET_WCHAR;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_RECORDLIST {
    pub cbStruct: JET_UINT32,
    pub tableid: JET_TABLEID,
    pub cRecord: JET_UINT32,
    pub columnidBookmark: JET_COLUMNID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_RECPOS {
    pub cbStruct: JET_UINT32,
    pub centriesLT: JET_UINT32,
    pub centriesInRange: JET_UINT32,
    pub centriesTotal: JET_UINT32,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct JET_RECPOS2 {
    pub cbStruct: JET_UINT32,
    pub centriesLTDeprecated: JET_UINT32,
    pub centriesInRangeDeprecated: JET_UINT32,
    pub centriesTotalDeprecated: JET_UINT32,
    pub centriesLT: JET_UINT64,
    pub centriesTotal: JET_UINT64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct JET_RECPOS2 {
    pub cbStruct: JET_UINT32,
    pub centriesLTDeprecated: JET_UINT32,
    pub centriesInRangeDeprecated: JET_UINT32,
    pub centriesTotalDeprecated: JET_UINT32,
    pub centriesLT: JET_UINT64,
    pub centriesTotal: JET_UINT64,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct JET_RECSIZE {
    pub cbData: JET_UINT64,
    pub cbLongValueData: JET_UINT64,
    pub cbOverhead: JET_UINT64,
    pub cbLongValueOverhead: JET_UINT64,
    pub cNonTaggedColumns: JET_UINT64,
    pub cTaggedColumns: JET_UINT64,
    pub cLongValues: JET_UINT64,
    pub cMultiValues: JET_UINT64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct JET_RECSIZE {
    pub cbData: JET_UINT64,
    pub cbLongValueData: JET_UINT64,
    pub cbOverhead: JET_UINT64,
    pub cbLongValueOverhead: JET_UINT64,
    pub cNonTaggedColumns: JET_UINT64,
    pub cTaggedColumns: JET_UINT64,
    pub cLongValues: JET_UINT64,
    pub cMultiValues: JET_UINT64,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct JET_RECSIZE2 {
    pub cbData: JET_UINT64,
    pub cbLongValueData: JET_UINT64,
    pub cbOverhead: JET_UINT64,
    pub cbLongValueOverhead: JET_UINT64,
    pub cNonTaggedColumns: JET_UINT64,
    pub cTaggedColumns: JET_UINT64,
    pub cLongValues: JET_UINT64,
    pub cMultiValues: JET_UINT64,
    pub cCompressedColumns: JET_UINT64,
    pub cbDataCompressed: JET_UINT64,
    pub cbLongValueDataCompressed: JET_UINT64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct JET_RECSIZE2 {
    pub cbData: JET_UINT64,
    pub cbLongValueData: JET_UINT64,
    pub cbOverhead: JET_UINT64,
    pub cbLongValueOverhead: JET_UINT64,
    pub cNonTaggedColumns: JET_UINT64,
    pub cTaggedColumns: JET_UINT64,
    pub cLongValues: JET_UINT64,
    pub cMultiValues: JET_UINT64,
    pub cCompressedColumns: JET_UINT64,
    pub cbDataCompressed: JET_UINT64,
    pub cbLongValueDataCompressed: JET_UINT64,
}
pub type JET_RELOP = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_RETINFO {
    pub cbStruct: JET_UINT32,
    pub ibLongValue: JET_UINT32,
    pub itagSequence: JET_UINT32,
    pub columnidNextTagged: JET_COLUMNID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_RETRIEVECOLUMN {
    pub columnid: JET_COLUMNID,
    pub pvData: JET_PVOID,
    pub cbData: JET_UINT32,
    pub cbActual: JET_UINT32,
    pub grbit: JET_GRBIT,
    pub ibLongValue: JET_UINT32,
    pub itagSequence: JET_UINT32,
    pub columnidNextTagged: JET_COLUMNID,
    pub err: JET_ERR,
}
impl Default for JET_RETRIEVECOLUMN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_RSTINFO_A {
    pub cbStruct: JET_UINT32,
    pub rgrstmap: *mut JET_RSTMAP_A,
    pub crstmap: JET_INT32,
    pub lgposStop: JET_LGPOS,
    pub logtimeStop: JET_LOGTIME,
    pub pfnStatus: JET_PFNSTATUS,
}
impl Default for JET_RSTINFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_RSTINFO_W {
    pub cbStruct: JET_UINT32,
    pub rgrstmap: *mut JET_RSTMAP_W,
    pub crstmap: JET_INT32,
    pub lgposStop: JET_LGPOS,
    pub logtimeStop: JET_LOGTIME,
    pub pfnStatus: JET_PFNSTATUS,
}
impl Default for JET_RSTINFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_RSTMAP_A {
    pub szDatabaseName: JET_PSTR,
    pub szNewDatabaseName: JET_PSTR,
}
impl Default for JET_RSTMAP_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_RSTMAP_W {
    pub szDatabaseName: JET_PWSTR,
    pub szNewDatabaseName: JET_PWSTR,
}
impl Default for JET_RSTMAP_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type JET_SESID = JET_API_PTR;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_SETCOLUMN {
    pub columnid: JET_COLUMNID,
    pub pvData: JET_PCVOID,
    pub cbData: JET_UINT32,
    pub grbit: JET_GRBIT,
    pub ibLongValue: JET_UINT32,
    pub itagSequence: JET_UINT32,
    pub err: JET_ERR,
}
impl Default for JET_SETCOLUMN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_SETINFO {
    pub cbStruct: JET_UINT32,
    pub ibLongValue: JET_UINT32,
    pub itagSequence: JET_UINT32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_SETSYSPARAM_A {
    pub paramid: JET_UINT32,
    pub lParam: JET_API_PTR,
    pub sz: JET_PCSTR,
    pub err: JET_ERR,
}
impl Default for JET_SETSYSPARAM_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_SETSYSPARAM_W {
    pub paramid: JET_UINT32,
    pub lParam: JET_API_PTR,
    pub sz: JET_PCWSTR,
    pub err: JET_ERR,
}
impl Default for JET_SETSYSPARAM_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct JET_SIGNATURE {
    pub ulRandom: JET_UINT32,
    pub logtimeCreate: JET_LOGTIME,
    pub szComputerName: [JET_CHAR; 16],
}
impl Default for JET_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type JET_SNP = JET_UINT32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_SNPROG {
    pub cbStruct: JET_UINT32,
    pub cunitDone: JET_UINT32,
    pub cunitTotal: JET_UINT32,
}
pub type JET_SNT = JET_UINT32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_SPACEHINTS {
    pub cbStruct: JET_UINT32,
    pub ulInitialDensity: JET_UINT32,
    pub cbInitial: JET_UINT32,
    pub grbit: JET_GRBIT,
    pub ulMaintDensity: JET_UINT32,
    pub ulGrowth: JET_UINT32,
    pub cbMinExtent: JET_UINT32,
    pub cbMaxExtent: JET_UINT32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_TABLECREATE2_A {
    pub cbStruct: JET_UINT32,
    pub szTableName: JET_PSTR,
    pub szTemplateTableName: JET_PSTR,
    pub ulPages: JET_UINT32,
    pub ulDensity: JET_UINT32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_A,
    pub cColumns: JET_UINT32,
    pub rgindexcreate: *mut JET_INDEXCREATE_A,
    pub cIndexes: JET_UINT32,
    pub szCallback: JET_PSTR,
    pub cbtyp: JET_CBTYP,
    pub grbit: JET_GRBIT,
    pub tableid: JET_TABLEID,
    pub cCreated: JET_UINT32,
}
impl Default for JET_TABLECREATE2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_TABLECREATE2_W {
    pub cbStruct: JET_UINT32,
    pub szTableName: JET_PWSTR,
    pub szTemplateTableName: JET_PWSTR,
    pub ulPages: JET_UINT32,
    pub ulDensity: JET_UINT32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_W,
    pub cColumns: JET_UINT32,
    pub rgindexcreate: *mut JET_INDEXCREATE_W,
    pub cIndexes: JET_UINT32,
    pub szCallback: JET_PWSTR,
    pub cbtyp: JET_CBTYP,
    pub grbit: JET_GRBIT,
    pub tableid: JET_TABLEID,
    pub cCreated: JET_UINT32,
}
impl Default for JET_TABLECREATE2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_TABLECREATE3_A {
    pub cbStruct: JET_UINT32,
    pub szTableName: JET_PSTR,
    pub szTemplateTableName: JET_PSTR,
    pub ulPages: JET_UINT32,
    pub ulDensity: JET_UINT32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_A,
    pub cColumns: JET_UINT32,
    pub rgindexcreate: *mut JET_INDEXCREATE2_A,
    pub cIndexes: JET_UINT32,
    pub szCallback: JET_PSTR,
    pub cbtyp: JET_CBTYP,
    pub grbit: JET_GRBIT,
    pub pSeqSpacehints: *mut JET_SPACEHINTS,
    pub pLVSpacehints: *mut JET_SPACEHINTS,
    pub cbSeparateLV: JET_UINT32,
    pub tableid: JET_TABLEID,
    pub cCreated: JET_UINT32,
}
impl Default for JET_TABLECREATE3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_TABLECREATE3_W {
    pub cbStruct: JET_UINT32,
    pub szTableName: JET_PWSTR,
    pub szTemplateTableName: JET_PWSTR,
    pub ulPages: JET_UINT32,
    pub ulDensity: JET_UINT32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_W,
    pub cColumns: JET_UINT32,
    pub rgindexcreate: *mut JET_INDEXCREATE2_W,
    pub cIndexes: JET_UINT32,
    pub szCallback: JET_PWSTR,
    pub cbtyp: JET_CBTYP,
    pub grbit: JET_GRBIT,
    pub pSeqSpacehints: *mut JET_SPACEHINTS,
    pub pLVSpacehints: *mut JET_SPACEHINTS,
    pub cbSeparateLV: JET_UINT32,
    pub tableid: JET_TABLEID,
    pub cCreated: JET_UINT32,
}
impl Default for JET_TABLECREATE3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_TABLECREATE4_A {
    pub cbStruct: JET_UINT32,
    pub szTableName: JET_PSTR,
    pub szTemplateTableName: JET_PSTR,
    pub ulPages: JET_UINT32,
    pub ulDensity: JET_UINT32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_A,
    pub cColumns: JET_UINT32,
    pub rgindexcreate: *mut JET_INDEXCREATE3_A,
    pub cIndexes: JET_UINT32,
    pub szCallback: JET_PSTR,
    pub cbtyp: JET_CBTYP,
    pub grbit: JET_GRBIT,
    pub pSeqSpacehints: *mut JET_SPACEHINTS,
    pub pLVSpacehints: *mut JET_SPACEHINTS,
    pub cbSeparateLV: JET_UINT32,
    pub tableid: JET_TABLEID,
    pub cCreated: JET_UINT32,
}
impl Default for JET_TABLECREATE4_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_TABLECREATE4_W {
    pub cbStruct: JET_UINT32,
    pub szTableName: JET_PWSTR,
    pub szTemplateTableName: JET_PWSTR,
    pub ulPages: JET_UINT32,
    pub ulDensity: JET_UINT32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_W,
    pub cColumns: JET_UINT32,
    pub rgindexcreate: *mut JET_INDEXCREATE3_W,
    pub cIndexes: JET_UINT32,
    pub szCallback: JET_PWSTR,
    pub cbtyp: JET_CBTYP,
    pub grbit: JET_GRBIT,
    pub pSeqSpacehints: *mut JET_SPACEHINTS,
    pub pLVSpacehints: *mut JET_SPACEHINTS,
    pub cbSeparateLV: JET_UINT32,
    pub tableid: JET_TABLEID,
    pub cCreated: JET_UINT32,
}
impl Default for JET_TABLECREATE4_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_TABLECREATE_A {
    pub cbStruct: JET_UINT32,
    pub szTableName: JET_PSTR,
    pub szTemplateTableName: JET_PSTR,
    pub ulPages: JET_UINT32,
    pub ulDensity: JET_UINT32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_A,
    pub cColumns: JET_UINT32,
    pub rgindexcreate: *mut JET_INDEXCREATE_A,
    pub cIndexes: JET_UINT32,
    pub grbit: JET_GRBIT,
    pub tableid: JET_TABLEID,
    pub cCreated: JET_UINT32,
}
impl Default for JET_TABLECREATE_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_TABLECREATE_W {
    pub cbStruct: JET_UINT32,
    pub szTableName: JET_PWSTR,
    pub szTemplateTableName: JET_PWSTR,
    pub ulPages: JET_UINT32,
    pub ulDensity: JET_UINT32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_W,
    pub cColumns: JET_UINT32,
    pub rgindexcreate: *mut JET_INDEXCREATE_W,
    pub cIndexes: JET_UINT32,
    pub grbit: JET_GRBIT,
    pub tableid: JET_TABLEID,
    pub cCreated: JET_UINT32,
}
impl Default for JET_TABLECREATE_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type JET_TABLEID = JET_API_PTR;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_THREADSTATS {
    pub cbStruct: JET_UINT32,
    pub cPageReferenced: JET_UINT32,
    pub cPageRead: JET_UINT32,
    pub cPagePreread: JET_UINT32,
    pub cPageDirtied: JET_UINT32,
    pub cPageRedirtied: JET_UINT32,
    pub cLogRecord: JET_UINT32,
    pub cbLogRecord: JET_UINT32,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct JET_THREADSTATS2 {
    pub cbStruct: JET_UINT32,
    pub cPageReferenced: JET_UINT32,
    pub cPageRead: JET_UINT32,
    pub cPagePreread: JET_UINT32,
    pub cPageDirtied: JET_UINT32,
    pub cPageRedirtied: JET_UINT32,
    pub cLogRecord: JET_UINT32,
    pub cbLogRecord: JET_UINT32,
    pub cusecPageCacheMiss: JET_UINT64,
    pub cPageCacheMiss: JET_UINT32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct JET_THREADSTATS2 {
    pub cbStruct: JET_UINT32,
    pub cPageReferenced: JET_UINT32,
    pub cPageRead: JET_UINT32,
    pub cPagePreread: JET_UINT32,
    pub cPageDirtied: JET_UINT32,
    pub cPageRedirtied: JET_UINT32,
    pub cLogRecord: JET_UINT32,
    pub cbLogRecord: JET_UINT32,
    pub cusecPageCacheMiss: JET_UINT64,
    pub cPageCacheMiss: JET_UINT32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_TUPLELIMITS {
    pub chLengthMin: JET_UINT32,
    pub chLengthMax: JET_UINT32,
    pub chToIndexMax: JET_UINT32,
    pub cchIncrement: JET_UINT32,
    pub ichStart: JET_UINT32,
}
pub const JET_TblInfo: u32 = 0;
pub const JET_TblInfoDbid: u32 = 2;
pub const JET_TblInfoDumpTable: u32 = 8;
pub const JET_TblInfoMostMany: u32 = 3;
pub const JET_TblInfoName: u32 = 1;
pub const JET_TblInfoOLC: u32 = 5;
pub const JET_TblInfoResetOLC: u32 = 6;
pub const JET_TblInfoRvt: u32 = 4;
pub const JET_TblInfoSpaceAlloc: u32 = 9;
pub const JET_TblInfoSpaceAvailable: u32 = 11;
pub const JET_TblInfoSpaceOwned: u32 = 10;
pub const JET_TblInfoSpaceUsage: u32 = 7;
pub const JET_TblInfoTemplateTableName: u32 = 12;
pub type JET_UINT16 = u16;
pub type JET_UINT32 = u32;
pub type JET_UINT64 = u64;
pub type JET_UINT8 = u8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JET_UNICODEINDEX {
    pub lcid: JET_LCID,
    pub dwMapFlags: JET_UINT32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_UNICODEINDEX2 {
    pub szLocaleName: JET_PWSTR,
    pub dwMapFlags: JET_UINT32,
}
impl Default for JET_UNICODEINDEX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_USERDEFINEDDEFAULT_A {
    pub szCallback: JET_PSTR,
    pub pbUserData: *mut JET_BYTE,
    pub cbUserData: JET_UINT32,
    pub szDependantColumns: JET_PSTR,
}
impl Default for JET_USERDEFINEDDEFAULT_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_USERDEFINEDDEFAULT_W {
    pub szCallback: JET_PWSTR,
    pub pbUserData: *mut JET_BYTE,
    pub cbUserData: JET_UINT32,
    pub szDependantColumns: JET_PWSTR,
}
impl Default for JET_USERDEFINEDDEFAULT_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const JET_VERSION: u32 = 2560;
pub type JET_VOID = core::ffi::c_void;
pub type JET_WCHAR = u16;
pub const JET_bitAbortSnapshot: u32 = 1;
pub const JET_bitAllDatabasesSnapshot: u32 = 1;
pub const JET_bitBackupAtomic: u32 = 4;
pub const JET_bitBackupEndAbort: u32 = 2;
pub const JET_bitBackupEndNormal: u32 = 1;
pub const JET_bitBackupIncremental: u32 = 1;
pub const JET_bitBackupSnapshot: u32 = 16;
pub const JET_bitBackupTruncateDone: u32 = 256;
pub const JET_bitBookmarkPermitVirtualCurrency: u32 = 1;
pub const JET_bitCheckUniqueness: u32 = 64;
pub const JET_bitColumnAutoincrement: u32 = 16;
pub const JET_bitColumnCompressed: u32 = 524288;
pub const JET_bitColumnDeleteOnZero: u32 = 131072;
pub const JET_bitColumnEscrowUpdate: u32 = 2048;
pub const JET_bitColumnFinalize: u32 = 16384;
pub const JET_bitColumnFixed: u32 = 1;
pub const JET_bitColumnMaybeNull: u32 = 8192;
pub const JET_bitColumnMultiValued: u32 = 1024;
pub const JET_bitColumnNotNULL: u32 = 4;
pub const JET_bitColumnTTDescending: u32 = 128;
pub const JET_bitColumnTTKey: u32 = 64;
pub const JET_bitColumnTagged: u32 = 2;
pub const JET_bitColumnUnversioned: u32 = 4096;
pub const JET_bitColumnUpdatable: u32 = 32;
pub const JET_bitColumnUserDefinedDefault: u32 = 32768;
pub const JET_bitColumnVersion: u32 = 8;
pub const JET_bitCommitLazyFlush: u32 = 1;
pub const JET_bitCompactRepair: u32 = 64;
pub const JET_bitCompactStats: u32 = 32;
pub const JET_bitConfigStoreReadControlDefault: u32 = 0;
pub const JET_bitConfigStoreReadControlDisableAll: u32 = 2;
pub const JET_bitConfigStoreReadControlInhibitRead: u32 = 1;
pub const JET_bitContinueAfterThaw: u32 = 4;
pub const JET_bitCopySnapshot: u32 = 2;
pub const JET_bitCreateHintAppendSequential: u32 = 2;
pub const JET_bitCreateHintHotpointSequential: u32 = 4;
pub const JET_bitDbDeleteCorruptIndexes: u32 = 16;
pub const JET_bitDbDeleteUnicodeIndexes: u32 = 1024;
pub const JET_bitDbEnableBackgroundMaintenance: u32 = 2048;
pub const JET_bitDbExclusive: u32 = 2;
pub const JET_bitDbOverwriteExisting: u32 = 512;
pub const JET_bitDbPurgeCacheOnAttach: u32 = 4096;
pub const JET_bitDbReadOnly: u32 = 1;
pub const JET_bitDbRecoveryOff: u32 = 8;
pub const JET_bitDbShadowingOff: u32 = 128;
pub const JET_bitDbUpgrade: u32 = 512;
pub const JET_bitDefragmentAvailSpaceTreesOnly: u32 = 64;
pub const JET_bitDefragmentBTree: u32 = 256;
pub const JET_bitDefragmentBatchStart: u32 = 1;
pub const JET_bitDefragmentBatchStop: u32 = 2;
pub const JET_bitDefragmentNoPartialMerges: u32 = 128;
pub const JET_bitDeleteColumnIgnoreTemplateColumns: u32 = 1;
pub const JET_bitDeleteHintTableSequential: u32 = 256;
pub const JET_bitDumpCacheIncludeCachedPages: u32 = 32;
pub const JET_bitDumpCacheIncludeCorruptedPages: u32 = 64;
pub const JET_bitDumpCacheIncludeDirtyPages: u32 = 16;
pub const JET_bitDumpCacheMaximum: u32 = 8;
pub const JET_bitDumpCacheMinimum: u32 = 4;
pub const JET_bitDumpCacheNoDecommit: u32 = 128;
pub const JET_bitDumpMaximum: u32 = 2;
pub const JET_bitDumpMinimum: u32 = 1;
pub const JET_bitDurableCommitCallbackLogUnavailable: u32 = 1;
pub const JET_bitESE98FileNames: u32 = 1;
pub const JET_bitEightDotThreeSoftCompat: u32 = 2;
pub const JET_bitEnumerateCompressOutput: u32 = 524288;
pub const JET_bitEnumerateCopy: u32 = 1;
pub const JET_bitEnumerateIgnoreDefault: u32 = 32;
pub const JET_bitEnumerateIgnoreUserDefinedDefault: u32 = 1048576;
pub const JET_bitEnumerateInRecordOnly: u32 = 2097152;
pub const JET_bitEnumeratePresenceOnly: u32 = 131072;
pub const JET_bitEnumerateTaggedOnly: u32 = 262144;
pub const JET_bitEscrowNoRollback: u32 = 1;
pub const JET_bitExplicitPrepare: u32 = 8;
pub const JET_bitForceCloseAndDetach: u32 = 3;
pub const JET_bitForceDetach: u32 = 1;
pub const JET_bitForceNewLog: u32 = 16;
pub const JET_bitFullColumnEndLimit: u32 = 512;
pub const JET_bitFullColumnStartLimit: u32 = 256;
pub const JET_bitHungIOEvent: u32 = 1;
pub const JET_bitIdleCompact: u32 = 2;
pub const JET_bitIdleFlushBuffers: u32 = 1;
pub const JET_bitIdleStatus: u32 = 4;
pub const JET_bitIncrementalSnapshot: u32 = 1;
pub const JET_bitIndexColumnMustBeNonNull: u32 = 2;
pub const JET_bitIndexColumnMustBeNull: u32 = 1;
pub const JET_bitIndexCrossProduct: u32 = 16384;
pub const JET_bitIndexDisallowNull: u32 = 4;
pub const JET_bitIndexDisallowTruncation: u32 = 65536;
pub const JET_bitIndexDotNetGuid: u32 = 262144;
pub const JET_bitIndexEmpty: u32 = 256;
pub const JET_bitIndexIgnoreAnyNull: u32 = 32;
pub const JET_bitIndexIgnoreFirstNull: u32 = 64;
pub const JET_bitIndexIgnoreNull: u32 = 8;
pub const JET_bitIndexImmutableStructure: u32 = 524288;
pub const JET_bitIndexKeyMost: u32 = 32768;
pub const JET_bitIndexLazyFlush: u32 = 128;
pub const JET_bitIndexNestedTable: u32 = 131072;
pub const JET_bitIndexPrimary: u32 = 2;
pub const JET_bitIndexSortNullsHigh: u32 = 1024;
pub const JET_bitIndexTupleLimits: u32 = 8192;
pub const JET_bitIndexTuples: u32 = 4096;
pub const JET_bitIndexUnicode: u32 = 2048;
pub const JET_bitIndexUnique: u32 = 1;
pub const JET_bitIndexUnversioned: u32 = 512;
pub const JET_bitKeepDbAttachedAtEndOfRecovery: u32 = 4096;
pub const JET_bitKeyAscending: u32 = 0;
pub const JET_bitKeyDataZeroLength: u32 = 16;
pub const JET_bitKeyDescending: u32 = 1;
pub const JET_bitLSCursor: u32 = 2;
pub const JET_bitLSReset: u32 = 1;
pub const JET_bitLSTable: u32 = 4;
pub const JET_bitLogStreamMustExist: u32 = 64;
pub const JET_bitMoveFirst: u32 = 0;
pub const JET_bitMoveKeyNE: u32 = 1;
pub const JET_bitNewKey: u32 = 1;
pub const JET_bitNil: JET_GRBIT = 0 as _;
pub const JET_bitNoMove: u32 = 2;
pub const JET_bitNormalizedKey: u32 = 8;
pub const JET_bitObjectSystem: u32 = 2147483648;
pub const JET_bitObjectTableDerived: u32 = 268435456;
pub const JET_bitObjectTableFixedDDL: u32 = 1073741824;
pub const JET_bitObjectTableNoFixedVarColumnsInDerivedTables: u32 = 67108864;
pub const JET_bitObjectTableTemplate: u32 = 536870912;
pub const JET_bitPartialColumnEndLimit: u32 = 2048;
pub const JET_bitPartialColumnStartLimit: u32 = 1024;
pub const JET_bitPrereadBackward: u32 = 2;
pub const JET_bitPrereadFirstPage: u32 = 4;
pub const JET_bitPrereadForward: u32 = 1;
pub const JET_bitPrereadNormalizedKey: u32 = 8;
pub const JET_bitRangeInclusive: u32 = 1;
pub const JET_bitRangeInstantDuration: u32 = 4;
pub const JET_bitRangeRemove: u32 = 8;
pub const JET_bitRangeUpperLimit: u32 = 2;
pub const JET_bitReadLock: u32 = 1;
pub const JET_bitRecordInIndex: u32 = 1;
pub const JET_bitRecordNotInIndex: u32 = 2;
pub const JET_bitRecordSizeInCopyBuffer: u32 = 1;
pub const JET_bitRecordSizeLocal: u32 = 4;
pub const JET_bitRecordSizeRunningTotal: u32 = 2;
pub const JET_bitRecoveryWithoutUndo: u32 = 8;
pub const JET_bitReplayIgnoreLostLogs: u32 = 128;
pub const JET_bitReplayIgnoreMissingDB: u32 = 4;
pub const JET_bitReplayMissingMapEntryDB: u32 = 32;
pub const JET_bitResizeDatabaseOnlyGrow: u32 = 1;
pub const JET_bitResizeDatabaseOnlyShrink: u32 = 2;
pub const JET_bitRetrieveCopy: u32 = 1;
pub const JET_bitRetrieveFromIndex: u32 = 2;
pub const JET_bitRetrieveFromPrimaryBookmark: u32 = 4;
pub const JET_bitRetrieveHintReserve1: u32 = 8;
pub const JET_bitRetrieveHintReserve2: u32 = 64;
pub const JET_bitRetrieveHintReserve3: u32 = 128;
pub const JET_bitRetrieveHintTableScanBackward: u32 = 32;
pub const JET_bitRetrieveHintTableScanForward: u32 = 16;
pub const JET_bitRetrieveIgnoreDefault: u32 = 32;
pub const JET_bitRetrieveNull: u32 = 16;
pub const JET_bitRetrieveTag: u32 = 8;
pub const JET_bitRetrieveTuple: u32 = 2048;
pub const JET_bitRollbackAll: u32 = 1;
pub const JET_bitSeekEQ: u32 = 1;
pub const JET_bitSeekGE: u32 = 8;
pub const JET_bitSeekGT: u32 = 16;
pub const JET_bitSeekLE: u32 = 4;
pub const JET_bitSeekLT: u32 = 2;
pub const JET_bitSetAppendLV: u32 = 1;
pub const JET_bitSetCompressed: u32 = 131072;
pub const JET_bitSetIndexRange: u32 = 32;
pub const JET_bitSetIntrinsicLV: u32 = 1024;
pub const JET_bitSetOverwriteLV: u32 = 4;
pub const JET_bitSetRevertToDefaultValue: u32 = 512;
pub const JET_bitSetSeparateLV: u32 = 64;
pub const JET_bitSetSizeLV: u32 = 8;
pub const JET_bitSetUncompressed: u32 = 65536;
pub const JET_bitSetUniqueMultiValues: u32 = 128;
pub const JET_bitSetUniqueNormalizedMultiValues: u32 = 256;
pub const JET_bitSetZeroLength: u32 = 32;
pub const JET_bitShrinkDatabaseOff: u32 = 0;
pub const JET_bitShrinkDatabaseOn: u32 = 1;
pub const JET_bitShrinkDatabaseRealtime: u32 = 2;
pub const JET_bitShrinkDatabaseTrim: u32 = 1;
pub const JET_bitSpaceHintsUtilizeParentSpace: u32 = 1;
pub const JET_bitStopServiceAll: u32 = 0;
pub const JET_bitStopServiceBackgroundUserTasks: u32 = 2;
pub const JET_bitStopServiceQuiesceCaches: u32 = 4;
pub const JET_bitStopServiceResume: u32 = 2147483648;
pub const JET_bitStrLimit: u32 = 2;
pub const JET_bitSubStrLimit: u32 = 4;
pub const JET_bitTTDotNetGuid: u32 = 256;
pub const JET_bitTTErrorOnDuplicateInsertion: u32 = 32;
pub const JET_bitTTForceMaterialization: u32 = 32;
pub const JET_bitTTForwardOnly: u32 = 64;
pub const JET_bitTTIndexed: u32 = 1;
pub const JET_bitTTIntrinsicLVsOnly: u32 = 128;
pub const JET_bitTTScrollable: u32 = 8;
pub const JET_bitTTSortNullsHigh: u32 = 16;
pub const JET_bitTTUnique: u32 = 2;
pub const JET_bitTTUpdatable: u32 = 4;
pub const JET_bitTableClass1: u32 = 65536;
pub const JET_bitTableClass10: u32 = 655360;
pub const JET_bitTableClass11: u32 = 720896;
pub const JET_bitTableClass12: u32 = 786432;
pub const JET_bitTableClass13: u32 = 851968;
pub const JET_bitTableClass14: u32 = 917504;
pub const JET_bitTableClass15: u32 = 983040;
pub const JET_bitTableClass2: u32 = 131072;
pub const JET_bitTableClass3: u32 = 196608;
pub const JET_bitTableClass4: u32 = 262144;
pub const JET_bitTableClass5: u32 = 327680;
pub const JET_bitTableClass6: u32 = 393216;
pub const JET_bitTableClass7: u32 = 458752;
pub const JET_bitTableClass8: u32 = 524288;
pub const JET_bitTableClass9: u32 = 589824;
pub const JET_bitTableClassMask: u32 = 2031616;
pub const JET_bitTableClassNone: u32 = 0;
pub const JET_bitTableCreateFixedDDL: u32 = 1;
pub const JET_bitTableCreateImmutableStructure: u32 = 8;
pub const JET_bitTableCreateNoFixedVarColumnsInDerivedTables: u32 = 4;
pub const JET_bitTableCreateTemplateTable: u32 = 2;
pub const JET_bitTableDenyRead: u32 = 2;
pub const JET_bitTableDenyWrite: u32 = 1;
pub const JET_bitTableInfoBookmark: u32 = 2;
pub const JET_bitTableInfoRollback: u32 = 4;
pub const JET_bitTableInfoUpdatable: u32 = 1;
pub const JET_bitTableNoCache: u32 = 32;
pub const JET_bitTableOpportuneRead: u32 = 128;
pub const JET_bitTablePermitDDL: u32 = 16;
pub const JET_bitTablePreread: u32 = 64;
pub const JET_bitTableReadOnly: u32 = 4;
pub const JET_bitTableSequential: u32 = 32768;
pub const JET_bitTableUpdatable: u32 = 8;
pub const JET_bitTermAbrupt: u32 = 2;
pub const JET_bitTermComplete: u32 = 1;
pub const JET_bitTermDirty: u32 = 8;
pub const JET_bitTermStopBackup: u32 = 4;
pub const JET_bitTransactionReadOnly: u32 = 1;
pub const JET_bitTruncateLogsAfterRecovery: u32 = 16;
pub const JET_bitUpdateCheckESE97Compatibility: u32 = 1;
pub const JET_bitWaitAllLevel0Commit: u32 = 8;
pub const JET_bitWaitLastLevel0Commit: u32 = 2;
pub const JET_bitWriteLock: u32 = 2;
pub const JET_bitZeroLength: u32 = 1;
pub const JET_cbBookmarkMost: u32 = 256;
pub const JET_cbBookmarkMostMost: u32 = 2000;
pub const JET_cbColumnLVPageOverhead: u32 = 82;
pub const JET_cbColumnMost: u32 = 255;
pub const JET_cbFullNameMost: u32 = 255;
pub const JET_cbKeyMost: u32 = 255;
pub const JET_cbKeyMost16KBytePage: u32 = 2000;
pub const JET_cbKeyMost2KBytePage: u32 = 500;
pub const JET_cbKeyMost32KBytePage: u32 = 2000;
pub const JET_cbKeyMost4KBytePage: u32 = 1000;
pub const JET_cbKeyMost8KBytePage: u32 = 2000;
pub const JET_cbKeyMostMin: u32 = 255;
pub const JET_cbKeyMostMost: u32 = 2000;
pub const JET_cbLVColumnMost: u32 = 2147483647;
pub const JET_cbLVDefaultValueMost: u32 = 255;
pub const JET_cbLimitKeyMost: u32 = 256;
pub const JET_cbNameMost: u32 = 64;
pub const JET_cbPrimaryKeyMost: u32 = 255;
pub const JET_cbSecondaryKeyMost: u32 = 255;
pub const JET_cbtypAfterDelete: u32 = 64;
pub const JET_cbtypAfterInsert: u32 = 4;
pub const JET_cbtypAfterReplace: u32 = 16;
pub const JET_cbtypBeforeDelete: u32 = 32;
pub const JET_cbtypBeforeInsert: u32 = 2;
pub const JET_cbtypBeforeReplace: u32 = 8;
pub const JET_cbtypFinalize: u32 = 1;
pub const JET_cbtypFreeCursorLS: u32 = 512;
pub const JET_cbtypFreeTableLS: u32 = 1024;
pub const JET_cbtypNull: u32 = 0;
pub const JET_cbtypOnlineDefragCompleted: u32 = 256;
pub const JET_cbtypUserDefinedDefaultValue: u32 = 128;
pub const JET_ccolFixedMost: u32 = 127;
pub const JET_ccolKeyMost: u32 = 16;
pub const JET_ccolMost: u32 = 65248;
pub const JET_ccolTaggedMost: u32 = 64993;
pub const JET_ccolVarMost: u32 = 128;
pub const JET_coltypBinary: u32 = 9;
pub const JET_coltypBit: u32 = 1;
pub const JET_coltypCurrency: u32 = 5;
pub const JET_coltypDateTime: u32 = 8;
pub const JET_coltypGUID: u32 = 16;
pub const JET_coltypIEEEDouble: u32 = 7;
pub const JET_coltypIEEESingle: u32 = 6;
pub const JET_coltypLong: u32 = 4;
pub const JET_coltypLongBinary: u32 = 11;
pub const JET_coltypLongLong: u32 = 15;
pub const JET_coltypLongText: u32 = 12;
pub const JET_coltypMax: u32 = 19;
pub const JET_coltypNil: u32 = 0;
pub const JET_coltypSLV: u32 = 13;
pub const JET_coltypShort: u32 = 3;
pub const JET_coltypText: u32 = 10;
pub const JET_coltypUnsignedByte: u32 = 2;
pub const JET_coltypUnsignedLong: u32 = 14;
pub const JET_coltypUnsignedLongLong: u32 = 18;
pub const JET_coltypUnsignedShort: u32 = 17;
pub const JET_configDefault: u32 = 1;
pub const JET_configDynamicMediumMemory: u32 = 32;
pub const JET_configHighConcurrencyScaling: u32 = 1024;
pub const JET_configLowDiskFootprint: u32 = 4;
pub const JET_configLowMemory: u32 = 16;
pub const JET_configLowPower: u32 = 64;
pub const JET_configMediumDiskFootprint: u32 = 8;
pub const JET_configRemoveQuotas: u32 = 2;
pub const JET_configRunSilent: u32 = 256;
pub const JET_configSSDProfileIO: u32 = 128;
pub const JET_configUnthrottledMemory: u32 = 512;
pub const JET_dbidNil: JET_DBID = 4294967295u32 as _;
pub const JET_dbstateBeingConverted: u32 = 4;
pub const JET_dbstateCleanShutdown: u32 = 3;
pub const JET_dbstateDirtyShutdown: u32 = 2;
pub const JET_dbstateForceDetach: u32 = 5;
pub const JET_dbstateJustCreated: u32 = 1;
pub const JET_efvAllowHigherPersistedFormat: u32 = 1090519040;
pub const JET_efvUseEngineDefault: u32 = 1073741825;
pub const JET_efvUsePersistedFormat: u32 = 1073741826;
pub const JET_efvWindows10v2004: u32 = 9180;
pub const JET_efvWindows11v21H2: u32 = 9400;
pub const JET_efvWindows11v22H2: u32 = 9480;
pub const JET_efvWindows19H1Rtm: u32 = 8920;
pub const JET_efvWindowsServer2022: u32 = 9360;
pub const JET_efvWindowsServer2025: u32 = 9620;
pub const JET_errAccessDenied: i32 = -1907;
pub const JET_errAfterInitialization: i32 = -1850;
pub const JET_errAlreadyInitialized: i32 = -1030;
pub const JET_errAlreadyPrepared: i32 = -1607;
pub const JET_errAttachedDatabaseMismatch: i32 = -1216;
pub const JET_errAutoIncrementNotSet: i32 = -1625;
pub const JET_errBBTBuffCorrupted: i32 = -365;
pub const JET_errBBTNodeCorrupted: i32 = -364;
pub const JET_errBackupAbortByServer: i32 = -801;
pub const JET_errBackupDirectoryNotEmpty: i32 = -504;
pub const JET_errBackupInProgress: i32 = -505;
pub const JET_errBackupNotAllowedYet: i32 = -523;
pub const JET_errBadBackupDatabaseSize: i32 = -561;
pub const JET_errBadBookmark: i32 = -328;
pub const JET_errBadCheckpointSignature: i32 = -532;
pub const JET_errBadColumnId: i32 = -1517;
pub const JET_errBadDbSignature: i32 = -531;
pub const JET_errBadEmptyPage: i32 = -351;
pub const JET_errBadItagSequence: i32 = -1518;
pub const JET_errBadLineCount: i32 = -354;
pub const JET_errBadLogSignature: i32 = -530;
pub const JET_errBadLogVersion: i32 = -514;
pub const JET_errBadPageLink: i32 = -327;
pub const JET_errBadParentPageLink: i32 = -338;
pub const JET_errBadPatchPage: i32 = -535;
pub const JET_errBadRestoreTargetInstance: i32 = -577;
pub const JET_errBadRootPageLink: i32 = -366;
pub const JET_errBlockedByCorruptionMark: i32 = -1233;
pub const JET_errBufferTooSmall: i32 = -1038;
pub const JET_errCallbackFailed: i32 = -2101;
pub const JET_errCallbackNotResolved: i32 = -2102;
pub const JET_errCannotAddFixedVarColumnToDerivedTable: i32 = -1330;
pub const JET_errCannotBeTagged: i32 = -1521;
pub const JET_errCannotDeleteSystemTable: i32 = -1318;
pub const JET_errCannotDeleteTempTable: i32 = -1317;
pub const JET_errCannotDeleteTemplateTable: i32 = -1319;
pub const JET_errCannotDisableVersioning: i32 = -1208;
pub const JET_errCannotIndex: i32 = -1071;
pub const JET_errCannotIndexOnEncryptedColumn: i32 = -1440;
pub const JET_errCannotLogDuringRecoveryRedo: i32 = -512;
pub const JET_errCannotMaterializeForwardOnlySort: i32 = -1113;
pub const JET_errCannotNestDDL: i32 = -1325;
pub const JET_errCannotSeparateIntrinsicLV: i32 = -416;
pub const JET_errCatalogCorrupted: i32 = -1220;
pub const JET_errCheckpointCorrupt: i32 = -533;
pub const JET_errCheckpointDepthTooDeep: i32 = -614;
pub const JET_errCheckpointFileNotFound: i32 = -542;
pub const JET_errClientRequestToStopJetService: i32 = -1329;
pub const JET_errClientSpaceBegin: i32 = -10000;
pub const JET_errClientSpaceEnd: i32 = -11999;
pub const JET_errColumnCannotBeCompressed: i32 = -1538;
pub const JET_errColumnCannotBeEncrypted: i32 = -1439;
pub const JET_errColumnDoesNotFit: i32 = -1503;
pub const JET_errColumnDuplicate: i32 = -1508;
pub const JET_errColumnIllegalNull: i32 = -1504;
pub const JET_errColumnInRelationship: i32 = -1519;
pub const JET_errColumnInUse: i32 = -1046;
pub const JET_errColumnIndexed: i32 = -1505;
pub const JET_errColumnLong: i32 = -1501;
pub const JET_errColumnNoChunk: i32 = -1502;
pub const JET_errColumnNoEncryptionKey: i32 = -1540;
pub const JET_errColumnNotFound: i32 = -1507;
pub const JET_errColumnNotUpdatable: i32 = -1048;
pub const JET_errColumnRedundant: i32 = -1510;
pub const JET_errColumnTooBig: i32 = -1506;
pub const JET_errCommittedLogFileCorrupt: i32 = -586;
pub const JET_errCommittedLogFilesMissing: i32 = -582;
pub const JET_errConsistentTimeMismatch: i32 = -551;
pub const JET_errContainerNotEmpty: i32 = -1043;
pub const JET_errCopySignatureMismatchCannotRestart: i32 = -8003;
pub const JET_errDDLNotInheritable: i32 = -1326;
pub const JET_errDataHasChanged: i32 = -1611;
pub const JET_errDatabase200Format: i32 = -1210;
pub const JET_errDatabase400Format: i32 = -1211;
pub const JET_errDatabase500Format: i32 = -1212;
pub const JET_errDatabaseAlreadyRunningMaintenance: i32 = -2004;
pub const JET_errDatabaseAlreadyUpgraded: i32 = -562;
pub const JET_errDatabaseAttachedForRecovery: i32 = -1231;
pub const JET_errDatabaseBufferDependenciesCorrupted: i32 = -255;
pub const JET_errDatabaseCorrupted: i32 = -1206;
pub const JET_errDatabaseCorruptedNoRepair: i32 = -1224;
pub const JET_errDatabaseDirtyShutdown: i32 = -550;
pub const JET_errDatabaseDuplicate: i32 = -1201;
pub const JET_errDatabaseFileReadOnly: i32 = -1008;
pub const JET_errDatabaseIdInUse: i32 = -1218;
pub const JET_errDatabaseInUse: i32 = -1202;
pub const JET_errDatabaseIncompleteUpgrade: i32 = -563;
pub const JET_errDatabaseInconsistent: i32 = -550;
pub const JET_errDatabaseInvalidName: i32 = -1204;
pub const JET_errDatabaseInvalidPages: i32 = -1205;
pub const JET_errDatabaseInvalidPath: i32 = -1217;
pub const JET_errDatabaseLeakInSpace: i32 = -348;
pub const JET_errDatabaseLocked: i32 = -1207;
pub const JET_errDatabaseLogSetMismatch: i32 = -539;
pub const JET_errDatabaseNotFound: i32 = -1203;
pub const JET_errDatabaseNotReady: i32 = -1230;
pub const JET_errDatabasePatchFileMismatch: i32 = -552;
pub const JET_errDatabaseSharingViolation: i32 = -1215;
pub const JET_errDatabaseSignInUse: i32 = -1222;
pub const JET_errDatabaseStreamingFileMismatch: i32 = -540;
pub const JET_errDatabaseUnavailable: i32 = -1091;
pub const JET_errDatabasesNotFromSameSnapshot: i32 = -580;
pub const JET_errDbTimeBeyondMaxRequired: i32 = -625;
pub const JET_errDbTimeCorrupted: i32 = -344;
pub const JET_errDbTimeTooNew: i32 = -567;
pub const JET_errDbTimeTooOld: i32 = -566;
pub const JET_errDecompressionFailed: i32 = -1620;
pub const JET_errDecryptionFailed: i32 = -1622;
pub const JET_errDefaultValueTooBig: i32 = -1524;
pub const JET_errDeleteBackupFileFail: i32 = -524;
pub const JET_errDensityInvalid: i32 = -1307;
pub const JET_errDerivedColumnCorruption: i32 = -1529;
pub const JET_errDirtyShutdown: i32 = -1116;
pub const JET_errDisabledFunctionality: i32 = -112;
pub const JET_errDiskFull: i32 = -1808;
pub const JET_errDiskIO: i32 = -1022;
pub const JET_errDiskReadVerificationFailure: i32 = -1021;
pub const JET_errEncryptionBadItag: i32 = -1623;
pub const JET_errEndingRestoreLogTooLow: i32 = -553;
pub const JET_errEngineFormatVersionNoLongerSupportedTooLow: i32 = -619;
pub const JET_errEngineFormatVersionNotYetImplementedTooHigh: i32 = -620;
pub const JET_errEngineFormatVersionParamTooLowForRequestedFeature: i32 = -621;
pub const JET_errEngineFormatVersionSpecifiedTooLowForDatabaseVersion: i32 = -623;
pub const JET_errEngineFormatVersionSpecifiedTooLowForLogVersion: i32 = -622;
pub const JET_errEntryPointNotFound: i32 = -1911;
pub const JET_errExclusiveTableLockRequired: i32 = -1322;
pub const JET_errExistingLogFileHasBadSignature: i32 = -610;
pub const JET_errExistingLogFileIsNotContiguous: i32 = -611;
pub const JET_errFeatureNotAvailable: i32 = -1001;
pub const JET_errFileAccessDenied: i32 = -1032;
pub const JET_errFileAlreadyExists: i32 = -1814;
pub const JET_errFileClose: i32 = -102;
pub const JET_errFileCompressed: i32 = -4005;
pub const JET_errFileIOAbort: i32 = -4002;
pub const JET_errFileIOBeyondEOF: i32 = -4001;
pub const JET_errFileIOFail: i32 = -4004;
pub const JET_errFileIORetry: i32 = -4003;
pub const JET_errFileIOSparse: i32 = -4000;
pub const JET_errFileInvalidType: i32 = -1812;
pub const JET_errFileNotFound: i32 = -1811;
pub const JET_errFileSystemCorruption: i32 = -1121;
pub const JET_errFilteredMoveNotSupported: i32 = -1124;
pub const JET_errFixedDDL: i32 = -1323;
pub const JET_errFixedInheritedDDL: i32 = -1324;
pub const JET_errFlushMapDatabaseMismatch: i32 = -1919;
pub const JET_errFlushMapUnrecoverable: i32 = -1920;
pub const JET_errFlushMapVersionUnsupported: i32 = -1918;
pub const JET_errForceDetachNotAllowed: i32 = -1219;
pub const JET_errGivenLogFileHasBadSignature: i32 = -555;
pub const JET_errGivenLogFileIsNotContiguous: i32 = -556;
pub const JET_errIllegalOperation: i32 = -1312;
pub const JET_errInTransaction: i32 = -1108;
pub const JET_errIndexBuildCorrupted: i32 = -1412;
pub const JET_errIndexCantBuild: i32 = -1401;
pub const JET_errIndexDuplicate: i32 = -1403;
pub const JET_errIndexHasPrimary: i32 = -1402;
pub const JET_errIndexInUse: i32 = -1051;
pub const JET_errIndexInvalidDef: i32 = -1406;
pub const JET_errIndexMustStay: i32 = -1405;
pub const JET_errIndexNotFound: i32 = -1404;
pub const JET_errIndexTuplesCannotRetrieveFromIndex: i32 = -1436;
pub const JET_errIndexTuplesInvalidLimits: i32 = -1435;
pub const JET_errIndexTuplesKeyTooSmall: i32 = -1437;
pub const JET_errIndexTuplesNonUniqueOnly: i32 = -1432;
pub const JET_errIndexTuplesOneColumnOnly: i32 = -1431;
pub const JET_errIndexTuplesSecondaryIndexOnly: i32 = -1430;
pub const JET_errIndexTuplesTextBinaryColumnsOnly: i32 = -1433;
pub const JET_errIndexTuplesTextColumnsOnly: i32 = -1433;
pub const JET_errIndexTuplesTooManyColumns: i32 = -1431;
pub const JET_errIndexTuplesVarSegMacNotAllowed: i32 = -1434;
pub const JET_errInitInProgress: i32 = -1031;
pub const JET_errInsertKeyOutOfOrder: i32 = -627;
pub const JET_errInstanceNameInUse: i32 = -1086;
pub const JET_errInstanceUnavailable: i32 = -1090;
pub const JET_errInstanceUnavailableDueToFatalLogDiskFull: i32 = -1092;
pub const JET_errInternalError: i32 = -107;
pub const JET_errInvalidBackup: i32 = -526;
pub const JET_errInvalidBackupSequence: i32 = -521;
pub const JET_errInvalidBookmark: i32 = -1045;
pub const JET_errInvalidBufferSize: i32 = -1047;
pub const JET_errInvalidCodePage: i32 = -1063;
pub const JET_errInvalidColumnType: i32 = -1511;
pub const JET_errInvalidCountry: i32 = -1061;
pub const JET_errInvalidCreateDbVersion: i32 = -1225;
pub const JET_errInvalidCreateIndex: i32 = -1409;
pub const JET_errInvalidDatabase: i32 = -1028;
pub const JET_errInvalidDatabaseId: i32 = -1010;
pub const JET_errInvalidDatabaseVersion: i32 = -1209;
pub const JET_errInvalidDbparamId: i32 = -1095;
pub const JET_errInvalidFilename: i32 = -1044;
pub const JET_errInvalidGrbit: i32 = -900;
pub const JET_errInvalidIndexId: i32 = -1416;
pub const JET_errInvalidInstance: i32 = -1115;
pub const JET_errInvalidLCMapStringFlags: i32 = -1064;
pub const JET_errInvalidLVChunkSize: i32 = -1438;
pub const JET_errInvalidLanguageId: i32 = -1062;
pub const JET_errInvalidLogDirectory: i32 = -1025;
pub const JET_errInvalidLogSequence: i32 = -515;
pub const JET_errInvalidLoggedOperation: i32 = -500;
pub const JET_errInvalidName: i32 = -1002;
pub const JET_errInvalidObject: i32 = -1316;
pub const JET_errInvalidOnSort: i32 = -1702;
pub const JET_errInvalidOperation: i32 = -1906;
pub const JET_errInvalidParameter: i32 = -1003;
pub const JET_errInvalidPath: i32 = -1023;
pub const JET_errInvalidPlaceholderColumn: i32 = -1530;
pub const JET_errInvalidPreread: i32 = -424;
pub const JET_errInvalidSesid: i32 = -1104;
pub const JET_errInvalidSesparamId: i32 = -1093;
pub const JET_errInvalidSettings: i32 = -1328;
pub const JET_errInvalidSystemPath: i32 = -1024;
pub const JET_errInvalidTableId: i32 = -1310;
pub const JET_errKeyBoundary: i32 = -324;
pub const JET_errKeyDuplicate: i32 = -1605;
pub const JET_errKeyIsMade: i32 = -1516;
pub const JET_errKeyNotMade: i32 = -1608;
pub const JET_errKeyTooBig: i32 = -408;
pub const JET_errKeyTruncated: i32 = -346;
pub const JET_errLSAlreadySet: i32 = -3001;
pub const JET_errLSCallbackNotSpecified: i32 = -3000;
pub const JET_errLSNotSet: i32 = -3002;
pub const JET_errLVCorrupted: i32 = -1526;
pub const JET_errLanguageNotSupported: i32 = -1619;
pub const JET_errLinkNotSupported: i32 = -1052;
pub const JET_errLogBufferTooSmall: i32 = -517;
pub const JET_errLogCorruptDuringHardRecovery: i32 = -574;
pub const JET_errLogCorruptDuringHardRestore: i32 = -573;
pub const JET_errLogCorrupted: i32 = -1852;
pub const JET_errLogDisabledDueToRecoveryFailure: i32 = -511;
pub const JET_errLogDiskFull: i32 = -529;
pub const JET_errLogFileCorrupt: i32 = -501;
pub const JET_errLogFileNotCopied: i32 = -616;
pub const JET_errLogFilePathInUse: i32 = -1084;
pub const JET_errLogFileSizeMismatch: i32 = -541;
pub const JET_errLogFileSizeMismatchDatabasesConsistent: i32 = -545;
pub const JET_errLogGenerationMismatch: i32 = -513;
pub const JET_errLogOperationInconsistentWithDatabase: i32 = -626;
pub const JET_errLogReadVerifyFailure: i32 = -612;
pub const JET_errLogSectorSizeMismatch: i32 = -546;
pub const JET_errLogSectorSizeMismatchDatabasesConsistent: i32 = -547;
pub const JET_errLogSequenceChecksumMismatch: i32 = -590;
pub const JET_errLogSequenceEnd: i32 = -519;
pub const JET_errLogSequenceEndDatabasesConsistent: i32 = -548;
pub const JET_errLogTornWriteDuringHardRecovery: i32 = -571;
pub const JET_errLogTornWriteDuringHardRestore: i32 = -570;
pub const JET_errLogWriteFail: i32 = -510;
pub const JET_errLoggingDisabled: i32 = -516;
pub const JET_errMakeBackupDirectoryFail: i32 = -525;
pub const JET_errMissingCurrentLogFiles: i32 = -565;
pub const JET_errMissingFileToBackup: i32 = -569;
pub const JET_errMissingFullBackup: i32 = -560;
pub const JET_errMissingLogFile: i32 = -528;
pub const JET_errMissingPatchPage: i32 = -534;
pub const JET_errMissingPreviousLogFile: i32 = -509;
pub const JET_errMissingRestoreLogFiles: i32 = -557;
pub const JET_errMultiValuedColumnMustBeTagged: i32 = -1509;
pub const JET_errMultiValuedDuplicate: i32 = -1525;
pub const JET_errMultiValuedDuplicateAfterTruncation: i32 = -1528;
pub const JET_errMultiValuedIndexViolation: i32 = -1411;
pub const JET_errMustBeSeparateLongValue: i32 = -423;
pub const JET_errMustDisableLoggingForDbUpgrade: i32 = -575;
pub const JET_errMustRollback: i32 = -1057;
pub const JET_errNTSystemCallFailed: i32 = -334;
pub const JET_errNoBackup: i32 = -520;
pub const JET_errNoBackupDirectory: i32 = -503;
pub const JET_errNoCurrentIndex: i32 = -1515;
pub const JET_errNoCurrentRecord: i32 = -1603;
pub const JET_errNodeCorrupted: i32 = -358;
pub const JET_errNotInTransaction: i32 = -1054;
pub const JET_errNotInitialized: i32 = -1029;
pub const JET_errNullInvalid: i32 = -1504;
pub const JET_errNullKeyDisallowed: i32 = -1053;
pub const JET_errOSSnapshotInvalidSequence: i32 = -2401;
pub const JET_errOSSnapshotInvalidSnapId: i32 = -2404;
pub const JET_errOSSnapshotNotAllowed: i32 = -2403;
pub const JET_errOSSnapshotTimeOut: i32 = -2402;
pub const JET_errObjectDuplicate: i32 = -1314;
pub const JET_errObjectNotFound: i32 = -1305;
pub const JET_errOneDatabasePerSession: i32 = -1916;
pub const JET_errOutOfAutoincrementValues: i32 = -1076;
pub const JET_errOutOfBuffers: i32 = -1014;
pub const JET_errOutOfCursors: i32 = -1013;
pub const JET_errOutOfDatabaseSpace: i32 = -1012;
pub const JET_errOutOfDbtimeValues: i32 = -1077;
pub const JET_errOutOfFileHandles: i32 = -1020;
pub const JET_errOutOfLongValueIDs: i32 = -1075;
pub const JET_errOutOfMemory: i32 = -1011;
pub const JET_errOutOfObjectIDs: i32 = -1074;
pub const JET_errOutOfSequentialIndexValues: i32 = -1078;
pub const JET_errOutOfSessions: i32 = -1101;
pub const JET_errOutOfThreads: i32 = -103;
pub const JET_errPageBoundary: i32 = -323;
pub const JET_errPageInitializedMismatch: i32 = -596;
pub const JET_errPageInitializedMismatchUninitLocal: i32 = -597;
pub const JET_errPageInitializedMismatchUninitRemote: i32 = -596;
pub const JET_errPageNotInitialized: i32 = -1019;
pub const JET_errPageSizeMismatch: i32 = -1213;
pub const JET_errPageTagCorrupted: i32 = -357;
pub const JET_errPartiallyAttachedDB: i32 = -1221;
pub const JET_errPatchFileMissing: i32 = -538;
pub const JET_errPermissionDenied: i32 = -1809;
pub const JET_errPreviousVersion: i32 = -322;
pub const JET_errPrimaryIndexCorrupted: i32 = -1413;
pub const JET_errReadLostFlushVerifyFailure: i32 = -1119;
pub const JET_errReadPgnoVerifyFailure: i32 = -1118;
pub const JET_errReadVerifyFailure: i32 = -1018;
pub const JET_errRecordDeleted: i32 = -1017;
pub const JET_errRecordFormatConversionFailed: i32 = -1915;
pub const JET_errRecordNoCopy: i32 = -1602;
pub const JET_errRecordNotDeleted: i32 = -1072;
pub const JET_errRecordNotFound: i32 = -1601;
pub const JET_errRecordPrimaryChanged: i32 = -1604;
pub const JET_errRecordTooBig: i32 = -1026;
pub const JET_errRecordTooBigForBackwardCompatibility: i32 = -1112;
pub const JET_errRecoveredWithErrors: i32 = -527;
pub const JET_errRecoveredWithoutUndo: i32 = -579;
pub const JET_errRecoveredWithoutUndoDatabasesConsistent: i32 = -584;
pub const JET_errRecoveryVerifyFailure: i32 = -1123;
pub const JET_errRedoAbruptEnded: i32 = -536;
pub const JET_errRequiredLogFilesMissing: i32 = -543;
pub const JET_errRestoreInProgress: i32 = -506;
pub const JET_errRestoreOfNonBackupDatabase: i32 = -615;
pub const JET_errRfsFailure: i32 = -100;
pub const JET_errRfsNotArmed: i32 = -101;
pub const JET_errRollbackError: i32 = -1917;
pub const JET_errRollbackRequired: i32 = -1109;
pub const JET_errRunningInMultiInstanceMode: i32 = -1081;
pub const JET_errRunningInOneInstanceMode: i32 = -1080;
pub const JET_errSPAvailExtCacheOutOfMemory: i32 = -342;
pub const JET_errSPAvailExtCacheOutOfSync: i32 = -340;
pub const JET_errSPAvailExtCorrupted: i32 = -341;
pub const JET_errSPOwnExtCorrupted: i32 = -343;
pub const JET_errSecondaryIndexCorrupted: i32 = -1414;
pub const JET_errSectorSizeNotSupported: i32 = -583;
pub const JET_errSeparatedLongValue: i32 = -421;
pub const JET_errSesidTableIdMismatch: i32 = -1114;
pub const JET_errSessionContextAlreadySet: i32 = -1912;
pub const JET_errSessionContextNotSetByThisThread: i32 = -1913;
pub const JET_errSessionInUse: i32 = -1914;
pub const JET_errSessionSharingViolation: i32 = -1910;
pub const JET_errSessionWriteConflict: i32 = -1111;
pub const JET_errSetAutoIncrementTooHigh: i32 = -1624;
pub const JET_errSoftRecoveryOnBackupDatabase: i32 = -544;
pub const JET_errSoftRecoveryOnSnapshot: i32 = -581;
pub const JET_errSpaceHintsInvalid: i32 = -2103;
pub const JET_errStartingRestoreLogTooHigh: i32 = -554;
pub const JET_errStreamingDataNotLogged: i32 = -549;
pub const JET_errSuccess: u32 = 0;
pub const JET_errSystemParameterConflict: i32 = -1087;
pub const JET_errSystemParamsAlreadySet: i32 = -1082;
pub const JET_errSystemPathInUse: i32 = -1083;
pub const JET_errTableDuplicate: i32 = -1303;
pub const JET_errTableInUse: i32 = -1304;
pub const JET_errTableLocked: i32 = -1302;
pub const JET_errTableNotEmpty: i32 = -1308;
pub const JET_errTaggedNotNULL: i32 = -1514;
pub const JET_errTaskDropped: i32 = -106;
pub const JET_errTempFileOpenError: i32 = -1803;
pub const JET_errTempPathInUse: i32 = -1085;
pub const JET_errTermInProgress: i32 = -1000;
pub const JET_errTooManyActiveUsers: i32 = -1059;
pub const JET_errTooManyAttachedDatabases: i32 = -1805;
pub const JET_errTooManyColumns: i32 = -1040;
pub const JET_errTooManyIO: i32 = -105;
pub const JET_errTooManyIndexes: i32 = -1015;
pub const JET_errTooManyInstances: i32 = -1214;
pub const JET_errTooManyKeys: i32 = -1016;
pub const JET_errTooManyMempoolEntries: i32 = -1073;
pub const JET_errTooManyOpenDatabases: i32 = -1027;
pub const JET_errTooManyOpenIndexes: i32 = -1410;
pub const JET_errTooManyOpenTables: i32 = -1311;
pub const JET_errTooManyOpenTablesAndCleanupTimedOut: i32 = -1313;
pub const JET_errTooManyRecords: i32 = -1094;
pub const JET_errTooManySorts: i32 = -1701;
pub const JET_errTooManySplits: i32 = -1909;
pub const JET_errTransReadOnly: i32 = -1110;
pub const JET_errTransTooDeep: i32 = -1103;
pub const JET_errTransactionTooLong: i32 = -618;
pub const JET_errTransactionsNotReadyDuringRecovery: i32 = -1232;
pub const JET_errUnicodeLanguageValidationFailure: i32 = -604;
pub const JET_errUnicodeNormalizationNotSupported: i32 = -603;
pub const JET_errUnicodeTranslationBufferTooSmall: i32 = -601;
pub const JET_errUnicodeTranslationFail: i32 = -602;
pub const JET_errUnloadableOSFunctionality: i32 = -113;
pub const JET_errUpdateMustVersion: i32 = -1621;
pub const JET_errUpdateNotPrepared: i32 = -1609;
pub const JET_errVersionStoreEntryTooBig: i32 = -1065;
pub const JET_errVersionStoreOutOfMemory: i32 = -1069;
pub const JET_errVersionStoreOutOfMemoryAndCleanupTimedOut: i32 = -1066;
pub const JET_errWriteConflict: i32 = -1102;
pub const JET_errWriteConflictPrimaryIndex: i32 = -1105;
pub const JET_errcatApi: JET_ERRCAT = 13;
pub const JET_errcatCorruption: JET_ERRCAT = 10;
pub const JET_errcatData: JET_ERRCAT = 9;
pub const JET_errcatDisk: JET_ERRCAT = 8;
pub const JET_errcatError: JET_ERRCAT = 1;
pub const JET_errcatFatal: JET_ERRCAT = 3;
pub const JET_errcatFragmentation: JET_ERRCAT = 12;
pub const JET_errcatIO: JET_ERRCAT = 4;
pub const JET_errcatInconsistent: JET_ERRCAT = 11;
pub const JET_errcatMax: JET_ERRCAT = 17;
pub const JET_errcatMemory: JET_ERRCAT = 6;
pub const JET_errcatObsolete: JET_ERRCAT = 16;
pub const JET_errcatOperation: JET_ERRCAT = 2;
pub const JET_errcatQuota: JET_ERRCAT = 7;
pub const JET_errcatResource: JET_ERRCAT = 5;
pub const JET_errcatState: JET_ERRCAT = 15;
pub const JET_errcatUnknown: JET_ERRCAT = 0;
pub const JET_errcatUsage: JET_ERRCAT = 14;
pub const JET_filetypeCheckpoint: u32 = 4;
pub const JET_filetypeDatabase: u32 = 1;
pub const JET_filetypeFlushMap: u32 = 7;
pub const JET_filetypeLog: u32 = 3;
pub const JET_filetypeTempDatabase: u32 = 5;
pub const JET_filetypeUnknown: u32 = 0;
pub const JET_instanceNil: i32 = -1;
pub const JET_objtypNil: u32 = 0;
pub const JET_objtypTable: u32 = 1;
pub const JET_paramAccessDeniedRetryPeriod: u32 = 53;
pub const JET_paramAlternateDatabaseRecoveryPath: u32 = 113;
pub const JET_paramBaseName: u32 = 3;
pub const JET_paramBatchIOBufferMax: u32 = 22;
pub const JET_paramCachePriority: u32 = 177;
pub const JET_paramCacheSize: u32 = 41;
pub const JET_paramCacheSizeMax: u32 = 23;
pub const JET_paramCacheSizeMin: u32 = 60;
pub const JET_paramCachedClosedTables: u32 = 125;
pub const JET_paramCheckFormatWhenOpenFail: u32 = 44;
pub const JET_paramCheckpointDepthMax: u32 = 24;
pub const JET_paramCheckpointIOMax: u32 = 135;
pub const JET_paramCircularLog: u32 = 17;
pub const JET_paramCleanupMismatchedLogFiles: u32 = 77;
pub const JET_paramCommitDefault: u32 = 16;
pub const JET_paramConfigStoreSpec: u32 = 189;
pub const JET_paramConfiguration: u32 = 129;
pub const JET_paramCreatePathIfNotExist: u32 = 100;
pub const JET_paramDatabasePageSize: u32 = 64;
pub const JET_paramDbExtensionSize: u32 = 18;
pub const JET_paramDbScanIntervalMaxSec: u32 = 172;
pub const JET_paramDbScanIntervalMinSec: u32 = 171;
pub const JET_paramDbScanThrottle: u32 = 170;
pub const JET_paramDefragmentSequentialBTrees: u32 = 160;
pub const JET_paramDefragmentSequentialBTreesDensityCheckFrequency: u32 = 161;
pub const JET_paramDeleteOldLogs: u32 = 48;
pub const JET_paramDeleteOutOfRangeLogs: u32 = 52;
pub const JET_paramDisableCallbacks: u32 = 65;
pub const JET_paramDisablePerfmon: u32 = 107;
pub const JET_paramDurableCommitCallback: u32 = 187;
pub const JET_paramEnableAdvanced: u32 = 130;
pub const JET_paramEnableDBScanInRecovery: u32 = 169;
pub const JET_paramEnableDBScanSerialization: u32 = 180;
pub const JET_paramEnableFileCache: u32 = 126;
pub const JET_paramEnableIndexChecking: u32 = 45;
pub const JET_paramEnableIndexCleanup: u32 = 54;
pub const JET_paramEnableOnlineDefrag: u32 = 35;
pub const JET_paramEnablePersistedCallbacks: u32 = 156;
pub const JET_paramEnableShrinkDatabase: u32 = 184;
pub const JET_paramEnableSqm: u32 = 188;
pub const JET_paramEnableTempTableVersioning: u32 = 46;
pub const JET_paramEnableViewCache: u32 = 127;
pub const JET_paramEngineFormatVersion: u32 = 194;
pub const JET_paramErrorToString: u32 = 70;
pub const JET_paramEventLogCache: u32 = 99;
pub const JET_paramEventLoggingLevel: u32 = 51;
pub const JET_paramEventSource: u32 = 4;
pub const JET_paramEventSourceKey: u32 = 49;
pub const JET_paramExceptionAction: u32 = 98;
pub const JET_paramGlobalMinVerPages: u32 = 81;
pub const JET_paramHungIOActions: u32 = 182;
pub const JET_paramHungIOThreshold: u32 = 181;
pub const JET_paramIOPriority: u32 = 152;
pub const JET_paramIOThrottlingTimeQuanta: u32 = 162;
pub const JET_paramIgnoreLogVersion: u32 = 47;
pub const JET_paramIndexTupleIncrement: u32 = 132;
pub const JET_paramIndexTupleStart: u32 = 133;
pub const JET_paramIndexTuplesLengthMax: u32 = 111;
pub const JET_paramIndexTuplesLengthMin: u32 = 110;
pub const JET_paramIndexTuplesToIndexMax: u32 = 112;
pub const JET_paramKeyMost: u32 = 134;
pub const JET_paramLRUKCorrInterval: u32 = 25;
pub const JET_paramLRUKHistoryMax: u32 = 26;
pub const JET_paramLRUKPolicy: u32 = 27;
pub const JET_paramLRUKTimeout: u32 = 28;
pub const JET_paramLRUKTrxCorrInterval: u32 = 29;
pub const JET_paramLVChunkSizeMost: u32 = 163;
pub const JET_paramLegacyFileNames: u32 = 136;
pub const JET_paramLogBuffers: u32 = 12;
pub const JET_paramLogCheckpointPeriod: u32 = 14;
pub const JET_paramLogFileCreateAsynch: u32 = 69;
pub const JET_paramLogFilePath: u32 = 2;
pub const JET_paramLogFileSize: u32 = 11;
pub const JET_paramLogWaitingUserMax: u32 = 15;
pub const JET_paramMaxCoalesceReadGapSize: u32 = 166;
pub const JET_paramMaxCoalesceReadSize: u32 = 164;
pub const JET_paramMaxCoalesceWriteGapSize: u32 = 167;
pub const JET_paramMaxCoalesceWriteSize: u32 = 165;
pub const JET_paramMaxColtyp: u32 = 131;
pub const JET_paramMaxCursors: u32 = 8;
pub const JET_paramMaxInstances: u32 = 104;
pub const JET_paramMaxOpenTables: u32 = 6;
pub const JET_paramMaxSessions: u32 = 5;
pub const JET_paramMaxTemporaryTables: u32 = 10;
pub const JET_paramMaxTransactionSize: u32 = 178;
pub const JET_paramMaxValueInvalid: u32 = 249;
pub const JET_paramMaxVerPages: u32 = 9;
pub const JET_paramMinDataForXpress: u32 = 183;
pub const JET_paramNoInformationEvent: u32 = 50;
pub const JET_paramOSSnapshotTimeout: u32 = 82;
pub const JET_paramOneDatabasePerSession: u32 = 102;
pub const JET_paramOutstandingIOMax: u32 = 30;
pub const JET_paramPageFragment: u32 = 20;
pub const JET_paramPageHintCacheSize: u32 = 101;
pub const JET_paramPageTempDBMin: u32 = 19;
pub const JET_paramPreferredMaxOpenTables: u32 = 7;
pub const JET_paramPreferredVerPages: u32 = 63;
pub const JET_paramPrereadIOMax: u32 = 179;
pub const JET_paramProcessFriendlyName: u32 = 186;
pub const JET_paramRecordUpgradeDirtyLevel: u32 = 78;
pub const JET_paramRecovery: u32 = 34;
pub const JET_paramRuntimeCallback: u32 = 73;
pub const JET_paramStartFlushThreshold: u32 = 31;
pub const JET_paramStopFlushThreshold: u32 = 32;
pub const JET_paramSystemPath: u32 = 0;
pub const JET_paramTableClass10Name: u32 = 146;
pub const JET_paramTableClass11Name: u32 = 147;
pub const JET_paramTableClass12Name: u32 = 148;
pub const JET_paramTableClass13Name: u32 = 149;
pub const JET_paramTableClass14Name: u32 = 150;
pub const JET_paramTableClass15Name: u32 = 151;
pub const JET_paramTableClass1Name: u32 = 137;
pub const JET_paramTableClass2Name: u32 = 138;
pub const JET_paramTableClass3Name: u32 = 139;
pub const JET_paramTableClass4Name: u32 = 140;
pub const JET_paramTableClass5Name: u32 = 141;
pub const JET_paramTableClass6Name: u32 = 142;
pub const JET_paramTableClass7Name: u32 = 143;
pub const JET_paramTableClass8Name: u32 = 144;
pub const JET_paramTableClass9Name: u32 = 145;
pub const JET_paramTempPath: u32 = 1;
pub const JET_paramTraceFlags: u32 = 223;
pub const JET_paramUnicodeIndexDefault: u32 = 72;
pub const JET_paramVerPageSize: u32 = 128;
pub const JET_paramVersionStoreTaskQueueMax: u32 = 105;
pub const JET_paramWaitLogFlush: u32 = 13;
pub const JET_paramWaypointLatency: u32 = 153;
pub const JET_paramZeroDatabaseDuringBackup: u32 = 71;
pub const JET_prepCancel: u32 = 3;
pub const JET_prepInsert: u32 = 0;
pub const JET_prepInsertCopy: u32 = 5;
pub const JET_prepInsertCopyDeleteOriginal: u32 = 7;
pub const JET_prepInsertCopyReplaceOriginal: u32 = 9;
pub const JET_prepReplace: u32 = 2;
pub const JET_prepReplaceNoLock: u32 = 4;
pub const JET_relopBitmaskEqualsZero: JET_RELOP = 7;
pub const JET_relopBitmaskNotEqualsZero: JET_RELOP = 8;
pub const JET_relopEquals: JET_RELOP = 0;
pub const JET_relopGreaterThan: JET_RELOP = 6;
pub const JET_relopGreaterThanOrEqual: JET_RELOP = 5;
pub const JET_relopLessThan: JET_RELOP = 4;
pub const JET_relopLessThanOrEqual: JET_RELOP = 3;
pub const JET_relopNotEquals: JET_RELOP = 2;
pub const JET_relopPrefixEquals: JET_RELOP = 1;
pub const JET_sesidNil: i32 = -1;
pub const JET_sesparamCommitDefault: u32 = 4097;
pub const JET_sesparamCorrelationID: u32 = 4101;
pub const JET_sesparamMaxValueInvalid: u32 = 4111;
pub const JET_sesparamOperationContext: u32 = 4100;
pub const JET_sesparamTransactionLevel: u32 = 4099;
pub const JET_snpBackup: u32 = 9;
pub const JET_snpCompact: u32 = 4;
pub const JET_snpRepair: u32 = 2;
pub const JET_snpRestore: u32 = 8;
pub const JET_snpScrub: u32 = 11;
pub const JET_snpUpgrade: u32 = 10;
pub const JET_snpUpgradeRecordFormat: u32 = 12;
pub const JET_sntBegin: u32 = 5;
pub const JET_sntComplete: u32 = 6;
pub const JET_sntFail: u32 = 3;
pub const JET_sntProgress: u32 = 0;
pub const JET_sntRequirements: u32 = 7;
pub const JET_sqmDisable: u32 = 0;
pub const JET_sqmEnable: u32 = 1;
pub const JET_sqmFromCEIP: u32 = 2;
pub const JET_tableidNil: i32 = -1;
pub const JET_wrnBufferTruncated: u32 = 1006;
pub const JET_wrnCallbackNotRegistered: u32 = 2100;
pub const JET_wrnColumnDefault: u32 = 1537;
pub const JET_wrnColumnMaxTruncated: u32 = 1512;
pub const JET_wrnColumnMoreTags: u32 = 1533;
pub const JET_wrnColumnNotInRecord: u32 = 1539;
pub const JET_wrnColumnNotLocal: u32 = 1532;
pub const JET_wrnColumnNull: u32 = 1004;
pub const JET_wrnColumnPresent: u32 = 1535;
pub const JET_wrnColumnReference: u32 = 1541;
pub const JET_wrnColumnSetNull: u32 = 1068;
pub const JET_wrnColumnSingleValue: u32 = 1536;
pub const JET_wrnColumnSkipped: u32 = 1531;
pub const JET_wrnColumnTruncated: u32 = 1534;
pub const JET_wrnCommittedLogFilesLost: u32 = 585;
pub const JET_wrnCommittedLogFilesRemoved: u32 = 587;
pub const JET_wrnCopyCompletedAlready: u32 = 8002;
pub const JET_wrnCopyLongValue: u32 = 1520;
pub const JET_wrnCorruptIndexDeleted: u32 = 1415;
pub const JET_wrnDataHasChanged: u32 = 1610;
pub const JET_wrnDatabaseAttached: u32 = 1007;
pub const JET_wrnDatabaseRepaired: u32 = 595;
pub const JET_wrnDefragAlreadyRunning: u32 = 2000;
pub const JET_wrnDefragNotRunning: u32 = 2001;
pub const JET_wrnExistingLogFileHasBadSignature: u32 = 558;
pub const JET_wrnExistingLogFileIsNotContiguous: u32 = 559;
pub const JET_wrnFileOpenReadOnly: u32 = 1813;
pub const JET_wrnFinishWithUndo: u32 = 588;
pub const JET_wrnIdleFull: u32 = 1908;
pub const JET_wrnKeyChanged: u32 = 1618;
pub const JET_wrnNoErrorInfo: u32 = 1055;
pub const JET_wrnNoIdleActivity: u32 = 1058;
pub const JET_wrnNoWriteLock: u32 = 1067;
pub const JET_wrnNyi: i32 = -1;
pub const JET_wrnPrimaryIndexOutOfDate: u32 = 1417;
pub const JET_wrnRecordFoundGreater: u32 = 1039;
pub const JET_wrnRecordFoundLess: u32 = 1039;
pub const JET_wrnRemainingVersions: u32 = 321;
pub const JET_wrnSecondaryIndexOutOfDate: u32 = 1418;
pub const JET_wrnSeekNotEqual: u32 = 1039;
pub const JET_wrnSeparateLongValue: u32 = 406;
pub const JET_wrnShrinkNotPossible: u32 = 1122;
pub const JET_wrnSkipThisRecord: u32 = 564;
pub const JET_wrnSortOverflow: u32 = 1009;
pub const JET_wrnTableEmpty: u32 = 1301;
pub const JET_wrnTableInUseBySystem: u32 = 1327;
pub const JET_wrnTargetInstanceRunning: u32 = 578;
pub const JET_wrnUniqueKey: u32 = 345;
pub const JET_wszConfigStoreReadControl: windows_sys::core::PCWSTR = windows_sys::core::w!("CsReadControl");
pub const JET_wszConfigStoreRelPathSysParamDefault: windows_sys::core::PCWSTR = windows_sys::core::w!("SysParamDefault");
pub const JET_wszConfigStoreRelPathSysParamOverride: windows_sys::core::PCWSTR = windows_sys::core::w!("SysParamOverride");
pub const cColumnInfoCols: u32 = 14;
pub const cIndexInfoCols: u32 = 15;
pub const cObjectInfoCols: u32 = 9;
pub const wrnBTNotVisibleAccumulated: u32 = 353;
pub const wrnBTNotVisibleRejected: u32 = 352;
