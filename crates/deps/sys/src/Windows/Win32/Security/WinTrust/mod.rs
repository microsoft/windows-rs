#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn OpenPersonalTrustDBDialog();
    fn OpenPersonalTrustDBDialogEx();
    fn WTHelperCertCheckValidSignature();
    fn WTHelperCertIsSelfSigned();
    fn WTHelperGetProvCertFromChain();
    fn WTHelperGetProvPrivateDataFromChain();
    fn WTHelperGetProvSignerFromChain();
    fn WTHelperProvDataFromStateData();
    fn WinVerifyTrust();
    fn WinVerifyTrustEx();
    fn WintrustAddActionID();
    fn WintrustAddDefaultForUsage();
    fn WintrustGetDefaultForUsage();
    fn WintrustGetRegPolicyFlags();
    fn WintrustLoadFunctionPointers();
    fn WintrustRemoveActionID();
    fn WintrustSetDefaultIncludePEPageHashes();
    fn WintrustSetRegPolicyFlags();
}
