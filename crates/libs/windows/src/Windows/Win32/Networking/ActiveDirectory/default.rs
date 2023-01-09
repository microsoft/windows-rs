impl ::core::default::Default for ADSI_DIALECT_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADSI_DIALECT_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADSI_DIALECT_ENUM").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADSPROPERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADSPROPERROR {
    fn eq(&self, other: &Self) -> bool {
        self.hwndPage == other.hwndPage && self.pszPageTitle == other.pszPageTitle && self.pszObjPath == other.pszObjPath && self.pszObjClass == other.pszObjClass && self.hr == other.hr && self.pszError == other.pszError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADSPROPERROR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADSPROPERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADSPROPERROR").field("hwndPage", &self.hwndPage).field("pszPageTitle", &self.pszPageTitle).field("pszObjPath", &self.pszObjPath).field("pszObjClass", &self.pszObjClass).field("hr", &self.hr).field("pszError", &self.pszError).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADSPROPINITPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADSPROPINITPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.hr == other.hr && self.pDsObj == other.pDsObj && self.pwzCN == other.pwzCN && self.pWritableAttrs == other.pWritableAttrs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADSPROPINITPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADSPROPINITPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADSPROPINITPARAMS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("hr", &self.hr).field("pDsObj", &self.pDsObj).field("pwzCN", &self.pwzCN).field("pWritableAttrs", &self.pWritableAttrs).finish()
    }
}
impl ::core::default::Default for ADSTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADSTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADSTYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADSVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ADS_ACEFLAG_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_ACEFLAG_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_ACEFLAG_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_ACETYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_ACETYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_ACETYPE_ENUM").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADS_ATTR_DEF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADS_ATTR_DEF {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttrName == other.pszAttrName && self.dwADsType == other.dwADsType && self.dwMinRange == other.dwMinRange && self.dwMaxRange == other.dwMaxRange && self.fMultiValued == other.fMultiValued
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADS_ATTR_DEF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADS_ATTR_DEF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_ATTR_DEF").field("pszAttrName", &self.pszAttrName).field("dwADsType", &self.dwADsType).field("dwMinRange", &self.dwMinRange).field("dwMaxRange", &self.dwMaxRange).field("fMultiValued", &self.fMultiValued).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADS_ATTR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADS_ATTR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttrName == other.pszAttrName && self.dwControlCode == other.dwControlCode && self.dwADsType == other.dwADsType && self.pADsValues == other.pADsValues && self.dwNumValues == other.dwNumValues
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADS_ATTR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADS_ATTR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_ATTR_INFO").field("pszAttrName", &self.pszAttrName).field("dwControlCode", &self.dwControlCode).field("dwADsType", &self.dwADsType).field("pADsValues", &self.pADsValues).field("dwNumValues", &self.dwNumValues).finish()
    }
}
impl ::core::default::Default for ADS_AUTHENTICATION_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_AUTHENTICATION_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_AUTHENTICATION_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_BACKLINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_BACKLINK {
    fn eq(&self, other: &Self) -> bool {
        self.RemoteID == other.RemoteID && self.ObjectName == other.ObjectName
    }
}
impl ::core::cmp::Eq for ADS_BACKLINK {}
impl ::core::fmt::Debug for ADS_BACKLINK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_BACKLINK").field("RemoteID", &self.RemoteID).field("ObjectName", &self.ObjectName).finish()
    }
}
impl ::core::default::Default for ADS_CASEIGNORE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_CASEIGNORE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.String == other.String
    }
}
impl ::core::cmp::Eq for ADS_CASEIGNORE_LIST {}
impl ::core::fmt::Debug for ADS_CASEIGNORE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_CASEIGNORE_LIST").field("Next", &self.Next).field("String", &self.String).finish()
    }
}
impl ::core::default::Default for ADS_CHASE_REFERRALS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_CHASE_REFERRALS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_CHASE_REFERRALS_ENUM").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADS_CLASS_DEF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADS_CLASS_DEF {
    fn eq(&self, other: &Self) -> bool {
        self.pszClassName == other.pszClassName && self.dwMandatoryAttrs == other.dwMandatoryAttrs && self.ppszMandatoryAttrs == other.ppszMandatoryAttrs && self.optionalAttrs == other.optionalAttrs && self.ppszOptionalAttrs == other.ppszOptionalAttrs && self.dwNamingAttrs == other.dwNamingAttrs && self.ppszNamingAttrs == other.ppszNamingAttrs && self.dwSuperClasses == other.dwSuperClasses && self.ppszSuperClasses == other.ppszSuperClasses && self.fIsContainer == other.fIsContainer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADS_CLASS_DEF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADS_CLASS_DEF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_CLASS_DEF")
            .field("pszClassName", &self.pszClassName)
            .field("dwMandatoryAttrs", &self.dwMandatoryAttrs)
            .field("ppszMandatoryAttrs", &self.ppszMandatoryAttrs)
            .field("optionalAttrs", &self.optionalAttrs)
            .field("ppszOptionalAttrs", &self.ppszOptionalAttrs)
            .field("dwNamingAttrs", &self.dwNamingAttrs)
            .field("ppszNamingAttrs", &self.ppszNamingAttrs)
            .field("dwSuperClasses", &self.dwSuperClasses)
            .field("ppszSuperClasses", &self.ppszSuperClasses)
            .field("fIsContainer", &self.fIsContainer)
            .finish()
    }
}
impl ::core::default::Default for ADS_DEREFENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_DEREFENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_DEREFENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_DISPLAY_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_DISPLAY_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_DISPLAY_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_DN_WITH_BINARY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_DN_WITH_BINARY {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.lpBinaryValue == other.lpBinaryValue && self.pszDNString == other.pszDNString
    }
}
impl ::core::cmp::Eq for ADS_DN_WITH_BINARY {}
impl ::core::fmt::Debug for ADS_DN_WITH_BINARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_DN_WITH_BINARY").field("dwLength", &self.dwLength).field("lpBinaryValue", &self.lpBinaryValue).field("pszDNString", &self.pszDNString).finish()
    }
}
impl ::core::default::Default for ADS_DN_WITH_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_DN_WITH_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.pszStringValue == other.pszStringValue && self.pszDNString == other.pszDNString
    }
}
impl ::core::cmp::Eq for ADS_DN_WITH_STRING {}
impl ::core::fmt::Debug for ADS_DN_WITH_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_DN_WITH_STRING").field("pszStringValue", &self.pszStringValue).field("pszDNString", &self.pszDNString).finish()
    }
}
impl ::core::default::Default for ADS_EMAIL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_EMAIL {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for ADS_EMAIL {}
impl ::core::fmt::Debug for ADS_EMAIL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_EMAIL").field("Address", &self.Address).field("Type", &self.Type).finish()
    }
}
impl ::core::default::Default for ADS_ESCAPE_MODE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_ESCAPE_MODE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_ESCAPE_MODE_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_FAXNUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_FAXNUMBER {
    fn eq(&self, other: &Self) -> bool {
        self.TelephoneNumber == other.TelephoneNumber && self.NumberOfBits == other.NumberOfBits && self.Parameters == other.Parameters
    }
}
impl ::core::cmp::Eq for ADS_FAXNUMBER {}
impl ::core::fmt::Debug for ADS_FAXNUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_FAXNUMBER").field("TelephoneNumber", &self.TelephoneNumber).field("NumberOfBits", &self.NumberOfBits).field("Parameters", &self.Parameters).finish()
    }
}
impl ::core::default::Default for ADS_FLAGTYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_FLAGTYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_FLAGTYPE_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_FORMAT_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_FORMAT_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_FORMAT_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_GROUP_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_GROUP_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_GROUP_TYPE_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_HOLD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_HOLD {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectName == other.ObjectName && self.Amount == other.Amount
    }
}
impl ::core::cmp::Eq for ADS_HOLD {}
impl ::core::fmt::Debug for ADS_HOLD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_HOLD").field("ObjectName", &self.ObjectName).field("Amount", &self.Amount).finish()
    }
}
impl ::core::default::Default for ADS_NAME_INITTYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_NAME_INITTYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_NAME_INITTYPE_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_NAME_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_NAME_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_NAME_TYPE_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_NETADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_NETADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressType == other.AddressType && self.AddressLength == other.AddressLength && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for ADS_NETADDRESS {}
impl ::core::fmt::Debug for ADS_NETADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_NETADDRESS").field("AddressType", &self.AddressType).field("AddressLength", &self.AddressLength).field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for ADS_NT_SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_NT_SECURITY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.lpValue == other.lpValue
    }
}
impl ::core::cmp::Eq for ADS_NT_SECURITY_DESCRIPTOR {}
impl ::core::fmt::Debug for ADS_NT_SECURITY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_NT_SECURITY_DESCRIPTOR").field("dwLength", &self.dwLength).field("lpValue", &self.lpValue).finish()
    }
}
impl ::core::default::Default for ADS_OBJECT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_OBJECT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszRDN == other.pszRDN && self.pszObjectDN == other.pszObjectDN && self.pszParentDN == other.pszParentDN && self.pszSchemaDN == other.pszSchemaDN && self.pszClassName == other.pszClassName
    }
}
impl ::core::cmp::Eq for ADS_OBJECT_INFO {}
impl ::core::fmt::Debug for ADS_OBJECT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_OBJECT_INFO").field("pszRDN", &self.pszRDN).field("pszObjectDN", &self.pszObjectDN).field("pszParentDN", &self.pszParentDN).field("pszSchemaDN", &self.pszSchemaDN).field("pszClassName", &self.pszClassName).finish()
    }
}
impl ::core::default::Default for ADS_OCTET_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_OCTET_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Length == other.Length && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for ADS_OCTET_LIST {}
impl ::core::fmt::Debug for ADS_OCTET_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_OCTET_LIST").field("Next", &self.Next).field("Length", &self.Length).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for ADS_OCTET_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_OCTET_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.lpValue == other.lpValue
    }
}
impl ::core::cmp::Eq for ADS_OCTET_STRING {}
impl ::core::fmt::Debug for ADS_OCTET_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_OCTET_STRING").field("dwLength", &self.dwLength).field("lpValue", &self.lpValue).finish()
    }
}
impl ::core::default::Default for ADS_OPTION_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_OPTION_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_OPTION_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_PASSWORD_ENCODING_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_PASSWORD_ENCODING_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_PASSWORD_ENCODING_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_PATH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_PATH {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.VolumeName == other.VolumeName && self.Path == other.Path
    }
}
impl ::core::cmp::Eq for ADS_PATH {}
impl ::core::fmt::Debug for ADS_PATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_PATH").field("Type", &self.Type).field("VolumeName", &self.VolumeName).field("Path", &self.Path).finish()
    }
}
impl ::core::default::Default for ADS_PATHTYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_PATHTYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_PATHTYPE_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_POSTALADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_POSTALADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.PostalAddress == other.PostalAddress
    }
}
impl ::core::cmp::Eq for ADS_POSTALADDRESS {}
impl ::core::fmt::Debug for ADS_POSTALADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_POSTALADDRESS").field("PostalAddress", &self.PostalAddress).finish()
    }
}
impl ::core::default::Default for ADS_PREFERENCES_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_PREFERENCES_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_PREFERENCES_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_PROPERTY_OPERATION_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_PROPERTY_OPERATION_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_PROPERTY_OPERATION_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_PROV_SPECIFIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_PROV_SPECIFIC {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.lpValue == other.lpValue
    }
}
impl ::core::cmp::Eq for ADS_PROV_SPECIFIC {}
impl ::core::fmt::Debug for ADS_PROV_SPECIFIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_PROV_SPECIFIC").field("dwLength", &self.dwLength).field("lpValue", &self.lpValue).finish()
    }
}
impl ::core::default::Default for ADS_REPLICAPOINTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_REPLICAPOINTER {
    fn eq(&self, other: &Self) -> bool {
        self.ServerName == other.ServerName && self.ReplicaType == other.ReplicaType && self.ReplicaNumber == other.ReplicaNumber && self.Count == other.Count && self.ReplicaAddressHints == other.ReplicaAddressHints
    }
}
impl ::core::cmp::Eq for ADS_REPLICAPOINTER {}
impl ::core::fmt::Debug for ADS_REPLICAPOINTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_REPLICAPOINTER").field("ServerName", &self.ServerName).field("ReplicaType", &self.ReplicaType).field("ReplicaNumber", &self.ReplicaNumber).field("Count", &self.Count).field("ReplicaAddressHints", &self.ReplicaAddressHints).finish()
    }
}
impl ::core::default::Default for ADS_RIGHTS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_RIGHTS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_RIGHTS_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_SCOPEENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_SCOPEENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SCOPEENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_SD_CONTROL_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_SD_CONTROL_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SD_CONTROL_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_SD_FORMAT_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_SD_FORMAT_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SD_FORMAT_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_SD_REVISION_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_SD_REVISION_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SD_REVISION_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_SEARCHPREF_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_SEARCHPREF_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SEARCHPREF_ENUM").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADS_SEARCHPREF_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADS_SEARCH_COLUMN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADS_SEARCH_COLUMN {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttrName == other.pszAttrName && self.dwADsType == other.dwADsType && self.pADsValues == other.pADsValues && self.dwNumValues == other.dwNumValues && self.hReserved == other.hReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADS_SEARCH_COLUMN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADS_SEARCH_COLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_SEARCH_COLUMN").field("pszAttrName", &self.pszAttrName).field("dwADsType", &self.dwADsType).field("pADsValues", &self.pADsValues).field("dwNumValues", &self.dwNumValues).field("hReserved", &self.hReserved).finish()
    }
}
impl ::core::default::Default for ADS_SECURITY_INFO_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_SECURITY_INFO_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SECURITY_INFO_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_SETTYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_SETTYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SETTYPE_ENUM").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADS_SORTKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADS_SORTKEY {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttrType == other.pszAttrType && self.pszReserved == other.pszReserved && self.fReverseorder == other.fReverseorder
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADS_SORTKEY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADS_SORTKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_SORTKEY").field("pszAttrType", &self.pszAttrType).field("pszReserved", &self.pszReserved).field("fReverseorder", &self.fReverseorder).finish()
    }
}
impl ::core::default::Default for ADS_STATUSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_STATUSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_STATUSENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_SYSTEMFLAG_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_SYSTEMFLAG_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SYSTEMFLAG_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_TIMESTAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_TIMESTAMP {
    fn eq(&self, other: &Self) -> bool {
        self.WholeSeconds == other.WholeSeconds && self.EventID == other.EventID
    }
}
impl ::core::cmp::Eq for ADS_TIMESTAMP {}
impl ::core::fmt::Debug for ADS_TIMESTAMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_TIMESTAMP").field("WholeSeconds", &self.WholeSeconds).field("EventID", &self.EventID).finish()
    }
}
impl ::core::default::Default for ADS_TYPEDNAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_TYPEDNAME {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectName == other.ObjectName && self.Level == other.Level && self.Interval == other.Interval
    }
}
impl ::core::cmp::Eq for ADS_TYPEDNAME {}
impl ::core::fmt::Debug for ADS_TYPEDNAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_TYPEDNAME").field("ObjectName", &self.ObjectName).field("Level", &self.Level).field("Interval", &self.Interval).finish()
    }
}
impl ::core::default::Default for ADS_USER_FLAG_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADS_USER_FLAG_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_USER_FLAG_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADS_VLV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADS_VLV {
    fn eq(&self, other: &Self) -> bool {
        self.dwBeforeCount == other.dwBeforeCount && self.dwAfterCount == other.dwAfterCount && self.dwOffset == other.dwOffset && self.dwContentCount == other.dwContentCount && self.pszTarget == other.pszTarget && self.dwContextIDLength == other.dwContextIDLength && self.lpContextID == other.lpContextID
    }
}
impl ::core::cmp::Eq for ADS_VLV {}
impl ::core::fmt::Debug for ADS_VLV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_VLV").field("dwBeforeCount", &self.dwBeforeCount).field("dwAfterCount", &self.dwAfterCount).field("dwOffset", &self.dwOffset).field("dwContentCount", &self.dwContentCount).field("pszTarget", &self.pszTarget).field("dwContextIDLength", &self.dwContextIDLength).field("lpContextID", &self.lpContextID).finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for CQFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for CQFORM {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.clsid == other.clsid && self.hIcon == other.hIcon && self.pszTitle == other.pszTitle
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for CQFORM {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::fmt::Debug for CQFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CQFORM").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("clsid", &self.clsid).field("hIcon", &self.hIcon).field("pszTitle", &self.pszTitle).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for CQPAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOMAINDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOMAINDESC {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.pszPath == other.pszPath && self.pszNCName == other.pszNCName && self.pszTrustParent == other.pszTrustParent && self.pszObjectClass == other.pszObjectClass && self.ulFlags == other.ulFlags && self.fDownLevel == other.fDownLevel && self.pdChildList == other.pdChildList && self.pdNextSibling == other.pdNextSibling
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOMAINDESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOMAINDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOMAINDESC").field("pszName", &self.pszName).field("pszPath", &self.pszPath).field("pszNCName", &self.pszNCName).field("pszTrustParent", &self.pszTrustParent).field("pszObjectClass", &self.pszObjectClass).field("ulFlags", &self.ulFlags).field("fDownLevel", &self.fDownLevel).field("pdChildList", &self.pdChildList).field("pdNextSibling", &self.pdNextSibling).finish()
    }
}
impl ::core::default::Default for DOMAIN_CONTROLLER_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOMAIN_CONTROLLER_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.DomainControllerName == other.DomainControllerName && self.DomainControllerAddress == other.DomainControllerAddress && self.DomainControllerAddressType == other.DomainControllerAddressType && self.DomainGuid == other.DomainGuid && self.DomainName == other.DomainName && self.DnsForestName == other.DnsForestName && self.Flags == other.Flags && self.DcSiteName == other.DcSiteName && self.ClientSiteName == other.ClientSiteName
    }
}
impl ::core::cmp::Eq for DOMAIN_CONTROLLER_INFOA {}
impl ::core::fmt::Debug for DOMAIN_CONTROLLER_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOMAIN_CONTROLLER_INFOA").field("DomainControllerName", &self.DomainControllerName).field("DomainControllerAddress", &self.DomainControllerAddress).field("DomainControllerAddressType", &self.DomainControllerAddressType).field("DomainGuid", &self.DomainGuid).field("DomainName", &self.DomainName).field("DnsForestName", &self.DnsForestName).field("Flags", &self.Flags).field("DcSiteName", &self.DcSiteName).field("ClientSiteName", &self.ClientSiteName).finish()
    }
}
impl ::core::default::Default for DOMAIN_CONTROLLER_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOMAIN_CONTROLLER_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.DomainControllerName == other.DomainControllerName && self.DomainControllerAddress == other.DomainControllerAddress && self.DomainControllerAddressType == other.DomainControllerAddressType && self.DomainGuid == other.DomainGuid && self.DomainName == other.DomainName && self.DnsForestName == other.DnsForestName && self.Flags == other.Flags && self.DcSiteName == other.DcSiteName && self.ClientSiteName == other.ClientSiteName
    }
}
impl ::core::cmp::Eq for DOMAIN_CONTROLLER_INFOW {}
impl ::core::fmt::Debug for DOMAIN_CONTROLLER_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOMAIN_CONTROLLER_INFOW").field("DomainControllerName", &self.DomainControllerName).field("DomainControllerAddress", &self.DomainControllerAddress).field("DomainControllerAddressType", &self.DomainControllerAddressType).field("DomainGuid", &self.DomainGuid).field("DomainName", &self.DomainName).field("DnsForestName", &self.DnsForestName).field("Flags", &self.Flags).field("DcSiteName", &self.DcSiteName).field("ClientSiteName", &self.ClientSiteName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOMAIN_TREE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOMAIN_TREE {
    fn eq(&self, other: &Self) -> bool {
        self.dsSize == other.dsSize && self.dwCount == other.dwCount && self.aDomains == other.aDomains
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOMAIN_TREE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOMAIN_TREE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOMAIN_TREE").field("dsSize", &self.dsSize).field("dwCount", &self.dwCount).field("aDomains", &self.aDomains).finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for DSA_NEWOBJ_DISPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for DSA_NEWOBJ_DISPINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hObjClassIcon == other.hObjClassIcon && self.lpszWizTitle == other.lpszWizTitle && self.lpszContDisplayName == other.lpszContDisplayName
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for DSA_NEWOBJ_DISPINFO {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::fmt::Debug for DSA_NEWOBJ_DISPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSA_NEWOBJ_DISPINFO").field("dwSize", &self.dwSize).field("hObjClassIcon", &self.hObjClassIcon).field("lpszWizTitle", &self.lpszWizTitle).field("lpszContDisplayName", &self.lpszContDisplayName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSBITEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSBITEMA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pszADsPath == other.pszADsPath && self.pszClass == other.pszClass && self.dwMask == other.dwMask && self.dwState == other.dwState && self.dwStateMask == other.dwStateMask && self.szDisplayName == other.szDisplayName && self.szIconLocation == other.szIconLocation && self.iIconResID == other.iIconResID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSBITEMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSBITEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSBITEMA").field("cbStruct", &self.cbStruct).field("pszADsPath", &self.pszADsPath).field("pszClass", &self.pszClass).field("dwMask", &self.dwMask).field("dwState", &self.dwState).field("dwStateMask", &self.dwStateMask).field("szDisplayName", &self.szDisplayName).field("szIconLocation", &self.szIconLocation).field("iIconResID", &self.iIconResID).finish()
    }
}
impl ::core::default::Default for DSBITEMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSBITEMW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pszADsPath == other.pszADsPath && self.pszClass == other.pszClass && self.dwMask == other.dwMask && self.dwState == other.dwState && self.dwStateMask == other.dwStateMask && self.szDisplayName == other.szDisplayName && self.szIconLocation == other.szIconLocation && self.iIconResID == other.iIconResID
    }
}
impl ::core::cmp::Eq for DSBITEMW {}
impl ::core::fmt::Debug for DSBITEMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSBITEMW").field("cbStruct", &self.cbStruct).field("pszADsPath", &self.pszADsPath).field("pszClass", &self.pszClass).field("dwMask", &self.dwMask).field("dwState", &self.dwState).field("dwStateMask", &self.dwStateMask).field("szDisplayName", &self.szDisplayName).field("szIconLocation", &self.szIconLocation).field("iIconResID", &self.iIconResID).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::default::Default for DSBROWSEINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::default::Default for DSBROWSEINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DSCLASSCREATIONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSCLASSCREATIONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.clsidWizardDialog == other.clsidWizardDialog && self.clsidWizardPrimaryPage == other.clsidWizardPrimaryPage && self.cWizardExtensions == other.cWizardExtensions && self.aWizardExtensions == other.aWizardExtensions
    }
}
impl ::core::cmp::Eq for DSCLASSCREATIONINFO {}
impl ::core::fmt::Debug for DSCLASSCREATIONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSCLASSCREATIONINFO").field("dwFlags", &self.dwFlags).field("clsidWizardDialog", &self.clsidWizardDialog).field("clsidWizardPrimaryPage", &self.clsidWizardPrimaryPage).field("cWizardExtensions", &self.cWizardExtensions).field("aWizardExtensions", &self.aWizardExtensions).finish()
    }
}
impl ::core::default::Default for DSCOLUMN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSCOLUMN {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.fmt == other.fmt && self.cx == other.cx && self.idsName == other.idsName && self.offsetProperty == other.offsetProperty && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for DSCOLUMN {}
impl ::core::fmt::Debug for DSCOLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSCOLUMN").field("dwFlags", &self.dwFlags).field("fmt", &self.fmt).field("cx", &self.cx).field("idsName", &self.idsName).field("offsetProperty", &self.offsetProperty).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::default::Default for DSDISPLAYSPECOPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSDISPLAYSPECOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.offsetAttribPrefix == other.offsetAttribPrefix && self.offsetUserName == other.offsetUserName && self.offsetPassword == other.offsetPassword && self.offsetServer == other.offsetServer && self.offsetServerConfigPath == other.offsetServerConfigPath
    }
}
impl ::core::cmp::Eq for DSDISPLAYSPECOPTIONS {}
impl ::core::fmt::Debug for DSDISPLAYSPECOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSDISPLAYSPECOPTIONS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("offsetAttribPrefix", &self.offsetAttribPrefix).field("offsetUserName", &self.offsetUserName).field("offsetPassword", &self.offsetPassword).field("offsetServer", &self.offsetServer).field("offsetServerConfigPath", &self.offsetServerConfigPath).finish()
    }
}
impl ::core::default::Default for DSOBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSOBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwProviderFlags == other.dwProviderFlags && self.offsetName == other.offsetName && self.offsetClass == other.offsetClass
    }
}
impl ::core::cmp::Eq for DSOBJECT {}
impl ::core::fmt::Debug for DSOBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSOBJECT").field("dwFlags", &self.dwFlags).field("dwProviderFlags", &self.dwProviderFlags).field("offsetName", &self.offsetName).field("offsetClass", &self.offsetClass).finish()
    }
}
impl ::core::default::Default for DSOBJECTNAMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSOBJECTNAMES {
    fn eq(&self, other: &Self) -> bool {
        self.clsidNamespace == other.clsidNamespace && self.cItems == other.cItems && self.aObjects == other.aObjects
    }
}
impl ::core::cmp::Eq for DSOBJECTNAMES {}
impl ::core::fmt::Debug for DSOBJECTNAMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSOBJECTNAMES").field("clsidNamespace", &self.clsidNamespace).field("cItems", &self.cItems).field("aObjects", &self.aObjects).finish()
    }
}
impl ::core::default::Default for DSOP_FILTER_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSOP_FILTER_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.Uplevel == other.Uplevel && self.flDownlevel == other.flDownlevel
    }
}
impl ::core::cmp::Eq for DSOP_FILTER_FLAGS {}
impl ::core::fmt::Debug for DSOP_FILTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSOP_FILTER_FLAGS").field("Uplevel", &self.Uplevel).field("flDownlevel", &self.flDownlevel).finish()
    }
}
impl ::core::default::Default for DSOP_INIT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSOP_INIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pwzTargetComputer == other.pwzTargetComputer && self.cDsScopeInfos == other.cDsScopeInfos && self.aDsScopeInfos == other.aDsScopeInfos && self.flOptions == other.flOptions && self.cAttributesToFetch == other.cAttributesToFetch && self.apwzAttributeNames == other.apwzAttributeNames
    }
}
impl ::core::cmp::Eq for DSOP_INIT_INFO {}
impl ::core::fmt::Debug for DSOP_INIT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSOP_INIT_INFO").field("cbSize", &self.cbSize).field("pwzTargetComputer", &self.pwzTargetComputer).field("cDsScopeInfos", &self.cDsScopeInfos).field("aDsScopeInfos", &self.aDsScopeInfos).field("flOptions", &self.flOptions).field("cAttributesToFetch", &self.cAttributesToFetch).field("apwzAttributeNames", &self.apwzAttributeNames).finish()
    }
}
impl ::core::default::Default for DSOP_SCOPE_INIT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSOP_SCOPE_INIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.flType == other.flType && self.flScope == other.flScope && self.FilterFlags == other.FilterFlags && self.pwzDcName == other.pwzDcName && self.pwzADsPath == other.pwzADsPath && self.hr == other.hr
    }
}
impl ::core::cmp::Eq for DSOP_SCOPE_INIT_INFO {}
impl ::core::fmt::Debug for DSOP_SCOPE_INIT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSOP_SCOPE_INIT_INFO").field("cbSize", &self.cbSize).field("flType", &self.flType).field("flScope", &self.flScope).field("FilterFlags", &self.FilterFlags).field("pwzDcName", &self.pwzDcName).field("pwzADsPath", &self.pwzADsPath).field("hr", &self.hr).finish()
    }
}
impl ::core::default::Default for DSOP_UPLEVEL_FILTER_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSOP_UPLEVEL_FILTER_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.flBothModes == other.flBothModes && self.flMixedModeOnly == other.flMixedModeOnly && self.flNativeModeOnly == other.flNativeModeOnly
    }
}
impl ::core::cmp::Eq for DSOP_UPLEVEL_FILTER_FLAGS {}
impl ::core::fmt::Debug for DSOP_UPLEVEL_FILTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSOP_UPLEVEL_FILTER_FLAGS").field("flBothModes", &self.flBothModes).field("flMixedModeOnly", &self.flMixedModeOnly).field("flNativeModeOnly", &self.flNativeModeOnly).finish()
    }
}
impl ::core::default::Default for DSPROPERTYPAGEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSPROPERTYPAGEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.offsetString == other.offsetString
    }
}
impl ::core::cmp::Eq for DSPROPERTYPAGEINFO {}
impl ::core::fmt::Debug for DSPROPERTYPAGEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSPROPERTYPAGEINFO").field("offsetString", &self.offsetString).finish()
    }
}
impl ::core::default::Default for DSQUERYCLASSLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSQUERYCLASSLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.cClasses == other.cClasses && self.offsetClass == other.offsetClass
    }
}
impl ::core::cmp::Eq for DSQUERYCLASSLIST {}
impl ::core::fmt::Debug for DSQUERYCLASSLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSQUERYCLASSLIST").field("cbStruct", &self.cbStruct).field("cClasses", &self.cClasses).field("offsetClass", &self.offsetClass).finish()
    }
}
impl ::core::default::Default for DSQUERYINITPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSQUERYINITPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.pDefaultScope == other.pDefaultScope && self.pDefaultSaveLocation == other.pDefaultSaveLocation && self.pUserName == other.pUserName && self.pPassword == other.pPassword && self.pServer == other.pServer
    }
}
impl ::core::cmp::Eq for DSQUERYINITPARAMS {}
impl ::core::fmt::Debug for DSQUERYINITPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSQUERYINITPARAMS").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("pDefaultScope", &self.pDefaultScope).field("pDefaultSaveLocation", &self.pDefaultSaveLocation).field("pUserName", &self.pUserName).field("pPassword", &self.pPassword).field("pServer", &self.pServer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSQUERYPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSQUERYPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.hInstance == other.hInstance && self.offsetQuery == other.offsetQuery && self.iColumns == other.iColumns && self.dwReserved == other.dwReserved && self.aColumns == other.aColumns
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSQUERYPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSQUERYPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSQUERYPARAMS").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("hInstance", &self.hInstance).field("offsetQuery", &self.offsetQuery).field("iColumns", &self.iColumns).field("dwReserved", &self.dwReserved).field("aColumns", &self.aColumns).finish()
    }
}
impl ::core::default::Default for DSROLE_MACHINE_ROLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DSROLE_MACHINE_ROLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSROLE_MACHINE_ROLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DSROLE_OPERATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DSROLE_OPERATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSROLE_OPERATION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DSROLE_OPERATION_STATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSROLE_OPERATION_STATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.OperationState == other.OperationState
    }
}
impl ::core::cmp::Eq for DSROLE_OPERATION_STATE_INFO {}
impl ::core::fmt::Debug for DSROLE_OPERATION_STATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSROLE_OPERATION_STATE_INFO").field("OperationState", &self.OperationState).finish()
    }
}
impl ::core::default::Default for DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    fn eq(&self, other: &Self) -> bool {
        self.MachineRole == other.MachineRole && self.Flags == other.Flags && self.DomainNameFlat == other.DomainNameFlat && self.DomainNameDns == other.DomainNameDns && self.DomainForestName == other.DomainForestName && self.DomainGuid == other.DomainGuid
    }
}
impl ::core::cmp::Eq for DSROLE_PRIMARY_DOMAIN_INFO_BASIC {}
impl ::core::fmt::Debug for DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSROLE_PRIMARY_DOMAIN_INFO_BASIC").field("MachineRole", &self.MachineRole).field("Flags", &self.Flags).field("DomainNameFlat", &self.DomainNameFlat).field("DomainNameDns", &self.DomainNameDns).field("DomainForestName", &self.DomainForestName).field("DomainGuid", &self.DomainGuid).finish()
    }
}
impl ::core::default::Default for DSROLE_PRIMARY_DOMAIN_INFO_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DSROLE_PRIMARY_DOMAIN_INFO_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSROLE_PRIMARY_DOMAIN_INFO_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for DSROLE_SERVER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DSROLE_SERVER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSROLE_SERVER_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DSROLE_UPGRADE_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSROLE_UPGRADE_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.OperationState == other.OperationState && self.PreviousServerState == other.PreviousServerState
    }
}
impl ::core::cmp::Eq for DSROLE_UPGRADE_STATUS_INFO {}
impl ::core::fmt::Debug for DSROLE_UPGRADE_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSROLE_UPGRADE_STATUS_INFO").field("OperationState", &self.OperationState).field("PreviousServerState", &self.PreviousServerState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_DOMAIN_CONTROLLER_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_DOMAIN_CONTROLLER_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        self.NetbiosName == other.NetbiosName && self.DnsHostName == other.DnsHostName && self.SiteName == other.SiteName && self.ComputerObjectName == other.ComputerObjectName && self.ServerObjectName == other.ServerObjectName && self.fIsPdc == other.fIsPdc && self.fDsEnabled == other.fDsEnabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_DOMAIN_CONTROLLER_INFO_1A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_DOMAIN_CONTROLLER_INFO_1A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_DOMAIN_CONTROLLER_INFO_1A").field("NetbiosName", &self.NetbiosName).field("DnsHostName", &self.DnsHostName).field("SiteName", &self.SiteName).field("ComputerObjectName", &self.ComputerObjectName).field("ServerObjectName", &self.ServerObjectName).field("fIsPdc", &self.fIsPdc).field("fDsEnabled", &self.fDsEnabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_DOMAIN_CONTROLLER_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_DOMAIN_CONTROLLER_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        self.NetbiosName == other.NetbiosName && self.DnsHostName == other.DnsHostName && self.SiteName == other.SiteName && self.ComputerObjectName == other.ComputerObjectName && self.ServerObjectName == other.ServerObjectName && self.fIsPdc == other.fIsPdc && self.fDsEnabled == other.fDsEnabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_DOMAIN_CONTROLLER_INFO_1W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_DOMAIN_CONTROLLER_INFO_1W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_DOMAIN_CONTROLLER_INFO_1W").field("NetbiosName", &self.NetbiosName).field("DnsHostName", &self.DnsHostName).field("SiteName", &self.SiteName).field("ComputerObjectName", &self.ComputerObjectName).field("ServerObjectName", &self.ServerObjectName).field("fIsPdc", &self.fIsPdc).field("fDsEnabled", &self.fDsEnabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_DOMAIN_CONTROLLER_INFO_2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_DOMAIN_CONTROLLER_INFO_2A {
    fn eq(&self, other: &Self) -> bool {
        self.NetbiosName == other.NetbiosName && self.DnsHostName == other.DnsHostName && self.SiteName == other.SiteName && self.SiteObjectName == other.SiteObjectName && self.ComputerObjectName == other.ComputerObjectName && self.ServerObjectName == other.ServerObjectName && self.NtdsDsaObjectName == other.NtdsDsaObjectName && self.fIsPdc == other.fIsPdc && self.fDsEnabled == other.fDsEnabled && self.fIsGc == other.fIsGc && self.SiteObjectGuid == other.SiteObjectGuid && self.ComputerObjectGuid == other.ComputerObjectGuid && self.ServerObjectGuid == other.ServerObjectGuid && self.NtdsDsaObjectGuid == other.NtdsDsaObjectGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_DOMAIN_CONTROLLER_INFO_2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_DOMAIN_CONTROLLER_INFO_2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_DOMAIN_CONTROLLER_INFO_2A")
            .field("NetbiosName", &self.NetbiosName)
            .field("DnsHostName", &self.DnsHostName)
            .field("SiteName", &self.SiteName)
            .field("SiteObjectName", &self.SiteObjectName)
            .field("ComputerObjectName", &self.ComputerObjectName)
            .field("ServerObjectName", &self.ServerObjectName)
            .field("NtdsDsaObjectName", &self.NtdsDsaObjectName)
            .field("fIsPdc", &self.fIsPdc)
            .field("fDsEnabled", &self.fDsEnabled)
            .field("fIsGc", &self.fIsGc)
            .field("SiteObjectGuid", &self.SiteObjectGuid)
            .field("ComputerObjectGuid", &self.ComputerObjectGuid)
            .field("ServerObjectGuid", &self.ServerObjectGuid)
            .field("NtdsDsaObjectGuid", &self.NtdsDsaObjectGuid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_DOMAIN_CONTROLLER_INFO_2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_DOMAIN_CONTROLLER_INFO_2W {
    fn eq(&self, other: &Self) -> bool {
        self.NetbiosName == other.NetbiosName && self.DnsHostName == other.DnsHostName && self.SiteName == other.SiteName && self.SiteObjectName == other.SiteObjectName && self.ComputerObjectName == other.ComputerObjectName && self.ServerObjectName == other.ServerObjectName && self.NtdsDsaObjectName == other.NtdsDsaObjectName && self.fIsPdc == other.fIsPdc && self.fDsEnabled == other.fDsEnabled && self.fIsGc == other.fIsGc && self.SiteObjectGuid == other.SiteObjectGuid && self.ComputerObjectGuid == other.ComputerObjectGuid && self.ServerObjectGuid == other.ServerObjectGuid && self.NtdsDsaObjectGuid == other.NtdsDsaObjectGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_DOMAIN_CONTROLLER_INFO_2W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_DOMAIN_CONTROLLER_INFO_2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_DOMAIN_CONTROLLER_INFO_2W")
            .field("NetbiosName", &self.NetbiosName)
            .field("DnsHostName", &self.DnsHostName)
            .field("SiteName", &self.SiteName)
            .field("SiteObjectName", &self.SiteObjectName)
            .field("ComputerObjectName", &self.ComputerObjectName)
            .field("ServerObjectName", &self.ServerObjectName)
            .field("NtdsDsaObjectName", &self.NtdsDsaObjectName)
            .field("fIsPdc", &self.fIsPdc)
            .field("fDsEnabled", &self.fDsEnabled)
            .field("fIsGc", &self.fIsGc)
            .field("SiteObjectGuid", &self.SiteObjectGuid)
            .field("ComputerObjectGuid", &self.ComputerObjectGuid)
            .field("ServerObjectGuid", &self.ServerObjectGuid)
            .field("NtdsDsaObjectGuid", &self.NtdsDsaObjectGuid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_DOMAIN_CONTROLLER_INFO_3A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_DOMAIN_CONTROLLER_INFO_3A {
    fn eq(&self, other: &Self) -> bool {
        self.NetbiosName == other.NetbiosName && self.DnsHostName == other.DnsHostName && self.SiteName == other.SiteName && self.SiteObjectName == other.SiteObjectName && self.ComputerObjectName == other.ComputerObjectName && self.ServerObjectName == other.ServerObjectName && self.NtdsDsaObjectName == other.NtdsDsaObjectName && self.fIsPdc == other.fIsPdc && self.fDsEnabled == other.fDsEnabled && self.fIsGc == other.fIsGc && self.fIsRodc == other.fIsRodc && self.SiteObjectGuid == other.SiteObjectGuid && self.ComputerObjectGuid == other.ComputerObjectGuid && self.ServerObjectGuid == other.ServerObjectGuid && self.NtdsDsaObjectGuid == other.NtdsDsaObjectGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_DOMAIN_CONTROLLER_INFO_3A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_DOMAIN_CONTROLLER_INFO_3A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_DOMAIN_CONTROLLER_INFO_3A")
            .field("NetbiosName", &self.NetbiosName)
            .field("DnsHostName", &self.DnsHostName)
            .field("SiteName", &self.SiteName)
            .field("SiteObjectName", &self.SiteObjectName)
            .field("ComputerObjectName", &self.ComputerObjectName)
            .field("ServerObjectName", &self.ServerObjectName)
            .field("NtdsDsaObjectName", &self.NtdsDsaObjectName)
            .field("fIsPdc", &self.fIsPdc)
            .field("fDsEnabled", &self.fDsEnabled)
            .field("fIsGc", &self.fIsGc)
            .field("fIsRodc", &self.fIsRodc)
            .field("SiteObjectGuid", &self.SiteObjectGuid)
            .field("ComputerObjectGuid", &self.ComputerObjectGuid)
            .field("ServerObjectGuid", &self.ServerObjectGuid)
            .field("NtdsDsaObjectGuid", &self.NtdsDsaObjectGuid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_DOMAIN_CONTROLLER_INFO_3W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_DOMAIN_CONTROLLER_INFO_3W {
    fn eq(&self, other: &Self) -> bool {
        self.NetbiosName == other.NetbiosName && self.DnsHostName == other.DnsHostName && self.SiteName == other.SiteName && self.SiteObjectName == other.SiteObjectName && self.ComputerObjectName == other.ComputerObjectName && self.ServerObjectName == other.ServerObjectName && self.NtdsDsaObjectName == other.NtdsDsaObjectName && self.fIsPdc == other.fIsPdc && self.fDsEnabled == other.fDsEnabled && self.fIsGc == other.fIsGc && self.fIsRodc == other.fIsRodc && self.SiteObjectGuid == other.SiteObjectGuid && self.ComputerObjectGuid == other.ComputerObjectGuid && self.ServerObjectGuid == other.ServerObjectGuid && self.NtdsDsaObjectGuid == other.NtdsDsaObjectGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_DOMAIN_CONTROLLER_INFO_3W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_DOMAIN_CONTROLLER_INFO_3W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_DOMAIN_CONTROLLER_INFO_3W")
            .field("NetbiosName", &self.NetbiosName)
            .field("DnsHostName", &self.DnsHostName)
            .field("SiteName", &self.SiteName)
            .field("SiteObjectName", &self.SiteObjectName)
            .field("ComputerObjectName", &self.ComputerObjectName)
            .field("ServerObjectName", &self.ServerObjectName)
            .field("NtdsDsaObjectName", &self.NtdsDsaObjectName)
            .field("fIsPdc", &self.fIsPdc)
            .field("fDsEnabled", &self.fDsEnabled)
            .field("fIsGc", &self.fIsGc)
            .field("fIsRodc", &self.fIsRodc)
            .field("SiteObjectGuid", &self.SiteObjectGuid)
            .field("ComputerObjectGuid", &self.ComputerObjectGuid)
            .field("ServerObjectGuid", &self.ServerObjectGuid)
            .field("NtdsDsaObjectGuid", &self.NtdsDsaObjectGuid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_DOMAIN_TRUSTSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_DOMAIN_TRUSTSA {
    fn eq(&self, other: &Self) -> bool {
        self.NetbiosDomainName == other.NetbiosDomainName && self.DnsDomainName == other.DnsDomainName && self.Flags == other.Flags && self.ParentIndex == other.ParentIndex && self.TrustType == other.TrustType && self.TrustAttributes == other.TrustAttributes && self.DomainSid == other.DomainSid && self.DomainGuid == other.DomainGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_DOMAIN_TRUSTSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_DOMAIN_TRUSTSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_DOMAIN_TRUSTSA").field("NetbiosDomainName", &self.NetbiosDomainName).field("DnsDomainName", &self.DnsDomainName).field("Flags", &self.Flags).field("ParentIndex", &self.ParentIndex).field("TrustType", &self.TrustType).field("TrustAttributes", &self.TrustAttributes).field("DomainSid", &self.DomainSid).field("DomainGuid", &self.DomainGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_DOMAIN_TRUSTSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_DOMAIN_TRUSTSW {
    fn eq(&self, other: &Self) -> bool {
        self.NetbiosDomainName == other.NetbiosDomainName && self.DnsDomainName == other.DnsDomainName && self.Flags == other.Flags && self.ParentIndex == other.ParentIndex && self.TrustType == other.TrustType && self.TrustAttributes == other.TrustAttributes && self.DomainSid == other.DomainSid && self.DomainGuid == other.DomainGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_DOMAIN_TRUSTSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_DOMAIN_TRUSTSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_DOMAIN_TRUSTSW").field("NetbiosDomainName", &self.NetbiosDomainName).field("DnsDomainName", &self.DnsDomainName).field("Flags", &self.Flags).field("ParentIndex", &self.ParentIndex).field("TrustType", &self.TrustType).field("TrustAttributes", &self.TrustAttributes).field("DomainSid", &self.DomainSid).field("DomainGuid", &self.DomainGuid).finish()
    }
}
impl ::core::default::Default for DS_KCC_TASKID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DS_KCC_TASKID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_KCC_TASKID").field(&self.0).finish()
    }
}
impl ::core::default::Default for DS_MANGLE_FOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DS_MANGLE_FOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_MANGLE_FOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for DS_NAME_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DS_NAME_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_NAME_ERROR").field(&self.0).finish()
    }
}
impl ::core::default::Default for DS_NAME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DS_NAME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_NAME_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DS_NAME_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DS_NAME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_NAME_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for DS_NAME_RESULTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS_NAME_RESULTA {
    fn eq(&self, other: &Self) -> bool {
        self.cItems == other.cItems && self.rItems == other.rItems
    }
}
impl ::core::cmp::Eq for DS_NAME_RESULTA {}
impl ::core::fmt::Debug for DS_NAME_RESULTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_NAME_RESULTA").field("cItems", &self.cItems).field("rItems", &self.rItems).finish()
    }
}
impl ::core::default::Default for DS_NAME_RESULTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS_NAME_RESULTW {
    fn eq(&self, other: &Self) -> bool {
        self.cItems == other.cItems && self.rItems == other.rItems
    }
}
impl ::core::cmp::Eq for DS_NAME_RESULTW {}
impl ::core::fmt::Debug for DS_NAME_RESULTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_NAME_RESULTW").field("cItems", &self.cItems).field("rItems", &self.rItems).finish()
    }
}
impl ::core::default::Default for DS_NAME_RESULT_ITEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS_NAME_RESULT_ITEMA {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.pDomain == other.pDomain && self.pName == other.pName
    }
}
impl ::core::cmp::Eq for DS_NAME_RESULT_ITEMA {}
impl ::core::fmt::Debug for DS_NAME_RESULT_ITEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_NAME_RESULT_ITEMA").field("status", &self.status).field("pDomain", &self.pDomain).field("pName", &self.pName).finish()
    }
}
impl ::core::default::Default for DS_NAME_RESULT_ITEMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS_NAME_RESULT_ITEMW {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.pDomain == other.pDomain && self.pName == other.pName
    }
}
impl ::core::cmp::Eq for DS_NAME_RESULT_ITEMW {}
impl ::core::fmt::Debug for DS_NAME_RESULT_ITEMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_NAME_RESULT_ITEMW").field("status", &self.status).field("pDomain", &self.pDomain).field("pName", &self.pName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_ATTR_META_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_ATTR_META_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttributeName == other.pszAttributeName && self.dwVersion == other.dwVersion && self.ftimeLastOriginatingChange == other.ftimeLastOriginatingChange && self.uuidLastOriginatingDsaInvocationID == other.uuidLastOriginatingDsaInvocationID && self.usnOriginatingChange == other.usnOriginatingChange && self.usnLocalChange == other.usnLocalChange
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_ATTR_META_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_ATTR_META_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_ATTR_META_DATA").field("pszAttributeName", &self.pszAttributeName).field("dwVersion", &self.dwVersion).field("ftimeLastOriginatingChange", &self.ftimeLastOriginatingChange).field("uuidLastOriginatingDsaInvocationID", &self.uuidLastOriginatingDsaInvocationID).field("usnOriginatingChange", &self.usnOriginatingChange).field("usnLocalChange", &self.usnLocalChange).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_ATTR_META_DATA_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_ATTR_META_DATA_2 {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttributeName == other.pszAttributeName && self.dwVersion == other.dwVersion && self.ftimeLastOriginatingChange == other.ftimeLastOriginatingChange && self.uuidLastOriginatingDsaInvocationID == other.uuidLastOriginatingDsaInvocationID && self.usnOriginatingChange == other.usnOriginatingChange && self.usnLocalChange == other.usnLocalChange && self.pszLastOriginatingDsaDN == other.pszLastOriginatingDsaDN
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_ATTR_META_DATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_ATTR_META_DATA_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_ATTR_META_DATA_2").field("pszAttributeName", &self.pszAttributeName).field("dwVersion", &self.dwVersion).field("ftimeLastOriginatingChange", &self.ftimeLastOriginatingChange).field("uuidLastOriginatingDsaInvocationID", &self.uuidLastOriginatingDsaInvocationID).field("usnOriginatingChange", &self.usnOriginatingChange).field("usnLocalChange", &self.usnLocalChange).field("pszLastOriginatingDsaDN", &self.pszLastOriginatingDsaDN).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_ATTR_META_DATA_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_ATTR_META_DATA_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.oszAttributeName == other.oszAttributeName && self.dwVersion == other.dwVersion && self.ftimeLastOriginatingChange == other.ftimeLastOriginatingChange && self.uuidLastOriginatingDsaInvocationID == other.uuidLastOriginatingDsaInvocationID && self.usnOriginatingChange == other.usnOriginatingChange && self.usnLocalChange == other.usnLocalChange && self.oszLastOriginatingDsaDN == other.oszLastOriginatingDsaDN
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_ATTR_META_DATA_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_ATTR_META_DATA_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_ATTR_META_DATA_BLOB").field("oszAttributeName", &self.oszAttributeName).field("dwVersion", &self.dwVersion).field("ftimeLastOriginatingChange", &self.ftimeLastOriginatingChange).field("uuidLastOriginatingDsaInvocationID", &self.uuidLastOriginatingDsaInvocationID).field("usnOriginatingChange", &self.usnOriginatingChange).field("usnLocalChange", &self.usnLocalChange).field("oszLastOriginatingDsaDN", &self.oszLastOriginatingDsaDN).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_ATTR_VALUE_META_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_ATTR_VALUE_META_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cNumEntries == other.cNumEntries && self.dwEnumerationContext == other.dwEnumerationContext && self.rgMetaData == other.rgMetaData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_ATTR_VALUE_META_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_ATTR_VALUE_META_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_ATTR_VALUE_META_DATA").field("cNumEntries", &self.cNumEntries).field("dwEnumerationContext", &self.dwEnumerationContext).field("rgMetaData", &self.rgMetaData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_ATTR_VALUE_META_DATA_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_ATTR_VALUE_META_DATA_2 {
    fn eq(&self, other: &Self) -> bool {
        self.cNumEntries == other.cNumEntries && self.dwEnumerationContext == other.dwEnumerationContext && self.rgMetaData == other.rgMetaData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_ATTR_VALUE_META_DATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_ATTR_VALUE_META_DATA_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_ATTR_VALUE_META_DATA_2").field("cNumEntries", &self.cNumEntries).field("dwEnumerationContext", &self.dwEnumerationContext).field("rgMetaData", &self.rgMetaData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_ATTR_VALUE_META_DATA_EXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_ATTR_VALUE_META_DATA_EXT {
    fn eq(&self, other: &Self) -> bool {
        self.cNumEntries == other.cNumEntries && self.dwEnumerationContext == other.dwEnumerationContext && self.rgMetaData == other.rgMetaData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_ATTR_VALUE_META_DATA_EXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_ATTR_VALUE_META_DATA_EXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_ATTR_VALUE_META_DATA_EXT").field("cNumEntries", &self.cNumEntries).field("dwEnumerationContext", &self.dwEnumerationContext).field("rgMetaData", &self.rgMetaData).finish()
    }
}
impl ::core::default::Default for DS_REPL_CURSOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS_REPL_CURSOR {
    fn eq(&self, other: &Self) -> bool {
        self.uuidSourceDsaInvocationID == other.uuidSourceDsaInvocationID && self.usnAttributeFilter == other.usnAttributeFilter
    }
}
impl ::core::cmp::Eq for DS_REPL_CURSOR {}
impl ::core::fmt::Debug for DS_REPL_CURSOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_CURSOR").field("uuidSourceDsaInvocationID", &self.uuidSourceDsaInvocationID).field("usnAttributeFilter", &self.usnAttributeFilter).finish()
    }
}
impl ::core::default::Default for DS_REPL_CURSORS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS_REPL_CURSORS {
    fn eq(&self, other: &Self) -> bool {
        self.cNumCursors == other.cNumCursors && self.dwReserved == other.dwReserved && self.rgCursor == other.rgCursor
    }
}
impl ::core::cmp::Eq for DS_REPL_CURSORS {}
impl ::core::fmt::Debug for DS_REPL_CURSORS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_CURSORS").field("cNumCursors", &self.cNumCursors).field("dwReserved", &self.dwReserved).field("rgCursor", &self.rgCursor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_CURSORS_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_CURSORS_2 {
    fn eq(&self, other: &Self) -> bool {
        self.cNumCursors == other.cNumCursors && self.dwEnumerationContext == other.dwEnumerationContext && self.rgCursor == other.rgCursor
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_CURSORS_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_CURSORS_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_CURSORS_2").field("cNumCursors", &self.cNumCursors).field("dwEnumerationContext", &self.dwEnumerationContext).field("rgCursor", &self.rgCursor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_CURSORS_3W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_CURSORS_3W {
    fn eq(&self, other: &Self) -> bool {
        self.cNumCursors == other.cNumCursors && self.dwEnumerationContext == other.dwEnumerationContext && self.rgCursor == other.rgCursor
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_CURSORS_3W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_CURSORS_3W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_CURSORS_3W").field("cNumCursors", &self.cNumCursors).field("dwEnumerationContext", &self.dwEnumerationContext).field("rgCursor", &self.rgCursor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_CURSOR_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_CURSOR_2 {
    fn eq(&self, other: &Self) -> bool {
        self.uuidSourceDsaInvocationID == other.uuidSourceDsaInvocationID && self.usnAttributeFilter == other.usnAttributeFilter && self.ftimeLastSyncSuccess == other.ftimeLastSyncSuccess
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_CURSOR_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_CURSOR_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_CURSOR_2").field("uuidSourceDsaInvocationID", &self.uuidSourceDsaInvocationID).field("usnAttributeFilter", &self.usnAttributeFilter).field("ftimeLastSyncSuccess", &self.ftimeLastSyncSuccess).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_CURSOR_3W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_CURSOR_3W {
    fn eq(&self, other: &Self) -> bool {
        self.uuidSourceDsaInvocationID == other.uuidSourceDsaInvocationID && self.usnAttributeFilter == other.usnAttributeFilter && self.ftimeLastSyncSuccess == other.ftimeLastSyncSuccess && self.pszSourceDsaDN == other.pszSourceDsaDN
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_CURSOR_3W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_CURSOR_3W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_CURSOR_3W").field("uuidSourceDsaInvocationID", &self.uuidSourceDsaInvocationID).field("usnAttributeFilter", &self.usnAttributeFilter).field("ftimeLastSyncSuccess", &self.ftimeLastSyncSuccess).field("pszSourceDsaDN", &self.pszSourceDsaDN).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_CURSOR_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_CURSOR_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.uuidSourceDsaInvocationID == other.uuidSourceDsaInvocationID && self.usnAttributeFilter == other.usnAttributeFilter && self.ftimeLastSyncSuccess == other.ftimeLastSyncSuccess && self.oszSourceDsaDN == other.oszSourceDsaDN
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_CURSOR_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_CURSOR_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_CURSOR_BLOB").field("uuidSourceDsaInvocationID", &self.uuidSourceDsaInvocationID).field("usnAttributeFilter", &self.usnAttributeFilter).field("ftimeLastSyncSuccess", &self.ftimeLastSyncSuccess).field("oszSourceDsaDN", &self.oszSourceDsaDN).finish()
    }
}
impl ::core::default::Default for DS_REPL_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DS_REPL_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_REPL_INFO_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_KCC_DSA_FAILURESW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_KCC_DSA_FAILURESW {
    fn eq(&self, other: &Self) -> bool {
        self.cNumEntries == other.cNumEntries && self.dwReserved == other.dwReserved && self.rgDsaFailure == other.rgDsaFailure
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_KCC_DSA_FAILURESW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_KCC_DSA_FAILURESW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_KCC_DSA_FAILURESW").field("cNumEntries", &self.cNumEntries).field("dwReserved", &self.dwReserved).field("rgDsaFailure", &self.rgDsaFailure).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_KCC_DSA_FAILUREW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_KCC_DSA_FAILUREW {
    fn eq(&self, other: &Self) -> bool {
        self.pszDsaDN == other.pszDsaDN && self.uuidDsaObjGuid == other.uuidDsaObjGuid && self.ftimeFirstFailure == other.ftimeFirstFailure && self.cNumFailures == other.cNumFailures && self.dwLastResult == other.dwLastResult
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_KCC_DSA_FAILUREW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_KCC_DSA_FAILUREW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_KCC_DSA_FAILUREW").field("pszDsaDN", &self.pszDsaDN).field("uuidDsaObjGuid", &self.uuidDsaObjGuid).field("ftimeFirstFailure", &self.ftimeFirstFailure).field("cNumFailures", &self.cNumFailures).field("dwLastResult", &self.dwLastResult).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_KCC_DSA_FAILUREW_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_KCC_DSA_FAILUREW_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.oszDsaDN == other.oszDsaDN && self.uuidDsaObjGuid == other.uuidDsaObjGuid && self.ftimeFirstFailure == other.ftimeFirstFailure && self.cNumFailures == other.cNumFailures && self.dwLastResult == other.dwLastResult
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_KCC_DSA_FAILUREW_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_KCC_DSA_FAILUREW_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_KCC_DSA_FAILUREW_BLOB").field("oszDsaDN", &self.oszDsaDN).field("uuidDsaObjGuid", &self.uuidDsaObjGuid).field("ftimeFirstFailure", &self.ftimeFirstFailure).field("cNumFailures", &self.cNumFailures).field("dwLastResult", &self.dwLastResult).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_NEIGHBORSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_NEIGHBORSW {
    fn eq(&self, other: &Self) -> bool {
        self.cNumNeighbors == other.cNumNeighbors && self.dwReserved == other.dwReserved && self.rgNeighbor == other.rgNeighbor
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_NEIGHBORSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_NEIGHBORSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_NEIGHBORSW").field("cNumNeighbors", &self.cNumNeighbors).field("dwReserved", &self.dwReserved).field("rgNeighbor", &self.rgNeighbor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_NEIGHBORW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_NEIGHBORW {
    fn eq(&self, other: &Self) -> bool {
        self.pszNamingContext == other.pszNamingContext
            && self.pszSourceDsaDN == other.pszSourceDsaDN
            && self.pszSourceDsaAddress == other.pszSourceDsaAddress
            && self.pszAsyncIntersiteTransportDN == other.pszAsyncIntersiteTransportDN
            && self.dwReplicaFlags == other.dwReplicaFlags
            && self.dwReserved == other.dwReserved
            && self.uuidNamingContextObjGuid == other.uuidNamingContextObjGuid
            && self.uuidSourceDsaObjGuid == other.uuidSourceDsaObjGuid
            && self.uuidSourceDsaInvocationID == other.uuidSourceDsaInvocationID
            && self.uuidAsyncIntersiteTransportObjGuid == other.uuidAsyncIntersiteTransportObjGuid
            && self.usnLastObjChangeSynced == other.usnLastObjChangeSynced
            && self.usnAttributeFilter == other.usnAttributeFilter
            && self.ftimeLastSyncSuccess == other.ftimeLastSyncSuccess
            && self.ftimeLastSyncAttempt == other.ftimeLastSyncAttempt
            && self.dwLastSyncResult == other.dwLastSyncResult
            && self.cNumConsecutiveSyncFailures == other.cNumConsecutiveSyncFailures
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_NEIGHBORW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_NEIGHBORW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_NEIGHBORW")
            .field("pszNamingContext", &self.pszNamingContext)
            .field("pszSourceDsaDN", &self.pszSourceDsaDN)
            .field("pszSourceDsaAddress", &self.pszSourceDsaAddress)
            .field("pszAsyncIntersiteTransportDN", &self.pszAsyncIntersiteTransportDN)
            .field("dwReplicaFlags", &self.dwReplicaFlags)
            .field("dwReserved", &self.dwReserved)
            .field("uuidNamingContextObjGuid", &self.uuidNamingContextObjGuid)
            .field("uuidSourceDsaObjGuid", &self.uuidSourceDsaObjGuid)
            .field("uuidSourceDsaInvocationID", &self.uuidSourceDsaInvocationID)
            .field("uuidAsyncIntersiteTransportObjGuid", &self.uuidAsyncIntersiteTransportObjGuid)
            .field("usnLastObjChangeSynced", &self.usnLastObjChangeSynced)
            .field("usnAttributeFilter", &self.usnAttributeFilter)
            .field("ftimeLastSyncSuccess", &self.ftimeLastSyncSuccess)
            .field("ftimeLastSyncAttempt", &self.ftimeLastSyncAttempt)
            .field("dwLastSyncResult", &self.dwLastSyncResult)
            .field("cNumConsecutiveSyncFailures", &self.cNumConsecutiveSyncFailures)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_NEIGHBORW_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_NEIGHBORW_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.oszNamingContext == other.oszNamingContext
            && self.oszSourceDsaDN == other.oszSourceDsaDN
            && self.oszSourceDsaAddress == other.oszSourceDsaAddress
            && self.oszAsyncIntersiteTransportDN == other.oszAsyncIntersiteTransportDN
            && self.dwReplicaFlags == other.dwReplicaFlags
            && self.dwReserved == other.dwReserved
            && self.uuidNamingContextObjGuid == other.uuidNamingContextObjGuid
            && self.uuidSourceDsaObjGuid == other.uuidSourceDsaObjGuid
            && self.uuidSourceDsaInvocationID == other.uuidSourceDsaInvocationID
            && self.uuidAsyncIntersiteTransportObjGuid == other.uuidAsyncIntersiteTransportObjGuid
            && self.usnLastObjChangeSynced == other.usnLastObjChangeSynced
            && self.usnAttributeFilter == other.usnAttributeFilter
            && self.ftimeLastSyncSuccess == other.ftimeLastSyncSuccess
            && self.ftimeLastSyncAttempt == other.ftimeLastSyncAttempt
            && self.dwLastSyncResult == other.dwLastSyncResult
            && self.cNumConsecutiveSyncFailures == other.cNumConsecutiveSyncFailures
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_NEIGHBORW_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_NEIGHBORW_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_NEIGHBORW_BLOB")
            .field("oszNamingContext", &self.oszNamingContext)
            .field("oszSourceDsaDN", &self.oszSourceDsaDN)
            .field("oszSourceDsaAddress", &self.oszSourceDsaAddress)
            .field("oszAsyncIntersiteTransportDN", &self.oszAsyncIntersiteTransportDN)
            .field("dwReplicaFlags", &self.dwReplicaFlags)
            .field("dwReserved", &self.dwReserved)
            .field("uuidNamingContextObjGuid", &self.uuidNamingContextObjGuid)
            .field("uuidSourceDsaObjGuid", &self.uuidSourceDsaObjGuid)
            .field("uuidSourceDsaInvocationID", &self.uuidSourceDsaInvocationID)
            .field("uuidAsyncIntersiteTransportObjGuid", &self.uuidAsyncIntersiteTransportObjGuid)
            .field("usnLastObjChangeSynced", &self.usnLastObjChangeSynced)
            .field("usnAttributeFilter", &self.usnAttributeFilter)
            .field("ftimeLastSyncSuccess", &self.ftimeLastSyncSuccess)
            .field("ftimeLastSyncAttempt", &self.ftimeLastSyncAttempt)
            .field("dwLastSyncResult", &self.dwLastSyncResult)
            .field("cNumConsecutiveSyncFailures", &self.cNumConsecutiveSyncFailures)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_OBJ_META_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_OBJ_META_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cNumEntries == other.cNumEntries && self.dwReserved == other.dwReserved && self.rgMetaData == other.rgMetaData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_OBJ_META_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_OBJ_META_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_OBJ_META_DATA").field("cNumEntries", &self.cNumEntries).field("dwReserved", &self.dwReserved).field("rgMetaData", &self.rgMetaData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_OBJ_META_DATA_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_OBJ_META_DATA_2 {
    fn eq(&self, other: &Self) -> bool {
        self.cNumEntries == other.cNumEntries && self.dwReserved == other.dwReserved && self.rgMetaData == other.rgMetaData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_OBJ_META_DATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_OBJ_META_DATA_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_OBJ_META_DATA_2").field("cNumEntries", &self.cNumEntries).field("dwReserved", &self.dwReserved).field("rgMetaData", &self.rgMetaData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_OPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_OPW {
    fn eq(&self, other: &Self) -> bool {
        self.ftimeEnqueued == other.ftimeEnqueued && self.ulSerialNumber == other.ulSerialNumber && self.ulPriority == other.ulPriority && self.OpType == other.OpType && self.ulOptions == other.ulOptions && self.pszNamingContext == other.pszNamingContext && self.pszDsaDN == other.pszDsaDN && self.pszDsaAddress == other.pszDsaAddress && self.uuidNamingContextObjGuid == other.uuidNamingContextObjGuid && self.uuidDsaObjGuid == other.uuidDsaObjGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_OPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_OPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_OPW").field("ftimeEnqueued", &self.ftimeEnqueued).field("ulSerialNumber", &self.ulSerialNumber).field("ulPriority", &self.ulPriority).field("OpType", &self.OpType).field("ulOptions", &self.ulOptions).field("pszNamingContext", &self.pszNamingContext).field("pszDsaDN", &self.pszDsaDN).field("pszDsaAddress", &self.pszDsaAddress).field("uuidNamingContextObjGuid", &self.uuidNamingContextObjGuid).field("uuidDsaObjGuid", &self.uuidDsaObjGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_OPW_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_OPW_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.ftimeEnqueued == other.ftimeEnqueued && self.ulSerialNumber == other.ulSerialNumber && self.ulPriority == other.ulPriority && self.OpType == other.OpType && self.ulOptions == other.ulOptions && self.oszNamingContext == other.oszNamingContext && self.oszDsaDN == other.oszDsaDN && self.oszDsaAddress == other.oszDsaAddress && self.uuidNamingContextObjGuid == other.uuidNamingContextObjGuid && self.uuidDsaObjGuid == other.uuidDsaObjGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_OPW_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_OPW_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_OPW_BLOB").field("ftimeEnqueued", &self.ftimeEnqueued).field("ulSerialNumber", &self.ulSerialNumber).field("ulPriority", &self.ulPriority).field("OpType", &self.OpType).field("ulOptions", &self.ulOptions).field("oszNamingContext", &self.oszNamingContext).field("oszDsaDN", &self.oszDsaDN).field("oszDsaAddress", &self.oszDsaAddress).field("uuidNamingContextObjGuid", &self.uuidNamingContextObjGuid).field("uuidDsaObjGuid", &self.uuidDsaObjGuid).finish()
    }
}
impl ::core::default::Default for DS_REPL_OP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DS_REPL_OP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_REPL_OP_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_PENDING_OPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_PENDING_OPSW {
    fn eq(&self, other: &Self) -> bool {
        self.ftimeCurrentOpStarted == other.ftimeCurrentOpStarted && self.cNumPendingOps == other.cNumPendingOps && self.rgPendingOp == other.rgPendingOp
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_PENDING_OPSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_PENDING_OPSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_PENDING_OPSW").field("ftimeCurrentOpStarted", &self.ftimeCurrentOpStarted).field("cNumPendingOps", &self.cNumPendingOps).field("rgPendingOp", &self.rgPendingOp).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_QUEUE_STATISTICSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_QUEUE_STATISTICSW {
    fn eq(&self, other: &Self) -> bool {
        self.ftimeCurrentOpStarted == other.ftimeCurrentOpStarted && self.cNumPendingOps == other.cNumPendingOps && self.ftimeOldestSync == other.ftimeOldestSync && self.ftimeOldestAdd == other.ftimeOldestAdd && self.ftimeOldestMod == other.ftimeOldestMod && self.ftimeOldestDel == other.ftimeOldestDel && self.ftimeOldestUpdRefs == other.ftimeOldestUpdRefs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_QUEUE_STATISTICSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_QUEUE_STATISTICSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_QUEUE_STATISTICSW").field("ftimeCurrentOpStarted", &self.ftimeCurrentOpStarted).field("cNumPendingOps", &self.cNumPendingOps).field("ftimeOldestSync", &self.ftimeOldestSync).field("ftimeOldestAdd", &self.ftimeOldestAdd).field("ftimeOldestMod", &self.ftimeOldestMod).field("ftimeOldestDel", &self.ftimeOldestDel).field("ftimeOldestUpdRefs", &self.ftimeOldestUpdRefs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_VALUE_META_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_VALUE_META_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttributeName == other.pszAttributeName && self.pszObjectDn == other.pszObjectDn && self.cbData == other.cbData && self.pbData == other.pbData && self.ftimeDeleted == other.ftimeDeleted && self.ftimeCreated == other.ftimeCreated && self.dwVersion == other.dwVersion && self.ftimeLastOriginatingChange == other.ftimeLastOriginatingChange && self.uuidLastOriginatingDsaInvocationID == other.uuidLastOriginatingDsaInvocationID && self.usnOriginatingChange == other.usnOriginatingChange && self.usnLocalChange == other.usnLocalChange
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_VALUE_META_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_VALUE_META_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_VALUE_META_DATA")
            .field("pszAttributeName", &self.pszAttributeName)
            .field("pszObjectDn", &self.pszObjectDn)
            .field("cbData", &self.cbData)
            .field("pbData", &self.pbData)
            .field("ftimeDeleted", &self.ftimeDeleted)
            .field("ftimeCreated", &self.ftimeCreated)
            .field("dwVersion", &self.dwVersion)
            .field("ftimeLastOriginatingChange", &self.ftimeLastOriginatingChange)
            .field("uuidLastOriginatingDsaInvocationID", &self.uuidLastOriginatingDsaInvocationID)
            .field("usnOriginatingChange", &self.usnOriginatingChange)
            .field("usnLocalChange", &self.usnLocalChange)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_VALUE_META_DATA_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_VALUE_META_DATA_2 {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttributeName == other.pszAttributeName && self.pszObjectDn == other.pszObjectDn && self.cbData == other.cbData && self.pbData == other.pbData && self.ftimeDeleted == other.ftimeDeleted && self.ftimeCreated == other.ftimeCreated && self.dwVersion == other.dwVersion && self.ftimeLastOriginatingChange == other.ftimeLastOriginatingChange && self.uuidLastOriginatingDsaInvocationID == other.uuidLastOriginatingDsaInvocationID && self.usnOriginatingChange == other.usnOriginatingChange && self.usnLocalChange == other.usnLocalChange && self.pszLastOriginatingDsaDN == other.pszLastOriginatingDsaDN
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_VALUE_META_DATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_VALUE_META_DATA_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_VALUE_META_DATA_2")
            .field("pszAttributeName", &self.pszAttributeName)
            .field("pszObjectDn", &self.pszObjectDn)
            .field("cbData", &self.cbData)
            .field("pbData", &self.pbData)
            .field("ftimeDeleted", &self.ftimeDeleted)
            .field("ftimeCreated", &self.ftimeCreated)
            .field("dwVersion", &self.dwVersion)
            .field("ftimeLastOriginatingChange", &self.ftimeLastOriginatingChange)
            .field("uuidLastOriginatingDsaInvocationID", &self.uuidLastOriginatingDsaInvocationID)
            .field("usnOriginatingChange", &self.usnOriginatingChange)
            .field("usnLocalChange", &self.usnLocalChange)
            .field("pszLastOriginatingDsaDN", &self.pszLastOriginatingDsaDN)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_VALUE_META_DATA_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_VALUE_META_DATA_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.oszAttributeName == other.oszAttributeName && self.oszObjectDn == other.oszObjectDn && self.cbData == other.cbData && self.obData == other.obData && self.ftimeDeleted == other.ftimeDeleted && self.ftimeCreated == other.ftimeCreated && self.dwVersion == other.dwVersion && self.ftimeLastOriginatingChange == other.ftimeLastOriginatingChange && self.uuidLastOriginatingDsaInvocationID == other.uuidLastOriginatingDsaInvocationID && self.usnOriginatingChange == other.usnOriginatingChange && self.usnLocalChange == other.usnLocalChange && self.oszLastOriginatingDsaDN == other.oszLastOriginatingDsaDN
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_VALUE_META_DATA_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_VALUE_META_DATA_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_VALUE_META_DATA_BLOB")
            .field("oszAttributeName", &self.oszAttributeName)
            .field("oszObjectDn", &self.oszObjectDn)
            .field("cbData", &self.cbData)
            .field("obData", &self.obData)
            .field("ftimeDeleted", &self.ftimeDeleted)
            .field("ftimeCreated", &self.ftimeCreated)
            .field("dwVersion", &self.dwVersion)
            .field("ftimeLastOriginatingChange", &self.ftimeLastOriginatingChange)
            .field("uuidLastOriginatingDsaInvocationID", &self.uuidLastOriginatingDsaInvocationID)
            .field("usnOriginatingChange", &self.usnOriginatingChange)
            .field("usnLocalChange", &self.usnLocalChange)
            .field("oszLastOriginatingDsaDN", &self.oszLastOriginatingDsaDN)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_VALUE_META_DATA_BLOB_EXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_VALUE_META_DATA_BLOB_EXT {
    fn eq(&self, other: &Self) -> bool {
        self.oszAttributeName == other.oszAttributeName && self.oszObjectDn == other.oszObjectDn && self.cbData == other.cbData && self.obData == other.obData && self.ftimeDeleted == other.ftimeDeleted && self.ftimeCreated == other.ftimeCreated && self.dwVersion == other.dwVersion && self.ftimeLastOriginatingChange == other.ftimeLastOriginatingChange && self.uuidLastOriginatingDsaInvocationID == other.uuidLastOriginatingDsaInvocationID && self.usnOriginatingChange == other.usnOriginatingChange && self.usnLocalChange == other.usnLocalChange && self.oszLastOriginatingDsaDN == other.oszLastOriginatingDsaDN && self.dwUserIdentifier == other.dwUserIdentifier && self.dwPriorLinkState == other.dwPriorLinkState && self.dwCurrentLinkState == other.dwCurrentLinkState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_VALUE_META_DATA_BLOB_EXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_VALUE_META_DATA_BLOB_EXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_VALUE_META_DATA_BLOB_EXT")
            .field("oszAttributeName", &self.oszAttributeName)
            .field("oszObjectDn", &self.oszObjectDn)
            .field("cbData", &self.cbData)
            .field("obData", &self.obData)
            .field("ftimeDeleted", &self.ftimeDeleted)
            .field("ftimeCreated", &self.ftimeCreated)
            .field("dwVersion", &self.dwVersion)
            .field("ftimeLastOriginatingChange", &self.ftimeLastOriginatingChange)
            .field("uuidLastOriginatingDsaInvocationID", &self.uuidLastOriginatingDsaInvocationID)
            .field("usnOriginatingChange", &self.usnOriginatingChange)
            .field("usnLocalChange", &self.usnLocalChange)
            .field("oszLastOriginatingDsaDN", &self.oszLastOriginatingDsaDN)
            .field("dwUserIdentifier", &self.dwUserIdentifier)
            .field("dwPriorLinkState", &self.dwPriorLinkState)
            .field("dwCurrentLinkState", &self.dwCurrentLinkState)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_VALUE_META_DATA_EXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_VALUE_META_DATA_EXT {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttributeName == other.pszAttributeName && self.pszObjectDn == other.pszObjectDn && self.cbData == other.cbData && self.pbData == other.pbData && self.ftimeDeleted == other.ftimeDeleted && self.ftimeCreated == other.ftimeCreated && self.dwVersion == other.dwVersion && self.ftimeLastOriginatingChange == other.ftimeLastOriginatingChange && self.uuidLastOriginatingDsaInvocationID == other.uuidLastOriginatingDsaInvocationID && self.usnOriginatingChange == other.usnOriginatingChange && self.usnLocalChange == other.usnLocalChange && self.pszLastOriginatingDsaDN == other.pszLastOriginatingDsaDN && self.dwUserIdentifier == other.dwUserIdentifier && self.dwPriorLinkState == other.dwPriorLinkState && self.dwCurrentLinkState == other.dwCurrentLinkState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_VALUE_META_DATA_EXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_VALUE_META_DATA_EXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_VALUE_META_DATA_EXT")
            .field("pszAttributeName", &self.pszAttributeName)
            .field("pszObjectDn", &self.pszObjectDn)
            .field("cbData", &self.cbData)
            .field("pbData", &self.pbData)
            .field("ftimeDeleted", &self.ftimeDeleted)
            .field("ftimeCreated", &self.ftimeCreated)
            .field("dwVersion", &self.dwVersion)
            .field("ftimeLastOriginatingChange", &self.ftimeLastOriginatingChange)
            .field("uuidLastOriginatingDsaInvocationID", &self.uuidLastOriginatingDsaInvocationID)
            .field("usnOriginatingChange", &self.usnOriginatingChange)
            .field("usnLocalChange", &self.usnLocalChange)
            .field("pszLastOriginatingDsaDN", &self.pszLastOriginatingDsaDN)
            .field("dwUserIdentifier", &self.dwUserIdentifier)
            .field("dwPriorLinkState", &self.dwPriorLinkState)
            .field("dwCurrentLinkState", &self.dwCurrentLinkState)
            .finish()
    }
}
impl ::core::default::Default for DS_REPSYNCALL_ERRINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS_REPSYNCALL_ERRINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.pszSvrId == other.pszSvrId && self.error == other.error && self.dwWin32Err == other.dwWin32Err && self.pszSrcId == other.pszSrcId
    }
}
impl ::core::cmp::Eq for DS_REPSYNCALL_ERRINFOA {}
impl ::core::fmt::Debug for DS_REPSYNCALL_ERRINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPSYNCALL_ERRINFOA").field("pszSvrId", &self.pszSvrId).field("error", &self.error).field("dwWin32Err", &self.dwWin32Err).field("pszSrcId", &self.pszSrcId).finish()
    }
}
impl ::core::default::Default for DS_REPSYNCALL_ERRINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS_REPSYNCALL_ERRINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.pszSvrId == other.pszSvrId && self.error == other.error && self.dwWin32Err == other.dwWin32Err && self.pszSrcId == other.pszSrcId
    }
}
impl ::core::cmp::Eq for DS_REPSYNCALL_ERRINFOW {}
impl ::core::fmt::Debug for DS_REPSYNCALL_ERRINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPSYNCALL_ERRINFOW").field("pszSvrId", &self.pszSvrId).field("error", &self.error).field("dwWin32Err", &self.dwWin32Err).field("pszSrcId", &self.pszSrcId).finish()
    }
}
impl ::core::default::Default for DS_REPSYNCALL_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DS_REPSYNCALL_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_REPSYNCALL_ERROR").field(&self.0).finish()
    }
}
impl ::core::default::Default for DS_REPSYNCALL_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DS_REPSYNCALL_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_REPSYNCALL_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for DS_REPSYNCALL_SYNCA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS_REPSYNCALL_SYNCA {
    fn eq(&self, other: &Self) -> bool {
        self.pszSrcId == other.pszSrcId && self.pszDstId == other.pszDstId && self.pszNC == other.pszNC && self.pguidSrc == other.pguidSrc && self.pguidDst == other.pguidDst
    }
}
impl ::core::cmp::Eq for DS_REPSYNCALL_SYNCA {}
impl ::core::fmt::Debug for DS_REPSYNCALL_SYNCA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPSYNCALL_SYNCA").field("pszSrcId", &self.pszSrcId).field("pszDstId", &self.pszDstId).field("pszNC", &self.pszNC).field("pguidSrc", &self.pguidSrc).field("pguidDst", &self.pguidDst).finish()
    }
}
impl ::core::default::Default for DS_REPSYNCALL_SYNCW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS_REPSYNCALL_SYNCW {
    fn eq(&self, other: &Self) -> bool {
        self.pszSrcId == other.pszSrcId && self.pszDstId == other.pszDstId && self.pszNC == other.pszNC && self.pguidSrc == other.pguidSrc && self.pguidDst == other.pguidDst
    }
}
impl ::core::cmp::Eq for DS_REPSYNCALL_SYNCW {}
impl ::core::fmt::Debug for DS_REPSYNCALL_SYNCW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPSYNCALL_SYNCW").field("pszSrcId", &self.pszSrcId).field("pszDstId", &self.pszDstId).field("pszNC", &self.pszNC).field("pguidSrc", &self.pguidSrc).field("pguidDst", &self.pguidDst).finish()
    }
}
impl ::core::default::Default for DS_REPSYNCALL_UPDATEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS_REPSYNCALL_UPDATEA {
    fn eq(&self, other: &Self) -> bool {
        self.event == other.event && self.pErrInfo == other.pErrInfo && self.pSync == other.pSync
    }
}
impl ::core::cmp::Eq for DS_REPSYNCALL_UPDATEA {}
impl ::core::fmt::Debug for DS_REPSYNCALL_UPDATEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPSYNCALL_UPDATEA").field("event", &self.event).field("pErrInfo", &self.pErrInfo).field("pSync", &self.pSync).finish()
    }
}
impl ::core::default::Default for DS_REPSYNCALL_UPDATEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS_REPSYNCALL_UPDATEW {
    fn eq(&self, other: &Self) -> bool {
        self.event == other.event && self.pErrInfo == other.pErrInfo && self.pSync == other.pSync
    }
}
impl ::core::cmp::Eq for DS_REPSYNCALL_UPDATEW {}
impl ::core::fmt::Debug for DS_REPSYNCALL_UPDATEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPSYNCALL_UPDATEW").field("event", &self.event).field("pErrInfo", &self.pErrInfo).field("pSync", &self.pSync).finish()
    }
}
impl ::core::default::Default for DS_SCHEMA_GUID_MAPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS_SCHEMA_GUID_MAPA {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.guidType == other.guidType && self.pName == other.pName
    }
}
impl ::core::cmp::Eq for DS_SCHEMA_GUID_MAPA {}
impl ::core::fmt::Debug for DS_SCHEMA_GUID_MAPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_SCHEMA_GUID_MAPA").field("guid", &self.guid).field("guidType", &self.guidType).field("pName", &self.pName).finish()
    }
}
impl ::core::default::Default for DS_SCHEMA_GUID_MAPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS_SCHEMA_GUID_MAPW {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.guidType == other.guidType && self.pName == other.pName
    }
}
impl ::core::cmp::Eq for DS_SCHEMA_GUID_MAPW {}
impl ::core::fmt::Debug for DS_SCHEMA_GUID_MAPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_SCHEMA_GUID_MAPW").field("guid", &self.guid).field("guidType", &self.guidType).field("pName", &self.pName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DS_SELECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for DS_SELECTION {
    fn eq(&self, other: &Self) -> bool {
        self.pwzName == other.pwzName && self.pwzADsPath == other.pwzADsPath && self.pwzClass == other.pwzClass && self.pwzUPN == other.pwzUPN && self.pvarFetchedAttributes == other.pvarFetchedAttributes && self.flScopeType == other.flScopeType
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for DS_SELECTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for DS_SELECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_SELECTION").field("pwzName", &self.pwzName).field("pwzADsPath", &self.pwzADsPath).field("pwzClass", &self.pwzClass).field("pwzUPN", &self.pwzUPN).field("pvarFetchedAttributes", &self.pvarFetchedAttributes).field("flScopeType", &self.flScopeType).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DS_SELECTION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for DS_SELECTION_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.cItems == other.cItems && self.cFetchedAttributes == other.cFetchedAttributes && self.aDsSelection == other.aDsSelection
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for DS_SELECTION_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for DS_SELECTION_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_SELECTION_LIST").field("cItems", &self.cItems).field("cFetchedAttributes", &self.cFetchedAttributes).field("aDsSelection", &self.aDsSelection).finish()
    }
}
impl ::core::default::Default for DS_SITE_COST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DS_SITE_COST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.errorCode == other.errorCode && self.cost == other.cost
    }
}
impl ::core::cmp::Eq for DS_SITE_COST_INFO {}
impl ::core::fmt::Debug for DS_SITE_COST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_SITE_COST_INFO").field("errorCode", &self.errorCode).field("cost", &self.cost).finish()
    }
}
impl ::core::default::Default for DS_SPN_NAME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DS_SPN_NAME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_SPN_NAME_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DS_SPN_WRITE_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DS_SPN_WRITE_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_SPN_WRITE_OP").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADs").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsADSystemInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsADSystemInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsADSystemInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsADSystemInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsAccessControlEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsAccessControlEntry {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsAccessControlEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsAccessControlEntry").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsAccessControlList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsAccessControlList {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsAccessControlList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsAccessControlList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsAcl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsAcl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsAcl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsAcl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IADsAggregatee {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IADsAggregatee {}
impl ::core::fmt::Debug for IADsAggregatee {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsAggregatee").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IADsAggregator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IADsAggregator {}
impl ::core::fmt::Debug for IADsAggregator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsAggregator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsBackLink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsBackLink {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsBackLink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsBackLink").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsCaseIgnoreList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsCaseIgnoreList {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsCaseIgnoreList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsCaseIgnoreList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsClass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsClass {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsClass").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsClass {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsComputer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsComputer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsComputer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsComputer").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsComputer {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsComputerOperations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsComputerOperations {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsComputerOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsComputerOperations").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsComputerOperations {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsContainer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsContainer").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsDNWithBinary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsDNWithBinary {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsDNWithBinary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsDNWithBinary").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsDNWithString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsDNWithString {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsDNWithString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsDNWithString").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsDeleteOps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsDeleteOps {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsDeleteOps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsDeleteOps").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsDomain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsDomain {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsDomain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsDomain").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsDomain {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsEmail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsEmail {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsEmail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsEmail").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IADsExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IADsExtension {}
impl ::core::fmt::Debug for IADsExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsExtension").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsFaxNumber {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsFaxNumber {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsFaxNumber {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsFaxNumber").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsFileService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsFileService {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsFileService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsFileService").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsFileService {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn HostComputer(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.HostComputer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHostComputer(&self, bstrhostcomputer: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetHostComputer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrhostcomputer)).ok()
    }
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDisplayName(&self, bstrdisplayname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDisplayName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdisplayname)).ok()
    }
    pub unsafe fn Version(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Version)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetVersion(&self, bstrversion: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVersion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrversion)).ok()
    }
    pub unsafe fn ServiceType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ServiceType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetServiceType(&self, lnservicetype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetServiceType)(::windows::core::Vtable::as_raw(self), lnservicetype).ok()
    }
    pub unsafe fn StartType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StartType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStartType(&self, lnstarttype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetStartType)(::windows::core::Vtable::as_raw(self), lnstarttype).ok()
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Path)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPath(&self, bstrpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpath)).ok()
    }
    pub unsafe fn StartupParameters(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StartupParameters)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStartupParameters(&self, bstrstartupparameters: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetStartupParameters)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrstartupparameters)).ok()
    }
    pub unsafe fn ErrorControl(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ErrorControl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetErrorControl(&self, lnerrorcontrol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetErrorControl)(::windows::core::Vtable::as_raw(self), lnerrorcontrol).ok()
    }
    pub unsafe fn LoadOrderGroup(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LoadOrderGroup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLoadOrderGroup(&self, bstrloadordergroup: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLoadOrderGroup)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrloadordergroup)).ok()
    }
    pub unsafe fn ServiceAccountName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ServiceAccountName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetServiceAccountName(&self, bstrserviceaccountname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetServiceAccountName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrserviceaccountname)).ok()
    }
    pub unsafe fn ServiceAccountPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ServiceAccountPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetServiceAccountPath(&self, bstrserviceaccountpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetServiceAccountPath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrserviceaccountpath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Dependencies(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Dependencies)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetDependencies(&self, vdependencies: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDependencies)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vdependencies)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsFileServiceOperations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsFileServiceOperations {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsFileServiceOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsFileServiceOperations").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsFileServiceOperations {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn Status(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Status)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Start)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Continue(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Continue)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetPassword(&self, bstrnewpassword: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPassword)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrnewpassword)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsFileShare {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsFileShare {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsFileShare {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsFileShare").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsFileShare {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsGroup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsGroup {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsHold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsHold {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsHold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsHold").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsLargeInteger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsLargeInteger {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsLargeInteger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsLargeInteger").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsLocality {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsLocality {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsLocality {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsLocality").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsLocality {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsMembers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsMembers {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsMembers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsMembers").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsNameTranslate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsNameTranslate {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsNameTranslate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsNameTranslate").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsNamespaces {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsNamespaces {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsNamespaces {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsNamespaces").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsNamespaces {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsNetAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsNetAddress {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsNetAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsNetAddress").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsO").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsO {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsOU {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsOU {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsOU {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsOU").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsOU {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsObjectOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsObjectOptions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsObjectOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsObjectOptions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsOctetList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsOctetList {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsOctetList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsOctetList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsOpenDSObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsOpenDSObject {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsOpenDSObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsOpenDSObject").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsPath {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsPath {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsPath").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsPathname {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsPathname {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsPathname {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsPathname").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsPostalAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsPostalAddress {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsPostalAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsPostalAddress").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsPrintJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsPrintJob {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsPrintJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsPrintJob").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintJob {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsPrintJobOperations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsPrintJobOperations {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsPrintJobOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsPrintJobOperations").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintJobOperations {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsPrintQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsPrintQueue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsPrintQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsPrintQueue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintQueue {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsPrintQueueOperations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsPrintQueueOperations {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsPrintQueueOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsPrintQueueOperations").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintQueueOperations {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsProperty {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsPropertyEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsPropertyEntry {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsPropertyEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsPropertyEntry").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsPropertyList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsPropertyList {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsPropertyList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsPropertyList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsPropertyValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsPropertyValue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsPropertyValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsPropertyValue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsPropertyValue2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsPropertyValue2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsPropertyValue2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsPropertyValue2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsReplicaPointer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsReplicaPointer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsReplicaPointer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsReplicaPointer").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsResource {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsResource").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsResource {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsSecurityDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsSecurityDescriptor {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsSecurityDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsSecurityDescriptor").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsSecurityUtility {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsSecurityUtility {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsSecurityUtility {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsSecurityUtility").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsService {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsService").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsService {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsServiceOperations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsServiceOperations {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsServiceOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsServiceOperations").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsServiceOperations {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsSession {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsSession").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsSession {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsSyntax {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsSyntax {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsSyntax {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsSyntax").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsSyntax {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsTimestamp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsTimestamp {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsTimestamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsTimestamp").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsTypedName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsTypedName {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsTypedName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsTypedName").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsUser {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsUser").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IADsUser {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsWinNTSystemInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsWinNTSystemInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsWinNTSystemInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsWinNTSystemInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICommonQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommonQuery {}
impl ::core::fmt::Debug for ICommonQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommonQuery").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectoryObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectoryObject {}
impl ::core::fmt::Debug for IDirectoryObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectoryObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectorySchemaMgmt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectorySchemaMgmt {}
impl ::core::fmt::Debug for IDirectorySchemaMgmt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectorySchemaMgmt").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectorySearch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectorySearch {}
impl ::core::fmt::Debug for IDirectorySearch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectorySearch").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDsAdminCreateObj {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDsAdminCreateObj {}
impl ::core::fmt::Debug for IDsAdminCreateObj {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDsAdminCreateObj").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDsAdminNewObj {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDsAdminNewObj {}
impl ::core::fmt::Debug for IDsAdminNewObj {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDsAdminNewObj").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDsAdminNewObjExt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDsAdminNewObjExt {}
impl ::core::fmt::Debug for IDsAdminNewObjExt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDsAdminNewObjExt").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDsAdminNewObjPrimarySite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDsAdminNewObjPrimarySite {}
impl ::core::fmt::Debug for IDsAdminNewObjPrimarySite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDsAdminNewObjPrimarySite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDsAdminNotifyHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDsAdminNotifyHandler {}
impl ::core::fmt::Debug for IDsAdminNotifyHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDsAdminNotifyHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDsBrowseDomainTree {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDsBrowseDomainTree {}
impl ::core::fmt::Debug for IDsBrowseDomainTree {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDsBrowseDomainTree").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDsDisplaySpecifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDsDisplaySpecifier {}
impl ::core::fmt::Debug for IDsDisplaySpecifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDsDisplaySpecifier").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDsObjectPicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDsObjectPicker {}
impl ::core::fmt::Debug for IDsObjectPicker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDsObjectPicker").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDsObjectPickerCredentials {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDsObjectPickerCredentials {}
impl ::core::fmt::Debug for IDsObjectPickerCredentials {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDsObjectPickerCredentials").field(&self.0).finish()
    }
}
impl IDsObjectPickerCredentials {
    pub unsafe fn Initialize(&self, pinitinfo: *mut DSOP_INIT_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pinitinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn InvokeDialog<P0>(&self, hwndparent: P0) -> ::windows::core::Result<super::super::System::Com::IDataObject>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.InvokeDialog)(::windows::core::Vtable::as_raw(self), hwndparent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPersistQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPersistQuery {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPersistQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistQuery").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPersistQuery {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IPrivateDispatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrivateDispatch {}
impl ::core::fmt::Debug for IPrivateDispatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrivateDispatch").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrivateUnknown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrivateUnknown {}
impl ::core::fmt::Debug for IPrivateUnknown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrivateUnknown").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IQueryForm {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IQueryForm {}
impl ::core::fmt::Debug for IQueryForm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQueryForm").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::default::Default for OPENQUERYWINDOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SCHEDULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCHEDULE {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Bandwidth == other.Bandwidth && self.NumberOfSchedules == other.NumberOfSchedules && self.Schedules == other.Schedules
    }
}
impl ::core::cmp::Eq for SCHEDULE {}
impl ::core::fmt::Debug for SCHEDULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCHEDULE").field("Size", &self.Size).field("Bandwidth", &self.Bandwidth).field("NumberOfSchedules", &self.NumberOfSchedules).field("Schedules", &self.Schedules).finish()
    }
}
impl ::core::default::Default for SCHEDULE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCHEDULE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for SCHEDULE_HEADER {}
impl ::core::fmt::Debug for SCHEDULE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCHEDULE_HEADER").field("Type", &self.Type).field("Offset", &self.Offset).finish()
    }
}
