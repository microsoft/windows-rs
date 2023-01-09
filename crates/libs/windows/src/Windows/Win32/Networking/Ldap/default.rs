impl ::core::default::Default for BerElement {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BerElement {
    fn eq(&self, other: &Self) -> bool {
        self.opaque == other.opaque
    }
}
impl ::core::cmp::Eq for BerElement {}
impl ::core::fmt::Debug for BerElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BerElement").field("opaque", &self.opaque).finish()
    }
}
impl ::core::default::Default for LDAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LDAP {
    fn eq(&self, other: &Self) -> bool {
        self.ld_sb == other.ld_sb && self.ld_host == other.ld_host && self.ld_version == other.ld_version && self.ld_lberoptions == other.ld_lberoptions && self.ld_deref == other.ld_deref && self.ld_timelimit == other.ld_timelimit && self.ld_sizelimit == other.ld_sizelimit && self.ld_errno == other.ld_errno && self.ld_matched == other.ld_matched && self.ld_error == other.ld_error && self.ld_msgid == other.ld_msgid && self.Reserved3 == other.Reserved3 && self.ld_cldaptries == other.ld_cldaptries && self.ld_cldaptimeout == other.ld_cldaptimeout && self.ld_refhoplimit == other.ld_refhoplimit && self.ld_options == other.ld_options
    }
}
impl ::core::cmp::Eq for LDAP {}
impl ::core::fmt::Debug for LDAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAP")
            .field("ld_sb", &self.ld_sb)
            .field("ld_host", &self.ld_host)
            .field("ld_version", &self.ld_version)
            .field("ld_lberoptions", &self.ld_lberoptions)
            .field("ld_deref", &self.ld_deref)
            .field("ld_timelimit", &self.ld_timelimit)
            .field("ld_sizelimit", &self.ld_sizelimit)
            .field("ld_errno", &self.ld_errno)
            .field("ld_matched", &self.ld_matched)
            .field("ld_error", &self.ld_error)
            .field("ld_msgid", &self.ld_msgid)
            .field("Reserved3", &self.Reserved3)
            .field("ld_cldaptries", &self.ld_cldaptries)
            .field("ld_cldaptimeout", &self.ld_cldaptimeout)
            .field("ld_refhoplimit", &self.ld_refhoplimit)
            .field("ld_options", &self.ld_options)
            .finish()
    }
}
impl ::core::default::Default for LDAP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LDAP_0 {
    fn eq(&self, other: &Self) -> bool {
        self.sb_sd == other.sb_sd && self.Reserved1 == other.Reserved1 && self.sb_naddr == other.sb_naddr && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for LDAP_0 {}
impl ::core::fmt::Debug for LDAP_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAP_0").field("sb_sd", &self.sb_sd).field("Reserved1", &self.Reserved1).field("sb_naddr", &self.sb_naddr).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::default::Default for LDAPAPIFeatureInfoA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LDAPAPIFeatureInfoA {
    fn eq(&self, other: &Self) -> bool {
        self.ldapaif_info_version == other.ldapaif_info_version && self.ldapaif_name == other.ldapaif_name && self.ldapaif_version == other.ldapaif_version
    }
}
impl ::core::cmp::Eq for LDAPAPIFeatureInfoA {}
impl ::core::fmt::Debug for LDAPAPIFeatureInfoA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPAPIFeatureInfoA").field("ldapaif_info_version", &self.ldapaif_info_version).field("ldapaif_name", &self.ldapaif_name).field("ldapaif_version", &self.ldapaif_version).finish()
    }
}
impl ::core::default::Default for LDAPAPIFeatureInfoW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LDAPAPIFeatureInfoW {
    fn eq(&self, other: &Self) -> bool {
        self.ldapaif_info_version == other.ldapaif_info_version && self.ldapaif_name == other.ldapaif_name && self.ldapaif_version == other.ldapaif_version
    }
}
impl ::core::cmp::Eq for LDAPAPIFeatureInfoW {}
impl ::core::fmt::Debug for LDAPAPIFeatureInfoW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPAPIFeatureInfoW").field("ldapaif_info_version", &self.ldapaif_info_version).field("ldapaif_name", &self.ldapaif_name).field("ldapaif_version", &self.ldapaif_version).finish()
    }
}
impl ::core::default::Default for LDAPAPIInfoA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LDAPAPIInfoA {
    fn eq(&self, other: &Self) -> bool {
        self.ldapai_info_version == other.ldapai_info_version && self.ldapai_api_version == other.ldapai_api_version && self.ldapai_protocol_version == other.ldapai_protocol_version && self.ldapai_extensions == other.ldapai_extensions && self.ldapai_vendor_name == other.ldapai_vendor_name && self.ldapai_vendor_version == other.ldapai_vendor_version
    }
}
impl ::core::cmp::Eq for LDAPAPIInfoA {}
impl ::core::fmt::Debug for LDAPAPIInfoA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPAPIInfoA").field("ldapai_info_version", &self.ldapai_info_version).field("ldapai_api_version", &self.ldapai_api_version).field("ldapai_protocol_version", &self.ldapai_protocol_version).field("ldapai_extensions", &self.ldapai_extensions).field("ldapai_vendor_name", &self.ldapai_vendor_name).field("ldapai_vendor_version", &self.ldapai_vendor_version).finish()
    }
}
impl ::core::default::Default for LDAPAPIInfoW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LDAPAPIInfoW {
    fn eq(&self, other: &Self) -> bool {
        self.ldapai_info_version == other.ldapai_info_version && self.ldapai_api_version == other.ldapai_api_version && self.ldapai_protocol_version == other.ldapai_protocol_version && self.ldapai_extensions == other.ldapai_extensions && self.ldapai_vendor_name == other.ldapai_vendor_name && self.ldapai_vendor_version == other.ldapai_vendor_version
    }
}
impl ::core::cmp::Eq for LDAPAPIInfoW {}
impl ::core::fmt::Debug for LDAPAPIInfoW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPAPIInfoW").field("ldapai_info_version", &self.ldapai_info_version).field("ldapai_api_version", &self.ldapai_api_version).field("ldapai_protocol_version", &self.ldapai_protocol_version).field("ldapai_extensions", &self.ldapai_extensions).field("ldapai_vendor_name", &self.ldapai_vendor_name).field("ldapai_vendor_version", &self.ldapai_vendor_version).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LDAPControlA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LDAPControlA {
    fn eq(&self, other: &Self) -> bool {
        self.ldctl_oid == other.ldctl_oid && self.ldctl_value == other.ldctl_value && self.ldctl_iscritical == other.ldctl_iscritical
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LDAPControlA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LDAPControlA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPControlA").field("ldctl_oid", &self.ldctl_oid).field("ldctl_value", &self.ldctl_value).field("ldctl_iscritical", &self.ldctl_iscritical).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LDAPControlW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LDAPControlW {
    fn eq(&self, other: &Self) -> bool {
        self.ldctl_oid == other.ldctl_oid && self.ldctl_value == other.ldctl_value && self.ldctl_iscritical == other.ldctl_iscritical
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LDAPControlW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LDAPControlW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPControlW").field("ldctl_oid", &self.ldctl_oid).field("ldctl_value", &self.ldctl_value).field("ldctl_iscritical", &self.ldctl_iscritical).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LDAPMessage {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LDAPMessage {
    fn eq(&self, other: &Self) -> bool {
        self.lm_msgid == other.lm_msgid && self.lm_msgtype == other.lm_msgtype && self.lm_ber == other.lm_ber && self.lm_chain == other.lm_chain && self.lm_next == other.lm_next && self.lm_time == other.lm_time && self.Connection == other.Connection && self.Request == other.Request && self.lm_returncode == other.lm_returncode && self.lm_referral == other.lm_referral && self.lm_chased == other.lm_chased && self.lm_eom == other.lm_eom && self.ConnectionReferenced == other.ConnectionReferenced
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LDAPMessage {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LDAPMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPMessage")
            .field("lm_msgid", &self.lm_msgid)
            .field("lm_msgtype", &self.lm_msgtype)
            .field("lm_ber", &self.lm_ber)
            .field("lm_chain", &self.lm_chain)
            .field("lm_next", &self.lm_next)
            .field("lm_time", &self.lm_time)
            .field("Connection", &self.Connection)
            .field("Request", &self.Request)
            .field("lm_returncode", &self.lm_returncode)
            .field("lm_referral", &self.lm_referral)
            .field("lm_chased", &self.lm_chased)
            .field("lm_eom", &self.lm_eom)
            .field("ConnectionReferenced", &self.ConnectionReferenced)
            .finish()
    }
}
impl ::core::default::Default for LDAPModA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LDAPModW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LDAPSortKeyA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LDAPSortKeyA {
    fn eq(&self, other: &Self) -> bool {
        self.sk_attrtype == other.sk_attrtype && self.sk_matchruleoid == other.sk_matchruleoid && self.sk_reverseorder == other.sk_reverseorder
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LDAPSortKeyA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LDAPSortKeyA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPSortKeyA").field("sk_attrtype", &self.sk_attrtype).field("sk_matchruleoid", &self.sk_matchruleoid).field("sk_reverseorder", &self.sk_reverseorder).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LDAPSortKeyW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LDAPSortKeyW {
    fn eq(&self, other: &Self) -> bool {
        self.sk_attrtype == other.sk_attrtype && self.sk_matchruleoid == other.sk_matchruleoid && self.sk_reverseorder == other.sk_reverseorder
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LDAPSortKeyW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LDAPSortKeyW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPSortKeyW").field("sk_attrtype", &self.sk_attrtype).field("sk_matchruleoid", &self.sk_matchruleoid).field("sk_reverseorder", &self.sk_reverseorder).finish()
    }
}
impl ::core::default::Default for LDAPVLVInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LDAPVLVInfo {
    fn eq(&self, other: &Self) -> bool {
        self.ldvlv_version == other.ldvlv_version && self.ldvlv_before_count == other.ldvlv_before_count && self.ldvlv_after_count == other.ldvlv_after_count && self.ldvlv_offset == other.ldvlv_offset && self.ldvlv_count == other.ldvlv_count && self.ldvlv_attrvalue == other.ldvlv_attrvalue && self.ldvlv_context == other.ldvlv_context && self.ldvlv_extradata == other.ldvlv_extradata
    }
}
impl ::core::cmp::Eq for LDAPVLVInfo {}
impl ::core::fmt::Debug for LDAPVLVInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPVLVInfo").field("ldvlv_version", &self.ldvlv_version).field("ldvlv_before_count", &self.ldvlv_before_count).field("ldvlv_after_count", &self.ldvlv_after_count).field("ldvlv_offset", &self.ldvlv_offset).field("ldvlv_count", &self.ldvlv_count).field("ldvlv_attrvalue", &self.ldvlv_attrvalue).field("ldvlv_context", &self.ldvlv_context).field("ldvlv_extradata", &self.ldvlv_extradata).finish()
    }
}
impl ::core::default::Default for LDAP_BERVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LDAP_BERVAL {
    fn eq(&self, other: &Self) -> bool {
        self.bv_len == other.bv_len && self.bv_val == other.bv_val
    }
}
impl ::core::cmp::Eq for LDAP_BERVAL {}
impl ::core::fmt::Debug for LDAP_BERVAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAP_BERVAL").field("bv_len", &self.bv_len).field("bv_val", &self.bv_val).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LDAP_REFERRAL_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LDAP_RETCODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LDAP_RETCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LDAP_RETCODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for LDAP_TIMEVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LDAP_TIMEVAL {
    fn eq(&self, other: &Self) -> bool {
        self.tv_sec == other.tv_sec && self.tv_usec == other.tv_usec
    }
}
impl ::core::cmp::Eq for LDAP_TIMEVAL {}
impl ::core::fmt::Debug for LDAP_TIMEVAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAP_TIMEVAL").field("tv_sec", &self.tv_sec).field("tv_usec", &self.tv_usec).finish()
    }
}
impl ::core::default::Default for LDAP_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LDAP_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lv_size == other.lv_size && self.lv_major == other.lv_major && self.lv_minor == other.lv_minor
    }
}
impl ::core::cmp::Eq for LDAP_VERSION_INFO {}
impl ::core::fmt::Debug for LDAP_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAP_VERSION_INFO").field("lv_size", &self.lv_size).field("lv_major", &self.lv_major).field("lv_minor", &self.lv_minor).finish()
    }
}
