#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ProtectFileToEnterpriseIdentity();
    fn SrpCloseThreadNetworkContext();
    fn SrpCreateThreadNetworkContext();
    fn SrpDisablePermissiveModeFileEncryption();
    fn SrpDoesPolicyAllowAppExecution();
    fn SrpEnablePermissiveModeFileEncryption();
    fn SrpGetEnterpriseIds();
    fn SrpGetEnterprisePolicy();
    fn SrpHostingInitialize();
    fn SrpHostingTerminate();
    fn SrpIsTokenService();
    fn SrpSetTokenEnterpriseId();
    fn UnprotectFile();
}
