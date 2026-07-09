pub const ADSIPROP_ADSIFLAG: ADS_PREFERENCES_ENUM = 12;
pub const ADSIPROP_ASYNCHRONOUS: ADS_PREFERENCES_ENUM = 0;
pub const ADSIPROP_ATTRIBTYPES_ONLY: ADS_PREFERENCES_ENUM = 4;
pub const ADSIPROP_CACHE_RESULTS: ADS_PREFERENCES_ENUM = 11;
pub const ADSIPROP_CHASE_REFERRALS: ADS_PREFERENCES_ENUM = 9;
pub const ADSIPROP_DEREF_ALIASES: ADS_PREFERENCES_ENUM = 1;
pub const ADSIPROP_PAGED_TIME_LIMIT: ADS_PREFERENCES_ENUM = 8;
pub const ADSIPROP_PAGESIZE: ADS_PREFERENCES_ENUM = 7;
pub const ADSIPROP_SEARCH_SCOPE: ADS_PREFERENCES_ENUM = 5;
pub const ADSIPROP_SIZE_LIMIT: ADS_PREFERENCES_ENUM = 2;
pub const ADSIPROP_SORT_ON: ADS_PREFERENCES_ENUM = 10;
pub const ADSIPROP_TIMEOUT: ADS_PREFERENCES_ENUM = 6;
pub const ADSIPROP_TIME_LIMIT: ADS_PREFERENCES_ENUM = 3;
pub type ADSI_DIALECT_ENUM = i32;
pub const ADSI_DIALECT_LDAP: ADSI_DIALECT_ENUM = 0;
pub const ADSI_DIALECT_SQL: ADSI_DIALECT_ENUM = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ADSTYPE(pub ADSTYPEENUM);
pub type ADSTYPEENUM = i32;
pub const ADSTYPE_BACKLINK: ADSTYPEENUM = 18;
pub const ADSTYPE_BOOLEAN: ADSTYPEENUM = 6;
pub const ADSTYPE_CASEIGNORE_LIST: ADSTYPEENUM = 13;
pub const ADSTYPE_CASE_EXACT_STRING: ADSTYPEENUM = 2;
pub const ADSTYPE_CASE_IGNORE_STRING: ADSTYPEENUM = 3;
pub const ADSTYPE_DN_STRING: ADSTYPEENUM = 1;
pub const ADSTYPE_DN_WITH_BINARY: ADSTYPEENUM = 27;
pub const ADSTYPE_DN_WITH_STRING: ADSTYPEENUM = 28;
pub const ADSTYPE_EMAIL: ADSTYPEENUM = 24;
pub const ADSTYPE_FAXNUMBER: ADSTYPEENUM = 23;
pub const ADSTYPE_HOLD: ADSTYPEENUM = 20;
pub const ADSTYPE_INTEGER: ADSTYPEENUM = 7;
pub const ADSTYPE_INVALID: ADSTYPEENUM = 0;
pub const ADSTYPE_LARGE_INTEGER: ADSTYPEENUM = 10;
pub const ADSTYPE_NETADDRESS: ADSTYPEENUM = 21;
pub const ADSTYPE_NT_SECURITY_DESCRIPTOR: ADSTYPEENUM = 25;
pub const ADSTYPE_NUMERIC_STRING: ADSTYPEENUM = 5;
pub const ADSTYPE_OBJECT_CLASS: ADSTYPEENUM = 12;
pub const ADSTYPE_OCTET_LIST: ADSTYPEENUM = 14;
pub const ADSTYPE_OCTET_STRING: ADSTYPEENUM = 8;
pub const ADSTYPE_PATH: ADSTYPEENUM = 15;
pub const ADSTYPE_POSTALADDRESS: ADSTYPEENUM = 16;
pub const ADSTYPE_PRINTABLE_STRING: ADSTYPEENUM = 4;
pub const ADSTYPE_PROV_SPECIFIC: ADSTYPEENUM = 11;
pub const ADSTYPE_REPLICAPOINTER: ADSTYPEENUM = 22;
pub const ADSTYPE_TIMESTAMP: ADSTYPEENUM = 17;
pub const ADSTYPE_TYPEDNAME: ADSTYPEENUM = 19;
pub const ADSTYPE_UNKNOWN: ADSTYPEENUM = 26;
pub const ADSTYPE_UTC_TIME: ADSTYPEENUM = 9;
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub struct ADSVALUE {
    pub dwType: ADSTYPE,
    pub Anonymous: ADSVALUE_0,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
impl Default for ADSVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub union ADSVALUE_0 {
    pub DNString: ADS_DN_STRING,
    pub CaseExactString: ADS_CASE_EXACT_STRING,
    pub CaseIgnoreString: ADS_CASE_IGNORE_STRING,
    pub PrintableString: ADS_PRINTABLE_STRING,
    pub NumericString: ADS_NUMERIC_STRING,
    pub Boolean: ADS_BOOLEAN,
    pub Integer: ADS_INTEGER,
    pub OctetString: ADS_OCTET_STRING,
    pub UTCTime: ADS_UTC_TIME,
    pub LargeInteger: ADS_LARGE_INTEGER,
    pub ClassName: ADS_OBJECT_CLASS,
    pub ProviderSpecific: ADS_PROV_SPECIFIC,
    pub pCaseIgnoreList: PADS_CASEIGNORE_LIST,
    pub pOctetList: PADS_OCTET_LIST,
    pub pPath: PADS_PATH,
    pub pPostalAddress: PADS_POSTALADDRESS,
    pub Timestamp: ADS_TIMESTAMP,
    pub BackLink: ADS_BACKLINK,
    pub pTypedName: PADS_TYPEDNAME,
    pub Hold: ADS_HOLD,
    pub pNetAddress: PADS_NETADDRESS,
    pub pReplicaPointer: PADS_REPLICAPOINTER,
    pub pFaxNumber: PADS_FAXNUMBER,
    pub Email: ADS_EMAIL,
    pub SecurityDescriptor: ADS_NT_SECURITY_DESCRIPTOR,
    pub pDNWithBinary: PADS_DN_WITH_BINARY,
    pub pDNWithString: PADS_DN_WITH_STRING,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
impl Default for ADSVALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ADS_ACEFLAG_ENUM = i32;
pub const ADS_ACEFLAG_FAILED_ACCESS: ADS_ACEFLAG_ENUM = 128;
pub const ADS_ACEFLAG_INHERITED_ACE: ADS_ACEFLAG_ENUM = 16;
pub const ADS_ACEFLAG_INHERIT_ACE: ADS_ACEFLAG_ENUM = 2;
pub const ADS_ACEFLAG_INHERIT_ONLY_ACE: ADS_ACEFLAG_ENUM = 8;
pub const ADS_ACEFLAG_NO_PROPAGATE_INHERIT_ACE: ADS_ACEFLAG_ENUM = 4;
pub const ADS_ACEFLAG_SUCCESSFUL_ACCESS: ADS_ACEFLAG_ENUM = 64;
pub const ADS_ACEFLAG_VALID_INHERIT_FLAGS: ADS_ACEFLAG_ENUM = 31;
pub const ADS_ACETYPE_ACCESS_ALLOWED: ADS_ACETYPE_ENUM = 0;
pub const ADS_ACETYPE_ACCESS_ALLOWED_CALLBACK: ADS_ACETYPE_ENUM = 9;
pub const ADS_ACETYPE_ACCESS_ALLOWED_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = 11;
pub const ADS_ACETYPE_ACCESS_ALLOWED_OBJECT: ADS_ACETYPE_ENUM = 5;
pub const ADS_ACETYPE_ACCESS_DENIED: ADS_ACETYPE_ENUM = 1;
pub const ADS_ACETYPE_ACCESS_DENIED_CALLBACK: ADS_ACETYPE_ENUM = 10;
pub const ADS_ACETYPE_ACCESS_DENIED_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = 12;
pub const ADS_ACETYPE_ACCESS_DENIED_OBJECT: ADS_ACETYPE_ENUM = 6;
pub type ADS_ACETYPE_ENUM = i32;
pub const ADS_ACETYPE_SYSTEM_ALARM_CALLBACK: ADS_ACETYPE_ENUM = 14;
pub const ADS_ACETYPE_SYSTEM_ALARM_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = 16;
pub const ADS_ACETYPE_SYSTEM_ALARM_OBJECT: ADS_ACETYPE_ENUM = 8;
pub const ADS_ACETYPE_SYSTEM_AUDIT: ADS_ACETYPE_ENUM = 2;
pub const ADS_ACETYPE_SYSTEM_AUDIT_CALLBACK: ADS_ACETYPE_ENUM = 13;
pub const ADS_ACETYPE_SYSTEM_AUDIT_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = 15;
pub const ADS_ACETYPE_SYSTEM_AUDIT_OBJECT: ADS_ACETYPE_ENUM = 7;
pub const ADS_ATTR_APPEND: u32 = 3;
pub const ADS_ATTR_CLEAR: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_ATTR_DEF {
    pub pszAttrName: windows_core::PWSTR,
    pub dwADsType: ADSTYPE,
    pub dwMinRange: u32,
    pub dwMaxRange: u32,
    pub fMultiValued: windows_core::BOOL,
}
pub const ADS_ATTR_DELETE: u32 = 4;
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_ATTR_INFO {
    pub pszAttrName: windows_core::PWSTR,
    pub dwControlCode: u32,
    pub dwADsType: ADSTYPE,
    pub pADsValues: PADSVALUE,
    pub dwNumValues: u32,
}
pub const ADS_ATTR_UPDATE: u32 = 2;
pub type ADS_AUTHENTICATION_ENUM = i32;
pub const ADS_AUTH_RESERVED: ADS_AUTHENTICATION_ENUM = -2147483648;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_BACKLINK {
    pub RemoteID: u32,
    pub ObjectName: windows_core::PWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ADS_BOOLEAN(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ADS_CASEIGNORE_LIST {
    pub Next: *mut Self,
    pub String: windows_core::PWSTR,
}
impl Default for ADS_CASEIGNORE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ADS_CASE_EXACT_STRING = windows_core::PWSTR;
pub type ADS_CASE_IGNORE_STRING = windows_core::PWSTR;
pub const ADS_CHASE_REFERRALS_ALWAYS: ADS_CHASE_REFERRALS_ENUM = 96;
pub type ADS_CHASE_REFERRALS_ENUM = i32;
pub const ADS_CHASE_REFERRALS_EXTERNAL: ADS_CHASE_REFERRALS_ENUM = 64;
pub const ADS_CHASE_REFERRALS_NEVER: ADS_CHASE_REFERRALS_ENUM = 0;
pub const ADS_CHASE_REFERRALS_SUBORDINATE: ADS_CHASE_REFERRALS_ENUM = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ADS_CLASS_DEF {
    pub pszClassName: windows_core::PWSTR,
    pub dwMandatoryAttrs: u32,
    pub ppszMandatoryAttrs: *mut windows_core::PWSTR,
    pub optionalAttrs: u32,
    pub ppszOptionalAttrs: *mut *mut windows_core::PWSTR,
    pub dwNamingAttrs: u32,
    pub ppszNamingAttrs: *mut *mut windows_core::PWSTR,
    pub dwSuperClasses: u32,
    pub ppszSuperClasses: *mut *mut windows_core::PWSTR,
    pub fIsContainer: windows_core::BOOL,
}
impl Default for ADS_CLASS_DEF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ADS_DEREFENUM = i32;
pub const ADS_DEREF_ALWAYS: ADS_DEREFENUM = 3;
pub const ADS_DEREF_FINDING: ADS_DEREFENUM = 2;
pub const ADS_DEREF_NEVER: ADS_DEREFENUM = 0;
pub const ADS_DEREF_SEARCHING: ADS_DEREFENUM = 1;
pub const ADS_DIRSYNC_COOKIE: windows_core::PCWSTR = windows_core::w!("fc8cb04d-311d-406c-8cb9-1ae8b843b418");
pub type ADS_DISPLAY_ENUM = i32;
pub const ADS_DISPLAY_FULL: ADS_DISPLAY_ENUM = 1;
pub const ADS_DISPLAY_VALUE_ONLY: ADS_DISPLAY_ENUM = 2;
pub type ADS_DN_STRING = windows_core::PWSTR;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_DN_WITH_BINARY {
    pub dwLength: u32,
    pub lpBinaryValue: super::minwindef::LPBYTE,
    pub pszDNString: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_DN_WITH_STRING {
    pub pszStringValue: windows_core::PWSTR,
    pub pszDNString: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_EMAIL {
    pub Address: windows_core::PWSTR,
    pub Type: u32,
}
pub const ADS_ESCAPEDMODE_DEFAULT: ADS_ESCAPE_MODE_ENUM = 1;
pub const ADS_ESCAPEDMODE_OFF: ADS_ESCAPE_MODE_ENUM = 3;
pub const ADS_ESCAPEDMODE_OFF_EX: ADS_ESCAPE_MODE_ENUM = 4;
pub const ADS_ESCAPEDMODE_ON: ADS_ESCAPE_MODE_ENUM = 2;
pub type ADS_ESCAPE_MODE_ENUM = i32;
pub const ADS_EXT_INITCREDENTIALS: u32 = 1;
pub const ADS_EXT_INITIALIZE_COMPLETE: u32 = 2;
pub const ADS_EXT_MAXEXTDISPID: u32 = 16777215;
pub const ADS_EXT_MINEXTDISPID: u32 = 1;
pub const ADS_FAST_BIND: ADS_AUTHENTICATION_ENUM = 32;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_FAXNUMBER {
    pub TelephoneNumber: windows_core::PWSTR,
    pub NumberOfBits: u32,
    pub Parameters: super::minwindef::LPBYTE,
}
pub type ADS_FLAGTYPE_ENUM = i32;
pub const ADS_FLAG_INHERITED_OBJECT_TYPE_PRESENT: ADS_FLAGTYPE_ENUM = 2;
pub const ADS_FLAG_OBJECT_TYPE_PRESENT: ADS_FLAGTYPE_ENUM = 1;
pub type ADS_FORMAT_ENUM = i32;
pub const ADS_FORMAT_LEAF: ADS_FORMAT_ENUM = 11;
pub const ADS_FORMAT_PROVIDER: ADS_FORMAT_ENUM = 10;
pub const ADS_FORMAT_SERVER: ADS_FORMAT_ENUM = 9;
pub const ADS_FORMAT_WINDOWS: ADS_FORMAT_ENUM = 1;
pub const ADS_FORMAT_WINDOWS_DN: ADS_FORMAT_ENUM = 3;
pub const ADS_FORMAT_WINDOWS_NO_SERVER: ADS_FORMAT_ENUM = 2;
pub const ADS_FORMAT_WINDOWS_PARENT: ADS_FORMAT_ENUM = 4;
pub const ADS_FORMAT_X500: ADS_FORMAT_ENUM = 5;
pub const ADS_FORMAT_X500_DN: ADS_FORMAT_ENUM = 7;
pub const ADS_FORMAT_X500_NO_SERVER: ADS_FORMAT_ENUM = 6;
pub const ADS_FORMAT_X500_PARENT: ADS_FORMAT_ENUM = 8;
pub const ADS_GROUP_TYPE_DOMAIN_LOCAL_GROUP: ADS_GROUP_TYPE_ENUM = 4;
pub type ADS_GROUP_TYPE_ENUM = i32;
pub const ADS_GROUP_TYPE_GLOBAL_GROUP: ADS_GROUP_TYPE_ENUM = 2;
pub const ADS_GROUP_TYPE_LOCAL_GROUP: ADS_GROUP_TYPE_ENUM = 4;
pub const ADS_GROUP_TYPE_SECURITY_ENABLED: ADS_GROUP_TYPE_ENUM = -2147483648;
pub const ADS_GROUP_TYPE_UNIVERSAL_GROUP: ADS_GROUP_TYPE_ENUM = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_HOLD {
    pub ObjectName: windows_core::PWSTR,
    pub Amount: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ADS_INTEGER(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ADS_LARGE_INTEGER(pub i64);
pub const ADS_NAME_INITTYPE_DOMAIN: ADS_NAME_INITTYPE_ENUM = 1;
pub type ADS_NAME_INITTYPE_ENUM = i32;
pub const ADS_NAME_INITTYPE_GC: ADS_NAME_INITTYPE_ENUM = 3;
pub const ADS_NAME_INITTYPE_SERVER: ADS_NAME_INITTYPE_ENUM = 2;
pub const ADS_NAME_TYPE_1779: ADS_NAME_TYPE_ENUM = 1;
pub const ADS_NAME_TYPE_CANONICAL: ADS_NAME_TYPE_ENUM = 2;
pub const ADS_NAME_TYPE_CANONICAL_EX: ADS_NAME_TYPE_ENUM = 10;
pub const ADS_NAME_TYPE_DISPLAY: ADS_NAME_TYPE_ENUM = 4;
pub const ADS_NAME_TYPE_DOMAIN_SIMPLE: ADS_NAME_TYPE_ENUM = 5;
pub const ADS_NAME_TYPE_ENTERPRISE_SIMPLE: ADS_NAME_TYPE_ENUM = 6;
pub type ADS_NAME_TYPE_ENUM = i32;
pub const ADS_NAME_TYPE_GUID: ADS_NAME_TYPE_ENUM = 7;
pub const ADS_NAME_TYPE_NT4: ADS_NAME_TYPE_ENUM = 3;
pub const ADS_NAME_TYPE_SERVICE_PRINCIPAL_NAME: ADS_NAME_TYPE_ENUM = 11;
pub const ADS_NAME_TYPE_SID_OR_SID_HISTORY_NAME: ADS_NAME_TYPE_ENUM = 12;
pub const ADS_NAME_TYPE_UNKNOWN: ADS_NAME_TYPE_ENUM = 8;
pub const ADS_NAME_TYPE_USER_PRINCIPAL_NAME: ADS_NAME_TYPE_ENUM = 9;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ADS_NETADDRESS {
    pub AddressType: u32,
    pub AddressLength: u32,
    pub Address: *mut u8,
}
impl Default for ADS_NETADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ADS_NO_AUTHENTICATION: ADS_AUTHENTICATION_ENUM = 16;
pub const ADS_NO_REFERRAL_CHASING: ADS_AUTHENTICATION_ENUM = 1024;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_NT_SECURITY_DESCRIPTOR {
    pub dwLength: u32,
    pub lpValue: super::minwindef::LPBYTE,
}
pub type ADS_NUMERIC_STRING = windows_core::PWSTR;
pub type ADS_OBJECT_CLASS = windows_core::PWSTR;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_OBJECT_INFO {
    pub pszRDN: windows_core::PWSTR,
    pub pszObjectDN: windows_core::PWSTR,
    pub pszParentDN: windows_core::PWSTR,
    pub pszSchemaDN: windows_core::PWSTR,
    pub pszClassName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ADS_OCTET_LIST {
    pub Next: *mut Self,
    pub Length: u32,
    pub Data: *mut u8,
}
impl Default for ADS_OCTET_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_OCTET_STRING {
    pub dwLength: u32,
    pub lpValue: super::minwindef::LPBYTE,
}
pub const ADS_OPTION_ACCUMULATIVE_MODIFICATION: ADS_OPTION_ENUM = 8;
pub type ADS_OPTION_ENUM = i32;
pub const ADS_OPTION_MUTUAL_AUTH_STATUS: ADS_OPTION_ENUM = 4;
pub const ADS_OPTION_PAGE_SIZE: ADS_OPTION_ENUM = 2;
pub const ADS_OPTION_PASSWORD_METHOD: ADS_OPTION_ENUM = 7;
pub const ADS_OPTION_PASSWORD_PORTNUMBER: ADS_OPTION_ENUM = 6;
pub const ADS_OPTION_QUOTA: ADS_OPTION_ENUM = 5;
pub const ADS_OPTION_REFERRALS: ADS_OPTION_ENUM = 1;
pub const ADS_OPTION_SECURITY_MASK: ADS_OPTION_ENUM = 3;
pub const ADS_OPTION_SERVERNAME: ADS_OPTION_ENUM = 0;
pub const ADS_OPTION_SKIP_SID_LOOKUP: ADS_OPTION_ENUM = 9;
pub const ADS_PASSWORD_ENCODE_CLEAR: ADS_PASSWORD_ENCODING_ENUM = 1;
pub const ADS_PASSWORD_ENCODE_REQUIRE_SSL: ADS_PASSWORD_ENCODING_ENUM = 0;
pub type ADS_PASSWORD_ENCODING_ENUM = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_PATH {
    pub Type: u32,
    pub VolumeName: windows_core::PWSTR,
    pub Path: windows_core::PWSTR,
}
pub type ADS_PATHTYPE_ENUM = i32;
pub const ADS_PATH_FILE: ADS_PATHTYPE_ENUM = 1;
pub const ADS_PATH_FILESHARE: ADS_PATHTYPE_ENUM = 2;
pub const ADS_PATH_REGISTRY: ADS_PATHTYPE_ENUM = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ADS_POSTALADDRESS {
    pub PostalAddress: [windows_core::PWSTR; 6],
}
impl Default for ADS_POSTALADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ADS_PREFERENCES_ENUM = i32;
pub type ADS_PRINTABLE_STRING = windows_core::PWSTR;
pub const ADS_PROMPT_CREDENTIALS: ADS_AUTHENTICATION_ENUM = 8;
pub const ADS_PROPERTY_APPEND: ADS_PROPERTY_OPERATION_ENUM = 3;
pub const ADS_PROPERTY_CLEAR: ADS_PROPERTY_OPERATION_ENUM = 1;
pub const ADS_PROPERTY_DELETE: ADS_PROPERTY_OPERATION_ENUM = 4;
pub type ADS_PROPERTY_OPERATION_ENUM = i32;
pub const ADS_PROPERTY_UPDATE: ADS_PROPERTY_OPERATION_ENUM = 2;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_PROV_SPECIFIC {
    pub dwLength: u32,
    pub lpValue: super::minwindef::LPBYTE,
}
pub const ADS_READONLY_SERVER: ADS_AUTHENTICATION_ENUM = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_REPLICAPOINTER {
    pub ServerName: windows_core::PWSTR,
    pub ReplicaType: u32,
    pub ReplicaNumber: u32,
    pub Count: u32,
    pub ReplicaAddressHints: PADS_NETADDRESS,
}
pub type ADS_RIGHTS_ENUM = i32;
pub const ADS_RIGHT_ACCESS_SYSTEM_SECURITY: ADS_RIGHTS_ENUM = 16777216;
pub const ADS_RIGHT_ACTRL_DS_LIST: ADS_RIGHTS_ENUM = 4;
pub const ADS_RIGHT_DELETE: ADS_RIGHTS_ENUM = 65536;
pub const ADS_RIGHT_DS_CONTROL_ACCESS: ADS_RIGHTS_ENUM = 256;
pub const ADS_RIGHT_DS_CREATE_CHILD: ADS_RIGHTS_ENUM = 1;
pub const ADS_RIGHT_DS_DELETE_CHILD: ADS_RIGHTS_ENUM = 2;
pub const ADS_RIGHT_DS_DELETE_TREE: ADS_RIGHTS_ENUM = 64;
pub const ADS_RIGHT_DS_LIST_OBJECT: ADS_RIGHTS_ENUM = 128;
pub const ADS_RIGHT_DS_READ_PROP: ADS_RIGHTS_ENUM = 16;
pub const ADS_RIGHT_DS_SELF: ADS_RIGHTS_ENUM = 8;
pub const ADS_RIGHT_DS_WRITE_PROP: ADS_RIGHTS_ENUM = 32;
pub const ADS_RIGHT_GENERIC_ALL: ADS_RIGHTS_ENUM = 268435456;
pub const ADS_RIGHT_GENERIC_EXECUTE: ADS_RIGHTS_ENUM = 536870912;
pub const ADS_RIGHT_GENERIC_READ: ADS_RIGHTS_ENUM = -2147483648;
pub const ADS_RIGHT_GENERIC_WRITE: ADS_RIGHTS_ENUM = 1073741824;
pub const ADS_RIGHT_READ_CONTROL: ADS_RIGHTS_ENUM = 131072;
pub const ADS_RIGHT_SYNCHRONIZE: ADS_RIGHTS_ENUM = 1048576;
pub const ADS_RIGHT_WRITE_DAC: ADS_RIGHTS_ENUM = 262144;
pub const ADS_RIGHT_WRITE_OWNER: ADS_RIGHTS_ENUM = 524288;
pub type ADS_SCOPEENUM = i32;
pub const ADS_SCOPE_BASE: ADS_SCOPEENUM = 0;
pub const ADS_SCOPE_ONELEVEL: ADS_SCOPEENUM = 1;
pub const ADS_SCOPE_SUBTREE: ADS_SCOPEENUM = 2;
pub type ADS_SD_CONTROL_ENUM = i32;
pub const ADS_SD_CONTROL_SE_DACL_AUTO_INHERITED: ADS_SD_CONTROL_ENUM = 1024;
pub const ADS_SD_CONTROL_SE_DACL_AUTO_INHERIT_REQ: ADS_SD_CONTROL_ENUM = 256;
pub const ADS_SD_CONTROL_SE_DACL_DEFAULTED: ADS_SD_CONTROL_ENUM = 8;
pub const ADS_SD_CONTROL_SE_DACL_PRESENT: ADS_SD_CONTROL_ENUM = 4;
pub const ADS_SD_CONTROL_SE_DACL_PROTECTED: ADS_SD_CONTROL_ENUM = 4096;
pub const ADS_SD_CONTROL_SE_GROUP_DEFAULTED: ADS_SD_CONTROL_ENUM = 2;
pub const ADS_SD_CONTROL_SE_OWNER_DEFAULTED: ADS_SD_CONTROL_ENUM = 1;
pub const ADS_SD_CONTROL_SE_SACL_AUTO_INHERITED: ADS_SD_CONTROL_ENUM = 2048;
pub const ADS_SD_CONTROL_SE_SACL_AUTO_INHERIT_REQ: ADS_SD_CONTROL_ENUM = 512;
pub const ADS_SD_CONTROL_SE_SACL_DEFAULTED: ADS_SD_CONTROL_ENUM = 32;
pub const ADS_SD_CONTROL_SE_SACL_PRESENT: ADS_SD_CONTROL_ENUM = 16;
pub const ADS_SD_CONTROL_SE_SACL_PROTECTED: ADS_SD_CONTROL_ENUM = 8192;
pub const ADS_SD_CONTROL_SE_SELF_RELATIVE: ADS_SD_CONTROL_ENUM = 32768;
pub type ADS_SD_FORMAT_ENUM = i32;
pub const ADS_SD_FORMAT_HEXSTRING: ADS_SD_FORMAT_ENUM = 3;
pub const ADS_SD_FORMAT_IID: ADS_SD_FORMAT_ENUM = 1;
pub const ADS_SD_FORMAT_RAW: ADS_SD_FORMAT_ENUM = 2;
pub const ADS_SD_REVISION_DS: ADS_SD_REVISION_ENUM = 4;
pub type ADS_SD_REVISION_ENUM = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ADS_SEARCHPREF(pub ADS_SEARCHPREF_ENUM);
pub const ADS_SEARCHPREF_ASYNCHRONOUS: ADS_SEARCHPREF_ENUM = 0;
pub const ADS_SEARCHPREF_ATTRIBTYPES_ONLY: ADS_SEARCHPREF_ENUM = 4;
pub const ADS_SEARCHPREF_ATTRIBUTE_QUERY: ADS_SEARCHPREF_ENUM = 15;
pub const ADS_SEARCHPREF_CACHE_RESULTS: ADS_SEARCHPREF_ENUM = 11;
pub const ADS_SEARCHPREF_CHASE_REFERRALS: ADS_SEARCHPREF_ENUM = 9;
pub const ADS_SEARCHPREF_DEREF_ALIASES: ADS_SEARCHPREF_ENUM = 1;
pub const ADS_SEARCHPREF_DIRSYNC: ADS_SEARCHPREF_ENUM = 12;
pub const ADS_SEARCHPREF_DIRSYNC_FLAG: ADS_SEARCHPREF_ENUM = 17;
pub type ADS_SEARCHPREF_ENUM = i32;
pub const ADS_SEARCHPREF_EXTENDED_DN: ADS_SEARCHPREF_ENUM = 18;
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub struct ADS_SEARCHPREF_INFO {
    pub dwSearchPref: ADS_SEARCHPREF,
    pub vValue: ADSVALUE,
    pub dwStatus: ADS_STATUS,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
impl Default for ADS_SEARCHPREF_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ADS_SEARCHPREF_PAGED_TIME_LIMIT: ADS_SEARCHPREF_ENUM = 8;
pub const ADS_SEARCHPREF_PAGESIZE: ADS_SEARCHPREF_ENUM = 7;
pub const ADS_SEARCHPREF_SEARCH_SCOPE: ADS_SEARCHPREF_ENUM = 5;
pub const ADS_SEARCHPREF_SECURITY_MASK: ADS_SEARCHPREF_ENUM = 16;
pub const ADS_SEARCHPREF_SIZE_LIMIT: ADS_SEARCHPREF_ENUM = 2;
pub const ADS_SEARCHPREF_SORT_ON: ADS_SEARCHPREF_ENUM = 10;
pub const ADS_SEARCHPREF_TIMEOUT: ADS_SEARCHPREF_ENUM = 6;
pub const ADS_SEARCHPREF_TIME_LIMIT: ADS_SEARCHPREF_ENUM = 3;
pub const ADS_SEARCHPREF_TOMBSTONE: ADS_SEARCHPREF_ENUM = 13;
pub const ADS_SEARCHPREF_VLV: ADS_SEARCHPREF_ENUM = 14;
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_SEARCH_COLUMN {
    pub pszAttrName: windows_core::PWSTR,
    pub dwADsType: ADSTYPE,
    pub pADsValues: PADSVALUE,
    pub dwNumValues: u32,
    pub hReserved: super::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ADS_SEARCH_HANDLE(pub super::winnt::HANDLE);
pub const ADS_SECURE_AUTHENTICATION: ADS_AUTHENTICATION_ENUM = 1;
pub const ADS_SECURITY_INFO_DACL: ADS_SECURITY_INFO_ENUM = 4;
pub type ADS_SECURITY_INFO_ENUM = i32;
pub const ADS_SECURITY_INFO_GROUP: ADS_SECURITY_INFO_ENUM = 2;
pub const ADS_SECURITY_INFO_OWNER: ADS_SECURITY_INFO_ENUM = 1;
pub const ADS_SECURITY_INFO_SACL: ADS_SECURITY_INFO_ENUM = 8;
pub const ADS_SERVER_BIND: ADS_AUTHENTICATION_ENUM = 512;
pub const ADS_SETTYPE_DN: ADS_SETTYPE_ENUM = 4;
pub type ADS_SETTYPE_ENUM = i32;
pub const ADS_SETTYPE_FULL: ADS_SETTYPE_ENUM = 1;
pub const ADS_SETTYPE_PROVIDER: ADS_SETTYPE_ENUM = 2;
pub const ADS_SETTYPE_SERVER: ADS_SETTYPE_ENUM = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_SORTKEY {
    pub pszAttrType: windows_core::PWSTR,
    pub pszReserved: windows_core::PWSTR,
    pub fReverseorder: bool,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ADS_STATUS(pub ADS_STATUSENUM);
pub type ADS_STATUSENUM = i32;
pub const ADS_STATUS_INVALID_SEARCHPREF: ADS_STATUSENUM = 1;
pub const ADS_STATUS_INVALID_SEARCHPREFVALUE: ADS_STATUSENUM = 2;
pub const ADS_STATUS_S_OK: ADS_STATUSENUM = 0;
pub const ADS_SYSTEMFLAG_ATTR_IS_CONSTRUCTED: ADS_SYSTEMFLAG_ENUM = 4;
pub const ADS_SYSTEMFLAG_ATTR_NOT_REPLICATED: ADS_SYSTEMFLAG_ENUM = 1;
pub const ADS_SYSTEMFLAG_CONFIG_ALLOW_LIMITED_MOVE: ADS_SYSTEMFLAG_ENUM = 268435456;
pub const ADS_SYSTEMFLAG_CONFIG_ALLOW_MOVE: ADS_SYSTEMFLAG_ENUM = 536870912;
pub const ADS_SYSTEMFLAG_CONFIG_ALLOW_RENAME: ADS_SYSTEMFLAG_ENUM = 1073741824;
pub const ADS_SYSTEMFLAG_CR_NTDS_DOMAIN: ADS_SYSTEMFLAG_ENUM = 2;
pub const ADS_SYSTEMFLAG_CR_NTDS_NC: ADS_SYSTEMFLAG_ENUM = 1;
pub const ADS_SYSTEMFLAG_DISALLOW_DELETE: ADS_SYSTEMFLAG_ENUM = -2147483648;
pub const ADS_SYSTEMFLAG_DOMAIN_DISALLOW_MOVE: ADS_SYSTEMFLAG_ENUM = 67108864;
pub const ADS_SYSTEMFLAG_DOMAIN_DISALLOW_RENAME: ADS_SYSTEMFLAG_ENUM = 134217728;
pub type ADS_SYSTEMFLAG_ENUM = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_TIMESTAMP {
    pub WholeSeconds: u32,
    pub EventID: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_TYPEDNAME {
    pub ObjectName: windows_core::PWSTR,
    pub Level: u32,
    pub Interval: u32,
}
pub const ADS_UF_ACCOUNTDISABLE: ADS_USER_FLAG_ENUM = 2;
pub const ADS_UF_DONT_EXPIRE_PASSWD: ADS_USER_FLAG_ENUM = 65536;
pub const ADS_UF_DONT_REQUIRE_PREAUTH: ADS_USER_FLAG_ENUM = 4194304;
pub const ADS_UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED: ADS_USER_FLAG_ENUM = 128;
pub const ADS_UF_HOMEDIR_REQUIRED: ADS_USER_FLAG_ENUM = 8;
pub const ADS_UF_INTERDOMAIN_TRUST_ACCOUNT: ADS_USER_FLAG_ENUM = 2048;
pub const ADS_UF_LOCKOUT: ADS_USER_FLAG_ENUM = 16;
pub const ADS_UF_MNS_LOGON_ACCOUNT: ADS_USER_FLAG_ENUM = 131072;
pub const ADS_UF_NORMAL_ACCOUNT: ADS_USER_FLAG_ENUM = 512;
pub const ADS_UF_NOT_DELEGATED: ADS_USER_FLAG_ENUM = 1048576;
pub const ADS_UF_PASSWD_CANT_CHANGE: ADS_USER_FLAG_ENUM = 64;
pub const ADS_UF_PASSWD_NOTREQD: ADS_USER_FLAG_ENUM = 32;
pub const ADS_UF_PASSWORD_EXPIRED: ADS_USER_FLAG_ENUM = 8388608;
pub const ADS_UF_SCRIPT: ADS_USER_FLAG_ENUM = 1;
pub const ADS_UF_SERVER_TRUST_ACCOUNT: ADS_USER_FLAG_ENUM = 8192;
pub const ADS_UF_SMARTCARD_REQUIRED: ADS_USER_FLAG_ENUM = 262144;
pub const ADS_UF_TEMP_DUPLICATE_ACCOUNT: ADS_USER_FLAG_ENUM = 256;
pub const ADS_UF_TRUSTED_FOR_DELEGATION: ADS_USER_FLAG_ENUM = 524288;
pub const ADS_UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: ADS_USER_FLAG_ENUM = 16777216;
pub const ADS_UF_USE_DES_KEY_ONLY: ADS_USER_FLAG_ENUM = 2097152;
pub const ADS_UF_WORKSTATION_TRUST_ACCOUNT: ADS_USER_FLAG_ENUM = 4096;
pub type ADS_USER_FLAG_ENUM = i32;
pub const ADS_USE_DELEGATION: ADS_AUTHENTICATION_ENUM = 256;
pub const ADS_USE_ENCRYPTION: ADS_AUTHENTICATION_ENUM = 2;
pub const ADS_USE_SEALING: ADS_AUTHENTICATION_ENUM = 128;
pub const ADS_USE_SIGNING: ADS_AUTHENTICATION_ENUM = 64;
pub const ADS_USE_SSL: ADS_AUTHENTICATION_ENUM = 2;
#[cfg(feature = "Win32_minwinbase")]
pub type ADS_UTC_TIME = super::minwinbase::SYSTEMTIME;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADS_VLV {
    pub dwBeforeCount: u32,
    pub dwAfterCount: u32,
    pub dwOffset: u32,
    pub dwContentCount: u32,
    pub pszTarget: windows_core::PWSTR,
    pub dwContextIDLength: u32,
    pub lpContextID: super::minwindef::LPBYTE,
}
pub const ADS_VLV_RESPONSE: windows_core::PCWSTR = windows_core::w!("fc8cb04d-311d-406c-8cb9-1ae8b843b419");
pub const ADSystemInfo: windows_core::GUID = windows_core::GUID::from_u128(0x50b6327f_afd1_11d2_9cb9_0000f87a369e);
pub const ADsSecurityUtility: windows_core::GUID = windows_core::GUID::from_u128(0xf270c64a_ffb8_4ae4_85fe_3a75e5347966);
pub const AccessControlEntry: windows_core::GUID = windows_core::GUID::from_u128(0xb75ac000_9bdd_11d0_852c_00c04fd8d503);
pub const AccessControlList: windows_core::GUID = windows_core::GUID::from_u128(0xb85ea052_9bdd_11d0_852c_00c04fd8d503);
pub const BackLink: windows_core::GUID = windows_core::GUID::from_u128(0xfcbf906f_4080_11d1_a3ac_00c04fb950dc);
pub const CaseIgnoreList: windows_core::GUID = windows_core::GUID::from_u128(0x15f88a55_4680_11d1_a3b4_00c04fb950dc);
pub const DNWithBinary: windows_core::GUID = windows_core::GUID::from_u128(0x7e99c0a3_f935_11d2_ba96_00c04fb6d0d1);
pub const DNWithString: windows_core::GUID = windows_core::GUID::from_u128(0x334857cc_f934_11d2_ba96_00c04fb6d0d1);
pub const Email: windows_core::GUID = windows_core::GUID::from_u128(0x8f92a857_478e_11d1_a3b4_00c04fb950dc);
pub const FaxNumber: windows_core::GUID = windows_core::GUID::from_u128(0xa5062215_4681_11d1_a3b4_00c04fb950dc);
pub const Hold: windows_core::GUID = windows_core::GUID::from_u128(0xb3ad3e13_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADs, IADs_Vtbl, 0xfd8256d0_fd15_11ce_abc4_02608c9e7553);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADs {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADs, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADs {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Class(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Class)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GUID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ADsPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ADsPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Parent(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parent)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Schema(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Schema)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetInfo(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetInfo(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInfo)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Get(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Put(&self, bstrname: &windows_core::BSTR, vprop: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Put)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(vprop)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetEx(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &windows_core::BSTR, vprop: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutEx)(windows_core::Interface::as_raw(self), lncontrolcode, core::mem::transmute_copy(bstrname), core::mem::transmute_copy(vprop)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetInfoEx(&self, vproperties: &super::oaidl::VARIANT, lnreserved: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInfoEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vproperties), lnreserved) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADs_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Class: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ADsPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Schema: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInfo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Get: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Put: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Put: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetEx: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub PutEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    PutEx: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetInfoEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetInfoEx: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADs_Impl: super::oaidl::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Class(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GUID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ADsPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Parent(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Schema(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetInfo(&self) -> windows_core::Result<()>;
    fn SetInfo(&self) -> windows_core::Result<()>;
    fn Get(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn Put(&self, bstrname: &windows_core::BSTR, vprop: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn GetEx(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn PutEx(&self, lncontrolcode: i32, bstrname: &windows_core::BSTR, vprop: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn GetInfoEx(&self, vproperties: &super::oaidl::VARIANT, lnreserved: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADs_Vtbl {
    pub const fn new<Identity: IADs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADs_Impl::Name(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Class<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADs_Impl::Class(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GUID<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADs_Impl::GUID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ADsPath<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADs_Impl::ADsPath(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Parent<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADs_Impl::Parent(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Schema<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADs_Impl::Schema(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInfo<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADs_Impl::GetInfo(this).into()
            }
        }
        unsafe extern "system" fn SetInfo<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADs_Impl::SetInfo(this).into()
            }
        }
        unsafe extern "system" fn Get<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pvprop: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADs_Impl::Get(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        pvprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Put<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, vprop: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADs_Impl::Put(this, core::mem::transmute(&bstrname), core::mem::transmute(&vprop)).into()
            }
        }
        unsafe extern "system" fn GetEx<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pvprop: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADs_Impl::GetEx(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        pvprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PutEx<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lncontrolcode: i32, bstrname: *mut core::ffi::c_void, vprop: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADs_Impl::PutEx(this, core::mem::transmute_copy(&lncontrolcode), core::mem::transmute(&bstrname), core::mem::transmute(&vprop)).into()
            }
        }
        unsafe extern "system" fn GetInfoEx<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vproperties: super::oaidl::VARIANT, lnreserved: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADs_Impl::GetInfoEx(this, core::mem::transmute(&vproperties), core::mem::transmute_copy(&lnreserved)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Class: Class::<Identity, OFFSET>,
            GUID: GUID::<Identity, OFFSET>,
            ADsPath: ADsPath::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            Schema: Schema::<Identity, OFFSET>,
            GetInfo: GetInfo::<Identity, OFFSET>,
            SetInfo: SetInfo::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            Put: Put::<Identity, OFFSET>,
            GetEx: GetEx::<Identity, OFFSET>,
            PutEx: PutEx::<Identity, OFFSET>,
            GetInfoEx: GetInfoEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADs as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADs {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsADSystemInfo, IADsADSystemInfo_Vtbl, 0x5bb11929_afd1_11d2_9cb9_0000f87a369e);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsADSystemInfo {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsADSystemInfo, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsADSystemInfo {
    pub unsafe fn UserName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ComputerName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputerName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SiteName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SiteName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DomainShortName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DomainShortName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DomainDNSName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DomainDNSName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ForestDNSName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ForestDNSName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn PDCRoleOwner(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PDCRoleOwner)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SchemaRoleOwner(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SchemaRoleOwner)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn IsNativeMode(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsNativeMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAnyDCName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAnyDCName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDCSiteName(&self, szserver: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDCSiteName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(szserver), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RefreshSchemaCache(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RefreshSchemaCache)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetTrees(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTrees)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsADSystemInfo_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComputerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SiteName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DomainShortName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DomainDNSName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ForestDNSName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PDCRoleOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SchemaRoleOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub IsNativeMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    IsNativeMode: usize,
    pub GetAnyDCName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDCSiteName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RefreshSchemaCache: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetTrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetTrees: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsADSystemInfo_Impl: super::oaidl::IDispatch_Impl {
    fn UserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ComputerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SiteName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DomainShortName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DomainDNSName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ForestDNSName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PDCRoleOwner(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SchemaRoleOwner(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsNativeMode(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn GetAnyDCName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDCSiteName(&self, szserver: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn RefreshSchemaCache(&self) -> windows_core::Result<()>;
    fn GetTrees(&self) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsADSystemInfo_Vtbl {
    pub const fn new<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UserName<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::UserName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ComputerName<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::ComputerName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SiteName<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::SiteName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DomainShortName<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::DomainShortName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DomainDNSName<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::DomainDNSName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ForestDNSName<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::ForestDNSName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PDCRoleOwner<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::PDCRoleOwner(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SchemaRoleOwner<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::SchemaRoleOwner(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsNativeMode<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::IsNativeMode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAnyDCName<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdcname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::GetAnyDCName(this) {
                    Ok(ok__) => {
                        pszdcname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDCSiteName<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szserver: *mut core::ffi::c_void, pszsitename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::GetDCSiteName(this, core::mem::transmute(&szserver)) {
                    Ok(ok__) => {
                        pszsitename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RefreshSchemaCache<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsADSystemInfo_Impl::RefreshSchemaCache(this).into()
            }
        }
        unsafe extern "system" fn GetTrees<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtrees: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::GetTrees(this) {
                    Ok(ok__) => {
                        pvtrees.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            UserName: UserName::<Identity, OFFSET>,
            ComputerName: ComputerName::<Identity, OFFSET>,
            SiteName: SiteName::<Identity, OFFSET>,
            DomainShortName: DomainShortName::<Identity, OFFSET>,
            DomainDNSName: DomainDNSName::<Identity, OFFSET>,
            ForestDNSName: ForestDNSName::<Identity, OFFSET>,
            PDCRoleOwner: PDCRoleOwner::<Identity, OFFSET>,
            SchemaRoleOwner: SchemaRoleOwner::<Identity, OFFSET>,
            IsNativeMode: IsNativeMode::<Identity, OFFSET>,
            GetAnyDCName: GetAnyDCName::<Identity, OFFSET>,
            GetDCSiteName: GetDCSiteName::<Identity, OFFSET>,
            RefreshSchemaCache: RefreshSchemaCache::<Identity, OFFSET>,
            GetTrees: GetTrees::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsADSystemInfo as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsADSystemInfo {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsAccessControlEntry, IADsAccessControlEntry_Vtbl, 0xb4f3a14c_9bdd_11d0_852c_00c04fd8d503);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsAccessControlEntry {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsAccessControlEntry, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsAccessControlEntry {
    pub unsafe fn AccessMask(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AccessMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAccessMask(&self, lnaccessmask: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAccessMask)(windows_core::Interface::as_raw(self), lnaccessmask) }
    }
    pub unsafe fn AceType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAceType(&self, lnacetype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAceType)(windows_core::Interface::as_raw(self), lnacetype) }
    }
    pub unsafe fn AceFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AceFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAceFlags(&self, lnaceflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAceFlags)(windows_core::Interface::as_raw(self), lnaceflags) }
    }
    pub unsafe fn Flags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFlags(&self, lnflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), lnflags) }
    }
    pub unsafe fn ObjectType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ObjectType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetObjectType(&self, bstrobjecttype: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetObjectType)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrobjecttype)) }
    }
    pub unsafe fn InheritedObjectType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InheritedObjectType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetInheritedObjectType(&self, bstrinheritedobjecttype: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInheritedObjectType)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrinheritedobjecttype)) }
    }
    pub unsafe fn Trustee(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Trustee)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTrustee(&self, bstrtrustee: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTrustee)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtrustee)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsAccessControlEntry_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub AccessMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAccessMask: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAceType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AceFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAceFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InheritedObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInheritedObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Trustee: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTrustee: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsAccessControlEntry_Impl: super::oaidl::IDispatch_Impl {
    fn AccessMask(&self) -> windows_core::Result<i32>;
    fn SetAccessMask(&self, lnaccessmask: i32) -> windows_core::Result<()>;
    fn AceType(&self) -> windows_core::Result<i32>;
    fn SetAceType(&self, lnacetype: i32) -> windows_core::Result<()>;
    fn AceFlags(&self) -> windows_core::Result<i32>;
    fn SetAceFlags(&self, lnaceflags: i32) -> windows_core::Result<()>;
    fn Flags(&self) -> windows_core::Result<i32>;
    fn SetFlags(&self, lnflags: i32) -> windows_core::Result<()>;
    fn ObjectType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetObjectType(&self, bstrobjecttype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InheritedObjectType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetInheritedObjectType(&self, bstrinheritedobjecttype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Trustee(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTrustee(&self, bstrtrustee: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsAccessControlEntry_Vtbl {
    pub const fn new<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AccessMask<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlEntry_Impl::AccessMask(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAccessMask<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnaccessmask: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlEntry_Impl::SetAccessMask(this, core::mem::transmute_copy(&lnaccessmask)).into()
            }
        }
        unsafe extern "system" fn AceType<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlEntry_Impl::AceType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAceType<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnacetype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlEntry_Impl::SetAceType(this, core::mem::transmute_copy(&lnacetype)).into()
            }
        }
        unsafe extern "system" fn AceFlags<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlEntry_Impl::AceFlags(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAceFlags<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnaceflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlEntry_Impl::SetAceFlags(this, core::mem::transmute_copy(&lnaceflags)).into()
            }
        }
        unsafe extern "system" fn Flags<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlEntry_Impl::Flags(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFlags<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlEntry_Impl::SetFlags(this, core::mem::transmute_copy(&lnflags)).into()
            }
        }
        unsafe extern "system" fn ObjectType<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlEntry_Impl::ObjectType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetObjectType<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrobjecttype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlEntry_Impl::SetObjectType(this, core::mem::transmute(&bstrobjecttype)).into()
            }
        }
        unsafe extern "system" fn InheritedObjectType<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlEntry_Impl::InheritedObjectType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInheritedObjectType<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinheritedobjecttype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlEntry_Impl::SetInheritedObjectType(this, core::mem::transmute(&bstrinheritedobjecttype)).into()
            }
        }
        unsafe extern "system" fn Trustee<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlEntry_Impl::Trustee(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTrustee<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtrustee: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlEntry_Impl::SetTrustee(this, core::mem::transmute(&bstrtrustee)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AccessMask: AccessMask::<Identity, OFFSET>,
            SetAccessMask: SetAccessMask::<Identity, OFFSET>,
            AceType: AceType::<Identity, OFFSET>,
            SetAceType: SetAceType::<Identity, OFFSET>,
            AceFlags: AceFlags::<Identity, OFFSET>,
            SetAceFlags: SetAceFlags::<Identity, OFFSET>,
            Flags: Flags::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
            ObjectType: ObjectType::<Identity, OFFSET>,
            SetObjectType: SetObjectType::<Identity, OFFSET>,
            InheritedObjectType: InheritedObjectType::<Identity, OFFSET>,
            SetInheritedObjectType: SetInheritedObjectType::<Identity, OFFSET>,
            Trustee: Trustee::<Identity, OFFSET>,
            SetTrustee: SetTrustee::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsAccessControlEntry as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsAccessControlEntry {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsAccessControlList, IADsAccessControlList_Vtbl, 0xb7ee91cc_9bdd_11d0_852c_00c04fd8d503);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsAccessControlList {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsAccessControlList, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsAccessControlList {
    pub unsafe fn AclRevision(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AclRevision)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAclRevision(&self, lnaclrevision: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAclRevision)(windows_core::Interface::as_raw(self), lnaclrevision) }
    }
    pub unsafe fn AceCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAceCount(&self, lnacecount: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAceCount)(windows_core::Interface::as_raw(self), lnacecount) }
    }
    pub unsafe fn AddAce<P0>(&self, paccesscontrolentry: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAce)(windows_core::Interface::as_raw(self), paccesscontrolentry.param().abi()) }
    }
    pub unsafe fn RemoveAce<P0>(&self, paccesscontrolentry: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveAce)(windows_core::Interface::as_raw(self), paccesscontrolentry.param().abi()) }
    }
    pub unsafe fn CopyAccessList(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyAccessList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsAccessControlList_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub AclRevision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAclRevision: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAceCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AddAce: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAce: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyAccessList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsAccessControlList_Impl: super::oaidl::IDispatch_Impl {
    fn AclRevision(&self) -> windows_core::Result<i32>;
    fn SetAclRevision(&self, lnaclrevision: i32) -> windows_core::Result<()>;
    fn AceCount(&self) -> windows_core::Result<i32>;
    fn SetAceCount(&self, lnacecount: i32) -> windows_core::Result<()>;
    fn AddAce(&self, paccesscontrolentry: windows_core::Ref<super::oaidl::IDispatch>) -> windows_core::Result<()>;
    fn RemoveAce(&self, paccesscontrolentry: windows_core::Ref<super::oaidl::IDispatch>) -> windows_core::Result<()>;
    fn CopyAccessList(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsAccessControlList_Vtbl {
    pub const fn new<Identity: IADsAccessControlList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AclRevision<Identity: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlList_Impl::AclRevision(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAclRevision<Identity: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnaclrevision: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlList_Impl::SetAclRevision(this, core::mem::transmute_copy(&lnaclrevision)).into()
            }
        }
        unsafe extern "system" fn AceCount<Identity: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlList_Impl::AceCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAceCount<Identity: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnacecount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlList_Impl::SetAceCount(this, core::mem::transmute_copy(&lnacecount)).into()
            }
        }
        unsafe extern "system" fn AddAce<Identity: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paccesscontrolentry: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlList_Impl::AddAce(this, core::mem::transmute_copy(&paccesscontrolentry)).into()
            }
        }
        unsafe extern "system" fn RemoveAce<Identity: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paccesscontrolentry: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlList_Impl::RemoveAce(this, core::mem::transmute_copy(&paccesscontrolentry)).into()
            }
        }
        unsafe extern "system" fn CopyAccessList<Identity: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaccesscontrollist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlList_Impl::CopyAccessList(this) {
                    Ok(ok__) => {
                        ppaccesscontrollist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlList_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AclRevision: AclRevision::<Identity, OFFSET>,
            SetAclRevision: SetAclRevision::<Identity, OFFSET>,
            AceCount: AceCount::<Identity, OFFSET>,
            SetAceCount: SetAceCount::<Identity, OFFSET>,
            AddAce: AddAce::<Identity, OFFSET>,
            RemoveAce: RemoveAce::<Identity, OFFSET>,
            CopyAccessList: CopyAccessList::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsAccessControlList as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsAccessControlList {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsAcl, IADsAcl_Vtbl, 0x8452d3ab_0869_11d1_a377_00c04fb950dc);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsAcl {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsAcl, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsAcl {
    pub unsafe fn ProtectedAttrName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProtectedAttrName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetProtectedAttrName(&self, bstrprotectedattrname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProtectedAttrName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprotectedattrname)) }
    }
    pub unsafe fn SubjectName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubjectName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSubjectName(&self, bstrsubjectname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSubjectName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsubjectname)) }
    }
    pub unsafe fn Privileges(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Privileges)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPrivileges(&self, lnprivileges: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivileges)(windows_core::Interface::as_raw(self), lnprivileges) }
    }
    pub unsafe fn CopyAcl(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyAcl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsAcl_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub ProtectedAttrName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProtectedAttrName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SubjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSubjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Privileges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPrivileges: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CopyAcl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsAcl_Impl: super::oaidl::IDispatch_Impl {
    fn ProtectedAttrName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProtectedAttrName(&self, bstrprotectedattrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SubjectName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubjectName(&self, bstrsubjectname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Privileges(&self) -> windows_core::Result<i32>;
    fn SetPrivileges(&self, lnprivileges: i32) -> windows_core::Result<()>;
    fn CopyAcl(&self) -> windows_core::Result<super::oaidl::IDispatch>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsAcl_Vtbl {
    pub const fn new<Identity: IADsAcl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProtectedAttrName<Identity: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAcl_Impl::ProtectedAttrName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProtectedAttrName<Identity: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotectedattrname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAcl_Impl::SetProtectedAttrName(this, core::mem::transmute(&bstrprotectedattrname)).into()
            }
        }
        unsafe extern "system" fn SubjectName<Identity: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAcl_Impl::SubjectName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSubjectName<Identity: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsubjectname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAcl_Impl::SetSubjectName(this, core::mem::transmute(&bstrsubjectname)).into()
            }
        }
        unsafe extern "system" fn Privileges<Identity: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAcl_Impl::Privileges(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivileges<Identity: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnprivileges: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAcl_Impl::SetPrivileges(this, core::mem::transmute_copy(&lnprivileges)).into()
            }
        }
        unsafe extern "system" fn CopyAcl<Identity: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppacl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAcl_Impl::CopyAcl(this) {
                    Ok(ok__) => {
                        ppacl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ProtectedAttrName: ProtectedAttrName::<Identity, OFFSET>,
            SetProtectedAttrName: SetProtectedAttrName::<Identity, OFFSET>,
            SubjectName: SubjectName::<Identity, OFFSET>,
            SetSubjectName: SetSubjectName::<Identity, OFFSET>,
            Privileges: Privileges::<Identity, OFFSET>,
            SetPrivileges: SetPrivileges::<Identity, OFFSET>,
            CopyAcl: CopyAcl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsAcl as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsAcl {}
windows_core::imp::define_interface!(IADsAggregatee, IADsAggregatee_Vtbl, 0x1346ce8c_9039_11d0_8528_00c04fd8d503);
windows_core::imp::interface_hierarchy!(IADsAggregatee, windows_core::IUnknown);
impl IADsAggregatee {
    pub unsafe fn ConnectAsAggregatee<P0>(&self, pouterunknown: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConnectAsAggregatee)(windows_core::Interface::as_raw(self), pouterunknown.param().abi()) }
    }
    pub unsafe fn DisconnectAsAggregatee(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisconnectAsAggregatee)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RelinquishInterface(&self, riid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RelinquishInterface)(windows_core::Interface::as_raw(self), riid) }
    }
    pub unsafe fn RestoreInterface(&self, riid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestoreInterface)(windows_core::Interface::as_raw(self), riid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IADsAggregatee_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectAsAggregatee: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisconnectAsAggregatee: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RelinquishInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub RestoreInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IADsAggregatee_Impl: windows_core::IUnknownImpl {
    fn ConnectAsAggregatee(&self, pouterunknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DisconnectAsAggregatee(&self) -> windows_core::Result<()>;
    fn RelinquishInterface(&self, riid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RestoreInterface(&self, riid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IADsAggregatee_Vtbl {
    pub const fn new<Identity: IADsAggregatee_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectAsAggregatee<Identity: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pouterunknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAggregatee_Impl::ConnectAsAggregatee(this, core::mem::transmute_copy(&pouterunknown)).into()
            }
        }
        unsafe extern "system" fn DisconnectAsAggregatee<Identity: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAggregatee_Impl::DisconnectAsAggregatee(this).into()
            }
        }
        unsafe extern "system" fn RelinquishInterface<Identity: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAggregatee_Impl::RelinquishInterface(this, core::mem::transmute_copy(&riid)).into()
            }
        }
        unsafe extern "system" fn RestoreInterface<Identity: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAggregatee_Impl::RestoreInterface(this, core::mem::transmute_copy(&riid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectAsAggregatee: ConnectAsAggregatee::<Identity, OFFSET>,
            DisconnectAsAggregatee: DisconnectAsAggregatee::<Identity, OFFSET>,
            RelinquishInterface: RelinquishInterface::<Identity, OFFSET>,
            RestoreInterface: RestoreInterface::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsAggregatee as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IADsAggregatee {}
windows_core::imp::define_interface!(IADsAggregator, IADsAggregator_Vtbl, 0x52db5fb0_941f_11d0_8529_00c04fd8d503);
windows_core::imp::interface_hierarchy!(IADsAggregator, windows_core::IUnknown);
impl IADsAggregator {
    pub unsafe fn ConnectAsAggregator<P0>(&self, paggregatee: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConnectAsAggregator)(windows_core::Interface::as_raw(self), paggregatee.param().abi()) }
    }
    pub unsafe fn DisconnectAsAggregator(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisconnectAsAggregator)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IADsAggregator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectAsAggregator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisconnectAsAggregator: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IADsAggregator_Impl: windows_core::IUnknownImpl {
    fn ConnectAsAggregator(&self, paggregatee: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DisconnectAsAggregator(&self) -> windows_core::Result<()>;
}
impl IADsAggregator_Vtbl {
    pub const fn new<Identity: IADsAggregator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectAsAggregator<Identity: IADsAggregator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggregatee: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAggregator_Impl::ConnectAsAggregator(this, core::mem::transmute_copy(&paggregatee)).into()
            }
        }
        unsafe extern "system" fn DisconnectAsAggregator<Identity: IADsAggregator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAggregator_Impl::DisconnectAsAggregator(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectAsAggregator: ConnectAsAggregator::<Identity, OFFSET>,
            DisconnectAsAggregator: DisconnectAsAggregator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsAggregator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IADsAggregator {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsBackLink, IADsBackLink_Vtbl, 0xfd1302bd_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsBackLink {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsBackLink, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsBackLink {
    pub unsafe fn RemoteID(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRemoteID(&self, lnremoteid: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteID)(windows_core::Interface::as_raw(self), lnremoteid) }
    }
    pub unsafe fn ObjectName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ObjectName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetObjectName(&self, bstrobjectname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetObjectName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrobjectname)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsBackLink_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub RemoteID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRemoteID: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsBackLink_Impl: super::oaidl::IDispatch_Impl {
    fn RemoteID(&self) -> windows_core::Result<i32>;
    fn SetRemoteID(&self, lnremoteid: i32) -> windows_core::Result<()>;
    fn ObjectName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetObjectName(&self, bstrobjectname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsBackLink_Vtbl {
    pub const fn new<Identity: IADsBackLink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RemoteID<Identity: IADsBackLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsBackLink_Impl::RemoteID(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteID<Identity: IADsBackLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnremoteid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsBackLink_Impl::SetRemoteID(this, core::mem::transmute_copy(&lnremoteid)).into()
            }
        }
        unsafe extern "system" fn ObjectName<Identity: IADsBackLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsBackLink_Impl::ObjectName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetObjectName<Identity: IADsBackLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrobjectname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsBackLink_Impl::SetObjectName(this, core::mem::transmute(&bstrobjectname)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RemoteID: RemoteID::<Identity, OFFSET>,
            SetRemoteID: SetRemoteID::<Identity, OFFSET>,
            ObjectName: ObjectName::<Identity, OFFSET>,
            SetObjectName: SetObjectName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsBackLink as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsBackLink {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsCaseIgnoreList, IADsCaseIgnoreList_Vtbl, 0x7b66b533_4680_11d1_a3b4_00c04fb950dc);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsCaseIgnoreList {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsCaseIgnoreList, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsCaseIgnoreList {
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn CaseIgnoreList(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CaseIgnoreList)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetCaseIgnoreList(&self, vcaseignorelist: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCaseIgnoreList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vcaseignorelist)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsCaseIgnoreList_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub CaseIgnoreList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    CaseIgnoreList: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetCaseIgnoreList: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetCaseIgnoreList: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsCaseIgnoreList_Impl: super::oaidl::IDispatch_Impl {
    fn CaseIgnoreList(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetCaseIgnoreList(&self, vcaseignorelist: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsCaseIgnoreList_Vtbl {
    pub const fn new<Identity: IADsCaseIgnoreList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CaseIgnoreList<Identity: IADsCaseIgnoreList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsCaseIgnoreList_Impl::CaseIgnoreList(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCaseIgnoreList<Identity: IADsCaseIgnoreList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcaseignorelist: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsCaseIgnoreList_Impl::SetCaseIgnoreList(this, core::mem::transmute(&vcaseignorelist)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CaseIgnoreList: CaseIgnoreList::<Identity, OFFSET>,
            SetCaseIgnoreList: SetCaseIgnoreList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsCaseIgnoreList as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsCaseIgnoreList {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsClass, IADsClass_Vtbl, 0xc8f93dd0_4ae0_11cf_9e73_00aa004a5691);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsClass {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsClass, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsClass {
    pub unsafe fn PrimaryInterface(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrimaryInterface)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CLSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCLSID(&self, bstrclsid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCLSID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrclsid)) }
    }
    pub unsafe fn OID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetOID(&self, bstroid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstroid)) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn Abstract(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Abstract)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetAbstract(&self, fabstract: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAbstract)(windows_core::Interface::as_raw(self), fabstract) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn Auxiliary(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Auxiliary)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetAuxiliary(&self, fauxiliary: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAuxiliary)(windows_core::Interface::as_raw(self), fauxiliary) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn MandatoryProperties(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MandatoryProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetMandatoryProperties(&self, vmandatoryproperties: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMandatoryProperties)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vmandatoryproperties)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn OptionalProperties(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OptionalProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetOptionalProperties(&self, voptionalproperties: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOptionalProperties)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(voptionalproperties)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn NamingProperties(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NamingProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetNamingProperties(&self, vnamingproperties: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNamingProperties)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vnamingproperties)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn DerivedFrom(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DerivedFrom)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetDerivedFrom(&self, vderivedfrom: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDerivedFrom)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vderivedfrom)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn AuxDerivedFrom(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AuxDerivedFrom)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetAuxDerivedFrom(&self, vauxderivedfrom: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAuxDerivedFrom)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vauxderivedfrom)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn PossibleSuperiors(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PossibleSuperiors)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetPossibleSuperiors(&self, vpossiblesuperiors: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPossibleSuperiors)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vpossiblesuperiors)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Containment(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Containment)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetContainment(&self, vcontainment: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetContainment)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vcontainment)) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn Container(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Container)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetContainer(&self, fcontainer: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetContainer)(windows_core::Interface::as_raw(self), fcontainer) }
    }
    pub unsafe fn HelpFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HelpFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetHelpFileName(&self, bstrhelpfilename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelpFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrhelpfilename)) }
    }
    pub unsafe fn HelpFileContext(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HelpFileContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHelpFileContext(&self, lnhelpfilecontext: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelpFileContext)(windows_core::Interface::as_raw(self), lnhelpfilecontext) }
    }
    pub unsafe fn Qualifiers(&self) -> windows_core::Result<IADsCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Qualifiers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsClass_Vtbl {
    pub base__: IADs_Vtbl,
    pub PrimaryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub Abstract: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    Abstract: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetAbstract: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetAbstract: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub Auxiliary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    Auxiliary: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetAuxiliary: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetAuxiliary: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub MandatoryProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    MandatoryProperties: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetMandatoryProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetMandatoryProperties: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub OptionalProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    OptionalProperties: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetOptionalProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetOptionalProperties: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub NamingProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    NamingProperties: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetNamingProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetNamingProperties: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub DerivedFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    DerivedFrom: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetDerivedFrom: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetDerivedFrom: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub AuxDerivedFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    AuxDerivedFrom: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetAuxDerivedFrom: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetAuxDerivedFrom: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub PossibleSuperiors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    PossibleSuperiors: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetPossibleSuperiors: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetPossibleSuperiors: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Containment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Containment: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetContainment: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetContainment: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub Container: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    Container: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetContainer: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetContainer: usize,
    pub HelpFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHelpFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HelpFileContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHelpFileContext: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Qualifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsClass_Impl: IADs_Impl {
    fn PrimaryInterface(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CLSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCLSID(&self, bstrclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOID(&self, bstroid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Abstract(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetAbstract(&self, fabstract: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Auxiliary(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetAuxiliary(&self, fauxiliary: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MandatoryProperties(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetMandatoryProperties(&self, vmandatoryproperties: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn OptionalProperties(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetOptionalProperties(&self, voptionalproperties: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn NamingProperties(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetNamingProperties(&self, vnamingproperties: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn DerivedFrom(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetDerivedFrom(&self, vderivedfrom: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn AuxDerivedFrom(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetAuxDerivedFrom(&self, vauxderivedfrom: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn PossibleSuperiors(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetPossibleSuperiors(&self, vpossiblesuperiors: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Containment(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetContainment(&self, vcontainment: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Container(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetContainer(&self, fcontainer: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn HelpFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetHelpFileName(&self, bstrhelpfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn HelpFileContext(&self) -> windows_core::Result<i32>;
    fn SetHelpFileContext(&self, lnhelpfilecontext: i32) -> windows_core::Result<()>;
    fn Qualifiers(&self) -> windows_core::Result<IADsCollection>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsClass_Vtbl {
    pub const fn new<Identity: IADsClass_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrimaryInterface<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::PrimaryInterface(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CLSID<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::CLSID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCLSID<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclsid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::SetCLSID(this, core::mem::transmute(&bstrclsid)).into()
            }
        }
        unsafe extern "system" fn OID<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::OID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOID<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::SetOID(this, core::mem::transmute(&bstroid)).into()
            }
        }
        unsafe extern "system" fn Abstract<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::Abstract(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAbstract<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fabstract: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::SetAbstract(this, core::mem::transmute_copy(&fabstract)).into()
            }
        }
        unsafe extern "system" fn Auxiliary<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::Auxiliary(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuxiliary<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fauxiliary: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::SetAuxiliary(this, core::mem::transmute_copy(&fauxiliary)).into()
            }
        }
        unsafe extern "system" fn MandatoryProperties<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::MandatoryProperties(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMandatoryProperties<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vmandatoryproperties: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::SetMandatoryProperties(this, core::mem::transmute(&vmandatoryproperties)).into()
            }
        }
        unsafe extern "system" fn OptionalProperties<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::OptionalProperties(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOptionalProperties<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, voptionalproperties: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::SetOptionalProperties(this, core::mem::transmute(&voptionalproperties)).into()
            }
        }
        unsafe extern "system" fn NamingProperties<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::NamingProperties(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNamingProperties<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vnamingproperties: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::SetNamingProperties(this, core::mem::transmute(&vnamingproperties)).into()
            }
        }
        unsafe extern "system" fn DerivedFrom<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::DerivedFrom(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDerivedFrom<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vderivedfrom: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::SetDerivedFrom(this, core::mem::transmute(&vderivedfrom)).into()
            }
        }
        unsafe extern "system" fn AuxDerivedFrom<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::AuxDerivedFrom(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuxDerivedFrom<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vauxderivedfrom: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::SetAuxDerivedFrom(this, core::mem::transmute(&vauxderivedfrom)).into()
            }
        }
        unsafe extern "system" fn PossibleSuperiors<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::PossibleSuperiors(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPossibleSuperiors<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vpossiblesuperiors: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::SetPossibleSuperiors(this, core::mem::transmute(&vpossiblesuperiors)).into()
            }
        }
        unsafe extern "system" fn Containment<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::Containment(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContainment<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcontainment: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::SetContainment(this, core::mem::transmute(&vcontainment)).into()
            }
        }
        unsafe extern "system" fn Container<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::Container(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContainer<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcontainer: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::SetContainer(this, core::mem::transmute_copy(&fcontainer)).into()
            }
        }
        unsafe extern "system" fn HelpFileName<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::HelpFileName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHelpFileName<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhelpfilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::SetHelpFileName(this, core::mem::transmute(&bstrhelpfilename)).into()
            }
        }
        unsafe extern "system" fn HelpFileContext<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::HelpFileContext(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHelpFileContext<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnhelpfilecontext: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::SetHelpFileContext(this, core::mem::transmute_copy(&lnhelpfilecontext)).into()
            }
        }
        unsafe extern "system" fn Qualifiers<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqualifiers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::Qualifiers(this) {
                    Ok(ok__) => {
                        ppqualifiers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            PrimaryInterface: PrimaryInterface::<Identity, OFFSET>,
            CLSID: CLSID::<Identity, OFFSET>,
            SetCLSID: SetCLSID::<Identity, OFFSET>,
            OID: OID::<Identity, OFFSET>,
            SetOID: SetOID::<Identity, OFFSET>,
            Abstract: Abstract::<Identity, OFFSET>,
            SetAbstract: SetAbstract::<Identity, OFFSET>,
            Auxiliary: Auxiliary::<Identity, OFFSET>,
            SetAuxiliary: SetAuxiliary::<Identity, OFFSET>,
            MandatoryProperties: MandatoryProperties::<Identity, OFFSET>,
            SetMandatoryProperties: SetMandatoryProperties::<Identity, OFFSET>,
            OptionalProperties: OptionalProperties::<Identity, OFFSET>,
            SetOptionalProperties: SetOptionalProperties::<Identity, OFFSET>,
            NamingProperties: NamingProperties::<Identity, OFFSET>,
            SetNamingProperties: SetNamingProperties::<Identity, OFFSET>,
            DerivedFrom: DerivedFrom::<Identity, OFFSET>,
            SetDerivedFrom: SetDerivedFrom::<Identity, OFFSET>,
            AuxDerivedFrom: AuxDerivedFrom::<Identity, OFFSET>,
            SetAuxDerivedFrom: SetAuxDerivedFrom::<Identity, OFFSET>,
            PossibleSuperiors: PossibleSuperiors::<Identity, OFFSET>,
            SetPossibleSuperiors: SetPossibleSuperiors::<Identity, OFFSET>,
            Containment: Containment::<Identity, OFFSET>,
            SetContainment: SetContainment::<Identity, OFFSET>,
            Container: Container::<Identity, OFFSET>,
            SetContainer: SetContainer::<Identity, OFFSET>,
            HelpFileName: HelpFileName::<Identity, OFFSET>,
            SetHelpFileName: SetHelpFileName::<Identity, OFFSET>,
            HelpFileContext: HelpFileContext::<Identity, OFFSET>,
            SetHelpFileContext: SetHelpFileContext::<Identity, OFFSET>,
            Qualifiers: Qualifiers::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsClass as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsClass {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsCollection, IADsCollection_Vtbl, 0x72b945e0_253b_11cf_a988_00aa006bc149);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsCollection {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsCollection, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Add(&self, bstrname: &windows_core::BSTR, vitem: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(vitem)) }
    }
    pub unsafe fn Remove(&self, bstritemtoberemoved: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstritemtoberemoved)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetObject(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsCollection_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetObject: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsCollection_Impl: super::oaidl::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, bstrname: &windows_core::BSTR, vitem: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Remove(&self, bstritemtoberemoved: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetObject(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsCollection_Vtbl {
    pub const fn new<Identity: IADsCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IADsCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IADsCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, vitem: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsCollection_Impl::Add(this, core::mem::transmute(&bstrname), core::mem::transmute(&vitem)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IADsCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemtoberemoved: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsCollection_Impl::Remove(this, core::mem::transmute(&bstritemtoberemoved)).into()
            }
        }
        unsafe extern "system" fn GetObject<Identity: IADsCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pvitem: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsCollection_Impl::GetObject(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        pvitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsCollection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsCollection {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsComputer, IADsComputer_Vtbl, 0xefe3cc70_1d9f_11cf_b1f3_02608c9e7553);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsComputer {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsComputer, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsComputer {
    pub unsafe fn ComputerID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputerID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Site(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Site)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn Location(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Location)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocation(&self, bstrlocation: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLocation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlocation)) }
    }
    pub unsafe fn PrimaryUser(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrimaryUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPrimaryUser(&self, bstrprimaryuser: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrimaryUser)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprimaryuser)) }
    }
    pub unsafe fn Owner(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Owner)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetOwner(&self, bstrowner: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOwner)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrowner)) }
    }
    pub unsafe fn Division(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Division)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDivision(&self, bstrdivision: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDivision)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdivision)) }
    }
    pub unsafe fn Department(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Department)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDepartment(&self, bstrdepartment: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDepartment)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdepartment)) }
    }
    pub unsafe fn Role(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Role)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRole(&self, bstrrole: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRole)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrole)) }
    }
    pub unsafe fn OperatingSystem(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OperatingSystem)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetOperatingSystem(&self, bstroperatingsystem: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOperatingSystem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstroperatingsystem)) }
    }
    pub unsafe fn OperatingSystemVersion(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OperatingSystemVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetOperatingSystemVersion(&self, bstroperatingsystemversion: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOperatingSystemVersion)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstroperatingsystemversion)) }
    }
    pub unsafe fn Model(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Model)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetModel(&self, bstrmodel: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetModel)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmodel)) }
    }
    pub unsafe fn Processor(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Processor)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetProcessor(&self, bstrprocessor: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProcessor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprocessor)) }
    }
    pub unsafe fn ProcessorCount(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProcessorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetProcessorCount(&self, bstrprocessorcount: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProcessorCount)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprocessorcount)) }
    }
    pub unsafe fn MemorySize(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MemorySize)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMemorySize(&self, bstrmemorysize: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMemorySize)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmemorysize)) }
    }
    pub unsafe fn StorageCapacity(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StorageCapacity)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetStorageCapacity(&self, bstrstoragecapacity: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStorageCapacity)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrstoragecapacity)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn NetAddresses(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NetAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetNetAddresses(&self, vnetaddresses: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNetAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vnetaddresses)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsComputer_Vtbl {
    pub base__: IADs_Vtbl,
    pub ComputerID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Site: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Location: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrimaryUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrimaryUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Owner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Division: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDivision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Department: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDepartment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Role: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OperatingSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOperatingSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OperatingSystemVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOperatingSystemVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Model: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Processor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProcessorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MemorySize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMemorySize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StorageCapacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStorageCapacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub NetAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    NetAddresses: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetNetAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetNetAddresses: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsComputer_Impl: IADs_Impl {
    fn ComputerID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Site(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Location(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocation(&self, bstrlocation: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PrimaryUser(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPrimaryUser(&self, bstrprimaryuser: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Owner(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOwner(&self, bstrowner: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Division(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDivision(&self, bstrdivision: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Department(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDepartment(&self, bstrdepartment: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Role(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRole(&self, bstrrole: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OperatingSystem(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOperatingSystem(&self, bstroperatingsystem: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OperatingSystemVersion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOperatingSystemVersion(&self, bstroperatingsystemversion: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Model(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetModel(&self, bstrmodel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Processor(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProcessor(&self, bstrprocessor: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ProcessorCount(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProcessorCount(&self, bstrprocessorcount: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MemorySize(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMemorySize(&self, bstrmemorysize: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StorageCapacity(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetStorageCapacity(&self, bstrstoragecapacity: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NetAddresses(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetNetAddresses(&self, vnetaddresses: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsComputer_Vtbl {
    pub const fn new<Identity: IADsComputer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ComputerID<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::ComputerID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Site<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::Site(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn Location<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::Location(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocation<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlocation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::SetLocation(this, core::mem::transmute(&bstrlocation)).into()
            }
        }
        unsafe extern "system" fn PrimaryUser<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::PrimaryUser(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrimaryUser<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprimaryuser: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::SetPrimaryUser(this, core::mem::transmute(&bstrprimaryuser)).into()
            }
        }
        unsafe extern "system" fn Owner<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::Owner(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOwner<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrowner: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::SetOwner(this, core::mem::transmute(&bstrowner)).into()
            }
        }
        unsafe extern "system" fn Division<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::Division(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDivision<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdivision: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::SetDivision(this, core::mem::transmute(&bstrdivision)).into()
            }
        }
        unsafe extern "system" fn Department<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::Department(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDepartment<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdepartment: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::SetDepartment(this, core::mem::transmute(&bstrdepartment)).into()
            }
        }
        unsafe extern "system" fn Role<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::Role(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRole<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrole: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::SetRole(this, core::mem::transmute(&bstrrole)).into()
            }
        }
        unsafe extern "system" fn OperatingSystem<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::OperatingSystem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOperatingSystem<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroperatingsystem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::SetOperatingSystem(this, core::mem::transmute(&bstroperatingsystem)).into()
            }
        }
        unsafe extern "system" fn OperatingSystemVersion<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::OperatingSystemVersion(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOperatingSystemVersion<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroperatingsystemversion: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::SetOperatingSystemVersion(this, core::mem::transmute(&bstroperatingsystemversion)).into()
            }
        }
        unsafe extern "system" fn Model<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::Model(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetModel<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmodel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::SetModel(this, core::mem::transmute(&bstrmodel)).into()
            }
        }
        unsafe extern "system" fn Processor<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::Processor(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProcessor<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprocessor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::SetProcessor(this, core::mem::transmute(&bstrprocessor)).into()
            }
        }
        unsafe extern "system" fn ProcessorCount<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::ProcessorCount(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProcessorCount<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprocessorcount: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::SetProcessorCount(this, core::mem::transmute(&bstrprocessorcount)).into()
            }
        }
        unsafe extern "system" fn MemorySize<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::MemorySize(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMemorySize<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmemorysize: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::SetMemorySize(this, core::mem::transmute(&bstrmemorysize)).into()
            }
        }
        unsafe extern "system" fn StorageCapacity<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::StorageCapacity(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStorageCapacity<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrstoragecapacity: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::SetStorageCapacity(this, core::mem::transmute(&bstrstoragecapacity)).into()
            }
        }
        unsafe extern "system" fn NetAddresses<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::NetAddresses(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNetAddresses<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vnetaddresses: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::SetNetAddresses(this, core::mem::transmute(&vnetaddresses)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            ComputerID: ComputerID::<Identity, OFFSET>,
            Site: Site::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            Location: Location::<Identity, OFFSET>,
            SetLocation: SetLocation::<Identity, OFFSET>,
            PrimaryUser: PrimaryUser::<Identity, OFFSET>,
            SetPrimaryUser: SetPrimaryUser::<Identity, OFFSET>,
            Owner: Owner::<Identity, OFFSET>,
            SetOwner: SetOwner::<Identity, OFFSET>,
            Division: Division::<Identity, OFFSET>,
            SetDivision: SetDivision::<Identity, OFFSET>,
            Department: Department::<Identity, OFFSET>,
            SetDepartment: SetDepartment::<Identity, OFFSET>,
            Role: Role::<Identity, OFFSET>,
            SetRole: SetRole::<Identity, OFFSET>,
            OperatingSystem: OperatingSystem::<Identity, OFFSET>,
            SetOperatingSystem: SetOperatingSystem::<Identity, OFFSET>,
            OperatingSystemVersion: OperatingSystemVersion::<Identity, OFFSET>,
            SetOperatingSystemVersion: SetOperatingSystemVersion::<Identity, OFFSET>,
            Model: Model::<Identity, OFFSET>,
            SetModel: SetModel::<Identity, OFFSET>,
            Processor: Processor::<Identity, OFFSET>,
            SetProcessor: SetProcessor::<Identity, OFFSET>,
            ProcessorCount: ProcessorCount::<Identity, OFFSET>,
            SetProcessorCount: SetProcessorCount::<Identity, OFFSET>,
            MemorySize: MemorySize::<Identity, OFFSET>,
            SetMemorySize: SetMemorySize::<Identity, OFFSET>,
            StorageCapacity: StorageCapacity::<Identity, OFFSET>,
            SetStorageCapacity: SetStorageCapacity::<Identity, OFFSET>,
            NetAddresses: NetAddresses::<Identity, OFFSET>,
            SetNetAddresses: SetNetAddresses::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsComputer as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsComputer {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsComputerOperations, IADsComputerOperations_Vtbl, 0xef497680_1d9f_11cf_b1f3_02608c9e7553);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsComputerOperations {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsComputerOperations, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsComputerOperations {
    pub unsafe fn Status(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn Shutdown(&self, breboot: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self), breboot) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsComputerOperations_Vtbl {
    pub base__: IADs_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    Shutdown: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsComputerOperations_Impl: IADs_Impl {
    fn Status(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Shutdown(&self, breboot: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsComputerOperations_Vtbl {
    pub const fn new<Identity: IADsComputerOperations_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Status<Identity: IADsComputerOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputerOperations_Impl::Status(this) {
                    Ok(ok__) => {
                        ppobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IADsComputerOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, breboot: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputerOperations_Impl::Shutdown(this, core::mem::transmute_copy(&breboot)).into()
            }
        }
        Self { base__: IADs_Vtbl::new::<Identity, OFFSET>(), Status: Status::<Identity, OFFSET>, Shutdown: Shutdown::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsComputerOperations as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsComputerOperations {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsContainer, IADsContainer_Vtbl, 0x001677d0_fd16_11ce_abc4_02608c9e7553);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsContainer {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsContainer, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsContainer {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Filter(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Filter)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetFilter(&self, var: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFilter)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(var)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Hints(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Hints)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetHints(&self, vhints: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHints)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vhints)) }
    }
    pub unsafe fn GetObject(&self, classname: &windows_core::BSTR, relativename: &windows_core::BSTR) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(classname), core::mem::transmute_copy(relativename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Create(&self, classname: &windows_core::BSTR, relativename: &windows_core::BSTR) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(classname), core::mem::transmute_copy(relativename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete(&self, bstrclassname: &windows_core::BSTR, bstrrelativename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrclassname), core::mem::transmute_copy(bstrrelativename)) }
    }
    pub unsafe fn CopyHere(&self, sourcename: &windows_core::BSTR, newname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyHere)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sourcename), core::mem::transmute_copy(newname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn MoveHere(&self, sourcename: &windows_core::BSTR, newname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveHere)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sourcename), core::mem::transmute_copy(newname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsContainer_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Filter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Filter: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetFilter: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetFilter: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Hints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Hints: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetHints: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetHints: usize,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyHere: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveHere: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsContainer_Impl: super::oaidl::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Filter(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetFilter(&self, var: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Hints(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetHints(&self, vhints: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn GetObject(&self, classname: &windows_core::BSTR, relativename: &windows_core::BSTR) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Create(&self, classname: &windows_core::BSTR, relativename: &windows_core::BSTR) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Delete(&self, bstrclassname: &windows_core::BSTR, bstrrelativename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CopyHere(&self, sourcename: &windows_core::BSTR, newname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::IDispatch>;
    fn MoveHere(&self, sourcename: &windows_core::BSTR, newname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::IDispatch>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsContainer_Vtbl {
    pub const fn new<Identity: IADsContainer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsContainer_Impl::Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsContainer_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Filter<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsContainer_Impl::Filter(this) {
                    Ok(ok__) => {
                        pvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFilter<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, var: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsContainer_Impl::SetFilter(this, core::mem::transmute(&var)).into()
            }
        }
        unsafe extern "system" fn Hints<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvfilter: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsContainer_Impl::Hints(this) {
                    Ok(ok__) => {
                        pvfilter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHints<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vhints: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsContainer_Impl::SetHints(this, core::mem::transmute(&vhints)).into()
            }
        }
        unsafe extern "system" fn GetObject<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classname: *mut core::ffi::c_void, relativename: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsContainer_Impl::GetObject(this, core::mem::transmute(&classname), core::mem::transmute(&relativename)) {
                    Ok(ok__) => {
                        ppobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Create<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classname: *mut core::ffi::c_void, relativename: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsContainer_Impl::Create(this, core::mem::transmute(&classname), core::mem::transmute(&relativename)) {
                    Ok(ok__) => {
                        ppobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclassname: *mut core::ffi::c_void, bstrrelativename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsContainer_Impl::Delete(this, core::mem::transmute(&bstrclassname), core::mem::transmute(&bstrrelativename)).into()
            }
        }
        unsafe extern "system" fn CopyHere<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcename: *mut core::ffi::c_void, newname: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsContainer_Impl::CopyHere(this, core::mem::transmute(&sourcename), core::mem::transmute(&newname)) {
                    Ok(ok__) => {
                        ppobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveHere<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcename: *mut core::ffi::c_void, newname: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsContainer_Impl::MoveHere(this, core::mem::transmute(&sourcename), core::mem::transmute(&newname)) {
                    Ok(ok__) => {
                        ppobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Filter: Filter::<Identity, OFFSET>,
            SetFilter: SetFilter::<Identity, OFFSET>,
            Hints: Hints::<Identity, OFFSET>,
            SetHints: SetHints::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            CopyHere: CopyHere::<Identity, OFFSET>,
            MoveHere: MoveHere::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsContainer as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsContainer {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsDNWithBinary, IADsDNWithBinary_Vtbl, 0x7e99c0a2_f935_11d2_ba96_00c04fb6d0d1);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsDNWithBinary {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsDNWithBinary, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsDNWithBinary {
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn BinaryValue(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BinaryValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetBinaryValue(&self, vbinaryvalue: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBinaryValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vbinaryvalue)) }
    }
    pub unsafe fn DNString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DNString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDNString(&self, bstrdnstring: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDNString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdnstring)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsDNWithBinary_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub BinaryValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    BinaryValue: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetBinaryValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetBinaryValue: usize,
    pub DNString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDNString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsDNWithBinary_Impl: super::oaidl::IDispatch_Impl {
    fn BinaryValue(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetBinaryValue(&self, vbinaryvalue: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn DNString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDNString(&self, bstrdnstring: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsDNWithBinary_Vtbl {
    pub const fn new<Identity: IADsDNWithBinary_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BinaryValue<Identity: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDNWithBinary_Impl::BinaryValue(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBinaryValue<Identity: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbinaryvalue: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDNWithBinary_Impl::SetBinaryValue(this, core::mem::transmute(&vbinaryvalue)).into()
            }
        }
        unsafe extern "system" fn DNString<Identity: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDNWithBinary_Impl::DNString(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDNString<Identity: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdnstring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDNWithBinary_Impl::SetDNString(this, core::mem::transmute(&bstrdnstring)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            BinaryValue: BinaryValue::<Identity, OFFSET>,
            SetBinaryValue: SetBinaryValue::<Identity, OFFSET>,
            DNString: DNString::<Identity, OFFSET>,
            SetDNString: SetDNString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsDNWithBinary as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsDNWithBinary {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsDNWithString, IADsDNWithString_Vtbl, 0x370df02e_f934_11d2_ba96_00c04fb6d0d1);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsDNWithString {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsDNWithString, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsDNWithString {
    pub unsafe fn StringValue(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StringValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetStringValue(&self, bstrstringvalue: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStringValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrstringvalue)) }
    }
    pub unsafe fn DNString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DNString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDNString(&self, bstrdnstring: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDNString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdnstring)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsDNWithString_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub StringValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DNString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDNString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsDNWithString_Impl: super::oaidl::IDispatch_Impl {
    fn StringValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetStringValue(&self, bstrstringvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DNString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDNString(&self, bstrdnstring: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsDNWithString_Vtbl {
    pub const fn new<Identity: IADsDNWithString_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StringValue<Identity: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDNWithString_Impl::StringValue(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStringValue<Identity: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrstringvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDNWithString_Impl::SetStringValue(this, core::mem::transmute(&bstrstringvalue)).into()
            }
        }
        unsafe extern "system" fn DNString<Identity: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDNWithString_Impl::DNString(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDNString<Identity: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdnstring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDNWithString_Impl::SetDNString(this, core::mem::transmute(&bstrdnstring)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StringValue: StringValue::<Identity, OFFSET>,
            SetStringValue: SetStringValue::<Identity, OFFSET>,
            DNString: DNString::<Identity, OFFSET>,
            SetDNString: SetDNString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsDNWithString as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsDNWithString {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsDeleteOps, IADsDeleteOps_Vtbl, 0xb2bd0902_8878_11d1_8c21_00c04fd8d503);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsDeleteOps {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsDeleteOps, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsDeleteOps {
    pub unsafe fn DeleteObject(&self, lnflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteObject)(windows_core::Interface::as_raw(self), lnflags) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsDeleteOps_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub DeleteObject: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsDeleteOps_Impl: super::oaidl::IDispatch_Impl {
    fn DeleteObject(&self, lnflags: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsDeleteOps_Vtbl {
    pub const fn new<Identity: IADsDeleteOps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeleteObject<Identity: IADsDeleteOps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDeleteOps_Impl::DeleteObject(this, core::mem::transmute_copy(&lnflags)).into()
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), DeleteObject: DeleteObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsDeleteOps as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsDeleteOps {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsDomain, IADsDomain_Vtbl, 0x00e4c220_fd16_11ce_abc4_02608c9e7553);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsDomain {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsDomain, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsDomain {
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn IsWorkgroup(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsWorkgroup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MinPasswordLength(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinPasswordLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMinPasswordLength(&self, lnminpasswordlength: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMinPasswordLength)(windows_core::Interface::as_raw(self), lnminpasswordlength) }
    }
    pub unsafe fn MinPasswordAge(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinPasswordAge)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMinPasswordAge(&self, lnminpasswordage: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMinPasswordAge)(windows_core::Interface::as_raw(self), lnminpasswordage) }
    }
    pub unsafe fn MaxPasswordAge(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxPasswordAge)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxPasswordAge(&self, lnmaxpasswordage: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaxPasswordAge)(windows_core::Interface::as_raw(self), lnmaxpasswordage) }
    }
    pub unsafe fn MaxBadPasswordsAllowed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxBadPasswordsAllowed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxBadPasswordsAllowed(&self, lnmaxbadpasswordsallowed: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaxBadPasswordsAllowed)(windows_core::Interface::as_raw(self), lnmaxbadpasswordsallowed) }
    }
    pub unsafe fn PasswordHistoryLength(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PasswordHistoryLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPasswordHistoryLength(&self, lnpasswordhistorylength: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPasswordHistoryLength)(windows_core::Interface::as_raw(self), lnpasswordhistorylength) }
    }
    pub unsafe fn PasswordAttributes(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PasswordAttributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPasswordAttributes(&self, lnpasswordattributes: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPasswordAttributes)(windows_core::Interface::as_raw(self), lnpasswordattributes) }
    }
    pub unsafe fn AutoUnlockInterval(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoUnlockInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAutoUnlockInterval(&self, lnautounlockinterval: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutoUnlockInterval)(windows_core::Interface::as_raw(self), lnautounlockinterval) }
    }
    pub unsafe fn LockoutObservationInterval(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LockoutObservationInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLockoutObservationInterval(&self, lnlockoutobservationinterval: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLockoutObservationInterval)(windows_core::Interface::as_raw(self), lnlockoutobservationinterval) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsDomain_Vtbl {
    pub base__: IADs_Vtbl,
    #[cfg(feature = "Win32_wtypes")]
    pub IsWorkgroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    IsWorkgroup: usize,
    pub MinPasswordLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMinPasswordLength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MinPasswordAge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMinPasswordAge: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxPasswordAge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxPasswordAge: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxBadPasswordsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxBadPasswordsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub PasswordHistoryLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPasswordHistoryLength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub PasswordAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPasswordAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AutoUnlockInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAutoUnlockInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LockoutObservationInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLockoutObservationInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsDomain_Impl: IADs_Impl {
    fn IsWorkgroup(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn MinPasswordLength(&self) -> windows_core::Result<i32>;
    fn SetMinPasswordLength(&self, lnminpasswordlength: i32) -> windows_core::Result<()>;
    fn MinPasswordAge(&self) -> windows_core::Result<i32>;
    fn SetMinPasswordAge(&self, lnminpasswordage: i32) -> windows_core::Result<()>;
    fn MaxPasswordAge(&self) -> windows_core::Result<i32>;
    fn SetMaxPasswordAge(&self, lnmaxpasswordage: i32) -> windows_core::Result<()>;
    fn MaxBadPasswordsAllowed(&self) -> windows_core::Result<i32>;
    fn SetMaxBadPasswordsAllowed(&self, lnmaxbadpasswordsallowed: i32) -> windows_core::Result<()>;
    fn PasswordHistoryLength(&self) -> windows_core::Result<i32>;
    fn SetPasswordHistoryLength(&self, lnpasswordhistorylength: i32) -> windows_core::Result<()>;
    fn PasswordAttributes(&self) -> windows_core::Result<i32>;
    fn SetPasswordAttributes(&self, lnpasswordattributes: i32) -> windows_core::Result<()>;
    fn AutoUnlockInterval(&self) -> windows_core::Result<i32>;
    fn SetAutoUnlockInterval(&self, lnautounlockinterval: i32) -> windows_core::Result<()>;
    fn LockoutObservationInterval(&self) -> windows_core::Result<i32>;
    fn SetLockoutObservationInterval(&self, lnlockoutobservationinterval: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsDomain_Vtbl {
    pub const fn new<Identity: IADsDomain_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsWorkgroup<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::IsWorkgroup(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MinPasswordLength<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::MinPasswordLength(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinPasswordLength<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnminpasswordlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDomain_Impl::SetMinPasswordLength(this, core::mem::transmute_copy(&lnminpasswordlength)).into()
            }
        }
        unsafe extern "system" fn MinPasswordAge<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::MinPasswordAge(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinPasswordAge<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnminpasswordage: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDomain_Impl::SetMinPasswordAge(this, core::mem::transmute_copy(&lnminpasswordage)).into()
            }
        }
        unsafe extern "system" fn MaxPasswordAge<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::MaxPasswordAge(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxPasswordAge<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxpasswordage: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDomain_Impl::SetMaxPasswordAge(this, core::mem::transmute_copy(&lnmaxpasswordage)).into()
            }
        }
        unsafe extern "system" fn MaxBadPasswordsAllowed<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::MaxBadPasswordsAllowed(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxBadPasswordsAllowed<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxbadpasswordsallowed: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDomain_Impl::SetMaxBadPasswordsAllowed(this, core::mem::transmute_copy(&lnmaxbadpasswordsallowed)).into()
            }
        }
        unsafe extern "system" fn PasswordHistoryLength<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::PasswordHistoryLength(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPasswordHistoryLength<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnpasswordhistorylength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDomain_Impl::SetPasswordHistoryLength(this, core::mem::transmute_copy(&lnpasswordhistorylength)).into()
            }
        }
        unsafe extern "system" fn PasswordAttributes<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::PasswordAttributes(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPasswordAttributes<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnpasswordattributes: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDomain_Impl::SetPasswordAttributes(this, core::mem::transmute_copy(&lnpasswordattributes)).into()
            }
        }
        unsafe extern "system" fn AutoUnlockInterval<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::AutoUnlockInterval(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutoUnlockInterval<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnautounlockinterval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDomain_Impl::SetAutoUnlockInterval(this, core::mem::transmute_copy(&lnautounlockinterval)).into()
            }
        }
        unsafe extern "system" fn LockoutObservationInterval<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::LockoutObservationInterval(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLockoutObservationInterval<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnlockoutobservationinterval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDomain_Impl::SetLockoutObservationInterval(this, core::mem::transmute_copy(&lnlockoutobservationinterval)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            IsWorkgroup: IsWorkgroup::<Identity, OFFSET>,
            MinPasswordLength: MinPasswordLength::<Identity, OFFSET>,
            SetMinPasswordLength: SetMinPasswordLength::<Identity, OFFSET>,
            MinPasswordAge: MinPasswordAge::<Identity, OFFSET>,
            SetMinPasswordAge: SetMinPasswordAge::<Identity, OFFSET>,
            MaxPasswordAge: MaxPasswordAge::<Identity, OFFSET>,
            SetMaxPasswordAge: SetMaxPasswordAge::<Identity, OFFSET>,
            MaxBadPasswordsAllowed: MaxBadPasswordsAllowed::<Identity, OFFSET>,
            SetMaxBadPasswordsAllowed: SetMaxBadPasswordsAllowed::<Identity, OFFSET>,
            PasswordHistoryLength: PasswordHistoryLength::<Identity, OFFSET>,
            SetPasswordHistoryLength: SetPasswordHistoryLength::<Identity, OFFSET>,
            PasswordAttributes: PasswordAttributes::<Identity, OFFSET>,
            SetPasswordAttributes: SetPasswordAttributes::<Identity, OFFSET>,
            AutoUnlockInterval: AutoUnlockInterval::<Identity, OFFSET>,
            SetAutoUnlockInterval: SetAutoUnlockInterval::<Identity, OFFSET>,
            LockoutObservationInterval: LockoutObservationInterval::<Identity, OFFSET>,
            SetLockoutObservationInterval: SetLockoutObservationInterval::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsDomain as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsDomain {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsEmail, IADsEmail_Vtbl, 0x97af011a_478e_11d1_a3b4_00c04fb950dc);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsEmail {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsEmail, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsEmail {
    pub unsafe fn Type(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetType(&self, lntype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), lntype) }
    }
    pub unsafe fn Address(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Address)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetAddress(&self, bstraddress: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstraddress)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsEmail_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsEmail_Impl: super::oaidl::IDispatch_Impl {
    fn Type(&self) -> windows_core::Result<i32>;
    fn SetType(&self, lntype: i32) -> windows_core::Result<()>;
    fn Address(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetAddress(&self, bstraddress: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsEmail_Vtbl {
    pub const fn new<Identity: IADsEmail_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Type<Identity: IADsEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsEmail_Impl::Type(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetType<Identity: IADsEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lntype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsEmail_Impl::SetType(this, core::mem::transmute_copy(&lntype)).into()
            }
        }
        unsafe extern "system" fn Address<Identity: IADsEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsEmail_Impl::Address(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAddress<Identity: IADsEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstraddress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsEmail_Impl::SetAddress(this, core::mem::transmute(&bstraddress)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            Address: Address::<Identity, OFFSET>,
            SetAddress: SetAddress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsEmail as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsEmail {}
windows_core::imp::define_interface!(IADsExtension, IADsExtension_Vtbl, 0x3d35553c_d2b0_11d1_b17b_0000f87593a0);
windows_core::imp::interface_hierarchy!(IADsExtension, windows_core::IUnknown);
impl IADsExtension {
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Operate(&self, dwcode: u32, vardata1: &super::oaidl::VARIANT, vardata2: &super::oaidl::VARIANT, vardata3: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Operate)(windows_core::Interface::as_raw(self), dwcode, core::mem::transmute_copy(vardata1), core::mem::transmute_copy(vardata2), core::mem::transmute_copy(vardata3)) }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypesbase"))]
    pub unsafe fn PrivateGetIDsOfNames(&self, riid: *const windows_core::GUID, rgsznames: *const *const super::wtypesbase::OLECHAR, cnames: u32, lcid: super::winnt::LCID) -> windows_core::Result<super::oaidl::DISPID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivateGetIDsOfNames)(windows_core::Interface::as_raw(self), riid, rgsznames, cnames, lcid, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn PrivateInvoke(&self, dispidmember: super::oaidl::DISPID, riid: *const windows_core::GUID, lcid: super::winnt::LCID, wflags: u16, pdispparams: *const super::oaidl::DISPPARAMS, pvarresult: *mut super::oaidl::VARIANT, pexcepinfo: *mut super::oaidl::EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PrivateInvoke)(windows_core::Interface::as_raw(self), dispidmember, riid, lcid, wflags, pdispparams, core::mem::transmute(pvarresult), core::mem::transmute(pexcepinfo), puargerr as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IADsExtension_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Operate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::oaidl::VARIANT, super::oaidl::VARIANT, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Operate: usize,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypesbase"))]
    pub PrivateGetIDsOfNames: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const *const super::wtypesbase::OLECHAR, u32, super::winnt::LCID, *mut super::oaidl::DISPID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypesbase")))]
    PrivateGetIDsOfNames: usize,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub PrivateInvoke: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID, *const windows_core::GUID, super::winnt::LCID, u16, *const super::oaidl::DISPPARAMS, *mut super::oaidl::VARIANT, *mut super::oaidl::EXCEPINFO, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    PrivateInvoke: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsExtension_Impl: windows_core::IUnknownImpl {
    fn Operate(&self, dwcode: u32, vardata1: &super::oaidl::VARIANT, vardata2: &super::oaidl::VARIANT, vardata3: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn PrivateGetIDsOfNames(&self, riid: *const windows_core::GUID, rgsznames: *const *const super::wtypesbase::OLECHAR, cnames: u32, lcid: super::winnt::LCID) -> windows_core::Result<super::oaidl::DISPID>;
    fn PrivateInvoke(&self, dispidmember: super::oaidl::DISPID, riid: *const windows_core::GUID, lcid: super::winnt::LCID, wflags: u16, pdispparams: *const super::oaidl::DISPPARAMS, pvarresult: *mut super::oaidl::VARIANT, pexcepinfo: *mut super::oaidl::EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsExtension_Vtbl {
    pub const fn new<Identity: IADsExtension_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Operate<Identity: IADsExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcode: u32, vardata1: super::oaidl::VARIANT, vardata2: super::oaidl::VARIANT, vardata3: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsExtension_Impl::Operate(this, core::mem::transmute_copy(&dwcode), core::mem::transmute(&vardata1), core::mem::transmute(&vardata2), core::mem::transmute(&vardata3)).into()
            }
        }
        unsafe extern "system" fn PrivateGetIDsOfNames<Identity: IADsExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, rgsznames: *const *const super::wtypesbase::OLECHAR, cnames: u32, lcid: super::winnt::LCID, rgdispid: *mut super::oaidl::DISPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsExtension_Impl::PrivateGetIDsOfNames(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&rgsznames), core::mem::transmute_copy(&cnames), core::mem::transmute_copy(&lcid)) {
                    Ok(ok__) => {
                        rgdispid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivateInvoke<Identity: IADsExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispidmember: super::oaidl::DISPID, riid: *const windows_core::GUID, lcid: super::winnt::LCID, wflags: u16, pdispparams: *const super::oaidl::DISPPARAMS, pvarresult: *mut super::oaidl::VARIANT, pexcepinfo: *mut super::oaidl::EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsExtension_Impl::PrivateInvoke(this, core::mem::transmute_copy(&dispidmember), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&pexcepinfo), core::mem::transmute_copy(&puargerr)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Operate: Operate::<Identity, OFFSET>,
            PrivateGetIDsOfNames: PrivateGetIDsOfNames::<Identity, OFFSET>,
            PrivateInvoke: PrivateInvoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsExtension as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsExtension {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsFaxNumber, IADsFaxNumber_Vtbl, 0xa910dea9_4680_11d1_a3b4_00c04fb950dc);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsFaxNumber {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsFaxNumber, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsFaxNumber {
    pub unsafe fn TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TelephoneNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTelephoneNumber(&self, bstrtelephonenumber: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTelephoneNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtelephonenumber)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Parameters(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetParameters(&self, vparameters: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vparameters)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsFaxNumber_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub TelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Parameters: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetParameters: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsFaxNumber_Impl: super::oaidl::IDispatch_Impl {
    fn TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTelephoneNumber(&self, bstrtelephonenumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Parameters(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetParameters(&self, vparameters: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsFaxNumber_Vtbl {
    pub const fn new<Identity: IADsFaxNumber_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TelephoneNumber<Identity: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFaxNumber_Impl::TelephoneNumber(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtelephonenumber: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsFaxNumber_Impl::SetTelephoneNumber(this, core::mem::transmute(&bstrtelephonenumber)).into()
            }
        }
        unsafe extern "system" fn Parameters<Identity: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFaxNumber_Impl::Parameters(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetParameters<Identity: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vparameters: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsFaxNumber_Impl::SetParameters(this, core::mem::transmute(&vparameters)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            TelephoneNumber: TelephoneNumber::<Identity, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, OFFSET>,
            Parameters: Parameters::<Identity, OFFSET>,
            SetParameters: SetParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsFaxNumber as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsFaxNumber {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsFileService, IADsFileService_Vtbl, 0xa89d1900_31ca_11cf_a98a_00aa006bc149);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsFileService {
    type Target = IADsService;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsFileService, windows_core::IUnknown, super::oaidl::IDispatch, IADs, IADsService);
#[cfg(feature = "Win32_oaidl")]
impl IADsFileService {
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn MaxUserCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxUserCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxUserCount(&self, lnmaxusercount: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaxUserCount)(windows_core::Interface::as_raw(self), lnmaxusercount) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsFileService_Vtbl {
    pub base__: IADsService_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxUserCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxUserCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsFileService_Impl: IADsService_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MaxUserCount(&self) -> windows_core::Result<i32>;
    fn SetMaxUserCount(&self, lnmaxusercount: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsFileService_Vtbl {
    pub const fn new<Identity: IADsFileService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Description<Identity: IADsFileService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFileService_Impl::Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IADsFileService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsFileService_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn MaxUserCount<Identity: IADsFileService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFileService_Impl::MaxUserCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxUserCount<Identity: IADsFileService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxusercount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsFileService_Impl::SetMaxUserCount(this, core::mem::transmute_copy(&lnmaxusercount)).into()
            }
        }
        Self {
            base__: IADsService_Vtbl::new::<Identity, OFFSET>(),
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            MaxUserCount: MaxUserCount::<Identity, OFFSET>,
            SetMaxUserCount: SetMaxUserCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsFileService as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID || iid == &<IADsService as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsFileService {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsFileServiceOperations, IADsFileServiceOperations_Vtbl, 0xa02ded10_31ca_11cf_a98a_00aa006bc149);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsFileServiceOperations {
    type Target = IADsServiceOperations;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsFileServiceOperations, windows_core::IUnknown, super::oaidl::IDispatch, IADs, IADsServiceOperations);
#[cfg(feature = "Win32_oaidl")]
impl IADsFileServiceOperations {
    pub unsafe fn Sessions(&self) -> windows_core::Result<IADsCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Sessions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Resources(&self) -> windows_core::Result<IADsCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Resources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsFileServiceOperations_Vtbl {
    pub base__: IADsServiceOperations_Vtbl,
    pub Sessions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsFileServiceOperations_Impl: IADsServiceOperations_Impl {
    fn Sessions(&self) -> windows_core::Result<IADsCollection>;
    fn Resources(&self) -> windows_core::Result<IADsCollection>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsFileServiceOperations_Vtbl {
    pub const fn new<Identity: IADsFileServiceOperations_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Sessions<Identity: IADsFileServiceOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsessions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFileServiceOperations_Impl::Sessions(this) {
                    Ok(ok__) => {
                        ppsessions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Resources<Identity: IADsFileServiceOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFileServiceOperations_Impl::Resources(this) {
                    Ok(ok__) => {
                        ppresources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IADsServiceOperations_Vtbl::new::<Identity, OFFSET>(), Sessions: Sessions::<Identity, OFFSET>, Resources: Resources::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsFileServiceOperations as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID || iid == &<IADsServiceOperations as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsFileServiceOperations {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsFileShare, IADsFileShare_Vtbl, 0xeb6dcaf0_4b83_11cf_a995_00aa006bc149);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsFileShare {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsFileShare, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsFileShare {
    pub unsafe fn CurrentUserCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentUserCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn HostComputer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HostComputer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetHostComputer(&self, bstrhostcomputer: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHostComputer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrhostcomputer)) }
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPath(&self, bstrpath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpath)) }
    }
    pub unsafe fn MaxUserCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxUserCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxUserCount(&self, lnmaxusercount: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaxUserCount)(windows_core::Interface::as_raw(self), lnmaxusercount) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsFileShare_Vtbl {
    pub base__: IADs_Vtbl,
    pub CurrentUserCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HostComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHostComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxUserCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxUserCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsFileShare_Impl: IADs_Impl {
    fn CurrentUserCount(&self) -> windows_core::Result<i32>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn HostComputer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetHostComputer(&self, bstrhostcomputer: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPath(&self, bstrpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MaxUserCount(&self) -> windows_core::Result<i32>;
    fn SetMaxUserCount(&self, lnmaxusercount: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsFileShare_Vtbl {
    pub const fn new<Identity: IADsFileShare_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentUserCount<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFileShare_Impl::CurrentUserCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFileShare_Impl::Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsFileShare_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn HostComputer<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFileShare_Impl::HostComputer(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHostComputer<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhostcomputer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsFileShare_Impl::SetHostComputer(this, core::mem::transmute(&bstrhostcomputer)).into()
            }
        }
        unsafe extern "system" fn Path<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFileShare_Impl::Path(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPath<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsFileShare_Impl::SetPath(this, core::mem::transmute(&bstrpath)).into()
            }
        }
        unsafe extern "system" fn MaxUserCount<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFileShare_Impl::MaxUserCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxUserCount<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxusercount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsFileShare_Impl::SetMaxUserCount(this, core::mem::transmute_copy(&lnmaxusercount)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            CurrentUserCount: CurrentUserCount::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            HostComputer: HostComputer::<Identity, OFFSET>,
            SetHostComputer: SetHostComputer::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
            MaxUserCount: MaxUserCount::<Identity, OFFSET>,
            SetMaxUserCount: SetMaxUserCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsFileShare as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsFileShare {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsGroup, IADsGroup_Vtbl, 0x27636b00_410f_11cf_b1ff_02608c9e7553);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsGroup {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsGroup, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsGroup {
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn Members(&self) -> windows_core::Result<IADsMembers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Members)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn IsMember(&self, bstrmember: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMember)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmember), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add(&self, bstrnewitem: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnewitem)) }
    }
    pub unsafe fn Remove(&self, bstritemtoberemoved: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstritemtoberemoved)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsGroup_Vtbl {
    pub base__: IADs_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Members: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub IsMember: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    IsMember: usize,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsGroup_Impl: IADs_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Members(&self) -> windows_core::Result<IADsMembers>;
    fn IsMember(&self, bstrmember: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn Add(&self, bstrnewitem: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Remove(&self, bstritemtoberemoved: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsGroup_Vtbl {
    pub const fn new<Identity: IADsGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Description<Identity: IADsGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsGroup_Impl::Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IADsGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsGroup_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn Members<Identity: IADsGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmembers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsGroup_Impl::Members(this) {
                    Ok(ok__) => {
                        ppmembers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsMember<Identity: IADsGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmember: *mut core::ffi::c_void, bmember: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsGroup_Impl::IsMember(this, core::mem::transmute(&bstrmember)) {
                    Ok(ok__) => {
                        bmember.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IADsGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnewitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsGroup_Impl::Add(this, core::mem::transmute(&bstrnewitem)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IADsGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemtoberemoved: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsGroup_Impl::Remove(this, core::mem::transmute(&bstritemtoberemoved)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            Members: Members::<Identity, OFFSET>,
            IsMember: IsMember::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsGroup as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsGroup {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsHold, IADsHold_Vtbl, 0xb3eb3b37_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsHold {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsHold, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsHold {
    pub unsafe fn ObjectName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ObjectName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetObjectName(&self, bstrobjectname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetObjectName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrobjectname)) }
    }
    pub unsafe fn Amount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Amount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAmount(&self, lnamount: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAmount)(windows_core::Interface::as_raw(self), lnamount) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsHold_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub ObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Amount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAmount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsHold_Impl: super::oaidl::IDispatch_Impl {
    fn ObjectName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetObjectName(&self, bstrobjectname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Amount(&self) -> windows_core::Result<i32>;
    fn SetAmount(&self, lnamount: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsHold_Vtbl {
    pub const fn new<Identity: IADsHold_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ObjectName<Identity: IADsHold_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsHold_Impl::ObjectName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetObjectName<Identity: IADsHold_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrobjectname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsHold_Impl::SetObjectName(this, core::mem::transmute(&bstrobjectname)).into()
            }
        }
        unsafe extern "system" fn Amount<Identity: IADsHold_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsHold_Impl::Amount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAmount<Identity: IADsHold_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnamount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsHold_Impl::SetAmount(this, core::mem::transmute_copy(&lnamount)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ObjectName: ObjectName::<Identity, OFFSET>,
            SetObjectName: SetObjectName::<Identity, OFFSET>,
            Amount: Amount::<Identity, OFFSET>,
            SetAmount: SetAmount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsHold as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsHold {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsLargeInteger, IADsLargeInteger_Vtbl, 0x9068270b_0939_11d1_8be1_00c04fd8d503);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsLargeInteger {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsLargeInteger, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsLargeInteger {
    pub unsafe fn HighPart(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HighPart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHighPart(&self, lnhighpart: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHighPart)(windows_core::Interface::as_raw(self), lnhighpart) }
    }
    pub unsafe fn LowPart(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LowPart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLowPart(&self, lnlowpart: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLowPart)(windows_core::Interface::as_raw(self), lnlowpart) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsLargeInteger_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub HighPart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHighPart: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LowPart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLowPart: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsLargeInteger_Impl: super::oaidl::IDispatch_Impl {
    fn HighPart(&self) -> windows_core::Result<i32>;
    fn SetHighPart(&self, lnhighpart: i32) -> windows_core::Result<()>;
    fn LowPart(&self) -> windows_core::Result<i32>;
    fn SetLowPart(&self, lnlowpart: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsLargeInteger_Vtbl {
    pub const fn new<Identity: IADsLargeInteger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HighPart<Identity: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsLargeInteger_Impl::HighPart(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHighPart<Identity: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnhighpart: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsLargeInteger_Impl::SetHighPart(this, core::mem::transmute_copy(&lnhighpart)).into()
            }
        }
        unsafe extern "system" fn LowPart<Identity: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsLargeInteger_Impl::LowPart(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLowPart<Identity: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnlowpart: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsLargeInteger_Impl::SetLowPart(this, core::mem::transmute_copy(&lnlowpart)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            HighPart: HighPart::<Identity, OFFSET>,
            SetHighPart: SetHighPart::<Identity, OFFSET>,
            LowPart: LowPart::<Identity, OFFSET>,
            SetLowPart: SetLowPart::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsLargeInteger as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsLargeInteger {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsLocality, IADsLocality_Vtbl, 0xa05e03a2_effe_11cf_8abc_00c04fd8d503);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsLocality {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsLocality, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsLocality {
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn LocalityName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalityName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocalityName(&self, bstrlocalityname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLocalityName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlocalityname)) }
    }
    pub unsafe fn PostalAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PostalAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPostalAddress(&self, bstrpostaladdress: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPostalAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpostaladdress)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SeeAlso(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SeeAlso)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetSeeAlso(&self, vseealso: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSeeAlso)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vseealso)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsLocality_Vtbl {
    pub base__: IADs_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SeeAlso: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetSeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetSeeAlso: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsLocality_Impl: IADs_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LocalityName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalityName(&self, bstrlocalityname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PostalAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPostalAddress(&self, bstrpostaladdress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SeeAlso(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetSeeAlso(&self, vseealso: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsLocality_Vtbl {
    pub const fn new<Identity: IADsLocality_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Description<Identity: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsLocality_Impl::Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsLocality_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn LocalityName<Identity: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsLocality_Impl::LocalityName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalityName<Identity: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlocalityname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsLocality_Impl::SetLocalityName(this, core::mem::transmute(&bstrlocalityname)).into()
            }
        }
        unsafe extern "system" fn PostalAddress<Identity: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsLocality_Impl::PostalAddress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPostalAddress<Identity: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpostaladdress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsLocality_Impl::SetPostalAddress(this, core::mem::transmute(&bstrpostaladdress)).into()
            }
        }
        unsafe extern "system" fn SeeAlso<Identity: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsLocality_Impl::SeeAlso(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSeeAlso<Identity: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vseealso: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsLocality_Impl::SetSeeAlso(this, core::mem::transmute(&vseealso)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            LocalityName: LocalityName::<Identity, OFFSET>,
            SetLocalityName: SetLocalityName::<Identity, OFFSET>,
            PostalAddress: PostalAddress::<Identity, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, OFFSET>,
            SeeAlso: SeeAlso::<Identity, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsLocality as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsLocality {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsMembers, IADsMembers_Vtbl, 0x451a0030_72ec_11cf_b03b_00aa006e0975);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsMembers {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsMembers, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsMembers {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Filter(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Filter)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetFilter(&self, pvfilter: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFilter)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pvfilter)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsMembers_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Filter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Filter: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetFilter: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetFilter: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsMembers_Impl: super::oaidl::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Filter(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetFilter(&self, pvfilter: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsMembers_Vtbl {
    pub const fn new<Identity: IADsMembers_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IADsMembers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsMembers_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IADsMembers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsMembers_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Filter<Identity: IADsMembers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvfilter: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsMembers_Impl::Filter(this) {
                    Ok(ok__) => {
                        pvfilter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFilter<Identity: IADsMembers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvfilter: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsMembers_Impl::SetFilter(this, core::mem::transmute(&pvfilter)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Filter: Filter::<Identity, OFFSET>,
            SetFilter: SetFilter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsMembers as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsMembers {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsNameTranslate, IADsNameTranslate_Vtbl, 0xb1b272a3_3625_11d1_a3a4_00c04fb950dc);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsNameTranslate {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsNameTranslate, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsNameTranslate {
    pub unsafe fn SetChaseReferral(&self, lnchasereferral: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetChaseReferral)(windows_core::Interface::as_raw(self), lnchasereferral) }
    }
    pub unsafe fn Init(&self, lnsettype: i32, bstradspath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), lnsettype, core::mem::transmute_copy(bstradspath)) }
    }
    pub unsafe fn InitEx(&self, lnsettype: i32, bstradspath: &windows_core::BSTR, bstruserid: &windows_core::BSTR, bstrdomain: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitEx)(windows_core::Interface::as_raw(self), lnsettype, core::mem::transmute_copy(bstradspath), core::mem::transmute_copy(bstruserid), core::mem::transmute_copy(bstrdomain), core::mem::transmute_copy(bstrpassword)) }
    }
    pub unsafe fn Set(&self, lnsettype: i32, bstradspath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), lnsettype, core::mem::transmute_copy(bstradspath)) }
    }
    pub unsafe fn Get(&self, lnformattype: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), lnformattype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetEx(&self, lnformattype: i32, pvar: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEx)(windows_core::Interface::as_raw(self), lnformattype, core::mem::transmute_copy(pvar)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetEx(&self, lnformattype: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEx)(windows_core::Interface::as_raw(self), lnformattype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsNameTranslate_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub SetChaseReferral: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetEx: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetEx: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsNameTranslate_Impl: super::oaidl::IDispatch_Impl {
    fn SetChaseReferral(&self, lnchasereferral: i32) -> windows_core::Result<()>;
    fn Init(&self, lnsettype: i32, bstradspath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitEx(&self, lnsettype: i32, bstradspath: &windows_core::BSTR, bstruserid: &windows_core::BSTR, bstrdomain: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Set(&self, lnsettype: i32, bstradspath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Get(&self, lnformattype: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SetEx(&self, lnformattype: i32, pvar: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn GetEx(&self, lnformattype: i32) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsNameTranslate_Vtbl {
    pub const fn new<Identity: IADsNameTranslate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetChaseReferral<Identity: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnchasereferral: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsNameTranslate_Impl::SetChaseReferral(this, core::mem::transmute_copy(&lnchasereferral)).into()
            }
        }
        unsafe extern "system" fn Init<Identity: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnsettype: i32, bstradspath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsNameTranslate_Impl::Init(this, core::mem::transmute_copy(&lnsettype), core::mem::transmute(&bstradspath)).into()
            }
        }
        unsafe extern "system" fn InitEx<Identity: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnsettype: i32, bstradspath: *mut core::ffi::c_void, bstruserid: *mut core::ffi::c_void, bstrdomain: *mut core::ffi::c_void, bstrpassword: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsNameTranslate_Impl::InitEx(this, core::mem::transmute_copy(&lnsettype), core::mem::transmute(&bstradspath), core::mem::transmute(&bstruserid), core::mem::transmute(&bstrdomain), core::mem::transmute(&bstrpassword)).into()
            }
        }
        unsafe extern "system" fn Set<Identity: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnsettype: i32, bstradspath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsNameTranslate_Impl::Set(this, core::mem::transmute_copy(&lnsettype), core::mem::transmute(&bstradspath)).into()
            }
        }
        unsafe extern "system" fn Get<Identity: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnformattype: i32, pbstradspath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsNameTranslate_Impl::Get(this, core::mem::transmute_copy(&lnformattype)) {
                    Ok(ok__) => {
                        pbstradspath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEx<Identity: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnformattype: i32, pvar: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsNameTranslate_Impl::SetEx(this, core::mem::transmute_copy(&lnformattype), core::mem::transmute(&pvar)).into()
            }
        }
        unsafe extern "system" fn GetEx<Identity: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnformattype: i32, pvar: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsNameTranslate_Impl::GetEx(this, core::mem::transmute_copy(&lnformattype)) {
                    Ok(ok__) => {
                        pvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetChaseReferral: SetChaseReferral::<Identity, OFFSET>,
            Init: Init::<Identity, OFFSET>,
            InitEx: InitEx::<Identity, OFFSET>,
            Set: Set::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            SetEx: SetEx::<Identity, OFFSET>,
            GetEx: GetEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsNameTranslate as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsNameTranslate {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsNamespaces, IADsNamespaces_Vtbl, 0x28b96ba0_b330_11cf_a9ad_00aa006bc149);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsNamespaces {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsNamespaces, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsNamespaces {
    pub unsafe fn DefaultContainer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultContainer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDefaultContainer(&self, bstrdefaultcontainer: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultContainer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdefaultcontainer)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsNamespaces_Vtbl {
    pub base__: IADs_Vtbl,
    pub DefaultContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsNamespaces_Impl: IADs_Impl {
    fn DefaultContainer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDefaultContainer(&self, bstrdefaultcontainer: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsNamespaces_Vtbl {
    pub const fn new<Identity: IADsNamespaces_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DefaultContainer<Identity: IADsNamespaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsNamespaces_Impl::DefaultContainer(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultContainer<Identity: IADsNamespaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdefaultcontainer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsNamespaces_Impl::SetDefaultContainer(this, core::mem::transmute(&bstrdefaultcontainer)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            DefaultContainer: DefaultContainer::<Identity, OFFSET>,
            SetDefaultContainer: SetDefaultContainer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsNamespaces as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsNamespaces {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsNetAddress, IADsNetAddress_Vtbl, 0xb21a50a9_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsNetAddress {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsNetAddress, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsNetAddress {
    pub unsafe fn AddressType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddressType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAddressType(&self, lnaddresstype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAddressType)(windows_core::Interface::as_raw(self), lnaddresstype) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Address(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Address)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetAddress(&self, vaddress: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vaddress)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsNetAddress_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub AddressType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAddressType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Address: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetAddress: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetAddress: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsNetAddress_Impl: super::oaidl::IDispatch_Impl {
    fn AddressType(&self) -> windows_core::Result<i32>;
    fn SetAddressType(&self, lnaddresstype: i32) -> windows_core::Result<()>;
    fn Address(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetAddress(&self, vaddress: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsNetAddress_Vtbl {
    pub const fn new<Identity: IADsNetAddress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddressType<Identity: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsNetAddress_Impl::AddressType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAddressType<Identity: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnaddresstype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsNetAddress_Impl::SetAddressType(this, core::mem::transmute_copy(&lnaddresstype)).into()
            }
        }
        unsafe extern "system" fn Address<Identity: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsNetAddress_Impl::Address(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAddress<Identity: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vaddress: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsNetAddress_Impl::SetAddress(this, core::mem::transmute(&vaddress)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AddressType: AddressType::<Identity, OFFSET>,
            SetAddressType: SetAddressType::<Identity, OFFSET>,
            Address: Address::<Identity, OFFSET>,
            SetAddress: SetAddress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsNetAddress as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsNetAddress {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsO, IADsO_Vtbl, 0xa1cd2dc6_effe_11cf_8abc_00c04fd8d503);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsO {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsO, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsO {
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn LocalityName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalityName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocalityName(&self, bstrlocalityname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLocalityName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlocalityname)) }
    }
    pub unsafe fn PostalAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PostalAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPostalAddress(&self, bstrpostaladdress: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPostalAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpostaladdress)) }
    }
    pub unsafe fn TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TelephoneNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTelephoneNumber(&self, bstrtelephonenumber: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTelephoneNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtelephonenumber)) }
    }
    pub unsafe fn FaxNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FaxNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFaxNumber(&self, bstrfaxnumber: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFaxNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfaxnumber)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SeeAlso(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SeeAlso)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetSeeAlso(&self, vseealso: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSeeAlso)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vseealso)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsO_Vtbl {
    pub base__: IADs_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SeeAlso: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetSeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetSeeAlso: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsO_Impl: IADs_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LocalityName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalityName(&self, bstrlocalityname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PostalAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPostalAddress(&self, bstrpostaladdress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTelephoneNumber(&self, bstrtelephonenumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FaxNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFaxNumber(&self, bstrfaxnumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SeeAlso(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetSeeAlso(&self, vseealso: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsO_Vtbl {
    pub const fn new<Identity: IADsO_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Description<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsO_Impl::Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsO_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn LocalityName<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsO_Impl::LocalityName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalityName<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlocalityname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsO_Impl::SetLocalityName(this, core::mem::transmute(&bstrlocalityname)).into()
            }
        }
        unsafe extern "system" fn PostalAddress<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsO_Impl::PostalAddress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPostalAddress<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpostaladdress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsO_Impl::SetPostalAddress(this, core::mem::transmute(&bstrpostaladdress)).into()
            }
        }
        unsafe extern "system" fn TelephoneNumber<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsO_Impl::TelephoneNumber(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtelephonenumber: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsO_Impl::SetTelephoneNumber(this, core::mem::transmute(&bstrtelephonenumber)).into()
            }
        }
        unsafe extern "system" fn FaxNumber<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsO_Impl::FaxNumber(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfaxnumber: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsO_Impl::SetFaxNumber(this, core::mem::transmute(&bstrfaxnumber)).into()
            }
        }
        unsafe extern "system" fn SeeAlso<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsO_Impl::SeeAlso(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSeeAlso<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vseealso: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsO_Impl::SetSeeAlso(this, core::mem::transmute(&vseealso)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            LocalityName: LocalityName::<Identity, OFFSET>,
            SetLocalityName: SetLocalityName::<Identity, OFFSET>,
            PostalAddress: PostalAddress::<Identity, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, OFFSET>,
            TelephoneNumber: TelephoneNumber::<Identity, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, OFFSET>,
            FaxNumber: FaxNumber::<Identity, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, OFFSET>,
            SeeAlso: SeeAlso::<Identity, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsO as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsO {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsOU, IADsOU_Vtbl, 0xa2f733b8_effe_11cf_8abc_00c04fd8d503);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsOU {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsOU, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsOU {
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn LocalityName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalityName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocalityName(&self, bstrlocalityname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLocalityName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlocalityname)) }
    }
    pub unsafe fn PostalAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PostalAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPostalAddress(&self, bstrpostaladdress: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPostalAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpostaladdress)) }
    }
    pub unsafe fn TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TelephoneNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTelephoneNumber(&self, bstrtelephonenumber: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTelephoneNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtelephonenumber)) }
    }
    pub unsafe fn FaxNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FaxNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFaxNumber(&self, bstrfaxnumber: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFaxNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfaxnumber)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SeeAlso(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SeeAlso)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetSeeAlso(&self, vseealso: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSeeAlso)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vseealso)) }
    }
    pub unsafe fn BusinessCategory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BusinessCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetBusinessCategory(&self, bstrbusinesscategory: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBusinessCategory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbusinesscategory)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsOU_Vtbl {
    pub base__: IADs_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SeeAlso: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetSeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetSeeAlso: usize,
    pub BusinessCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBusinessCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsOU_Impl: IADs_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LocalityName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalityName(&self, bstrlocalityname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PostalAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPostalAddress(&self, bstrpostaladdress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTelephoneNumber(&self, bstrtelephonenumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FaxNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFaxNumber(&self, bstrfaxnumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SeeAlso(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetSeeAlso(&self, vseealso: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn BusinessCategory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetBusinessCategory(&self, bstrbusinesscategory: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsOU_Vtbl {
    pub const fn new<Identity: IADsOU_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Description<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsOU_Impl::Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsOU_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn LocalityName<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsOU_Impl::LocalityName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalityName<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlocalityname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsOU_Impl::SetLocalityName(this, core::mem::transmute(&bstrlocalityname)).into()
            }
        }
        unsafe extern "system" fn PostalAddress<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsOU_Impl::PostalAddress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPostalAddress<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpostaladdress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsOU_Impl::SetPostalAddress(this, core::mem::transmute(&bstrpostaladdress)).into()
            }
        }
        unsafe extern "system" fn TelephoneNumber<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsOU_Impl::TelephoneNumber(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtelephonenumber: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsOU_Impl::SetTelephoneNumber(this, core::mem::transmute(&bstrtelephonenumber)).into()
            }
        }
        unsafe extern "system" fn FaxNumber<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsOU_Impl::FaxNumber(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfaxnumber: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsOU_Impl::SetFaxNumber(this, core::mem::transmute(&bstrfaxnumber)).into()
            }
        }
        unsafe extern "system" fn SeeAlso<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsOU_Impl::SeeAlso(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSeeAlso<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vseealso: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsOU_Impl::SetSeeAlso(this, core::mem::transmute(&vseealso)).into()
            }
        }
        unsafe extern "system" fn BusinessCategory<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsOU_Impl::BusinessCategory(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBusinessCategory<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbusinesscategory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsOU_Impl::SetBusinessCategory(this, core::mem::transmute(&bstrbusinesscategory)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            LocalityName: LocalityName::<Identity, OFFSET>,
            SetLocalityName: SetLocalityName::<Identity, OFFSET>,
            PostalAddress: PostalAddress::<Identity, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, OFFSET>,
            TelephoneNumber: TelephoneNumber::<Identity, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, OFFSET>,
            FaxNumber: FaxNumber::<Identity, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, OFFSET>,
            SeeAlso: SeeAlso::<Identity, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, OFFSET>,
            BusinessCategory: BusinessCategory::<Identity, OFFSET>,
            SetBusinessCategory: SetBusinessCategory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsOU as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsOU {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsObjectOptions, IADsObjectOptions_Vtbl, 0x46f14fda_232b_11d1_a808_00c04fd8d5a8);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsObjectOptions {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsObjectOptions, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsObjectOptions {
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetOption(&self, lnoption: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOption)(windows_core::Interface::as_raw(self), lnoption, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetOption(&self, lnoption: i32, vvalue: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOption)(windows_core::Interface::as_raw(self), lnoption, core::mem::transmute_copy(vvalue)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsObjectOptions_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetOption: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetOption: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetOption: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetOption: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsObjectOptions_Impl: super::oaidl::IDispatch_Impl {
    fn GetOption(&self, lnoption: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetOption(&self, lnoption: i32, vvalue: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsObjectOptions_Vtbl {
    pub const fn new<Identity: IADsObjectOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOption<Identity: IADsObjectOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnoption: i32, pvvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsObjectOptions_Impl::GetOption(this, core::mem::transmute_copy(&lnoption)) {
                    Ok(ok__) => {
                        pvvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOption<Identity: IADsObjectOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnoption: i32, vvalue: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsObjectOptions_Impl::SetOption(this, core::mem::transmute_copy(&lnoption), core::mem::transmute(&vvalue)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetOption: GetOption::<Identity, OFFSET>,
            SetOption: SetOption::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsObjectOptions as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsObjectOptions {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsOctetList, IADsOctetList_Vtbl, 0x7b28b80f_4680_11d1_a3b4_00c04fb950dc);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsOctetList {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsOctetList, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsOctetList {
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn OctetList(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OctetList)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetOctetList(&self, voctetlist: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOctetList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(voctetlist)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsOctetList_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub OctetList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    OctetList: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetOctetList: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetOctetList: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsOctetList_Impl: super::oaidl::IDispatch_Impl {
    fn OctetList(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetOctetList(&self, voctetlist: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsOctetList_Vtbl {
    pub const fn new<Identity: IADsOctetList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OctetList<Identity: IADsOctetList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsOctetList_Impl::OctetList(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOctetList<Identity: IADsOctetList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, voctetlist: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsOctetList_Impl::SetOctetList(this, core::mem::transmute(&voctetlist)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            OctetList: OctetList::<Identity, OFFSET>,
            SetOctetList: SetOctetList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsOctetList as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsOctetList {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsOpenDSObject, IADsOpenDSObject_Vtbl, 0xddf2891e_0f9c_11d0_8ad4_00c04fd8d503);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsOpenDSObject {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsOpenDSObject, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsOpenDSObject {
    pub unsafe fn OpenDSObject(&self, lpszdnname: &windows_core::BSTR, lpszusername: &windows_core::BSTR, lpszpassword: &windows_core::BSTR, lnreserved: i32) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenDSObject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lpszdnname), core::mem::transmute_copy(lpszusername), core::mem::transmute_copy(lpszpassword), lnreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsOpenDSObject_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub OpenDSObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsOpenDSObject_Impl: super::oaidl::IDispatch_Impl {
    fn OpenDSObject(&self, lpszdnname: &windows_core::BSTR, lpszusername: &windows_core::BSTR, lpszpassword: &windows_core::BSTR, lnreserved: i32) -> windows_core::Result<super::oaidl::IDispatch>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsOpenDSObject_Vtbl {
    pub const fn new<Identity: IADsOpenDSObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenDSObject<Identity: IADsOpenDSObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszdnname: *mut core::ffi::c_void, lpszusername: *mut core::ffi::c_void, lpszpassword: *mut core::ffi::c_void, lnreserved: i32, ppoledsobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsOpenDSObject_Impl::OpenDSObject(this, core::mem::transmute(&lpszdnname), core::mem::transmute(&lpszusername), core::mem::transmute(&lpszpassword), core::mem::transmute_copy(&lnreserved)) {
                    Ok(ok__) => {
                        ppoledsobj.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), OpenDSObject: OpenDSObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsOpenDSObject as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsOpenDSObject {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsPath, IADsPath_Vtbl, 0xb287fcd5_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsPath {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsPath, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsPath {
    pub unsafe fn Type(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetType(&self, lntype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), lntype) }
    }
    pub unsafe fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetVolumeName(&self, bstrvolumename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVolumeName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrvolumename)) }
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPath(&self, bstrpath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpath)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPath_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub VolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsPath_Impl: super::oaidl::IDispatch_Impl {
    fn Type(&self) -> windows_core::Result<i32>;
    fn SetType(&self, lntype: i32) -> windows_core::Result<()>;
    fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetVolumeName(&self, bstrvolumename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPath(&self, bstrpath: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsPath_Vtbl {
    pub const fn new<Identity: IADsPath_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Type<Identity: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPath_Impl::Type(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetType<Identity: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lntype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPath_Impl::SetType(this, core::mem::transmute_copy(&lntype)).into()
            }
        }
        unsafe extern "system" fn VolumeName<Identity: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPath_Impl::VolumeName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVolumeName<Identity: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrvolumename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPath_Impl::SetVolumeName(this, core::mem::transmute(&bstrvolumename)).into()
            }
        }
        unsafe extern "system" fn Path<Identity: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPath_Impl::Path(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPath<Identity: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPath_Impl::SetPath(this, core::mem::transmute(&bstrpath)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            VolumeName: VolumeName::<Identity, OFFSET>,
            SetVolumeName: SetVolumeName::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPath as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsPath {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsPathname, IADsPathname_Vtbl, 0xd592aed4_f420_11d0_a36e_00c04fb950dc);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsPathname {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsPathname, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsPathname {
    pub unsafe fn Set(&self, bstradspath: &windows_core::BSTR, lnsettype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstradspath), lnsettype) }
    }
    pub unsafe fn SetDisplayType(&self, lndisplaytype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayType)(windows_core::Interface::as_raw(self), lndisplaytype) }
    }
    pub unsafe fn Retrieve(&self, lnformattype: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Retrieve)(windows_core::Interface::as_raw(self), lnformattype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetNumElements(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumElements)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetElement(&self, lnelementindex: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), lnelementindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AddLeafElement(&self, bstrleafelement: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddLeafElement)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrleafelement)) }
    }
    pub unsafe fn RemoveLeafElement(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveLeafElement)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CopyPath(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEscapedElement(&self, lnreserved: i32, bstrinstr: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEscapedElement)(windows_core::Interface::as_raw(self), lnreserved, core::mem::transmute_copy(bstrinstr), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn EscapedMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EscapedMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEscapedMode(&self, lnescapedmode: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEscapedMode)(windows_core::Interface::as_raw(self), lnescapedmode) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPathname_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetDisplayType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Retrieve: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNumElements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddLeafElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveLeafElement: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEscapedElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EscapedMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEscapedMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsPathname_Impl: super::oaidl::IDispatch_Impl {
    fn Set(&self, bstradspath: &windows_core::BSTR, lnsettype: i32) -> windows_core::Result<()>;
    fn SetDisplayType(&self, lndisplaytype: i32) -> windows_core::Result<()>;
    fn Retrieve(&self, lnformattype: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetNumElements(&self) -> windows_core::Result<i32>;
    fn GetElement(&self, lnelementindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn AddLeafElement(&self, bstrleafelement: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveLeafElement(&self) -> windows_core::Result<()>;
    fn CopyPath(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn GetEscapedElement(&self, lnreserved: i32, bstrinstr: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn EscapedMode(&self) -> windows_core::Result<i32>;
    fn SetEscapedMode(&self, lnescapedmode: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsPathname_Vtbl {
    pub const fn new<Identity: IADsPathname_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Set<Identity: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradspath: *mut core::ffi::c_void, lnsettype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPathname_Impl::Set(this, core::mem::transmute(&bstradspath), core::mem::transmute_copy(&lnsettype)).into()
            }
        }
        unsafe extern "system" fn SetDisplayType<Identity: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lndisplaytype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPathname_Impl::SetDisplayType(this, core::mem::transmute_copy(&lndisplaytype)).into()
            }
        }
        unsafe extern "system" fn Retrieve<Identity: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnformattype: i32, pbstradspath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPathname_Impl::Retrieve(this, core::mem::transmute_copy(&lnformattype)) {
                    Ok(ok__) => {
                        pbstradspath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNumElements<Identity: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plnnumpathelements: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPathname_Impl::GetNumElements(this) {
                    Ok(ok__) => {
                        plnnumpathelements.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetElement<Identity: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnelementindex: i32, pbstrelement: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPathname_Impl::GetElement(this, core::mem::transmute_copy(&lnelementindex)) {
                    Ok(ok__) => {
                        pbstrelement.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddLeafElement<Identity: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrleafelement: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPathname_Impl::AddLeafElement(this, core::mem::transmute(&bstrleafelement)).into()
            }
        }
        unsafe extern "system" fn RemoveLeafElement<Identity: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPathname_Impl::RemoveLeafElement(this).into()
            }
        }
        unsafe extern "system" fn CopyPath<Identity: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppadspath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPathname_Impl::CopyPath(this) {
                    Ok(ok__) => {
                        ppadspath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEscapedElement<Identity: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnreserved: i32, bstrinstr: *mut core::ffi::c_void, pbstroutstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPathname_Impl::GetEscapedElement(this, core::mem::transmute_copy(&lnreserved), core::mem::transmute(&bstrinstr)) {
                    Ok(ok__) => {
                        pbstroutstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EscapedMode<Identity: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPathname_Impl::EscapedMode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEscapedMode<Identity: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnescapedmode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPathname_Impl::SetEscapedMode(this, core::mem::transmute_copy(&lnescapedmode)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Set: Set::<Identity, OFFSET>,
            SetDisplayType: SetDisplayType::<Identity, OFFSET>,
            Retrieve: Retrieve::<Identity, OFFSET>,
            GetNumElements: GetNumElements::<Identity, OFFSET>,
            GetElement: GetElement::<Identity, OFFSET>,
            AddLeafElement: AddLeafElement::<Identity, OFFSET>,
            RemoveLeafElement: RemoveLeafElement::<Identity, OFFSET>,
            CopyPath: CopyPath::<Identity, OFFSET>,
            GetEscapedElement: GetEscapedElement::<Identity, OFFSET>,
            EscapedMode: EscapedMode::<Identity, OFFSET>,
            SetEscapedMode: SetEscapedMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPathname as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsPathname {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsPostalAddress, IADsPostalAddress_Vtbl, 0x7adecf29_4680_11d1_a3b4_00c04fb950dc);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsPostalAddress {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsPostalAddress, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsPostalAddress {
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn PostalAddress(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PostalAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetPostalAddress(&self, vpostaladdress: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPostalAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vpostaladdress)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPostalAddress_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub PostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    PostalAddress: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetPostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetPostalAddress: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsPostalAddress_Impl: super::oaidl::IDispatch_Impl {
    fn PostalAddress(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetPostalAddress(&self, vpostaladdress: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsPostalAddress_Vtbl {
    pub const fn new<Identity: IADsPostalAddress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PostalAddress<Identity: IADsPostalAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPostalAddress_Impl::PostalAddress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPostalAddress<Identity: IADsPostalAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vpostaladdress: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPostalAddress_Impl::SetPostalAddress(this, core::mem::transmute(&vpostaladdress)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            PostalAddress: PostalAddress::<Identity, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPostalAddress as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsPostalAddress {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsPrintJob, IADsPrintJob_Vtbl, 0x32fb6780_1ed0_11cf_a988_00aa006bc149);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsPrintJob {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsPrintJob, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsPrintJob {
    pub unsafe fn HostPrintQueue(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HostPrintQueue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn User(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).User)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UserPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn TimeSubmitted(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TimeSubmitted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TotalPages(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalPages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Size(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPriority(&self, lnpriority: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), lnpriority) }
    }
    pub unsafe fn StartTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStartTime(&self, dastarttime: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStartTime)(windows_core::Interface::as_raw(self), dastarttime) }
    }
    pub unsafe fn UntilTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UntilTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUntilTime(&self, dauntiltime: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUntilTime)(windows_core::Interface::as_raw(self), dauntiltime) }
    }
    pub unsafe fn Notify(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetNotify(&self, bstrnotify: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNotify)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnotify)) }
    }
    pub unsafe fn NotifyPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NotifyPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetNotifyPath(&self, bstrnotifypath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNotifyPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnotifypath)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPrintJob_Vtbl {
    pub base__: IADs_Vtbl,
    pub HostPrintQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TimeSubmitted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub TotalPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub UntilTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetUntilTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNotifyPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsPrintJob_Impl: IADs_Impl {
    fn HostPrintQueue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn User(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TimeSubmitted(&self) -> windows_core::Result<f64>;
    fn TotalPages(&self) -> windows_core::Result<i32>;
    fn Size(&self) -> windows_core::Result<i32>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Priority(&self) -> windows_core::Result<i32>;
    fn SetPriority(&self, lnpriority: i32) -> windows_core::Result<()>;
    fn StartTime(&self) -> windows_core::Result<f64>;
    fn SetStartTime(&self, dastarttime: f64) -> windows_core::Result<()>;
    fn UntilTime(&self) -> windows_core::Result<f64>;
    fn SetUntilTime(&self, dauntiltime: f64) -> windows_core::Result<()>;
    fn Notify(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetNotify(&self, bstrnotify: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NotifyPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetNotifyPath(&self, bstrnotifypath: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsPrintJob_Vtbl {
    pub const fn new<Identity: IADsPrintJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HostPrintQueue<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::HostPrintQueue(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn User<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::User(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserPath<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::UserPath(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TimeSubmitted<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::TimeSubmitted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TotalPages<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::TotalPages(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Size<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::Size(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintJob_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn Priority<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::Priority(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnpriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintJob_Impl::SetPriority(this, core::mem::transmute_copy(&lnpriority)).into()
            }
        }
        unsafe extern "system" fn StartTime<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::StartTime(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dastarttime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintJob_Impl::SetStartTime(this, core::mem::transmute_copy(&dastarttime)).into()
            }
        }
        unsafe extern "system" fn UntilTime<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::UntilTime(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUntilTime<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dauntiltime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintJob_Impl::SetUntilTime(this, core::mem::transmute_copy(&dauntiltime)).into()
            }
        }
        unsafe extern "system" fn Notify<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::Notify(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNotify<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnotify: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintJob_Impl::SetNotify(this, core::mem::transmute(&bstrnotify)).into()
            }
        }
        unsafe extern "system" fn NotifyPath<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::NotifyPath(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNotifyPath<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnotifypath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintJob_Impl::SetNotifyPath(this, core::mem::transmute(&bstrnotifypath)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            HostPrintQueue: HostPrintQueue::<Identity, OFFSET>,
            User: User::<Identity, OFFSET>,
            UserPath: UserPath::<Identity, OFFSET>,
            TimeSubmitted: TimeSubmitted::<Identity, OFFSET>,
            TotalPages: TotalPages::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            StartTime: StartTime::<Identity, OFFSET>,
            SetStartTime: SetStartTime::<Identity, OFFSET>,
            UntilTime: UntilTime::<Identity, OFFSET>,
            SetUntilTime: SetUntilTime::<Identity, OFFSET>,
            Notify: Notify::<Identity, OFFSET>,
            SetNotify: SetNotify::<Identity, OFFSET>,
            NotifyPath: NotifyPath::<Identity, OFFSET>,
            SetNotifyPath: SetNotifyPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPrintJob as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsPrintJob {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsPrintJobOperations, IADsPrintJobOperations_Vtbl, 0x9a52db30_1ecf_11cf_a988_00aa006bc149);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsPrintJobOperations {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsPrintJobOperations, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsPrintJobOperations {
    pub unsafe fn Status(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TimeElapsed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TimeElapsed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PagesPrinted(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PagesPrinted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Position(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPosition(&self, lnposition: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPosition)(windows_core::Interface::as_raw(self), lnposition) }
    }
    pub unsafe fn Pause(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Resume(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPrintJobOperations_Vtbl {
    pub base__: IADs_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TimeElapsed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PagesPrinted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsPrintJobOperations_Impl: IADs_Impl {
    fn Status(&self) -> windows_core::Result<i32>;
    fn TimeElapsed(&self) -> windows_core::Result<i32>;
    fn PagesPrinted(&self) -> windows_core::Result<i32>;
    fn Position(&self) -> windows_core::Result<i32>;
    fn SetPosition(&self, lnposition: i32) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsPrintJobOperations_Vtbl {
    pub const fn new<Identity: IADsPrintJobOperations_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Status<Identity: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJobOperations_Impl::Status(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TimeElapsed<Identity: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJobOperations_Impl::TimeElapsed(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PagesPrinted<Identity: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJobOperations_Impl::PagesPrinted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Position<Identity: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJobOperations_Impl::Position(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPosition<Identity: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnposition: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintJobOperations_Impl::SetPosition(this, core::mem::transmute_copy(&lnposition)).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintJobOperations_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintJobOperations_Impl::Resume(this).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            Status: Status::<Identity, OFFSET>,
            TimeElapsed: TimeElapsed::<Identity, OFFSET>,
            PagesPrinted: PagesPrinted::<Identity, OFFSET>,
            Position: Position::<Identity, OFFSET>,
            SetPosition: SetPosition::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPrintJobOperations as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsPrintJobOperations {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsPrintQueue, IADsPrintQueue_Vtbl, 0xb15160d0_1226_11cf_a985_00aa006bc149);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsPrintQueue {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsPrintQueue, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsPrintQueue {
    pub unsafe fn PrinterPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrinterPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPrinterPath(&self, bstrprinterpath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrinterPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprinterpath)) }
    }
    pub unsafe fn Model(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Model)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetModel(&self, bstrmodel: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetModel)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmodel)) }
    }
    pub unsafe fn Datatype(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Datatype)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDatatype(&self, bstrdatatype: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDatatype)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdatatype)) }
    }
    pub unsafe fn PrintProcessor(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrintProcessor)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPrintProcessor(&self, bstrprintprocessor: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrintProcessor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprintprocessor)) }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn Location(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Location)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocation(&self, bstrlocation: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLocation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlocation)) }
    }
    pub unsafe fn StartTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStartTime(&self, dastarttime: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStartTime)(windows_core::Interface::as_raw(self), dastarttime) }
    }
    pub unsafe fn UntilTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UntilTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUntilTime(&self, dauntiltime: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUntilTime)(windows_core::Interface::as_raw(self), dauntiltime) }
    }
    pub unsafe fn DefaultJobPriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultJobPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDefaultJobPriority(&self, lndefaultjobpriority: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultJobPriority)(windows_core::Interface::as_raw(self), lndefaultjobpriority) }
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPriority(&self, lnpriority: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), lnpriority) }
    }
    pub unsafe fn BannerPage(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BannerPage)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetBannerPage(&self, bstrbannerpage: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBannerPage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbannerpage)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn PrintDevices(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrintDevices)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetPrintDevices(&self, vprintdevices: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrintDevices)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vprintdevices)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn NetAddresses(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NetAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetNetAddresses(&self, vnetaddresses: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNetAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vnetaddresses)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPrintQueue_Vtbl {
    pub base__: IADs_Vtbl,
    pub PrinterPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrinterPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Model: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Datatype: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDatatype: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrintProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrintProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Location: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub UntilTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetUntilTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub DefaultJobPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDefaultJobPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BannerPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBannerPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub PrintDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    PrintDevices: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetPrintDevices: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetPrintDevices: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub NetAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    NetAddresses: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetNetAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetNetAddresses: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsPrintQueue_Impl: IADs_Impl {
    fn PrinterPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPrinterPath(&self, bstrprinterpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Model(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetModel(&self, bstrmodel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Datatype(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDatatype(&self, bstrdatatype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PrintProcessor(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPrintProcessor(&self, bstrprintprocessor: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Location(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocation(&self, bstrlocation: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StartTime(&self) -> windows_core::Result<f64>;
    fn SetStartTime(&self, dastarttime: f64) -> windows_core::Result<()>;
    fn UntilTime(&self) -> windows_core::Result<f64>;
    fn SetUntilTime(&self, dauntiltime: f64) -> windows_core::Result<()>;
    fn DefaultJobPriority(&self) -> windows_core::Result<i32>;
    fn SetDefaultJobPriority(&self, lndefaultjobpriority: i32) -> windows_core::Result<()>;
    fn Priority(&self) -> windows_core::Result<i32>;
    fn SetPriority(&self, lnpriority: i32) -> windows_core::Result<()>;
    fn BannerPage(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetBannerPage(&self, bstrbannerpage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PrintDevices(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetPrintDevices(&self, vprintdevices: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn NetAddresses(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetNetAddresses(&self, vnetaddresses: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsPrintQueue_Vtbl {
    pub const fn new<Identity: IADsPrintQueue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrinterPath<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::PrinterPath(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrinterPath<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprinterpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::SetPrinterPath(this, core::mem::transmute(&bstrprinterpath)).into()
            }
        }
        unsafe extern "system" fn Model<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::Model(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetModel<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmodel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::SetModel(this, core::mem::transmute(&bstrmodel)).into()
            }
        }
        unsafe extern "system" fn Datatype<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::Datatype(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDatatype<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdatatype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::SetDatatype(this, core::mem::transmute(&bstrdatatype)).into()
            }
        }
        unsafe extern "system" fn PrintProcessor<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::PrintProcessor(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrintProcessor<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprintprocessor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::SetPrintProcessor(this, core::mem::transmute(&bstrprintprocessor)).into()
            }
        }
        unsafe extern "system" fn Description<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn Location<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::Location(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocation<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlocation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::SetLocation(this, core::mem::transmute(&bstrlocation)).into()
            }
        }
        unsafe extern "system" fn StartTime<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::StartTime(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dastarttime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::SetStartTime(this, core::mem::transmute_copy(&dastarttime)).into()
            }
        }
        unsafe extern "system" fn UntilTime<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::UntilTime(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUntilTime<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dauntiltime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::SetUntilTime(this, core::mem::transmute_copy(&dauntiltime)).into()
            }
        }
        unsafe extern "system" fn DefaultJobPriority<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::DefaultJobPriority(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultJobPriority<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lndefaultjobpriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::SetDefaultJobPriority(this, core::mem::transmute_copy(&lndefaultjobpriority)).into()
            }
        }
        unsafe extern "system" fn Priority<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::Priority(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnpriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::SetPriority(this, core::mem::transmute_copy(&lnpriority)).into()
            }
        }
        unsafe extern "system" fn BannerPage<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::BannerPage(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBannerPage<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbannerpage: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::SetBannerPage(this, core::mem::transmute(&bstrbannerpage)).into()
            }
        }
        unsafe extern "system" fn PrintDevices<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::PrintDevices(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrintDevices<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vprintdevices: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::SetPrintDevices(this, core::mem::transmute(&vprintdevices)).into()
            }
        }
        unsafe extern "system" fn NetAddresses<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::NetAddresses(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNetAddresses<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vnetaddresses: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::SetNetAddresses(this, core::mem::transmute(&vnetaddresses)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            PrinterPath: PrinterPath::<Identity, OFFSET>,
            SetPrinterPath: SetPrinterPath::<Identity, OFFSET>,
            Model: Model::<Identity, OFFSET>,
            SetModel: SetModel::<Identity, OFFSET>,
            Datatype: Datatype::<Identity, OFFSET>,
            SetDatatype: SetDatatype::<Identity, OFFSET>,
            PrintProcessor: PrintProcessor::<Identity, OFFSET>,
            SetPrintProcessor: SetPrintProcessor::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            Location: Location::<Identity, OFFSET>,
            SetLocation: SetLocation::<Identity, OFFSET>,
            StartTime: StartTime::<Identity, OFFSET>,
            SetStartTime: SetStartTime::<Identity, OFFSET>,
            UntilTime: UntilTime::<Identity, OFFSET>,
            SetUntilTime: SetUntilTime::<Identity, OFFSET>,
            DefaultJobPriority: DefaultJobPriority::<Identity, OFFSET>,
            SetDefaultJobPriority: SetDefaultJobPriority::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            BannerPage: BannerPage::<Identity, OFFSET>,
            SetBannerPage: SetBannerPage::<Identity, OFFSET>,
            PrintDevices: PrintDevices::<Identity, OFFSET>,
            SetPrintDevices: SetPrintDevices::<Identity, OFFSET>,
            NetAddresses: NetAddresses::<Identity, OFFSET>,
            SetNetAddresses: SetNetAddresses::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPrintQueue as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsPrintQueue {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsPrintQueueOperations, IADsPrintQueueOperations_Vtbl, 0x124be5c0_156e_11cf_a986_00aa006bc149);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsPrintQueueOperations {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsPrintQueueOperations, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsPrintQueueOperations {
    pub unsafe fn Status(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PrintJobs(&self) -> windows_core::Result<IADsCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrintJobs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Pause(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Resume(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Purge(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Purge)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPrintQueueOperations_Vtbl {
    pub base__: IADs_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PrintJobs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Purge: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsPrintQueueOperations_Impl: IADs_Impl {
    fn Status(&self) -> windows_core::Result<i32>;
    fn PrintJobs(&self) -> windows_core::Result<IADsCollection>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn Purge(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsPrintQueueOperations_Vtbl {
    pub const fn new<Identity: IADsPrintQueueOperations_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Status<Identity: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueueOperations_Impl::Status(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrintJobs<Identity: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueueOperations_Impl::PrintJobs(this) {
                    Ok(ok__) => {
                        pobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Pause<Identity: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueueOperations_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueueOperations_Impl::Resume(this).into()
            }
        }
        unsafe extern "system" fn Purge<Identity: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueueOperations_Impl::Purge(this).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            Status: Status::<Identity, OFFSET>,
            PrintJobs: PrintJobs::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            Purge: Purge::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPrintQueueOperations as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsPrintQueueOperations {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsProperty, IADsProperty_Vtbl, 0xc8f93dd3_4ae0_11cf_9e73_00aa004a5691);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsProperty {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsProperty, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsProperty {
    pub unsafe fn OID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetOID(&self, bstroid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstroid)) }
    }
    pub unsafe fn Syntax(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Syntax)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSyntax(&self, bstrsyntax: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSyntax)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsyntax)) }
    }
    pub unsafe fn MaxRange(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxRange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxRange(&self, lnmaxrange: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaxRange)(windows_core::Interface::as_raw(self), lnmaxrange) }
    }
    pub unsafe fn MinRange(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinRange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMinRange(&self, lnminrange: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMinRange)(windows_core::Interface::as_raw(self), lnminrange) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn MultiValued(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MultiValued)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetMultiValued(&self, fmultivalued: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMultiValued)(windows_core::Interface::as_raw(self), fmultivalued) }
    }
    pub unsafe fn Qualifiers(&self) -> windows_core::Result<IADsCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Qualifiers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsProperty_Vtbl {
    pub base__: IADs_Vtbl,
    pub OID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Syntax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSyntax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MinRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMinRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub MultiValued: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    MultiValued: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetMultiValued: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetMultiValued: usize,
    pub Qualifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsProperty_Impl: IADs_Impl {
    fn OID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOID(&self, bstroid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Syntax(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSyntax(&self, bstrsyntax: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MaxRange(&self) -> windows_core::Result<i32>;
    fn SetMaxRange(&self, lnmaxrange: i32) -> windows_core::Result<()>;
    fn MinRange(&self) -> windows_core::Result<i32>;
    fn SetMinRange(&self, lnminrange: i32) -> windows_core::Result<()>;
    fn MultiValued(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetMultiValued(&self, fmultivalued: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Qualifiers(&self) -> windows_core::Result<IADsCollection>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsProperty_Vtbl {
    pub const fn new<Identity: IADsProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OID<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsProperty_Impl::OID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOID<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsProperty_Impl::SetOID(this, core::mem::transmute(&bstroid)).into()
            }
        }
        unsafe extern "system" fn Syntax<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsProperty_Impl::Syntax(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSyntax<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsyntax: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsProperty_Impl::SetSyntax(this, core::mem::transmute(&bstrsyntax)).into()
            }
        }
        unsafe extern "system" fn MaxRange<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsProperty_Impl::MaxRange(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxRange<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxrange: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsProperty_Impl::SetMaxRange(this, core::mem::transmute_copy(&lnmaxrange)).into()
            }
        }
        unsafe extern "system" fn MinRange<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsProperty_Impl::MinRange(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinRange<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnminrange: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsProperty_Impl::SetMinRange(this, core::mem::transmute_copy(&lnminrange)).into()
            }
        }
        unsafe extern "system" fn MultiValued<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsProperty_Impl::MultiValued(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMultiValued<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmultivalued: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsProperty_Impl::SetMultiValued(this, core::mem::transmute_copy(&fmultivalued)).into()
            }
        }
        unsafe extern "system" fn Qualifiers<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqualifiers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsProperty_Impl::Qualifiers(this) {
                    Ok(ok__) => {
                        ppqualifiers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            OID: OID::<Identity, OFFSET>,
            SetOID: SetOID::<Identity, OFFSET>,
            Syntax: Syntax::<Identity, OFFSET>,
            SetSyntax: SetSyntax::<Identity, OFFSET>,
            MaxRange: MaxRange::<Identity, OFFSET>,
            SetMaxRange: SetMaxRange::<Identity, OFFSET>,
            MinRange: MinRange::<Identity, OFFSET>,
            SetMinRange: SetMinRange::<Identity, OFFSET>,
            MultiValued: MultiValued::<Identity, OFFSET>,
            SetMultiValued: SetMultiValued::<Identity, OFFSET>,
            Qualifiers: Qualifiers::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsProperty as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsProperty {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsPropertyEntry, IADsPropertyEntry_Vtbl, 0x05792c8e_941f_11d0_8529_00c04fd8d503);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsPropertyEntry {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsPropertyEntry, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsPropertyEntry {
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname)) }
    }
    pub unsafe fn ADsType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ADsType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetADsType(&self, lnadstype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetADsType)(windows_core::Interface::as_raw(self), lnadstype) }
    }
    pub unsafe fn ControlCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ControlCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetControlCode(&self, lncontrolcode: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetControlCode)(windows_core::Interface::as_raw(self), lncontrolcode) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Values(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Values)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetValues(&self, vvalues: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValues)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vvalues)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPropertyEntry_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ADsType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetADsType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ControlCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetControlCode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Values: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Values: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetValues: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetValues: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsPropertyEntry_Impl: super::oaidl::IDispatch_Impl {
    fn Clear(&self) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ADsType(&self) -> windows_core::Result<i32>;
    fn SetADsType(&self, lnadstype: i32) -> windows_core::Result<()>;
    fn ControlCode(&self) -> windows_core::Result<i32>;
    fn SetControlCode(&self, lncontrolcode: i32) -> windows_core::Result<()>;
    fn Values(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetValues(&self, vvalues: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsPropertyEntry_Vtbl {
    pub const fn new<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clear<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyEntry_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn Name<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyEntry_Impl::Name(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyEntry_Impl::SetName(this, core::mem::transmute(&bstrname)).into()
            }
        }
        unsafe extern "system" fn ADsType<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyEntry_Impl::ADsType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetADsType<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnadstype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyEntry_Impl::SetADsType(this, core::mem::transmute_copy(&lnadstype)).into()
            }
        }
        unsafe extern "system" fn ControlCode<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyEntry_Impl::ControlCode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetControlCode<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lncontrolcode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyEntry_Impl::SetControlCode(this, core::mem::transmute_copy(&lncontrolcode)).into()
            }
        }
        unsafe extern "system" fn Values<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyEntry_Impl::Values(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValues<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vvalues: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyEntry_Impl::SetValues(this, core::mem::transmute(&vvalues)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Clear: Clear::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            ADsType: ADsType::<Identity, OFFSET>,
            SetADsType: SetADsType::<Identity, OFFSET>,
            ControlCode: ControlCode::<Identity, OFFSET>,
            SetControlCode: SetControlCode::<Identity, OFFSET>,
            Values: Values::<Identity, OFFSET>,
            SetValues: SetValues::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPropertyEntry as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsPropertyEntry {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsPropertyList, IADsPropertyList_Vtbl, 0xc6f602b6_8f69_11d0_8528_00c04fd8d503);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsPropertyList {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsPropertyList, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsPropertyList {
    pub unsafe fn PropertyCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertyCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Next(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Skip(&self, celements: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celements) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Item(&self, varindex: &super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetPropertyItem(&self, bstrname: &windows_core::BSTR, lnadstype: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), lnadstype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn PutPropertyItem(&self, vardata: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutPropertyItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vardata)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn ResetPropertyItem(&self, varentry: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetPropertyItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varentry)) }
    }
    pub unsafe fn PurgePropertyList(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PurgePropertyList)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPropertyList_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub PropertyCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Item: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetPropertyItem: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub PutPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    PutPropertyItem: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub ResetPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    ResetPropertyItem: usize,
    pub PurgePropertyList: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsPropertyList_Impl: super::oaidl::IDispatch_Impl {
    fn PropertyCount(&self) -> windows_core::Result<i32>;
    fn Next(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn Skip(&self, celements: i32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Item(&self, varindex: &super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT>;
    fn GetPropertyItem(&self, bstrname: &windows_core::BSTR, lnadstype: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn PutPropertyItem(&self, vardata: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn ResetPropertyItem(&self, varentry: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn PurgePropertyList(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsPropertyList_Vtbl {
    pub const fn new<Identity: IADsPropertyList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PropertyCount<Identity: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyList_Impl::PropertyCount(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyList_Impl::Next(this) {
                    Ok(ok__) => {
                        pvariant.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Skip<Identity: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celements: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyList_Impl::Skip(this, core::mem::transmute_copy(&celements)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyList_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Item<Identity: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::oaidl::VARIANT, pvariant: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyList_Impl::Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        pvariant.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyItem<Identity: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, lnadstype: i32, pvariant: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyList_Impl::GetPropertyItem(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&lnadstype)) {
                    Ok(ok__) => {
                        pvariant.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PutPropertyItem<Identity: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vardata: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyList_Impl::PutPropertyItem(this, core::mem::transmute(&vardata)).into()
            }
        }
        unsafe extern "system" fn ResetPropertyItem<Identity: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varentry: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyList_Impl::ResetPropertyItem(this, core::mem::transmute(&varentry)).into()
            }
        }
        unsafe extern "system" fn PurgePropertyList<Identity: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyList_Impl::PurgePropertyList(this).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            PropertyCount: PropertyCount::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            GetPropertyItem: GetPropertyItem::<Identity, OFFSET>,
            PutPropertyItem: PutPropertyItem::<Identity, OFFSET>,
            ResetPropertyItem: ResetPropertyItem::<Identity, OFFSET>,
            PurgePropertyList: PurgePropertyList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPropertyList as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsPropertyList {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsPropertyValue, IADsPropertyValue_Vtbl, 0x79fa9ad0_a97c_11d0_8534_00c04fd8d503);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsPropertyValue {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsPropertyValue, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsPropertyValue {
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ADsType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ADsType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetADsType(&self, lnadstype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetADsType)(windows_core::Interface::as_raw(self), lnadstype) }
    }
    pub unsafe fn DNString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DNString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDNString(&self, bstrdnstring: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDNString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdnstring)) }
    }
    pub unsafe fn CaseExactString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CaseExactString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCaseExactString(&self, bstrcaseexactstring: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCaseExactString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcaseexactstring)) }
    }
    pub unsafe fn CaseIgnoreString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CaseIgnoreString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCaseIgnoreString(&self, bstrcaseignorestring: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCaseIgnoreString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcaseignorestring)) }
    }
    pub unsafe fn PrintableString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrintableString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPrintableString(&self, bstrprintablestring: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrintableString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprintablestring)) }
    }
    pub unsafe fn NumericString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NumericString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetNumericString(&self, bstrnumericstring: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNumericString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnumericstring)) }
    }
    pub unsafe fn Boolean(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Boolean)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBoolean(&self, lnboolean: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBoolean)(windows_core::Interface::as_raw(self), lnboolean) }
    }
    pub unsafe fn Integer(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Integer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInteger(&self, lninteger: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInteger)(windows_core::Interface::as_raw(self), lninteger) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn OctetString(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OctetString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetOctetString(&self, voctetstring: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOctetString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(voctetstring)) }
    }
    pub unsafe fn SecurityDescriptor(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SecurityDescriptor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSecurityDescriptor<P0>(&self, psecuritydescriptor: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSecurityDescriptor)(windows_core::Interface::as_raw(self), psecuritydescriptor.param().abi()) }
    }
    pub unsafe fn LargeInteger(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LargeInteger)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetLargeInteger<P0>(&self, plargeinteger: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLargeInteger)(windows_core::Interface::as_raw(self), plargeinteger.param().abi()) }
    }
    pub unsafe fn UTCTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UTCTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUTCTime(&self, dautctime: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUTCTime)(windows_core::Interface::as_raw(self), dautctime) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPropertyValue_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ADsType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetADsType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DNString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDNString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CaseExactString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCaseExactString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CaseIgnoreString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCaseIgnoreString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrintableString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrintableString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NumericString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNumericString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Boolean: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBoolean: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Integer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetInteger: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub OctetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    OctetString: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetOctetString: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetOctetString: usize,
    pub SecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LargeInteger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLargeInteger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UTCTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetUTCTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsPropertyValue_Impl: super::oaidl::IDispatch_Impl {
    fn Clear(&self) -> windows_core::Result<()>;
    fn ADsType(&self) -> windows_core::Result<i32>;
    fn SetADsType(&self, lnadstype: i32) -> windows_core::Result<()>;
    fn DNString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDNString(&self, bstrdnstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CaseExactString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCaseExactString(&self, bstrcaseexactstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CaseIgnoreString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCaseIgnoreString(&self, bstrcaseignorestring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PrintableString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPrintableString(&self, bstrprintablestring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NumericString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetNumericString(&self, bstrnumericstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Boolean(&self) -> windows_core::Result<i32>;
    fn SetBoolean(&self, lnboolean: i32) -> windows_core::Result<()>;
    fn Integer(&self) -> windows_core::Result<i32>;
    fn SetInteger(&self, lninteger: i32) -> windows_core::Result<()>;
    fn OctetString(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetOctetString(&self, voctetstring: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn SecurityDescriptor(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn SetSecurityDescriptor(&self, psecuritydescriptor: windows_core::Ref<super::oaidl::IDispatch>) -> windows_core::Result<()>;
    fn LargeInteger(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn SetLargeInteger(&self, plargeinteger: windows_core::Ref<super::oaidl::IDispatch>) -> windows_core::Result<()>;
    fn UTCTime(&self) -> windows_core::Result<f64>;
    fn SetUTCTime(&self, dautctime: f64) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsPropertyValue_Vtbl {
    pub const fn new<Identity: IADsPropertyValue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clear<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn ADsType<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::ADsType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetADsType<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnadstype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::SetADsType(this, core::mem::transmute_copy(&lnadstype)).into()
            }
        }
        unsafe extern "system" fn DNString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::DNString(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDNString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdnstring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::SetDNString(this, core::mem::transmute(&bstrdnstring)).into()
            }
        }
        unsafe extern "system" fn CaseExactString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::CaseExactString(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCaseExactString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcaseexactstring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::SetCaseExactString(this, core::mem::transmute(&bstrcaseexactstring)).into()
            }
        }
        unsafe extern "system" fn CaseIgnoreString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::CaseIgnoreString(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCaseIgnoreString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcaseignorestring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::SetCaseIgnoreString(this, core::mem::transmute(&bstrcaseignorestring)).into()
            }
        }
        unsafe extern "system" fn PrintableString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::PrintableString(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrintableString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprintablestring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::SetPrintableString(this, core::mem::transmute(&bstrprintablestring)).into()
            }
        }
        unsafe extern "system" fn NumericString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::NumericString(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNumericString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnumericstring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::SetNumericString(this, core::mem::transmute(&bstrnumericstring)).into()
            }
        }
        unsafe extern "system" fn Boolean<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::Boolean(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBoolean<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnboolean: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::SetBoolean(this, core::mem::transmute_copy(&lnboolean)).into()
            }
        }
        unsafe extern "system" fn Integer<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::Integer(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInteger<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lninteger: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::SetInteger(this, core::mem::transmute_copy(&lninteger)).into()
            }
        }
        unsafe extern "system" fn OctetString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::OctetString(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOctetString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, voctetstring: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::SetOctetString(this, core::mem::transmute(&voctetstring)).into()
            }
        }
        unsafe extern "system" fn SecurityDescriptor<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::SecurityDescriptor(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psecuritydescriptor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::SetSecurityDescriptor(this, core::mem::transmute_copy(&psecuritydescriptor)).into()
            }
        }
        unsafe extern "system" fn LargeInteger<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::LargeInteger(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLargeInteger<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plargeinteger: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::SetLargeInteger(this, core::mem::transmute_copy(&plargeinteger)).into()
            }
        }
        unsafe extern "system" fn UTCTime<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::UTCTime(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUTCTime<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dautctime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::SetUTCTime(this, core::mem::transmute_copy(&dautctime)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Clear: Clear::<Identity, OFFSET>,
            ADsType: ADsType::<Identity, OFFSET>,
            SetADsType: SetADsType::<Identity, OFFSET>,
            DNString: DNString::<Identity, OFFSET>,
            SetDNString: SetDNString::<Identity, OFFSET>,
            CaseExactString: CaseExactString::<Identity, OFFSET>,
            SetCaseExactString: SetCaseExactString::<Identity, OFFSET>,
            CaseIgnoreString: CaseIgnoreString::<Identity, OFFSET>,
            SetCaseIgnoreString: SetCaseIgnoreString::<Identity, OFFSET>,
            PrintableString: PrintableString::<Identity, OFFSET>,
            SetPrintableString: SetPrintableString::<Identity, OFFSET>,
            NumericString: NumericString::<Identity, OFFSET>,
            SetNumericString: SetNumericString::<Identity, OFFSET>,
            Boolean: Boolean::<Identity, OFFSET>,
            SetBoolean: SetBoolean::<Identity, OFFSET>,
            Integer: Integer::<Identity, OFFSET>,
            SetInteger: SetInteger::<Identity, OFFSET>,
            OctetString: OctetString::<Identity, OFFSET>,
            SetOctetString: SetOctetString::<Identity, OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Identity, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, OFFSET>,
            LargeInteger: LargeInteger::<Identity, OFFSET>,
            SetLargeInteger: SetLargeInteger::<Identity, OFFSET>,
            UTCTime: UTCTime::<Identity, OFFSET>,
            SetUTCTime: SetUTCTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPropertyValue as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsPropertyValue {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsPropertyValue2, IADsPropertyValue2_Vtbl, 0x306e831c_5bc7_11d1_a3b8_00c04fb950dc);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsPropertyValue2 {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsPropertyValue2, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsPropertyValue2 {
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetObjectProperty(&self, lnadstype: *mut i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectProperty)(windows_core::Interface::as_raw(self), lnadstype as _, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn PutObjectProperty(&self, lnadstype: i32, vprop: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutObjectProperty)(windows_core::Interface::as_raw(self), lnadstype, core::mem::transmute_copy(vprop)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPropertyValue2_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetObjectProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetObjectProperty: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub PutObjectProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    PutObjectProperty: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsPropertyValue2_Impl: super::oaidl::IDispatch_Impl {
    fn GetObjectProperty(&self, lnadstype: *mut i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn PutObjectProperty(&self, lnadstype: i32, vprop: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsPropertyValue2_Vtbl {
    pub const fn new<Identity: IADsPropertyValue2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetObjectProperty<Identity: IADsPropertyValue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnadstype: *mut i32, pvprop: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue2_Impl::GetObjectProperty(this, core::mem::transmute_copy(&lnadstype)) {
                    Ok(ok__) => {
                        pvprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PutObjectProperty<Identity: IADsPropertyValue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnadstype: i32, vprop: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue2_Impl::PutObjectProperty(this, core::mem::transmute_copy(&lnadstype), core::mem::transmute(&vprop)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetObjectProperty: GetObjectProperty::<Identity, OFFSET>,
            PutObjectProperty: PutObjectProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPropertyValue2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsPropertyValue2 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsReplicaPointer, IADsReplicaPointer_Vtbl, 0xf60fb803_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsReplicaPointer {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsReplicaPointer, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsReplicaPointer {
    pub unsafe fn ServerName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServerName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetServerName(&self, bstrservername: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetServerName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservername)) }
    }
    pub unsafe fn ReplicaType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReplicaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetReplicaType(&self, lnreplicatype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetReplicaType)(windows_core::Interface::as_raw(self), lnreplicatype) }
    }
    pub unsafe fn ReplicaNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReplicaNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetReplicaNumber(&self, lnreplicanumber: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetReplicaNumber)(windows_core::Interface::as_raw(self), lnreplicanumber) }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCount(&self, lncount: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCount)(windows_core::Interface::as_raw(self), lncount) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn ReplicaAddressHints(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReplicaAddressHints)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetReplicaAddressHints(&self, vreplicaaddresshints: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetReplicaAddressHints)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vreplicaaddresshints)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsReplicaPointer_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub ServerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetServerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReplicaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetReplicaType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ReplicaNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetReplicaNumber: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub ReplicaAddressHints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    ReplicaAddressHints: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetReplicaAddressHints: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetReplicaAddressHints: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsReplicaPointer_Impl: super::oaidl::IDispatch_Impl {
    fn ServerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServerName(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReplicaType(&self) -> windows_core::Result<i32>;
    fn SetReplicaType(&self, lnreplicatype: i32) -> windows_core::Result<()>;
    fn ReplicaNumber(&self) -> windows_core::Result<i32>;
    fn SetReplicaNumber(&self, lnreplicanumber: i32) -> windows_core::Result<()>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn SetCount(&self, lncount: i32) -> windows_core::Result<()>;
    fn ReplicaAddressHints(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetReplicaAddressHints(&self, vreplicaaddresshints: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsReplicaPointer_Vtbl {
    pub const fn new<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ServerName<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsReplicaPointer_Impl::ServerName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServerName<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsReplicaPointer_Impl::SetServerName(this, core::mem::transmute(&bstrservername)).into()
            }
        }
        unsafe extern "system" fn ReplicaType<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsReplicaPointer_Impl::ReplicaType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReplicaType<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnreplicatype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsReplicaPointer_Impl::SetReplicaType(this, core::mem::transmute_copy(&lnreplicatype)).into()
            }
        }
        unsafe extern "system" fn ReplicaNumber<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsReplicaPointer_Impl::ReplicaNumber(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReplicaNumber<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnreplicanumber: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsReplicaPointer_Impl::SetReplicaNumber(this, core::mem::transmute_copy(&lnreplicanumber)).into()
            }
        }
        unsafe extern "system" fn Count<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsReplicaPointer_Impl::Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCount<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lncount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsReplicaPointer_Impl::SetCount(this, core::mem::transmute_copy(&lncount)).into()
            }
        }
        unsafe extern "system" fn ReplicaAddressHints<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsReplicaPointer_Impl::ReplicaAddressHints(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReplicaAddressHints<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vreplicaaddresshints: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsReplicaPointer_Impl::SetReplicaAddressHints(this, core::mem::transmute(&vreplicaaddresshints)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ServerName: ServerName::<Identity, OFFSET>,
            SetServerName: SetServerName::<Identity, OFFSET>,
            ReplicaType: ReplicaType::<Identity, OFFSET>,
            SetReplicaType: SetReplicaType::<Identity, OFFSET>,
            ReplicaNumber: ReplicaNumber::<Identity, OFFSET>,
            SetReplicaNumber: SetReplicaNumber::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            SetCount: SetCount::<Identity, OFFSET>,
            ReplicaAddressHints: ReplicaAddressHints::<Identity, OFFSET>,
            SetReplicaAddressHints: SetReplicaAddressHints::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsReplicaPointer as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsReplicaPointer {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsResource, IADsResource_Vtbl, 0x34a05b20_4aab_11cf_ae2c_00aa006ebfb9);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsResource {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsResource, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsResource {
    pub unsafe fn User(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).User)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UserPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn LockCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LockCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsResource_Vtbl {
    pub base__: IADs_Vtbl,
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsResource_Impl: IADs_Impl {
    fn User(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LockCount(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsResource_Vtbl {
    pub const fn new<Identity: IADsResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn User<Identity: IADsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsResource_Impl::User(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserPath<Identity: IADsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsResource_Impl::UserPath(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Path<Identity: IADsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsResource_Impl::Path(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LockCount<Identity: IADsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsResource_Impl::LockCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            User: User::<Identity, OFFSET>,
            UserPath: UserPath::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            LockCount: LockCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsResource as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsResource {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsSecurityDescriptor, IADsSecurityDescriptor_Vtbl, 0xb8c787ca_9bdd_11d0_852c_00c04fd8d503);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsSecurityDescriptor {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsSecurityDescriptor, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsSecurityDescriptor {
    pub unsafe fn Revision(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Revision)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRevision(&self, lnrevision: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRevision)(windows_core::Interface::as_raw(self), lnrevision) }
    }
    pub unsafe fn Control(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Control)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetControl(&self, lncontrol: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetControl)(windows_core::Interface::as_raw(self), lncontrol) }
    }
    pub unsafe fn Owner(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Owner)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetOwner(&self, bstrowner: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOwner)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrowner)) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn OwnerDefaulted(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OwnerDefaulted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetOwnerDefaulted(&self, fownerdefaulted: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOwnerDefaulted)(windows_core::Interface::as_raw(self), fownerdefaulted) }
    }
    pub unsafe fn Group(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Group)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetGroup(&self, bstrgroup: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGroup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgroup)) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn GroupDefaulted(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GroupDefaulted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetGroupDefaulted(&self, fgroupdefaulted: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGroupDefaulted)(windows_core::Interface::as_raw(self), fgroupdefaulted) }
    }
    pub unsafe fn DiscretionaryAcl(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DiscretionaryAcl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDiscretionaryAcl<P0>(&self, pdiscretionaryacl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDiscretionaryAcl)(windows_core::Interface::as_raw(self), pdiscretionaryacl.param().abi()) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn DaclDefaulted(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DaclDefaulted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetDaclDefaulted(&self, fdacldefaulted: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDaclDefaulted)(windows_core::Interface::as_raw(self), fdacldefaulted) }
    }
    pub unsafe fn SystemAcl(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SystemAcl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSystemAcl<P0>(&self, psystemacl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSystemAcl)(windows_core::Interface::as_raw(self), psystemacl.param().abi()) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SaclDefaulted(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SaclDefaulted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetSaclDefaulted(&self, fsacldefaulted: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSaclDefaulted)(windows_core::Interface::as_raw(self), fsacldefaulted) }
    }
    pub unsafe fn CopySecurityDescriptor(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopySecurityDescriptor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsSecurityDescriptor_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Revision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRevision: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Control: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetControl: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Owner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub OwnerDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    OwnerDefaulted: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetOwnerDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetOwnerDefaulted: usize,
    pub Group: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub GroupDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    GroupDefaulted: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetGroupDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetGroupDefaulted: usize,
    pub DiscretionaryAcl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDiscretionaryAcl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub DaclDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    DaclDefaulted: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetDaclDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetDaclDefaulted: usize,
    pub SystemAcl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSystemAcl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub SaclDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SaclDefaulted: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetSaclDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetSaclDefaulted: usize,
    pub CopySecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsSecurityDescriptor_Impl: super::oaidl::IDispatch_Impl {
    fn Revision(&self) -> windows_core::Result<i32>;
    fn SetRevision(&self, lnrevision: i32) -> windows_core::Result<()>;
    fn Control(&self) -> windows_core::Result<i32>;
    fn SetControl(&self, lncontrol: i32) -> windows_core::Result<()>;
    fn Owner(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOwner(&self, bstrowner: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OwnerDefaulted(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetOwnerDefaulted(&self, fownerdefaulted: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Group(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetGroup(&self, bstrgroup: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GroupDefaulted(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetGroupDefaulted(&self, fgroupdefaulted: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DiscretionaryAcl(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn SetDiscretionaryAcl(&self, pdiscretionaryacl: windows_core::Ref<super::oaidl::IDispatch>) -> windows_core::Result<()>;
    fn DaclDefaulted(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetDaclDefaulted(&self, fdacldefaulted: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SystemAcl(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn SetSystemAcl(&self, psystemacl: windows_core::Ref<super::oaidl::IDispatch>) -> windows_core::Result<()>;
    fn SaclDefaulted(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetSaclDefaulted(&self, fsacldefaulted: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CopySecurityDescriptor(&self) -> windows_core::Result<super::oaidl::IDispatch>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsSecurityDescriptor_Vtbl {
    pub const fn new<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Revision<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::Revision(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRevision<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnrevision: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::SetRevision(this, core::mem::transmute_copy(&lnrevision)).into()
            }
        }
        unsafe extern "system" fn Control<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::Control(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetControl<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lncontrol: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::SetControl(this, core::mem::transmute_copy(&lncontrol)).into()
            }
        }
        unsafe extern "system" fn Owner<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::Owner(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOwner<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrowner: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::SetOwner(this, core::mem::transmute(&bstrowner)).into()
            }
        }
        unsafe extern "system" fn OwnerDefaulted<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::OwnerDefaulted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOwnerDefaulted<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fownerdefaulted: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::SetOwnerDefaulted(this, core::mem::transmute_copy(&fownerdefaulted)).into()
            }
        }
        unsafe extern "system" fn Group<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::Group(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGroup<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroup: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::SetGroup(this, core::mem::transmute(&bstrgroup)).into()
            }
        }
        unsafe extern "system" fn GroupDefaulted<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::GroupDefaulted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGroupDefaulted<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fgroupdefaulted: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::SetGroupDefaulted(this, core::mem::transmute_copy(&fgroupdefaulted)).into()
            }
        }
        unsafe extern "system" fn DiscretionaryAcl<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::DiscretionaryAcl(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDiscretionaryAcl<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdiscretionaryacl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::SetDiscretionaryAcl(this, core::mem::transmute_copy(&pdiscretionaryacl)).into()
            }
        }
        unsafe extern "system" fn DaclDefaulted<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::DaclDefaulted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDaclDefaulted<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdacldefaulted: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::SetDaclDefaulted(this, core::mem::transmute_copy(&fdacldefaulted)).into()
            }
        }
        unsafe extern "system" fn SystemAcl<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::SystemAcl(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSystemAcl<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psystemacl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::SetSystemAcl(this, core::mem::transmute_copy(&psystemacl)).into()
            }
        }
        unsafe extern "system" fn SaclDefaulted<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::SaclDefaulted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSaclDefaulted<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsacldefaulted: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::SetSaclDefaulted(this, core::mem::transmute_copy(&fsacldefaulted)).into()
            }
        }
        unsafe extern "system" fn CopySecurityDescriptor<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecuritydescriptor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::CopySecurityDescriptor(this) {
                    Ok(ok__) => {
                        ppsecuritydescriptor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Revision: Revision::<Identity, OFFSET>,
            SetRevision: SetRevision::<Identity, OFFSET>,
            Control: Control::<Identity, OFFSET>,
            SetControl: SetControl::<Identity, OFFSET>,
            Owner: Owner::<Identity, OFFSET>,
            SetOwner: SetOwner::<Identity, OFFSET>,
            OwnerDefaulted: OwnerDefaulted::<Identity, OFFSET>,
            SetOwnerDefaulted: SetOwnerDefaulted::<Identity, OFFSET>,
            Group: Group::<Identity, OFFSET>,
            SetGroup: SetGroup::<Identity, OFFSET>,
            GroupDefaulted: GroupDefaulted::<Identity, OFFSET>,
            SetGroupDefaulted: SetGroupDefaulted::<Identity, OFFSET>,
            DiscretionaryAcl: DiscretionaryAcl::<Identity, OFFSET>,
            SetDiscretionaryAcl: SetDiscretionaryAcl::<Identity, OFFSET>,
            DaclDefaulted: DaclDefaulted::<Identity, OFFSET>,
            SetDaclDefaulted: SetDaclDefaulted::<Identity, OFFSET>,
            SystemAcl: SystemAcl::<Identity, OFFSET>,
            SetSystemAcl: SetSystemAcl::<Identity, OFFSET>,
            SaclDefaulted: SaclDefaulted::<Identity, OFFSET>,
            SetSaclDefaulted: SetSaclDefaulted::<Identity, OFFSET>,
            CopySecurityDescriptor: CopySecurityDescriptor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsSecurityDescriptor as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsSecurityDescriptor {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsSecurityUtility, IADsSecurityUtility_Vtbl, 0xa63251b2_5f21_474b_ab52_4a8efad10895);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsSecurityUtility {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsSecurityUtility, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsSecurityUtility {
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetSecurityDescriptor(&self, varpath: &super::oaidl::VARIANT, lpathformat: i32, lformat: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurityDescriptor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varpath), lpathformat, lformat, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetSecurityDescriptor(&self, varpath: &super::oaidl::VARIANT, lpathformat: i32, vardata: &super::oaidl::VARIANT, ldataformat: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSecurityDescriptor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varpath), lpathformat, core::mem::transmute_copy(vardata), ldataformat) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn ConvertSecurityDescriptor(&self, varsd: &super::oaidl::VARIANT, ldataformat: i32, loutformat: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConvertSecurityDescriptor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varsd), ldataformat, loutformat, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SecurityMask(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SecurityMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSecurityMask(&self, lnsecuritymask: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSecurityMask)(windows_core::Interface::as_raw(self), lnsecuritymask) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsSecurityUtility_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, i32, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetSecurityDescriptor: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, i32, super::oaidl::VARIANT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetSecurityDescriptor: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub ConvertSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, i32, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    ConvertSecurityDescriptor: usize,
    pub SecurityMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSecurityMask: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsSecurityUtility_Impl: super::oaidl::IDispatch_Impl {
    fn GetSecurityDescriptor(&self, varpath: &super::oaidl::VARIANT, lpathformat: i32, lformat: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetSecurityDescriptor(&self, varpath: &super::oaidl::VARIANT, lpathformat: i32, vardata: &super::oaidl::VARIANT, ldataformat: i32) -> windows_core::Result<()>;
    fn ConvertSecurityDescriptor(&self, varsd: &super::oaidl::VARIANT, ldataformat: i32, loutformat: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SecurityMask(&self) -> windows_core::Result<i32>;
    fn SetSecurityMask(&self, lnsecuritymask: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsSecurityUtility_Vtbl {
    pub const fn new<Identity: IADsSecurityUtility_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSecurityDescriptor<Identity: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varpath: super::oaidl::VARIANT, lpathformat: i32, lformat: i32, pvariant: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityUtility_Impl::GetSecurityDescriptor(this, core::mem::transmute(&varpath), core::mem::transmute_copy(&lpathformat), core::mem::transmute_copy(&lformat)) {
                    Ok(ok__) => {
                        pvariant.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varpath: super::oaidl::VARIANT, lpathformat: i32, vardata: super::oaidl::VARIANT, ldataformat: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityUtility_Impl::SetSecurityDescriptor(this, core::mem::transmute(&varpath), core::mem::transmute_copy(&lpathformat), core::mem::transmute(&vardata), core::mem::transmute_copy(&ldataformat)).into()
            }
        }
        unsafe extern "system" fn ConvertSecurityDescriptor<Identity: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsd: super::oaidl::VARIANT, ldataformat: i32, loutformat: i32, presult: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityUtility_Impl::ConvertSecurityDescriptor(this, core::mem::transmute(&varsd), core::mem::transmute_copy(&ldataformat), core::mem::transmute_copy(&loutformat)) {
                    Ok(ok__) => {
                        presult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SecurityMask<Identity: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityUtility_Impl::SecurityMask(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurityMask<Identity: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnsecuritymask: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityUtility_Impl::SetSecurityMask(this, core::mem::transmute_copy(&lnsecuritymask)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, OFFSET>,
            ConvertSecurityDescriptor: ConvertSecurityDescriptor::<Identity, OFFSET>,
            SecurityMask: SecurityMask::<Identity, OFFSET>,
            SetSecurityMask: SetSecurityMask::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsSecurityUtility as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsSecurityUtility {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsService, IADsService_Vtbl, 0x68af66e0_31ca_11cf_a98a_00aa006bc149);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsService {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsService, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsService {
    pub unsafe fn HostComputer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HostComputer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetHostComputer(&self, bstrhostcomputer: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHostComputer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrhostcomputer)) }
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDisplayName(&self, bstrdisplayname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdisplayname)) }
    }
    pub unsafe fn Version(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetVersion(&self, bstrversion: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVersion)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrversion)) }
    }
    pub unsafe fn ServiceType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetServiceType(&self, lnservicetype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetServiceType)(windows_core::Interface::as_raw(self), lnservicetype) }
    }
    pub unsafe fn StartType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStartType(&self, lnstarttype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStartType)(windows_core::Interface::as_raw(self), lnstarttype) }
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPath(&self, bstrpath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpath)) }
    }
    pub unsafe fn StartupParameters(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartupParameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetStartupParameters(&self, bstrstartupparameters: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStartupParameters)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrstartupparameters)) }
    }
    pub unsafe fn ErrorControl(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ErrorControl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetErrorControl(&self, lnerrorcontrol: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetErrorControl)(windows_core::Interface::as_raw(self), lnerrorcontrol) }
    }
    pub unsafe fn LoadOrderGroup(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadOrderGroup)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLoadOrderGroup(&self, bstrloadordergroup: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLoadOrderGroup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrloadordergroup)) }
    }
    pub unsafe fn ServiceAccountName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceAccountName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetServiceAccountName(&self, bstrserviceaccountname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetServiceAccountName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrserviceaccountname)) }
    }
    pub unsafe fn ServiceAccountPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceAccountPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetServiceAccountPath(&self, bstrserviceaccountpath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetServiceAccountPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrserviceaccountpath)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Dependencies(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Dependencies)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetDependencies(&self, vdependencies: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDependencies)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vdependencies)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsService_Vtbl {
    pub base__: IADs_Vtbl,
    pub HostComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHostComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetServiceType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StartType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetStartType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartupParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStartupParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ErrorControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetErrorControl: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LoadOrderGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLoadOrderGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceAccountName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetServiceAccountName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceAccountPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetServiceAccountPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Dependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Dependencies: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetDependencies: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetDependencies: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsService_Impl: IADs_Impl {
    fn HostComputer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetHostComputer(&self, bstrhostcomputer: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, bstrdisplayname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetVersion(&self, bstrversion: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ServiceType(&self) -> windows_core::Result<i32>;
    fn SetServiceType(&self, lnservicetype: i32) -> windows_core::Result<()>;
    fn StartType(&self) -> windows_core::Result<i32>;
    fn SetStartType(&self, lnstarttype: i32) -> windows_core::Result<()>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPath(&self, bstrpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StartupParameters(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetStartupParameters(&self, bstrstartupparameters: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ErrorControl(&self) -> windows_core::Result<i32>;
    fn SetErrorControl(&self, lnerrorcontrol: i32) -> windows_core::Result<()>;
    fn LoadOrderGroup(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLoadOrderGroup(&self, bstrloadordergroup: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ServiceAccountName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceAccountName(&self, bstrserviceaccountname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ServiceAccountPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceAccountPath(&self, bstrserviceaccountpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Dependencies(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetDependencies(&self, vdependencies: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsService_Vtbl {
    pub const fn new<Identity: IADsService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HostComputer<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::HostComputer(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHostComputer<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhostcomputer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::SetHostComputer(this, core::mem::transmute(&bstrhostcomputer)).into()
            }
        }
        unsafe extern "system" fn DisplayName<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdisplayname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::SetDisplayName(this, core::mem::transmute(&bstrdisplayname)).into()
            }
        }
        unsafe extern "system" fn Version<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::Version(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVersion<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrversion: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::SetVersion(this, core::mem::transmute(&bstrversion)).into()
            }
        }
        unsafe extern "system" fn ServiceType<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::ServiceType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServiceType<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnservicetype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::SetServiceType(this, core::mem::transmute_copy(&lnservicetype)).into()
            }
        }
        unsafe extern "system" fn StartType<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::StartType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartType<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnstarttype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::SetStartType(this, core::mem::transmute_copy(&lnstarttype)).into()
            }
        }
        unsafe extern "system" fn Path<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::Path(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPath<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::SetPath(this, core::mem::transmute(&bstrpath)).into()
            }
        }
        unsafe extern "system" fn StartupParameters<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::StartupParameters(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartupParameters<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrstartupparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::SetStartupParameters(this, core::mem::transmute(&bstrstartupparameters)).into()
            }
        }
        unsafe extern "system" fn ErrorControl<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::ErrorControl(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetErrorControl<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnerrorcontrol: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::SetErrorControl(this, core::mem::transmute_copy(&lnerrorcontrol)).into()
            }
        }
        unsafe extern "system" fn LoadOrderGroup<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::LoadOrderGroup(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLoadOrderGroup<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrloadordergroup: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::SetLoadOrderGroup(this, core::mem::transmute(&bstrloadordergroup)).into()
            }
        }
        unsafe extern "system" fn ServiceAccountName<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::ServiceAccountName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServiceAccountName<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrserviceaccountname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::SetServiceAccountName(this, core::mem::transmute(&bstrserviceaccountname)).into()
            }
        }
        unsafe extern "system" fn ServiceAccountPath<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::ServiceAccountPath(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServiceAccountPath<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrserviceaccountpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::SetServiceAccountPath(this, core::mem::transmute(&bstrserviceaccountpath)).into()
            }
        }
        unsafe extern "system" fn Dependencies<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::Dependencies(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDependencies<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vdependencies: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::SetDependencies(this, core::mem::transmute(&vdependencies)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            HostComputer: HostComputer::<Identity, OFFSET>,
            SetHostComputer: SetHostComputer::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            SetVersion: SetVersion::<Identity, OFFSET>,
            ServiceType: ServiceType::<Identity, OFFSET>,
            SetServiceType: SetServiceType::<Identity, OFFSET>,
            StartType: StartType::<Identity, OFFSET>,
            SetStartType: SetStartType::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
            StartupParameters: StartupParameters::<Identity, OFFSET>,
            SetStartupParameters: SetStartupParameters::<Identity, OFFSET>,
            ErrorControl: ErrorControl::<Identity, OFFSET>,
            SetErrorControl: SetErrorControl::<Identity, OFFSET>,
            LoadOrderGroup: LoadOrderGroup::<Identity, OFFSET>,
            SetLoadOrderGroup: SetLoadOrderGroup::<Identity, OFFSET>,
            ServiceAccountName: ServiceAccountName::<Identity, OFFSET>,
            SetServiceAccountName: SetServiceAccountName::<Identity, OFFSET>,
            ServiceAccountPath: ServiceAccountPath::<Identity, OFFSET>,
            SetServiceAccountPath: SetServiceAccountPath::<Identity, OFFSET>,
            Dependencies: Dependencies::<Identity, OFFSET>,
            SetDependencies: SetDependencies::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsService as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsService {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsServiceOperations, IADsServiceOperations_Vtbl, 0x5d7b33f0_31ca_11cf_a98a_00aa006bc149);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsServiceOperations {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsServiceOperations, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsServiceOperations {
    pub unsafe fn Status(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Start(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Pause(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Continue(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Continue)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetPassword(&self, bstrnewpassword: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPassword)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnewpassword)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsServiceOperations_Vtbl {
    pub base__: IADs_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Continue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsServiceOperations_Impl: IADs_Impl {
    fn Status(&self) -> windows_core::Result<i32>;
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Continue(&self) -> windows_core::Result<()>;
    fn SetPassword(&self, bstrnewpassword: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsServiceOperations_Vtbl {
    pub const fn new<Identity: IADsServiceOperations_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Status<Identity: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsServiceOperations_Impl::Status(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Start<Identity: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsServiceOperations_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsServiceOperations_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsServiceOperations_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Continue<Identity: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsServiceOperations_Impl::Continue(this).into()
            }
        }
        unsafe extern "system" fn SetPassword<Identity: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnewpassword: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsServiceOperations_Impl::SetPassword(this, core::mem::transmute(&bstrnewpassword)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            Status: Status::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Continue: Continue::<Identity, OFFSET>,
            SetPassword: SetPassword::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsServiceOperations as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsServiceOperations {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsSession, IADsSession_Vtbl, 0x398b7da0_4aab_11cf_ae2c_00aa006ebfb9);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsSession {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsSession, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsSession {
    pub unsafe fn User(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).User)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UserPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Computer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Computer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ComputerPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputerPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ConnectTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IdleTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IdleTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsSession_Vtbl {
    pub base__: IADs_Vtbl,
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Computer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComputerPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IdleTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsSession_Impl: IADs_Impl {
    fn User(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Computer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ComputerPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ConnectTime(&self) -> windows_core::Result<i32>;
    fn IdleTime(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsSession_Vtbl {
    pub const fn new<Identity: IADsSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn User<Identity: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSession_Impl::User(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserPath<Identity: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSession_Impl::UserPath(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Computer<Identity: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSession_Impl::Computer(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ComputerPath<Identity: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSession_Impl::ComputerPath(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConnectTime<Identity: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSession_Impl::ConnectTime(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IdleTime<Identity: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSession_Impl::IdleTime(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            User: User::<Identity, OFFSET>,
            UserPath: UserPath::<Identity, OFFSET>,
            Computer: Computer::<Identity, OFFSET>,
            ComputerPath: ComputerPath::<Identity, OFFSET>,
            ConnectTime: ConnectTime::<Identity, OFFSET>,
            IdleTime: IdleTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsSession as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsSession {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsSyntax, IADsSyntax_Vtbl, 0xc8f93dd2_4ae0_11cf_9e73_00aa004a5691);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsSyntax {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsSyntax, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsSyntax {
    pub unsafe fn OleAutoDataType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OleAutoDataType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOleAutoDataType(&self, lnoleautodatatype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOleAutoDataType)(windows_core::Interface::as_raw(self), lnoleautodatatype) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsSyntax_Vtbl {
    pub base__: IADs_Vtbl,
    pub OleAutoDataType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetOleAutoDataType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsSyntax_Impl: IADs_Impl {
    fn OleAutoDataType(&self) -> windows_core::Result<i32>;
    fn SetOleAutoDataType(&self, lnoleautodatatype: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsSyntax_Vtbl {
    pub const fn new<Identity: IADsSyntax_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OleAutoDataType<Identity: IADsSyntax_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSyntax_Impl::OleAutoDataType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOleAutoDataType<Identity: IADsSyntax_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnoleautodatatype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSyntax_Impl::SetOleAutoDataType(this, core::mem::transmute_copy(&lnoleautodatatype)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            OleAutoDataType: OleAutoDataType::<Identity, OFFSET>,
            SetOleAutoDataType: SetOleAutoDataType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsSyntax as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsSyntax {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsTimestamp, IADsTimestamp_Vtbl, 0xb2f5a901_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsTimestamp {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsTimestamp, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsTimestamp {
    pub unsafe fn WholeSeconds(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WholeSeconds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWholeSeconds(&self, lnwholeseconds: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWholeSeconds)(windows_core::Interface::as_raw(self), lnwholeseconds) }
    }
    pub unsafe fn EventID(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEventID(&self, lneventid: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEventID)(windows_core::Interface::as_raw(self), lneventid) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsTimestamp_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub WholeSeconds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetWholeSeconds: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EventID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEventID: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsTimestamp_Impl: super::oaidl::IDispatch_Impl {
    fn WholeSeconds(&self) -> windows_core::Result<i32>;
    fn SetWholeSeconds(&self, lnwholeseconds: i32) -> windows_core::Result<()>;
    fn EventID(&self) -> windows_core::Result<i32>;
    fn SetEventID(&self, lneventid: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsTimestamp_Vtbl {
    pub const fn new<Identity: IADsTimestamp_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WholeSeconds<Identity: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTimestamp_Impl::WholeSeconds(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWholeSeconds<Identity: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnwholeseconds: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTimestamp_Impl::SetWholeSeconds(this, core::mem::transmute_copy(&lnwholeseconds)).into()
            }
        }
        unsafe extern "system" fn EventID<Identity: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTimestamp_Impl::EventID(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEventID<Identity: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lneventid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTimestamp_Impl::SetEventID(this, core::mem::transmute_copy(&lneventid)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            WholeSeconds: WholeSeconds::<Identity, OFFSET>,
            SetWholeSeconds: SetWholeSeconds::<Identity, OFFSET>,
            EventID: EventID::<Identity, OFFSET>,
            SetEventID: SetEventID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsTimestamp as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsTimestamp {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsTypedName, IADsTypedName_Vtbl, 0xb371a349_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsTypedName {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsTypedName, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsTypedName {
    pub unsafe fn ObjectName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ObjectName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetObjectName(&self, bstrobjectname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetObjectName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrobjectname)) }
    }
    pub unsafe fn Level(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Level)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLevel(&self, lnlevel: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLevel)(windows_core::Interface::as_raw(self), lnlevel) }
    }
    pub unsafe fn Interval(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Interval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInterval(&self, lninterval: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInterval)(windows_core::Interface::as_raw(self), lninterval) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsTypedName_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub ObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Level: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Interval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsTypedName_Impl: super::oaidl::IDispatch_Impl {
    fn ObjectName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetObjectName(&self, bstrobjectname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Level(&self) -> windows_core::Result<i32>;
    fn SetLevel(&self, lnlevel: i32) -> windows_core::Result<()>;
    fn Interval(&self) -> windows_core::Result<i32>;
    fn SetInterval(&self, lninterval: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsTypedName_Vtbl {
    pub const fn new<Identity: IADsTypedName_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ObjectName<Identity: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTypedName_Impl::ObjectName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetObjectName<Identity: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrobjectname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTypedName_Impl::SetObjectName(this, core::mem::transmute(&bstrobjectname)).into()
            }
        }
        unsafe extern "system" fn Level<Identity: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTypedName_Impl::Level(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLevel<Identity: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnlevel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTypedName_Impl::SetLevel(this, core::mem::transmute_copy(&lnlevel)).into()
            }
        }
        unsafe extern "system" fn Interval<Identity: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTypedName_Impl::Interval(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInterval<Identity: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lninterval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTypedName_Impl::SetInterval(this, core::mem::transmute_copy(&lninterval)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ObjectName: ObjectName::<Identity, OFFSET>,
            SetObjectName: SetObjectName::<Identity, OFFSET>,
            Level: Level::<Identity, OFFSET>,
            SetLevel: SetLevel::<Identity, OFFSET>,
            Interval: Interval::<Identity, OFFSET>,
            SetInterval: SetInterval::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsTypedName as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsTypedName {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsUser, IADsUser_Vtbl, 0x3e37e320_17e2_11cf_abc4_02608c9e7553);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsUser {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsUser, windows_core::IUnknown, super::oaidl::IDispatch, IADs);
#[cfg(feature = "Win32_oaidl")]
impl IADsUser {
    pub unsafe fn BadLoginAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BadLoginAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn BadLoginCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BadLoginCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastLogin(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastLogin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastLogoff(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastLogoff)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastFailedLogin(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastFailedLogin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PasswordLastChanged(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PasswordLastChanged)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn Division(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Division)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDivision(&self, bstrdivision: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDivision)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdivision)) }
    }
    pub unsafe fn Department(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Department)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDepartment(&self, bstrdepartment: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDepartment)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdepartment)) }
    }
    pub unsafe fn EmployeeID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EmployeeID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetEmployeeID(&self, bstremployeeid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEmployeeID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstremployeeid)) }
    }
    pub unsafe fn FullName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FullName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFullName(&self, bstrfullname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFullName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfullname)) }
    }
    pub unsafe fn FirstName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FirstName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFirstName(&self, bstrfirstname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFirstName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfirstname)) }
    }
    pub unsafe fn LastName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLastName(&self, bstrlastname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLastName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlastname)) }
    }
    pub unsafe fn OtherName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OtherName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetOtherName(&self, bstrothername: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOtherName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrothername)) }
    }
    pub unsafe fn NamePrefix(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NamePrefix)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetNamePrefix(&self, bstrnameprefix: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNamePrefix)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnameprefix)) }
    }
    pub unsafe fn NameSuffix(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NameSuffix)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetNameSuffix(&self, bstrnamesuffix: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNameSuffix)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnamesuffix)) }
    }
    pub unsafe fn Title(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTitle(&self, bstrtitle: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtitle)) }
    }
    pub unsafe fn Manager(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Manager)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetManager(&self, bstrmanager: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetManager)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmanager)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn TelephoneHome(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TelephoneHome)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetTelephoneHome(&self, vtelephonehome: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTelephoneHome)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vtelephonehome)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn TelephoneMobile(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TelephoneMobile)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetTelephoneMobile(&self, vtelephonemobile: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTelephoneMobile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vtelephonemobile)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn TelephoneNumber(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TelephoneNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetTelephoneNumber(&self, vtelephonenumber: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTelephoneNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vtelephonenumber)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn TelephonePager(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TelephonePager)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetTelephonePager(&self, vtelephonepager: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTelephonePager)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vtelephonepager)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn FaxNumber(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FaxNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetFaxNumber(&self, vfaxnumber: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFaxNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vfaxnumber)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn OfficeLocations(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OfficeLocations)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetOfficeLocations(&self, vofficelocations: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOfficeLocations)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vofficelocations)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn PostalAddresses(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PostalAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetPostalAddresses(&self, vpostaladdresses: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPostalAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vpostaladdresses)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn PostalCodes(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PostalCodes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetPostalCodes(&self, vpostalcodes: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPostalCodes)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vpostalcodes)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SeeAlso(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SeeAlso)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetSeeAlso(&self, vseealso: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSeeAlso)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vseealso)) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn AccountDisabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AccountDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetAccountDisabled(&self, faccountdisabled: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAccountDisabled)(windows_core::Interface::as_raw(self), faccountdisabled) }
    }
    pub unsafe fn AccountExpirationDate(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AccountExpirationDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAccountExpirationDate(&self, daaccountexpirationdate: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAccountExpirationDate)(windows_core::Interface::as_raw(self), daaccountexpirationdate) }
    }
    pub unsafe fn GraceLoginsAllowed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GraceLoginsAllowed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGraceLoginsAllowed(&self, lngraceloginsallowed: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGraceLoginsAllowed)(windows_core::Interface::as_raw(self), lngraceloginsallowed) }
    }
    pub unsafe fn GraceLoginsRemaining(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GraceLoginsRemaining)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGraceLoginsRemaining(&self, lngraceloginsremaining: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGraceLoginsRemaining)(windows_core::Interface::as_raw(self), lngraceloginsremaining) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn IsAccountLocked(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAccountLocked)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetIsAccountLocked(&self, fisaccountlocked: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIsAccountLocked)(windows_core::Interface::as_raw(self), fisaccountlocked) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn LoginHours(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoginHours)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetLoginHours(&self, vloginhours: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLoginHours)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vloginhours)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn LoginWorkstations(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoginWorkstations)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetLoginWorkstations(&self, vloginworkstations: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLoginWorkstations)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vloginworkstations)) }
    }
    pub unsafe fn MaxLogins(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxLogins)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxLogins(&self, lnmaxlogins: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaxLogins)(windows_core::Interface::as_raw(self), lnmaxlogins) }
    }
    pub unsafe fn MaxStorage(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxStorage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxStorage(&self, lnmaxstorage: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaxStorage)(windows_core::Interface::as_raw(self), lnmaxstorage) }
    }
    pub unsafe fn PasswordExpirationDate(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PasswordExpirationDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPasswordExpirationDate(&self, dapasswordexpirationdate: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPasswordExpirationDate)(windows_core::Interface::as_raw(self), dapasswordexpirationdate) }
    }
    pub unsafe fn PasswordMinimumLength(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PasswordMinimumLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPasswordMinimumLength(&self, lnpasswordminimumlength: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPasswordMinimumLength)(windows_core::Interface::as_raw(self), lnpasswordminimumlength) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn PasswordRequired(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PasswordRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetPasswordRequired(&self, fpasswordrequired: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPasswordRequired)(windows_core::Interface::as_raw(self), fpasswordrequired) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn RequireUniquePassword(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequireUniquePassword)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetRequireUniquePassword(&self, frequireuniquepassword: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRequireUniquePassword)(windows_core::Interface::as_raw(self), frequireuniquepassword) }
    }
    pub unsafe fn EmailAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EmailAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetEmailAddress(&self, bstremailaddress: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEmailAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstremailaddress)) }
    }
    pub unsafe fn HomeDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HomeDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetHomeDirectory(&self, bstrhomedirectory: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHomeDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrhomedirectory)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Languages(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Languages)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetLanguages(&self, vlanguages: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLanguages)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vlanguages)) }
    }
    pub unsafe fn Profile(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetProfile(&self, bstrprofile: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProfile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprofile)) }
    }
    pub unsafe fn LoginScript(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoginScript)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLoginScript(&self, bstrloginscript: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLoginScript)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrloginscript)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Picture(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Picture)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetPicture(&self, vpicture: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPicture)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vpicture)) }
    }
    pub unsafe fn HomePage(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HomePage)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetHomePage(&self, bstrhomepage: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHomePage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrhomepage)) }
    }
    pub unsafe fn Groups(&self) -> windows_core::Result<IADsMembers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Groups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPassword(&self, newpassword: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPassword)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newpassword)) }
    }
    pub unsafe fn ChangePassword(&self, bstroldpassword: &windows_core::BSTR, bstrnewpassword: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ChangePassword)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstroldpassword), core::mem::transmute_copy(bstrnewpassword)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsUser_Vtbl {
    pub base__: IADs_Vtbl,
    pub BadLoginAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BadLoginCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastLogin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub LastLogoff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub LastFailedLogin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub PasswordLastChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Division: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDivision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Department: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDepartment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EmployeeID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEmployeeID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FullName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFullName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FirstName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFirstName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LastName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLastName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OtherName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOtherName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NamePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNamePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NameSuffix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNameSuffix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Manager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub TelephoneHome: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    TelephoneHome: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetTelephoneHome: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetTelephoneHome: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub TelephoneMobile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    TelephoneMobile: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetTelephoneMobile: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetTelephoneMobile: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub TelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    TelephoneNumber: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetTelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetTelephoneNumber: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub TelephonePager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    TelephonePager: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetTelephonePager: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetTelephonePager: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub FaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    FaxNumber: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetFaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetFaxNumber: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub OfficeLocations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    OfficeLocations: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetOfficeLocations: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetOfficeLocations: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub PostalAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    PostalAddresses: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetPostalAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetPostalAddresses: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub PostalCodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    PostalCodes: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetPostalCodes: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetPostalCodes: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SeeAlso: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetSeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetSeeAlso: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub AccountDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    AccountDisabled: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetAccountDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetAccountDisabled: usize,
    pub AccountExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetAccountExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GraceLoginsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetGraceLoginsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GraceLoginsRemaining: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetGraceLoginsRemaining: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub IsAccountLocked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    IsAccountLocked: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetIsAccountLocked: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetIsAccountLocked: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub LoginHours: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    LoginHours: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetLoginHours: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetLoginHours: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub LoginWorkstations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    LoginWorkstations: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetLoginWorkstations: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetLoginWorkstations: usize,
    pub MaxLogins: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxLogins: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxStorage: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub PasswordExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetPasswordExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub PasswordMinimumLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPasswordMinimumLength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub PasswordRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    PasswordRequired: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetPasswordRequired: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetPasswordRequired: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub RequireUniquePassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    RequireUniquePassword: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetRequireUniquePassword: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetRequireUniquePassword: usize,
    pub EmailAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEmailAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HomeDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHomeDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Languages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Languages: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetLanguages: usize,
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoginScript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLoginScript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Picture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Picture: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetPicture: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetPicture: usize,
    pub HomePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHomePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Groups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChangePassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsUser_Impl: IADs_Impl {
    fn BadLoginAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BadLoginCount(&self) -> windows_core::Result<i32>;
    fn LastLogin(&self) -> windows_core::Result<f64>;
    fn LastLogoff(&self) -> windows_core::Result<f64>;
    fn LastFailedLogin(&self) -> windows_core::Result<f64>;
    fn PasswordLastChanged(&self) -> windows_core::Result<f64>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Division(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDivision(&self, bstrdivision: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Department(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDepartment(&self, bstrdepartment: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EmployeeID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEmployeeID(&self, bstremployeeid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FullName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFullName(&self, bstrfullname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FirstName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFirstName(&self, bstrfirstname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LastName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLastName(&self, bstrlastname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OtherName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOtherName(&self, bstrothername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NamePrefix(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetNamePrefix(&self, bstrnameprefix: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NameSuffix(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetNameSuffix(&self, bstrnamesuffix: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTitle(&self, bstrtitle: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Manager(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetManager(&self, bstrmanager: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TelephoneHome(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetTelephoneHome(&self, vtelephonehome: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn TelephoneMobile(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetTelephoneMobile(&self, vtelephonemobile: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn TelephoneNumber(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetTelephoneNumber(&self, vtelephonenumber: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn TelephonePager(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetTelephonePager(&self, vtelephonepager: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn FaxNumber(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetFaxNumber(&self, vfaxnumber: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn OfficeLocations(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetOfficeLocations(&self, vofficelocations: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn PostalAddresses(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetPostalAddresses(&self, vpostaladdresses: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn PostalCodes(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetPostalCodes(&self, vpostalcodes: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn SeeAlso(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetSeeAlso(&self, vseealso: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn AccountDisabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetAccountDisabled(&self, faccountdisabled: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AccountExpirationDate(&self) -> windows_core::Result<f64>;
    fn SetAccountExpirationDate(&self, daaccountexpirationdate: f64) -> windows_core::Result<()>;
    fn GraceLoginsAllowed(&self) -> windows_core::Result<i32>;
    fn SetGraceLoginsAllowed(&self, lngraceloginsallowed: i32) -> windows_core::Result<()>;
    fn GraceLoginsRemaining(&self) -> windows_core::Result<i32>;
    fn SetGraceLoginsRemaining(&self, lngraceloginsremaining: i32) -> windows_core::Result<()>;
    fn IsAccountLocked(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetIsAccountLocked(&self, fisaccountlocked: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn LoginHours(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetLoginHours(&self, vloginhours: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn LoginWorkstations(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetLoginWorkstations(&self, vloginworkstations: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn MaxLogins(&self) -> windows_core::Result<i32>;
    fn SetMaxLogins(&self, lnmaxlogins: i32) -> windows_core::Result<()>;
    fn MaxStorage(&self) -> windows_core::Result<i32>;
    fn SetMaxStorage(&self, lnmaxstorage: i32) -> windows_core::Result<()>;
    fn PasswordExpirationDate(&self) -> windows_core::Result<f64>;
    fn SetPasswordExpirationDate(&self, dapasswordexpirationdate: f64) -> windows_core::Result<()>;
    fn PasswordMinimumLength(&self) -> windows_core::Result<i32>;
    fn SetPasswordMinimumLength(&self, lnpasswordminimumlength: i32) -> windows_core::Result<()>;
    fn PasswordRequired(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetPasswordRequired(&self, fpasswordrequired: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RequireUniquePassword(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetRequireUniquePassword(&self, frequireuniquepassword: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EmailAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEmailAddress(&self, bstremailaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn HomeDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetHomeDirectory(&self, bstrhomedirectory: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Languages(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetLanguages(&self, vlanguages: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Profile(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProfile(&self, bstrprofile: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LoginScript(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLoginScript(&self, bstrloginscript: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Picture(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetPicture(&self, vpicture: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn HomePage(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetHomePage(&self, bstrhomepage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Groups(&self) -> windows_core::Result<IADsMembers>;
    fn SetPassword(&self, newpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ChangePassword(&self, bstroldpassword: &windows_core::BSTR, bstrnewpassword: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsUser_Vtbl {
    pub const fn new<Identity: IADsUser_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BadLoginAddress<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::BadLoginAddress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BadLoginCount<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::BadLoginCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastLogin<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::LastLogin(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastLogoff<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::LastLogoff(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastFailedLogin<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::LastFailedLogin(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PasswordLastChanged<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::PasswordLastChanged(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn Division<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::Division(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDivision<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdivision: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetDivision(this, core::mem::transmute(&bstrdivision)).into()
            }
        }
        unsafe extern "system" fn Department<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::Department(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDepartment<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdepartment: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetDepartment(this, core::mem::transmute(&bstrdepartment)).into()
            }
        }
        unsafe extern "system" fn EmployeeID<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::EmployeeID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEmployeeID<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstremployeeid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetEmployeeID(this, core::mem::transmute(&bstremployeeid)).into()
            }
        }
        unsafe extern "system" fn FullName<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::FullName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFullName<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfullname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetFullName(this, core::mem::transmute(&bstrfullname)).into()
            }
        }
        unsafe extern "system" fn FirstName<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::FirstName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFirstName<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfirstname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetFirstName(this, core::mem::transmute(&bstrfirstname)).into()
            }
        }
        unsafe extern "system" fn LastName<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::LastName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLastName<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlastname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetLastName(this, core::mem::transmute(&bstrlastname)).into()
            }
        }
        unsafe extern "system" fn OtherName<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::OtherName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOtherName<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrothername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetOtherName(this, core::mem::transmute(&bstrothername)).into()
            }
        }
        unsafe extern "system" fn NamePrefix<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::NamePrefix(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNamePrefix<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnameprefix: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetNamePrefix(this, core::mem::transmute(&bstrnameprefix)).into()
            }
        }
        unsafe extern "system" fn NameSuffix<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::NameSuffix(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNameSuffix<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnamesuffix: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetNameSuffix(this, core::mem::transmute(&bstrnamesuffix)).into()
            }
        }
        unsafe extern "system" fn Title<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::Title(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTitle<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtitle: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetTitle(this, core::mem::transmute(&bstrtitle)).into()
            }
        }
        unsafe extern "system" fn Manager<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::Manager(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetManager<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmanager: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetManager(this, core::mem::transmute(&bstrmanager)).into()
            }
        }
        unsafe extern "system" fn TelephoneHome<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::TelephoneHome(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTelephoneHome<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtelephonehome: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetTelephoneHome(this, core::mem::transmute(&vtelephonehome)).into()
            }
        }
        unsafe extern "system" fn TelephoneMobile<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::TelephoneMobile(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTelephoneMobile<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtelephonemobile: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetTelephoneMobile(this, core::mem::transmute(&vtelephonemobile)).into()
            }
        }
        unsafe extern "system" fn TelephoneNumber<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::TelephoneNumber(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtelephonenumber: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetTelephoneNumber(this, core::mem::transmute(&vtelephonenumber)).into()
            }
        }
        unsafe extern "system" fn TelephonePager<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::TelephonePager(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTelephonePager<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtelephonepager: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetTelephonePager(this, core::mem::transmute(&vtelephonepager)).into()
            }
        }
        unsafe extern "system" fn FaxNumber<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::FaxNumber(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vfaxnumber: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetFaxNumber(this, core::mem::transmute(&vfaxnumber)).into()
            }
        }
        unsafe extern "system" fn OfficeLocations<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::OfficeLocations(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOfficeLocations<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vofficelocations: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetOfficeLocations(this, core::mem::transmute(&vofficelocations)).into()
            }
        }
        unsafe extern "system" fn PostalAddresses<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::PostalAddresses(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPostalAddresses<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vpostaladdresses: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetPostalAddresses(this, core::mem::transmute(&vpostaladdresses)).into()
            }
        }
        unsafe extern "system" fn PostalCodes<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::PostalCodes(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPostalCodes<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vpostalcodes: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetPostalCodes(this, core::mem::transmute(&vpostalcodes)).into()
            }
        }
        unsafe extern "system" fn SeeAlso<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::SeeAlso(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSeeAlso<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vseealso: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetSeeAlso(this, core::mem::transmute(&vseealso)).into()
            }
        }
        unsafe extern "system" fn AccountDisabled<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::AccountDisabled(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAccountDisabled<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, faccountdisabled: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetAccountDisabled(this, core::mem::transmute_copy(&faccountdisabled)).into()
            }
        }
        unsafe extern "system" fn AccountExpirationDate<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::AccountExpirationDate(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAccountExpirationDate<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, daaccountexpirationdate: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetAccountExpirationDate(this, core::mem::transmute_copy(&daaccountexpirationdate)).into()
            }
        }
        unsafe extern "system" fn GraceLoginsAllowed<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::GraceLoginsAllowed(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGraceLoginsAllowed<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lngraceloginsallowed: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetGraceLoginsAllowed(this, core::mem::transmute_copy(&lngraceloginsallowed)).into()
            }
        }
        unsafe extern "system" fn GraceLoginsRemaining<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::GraceLoginsRemaining(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGraceLoginsRemaining<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lngraceloginsremaining: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetGraceLoginsRemaining(this, core::mem::transmute_copy(&lngraceloginsremaining)).into()
            }
        }
        unsafe extern "system" fn IsAccountLocked<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::IsAccountLocked(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsAccountLocked<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fisaccountlocked: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetIsAccountLocked(this, core::mem::transmute_copy(&fisaccountlocked)).into()
            }
        }
        unsafe extern "system" fn LoginHours<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::LoginHours(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLoginHours<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vloginhours: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetLoginHours(this, core::mem::transmute(&vloginhours)).into()
            }
        }
        unsafe extern "system" fn LoginWorkstations<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::LoginWorkstations(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLoginWorkstations<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vloginworkstations: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetLoginWorkstations(this, core::mem::transmute(&vloginworkstations)).into()
            }
        }
        unsafe extern "system" fn MaxLogins<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::MaxLogins(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxLogins<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxlogins: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetMaxLogins(this, core::mem::transmute_copy(&lnmaxlogins)).into()
            }
        }
        unsafe extern "system" fn MaxStorage<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::MaxStorage(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxStorage<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxstorage: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetMaxStorage(this, core::mem::transmute_copy(&lnmaxstorage)).into()
            }
        }
        unsafe extern "system" fn PasswordExpirationDate<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::PasswordExpirationDate(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPasswordExpirationDate<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dapasswordexpirationdate: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetPasswordExpirationDate(this, core::mem::transmute_copy(&dapasswordexpirationdate)).into()
            }
        }
        unsafe extern "system" fn PasswordMinimumLength<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::PasswordMinimumLength(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPasswordMinimumLength<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnpasswordminimumlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetPasswordMinimumLength(this, core::mem::transmute_copy(&lnpasswordminimumlength)).into()
            }
        }
        unsafe extern "system" fn PasswordRequired<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::PasswordRequired(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPasswordRequired<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fpasswordrequired: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetPasswordRequired(this, core::mem::transmute_copy(&fpasswordrequired)).into()
            }
        }
        unsafe extern "system" fn RequireUniquePassword<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::RequireUniquePassword(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRequireUniquePassword<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frequireuniquepassword: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetRequireUniquePassword(this, core::mem::transmute_copy(&frequireuniquepassword)).into()
            }
        }
        unsafe extern "system" fn EmailAddress<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::EmailAddress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEmailAddress<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstremailaddress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetEmailAddress(this, core::mem::transmute(&bstremailaddress)).into()
            }
        }
        unsafe extern "system" fn HomeDirectory<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::HomeDirectory(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHomeDirectory<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhomedirectory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetHomeDirectory(this, core::mem::transmute(&bstrhomedirectory)).into()
            }
        }
        unsafe extern "system" fn Languages<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::Languages(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLanguages<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vlanguages: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetLanguages(this, core::mem::transmute(&vlanguages)).into()
            }
        }
        unsafe extern "system" fn Profile<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::Profile(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProfile<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetProfile(this, core::mem::transmute(&bstrprofile)).into()
            }
        }
        unsafe extern "system" fn LoginScript<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::LoginScript(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLoginScript<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrloginscript: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetLoginScript(this, core::mem::transmute(&bstrloginscript)).into()
            }
        }
        unsafe extern "system" fn Picture<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::Picture(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPicture<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vpicture: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetPicture(this, core::mem::transmute(&vpicture)).into()
            }
        }
        unsafe extern "system" fn HomePage<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::HomePage(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHomePage<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhomepage: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetHomePage(this, core::mem::transmute(&bstrhomepage)).into()
            }
        }
        unsafe extern "system" fn Groups<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::Groups(this) {
                    Ok(ok__) => {
                        ppgroups.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPassword<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newpassword: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::SetPassword(this, core::mem::transmute(&newpassword)).into()
            }
        }
        unsafe extern "system" fn ChangePassword<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroldpassword: *mut core::ffi::c_void, bstrnewpassword: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::ChangePassword(this, core::mem::transmute(&bstroldpassword), core::mem::transmute(&bstrnewpassword)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            BadLoginAddress: BadLoginAddress::<Identity, OFFSET>,
            BadLoginCount: BadLoginCount::<Identity, OFFSET>,
            LastLogin: LastLogin::<Identity, OFFSET>,
            LastLogoff: LastLogoff::<Identity, OFFSET>,
            LastFailedLogin: LastFailedLogin::<Identity, OFFSET>,
            PasswordLastChanged: PasswordLastChanged::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            Division: Division::<Identity, OFFSET>,
            SetDivision: SetDivision::<Identity, OFFSET>,
            Department: Department::<Identity, OFFSET>,
            SetDepartment: SetDepartment::<Identity, OFFSET>,
            EmployeeID: EmployeeID::<Identity, OFFSET>,
            SetEmployeeID: SetEmployeeID::<Identity, OFFSET>,
            FullName: FullName::<Identity, OFFSET>,
            SetFullName: SetFullName::<Identity, OFFSET>,
            FirstName: FirstName::<Identity, OFFSET>,
            SetFirstName: SetFirstName::<Identity, OFFSET>,
            LastName: LastName::<Identity, OFFSET>,
            SetLastName: SetLastName::<Identity, OFFSET>,
            OtherName: OtherName::<Identity, OFFSET>,
            SetOtherName: SetOtherName::<Identity, OFFSET>,
            NamePrefix: NamePrefix::<Identity, OFFSET>,
            SetNamePrefix: SetNamePrefix::<Identity, OFFSET>,
            NameSuffix: NameSuffix::<Identity, OFFSET>,
            SetNameSuffix: SetNameSuffix::<Identity, OFFSET>,
            Title: Title::<Identity, OFFSET>,
            SetTitle: SetTitle::<Identity, OFFSET>,
            Manager: Manager::<Identity, OFFSET>,
            SetManager: SetManager::<Identity, OFFSET>,
            TelephoneHome: TelephoneHome::<Identity, OFFSET>,
            SetTelephoneHome: SetTelephoneHome::<Identity, OFFSET>,
            TelephoneMobile: TelephoneMobile::<Identity, OFFSET>,
            SetTelephoneMobile: SetTelephoneMobile::<Identity, OFFSET>,
            TelephoneNumber: TelephoneNumber::<Identity, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, OFFSET>,
            TelephonePager: TelephonePager::<Identity, OFFSET>,
            SetTelephonePager: SetTelephonePager::<Identity, OFFSET>,
            FaxNumber: FaxNumber::<Identity, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, OFFSET>,
            OfficeLocations: OfficeLocations::<Identity, OFFSET>,
            SetOfficeLocations: SetOfficeLocations::<Identity, OFFSET>,
            PostalAddresses: PostalAddresses::<Identity, OFFSET>,
            SetPostalAddresses: SetPostalAddresses::<Identity, OFFSET>,
            PostalCodes: PostalCodes::<Identity, OFFSET>,
            SetPostalCodes: SetPostalCodes::<Identity, OFFSET>,
            SeeAlso: SeeAlso::<Identity, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, OFFSET>,
            AccountDisabled: AccountDisabled::<Identity, OFFSET>,
            SetAccountDisabled: SetAccountDisabled::<Identity, OFFSET>,
            AccountExpirationDate: AccountExpirationDate::<Identity, OFFSET>,
            SetAccountExpirationDate: SetAccountExpirationDate::<Identity, OFFSET>,
            GraceLoginsAllowed: GraceLoginsAllowed::<Identity, OFFSET>,
            SetGraceLoginsAllowed: SetGraceLoginsAllowed::<Identity, OFFSET>,
            GraceLoginsRemaining: GraceLoginsRemaining::<Identity, OFFSET>,
            SetGraceLoginsRemaining: SetGraceLoginsRemaining::<Identity, OFFSET>,
            IsAccountLocked: IsAccountLocked::<Identity, OFFSET>,
            SetIsAccountLocked: SetIsAccountLocked::<Identity, OFFSET>,
            LoginHours: LoginHours::<Identity, OFFSET>,
            SetLoginHours: SetLoginHours::<Identity, OFFSET>,
            LoginWorkstations: LoginWorkstations::<Identity, OFFSET>,
            SetLoginWorkstations: SetLoginWorkstations::<Identity, OFFSET>,
            MaxLogins: MaxLogins::<Identity, OFFSET>,
            SetMaxLogins: SetMaxLogins::<Identity, OFFSET>,
            MaxStorage: MaxStorage::<Identity, OFFSET>,
            SetMaxStorage: SetMaxStorage::<Identity, OFFSET>,
            PasswordExpirationDate: PasswordExpirationDate::<Identity, OFFSET>,
            SetPasswordExpirationDate: SetPasswordExpirationDate::<Identity, OFFSET>,
            PasswordMinimumLength: PasswordMinimumLength::<Identity, OFFSET>,
            SetPasswordMinimumLength: SetPasswordMinimumLength::<Identity, OFFSET>,
            PasswordRequired: PasswordRequired::<Identity, OFFSET>,
            SetPasswordRequired: SetPasswordRequired::<Identity, OFFSET>,
            RequireUniquePassword: RequireUniquePassword::<Identity, OFFSET>,
            SetRequireUniquePassword: SetRequireUniquePassword::<Identity, OFFSET>,
            EmailAddress: EmailAddress::<Identity, OFFSET>,
            SetEmailAddress: SetEmailAddress::<Identity, OFFSET>,
            HomeDirectory: HomeDirectory::<Identity, OFFSET>,
            SetHomeDirectory: SetHomeDirectory::<Identity, OFFSET>,
            Languages: Languages::<Identity, OFFSET>,
            SetLanguages: SetLanguages::<Identity, OFFSET>,
            Profile: Profile::<Identity, OFFSET>,
            SetProfile: SetProfile::<Identity, OFFSET>,
            LoginScript: LoginScript::<Identity, OFFSET>,
            SetLoginScript: SetLoginScript::<Identity, OFFSET>,
            Picture: Picture::<Identity, OFFSET>,
            SetPicture: SetPicture::<Identity, OFFSET>,
            HomePage: HomePage::<Identity, OFFSET>,
            SetHomePage: SetHomePage::<Identity, OFFSET>,
            Groups: Groups::<Identity, OFFSET>,
            SetPassword: SetPassword::<Identity, OFFSET>,
            ChangePassword: ChangePassword::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsUser as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsUser {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IADsWinNTSystemInfo, IADsWinNTSystemInfo_Vtbl, 0x6c6d65dc_afd1_11d2_9cb9_0000f87a369e);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IADsWinNTSystemInfo {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IADsWinNTSystemInfo, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IADsWinNTSystemInfo {
    pub unsafe fn UserName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ComputerName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputerName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DomainName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DomainName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn PDC(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsWinNTSystemInfo_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComputerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DomainName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IADsWinNTSystemInfo_Impl: super::oaidl::IDispatch_Impl {
    fn UserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ComputerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DomainName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PDC(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IADsWinNTSystemInfo_Vtbl {
    pub const fn new<Identity: IADsWinNTSystemInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UserName<Identity: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsWinNTSystemInfo_Impl::UserName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ComputerName<Identity: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsWinNTSystemInfo_Impl::ComputerName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DomainName<Identity: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsWinNTSystemInfo_Impl::DomainName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PDC<Identity: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsWinNTSystemInfo_Impl::PDC(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            UserName: UserName::<Identity, OFFSET>,
            ComputerName: ComputerName::<Identity, OFFSET>,
            DomainName: DomainName::<Identity, OFFSET>,
            PDC: PDC::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsWinNTSystemInfo as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IADsWinNTSystemInfo {}
windows_core::imp::define_interface!(IDirectoryObject, IDirectoryObject_Vtbl, 0xe798de2c_22e4_11d0_84fe_00c04fd8d503);
windows_core::imp::interface_hierarchy!(IDirectoryObject, windows_core::IUnknown);
impl IDirectoryObject {
    pub unsafe fn GetObjectInformation(&self) -> windows_core::Result<PADS_OBJECT_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectInformation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
    pub unsafe fn GetObjectAttributes(&self, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: u32, ppattributeentries: *mut PADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetObjectAttributes)(windows_core::Interface::as_raw(self), pattributenames, dwnumberattributes, ppattributeentries as _, pdwnumattributesreturned as _) }
    }
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
    pub unsafe fn SetObjectAttributes(&self, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetObjectAttributes)(windows_core::Interface::as_raw(self), pattributeentries, dwnumattributes, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_oaidl"))]
    pub unsafe fn CreateDSObject<P0>(&self, pszrdnname: P0, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> windows_core::Result<super::oaidl::IDispatch>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDSObject)(windows_core::Interface::as_raw(self), pszrdnname.param().abi(), pattributeentries, dwnumattributes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteDSObject<P0>(&self, pszrdnname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteDSObject)(windows_core::Interface::as_raw(self), pszrdnname.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectoryObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetObjectInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PADS_OBJECT_INFO) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
    pub GetObjectAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, u32, *mut PADS_ATTR_INFO, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwinbase", feature = "Win32_minwindef")))]
    GetObjectAttributes: usize,
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
    pub SetObjectAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const ADS_ATTR_INFO, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwinbase", feature = "Win32_minwindef")))]
    SetObjectAttributes: usize,
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_oaidl"))]
    pub CreateDSObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const ADS_ATTR_INFO, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_oaidl")))]
    CreateDSObject: usize,
    pub DeleteDSObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_oaidl"))]
pub trait IDirectoryObject_Impl: windows_core::IUnknownImpl {
    fn GetObjectInformation(&self) -> windows_core::Result<PADS_OBJECT_INFO>;
    fn GetObjectAttributes(&self, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: u32, ppattributeentries: *mut PADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> windows_core::Result<()>;
    fn SetObjectAttributes(&self, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> windows_core::Result<u32>;
    fn CreateDSObject(&self, pszrdnname: &windows_core::PCWSTR, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> windows_core::Result<super::oaidl::IDispatch>;
    fn DeleteDSObject(&self, pszrdnname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_oaidl"))]
impl IDirectoryObject_Vtbl {
    pub const fn new<Identity: IDirectoryObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetObjectInformation<Identity: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobjinfo: *mut PADS_OBJECT_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectoryObject_Impl::GetObjectInformation(this) {
                    Ok(ok__) => {
                        ppobjinfo.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObjectAttributes<Identity: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: u32, ppattributeentries: *mut PADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectoryObject_Impl::GetObjectAttributes(this, core::mem::transmute_copy(&pattributenames), core::mem::transmute_copy(&dwnumberattributes), core::mem::transmute_copy(&ppattributeentries), core::mem::transmute_copy(&pdwnumattributesreturned)).into()
            }
        }
        unsafe extern "system" fn SetObjectAttributes<Identity: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, pdwnumattributesmodified: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectoryObject_Impl::SetObjectAttributes(this, core::mem::transmute_copy(&pattributeentries), core::mem::transmute_copy(&dwnumattributes)) {
                    Ok(ok__) => {
                        pdwnumattributesmodified.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDSObject<Identity: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrdnname: windows_core::PCWSTR, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectoryObject_Impl::CreateDSObject(this, core::mem::transmute(&pszrdnname), core::mem::transmute_copy(&pattributeentries), core::mem::transmute_copy(&dwnumattributes)) {
                    Ok(ok__) => {
                        ppobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteDSObject<Identity: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrdnname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectoryObject_Impl::DeleteDSObject(this, core::mem::transmute(&pszrdnname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetObjectInformation: GetObjectInformation::<Identity, OFFSET>,
            GetObjectAttributes: GetObjectAttributes::<Identity, OFFSET>,
            SetObjectAttributes: SetObjectAttributes::<Identity, OFFSET>,
            CreateDSObject: CreateDSObject::<Identity, OFFSET>,
            DeleteDSObject: DeleteDSObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectoryObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_oaidl"))]
impl windows_core::RuntimeName for IDirectoryObject {}
windows_core::imp::define_interface!(IDirectorySchemaMgmt, IDirectorySchemaMgmt_Vtbl, 0x75db3b9c_a4d8_11d0_a79c_00c04fd8d5a8);
windows_core::imp::interface_hierarchy!(IDirectorySchemaMgmt, windows_core::IUnknown);
impl IDirectorySchemaMgmt {
    pub unsafe fn EnumAttributes(&self, ppszattrnames: *mut windows_core::PWSTR, dwnumattributes: u32, ppattrdefinition: *mut PADS_ATTR_DEF, pdwnumattributes: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumAttributes)(windows_core::Interface::as_raw(self), ppszattrnames as _, dwnumattributes, ppattrdefinition as _, pdwnumattributes as _) }
    }
    pub unsafe fn CreateAttributeDefinition<P0>(&self, pszattributename: P0, pattributedefinition: *mut ADS_ATTR_DEF) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateAttributeDefinition)(windows_core::Interface::as_raw(self), pszattributename.param().abi(), pattributedefinition as _) }
    }
    pub unsafe fn WriteAttributeDefinition<P0>(&self, pszattributename: P0, pattributedefinition: *mut ADS_ATTR_DEF) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteAttributeDefinition)(windows_core::Interface::as_raw(self), pszattributename.param().abi(), pattributedefinition as _) }
    }
    pub unsafe fn DeleteAttributeDefinition<P0>(&self, pszattributename: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteAttributeDefinition)(windows_core::Interface::as_raw(self), pszattributename.param().abi()) }
    }
    pub unsafe fn EnumClasses(&self, ppszclassnames: *mut windows_core::PWSTR, dwnumclasses: u32, ppclassdefinition: *mut PADS_CLASS_DEF, pdwnumclasses: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumClasses)(windows_core::Interface::as_raw(self), ppszclassnames as _, dwnumclasses, ppclassdefinition as _, pdwnumclasses as _) }
    }
    pub unsafe fn WriteClassDefinition<P0>(&self, pszclassname: P0, pclassdefinition: *mut ADS_CLASS_DEF) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteClassDefinition)(windows_core::Interface::as_raw(self), pszclassname.param().abi(), pclassdefinition as _) }
    }
    pub unsafe fn CreateClassDefinition<P0>(&self, pszclassname: P0, pclassdefinition: *mut ADS_CLASS_DEF) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateClassDefinition)(windows_core::Interface::as_raw(self), pszclassname.param().abi(), pclassdefinition as _) }
    }
    pub unsafe fn DeleteClassDefinition<P0>(&self, pszclassname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteClassDefinition)(windows_core::Interface::as_raw(self), pszclassname.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectorySchemaMgmt_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, u32, *mut PADS_ATTR_DEF, *mut u32) -> windows_core::HRESULT,
    pub CreateAttributeDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut ADS_ATTR_DEF) -> windows_core::HRESULT,
    pub WriteAttributeDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut ADS_ATTR_DEF) -> windows_core::HRESULT,
    pub DeleteAttributeDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnumClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, u32, *mut PADS_CLASS_DEF, *mut u32) -> windows_core::HRESULT,
    pub WriteClassDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut ADS_CLASS_DEF) -> windows_core::HRESULT,
    pub CreateClassDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut ADS_CLASS_DEF) -> windows_core::HRESULT,
    pub DeleteClassDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IDirectorySchemaMgmt_Impl: windows_core::IUnknownImpl {
    fn EnumAttributes(&self, ppszattrnames: *mut windows_core::PWSTR, dwnumattributes: u32, ppattrdefinition: *mut PADS_ATTR_DEF, pdwnumattributes: *mut u32) -> windows_core::Result<()>;
    fn CreateAttributeDefinition(&self, pszattributename: &windows_core::PCWSTR, pattributedefinition: *mut ADS_ATTR_DEF) -> windows_core::Result<()>;
    fn WriteAttributeDefinition(&self, pszattributename: &windows_core::PCWSTR, pattributedefinition: *mut ADS_ATTR_DEF) -> windows_core::Result<()>;
    fn DeleteAttributeDefinition(&self, pszattributename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumClasses(&self, ppszclassnames: *mut windows_core::PWSTR, dwnumclasses: u32, ppclassdefinition: *mut PADS_CLASS_DEF, pdwnumclasses: *mut u32) -> windows_core::Result<()>;
    fn WriteClassDefinition(&self, pszclassname: &windows_core::PCWSTR, pclassdefinition: *mut ADS_CLASS_DEF) -> windows_core::Result<()>;
    fn CreateClassDefinition(&self, pszclassname: &windows_core::PCWSTR, pclassdefinition: *mut ADS_CLASS_DEF) -> windows_core::Result<()>;
    fn DeleteClassDefinition(&self, pszclassname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IDirectorySchemaMgmt_Vtbl {
    pub const fn new<Identity: IDirectorySchemaMgmt_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumAttributes<Identity: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszattrnames: *mut windows_core::PWSTR, dwnumattributes: u32, ppattrdefinition: *mut PADS_ATTR_DEF, pdwnumattributes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySchemaMgmt_Impl::EnumAttributes(this, core::mem::transmute_copy(&ppszattrnames), core::mem::transmute_copy(&dwnumattributes), core::mem::transmute_copy(&ppattrdefinition), core::mem::transmute_copy(&pdwnumattributes)).into()
            }
        }
        unsafe extern "system" fn CreateAttributeDefinition<Identity: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszattributename: windows_core::PCWSTR, pattributedefinition: *mut ADS_ATTR_DEF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySchemaMgmt_Impl::CreateAttributeDefinition(this, core::mem::transmute(&pszattributename), core::mem::transmute_copy(&pattributedefinition)).into()
            }
        }
        unsafe extern "system" fn WriteAttributeDefinition<Identity: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszattributename: windows_core::PCWSTR, pattributedefinition: *mut ADS_ATTR_DEF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySchemaMgmt_Impl::WriteAttributeDefinition(this, core::mem::transmute(&pszattributename), core::mem::transmute_copy(&pattributedefinition)).into()
            }
        }
        unsafe extern "system" fn DeleteAttributeDefinition<Identity: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszattributename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySchemaMgmt_Impl::DeleteAttributeDefinition(this, core::mem::transmute(&pszattributename)).into()
            }
        }
        unsafe extern "system" fn EnumClasses<Identity: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszclassnames: *mut windows_core::PWSTR, dwnumclasses: u32, ppclassdefinition: *mut PADS_CLASS_DEF, pdwnumclasses: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySchemaMgmt_Impl::EnumClasses(this, core::mem::transmute_copy(&ppszclassnames), core::mem::transmute_copy(&dwnumclasses), core::mem::transmute_copy(&ppclassdefinition), core::mem::transmute_copy(&pdwnumclasses)).into()
            }
        }
        unsafe extern "system" fn WriteClassDefinition<Identity: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszclassname: windows_core::PCWSTR, pclassdefinition: *mut ADS_CLASS_DEF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySchemaMgmt_Impl::WriteClassDefinition(this, core::mem::transmute(&pszclassname), core::mem::transmute_copy(&pclassdefinition)).into()
            }
        }
        unsafe extern "system" fn CreateClassDefinition<Identity: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszclassname: windows_core::PCWSTR, pclassdefinition: *mut ADS_CLASS_DEF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySchemaMgmt_Impl::CreateClassDefinition(this, core::mem::transmute(&pszclassname), core::mem::transmute_copy(&pclassdefinition)).into()
            }
        }
        unsafe extern "system" fn DeleteClassDefinition<Identity: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszclassname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySchemaMgmt_Impl::DeleteClassDefinition(this, core::mem::transmute(&pszclassname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumAttributes: EnumAttributes::<Identity, OFFSET>,
            CreateAttributeDefinition: CreateAttributeDefinition::<Identity, OFFSET>,
            WriteAttributeDefinition: WriteAttributeDefinition::<Identity, OFFSET>,
            DeleteAttributeDefinition: DeleteAttributeDefinition::<Identity, OFFSET>,
            EnumClasses: EnumClasses::<Identity, OFFSET>,
            WriteClassDefinition: WriteClassDefinition::<Identity, OFFSET>,
            CreateClassDefinition: CreateClassDefinition::<Identity, OFFSET>,
            DeleteClassDefinition: DeleteClassDefinition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectorySchemaMgmt as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectorySchemaMgmt {}
windows_core::imp::define_interface!(IDirectorySearch, IDirectorySearch_Vtbl, 0x109ba8ec_92f0_11d0_a790_00c04fd8d5a8);
windows_core::imp::interface_hierarchy!(IDirectorySearch, windows_core::IUnknown);
impl IDirectorySearch {
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
    pub unsafe fn SetSearchPreference(&self, psearchprefs: *const ADS_SEARCHPREF_INFO, dwnumprefs: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSearchPreference)(windows_core::Interface::as_raw(self), psearchprefs, dwnumprefs) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn ExecuteSearch<P0>(&self, pszsearchfilter: P0, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: u32) -> windows_core::Result<super::winnt::HANDLE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecuteSearch)(windows_core::Interface::as_raw(self), pszsearchfilter.param().abi(), pattributenames, dwnumberattributes, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn AbandonSearch(&self, phsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AbandonSearch)(windows_core::Interface::as_raw(self), phsearchresult) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetFirstRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFirstRow)(windows_core::Interface::as_raw(self), hsearchresult) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetNextRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNextRow)(windows_core::Interface::as_raw(self), hsearchresult) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetPreviousRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPreviousRow)(windows_core::Interface::as_raw(self), hsearchresult) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetNextColumnName(&self, hsearchhandle: ADS_SEARCH_HANDLE) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextColumnName)(windows_core::Interface::as_raw(self), hsearchhandle, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
    pub unsafe fn GetColumn<P1>(&self, hsearchresult: ADS_SEARCH_HANDLE, szcolumnname: P1, psearchcolumn: *mut ADS_SEARCH_COLUMN) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetColumn)(windows_core::Interface::as_raw(self), hsearchresult, szcolumnname.param().abi(), psearchcolumn as _) }
    }
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
    pub unsafe fn FreeColumn(&self, psearchcolumn: *const ADS_SEARCH_COLUMN) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeColumn)(windows_core::Interface::as_raw(self), psearchcolumn) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn CloseSearchHandle(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CloseSearchHandle)(windows_core::Interface::as_raw(self), hsearchresult) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectorySearch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
    pub SetSearchPreference: unsafe extern "system" fn(*mut core::ffi::c_void, *const ADS_SEARCHPREF_INFO, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwinbase", feature = "Win32_minwindef")))]
    SetSearchPreference: usize,
    #[cfg(feature = "Win32_winnt")]
    pub ExecuteSearch: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::PCWSTR, u32, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    ExecuteSearch: usize,
    #[cfg(feature = "Win32_winnt")]
    pub AbandonSearch: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    AbandonSearch: usize,
    #[cfg(feature = "Win32_winnt")]
    pub GetFirstRow: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetFirstRow: usize,
    #[cfg(feature = "Win32_winnt")]
    pub GetNextRow: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetNextRow: usize,
    #[cfg(feature = "Win32_winnt")]
    pub GetPreviousRow: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetPreviousRow: usize,
    #[cfg(feature = "Win32_winnt")]
    pub GetNextColumnName: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetNextColumnName: usize,
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
    pub GetColumn: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE, windows_core::PCWSTR, *mut ADS_SEARCH_COLUMN) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt")))]
    GetColumn: usize,
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
    pub FreeColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *const ADS_SEARCH_COLUMN) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt")))]
    FreeColumn: usize,
    #[cfg(feature = "Win32_winnt")]
    pub CloseSearchHandle: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    CloseSearchHandle: usize,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub trait IDirectorySearch_Impl: windows_core::IUnknownImpl {
    fn SetSearchPreference(&self, psearchprefs: *const ADS_SEARCHPREF_INFO, dwnumprefs: u32) -> windows_core::Result<()>;
    fn ExecuteSearch(&self, pszsearchfilter: &windows_core::PCWSTR, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: u32) -> windows_core::Result<super::winnt::HANDLE>;
    fn AbandonSearch(&self, phsearchresult: ADS_SEARCH_HANDLE) -> windows_core::Result<()>;
    fn GetFirstRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::Result<()>;
    fn GetNextRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::Result<()>;
    fn GetPreviousRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::Result<()>;
    fn GetNextColumnName(&self, hsearchhandle: ADS_SEARCH_HANDLE) -> windows_core::Result<windows_core::PWSTR>;
    fn GetColumn(&self, hsearchresult: ADS_SEARCH_HANDLE, szcolumnname: &windows_core::PCWSTR, psearchcolumn: *mut ADS_SEARCH_COLUMN) -> windows_core::Result<()>;
    fn FreeColumn(&self, psearchcolumn: *const ADS_SEARCH_COLUMN) -> windows_core::Result<()>;
    fn CloseSearchHandle(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl IDirectorySearch_Vtbl {
    pub const fn new<Identity: IDirectorySearch_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSearchPreference<Identity: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psearchprefs: *const ADS_SEARCHPREF_INFO, dwnumprefs: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySearch_Impl::SetSearchPreference(this, core::mem::transmute_copy(&psearchprefs), core::mem::transmute_copy(&dwnumprefs)).into()
            }
        }
        unsafe extern "system" fn ExecuteSearch<Identity: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsearchfilter: windows_core::PCWSTR, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: u32, phsearchresult: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectorySearch_Impl::ExecuteSearch(this, core::mem::transmute(&pszsearchfilter), core::mem::transmute_copy(&pattributenames), core::mem::transmute_copy(&dwnumberattributes)) {
                    Ok(ok__) => {
                        phsearchresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AbandonSearch<Identity: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySearch_Impl::AbandonSearch(this, core::mem::transmute_copy(&phsearchresult)).into()
            }
        }
        unsafe extern "system" fn GetFirstRow<Identity: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySearch_Impl::GetFirstRow(this, core::mem::transmute_copy(&hsearchresult)).into()
            }
        }
        unsafe extern "system" fn GetNextRow<Identity: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySearch_Impl::GetNextRow(this, core::mem::transmute_copy(&hsearchresult)).into()
            }
        }
        unsafe extern "system" fn GetPreviousRow<Identity: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySearch_Impl::GetPreviousRow(this, core::mem::transmute_copy(&hsearchresult)).into()
            }
        }
        unsafe extern "system" fn GetNextColumnName<Identity: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsearchhandle: ADS_SEARCH_HANDLE, ppszcolumnname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectorySearch_Impl::GetNextColumnName(this, core::mem::transmute_copy(&hsearchhandle)) {
                    Ok(ok__) => {
                        ppszcolumnname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColumn<Identity: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE, szcolumnname: windows_core::PCWSTR, psearchcolumn: *mut ADS_SEARCH_COLUMN) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySearch_Impl::GetColumn(this, core::mem::transmute_copy(&hsearchresult), core::mem::transmute(&szcolumnname), core::mem::transmute_copy(&psearchcolumn)).into()
            }
        }
        unsafe extern "system" fn FreeColumn<Identity: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psearchcolumn: *const ADS_SEARCH_COLUMN) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySearch_Impl::FreeColumn(this, core::mem::transmute_copy(&psearchcolumn)).into()
            }
        }
        unsafe extern "system" fn CloseSearchHandle<Identity: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySearch_Impl::CloseSearchHandle(this, core::mem::transmute_copy(&hsearchresult)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSearchPreference: SetSearchPreference::<Identity, OFFSET>,
            ExecuteSearch: ExecuteSearch::<Identity, OFFSET>,
            AbandonSearch: AbandonSearch::<Identity, OFFSET>,
            GetFirstRow: GetFirstRow::<Identity, OFFSET>,
            GetNextRow: GetNextRow::<Identity, OFFSET>,
            GetPreviousRow: GetPreviousRow::<Identity, OFFSET>,
            GetNextColumnName: GetNextColumnName::<Identity, OFFSET>,
            GetColumn: GetColumn::<Identity, OFFSET>,
            FreeColumn: FreeColumn::<Identity, OFFSET>,
            CloseSearchHandle: CloseSearchHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectorySearch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDirectorySearch {}
windows_core::imp::define_interface!(IPrivateDispatch, IPrivateDispatch_Vtbl, 0x86ab4bbe_65f6_11d1_8c13_00c04fd8d503);
windows_core::imp::interface_hierarchy!(IPrivateDispatch, windows_core::IUnknown);
impl IPrivateDispatch {
    pub unsafe fn ADSIInitializeDispatchManager(&self, dwextensionid: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ADSIInitializeDispatchManager)(windows_core::Interface::as_raw(self), dwextensionid) }
    }
    pub unsafe fn ADSIGetTypeInfoCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ADSIGetTypeInfoCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
    pub unsafe fn ADSIGetTypeInfo(&self, itinfo: u32, lcid: super::winnt::LCID) -> windows_core::Result<super::oaidl::ITypeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ADSIGetTypeInfo)(windows_core::Interface::as_raw(self), itinfo, lcid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypesbase"))]
    pub unsafe fn ADSIGetIDsOfNames(&self, riid: *const windows_core::GUID, rgsznames: *const *const super::wtypesbase::OLECHAR, cnames: u32, lcid: super::winnt::LCID) -> windows_core::Result<super::oaidl::DISPID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ADSIGetIDsOfNames)(windows_core::Interface::as_raw(self), riid, rgsznames, cnames, lcid, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn ADSIInvoke(&self, dispidmember: super::oaidl::DISPID, riid: *const windows_core::GUID, lcid: super::winnt::LCID, wflags: u16, pdispparams: *const super::oaidl::DISPPARAMS, pvarresult: *mut super::oaidl::VARIANT, pexcepinfo: *mut super::oaidl::EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ADSIInvoke)(windows_core::Interface::as_raw(self), dispidmember, riid, lcid, wflags, pdispparams, core::mem::transmute(pvarresult), core::mem::transmute(pexcepinfo), puargerr as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrivateDispatch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ADSIInitializeDispatchManager: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ADSIGetTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
    pub ADSIGetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::winnt::LCID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_winnt")))]
    ADSIGetTypeInfo: usize,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypesbase"))]
    pub ADSIGetIDsOfNames: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const *const super::wtypesbase::OLECHAR, u32, super::winnt::LCID, *mut super::oaidl::DISPID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypesbase")))]
    ADSIGetIDsOfNames: usize,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub ADSIInvoke: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID, *const windows_core::GUID, super::winnt::LCID, u16, *const super::oaidl::DISPPARAMS, *mut super::oaidl::VARIANT, *mut super::oaidl::EXCEPINFO, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    ADSIInvoke: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IPrivateDispatch_Impl: windows_core::IUnknownImpl {
    fn ADSIInitializeDispatchManager(&self, dwextensionid: i32) -> windows_core::Result<()>;
    fn ADSIGetTypeInfoCount(&self) -> windows_core::Result<u32>;
    fn ADSIGetTypeInfo(&self, itinfo: u32, lcid: super::winnt::LCID) -> windows_core::Result<super::oaidl::ITypeInfo>;
    fn ADSIGetIDsOfNames(&self, riid: *const windows_core::GUID, rgsznames: *const *const super::wtypesbase::OLECHAR, cnames: u32, lcid: super::winnt::LCID) -> windows_core::Result<super::oaidl::DISPID>;
    fn ADSIInvoke(&self, dispidmember: super::oaidl::DISPID, riid: *const windows_core::GUID, lcid: super::winnt::LCID, wflags: u16, pdispparams: *const super::oaidl::DISPPARAMS, pvarresult: *mut super::oaidl::VARIANT, pexcepinfo: *mut super::oaidl::EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IPrivateDispatch_Vtbl {
    pub const fn new<Identity: IPrivateDispatch_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ADSIInitializeDispatchManager<Identity: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwextensionid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrivateDispatch_Impl::ADSIInitializeDispatchManager(this, core::mem::transmute_copy(&dwextensionid)).into()
            }
        }
        unsafe extern "system" fn ADSIGetTypeInfoCount<Identity: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctinfo: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrivateDispatch_Impl::ADSIGetTypeInfoCount(this) {
                    Ok(ok__) => {
                        pctinfo.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ADSIGetTypeInfo<Identity: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itinfo: u32, lcid: super::winnt::LCID, pptinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrivateDispatch_Impl::ADSIGetTypeInfo(this, core::mem::transmute_copy(&itinfo), core::mem::transmute_copy(&lcid)) {
                    Ok(ok__) => {
                        pptinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ADSIGetIDsOfNames<Identity: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, rgsznames: *const *const super::wtypesbase::OLECHAR, cnames: u32, lcid: super::winnt::LCID, rgdispid: *mut super::oaidl::DISPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrivateDispatch_Impl::ADSIGetIDsOfNames(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&rgsznames), core::mem::transmute_copy(&cnames), core::mem::transmute_copy(&lcid)) {
                    Ok(ok__) => {
                        rgdispid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ADSIInvoke<Identity: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispidmember: super::oaidl::DISPID, riid: *const windows_core::GUID, lcid: super::winnt::LCID, wflags: u16, pdispparams: *const super::oaidl::DISPPARAMS, pvarresult: *mut super::oaidl::VARIANT, pexcepinfo: *mut super::oaidl::EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrivateDispatch_Impl::ADSIInvoke(this, core::mem::transmute_copy(&dispidmember), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&pexcepinfo), core::mem::transmute_copy(&puargerr)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ADSIInitializeDispatchManager: ADSIInitializeDispatchManager::<Identity, OFFSET>,
            ADSIGetTypeInfoCount: ADSIGetTypeInfoCount::<Identity, OFFSET>,
            ADSIGetTypeInfo: ADSIGetTypeInfo::<Identity, OFFSET>,
            ADSIGetIDsOfNames: ADSIGetIDsOfNames::<Identity, OFFSET>,
            ADSIInvoke: ADSIInvoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrivateDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IPrivateDispatch {}
windows_core::imp::define_interface!(IPrivateUnknown, IPrivateUnknown_Vtbl, 0x89126bab_6ead_11d1_8c18_00c04fd8d503);
windows_core::imp::interface_hierarchy!(IPrivateUnknown, windows_core::IUnknown);
impl IPrivateUnknown {
    pub unsafe fn ADSIInitializeObject(&self, lpszusername: &windows_core::BSTR, lpszpassword: &windows_core::BSTR, lnreserved: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ADSIInitializeObject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lpszusername), core::mem::transmute_copy(lpszpassword), lnreserved) }
    }
    pub unsafe fn ADSIReleaseObject(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ADSIReleaseObject)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrivateUnknown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ADSIInitializeObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ADSIReleaseObject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrivateUnknown_Impl: windows_core::IUnknownImpl {
    fn ADSIInitializeObject(&self, lpszusername: &windows_core::BSTR, lpszpassword: &windows_core::BSTR, lnreserved: i32) -> windows_core::Result<()>;
    fn ADSIReleaseObject(&self) -> windows_core::Result<()>;
}
impl IPrivateUnknown_Vtbl {
    pub const fn new<Identity: IPrivateUnknown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ADSIInitializeObject<Identity: IPrivateUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszusername: *mut core::ffi::c_void, lpszpassword: *mut core::ffi::c_void, lnreserved: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrivateUnknown_Impl::ADSIInitializeObject(this, core::mem::transmute(&lpszusername), core::mem::transmute(&lpszpassword), core::mem::transmute_copy(&lnreserved)).into()
            }
        }
        unsafe extern "system" fn ADSIReleaseObject<Identity: IPrivateUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrivateUnknown_Impl::ADSIReleaseObject(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ADSIInitializeObject: ADSIInitializeObject::<Identity, OFFSET>,
            ADSIReleaseObject: ADSIReleaseObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrivateUnknown as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrivateUnknown {}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPADSVALUE(pub *mut ADSVALUE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
impl LPADSVALUE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
impl Default for LPADSVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPADS_SEARCHPREF_INFO(pub *mut ADS_SEARCHPREF_INFO);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
impl LPADS_SEARCHPREF_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
impl Default for LPADS_SEARCHPREF_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNDS_BOOLEAN(pub *mut u32);
impl LPNDS_BOOLEAN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPNDS_BOOLEAN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LargeInteger: windows_core::GUID = windows_core::GUID::from_u128(0x927971f5_0939_11d1_8be1_00c04fd8d503);
pub const NameTranslate: windows_core::GUID = windows_core::GUID::from_u128(0x274fae1f_3626_11d1_a3a4_00c04fb950dc);
pub const NetAddress: windows_core::GUID = windows_core::GUID::from_u128(0xb0b71247_4080_11d1_a3ac_00c04fb950dc);
pub const OctetList: windows_core::GUID = windows_core::GUID::from_u128(0x1241400f_4680_11d1_a3b4_00c04fb950dc);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADSVALUE(pub *mut ADSVALUE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
impl PADSVALUE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
impl Default for PADSVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_ATTR_DEF(pub *mut ADS_ATTR_DEF);
impl PADS_ATTR_DEF {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_ATTR_DEF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_ATTR_INFO(pub *mut ADS_ATTR_INFO);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
impl PADS_ATTR_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
impl Default for PADS_ATTR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_BACKLINK(pub *mut ADS_BACKLINK);
impl PADS_BACKLINK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_BACKLINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_CASEIGNORE_LIST(pub *mut ADS_CASEIGNORE_LIST);
impl PADS_CASEIGNORE_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_CASEIGNORE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_CASE_EXACT_STRING(pub *mut windows_core::PWSTR);
impl PADS_CASE_EXACT_STRING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_CASE_EXACT_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_CASE_IGNORE_STRING(pub *mut windows_core::PWSTR);
impl PADS_CASE_IGNORE_STRING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_CASE_IGNORE_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_CLASS_DEF(pub *mut ADS_CLASS_DEF);
impl PADS_CLASS_DEF {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_CLASS_DEF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_DN_STRING(pub *mut windows_core::PWSTR);
impl PADS_DN_STRING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_DN_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_DN_WITH_BINARY(pub *mut ADS_DN_WITH_BINARY);
#[cfg(feature = "Win32_minwindef")]
impl PADS_DN_WITH_BINARY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PADS_DN_WITH_BINARY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_DN_WITH_STRING(pub *mut ADS_DN_WITH_STRING);
impl PADS_DN_WITH_STRING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_DN_WITH_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_EMAIL(pub *mut ADS_EMAIL);
impl PADS_EMAIL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_EMAIL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_FAXNUMBER(pub *mut ADS_FAXNUMBER);
#[cfg(feature = "Win32_minwindef")]
impl PADS_FAXNUMBER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PADS_FAXNUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_HOLD(pub *mut ADS_HOLD);
impl PADS_HOLD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_HOLD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_INTEGER(pub *mut u32);
impl PADS_INTEGER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_INTEGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_LARGE_INTEGER(pub *mut i64);
impl PADS_LARGE_INTEGER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_LARGE_INTEGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_NETADDRESS(pub *mut ADS_NETADDRESS);
impl PADS_NETADDRESS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_NETADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_NT_SECURITY_DESCRIPTOR(pub *mut ADS_NT_SECURITY_DESCRIPTOR);
#[cfg(feature = "Win32_minwindef")]
impl PADS_NT_SECURITY_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PADS_NT_SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_NUMERIC_STRING(pub *mut windows_core::PWSTR);
impl PADS_NUMERIC_STRING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_NUMERIC_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_OBJECT_CLASS(pub *mut windows_core::PWSTR);
impl PADS_OBJECT_CLASS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_OBJECT_CLASS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_OBJECT_INFO(pub *mut ADS_OBJECT_INFO);
impl PADS_OBJECT_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_OBJECT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_OCTET_LIST(pub *mut ADS_OCTET_LIST);
impl PADS_OCTET_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_OCTET_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_OCTET_STRING(pub *mut ADS_OCTET_STRING);
#[cfg(feature = "Win32_minwindef")]
impl PADS_OCTET_STRING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PADS_OCTET_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_PATH(pub *mut ADS_PATH);
impl PADS_PATH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_PATH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_POSTALADDRESS(pub *mut ADS_POSTALADDRESS);
impl PADS_POSTALADDRESS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_POSTALADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_PRINTABLE_STRING(pub *mut windows_core::PWSTR);
impl PADS_PRINTABLE_STRING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_PRINTABLE_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_PROV_SPECIFIC(pub *mut ADS_PROV_SPECIFIC);
#[cfg(feature = "Win32_minwindef")]
impl PADS_PROV_SPECIFIC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PADS_PROV_SPECIFIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_REPLICAPOINTER(pub *mut ADS_REPLICAPOINTER);
impl PADS_REPLICAPOINTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_REPLICAPOINTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_SEARCHPREF_INFO(pub *mut ADS_SEARCHPREF_INFO);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
impl PADS_SEARCHPREF_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
impl Default for PADS_SEARCHPREF_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_SEARCH_COLUMN(pub *mut ADS_SEARCH_COLUMN);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl PADS_SEARCH_COLUMN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for PADS_SEARCH_COLUMN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_SEARCH_HANDLE(pub *mut super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
impl PADS_SEARCH_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PADS_SEARCH_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_SORTKEY(pub *mut ADS_SORTKEY);
impl PADS_SORTKEY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_SORTKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_STATUS(pub *mut ADS_STATUSENUM);
impl PADS_STATUS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_TIMESTAMP(pub *mut ADS_TIMESTAMP);
impl PADS_TIMESTAMP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_TIMESTAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_TYPEDNAME(pub *mut ADS_TYPEDNAME);
impl PADS_TYPEDNAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADS_TYPEDNAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwinbase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_UTC_TIME(pub *mut super::minwinbase::SYSTEMTIME);
#[cfg(feature = "Win32_minwinbase")]
impl PADS_UTC_TIME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwinbase")]
impl Default for PADS_UTC_TIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADS_VLV(pub *mut ADS_VLV);
#[cfg(feature = "Win32_minwindef")]
impl PADS_VLV {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PADS_VLV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const Path: windows_core::GUID = windows_core::GUID::from_u128(0xb2538919_4080_11d1_a3ac_00c04fb950dc);
pub const Pathname: windows_core::GUID = windows_core::GUID::from_u128(0x080d0d78_f421_11d0_a36e_00c04fb950dc);
pub const PostalAddress: windows_core::GUID = windows_core::GUID::from_u128(0x0a75afcd_4680_11d1_a3b4_00c04fb950dc);
pub const PropertyEntry: windows_core::GUID = windows_core::GUID::from_u128(0x72d3edc2_a4c4_11d0_8533_00c04fd8d503);
pub const PropertyValue: windows_core::GUID = windows_core::GUID::from_u128(0x7b9e38b0_a97c_11d0_8534_00c04fd8d503);
pub const ReplicaPointer: windows_core::GUID = windows_core::GUID::from_u128(0xf5d1badf_4080_11d1_a3ac_00c04fb950dc);
pub const SecurityDescriptor: windows_core::GUID = windows_core::GUID::from_u128(0xb958f73c_9bdd_11d0_852c_00c04fd8d503);
pub const Timestamp: windows_core::GUID = windows_core::GUID::from_u128(0xb2bed2eb_4080_11d1_a3ac_00c04fb950dc);
pub const TypedName: windows_core::GUID = windows_core::GUID::from_u128(0xb33143cb_4080_11d1_a3ac_00c04fb950dc);
pub const WinNTSystemInfo: windows_core::GUID = windows_core::GUID::from_u128(0x66182ec4_afd1_11d2_9cb9_0000f87a369e);
