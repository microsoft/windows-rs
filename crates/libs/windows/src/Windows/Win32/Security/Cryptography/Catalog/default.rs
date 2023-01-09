impl ::core::default::Default for CATALOG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CATALOG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.wszCatalogFile == other.wszCatalogFile
    }
}
impl ::core::cmp::Eq for CATALOG_INFO {}
impl ::core::fmt::Debug for CATALOG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CATALOG_INFO").field("cbStruct", &self.cbStruct).field("wszCatalogFile", &self.wszCatalogFile).finish()
    }
}
impl ::core::default::Default for CRYPTCATATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPTCATATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pwszReferenceTag == other.pwszReferenceTag && self.dwAttrTypeAndAction == other.dwAttrTypeAndAction && self.cbValue == other.cbValue && self.pbValue == other.pbValue && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for CRYPTCATATTRIBUTE {}
impl ::core::fmt::Debug for CRYPTCATATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTCATATTRIBUTE").field("cbStruct", &self.cbStruct).field("pwszReferenceTag", &self.pwszReferenceTag).field("dwAttrTypeAndAction", &self.dwAttrTypeAndAction).field("cbValue", &self.cbValue).field("pbValue", &self.pbValue).field("dwReserved", &self.dwReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTCATCDF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTCATCDF {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.hFile == other.hFile && self.dwCurFilePos == other.dwCurFilePos && self.dwLastMemberOffset == other.dwLastMemberOffset && self.fEOF == other.fEOF && self.pwszResultDir == other.pwszResultDir && self.hCATStore == other.hCATStore
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTCATCDF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPTCATCDF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTCATCDF").field("cbStruct", &self.cbStruct).field("hFile", &self.hFile).field("dwCurFilePos", &self.dwCurFilePos).field("dwLastMemberOffset", &self.dwLastMemberOffset).field("fEOF", &self.fEOF).field("pwszResultDir", &self.pwszResultDir).field("hCATStore", &self.hCATStore).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::default::Default for CRYPTCATMEMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::PartialEq for CRYPTCATMEMBER {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pwszReferenceTag == other.pwszReferenceTag && self.pwszFileName == other.pwszFileName && self.gSubjectType == other.gSubjectType && self.fdwMemberFlags == other.fdwMemberFlags && self.pIndirectData == other.pIndirectData && self.dwCertVersion == other.dwCertVersion && self.dwReserved == other.dwReserved && self.hReserved == other.hReserved && self.sEncodedIndirectData == other.sEncodedIndirectData && self.sEncodedMemberInfo == other.sEncodedMemberInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::Eq for CRYPTCATMEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::fmt::Debug for CRYPTCATMEMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTCATMEMBER")
            .field("cbStruct", &self.cbStruct)
            .field("pwszReferenceTag", &self.pwszReferenceTag)
            .field("pwszFileName", &self.pwszFileName)
            .field("gSubjectType", &self.gSubjectType)
            .field("fdwMemberFlags", &self.fdwMemberFlags)
            .field("pIndirectData", &self.pIndirectData)
            .field("dwCertVersion", &self.dwCertVersion)
            .field("dwReserved", &self.dwReserved)
            .field("hReserved", &self.hReserved)
            .field("sEncodedIndirectData", &self.sEncodedIndirectData)
            .field("sEncodedMemberInfo", &self.sEncodedMemberInfo)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTCATSTORE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTCATSTORE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwPublicVersion == other.dwPublicVersion && self.pwszP7File == other.pwszP7File && self.hProv == other.hProv && self.dwEncodingType == other.dwEncodingType && self.fdwStoreFlags == other.fdwStoreFlags && self.hReserved == other.hReserved && self.hAttrs == other.hAttrs && self.hCryptMsg == other.hCryptMsg && self.hSorted == other.hSorted
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTCATSTORE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPTCATSTORE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTCATSTORE").field("cbStruct", &self.cbStruct).field("dwPublicVersion", &self.dwPublicVersion).field("pwszP7File", &self.pwszP7File).field("hProv", &self.hProv).field("dwEncodingType", &self.dwEncodingType).field("fdwStoreFlags", &self.fdwStoreFlags).field("hReserved", &self.hReserved).field("hAttrs", &self.hAttrs).field("hCryptMsg", &self.hCryptMsg).field("hSorted", &self.hSorted).finish()
    }
}
impl ::core::default::Default for CRYPTCAT_OPEN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPTCAT_OPEN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTCAT_OPEN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CRYPTCAT_OPEN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRYPTCAT_OPEN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRYPTCAT_OPEN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRYPTCAT_OPEN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRYPTCAT_OPEN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CRYPTCAT_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPTCAT_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTCAT_VERSION").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::default::Default for MS_ADDINFO_CATALOGMEMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::PartialEq for MS_ADDINFO_CATALOGMEMBER {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pStore == other.pStore && self.pMember == other.pMember
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::Eq for MS_ADDINFO_CATALOGMEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::fmt::Debug for MS_ADDINFO_CATALOGMEMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MS_ADDINFO_CATALOGMEMBER").field("cbStruct", &self.cbStruct).field("pStore", &self.pStore).field("pMember", &self.pMember).finish()
    }
}
