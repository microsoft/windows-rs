#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSIPAddProvider(psnewprov: *mut SIP_ADD_NEWPROVIDER) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPCreateIndirectData(psubjectinfo: *mut SIP_SUBJECTINFO, pcbindirectdata: *mut u32, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPGetCaps(psubjinfo: *const SIP_SUBJECTINFO, pcaps: *mut SIP_CAP_SET_V3) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPGetSealedDigest(psubjectinfo: *const SIP_SUBJECTINFO, psig: *const u8, dwsig: u32, pbdigest: *mut u8, pcbdigest: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPGetSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, pdwencodingtype: *mut super::CERT_QUERY_ENCODING_TYPE, dwindex: u32, pcbsigneddatamsg: *mut u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPLoad(pgsubject: *const ::windows::runtime::GUID, dwflags: u32, psipdispatch: *mut ::core::mem::ManuallyDrop<SIP_DISPATCH_INFO>) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPPutSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, dwencodingtype: super::CERT_QUERY_ENCODING_TYPE, pdwindex: *mut u32, cbsigneddatamsg: u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSIPRemoveProvider(pgprov: *mut ::windows::runtime::GUID) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPRemoveSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, dwindex: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSIPRetrieveSubjectGuid(filename: super::super::super::Foundation::PWSTR, hfilein: super::super::super::Foundation::HANDLE, pgsubject: *mut ::windows::runtime::GUID) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSIPRetrieveSubjectGuidForCatalogFile(filename: super::super::super::Foundation::PWSTR, hfilein: super::super::super::Foundation::HANDLE, pgsubject: *mut ::windows::runtime::GUID) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPVerifyIndirectData(psubjectinfo: *mut SIP_SUBJECTINFO, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL;
}
