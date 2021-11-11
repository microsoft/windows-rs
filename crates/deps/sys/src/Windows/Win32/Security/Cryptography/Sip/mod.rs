#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CryptSIPAddProvider();
    fn CryptSIPCreateIndirectData();
    fn CryptSIPGetCaps();
    fn CryptSIPGetSealedDigest();
    fn CryptSIPGetSignedDataMsg();
    fn CryptSIPLoad();
    fn CryptSIPPutSignedDataMsg();
    fn CryptSIPRemoveProvider();
    fn CryptSIPRemoveSignedDataMsg();
    fn CryptSIPRetrieveSubjectGuid();
    fn CryptSIPRetrieveSubjectGuidForCatalogFile();
    fn CryptSIPVerifyIndirectData();
}
