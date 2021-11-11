#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CryptCATAdminAcquireContext();
    fn CryptCATAdminAcquireContext2();
    fn CryptCATAdminAddCatalog();
    fn CryptCATAdminCalcHashFromFileHandle();
    fn CryptCATAdminCalcHashFromFileHandle2();
    fn CryptCATAdminEnumCatalogFromHash();
    fn CryptCATAdminPauseServiceForBackup();
    fn CryptCATAdminReleaseCatalogContext();
    fn CryptCATAdminReleaseContext();
    fn CryptCATAdminRemoveCatalog();
    fn CryptCATAdminResolveCatalogPath();
    fn CryptCATAllocSortedMemberInfo();
    fn CryptCATCDFClose();
    fn CryptCATCDFEnumAttributes();
    fn CryptCATCDFEnumCatAttributes();
    fn CryptCATCDFEnumMembers();
    fn CryptCATCDFOpen();
    fn CryptCATCatalogInfoFromContext();
    fn CryptCATClose();
    fn CryptCATEnumerateAttr();
    fn CryptCATEnumerateCatAttr();
    fn CryptCATEnumerateMember();
    fn CryptCATFreeSortedMemberInfo();
    fn CryptCATGetAttrInfo();
    fn CryptCATGetCatAttrInfo();
    fn CryptCATGetMemberInfo();
    fn CryptCATHandleFromStore();
    fn CryptCATOpen();
    fn CryptCATPersistStore();
    fn CryptCATPutAttrInfo();
    fn CryptCATPutCatAttrInfo();
    fn CryptCATPutMemberInfo();
    fn CryptCATStoreFromHandle();
    fn IsCatalogFile();
}
