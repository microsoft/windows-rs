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
pub type ADSTYPE = ADSTYPEENUM;
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
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub struct ADSVALUE {
    pub dwType: ADSTYPE,
    pub Anonymous: ADSVALUE_0,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
impl Default for ADSVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
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
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
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
#[derive(Clone, Copy)]
pub struct ADS_ATTR_DEF {
    pub pszAttrName: windows_sys::core::PWSTR,
    pub dwADsType: ADSTYPE,
    pub dwMinRange: u32,
    pub dwMaxRange: u32,
    pub fMultiValued: windows_sys::core::BOOL,
}
impl Default for ADS_ATTR_DEF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ADS_ATTR_DELETE: u32 = 4;
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub struct ADS_ATTR_INFO {
    pub pszAttrName: windows_sys::core::PWSTR,
    pub dwControlCode: u32,
    pub dwADsType: ADSTYPE,
    pub pADsValues: PADSVALUE,
    pub dwNumValues: u32,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
impl Default for ADS_ATTR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ADS_ATTR_UPDATE: u32 = 2;
pub type ADS_AUTHENTICATION_ENUM = i32;
pub const ADS_AUTH_RESERVED: ADS_AUTHENTICATION_ENUM = -2147483648;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ADS_BACKLINK {
    pub RemoteID: u32,
    pub ObjectName: windows_sys::core::PWSTR,
}
impl Default for ADS_BACKLINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ADS_BOOLEAN = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ADS_CASEIGNORE_LIST {
    pub Next: *mut Self,
    pub String: windows_sys::core::PWSTR,
}
impl Default for ADS_CASEIGNORE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ADS_CASE_EXACT_STRING = windows_sys::core::PWSTR;
pub type ADS_CASE_IGNORE_STRING = windows_sys::core::PWSTR;
pub const ADS_CHASE_REFERRALS_ALWAYS: ADS_CHASE_REFERRALS_ENUM = 96;
pub type ADS_CHASE_REFERRALS_ENUM = i32;
pub const ADS_CHASE_REFERRALS_EXTERNAL: ADS_CHASE_REFERRALS_ENUM = 64;
pub const ADS_CHASE_REFERRALS_NEVER: ADS_CHASE_REFERRALS_ENUM = 0;
pub const ADS_CHASE_REFERRALS_SUBORDINATE: ADS_CHASE_REFERRALS_ENUM = 32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ADS_CLASS_DEF {
    pub pszClassName: windows_sys::core::PWSTR,
    pub dwMandatoryAttrs: u32,
    pub ppszMandatoryAttrs: *mut windows_sys::core::PWSTR,
    pub optionalAttrs: u32,
    pub ppszOptionalAttrs: *mut *mut windows_sys::core::PWSTR,
    pub dwNamingAttrs: u32,
    pub ppszNamingAttrs: *mut *mut windows_sys::core::PWSTR,
    pub dwSuperClasses: u32,
    pub ppszSuperClasses: *mut *mut windows_sys::core::PWSTR,
    pub fIsContainer: windows_sys::core::BOOL,
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
pub const ADS_DIRSYNC_COOKIE: windows_sys::core::PCWSTR = windows_sys::core::w!("fc8cb04d-311d-406c-8cb9-1ae8b843b418");
pub type ADS_DISPLAY_ENUM = i32;
pub const ADS_DISPLAY_FULL: ADS_DISPLAY_ENUM = 1;
pub const ADS_DISPLAY_VALUE_ONLY: ADS_DISPLAY_ENUM = 2;
pub type ADS_DN_STRING = windows_sys::core::PWSTR;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct ADS_DN_WITH_BINARY {
    pub dwLength: u32,
    pub lpBinaryValue: super::LPBYTE,
    pub pszDNString: windows_sys::core::PWSTR,
}
#[cfg(feature = "minwindef")]
impl Default for ADS_DN_WITH_BINARY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ADS_DN_WITH_STRING {
    pub pszStringValue: windows_sys::core::PWSTR,
    pub pszDNString: windows_sys::core::PWSTR,
}
impl Default for ADS_DN_WITH_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ADS_EMAIL {
    pub Address: windows_sys::core::PWSTR,
    pub Type: u32,
}
impl Default for ADS_EMAIL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct ADS_FAXNUMBER {
    pub TelephoneNumber: windows_sys::core::PWSTR,
    pub NumberOfBits: u32,
    pub Parameters: super::LPBYTE,
}
#[cfg(feature = "minwindef")]
impl Default for ADS_FAXNUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[derive(Clone, Copy)]
pub struct ADS_HOLD {
    pub ObjectName: windows_sys::core::PWSTR,
    pub Amount: u32,
}
impl Default for ADS_HOLD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ADS_INTEGER = u32;
pub type ADS_LARGE_INTEGER = i64;
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
#[derive(Clone, Copy)]
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct ADS_NT_SECURITY_DESCRIPTOR {
    pub dwLength: u32,
    pub lpValue: super::LPBYTE,
}
#[cfg(feature = "minwindef")]
impl Default for ADS_NT_SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ADS_NUMERIC_STRING = windows_sys::core::PWSTR;
pub type ADS_OBJECT_CLASS = windows_sys::core::PWSTR;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ADS_OBJECT_INFO {
    pub pszRDN: windows_sys::core::PWSTR,
    pub pszObjectDN: windows_sys::core::PWSTR,
    pub pszParentDN: windows_sys::core::PWSTR,
    pub pszSchemaDN: windows_sys::core::PWSTR,
    pub pszClassName: windows_sys::core::PWSTR,
}
impl Default for ADS_OBJECT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct ADS_OCTET_STRING {
    pub dwLength: u32,
    pub lpValue: super::LPBYTE,
}
#[cfg(feature = "minwindef")]
impl Default for ADS_OCTET_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[derive(Clone, Copy)]
pub struct ADS_PATH {
    pub Type: u32,
    pub VolumeName: windows_sys::core::PWSTR,
    pub Path: windows_sys::core::PWSTR,
}
impl Default for ADS_PATH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ADS_PATHTYPE_ENUM = i32;
pub const ADS_PATH_FILE: ADS_PATHTYPE_ENUM = 1;
pub const ADS_PATH_FILESHARE: ADS_PATHTYPE_ENUM = 2;
pub const ADS_PATH_REGISTRY: ADS_PATHTYPE_ENUM = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ADS_POSTALADDRESS {
    pub PostalAddress: [windows_sys::core::PWSTR; 6],
}
impl Default for ADS_POSTALADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ADS_PREFERENCES_ENUM = i32;
pub type ADS_PRINTABLE_STRING = windows_sys::core::PWSTR;
pub const ADS_PROMPT_CREDENTIALS: ADS_AUTHENTICATION_ENUM = 8;
pub const ADS_PROPERTY_APPEND: ADS_PROPERTY_OPERATION_ENUM = 3;
pub const ADS_PROPERTY_CLEAR: ADS_PROPERTY_OPERATION_ENUM = 1;
pub const ADS_PROPERTY_DELETE: ADS_PROPERTY_OPERATION_ENUM = 4;
pub type ADS_PROPERTY_OPERATION_ENUM = i32;
pub const ADS_PROPERTY_UPDATE: ADS_PROPERTY_OPERATION_ENUM = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct ADS_PROV_SPECIFIC {
    pub dwLength: u32,
    pub lpValue: super::LPBYTE,
}
#[cfg(feature = "minwindef")]
impl Default for ADS_PROV_SPECIFIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ADS_READONLY_SERVER: ADS_AUTHENTICATION_ENUM = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ADS_REPLICAPOINTER {
    pub ServerName: windows_sys::core::PWSTR,
    pub ReplicaType: u32,
    pub ReplicaNumber: u32,
    pub Count: u32,
    pub ReplicaAddressHints: PADS_NETADDRESS,
}
impl Default for ADS_REPLICAPOINTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
pub type ADS_SEARCHPREF = ADS_SEARCHPREF_ENUM;
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
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub struct ADS_SEARCHPREF_INFO {
    pub dwSearchPref: ADS_SEARCHPREF,
    pub vValue: ADSVALUE,
    pub dwStatus: ADS_STATUS,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
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
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ADS_SEARCH_COLUMN {
    pub pszAttrName: windows_sys::core::PWSTR,
    pub dwADsType: ADSTYPE,
    pub pADsValues: PADSVALUE,
    pub dwNumValues: u32,
    pub hReserved: super::HANDLE,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
impl Default for ADS_SEARCH_COLUMN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub type ADS_SEARCH_HANDLE = super::HANDLE;
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
#[derive(Clone, Copy)]
pub struct ADS_SORTKEY {
    pub pszAttrType: windows_sys::core::PWSTR,
    pub pszReserved: windows_sys::core::PWSTR,
    pub fReverseorder: bool,
}
impl Default for ADS_SORTKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ADS_STATUS = ADS_STATUSENUM;
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
#[derive(Clone, Copy, Default)]
pub struct ADS_TIMESTAMP {
    pub WholeSeconds: u32,
    pub EventID: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ADS_TYPEDNAME {
    pub ObjectName: windows_sys::core::PWSTR,
    pub Level: u32,
    pub Interval: u32,
}
impl Default for ADS_TYPEDNAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[cfg(feature = "minwinbase")]
pub type ADS_UTC_TIME = super::SYSTEMTIME;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct ADS_VLV {
    pub dwBeforeCount: u32,
    pub dwAfterCount: u32,
    pub dwOffset: u32,
    pub dwContentCount: u32,
    pub pszTarget: windows_sys::core::PWSTR,
    pub dwContextIDLength: u32,
    pub lpContextID: super::LPBYTE,
}
#[cfg(feature = "minwindef")]
impl Default for ADS_VLV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ADS_VLV_RESPONSE: windows_sys::core::PCWSTR = windows_sys::core::w!("fc8cb04d-311d-406c-8cb9-1ae8b843b419");
pub const ADSystemInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x50b6327f_afd1_11d2_9cb9_0000f87a369e);
pub const ADsSecurityUtility: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf270c64a_ffb8_4ae4_85fe_3a75e5347966);
pub const AccessControlEntry: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb75ac000_9bdd_11d0_852c_00c04fd8d503);
pub const AccessControlList: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb85ea052_9bdd_11d0_852c_00c04fd8d503);
pub const BackLink: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfcbf906f_4080_11d1_a3ac_00c04fb950dc);
pub const CaseIgnoreList: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x15f88a55_4680_11d1_a3b4_00c04fb950dc);
pub const DNWithBinary: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7e99c0a3_f935_11d2_ba96_00c04fb6d0d1);
pub const DNWithString: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x334857cc_f934_11d2_ba96_00c04fb6d0d1);
pub const Email: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8f92a857_478e_11d1_a3b4_00c04fb950dc);
pub const FaxNumber: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa5062215_4681_11d1_a3b4_00c04fb950dc);
pub const Hold: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb3ad3e13_4080_11d1_a3ac_00c04fb950dc);
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
pub type LPADSVALUE = *mut ADSVALUE;
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
pub type LPADS_SEARCHPREF_INFO = *mut ADS_SEARCHPREF_INFO;
pub type LPNDS_BOOLEAN = *mut u32;
pub const LargeInteger: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x927971f5_0939_11d1_8be1_00c04fd8d503);
pub const NameTranslate: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x274fae1f_3626_11d1_a3a4_00c04fb950dc);
pub const NetAddress: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb0b71247_4080_11d1_a3ac_00c04fb950dc);
pub const OctetList: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1241400f_4680_11d1_a3b4_00c04fb950dc);
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
pub type PADSVALUE = *mut ADSVALUE;
pub type PADS_ATTR_DEF = *mut ADS_ATTR_DEF;
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
pub type PADS_ATTR_INFO = *mut ADS_ATTR_INFO;
pub type PADS_BACKLINK = *mut ADS_BACKLINK;
pub type PADS_CASEIGNORE_LIST = *mut ADS_CASEIGNORE_LIST;
pub type PADS_CASE_EXACT_STRING = *mut windows_sys::core::PWSTR;
pub type PADS_CASE_IGNORE_STRING = *mut windows_sys::core::PWSTR;
pub type PADS_CLASS_DEF = *mut ADS_CLASS_DEF;
pub type PADS_DN_STRING = *mut windows_sys::core::PWSTR;
#[cfg(feature = "minwindef")]
pub type PADS_DN_WITH_BINARY = *mut ADS_DN_WITH_BINARY;
pub type PADS_DN_WITH_STRING = *mut ADS_DN_WITH_STRING;
pub type PADS_EMAIL = *mut ADS_EMAIL;
#[cfg(feature = "minwindef")]
pub type PADS_FAXNUMBER = *mut ADS_FAXNUMBER;
pub type PADS_HOLD = *mut ADS_HOLD;
pub type PADS_INTEGER = *mut u32;
pub type PADS_LARGE_INTEGER = *mut i64;
pub type PADS_NETADDRESS = *mut ADS_NETADDRESS;
#[cfg(feature = "minwindef")]
pub type PADS_NT_SECURITY_DESCRIPTOR = *mut ADS_NT_SECURITY_DESCRIPTOR;
pub type PADS_NUMERIC_STRING = *mut windows_sys::core::PWSTR;
pub type PADS_OBJECT_CLASS = *mut windows_sys::core::PWSTR;
pub type PADS_OBJECT_INFO = *mut ADS_OBJECT_INFO;
pub type PADS_OCTET_LIST = *mut ADS_OCTET_LIST;
#[cfg(feature = "minwindef")]
pub type PADS_OCTET_STRING = *mut ADS_OCTET_STRING;
pub type PADS_PATH = *mut ADS_PATH;
pub type PADS_POSTALADDRESS = *mut ADS_POSTALADDRESS;
pub type PADS_PRINTABLE_STRING = *mut windows_sys::core::PWSTR;
#[cfg(feature = "minwindef")]
pub type PADS_PROV_SPECIFIC = *mut ADS_PROV_SPECIFIC;
pub type PADS_REPLICAPOINTER = *mut ADS_REPLICAPOINTER;
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
pub type PADS_SEARCHPREF_INFO = *mut ADS_SEARCHPREF_INFO;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
pub type PADS_SEARCH_COLUMN = *mut ADS_SEARCH_COLUMN;
#[cfg(feature = "winnt")]
pub type PADS_SEARCH_HANDLE = *mut super::HANDLE;
pub type PADS_SORTKEY = *mut ADS_SORTKEY;
pub type PADS_STATUS = *mut ADS_STATUSENUM;
pub type PADS_TIMESTAMP = *mut ADS_TIMESTAMP;
pub type PADS_TYPEDNAME = *mut ADS_TYPEDNAME;
#[cfg(feature = "minwinbase")]
pub type PADS_UTC_TIME = *mut super::SYSTEMTIME;
#[cfg(feature = "minwindef")]
pub type PADS_VLV = *mut ADS_VLV;
pub const Path: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb2538919_4080_11d1_a3ac_00c04fb950dc);
pub const Pathname: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x080d0d78_f421_11d0_a36e_00c04fb950dc);
pub const PostalAddress: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0a75afcd_4680_11d1_a3b4_00c04fb950dc);
pub const PropertyEntry: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x72d3edc2_a4c4_11d0_8533_00c04fd8d503);
pub const PropertyValue: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7b9e38b0_a97c_11d0_8534_00c04fd8d503);
pub const ReplicaPointer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf5d1badf_4080_11d1_a3ac_00c04fb950dc);
pub const SecurityDescriptor: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb958f73c_9bdd_11d0_852c_00c04fd8d503);
pub const Timestamp: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb2bed2eb_4080_11d1_a3ac_00c04fb950dc);
pub const TypedName: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb33143cb_4080_11d1_a3ac_00c04fb950dc);
pub const WinNTSystemInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66182ec4_afd1_11d2_9cb9_0000f87a369e);
