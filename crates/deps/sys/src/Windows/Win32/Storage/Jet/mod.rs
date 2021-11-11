#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetAddColumnA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetAddColumnW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetAttachDatabase2A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetAttachDatabase2W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetAttachDatabaseA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetAttachDatabaseW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBackupA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBackupInstanceA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBackupInstanceW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBackupW();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetBeginExternalBackup();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBeginExternalBackupInstance();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBeginSessionA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBeginSessionW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBeginTransaction();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBeginTransaction2();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetBeginTransaction3();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCloseDatabase();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCloseFile();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCloseFileInstance();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCloseTable();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCommitTransaction();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCommitTransaction2();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCompactA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCompactW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetComputeStats();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetConfigureProcessForCrashDump();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateDatabase2A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateDatabase2W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateDatabaseA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateDatabaseW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateIndex2A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateIndex2W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateIndex3A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateIndex3W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateIndex4A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateIndex4W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateIndexA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateIndexW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateInstance2A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateInstance2W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateInstanceA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateInstanceW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateTableA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateTableColumnIndex2A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateTableColumnIndex2W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateTableColumnIndex3A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateTableColumnIndex3W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateTableColumnIndex4A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateTableColumnIndex4W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateTableColumnIndexA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetCreateTableColumnIndexW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetCreateTableW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDefragment2A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDefragment2W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDefragment3A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDefragment3W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDefragmentA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDefragmentW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDelete();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDeleteColumn2A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDeleteColumn2W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDeleteColumnA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDeleteColumnW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDeleteIndexA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDeleteIndexW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDeleteTableA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDeleteTableW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDetachDatabase2A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDetachDatabase2W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDetachDatabaseA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDetachDatabaseW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDupCursor();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetDupSession();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetEnableMultiInstanceA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetEnableMultiInstanceW();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetEndExternalBackup();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetEndExternalBackupInstance();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetEndExternalBackupInstance2();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetEndSession();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetEnumerateColumns();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetEscrowUpdate();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetExternalRestore2A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetExternalRestore2W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetExternalRestoreA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetExternalRestoreW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn JetFreeBuffer();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetGetAttachInfoA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetAttachInfoInstanceA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetAttachInfoInstanceW();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetGetAttachInfoW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetBookmark();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetColumnInfoA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetColumnInfoW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetCurrentIndexA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetCurrentIndexW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetCursorInfo();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetGetDatabaseFileInfoA();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetGetDatabaseFileInfoW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetDatabaseInfoA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetDatabaseInfoW();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetGetErrorInfoW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetIndexInfoA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetIndexInfoW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetGetInstanceInfoA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetGetInstanceInfoW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetInstanceMiscInfo();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetLS();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetLock();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetGetLogInfoA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetGetLogInfoInstance2A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetLogInfoInstance2W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetLogInfoInstanceA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetLogInfoInstanceW();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetGetLogInfoW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetObjectInfoA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetObjectInfoW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetRecordPosition();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetRecordSize();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetRecordSize2();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetSecondaryIndexBookmark();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetSessionParameter();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetSystemParameterA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetSystemParameterW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetTableColumnInfoA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetTableColumnInfoW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetTableIndexInfoA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetTableIndexInfoW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetTableInfoA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetTableInfoW();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetGetThreadStats();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetTruncateLogInfoInstanceA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetTruncateLogInfoInstanceW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGetVersion();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGotoBookmark();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGotoPosition();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGotoSecondaryIndexBookmark();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetGrowDatabase();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetIdle();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetIndexRecordCount();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetInit();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetInit2();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetInit3A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetInit3W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetIntersectIndexes();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetMakeKey();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetMove();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetOSSnapshotAbort();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetOSSnapshotEnd();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetOSSnapshotFreezeA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetOSSnapshotFreezeW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetOSSnapshotGetFreezeInfoA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetOSSnapshotGetFreezeInfoW();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetOSSnapshotPrepare();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOSSnapshotPrepareInstance();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetOSSnapshotThaw();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetOSSnapshotTruncateLog();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOSSnapshotTruncateLogInstance();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenDatabaseA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenDatabaseW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenFileA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenFileInstanceA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenFileInstanceW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenFileW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenTableA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenTableW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenTempTable();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenTempTable2();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenTempTable3();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetOpenTemporaryTable();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Foundation`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
    pub fn JetOpenTemporaryTable2();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetPrepareUpdate();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetPrereadIndexRanges();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetPrereadKeys();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetReadFile();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetReadFileInstance();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRegisterCallback();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRenameColumnA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRenameColumnW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRenameTableA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRenameTableW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetResetSessionContext();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetResetTableSequential();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetResizeDatabase();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRestore2A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRestore2W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRestoreA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRestoreInstanceA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRestoreInstanceW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRestoreW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRetrieveColumn();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRetrieveColumns();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRetrieveKey();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetRollback();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSeek();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetColumn();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetColumnDefaultValueA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetColumnDefaultValueW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetColumns();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCurrentIndex2A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCurrentIndex2W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCurrentIndex3A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCurrentIndex3W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCurrentIndex4A();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCurrentIndex4W();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCurrentIndexA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCurrentIndexW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetCursorFilter();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetDatabaseSizeA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetDatabaseSizeW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetIndexRange();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetLS();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetSessionContext();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetSessionParameter();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetSystemParameterA();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetSystemParameterW();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetSetTableSequential();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetStopBackup();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetStopBackupInstance();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetStopService();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetStopServiceInstance();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetStopServiceInstance2();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetTerm();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetTerm2();
    #[doc = "*Required features: `Win32_Storage_Jet`*"]
    pub fn JetTruncateLog();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetTruncateLogInstance();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetUnregisterCallback();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetUpdate();
    #[doc = "*Required features: `Win32_Storage_Jet`, `Win32_Storage_StructuredStorage`*"]
    #[cfg(feature = "Win32_Storage_StructuredStorage")]
    pub fn JetUpdate2();
}
