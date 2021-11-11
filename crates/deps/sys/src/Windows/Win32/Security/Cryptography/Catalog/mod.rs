#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminAcquireContext();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminAcquireContext2();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminAddCatalog();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminCalcHashFromFileHandle();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminCalcHashFromFileHandle2();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`*"]
    pub fn CryptCATAdminEnumCatalogFromHash();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminPauseServiceForBackup();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminReleaseCatalogContext();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminReleaseContext();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminRemoveCatalog();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATAdminResolveCatalogPath();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATAllocSortedMemberInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATCDFClose();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATCDFEnumAttributes();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATCDFEnumCatAttributes();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATCDFEnumMembers();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATCDFOpen();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATCatalogInfoFromContext();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATClose();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATEnumerateAttr();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATEnumerateCatAttr();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATEnumerateMember();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATFreeSortedMemberInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATGetAttrInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATGetCatAttrInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATGetMemberInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATHandleFromStore();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATOpen();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATPersistStore();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATPutAttrInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATPutCatAttrInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn CryptCATPutMemberInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCATStoreFromHandle();
    #[doc = "*Required features: `Win32_Security_Cryptography_Catalog`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCatalogFile();
}
