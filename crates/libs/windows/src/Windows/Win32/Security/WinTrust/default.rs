impl ::core::default::Default for CAT_MEMBERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CAT_MEMBERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwszSubjGuid == other.pwszSubjGuid && self.dwCertVersion == other.dwCertVersion
    }
}
impl ::core::cmp::Eq for CAT_MEMBERINFO {}
impl ::core::fmt::Debug for CAT_MEMBERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAT_MEMBERINFO").field("pwszSubjGuid", &self.pwszSubjGuid).field("dwCertVersion", &self.dwCertVersion).finish()
    }
}
impl ::core::default::Default for CAT_MEMBERINFO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CAT_MEMBERINFO2 {
    fn eq(&self, other: &Self) -> bool {
        self.SubjectGuid == other.SubjectGuid && self.dwCertVersion == other.dwCertVersion
    }
}
impl ::core::cmp::Eq for CAT_MEMBERINFO2 {}
impl ::core::fmt::Debug for CAT_MEMBERINFO2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAT_MEMBERINFO2").field("SubjectGuid", &self.SubjectGuid).field("dwCertVersion", &self.dwCertVersion).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for CAT_NAMEVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for CAT_NAMEVALUE {
    fn eq(&self, other: &Self) -> bool {
        self.pwszTag == other.pwszTag && self.fdwFlags == other.fdwFlags && self.Value == other.Value
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for CAT_NAMEVALUE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for CAT_NAMEVALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAT_NAMEVALUE").field("pwszTag", &self.pwszTag).field("fdwFlags", &self.fdwFlags).field("Value", &self.Value).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for CONFIG_CI_PROV_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for CONFIG_CI_PROV_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwPolicies == other.dwPolicies && self.pPolicies == other.pPolicies && self.result == other.result && self.dwScenario == other.dwScenario
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for CONFIG_CI_PROV_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for CONFIG_CI_PROV_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONFIG_CI_PROV_INFO").field("cbSize", &self.cbSize).field("dwPolicies", &self.dwPolicies).field("pPolicies", &self.pPolicies).field("result", &self.result).field("dwScenario", &self.dwScenario).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CONFIG_CI_PROV_INFO_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CONFIG_CI_PROV_INFO_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.hr == other.hr && self.dwResult == other.dwResult && self.dwPolicyIndex == other.dwPolicyIndex && self.fIsExplicitDeny == other.fIsExplicitDeny
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CONFIG_CI_PROV_INFO_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CONFIG_CI_PROV_INFO_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONFIG_CI_PROV_INFO_RESULT").field("hr", &self.hr).field("dwResult", &self.dwResult).field("dwPolicyIndex", &self.dwPolicyIndex).field("fIsExplicitDeny", &self.fIsExplicitDeny).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for CRYPT_PROVIDER_CERT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_CERT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pCert == other.pCert && self.fCommercial == other.fCommercial && self.fTrustedRoot == other.fTrustedRoot && self.fSelfSigned == other.fSelfSigned && self.fTestCert == other.fTestCert && self.dwRevokedReason == other.dwRevokedReason && self.dwConfidence == other.dwConfidence && self.dwError == other.dwError && self.pTrustListContext == other.pTrustListContext && self.fTrustListSignerCert == other.fTrustListSignerCert && self.pCtlContext == other.pCtlContext && self.dwCtlError == other.dwCtlError && self.fIsCyclic == other.fIsCyclic && self.pChainElement == other.pChainElement
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for CRYPT_PROVIDER_CERT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for CRYPT_PROVIDER_CERT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVIDER_CERT")
            .field("cbStruct", &self.cbStruct)
            .field("pCert", &self.pCert)
            .field("fCommercial", &self.fCommercial)
            .field("fTrustedRoot", &self.fTrustedRoot)
            .field("fSelfSigned", &self.fSelfSigned)
            .field("fTestCert", &self.fTestCert)
            .field("dwRevokedReason", &self.dwRevokedReason)
            .field("dwConfidence", &self.dwConfidence)
            .field("dwError", &self.dwError)
            .field("pTrustListContext", &self.pTrustListContext)
            .field("fTrustListSignerCert", &self.fTrustListSignerCert)
            .field("pCtlContext", &self.pCtlContext)
            .field("dwCtlError", &self.dwCtlError)
            .field("fIsCyclic", &self.fIsCyclic)
            .field("pChainElement", &self.pChainElement)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::default::Default for CRYPT_PROVIDER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPT_PROVIDER_DEFUSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_DEFUSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.gActionID == other.gActionID && self.pDefPolicyCallbackData == other.pDefPolicyCallbackData && self.pDefSIPClientData == other.pDefSIPClientData
    }
}
impl ::core::cmp::Eq for CRYPT_PROVIDER_DEFUSAGE {}
impl ::core::fmt::Debug for CRYPT_PROVIDER_DEFUSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVIDER_DEFUSAGE").field("cbStruct", &self.cbStruct).field("gActionID", &self.gActionID).field("pDefPolicyCallbackData", &self.pDefPolicyCallbackData).field("pDefSIPClientData", &self.pDefSIPClientData).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::default::Default for CRYPT_PROVIDER_FUNCTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPT_PROVIDER_PRIVDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_PRIVDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.gProviderID == other.gProviderID && self.cbProvData == other.cbProvData && self.pvProvData == other.pvProvData
    }
}
impl ::core::cmp::Eq for CRYPT_PROVIDER_PRIVDATA {}
impl ::core::fmt::Debug for CRYPT_PROVIDER_PRIVDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVIDER_PRIVDATA").field("cbStruct", &self.cbStruct).field("gProviderID", &self.gProviderID).field("cbProvData", &self.cbProvData).field("pvProvData", &self.pvProvData).finish()
    }
}
impl ::core::default::Default for CRYPT_PROVIDER_REGDEFUSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_REGDEFUSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pgActionID == other.pgActionID && self.pwszDllName == other.pwszDllName && self.pwszLoadCallbackDataFunctionName == other.pwszLoadCallbackDataFunctionName && self.pwszFreeCallbackDataFunctionName == other.pwszFreeCallbackDataFunctionName
    }
}
impl ::core::cmp::Eq for CRYPT_PROVIDER_REGDEFUSAGE {}
impl ::core::fmt::Debug for CRYPT_PROVIDER_REGDEFUSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVIDER_REGDEFUSAGE").field("cbStruct", &self.cbStruct).field("pgActionID", &self.pgActionID).field("pwszDllName", &self.pwszDllName).field("pwszLoadCallbackDataFunctionName", &self.pwszLoadCallbackDataFunctionName).field("pwszFreeCallbackDataFunctionName", &self.pwszFreeCallbackDataFunctionName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for CRYPT_PROVIDER_SGNR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_SGNR {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.sftVerifyAsOf == other.sftVerifyAsOf && self.csCertChain == other.csCertChain && self.pasCertChain == other.pasCertChain && self.dwSignerType == other.dwSignerType && self.psSigner == other.psSigner && self.dwError == other.dwError && self.csCounterSigners == other.csCounterSigners && self.pasCounterSigners == other.pasCounterSigners && self.pChainContext == other.pChainContext
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for CRYPT_PROVIDER_SGNR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for CRYPT_PROVIDER_SGNR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVIDER_SGNR").field("cbStruct", &self.cbStruct).field("sftVerifyAsOf", &self.sftVerifyAsOf).field("csCertChain", &self.csCertChain).field("pasCertChain", &self.pasCertChain).field("dwSignerType", &self.dwSignerType).field("psSigner", &self.psSigner).field("dwError", &self.dwError).field("csCounterSigners", &self.csCounterSigners).field("pasCounterSigners", &self.pasCounterSigners).field("pChainContext", &self.pChainContext).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for CRYPT_PROVIDER_SIGSTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_SIGSTATE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.rhSecondarySigs == other.rhSecondarySigs && self.hPrimarySig == other.hPrimarySig && self.fFirstAttemptMade == other.fFirstAttemptMade && self.fNoMoreSigs == other.fNoMoreSigs && self.cSecondarySigs == other.cSecondarySigs && self.dwCurrentIndex == other.dwCurrentIndex && self.fSupportMultiSig == other.fSupportMultiSig && self.dwCryptoPolicySupport == other.dwCryptoPolicySupport && self.iAttemptCount == other.iAttemptCount && self.fCheckedSealing == other.fCheckedSealing && self.pSealingSignature == other.pSealingSignature
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for CRYPT_PROVIDER_SIGSTATE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for CRYPT_PROVIDER_SIGSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVIDER_SIGSTATE")
            .field("cbStruct", &self.cbStruct)
            .field("rhSecondarySigs", &self.rhSecondarySigs)
            .field("hPrimarySig", &self.hPrimarySig)
            .field("fFirstAttemptMade", &self.fFirstAttemptMade)
            .field("fNoMoreSigs", &self.fNoMoreSigs)
            .field("cSecondarySigs", &self.cSecondarySigs)
            .field("dwCurrentIndex", &self.dwCurrentIndex)
            .field("fSupportMultiSig", &self.fSupportMultiSig)
            .field("dwCryptoPolicySupport", &self.dwCryptoPolicySupport)
            .field("iAttemptCount", &self.iAttemptCount)
            .field("fCheckedSealing", &self.fCheckedSealing)
            .field("pSealingSignature", &self.pSealingSignature)
            .finish()
    }
}
impl ::core::default::Default for CRYPT_PROVUI_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_PROVUI_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFinalError == other.dwFinalError && self.pYesButtonText == other.pYesButtonText && self.pNoButtonText == other.pNoButtonText && self.pMoreInfoButtonText == other.pMoreInfoButtonText && self.pAdvancedLinkText == other.pAdvancedLinkText && self.pCopyActionText == other.pCopyActionText && self.pCopyActionTextNoTS == other.pCopyActionTextNoTS && self.pCopyActionTextNotSigned == other.pCopyActionTextNotSigned
    }
}
impl ::core::cmp::Eq for CRYPT_PROVUI_DATA {}
impl ::core::fmt::Debug for CRYPT_PROVUI_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVUI_DATA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFinalError", &self.dwFinalError)
            .field("pYesButtonText", &self.pYesButtonText)
            .field("pNoButtonText", &self.pNoButtonText)
            .field("pMoreInfoButtonText", &self.pMoreInfoButtonText)
            .field("pAdvancedLinkText", &self.pAdvancedLinkText)
            .field("pCopyActionText", &self.pCopyActionText)
            .field("pCopyActionTextNoTS", &self.pCopyActionTextNoTS)
            .field("pCopyActionTextNotSigned", &self.pCopyActionTextNotSigned)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::default::Default for CRYPT_PROVUI_FUNCS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPT_REGISTER_ACTIONID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_REGISTER_ACTIONID {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.sInitProvider == other.sInitProvider && self.sObjectProvider == other.sObjectProvider && self.sSignatureProvider == other.sSignatureProvider && self.sCertificateProvider == other.sCertificateProvider && self.sCertificatePolicyProvider == other.sCertificatePolicyProvider && self.sFinalPolicyProvider == other.sFinalPolicyProvider && self.sTestPolicyProvider == other.sTestPolicyProvider && self.sCleanupProvider == other.sCleanupProvider
    }
}
impl ::core::cmp::Eq for CRYPT_REGISTER_ACTIONID {}
impl ::core::fmt::Debug for CRYPT_REGISTER_ACTIONID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_REGISTER_ACTIONID")
            .field("cbStruct", &self.cbStruct)
            .field("sInitProvider", &self.sInitProvider)
            .field("sObjectProvider", &self.sObjectProvider)
            .field("sSignatureProvider", &self.sSignatureProvider)
            .field("sCertificateProvider", &self.sCertificateProvider)
            .field("sCertificatePolicyProvider", &self.sCertificatePolicyProvider)
            .field("sFinalPolicyProvider", &self.sFinalPolicyProvider)
            .field("sTestPolicyProvider", &self.sTestPolicyProvider)
            .field("sCleanupProvider", &self.sCleanupProvider)
            .finish()
    }
}
impl ::core::default::Default for CRYPT_TRUST_REG_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_TRUST_REG_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pwszDLLName == other.pwszDLLName && self.pwszFunctionName == other.pwszFunctionName
    }
}
impl ::core::cmp::Eq for CRYPT_TRUST_REG_ENTRY {}
impl ::core::fmt::Debug for CRYPT_TRUST_REG_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_TRUST_REG_ENTRY").field("cbStruct", &self.cbStruct).field("pwszDLLName", &self.pwszDLLName).field("pwszFunctionName", &self.pwszFunctionName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for DRIVER_VER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for DRIVER_VER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2 && self.dwPlatform == other.dwPlatform && self.dwVersion == other.dwVersion && self.wszVersion == other.wszVersion && self.wszSignedBy == other.wszSignedBy && self.pcSignerCertContext == other.pcSignerCertContext && self.sOSVersionLow == other.sOSVersionLow && self.sOSVersionHigh == other.sOSVersionHigh && self.dwBuildNumberLow == other.dwBuildNumberLow && self.dwBuildNumberHigh == other.dwBuildNumberHigh
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for DRIVER_VER_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for DRIVER_VER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_VER_INFO")
            .field("cbStruct", &self.cbStruct)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwPlatform", &self.dwPlatform)
            .field("dwVersion", &self.dwVersion)
            .field("wszVersion", &self.wszVersion)
            .field("wszSignedBy", &self.wszSignedBy)
            .field("pcSignerCertContext", &self.pcSignerCertContext)
            .field("sOSVersionLow", &self.sOSVersionLow)
            .field("sOSVersionHigh", &self.sOSVersionHigh)
            .field("dwBuildNumberLow", &self.dwBuildNumberLow)
            .field("dwBuildNumberHigh", &self.dwBuildNumberHigh)
            .finish()
    }
}
impl ::core::default::Default for DRIVER_VER_MAJORMINOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRIVER_VER_MAJORMINOR {
    fn eq(&self, other: &Self) -> bool {
        self.dwMajor == other.dwMajor && self.dwMinor == other.dwMinor
    }
}
impl ::core::cmp::Eq for DRIVER_VER_MAJORMINOR {}
impl ::core::fmt::Debug for DRIVER_VER_MAJORMINOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_VER_MAJORMINOR").field("dwMajor", &self.dwMajor).field("dwMinor", &self.dwMinor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTENT_TO_SEAL_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTENT_TO_SEAL_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.seal == other.seal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTENT_TO_SEAL_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTENT_TO_SEAL_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTENT_TO_SEAL_ATTRIBUTE").field("version", &self.version).field("seal", &self.seal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::default::Default for PROVDATA_SIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::PartialEq for PROVDATA_SIP {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.gSubject == other.gSubject && self.pSip == other.pSip && self.pCATSip == other.pCATSip && self.psSipSubjectInfo == other.psSipSubjectInfo && self.psSipCATSubjectInfo == other.psSipCATSubjectInfo && self.psIndirectData == other.psIndirectData
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::Eq for PROVDATA_SIP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::fmt::Debug for PROVDATA_SIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROVDATA_SIP").field("cbStruct", &self.cbStruct).field("gSubject", &self.gSubject).field("pSip", &self.pSip).field("pCATSip", &self.pCATSip).field("psSipSubjectInfo", &self.psSipSubjectInfo).field("psSipCATSubjectInfo", &self.psSipCATSubjectInfo).field("psIndirectData", &self.psIndirectData).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SEALING_SIGNATURE_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SEALING_SIGNATURE_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.signerIndex == other.signerIndex && self.signatureAlgorithm == other.signatureAlgorithm && self.encryptedDigest == other.encryptedDigest
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SEALING_SIGNATURE_ATTRIBUTE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for SEALING_SIGNATURE_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEALING_SIGNATURE_ATTRIBUTE").field("version", &self.version).field("signerIndex", &self.signerIndex).field("signatureAlgorithm", &self.signatureAlgorithm).field("encryptedDigest", &self.encryptedDigest).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SEALING_TIMESTAMP_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SEALING_TIMESTAMP_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.signerIndex == other.signerIndex && self.sealTimeStampToken == other.sealTimeStampToken
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SEALING_TIMESTAMP_ATTRIBUTE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for SEALING_TIMESTAMP_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEALING_TIMESTAMP_ATTRIBUTE").field("version", &self.version).field("signerIndex", &self.signerIndex).field("sealTimeStampToken", &self.sealTimeStampToken).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SPC_FINANCIAL_CRITERIA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SPC_FINANCIAL_CRITERIA {
    fn eq(&self, other: &Self) -> bool {
        self.fFinancialInfoAvailable == other.fFinancialInfoAvailable && self.fMeetsCriteria == other.fMeetsCriteria
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SPC_FINANCIAL_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SPC_FINANCIAL_CRITERIA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_FINANCIAL_CRITERIA").field("fFinancialInfoAvailable", &self.fFinancialInfoAvailable).field("fMeetsCriteria", &self.fMeetsCriteria).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SPC_IMAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SPC_IMAGE {
    fn eq(&self, other: &Self) -> bool {
        self.pImageLink == other.pImageLink && self.Bitmap == other.Bitmap && self.Metafile == other.Metafile && self.EnhancedMetafile == other.EnhancedMetafile && self.GifFile == other.GifFile
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SPC_IMAGE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for SPC_IMAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_IMAGE").field("pImageLink", &self.pImageLink).field("Bitmap", &self.Bitmap).field("Metafile", &self.Metafile).field("EnhancedMetafile", &self.EnhancedMetafile).field("GifFile", &self.GifFile).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SPC_INDIRECT_DATA_CONTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SPC_INDIRECT_DATA_CONTENT {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data && self.DigestAlgorithm == other.DigestAlgorithm && self.Digest == other.Digest
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SPC_INDIRECT_DATA_CONTENT {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for SPC_INDIRECT_DATA_CONTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_INDIRECT_DATA_CONTENT").field("Data", &self.Data).field("DigestAlgorithm", &self.DigestAlgorithm).field("Digest", &self.Digest).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SPC_LINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SPC_PE_IMAGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SPC_PE_IMAGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.pFile == other.pFile
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SPC_PE_IMAGE_DATA {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for SPC_PE_IMAGE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_PE_IMAGE_DATA").field("Flags", &self.Flags).field("pFile", &self.pFile).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SPC_SERIALIZED_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SPC_SERIALIZED_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.ClassId == other.ClassId && self.SerializedData == other.SerializedData
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SPC_SERIALIZED_OBJECT {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for SPC_SERIALIZED_OBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_SERIALIZED_OBJECT").field("ClassId", &self.ClassId).field("SerializedData", &self.SerializedData).finish()
    }
}
impl ::core::default::Default for SPC_SIGINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SPC_SIGINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSipVersion == other.dwSipVersion && self.gSIPGuid == other.gSIPGuid && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2 && self.dwReserved3 == other.dwReserved3 && self.dwReserved4 == other.dwReserved4 && self.dwReserved5 == other.dwReserved5
    }
}
impl ::core::cmp::Eq for SPC_SIGINFO {}
impl ::core::fmt::Debug for SPC_SIGINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_SIGINFO").field("dwSipVersion", &self.dwSipVersion).field("gSIPGuid", &self.gSIPGuid).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).field("dwReserved3", &self.dwReserved3).field("dwReserved4", &self.dwReserved4).field("dwReserved5", &self.dwReserved5).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SPC_SP_AGENCY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SPC_SP_AGENCY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pPolicyInformation == other.pPolicyInformation && self.pwszPolicyDisplayText == other.pwszPolicyDisplayText && self.pLogoImage == other.pLogoImage && self.pLogoLink == other.pLogoLink
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SPC_SP_AGENCY_INFO {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for SPC_SP_AGENCY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_SP_AGENCY_INFO").field("pPolicyInformation", &self.pPolicyInformation).field("pwszPolicyDisplayText", &self.pwszPolicyDisplayText).field("pLogoImage", &self.pLogoImage).field("pLogoLink", &self.pLogoLink).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SPC_SP_OPUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SPC_SP_OPUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwszProgramName == other.pwszProgramName && self.pMoreInfo == other.pMoreInfo && self.pPublisherInfo == other.pPublisherInfo
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SPC_SP_OPUS_INFO {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for SPC_SP_OPUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_SP_OPUS_INFO").field("pwszProgramName", &self.pwszProgramName).field("pMoreInfo", &self.pMoreInfo).field("pPublisherInfo", &self.pPublisherInfo).finish()
    }
}
impl ::core::default::Default for SPC_STATEMENT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SPC_STATEMENT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.cKeyPurposeId == other.cKeyPurposeId && self.rgpszKeyPurposeId == other.rgpszKeyPurposeId
    }
}
impl ::core::cmp::Eq for SPC_STATEMENT_TYPE {}
impl ::core::fmt::Debug for SPC_STATEMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_STATEMENT_TYPE").field("cKeyPurposeId", &self.cKeyPurposeId).field("rgpszKeyPurposeId", &self.rgpszKeyPurposeId).finish()
    }
}
impl ::core::default::Default for WINTRUST_BLOB_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINTRUST_BLOB_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.gSubject == other.gSubject && self.pcwszDisplayName == other.pcwszDisplayName && self.cbMemObject == other.cbMemObject && self.pbMemObject == other.pbMemObject && self.cbMemSignedMsg == other.cbMemSignedMsg && self.pbMemSignedMsg == other.pbMemSignedMsg
    }
}
impl ::core::cmp::Eq for WINTRUST_BLOB_INFO {}
impl ::core::fmt::Debug for WINTRUST_BLOB_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINTRUST_BLOB_INFO").field("cbStruct", &self.cbStruct).field("gSubject", &self.gSubject).field("pcwszDisplayName", &self.pcwszDisplayName).field("cbMemObject", &self.cbMemObject).field("pbMemObject", &self.pbMemObject).field("cbMemSignedMsg", &self.cbMemSignedMsg).field("pbMemSignedMsg", &self.pbMemSignedMsg).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WINTRUST_CATALOG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WINTRUST_CATALOG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwCatalogVersion == other.dwCatalogVersion && self.pcwszCatalogFilePath == other.pcwszCatalogFilePath && self.pcwszMemberTag == other.pcwszMemberTag && self.pcwszMemberFilePath == other.pcwszMemberFilePath && self.hMemberFile == other.hMemberFile && self.pbCalculatedFileHash == other.pbCalculatedFileHash && self.cbCalculatedFileHash == other.cbCalculatedFileHash && self.pcCatalogContext == other.pcCatalogContext && self.hCatAdmin == other.hCatAdmin
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WINTRUST_CATALOG_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WINTRUST_CATALOG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINTRUST_CATALOG_INFO")
            .field("cbStruct", &self.cbStruct)
            .field("dwCatalogVersion", &self.dwCatalogVersion)
            .field("pcwszCatalogFilePath", &self.pcwszCatalogFilePath)
            .field("pcwszMemberTag", &self.pcwszMemberTag)
            .field("pcwszMemberFilePath", &self.pcwszMemberFilePath)
            .field("hMemberFile", &self.hMemberFile)
            .field("pbCalculatedFileHash", &self.pbCalculatedFileHash)
            .field("cbCalculatedFileHash", &self.cbCalculatedFileHash)
            .field("pcCatalogContext", &self.pcCatalogContext)
            .field("hCatAdmin", &self.hCatAdmin)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WINTRUST_CERT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WINTRUST_CERT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pcwszDisplayName == other.pcwszDisplayName && self.psCertContext == other.psCertContext && self.chStores == other.chStores && self.pahStores == other.pahStores && self.dwFlags == other.dwFlags && self.psftVerifyAsOf == other.psftVerifyAsOf
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WINTRUST_CERT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WINTRUST_CERT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINTRUST_CERT_INFO").field("cbStruct", &self.cbStruct).field("pcwszDisplayName", &self.pcwszDisplayName).field("psCertContext", &self.psCertContext).field("chStores", &self.chStores).field("pahStores", &self.pahStores).field("dwFlags", &self.dwFlags).field("psftVerifyAsOf", &self.psftVerifyAsOf).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WINTRUST_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINTRUST_DATA_PROVIDER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINTRUST_DATA_PROVIDER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_DATA_PROVIDER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WINTRUST_DATA_PROVIDER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WINTRUST_DATA_PROVIDER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WINTRUST_DATA_PROVIDER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WINTRUST_DATA_PROVIDER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WINTRUST_DATA_PROVIDER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for WINTRUST_DATA_REVOCATION_CHECKS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINTRUST_DATA_REVOCATION_CHECKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_DATA_REVOCATION_CHECKS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINTRUST_DATA_STATE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINTRUST_DATA_STATE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_DATA_STATE_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINTRUST_DATA_UICHOICE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINTRUST_DATA_UICHOICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_DATA_UICHOICE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINTRUST_DATA_UICONTEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINTRUST_DATA_UICONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_DATA_UICONTEXT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINTRUST_DATA_UNION_CHOICE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINTRUST_DATA_UNION_CHOICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_DATA_UNION_CHOICE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINTRUST_FILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINTRUST_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pcwszFilePath == other.pcwszFilePath && self.hFile == other.hFile && self.pgKnownSubject == other.pgKnownSubject
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINTRUST_FILE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINTRUST_FILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINTRUST_FILE_INFO").field("cbStruct", &self.cbStruct).field("pcwszFilePath", &self.pcwszFilePath).field("hFile", &self.hFile).field("pgKnownSubject", &self.pgKnownSubject).finish()
    }
}
impl ::core::default::Default for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINTRUST_POLICY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINTRUST_POLICY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_POLICY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WINTRUST_POLICY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WINTRUST_POLICY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WINTRUST_POLICY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WINTRUST_POLICY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WINTRUST_POLICY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for WINTRUST_SGNR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for WINTRUST_SGNR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pcwszDisplayName == other.pcwszDisplayName && self.psSignerInfo == other.psSignerInfo && self.chStores == other.chStores && self.pahStores == other.pahStores
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for WINTRUST_SGNR_INFO {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for WINTRUST_SGNR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINTRUST_SGNR_INFO").field("cbStruct", &self.cbStruct).field("pcwszDisplayName", &self.pcwszDisplayName).field("psSignerInfo", &self.psSignerInfo).field("chStores", &self.chStores).field("pahStores", &self.pahStores).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for WINTRUST_SIGNATURE_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for WINTRUST_SIGNATURE_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwIndex == other.dwIndex && self.dwFlags == other.dwFlags && self.cSecondarySigs == other.cSecondarySigs && self.dwVerifiedSigIndex == other.dwVerifiedSigIndex && self.pCryptoPolicy == other.pCryptoPolicy
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for WINTRUST_SIGNATURE_SETTINGS {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for WINTRUST_SIGNATURE_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINTRUST_SIGNATURE_SETTINGS").field("cbStruct", &self.cbStruct).field("dwIndex", &self.dwIndex).field("dwFlags", &self.dwFlags).field("cSecondarySigs", &self.cSecondarySigs).field("dwVerifiedSigIndex", &self.dwVerifiedSigIndex).field("pCryptoPolicy", &self.pCryptoPolicy).finish()
    }
}
impl ::core::default::Default for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_SIGNATURE_SETTINGS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WIN_CERTIFICATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIN_CERTIFICATE {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.wRevision == other.wRevision && self.wCertificateType == other.wCertificateType && self.bCertificate == other.bCertificate
    }
}
impl ::core::cmp::Eq for WIN_CERTIFICATE {}
impl ::core::fmt::Debug for WIN_CERTIFICATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN_CERTIFICATE").field("dwLength", &self.dwLength).field("wRevision", &self.wRevision).field("wCertificateType", &self.wCertificateType).field("bCertificate", &self.bCertificate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.hClientToken == other.hClientToken && self.lpCertificate == other.lpCertificate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIN_SPUB_TRUSTED_PUBLISHER_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN_SPUB_TRUSTED_PUBLISHER_DATA").field("hClientToken", &self.hClientToken).field("lpCertificate", &self.lpCertificate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.hClientToken == other.hClientToken && self.SubjectType == other.SubjectType && self.Subject == other.Subject
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT").field("hClientToken", &self.hClientToken).field("SubjectType", &self.SubjectType).field("Subject", &self.Subject).finish()
    }
}
impl ::core::default::Default for WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    fn eq(&self, other: &Self) -> bool {
        self.SubjectType == other.SubjectType && self.Subject == other.Subject
    }
}
impl ::core::cmp::Eq for WIN_TRUST_ACTDATA_SUBJECT_ONLY {}
impl ::core::fmt::Debug for WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN_TRUST_ACTDATA_SUBJECT_ONLY").field("SubjectType", &self.SubjectType).field("Subject", &self.Subject).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIN_TRUST_SUBJECT_FILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIN_TRUST_SUBJECT_FILE {
    fn eq(&self, other: &Self) -> bool {
        self.hFile == other.hFile && self.lpPath == other.lpPath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIN_TRUST_SUBJECT_FILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIN_TRUST_SUBJECT_FILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN_TRUST_SUBJECT_FILE").field("hFile", &self.hFile).field("lpPath", &self.lpPath).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    fn eq(&self, other: &Self) -> bool {
        self.hFile == other.hFile && self.lpPath == other.lpPath && self.lpDisplayName == other.lpDisplayName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN_TRUST_SUBJECT_FILE_AND_DISPLAY").field("hFile", &self.hFile).field("lpPath", &self.lpPath).field("lpDisplayName", &self.lpDisplayName).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::default::Default for WTD_GENERIC_CHAIN_POLICY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
