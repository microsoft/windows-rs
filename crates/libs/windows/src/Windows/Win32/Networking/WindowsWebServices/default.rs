impl ::core::cmp::PartialEq for IContentPrefetcherTaskTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContentPrefetcherTaskTrigger {}
impl ::core::fmt::Debug for IContentPrefetcherTaskTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContentPrefetcherTaskTrigger").field(&self.0).finish()
    }
}
impl ::core::default::Default for WEBAUTHN_ASSERTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEBAUTHN_ASSERTION {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbAuthenticatorData == other.cbAuthenticatorData && self.pbAuthenticatorData == other.pbAuthenticatorData && self.cbSignature == other.cbSignature && self.pbSignature == other.pbSignature && self.Credential == other.Credential && self.cbUserId == other.cbUserId && self.pbUserId == other.pbUserId && self.Extensions == other.Extensions && self.cbCredLargeBlob == other.cbCredLargeBlob && self.pbCredLargeBlob == other.pbCredLargeBlob && self.dwCredLargeBlobStatus == other.dwCredLargeBlobStatus
    }
}
impl ::core::cmp::Eq for WEBAUTHN_ASSERTION {}
impl ::core::fmt::Debug for WEBAUTHN_ASSERTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_ASSERTION")
            .field("dwVersion", &self.dwVersion)
            .field("cbAuthenticatorData", &self.cbAuthenticatorData)
            .field("pbAuthenticatorData", &self.pbAuthenticatorData)
            .field("cbSignature", &self.cbSignature)
            .field("pbSignature", &self.pbSignature)
            .field("Credential", &self.Credential)
            .field("cbUserId", &self.cbUserId)
            .field("pbUserId", &self.pbUserId)
            .field("Extensions", &self.Extensions)
            .field("cbCredLargeBlob", &self.cbCredLargeBlob)
            .field("pbCredLargeBlob", &self.pbCredLargeBlob)
            .field("dwCredLargeBlobStatus", &self.dwCredLargeBlobStatus)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwTimeoutMilliseconds == other.dwTimeoutMilliseconds && self.CredentialList == other.CredentialList && self.Extensions == other.Extensions && self.dwAuthenticatorAttachment == other.dwAuthenticatorAttachment && self.dwUserVerificationRequirement == other.dwUserVerificationRequirement && self.dwFlags == other.dwFlags && self.pwszU2fAppId == other.pwszU2fAppId && self.pbU2fAppId == other.pbU2fAppId && self.pCancellationId == other.pCancellationId && self.pAllowCredentialList == other.pAllowCredentialList && self.dwCredLargeBlobOperation == other.dwCredLargeBlobOperation && self.cbCredLargeBlob == other.cbCredLargeBlob && self.pbCredLargeBlob == other.pbCredLargeBlob
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS")
            .field("dwVersion", &self.dwVersion)
            .field("dwTimeoutMilliseconds", &self.dwTimeoutMilliseconds)
            .field("CredentialList", &self.CredentialList)
            .field("Extensions", &self.Extensions)
            .field("dwAuthenticatorAttachment", &self.dwAuthenticatorAttachment)
            .field("dwUserVerificationRequirement", &self.dwUserVerificationRequirement)
            .field("dwFlags", &self.dwFlags)
            .field("pwszU2fAppId", &self.pwszU2fAppId)
            .field("pbU2fAppId", &self.pbU2fAppId)
            .field("pCancellationId", &self.pCancellationId)
            .field("pAllowCredentialList", &self.pAllowCredentialList)
            .field("dwCredLargeBlobOperation", &self.dwCredLargeBlobOperation)
            .field("cbCredLargeBlob", &self.cbCredLargeBlob)
            .field("pbCredLargeBlob", &self.pbCredLargeBlob)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwTimeoutMilliseconds == other.dwTimeoutMilliseconds && self.CredentialList == other.CredentialList && self.Extensions == other.Extensions && self.dwAuthenticatorAttachment == other.dwAuthenticatorAttachment && self.bRequireResidentKey == other.bRequireResidentKey && self.dwUserVerificationRequirement == other.dwUserVerificationRequirement && self.dwAttestationConveyancePreference == other.dwAttestationConveyancePreference && self.dwFlags == other.dwFlags && self.pCancellationId == other.pCancellationId && self.pExcludeCredentialList == other.pExcludeCredentialList && self.dwEnterpriseAttestation == other.dwEnterpriseAttestation && self.dwLargeBlobSupport == other.dwLargeBlobSupport && self.bPreferResidentKey == other.bPreferResidentKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS")
            .field("dwVersion", &self.dwVersion)
            .field("dwTimeoutMilliseconds", &self.dwTimeoutMilliseconds)
            .field("CredentialList", &self.CredentialList)
            .field("Extensions", &self.Extensions)
            .field("dwAuthenticatorAttachment", &self.dwAuthenticatorAttachment)
            .field("bRequireResidentKey", &self.bRequireResidentKey)
            .field("dwUserVerificationRequirement", &self.dwUserVerificationRequirement)
            .field("dwAttestationConveyancePreference", &self.dwAttestationConveyancePreference)
            .field("dwFlags", &self.dwFlags)
            .field("pCancellationId", &self.pCancellationId)
            .field("pExcludeCredentialList", &self.pExcludeCredentialList)
            .field("dwEnterpriseAttestation", &self.dwEnterpriseAttestation)
            .field("dwLargeBlobSupport", &self.dwLargeBlobSupport)
            .field("bPreferResidentKey", &self.bPreferResidentKey)
            .finish()
    }
}
impl ::core::default::Default for WEBAUTHN_CLIENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEBAUTHN_CLIENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbClientDataJSON == other.cbClientDataJSON && self.pbClientDataJSON == other.pbClientDataJSON && self.pwszHashAlgId == other.pwszHashAlgId
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CLIENT_DATA {}
impl ::core::fmt::Debug for WEBAUTHN_CLIENT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CLIENT_DATA").field("dwVersion", &self.dwVersion).field("cbClientDataJSON", &self.cbClientDataJSON).field("pbClientDataJSON", &self.pbClientDataJSON).field("pwszHashAlgId", &self.pwszHashAlgId).finish()
    }
}
impl ::core::default::Default for WEBAUTHN_COMMON_ATTESTATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEBAUTHN_COMMON_ATTESTATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.pwszAlg == other.pwszAlg && self.lAlg == other.lAlg && self.cbSignature == other.cbSignature && self.pbSignature == other.pbSignature && self.cX5c == other.cX5c && self.pX5c == other.pX5c && self.pwszVer == other.pwszVer && self.cbCertInfo == other.cbCertInfo && self.pbCertInfo == other.pbCertInfo && self.cbPubArea == other.cbPubArea && self.pbPubArea == other.pbPubArea
    }
}
impl ::core::cmp::Eq for WEBAUTHN_COMMON_ATTESTATION {}
impl ::core::fmt::Debug for WEBAUTHN_COMMON_ATTESTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_COMMON_ATTESTATION").field("dwVersion", &self.dwVersion).field("pwszAlg", &self.pwszAlg).field("lAlg", &self.lAlg).field("cbSignature", &self.cbSignature).field("pbSignature", &self.pbSignature).field("cX5c", &self.cX5c).field("pX5c", &self.pX5c).field("pwszVer", &self.pwszVer).field("cbCertInfo", &self.cbCertInfo).field("pbCertInfo", &self.pbCertInfo).field("cbPubArea", &self.cbPubArea).field("pbPubArea", &self.pbPubArea).finish()
    }
}
impl ::core::default::Default for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.pwszCredentialType == other.pwszCredentialType && self.lAlg == other.lAlg
    }
}
impl ::core::cmp::Eq for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {}
impl ::core::fmt::Debug for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_COSE_CREDENTIAL_PARAMETER").field("dwVersion", &self.dwVersion).field("pwszCredentialType", &self.pwszCredentialType).field("lAlg", &self.lAlg).finish()
    }
}
impl ::core::default::Default for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.cCredentialParameters == other.cCredentialParameters && self.pCredentialParameters == other.pCredentialParameters
    }
}
impl ::core::cmp::Eq for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {}
impl ::core::fmt::Debug for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_COSE_CREDENTIAL_PARAMETERS").field("cCredentialParameters", &self.cCredentialParameters).field("pCredentialParameters", &self.pCredentialParameters).finish()
    }
}
impl ::core::default::Default for WEBAUTHN_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEBAUTHN_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbId == other.cbId && self.pbId == other.pbId && self.pwszCredentialType == other.pwszCredentialType
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CREDENTIAL {}
impl ::core::fmt::Debug for WEBAUTHN_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CREDENTIAL").field("dwVersion", &self.dwVersion).field("cbId", &self.cbId).field("pbId", &self.pbId).field("pwszCredentialType", &self.pwszCredentialType).finish()
    }
}
impl ::core::default::Default for WEBAUTHN_CREDENTIALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEBAUTHN_CREDENTIALS {
    fn eq(&self, other: &Self) -> bool {
        self.cCredentials == other.cCredentials && self.pCredentials == other.pCredentials
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CREDENTIALS {}
impl ::core::fmt::Debug for WEBAUTHN_CREDENTIALS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CREDENTIALS").field("cCredentials", &self.cCredentials).field("pCredentials", &self.pCredentials).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WEBAUTHN_CREDENTIAL_ATTESTATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WEBAUTHN_CREDENTIAL_ATTESTATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion
            && self.pwszFormatType == other.pwszFormatType
            && self.cbAuthenticatorData == other.cbAuthenticatorData
            && self.pbAuthenticatorData == other.pbAuthenticatorData
            && self.cbAttestation == other.cbAttestation
            && self.pbAttestation == other.pbAttestation
            && self.dwAttestationDecodeType == other.dwAttestationDecodeType
            && self.pvAttestationDecode == other.pvAttestationDecode
            && self.cbAttestationObject == other.cbAttestationObject
            && self.pbAttestationObject == other.pbAttestationObject
            && self.cbCredentialId == other.cbCredentialId
            && self.pbCredentialId == other.pbCredentialId
            && self.Extensions == other.Extensions
            && self.dwUsedTransport == other.dwUsedTransport
            && self.bEpAtt == other.bEpAtt
            && self.bLargeBlobSupported == other.bLargeBlobSupported
            && self.bResidentKey == other.bResidentKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WEBAUTHN_CREDENTIAL_ATTESTATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WEBAUTHN_CREDENTIAL_ATTESTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CREDENTIAL_ATTESTATION")
            .field("dwVersion", &self.dwVersion)
            .field("pwszFormatType", &self.pwszFormatType)
            .field("cbAuthenticatorData", &self.cbAuthenticatorData)
            .field("pbAuthenticatorData", &self.pbAuthenticatorData)
            .field("cbAttestation", &self.cbAttestation)
            .field("pbAttestation", &self.pbAttestation)
            .field("dwAttestationDecodeType", &self.dwAttestationDecodeType)
            .field("pvAttestationDecode", &self.pvAttestationDecode)
            .field("cbAttestationObject", &self.cbAttestationObject)
            .field("pbAttestationObject", &self.pbAttestationObject)
            .field("cbCredentialId", &self.cbCredentialId)
            .field("pbCredentialId", &self.pbCredentialId)
            .field("Extensions", &self.Extensions)
            .field("dwUsedTransport", &self.dwUsedTransport)
            .field("bEpAtt", &self.bEpAtt)
            .field("bLargeBlobSupported", &self.bLargeBlobSupported)
            .field("bResidentKey", &self.bResidentKey)
            .finish()
    }
}
impl ::core::default::Default for WEBAUTHN_CREDENTIAL_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEBAUTHN_CREDENTIAL_EX {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbId == other.cbId && self.pbId == other.pbId && self.pwszCredentialType == other.pwszCredentialType && self.dwTransports == other.dwTransports
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CREDENTIAL_EX {}
impl ::core::fmt::Debug for WEBAUTHN_CREDENTIAL_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CREDENTIAL_EX").field("dwVersion", &self.dwVersion).field("cbId", &self.cbId).field("pbId", &self.pbId).field("pwszCredentialType", &self.pwszCredentialType).field("dwTransports", &self.dwTransports).finish()
    }
}
impl ::core::default::Default for WEBAUTHN_CREDENTIAL_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEBAUTHN_CREDENTIAL_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.cCredentials == other.cCredentials && self.ppCredentials == other.ppCredentials
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CREDENTIAL_LIST {}
impl ::core::fmt::Debug for WEBAUTHN_CREDENTIAL_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CREDENTIAL_LIST").field("cCredentials", &self.cCredentials).field("ppCredentials", &self.ppCredentials).finish()
    }
}
impl ::core::default::Default for WEBAUTHN_CRED_BLOB_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEBAUTHN_CRED_BLOB_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.cbCredBlob == other.cbCredBlob && self.pbCredBlob == other.pbCredBlob
    }
}
impl ::core::cmp::Eq for WEBAUTHN_CRED_BLOB_EXTENSION {}
impl ::core::fmt::Debug for WEBAUTHN_CRED_BLOB_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CRED_BLOB_EXTENSION").field("cbCredBlob", &self.cbCredBlob).field("pbCredBlob", &self.pbCredBlob).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    fn eq(&self, other: &Self) -> bool {
        self.dwCredProtect == other.dwCredProtect && self.bRequireCredProtect == other.bRequireCredProtect
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_CRED_PROTECT_EXTENSION_IN").field("dwCredProtect", &self.dwCredProtect).field("bRequireCredProtect", &self.bRequireCredProtect).finish()
    }
}
impl ::core::default::Default for WEBAUTHN_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEBAUTHN_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.pwszExtensionIdentifier == other.pwszExtensionIdentifier && self.cbExtension == other.cbExtension && self.pvExtension == other.pvExtension
    }
}
impl ::core::cmp::Eq for WEBAUTHN_EXTENSION {}
impl ::core::fmt::Debug for WEBAUTHN_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_EXTENSION").field("pwszExtensionIdentifier", &self.pwszExtensionIdentifier).field("cbExtension", &self.cbExtension).field("pvExtension", &self.pvExtension).finish()
    }
}
impl ::core::default::Default for WEBAUTHN_EXTENSIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEBAUTHN_EXTENSIONS {
    fn eq(&self, other: &Self) -> bool {
        self.cExtensions == other.cExtensions && self.pExtensions == other.pExtensions
    }
}
impl ::core::cmp::Eq for WEBAUTHN_EXTENSIONS {}
impl ::core::fmt::Debug for WEBAUTHN_EXTENSIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_EXTENSIONS").field("cExtensions", &self.cExtensions).field("pExtensions", &self.pExtensions).finish()
    }
}
impl ::core::default::Default for WEBAUTHN_RP_ENTITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEBAUTHN_RP_ENTITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.pwszId == other.pwszId && self.pwszName == other.pwszName && self.pwszIcon == other.pwszIcon
    }
}
impl ::core::cmp::Eq for WEBAUTHN_RP_ENTITY_INFORMATION {}
impl ::core::fmt::Debug for WEBAUTHN_RP_ENTITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_RP_ENTITY_INFORMATION").field("dwVersion", &self.dwVersion).field("pwszId", &self.pwszId).field("pwszName", &self.pwszName).field("pwszIcon", &self.pwszIcon).finish()
    }
}
impl ::core::default::Default for WEBAUTHN_USER_ENTITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEBAUTHN_USER_ENTITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbId == other.cbId && self.pbId == other.pbId && self.pwszName == other.pwszName && self.pwszIcon == other.pwszIcon && self.pwszDisplayName == other.pwszDisplayName
    }
}
impl ::core::cmp::Eq for WEBAUTHN_USER_ENTITY_INFORMATION {}
impl ::core::fmt::Debug for WEBAUTHN_USER_ENTITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_USER_ENTITY_INFORMATION").field("dwVersion", &self.dwVersion).field("cbId", &self.cbId).field("pbId", &self.pbId).field("pwszName", &self.pwszName).field("pwszIcon", &self.pwszIcon).field("pwszDisplayName", &self.pwszDisplayName).finish()
    }
}
impl ::core::default::Default for WEBAUTHN_X5C {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WEBAUTHN_X5C {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for WEBAUTHN_X5C {}
impl ::core::fmt::Debug for WEBAUTHN_X5C {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEBAUTHN_X5C").field("cbData", &self.cbData).field("pbData", &self.pbData).finish()
    }
}
impl ::core::default::Default for WS_ADDRESSING_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_ADDRESSING_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ADDRESSING_VERSION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ANY_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ANY_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.localName == other.localName && self.ns == other.ns && self.value == other.value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ANY_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ANY_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ANY_ATTRIBUTE").field("localName", &self.localName).field("ns", &self.ns).field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ANY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ANY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.attributes == other.attributes && self.attributeCount == other.attributeCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ANY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ANY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ANY_ATTRIBUTES").field("attributes", &self.attributes).field("attributeCount", &self.attributeCount).finish()
    }
}
impl ::core::default::Default for WS_ASYNC_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_ASYNC_OPERATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_ASYNC_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_ASYNC_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.internal0 == other.internal0 && self.internal1 == other.internal1 && self.internal2 == other.internal2 && self.internal3 == other.internal3 && self.internal4 == other.internal4
    }
}
impl ::core::cmp::Eq for WS_ASYNC_STATE {}
impl ::core::fmt::Debug for WS_ASYNC_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ASYNC_STATE").field("internal0", &self.internal0).field("internal1", &self.internal1).field("internal2", &self.internal2).field("internal3", &self.internal3).field("internal4", &self.internal4).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ATTRIBUTE_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ATTRIBUTE_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.attributeLocalName == other.attributeLocalName && self.attributeNs == other.attributeNs && self.r#type == other.r#type && self.typeDescription == other.typeDescription
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ATTRIBUTE_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ATTRIBUTE_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ATTRIBUTE_DESCRIPTION").field("attributeLocalName", &self.attributeLocalName).field("attributeNs", &self.attributeNs).field("type", &self.r#type).field("typeDescription", &self.typeDescription).finish()
    }
}
impl ::core::default::Default for WS_BINDING_TEMPLATE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_BINDING_TEMPLATE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_BINDING_TEMPLATE_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_BOOL_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_BOOL_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_BOOL_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_BOOL_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_BOOL_DESCRIPTION").field("value", &self.value).finish()
    }
}
impl ::core::default::Default for WS_BUFFERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_BUFFERS {
    fn eq(&self, other: &Self) -> bool {
        self.bufferCount == other.bufferCount && self.buffers == other.buffers
    }
}
impl ::core::cmp::Eq for WS_BUFFERS {}
impl ::core::fmt::Debug for WS_BUFFERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_BUFFERS").field("bufferCount", &self.bufferCount).field("buffers", &self.buffers).finish()
    }
}
impl ::core::default::Default for WS_BYTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_BYTES {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.bytes == other.bytes
    }
}
impl ::core::cmp::Eq for WS_BYTES {}
impl ::core::fmt::Debug for WS_BYTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_BYTES").field("length", &self.length).field("bytes", &self.bytes).finish()
    }
}
impl ::core::default::Default for WS_BYTES_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_BYTES_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minByteCount == other.minByteCount && self.maxByteCount == other.maxByteCount
    }
}
impl ::core::cmp::Eq for WS_BYTES_DESCRIPTION {}
impl ::core::fmt::Debug for WS_BYTES_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_BYTES_DESCRIPTION").field("minByteCount", &self.minByteCount).field("maxByteCount", &self.maxByteCount).finish()
    }
}
impl ::core::default::Default for WS_BYTE_ARRAY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_BYTE_ARRAY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minByteCount == other.minByteCount && self.maxByteCount == other.maxByteCount
    }
}
impl ::core::cmp::Eq for WS_BYTE_ARRAY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_BYTE_ARRAY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_BYTE_ARRAY_DESCRIPTION").field("minByteCount", &self.minByteCount).field("maxByteCount", &self.maxByteCount).finish()
    }
}
impl ::core::default::Default for WS_CALLBACK_MODEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_CALLBACK_MODEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CALLBACK_MODEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_CALL_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_CALL_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_CALL_PROPERTY {}
impl ::core::fmt::Debug for WS_CALL_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CALL_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_CALL_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_CALL_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CALL_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.keyHandle == other.keyHandle && self.provider == other.provider && self.keySpec == other.keySpec
    }
}
impl ::core::cmp::Eq for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {}
impl ::core::fmt::Debug for WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CAPI_ASYMMETRIC_SECURITY_KEY_HANDLE").field("keyHandle", &self.keyHandle).field("provider", &self.provider).field("keySpec", &self.keySpec).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WS_CERTIFICATE_VALIDATION_CALLBACK_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_CERT_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_CERT_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credentialType == other.credentialType
    }
}
impl ::core::cmp::Eq for WS_CERT_CREDENTIAL {}
impl ::core::fmt::Debug for WS_CERT_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CERT_CREDENTIAL").field("credentialType", &self.credentialType).finish()
    }
}
impl ::core::default::Default for WS_CERT_CREDENTIAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_CERT_CREDENTIAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CERT_CREDENTIAL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_CERT_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_CERT_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity && self.rawCertificateData == other.rawCertificateData
    }
}
impl ::core::cmp::Eq for WS_CERT_ENDPOINT_IDENTITY {}
impl ::core::fmt::Debug for WS_CERT_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CERT_ENDPOINT_IDENTITY").field("identity", &self.identity).field("rawCertificateData", &self.rawCertificateData).finish()
    }
}
impl ::core::default::Default for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.bindingConstraint == other.bindingConstraint && self.bindingUsage == other.bindingUsage
    }
}
impl ::core::cmp::Eq for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::fmt::Debug for WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CERT_MESSAGE_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).field("bindingUsage", &self.bindingUsage).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WS_CERT_SIGNED_SAML_AUTHENTICATOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_CHANNEL_BINDING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_CHANNEL_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CHANNEL_BINDING").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_CHANNEL_DECODER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_CHANNEL_ENCODER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_CHANNEL_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_CHANNEL_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_CHANNEL_PROPERTIES {}
impl ::core::fmt::Debug for WS_CHANNEL_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHANNEL_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::core::default::Default for WS_CHANNEL_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_CHANNEL_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_CHANNEL_PROPERTY {}
impl ::core::fmt::Debug for WS_CHANNEL_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHANNEL_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_CHANNEL_PROPERTY_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_CHANNEL_PROPERTY_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.allowedValues == other.allowedValues && self.allowedValuesSize == other.allowedValuesSize && self.out == other.out
    }
}
impl ::core::cmp::Eq for WS_CHANNEL_PROPERTY_CONSTRAINT {}
impl ::core::fmt::Debug for WS_CHANNEL_PROPERTY_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHANNEL_PROPERTY_CONSTRAINT").field("id", &self.id).field("allowedValues", &self.allowedValues).field("allowedValuesSize", &self.allowedValuesSize).field("out", &self.out).finish()
    }
}
impl ::core::default::Default for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperty == other.channelProperty
    }
}
impl ::core::cmp::Eq for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {}
impl ::core::fmt::Debug for WS_CHANNEL_PROPERTY_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHANNEL_PROPERTY_CONSTRAINT_0").field("channelProperty", &self.channelProperty).finish()
    }
}
impl ::core::default::Default for WS_CHANNEL_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_CHANNEL_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CHANNEL_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_CHANNEL_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_CHANNEL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CHANNEL_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_CHANNEL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_CHANNEL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CHANNEL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_CHARSET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_CHARSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_CHARSET").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_CHAR_ARRAY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_CHAR_ARRAY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minCharCount == other.minCharCount && self.maxCharCount == other.maxCharCount
    }
}
impl ::core::cmp::Eq for WS_CHAR_ARRAY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_CHAR_ARRAY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CHAR_ARRAY_DESCRIPTION").field("minCharCount", &self.minCharCount).field("maxCharCount", &self.maxCharCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_CONTRACT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_CONTRACT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.operationCount == other.operationCount && self.operations == other.operations
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_CONTRACT_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_CONTRACT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CONTRACT_DESCRIPTION").field("operationCount", &self.operationCount).field("operations", &self.operations).finish()
    }
}
impl ::core::default::Default for WS_COOKIE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_COOKIE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_COOKIE_MODE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WS_CUSTOM_CERT_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_CUSTOM_CHANNEL_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_CUSTOM_HTTP_PROXY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_CUSTOM_HTTP_PROXY {
    fn eq(&self, other: &Self) -> bool {
        self.servers == other.servers && self.bypass == other.bypass
    }
}
impl ::core::cmp::Eq for WS_CUSTOM_HTTP_PROXY {}
impl ::core::fmt::Debug for WS_CUSTOM_HTTP_PROXY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_CUSTOM_HTTP_PROXY").field("servers", &self.servers).field("bypass", &self.bypass).finish()
    }
}
impl ::core::default::Default for WS_CUSTOM_LISTENER_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_CUSTOM_TYPE_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_DATETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_DATETIME {
    fn eq(&self, other: &Self) -> bool {
        self.ticks == other.ticks && self.format == other.format
    }
}
impl ::core::cmp::Eq for WS_DATETIME {}
impl ::core::fmt::Debug for WS_DATETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DATETIME").field("ticks", &self.ticks).field("format", &self.format).finish()
    }
}
impl ::core::default::Default for WS_DATETIME_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_DATETIME_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_DATETIME_DESCRIPTION {}
impl ::core::fmt::Debug for WS_DATETIME_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DATETIME_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::core::default::Default for WS_DATETIME_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_DATETIME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_DATETIME_FORMAT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_DECIMAL_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_DEFAULT_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_DEFAULT_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_DEFAULT_VALUE {}
impl ::core::fmt::Debug for WS_DEFAULT_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DEFAULT_VALUE").field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credential == other.credential
    }
}
impl ::core::cmp::Eq for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::fmt::Debug for WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DEFAULT_WINDOWS_INTEGRATED_AUTH_CREDENTIAL").field("credential", &self.credential).finish()
    }
}
impl ::core::default::Default for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {
    fn eq(&self, other: &Self) -> bool {
        self.subStringCount == other.subStringCount && self.subStrings == other.subStrings
    }
}
impl ::core::cmp::Eq for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {}
impl ::core::fmt::Debug for WS_DISALLOWED_USER_AGENT_SUBSTRINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DISALLOWED_USER_AGENT_SUBSTRINGS").field("subStringCount", &self.subStringCount).field("subStrings", &self.subStrings).finish()
    }
}
impl ::core::default::Default for WS_DNS_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_DNS_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity && self.dns == other.dns
    }
}
impl ::core::cmp::Eq for WS_DNS_ENDPOINT_IDENTITY {}
impl ::core::fmt::Debug for WS_DNS_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DNS_ENDPOINT_IDENTITY").field("identity", &self.identity).field("dns", &self.dns).finish()
    }
}
impl ::core::default::Default for WS_DOUBLE_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_DOUBLE_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_DOUBLE_DESCRIPTION {}
impl ::core::fmt::Debug for WS_DOUBLE_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DOUBLE_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_DURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_DURATION {
    fn eq(&self, other: &Self) -> bool {
        self.negative == other.negative && self.years == other.years && self.months == other.months && self.days == other.days && self.hours == other.hours && self.minutes == other.minutes && self.seconds == other.seconds && self.milliseconds == other.milliseconds && self.ticks == other.ticks
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_DURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_DURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_DURATION").field("negative", &self.negative).field("years", &self.years).field("months", &self.months).field("days", &self.days).field("hours", &self.hours).field("minutes", &self.minutes).field("seconds", &self.seconds).field("milliseconds", &self.milliseconds).field("ticks", &self.ticks).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_DURATION_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ELEMENT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ELEMENT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.elementLocalName == other.elementLocalName && self.elementNs == other.elementNs && self.r#type == other.r#type && self.typeDescription == other.typeDescription
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ELEMENT_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ELEMENT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ELEMENT_DESCRIPTION").field("elementLocalName", &self.elementLocalName).field("elementNs", &self.elementNs).field("type", &self.r#type).field("typeDescription", &self.typeDescription).finish()
    }
}
impl ::core::default::Default for WS_ENCODING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ENCODING").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_ENDPOINT_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_ENDPOINT_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.headers == other.headers && self.extensions == other.extensions && self.identity == other.identity
    }
}
impl ::core::cmp::Eq for WS_ENDPOINT_ADDRESS {}
impl ::core::fmt::Debug for WS_ENDPOINT_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENDPOINT_ADDRESS").field("url", &self.url).field("headers", &self.headers).field("extensions", &self.extensions).field("identity", &self.identity).finish()
    }
}
impl ::core::default::Default for WS_ENDPOINT_ADDRESS_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_ENDPOINT_ADDRESS_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.addressingVersion == other.addressingVersion
    }
}
impl ::core::cmp::Eq for WS_ENDPOINT_ADDRESS_DESCRIPTION {}
impl ::core::fmt::Debug for WS_ENDPOINT_ADDRESS_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENDPOINT_ADDRESS_DESCRIPTION").field("addressingVersion", &self.addressingVersion).finish()
    }
}
impl ::core::default::Default for WS_ENDPOINT_ADDRESS_EXTENSION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_ENDPOINT_ADDRESS_EXTENSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ENDPOINT_ADDRESS_EXTENSION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.identityType == other.identityType
    }
}
impl ::core::cmp::Eq for WS_ENDPOINT_IDENTITY {}
impl ::core::fmt::Debug for WS_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENDPOINT_IDENTITY").field("identityType", &self.identityType).finish()
    }
}
impl ::core::default::Default for WS_ENDPOINT_IDENTITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_ENDPOINT_IDENTITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ENDPOINT_IDENTITY_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ENDPOINT_POLICY_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ENDPOINT_POLICY_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.policyExtension == other.policyExtension && self.assertionName == other.assertionName && self.assertionNs == other.assertionNs && self.out == other.out
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ENDPOINT_POLICY_EXTENSION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ENDPOINT_POLICY_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENDPOINT_POLICY_EXTENSION").field("policyExtension", &self.policyExtension).field("assertionName", &self.assertionName).field("assertionNs", &self.assertionNs).field("out", &self.out).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ENDPOINT_POLICY_EXTENSION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ENDPOINT_POLICY_EXTENSION_0 {
    fn eq(&self, other: &Self) -> bool {
        self.assertionValue == other.assertionValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ENDPOINT_POLICY_EXTENSION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ENDPOINT_POLICY_EXTENSION_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENDPOINT_POLICY_EXTENSION_0").field("assertionValue", &self.assertionValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ENUM_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ENUM_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values && self.valueCount == other.valueCount && self.maxByteCount == other.maxByteCount && self.nameIndices == other.nameIndices
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ENUM_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ENUM_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENUM_DESCRIPTION").field("values", &self.values).field("valueCount", &self.valueCount).field("maxByteCount", &self.maxByteCount).field("nameIndices", &self.nameIndices).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ENUM_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ENUM_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.name == other.name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ENUM_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ENUM_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ENUM_VALUE").field("value", &self.value).field("name", &self.name).finish()
    }
}
impl ::core::default::Default for WS_ENVELOPE_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_ENVELOPE_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ENVELOPE_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_ERROR_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_ERROR_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_ERROR_PROPERTY {}
impl ::core::fmt::Debug for WS_ERROR_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ERROR_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_ERROR_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_ERROR_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_ERROR_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_EXCEPTION_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_EXCEPTION_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_EXCEPTION_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_EXTENDED_PROTECTION_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_EXTENDED_PROTECTION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_EXTENDED_PROTECTION_POLICY").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_EXTENDED_PROTECTION_SCENARIO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_EXTENDED_PROTECTION_SCENARIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_EXTENDED_PROTECTION_SCENARIO").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_FAULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_FAULT {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.reasons == other.reasons && self.reasonCount == other.reasonCount && self.actor == other.actor && self.node == other.node && self.detail == other.detail
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_FAULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_FAULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FAULT").field("code", &self.code).field("reasons", &self.reasons).field("reasonCount", &self.reasonCount).field("actor", &self.actor).field("node", &self.node).field("detail", &self.detail).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_FAULT_CODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_FAULT_CODE {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.subCode == other.subCode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_FAULT_CODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_FAULT_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FAULT_CODE").field("value", &self.value).field("subCode", &self.subCode).finish()
    }
}
impl ::core::default::Default for WS_FAULT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_FAULT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.envelopeVersion == other.envelopeVersion
    }
}
impl ::core::cmp::Eq for WS_FAULT_DESCRIPTION {}
impl ::core::fmt::Debug for WS_FAULT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FAULT_DESCRIPTION").field("envelopeVersion", &self.envelopeVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_FAULT_DETAIL_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_FAULT_DETAIL_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.action == other.action && self.detailElementDescription == other.detailElementDescription
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_FAULT_DETAIL_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_FAULT_DETAIL_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FAULT_DETAIL_DESCRIPTION").field("action", &self.action).field("detailElementDescription", &self.detailElementDescription).finish()
    }
}
impl ::core::default::Default for WS_FAULT_DISCLOSURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_FAULT_DISCLOSURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_FAULT_DISCLOSURE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_FAULT_ERROR_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_FAULT_ERROR_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_FAULT_ERROR_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_FAULT_REASON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_FAULT_REASON {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.lang == other.lang
    }
}
impl ::core::cmp::Eq for WS_FAULT_REASON {}
impl ::core::fmt::Debug for WS_FAULT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FAULT_REASON").field("text", &self.text).field("lang", &self.lang).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_FIELD_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_FIELD_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.mapping == other.mapping && self.localName == other.localName && self.ns == other.ns && self.r#type == other.r#type && self.typeDescription == other.typeDescription && self.offset == other.offset && self.options == other.options && self.defaultValue == other.defaultValue && self.countOffset == other.countOffset && self.itemLocalName == other.itemLocalName && self.itemNs == other.itemNs && self.itemRange == other.itemRange
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_FIELD_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_FIELD_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FIELD_DESCRIPTION").field("mapping", &self.mapping).field("localName", &self.localName).field("ns", &self.ns).field("type", &self.r#type).field("typeDescription", &self.typeDescription).field("offset", &self.offset).field("options", &self.options).field("defaultValue", &self.defaultValue).field("countOffset", &self.countOffset).field("itemLocalName", &self.itemLocalName).field("itemNs", &self.itemNs).field("itemRange", &self.itemRange).finish()
    }
}
impl ::core::default::Default for WS_FIELD_MAPPING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_FIELD_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_FIELD_MAPPING").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_FLOAT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_FLOAT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_FLOAT_DESCRIPTION {}
impl ::core::fmt::Debug for WS_FLOAT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_FLOAT_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::core::default::Default for WS_GUID_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_GUID_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_GUID_DESCRIPTION {}
impl ::core::fmt::Debug for WS_GUID_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_GUID_DESCRIPTION").field("value", &self.value).finish()
    }
}
impl ::core::default::Default for WS_HEADER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_HEADER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_HEADER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_HEAP_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HEAP_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_HEAP_PROPERTIES {}
impl ::core::fmt::Debug for WS_HEAP_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HEAP_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::core::default::Default for WS_HEAP_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HEAP_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_HEAP_PROPERTY {}
impl ::core::fmt::Debug for WS_HEAP_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HEAP_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_HEAP_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_HEAP_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_HEAP_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_HOST_NAMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HOST_NAMES {
    fn eq(&self, other: &Self) -> bool {
        self.hostNames == other.hostNames && self.hostNameCount == other.hostNameCount
    }
}
impl ::core::cmp::Eq for WS_HOST_NAMES {}
impl ::core::fmt::Debug for WS_HOST_NAMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HOST_NAMES").field("hostNames", &self.hostNames).field("hostNameCount", &self.hostNameCount).finish()
    }
}
impl ::core::default::Default for WS_HTTPS_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTPS_URL {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.host == other.host && self.port == other.port && self.portAsString == other.portAsString && self.path == other.path && self.query == other.query && self.fragment == other.fragment
    }
}
impl ::core::cmp::Eq for WS_HTTPS_URL {}
impl ::core::fmt::Debug for WS_HTTPS_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTPS_URL").field("url", &self.url).field("host", &self.host).field("port", &self.port).field("portAsString", &self.portAsString).field("path", &self.path).field("query", &self.query).field("fragment", &self.fragment).finish()
    }
}
impl ::core::default::Default for WS_HTTP_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties
    }
}
impl ::core::cmp::Eq for WS_HTTP_BINDING_TEMPLATE {}
impl ::core::fmt::Debug for WS_HTTP_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).finish()
    }
}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.httpHeaderAuthSecurityBinding == other.httpHeaderAuthSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("httpHeaderAuthSecurityBinding", &self.httpHeaderAuthSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.httpHeaderAuthSecurityBinding == other.httpHeaderAuthSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("httpHeaderAuthSecurityBinding", &self.httpHeaderAuthSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.clientCredential == other.clientCredential
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_SECURITY_BINDING").field("binding", &self.binding).field("clientCredential", &self.clientCredential).finish()
    }
}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.bindingConstraint == other.bindingConstraint
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).finish()
    }
}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).finish()
    }
}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties && self.clientCredential == other.clientCredential
    }
}
impl ::core::cmp::Eq for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_AUTH_SECURITY_BINDING_TEMPLATE").field("securityBindingProperties", &self.securityBindingProperties).field("clientCredential", &self.clientCredential).finish()
    }
}
impl ::core::default::Default for WS_HTTP_HEADER_AUTH_TARGET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_HTTP_HEADER_AUTH_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_HTTP_HEADER_AUTH_TARGET").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_HTTP_HEADER_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_HTTP_HEADER_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.headerName == other.headerName && self.headerMappingOptions == other.headerMappingOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_HTTP_HEADER_MAPPING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_HTTP_HEADER_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_HEADER_MAPPING").field("headerName", &self.headerName).field("headerMappingOptions", &self.headerMappingOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_HTTP_MESSAGE_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_HTTP_MESSAGE_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.requestMappingOptions == other.requestMappingOptions && self.responseMappingOptions == other.responseMappingOptions && self.requestHeaderMappings == other.requestHeaderMappings && self.requestHeaderMappingCount == other.requestHeaderMappingCount && self.responseHeaderMappings == other.responseHeaderMappings && self.responseHeaderMappingCount == other.responseHeaderMappingCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_HTTP_MESSAGE_MAPPING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_HTTP_MESSAGE_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_MESSAGE_MAPPING").field("requestMappingOptions", &self.requestMappingOptions).field("responseMappingOptions", &self.responseMappingOptions).field("requestHeaderMappings", &self.requestHeaderMappings).field("requestHeaderMappingCount", &self.requestHeaderMappingCount).field("responseHeaderMappings", &self.responseHeaderMappings).field("responseHeaderMappingCount", &self.responseHeaderMappingCount).finish()
    }
}
impl ::core::default::Default for WS_HTTP_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties
    }
}
impl ::core::cmp::Eq for WS_HTTP_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_HTTP_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).finish()
    }
}
impl ::core::default::Default for WS_HTTP_PROXY_SETTING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_HTTP_PROXY_SETTING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_HTTP_PROXY_SETTING_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_HTTP_REDIRECT_CALLBACK_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_HTTP_SSL_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_BINDING_TEMPLATE {}
impl ::core::fmt::Debug for WS_HTTP_SSL_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding && self.httpHeaderAuthSecurityBinding == other.httpHeaderAuthSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {}
impl ::core::fmt::Debug for WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_HEADER_AUTH_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("httpHeaderAuthSecurityBinding", &self.httpHeaderAuthSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding && self.httpHeaderAuthSecurityBinding == other.httpHeaderAuthSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_HEADER_AUTH_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("httpHeaderAuthSecurityBinding", &self.httpHeaderAuthSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding && self.kerberosApreqMessageSecurityBinding == other.kerberosApreqMessageSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {}
impl ::core::fmt::Debug for WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_KERBEROS_APREQ_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding && self.kerberosApreqMessageSecurityBinding == other.kerberosApreqMessageSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_KERBEROS_APREQ_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding && self.kerberosApreqMessageSecurityBinding == other.kerberosApreqMessageSecurityBinding && self.securityContextSecurityBinding == other.securityContextSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {}
impl ::core::fmt::Debug for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding && self.kerberosApreqMessageSecurityBinding == other.kerberosApreqMessageSecurityBinding && self.securityContextSecurityBinding == other.securityContextSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_HTTP_SSL_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_HTTP_SSL_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_HTTP_SSL_USERNAME_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding && self.usernameMessageSecurityBinding == other.usernameMessageSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_USERNAME_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("usernameMessageSecurityBinding", &self.usernameMessageSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sslTransportSecurityBinding == other.sslTransportSecurityBinding && self.usernameMessageSecurityBinding == other.usernameMessageSecurityBinding && self.securityContextSecurityBinding == other.securityContextSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_SSL_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sslTransportSecurityBinding", &self.sslTransportSecurityBinding).field("usernameMessageSecurityBinding", &self.usernameMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_HTTP_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_HTTP_URL {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.host == other.host && self.port == other.port && self.portAsString == other.portAsString && self.path == other.path && self.query == other.query && self.fragment == other.fragment
    }
}
impl ::core::cmp::Eq for WS_HTTP_URL {}
impl ::core::fmt::Debug for WS_HTTP_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_HTTP_URL").field("url", &self.url).field("host", &self.host).field("port", &self.port).field("portAsString", &self.portAsString).field("path", &self.path).field("query", &self.query).field("fragment", &self.fragment).finish()
    }
}
impl ::core::default::Default for WS_INT16_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_INT16_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_INT16_DESCRIPTION {}
impl ::core::fmt::Debug for WS_INT16_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_INT16_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::core::default::Default for WS_INT32_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_INT32_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_INT32_DESCRIPTION {}
impl ::core::fmt::Debug for WS_INT32_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_INT32_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::core::default::Default for WS_INT64_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_INT64_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_INT64_DESCRIPTION {}
impl ::core::fmt::Debug for WS_INT64_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_INT64_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_INT8_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_INT8_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_INT8_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_INT8_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_INT8_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::core::default::Default for WS_IP_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_IP_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_IP_VERSION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.bindingConstraint == other.bindingConstraint && self.bindingUsage == other.bindingUsage && self.claimConstraints == other.claimConstraints && self.claimConstraintCount == other.claimConstraintCount && self.requestSecurityTokenPropertyConstraints == other.requestSecurityTokenPropertyConstraints && self.requestSecurityTokenPropertyConstraintCount == other.requestSecurityTokenPropertyConstraintCount && self.out == other.out
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT")
            .field("bindingConstraint", &self.bindingConstraint)
            .field("bindingUsage", &self.bindingUsage)
            .field("claimConstraints", &self.claimConstraints)
            .field("claimConstraintCount", &self.claimConstraintCount)
            .field("requestSecurityTokenPropertyConstraints", &self.requestSecurityTokenPropertyConstraints)
            .field("requestSecurityTokenPropertyConstraintCount", &self.requestSecurityTokenPropertyConstraintCount)
            .field("out", &self.out)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.issuerAddress == other.issuerAddress && self.requestSecurityTokenTemplate == other.requestSecurityTokenTemplate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ISSUED_TOKEN_MESSAGE_SECURITY_BINDING_CONSTRAINT_0").field("issuerAddress", &self.issuerAddress).field("requestSecurityTokenTemplate", &self.requestSecurityTokenTemplate).finish()
    }
}
impl ::core::default::Default for WS_ITEM_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_ITEM_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.minItemCount == other.minItemCount && self.maxItemCount == other.maxItemCount
    }
}
impl ::core::cmp::Eq for WS_ITEM_RANGE {}
impl ::core::fmt::Debug for WS_ITEM_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_ITEM_RANGE").field("minItemCount", &self.minItemCount).field("maxItemCount", &self.maxItemCount).finish()
    }
}
impl ::core::default::Default for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.bindingUsage == other.bindingUsage && self.clientCredential == other.clientCredential
    }
}
impl ::core::cmp::Eq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {}
impl ::core::fmt::Debug for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING").field("binding", &self.binding).field("bindingUsage", &self.bindingUsage).field("clientCredential", &self.clientCredential).finish()
    }
}
impl ::core::default::Default for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.bindingConstraint == other.bindingConstraint && self.bindingUsage == other.bindingUsage
    }
}
impl ::core::cmp::Eq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::fmt::Debug for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).field("bindingUsage", &self.bindingUsage).finish()
    }
}
impl ::core::default::Default for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties && self.bindingUsage == other.bindingUsage
    }
}
impl ::core::cmp::Eq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).field("bindingUsage", &self.bindingUsage).finish()
    }
}
impl ::core::default::Default for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties && self.clientCredential == other.clientCredential
    }
}
impl ::core::cmp::Eq for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {}
impl ::core::fmt::Debug for WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_KERBEROS_APREQ_MESSAGE_SECURITY_BINDING_TEMPLATE").field("securityBindingProperties", &self.securityBindingProperties).field("clientCredential", &self.clientCredential).finish()
    }
}
impl ::core::default::Default for WS_LISTENER_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_LISTENER_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_LISTENER_PROPERTIES {}
impl ::core::fmt::Debug for WS_LISTENER_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_LISTENER_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::core::default::Default for WS_LISTENER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_LISTENER_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_LISTENER_PROPERTY {}
impl ::core::fmt::Debug for WS_LISTENER_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_LISTENER_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_LISTENER_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_LISTENER_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_LISTENER_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_LISTENER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_LISTENER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_LISTENER_STATE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_MESSAGE_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_MESSAGE_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.action == other.action && self.bodyElementDescription == other.bodyElementDescription
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_MESSAGE_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_MESSAGE_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_MESSAGE_DESCRIPTION").field("action", &self.action).field("bodyElementDescription", &self.bodyElementDescription).finish()
    }
}
impl ::core::default::Default for WS_MESSAGE_INITIALIZATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_MESSAGE_INITIALIZATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_MESSAGE_INITIALIZATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_MESSAGE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_MESSAGE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_MESSAGE_PROPERTIES {}
impl ::core::fmt::Debug for WS_MESSAGE_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_MESSAGE_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::core::default::Default for WS_MESSAGE_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_MESSAGE_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_MESSAGE_PROPERTY {}
impl ::core::fmt::Debug for WS_MESSAGE_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_MESSAGE_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_MESSAGE_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_MESSAGE_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_MESSAGE_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_MESSAGE_SECURITY_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_MESSAGE_SECURITY_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_MESSAGE_SECURITY_USAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_MESSAGE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_MESSAGE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_MESSAGE_STATE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_METADATA_ENDPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_METADATA_ENDPOINT {
    fn eq(&self, other: &Self) -> bool {
        self.endpointAddress == other.endpointAddress && self.endpointPolicy == other.endpointPolicy && self.portName == other.portName && self.serviceName == other.serviceName && self.serviceNs == other.serviceNs && self.bindingName == other.bindingName && self.bindingNs == other.bindingNs && self.portTypeName == other.portTypeName && self.portTypeNs == other.portTypeNs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_METADATA_ENDPOINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_METADATA_ENDPOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_METADATA_ENDPOINT").field("endpointAddress", &self.endpointAddress).field("endpointPolicy", &self.endpointPolicy).field("portName", &self.portName).field("serviceName", &self.serviceName).field("serviceNs", &self.serviceNs).field("bindingName", &self.bindingName).field("bindingNs", &self.bindingNs).field("portTypeName", &self.portTypeName).field("portTypeNs", &self.portTypeNs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_METADATA_ENDPOINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_METADATA_ENDPOINTS {
    fn eq(&self, other: &Self) -> bool {
        self.endpoints == other.endpoints && self.endpointCount == other.endpointCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_METADATA_ENDPOINTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_METADATA_ENDPOINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_METADATA_ENDPOINTS").field("endpoints", &self.endpoints).field("endpointCount", &self.endpointCount).finish()
    }
}
impl ::core::default::Default for WS_METADATA_EXCHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_METADATA_EXCHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_METADATA_EXCHANGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_METADATA_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_METADATA_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_METADATA_PROPERTY {}
impl ::core::fmt::Debug for WS_METADATA_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_METADATA_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_METADATA_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_METADATA_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_METADATA_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_METADATA_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_METADATA_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_METADATA_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_MOVE_TO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_MOVE_TO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_MOVE_TO").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.clientCredential == other.clientCredential
    }
}
impl ::core::cmp::Eq for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {}
impl ::core::fmt::Debug for WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_NAMEDPIPE_SSPI_TRANSPORT_SECURITY_BINDING").field("binding", &self.binding).field("clientCredential", &self.clientCredential).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.keyHandle == other.keyHandle && self.asymmetricKey == other.asymmetricKey
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_NCRYPT_ASYMMETRIC_SECURITY_KEY_HANDLE").field("keyHandle", &self.keyHandle).field("asymmetricKey", &self.asymmetricKey).finish()
    }
}
impl ::core::default::Default for WS_NETPIPE_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_NETPIPE_URL {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.host == other.host && self.port == other.port && self.portAsString == other.portAsString && self.path == other.path && self.query == other.query && self.fragment == other.fragment
    }
}
impl ::core::cmp::Eq for WS_NETPIPE_URL {}
impl ::core::fmt::Debug for WS_NETPIPE_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_NETPIPE_URL").field("url", &self.url).field("host", &self.host).field("port", &self.port).field("portAsString", &self.portAsString).field("path", &self.path).field("query", &self.query).field("fragment", &self.fragment).finish()
    }
}
impl ::core::default::Default for WS_NETTCP_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_NETTCP_URL {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.host == other.host && self.port == other.port && self.portAsString == other.portAsString && self.path == other.path && self.query == other.query && self.fragment == other.fragment
    }
}
impl ::core::cmp::Eq for WS_NETTCP_URL {}
impl ::core::fmt::Debug for WS_NETTCP_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_NETTCP_URL").field("url", &self.url).field("host", &self.host).field("port", &self.port).field("portAsString", &self.portAsString).field("path", &self.path).field("query", &self.query).field("fragment", &self.fragment).finish()
    }
}
impl ::core::default::Default for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credential == other.credential && self.opaqueAuthIdentity == other.opaqueAuthIdentity
    }
}
impl ::core::cmp::Eq for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::fmt::Debug for WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_OPAQUE_WINDOWS_INTEGRATED_AUTH_CREDENTIAL").field("credential", &self.credential).field("opaqueAuthIdentity", &self.opaqueAuthIdentity).finish()
    }
}
impl ::core::default::Default for WS_OPERATION_CONTEXT_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_OPERATION_CONTEXT_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_OPERATION_CONTEXT_PROPERTY_ID").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_OPERATION_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_OPERATION_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_OPERATION_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_OPERATION_STYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_PARAMETER_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_PARAMETER_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.parameterType == other.parameterType && self.inputMessageIndex == other.inputMessageIndex && self.outputMessageIndex == other.outputMessageIndex
    }
}
impl ::core::cmp::Eq for WS_PARAMETER_DESCRIPTION {}
impl ::core::fmt::Debug for WS_PARAMETER_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_PARAMETER_DESCRIPTION").field("parameterType", &self.parameterType).field("inputMessageIndex", &self.inputMessageIndex).field("outputMessageIndex", &self.outputMessageIndex).finish()
    }
}
impl ::core::default::Default for WS_PARAMETER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_PARAMETER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_PARAMETER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_POLICY_CONSTRAINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_POLICY_CONSTRAINTS {
    fn eq(&self, other: &Self) -> bool {
        self.channelBinding == other.channelBinding && self.channelPropertyConstraints == other.channelPropertyConstraints && self.channelPropertyConstraintCount == other.channelPropertyConstraintCount && self.securityConstraints == other.securityConstraints && self.policyExtensions == other.policyExtensions && self.policyExtensionCount == other.policyExtensionCount
    }
}
impl ::core::cmp::Eq for WS_POLICY_CONSTRAINTS {}
impl ::core::fmt::Debug for WS_POLICY_CONSTRAINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_POLICY_CONSTRAINTS").field("channelBinding", &self.channelBinding).field("channelPropertyConstraints", &self.channelPropertyConstraints).field("channelPropertyConstraintCount", &self.channelPropertyConstraintCount).field("securityConstraints", &self.securityConstraints).field("policyExtensions", &self.policyExtensions).field("policyExtensionCount", &self.policyExtensionCount).finish()
    }
}
impl ::core::default::Default for WS_POLICY_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_POLICY_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type
    }
}
impl ::core::cmp::Eq for WS_POLICY_EXTENSION {}
impl ::core::fmt::Debug for WS_POLICY_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_POLICY_EXTENSION").field("type", &self.r#type).finish()
    }
}
impl ::core::default::Default for WS_POLICY_EXTENSION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_POLICY_EXTENSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_POLICY_EXTENSION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_POLICY_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_POLICY_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_POLICY_PROPERTIES {}
impl ::core::fmt::Debug for WS_POLICY_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_POLICY_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::core::default::Default for WS_POLICY_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_POLICY_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_POLICY_PROPERTY {}
impl ::core::fmt::Debug for WS_POLICY_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_POLICY_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_POLICY_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_POLICY_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_POLICY_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_POLICY_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_POLICY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_POLICY_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_PROTECTION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_PROTECTION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_PROTECTION_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_PROXY_MESSAGE_CALLBACK_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_PROXY_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_PROXY_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_PROXY_PROPERTY {}
impl ::core::fmt::Debug for WS_PROXY_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_PROXY_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_PROXY_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_PROXY_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_PROXY_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.keyHandle == other.keyHandle && self.rawKeyBytes == other.rawKeyBytes
    }
}
impl ::core::cmp::Eq for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {}
impl ::core::fmt::Debug for WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_RAW_SYMMETRIC_SECURITY_KEY_HANDLE").field("keyHandle", &self.keyHandle).field("rawKeyBytes", &self.rawKeyBytes).finish()
    }
}
impl ::core::default::Default for WS_READ_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_READ_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_READ_OPTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_RECEIVE_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_RECEIVE_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_RECEIVE_OPTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_REPEATING_HEADER_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_REPEATING_HEADER_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_REPEATING_HEADER_OPTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_REQUEST_SECURITY_TOKEN_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_REQUEST_SECURITY_TOKEN_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_REQUEST_SECURITY_TOKEN_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_REQUEST_SECURITY_TOKEN_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_REQUEST_SECURITY_TOKEN_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_REQUEST_SECURITY_TOKEN_PROPERTY {}
impl ::core::fmt::Debug for WS_REQUEST_SECURITY_TOKEN_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_REQUEST_SECURITY_TOKEN_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.allowedValues == other.allowedValues && self.allowedValuesSize == other.allowedValuesSize && self.out == other.out
    }
}
impl ::core::cmp::Eq for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {}
impl ::core::fmt::Debug for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT").field("id", &self.id).field("allowedValues", &self.allowedValues).field("allowedValuesSize", &self.allowedValuesSize).field("out", &self.out).finish()
    }
}
impl ::core::default::Default for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.requestSecurityTokenProperty == other.requestSecurityTokenProperty
    }
}
impl ::core::cmp::Eq for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {}
impl ::core::fmt::Debug for WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_REQUEST_SECURITY_TOKEN_PROPERTY_CONSTRAINT_0").field("requestSecurityTokenProperty", &self.requestSecurityTokenProperty).finish()
    }
}
impl ::core::default::Default for WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_REQUEST_SECURITY_TOKEN_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_RSA_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_RSA_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity && self.modulus == other.modulus && self.exponent == other.exponent
    }
}
impl ::core::cmp::Eq for WS_RSA_ENDPOINT_IDENTITY {}
impl ::core::fmt::Debug for WS_RSA_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_RSA_ENDPOINT_IDENTITY").field("identity", &self.identity).field("modulus", &self.modulus).field("exponent", &self.exponent).finish()
    }
}
impl ::core::default::Default for WS_SAML_AUTHENTICATOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SAML_AUTHENTICATOR {
    fn eq(&self, other: &Self) -> bool {
        self.authenticatorType == other.authenticatorType
    }
}
impl ::core::cmp::Eq for WS_SAML_AUTHENTICATOR {}
impl ::core::fmt::Debug for WS_SAML_AUTHENTICATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SAML_AUTHENTICATOR").field("authenticatorType", &self.authenticatorType).finish()
    }
}
impl ::core::default::Default for WS_SAML_AUTHENTICATOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SAML_AUTHENTICATOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SAML_AUTHENTICATOR_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SAML_MESSAGE_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SAML_MESSAGE_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.bindingUsage == other.bindingUsage && self.authenticator == other.authenticator
    }
}
impl ::core::cmp::Eq for WS_SAML_MESSAGE_SECURITY_BINDING {}
impl ::core::fmt::Debug for WS_SAML_MESSAGE_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SAML_MESSAGE_SECURITY_BINDING").field("binding", &self.binding).field("bindingUsage", &self.bindingUsage).field("authenticator", &self.authenticator).finish()
    }
}
impl ::core::default::Default for WS_SECURE_CONVERSATION_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURE_CONVERSATION_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURE_CONVERSATION_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURE_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURE_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURE_PROTOCOL").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_ALGORITHM_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURITY_ALGORITHM_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_ALGORITHM_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_ALGORITHM_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_ALGORITHM_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_SECURITY_ALGORITHM_PROPERTY {}
impl ::core::fmt::Debug for WS_SECURITY_ALGORITHM_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_ALGORITHM_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_ALGORITHM_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURITY_ALGORITHM_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_ALGORITHM_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_ALGORITHM_SUITE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_ALGORITHM_SUITE {
    fn eq(&self, other: &Self) -> bool {
        self.canonicalizationAlgorithm == other.canonicalizationAlgorithm
            && self.digestAlgorithm == other.digestAlgorithm
            && self.symmetricSignatureAlgorithm == other.symmetricSignatureAlgorithm
            && self.asymmetricSignatureAlgorithm == other.asymmetricSignatureAlgorithm
            && self.encryptionAlgorithm == other.encryptionAlgorithm
            && self.keyDerivationAlgorithm == other.keyDerivationAlgorithm
            && self.symmetricKeyWrapAlgorithm == other.symmetricKeyWrapAlgorithm
            && self.asymmetricKeyWrapAlgorithm == other.asymmetricKeyWrapAlgorithm
            && self.minSymmetricKeyLength == other.minSymmetricKeyLength
            && self.maxSymmetricKeyLength == other.maxSymmetricKeyLength
            && self.minAsymmetricKeyLength == other.minAsymmetricKeyLength
            && self.maxAsymmetricKeyLength == other.maxAsymmetricKeyLength
            && self.properties == other.properties
            && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_SECURITY_ALGORITHM_SUITE {}
impl ::core::fmt::Debug for WS_SECURITY_ALGORITHM_SUITE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_ALGORITHM_SUITE")
            .field("canonicalizationAlgorithm", &self.canonicalizationAlgorithm)
            .field("digestAlgorithm", &self.digestAlgorithm)
            .field("symmetricSignatureAlgorithm", &self.symmetricSignatureAlgorithm)
            .field("asymmetricSignatureAlgorithm", &self.asymmetricSignatureAlgorithm)
            .field("encryptionAlgorithm", &self.encryptionAlgorithm)
            .field("keyDerivationAlgorithm", &self.keyDerivationAlgorithm)
            .field("symmetricKeyWrapAlgorithm", &self.symmetricKeyWrapAlgorithm)
            .field("asymmetricKeyWrapAlgorithm", &self.asymmetricKeyWrapAlgorithm)
            .field("minSymmetricKeyLength", &self.minSymmetricKeyLength)
            .field("maxSymmetricKeyLength", &self.maxSymmetricKeyLength)
            .field("minAsymmetricKeyLength", &self.minAsymmetricKeyLength)
            .field("maxAsymmetricKeyLength", &self.maxAsymmetricKeyLength)
            .field("properties", &self.properties)
            .field("propertyCount", &self.propertyCount)
            .finish()
    }
}
impl ::core::default::Default for WS_SECURITY_ALGORITHM_SUITE_NAME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURITY_ALGORITHM_SUITE_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_ALGORITHM_SUITE_NAME").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_BEARER_KEY_TYPE_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURITY_BEARER_KEY_TYPE_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_BEARER_KEY_TYPE_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.bindingType == other.bindingType && self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING {}
impl ::core::fmt::Debug for WS_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING").field("bindingType", &self.bindingType).field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.propertyConstraints == other.propertyConstraints && self.propertyConstraintCount == other.propertyConstraintCount
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING_CONSTRAINT {}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING_CONSTRAINT").field("type", &self.r#type).field("propertyConstraints", &self.propertyConstraints).field("propertyConstraintCount", &self.propertyConstraintCount).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_BINDING_CONSTRAINT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_CONSTRAINT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_BINDING_CONSTRAINT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_BINDING_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING_PROPERTIES {}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_BINDING_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING_PROPERTY {}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.allowedValues == other.allowedValues && self.allowedValuesSize == other.allowedValuesSize && self.out == other.out
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING_PROPERTY_CONSTRAINT").field("id", &self.id).field("allowedValues", &self.allowedValues).field("allowedValuesSize", &self.allowedValuesSize).field("out", &self.out).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperty == other.securityBindingProperty
    }
}
impl ::core::cmp::Eq for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_BINDING_PROPERTY_CONSTRAINT_0").field("securityBindingProperty", &self.securityBindingProperty).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_BINDING_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_BINDING_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_BINDING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURITY_BINDING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_BINDING_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_CONSTRAINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONSTRAINTS {
    fn eq(&self, other: &Self) -> bool {
        self.securityPropertyConstraints == other.securityPropertyConstraints && self.securityPropertyConstraintCount == other.securityPropertyConstraintCount && self.securityBindingConstraints == other.securityBindingConstraints && self.securityBindingConstraintCount == other.securityBindingConstraintCount
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONSTRAINTS {}
impl ::core::fmt::Debug for WS_SECURITY_CONSTRAINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONSTRAINTS").field("securityPropertyConstraints", &self.securityPropertyConstraints).field("securityPropertyConstraintCount", &self.securityPropertyConstraintCount).field("securityBindingConstraints", &self.securityBindingConstraints).field("securityBindingConstraintCount", &self.securityBindingConstraintCount).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.bindingUsage == other.bindingUsage && self.bootstrapSecurityDescription == other.bootstrapSecurityDescription
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING").field("binding", &self.binding).field("bindingUsage", &self.bindingUsage).field("bootstrapSecurityDescription", &self.bootstrapSecurityDescription).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.bindingConstraint == other.bindingConstraint && self.bindingUsage == other.bindingUsage && self.bootstrapSecurityConstraint == other.bootstrapSecurityConstraint
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).field("bindingUsage", &self.bindingUsage).field("bootstrapSecurityConstraint", &self.bootstrapSecurityConstraint).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties && self.bindingUsage == other.bindingUsage
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).field("bindingUsage", &self.bindingUsage).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_MESSAGE_SECURITY_BINDING_TEMPLATE").field("securityBindingProperties", &self.securityBindingProperties).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_CONTEXT_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_PROPERTY {}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_CONTEXT_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_CONTEXT_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.securityContextMessageSecurityBinding == other.securityContextMessageSecurityBinding && self.securityProperties == other.securityProperties
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityContextMessageSecurityBinding", &self.securityContextMessageSecurityBinding).field("securityProperties", &self.securityProperties).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.securityContextMessageSecurityBinding == other.securityContextMessageSecurityBinding && self.securityProperties == other.securityProperties
    }
}
impl ::core::cmp::Eq for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {}
impl ::core::fmt::Debug for WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_CONTEXT_SECURITY_BINDING_TEMPLATE").field("securityContextMessageSecurityBinding", &self.securityContextMessageSecurityBinding).field("securityProperties", &self.securityProperties).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindings == other.securityBindings && self.securityBindingCount == other.securityBindingCount && self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_SECURITY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_SECURITY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_DESCRIPTION").field("securityBindings", &self.securityBindings).field("securityBindingCount", &self.securityBindingCount).field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_HEADER_LAYOUT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURITY_HEADER_LAYOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_HEADER_LAYOUT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_HEADER_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURITY_HEADER_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_HEADER_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_KEY_ENTROPY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURITY_KEY_ENTROPY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_KEY_ENTROPY_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_KEY_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_KEY_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.keyHandleType == other.keyHandleType
    }
}
impl ::core::cmp::Eq for WS_SECURITY_KEY_HANDLE {}
impl ::core::fmt::Debug for WS_SECURITY_KEY_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_KEY_HANDLE").field("keyHandleType", &self.keyHandleType).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_KEY_HANDLE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURITY_KEY_HANDLE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_KEY_HANDLE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_KEY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURITY_KEY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_KEY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_SECURITY_PROPERTIES {}
impl ::core::fmt::Debug for WS_SECURITY_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_SECURITY_PROPERTY {}
impl ::core::fmt::Debug for WS_SECURITY_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_PROPERTY_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_PROPERTY_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.allowedValues == other.allowedValues && self.allowedValuesSize == other.allowedValuesSize && self.out == other.out
    }
}
impl ::core::cmp::Eq for WS_SECURITY_PROPERTY_CONSTRAINT {}
impl ::core::fmt::Debug for WS_SECURITY_PROPERTY_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_PROPERTY_CONSTRAINT").field("id", &self.id).field("allowedValues", &self.allowedValues).field("allowedValuesSize", &self.allowedValuesSize).field("out", &self.out).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_PROPERTY_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SECURITY_PROPERTY_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.securityProperty == other.securityProperty
    }
}
impl ::core::cmp::Eq for WS_SECURITY_PROPERTY_CONSTRAINT_0 {}
impl ::core::fmt::Debug for WS_SECURITY_PROPERTY_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SECURITY_PROPERTY_CONSTRAINT_0").field("securityProperty", &self.securityProperty).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURITY_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_TIMESTAMP_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURITY_TIMESTAMP_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_TIMESTAMP_USAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_TOKEN_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURITY_TOKEN_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_TOKEN_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SECURITY_TOKEN_REFERENCE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SECURITY_TOKEN_REFERENCE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SECURITY_TOKEN_REFERENCE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SERVICE_CANCEL_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SERVICE_CANCEL_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SERVICE_CANCEL_REASON").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SERVICE_CONTRACT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SERVICE_ENDPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SERVICE_ENDPOINT_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_SERVICE_ENDPOINT_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.portName == other.portName && self.bindingName == other.bindingName && self.bindingNs == other.bindingNs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_SERVICE_ENDPOINT_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SERVICE_ENDPOINT_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_ENDPOINT_METADATA").field("portName", &self.portName).field("bindingName", &self.bindingName).field("bindingNs", &self.bindingNs).finish()
    }
}
impl ::core::default::Default for WS_SERVICE_ENDPOINT_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SERVICE_ENDPOINT_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_SERVICE_ENDPOINT_PROPERTY {}
impl ::core::fmt::Debug for WS_SERVICE_ENDPOINT_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_ENDPOINT_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_SERVICE_ENDPOINT_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SERVICE_ENDPOINT_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SERVICE_ENDPOINT_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SERVICE_HOST_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SERVICE_HOST_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SERVICE_HOST_STATE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SERVICE_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_SERVICE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.documentCount == other.documentCount && self.documents == other.documents && self.serviceName == other.serviceName && self.serviceNs == other.serviceNs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_SERVICE_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SERVICE_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_METADATA").field("documentCount", &self.documentCount).field("documents", &self.documents).field("serviceName", &self.serviceName).field("serviceNs", &self.serviceNs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SERVICE_METADATA_DOCUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_SERVICE_METADATA_DOCUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content && self.name == other.name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_SERVICE_METADATA_DOCUMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SERVICE_METADATA_DOCUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_METADATA_DOCUMENT").field("content", &self.content).field("name", &self.name).finish()
    }
}
impl ::core::default::Default for WS_SERVICE_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SERVICE_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_SERVICE_PROPERTY {}
impl ::core::fmt::Debug for WS_SERVICE_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_SERVICE_PROPERTY_ACCEPT_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_SERVICE_PROPERTY_CLOSE_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_SERVICE_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SERVICE_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SERVICE_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SERVICE_PROXY_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_SERVICE_PROXY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_SERVICE_PROXY_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_SERVICE_SECURITY_IDENTITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SERVICE_SECURITY_IDENTITIES {
    fn eq(&self, other: &Self) -> bool {
        self.serviceIdentities == other.serviceIdentities && self.serviceIdentityCount == other.serviceIdentityCount
    }
}
impl ::core::cmp::Eq for WS_SERVICE_SECURITY_IDENTITIES {}
impl ::core::fmt::Debug for WS_SERVICE_SECURITY_IDENTITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SERVICE_SECURITY_IDENTITIES").field("serviceIdentities", &self.serviceIdentities).field("serviceIdentityCount", &self.serviceIdentityCount).finish()
    }
}
impl ::core::default::Default for WS_SOAPUDP_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SOAPUDP_URL {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.host == other.host && self.port == other.port && self.portAsString == other.portAsString && self.path == other.path && self.query == other.query && self.fragment == other.fragment
    }
}
impl ::core::cmp::Eq for WS_SOAPUDP_URL {}
impl ::core::fmt::Debug for WS_SOAPUDP_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SOAPUDP_URL").field("url", &self.url).field("host", &self.host).field("port", &self.port).field("portAsString", &self.portAsString).field("path", &self.path).field("query", &self.query).field("fragment", &self.fragment).finish()
    }
}
impl ::core::default::Default for WS_SPN_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SPN_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity && self.spn == other.spn
    }
}
impl ::core::cmp::Eq for WS_SPN_ENDPOINT_IDENTITY {}
impl ::core::fmt::Debug for WS_SPN_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SPN_ENDPOINT_IDENTITY").field("identity", &self.identity).field("spn", &self.spn).finish()
    }
}
impl ::core::default::Default for WS_SSL_TRANSPORT_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SSL_TRANSPORT_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.localCertCredential == other.localCertCredential
    }
}
impl ::core::cmp::Eq for WS_SSL_TRANSPORT_SECURITY_BINDING {}
impl ::core::fmt::Debug for WS_SSL_TRANSPORT_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSL_TRANSPORT_SECURITY_BINDING").field("binding", &self.binding).field("localCertCredential", &self.localCertCredential).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.bindingConstraint == other.bindingConstraint && self.out == other.out
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).field("out", &self.out).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.clientCertCredentialRequired == other.clientCertCredentialRequired
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSL_TRANSPORT_SECURITY_BINDING_CONSTRAINT_0").field("clientCertCredentialRequired", &self.clientCertCredentialRequired).finish()
    }
}
impl ::core::default::Default for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties
    }
}
impl ::core::cmp::Eq for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSL_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).finish()
    }
}
impl ::core::default::Default for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties && self.localCertCredential == other.localCertCredential
    }
}
impl ::core::cmp::Eq for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {}
impl ::core::fmt::Debug for WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSL_TRANSPORT_SECURITY_BINDING_TEMPLATE").field("securityBindingProperties", &self.securityBindingProperties).field("localCertCredential", &self.localCertCredential).finish()
    }
}
impl ::core::default::Default for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties
    }
}
impl ::core::cmp::Eq for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SSPI_TRANSPORT_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).finish()
    }
}
impl ::core::default::Default for WS_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.chars == other.chars
    }
}
impl ::core::cmp::Eq for WS_STRING {}
impl ::core::fmt::Debug for WS_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_STRING").field("length", &self.length).field("chars", &self.chars).finish()
    }
}
impl ::core::default::Default for WS_STRING_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_STRING_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minCharCount == other.minCharCount && self.maxCharCount == other.maxCharCount
    }
}
impl ::core::cmp::Eq for WS_STRING_DESCRIPTION {}
impl ::core::fmt::Debug for WS_STRING_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_STRING_DESCRIPTION").field("minCharCount", &self.minCharCount).field("maxCharCount", &self.maxCharCount).finish()
    }
}
impl ::core::default::Default for WS_STRING_USERNAME_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_STRING_USERNAME_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credential == other.credential && self.username == other.username && self.password == other.password
    }
}
impl ::core::cmp::Eq for WS_STRING_USERNAME_CREDENTIAL {}
impl ::core::fmt::Debug for WS_STRING_USERNAME_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_STRING_USERNAME_CREDENTIAL").field("credential", &self.credential).field("username", &self.username).field("password", &self.password).finish()
    }
}
impl ::core::default::Default for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credential == other.credential && self.username == other.username && self.password == other.password && self.domain == other.domain
    }
}
impl ::core::cmp::Eq for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::fmt::Debug for WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_STRING_WINDOWS_INTEGRATED_AUTH_CREDENTIAL").field("credential", &self.credential).field("username", &self.username).field("password", &self.password).field("domain", &self.domain).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_STRUCT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_STRUCT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.alignment == other.alignment && self.fields == other.fields && self.fieldCount == other.fieldCount && self.typeLocalName == other.typeLocalName && self.typeNs == other.typeNs && self.parentType == other.parentType && self.subTypes == other.subTypes && self.subTypeCount == other.subTypeCount && self.structOptions == other.structOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_STRUCT_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_STRUCT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_STRUCT_DESCRIPTION").field("size", &self.size).field("alignment", &self.alignment).field("fields", &self.fields).field("fieldCount", &self.fieldCount).field("typeLocalName", &self.typeLocalName).field("typeNs", &self.typeNs).field("parentType", &self.parentType).field("subTypes", &self.subTypes).field("subTypeCount", &self.subTypeCount).field("structOptions", &self.structOptions).finish()
    }
}
impl ::core::default::Default for WS_SUBJECT_NAME_CERT_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_SUBJECT_NAME_CERT_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credential == other.credential && self.storeLocation == other.storeLocation && self.storeName == other.storeName && self.subjectName == other.subjectName
    }
}
impl ::core::cmp::Eq for WS_SUBJECT_NAME_CERT_CREDENTIAL {}
impl ::core::fmt::Debug for WS_SUBJECT_NAME_CERT_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_SUBJECT_NAME_CERT_CREDENTIAL").field("credential", &self.credential).field("storeLocation", &self.storeLocation).field("storeName", &self.storeName).field("subjectName", &self.subjectName).finish()
    }
}
impl ::core::default::Default for WS_TCP_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_TCP_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties
    }
}
impl ::core::cmp::Eq for WS_TCP_BINDING_TEMPLATE {}
impl ::core::fmt::Debug for WS_TCP_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).finish()
    }
}
impl ::core::default::Default for WS_TCP_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_TCP_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties
    }
}
impl ::core::cmp::Eq for WS_TCP_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_TCP_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).finish()
    }
}
impl ::core::default::Default for WS_TCP_SSPI_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sspiTransportSecurityBinding == other.sspiTransportSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_BINDING_TEMPLATE {}
impl ::core::fmt::Debug for WS_TCP_SSPI_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sspiTransportSecurityBinding == other.sspiTransportSecurityBinding && self.kerberosApreqMessageSecurityBinding == other.kerberosApreqMessageSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {}
impl ::core::fmt::Debug for WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_KERBEROS_APREQ_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sspiTransportSecurityBinding == other.sspiTransportSecurityBinding && self.kerberosApreqMessageSecurityBinding == other.kerberosApreqMessageSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_KERBEROS_APREQ_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sspiTransportSecurityBinding == other.sspiTransportSecurityBinding && self.kerberosApreqMessageSecurityBinding == other.kerberosApreqMessageSecurityBinding && self.securityContextSecurityBinding == other.securityContextSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {}
impl ::core::fmt::Debug for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_BINDING_TEMPLATE").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sspiTransportSecurityBinding == other.sspiTransportSecurityBinding && self.kerberosApreqMessageSecurityBinding == other.kerberosApreqMessageSecurityBinding && self.securityContextSecurityBinding == other.securityContextSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_KERBEROS_APREQ_SECURITY_CONTEXT_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("kerberosApreqMessageSecurityBinding", &self.kerberosApreqMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_TCP_SSPI_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sspiTransportSecurityBinding == other.sspiTransportSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_TCP_SSPI_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.clientCredential == other.clientCredential
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {}
impl ::core::fmt::Debug for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING").field("binding", &self.binding).field("clientCredential", &self.clientCredential).finish()
    }
}
impl ::core::default::Default for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.bindingConstraint == other.bindingConstraint
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {}
impl ::core::fmt::Debug for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).finish()
    }
}
impl ::core::default::Default for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties && self.clientCredential == other.clientCredential
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {}
impl ::core::fmt::Debug for WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_TRANSPORT_SECURITY_BINDING_TEMPLATE").field("securityBindingProperties", &self.securityBindingProperties).field("clientCredential", &self.clientCredential).finish()
    }
}
impl ::core::default::Default for WS_TCP_SSPI_USERNAME_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sspiTransportSecurityBinding == other.sspiTransportSecurityBinding && self.usernameMessageSecurityBinding == other.usernameMessageSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_USERNAME_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("usernameMessageSecurityBinding", &self.usernameMessageSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.channelProperties == other.channelProperties && self.securityProperties == other.securityProperties && self.sspiTransportSecurityBinding == other.sspiTransportSecurityBinding && self.usernameMessageSecurityBinding == other.usernameMessageSecurityBinding && self.securityContextSecurityBinding == other.securityContextSecurityBinding
    }
}
impl ::core::cmp::Eq for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TCP_SSPI_USERNAME_SECURITY_CONTEXT_POLICY_DESCRIPTION").field("channelProperties", &self.channelProperties).field("securityProperties", &self.securityProperties).field("sspiTransportSecurityBinding", &self.sspiTransportSecurityBinding).field("usernameMessageSecurityBinding", &self.usernameMessageSecurityBinding).field("securityContextSecurityBinding", &self.securityContextSecurityBinding).finish()
    }
}
impl ::core::default::Default for WS_THUMBPRINT_CERT_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_THUMBPRINT_CERT_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credential == other.credential && self.storeLocation == other.storeLocation && self.storeName == other.storeName && self.thumbprint == other.thumbprint
    }
}
impl ::core::cmp::Eq for WS_THUMBPRINT_CERT_CREDENTIAL {}
impl ::core::fmt::Debug for WS_THUMBPRINT_CERT_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_THUMBPRINT_CERT_CREDENTIAL").field("credential", &self.credential).field("storeLocation", &self.storeLocation).field("storeName", &self.storeName).field("thumbprint", &self.thumbprint).finish()
    }
}
impl ::core::default::Default for WS_TIMESPAN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_TIMESPAN {
    fn eq(&self, other: &Self) -> bool {
        self.ticks == other.ticks
    }
}
impl ::core::cmp::Eq for WS_TIMESPAN {}
impl ::core::fmt::Debug for WS_TIMESPAN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TIMESPAN").field("ticks", &self.ticks).finish()
    }
}
impl ::core::default::Default for WS_TIMESPAN_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_TIMESPAN_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_TIMESPAN_DESCRIPTION {}
impl ::core::fmt::Debug for WS_TIMESPAN_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_TIMESPAN_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::core::default::Default for WS_TRACE_API {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_TRACE_API {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_TRACE_API").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_TRANSFER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_TRANSFER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_TRANSFER_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_TRUST_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_TRUST_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_TRUST_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_TYPE_MAPPING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_TYPE_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_TYPE_MAPPING").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_UINT16_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_UINT16_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_UINT16_DESCRIPTION {}
impl ::core::fmt::Debug for WS_UINT16_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UINT16_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::core::default::Default for WS_UINT32_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_UINT32_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_UINT32_DESCRIPTION {}
impl ::core::fmt::Debug for WS_UINT32_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UINT32_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::core::default::Default for WS_UINT64_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_UINT64_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_UINT64_DESCRIPTION {}
impl ::core::fmt::Debug for WS_UINT64_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UINT64_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::core::default::Default for WS_UINT8_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_UINT8_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for WS_UINT8_DESCRIPTION {}
impl ::core::fmt::Debug for WS_UINT8_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UINT8_DESCRIPTION").field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_UNION_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_UNION_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.alignment == other.alignment && self.fields == other.fields && self.fieldCount == other.fieldCount && self.enumOffset == other.enumOffset && self.noneEnumValue == other.noneEnumValue && self.valueIndices == other.valueIndices
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_UNION_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_UNION_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UNION_DESCRIPTION").field("size", &self.size).field("alignment", &self.alignment).field("fields", &self.fields).field("fieldCount", &self.fieldCount).field("enumOffset", &self.enumOffset).field("noneEnumValue", &self.noneEnumValue).field("valueIndices", &self.valueIndices).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_UNION_FIELD_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_UNION_FIELD_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.field == other.field
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_UNION_FIELD_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_UNION_FIELD_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UNION_FIELD_DESCRIPTION").field("value", &self.value).field("field", &self.field).finish()
    }
}
impl ::core::default::Default for WS_UNIQUE_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_UNIQUE_ID {
    fn eq(&self, other: &Self) -> bool {
        self.uri == other.uri && self.guid == other.guid
    }
}
impl ::core::cmp::Eq for WS_UNIQUE_ID {}
impl ::core::fmt::Debug for WS_UNIQUE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UNIQUE_ID").field("uri", &self.uri).field("guid", &self.guid).finish()
    }
}
impl ::core::default::Default for WS_UNIQUE_ID_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_UNIQUE_ID_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minCharCount == other.minCharCount && self.maxCharCount == other.maxCharCount
    }
}
impl ::core::cmp::Eq for WS_UNIQUE_ID_DESCRIPTION {}
impl ::core::fmt::Debug for WS_UNIQUE_ID_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UNIQUE_ID_DESCRIPTION").field("minCharCount", &self.minCharCount).field("maxCharCount", &self.maxCharCount).finish()
    }
}
impl ::core::default::Default for WS_UNKNOWN_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_UNKNOWN_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity && self.element == other.element
    }
}
impl ::core::cmp::Eq for WS_UNKNOWN_ENDPOINT_IDENTITY {}
impl ::core::fmt::Debug for WS_UNKNOWN_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UNKNOWN_ENDPOINT_IDENTITY").field("identity", &self.identity).field("element", &self.element).finish()
    }
}
impl ::core::default::Default for WS_UPN_ENDPOINT_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_UPN_ENDPOINT_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity && self.upn == other.upn
    }
}
impl ::core::cmp::Eq for WS_UPN_ENDPOINT_IDENTITY {}
impl ::core::fmt::Debug for WS_UPN_ENDPOINT_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UPN_ENDPOINT_IDENTITY").field("identity", &self.identity).field("upn", &self.upn).finish()
    }
}
impl ::core::default::Default for WS_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_URL {
    fn eq(&self, other: &Self) -> bool {
        self.scheme == other.scheme
    }
}
impl ::core::cmp::Eq for WS_URL {}
impl ::core::fmt::Debug for WS_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_URL").field("scheme", &self.scheme).finish()
    }
}
impl ::core::default::Default for WS_URL_SCHEME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_URL_SCHEME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_URL_SCHEME_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_USERNAME_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_USERNAME_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credentialType == other.credentialType
    }
}
impl ::core::cmp::Eq for WS_USERNAME_CREDENTIAL {}
impl ::core::fmt::Debug for WS_USERNAME_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_USERNAME_CREDENTIAL").field("credentialType", &self.credentialType).finish()
    }
}
impl ::core::default::Default for WS_USERNAME_CREDENTIAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_USERNAME_CREDENTIAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_USERNAME_CREDENTIAL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_USERNAME_MESSAGE_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.bindingConstraint == other.bindingConstraint && self.bindingUsage == other.bindingUsage
    }
}
impl ::core::cmp::Eq for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {}
impl ::core::fmt::Debug for WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_USERNAME_MESSAGE_SECURITY_BINDING_CONSTRAINT").field("bindingConstraint", &self.bindingConstraint).field("bindingUsage", &self.bindingUsage).finish()
    }
}
impl ::core::default::Default for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.securityBindingProperties == other.securityBindingProperties && self.bindingUsage == other.bindingUsage
    }
}
impl ::core::cmp::Eq for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_USERNAME_MESSAGE_SECURITY_BINDING_POLICY_DESCRIPTION").field("securityBindingProperties", &self.securityBindingProperties).field("bindingUsage", &self.bindingUsage).finish()
    }
}
impl ::core::default::Default for WS_USERNAME_MESSAGE_SECURITY_BINDING_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_UTF8_ARRAY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_UTF8_ARRAY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minByteCount == other.minByteCount && self.maxByteCount == other.maxByteCount
    }
}
impl ::core::cmp::Eq for WS_UTF8_ARRAY_DESCRIPTION {}
impl ::core::fmt::Debug for WS_UTF8_ARRAY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_UTF8_ARRAY_DESCRIPTION").field("minByteCount", &self.minByteCount).field("maxByteCount", &self.maxByteCount).finish()
    }
}
impl ::core::default::Default for WS_VALUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_VALUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_VALUE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_VOID_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_VOID_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}
impl ::core::cmp::Eq for WS_VOID_DESCRIPTION {}
impl ::core::fmt::Debug for WS_VOID_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_VOID_DESCRIPTION").field("size", &self.size).finish()
    }
}
impl ::core::default::Default for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.credentialType == other.credentialType
    }
}
impl ::core::cmp::Eq for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {}
impl ::core::fmt::Debug for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL").field("credentialType", &self.credentialType).finish()
    }
}
impl ::core::default::Default for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_WINDOWS_INTEGRATED_AUTH_CREDENTIAL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_WINDOWS_INTEGRATED_AUTH_PACKAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_WINDOWS_INTEGRATED_AUTH_PACKAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_WINDOWS_INTEGRATED_AUTH_PACKAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_WRITE_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_WRITE_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_WRITE_OPTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_WSZ_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_WSZ_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minCharCount == other.minCharCount && self.maxCharCount == other.maxCharCount
    }
}
impl ::core::cmp::Eq for WS_WSZ_DESCRIPTION {}
impl ::core::fmt::Debug for WS_WSZ_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_WSZ_DESCRIPTION").field("minCharCount", &self.minCharCount).field("maxCharCount", &self.maxCharCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.singleQuote == other.singleQuote && self.isXmlNs == other.isXmlNs && self.prefix == other.prefix && self.localName == other.localName && self.ns == other.ns && self.value == other.value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_ATTRIBUTE").field("singleQuote", &self.singleQuote).field("isXmlNs", &self.isXmlNs).field("prefix", &self.prefix).field("localName", &self.localName).field("ns", &self.ns).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for WS_XML_BASE64_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_BASE64_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.bytes == other.bytes && self.length == other.length
    }
}
impl ::core::cmp::Eq for WS_XML_BASE64_TEXT {}
impl ::core::fmt::Debug for WS_XML_BASE64_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_BASE64_TEXT").field("text", &self.text).field("bytes", &self.bytes).field("length", &self.length).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_BOOL_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_BOOL_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_BOOL_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_BOOL_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_BOOL_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for WS_XML_BUFFER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_BUFFER_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_XML_BUFFER_PROPERTY {}
impl ::core::fmt::Debug for WS_XML_BUFFER_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_BUFFER_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_XML_BUFFER_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_XML_BUFFER_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_BUFFER_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_XML_CANONICALIZATION_ALGORITHM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_XML_CANONICALIZATION_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_CANONICALIZATION_ALGORITHM").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {
    fn eq(&self, other: &Self) -> bool {
        self.prefixCount == other.prefixCount && self.prefixes == other.prefixes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_CANONICALIZATION_INCLUSIVE_PREFIXES").field("prefixCount", &self.prefixCount).field("prefixes", &self.prefixes).finish()
    }
}
impl ::core::default::Default for WS_XML_CANONICALIZATION_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_CANONICALIZATION_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_XML_CANONICALIZATION_PROPERTY {}
impl ::core::fmt::Debug for WS_XML_CANONICALIZATION_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_CANONICALIZATION_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_XML_CANONICALIZATION_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_XML_CANONICALIZATION_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_CANONICALIZATION_PROPERTY_ID").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_COMMENT_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_COMMENT_NODE {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.value == other.value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_COMMENT_NODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_COMMENT_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_COMMENT_NODE").field("node", &self.node).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for WS_XML_DATETIME_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_DATETIME_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_DATETIME_TEXT {}
impl ::core::fmt::Debug for WS_XML_DATETIME_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_DATETIME_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_DECIMAL_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_DICTIONARY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_DICTIONARY {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.strings == other.strings && self.stringCount == other.stringCount && self.isConst == other.isConst
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_DICTIONARY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_DICTIONARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_DICTIONARY").field("guid", &self.guid).field("strings", &self.strings).field("stringCount", &self.stringCount).field("isConst", &self.isConst).finish()
    }
}
impl ::core::default::Default for WS_XML_DOUBLE_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_DOUBLE_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_DOUBLE_TEXT {}
impl ::core::fmt::Debug for WS_XML_DOUBLE_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_DOUBLE_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_ELEMENT_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_ELEMENT_NODE {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.prefix == other.prefix && self.localName == other.localName && self.ns == other.ns && self.attributeCount == other.attributeCount && self.attributes == other.attributes && self.isEmpty == other.isEmpty
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_ELEMENT_NODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_ELEMENT_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_ELEMENT_NODE").field("node", &self.node).field("prefix", &self.prefix).field("localName", &self.localName).field("ns", &self.ns).field("attributeCount", &self.attributeCount).field("attributes", &self.attributes).field("isEmpty", &self.isEmpty).finish()
    }
}
impl ::core::default::Default for WS_XML_FLOAT_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_FLOAT_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_FLOAT_TEXT {}
impl ::core::fmt::Debug for WS_XML_FLOAT_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_FLOAT_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for WS_XML_GUID_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_GUID_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_GUID_TEXT {}
impl ::core::fmt::Debug for WS_XML_GUID_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_GUID_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for WS_XML_INT32_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_INT32_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_INT32_TEXT {}
impl ::core::fmt::Debug for WS_XML_INT32_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_INT32_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for WS_XML_INT64_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_INT64_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_INT64_TEXT {}
impl ::core::fmt::Debug for WS_XML_INT64_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_INT64_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for WS_XML_LIST_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_LIST_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.itemCount == other.itemCount && self.items == other.items
    }
}
impl ::core::cmp::Eq for WS_XML_LIST_TEXT {}
impl ::core::fmt::Debug for WS_XML_LIST_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_LIST_TEXT").field("text", &self.text).field("itemCount", &self.itemCount).field("items", &self.items).finish()
    }
}
impl ::core::default::Default for WS_XML_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_NODE {
    fn eq(&self, other: &Self) -> bool {
        self.nodeType == other.nodeType
    }
}
impl ::core::cmp::Eq for WS_XML_NODE {}
impl ::core::fmt::Debug for WS_XML_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_NODE").field("nodeType", &self.nodeType).finish()
    }
}
impl ::core::default::Default for WS_XML_NODE_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_NODE_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer && self.node == other.node
    }
}
impl ::core::cmp::Eq for WS_XML_NODE_POSITION {}
impl ::core::fmt::Debug for WS_XML_NODE_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_NODE_POSITION").field("buffer", &self.buffer).field("node", &self.node).finish()
    }
}
impl ::core::default::Default for WS_XML_NODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_XML_NODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_NODE_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_QNAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_QNAME {
    fn eq(&self, other: &Self) -> bool {
        self.localName == other.localName && self.ns == other.ns
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_QNAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_QNAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_QNAME").field("localName", &self.localName).field("ns", &self.ns).finish()
    }
}
impl ::core::default::Default for WS_XML_QNAME_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_QNAME_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minLocalNameByteCount == other.minLocalNameByteCount && self.maxLocalNameByteCount == other.maxLocalNameByteCount && self.minNsByteCount == other.minNsByteCount && self.maxNsByteCount == other.maxNsByteCount
    }
}
impl ::core::cmp::Eq for WS_XML_QNAME_DESCRIPTION {}
impl ::core::fmt::Debug for WS_XML_QNAME_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_QNAME_DESCRIPTION").field("minLocalNameByteCount", &self.minLocalNameByteCount).field("maxLocalNameByteCount", &self.maxLocalNameByteCount).field("minNsByteCount", &self.minNsByteCount).field("maxNsByteCount", &self.maxNsByteCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_QNAME_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_QNAME_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.prefix == other.prefix && self.localName == other.localName && self.ns == other.ns
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_QNAME_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_QNAME_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_QNAME_TEXT").field("text", &self.text).field("prefix", &self.prefix).field("localName", &self.localName).field("ns", &self.ns).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_READER_BINARY_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_READER_BINARY_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encoding == other.encoding && self.staticDictionary == other.staticDictionary && self.dynamicDictionary == other.dynamicDictionary
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_READER_BINARY_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_READER_BINARY_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_BINARY_ENCODING").field("encoding", &self.encoding).field("staticDictionary", &self.staticDictionary).field("dynamicDictionary", &self.dynamicDictionary).finish()
    }
}
impl ::core::default::Default for WS_XML_READER_BUFFER_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_READER_BUFFER_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.input == other.input && self.encodedData == other.encodedData && self.encodedDataSize == other.encodedDataSize
    }
}
impl ::core::cmp::Eq for WS_XML_READER_BUFFER_INPUT {}
impl ::core::fmt::Debug for WS_XML_READER_BUFFER_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_BUFFER_INPUT").field("input", &self.input).field("encodedData", &self.encodedData).field("encodedDataSize", &self.encodedDataSize).finish()
    }
}
impl ::core::default::Default for WS_XML_READER_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_READER_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encodingType == other.encodingType
    }
}
impl ::core::cmp::Eq for WS_XML_READER_ENCODING {}
impl ::core::fmt::Debug for WS_XML_READER_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_ENCODING").field("encodingType", &self.encodingType).finish()
    }
}
impl ::core::default::Default for WS_XML_READER_ENCODING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_XML_READER_ENCODING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_READER_ENCODING_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_XML_READER_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_READER_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.inputType == other.inputType
    }
}
impl ::core::cmp::Eq for WS_XML_READER_INPUT {}
impl ::core::fmt::Debug for WS_XML_READER_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_INPUT").field("inputType", &self.inputType).finish()
    }
}
impl ::core::default::Default for WS_XML_READER_INPUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_XML_READER_INPUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_READER_INPUT_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_READER_MTOM_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_READER_MTOM_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encoding == other.encoding && self.textEncoding == other.textEncoding && self.readMimeHeader == other.readMimeHeader && self.startInfo == other.startInfo && self.boundary == other.boundary && self.startUri == other.startUri
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_READER_MTOM_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_READER_MTOM_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_MTOM_ENCODING").field("encoding", &self.encoding).field("textEncoding", &self.textEncoding).field("readMimeHeader", &self.readMimeHeader).field("startInfo", &self.startInfo).field("boundary", &self.boundary).field("startUri", &self.startUri).finish()
    }
}
impl ::core::default::Default for WS_XML_READER_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_READER_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_XML_READER_PROPERTIES {}
impl ::core::fmt::Debug for WS_XML_READER_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::core::default::Default for WS_XML_READER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_READER_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_XML_READER_PROPERTY {}
impl ::core::fmt::Debug for WS_XML_READER_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_XML_READER_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_XML_READER_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_READER_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_XML_READER_RAW_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_READER_RAW_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encoding == other.encoding
    }
}
impl ::core::cmp::Eq for WS_XML_READER_RAW_ENCODING {}
impl ::core::fmt::Debug for WS_XML_READER_RAW_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_RAW_ENCODING").field("encoding", &self.encoding).finish()
    }
}
impl ::core::default::Default for WS_XML_READER_STREAM_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_XML_READER_TEXT_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_READER_TEXT_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encoding == other.encoding && self.charSet == other.charSet
    }
}
impl ::core::cmp::Eq for WS_XML_READER_TEXT_ENCODING {}
impl ::core::fmt::Debug for WS_XML_READER_TEXT_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_READER_TEXT_ENCODING").field("encoding", &self.encoding).field("charSet", &self.charSet).finish()
    }
}
impl ::core::default::Default for WS_XML_SECURITY_TOKEN_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_SECURITY_TOKEN_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_XML_SECURITY_TOKEN_PROPERTY {}
impl ::core::fmt::Debug for WS_XML_SECURITY_TOKEN_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_SECURITY_TOKEN_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_XML_SECURITY_TOKEN_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_XML_SECURITY_TOKEN_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_SECURITY_TOKEN_PROPERTY_ID").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.bytes == other.bytes && self.dictionary == other.dictionary && self.id == other.id
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_STRING").field("length", &self.length).field("bytes", &self.bytes).field("dictionary", &self.dictionary).field("id", &self.id).finish()
    }
}
impl ::core::default::Default for WS_XML_STRING_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_STRING_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.minByteCount == other.minByteCount && self.maxByteCount == other.maxByteCount
    }
}
impl ::core::cmp::Eq for WS_XML_STRING_DESCRIPTION {}
impl ::core::fmt::Debug for WS_XML_STRING_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_STRING_DESCRIPTION").field("minByteCount", &self.minByteCount).field("maxByteCount", &self.maxByteCount).finish()
    }
}
impl ::core::default::Default for WS_XML_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.textType == other.textType
    }
}
impl ::core::cmp::Eq for WS_XML_TEXT {}
impl ::core::fmt::Debug for WS_XML_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_TEXT").field("textType", &self.textType).finish()
    }
}
impl ::core::default::Default for WS_XML_TEXT_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_TEXT_NODE {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.text == other.text
    }
}
impl ::core::cmp::Eq for WS_XML_TEXT_NODE {}
impl ::core::fmt::Debug for WS_XML_TEXT_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_TEXT_NODE").field("node", &self.node).field("text", &self.text).finish()
    }
}
impl ::core::default::Default for WS_XML_TEXT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_XML_TEXT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_TEXT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_XML_TIMESPAN_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_TIMESPAN_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_TIMESPAN_TEXT {}
impl ::core::fmt::Debug for WS_XML_TIMESPAN_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_TIMESPAN_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.bindingUsage == other.bindingUsage && self.xmlToken == other.xmlToken
    }
}
impl ::core::cmp::Eq for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {}
impl ::core::fmt::Debug for WS_XML_TOKEN_MESSAGE_SECURITY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_TOKEN_MESSAGE_SECURITY_BINDING").field("binding", &self.binding).field("bindingUsage", &self.bindingUsage).field("xmlToken", &self.xmlToken).finish()
    }
}
impl ::core::default::Default for WS_XML_UINT64_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_UINT64_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_UINT64_TEXT {}
impl ::core::fmt::Debug for WS_XML_UINT64_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_UINT64_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for WS_XML_UNIQUE_ID_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_UNIQUE_ID_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
impl ::core::cmp::Eq for WS_XML_UNIQUE_ID_TEXT {}
impl ::core::fmt::Debug for WS_XML_UNIQUE_ID_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_UNIQUE_ID_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for WS_XML_UTF16_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_UTF16_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.bytes == other.bytes && self.byteCount == other.byteCount
    }
}
impl ::core::cmp::Eq for WS_XML_UTF16_TEXT {}
impl ::core::fmt::Debug for WS_XML_UTF16_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_UTF16_TEXT").field("text", &self.text).field("bytes", &self.bytes).field("byteCount", &self.byteCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_UTF8_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_UTF8_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.value == other.value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_UTF8_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_UTF8_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_UTF8_TEXT").field("text", &self.text).field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_WRITER_BINARY_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_XML_WRITER_BUFFER_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_BUFFER_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.output == other.output
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_BUFFER_OUTPUT {}
impl ::core::fmt::Debug for WS_XML_WRITER_BUFFER_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_BUFFER_OUTPUT").field("output", &self.output).finish()
    }
}
impl ::core::default::Default for WS_XML_WRITER_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encodingType == other.encodingType
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_ENCODING {}
impl ::core::fmt::Debug for WS_XML_WRITER_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_ENCODING").field("encodingType", &self.encodingType).finish()
    }
}
impl ::core::default::Default for WS_XML_WRITER_ENCODING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_ENCODING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_WRITER_ENCODING_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WS_XML_WRITER_MTOM_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WS_XML_WRITER_MTOM_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encoding == other.encoding && self.textEncoding == other.textEncoding && self.writeMimeHeader == other.writeMimeHeader && self.boundary == other.boundary && self.startInfo == other.startInfo && self.startUri == other.startUri && self.maxInlineByteCount == other.maxInlineByteCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WS_XML_WRITER_MTOM_ENCODING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WS_XML_WRITER_MTOM_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_MTOM_ENCODING").field("encoding", &self.encoding).field("textEncoding", &self.textEncoding).field("writeMimeHeader", &self.writeMimeHeader).field("boundary", &self.boundary).field("startInfo", &self.startInfo).field("startUri", &self.startUri).field("maxInlineByteCount", &self.maxInlineByteCount).finish()
    }
}
impl ::core::default::Default for WS_XML_WRITER_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.outputType == other.outputType
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_OUTPUT {}
impl ::core::fmt::Debug for WS_XML_WRITER_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_OUTPUT").field("outputType", &self.outputType).finish()
    }
}
impl ::core::default::Default for WS_XML_WRITER_OUTPUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_OUTPUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_WRITER_OUTPUT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_XML_WRITER_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties && self.propertyCount == other.propertyCount
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_PROPERTIES {}
impl ::core::fmt::Debug for WS_XML_WRITER_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_PROPERTIES").field("properties", &self.properties).field("propertyCount", &self.propertyCount).finish()
    }
}
impl ::core::default::Default for WS_XML_WRITER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.valueSize == other.valueSize
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_PROPERTY {}
impl ::core::fmt::Debug for WS_XML_WRITER_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_PROPERTY").field("id", &self.id).field("value", &self.value).field("valueSize", &self.valueSize).finish()
    }
}
impl ::core::default::Default for WS_XML_WRITER_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WS_XML_WRITER_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WS_XML_WRITER_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WS_XML_WRITER_RAW_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_RAW_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encoding == other.encoding
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_RAW_ENCODING {}
impl ::core::fmt::Debug for WS_XML_WRITER_RAW_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_RAW_ENCODING").field("encoding", &self.encoding).finish()
    }
}
impl ::core::default::Default for WS_XML_WRITER_STREAM_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WS_XML_WRITER_TEXT_ENCODING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WS_XML_WRITER_TEXT_ENCODING {
    fn eq(&self, other: &Self) -> bool {
        self.encoding == other.encoding && self.charSet == other.charSet
    }
}
impl ::core::cmp::Eq for WS_XML_WRITER_TEXT_ENCODING {}
impl ::core::fmt::Debug for WS_XML_WRITER_TEXT_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WS_XML_WRITER_TEXT_ENCODING").field("encoding", &self.encoding).field("charSet", &self.charSet).finish()
    }
}
