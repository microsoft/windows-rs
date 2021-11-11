#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn BuildDisplayTable();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeIdleRoutine();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn CreateIProp();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn CreateTable();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn DeinitMapiUtil();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn DeregisterIdleRoutine();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableIdleRoutine();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FEqualNames();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn FPropCompareProp();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn FPropContainsProp();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FPropExists();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn FreePadrlist();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn FreeProws();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtAddFt();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtMulDw();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtMulDwDw();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtNegFt();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtSubFt();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtgRegisterIdleRoutine();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn HrAddColumns();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn HrAddColumnsEx();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrAllocAdviseSink();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn HrDispatchNotifications();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrGetOneProp();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn HrIStorageFromStream();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrQueryAllRows();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrSetOneProp();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn HrThisThreadAdviseSink();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LPropCompareProp();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LpValFindProp();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn MAPIDeinitIdle();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MAPIGetDefaultMalloc();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn MAPIInitIdle();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OpenStreamOnFile();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn PpropFindProp();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn PropCopyMore();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RTFSync();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScCopyNotifications();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScCopyProps();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScCountNotifications();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScCountProps();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn ScCreateConversationIndex();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScDupPropset();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn ScInitMapiUtil();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScLocalPathFromUNC();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScRelocNotifications();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScRelocProps();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScUNCFromLocalPath();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn SzFindCh();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn SzFindLastCh();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn SzFindSz();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn UFromSz();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn UlAddRef();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UlPropSize();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn UlRelease();
    #[doc = "*Required features: `Win32_System_AddressBook`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn WrapCompressedRTFStream();
    #[doc = "*Required features: `Win32_System_AddressBook`*"]
    pub fn WrapStoreEntryID();
}
