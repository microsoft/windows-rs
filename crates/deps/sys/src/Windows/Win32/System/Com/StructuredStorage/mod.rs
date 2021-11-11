#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoGetInstanceFromFile();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CoGetInstanceFromIStorage();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn CoGetInterfaceAndReleaseStream();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateILockBytesOnHGlobal();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStreamOnHGlobal();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FmtIdToPropStgName();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreePropVariantArray();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn GetConvertStg();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn GetHGlobalFromILockBytes();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn GetHGlobalFromStream();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn OleConvertIStorageToOLESTREAM();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OleConvertIStorageToOLESTREAMEx();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn OleConvertOLESTREAMToIStorage();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OleConvertOLESTREAMToIStorageEx();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PropStgNameToFmtId();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PropVariantClear();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PropVariantCopy();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn ReadClassStg();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn ReadClassStm();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadFmtUserTypeStg();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConvertStg();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgConvertPropertyToVariant();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgConvertVariantToProperty();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgCreateDocfile();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgCreateDocfileOnILockBytes();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgCreatePropSetStg();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgCreatePropStg();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn StgCreateStorageEx();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgDeserializePropVariant();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgGetIFillLockBytesOnFile();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgGetIFillLockBytesOnILockBytes();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgIsStorageFile();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgIsStorageILockBytes();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgOpenAsyncDocfileOnIFillLockBytes();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgOpenLayoutDocfile();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgOpenPropStg();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgOpenStorage();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn StgOpenStorageEx();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgOpenStorageOnILockBytes();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn StgPropertyLengthAsVariant();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgSerializePropVariant();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StgSetTimes();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn WriteClassStg();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub fn WriteClassStm();
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteFmtUserTypeStg();
}
