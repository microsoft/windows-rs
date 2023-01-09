impl ::core::cmp::PartialEq for Certificate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Certificate {}
impl ::core::fmt::Debug for Certificate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Certificate").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CertificateChain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CertificateChain {}
impl ::core::fmt::Debug for CertificateChain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CertificateChain").field(&self.0).finish()
    }
}
impl ::core::default::Default for CertificateChainPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CertificateChainPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CertificateChainPolicy").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CertificateExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CertificateExtension {}
impl ::core::fmt::Debug for CertificateExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CertificateExtension").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CertificateKeyUsages {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CertificateKeyUsages {}
impl ::core::fmt::Debug for CertificateKeyUsages {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CertificateKeyUsages").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CertificateQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CertificateQuery {}
impl ::core::fmt::Debug for CertificateQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CertificateQuery").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CertificateRequestProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CertificateRequestProperties {}
impl ::core::fmt::Debug for CertificateRequestProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CertificateRequestProperties").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CertificateStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CertificateStore {}
impl ::core::fmt::Debug for CertificateStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CertificateStore").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChainBuildingParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChainBuildingParameters {}
impl ::core::fmt::Debug for ChainBuildingParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChainBuildingParameters").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChainValidationParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChainValidationParameters {}
impl ::core::fmt::Debug for ChainValidationParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChainValidationParameters").field(&self.0).finish()
    }
}
impl ::core::default::Default for ChainValidationResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ChainValidationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChainValidationResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CmsAttachedSignature {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CmsAttachedSignature {}
impl ::core::fmt::Debug for CmsAttachedSignature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CmsAttachedSignature").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CmsDetachedSignature {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CmsDetachedSignature {}
impl ::core::fmt::Debug for CmsDetachedSignature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CmsDetachedSignature").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CmsSignerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CmsSignerInfo {}
impl ::core::fmt::Debug for CmsSignerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CmsSignerInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CmsTimestampInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CmsTimestampInfo {}
impl ::core::fmt::Debug for CmsTimestampInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CmsTimestampInfo").field(&self.0).finish()
    }
}
impl ::core::default::Default for EnrollKeyUsages {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EnrollKeyUsages {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnrollKeyUsages").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for EnrollKeyUsages {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for EnrollKeyUsages {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for EnrollKeyUsages {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for EnrollKeyUsages {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for EnrollKeyUsages {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for ExportOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ExportOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExportOption").field(&self.0).finish()
    }
}
impl ::core::default::Default for InstallOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InstallOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InstallOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InstallOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InstallOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InstallOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InstallOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InstallOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for KeyProtectionLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KeyProtectionLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyProtectionLevel").field(&self.0).finish()
    }
}
impl ::core::default::Default for KeySize {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KeySize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeySize").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PfxImportParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PfxImportParameters {}
impl ::core::fmt::Debug for PfxImportParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PfxImportParameters").field(&self.0).finish()
    }
}
impl ::core::default::Default for SignatureValidationResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SignatureValidationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SignatureValidationResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SubjectAlternativeNameInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SubjectAlternativeNameInfo {}
impl ::core::fmt::Debug for SubjectAlternativeNameInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SubjectAlternativeNameInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for UserCertificateEnrollmentManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserCertificateEnrollmentManager {}
impl ::core::fmt::Debug for UserCertificateEnrollmentManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserCertificateEnrollmentManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for UserCertificateStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserCertificateStore {}
impl ::core::fmt::Debug for UserCertificateStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserCertificateStore").field(&self.0).finish()
    }
}
