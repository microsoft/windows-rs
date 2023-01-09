impl ::core::default::Default for AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwRegPolicySettings == other.dwRegPolicySettings && self.pSignerInfo == other.pSignerInfo
    }
}
impl ::core::cmp::Eq for AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA {}
impl ::core::fmt::Debug for AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA").field("cbSize", &self.cbSize).field("dwRegPolicySettings", &self.dwRegPolicySettings).field("pSignerInfo", &self.pSignerInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fCommercial == other.fCommercial
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS").field("cbSize", &self.cbSize).field("fCommercial", &self.fCommercial).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwRegPolicySettings == other.dwRegPolicySettings && self.fCommercial == other.fCommercial
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA").field("cbSize", &self.cbSize).field("dwRegPolicySettings", &self.dwRegPolicySettings).field("fCommercial", &self.fCommercial).finish()
    }
}
impl ::core::default::Default for BCRYPT_ALGORITHM_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_ALGORITHM_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.dwClass == other.dwClass && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for BCRYPT_ALGORITHM_IDENTIFIER {}
impl ::core::fmt::Debug for BCRYPT_ALGORITHM_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_ALGORITHM_IDENTIFIER").field("pszName", &self.pszName).field("dwClass", &self.dwClass).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwInfoVersion == other.dwInfoVersion && self.pbNonce == other.pbNonce && self.cbNonce == other.cbNonce && self.pbAuthData == other.pbAuthData && self.cbAuthData == other.cbAuthData && self.pbTag == other.pbTag && self.cbTag == other.cbTag && self.pbMacContext == other.pbMacContext && self.cbMacContext == other.cbMacContext && self.cbAAD == other.cbAAD && self.cbData == other.cbData && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO {}
impl ::core::fmt::Debug for BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO")
            .field("cbSize", &self.cbSize)
            .field("dwInfoVersion", &self.dwInfoVersion)
            .field("pbNonce", &self.pbNonce)
            .field("cbNonce", &self.cbNonce)
            .field("pbAuthData", &self.pbAuthData)
            .field("cbAuthData", &self.cbAuthData)
            .field("pbTag", &self.pbTag)
            .field("cbTag", &self.cbTag)
            .field("pbMacContext", &self.pbMacContext)
            .field("cbMacContext", &self.cbMacContext)
            .field("cbAAD", &self.cbAAD)
            .field("cbData", &self.cbData)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
