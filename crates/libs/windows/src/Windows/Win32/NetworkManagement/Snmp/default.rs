#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AsnAny {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for AsnObjectIdentifier {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AsnOctetString {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SNMP_API_TRANSLATE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SNMP_API_TRANSLATE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_API_TRANSLATE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SNMP_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SNMP_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_ERROR").field(&self.0).finish()
    }
}
impl ::core::default::Default for SNMP_ERROR_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SNMP_ERROR_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_ERROR_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SNMP_EXTENSION_REQUEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SNMP_EXTENSION_REQUEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_EXTENSION_REQUEST_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SNMP_GENERICTRAP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SNMP_GENERICTRAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_GENERICTRAP").field(&self.0).finish()
    }
}
impl ::core::default::Default for SNMP_LOG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SNMP_LOG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_LOG").field(&self.0).finish()
    }
}
impl ::core::default::Default for SNMP_OUTPUT_LOG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SNMP_OUTPUT_LOG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_OUTPUT_LOG_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SNMP_PDU_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SNMP_PDU_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_PDU_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SNMP_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SNMP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_STATUS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SnmpVarBind {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SnmpVarBindList {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for smiCNTR64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for smiCNTR64 {
    fn eq(&self, other: &Self) -> bool {
        self.hipart == other.hipart && self.lopart == other.lopart
    }
}
impl ::core::cmp::Eq for smiCNTR64 {}
impl ::core::fmt::Debug for smiCNTR64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("smiCNTR64").field("hipart", &self.hipart).field("lopart", &self.lopart).finish()
    }
}
impl ::core::default::Default for smiOCTETS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for smiOCTETS {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.ptr == other.ptr
    }
}
impl ::core::cmp::Eq for smiOCTETS {}
impl ::core::fmt::Debug for smiOCTETS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("smiOCTETS").field("len", &self.len).field("ptr", &self.ptr).finish()
    }
}
impl ::core::default::Default for smiOID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for smiOID {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.ptr == other.ptr
    }
}
impl ::core::cmp::Eq for smiOID {}
impl ::core::fmt::Debug for smiOID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("smiOID").field("len", &self.len).field("ptr", &self.ptr).finish()
    }
}
impl ::core::default::Default for smiVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for smiVENDORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for smiVENDORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.vendorName == other.vendorName && self.vendorContact == other.vendorContact && self.vendorVersionId == other.vendorVersionId && self.vendorVersionDate == other.vendorVersionDate && self.vendorEnterprise == other.vendorEnterprise
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for smiVENDORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for smiVENDORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("smiVENDORINFO").field("vendorName", &self.vendorName).field("vendorContact", &self.vendorContact).field("vendorVersionId", &self.vendorVersionId).field("vendorVersionDate", &self.vendorVersionDate).field("vendorEnterprise", &self.vendorEnterprise).finish()
    }
}
