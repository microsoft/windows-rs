#[inline]
pub unsafe fn JetAddColumnA(sesid: JET_SESID, tableid: JET_TABLEID, szcolumnname: *const JET_CHAR, pcolumndef: *const JET_COLUMNDEF, pvdefault: Option<JET_PCVOID>, cbdefault: JET_UINT32, pcolumnid: Option<*mut JET_COLUMNID>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetAddColumnA(sesid : JET_SESID, tableid : JET_TABLEID, szcolumnname : *const JET_CHAR, pcolumndef : *const JET_COLUMNDEF, pvdefault : JET_PCVOID, cbdefault : JET_UINT32, pcolumnid : *mut JET_COLUMNID) -> JET_ERR);
    unsafe { JetAddColumnA(sesid, tableid, szcolumnname, pcolumndef, pvdefault.unwrap_or(core::mem::zeroed()) as _, cbdefault, pcolumnid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetAddColumnW(sesid: JET_SESID, tableid: JET_TABLEID, szcolumnname: *const JET_WCHAR, pcolumndef: *const JET_COLUMNDEF, pvdefault: Option<JET_PCVOID>, cbdefault: JET_UINT32, pcolumnid: Option<*mut JET_COLUMNID>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetAddColumnW(sesid : JET_SESID, tableid : JET_TABLEID, szcolumnname : *const JET_WCHAR, pcolumndef : *const JET_COLUMNDEF, pvdefault : JET_PCVOID, cbdefault : JET_UINT32, pcolumnid : *mut JET_COLUMNID) -> JET_ERR);
    unsafe { JetAddColumnW(sesid, tableid, szcolumnname, pcolumndef, pvdefault.unwrap_or(core::mem::zeroed()) as _, cbdefault, pcolumnid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetAttachDatabase2A(sesid: JET_SESID, szfilename: *const JET_CHAR, cpgdatabasesizemax: JET_UINT32, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetAttachDatabase2A(sesid : JET_SESID, szfilename : *const JET_CHAR, cpgdatabasesizemax : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetAttachDatabase2A(sesid, szfilename, cpgdatabasesizemax, grbit) }
}
#[inline]
pub unsafe fn JetAttachDatabase2W(sesid: JET_SESID, szfilename: *const JET_WCHAR, cpgdatabasesizemax: JET_UINT32, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetAttachDatabase2W(sesid : JET_SESID, szfilename : *const JET_WCHAR, cpgdatabasesizemax : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetAttachDatabase2W(sesid, szfilename, cpgdatabasesizemax, grbit) }
}
#[inline]
pub unsafe fn JetAttachDatabaseA(sesid: JET_SESID, szfilename: *const JET_CHAR, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetAttachDatabaseA(sesid : JET_SESID, szfilename : *const JET_CHAR, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetAttachDatabaseA(sesid, szfilename, grbit) }
}
#[inline]
pub unsafe fn JetAttachDatabaseW(sesid: JET_SESID, szfilename: *const JET_WCHAR, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetAttachDatabaseW(sesid : JET_SESID, szfilename : *const JET_WCHAR, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetAttachDatabaseW(sesid, szfilename, grbit) }
}
#[inline]
pub unsafe fn JetBackupA(szbackuppath: *const JET_CHAR, grbit: JET_GRBIT, pfnstatus: JET_PFNSTATUS) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetBackupA(szbackuppath : *const JET_CHAR, grbit : JET_GRBIT, pfnstatus : JET_PFNSTATUS) -> JET_ERR);
    unsafe { JetBackupA(szbackuppath, grbit, pfnstatus) }
}
#[inline]
pub unsafe fn JetBackupInstanceA(instance: JET_INSTANCE, szbackuppath: *const JET_CHAR, grbit: JET_GRBIT, pfnstatus: JET_PFNSTATUS) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetBackupInstanceA(instance : JET_INSTANCE, szbackuppath : *const JET_CHAR, grbit : JET_GRBIT, pfnstatus : JET_PFNSTATUS) -> JET_ERR);
    unsafe { JetBackupInstanceA(instance, szbackuppath, grbit, pfnstatus) }
}
#[inline]
pub unsafe fn JetBackupInstanceW(instance: JET_INSTANCE, szbackuppath: *const JET_WCHAR, grbit: JET_GRBIT, pfnstatus: JET_PFNSTATUS) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetBackupInstanceW(instance : JET_INSTANCE, szbackuppath : *const JET_WCHAR, grbit : JET_GRBIT, pfnstatus : JET_PFNSTATUS) -> JET_ERR);
    unsafe { JetBackupInstanceW(instance, szbackuppath, grbit, pfnstatus) }
}
#[inline]
pub unsafe fn JetBackupW(szbackuppath: *const JET_WCHAR, grbit: JET_GRBIT, pfnstatus: JET_PFNSTATUS) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetBackupW(szbackuppath : *const JET_WCHAR, grbit : JET_GRBIT, pfnstatus : JET_PFNSTATUS) -> JET_ERR);
    unsafe { JetBackupW(szbackuppath, grbit, pfnstatus) }
}
#[inline]
pub unsafe fn JetBeginExternalBackup(grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetBeginExternalBackup(grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetBeginExternalBackup(grbit) }
}
#[inline]
pub unsafe fn JetBeginExternalBackupInstance(instance: JET_INSTANCE, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetBeginExternalBackupInstance(instance : JET_INSTANCE, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetBeginExternalBackupInstance(instance, grbit) }
}
#[inline]
pub unsafe fn JetBeginSessionA(instance: JET_INSTANCE, psesid: *mut JET_SESID, szusername: Option<*const JET_CHAR>, szpassword: Option<*const JET_CHAR>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetBeginSessionA(instance : JET_INSTANCE, psesid : *mut JET_SESID, szusername : *const JET_CHAR, szpassword : *const JET_CHAR) -> JET_ERR);
    unsafe { JetBeginSessionA(instance, psesid as _, szusername.unwrap_or(core::mem::zeroed()) as _, szpassword.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetBeginSessionW(instance: JET_INSTANCE, psesid: *mut JET_SESID, szusername: Option<*const JET_WCHAR>, szpassword: Option<*const JET_WCHAR>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetBeginSessionW(instance : JET_INSTANCE, psesid : *mut JET_SESID, szusername : *const JET_WCHAR, szpassword : *const JET_WCHAR) -> JET_ERR);
    unsafe { JetBeginSessionW(instance, psesid as _, szusername.unwrap_or(core::mem::zeroed()) as _, szpassword.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetBeginTransaction(sesid: JET_SESID) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetBeginTransaction(sesid : JET_SESID) -> JET_ERR);
    unsafe { JetBeginTransaction(sesid) }
}
#[inline]
pub unsafe fn JetBeginTransaction2(sesid: JET_SESID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetBeginTransaction2(sesid : JET_SESID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetBeginTransaction2(sesid, grbit) }
}
#[inline]
pub unsafe fn JetBeginTransaction3(sesid: JET_SESID, trxid: JET_INT64, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetBeginTransaction3(sesid : JET_SESID, trxid : JET_INT64, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetBeginTransaction3(sesid, trxid, grbit) }
}
#[inline]
pub unsafe fn JetCloseDatabase(sesid: JET_SESID, dbid: JET_DBID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCloseDatabase(sesid : JET_SESID, dbid : JET_DBID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetCloseDatabase(sesid, dbid, grbit) }
}
#[inline]
pub unsafe fn JetCloseFile(hffile: JET_HANDLE) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCloseFile(hffile : JET_HANDLE) -> JET_ERR);
    unsafe { JetCloseFile(hffile) }
}
#[inline]
pub unsafe fn JetCloseFileInstance(instance: JET_INSTANCE, hffile: JET_HANDLE) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCloseFileInstance(instance : JET_INSTANCE, hffile : JET_HANDLE) -> JET_ERR);
    unsafe { JetCloseFileInstance(instance, hffile) }
}
#[inline]
pub unsafe fn JetCloseTable(sesid: JET_SESID, tableid: JET_TABLEID) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCloseTable(sesid : JET_SESID, tableid : JET_TABLEID) -> JET_ERR);
    unsafe { JetCloseTable(sesid, tableid) }
}
#[inline]
pub unsafe fn JetCommitTransaction(sesid: JET_SESID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCommitTransaction(sesid : JET_SESID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetCommitTransaction(sesid, grbit) }
}
#[inline]
pub unsafe fn JetCommitTransaction2(sesid: JET_SESID, grbit: JET_GRBIT, cmsecdurablecommit: JET_UINT32, pcommitid: Option<*mut JET_COMMIT_ID>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCommitTransaction2(sesid : JET_SESID, grbit : JET_GRBIT, cmsecdurablecommit : JET_UINT32, pcommitid : *mut JET_COMMIT_ID) -> JET_ERR);
    unsafe { JetCommitTransaction2(sesid, grbit, cmsecdurablecommit, pcommitid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetCompactA(sesid: JET_SESID, szdatabasesrc: *const JET_CHAR, szdatabasedest: *const JET_CHAR, pfnstatus: JET_PFNSTATUS, pconvert: Option<*const JET_CONVERT_A>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCompactA(sesid : JET_SESID, szdatabasesrc : *const JET_CHAR, szdatabasedest : *const JET_CHAR, pfnstatus : JET_PFNSTATUS, pconvert : *const JET_CONVERT_A, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetCompactA(sesid, szdatabasesrc, szdatabasedest, pfnstatus, pconvert.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[inline]
pub unsafe fn JetCompactW(sesid: JET_SESID, szdatabasesrc: *const JET_WCHAR, szdatabasedest: *const JET_WCHAR, pfnstatus: JET_PFNSTATUS, pconvert: Option<*const JET_CONVERT_W>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCompactW(sesid : JET_SESID, szdatabasesrc : *const JET_WCHAR, szdatabasedest : *const JET_WCHAR, pfnstatus : JET_PFNSTATUS, pconvert : *const JET_CONVERT_W, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetCompactW(sesid, szdatabasesrc, szdatabasedest, pfnstatus, pconvert.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[inline]
pub unsafe fn JetComputeStats(sesid: JET_SESID, tableid: JET_TABLEID) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetComputeStats(sesid : JET_SESID, tableid : JET_TABLEID) -> JET_ERR);
    unsafe { JetComputeStats(sesid, tableid) }
}
#[inline]
pub unsafe fn JetConfigureProcessForCrashDump(grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetConfigureProcessForCrashDump(grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetConfigureProcessForCrashDump(grbit) }
}
#[inline]
pub unsafe fn JetCreateDatabase2A(sesid: JET_SESID, szfilename: *const JET_CHAR, cpgdatabasesizemax: JET_UINT32, pdbid: *mut JET_DBID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateDatabase2A(sesid : JET_SESID, szfilename : *const JET_CHAR, cpgdatabasesizemax : JET_UINT32, pdbid : *mut JET_DBID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetCreateDatabase2A(sesid, szfilename, cpgdatabasesizemax, pdbid as _, grbit) }
}
#[inline]
pub unsafe fn JetCreateDatabase2W(sesid: JET_SESID, szfilename: *const JET_WCHAR, cpgdatabasesizemax: JET_UINT32, pdbid: *mut JET_DBID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateDatabase2W(sesid : JET_SESID, szfilename : *const JET_WCHAR, cpgdatabasesizemax : JET_UINT32, pdbid : *mut JET_DBID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetCreateDatabase2W(sesid, szfilename, cpgdatabasesizemax, pdbid as _, grbit) }
}
#[inline]
pub unsafe fn JetCreateDatabaseA(sesid: JET_SESID, szfilename: *const JET_CHAR, szconnect: Option<*const JET_CHAR>, pdbid: *mut JET_DBID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateDatabaseA(sesid : JET_SESID, szfilename : *const JET_CHAR, szconnect : *const JET_CHAR, pdbid : *mut JET_DBID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetCreateDatabaseA(sesid, szfilename, szconnect.unwrap_or(core::mem::zeroed()) as _, pdbid as _, grbit) }
}
#[inline]
pub unsafe fn JetCreateDatabaseW(sesid: JET_SESID, szfilename: *const JET_WCHAR, szconnect: Option<*const JET_WCHAR>, pdbid: *mut JET_DBID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateDatabaseW(sesid : JET_SESID, szfilename : *const JET_WCHAR, szconnect : *const JET_WCHAR, pdbid : *mut JET_DBID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetCreateDatabaseW(sesid, szfilename, szconnect.unwrap_or(core::mem::zeroed()) as _, pdbid as _, grbit) }
}
#[inline]
pub unsafe fn JetCreateIndex2A(sesid: JET_SESID, tableid: JET_TABLEID, pindexcreate: *const JET_INDEXCREATE_A, cindexcreate: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateIndex2A(sesid : JET_SESID, tableid : JET_TABLEID, pindexcreate : *const JET_INDEXCREATE_A, cindexcreate : JET_UINT32) -> JET_ERR);
    unsafe { JetCreateIndex2A(sesid, tableid, pindexcreate, cindexcreate) }
}
#[inline]
pub unsafe fn JetCreateIndex2W(sesid: JET_SESID, tableid: JET_TABLEID, pindexcreate: *const JET_INDEXCREATE_W, cindexcreate: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateIndex2W(sesid : JET_SESID, tableid : JET_TABLEID, pindexcreate : *const JET_INDEXCREATE_W, cindexcreate : JET_UINT32) -> JET_ERR);
    unsafe { JetCreateIndex2W(sesid, tableid, pindexcreate, cindexcreate) }
}
#[inline]
pub unsafe fn JetCreateIndex3A(sesid: JET_SESID, tableid: JET_TABLEID, pindexcreate: *const JET_INDEXCREATE2_A, cindexcreate: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateIndex3A(sesid : JET_SESID, tableid : JET_TABLEID, pindexcreate : *const JET_INDEXCREATE2_A, cindexcreate : JET_UINT32) -> JET_ERR);
    unsafe { JetCreateIndex3A(sesid, tableid, pindexcreate, cindexcreate) }
}
#[inline]
pub unsafe fn JetCreateIndex3W(sesid: JET_SESID, tableid: JET_TABLEID, pindexcreate: *const JET_INDEXCREATE2_W, cindexcreate: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateIndex3W(sesid : JET_SESID, tableid : JET_TABLEID, pindexcreate : *const JET_INDEXCREATE2_W, cindexcreate : JET_UINT32) -> JET_ERR);
    unsafe { JetCreateIndex3W(sesid, tableid, pindexcreate, cindexcreate) }
}
#[inline]
pub unsafe fn JetCreateIndex4A(sesid: JET_SESID, tableid: JET_TABLEID, pindexcreate: *const JET_INDEXCREATE3_A, cindexcreate: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateIndex4A(sesid : JET_SESID, tableid : JET_TABLEID, pindexcreate : *const JET_INDEXCREATE3_A, cindexcreate : JET_UINT32) -> JET_ERR);
    unsafe { JetCreateIndex4A(sesid, tableid, pindexcreate, cindexcreate) }
}
#[inline]
pub unsafe fn JetCreateIndex4W(sesid: JET_SESID, tableid: JET_TABLEID, pindexcreate: *const JET_INDEXCREATE3_W, cindexcreate: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateIndex4W(sesid : JET_SESID, tableid : JET_TABLEID, pindexcreate : *const JET_INDEXCREATE3_W, cindexcreate : JET_UINT32) -> JET_ERR);
    unsafe { JetCreateIndex4W(sesid, tableid, pindexcreate, cindexcreate) }
}
#[inline]
pub unsafe fn JetCreateIndexA(sesid: JET_SESID, tableid: JET_TABLEID, szindexname: *const JET_CHAR, grbit: JET_GRBIT, szkey: *const JET_CHAR, cbkey: JET_UINT32, ldensity: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateIndexA(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_CHAR, grbit : JET_GRBIT, szkey : *const JET_CHAR, cbkey : JET_UINT32, ldensity : JET_UINT32) -> JET_ERR);
    unsafe { JetCreateIndexA(sesid, tableid, szindexname, grbit, szkey, cbkey, ldensity) }
}
#[inline]
pub unsafe fn JetCreateIndexW(sesid: JET_SESID, tableid: JET_TABLEID, szindexname: *const JET_WCHAR, grbit: JET_GRBIT, szkey: *const JET_WCHAR, cbkey: JET_UINT32, ldensity: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateIndexW(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_WCHAR, grbit : JET_GRBIT, szkey : *const JET_WCHAR, cbkey : JET_UINT32, ldensity : JET_UINT32) -> JET_ERR);
    unsafe { JetCreateIndexW(sesid, tableid, szindexname, grbit, szkey, cbkey, ldensity) }
}
#[inline]
pub unsafe fn JetCreateInstance2A(pinstance: *mut JET_INSTANCE, szinstancename: Option<*const JET_CHAR>, szdisplayname: Option<*const JET_CHAR>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateInstance2A(pinstance : *mut JET_INSTANCE, szinstancename : *const JET_CHAR, szdisplayname : *const JET_CHAR, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetCreateInstance2A(pinstance as _, szinstancename.unwrap_or(core::mem::zeroed()) as _, szdisplayname.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[inline]
pub unsafe fn JetCreateInstance2W(pinstance: *mut JET_INSTANCE, szinstancename: Option<*const JET_WCHAR>, szdisplayname: Option<*const JET_WCHAR>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateInstance2W(pinstance : *mut JET_INSTANCE, szinstancename : *const JET_WCHAR, szdisplayname : *const JET_WCHAR, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetCreateInstance2W(pinstance as _, szinstancename.unwrap_or(core::mem::zeroed()) as _, szdisplayname.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[inline]
pub unsafe fn JetCreateInstanceA(pinstance: *mut JET_INSTANCE, szinstancename: Option<*const JET_CHAR>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateInstanceA(pinstance : *mut JET_INSTANCE, szinstancename : *const JET_CHAR) -> JET_ERR);
    unsafe { JetCreateInstanceA(pinstance as _, szinstancename.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetCreateInstanceW(pinstance: *mut JET_INSTANCE, szinstancename: Option<*const JET_WCHAR>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateInstanceW(pinstance : *mut JET_INSTANCE, szinstancename : *const JET_WCHAR) -> JET_ERR);
    unsafe { JetCreateInstanceW(pinstance as _, szinstancename.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetCreateTableA(sesid: JET_SESID, dbid: JET_DBID, sztablename: *const JET_CHAR, lpages: JET_UINT32, ldensity: JET_UINT32, ptableid: *mut JET_TABLEID) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateTableA(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_CHAR, lpages : JET_UINT32, ldensity : JET_UINT32, ptableid : *mut JET_TABLEID) -> JET_ERR);
    unsafe { JetCreateTableA(sesid, dbid, sztablename, lpages, ldensity, ptableid as _) }
}
#[inline]
pub unsafe fn JetCreateTableColumnIndex2A(sesid: JET_SESID, dbid: JET_DBID, ptablecreate: *mut JET_TABLECREATE2_A) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateTableColumnIndex2A(sesid : JET_SESID, dbid : JET_DBID, ptablecreate : *mut JET_TABLECREATE2_A) -> JET_ERR);
    unsafe { JetCreateTableColumnIndex2A(sesid, dbid, ptablecreate as _) }
}
#[inline]
pub unsafe fn JetCreateTableColumnIndex2W(sesid: JET_SESID, dbid: JET_DBID, ptablecreate: *mut JET_TABLECREATE2_W) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateTableColumnIndex2W(sesid : JET_SESID, dbid : JET_DBID, ptablecreate : *mut JET_TABLECREATE2_W) -> JET_ERR);
    unsafe { JetCreateTableColumnIndex2W(sesid, dbid, ptablecreate as _) }
}
#[inline]
pub unsafe fn JetCreateTableColumnIndex3A(sesid: JET_SESID, dbid: JET_DBID, ptablecreate: *mut JET_TABLECREATE3_A) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateTableColumnIndex3A(sesid : JET_SESID, dbid : JET_DBID, ptablecreate : *mut JET_TABLECREATE3_A) -> JET_ERR);
    unsafe { JetCreateTableColumnIndex3A(sesid, dbid, ptablecreate as _) }
}
#[inline]
pub unsafe fn JetCreateTableColumnIndex3W(sesid: JET_SESID, dbid: JET_DBID, ptablecreate: *mut JET_TABLECREATE3_W) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateTableColumnIndex3W(sesid : JET_SESID, dbid : JET_DBID, ptablecreate : *mut JET_TABLECREATE3_W) -> JET_ERR);
    unsafe { JetCreateTableColumnIndex3W(sesid, dbid, ptablecreate as _) }
}
#[inline]
pub unsafe fn JetCreateTableColumnIndex4A(sesid: JET_SESID, dbid: JET_DBID, ptablecreate: *mut JET_TABLECREATE4_A) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateTableColumnIndex4A(sesid : JET_SESID, dbid : JET_DBID, ptablecreate : *mut JET_TABLECREATE4_A) -> JET_ERR);
    unsafe { JetCreateTableColumnIndex4A(sesid, dbid, ptablecreate as _) }
}
#[inline]
pub unsafe fn JetCreateTableColumnIndex4W(sesid: JET_SESID, dbid: JET_DBID, ptablecreate: *mut JET_TABLECREATE4_W) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateTableColumnIndex4W(sesid : JET_SESID, dbid : JET_DBID, ptablecreate : *mut JET_TABLECREATE4_W) -> JET_ERR);
    unsafe { JetCreateTableColumnIndex4W(sesid, dbid, ptablecreate as _) }
}
#[inline]
pub unsafe fn JetCreateTableColumnIndexA(sesid: JET_SESID, dbid: JET_DBID, ptablecreate: *mut JET_TABLECREATE_A) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateTableColumnIndexA(sesid : JET_SESID, dbid : JET_DBID, ptablecreate : *mut JET_TABLECREATE_A) -> JET_ERR);
    unsafe { JetCreateTableColumnIndexA(sesid, dbid, ptablecreate as _) }
}
#[inline]
pub unsafe fn JetCreateTableColumnIndexW(sesid: JET_SESID, dbid: JET_DBID, ptablecreate: *mut JET_TABLECREATE_W) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateTableColumnIndexW(sesid : JET_SESID, dbid : JET_DBID, ptablecreate : *mut JET_TABLECREATE_W) -> JET_ERR);
    unsafe { JetCreateTableColumnIndexW(sesid, dbid, ptablecreate as _) }
}
#[inline]
pub unsafe fn JetCreateTableW(sesid: JET_SESID, dbid: JET_DBID, sztablename: *const JET_WCHAR, lpages: JET_UINT32, ldensity: JET_UINT32, ptableid: *mut JET_TABLEID) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetCreateTableW(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_WCHAR, lpages : JET_UINT32, ldensity : JET_UINT32, ptableid : *mut JET_TABLEID) -> JET_ERR);
    unsafe { JetCreateTableW(sesid, dbid, sztablename, lpages, ldensity, ptableid as _) }
}
#[inline]
pub unsafe fn JetDefragment2A(sesid: JET_SESID, dbid: JET_DBID, sztablename: Option<*const JET_CHAR>, pcpasses: Option<*mut JET_UINT32>, pcseconds: Option<*mut JET_UINT32>, callback: JET_CALLBACK, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDefragment2A(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_CHAR, pcpasses : *mut JET_UINT32, pcseconds : *mut JET_UINT32, callback : JET_CALLBACK, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetDefragment2A(sesid, dbid, sztablename.unwrap_or(core::mem::zeroed()) as _, pcpasses.unwrap_or(core::mem::zeroed()) as _, pcseconds.unwrap_or(core::mem::zeroed()) as _, callback, grbit) }
}
#[inline]
pub unsafe fn JetDefragment2W(sesid: JET_SESID, dbid: JET_DBID, sztablename: Option<*const JET_WCHAR>, pcpasses: Option<*mut JET_UINT32>, pcseconds: Option<*mut JET_UINT32>, callback: JET_CALLBACK, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDefragment2W(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_WCHAR, pcpasses : *mut JET_UINT32, pcseconds : *mut JET_UINT32, callback : JET_CALLBACK, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetDefragment2W(sesid, dbid, sztablename.unwrap_or(core::mem::zeroed()) as _, pcpasses.unwrap_or(core::mem::zeroed()) as _, pcseconds.unwrap_or(core::mem::zeroed()) as _, callback, grbit) }
}
#[inline]
pub unsafe fn JetDefragment3A(sesid: JET_SESID, szdatabasename: *const JET_CHAR, sztablename: Option<*const JET_CHAR>, pcpasses: Option<*mut JET_UINT32>, pcseconds: Option<*mut JET_UINT32>, callback: JET_CALLBACK, pvcontext: JET_PVOID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDefragment3A(sesid : JET_SESID, szdatabasename : *const JET_CHAR, sztablename : *const JET_CHAR, pcpasses : *mut JET_UINT32, pcseconds : *mut JET_UINT32, callback : JET_CALLBACK, pvcontext : JET_PVOID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetDefragment3A(sesid, szdatabasename, sztablename.unwrap_or(core::mem::zeroed()) as _, pcpasses.unwrap_or(core::mem::zeroed()) as _, pcseconds.unwrap_or(core::mem::zeroed()) as _, callback, pvcontext, grbit) }
}
#[inline]
pub unsafe fn JetDefragment3W(sesid: JET_SESID, szdatabasename: *const JET_WCHAR, sztablename: Option<*const JET_WCHAR>, pcpasses: Option<*mut JET_UINT32>, pcseconds: Option<*mut JET_UINT32>, callback: JET_CALLBACK, pvcontext: JET_PVOID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDefragment3W(sesid : JET_SESID, szdatabasename : *const JET_WCHAR, sztablename : *const JET_WCHAR, pcpasses : *mut JET_UINT32, pcseconds : *mut JET_UINT32, callback : JET_CALLBACK, pvcontext : JET_PVOID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetDefragment3W(sesid, szdatabasename, sztablename.unwrap_or(core::mem::zeroed()) as _, pcpasses.unwrap_or(core::mem::zeroed()) as _, pcseconds.unwrap_or(core::mem::zeroed()) as _, callback, pvcontext, grbit) }
}
#[inline]
pub unsafe fn JetDefragmentA(sesid: JET_SESID, dbid: JET_DBID, sztablename: Option<*const JET_CHAR>, pcpasses: Option<*mut JET_UINT32>, pcseconds: Option<*mut JET_UINT32>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDefragmentA(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_CHAR, pcpasses : *mut JET_UINT32, pcseconds : *mut JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetDefragmentA(sesid, dbid, sztablename.unwrap_or(core::mem::zeroed()) as _, pcpasses.unwrap_or(core::mem::zeroed()) as _, pcseconds.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[inline]
pub unsafe fn JetDefragmentW(sesid: JET_SESID, dbid: JET_DBID, sztablename: Option<*const JET_WCHAR>, pcpasses: Option<*mut JET_UINT32>, pcseconds: Option<*mut JET_UINT32>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDefragmentW(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_WCHAR, pcpasses : *mut JET_UINT32, pcseconds : *mut JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetDefragmentW(sesid, dbid, sztablename.unwrap_or(core::mem::zeroed()) as _, pcpasses.unwrap_or(core::mem::zeroed()) as _, pcseconds.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[inline]
pub unsafe fn JetDelete(sesid: JET_SESID, tableid: JET_TABLEID) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDelete(sesid : JET_SESID, tableid : JET_TABLEID) -> JET_ERR);
    unsafe { JetDelete(sesid, tableid) }
}
#[inline]
pub unsafe fn JetDeleteColumn2A(sesid: JET_SESID, tableid: JET_TABLEID, szcolumnname: *const JET_CHAR, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDeleteColumn2A(sesid : JET_SESID, tableid : JET_TABLEID, szcolumnname : *const JET_CHAR, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetDeleteColumn2A(sesid, tableid, szcolumnname, grbit) }
}
#[inline]
pub unsafe fn JetDeleteColumn2W(sesid: JET_SESID, tableid: JET_TABLEID, szcolumnname: *const JET_WCHAR, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDeleteColumn2W(sesid : JET_SESID, tableid : JET_TABLEID, szcolumnname : *const JET_WCHAR, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetDeleteColumn2W(sesid, tableid, szcolumnname, grbit) }
}
#[inline]
pub unsafe fn JetDeleteColumnA(sesid: JET_SESID, tableid: JET_TABLEID, szcolumnname: *const JET_CHAR) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDeleteColumnA(sesid : JET_SESID, tableid : JET_TABLEID, szcolumnname : *const JET_CHAR) -> JET_ERR);
    unsafe { JetDeleteColumnA(sesid, tableid, szcolumnname) }
}
#[inline]
pub unsafe fn JetDeleteColumnW(sesid: JET_SESID, tableid: JET_TABLEID, szcolumnname: *const JET_WCHAR) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDeleteColumnW(sesid : JET_SESID, tableid : JET_TABLEID, szcolumnname : *const JET_WCHAR) -> JET_ERR);
    unsafe { JetDeleteColumnW(sesid, tableid, szcolumnname) }
}
#[inline]
pub unsafe fn JetDeleteIndexA(sesid: JET_SESID, tableid: JET_TABLEID, szindexname: *const JET_CHAR) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDeleteIndexA(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_CHAR) -> JET_ERR);
    unsafe { JetDeleteIndexA(sesid, tableid, szindexname) }
}
#[inline]
pub unsafe fn JetDeleteIndexW(sesid: JET_SESID, tableid: JET_TABLEID, szindexname: *const JET_WCHAR) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDeleteIndexW(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_WCHAR) -> JET_ERR);
    unsafe { JetDeleteIndexW(sesid, tableid, szindexname) }
}
#[inline]
pub unsafe fn JetDeleteTableA(sesid: JET_SESID, dbid: JET_DBID, sztablename: *const JET_CHAR) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDeleteTableA(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_CHAR) -> JET_ERR);
    unsafe { JetDeleteTableA(sesid, dbid, sztablename) }
}
#[inline]
pub unsafe fn JetDeleteTableW(sesid: JET_SESID, dbid: JET_DBID, sztablename: *const JET_WCHAR) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDeleteTableW(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_WCHAR) -> JET_ERR);
    unsafe { JetDeleteTableW(sesid, dbid, sztablename) }
}
#[inline]
pub unsafe fn JetDetachDatabase2A(sesid: JET_SESID, szfilename: Option<*const JET_CHAR>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDetachDatabase2A(sesid : JET_SESID, szfilename : *const JET_CHAR, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetDetachDatabase2A(sesid, szfilename.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[inline]
pub unsafe fn JetDetachDatabase2W(sesid: JET_SESID, szfilename: Option<*const JET_WCHAR>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDetachDatabase2W(sesid : JET_SESID, szfilename : *const JET_WCHAR, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetDetachDatabase2W(sesid, szfilename.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[inline]
pub unsafe fn JetDetachDatabaseA(sesid: JET_SESID, szfilename: Option<*const JET_CHAR>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDetachDatabaseA(sesid : JET_SESID, szfilename : *const JET_CHAR) -> JET_ERR);
    unsafe { JetDetachDatabaseA(sesid, szfilename.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetDetachDatabaseW(sesid: JET_SESID, szfilename: Option<*const JET_WCHAR>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDetachDatabaseW(sesid : JET_SESID, szfilename : *const JET_WCHAR) -> JET_ERR);
    unsafe { JetDetachDatabaseW(sesid, szfilename.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetDupCursor(sesid: JET_SESID, tableid: JET_TABLEID, ptableid: *mut JET_TABLEID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDupCursor(sesid : JET_SESID, tableid : JET_TABLEID, ptableid : *mut JET_TABLEID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetDupCursor(sesid, tableid, ptableid as _, grbit) }
}
#[inline]
pub unsafe fn JetDupSession(sesid: JET_SESID, psesid: *mut JET_SESID) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetDupSession(sesid : JET_SESID, psesid : *mut JET_SESID) -> JET_ERR);
    unsafe { JetDupSession(sesid, psesid as _) }
}
#[inline]
pub unsafe fn JetEnableMultiInstanceA(psetsysparam: Option<*const JET_SETSYSPARAM_A>, csetsysparam: JET_UINT32, pcsetsucceed: Option<*mut JET_UINT32>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetEnableMultiInstanceA(psetsysparam : *const JET_SETSYSPARAM_A, csetsysparam : JET_UINT32, pcsetsucceed : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetEnableMultiInstanceA(psetsysparam.unwrap_or(core::mem::zeroed()) as _, csetsysparam, pcsetsucceed.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetEnableMultiInstanceW(psetsysparam: Option<*const JET_SETSYSPARAM_W>, csetsysparam: JET_UINT32, pcsetsucceed: Option<*mut JET_UINT32>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetEnableMultiInstanceW(psetsysparam : *const JET_SETSYSPARAM_W, csetsysparam : JET_UINT32, pcsetsucceed : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetEnableMultiInstanceW(psetsysparam.unwrap_or(core::mem::zeroed()) as _, csetsysparam, pcsetsucceed.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetEndExternalBackup() -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetEndExternalBackup() -> JET_ERR);
    unsafe { JetEndExternalBackup() }
}
#[inline]
pub unsafe fn JetEndExternalBackupInstance(instance: JET_INSTANCE) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetEndExternalBackupInstance(instance : JET_INSTANCE) -> JET_ERR);
    unsafe { JetEndExternalBackupInstance(instance) }
}
#[inline]
pub unsafe fn JetEndExternalBackupInstance2(instance: JET_INSTANCE, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetEndExternalBackupInstance2(instance : JET_INSTANCE, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetEndExternalBackupInstance2(instance, grbit) }
}
#[inline]
pub unsafe fn JetEndSession(sesid: JET_SESID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetEndSession(sesid : JET_SESID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetEndSession(sesid, grbit) }
}
#[inline]
pub unsafe fn JetEnumerateColumns(sesid: JET_SESID, tableid: JET_TABLEID, cenumcolumnid: JET_UINT32, rgenumcolumnid: Option<*const JET_ENUMCOLUMNID>, pcenumcolumn: *mut JET_UINT32, prgenumcolumn: *mut *mut JET_ENUMCOLUMN, pfnrealloc: JET_PFNREALLOC, pvrealloccontext: Option<JET_PVOID>, cbdatamost: JET_UINT32, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetEnumerateColumns(sesid : JET_SESID, tableid : JET_TABLEID, cenumcolumnid : JET_UINT32, rgenumcolumnid : *const JET_ENUMCOLUMNID, pcenumcolumn : *mut JET_UINT32, prgenumcolumn : *mut *mut JET_ENUMCOLUMN, pfnrealloc : JET_PFNREALLOC, pvrealloccontext : JET_PVOID, cbdatamost : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetEnumerateColumns(sesid, tableid, cenumcolumnid, rgenumcolumnid.unwrap_or(core::mem::zeroed()) as _, pcenumcolumn as _, prgenumcolumn as _, pfnrealloc, pvrealloccontext.unwrap_or(core::mem::zeroed()) as _, cbdatamost, grbit) }
}
#[inline]
pub unsafe fn JetEscrowUpdate(sesid: JET_SESID, tableid: JET_TABLEID, columnid: JET_COLUMNID, pv: JET_PVOID, cbmax: JET_UINT32, pvold: Option<JET_PVOID>, cboldmax: JET_UINT32, pcboldactual: Option<*mut JET_UINT32>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetEscrowUpdate(sesid : JET_SESID, tableid : JET_TABLEID, columnid : JET_COLUMNID, pv : JET_PVOID, cbmax : JET_UINT32, pvold : JET_PVOID, cboldmax : JET_UINT32, pcboldactual : *mut JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetEscrowUpdate(sesid, tableid, columnid, pv, cbmax, pvold.unwrap_or(core::mem::zeroed()) as _, cboldmax, pcboldactual.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[inline]
pub unsafe fn JetExternalRestore2A(szcheckpointfilepath: *const JET_CHAR, szlogpath: *const JET_CHAR, rgrstmap: Option<*const JET_RSTMAP_A>, crstfilemap: JET_INT32, szbackuplogpath: *const JET_CHAR, ploginfo: *mut JET_LOGINFO_A, sztargetinstancename: Option<*const JET_CHAR>, sztargetinstancelogpath: Option<*const JET_CHAR>, sztargetinstancecheckpointpath: Option<*const JET_CHAR>, pfn: JET_PFNSTATUS) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetExternalRestore2A(szcheckpointfilepath : *const JET_CHAR, szlogpath : *const JET_CHAR, rgrstmap : *const JET_RSTMAP_A, crstfilemap : JET_INT32, szbackuplogpath : *const JET_CHAR, ploginfo : *mut JET_LOGINFO_A, sztargetinstancename : *const JET_CHAR, sztargetinstancelogpath : *const JET_CHAR, sztargetinstancecheckpointpath : *const JET_CHAR, pfn : JET_PFNSTATUS) -> JET_ERR);
    unsafe { JetExternalRestore2A(szcheckpointfilepath, szlogpath, rgrstmap.unwrap_or(core::mem::zeroed()) as _, crstfilemap, szbackuplogpath, ploginfo as _, sztargetinstancename.unwrap_or(core::mem::zeroed()) as _, sztargetinstancelogpath.unwrap_or(core::mem::zeroed()) as _, sztargetinstancecheckpointpath.unwrap_or(core::mem::zeroed()) as _, pfn) }
}
#[inline]
pub unsafe fn JetExternalRestore2W(szcheckpointfilepath: *const JET_WCHAR, szlogpath: *const JET_WCHAR, rgrstmap: Option<*const JET_RSTMAP_W>, crstfilemap: JET_INT32, szbackuplogpath: *const JET_WCHAR, ploginfo: *mut JET_LOGINFO_W, sztargetinstancename: Option<*const JET_WCHAR>, sztargetinstancelogpath: Option<*const JET_WCHAR>, sztargetinstancecheckpointpath: Option<*const JET_WCHAR>, pfn: JET_PFNSTATUS) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetExternalRestore2W(szcheckpointfilepath : *const JET_WCHAR, szlogpath : *const JET_WCHAR, rgrstmap : *const JET_RSTMAP_W, crstfilemap : JET_INT32, szbackuplogpath : *const JET_WCHAR, ploginfo : *mut JET_LOGINFO_W, sztargetinstancename : *const JET_WCHAR, sztargetinstancelogpath : *const JET_WCHAR, sztargetinstancecheckpointpath : *const JET_WCHAR, pfn : JET_PFNSTATUS) -> JET_ERR);
    unsafe { JetExternalRestore2W(szcheckpointfilepath, szlogpath, rgrstmap.unwrap_or(core::mem::zeroed()) as _, crstfilemap, szbackuplogpath, ploginfo as _, sztargetinstancename.unwrap_or(core::mem::zeroed()) as _, sztargetinstancelogpath.unwrap_or(core::mem::zeroed()) as _, sztargetinstancecheckpointpath.unwrap_or(core::mem::zeroed()) as _, pfn) }
}
#[inline]
pub unsafe fn JetExternalRestoreA(szcheckpointfilepath: *const JET_CHAR, szlogpath: *const JET_CHAR, rgrstmap: Option<*const JET_RSTMAP_A>, crstfilemap: JET_INT32, szbackuplogpath: *const JET_CHAR, genlow: JET_INT32, genhigh: JET_INT32, pfn: JET_PFNSTATUS) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetExternalRestoreA(szcheckpointfilepath : *const JET_CHAR, szlogpath : *const JET_CHAR, rgrstmap : *const JET_RSTMAP_A, crstfilemap : JET_INT32, szbackuplogpath : *const JET_CHAR, genlow : JET_INT32, genhigh : JET_INT32, pfn : JET_PFNSTATUS) -> JET_ERR);
    unsafe { JetExternalRestoreA(szcheckpointfilepath, szlogpath, rgrstmap.unwrap_or(core::mem::zeroed()) as _, crstfilemap, szbackuplogpath, genlow, genhigh, pfn) }
}
#[inline]
pub unsafe fn JetExternalRestoreW(szcheckpointfilepath: *const JET_WCHAR, szlogpath: *const JET_WCHAR, rgrstmap: Option<*const JET_RSTMAP_W>, crstfilemap: JET_INT32, szbackuplogpath: *const JET_WCHAR, genlow: JET_INT32, genhigh: JET_INT32, pfn: JET_PFNSTATUS) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetExternalRestoreW(szcheckpointfilepath : *const JET_WCHAR, szlogpath : *const JET_WCHAR, rgrstmap : *const JET_RSTMAP_W, crstfilemap : JET_INT32, szbackuplogpath : *const JET_WCHAR, genlow : JET_INT32, genhigh : JET_INT32, pfn : JET_PFNSTATUS) -> JET_ERR);
    unsafe { JetExternalRestoreW(szcheckpointfilepath, szlogpath, rgrstmap.unwrap_or(core::mem::zeroed()) as _, crstfilemap, szbackuplogpath, genlow, genhigh, pfn) }
}
#[inline]
pub unsafe fn JetFreeBuffer(pbbuf: *mut JET_CHAR) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetFreeBuffer(pbbuf : *mut JET_CHAR) -> JET_ERR);
    unsafe { JetFreeBuffer(pbbuf as _) }
}
#[inline]
pub unsafe fn JetGetAttachInfoA(szzdatabases: Option<*mut JET_CHAR>, cbmax: JET_UINT32, pcbactual: Option<*mut JET_UINT32>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetAttachInfoA(szzdatabases : *mut JET_CHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetGetAttachInfoA(szzdatabases.unwrap_or(core::mem::zeroed()) as _, cbmax, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetGetAttachInfoInstanceA(instance: JET_INSTANCE, szzdatabases: Option<*mut JET_CHAR>, cbmax: JET_UINT32, pcbactual: Option<*mut JET_UINT32>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetAttachInfoInstanceA(instance : JET_INSTANCE, szzdatabases : *mut JET_CHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetGetAttachInfoInstanceA(instance, szzdatabases.unwrap_or(core::mem::zeroed()) as _, cbmax, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetGetAttachInfoInstanceW(instance: JET_INSTANCE, szzdatabases: Option<*mut JET_WCHAR>, cbmax: JET_UINT32, pcbactual: Option<*mut JET_UINT32>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetAttachInfoInstanceW(instance : JET_INSTANCE, szzdatabases : *mut JET_WCHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetGetAttachInfoInstanceW(instance, szzdatabases.unwrap_or(core::mem::zeroed()) as _, cbmax, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetGetAttachInfoW(wszzdatabases: Option<*mut JET_WCHAR>, cbmax: JET_UINT32, pcbactual: Option<*mut JET_UINT32>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetAttachInfoW(wszzdatabases : *mut JET_WCHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetGetAttachInfoW(wszzdatabases.unwrap_or(core::mem::zeroed()) as _, cbmax, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetGetBookmark(sesid: JET_SESID, tableid: JET_TABLEID, pvbookmark: Option<JET_PVOID>, cbmax: JET_UINT32, pcbactual: Option<*mut JET_UINT32>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetBookmark(sesid : JET_SESID, tableid : JET_TABLEID, pvbookmark : JET_PVOID, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetGetBookmark(sesid, tableid, pvbookmark.unwrap_or(core::mem::zeroed()) as _, cbmax, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetGetColumnInfoA(sesid: JET_SESID, dbid: JET_DBID, sztablename: *const JET_CHAR, pcolumnnameorid: Option<*const JET_CHAR>, pvresult: JET_PVOID, cbmax: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetColumnInfoA(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_CHAR, pcolumnnameorid : *const JET_CHAR, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetColumnInfoA(sesid, dbid, sztablename, pcolumnnameorid.unwrap_or(core::mem::zeroed()) as _, pvresult as _, cbmax, infolevel) }
}
#[inline]
pub unsafe fn JetGetColumnInfoW(sesid: JET_SESID, dbid: JET_DBID, sztablename: *const JET_WCHAR, pwcolumnnameorid: Option<*const JET_WCHAR>, pvresult: JET_PVOID, cbmax: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetColumnInfoW(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_WCHAR, pwcolumnnameorid : *const JET_WCHAR, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetColumnInfoW(sesid, dbid, sztablename, pwcolumnnameorid.unwrap_or(core::mem::zeroed()) as _, pvresult as _, cbmax, infolevel) }
}
#[inline]
pub unsafe fn JetGetCurrentIndexA(sesid: JET_SESID, tableid: JET_TABLEID, szindexname: *mut JET_CHAR, cbindexname: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetCurrentIndexA(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *mut JET_CHAR, cbindexname : JET_UINT32) -> JET_ERR);
    unsafe { JetGetCurrentIndexA(sesid, tableid, szindexname as _, cbindexname) }
}
#[inline]
pub unsafe fn JetGetCurrentIndexW(sesid: JET_SESID, tableid: JET_TABLEID, szindexname: *mut JET_WCHAR, cbindexname: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetCurrentIndexW(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *mut JET_WCHAR, cbindexname : JET_UINT32) -> JET_ERR);
    unsafe { JetGetCurrentIndexW(sesid, tableid, szindexname as _, cbindexname) }
}
#[inline]
pub unsafe fn JetGetCursorInfo(sesid: JET_SESID, tableid: JET_TABLEID, pvresult: JET_PVOID, cbmax: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetCursorInfo(sesid : JET_SESID, tableid : JET_TABLEID, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetCursorInfo(sesid, tableid, pvresult as _, cbmax, infolevel) }
}
#[inline]
pub unsafe fn JetGetDatabaseFileInfoA(szdatabasename: *const JET_CHAR, pvresult: JET_PVOID, cbmax: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetDatabaseFileInfoA(szdatabasename : *const JET_CHAR, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetDatabaseFileInfoA(szdatabasename, pvresult as _, cbmax, infolevel) }
}
#[inline]
pub unsafe fn JetGetDatabaseFileInfoW(szdatabasename: *const JET_WCHAR, pvresult: JET_PVOID, cbmax: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetDatabaseFileInfoW(szdatabasename : *const JET_WCHAR, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetDatabaseFileInfoW(szdatabasename, pvresult as _, cbmax, infolevel) }
}
#[inline]
pub unsafe fn JetGetDatabaseInfoA(sesid: JET_SESID, dbid: JET_DBID, pvresult: JET_PVOID, cbmax: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetDatabaseInfoA(sesid : JET_SESID, dbid : JET_DBID, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetDatabaseInfoA(sesid, dbid, pvresult as _, cbmax, infolevel) }
}
#[inline]
pub unsafe fn JetGetDatabaseInfoW(sesid: JET_SESID, dbid: JET_DBID, pvresult: JET_PVOID, cbmax: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetDatabaseInfoW(sesid : JET_SESID, dbid : JET_DBID, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetDatabaseInfoW(sesid, dbid, pvresult as _, cbmax, infolevel) }
}
#[inline]
pub unsafe fn JetGetErrorInfoW(pvcontext: Option<JET_PVOID>, pvresult: JET_PVOID, cbmax: JET_UINT32, infolevel: JET_UINT32, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetErrorInfoW(pvcontext : JET_PVOID, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetGetErrorInfoW(pvcontext.unwrap_or(core::mem::zeroed()) as _, pvresult as _, cbmax, infolevel, grbit) }
}
#[inline]
pub unsafe fn JetGetIndexInfoA(sesid: JET_SESID, dbid: JET_DBID, sztablename: *const JET_CHAR, szindexname: Option<*const JET_CHAR>, pvresult: JET_PVOID, cbresult: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetIndexInfoA(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_CHAR, szindexname : *const JET_CHAR, pvresult : JET_PVOID, cbresult : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetIndexInfoA(sesid, dbid, sztablename, szindexname.unwrap_or(core::mem::zeroed()) as _, pvresult as _, cbresult, infolevel) }
}
#[inline]
pub unsafe fn JetGetIndexInfoW(sesid: JET_SESID, dbid: JET_DBID, sztablename: *const JET_WCHAR, szindexname: Option<*const JET_WCHAR>, pvresult: JET_PVOID, cbresult: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetIndexInfoW(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_WCHAR, szindexname : *const JET_WCHAR, pvresult : JET_PVOID, cbresult : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetIndexInfoW(sesid, dbid, sztablename, szindexname.unwrap_or(core::mem::zeroed()) as _, pvresult as _, cbresult, infolevel) }
}
#[inline]
pub unsafe fn JetGetInstanceInfoA(pcinstanceinfo: *mut JET_UINT32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_A) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetInstanceInfoA(pcinstanceinfo : *mut JET_UINT32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_A) -> JET_ERR);
    unsafe { JetGetInstanceInfoA(pcinstanceinfo as _, painstanceinfo as _) }
}
#[inline]
pub unsafe fn JetGetInstanceInfoW(pcinstanceinfo: *mut JET_UINT32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_W) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetInstanceInfoW(pcinstanceinfo : *mut JET_UINT32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_W) -> JET_ERR);
    unsafe { JetGetInstanceInfoW(pcinstanceinfo as _, painstanceinfo as _) }
}
#[inline]
pub unsafe fn JetGetInstanceMiscInfo(instance: JET_INSTANCE, pvresult: JET_PVOID, cbmax: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetInstanceMiscInfo(instance : JET_INSTANCE, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetInstanceMiscInfo(instance, pvresult as _, cbmax, infolevel) }
}
#[inline]
pub unsafe fn JetGetLS(sesid: JET_SESID, tableid: JET_TABLEID, pls: *mut JET_LS, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetLS(sesid : JET_SESID, tableid : JET_TABLEID, pls : *mut JET_LS, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetGetLS(sesid, tableid, pls as _, grbit) }
}
#[inline]
pub unsafe fn JetGetLock(sesid: JET_SESID, tableid: JET_TABLEID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetLock(sesid : JET_SESID, tableid : JET_TABLEID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetGetLock(sesid, tableid, grbit) }
}
#[inline]
pub unsafe fn JetGetLogInfoA(szzlogs: Option<*mut JET_CHAR>, cbmax: JET_UINT32, pcbactual: Option<*mut JET_UINT32>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetLogInfoA(szzlogs : *mut JET_CHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetGetLogInfoA(szzlogs.unwrap_or(core::mem::zeroed()) as _, cbmax, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetGetLogInfoInstance2A(instance: JET_INSTANCE, szzlogs: Option<*mut JET_CHAR>, cbmax: JET_UINT32, pcbactual: Option<*mut JET_UINT32>, ploginfo: Option<*mut JET_LOGINFO_A>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetLogInfoInstance2A(instance : JET_INSTANCE, szzlogs : *mut JET_CHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32, ploginfo : *mut JET_LOGINFO_A) -> JET_ERR);
    unsafe { JetGetLogInfoInstance2A(instance, szzlogs.unwrap_or(core::mem::zeroed()) as _, cbmax, pcbactual.unwrap_or(core::mem::zeroed()) as _, ploginfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetGetLogInfoInstance2W(instance: JET_INSTANCE, wszzlogs: Option<*mut JET_WCHAR>, cbmax: JET_UINT32, pcbactual: Option<*mut JET_UINT32>, ploginfo: Option<*mut JET_LOGINFO_W>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetLogInfoInstance2W(instance : JET_INSTANCE, wszzlogs : *mut JET_WCHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32, ploginfo : *mut JET_LOGINFO_W) -> JET_ERR);
    unsafe { JetGetLogInfoInstance2W(instance, wszzlogs.unwrap_or(core::mem::zeroed()) as _, cbmax, pcbactual.unwrap_or(core::mem::zeroed()) as _, ploginfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetGetLogInfoInstanceA(instance: JET_INSTANCE, szzlogs: Option<*mut JET_CHAR>, cbmax: JET_UINT32, pcbactual: Option<*mut JET_UINT32>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetLogInfoInstanceA(instance : JET_INSTANCE, szzlogs : *mut JET_CHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetGetLogInfoInstanceA(instance, szzlogs.unwrap_or(core::mem::zeroed()) as _, cbmax, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetGetLogInfoInstanceW(instance: JET_INSTANCE, wszzlogs: Option<*mut JET_WCHAR>, cbmax: JET_UINT32, pcbactual: Option<*mut JET_UINT32>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetLogInfoInstanceW(instance : JET_INSTANCE, wszzlogs : *mut JET_WCHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetGetLogInfoInstanceW(instance, wszzlogs.unwrap_or(core::mem::zeroed()) as _, cbmax, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetGetLogInfoW(szzlogs: Option<*mut JET_WCHAR>, cbmax: JET_UINT32, pcbactual: Option<*mut JET_UINT32>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetLogInfoW(szzlogs : *mut JET_WCHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetGetLogInfoW(szzlogs.unwrap_or(core::mem::zeroed()) as _, cbmax, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetGetObjectInfoA(sesid: JET_SESID, dbid: JET_DBID, objtyp: JET_OBJTYP, szcontainername: Option<*const JET_CHAR>, szobjectname: Option<*const JET_CHAR>, pvresult: JET_PVOID, cbmax: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetObjectInfoA(sesid : JET_SESID, dbid : JET_DBID, objtyp : JET_OBJTYP, szcontainername : *const JET_CHAR, szobjectname : *const JET_CHAR, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetObjectInfoA(sesid, dbid, objtyp, szcontainername.unwrap_or(core::mem::zeroed()) as _, szobjectname.unwrap_or(core::mem::zeroed()) as _, pvresult as _, cbmax, infolevel) }
}
#[inline]
pub unsafe fn JetGetObjectInfoW(sesid: JET_SESID, dbid: JET_DBID, objtyp: JET_OBJTYP, szcontainername: Option<*const JET_WCHAR>, szobjectname: Option<*const JET_WCHAR>, pvresult: JET_PVOID, cbmax: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetObjectInfoW(sesid : JET_SESID, dbid : JET_DBID, objtyp : JET_OBJTYP, szcontainername : *const JET_WCHAR, szobjectname : *const JET_WCHAR, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetObjectInfoW(sesid, dbid, objtyp, szcontainername.unwrap_or(core::mem::zeroed()) as _, szobjectname.unwrap_or(core::mem::zeroed()) as _, pvresult as _, cbmax, infolevel) }
}
#[inline]
pub unsafe fn JetGetRecordPosition(sesid: JET_SESID, tableid: JET_TABLEID, precpos: *mut JET_RECPOS, cbrecpos: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetRecordPosition(sesid : JET_SESID, tableid : JET_TABLEID, precpos : *mut JET_RECPOS, cbrecpos : JET_UINT32) -> JET_ERR);
    unsafe { JetGetRecordPosition(sesid, tableid, precpos as _, cbrecpos) }
}
#[inline]
pub unsafe fn JetGetRecordSize(sesid: JET_SESID, tableid: JET_TABLEID, precsize: *mut JET_RECSIZE, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetRecordSize(sesid : JET_SESID, tableid : JET_TABLEID, precsize : *mut JET_RECSIZE, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetGetRecordSize(sesid, tableid, precsize as _, grbit) }
}
#[inline]
pub unsafe fn JetGetRecordSize2(sesid: JET_SESID, tableid: JET_TABLEID, precsize: *mut JET_RECSIZE2, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetRecordSize2(sesid : JET_SESID, tableid : JET_TABLEID, precsize : *mut JET_RECSIZE2, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetGetRecordSize2(sesid, tableid, precsize as _, grbit) }
}
#[inline]
pub unsafe fn JetGetSecondaryIndexBookmark(sesid: JET_SESID, tableid: JET_TABLEID, pvsecondarykey: Option<JET_PVOID>, cbsecondarykeymax: JET_UINT32, pcbsecondarykeyactual: Option<*mut JET_UINT32>, pvprimarybookmark: Option<JET_PVOID>, cbprimarybookmarkmax: JET_UINT32, pcbprimarybookmarkactual: Option<*mut JET_UINT32>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetSecondaryIndexBookmark(sesid : JET_SESID, tableid : JET_TABLEID, pvsecondarykey : JET_PVOID, cbsecondarykeymax : JET_UINT32, pcbsecondarykeyactual : *mut JET_UINT32, pvprimarybookmark : JET_PVOID, cbprimarybookmarkmax : JET_UINT32, pcbprimarybookmarkactual : *mut JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetGetSecondaryIndexBookmark(sesid, tableid, pvsecondarykey.unwrap_or(core::mem::zeroed()) as _, cbsecondarykeymax, pcbsecondarykeyactual.unwrap_or(core::mem::zeroed()) as _, pvprimarybookmark.unwrap_or(core::mem::zeroed()) as _, cbprimarybookmarkmax, pcbprimarybookmarkactual.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[inline]
pub unsafe fn JetGetSessionParameter(sesid: Option<JET_SESID>, sesparamid: JET_UINT32, pvparam: JET_PVOID, cbparammax: JET_UINT32, pcbparamactual: Option<*mut JET_UINT32>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetSessionParameter(sesid : JET_SESID, sesparamid : JET_UINT32, pvparam : JET_PVOID, cbparammax : JET_UINT32, pcbparamactual : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetGetSessionParameter(sesid.unwrap_or(core::mem::zeroed()) as _, sesparamid, pvparam, cbparammax, pcbparamactual.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetGetSystemParameterA(instance: JET_INSTANCE, sesid: Option<JET_SESID>, paramid: JET_UINT32, plparam: Option<*mut JET_API_PTR>, szparam: Option<*mut JET_CHAR>, cbmax: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetSystemParameterA(instance : JET_INSTANCE, sesid : JET_SESID, paramid : JET_UINT32, plparam : *mut JET_API_PTR, szparam : *mut JET_CHAR, cbmax : JET_UINT32) -> JET_ERR);
    unsafe { JetGetSystemParameterA(instance, sesid.unwrap_or(core::mem::zeroed()) as _, paramid, plparam.unwrap_or(core::mem::zeroed()) as _, szparam.unwrap_or(core::mem::zeroed()) as _, cbmax) }
}
#[inline]
pub unsafe fn JetGetSystemParameterW(instance: JET_INSTANCE, sesid: Option<JET_SESID>, paramid: JET_UINT32, plparam: Option<*mut JET_API_PTR>, szparam: Option<*mut JET_WCHAR>, cbmax: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetSystemParameterW(instance : JET_INSTANCE, sesid : JET_SESID, paramid : JET_UINT32, plparam : *mut JET_API_PTR, szparam : *mut JET_WCHAR, cbmax : JET_UINT32) -> JET_ERR);
    unsafe { JetGetSystemParameterW(instance, sesid.unwrap_or(core::mem::zeroed()) as _, paramid, plparam.unwrap_or(core::mem::zeroed()) as _, szparam.unwrap_or(core::mem::zeroed()) as _, cbmax) }
}
#[inline]
pub unsafe fn JetGetTableColumnInfoA(sesid: JET_SESID, tableid: JET_TABLEID, szcolumnname: Option<*const JET_CHAR>, pvresult: JET_PVOID, cbmax: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetTableColumnInfoA(sesid : JET_SESID, tableid : JET_TABLEID, szcolumnname : *const JET_CHAR, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetTableColumnInfoA(sesid, tableid, szcolumnname.unwrap_or(core::mem::zeroed()) as _, pvresult as _, cbmax, infolevel) }
}
#[inline]
pub unsafe fn JetGetTableColumnInfoW(sesid: JET_SESID, tableid: JET_TABLEID, szcolumnname: Option<*const JET_WCHAR>, pvresult: JET_PVOID, cbmax: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetTableColumnInfoW(sesid : JET_SESID, tableid : JET_TABLEID, szcolumnname : *const JET_WCHAR, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetTableColumnInfoW(sesid, tableid, szcolumnname.unwrap_or(core::mem::zeroed()) as _, pvresult as _, cbmax, infolevel) }
}
#[inline]
pub unsafe fn JetGetTableIndexInfoA(sesid: JET_SESID, tableid: JET_TABLEID, szindexname: Option<*const JET_CHAR>, pvresult: JET_PVOID, cbresult: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetTableIndexInfoA(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_CHAR, pvresult : JET_PVOID, cbresult : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetTableIndexInfoA(sesid, tableid, szindexname.unwrap_or(core::mem::zeroed()) as _, pvresult as _, cbresult, infolevel) }
}
#[inline]
pub unsafe fn JetGetTableIndexInfoW(sesid: JET_SESID, tableid: JET_TABLEID, szindexname: Option<*const JET_WCHAR>, pvresult: JET_PVOID, cbresult: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetTableIndexInfoW(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_WCHAR, pvresult : JET_PVOID, cbresult : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetTableIndexInfoW(sesid, tableid, szindexname.unwrap_or(core::mem::zeroed()) as _, pvresult as _, cbresult, infolevel) }
}
#[inline]
pub unsafe fn JetGetTableInfoA(sesid: JET_SESID, tableid: JET_TABLEID, pvresult: JET_PVOID, cbmax: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetTableInfoA(sesid : JET_SESID, tableid : JET_TABLEID, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetTableInfoA(sesid, tableid, pvresult as _, cbmax, infolevel) }
}
#[inline]
pub unsafe fn JetGetTableInfoW(sesid: JET_SESID, tableid: JET_TABLEID, pvresult: JET_PVOID, cbmax: JET_UINT32, infolevel: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetTableInfoW(sesid : JET_SESID, tableid : JET_TABLEID, pvresult : JET_PVOID, cbmax : JET_UINT32, infolevel : JET_UINT32) -> JET_ERR);
    unsafe { JetGetTableInfoW(sesid, tableid, pvresult as _, cbmax, infolevel) }
}
#[inline]
pub unsafe fn JetGetThreadStats(pvresult: JET_PVOID, cbmax: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetThreadStats(pvresult : JET_PVOID, cbmax : JET_UINT32) -> JET_ERR);
    unsafe { JetGetThreadStats(pvresult as _, cbmax) }
}
#[inline]
pub unsafe fn JetGetTruncateLogInfoInstanceA(instance: JET_INSTANCE, szzlogs: Option<*mut JET_CHAR>, cbmax: JET_UINT32, pcbactual: Option<*mut JET_UINT32>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetTruncateLogInfoInstanceA(instance : JET_INSTANCE, szzlogs : *mut JET_CHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetGetTruncateLogInfoInstanceA(instance, szzlogs.unwrap_or(core::mem::zeroed()) as _, cbmax, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetGetTruncateLogInfoInstanceW(instance: JET_INSTANCE, wszzlogs: Option<*mut JET_WCHAR>, cbmax: JET_UINT32, pcbactual: Option<*mut JET_UINT32>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetTruncateLogInfoInstanceW(instance : JET_INSTANCE, wszzlogs : *mut JET_WCHAR, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetGetTruncateLogInfoInstanceW(instance, wszzlogs.unwrap_or(core::mem::zeroed()) as _, cbmax, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetGetVersion(sesid: JET_SESID, pwversion: *mut JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGetVersion(sesid : JET_SESID, pwversion : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetGetVersion(sesid, pwversion as _) }
}
#[inline]
pub unsafe fn JetGotoBookmark(sesid: JET_SESID, tableid: JET_TABLEID, pvbookmark: JET_PVOID, cbbookmark: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGotoBookmark(sesid : JET_SESID, tableid : JET_TABLEID, pvbookmark : JET_PVOID, cbbookmark : JET_UINT32) -> JET_ERR);
    unsafe { JetGotoBookmark(sesid, tableid, pvbookmark, cbbookmark) }
}
#[inline]
pub unsafe fn JetGotoPosition(sesid: JET_SESID, tableid: JET_TABLEID, precpos: *const JET_RECPOS) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGotoPosition(sesid : JET_SESID, tableid : JET_TABLEID, precpos : *const JET_RECPOS) -> JET_ERR);
    unsafe { JetGotoPosition(sesid, tableid, precpos) }
}
#[inline]
pub unsafe fn JetGotoSecondaryIndexBookmark(sesid: JET_SESID, tableid: JET_TABLEID, pvsecondarykey: JET_PVOID, cbsecondarykey: JET_UINT32, pvprimarybookmark: Option<JET_PVOID>, cbprimarybookmark: JET_UINT32, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGotoSecondaryIndexBookmark(sesid : JET_SESID, tableid : JET_TABLEID, pvsecondarykey : JET_PVOID, cbsecondarykey : JET_UINT32, pvprimarybookmark : JET_PVOID, cbprimarybookmark : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetGotoSecondaryIndexBookmark(sesid, tableid, pvsecondarykey, cbsecondarykey, pvprimarybookmark.unwrap_or(core::mem::zeroed()) as _, cbprimarybookmark, grbit) }
}
#[inline]
pub unsafe fn JetGrowDatabase(sesid: JET_SESID, dbid: JET_DBID, cpg: JET_UINT32, pcpgreal: *const JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetGrowDatabase(sesid : JET_SESID, dbid : JET_DBID, cpg : JET_UINT32, pcpgreal : *const JET_UINT32) -> JET_ERR);
    unsafe { JetGrowDatabase(sesid, dbid, cpg, pcpgreal) }
}
#[inline]
pub unsafe fn JetIdle(sesid: JET_SESID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetIdle(sesid : JET_SESID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetIdle(sesid, grbit) }
}
#[inline]
pub unsafe fn JetIndexRecordCount(sesid: JET_SESID, tableid: JET_TABLEID, pcrec: *mut JET_UINT32, crecmax: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetIndexRecordCount(sesid : JET_SESID, tableid : JET_TABLEID, pcrec : *mut JET_UINT32, crecmax : JET_UINT32) -> JET_ERR);
    unsafe { JetIndexRecordCount(sesid, tableid, pcrec as _, crecmax) }
}
#[inline]
pub unsafe fn JetInit(pinstance: Option<*mut JET_INSTANCE>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetInit(pinstance : *mut JET_INSTANCE) -> JET_ERR);
    unsafe { JetInit(pinstance.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetInit2(pinstance: Option<*mut JET_INSTANCE>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetInit2(pinstance : *mut JET_INSTANCE, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetInit2(pinstance.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[inline]
pub unsafe fn JetInit3A(pinstance: Option<*mut JET_INSTANCE>, prstinfo: Option<*const JET_RSTINFO_A>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetInit3A(pinstance : *mut JET_INSTANCE, prstinfo : *const JET_RSTINFO_A, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetInit3A(pinstance.unwrap_or(core::mem::zeroed()) as _, prstinfo.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[inline]
pub unsafe fn JetInit3W(pinstance: Option<*mut JET_INSTANCE>, prstinfo: Option<*const JET_RSTINFO_W>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetInit3W(pinstance : *mut JET_INSTANCE, prstinfo : *const JET_RSTINFO_W, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetInit3W(pinstance.unwrap_or(core::mem::zeroed()) as _, prstinfo.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[inline]
pub unsafe fn JetIntersectIndexes(sesid: JET_SESID, rgindexrange: *const JET_INDEXRANGE, cindexrange: JET_UINT32, precordlist: *mut JET_RECORDLIST, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetIntersectIndexes(sesid : JET_SESID, rgindexrange : *const JET_INDEXRANGE, cindexrange : JET_UINT32, precordlist : *mut JET_RECORDLIST, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetIntersectIndexes(sesid, rgindexrange, cindexrange, precordlist as _, grbit) }
}
#[inline]
pub unsafe fn JetMakeKey(sesid: JET_SESID, tableid: JET_TABLEID, pvdata: Option<JET_PCVOID>, cbdata: JET_UINT32, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetMakeKey(sesid : JET_SESID, tableid : JET_TABLEID, pvdata : JET_PCVOID, cbdata : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetMakeKey(sesid, tableid, pvdata.unwrap_or(core::mem::zeroed()) as _, cbdata, grbit) }
}
#[inline]
pub unsafe fn JetMove(sesid: JET_SESID, tableid: JET_TABLEID, crow: JET_INT32, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetMove(sesid : JET_SESID, tableid : JET_TABLEID, crow : JET_INT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetMove(sesid, tableid, crow, grbit) }
}
#[inline]
pub unsafe fn JetOSSnapshotAbort(snapid: JET_OSSNAPID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOSSnapshotAbort(snapid : JET_OSSNAPID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetOSSnapshotAbort(snapid, grbit) }
}
#[inline]
pub unsafe fn JetOSSnapshotEnd(snapid: JET_OSSNAPID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOSSnapshotEnd(snapid : JET_OSSNAPID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetOSSnapshotEnd(snapid, grbit) }
}
#[inline]
pub unsafe fn JetOSSnapshotFreezeA(snapid: JET_OSSNAPID, pcinstanceinfo: *mut JET_UINT32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_A, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOSSnapshotFreezeA(snapid : JET_OSSNAPID, pcinstanceinfo : *mut JET_UINT32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_A, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetOSSnapshotFreezeA(snapid, pcinstanceinfo as _, painstanceinfo as _, grbit) }
}
#[inline]
pub unsafe fn JetOSSnapshotFreezeW(snapid: JET_OSSNAPID, pcinstanceinfo: *mut JET_UINT32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_W, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOSSnapshotFreezeW(snapid : JET_OSSNAPID, pcinstanceinfo : *mut JET_UINT32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_W, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetOSSnapshotFreezeW(snapid, pcinstanceinfo as _, painstanceinfo as _, grbit) }
}
#[inline]
pub unsafe fn JetOSSnapshotGetFreezeInfoA(snapid: JET_OSSNAPID, pcinstanceinfo: *mut JET_UINT32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_A, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOSSnapshotGetFreezeInfoA(snapid : JET_OSSNAPID, pcinstanceinfo : *mut JET_UINT32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_A, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetOSSnapshotGetFreezeInfoA(snapid, pcinstanceinfo as _, painstanceinfo as _, grbit) }
}
#[inline]
pub unsafe fn JetOSSnapshotGetFreezeInfoW(snapid: JET_OSSNAPID, pcinstanceinfo: *mut JET_UINT32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_W, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOSSnapshotGetFreezeInfoW(snapid : JET_OSSNAPID, pcinstanceinfo : *mut JET_UINT32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_W, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetOSSnapshotGetFreezeInfoW(snapid, pcinstanceinfo as _, painstanceinfo as _, grbit) }
}
#[inline]
pub unsafe fn JetOSSnapshotPrepare(psnapid: *mut JET_OSSNAPID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOSSnapshotPrepare(psnapid : *mut JET_OSSNAPID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetOSSnapshotPrepare(psnapid as _, grbit) }
}
#[inline]
pub unsafe fn JetOSSnapshotPrepareInstance(snapid: JET_OSSNAPID, instance: JET_INSTANCE, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOSSnapshotPrepareInstance(snapid : JET_OSSNAPID, instance : JET_INSTANCE, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetOSSnapshotPrepareInstance(snapid, instance, grbit) }
}
#[inline]
pub unsafe fn JetOSSnapshotThaw(snapid: JET_OSSNAPID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOSSnapshotThaw(snapid : JET_OSSNAPID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetOSSnapshotThaw(snapid, grbit) }
}
#[inline]
pub unsafe fn JetOSSnapshotTruncateLog(snapid: JET_OSSNAPID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOSSnapshotTruncateLog(snapid : JET_OSSNAPID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetOSSnapshotTruncateLog(snapid, grbit) }
}
#[inline]
pub unsafe fn JetOSSnapshotTruncateLogInstance(snapid: JET_OSSNAPID, instance: JET_INSTANCE, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOSSnapshotTruncateLogInstance(snapid : JET_OSSNAPID, instance : JET_INSTANCE, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetOSSnapshotTruncateLogInstance(snapid, instance, grbit) }
}
#[inline]
pub unsafe fn JetOpenDatabaseA(sesid: JET_SESID, szfilename: *const JET_CHAR, szconnect: Option<*const JET_CHAR>, pdbid: *mut JET_DBID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOpenDatabaseA(sesid : JET_SESID, szfilename : *const JET_CHAR, szconnect : *const JET_CHAR, pdbid : *mut JET_DBID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetOpenDatabaseA(sesid, szfilename, szconnect.unwrap_or(core::mem::zeroed()) as _, pdbid as _, grbit) }
}
#[inline]
pub unsafe fn JetOpenDatabaseW(sesid: JET_SESID, szfilename: *const JET_WCHAR, szconnect: Option<*const JET_WCHAR>, pdbid: *mut JET_DBID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOpenDatabaseW(sesid : JET_SESID, szfilename : *const JET_WCHAR, szconnect : *const JET_WCHAR, pdbid : *mut JET_DBID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetOpenDatabaseW(sesid, szfilename, szconnect.unwrap_or(core::mem::zeroed()) as _, pdbid as _, grbit) }
}
#[inline]
pub unsafe fn JetOpenFileA(szfilename: *const JET_CHAR, phffile: *mut JET_HANDLE, pulfilesizelow: *mut JET_UINT32, pulfilesizehigh: *mut JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOpenFileA(szfilename : *const JET_CHAR, phffile : *mut JET_HANDLE, pulfilesizelow : *mut JET_UINT32, pulfilesizehigh : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetOpenFileA(szfilename, phffile as _, pulfilesizelow as _, pulfilesizehigh as _) }
}
#[inline]
pub unsafe fn JetOpenFileInstanceA(instance: JET_INSTANCE, szfilename: *const JET_CHAR, phffile: *mut JET_HANDLE, pulfilesizelow: *mut JET_UINT32, pulfilesizehigh: *mut JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOpenFileInstanceA(instance : JET_INSTANCE, szfilename : *const JET_CHAR, phffile : *mut JET_HANDLE, pulfilesizelow : *mut JET_UINT32, pulfilesizehigh : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetOpenFileInstanceA(instance, szfilename, phffile as _, pulfilesizelow as _, pulfilesizehigh as _) }
}
#[inline]
pub unsafe fn JetOpenFileInstanceW(instance: JET_INSTANCE, szfilename: *const JET_WCHAR, phffile: *mut JET_HANDLE, pulfilesizelow: *mut JET_UINT32, pulfilesizehigh: *mut JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOpenFileInstanceW(instance : JET_INSTANCE, szfilename : *const JET_WCHAR, phffile : *mut JET_HANDLE, pulfilesizelow : *mut JET_UINT32, pulfilesizehigh : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetOpenFileInstanceW(instance, szfilename, phffile as _, pulfilesizelow as _, pulfilesizehigh as _) }
}
#[inline]
pub unsafe fn JetOpenFileW(szfilename: *const JET_WCHAR, phffile: *mut JET_HANDLE, pulfilesizelow: *mut JET_UINT32, pulfilesizehigh: *mut JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOpenFileW(szfilename : *const JET_WCHAR, phffile : *mut JET_HANDLE, pulfilesizelow : *mut JET_UINT32, pulfilesizehigh : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetOpenFileW(szfilename, phffile as _, pulfilesizelow as _, pulfilesizehigh as _) }
}
#[inline]
pub unsafe fn JetOpenTableA(sesid: JET_SESID, dbid: JET_DBID, sztablename: *const JET_CHAR, pvparameters: Option<JET_PCVOID>, cbparameters: JET_UINT32, grbit: JET_GRBIT, ptableid: *mut JET_TABLEID) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOpenTableA(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_CHAR, pvparameters : JET_PCVOID, cbparameters : JET_UINT32, grbit : JET_GRBIT, ptableid : *mut JET_TABLEID) -> JET_ERR);
    unsafe { JetOpenTableA(sesid, dbid, sztablename, pvparameters.unwrap_or(core::mem::zeroed()) as _, cbparameters, grbit, ptableid as _) }
}
#[inline]
pub unsafe fn JetOpenTableW(sesid: JET_SESID, dbid: JET_DBID, sztablename: *const JET_WCHAR, pvparameters: Option<JET_PCVOID>, cbparameters: JET_UINT32, grbit: JET_GRBIT, ptableid: *mut JET_TABLEID) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOpenTableW(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_WCHAR, pvparameters : JET_PCVOID, cbparameters : JET_UINT32, grbit : JET_GRBIT, ptableid : *mut JET_TABLEID) -> JET_ERR);
    unsafe { JetOpenTableW(sesid, dbid, sztablename, pvparameters.unwrap_or(core::mem::zeroed()) as _, cbparameters, grbit, ptableid as _) }
}
#[inline]
pub unsafe fn JetOpenTempTable(sesid: JET_SESID, prgcolumndef: *const JET_COLUMNDEF, ccolumn: JET_UINT32, grbit: JET_GRBIT, ptableid: *mut JET_TABLEID, prgcolumnid: *mut JET_COLUMNID) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOpenTempTable(sesid : JET_SESID, prgcolumndef : *const JET_COLUMNDEF, ccolumn : JET_UINT32, grbit : JET_GRBIT, ptableid : *mut JET_TABLEID, prgcolumnid : *mut JET_COLUMNID) -> JET_ERR);
    unsafe { JetOpenTempTable(sesid, prgcolumndef, ccolumn, grbit, ptableid as _, prgcolumnid as _) }
}
#[inline]
pub unsafe fn JetOpenTempTable2(sesid: JET_SESID, prgcolumndef: *const JET_COLUMNDEF, ccolumn: JET_UINT32, lcid: JET_LCID, grbit: JET_GRBIT, ptableid: *mut JET_TABLEID, prgcolumnid: *mut JET_COLUMNID) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOpenTempTable2(sesid : JET_SESID, prgcolumndef : *const JET_COLUMNDEF, ccolumn : JET_UINT32, lcid : JET_LCID, grbit : JET_GRBIT, ptableid : *mut JET_TABLEID, prgcolumnid : *mut JET_COLUMNID) -> JET_ERR);
    unsafe { JetOpenTempTable2(sesid, prgcolumndef, ccolumn, lcid, grbit, ptableid as _, prgcolumnid as _) }
}
#[inline]
pub unsafe fn JetOpenTempTable3(sesid: JET_SESID, prgcolumndef: *const JET_COLUMNDEF, ccolumn: JET_UINT32, pidxunicode: Option<*const JET_UNICODEINDEX>, grbit: JET_GRBIT, ptableid: *mut JET_TABLEID, prgcolumnid: *mut JET_COLUMNID) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOpenTempTable3(sesid : JET_SESID, prgcolumndef : *const JET_COLUMNDEF, ccolumn : JET_UINT32, pidxunicode : *const JET_UNICODEINDEX, grbit : JET_GRBIT, ptableid : *mut JET_TABLEID, prgcolumnid : *mut JET_COLUMNID) -> JET_ERR);
    unsafe { JetOpenTempTable3(sesid, prgcolumndef, ccolumn, pidxunicode.unwrap_or(core::mem::zeroed()) as _, grbit, ptableid as _, prgcolumnid as _) }
}
#[inline]
pub unsafe fn JetOpenTemporaryTable(sesid: JET_SESID, popentemporarytable: *const JET_OPENTEMPORARYTABLE) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOpenTemporaryTable(sesid : JET_SESID, popentemporarytable : *const JET_OPENTEMPORARYTABLE) -> JET_ERR);
    unsafe { JetOpenTemporaryTable(sesid, popentemporarytable) }
}
#[inline]
pub unsafe fn JetOpenTemporaryTable2(sesid: JET_SESID, popentemporarytable: *const JET_OPENTEMPORARYTABLE2) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetOpenTemporaryTable2(sesid : JET_SESID, popentemporarytable : *const JET_OPENTEMPORARYTABLE2) -> JET_ERR);
    unsafe { JetOpenTemporaryTable2(sesid, popentemporarytable) }
}
#[inline]
pub unsafe fn JetPrepareUpdate(sesid: JET_SESID, tableid: JET_TABLEID, prep: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetPrepareUpdate(sesid : JET_SESID, tableid : JET_TABLEID, prep : JET_UINT32) -> JET_ERR);
    unsafe { JetPrepareUpdate(sesid, tableid, prep) }
}
#[inline]
pub unsafe fn JetPrereadIndexRanges(sesid: JET_SESID, tableid: JET_TABLEID, rgindexranges: *const JET_INDEX_RANGE, cindexranges: JET_UINT32, pcrangespreread: Option<*mut JET_UINT32>, rgcolumnidpreread: *const JET_COLUMNID, ccolumnidpreread: JET_UINT32, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetPrereadIndexRanges(sesid : JET_SESID, tableid : JET_TABLEID, rgindexranges : *const JET_INDEX_RANGE, cindexranges : JET_UINT32, pcrangespreread : *mut JET_UINT32, rgcolumnidpreread : *const JET_COLUMNID, ccolumnidpreread : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetPrereadIndexRanges(sesid, tableid, rgindexranges, cindexranges, pcrangespreread.unwrap_or(core::mem::zeroed()) as _, rgcolumnidpreread, ccolumnidpreread, grbit) }
}
#[inline]
pub unsafe fn JetPrereadKeys(sesid: JET_SESID, tableid: JET_TABLEID, rgpvkeys: *const JET_PCVOID, rgcbkeys: *const JET_UINT32, ckeys: JET_INT32, pckeyspreread: Option<*mut JET_INT32>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetPrereadKeys(sesid : JET_SESID, tableid : JET_TABLEID, rgpvkeys : *const JET_PCVOID, rgcbkeys : *const JET_UINT32, ckeys : JET_INT32, pckeyspreread : *mut JET_INT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetPrereadKeys(sesid, tableid, rgpvkeys, rgcbkeys, ckeys, pckeyspreread.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[inline]
pub unsafe fn JetReadFile(hffile: JET_HANDLE, pv: JET_PVOID, cb: JET_UINT32, pcbactual: Option<*mut JET_UINT32>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetReadFile(hffile : JET_HANDLE, pv : JET_PVOID, cb : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetReadFile(hffile, pv as _, cb, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetReadFileInstance(instance: JET_INSTANCE, hffile: JET_HANDLE, pv: JET_PVOID, cb: JET_UINT32, pcbactual: Option<*mut JET_UINT32>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetReadFileInstance(instance : JET_INSTANCE, hffile : JET_HANDLE, pv : JET_PVOID, cb : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetReadFileInstance(instance, hffile, pv as _, cb, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetRegisterCallback(sesid: JET_SESID, tableid: JET_TABLEID, cbtyp: JET_CBTYP, pcallback: JET_CALLBACK, pvcontext: Option<JET_PVOID>, phcallbackid: *const JET_HANDLE) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetRegisterCallback(sesid : JET_SESID, tableid : JET_TABLEID, cbtyp : JET_CBTYP, pcallback : JET_CALLBACK, pvcontext : JET_PVOID, phcallbackid : *const JET_HANDLE) -> JET_ERR);
    unsafe { JetRegisterCallback(sesid, tableid, cbtyp, pcallback, pvcontext.unwrap_or(core::mem::zeroed()) as _, phcallbackid) }
}
#[inline]
pub unsafe fn JetRenameColumnA(sesid: JET_SESID, tableid: JET_TABLEID, szname: *const JET_CHAR, sznamenew: *const JET_CHAR, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetRenameColumnA(sesid : JET_SESID, tableid : JET_TABLEID, szname : *const JET_CHAR, sznamenew : *const JET_CHAR, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetRenameColumnA(sesid, tableid, szname, sznamenew, grbit) }
}
#[inline]
pub unsafe fn JetRenameColumnW(sesid: JET_SESID, tableid: JET_TABLEID, szname: *const JET_WCHAR, sznamenew: *const JET_WCHAR, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetRenameColumnW(sesid : JET_SESID, tableid : JET_TABLEID, szname : *const JET_WCHAR, sznamenew : *const JET_WCHAR, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetRenameColumnW(sesid, tableid, szname, sznamenew, grbit) }
}
#[inline]
pub unsafe fn JetRenameTableA(sesid: JET_SESID, dbid: JET_DBID, szname: *const JET_CHAR, sznamenew: *const JET_CHAR) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetRenameTableA(sesid : JET_SESID, dbid : JET_DBID, szname : *const JET_CHAR, sznamenew : *const JET_CHAR) -> JET_ERR);
    unsafe { JetRenameTableA(sesid, dbid, szname, sznamenew) }
}
#[inline]
pub unsafe fn JetRenameTableW(sesid: JET_SESID, dbid: JET_DBID, szname: *const JET_WCHAR, sznamenew: *const JET_WCHAR) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetRenameTableW(sesid : JET_SESID, dbid : JET_DBID, szname : *const JET_WCHAR, sznamenew : *const JET_WCHAR) -> JET_ERR);
    unsafe { JetRenameTableW(sesid, dbid, szname, sznamenew) }
}
#[inline]
pub unsafe fn JetResetSessionContext(sesid: JET_SESID) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetResetSessionContext(sesid : JET_SESID) -> JET_ERR);
    unsafe { JetResetSessionContext(sesid) }
}
#[inline]
pub unsafe fn JetResetTableSequential(sesid: JET_SESID, tableid: JET_TABLEID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetResetTableSequential(sesid : JET_SESID, tableid : JET_TABLEID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetResetTableSequential(sesid, tableid, grbit) }
}
#[inline]
pub unsafe fn JetResizeDatabase(sesid: JET_SESID, dbid: JET_DBID, cpgtarget: JET_UINT32, pcpgactual: *mut JET_UINT32, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetResizeDatabase(sesid : JET_SESID, dbid : JET_DBID, cpgtarget : JET_UINT32, pcpgactual : *mut JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetResizeDatabase(sesid, dbid, cpgtarget, pcpgactual as _, grbit) }
}
#[inline]
pub unsafe fn JetRestore2A(sz: *const JET_CHAR, szdest: Option<*const JET_CHAR>, pfn: JET_PFNSTATUS) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetRestore2A(sz : *const JET_CHAR, szdest : *const JET_CHAR, pfn : JET_PFNSTATUS) -> JET_ERR);
    unsafe { JetRestore2A(sz, szdest.unwrap_or(core::mem::zeroed()) as _, pfn) }
}
#[inline]
pub unsafe fn JetRestore2W(sz: *const JET_WCHAR, szdest: Option<*const JET_WCHAR>, pfn: JET_PFNSTATUS) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetRestore2W(sz : *const JET_WCHAR, szdest : *const JET_WCHAR, pfn : JET_PFNSTATUS) -> JET_ERR);
    unsafe { JetRestore2W(sz, szdest.unwrap_or(core::mem::zeroed()) as _, pfn) }
}
#[inline]
pub unsafe fn JetRestoreA(szsource: *const JET_CHAR, pfn: JET_PFNSTATUS) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetRestoreA(szsource : *const JET_CHAR, pfn : JET_PFNSTATUS) -> JET_ERR);
    unsafe { JetRestoreA(szsource, pfn) }
}
#[inline]
pub unsafe fn JetRestoreInstanceA(instance: JET_INSTANCE, sz: *const JET_CHAR, szdest: Option<*const JET_CHAR>, pfn: JET_PFNSTATUS) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetRestoreInstanceA(instance : JET_INSTANCE, sz : *const JET_CHAR, szdest : *const JET_CHAR, pfn : JET_PFNSTATUS) -> JET_ERR);
    unsafe { JetRestoreInstanceA(instance, sz, szdest.unwrap_or(core::mem::zeroed()) as _, pfn) }
}
#[inline]
pub unsafe fn JetRestoreInstanceW(instance: JET_INSTANCE, sz: *const JET_WCHAR, szdest: Option<*const JET_WCHAR>, pfn: JET_PFNSTATUS) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetRestoreInstanceW(instance : JET_INSTANCE, sz : *const JET_WCHAR, szdest : *const JET_WCHAR, pfn : JET_PFNSTATUS) -> JET_ERR);
    unsafe { JetRestoreInstanceW(instance, sz, szdest.unwrap_or(core::mem::zeroed()) as _, pfn) }
}
#[inline]
pub unsafe fn JetRestoreW(szsource: *const JET_WCHAR, pfn: JET_PFNSTATUS) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetRestoreW(szsource : *const JET_WCHAR, pfn : JET_PFNSTATUS) -> JET_ERR);
    unsafe { JetRestoreW(szsource, pfn) }
}
#[inline]
pub unsafe fn JetRetrieveColumn(sesid: JET_SESID, tableid: JET_TABLEID, columnid: JET_COLUMNID, pvdata: Option<JET_PVOID>, cbdata: JET_UINT32, pcbactual: Option<*mut JET_UINT32>, grbit: JET_GRBIT, pretinfo: Option<*mut JET_RETINFO>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetRetrieveColumn(sesid : JET_SESID, tableid : JET_TABLEID, columnid : JET_COLUMNID, pvdata : JET_PVOID, cbdata : JET_UINT32, pcbactual : *mut JET_UINT32, grbit : JET_GRBIT, pretinfo : *mut JET_RETINFO) -> JET_ERR);
    unsafe { JetRetrieveColumn(sesid, tableid, columnid, pvdata.unwrap_or(core::mem::zeroed()) as _, cbdata, pcbactual.unwrap_or(core::mem::zeroed()) as _, grbit, pretinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetRetrieveColumns(sesid: JET_SESID, tableid: JET_TABLEID, pretrievecolumn: Option<*mut JET_RETRIEVECOLUMN>, cretrievecolumn: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetRetrieveColumns(sesid : JET_SESID, tableid : JET_TABLEID, pretrievecolumn : *mut JET_RETRIEVECOLUMN, cretrievecolumn : JET_UINT32) -> JET_ERR);
    unsafe { JetRetrieveColumns(sesid, tableid, pretrievecolumn.unwrap_or(core::mem::zeroed()) as _, cretrievecolumn) }
}
#[inline]
pub unsafe fn JetRetrieveKey(sesid: JET_SESID, tableid: JET_TABLEID, pvkey: Option<JET_PVOID>, cbmax: JET_UINT32, pcbactual: Option<*mut JET_UINT32>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetRetrieveKey(sesid : JET_SESID, tableid : JET_TABLEID, pvkey : JET_PVOID, cbmax : JET_UINT32, pcbactual : *mut JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetRetrieveKey(sesid, tableid, pvkey.unwrap_or(core::mem::zeroed()) as _, cbmax, pcbactual.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[inline]
pub unsafe fn JetRollback(sesid: JET_SESID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetRollback(sesid : JET_SESID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetRollback(sesid, grbit) }
}
#[inline]
pub unsafe fn JetSeek(sesid: JET_SESID, tableid: JET_TABLEID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSeek(sesid : JET_SESID, tableid : JET_TABLEID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetSeek(sesid, tableid, grbit) }
}
#[inline]
pub unsafe fn JetSetColumn(sesid: JET_SESID, tableid: JET_TABLEID, columnid: JET_COLUMNID, pvdata: Option<JET_PCVOID>, cbdata: JET_UINT32, grbit: JET_GRBIT, psetinfo: Option<*const JET_SETINFO>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetColumn(sesid : JET_SESID, tableid : JET_TABLEID, columnid : JET_COLUMNID, pvdata : JET_PCVOID, cbdata : JET_UINT32, grbit : JET_GRBIT, psetinfo : *const JET_SETINFO) -> JET_ERR);
    unsafe { JetSetColumn(sesid, tableid, columnid, pvdata.unwrap_or(core::mem::zeroed()) as _, cbdata, grbit, psetinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetSetColumnDefaultValueA(sesid: JET_SESID, dbid: JET_DBID, sztablename: *const JET_CHAR, szcolumnname: *const JET_CHAR, pvdata: JET_PCVOID, cbdata: JET_UINT32, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetColumnDefaultValueA(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_CHAR, szcolumnname : *const JET_CHAR, pvdata : JET_PCVOID, cbdata : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetSetColumnDefaultValueA(sesid, dbid, sztablename, szcolumnname, pvdata, cbdata, grbit) }
}
#[inline]
pub unsafe fn JetSetColumnDefaultValueW(sesid: JET_SESID, dbid: JET_DBID, sztablename: *const JET_WCHAR, szcolumnname: *const JET_WCHAR, pvdata: JET_PCVOID, cbdata: JET_UINT32, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetColumnDefaultValueW(sesid : JET_SESID, dbid : JET_DBID, sztablename : *const JET_WCHAR, szcolumnname : *const JET_WCHAR, pvdata : JET_PCVOID, cbdata : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetSetColumnDefaultValueW(sesid, dbid, sztablename, szcolumnname, pvdata, cbdata, grbit) }
}
#[inline]
pub unsafe fn JetSetColumns(sesid: JET_SESID, tableid: JET_TABLEID, psetcolumn: Option<*const JET_SETCOLUMN>, csetcolumn: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetColumns(sesid : JET_SESID, tableid : JET_TABLEID, psetcolumn : *const JET_SETCOLUMN, csetcolumn : JET_UINT32) -> JET_ERR);
    unsafe { JetSetColumns(sesid, tableid, psetcolumn.unwrap_or(core::mem::zeroed()) as _, csetcolumn) }
}
#[inline]
pub unsafe fn JetSetCurrentIndex2A(sesid: JET_SESID, tableid: JET_TABLEID, szindexname: Option<*const JET_CHAR>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetCurrentIndex2A(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_CHAR, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetSetCurrentIndex2A(sesid, tableid, szindexname.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[inline]
pub unsafe fn JetSetCurrentIndex2W(sesid: JET_SESID, tableid: JET_TABLEID, szindexname: Option<*const JET_WCHAR>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetCurrentIndex2W(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_WCHAR, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetSetCurrentIndex2W(sesid, tableid, szindexname.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[inline]
pub unsafe fn JetSetCurrentIndex3A(sesid: JET_SESID, tableid: JET_TABLEID, szindexname: Option<*const JET_CHAR>, grbit: JET_GRBIT, itagsequence: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetCurrentIndex3A(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_CHAR, grbit : JET_GRBIT, itagsequence : JET_UINT32) -> JET_ERR);
    unsafe { JetSetCurrentIndex3A(sesid, tableid, szindexname.unwrap_or(core::mem::zeroed()) as _, grbit, itagsequence) }
}
#[inline]
pub unsafe fn JetSetCurrentIndex3W(sesid: JET_SESID, tableid: JET_TABLEID, szindexname: Option<*const JET_WCHAR>, grbit: JET_GRBIT, itagsequence: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetCurrentIndex3W(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_WCHAR, grbit : JET_GRBIT, itagsequence : JET_UINT32) -> JET_ERR);
    unsafe { JetSetCurrentIndex3W(sesid, tableid, szindexname.unwrap_or(core::mem::zeroed()) as _, grbit, itagsequence) }
}
#[inline]
pub unsafe fn JetSetCurrentIndex4A(sesid: JET_SESID, tableid: JET_TABLEID, szindexname: Option<*const JET_CHAR>, pindexid: Option<*const JET_INDEXID>, grbit: JET_GRBIT, itagsequence: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetCurrentIndex4A(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_CHAR, pindexid : *const JET_INDEXID, grbit : JET_GRBIT, itagsequence : JET_UINT32) -> JET_ERR);
    unsafe { JetSetCurrentIndex4A(sesid, tableid, szindexname.unwrap_or(core::mem::zeroed()) as _, pindexid.unwrap_or(core::mem::zeroed()) as _, grbit, itagsequence) }
}
#[inline]
pub unsafe fn JetSetCurrentIndex4W(sesid: JET_SESID, tableid: JET_TABLEID, szindexname: Option<*const JET_WCHAR>, pindexid: Option<*const JET_INDEXID>, grbit: JET_GRBIT, itagsequence: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetCurrentIndex4W(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_WCHAR, pindexid : *const JET_INDEXID, grbit : JET_GRBIT, itagsequence : JET_UINT32) -> JET_ERR);
    unsafe { JetSetCurrentIndex4W(sesid, tableid, szindexname.unwrap_or(core::mem::zeroed()) as _, pindexid.unwrap_or(core::mem::zeroed()) as _, grbit, itagsequence) }
}
#[inline]
pub unsafe fn JetSetCurrentIndexA(sesid: JET_SESID, tableid: JET_TABLEID, szindexname: Option<*const JET_CHAR>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetCurrentIndexA(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_CHAR) -> JET_ERR);
    unsafe { JetSetCurrentIndexA(sesid, tableid, szindexname.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetSetCurrentIndexW(sesid: JET_SESID, tableid: JET_TABLEID, szindexname: Option<*const JET_WCHAR>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetCurrentIndexW(sesid : JET_SESID, tableid : JET_TABLEID, szindexname : *const JET_WCHAR) -> JET_ERR);
    unsafe { JetSetCurrentIndexW(sesid, tableid, szindexname.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetSetCursorFilter(sesid: JET_SESID, tableid: JET_TABLEID, rgcolumnfilters: *const JET_INDEX_COLUMN, ccolumnfilters: JET_UINT32, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetCursorFilter(sesid : JET_SESID, tableid : JET_TABLEID, rgcolumnfilters : *const JET_INDEX_COLUMN, ccolumnfilters : JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetSetCursorFilter(sesid, tableid, rgcolumnfilters, ccolumnfilters, grbit) }
}
#[inline]
pub unsafe fn JetSetDatabaseSizeA(sesid: JET_SESID, szdatabasename: *const JET_CHAR, cpg: JET_UINT32, pcpgreal: *mut JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetDatabaseSizeA(sesid : JET_SESID, szdatabasename : *const JET_CHAR, cpg : JET_UINT32, pcpgreal : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetSetDatabaseSizeA(sesid, szdatabasename, cpg, pcpgreal as _) }
}
#[inline]
pub unsafe fn JetSetDatabaseSizeW(sesid: JET_SESID, szdatabasename: *const JET_WCHAR, cpg: JET_UINT32, pcpgreal: *mut JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetDatabaseSizeW(sesid : JET_SESID, szdatabasename : *const JET_WCHAR, cpg : JET_UINT32, pcpgreal : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetSetDatabaseSizeW(sesid, szdatabasename, cpg, pcpgreal as _) }
}
#[inline]
pub unsafe fn JetSetIndexRange(sesid: JET_SESID, tableidsrc: JET_TABLEID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetIndexRange(sesid : JET_SESID, tableidsrc : JET_TABLEID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetSetIndexRange(sesid, tableidsrc, grbit) }
}
#[inline]
pub unsafe fn JetSetLS(sesid: JET_SESID, tableid: JET_TABLEID, ls: JET_LS, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetLS(sesid : JET_SESID, tableid : JET_TABLEID, ls : JET_LS, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetSetLS(sesid, tableid, ls, grbit) }
}
#[inline]
pub unsafe fn JetSetSessionContext(sesid: JET_SESID, ulcontext: JET_API_PTR) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetSessionContext(sesid : JET_SESID, ulcontext : JET_API_PTR) -> JET_ERR);
    unsafe { JetSetSessionContext(sesid, ulcontext) }
}
#[inline]
pub unsafe fn JetSetSessionParameter(sesid: Option<JET_SESID>, sesparamid: JET_UINT32, pvparam: Option<JET_PVOID>, cbparam: JET_UINT32) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetSessionParameter(sesid : JET_SESID, sesparamid : JET_UINT32, pvparam : JET_PVOID, cbparam : JET_UINT32) -> JET_ERR);
    unsafe { JetSetSessionParameter(sesid.unwrap_or(core::mem::zeroed()) as _, sesparamid, pvparam.unwrap_or(core::mem::zeroed()) as _, cbparam) }
}
#[inline]
pub unsafe fn JetSetSystemParameterA(pinstance: Option<*mut JET_INSTANCE>, sesid: Option<JET_SESID>, paramid: JET_UINT32, lparam: Option<JET_API_PTR>, szparam: Option<*const JET_CHAR>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetSystemParameterA(pinstance : *mut JET_INSTANCE, sesid : JET_SESID, paramid : JET_UINT32, lparam : JET_API_PTR, szparam : *const JET_CHAR) -> JET_ERR);
    unsafe { JetSetSystemParameterA(pinstance.unwrap_or(core::mem::zeroed()) as _, sesid.unwrap_or(core::mem::zeroed()) as _, paramid, lparam.unwrap_or(core::mem::zeroed()) as _, szparam.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetSetSystemParameterW(pinstance: Option<*mut JET_INSTANCE>, sesid: Option<JET_SESID>, paramid: JET_UINT32, lparam: Option<JET_API_PTR>, szparam: Option<*const JET_WCHAR>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetSystemParameterW(pinstance : *mut JET_INSTANCE, sesid : JET_SESID, paramid : JET_UINT32, lparam : JET_API_PTR, szparam : *const JET_WCHAR) -> JET_ERR);
    unsafe { JetSetSystemParameterW(pinstance.unwrap_or(core::mem::zeroed()) as _, sesid.unwrap_or(core::mem::zeroed()) as _, paramid, lparam.unwrap_or(core::mem::zeroed()) as _, szparam.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetSetTableSequential(sesid: JET_SESID, tableid: JET_TABLEID, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetSetTableSequential(sesid : JET_SESID, tableid : JET_TABLEID, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetSetTableSequential(sesid, tableid, grbit) }
}
#[inline]
pub unsafe fn JetStopBackup() -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetStopBackup() -> JET_ERR);
    unsafe { JetStopBackup() }
}
#[inline]
pub unsafe fn JetStopBackupInstance(instance: JET_INSTANCE) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetStopBackupInstance(instance : JET_INSTANCE) -> JET_ERR);
    unsafe { JetStopBackupInstance(instance) }
}
#[inline]
pub unsafe fn JetStopService() -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetStopService() -> JET_ERR);
    unsafe { JetStopService() }
}
#[inline]
pub unsafe fn JetStopServiceInstance(instance: JET_INSTANCE) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetStopServiceInstance(instance : JET_INSTANCE) -> JET_ERR);
    unsafe { JetStopServiceInstance(instance) }
}
#[inline]
pub unsafe fn JetStopServiceInstance2(instance: JET_INSTANCE, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetStopServiceInstance2(instance : JET_INSTANCE, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetStopServiceInstance2(instance, grbit) }
}
#[inline]
pub unsafe fn JetTerm(instance: JET_INSTANCE) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetTerm(instance : JET_INSTANCE) -> JET_ERR);
    unsafe { JetTerm(instance) }
}
#[inline]
pub unsafe fn JetTerm2(instance: JET_INSTANCE, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetTerm2(instance : JET_INSTANCE, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetTerm2(instance, grbit) }
}
#[inline]
pub unsafe fn JetTruncateLog() -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetTruncateLog() -> JET_ERR);
    unsafe { JetTruncateLog() }
}
#[inline]
pub unsafe fn JetTruncateLogInstance(instance: JET_INSTANCE) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetTruncateLogInstance(instance : JET_INSTANCE) -> JET_ERR);
    unsafe { JetTruncateLogInstance(instance) }
}
#[inline]
pub unsafe fn JetUnregisterCallback(sesid: JET_SESID, tableid: JET_TABLEID, cbtyp: JET_CBTYP, hcallbackid: JET_HANDLE) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetUnregisterCallback(sesid : JET_SESID, tableid : JET_TABLEID, cbtyp : JET_CBTYP, hcallbackid : JET_HANDLE) -> JET_ERR);
    unsafe { JetUnregisterCallback(sesid, tableid, cbtyp, hcallbackid) }
}
#[inline]
pub unsafe fn JetUpdate(sesid: JET_SESID, tableid: JET_TABLEID, pvbookmark: Option<JET_PVOID>, cbbookmark: JET_UINT32, pcbactual: Option<*mut JET_UINT32>) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetUpdate(sesid : JET_SESID, tableid : JET_TABLEID, pvbookmark : JET_PVOID, cbbookmark : JET_UINT32, pcbactual : *mut JET_UINT32) -> JET_ERR);
    unsafe { JetUpdate(sesid, tableid, pvbookmark.unwrap_or(core::mem::zeroed()) as _, cbbookmark, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn JetUpdate2(sesid: JET_SESID, tableid: JET_TABLEID, pvbookmark: Option<JET_PVOID>, cbbookmark: JET_UINT32, pcbactual: Option<*mut JET_UINT32>, grbit: JET_GRBIT) -> JET_ERR {
    windows_core::link!("esent.dll" "system" fn JetUpdate2(sesid : JET_SESID, tableid : JET_TABLEID, pvbookmark : JET_PVOID, cbbookmark : JET_UINT32, pcbactual : *mut JET_UINT32, grbit : JET_GRBIT) -> JET_ERR);
    unsafe { JetUpdate2(sesid, tableid, pvbookmark.unwrap_or(core::mem::zeroed()) as _, cbbookmark, pcbactual.unwrap_or(core::mem::zeroed()) as _, grbit) }
}
#[cfg(target_arch = "x86")]
pub type JET_API_PTR = JET_UINT32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type JET_API_PTR = JET_UINT64;
pub const JET_BASE_NAME_LENGTH: i32 = 3;
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_BKLOGTIME_1_0 {
    pub _bitfield: JET_BYTE,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct JET_BYTE(pub u8);
pub type JET_CALLBACK = Option<unsafe extern "system" fn(sesid: JET_SESID, dbid: JET_DBID, tableid: JET_TABLEID, cbtyp: JET_CBTYP, pvarg1: JET_PVOID, pvarg2: JET_PVOID, pvcontext: JET_PVOID, ulunused: JET_API_PTR) -> JET_ERR>;
pub type JET_CBTYP = JET_UINT32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct JET_CHAR(pub i8);
pub type JET_COLTYP = JET_UINT32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_CONDITIONALCOLUMN_A {
    pub cbStruct: JET_UINT32,
    pub szColumnName: JET_PSTR,
    pub grbit: JET_GRBIT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_CONDITIONALCOLUMN_W {
    pub cbStruct: JET_UINT32,
    pub szColumnName: JET_PWSTR,
    pub grbit: JET_GRBIT,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_CONVERT_W_0_0 {
    pub _bitfield: JET_UINT32,
}
pub type JET_CP = JET_UINT16;
pub const JET_ColInfo: u32 = 0;
pub const JET_ColInfoBase: u32 = 4;
pub const JET_ColInfoBaseByColid: u32 = 8;
pub const JET_ColInfoByColid: u32 = 6;
pub const JET_ColInfoGrbitMinimalInfo: i32 = 1073741824;
pub const JET_ColInfoGrbitNonDerivedColumnsOnly: u32 = 2147483648;
pub const JET_ColInfoGrbitSortByColumnid: i32 = 536870912;
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_DBINFOUPGRADE_0_0 {
    pub _bitfield: JET_UINT32,
}
pub const JET_DbInfoCollate: i32 = 5;
pub const JET_DbInfoConnect: i32 = 1;
pub const JET_DbInfoCountry: i32 = 2;
pub const JET_DbInfoCp: i32 = 4;
pub const JET_DbInfoDBInUse: i32 = 15;
pub const JET_DbInfoFileType: i32 = 19;
pub const JET_DbInfoFilename: i32 = 0;
pub const JET_DbInfoFilesize: i32 = 10;
pub const JET_DbInfoFilesizeOnDisk: i32 = 21;
pub const JET_DbInfoIsam: i32 = 9;
pub const JET_DbInfoLCID: i32 = 3;
pub const JET_DbInfoLangid: i32 = 3;
pub const JET_DbInfoMisc: i32 = 14;
pub const JET_DbInfoOptions: i32 = 6;
pub const JET_DbInfoPageSize: i32 = 17;
pub const JET_DbInfoSpaceAvailable: i32 = 12;
pub const JET_DbInfoSpaceOwned: i32 = 11;
pub const JET_DbInfoTransactions: i32 = 7;
pub const JET_DbInfoUpgrade: i32 = 13;
pub const JET_DbInfoVersion: i32 = 8;
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_ENUMCOLUMN_0_0 {
    pub cEnumColumnValue: JET_UINT32,
    pub rgEnumColumnValue: *mut JET_ENUMCOLUMNVALUE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_ENUMCOLUMN_0_1 {
    pub cbData: JET_UINT32,
    pub pvData: JET_PVOID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_ENUMCOLUMNID {
    pub columnid: JET_COLUMNID,
    pub ctagSequence: JET_UINT32,
    pub rgtagSequence: *mut JET_UINT32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_ENUMCOLUMNVALUE {
    pub itagSequence: JET_UINT32,
    pub err: JET_ERR,
    pub cbData: JET_UINT32,
    pub pvData: JET_PVOID,
}
pub type JET_ERR = JET_INT32;
pub type JET_ERRCAT = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const JET_EventLoggingDisable: i32 = 0;
pub const JET_EventLoggingLevelHigh: i32 = 75;
pub const JET_EventLoggingLevelLow: i32 = 25;
pub const JET_EventLoggingLevelMax: i32 = 100;
pub const JET_EventLoggingLevelMedium: i32 = 50;
pub const JET_EventLoggingLevelMin: i32 = 1;
pub const JET_ExceptionFailFast: i32 = 4;
pub const JET_ExceptionMsgBox: i32 = 1;
pub const JET_ExceptionNone: i32 = 2;
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_INDEXRANGE {
    pub cbStruct: JET_UINT32,
    pub tableid: JET_TABLEID,
    pub grbit: JET_GRBIT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_INDEX_COLUMN {
    pub columnid: JET_COLUMNID,
    pub relop: JET_RELOP,
    pub pv: JET_PVOID,
    pub cb: JET_UINT32,
    pub grbit: JET_GRBIT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_INDEX_RANGE {
    pub rgStartColumns: *mut JET_INDEX_COLUMN,
    pub cStartColumns: JET_UINT32,
    pub rgEndColumns: *mut JET_INDEX_COLUMN,
    pub cEndColumns: JET_UINT32,
}
pub type JET_INSTANCE = JET_API_PTR;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_INSTANCE_INFO_A {
    pub hInstanceId: JET_INSTANCE,
    pub szInstanceName: JET_PSTR,
    pub cDatabases: JET_API_PTR,
    pub szDatabaseFileName: *mut JET_PSTR,
    pub szDatabaseDisplayName: *mut JET_PSTR,
    pub szDatabaseSLVFileName_Obsolete: *mut JET_PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_INSTANCE_INFO_W {
    pub hInstanceId: JET_INSTANCE,
    pub szInstanceName: JET_PWSTR,
    pub cDatabases: JET_API_PTR,
    pub szDatabaseFileName: *mut JET_PWSTR,
    pub szDatabaseDisplayName: *mut JET_PWSTR,
    pub szDatabaseSLVFileName_Obsolete: *mut JET_PWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct JET_INT16(pub i16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct JET_INT32(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct JET_INT64(pub i64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct JET_INT8(pub i8);
pub const JET_IOPriorityLow: i32 = 1;
pub const JET_IOPriorityNormal: i32 = 0;
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_LOGTIME_1_0 {
    pub _bitfield: JET_BYTE,
}
pub type JET_LS = JET_API_PTR;
#[cfg(target_arch = "x86")]
pub const JET_LSNil: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const JET_LSNil: u64 = 18446744073709551615;
pub const JET_MAX_COMPUTERNAME_LENGTH: i32 = 15;
pub const JET_MoveFirst: u32 = 2147483648;
pub const JET_MoveLast: i32 = 2147483647;
pub const JET_MoveNext: i32 = 1;
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub const JET_OnlineDefragAll: i32 = 65535;
pub const JET_OnlineDefragAllOBSOLETE: i32 = 1;
pub const JET_OnlineDefragDatabases: i32 = 2;
pub const JET_OnlineDefragDisable: i32 = 0;
pub const JET_OnlineDefragSpaceTrees: i32 = 4;
pub type JET_PCSTR = *const JET_CHAR;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct JET_PCVOID(pub *const core::ffi::c_void);
pub type JET_PCWSTR = *const JET_WCHAR;
pub type JET_PFNDURABLECOMMITCALLBACK = Option<unsafe extern "system" fn(instance: JET_INSTANCE, pcommitidseen: *const JET_COMMIT_ID, grbit: JET_GRBIT) -> JET_ERR>;
pub type JET_PFNREALLOC = Option<unsafe extern "system" fn(pvcontext: JET_PVOID, pv: JET_PVOID, cb: JET_UINT32) -> JET_PVOID>;
pub type JET_PFNSTATUS = Option<unsafe extern "system" fn(sesid: JET_SESID, snp: JET_SNP, snt: JET_SNT, pv: JET_PVOID) -> JET_ERR>;
pub type JET_PSTR = *mut JET_CHAR;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct JET_PVOID(pub *mut core::ffi::c_void);
pub type JET_PWSTR = *mut JET_WCHAR;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_RECORDLIST {
    pub cbStruct: JET_UINT32,
    pub tableid: JET_TABLEID,
    pub cRecord: JET_UINT32,
    pub columnidBookmark: JET_COLUMNID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_RETINFO {
    pub cbStruct: JET_UINT32,
    pub ibLongValue: JET_UINT32,
    pub itagSequence: JET_UINT32,
    pub columnidNextTagged: JET_COLUMNID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_RSTMAP_A {
    pub szDatabaseName: JET_PSTR,
    pub szNewDatabaseName: JET_PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_RSTMAP_W {
    pub szDatabaseName: JET_PWSTR,
    pub szNewDatabaseName: JET_PWSTR,
}
pub type JET_SESID = JET_API_PTR;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_SETCOLUMN {
    pub columnid: JET_COLUMNID,
    pub pvData: JET_PCVOID,
    pub cbData: JET_UINT32,
    pub grbit: JET_GRBIT,
    pub ibLongValue: JET_UINT32,
    pub itagSequence: JET_UINT32,
    pub err: JET_ERR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_SETINFO {
    pub cbStruct: JET_UINT32,
    pub ibLongValue: JET_UINT32,
    pub itagSequence: JET_UINT32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_SETSYSPARAM_A {
    pub paramid: JET_UINT32,
    pub lParam: JET_API_PTR,
    pub sz: JET_PCSTR,
    pub err: JET_ERR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_SETSYSPARAM_W {
    pub paramid: JET_UINT32,
    pub lParam: JET_API_PTR,
    pub sz: JET_PCWSTR,
    pub err: JET_ERR,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_SNPROG {
    pub cbStruct: JET_UINT32,
    pub cunitDone: JET_UINT32,
    pub cunitTotal: JET_UINT32,
}
pub type JET_SNT = JET_UINT32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub type JET_TABLEID = JET_API_PTR;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct JET_UINT16(pub u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct JET_UINT32(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct JET_UINT64(pub u64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct JET_UINT8(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_UNICODEINDEX {
    pub lcid: JET_LCID,
    pub dwMapFlags: JET_UINT32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_UNICODEINDEX2 {
    pub szLocaleName: JET_PWSTR,
    pub dwMapFlags: JET_UINT32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_USERDEFINEDDEFAULT_A {
    pub szCallback: JET_PSTR,
    pub pbUserData: *mut JET_BYTE,
    pub cbUserData: JET_UINT32,
    pub szDependantColumns: JET_PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JET_USERDEFINEDDEFAULT_W {
    pub szCallback: JET_PWSTR,
    pub pbUserData: *mut JET_BYTE,
    pub cbUserData: JET_UINT32,
    pub szDependantColumns: JET_PWSTR,
}
pub const JET_VERSION: i32 = 2560;
pub type JET_VOID = core::ffi::c_void;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct JET_WCHAR(pub u16);
pub const JET_bitAbortSnapshot: i32 = 1;
pub const JET_bitAllDatabasesSnapshot: i32 = 1;
pub const JET_bitBackupAtomic: i32 = 4;
pub const JET_bitBackupEndAbort: i32 = 2;
pub const JET_bitBackupEndNormal: i32 = 1;
pub const JET_bitBackupIncremental: i32 = 1;
pub const JET_bitBackupSnapshot: i32 = 16;
pub const JET_bitBackupTruncateDone: i32 = 256;
pub const JET_bitBookmarkPermitVirtualCurrency: i32 = 1;
pub const JET_bitCheckUniqueness: i32 = 64;
pub const JET_bitColumnAutoincrement: i32 = 16;
pub const JET_bitColumnCompressed: i32 = 524288;
pub const JET_bitColumnDeleteOnZero: i32 = 131072;
pub const JET_bitColumnEscrowUpdate: i32 = 2048;
pub const JET_bitColumnFinalize: i32 = 16384;
pub const JET_bitColumnFixed: i32 = 1;
pub const JET_bitColumnMaybeNull: i32 = 8192;
pub const JET_bitColumnMultiValued: i32 = 1024;
pub const JET_bitColumnNotNULL: i32 = 4;
pub const JET_bitColumnTTDescending: i32 = 128;
pub const JET_bitColumnTTKey: i32 = 64;
pub const JET_bitColumnTagged: i32 = 2;
pub const JET_bitColumnUnversioned: i32 = 4096;
pub const JET_bitColumnUpdatable: i32 = 32;
pub const JET_bitColumnUserDefinedDefault: i32 = 32768;
pub const JET_bitColumnVersion: i32 = 8;
pub const JET_bitCommitLazyFlush: i32 = 1;
pub const JET_bitCompactRepair: i32 = 64;
pub const JET_bitCompactStats: i32 = 32;
pub const JET_bitConfigStoreReadControlDefault: i32 = 0;
pub const JET_bitConfigStoreReadControlDisableAll: i32 = 2;
pub const JET_bitConfigStoreReadControlInhibitRead: i32 = 1;
pub const JET_bitContinueAfterThaw: i32 = 4;
pub const JET_bitCopySnapshot: i32 = 2;
pub const JET_bitCreateHintAppendSequential: i32 = 2;
pub const JET_bitCreateHintHotpointSequential: i32 = 4;
pub const JET_bitDbDeleteCorruptIndexes: i32 = 16;
pub const JET_bitDbDeleteUnicodeIndexes: i32 = 1024;
pub const JET_bitDbEnableBackgroundMaintenance: i32 = 2048;
pub const JET_bitDbExclusive: i32 = 2;
pub const JET_bitDbOverwriteExisting: i32 = 512;
pub const JET_bitDbPurgeCacheOnAttach: i32 = 4096;
pub const JET_bitDbReadOnly: i32 = 1;
pub const JET_bitDbRecoveryOff: i32 = 8;
pub const JET_bitDbShadowingOff: i32 = 128;
pub const JET_bitDbUpgrade: i32 = 512;
pub const JET_bitDefragmentAvailSpaceTreesOnly: i32 = 64;
pub const JET_bitDefragmentBTree: i32 = 256;
pub const JET_bitDefragmentBatchStart: i32 = 1;
pub const JET_bitDefragmentBatchStop: i32 = 2;
pub const JET_bitDefragmentNoPartialMerges: i32 = 128;
pub const JET_bitDeleteColumnIgnoreTemplateColumns: i32 = 1;
pub const JET_bitDeleteHintTableSequential: i32 = 256;
pub const JET_bitDumpCacheIncludeCachedPages: i32 = 32;
pub const JET_bitDumpCacheIncludeCorruptedPages: i32 = 64;
pub const JET_bitDumpCacheIncludeDirtyPages: i32 = 16;
pub const JET_bitDumpCacheMaximum: i32 = 8;
pub const JET_bitDumpCacheMinimum: i32 = 4;
pub const JET_bitDumpCacheNoDecommit: i32 = 128;
pub const JET_bitDumpMaximum: i32 = 2;
pub const JET_bitDumpMinimum: i32 = 1;
pub const JET_bitDurableCommitCallbackLogUnavailable: i32 = 1;
pub const JET_bitESE98FileNames: i32 = 1;
pub const JET_bitEightDotThreeSoftCompat: i32 = 2;
pub const JET_bitEnumerateCompressOutput: i32 = 524288;
pub const JET_bitEnumerateCopy: i32 = 1;
pub const JET_bitEnumerateIgnoreDefault: i32 = 32;
pub const JET_bitEnumerateIgnoreUserDefinedDefault: i32 = 1048576;
pub const JET_bitEnumerateInRecordOnly: i32 = 2097152;
pub const JET_bitEnumeratePresenceOnly: i32 = 131072;
pub const JET_bitEnumerateTaggedOnly: i32 = 262144;
pub const JET_bitEscrowNoRollback: i32 = 1;
pub const JET_bitExplicitPrepare: i32 = 8;
pub const JET_bitForceCloseAndDetach: i32 = 3;
pub const JET_bitForceDetach: i32 = 1;
pub const JET_bitForceNewLog: i32 = 16;
pub const JET_bitFullColumnEndLimit: i32 = 512;
pub const JET_bitFullColumnStartLimit: i32 = 256;
pub const JET_bitHungIOEvent: i32 = 1;
pub const JET_bitIdleCompact: i32 = 2;
pub const JET_bitIdleFlushBuffers: i32 = 1;
pub const JET_bitIdleStatus: i32 = 4;
pub const JET_bitIncrementalSnapshot: i32 = 1;
pub const JET_bitIndexColumnMustBeNonNull: i32 = 2;
pub const JET_bitIndexColumnMustBeNull: i32 = 1;
pub const JET_bitIndexCrossProduct: i32 = 16384;
pub const JET_bitIndexDisallowNull: i32 = 4;
pub const JET_bitIndexDisallowTruncation: i32 = 65536;
pub const JET_bitIndexDotNetGuid: i32 = 262144;
pub const JET_bitIndexEmpty: i32 = 256;
pub const JET_bitIndexIgnoreAnyNull: i32 = 32;
pub const JET_bitIndexIgnoreFirstNull: i32 = 64;
pub const JET_bitIndexIgnoreNull: i32 = 8;
pub const JET_bitIndexImmutableStructure: i32 = 524288;
pub const JET_bitIndexKeyMost: i32 = 32768;
pub const JET_bitIndexLazyFlush: i32 = 128;
pub const JET_bitIndexNestedTable: i32 = 131072;
pub const JET_bitIndexPrimary: i32 = 2;
pub const JET_bitIndexSortNullsHigh: i32 = 1024;
pub const JET_bitIndexTupleLimits: i32 = 8192;
pub const JET_bitIndexTuples: i32 = 4096;
pub const JET_bitIndexUnicode: i32 = 2048;
pub const JET_bitIndexUnique: i32 = 1;
pub const JET_bitIndexUnversioned: i32 = 512;
pub const JET_bitKeepDbAttachedAtEndOfRecovery: i32 = 4096;
pub const JET_bitKeyAscending: i32 = 0;
pub const JET_bitKeyDataZeroLength: i32 = 16;
pub const JET_bitKeyDescending: i32 = 1;
pub const JET_bitLSCursor: i32 = 2;
pub const JET_bitLSReset: i32 = 1;
pub const JET_bitLSTable: i32 = 4;
pub const JET_bitLogStreamMustExist: i32 = 64;
pub const JET_bitMoveFirst: i32 = 0;
pub const JET_bitMoveKeyNE: i32 = 1;
pub const JET_bitNewKey: i32 = 1;
pub const JET_bitNil: JET_GRBIT = JET_UINT32(0 as _);
pub const JET_bitNoMove: i32 = 2;
pub const JET_bitNormalizedKey: i32 = 8;
pub const JET_bitObjectSystem: u32 = 2147483648;
pub const JET_bitObjectTableDerived: i32 = 268435456;
pub const JET_bitObjectTableFixedDDL: i32 = 1073741824;
pub const JET_bitObjectTableNoFixedVarColumnsInDerivedTables: i32 = 67108864;
pub const JET_bitObjectTableTemplate: i32 = 536870912;
pub const JET_bitPartialColumnEndLimit: i32 = 2048;
pub const JET_bitPartialColumnStartLimit: i32 = 1024;
pub const JET_bitPrereadBackward: i32 = 2;
pub const JET_bitPrereadFirstPage: i32 = 4;
pub const JET_bitPrereadForward: i32 = 1;
pub const JET_bitPrereadNormalizedKey: i32 = 8;
pub const JET_bitRangeInclusive: i32 = 1;
pub const JET_bitRangeInstantDuration: i32 = 4;
pub const JET_bitRangeRemove: i32 = 8;
pub const JET_bitRangeUpperLimit: i32 = 2;
pub const JET_bitReadLock: i32 = 1;
pub const JET_bitRecordInIndex: i32 = 1;
pub const JET_bitRecordNotInIndex: i32 = 2;
pub const JET_bitRecordSizeInCopyBuffer: i32 = 1;
pub const JET_bitRecordSizeLocal: i32 = 4;
pub const JET_bitRecordSizeRunningTotal: i32 = 2;
pub const JET_bitRecoveryWithoutUndo: i32 = 8;
pub const JET_bitReplayIgnoreLostLogs: i32 = 128;
pub const JET_bitReplayIgnoreMissingDB: i32 = 4;
pub const JET_bitReplayMissingMapEntryDB: i32 = 32;
pub const JET_bitResizeDatabaseOnlyGrow: i32 = 1;
pub const JET_bitResizeDatabaseOnlyShrink: i32 = 2;
pub const JET_bitRetrieveCopy: i32 = 1;
pub const JET_bitRetrieveFromIndex: i32 = 2;
pub const JET_bitRetrieveFromPrimaryBookmark: i32 = 4;
pub const JET_bitRetrieveHintReserve1: i32 = 8;
pub const JET_bitRetrieveHintReserve2: i32 = 64;
pub const JET_bitRetrieveHintReserve3: i32 = 128;
pub const JET_bitRetrieveHintTableScanBackward: i32 = 32;
pub const JET_bitRetrieveHintTableScanForward: i32 = 16;
pub const JET_bitRetrieveIgnoreDefault: i32 = 32;
pub const JET_bitRetrieveNull: i32 = 16;
pub const JET_bitRetrieveTag: i32 = 8;
pub const JET_bitRetrieveTuple: i32 = 2048;
pub const JET_bitRollbackAll: i32 = 1;
pub const JET_bitSeekEQ: i32 = 1;
pub const JET_bitSeekGE: i32 = 8;
pub const JET_bitSeekGT: i32 = 16;
pub const JET_bitSeekLE: i32 = 4;
pub const JET_bitSeekLT: i32 = 2;
pub const JET_bitSetAppendLV: i32 = 1;
pub const JET_bitSetCompressed: i32 = 131072;
pub const JET_bitSetIndexRange: i32 = 32;
pub const JET_bitSetIntrinsicLV: i32 = 1024;
pub const JET_bitSetOverwriteLV: i32 = 4;
pub const JET_bitSetRevertToDefaultValue: i32 = 512;
pub const JET_bitSetSeparateLV: i32 = 64;
pub const JET_bitSetSizeLV: i32 = 8;
pub const JET_bitSetUncompressed: i32 = 65536;
pub const JET_bitSetUniqueMultiValues: i32 = 128;
pub const JET_bitSetUniqueNormalizedMultiValues: i32 = 256;
pub const JET_bitSetZeroLength: i32 = 32;
pub const JET_bitShrinkDatabaseOff: i32 = 0;
pub const JET_bitShrinkDatabaseOn: i32 = 1;
pub const JET_bitShrinkDatabaseRealtime: i32 = 2;
pub const JET_bitShrinkDatabaseTrim: i32 = 1;
pub const JET_bitSpaceHintsUtilizeParentSpace: i32 = 1;
pub const JET_bitStopServiceAll: i32 = 0;
pub const JET_bitStopServiceBackgroundUserTasks: i32 = 2;
pub const JET_bitStopServiceQuiesceCaches: i32 = 4;
pub const JET_bitStopServiceResume: u32 = 2147483648;
pub const JET_bitStrLimit: i32 = 2;
pub const JET_bitSubStrLimit: i32 = 4;
pub const JET_bitTTDotNetGuid: i32 = 256;
pub const JET_bitTTErrorOnDuplicateInsertion: i32 = 32;
pub const JET_bitTTForceMaterialization: i32 = 32;
pub const JET_bitTTForwardOnly: i32 = 64;
pub const JET_bitTTIndexed: i32 = 1;
pub const JET_bitTTIntrinsicLVsOnly: i32 = 128;
pub const JET_bitTTScrollable: i32 = 8;
pub const JET_bitTTSortNullsHigh: i32 = 16;
pub const JET_bitTTUnique: i32 = 2;
pub const JET_bitTTUpdatable: i32 = 4;
pub const JET_bitTableClass1: i32 = 65536;
pub const JET_bitTableClass10: i32 = 655360;
pub const JET_bitTableClass11: i32 = 720896;
pub const JET_bitTableClass12: i32 = 786432;
pub const JET_bitTableClass13: i32 = 851968;
pub const JET_bitTableClass14: i32 = 917504;
pub const JET_bitTableClass15: i32 = 983040;
pub const JET_bitTableClass2: i32 = 131072;
pub const JET_bitTableClass3: i32 = 196608;
pub const JET_bitTableClass4: i32 = 262144;
pub const JET_bitTableClass5: i32 = 327680;
pub const JET_bitTableClass6: i32 = 393216;
pub const JET_bitTableClass7: i32 = 458752;
pub const JET_bitTableClass8: i32 = 524288;
pub const JET_bitTableClass9: i32 = 589824;
pub const JET_bitTableClassMask: i32 = 2031616;
pub const JET_bitTableClassNone: i32 = 0;
pub const JET_bitTableCreateFixedDDL: i32 = 1;
pub const JET_bitTableCreateImmutableStructure: i32 = 8;
pub const JET_bitTableCreateNoFixedVarColumnsInDerivedTables: i32 = 4;
pub const JET_bitTableCreateTemplateTable: i32 = 2;
pub const JET_bitTableDenyRead: i32 = 2;
pub const JET_bitTableDenyWrite: i32 = 1;
pub const JET_bitTableInfoBookmark: i32 = 2;
pub const JET_bitTableInfoRollback: i32 = 4;
pub const JET_bitTableInfoUpdatable: i32 = 1;
pub const JET_bitTableNoCache: i32 = 32;
pub const JET_bitTableOpportuneRead: i32 = 128;
pub const JET_bitTablePermitDDL: i32 = 16;
pub const JET_bitTablePreread: i32 = 64;
pub const JET_bitTableReadOnly: i32 = 4;
pub const JET_bitTableSequential: i32 = 32768;
pub const JET_bitTableUpdatable: i32 = 8;
pub const JET_bitTermAbrupt: i32 = 2;
pub const JET_bitTermComplete: i32 = 1;
pub const JET_bitTermDirty: i32 = 8;
pub const JET_bitTermStopBackup: i32 = 4;
pub const JET_bitTransactionReadOnly: i32 = 1;
pub const JET_bitTruncateLogsAfterRecovery: i32 = 16;
pub const JET_bitUpdateCheckESE97Compatibility: i32 = 1;
pub const JET_bitWaitAllLevel0Commit: i32 = 8;
pub const JET_bitWaitLastLevel0Commit: i32 = 2;
pub const JET_bitWriteLock: i32 = 2;
pub const JET_bitZeroLength: i32 = 1;
pub const JET_cbBookmarkMost: i32 = 256;
pub const JET_cbBookmarkMostMost: i32 = 2000;
pub const JET_cbColumnLVPageOverhead: i32 = 82;
pub const JET_cbColumnMost: i32 = 255;
pub const JET_cbFullNameMost: i32 = 255;
pub const JET_cbKeyMost: i32 = 255;
pub const JET_cbKeyMost16KBytePage: i32 = 2000;
pub const JET_cbKeyMost2KBytePage: i32 = 500;
pub const JET_cbKeyMost32KBytePage: i32 = 2000;
pub const JET_cbKeyMost4KBytePage: i32 = 1000;
pub const JET_cbKeyMost8KBytePage: i32 = 2000;
pub const JET_cbKeyMostMin: i32 = 255;
pub const JET_cbKeyMostMost: i32 = 2000;
pub const JET_cbLVColumnMost: i32 = 2147483647;
pub const JET_cbLVDefaultValueMost: i32 = 255;
pub const JET_cbLimitKeyMost: i32 = 256;
pub const JET_cbNameMost: i32 = 64;
pub const JET_cbPrimaryKeyMost: i32 = 255;
pub const JET_cbSecondaryKeyMost: i32 = 255;
pub const JET_cbtypAfterDelete: i32 = 64;
pub const JET_cbtypAfterInsert: i32 = 4;
pub const JET_cbtypAfterReplace: i32 = 16;
pub const JET_cbtypBeforeDelete: i32 = 32;
pub const JET_cbtypBeforeInsert: i32 = 2;
pub const JET_cbtypBeforeReplace: i32 = 8;
pub const JET_cbtypFinalize: i32 = 1;
pub const JET_cbtypFreeCursorLS: i32 = 512;
pub const JET_cbtypFreeTableLS: i32 = 1024;
pub const JET_cbtypNull: i32 = 0;
pub const JET_cbtypOnlineDefragCompleted: i32 = 256;
pub const JET_cbtypUserDefinedDefaultValue: i32 = 128;
pub const JET_ccolFixedMost: i32 = 127;
pub const JET_ccolKeyMost: i32 = 16;
pub const JET_ccolMost: i32 = 65248;
pub const JET_ccolTaggedMost: i32 = 64993;
pub const JET_ccolVarMost: i32 = 128;
pub const JET_coltypBinary: i32 = 9;
pub const JET_coltypBit: i32 = 1;
pub const JET_coltypCurrency: i32 = 5;
pub const JET_coltypDateTime: i32 = 8;
pub const JET_coltypGUID: i32 = 16;
pub const JET_coltypIEEEDouble: i32 = 7;
pub const JET_coltypIEEESingle: i32 = 6;
pub const JET_coltypLong: i32 = 4;
pub const JET_coltypLongBinary: i32 = 11;
pub const JET_coltypLongLong: i32 = 15;
pub const JET_coltypLongText: i32 = 12;
pub const JET_coltypMax: i32 = 19;
pub const JET_coltypNil: i32 = 0;
pub const JET_coltypSLV: i32 = 13;
pub const JET_coltypShort: i32 = 3;
pub const JET_coltypText: i32 = 10;
pub const JET_coltypUnsignedByte: i32 = 2;
pub const JET_coltypUnsignedLong: i32 = 14;
pub const JET_coltypUnsignedLongLong: i32 = 18;
pub const JET_coltypUnsignedShort: i32 = 17;
pub const JET_configDefault: i32 = 1;
pub const JET_configDynamicMediumMemory: i32 = 32;
pub const JET_configHighConcurrencyScaling: i32 = 1024;
pub const JET_configLowDiskFootprint: i32 = 4;
pub const JET_configLowMemory: i32 = 16;
pub const JET_configLowPower: i32 = 64;
pub const JET_configMediumDiskFootprint: i32 = 8;
pub const JET_configRemoveQuotas: i32 = 2;
pub const JET_configRunSilent: i32 = 256;
pub const JET_configSSDProfileIO: i32 = 128;
pub const JET_configUnthrottledMemory: i32 = 512;
pub const JET_dbidNil: JET_DBID = JET_UINT32(4294967295u32 as _);
pub const JET_dbstateBeingConverted: i32 = 4;
pub const JET_dbstateCleanShutdown: i32 = 3;
pub const JET_dbstateDirtyShutdown: i32 = 2;
pub const JET_dbstateForceDetach: i32 = 5;
pub const JET_dbstateJustCreated: i32 = 1;
pub const JET_efvAllowHigherPersistedFormat: i32 = 1090519040;
pub const JET_efvUseEngineDefault: i32 = 1073741825;
pub const JET_efvUsePersistedFormat: i32 = 1073741826;
pub const JET_efvWindows10v2004: i32 = 9180;
pub const JET_efvWindows11v21H2: i32 = 9400;
pub const JET_efvWindows11v22H2: i32 = 9480;
pub const JET_efvWindows19H1Rtm: i32 = 8920;
pub const JET_efvWindowsServer2022: i32 = 9360;
pub const JET_efvWindowsServer2025: i32 = 9620;
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
pub const JET_errSuccess: i32 = 0;
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
pub const JET_filetypeCheckpoint: i32 = 4;
pub const JET_filetypeDatabase: i32 = 1;
pub const JET_filetypeFlushMap: i32 = 7;
pub const JET_filetypeLog: i32 = 3;
pub const JET_filetypeTempDatabase: i32 = 5;
pub const JET_filetypeUnknown: i32 = 0;
#[cfg(target_arch = "x86")]
pub const JET_instanceNil: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const JET_instanceNil: u64 = 18446744073709551615;
pub const JET_objtypNil: i32 = 0;
pub const JET_objtypTable: i32 = 1;
pub const JET_paramAccessDeniedRetryPeriod: i32 = 53;
pub const JET_paramAlternateDatabaseRecoveryPath: i32 = 113;
pub const JET_paramBaseName: i32 = 3;
pub const JET_paramBatchIOBufferMax: i32 = 22;
pub const JET_paramCachePriority: i32 = 177;
pub const JET_paramCacheSize: i32 = 41;
pub const JET_paramCacheSizeMax: i32 = 23;
pub const JET_paramCacheSizeMin: i32 = 60;
pub const JET_paramCachedClosedTables: i32 = 125;
pub const JET_paramCheckFormatWhenOpenFail: i32 = 44;
pub const JET_paramCheckpointDepthMax: i32 = 24;
pub const JET_paramCheckpointIOMax: i32 = 135;
pub const JET_paramCircularLog: i32 = 17;
pub const JET_paramCleanupMismatchedLogFiles: i32 = 77;
pub const JET_paramCommitDefault: i32 = 16;
pub const JET_paramConfigStoreSpec: i32 = 189;
pub const JET_paramConfiguration: i32 = 129;
pub const JET_paramCreatePathIfNotExist: i32 = 100;
pub const JET_paramDatabasePageSize: i32 = 64;
pub const JET_paramDbExtensionSize: i32 = 18;
pub const JET_paramDbScanIntervalMaxSec: i32 = 172;
pub const JET_paramDbScanIntervalMinSec: i32 = 171;
pub const JET_paramDbScanThrottle: i32 = 170;
pub const JET_paramDefragmentSequentialBTrees: i32 = 160;
pub const JET_paramDefragmentSequentialBTreesDensityCheckFrequency: i32 = 161;
pub const JET_paramDeleteOldLogs: i32 = 48;
pub const JET_paramDeleteOutOfRangeLogs: i32 = 52;
pub const JET_paramDisableCallbacks: i32 = 65;
pub const JET_paramDisablePerfmon: i32 = 107;
pub const JET_paramDurableCommitCallback: i32 = 187;
pub const JET_paramEnableAdvanced: i32 = 130;
pub const JET_paramEnableDBScanInRecovery: i32 = 169;
pub const JET_paramEnableDBScanSerialization: i32 = 180;
pub const JET_paramEnableFileCache: i32 = 126;
pub const JET_paramEnableIndexChecking: i32 = 45;
pub const JET_paramEnableIndexCleanup: i32 = 54;
pub const JET_paramEnableOnlineDefrag: i32 = 35;
pub const JET_paramEnablePersistedCallbacks: i32 = 156;
pub const JET_paramEnableShrinkDatabase: i32 = 184;
pub const JET_paramEnableSqm: i32 = 188;
pub const JET_paramEnableTempTableVersioning: i32 = 46;
pub const JET_paramEnableViewCache: i32 = 127;
pub const JET_paramEngineFormatVersion: i32 = 194;
pub const JET_paramErrorToString: i32 = 70;
pub const JET_paramEventLogCache: i32 = 99;
pub const JET_paramEventLoggingLevel: i32 = 51;
pub const JET_paramEventSource: i32 = 4;
pub const JET_paramEventSourceKey: i32 = 49;
pub const JET_paramExceptionAction: i32 = 98;
pub const JET_paramGlobalMinVerPages: i32 = 81;
pub const JET_paramHungIOActions: i32 = 182;
pub const JET_paramHungIOThreshold: i32 = 181;
pub const JET_paramIOPriority: i32 = 152;
pub const JET_paramIOThrottlingTimeQuanta: i32 = 162;
pub const JET_paramIgnoreLogVersion: i32 = 47;
pub const JET_paramIndexTupleIncrement: i32 = 132;
pub const JET_paramIndexTupleStart: i32 = 133;
pub const JET_paramIndexTuplesLengthMax: i32 = 111;
pub const JET_paramIndexTuplesLengthMin: i32 = 110;
pub const JET_paramIndexTuplesToIndexMax: i32 = 112;
pub const JET_paramKeyMost: i32 = 134;
pub const JET_paramLRUKCorrInterval: i32 = 25;
pub const JET_paramLRUKHistoryMax: i32 = 26;
pub const JET_paramLRUKPolicy: i32 = 27;
pub const JET_paramLRUKTimeout: i32 = 28;
pub const JET_paramLRUKTrxCorrInterval: i32 = 29;
pub const JET_paramLVChunkSizeMost: i32 = 163;
pub const JET_paramLegacyFileNames: i32 = 136;
pub const JET_paramLogBuffers: i32 = 12;
pub const JET_paramLogCheckpointPeriod: i32 = 14;
pub const JET_paramLogFileCreateAsynch: i32 = 69;
pub const JET_paramLogFilePath: i32 = 2;
pub const JET_paramLogFileSize: i32 = 11;
pub const JET_paramLogWaitingUserMax: i32 = 15;
pub const JET_paramMaxCoalesceReadGapSize: i32 = 166;
pub const JET_paramMaxCoalesceReadSize: i32 = 164;
pub const JET_paramMaxCoalesceWriteGapSize: i32 = 167;
pub const JET_paramMaxCoalesceWriteSize: i32 = 165;
pub const JET_paramMaxColtyp: i32 = 131;
pub const JET_paramMaxCursors: i32 = 8;
pub const JET_paramMaxInstances: i32 = 104;
pub const JET_paramMaxOpenTables: i32 = 6;
pub const JET_paramMaxSessions: i32 = 5;
pub const JET_paramMaxTemporaryTables: i32 = 10;
pub const JET_paramMaxTransactionSize: i32 = 178;
pub const JET_paramMaxValueInvalid: i32 = 249;
pub const JET_paramMaxVerPages: i32 = 9;
pub const JET_paramMinDataForXpress: i32 = 183;
pub const JET_paramNoInformationEvent: i32 = 50;
pub const JET_paramOSSnapshotTimeout: i32 = 82;
pub const JET_paramOneDatabasePerSession: i32 = 102;
pub const JET_paramOutstandingIOMax: i32 = 30;
pub const JET_paramPageFragment: i32 = 20;
pub const JET_paramPageHintCacheSize: i32 = 101;
pub const JET_paramPageTempDBMin: i32 = 19;
pub const JET_paramPreferredMaxOpenTables: i32 = 7;
pub const JET_paramPreferredVerPages: i32 = 63;
pub const JET_paramPrereadIOMax: i32 = 179;
pub const JET_paramProcessFriendlyName: i32 = 186;
pub const JET_paramRecordUpgradeDirtyLevel: i32 = 78;
pub const JET_paramRecovery: i32 = 34;
pub const JET_paramRuntimeCallback: i32 = 73;
pub const JET_paramStartFlushThreshold: i32 = 31;
pub const JET_paramStopFlushThreshold: i32 = 32;
pub const JET_paramSystemPath: i32 = 0;
pub const JET_paramTableClass10Name: i32 = 146;
pub const JET_paramTableClass11Name: i32 = 147;
pub const JET_paramTableClass12Name: i32 = 148;
pub const JET_paramTableClass13Name: i32 = 149;
pub const JET_paramTableClass14Name: i32 = 150;
pub const JET_paramTableClass15Name: i32 = 151;
pub const JET_paramTableClass1Name: i32 = 137;
pub const JET_paramTableClass2Name: i32 = 138;
pub const JET_paramTableClass3Name: i32 = 139;
pub const JET_paramTableClass4Name: i32 = 140;
pub const JET_paramTableClass5Name: i32 = 141;
pub const JET_paramTableClass6Name: i32 = 142;
pub const JET_paramTableClass7Name: i32 = 143;
pub const JET_paramTableClass8Name: i32 = 144;
pub const JET_paramTableClass9Name: i32 = 145;
pub const JET_paramTempPath: i32 = 1;
pub const JET_paramTraceFlags: i32 = 223;
pub const JET_paramUnicodeIndexDefault: i32 = 72;
pub const JET_paramVerPageSize: i32 = 128;
pub const JET_paramVersionStoreTaskQueueMax: i32 = 105;
pub const JET_paramWaitLogFlush: i32 = 13;
pub const JET_paramWaypointLatency: i32 = 153;
pub const JET_paramZeroDatabaseDuringBackup: i32 = 71;
pub const JET_prepCancel: i32 = 3;
pub const JET_prepInsert: i32 = 0;
pub const JET_prepInsertCopy: i32 = 5;
pub const JET_prepInsertCopyDeleteOriginal: i32 = 7;
pub const JET_prepInsertCopyReplaceOriginal: i32 = 9;
pub const JET_prepReplace: i32 = 2;
pub const JET_prepReplaceNoLock: i32 = 4;
pub const JET_relopBitmaskEqualsZero: JET_RELOP = 7;
pub const JET_relopBitmaskNotEqualsZero: JET_RELOP = 8;
pub const JET_relopEquals: JET_RELOP = 0;
pub const JET_relopGreaterThan: JET_RELOP = 6;
pub const JET_relopGreaterThanOrEqual: JET_RELOP = 5;
pub const JET_relopLessThan: JET_RELOP = 4;
pub const JET_relopLessThanOrEqual: JET_RELOP = 3;
pub const JET_relopNotEquals: JET_RELOP = 2;
pub const JET_relopPrefixEquals: JET_RELOP = 1;
#[cfg(target_arch = "x86")]
pub const JET_sesidNil: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const JET_sesidNil: u64 = 18446744073709551615;
pub const JET_sesparamCommitDefault: i32 = 4097;
pub const JET_sesparamCorrelationID: i32 = 4101;
pub const JET_sesparamMaxValueInvalid: i32 = 4111;
pub const JET_sesparamOperationContext: i32 = 4100;
pub const JET_sesparamTransactionLevel: i32 = 4099;
pub const JET_snpBackup: i32 = 9;
pub const JET_snpCompact: i32 = 4;
pub const JET_snpRepair: i32 = 2;
pub const JET_snpRestore: i32 = 8;
pub const JET_snpScrub: i32 = 11;
pub const JET_snpUpgrade: i32 = 10;
pub const JET_snpUpgradeRecordFormat: i32 = 12;
pub const JET_sntBegin: i32 = 5;
pub const JET_sntComplete: i32 = 6;
pub const JET_sntFail: i32 = 3;
pub const JET_sntProgress: i32 = 0;
pub const JET_sntRequirements: i32 = 7;
pub const JET_sqmDisable: i32 = 0;
pub const JET_sqmEnable: i32 = 1;
pub const JET_sqmFromCEIP: i32 = 2;
#[cfg(target_arch = "x86")]
pub const JET_tableidNil: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const JET_tableidNil: u64 = 18446744073709551615;
pub const JET_wrnBufferTruncated: i32 = 1006;
pub const JET_wrnCallbackNotRegistered: i32 = 2100;
pub const JET_wrnColumnDefault: i32 = 1537;
pub const JET_wrnColumnMaxTruncated: i32 = 1512;
pub const JET_wrnColumnMoreTags: i32 = 1533;
pub const JET_wrnColumnNotInRecord: i32 = 1539;
pub const JET_wrnColumnNotLocal: i32 = 1532;
pub const JET_wrnColumnNull: i32 = 1004;
pub const JET_wrnColumnPresent: i32 = 1535;
pub const JET_wrnColumnReference: i32 = 1541;
pub const JET_wrnColumnSetNull: i32 = 1068;
pub const JET_wrnColumnSingleValue: i32 = 1536;
pub const JET_wrnColumnSkipped: i32 = 1531;
pub const JET_wrnColumnTruncated: i32 = 1534;
pub const JET_wrnCommittedLogFilesLost: i32 = 585;
pub const JET_wrnCommittedLogFilesRemoved: i32 = 587;
pub const JET_wrnCopyCompletedAlready: i32 = 8002;
pub const JET_wrnCopyLongValue: i32 = 1520;
pub const JET_wrnCorruptIndexDeleted: i32 = 1415;
pub const JET_wrnDataHasChanged: i32 = 1610;
pub const JET_wrnDatabaseAttached: i32 = 1007;
pub const JET_wrnDatabaseRepaired: i32 = 595;
pub const JET_wrnDefragAlreadyRunning: i32 = 2000;
pub const JET_wrnDefragNotRunning: i32 = 2001;
pub const JET_wrnExistingLogFileHasBadSignature: i32 = 558;
pub const JET_wrnExistingLogFileIsNotContiguous: i32 = 559;
pub const JET_wrnFileOpenReadOnly: i32 = 1813;
pub const JET_wrnFinishWithUndo: i32 = 588;
pub const JET_wrnIdleFull: i32 = 1908;
pub const JET_wrnKeyChanged: i32 = 1618;
pub const JET_wrnNoErrorInfo: i32 = 1055;
pub const JET_wrnNoIdleActivity: i32 = 1058;
pub const JET_wrnNoWriteLock: i32 = 1067;
pub const JET_wrnNyi: i32 = -1;
pub const JET_wrnPrimaryIndexOutOfDate: i32 = 1417;
pub const JET_wrnRecordFoundGreater: i32 = 1039;
pub const JET_wrnRecordFoundLess: i32 = 1039;
pub const JET_wrnRemainingVersions: i32 = 321;
pub const JET_wrnSecondaryIndexOutOfDate: i32 = 1418;
pub const JET_wrnSeekNotEqual: i32 = 1039;
pub const JET_wrnSeparateLongValue: i32 = 406;
pub const JET_wrnShrinkNotPossible: i32 = 1122;
pub const JET_wrnSkipThisRecord: i32 = 564;
pub const JET_wrnSortOverflow: i32 = 1009;
pub const JET_wrnTableEmpty: i32 = 1301;
pub const JET_wrnTableInUseBySystem: i32 = 1327;
pub const JET_wrnTargetInstanceRunning: i32 = 578;
pub const JET_wrnUniqueKey: i32 = 345;
pub const JET_wszConfigStoreReadControl: windows_core::PCWSTR = windows_core::w!("CsReadControl");
pub const JET_wszConfigStoreRelPathSysParamDefault: windows_core::PCWSTR = windows_core::w!("SysParamDefault");
pub const JET_wszConfigStoreRelPathSysParamOverride: windows_core::PCWSTR = windows_core::w!("SysParamOverride");
pub const cColumnInfoCols: i32 = 14;
pub const cIndexInfoCols: i32 = 15;
pub const cObjectInfoCols: i32 = 9;
pub const wrnBTNotVisibleAccumulated: i32 = 353;
pub const wrnBTNotVisibleRejected: i32 = 352;