impl ::core::default::Default for BCRYPT_DH_KEY_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_DH_KEY_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwMagic == other.dwMagic && self.cbKey == other.cbKey
    }
}
impl ::core::cmp::Eq for BCRYPT_DH_KEY_BLOB {}
impl ::core::fmt::Debug for BCRYPT_DH_KEY_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_DH_KEY_BLOB").field("dwMagic", &self.dwMagic).field("cbKey", &self.cbKey).finish()
    }
}
impl ::core::default::Default for BCRYPT_DH_KEY_BLOB_MAGIC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BCRYPT_DH_KEY_BLOB_MAGIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BCRYPT_DH_KEY_BLOB_MAGIC").field(&self.0).finish()
    }
}
impl ::core::default::Default for BCRYPT_DH_PARAMETER_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_DH_PARAMETER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cbLength == other.cbLength && self.dwMagic == other.dwMagic && self.cbKeyLength == other.cbKeyLength
    }
}
impl ::core::cmp::Eq for BCRYPT_DH_PARAMETER_HEADER {}
impl ::core::fmt::Debug for BCRYPT_DH_PARAMETER_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_DH_PARAMETER_HEADER").field("cbLength", &self.cbLength).field("dwMagic", &self.dwMagic).field("cbKeyLength", &self.cbKeyLength).finish()
    }
}
impl ::core::default::Default for BCRYPT_DSA_KEY_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_DSA_KEY_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwMagic == other.dwMagic && self.cbKey == other.cbKey && self.Count == other.Count && self.Seed == other.Seed && self.q == other.q
    }
}
impl ::core::cmp::Eq for BCRYPT_DSA_KEY_BLOB {}
impl ::core::fmt::Debug for BCRYPT_DSA_KEY_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_DSA_KEY_BLOB").field("dwMagic", &self.dwMagic).field("cbKey", &self.cbKey).field("Count", &self.Count).field("Seed", &self.Seed).field("q", &self.q).finish()
    }
}
impl ::core::default::Default for BCRYPT_DSA_KEY_BLOB_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_DSA_KEY_BLOB_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwMagic == other.dwMagic && self.cbKey == other.cbKey && self.hashAlgorithm == other.hashAlgorithm && self.standardVersion == other.standardVersion && self.cbSeedLength == other.cbSeedLength && self.cbGroupSize == other.cbGroupSize && self.Count == other.Count
    }
}
impl ::core::cmp::Eq for BCRYPT_DSA_KEY_BLOB_V2 {}
impl ::core::fmt::Debug for BCRYPT_DSA_KEY_BLOB_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_DSA_KEY_BLOB_V2").field("dwMagic", &self.dwMagic).field("cbKey", &self.cbKey).field("hashAlgorithm", &self.hashAlgorithm).field("standardVersion", &self.standardVersion).field("cbSeedLength", &self.cbSeedLength).field("cbGroupSize", &self.cbGroupSize).field("Count", &self.Count).finish()
    }
}
impl ::core::default::Default for BCRYPT_DSA_MAGIC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BCRYPT_DSA_MAGIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BCRYPT_DSA_MAGIC").field(&self.0).finish()
    }
}
impl ::core::default::Default for BCRYPT_DSA_PARAMETER_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_DSA_PARAMETER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cbLength == other.cbLength && self.dwMagic == other.dwMagic && self.cbKeyLength == other.cbKeyLength && self.Count == other.Count && self.Seed == other.Seed && self.q == other.q
    }
}
impl ::core::cmp::Eq for BCRYPT_DSA_PARAMETER_HEADER {}
impl ::core::fmt::Debug for BCRYPT_DSA_PARAMETER_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_DSA_PARAMETER_HEADER").field("cbLength", &self.cbLength).field("dwMagic", &self.dwMagic).field("cbKeyLength", &self.cbKeyLength).field("Count", &self.Count).field("Seed", &self.Seed).field("q", &self.q).finish()
    }
}
impl ::core::default::Default for BCRYPT_DSA_PARAMETER_HEADER_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_DSA_PARAMETER_HEADER_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.cbLength == other.cbLength && self.dwMagic == other.dwMagic && self.cbKeyLength == other.cbKeyLength && self.hashAlgorithm == other.hashAlgorithm && self.standardVersion == other.standardVersion && self.cbSeedLength == other.cbSeedLength && self.cbGroupSize == other.cbGroupSize && self.Count == other.Count
    }
}
impl ::core::cmp::Eq for BCRYPT_DSA_PARAMETER_HEADER_V2 {}
impl ::core::fmt::Debug for BCRYPT_DSA_PARAMETER_HEADER_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_DSA_PARAMETER_HEADER_V2").field("cbLength", &self.cbLength).field("dwMagic", &self.dwMagic).field("cbKeyLength", &self.cbKeyLength).field("hashAlgorithm", &self.hashAlgorithm).field("standardVersion", &self.standardVersion).field("cbSeedLength", &self.cbSeedLength).field("cbGroupSize", &self.cbGroupSize).field("Count", &self.Count).finish()
    }
}
impl ::core::default::Default for BCRYPT_ECCFULLKEY_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_ECCFULLKEY_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwMagic == other.dwMagic && self.dwVersion == other.dwVersion && self.dwCurveType == other.dwCurveType && self.dwCurveGenerationAlgId == other.dwCurveGenerationAlgId && self.cbFieldLength == other.cbFieldLength && self.cbSubgroupOrder == other.cbSubgroupOrder && self.cbCofactor == other.cbCofactor && self.cbSeed == other.cbSeed
    }
}
impl ::core::cmp::Eq for BCRYPT_ECCFULLKEY_BLOB {}
impl ::core::fmt::Debug for BCRYPT_ECCFULLKEY_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_ECCFULLKEY_BLOB").field("dwMagic", &self.dwMagic).field("dwVersion", &self.dwVersion).field("dwCurveType", &self.dwCurveType).field("dwCurveGenerationAlgId", &self.dwCurveGenerationAlgId).field("cbFieldLength", &self.cbFieldLength).field("cbSubgroupOrder", &self.cbSubgroupOrder).field("cbCofactor", &self.cbCofactor).field("cbSeed", &self.cbSeed).finish()
    }
}
impl ::core::default::Default for BCRYPT_ECCKEY_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_ECCKEY_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwMagic == other.dwMagic && self.cbKey == other.cbKey
    }
}
impl ::core::cmp::Eq for BCRYPT_ECCKEY_BLOB {}
impl ::core::fmt::Debug for BCRYPT_ECCKEY_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_ECCKEY_BLOB").field("dwMagic", &self.dwMagic).field("cbKey", &self.cbKey).finish()
    }
}
impl ::core::default::Default for BCRYPT_ECC_CURVE_NAMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_ECC_CURVE_NAMES {
    fn eq(&self, other: &Self) -> bool {
        self.dwEccCurveNames == other.dwEccCurveNames && self.pEccCurveNames == other.pEccCurveNames
    }
}
impl ::core::cmp::Eq for BCRYPT_ECC_CURVE_NAMES {}
impl ::core::fmt::Debug for BCRYPT_ECC_CURVE_NAMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_ECC_CURVE_NAMES").field("dwEccCurveNames", &self.dwEccCurveNames).field("pEccCurveNames", &self.pEccCurveNames).finish()
    }
}
impl ::core::default::Default for BCRYPT_HASH_OPERATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BCRYPT_HASH_OPERATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BCRYPT_HASH_OPERATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for BCRYPT_INTERFACE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BCRYPT_INTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BCRYPT_INTERFACE").field(&self.0).finish()
    }
}
impl ::core::default::Default for BCRYPT_INTERFACE_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_INTERFACE_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::core::cmp::Eq for BCRYPT_INTERFACE_VERSION {}
impl ::core::fmt::Debug for BCRYPT_INTERFACE_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_INTERFACE_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
impl ::core::default::Default for BCRYPT_KEY_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_KEY_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.Magic == other.Magic
    }
}
impl ::core::cmp::Eq for BCRYPT_KEY_BLOB {}
impl ::core::fmt::Debug for BCRYPT_KEY_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_KEY_BLOB").field("Magic", &self.Magic).finish()
    }
}
impl ::core::default::Default for BCRYPT_KEY_DATA_BLOB_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_KEY_DATA_BLOB_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwMagic == other.dwMagic && self.dwVersion == other.dwVersion && self.cbKeyData == other.cbKeyData
    }
}
impl ::core::cmp::Eq for BCRYPT_KEY_DATA_BLOB_HEADER {}
impl ::core::fmt::Debug for BCRYPT_KEY_DATA_BLOB_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_KEY_DATA_BLOB_HEADER").field("dwMagic", &self.dwMagic).field("dwVersion", &self.dwVersion).field("cbKeyData", &self.cbKeyData).finish()
    }
}
impl ::core::default::Default for BCRYPT_KEY_LENGTHS_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_KEY_LENGTHS_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.dwMinLength == other.dwMinLength && self.dwMaxLength == other.dwMaxLength && self.dwIncrement == other.dwIncrement
    }
}
impl ::core::cmp::Eq for BCRYPT_KEY_LENGTHS_STRUCT {}
impl ::core::fmt::Debug for BCRYPT_KEY_LENGTHS_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_KEY_LENGTHS_STRUCT").field("dwMinLength", &self.dwMinLength).field("dwMaxLength", &self.dwMaxLength).field("dwIncrement", &self.dwIncrement).finish()
    }
}
impl ::core::default::Default for BCRYPT_MULTI_HASH_OPERATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_MULTI_HASH_OPERATION {
    fn eq(&self, other: &Self) -> bool {
        self.iHash == other.iHash && self.hashOperation == other.hashOperation && self.pbBuffer == other.pbBuffer && self.cbBuffer == other.cbBuffer
    }
}
impl ::core::cmp::Eq for BCRYPT_MULTI_HASH_OPERATION {}
impl ::core::fmt::Debug for BCRYPT_MULTI_HASH_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_MULTI_HASH_OPERATION").field("iHash", &self.iHash).field("hashOperation", &self.hashOperation).field("pbBuffer", &self.pbBuffer).field("cbBuffer", &self.cbBuffer).finish()
    }
}
impl ::core::default::Default for BCRYPT_MULTI_OBJECT_LENGTH_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_MULTI_OBJECT_LENGTH_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cbPerObject == other.cbPerObject && self.cbPerElement == other.cbPerElement
    }
}
impl ::core::cmp::Eq for BCRYPT_MULTI_OBJECT_LENGTH_STRUCT {}
impl ::core::fmt::Debug for BCRYPT_MULTI_OBJECT_LENGTH_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_MULTI_OBJECT_LENGTH_STRUCT").field("cbPerObject", &self.cbPerObject).field("cbPerElement", &self.cbPerElement).finish()
    }
}
impl ::core::default::Default for BCRYPT_MULTI_OPERATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BCRYPT_MULTI_OPERATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BCRYPT_MULTI_OPERATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for BCRYPT_OAEP_PADDING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_OAEP_PADDING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszAlgId == other.pszAlgId && self.pbLabel == other.pbLabel && self.cbLabel == other.cbLabel
    }
}
impl ::core::cmp::Eq for BCRYPT_OAEP_PADDING_INFO {}
impl ::core::fmt::Debug for BCRYPT_OAEP_PADDING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_OAEP_PADDING_INFO").field("pszAlgId", &self.pszAlgId).field("pbLabel", &self.pbLabel).field("cbLabel", &self.cbLabel).finish()
    }
}
impl ::core::default::Default for BCRYPT_OID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_OID {
    fn eq(&self, other: &Self) -> bool {
        self.cbOID == other.cbOID && self.pbOID == other.pbOID
    }
}
impl ::core::cmp::Eq for BCRYPT_OID {}
impl ::core::fmt::Debug for BCRYPT_OID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_OID").field("cbOID", &self.cbOID).field("pbOID", &self.pbOID).finish()
    }
}
impl ::core::default::Default for BCRYPT_OID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_OID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwOIDCount == other.dwOIDCount && self.pOIDs == other.pOIDs
    }
}
impl ::core::cmp::Eq for BCRYPT_OID_LIST {}
impl ::core::fmt::Debug for BCRYPT_OID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_OID_LIST").field("dwOIDCount", &self.dwOIDCount).field("pOIDs", &self.pOIDs).finish()
    }
}
impl ::core::default::Default for BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for BCRYPT_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BCRYPT_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BCRYPT_OPERATION").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for BCRYPT_OPERATION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for BCRYPT_OPERATION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for BCRYPT_OPERATION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for BCRYPT_OPERATION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for BCRYPT_OPERATION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for BCRYPT_PKCS1_PADDING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_PKCS1_PADDING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszAlgId == other.pszAlgId
    }
}
impl ::core::cmp::Eq for BCRYPT_PKCS1_PADDING_INFO {}
impl ::core::fmt::Debug for BCRYPT_PKCS1_PADDING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_PKCS1_PADDING_INFO").field("pszAlgId", &self.pszAlgId).finish()
    }
}
impl ::core::default::Default for BCRYPT_PROVIDER_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_PROVIDER_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.pszProviderName == other.pszProviderName
    }
}
impl ::core::cmp::Eq for BCRYPT_PROVIDER_NAME {}
impl ::core::fmt::Debug for BCRYPT_PROVIDER_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_PROVIDER_NAME").field("pszProviderName", &self.pszProviderName).finish()
    }
}
impl ::core::default::Default for BCRYPT_PSS_PADDING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_PSS_PADDING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszAlgId == other.pszAlgId && self.cbSalt == other.cbSalt
    }
}
impl ::core::cmp::Eq for BCRYPT_PSS_PADDING_INFO {}
impl ::core::fmt::Debug for BCRYPT_PSS_PADDING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_PSS_PADDING_INFO").field("pszAlgId", &self.pszAlgId).field("cbSalt", &self.cbSalt).finish()
    }
}
impl ::core::default::Default for BCRYPT_QUERY_PROVIDER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BCRYPT_QUERY_PROVIDER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BCRYPT_QUERY_PROVIDER_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for BCRYPT_RESOLVE_PROVIDERS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BCRYPT_RESOLVE_PROVIDERS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BCRYPT_RESOLVE_PROVIDERS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for BCRYPT_RESOLVE_PROVIDERS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for BCRYPT_RESOLVE_PROVIDERS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for BCRYPT_RESOLVE_PROVIDERS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for BCRYPT_RESOLVE_PROVIDERS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for BCRYPT_RESOLVE_PROVIDERS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for BCRYPT_RSAKEY_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCRYPT_RSAKEY_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.Magic == other.Magic && self.BitLength == other.BitLength && self.cbPublicExp == other.cbPublicExp && self.cbModulus == other.cbModulus && self.cbPrime1 == other.cbPrime1 && self.cbPrime2 == other.cbPrime2
    }
}
impl ::core::cmp::Eq for BCRYPT_RSAKEY_BLOB {}
impl ::core::fmt::Debug for BCRYPT_RSAKEY_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCRYPT_RSAKEY_BLOB").field("Magic", &self.Magic).field("BitLength", &self.BitLength).field("cbPublicExp", &self.cbPublicExp).field("cbModulus", &self.cbModulus).field("cbPrime1", &self.cbPrime1).field("cbPrime2", &self.cbPrime2).finish()
    }
}
impl ::core::default::Default for BCRYPT_RSAKEY_BLOB_MAGIC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BCRYPT_RSAKEY_BLOB_MAGIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BCRYPT_RSAKEY_BLOB_MAGIC").field(&self.0).finish()
    }
}
impl ::core::default::Default for BCRYPT_TABLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BCRYPT_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BCRYPT_TABLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for BCryptBuffer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCryptBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.cbBuffer == other.cbBuffer && self.BufferType == other.BufferType && self.pvBuffer == other.pvBuffer
    }
}
impl ::core::cmp::Eq for BCryptBuffer {}
impl ::core::fmt::Debug for BCryptBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCryptBuffer").field("cbBuffer", &self.cbBuffer).field("BufferType", &self.BufferType).field("pvBuffer", &self.pvBuffer).finish()
    }
}
impl ::core::default::Default for BCryptBufferDesc {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BCryptBufferDesc {
    fn eq(&self, other: &Self) -> bool {
        self.ulVersion == other.ulVersion && self.cBuffers == other.cBuffers && self.pBuffers == other.pBuffers
    }
}
impl ::core::cmp::Eq for BCryptBufferDesc {}
impl ::core::fmt::Debug for BCryptBufferDesc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BCryptBufferDesc").field("ulVersion", &self.ulVersion).field("cBuffers", &self.cBuffers).field("pBuffers", &self.pBuffers).finish()
    }
}
impl ::core::default::Default for CASetupProperty {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CASetupProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CASetupProperty").field(&self.0).finish()
    }
}
impl ::core::default::Default for CEPSetupProperty {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CEPSetupProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CEPSetupProperty").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERTIFICATE_CHAIN_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERTIFICATE_CHAIN_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.certCount == other.certCount && self.rawCertificates == other.rawCertificates
    }
}
impl ::core::cmp::Eq for CERTIFICATE_CHAIN_BLOB {}
impl ::core::fmt::Debug for CERTIFICATE_CHAIN_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERTIFICATE_CHAIN_BLOB").field("certCount", &self.certCount).field("rawCertificates", &self.rawCertificates).finish()
    }
}
impl ::core::default::Default for CERT_ACCESS_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CERT_ALT_NAME_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CERT_ALT_NAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_ALT_NAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cAltEntry == other.cAltEntry && self.rgAltEntry == other.rgAltEntry
    }
}
impl ::core::cmp::Eq for CERT_ALT_NAME_INFO {}
impl ::core::fmt::Debug for CERT_ALT_NAME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_ALT_NAME_INFO").field("cAltEntry", &self.cAltEntry).field("rgAltEntry", &self.rgAltEntry).finish()
    }
}
impl ::core::default::Default for CERT_AUTHORITY_INFO_ACCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_AUTHORITY_INFO_ACCESS {
    fn eq(&self, other: &Self) -> bool {
        self.cAccDescr == other.cAccDescr && self.rgAccDescr == other.rgAccDescr
    }
}
impl ::core::cmp::Eq for CERT_AUTHORITY_INFO_ACCESS {}
impl ::core::fmt::Debug for CERT_AUTHORITY_INFO_ACCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_AUTHORITY_INFO_ACCESS").field("cAccDescr", &self.cAccDescr).field("rgAccDescr", &self.rgAccDescr).finish()
    }
}
impl ::core::default::Default for CERT_AUTHORITY_KEY_ID2_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_AUTHORITY_KEY_ID2_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.KeyId == other.KeyId && self.AuthorityCertIssuer == other.AuthorityCertIssuer && self.AuthorityCertSerialNumber == other.AuthorityCertSerialNumber
    }
}
impl ::core::cmp::Eq for CERT_AUTHORITY_KEY_ID2_INFO {}
impl ::core::fmt::Debug for CERT_AUTHORITY_KEY_ID2_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_AUTHORITY_KEY_ID2_INFO").field("KeyId", &self.KeyId).field("AuthorityCertIssuer", &self.AuthorityCertIssuer).field("AuthorityCertSerialNumber", &self.AuthorityCertSerialNumber).finish()
    }
}
impl ::core::default::Default for CERT_AUTHORITY_KEY_ID_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_AUTHORITY_KEY_ID_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.KeyId == other.KeyId && self.CertIssuer == other.CertIssuer && self.CertSerialNumber == other.CertSerialNumber
    }
}
impl ::core::cmp::Eq for CERT_AUTHORITY_KEY_ID_INFO {}
impl ::core::fmt::Debug for CERT_AUTHORITY_KEY_ID_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_AUTHORITY_KEY_ID_INFO").field("KeyId", &self.KeyId).field("CertIssuer", &self.CertIssuer).field("CertSerialNumber", &self.CertSerialNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_BASIC_CONSTRAINTS2_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_BASIC_CONSTRAINTS2_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.fCA == other.fCA && self.fPathLenConstraint == other.fPathLenConstraint && self.dwPathLenConstraint == other.dwPathLenConstraint
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_BASIC_CONSTRAINTS2_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_BASIC_CONSTRAINTS2_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_BASIC_CONSTRAINTS2_INFO").field("fCA", &self.fCA).field("fPathLenConstraint", &self.fPathLenConstraint).field("dwPathLenConstraint", &self.dwPathLenConstraint).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_BASIC_CONSTRAINTS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_BASIC_CONSTRAINTS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SubjectType == other.SubjectType && self.fPathLenConstraint == other.fPathLenConstraint && self.dwPathLenConstraint == other.dwPathLenConstraint && self.cSubtreesConstraint == other.cSubtreesConstraint && self.rgSubtreesConstraint == other.rgSubtreesConstraint
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_BASIC_CONSTRAINTS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_BASIC_CONSTRAINTS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_BASIC_CONSTRAINTS_INFO").field("SubjectType", &self.SubjectType).field("fPathLenConstraint", &self.fPathLenConstraint).field("dwPathLenConstraint", &self.dwPathLenConstraint).field("cSubtreesConstraint", &self.cSubtreesConstraint).field("rgSubtreesConstraint", &self.rgSubtreesConstraint).finish()
    }
}
impl ::core::default::Default for CERT_BIOMETRIC_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CERT_BIOMETRIC_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_BIOMETRIC_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_BIOMETRIC_DATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_BIOMETRIC_EXT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_BIOMETRIC_EXT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cBiometricData == other.cBiometricData && self.rgBiometricData == other.rgBiometricData
    }
}
impl ::core::cmp::Eq for CERT_BIOMETRIC_EXT_INFO {}
impl ::core::fmt::Debug for CERT_BIOMETRIC_EXT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_BIOMETRIC_EXT_INFO").field("cBiometricData", &self.cBiometricData).field("rgBiometricData", &self.rgBiometricData).finish()
    }
}
impl ::core::default::Default for CERT_CHAIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_CHAIN {
    fn eq(&self, other: &Self) -> bool {
        self.cCerts == other.cCerts && self.certs == other.certs && self.keyLocatorInfo == other.keyLocatorInfo
    }
}
impl ::core::cmp::Eq for CERT_CHAIN {}
impl ::core::fmt::Debug for CERT_CHAIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_CHAIN").field("cCerts", &self.cCerts).field("certs", &self.certs).field("keyLocatorInfo", &self.keyLocatorInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_CHAIN_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_CHAIN_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.TrustStatus == other.TrustStatus && self.cChain == other.cChain && self.rgpChain == other.rgpChain && self.cLowerQualityChainContext == other.cLowerQualityChainContext && self.rgpLowerQualityChainContext == other.rgpLowerQualityChainContext && self.fHasRevocationFreshnessTime == other.fHasRevocationFreshnessTime && self.dwRevocationFreshnessTime == other.dwRevocationFreshnessTime && self.dwCreateFlags == other.dwCreateFlags && self.ChainId == other.ChainId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_CHAIN_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_CHAIN_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_CHAIN_CONTEXT")
            .field("cbSize", &self.cbSize)
            .field("TrustStatus", &self.TrustStatus)
            .field("cChain", &self.cChain)
            .field("rgpChain", &self.rgpChain)
            .field("cLowerQualityChainContext", &self.cLowerQualityChainContext)
            .field("rgpLowerQualityChainContext", &self.rgpLowerQualityChainContext)
            .field("fHasRevocationFreshnessTime", &self.fHasRevocationFreshnessTime)
            .field("dwRevocationFreshnessTime", &self.dwRevocationFreshnessTime)
            .field("dwCreateFlags", &self.dwCreateFlags)
            .field("ChainId", &self.ChainId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_CHAIN_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_CHAIN_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pCertContext == other.pCertContext && self.TrustStatus == other.TrustStatus && self.pRevocationInfo == other.pRevocationInfo && self.pIssuanceUsage == other.pIssuanceUsage && self.pApplicationUsage == other.pApplicationUsage && self.pwszExtendedErrorInfo == other.pwszExtendedErrorInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_CHAIN_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_CHAIN_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_CHAIN_ELEMENT").field("cbSize", &self.cbSize).field("pCertContext", &self.pCertContext).field("TrustStatus", &self.TrustStatus).field("pRevocationInfo", &self.pRevocationInfo).field("pIssuanceUsage", &self.pIssuanceUsage).field("pApplicationUsage", &self.pApplicationUsage).field("pwszExtendedErrorInfo", &self.pwszExtendedErrorInfo).finish()
    }
}
impl ::core::default::Default for CERT_CHAIN_ENGINE_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_CHAIN_ENGINE_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hRestrictedRoot == other.hRestrictedRoot && self.hRestrictedTrust == other.hRestrictedTrust && self.hRestrictedOther == other.hRestrictedOther && self.cAdditionalStore == other.cAdditionalStore && self.rghAdditionalStore == other.rghAdditionalStore && self.dwFlags == other.dwFlags && self.dwUrlRetrievalTimeout == other.dwUrlRetrievalTimeout && self.MaximumCachedCertificates == other.MaximumCachedCertificates && self.CycleDetectionModulus == other.CycleDetectionModulus && self.hExclusiveRoot == other.hExclusiveRoot && self.hExclusiveTrustedPeople == other.hExclusiveTrustedPeople && self.dwExclusiveFlags == other.dwExclusiveFlags
    }
}
impl ::core::cmp::Eq for CERT_CHAIN_ENGINE_CONFIG {}
impl ::core::fmt::Debug for CERT_CHAIN_ENGINE_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_CHAIN_ENGINE_CONFIG")
            .field("cbSize", &self.cbSize)
            .field("hRestrictedRoot", &self.hRestrictedRoot)
            .field("hRestrictedTrust", &self.hRestrictedTrust)
            .field("hRestrictedOther", &self.hRestrictedOther)
            .field("cAdditionalStore", &self.cAdditionalStore)
            .field("rghAdditionalStore", &self.rghAdditionalStore)
            .field("dwFlags", &self.dwFlags)
            .field("dwUrlRetrievalTimeout", &self.dwUrlRetrievalTimeout)
            .field("MaximumCachedCertificates", &self.MaximumCachedCertificates)
            .field("CycleDetectionModulus", &self.CycleDetectionModulus)
            .field("hExclusiveRoot", &self.hExclusiveRoot)
            .field("hExclusiveTrustedPeople", &self.hExclusiveTrustedPeople)
            .field("dwExclusiveFlags", &self.dwExclusiveFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_CHAIN_FIND_BY_ISSUER_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CERT_CHAIN_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_CHAIN_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.RequestedUsage == other.RequestedUsage
    }
}
impl ::core::cmp::Eq for CERT_CHAIN_PARA {}
impl ::core::fmt::Debug for CERT_CHAIN_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_CHAIN_PARA").field("cbSize", &self.cbSize).field("RequestedUsage", &self.RequestedUsage).finish()
    }
}
impl ::core::default::Default for CERT_CHAIN_POLICY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_CHAIN_POLICY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_CHAIN_POLICY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_CHAIN_POLICY_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_CHAIN_POLICY_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.pvExtraPolicyPara == other.pvExtraPolicyPara
    }
}
impl ::core::cmp::Eq for CERT_CHAIN_POLICY_PARA {}
impl ::core::fmt::Debug for CERT_CHAIN_POLICY_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_CHAIN_POLICY_PARA").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("pvExtraPolicyPara", &self.pvExtraPolicyPara).finish()
    }
}
impl ::core::default::Default for CERT_CHAIN_POLICY_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_CHAIN_POLICY_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwError == other.dwError && self.lChainIndex == other.lChainIndex && self.lElementIndex == other.lElementIndex && self.pvExtraPolicyStatus == other.pvExtraPolicyStatus
    }
}
impl ::core::cmp::Eq for CERT_CHAIN_POLICY_STATUS {}
impl ::core::fmt::Debug for CERT_CHAIN_POLICY_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_CHAIN_POLICY_STATUS").field("cbSize", &self.cbSize).field("dwError", &self.dwError).field("lChainIndex", &self.lChainIndex).field("lElementIndex", &self.lElementIndex).field("pvExtraPolicyStatus", &self.pvExtraPolicyStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.dwCertEncodingType == other.dwCertEncodingType && self.pbCertEncoded == other.pbCertEncoded && self.cbCertEncoded == other.cbCertEncoded && self.pCertInfo == other.pCertInfo && self.hCertStore == other.hCertStore
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_CONTEXT").field("dwCertEncodingType", &self.dwCertEncodingType).field("pbCertEncoded", &self.pbCertEncoded).field("cbCertEncoded", &self.cbCertEncoded).field("pCertInfo", &self.pCertInfo).field("hCertStore", &self.hCertStore).finish()
    }
}
impl ::core::default::Default for CERT_CONTROL_STORE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_CONTROL_STORE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_CONTROL_STORE_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_CREATE_CONTEXT_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CERT_CREATE_SELFSIGN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_CREATE_SELFSIGN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_CREATE_SELFSIGN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CERT_CREATE_SELFSIGN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CERT_CREATE_SELFSIGN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CERT_CREATE_SELFSIGN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CERT_CREATE_SELFSIGN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CERT_CREATE_SELFSIGN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_CRL_CONTEXT_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_CRL_CONTEXT_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.pCertContext == other.pCertContext && self.pCrlContext == other.pCrlContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_CRL_CONTEXT_PAIR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_CRL_CONTEXT_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_CRL_CONTEXT_PAIR").field("pCertContext", &self.pCertContext).field("pCrlContext", &self.pCrlContext).finish()
    }
}
impl ::core::default::Default for CERT_DH_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_DH_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p && self.g == other.g
    }
}
impl ::core::cmp::Eq for CERT_DH_PARAMETERS {}
impl ::core::fmt::Debug for CERT_DH_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_DH_PARAMETERS").field("p", &self.p).field("g", &self.g).finish()
    }
}
impl ::core::default::Default for CERT_DSS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_DSS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p && self.q == other.q && self.g == other.g
    }
}
impl ::core::cmp::Eq for CERT_DSS_PARAMETERS {}
impl ::core::fmt::Debug for CERT_DSS_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_DSS_PARAMETERS").field("p", &self.p).field("q", &self.q).field("g", &self.g).finish()
    }
}
impl ::core::default::Default for CERT_ECC_SIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_ECC_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.s == other.s
    }
}
impl ::core::cmp::Eq for CERT_ECC_SIGNATURE {}
impl ::core::fmt::Debug for CERT_ECC_SIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_ECC_SIGNATURE").field("r", &self.r).field("s", &self.s).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.pszObjId == other.pszObjId && self.fCritical == other.fCritical && self.Value == other.Value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_EXTENSION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_EXTENSION").field("pszObjId", &self.pszObjId).field("fCritical", &self.fCritical).field("Value", &self.Value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_EXTENSIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_EXTENSIONS {
    fn eq(&self, other: &Self) -> bool {
        self.cExtension == other.cExtension && self.rgExtension == other.rgExtension
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_EXTENSIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_EXTENSIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_EXTENSIONS").field("cExtension", &self.cExtension).field("rgExtension", &self.rgExtension).finish()
    }
}
impl ::core::default::Default for CERT_FIND_CHAIN_IN_STORE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_FIND_CHAIN_IN_STORE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_FIND_CHAIN_IN_STORE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CERT_FIND_CHAIN_IN_STORE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CERT_FIND_CHAIN_IN_STORE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CERT_FIND_CHAIN_IN_STORE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CERT_FIND_CHAIN_IN_STORE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CERT_FIND_CHAIN_IN_STORE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CERT_FIND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_FIND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_FIND_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CERT_FIND_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CERT_FIND_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CERT_FIND_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CERT_FIND_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CERT_FIND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CERT_FIND_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_FIND_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_FIND_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_FORTEZZA_DATA_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_FORTEZZA_DATA_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.SerialNumber == other.SerialNumber && self.CertIndex == other.CertIndex && self.CertLabel == other.CertLabel
    }
}
impl ::core::cmp::Eq for CERT_FORTEZZA_DATA_PROP {}
impl ::core::fmt::Debug for CERT_FORTEZZA_DATA_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_FORTEZZA_DATA_PROP").field("SerialNumber", &self.SerialNumber).field("CertIndex", &self.CertIndex).field("CertLabel", &self.CertLabel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_GENERAL_SUBTREE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CERT_HASHED_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_HASHED_URL {
    fn eq(&self, other: &Self) -> bool {
        self.HashAlgorithm == other.HashAlgorithm && self.Hash == other.Hash && self.pwszUrl == other.pwszUrl
    }
}
impl ::core::cmp::Eq for CERT_HASHED_URL {}
impl ::core::fmt::Debug for CERT_HASHED_URL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_HASHED_URL").field("HashAlgorithm", &self.HashAlgorithm).field("Hash", &self.Hash).field("pwszUrl", &self.pwszUrl).finish()
    }
}
impl ::core::default::Default for CERT_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CERT_ID_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_ID_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_ID_OPTION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.SerialNumber == other.SerialNumber && self.SignatureAlgorithm == other.SignatureAlgorithm && self.Issuer == other.Issuer && self.NotBefore == other.NotBefore && self.NotAfter == other.NotAfter && self.Subject == other.Subject && self.SubjectPublicKeyInfo == other.SubjectPublicKeyInfo && self.IssuerUniqueId == other.IssuerUniqueId && self.SubjectUniqueId == other.SubjectUniqueId && self.cExtension == other.cExtension && self.rgExtension == other.rgExtension
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_INFO")
            .field("dwVersion", &self.dwVersion)
            .field("SerialNumber", &self.SerialNumber)
            .field("SignatureAlgorithm", &self.SignatureAlgorithm)
            .field("Issuer", &self.Issuer)
            .field("NotBefore", &self.NotBefore)
            .field("NotAfter", &self.NotAfter)
            .field("Subject", &self.Subject)
            .field("SubjectPublicKeyInfo", &self.SubjectPublicKeyInfo)
            .field("IssuerUniqueId", &self.IssuerUniqueId)
            .field("SubjectUniqueId", &self.SubjectUniqueId)
            .field("cExtension", &self.cExtension)
            .field("rgExtension", &self.rgExtension)
            .finish()
    }
}
impl ::core::default::Default for CERT_ISSUER_SERIAL_NUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_ISSUER_SERIAL_NUMBER {
    fn eq(&self, other: &Self) -> bool {
        self.Issuer == other.Issuer && self.SerialNumber == other.SerialNumber
    }
}
impl ::core::cmp::Eq for CERT_ISSUER_SERIAL_NUMBER {}
impl ::core::fmt::Debug for CERT_ISSUER_SERIAL_NUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_ISSUER_SERIAL_NUMBER").field("Issuer", &self.Issuer).field("SerialNumber", &self.SerialNumber).finish()
    }
}
impl ::core::default::Default for CERT_KEYGEN_REQUEST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_KEYGEN_REQUEST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.SubjectPublicKeyInfo == other.SubjectPublicKeyInfo && self.pwszChallengeString == other.pwszChallengeString
    }
}
impl ::core::cmp::Eq for CERT_KEYGEN_REQUEST_INFO {}
impl ::core::fmt::Debug for CERT_KEYGEN_REQUEST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_KEYGEN_REQUEST_INFO").field("dwVersion", &self.dwVersion).field("SubjectPublicKeyInfo", &self.SubjectPublicKeyInfo).field("pwszChallengeString", &self.pwszChallengeString).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_KEY_ATTRIBUTES_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_KEY_ATTRIBUTES_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.KeyId == other.KeyId && self.IntendedKeyUsage == other.IntendedKeyUsage && self.pPrivateKeyUsagePeriod == other.pPrivateKeyUsagePeriod
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_KEY_ATTRIBUTES_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_KEY_ATTRIBUTES_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_KEY_ATTRIBUTES_INFO").field("KeyId", &self.KeyId).field("IntendedKeyUsage", &self.IntendedKeyUsage).field("pPrivateKeyUsagePeriod", &self.pPrivateKeyUsagePeriod).finish()
    }
}
impl ::core::default::Default for CERT_KEY_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CERT_KEY_SPEC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_KEY_SPEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_KEY_SPEC").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_KEY_USAGE_RESTRICTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_KEY_USAGE_RESTRICTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cCertPolicyId == other.cCertPolicyId && self.rgCertPolicyId == other.rgCertPolicyId && self.RestrictedKeyUsage == other.RestrictedKeyUsage
    }
}
impl ::core::cmp::Eq for CERT_KEY_USAGE_RESTRICTION_INFO {}
impl ::core::fmt::Debug for CERT_KEY_USAGE_RESTRICTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_KEY_USAGE_RESTRICTION_INFO").field("cCertPolicyId", &self.cCertPolicyId).field("rgCertPolicyId", &self.rgCertPolicyId).field("RestrictedKeyUsage", &self.RestrictedKeyUsage).finish()
    }
}
impl ::core::default::Default for CERT_LDAP_STORE_OPENED_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_LDAP_STORE_OPENED_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.pvLdapSessionHandle == other.pvLdapSessionHandle && self.pwszLdapUrl == other.pwszLdapUrl
    }
}
impl ::core::cmp::Eq for CERT_LDAP_STORE_OPENED_PARA {}
impl ::core::fmt::Debug for CERT_LDAP_STORE_OPENED_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_LDAP_STORE_OPENED_PARA").field("pvLdapSessionHandle", &self.pvLdapSessionHandle).field("pwszLdapUrl", &self.pwszLdapUrl).finish()
    }
}
impl ::core::default::Default for CERT_LOGOTYPE_AUDIO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_LOGOTYPE_AUDIO {
    fn eq(&self, other: &Self) -> bool {
        self.LogotypeDetails == other.LogotypeDetails && self.pLogotypeAudioInfo == other.pLogotypeAudioInfo
    }
}
impl ::core::cmp::Eq for CERT_LOGOTYPE_AUDIO {}
impl ::core::fmt::Debug for CERT_LOGOTYPE_AUDIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_LOGOTYPE_AUDIO").field("LogotypeDetails", &self.LogotypeDetails).field("pLogotypeAudioInfo", &self.pLogotypeAudioInfo).finish()
    }
}
impl ::core::default::Default for CERT_LOGOTYPE_AUDIO_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_LOGOTYPE_AUDIO_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileSize == other.dwFileSize && self.dwPlayTime == other.dwPlayTime && self.dwChannels == other.dwChannels && self.dwSampleRate == other.dwSampleRate && self.pwszLanguage == other.pwszLanguage
    }
}
impl ::core::cmp::Eq for CERT_LOGOTYPE_AUDIO_INFO {}
impl ::core::fmt::Debug for CERT_LOGOTYPE_AUDIO_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_LOGOTYPE_AUDIO_INFO").field("dwFileSize", &self.dwFileSize).field("dwPlayTime", &self.dwPlayTime).field("dwChannels", &self.dwChannels).field("dwSampleRate", &self.dwSampleRate).field("pwszLanguage", &self.pwszLanguage).finish()
    }
}
impl ::core::default::Default for CERT_LOGOTYPE_CHOICE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_LOGOTYPE_CHOICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_LOGOTYPE_CHOICE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_LOGOTYPE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_LOGOTYPE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cLogotypeImage == other.cLogotypeImage && self.rgLogotypeImage == other.rgLogotypeImage && self.cLogotypeAudio == other.cLogotypeAudio && self.rgLogotypeAudio == other.rgLogotypeAudio
    }
}
impl ::core::cmp::Eq for CERT_LOGOTYPE_DATA {}
impl ::core::fmt::Debug for CERT_LOGOTYPE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_LOGOTYPE_DATA").field("cLogotypeImage", &self.cLogotypeImage).field("rgLogotypeImage", &self.rgLogotypeImage).field("cLogotypeAudio", &self.cLogotypeAudio).field("rgLogotypeAudio", &self.rgLogotypeAudio).finish()
    }
}
impl ::core::default::Default for CERT_LOGOTYPE_DETAILS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_LOGOTYPE_DETAILS {
    fn eq(&self, other: &Self) -> bool {
        self.pwszMimeType == other.pwszMimeType && self.cHashedUrl == other.cHashedUrl && self.rgHashedUrl == other.rgHashedUrl
    }
}
impl ::core::cmp::Eq for CERT_LOGOTYPE_DETAILS {}
impl ::core::fmt::Debug for CERT_LOGOTYPE_DETAILS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_LOGOTYPE_DETAILS").field("pwszMimeType", &self.pwszMimeType).field("cHashedUrl", &self.cHashedUrl).field("rgHashedUrl", &self.rgHashedUrl).finish()
    }
}
impl ::core::default::Default for CERT_LOGOTYPE_EXT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_LOGOTYPE_EXT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cCommunityLogo == other.cCommunityLogo && self.rgCommunityLogo == other.rgCommunityLogo && self.pIssuerLogo == other.pIssuerLogo && self.pSubjectLogo == other.pSubjectLogo && self.cOtherLogo == other.cOtherLogo && self.rgOtherLogo == other.rgOtherLogo
    }
}
impl ::core::cmp::Eq for CERT_LOGOTYPE_EXT_INFO {}
impl ::core::fmt::Debug for CERT_LOGOTYPE_EXT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_LOGOTYPE_EXT_INFO").field("cCommunityLogo", &self.cCommunityLogo).field("rgCommunityLogo", &self.rgCommunityLogo).field("pIssuerLogo", &self.pIssuerLogo).field("pSubjectLogo", &self.pSubjectLogo).field("cOtherLogo", &self.cOtherLogo).field("rgOtherLogo", &self.rgOtherLogo).finish()
    }
}
impl ::core::default::Default for CERT_LOGOTYPE_IMAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_LOGOTYPE_IMAGE {
    fn eq(&self, other: &Self) -> bool {
        self.LogotypeDetails == other.LogotypeDetails && self.pLogotypeImageInfo == other.pLogotypeImageInfo
    }
}
impl ::core::cmp::Eq for CERT_LOGOTYPE_IMAGE {}
impl ::core::fmt::Debug for CERT_LOGOTYPE_IMAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_LOGOTYPE_IMAGE").field("LogotypeDetails", &self.LogotypeDetails).field("pLogotypeImageInfo", &self.pLogotypeImageInfo).finish()
    }
}
impl ::core::default::Default for CERT_LOGOTYPE_IMAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CERT_LOGOTYPE_IMAGE_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_LOGOTYPE_IMAGE_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_LOGOTYPE_IMAGE_INFO_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_LOGOTYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CERT_LOGOTYPE_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_LOGOTYPE_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_LOGOTYPE_OPTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_LOGOTYPE_REFERENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_LOGOTYPE_REFERENCE {
    fn eq(&self, other: &Self) -> bool {
        self.cHashedUrl == other.cHashedUrl && self.rgHashedUrl == other.rgHashedUrl
    }
}
impl ::core::cmp::Eq for CERT_LOGOTYPE_REFERENCE {}
impl ::core::fmt::Debug for CERT_LOGOTYPE_REFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_LOGOTYPE_REFERENCE").field("cHashedUrl", &self.cHashedUrl).field("rgHashedUrl", &self.rgHashedUrl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_NAME_CONSTRAINTS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_NAME_CONSTRAINTS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cPermittedSubtree == other.cPermittedSubtree && self.rgPermittedSubtree == other.rgPermittedSubtree && self.cExcludedSubtree == other.cExcludedSubtree && self.rgExcludedSubtree == other.rgExcludedSubtree
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_NAME_CONSTRAINTS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_NAME_CONSTRAINTS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_NAME_CONSTRAINTS_INFO").field("cPermittedSubtree", &self.cPermittedSubtree).field("rgPermittedSubtree", &self.rgPermittedSubtree).field("cExcludedSubtree", &self.cExcludedSubtree).field("rgExcludedSubtree", &self.rgExcludedSubtree).finish()
    }
}
impl ::core::default::Default for CERT_NAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_NAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cRDN == other.cRDN && self.rgRDN == other.rgRDN
    }
}
impl ::core::cmp::Eq for CERT_NAME_INFO {}
impl ::core::fmt::Debug for CERT_NAME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_NAME_INFO").field("cRDN", &self.cRDN).field("rgRDN", &self.rgRDN).finish()
    }
}
impl ::core::default::Default for CERT_NAME_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_NAME_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.dwValueType == other.dwValueType && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for CERT_NAME_VALUE {}
impl ::core::fmt::Debug for CERT_NAME_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_NAME_VALUE").field("dwValueType", &self.dwValueType).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for CERT_OPEN_STORE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_OPEN_STORE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_OPEN_STORE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_OR_CRL_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_OR_CRL_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwChoice == other.dwChoice && self.cbEncoded == other.cbEncoded && self.pbEncoded == other.pbEncoded
    }
}
impl ::core::cmp::Eq for CERT_OR_CRL_BLOB {}
impl ::core::fmt::Debug for CERT_OR_CRL_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_OR_CRL_BLOB").field("dwChoice", &self.dwChoice).field("cbEncoded", &self.cbEncoded).field("pbEncoded", &self.pbEncoded).finish()
    }
}
impl ::core::default::Default for CERT_OR_CRL_BUNDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_OR_CRL_BUNDLE {
    fn eq(&self, other: &Self) -> bool {
        self.cItem == other.cItem && self.rgItem == other.rgItem
    }
}
impl ::core::cmp::Eq for CERT_OR_CRL_BUNDLE {}
impl ::core::fmt::Debug for CERT_OR_CRL_BUNDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_OR_CRL_BUNDLE").field("cItem", &self.cItem).field("rgItem", &self.rgItem).finish()
    }
}
impl ::core::default::Default for CERT_OTHER_LOGOTYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CERT_OTHER_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_OTHER_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.pszObjId == other.pszObjId && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for CERT_OTHER_NAME {}
impl ::core::fmt::Debug for CERT_OTHER_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_OTHER_NAME").field("pszObjId", &self.pszObjId).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for CERT_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.Forward == other.Forward && self.Reverse == other.Reverse
    }
}
impl ::core::cmp::Eq for CERT_PAIR {}
impl ::core::fmt::Debug for CERT_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_PAIR").field("Forward", &self.Forward).field("Reverse", &self.Reverse).finish()
    }
}
impl ::core::default::Default for CERT_PHYSICAL_STORE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_PHYSICAL_STORE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pszOpenStoreProvider == other.pszOpenStoreProvider && self.dwOpenEncodingType == other.dwOpenEncodingType && self.dwOpenFlags == other.dwOpenFlags && self.OpenParameters == other.OpenParameters && self.dwFlags == other.dwFlags && self.dwPriority == other.dwPriority
    }
}
impl ::core::cmp::Eq for CERT_PHYSICAL_STORE_INFO {}
impl ::core::fmt::Debug for CERT_PHYSICAL_STORE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_PHYSICAL_STORE_INFO").field("cbSize", &self.cbSize).field("pszOpenStoreProvider", &self.pszOpenStoreProvider).field("dwOpenEncodingType", &self.dwOpenEncodingType).field("dwOpenFlags", &self.dwOpenFlags).field("OpenParameters", &self.OpenParameters).field("dwFlags", &self.dwFlags).field("dwPriority", &self.dwPriority).finish()
    }
}
impl ::core::default::Default for CERT_POLICIES_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_POLICIES_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cPolicyInfo == other.cPolicyInfo && self.rgPolicyInfo == other.rgPolicyInfo
    }
}
impl ::core::cmp::Eq for CERT_POLICIES_INFO {}
impl ::core::fmt::Debug for CERT_POLICIES_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_POLICIES_INFO").field("cPolicyInfo", &self.cPolicyInfo).field("rgPolicyInfo", &self.rgPolicyInfo).finish()
    }
}
impl ::core::default::Default for CERT_POLICY95_QUALIFIER1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_POLICY95_QUALIFIER1 {
    fn eq(&self, other: &Self) -> bool {
        self.pszPracticesReference == other.pszPracticesReference && self.pszNoticeIdentifier == other.pszNoticeIdentifier && self.pszNSINoticeIdentifier == other.pszNSINoticeIdentifier && self.cCPSURLs == other.cCPSURLs && self.rgCPSURLs == other.rgCPSURLs
    }
}
impl ::core::cmp::Eq for CERT_POLICY95_QUALIFIER1 {}
impl ::core::fmt::Debug for CERT_POLICY95_QUALIFIER1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_POLICY95_QUALIFIER1").field("pszPracticesReference", &self.pszPracticesReference).field("pszNoticeIdentifier", &self.pszNoticeIdentifier).field("pszNSINoticeIdentifier", &self.pszNSINoticeIdentifier).field("cCPSURLs", &self.cCPSURLs).field("rgCPSURLs", &self.rgCPSURLs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_POLICY_CONSTRAINTS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_POLICY_CONSTRAINTS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.fRequireExplicitPolicy == other.fRequireExplicitPolicy && self.dwRequireExplicitPolicySkipCerts == other.dwRequireExplicitPolicySkipCerts && self.fInhibitPolicyMapping == other.fInhibitPolicyMapping && self.dwInhibitPolicyMappingSkipCerts == other.dwInhibitPolicyMappingSkipCerts
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_POLICY_CONSTRAINTS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_POLICY_CONSTRAINTS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_POLICY_CONSTRAINTS_INFO").field("fRequireExplicitPolicy", &self.fRequireExplicitPolicy).field("dwRequireExplicitPolicySkipCerts", &self.dwRequireExplicitPolicySkipCerts).field("fInhibitPolicyMapping", &self.fInhibitPolicyMapping).field("dwInhibitPolicyMappingSkipCerts", &self.dwInhibitPolicyMappingSkipCerts).finish()
    }
}
impl ::core::default::Default for CERT_POLICY_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_POLICY_ID {
    fn eq(&self, other: &Self) -> bool {
        self.cCertPolicyElementId == other.cCertPolicyElementId && self.rgpszCertPolicyElementId == other.rgpszCertPolicyElementId
    }
}
impl ::core::cmp::Eq for CERT_POLICY_ID {}
impl ::core::fmt::Debug for CERT_POLICY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_POLICY_ID").field("cCertPolicyElementId", &self.cCertPolicyElementId).field("rgpszCertPolicyElementId", &self.rgpszCertPolicyElementId).finish()
    }
}
impl ::core::default::Default for CERT_POLICY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_POLICY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszPolicyIdentifier == other.pszPolicyIdentifier && self.cPolicyQualifier == other.cPolicyQualifier && self.rgPolicyQualifier == other.rgPolicyQualifier
    }
}
impl ::core::cmp::Eq for CERT_POLICY_INFO {}
impl ::core::fmt::Debug for CERT_POLICY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_POLICY_INFO").field("pszPolicyIdentifier", &self.pszPolicyIdentifier).field("cPolicyQualifier", &self.cPolicyQualifier).field("rgPolicyQualifier", &self.rgPolicyQualifier).finish()
    }
}
impl ::core::default::Default for CERT_POLICY_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_POLICY_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.pszIssuerDomainPolicy == other.pszIssuerDomainPolicy && self.pszSubjectDomainPolicy == other.pszSubjectDomainPolicy
    }
}
impl ::core::cmp::Eq for CERT_POLICY_MAPPING {}
impl ::core::fmt::Debug for CERT_POLICY_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_POLICY_MAPPING").field("pszIssuerDomainPolicy", &self.pszIssuerDomainPolicy).field("pszSubjectDomainPolicy", &self.pszSubjectDomainPolicy).finish()
    }
}
impl ::core::default::Default for CERT_POLICY_MAPPINGS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_POLICY_MAPPINGS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cPolicyMapping == other.cPolicyMapping && self.rgPolicyMapping == other.rgPolicyMapping
    }
}
impl ::core::cmp::Eq for CERT_POLICY_MAPPINGS_INFO {}
impl ::core::fmt::Debug for CERT_POLICY_MAPPINGS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_POLICY_MAPPINGS_INFO").field("cPolicyMapping", &self.cPolicyMapping).field("rgPolicyMapping", &self.rgPolicyMapping).finish()
    }
}
impl ::core::default::Default for CERT_POLICY_QUALIFIER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_POLICY_QUALIFIER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszPolicyQualifierId == other.pszPolicyQualifierId && self.Qualifier == other.Qualifier
    }
}
impl ::core::cmp::Eq for CERT_POLICY_QUALIFIER_INFO {}
impl ::core::fmt::Debug for CERT_POLICY_QUALIFIER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_POLICY_QUALIFIER_INFO").field("pszPolicyQualifierId", &self.pszPolicyQualifierId).field("Qualifier", &self.Qualifier).finish()
    }
}
impl ::core::default::Default for CERT_POLICY_QUALIFIER_NOTICE_REFERENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_POLICY_QUALIFIER_NOTICE_REFERENCE {
    fn eq(&self, other: &Self) -> bool {
        self.pszOrganization == other.pszOrganization && self.cNoticeNumbers == other.cNoticeNumbers && self.rgNoticeNumbers == other.rgNoticeNumbers
    }
}
impl ::core::cmp::Eq for CERT_POLICY_QUALIFIER_NOTICE_REFERENCE {}
impl ::core::fmt::Debug for CERT_POLICY_QUALIFIER_NOTICE_REFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_POLICY_QUALIFIER_NOTICE_REFERENCE").field("pszOrganization", &self.pszOrganization).field("cNoticeNumbers", &self.cNoticeNumbers).field("rgNoticeNumbers", &self.rgNoticeNumbers).finish()
    }
}
impl ::core::default::Default for CERT_POLICY_QUALIFIER_USER_NOTICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_POLICY_QUALIFIER_USER_NOTICE {
    fn eq(&self, other: &Self) -> bool {
        self.pNoticeReference == other.pNoticeReference && self.pszDisplayText == other.pszDisplayText
    }
}
impl ::core::cmp::Eq for CERT_POLICY_QUALIFIER_USER_NOTICE {}
impl ::core::fmt::Debug for CERT_POLICY_QUALIFIER_USER_NOTICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_POLICY_QUALIFIER_USER_NOTICE").field("pNoticeReference", &self.pNoticeReference).field("pszDisplayText", &self.pszDisplayText).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_PRIVATE_KEY_VALIDITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_PRIVATE_KEY_VALIDITY {
    fn eq(&self, other: &Self) -> bool {
        self.NotBefore == other.NotBefore && self.NotAfter == other.NotAfter
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_PRIVATE_KEY_VALIDITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_PRIVATE_KEY_VALIDITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_PRIVATE_KEY_VALIDITY").field("NotBefore", &self.NotBefore).field("NotAfter", &self.NotAfter).finish()
    }
}
impl ::core::default::Default for CERT_PUBLIC_KEY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_PUBLIC_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Algorithm == other.Algorithm && self.PublicKey == other.PublicKey
    }
}
impl ::core::cmp::Eq for CERT_PUBLIC_KEY_INFO {}
impl ::core::fmt::Debug for CERT_PUBLIC_KEY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_PUBLIC_KEY_INFO").field("Algorithm", &self.Algorithm).field("PublicKey", &self.PublicKey).finish()
    }
}
impl ::core::default::Default for CERT_QC_STATEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_QC_STATEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.pszStatementId == other.pszStatementId && self.StatementInfo == other.StatementInfo
    }
}
impl ::core::cmp::Eq for CERT_QC_STATEMENT {}
impl ::core::fmt::Debug for CERT_QC_STATEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_QC_STATEMENT").field("pszStatementId", &self.pszStatementId).field("StatementInfo", &self.StatementInfo).finish()
    }
}
impl ::core::default::Default for CERT_QC_STATEMENTS_EXT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_QC_STATEMENTS_EXT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cStatement == other.cStatement && self.rgStatement == other.rgStatement
    }
}
impl ::core::cmp::Eq for CERT_QC_STATEMENTS_EXT_INFO {}
impl ::core::fmt::Debug for CERT_QC_STATEMENTS_EXT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_QC_STATEMENTS_EXT_INFO").field("cStatement", &self.cStatement).field("rgStatement", &self.rgStatement).finish()
    }
}
impl ::core::default::Default for CERT_QUERY_CONTENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_QUERY_CONTENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_QUERY_CONTENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_QUERY_CONTENT_TYPE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_QUERY_CONTENT_TYPE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_QUERY_CONTENT_TYPE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_QUERY_ENCODING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_QUERY_ENCODING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_QUERY_ENCODING_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CERT_QUERY_ENCODING_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CERT_QUERY_ENCODING_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CERT_QUERY_ENCODING_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CERT_QUERY_ENCODING_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CERT_QUERY_ENCODING_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CERT_QUERY_FORMAT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_QUERY_FORMAT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_QUERY_FORMAT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_QUERY_FORMAT_TYPE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_QUERY_FORMAT_TYPE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_QUERY_FORMAT_TYPE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_QUERY_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_QUERY_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_QUERY_OBJECT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_RDN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_RDN {
    fn eq(&self, other: &Self) -> bool {
        self.cRDNAttr == other.cRDNAttr && self.rgRDNAttr == other.rgRDNAttr
    }
}
impl ::core::cmp::Eq for CERT_RDN {}
impl ::core::fmt::Debug for CERT_RDN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_RDN").field("cRDNAttr", &self.cRDNAttr).field("rgRDNAttr", &self.rgRDNAttr).finish()
    }
}
impl ::core::default::Default for CERT_RDN_ATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_RDN_ATTR {
    fn eq(&self, other: &Self) -> bool {
        self.pszObjId == other.pszObjId && self.dwValueType == other.dwValueType && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for CERT_RDN_ATTR {}
impl ::core::fmt::Debug for CERT_RDN_ATTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_RDN_ATTR").field("pszObjId", &self.pszObjId).field("dwValueType", &self.dwValueType).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for CERT_RDN_ATTR_VALUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_RDN_ATTR_VALUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_RDN_ATTR_VALUE_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::default::Default for CERT_REGISTRY_STORE_CLIENT_GPT_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::cmp::PartialEq for CERT_REGISTRY_STORE_CLIENT_GPT_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.hKeyBase == other.hKeyBase && self.pwszRegPath == other.pwszRegPath
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::cmp::Eq for CERT_REGISTRY_STORE_CLIENT_GPT_PARA {}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::fmt::Debug for CERT_REGISTRY_STORE_CLIENT_GPT_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_REGISTRY_STORE_CLIENT_GPT_PARA").field("hKeyBase", &self.hKeyBase).field("pwszRegPath", &self.pwszRegPath).finish()
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::default::Default for CERT_REGISTRY_STORE_ROAMING_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::cmp::PartialEq for CERT_REGISTRY_STORE_ROAMING_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.hKey == other.hKey && self.pwszStoreDirectory == other.pwszStoreDirectory
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::cmp::Eq for CERT_REGISTRY_STORE_ROAMING_PARA {}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::fmt::Debug for CERT_REGISTRY_STORE_ROAMING_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_REGISTRY_STORE_ROAMING_PARA").field("hKey", &self.hKey).field("pwszStoreDirectory", &self.pwszStoreDirectory).finish()
    }
}
impl ::core::default::Default for CERT_REQUEST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_REQUEST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.Subject == other.Subject && self.SubjectPublicKeyInfo == other.SubjectPublicKeyInfo && self.cAttribute == other.cAttribute && self.rgAttribute == other.rgAttribute
    }
}
impl ::core::cmp::Eq for CERT_REQUEST_INFO {}
impl ::core::fmt::Debug for CERT_REQUEST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_REQUEST_INFO").field("dwVersion", &self.dwVersion).field("Subject", &self.Subject).field("SubjectPublicKeyInfo", &self.SubjectPublicKeyInfo).field("cAttribute", &self.cAttribute).field("rgAttribute", &self.rgAttribute).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_REVOCATION_CHAIN_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_REVOCATION_CHAIN_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hChainEngine == other.hChainEngine && self.hAdditionalStore == other.hAdditionalStore && self.dwChainFlags == other.dwChainFlags && self.dwUrlRetrievalTimeout == other.dwUrlRetrievalTimeout && self.pftCurrentTime == other.pftCurrentTime && self.pftCacheResync == other.pftCacheResync && self.cbMaxUrlRetrievalByteCount == other.cbMaxUrlRetrievalByteCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_REVOCATION_CHAIN_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_REVOCATION_CHAIN_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_REVOCATION_CHAIN_PARA").field("cbSize", &self.cbSize).field("hChainEngine", &self.hChainEngine).field("hAdditionalStore", &self.hAdditionalStore).field("dwChainFlags", &self.dwChainFlags).field("dwUrlRetrievalTimeout", &self.dwUrlRetrievalTimeout).field("pftCurrentTime", &self.pftCurrentTime).field("pftCacheResync", &self.pftCacheResync).field("cbMaxUrlRetrievalByteCount", &self.cbMaxUrlRetrievalByteCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_REVOCATION_CRL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_REVOCATION_CRL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pBaseCrlContext == other.pBaseCrlContext && self.pDeltaCrlContext == other.pDeltaCrlContext && self.pCrlEntry == other.pCrlEntry && self.fDeltaCrlEntry == other.fDeltaCrlEntry
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_REVOCATION_CRL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_REVOCATION_CRL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_REVOCATION_CRL_INFO").field("cbSize", &self.cbSize).field("pBaseCrlContext", &self.pBaseCrlContext).field("pDeltaCrlContext", &self.pDeltaCrlContext).field("pCrlEntry", &self.pCrlEntry).field("fDeltaCrlEntry", &self.fDeltaCrlEntry).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_REVOCATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_REVOCATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwRevocationResult == other.dwRevocationResult && self.pszRevocationOid == other.pszRevocationOid && self.pvOidSpecificInfo == other.pvOidSpecificInfo && self.fHasFreshnessTime == other.fHasFreshnessTime && self.dwFreshnessTime == other.dwFreshnessTime && self.pCrlInfo == other.pCrlInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_REVOCATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_REVOCATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_REVOCATION_INFO").field("cbSize", &self.cbSize).field("dwRevocationResult", &self.dwRevocationResult).field("pszRevocationOid", &self.pszRevocationOid).field("pvOidSpecificInfo", &self.pvOidSpecificInfo).field("fHasFreshnessTime", &self.fHasFreshnessTime).field("dwFreshnessTime", &self.dwFreshnessTime).field("pCrlInfo", &self.pCrlInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_REVOCATION_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_REVOCATION_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pIssuerCert == other.pIssuerCert && self.cCertStore == other.cCertStore && self.rgCertStore == other.rgCertStore && self.hCrlStore == other.hCrlStore && self.pftTimeToUse == other.pftTimeToUse
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_REVOCATION_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_REVOCATION_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_REVOCATION_PARA").field("cbSize", &self.cbSize).field("pIssuerCert", &self.pIssuerCert).field("cCertStore", &self.cCertStore).field("rgCertStore", &self.rgCertStore).field("hCrlStore", &self.hCrlStore).field("pftTimeToUse", &self.pftTimeToUse).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_REVOCATION_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_REVOCATION_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwIndex == other.dwIndex && self.dwError == other.dwError && self.dwReason == other.dwReason && self.fHasFreshnessTime == other.fHasFreshnessTime && self.dwFreshnessTime == other.dwFreshnessTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_REVOCATION_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_REVOCATION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_REVOCATION_STATUS").field("cbSize", &self.cbSize).field("dwIndex", &self.dwIndex).field("dwError", &self.dwError).field("dwReason", &self.dwReason).field("fHasFreshnessTime", &self.fHasFreshnessTime).field("dwFreshnessTime", &self.dwFreshnessTime).finish()
    }
}
impl ::core::default::Default for CERT_REVOCATION_STATUS_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_REVOCATION_STATUS_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_REVOCATION_STATUS_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_ROOT_PROGRAM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_ROOT_PROGRAM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_ROOT_PROGRAM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CERT_ROOT_PROGRAM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CERT_ROOT_PROGRAM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CERT_ROOT_PROGRAM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CERT_ROOT_PROGRAM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CERT_ROOT_PROGRAM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_SELECT_CHAIN_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_SELECT_CHAIN_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.hChainEngine == other.hChainEngine && self.pTime == other.pTime && self.hAdditionalStore == other.hAdditionalStore && self.pChainPara == other.pChainPara && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_SELECT_CHAIN_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_SELECT_CHAIN_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_SELECT_CHAIN_PARA").field("hChainEngine", &self.hChainEngine).field("pTime", &self.pTime).field("hAdditionalStore", &self.hAdditionalStore).field("pChainPara", &self.pChainPara).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for CERT_SELECT_CRITERIA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_SELECT_CRITERIA {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.cPara == other.cPara && self.ppPara == other.ppPara
    }
}
impl ::core::cmp::Eq for CERT_SELECT_CRITERIA {}
impl ::core::fmt::Debug for CERT_SELECT_CRITERIA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_SELECT_CRITERIA").field("dwType", &self.dwType).field("cPara", &self.cPara).field("ppPara", &self.ppPara).finish()
    }
}
impl ::core::default::Default for CERT_SELECT_CRITERIA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_SELECT_CRITERIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_SELECT_CRITERIA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_SERVER_OCSP_RESPONSE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_SERVER_OCSP_RESPONSE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pbEncodedOcspResponse == other.pbEncodedOcspResponse && self.cbEncodedOcspResponse == other.cbEncodedOcspResponse
    }
}
impl ::core::cmp::Eq for CERT_SERVER_OCSP_RESPONSE_CONTEXT {}
impl ::core::fmt::Debug for CERT_SERVER_OCSP_RESPONSE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_SERVER_OCSP_RESPONSE_CONTEXT").field("cbSize", &self.cbSize).field("pbEncodedOcspResponse", &self.pbEncodedOcspResponse).field("cbEncodedOcspResponse", &self.cbEncodedOcspResponse).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_SERVER_OCSP_RESPONSE_OPEN_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CERT_SIGNED_CONTENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_SIGNED_CONTENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ToBeSigned == other.ToBeSigned && self.SignatureAlgorithm == other.SignatureAlgorithm && self.Signature == other.Signature
    }
}
impl ::core::cmp::Eq for CERT_SIGNED_CONTENT_INFO {}
impl ::core::fmt::Debug for CERT_SIGNED_CONTENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_SIGNED_CONTENT_INFO").field("ToBeSigned", &self.ToBeSigned).field("SignatureAlgorithm", &self.SignatureAlgorithm).field("Signature", &self.Signature).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_SIMPLE_CHAIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_SIMPLE_CHAIN {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.TrustStatus == other.TrustStatus && self.cElement == other.cElement && self.rgpElement == other.rgpElement && self.pTrustListInfo == other.pTrustListInfo && self.fHasRevocationFreshnessTime == other.fHasRevocationFreshnessTime && self.dwRevocationFreshnessTime == other.dwRevocationFreshnessTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_SIMPLE_CHAIN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_SIMPLE_CHAIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_SIMPLE_CHAIN").field("cbSize", &self.cbSize).field("TrustStatus", &self.TrustStatus).field("cElement", &self.cElement).field("rgpElement", &self.rgpElement).field("pTrustListInfo", &self.pTrustListInfo).field("fHasRevocationFreshnessTime", &self.fHasRevocationFreshnessTime).field("dwRevocationFreshnessTime", &self.dwRevocationFreshnessTime).finish()
    }
}
impl ::core::default::Default for CERT_STORE_PROV_FIND_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_STORE_PROV_FIND_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMsgAndCertEncodingType == other.dwMsgAndCertEncodingType && self.dwFindFlags == other.dwFindFlags && self.dwFindType == other.dwFindType && self.pvFindPara == other.pvFindPara
    }
}
impl ::core::cmp::Eq for CERT_STORE_PROV_FIND_INFO {}
impl ::core::fmt::Debug for CERT_STORE_PROV_FIND_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_STORE_PROV_FIND_INFO").field("cbSize", &self.cbSize).field("dwMsgAndCertEncodingType", &self.dwMsgAndCertEncodingType).field("dwFindFlags", &self.dwFindFlags).field("dwFindType", &self.dwFindType).field("pvFindPara", &self.pvFindPara).finish()
    }
}
impl ::core::default::Default for CERT_STORE_PROV_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_STORE_PROV_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_STORE_PROV_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CERT_STORE_PROV_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CERT_STORE_PROV_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CERT_STORE_PROV_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CERT_STORE_PROV_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CERT_STORE_PROV_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CERT_STORE_PROV_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_STORE_PROV_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cStoreProvFunc == other.cStoreProvFunc && self.rgpvStoreProvFunc == other.rgpvStoreProvFunc && self.hStoreProv == other.hStoreProv && self.dwStoreProvFlags == other.dwStoreProvFlags && self.hStoreProvFuncAddr2 == other.hStoreProvFuncAddr2
    }
}
impl ::core::cmp::Eq for CERT_STORE_PROV_INFO {}
impl ::core::fmt::Debug for CERT_STORE_PROV_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_STORE_PROV_INFO").field("cbSize", &self.cbSize).field("cStoreProvFunc", &self.cStoreProvFunc).field("rgpvStoreProvFunc", &self.rgpvStoreProvFunc).field("hStoreProv", &self.hStoreProv).field("dwStoreProvFlags", &self.dwStoreProvFlags).field("hStoreProvFuncAddr2", &self.hStoreProvFuncAddr2).finish()
    }
}
impl ::core::default::Default for CERT_STORE_SAVE_AS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_STORE_SAVE_AS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_STORE_SAVE_AS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_STORE_SAVE_TO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_STORE_SAVE_TO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_STORE_SAVE_TO").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_STRING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_STRING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_STRING_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_STRONG_SIGN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_STRONG_SIGN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_STRONG_SIGN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CERT_STRONG_SIGN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CERT_STRONG_SIGN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CERT_STRONG_SIGN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CERT_STRONG_SIGN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CERT_STRONG_SIGN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CERT_STRONG_SIGN_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CERT_STRONG_SIGN_SERIALIZED_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_STRONG_SIGN_SERIALIZED_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.pwszCNGSignHashAlgids == other.pwszCNGSignHashAlgids && self.pwszCNGPubKeyMinBitLengths == other.pwszCNGPubKeyMinBitLengths
    }
}
impl ::core::cmp::Eq for CERT_STRONG_SIGN_SERIALIZED_INFO {}
impl ::core::fmt::Debug for CERT_STRONG_SIGN_SERIALIZED_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_STRONG_SIGN_SERIALIZED_INFO").field("dwFlags", &self.dwFlags).field("pwszCNGSignHashAlgids", &self.pwszCNGSignHashAlgids).field("pwszCNGPubKeyMinBitLengths", &self.pwszCNGPubKeyMinBitLengths).finish()
    }
}
impl ::core::default::Default for CERT_SUPPORTED_ALGORITHM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_SUPPORTED_ALGORITHM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Algorithm == other.Algorithm && self.IntendedKeyUsage == other.IntendedKeyUsage && self.IntendedCertPolicies == other.IntendedCertPolicies
    }
}
impl ::core::cmp::Eq for CERT_SUPPORTED_ALGORITHM_INFO {}
impl ::core::fmt::Debug for CERT_SUPPORTED_ALGORITHM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_SUPPORTED_ALGORITHM_INFO").field("Algorithm", &self.Algorithm).field("IntendedKeyUsage", &self.IntendedKeyUsage).field("IntendedCertPolicies", &self.IntendedCertPolicies).finish()
    }
}
impl ::core::default::Default for CERT_SYSTEM_STORE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_SYSTEM_STORE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_SYSTEM_STORE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_SYSTEM_STORE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_SYSTEM_STORE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
    }
}
impl ::core::cmp::Eq for CERT_SYSTEM_STORE_INFO {}
impl ::core::fmt::Debug for CERT_SYSTEM_STORE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_SYSTEM_STORE_INFO").field("cbSize", &self.cbSize).finish()
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::default::Default for CERT_SYSTEM_STORE_RELOCATE_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_TEMPLATE_EXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_TEMPLATE_EXT {
    fn eq(&self, other: &Self) -> bool {
        self.pszObjId == other.pszObjId && self.dwMajorVersion == other.dwMajorVersion && self.fMinorVersion == other.fMinorVersion && self.dwMinorVersion == other.dwMinorVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_TEMPLATE_EXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_TEMPLATE_EXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_TEMPLATE_EXT").field("pszObjId", &self.pszObjId).field("dwMajorVersion", &self.dwMajorVersion).field("fMinorVersion", &self.fMinorVersion).field("dwMinorVersion", &self.dwMinorVersion).finish()
    }
}
impl ::core::default::Default for CERT_TPM_SPECIFICATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_TPM_SPECIFICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwszFamily == other.pwszFamily && self.dwLevel == other.dwLevel && self.dwRevision == other.dwRevision
    }
}
impl ::core::cmp::Eq for CERT_TPM_SPECIFICATION_INFO {}
impl ::core::fmt::Debug for CERT_TPM_SPECIFICATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_TPM_SPECIFICATION_INFO").field("pwszFamily", &self.pwszFamily).field("dwLevel", &self.dwLevel).field("dwRevision", &self.dwRevision).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_TRUST_LIST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_TRUST_LIST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pCtlEntry == other.pCtlEntry && self.pCtlContext == other.pCtlContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_TRUST_LIST_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_TRUST_LIST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_TRUST_LIST_INFO").field("cbSize", &self.cbSize).field("pCtlEntry", &self.pCtlEntry).field("pCtlContext", &self.pCtlContext).finish()
    }
}
impl ::core::default::Default for CERT_TRUST_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_TRUST_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwErrorStatus == other.dwErrorStatus && self.dwInfoStatus == other.dwInfoStatus
    }
}
impl ::core::cmp::Eq for CERT_TRUST_STATUS {}
impl ::core::fmt::Debug for CERT_TRUST_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_TRUST_STATUS").field("dwErrorStatus", &self.dwErrorStatus).field("dwInfoStatus", &self.dwInfoStatus).finish()
    }
}
impl ::core::default::Default for CERT_USAGE_MATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_USAGE_MATCH {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.Usage == other.Usage
    }
}
impl ::core::cmp::Eq for CERT_USAGE_MATCH {}
impl ::core::fmt::Debug for CERT_USAGE_MATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_USAGE_MATCH").field("dwType", &self.dwType).field("Usage", &self.Usage).finish()
    }
}
impl ::core::default::Default for CERT_X942_DH_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_X942_DH_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p && self.g == other.g && self.q == other.q && self.j == other.j && self.pValidationParams == other.pValidationParams
    }
}
impl ::core::cmp::Eq for CERT_X942_DH_PARAMETERS {}
impl ::core::fmt::Debug for CERT_X942_DH_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_X942_DH_PARAMETERS").field("p", &self.p).field("g", &self.g).field("q", &self.q).field("j", &self.j).field("pValidationParams", &self.pValidationParams).finish()
    }
}
impl ::core::default::Default for CERT_X942_DH_VALIDATION_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_X942_DH_VALIDATION_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.seed == other.seed && self.pgenCounter == other.pgenCounter
    }
}
impl ::core::cmp::Eq for CERT_X942_DH_VALIDATION_PARAMS {}
impl ::core::fmt::Debug for CERT_X942_DH_VALIDATION_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_X942_DH_VALIDATION_PARAMS").field("seed", &self.seed).field("pgenCounter", &self.pgenCounter).finish()
    }
}
impl ::core::default::Default for CESSetupProperty {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CESSetupProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CESSetupProperty").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLAIMLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLAIMLIST {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.claims == other.claims
    }
}
impl ::core::cmp::Eq for CLAIMLIST {}
impl ::core::fmt::Debug for CLAIMLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLAIMLIST").field("count", &self.count).field("claims", &self.claims).finish()
    }
}
impl ::core::default::Default for CMC_ADD_ATTRIBUTES_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMC_ADD_ATTRIBUTES_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwCmcDataReference == other.dwCmcDataReference && self.cCertReference == other.cCertReference && self.rgdwCertReference == other.rgdwCertReference && self.cAttribute == other.cAttribute && self.rgAttribute == other.rgAttribute
    }
}
impl ::core::cmp::Eq for CMC_ADD_ATTRIBUTES_INFO {}
impl ::core::fmt::Debug for CMC_ADD_ATTRIBUTES_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMC_ADD_ATTRIBUTES_INFO").field("dwCmcDataReference", &self.dwCmcDataReference).field("cCertReference", &self.cCertReference).field("rgdwCertReference", &self.rgdwCertReference).field("cAttribute", &self.cAttribute).field("rgAttribute", &self.rgAttribute).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMC_ADD_EXTENSIONS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CMC_ADD_EXTENSIONS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwCmcDataReference == other.dwCmcDataReference && self.cCertReference == other.cCertReference && self.rgdwCertReference == other.rgdwCertReference && self.cExtension == other.cExtension && self.rgExtension == other.rgExtension
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CMC_ADD_EXTENSIONS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CMC_ADD_EXTENSIONS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMC_ADD_EXTENSIONS_INFO").field("dwCmcDataReference", &self.dwCmcDataReference).field("cCertReference", &self.cCertReference).field("rgdwCertReference", &self.rgdwCertReference).field("cExtension", &self.cExtension).field("rgExtension", &self.rgExtension).finish()
    }
}
impl ::core::default::Default for CMC_DATA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMC_DATA_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cTaggedAttribute == other.cTaggedAttribute && self.rgTaggedAttribute == other.rgTaggedAttribute && self.cTaggedRequest == other.cTaggedRequest && self.rgTaggedRequest == other.rgTaggedRequest && self.cTaggedContentInfo == other.cTaggedContentInfo && self.rgTaggedContentInfo == other.rgTaggedContentInfo && self.cTaggedOtherMsg == other.cTaggedOtherMsg && self.rgTaggedOtherMsg == other.rgTaggedOtherMsg
    }
}
impl ::core::cmp::Eq for CMC_DATA_INFO {}
impl ::core::fmt::Debug for CMC_DATA_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMC_DATA_INFO").field("cTaggedAttribute", &self.cTaggedAttribute).field("rgTaggedAttribute", &self.rgTaggedAttribute).field("cTaggedRequest", &self.cTaggedRequest).field("rgTaggedRequest", &self.rgTaggedRequest).field("cTaggedContentInfo", &self.cTaggedContentInfo).field("rgTaggedContentInfo", &self.rgTaggedContentInfo).field("cTaggedOtherMsg", &self.cTaggedOtherMsg).field("rgTaggedOtherMsg", &self.rgTaggedOtherMsg).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMC_PEND_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CMC_PEND_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PendToken == other.PendToken && self.PendTime == other.PendTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CMC_PEND_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CMC_PEND_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMC_PEND_INFO").field("PendToken", &self.PendToken).field("PendTime", &self.PendTime).finish()
    }
}
impl ::core::default::Default for CMC_RESPONSE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMC_RESPONSE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cTaggedAttribute == other.cTaggedAttribute && self.rgTaggedAttribute == other.rgTaggedAttribute && self.cTaggedContentInfo == other.cTaggedContentInfo && self.rgTaggedContentInfo == other.rgTaggedContentInfo && self.cTaggedOtherMsg == other.cTaggedOtherMsg && self.rgTaggedOtherMsg == other.rgTaggedOtherMsg
    }
}
impl ::core::cmp::Eq for CMC_RESPONSE_INFO {}
impl ::core::fmt::Debug for CMC_RESPONSE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMC_RESPONSE_INFO").field("cTaggedAttribute", &self.cTaggedAttribute).field("rgTaggedAttribute", &self.rgTaggedAttribute).field("cTaggedContentInfo", &self.cTaggedContentInfo).field("rgTaggedContentInfo", &self.rgTaggedContentInfo).field("cTaggedOtherMsg", &self.cTaggedOtherMsg).field("rgTaggedOtherMsg", &self.rgTaggedOtherMsg).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMC_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CMC_TAGGED_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMC_TAGGED_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.dwBodyPartID == other.dwBodyPartID && self.Attribute == other.Attribute
    }
}
impl ::core::cmp::Eq for CMC_TAGGED_ATTRIBUTE {}
impl ::core::fmt::Debug for CMC_TAGGED_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMC_TAGGED_ATTRIBUTE").field("dwBodyPartID", &self.dwBodyPartID).field("Attribute", &self.Attribute).finish()
    }
}
impl ::core::default::Default for CMC_TAGGED_CERT_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMC_TAGGED_CERT_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.dwBodyPartID == other.dwBodyPartID && self.SignedCertRequest == other.SignedCertRequest
    }
}
impl ::core::cmp::Eq for CMC_TAGGED_CERT_REQUEST {}
impl ::core::fmt::Debug for CMC_TAGGED_CERT_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMC_TAGGED_CERT_REQUEST").field("dwBodyPartID", &self.dwBodyPartID).field("SignedCertRequest", &self.SignedCertRequest).finish()
    }
}
impl ::core::default::Default for CMC_TAGGED_CONTENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMC_TAGGED_CONTENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwBodyPartID == other.dwBodyPartID && self.EncodedContentInfo == other.EncodedContentInfo
    }
}
impl ::core::cmp::Eq for CMC_TAGGED_CONTENT_INFO {}
impl ::core::fmt::Debug for CMC_TAGGED_CONTENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMC_TAGGED_CONTENT_INFO").field("dwBodyPartID", &self.dwBodyPartID).field("EncodedContentInfo", &self.EncodedContentInfo).finish()
    }
}
impl ::core::default::Default for CMC_TAGGED_OTHER_MSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMC_TAGGED_OTHER_MSG {
    fn eq(&self, other: &Self) -> bool {
        self.dwBodyPartID == other.dwBodyPartID && self.pszObjId == other.pszObjId && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for CMC_TAGGED_OTHER_MSG {}
impl ::core::fmt::Debug for CMC_TAGGED_OTHER_MSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMC_TAGGED_OTHER_MSG").field("dwBodyPartID", &self.dwBodyPartID).field("pszObjId", &self.pszObjId).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for CMC_TAGGED_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMSG_CMS_RECIPIENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CMSG_CMS_SIGNER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CMSG_CNG_CONTENT_DECRYPT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMSG_CONTENT_ENCRYPT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwSignerIndex == other.dwSignerIndex && self.blob == other.blob
    }
}
impl ::core::cmp::Eq for CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA {}
impl ::core::fmt::Debug for CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA").field("cbSize", &self.cbSize).field("dwSignerIndex", &self.dwSignerIndex).field("blob", &self.blob).finish()
    }
}
impl ::core::default::Default for CMSG_CTRL_DECRYPT_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwSignerIndex == other.dwSignerIndex && self.dwUnauthAttrIndex == other.dwUnauthAttrIndex
    }
}
impl ::core::cmp::Eq for CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA {}
impl ::core::fmt::Debug for CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA").field("cbSize", &self.cbSize).field("dwSignerIndex", &self.dwSignerIndex).field("dwUnauthAttrIndex", &self.dwUnauthAttrIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMSG_CTRL_KEY_AGREE_DECRYPT_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CMSG_CTRL_KEY_TRANS_DECRYPT_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMSG_CTRL_MAIL_LIST_DECRYPT_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hCryptProv == other.hCryptProv && self.dwSignerIndex == other.dwSignerIndex && self.dwSignerType == other.dwSignerType && self.pvSigner == other.pvSigner
    }
}
impl ::core::cmp::Eq for CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA {}
impl ::core::fmt::Debug for CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA").field("cbSize", &self.cbSize).field("hCryptProv", &self.hCryptProv).field("dwSignerIndex", &self.dwSignerIndex).field("dwSignerType", &self.dwSignerType).field("pvSigner", &self.pvSigner).finish()
    }
}
impl ::core::default::Default for CMSG_ENCRYPTED_ENCODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMSG_ENCRYPTED_ENCODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ContentEncryptionAlgorithm == other.ContentEncryptionAlgorithm && self.pvEncryptionAuxInfo == other.pvEncryptionAuxInfo
    }
}
impl ::core::cmp::Eq for CMSG_ENCRYPTED_ENCODE_INFO {}
impl ::core::fmt::Debug for CMSG_ENCRYPTED_ENCODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMSG_ENCRYPTED_ENCODE_INFO").field("cbSize", &self.cbSize).field("ContentEncryptionAlgorithm", &self.ContentEncryptionAlgorithm).field("pvEncryptionAuxInfo", &self.pvEncryptionAuxInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMSG_ENVELOPED_ENCODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CMSG_ENVELOPED_ENCODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hCryptProv == other.hCryptProv && self.ContentEncryptionAlgorithm == other.ContentEncryptionAlgorithm && self.pvEncryptionAuxInfo == other.pvEncryptionAuxInfo && self.cRecipients == other.cRecipients && self.rgpRecipients == other.rgpRecipients
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CMSG_ENVELOPED_ENCODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CMSG_ENVELOPED_ENCODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMSG_ENVELOPED_ENCODE_INFO").field("cbSize", &self.cbSize).field("hCryptProv", &self.hCryptProv).field("ContentEncryptionAlgorithm", &self.ContentEncryptionAlgorithm).field("pvEncryptionAuxInfo", &self.pvEncryptionAuxInfo).field("cRecipients", &self.cRecipients).field("rgpRecipients", &self.rgpRecipients).finish()
    }
}
impl ::core::default::Default for CMSG_HASHED_ENCODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMSG_HASHED_ENCODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hCryptProv == other.hCryptProv && self.HashAlgorithm == other.HashAlgorithm && self.pvHashAuxInfo == other.pvHashAuxInfo
    }
}
impl ::core::cmp::Eq for CMSG_HASHED_ENCODE_INFO {}
impl ::core::fmt::Debug for CMSG_HASHED_ENCODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMSG_HASHED_ENCODE_INFO").field("cbSize", &self.cbSize).field("hCryptProv", &self.hCryptProv).field("HashAlgorithm", &self.HashAlgorithm).field("pvHashAuxInfo", &self.pvHashAuxInfo).finish()
    }
}
impl ::core::default::Default for CMSG_KEY_AGREE_ENCRYPT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CMSG_KEY_AGREE_KEY_ENCRYPT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMSG_KEY_AGREE_KEY_ENCRYPT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.EncryptedKey == other.EncryptedKey
    }
}
impl ::core::cmp::Eq for CMSG_KEY_AGREE_KEY_ENCRYPT_INFO {}
impl ::core::fmt::Debug for CMSG_KEY_AGREE_KEY_ENCRYPT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMSG_KEY_AGREE_KEY_ENCRYPT_INFO").field("cbSize", &self.cbSize).field("EncryptedKey", &self.EncryptedKey).finish()
    }
}
impl ::core::default::Default for CMSG_KEY_AGREE_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CMSG_KEY_AGREE_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CMSG_KEY_AGREE_OPTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for CMSG_KEY_AGREE_ORIGINATOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CMSG_KEY_AGREE_ORIGINATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CMSG_KEY_AGREE_ORIGINATOR").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMSG_KEY_AGREE_RECIPIENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CMSG_KEY_TRANS_ENCRYPT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMSG_KEY_TRANS_ENCRYPT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwRecipientIndex == other.dwRecipientIndex && self.KeyEncryptionAlgorithm == other.KeyEncryptionAlgorithm && self.EncryptedKey == other.EncryptedKey && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for CMSG_KEY_TRANS_ENCRYPT_INFO {}
impl ::core::fmt::Debug for CMSG_KEY_TRANS_ENCRYPT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMSG_KEY_TRANS_ENCRYPT_INFO").field("cbSize", &self.cbSize).field("dwRecipientIndex", &self.dwRecipientIndex).field("KeyEncryptionAlgorithm", &self.KeyEncryptionAlgorithm).field("EncryptedKey", &self.EncryptedKey).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CMSG_KEY_TRANS_RECIPIENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CMSG_MAIL_LIST_ENCRYPT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMSG_MAIL_LIST_ENCRYPT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwRecipientIndex == other.dwRecipientIndex && self.KeyEncryptionAlgorithm == other.KeyEncryptionAlgorithm && self.EncryptedKey == other.EncryptedKey && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for CMSG_MAIL_LIST_ENCRYPT_INFO {}
impl ::core::fmt::Debug for CMSG_MAIL_LIST_ENCRYPT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMSG_MAIL_LIST_ENCRYPT_INFO").field("cbSize", &self.cbSize).field("dwRecipientIndex", &self.dwRecipientIndex).field("KeyEncryptionAlgorithm", &self.KeyEncryptionAlgorithm).field("EncryptedKey", &self.EncryptedKey).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMSG_MAIL_LIST_RECIPIENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CMSG_MAIL_LIST_RECIPIENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.KeyId == other.KeyId && self.KeyEncryptionAlgorithm == other.KeyEncryptionAlgorithm && self.EncryptedKey == other.EncryptedKey && self.Date == other.Date && self.pOtherAttr == other.pOtherAttr
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CMSG_MAIL_LIST_RECIPIENT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CMSG_MAIL_LIST_RECIPIENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMSG_MAIL_LIST_RECIPIENT_INFO").field("dwVersion", &self.dwVersion).field("KeyId", &self.KeyId).field("KeyEncryptionAlgorithm", &self.KeyEncryptionAlgorithm).field("EncryptedKey", &self.EncryptedKey).field("Date", &self.Date).field("pOtherAttr", &self.pOtherAttr).finish()
    }
}
impl ::core::default::Default for CMSG_RC2_AUX_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMSG_RC2_AUX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwBitLen == other.dwBitLen
    }
}
impl ::core::cmp::Eq for CMSG_RC2_AUX_INFO {}
impl ::core::fmt::Debug for CMSG_RC2_AUX_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMSG_RC2_AUX_INFO").field("cbSize", &self.cbSize).field("dwBitLen", &self.dwBitLen).finish()
    }
}
impl ::core::default::Default for CMSG_RC4_AUX_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMSG_RC4_AUX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwBitLen == other.dwBitLen
    }
}
impl ::core::cmp::Eq for CMSG_RC4_AUX_INFO {}
impl ::core::fmt::Debug for CMSG_RC4_AUX_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMSG_RC4_AUX_INFO").field("cbSize", &self.cbSize).field("dwBitLen", &self.dwBitLen).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMSG_RECIPIENT_ENCODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMSG_RECIPIENT_ENCRYPTED_KEY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.SignedInfo == other.SignedInfo && self.EnvelopedInfo == other.EnvelopedInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO").field("cbSize", &self.cbSize).field("SignedInfo", &self.SignedInfo).field("EnvelopedInfo", &self.EnvelopedInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMSG_SIGNED_ENCODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CMSG_SIGNED_ENCODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cSigners == other.cSigners && self.rgSigners == other.rgSigners && self.cCertEncoded == other.cCertEncoded && self.rgCertEncoded == other.rgCertEncoded && self.cCrlEncoded == other.cCrlEncoded && self.rgCrlEncoded == other.rgCrlEncoded
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CMSG_SIGNED_ENCODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CMSG_SIGNED_ENCODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMSG_SIGNED_ENCODE_INFO").field("cbSize", &self.cbSize).field("cSigners", &self.cSigners).field("rgSigners", &self.rgSigners).field("cCertEncoded", &self.cCertEncoded).field("rgCertEncoded", &self.rgCertEncoded).field("cCrlEncoded", &self.cCrlEncoded).field("rgCrlEncoded", &self.rgCrlEncoded).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMSG_SIGNER_ENCODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CMSG_SIGNER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMSG_SIGNER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.Issuer == other.Issuer && self.SerialNumber == other.SerialNumber && self.HashAlgorithm == other.HashAlgorithm && self.HashEncryptionAlgorithm == other.HashEncryptionAlgorithm && self.EncryptedHash == other.EncryptedHash && self.AuthAttrs == other.AuthAttrs && self.UnauthAttrs == other.UnauthAttrs
    }
}
impl ::core::cmp::Eq for CMSG_SIGNER_INFO {}
impl ::core::fmt::Debug for CMSG_SIGNER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMSG_SIGNER_INFO").field("dwVersion", &self.dwVersion).field("Issuer", &self.Issuer).field("SerialNumber", &self.SerialNumber).field("HashAlgorithm", &self.HashAlgorithm).field("HashEncryptionAlgorithm", &self.HashEncryptionAlgorithm).field("EncryptedHash", &self.EncryptedHash).field("AuthAttrs", &self.AuthAttrs).field("UnauthAttrs", &self.UnauthAttrs).finish()
    }
}
impl ::core::default::Default for CMSG_SP3_COMPATIBLE_AUX_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMSG_SP3_COMPATIBLE_AUX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for CMSG_SP3_COMPATIBLE_AUX_INFO {}
impl ::core::fmt::Debug for CMSG_SP3_COMPATIBLE_AUX_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMSG_SP3_COMPATIBLE_AUX_INFO").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMSG_STREAM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CMS_DH_KEY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMS_DH_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.Algid == other.Algid && self.pszContentEncObjId == other.pszContentEncObjId && self.PubInfo == other.PubInfo && self.pReserved == other.pReserved
    }
}
impl ::core::cmp::Eq for CMS_DH_KEY_INFO {}
impl ::core::fmt::Debug for CMS_DH_KEY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMS_DH_KEY_INFO").field("dwVersion", &self.dwVersion).field("Algid", &self.Algid).field("pszContentEncObjId", &self.pszContentEncObjId).field("PubInfo", &self.PubInfo).field("pReserved", &self.pReserved).finish()
    }
}
impl ::core::default::Default for CMS_KEY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMS_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.Algid == other.Algid && self.pbOID == other.pbOID && self.cbOID == other.cbOID
    }
}
impl ::core::cmp::Eq for CMS_KEY_INFO {}
impl ::core::fmt::Debug for CMS_KEY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMS_KEY_INFO").field("dwVersion", &self.dwVersion).field("Algid", &self.Algid).field("pbOID", &self.pbOID).field("cbOID", &self.cbOID).finish()
    }
}
impl ::core::default::Default for CPS_URLS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CPS_URLS {
    fn eq(&self, other: &Self) -> bool {
        self.pszURL == other.pszURL && self.pAlgorithm == other.pAlgorithm && self.pDigest == other.pDigest
    }
}
impl ::core::cmp::Eq for CPS_URLS {}
impl ::core::fmt::Debug for CPS_URLS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CPS_URLS").field("pszURL", &self.pszURL).field("pAlgorithm", &self.pAlgorithm).field("pDigest", &self.pDigest).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRL_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRL_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.dwCertEncodingType == other.dwCertEncodingType && self.pbCrlEncoded == other.pbCrlEncoded && self.cbCrlEncoded == other.cbCrlEncoded && self.pCrlInfo == other.pCrlInfo && self.hCertStore == other.hCertStore
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRL_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRL_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRL_CONTEXT").field("dwCertEncodingType", &self.dwCertEncodingType).field("pbCrlEncoded", &self.pbCrlEncoded).field("cbCrlEncoded", &self.cbCrlEncoded).field("pCrlInfo", &self.pCrlInfo).field("hCertStore", &self.hCertStore).finish()
    }
}
impl ::core::default::Default for CRL_DIST_POINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRL_DIST_POINTS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRL_DIST_POINTS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cDistPoint == other.cDistPoint && self.rgDistPoint == other.rgDistPoint
    }
}
impl ::core::cmp::Eq for CRL_DIST_POINTS_INFO {}
impl ::core::fmt::Debug for CRL_DIST_POINTS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRL_DIST_POINTS_INFO").field("cDistPoint", &self.cDistPoint).field("rgDistPoint", &self.rgDistPoint).finish()
    }
}
impl ::core::default::Default for CRL_DIST_POINT_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRL_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRL_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.SerialNumber == other.SerialNumber && self.RevocationDate == other.RevocationDate && self.cExtension == other.cExtension && self.rgExtension == other.rgExtension
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRL_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRL_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRL_ENTRY").field("SerialNumber", &self.SerialNumber).field("RevocationDate", &self.RevocationDate).field("cExtension", &self.cExtension).field("rgExtension", &self.rgExtension).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRL_FIND_ISSUED_FOR_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRL_FIND_ISSUED_FOR_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.pSubjectCert == other.pSubjectCert && self.pIssuerCert == other.pIssuerCert
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRL_FIND_ISSUED_FOR_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRL_FIND_ISSUED_FOR_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRL_FIND_ISSUED_FOR_PARA").field("pSubjectCert", &self.pSubjectCert).field("pIssuerCert", &self.pIssuerCert).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.SignatureAlgorithm == other.SignatureAlgorithm && self.Issuer == other.Issuer && self.ThisUpdate == other.ThisUpdate && self.NextUpdate == other.NextUpdate && self.cCRLEntry == other.cCRLEntry && self.rgCRLEntry == other.rgCRLEntry && self.cExtension == other.cExtension && self.rgExtension == other.rgExtension
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRL_INFO").field("dwVersion", &self.dwVersion).field("SignatureAlgorithm", &self.SignatureAlgorithm).field("Issuer", &self.Issuer).field("ThisUpdate", &self.ThisUpdate).field("NextUpdate", &self.NextUpdate).field("cCRLEntry", &self.cCRLEntry).field("rgCRLEntry", &self.rgCRLEntry).field("cExtension", &self.cExtension).field("rgExtension", &self.rgExtension).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRL_ISSUING_DIST_POINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRL_REVOCATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRL_REVOCATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pCrlEntry == other.pCrlEntry && self.pCrlContext == other.pCrlContext && self.pCrlIssuerChain == other.pCrlIssuerChain
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRL_REVOCATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRL_REVOCATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRL_REVOCATION_INFO").field("pCrlEntry", &self.pCrlEntry).field("pCrlContext", &self.pCrlContext).field("pCrlIssuerChain", &self.pCrlIssuerChain).finish()
    }
}
impl ::core::default::Default for CROSS_CERT_DIST_POINTS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CROSS_CERT_DIST_POINTS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSyncDeltaTime == other.dwSyncDeltaTime && self.cDistPoint == other.cDistPoint && self.rgDistPoint == other.rgDistPoint
    }
}
impl ::core::cmp::Eq for CROSS_CERT_DIST_POINTS_INFO {}
impl ::core::fmt::Debug for CROSS_CERT_DIST_POINTS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CROSS_CERT_DIST_POINTS_INFO").field("dwSyncDeltaTime", &self.dwSyncDeltaTime).field("cDistPoint", &self.cDistPoint).field("rgDistPoint", &self.rgDistPoint).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTNET_URL_CACHE_FLUSH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTNET_URL_CACHE_FLUSH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwExemptSeconds == other.dwExemptSeconds && self.ExpireTime == other.ExpireTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTNET_URL_CACHE_FLUSH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPTNET_URL_CACHE_FLUSH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTNET_URL_CACHE_FLUSH_INFO").field("cbSize", &self.cbSize).field("dwExemptSeconds", &self.dwExemptSeconds).field("ExpireTime", &self.ExpireTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTNET_URL_CACHE_PRE_FETCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTNET_URL_CACHE_PRE_FETCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwObjectType == other.dwObjectType && self.dwError == other.dwError && self.dwReserved == other.dwReserved && self.ThisUpdateTime == other.ThisUpdateTime && self.NextUpdateTime == other.NextUpdateTime && self.PublishTime == other.PublishTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTNET_URL_CACHE_PRE_FETCH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPTNET_URL_CACHE_PRE_FETCH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTNET_URL_CACHE_PRE_FETCH_INFO").field("cbSize", &self.cbSize).field("dwObjectType", &self.dwObjectType).field("dwError", &self.dwError).field("dwReserved", &self.dwReserved).field("ThisUpdateTime", &self.ThisUpdateTime).field("NextUpdateTime", &self.NextUpdateTime).field("PublishTime", &self.PublishTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTNET_URL_CACHE_RESPONSE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTNET_URL_CACHE_RESPONSE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.wResponseType == other.wResponseType && self.wResponseFlags == other.wResponseFlags && self.LastModifiedTime == other.LastModifiedTime && self.dwMaxAge == other.dwMaxAge && self.pwszETag == other.pwszETag && self.dwProxyId == other.dwProxyId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTNET_URL_CACHE_RESPONSE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPTNET_URL_CACHE_RESPONSE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTNET_URL_CACHE_RESPONSE_INFO").field("cbSize", &self.cbSize).field("wResponseType", &self.wResponseType).field("wResponseFlags", &self.wResponseFlags).field("LastModifiedTime", &self.LastModifiedTime).field("dwMaxAge", &self.dwMaxAge).field("pwszETag", &self.pwszETag).field("dwProxyId", &self.dwProxyId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTPROTECT_PROMPTSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTPROTECT_PROMPTSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwPromptFlags == other.dwPromptFlags && self.hwndApp == other.hwndApp && self.szPrompt == other.szPrompt
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTPROTECT_PROMPTSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPTPROTECT_PROMPTSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTPROTECT_PROMPTSTRUCT").field("cbSize", &self.cbSize).field("dwPromptFlags", &self.dwPromptFlags).field("hwndApp", &self.hwndApp).field("szPrompt", &self.szPrompt).finish()
    }
}
impl ::core::default::Default for CRYPT_3DES_KEY_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_3DES_KEY_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.IV == other.IV && self.Feedback == other.Feedback
    }
}
impl ::core::cmp::Eq for CRYPT_3DES_KEY_STATE {}
impl ::core::fmt::Debug for CRYPT_3DES_KEY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_3DES_KEY_STATE").field("Key", &self.Key).field("IV", &self.IV).field("Feedback", &self.Feedback).finish()
    }
}
impl ::core::default::Default for CRYPT_ACQUIRE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_ACQUIRE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_ACQUIRE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CRYPT_ACQUIRE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRYPT_ACQUIRE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRYPT_ACQUIRE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRYPT_ACQUIRE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRYPT_ACQUIRE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CRYPT_AES_128_KEY_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_AES_128_KEY_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.IV == other.IV && self.EncryptionState == other.EncryptionState && self.DecryptionState == other.DecryptionState && self.Feedback == other.Feedback
    }
}
impl ::core::cmp::Eq for CRYPT_AES_128_KEY_STATE {}
impl ::core::fmt::Debug for CRYPT_AES_128_KEY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_AES_128_KEY_STATE").field("Key", &self.Key).field("IV", &self.IV).field("EncryptionState", &self.EncryptionState).field("DecryptionState", &self.DecryptionState).field("Feedback", &self.Feedback).finish()
    }
}
impl ::core::default::Default for CRYPT_AES_256_KEY_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_AES_256_KEY_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.IV == other.IV && self.EncryptionState == other.EncryptionState && self.DecryptionState == other.DecryptionState && self.Feedback == other.Feedback
    }
}
impl ::core::cmp::Eq for CRYPT_AES_256_KEY_STATE {}
impl ::core::fmt::Debug for CRYPT_AES_256_KEY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_AES_256_KEY_STATE").field("Key", &self.Key).field("IV", &self.IV).field("EncryptionState", &self.EncryptionState).field("DecryptionState", &self.DecryptionState).field("Feedback", &self.Feedback).finish()
    }
}
impl ::core::default::Default for CRYPT_ALGORITHM_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_ALGORITHM_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.pszObjId == other.pszObjId && self.Parameters == other.Parameters
    }
}
impl ::core::cmp::Eq for CRYPT_ALGORITHM_IDENTIFIER {}
impl ::core::fmt::Debug for CRYPT_ALGORITHM_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_ALGORITHM_IDENTIFIER").field("pszObjId", &self.pszObjId).field("Parameters", &self.Parameters).finish()
    }
}
impl ::core::default::Default for CRYPT_ASYNC_RETRIEVAL_COMPLETION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPT_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.pszObjId == other.pszObjId && self.cValue == other.cValue && self.rgValue == other.rgValue
    }
}
impl ::core::cmp::Eq for CRYPT_ATTRIBUTE {}
impl ::core::fmt::Debug for CRYPT_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_ATTRIBUTE").field("pszObjId", &self.pszObjId).field("cValue", &self.cValue).field("rgValue", &self.rgValue).finish()
    }
}
impl ::core::default::Default for CRYPT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.cAttr == other.cAttr && self.rgAttr == other.rgAttr
    }
}
impl ::core::cmp::Eq for CRYPT_ATTRIBUTES {}
impl ::core::fmt::Debug for CRYPT_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_ATTRIBUTES").field("cAttr", &self.cAttr).field("rgAttr", &self.rgAttr).finish()
    }
}
impl ::core::default::Default for CRYPT_ATTRIBUTE_TYPE_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_ATTRIBUTE_TYPE_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.pszObjId == other.pszObjId && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for CRYPT_ATTRIBUTE_TYPE_VALUE {}
impl ::core::fmt::Debug for CRYPT_ATTRIBUTE_TYPE_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_ATTRIBUTE_TYPE_VALUE").field("pszObjId", &self.pszObjId).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for CRYPT_BIT_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_BIT_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.pbData == other.pbData && self.cUnusedBits == other.cUnusedBits
    }
}
impl ::core::cmp::Eq for CRYPT_BIT_BLOB {}
impl ::core::fmt::Debug for CRYPT_BIT_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_BIT_BLOB").field("cbData", &self.cbData).field("pbData", &self.pbData).field("cUnusedBits", &self.cUnusedBits).finish()
    }
}
impl ::core::default::Default for CRYPT_BLOB_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_BLOB_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.cBlob == other.cBlob && self.rgBlob == other.rgBlob
    }
}
impl ::core::cmp::Eq for CRYPT_BLOB_ARRAY {}
impl ::core::fmt::Debug for CRYPT_BLOB_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_BLOB_ARRAY").field("cBlob", &self.cBlob).field("rgBlob", &self.rgBlob).finish()
    }
}
impl ::core::default::Default for CRYPT_CONTENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_CONTENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszObjId == other.pszObjId && self.Content == other.Content
    }
}
impl ::core::cmp::Eq for CRYPT_CONTENT_INFO {}
impl ::core::fmt::Debug for CRYPT_CONTENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_CONTENT_INFO").field("pszObjId", &self.pszObjId).field("Content", &self.Content).finish()
    }
}
impl ::core::default::Default for CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY {
    fn eq(&self, other: &Self) -> bool {
        self.pszObjId == other.pszObjId && self.cValue == other.cValue && self.rgValue == other.rgValue
    }
}
impl ::core::cmp::Eq for CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY {}
impl ::core::fmt::Debug for CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY").field("pszObjId", &self.pszObjId).field("cValue", &self.cValue).field("rgValue", &self.rgValue).finish()
    }
}
impl ::core::default::Default for CRYPT_CONTEXTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_CONTEXTS {
    fn eq(&self, other: &Self) -> bool {
        self.cContexts == other.cContexts && self.rgpszContexts == other.rgpszContexts
    }
}
impl ::core::cmp::Eq for CRYPT_CONTEXTS {}
impl ::core::fmt::Debug for CRYPT_CONTEXTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_CONTEXTS").field("cContexts", &self.cContexts).field("rgpszContexts", &self.rgpszContexts).finish()
    }
}
impl ::core::default::Default for CRYPT_CONTEXT_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_CONTEXT_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for CRYPT_CONTEXT_CONFIG {}
impl ::core::fmt::Debug for CRYPT_CONTEXT_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_CONTEXT_CONFIG").field("dwFlags", &self.dwFlags).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::default::Default for CRYPT_CONTEXT_CONFIG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_CONTEXT_CONFIG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_CONTEXT_CONFIG_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CRYPT_CONTEXT_CONFIG_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRYPT_CONTEXT_CONFIG_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRYPT_CONTEXT_CONFIG_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRYPT_CONTEXT_CONFIG_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRYPT_CONTEXT_CONFIG_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CRYPT_CONTEXT_FUNCTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_CONTEXT_FUNCTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.cFunctions == other.cFunctions && self.rgpszFunctions == other.rgpszFunctions
    }
}
impl ::core::cmp::Eq for CRYPT_CONTEXT_FUNCTIONS {}
impl ::core::fmt::Debug for CRYPT_CONTEXT_FUNCTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_CONTEXT_FUNCTIONS").field("cFunctions", &self.cFunctions).field("rgpszFunctions", &self.rgpszFunctions).finish()
    }
}
impl ::core::default::Default for CRYPT_CONTEXT_FUNCTION_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_CONTEXT_FUNCTION_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for CRYPT_CONTEXT_FUNCTION_CONFIG {}
impl ::core::fmt::Debug for CRYPT_CONTEXT_FUNCTION_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_CONTEXT_FUNCTION_CONFIG").field("dwFlags", &self.dwFlags).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::default::Default for CRYPT_CONTEXT_FUNCTION_PROVIDERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_CONTEXT_FUNCTION_PROVIDERS {
    fn eq(&self, other: &Self) -> bool {
        self.cProviders == other.cProviders && self.rgpszProviders == other.rgpszProviders
    }
}
impl ::core::cmp::Eq for CRYPT_CONTEXT_FUNCTION_PROVIDERS {}
impl ::core::fmt::Debug for CRYPT_CONTEXT_FUNCTION_PROVIDERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_CONTEXT_FUNCTION_PROVIDERS").field("cProviders", &self.cProviders).field("rgpszProviders", &self.rgpszProviders).finish()
    }
}
impl ::core::default::Default for CRYPT_CREDENTIALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_CREDENTIALS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pszCredentialsOid == other.pszCredentialsOid && self.pvCredentials == other.pvCredentials
    }
}
impl ::core::cmp::Eq for CRYPT_CREDENTIALS {}
impl ::core::fmt::Debug for CRYPT_CREDENTIALS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_CREDENTIALS").field("cbSize", &self.cbSize).field("pszCredentialsOid", &self.pszCredentialsOid).field("pvCredentials", &self.pvCredentials).finish()
    }
}
impl ::core::default::Default for CRYPT_CSP_PROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_CSP_PROVIDER {
    fn eq(&self, other: &Self) -> bool {
        self.dwKeySpec == other.dwKeySpec && self.pwszProviderName == other.pwszProviderName && self.Signature == other.Signature
    }
}
impl ::core::cmp::Eq for CRYPT_CSP_PROVIDER {}
impl ::core::fmt::Debug for CRYPT_CSP_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_CSP_PROVIDER").field("dwKeySpec", &self.dwKeySpec).field("pwszProviderName", &self.pwszProviderName).field("Signature", &self.Signature).finish()
    }
}
impl ::core::default::Default for CRYPT_DECODE_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPT_DECRYPT_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_DECRYPT_MESSAGE_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMsgAndCertEncodingType == other.dwMsgAndCertEncodingType && self.cCertStore == other.cCertStore && self.rghCertStore == other.rghCertStore
    }
}
impl ::core::cmp::Eq for CRYPT_DECRYPT_MESSAGE_PARA {}
impl ::core::fmt::Debug for CRYPT_DECRYPT_MESSAGE_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_DECRYPT_MESSAGE_PARA").field("cbSize", &self.cbSize).field("dwMsgAndCertEncodingType", &self.dwMsgAndCertEncodingType).field("cCertStore", &self.cCertStore).field("rghCertStore", &self.rghCertStore).finish()
    }
}
impl ::core::default::Default for CRYPT_DEFAULT_CONTEXT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_DEFAULT_CONTEXT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_DEFAULT_CONTEXT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CRYPT_DEFAULT_CONTEXT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRYPT_DEFAULT_CONTEXT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRYPT_DEFAULT_CONTEXT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRYPT_DEFAULT_CONTEXT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRYPT_DEFAULT_CONTEXT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cOID == other.cOID && self.rgpszOID == other.rgpszOID
    }
}
impl ::core::cmp::Eq for CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA {}
impl ::core::fmt::Debug for CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA").field("cOID", &self.cOID).field("rgpszOID", &self.rgpszOID).finish()
    }
}
impl ::core::default::Default for CRYPT_DEFAULT_CONTEXT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_DEFAULT_CONTEXT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_DEFAULT_CONTEXT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPT_DES_KEY_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_DES_KEY_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.IV == other.IV && self.Feedback == other.Feedback
    }
}
impl ::core::cmp::Eq for CRYPT_DES_KEY_STATE {}
impl ::core::fmt::Debug for CRYPT_DES_KEY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_DES_KEY_STATE").field("Key", &self.Key).field("IV", &self.IV).field("Feedback", &self.Feedback).finish()
    }
}
impl ::core::default::Default for CRYPT_ECC_CMS_SHARED_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_ECC_CMS_SHARED_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Algorithm == other.Algorithm && self.EntityUInfo == other.EntityUInfo && self.rgbSuppPubInfo == other.rgbSuppPubInfo
    }
}
impl ::core::cmp::Eq for CRYPT_ECC_CMS_SHARED_INFO {}
impl ::core::fmt::Debug for CRYPT_ECC_CMS_SHARED_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_ECC_CMS_SHARED_INFO").field("Algorithm", &self.Algorithm).field("EntityUInfo", &self.EntityUInfo).field("rgbSuppPubInfo", &self.rgbSuppPubInfo).finish()
    }
}
impl ::core::default::Default for CRYPT_ECC_PRIVATE_KEY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_ECC_PRIVATE_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.PrivateKey == other.PrivateKey && self.szCurveOid == other.szCurveOid && self.PublicKey == other.PublicKey
    }
}
impl ::core::cmp::Eq for CRYPT_ECC_PRIVATE_KEY_INFO {}
impl ::core::fmt::Debug for CRYPT_ECC_PRIVATE_KEY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_ECC_PRIVATE_KEY_INFO").field("dwVersion", &self.dwVersion).field("PrivateKey", &self.PrivateKey).field("szCurveOid", &self.szCurveOid).field("PublicKey", &self.PublicKey).finish()
    }
}
impl ::core::default::Default for CRYPT_ENCODE_OBJECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_ENCODE_OBJECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_ENCODE_OBJECT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CRYPT_ENCODE_OBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRYPT_ENCODE_OBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRYPT_ENCODE_OBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRYPT_ENCODE_OBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRYPT_ENCODE_OBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CRYPT_ENCODE_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPT_ENCRYPTED_PRIVATE_KEY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_ENCRYPTED_PRIVATE_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EncryptionAlgorithm == other.EncryptionAlgorithm && self.EncryptedPrivateKey == other.EncryptedPrivateKey
    }
}
impl ::core::cmp::Eq for CRYPT_ENCRYPTED_PRIVATE_KEY_INFO {}
impl ::core::fmt::Debug for CRYPT_ENCRYPTED_PRIVATE_KEY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_ENCRYPTED_PRIVATE_KEY_INFO").field("EncryptionAlgorithm", &self.EncryptionAlgorithm).field("EncryptedPrivateKey", &self.EncryptedPrivateKey).finish()
    }
}
impl ::core::default::Default for CRYPT_ENCRYPT_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_ENCRYPT_MESSAGE_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMsgEncodingType == other.dwMsgEncodingType && self.hCryptProv == other.hCryptProv && self.ContentEncryptionAlgorithm == other.ContentEncryptionAlgorithm && self.pvEncryptionAuxInfo == other.pvEncryptionAuxInfo && self.dwFlags == other.dwFlags && self.dwInnerContentType == other.dwInnerContentType
    }
}
impl ::core::cmp::Eq for CRYPT_ENCRYPT_MESSAGE_PARA {}
impl ::core::fmt::Debug for CRYPT_ENCRYPT_MESSAGE_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_ENCRYPT_MESSAGE_PARA").field("cbSize", &self.cbSize).field("dwMsgEncodingType", &self.dwMsgEncodingType).field("hCryptProv", &self.hCryptProv).field("ContentEncryptionAlgorithm", &self.ContentEncryptionAlgorithm).field("pvEncryptionAuxInfo", &self.pvEncryptionAuxInfo).field("dwFlags", &self.dwFlags).field("dwInnerContentType", &self.dwInnerContentType).finish()
    }
}
impl ::core::default::Default for CRYPT_ENROLLMENT_NAME_VALUE_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_ENROLLMENT_NAME_VALUE_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.pwszName == other.pwszName && self.pwszValue == other.pwszValue
    }
}
impl ::core::cmp::Eq for CRYPT_ENROLLMENT_NAME_VALUE_PAIR {}
impl ::core::fmt::Debug for CRYPT_ENROLLMENT_NAME_VALUE_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_ENROLLMENT_NAME_VALUE_PAIR").field("pwszName", &self.pwszName).field("pwszValue", &self.pwszValue).finish()
    }
}
impl ::core::default::Default for CRYPT_FIND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_FIND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_FIND_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.iDeltaCrlIndicator == other.iDeltaCrlIndicator && self.pftCacheResync == other.pftCacheResync && self.pLastSyncTime == other.pLastSyncTime && self.pMaxAgeTime == other.pMaxAgeTime && self.pChainPara == other.pChainPara && self.pDeltaCrlIndicator == other.pDeltaCrlIndicator
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO").field("cbSize", &self.cbSize).field("iDeltaCrlIndicator", &self.iDeltaCrlIndicator).field("pftCacheResync", &self.pftCacheResync).field("pLastSyncTime", &self.pLastSyncTime).field("pMaxAgeTime", &self.pMaxAgeTime).field("pChainPara", &self.pChainPara).field("pDeltaCrlIndicator", &self.pDeltaCrlIndicator).finish()
    }
}
impl ::core::default::Default for CRYPT_GET_URL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_GET_URL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_GET_URL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CRYPT_GET_URL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRYPT_GET_URL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRYPT_GET_URL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRYPT_GET_URL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRYPT_GET_URL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CRYPT_HASH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_HASH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.HashAlgorithm == other.HashAlgorithm && self.Hash == other.Hash
    }
}
impl ::core::cmp::Eq for CRYPT_HASH_INFO {}
impl ::core::fmt::Debug for CRYPT_HASH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_HASH_INFO").field("HashAlgorithm", &self.HashAlgorithm).field("Hash", &self.Hash).finish()
    }
}
impl ::core::default::Default for CRYPT_HASH_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_HASH_MESSAGE_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMsgEncodingType == other.dwMsgEncodingType && self.hCryptProv == other.hCryptProv && self.HashAlgorithm == other.HashAlgorithm && self.pvHashAuxInfo == other.pvHashAuxInfo
    }
}
impl ::core::cmp::Eq for CRYPT_HASH_MESSAGE_PARA {}
impl ::core::fmt::Debug for CRYPT_HASH_MESSAGE_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_HASH_MESSAGE_PARA").field("cbSize", &self.cbSize).field("dwMsgEncodingType", &self.dwMsgEncodingType).field("hCryptProv", &self.hCryptProv).field("HashAlgorithm", &self.HashAlgorithm).field("pvHashAuxInfo", &self.pvHashAuxInfo).finish()
    }
}
impl ::core::default::Default for CRYPT_IMAGE_REF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_IMAGE_REF {
    fn eq(&self, other: &Self) -> bool {
        self.pszImage == other.pszImage && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for CRYPT_IMAGE_REF {}
impl ::core::fmt::Debug for CRYPT_IMAGE_REF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_IMAGE_REF").field("pszImage", &self.pszImage).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for CRYPT_IMAGE_REF_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_IMAGE_REF_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_IMAGE_REF_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CRYPT_IMAGE_REF_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRYPT_IMAGE_REF_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRYPT_IMAGE_REF_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRYPT_IMAGE_REF_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRYPT_IMAGE_REF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CRYPT_IMAGE_REG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_IMAGE_REG {
    fn eq(&self, other: &Self) -> bool {
        self.pszImage == other.pszImage && self.cInterfaces == other.cInterfaces && self.rgpInterfaces == other.rgpInterfaces
    }
}
impl ::core::cmp::Eq for CRYPT_IMAGE_REG {}
impl ::core::fmt::Debug for CRYPT_IMAGE_REG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_IMAGE_REG").field("pszImage", &self.pszImage).field("cInterfaces", &self.cInterfaces).field("rgpInterfaces", &self.rgpInterfaces).finish()
    }
}
impl ::core::default::Default for CRYPT_IMPORT_PUBLIC_KEY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_IMPORT_PUBLIC_KEY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_IMPORT_PUBLIC_KEY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CRYPT_IMPORT_PUBLIC_KEY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRYPT_IMPORT_PUBLIC_KEY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRYPT_IMPORT_PUBLIC_KEY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRYPT_IMPORT_PUBLIC_KEY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRYPT_IMPORT_PUBLIC_KEY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CRYPT_INTEGER_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_INTEGER_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for CRYPT_INTEGER_BLOB {}
impl ::core::fmt::Debug for CRYPT_INTEGER_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_INTEGER_BLOB").field("cbData", &self.cbData).field("pbData", &self.pbData).finish()
    }
}
impl ::core::default::Default for CRYPT_INTERFACE_REG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_INTERFACE_REG {
    fn eq(&self, other: &Self) -> bool {
        self.dwInterface == other.dwInterface && self.dwFlags == other.dwFlags && self.cFunctions == other.cFunctions && self.rgpszFunctions == other.rgpszFunctions
    }
}
impl ::core::cmp::Eq for CRYPT_INTERFACE_REG {}
impl ::core::fmt::Debug for CRYPT_INTERFACE_REG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_INTERFACE_REG").field("dwInterface", &self.dwInterface).field("dwFlags", &self.dwFlags).field("cFunctions", &self.cFunctions).field("rgpszFunctions", &self.rgpszFunctions).finish()
    }
}
impl ::core::default::Default for CRYPT_KEY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_KEY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_KEY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CRYPT_KEY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRYPT_KEY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRYPT_KEY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRYPT_KEY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRYPT_KEY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CRYPT_KEY_PARAM_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_KEY_PARAM_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_KEY_PARAM_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPT_KEY_PROV_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_KEY_PROV_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwszContainerName == other.pwszContainerName && self.pwszProvName == other.pwszProvName && self.dwProvType == other.dwProvType && self.dwFlags == other.dwFlags && self.cProvParam == other.cProvParam && self.rgProvParam == other.rgProvParam && self.dwKeySpec == other.dwKeySpec
    }
}
impl ::core::cmp::Eq for CRYPT_KEY_PROV_INFO {}
impl ::core::fmt::Debug for CRYPT_KEY_PROV_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_KEY_PROV_INFO").field("pwszContainerName", &self.pwszContainerName).field("pwszProvName", &self.pwszProvName).field("dwProvType", &self.dwProvType).field("dwFlags", &self.dwFlags).field("cProvParam", &self.cProvParam).field("rgProvParam", &self.rgProvParam).field("dwKeySpec", &self.dwKeySpec).finish()
    }
}
impl ::core::default::Default for CRYPT_KEY_PROV_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_KEY_PROV_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.dwParam == other.dwParam && self.pbData == other.pbData && self.cbData == other.cbData && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for CRYPT_KEY_PROV_PARAM {}
impl ::core::fmt::Debug for CRYPT_KEY_PROV_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_KEY_PROV_PARAM").field("dwParam", &self.dwParam).field("pbData", &self.pbData).field("cbData", &self.cbData).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for CRYPT_KEY_SIGN_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPT_KEY_VERIFY_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_KEY_VERIFY_MESSAGE_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMsgEncodingType == other.dwMsgEncodingType && self.hCryptProv == other.hCryptProv
    }
}
impl ::core::cmp::Eq for CRYPT_KEY_VERIFY_MESSAGE_PARA {}
impl ::core::fmt::Debug for CRYPT_KEY_VERIFY_MESSAGE_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_KEY_VERIFY_MESSAGE_PARA").field("cbSize", &self.cbSize).field("dwMsgEncodingType", &self.dwMsgEncodingType).field("hCryptProv", &self.hCryptProv).finish()
    }
}
impl ::core::default::Default for CRYPT_MASK_GEN_ALGORITHM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_MASK_GEN_ALGORITHM {
    fn eq(&self, other: &Self) -> bool {
        self.pszObjId == other.pszObjId && self.HashAlgorithm == other.HashAlgorithm
    }
}
impl ::core::cmp::Eq for CRYPT_MASK_GEN_ALGORITHM {}
impl ::core::fmt::Debug for CRYPT_MASK_GEN_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_MASK_GEN_ALGORITHM").field("pszObjId", &self.pszObjId).field("HashAlgorithm", &self.HashAlgorithm).finish()
    }
}
impl ::core::default::Default for CRYPT_MSG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_MSG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_MSG_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPT_OBJECT_LOCATOR_RELEASE_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_OBJECT_LOCATOR_RELEASE_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_OBJECT_LOCATOR_RELEASE_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPT_OBJID_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_OBJID_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.dwAlgId == other.dwAlgId && self.pszObjId == other.pszObjId
    }
}
impl ::core::cmp::Eq for CRYPT_OBJID_TABLE {}
impl ::core::fmt::Debug for CRYPT_OBJID_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_OBJID_TABLE").field("dwAlgId", &self.dwAlgId).field("pszObjId", &self.pszObjId).finish()
    }
}
impl ::core::default::Default for CRYPT_OID_FUNC_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_OID_FUNC_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.pszOID == other.pszOID && self.pvFuncAddr == other.pvFuncAddr
    }
}
impl ::core::cmp::Eq for CRYPT_OID_FUNC_ENTRY {}
impl ::core::fmt::Debug for CRYPT_OID_FUNC_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_OID_FUNC_ENTRY").field("pszOID", &self.pszOID).field("pvFuncAddr", &self.pvFuncAddr).finish()
    }
}
impl ::core::default::Default for CRYPT_OID_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPT_PASSWORD_CREDENTIALSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_PASSWORD_CREDENTIALSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pszUsername == other.pszUsername && self.pszPassword == other.pszPassword
    }
}
impl ::core::cmp::Eq for CRYPT_PASSWORD_CREDENTIALSA {}
impl ::core::fmt::Debug for CRYPT_PASSWORD_CREDENTIALSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PASSWORD_CREDENTIALSA").field("cbSize", &self.cbSize).field("pszUsername", &self.pszUsername).field("pszPassword", &self.pszPassword).finish()
    }
}
impl ::core::default::Default for CRYPT_PASSWORD_CREDENTIALSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_PASSWORD_CREDENTIALSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pszUsername == other.pszUsername && self.pszPassword == other.pszPassword
    }
}
impl ::core::cmp::Eq for CRYPT_PASSWORD_CREDENTIALSW {}
impl ::core::fmt::Debug for CRYPT_PASSWORD_CREDENTIALSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PASSWORD_CREDENTIALSW").field("cbSize", &self.cbSize).field("pszUsername", &self.pszUsername).field("pszPassword", &self.pszPassword).finish()
    }
}
impl ::core::default::Default for CRYPT_PKCS12_PBE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_PKCS12_PBE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.iIterations == other.iIterations && self.cbSalt == other.cbSalt
    }
}
impl ::core::cmp::Eq for CRYPT_PKCS12_PBE_PARAMS {}
impl ::core::fmt::Debug for CRYPT_PKCS12_PBE_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PKCS12_PBE_PARAMS").field("iIterations", &self.iIterations).field("cbSalt", &self.cbSalt).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPT_PKCS8_EXPORT_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPT_PKCS8_IMPORT_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPT_PRIVATE_KEY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_PRIVATE_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Algorithm == other.Algorithm && self.PrivateKey == other.PrivateKey && self.pAttributes == other.pAttributes
    }
}
impl ::core::cmp::Eq for CRYPT_PRIVATE_KEY_INFO {}
impl ::core::fmt::Debug for CRYPT_PRIVATE_KEY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PRIVATE_KEY_INFO").field("Version", &self.Version).field("Algorithm", &self.Algorithm).field("PrivateKey", &self.PrivateKey).field("pAttributes", &self.pAttributes).finish()
    }
}
impl ::core::default::Default for CRYPT_PROPERTY_REF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_PROPERTY_REF {
    fn eq(&self, other: &Self) -> bool {
        self.pszProperty == other.pszProperty && self.cbValue == other.cbValue && self.pbValue == other.pbValue
    }
}
impl ::core::cmp::Eq for CRYPT_PROPERTY_REF {}
impl ::core::fmt::Debug for CRYPT_PROPERTY_REF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROPERTY_REF").field("pszProperty", &self.pszProperty).field("cbValue", &self.cbValue).field("pbValue", &self.pbValue).finish()
    }
}
impl ::core::default::Default for CRYPT_PROVIDERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_PROVIDERS {
    fn eq(&self, other: &Self) -> bool {
        self.cProviders == other.cProviders && self.rgpszProviders == other.rgpszProviders
    }
}
impl ::core::cmp::Eq for CRYPT_PROVIDERS {}
impl ::core::fmt::Debug for CRYPT_PROVIDERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVIDERS").field("cProviders", &self.cProviders).field("rgpszProviders", &self.rgpszProviders).finish()
    }
}
impl ::core::default::Default for CRYPT_PROVIDER_REF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_REF {
    fn eq(&self, other: &Self) -> bool {
        self.dwInterface == other.dwInterface && self.pszFunction == other.pszFunction && self.pszProvider == other.pszProvider && self.cProperties == other.cProperties && self.rgpProperties == other.rgpProperties && self.pUM == other.pUM && self.pKM == other.pKM
    }
}
impl ::core::cmp::Eq for CRYPT_PROVIDER_REF {}
impl ::core::fmt::Debug for CRYPT_PROVIDER_REF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVIDER_REF").field("dwInterface", &self.dwInterface).field("pszFunction", &self.pszFunction).field("pszProvider", &self.pszProvider).field("cProperties", &self.cProperties).field("rgpProperties", &self.rgpProperties).field("pUM", &self.pUM).field("pKM", &self.pKM).finish()
    }
}
impl ::core::default::Default for CRYPT_PROVIDER_REFS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_REFS {
    fn eq(&self, other: &Self) -> bool {
        self.cProviders == other.cProviders && self.rgpProviders == other.rgpProviders
    }
}
impl ::core::cmp::Eq for CRYPT_PROVIDER_REFS {}
impl ::core::fmt::Debug for CRYPT_PROVIDER_REFS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVIDER_REFS").field("cProviders", &self.cProviders).field("rgpProviders", &self.rgpProviders).finish()
    }
}
impl ::core::default::Default for CRYPT_PROVIDER_REG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_REG {
    fn eq(&self, other: &Self) -> bool {
        self.cAliases == other.cAliases && self.rgpszAliases == other.rgpszAliases && self.pUM == other.pUM && self.pKM == other.pKM
    }
}
impl ::core::cmp::Eq for CRYPT_PROVIDER_REG {}
impl ::core::fmt::Debug for CRYPT_PROVIDER_REG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVIDER_REG").field("cAliases", &self.cAliases).field("rgpszAliases", &self.rgpszAliases).field("pUM", &self.pUM).field("pKM", &self.pKM).finish()
    }
}
impl ::core::default::Default for CRYPT_PSOURCE_ALGORITHM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_PSOURCE_ALGORITHM {
    fn eq(&self, other: &Self) -> bool {
        self.pszObjId == other.pszObjId && self.EncodingParameters == other.EncodingParameters
    }
}
impl ::core::cmp::Eq for CRYPT_PSOURCE_ALGORITHM {}
impl ::core::fmt::Debug for CRYPT_PSOURCE_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PSOURCE_ALGORITHM").field("pszObjId", &self.pszObjId).field("EncodingParameters", &self.EncodingParameters).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPT_RC2_CBC_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPT_RC2_CBC_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.fIV == other.fIV && self.rgbIV == other.rgbIV
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPT_RC2_CBC_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPT_RC2_CBC_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_RC2_CBC_PARAMETERS").field("dwVersion", &self.dwVersion).field("fIV", &self.fIV).field("rgbIV", &self.rgbIV).finish()
    }
}
impl ::core::default::Default for CRYPT_RC4_KEY_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_RC4_KEY_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.SBox == other.SBox && self.i == other.i && self.j == other.j
    }
}
impl ::core::cmp::Eq for CRYPT_RC4_KEY_STATE {}
impl ::core::fmt::Debug for CRYPT_RC4_KEY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_RC4_KEY_STATE").field("Key", &self.Key).field("SBox", &self.SBox).field("i", &self.i).field("j", &self.j).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPT_RETRIEVE_AUX_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPT_RETRIEVE_AUX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pLastSyncTime == other.pLastSyncTime && self.dwMaxUrlRetrievalByteCount == other.dwMaxUrlRetrievalByteCount && self.pPreFetchInfo == other.pPreFetchInfo && self.pFlushInfo == other.pFlushInfo && self.ppResponseInfo == other.ppResponseInfo && self.pwszCacheFileNamePrefix == other.pwszCacheFileNamePrefix && self.pftCacheResync == other.pftCacheResync && self.fProxyCacheRetrieval == other.fProxyCacheRetrieval && self.dwHttpStatusCode == other.dwHttpStatusCode && self.ppwszErrorResponseHeaders == other.ppwszErrorResponseHeaders && self.ppErrorContentBlob == other.ppErrorContentBlob
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPT_RETRIEVE_AUX_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPT_RETRIEVE_AUX_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_RETRIEVE_AUX_INFO")
            .field("cbSize", &self.cbSize)
            .field("pLastSyncTime", &self.pLastSyncTime)
            .field("dwMaxUrlRetrievalByteCount", &self.dwMaxUrlRetrievalByteCount)
            .field("pPreFetchInfo", &self.pPreFetchInfo)
            .field("pFlushInfo", &self.pFlushInfo)
            .field("ppResponseInfo", &self.ppResponseInfo)
            .field("pwszCacheFileNamePrefix", &self.pwszCacheFileNamePrefix)
            .field("pftCacheResync", &self.pftCacheResync)
            .field("fProxyCacheRetrieval", &self.fProxyCacheRetrieval)
            .field("dwHttpStatusCode", &self.dwHttpStatusCode)
            .field("ppwszErrorResponseHeaders", &self.ppwszErrorResponseHeaders)
            .field("ppErrorContentBlob", &self.ppErrorContentBlob)
            .finish()
    }
}
impl ::core::default::Default for CRYPT_RSAES_OAEP_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_RSAES_OAEP_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.HashAlgorithm == other.HashAlgorithm && self.MaskGenAlgorithm == other.MaskGenAlgorithm && self.PSourceAlgorithm == other.PSourceAlgorithm
    }
}
impl ::core::cmp::Eq for CRYPT_RSAES_OAEP_PARAMETERS {}
impl ::core::fmt::Debug for CRYPT_RSAES_OAEP_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_RSAES_OAEP_PARAMETERS").field("HashAlgorithm", &self.HashAlgorithm).field("MaskGenAlgorithm", &self.MaskGenAlgorithm).field("PSourceAlgorithm", &self.PSourceAlgorithm).finish()
    }
}
impl ::core::default::Default for CRYPT_RSA_SSA_PSS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_RSA_SSA_PSS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.HashAlgorithm == other.HashAlgorithm && self.MaskGenAlgorithm == other.MaskGenAlgorithm && self.dwSaltLength == other.dwSaltLength && self.dwTrailerField == other.dwTrailerField
    }
}
impl ::core::cmp::Eq for CRYPT_RSA_SSA_PSS_PARAMETERS {}
impl ::core::fmt::Debug for CRYPT_RSA_SSA_PSS_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_RSA_SSA_PSS_PARAMETERS").field("HashAlgorithm", &self.HashAlgorithm).field("MaskGenAlgorithm", &self.MaskGenAlgorithm).field("dwSaltLength", &self.dwSaltLength).field("dwTrailerField", &self.dwTrailerField).finish()
    }
}
impl ::core::default::Default for CRYPT_SEQUENCE_OF_ANY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_SEQUENCE_OF_ANY {
    fn eq(&self, other: &Self) -> bool {
        self.cValue == other.cValue && self.rgValue == other.rgValue
    }
}
impl ::core::cmp::Eq for CRYPT_SEQUENCE_OF_ANY {}
impl ::core::fmt::Debug for CRYPT_SEQUENCE_OF_ANY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_SEQUENCE_OF_ANY").field("cValue", &self.cValue).field("rgValue", &self.rgValue).finish()
    }
}
impl ::core::default::Default for CRYPT_SET_HASH_PARAM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_SET_HASH_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_SET_HASH_PARAM").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPT_SET_PROV_PARAM_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_SET_PROV_PARAM_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_SET_PROV_PARAM_ID").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPT_SIGN_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPT_SIGN_MESSAGE_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMsgEncodingType == other.dwMsgEncodingType && self.pSigningCert == other.pSigningCert && self.HashAlgorithm == other.HashAlgorithm && self.pvHashAuxInfo == other.pvHashAuxInfo && self.cMsgCert == other.cMsgCert && self.rgpMsgCert == other.rgpMsgCert && self.cMsgCrl == other.cMsgCrl && self.rgpMsgCrl == other.rgpMsgCrl && self.cAuthAttr == other.cAuthAttr && self.rgAuthAttr == other.rgAuthAttr && self.cUnauthAttr == other.cUnauthAttr && self.rgUnauthAttr == other.rgUnauthAttr && self.dwFlags == other.dwFlags && self.dwInnerContentType == other.dwInnerContentType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPT_SIGN_MESSAGE_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPT_SIGN_MESSAGE_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_SIGN_MESSAGE_PARA")
            .field("cbSize", &self.cbSize)
            .field("dwMsgEncodingType", &self.dwMsgEncodingType)
            .field("pSigningCert", &self.pSigningCert)
            .field("HashAlgorithm", &self.HashAlgorithm)
            .field("pvHashAuxInfo", &self.pvHashAuxInfo)
            .field("cMsgCert", &self.cMsgCert)
            .field("rgpMsgCert", &self.rgpMsgCert)
            .field("cMsgCrl", &self.cMsgCrl)
            .field("rgpMsgCrl", &self.rgpMsgCrl)
            .field("cAuthAttr", &self.cAuthAttr)
            .field("rgAuthAttr", &self.rgAuthAttr)
            .field("cUnauthAttr", &self.cUnauthAttr)
            .field("rgUnauthAttr", &self.rgUnauthAttr)
            .field("dwFlags", &self.dwFlags)
            .field("dwInnerContentType", &self.dwInnerContentType)
            .finish()
    }
}
impl ::core::default::Default for CRYPT_SMART_CARD_ROOT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_SMART_CARD_ROOT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.rgbCardID == other.rgbCardID && self.luid == other.luid
    }
}
impl ::core::cmp::Eq for CRYPT_SMART_CARD_ROOT_INFO {}
impl ::core::fmt::Debug for CRYPT_SMART_CARD_ROOT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_SMART_CARD_ROOT_INFO").field("rgbCardID", &self.rgbCardID).field("luid", &self.luid).finish()
    }
}
impl ::core::default::Default for CRYPT_SMIME_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_SMIME_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.cCapability == other.cCapability && self.rgCapability == other.rgCapability
    }
}
impl ::core::cmp::Eq for CRYPT_SMIME_CAPABILITIES {}
impl ::core::fmt::Debug for CRYPT_SMIME_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_SMIME_CAPABILITIES").field("cCapability", &self.cCapability).field("rgCapability", &self.rgCapability).finish()
    }
}
impl ::core::default::Default for CRYPT_SMIME_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_SMIME_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.pszObjId == other.pszObjId && self.Parameters == other.Parameters
    }
}
impl ::core::cmp::Eq for CRYPT_SMIME_CAPABILITY {}
impl ::core::fmt::Debug for CRYPT_SMIME_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_SMIME_CAPABILITY").field("pszObjId", &self.pszObjId).field("Parameters", &self.Parameters).finish()
    }
}
impl ::core::default::Default for CRYPT_STRING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_STRING").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPT_TIMESTAMP_ACCURACY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_TIMESTAMP_ACCURACY {
    fn eq(&self, other: &Self) -> bool {
        self.dwSeconds == other.dwSeconds && self.dwMillis == other.dwMillis && self.dwMicros == other.dwMicros
    }
}
impl ::core::cmp::Eq for CRYPT_TIMESTAMP_ACCURACY {}
impl ::core::fmt::Debug for CRYPT_TIMESTAMP_ACCURACY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_TIMESTAMP_ACCURACY").field("dwSeconds", &self.dwSeconds).field("dwMillis", &self.dwMillis).field("dwMicros", &self.dwMicros).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPT_TIMESTAMP_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPT_TIMESTAMP_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cbEncoded == other.cbEncoded && self.pbEncoded == other.pbEncoded && self.pTimeStamp == other.pTimeStamp
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPT_TIMESTAMP_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPT_TIMESTAMP_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_TIMESTAMP_CONTEXT").field("cbEncoded", &self.cbEncoded).field("pbEncoded", &self.pbEncoded).field("pTimeStamp", &self.pTimeStamp).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPT_TIMESTAMP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPT_TIMESTAMP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.pszTSAPolicyId == other.pszTSAPolicyId && self.HashAlgorithm == other.HashAlgorithm && self.HashedMessage == other.HashedMessage && self.SerialNumber == other.SerialNumber && self.ftTime == other.ftTime && self.pvAccuracy == other.pvAccuracy && self.fOrdering == other.fOrdering && self.Nonce == other.Nonce && self.Tsa == other.Tsa && self.cExtension == other.cExtension && self.rgExtension == other.rgExtension
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPT_TIMESTAMP_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPT_TIMESTAMP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_TIMESTAMP_INFO")
            .field("dwVersion", &self.dwVersion)
            .field("pszTSAPolicyId", &self.pszTSAPolicyId)
            .field("HashAlgorithm", &self.HashAlgorithm)
            .field("HashedMessage", &self.HashedMessage)
            .field("SerialNumber", &self.SerialNumber)
            .field("ftTime", &self.ftTime)
            .field("pvAccuracy", &self.pvAccuracy)
            .field("fOrdering", &self.fOrdering)
            .field("Nonce", &self.Nonce)
            .field("Tsa", &self.Tsa)
            .field("cExtension", &self.cExtension)
            .field("rgExtension", &self.rgExtension)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPT_TIMESTAMP_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPT_TIMESTAMP_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.pszTSAPolicyId == other.pszTSAPolicyId && self.fRequestCerts == other.fRequestCerts && self.Nonce == other.Nonce && self.cExtension == other.cExtension && self.rgExtension == other.rgExtension
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPT_TIMESTAMP_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPT_TIMESTAMP_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_TIMESTAMP_PARA").field("pszTSAPolicyId", &self.pszTSAPolicyId).field("fRequestCerts", &self.fRequestCerts).field("Nonce", &self.Nonce).field("cExtension", &self.cExtension).field("rgExtension", &self.rgExtension).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPT_TIMESTAMP_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPT_TIMESTAMP_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.HashAlgorithm == other.HashAlgorithm && self.HashedMessage == other.HashedMessage && self.pszTSAPolicyId == other.pszTSAPolicyId && self.Nonce == other.Nonce && self.fCertReq == other.fCertReq && self.cExtension == other.cExtension && self.rgExtension == other.rgExtension
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPT_TIMESTAMP_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPT_TIMESTAMP_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_TIMESTAMP_REQUEST").field("dwVersion", &self.dwVersion).field("HashAlgorithm", &self.HashAlgorithm).field("HashedMessage", &self.HashedMessage).field("pszTSAPolicyId", &self.pszTSAPolicyId).field("Nonce", &self.Nonce).field("fCertReq", &self.fCertReq).field("cExtension", &self.cExtension).field("rgExtension", &self.rgExtension).finish()
    }
}
impl ::core::default::Default for CRYPT_TIMESTAMP_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_TIMESTAMP_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.dwStatus == other.dwStatus && self.cFreeText == other.cFreeText && self.rgFreeText == other.rgFreeText && self.FailureInfo == other.FailureInfo && self.ContentInfo == other.ContentInfo
    }
}
impl ::core::cmp::Eq for CRYPT_TIMESTAMP_RESPONSE {}
impl ::core::fmt::Debug for CRYPT_TIMESTAMP_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_TIMESTAMP_RESPONSE").field("dwStatus", &self.dwStatus).field("cFreeText", &self.cFreeText).field("rgFreeText", &self.rgFreeText).field("FailureInfo", &self.FailureInfo).field("ContentInfo", &self.ContentInfo).finish()
    }
}
impl ::core::default::Default for CRYPT_TIMESTAMP_RESPONSE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_TIMESTAMP_RESPONSE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_TIMESTAMP_RESPONSE_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPT_TIMESTAMP_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_TIMESTAMP_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_TIMESTAMP_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPT_TIME_STAMP_REQUEST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_TIME_STAMP_REQUEST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszTimeStampAlgorithm == other.pszTimeStampAlgorithm && self.pszContentType == other.pszContentType && self.Content == other.Content && self.cAttribute == other.cAttribute && self.rgAttribute == other.rgAttribute
    }
}
impl ::core::cmp::Eq for CRYPT_TIME_STAMP_REQUEST_INFO {}
impl ::core::fmt::Debug for CRYPT_TIME_STAMP_REQUEST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_TIME_STAMP_REQUEST_INFO").field("pszTimeStampAlgorithm", &self.pszTimeStampAlgorithm).field("pszContentType", &self.pszContentType).field("Content", &self.Content).field("cAttribute", &self.cAttribute).field("rgAttribute", &self.rgAttribute).finish()
    }
}
impl ::core::default::Default for CRYPT_URL_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_URL_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.cUrl == other.cUrl && self.rgwszUrl == other.rgwszUrl
    }
}
impl ::core::cmp::Eq for CRYPT_URL_ARRAY {}
impl ::core::fmt::Debug for CRYPT_URL_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_URL_ARRAY").field("cUrl", &self.cUrl).field("rgwszUrl", &self.rgwszUrl).finish()
    }
}
impl ::core::default::Default for CRYPT_URL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_URL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwSyncDeltaTime == other.dwSyncDeltaTime && self.cGroup == other.cGroup && self.rgcGroupEntry == other.rgcGroupEntry
    }
}
impl ::core::cmp::Eq for CRYPT_URL_INFO {}
impl ::core::fmt::Debug for CRYPT_URL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_URL_INFO").field("cbSize", &self.cbSize).field("dwSyncDeltaTime", &self.dwSyncDeltaTime).field("cGroup", &self.cGroup).field("rgcGroupEntry", &self.rgcGroupEntry).finish()
    }
}
impl ::core::default::Default for CRYPT_VERIFY_CERT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_VERIFY_CERT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_VERIFY_CERT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CertSignHashCNGAlgPropData == other.CertSignHashCNGAlgPropData && self.CertIssuerPubKeyBitLengthPropData == other.CertIssuerPubKeyBitLengthPropData
    }
}
impl ::core::cmp::Eq for CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO {}
impl ::core::fmt::Debug for CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO").field("CertSignHashCNGAlgPropData", &self.CertSignHashCNGAlgPropData).field("CertIssuerPubKeyBitLengthPropData", &self.CertIssuerPubKeyBitLengthPropData).finish()
    }
}
impl ::core::default::Default for CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cCNGHashAlgid == other.cCNGHashAlgid && self.rgpwszCNGHashAlgid == other.rgpwszCNGHashAlgid && self.dwWeakIndex == other.dwWeakIndex
    }
}
impl ::core::cmp::Eq for CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO {}
impl ::core::fmt::Debug for CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO").field("cCNGHashAlgid", &self.cCNGHashAlgid).field("rgpwszCNGHashAlgid", &self.rgpwszCNGHashAlgid).field("dwWeakIndex", &self.dwWeakIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPT_VERIFY_MESSAGE_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPT_X942_OTHER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_X942_OTHER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszContentEncryptionObjId == other.pszContentEncryptionObjId && self.rgbCounter == other.rgbCounter && self.rgbKeyLength == other.rgbKeyLength && self.PubInfo == other.PubInfo
    }
}
impl ::core::cmp::Eq for CRYPT_X942_OTHER_INFO {}
impl ::core::fmt::Debug for CRYPT_X942_OTHER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_X942_OTHER_INFO").field("pszContentEncryptionObjId", &self.pszContentEncryptionObjId).field("rgbCounter", &self.rgbCounter).field("rgbKeyLength", &self.rgbKeyLength).field("PubInfo", &self.PubInfo).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_ALGORITHM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_ALGORITHM {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.wszAlgorithm == other.wszAlgorithm && self.Encoded == other.Encoded
    }
}
impl ::core::cmp::Eq for CRYPT_XML_ALGORITHM {}
impl ::core::fmt::Debug for CRYPT_XML_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_ALGORITHM").field("cbSize", &self.cbSize).field("wszAlgorithm", &self.wszAlgorithm).field("Encoded", &self.Encoded).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_ALGORITHM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_ALGORITHM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.wszAlgorithmURI == other.wszAlgorithmURI && self.wszName == other.wszName && self.dwGroupId == other.dwGroupId && self.wszCNGAlgid == other.wszCNGAlgid && self.wszCNGExtraAlgid == other.wszCNGExtraAlgid && self.dwSignFlags == other.dwSignFlags && self.dwVerifyFlags == other.dwVerifyFlags && self.pvPaddingInfo == other.pvPaddingInfo && self.pvExtraInfo == other.pvExtraInfo
    }
}
impl ::core::cmp::Eq for CRYPT_XML_ALGORITHM_INFO {}
impl ::core::fmt::Debug for CRYPT_XML_ALGORITHM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_ALGORITHM_INFO").field("cbSize", &self.cbSize).field("wszAlgorithmURI", &self.wszAlgorithmURI).field("wszName", &self.wszName).field("dwGroupId", &self.dwGroupId).field("wszCNGAlgid", &self.wszCNGAlgid).field("wszCNGExtraAlgid", &self.wszCNGExtraAlgid).field("dwSignFlags", &self.dwSignFlags).field("dwVerifyFlags", &self.dwVerifyFlags).field("pvPaddingInfo", &self.pvPaddingInfo).field("pvExtraInfo", &self.pvExtraInfo).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwCharset == other.dwCharset && self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for CRYPT_XML_BLOB {}
impl ::core::fmt::Debug for CRYPT_XML_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_BLOB").field("dwCharset", &self.dwCharset).field("cbData", &self.cbData).field("pbData", &self.pbData).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_CHARSET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_XML_CHARSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_XML_CHARSET").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_CRYPTOGRAPHIC_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPT_XML_DATA_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_DATA_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for CRYPT_XML_DATA_BLOB {}
impl ::core::fmt::Debug for CRYPT_XML_DATA_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_DATA_BLOB").field("cbData", &self.cbData).field("pbData", &self.pbData).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_DATA_PROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPT_XML_DOC_CTXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_DOC_CTXT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hDocCtxt == other.hDocCtxt && self.pTransformsConfig == other.pTransformsConfig && self.cSignature == other.cSignature && self.rgpSignature == other.rgpSignature
    }
}
impl ::core::cmp::Eq for CRYPT_XML_DOC_CTXT {}
impl ::core::fmt::Debug for CRYPT_XML_DOC_CTXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_DOC_CTXT").field("cbSize", &self.cbSize).field("hDocCtxt", &self.hDocCtxt).field("pTransformsConfig", &self.pTransformsConfig).field("cSignature", &self.cSignature).field("rgpSignature", &self.rgpSignature).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_XML_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_XML_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_GROUP_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_XML_GROUP_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_XML_GROUP_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_ISSUER_SERIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_ISSUER_SERIAL {
    fn eq(&self, other: &Self) -> bool {
        self.wszIssuer == other.wszIssuer && self.wszSerial == other.wszSerial
    }
}
impl ::core::cmp::Eq for CRYPT_XML_ISSUER_SERIAL {}
impl ::core::fmt::Debug for CRYPT_XML_ISSUER_SERIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_ISSUER_SERIAL").field("wszIssuer", &self.wszIssuer).field("wszSerial", &self.wszSerial).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_KEYINFO_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_KEYINFO_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.wszId == other.wszId && self.wszKeyName == other.wszKeyName && self.SKI == other.SKI && self.wszSubjectName == other.wszSubjectName && self.cCertificate == other.cCertificate && self.rgCertificate == other.rgCertificate && self.cCRL == other.cCRL && self.rgCRL == other.rgCRL
    }
}
impl ::core::cmp::Eq for CRYPT_XML_KEYINFO_PARAM {}
impl ::core::fmt::Debug for CRYPT_XML_KEYINFO_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_KEYINFO_PARAM").field("wszId", &self.wszId).field("wszKeyName", &self.wszKeyName).field("SKI", &self.SKI).field("wszSubjectName", &self.wszSubjectName).field("cCertificate", &self.cCertificate).field("rgCertificate", &self.rgCertificate).field("cCRL", &self.cCRL).field("rgCRL", &self.rgCRL).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_KEYINFO_SPEC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_XML_KEYINFO_SPEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_XML_KEYINFO_SPEC").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_KEYINFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_XML_KEYINFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_XML_KEYINFO_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_KEY_DSA_KEY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_KEY_DSA_KEY_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.P == other.P && self.Q == other.Q && self.G == other.G && self.Y == other.Y && self.J == other.J && self.Seed == other.Seed && self.Counter == other.Counter
    }
}
impl ::core::cmp::Eq for CRYPT_XML_KEY_DSA_KEY_VALUE {}
impl ::core::fmt::Debug for CRYPT_XML_KEY_DSA_KEY_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_KEY_DSA_KEY_VALUE").field("P", &self.P).field("Q", &self.Q).field("G", &self.G).field("Y", &self.Y).field("J", &self.J).field("Seed", &self.Seed).field("Counter", &self.Counter).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_KEY_ECDSA_KEY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_KEY_ECDSA_KEY_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.wszNamedCurve == other.wszNamedCurve && self.X == other.X && self.Y == other.Y && self.ExplicitPara == other.ExplicitPara
    }
}
impl ::core::cmp::Eq for CRYPT_XML_KEY_ECDSA_KEY_VALUE {}
impl ::core::fmt::Debug for CRYPT_XML_KEY_ECDSA_KEY_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_KEY_ECDSA_KEY_VALUE").field("wszNamedCurve", &self.wszNamedCurve).field("X", &self.X).field("Y", &self.Y).field("ExplicitPara", &self.ExplicitPara).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_KEY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.wszId == other.wszId && self.cKeyInfo == other.cKeyInfo && self.rgKeyInfo == other.rgKeyInfo && self.hVerifyKey == other.hVerifyKey
    }
}
impl ::core::cmp::Eq for CRYPT_XML_KEY_INFO {}
impl ::core::fmt::Debug for CRYPT_XML_KEY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_KEY_INFO").field("cbSize", &self.cbSize).field("wszId", &self.wszId).field("cKeyInfo", &self.cKeyInfo).field("rgKeyInfo", &self.rgKeyInfo).field("hVerifyKey", &self.hVerifyKey).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_KEY_INFO_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPT_XML_KEY_RSA_KEY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_KEY_RSA_KEY_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.Modulus == other.Modulus && self.Exponent == other.Exponent
    }
}
impl ::core::cmp::Eq for CRYPT_XML_KEY_RSA_KEY_VALUE {}
impl ::core::fmt::Debug for CRYPT_XML_KEY_RSA_KEY_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_KEY_RSA_KEY_VALUE").field("Modulus", &self.Modulus).field("Exponent", &self.Exponent).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_KEY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPT_XML_KEY_VALUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_XML_KEY_VALUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_XML_KEY_VALUE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hObject == other.hObject && self.wszId == other.wszId && self.wszMimeType == other.wszMimeType && self.wszEncoding == other.wszEncoding && self.Manifest == other.Manifest && self.Encoded == other.Encoded
    }
}
impl ::core::cmp::Eq for CRYPT_XML_OBJECT {}
impl ::core::fmt::Debug for CRYPT_XML_OBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_OBJECT").field("cbSize", &self.cbSize).field("hObject", &self.hObject).field("wszId", &self.wszId).field("wszMimeType", &self.wszMimeType).field("wszEncoding", &self.wszEncoding).field("Manifest", &self.Manifest).field("Encoded", &self.Encoded).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.dwPropId == other.dwPropId && self.pvValue == other.pvValue && self.cbValue == other.cbValue
    }
}
impl ::core::cmp::Eq for CRYPT_XML_PROPERTY {}
impl ::core::fmt::Debug for CRYPT_XML_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_PROPERTY").field("dwPropId", &self.dwPropId).field("pvValue", &self.pvValue).field("cbValue", &self.cbValue).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_XML_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_XML_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_REFERENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_REFERENCE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hReference == other.hReference && self.wszId == other.wszId && self.wszUri == other.wszUri && self.wszType == other.wszType && self.DigestMethod == other.DigestMethod && self.DigestValue == other.DigestValue && self.cTransform == other.cTransform && self.rgTransform == other.rgTransform
    }
}
impl ::core::cmp::Eq for CRYPT_XML_REFERENCE {}
impl ::core::fmt::Debug for CRYPT_XML_REFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_REFERENCE").field("cbSize", &self.cbSize).field("hReference", &self.hReference).field("wszId", &self.wszId).field("wszUri", &self.wszUri).field("wszType", &self.wszType).field("DigestMethod", &self.DigestMethod).field("DigestValue", &self.DigestValue).field("cTransform", &self.cTransform).field("rgTransform", &self.rgTransform).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_REFERENCES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_REFERENCES {
    fn eq(&self, other: &Self) -> bool {
        self.cReference == other.cReference && self.rgpReference == other.rgpReference
    }
}
impl ::core::cmp::Eq for CRYPT_XML_REFERENCES {}
impl ::core::fmt::Debug for CRYPT_XML_REFERENCES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_REFERENCES").field("cReference", &self.cReference).field("rgpReference", &self.rgpReference).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_SIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hSignature == other.hSignature && self.wszId == other.wszId && self.SignedInfo == other.SignedInfo && self.SignatureValue == other.SignatureValue && self.pKeyInfo == other.pKeyInfo && self.cObject == other.cObject && self.rgpObject == other.rgpObject
    }
}
impl ::core::cmp::Eq for CRYPT_XML_SIGNATURE {}
impl ::core::fmt::Debug for CRYPT_XML_SIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_SIGNATURE").field("cbSize", &self.cbSize).field("hSignature", &self.hSignature).field("wszId", &self.wszId).field("SignedInfo", &self.SignedInfo).field("SignatureValue", &self.SignatureValue).field("pKeyInfo", &self.pKeyInfo).field("cObject", &self.cObject).field("rgpObject", &self.rgpObject).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_SIGNED_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_SIGNED_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.wszId == other.wszId && self.Canonicalization == other.Canonicalization && self.SignatureMethod == other.SignatureMethod && self.cReference == other.cReference && self.rgpReference == other.rgpReference && self.Encoded == other.Encoded
    }
}
impl ::core::cmp::Eq for CRYPT_XML_SIGNED_INFO {}
impl ::core::fmt::Debug for CRYPT_XML_SIGNED_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_SIGNED_INFO").field("cbSize", &self.cbSize).field("wszId", &self.wszId).field("Canonicalization", &self.Canonicalization).field("SignatureMethod", &self.SignatureMethod).field("cReference", &self.cReference).field("rgpReference", &self.rgpReference).field("Encoded", &self.Encoded).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwErrorStatus == other.dwErrorStatus && self.dwInfoStatus == other.dwInfoStatus
    }
}
impl ::core::cmp::Eq for CRYPT_XML_STATUS {}
impl ::core::fmt::Debug for CRYPT_XML_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_STATUS").field("cbSize", &self.cbSize).field("dwErrorStatus", &self.dwErrorStatus).field("dwInfoStatus", &self.dwInfoStatus).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_STATUS_ERROR_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_XML_STATUS_ERROR_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_XML_STATUS_ERROR_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_STATUS_INFO_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_XML_STATUS_INFO_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_XML_STATUS_INFO_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_TRANSFORM_CHAIN_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_TRANSFORM_CHAIN_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cTransformInfo == other.cTransformInfo && self.rgpTransformInfo == other.rgpTransformInfo
    }
}
impl ::core::cmp::Eq for CRYPT_XML_TRANSFORM_CHAIN_CONFIG {}
impl ::core::fmt::Debug for CRYPT_XML_TRANSFORM_CHAIN_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_TRANSFORM_CHAIN_CONFIG").field("cbSize", &self.cbSize).field("cTransformInfo", &self.cTransformInfo).field("rgpTransformInfo", &self.rgpTransformInfo).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_TRANSFORM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_XML_TRANSFORM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_XML_TRANSFORM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CRYPT_XML_TRANSFORM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRYPT_XML_TRANSFORM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRYPT_XML_TRANSFORM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRYPT_XML_TRANSFORM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRYPT_XML_TRANSFORM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CRYPT_XML_TRANSFORM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPT_XML_X509DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPT_XML_X509DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cX509Data == other.cX509Data && self.rgX509Data == other.rgX509Data
    }
}
impl ::core::cmp::Eq for CRYPT_XML_X509DATA {}
impl ::core::fmt::Debug for CRYPT_XML_X509DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_XML_X509DATA").field("cX509Data", &self.cX509Data).field("rgX509Data", &self.rgX509Data).finish()
    }
}
impl ::core::default::Default for CRYPT_XML_X509DATA_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPT_XML_X509DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPT_XML_X509DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPT_XML_X509DATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CTL_ANY_SUBJECT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CTL_ANY_SUBJECT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SubjectAlgorithm == other.SubjectAlgorithm && self.SubjectIdentifier == other.SubjectIdentifier
    }
}
impl ::core::cmp::Eq for CTL_ANY_SUBJECT_INFO {}
impl ::core::fmt::Debug for CTL_ANY_SUBJECT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CTL_ANY_SUBJECT_INFO").field("SubjectAlgorithm", &self.SubjectAlgorithm).field("SubjectIdentifier", &self.SubjectIdentifier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CTL_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CTL_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.dwMsgAndCertEncodingType == other.dwMsgAndCertEncodingType && self.pbCtlEncoded == other.pbCtlEncoded && self.cbCtlEncoded == other.cbCtlEncoded && self.pCtlInfo == other.pCtlInfo && self.hCertStore == other.hCertStore && self.hCryptMsg == other.hCryptMsg && self.pbCtlContent == other.pbCtlContent && self.cbCtlContent == other.cbCtlContent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CTL_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CTL_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CTL_CONTEXT").field("dwMsgAndCertEncodingType", &self.dwMsgAndCertEncodingType).field("pbCtlEncoded", &self.pbCtlEncoded).field("cbCtlEncoded", &self.cbCtlEncoded).field("pCtlInfo", &self.pCtlInfo).field("hCertStore", &self.hCertStore).field("hCryptMsg", &self.hCryptMsg).field("pbCtlContent", &self.pbCtlContent).field("cbCtlContent", &self.cbCtlContent).finish()
    }
}
impl ::core::default::Default for CTL_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CTL_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.SubjectIdentifier == other.SubjectIdentifier && self.cAttribute == other.cAttribute && self.rgAttribute == other.rgAttribute
    }
}
impl ::core::cmp::Eq for CTL_ENTRY {}
impl ::core::fmt::Debug for CTL_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CTL_ENTRY").field("SubjectIdentifier", &self.SubjectIdentifier).field("cAttribute", &self.cAttribute).field("rgAttribute", &self.rgAttribute).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CTL_FIND_SUBJECT_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CTL_FIND_SUBJECT_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pUsagePara == other.pUsagePara && self.dwSubjectType == other.dwSubjectType && self.pvSubject == other.pvSubject
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CTL_FIND_SUBJECT_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CTL_FIND_SUBJECT_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CTL_FIND_SUBJECT_PARA").field("cbSize", &self.cbSize).field("pUsagePara", &self.pUsagePara).field("dwSubjectType", &self.dwSubjectType).field("pvSubject", &self.pvSubject).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CTL_FIND_USAGE_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CTL_FIND_USAGE_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.SubjectUsage == other.SubjectUsage && self.ListIdentifier == other.ListIdentifier && self.pSigner == other.pSigner
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CTL_FIND_USAGE_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CTL_FIND_USAGE_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CTL_FIND_USAGE_PARA").field("cbSize", &self.cbSize).field("SubjectUsage", &self.SubjectUsage).field("ListIdentifier", &self.ListIdentifier).field("pSigner", &self.pSigner).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CTL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CTL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.SubjectUsage == other.SubjectUsage && self.ListIdentifier == other.ListIdentifier && self.SequenceNumber == other.SequenceNumber && self.ThisUpdate == other.ThisUpdate && self.NextUpdate == other.NextUpdate && self.SubjectAlgorithm == other.SubjectAlgorithm && self.cCTLEntry == other.cCTLEntry && self.rgCTLEntry == other.rgCTLEntry && self.cExtension == other.cExtension && self.rgExtension == other.rgExtension
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CTL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CTL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CTL_INFO")
            .field("dwVersion", &self.dwVersion)
            .field("SubjectUsage", &self.SubjectUsage)
            .field("ListIdentifier", &self.ListIdentifier)
            .field("SequenceNumber", &self.SequenceNumber)
            .field("ThisUpdate", &self.ThisUpdate)
            .field("NextUpdate", &self.NextUpdate)
            .field("SubjectAlgorithm", &self.SubjectAlgorithm)
            .field("cCTLEntry", &self.cCTLEntry)
            .field("rgCTLEntry", &self.rgCTLEntry)
            .field("cExtension", &self.cExtension)
            .field("rgExtension", &self.rgExtension)
            .finish()
    }
}
impl ::core::default::Default for CTL_USAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CTL_USAGE {
    fn eq(&self, other: &Self) -> bool {
        self.cUsageIdentifier == other.cUsageIdentifier && self.rgpszUsageIdentifier == other.rgpszUsageIdentifier
    }
}
impl ::core::cmp::Eq for CTL_USAGE {}
impl ::core::fmt::Debug for CTL_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CTL_USAGE").field("cUsageIdentifier", &self.cUsageIdentifier).field("rgpszUsageIdentifier", &self.rgpszUsageIdentifier).finish()
    }
}
impl ::core::default::Default for CTL_USAGE_MATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CTL_USAGE_MATCH {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.Usage == other.Usage
    }
}
impl ::core::cmp::Eq for CTL_USAGE_MATCH {}
impl ::core::fmt::Debug for CTL_USAGE_MATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CTL_USAGE_MATCH").field("dwType", &self.dwType).field("Usage", &self.Usage).finish()
    }
}
impl ::core::default::Default for CTL_VERIFY_USAGE_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CTL_VERIFY_USAGE_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ListIdentifier == other.ListIdentifier && self.cCtlStore == other.cCtlStore && self.rghCtlStore == other.rghCtlStore && self.cSignerStore == other.cSignerStore && self.rghSignerStore == other.rghSignerStore
    }
}
impl ::core::cmp::Eq for CTL_VERIFY_USAGE_PARA {}
impl ::core::fmt::Debug for CTL_VERIFY_USAGE_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CTL_VERIFY_USAGE_PARA").field("cbSize", &self.cbSize).field("ListIdentifier", &self.ListIdentifier).field("cCtlStore", &self.cCtlStore).field("rghCtlStore", &self.rghCtlStore).field("cSignerStore", &self.cSignerStore).field("rghSignerStore", &self.rghSignerStore).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CTL_VERIFY_USAGE_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CTL_VERIFY_USAGE_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwError == other.dwError && self.dwFlags == other.dwFlags && self.ppCtl == other.ppCtl && self.dwCtlEntryIndex == other.dwCtlEntryIndex && self.ppSigner == other.ppSigner && self.dwSignerIndex == other.dwSignerIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CTL_VERIFY_USAGE_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CTL_VERIFY_USAGE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CTL_VERIFY_USAGE_STATUS").field("cbSize", &self.cbSize).field("dwError", &self.dwError).field("dwFlags", &self.dwFlags).field("ppCtl", &self.ppCtl).field("dwCtlEntryIndex", &self.dwCtlEntryIndex).field("ppSigner", &self.ppSigner).field("dwSignerIndex", &self.dwSignerIndex).finish()
    }
}
impl ::core::default::Default for CertKeyType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CertKeyType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CertKeyType").field(&self.0).finish()
    }
}
impl ::core::default::Default for DSAFIPSVERSION_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DSAFIPSVERSION_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSAFIPSVERSION_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DSSSEED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSSSEED {
    fn eq(&self, other: &Self) -> bool {
        self.counter == other.counter && self.seed == other.seed
    }
}
impl ::core::cmp::Eq for DSSSEED {}
impl ::core::fmt::Debug for DSSSEED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSSSEED").field("counter", &self.counter).field("seed", &self.seed).finish()
    }
}
impl ::core::default::Default for Direction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Direction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Direction").field(&self.0).finish()
    }
}
impl ::core::default::Default for ECC_CURVE_ALG_ID_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ECC_CURVE_ALG_ID_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ECC_CURVE_ALG_ID_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ECC_CURVE_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ECC_CURVE_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ECC_CURVE_TYPE_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ENDPOINTADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENDPOINTADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.serviceUrl == other.serviceUrl && self.policyUrl == other.policyUrl && self.rawCertificate == other.rawCertificate
    }
}
impl ::core::cmp::Eq for ENDPOINTADDRESS {}
impl ::core::fmt::Debug for ENDPOINTADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENDPOINTADDRESS").field("serviceUrl", &self.serviceUrl).field("policyUrl", &self.policyUrl).field("rawCertificate", &self.rawCertificate).finish()
    }
}
impl ::core::default::Default for ENDPOINTADDRESS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENDPOINTADDRESS2 {
    fn eq(&self, other: &Self) -> bool {
        self.serviceUrl == other.serviceUrl && self.policyUrl == other.policyUrl && self.identityType == other.identityType && self.identityBytes == other.identityBytes
    }
}
impl ::core::cmp::Eq for ENDPOINTADDRESS2 {}
impl ::core::fmt::Debug for ENDPOINTADDRESS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENDPOINTADDRESS2").field("serviceUrl", &self.serviceUrl).field("policyUrl", &self.policyUrl).field("identityType", &self.identityType).field("identityBytes", &self.identityBytes).finish()
    }
}
impl ::core::default::Default for EV_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EV_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwRootProgramQualifierFlags == other.dwRootProgramQualifierFlags
    }
}
impl ::core::cmp::Eq for EV_EXTRA_CERT_CHAIN_POLICY_PARA {}
impl ::core::fmt::Debug for EV_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EV_EXTRA_CERT_CHAIN_POLICY_PARA").field("cbSize", &self.cbSize).field("dwRootProgramQualifierFlags", &self.dwRootProgramQualifierFlags).finish()
    }
}
impl ::core::default::Default for EV_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EV_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwQualifiers == other.dwQualifiers && self.dwIssuanceUsageIndex == other.dwIssuanceUsageIndex
    }
}
impl ::core::cmp::Eq for EV_EXTRA_CERT_CHAIN_POLICY_STATUS {}
impl ::core::fmt::Debug for EV_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EV_EXTRA_CERT_CHAIN_POLICY_STATUS").field("cbSize", &self.cbSize).field("dwQualifiers", &self.dwQualifiers).field("dwIssuanceUsageIndex", &self.dwIssuanceUsageIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GENERIC_XML_TOKEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HASHALGORITHM_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HASHALGORITHM_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HASHALGORITHM_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for HMAC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HMAC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.HashAlgid == other.HashAlgid && self.pbInnerString == other.pbInnerString && self.cbInnerString == other.cbInnerString && self.pbOuterString == other.pbOuterString && self.cbOuterString == other.cbOuterString
    }
}
impl ::core::cmp::Eq for HMAC_INFO {}
impl ::core::fmt::Debug for HMAC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HMAC_INFO").field("HashAlgid", &self.HashAlgid).field("pbInnerString", &self.pbInnerString).field("cbInnerString", &self.cbInnerString).field("pbOuterString", &self.pbOuterString).field("cbOuterString", &self.cbOuterString).finish()
    }
}
impl ::core::default::Default for HTTPSPOLICY_CALLBACK_DATA_AUTH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HTTPSPOLICY_CALLBACK_DATA_AUTH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTPSPOLICY_CALLBACK_DATA_AUTH_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HTTPSPolicyCallbackData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HandleType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HandleType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HandleType").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertSrvSetup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertSrvSetup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertSrvSetup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertSrvSetup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertSrvSetupKeyInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertSrvSetupKeyInformation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertSrvSetupKeyInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertSrvSetupKeyInformation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertSrvSetupKeyInformationCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertSrvSetupKeyInformationCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertSrvSetupKeyInformationCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertSrvSetupKeyInformationCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertificateEnrollmentPolicyServerSetup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertificateEnrollmentPolicyServerSetup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertificateEnrollmentPolicyServerSetup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertificateEnrollmentPolicyServerSetup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertificateEnrollmentServerSetup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertificateEnrollmentServerSetup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertificateEnrollmentServerSetup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertificateEnrollmentServerSetup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSCEPSetup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSCEPSetup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSCEPSetup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSCEPSetup").field(&self.0).finish()
    }
}
impl ::core::default::Default for INFORMATIONCARD_ASYMMETRIC_CRYPTO_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INFORMATIONCARD_ASYMMETRIC_CRYPTO_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.keySize == other.keySize && self.keyExchangeAlgorithm == other.keyExchangeAlgorithm && self.signatureAlgorithm == other.signatureAlgorithm
    }
}
impl ::core::cmp::Eq for INFORMATIONCARD_ASYMMETRIC_CRYPTO_PARAMETERS {}
impl ::core::fmt::Debug for INFORMATIONCARD_ASYMMETRIC_CRYPTO_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INFORMATIONCARD_ASYMMETRIC_CRYPTO_PARAMETERS").field("keySize", &self.keySize).field("keyExchangeAlgorithm", &self.keyExchangeAlgorithm).field("signatureAlgorithm", &self.signatureAlgorithm).finish()
    }
}
impl ::core::default::Default for INFORMATIONCARD_CRYPTO_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INFORMATIONCARD_CRYPTO_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.expiration == other.expiration && self.cryptoParameters == other.cryptoParameters
    }
}
impl ::core::cmp::Eq for INFORMATIONCARD_CRYPTO_HANDLE {}
impl ::core::fmt::Debug for INFORMATIONCARD_CRYPTO_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INFORMATIONCARD_CRYPTO_HANDLE").field("type", &self.r#type).field("expiration", &self.expiration).field("cryptoParameters", &self.cryptoParameters).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INFORMATIONCARD_HASH_CRYPTO_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INFORMATIONCARD_HASH_CRYPTO_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.hashSize == other.hashSize && self.transform == other.transform
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INFORMATIONCARD_HASH_CRYPTO_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INFORMATIONCARD_HASH_CRYPTO_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INFORMATIONCARD_HASH_CRYPTO_PARAMETERS").field("hashSize", &self.hashSize).field("transform", &self.transform).finish()
    }
}
impl ::core::default::Default for INFORMATIONCARD_SYMMETRIC_CRYPTO_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INFORMATIONCARD_SYMMETRIC_CRYPTO_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.keySize == other.keySize && self.blockSize == other.blockSize && self.feedbackSize == other.feedbackSize
    }
}
impl ::core::cmp::Eq for INFORMATIONCARD_SYMMETRIC_CRYPTO_PARAMETERS {}
impl ::core::fmt::Debug for INFORMATIONCARD_SYMMETRIC_CRYPTO_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INFORMATIONCARD_SYMMETRIC_CRYPTO_PARAMETERS").field("keySize", &self.keySize).field("blockSize", &self.blockSize).field("feedbackSize", &self.feedbackSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INFORMATIONCARD_TRANSFORM_CRYPTO_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INFORMATIONCARD_TRANSFORM_CRYPTO_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.inputBlockSize == other.inputBlockSize && self.outputBlockSize == other.outputBlockSize && self.canTransformMultipleBlocks == other.canTransformMultipleBlocks && self.canReuseTransform == other.canReuseTransform
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INFORMATIONCARD_TRANSFORM_CRYPTO_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INFORMATIONCARD_TRANSFORM_CRYPTO_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INFORMATIONCARD_TRANSFORM_CRYPTO_PARAMETERS").field("inputBlockSize", &self.inputBlockSize).field("outputBlockSize", &self.outputBlockSize).field("canTransformMultipleBlocks", &self.canTransformMultipleBlocks).field("canReuseTransform", &self.canReuseTransform).finish()
    }
}
impl ::core::default::Default for KEY_TYPE_SUBTYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KEY_TYPE_SUBTYPE {
    fn eq(&self, other: &Self) -> bool {
        self.dwKeySpec == other.dwKeySpec && self.Type == other.Type && self.Subtype == other.Subtype
    }
}
impl ::core::cmp::Eq for KEY_TYPE_SUBTYPE {}
impl ::core::fmt::Debug for KEY_TYPE_SUBTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEY_TYPE_SUBTYPE").field("dwKeySpec", &self.dwKeySpec).field("Type", &self.Type).field("Subtype", &self.Subtype).finish()
    }
}
impl ::core::default::Default for MSCEPSetupProperty {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSCEPSetupProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSCEPSetupProperty").field(&self.0).finish()
    }
}
impl ::core::default::Default for NCRYPT_ALGORITHM_NAME_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NCRYPT_ALGORITHM_NAME_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NCRYPT_ALGORITHM_NAME_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for NCRYPT_ALLOC_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NCRYPT_CIPHER_PADDING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_CIPHER_PADDING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.pbIV == other.pbIV && self.cbIV == other.cbIV && self.pbOtherInfo == other.pbOtherInfo && self.cbOtherInfo == other.cbOtherInfo
    }
}
impl ::core::cmp::Eq for NCRYPT_CIPHER_PADDING_INFO {}
impl ::core::fmt::Debug for NCRYPT_CIPHER_PADDING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_CIPHER_PADDING_INFO").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("pbIV", &self.pbIV).field("cbIV", &self.cbIV).field("pbOtherInfo", &self.pbOtherInfo).field("cbOtherInfo", &self.cbOtherInfo).finish()
    }
}
impl ::core::default::Default for NCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
    }
}
impl ::core::cmp::Eq for NCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE {}
impl ::core::fmt::Debug for NCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE").field("Header", &self.Header).finish()
    }
}
impl ::core::default::Default for NCRYPT_EXPORTED_ISOLATED_KEY_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_EXPORTED_ISOLATED_KEY_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.KeyUsage == other.KeyUsage && self._bitfield == other._bitfield && self.cbAlgName == other.cbAlgName && self.cbNonce == other.cbNonce && self.cbAuthTag == other.cbAuthTag && self.cbWrappingKey == other.cbWrappingKey && self.cbIsolatedKey == other.cbIsolatedKey
    }
}
impl ::core::cmp::Eq for NCRYPT_EXPORTED_ISOLATED_KEY_HEADER {}
impl ::core::fmt::Debug for NCRYPT_EXPORTED_ISOLATED_KEY_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_EXPORTED_ISOLATED_KEY_HEADER").field("Version", &self.Version).field("KeyUsage", &self.KeyUsage).field("_bitfield", &self._bitfield).field("cbAlgName", &self.cbAlgName).field("cbNonce", &self.cbNonce).field("cbAuthTag", &self.cbAuthTag).field("cbWrappingKey", &self.cbWrappingKey).field("cbIsolatedKey", &self.cbIsolatedKey).finish()
    }
}
impl ::core::default::Default for NCRYPT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NCRYPT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NCRYPT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NCRYPT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NCRYPT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NCRYPT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NCRYPT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NCRYPT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.cbPublicKeyBlob == other.cbPublicKeyBlob
    }
}
impl ::core::cmp::Eq for NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES {}
impl ::core::fmt::Debug for NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES").field("Version", &self.Version).field("Flags", &self.Flags).field("cbPublicKeyBlob", &self.cbPublicKeyBlob).finish()
    }
}
impl ::core::default::Default for NCRYPT_KEY_ACCESS_POLICY_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_KEY_ACCESS_POLICY_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwPolicyFlags == other.dwPolicyFlags && self.cbUserSid == other.cbUserSid && self.cbApplicationSid == other.cbApplicationSid
    }
}
impl ::core::cmp::Eq for NCRYPT_KEY_ACCESS_POLICY_BLOB {}
impl ::core::fmt::Debug for NCRYPT_KEY_ACCESS_POLICY_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_KEY_ACCESS_POLICY_BLOB").field("dwVersion", &self.dwVersion).field("dwPolicyFlags", &self.dwPolicyFlags).field("cbUserSid", &self.cbUserSid).field("cbApplicationSid", &self.cbApplicationSid).finish()
    }
}
impl ::core::default::Default for NCRYPT_KEY_ATTEST_PADDING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_KEY_ATTEST_PADDING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.magic == other.magic && self.pbKeyBlob == other.pbKeyBlob && self.cbKeyBlob == other.cbKeyBlob && self.pbKeyAuth == other.pbKeyAuth && self.cbKeyAuth == other.cbKeyAuth
    }
}
impl ::core::cmp::Eq for NCRYPT_KEY_ATTEST_PADDING_INFO {}
impl ::core::fmt::Debug for NCRYPT_KEY_ATTEST_PADDING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_KEY_ATTEST_PADDING_INFO").field("magic", &self.magic).field("pbKeyBlob", &self.pbKeyBlob).field("cbKeyBlob", &self.cbKeyBlob).field("pbKeyAuth", &self.pbKeyAuth).field("cbKeyAuth", &self.cbKeyAuth).finish()
    }
}
impl ::core::default::Default for NCRYPT_KEY_BLOB_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_KEY_BLOB_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMagic == other.dwMagic && self.cbAlgName == other.cbAlgName && self.cbKeyData == other.cbKeyData
    }
}
impl ::core::cmp::Eq for NCRYPT_KEY_BLOB_HEADER {}
impl ::core::fmt::Debug for NCRYPT_KEY_BLOB_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_KEY_BLOB_HEADER").field("cbSize", &self.cbSize).field("dwMagic", &self.dwMagic).field("cbAlgName", &self.cbAlgName).field("cbKeyData", &self.cbKeyData).finish()
    }
}
impl ::core::default::Default for NCRYPT_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NCRYPT_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NCRYPT_OPERATION").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NCRYPT_OPERATION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NCRYPT_OPERATION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NCRYPT_OPERATION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NCRYPT_OPERATION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NCRYPT_OPERATION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for NCRYPT_PCP_HMAC_AUTH_SIGNATURE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_PCP_HMAC_AUTH_SIGNATURE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.iExpiration == other.iExpiration && self.pabNonce == other.pabNonce && self.pabPolicyRef == other.pabPolicyRef && self.pabHMAC == other.pabHMAC
    }
}
impl ::core::cmp::Eq for NCRYPT_PCP_HMAC_AUTH_SIGNATURE_INFO {}
impl ::core::fmt::Debug for NCRYPT_PCP_HMAC_AUTH_SIGNATURE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_PCP_HMAC_AUTH_SIGNATURE_INFO").field("dwVersion", &self.dwVersion).field("iExpiration", &self.iExpiration).field("pabNonce", &self.pabNonce).field("pabPolicyRef", &self.pabPolicyRef).field("pabHMAC", &self.pabHMAC).finish()
    }
}
impl ::core::default::Default for NCRYPT_PCP_RAW_POLICYDIGEST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_PCP_RAW_POLICYDIGEST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbDigest == other.cbDigest
    }
}
impl ::core::cmp::Eq for NCRYPT_PCP_RAW_POLICYDIGEST_INFO {}
impl ::core::fmt::Debug for NCRYPT_PCP_RAW_POLICYDIGEST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_PCP_RAW_POLICYDIGEST_INFO").field("dwVersion", &self.dwVersion).field("cbDigest", &self.cbDigest).finish()
    }
}
impl ::core::default::Default for NCRYPT_PCP_TPM_FW_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_PCP_TPM_FW_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.major1 == other.major1 && self.major2 == other.major2 && self.minor1 == other.minor1 && self.minor2 == other.minor2
    }
}
impl ::core::cmp::Eq for NCRYPT_PCP_TPM_FW_VERSION_INFO {}
impl ::core::fmt::Debug for NCRYPT_PCP_TPM_FW_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_PCP_TPM_FW_VERSION_INFO").field("major1", &self.major1).field("major2", &self.major2).field("minor1", &self.minor1).field("minor2", &self.minor2).finish()
    }
}
impl ::core::default::Default for NCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Magic == other.Magic && self.Version == other.Version && self.HeaderSize == other.HeaderSize && self.cbCertifyInfo == other.cbCertifyInfo && self.cbSignature == other.cbSignature && self.cbTpmPublic == other.cbTpmPublic
    }
}
impl ::core::cmp::Eq for NCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT {}
impl ::core::fmt::Debug for NCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT").field("Magic", &self.Magic).field("Version", &self.Version).field("HeaderSize", &self.HeaderSize).field("cbCertifyInfo", &self.cbCertifyInfo).field("cbSignature", &self.cbSignature).field("cbTpmPublic", &self.cbTpmPublic).finish()
    }
}
impl ::core::default::Default for NCRYPT_PLATFORM_ATTEST_PADDING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_PLATFORM_ATTEST_PADDING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.magic == other.magic && self.pcrMask == other.pcrMask
    }
}
impl ::core::cmp::Eq for NCRYPT_PLATFORM_ATTEST_PADDING_INFO {}
impl ::core::fmt::Debug for NCRYPT_PLATFORM_ATTEST_PADDING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_PLATFORM_ATTEST_PADDING_INFO").field("magic", &self.magic).field("pcrMask", &self.pcrMask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NCRYPT_PROTECT_STREAM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NCRYPT_PROTECT_STREAM_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NCRYPT_SUPPORTED_LENGTHS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_SUPPORTED_LENGTHS {
    fn eq(&self, other: &Self) -> bool {
        self.dwMinLength == other.dwMinLength && self.dwMaxLength == other.dwMaxLength && self.dwIncrement == other.dwIncrement && self.dwDefaultLength == other.dwDefaultLength
    }
}
impl ::core::cmp::Eq for NCRYPT_SUPPORTED_LENGTHS {}
impl ::core::fmt::Debug for NCRYPT_SUPPORTED_LENGTHS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_SUPPORTED_LENGTHS").field("dwMinLength", &self.dwMinLength).field("dwMaxLength", &self.dwMaxLength).field("dwIncrement", &self.dwIncrement).field("dwDefaultLength", &self.dwDefaultLength).finish()
    }
}
impl ::core::default::Default for NCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.magic == other.magic && self.cbHeader == other.cbHeader && self.cbPublic == other.cbPublic && self.cbPrivate == other.cbPrivate && self.cbName == other.cbName
    }
}
impl ::core::cmp::Eq for NCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER {}
impl ::core::fmt::Debug for NCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER").field("magic", &self.magic).field("cbHeader", &self.cbHeader).field("cbPublic", &self.cbPublic).field("cbPrivate", &self.cbPrivate).field("cbName", &self.cbName).finish()
    }
}
impl ::core::default::Default for NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Magic == other.Magic && self.Version == other.Version && self.pcrAlg == other.pcrAlg && self.cbSignature == other.cbSignature && self.cbQuote == other.cbQuote && self.cbPcrs == other.cbPcrs
    }
}
impl ::core::cmp::Eq for NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT {}
impl ::core::fmt::Debug for NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT").field("Magic", &self.Magic).field("Version", &self.Version).field("pcrAlg", &self.pcrAlg).field("cbSignature", &self.cbSignature).field("cbQuote", &self.cbQuote).field("cbPcrs", &self.cbPcrs).finish()
    }
}
impl ::core::default::Default for NCRYPT_UI_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_UI_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwFlags == other.dwFlags && self.pszCreationTitle == other.pszCreationTitle && self.pszFriendlyName == other.pszFriendlyName && self.pszDescription == other.pszDescription
    }
}
impl ::core::cmp::Eq for NCRYPT_UI_POLICY {}
impl ::core::fmt::Debug for NCRYPT_UI_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_UI_POLICY").field("dwVersion", &self.dwVersion).field("dwFlags", &self.dwFlags).field("pszCreationTitle", &self.pszCreationTitle).field("pszFriendlyName", &self.pszFriendlyName).field("pszDescription", &self.pszDescription).finish()
    }
}
impl ::core::default::Default for NCRYPT_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.TrustletId == other.TrustletId && self.MinSvn == other.MinSvn && self.FlagsMask == other.FlagsMask && self.FlagsExpected == other.FlagsExpected && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NCRYPT_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS {}
impl ::core::fmt::Debug for NCRYPT_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS").field("Version", &self.Version).field("TrustletId", &self.TrustletId).field("MinSvn", &self.MinSvn).field("FlagsMask", &self.FlagsMask).field("FlagsExpected", &self.FlagsExpected).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NCRYPT_VSM_KEY_ATTESTATION_STATEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCRYPT_VSM_KEY_ATTESTATION_STATEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Magic == other.Magic && self.Version == other.Version && self.cbSignature == other.cbSignature && self.cbReport == other.cbReport && self.cbAttributes == other.cbAttributes
    }
}
impl ::core::cmp::Eq for NCRYPT_VSM_KEY_ATTESTATION_STATEMENT {}
impl ::core::fmt::Debug for NCRYPT_VSM_KEY_ATTESTATION_STATEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCRYPT_VSM_KEY_ATTESTATION_STATEMENT").field("Magic", &self.Magic).field("Version", &self.Version).field("cbSignature", &self.cbSignature).field("cbReport", &self.cbReport).field("cbAttributes", &self.cbAttributes).finish()
    }
}
impl ::core::default::Default for NCryptAlgorithmName {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCryptAlgorithmName {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.dwClass == other.dwClass && self.dwAlgOperations == other.dwAlgOperations && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for NCryptAlgorithmName {}
impl ::core::fmt::Debug for NCryptAlgorithmName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCryptAlgorithmName").field("pszName", &self.pszName).field("dwClass", &self.dwClass).field("dwAlgOperations", &self.dwAlgOperations).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for NCryptKeyName {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCryptKeyName {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.pszAlgid == other.pszAlgid && self.dwLegacyKeySpec == other.dwLegacyKeySpec && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for NCryptKeyName {}
impl ::core::fmt::Debug for NCryptKeyName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCryptKeyName").field("pszName", &self.pszName).field("pszAlgid", &self.pszAlgid).field("dwLegacyKeySpec", &self.dwLegacyKeySpec).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for NCryptProviderName {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NCryptProviderName {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.pszComment == other.pszComment
    }
}
impl ::core::cmp::Eq for NCryptProviderName {}
impl ::core::fmt::Debug for NCryptProviderName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCryptProviderName").field("pszName", &self.pszName).field("pszComment", &self.pszComment).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OCSP_BASIC_RESPONSE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OCSP_BASIC_RESPONSE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OCSP_BASIC_REVOKED_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OCSP_BASIC_REVOKED_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.RevocationDate == other.RevocationDate && self.dwCrlReasonCode == other.dwCrlReasonCode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OCSP_BASIC_REVOKED_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OCSP_BASIC_REVOKED_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OCSP_BASIC_REVOKED_INFO").field("RevocationDate", &self.RevocationDate).field("dwCrlReasonCode", &self.dwCrlReasonCode).finish()
    }
}
impl ::core::default::Default for OCSP_BASIC_SIGNED_RESPONSE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OCSP_BASIC_SIGNED_RESPONSE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ToBeSigned == other.ToBeSigned && self.SignatureInfo == other.SignatureInfo
    }
}
impl ::core::cmp::Eq for OCSP_BASIC_SIGNED_RESPONSE_INFO {}
impl ::core::fmt::Debug for OCSP_BASIC_SIGNED_RESPONSE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OCSP_BASIC_SIGNED_RESPONSE_INFO").field("ToBeSigned", &self.ToBeSigned).field("SignatureInfo", &self.SignatureInfo).finish()
    }
}
impl ::core::default::Default for OCSP_CERT_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OCSP_CERT_ID {
    fn eq(&self, other: &Self) -> bool {
        self.HashAlgorithm == other.HashAlgorithm && self.IssuerNameHash == other.IssuerNameHash && self.IssuerKeyHash == other.IssuerKeyHash && self.SerialNumber == other.SerialNumber
    }
}
impl ::core::cmp::Eq for OCSP_CERT_ID {}
impl ::core::fmt::Debug for OCSP_CERT_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OCSP_CERT_ID").field("HashAlgorithm", &self.HashAlgorithm).field("IssuerNameHash", &self.IssuerNameHash).field("IssuerKeyHash", &self.IssuerKeyHash).field("SerialNumber", &self.SerialNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OCSP_REQUEST_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OCSP_REQUEST_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.CertId == other.CertId && self.cExtension == other.cExtension && self.rgExtension == other.rgExtension
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OCSP_REQUEST_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OCSP_REQUEST_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OCSP_REQUEST_ENTRY").field("CertId", &self.CertId).field("cExtension", &self.cExtension).field("rgExtension", &self.rgExtension).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OCSP_REQUEST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OCSP_REQUEST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.pRequestorName == other.pRequestorName && self.cRequestEntry == other.cRequestEntry && self.rgRequestEntry == other.rgRequestEntry && self.cExtension == other.cExtension && self.rgExtension == other.rgExtension
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OCSP_REQUEST_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OCSP_REQUEST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OCSP_REQUEST_INFO").field("dwVersion", &self.dwVersion).field("pRequestorName", &self.pRequestorName).field("cRequestEntry", &self.cRequestEntry).field("rgRequestEntry", &self.rgRequestEntry).field("cExtension", &self.cExtension).field("rgExtension", &self.rgExtension).finish()
    }
}
impl ::core::default::Default for OCSP_RESPONSE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OCSP_RESPONSE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwStatus == other.dwStatus && self.pszObjId == other.pszObjId && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for OCSP_RESPONSE_INFO {}
impl ::core::fmt::Debug for OCSP_RESPONSE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OCSP_RESPONSE_INFO").field("dwStatus", &self.dwStatus).field("pszObjId", &self.pszObjId).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for OCSP_SIGNATURE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OCSP_SIGNATURE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SignatureAlgorithm == other.SignatureAlgorithm && self.Signature == other.Signature && self.cCertEncoded == other.cCertEncoded && self.rgCertEncoded == other.rgCertEncoded
    }
}
impl ::core::cmp::Eq for OCSP_SIGNATURE_INFO {}
impl ::core::fmt::Debug for OCSP_SIGNATURE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OCSP_SIGNATURE_INFO").field("SignatureAlgorithm", &self.SignatureAlgorithm).field("Signature", &self.Signature).field("cCertEncoded", &self.cCertEncoded).field("rgCertEncoded", &self.rgCertEncoded).finish()
    }
}
impl ::core::default::Default for OCSP_SIGNED_REQUEST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OCSP_SIGNED_REQUEST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ToBeSigned == other.ToBeSigned && self.pOptionalSignatureInfo == other.pOptionalSignatureInfo
    }
}
impl ::core::cmp::Eq for OCSP_SIGNED_REQUEST_INFO {}
impl ::core::fmt::Debug for OCSP_SIGNED_REQUEST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OCSP_SIGNED_REQUEST_INFO").field("ToBeSigned", &self.ToBeSigned).field("pOptionalSignatureInfo", &self.pOptionalSignatureInfo).finish()
    }
}
impl ::core::default::Default for PKCS12_PBES2_EXPORT_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PKCS12_PBES2_EXPORT_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hNcryptDescriptor == other.hNcryptDescriptor && self.pwszPbes2Alg == other.pwszPbes2Alg
    }
}
impl ::core::cmp::Eq for PKCS12_PBES2_EXPORT_PARAMS {}
impl ::core::fmt::Debug for PKCS12_PBES2_EXPORT_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PKCS12_PBES2_EXPORT_PARAMS").field("dwSize", &self.dwSize).field("hNcryptDescriptor", &self.hNcryptDescriptor).field("pwszPbes2Alg", &self.pwszPbes2Alg).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLICY_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLICY_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.targetEndpointAddress == other.targetEndpointAddress && self.issuerEndpointAddress == other.issuerEndpointAddress && self.issuedTokenParameters == other.issuedTokenParameters && self.privacyNoticeLink == other.privacyNoticeLink && self.privacyNoticeVersion == other.privacyNoticeVersion && self.useManagedPresentation == other.useManagedPresentation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLICY_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLICY_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_ELEMENT").field("targetEndpointAddress", &self.targetEndpointAddress).field("issuerEndpointAddress", &self.issuerEndpointAddress).field("issuedTokenParameters", &self.issuedTokenParameters).field("privacyNoticeLink", &self.privacyNoticeLink).field("privacyNoticeVersion", &self.privacyNoticeVersion).field("useManagedPresentation", &self.useManagedPresentation).finish()
    }
}
impl ::core::default::Default for PRIVKEYVER3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRIVKEYVER3 {
    fn eq(&self, other: &Self) -> bool {
        self.magic == other.magic && self.bitlenP == other.bitlenP && self.bitlenQ == other.bitlenQ && self.bitlenJ == other.bitlenJ && self.bitlenX == other.bitlenX && self.DSSSeed == other.DSSSeed
    }
}
impl ::core::cmp::Eq for PRIVKEYVER3 {}
impl ::core::fmt::Debug for PRIVKEYVER3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRIVKEYVER3").field("magic", &self.magic).field("bitlenP", &self.bitlenP).field("bitlenQ", &self.bitlenQ).field("bitlenJ", &self.bitlenJ).field("bitlenX", &self.bitlenX).field("DSSSeed", &self.DSSSeed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROV_ENUMALGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROV_ENUMALGS {
    fn eq(&self, other: &Self) -> bool {
        self.aiAlgid == other.aiAlgid && self.dwBitLen == other.dwBitLen && self.dwNameLen == other.dwNameLen && self.szName == other.szName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROV_ENUMALGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PROV_ENUMALGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROV_ENUMALGS").field("aiAlgid", &self.aiAlgid).field("dwBitLen", &self.dwBitLen).field("dwNameLen", &self.dwNameLen).field("szName", &self.szName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROV_ENUMALGS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROV_ENUMALGS_EX {
    fn eq(&self, other: &Self) -> bool {
        self.aiAlgid == other.aiAlgid && self.dwDefaultLen == other.dwDefaultLen && self.dwMinLen == other.dwMinLen && self.dwMaxLen == other.dwMaxLen && self.dwProtocols == other.dwProtocols && self.dwNameLen == other.dwNameLen && self.szName == other.szName && self.dwLongNameLen == other.dwLongNameLen && self.szLongName == other.szLongName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROV_ENUMALGS_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PROV_ENUMALGS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROV_ENUMALGS_EX").field("aiAlgid", &self.aiAlgid).field("dwDefaultLen", &self.dwDefaultLen).field("dwMinLen", &self.dwMinLen).field("dwMaxLen", &self.dwMaxLen).field("dwProtocols", &self.dwProtocols).field("dwNameLen", &self.dwNameLen).field("szName", &self.szName).field("dwLongNameLen", &self.dwLongNameLen).field("szLongName", &self.szLongName).finish()
    }
}
impl ::core::default::Default for PUBKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PUBKEY {
    fn eq(&self, other: &Self) -> bool {
        self.magic == other.magic && self.bitlen == other.bitlen
    }
}
impl ::core::cmp::Eq for PUBKEY {}
impl ::core::fmt::Debug for PUBKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PUBKEY").field("magic", &self.magic).field("bitlen", &self.bitlen).finish()
    }
}
impl ::core::default::Default for PUBKEYVER3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PUBKEYVER3 {
    fn eq(&self, other: &Self) -> bool {
        self.magic == other.magic && self.bitlenP == other.bitlenP && self.bitlenQ == other.bitlenQ && self.bitlenJ == other.bitlenJ && self.DSSSeed == other.DSSSeed
    }
}
impl ::core::cmp::Eq for PUBKEYVER3 {}
impl ::core::fmt::Debug for PUBKEYVER3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PUBKEYVER3").field("magic", &self.magic).field("bitlenP", &self.bitlenP).field("bitlenQ", &self.bitlenQ).field("bitlenJ", &self.bitlenJ).field("DSSSeed", &self.DSSSeed).finish()
    }
}
impl ::core::default::Default for PUBLICKEYSTRUC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PUBLICKEYSTRUC {
    fn eq(&self, other: &Self) -> bool {
        self.bType == other.bType && self.bVersion == other.bVersion && self.reserved == other.reserved && self.aiKeyAlg == other.aiKeyAlg
    }
}
impl ::core::cmp::Eq for PUBLICKEYSTRUC {}
impl ::core::fmt::Debug for PUBLICKEYSTRUC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PUBLICKEYSTRUC").field("bType", &self.bType).field("bVersion", &self.bVersion).field("reserved", &self.reserved).field("aiKeyAlg", &self.aiKeyAlg).finish()
    }
}
impl ::core::default::Default for PaddingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PaddingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaddingMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for RECIPIENTPOLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RECIPIENTPOLICY {
    fn eq(&self, other: &Self) -> bool {
        self.recipient == other.recipient && self.issuer == other.issuer && self.tokenType == other.tokenType && self.requiredClaims == other.requiredClaims && self.optionalClaims == other.optionalClaims && self.privacyUrl == other.privacyUrl && self.privacyVersion == other.privacyVersion
    }
}
impl ::core::cmp::Eq for RECIPIENTPOLICY {}
impl ::core::fmt::Debug for RECIPIENTPOLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECIPIENTPOLICY").field("recipient", &self.recipient).field("issuer", &self.issuer).field("tokenType", &self.tokenType).field("requiredClaims", &self.requiredClaims).field("optionalClaims", &self.optionalClaims).field("privacyUrl", &self.privacyUrl).field("privacyVersion", &self.privacyVersion).finish()
    }
}
impl ::core::default::Default for RECIPIENTPOLICY2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RECIPIENTPOLICY2 {
    fn eq(&self, other: &Self) -> bool {
        self.recipient == other.recipient && self.issuer == other.issuer && self.tokenType == other.tokenType && self.requiredClaims == other.requiredClaims && self.optionalClaims == other.optionalClaims && self.privacyUrl == other.privacyUrl && self.privacyVersion == other.privacyVersion
    }
}
impl ::core::cmp::Eq for RECIPIENTPOLICY2 {}
impl ::core::fmt::Debug for RECIPIENTPOLICY2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECIPIENTPOLICY2").field("recipient", &self.recipient).field("issuer", &self.issuer).field("tokenType", &self.tokenType).field("requiredClaims", &self.requiredClaims).field("optionalClaims", &self.optionalClaims).field("privacyUrl", &self.privacyUrl).field("privacyVersion", &self.privacyVersion).finish()
    }
}
impl ::core::default::Default for ROOT_INFO_LUID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ROOT_INFO_LUID {
    fn eq(&self, other: &Self) -> bool {
        self.LowPart == other.LowPart && self.HighPart == other.HighPart
    }
}
impl ::core::cmp::Eq for ROOT_INFO_LUID {}
impl ::core::fmt::Debug for ROOT_INFO_LUID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ROOT_INFO_LUID").field("LowPart", &self.LowPart).field("HighPart", &self.HighPart).finish()
    }
}
impl ::core::default::Default for RSAPUBKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RSAPUBKEY {
    fn eq(&self, other: &Self) -> bool {
        self.magic == other.magic && self.bitlen == other.bitlen && self.pubexp == other.pubexp
    }
}
impl ::core::cmp::Eq for RSAPUBKEY {}
impl ::core::fmt::Debug for RSAPUBKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSAPUBKEY").field("magic", &self.magic).field("bitlen", &self.bitlen).field("pubexp", &self.pubexp).finish()
    }
}
impl ::core::default::Default for SCHANNEL_ALG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCHANNEL_ALG {
    fn eq(&self, other: &Self) -> bool {
        self.dwUse == other.dwUse && self.Algid == other.Algid && self.cBits == other.cBits && self.dwFlags == other.dwFlags && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for SCHANNEL_ALG {}
impl ::core::fmt::Debug for SCHANNEL_ALG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCHANNEL_ALG").field("dwUse", &self.dwUse).field("Algid", &self.Algid).field("cBits", &self.cBits).field("dwFlags", &self.dwFlags).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::default::Default for SSL_ECCKEY_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SSL_ECCKEY_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwCurveType == other.dwCurveType && self.cbKey == other.cbKey
    }
}
impl ::core::cmp::Eq for SSL_ECCKEY_BLOB {}
impl ::core::fmt::Debug for SSL_ECCKEY_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSL_ECCKEY_BLOB").field("dwCurveType", &self.dwCurveType).field("cbKey", &self.cbKey).finish()
    }
}
impl ::core::default::Default for SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwErrorLevel == other.dwErrorLevel && self.dwErrorCategory == other.dwErrorCategory && self.dwReserved == other.dwReserved && self.wszErrorText == other.wszErrorText
    }
}
impl ::core::cmp::Eq for SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS {}
impl ::core::fmt::Debug for SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS").field("cbSize", &self.cbSize).field("dwErrorLevel", &self.dwErrorLevel).field("dwErrorCategory", &self.dwErrorCategory).field("dwReserved", &self.dwReserved).field("wszErrorText", &self.wszErrorText).finish()
    }
}
impl ::core::default::Default for SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwReserved == other.dwReserved && self.pwszServerName == other.pwszServerName && self.rgpszHpkpValue == other.rgpszHpkpValue
    }
}
impl ::core::cmp::Eq for SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA {}
impl ::core::fmt::Debug for SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA").field("cbSize", &self.cbSize).field("dwReserved", &self.dwReserved).field("pwszServerName", &self.pwszServerName).field("rgpszHpkpValue", &self.rgpszHpkpValue).finish()
    }
}
impl ::core::default::Default for SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwReserved == other.dwReserved && self.pwszServerName == other.pwszServerName
    }
}
impl ::core::cmp::Eq for SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA {}
impl ::core::fmt::Debug for SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA").field("cbSize", &self.cbSize).field("dwReserved", &self.dwReserved).field("pwszServerName", &self.pwszServerName).finish()
    }
}
impl ::core::default::Default for SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.lError == other.lError && self.wszErrorText == other.wszErrorText
    }
}
impl ::core::cmp::Eq for SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS {}
impl ::core::fmt::Debug for SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS").field("cbSize", &self.cbSize).field("lError", &self.lError).field("wszErrorText", &self.wszErrorText).finish()
    }
}
