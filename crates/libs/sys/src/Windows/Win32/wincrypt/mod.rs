#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertAddCRLContextToStore(hcertstore : HCERTSTORE, pcrlcontext : *const CRL_CONTEXT, dwadddisposition : u32, ppstorecontext : *mut PCCRL_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertAddCRLLinkToStore(hcertstore : HCERTSTORE, pcrlcontext : *const CRL_CONTEXT, dwadddisposition : u32, ppstorecontext : *mut PCCRL_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertAddCTLContextToStore(hcertstore : HCERTSTORE, pctlcontext : *const CTL_CONTEXT, dwadddisposition : u32, ppstorecontext : *mut PCCTL_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertAddCTLLinkToStore(hcertstore : HCERTSTORE, pctlcontext : *const CTL_CONTEXT, dwadddisposition : u32, ppstorecontext : *mut PCCTL_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertAddCertificateContextToStore(hcertstore : HCERTSTORE, pcertcontext : *const CERT_CONTEXT, dwadddisposition : u32, ppstorecontext : *mut PCCERT_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertAddCertificateLinkToStore(hcertstore : HCERTSTORE, pcertcontext : *const CERT_CONTEXT, dwadddisposition : u32, ppstorecontext : *mut PCCERT_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertAddEncodedCRLToStore(hcertstore : HCERTSTORE, dwcertencodingtype : u32, pbcrlencoded : *const u8, cbcrlencoded : u32, dwadddisposition : u32, ppcrlcontext : *mut PCCRL_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertAddEncodedCTLToStore(hcertstore : HCERTSTORE, dwmsgandcertencodingtype : u32, pbctlencoded : *const u8, cbctlencoded : u32, dwadddisposition : u32, ppctlcontext : *mut PCCTL_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertAddEncodedCertificateToStore(hcertstore : HCERTSTORE, dwcertencodingtype : u32, pbcertencoded : *const u8, cbcertencoded : u32, dwadddisposition : u32, ppcertcontext : *mut PCCERT_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertAddEncodedCertificateToSystemStoreA(szcertstorename : windows_sys::core::PCSTR, pbcertencoded : *const u8, cbcertencoded : u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertAddEncodedCertificateToSystemStoreW(szcertstorename : windows_sys::core::PCWSTR, pbcertencoded : *const u8, cbcertencoded : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertAddEnhancedKeyUsageIdentifier(pcertcontext : *const CERT_CONTEXT, pszusageidentifier : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertAddRefServerOcspResponse(hserverocspresponse : HCERT_SERVER_OCSP_RESPONSE));
windows_link::link!("crypt32.dll" "system" fn CertAddRefServerOcspResponseContext(pserverocspresponsecontext : *const CERT_SERVER_OCSP_RESPONSE_CONTEXT));
windows_link::link!("crypt32.dll" "system" fn CertAddSerializedElementToStore(hcertstore : HCERTSTORE, pbelement : *const u8, cbelement : u32, dwadddisposition : u32, dwflags : u32, dwcontexttypeflags : u32, pdwcontexttype : *mut u32, ppvcontext : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertAddStoreToCollection(hcollectionstore : HCERTSTORE, hsiblingstore : HCERTSTORE, dwupdateflags : u32, dwpriority : u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertAlgIdToOID(dwalgid : u32) -> windows_sys::core::PCSTR);
windows_link::link!("crypt32.dll" "system" fn CertCloseServerOcspResponse(hserverocspresponse : HCERT_SERVER_OCSP_RESPONSE, dwflags : u32));
windows_link::link!("crypt32.dll" "system" fn CertCloseStore(hcertstore : HCERTSTORE, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertCompareCertificate(dwcertencodingtype : u32, pcertid1 : *const CERT_INFO, pcertid2 : *const CERT_INFO) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertCompareCertificateName(dwcertencodingtype : u32, pcertname1 : *const CRYPT_INTEGER_BLOB, pcertname2 : *const CRYPT_INTEGER_BLOB) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertCompareIntegerBlob(pint1 : *const CRYPT_INTEGER_BLOB, pint2 : *const CRYPT_INTEGER_BLOB) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertComparePublicKeyInfo(dwcertencodingtype : u32, ppublickey1 : *const CERT_PUBLIC_KEY_INFO, ppublickey2 : *const CERT_PUBLIC_KEY_INFO) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertControlStore(hcertstore : HCERTSTORE, dwflags : u32, dwctrltype : u32, pvctrlpara : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertCreateCRLContext(dwcertencodingtype : u32, pbcrlencoded : *const u8, cbcrlencoded : u32) -> PCCRL_CONTEXT);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertCreateCTLContext(dwmsgandcertencodingtype : u32, pbctlencoded : *const u8, cbctlencoded : u32) -> PCCTL_CONTEXT);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertCreateCTLEntryFromCertificateContextProperties(pcertcontext : *const CERT_CONTEXT, coptattr : u32, rgoptattr : *const CRYPT_ATTRIBUTE, dwflags : u32, pvreserved : *const core::ffi::c_void, pctlentry : *mut CTL_ENTRY, pcbctlentry : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("crypt32.dll" "system" fn CertCreateCertificateChainEngine(pconfig : *const CERT_CHAIN_ENGINE_CONFIG, phchainengine : *mut HCERTCHAINENGINE) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertCreateCertificateContext(dwcertencodingtype : u32, pbcertencoded : *const u8, cbcertencoded : u32) -> PCCERT_CONTEXT);
windows_link::link!("crypt32.dll" "system" fn CertCreateContext(dwcontexttype : u32, dwencodingtype : u32, pbencoded : *const u8, cbencoded : u32, dwflags : u32, pcreatepara : *const CERT_CREATE_CONTEXT_PARA) -> *const core::ffi::c_void);
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
windows_link::link!("crypt32.dll" "system" fn CertCreateSelfSignCertificate(hcryptprovorncryptkey : HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, psubjectissuerblob : *const CRYPT_INTEGER_BLOB, dwflags : u32, pkeyprovinfo : *const CRYPT_KEY_PROV_INFO, psignaturealgorithm : *const CRYPT_ALGORITHM_IDENTIFIER, pstarttime : *const super::SYSTEMTIME, pendtime : *const super::SYSTEMTIME, pextensions : *const CERT_EXTENSIONS) -> PCCERT_CONTEXT);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertDeleteCRLFromStore(pcrlcontext : *const CRL_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertDeleteCTLFromStore(pctlcontext : *const CTL_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertDeleteCertificateFromStore(pcertcontext : *const CERT_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertDuplicateCRLContext(pcrlcontext : *const CRL_CONTEXT) -> PCCRL_CONTEXT);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertDuplicateCTLContext(pctlcontext : *const CTL_CONTEXT) -> PCCTL_CONTEXT);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertDuplicateCertificateChain(pchaincontext : *const CERT_CHAIN_CONTEXT) -> PCCERT_CHAIN_CONTEXT);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertDuplicateCertificateContext(pcertcontext : *const CERT_CONTEXT) -> PCCERT_CONTEXT);
windows_link::link!("crypt32.dll" "system" fn CertDuplicateStore(hcertstore : HCERTSTORE) -> HCERTSTORE);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertEnumCRLContextProperties(pcrlcontext : *const CRL_CONTEXT, dwpropid : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertEnumCRLsInStore(hcertstore : HCERTSTORE, pprevcrlcontext : *const CRL_CONTEXT) -> PCCRL_CONTEXT);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertEnumCTLContextProperties(pctlcontext : *const CTL_CONTEXT, dwpropid : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertEnumCTLsInStore(hcertstore : HCERTSTORE, pprevctlcontext : *const CTL_CONTEXT) -> PCCTL_CONTEXT);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertEnumCertificateContextProperties(pcertcontext : *const CERT_CONTEXT, dwpropid : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertEnumCertificatesInStore(hcertstore : HCERTSTORE, pprevcertcontext : *const CERT_CONTEXT) -> PCCERT_CONTEXT);
windows_link::link!("crypt32.dll" "system" fn CertEnumPhysicalStore(pvsystemstore : *const core::ffi::c_void, dwflags : u32, pvarg : *mut core::ffi::c_void, pfnenum : PFN_CERT_ENUM_PHYSICAL_STORE) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertEnumSubjectInSortedCTL(pctlcontext : *const CTL_CONTEXT, ppvnextsubject : *mut *mut core::ffi::c_void, psubjectidentifier : *mut CRYPT_INTEGER_BLOB, pencodedattributes : *mut CRYPT_INTEGER_BLOB) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertEnumSystemStore(dwflags : u32, pvsystemstorelocationpara : *const core::ffi::c_void, pvarg : *mut core::ffi::c_void, pfnenum : PFN_CERT_ENUM_SYSTEM_STORE) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertEnumSystemStoreLocation(dwflags : u32, pvarg : *mut core::ffi::c_void, pfnenum : PFN_CERT_ENUM_SYSTEM_STORE_LOCATION) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertFindAttribute(pszobjid : windows_sys::core::PCSTR, cattr : u32, rgattr : *const CRYPT_ATTRIBUTE) -> PCRYPT_ATTRIBUTE);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertFindCRLInStore(hcertstore : HCERTSTORE, dwcertencodingtype : u32, dwfindflags : u32, dwfindtype : u32, pvfindpara : *const core::ffi::c_void, pprevcrlcontext : *const CRL_CONTEXT) -> PCCRL_CONTEXT);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertFindCTLInStore(hcertstore : HCERTSTORE, dwmsgandcertencodingtype : u32, dwfindflags : u32, dwfindtype : u32, pvfindpara : *const core::ffi::c_void, pprevctlcontext : *const CTL_CONTEXT) -> PCCTL_CONTEXT);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertFindCertificateInCRL(pcert : *const CERT_CONTEXT, pcrlcontext : *const CRL_CONTEXT, dwflags : u32, pvreserved : *const core::ffi::c_void, ppcrlentry : *mut PCRL_ENTRY) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertFindCertificateInStore(hcertstore : HCERTSTORE, dwcertencodingtype : u32, dwfindflags : u32, dwfindtype : u32, pvfindpara : *const core::ffi::c_void, pprevcertcontext : *const CERT_CONTEXT) -> PCCERT_CONTEXT);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertFindChainInStore(hcertstore : HCERTSTORE, dwcertencodingtype : u32, dwfindflags : u32, dwfindtype : u32, pvfindpara : *const core::ffi::c_void, pprevchaincontext : *const CERT_CHAIN_CONTEXT) -> PCCERT_CHAIN_CONTEXT);
windows_link::link!("crypt32.dll" "system" fn CertFindExtension(pszobjid : windows_sys::core::PCSTR, cextensions : u32, rgextensions : *const CERT_EXTENSION) -> PCERT_EXTENSION);
windows_link::link!("crypt32.dll" "system" fn CertFindRDNAttr(pszobjid : windows_sys::core::PCSTR, pname : *const CERT_NAME_INFO) -> PCERT_RDN_ATTR);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertFindSubjectInCTL(dwencodingtype : u32, dwsubjecttype : u32, pvsubject : *const core::ffi::c_void, pctlcontext : *const CTL_CONTEXT, dwflags : u32) -> PCTL_ENTRY);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertFindSubjectInSortedCTL(psubjectidentifier : *const CRYPT_INTEGER_BLOB, pctlcontext : *const CTL_CONTEXT, dwflags : u32, pvreserved : *const core::ffi::c_void, pencodedattributes : *mut CRYPT_INTEGER_BLOB) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertFreeCRLContext(pcrlcontext : *const CRL_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertFreeCTLContext(pctlcontext : *const CTL_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertFreeCertificateChain(pchaincontext : *const CERT_CHAIN_CONTEXT));
#[cfg(feature = "winnt")]
windows_link::link!("crypt32.dll" "system" fn CertFreeCertificateChainEngine(hchainengine : HCERTCHAINENGINE));
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertFreeCertificateChainList(prgpselection : *const PCCERT_CHAIN_CONTEXT));
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertFreeCertificateContext(pcertcontext : *const CERT_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertFreeServerOcspResponseContext(pserverocspresponsecontext : *const CERT_SERVER_OCSP_RESPONSE_CONTEXT));
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertGetCRLContextProperty(pcrlcontext : *const CRL_CONTEXT, dwpropid : u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertGetCRLFromStore(hcertstore : HCERTSTORE, pissuercontext : *const CERT_CONTEXT, pprevcrlcontext : *const CRL_CONTEXT, pdwflags : *mut u32) -> PCCRL_CONTEXT);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertGetCTLContextProperty(pctlcontext : *const CTL_CONTEXT, dwpropid : u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CertGetCertificateChain(hchainengine : HCERTCHAINENGINE, pcertcontext : *const CERT_CONTEXT, ptime : *const super::FILETIME, hadditionalstore : HCERTSTORE, pchainpara : *const CERT_CHAIN_PARA, dwflags : u32, pvreserved : *const core::ffi::c_void, ppchaincontext : *mut PCCERT_CHAIN_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertGetCertificateContextProperty(pcertcontext : *const CERT_CONTEXT, dwpropid : u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertGetEnhancedKeyUsage(pcertcontext : *const CERT_CONTEXT, dwflags : u32, pusage : *mut CTL_USAGE, pcbusage : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertGetIntendedKeyUsage(dwcertencodingtype : u32, pcertinfo : *const CERT_INFO, pbkeyusage : *mut u8, cbkeyusage : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertGetIssuerCertificateFromStore(hcertstore : HCERTSTORE, psubjectcontext : *const CERT_CONTEXT, pprevissuercontext : *const CERT_CONTEXT, pdwflags : *mut u32) -> PCCERT_CONTEXT);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertGetNameStringA(pcertcontext : *const CERT_CONTEXT, dwtype : u32, dwflags : u32, pvtypepara : *const core::ffi::c_void, psznamestring : windows_sys::core::PSTR, cchnamestring : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertGetNameStringW(pcertcontext : *const CERT_CONTEXT, dwtype : u32, dwflags : u32, pvtypepara : *const core::ffi::c_void, psznamestring : windows_sys::core::PWSTR, cchnamestring : u32) -> u32);
windows_link::link!("crypt32.dll" "system" fn CertGetPublicKeyLength(dwcertencodingtype : u32, ppublickey : *const CERT_PUBLIC_KEY_INFO) -> u32);
windows_link::link!("crypt32.dll" "system" fn CertGetServerOcspResponseContext(hserverocspresponse : HCERT_SERVER_OCSP_RESPONSE, dwflags : u32, pvreserved : *const core::ffi::c_void) -> PCCERT_SERVER_OCSP_RESPONSE_CONTEXT);
windows_link::link!("crypt32.dll" "system" fn CertGetStoreProperty(hcertstore : HCERTSTORE, dwpropid : u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertGetSubjectCertificateFromStore(hcertstore : HCERTSTORE, dwcertencodingtype : u32, pcertid : *const CERT_INFO) -> PCCERT_CONTEXT);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertGetValidUsages(ccerts : u32, rghcerts : *const PCCERT_CONTEXT, cnumoids : *mut i32, rghoids : *mut windows_sys::core::PSTR, pcboids : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertIsRDNAttrsInCertificateName(dwcertencodingtype : u32, dwflags : u32, pcertname : *const CRYPT_INTEGER_BLOB, prdn : *const CERT_RDN) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertIsStrongHashToSign(pstrongsignpara : *const CERT_STRONG_SIGN_PARA, pwszcnghashalgid : windows_sys::core::PCWSTR, psigningcert : *const CERT_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertIsValidCRLForCertificate(pcert : *const CERT_CONTEXT, pcrl : *const CRL_CONTEXT, dwflags : u32, pvreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertIsWeakHash(dwhashusetype : u32, pwszcnghashalgid : windows_sys::core::PCWSTR, dwchainflags : u32, psignerchaincontext : *const CERT_CHAIN_CONTEXT, ptimestamp : *const super::FILETIME, pwszfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertNameToStrA(dwcertencodingtype : u32, pname : *const CRYPT_INTEGER_BLOB, dwstrtype : u32, psz : windows_sys::core::PSTR, csz : u32) -> u32);
windows_link::link!("crypt32.dll" "system" fn CertNameToStrW(dwcertencodingtype : u32, pname : *const CRYPT_INTEGER_BLOB, dwstrtype : u32, psz : windows_sys::core::PWSTR, csz : u32) -> u32);
windows_link::link!("crypt32.dll" "system" fn CertOIDToAlgId(pszobjid : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertOpenServerOcspResponse(pchaincontext : *const CERT_CHAIN_CONTEXT, dwflags : u32, popenpara : *const CERT_SERVER_OCSP_RESPONSE_OPEN_PARA) -> HCERT_SERVER_OCSP_RESPONSE);
windows_link::link!("crypt32.dll" "system" fn CertOpenStore(lpszstoreprovider : windows_sys::core::PCSTR, dwencodingtype : u32, hcryptprov : HCRYPTPROV_LEGACY, dwflags : u32, pvpara : *const core::ffi::c_void) -> HCERTSTORE);
windows_link::link!("crypt32.dll" "system" fn CertOpenSystemStoreA(hprov : HCRYPTPROV_LEGACY, szsubsystemprotocol : windows_sys::core::PCSTR) -> HCERTSTORE);
windows_link::link!("crypt32.dll" "system" fn CertOpenSystemStoreW(hprov : HCRYPTPROV_LEGACY, szsubsystemprotocol : windows_sys::core::PCWSTR) -> HCERTSTORE);
windows_link::link!("crypt32.dll" "system" fn CertRDNValueToStrA(dwvaluetype : u32, pvalue : *const CRYPT_INTEGER_BLOB, psz : windows_sys::core::PSTR, csz : u32) -> u32);
windows_link::link!("crypt32.dll" "system" fn CertRDNValueToStrW(dwvaluetype : u32, pvalue : *const CRYPT_INTEGER_BLOB, psz : windows_sys::core::PWSTR, csz : u32) -> u32);
windows_link::link!("crypt32.dll" "system" fn CertRegisterPhysicalStore(pvsystemstore : *const core::ffi::c_void, dwflags : u32, pwszstorename : windows_sys::core::PCWSTR, pstoreinfo : *const CERT_PHYSICAL_STORE_INFO, pvreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertRegisterSystemStore(pvsystemstore : *const core::ffi::c_void, dwflags : u32, pstoreinfo : *const CERT_SYSTEM_STORE_INFO, pvreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertRemoveEnhancedKeyUsageIdentifier(pcertcontext : *const CERT_CONTEXT, pszusageidentifier : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertRemoveStoreFromCollection(hcollectionstore : HCERTSTORE, hsiblingstore : HCERTSTORE));
#[cfg(feature = "winnt")]
windows_link::link!("crypt32.dll" "system" fn CertResyncCertificateChainEngine(hchainengine : HCERTCHAINENGINE) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertRetrieveLogoOrBiometricInfo(pcertcontext : *const CERT_CONTEXT, lpszlogoorbiometrictype : windows_sys::core::PCSTR, dwretrievalflags : u32, dwtimeout : u32, dwflags : u32, pvreserved : *const core::ffi::c_void, ppbdata : *mut *mut u8, pcbdata : *mut u32, ppwszmimetype : *mut windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertSaveStore(hcertstore : HCERTSTORE, dwencodingtype : u32, dwsaveas : u32, dwsaveto : u32, pvsavetopara : *mut core::ffi::c_void, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CertSelectCertificateChains(pselectioncontext : *const windows_sys::core::GUID, dwflags : u32, pchainparameters : *const CERT_SELECT_CHAIN_PARA, ccriteria : u32, rgpcriteria : *const CERT_SELECT_CRITERIA, hstore : HCERTSTORE, pcselection : *mut u32, pprgpselection : *mut *mut PCCERT_CHAIN_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertSerializeCRLStoreElement(pcrlcontext : *const CRL_CONTEXT, dwflags : u32, pbelement : *mut u8, pcbelement : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertSerializeCTLStoreElement(pctlcontext : *const CTL_CONTEXT, dwflags : u32, pbelement : *mut u8, pcbelement : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertSerializeCertificateStoreElement(pcertcontext : *const CERT_CONTEXT, dwflags : u32, pbelement : *mut u8, pcbelement : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertSetCRLContextProperty(pcrlcontext : *const CRL_CONTEXT, dwpropid : u32, dwflags : u32, pvdata : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertSetCTLContextProperty(pctlcontext : *const CTL_CONTEXT, dwpropid : u32, dwflags : u32, pvdata : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertSetCertificateContextPropertiesFromCTLEntry(pcertcontext : *const CERT_CONTEXT, pctlentry : *const CTL_ENTRY, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertSetCertificateContextProperty(pcertcontext : *const CERT_CONTEXT, dwpropid : u32, dwflags : u32, pvdata : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertSetEnhancedKeyUsage(pcertcontext : *const CERT_CONTEXT, pusage : *const CTL_USAGE) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertSetStoreProperty(hcertstore : HCERTSTORE, dwpropid : u32, dwflags : u32, pvdata : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertStrToNameA(dwcertencodingtype : u32, pszx500 : windows_sys::core::PCSTR, dwstrtype : u32, pvreserved : *const core::ffi::c_void, pbencoded : *mut u8, pcbencoded : *mut u32, ppszerror : *mut windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertStrToNameW(dwcertencodingtype : u32, pszx500 : windows_sys::core::PCWSTR, dwstrtype : u32, pvreserved : *const core::ffi::c_void, pbencoded : *mut u8, pcbencoded : *mut u32, ppszerror : *mut windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertUnregisterPhysicalStore(pvsystemstore : *const core::ffi::c_void, dwflags : u32, pwszstorename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CertUnregisterSystemStore(pvsystemstore : *const core::ffi::c_void, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertVerifyCRLRevocation(dwcertencodingtype : u32, pcertid : *const CERT_INFO, ccrlinfo : u32, rgpcrlinfo : *const PCRL_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertVerifyCRLTimeValidity(ptimetoverify : *const super::FILETIME, pcrlinfo : *const CRL_INFO) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertVerifyCTLUsage(dwencodingtype : u32, dwsubjecttype : u32, pvsubject : *const core::ffi::c_void, psubjectusage : *const CTL_USAGE, dwflags : u32, pverifyusagepara : *const CTL_VERIFY_USAGE_PARA, pverifyusagestatus : *mut CTL_VERIFY_USAGE_STATUS) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertVerifyCertificateChainPolicy(pszpolicyoid : windows_sys::core::PCSTR, pchaincontext : *const CERT_CHAIN_CONTEXT, ppolicypara : *const CERT_CHAIN_POLICY_PARA, ppolicystatus : *mut CERT_CHAIN_POLICY_STATUS) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertVerifyRevocation(dwencodingtype : u32, dwrevtype : u32, ccontext : u32, rgpvcontext : *const *const core::ffi::c_void, dwflags : u32, prevpara : *const CERT_REVOCATION_PARA, prevstatus : *mut CERT_REVOCATION_STATUS) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertVerifySubjectCertificateContext(psubject : *const CERT_CONTEXT, pissuer : *const CERT_CONTEXT, pdwflags : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertVerifyTimeValidity(ptimetoverify : *const super::FILETIME, pcertinfo : *const CERT_INFO) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CertVerifyValidityNesting(psubjectinfo : *const CERT_INFO, pissuerinfo : *const CERT_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CryptAcquireCertificatePrivateKey(pcert : *const CERT_CONTEXT, dwflags : u32, pvparameters : *const core::ffi::c_void, phcryptprovorncryptkey : *mut HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, pdwkeyspec : *mut u32, pfcallerfreeprovorncryptkey : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptAcquireContextA(phprov : *mut HCRYPTPROV, szcontainer : windows_sys::core::PCSTR, szprovider : windows_sys::core::PCSTR, dwprovtype : u32, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptAcquireContextW(phprov : *mut HCRYPTPROV, szcontainer : windows_sys::core::PCWSTR, szprovider : windows_sys::core::PCWSTR, dwprovtype : u32, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptBinaryToStringA(pbbinary : *const u8, cbbinary : u32, dwflags : u32, pszstring : windows_sys::core::PSTR, pcchstring : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptBinaryToStringW(pbbinary : *const u8, cbbinary : u32, dwflags : u32, pszstring : windows_sys::core::PWSTR, pcchstring : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("crypt32.dll" "system" fn CryptCloseAsyncHandle(hasync : HCRYPTASYNC) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptContextAddRef(hprov : HCRYPTPROV, pdwreserved : *const u32, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("crypt32.dll" "system" fn CryptCreateAsyncHandle(dwflags : u32, phasync : *mut super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptCreateHash(hprov : HCRYPTPROV, algid : ALG_ID, hkey : HCRYPTKEY, dwflags : u32, phhash : *mut HCRYPTHASH) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptCreateKeyIdentifierFromCSP(dwcertencodingtype : u32, pszpubkeyoid : windows_sys::core::PCSTR, ppubkeystruc : *const PUBLICKEYSTRUC, cbpubkeystruc : u32, dwflags : u32, pvreserved : *const core::ffi::c_void, pbhash : *mut u8, pcbhash : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CryptDecodeMessage(dwmsgtypeflags : u32, pdecryptpara : *const CRYPT_DECRYPT_MESSAGE_PARA, pverifypara : *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex : u32, pbencodedblob : *const u8, cbencodedblob : u32, dwprevinnercontenttype : u32, pdwmsgtype : *mut u32, pdwinnercontenttype : *mut u32, pbdecoded : *mut u8, pcbdecoded : *mut u32, ppxchgcert : *mut PCCERT_CONTEXT, ppsignercert : *mut PCCERT_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptDecodeObject(dwcertencodingtype : u32, lpszstructtype : windows_sys::core::PCSTR, pbencoded : *const u8, cbencoded : u32, dwflags : u32, pvstructinfo : *mut core::ffi::c_void, pcbstructinfo : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptDecodeObjectEx(dwcertencodingtype : u32, lpszstructtype : windows_sys::core::PCSTR, pbencoded : *const u8, cbencoded : u32, dwflags : u32, pdecodepara : *const CRYPT_DECODE_PARA, pvstructinfo : *mut core::ffi::c_void, pcbstructinfo : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptDecrypt(hkey : HCRYPTKEY, hhash : HCRYPTHASH, r#final : windows_sys::core::BOOL, dwflags : u32, pbdata : *mut u8, pdwdatalen : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CryptDecryptAndVerifyMessageSignature(pdecryptpara : *const CRYPT_DECRYPT_MESSAGE_PARA, pverifypara : *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex : u32, pbencryptedblob : *const u8, cbencryptedblob : u32, pbdecrypted : *mut u8, pcbdecrypted : *mut u32, ppxchgcert : *mut PCCERT_CONTEXT, ppsignercert : *mut PCCERT_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CryptDecryptMessage(pdecryptpara : *const CRYPT_DECRYPT_MESSAGE_PARA, pbencryptedblob : *const u8, cbencryptedblob : u32, pbdecrypted : *mut u8, pcbdecrypted : *mut u32, ppxchgcert : *mut PCCERT_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptDeriveKey(hprov : HCRYPTPROV, algid : ALG_ID, hbasedata : HCRYPTHASH, dwflags : u32, phkey : *mut HCRYPTKEY) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptDestroyHash(hhash : HCRYPTHASH) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptDestroyKey(hkey : HCRYPTKEY) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptDuplicateHash(hhash : HCRYPTHASH, pdwreserved : *const u32, dwflags : u32, phhash : *mut HCRYPTHASH) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptDuplicateKey(hkey : HCRYPTKEY, pdwreserved : *const u32, dwflags : u32, phkey : *mut HCRYPTKEY) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptEncodeObject(dwcertencodingtype : u32, lpszstructtype : windows_sys::core::PCSTR, pvstructinfo : *const core::ffi::c_void, pbencoded : *mut u8, pcbencoded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptEncodeObjectEx(dwcertencodingtype : u32, lpszstructtype : windows_sys::core::PCSTR, pvstructinfo : *const core::ffi::c_void, dwflags : u32, pencodepara : *const CRYPT_ENCODE_PARA, pvencoded : *mut core::ffi::c_void, pcbencoded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptEncrypt(hkey : HCRYPTKEY, hhash : HCRYPTHASH, r#final : windows_sys::core::BOOL, dwflags : u32, pbdata : *mut u8, pdwdatalen : *mut u32, dwbuflen : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CryptEncryptMessage(pencryptpara : *const CRYPT_ENCRYPT_MESSAGE_PARA, crecipientcert : u32, rgprecipientcert : *const PCCERT_CONTEXT, pbtobeencrypted : *const u8, cbtobeencrypted : u32, pbencryptedblob : *mut u8, pcbencryptedblob : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptEnumKeyIdentifierProperties(pkeyidentifier : *const CRYPT_HASH_BLOB, dwpropid : u32, dwflags : u32, pwszcomputername : windows_sys::core::PCWSTR, pvreserved : *const core::ffi::c_void, pvarg : *mut core::ffi::c_void, pfnenum : PFN_CRYPT_ENUM_KEYID_PROP) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptEnumOIDFunction(dwencodingtype : u32, pszfuncname : windows_sys::core::PCSTR, pszoid : windows_sys::core::PCSTR, dwflags : u32, pvarg : *mut core::ffi::c_void, pfnenumoidfunc : PFN_CRYPT_ENUM_OID_FUNC) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptEnumOIDInfo(dwgroupid : u32, dwflags : u32, pvarg : *mut core::ffi::c_void, pfnenumoidinfo : PFN_CRYPT_ENUM_OID_INFO) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptEnumProviderTypesA(dwindex : u32, pdwreserved : *const u32, dwflags : u32, pdwprovtype : *mut u32, sztypename : windows_sys::core::PSTR, pcbtypename : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptEnumProviderTypesW(dwindex : u32, pdwreserved : *const u32, dwflags : u32, pdwprovtype : *mut u32, sztypename : windows_sys::core::PWSTR, pcbtypename : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptEnumProvidersA(dwindex : u32, pdwreserved : *const u32, dwflags : u32, pdwprovtype : *mut u32, szprovname : windows_sys::core::PSTR, pcbprovname : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptEnumProvidersW(dwindex : u32, pdwreserved : *const u32, dwflags : u32, pdwprovtype : *mut u32, szprovname : windows_sys::core::PWSTR, pcbprovname : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptExportKey(hkey : HCRYPTKEY, hexpkey : HCRYPTKEY, dwblobtype : u32, dwflags : u32, pbdata : *mut u8, pdwdatalen : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptExportPKCS8(hcryptprov : HCRYPTPROV, dwkeyspec : u32, pszprivatekeyobjid : windows_sys::core::PCSTR, dwflags : u32, pvauxinfo : *const core::ffi::c_void, pbprivatekeyblob : *mut u8, pcbprivatekeyblob : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptExportPublicKeyInfo(hcryptprovorncryptkey : HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec : u32, dwcertencodingtype : u32, pinfo : *mut CERT_PUBLIC_KEY_INFO, pcbinfo : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptExportPublicKeyInfoEx(hcryptprovorncryptkey : HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec : u32, dwcertencodingtype : u32, pszpublickeyobjid : windows_sys::core::PCSTR, dwflags : u32, pvauxinfo : *const core::ffi::c_void, pinfo : *mut CERT_PUBLIC_KEY_INFO, pcbinfo : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "bcrypt")]
windows_link::link!("crypt32.dll" "system" fn CryptExportPublicKeyInfoFromBCryptKeyHandle(hbcryptkey : super::BCRYPT_KEY_HANDLE, dwcertencodingtype : u32, pszpublickeyobjid : windows_sys::core::PCSTR, dwflags : u32, pvauxinfo : *const core::ffi::c_void, pinfo : *mut CERT_PUBLIC_KEY_INFO, pcbinfo : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CryptFindCertificateKeyProvInfo(pcert : *const CERT_CONTEXT, dwflags : u32, pvreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptFindLocalizedName(pwszcryptname : windows_sys::core::PCWSTR) -> windows_sys::core::PCWSTR);
windows_link::link!("crypt32.dll" "system" fn CryptFindOIDInfo(dwkeytype : u32, pvkey : *const core::ffi::c_void, dwgroupid : u32) -> PCCRYPT_OID_INFO);
windows_link::link!("crypt32.dll" "system" fn CryptFormatObject(dwcertencodingtype : u32, dwformattype : u32, dwformatstrtype : u32, pformatstruct : *const core::ffi::c_void, lpszstructtype : windows_sys::core::PCSTR, pbencoded : *const u8, cbencoded : u32, pbformat : *mut core::ffi::c_void, pcbformat : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptFreeOIDFunctionAddress(hfuncaddr : HCRYPTOIDFUNCADDR, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptGenKey(hprov : HCRYPTPROV, algid : ALG_ID, dwflags : u32, phkey : *mut HCRYPTKEY) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptGenRandom(hprov : HCRYPTPROV, dwlen : u32, pbbuffer : *mut u8) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("crypt32.dll" "system" fn CryptGetAsyncParam(hasync : HCRYPTASYNC, pszparamoid : windows_sys::core::PCSTR, ppvparam : *mut *mut core::ffi::c_void, ppfnfree : *mut PFN_CRYPT_ASYNC_PARAM_FREE_FUNC) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptGetDefaultOIDDllList(hfuncset : HCRYPTOIDFUNCSET, dwencodingtype : u32, pwszdlllist : *mut u16, pcchdlllist : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptGetDefaultOIDFunctionAddress(hfuncset : HCRYPTOIDFUNCSET, dwencodingtype : u32, pwszdll : windows_sys::core::PCWSTR, dwflags : u32, ppvfuncaddr : *mut *mut core::ffi::c_void, phfuncaddr : *mut HCRYPTOIDFUNCADDR) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptGetDefaultProviderA(dwprovtype : u32, pdwreserved : *const u32, dwflags : u32, pszprovname : windows_sys::core::PSTR, pcbprovname : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptGetDefaultProviderW(dwprovtype : u32, pdwreserved : *const u32, dwflags : u32, pszprovname : windows_sys::core::PWSTR, pcbprovname : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptGetHashParam(hhash : HCRYPTHASH, dwparam : u32, pbdata : *mut u8, pdwdatalen : *mut u32, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptGetKeyIdentifierProperty(pkeyidentifier : *const CRYPT_HASH_BLOB, dwpropid : u32, dwflags : u32, pwszcomputername : windows_sys::core::PCWSTR, pvreserved : *const core::ffi::c_void, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptGetKeyParam(hkey : HCRYPTKEY, dwparam : u32, pbdata : *mut u8, pdwdatalen : *mut u32, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptGetMessageCertificates(dwmsgandcertencodingtype : u32, hcryptprov : HCRYPTPROV_LEGACY, dwflags : u32, pbsignedblob : *const u8, cbsignedblob : u32) -> HCERTSTORE);
windows_link::link!("crypt32.dll" "system" fn CryptGetMessageSignerCount(dwmsgencodingtype : u32, pbsignedblob : *const u8, cbsignedblob : u32) -> i32);
windows_link::link!("crypt32.dll" "system" fn CryptGetOIDFunctionAddress(hfuncset : HCRYPTOIDFUNCSET, dwencodingtype : u32, pszoid : windows_sys::core::PCSTR, dwflags : u32, ppvfuncaddr : *mut *mut core::ffi::c_void, phfuncaddr : *mut HCRYPTOIDFUNCADDR) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptGetOIDFunctionValue(dwencodingtype : u32, pszfuncname : windows_sys::core::PCSTR, pszoid : windows_sys::core::PCSTR, pwszvaluename : windows_sys::core::PCWSTR, pdwvaluetype : *mut u32, pbvaluedata : *mut u8, pcbvaluedata : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("cryptnet.dll" "system" fn CryptGetObjectUrl(pszurloid : windows_sys::core::PCSTR, pvpara : *const core::ffi::c_void, dwflags : u32, purlarray : *mut CRYPT_URL_ARRAY, pcburlarray : *mut u32, purlinfo : *mut CRYPT_URL_INFO, pcburlinfo : *mut u32, pvreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptGetProvParam(hprov : HCRYPTPROV, dwparam : u32, pbdata : *mut u8, pdwdatalen : *mut u32, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptGetUserKey(hprov : HCRYPTPROV, dwkeyspec : u32, phuserkey : *mut HCRYPTKEY) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptHashCertificate(hcryptprov : HCRYPTPROV_LEGACY, algid : ALG_ID, dwflags : u32, pbencoded : *const u8, cbencoded : u32, pbcomputedhash : *mut u8, pcbcomputedhash : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptHashCertificate2(pwszcnghashalgid : windows_sys::core::PCWSTR, dwflags : u32, pvreserved : *const core::ffi::c_void, pbencoded : *const u8, cbencoded : u32, pbcomputedhash : *mut u8, pcbcomputedhash : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptHashData(hhash : HCRYPTHASH, pbdata : *const u8, dwdatalen : u32, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptHashMessage(phashpara : *const CRYPT_HASH_MESSAGE_PARA, fdetachedhash : windows_sys::core::BOOL, ctobehashed : u32, rgpbtobehashed : *const *const u8, rgcbtobehashed : *const u32, pbhashedblob : *mut u8, pcbhashedblob : *mut u32, pbcomputedhash : *mut u8, pcbcomputedhash : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptHashPublicKeyInfo(hcryptprov : HCRYPTPROV_LEGACY, algid : ALG_ID, dwflags : u32, dwcertencodingtype : u32, pinfo : *const CERT_PUBLIC_KEY_INFO, pbcomputedhash : *mut u8, pcbcomputedhash : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptHashSessionKey(hhash : HCRYPTHASH, hkey : HCRYPTKEY, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptHashToBeSigned(hcryptprov : HCRYPTPROV_LEGACY, dwcertencodingtype : u32, pbencoded : *const u8, cbencoded : u32, pbcomputedhash : *mut u8, pcbcomputedhash : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptImportKey(hprov : HCRYPTPROV, pbdata : *const u8, dwdatalen : u32, hpubkey : HCRYPTKEY, dwflags : u32, phkey : *mut HCRYPTKEY) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptImportPKCS8(sprivatekeyandparams : CRYPT_PKCS8_IMPORT_PARAMS, dwflags : u32, phcryptprov : *mut HCRYPTPROV, pvauxinfo : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptImportPublicKeyInfo(hcryptprov : HCRYPTPROV, dwcertencodingtype : u32, pinfo : *const CERT_PUBLIC_KEY_INFO, phkey : *mut HCRYPTKEY) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptImportPublicKeyInfoEx(hcryptprov : HCRYPTPROV, dwcertencodingtype : u32, pinfo : *const CERT_PUBLIC_KEY_INFO, aikeyalg : ALG_ID, dwflags : u32, pvauxinfo : *const core::ffi::c_void, phkey : *mut HCRYPTKEY) -> windows_sys::core::BOOL);
#[cfg(feature = "bcrypt")]
windows_link::link!("crypt32.dll" "system" fn CryptImportPublicKeyInfoEx2(dwcertencodingtype : u32, pinfo : *const CERT_PUBLIC_KEY_INFO, dwflags : u32, pvauxinfo : *const core::ffi::c_void, phkey : *mut super::BCRYPT_KEY_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptInitOIDFunctionSet(pszfuncname : windows_sys::core::PCSTR, dwflags : u32) -> HCRYPTOIDFUNCSET);
windows_link::link!("cryptnet.dll" "system" fn CryptInstallCancelRetrieval(pfncancel : PFN_CRYPT_CANCEL_RETRIEVAL, pvarg : *const core::ffi::c_void, dwflags : u32, pvreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptInstallDefaultContext(hcryptprov : HCRYPTPROV, dwdefaulttype : u32, pvdefaultpara : *const core::ffi::c_void, dwflags : u32, pvreserved : *const core::ffi::c_void, phdefaultcontext : *mut HCRYPTDEFAULTCONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CryptInstallOIDFunctionAddress(hmodule : super::HMODULE, dwencodingtype : u32, pszfuncname : windows_sys::core::PCSTR, cfuncentry : u32, rgfuncentry : *const CRYPT_OID_FUNC_ENTRY, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptMemAlloc(cbsize : u32) -> *mut core::ffi::c_void);
windows_link::link!("crypt32.dll" "system" fn CryptMemFree(pv : *const core::ffi::c_void));
windows_link::link!("crypt32.dll" "system" fn CryptMemRealloc(pv : *const core::ffi::c_void, cbsize : u32) -> *mut core::ffi::c_void);
windows_link::link!("crypt32.dll" "system" fn CryptMsgCalculateEncodedLength(dwmsgencodingtype : u32, dwflags : u32, dwmsgtype : u32, pvmsgencodeinfo : *const core::ffi::c_void, pszinnercontentobjid : windows_sys::core::PCSTR, cbdata : u32) -> u32);
windows_link::link!("crypt32.dll" "system" fn CryptMsgClose(hcryptmsg : HCRYPTMSG) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptMsgControl(hcryptmsg : HCRYPTMSG, dwflags : u32, dwctrltype : u32, pvctrlpara : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
windows_link::link!("crypt32.dll" "system" fn CryptMsgCountersign(hcryptmsg : HCRYPTMSG, dwindex : u32, ccountersigners : u32, rgcountersigners : *const CMSG_SIGNER_ENCODE_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
windows_link::link!("crypt32.dll" "system" fn CryptMsgCountersignEncoded(dwencodingtype : u32, pbsignerinfo : *const u8, cbsignerinfo : u32, ccountersigners : u32, rgcountersigners : *const CMSG_SIGNER_ENCODE_INFO, pbcountersignature : *mut u8, pcbcountersignature : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptMsgDuplicate(hcryptmsg : HCRYPTMSG) -> HCRYPTMSG);
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
windows_link::link!("crypt32.dll" "system" fn CryptMsgEncodeAndSignCTL(dwmsgencodingtype : u32, pctlinfo : *const CTL_INFO, psigninfo : *const CMSG_SIGNED_ENCODE_INFO, dwflags : u32, pbencoded : *mut u8, pcbencoded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CryptMsgGetAndVerifySigner(hcryptmsg : HCRYPTMSG, csignerstore : u32, rghsignerstore : *const HCERTSTORE, dwflags : u32, ppsigner : *mut PCCERT_CONTEXT, pdwsignerindex : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptMsgGetParam(hcryptmsg : HCRYPTMSG, dwparamtype : u32, dwindex : u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CryptMsgOpenToDecode(dwmsgencodingtype : u32, dwflags : u32, dwmsgtype : u32, hcryptprov : HCRYPTPROV_LEGACY, precipientinfo : *const CERT_INFO, pstreaminfo : *const CMSG_STREAM_INFO) -> HCRYPTMSG);
windows_link::link!("crypt32.dll" "system" fn CryptMsgOpenToEncode(dwmsgencodingtype : u32, dwflags : u32, dwmsgtype : u32, pvmsgencodeinfo : *const core::ffi::c_void, pszinnercontentobjid : windows_sys::core::PCSTR, pstreaminfo : *const CMSG_STREAM_INFO) -> HCRYPTMSG);
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
windows_link::link!("crypt32.dll" "system" fn CryptMsgSignCTL(dwmsgencodingtype : u32, pbctlcontent : *const u8, cbctlcontent : u32, psigninfo : *const CMSG_SIGNED_ENCODE_INFO, dwflags : u32, pbencoded : *mut u8, pcbencoded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptMsgUpdate(hcryptmsg : HCRYPTMSG, pbdata : *const u8, cbdata : u32, ffinal : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CryptMsgVerifyCountersignatureEncoded(hcryptprov : HCRYPTPROV_LEGACY, dwencodingtype : u32, pbsignerinfo : *const u8, cbsignerinfo : u32, pbsignerinfocountersignature : *const u8, cbsignerinfocountersignature : u32, pcicountersigner : *const CERT_INFO) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptMsgVerifyCountersignatureEncodedEx(hcryptprov : HCRYPTPROV_LEGACY, dwencodingtype : u32, pbsignerinfo : *const u8, cbsignerinfo : u32, pbsignerinfocountersignature : *const u8, cbsignerinfocountersignature : u32, dwsignertype : u32, pvsigner : *const core::ffi::c_void, dwflags : u32, pvextra : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptQueryObject(dwobjecttype : u32, pvobject : *const core::ffi::c_void, dwexpectedcontenttypeflags : u32, dwexpectedformattypeflags : u32, dwflags : u32, pdwmsgandcertencodingtype : *mut u32, pdwcontenttype : *mut u32, pdwformattype : *mut u32, phcertstore : *mut HCERTSTORE, phmsg : *mut HCRYPTMSG, ppvcontext : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptRegisterDefaultOIDFunction(dwencodingtype : u32, pszfuncname : windows_sys::core::PCSTR, dwindex : u32, pwszdll : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptRegisterOIDFunction(dwencodingtype : u32, pszfuncname : windows_sys::core::PCSTR, pszoid : windows_sys::core::PCSTR, pwszdll : windows_sys::core::PCWSTR, pszoverridefuncname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptRegisterOIDInfo(pinfo : *const CRYPT_OID_INFO, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptReleaseContext(hprov : HCRYPTPROV, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("cryptnet.dll" "system" fn CryptRetrieveObjectByUrlA(pszurl : windows_sys::core::PCSTR, pszobjectoid : windows_sys::core::PCSTR, dwretrievalflags : u32, dwtimeout : u32, ppvobject : *mut *mut core::ffi::c_void, hasyncretrieve : HCRYPTASYNC, pcredentials : *const CRYPT_CREDENTIALS, pvverify : *const core::ffi::c_void, pauxinfo : *mut CRYPT_RETRIEVE_AUX_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("cryptnet.dll" "system" fn CryptRetrieveObjectByUrlW(pszurl : windows_sys::core::PCWSTR, pszobjectoid : windows_sys::core::PCSTR, dwretrievalflags : u32, dwtimeout : u32, ppvobject : *mut *mut core::ffi::c_void, hasyncretrieve : HCRYPTASYNC, pcredentials : *const CRYPT_CREDENTIALS, pvverify : *const core::ffi::c_void, pauxinfo : *mut CRYPT_RETRIEVE_AUX_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CryptRetrieveTimeStamp(wszurl : windows_sys::core::PCWSTR, dwretrievalflags : u32, dwtimeout : u32, pszhashid : windows_sys::core::PCSTR, ppara : *const CRYPT_TIMESTAMP_PARA, pbdata : *const u8, cbdata : u32, pptscontext : *mut PCRYPT_TIMESTAMP_CONTEXT, pptssigner : *mut PCCERT_CONTEXT, phstore : *mut HCERTSTORE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("crypt32.dll" "system" fn CryptSetAsyncParam(hasync : HCRYPTASYNC, pszparamoid : windows_sys::core::PCSTR, pvparam : *const core::ffi::c_void, pfnfree : PFN_CRYPT_ASYNC_PARAM_FREE_FUNC) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptSetHashParam(hhash : HCRYPTHASH, dwparam : u32, pbdata : *const u8, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptSetKeyIdentifierProperty(pkeyidentifier : *const CRYPT_HASH_BLOB, dwpropid : u32, dwflags : u32, pwszcomputername : windows_sys::core::PCWSTR, pvreserved : *const core::ffi::c_void, pvdata : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptSetKeyParam(hkey : HCRYPTKEY, dwparam : u32, pbdata : *const u8, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptSetOIDFunctionValue(dwencodingtype : u32, pszfuncname : windows_sys::core::PCSTR, pszoid : windows_sys::core::PCSTR, pwszvaluename : windows_sys::core::PCWSTR, dwvaluetype : u32, pbvaluedata : *const u8, cbvaluedata : u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptSetProvParam(hprov : HCRYPTPROV, dwparam : u32, pbdata : *const u8, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptSetProviderA(pszprovname : windows_sys::core::PCSTR, dwprovtype : u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptSetProviderExA(pszprovname : windows_sys::core::PCSTR, dwprovtype : u32, pdwreserved : *const u32, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptSetProviderExW(pszprovname : windows_sys::core::PCWSTR, dwprovtype : u32, pdwreserved : *const u32, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptSetProviderW(pszprovname : windows_sys::core::PCWSTR, dwprovtype : u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptSignAndEncodeCertificate(hcryptprovorncryptkey : HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec : u32, dwcertencodingtype : u32, lpszstructtype : windows_sys::core::PCSTR, pvstructinfo : *const core::ffi::c_void, psignaturealgorithm : *const CRYPT_ALGORITHM_IDENTIFIER, pvhashauxinfo : *const core::ffi::c_void, pbencoded : *mut u8, pcbencoded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CryptSignAndEncryptMessage(psignpara : *const CRYPT_SIGN_MESSAGE_PARA, pencryptpara : *const CRYPT_ENCRYPT_MESSAGE_PARA, crecipientcert : u32, rgprecipientcert : *const PCCERT_CONTEXT, pbtobesignedandencrypted : *const u8, cbtobesignedandencrypted : u32, pbsignedandencryptedblob : *mut u8, pcbsignedandencryptedblob : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptSignCertificate(hcryptprovorncryptkey : HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec : u32, dwcertencodingtype : u32, pbencodedtobesigned : *const u8, cbencodedtobesigned : u32, psignaturealgorithm : *const CRYPT_ALGORITHM_IDENTIFIER, pvhashauxinfo : *const core::ffi::c_void, pbsignature : *mut u8, pcbsignature : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptSignHashA(hhash : HCRYPTHASH, dwkeyspec : u32, szdescription : windows_sys::core::PCSTR, dwflags : u32, pbsignature : *mut u8, pdwsiglen : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptSignHashW(hhash : HCRYPTHASH, dwkeyspec : u32, szdescription : windows_sys::core::PCWSTR, dwflags : u32, pbsignature : *mut u8, pdwsiglen : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CryptSignMessage(psignpara : *const CRYPT_SIGN_MESSAGE_PARA, fdetachedsignature : windows_sys::core::BOOL, ctobesigned : u32, rgpbtobesigned : *const *const u8, rgcbtobesigned : *const u32, pbsignedblob : *mut u8, pcbsignedblob : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "ncrypt")]
windows_link::link!("crypt32.dll" "system" fn CryptSignMessageWithKey(psignpara : *const CRYPT_KEY_SIGN_MESSAGE_PARA, pbtobesigned : *const u8, cbtobesigned : u32, pbsignedblob : *mut u8, pcbsignedblob : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptStringToBinaryA(pszstring : windows_sys::core::PCSTR, cchstring : u32, dwflags : u32, pbbinary : *mut u8, pcbbinary : *mut u32, pdwskip : *mut u32, pdwflags : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptStringToBinaryW(pszstring : windows_sys::core::PCWSTR, cchstring : u32, dwflags : u32, pbbinary : *mut u8, pcbbinary : *mut u32, pdwskip : *mut u32, pdwflags : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("cryptnet.dll" "system" fn CryptUninstallCancelRetrieval(dwflags : u32, pvreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptUninstallDefaultContext(hdefaultcontext : HCRYPTDEFAULTCONTEXT, dwflags : u32, pvreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptUnregisterDefaultOIDFunction(dwencodingtype : u32, pszfuncname : windows_sys::core::PCSTR, pwszdll : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptUnregisterOIDFunction(dwencodingtype : u32, pszfuncname : windows_sys::core::PCSTR, pszoid : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptUnregisterOIDInfo(pinfo : *const CRYPT_OID_INFO) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptVerifyCertificateSignature(hcryptprov : HCRYPTPROV_LEGACY, dwcertencodingtype : u32, pbencoded : *const u8, cbencoded : u32, ppublickey : *const CERT_PUBLIC_KEY_INFO) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptVerifyCertificateSignatureEx(hcryptprov : HCRYPTPROV_LEGACY, dwcertencodingtype : u32, dwsubjecttype : u32, pvsubject : *const core::ffi::c_void, dwissuertype : u32, pvissuer : *const core::ffi::c_void, dwflags : u32, pvextra : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptVerifyDetachedMessageHash(phashpara : *const CRYPT_HASH_MESSAGE_PARA, pbdetachedhashblob : *const u8, cbdetachedhashblob : u32, ctobehashed : u32, rgpbtobehashed : *const *const u8, rgcbtobehashed : *const u32, pbcomputedhash : *mut u8, pcbcomputedhash : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CryptVerifyDetachedMessageSignature(pverifypara : *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex : u32, pbdetachedsignblob : *const u8, cbdetachedsignblob : u32, ctobesigned : u32, rgpbtobesigned : *const *const u8, rgcbtobesigned : *const u32, ppsignercert : *mut PCCERT_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptVerifyMessageHash(phashpara : *const CRYPT_HASH_MESSAGE_PARA, pbhashedblob : *const u8, cbhashedblob : u32, pbtobehashed : *mut u8, pcbtobehashed : *mut u32, pbcomputedhash : *mut u8, pcbcomputedhash : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CryptVerifyMessageSignature(pverifypara : *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex : u32, pbsignedblob : *const u8, cbsignedblob : u32, pbdecoded : *mut u8, pcbdecoded : *mut u32, ppsignercert : *mut PCCERT_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptVerifyMessageSignatureWithKey(pverifypara : *const CRYPT_KEY_VERIFY_MESSAGE_PARA, ppublickeyinfo : *const CERT_PUBLIC_KEY_INFO, pbsignedblob : *const u8, cbsignedblob : u32, pbdecoded : *mut u8, pcbdecoded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptVerifySignatureA(hhash : HCRYPTHASH, pbsignature : *const u8, dwsiglen : u32, hpubkey : HCRYPTKEY, szdescription : windows_sys::core::PCSTR, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CryptVerifySignatureW(hhash : HCRYPTHASH, pbsignature : *const u8, dwsiglen : u32, hpubkey : HCRYPTKEY, szdescription : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("crypt32.dll" "system" fn CryptVerifyTimeStampSignature(pbtscontentinfo : *const u8, cbtscontentinfo : u32, pbdata : *const u8, cbdata : u32, hadditionalstore : HCERTSTORE, pptscontext : *mut PCRYPT_TIMESTAMP_CONTEXT, pptssigner : *mut PCCERT_CONTEXT, phstore : *mut HCERTSTORE) -> windows_sys::core::BOOL);
windows_link::link!("wintrust.dll" "system" fn FindCertsByIssuer(pcertchains : *mut CERT_CHAIN, pcbcertchains : *mut u32, pccertchains : *mut u32, pbencodedissuername : *const u8, cbencodedissuername : u32, pwszpurpose : windows_sys::core::PCWSTR, dwkeyspec : u32) -> windows_sys::core::HRESULT);
windows_link::link!("crypt32.dll" "system" fn PFXExportCertStore(hstore : HCERTSTORE, ppfx : *mut CRYPT_DATA_BLOB, szpassword : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn PFXExportCertStoreEx(hstore : HCERTSTORE, ppfx : *mut CRYPT_DATA_BLOB, szpassword : windows_sys::core::PCWSTR, pvpara : *const core::ffi::c_void, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn PFXImportCertStore(ppfx : *const CRYPT_DATA_BLOB, szpassword : windows_sys::core::PCWSTR, dwflags : u32) -> HCERTSTORE);
windows_link::link!("crypt32.dll" "system" fn PFXIsPFXBlob(ppfx : *const CRYPT_DATA_BLOB) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn PFXVerifyPassword(ppfx : *const CRYPT_DATA_BLOB, szpassword : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::BOOL);
pub const ALG_CLASS_ALL: u32 = 57344;
pub const ALG_CLASS_ANY: u32 = 0;
pub const ALG_CLASS_DATA_ENCRYPT: u32 = 24576;
pub const ALG_CLASS_HASH: u32 = 32768;
pub const ALG_CLASS_KEY_EXCHANGE: u32 = 40960;
pub const ALG_CLASS_MSG_ENCRYPT: u32 = 16384;
pub const ALG_CLASS_SIGNATURE: u32 = 8192;
pub type ALG_ID = u32;
pub const ALG_SID_3DES: u32 = 3;
pub const ALG_SID_3DES_112: u32 = 9;
pub const ALG_SID_AES: u32 = 17;
pub const ALG_SID_AES_128: u32 = 14;
pub const ALG_SID_AES_192: u32 = 15;
pub const ALG_SID_AES_256: u32 = 16;
pub const ALG_SID_AGREED_KEY_ANY: u32 = 3;
pub const ALG_SID_ANY: u32 = 0;
pub const ALG_SID_CAST: u32 = 6;
pub const ALG_SID_CYLINK_MEK: u32 = 12;
pub const ALG_SID_DES: u32 = 1;
pub const ALG_SID_DESX: u32 = 4;
pub const ALG_SID_DH_EPHEM: u32 = 2;
pub const ALG_SID_DH_SANDF: u32 = 1;
pub const ALG_SID_DSS_ANY: u32 = 0;
pub const ALG_SID_DSS_DMS: u32 = 2;
pub const ALG_SID_DSS_PKCS: u32 = 1;
pub const ALG_SID_ECDH: u32 = 5;
pub const ALG_SID_ECDH_EPHEM: u32 = 6;
pub const ALG_SID_ECDSA: u32 = 3;
pub const ALG_SID_ECMQV: u32 = 1;
pub const ALG_SID_EXAMPLE: u32 = 80;
pub const ALG_SID_HASH_REPLACE_OWF: u32 = 11;
pub const ALG_SID_HMAC: u32 = 9;
pub const ALG_SID_IDEA: u32 = 5;
pub const ALG_SID_KEA: u32 = 4;
pub const ALG_SID_MAC: u32 = 5;
pub const ALG_SID_MD2: u32 = 1;
pub const ALG_SID_MD4: u32 = 2;
pub const ALG_SID_MD5: u32 = 3;
pub const ALG_SID_PCT1_MASTER: u32 = 4;
pub const ALG_SID_RC2: u32 = 2;
pub const ALG_SID_RC4: u32 = 1;
pub const ALG_SID_RC5: u32 = 13;
pub const ALG_SID_RIPEMD: u32 = 6;
pub const ALG_SID_RIPEMD160: u32 = 7;
pub const ALG_SID_RSA_ANY: u32 = 0;
pub const ALG_SID_RSA_ENTRUST: u32 = 3;
pub const ALG_SID_RSA_MSATWORK: u32 = 2;
pub const ALG_SID_RSA_PGP: u32 = 4;
pub const ALG_SID_RSA_PKCS: u32 = 1;
pub const ALG_SID_SAFERSK128: u32 = 8;
pub const ALG_SID_SAFERSK64: u32 = 7;
pub const ALG_SID_SCHANNEL_ENC_KEY: u32 = 7;
pub const ALG_SID_SCHANNEL_MAC_KEY: u32 = 3;
pub const ALG_SID_SCHANNEL_MASTER_HASH: u32 = 2;
pub const ALG_SID_SEAL: u32 = 2;
pub const ALG_SID_SHA: u32 = 4;
pub const ALG_SID_SHA1: u32 = 4;
pub const ALG_SID_SHA_256: u32 = 12;
pub const ALG_SID_SHA_384: u32 = 13;
pub const ALG_SID_SHA_512: u32 = 14;
pub const ALG_SID_SKIPJACK: u32 = 10;
pub const ALG_SID_SSL2_MASTER: u32 = 5;
pub const ALG_SID_SSL3SHAMD5: u32 = 8;
pub const ALG_SID_SSL3_MASTER: u32 = 1;
pub const ALG_SID_TEK: u32 = 11;
pub const ALG_SID_THIRDPARTY_ANY: u32 = 0;
pub const ALG_SID_TLS1PRF: u32 = 10;
pub const ALG_SID_TLS1_MASTER: u32 = 6;
pub const ALG_TYPE_ANY: u32 = 0;
pub const ALG_TYPE_BLOCK: u32 = 1536;
pub const ALG_TYPE_DH: u32 = 2560;
pub const ALG_TYPE_DSS: u32 = 512;
pub const ALG_TYPE_ECDH: u32 = 3584;
pub const ALG_TYPE_RSA: u32 = 1024;
pub const ALG_TYPE_SECURECHANNEL: u32 = 3072;
pub const ALG_TYPE_STREAM: u32 = 2048;
pub const ALG_TYPE_THIRDPARTY: u32 = 4096;
pub const AT_KEYEXCHANGE: u32 = 1;
pub const AT_SIGNATURE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwRegPolicySettings: u32,
    pub pSignerInfo: PCMSG_SIGNER_INFO,
}
impl Default for AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS {
    pub cbSize: u32,
    pub fCommercial: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwRegPolicySettings: u32,
    pub fCommercial: windows_sys::core::BOOL,
}
pub const AUTHTYPE_CLIENT: u32 = 1;
pub const AUTHTYPE_SERVER: u32 = 2;
pub const BASIC_CONSTRAINTS_CERT_CHAIN_POLICY_CA_FLAG: u32 = 2147483648;
pub const BASIC_CONSTRAINTS_CERT_CHAIN_POLICY_END_ENTITY_FLAG: u32 = 1073741824;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BLOBHEADER {
    pub bType: u8,
    pub bVersion: u8,
    pub reserved: u16,
    pub aiKeyAlg: ALG_ID,
}
pub const CALG_3DES: u32 = 26115;
pub const CALG_3DES_112: u32 = 26121;
pub const CALG_AES: u32 = 26129;
pub const CALG_AES_128: u32 = 26126;
pub const CALG_AES_192: u32 = 26127;
pub const CALG_AES_256: u32 = 26128;
pub const CALG_AGREEDKEY_ANY: u32 = 43523;
pub const CALG_CYLINK_MEK: u32 = 26124;
pub const CALG_DES: u32 = 26113;
pub const CALG_DESX: u32 = 26116;
pub const CALG_DH_EPHEM: u32 = 43522;
pub const CALG_DH_SF: u32 = 43521;
pub const CALG_DSS_SIGN: u32 = 8704;
pub const CALG_ECDH: u32 = 43525;
pub const CALG_ECDH_EPHEM: u32 = 44550;
pub const CALG_ECDSA: u32 = 8707;
pub const CALG_ECMQV: u32 = 40961;
pub const CALG_HASH_REPLACE_OWF: u32 = 32779;
pub const CALG_HMAC: u32 = 32777;
pub const CALG_HUGHES_MD5: u32 = 40963;
pub const CALG_KEA_KEYX: u32 = 43524;
pub const CALG_MAC: u32 = 32773;
pub const CALG_MD2: u32 = 32769;
pub const CALG_MD4: u32 = 32770;
pub const CALG_MD5: u32 = 32771;
pub const CALG_NO_SIGN: u32 = 8192;
pub const CALG_NULLCIPHER: u32 = 24576;
pub const CALG_OID_INFO_CNG_ONLY: u32 = 4294967295;
pub const CALG_OID_INFO_PARAMETERS: u32 = 4294967294;
pub const CALG_OID_INFO_PQ: u32 = 4294967293;
pub const CALG_PCT1_MASTER: u32 = 19460;
pub const CALG_RC2: u32 = 26114;
pub const CALG_RC4: u32 = 26625;
pub const CALG_RC5: u32 = 26125;
pub const CALG_RSA_KEYX: u32 = 41984;
pub const CALG_RSA_SIGN: u32 = 9216;
pub const CALG_SCHANNEL_ENC_KEY: u32 = 19463;
pub const CALG_SCHANNEL_MAC_KEY: u32 = 19459;
pub const CALG_SCHANNEL_MASTER_HASH: u32 = 19458;
pub const CALG_SEAL: u32 = 26626;
pub const CALG_SHA: u32 = 32772;
pub const CALG_SHA1: u32 = 32772;
pub const CALG_SHA_256: u32 = 32780;
pub const CALG_SHA_384: u32 = 32781;
pub const CALG_SHA_512: u32 = 32782;
pub const CALG_SKIPJACK: u32 = 26122;
pub const CALG_SSL2_MASTER: u32 = 19461;
pub const CALG_SSL3_MASTER: u32 = 19457;
pub const CALG_SSL3_SHAMD5: u32 = 32776;
pub const CALG_TEK: u32 = 26123;
pub const CALG_THIRDPARTY_CIPHER: u32 = 28672;
pub const CALG_THIRDPARTY_HASH: u32 = 36864;
pub const CALG_THIRDPARTY_KEY_EXCHANGE: u32 = 45056;
pub const CALG_THIRDPARTY_SIGNATURE: u32 = 12288;
pub const CALG_TLS1PRF: u32 = 32778;
pub const CALG_TLS1_MASTER: u32 = 19462;
pub type CCERT_STORE_PROV_FIND_INFO = CERT_STORE_PROV_FIND_INFO;
pub type CCRYPT_OID_INFO = CRYPT_OID_INFO;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_ACCESS_DESCRIPTION {
    pub pszAccessMethod: windows_sys::core::PSTR,
    pub AccessLocation: CERT_ALT_NAME_ENTRY,
}
impl Default for CERT_ACCESS_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_ACCESS_STATE_GP_SYSTEM_STORE_FLAG: u32 = 8;
pub const CERT_ACCESS_STATE_LM_SYSTEM_STORE_FLAG: u32 = 4;
pub const CERT_ACCESS_STATE_PROP_ID: u32 = 14;
pub const CERT_ACCESS_STATE_SHARED_USER_FLAG: u32 = 16;
pub const CERT_ACCESS_STATE_SYSTEM_STORE_FLAG: u32 = 2;
pub const CERT_ACCESS_STATE_WRITE_PERSIST_FLAG: u32 = 1;
pub const CERT_AIA_URL_RETRIEVED_PROP_ID: u32 = 67;
pub const CERT_ALT_NAME_DIRECTORY_NAME: u32 = 5;
pub const CERT_ALT_NAME_DNS_NAME: u32 = 3;
pub const CERT_ALT_NAME_EDI_PARTY_NAME: u32 = 6;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_ALT_NAME_ENTRY {
    pub dwAltNameChoice: u32,
    pub Anonymous: CERT_ALT_NAME_ENTRY_0,
}
impl Default for CERT_ALT_NAME_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CERT_ALT_NAME_ENTRY_0 {
    pub pOtherName: PCERT_OTHER_NAME,
    pub pwszRfc822Name: windows_sys::core::PWSTR,
    pub pwszDNSName: windows_sys::core::PWSTR,
    pub DirectoryName: CERT_NAME_BLOB,
    pub pwszURL: windows_sys::core::PWSTR,
    pub IPAddress: CRYPT_DATA_BLOB,
    pub pszRegisteredID: windows_sys::core::PSTR,
}
impl Default for CERT_ALT_NAME_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_ALT_NAME_ENTRY_ERR_INDEX_MASK: u32 = 255;
pub const CERT_ALT_NAME_ENTRY_ERR_INDEX_SHIFT: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_ALT_NAME_INFO {
    pub cAltEntry: u32,
    pub rgAltEntry: PCERT_ALT_NAME_ENTRY,
}
impl Default for CERT_ALT_NAME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_ALT_NAME_IP_ADDRESS: u32 = 8;
pub const CERT_ALT_NAME_OTHER_NAME: u32 = 1;
pub const CERT_ALT_NAME_REGISTERED_ID: u32 = 9;
pub const CERT_ALT_NAME_RFC822_NAME: u32 = 2;
pub const CERT_ALT_NAME_URL: u32 = 7;
pub const CERT_ALT_NAME_VALUE_ERR_INDEX_MASK: u32 = 65535;
pub const CERT_ALT_NAME_VALUE_ERR_INDEX_SHIFT: u32 = 0;
pub const CERT_ALT_NAME_X400_ADDRESS: u32 = 4;
pub const CERT_ARCHIVED_KEY_HASH_PROP_ID: u32 = 65;
pub const CERT_ARCHIVED_PROP_ID: u32 = 19;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_AUTHORITY_INFO_ACCESS {
    pub cAccDescr: u32,
    pub rgAccDescr: PCERT_ACCESS_DESCRIPTION,
}
impl Default for CERT_AUTHORITY_INFO_ACCESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_AUTHORITY_INFO_ACCESS_PROP_ID: u32 = 68;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_AUTHORITY_KEY_ID2_INFO {
    pub KeyId: CRYPT_DATA_BLOB,
    pub AuthorityCertIssuer: CERT_ALT_NAME_INFO,
    pub AuthorityCertSerialNumber: CRYPT_INTEGER_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_AUTHORITY_KEY_ID_INFO {
    pub KeyId: CRYPT_DATA_BLOB,
    pub CertIssuer: CERT_NAME_BLOB,
    pub CertSerialNumber: CRYPT_INTEGER_BLOB,
}
pub const CERT_AUTH_ROOT_AUTO_UPDATE_DISABLE_PARTIAL_CHAIN_LOGGING_FLAG: u32 = 2;
pub const CERT_AUTH_ROOT_AUTO_UPDATE_DISABLE_UNTRUSTED_ROOT_LOGGING_FLAG: u32 = 1;
pub const CERT_AUTH_ROOT_AUTO_UPDATE_ENCODED_CTL_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("EncodedCtl");
pub const CERT_AUTH_ROOT_AUTO_UPDATE_FLAGS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("Flags");
pub const CERT_AUTH_ROOT_AUTO_UPDATE_LAST_SYNC_TIME_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("LastSyncTime");
pub const CERT_AUTH_ROOT_AUTO_UPDATE_SYNC_DELTA_TIME_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("SyncDeltaTime");
pub const CERT_AUTH_ROOT_CAB_FILENAME: windows_sys::core::PCWSTR = windows_sys::core::w!("authrootstl.cab");
pub const CERT_AUTH_ROOT_CERT_EXT: windows_sys::core::PCWSTR = windows_sys::core::w!(".crt");
pub const CERT_AUTH_ROOT_CTL_FILENAME: windows_sys::core::PCWSTR = windows_sys::core::w!("authroot.stl");
pub const CERT_AUTH_ROOT_CTL_FILENAME_A: windows_sys::core::PCSTR = windows_sys::core::s!("authroot.stl");
pub const CERT_AUTH_ROOT_SEQ_FILENAME: windows_sys::core::PCWSTR = windows_sys::core::w!("authrootseq.txt");
pub const CERT_AUTH_ROOT_SHA256_HASH_PROP_ID: u32 = 98;
pub const CERT_AUTO_ENROLL_PROP_ID: u32 = 21;
pub const CERT_AUTO_ENROLL_RETRY_PROP_ID: u32 = 66;
pub const CERT_AUTO_UPDATE_DISABLE_RANDOM_QUERY_STRING_FLAG: u32 = 4;
pub const CERT_AUTO_UPDATE_ROOT_DIR_URL_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("RootDirUrl");
pub const CERT_AUTO_UPDATE_SYNC_FROM_DIR_URL_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("SyncFromDirUrl");
pub const CERT_BACKED_UP_PROP_ID: u32 = 69;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_BASIC_CONSTRAINTS2_INFO {
    pub fCA: windows_sys::core::BOOL,
    pub fPathLenConstraint: windows_sys::core::BOOL,
    pub dwPathLenConstraint: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_BASIC_CONSTRAINTS_INFO {
    pub SubjectType: CRYPT_BIT_BLOB,
    pub fPathLenConstraint: windows_sys::core::BOOL,
    pub dwPathLenConstraint: u32,
    pub cSubtreesConstraint: u32,
    pub rgSubtreesConstraint: *mut CERT_NAME_BLOB,
}
impl Default for CERT_BASIC_CONSTRAINTS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_BIOMETRIC_DATA {
    pub dwTypeOfBiometricDataChoice: u32,
    pub Anonymous: CERT_BIOMETRIC_DATA_0,
    pub HashedUrl: CERT_HASHED_URL,
}
impl Default for CERT_BIOMETRIC_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CERT_BIOMETRIC_DATA_0 {
    pub dwPredefined: u32,
    pub pszObjId: windows_sys::core::PSTR,
}
impl Default for CERT_BIOMETRIC_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_BIOMETRIC_EXT_INFO {
    pub cBiometricData: u32,
    pub rgBiometricData: PCERT_BIOMETRIC_DATA,
}
impl Default for CERT_BIOMETRIC_EXT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_BIOMETRIC_OID_DATA_CHOICE: u32 = 2;
pub const CERT_BIOMETRIC_PICTURE_TYPE: u32 = 0;
pub const CERT_BIOMETRIC_PREDEFINED_DATA_CHOICE: u32 = 1;
pub const CERT_BIOMETRIC_SIGNATURE_TYPE: u32 = 1;
pub type CERT_BLOB = CRYPT_INTEGER_BLOB;
pub const CERT_BUNDLE_CERTIFICATE: u32 = 0;
pub const CERT_BUNDLE_CRL: u32 = 1;
pub const CERT_CASE_INSENSITIVE_IS_RDN_ATTRS_FLAG: u32 = 2;
pub const CERT_CA_DISABLE_CRL_PROP_ID: u32 = 82;
pub const CERT_CA_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: u32 = 81;
pub const CERT_CA_SUBJECT_FLAG: u32 = 128;
pub const CERT_CEP_PROP_ID: u32 = 87;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_CHAIN {
    pub cCerts: u32,
    pub certs: PCERT_BLOB,
    pub keyLocatorInfo: CRYPT_KEY_PROV_INFO,
}
impl Default for CERT_CHAIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CHAIN_AUTO_CURRENT_USER: u32 = 1;
pub const CERT_CHAIN_AUTO_FLAGS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("AutoFlags");
pub const CERT_CHAIN_AUTO_FLUSH_DISABLE_FLAG: u32 = 1;
pub const CERT_CHAIN_AUTO_FLUSH_FIRST_DELTA_SECONDS_DEFAULT: u32 = 300;
pub const CERT_CHAIN_AUTO_FLUSH_FIRST_DELTA_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("AutoFlushFirstDeltaSeconds");
pub const CERT_CHAIN_AUTO_FLUSH_NEXT_DELTA_SECONDS_DEFAULT: u32 = 1800;
pub const CERT_CHAIN_AUTO_FLUSH_NEXT_DELTA_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("AutoFlushNextDeltaSeconds");
pub const CERT_CHAIN_AUTO_HPKP_RULE_INFO: u32 = 8;
pub const CERT_CHAIN_AUTO_IMPERSONATED: u32 = 3;
pub const CERT_CHAIN_AUTO_LOCAL_MACHINE: u32 = 2;
pub const CERT_CHAIN_AUTO_LOG_CREATE_FLAG: u32 = 2;
pub const CERT_CHAIN_AUTO_LOG_FILE_NAME_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("AutoLogFileName");
pub const CERT_CHAIN_AUTO_LOG_FLAGS: u32 = 14;
pub const CERT_CHAIN_AUTO_LOG_FLUSH_FLAG: u32 = 8;
pub const CERT_CHAIN_AUTO_LOG_FREE_FLAG: u32 = 4;
pub const CERT_CHAIN_AUTO_NETWORK_INFO: u32 = 6;
pub const CERT_CHAIN_AUTO_PINRULE_INFO: u32 = 5;
pub const CERT_CHAIN_AUTO_PROCESS_INFO: u32 = 4;
pub const CERT_CHAIN_AUTO_SERIAL_LOCAL_MACHINE: u32 = 7;
pub const CERT_CHAIN_CACHE_END_CERT: u32 = 1;
pub const CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL: u32 = 4;
pub const CERT_CHAIN_CACHE_RESYNC_FILETIME_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("ChainCacheResyncFiletime");
pub const CERT_CHAIN_CONFIG_REGPATH: windows_sys::core::PCWSTR = windows_sys::core::w!("Software\\Microsoft\\Cryptography\\OID\\EncodingType 0\\CertDllCreateCertificateChainEngine\\Config");
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CERT_CHAIN_CONTEXT {
    pub cbSize: u32,
    pub TrustStatus: CERT_TRUST_STATUS,
    pub cChain: u32,
    pub rgpChain: *mut PCERT_SIMPLE_CHAIN,
    pub cLowerQualityChainContext: u32,
    pub rgpLowerQualityChainContext: *mut PCCERT_CHAIN_CONTEXT,
    pub fHasRevocationFreshnessTime: windows_sys::core::BOOL,
    pub dwRevocationFreshnessTime: u32,
    pub dwCreateFlags: u32,
    pub ChainId: windows_sys::core::GUID,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_CHAIN_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CHAIN_CRL_VALIDITY_EXT_PERIOD_HOURS_DEFAULT: u32 = 12;
pub const CERT_CHAIN_CRL_VALIDITY_EXT_PERIOD_HOURS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CRLValidityExtensionPeriod");
pub const CERT_CHAIN_CROSS_CERT_DOWNLOAD_INTERVAL_HOURS_DEFAULT: u32 = 168;
pub const CERT_CHAIN_CROSS_CERT_DOWNLOAD_INTERVAL_HOURS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CrossCertDownloadIntervalHours");
pub const CERT_CHAIN_DEFAULT_CONFIG_SUBDIR: windows_sys::core::PCWSTR = windows_sys::core::w!("Default");
pub const CERT_CHAIN_DISABLE_AIA: u32 = 8192;
pub const CERT_CHAIN_DISABLE_AIA_URL_RETRIEVAL_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("DisableAIAUrlRetrieval");
pub const CERT_CHAIN_DISABLE_ALL_EKU_WEAK_FLAG: u32 = 65536;
pub const CERT_CHAIN_DISABLE_AUTH_ROOT_AUTO_UPDATE: u32 = 256;
pub const CERT_CHAIN_DISABLE_AUTO_FLUSH_PROCESS_NAME_LIST_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("DisableAutoFlushProcessNameList");
pub const CERT_CHAIN_DISABLE_CA_NAME_CONSTRAINTS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("DisableCANameConstraints");
pub const CERT_CHAIN_DISABLE_CODE_SIGNING_WEAK_FLAG: u32 = 4194304;
pub const CERT_CHAIN_DISABLE_ECC_PARA_FLAG: u32 = 16;
pub const CERT_CHAIN_DISABLE_FILE_HASH_WEAK_FLAG: u32 = 4096;
pub const CERT_CHAIN_DISABLE_FILE_HASH_WEAK_FLAGS: u32 = 12288;
pub const CERT_CHAIN_DISABLE_MANDATORY_BASIC_CONSTRAINTS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("DisableMandatoryBasicConstraints");
pub const CERT_CHAIN_DISABLE_MD2_MD4: u32 = 4096;
pub const CERT_CHAIN_DISABLE_MOTW_CODE_SIGNING_WEAK_FLAG: u32 = 8388608;
pub const CERT_CHAIN_DISABLE_MOTW_FILE_HASH_WEAK_FLAG: u32 = 8192;
pub const CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_HASH_WEAK_FLAG: u32 = 32768;
pub const CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_WEAK_FLAG: u32 = 134217728;
pub const CERT_CHAIN_DISABLE_MY_PEER_TRUST: u32 = 2048;
pub const CERT_CHAIN_DISABLE_OPT_IN_SERVER_AUTH_WEAK_FLAG: u32 = 262144;
pub const CERT_CHAIN_DISABLE_PASS1_QUALITY_FILTERING: u32 = 64;
pub const CERT_CHAIN_DISABLE_SERIAL_CHAIN_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("DisableSerialChain");
pub const CERT_CHAIN_DISABLE_SERVER_AUTH_WEAK_FLAG: u32 = 1048576;
pub const CERT_CHAIN_DISABLE_SYNC_WITH_SSL_TIME_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("DisableSyncWithSslTime");
pub const CERT_CHAIN_DISABLE_TIMESTAMP_HASH_WEAK_FLAG: u32 = 16384;
pub const CERT_CHAIN_DISABLE_TIMESTAMP_HASH_WEAK_FLAGS: u32 = 49152;
pub const CERT_CHAIN_DISABLE_TIMESTAMP_WEAK_FLAG: u32 = 67108864;
pub const CERT_CHAIN_DISABLE_UNSUPPORTED_CRITICAL_EXTENSIONS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("DisableUnsupportedCriticalExtensions");
pub const CERT_CHAIN_DISABLE_WEAK_FLAGS: u32 = 215285776;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CERT_CHAIN_ELEMENT {
    pub cbSize: u32,
    pub pCertContext: PCCERT_CONTEXT,
    pub TrustStatus: CERT_TRUST_STATUS,
    pub pRevocationInfo: PCERT_REVOCATION_INFO,
    pub pIssuanceUsage: PCERT_ENHKEY_USAGE,
    pub pApplicationUsage: PCERT_ENHKEY_USAGE,
    pub pwszExtendedErrorInfo: windows_sys::core::PCWSTR,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_CHAIN_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CHAIN_ENABLE_ALL_EKU_HYGIENE_FLAG: u32 = 131072;
pub const CERT_CHAIN_ENABLE_CACHE_AUTO_UPDATE: u32 = 16;
pub const CERT_CHAIN_ENABLE_CODE_SIGNING_HYGIENE_FLAG: u32 = 16777216;
pub const CERT_CHAIN_ENABLE_DISALLOWED_CA: u32 = 131072;
pub const CERT_CHAIN_ENABLE_HYGIENE_FLAGS: u32 = 857866240;
pub const CERT_CHAIN_ENABLE_MD2_MD4_FLAG: u32 = 1;
pub const CERT_CHAIN_ENABLE_MOTW_CODE_SIGNING_HYGIENE_FLAG: u32 = 33554432;
pub const CERT_CHAIN_ENABLE_MOTW_TIMESTAMP_HYGIENE_FLAG: u32 = 536870912;
pub const CERT_CHAIN_ENABLE_ONLY_WEAK_LOGGING_FLAG: u32 = 8;
pub const CERT_CHAIN_ENABLE_PEER_TRUST: u32 = 1024;
pub const CERT_CHAIN_ENABLE_SERVER_AUTH_HYGIENE_FLAG: u32 = 2097152;
pub const CERT_CHAIN_ENABLE_SHARE_STORE: u32 = 32;
pub const CERT_CHAIN_ENABLE_TIMESTAMP_HYGIENE_FLAG: u32 = 268435456;
pub const CERT_CHAIN_ENABLE_WEAK_LOGGING_FLAG: u32 = 4;
pub const CERT_CHAIN_ENABLE_WEAK_RSA_ROOT_FLAG: u32 = 2;
pub const CERT_CHAIN_ENABLE_WEAK_SETTINGS_FLAG: u32 = 2147483648;
pub const CERT_CHAIN_ENABLE_WEAK_SIGNATURE_FLAGS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("EnableWeakSignatureFlags");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_CHAIN_ENGINE_CONFIG {
    pub cbSize: u32,
    pub hRestrictedRoot: HCERTSTORE,
    pub hRestrictedTrust: HCERTSTORE,
    pub hRestrictedOther: HCERTSTORE,
    pub cAdditionalStore: u32,
    pub rghAdditionalStore: *mut HCERTSTORE,
    pub dwFlags: u32,
    pub dwUrlRetrievalTimeout: u32,
    pub MaximumCachedCertificates: u32,
    pub CycleDetectionModulus: u32,
    pub hExclusiveRoot: HCERTSTORE,
    pub hExclusiveTrustedPeople: HCERTSTORE,
    pub dwExclusiveFlags: u32,
}
impl Default for CERT_CHAIN_ENGINE_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CHAIN_EXCLUSIVE_ENABLE_CA_FLAG: u32 = 1;
pub const CERT_CHAIN_FIND_BY_ISSUER: u32 = 1;
pub const CERT_CHAIN_FIND_BY_ISSUER_CACHE_ONLY_FLAG: u32 = 32768;
pub const CERT_CHAIN_FIND_BY_ISSUER_CACHE_ONLY_URL_FLAG: u32 = 4;
pub const CERT_CHAIN_FIND_BY_ISSUER_COMPARE_KEY_FLAG: u32 = 1;
pub const CERT_CHAIN_FIND_BY_ISSUER_COMPLEX_CHAIN_FLAG: u32 = 2;
pub const CERT_CHAIN_FIND_BY_ISSUER_LOCAL_MACHINE_FLAG: u32 = 8;
pub const CERT_CHAIN_FIND_BY_ISSUER_NO_KEY_FLAG: u32 = 16384;
#[cfg(feature = "minwindef")]
pub type CERT_CHAIN_FIND_BY_ISSUER_PARA = CERT_CHAIN_FIND_ISSUER_PARA;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CERT_CHAIN_FIND_ISSUER_PARA {
    pub cbSize: u32,
    pub pszUsageIdentifier: windows_sys::core::PCSTR,
    pub dwKeySpec: u32,
    pub dwAcquirePrivateKeyFlags: u32,
    pub cIssuer: u32,
    pub rgIssuer: *mut CERT_NAME_BLOB,
    pub pfnFindCallback: PFN_CERT_CHAIN_FIND_BY_ISSUER_CALLBACK,
    pub pvFindArg: *mut core::ffi::c_void,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_CHAIN_FIND_ISSUER_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CHAIN_HAS_MOTW: u32 = 16384;
pub const CERT_CHAIN_MAX_AIA_URL_COUNT_IN_CERT_DEFAULT: u32 = 5;
pub const CERT_CHAIN_MAX_AIA_URL_COUNT_IN_CERT_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("MaxAIAUrlCountInCert");
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_BYTE_COUNT_DEFAULT: u32 = 100000;
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_BYTE_COUNT_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("MaxAIAUrlRetrievalByteCount");
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_CERT_COUNT_DEFAULT: u32 = 10;
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_CERT_COUNT_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("MaxAIAUrlRetrievalCertCount");
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_COUNT_PER_CHAIN_DEFAULT: u32 = 3;
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_COUNT_PER_CHAIN_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("MaxAIAUrlRetrievalCountPerChain");
pub const CERT_CHAIN_MAX_SSL_TIME_UPDATED_EVENT_COUNT_DEFAULT: u32 = 5;
pub const CERT_CHAIN_MAX_SSL_TIME_UPDATED_EVENT_COUNT_DISABLE: u32 = 4294967295;
pub const CERT_CHAIN_MAX_SSL_TIME_UPDATED_EVENT_COUNT_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("MaxSslTimeUpdatedEventCount");
pub const CERT_CHAIN_MAX_URL_RETRIEVAL_BYTE_COUNT_DEFAULT: u32 = 104857600;
pub const CERT_CHAIN_MAX_URL_RETRIEVAL_BYTE_COUNT_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("MaxUrlRetrievalByteCount");
pub const CERT_CHAIN_MIN_PUB_KEY_BIT_LENGTH_DISABLE: u32 = 4294967295;
pub const CERT_CHAIN_MIN_RSA_PUB_KEY_BIT_LENGTH_DEFAULT: u32 = 1023;
pub const CERT_CHAIN_MIN_RSA_PUB_KEY_BIT_LENGTH_DISABLE: u32 = 4294967295;
pub const CERT_CHAIN_MIN_RSA_PUB_KEY_BIT_LENGTH_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("MinRsaPubKeyBitLength");
pub const CERT_CHAIN_MIN_TELEMETRY_RSA_PUB_KEY_BIT_LENGTH_DEFAULT: u32 = 2047;
pub const CERT_CHAIN_MIN_TELEMETRY_RSA_PUB_KEY_BIT_LENGTH_DISABLE: u32 = 4294967295;
pub const CERT_CHAIN_MIN_TELEMETRY_RSA_PUB_KEY_BIT_LENGTH_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("MinTelemetryRsaPubKeyBitLength");
pub const CERT_CHAIN_MIN_WEAK_RSA_PUB_KEY_BIT_LENGTH_DEFAULT: u32 = 2047;
pub const CERT_CHAIN_MIN_WEAK_RSA_PUB_KEY_BIT_LENGTH_DISABLE: u32 = 4294967295;
pub const CERT_CHAIN_MIN_WEAK_RSA_PUB_KEY_BIT_LENGTH_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("MinWeakRsaPubKeyBitLength");
pub const CERT_CHAIN_MOTW_IGNORE_AFTER_TIME_WEAK_FLAG: u32 = 1073741824;
pub const CERT_CHAIN_MOTW_WEAK_FLAGS: u32 = 1786773504;
pub const CERT_CHAIN_OCSP_VALIDITY_SECONDS_DEFAULT: u32 = 43200;
pub const CERT_CHAIN_OCSP_VALIDITY_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("OcspValiditySeconds");
pub const CERT_CHAIN_ONLY_ADDITIONAL_AND_AUTH_ROOT: u32 = 32768;
pub const CERT_CHAIN_OPTIONS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("Options");
pub const CERT_CHAIN_OPTION_DISABLE_AIA_URL_RETRIEVAL: u32 = 2;
pub const CERT_CHAIN_OPTION_ENABLE_SIA_URL_RETRIEVAL: u32 = 4;
pub const CERT_CHAIN_OPT_IN_WEAK_FLAGS: u32 = 262144;
pub const CERT_CHAIN_OPT_IN_WEAK_SIGNATURE: u32 = 65536;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_CHAIN_PARA {
    pub cbSize: u32,
    pub RequestedUsage: CERT_USAGE_MATCH,
}
pub const CERT_CHAIN_POLICY_ALLOW_TESTROOT_FLAG: u32 = 32768;
pub const CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG: u32 = 16;
pub const CERT_CHAIN_POLICY_AUTHENTICODE: windows_sys::core::PCSTR = 2 as _;
pub const CERT_CHAIN_POLICY_AUTHENTICODE_TS: windows_sys::core::PCSTR = 3 as _;
pub const CERT_CHAIN_POLICY_BASE: windows_sys::core::PCSTR = 1 as _;
pub const CERT_CHAIN_POLICY_BASIC_CONSTRAINTS: windows_sys::core::PCSTR = 5 as _;
pub const CERT_CHAIN_POLICY_CT: windows_sys::core::PCSTR = 13 as _;
pub const CERT_CHAIN_POLICY_CT_ERROR_CANNOT_VALIDATE_SCT: i32 = -50;
pub const CERT_CHAIN_POLICY_CT_ERROR_INVALID_ISSUER_CERT: i32 = -101;
pub const CERT_CHAIN_POLICY_CT_ERROR_INVALID_SUBJECT_CERT: i32 = -100;
pub const CERT_CHAIN_POLICY_CT_ERROR_MISSING_SCT_EXTENSION: i32 = -110;
pub const CERT_CHAIN_POLICY_CT_ERROR_SCT_VALIDATION_STATUS_INSUFFICIENT: i32 = -4;
pub const CERT_CHAIN_POLICY_CT_ERROR_SCT_VALIDATION_STATUS_INVALID: i32 = -1;
pub const CERT_CHAIN_POLICY_CT_ERROR_SCT_VALIDATION_STATUS_UNKNOWN_LOG: i32 = -2;
pub const CERT_CHAIN_POLICY_CT_ERROR_SCT_VALIDATION_STATUS_UNKNOWN_VERSION: i32 = -3;
pub const CERT_CHAIN_POLICY_CT_ERROR_UNDECODABLE_SCT_EXTENSION: i32 = -112;
pub const CERT_CHAIN_POLICY_CT_ERROR_UNRETRIEVABLE_SCT_EXTENSION: i32 = -111;
pub const CERT_CHAIN_POLICY_CT_SUCCESS_SCT_VALIDIDATION_STATUS_VALID: u32 = 0;
pub const CERT_CHAIN_POLICY_CT_WARNING_BEFORE_CODE_SIGNING_CT_LOGGING: u32 = 2;
pub const CERT_CHAIN_POLICY_CT_WARNING_CANNOT_CREATE_POLICY: u32 = 300;
pub const CERT_CHAIN_POLICY_CT_WARNING_CANNOT_CREATE_TEMP_FILE: u32 = 81;
pub const CERT_CHAIN_POLICY_CT_WARNING_CANNOT_LOAD_CTLOG_STORE_FILE: u32 = 83;
pub const CERT_CHAIN_POLICY_CT_WARNING_CANNOT_WRITE_TEMP_FILE: u32 = 82;
pub const CERT_CHAIN_POLICY_CT_WARNING_EXPIRED_ROOT_CTL: u32 = 4;
pub const CERT_CHAIN_POLICY_CT_WARNING_FAILED_INIT: u32 = 90;
pub const CERT_CHAIN_POLICY_CT_WARNING_HASHING_ERROR: u32 = 200;
pub const CERT_CHAIN_POLICY_CT_WARNING_INVALID_CHAIN_CONTEXT: u32 = 50;
pub const CERT_CHAIN_POLICY_CT_WARNING_INVALID_CT_EXT: u32 = 61;
pub const CERT_CHAIN_POLICY_CT_WARNING_INVALID_STR: u32 = 201;
pub const CERT_CHAIN_POLICY_CT_WARNING_INVALID_TEMP_FILE: u32 = 80;
pub const CERT_CHAIN_POLICY_CT_WARNING_MISSING_CT_EXT: u32 = 60;
pub const CERT_CHAIN_POLICY_CT_WARNING_MISSING_ROOT_CTL: u32 = 52;
pub const CERT_CHAIN_POLICY_CT_WARNING_NOT_SUPPORTED_CA: u32 = 51;
pub const CERT_CHAIN_POLICY_CT_WARNING_NOT_THIRD_PARTY_CERT: u32 = 3;
pub const CERT_CHAIN_POLICY_CT_WARNING_OUT_OF_MEMORY: u32 = 1;
pub const CERT_CHAIN_POLICY_CT_WARNING_UNABLE_TO_DECODE_EXT: u32 = 62;
pub const CERT_CHAIN_POLICY_CT_WARNING_UNABLE_TO_DECODE_PARAMETERS: u32 = 70;
pub const CERT_CHAIN_POLICY_EV: windows_sys::core::PCSTR = 8 as _;
pub const CERT_CHAIN_POLICY_IGNORE_ALL_NOT_TIME_VALID_FLAGS: u32 = 7;
pub const CERT_CHAIN_POLICY_IGNORE_ALL_REV_UNKNOWN_FLAGS: u32 = 3840;
pub const CERT_CHAIN_POLICY_IGNORE_CA_REV_UNKNOWN_FLAG: u32 = 1024;
pub const CERT_CHAIN_POLICY_IGNORE_CTL_NOT_TIME_VALID_FLAG: u32 = 2;
pub const CERT_CHAIN_POLICY_IGNORE_CTL_SIGNER_REV_UNKNOWN_FLAG: u32 = 512;
pub const CERT_CHAIN_POLICY_IGNORE_END_REV_UNKNOWN_FLAG: u32 = 256;
pub const CERT_CHAIN_POLICY_IGNORE_INVALID_BASIC_CONSTRAINTS_FLAG: u32 = 8;
pub const CERT_CHAIN_POLICY_IGNORE_INVALID_NAME_FLAG: u32 = 64;
pub const CERT_CHAIN_POLICY_IGNORE_INVALID_POLICY_FLAG: u32 = 128;
pub const CERT_CHAIN_POLICY_IGNORE_NOT_SUPPORTED_CRITICAL_EXT_FLAG: u32 = 8192;
pub const CERT_CHAIN_POLICY_IGNORE_NOT_TIME_NESTED_FLAG: u32 = 4;
pub const CERT_CHAIN_POLICY_IGNORE_NOT_TIME_VALID_FLAG: u32 = 1;
pub const CERT_CHAIN_POLICY_IGNORE_PEER_TRUST_FLAG: u32 = 4096;
pub const CERT_CHAIN_POLICY_IGNORE_ROOT_REV_UNKNOWN_FLAG: u32 = 2048;
pub const CERT_CHAIN_POLICY_IGNORE_WEAK_SIGNATURE_FLAG: u32 = 134217728;
pub const CERT_CHAIN_POLICY_IGNORE_WRONG_USAGE_FLAG: u32 = 32;
pub const CERT_CHAIN_POLICY_MICROSOFT_ROOT: windows_sys::core::PCSTR = 7 as _;
pub const CERT_CHAIN_POLICY_NT_AUTH: windows_sys::core::PCSTR = 6 as _;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub pvExtraPolicyPara: *mut core::ffi::c_void,
}
impl Default for CERT_CHAIN_POLICY_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CHAIN_POLICY_SSL: windows_sys::core::PCSTR = 4 as _;
pub const CERT_CHAIN_POLICY_SSL_F12: windows_sys::core::PCSTR = 9 as _;
pub const CERT_CHAIN_POLICY_SSL_F12_ERROR_LEVEL: u32 = 2;
pub const CERT_CHAIN_POLICY_SSL_F12_NONE_CATEGORY: u32 = 0;
pub const CERT_CHAIN_POLICY_SSL_F12_ROOT_PROGRAM_CATEGORY: u32 = 2;
pub const CERT_CHAIN_POLICY_SSL_F12_SUCCESS_LEVEL: u32 = 0;
pub const CERT_CHAIN_POLICY_SSL_F12_WARNING_LEVEL: u32 = 1;
pub const CERT_CHAIN_POLICY_SSL_F12_WEAK_CRYPTO_CATEGORY: u32 = 1;
pub const CERT_CHAIN_POLICY_SSL_HPKP_HEADER: windows_sys::core::PCSTR = 10 as _;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN: windows_sys::core::PCSTR = 12 as _;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MISMATCH_ERROR: i32 = -2;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MISMATCH_WARNING: u32 = 2;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MITM_ERROR: i32 = -1;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MITM_WARNING: u32 = 1;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_SUCCESS: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_CHAIN_POLICY_STATUS {
    pub cbSize: u32,
    pub dwError: u32,
    pub lChainIndex: i32,
    pub lElementIndex: i32,
    pub pvExtraPolicyStatus: *mut core::ffi::c_void,
}
impl Default for CERT_CHAIN_POLICY_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CHAIN_POLICY_THIRD_PARTY_ROOT: windows_sys::core::PCSTR = 11 as _;
pub const CERT_CHAIN_POLICY_TRUST_TESTROOT_FLAG: u32 = 16384;
pub const CERT_CHAIN_RETURN_LOWER_QUALITY_CONTEXTS: u32 = 128;
pub const CERT_CHAIN_REVOCATION_ACCUMULATIVE_TIMEOUT: u32 = 134217728;
pub const CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY: u32 = 2147483648;
pub const CERT_CHAIN_REVOCATION_CHECK_CHAIN: u32 = 536870912;
pub const CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: u32 = 1073741824;
pub const CERT_CHAIN_REVOCATION_CHECK_END_CERT: u32 = 268435456;
pub const CERT_CHAIN_REVOCATION_CHECK_OCSP_CERT: u32 = 67108864;
pub const CERT_CHAIN_REV_ACCUMULATIVE_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_DEFAULT: u32 = 20000;
pub const CERT_CHAIN_REV_ACCUMULATIVE_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("ChainRevAccumulativeUrlRetrievalTimeoutMilliseconds");
pub const CERT_CHAIN_SERIAL_CHAIN_LOG_FILE_NAME_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("SerialChainLogFileName");
pub const CERT_CHAIN_SSL_HANDSHAKE_LOG_FILE_NAME_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("SslHandshakeLogFileName");
pub const CERT_CHAIN_STRONG_SIGN_DISABLE_END_CHECK_FLAG: u32 = 1;
pub const CERT_CHAIN_THREAD_STORE_SYNC: u32 = 2;
pub const CERT_CHAIN_TIMESTAMP_TIME: u32 = 512;
pub const CERT_CHAIN_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_DEFAULT: u32 = 15000;
pub const CERT_CHAIN_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("ChainUrlRetrievalTimeoutMilliseconds");
pub const CERT_CHAIN_USE_LOCAL_MACHINE_STORE: u32 = 8;
pub const CERT_CHAIN_WEAK_AFTER_TIME_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("AfterTime");
pub const CERT_CHAIN_WEAK_ALL_CONFIG_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("All");
pub const CERT_CHAIN_WEAK_FILE_HASH_AFTER_TIME_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("FileHashAfterTime");
pub const CERT_CHAIN_WEAK_FLAGS_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("Flags");
pub const CERT_CHAIN_WEAK_HYGIENE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("Hygiene");
pub const CERT_CHAIN_WEAK_MIN_BIT_LENGTH_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("MinBitLength");
pub const CERT_CHAIN_WEAK_PREFIX_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("Weak");
pub const CERT_CHAIN_WEAK_RSA_PUB_KEY_TIME_DEFAULT: u32 = 1550712832;
pub const CERT_CHAIN_WEAK_RSA_PUB_KEY_TIME_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("WeakRsaPubKeyTime");
pub const CERT_CHAIN_WEAK_SHA256_ALLOW_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("Sha256Allow");
pub const CERT_CHAIN_WEAK_SIGNATURE_LOG_DIR_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("WeakSignatureLogDir");
pub const CERT_CHAIN_WEAK_THIRD_PARTY_CONFIG_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("ThirdParty");
pub const CERT_CHAIN_WEAK_TIMESTAMP_HASH_AFTER_TIME_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("TimestampHashAfterTime");
pub const CERT_CLOSE_STORE_CHECK_FLAG: u32 = 2;
pub const CERT_CLOSE_STORE_FORCE_FLAG: u32 = 1;
pub const CERT_CLR_DELETE_KEY_PROP_ID: u32 = 125;
pub const CERT_COMPARE_ANY: u32 = 0;
pub const CERT_COMPARE_ATTR: u32 = 3;
pub const CERT_COMPARE_CERT_ID: u32 = 16;
pub const CERT_COMPARE_CROSS_CERT_DIST_POINTS: u32 = 17;
pub const CERT_COMPARE_CTL_USAGE: u32 = 10;
pub const CERT_COMPARE_ENHKEY_USAGE: u32 = 10;
pub const CERT_COMPARE_EXISTING: u32 = 13;
pub const CERT_COMPARE_HASH: u32 = 1;
pub const CERT_COMPARE_HASH_STR: u32 = 20;
pub const CERT_COMPARE_HAS_PRIVATE_KEY: u32 = 21;
pub const CERT_COMPARE_ISSUER_OF: u32 = 12;
pub const CERT_COMPARE_KEY_IDENTIFIER: u32 = 15;
pub const CERT_COMPARE_KEY_SPEC: u32 = 9;
pub const CERT_COMPARE_MASK: u32 = 65535;
pub const CERT_COMPARE_MD5_HASH: u32 = 4;
pub const CERT_COMPARE_NAME: u32 = 2;
pub const CERT_COMPARE_NAME_STR_A: u32 = 7;
pub const CERT_COMPARE_NAME_STR_W: u32 = 8;
pub const CERT_COMPARE_PROPERTY: u32 = 5;
pub const CERT_COMPARE_PUBKEY_MD5_HASH: u32 = 18;
pub const CERT_COMPARE_PUBLIC_KEY: u32 = 6;
pub const CERT_COMPARE_SHA1_HASH: u32 = 1;
pub const CERT_COMPARE_SHA1_SHA256_HASH: u32 = 23;
pub const CERT_COMPARE_SHA256_HASH: u32 = 22;
pub const CERT_COMPARE_SHIFT: u32 = 16;
pub const CERT_COMPARE_SIGNATURE_HASH: u32 = 14;
pub const CERT_COMPARE_SUBJECT_CERT: u32 = 11;
pub const CERT_COMPARE_SUBJECT_INFO_ACCESS: u32 = 19;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CERT_CONTEXT {
    pub dwCertEncodingType: u32,
    pub pbCertEncoded: *mut u8,
    pub cbCertEncoded: u32,
    pub pCertInfo: PCERT_INFO,
    pub hCertStore: HCERTSTORE,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CONTEXT_REVOCATION_TYPE: u32 = 1;
pub const CERT_CREATE_CONTEXT_NOCOPY_FLAG: u32 = 1;
pub const CERT_CREATE_CONTEXT_NO_ENTRY_FLAG: u32 = 8;
pub const CERT_CREATE_CONTEXT_NO_HCRYPTMSG_FLAG: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_CREATE_CONTEXT_PARA {
    pub cbSize: u32,
    pub pfnFree: PFN_CRYPT_FREE,
    pub pvFree: *mut core::ffi::c_void,
    pub pfnSort: PFN_CERT_CREATE_CONTEXT_SORT_FUNC,
    pub pvSort: *mut core::ffi::c_void,
}
impl Default for CERT_CREATE_CONTEXT_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CREATE_CONTEXT_SORTED_FLAG: u32 = 2;
pub const CERT_CREATE_SELFSIGN_NO_KEY_INFO: u32 = 2;
pub const CERT_CREATE_SELFSIGN_NO_SIGN: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CERT_CRL_CONTEXT_PAIR {
    pub pCertContext: PCCERT_CONTEXT,
    pub pCrlContext: PCCRL_CONTEXT,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_CRL_CONTEXT_PAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_CRL_SIGN_KEY_USAGE: u32 = 2;
pub const CERT_CROSS_CERT_DIST_POINTS_PROP_ID: u32 = 23;
pub const CERT_CTL_USAGE_PROP_ID: u32 = 9;
pub const CERT_DATA_ENCIPHERMENT_KEY_USAGE: u32 = 16;
pub const CERT_DATE_STAMP_PROP_ID: u32 = 27;
pub const CERT_DECIPHER_ONLY_KEY_USAGE: u32 = 128;
pub const CERT_DESCRIPTION_PROP_ID: u32 = 13;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_DH_PARAMETERS {
    pub p: CRYPT_UINT_BLOB,
    pub g: CRYPT_UINT_BLOB,
}
pub const CERT_DIGITAL_SIGNATURE_KEY_USAGE: u32 = 128;
pub const CERT_DISABLE_PIN_RULES_AUTO_UPDATE_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("DisablePinRulesAutoUpdate");
pub const CERT_DISABLE_ROOT_AUTO_UPDATE_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("DisableRootAutoUpdate");
pub const CERT_DISALLOWED_CA_FILETIME_PROP_ID: u32 = 128;
pub const CERT_DISALLOWED_CERT_AUTO_UPDATE_ENCODED_CTL_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("DisallowedCertEncodedCtl");
pub const CERT_DISALLOWED_CERT_AUTO_UPDATE_LAST_SYNC_TIME_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("DisallowedCertLastSyncTime");
pub const CERT_DISALLOWED_CERT_AUTO_UPDATE_LIST_IDENTIFIER: windows_sys::core::PCWSTR = windows_sys::core::w!("DisallowedCert_AutoUpdate_1");
pub const CERT_DISALLOWED_CERT_AUTO_UPDATE_SYNC_DELTA_TIME_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("DisallowedCertSyncDeltaTime");
pub const CERT_DISALLOWED_CERT_CAB_FILENAME: windows_sys::core::PCWSTR = windows_sys::core::w!("disallowedcertstl.cab");
pub const CERT_DISALLOWED_CERT_CTL_FILENAME: windows_sys::core::PCWSTR = windows_sys::core::w!("disallowedcert.stl");
pub const CERT_DISALLOWED_CERT_CTL_FILENAME_A: windows_sys::core::PCSTR = windows_sys::core::s!("disallowedcert.stl");
pub const CERT_DISALLOWED_ENHKEY_USAGE_PROP_ID: u32 = 122;
pub const CERT_DISALLOWED_FILETIME_PROP_ID: u32 = 104;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_DSS_PARAMETERS {
    pub p: CRYPT_UINT_BLOB,
    pub q: CRYPT_UINT_BLOB,
    pub g: CRYPT_UINT_BLOB,
}
pub const CERT_DSS_R_LEN: u32 = 20;
pub const CERT_DSS_SIGNATURE_LEN: u32 = 40;
pub const CERT_DSS_S_LEN: u32 = 20;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_ECC_SIGNATURE {
    pub r: CRYPT_UINT_BLOB,
    pub s: CRYPT_UINT_BLOB,
}
pub const CERT_EFSBLOB_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("EFSBlob");
pub const CERT_EFS_PROP_ID: u32 = 17;
pub const CERT_ENABLE_DISALLOWED_CERT_AUTO_UPDATE_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("EnableDisallowedCertAutoUpdate");
pub const CERT_ENCIPHER_ONLY_KEY_USAGE: u32 = 1;
pub const CERT_ENCODING_TYPE_MASK: u32 = 65535;
pub const CERT_END_ENTITY_SUBJECT_FLAG: u32 = 64;
pub type CERT_ENHKEY_USAGE = CTL_USAGE;
pub const CERT_ENHKEY_USAGE_PROP_ID: u32 = 9;
pub const CERT_ENROLLMENT_PROP_ID: u32 = 26;
pub const CERT_EXCLUDED_SUBTREE_BIT: u32 = 2147483648;
pub const CERT_EXTENDED_ERROR_INFO_PROP_ID: u32 = 30;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_EXTENSION {
    pub pszObjId: windows_sys::core::PSTR,
    pub fCritical: windows_sys::core::BOOL,
    pub Value: CRYPT_OBJID_BLOB,
}
impl Default for CERT_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_EXTENSIONS {
    pub cExtension: u32,
    pub rgExtension: PCERT_EXTENSION,
}
impl Default for CERT_EXTENSIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_FILE_HASH_USE_TYPE: u32 = 1;
pub const CERT_FILE_STORE_COMMIT_ENABLE_FLAG: u32 = 65536;
pub const CERT_FIND_ANY: u32 = 0;
pub const CERT_FIND_CERT_ID: u32 = 1048576;
pub const CERT_FIND_CROSS_CERT_DIST_POINTS: u32 = 1114112;
pub const CERT_FIND_CTL_USAGE: u32 = 655360;
pub const CERT_FIND_ENHKEY_USAGE: u32 = 655360;
pub const CERT_FIND_EXISTING: u32 = 851968;
pub const CERT_FIND_EXT_ONLY_CTL_USAGE_FLAG: u32 = 2;
pub const CERT_FIND_EXT_ONLY_ENHKEY_USAGE_FLAG: u32 = 2;
pub const CERT_FIND_HASH: u32 = 65536;
pub const CERT_FIND_HASH_STR: u32 = 1310720;
pub const CERT_FIND_HAS_PRIVATE_KEY: u32 = 1376256;
pub const CERT_FIND_ISSUER_ATTR: u32 = 196612;
pub const CERT_FIND_ISSUER_NAME: u32 = 131076;
pub const CERT_FIND_ISSUER_OF: u32 = 786432;
pub const CERT_FIND_ISSUER_STR: u32 = 524292;
pub const CERT_FIND_ISSUER_STR_A: u32 = 458756;
pub const CERT_FIND_ISSUER_STR_W: u32 = 524292;
pub const CERT_FIND_KEY_IDENTIFIER: u32 = 983040;
pub const CERT_FIND_KEY_SPEC: u32 = 589824;
pub const CERT_FIND_MD5_HASH: u32 = 262144;
pub const CERT_FIND_NO_CTL_USAGE_FLAG: u32 = 8;
pub const CERT_FIND_NO_ENHKEY_USAGE_FLAG: u32 = 8;
pub const CERT_FIND_OPTIONAL_CTL_USAGE_FLAG: u32 = 1;
pub const CERT_FIND_OPTIONAL_ENHKEY_USAGE_FLAG: u32 = 1;
pub const CERT_FIND_OR_CTL_USAGE_FLAG: u32 = 16;
pub const CERT_FIND_OR_ENHKEY_USAGE_FLAG: u32 = 16;
pub const CERT_FIND_PROPERTY: u32 = 327680;
pub const CERT_FIND_PROP_ONLY_CTL_USAGE_FLAG: u32 = 4;
pub const CERT_FIND_PROP_ONLY_ENHKEY_USAGE_FLAG: u32 = 4;
pub const CERT_FIND_PUBKEY_MD5_HASH: u32 = 1179648;
pub const CERT_FIND_PUBLIC_KEY: u32 = 393216;
pub const CERT_FIND_SHA1_HASH: u32 = 65536;
pub const CERT_FIND_SHA1_SHA256_HASH: u32 = 1507328;
pub const CERT_FIND_SHA256_HASH: u32 = 1441792;
pub const CERT_FIND_SIGNATURE_HASH: u32 = 917504;
pub const CERT_FIND_SUBJECT_ATTR: u32 = 196615;
pub const CERT_FIND_SUBJECT_CERT: u32 = 720896;
pub const CERT_FIND_SUBJECT_INFO_ACCESS: u32 = 1245184;
pub const CERT_FIND_SUBJECT_NAME: u32 = 131079;
pub const CERT_FIND_SUBJECT_STR: u32 = 524295;
pub const CERT_FIND_SUBJECT_STR_A: u32 = 458759;
pub const CERT_FIND_SUBJECT_STR_W: u32 = 524295;
pub const CERT_FIND_VALID_CTL_USAGE_FLAG: u32 = 32;
pub const CERT_FIND_VALID_ENHKEY_USAGE_FLAG: u32 = 32;
pub const CERT_FIRST_RESERVED_PROP_ID: u32 = 130;
pub const CERT_FIRST_USER_PROP_ID: u32 = 32768;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_FORTEZZA_DATA_PROP {
    pub SerialNumber: [u8; 8],
    pub CertIndex: i32,
    pub CertLabel: [u8; 36],
}
impl Default for CERT_FORTEZZA_DATA_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_FORTEZZA_DATA_PROP_ID: u32 = 18;
pub const CERT_FRIENDLY_NAME_PROP_ID: u32 = 11;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_GENERAL_SUBTREE {
    pub Base: CERT_ALT_NAME_ENTRY,
    pub dwMinimum: u32,
    pub fMaximum: windows_sys::core::BOOL,
    pub dwMaximum: u32,
}
impl Default for CERT_GENERAL_SUBTREE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_GROUP_POLICY_SYSTEM_STORE_REGPATH: windows_sys::core::PCWSTR = windows_sys::core::w!("Software\\Policies\\Microsoft\\SystemCertificates");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_HASHED_URL {
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Hash: CRYPT_HASH_BLOB,
    pub pwszUrl: windows_sys::core::PWSTR,
}
impl Default for CERT_HASHED_URL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_HASH_PROP_ID: u32 = 3;
pub const CERT_HCRYPTPROV_OR_NCRYPT_KEY_HANDLE_PROP_ID: u32 = 79;
pub const CERT_HCRYPTPROV_TRANSFER_PROP_ID: u32 = 100;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_ID {
    pub dwIdChoice: u32,
    pub Anonymous: CERT_ID_0,
}
impl Default for CERT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CERT_ID_0 {
    pub IssuerSerialNumber: CERT_ISSUER_SERIAL_NUMBER,
    pub KeyId: CRYPT_HASH_BLOB,
    pub HashId: CRYPT_HASH_BLOB,
}
impl Default for CERT_ID_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_ID_ISSUER_SERIAL_NUMBER: u32 = 1;
pub const CERT_ID_KEY_IDENTIFIER: u32 = 2;
pub const CERT_ID_SHA1_HASH: u32 = 3;
pub const CERT_IE30_RESERVED_PROP_ID: u32 = 7;
pub const CERT_IE_DIRTY_FLAGS_REGPATH: windows_sys::core::PCWSTR = windows_sys::core::w!("Software\\Microsoft\\Cryptography\\IEDirtyFlags");
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CERT_INFO {
    pub dwVersion: u32,
    pub SerialNumber: CRYPT_INTEGER_BLOB,
    pub SignatureAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Issuer: CERT_NAME_BLOB,
    pub NotBefore: super::FILETIME,
    pub NotAfter: super::FILETIME,
    pub Subject: CERT_NAME_BLOB,
    pub SubjectPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
    pub IssuerUniqueId: CRYPT_BIT_BLOB,
    pub SubjectUniqueId: CRYPT_BIT_BLOB,
    pub cExtension: u32,
    pub rgExtension: PCERT_EXTENSION,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_INFO_EXTENSION_FLAG: u32 = 11;
pub const CERT_INFO_ISSUER_FLAG: u32 = 4;
pub const CERT_INFO_ISSUER_UNIQUE_ID_FLAG: u32 = 9;
pub const CERT_INFO_NOT_AFTER_FLAG: u32 = 6;
pub const CERT_INFO_NOT_BEFORE_FLAG: u32 = 5;
pub const CERT_INFO_SERIAL_NUMBER_FLAG: u32 = 2;
pub const CERT_INFO_SIGNATURE_ALGORITHM_FLAG: u32 = 3;
pub const CERT_INFO_SUBJECT_FLAG: u32 = 7;
pub const CERT_INFO_SUBJECT_PUBLIC_KEY_INFO_FLAG: u32 = 8;
pub const CERT_INFO_SUBJECT_UNIQUE_ID_FLAG: u32 = 10;
pub const CERT_INFO_VERSION_FLAG: u32 = 1;
pub const CERT_ISOLATED_KEY_PROP_ID: u32 = 118;
pub const CERT_ISSUER_CHAIN_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: u32 = 96;
pub const CERT_ISSUER_CHAIN_SIGN_HASH_CNG_ALG_PROP_ID: u32 = 95;
pub const CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID: u32 = 24;
pub const CERT_ISSUER_PUB_KEY_BIT_LENGTH_PROP_ID: u32 = 94;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_ISSUER_SERIAL_NUMBER {
    pub Issuer: CERT_NAME_BLOB,
    pub SerialNumber: CRYPT_INTEGER_BLOB,
}
pub const CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID: u32 = 28;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_KEYGEN_REQUEST_INFO {
    pub dwVersion: u32,
    pub SubjectPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
    pub pwszChallengeString: windows_sys::core::PWSTR,
}
impl Default for CERT_KEYGEN_REQUEST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_KEYGEN_REQUEST_V1: u32 = 0;
pub const CERT_KEY_AGREEMENT_KEY_USAGE: u32 = 8;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CERT_KEY_ATTRIBUTES_INFO {
    pub KeyId: CRYPT_DATA_BLOB,
    pub IntendedKeyUsage: CRYPT_BIT_BLOB,
    pub pPrivateKeyUsagePeriod: PCERT_PRIVATE_KEY_VALIDITY,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_KEY_ATTRIBUTES_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_KEY_CERT_SIGN_KEY_USAGE: u32 = 4;
pub const CERT_KEY_CLASSIFICATION_PROP_ID: u32 = 120;
#[repr(C)]
#[cfg(feature = "ncrypt")]
#[derive(Clone, Copy)]
pub struct CERT_KEY_CONTEXT {
    pub cbSize: u32,
    pub Anonymous: CERT_KEY_CONTEXT_0,
    pub dwKeySpec: u32,
}
#[cfg(feature = "ncrypt")]
impl Default for CERT_KEY_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ncrypt")]
#[derive(Clone, Copy)]
pub union CERT_KEY_CONTEXT_0 {
    pub hCryptProv: HCRYPTPROV,
    pub hNCryptKey: super::NCRYPT_KEY_HANDLE,
}
#[cfg(feature = "ncrypt")]
impl Default for CERT_KEY_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_KEY_CONTEXT_PROP_ID: u32 = 5;
pub const CERT_KEY_ENCIPHERMENT_KEY_USAGE: u32 = 32;
pub const CERT_KEY_IDENTIFIER_PROP_ID: u32 = 20;
pub const CERT_KEY_PROV_HANDLE_PROP_ID: u32 = 1;
pub const CERT_KEY_PROV_INFO_PROP_ID: u32 = 2;
pub const CERT_KEY_REPAIR_ATTEMPTED_PROP_ID: u32 = 103;
pub const CERT_KEY_SPEC_PROP_ID: u32 = 6;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_KEY_USAGE_RESTRICTION_INFO {
    pub cCertPolicyId: u32,
    pub rgCertPolicyId: PCERT_POLICY_ID,
    pub RestrictedKeyUsage: CRYPT_BIT_BLOB,
}
impl Default for CERT_KEY_USAGE_RESTRICTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_LAST_RESERVED_PROP_ID: u32 = 32767;
pub const CERT_LAST_USER_PROP_ID: u32 = 65535;
pub const CERT_LDAP_STORE_AREC_EXCLUSIVE_FLAG: u32 = 131072;
pub const CERT_LDAP_STORE_OPENED_FLAG: u32 = 262144;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_LDAP_STORE_OPENED_PARA {
    pub pvLdapSessionHandle: *mut core::ffi::c_void,
    pub pwszLdapUrl: windows_sys::core::PCWSTR,
}
impl Default for CERT_LDAP_STORE_OPENED_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_LDAP_STORE_SIGN_FLAG: u32 = 65536;
pub const CERT_LDAP_STORE_UNBIND_FLAG: u32 = 524288;
pub const CERT_LOCAL_MACHINE_SYSTEM_STORE_REGPATH: windows_sys::core::PCWSTR = windows_sys::core::w!("Software\\Microsoft\\SystemCertificates");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_LOGOTYPE_AUDIO {
    pub LogotypeDetails: CERT_LOGOTYPE_DETAILS,
    pub pLogotypeAudioInfo: PCERT_LOGOTYPE_AUDIO_INFO,
}
impl Default for CERT_LOGOTYPE_AUDIO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_LOGOTYPE_AUDIO_INFO {
    pub dwFileSize: u32,
    pub dwPlayTime: u32,
    pub dwChannels: u32,
    pub dwSampleRate: u32,
    pub pwszLanguage: windows_sys::core::PWSTR,
}
impl Default for CERT_LOGOTYPE_AUDIO_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_LOGOTYPE_BITS_IMAGE_RESOLUTION_CHOICE: u32 = 1;
pub const CERT_LOGOTYPE_COLOR_IMAGE_INFO_CHOICE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_LOGOTYPE_DATA {
    pub cLogotypeImage: u32,
    pub rgLogotypeImage: PCERT_LOGOTYPE_IMAGE,
    pub cLogotypeAudio: u32,
    pub rgLogotypeAudio: PCERT_LOGOTYPE_AUDIO,
}
impl Default for CERT_LOGOTYPE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_LOGOTYPE_DETAILS {
    pub pwszMimeType: windows_sys::core::PWSTR,
    pub cHashedUrl: u32,
    pub rgHashedUrl: PCERT_HASHED_URL,
}
impl Default for CERT_LOGOTYPE_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_LOGOTYPE_DIRECT_INFO_CHOICE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_LOGOTYPE_EXT_INFO {
    pub cCommunityLogo: u32,
    pub rgCommunityLogo: PCERT_LOGOTYPE_INFO,
    pub pIssuerLogo: PCERT_LOGOTYPE_INFO,
    pub pSubjectLogo: PCERT_LOGOTYPE_INFO,
    pub cOtherLogo: u32,
    pub rgOtherLogo: PCERT_OTHER_LOGOTYPE_INFO,
}
impl Default for CERT_LOGOTYPE_EXT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_LOGOTYPE_GRAY_SCALE_IMAGE_INFO_CHOICE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_LOGOTYPE_IMAGE {
    pub LogotypeDetails: CERT_LOGOTYPE_DETAILS,
    pub pLogotypeImageInfo: PCERT_LOGOTYPE_IMAGE_INFO,
}
impl Default for CERT_LOGOTYPE_IMAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_LOGOTYPE_IMAGE_INFO {
    pub dwLogotypeImageInfoChoice: u32,
    pub dwFileSize: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwLogotypeImageResolutionChoice: u32,
    pub Anonymous: CERT_LOGOTYPE_IMAGE_INFO_0,
    pub pwszLanguage: windows_sys::core::PWSTR,
}
impl Default for CERT_LOGOTYPE_IMAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CERT_LOGOTYPE_IMAGE_INFO_0 {
    pub dwNumBits: u32,
    pub dwTableSize: u32,
}
impl Default for CERT_LOGOTYPE_IMAGE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_LOGOTYPE_INDIRECT_INFO_CHOICE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_LOGOTYPE_INFO {
    pub dwLogotypeInfoChoice: u32,
    pub Anonymous: CERT_LOGOTYPE_INFO_0,
}
impl Default for CERT_LOGOTYPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CERT_LOGOTYPE_INFO_0 {
    pub pLogotypeDirectInfo: PCERT_LOGOTYPE_DATA,
    pub pLogotypeIndirectInfo: PCERT_LOGOTYPE_REFERENCE,
}
impl Default for CERT_LOGOTYPE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_LOGOTYPE_NO_IMAGE_RESOLUTION_CHOICE: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_LOGOTYPE_REFERENCE {
    pub cHashedUrl: u32,
    pub rgHashedUrl: PCERT_HASHED_URL,
}
impl Default for CERT_LOGOTYPE_REFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_LOGOTYPE_TABLE_SIZE_IMAGE_RESOLUTION_CHOICE: u32 = 2;
pub const CERT_MAX_ASN_ENCODED_DSS_SIGNATURE_LEN: u32 = 48;
pub const CERT_MD5_HASH_PROP_ID: u32 = 4;
pub const CERT_NAME_ATTR_TYPE: u32 = 3;
pub type CERT_NAME_BLOB = CRYPT_INTEGER_BLOB;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_NAME_CONSTRAINTS_INFO {
    pub cPermittedSubtree: u32,
    pub rgPermittedSubtree: PCERT_GENERAL_SUBTREE,
    pub cExcludedSubtree: u32,
    pub rgExcludedSubtree: PCERT_GENERAL_SUBTREE,
}
impl Default for CERT_NAME_CONSTRAINTS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_NAME_DISABLE_IE4_UTF8_FLAG: u32 = 65536;
pub const CERT_NAME_DNS_TYPE: u32 = 6;
pub const CERT_NAME_EMAIL_TYPE: u32 = 1;
pub const CERT_NAME_FRIENDLY_DISPLAY_TYPE: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_NAME_INFO {
    pub cRDN: u32,
    pub rgRDN: PCERT_RDN,
}
impl Default for CERT_NAME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_NAME_ISSUER_FLAG: u32 = 1;
pub const CERT_NAME_RDN_TYPE: u32 = 2;
pub const CERT_NAME_SEARCH_ALL_NAMES_FLAG: u32 = 2;
pub const CERT_NAME_SIMPLE_DISPLAY_TYPE: u32 = 4;
pub const CERT_NAME_STR_COMMA_FLAG: u32 = 67108864;
pub const CERT_NAME_STR_CRLF_FLAG: u32 = 134217728;
pub const CERT_NAME_STR_DISABLE_IE4_UTF8_FLAG: u32 = 65536;
pub const CERT_NAME_STR_DISABLE_UTF8_DIR_STR_FLAG: u32 = 1048576;
pub const CERT_NAME_STR_ENABLE_PUNYCODE_FLAG: u32 = 2097152;
pub const CERT_NAME_STR_ENABLE_T61_UNICODE_FLAG: u32 = 131072;
pub const CERT_NAME_STR_ENABLE_UTF8_UNICODE_FLAG: u32 = 262144;
pub const CERT_NAME_STR_FORCE_UTF8_DIR_STR_FLAG: u32 = 524288;
pub const CERT_NAME_STR_FORWARD_FLAG: u32 = 16777216;
pub const CERT_NAME_STR_NO_PLUS_FLAG: u32 = 536870912;
pub const CERT_NAME_STR_NO_QUOTING_FLAG: u32 = 268435456;
pub const CERT_NAME_STR_REVERSE_FLAG: u32 = 33554432;
pub const CERT_NAME_STR_SEMICOLON_FLAG: u32 = 1073741824;
pub const CERT_NAME_UPN_TYPE: u32 = 8;
pub const CERT_NAME_URL_TYPE: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_NAME_VALUE {
    pub dwValueType: u32,
    pub Value: CERT_RDN_VALUE_BLOB,
}
pub const CERT_NCRYPT_KEY_HANDLE_PROP_ID: u32 = 78;
pub const CERT_NCRYPT_KEY_HANDLE_TRANSFER_PROP_ID: u32 = 99;
pub const CERT_NCRYPT_KEY_SPEC: u32 = 4294967295;
pub const CERT_NEW_KEY_PROP_ID: u32 = 74;
pub const CERT_NEXT_UPDATE_LOCATION_PROP_ID: u32 = 10;
pub const CERT_NONCOMPLIANT_ROOT_URL_PROP_ID: u32 = 123;
pub const CERT_NON_REPUDIATION_KEY_USAGE: u32 = 64;
pub const CERT_NOT_BEFORE_ENHKEY_USAGE_PROP_ID: u32 = 127;
pub const CERT_NOT_BEFORE_FILETIME_PROP_ID: u32 = 126;
pub const CERT_NO_AUTO_EXPIRE_CHECK_PROP_ID: u32 = 77;
pub const CERT_NO_EXPIRE_NOTIFICATION_PROP_ID: u32 = 97;
pub const CERT_OCM_SUBCOMPONENTS_LOCAL_MACHINE_REGPATH: windows_sys::core::PCWSTR = windows_sys::core::w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Setup\\OC Manager\\Subcomponents");
pub const CERT_OCM_SUBCOMPONENTS_ROOT_AUTO_UPDATE_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("RootAutoUpdate");
pub const CERT_OCSP_CACHE_PREFIX_PROP_ID: u32 = 75;
pub const CERT_OCSP_MUST_STAPLE_PROP_ID: u32 = 121;
pub const CERT_OCSP_RESPONSE_PROP_ID: u32 = 70;
pub const CERT_OFFLINE_CRL_SIGN_KEY_USAGE: u32 = 2;
pub const CERT_OID_NAME_STR: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_OR_CRL_BLOB {
    pub dwChoice: u32,
    pub cbEncoded: u32,
    pub pbEncoded: *mut u8,
}
impl Default for CERT_OR_CRL_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_OR_CRL_BUNDLE {
    pub cItem: u32,
    pub rgItem: PCERT_OR_CRL_BLOB,
}
impl Default for CERT_OR_CRL_BUNDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_OTHER_LOGOTYPE_INFO {
    pub pszObjId: windows_sys::core::PSTR,
    pub LogotypeInfo: CERT_LOGOTYPE_INFO,
}
impl Default for CERT_OTHER_LOGOTYPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_OTHER_NAME {
    pub pszObjId: windows_sys::core::PSTR,
    pub Value: CRYPT_OBJID_BLOB,
}
impl Default for CERT_OTHER_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_PAIR {
    pub Forward: CERT_BLOB,
    pub Reverse: CERT_BLOB,
}
pub const CERT_PHYSICAL_STORE_ADD_ENABLE_FLAG: u32 = 1;
pub const CERT_PHYSICAL_STORE_AUTH_ROOT_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!(".AuthRoot");
pub const CERT_PHYSICAL_STORE_DEFAULT_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!(".Default");
pub const CERT_PHYSICAL_STORE_DS_USER_CERTIFICATE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!(".UserCertificate");
pub const CERT_PHYSICAL_STORE_ENTERPRISE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!(".Enterprise");
pub const CERT_PHYSICAL_STORE_GROUP_POLICY_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!(".GroupPolicy");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_PHYSICAL_STORE_INFO {
    pub cbSize: u32,
    pub pszOpenStoreProvider: windows_sys::core::PSTR,
    pub dwOpenEncodingType: u32,
    pub dwOpenFlags: u32,
    pub OpenParameters: CRYPT_DATA_BLOB,
    pub dwFlags: u32,
    pub dwPriority: u32,
}
impl Default for CERT_PHYSICAL_STORE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_PHYSICAL_STORE_INSERT_COMPUTER_NAME_ENABLE_FLAG: u32 = 8;
pub const CERT_PHYSICAL_STORE_LOCAL_MACHINE_GROUP_POLICY_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!(".LocalMachineGroupPolicy");
pub const CERT_PHYSICAL_STORE_LOCAL_MACHINE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!(".LocalMachine");
pub const CERT_PHYSICAL_STORE_OPEN_DISABLE_FLAG: u32 = 2;
pub const CERT_PHYSICAL_STORE_PREDEFINED_ENUM_FLAG: u32 = 1;
pub const CERT_PHYSICAL_STORE_REMOTE_OPEN_DISABLE_FLAG: u32 = 4;
pub const CERT_PHYSICAL_STORE_SMART_CARD_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!(".SmartCard");
pub const CERT_PIN_RULES_AUTO_UPDATE_ENCODED_CTL_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("PinRulesEncodedCtl");
pub const CERT_PIN_RULES_AUTO_UPDATE_LAST_SYNC_TIME_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("PinRulesLastSyncTime");
pub const CERT_PIN_RULES_AUTO_UPDATE_LIST_IDENTIFIER: windows_sys::core::PCWSTR = windows_sys::core::w!("PinRules_AutoUpdate_1");
pub const CERT_PIN_RULES_AUTO_UPDATE_SYNC_DELTA_TIME_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("PinRulesSyncDeltaTime");
pub const CERT_PIN_RULES_CAB_FILENAME: windows_sys::core::PCWSTR = windows_sys::core::w!("pinrulesstl.cab");
pub const CERT_PIN_RULES_CTL_FILENAME: windows_sys::core::PCWSTR = windows_sys::core::w!("pinrules.stl");
pub const CERT_PIN_RULES_CTL_FILENAME_A: windows_sys::core::PCSTR = windows_sys::core::s!("pinrules.stl");
pub const CERT_PIN_SHA256_HASH_PROP_ID: u32 = 124;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_POLICIES_INFO {
    pub cPolicyInfo: u32,
    pub rgPolicyInfo: *mut CERT_POLICY_INFO,
}
impl Default for CERT_POLICIES_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_POLICY95_QUALIFIER1 {
    pub pszPracticesReference: windows_sys::core::PWSTR,
    pub pszNoticeIdentifier: windows_sys::core::PSTR,
    pub pszNSINoticeIdentifier: windows_sys::core::PSTR,
    pub cCPSURLs: u32,
    pub rgCPSURLs: *mut CPS_URLS,
}
impl Default for CERT_POLICY95_QUALIFIER1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_POLICY_CONSTRAINTS_INFO {
    pub fRequireExplicitPolicy: windows_sys::core::BOOL,
    pub dwRequireExplicitPolicySkipCerts: u32,
    pub fInhibitPolicyMapping: windows_sys::core::BOOL,
    pub dwInhibitPolicyMappingSkipCerts: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_POLICY_ID {
    pub cCertPolicyElementId: u32,
    pub rgpszCertPolicyElementId: *mut windows_sys::core::PSTR,
}
impl Default for CERT_POLICY_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_POLICY_INFO {
    pub pszPolicyIdentifier: windows_sys::core::PSTR,
    pub cPolicyQualifier: u32,
    pub rgPolicyQualifier: *mut CERT_POLICY_QUALIFIER_INFO,
}
impl Default for CERT_POLICY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_POLICY_MAPPING {
    pub pszIssuerDomainPolicy: windows_sys::core::PSTR,
    pub pszSubjectDomainPolicy: windows_sys::core::PSTR,
}
impl Default for CERT_POLICY_MAPPING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_POLICY_MAPPINGS_INFO {
    pub cPolicyMapping: u32,
    pub rgPolicyMapping: PCERT_POLICY_MAPPING,
}
impl Default for CERT_POLICY_MAPPINGS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_POLICY_QUALIFIER_INFO {
    pub pszPolicyQualifierId: windows_sys::core::PSTR,
    pub Qualifier: CRYPT_OBJID_BLOB,
}
impl Default for CERT_POLICY_QUALIFIER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_POLICY_QUALIFIER_NOTICE_REFERENCE {
    pub pszOrganization: windows_sys::core::PSTR,
    pub cNoticeNumbers: u32,
    pub rgNoticeNumbers: *mut i32,
}
impl Default for CERT_POLICY_QUALIFIER_NOTICE_REFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_POLICY_QUALIFIER_USER_NOTICE {
    pub pNoticeReference: *mut CERT_POLICY_QUALIFIER_NOTICE_REFERENCE,
    pub pszDisplayText: windows_sys::core::PWSTR,
}
impl Default for CERT_POLICY_QUALIFIER_USER_NOTICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct CERT_PRIVATE_KEY_VALIDITY {
    pub NotBefore: super::FILETIME,
    pub NotAfter: super::FILETIME,
}
pub const CERT_PROT_ROOT_DISABLE_CURRENT_USER_FLAG: u32 = 1;
pub const CERT_PROT_ROOT_DISABLE_LM_AUTH_FLAG: u32 = 8;
pub const CERT_PROT_ROOT_DISABLE_NOT_DEFINED_NAME_CONSTRAINT_FLAG: u32 = 32;
pub const CERT_PROT_ROOT_DISABLE_NT_AUTH_REQUIRED_FLAG: u32 = 16;
pub const CERT_PROT_ROOT_DISABLE_PEER_TRUST: u32 = 65536;
pub const CERT_PROT_ROOT_FLAGS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("Flags");
pub const CERT_PROT_ROOT_INHIBIT_ADD_AT_INIT_FLAG: u32 = 2;
pub const CERT_PROT_ROOT_INHIBIT_PURGE_LM_FLAG: u32 = 4;
pub const CERT_PROT_ROOT_ONLY_LM_GPT_FLAG: u32 = 8;
pub const CERT_PROT_ROOT_PEER_USAGES_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("PeerUsages");
pub const CERT_PROT_ROOT_PEER_USAGES_VALUE_NAME_A: windows_sys::core::PCSTR = windows_sys::core::s!("PeerUsages");
pub const CERT_PUBKEY_ALG_PARA_PROP_ID: u32 = 22;
pub const CERT_PUBKEY_HASH_RESERVED_PROP_ID: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_PUBLIC_KEY_INFO {
    pub Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub PublicKey: CRYPT_BIT_BLOB,
}
pub const CERT_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: u32 = 93;
pub const CERT_PVK_FILE_PROP_ID: u32 = 12;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_QC_STATEMENT {
    pub pszStatementId: windows_sys::core::PSTR,
    pub StatementInfo: CRYPT_OBJID_BLOB,
}
impl Default for CERT_QC_STATEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_QC_STATEMENTS_EXT_INFO {
    pub cStatement: u32,
    pub rgStatement: PCERT_QC_STATEMENT,
}
impl Default for CERT_QC_STATEMENTS_EXT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_QUERY_CONTENT_CERT: u32 = 1;
pub const CERT_QUERY_CONTENT_CERT_PAIR: u32 = 13;
pub const CERT_QUERY_CONTENT_CRL: u32 = 3;
pub const CERT_QUERY_CONTENT_CTL: u32 = 2;
pub const CERT_QUERY_CONTENT_FLAG_ALL: u32 = 16382;
pub const CERT_QUERY_CONTENT_FLAG_ALL_ISSUER_CERT: u32 = 818;
pub const CERT_QUERY_CONTENT_FLAG_CERT: u32 = 2;
pub const CERT_QUERY_CONTENT_FLAG_CERT_PAIR: u32 = 8192;
pub const CERT_QUERY_CONTENT_FLAG_CRL: u32 = 8;
pub const CERT_QUERY_CONTENT_FLAG_CTL: u32 = 4;
pub const CERT_QUERY_CONTENT_FLAG_PFX: u32 = 4096;
pub const CERT_QUERY_CONTENT_FLAG_PFX_AND_LOAD: u32 = 16384;
pub const CERT_QUERY_CONTENT_FLAG_PKCS10: u32 = 2048;
pub const CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED: u32 = 256;
pub const CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED_EMBED: u32 = 1024;
pub const CERT_QUERY_CONTENT_FLAG_PKCS7_UNSIGNED: u32 = 512;
pub const CERT_QUERY_CONTENT_FLAG_SERIALIZED_CERT: u32 = 32;
pub const CERT_QUERY_CONTENT_FLAG_SERIALIZED_CRL: u32 = 128;
pub const CERT_QUERY_CONTENT_FLAG_SERIALIZED_CTL: u32 = 64;
pub const CERT_QUERY_CONTENT_FLAG_SERIALIZED_STORE: u32 = 16;
pub const CERT_QUERY_CONTENT_PFX: u32 = 12;
pub const CERT_QUERY_CONTENT_PFX_AND_LOAD: u32 = 14;
pub const CERT_QUERY_CONTENT_PKCS10: u32 = 11;
pub const CERT_QUERY_CONTENT_PKCS7_SIGNED: u32 = 8;
pub const CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED: u32 = 10;
pub const CERT_QUERY_CONTENT_PKCS7_UNSIGNED: u32 = 9;
pub const CERT_QUERY_CONTENT_SERIALIZED_CERT: u32 = 5;
pub const CERT_QUERY_CONTENT_SERIALIZED_CRL: u32 = 7;
pub const CERT_QUERY_CONTENT_SERIALIZED_CTL: u32 = 6;
pub const CERT_QUERY_CONTENT_SERIALIZED_STORE: u32 = 4;
pub const CERT_QUERY_FORMAT_ASN_ASCII_HEX_ENCODED: u32 = 3;
pub const CERT_QUERY_FORMAT_BASE64_ENCODED: u32 = 2;
pub const CERT_QUERY_FORMAT_BINARY: u32 = 1;
pub const CERT_QUERY_FORMAT_FLAG_ALL: u32 = 14;
pub const CERT_QUERY_FORMAT_FLAG_ASN_ASCII_HEX_ENCODED: u32 = 8;
pub const CERT_QUERY_FORMAT_FLAG_BASE64_ENCODED: u32 = 4;
pub const CERT_QUERY_FORMAT_FLAG_BINARY: u32 = 2;
pub const CERT_QUERY_OBJECT_BLOB: u32 = 2;
pub const CERT_QUERY_OBJECT_FILE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_RDN {
    pub cRDNAttr: u32,
    pub rgRDNAttr: PCERT_RDN_ATTR,
}
impl Default for CERT_RDN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_RDN_ANY_TYPE: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_RDN_ATTR {
    pub pszObjId: windows_sys::core::PSTR,
    pub dwValueType: u32,
    pub Value: CERT_RDN_VALUE_BLOB,
}
impl Default for CERT_RDN_ATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_RDN_BMP_STRING: u32 = 12;
pub const CERT_RDN_DISABLE_CHECK_TYPE_FLAG: u32 = 1073741824;
pub const CERT_RDN_DISABLE_IE4_UTF8_FLAG: u32 = 16777216;
pub const CERT_RDN_ENABLE_PUNYCODE_FLAG: u32 = 33554432;
pub const CERT_RDN_ENABLE_T61_UNICODE_FLAG: u32 = 2147483648;
pub const CERT_RDN_ENABLE_UTF8_UNICODE_FLAG: u32 = 536870912;
pub const CERT_RDN_ENCODED_BLOB: u32 = 1;
pub const CERT_RDN_FLAGS_MASK: u32 = 4278190080;
pub const CERT_RDN_FORCE_UTF8_UNICODE_FLAG: u32 = 268435456;
pub const CERT_RDN_GENERAL_STRING: u32 = 10;
pub const CERT_RDN_GRAPHIC_STRING: u32 = 8;
pub const CERT_RDN_IA5_STRING: u32 = 7;
pub const CERT_RDN_INT4_STRING: u32 = 11;
pub const CERT_RDN_ISO646_STRING: u32 = 9;
pub const CERT_RDN_NUMERIC_STRING: u32 = 3;
pub const CERT_RDN_OCTET_STRING: u32 = 2;
pub const CERT_RDN_PRINTABLE_STRING: u32 = 4;
pub const CERT_RDN_T61_STRING: u32 = 5;
pub const CERT_RDN_TELETEX_STRING: u32 = 5;
pub const CERT_RDN_TYPE_MASK: u32 = 255;
pub const CERT_RDN_UNICODE_STRING: u32 = 12;
pub const CERT_RDN_UNIVERSAL_STRING: u32 = 11;
pub const CERT_RDN_UTF8_STRING: u32 = 13;
pub type CERT_RDN_VALUE_BLOB = CRYPT_INTEGER_BLOB;
pub const CERT_RDN_VIDEOTEX_STRING: u32 = 6;
pub const CERT_RDN_VISIBLE_STRING: u32 = 9;
pub const CERT_REGISTRY_STORE_CLIENT_GPT_FLAG: u32 = 2147483648;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CERT_REGISTRY_STORE_CLIENT_GPT_PARA {
    pub hKeyBase: super::HKEY,
    pub pwszRegPath: windows_sys::core::PWSTR,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_REGISTRY_STORE_CLIENT_GPT_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_REGISTRY_STORE_EXTERNAL_FLAG: u32 = 1048576;
pub const CERT_REGISTRY_STORE_LM_GPT_FLAG: u32 = 16777216;
pub const CERT_REGISTRY_STORE_MY_IE_DIRTY_FLAG: u32 = 524288;
pub const CERT_REGISTRY_STORE_REMOTE_FLAG: u32 = 65536;
pub const CERT_REGISTRY_STORE_ROAMING_FLAG: u32 = 262144;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CERT_REGISTRY_STORE_ROAMING_PARA {
    pub hKey: super::HKEY,
    pub pwszStoreDirectory: windows_sys::core::PWSTR,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_REGISTRY_STORE_ROAMING_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_REGISTRY_STORE_SERIALIZED_FLAG: u32 = 131072;
pub const CERT_RENEWAL_PROP_ID: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_REQUEST_INFO {
    pub dwVersion: u32,
    pub Subject: CERT_NAME_BLOB,
    pub SubjectPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
    pub cAttribute: u32,
    pub rgAttribute: PCRYPT_ATTRIBUTE,
}
impl Default for CERT_REQUEST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_REQUEST_ORIGINATOR_PROP_ID: u32 = 71;
pub const CERT_REQUEST_V1: u32 = 0;
pub const CERT_RETRIEVE_BIOMETRIC_PREDEFINED_BASE_TYPE: windows_sys::core::PCSTR = 1000 as _;
pub const CERT_RETRIEVE_COMMUNITY_LOGO: windows_sys::core::PCSTR = 3 as _;
pub const CERT_RETRIEVE_ISSUER_LOGO: windows_sys::core::PCSTR = 1 as _;
pub const CERT_RETRIEVE_SUBJECT_LOGO: windows_sys::core::PCSTR = 2 as _;
pub const CERT_RETR_BEHAVIOR_FILE_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("AllowFileUrlScheme");
pub const CERT_RETR_BEHAVIOR_INET_AUTH_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("EnableInetUnknownAuth");
pub const CERT_RETR_BEHAVIOR_INET_STATUS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("EnableInetLocal");
pub const CERT_RETR_BEHAVIOR_LDAP_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("DisableLDAPSignAndEncrypt");
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CERT_REVOCATION_CHAIN_PARA {
    pub cbSize: u32,
    pub hChainEngine: HCERTCHAINENGINE,
    pub hAdditionalStore: HCERTSTORE,
    pub dwChainFlags: u32,
    pub dwUrlRetrievalTimeout: u32,
    pub pftCurrentTime: super::LPFILETIME,
    pub pftCacheResync: super::LPFILETIME,
    pub cbMaxUrlRetrievalByteCount: u32,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for CERT_REVOCATION_CHAIN_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CERT_REVOCATION_CRL_INFO {
    pub cbSize: u32,
    pub pBaseCrlContext: PCCRL_CONTEXT,
    pub pDeltaCrlContext: PCCRL_CONTEXT,
    pub pCrlEntry: PCRL_ENTRY,
    pub fDeltaCrlEntry: windows_sys::core::BOOL,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_REVOCATION_CRL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CERT_REVOCATION_INFO {
    pub cbSize: u32,
    pub dwRevocationResult: u32,
    pub pszRevocationOid: windows_sys::core::PCSTR,
    pub pvOidSpecificInfo: *mut core::ffi::c_void,
    pub fHasFreshnessTime: windows_sys::core::BOOL,
    pub dwFreshnessTime: u32,
    pub pCrlInfo: PCERT_REVOCATION_CRL_INFO,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_REVOCATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CERT_REVOCATION_PARA {
    pub cbSize: u32,
    pub pIssuerCert: PCCERT_CONTEXT,
    pub cCertStore: u32,
    pub rgCertStore: *mut HCERTSTORE,
    pub hCrlStore: HCERTSTORE,
    pub pftTimeToUse: super::LPFILETIME,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_REVOCATION_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_REVOCATION_STATUS {
    pub cbSize: u32,
    pub dwIndex: u32,
    pub dwError: u32,
    pub dwReason: u32,
    pub fHasFreshnessTime: windows_sys::core::BOOL,
    pub dwFreshnessTime: u32,
}
pub const CERT_ROOT_PROGRAM_CERT_POLICIES_PROP_ID: u32 = 83;
pub const CERT_ROOT_PROGRAM_CHAIN_POLICIES_PROP_ID: u32 = 105;
pub const CERT_ROOT_PROGRAM_FLAG_ADDRESS: u32 = 8;
pub const CERT_ROOT_PROGRAM_FLAG_LSC: u32 = 64;
pub const CERT_ROOT_PROGRAM_FLAG_ORG: u32 = 128;
pub const CERT_ROOT_PROGRAM_FLAG_OU: u32 = 16;
pub const CERT_ROOT_PROGRAM_FLAG_SUBJECT_LOGO: u32 = 32;
pub const CERT_ROOT_PROGRAM_NAME_CONSTRAINTS_PROP_ID: u32 = 84;
pub const CERT_SCARD_PIN_ID_PROP_ID: u32 = 90;
pub const CERT_SCARD_PIN_INFO_PROP_ID: u32 = 91;
pub const CERT_SCEP_CA_CERT_PROP_ID: u32 = 111;
pub const CERT_SCEP_ENCRYPT_HASH_CNG_ALG_PROP_ID: u32 = 114;
pub const CERT_SCEP_FLAGS_PROP_ID: u32 = 115;
pub const CERT_SCEP_GUID_PROP_ID: u32 = 116;
pub const CERT_SCEP_NONCE_PROP_ID: u32 = 113;
pub const CERT_SCEP_RA_ENCRYPTION_CERT_PROP_ID: u32 = 110;
pub const CERT_SCEP_RA_SIGNATURE_CERT_PROP_ID: u32 = 109;
pub const CERT_SCEP_SERVER_CERTS_PROP_ID: u32 = 108;
pub const CERT_SCEP_SIGNER_CERT_PROP_ID: u32 = 112;
pub const CERT_SELECT_ALLOW_DUPLICATES: u32 = 128;
pub const CERT_SELECT_ALLOW_EXPIRED: u32 = 1;
pub const CERT_SELECT_BY_ENHKEY_USAGE: u32 = 1;
pub const CERT_SELECT_BY_EXTENSION: u32 = 5;
pub const CERT_SELECT_BY_FRIENDLYNAME: u32 = 13;
pub const CERT_SELECT_BY_ISSUER_ATTR: u32 = 7;
pub const CERT_SELECT_BY_ISSUER_DISPLAYNAME: u32 = 12;
pub const CERT_SELECT_BY_ISSUER_NAME: u32 = 9;
pub const CERT_SELECT_BY_KEY_USAGE: u32 = 2;
pub const CERT_SELECT_BY_POLICY_OID: u32 = 3;
pub const CERT_SELECT_BY_PROV_NAME: u32 = 4;
pub const CERT_SELECT_BY_PUBLIC_KEY: u32 = 10;
pub const CERT_SELECT_BY_SUBJECT_ATTR: u32 = 8;
pub const CERT_SELECT_BY_SUBJECT_HOST_NAME: u32 = 6;
pub const CERT_SELECT_BY_THUMBPRINT: u32 = 14;
pub const CERT_SELECT_BY_TLS_SIGNATURES: u32 = 11;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CERT_SELECT_CHAIN_PARA {
    pub hChainEngine: HCERTCHAINENGINE,
    pub pTime: super::PFILETIME,
    pub hAdditionalStore: HCERTSTORE,
    pub pChainPara: PCERT_CHAIN_PARA,
    pub dwFlags: u32,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for CERT_SELECT_CHAIN_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_SELECT_CRITERIA {
    pub dwType: u32,
    pub cPara: u32,
    pub ppPara: *mut *mut core::ffi::c_void,
}
impl Default for CERT_SELECT_CRITERIA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_SELECT_DISALLOW_SELFSIGNED: u32 = 4;
pub const CERT_SELECT_HARDWARE_ONLY: u32 = 64;
pub const CERT_SELECT_HAS_KEY_FOR_KEY_EXCHANGE: u32 = 32;
pub const CERT_SELECT_HAS_KEY_FOR_SIGNATURE: u32 = 16;
pub const CERT_SELECT_HAS_PRIVATE_KEY: u32 = 8;
pub const CERT_SELECT_IGNORE_AUTOSELECT: u32 = 256;
pub const CERT_SELECT_LAST: u32 = 11;
pub const CERT_SELECT_MAX: u32 = 33;
pub const CERT_SELECT_MAX_PARA: u32 = 500;
pub const CERT_SELECT_TRUSTED_ROOT: u32 = 2;
pub const CERT_SEND_AS_TRUSTED_ISSUER_PROP_ID: u32 = 102;
pub const CERT_SERIALIZABLE_KEY_CONTEXT_PROP_ID: u32 = 117;
pub const CERT_SERIAL_CHAIN_PROP_ID: u32 = 119;
pub const CERT_SERVER_OCSP_RESPONSE_ASYNC_FLAG: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_SERVER_OCSP_RESPONSE_CONTEXT {
    pub cbSize: u32,
    pub pbEncodedOcspResponse: *mut u8,
    pub cbEncodedOcspResponse: u32,
}
impl Default for CERT_SERVER_OCSP_RESPONSE_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CERT_SERVER_OCSP_RESPONSE_OPEN_PARA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub pcbUsedSize: *mut u32,
    pub pwszOcspDirectory: windows_sys::core::PWSTR,
    pub pfnUpdateCallback: PFN_CERT_SERVER_OCSP_RESPONSE_UPDATE_CALLBACK,
    pub pvUpdateCallbackArg: *mut core::ffi::c_void,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_SERVER_OCSP_RESPONSE_OPEN_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_SERVER_OCSP_RESPONSE_OPEN_PARA_READ_FLAG: u32 = 1;
pub const CERT_SERVER_OCSP_RESPONSE_OPEN_PARA_WRITE_FLAG: u32 = 2;
pub const CERT_SET_KEY_CONTEXT_PROP_ID: u32 = 1;
pub const CERT_SET_KEY_PROV_HANDLE_PROP_ID: u32 = 1;
pub const CERT_SET_PROPERTY_IGNORE_PERSIST_ERROR_FLAG: u32 = 2147483648;
pub const CERT_SET_PROPERTY_INHIBIT_PERSIST_FLAG: u32 = 1073741824;
pub const CERT_SHA1_HASH_PROP_ID: u32 = 3;
pub const CERT_SHA1_SHA256_HASH_PROP_ID: u32 = 129;
pub const CERT_SHA256_HASH_PROP_ID: u32 = 107;
pub const CERT_SIGNATURE_HASH_PROP_ID: u32 = 15;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_SIGNED_CONTENT_INFO {
    pub ToBeSigned: CRYPT_DER_BLOB,
    pub SignatureAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Signature: CRYPT_BIT_BLOB,
}
pub const CERT_SIGN_HASH_CNG_ALG_PROP_ID: u32 = 89;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CERT_SIMPLE_CHAIN {
    pub cbSize: u32,
    pub TrustStatus: CERT_TRUST_STATUS,
    pub cElement: u32,
    pub rgpElement: *mut PCERT_CHAIN_ELEMENT,
    pub pTrustListInfo: PCERT_TRUST_LIST_INFO,
    pub fHasRevocationFreshnessTime: windows_sys::core::BOOL,
    pub dwRevocationFreshnessTime: u32,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_SIMPLE_CHAIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_SIMPLE_NAME_STR: u32 = 1;
pub const CERT_SMART_CARD_DATA_PROP_ID: u32 = 16;
pub const CERT_SMART_CARD_READER_NON_REMOVABLE_PROP_ID: u32 = 106;
pub const CERT_SMART_CARD_READER_PROP_ID: u32 = 101;
pub const CERT_SMART_CARD_ROOT_INFO_PROP_ID: u32 = 76;
pub const CERT_SOURCE_LOCATION_PROP_ID: u32 = 72;
pub const CERT_SOURCE_URL_PROP_ID: u32 = 73;
pub const CERT_SRV_OCSP_RESP_MAX_BEFORE_NEXT_UPDATE_SECONDS_DEFAULT: u32 = 14400;
pub const CERT_SRV_OCSP_RESP_MAX_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("SrvOcspRespMaxBeforeNextUpdateSeconds");
pub const CERT_SRV_OCSP_RESP_MAX_SYNC_CERT_FILE_SECONDS_DEFAULT: u32 = 3600;
pub const CERT_SRV_OCSP_RESP_MAX_SYNC_CERT_FILE_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("SrvOcspRespMaxSyncCertFileSeconds");
pub const CERT_SRV_OCSP_RESP_MIN_AFTER_NEXT_UPDATE_SECONDS_DEFAULT: u32 = 60;
pub const CERT_SRV_OCSP_RESP_MIN_AFTER_NEXT_UPDATE_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("SrvOcspRespMinAfterNextUpdateSeconds");
pub const CERT_SRV_OCSP_RESP_MIN_BEFORE_NEXT_UPDATE_SECONDS_DEFAULT: u32 = 120;
pub const CERT_SRV_OCSP_RESP_MIN_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("SrvOcspRespMinBeforeNextUpdateSeconds");
pub const CERT_SRV_OCSP_RESP_MIN_SYNC_CERT_FILE_SECONDS_DEFAULT: u32 = 5;
pub const CERT_SRV_OCSP_RESP_MIN_SYNC_CERT_FILE_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("SrvOcspRespMinSyncCertFileSeconds");
pub const CERT_SRV_OCSP_RESP_MIN_VALIDITY_SECONDS_DEFAULT: u32 = 600;
pub const CERT_SRV_OCSP_RESP_MIN_VALIDITY_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("SrvOcspRespMinValiditySeconds");
pub const CERT_SRV_OCSP_RESP_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_DEFAULT: u32 = 15000;
pub const CERT_SRV_OCSP_RESP_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("SrvOcspRespUrlRetrievalTimeoutMilliseconds");
pub const CERT_STORE_ADD_ALWAYS: u32 = 4;
pub const CERT_STORE_ADD_NEW: u32 = 1;
pub const CERT_STORE_ADD_NEWER: u32 = 6;
pub const CERT_STORE_ADD_NEWER_INHERIT_PROPERTIES: u32 = 7;
pub const CERT_STORE_ADD_REPLACE_EXISTING: u32 = 3;
pub const CERT_STORE_ADD_REPLACE_EXISTING_INHERIT_PROPERTIES: u32 = 5;
pub const CERT_STORE_ADD_USE_EXISTING: u32 = 2;
pub const CERT_STORE_ALL_CONTEXT_FLAG: i32 = -1;
pub const CERT_STORE_BACKUP_RESTORE_FLAG: u32 = 2048;
pub const CERT_STORE_BASE_CRL_FLAG: u32 = 256;
pub const CERT_STORE_CERTIFICATE_CONTEXT: u32 = 1;
pub const CERT_STORE_CERTIFICATE_CONTEXT_FLAG: u32 = 2;
pub const CERT_STORE_CREATE_NEW_FLAG: u32 = 8192;
pub const CERT_STORE_CRL_CONTEXT: u32 = 2;
pub const CERT_STORE_CRL_CONTEXT_FLAG: u32 = 4;
pub const CERT_STORE_CTL_CONTEXT: u32 = 3;
pub const CERT_STORE_CTL_CONTEXT_FLAG: u32 = 8;
pub const CERT_STORE_CTRL_AUTO_RESYNC: u32 = 4;
pub const CERT_STORE_CTRL_CANCEL_NOTIFY: u32 = 5;
pub const CERT_STORE_CTRL_COMMIT: u32 = 3;
pub const CERT_STORE_CTRL_COMMIT_CLEAR_FLAG: u32 = 2;
pub const CERT_STORE_CTRL_COMMIT_FORCE_FLAG: u32 = 1;
pub const CERT_STORE_CTRL_INHIBIT_DUPLICATE_HANDLE_FLAG: u32 = 1;
pub const CERT_STORE_CTRL_NOTIFY_CHANGE: u32 = 2;
pub const CERT_STORE_CTRL_RESYNC: u32 = 1;
pub const CERT_STORE_DEFER_CLOSE_UNTIL_LAST_FREE_FLAG: u32 = 4;
pub const CERT_STORE_DELETE_FLAG: u32 = 16;
pub const CERT_STORE_DELTA_CRL_FLAG: u32 = 512;
pub const CERT_STORE_ENUM_ARCHIVED_FLAG: u32 = 512;
pub const CERT_STORE_LOCALIZED_NAME_PROP_ID: u32 = 4096;
pub const CERT_STORE_MANIFOLD_FLAG: u32 = 256;
pub const CERT_STORE_MAXIMUM_ALLOWED_FLAG: u32 = 4096;
pub const CERT_STORE_NO_CRL_FLAG: u32 = 65536;
pub const CERT_STORE_NO_CRYPT_RELEASE_FLAG: u32 = 1;
pub const CERT_STORE_NO_ISSUER_FLAG: u32 = 131072;
pub const CERT_STORE_OPEN_EXISTING_FLAG: u32 = 16384;
pub const CERT_STORE_PROV_CLOSE_FUNC: u32 = 0;
pub const CERT_STORE_PROV_COLLECTION: windows_sys::core::PCSTR = 11 as _;
pub const CERT_STORE_PROV_CONTROL_FUNC: u32 = 13;
pub const CERT_STORE_PROV_DELETED_FLAG: u32 = 2;
pub const CERT_STORE_PROV_DELETE_CERT_FUNC: u32 = 3;
pub const CERT_STORE_PROV_DELETE_CRL_FUNC: u32 = 7;
pub const CERT_STORE_PROV_DELETE_CTL_FUNC: u32 = 11;
pub const CERT_STORE_PROV_EXTERNAL_FLAG: u32 = 1;
pub const CERT_STORE_PROV_FILE: windows_sys::core::PCSTR = 3 as _;
pub const CERT_STORE_PROV_FILENAME_A: windows_sys::core::PCSTR = 7 as _;
pub const CERT_STORE_PROV_FILENAME_W: windows_sys::core::PCSTR = 8 as _;
pub const CERT_STORE_PROV_FIND_CERT_FUNC: u32 = 14;
pub const CERT_STORE_PROV_FIND_CRL_FUNC: u32 = 17;
pub const CERT_STORE_PROV_FIND_CTL_FUNC: u32 = 20;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_STORE_PROV_FIND_INFO {
    pub cbSize: u32,
    pub dwMsgAndCertEncodingType: u32,
    pub dwFindFlags: u32,
    pub dwFindType: u32,
    pub pvFindPara: *const core::ffi::c_void,
}
impl Default for CERT_STORE_PROV_FIND_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_STORE_PROV_FREE_FIND_CERT_FUNC: u32 = 15;
pub const CERT_STORE_PROV_FREE_FIND_CRL_FUNC: u32 = 18;
pub const CERT_STORE_PROV_FREE_FIND_CTL_FUNC: u32 = 21;
pub const CERT_STORE_PROV_GET_CERT_PROPERTY_FUNC: u32 = 16;
pub const CERT_STORE_PROV_GET_CRL_PROPERTY_FUNC: u32 = 19;
pub const CERT_STORE_PROV_GET_CTL_PROPERTY_FUNC: u32 = 22;
pub const CERT_STORE_PROV_GP_SYSTEM_STORE_FLAG: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_STORE_PROV_INFO {
    pub cbSize: u32,
    pub cStoreProvFunc: u32,
    pub rgpvStoreProvFunc: *mut *mut core::ffi::c_void,
    pub hStoreProv: HCERTSTOREPROV,
    pub dwStoreProvFlags: u32,
    pub hStoreProvFuncAddr2: HCRYPTOIDFUNCADDR,
}
impl Default for CERT_STORE_PROV_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_STORE_PROV_LDAP_W: windows_sys::core::PCSTR = 16 as _;
pub const CERT_STORE_PROV_LM_SYSTEM_STORE_FLAG: u32 = 16;
pub const CERT_STORE_PROV_MEMORY: windows_sys::core::PCSTR = 2 as _;
pub const CERT_STORE_PROV_MSG: windows_sys::core::PCSTR = 1 as _;
pub const CERT_STORE_PROV_NO_PERSIST_FLAG: u32 = 4;
pub const CERT_STORE_PROV_PHYSICAL_W: windows_sys::core::PCSTR = 14 as _;
pub const CERT_STORE_PROV_PKCS12: windows_sys::core::PCSTR = 17 as _;
pub const CERT_STORE_PROV_PKCS7: windows_sys::core::PCSTR = 5 as _;
pub const CERT_STORE_PROV_READ_CERT_FUNC: u32 = 1;
pub const CERT_STORE_PROV_READ_CRL_FUNC: u32 = 5;
pub const CERT_STORE_PROV_READ_CTL_FUNC: u32 = 9;
pub const CERT_STORE_PROV_REG: windows_sys::core::PCSTR = 4 as _;
pub const CERT_STORE_PROV_SERIALIZED: windows_sys::core::PCSTR = 6 as _;
pub const CERT_STORE_PROV_SET_CERT_PROPERTY_FUNC: u32 = 4;
pub const CERT_STORE_PROV_SET_CRL_PROPERTY_FUNC: u32 = 8;
pub const CERT_STORE_PROV_SET_CTL_PROPERTY_FUNC: u32 = 12;
pub const CERT_STORE_PROV_SHARED_USER_FLAG: u32 = 64;
pub const CERT_STORE_PROV_SMART_CARD_W: windows_sys::core::PCSTR = 15 as _;
pub const CERT_STORE_PROV_SYSTEM_A: windows_sys::core::PCSTR = 9 as _;
pub const CERT_STORE_PROV_SYSTEM_REGISTRY_A: windows_sys::core::PCSTR = 12 as _;
pub const CERT_STORE_PROV_SYSTEM_REGISTRY_W: windows_sys::core::PCSTR = 13 as _;
pub const CERT_STORE_PROV_SYSTEM_STORE_FLAG: u32 = 8;
pub const CERT_STORE_PROV_SYSTEM_W: windows_sys::core::PCSTR = 10 as _;
pub const CERT_STORE_PROV_WRITE_ADD_FLAG: u32 = 1;
pub const CERT_STORE_PROV_WRITE_CERT_FUNC: u32 = 2;
pub const CERT_STORE_PROV_WRITE_CRL_FUNC: u32 = 6;
pub const CERT_STORE_PROV_WRITE_CTL_FUNC: u32 = 10;
pub const CERT_STORE_READONLY_FLAG: u32 = 32768;
pub const CERT_STORE_REVOCATION_FLAG: u32 = 4;
pub const CERT_STORE_SAVE_AS_PKCS12: u32 = 3;
pub const CERT_STORE_SAVE_AS_PKCS7: u32 = 2;
pub const CERT_STORE_SAVE_AS_STORE: u32 = 1;
pub const CERT_STORE_SAVE_TO_FILE: u32 = 1;
pub const CERT_STORE_SAVE_TO_FILENAME: u32 = 4;
pub const CERT_STORE_SAVE_TO_FILENAME_A: u32 = 3;
pub const CERT_STORE_SAVE_TO_FILENAME_W: u32 = 4;
pub const CERT_STORE_SAVE_TO_MEMORY: u32 = 2;
pub const CERT_STORE_SET_LOCALIZED_NAME_FLAG: u32 = 2;
pub const CERT_STORE_SHARE_CONTEXT_FLAG: u32 = 128;
pub const CERT_STORE_SHARE_STORE_FLAG: u32 = 64;
pub const CERT_STORE_SIGNATURE_FLAG: u32 = 1;
pub const CERT_STORE_TIME_VALIDITY_FLAG: u32 = 2;
pub const CERT_STORE_UNSAFE_PHYSICAL_FLAG: u32 = 32;
pub const CERT_STORE_UPDATE_KEYID_FLAG: u32 = 1024;
pub const CERT_STRONG_SIGN_ECDSA_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("ECDSA");
pub const CERT_STRONG_SIGN_ENABLE_CRL_CHECK: u32 = 1;
pub const CERT_STRONG_SIGN_ENABLE_OCSP_CHECK: u32 = 2;
pub const CERT_STRONG_SIGN_OID_INFO_CHOICE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_STRONG_SIGN_PARA {
    pub cbSize: u32,
    pub dwInfoChoice: u32,
    pub Anonymous: CERT_STRONG_SIGN_PARA_0,
}
impl Default for CERT_STRONG_SIGN_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CERT_STRONG_SIGN_PARA_0 {
    pub pvInfo: *mut core::ffi::c_void,
    pub pSerializedInfo: PCERT_STRONG_SIGN_SERIALIZED_INFO,
    pub pszOID: windows_sys::core::PSTR,
}
impl Default for CERT_STRONG_SIGN_PARA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_STRONG_SIGN_SERIALIZED_INFO {
    pub dwFlags: u32,
    pub pwszCNGSignHashAlgids: windows_sys::core::PWSTR,
    pub pwszCNGPubKeyMinBitLengths: windows_sys::core::PWSTR,
}
impl Default for CERT_STRONG_SIGN_SERIALIZED_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_STRONG_SIGN_SERIALIZED_INFO_CHOICE: u32 = 1;
pub const CERT_SUBJECT_DISABLE_CRL_PROP_ID: u32 = 86;
pub type CERT_SUBJECT_INFO_ACCESS = CERT_AUTHORITY_INFO_ACCESS;
pub const CERT_SUBJECT_INFO_ACCESS_PROP_ID: u32 = 80;
pub const CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: u32 = 29;
pub const CERT_SUBJECT_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: u32 = 85;
pub const CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID: u32 = 25;
pub const CERT_SUBJECT_PUB_KEY_BIT_LENGTH_PROP_ID: u32 = 92;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_SUPPORTED_ALGORITHM_INFO {
    pub Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub IntendedKeyUsage: CRYPT_BIT_BLOB,
    pub IntendedCertPolicies: CERT_POLICIES_INFO,
}
pub const CERT_SYSTEM_STORE_CURRENT_SERVICE: u32 = 262144;
pub const CERT_SYSTEM_STORE_CURRENT_SERVICE_ID: u32 = 4;
pub const CERT_SYSTEM_STORE_CURRENT_USER: u32 = 65536;
pub const CERT_SYSTEM_STORE_CURRENT_USER_GROUP_POLICY: u32 = 458752;
pub const CERT_SYSTEM_STORE_CURRENT_USER_GROUP_POLICY_ID: u32 = 7;
pub const CERT_SYSTEM_STORE_CURRENT_USER_ID: u32 = 1;
pub const CERT_SYSTEM_STORE_DEFER_READ_FLAG: u32 = 536870912;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_SYSTEM_STORE_INFO {
    pub cbSize: u32,
}
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE: u32 = 131072;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRISE: u32 = 589824;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRISE_ID: u32 = 9;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_GROUP_POLICY: u32 = 524288;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_GROUP_POLICY_ID: u32 = 8;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_ID: u32 = 2;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_WCOS: u32 = 655360;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_WCOS_ID: u32 = 10;
pub const CERT_SYSTEM_STORE_LOCATION_MASK: u32 = 16711680;
pub const CERT_SYSTEM_STORE_LOCATION_SHIFT: u32 = 16;
pub const CERT_SYSTEM_STORE_MASK: u32 = 4294901760;
pub const CERT_SYSTEM_STORE_RELOCATE_FLAG: u32 = 2147483648;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CERT_SYSTEM_STORE_RELOCATE_PARA {
    pub Anonymous: CERT_SYSTEM_STORE_RELOCATE_PARA_0,
    pub Anonymous2: CERT_SYSTEM_STORE_RELOCATE_PARA_1,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_SYSTEM_STORE_RELOCATE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union CERT_SYSTEM_STORE_RELOCATE_PARA_0 {
    pub hKeyBase: super::HKEY,
    pub pvBase: *mut core::ffi::c_void,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_SYSTEM_STORE_RELOCATE_PARA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union CERT_SYSTEM_STORE_RELOCATE_PARA_1 {
    pub pvSystemStore: *mut core::ffi::c_void,
    pub pszSystemStore: windows_sys::core::PCSTR,
    pub pwszSystemStore: windows_sys::core::PCWSTR,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_SYSTEM_STORE_RELOCATE_PARA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_SYSTEM_STORE_SERVICES: u32 = 327680;
pub const CERT_SYSTEM_STORE_SERVICES_ID: u32 = 5;
pub const CERT_SYSTEM_STORE_UNPROTECTED_FLAG: u32 = 1073741824;
pub const CERT_SYSTEM_STORE_USERS: u32 = 393216;
pub const CERT_SYSTEM_STORE_USERS_ID: u32 = 6;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_TEMPLATE_EXT {
    pub pszObjId: windows_sys::core::PSTR,
    pub dwMajorVersion: u32,
    pub fMinorVersion: windows_sys::core::BOOL,
    pub dwMinorVersion: u32,
}
impl Default for CERT_TEMPLATE_EXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_TIMESTAMP_HASH_USE_TYPE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_TPM_SPECIFICATION_INFO {
    pub pwszFamily: windows_sys::core::PWSTR,
    pub dwLevel: u32,
    pub dwRevision: u32,
}
impl Default for CERT_TPM_SPECIFICATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_TRUST_AUTO_UPDATE_CA_REVOCATION: u32 = 16;
pub const CERT_TRUST_AUTO_UPDATE_END_REVOCATION: u32 = 32;
pub const CERT_TRUST_BEFORE_DISALLOWED_CA_FILETIME: u32 = 2097152;
pub const CERT_TRUST_CTL_IS_NOT_SIGNATURE_VALID: u32 = 262144;
pub const CERT_TRUST_CTL_IS_NOT_TIME_VALID: u32 = 131072;
pub const CERT_TRUST_CTL_IS_NOT_VALID_FOR_USAGE: u32 = 524288;
pub const CERT_TRUST_HAS_ALLOW_WEAK_SIGNATURE: u32 = 131072;
pub const CERT_TRUST_HAS_AUTO_UPDATE_WEAK_SIGNATURE: u32 = 32768;
pub const CERT_TRUST_HAS_CRL_VALIDITY_EXTENDED: u32 = 4096;
pub const CERT_TRUST_HAS_EXACT_MATCH_ISSUER: u32 = 1;
pub const CERT_TRUST_HAS_EXCLUDED_NAME_CONSTRAINT: u32 = 32768;
pub const CERT_TRUST_HAS_ISSUANCE_CHAIN_POLICY: u32 = 512;
pub const CERT_TRUST_HAS_KEY_MATCH_ISSUER: u32 = 2;
pub const CERT_TRUST_HAS_MIN_TELEMETRY_RSA: u32 = 4194304;
pub const CERT_TRUST_HAS_MIN_WEAK_RSA: u32 = 8388608;
pub const CERT_TRUST_HAS_NAME_MATCH_ISSUER: u32 = 4;
pub const CERT_TRUST_HAS_NOT_DEFINED_NAME_CONSTRAINT: u32 = 8192;
pub const CERT_TRUST_HAS_NOT_PERMITTED_NAME_CONSTRAINT: u32 = 16384;
pub const CERT_TRUST_HAS_NOT_SUPPORTED_CRITICAL_EXT: u32 = 134217728;
pub const CERT_TRUST_HAS_NOT_SUPPORTED_NAME_CONSTRAINT: u32 = 4096;
pub const CERT_TRUST_HAS_PREFERRED_ISSUER: u32 = 256;
pub const CERT_TRUST_HAS_VALID_NAME_CONSTRAINTS: u32 = 1024;
pub const CERT_TRUST_HAS_WEAK_HYGIENE: u32 = 2097152;
pub const CERT_TRUST_HAS_WEAK_SIGNATURE: u32 = 1048576;
pub const CERT_TRUST_INVALID_BASIC_CONSTRAINTS: u32 = 1024;
pub const CERT_TRUST_INVALID_EXTENSION: u32 = 256;
pub const CERT_TRUST_INVALID_NAME_CONSTRAINTS: u32 = 2048;
pub const CERT_TRUST_INVALID_POLICY_CONSTRAINTS: u32 = 512;
pub const CERT_TRUST_IS_CA_TRUSTED: u32 = 16384;
pub const CERT_TRUST_IS_COMPLEX_CHAIN: u32 = 65536;
pub const CERT_TRUST_IS_CYCLIC: u32 = 128;
pub const CERT_TRUST_IS_EXPLICIT_DISTRUST: u32 = 67108864;
pub const CERT_TRUST_IS_FROM_EXCLUSIVE_TRUST_STORE: u32 = 8192;
pub const CERT_TRUST_IS_KEY_ROLLOVER: u32 = 128;
pub const CERT_TRUST_IS_NOT_SIGNATURE_VALID: u32 = 8;
pub const CERT_TRUST_IS_NOT_TIME_NESTED: u32 = 2;
pub const CERT_TRUST_IS_NOT_TIME_VALID: u32 = 1;
pub const CERT_TRUST_IS_NOT_VALID_FOR_USAGE: u32 = 16;
pub const CERT_TRUST_IS_OFFLINE_REVOCATION: u32 = 16777216;
pub const CERT_TRUST_IS_PARTIAL_CHAIN: u32 = 65536;
pub const CERT_TRUST_IS_PEER_TRUSTED: u32 = 2048;
pub const CERT_TRUST_IS_REVOKED: u32 = 4;
pub const CERT_TRUST_IS_SELF_SIGNED: u32 = 8;
pub const CERT_TRUST_IS_UNTRUSTED_ROOT: u32 = 32;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CERT_TRUST_LIST_INFO {
    pub cbSize: u32,
    pub pCtlEntry: PCTL_ENTRY,
    pub pCtlContext: PCCTL_CONTEXT,
}
#[cfg(feature = "minwindef")]
impl Default for CERT_TRUST_LIST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_TRUST_NO_ERROR: u32 = 0;
pub const CERT_TRUST_NO_ISSUANCE_CHAIN_POLICY: u32 = 33554432;
pub const CERT_TRUST_NO_OCSP_FAILOVER_TO_CRL: u32 = 64;
pub const CERT_TRUST_NO_TIME_CHECK: u32 = 33554432;
pub const CERT_TRUST_PUB_ALLOW_END_USER_TRUST: u32 = 0;
pub const CERT_TRUST_PUB_ALLOW_ENTERPRISE_ADMIN_TRUST: u32 = 2;
pub const CERT_TRUST_PUB_ALLOW_MACHINE_ADMIN_TRUST: u32 = 1;
pub const CERT_TRUST_PUB_ALLOW_TRUST_MASK: u32 = 3;
pub const CERT_TRUST_PUB_AUTHENTICODE_FLAGS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("AuthenticodeFlags");
pub const CERT_TRUST_PUB_CHECK_PUBLISHER_REV_FLAG: u32 = 256;
pub const CERT_TRUST_PUB_CHECK_TIMESTAMP_REV_FLAG: u32 = 512;
pub const CERT_TRUST_REVOCATION_STATUS_UNKNOWN: u32 = 64;
pub const CERT_TRUST_SSL_HANDSHAKE_OCSP: u32 = 262144;
pub const CERT_TRUST_SSL_RECONNECT_OCSP: u32 = 1048576;
pub const CERT_TRUST_SSL_TIME_VALID: u32 = 16777216;
pub const CERT_TRUST_SSL_TIME_VALID_OCSP: u32 = 524288;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_TRUST_STATUS {
    pub dwErrorStatus: u32,
    pub dwInfoStatus: u32,
}
pub const CERT_UNICODE_ATTR_ERR_INDEX_MASK: u32 = 63;
pub const CERT_UNICODE_ATTR_ERR_INDEX_SHIFT: u32 = 16;
pub const CERT_UNICODE_IS_RDN_ATTRS_FLAG: u32 = 1;
pub const CERT_UNICODE_RDN_ERR_INDEX_MASK: u32 = 1023;
pub const CERT_UNICODE_RDN_ERR_INDEX_SHIFT: u32 = 22;
pub const CERT_UNICODE_VALUE_ERR_INDEX_MASK: u32 = 65535;
pub const CERT_UNICODE_VALUE_ERR_INDEX_SHIFT: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_USAGE_MATCH {
    pub dwType: u32,
    pub Usage: CERT_ENHKEY_USAGE,
}
pub const CERT_V1: u32 = 0;
pub const CERT_V2: u32 = 1;
pub const CERT_V3: u32 = 2;
pub const CERT_VERIFY_ALLOW_MORE_USAGE_FLAG: u32 = 8;
pub const CERT_VERIFY_CACHE_ONLY_BASED_REVOCATION: u32 = 2;
pub const CERT_VERIFY_INHIBIT_CTL_UPDATE_FLAG: u32 = 1;
pub const CERT_VERIFY_NO_TIME_CHECK_FLAG: u32 = 4;
pub const CERT_VERIFY_REV_ACCUMULATIVE_TIMEOUT_FLAG: u32 = 4;
pub const CERT_VERIFY_REV_CHAIN_FLAG: u32 = 1;
pub const CERT_VERIFY_REV_NO_OCSP_FAILOVER_TO_CRL_FLAG: u32 = 16;
pub const CERT_VERIFY_REV_SERVER_OCSP_FLAG: u32 = 8;
pub const CERT_VERIFY_REV_SERVER_OCSP_WIRE_ONLY_FLAG: u32 = 32;
pub const CERT_VERIFY_TRUSTED_SIGNERS_FLAG: u32 = 2;
pub const CERT_VERIFY_UPDATED_CTL_FLAG: u32 = 1;
pub const CERT_X500_NAME_STR: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CERT_X942_DH_PARAMETERS {
    pub p: CRYPT_UINT_BLOB,
    pub g: CRYPT_UINT_BLOB,
    pub q: CRYPT_UINT_BLOB,
    pub j: CRYPT_UINT_BLOB,
    pub pValidationParams: PCERT_X942_DH_VALIDATION_PARAMS,
}
impl Default for CERT_X942_DH_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CERT_X942_DH_VALIDATION_PARAMS {
    pub seed: CRYPT_BIT_BLOB,
    pub pgenCounter: u32,
}
pub const CERT_XML_NAME_STR: u32 = 4;
pub const CMC_ADD_ATTRIBUTES: windows_sys::core::PCSTR = 63 as _;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMC_ADD_ATTRIBUTES_INFO {
    pub dwCmcDataReference: u32,
    pub cCertReference: u32,
    pub rgdwCertReference: *mut u32,
    pub cAttribute: u32,
    pub rgAttribute: PCRYPT_ATTRIBUTE,
}
impl Default for CMC_ADD_ATTRIBUTES_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMC_ADD_EXTENSIONS: windows_sys::core::PCSTR = 62 as _;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMC_ADD_EXTENSIONS_INFO {
    pub dwCmcDataReference: u32,
    pub cCertReference: u32,
    pub rgdwCertReference: *mut u32,
    pub cExtension: u32,
    pub rgExtension: PCERT_EXTENSION,
}
impl Default for CMC_ADD_EXTENSIONS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMC_DATA: windows_sys::core::PCSTR = 59 as _;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMC_DATA_INFO {
    pub cTaggedAttribute: u32,
    pub rgTaggedAttribute: PCMC_TAGGED_ATTRIBUTE,
    pub cTaggedRequest: u32,
    pub rgTaggedRequest: PCMC_TAGGED_REQUEST,
    pub cTaggedContentInfo: u32,
    pub rgTaggedContentInfo: PCMC_TAGGED_CONTENT_INFO,
    pub cTaggedOtherMsg: u32,
    pub rgTaggedOtherMsg: PCMC_TAGGED_OTHER_MSG,
}
impl Default for CMC_DATA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMC_FAIL_BAD_ALG: u32 = 0;
pub const CMC_FAIL_BAD_CERT_ID: u32 = 4;
pub const CMC_FAIL_BAD_IDENTITY: u32 = 7;
pub const CMC_FAIL_BAD_MESSAGE_CHECK: u32 = 1;
pub const CMC_FAIL_BAD_REQUEST: u32 = 2;
pub const CMC_FAIL_BAD_TIME: u32 = 3;
pub const CMC_FAIL_INTERNAL_CA_ERROR: u32 = 11;
pub const CMC_FAIL_MUST_ARCHIVE_KEYS: u32 = 6;
pub const CMC_FAIL_NO_KEY_REUSE: u32 = 10;
pub const CMC_FAIL_POP_FAILED: u32 = 9;
pub const CMC_FAIL_POP_REQUIRED: u32 = 8;
pub const CMC_FAIL_TRY_LATER: u32 = 12;
pub const CMC_FAIL_UNSUPORTED_EXT: u32 = 5;
pub const CMC_OTHER_INFO_FAIL_CHOICE: u32 = 1;
pub const CMC_OTHER_INFO_NO_CHOICE: u32 = 0;
pub const CMC_OTHER_INFO_PEND_CHOICE: u32 = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct CMC_PEND_INFO {
    pub PendToken: CRYPT_DATA_BLOB,
    pub PendTime: super::FILETIME,
}
pub const CMC_RESPONSE: windows_sys::core::PCSTR = 60 as _;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMC_RESPONSE_INFO {
    pub cTaggedAttribute: u32,
    pub rgTaggedAttribute: PCMC_TAGGED_ATTRIBUTE,
    pub cTaggedContentInfo: u32,
    pub rgTaggedContentInfo: PCMC_TAGGED_CONTENT_INFO,
    pub cTaggedOtherMsg: u32,
    pub rgTaggedOtherMsg: PCMC_TAGGED_OTHER_MSG,
}
impl Default for CMC_RESPONSE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMC_STATUS: windows_sys::core::PCSTR = 61 as _;
pub const CMC_STATUS_CONFIRM_REQUIRED: u32 = 5;
pub const CMC_STATUS_FAILED: u32 = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CMC_STATUS_INFO {
    pub dwStatus: u32,
    pub cBodyList: u32,
    pub rgdwBodyList: *mut u32,
    pub pwszStatusString: windows_sys::core::PWSTR,
    pub dwOtherInfoChoice: u32,
    pub Anonymous: CMC_STATUS_INFO_0,
}
#[cfg(feature = "minwindef")]
impl Default for CMC_STATUS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union CMC_STATUS_INFO_0 {
    pub dwFailInfo: u32,
    pub pPendInfo: PCMC_PEND_INFO,
}
#[cfg(feature = "minwindef")]
impl Default for CMC_STATUS_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMC_STATUS_NO_SUPPORT: u32 = 4;
pub const CMC_STATUS_PENDING: u32 = 3;
pub const CMC_STATUS_SUCCESS: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CMC_TAGGED_ATTRIBUTE {
    pub dwBodyPartID: u32,
    pub Attribute: CRYPT_ATTRIBUTE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CMC_TAGGED_CERT_REQUEST {
    pub dwBodyPartID: u32,
    pub SignedCertRequest: CRYPT_DER_BLOB,
}
pub const CMC_TAGGED_CERT_REQUEST_CHOICE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CMC_TAGGED_CONTENT_INFO {
    pub dwBodyPartID: u32,
    pub EncodedContentInfo: CRYPT_DER_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMC_TAGGED_OTHER_MSG {
    pub dwBodyPartID: u32,
    pub pszObjId: windows_sys::core::PSTR,
    pub Value: CRYPT_OBJID_BLOB,
}
impl Default for CMC_TAGGED_OTHER_MSG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMC_TAGGED_REQUEST {
    pub dwTaggedRequestChoice: u32,
    pub Anonymous: CMC_TAGGED_REQUEST_0,
}
impl Default for CMC_TAGGED_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CMC_TAGGED_REQUEST_0 {
    pub pTaggedCertRequest: PCMC_TAGGED_CERT_REQUEST,
}
impl Default for CMC_TAGGED_REQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_ALL_FLAGS: i32 = -1;
pub type CMSG_ATTR = CRYPT_ATTRIBUTES;
pub const CMSG_ATTR_CERT_COUNT_PARAM: u32 = 31;
pub const CMSG_ATTR_CERT_PARAM: u32 = 32;
pub const CMSG_AUTHENTICATED_ATTRIBUTES_FLAG: u32 = 8;
pub const CMSG_BARE_CONTENT_FLAG: u32 = 1;
pub const CMSG_BARE_CONTENT_PARAM: u32 = 3;
pub const CMSG_CERT_COUNT_PARAM: u32 = 11;
pub const CMSG_CERT_PARAM: u32 = 12;
pub const CMSG_CMS_ENCAPSULATED_CONTENT_FLAG: u32 = 64;
pub const CMSG_CMS_ENCAPSULATED_CTL_FLAG: u32 = 32768;
pub const CMSG_CMS_RECIPIENT_COUNT_PARAM: u32 = 33;
pub const CMSG_CMS_RECIPIENT_ENCRYPTED_KEY_INDEX_PARAM: u32 = 35;
pub const CMSG_CMS_RECIPIENT_INDEX_PARAM: u32 = 34;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CMSG_CMS_RECIPIENT_INFO {
    pub dwRecipientChoice: u32,
    pub Anonymous: CMSG_CMS_RECIPIENT_INFO_0,
}
#[cfg(feature = "minwindef")]
impl Default for CMSG_CMS_RECIPIENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union CMSG_CMS_RECIPIENT_INFO_0 {
    pub pKeyTrans: PCMSG_KEY_TRANS_RECIPIENT_INFO,
    pub pKeyAgree: PCMSG_KEY_AGREE_RECIPIENT_INFO,
    pub pMailList: PCMSG_MAIL_LIST_RECIPIENT_INFO,
}
#[cfg(feature = "minwindef")]
impl Default for CMSG_CMS_RECIPIENT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_CMS_RECIPIENT_INFO_PARAM: u32 = 36;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_CMS_SIGNER_INFO {
    pub dwVersion: u32,
    pub SignerId: CERT_ID,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub HashEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedHash: CRYPT_DATA_BLOB,
    pub AuthAttrs: CRYPT_ATTRIBUTES,
    pub UnauthAttrs: CRYPT_ATTRIBUTES,
}
impl Default for CMSG_CMS_SIGNER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_CMS_SIGNER_INFO_PARAM: u32 = 39;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "ncrypt"))]
#[derive(Clone, Copy)]
pub struct CMSG_CNG_CONTENT_DECRYPT_INFO {
    pub cbSize: u32,
    pub ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pfnAlloc: PFN_CMSG_ALLOC,
    pub pfnFree: PFN_CMSG_FREE,
    pub hNCryptKey: super::NCRYPT_KEY_HANDLE,
    pub pbContentEncryptKey: *mut u8,
    pub cbContentEncryptKey: u32,
    pub hCNGContentEncryptKey: super::BCRYPT_KEY_HANDLE,
    pub pbCNGContentEncryptKeyObject: *mut u8,
}
#[cfg(all(feature = "bcrypt", feature = "ncrypt"))]
impl Default for CMSG_CNG_CONTENT_DECRYPT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_COMPUTED_HASH_PARAM: u32 = 22;
pub const CMSG_CONTENTS_OCTETS_FLAG: u32 = 16;
pub const CMSG_CONTENT_ENCRYPT_FREE_OBJID_FLAG: u32 = 2;
pub const CMSG_CONTENT_ENCRYPT_FREE_PARA_FLAG: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub struct CMSG_CONTENT_ENCRYPT_INFO {
    pub cbSize: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvEncryptionAuxInfo: *mut core::ffi::c_void,
    pub cRecipients: u32,
    pub rgCmsRecipients: PCMSG_RECIPIENT_ENCODE_INFO,
    pub pfnAlloc: PFN_CMSG_ALLOC,
    pub pfnFree: PFN_CMSG_FREE,
    pub dwEncryptFlags: u32,
    pub Anonymous: CMSG_CONTENT_ENCRYPT_INFO_0,
    pub dwFlags: u32,
    pub fCNG: windows_sys::core::BOOL,
    pub pbCNGContentEncryptKeyObject: *mut u8,
    pub pbContentEncryptKey: *mut u8,
    pub cbContentEncryptKey: u32,
}
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
impl Default for CMSG_CONTENT_ENCRYPT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub union CMSG_CONTENT_ENCRYPT_INFO_0 {
    pub hContentEncryptKey: HCRYPTKEY,
    pub hCNGContentEncryptKey: super::BCRYPT_KEY_HANDLE,
}
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
impl Default for CMSG_CONTENT_ENCRYPT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_CONTENT_ENCRYPT_PAD_ENCODED_LEN_FLAG: u32 = 1;
pub const CMSG_CONTENT_ENCRYPT_RELEASE_CONTEXT_FLAG: u32 = 32768;
pub const CMSG_CONTENT_PARAM: u32 = 2;
pub const CMSG_CRL_COUNT_PARAM: u32 = 13;
pub const CMSG_CRL_PARAM: u32 = 14;
pub const CMSG_CRYPT_RELEASE_CONTEXT_FLAG: u32 = 32768;
pub const CMSG_CTRL_ADD_ATTR_CERT: u32 = 14;
pub const CMSG_CTRL_ADD_CERT: u32 = 10;
pub const CMSG_CTRL_ADD_CMS_SIGNER_INFO: u32 = 20;
pub const CMSG_CTRL_ADD_CRL: u32 = 12;
pub const CMSG_CTRL_ADD_SIGNER: u32 = 6;
pub const CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA {
    pub cbSize: u32,
    pub dwSignerIndex: u32,
    pub blob: CRYPT_DATA_BLOB,
}
pub const CMSG_CTRL_DECRYPT: u32 = 2;
#[repr(C)]
#[cfg(feature = "ncrypt")]
#[derive(Clone, Copy)]
pub struct CMSG_CTRL_DECRYPT_PARA {
    pub cbSize: u32,
    pub Anonymous: CMSG_CTRL_DECRYPT_PARA_0,
    pub dwKeySpec: u32,
    pub dwRecipientIndex: u32,
}
#[cfg(feature = "ncrypt")]
impl Default for CMSG_CTRL_DECRYPT_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ncrypt")]
#[derive(Clone, Copy)]
pub union CMSG_CTRL_DECRYPT_PARA_0 {
    pub hCryptProv: HCRYPTPROV,
    pub hNCryptKey: super::NCRYPT_KEY_HANDLE,
}
#[cfg(feature = "ncrypt")]
impl Default for CMSG_CTRL_DECRYPT_PARA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_CTRL_DEL_ATTR_CERT: u32 = 15;
pub const CMSG_CTRL_DEL_CERT: u32 = 11;
pub const CMSG_CTRL_DEL_CRL: u32 = 13;
pub const CMSG_CTRL_DEL_SIGNER: u32 = 7;
pub const CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR: u32 = 9;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA {
    pub cbSize: u32,
    pub dwSignerIndex: u32,
    pub dwUnauthAttrIndex: u32,
}
pub const CMSG_CTRL_ENABLE_STRONG_SIGNATURE: u32 = 21;
pub const CMSG_CTRL_KEY_AGREE_DECRYPT: u32 = 17;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
#[derive(Clone, Copy)]
pub struct CMSG_CTRL_KEY_AGREE_DECRYPT_PARA {
    pub cbSize: u32,
    pub Anonymous: CMSG_CTRL_KEY_AGREE_DECRYPT_PARA_0,
    pub dwKeySpec: u32,
    pub pKeyAgree: PCMSG_KEY_AGREE_RECIPIENT_INFO,
    pub dwRecipientIndex: u32,
    pub dwRecipientEncryptedKeyIndex: u32,
    pub OriginatorPublicKey: CRYPT_BIT_BLOB,
}
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
impl Default for CMSG_CTRL_KEY_AGREE_DECRYPT_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
#[derive(Clone, Copy)]
pub union CMSG_CTRL_KEY_AGREE_DECRYPT_PARA_0 {
    pub hCryptProv: HCRYPTPROV,
    pub hNCryptKey: super::NCRYPT_KEY_HANDLE,
}
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
impl Default for CMSG_CTRL_KEY_AGREE_DECRYPT_PARA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_CTRL_KEY_TRANS_DECRYPT: u32 = 16;
#[repr(C)]
#[cfg(feature = "ncrypt")]
#[derive(Clone, Copy)]
pub struct CMSG_CTRL_KEY_TRANS_DECRYPT_PARA {
    pub cbSize: u32,
    pub Anonymous: CMSG_CTRL_KEY_TRANS_DECRYPT_PARA_0,
    pub dwKeySpec: u32,
    pub pKeyTrans: PCMSG_KEY_TRANS_RECIPIENT_INFO,
    pub dwRecipientIndex: u32,
}
#[cfg(feature = "ncrypt")]
impl Default for CMSG_CTRL_KEY_TRANS_DECRYPT_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ncrypt")]
#[derive(Clone, Copy)]
pub union CMSG_CTRL_KEY_TRANS_DECRYPT_PARA_0 {
    pub hCryptProv: HCRYPTPROV,
    pub hNCryptKey: super::NCRYPT_KEY_HANDLE,
}
#[cfg(feature = "ncrypt")]
impl Default for CMSG_CTRL_KEY_TRANS_DECRYPT_PARA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_CTRL_MAIL_LIST_DECRYPT: u32 = 18;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CMSG_CTRL_MAIL_LIST_DECRYPT_PARA {
    pub cbSize: u32,
    pub hCryptProv: HCRYPTPROV,
    pub pMailList: PCMSG_MAIL_LIST_RECIPIENT_INFO,
    pub dwRecipientIndex: u32,
    pub dwKeyChoice: u32,
    pub Anonymous: CMSG_CTRL_MAIL_LIST_DECRYPT_PARA_0,
}
#[cfg(feature = "minwindef")]
impl Default for CMSG_CTRL_MAIL_LIST_DECRYPT_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union CMSG_CTRL_MAIL_LIST_DECRYPT_PARA_0 {
    pub hKeyEncryptionKey: HCRYPTKEY,
    pub pvKeyEncryptionKey: *mut core::ffi::c_void,
}
#[cfg(feature = "minwindef")]
impl Default for CMSG_CTRL_MAIL_LIST_DECRYPT_PARA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_CTRL_VERIFY_HASH: u32 = 5;
pub const CMSG_CTRL_VERIFY_SIGNATURE: u32 = 1;
pub const CMSG_CTRL_VERIFY_SIGNATURE_EX: u32 = 19;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA {
    pub cbSize: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub dwSignerIndex: u32,
    pub dwSignerType: u32,
    pub pvSigner: *mut core::ffi::c_void,
}
impl Default for CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_DATA: u32 = 1;
pub const CMSG_DATA_FLAG: u32 = 2;
pub const CMSG_DEFAULT_INSTALLABLE_FUNC_OID: windows_sys::core::PCSTR = 1 as _;
pub const CMSG_DETACHED_FLAG: u32 = 4;
pub const CMSG_ENCODED_MESSAGE: u32 = 29;
pub const CMSG_ENCODED_SIGNER: u32 = 28;
pub const CMSG_ENCODE_HASHED_SUBJECT_IDENTIFIER_FLAG: u32 = 2;
pub const CMSG_ENCODE_SORTED_CTL_FLAG: u32 = 1;
pub const CMSG_ENCODING_TYPE_MASK: u32 = 4294901760;
pub const CMSG_ENCRYPTED: u32 = 6;
pub const CMSG_ENCRYPTED_DIGEST: u32 = 27;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_ENCRYPTED_ENCODE_INFO {
    pub cbSize: u32,
    pub ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvEncryptionAuxInfo: *mut core::ffi::c_void,
}
impl Default for CMSG_ENCRYPTED_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_ENCRYPTED_FLAG: u32 = 64;
pub const CMSG_ENCRYPT_PARAM: u32 = 26;
pub const CMSG_ENVELOPED: u32 = 3;
pub const CMSG_ENVELOPED_DATA_CMS_VERSION: u32 = 2;
pub const CMSG_ENVELOPED_DATA_PKCS_1_5_VERSION: u32 = 0;
pub const CMSG_ENVELOPED_DATA_V0: u32 = 0;
pub const CMSG_ENVELOPED_DATA_V2: u32 = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CMSG_ENVELOPED_ENCODE_INFO {
    pub cbSize: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvEncryptionAuxInfo: *mut core::ffi::c_void,
    pub cRecipients: u32,
    pub rgpRecipients: *mut PCERT_INFO,
}
#[cfg(feature = "minwindef")]
impl Default for CMSG_ENVELOPED_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_ENVELOPED_FLAG: u32 = 8;
pub const CMSG_ENVELOPED_RECIPIENT_V0: u32 = 0;
pub const CMSG_ENVELOPED_RECIPIENT_V2: u32 = 2;
pub const CMSG_ENVELOPED_RECIPIENT_V3: u32 = 3;
pub const CMSG_ENVELOPED_RECIPIENT_V4: u32 = 4;
pub const CMSG_ENVELOPE_ALGORITHM_PARAM: u32 = 15;
pub const CMSG_HASHED: u32 = 5;
pub const CMSG_HASHED_DATA_CMS_VERSION: u32 = 2;
pub const CMSG_HASHED_DATA_PKCS_1_5_VERSION: u32 = 0;
pub const CMSG_HASHED_DATA_V0: u32 = 0;
pub const CMSG_HASHED_DATA_V2: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_HASHED_ENCODE_INFO {
    pub cbSize: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvHashAuxInfo: *mut core::ffi::c_void,
}
impl Default for CMSG_HASHED_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_HASHED_FLAG: u32 = 32;
pub const CMSG_HASH_ALGORITHM_PARAM: u32 = 20;
pub const CMSG_HASH_DATA_PARAM: u32 = 21;
pub const CMSG_INDEFINITE_LENGTH: u32 = 4294967295;
pub const CMSG_INNER_CONTENT_TYPE_PARAM: u32 = 4;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_MATERIAL_FLAG: u32 = 2;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_OBJID_FLAG: u32 = 32;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PARA_FLAG: u32 = 1;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_ALG_FLAG: u32 = 4;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_BITS_FLAG: u32 = 16;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_PARA_FLAG: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_KEY_AGREE_ENCRYPT_INFO {
    pub cbSize: u32,
    pub dwRecipientIndex: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub UserKeyingMaterial: CRYPT_DATA_BLOB,
    pub dwOriginatorChoice: u32,
    pub Anonymous: CMSG_KEY_AGREE_ENCRYPT_INFO_0,
    pub cKeyAgreeKeyEncryptInfo: u32,
    pub rgpKeyAgreeKeyEncryptInfo: *mut PCMSG_KEY_AGREE_KEY_ENCRYPT_INFO,
    pub dwFlags: u32,
}
impl Default for CMSG_KEY_AGREE_ENCRYPT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CMSG_KEY_AGREE_ENCRYPT_INFO_0 {
    pub OriginatorCertId: CERT_ID,
    pub OriginatorPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
}
impl Default for CMSG_KEY_AGREE_ENCRYPT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_KEY_AGREE_EPHEMERAL_KEY_CHOICE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CMSG_KEY_AGREE_KEY_ENCRYPT_INFO {
    pub cbSize: u32,
    pub EncryptedKey: CRYPT_DATA_BLOB,
}
pub const CMSG_KEY_AGREE_ORIGINATOR_CERT: u32 = 1;
pub const CMSG_KEY_AGREE_ORIGINATOR_PUBLIC_KEY: u32 = 2;
pub const CMSG_KEY_AGREE_RECIPIENT: u32 = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO {
    pub cbSize: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvKeyEncryptionAuxInfo: *mut core::ffi::c_void,
    pub KeyWrapAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvKeyWrapAuxInfo: *mut core::ffi::c_void,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub dwKeySpec: u32,
    pub dwKeyChoice: u32,
    pub Anonymous: CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO_0,
    pub UserKeyingMaterial: CRYPT_DATA_BLOB,
    pub cRecipientEncryptedKeys: u32,
    pub rgpRecipientEncryptedKeys: *mut PCMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO,
}
#[cfg(feature = "minwindef")]
impl Default for CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO_0 {
    pub pEphemeralAlgorithm: PCRYPT_ALGORITHM_IDENTIFIER,
    pub pSenderId: PCERT_ID,
}
#[cfg(feature = "minwindef")]
impl Default for CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CMSG_KEY_AGREE_RECIPIENT_INFO {
    pub dwVersion: u32,
    pub dwOriginatorChoice: u32,
    pub Anonymous: CMSG_KEY_AGREE_RECIPIENT_INFO_0,
    pub UserKeyingMaterial: CRYPT_DATA_BLOB,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub cRecipientEncryptedKeys: u32,
    pub rgpRecipientEncryptedKeys: *mut PCMSG_RECIPIENT_ENCRYPTED_KEY_INFO,
}
#[cfg(feature = "minwindef")]
impl Default for CMSG_KEY_AGREE_RECIPIENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union CMSG_KEY_AGREE_RECIPIENT_INFO_0 {
    pub OriginatorCertId: CERT_ID,
    pub OriginatorPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
}
#[cfg(feature = "minwindef")]
impl Default for CMSG_KEY_AGREE_RECIPIENT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_KEY_AGREE_STATIC_KEY_CHOICE: u32 = 2;
pub const CMSG_KEY_AGREE_VERSION: u32 = 3;
pub const CMSG_KEY_TRANS_CMS_VERSION: u32 = 2;
pub const CMSG_KEY_TRANS_ENCRYPT_FREE_OBJID_FLAG: u32 = 2;
pub const CMSG_KEY_TRANS_ENCRYPT_FREE_PARA_FLAG: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CMSG_KEY_TRANS_ENCRYPT_INFO {
    pub cbSize: u32,
    pub dwRecipientIndex: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedKey: CRYPT_DATA_BLOB,
    pub dwFlags: u32,
}
pub const CMSG_KEY_TRANS_PKCS_1_5_VERSION: u32 = 0;
pub const CMSG_KEY_TRANS_RECIPIENT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO {
    pub cbSize: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvKeyEncryptionAuxInfo: *mut core::ffi::c_void,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub RecipientPublicKey: CRYPT_BIT_BLOB,
    pub RecipientId: CERT_ID,
}
impl Default for CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_KEY_TRANS_RECIPIENT_INFO {
    pub dwVersion: u32,
    pub RecipientId: CERT_ID,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedKey: CRYPT_DATA_BLOB,
}
impl Default for CMSG_KEY_TRANS_RECIPIENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_LENGTH_ONLY_FLAG: u32 = 2;
pub const CMSG_MAIL_LIST_ENCRYPT_FREE_OBJID_FLAG: u32 = 2;
pub const CMSG_MAIL_LIST_ENCRYPT_FREE_PARA_FLAG: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CMSG_MAIL_LIST_ENCRYPT_INFO {
    pub cbSize: u32,
    pub dwRecipientIndex: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedKey: CRYPT_DATA_BLOB,
    pub dwFlags: u32,
}
pub const CMSG_MAIL_LIST_HANDLE_KEY_CHOICE: u32 = 1;
pub const CMSG_MAIL_LIST_RECIPIENT: u32 = 3;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO {
    pub cbSize: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvKeyEncryptionAuxInfo: *mut core::ffi::c_void,
    pub hCryptProv: HCRYPTPROV,
    pub dwKeyChoice: u32,
    pub Anonymous: CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO_0,
    pub KeyId: CRYPT_DATA_BLOB,
    pub Date: super::FILETIME,
    pub pOtherAttr: PCRYPT_ATTRIBUTE_TYPE_VALUE,
}
#[cfg(feature = "minwindef")]
impl Default for CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO_0 {
    pub hKeyEncryptionKey: HCRYPTKEY,
    pub pvKeyEncryptionKey: *mut core::ffi::c_void,
}
#[cfg(feature = "minwindef")]
impl Default for CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CMSG_MAIL_LIST_RECIPIENT_INFO {
    pub dwVersion: u32,
    pub KeyId: CRYPT_DATA_BLOB,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedKey: CRYPT_DATA_BLOB,
    pub Date: super::FILETIME,
    pub pOtherAttr: PCRYPT_ATTRIBUTE_TYPE_VALUE,
}
#[cfg(feature = "minwindef")]
impl Default for CMSG_MAIL_LIST_RECIPIENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_MAIL_LIST_VERSION: u32 = 4;
pub const CMSG_MAX_LENGTH_FLAG: u32 = 32;
pub const CMSG_OID_CNG_EXPORT_KEY_AGREE_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptMsgDllCNGExportKeyAgree");
pub const CMSG_OID_CNG_EXPORT_KEY_TRANS_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptMsgDllCNGExportKeyTrans");
pub const CMSG_OID_CNG_GEN_CONTENT_ENCRYPT_KEY_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptMsgDllCNGGenContentEncryptKey");
pub const CMSG_OID_CNG_IMPORT_CONTENT_ENCRYPT_KEY_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptMsgDllCNGImportContentEncryptKey");
pub const CMSG_OID_CNG_IMPORT_KEY_AGREE_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptMsgDllCNGImportKeyAgree");
pub const CMSG_OID_CNG_IMPORT_KEY_TRANS_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptMsgDllCNGImportKeyTrans");
pub const CMSG_OID_EXPORT_ENCRYPT_KEY_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptMsgDllExportEncryptKey");
pub const CMSG_OID_EXPORT_KEY_AGREE_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptMsgDllExportKeyAgree");
pub const CMSG_OID_EXPORT_KEY_TRANS_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptMsgDllExportKeyTrans");
pub const CMSG_OID_EXPORT_MAIL_LIST_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptMsgDllExportMailList");
pub const CMSG_OID_GEN_CONTENT_ENCRYPT_KEY_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptMsgDllGenContentEncryptKey");
pub const CMSG_OID_GEN_ENCRYPT_KEY_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptMsgDllGenEncryptKey");
pub const CMSG_OID_IMPORT_ENCRYPT_KEY_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptMsgDllImportEncryptKey");
pub const CMSG_OID_IMPORT_KEY_AGREE_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptMsgDllImportKeyAgree");
pub const CMSG_OID_IMPORT_KEY_TRANS_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptMsgDllImportKeyTrans");
pub const CMSG_OID_IMPORT_MAIL_LIST_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptMsgDllImportMailList");
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CMSG_RC2_AUX_INFO {
    pub cbSize: u32,
    pub dwBitLen: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CMSG_RC4_AUX_INFO {
    pub cbSize: u32,
    pub dwBitLen: u32,
}
pub const CMSG_RC4_NO_SALT_FLAG: u32 = 1073741824;
pub const CMSG_RECIPIENT_COUNT_PARAM: u32 = 17;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CMSG_RECIPIENT_ENCODE_INFO {
    pub dwRecipientChoice: u32,
    pub Anonymous: CMSG_RECIPIENT_ENCODE_INFO_0,
}
#[cfg(feature = "minwindef")]
impl Default for CMSG_RECIPIENT_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union CMSG_RECIPIENT_ENCODE_INFO_0 {
    pub pKeyTrans: PCMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO,
    pub pKeyAgree: PCMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO,
    pub pMailList: PCMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO,
}
#[cfg(feature = "minwindef")]
impl Default for CMSG_RECIPIENT_ENCODE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO {
    pub cbSize: u32,
    pub RecipientPublicKey: CRYPT_BIT_BLOB,
    pub RecipientId: CERT_ID,
    pub Date: super::FILETIME,
    pub pOtherAttr: PCRYPT_ATTRIBUTE_TYPE_VALUE,
}
#[cfg(feature = "minwindef")]
impl Default for CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CMSG_RECIPIENT_ENCRYPTED_KEY_INFO {
    pub RecipientId: CERT_ID,
    pub EncryptedKey: CRYPT_DATA_BLOB,
    pub Date: super::FILETIME,
    pub pOtherAttr: PCRYPT_ATTRIBUTE_TYPE_VALUE,
}
#[cfg(feature = "minwindef")]
impl Default for CMSG_RECIPIENT_ENCRYPTED_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_RECIPIENT_INDEX_PARAM: u32 = 18;
pub const CMSG_RECIPIENT_INFO_PARAM: u32 = 19;
pub const CMSG_SIGNED: u32 = 2;
pub const CMSG_SIGNED_AND_ENVELOPED: u32 = 4;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
#[derive(Clone, Copy, Default)]
pub struct CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO {
    pub cbSize: u32,
    pub SignedInfo: CMSG_SIGNED_ENCODE_INFO,
    pub EnvelopedInfo: CMSG_ENVELOPED_ENCODE_INFO,
}
pub const CMSG_SIGNED_AND_ENVELOPED_FLAG: u32 = 16;
pub const CMSG_SIGNED_DATA_CMS_VERSION: u32 = 3;
pub const CMSG_SIGNED_DATA_NO_SIGN_FLAG: u32 = 128;
pub const CMSG_SIGNED_DATA_PKCS_1_5_VERSION: u32 = 1;
pub const CMSG_SIGNED_DATA_V1: u32 = 1;
pub const CMSG_SIGNED_DATA_V3: u32 = 3;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
#[derive(Clone, Copy)]
pub struct CMSG_SIGNED_ENCODE_INFO {
    pub cbSize: u32,
    pub cSigners: u32,
    pub rgSigners: PCMSG_SIGNER_ENCODE_INFO,
    pub cCertEncoded: u32,
    pub rgCertEncoded: PCERT_BLOB,
    pub cCrlEncoded: u32,
    pub rgCrlEncoded: PCRL_BLOB,
}
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
impl Default for CMSG_SIGNED_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_SIGNED_FLAG: u32 = 4;
pub const CMSG_SIGNER_AUTH_ATTR_PARAM: u32 = 9;
pub const CMSG_SIGNER_CERT_ID_PARAM: u32 = 38;
pub const CMSG_SIGNER_CERT_INFO_PARAM: u32 = 7;
pub const CMSG_SIGNER_COUNT_PARAM: u32 = 5;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
#[derive(Clone, Copy)]
pub struct CMSG_SIGNER_ENCODE_INFO {
    pub cbSize: u32,
    pub pCertInfo: PCERT_INFO,
    pub Anonymous: CMSG_SIGNER_ENCODE_INFO_0,
    pub dwKeySpec: u32,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvHashAuxInfo: *mut core::ffi::c_void,
    pub cAuthAttr: u32,
    pub rgAuthAttr: PCRYPT_ATTRIBUTE,
    pub cUnauthAttr: u32,
    pub rgUnauthAttr: PCRYPT_ATTRIBUTE,
}
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
impl Default for CMSG_SIGNER_ENCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
#[derive(Clone, Copy)]
pub union CMSG_SIGNER_ENCODE_INFO_0 {
    pub hCryptProv: HCRYPTPROV,
    pub hNCryptKey: super::NCRYPT_KEY_HANDLE,
}
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
impl Default for CMSG_SIGNER_ENCODE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_SIGNER_HASH_ALGORITHM_PARAM: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CMSG_SIGNER_INFO {
    pub dwVersion: u32,
    pub Issuer: CERT_NAME_BLOB,
    pub SerialNumber: CRYPT_INTEGER_BLOB,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub HashEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedHash: CRYPT_DATA_BLOB,
    pub AuthAttrs: CRYPT_ATTRIBUTES,
    pub UnauthAttrs: CRYPT_ATTRIBUTES,
}
pub const CMSG_SIGNER_INFO_CMS_VERSION: u32 = 3;
pub const CMSG_SIGNER_INFO_PARAM: u32 = 6;
pub const CMSG_SIGNER_INFO_PKCS_1_5_VERSION: u32 = 1;
pub const CMSG_SIGNER_INFO_V1: u32 = 1;
pub const CMSG_SIGNER_INFO_V3: u32 = 3;
pub const CMSG_SIGNER_ONLY_FLAG: u32 = 2;
pub const CMSG_SIGNER_UNAUTH_ATTR_PARAM: u32 = 10;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CMSG_SP3_COMPATIBLE_AUX_INFO {
    pub cbSize: u32,
    pub dwFlags: u32,
}
pub const CMSG_SP3_COMPATIBLE_ENCRYPT_FLAG: u32 = 2147483648;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMSG_STREAM_INFO {
    pub cbContent: u32,
    pub pfnStreamOutput: PFN_CMSG_STREAM_OUTPUT,
    pub pvArg: *mut core::ffi::c_void,
}
impl Default for CMSG_STREAM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMSG_TRUSTED_SIGNER_FLAG: u32 = 1;
pub const CMSG_TYPE_PARAM: u32 = 1;
pub const CMSG_UNPROTECTED_ATTR_PARAM: u32 = 37;
pub const CMSG_USE_SIGNER_INDEX_FLAG: u32 = 4;
pub const CMSG_VERIFY_COUNTER_SIGN_ENABLE_STRONG_FLAG: u32 = 1;
pub const CMSG_VERIFY_SIGNER_CERT: u32 = 2;
pub const CMSG_VERIFY_SIGNER_CHAIN: u32 = 3;
pub const CMSG_VERIFY_SIGNER_NULL: u32 = 4;
pub const CMSG_VERIFY_SIGNER_PUBKEY: u32 = 1;
pub const CMSG_VERSION_PARAM: u32 = 30;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMS_DH_KEY_INFO {
    pub dwVersion: u32,
    pub Algid: ALG_ID,
    pub pszContentEncObjId: windows_sys::core::PSTR,
    pub PubInfo: CRYPT_DATA_BLOB,
    pub pReserved: *mut core::ffi::c_void,
}
impl Default for CMS_DH_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMS_KEY_INFO {
    pub dwVersion: u32,
    pub Algid: ALG_ID,
    pub pbOID: *mut u8,
    pub cbOID: u32,
}
impl Default for CMS_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMS_SIGNER_INFO: windows_sys::core::PCSTR = 501 as _;
pub const CNG_RSA_PRIVATE_KEY_BLOB: windows_sys::core::PCSTR = 83 as _;
pub const CNG_RSA_PUBLIC_KEY_BLOB: windows_sys::core::PCSTR = 72 as _;
pub const CONTEXT_OID_CAPI2_ANY: windows_sys::core::PCSTR = 5 as _;
pub const CONTEXT_OID_CERTIFICATE: windows_sys::core::PCSTR = 1 as _;
pub const CONTEXT_OID_CREATE_OBJECT_CONTEXT_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("ContextDllCreateObjectContext");
pub const CONTEXT_OID_CRL: windows_sys::core::PCSTR = 2 as _;
pub const CONTEXT_OID_CTL: windows_sys::core::PCSTR = 3 as _;
pub const CONTEXT_OID_OCSP_RESP: windows_sys::core::PCSTR = 6 as _;
pub const CONTEXT_OID_PKCS7: windows_sys::core::PCSTR = 4 as _;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CPS_URLS {
    pub pszURL: windows_sys::core::PWSTR,
    pub pAlgorithm: *mut CRYPT_ALGORITHM_IDENTIFIER,
    pub pDigest: *mut CRYPT_DATA_BLOB,
}
impl Default for CPS_URLS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CREDENTIAL_OID_PASSWORD_CREDENTIALS_A: windows_sys::core::PCSTR = 1 as _;
pub const CREDENTIAL_OID_PASSWORD_CREDENTIALS_W: windows_sys::core::PCSTR = 2 as _;
pub type CRL_BLOB = CRYPT_INTEGER_BLOB;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CRL_CONTEXT {
    pub dwCertEncodingType: u32,
    pub pbCrlEncoded: *mut u8,
    pub cbCrlEncoded: u32,
    pub pCrlInfo: PCRL_INFO,
    pub hCertStore: HCERTSTORE,
}
#[cfg(feature = "minwindef")]
impl Default for CRL_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRL_DIST_POINT {
    pub DistPointName: CRL_DIST_POINT_NAME,
    pub ReasonFlags: CRYPT_BIT_BLOB,
    pub CRLIssuer: CERT_ALT_NAME_INFO,
}
impl Default for CRL_DIST_POINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRL_DIST_POINTS_INFO {
    pub cDistPoint: u32,
    pub rgDistPoint: PCRL_DIST_POINT,
}
impl Default for CRL_DIST_POINTS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRL_DIST_POINT_ERR_CRL_ISSUER_BIT: u32 = 2147483648;
pub const CRL_DIST_POINT_ERR_INDEX_MASK: u32 = 127;
pub const CRL_DIST_POINT_ERR_INDEX_SHIFT: u32 = 24;
pub const CRL_DIST_POINT_FULL_NAME: u32 = 1;
pub const CRL_DIST_POINT_ISSUER_RDN_NAME: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRL_DIST_POINT_NAME {
    pub dwDistPointNameChoice: u32,
    pub Anonymous: CRL_DIST_POINT_NAME_0,
}
impl Default for CRL_DIST_POINT_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CRL_DIST_POINT_NAME_0 {
    pub FullName: CERT_ALT_NAME_INFO,
}
impl Default for CRL_DIST_POINT_NAME_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRL_DIST_POINT_NO_NAME: u32 = 0;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CRL_ENTRY {
    pub SerialNumber: CRYPT_INTEGER_BLOB,
    pub RevocationDate: super::FILETIME,
    pub cExtension: u32,
    pub rgExtension: PCERT_EXTENSION,
}
#[cfg(feature = "minwindef")]
impl Default for CRL_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRL_FIND_ANY: u32 = 0;
pub const CRL_FIND_EXISTING: u32 = 2;
pub const CRL_FIND_ISSUED_BY: u32 = 1;
pub const CRL_FIND_ISSUED_BY_AKI_FLAG: u32 = 1;
pub const CRL_FIND_ISSUED_BY_BASE_FLAG: u32 = 8;
pub const CRL_FIND_ISSUED_BY_DELTA_FLAG: u32 = 4;
pub const CRL_FIND_ISSUED_BY_SIGNATURE_FLAG: u32 = 2;
pub const CRL_FIND_ISSUED_FOR: u32 = 3;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CRL_FIND_ISSUED_FOR_PARA {
    pub pSubjectCert: PCCERT_CONTEXT,
    pub pIssuerCert: PCCERT_CONTEXT,
}
#[cfg(feature = "minwindef")]
impl Default for CRL_FIND_ISSUED_FOR_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRL_FIND_ISSUED_FOR_SET_STRONG_PROPERTIES_FLAG: u32 = 16;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CRL_INFO {
    pub dwVersion: u32,
    pub SignatureAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Issuer: CERT_NAME_BLOB,
    pub ThisUpdate: super::FILETIME,
    pub NextUpdate: super::FILETIME,
    pub cCRLEntry: u32,
    pub rgCRLEntry: PCRL_ENTRY,
    pub cExtension: u32,
    pub rgExtension: PCERT_EXTENSION,
}
#[cfg(feature = "minwindef")]
impl Default for CRL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRL_ISSUING_DIST_POINT {
    pub DistPointName: CRL_DIST_POINT_NAME,
    pub fOnlyContainsUserCerts: windows_sys::core::BOOL,
    pub fOnlyContainsCACerts: windows_sys::core::BOOL,
    pub OnlySomeReasonFlags: CRYPT_BIT_BLOB,
    pub fIndirectCRL: windows_sys::core::BOOL,
}
impl Default for CRL_ISSUING_DIST_POINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRL_REASON_AA_COMPROMISE: u32 = 10;
pub const CRL_REASON_AA_COMPROMISE_FLAG: u32 = 128;
pub const CRL_REASON_AFFILIATION_CHANGED: u32 = 3;
pub const CRL_REASON_AFFILIATION_CHANGED_FLAG: u32 = 16;
pub const CRL_REASON_CA_COMPROMISE: u32 = 2;
pub const CRL_REASON_CA_COMPROMISE_FLAG: u32 = 32;
pub const CRL_REASON_CERTIFICATE_HOLD: u32 = 6;
pub const CRL_REASON_CERTIFICATE_HOLD_FLAG: u32 = 2;
pub const CRL_REASON_CESSATION_OF_OPERATION: u32 = 5;
pub const CRL_REASON_CESSATION_OF_OPERATION_FLAG: u32 = 4;
pub const CRL_REASON_KEY_COMPROMISE: u32 = 1;
pub const CRL_REASON_KEY_COMPROMISE_FLAG: u32 = 64;
pub const CRL_REASON_PRIVILEGE_WITHDRAWN: u32 = 9;
pub const CRL_REASON_PRIVILEGE_WITHDRAWN_FLAG: u32 = 1;
pub const CRL_REASON_REMOVE_FROM_CRL: u32 = 8;
pub const CRL_REASON_SUPERSEDED: u32 = 4;
pub const CRL_REASON_SUPERSEDED_FLAG: u32 = 8;
pub const CRL_REASON_UNSPECIFIED: u32 = 0;
pub const CRL_REASON_UNUSED_FLAG: u32 = 128;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CRL_REVOCATION_INFO {
    pub pCrlEntry: PCRL_ENTRY,
    pub pCrlContext: PCCRL_CONTEXT,
    pub pCrlIssuerChain: PCCERT_CHAIN_CONTEXT,
}
#[cfg(feature = "minwindef")]
impl Default for CRL_REVOCATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRL_V1: u32 = 0;
pub const CRL_V2: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CROSS_CERT_DIST_POINTS_INFO {
    pub dwSyncDeltaTime: u32,
    pub cDistPoint: u32,
    pub rgDistPoint: PCERT_ALT_NAME_INFO,
}
impl Default for CROSS_CERT_DIST_POINTS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CROSS_CERT_DIST_POINT_ERR_INDEX_MASK: u32 = 255;
pub const CROSS_CERT_DIST_POINT_ERR_INDEX_SHIFT: u32 = 24;
pub const CRYPTNET_CACHED_OCSP_SWITCH_TO_CRL_COUNT_DEFAULT: u32 = 50;
pub const CRYPTNET_CACHED_OCSP_SWITCH_TO_CRL_COUNT_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptnetCachedOcspSwitchToCrlCount");
pub const CRYPTNET_CRL_BEFORE_OCSP_ENABLE: u32 = 4294967295;
pub const CRYPTNET_CRL_PRE_FETCH_DISABLE_INFORMATION_EVENTS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("DisableInformationEvents");
pub const CRYPTNET_CRL_PRE_FETCH_LOG_FILE_NAME_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("LogFileName");
pub const CRYPTNET_CRL_PRE_FETCH_MAX_AGE_SECONDS_DEFAULT: u32 = 7200;
pub const CRYPTNET_CRL_PRE_FETCH_MAX_AGE_SECONDS_MIN: u32 = 300;
pub const CRYPTNET_CRL_PRE_FETCH_MAX_AGE_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("MaxAgeSeconds");
pub const CRYPTNET_CRL_PRE_FETCH_MIN_AFTER_NEXT_UPDATE_SECONDS_DEFAULT: u32 = 300;
pub const CRYPTNET_CRL_PRE_FETCH_MIN_AFTER_NEXT_UPDATE_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("MinAfterNextUpdateSeconds");
pub const CRYPTNET_CRL_PRE_FETCH_MIN_BEFORE_NEXT_UPDATE_SECONDS_DEFAULT: u32 = 300;
pub const CRYPTNET_CRL_PRE_FETCH_MIN_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("MinBeforeNextUpdateSeconds");
pub const CRYPTNET_CRL_PRE_FETCH_PROCESS_NAME_LIST_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("ProcessNameList");
pub const CRYPTNET_CRL_PRE_FETCH_PUBLISH_BEFORE_NEXT_UPDATE_SECONDS_DEFAULT: u32 = 3600;
pub const CRYPTNET_CRL_PRE_FETCH_PUBLISH_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("PublishBeforeNextUpdateSeconds");
pub const CRYPTNET_CRL_PRE_FETCH_PUBLISH_RANDOM_INTERVAL_SECONDS_DEFAULT: u32 = 300;
pub const CRYPTNET_CRL_PRE_FETCH_PUBLISH_RANDOM_INTERVAL_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("PublishRandomIntervalSeconds");
pub const CRYPTNET_CRL_PRE_FETCH_TIMEOUT_SECONDS_DEFAULT: u32 = 300;
pub const CRYPTNET_CRL_PRE_FETCH_TIMEOUT_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("TimeoutSeconds");
pub const CRYPTNET_CRL_PRE_FETCH_URL_LIST_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("PreFetchUrlList");
pub const CRYPTNET_MAX_CACHED_OCSP_PER_CRL_COUNT_DEFAULT: u32 = 500;
pub const CRYPTNET_MAX_CACHED_OCSP_PER_CRL_COUNT_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptnetMaxCachedOcspPerCrlCount");
pub const CRYPTNET_OCSP_AFTER_CRL_DISABLE: u32 = 4294967295;
pub const CRYPTNET_PRE_FETCH_AFTER_CURRENT_TIME_PRE_FETCH_PERIOD_SECONDS_DEFAULT: u32 = 1800;
pub const CRYPTNET_PRE_FETCH_AFTER_CURRENT_TIME_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptnetPreFetchAfterCurrentTimePreFetchPeriodSeconds");
pub const CRYPTNET_PRE_FETCH_AFTER_PUBLISH_PRE_FETCH_DIVISOR_DEFAULT: u32 = 10;
pub const CRYPTNET_PRE_FETCH_AFTER_PUBLISH_PRE_FETCH_DIVISOR_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptnetPreFetchAfterPublishPreFetchDivisor");
pub const CRYPTNET_PRE_FETCH_BEFORE_NEXT_UPDATE_PRE_FETCH_DIVISOR_DEFAULT: u32 = 20;
pub const CRYPTNET_PRE_FETCH_BEFORE_NEXT_UPDATE_PRE_FETCH_DIVISOR_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptnetPreFetchBeforeNextUpdatePreFetchDivisor");
pub const CRYPTNET_PRE_FETCH_MAX_AFTER_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_DEFAULT: u32 = 14400;
pub const CRYPTNET_PRE_FETCH_MAX_AFTER_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptnetPreFetchMaxAfterNextUpdatePreFetchPeriodSeconds");
pub const CRYPTNET_PRE_FETCH_MAX_MAX_AGE_SECONDS_DEFAULT: u32 = 1209600;
pub const CRYPTNET_PRE_FETCH_MAX_MAX_AGE_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptnetPreFetchMaxMaxAgeSeconds");
pub const CRYPTNET_PRE_FETCH_MIN_AFTER_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_DEFAULT: u32 = 1800;
pub const CRYPTNET_PRE_FETCH_MIN_AFTER_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptnetPreFetchMinAfterNextUpdatePreFetchPeriodSeconds");
pub const CRYPTNET_PRE_FETCH_MIN_BEFORE_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_DEFAULT: u32 = 3600;
pub const CRYPTNET_PRE_FETCH_MIN_BEFORE_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptnetPreFetchMinBeforeNextUpdatePreFetchSeconds");
pub const CRYPTNET_PRE_FETCH_MIN_MAX_AGE_SECONDS_DEFAULT: u32 = 3600;
pub const CRYPTNET_PRE_FETCH_MIN_MAX_AGE_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptnetPreFetchMinMaxAgeSeconds");
pub const CRYPTNET_PRE_FETCH_MIN_OCSP_VALIDITY_PERIOD_SECONDS_DEFAULT: u32 = 1209600;
pub const CRYPTNET_PRE_FETCH_MIN_OCSP_VALIDITY_PERIOD_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptnetPreFetchMinOcspValidityPeriodSeconds");
pub const CRYPTNET_PRE_FETCH_RETRIEVAL_TIMEOUT_SECONDS_DEFAULT: u32 = 300;
pub const CRYPTNET_PRE_FETCH_RETRIEVAL_TIMEOUT_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptnetPreFetchRetrievalTimeoutSeconds");
pub const CRYPTNET_PRE_FETCH_SCAN_AFTER_TRIGGER_DELAY_SECONDS_DEFAULT: u32 = 60;
pub const CRYPTNET_PRE_FETCH_SCAN_AFTER_TRIGGER_DELAY_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptnetPreFetchScanAfterTriggerDelaySeconds");
pub const CRYPTNET_PRE_FETCH_TRIGGER_DISABLE: u32 = 4294967295;
pub const CRYPTNET_PRE_FETCH_TRIGGER_PERIOD_SECONDS_DEFAULT: u32 = 600;
pub const CRYPTNET_PRE_FETCH_TRIGGER_PERIOD_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptnetPreFetchTriggerPeriodSeconds");
pub const CRYPTNET_PRE_FETCH_VALIDITY_PERIOD_AFTER_NEXT_UPDATE_PRE_FETCH_DIVISOR_DEFAULT: u32 = 10;
pub const CRYPTNET_PRE_FETCH_VALIDITY_PERIOD_AFTER_NEXT_UPDATE_PRE_FETCH_DIVISOR_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptnetPreFetchValidityPeriodAfterNextUpdatePreFetchDivisor");
pub const CRYPTNET_URL_CACHE_DEFAULT_FLUSH: u32 = 0;
pub const CRYPTNET_URL_CACHE_DEFAULT_FLUSH_EXEMPT_SECONDS_DEFAULT: u32 = 2419200;
pub const CRYPTNET_URL_CACHE_DEFAULT_FLUSH_EXEMPT_SECONDS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptnetDefaultFlushExemptSeconds");
pub const CRYPTNET_URL_CACHE_DISABLE_FLUSH: u32 = 4294967295;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct CRYPTNET_URL_CACHE_FLUSH_INFO {
    pub cbSize: u32,
    pub dwExemptSeconds: u32,
    pub ExpireTime: super::FILETIME,
}
pub const CRYPTNET_URL_CACHE_PRE_FETCH_AUTOROOT_CAB: u32 = 5;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_BLOB: u32 = 1;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_CRL: u32 = 2;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_DISALLOWED_CERT_CAB: u32 = 6;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct CRYPTNET_URL_CACHE_PRE_FETCH_INFO {
    pub cbSize: u32,
    pub dwObjectType: u32,
    pub dwError: u32,
    pub dwReserved: u32,
    pub ThisUpdateTime: super::FILETIME,
    pub NextUpdateTime: super::FILETIME,
    pub PublishTime: super::FILETIME,
}
pub const CRYPTNET_URL_CACHE_PRE_FETCH_NONE: u32 = 0;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_OCSP: u32 = 3;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_PIN_RULES_CAB: u32 = 7;
pub const CRYPTNET_URL_CACHE_RESPONSE_HTTP: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CRYPTNET_URL_CACHE_RESPONSE_INFO {
    pub cbSize: u32,
    pub wResponseType: u16,
    pub wResponseFlags: u16,
    pub LastModifiedTime: super::FILETIME,
    pub dwMaxAge: u32,
    pub pwszETag: windows_sys::core::PCWSTR,
    pub dwProxyId: u32,
}
#[cfg(feature = "minwindef")]
impl Default for CRYPTNET_URL_CACHE_RESPONSE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTNET_URL_CACHE_RESPONSE_NONE: u32 = 0;
pub const CRYPTNET_URL_CACHE_RESPONSE_VALIDATED: u32 = 32768;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_3DES_KEY_STATE {
    pub Key: [u8; 24],
    pub IV: [u8; 8],
    pub Feedback: [u8; 8],
}
impl Default for CRYPT_3DES_KEY_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_ACCUMULATIVE_TIMEOUT: u32 = 2048;
pub const CRYPT_ACQUIRE_ALLOW_NCRYPT_KEY_FLAG: u32 = 65536;
pub const CRYPT_ACQUIRE_CACHE_FLAG: u32 = 1;
pub const CRYPT_ACQUIRE_COMPARE_KEY_FLAG: u32 = 4;
pub const CRYPT_ACQUIRE_NCRYPT_KEY_FLAGS_MASK: u32 = 458752;
pub const CRYPT_ACQUIRE_NO_HEALING: u32 = 8;
pub const CRYPT_ACQUIRE_ONLY_NCRYPT_KEY_FLAG: u32 = 262144;
pub const CRYPT_ACQUIRE_PREFER_NCRYPT_KEY_FLAG: u32 = 131072;
pub const CRYPT_ACQUIRE_SILENT_FLAG: u32 = 64;
pub const CRYPT_ACQUIRE_USE_PROV_INFO_FLAG: u32 = 2;
pub const CRYPT_ACQUIRE_WINDOW_HANDLE_FLAG: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_AES_128_KEY_STATE {
    pub Key: [u8; 16],
    pub IV: [u8; 16],
    pub EncryptionState: [[u8; 16]; 11],
    pub DecryptionState: [[u8; 16]; 11],
    pub Feedback: [u8; 16],
}
impl Default for CRYPT_AES_128_KEY_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_AES_256_KEY_STATE {
    pub Key: [u8; 32],
    pub IV: [u8; 16],
    pub EncryptionState: [[u8; 16]; 15],
    pub DecryptionState: [[u8; 16]; 15],
    pub Feedback: [u8; 16],
}
impl Default for CRYPT_AES_256_KEY_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_AIA_RETRIEVAL: u32 = 524288;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_ALGORITHM_IDENTIFIER {
    pub pszObjId: windows_sys::core::PSTR,
    pub Parameters: CRYPT_OBJID_BLOB,
}
impl Default for CRYPT_ALGORITHM_IDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_ARCHIVABLE: u32 = 16384;
pub const CRYPT_ARCHIVE: u32 = 256;
pub const CRYPT_ASN_ENCODING: u32 = 1;
pub const CRYPT_ASYNC_RETRIEVAL: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_ASYNC_RETRIEVAL_COMPLETION {
    pub pfnCompletion: PFN_CRYPT_ASYNC_RETRIEVAL_COMPLETION_FUNC,
    pub pvCompletion: *mut core::ffi::c_void,
}
impl Default for CRYPT_ASYNC_RETRIEVAL_COMPLETION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_ATTRIBUTE {
    pub pszObjId: windows_sys::core::PSTR,
    pub cValue: u32,
    pub rgValue: PCRYPT_ATTR_BLOB,
}
impl Default for CRYPT_ATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_ATTRIBUTES {
    pub cAttr: u32,
    pub rgAttr: PCRYPT_ATTRIBUTE,
}
impl Default for CRYPT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_ATTRIBUTE_TYPE_VALUE {
    pub pszObjId: windows_sys::core::PSTR,
    pub Value: CRYPT_OBJID_BLOB,
}
impl Default for CRYPT_ATTRIBUTE_TYPE_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CRYPT_ATTR_BLOB = CRYPT_INTEGER_BLOB;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_BIT_BLOB {
    pub cbData: u32,
    pub pbData: *mut u8,
    pub cUnusedBits: u32,
}
impl Default for CRYPT_BIT_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_BLOB_ARRAY {
    pub cBlob: u32,
    pub rgBlob: PCRYPT_DATA_BLOB,
}
impl Default for CRYPT_BLOB_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_BLOB_VER3: u32 = 128;
pub const CRYPT_CACHE_ONLY_RETRIEVAL: u32 = 2;
pub const CRYPT_CHECK_FRESHNESS_TIME_VALIDITY: u32 = 1024;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_CONTENT_INFO {
    pub pszObjId: windows_sys::core::PSTR,
    pub Content: CRYPT_DER_BLOB,
}
impl Default for CRYPT_CONTENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY {
    pub pszObjId: windows_sys::core::PSTR,
    pub cValue: u32,
    pub rgValue: PCRYPT_DER_BLOB,
}
impl Default for CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_CREATE_IV: u32 = 512;
pub const CRYPT_CREATE_NEW_FLUSH_ENTRY: u32 = 268435456;
pub const CRYPT_CREATE_SALT: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_CREDENTIALS {
    pub cbSize: u32,
    pub pszCredentialsOid: windows_sys::core::PCSTR,
    pub pvCredentials: *mut core::ffi::c_void,
}
impl Default for CRYPT_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_CSP_PROVIDER {
    pub dwKeySpec: u32,
    pub pwszProviderName: windows_sys::core::PWSTR,
    pub Signature: CRYPT_BIT_BLOB,
}
impl Default for CRYPT_CSP_PROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CRYPT_DATA_BLOB = CRYPT_INTEGER_BLOB;
pub const CRYPT_DATA_KEY: u32 = 2048;
pub const CRYPT_DECODE_ALLOC_FLAG: u32 = 32768;
pub const CRYPT_DECODE_ENABLE_IA5CONVERSION_FLAG: u32 = 100663296;
pub const CRYPT_DECODE_ENABLE_PUNYCODE_FLAG: u32 = 33554432;
pub const CRYPT_DECODE_ENABLE_UTF8PERCENT_FLAG: u32 = 67108864;
pub const CRYPT_DECODE_NOCOPY_FLAG: u32 = 1;
pub const CRYPT_DECODE_NO_SIGNATURE_BYTE_REVERSAL_FLAG: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_DECODE_PARA {
    pub cbSize: u32,
    pub pfnAlloc: PFN_CRYPT_ALLOC,
    pub pfnFree: PFN_CRYPT_FREE,
}
pub const CRYPT_DECODE_SHARE_OID_STRING_FLAG: u32 = 4;
pub const CRYPT_DECODE_TO_BE_SIGNED_FLAG: u32 = 2;
pub const CRYPT_DECRYPT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_DECRYPT_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgAndCertEncodingType: u32,
    pub cCertStore: u32,
    pub rghCertStore: *mut HCERTSTORE,
}
impl Default for CRYPT_DECRYPT_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_DECRYPT_RSA_NO_PADDING_CHECK: u32 = 32;
pub const CRYPT_DEFAULT_CONTAINER_OPTIONAL: u32 = 128;
pub const CRYPT_DEFAULT_CONTEXT_AUTO_RELEASE_FLAG: u32 = 1;
pub const CRYPT_DEFAULT_CONTEXT_CERT_SIGN_OID: u32 = 1;
pub const CRYPT_DEFAULT_CONTEXT_MULTI_CERT_SIGN_OID: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA {
    pub cOID: u32,
    pub rgpszOID: *mut windows_sys::core::PSTR,
}
impl Default for CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_DEFAULT_CONTEXT_PROCESS_FLAG: u32 = 2;
pub const CRYPT_DEFAULT_OID: windows_sys::core::PCSTR = windows_sys::core::s!("DEFAULT");
pub const CRYPT_DELETEKEYSET: u32 = 16;
pub const CRYPT_DELETE_DEFAULT: u32 = 4;
pub const CRYPT_DELETE_KEYSET: u32 = 16;
pub type CRYPT_DER_BLOB = CRYPT_INTEGER_BLOB;
pub const CRYPT_DESTROYKEY: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_DES_KEY_STATE {
    pub Key: [u8; 8],
    pub IV: [u8; 8],
    pub Feedback: [u8; 8],
}
impl Default for CRYPT_DES_KEY_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CRYPT_DIGEST_BLOB = CRYPT_INTEGER_BLOB;
pub const CRYPT_DONT_CACHE_RESULT: u32 = 8;
pub const CRYPT_DONT_CHECK_TIME_VALIDITY: u32 = 512;
pub const CRYPT_DONT_VERIFY_SIGNATURE: u32 = 256;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_ECC_CMS_SHARED_INFO {
    pub Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EntityUInfo: CRYPT_DATA_BLOB,
    pub rgbSuppPubInfo: [u8; 4],
}
impl Default for CRYPT_ECC_CMS_SHARED_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_ECC_CMS_SHARED_INFO_SUPPPUBINFO_BYTE_LENGTH: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_ECC_PRIVATE_KEY_INFO {
    pub dwVersion: u32,
    pub PrivateKey: CRYPT_DER_BLOB,
    pub szCurveOid: windows_sys::core::PSTR,
    pub PublicKey: CRYPT_BIT_BLOB,
}
impl Default for CRYPT_ECC_PRIVATE_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_ECC_PRIVATE_KEY_INFO_v1: u32 = 1;
pub const CRYPT_ENABLE_FILE_RETRIEVAL: u32 = 134217728;
pub const CRYPT_ENABLE_SSL_REVOCATION_RETRIEVAL: u32 = 8388608;
pub const CRYPT_ENCODE_ALLOC_FLAG: u32 = 32768;
pub const CRYPT_ENCODE_DECODE_NONE: u32 = 0;
pub const CRYPT_ENCODE_ENABLE_IA5CONVERSION_FLAG: u32 = 393216;
pub const CRYPT_ENCODE_ENABLE_PUNYCODE_FLAG: u32 = 131072;
pub const CRYPT_ENCODE_ENABLE_UTF8PERCENT_FLAG: u32 = 262144;
pub const CRYPT_ENCODE_NO_SIGNATURE_BYTE_REVERSAL_FLAG: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_ENCODE_PARA {
    pub cbSize: u32,
    pub pfnAlloc: PFN_CRYPT_ALLOC,
    pub pfnFree: PFN_CRYPT_FREE,
}
pub const CRYPT_ENCRYPT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_ENCRYPTED_PRIVATE_KEY_INFO {
    pub EncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedPrivateKey: CRYPT_DATA_BLOB,
}
pub const CRYPT_ENCRYPT_ALG_OID_GROUP_ID: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_ENCRYPT_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgEncodingType: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvEncryptionAuxInfo: *mut core::ffi::c_void,
    pub dwFlags: u32,
    pub dwInnerContentType: u32,
}
impl Default for CRYPT_ENCRYPT_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_ENHKEY_USAGE_OID_GROUP_ID: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_ENROLLMENT_NAME_VALUE_PAIR {
    pub pwszName: windows_sys::core::PWSTR,
    pub pwszValue: windows_sys::core::PWSTR,
}
impl Default for CRYPT_ENROLLMENT_NAME_VALUE_PAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_EXPORT: u32 = 4;
pub const CRYPT_EXPORTABLE: u32 = 1;
pub const CRYPT_EXPORT_KEY: u32 = 64;
pub const CRYPT_EXT_OR_ATTR_OID_GROUP_ID: u32 = 6;
pub const CRYPT_FAILED: u32 = 0;
pub const CRYPT_FASTSGC: u32 = 2;
pub const CRYPT_FIND_MACHINE_KEYSET_FLAG: u32 = 2;
pub const CRYPT_FIND_SILENT_KEYSET_FLAG: u32 = 64;
pub const CRYPT_FIND_USER_KEYSET_FLAG: u32 = 1;
pub const CRYPT_FIRST: u32 = 1;
pub const CRYPT_FIRST_ALG_OID_GROUP_ID: u32 = 1;
pub const CRYPT_FLAG_IPSEC: u32 = 16;
pub const CRYPT_FLAG_PCT1: u32 = 1;
pub const CRYPT_FLAG_SIGNING: u32 = 32;
pub const CRYPT_FLAG_SSL2: u32 = 2;
pub const CRYPT_FLAG_SSL3: u32 = 4;
pub const CRYPT_FLAG_TLS1: u32 = 8;
pub const CRYPT_FORCE_KEY_PROTECTION_HIGH: u32 = 32768;
pub const CRYPT_FORMAT_COMMA: u32 = 4096;
pub const CRYPT_FORMAT_CRLF: u32 = 512;
pub const CRYPT_FORMAT_OID: u32 = 4;
pub const CRYPT_FORMAT_RDN_CRLF: u32 = 512;
pub const CRYPT_FORMAT_RDN_REVERSE: u32 = 2048;
pub const CRYPT_FORMAT_RDN_SEMICOLON: u32 = 256;
pub const CRYPT_FORMAT_RDN_UNQUOTE: u32 = 1024;
pub const CRYPT_FORMAT_SEMICOLON: u32 = 256;
pub const CRYPT_FORMAT_SIMPLE: u32 = 1;
pub const CRYPT_FORMAT_STR_MULTI_LINE: u32 = 1;
pub const CRYPT_FORMAT_STR_NO_HEX: u32 = 16;
pub const CRYPT_FORMAT_X509: u32 = 2;
pub const CRYPT_GET_INSTALLED_OID_FUNC_FLAG: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO {
    pub cbSize: u32,
    pub iDeltaCrlIndicator: i32,
    pub pftCacheResync: super::LPFILETIME,
    pub pLastSyncTime: super::LPFILETIME,
    pub pMaxAgeTime: super::LPFILETIME,
    pub pChainPara: PCERT_REVOCATION_CHAIN_PARA,
    pub pDeltaCrlIndicator: PCRYPT_INTEGER_BLOB,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_GET_URL_FROM_AUTH_ATTRIBUTE: u32 = 8;
pub const CRYPT_GET_URL_FROM_EXTENSION: u32 = 2;
pub const CRYPT_GET_URL_FROM_PROPERTY: u32 = 1;
pub const CRYPT_GET_URL_FROM_UNAUTH_ATTRIBUTE: u32 = 4;
pub const CRYPT_HASH_ALG_OID_GROUP_ID: u32 = 1;
pub type CRYPT_HASH_BLOB = CRYPT_INTEGER_BLOB;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_HASH_INFO {
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Hash: CRYPT_HASH_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_HASH_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgEncodingType: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvHashAuxInfo: *mut core::ffi::c_void,
}
impl Default for CRYPT_HASH_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_HTTP_POST_RETRIEVAL: u32 = 1048576;
pub const CRYPT_IMPL_HARDWARE: u32 = 1;
pub const CRYPT_IMPL_MIXED: u32 = 3;
pub const CRYPT_IMPL_REMOVABLE: u32 = 8;
pub const CRYPT_IMPL_SOFTWARE: u32 = 2;
pub const CRYPT_IMPL_UNKNOWN: u32 = 4;
pub const CRYPT_IMPORT_KEY: u32 = 128;
pub const CRYPT_INITIATOR: u32 = 64;
pub const CRYPT_INSTALL_OID_FUNC_BEFORE_FLAG: u32 = 1;
pub const CRYPT_INSTALL_OID_INFO_BEFORE_FLAG: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_INTEGER_BLOB {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl Default for CRYPT_INTEGER_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_IPSEC_HMAC_KEY: u32 = 256;
pub const CRYPT_KDF_OID_GROUP_ID: u32 = 10;
pub const CRYPT_KEEP_TIME_VALID: u32 = 128;
pub const CRYPT_KEK: u32 = 1024;
pub const CRYPT_KEYID_ALLOC_FLAG: u32 = 32768;
pub const CRYPT_KEYID_DELETE_FLAG: u32 = 16;
pub const CRYPT_KEYID_MACHINE_FLAG: u32 = 32;
pub const CRYPT_KEYID_SET_NEW_FLAG: u32 = 8192;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_KEY_PROV_INFO {
    pub pwszContainerName: windows_sys::core::PWSTR,
    pub pwszProvName: windows_sys::core::PWSTR,
    pub dwProvType: u32,
    pub dwFlags: u32,
    pub cProvParam: u32,
    pub rgProvParam: PCRYPT_KEY_PROV_PARAM,
    pub dwKeySpec: u32,
}
impl Default for CRYPT_KEY_PROV_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_KEY_PROV_PARAM {
    pub dwParam: u32,
    pub pbData: *mut u8,
    pub cbData: u32,
    pub dwFlags: u32,
}
impl Default for CRYPT_KEY_PROV_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ncrypt")]
#[derive(Clone, Copy)]
pub struct CRYPT_KEY_SIGN_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgAndCertEncodingType: u32,
    pub Anonymous: CRYPT_KEY_SIGN_MESSAGE_PARA_0,
    pub dwKeySpec: u32,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvHashAuxInfo: *mut core::ffi::c_void,
    pub PubKeyAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
}
#[cfg(feature = "ncrypt")]
impl Default for CRYPT_KEY_SIGN_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ncrypt")]
#[derive(Clone, Copy)]
pub union CRYPT_KEY_SIGN_MESSAGE_PARA_0 {
    pub hCryptProv: HCRYPTPROV,
    pub hNCryptKey: super::NCRYPT_KEY_HANDLE,
}
#[cfg(feature = "ncrypt")]
impl Default for CRYPT_KEY_SIGN_MESSAGE_PARA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_KEY_VERIFY_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgEncodingType: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
}
pub const CRYPT_LAST_ALG_OID_GROUP_ID: u32 = 4;
pub const CRYPT_LAST_OID_GROUP_ID: u32 = 10;
pub const CRYPT_LDAP_AREC_EXCLUSIVE_RETRIEVAL: u32 = 262144;
pub const CRYPT_LDAP_INSERT_ENTRY_ATTRIBUTE: u32 = 32768;
pub const CRYPT_LDAP_SCOPE_BASE_ONLY_RETRIEVAL: u32 = 8192;
pub const CRYPT_LDAP_SIGN_RETRIEVAL: u32 = 65536;
pub const CRYPT_LITTLE_ENDIAN: u32 = 1;
pub const CRYPT_LOCALIZED_NAME_ENCODING_TYPE: u32 = 0;
pub const CRYPT_LOCALIZED_NAME_OID: windows_sys::core::PCSTR = windows_sys::core::s!("LocalizedNames");
pub const CRYPT_MAC: u32 = 32;
pub const CRYPT_MACHINE_DEFAULT: u32 = 1;
pub const CRYPT_MACHINE_KEYSET: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_MASK_GEN_ALGORITHM {
    pub pszObjId: windows_sys::core::PSTR,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
}
impl Default for CRYPT_MASK_GEN_ALGORITHM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_MATCH_ANY_ENCODING_TYPE: u32 = 4294967295;
pub const CRYPT_MESSAGE_BARE_CONTENT_OUT_FLAG: u32 = 1;
pub const CRYPT_MESSAGE_ENCAPSULATED_CONTENT_OUT_FLAG: u32 = 2;
pub const CRYPT_MESSAGE_KEYID_RECIPIENT_FLAG: u32 = 4;
pub const CRYPT_MESSAGE_KEYID_SIGNER_FLAG: u32 = 4;
pub const CRYPT_MESSAGE_SILENT_KEYSET_FLAG: u32 = 64;
pub const CRYPT_MODE_CBC: u32 = 1;
pub const CRYPT_MODE_CBCI: u32 = 6;
pub const CRYPT_MODE_CBCOFM: u32 = 9;
pub const CRYPT_MODE_CBCOFMI: u32 = 10;
pub const CRYPT_MODE_CFB: u32 = 4;
pub const CRYPT_MODE_CFBP: u32 = 7;
pub const CRYPT_MODE_CTS: u32 = 5;
pub const CRYPT_MODE_ECB: u32 = 2;
pub const CRYPT_MODE_OFB: u32 = 3;
pub const CRYPT_MODE_OFBP: u32 = 8;
pub const CRYPT_NDR_ENCODING: u32 = 2;
pub const CRYPT_NEWKEYSET: u32 = 8;
pub const CRYPT_NEXT: u32 = 2;
pub const CRYPT_NOHASHOID: u32 = 1;
pub const CRYPT_NOT_MODIFIED_RETRIEVAL: u32 = 4194304;
pub const CRYPT_NO_AUTH_RETRIEVAL: u32 = 131072;
pub const CRYPT_NO_OCSP_FAILOVER_TO_CRL_RETRIEVAL: u32 = 33554432;
pub const CRYPT_NO_SALT: u32 = 16;
pub const CRYPT_OAEP: u32 = 64;
pub const CRYPT_OBJECT_LOCATOR_FIRST_RESERVED_USER_NAME_TYPE: u32 = 33;
pub const CRYPT_OBJECT_LOCATOR_LAST_RESERVED_NAME_TYPE: u32 = 32;
pub const CRYPT_OBJECT_LOCATOR_LAST_RESERVED_USER_NAME_TYPE: u32 = 65535;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE {
    pub cbSize: u32,
    pub pfnGet: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_GET,
    pub pfnRelease: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_RELEASE,
    pub pfnFreePassword: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_PASSWORD,
    pub pfnFree: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE,
    pub pfnFreeIdentifier: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_IDENTIFIER,
}
pub const CRYPT_OBJECT_LOCATOR_RELEASE_DLL_UNLOAD: u32 = 4;
pub const CRYPT_OBJECT_LOCATOR_RELEASE_PROCESS_EXIT: u32 = 3;
pub const CRYPT_OBJECT_LOCATOR_RELEASE_SERVICE_STOP: u32 = 2;
pub const CRYPT_OBJECT_LOCATOR_RELEASE_SYSTEM_SHUTDOWN: u32 = 1;
pub const CRYPT_OBJECT_LOCATOR_SPN_NAME_TYPE: u32 = 1;
pub type CRYPT_OBJID_BLOB = CRYPT_INTEGER_BLOB;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_OBJID_TABLE {
    pub dwAlgId: u32,
    pub pszObjId: windows_sys::core::PCSTR,
}
impl Default for CRYPT_OBJID_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_OCSP_ONLY_RETRIEVAL: u32 = 16777216;
pub const CRYPT_OFFLINE_CHECK_RETRIEVAL: u32 = 16384;
pub const CRYPT_OID_CREATE_COM_OBJECT_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllCreateCOMObject");
pub const CRYPT_OID_DECODE_OBJECT_EX_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllDecodeObjectEx");
pub const CRYPT_OID_DECODE_OBJECT_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllDecodeObject");
pub const CRYPT_OID_DISABLE_SEARCH_DS_FLAG: u32 = 2147483648;
pub const CRYPT_OID_ENCODE_OBJECT_EX_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllEncodeObjectEx");
pub const CRYPT_OID_ENCODE_OBJECT_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllEncodeObject");
pub const CRYPT_OID_ENUM_PHYSICAL_STORE_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CertDllEnumPhysicalStore");
pub const CRYPT_OID_ENUM_SYSTEM_STORE_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CertDllEnumSystemStore");
pub const CRYPT_OID_EXPORT_PRIVATE_KEY_INFO_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllExportPrivateKeyInfoEx");
pub const CRYPT_OID_EXPORT_PUBLIC_KEY_INFO_EX2_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllExportPublicKeyInfoEx2");
pub const CRYPT_OID_EXPORT_PUBLIC_KEY_INFO_FROM_BCRYPT_HANDLE_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllExportPublicKeyInfoFromBCryptKeyHandle");
pub const CRYPT_OID_EXPORT_PUBLIC_KEY_INFO_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllExportPublicKeyInfoEx");
pub const CRYPT_OID_EXTRACT_ENCODED_SIGNATURE_PARAMETERS_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllExtractEncodedSignatureParameters");
pub const CRYPT_OID_FIND_LOCALIZED_NAME_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllFindLocalizedName");
pub const CRYPT_OID_FIND_OID_INFO_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllFindOIDInfo");
pub const CRYPT_OID_FORMAT_OBJECT_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllFormatObject");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_OID_FUNC_ENTRY {
    pub pszOID: windows_sys::core::PCSTR,
    pub pvFuncAddr: *mut core::ffi::c_void,
}
impl Default for CRYPT_OID_FUNC_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_OID_IMPORT_PRIVATE_KEY_INFO_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllImportPrivateKeyInfoEx");
pub const CRYPT_OID_IMPORT_PUBLIC_KEY_INFO_EX2_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllImportPublicKeyInfoEx2");
pub const CRYPT_OID_IMPORT_PUBLIC_KEY_INFO_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllImportPublicKeyInfoEx");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_OID_INFO {
    pub cbSize: u32,
    pub pszOID: windows_sys::core::PCSTR,
    pub pwszName: windows_sys::core::PCWSTR,
    pub dwGroupId: u32,
    pub Anonymous: CRYPT_OID_INFO_0,
    pub ExtraInfo: CRYPT_DATA_BLOB,
}
impl Default for CRYPT_OID_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CRYPT_OID_INFO_0 {
    pub dwValue: u32,
    pub Algid: ALG_ID,
    pub dwLength: u32,
}
impl Default for CRYPT_OID_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_OID_INFO_ALGID_KEY: u32 = 3;
pub const CRYPT_OID_INFO_CNG_ALGID_KEY: u32 = 5;
pub const CRYPT_OID_INFO_CNG_SIGN_KEY: u32 = 6;
pub const CRYPT_OID_INFO_ECC_PARAMETERS_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptOIDInfoECCParameters");
pub const CRYPT_OID_INFO_ECC_WRAP_PARAMETERS_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptOIDInfoECCWrapParameters");
pub const CRYPT_OID_INFO_HASH_PARAMETERS_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptOIDInfoHashParameters");
pub const CRYPT_OID_INFO_MGF1_PARAMETERS_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptOIDInfoMgf1Parameters");
pub const CRYPT_OID_INFO_NAME_KEY: u32 = 2;
pub const CRYPT_OID_INFO_NO_HASH_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("NoHash");
pub const CRYPT_OID_INFO_NO_PARAMETERS_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptOIDInfoNoParameters");
pub const CRYPT_OID_INFO_NO_SIGN_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptOIDInfoNoSign");
pub const CRYPT_OID_INFO_OAEP_PARAMETERS_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptOIDInfoOAEPParameters");
pub const CRYPT_OID_INFO_OID_GROUP_BIT_LEN_MASK: u32 = 268369920;
pub const CRYPT_OID_INFO_OID_GROUP_BIT_LEN_SHIFT: u32 = 16;
pub const CRYPT_OID_INFO_OID_KEY: u32 = 1;
pub const CRYPT_OID_INFO_OID_KEY_FLAGS_MASK: u32 = 4294901760;
pub const CRYPT_OID_INFO_PREHASH_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("PreHash");
pub const CRYPT_OID_INFO_PUBKEY_ENCRYPT_KEY_FLAG: u32 = 1073741824;
pub const CRYPT_OID_INFO_PUBKEY_PREHASH_KEY_FLAG: u32 = 67108864;
pub const CRYPT_OID_INFO_PUBKEY_PURE_KEY_FLAG: u32 = 134217728;
pub const CRYPT_OID_INFO_PUBKEY_SIGN_KEY_FLAG: u32 = 2147483648;
pub const CRYPT_OID_INFO_SIGN_KEY: u32 = 4;
pub const CRYPT_OID_INHIBIT_SIGNATURE_FORMAT_FLAG: u32 = 1;
pub const CRYPT_OID_NO_NULL_ALGORITHM_PARA_FLAG: u32 = 4;
pub const CRYPT_OID_OPEN_STORE_PROV_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CertDllOpenStoreProv");
pub const CRYPT_OID_OPEN_SYSTEM_STORE_PROV_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CertDllOpenSystemStoreProv");
pub const CRYPT_OID_PQ_EXTRA_INFO_FLAGS_INDEX: u32 = 0;
pub const CRYPT_OID_PQ_EXTRA_INFO_MAX_LENGTH: u32 = 6;
pub const CRYPT_OID_PQ_EXTRA_INFO_PRIVATE_KEY_LENGTH_INDEX: u32 = 4;
pub const CRYPT_OID_PQ_EXTRA_INFO_PRIVATE_MAGIC_INDEX: u32 = 2;
pub const CRYPT_OID_PQ_EXTRA_INFO_PUBLIC_KEY_LENGTH_INDEX: u32 = 3;
pub const CRYPT_OID_PQ_EXTRA_INFO_PUBLIC_MAGIC_INDEX: u32 = 1;
pub const CRYPT_OID_PQ_EXTRA_INFO_SIGNATURE_LENGTH_INDEX: u32 = 5;
pub const CRYPT_OID_PUBKEY_ENCRYPT_ONLY_FLAG: u32 = 1073741824;
pub const CRYPT_OID_PUBKEY_PREHASH_ONLY_FLAG: u32 = 67108864;
pub const CRYPT_OID_PUBKEY_PURE_ONLY_FLAG: u32 = 134217728;
pub const CRYPT_OID_PUBKEY_SIGN_ONLY_FLAG: u32 = 2147483648;
pub const CRYPT_OID_REGISTER_PHYSICAL_STORE_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CertDllRegisterPhysicalStore");
pub const CRYPT_OID_REGISTER_SYSTEM_STORE_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CertDllRegisterSystemStore");
pub const CRYPT_OID_REGPATH: windows_sys::core::PCSTR = windows_sys::core::s!("Software\\Microsoft\\Cryptography\\OID");
pub const CRYPT_OID_REG_DLL_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("Dll");
pub const CRYPT_OID_REG_ENCODING_TYPE_PREFIX: windows_sys::core::PCSTR = windows_sys::core::s!("EncodingType ");
pub const CRYPT_OID_REG_FLAGS_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("CryptFlags");
pub const CRYPT_OID_REG_FUNC_NAME_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("FuncName");
pub const CRYPT_OID_REG_FUNC_NAME_VALUE_NAME_A: windows_sys::core::PCSTR = windows_sys::core::s!("FuncName");
pub const CRYPT_OID_SIGN_AND_ENCODE_HASH_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllSignAndEncodeHash");
pub const CRYPT_OID_SYSTEM_STORE_LOCATION_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("SystemStoreLocation");
pub const CRYPT_OID_UNREGISTER_PHYSICAL_STORE_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CertDllUnregisterPhysicalStore");
pub const CRYPT_OID_UNREGISTER_SYSTEM_STORE_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CertDllUnregisterSystemStore");
pub const CRYPT_OID_USE_CURVE_NAME_FOR_ENCODE_FLAG: u32 = 536870912;
pub const CRYPT_OID_USE_CURVE_PARAMETERS_FOR_ENCODE_FLAG: u32 = 268435456;
pub const CRYPT_OID_USE_PUBKEY_PARA_FOR_PKCS7_FLAG: u32 = 2;
pub const CRYPT_OID_VERIFY_CERTIFICATE_CHAIN_POLICY_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CertDllVerifyCertificateChainPolicy");
pub const CRYPT_OID_VERIFY_CTL_USAGE_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CertDllVerifyCTLUsage");
pub const CRYPT_OID_VERIFY_ENCODED_SIGNATURE_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CryptDllVerifyEncodedSignature");
pub const CRYPT_OID_VERIFY_REVOCATION_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("CertDllVerifyRevocation");
pub const CRYPT_ONLINE: u32 = 128;
pub const CRYPT_OWF_REPL_LM_HASH: u32 = 1;
pub const CRYPT_PARAM_ASYNC_RETRIEVAL_COMPLETION: windows_sys::core::PCSTR = 1 as _;
pub const CRYPT_PARAM_CANCEL_ASYNC_RETRIEVAL: windows_sys::core::PCSTR = 2 as _;
pub type CRYPT_PASSWORD_CREDENTIALS = CRYPT_PASSWORD_CREDENTIALSA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_PASSWORD_CREDENTIALSA {
    pub cbSize: u32,
    pub pszUsername: windows_sys::core::PSTR,
    pub pszPassword: windows_sys::core::PSTR,
}
impl Default for CRYPT_PASSWORD_CREDENTIALSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_PASSWORD_CREDENTIALSW {
    pub cbSize: u32,
    pub pszUsername: windows_sys::core::PWSTR,
    pub pszPassword: windows_sys::core::PWSTR,
}
impl Default for CRYPT_PASSWORD_CREDENTIALSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_PKCS12_PBE_PARAMS {
    pub iIterations: i32,
    pub cbSalt: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_PKCS8_EXPORT_PARAMS {
    pub hCryptProv: HCRYPTPROV,
    pub dwKeySpec: u32,
    pub pszPrivateKeyObjId: windows_sys::core::PSTR,
    pub pEncryptPrivateKeyFunc: PCRYPT_ENCRYPT_PRIVATE_KEY_FUNC,
    pub pVoidEncryptFunc: *mut core::ffi::c_void,
}
impl Default for CRYPT_PKCS8_EXPORT_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_PKCS8_IMPORT_PARAMS {
    pub PrivateKey: CRYPT_DIGEST_BLOB,
    pub pResolvehCryptProvFunc: PCRYPT_RESOLVE_HCRYPTPROV_FUNC,
    pub pVoidResolveFunc: *mut core::ffi::c_void,
    pub pDecryptPrivateKeyFunc: PCRYPT_DECRYPT_PRIVATE_KEY_FUNC,
    pub pVoidDecryptFunc: *mut core::ffi::c_void,
}
impl Default for CRYPT_PKCS8_IMPORT_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_POLICY_OID_GROUP_ID: u32 = 8;
pub const CRYPT_PREGEN: u32 = 64;
pub type CRYPT_PRIVATE_KEY_BLOB_AND_PARAMS = CRYPT_PKCS8_IMPORT_PARAMS;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_PRIVATE_KEY_INFO {
    pub Version: u32,
    pub Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub PrivateKey: CRYPT_DER_BLOB,
    pub pAttributes: PCRYPT_ATTRIBUTES,
}
impl Default for CRYPT_PRIVATE_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_PROXY_CACHE_RETRIEVAL: u32 = 2097152;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_PSOURCE_ALGORITHM {
    pub pszObjId: windows_sys::core::PSTR,
    pub EncodingParameters: CRYPT_DATA_BLOB,
}
impl Default for CRYPT_PSOURCE_ALGORITHM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_PSTORE: u32 = 2;
pub const CRYPT_PUBKEY_ALG_OID_GROUP_ID: u32 = 3;
pub const CRYPT_RANDOM_QUERY_STRING_RETRIEVAL: u32 = 67108864;
pub const CRYPT_RC2_128BIT_VERSION: u32 = 58;
pub const CRYPT_RC2_40BIT_VERSION: u32 = 160;
pub const CRYPT_RC2_56BIT_VERSION: u32 = 52;
pub const CRYPT_RC2_64BIT_VERSION: u32 = 120;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_RC2_CBC_PARAMETERS {
    pub dwVersion: u32,
    pub fIV: windows_sys::core::BOOL,
    pub rgbIV: [u8; 8],
}
impl Default for CRYPT_RC2_CBC_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_RC4_KEY_STATE {
    pub Key: [u8; 16],
    pub SBox: [u8; 256],
    pub i: u8,
    pub j: u8,
}
impl Default for CRYPT_RC4_KEY_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_RDN_ATTR_OID_GROUP_ID: u32 = 5;
pub const CRYPT_READ: u32 = 8;
pub const CRYPT_RECIPIENT: u32 = 16;
pub const CRYPT_REGISTER_FIRST_INDEX: u32 = 0;
pub const CRYPT_REGISTER_LAST_INDEX: u32 = 4294967295;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CRYPT_RETRIEVE_AUX_INFO {
    pub cbSize: u32,
    pub pLastSyncTime: *mut super::FILETIME,
    pub dwMaxUrlRetrievalByteCount: u32,
    pub pPreFetchInfo: PCRYPTNET_URL_CACHE_PRE_FETCH_INFO,
    pub pFlushInfo: PCRYPTNET_URL_CACHE_FLUSH_INFO,
    pub ppResponseInfo: *mut PCRYPTNET_URL_CACHE_RESPONSE_INFO,
    pub pwszCacheFileNamePrefix: windows_sys::core::PWSTR,
    pub pftCacheResync: super::LPFILETIME,
    pub fProxyCacheRetrieval: windows_sys::core::BOOL,
    pub dwHttpStatusCode: u32,
    pub ppwszErrorResponseHeaders: *mut windows_sys::core::PWSTR,
    pub ppErrorContentBlob: *mut PCRYPT_DATA_BLOB,
}
#[cfg(feature = "minwindef")]
impl Default for CRYPT_RETRIEVE_AUX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_RETRIEVE_MAX_ERROR_CONTENT_LENGTH: u32 = 4096;
pub const CRYPT_RETRIEVE_MULTIPLE_OBJECTS: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_RSAES_OAEP_PARAMETERS {
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub MaskGenAlgorithm: CRYPT_MASK_GEN_ALGORITHM,
    pub PSourceAlgorithm: CRYPT_PSOURCE_ALGORITHM,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_RSA_SSA_PSS_PARAMETERS {
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub MaskGenAlgorithm: CRYPT_MASK_GEN_ALGORITHM,
    pub dwSaltLength: u32,
    pub dwTrailerField: u32,
}
pub const CRYPT_SECRETDIGEST: u32 = 1;
pub const CRYPT_SEC_DESCR: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_SEQUENCE_OF_ANY {
    pub cValue: u32,
    pub rgValue: PCRYPT_DER_BLOB,
}
impl Default for CRYPT_SEQUENCE_OF_ANY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_SERVER: u32 = 1024;
pub const CRYPT_SF: u32 = 256;
pub const CRYPT_SGC: u32 = 1;
pub const CRYPT_SGCKEY: u32 = 8192;
pub const CRYPT_SGC_ENUM: u32 = 4;
pub const CRYPT_SIGN_ALG_OID_GROUP_ID: u32 = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CRYPT_SIGN_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgEncodingType: u32,
    pub pSigningCert: PCCERT_CONTEXT,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvHashAuxInfo: *mut core::ffi::c_void,
    pub cMsgCert: u32,
    pub rgpMsgCert: *mut PCCERT_CONTEXT,
    pub cMsgCrl: u32,
    pub rgpMsgCrl: *mut PCCRL_CONTEXT,
    pub cAuthAttr: u32,
    pub rgAuthAttr: PCRYPT_ATTRIBUTE,
    pub cUnauthAttr: u32,
    pub rgUnauthAttr: PCRYPT_ATTRIBUTE,
    pub dwFlags: u32,
    pub dwInnerContentType: u32,
}
#[cfg(feature = "minwindef")]
impl Default for CRYPT_SIGN_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_SILENT: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_SMART_CARD_ROOT_INFO {
    pub rgbCardID: [u8; 16],
    pub luid: ROOT_INFO_LUID,
}
impl Default for CRYPT_SMART_CARD_ROOT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_SMIME_CAPABILITIES {
    pub cCapability: u32,
    pub rgCapability: PCRYPT_SMIME_CAPABILITY,
}
impl Default for CRYPT_SMIME_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_SMIME_CAPABILITY {
    pub pszObjId: windows_sys::core::PSTR,
    pub Parameters: CRYPT_OBJID_BLOB,
}
impl Default for CRYPT_SMIME_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_SORTED_CTL_ENCODE_HASHED_SUBJECT_IDENTIFIER_FLAG: u32 = 65536;
pub const CRYPT_SSL2_FALLBACK: u32 = 2;
pub const CRYPT_STICKY_CACHE_RETRIEVAL: u32 = 4096;
pub const CRYPT_STRING_ANY: u32 = 7;
pub const CRYPT_STRING_BASE64: u32 = 1;
pub const CRYPT_STRING_BASE64HEADER: u32 = 0;
pub const CRYPT_STRING_BASE64REQUESTHEADER: u32 = 3;
pub const CRYPT_STRING_BASE64URI: u32 = 13;
pub const CRYPT_STRING_BASE64X509CRLHEADER: u32 = 9;
pub const CRYPT_STRING_BASE64_ANY: u32 = 6;
pub const CRYPT_STRING_BINARY: u32 = 2;
pub const CRYPT_STRING_ENCODEMASK: u32 = 255;
pub const CRYPT_STRING_HASHDATA: u32 = 268435456;
pub const CRYPT_STRING_HEX: u32 = 4;
pub const CRYPT_STRING_HEXADDR: u32 = 10;
pub const CRYPT_STRING_HEXASCII: u32 = 5;
pub const CRYPT_STRING_HEXASCIIADDR: u32 = 11;
pub const CRYPT_STRING_HEXRAW: u32 = 12;
pub const CRYPT_STRING_HEX_ANY: u32 = 8;
pub const CRYPT_STRING_NOCR: u32 = 2147483648;
pub const CRYPT_STRING_NOCRLF: u32 = 1073741824;
pub const CRYPT_STRING_PERCENTESCAPE: u32 = 134217728;
pub const CRYPT_STRING_RESERVED100: u32 = 256;
pub const CRYPT_STRING_RESERVED200: u32 = 512;
pub const CRYPT_STRING_STRICT: u32 = 536870912;
pub const CRYPT_SUCCEED: u32 = 1;
pub const CRYPT_TEMPLATE_OID_GROUP_ID: u32 = 9;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_TIMESTAMP_ACCURACY {
    pub dwSeconds: u32,
    pub dwMillis: u32,
    pub dwMicros: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CRYPT_TIMESTAMP_CONTEXT {
    pub cbEncoded: u32,
    pub pbEncoded: *mut u8,
    pub pTimeStamp: PCRYPT_TIMESTAMP_INFO,
}
#[cfg(feature = "minwindef")]
impl Default for CRYPT_TIMESTAMP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CRYPT_TIMESTAMP_INFO {
    pub dwVersion: u32,
    pub pszTSAPolicyId: windows_sys::core::PSTR,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub HashedMessage: CRYPT_DER_BLOB,
    pub SerialNumber: CRYPT_INTEGER_BLOB,
    pub ftTime: super::FILETIME,
    pub pvAccuracy: PCRYPT_TIMESTAMP_ACCURACY,
    pub fOrdering: windows_sys::core::BOOL,
    pub Nonce: CRYPT_DER_BLOB,
    pub Tsa: CRYPT_DER_BLOB,
    pub cExtension: u32,
    pub rgExtension: PCERT_EXTENSION,
}
#[cfg(feature = "minwindef")]
impl Default for CRYPT_TIMESTAMP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_TIMESTAMP_PARA {
    pub pszTSAPolicyId: windows_sys::core::PCSTR,
    pub fRequestCerts: windows_sys::core::BOOL,
    pub Nonce: CRYPT_INTEGER_BLOB,
    pub cExtension: u32,
    pub rgExtension: PCERT_EXTENSION,
}
impl Default for CRYPT_TIMESTAMP_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_TIMESTAMP_REQUEST {
    pub dwVersion: u32,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub HashedMessage: CRYPT_DER_BLOB,
    pub pszTSAPolicyId: windows_sys::core::PSTR,
    pub Nonce: CRYPT_INTEGER_BLOB,
    pub fCertReq: windows_sys::core::BOOL,
    pub cExtension: u32,
    pub rgExtension: PCERT_EXTENSION,
}
impl Default for CRYPT_TIMESTAMP_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_TIMESTAMP_RESPONSE {
    pub dwStatus: u32,
    pub cFreeText: u32,
    pub rgFreeText: *mut windows_sys::core::PWSTR,
    pub FailureInfo: CRYPT_BIT_BLOB,
    pub ContentInfo: CRYPT_DER_BLOB,
}
impl Default for CRYPT_TIMESTAMP_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_TIME_STAMP_REQUEST_INFO {
    pub pszTimeStampAlgorithm: windows_sys::core::PSTR,
    pub pszContentType: windows_sys::core::PSTR,
    pub Content: CRYPT_OBJID_BLOB,
    pub cAttribute: u32,
    pub rgAttribute: PCRYPT_ATTRIBUTE,
}
impl Default for CRYPT_TIME_STAMP_REQUEST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_TYPE2_FORMAT: u32 = 2;
pub type CRYPT_UINT_BLOB = CRYPT_INTEGER_BLOB;
pub const CRYPT_UI_PROMPT: u32 = 4;
pub const CRYPT_UNICODE_NAME_DECODE_DISABLE_IE4_UTF8_FLAG: u32 = 16777216;
pub const CRYPT_UNICODE_NAME_ENCODE_DISABLE_CHECK_TYPE_FLAG: u32 = 1073741824;
pub const CRYPT_UNICODE_NAME_ENCODE_ENABLE_T61_UNICODE_FLAG: i32 = -2147483648;
pub const CRYPT_UNICODE_NAME_ENCODE_ENABLE_UTF8_UNICODE_FLAG: u32 = 536870912;
pub const CRYPT_UNICODE_NAME_ENCODE_FORCE_UTF8_UNICODE_FLAG: u32 = 268435456;
pub const CRYPT_UPDATE_KEY: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_URL_ARRAY {
    pub cUrl: u32,
    pub rgwszUrl: *mut windows_sys::core::PWSTR,
}
impl Default for CRYPT_URL_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_URL_INFO {
    pub cbSize: u32,
    pub dwSyncDeltaTime: u32,
    pub cGroup: u32,
    pub rgcGroupEntry: *mut u32,
}
impl Default for CRYPT_URL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_USERDATA: u32 = 1;
pub const CRYPT_USER_DEFAULT: u32 = 2;
pub const CRYPT_USER_KEYSET: u32 = 4096;
pub const CRYPT_USER_PROTECTED: u32 = 2;
pub const CRYPT_USER_PROTECTED_STRONG: u32 = 1048576;
pub const CRYPT_VERIFYCONTEXT: u32 = 4026531840;
pub const CRYPT_VERIFY_CERT_SIGN_CHECK_WEAK_HASH_FLAG: u32 = 8;
pub const CRYPT_VERIFY_CERT_SIGN_DISABLE_MD2_MD4_FLAG: u32 = 1;
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_CERT: u32 = 2;
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_CHAIN: u32 = 3;
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_NULL: u32 = 4;
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_PUBKEY: u32 = 1;
pub const CRYPT_VERIFY_CERT_SIGN_RETURN_STRONG_PROPERTIES_FLAG: u32 = 4;
pub const CRYPT_VERIFY_CERT_SIGN_SET_STRONG_PROPERTIES_FLAG: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO {
    pub CertSignHashCNGAlgPropData: CRYPT_DATA_BLOB,
    pub CertIssuerPubKeyBitLengthPropData: CRYPT_DATA_BLOB,
}
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_BLOB: u32 = 1;
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_CERT: u32 = 2;
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_CRL: u32 = 3;
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_OCSP_BASIC_SIGNED_RESPONSE: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO {
    pub cCNGHashAlgid: u32,
    pub rgpwszCNGHashAlgid: *mut windows_sys::core::PCWSTR,
    pub dwWeakIndex: u32,
}
impl Default for CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_VERIFY_CONTEXT_SIGNATURE: u32 = 32;
pub const CRYPT_VERIFY_DATA_HASH: u32 = 64;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CRYPT_VERIFY_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgAndCertEncodingType: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub pfnGetSignerCertificate: PFN_CRYPT_GET_SIGNER_CERTIFICATE,
    pub pvGetArg: *mut core::ffi::c_void,
}
#[cfg(feature = "minwindef")]
impl Default for CRYPT_VERIFY_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_VOLATILE: u32 = 4096;
pub const CRYPT_WIRE_ONLY_RETRIEVAL: u32 = 4;
pub const CRYPT_WRITE: u32 = 16;
pub const CRYPT_X931_FORMAT: u32 = 4;
pub const CRYPT_X942_COUNTER_BYTE_LENGTH: u32 = 4;
pub const CRYPT_X942_KEY_LENGTH_BYTE_LENGTH: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_X942_OTHER_INFO {
    pub pszContentEncryptionObjId: windows_sys::core::PSTR,
    pub rgbCounter: [u8; 4],
    pub rgbKeyLength: [u8; 4],
    pub PubInfo: CRYPT_DATA_BLOB,
}
impl Default for CRYPT_X942_OTHER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_X942_PUB_INFO_BYTE_LENGTH: u32 = 64;
pub const CRYPT_Y_ONLY: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CTL_ANY_SUBJECT_INFO {
    pub SubjectAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub SubjectIdentifier: CRYPT_DATA_BLOB,
}
pub const CTL_ANY_SUBJECT_TYPE: u32 = 1;
pub const CTL_CERT_SUBJECT_TYPE: u32 = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CTL_CONTEXT {
    pub dwMsgAndCertEncodingType: u32,
    pub pbCtlEncoded: *mut u8,
    pub cbCtlEncoded: u32,
    pub pCtlInfo: PCTL_INFO,
    pub hCertStore: HCERTSTORE,
    pub hCryptMsg: HCRYPTMSG,
    pub pbCtlContent: *mut u8,
    pub cbCtlContent: u32,
}
#[cfg(feature = "minwindef")]
impl Default for CTL_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CTL_ENTRY {
    pub SubjectIdentifier: CRYPT_DATA_BLOB,
    pub cAttribute: u32,
    pub rgAttribute: PCRYPT_ATTRIBUTE,
}
impl Default for CTL_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CTL_ENTRY_FROM_PROP_CHAIN_FLAG: u32 = 1;
pub const CTL_FIND_ANY: u32 = 0;
pub const CTL_FIND_EXISTING: u32 = 5;
pub const CTL_FIND_MD5_HASH: u32 = 2;
pub const CTL_FIND_NO_LIST_ID_CBDATA: u32 = 4294967295;
#[cfg(feature = "minwindef")]
pub const CTL_FIND_NO_SIGNER_PTR: PCERT_INFO = -1 as _;
pub const CTL_FIND_SAME_USAGE_FLAG: u32 = 1;
pub const CTL_FIND_SHA1_HASH: u32 = 1;
pub const CTL_FIND_SUBJECT: u32 = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CTL_FIND_SUBJECT_PARA {
    pub cbSize: u32,
    pub pUsagePara: PCTL_FIND_USAGE_PARA,
    pub dwSubjectType: u32,
    pub pvSubject: *mut core::ffi::c_void,
}
#[cfg(feature = "minwindef")]
impl Default for CTL_FIND_SUBJECT_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CTL_FIND_USAGE: u32 = 3;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CTL_FIND_USAGE_PARA {
    pub cbSize: u32,
    pub SubjectUsage: CTL_USAGE,
    pub ListIdentifier: CRYPT_DATA_BLOB,
    pub pSigner: PCERT_INFO,
}
#[cfg(feature = "minwindef")]
impl Default for CTL_FIND_USAGE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CTL_INFO {
    pub dwVersion: u32,
    pub SubjectUsage: CTL_USAGE,
    pub ListIdentifier: CRYPT_DATA_BLOB,
    pub SequenceNumber: CRYPT_INTEGER_BLOB,
    pub ThisUpdate: super::FILETIME,
    pub NextUpdate: super::FILETIME,
    pub SubjectAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub cCTLEntry: u32,
    pub rgCTLEntry: PCTL_ENTRY,
    pub cExtension: u32,
    pub rgExtension: PCERT_EXTENSION,
}
#[cfg(feature = "minwindef")]
impl Default for CTL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CTL_USAGE {
    pub cUsageIdentifier: u32,
    pub rgpszUsageIdentifier: *mut windows_sys::core::PSTR,
}
impl Default for CTL_USAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CTL_USAGE_MATCH {
    pub dwType: u32,
    pub Usage: CTL_USAGE,
}
pub const CTL_V1: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CTL_VERIFY_USAGE_PARA {
    pub cbSize: u32,
    pub ListIdentifier: CRYPT_DATA_BLOB,
    pub cCtlStore: u32,
    pub rghCtlStore: *mut HCERTSTORE,
    pub cSignerStore: u32,
    pub rghSignerStore: *mut HCERTSTORE,
}
impl Default for CTL_VERIFY_USAGE_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct CTL_VERIFY_USAGE_STATUS {
    pub cbSize: u32,
    pub dwError: u32,
    pub dwFlags: u32,
    pub ppCtl: *mut PCCTL_CONTEXT,
    pub dwCtlEntryIndex: u32,
    pub ppSigner: *mut PCCERT_CONTEXT,
    pub dwSignerIndex: u32,
}
#[cfg(feature = "minwindef")]
impl Default for CTL_VERIFY_USAGE_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CT_EXTRA_CERT_CHAIN_POLICY_STATUS {
    pub cbSize: u32,
    pub lErrorStatus: i32,
    pub lErrorSubStatus: i32,
    pub cEntries: u32,
    pub cValidated: u32,
}
pub const CUR_BLOB_VERSION: u32 = 2;
pub type CertKeyType = i32;
pub type DATA_BLOB = CRYPT_INTEGER_BLOB;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DHPRIVKEY_VER3 {
    pub magic: u32,
    pub bitlenP: u32,
    pub bitlenQ: u32,
    pub bitlenJ: u32,
    pub bitlenX: u32,
    pub DSSSeed: DSSSEED,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DHPUBKEY {
    pub magic: u32,
    pub bitlen: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DHPUBKEY_VER3 {
    pub magic: u32,
    pub bitlenP: u32,
    pub bitlenQ: u32,
    pub bitlenJ: u32,
    pub DSSSeed: DSSSEED,
}
pub type DSSPRIVKEY_VER3 = DHPRIVKEY_VER3;
pub type DSSPUBKEY = DHPUBKEY;
pub type DSSPUBKEY_VER3 = DHPUBKEY_VER3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DSSSEED {
    pub counter: u32,
    pub seed: [u8; 20],
}
impl Default for DSSSEED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ECC_CMS_SHARED_INFO: windows_sys::core::PCSTR = 77 as _;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EV_EXTRA_CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwRootProgramQualifierFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EV_EXTRA_CERT_CHAIN_POLICY_STATUS {
    pub cbSize: u32,
    pub dwQualifiers: u32,
    pub dwIssuanceUsageIndex: u32,
}
pub const EXPORT_PRIVATE_KEYS: u32 = 4;
pub const EXPO_OFFLOAD_FUNC_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("OffloadModExpo");
pub const EXPO_OFFLOAD_REG_VALUE: windows_sys::core::PCSTR = windows_sys::core::s!("ExpoOffload");
#[cfg(feature = "winnt")]
pub const HCCE_LOCAL_MACHINE: HCERTCHAINENGINE = 1 as _;
#[cfg(feature = "winnt")]
pub const HCCE_SERIAL_LOCAL_MACHINE: HCERTCHAINENGINE = 2 as _;
#[cfg(feature = "winnt")]
pub type HCERTCHAINENGINE = super::HANDLE;
pub type HCERTSTORE = *mut core::ffi::c_void;
pub type HCERTSTOREPROV = *mut core::ffi::c_void;
pub type HCERT_SERVER_OCSP_RESPONSE = *mut core::ffi::c_void;
#[cfg(feature = "winnt")]
pub type HCRYPTASYNC = super::HANDLE;
pub type HCRYPTDEFAULTCONTEXT = *mut core::ffi::c_void;
pub type HCRYPTHASH = usize;
pub type HCRYPTKEY = usize;
pub type HCRYPTMSG = *mut core::ffi::c_void;
pub type HCRYPTOIDFUNCADDR = *mut core::ffi::c_void;
pub type HCRYPTOIDFUNCSET = *mut core::ffi::c_void;
pub type HCRYPTPROV = usize;
pub type HCRYPTPROV_LEGACY = usize;
pub type HCRYPTPROV_OR_NCRYPT_KEY_HANDLE = usize;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HMAC_INFO {
    pub HashAlgid: ALG_ID,
    pub pbInnerString: *mut u8,
    pub cbInnerString: u32,
    pub pbOuterString: *mut u8,
    pub cbOuterString: u32,
}
impl Default for HMAC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HP_ALGID: u32 = 1;
pub const HP_HASHSIZE: u32 = 4;
pub const HP_HASHVAL: u32 = 2;
pub const HP_HMAC_INFO: u32 = 5;
pub const HP_TLS1PRF_LABEL: u32 = 6;
pub const HP_TLS1PRF_SEED: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTPSPolicyCallbackData {
    pub Anonymous: HTTPSPolicyCallbackData_0,
    pub dwAuthType: u32,
    pub fdwChecks: u32,
    pub pwszServerName: *mut u16,
}
impl Default for HTTPSPolicyCallbackData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union HTTPSPolicyCallbackData_0 {
    pub cbStruct: u32,
    pub cbSize: u32,
}
impl Default for HTTPSPolicyCallbackData_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INTERNATIONAL_USAGE: u32 = 1;
pub type KEAPUBKEY = DHPUBKEY;
pub const KEYSTATEBLOB: u32 = 12;
pub const KEY_LENGTH_MASK: u32 = 4294901760;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KEY_TYPE_SUBTYPE {
    pub dwKeySpec: u32,
    pub Type: windows_sys::core::GUID,
    pub Subtype: windows_sys::core::GUID,
}
pub const KP_ADMIN_PIN: u32 = 31;
pub const KP_ALGID: u32 = 7;
pub const KP_BLOCKLEN: u32 = 8;
pub const KP_CERTIFICATE: u32 = 26;
pub const KP_CLEAR_KEY: u32 = 27;
pub const KP_CLIENT_RANDOM: u32 = 21;
pub const KP_CMS_DH_KEY_INFO: u32 = 38;
pub const KP_CMS_KEY_INFO: u32 = 37;
pub const KP_EFFECTIVE_KEYLEN: u32 = 19;
pub const KP_G: u32 = 12;
pub const KP_GET_USE_COUNT: u32 = 42;
pub const KP_HIGHEST_VERSION: u32 = 41;
pub const KP_INFO: u32 = 18;
pub const KP_IV: u32 = 1;
pub const KP_KEYEXCHANGE_PIN: u32 = 32;
pub const KP_KEYLEN: u32 = 9;
pub const KP_KEYVAL: u32 = 30;
pub const KP_MODE: u32 = 4;
pub const KP_MODE_BITS: u32 = 5;
pub const KP_OAEP_PARAMS: u32 = 36;
pub const KP_P: u32 = 11;
pub const KP_PADDING: u32 = 3;
pub const KP_PERMISSIONS: u32 = 6;
pub const KP_PIN_ID: u32 = 43;
pub const KP_PIN_INFO: u32 = 44;
pub const KP_PRECOMP_MD5: u32 = 24;
pub const KP_PRECOMP_SHA: u32 = 25;
pub const KP_PREHASH: u32 = 34;
pub const KP_PUB_EX_LEN: u32 = 28;
pub const KP_PUB_EX_VAL: u32 = 29;
pub const KP_PUB_PARAMS: u32 = 39;
pub const KP_Q: u32 = 13;
pub const KP_RA: u32 = 16;
pub const KP_RB: u32 = 17;
pub const KP_ROUNDS: u32 = 35;
pub const KP_RP: u32 = 23;
pub const KP_SALT: u32 = 2;
pub const KP_SALT_EX: u32 = 10;
pub const KP_SCHANNEL_ALG: u32 = 20;
pub const KP_SERVER_RANDOM: u32 = 22;
pub const KP_SIGNATURE_PIN: u32 = 33;
pub const KP_VERIFY_PARAMS: u32 = 40;
pub const KP_X: u32 = 14;
pub const KP_Y: u32 = 15;
pub const KeyTypeHardware: CertKeyType = 6;
pub const KeyTypeOther: CertKeyType = 0;
pub const KeyTypePassport: CertKeyType = 3;
pub const KeyTypePassportRemote: CertKeyType = 4;
pub const KeyTypePassportSmartCard: CertKeyType = 5;
pub const KeyTypePhysicalSmartCard: CertKeyType = 2;
pub const KeyTypeSelfSigned: CertKeyType = 8;
pub const KeyTypeSoftware: CertKeyType = 7;
pub const KeyTypeVirtualSmartCard: CertKeyType = 1;
pub const MAXUIDLEN: u32 = 64;
pub const MICROSOFT_ROOT_CERT_CHAIN_POLICY_CHECK_APPLICATION_ROOT_FLAG: u32 = 131072;
pub const MICROSOFT_ROOT_CERT_CHAIN_POLICY_DISABLE_FLIGHT_ROOT_FLAG: u32 = 262144;
pub const MICROSOFT_ROOT_CERT_CHAIN_POLICY_ENABLE_TEST_ROOT_FLAG: u32 = 65536;
pub const MS_DEF_DH_SCHANNEL_PROV_A: windows_sys::core::PCSTR = windows_sys::core::s!("Microsoft DH SChannel Cryptographic Provider");
pub const MS_DEF_DH_SCHANNEL_PROV_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft DH SChannel Cryptographic Provider");
pub const MS_DEF_DSS_DH_PROV_A: windows_sys::core::PCSTR = windows_sys::core::s!("Microsoft Base DSS and Diffie-Hellman Cryptographic Provider");
pub const MS_DEF_DSS_DH_PROV_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft Base DSS and Diffie-Hellman Cryptographic Provider");
pub const MS_DEF_DSS_PROV_A: windows_sys::core::PCSTR = windows_sys::core::s!("Microsoft Base DSS Cryptographic Provider");
pub const MS_DEF_DSS_PROV_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft Base DSS Cryptographic Provider");
pub const MS_DEF_PROV_A: windows_sys::core::PCSTR = windows_sys::core::s!("Microsoft Base Cryptographic Provider v1.0");
pub const MS_DEF_PROV_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft Base Cryptographic Provider v1.0");
pub const MS_DEF_RSA_SCHANNEL_PROV_A: windows_sys::core::PCSTR = windows_sys::core::s!("Microsoft RSA SChannel Cryptographic Provider");
pub const MS_DEF_RSA_SCHANNEL_PROV_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft RSA SChannel Cryptographic Provider");
pub const MS_DEF_RSA_SIG_PROV_A: windows_sys::core::PCSTR = windows_sys::core::s!("Microsoft RSA Signature Cryptographic Provider");
pub const MS_DEF_RSA_SIG_PROV_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft RSA Signature Cryptographic Provider");
pub const MS_ENHANCED_PROV_A: windows_sys::core::PCSTR = windows_sys::core::s!("Microsoft Enhanced Cryptographic Provider v1.0");
pub const MS_ENHANCED_PROV_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft Enhanced Cryptographic Provider v1.0");
pub const MS_ENH_DSS_DH_PROV_A: windows_sys::core::PCSTR = windows_sys::core::s!("Microsoft Enhanced DSS and Diffie-Hellman Cryptographic Provider");
pub const MS_ENH_DSS_DH_PROV_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft Enhanced DSS and Diffie-Hellman Cryptographic Provider");
pub const MS_ENH_RSA_AES_PROV_A: windows_sys::core::PCSTR = windows_sys::core::s!("Microsoft Enhanced RSA and AES Cryptographic Provider");
pub const MS_ENH_RSA_AES_PROV_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft Enhanced RSA and AES Cryptographic Provider");
pub const MS_ENH_RSA_AES_PROV_XP_A: windows_sys::core::PCSTR = windows_sys::core::s!("Microsoft Enhanced RSA and AES Cryptographic Provider (Prototype)");
pub const MS_ENH_RSA_AES_PROV_XP_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft Enhanced RSA and AES Cryptographic Provider (Prototype)");
pub const MS_SCARD_PROV_A: windows_sys::core::PCSTR = windows_sys::core::s!("Microsoft Base Smart Card Crypto Provider");
pub const MS_SCARD_PROV_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft Base Smart Card Crypto Provider");
pub const MS_STRONG_PROV_A: windows_sys::core::PCSTR = windows_sys::core::s!("Microsoft Strong Cryptographic Provider");
pub const MS_STRONG_PROV_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft Strong Cryptographic Provider");
pub const NETSCAPE_SIGN_CA_CERT_TYPE: u32 = 1;
pub const NETSCAPE_SIGN_CERT_TYPE: u32 = 16;
pub const NETSCAPE_SMIME_CA_CERT_TYPE: u32 = 2;
pub const NETSCAPE_SMIME_CERT_TYPE: u32 = 32;
pub const NETSCAPE_SSL_CA_CERT_TYPE: u32 = 4;
pub const NETSCAPE_SSL_CLIENT_AUTH_CERT_TYPE: u32 = 128;
pub const NETSCAPE_SSL_SERVER_AUTH_CERT_TYPE: u32 = 64;
pub const OCSP_BASIC_BY_KEY_RESPONDER_ID: u32 = 2;
pub const OCSP_BASIC_BY_NAME_RESPONDER_ID: u32 = 1;
pub const OCSP_BASIC_GOOD_CERT_STATUS: u32 = 0;
pub const OCSP_BASIC_RESPONSE: windows_sys::core::PCSTR = 69 as _;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct OCSP_BASIC_RESPONSE_ENTRY {
    pub CertId: OCSP_CERT_ID,
    pub dwCertStatus: u32,
    pub Anonymous: OCSP_BASIC_RESPONSE_ENTRY_0,
    pub ThisUpdate: super::FILETIME,
    pub NextUpdate: super::FILETIME,
    pub cExtension: u32,
    pub rgExtension: PCERT_EXTENSION,
}
#[cfg(feature = "minwindef")]
impl Default for OCSP_BASIC_RESPONSE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union OCSP_BASIC_RESPONSE_ENTRY_0 {
    pub pRevokedInfo: POCSP_BASIC_REVOKED_INFO,
}
#[cfg(feature = "minwindef")]
impl Default for OCSP_BASIC_RESPONSE_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct OCSP_BASIC_RESPONSE_INFO {
    pub dwVersion: u32,
    pub dwResponderIdChoice: u32,
    pub Anonymous: OCSP_BASIC_RESPONSE_INFO_0,
    pub ProducedAt: super::FILETIME,
    pub cResponseEntry: u32,
    pub rgResponseEntry: POCSP_BASIC_RESPONSE_ENTRY,
    pub cExtension: u32,
    pub rgExtension: PCERT_EXTENSION,
}
#[cfg(feature = "minwindef")]
impl Default for OCSP_BASIC_RESPONSE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union OCSP_BASIC_RESPONSE_INFO_0 {
    pub ByNameResponderId: CERT_NAME_BLOB,
    pub ByKeyResponderId: CRYPT_HASH_BLOB,
}
#[cfg(feature = "minwindef")]
impl Default for OCSP_BASIC_RESPONSE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OCSP_BASIC_RESPONSE_V1: u32 = 0;
pub const OCSP_BASIC_REVOKED_CERT_STATUS: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct OCSP_BASIC_REVOKED_INFO {
    pub RevocationDate: super::FILETIME,
    pub dwCrlReasonCode: u32,
}
pub const OCSP_BASIC_SIGNED_RESPONSE: windows_sys::core::PCSTR = 68 as _;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OCSP_BASIC_SIGNED_RESPONSE_INFO {
    pub ToBeSigned: CRYPT_DER_BLOB,
    pub SignatureInfo: OCSP_SIGNATURE_INFO,
}
pub const OCSP_BASIC_UNKNOWN_CERT_STATUS: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OCSP_CERT_ID {
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub IssuerNameHash: CRYPT_HASH_BLOB,
    pub IssuerKeyHash: CRYPT_HASH_BLOB,
    pub SerialNumber: CRYPT_INTEGER_BLOB,
}
pub const OCSP_INTERNAL_ERROR_RESPONSE: u32 = 2;
pub const OCSP_MALFORMED_REQUEST_RESPONSE: u32 = 1;
pub const OCSP_REQUEST: windows_sys::core::PCSTR = 66 as _;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OCSP_REQUEST_ENTRY {
    pub CertId: OCSP_CERT_ID,
    pub cExtension: u32,
    pub rgExtension: PCERT_EXTENSION,
}
impl Default for OCSP_REQUEST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OCSP_REQUEST_INFO {
    pub dwVersion: u32,
    pub pRequestorName: PCERT_ALT_NAME_ENTRY,
    pub cRequestEntry: u32,
    pub rgRequestEntry: POCSP_REQUEST_ENTRY,
    pub cExtension: u32,
    pub rgExtension: PCERT_EXTENSION,
}
impl Default for OCSP_REQUEST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OCSP_REQUEST_V1: u32 = 0;
pub const OCSP_RESPONSE: windows_sys::core::PCSTR = 67 as _;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OCSP_RESPONSE_INFO {
    pub dwStatus: u32,
    pub pszObjId: windows_sys::core::PSTR,
    pub Value: CRYPT_OBJID_BLOB,
}
impl Default for OCSP_RESPONSE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OCSP_SIGNATURE_INFO {
    pub SignatureAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Signature: CRYPT_BIT_BLOB,
    pub cCertEncoded: u32,
    pub rgCertEncoded: PCERT_BLOB,
}
impl Default for OCSP_SIGNATURE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OCSP_SIGNED_REQUEST: windows_sys::core::PCSTR = 65 as _;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OCSP_SIGNED_REQUEST_INFO {
    pub ToBeSigned: CRYPT_DER_BLOB,
    pub pOptionalSignatureInfo: POCSP_SIGNATURE_INFO,
}
impl Default for OCSP_SIGNED_REQUEST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OCSP_SIG_REQUIRED_RESPONSE: u32 = 5;
pub const OCSP_SUCCESSFUL_RESPONSE: u32 = 0;
pub const OCSP_TRY_LATER_RESPONSE: u32 = 3;
pub const OCSP_UNAUTHORIZED_RESPONSE: u32 = 6;
pub const OPAQUEKEYBLOB: u32 = 9;
pub type PAUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA = *mut AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA;
pub type PAUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS = *mut AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS;
pub type PAUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA = *mut AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA;
#[cfg(feature = "minwindef")]
pub type PCCERT_CHAIN_CONTEXT = *const CERT_CHAIN_CONTEXT;
#[cfg(feature = "minwindef")]
pub type PCCERT_CHAIN_ELEMENT = *const CERT_CHAIN_ELEMENT;
#[cfg(feature = "minwindef")]
pub type PCCERT_CONTEXT = *const CERT_CONTEXT;
#[cfg(feature = "minwindef")]
pub type PCCERT_CRL_CONTEXT_PAIR = *const CERT_CRL_CONTEXT_PAIR;
pub type PCCERT_ENHKEY_USAGE = *const CERT_ENHKEY_USAGE;
pub type PCCERT_EXTENSION = *const CERT_EXTENSION;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PCCERT_SELECT_CHAIN_PARA = *const CERT_SELECT_CHAIN_PARA;
pub type PCCERT_SELECT_CRITERIA = *const CERT_SELECT_CRITERIA;
pub type PCCERT_SERVER_OCSP_RESPONSE_CONTEXT = *const CERT_SERVER_OCSP_RESPONSE_CONTEXT;
#[cfg(feature = "minwindef")]
pub type PCCERT_SIMPLE_CHAIN = *const CERT_SIMPLE_CHAIN;
pub type PCCERT_STORE_PROV_FIND_INFO = *const CERT_STORE_PROV_FIND_INFO;
pub type PCCERT_STRONG_SIGN_PARA = *const CERT_STRONG_SIGN_PARA;
#[cfg(feature = "minwindef")]
pub type PCCRL_CONTEXT = *const CRL_CONTEXT;
pub type PCCRYPT_OID_INFO = *const CRYPT_OID_INFO;
#[cfg(feature = "minwindef")]
pub type PCCTL_CONTEXT = *const CTL_CONTEXT;
pub type PCCTL_USAGE = *const CTL_USAGE;
pub type PCERT_ACCESS_DESCRIPTION = *mut CERT_ACCESS_DESCRIPTION;
pub type PCERT_ALT_NAME_ENTRY = *mut CERT_ALT_NAME_ENTRY;
pub type PCERT_ALT_NAME_INFO = *mut CERT_ALT_NAME_INFO;
pub type PCERT_AUTHORITY_INFO_ACCESS = *mut CERT_AUTHORITY_INFO_ACCESS;
pub type PCERT_AUTHORITY_KEY_ID2_INFO = *mut CERT_AUTHORITY_KEY_ID2_INFO;
pub type PCERT_AUTHORITY_KEY_ID_INFO = *mut CERT_AUTHORITY_KEY_ID_INFO;
pub type PCERT_BASIC_CONSTRAINTS2_INFO = *mut CERT_BASIC_CONSTRAINTS2_INFO;
pub type PCERT_BASIC_CONSTRAINTS_INFO = *mut CERT_BASIC_CONSTRAINTS_INFO;
pub type PCERT_BIOMETRIC_DATA = *mut CERT_BIOMETRIC_DATA;
pub type PCERT_BIOMETRIC_EXT_INFO = *mut CERT_BIOMETRIC_EXT_INFO;
pub type PCERT_BLOB = *mut CRYPT_INTEGER_BLOB;
pub type PCERT_CHAIN = *mut CERT_CHAIN;
#[cfg(feature = "minwindef")]
pub type PCERT_CHAIN_CONTEXT = *mut CERT_CHAIN_CONTEXT;
#[cfg(feature = "minwindef")]
pub type PCERT_CHAIN_ELEMENT = *mut CERT_CHAIN_ELEMENT;
pub type PCERT_CHAIN_ENGINE_CONFIG = *mut CERT_CHAIN_ENGINE_CONFIG;
#[cfg(feature = "minwindef")]
pub type PCERT_CHAIN_FIND_BY_ISSUER_PARA = *mut CERT_CHAIN_FIND_ISSUER_PARA;
#[cfg(feature = "minwindef")]
pub type PCERT_CHAIN_FIND_ISSUER_PARA = *mut CERT_CHAIN_FIND_ISSUER_PARA;
pub type PCERT_CHAIN_PARA = *mut CERT_CHAIN_PARA;
pub type PCERT_CHAIN_POLICY_PARA = *mut CERT_CHAIN_POLICY_PARA;
pub type PCERT_CHAIN_POLICY_STATUS = *mut CERT_CHAIN_POLICY_STATUS;
#[cfg(feature = "minwindef")]
pub type PCERT_CONTEXT = *mut CERT_CONTEXT;
pub type PCERT_CREATE_CONTEXT_PARA = *mut CERT_CREATE_CONTEXT_PARA;
#[cfg(feature = "minwindef")]
pub type PCERT_CRL_CONTEXT_PAIR = *mut CERT_CRL_CONTEXT_PAIR;
pub type PCERT_DH_PARAMETERS = *mut CERT_DH_PARAMETERS;
pub type PCERT_DSS_PARAMETERS = *mut CERT_DSS_PARAMETERS;
pub type PCERT_ECC_SIGNATURE = *mut CERT_ECC_SIGNATURE;
pub type PCERT_ENHKEY_USAGE = *mut CTL_USAGE;
pub type PCERT_EXTENSION = *mut CERT_EXTENSION;
pub type PCERT_EXTENSIONS = *mut CERT_EXTENSIONS;
pub type PCERT_GENERAL_SUBTREE = *mut CERT_GENERAL_SUBTREE;
pub type PCERT_HASHED_URL = *mut CERT_HASHED_URL;
pub type PCERT_ID = *mut CERT_ID;
#[cfg(feature = "minwindef")]
pub type PCERT_INFO = *mut CERT_INFO;
pub type PCERT_ISSUER_SERIAL_NUMBER = *mut CERT_ISSUER_SERIAL_NUMBER;
pub type PCERT_KEYGEN_REQUEST_INFO = *mut CERT_KEYGEN_REQUEST_INFO;
#[cfg(feature = "minwindef")]
pub type PCERT_KEY_ATTRIBUTES_INFO = *mut CERT_KEY_ATTRIBUTES_INFO;
#[cfg(feature = "ncrypt")]
pub type PCERT_KEY_CONTEXT = *mut CERT_KEY_CONTEXT;
pub type PCERT_KEY_USAGE_RESTRICTION_INFO = *mut CERT_KEY_USAGE_RESTRICTION_INFO;
pub type PCERT_LDAP_STORE_OPENED_PARA = *mut CERT_LDAP_STORE_OPENED_PARA;
pub type PCERT_LOGOTYPE_AUDIO = *mut CERT_LOGOTYPE_AUDIO;
pub type PCERT_LOGOTYPE_AUDIO_INFO = *mut CERT_LOGOTYPE_AUDIO_INFO;
pub type PCERT_LOGOTYPE_DATA = *mut CERT_LOGOTYPE_DATA;
pub type PCERT_LOGOTYPE_DETAILS = *mut CERT_LOGOTYPE_DETAILS;
pub type PCERT_LOGOTYPE_EXT_INFO = *mut CERT_LOGOTYPE_EXT_INFO;
pub type PCERT_LOGOTYPE_IMAGE = *mut CERT_LOGOTYPE_IMAGE;
pub type PCERT_LOGOTYPE_IMAGE_INFO = *mut CERT_LOGOTYPE_IMAGE_INFO;
pub type PCERT_LOGOTYPE_INFO = *mut CERT_LOGOTYPE_INFO;
pub type PCERT_LOGOTYPE_REFERENCE = *mut CERT_LOGOTYPE_REFERENCE;
pub type PCERT_NAME_BLOB = *mut CRYPT_INTEGER_BLOB;
pub type PCERT_NAME_CONSTRAINTS_INFO = *mut CERT_NAME_CONSTRAINTS_INFO;
pub type PCERT_NAME_INFO = *mut CERT_NAME_INFO;
pub type PCERT_NAME_VALUE = *mut CERT_NAME_VALUE;
pub type PCERT_OR_CRL_BLOB = *mut CERT_OR_CRL_BLOB;
pub type PCERT_OR_CRL_BUNDLE = *mut CERT_OR_CRL_BUNDLE;
pub type PCERT_OTHER_LOGOTYPE_INFO = *mut CERT_OTHER_LOGOTYPE_INFO;
pub type PCERT_OTHER_NAME = *mut CERT_OTHER_NAME;
pub type PCERT_PAIR = *mut CERT_PAIR;
pub type PCERT_PHYSICAL_STORE_INFO = *mut CERT_PHYSICAL_STORE_INFO;
pub type PCERT_POLICIES_INFO = *mut CERT_POLICIES_INFO;
pub type PCERT_POLICY95_QUALIFIER1 = *mut CERT_POLICY95_QUALIFIER1;
pub type PCERT_POLICY_CONSTRAINTS_INFO = *mut CERT_POLICY_CONSTRAINTS_INFO;
pub type PCERT_POLICY_ID = *mut CERT_POLICY_ID;
pub type PCERT_POLICY_INFO = *mut CERT_POLICY_INFO;
pub type PCERT_POLICY_MAPPING = *mut CERT_POLICY_MAPPING;
pub type PCERT_POLICY_MAPPINGS_INFO = *mut CERT_POLICY_MAPPINGS_INFO;
pub type PCERT_POLICY_QUALIFIER_INFO = *mut CERT_POLICY_QUALIFIER_INFO;
pub type PCERT_POLICY_QUALIFIER_NOTICE_REFERENCE = *mut CERT_POLICY_QUALIFIER_NOTICE_REFERENCE;
pub type PCERT_POLICY_QUALIFIER_USER_NOTICE = *mut CERT_POLICY_QUALIFIER_USER_NOTICE;
#[cfg(feature = "minwindef")]
pub type PCERT_PRIVATE_KEY_VALIDITY = *mut CERT_PRIVATE_KEY_VALIDITY;
pub type PCERT_PUBLIC_KEY_INFO = *mut CERT_PUBLIC_KEY_INFO;
pub type PCERT_QC_STATEMENT = *mut CERT_QC_STATEMENT;
pub type PCERT_QC_STATEMENTS_EXT_INFO = *mut CERT_QC_STATEMENTS_EXT_INFO;
pub type PCERT_RDN = *mut CERT_RDN;
pub type PCERT_RDN_ATTR = *mut CERT_RDN_ATTR;
pub type PCERT_RDN_VALUE_BLOB = *mut CRYPT_INTEGER_BLOB;
#[cfg(feature = "minwindef")]
pub type PCERT_REGISTRY_STORE_CLIENT_GPT_PARA = *mut CERT_REGISTRY_STORE_CLIENT_GPT_PARA;
#[cfg(feature = "minwindef")]
pub type PCERT_REGISTRY_STORE_ROAMING_PARA = *mut CERT_REGISTRY_STORE_ROAMING_PARA;
pub type PCERT_REQUEST_INFO = *mut CERT_REQUEST_INFO;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PCERT_REVOCATION_CHAIN_PARA = *mut CERT_REVOCATION_CHAIN_PARA;
#[cfg(feature = "minwindef")]
pub type PCERT_REVOCATION_CRL_INFO = *mut CERT_REVOCATION_CRL_INFO;
#[cfg(feature = "minwindef")]
pub type PCERT_REVOCATION_INFO = *mut CERT_REVOCATION_INFO;
#[cfg(feature = "minwindef")]
pub type PCERT_REVOCATION_PARA = *mut CERT_REVOCATION_PARA;
pub type PCERT_REVOCATION_STATUS = *mut CERT_REVOCATION_STATUS;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PCERT_SELECT_CHAIN_PARA = *mut CERT_SELECT_CHAIN_PARA;
pub type PCERT_SELECT_CRITERIA = *mut CERT_SELECT_CRITERIA;
pub type PCERT_SERVER_OCSP_RESPONSE_CONTEXT = *mut CERT_SERVER_OCSP_RESPONSE_CONTEXT;
#[cfg(feature = "minwindef")]
pub type PCERT_SERVER_OCSP_RESPONSE_OPEN_PARA = *mut CERT_SERVER_OCSP_RESPONSE_OPEN_PARA;
pub type PCERT_SIGNED_CONTENT_INFO = *mut CERT_SIGNED_CONTENT_INFO;
#[cfg(feature = "minwindef")]
pub type PCERT_SIMPLE_CHAIN = *mut CERT_SIMPLE_CHAIN;
pub type PCERT_STORE_PROV_FIND_INFO = *mut CERT_STORE_PROV_FIND_INFO;
pub type PCERT_STORE_PROV_INFO = *mut CERT_STORE_PROV_INFO;
pub type PCERT_STRONG_SIGN_PARA = *mut CERT_STRONG_SIGN_PARA;
pub type PCERT_STRONG_SIGN_SERIALIZED_INFO = *mut CERT_STRONG_SIGN_SERIALIZED_INFO;
pub type PCERT_SUBJECT_INFO_ACCESS = *mut CERT_AUTHORITY_INFO_ACCESS;
pub type PCERT_SUPPORTED_ALGORITHM_INFO = *mut CERT_SUPPORTED_ALGORITHM_INFO;
pub type PCERT_SYSTEM_STORE_INFO = *mut CERT_SYSTEM_STORE_INFO;
#[cfg(feature = "minwindef")]
pub type PCERT_SYSTEM_STORE_RELOCATE_PARA = *mut CERT_SYSTEM_STORE_RELOCATE_PARA;
pub type PCERT_TEMPLATE_EXT = *mut CERT_TEMPLATE_EXT;
pub type PCERT_TPM_SPECIFICATION_INFO = *mut CERT_TPM_SPECIFICATION_INFO;
#[cfg(feature = "minwindef")]
pub type PCERT_TRUST_LIST_INFO = *mut CERT_TRUST_LIST_INFO;
pub type PCERT_TRUST_STATUS = *mut CERT_TRUST_STATUS;
pub type PCERT_USAGE_MATCH = *mut CERT_USAGE_MATCH;
pub type PCERT_X942_DH_PARAMETERS = *mut CERT_X942_DH_PARAMETERS;
pub type PCERT_X942_DH_VALIDATION_PARAMS = *mut CERT_X942_DH_VALIDATION_PARAMS;
pub type PCMC_ADD_ATTRIBUTES_INFO = *mut CMC_ADD_ATTRIBUTES_INFO;
pub type PCMC_ADD_EXTENSIONS_INFO = *mut CMC_ADD_EXTENSIONS_INFO;
pub type PCMC_DATA_INFO = *mut CMC_DATA_INFO;
#[cfg(feature = "minwindef")]
pub type PCMC_PEND_INFO = *mut CMC_PEND_INFO;
pub type PCMC_RESPONSE_INFO = *mut CMC_RESPONSE_INFO;
#[cfg(feature = "minwindef")]
pub type PCMC_STATUS_INFO = *mut CMC_STATUS_INFO;
pub type PCMC_TAGGED_ATTRIBUTE = *mut CMC_TAGGED_ATTRIBUTE;
pub type PCMC_TAGGED_CERT_REQUEST = *mut CMC_TAGGED_CERT_REQUEST;
pub type PCMC_TAGGED_CONTENT_INFO = *mut CMC_TAGGED_CONTENT_INFO;
pub type PCMC_TAGGED_OTHER_MSG = *mut CMC_TAGGED_OTHER_MSG;
pub type PCMC_TAGGED_REQUEST = *mut CMC_TAGGED_REQUEST;
pub type PCMSG_ATTR = *mut CRYPT_ATTRIBUTES;
#[cfg(feature = "minwindef")]
pub type PCMSG_CMS_RECIPIENT_INFO = *mut CMSG_CMS_RECIPIENT_INFO;
pub type PCMSG_CMS_SIGNER_INFO = *mut CMSG_CMS_SIGNER_INFO;
#[cfg(all(feature = "bcrypt", feature = "ncrypt"))]
pub type PCMSG_CNG_CONTENT_DECRYPT_INFO = *mut CMSG_CNG_CONTENT_DECRYPT_INFO;
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
pub type PCMSG_CONTENT_ENCRYPT_INFO = *mut CMSG_CONTENT_ENCRYPT_INFO;
pub type PCMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA = *mut CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA;
#[cfg(feature = "ncrypt")]
pub type PCMSG_CTRL_DECRYPT_PARA = *mut CMSG_CTRL_DECRYPT_PARA;
pub type PCMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA = *mut CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA;
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
pub type PCMSG_CTRL_KEY_AGREE_DECRYPT_PARA = *mut CMSG_CTRL_KEY_AGREE_DECRYPT_PARA;
#[cfg(feature = "ncrypt")]
pub type PCMSG_CTRL_KEY_TRANS_DECRYPT_PARA = *mut CMSG_CTRL_KEY_TRANS_DECRYPT_PARA;
#[cfg(feature = "minwindef")]
pub type PCMSG_CTRL_MAIL_LIST_DECRYPT_PARA = *mut CMSG_CTRL_MAIL_LIST_DECRYPT_PARA;
pub type PCMSG_CTRL_VERIFY_SIGNATURE_EX_PARA = *mut CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA;
pub type PCMSG_ENCRYPTED_ENCODE_INFO = *mut CMSG_ENCRYPTED_ENCODE_INFO;
#[cfg(feature = "minwindef")]
pub type PCMSG_ENVELOPED_ENCODE_INFO = *mut CMSG_ENVELOPED_ENCODE_INFO;
pub type PCMSG_HASHED_ENCODE_INFO = *mut CMSG_HASHED_ENCODE_INFO;
pub type PCMSG_KEY_AGREE_ENCRYPT_INFO = *mut CMSG_KEY_AGREE_ENCRYPT_INFO;
pub type PCMSG_KEY_AGREE_KEY_ENCRYPT_INFO = *mut CMSG_KEY_AGREE_KEY_ENCRYPT_INFO;
#[cfg(feature = "minwindef")]
pub type PCMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO = *mut CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO;
#[cfg(feature = "minwindef")]
pub type PCMSG_KEY_AGREE_RECIPIENT_INFO = *mut CMSG_KEY_AGREE_RECIPIENT_INFO;
pub type PCMSG_KEY_TRANS_ENCRYPT_INFO = *mut CMSG_KEY_TRANS_ENCRYPT_INFO;
pub type PCMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO = *mut CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO;
pub type PCMSG_KEY_TRANS_RECIPIENT_INFO = *mut CMSG_KEY_TRANS_RECIPIENT_INFO;
pub type PCMSG_MAIL_LIST_ENCRYPT_INFO = *mut CMSG_MAIL_LIST_ENCRYPT_INFO;
#[cfg(feature = "minwindef")]
pub type PCMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO = *mut CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO;
#[cfg(feature = "minwindef")]
pub type PCMSG_MAIL_LIST_RECIPIENT_INFO = *mut CMSG_MAIL_LIST_RECIPIENT_INFO;
pub type PCMSG_RC2_AUX_INFO = *mut CMSG_RC2_AUX_INFO;
pub type PCMSG_RC4_AUX_INFO = *mut CMSG_RC4_AUX_INFO;
#[cfg(feature = "minwindef")]
pub type PCMSG_RECIPIENT_ENCODE_INFO = *mut CMSG_RECIPIENT_ENCODE_INFO;
#[cfg(feature = "minwindef")]
pub type PCMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO = *mut CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO;
#[cfg(feature = "minwindef")]
pub type PCMSG_RECIPIENT_ENCRYPTED_KEY_INFO = *mut CMSG_RECIPIENT_ENCRYPTED_KEY_INFO;
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
pub type PCMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO = *mut CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO;
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
pub type PCMSG_SIGNED_ENCODE_INFO = *mut CMSG_SIGNED_ENCODE_INFO;
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
pub type PCMSG_SIGNER_ENCODE_INFO = *mut CMSG_SIGNER_ENCODE_INFO;
pub type PCMSG_SIGNER_INFO = *mut CMSG_SIGNER_INFO;
pub type PCMSG_SP3_COMPATIBLE_AUX_INFO = *mut CMSG_SP3_COMPATIBLE_AUX_INFO;
pub type PCMSG_STREAM_INFO = *mut CMSG_STREAM_INFO;
pub type PCMS_DH_KEY_INFO = *mut CMS_DH_KEY_INFO;
pub type PCMS_KEY_INFO = *mut CMS_KEY_INFO;
pub type PCPS_URLS = *mut CPS_URLS;
pub type PCRL_BLOB = *mut CRYPT_INTEGER_BLOB;
#[cfg(feature = "minwindef")]
pub type PCRL_CONTEXT = *mut CRL_CONTEXT;
pub type PCRL_DIST_POINT = *mut CRL_DIST_POINT;
pub type PCRL_DIST_POINTS_INFO = *mut CRL_DIST_POINTS_INFO;
pub type PCRL_DIST_POINT_NAME = *mut CRL_DIST_POINT_NAME;
#[cfg(feature = "minwindef")]
pub type PCRL_ENTRY = *mut CRL_ENTRY;
#[cfg(feature = "minwindef")]
pub type PCRL_FIND_ISSUED_FOR_PARA = *mut CRL_FIND_ISSUED_FOR_PARA;
#[cfg(feature = "minwindef")]
pub type PCRL_INFO = *mut CRL_INFO;
pub type PCRL_ISSUING_DIST_POINT = *mut CRL_ISSUING_DIST_POINT;
#[cfg(feature = "minwindef")]
pub type PCRL_REVOCATION_INFO = *mut CRL_REVOCATION_INFO;
pub type PCROSS_CERT_DIST_POINTS_INFO = *mut CROSS_CERT_DIST_POINTS_INFO;
#[cfg(feature = "minwindef")]
pub type PCRYPTNET_URL_CACHE_FLUSH_INFO = *mut CRYPTNET_URL_CACHE_FLUSH_INFO;
#[cfg(feature = "minwindef")]
pub type PCRYPTNET_URL_CACHE_PRE_FETCH_INFO = *mut CRYPTNET_URL_CACHE_PRE_FETCH_INFO;
#[cfg(feature = "minwindef")]
pub type PCRYPTNET_URL_CACHE_RESPONSE_INFO = *mut CRYPTNET_URL_CACHE_RESPONSE_INFO;
pub type PCRYPT_3DES_KEY_STATE = *mut CRYPT_3DES_KEY_STATE;
pub type PCRYPT_AES_128_KEY_STATE = *mut CRYPT_AES_128_KEY_STATE;
pub type PCRYPT_AES_256_KEY_STATE = *mut CRYPT_AES_256_KEY_STATE;
pub type PCRYPT_ALGORITHM_IDENTIFIER = *mut CRYPT_ALGORITHM_IDENTIFIER;
pub type PCRYPT_ASYNC_RETRIEVAL_COMPLETION = *mut CRYPT_ASYNC_RETRIEVAL_COMPLETION;
pub type PCRYPT_ATTRIBUTE = *mut CRYPT_ATTRIBUTE;
pub type PCRYPT_ATTRIBUTES = *mut CRYPT_ATTRIBUTES;
pub type PCRYPT_ATTRIBUTE_TYPE_VALUE = *mut CRYPT_ATTRIBUTE_TYPE_VALUE;
pub type PCRYPT_ATTR_BLOB = *mut CRYPT_INTEGER_BLOB;
pub type PCRYPT_BIT_BLOB = *mut CRYPT_BIT_BLOB;
pub type PCRYPT_BLOB_ARRAY = *mut CRYPT_BLOB_ARRAY;
pub type PCRYPT_CONTENT_INFO = *mut CRYPT_CONTENT_INFO;
pub type PCRYPT_CONTENT_INFO_SEQUENCE_OF_ANY = *mut CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY;
pub type PCRYPT_CREDENTIALS = *mut CRYPT_CREDENTIALS;
pub type PCRYPT_CSP_PROVIDER = *mut CRYPT_CSP_PROVIDER;
pub type PCRYPT_DATA_BLOB = *mut CRYPT_INTEGER_BLOB;
pub type PCRYPT_DECODE_PARA = *mut CRYPT_DECODE_PARA;
pub type PCRYPT_DECRYPT_MESSAGE_PARA = *mut CRYPT_DECRYPT_MESSAGE_PARA;
pub type PCRYPT_DECRYPT_PRIVATE_KEY_FUNC = Option<unsafe extern "system" fn(algorithm: CRYPT_ALGORITHM_IDENTIFIER, encryptedprivatekey: CRYPT_DATA_BLOB, pbcleartextkey: *mut u8, pcbcleartextkey: *mut u32, pvoiddecryptfunc: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PCRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA = *mut CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA;
pub type PCRYPT_DER_BLOB = *mut CRYPT_INTEGER_BLOB;
pub type PCRYPT_DES_KEY_STATE = *mut CRYPT_DES_KEY_STATE;
pub type PCRYPT_DIGEST_BLOB = *mut CRYPT_INTEGER_BLOB;
pub type PCRYPT_ECC_CMS_SHARED_INFO = *mut CRYPT_ECC_CMS_SHARED_INFO;
pub type PCRYPT_ECC_PRIVATE_KEY_INFO = *mut CRYPT_ECC_PRIVATE_KEY_INFO;
pub type PCRYPT_ENCODE_PARA = *mut CRYPT_ENCODE_PARA;
pub type PCRYPT_ENCRYPTED_PRIVATE_KEY_INFO = *mut CRYPT_ENCRYPTED_PRIVATE_KEY_INFO;
pub type PCRYPT_ENCRYPT_MESSAGE_PARA = *mut CRYPT_ENCRYPT_MESSAGE_PARA;
pub type PCRYPT_ENCRYPT_PRIVATE_KEY_FUNC = Option<unsafe extern "system" fn(palgorithm: *mut CRYPT_ALGORITHM_IDENTIFIER, pcleartextprivatekey: *const CRYPT_DATA_BLOB, pbencryptedkey: *mut u8, pcbencryptedkey: *mut u32, pvoidencryptfunc: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PCRYPT_ENROLLMENT_NAME_VALUE_PAIR = *mut CRYPT_ENROLLMENT_NAME_VALUE_PAIR;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PCRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO = *mut CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO;
pub type PCRYPT_HASH_BLOB = *mut CRYPT_INTEGER_BLOB;
pub type PCRYPT_HASH_INFO = *mut CRYPT_HASH_INFO;
pub type PCRYPT_HASH_MESSAGE_PARA = *mut CRYPT_HASH_MESSAGE_PARA;
pub type PCRYPT_INTEGER_BLOB = *mut CRYPT_INTEGER_BLOB;
pub type PCRYPT_KEY_PROV_INFO = *mut CRYPT_KEY_PROV_INFO;
pub type PCRYPT_KEY_PROV_PARAM = *mut CRYPT_KEY_PROV_PARAM;
#[cfg(feature = "ncrypt")]
pub type PCRYPT_KEY_SIGN_MESSAGE_PARA = *mut CRYPT_KEY_SIGN_MESSAGE_PARA;
pub type PCRYPT_KEY_VERIFY_MESSAGE_PARA = *mut CRYPT_KEY_VERIFY_MESSAGE_PARA;
pub type PCRYPT_MASK_GEN_ALGORITHM = *mut CRYPT_MASK_GEN_ALGORITHM;
#[cfg(feature = "minwindef")]
pub type PCRYPT_OBJECT_LOCATOR_PROVIDER_TABLE = *mut CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE;
pub type PCRYPT_OBJID_BLOB = *mut CRYPT_INTEGER_BLOB;
pub type PCRYPT_OBJID_TABLE = *mut CRYPT_OBJID_TABLE;
pub type PCRYPT_OID_FUNC_ENTRY = *mut CRYPT_OID_FUNC_ENTRY;
pub type PCRYPT_OID_INFO = *mut CRYPT_OID_INFO;
pub type PCRYPT_PASSWORD_CREDENTIALS = PCRYPT_PASSWORD_CREDENTIALSA;
pub type PCRYPT_PASSWORD_CREDENTIALSA = *mut CRYPT_PASSWORD_CREDENTIALSA;
pub type PCRYPT_PASSWORD_CREDENTIALSW = *mut CRYPT_PASSWORD_CREDENTIALSW;
pub type PCRYPT_PKCS8_EXPORT_PARAMS = *mut CRYPT_PKCS8_EXPORT_PARAMS;
pub type PCRYPT_PKCS8_IMPORT_PARAMS = *mut CRYPT_PKCS8_IMPORT_PARAMS;
pub type PCRYPT_PRIVATE_KEY_BLOB_AND_PARAMS = *mut CRYPT_PKCS8_IMPORT_PARAMS;
pub type PCRYPT_PRIVATE_KEY_INFO = *mut CRYPT_PRIVATE_KEY_INFO;
pub type PCRYPT_PSOURCE_ALGORITHM = *mut CRYPT_PSOURCE_ALGORITHM;
pub type PCRYPT_RC2_CBC_PARAMETERS = *mut CRYPT_RC2_CBC_PARAMETERS;
pub type PCRYPT_RC4_KEY_STATE = *mut CRYPT_RC4_KEY_STATE;
pub type PCRYPT_RESOLVE_HCRYPTPROV_FUNC = Option<unsafe extern "system" fn(pprivatekeyinfo: *mut CRYPT_PRIVATE_KEY_INFO, phcryptprov: *mut HCRYPTPROV, pvoidresolvefunc: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PCRYPT_RETRIEVE_AUX_INFO = *mut CRYPT_RETRIEVE_AUX_INFO;
pub type PCRYPT_RSAES_OAEP_PARAMETERS = *mut CRYPT_RSAES_OAEP_PARAMETERS;
pub type PCRYPT_RSA_SSA_PSS_PARAMETERS = *mut CRYPT_RSA_SSA_PSS_PARAMETERS;
pub type PCRYPT_SEQUENCE_OF_ANY = *mut CRYPT_SEQUENCE_OF_ANY;
#[cfg(feature = "minwindef")]
pub type PCRYPT_SIGN_MESSAGE_PARA = *mut CRYPT_SIGN_MESSAGE_PARA;
pub type PCRYPT_SMART_CARD_ROOT_INFO = *mut CRYPT_SMART_CARD_ROOT_INFO;
pub type PCRYPT_SMIME_CAPABILITIES = *mut CRYPT_SMIME_CAPABILITIES;
pub type PCRYPT_SMIME_CAPABILITY = *mut CRYPT_SMIME_CAPABILITY;
pub type PCRYPT_TIMESTAMP_ACCURACY = *mut CRYPT_TIMESTAMP_ACCURACY;
#[cfg(feature = "minwindef")]
pub type PCRYPT_TIMESTAMP_CONTEXT = *mut CRYPT_TIMESTAMP_CONTEXT;
#[cfg(feature = "minwindef")]
pub type PCRYPT_TIMESTAMP_INFO = *mut CRYPT_TIMESTAMP_INFO;
pub type PCRYPT_TIMESTAMP_PARA = *mut CRYPT_TIMESTAMP_PARA;
pub type PCRYPT_TIMESTAMP_REQUEST = *mut CRYPT_TIMESTAMP_REQUEST;
pub type PCRYPT_TIMESTAMP_RESPONSE = *mut CRYPT_TIMESTAMP_RESPONSE;
pub type PCRYPT_TIME_STAMP_REQUEST_INFO = *mut CRYPT_TIME_STAMP_REQUEST_INFO;
pub type PCRYPT_UINT_BLOB = *mut CRYPT_INTEGER_BLOB;
pub type PCRYPT_URL_ARRAY = *mut CRYPT_URL_ARRAY;
pub type PCRYPT_URL_INFO = *mut CRYPT_URL_INFO;
pub type PCRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO = *mut CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO;
pub type PCRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO = *mut CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO;
#[cfg(feature = "minwindef")]
pub type PCRYPT_VERIFY_MESSAGE_PARA = *mut CRYPT_VERIFY_MESSAGE_PARA;
pub type PCRYPT_X942_OTHER_INFO = *mut CRYPT_X942_OTHER_INFO;
pub type PCTL_ANY_SUBJECT_INFO = *mut CTL_ANY_SUBJECT_INFO;
#[cfg(feature = "minwindef")]
pub type PCTL_CONTEXT = *mut CTL_CONTEXT;
pub type PCTL_ENTRY = *mut CTL_ENTRY;
#[cfg(feature = "minwindef")]
pub type PCTL_FIND_SUBJECT_PARA = *mut CTL_FIND_SUBJECT_PARA;
#[cfg(feature = "minwindef")]
pub type PCTL_FIND_USAGE_PARA = *mut CTL_FIND_USAGE_PARA;
#[cfg(feature = "minwindef")]
pub type PCTL_INFO = *mut CTL_INFO;
pub type PCTL_USAGE = *mut CTL_USAGE;
pub type PCTL_USAGE_MATCH = *mut CTL_USAGE_MATCH;
pub type PCTL_VERIFY_USAGE_PARA = *mut CTL_VERIFY_USAGE_PARA;
#[cfg(feature = "minwindef")]
pub type PCTL_VERIFY_USAGE_STATUS = *mut CTL_VERIFY_USAGE_STATUS;
pub type PCT_EXTRA_CERT_CHAIN_POLICY_STATUS = *mut CT_EXTRA_CERT_CHAIN_POLICY_STATUS;
pub type PDATA_BLOB = *mut CRYPT_INTEGER_BLOB;
pub type PEV_EXTRA_CERT_CHAIN_POLICY_PARA = *mut EV_EXTRA_CERT_CHAIN_POLICY_PARA;
pub type PEV_EXTRA_CERT_CHAIN_POLICY_STATUS = *mut EV_EXTRA_CERT_CHAIN_POLICY_STATUS;
#[cfg(feature = "winnt")]
pub type PFN_CANCEL_ASYNC_RETRIEVAL_FUNC = Option<unsafe extern "system" fn(hasyncretrieve: HCRYPTASYNC) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_CHAIN_FIND_BY_ISSUER_CALLBACK = Option<unsafe extern "system" fn(pcert: *const CERT_CONTEXT, pvfindarg: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PFN_CERT_CREATE_CONTEXT_SORT_FUNC = Option<unsafe extern "system" fn(cbtotalencoded: u32, cbremainencoded: u32, centry: u32, pvsort: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PFN_CERT_DLL_OPEN_STORE_PROV_FUNC = Option<unsafe extern "system" fn(lpszstoreprovider: windows_sys::core::PCSTR, dwencodingtype: u32, hcryptprov: HCRYPTPROV_LEGACY, dwflags: u32, pvpara: *const core::ffi::c_void, hcertstore: HCERTSTORE, pstoreprovinfo: *mut CERT_STORE_PROV_INFO) -> windows_sys::core::BOOL>;
pub type PFN_CERT_ENUM_PHYSICAL_STORE = Option<unsafe extern "system" fn(pvsystemstore: *const core::ffi::c_void, dwflags: u32, pwszstorename: windows_sys::core::PCWSTR, pstoreinfo: *const CERT_PHYSICAL_STORE_INFO, pvreserved: *const core::ffi::c_void, pvarg: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PFN_CERT_ENUM_SYSTEM_STORE = Option<unsafe extern "system" fn(pvsystemstore: *const core::ffi::c_void, dwflags: u32, pstoreinfo: *const CERT_SYSTEM_STORE_INFO, pvreserved: *const core::ffi::c_void, pvarg: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PFN_CERT_ENUM_SYSTEM_STORE_LOCATION = Option<unsafe extern "system" fn(pwszstorelocation: windows_sys::core::PCWSTR, dwflags: u32, pvreserved: *const core::ffi::c_void, pvarg: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_IS_WEAK_HASH = Option<unsafe extern "system" fn(dwhashusetype: u32, pwszcnghashalgid: windows_sys::core::PCWSTR, dwchainflags: u32, psignerchaincontext: *const CERT_CHAIN_CONTEXT, ptimestamp: *const super::FILETIME, pwszfilename: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_SERVER_OCSP_RESPONSE_UPDATE_CALLBACK = Option<unsafe extern "system" fn(pchaincontext: *const CERT_CHAIN_CONTEXT, pserverocspresponsecontext: *const CERT_SERVER_OCSP_RESPONSE_CONTEXT, pnewcrlcontext: *const CRL_CONTEXT, pprevcrlcontext: *const CRL_CONTEXT, pvarg: *mut core::ffi::c_void, dwwriteocspfileerror: u32)>;
pub type PFN_CERT_STORE_PROV_CLOSE = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, dwflags: u32)>;
pub type PFN_CERT_STORE_PROV_CONTROL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, dwflags: u32, dwctrltype: u32, pvctrlpara: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_DELETE_CERT = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcertcontext: *const CERT_CONTEXT, dwflags: u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_DELETE_CRL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcrlcontext: *const CRL_CONTEXT, dwflags: u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_DELETE_CTL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pctlcontext: *const CTL_CONTEXT, dwflags: u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_FIND_CERT = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pfindinfo: *const CERT_STORE_PROV_FIND_INFO, pprevcertcontext: *const CERT_CONTEXT, dwflags: u32, ppvstoreprovfindinfo: *mut *mut core::ffi::c_void, ppprovcertcontext: *mut PCCERT_CONTEXT) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_FIND_CRL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pfindinfo: *const CERT_STORE_PROV_FIND_INFO, pprevcrlcontext: *const CRL_CONTEXT, dwflags: u32, ppvstoreprovfindinfo: *mut *mut core::ffi::c_void, ppprovcrlcontext: *mut PCCRL_CONTEXT) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_FIND_CTL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pfindinfo: *const CERT_STORE_PROV_FIND_INFO, pprevctlcontext: *const CTL_CONTEXT, dwflags: u32, ppvstoreprovfindinfo: *mut *mut core::ffi::c_void, ppprovctlcontext: *mut PCCTL_CONTEXT) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_FREE_FIND_CERT = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcertcontext: *const CERT_CONTEXT, pvstoreprovfindinfo: *const core::ffi::c_void, dwflags: u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_FREE_FIND_CRL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcrlcontext: *const CRL_CONTEXT, pvstoreprovfindinfo: *const core::ffi::c_void, dwflags: u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_FREE_FIND_CTL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pctlcontext: *const CTL_CONTEXT, pvstoreprovfindinfo: *const core::ffi::c_void, dwflags: u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_GET_CERT_PROPERTY = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcertcontext: *const CERT_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *mut core::ffi::c_void, pcbdata: *mut u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_GET_CRL_PROPERTY = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcrlcontext: *const CRL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *mut core::ffi::c_void, pcbdata: *mut u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_GET_CTL_PROPERTY = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pctlcontext: *const CTL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *mut core::ffi::c_void, pcbdata: *mut u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_READ_CERT = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pstorecertcontext: *const CERT_CONTEXT, dwflags: u32, ppprovcertcontext: *mut PCCERT_CONTEXT) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_READ_CRL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pstorecrlcontext: *const CRL_CONTEXT, dwflags: u32, ppprovcrlcontext: *mut PCCRL_CONTEXT) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_READ_CTL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pstorectlcontext: *const CTL_CONTEXT, dwflags: u32, ppprovctlcontext: *mut PCCTL_CONTEXT) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_SET_CERT_PROPERTY = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcertcontext: *const CERT_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_SET_CRL_PROPERTY = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcrlcontext: *const CRL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_SET_CTL_PROPERTY = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pctlcontext: *const CTL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_WRITE_CERT = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcertcontext: *const CERT_CONTEXT, dwflags: u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_WRITE_CRL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcrlcontext: *const CRL_CONTEXT, dwflags: u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CERT_STORE_PROV_WRITE_CTL = Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pctlcontext: *const CTL_CONTEXT, dwflags: u32) -> windows_sys::core::BOOL>;
pub type PFN_CMSG_ALLOC = Option<unsafe extern "system" fn(cb: usize) -> *mut core::ffi::c_void>;
#[cfg(all(feature = "bcrypt", feature = "ncrypt"))]
pub type PFN_CMSG_CNG_IMPORT_CONTENT_ENCRYPT_KEY = Option<unsafe extern "system" fn(pcngcontentdecryptinfo: *mut CMSG_CNG_CONTENT_DECRYPT_INFO, dwflags: u32, pvreserved: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "bcrypt", feature = "minwindef", feature = "ncrypt"))]
pub type PFN_CMSG_CNG_IMPORT_KEY_AGREE = Option<unsafe extern "system" fn(pcngcontentdecryptinfo: *mut CMSG_CNG_CONTENT_DECRYPT_INFO, pkeyagreedecryptpara: *const CMSG_CTRL_KEY_AGREE_DECRYPT_PARA, dwflags: u32, pvreserved: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "bcrypt", feature = "ncrypt"))]
pub type PFN_CMSG_CNG_IMPORT_KEY_TRANS = Option<unsafe extern "system" fn(pcngcontentdecryptinfo: *mut CMSG_CNG_CONTENT_DECRYPT_INFO, pkeytransdecryptpara: *const CMSG_CTRL_KEY_TRANS_DECRYPT_PARA, dwflags: u32, pvreserved: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PFN_CMSG_EXPORT_ENCRYPT_KEY = Option<unsafe extern "system" fn(hcryptprov: HCRYPTPROV, hencryptkey: HCRYPTKEY, ppublickeyinfo: *const CERT_PUBLIC_KEY_INFO, pbdata: *mut u8, pcbdata: *mut u32) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
pub type PFN_CMSG_EXPORT_KEY_AGREE = Option<unsafe extern "system" fn(pcontentencryptinfo: *const CMSG_CONTENT_ENCRYPT_INFO, pkeyagreeencodeinfo: *const CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO, pkeyagreeencryptinfo: *mut CMSG_KEY_AGREE_ENCRYPT_INFO, dwflags: u32, pvreserved: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
pub type PFN_CMSG_EXPORT_KEY_TRANS = Option<unsafe extern "system" fn(pcontentencryptinfo: *const CMSG_CONTENT_ENCRYPT_INFO, pkeytransencodeinfo: *const CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO, pkeytransencryptinfo: *mut CMSG_KEY_TRANS_ENCRYPT_INFO, dwflags: u32, pvreserved: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
pub type PFN_CMSG_EXPORT_MAIL_LIST = Option<unsafe extern "system" fn(pcontentencryptinfo: *const CMSG_CONTENT_ENCRYPT_INFO, pmaillistencodeinfo: *const CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO, pmaillistencryptinfo: *mut CMSG_MAIL_LIST_ENCRYPT_INFO, dwflags: u32, pvreserved: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PFN_CMSG_FREE = Option<unsafe extern "system" fn(pv: *mut core::ffi::c_void)>;
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
pub type PFN_CMSG_GEN_CONTENT_ENCRYPT_KEY = Option<unsafe extern "system" fn(pcontentencryptinfo: *mut CMSG_CONTENT_ENCRYPT_INFO, dwflags: u32, pvreserved: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CMSG_GEN_ENCRYPT_KEY = Option<unsafe extern "system" fn(phcryptprov: *mut HCRYPTPROV, paiencrypt: *const CRYPT_ALGORITHM_IDENTIFIER, pvencryptauxinfo: *const core::ffi::c_void, ppublickeyinfo: *const CERT_PUBLIC_KEY_INFO, pfnalloc: PFN_CMSG_ALLOC, phencryptkey: *mut HCRYPTKEY, ppbencryptparameters: *mut super::PBYTE, pcbencryptparameters: *mut u32) -> windows_sys::core::BOOL>;
pub type PFN_CMSG_IMPORT_ENCRYPT_KEY = Option<unsafe extern "system" fn(hcryptprov: HCRYPTPROV, dwkeyspec: u32, paiencrypt: *const CRYPT_ALGORITHM_IDENTIFIER, paipubkey: *const CRYPT_ALGORITHM_IDENTIFIER, pbencodedkey: *const u8, cbencodedkey: u32, phencryptkey: *mut HCRYPTKEY) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "ncrypt"))]
pub type PFN_CMSG_IMPORT_KEY_AGREE = Option<unsafe extern "system" fn(pcontentencryptionalgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pkeyagreedecryptpara: *const CMSG_CTRL_KEY_AGREE_DECRYPT_PARA, dwflags: u32, pvreserved: *const core::ffi::c_void, phcontentencryptkey: *mut HCRYPTKEY) -> windows_sys::core::BOOL>;
#[cfg(feature = "ncrypt")]
pub type PFN_CMSG_IMPORT_KEY_TRANS = Option<unsafe extern "system" fn(pcontentencryptionalgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pkeytransdecryptpara: *const CMSG_CTRL_KEY_TRANS_DECRYPT_PARA, dwflags: u32, pvreserved: *const core::ffi::c_void, phcontentencryptkey: *mut HCRYPTKEY) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CMSG_IMPORT_MAIL_LIST = Option<unsafe extern "system" fn(pcontentencryptionalgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pmaillistdecryptpara: *const CMSG_CTRL_MAIL_LIST_DECRYPT_PARA, dwflags: u32, pvreserved: *const core::ffi::c_void, phcontentencryptkey: *mut HCRYPTKEY) -> windows_sys::core::BOOL>;
pub type PFN_CMSG_STREAM_OUTPUT = Option<unsafe extern "system" fn(pvarg: *const core::ffi::c_void, pbdata: *const u8, cbdata: u32, ffinal: windows_sys::core::BOOL) -> windows_sys::core::BOOL>;
pub type PFN_CRYPT_ALLOC = Option<unsafe extern "system" fn(cbsize: usize) -> *mut core::ffi::c_void>;
pub type PFN_CRYPT_ASYNC_PARAM_FREE_FUNC = Option<unsafe extern "system" fn(pszparamoid: windows_sys::core::PCSTR, pvparam: *const core::ffi::c_void)>;
pub type PFN_CRYPT_ASYNC_RETRIEVAL_COMPLETION_FUNC = Option<unsafe extern "system" fn(pvcompletion: *mut core::ffi::c_void, dwcompletioncode: u32, pszurl: windows_sys::core::PCSTR, pszobjectoid: windows_sys::core::PCSTR, pvobject: *const core::ffi::c_void)>;
pub type PFN_CRYPT_CANCEL_RETRIEVAL = Option<unsafe extern "system" fn(dwflags: u32, pvarg: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PFN_CRYPT_ENUM_KEYID_PROP = Option<unsafe extern "system" fn(pkeyidentifier: *const CRYPT_HASH_BLOB, dwflags: u32, pvreserved: *const core::ffi::c_void, pvarg: *mut core::ffi::c_void, cprop: u32, rgdwpropid: *const u32, rgpvdata: *const *const core::ffi::c_void, rgcbdata: *const u32) -> windows_sys::core::BOOL>;
pub type PFN_CRYPT_ENUM_OID_FUNC = Option<unsafe extern "system" fn(dwencodingtype: u32, pszfuncname: windows_sys::core::PCSTR, pszoid: windows_sys::core::PCSTR, cvalue: u32, rgdwvaluetype: *const u32, rgpwszvaluename: *const windows_sys::core::PCWSTR, rgpbvaluedata: *const *const u8, rgcbvaluedata: *const u32, pvarg: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PFN_CRYPT_ENUM_OID_INFO = Option<unsafe extern "system" fn(pinfo: *const CRYPT_OID_INFO, pvarg: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "ncrypt")]
pub type PFN_CRYPT_EXPORT_PUBLIC_KEY_INFO_EX2_FUNC = Option<unsafe extern "system" fn(hncryptkey: super::NCRYPT_KEY_HANDLE, dwcertencodingtype: u32, pszpublickeyobjid: windows_sys::core::PCSTR, dwflags: u32, pvauxinfo: *const core::ffi::c_void, pinfo: *mut CERT_PUBLIC_KEY_INFO, pcbinfo: *mut u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "bcrypt")]
pub type PFN_CRYPT_EXPORT_PUBLIC_KEY_INFO_FROM_BCRYPT_HANDLE_FUNC = Option<unsafe extern "system" fn(hbcryptkey: super::BCRYPT_KEY_HANDLE, dwcertencodingtype: u32, pszpublickeyobjid: windows_sys::core::PCSTR, dwflags: u32, pvauxinfo: *const core::ffi::c_void, pinfo: *mut CERT_PUBLIC_KEY_INFO, pcbinfo: *mut u32) -> windows_sys::core::BOOL>;
pub type PFN_CRYPT_EXTRACT_ENCODED_SIGNATURE_PARAMETERS_FUNC = Option<unsafe extern "system" fn(dwcertencodingtype: u32, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, ppvdecodedsignpara: *mut *mut core::ffi::c_void, ppwszcnghashalgid: *mut windows_sys::core::PWSTR) -> windows_sys::core::BOOL>;
pub type PFN_CRYPT_FREE = Option<unsafe extern "system" fn(pv: *const core::ffi::c_void)>;
#[cfg(feature = "minwindef")]
pub type PFN_CRYPT_GET_SIGNER_CERTIFICATE = Option<unsafe extern "system" fn(pvgetarg: *mut core::ffi::c_void, dwcertencodingtype: u32, psignerid: *const CERT_INFO, hmsgcertstore: HCERTSTORE) -> PCCERT_CONTEXT>;
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FLUSH = Option<unsafe extern "system" fn(pcontext: *const core::ffi::c_void, rgidentifierornamelist: *const PCERT_NAME_BLOB, dwidentifierornamelistcount: u32) -> windows_sys::core::BOOL>;
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE = Option<unsafe extern "system" fn(pplugincontext: *const core::ffi::c_void, pbdata: *const u8)>;
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_IDENTIFIER = Option<unsafe extern "system" fn(pplugincontext: *const core::ffi::c_void, pidentifier: *const CRYPT_INTEGER_BLOB)>;
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_PASSWORD = Option<unsafe extern "system" fn(pplugincontext: *const core::ffi::c_void, pwszpassword: windows_sys::core::PCWSTR)>;
#[cfg(feature = "minwindef")]
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_GET = Option<unsafe extern "system" fn(pplugincontext: *const core::ffi::c_void, pidentifier: *const CRYPT_INTEGER_BLOB, dwnametype: u32, pnameblob: *const CRYPT_INTEGER_BLOB, ppbcontent: *mut super::PBYTE, pcbcontent: *mut u32, ppwszpassword: *mut windows_sys::core::PCWSTR, ppidentifier: *mut PCRYPT_DATA_BLOB) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_INITIALIZE = Option<unsafe extern "system" fn(pfnflush: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FLUSH, pcontext: *const core::ffi::c_void, pdwexpectedobjectcount: *mut u32, ppfunctable: *mut PCRYPT_OBJECT_LOCATOR_PROVIDER_TABLE, ppplugincontext: *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_RELEASE = Option<unsafe extern "system" fn(dwreason: u32, pplugincontext: *const core::ffi::c_void)>;
#[cfg(feature = "ncrypt")]
pub type PFN_CRYPT_SIGN_AND_ENCODE_HASH_FUNC = Option<unsafe extern "system" fn(hkey: super::NCRYPT_KEY_HANDLE, dwcertencodingtype: u32, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pvdecodedsignpara: *const core::ffi::c_void, pwszcngpubkeyalgid: windows_sys::core::PCWSTR, pwszcnghashalgid: windows_sys::core::PCWSTR, pbcomputedhash: *const u8, cbcomputedhash: u32, pbsignature: *mut u8, pcbsignature: *mut u32) -> windows_sys::core::BOOL>;
pub type PFN_CRYPT_VERIFY_ENCODED_SIGNATURE_FUNC = Option<unsafe extern "system" fn(dwcertencodingtype: u32, ppubkeyinfo: *const CERT_PUBLIC_KEY_INFO, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pvdecodedsignpara: *const core::ffi::c_void, pwszcngpubkeyalgid: windows_sys::core::PCWSTR, pwszcnghashalgid: windows_sys::core::PCWSTR, pbcomputedhash: *const u8, cbcomputedhash: u32, pbsignature: *const u8, cbsignature: u32) -> windows_sys::core::BOOL>;
pub type PFN_EXPORT_PRIV_KEY_FUNC = Option<unsafe extern "system" fn(hcryptprov: HCRYPTPROV, dwkeyspec: u32, pszprivatekeyobjid: windows_sys::core::PCSTR, dwflags: u32, pvauxinfo: *const core::ffi::c_void, pprivatekeyinfo: *mut CRYPT_PRIVATE_KEY_INFO, pcbprivatekeyinfo: *mut u32) -> windows_sys::core::BOOL>;
pub type PFN_FREE_ENCODED_OBJECT_FUNC = Option<unsafe extern "system" fn(pszobjectoid: windows_sys::core::PCSTR, pobject: *mut CRYPT_BLOB_ARRAY, pvfreecontext: *mut core::ffi::c_void)>;
pub type PFN_IMPORT_PRIV_KEY_FUNC = Option<unsafe extern "system" fn(hcryptprov: HCRYPTPROV, pprivatekeyinfo: *const CRYPT_PRIVATE_KEY_INFO, dwflags: u32, pvauxinfo: *const core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "bcrypt")]
pub type PFN_IMPORT_PUBLIC_KEY_INFO_EX2_FUNC = Option<unsafe extern "system" fn(dwcertencodingtype: u32, pinfo: *const CERT_PUBLIC_KEY_INFO, dwflags: u32, pvauxinfo: *const core::ffi::c_void, phkey: *mut super::BCRYPT_KEY_HANDLE) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PHCRYPTASYNC = *mut super::HANDLE;
pub type PHMAC_INFO = *mut HMAC_INFO;
pub type PHTTPSPolicyCallbackData = *mut HTTPSPolicyCallbackData;
pub const PKCS12_ALLOW_OVERWRITE_KEY: u32 = 16384;
pub const PKCS12_ALWAYS_CNG_KSP: u32 = 512;
pub const PKCS12_CONFIG_REGPATH: windows_sys::core::PCWSTR = windows_sys::core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\PFX");
pub const PKCS12_DISABLE_ENCRYPT_CERTIFICATES: u32 = 256;
pub const PKCS12_ENCRYPT_CERTIFICATES: u32 = 512;
pub const PKCS12_ENCRYPT_CERTIFICATES_VALUE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("EncryptCertificates");
pub const PKCS12_EXPORT_ECC_CURVE_OID: u32 = 8192;
pub const PKCS12_EXPORT_ECC_CURVE_PARAMETERS: u32 = 4096;
pub const PKCS12_EXPORT_PBES2_PARAMS: u32 = 128;
pub const PKCS12_EXPORT_RESERVED_MASK: u32 = 4294901760;
pub const PKCS12_EXPORT_SILENT: u32 = 64;
pub const PKCS12_IMPORT_RESERVED_MASK: u32 = 4294901760;
pub const PKCS12_IMPORT_SILENT: u32 = 64;
pub const PKCS12_INCLUDE_EXTENDED_PROPERTIES: u32 = 16;
pub const PKCS12_NAMED_NO_PERSIST_KEY: u32 = 131072;
pub const PKCS12_NO_PERSIST_KEY: u32 = 32768;
pub const PKCS12_OBJECT_LOCATOR_ALL_IMPORT_FLAGS: u32 = 33360;
pub const PKCS12_ONLY_CERTIFICATES: u32 = 1024;
pub const PKCS12_ONLY_CERTIFICATES_CONTAINER_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("PfxContainer");
pub const PKCS12_ONLY_CERTIFICATES_PROVIDER_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("PfxProvider");
pub const PKCS12_ONLY_CERTIFICATES_PROVIDER_TYPE: u32 = 0;
pub const PKCS12_ONLY_NOT_ENCRYPTED_CERTIFICATES: u32 = 2048;
pub const PKCS12_PBES2_ALG_AES256_SHA256: windows_sys::core::PCWSTR = windows_sys::core::w!("AES256-SHA256");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PKCS12_PBES2_EXPORT_PARAMS {
    pub dwSize: u32,
    pub hNcryptDescriptor: *mut core::ffi::c_void,
    pub pwszPbes2Alg: windows_sys::core::PWSTR,
}
impl Default for PKCS12_PBES2_EXPORT_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PKCS12_PBKDF2_ID_HMAC_SHA1: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.2.7");
pub const PKCS12_PBKDF2_ID_HMAC_SHA256: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.2.9");
pub const PKCS12_PBKDF2_ID_HMAC_SHA384: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.2.10");
pub const PKCS12_PBKDF2_ID_HMAC_SHA512: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.2.11");
pub const PKCS12_PREFER_CNG_KSP: u32 = 256;
pub const PKCS12_PROTECT_TO_DOMAIN_SIDS: u32 = 32;
pub const PKCS12_VIRTUAL_ISOLATION_KEY: u32 = 65536;
pub const PKCS5_PADDING: u32 = 1;
pub const PKCS7_SIGNER_INFO: windows_sys::core::PCSTR = 500 as _;
pub const PKCS_7_ASN_ENCODING: u32 = 65536;
pub const PKCS_7_NDR_ENCODING: u32 = 131072;
pub const PKCS_ATTRIBUTE: windows_sys::core::PCSTR = 22 as _;
pub const PKCS_ATTRIBUTES: windows_sys::core::PCSTR = 48 as _;
pub const PKCS_CONTENT_INFO: windows_sys::core::PCSTR = 33 as _;
pub const PKCS_CONTENT_INFO_SEQUENCE_OF_ANY: windows_sys::core::PCSTR = 23 as _;
pub const PKCS_CTL: windows_sys::core::PCSTR = 37 as _;
pub const PKCS_ENCRYPTED_PRIVATE_KEY_INFO: windows_sys::core::PCSTR = 45 as _;
pub const PKCS_PRIVATE_KEY_INFO: windows_sys::core::PCSTR = 44 as _;
pub const PKCS_RC2_CBC_PARAMETERS: windows_sys::core::PCSTR = 41 as _;
pub const PKCS_RSAES_OAEP_PARAMETERS: windows_sys::core::PCSTR = 76 as _;
pub const PKCS_RSA_PRIVATE_KEY: windows_sys::core::PCSTR = 43 as _;
pub const PKCS_RSA_SSA_PSS_PARAMETERS: windows_sys::core::PCSTR = 75 as _;
pub const PKCS_RSA_SSA_PSS_TRAILER_FIELD_BC: u32 = 1;
pub const PKCS_SMIME_CAPABILITIES: windows_sys::core::PCSTR = 42 as _;
pub const PKCS_SORTED_CTL: windows_sys::core::PCSTR = 49 as _;
pub const PKCS_TIME_REQUEST: windows_sys::core::PCSTR = 18 as _;
pub const PKCS_UTC_TIME: windows_sys::core::PCSTR = 17 as _;
pub type PKEY_TYPE_SUBTYPE = *mut KEY_TYPE_SUBTYPE;
pub const PLAINTEXTKEYBLOB: u32 = 8;
#[cfg(feature = "minwindef")]
pub type POCSP_BASIC_RESPONSE_ENTRY = *mut OCSP_BASIC_RESPONSE_ENTRY;
#[cfg(feature = "minwindef")]
pub type POCSP_BASIC_RESPONSE_INFO = *mut OCSP_BASIC_RESPONSE_INFO;
#[cfg(feature = "minwindef")]
pub type POCSP_BASIC_REVOKED_INFO = *mut OCSP_BASIC_REVOKED_INFO;
pub type POCSP_BASIC_SIGNED_RESPONSE_INFO = *mut OCSP_BASIC_SIGNED_RESPONSE_INFO;
pub type POCSP_CERT_ID = *mut OCSP_CERT_ID;
pub type POCSP_REQUEST_ENTRY = *mut OCSP_REQUEST_ENTRY;
pub type POCSP_REQUEST_INFO = *mut OCSP_REQUEST_INFO;
pub type POCSP_RESPONSE_INFO = *mut OCSP_RESPONSE_INFO;
pub type POCSP_SIGNATURE_INFO = *mut OCSP_SIGNATURE_INFO;
pub type POCSP_SIGNED_REQUEST_INFO = *mut OCSP_SIGNED_REQUEST_INFO;
pub type PPKCS12_PBES2_EXPORT_PARAMS = *mut PKCS12_PBES2_EXPORT_PARAMS;
pub const PP_ADMIN_PIN: u32 = 31;
pub const PP_APPLI_CERT: u32 = 18;
pub const PP_CERTCHAIN: u32 = 9;
pub const PP_CHANGE_PASSWORD: u32 = 7;
pub const PP_CLIENT_HWND: u32 = 1;
pub const PP_CONTAINER: u32 = 6;
pub const PP_CONTEXT_INFO: u32 = 11;
pub const PP_CRYPT_COUNT_KEY_USE: u32 = 41;
pub const PP_DELETEKEY: u32 = 24;
pub const PP_DISMISS_PIN_UI_SEC: u32 = 49;
pub const PP_ENUMALGS: u32 = 1;
pub const PP_ENUMALGS_EX: u32 = 22;
pub const PP_ENUMCONTAINERS: u32 = 2;
pub const PP_ENUMELECTROOTS: u32 = 26;
pub const PP_ENUMEX_SIGNING_PROT: u32 = 40;
pub const PP_ENUMMANDROOTS: u32 = 25;
pub const PP_IMPTYPE: u32 = 3;
pub const PP_IS_PFX_EPHEMERAL: u32 = 50;
pub const PP_KEYEXCHANGE_ALG: u32 = 14;
pub const PP_KEYEXCHANGE_KEYSIZE: u32 = 12;
pub const PP_KEYEXCHANGE_PIN: u32 = 32;
pub const PP_KEYSET_SEC_DESCR: u32 = 8;
pub const PP_KEYSET_TYPE: u32 = 27;
pub const PP_KEYSPEC: u32 = 39;
pub const PP_KEYSTORAGE: u32 = 17;
pub const PP_KEYX_KEYSIZE_INC: u32 = 35;
pub const PP_KEY_TYPE_SUBTYPE: u32 = 10;
pub const PP_NAME: u32 = 4;
pub const PP_PIN_PROMPT_STRING: u32 = 44;
pub const PP_PROVTYPE: u32 = 16;
pub const PP_ROOT_CERTSTORE: u32 = 46;
pub const PP_SECURE_KEYEXCHANGE_PIN: u32 = 47;
pub const PP_SECURE_SIGNATURE_PIN: u32 = 48;
pub const PP_SESSION_KEYSIZE: u32 = 20;
pub const PP_SGC_INFO: u32 = 37;
pub const PP_SIGNATURE_ALG: u32 = 15;
pub const PP_SIGNATURE_KEYSIZE: u32 = 13;
pub const PP_SIGNATURE_PIN: u32 = 33;
pub const PP_SIG_KEYSIZE_INC: u32 = 34;
pub const PP_SMARTCARD_GUID: u32 = 45;
pub const PP_SMARTCARD_READER: u32 = 43;
pub const PP_SMARTCARD_READER_ICON: u32 = 47;
pub const PP_SYM_KEYSIZE: u32 = 19;
pub const PP_UI_PROMPT: u32 = 21;
pub const PP_UNIQUE_CONTAINER: u32 = 36;
pub const PP_USER_CERTSTORE: u32 = 42;
pub const PP_USE_HARDWARE_RNG: u32 = 38;
pub const PP_VERSION: u32 = 5;
pub const PRIVATEKEYBLOB: u32 = 7;
pub type PROOT_INFO_LUID = *mut ROOT_INFO_LUID;
pub const PROV_DH_SCHANNEL: u32 = 18;
pub const PROV_DSS: u32 = 3;
pub const PROV_DSS_DH: u32 = 13;
pub const PROV_EC_ECDSA_FULL: u32 = 16;
pub const PROV_EC_ECDSA_SIG: u32 = 14;
pub const PROV_EC_ECNRA_FULL: u32 = 17;
pub const PROV_EC_ECNRA_SIG: u32 = 15;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROV_ENUMALGS {
    pub aiAlgid: ALG_ID,
    pub dwBitLen: u32,
    pub dwNameLen: u32,
    pub szName: [i8; 20],
}
impl Default for PROV_ENUMALGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROV_ENUMALGS_EX {
    pub aiAlgid: ALG_ID,
    pub dwDefaultLen: u32,
    pub dwMinLen: u32,
    pub dwMaxLen: u32,
    pub dwProtocols: u32,
    pub dwNameLen: u32,
    pub szName: [i8; 20],
    pub dwLongNameLen: u32,
    pub szLongName: [i8; 40],
}
impl Default for PROV_ENUMALGS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROV_FORTEZZA: u32 = 4;
pub const PROV_INTEL_SEC: u32 = 22;
pub const PROV_MS_EXCHANGE: u32 = 5;
pub const PROV_REPLACE_OWF: u32 = 23;
pub const PROV_RNG: u32 = 21;
pub const PROV_RSA_AES: u32 = 24;
pub const PROV_RSA_FULL: u32 = 1;
pub const PROV_RSA_SCHANNEL: u32 = 12;
pub const PROV_RSA_SIG: u32 = 2;
pub const PROV_SPYRUS_LYNKS: u32 = 20;
pub const PROV_SSL: u32 = 6;
pub type PSCHANNEL_ALG = *mut SCHANNEL_ALG;
pub type PSSL_EXTRA_CERT_CHAIN_POLICY_PARA = *mut HTTPSPolicyCallbackData;
pub type PSSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS = *mut SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS;
pub type PSSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA = *mut SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA;
pub type PSSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA = *mut SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA;
pub type PSSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS = *mut SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS;
pub const PUBLICKEYBLOB: u32 = 6;
pub const PUBLICKEYBLOBEX: u32 = 10;
pub type PUBLICKEYSTRUC = BLOBHEADER;
pub const RANDOM_PADDING: u32 = 2;
pub const REPORT_NOT_ABLE_TO_EXPORT_PRIVATE_KEY: u32 = 2;
pub const REPORT_NO_PRIVATE_KEY: u32 = 1;
pub const REVOCATION_OID_CRL_REVOCATION: windows_sys::core::PCSTR = 1 as _;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ROOT_INFO_LUID {
    pub LowPart: u32,
    pub HighPart: i32,
}
pub const RSA1024BIT_KEY: u32 = 67108864;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RSAPUBKEY {
    pub magic: u32,
    pub bitlen: u32,
    pub pubexp: u32,
}
pub const RSA_CSP_PUBLICKEYBLOB: windows_sys::core::PCSTR = 19 as _;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SCHANNEL_ALG {
    pub dwUse: u32,
    pub Algid: ALG_ID,
    pub cBits: u32,
    pub dwFlags: u32,
    pub dwReserved: u32,
}
pub const SCHANNEL_ENC_KEY: u32 = 1;
pub const SCHANNEL_MAC_KEY: u32 = 0;
pub const SCHEME_OID_RETRIEVE_ENCODED_OBJECTW_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("SchemeDllRetrieveEncodedObjectW");
pub const SCHEME_OID_RETRIEVE_ENCODED_OBJECT_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("SchemeDllRetrieveEncodedObject");
pub const SIMPLEBLOB: u32 = 1;
pub const SITE_PIN_RULES_ALL_SUBDOMAINS_FLAG: u32 = 1;
pub const SORTED_CTL_EXT_COUNT_OFFSET: u32 = 4;
pub const SORTED_CTL_EXT_FLAGS_OFFSET: u32 = 0;
pub const SORTED_CTL_EXT_HASHED_SUBJECT_IDENTIFIER_FLAG: u32 = 1;
pub const SORTED_CTL_EXT_HASH_BUCKET_OFFSET: u32 = 12;
pub const SORTED_CTL_EXT_MAX_COLLISION_OFFSET: u32 = 8;
pub type SSL_EXTRA_CERT_CHAIN_POLICY_PARA = HTTPSPolicyCallbackData;
pub const SSL_F12_ERROR_TEXT_LENGTH: u32 = 256;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS {
    pub cbSize: u32,
    pub dwErrorLevel: u32,
    pub dwErrorCategory: u32,
    pub dwReserved: u32,
    pub wszErrorText: [u16; 256],
}
impl Default for SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SSL_HPKP_HEADER_COUNT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwReserved: u32,
    pub pwszServerName: windows_sys::core::PWSTR,
    pub rgpszHpkpValue: [windows_sys::core::PSTR; 2],
}
impl Default for SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SSL_HPKP_PKP_HEADER_INDEX: u32 = 0;
pub const SSL_HPKP_PKP_RO_HEADER_INDEX: u32 = 1;
pub const SSL_KEY_PIN_ERROR_TEXT_LENGTH: u32 = 512;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwReserved: u32,
    pub pwszServerName: windows_sys::core::PCWSTR,
}
impl Default for SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS {
    pub cbSize: u32,
    pub lError: i32,
    pub wszErrorText: [u16; 512],
}
impl Default for SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SSL_OBJECT_LOCATOR_CERT_VALIDATION_CONFIG_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("SslObjectLocatorInitializeCertValidationConfig");
pub const SSL_OBJECT_LOCATOR_ISSUER_LIST_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("SslObjectLocatorInitializeIssuerList");
pub const SSL_OBJECT_LOCATOR_PFX_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("SslObjectLocatorInitializePfx");
pub const SYMMETRICWRAPKEYBLOB: u32 = 11;
pub type TEKPUBKEY = DHPUBKEY;
pub const TIMESTAMP_DONT_HASH_DATA: u32 = 1;
pub const TIMESTAMP_FAILURE_BAD_ALG: u32 = 0;
pub const TIMESTAMP_FAILURE_BAD_FORMAT: u32 = 5;
pub const TIMESTAMP_FAILURE_BAD_REQUEST: u32 = 2;
pub const TIMESTAMP_FAILURE_EXTENSION_NOT_SUPPORTED: u32 = 16;
pub const TIMESTAMP_FAILURE_INFO_NOT_AVAILABLE: u32 = 17;
pub const TIMESTAMP_FAILURE_POLICY_NOT_SUPPORTED: u32 = 15;
pub const TIMESTAMP_FAILURE_SYSTEM_FAILURE: u32 = 25;
pub const TIMESTAMP_FAILURE_TIME_NOT_AVAILABLE: u32 = 14;
pub const TIMESTAMP_INFO: windows_sys::core::PCSTR = 80 as _;
pub const TIMESTAMP_NO_AUTH_RETRIEVAL: u32 = 131072;
pub const TIMESTAMP_REQUEST: windows_sys::core::PCSTR = 78 as _;
pub const TIMESTAMP_RESPONSE: windows_sys::core::PCSTR = 79 as _;
pub const TIMESTAMP_STATUS_GRANTED: u32 = 0;
pub const TIMESTAMP_STATUS_GRANTED_WITH_MODS: u32 = 1;
pub const TIMESTAMP_STATUS_REJECTED: u32 = 2;
pub const TIMESTAMP_STATUS_REVOCATION_WARNING: u32 = 4;
pub const TIMESTAMP_STATUS_REVOKED: u32 = 5;
pub const TIMESTAMP_STATUS_WAITING: u32 = 3;
pub const TIMESTAMP_VERIFY_CONTEXT_SIGNATURE: u32 = 32;
pub const TIMESTAMP_VERSION: u32 = 1;
pub const TIME_VALID_OID_FLUSH_CRL: windows_sys::core::PCSTR = 2 as _;
pub const TIME_VALID_OID_FLUSH_CRL_FROM_CERT: windows_sys::core::PCSTR = 3 as _;
pub const TIME_VALID_OID_FLUSH_CTL: windows_sys::core::PCSTR = 1 as _;
pub const TIME_VALID_OID_FLUSH_FRESHEST_CRL_FROM_CERT: windows_sys::core::PCSTR = 4 as _;
pub const TIME_VALID_OID_FLUSH_FRESHEST_CRL_FROM_CRL: windows_sys::core::PCSTR = 5 as _;
pub const TIME_VALID_OID_FLUSH_OBJECT_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("TimeValidDllFlushObject");
pub const TIME_VALID_OID_GET_CRL: windows_sys::core::PCSTR = 2 as _;
pub const TIME_VALID_OID_GET_CRL_FROM_CERT: windows_sys::core::PCSTR = 3 as _;
pub const TIME_VALID_OID_GET_CTL: windows_sys::core::PCSTR = 1 as _;
pub const TIME_VALID_OID_GET_FRESHEST_CRL_FROM_CERT: windows_sys::core::PCSTR = 4 as _;
pub const TIME_VALID_OID_GET_FRESHEST_CRL_FROM_CRL: windows_sys::core::PCSTR = 5 as _;
pub const TIME_VALID_OID_GET_OBJECT_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("TimeValidDllGetObject");
pub const URL_OID_CERTIFICATE_CRL_DIST_POINT: windows_sys::core::PCSTR = 2 as _;
pub const URL_OID_CERTIFICATE_CRL_DIST_POINT_AND_OCSP: windows_sys::core::PCSTR = 11 as _;
pub const URL_OID_CERTIFICATE_FRESHEST_CRL: windows_sys::core::PCSTR = 6 as _;
pub const URL_OID_CERTIFICATE_ISSUER: windows_sys::core::PCSTR = 1 as _;
pub const URL_OID_CERTIFICATE_OCSP: windows_sys::core::PCSTR = 9 as _;
pub const URL_OID_CERTIFICATE_OCSP_AND_CRL_DIST_POINT: windows_sys::core::PCSTR = 10 as _;
pub const URL_OID_CERTIFICATE_ONLY_OCSP: windows_sys::core::PCSTR = 13 as _;
pub const URL_OID_CRL_FRESHEST_CRL: windows_sys::core::PCSTR = 7 as _;
pub const URL_OID_CRL_ISSUER: windows_sys::core::PCSTR = 5 as _;
pub const URL_OID_CROSS_CERT_DIST_POINT: windows_sys::core::PCSTR = 8 as _;
pub const URL_OID_CROSS_CERT_SUBJECT_INFO_ACCESS: windows_sys::core::PCSTR = 12 as _;
pub const URL_OID_CTL_ISSUER: windows_sys::core::PCSTR = 3 as _;
pub const URL_OID_CTL_NEXT_UPDATE: windows_sys::core::PCSTR = 4 as _;
pub const URL_OID_GET_OBJECT_URL_FUNC: windows_sys::core::PCSTR = windows_sys::core::s!("UrlDllGetObjectUrl");
pub const USAGE_MATCH_TYPE_AND: u32 = 0;
pub const USAGE_MATCH_TYPE_OR: u32 = 1;
pub const X509_ALGORITHM_IDENTIFIER: windows_sys::core::PCSTR = 74 as _;
pub const X509_ALTERNATE_NAME: windows_sys::core::PCSTR = 12 as _;
pub const X509_ASN_ENCODING: u32 = 1;
pub const X509_AUTHORITY_INFO_ACCESS: windows_sys::core::PCSTR = 32 as _;
pub const X509_AUTHORITY_KEY_ID: windows_sys::core::PCSTR = 9 as _;
pub const X509_AUTHORITY_KEY_ID2: windows_sys::core::PCSTR = 31 as _;
pub const X509_BASIC_CONSTRAINTS: windows_sys::core::PCSTR = 13 as _;
pub const X509_BASIC_CONSTRAINTS2: windows_sys::core::PCSTR = 15 as _;
pub const X509_BIOMETRIC_EXT: windows_sys::core::PCSTR = 71 as _;
pub const X509_BITS: windows_sys::core::PCSTR = 26 as _;
pub const X509_BITS_WITHOUT_TRAILING_ZEROES: windows_sys::core::PCSTR = 51 as _;
pub const X509_CERT: windows_sys::core::PCSTR = 1 as _;
pub const X509_CERTIFICATE_TEMPLATE: windows_sys::core::PCSTR = 64 as _;
pub const X509_CERT_BUNDLE: windows_sys::core::PCSTR = 81 as _;
pub const X509_CERT_CRL_TO_BE_SIGNED: windows_sys::core::PCSTR = 3 as _;
pub const X509_CERT_PAIR: windows_sys::core::PCSTR = 53 as _;
pub const X509_CERT_POLICIES: windows_sys::core::PCSTR = 16 as _;
pub const X509_CERT_REQUEST_TO_BE_SIGNED: windows_sys::core::PCSTR = 4 as _;
pub const X509_CERT_TO_BE_SIGNED: windows_sys::core::PCSTR = 2 as _;
pub const X509_CHOICE_OF_TIME: windows_sys::core::PCSTR = 30 as _;
pub const X509_CRL_DIST_POINTS: windows_sys::core::PCSTR = 35 as _;
pub const X509_CROSS_CERT_DIST_POINTS: windows_sys::core::PCSTR = 58 as _;
pub const X509_DH_PARAMETERS: windows_sys::core::PCSTR = 47 as _;
pub const X509_DSS_PARAMETERS: windows_sys::core::PCSTR = 39 as _;
pub const X509_DSS_SIGNATURE: windows_sys::core::PCSTR = 40 as _;
pub const X509_ECC_PARAMETERS: windows_sys::core::PCSTR = 85 as _;
pub const X509_ECC_PRIVATE_KEY: windows_sys::core::PCSTR = 82 as _;
pub const X509_ECC_SIGNATURE: windows_sys::core::PCSTR = 47 as _;
pub const X509_ENHANCED_KEY_USAGE: windows_sys::core::PCSTR = 36 as _;
pub const X509_ENUMERATED: windows_sys::core::PCSTR = 29 as _;
pub const X509_EXTENSIONS: windows_sys::core::PCSTR = 5 as _;
pub const X509_INTEGER: windows_sys::core::PCSTR = 27 as _;
pub const X509_ISSUING_DIST_POINT: windows_sys::core::PCSTR = 54 as _;
pub const X509_KEYGEN_REQUEST_TO_BE_SIGNED: windows_sys::core::PCSTR = 21 as _;
pub const X509_KEY_ATTRIBUTES: windows_sys::core::PCSTR = 10 as _;
pub const X509_KEY_USAGE: windows_sys::core::PCSTR = 14 as _;
pub const X509_KEY_USAGE_RESTRICTION: windows_sys::core::PCSTR = 11 as _;
pub const X509_LOGOTYPE_EXT: windows_sys::core::PCSTR = 70 as _;
pub const X509_MULTI_BYTE_INTEGER: windows_sys::core::PCSTR = 28 as _;
pub const X509_MULTI_BYTE_UINT: windows_sys::core::PCSTR = 38 as _;
pub const X509_NAME: windows_sys::core::PCSTR = 7 as _;
pub const X509_NAME_CONSTRAINTS: windows_sys::core::PCSTR = 55 as _;
pub const X509_NAME_VALUE: windows_sys::core::PCSTR = 6 as _;
pub const X509_NDR_ENCODING: u32 = 2;
pub const X509_OBJECT_IDENTIFIER: windows_sys::core::PCSTR = 73 as _;
pub const X509_OCTET_STRING: windows_sys::core::PCSTR = 25 as _;
pub const X509_PKIX_POLICY_QUALIFIER_USERNOTICE: windows_sys::core::PCSTR = 46 as _;
pub const X509_POLICY_CONSTRAINTS: windows_sys::core::PCSTR = 57 as _;
pub const X509_POLICY_MAPPINGS: windows_sys::core::PCSTR = 56 as _;
pub const X509_PUBLIC_KEY_INFO: windows_sys::core::PCSTR = 8 as _;
pub const X509_QC_STATEMENTS_EXT: windows_sys::core::PCSTR = 42 as _;
pub const X509_SEQUENCE_OF_ANY: windows_sys::core::PCSTR = 34 as _;
pub const X509_SUBJECT_DIR_ATTRS: windows_sys::core::PCSTR = 84 as _;
pub const X509_UNICODE_NAME: windows_sys::core::PCSTR = 20 as _;
pub const X509_UNICODE_NAME_VALUE: windows_sys::core::PCSTR = 24 as _;
pub const X942_DH_PARAMETERS: windows_sys::core::PCSTR = 50 as _;
pub const X942_OTHER_INFO: windows_sys::core::PCSTR = 52 as _;
pub const ZERO_PADDING: u32 = 3;
pub const cPRIV_KEY_CACHE_MAX_ITEMS_DEFAULT: u32 = 20;
pub const cPRIV_KEY_CACHE_PURGE_INTERVAL_SECONDS_DEFAULT: u32 = 86400;
pub const szKEY_CACHE_ENABLED: windows_sys::core::PCSTR = windows_sys::core::s!("CachePrivateKeys");
pub const szKEY_CACHE_SECONDS: windows_sys::core::PCSTR = windows_sys::core::s!("PrivateKeyLifetimeSeconds");
pub const szKEY_CRYPTOAPI_PRIVATE_KEY_OPTIONS: windows_sys::core::PCSTR = windows_sys::core::s!("Software\\Policies\\Microsoft\\Cryptography");
pub const szOIDVerisign_FailInfo: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113733.1.9.4");
pub const szOIDVerisign_MessageType: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113733.1.9.2");
pub const szOIDVerisign_PkiStatus: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113733.1.9.3");
pub const szOIDVerisign_RecipientNonce: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113733.1.9.6");
pub const szOIDVerisign_SenderNonce: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113733.1.9.5");
pub const szOIDVerisign_TransactionID: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113733.1.9.7");
pub const szOID_ANSI_X942: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10046");
pub const szOID_ANSI_X942_DH: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10046.2.1");
pub const szOID_ANY_APPLICATION_POLICY: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.12.1");
pub const szOID_ANY_CERT_POLICY: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.32.0");
pub const szOID_ANY_ENHANCED_KEY_USAGE: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.37.0");
pub const szOID_APPLICATION_CERT_POLICIES: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.10");
pub const szOID_APPLICATION_POLICY_CONSTRAINTS: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.12");
pub const szOID_APPLICATION_POLICY_MAPPINGS: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.11");
pub const szOID_ARCHIVED_KEY_ATTR: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.13");
pub const szOID_ARCHIVED_KEY_CERT_HASH: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.16");
pub const szOID_ATTEST_WHQL_CRYPTO: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.5.1");
pub const szOID_ATTR_PLATFORM_SPECIFICATION: windows_sys::core::PCSTR = windows_sys::core::s!("2.23.133.2.17");
pub const szOID_ATTR_SUPPORTED_ALGORITHMS: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.52");
pub const szOID_ATTR_TPM_SECURITY_ASSERTIONS: windows_sys::core::PCSTR = windows_sys::core::s!("2.23.133.2.18");
pub const szOID_ATTR_TPM_SPECIFICATION: windows_sys::core::PCSTR = windows_sys::core::s!("2.23.133.2.16");
pub const szOID_AUTHORITY_INFO_ACCESS: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.1.1");
pub const szOID_AUTHORITY_KEY_IDENTIFIER: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.1");
pub const szOID_AUTHORITY_KEY_IDENTIFIER2: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.35");
pub const szOID_AUTHORITY_REVOCATION_LIST: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.38");
pub const szOID_AUTO_ENROLL_CTL_USAGE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.20.1");
pub const szOID_BACKGROUND_OTHER_LOGOTYPE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.20.2");
pub const szOID_BASIC_CONSTRAINTS: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.10");
pub const szOID_BASIC_CONSTRAINTS2: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.19");
pub const szOID_BIOMETRIC_EXT: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.1.2");
pub const szOID_BIOMETRIC_SIGNING: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.41");
pub const szOID_BUSINESS_CATEGORY: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.15");
pub const szOID_CA_CERTIFICATE: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.37");
pub const szOID_CERTIFICATE_REVOCATION_LIST: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.39");
pub const szOID_CERTIFICATE_TEMPLATE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.7");
pub const szOID_CERTSRV_CA_VERSION: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.1");
pub const szOID_CERTSRV_CROSSCA_VERSION: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.22");
pub const szOID_CERTSRV_PREVIOUS_CERT_HASH: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.2");
pub const szOID_CERT_DISALLOWED_CA_FILETIME_PROP_ID: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.11.128");
pub const szOID_CERT_DISALLOWED_FILETIME_PROP_ID: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.11.104");
pub const szOID_CERT_EXTENSIONS: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.2.1.14");
pub const szOID_CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.11.28");
pub const szOID_CERT_KEY_IDENTIFIER_PROP_ID: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.11.20");
pub const szOID_CERT_LOG_LIST_EXT: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.52");
pub const szOID_CERT_MANIFOLD: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.20.3");
pub const szOID_CERT_MD5_HASH_PROP_ID: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.11.4");
pub const szOID_CERT_POLICIES: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.32");
pub const szOID_CERT_POLICIES_95: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.3");
pub const szOID_CERT_POLICIES_95_QUALIFIER1: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113733.1.7.1.1");
pub const szOID_CERT_PROP_ID_PREFIX: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.11.");
pub const szOID_CERT_SHA256_HASH_PROP_ID: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.11.107");
pub const szOID_CERT_SIGNATURE_HASH_PROP_ID: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.11.15");
pub const szOID_CERT_STRONG_KEY_OS_1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.72.2.1");
pub const szOID_CERT_STRONG_KEY_OS_PREFIX: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.72.2.");
pub const szOID_CERT_STRONG_SIGN_OS_1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.72.1.1");
pub const szOID_CERT_STRONG_SIGN_OS_PREFIX: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.72.1.");
pub const szOID_CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.11.29");
pub const szOID_CMC: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7");
pub const szOID_CMC_ADD_ATTRIBUTES: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.10.1");
pub const szOID_CMC_ADD_EXTENSIONS: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.8");
pub const szOID_CMC_DATA_RETURN: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.4");
pub const szOID_CMC_DECRYPTED_POP: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.10");
pub const szOID_CMC_ENCRYPTED_POP: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.9");
pub const szOID_CMC_GET_CERT: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.15");
pub const szOID_CMC_GET_CRL: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.16");
pub const szOID_CMC_IDENTIFICATION: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.2");
pub const szOID_CMC_IDENTITY_PROOF: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.3");
pub const szOID_CMC_ID_CONFIRM_CERT_ACCEPTANCE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.24");
pub const szOID_CMC_ID_POP_LINK_RANDOM: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.22");
pub const szOID_CMC_ID_POP_LINK_WITNESS: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.23");
pub const szOID_CMC_LRA_POP_WITNESS: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.11");
pub const szOID_CMC_QUERY_PENDING: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.21");
pub const szOID_CMC_RECIPIENT_NONCE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.7");
pub const szOID_CMC_REG_INFO: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.18");
pub const szOID_CMC_RESPONSE_INFO: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.19");
pub const szOID_CMC_REVOKE_REQUEST: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.17");
pub const szOID_CMC_SENDER_NONCE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.6");
pub const szOID_CMC_STATUS_INFO: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.1");
pub const szOID_CMC_TRANSACTION_ID: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.7.5");
pub const szOID_CN_ECDSA_SHA256: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.156.11235.1.1.1");
pub const szOID_COMMON_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.3");
pub const szOID_COUNTRY_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.6");
pub const szOID_CRL_DIST_POINTS: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.31");
pub const szOID_CRL_NEXT_PUBLISH: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.4");
pub const szOID_CRL_NUMBER: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.20");
pub const szOID_CRL_REASON_CODE: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.21");
pub const szOID_CRL_SELF_CDP: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.14");
pub const szOID_CRL_VIRTUAL_BASE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.3");
pub const szOID_CROSS_CERTIFICATE_PAIR: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.40");
pub const szOID_CROSS_CERT_DIST_POINTS: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.9.1");
pub const szOID_CTL: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.1");
pub const szOID_CT_CERT_SCTLIST: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.11129.2.4.2");
pub const szOID_CT_PKI_DATA: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.12.2");
pub const szOID_CT_PKI_RESPONSE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.12.3");
pub const szOID_DELTA_CRL_INDICATOR: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.27");
pub const szOID_DESCRIPTION: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.13");
pub const szOID_DESTINATION_INDICATOR: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.27");
pub const szOID_DEVICE_SERIAL_NUMBER: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.5");
pub const szOID_DH_SINGLE_PASS_STDDH_SHA1_KDF: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.133.16.840.63.0.2");
pub const szOID_DH_SINGLE_PASS_STDDH_SHA256_KDF: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.132.1.11.1");
pub const szOID_DH_SINGLE_PASS_STDDH_SHA384_KDF: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.132.1.11.2");
pub const szOID_DISALLOWED_LIST: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.30");
pub const szOID_DN_QUALIFIER: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.46");
pub const szOID_DOMAIN_COMPONENT: windows_sys::core::PCSTR = windows_sys::core::s!("0.9.2342.19200300.100.1.25");
pub const szOID_DRM: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.5.1");
pub const szOID_DRM_INDIVIDUALIZATION: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.5.2");
pub const szOID_DS: windows_sys::core::PCSTR = windows_sys::core::s!("2.5");
pub const szOID_DSALG: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.8");
pub const szOID_DSALG_CRPT: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.8.1");
pub const szOID_DSALG_HASH: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.8.2");
pub const szOID_DSALG_RSA: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.8.1.1");
pub const szOID_DSALG_SIGN: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.8.3");
pub const szOID_DS_EMAIL_REPLICATION: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.19");
pub const szOID_DYNAMIC_CODE_GEN_SIGNER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.76.5.1");
pub const szOID_ECC_CURVE_BRAINPOOLP160R1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.36.3.3.2.8.1.1.1");
pub const szOID_ECC_CURVE_BRAINPOOLP160T1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.36.3.3.2.8.1.1.2");
pub const szOID_ECC_CURVE_BRAINPOOLP192R1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.36.3.3.2.8.1.1.3");
pub const szOID_ECC_CURVE_BRAINPOOLP192T1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.36.3.3.2.8.1.1.4");
pub const szOID_ECC_CURVE_BRAINPOOLP224R1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.36.3.3.2.8.1.1.5");
pub const szOID_ECC_CURVE_BRAINPOOLP224T1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.36.3.3.2.8.1.1.6");
pub const szOID_ECC_CURVE_BRAINPOOLP256R1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.36.3.3.2.8.1.1.7");
pub const szOID_ECC_CURVE_BRAINPOOLP256T1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.36.3.3.2.8.1.1.8");
pub const szOID_ECC_CURVE_BRAINPOOLP320R1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.36.3.3.2.8.1.1.9");
pub const szOID_ECC_CURVE_BRAINPOOLP320T1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.36.3.3.2.8.1.1.10");
pub const szOID_ECC_CURVE_BRAINPOOLP384R1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.36.3.3.2.8.1.1.11");
pub const szOID_ECC_CURVE_BRAINPOOLP384T1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.36.3.3.2.8.1.1.12");
pub const szOID_ECC_CURVE_BRAINPOOLP512R1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.36.3.3.2.8.1.1.13");
pub const szOID_ECC_CURVE_BRAINPOOLP512T1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.36.3.3.2.8.1.1.14");
pub const szOID_ECC_CURVE_EC192WAPI: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.156.11235.1.1.2.1");
pub const szOID_ECC_CURVE_NISTP192: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10045.3.1.1");
pub const szOID_ECC_CURVE_NISTP224: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.132.0.33");
pub const szOID_ECC_CURVE_P256: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10045.3.1.7");
pub const szOID_ECC_CURVE_P384: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.132.0.34");
pub const szOID_ECC_CURVE_P521: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.132.0.35");
pub const szOID_ECC_CURVE_SECP160K1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.132.0.9");
pub const szOID_ECC_CURVE_SECP160R1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.132.0.8");
pub const szOID_ECC_CURVE_SECP160R2: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.132.0.30");
pub const szOID_ECC_CURVE_SECP192K1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.132.0.31");
pub const szOID_ECC_CURVE_SECP224K1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.132.0.32");
pub const szOID_ECC_CURVE_SECP256K1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.132.0.10");
pub const szOID_ECC_CURVE_WTLS9: windows_sys::core::PCSTR = windows_sys::core::s!("2.23.43.1.4.9");
pub const szOID_ECC_CURVE_X962P192V1: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10045.3.1.1");
pub const szOID_ECC_CURVE_X962P192V2: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10045.3.1.2");
pub const szOID_ECC_CURVE_X962P192V3: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10045.3.1.3");
pub const szOID_ECC_CURVE_X962P239V1: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10045.3.1.4");
pub const szOID_ECC_CURVE_X962P239V2: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10045.3.1.5");
pub const szOID_ECC_CURVE_X962P239V3: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10045.3.1.6");
pub const szOID_ECC_PUBLIC_KEY: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10045.2.1");
pub const szOID_ECDSA_SHA1: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10045.4.1");
pub const szOID_ECDSA_SHA256: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10045.4.3.2");
pub const szOID_ECDSA_SHA384: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10045.4.3.3");
pub const szOID_ECDSA_SHA512: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10045.4.3.4");
pub const szOID_ECDSA_SPECIFIED: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10045.4.3");
pub const szOID_EFS_RECOVERY: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.4.1");
pub const szOID_EMBEDDED_NT_CRYPTO: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.8");
pub const szOID_ENCLAVE_SIGNING: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.42");
pub const szOID_ENCRYPTED_KEY_HASH: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.21");
pub const szOID_ENHANCED_KEY_USAGE: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.37");
pub const szOID_ENROLLMENT_AGENT: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.20.2.1");
pub const szOID_ENROLLMENT_CSP_PROVIDER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.13.2.2");
pub const szOID_ENROLLMENT_NAME_VALUE_PAIR: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.13.2.1");
pub const szOID_ENROLL_AIK_INFO: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.39");
pub const szOID_ENROLL_ATTESTATION_CHALLENGE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.28");
pub const szOID_ENROLL_ATTESTATION_STATEMENT: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.24");
pub const szOID_ENROLL_CAXCHGCERT_HASH: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.27");
pub const szOID_ENROLL_CERTTYPE_EXTENSION: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.20.2");
pub const szOID_ENROLL_EKPUB_CHALLENGE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.26");
pub const szOID_ENROLL_EKVERIFYCERT: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.31");
pub const szOID_ENROLL_EKVERIFYCREDS: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.32");
pub const szOID_ENROLL_EKVERIFYKEY: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.30");
pub const szOID_ENROLL_EK_CA_KEYID: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.43");
pub const szOID_ENROLL_EK_INFO: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.23");
pub const szOID_ENROLL_ENCRYPTION_ALGORITHM: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.29");
pub const szOID_ENROLL_KEY_AFFINITY: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.41");
pub const szOID_ENROLL_KSP_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.25");
pub const szOID_ENROLL_SCEP_CHALLENGE_ANSWER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.35");
pub const szOID_ENROLL_SCEP_CLIENT_REQUEST: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.37");
pub const szOID_ENROLL_SCEP_ERROR: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.33");
pub const szOID_ENROLL_SCEP_SERVER_MESSAGE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.38");
pub const szOID_ENROLL_SCEP_SERVER_SECRET: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.40");
pub const szOID_ENROLL_SCEP_SERVER_STATE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.34");
pub const szOID_ENROLL_SCEP_SIGNER_HASH: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.42");
pub const szOID_ENTERPRISE_OID_ROOT: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.8");
pub const szOID_EV_RDN_COUNTRY: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.60.2.1.3");
pub const szOID_EV_RDN_LOCALE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.60.2.1.1");
pub const szOID_EV_RDN_STATE_OR_PROVINCE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.60.2.1.2");
pub const szOID_EV_WHQL_CRYPTO: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.39");
pub const szOID_FACSIMILE_TELEPHONE_NUMBER: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.23");
pub const szOID_FLIGHT_CTL_EXT: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.51");
pub const szOID_FRESHEST_CRL: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.46");
pub const szOID_GIVEN_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.42");
pub const szOID_HPKP_DOMAIN_NAME_CTL: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.60");
pub const szOID_HPKP_HEADER_VALUE_CTL: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.61");
pub const szOID_INFOSEC: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1");
pub const szOID_INFOSEC_SuiteAConfidentiality: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.14");
pub const szOID_INFOSEC_SuiteAIntegrity: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.15");
pub const szOID_INFOSEC_SuiteAKMandSig: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.18");
pub const szOID_INFOSEC_SuiteAKeyManagement: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.17");
pub const szOID_INFOSEC_SuiteASignature: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.13");
pub const szOID_INFOSEC_SuiteATokenProtection: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.16");
pub const szOID_INFOSEC_mosaicConfidentiality: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.4");
pub const szOID_INFOSEC_mosaicIntegrity: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.6");
pub const szOID_INFOSEC_mosaicKMandSig: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.12");
pub const szOID_INFOSEC_mosaicKMandUpdSig: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.20");
pub const szOID_INFOSEC_mosaicKeyManagement: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.10");
pub const szOID_INFOSEC_mosaicSignature: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.2");
pub const szOID_INFOSEC_mosaicTokenProtection: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.8");
pub const szOID_INFOSEC_mosaicUpdatedInteg: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.21");
pub const szOID_INFOSEC_mosaicUpdatedSig: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.19");
pub const szOID_INFOSEC_sdnsConfidentiality: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.3");
pub const szOID_INFOSEC_sdnsIntegrity: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.5");
pub const szOID_INFOSEC_sdnsKMandSig: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.11");
pub const szOID_INFOSEC_sdnsKeyManagement: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.9");
pub const szOID_INFOSEC_sdnsSignature: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.1");
pub const szOID_INFOSEC_sdnsTokenProtection: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.2.1.1.7");
pub const szOID_INHIBIT_ANY_POLICY: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.54");
pub const szOID_INITIALS: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.43");
pub const szOID_INTERNATIONALIZED_EMAIL_ADDRESS: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.20.2.4");
pub const szOID_INTERNATIONAL_ISDN_NUMBER: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.25");
pub const szOID_IPSEC_KP_IKE_INTERMEDIATE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.8.2.2");
pub const szOID_ISSUED_CERT_HASH: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.17");
pub const szOID_ISSUER_ALT_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.8");
pub const szOID_ISSUER_ALT_NAME2: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.18");
pub const szOID_ISSUING_DIST_POINT: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.28");
pub const szOID_IUM_SIGNING: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.37");
pub const szOID_KEYID_RDN: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.7.1");
pub const szOID_KEY_ATTRIBUTES: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.2");
pub const szOID_KEY_USAGE: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.15");
pub const szOID_KEY_USAGE_RESTRICTION: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.4");
pub const szOID_KP_CA_EXCHANGE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.5");
pub const szOID_KP_CSP_SIGNATURE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.16");
pub const szOID_KP_CTL_USAGE_SIGNING: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.1");
pub const szOID_KP_DOCUMENT_SIGNING: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.12");
pub const szOID_KP_EFS: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.4");
pub const szOID_KP_FLIGHT_SIGNING: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.27");
pub const szOID_KP_KERNEL_MODE_CODE_SIGNING: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.61.1.1");
pub const szOID_KP_KERNEL_MODE_HAL_EXTENSION_SIGNING: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.61.5.1");
pub const szOID_KP_KERNEL_MODE_TRUSTED_BOOT_SIGNING: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.61.4.1");
pub const szOID_KP_KEY_RECOVERY: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.11");
pub const szOID_KP_KEY_RECOVERY_AGENT: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.6");
pub const szOID_KP_LIFETIME_SIGNING: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.13");
pub const szOID_KP_MOBILE_DEVICE_SOFTWARE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.14");
pub const szOID_KP_PRIVACY_CA: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.36");
pub const szOID_KP_QUALIFIED_SUBORDINATION: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.10");
pub const szOID_KP_SMARTCARD_LOGON: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.20.2.2");
pub const szOID_KP_SMART_DISPLAY: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.15");
pub const szOID_KP_TIME_STAMP_SIGNING: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.2");
pub const szOID_KP_TPM_AIK_CERTIFICATE: windows_sys::core::PCSTR = windows_sys::core::s!("2.23.133.8.3");
pub const szOID_KP_TPM_EK_CERTIFICATE: windows_sys::core::PCSTR = windows_sys::core::s!("2.23.133.8.1");
pub const szOID_KP_TPM_PLATFORM_CERTIFICATE: windows_sys::core::PCSTR = windows_sys::core::s!("2.23.133.8.2");
pub const szOID_LEGACY_POLICY_MAPPINGS: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.5");
pub const szOID_LICENSES: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.6.1");
pub const szOID_LICENSE_SERVER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.6.2");
pub const szOID_LOCALITY_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.7");
pub const szOID_LOCAL_MACHINE_KEYSET: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.17.2");
pub const szOID_LOGOTYPE_EXT: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.1.12");
pub const szOID_LOYALTY_OTHER_LOGOTYPE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.20.1");
pub const szOID_MEMBER: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.31");
pub const szOID_MICROSOFT_PUBLISHER_SIGNER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.76.8.1");
pub const szOID_NAME_CONSTRAINTS: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.30");
pub const szOID_NETSCAPE: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113730");
pub const szOID_NETSCAPE_BASE_URL: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113730.1.2");
pub const szOID_NETSCAPE_CA_POLICY_URL: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113730.1.8");
pub const szOID_NETSCAPE_CA_REVOCATION_URL: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113730.1.4");
pub const szOID_NETSCAPE_CERT_EXTENSION: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113730.1");
pub const szOID_NETSCAPE_CERT_RENEWAL_URL: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113730.1.7");
pub const szOID_NETSCAPE_CERT_SEQUENCE: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113730.2.5");
pub const szOID_NETSCAPE_CERT_TYPE: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113730.1.1");
pub const szOID_NETSCAPE_COMMENT: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113730.1.13");
pub const szOID_NETSCAPE_DATA_TYPE: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113730.2");
pub const szOID_NETSCAPE_REVOCATION_URL: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113730.1.3");
pub const szOID_NETSCAPE_SSL_SERVER_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113730.1.12");
pub const szOID_NEXT_UPDATE_LOCATION: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.2");
pub const szOID_NIST_AES128_CBC: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.1.2");
pub const szOID_NIST_AES128_WRAP: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.1.5");
pub const szOID_NIST_AES192_CBC: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.1.22");
pub const szOID_NIST_AES192_WRAP: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.1.25");
pub const szOID_NIST_AES256_CBC: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.1.42");
pub const szOID_NIST_AES256_WRAP: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.1.45");
pub const szOID_NIST_hash_ml_dsa_44_with_sha512: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.32");
pub const szOID_NIST_hash_ml_dsa_65_with_sha512: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.33");
pub const szOID_NIST_hash_ml_dsa_87_with_sha512: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.34");
pub const szOID_NIST_hash_slh_dsa_sha2_128f_with_sha256: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.36");
pub const szOID_NIST_hash_slh_dsa_sha2_128s_with_sha256: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.35");
pub const szOID_NIST_hash_slh_dsa_sha2_192f_with_sha512: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.38");
pub const szOID_NIST_hash_slh_dsa_sha2_192s_with_sha512: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.37");
pub const szOID_NIST_hash_slh_dsa_sha2_256f_with_sha512: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.40");
pub const szOID_NIST_hash_slh_dsa_sha2_256s_with_sha512: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.39");
pub const szOID_NIST_hash_slh_dsa_shake_128f_with_shake128: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.42");
pub const szOID_NIST_hash_slh_dsa_shake_128s_with_shake128: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.41");
pub const szOID_NIST_hash_slh_dsa_shake_192f_with_shake256: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.44");
pub const szOID_NIST_hash_slh_dsa_shake_192s_with_shake256: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.43");
pub const szOID_NIST_hash_slh_dsa_shake_256f_with_shake256: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.46");
pub const szOID_NIST_hash_slh_dsa_shake_256s_with_shake256: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.45");
pub const szOID_NIST_ml_dsa_44: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.17");
pub const szOID_NIST_ml_dsa_65: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.18");
pub const szOID_NIST_ml_dsa_87: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.19");
pub const szOID_NIST_ml_kem_1024: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.4.3");
pub const szOID_NIST_ml_kem_512: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.4.1");
pub const szOID_NIST_ml_kem_768: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.4.2");
pub const szOID_NIST_sha256: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.2.1");
pub const szOID_NIST_sha384: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.2.2");
pub const szOID_NIST_sha512: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.2.3");
pub const szOID_NIST_shake128: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.2.11");
pub const szOID_NIST_shake256: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.2.12");
pub const szOID_NIST_slh_dsa_sha2_128f: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.21");
pub const szOID_NIST_slh_dsa_sha2_128s: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.20");
pub const szOID_NIST_slh_dsa_sha2_192f: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.23");
pub const szOID_NIST_slh_dsa_sha2_192s: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.22");
pub const szOID_NIST_slh_dsa_sha2_256f: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.25");
pub const szOID_NIST_slh_dsa_sha2_256s: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.24");
pub const szOID_NIST_slh_dsa_shake_128f: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.27");
pub const szOID_NIST_slh_dsa_shake_128s: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.26");
pub const szOID_NIST_slh_dsa_shake_192f: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.29");
pub const szOID_NIST_slh_dsa_shake_192s: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.28");
pub const szOID_NIST_slh_dsa_shake_256f: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.31");
pub const szOID_NIST_slh_dsa_shake_256s: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.101.3.4.3.30");
pub const szOID_NO_HASH: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.73.1");
pub const szOID_NT5_CRYPTO: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.6");
pub const szOID_NTDS_CA_SECURITY_EXT: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.25.2");
pub const szOID_NTDS_OBJECTSID: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.25.2.1");
pub const szOID_NTDS_REPLICATION: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.25.1");
pub const szOID_NT_PRINCIPAL_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.20.2.3");
pub const szOID_OEM_WHQL_CRYPTO: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.7");
pub const szOID_OIW: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14");
pub const szOID_OIWDIR: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.7.2");
pub const szOID_OIWDIR_CRPT: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.7.2.1");
pub const szOID_OIWDIR_HASH: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.7.2.2");
pub const szOID_OIWDIR_SIGN: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.7.2.3");
pub const szOID_OIWDIR_md2: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.7.2.2.1");
pub const szOID_OIWDIR_md2RSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.7.2.3.1");
pub const szOID_OIWSEC: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2");
pub const szOID_OIWSEC_desCBC: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.7");
pub const szOID_OIWSEC_desCFB: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.9");
pub const szOID_OIWSEC_desECB: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.6");
pub const szOID_OIWSEC_desEDE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.17");
pub const szOID_OIWSEC_desMAC: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.10");
pub const szOID_OIWSEC_desOFB: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.8");
pub const szOID_OIWSEC_dhCommMod: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.16");
pub const szOID_OIWSEC_dsa: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.12");
pub const szOID_OIWSEC_dsaComm: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.20");
pub const szOID_OIWSEC_dsaCommSHA: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.21");
pub const szOID_OIWSEC_dsaCommSHA1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.28");
pub const szOID_OIWSEC_dsaSHA1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.27");
pub const szOID_OIWSEC_keyHashSeal: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.23");
pub const szOID_OIWSEC_md2RSASign: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.24");
pub const szOID_OIWSEC_md4RSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.2");
pub const szOID_OIWSEC_md4RSA2: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.4");
pub const szOID_OIWSEC_md5RSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.3");
pub const szOID_OIWSEC_md5RSASign: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.25");
pub const szOID_OIWSEC_mdc2: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.19");
pub const szOID_OIWSEC_mdc2RSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.14");
pub const szOID_OIWSEC_rsaSign: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.11");
pub const szOID_OIWSEC_rsaXchg: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.22");
pub const szOID_OIWSEC_sha: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.18");
pub const szOID_OIWSEC_sha1: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.26");
pub const szOID_OIWSEC_sha1RSASign: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.29");
pub const szOID_OIWSEC_shaDSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.13");
pub const szOID_OIWSEC_shaRSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.14.3.2.15");
pub const szOID_ORGANIZATIONAL_UNIT_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.11");
pub const szOID_ORGANIZATION_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.10");
pub const szOID_OS_VERSION: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.13.2.3");
pub const szOID_OWNER: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.32");
pub const szOID_PHYSICAL_DELIVERY_OFFICE_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.19");
pub const szOID_PIN_RULES_CTL: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.32");
pub const szOID_PIN_RULES_DOMAIN_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.34");
pub const szOID_PIN_RULES_EXT: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.33");
pub const szOID_PIN_RULES_LOG_END_DATE_EXT: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.35");
pub const szOID_PIN_RULES_SIGNER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.31");
pub const szOID_PKCS: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1");
pub const szOID_PKCS_1: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.1");
pub const szOID_PKCS_10: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.10");
pub const szOID_PKCS_12: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.12");
pub const szOID_PKCS_12_EXTENDED_ATTRIBUTES: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.17.3");
pub const szOID_PKCS_12_FRIENDLY_NAME_ATTR: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.20");
pub const szOID_PKCS_12_KEY_PROVIDER_NAME_ATTR: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.17.1");
pub const szOID_PKCS_12_LOCAL_KEY_ID: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.21");
pub const szOID_PKCS_12_PROTECTED_PASSWORD_SECRET_BAG_TYPE_ID: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.17.4");
pub const szOID_PKCS_12_PbeIds: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.12.1");
pub const szOID_PKCS_12_pbeWithSHA1And128BitRC2: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.12.1.5");
pub const szOID_PKCS_12_pbeWithSHA1And128BitRC4: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.12.1.1");
pub const szOID_PKCS_12_pbeWithSHA1And2KeyTripleDES: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.12.1.4");
pub const szOID_PKCS_12_pbeWithSHA1And3KeyTripleDES: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.12.1.3");
pub const szOID_PKCS_12_pbeWithSHA1And40BitRC2: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.12.1.6");
pub const szOID_PKCS_12_pbeWithSHA1And40BitRC4: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.12.1.2");
pub const szOID_PKCS_2: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.2");
pub const szOID_PKCS_3: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.3");
pub const szOID_PKCS_4: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.4");
pub const szOID_PKCS_5: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.5");
pub const szOID_PKCS_5_PBES2: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.5.13");
pub const szOID_PKCS_5_PBKDF2: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.5.12");
pub const szOID_PKCS_6: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.6");
pub const szOID_PKCS_7: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.7");
pub const szOID_PKCS_7_DATA: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.7.1");
pub const szOID_PKCS_7_DIGESTED: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.7.5");
pub const szOID_PKCS_7_ENCRYPTED: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.7.6");
pub const szOID_PKCS_7_ENVELOPED: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.7.3");
pub const szOID_PKCS_7_SIGNED: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.7.2");
pub const szOID_PKCS_7_SIGNEDANDENVELOPED: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.7.4");
pub const szOID_PKCS_8: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.8");
pub const szOID_PKCS_9: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9");
pub const szOID_PKCS_9_CONTENT_TYPE: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.3");
pub const szOID_PKCS_9_MESSAGE_DIGEST: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.4");
pub const szOID_PKINIT_KP_KDC: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.2.3.5");
pub const szOID_PKIX: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7");
pub const szOID_PKIX_ACC_DESCR: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.48");
pub const szOID_PKIX_CA_ISSUERS: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.48.2");
pub const szOID_PKIX_CA_REPOSITORY: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.48.5");
pub const szOID_PKIX_KP: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.3");
pub const szOID_PKIX_KP_CLIENT_AUTH: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.3.2");
pub const szOID_PKIX_KP_CODE_SIGNING: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.3.3");
pub const szOID_PKIX_KP_EMAIL_PROTECTION: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.3.4");
pub const szOID_PKIX_KP_IPSEC_END_SYSTEM: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.3.5");
pub const szOID_PKIX_KP_IPSEC_TUNNEL: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.3.6");
pub const szOID_PKIX_KP_IPSEC_USER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.3.7");
pub const szOID_PKIX_KP_OCSP_SIGNING: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.3.9");
pub const szOID_PKIX_KP_SERVER_AUTH: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.3.1");
pub const szOID_PKIX_KP_TIMESTAMP_SIGNING: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.3.8");
pub const szOID_PKIX_NO_SIGNATURE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.6.2");
pub const szOID_PKIX_OCSP: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.48.1");
pub const szOID_PKIX_OCSP_BASIC_SIGNED_RESPONSE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.48.1.1");
pub const szOID_PKIX_OCSP_NOCHECK: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.48.1.5");
pub const szOID_PKIX_OCSP_NONCE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.48.1.2");
pub const szOID_PKIX_PE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.1");
pub const szOID_PKIX_POLICY_QUALIFIER_CPS: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.2.1");
pub const szOID_PKIX_POLICY_QUALIFIER_USERNOTICE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.2.2");
pub const szOID_PKIX_TIME_STAMPING: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.48.3");
pub const szOID_PLATFORM_MANIFEST_BINARY_ID: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.28");
pub const szOID_POLICY_CONSTRAINTS: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.36");
pub const szOID_POLICY_MAPPINGS: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.33");
pub const szOID_POSTAL_ADDRESS: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.16");
pub const szOID_POSTAL_CODE: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.17");
pub const szOID_POST_OFFICE_BOX: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.18");
pub const szOID_PREFERRED_DELIVERY_METHOD: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.28");
pub const szOID_PRESENTATION_ADDRESS: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.29");
pub const szOID_PRIVATEKEY_USAGE_PERIOD: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.16");
pub const szOID_PRODUCT_UPDATE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.31.1");
pub const szOID_PROTECTED_PROCESS_LIGHT_SIGNER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.22");
pub const szOID_PROTECTED_PROCESS_SIGNER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.24");
pub const szOID_QC_EU_COMPLIANCE: windows_sys::core::PCSTR = windows_sys::core::s!("0.4.0.1862.1.1");
pub const szOID_QC_SSCD: windows_sys::core::PCSTR = windows_sys::core::s!("0.4.0.1862.1.4");
pub const szOID_QC_STATEMENTS_EXT: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.1.3");
pub const szOID_RDN_DUMMY_SIGNER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.9");
pub const szOID_RDN_TCG_PLATFORM_MANUFACTURER: windows_sys::core::PCSTR = windows_sys::core::s!("2.23.133.2.4");
pub const szOID_RDN_TCG_PLATFORM_MODEL: windows_sys::core::PCSTR = windows_sys::core::s!("2.23.133.2.5");
pub const szOID_RDN_TCG_PLATFORM_VERSION: windows_sys::core::PCSTR = windows_sys::core::s!("2.23.133.2.6");
pub const szOID_RDN_TPM_MANUFACTURER: windows_sys::core::PCSTR = windows_sys::core::s!("2.23.133.2.1");
pub const szOID_RDN_TPM_MODEL: windows_sys::core::PCSTR = windows_sys::core::s!("2.23.133.2.2");
pub const szOID_RDN_TPM_VERSION: windows_sys::core::PCSTR = windows_sys::core::s!("2.23.133.2.3");
pub const szOID_REASON_CODE_HOLD: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.23");
pub const szOID_REGISTERED_ADDRESS: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.26");
pub const szOID_REMOVE_CERTIFICATE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.8.1");
pub const szOID_RENEWAL_CERTIFICATE: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.13.1");
pub const szOID_REQUEST_CLIENT_INFO: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.20");
pub const szOID_REQUIRE_CERT_CHAIN_POLICY: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.21.15");
pub const szOID_REVOKED_LIST_SIGNER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.19");
pub const szOID_RFC3161_counterSign: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.3.3.1");
pub const szOID_RFC3161v21_counterSign: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.3.3.2");
pub const szOID_RFC3161v21_thumbprints: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.3.3.3");
pub const szOID_ROLE_OCCUPANT: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.33");
pub const szOID_ROOT_LIST_SIGNER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.9");
pub const szOID_ROOT_PROGRAM_AUTO_UPDATE_CA_REVOCATION: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.60.3.1");
pub const szOID_ROOT_PROGRAM_AUTO_UPDATE_END_REVOCATION: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.60.3.2");
pub const szOID_ROOT_PROGRAM_FLAGS: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.60.1.1");
pub const szOID_ROOT_PROGRAM_NO_OCSP_FAILOVER_TO_CRL: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.60.3.3");
pub const szOID_RSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549");
pub const szOID_RSAES_OAEP: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.1.7");
pub const szOID_RSA_DES_EDE3_CBC: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.3.7");
pub const szOID_RSA_DH: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.3.1");
pub const szOID_RSA_ENCRYPT: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.3");
pub const szOID_RSA_HASH: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.2");
pub const szOID_RSA_MD2: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.2.2");
pub const szOID_RSA_MD2RSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.1.2");
pub const szOID_RSA_MD4: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.2.4");
pub const szOID_RSA_MD4RSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.1.3");
pub const szOID_RSA_MD5: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.2.5");
pub const szOID_RSA_MD5RSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.1.4");
pub const szOID_RSA_MGF1: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.1.8");
pub const szOID_RSA_PSPECIFIED: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.1.9");
pub const szOID_RSA_RC2CBC: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.3.2");
pub const szOID_RSA_RC4: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.3.4");
pub const szOID_RSA_RC5_CBCPad: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.3.9");
pub const szOID_RSA_RSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.1.1");
pub const szOID_RSA_SETOAEP_RSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.1.6");
pub const szOID_RSA_SHA1RSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.1.5");
pub const szOID_RSA_SHA256RSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.1.11");
pub const szOID_RSA_SHA384RSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.1.12");
pub const szOID_RSA_SHA512RSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.1.13");
pub const szOID_RSA_SMIMECapabilities: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.15");
pub const szOID_RSA_SMIMEalg: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.16.3");
pub const szOID_RSA_SMIMEalgCMS3DESwrap: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.16.3.6");
pub const szOID_RSA_SMIMEalgCMSRC2wrap: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.16.3.7");
pub const szOID_RSA_SMIMEalgESDH: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.16.3.5");
pub const szOID_RSA_SSA_PSS: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.1.10");
pub const szOID_RSA_certExtensions: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.14");
pub const szOID_RSA_challengePwd: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.7");
pub const szOID_RSA_contentType: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.3");
pub const szOID_RSA_counterSign: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.6");
pub const szOID_RSA_data: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.7.1");
pub const szOID_RSA_digestedData: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.7.5");
pub const szOID_RSA_emailAddr: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.1");
pub const szOID_RSA_encryptedData: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.7.6");
pub const szOID_RSA_envelopedData: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.7.3");
pub const szOID_RSA_extCertAttrs: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.9");
pub const szOID_RSA_hashedData: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.7.5");
pub const szOID_RSA_messageDigest: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.4");
pub const szOID_RSA_preferSignedData: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.15.1");
pub const szOID_RSA_signEnvData: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.7.4");
pub const szOID_RSA_signedData: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.7.2");
pub const szOID_RSA_signingTime: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.5");
pub const szOID_RSA_unstructAddr: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.8");
pub const szOID_RSA_unstructName: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.2");
pub const szOID_SEARCH_GUIDE: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.14");
pub const szOID_SEE_ALSO: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.34");
pub const szOID_SERIALIZED: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.3.1");
pub const szOID_SERVER_GATED_CRYPTO: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.3");
pub const szOID_SGC_NETSCAPE: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113730.4.1");
pub const szOID_SITE_PIN_RULES_FLAGS_ATTR: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.4.3");
pub const szOID_SITE_PIN_RULES_INDEX_ATTR: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.4.2");
pub const szOID_SORTED_CTL: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.1.1");
pub const szOID_STATE_OR_PROVINCE_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.8");
pub const szOID_STREET_ADDRESS: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.9");
pub const szOID_SUBJECT_ALT_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.7");
pub const szOID_SUBJECT_ALT_NAME2: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.17");
pub const szOID_SUBJECT_DIR_ATTRS: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.9");
pub const szOID_SUBJECT_INFO_ACCESS: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.1.11");
pub const szOID_SUBJECT_KEY_IDENTIFIER: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.29.14");
pub const szOID_SUPPORTED_APPLICATION_CONTEXT: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.30");
pub const szOID_SUR_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.4");
pub const szOID_SYNC_ROOT_CTL_EXT: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.50");
pub const szOID_TELEPHONE_NUMBER: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.20");
pub const szOID_TELETEXT_TERMINAL_IDENTIFIER: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.22");
pub const szOID_TELEX_NUMBER: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.21");
pub const szOID_TIMESTAMP_TOKEN: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113549.1.9.16.1.4");
pub const szOID_TITLE: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.12");
pub const szOID_TLS_FEATURES_EXT: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.5.5.7.1.24");
pub const szOID_USER_CERTIFICATE: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.36");
pub const szOID_USER_PASSWORD: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.35");
pub const szOID_VERISIGN_BITSTRING_6_13: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113733.1.6.13");
pub const szOID_VERISIGN_ISS_STRONG_CRYPTO: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113733.1.8.1");
pub const szOID_VERISIGN_ONSITE_JURISDICTION_HASH: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113733.1.6.11");
pub const szOID_VERISIGN_PRIVATE_6_9: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113733.1.6.9");
pub const szOID_WHQL_CRYPTO: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.5");
pub const szOID_WINDOWS_KITS_SIGNER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.20");
pub const szOID_WINDOWS_RT_SIGNER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.21");
pub const szOID_WINDOWS_SOFTWARE_EXTENSION_SIGNER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.26");
pub const szOID_WINDOWS_STORE_SIGNER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.76.3.1");
pub const szOID_WINDOWS_TCB_SIGNER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.23");
pub const szOID_WINDOWS_THIRD_PARTY_COMPONENT_SIGNER: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.3.25");
pub const szOID_X21_ADDRESS: windows_sys::core::PCSTR = windows_sys::core::s!("2.5.4.24");
pub const szOID_X957: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10040");
pub const szOID_X957_DSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10040.4.1");
pub const szOID_X957_SHA1DSA: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.10040.4.3");
pub const szOID_YESNO_TRUST_ATTR: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.311.10.4.1");
pub const szPRIV_KEY_CACHE_MAX_ITEMS: windows_sys::core::PCSTR = windows_sys::core::s!("PrivKeyCacheMaxItems");
pub const szPRIV_KEY_CACHE_PURGE_INTERVAL_SECONDS: windows_sys::core::PCSTR = windows_sys::core::s!("PrivKeyCachePurgeIntervalSeconds");
pub const sz_CERT_STORE_PROV_COLLECTION: windows_sys::core::PCSTR = windows_sys::core::s!("Collection");
pub const sz_CERT_STORE_PROV_FILENAME_W: windows_sys::core::PCSTR = windows_sys::core::s!("File");
pub const sz_CERT_STORE_PROV_LDAP_W: windows_sys::core::PCSTR = windows_sys::core::s!("Ldap");
pub const sz_CERT_STORE_PROV_MEMORY: windows_sys::core::PCSTR = windows_sys::core::s!("Memory");
pub const sz_CERT_STORE_PROV_PHYSICAL_W: windows_sys::core::PCSTR = windows_sys::core::s!("Physical");
pub const sz_CERT_STORE_PROV_PKCS12: windows_sys::core::PCSTR = windows_sys::core::s!("PKCS12");
pub const sz_CERT_STORE_PROV_PKCS7: windows_sys::core::PCSTR = windows_sys::core::s!("PKCS7");
pub const sz_CERT_STORE_PROV_SERIALIZED: windows_sys::core::PCSTR = windows_sys::core::s!("Serialized");
pub const sz_CERT_STORE_PROV_SMART_CARD_W: windows_sys::core::PCSTR = windows_sys::core::s!("SmartCard");
pub const sz_CERT_STORE_PROV_SYSTEM_REGISTRY_W: windows_sys::core::PCSTR = windows_sys::core::s!("SystemRegistry");
pub const sz_CERT_STORE_PROV_SYSTEM_W: windows_sys::core::PCSTR = windows_sys::core::s!("System");
pub const wszURI_NTDS_OBJECTSID_PREFIX: windows_sys::core::PCWSTR = windows_sys::core::w!("tag:microsoft.com,2022-09-14:sid:");
