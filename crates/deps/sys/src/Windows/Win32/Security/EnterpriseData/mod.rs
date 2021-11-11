#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ENTERPRISE_DATA_POLICIES();
    fn FILE_UNPROTECT_OPTIONS();
    fn HTHREAD_NETWORK_CONTEXT();
    fn IProtectionPolicyManagerInterop();
    fn IProtectionPolicyManagerInterop2();
    fn IProtectionPolicyManagerInterop3();
    fn ProtectFileToEnterpriseIdentity();
    fn SRPHOSTING_TYPE();
    fn SRPHOSTING_VERSION();
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
