#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CertSelectionGetSerializedBlob();
    fn CryptUIDlgCertMgr();
    fn CryptUIDlgSelectCertificateFromStore();
    fn CryptUIDlgViewCertificateA();
    fn CryptUIDlgViewCertificateW();
    fn CryptUIDlgViewContext();
    fn CryptUIWizDigitalSign();
    fn CryptUIWizExport();
    fn CryptUIWizFreeDigitalSignContext();
    fn CryptUIWizImport();
}
