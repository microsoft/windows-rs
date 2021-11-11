#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetAddColumnA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szcolumnname: *const i8, pcolumndef: *const JET_COLUMNDEF, pvdefault: *const ::core::ffi::c_void, cbdefault: u32, pcolumnid: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetAddColumnW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szcolumnname: *const u16, pcolumndef: *const JET_COLUMNDEF, pvdefault: *const ::core::ffi::c_void, cbdefault: u32, pcolumnid: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetAttachDatabase2A(sesid: super::StructuredStorage::JET_SESID, szfilename: *const i8, cpgdatabasesizemax: u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetAttachDatabase2W(sesid: super::StructuredStorage::JET_SESID, szfilename: *const u16, cpgdatabasesizemax: u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetAttachDatabaseA(sesid: super::StructuredStorage::JET_SESID, szfilename: *const i8, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetAttachDatabaseW(sesid: super::StructuredStorage::JET_SESID, szfilename: *const u16, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBackupA(szbackuppath: *const i8, grbit: u32, pfnstatus: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBackupInstanceA(instance: super::StructuredStorage::JET_INSTANCE, szbackuppath: *const i8, grbit: u32, pfnstatus: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBackupInstanceW(instance: super::StructuredStorage::JET_INSTANCE, szbackuppath: *const u16, grbit: u32, pfnstatus: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBackupW(szbackuppath: *const u16, grbit: u32, pfnstatus: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetBeginExternalBackup(grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBeginExternalBackupInstance(instance: super::StructuredStorage::JET_INSTANCE, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBeginSessionA(instance: super::StructuredStorage::JET_INSTANCE, psesid: *mut super::StructuredStorage::JET_SESID, szusername: *const i8, szpassword: *const i8) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBeginSessionW(instance: super::StructuredStorage::JET_INSTANCE, psesid: *mut super::StructuredStorage::JET_SESID, szusername: *const u16, szpassword: *const u16) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBeginTransaction(sesid: super::StructuredStorage::JET_SESID) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBeginTransaction2(sesid: super::StructuredStorage::JET_SESID, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBeginTransaction3(sesid: super::StructuredStorage::JET_SESID, trxid: i64, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCloseDatabase(sesid: super::StructuredStorage::JET_SESID, dbid: u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCloseFile(hffile: super::StructuredStorage::JET_HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCloseFileInstance(instance: super::StructuredStorage::JET_INSTANCE, hffile: super::StructuredStorage::JET_HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCloseTable(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCommitTransaction(sesid: super::StructuredStorage::JET_SESID, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCommitTransaction2(sesid: super::StructuredStorage::JET_SESID, grbit: u32, cmsecdurablecommit: u32, pcommitid: *mut JET_COMMIT_ID) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCompactA(sesid: super::StructuredStorage::JET_SESID, szdatabasesrc: *const i8, szdatabasedest: *const i8, pfnstatus: ::windows::runtime::RawPtr, pconvert: *const CONVERT_A, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCompactW(sesid: super::StructuredStorage::JET_SESID, szdatabasesrc: *const u16, szdatabasedest: *const u16, pfnstatus: ::windows::runtime::RawPtr, pconvert: *const CONVERT_W, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetComputeStats(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetConfigureProcessForCrashDump(grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateDatabase2A(sesid: super::StructuredStorage::JET_SESID, szfilename: *const i8, cpgdatabasesizemax: u32, pdbid: *mut u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateDatabase2W(sesid: super::StructuredStorage::JET_SESID, szfilename: *const u16, cpgdatabasesizemax: u32, pdbid: *mut u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateDatabaseA(sesid: super::StructuredStorage::JET_SESID, szfilename: *const i8, szconnect: *const i8, pdbid: *mut u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateDatabaseW(sesid: super::StructuredStorage::JET_SESID, szfilename: *const u16, szconnect: *const u16, pdbid: *mut u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateIndex2A(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pindexcreate: *const JET_INDEXCREATE_A, cindexcreate: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateIndex2W(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pindexcreate: *const JET_INDEXCREATE_W, cindexcreate: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateIndex3A(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pindexcreate: *const JET_INDEXCREATE2_A, cindexcreate: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateIndex3W(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pindexcreate: *const JET_INDEXCREATE2_W, cindexcreate: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateIndex4A(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pindexcreate: *const JET_INDEXCREATE3_A, cindexcreate: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateIndex4W(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pindexcreate: *const JET_INDEXCREATE3_W, cindexcreate: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateIndexA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const i8, grbit: u32, szkey: super::super::Foundation::PSTR, cbkey: u32, ldensity: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateIndexW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const u16, grbit: u32, szkey: super::super::Foundation::PWSTR, cbkey: u32, ldensity: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateInstance2A(pinstance: *mut super::StructuredStorage::JET_INSTANCE, szinstancename: *const i8, szdisplayname: *const i8, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateInstance2W(pinstance: *mut super::StructuredStorage::JET_INSTANCE, szinstancename: *const u16, szdisplayname: *const u16, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateInstanceA(pinstance: *mut super::StructuredStorage::JET_INSTANCE, szinstancename: *const i8) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateInstanceW(pinstance: *mut super::StructuredStorage::JET_INSTANCE, szinstancename: *const u16) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateTableA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const i8, lpages: u32, ldensity: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateTableColumnIndex2A(sesid: super::StructuredStorage::JET_SESID, dbid: u32, ptablecreate: *mut JET_TABLECREATE2_A) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateTableColumnIndex2W(sesid: super::StructuredStorage::JET_SESID, dbid: u32, ptablecreate: *mut JET_TABLECREATE2_W) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateTableColumnIndex3A(sesid: super::StructuredStorage::JET_SESID, dbid: u32, ptablecreate: *mut JET_TABLECREATE3_A) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateTableColumnIndex3W(sesid: super::StructuredStorage::JET_SESID, dbid: u32, ptablecreate: *mut JET_TABLECREATE3_W) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateTableColumnIndex4A(sesid: super::StructuredStorage::JET_SESID, dbid: u32, ptablecreate: *mut JET_TABLECREATE4_A) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateTableColumnIndex4W(sesid: super::StructuredStorage::JET_SESID, dbid: u32, ptablecreate: *mut JET_TABLECREATE4_W) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateTableColumnIndexA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, ptablecreate: *mut JET_TABLECREATE_A) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateTableColumnIndexW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, ptablecreate: *mut JET_TABLECREATE_W) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateTableW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const u16, lpages: u32, ldensity: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDefragment2A(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const i8, pcpasses: *mut u32, pcseconds: *mut u32, callback: ::windows::runtime::RawPtr, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDefragment2W(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const u16, pcpasses: *mut u32, pcseconds: *mut u32, callback: ::windows::runtime::RawPtr, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDefragment3A(sesid: super::StructuredStorage::JET_SESID, szdatabasename: *const i8, sztablename: *const i8, pcpasses: *mut u32, pcseconds: *mut u32, callback: ::windows::runtime::RawPtr, pvcontext: *const ::core::ffi::c_void, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDefragment3W(sesid: super::StructuredStorage::JET_SESID, szdatabasename: *const u16, sztablename: *const u16, pcpasses: *mut u32, pcseconds: *mut u32, callback: ::windows::runtime::RawPtr, pvcontext: *const ::core::ffi::c_void, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDefragmentA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const i8, pcpasses: *mut u32, pcseconds: *mut u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDefragmentW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const u16, pcpasses: *mut u32, pcseconds: *mut u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDelete(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDeleteColumn2A(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szcolumnname: *const i8, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDeleteColumn2W(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szcolumnname: *const u16, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDeleteColumnA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szcolumnname: *const i8) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDeleteColumnW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szcolumnname: *const u16) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDeleteIndexA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const i8) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDeleteIndexW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const u16) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDeleteTableA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const i8) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDeleteTableW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const u16) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDetachDatabase2A(sesid: super::StructuredStorage::JET_SESID, szfilename: *const i8, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDetachDatabase2W(sesid: super::StructuredStorage::JET_SESID, szfilename: *const u16, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDetachDatabaseA(sesid: super::StructuredStorage::JET_SESID, szfilename: *const i8) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDetachDatabaseW(sesid: super::StructuredStorage::JET_SESID, szfilename: *const u16) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDupCursor(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, ptableid: *mut super::StructuredStorage::JET_TABLEID, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDupSession(sesid: super::StructuredStorage::JET_SESID, psesid: *mut super::StructuredStorage::JET_SESID) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetEnableMultiInstanceA(psetsysparam: *const JET_SETSYSPARAM_A, csetsysparam: u32, pcsetsucceed: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetEnableMultiInstanceW(psetsysparam: *const JET_SETSYSPARAM_W, csetsysparam: u32, pcsetsucceed: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetEndExternalBackup() -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetEndExternalBackupInstance(instance: super::StructuredStorage::JET_INSTANCE) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetEndExternalBackupInstance2(instance: super::StructuredStorage::JET_INSTANCE, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetEndSession(sesid: super::StructuredStorage::JET_SESID, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetEnumerateColumns(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, cenumcolumnid: u32, rgenumcolumnid: *const JET_ENUMCOLUMNID, pcenumcolumn: *mut u32, prgenumcolumn: *mut *mut JET_ENUMCOLUMN, pfnrealloc: ::windows::runtime::RawPtr, pvrealloccontext: *const ::core::ffi::c_void, cbdatamost: u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetEscrowUpdate(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, columnid: u32, pv: *const ::core::ffi::c_void, cbmax: u32, pvold: *mut ::core::ffi::c_void, cboldmax: u32, pcboldactual: *mut u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetExternalRestore2A(szcheckpointfilepath: *const i8, szlogpath: *const i8, rgrstmap: *const JET_RSTMAP_A, crstfilemap: i32, szbackuplogpath: *const i8, ploginfo: *mut JET_LOGINFO_A, sztargetinstancename: *const i8, sztargetinstancelogpath: *const i8, sztargetinstancecheckpointpath: *const i8, pfn: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetExternalRestore2W(szcheckpointfilepath: *const u16, szlogpath: *const u16, rgrstmap: *const JET_RSTMAP_W, crstfilemap: i32, szbackuplogpath: *const u16, ploginfo: *mut JET_LOGINFO_W, sztargetinstancename: *const u16, sztargetinstancelogpath: *const u16, sztargetinstancecheckpointpath: *const u16, pfn: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetExternalRestoreA(szcheckpointfilepath: *const i8, szlogpath: *const i8, rgrstmap: *const JET_RSTMAP_A, crstfilemap: i32, szbackuplogpath: *const i8, genlow: i32, genhigh: i32, pfn: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetExternalRestoreW(szcheckpointfilepath: *const u16, szlogpath: *const u16, rgrstmap: *const JET_RSTMAP_W, crstfilemap: i32, szbackuplogpath: *const u16, genlow: i32, genhigh: i32, pfn: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn JetFreeBuffer(pbbuf: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetGetAttachInfoA(szzdatabases: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetAttachInfoInstanceA(instance: super::StructuredStorage::JET_INSTANCE, szzdatabases: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetAttachInfoInstanceW(instance: super::StructuredStorage::JET_INSTANCE, szzdatabases: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetGetAttachInfoW(wszzdatabases: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetBookmark(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvbookmark: *mut ::core::ffi::c_void, cbmax: u32, pcbactual: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetColumnInfoA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const i8, pcolumnnameorid: *const i8, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetColumnInfoW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const u16, pwcolumnnameorid: *const u16, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetCurrentIndexA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *mut i8, cbindexname: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetCurrentIndexW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *mut u16, cbindexname: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetCursorInfo(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetGetDatabaseFileInfoA(szdatabasename: *const i8, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetGetDatabaseFileInfoW(szdatabasename: *const u16, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetDatabaseInfoA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetDatabaseInfoW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetGetErrorInfoW(pvcontext: *const ::core::ffi::c_void, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetIndexInfoA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const i8, szindexname: *const i8, pvresult: *mut ::core::ffi::c_void, cbresult: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetIndexInfoW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const u16, szindexname: *const u16, pvresult: *mut ::core::ffi::c_void, cbresult: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetGetInstanceInfoA(pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_A) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetGetInstanceInfoW(pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_W) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetInstanceMiscInfo(instance: super::StructuredStorage::JET_INSTANCE, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetLS(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pls: *mut JET_LS, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetLock(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetGetLogInfoA(szzlogs: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetGetLogInfoInstance2A(instance: super::StructuredStorage::JET_INSTANCE, szzlogs: *mut i8, cbmax: u32, pcbactual: *mut u32, ploginfo: *mut JET_LOGINFO_A) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetLogInfoInstance2W(instance: super::StructuredStorage::JET_INSTANCE, wszzlogs: *mut u16, cbmax: u32, pcbactual: *mut u32, ploginfo: *mut JET_LOGINFO_W) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetLogInfoInstanceA(instance: super::StructuredStorage::JET_INSTANCE, szzlogs: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetLogInfoInstanceW(instance: super::StructuredStorage::JET_INSTANCE, wszzlogs: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetGetLogInfoW(szzlogs: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetObjectInfoA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, objtyp: u32, szcontainername: *const i8, szobjectname: *const i8, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetObjectInfoW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, objtyp: u32, szcontainername: *const u16, szobjectname: *const u16, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetRecordPosition(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, precpos: *mut JET_RECPOS, cbrecpos: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetRecordSize(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, precsize: *mut JET_RECSIZE, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetRecordSize2(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, precsize: *mut JET_RECSIZE2, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetSecondaryIndexBookmark(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvsecondarykey: *mut ::core::ffi::c_void, cbsecondarykeymax: u32, pcbsecondarykeyactual: *mut u32, pvprimarybookmark: *mut ::core::ffi::c_void, cbprimarybookmarkmax: u32, pcbprimarybookmarkactual: *mut u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetSessionParameter(sesid: super::StructuredStorage::JET_SESID, sesparamid: u32, pvparam: *mut ::core::ffi::c_void, cbparammax: u32, pcbparamactual: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetSystemParameterA(instance: super::StructuredStorage::JET_INSTANCE, sesid: super::StructuredStorage::JET_SESID, paramid: u32, plparam: *mut super::StructuredStorage::JET_API_PTR, szparam: *mut i8, cbmax: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetSystemParameterW(instance: super::StructuredStorage::JET_INSTANCE, sesid: super::StructuredStorage::JET_SESID, paramid: u32, plparam: *mut super::StructuredStorage::JET_API_PTR, szparam: *mut u16, cbmax: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetTableColumnInfoA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szcolumnname: *const i8, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetTableColumnInfoW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szcolumnname: *const u16, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetTableIndexInfoA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const i8, pvresult: *mut ::core::ffi::c_void, cbresult: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetTableIndexInfoW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const u16, pvresult: *mut ::core::ffi::c_void, cbresult: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetTableInfoA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetTableInfoW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetGetThreadStats(pvresult: *mut ::core::ffi::c_void, cbmax: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetTruncateLogInfoInstanceA(instance: super::StructuredStorage::JET_INSTANCE, szzlogs: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetTruncateLogInfoInstanceW(instance: super::StructuredStorage::JET_INSTANCE, wszzlogs: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetVersion(sesid: super::StructuredStorage::JET_SESID, pwversion: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGotoBookmark(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvbookmark: *const ::core::ffi::c_void, cbbookmark: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGotoPosition(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, precpos: *const JET_RECPOS) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGotoSecondaryIndexBookmark(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvsecondarykey: *const ::core::ffi::c_void, cbsecondarykey: u32, pvprimarybookmark: *const ::core::ffi::c_void, cbprimarybookmark: u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGrowDatabase(sesid: super::StructuredStorage::JET_SESID, dbid: u32, cpg: u32, pcpgreal: *const u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetIdle(sesid: super::StructuredStorage::JET_SESID, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetIndexRecordCount(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pcrec: *mut u32, crecmax: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetInit(pinstance: *mut super::StructuredStorage::JET_INSTANCE) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetInit2(pinstance: *mut super::StructuredStorage::JET_INSTANCE, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetInit3A(pinstance: *mut super::StructuredStorage::JET_INSTANCE, prstinfo: *const ::core::mem::ManuallyDrop<JET_RSTINFO_A>, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetInit3W(pinstance: *mut super::StructuredStorage::JET_INSTANCE, prstinfo: *const ::core::mem::ManuallyDrop<JET_RSTINFO_W>, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetIntersectIndexes(sesid: super::StructuredStorage::JET_SESID, rgindexrange: *const JET_INDEXRANGE, cindexrange: u32, precordlist: *mut JET_RECORDLIST, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetMakeKey(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvdata: *const ::core::ffi::c_void, cbdata: u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetMove(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, crow: i32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetOSSnapshotAbort(snapid: JET_OSSNAPID, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetOSSnapshotEnd(snapid: JET_OSSNAPID, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetOSSnapshotFreezeA(snapid: JET_OSSNAPID, pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_A, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetOSSnapshotFreezeW(snapid: JET_OSSNAPID, pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_W, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetOSSnapshotGetFreezeInfoA(snapid: JET_OSSNAPID, pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_A, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetOSSnapshotGetFreezeInfoW(snapid: JET_OSSNAPID, pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_W, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetOSSnapshotPrepare(psnapid: *mut JET_OSSNAPID, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOSSnapshotPrepareInstance(snapid: JET_OSSNAPID, instance: super::StructuredStorage::JET_INSTANCE, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetOSSnapshotThaw(snapid: JET_OSSNAPID, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetOSSnapshotTruncateLog(snapid: JET_OSSNAPID, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOSSnapshotTruncateLogInstance(snapid: JET_OSSNAPID, instance: super::StructuredStorage::JET_INSTANCE, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenDatabaseA(sesid: super::StructuredStorage::JET_SESID, szfilename: *const i8, szconnect: *const i8, pdbid: *mut u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenDatabaseW(sesid: super::StructuredStorage::JET_SESID, szfilename: *const u16, szconnect: *const u16, pdbid: *mut u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenFileA(szfilename: *const i8, phffile: *mut super::StructuredStorage::JET_HANDLE, pulfilesizelow: *mut u32, pulfilesizehigh: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenFileInstanceA(instance: super::StructuredStorage::JET_INSTANCE, szfilename: *const i8, phffile: *mut super::StructuredStorage::JET_HANDLE, pulfilesizelow: *mut u32, pulfilesizehigh: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenFileInstanceW(instance: super::StructuredStorage::JET_INSTANCE, szfilename: *const u16, phffile: *mut super::StructuredStorage::JET_HANDLE, pulfilesizelow: *mut u32, pulfilesizehigh: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenFileW(szfilename: *const u16, phffile: *mut super::StructuredStorage::JET_HANDLE, pulfilesizelow: *mut u32, pulfilesizehigh: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenTableA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const i8, pvparameters: *const ::core::ffi::c_void, cbparameters: u32, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenTableW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const u16, pvparameters: *const ::core::ffi::c_void, cbparameters: u32, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenTempTable(sesid: super::StructuredStorage::JET_SESID, prgcolumndef: *const JET_COLUMNDEF, ccolumn: u32, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID, prgcolumnid: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenTempTable2(sesid: super::StructuredStorage::JET_SESID, prgcolumndef: *const JET_COLUMNDEF, ccolumn: u32, lcid: u32, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID, prgcolumnid: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenTempTable3(sesid: super::StructuredStorage::JET_SESID, prgcolumndef: *const JET_COLUMNDEF, ccolumn: u32, pidxunicode: *const JET_UNICODEINDEX, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID, prgcolumnid: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenTemporaryTable(sesid: super::StructuredStorage::JET_SESID, popentemporarytable: *const JET_OPENTEMPORARYTABLE) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetOpenTemporaryTable2(sesid: super::StructuredStorage::JET_SESID, popentemporarytable: *const JET_OPENTEMPORARYTABLE2) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetPrepareUpdate(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, prep: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetPrereadIndexRanges(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, rgindexranges: *const JET_INDEX_RANGE, cindexranges: u32, pcrangespreread: *mut u32, rgcolumnidpreread: *const u32, ccolumnidpreread: u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetPrereadKeys(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, rgpvkeys: *const *const ::core::ffi::c_void, rgcbkeys: *const u32, ckeys: i32, pckeyspreread: *mut i32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetReadFile(hffile: super::StructuredStorage::JET_HANDLE, pv: *mut ::core::ffi::c_void, cb: u32, pcbactual: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetReadFileInstance(instance: super::StructuredStorage::JET_INSTANCE, hffile: super::StructuredStorage::JET_HANDLE, pv: *mut ::core::ffi::c_void, cb: u32, pcbactual: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRegisterCallback(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, cbtyp: u32, pcallback: ::windows::runtime::RawPtr, pvcontext: *const ::core::ffi::c_void, phcallbackid: *const super::StructuredStorage::JET_HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRenameColumnA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szname: *const i8, sznamenew: *const i8, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRenameColumnW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szname: *const u16, sznamenew: *const u16, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRenameTableA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, szname: *const i8, sznamenew: *const i8) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRenameTableW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, szname: *const u16, sznamenew: *const u16) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetResetSessionContext(sesid: super::StructuredStorage::JET_SESID) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetResetTableSequential(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetResizeDatabase(sesid: super::StructuredStorage::JET_SESID, dbid: u32, cpgtarget: u32, pcpgactual: *mut u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRestore2A(sz: *const i8, szdest: *const i8, pfn: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRestore2W(sz: *const u16, szdest: *const u16, pfn: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRestoreA(szsource: *const i8, pfn: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRestoreInstanceA(instance: super::StructuredStorage::JET_INSTANCE, sz: *const i8, szdest: *const i8, pfn: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRestoreInstanceW(instance: super::StructuredStorage::JET_INSTANCE, sz: *const u16, szdest: *const u16, pfn: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRestoreW(szsource: *const u16, pfn: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRetrieveColumn(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, columnid: u32, pvdata: *mut ::core::ffi::c_void, cbdata: u32, pcbactual: *mut u32, grbit: u32, pretinfo: *mut JET_RETINFO) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRetrieveColumns(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pretrievecolumn: *mut JET_RETRIEVECOLUMN, cretrievecolumn: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRetrieveKey(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvkey: *mut ::core::ffi::c_void, cbmax: u32, pcbactual: *mut u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRollback(sesid: super::StructuredStorage::JET_SESID, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSeek(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetColumn(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, columnid: u32, pvdata: *const ::core::ffi::c_void, cbdata: u32, grbit: u32, psetinfo: *const JET_SETINFO) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetColumnDefaultValueA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const i8, szcolumnname: *const i8, pvdata: *const ::core::ffi::c_void, cbdata: u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetColumnDefaultValueW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const u16, szcolumnname: *const u16, pvdata: *const ::core::ffi::c_void, cbdata: u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetColumns(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, psetcolumn: *const JET_SETCOLUMN, csetcolumn: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCurrentIndex2A(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const i8, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCurrentIndex2W(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const u16, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCurrentIndex3A(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const i8, grbit: u32, itagsequence: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCurrentIndex3W(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const u16, grbit: u32, itagsequence: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCurrentIndex4A(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const i8, pindexid: *const JET_INDEXID, grbit: u32, itagsequence: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCurrentIndex4W(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const u16, pindexid: *const JET_INDEXID, grbit: u32, itagsequence: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCurrentIndexA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const i8) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCurrentIndexW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const u16) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCursorFilter(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, rgcolumnfilters: *const JET_INDEX_COLUMN, ccolumnfilters: u32, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetDatabaseSizeA(sesid: super::StructuredStorage::JET_SESID, szdatabasename: *const i8, cpg: u32, pcpgreal: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetDatabaseSizeW(sesid: super::StructuredStorage::JET_SESID, szdatabasename: *const u16, cpg: u32, pcpgreal: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetIndexRange(sesid: super::StructuredStorage::JET_SESID, tableidsrc: super::StructuredStorage::JET_TABLEID, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetLS(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, ls: JET_LS, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetSessionContext(sesid: super::StructuredStorage::JET_SESID, ulcontext: super::StructuredStorage::JET_API_PTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetSessionParameter(sesid: super::StructuredStorage::JET_SESID, sesparamid: u32, pvparam: *const ::core::ffi::c_void, cbparam: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetSystemParameterA(pinstance: *mut super::StructuredStorage::JET_INSTANCE, sesid: super::StructuredStorage::JET_SESID, paramid: u32, lparam: super::StructuredStorage::JET_API_PTR, szparam: *const i8) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetSystemParameterW(pinstance: *mut super::StructuredStorage::JET_INSTANCE, sesid: super::StructuredStorage::JET_SESID, paramid: u32, lparam: super::StructuredStorage::JET_API_PTR, szparam: *const u16) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetTableSequential(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetStopBackup() -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetStopBackupInstance(instance: super::StructuredStorage::JET_INSTANCE) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetStopService() -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetStopServiceInstance(instance: super::StructuredStorage::JET_INSTANCE) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetStopServiceInstance2(instance: super::StructuredStorage::JET_INSTANCE, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetTerm(instance: super::StructuredStorage::JET_INSTANCE) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetTerm2(instance: super::StructuredStorage::JET_INSTANCE, grbit: u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetTruncateLog() -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetTruncateLogInstance(instance: super::StructuredStorage::JET_INSTANCE) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetUnregisterCallback(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, cbtyp: u32, hcallbackid: super::StructuredStorage::JET_HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetUpdate(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvbookmark: *mut ::core::ffi::c_void, cbbookmark: u32, pcbactual: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetUpdate2(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvbookmark: *mut ::core::ffi::c_void, cbbookmark: u32, pcbactual: *mut u32, grbit: u32) -> i32;
}
