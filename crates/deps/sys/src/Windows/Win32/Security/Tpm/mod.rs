#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ITpmVirtualSmartCardManager();
    fn ITpmVirtualSmartCardManager2();
    fn ITpmVirtualSmartCardManager3();
    fn ITpmVirtualSmartCardManagerStatusCallback();
    fn RemoteTpmVirtualSmartCardManager();
    fn TPMVSCMGR_ERROR();
    fn TPMVSCMGR_STATUS();
    fn TPMVSC_ATTESTATION_TYPE();
    fn TPMVSC_DEFAULT_ADMIN_ALGORITHM_ID();
    fn TpmVirtualSmartCardManager();
}
