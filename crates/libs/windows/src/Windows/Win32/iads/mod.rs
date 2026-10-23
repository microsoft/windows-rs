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
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ADSVALUE {
    pub dwType: ADSTYPE,
    pub Anonymous: ADSVALUE_0,
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
impl Default for ADSVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
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
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
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
pub const ADS_ATTR_APPEND: i32 = 3;
pub const ADS_ATTR_CLEAR: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_ATTR_DEF {
    pub pszAttrName: windows_core::PWSTR,
    pub dwADsType: ADSTYPE,
    pub dwMinRange: super::DWORD,
    pub dwMaxRange: super::DWORD,
    pub fMultiValued: windows_core::BOOL,
}
pub const ADS_ATTR_DELETE: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_ATTR_INFO {
    pub pszAttrName: windows_core::PWSTR,
    pub dwControlCode: super::DWORD,
    pub dwADsType: ADSTYPE,
    pub pADsValues: PADSVALUE,
    pub dwNumValues: super::DWORD,
}
pub const ADS_ATTR_UPDATE: i32 = 2;
pub type ADS_AUTHENTICATION_ENUM = i32;
pub const ADS_AUTH_RESERVED: ADS_AUTHENTICATION_ENUM = -2147483648;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_BACKLINK {
    pub RemoteID: super::DWORD,
    pub ObjectName: windows_core::PWSTR,
}
pub type ADS_BOOLEAN = u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_CASEIGNORE_LIST {
    pub Next: *mut Self,
    pub String: windows_core::PWSTR,
}
pub type ADS_CASE_EXACT_STRING = *mut u16;
pub type ADS_CASE_IGNORE_STRING = *mut u16;
pub const ADS_CHASE_REFERRALS_ALWAYS: ADS_CHASE_REFERRALS_ENUM = 96;
pub type ADS_CHASE_REFERRALS_ENUM = i32;
pub const ADS_CHASE_REFERRALS_EXTERNAL: ADS_CHASE_REFERRALS_ENUM = 64;
pub const ADS_CHASE_REFERRALS_NEVER: ADS_CHASE_REFERRALS_ENUM = 0;
pub const ADS_CHASE_REFERRALS_SUBORDINATE: ADS_CHASE_REFERRALS_ENUM = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_CLASS_DEF {
    pub pszClassName: windows_core::PWSTR,
    pub dwMandatoryAttrs: super::DWORD,
    pub ppszMandatoryAttrs: *mut windows_core::PWSTR,
    pub optionalAttrs: super::DWORD,
    pub ppszOptionalAttrs: *mut *mut windows_core::PWSTR,
    pub dwNamingAttrs: super::DWORD,
    pub ppszNamingAttrs: *mut *mut windows_core::PWSTR,
    pub dwSuperClasses: super::DWORD,
    pub ppszSuperClasses: *mut *mut windows_core::PWSTR,
    pub fIsContainer: windows_core::BOOL,
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
pub type ADS_DN_STRING = *mut u16;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_DN_WITH_BINARY {
    pub dwLength: super::DWORD,
    pub lpBinaryValue: super::LPBYTE,
    pub pszDNString: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_DN_WITH_STRING {
    pub pszStringValue: windows_core::PWSTR,
    pub pszDNString: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_EMAIL {
    pub Address: windows_core::PWSTR,
    pub Type: super::DWORD,
}
pub const ADS_ESCAPEDMODE_DEFAULT: ADS_ESCAPE_MODE_ENUM = 1;
pub const ADS_ESCAPEDMODE_OFF: ADS_ESCAPE_MODE_ENUM = 3;
pub const ADS_ESCAPEDMODE_OFF_EX: ADS_ESCAPE_MODE_ENUM = 4;
pub const ADS_ESCAPEDMODE_ON: ADS_ESCAPE_MODE_ENUM = 2;
pub type ADS_ESCAPE_MODE_ENUM = i32;
pub const ADS_EXT_INITCREDENTIALS: i32 = 1;
pub const ADS_EXT_INITIALIZE_COMPLETE: i32 = 2;
pub const ADS_EXT_MAXEXTDISPID: i32 = 16777215;
pub const ADS_EXT_MINEXTDISPID: i32 = 1;
pub const ADS_FAST_BIND: ADS_AUTHENTICATION_ENUM = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_FAXNUMBER {
    pub TelephoneNumber: windows_core::PWSTR,
    pub NumberOfBits: super::DWORD,
    pub Parameters: super::LPBYTE,
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_HOLD {
    pub ObjectName: windows_core::PWSTR,
    pub Amount: super::DWORD,
}
pub type ADS_INTEGER = u32;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type ADS_LARGE_INTEGER = super::LARGE_INTEGER;
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_NETADDRESS {
    pub AddressType: super::DWORD,
    pub AddressLength: super::DWORD,
    pub Address: *mut super::BYTE,
}
pub const ADS_NO_AUTHENTICATION: ADS_AUTHENTICATION_ENUM = 16;
pub const ADS_NO_REFERRAL_CHASING: ADS_AUTHENTICATION_ENUM = 1024;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_NT_SECURITY_DESCRIPTOR {
    pub dwLength: super::DWORD,
    pub lpValue: super::LPBYTE,
}
pub type ADS_NUMERIC_STRING = *mut u16;
pub type ADS_OBJECT_CLASS = *mut u16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_OBJECT_INFO {
    pub pszRDN: windows_core::PWSTR,
    pub pszObjectDN: windows_core::PWSTR,
    pub pszParentDN: windows_core::PWSTR,
    pub pszSchemaDN: windows_core::PWSTR,
    pub pszClassName: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_OCTET_LIST {
    pub Next: *mut Self,
    pub Length: super::DWORD,
    pub Data: *mut super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_OCTET_STRING {
    pub dwLength: super::DWORD,
    pub lpValue: super::LPBYTE,
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_PATH {
    pub Type: super::DWORD,
    pub VolumeName: windows_core::PWSTR,
    pub Path: windows_core::PWSTR,
}
pub type ADS_PATHTYPE_ENUM = i32;
pub const ADS_PATH_FILE: ADS_PATHTYPE_ENUM = 1;
pub const ADS_PATH_FILESHARE: ADS_PATHTYPE_ENUM = 2;
pub const ADS_PATH_REGISTRY: ADS_PATHTYPE_ENUM = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_POSTALADDRESS {
    pub PostalAddress: [windows_core::PWSTR; 6],
}
impl Default for ADS_POSTALADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ADS_PREFERENCES_ENUM = i32;
pub type ADS_PRINTABLE_STRING = *mut u16;
pub const ADS_PROMPT_CREDENTIALS: ADS_AUTHENTICATION_ENUM = 8;
pub const ADS_PROPERTY_APPEND: ADS_PROPERTY_OPERATION_ENUM = 3;
pub const ADS_PROPERTY_CLEAR: ADS_PROPERTY_OPERATION_ENUM = 1;
pub const ADS_PROPERTY_DELETE: ADS_PROPERTY_OPERATION_ENUM = 4;
pub type ADS_PROPERTY_OPERATION_ENUM = i32;
pub const ADS_PROPERTY_UPDATE: ADS_PROPERTY_OPERATION_ENUM = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_PROV_SPECIFIC {
    pub dwLength: super::DWORD,
    pub lpValue: super::LPBYTE,
}
pub const ADS_READONLY_SERVER: ADS_AUTHENTICATION_ENUM = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_REPLICAPOINTER {
    pub ServerName: windows_core::PWSTR,
    pub ReplicaType: super::DWORD,
    pub ReplicaNumber: super::DWORD,
    pub Count: super::DWORD,
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
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ADS_SEARCHPREF_INFO {
    pub dwSearchPref: ADS_SEARCHPREF,
    pub vValue: ADSVALUE,
    pub dwStatus: ADS_STATUS,
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
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
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_SEARCH_COLUMN {
    pub pszAttrName: windows_core::PWSTR,
    pub dwADsType: ADSTYPE,
    pub pADsValues: PADSVALUE,
    pub dwNumValues: super::DWORD,
    pub hReserved: super::HANDLE,
}
pub type ADS_SEARCH_HANDLE = *mut core::ffi::c_void;
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_SORTKEY {
    pub pszAttrType: windows_core::PWSTR,
    pub pszReserved: windows_core::PWSTR,
    pub fReverseorder: super::BOOLEAN,
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_TIMESTAMP {
    pub WholeSeconds: super::DWORD,
    pub EventID: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_TYPEDNAME {
    pub ObjectName: windows_core::PWSTR,
    pub Level: super::DWORD,
    pub Interval: super::DWORD,
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
#[cfg(all(feature = "minwinbase", feature = "shared"))]
pub type ADS_UTC_TIME = super::SYSTEMTIME;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADS_VLV {
    pub dwBeforeCount: super::DWORD,
    pub dwAfterCount: super::DWORD,
    pub dwOffset: super::DWORD,
    pub dwContentCount: super::DWORD,
    pub pszTarget: windows_core::PWSTR,
    pub dwContextIDLength: super::DWORD,
    pub lpContextID: super::LPBYTE,
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
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADs, IADs_Vtbl, 0xfd8256d0_fd15_11ce_abc4_02608c9e7553);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADs {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADs, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADs {
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Class(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Class)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_GUID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_GUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_ADsPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ADsPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Parent(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Parent)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Schema(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Schema)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetInfo(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetInfo(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInfo)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Get(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Put(&self, bstrname: &windows_core::BSTR, vprop: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Put)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(vprop)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetEx(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &windows_core::BSTR, vprop: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutEx)(windows_core::Interface::as_raw(self), lncontrolcode, core::mem::transmute_copy(bstrname), core::mem::transmute_copy(vprop)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetInfoEx(&self, vproperties: &super::VARIANT, lnreserved: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInfoEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vproperties), lnreserved) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADs_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Class: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_GUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ADsPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Schema: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInfo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Get: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Put: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Put: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetEx: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub PutEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    PutEx: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetInfoEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetInfoEx: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADs_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Class(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_GUID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_ADsPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Parent(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Schema(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetInfo(&self) -> windows_core::Result<()>;
    fn SetInfo(&self) -> windows_core::Result<()>;
    fn Get(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT>;
    fn Put(&self, bstrname: &windows_core::BSTR, vprop: &super::VARIANT) -> windows_core::Result<()>;
    fn GetEx(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT>;
    fn PutEx(&self, lncontrolcode: i32, bstrname: &windows_core::BSTR, vprop: &super::VARIANT) -> windows_core::Result<()>;
    fn GetInfoEx(&self, vproperties: &super::VARIANT, lnreserved: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADs_Vtbl {
    pub const fn new<Identity: IADs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADs_Impl::get_Name(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Class<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADs_Impl::get_Class(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_GUID<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADs_Impl::get_GUID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ADsPath<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADs_Impl::get_ADsPath(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Parent<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADs_Impl::get_Parent(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Schema<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADs_Impl::get_Schema(this) {
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
        unsafe extern "system" fn Get<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pvprop: *mut super::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Put<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, vprop: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADs_Impl::Put(this, core::mem::transmute(&bstrname), core::mem::transmute(&vprop)).into()
            }
        }
        unsafe extern "system" fn GetEx<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pvprop: *mut super::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn PutEx<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lncontrolcode: i32, bstrname: *mut core::ffi::c_void, vprop: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADs_Impl::PutEx(this, core::mem::transmute_copy(&lncontrolcode), core::mem::transmute(&bstrname), core::mem::transmute(&vprop)).into()
            }
        }
        unsafe extern "system" fn GetInfoEx<Identity: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vproperties: super::VARIANT, lnreserved: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADs_Impl::GetInfoEx(this, core::mem::transmute(&vproperties), core::mem::transmute_copy(&lnreserved)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            get_Class: get_Class::<Identity, OFFSET>,
            get_GUID: get_GUID::<Identity, OFFSET>,
            get_ADsPath: get_ADsPath::<Identity, OFFSET>,
            get_Parent: get_Parent::<Identity, OFFSET>,
            get_Schema: get_Schema::<Identity, OFFSET>,
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
        iid == &<IADs as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADs {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsADSystemInfo, IADsADSystemInfo_Vtbl, 0x5bb11929_afd1_11d2_9cb9_0000f87a369e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsADSystemInfo {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsADSystemInfo, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsADSystemInfo {
    pub unsafe fn get_UserName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UserName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_ComputerName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ComputerName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_SiteName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SiteName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_DomainShortName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DomainShortName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_DomainDNSName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DomainDNSName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_ForestDNSName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ForestDNSName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_PDCRoleOwner(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PDCRoleOwner)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_SchemaRoleOwner(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SchemaRoleOwner)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsNativeMode(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsNativeMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetTrees(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTrees)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsADSystemInfo_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ComputerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SiteName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DomainShortName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DomainDNSName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ForestDNSName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PDCRoleOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SchemaRoleOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IsNativeMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsNativeMode: usize,
    pub GetAnyDCName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDCSiteName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RefreshSchemaCache: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetTrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetTrees: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsADSystemInfo_Impl: super::IDispatch_Impl {
    fn get_UserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_ComputerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_SiteName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DomainShortName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DomainDNSName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_ForestDNSName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_PDCRoleOwner(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_SchemaRoleOwner(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_IsNativeMode(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn GetAnyDCName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDCSiteName(&self, szserver: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn RefreshSchemaCache(&self) -> windows_core::Result<()>;
    fn GetTrees(&self) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsADSystemInfo_Vtbl {
    pub const fn new<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_UserName<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::get_UserName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ComputerName<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::get_ComputerName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SiteName<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::get_SiteName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DomainShortName<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::get_DomainShortName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DomainDNSName<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::get_DomainDNSName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ForestDNSName<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::get_ForestDNSName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PDCRoleOwner<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::get_PDCRoleOwner(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SchemaRoleOwner<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::get_SchemaRoleOwner(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsNativeMode<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsADSystemInfo_Impl::get_IsNativeMode(this) {
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
        unsafe extern "system" fn GetTrees<Identity: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtrees: *mut super::VARIANT) -> windows_core::HRESULT {
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_UserName: get_UserName::<Identity, OFFSET>,
            get_ComputerName: get_ComputerName::<Identity, OFFSET>,
            get_SiteName: get_SiteName::<Identity, OFFSET>,
            get_DomainShortName: get_DomainShortName::<Identity, OFFSET>,
            get_DomainDNSName: get_DomainDNSName::<Identity, OFFSET>,
            get_ForestDNSName: get_ForestDNSName::<Identity, OFFSET>,
            get_PDCRoleOwner: get_PDCRoleOwner::<Identity, OFFSET>,
            get_SchemaRoleOwner: get_SchemaRoleOwner::<Identity, OFFSET>,
            get_IsNativeMode: get_IsNativeMode::<Identity, OFFSET>,
            GetAnyDCName: GetAnyDCName::<Identity, OFFSET>,
            GetDCSiteName: GetDCSiteName::<Identity, OFFSET>,
            RefreshSchemaCache: RefreshSchemaCache::<Identity, OFFSET>,
            GetTrees: GetTrees::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsADSystemInfo as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsADSystemInfo {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsAccessControlEntry, IADsAccessControlEntry_Vtbl, 0xb4f3a14c_9bdd_11d0_852c_00c04fd8d503);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsAccessControlEntry {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsAccessControlEntry, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsAccessControlEntry {
    pub unsafe fn get_AccessMask(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AccessMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_AccessMask(&self, lnaccessmask: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AccessMask)(windows_core::Interface::as_raw(self), lnaccessmask) }
    }
    pub unsafe fn get_AceType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_AceType(&self, lnacetype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AceType)(windows_core::Interface::as_raw(self), lnacetype) }
    }
    pub unsafe fn get_AceFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AceFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_AceFlags(&self, lnaceflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AceFlags)(windows_core::Interface::as_raw(self), lnaceflags) }
    }
    pub unsafe fn get_Flags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Flags(&self, lnflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Flags)(windows_core::Interface::as_raw(self), lnflags) }
    }
    pub unsafe fn get_ObjectType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ObjectType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ObjectType(&self, bstrobjecttype: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ObjectType)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrobjecttype)) }
    }
    pub unsafe fn get_InheritedObjectType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InheritedObjectType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_InheritedObjectType(&self, bstrinheritedobjecttype: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_InheritedObjectType)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrinheritedobjecttype)) }
    }
    pub unsafe fn get_Trustee(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Trustee)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Trustee(&self, bstrtrustee: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Trustee)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtrustee)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsAccessControlEntry_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_AccessMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_AccessMask: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_AceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_AceType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_AceFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_AceFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Flags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_ObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_InheritedObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_InheritedObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Trustee: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Trustee: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsAccessControlEntry_Impl: super::IDispatch_Impl {
    fn get_AccessMask(&self) -> windows_core::Result<i32>;
    fn put_AccessMask(&self, lnaccessmask: i32) -> windows_core::Result<()>;
    fn get_AceType(&self) -> windows_core::Result<i32>;
    fn put_AceType(&self, lnacetype: i32) -> windows_core::Result<()>;
    fn get_AceFlags(&self) -> windows_core::Result<i32>;
    fn put_AceFlags(&self, lnaceflags: i32) -> windows_core::Result<()>;
    fn get_Flags(&self) -> windows_core::Result<i32>;
    fn put_Flags(&self, lnflags: i32) -> windows_core::Result<()>;
    fn get_ObjectType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ObjectType(&self, bstrobjecttype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_InheritedObjectType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_InheritedObjectType(&self, bstrinheritedobjecttype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Trustee(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Trustee(&self, bstrtrustee: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsAccessControlEntry_Vtbl {
    pub const fn new<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AccessMask<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlEntry_Impl::get_AccessMask(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AccessMask<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnaccessmask: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlEntry_Impl::put_AccessMask(this, core::mem::transmute_copy(&lnaccessmask)).into()
            }
        }
        unsafe extern "system" fn get_AceType<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlEntry_Impl::get_AceType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AceType<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnacetype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlEntry_Impl::put_AceType(this, core::mem::transmute_copy(&lnacetype)).into()
            }
        }
        unsafe extern "system" fn get_AceFlags<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlEntry_Impl::get_AceFlags(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AceFlags<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnaceflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlEntry_Impl::put_AceFlags(this, core::mem::transmute_copy(&lnaceflags)).into()
            }
        }
        unsafe extern "system" fn get_Flags<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlEntry_Impl::get_Flags(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Flags<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlEntry_Impl::put_Flags(this, core::mem::transmute_copy(&lnflags)).into()
            }
        }
        unsafe extern "system" fn get_ObjectType<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlEntry_Impl::get_ObjectType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ObjectType<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrobjecttype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlEntry_Impl::put_ObjectType(this, core::mem::transmute(&bstrobjecttype)).into()
            }
        }
        unsafe extern "system" fn get_InheritedObjectType<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlEntry_Impl::get_InheritedObjectType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_InheritedObjectType<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinheritedobjecttype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlEntry_Impl::put_InheritedObjectType(this, core::mem::transmute(&bstrinheritedobjecttype)).into()
            }
        }
        unsafe extern "system" fn get_Trustee<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlEntry_Impl::get_Trustee(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Trustee<Identity: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtrustee: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlEntry_Impl::put_Trustee(this, core::mem::transmute(&bstrtrustee)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_AccessMask: get_AccessMask::<Identity, OFFSET>,
            put_AccessMask: put_AccessMask::<Identity, OFFSET>,
            get_AceType: get_AceType::<Identity, OFFSET>,
            put_AceType: put_AceType::<Identity, OFFSET>,
            get_AceFlags: get_AceFlags::<Identity, OFFSET>,
            put_AceFlags: put_AceFlags::<Identity, OFFSET>,
            get_Flags: get_Flags::<Identity, OFFSET>,
            put_Flags: put_Flags::<Identity, OFFSET>,
            get_ObjectType: get_ObjectType::<Identity, OFFSET>,
            put_ObjectType: put_ObjectType::<Identity, OFFSET>,
            get_InheritedObjectType: get_InheritedObjectType::<Identity, OFFSET>,
            put_InheritedObjectType: put_InheritedObjectType::<Identity, OFFSET>,
            get_Trustee: get_Trustee::<Identity, OFFSET>,
            put_Trustee: put_Trustee::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsAccessControlEntry as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsAccessControlEntry {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsAccessControlList, IADsAccessControlList_Vtbl, 0xb7ee91cc_9bdd_11d0_852c_00c04fd8d503);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsAccessControlList {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsAccessControlList, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsAccessControlList {
    pub unsafe fn get_AclRevision(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AclRevision)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_AclRevision(&self, lnaclrevision: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AclRevision)(windows_core::Interface::as_raw(self), lnaclrevision) }
    }
    pub unsafe fn get_AceCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_AceCount(&self, lnacecount: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AceCount)(windows_core::Interface::as_raw(self), lnacecount) }
    }
    pub unsafe fn AddAce<P0>(&self, paccesscontrolentry: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAce)(windows_core::Interface::as_raw(self), paccesscontrolentry.param().abi()) }
    }
    pub unsafe fn RemoveAce<P0>(&self, paccesscontrolentry: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveAce)(windows_core::Interface::as_raw(self), paccesscontrolentry.param().abi()) }
    }
    pub unsafe fn CopyAccessList(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyAccessList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsAccessControlList_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_AclRevision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_AclRevision: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_AceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_AceCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AddAce: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAce: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyAccessList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsAccessControlList_Impl: super::IDispatch_Impl {
    fn get_AclRevision(&self) -> windows_core::Result<i32>;
    fn put_AclRevision(&self, lnaclrevision: i32) -> windows_core::Result<()>;
    fn get_AceCount(&self) -> windows_core::Result<i32>;
    fn put_AceCount(&self, lnacecount: i32) -> windows_core::Result<()>;
    fn AddAce(&self, paccesscontrolentry: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn RemoveAce(&self, paccesscontrolentry: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn CopyAccessList(&self) -> windows_core::Result<super::IDispatch>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsAccessControlList_Vtbl {
    pub const fn new<Identity: IADsAccessControlList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AclRevision<Identity: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlList_Impl::get_AclRevision(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AclRevision<Identity: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnaclrevision: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlList_Impl::put_AclRevision(this, core::mem::transmute_copy(&lnaclrevision)).into()
            }
        }
        unsafe extern "system" fn get_AceCount<Identity: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlList_Impl::get_AceCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AceCount<Identity: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnacecount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAccessControlList_Impl::put_AceCount(this, core::mem::transmute_copy(&lnacecount)).into()
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
        unsafe extern "system" fn get__NewEnum<Identity: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAccessControlList_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_AclRevision: get_AclRevision::<Identity, OFFSET>,
            put_AclRevision: put_AclRevision::<Identity, OFFSET>,
            get_AceCount: get_AceCount::<Identity, OFFSET>,
            put_AceCount: put_AceCount::<Identity, OFFSET>,
            AddAce: AddAce::<Identity, OFFSET>,
            RemoveAce: RemoveAce::<Identity, OFFSET>,
            CopyAccessList: CopyAccessList::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsAccessControlList as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsAccessControlList {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsAcl, IADsAcl_Vtbl, 0x8452d3ab_0869_11d1_a377_00c04fb950dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsAcl {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsAcl, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsAcl {
    pub unsafe fn get_ProtectedAttrName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ProtectedAttrName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ProtectedAttrName(&self, bstrprotectedattrname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ProtectedAttrName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprotectedattrname)) }
    }
    pub unsafe fn get_SubjectName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SubjectName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_SubjectName(&self, bstrsubjectname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SubjectName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsubjectname)) }
    }
    pub unsafe fn get_Privileges(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Privileges)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Privileges(&self, lnprivileges: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Privileges)(windows_core::Interface::as_raw(self), lnprivileges) }
    }
    pub unsafe fn CopyAcl(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyAcl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsAcl_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_ProtectedAttrName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ProtectedAttrName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SubjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_SubjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Privileges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Privileges: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CopyAcl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsAcl_Impl: super::IDispatch_Impl {
    fn get_ProtectedAttrName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ProtectedAttrName(&self, bstrprotectedattrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_SubjectName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_SubjectName(&self, bstrsubjectname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Privileges(&self) -> windows_core::Result<i32>;
    fn put_Privileges(&self, lnprivileges: i32) -> windows_core::Result<()>;
    fn CopyAcl(&self) -> windows_core::Result<super::IDispatch>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsAcl_Vtbl {
    pub const fn new<Identity: IADsAcl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ProtectedAttrName<Identity: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAcl_Impl::get_ProtectedAttrName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ProtectedAttrName<Identity: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotectedattrname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAcl_Impl::put_ProtectedAttrName(this, core::mem::transmute(&bstrprotectedattrname)).into()
            }
        }
        unsafe extern "system" fn get_SubjectName<Identity: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAcl_Impl::get_SubjectName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SubjectName<Identity: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsubjectname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAcl_Impl::put_SubjectName(this, core::mem::transmute(&bstrsubjectname)).into()
            }
        }
        unsafe extern "system" fn get_Privileges<Identity: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsAcl_Impl::get_Privileges(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Privileges<Identity: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnprivileges: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAcl_Impl::put_Privileges(this, core::mem::transmute_copy(&lnprivileges)).into()
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ProtectedAttrName: get_ProtectedAttrName::<Identity, OFFSET>,
            put_ProtectedAttrName: put_ProtectedAttrName::<Identity, OFFSET>,
            get_SubjectName: get_SubjectName::<Identity, OFFSET>,
            put_SubjectName: put_SubjectName::<Identity, OFFSET>,
            get_Privileges: get_Privileges::<Identity, OFFSET>,
            put_Privileges: put_Privileges::<Identity, OFFSET>,
            CopyAcl: CopyAcl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsAcl as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "guiddef")]
    pub unsafe fn RelinquishInterface(&self, riid: *const super::IID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RelinquishInterface)(windows_core::Interface::as_raw(self), riid) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn RestoreInterface(&self, riid: *const super::IID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestoreInterface)(windows_core::Interface::as_raw(self), riid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IADsAggregatee_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectAsAggregatee: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisconnectAsAggregatee: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "guiddef")]
    pub RelinquishInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    RelinquishInterface: usize,
    #[cfg(feature = "guiddef")]
    pub RestoreInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    RestoreInterface: usize,
}
#[cfg(feature = "guiddef")]
pub trait IADsAggregatee_Impl: windows_core::IUnknownImpl {
    fn ConnectAsAggregatee(&self, pouterunknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DisconnectAsAggregatee(&self) -> windows_core::Result<()>;
    fn RelinquishInterface(&self, riid: *const super::IID) -> windows_core::Result<()>;
    fn RestoreInterface(&self, riid: *const super::IID) -> windows_core::Result<()>;
}
#[cfg(feature = "guiddef")]
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
        unsafe extern "system" fn RelinquishInterface<Identity: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsAggregatee_Impl::RelinquishInterface(this, core::mem::transmute_copy(&riid)).into()
            }
        }
        unsafe extern "system" fn RestoreInterface<Identity: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID) -> windows_core::HRESULT {
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
#[cfg(feature = "guiddef")]
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
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsBackLink, IADsBackLink_Vtbl, 0xfd1302bd_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsBackLink {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsBackLink, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsBackLink {
    pub unsafe fn get_RemoteID(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RemoteID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_RemoteID(&self, lnremoteid: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RemoteID)(windows_core::Interface::as_raw(self), lnremoteid) }
    }
    pub unsafe fn get_ObjectName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ObjectName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ObjectName(&self, bstrobjectname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ObjectName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrobjectname)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsBackLink_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_RemoteID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_RemoteID: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_ObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsBackLink_Impl: super::IDispatch_Impl {
    fn get_RemoteID(&self) -> windows_core::Result<i32>;
    fn put_RemoteID(&self, lnremoteid: i32) -> windows_core::Result<()>;
    fn get_ObjectName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ObjectName(&self, bstrobjectname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsBackLink_Vtbl {
    pub const fn new<Identity: IADsBackLink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_RemoteID<Identity: IADsBackLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsBackLink_Impl::get_RemoteID(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RemoteID<Identity: IADsBackLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnremoteid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsBackLink_Impl::put_RemoteID(this, core::mem::transmute_copy(&lnremoteid)).into()
            }
        }
        unsafe extern "system" fn get_ObjectName<Identity: IADsBackLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsBackLink_Impl::get_ObjectName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ObjectName<Identity: IADsBackLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrobjectname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsBackLink_Impl::put_ObjectName(this, core::mem::transmute(&bstrobjectname)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_RemoteID: get_RemoteID::<Identity, OFFSET>,
            put_RemoteID: put_RemoteID::<Identity, OFFSET>,
            get_ObjectName: get_ObjectName::<Identity, OFFSET>,
            put_ObjectName: put_ObjectName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsBackLink as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsBackLink {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsCaseIgnoreList, IADsCaseIgnoreList_Vtbl, 0x7b66b533_4680_11d1_a3b4_00c04fb950dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsCaseIgnoreList {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsCaseIgnoreList, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsCaseIgnoreList {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_CaseIgnoreList(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CaseIgnoreList)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_CaseIgnoreList(&self, vcaseignorelist: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_CaseIgnoreList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vcaseignorelist)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsCaseIgnoreList_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_CaseIgnoreList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_CaseIgnoreList: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_CaseIgnoreList: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_CaseIgnoreList: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsCaseIgnoreList_Impl: super::IDispatch_Impl {
    fn get_CaseIgnoreList(&self) -> windows_core::Result<super::VARIANT>;
    fn put_CaseIgnoreList(&self, vcaseignorelist: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsCaseIgnoreList_Vtbl {
    pub const fn new<Identity: IADsCaseIgnoreList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CaseIgnoreList<Identity: IADsCaseIgnoreList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsCaseIgnoreList_Impl::get_CaseIgnoreList(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_CaseIgnoreList<Identity: IADsCaseIgnoreList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcaseignorelist: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsCaseIgnoreList_Impl::put_CaseIgnoreList(this, core::mem::transmute(&vcaseignorelist)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_CaseIgnoreList: get_CaseIgnoreList::<Identity, OFFSET>,
            put_CaseIgnoreList: put_CaseIgnoreList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsCaseIgnoreList as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsCaseIgnoreList {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsClass, IADsClass_Vtbl, 0xc8f93dd0_4ae0_11cf_9e73_00aa004a5691);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsClass {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsClass, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsClass {
    pub unsafe fn get_PrimaryInterface(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PrimaryInterface)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CLSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_CLSID(&self, bstrclsid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_CLSID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrclsid)) }
    }
    pub unsafe fn get_OID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_OID(&self, bstroid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_OID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstroid)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Abstract(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Abstract)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Abstract(&self, fabstract: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Abstract)(windows_core::Interface::as_raw(self), fabstract) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Auxiliary(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Auxiliary)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Auxiliary(&self, fauxiliary: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Auxiliary)(windows_core::Interface::as_raw(self), fauxiliary) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_MandatoryProperties(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MandatoryProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_MandatoryProperties(&self, vmandatoryproperties: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MandatoryProperties)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vmandatoryproperties)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_OptionalProperties(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OptionalProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_OptionalProperties(&self, voptionalproperties: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_OptionalProperties)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(voptionalproperties)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_NamingProperties(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NamingProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_NamingProperties(&self, vnamingproperties: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_NamingProperties)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vnamingproperties)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_DerivedFrom(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DerivedFrom)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_DerivedFrom(&self, vderivedfrom: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DerivedFrom)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vderivedfrom)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_AuxDerivedFrom(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AuxDerivedFrom)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_AuxDerivedFrom(&self, vauxderivedfrom: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AuxDerivedFrom)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vauxderivedfrom)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_PossibleSuperiors(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PossibleSuperiors)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_PossibleSuperiors(&self, vpossiblesuperiors: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PossibleSuperiors)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vpossiblesuperiors)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Containment(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Containment)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_Containment(&self, vcontainment: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Containment)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vcontainment)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Container(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Container)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Container(&self, fcontainer: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Container)(windows_core::Interface::as_raw(self), fcontainer) }
    }
    pub unsafe fn get_HelpFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HelpFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_HelpFileName(&self, bstrhelpfilename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_HelpFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrhelpfilename)) }
    }
    pub unsafe fn get_HelpFileContext(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HelpFileContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_HelpFileContext(&self, lnhelpfilecontext: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_HelpFileContext)(windows_core::Interface::as_raw(self), lnhelpfilecontext) }
    }
    pub unsafe fn Qualifiers(&self) -> windows_core::Result<IADsCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Qualifiers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsClass_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_PrimaryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_CLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_OID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_OID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_Abstract: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Abstract: usize,
    #[cfg(feature = "wtypes")]
    pub put_Abstract: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Abstract: usize,
    #[cfg(feature = "wtypes")]
    pub get_Auxiliary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Auxiliary: usize,
    #[cfg(feature = "wtypes")]
    pub put_Auxiliary: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Auxiliary: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_MandatoryProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_MandatoryProperties: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_MandatoryProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_MandatoryProperties: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_OptionalProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_OptionalProperties: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_OptionalProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_OptionalProperties: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_NamingProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_NamingProperties: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_NamingProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_NamingProperties: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_DerivedFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_DerivedFrom: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_DerivedFrom: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_DerivedFrom: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_AuxDerivedFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_AuxDerivedFrom: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_AuxDerivedFrom: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_AuxDerivedFrom: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_PossibleSuperiors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_PossibleSuperiors: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_PossibleSuperiors: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_PossibleSuperiors: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Containment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Containment: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_Containment: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_Containment: usize,
    #[cfg(feature = "wtypes")]
    pub get_Container: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Container: usize,
    #[cfg(feature = "wtypes")]
    pub put_Container: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Container: usize,
    pub get_HelpFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_HelpFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_HelpFileContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_HelpFileContext: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Qualifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsClass_Impl: IADs_Impl {
    fn get_PrimaryInterface(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CLSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_CLSID(&self, bstrclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_OID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_OID(&self, bstroid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Abstract(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Abstract(&self, fabstract: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Auxiliary(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Auxiliary(&self, fauxiliary: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_MandatoryProperties(&self) -> windows_core::Result<super::VARIANT>;
    fn put_MandatoryProperties(&self, vmandatoryproperties: &super::VARIANT) -> windows_core::Result<()>;
    fn get_OptionalProperties(&self) -> windows_core::Result<super::VARIANT>;
    fn put_OptionalProperties(&self, voptionalproperties: &super::VARIANT) -> windows_core::Result<()>;
    fn get_NamingProperties(&self) -> windows_core::Result<super::VARIANT>;
    fn put_NamingProperties(&self, vnamingproperties: &super::VARIANT) -> windows_core::Result<()>;
    fn get_DerivedFrom(&self) -> windows_core::Result<super::VARIANT>;
    fn put_DerivedFrom(&self, vderivedfrom: &super::VARIANT) -> windows_core::Result<()>;
    fn get_AuxDerivedFrom(&self) -> windows_core::Result<super::VARIANT>;
    fn put_AuxDerivedFrom(&self, vauxderivedfrom: &super::VARIANT) -> windows_core::Result<()>;
    fn get_PossibleSuperiors(&self) -> windows_core::Result<super::VARIANT>;
    fn put_PossibleSuperiors(&self, vpossiblesuperiors: &super::VARIANT) -> windows_core::Result<()>;
    fn get_Containment(&self) -> windows_core::Result<super::VARIANT>;
    fn put_Containment(&self, vcontainment: &super::VARIANT) -> windows_core::Result<()>;
    fn get_Container(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Container(&self, fcontainer: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_HelpFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_HelpFileName(&self, bstrhelpfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_HelpFileContext(&self) -> windows_core::Result<i32>;
    fn put_HelpFileContext(&self, lnhelpfilecontext: i32) -> windows_core::Result<()>;
    fn Qualifiers(&self) -> windows_core::Result<IADsCollection>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsClass_Vtbl {
    pub const fn new<Identity: IADsClass_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_PrimaryInterface<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::get_PrimaryInterface(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CLSID<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::get_CLSID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_CLSID<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclsid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::put_CLSID(this, core::mem::transmute(&bstrclsid)).into()
            }
        }
        unsafe extern "system" fn get_OID<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::get_OID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_OID<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::put_OID(this, core::mem::transmute(&bstroid)).into()
            }
        }
        unsafe extern "system" fn get_Abstract<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::get_Abstract(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Abstract<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fabstract: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::put_Abstract(this, core::mem::transmute_copy(&fabstract)).into()
            }
        }
        unsafe extern "system" fn get_Auxiliary<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::get_Auxiliary(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Auxiliary<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fauxiliary: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::put_Auxiliary(this, core::mem::transmute_copy(&fauxiliary)).into()
            }
        }
        unsafe extern "system" fn get_MandatoryProperties<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::get_MandatoryProperties(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MandatoryProperties<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vmandatoryproperties: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::put_MandatoryProperties(this, core::mem::transmute(&vmandatoryproperties)).into()
            }
        }
        unsafe extern "system" fn get_OptionalProperties<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::get_OptionalProperties(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_OptionalProperties<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, voptionalproperties: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::put_OptionalProperties(this, core::mem::transmute(&voptionalproperties)).into()
            }
        }
        unsafe extern "system" fn get_NamingProperties<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::get_NamingProperties(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_NamingProperties<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vnamingproperties: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::put_NamingProperties(this, core::mem::transmute(&vnamingproperties)).into()
            }
        }
        unsafe extern "system" fn get_DerivedFrom<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::get_DerivedFrom(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DerivedFrom<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vderivedfrom: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::put_DerivedFrom(this, core::mem::transmute(&vderivedfrom)).into()
            }
        }
        unsafe extern "system" fn get_AuxDerivedFrom<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::get_AuxDerivedFrom(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AuxDerivedFrom<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vauxderivedfrom: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::put_AuxDerivedFrom(this, core::mem::transmute(&vauxderivedfrom)).into()
            }
        }
        unsafe extern "system" fn get_PossibleSuperiors<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::get_PossibleSuperiors(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PossibleSuperiors<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vpossiblesuperiors: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::put_PossibleSuperiors(this, core::mem::transmute(&vpossiblesuperiors)).into()
            }
        }
        unsafe extern "system" fn get_Containment<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::get_Containment(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Containment<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcontainment: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::put_Containment(this, core::mem::transmute(&vcontainment)).into()
            }
        }
        unsafe extern "system" fn get_Container<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::get_Container(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Container<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcontainer: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::put_Container(this, core::mem::transmute_copy(&fcontainer)).into()
            }
        }
        unsafe extern "system" fn get_HelpFileName<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::get_HelpFileName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_HelpFileName<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhelpfilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::put_HelpFileName(this, core::mem::transmute(&bstrhelpfilename)).into()
            }
        }
        unsafe extern "system" fn get_HelpFileContext<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsClass_Impl::get_HelpFileContext(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_HelpFileContext<Identity: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnhelpfilecontext: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsClass_Impl::put_HelpFileContext(this, core::mem::transmute_copy(&lnhelpfilecontext)).into()
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
            get_PrimaryInterface: get_PrimaryInterface::<Identity, OFFSET>,
            get_CLSID: get_CLSID::<Identity, OFFSET>,
            put_CLSID: put_CLSID::<Identity, OFFSET>,
            get_OID: get_OID::<Identity, OFFSET>,
            put_OID: put_OID::<Identity, OFFSET>,
            get_Abstract: get_Abstract::<Identity, OFFSET>,
            put_Abstract: put_Abstract::<Identity, OFFSET>,
            get_Auxiliary: get_Auxiliary::<Identity, OFFSET>,
            put_Auxiliary: put_Auxiliary::<Identity, OFFSET>,
            get_MandatoryProperties: get_MandatoryProperties::<Identity, OFFSET>,
            put_MandatoryProperties: put_MandatoryProperties::<Identity, OFFSET>,
            get_OptionalProperties: get_OptionalProperties::<Identity, OFFSET>,
            put_OptionalProperties: put_OptionalProperties::<Identity, OFFSET>,
            get_NamingProperties: get_NamingProperties::<Identity, OFFSET>,
            put_NamingProperties: put_NamingProperties::<Identity, OFFSET>,
            get_DerivedFrom: get_DerivedFrom::<Identity, OFFSET>,
            put_DerivedFrom: put_DerivedFrom::<Identity, OFFSET>,
            get_AuxDerivedFrom: get_AuxDerivedFrom::<Identity, OFFSET>,
            put_AuxDerivedFrom: put_AuxDerivedFrom::<Identity, OFFSET>,
            get_PossibleSuperiors: get_PossibleSuperiors::<Identity, OFFSET>,
            put_PossibleSuperiors: put_PossibleSuperiors::<Identity, OFFSET>,
            get_Containment: get_Containment::<Identity, OFFSET>,
            put_Containment: put_Containment::<Identity, OFFSET>,
            get_Container: get_Container::<Identity, OFFSET>,
            put_Container: put_Container::<Identity, OFFSET>,
            get_HelpFileName: get_HelpFileName::<Identity, OFFSET>,
            put_HelpFileName: put_HelpFileName::<Identity, OFFSET>,
            get_HelpFileContext: get_HelpFileContext::<Identity, OFFSET>,
            put_HelpFileContext: put_HelpFileContext::<Identity, OFFSET>,
            Qualifiers: Qualifiers::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsClass as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsClass {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsCollection, IADsCollection_Vtbl, 0x72b945e0_253b_11cf_a988_00aa006bc149);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsCollection {
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Add(&self, bstrname: &windows_core::BSTR, vitem: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(vitem)) }
    }
    pub unsafe fn Remove(&self, bstritemtoberemoved: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstritemtoberemoved)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetObjectA(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectA)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetObjectA: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetObjectA: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsCollection_Impl: super::IDispatch_Impl {
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, bstrname: &windows_core::BSTR, vitem: &super::VARIANT) -> windows_core::Result<()>;
    fn Remove(&self, bstritemtoberemoved: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetObjectA(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsCollection_Vtbl {
    pub const fn new<Identity: IADsCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get__NewEnum<Identity: IADsCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        ppenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IADsCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, vitem: super::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetObjectA<Identity: IADsCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pvitem: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsCollection_Impl::GetObjectA(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        pvitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            GetObjectA: GetObjectA::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsComputer, IADsComputer_Vtbl, 0xefe3cc70_1d9f_11cf_b1f3_02608c9e7553);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsComputer {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsComputer, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsComputer {
    pub unsafe fn get_ComputerID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ComputerID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Site(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Site)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Description)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn get_Location(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Location)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Location(&self, bstrlocation: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Location)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlocation)) }
    }
    pub unsafe fn get_PrimaryUser(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PrimaryUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_PrimaryUser(&self, bstrprimaryuser: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PrimaryUser)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprimaryuser)) }
    }
    pub unsafe fn get_Owner(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Owner)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Owner(&self, bstrowner: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Owner)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrowner)) }
    }
    pub unsafe fn get_Division(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Division)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Division(&self, bstrdivision: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Division)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdivision)) }
    }
    pub unsafe fn get_Department(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Department)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Department(&self, bstrdepartment: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Department)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdepartment)) }
    }
    pub unsafe fn get_Role(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Role)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Role(&self, bstrrole: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Role)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrole)) }
    }
    pub unsafe fn get_OperatingSystem(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OperatingSystem)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_OperatingSystem(&self, bstroperatingsystem: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_OperatingSystem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstroperatingsystem)) }
    }
    pub unsafe fn get_OperatingSystemVersion(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OperatingSystemVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_OperatingSystemVersion(&self, bstroperatingsystemversion: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_OperatingSystemVersion)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstroperatingsystemversion)) }
    }
    pub unsafe fn get_Model(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Model)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Model(&self, bstrmodel: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Model)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmodel)) }
    }
    pub unsafe fn get_Processor(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Processor)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Processor(&self, bstrprocessor: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Processor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprocessor)) }
    }
    pub unsafe fn get_ProcessorCount(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ProcessorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ProcessorCount(&self, bstrprocessorcount: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ProcessorCount)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprocessorcount)) }
    }
    pub unsafe fn get_MemorySize(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MemorySize)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_MemorySize(&self, bstrmemorysize: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MemorySize)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmemorysize)) }
    }
    pub unsafe fn get_StorageCapacity(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StorageCapacity)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_StorageCapacity(&self, bstrstoragecapacity: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StorageCapacity)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrstoragecapacity)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_NetAddresses(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NetAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_NetAddresses(&self, vnetaddresses: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_NetAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vnetaddresses)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsComputer_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_ComputerID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Site: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Location: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Location: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PrimaryUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_PrimaryUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Owner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Owner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Division: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Division: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Department: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Department: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Role: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Role: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_OperatingSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_OperatingSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_OperatingSystemVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_OperatingSystemVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Model: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Model: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Processor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Processor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ProcessorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ProcessorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_MemorySize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_MemorySize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_StorageCapacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_StorageCapacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_NetAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_NetAddresses: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_NetAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_NetAddresses: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsComputer_Impl: IADs_Impl {
    fn get_ComputerID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Site(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Location(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Location(&self, bstrlocation: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_PrimaryUser(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_PrimaryUser(&self, bstrprimaryuser: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Owner(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Owner(&self, bstrowner: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Division(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Division(&self, bstrdivision: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Department(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Department(&self, bstrdepartment: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Role(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Role(&self, bstrrole: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_OperatingSystem(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_OperatingSystem(&self, bstroperatingsystem: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_OperatingSystemVersion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_OperatingSystemVersion(&self, bstroperatingsystemversion: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Model(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Model(&self, bstrmodel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Processor(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Processor(&self, bstrprocessor: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ProcessorCount(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ProcessorCount(&self, bstrprocessorcount: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_MemorySize(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_MemorySize(&self, bstrmemorysize: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_StorageCapacity(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_StorageCapacity(&self, bstrstoragecapacity: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_NetAddresses(&self) -> windows_core::Result<super::VARIANT>;
    fn put_NetAddresses(&self, vnetaddresses: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsComputer_Vtbl {
    pub const fn new<Identity: IADsComputer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ComputerID<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::get_ComputerID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Site<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::get_Site(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Description<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::get_Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Description<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::put_Description(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn get_Location<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::get_Location(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Location<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlocation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::put_Location(this, core::mem::transmute(&bstrlocation)).into()
            }
        }
        unsafe extern "system" fn get_PrimaryUser<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::get_PrimaryUser(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PrimaryUser<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprimaryuser: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::put_PrimaryUser(this, core::mem::transmute(&bstrprimaryuser)).into()
            }
        }
        unsafe extern "system" fn get_Owner<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::get_Owner(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Owner<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrowner: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::put_Owner(this, core::mem::transmute(&bstrowner)).into()
            }
        }
        unsafe extern "system" fn get_Division<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::get_Division(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Division<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdivision: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::put_Division(this, core::mem::transmute(&bstrdivision)).into()
            }
        }
        unsafe extern "system" fn get_Department<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::get_Department(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Department<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdepartment: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::put_Department(this, core::mem::transmute(&bstrdepartment)).into()
            }
        }
        unsafe extern "system" fn get_Role<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::get_Role(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Role<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrole: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::put_Role(this, core::mem::transmute(&bstrrole)).into()
            }
        }
        unsafe extern "system" fn get_OperatingSystem<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::get_OperatingSystem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_OperatingSystem<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroperatingsystem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::put_OperatingSystem(this, core::mem::transmute(&bstroperatingsystem)).into()
            }
        }
        unsafe extern "system" fn get_OperatingSystemVersion<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::get_OperatingSystemVersion(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_OperatingSystemVersion<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroperatingsystemversion: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::put_OperatingSystemVersion(this, core::mem::transmute(&bstroperatingsystemversion)).into()
            }
        }
        unsafe extern "system" fn get_Model<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::get_Model(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Model<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmodel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::put_Model(this, core::mem::transmute(&bstrmodel)).into()
            }
        }
        unsafe extern "system" fn get_Processor<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::get_Processor(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Processor<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprocessor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::put_Processor(this, core::mem::transmute(&bstrprocessor)).into()
            }
        }
        unsafe extern "system" fn get_ProcessorCount<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::get_ProcessorCount(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ProcessorCount<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprocessorcount: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::put_ProcessorCount(this, core::mem::transmute(&bstrprocessorcount)).into()
            }
        }
        unsafe extern "system" fn get_MemorySize<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::get_MemorySize(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MemorySize<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmemorysize: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::put_MemorySize(this, core::mem::transmute(&bstrmemorysize)).into()
            }
        }
        unsafe extern "system" fn get_StorageCapacity<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::get_StorageCapacity(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StorageCapacity<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrstoragecapacity: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::put_StorageCapacity(this, core::mem::transmute(&bstrstoragecapacity)).into()
            }
        }
        unsafe extern "system" fn get_NetAddresses<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsComputer_Impl::get_NetAddresses(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_NetAddresses<Identity: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vnetaddresses: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputer_Impl::put_NetAddresses(this, core::mem::transmute(&vnetaddresses)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            get_ComputerID: get_ComputerID::<Identity, OFFSET>,
            get_Site: get_Site::<Identity, OFFSET>,
            get_Description: get_Description::<Identity, OFFSET>,
            put_Description: put_Description::<Identity, OFFSET>,
            get_Location: get_Location::<Identity, OFFSET>,
            put_Location: put_Location::<Identity, OFFSET>,
            get_PrimaryUser: get_PrimaryUser::<Identity, OFFSET>,
            put_PrimaryUser: put_PrimaryUser::<Identity, OFFSET>,
            get_Owner: get_Owner::<Identity, OFFSET>,
            put_Owner: put_Owner::<Identity, OFFSET>,
            get_Division: get_Division::<Identity, OFFSET>,
            put_Division: put_Division::<Identity, OFFSET>,
            get_Department: get_Department::<Identity, OFFSET>,
            put_Department: put_Department::<Identity, OFFSET>,
            get_Role: get_Role::<Identity, OFFSET>,
            put_Role: put_Role::<Identity, OFFSET>,
            get_OperatingSystem: get_OperatingSystem::<Identity, OFFSET>,
            put_OperatingSystem: put_OperatingSystem::<Identity, OFFSET>,
            get_OperatingSystemVersion: get_OperatingSystemVersion::<Identity, OFFSET>,
            put_OperatingSystemVersion: put_OperatingSystemVersion::<Identity, OFFSET>,
            get_Model: get_Model::<Identity, OFFSET>,
            put_Model: put_Model::<Identity, OFFSET>,
            get_Processor: get_Processor::<Identity, OFFSET>,
            put_Processor: put_Processor::<Identity, OFFSET>,
            get_ProcessorCount: get_ProcessorCount::<Identity, OFFSET>,
            put_ProcessorCount: put_ProcessorCount::<Identity, OFFSET>,
            get_MemorySize: get_MemorySize::<Identity, OFFSET>,
            put_MemorySize: put_MemorySize::<Identity, OFFSET>,
            get_StorageCapacity: get_StorageCapacity::<Identity, OFFSET>,
            put_StorageCapacity: put_StorageCapacity::<Identity, OFFSET>,
            get_NetAddresses: get_NetAddresses::<Identity, OFFSET>,
            put_NetAddresses: put_NetAddresses::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsComputer as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsComputer {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsComputerOperations, IADsComputerOperations_Vtbl, 0xef497680_1d9f_11cf_b1f3_02608c9e7553);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsComputerOperations {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsComputerOperations, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsComputerOperations {
    pub unsafe fn Status(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Shutdown(&self, breboot: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self), breboot) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsComputerOperations_Vtbl {
    pub base__: IADs_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Shutdown: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsComputerOperations_Impl: IADs_Impl {
    fn Status(&self) -> windows_core::Result<super::IDispatch>;
    fn Shutdown(&self, breboot: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn Shutdown<Identity: IADsComputerOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, breboot: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsComputerOperations_Impl::Shutdown(this, core::mem::transmute_copy(&breboot)).into()
            }
        }
        Self { base__: IADs_Vtbl::new::<Identity, OFFSET>(), Status: Status::<Identity, OFFSET>, Shutdown: Shutdown::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsComputerOperations as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsComputerOperations {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsContainer, IADsContainer_Vtbl, 0x001677d0_fd16_11ce_abc4_02608c9e7553);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsContainer {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsContainer, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsContainer {
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Filter(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Filter)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_Filter(&self, var: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Filter)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(var)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Hints(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Hints)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_Hints(&self, vhints: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Hints)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vhints)) }
    }
    pub unsafe fn GetObjectA(&self, classname: &windows_core::BSTR, relativename: &windows_core::BSTR) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectA)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(classname), core::mem::transmute_copy(relativename), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Create(&self, classname: &windows_core::BSTR, relativename: &windows_core::BSTR) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(classname), core::mem::transmute_copy(relativename), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete(&self, bstrclassname: &windows_core::BSTR, bstrrelativename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrclassname), core::mem::transmute_copy(bstrrelativename)) }
    }
    pub unsafe fn CopyHere(&self, sourcename: &windows_core::BSTR, newname: &windows_core::BSTR) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyHere)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sourcename), core::mem::transmute_copy(newname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn MoveHere(&self, sourcename: &windows_core::BSTR, newname: &windows_core::BSTR) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveHere)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sourcename), core::mem::transmute_copy(newname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsContainer_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Filter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Filter: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_Filter: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_Filter: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Hints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Hints: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_Hints: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_Hints: usize,
    pub GetObjectA: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyHere: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveHere: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsContainer_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Filter(&self) -> windows_core::Result<super::VARIANT>;
    fn put_Filter(&self, var: &super::VARIANT) -> windows_core::Result<()>;
    fn get_Hints(&self) -> windows_core::Result<super::VARIANT>;
    fn put_Hints(&self, vhints: &super::VARIANT) -> windows_core::Result<()>;
    fn GetObjectA(&self, classname: &windows_core::BSTR, relativename: &windows_core::BSTR) -> windows_core::Result<super::IDispatch>;
    fn Create(&self, classname: &windows_core::BSTR, relativename: &windows_core::BSTR) -> windows_core::Result<super::IDispatch>;
    fn Delete(&self, bstrclassname: &windows_core::BSTR, bstrrelativename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CopyHere(&self, sourcename: &windows_core::BSTR, newname: &windows_core::BSTR) -> windows_core::Result<super::IDispatch>;
    fn MoveHere(&self, sourcename: &windows_core::BSTR, newname: &windows_core::BSTR) -> windows_core::Result<super::IDispatch>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsContainer_Vtbl {
    pub const fn new<Identity: IADsContainer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsContainer_Impl::get_Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsContainer_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Filter<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsContainer_Impl::get_Filter(this) {
                    Ok(ok__) => {
                        pvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Filter<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, var: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsContainer_Impl::put_Filter(this, core::mem::transmute(&var)).into()
            }
        }
        unsafe extern "system" fn get_Hints<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvfilter: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsContainer_Impl::get_Hints(this) {
                    Ok(ok__) => {
                        pvfilter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Hints<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vhints: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsContainer_Impl::put_Hints(this, core::mem::transmute(&vhints)).into()
            }
        }
        unsafe extern "system" fn GetObjectA<Identity: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classname: *mut core::ffi::c_void, relativename: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsContainer_Impl::GetObjectA(this, core::mem::transmute(&classname), core::mem::transmute(&relativename)) {
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Count: get_Count::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_Filter: get_Filter::<Identity, OFFSET>,
            put_Filter: put_Filter::<Identity, OFFSET>,
            get_Hints: get_Hints::<Identity, OFFSET>,
            put_Hints: put_Hints::<Identity, OFFSET>,
            GetObjectA: GetObjectA::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            CopyHere: CopyHere::<Identity, OFFSET>,
            MoveHere: MoveHere::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsContainer as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsContainer {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsDNWithBinary, IADsDNWithBinary_Vtbl, 0x7e99c0a2_f935_11d2_ba96_00c04fb6d0d1);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsDNWithBinary {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsDNWithBinary, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsDNWithBinary {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_BinaryValue(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BinaryValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_BinaryValue(&self, vbinaryvalue: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BinaryValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vbinaryvalue)) }
    }
    pub unsafe fn get_DNString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DNString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_DNString(&self, bstrdnstring: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DNString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdnstring)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsDNWithBinary_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_BinaryValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_BinaryValue: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_BinaryValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_BinaryValue: usize,
    pub get_DNString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_DNString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsDNWithBinary_Impl: super::IDispatch_Impl {
    fn get_BinaryValue(&self) -> windows_core::Result<super::VARIANT>;
    fn put_BinaryValue(&self, vbinaryvalue: &super::VARIANT) -> windows_core::Result<()>;
    fn get_DNString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_DNString(&self, bstrdnstring: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsDNWithBinary_Vtbl {
    pub const fn new<Identity: IADsDNWithBinary_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_BinaryValue<Identity: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDNWithBinary_Impl::get_BinaryValue(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BinaryValue<Identity: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbinaryvalue: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDNWithBinary_Impl::put_BinaryValue(this, core::mem::transmute(&vbinaryvalue)).into()
            }
        }
        unsafe extern "system" fn get_DNString<Identity: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDNWithBinary_Impl::get_DNString(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DNString<Identity: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdnstring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDNWithBinary_Impl::put_DNString(this, core::mem::transmute(&bstrdnstring)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_BinaryValue: get_BinaryValue::<Identity, OFFSET>,
            put_BinaryValue: put_BinaryValue::<Identity, OFFSET>,
            get_DNString: get_DNString::<Identity, OFFSET>,
            put_DNString: put_DNString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsDNWithBinary as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsDNWithBinary {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsDNWithString, IADsDNWithString_Vtbl, 0x370df02e_f934_11d2_ba96_00c04fb6d0d1);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsDNWithString {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsDNWithString, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsDNWithString {
    pub unsafe fn get_StringValue(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StringValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_StringValue(&self, bstrstringvalue: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StringValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrstringvalue)) }
    }
    pub unsafe fn get_DNString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DNString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_DNString(&self, bstrdnstring: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DNString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdnstring)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsDNWithString_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_StringValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_StringValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DNString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_DNString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsDNWithString_Impl: super::IDispatch_Impl {
    fn get_StringValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_StringValue(&self, bstrstringvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_DNString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_DNString(&self, bstrdnstring: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsDNWithString_Vtbl {
    pub const fn new<Identity: IADsDNWithString_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_StringValue<Identity: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDNWithString_Impl::get_StringValue(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StringValue<Identity: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrstringvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDNWithString_Impl::put_StringValue(this, core::mem::transmute(&bstrstringvalue)).into()
            }
        }
        unsafe extern "system" fn get_DNString<Identity: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDNWithString_Impl::get_DNString(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DNString<Identity: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdnstring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDNWithString_Impl::put_DNString(this, core::mem::transmute(&bstrdnstring)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_StringValue: get_StringValue::<Identity, OFFSET>,
            put_StringValue: put_StringValue::<Identity, OFFSET>,
            get_DNString: get_DNString::<Identity, OFFSET>,
            put_DNString: put_DNString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsDNWithString as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsDNWithString {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsDeleteOps, IADsDeleteOps_Vtbl, 0xb2bd0902_8878_11d1_8c21_00c04fd8d503);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsDeleteOps {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsDeleteOps, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsDeleteOps {
    pub unsafe fn DeleteObject(&self, lnflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteObject)(windows_core::Interface::as_raw(self), lnflags) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsDeleteOps_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub DeleteObject: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsDeleteOps_Impl: super::IDispatch_Impl {
    fn DeleteObject(&self, lnflags: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsDeleteOps_Vtbl {
    pub const fn new<Identity: IADsDeleteOps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeleteObject<Identity: IADsDeleteOps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDeleteOps_Impl::DeleteObject(this, core::mem::transmute_copy(&lnflags)).into()
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), DeleteObject: DeleteObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsDeleteOps as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsDeleteOps {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsDomain, IADsDomain_Vtbl, 0x00e4c220_fd16_11ce_abc4_02608c9e7553);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsDomain {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsDomain, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsDomain {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsWorkgroup(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsWorkgroup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_MinPasswordLength(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MinPasswordLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MinPasswordLength(&self, lnminpasswordlength: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MinPasswordLength)(windows_core::Interface::as_raw(self), lnminpasswordlength) }
    }
    pub unsafe fn get_MinPasswordAge(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MinPasswordAge)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MinPasswordAge(&self, lnminpasswordage: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MinPasswordAge)(windows_core::Interface::as_raw(self), lnminpasswordage) }
    }
    pub unsafe fn get_MaxPasswordAge(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MaxPasswordAge)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MaxPasswordAge(&self, lnmaxpasswordage: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MaxPasswordAge)(windows_core::Interface::as_raw(self), lnmaxpasswordage) }
    }
    pub unsafe fn get_MaxBadPasswordsAllowed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MaxBadPasswordsAllowed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MaxBadPasswordsAllowed(&self, lnmaxbadpasswordsallowed: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MaxBadPasswordsAllowed)(windows_core::Interface::as_raw(self), lnmaxbadpasswordsallowed) }
    }
    pub unsafe fn get_PasswordHistoryLength(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PasswordHistoryLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_PasswordHistoryLength(&self, lnpasswordhistorylength: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PasswordHistoryLength)(windows_core::Interface::as_raw(self), lnpasswordhistorylength) }
    }
    pub unsafe fn get_PasswordAttributes(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PasswordAttributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_PasswordAttributes(&self, lnpasswordattributes: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PasswordAttributes)(windows_core::Interface::as_raw(self), lnpasswordattributes) }
    }
    pub unsafe fn get_AutoUnlockInterval(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AutoUnlockInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_AutoUnlockInterval(&self, lnautounlockinterval: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AutoUnlockInterval)(windows_core::Interface::as_raw(self), lnautounlockinterval) }
    }
    pub unsafe fn get_LockoutObservationInterval(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LockoutObservationInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_LockoutObservationInterval(&self, lnlockoutobservationinterval: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LockoutObservationInterval)(windows_core::Interface::as_raw(self), lnlockoutobservationinterval) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsDomain_Vtbl {
    pub base__: IADs_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_IsWorkgroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsWorkgroup: usize,
    pub get_MinPasswordLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_MinPasswordLength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_MinPasswordAge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_MinPasswordAge: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_MaxPasswordAge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_MaxPasswordAge: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_MaxBadPasswordsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_MaxBadPasswordsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_PasswordHistoryLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_PasswordHistoryLength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_PasswordAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_PasswordAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_AutoUnlockInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_AutoUnlockInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_LockoutObservationInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_LockoutObservationInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsDomain_Impl: IADs_Impl {
    fn get_IsWorkgroup(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_MinPasswordLength(&self) -> windows_core::Result<i32>;
    fn put_MinPasswordLength(&self, lnminpasswordlength: i32) -> windows_core::Result<()>;
    fn get_MinPasswordAge(&self) -> windows_core::Result<i32>;
    fn put_MinPasswordAge(&self, lnminpasswordage: i32) -> windows_core::Result<()>;
    fn get_MaxPasswordAge(&self) -> windows_core::Result<i32>;
    fn put_MaxPasswordAge(&self, lnmaxpasswordage: i32) -> windows_core::Result<()>;
    fn get_MaxBadPasswordsAllowed(&self) -> windows_core::Result<i32>;
    fn put_MaxBadPasswordsAllowed(&self, lnmaxbadpasswordsallowed: i32) -> windows_core::Result<()>;
    fn get_PasswordHistoryLength(&self) -> windows_core::Result<i32>;
    fn put_PasswordHistoryLength(&self, lnpasswordhistorylength: i32) -> windows_core::Result<()>;
    fn get_PasswordAttributes(&self) -> windows_core::Result<i32>;
    fn put_PasswordAttributes(&self, lnpasswordattributes: i32) -> windows_core::Result<()>;
    fn get_AutoUnlockInterval(&self) -> windows_core::Result<i32>;
    fn put_AutoUnlockInterval(&self, lnautounlockinterval: i32) -> windows_core::Result<()>;
    fn get_LockoutObservationInterval(&self) -> windows_core::Result<i32>;
    fn put_LockoutObservationInterval(&self, lnlockoutobservationinterval: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsDomain_Vtbl {
    pub const fn new<Identity: IADsDomain_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_IsWorkgroup<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::get_IsWorkgroup(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_MinPasswordLength<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::get_MinPasswordLength(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MinPasswordLength<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnminpasswordlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDomain_Impl::put_MinPasswordLength(this, core::mem::transmute_copy(&lnminpasswordlength)).into()
            }
        }
        unsafe extern "system" fn get_MinPasswordAge<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::get_MinPasswordAge(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MinPasswordAge<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnminpasswordage: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDomain_Impl::put_MinPasswordAge(this, core::mem::transmute_copy(&lnminpasswordage)).into()
            }
        }
        unsafe extern "system" fn get_MaxPasswordAge<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::get_MaxPasswordAge(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MaxPasswordAge<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxpasswordage: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDomain_Impl::put_MaxPasswordAge(this, core::mem::transmute_copy(&lnmaxpasswordage)).into()
            }
        }
        unsafe extern "system" fn get_MaxBadPasswordsAllowed<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::get_MaxBadPasswordsAllowed(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MaxBadPasswordsAllowed<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxbadpasswordsallowed: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDomain_Impl::put_MaxBadPasswordsAllowed(this, core::mem::transmute_copy(&lnmaxbadpasswordsallowed)).into()
            }
        }
        unsafe extern "system" fn get_PasswordHistoryLength<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::get_PasswordHistoryLength(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PasswordHistoryLength<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnpasswordhistorylength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDomain_Impl::put_PasswordHistoryLength(this, core::mem::transmute_copy(&lnpasswordhistorylength)).into()
            }
        }
        unsafe extern "system" fn get_PasswordAttributes<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::get_PasswordAttributes(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PasswordAttributes<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnpasswordattributes: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDomain_Impl::put_PasswordAttributes(this, core::mem::transmute_copy(&lnpasswordattributes)).into()
            }
        }
        unsafe extern "system" fn get_AutoUnlockInterval<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::get_AutoUnlockInterval(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AutoUnlockInterval<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnautounlockinterval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDomain_Impl::put_AutoUnlockInterval(this, core::mem::transmute_copy(&lnautounlockinterval)).into()
            }
        }
        unsafe extern "system" fn get_LockoutObservationInterval<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsDomain_Impl::get_LockoutObservationInterval(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LockoutObservationInterval<Identity: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnlockoutobservationinterval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsDomain_Impl::put_LockoutObservationInterval(this, core::mem::transmute_copy(&lnlockoutobservationinterval)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            get_IsWorkgroup: get_IsWorkgroup::<Identity, OFFSET>,
            get_MinPasswordLength: get_MinPasswordLength::<Identity, OFFSET>,
            put_MinPasswordLength: put_MinPasswordLength::<Identity, OFFSET>,
            get_MinPasswordAge: get_MinPasswordAge::<Identity, OFFSET>,
            put_MinPasswordAge: put_MinPasswordAge::<Identity, OFFSET>,
            get_MaxPasswordAge: get_MaxPasswordAge::<Identity, OFFSET>,
            put_MaxPasswordAge: put_MaxPasswordAge::<Identity, OFFSET>,
            get_MaxBadPasswordsAllowed: get_MaxBadPasswordsAllowed::<Identity, OFFSET>,
            put_MaxBadPasswordsAllowed: put_MaxBadPasswordsAllowed::<Identity, OFFSET>,
            get_PasswordHistoryLength: get_PasswordHistoryLength::<Identity, OFFSET>,
            put_PasswordHistoryLength: put_PasswordHistoryLength::<Identity, OFFSET>,
            get_PasswordAttributes: get_PasswordAttributes::<Identity, OFFSET>,
            put_PasswordAttributes: put_PasswordAttributes::<Identity, OFFSET>,
            get_AutoUnlockInterval: get_AutoUnlockInterval::<Identity, OFFSET>,
            put_AutoUnlockInterval: put_AutoUnlockInterval::<Identity, OFFSET>,
            get_LockoutObservationInterval: get_LockoutObservationInterval::<Identity, OFFSET>,
            put_LockoutObservationInterval: put_LockoutObservationInterval::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsDomain as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsDomain {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsEmail, IADsEmail_Vtbl, 0x97af011a_478e_11d1_a3b4_00c04fb950dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsEmail {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsEmail, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsEmail {
    pub unsafe fn get_Type(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Type(&self, lntype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Type)(windows_core::Interface::as_raw(self), lntype) }
    }
    pub unsafe fn get_Address(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Address)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Address(&self, bstraddress: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Address)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstraddress)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsEmail_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Type: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsEmail_Impl: super::IDispatch_Impl {
    fn get_Type(&self) -> windows_core::Result<i32>;
    fn put_Type(&self, lntype: i32) -> windows_core::Result<()>;
    fn get_Address(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Address(&self, bstraddress: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsEmail_Vtbl {
    pub const fn new<Identity: IADsEmail_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Type<Identity: IADsEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsEmail_Impl::get_Type(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Type<Identity: IADsEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lntype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsEmail_Impl::put_Type(this, core::mem::transmute_copy(&lntype)).into()
            }
        }
        unsafe extern "system" fn get_Address<Identity: IADsEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsEmail_Impl::get_Address(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Address<Identity: IADsEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstraddress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsEmail_Impl::put_Address(this, core::mem::transmute(&bstraddress)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Type: get_Type::<Identity, OFFSET>,
            put_Type: put_Type::<Identity, OFFSET>,
            get_Address: get_Address::<Identity, OFFSET>,
            put_Address: put_Address::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsEmail as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsEmail {}
windows_core::imp::define_interface!(IADsExtension, IADsExtension_Vtbl, 0x3d35553c_d2b0_11d1_b17b_0000f87593a0);
windows_core::imp::interface_hierarchy!(IADsExtension, windows_core::IUnknown);
impl IADsExtension {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Operate(&self, dwcode: super::DWORD, vardata1: &super::VARIANT, vardata2: &super::VARIANT, vardata3: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Operate)(windows_core::Interface::as_raw(self), dwcode, core::mem::transmute_copy(vardata1), core::mem::transmute_copy(vardata2), core::mem::transmute_copy(vardata3)) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn PrivateGetIDsOfNames(&self, riid: *const super::IID, rgsznames: *const *const super::OLECHAR, cnames: u32, lcid: super::LCID) -> windows_core::Result<super::DISPID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivateGetIDsOfNames)(windows_core::Interface::as_raw(self), riid, rgsznames, cnames, lcid, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn PrivateInvoke(&self, dispidmember: super::DISPID, riid: *const super::IID, lcid: super::LCID, wflags: super::WORD, pdispparams: *const super::DISPPARAMS, pvarresult: *mut super::VARIANT, pexcepinfo: *mut super::EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PrivateInvoke)(windows_core::Interface::as_raw(self), dispidmember, riid, lcid, wflags, pdispparams, pvarresult, pexcepinfo, puargerr as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IADsExtension_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Operate: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::VARIANT, super::VARIANT, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Operate: usize,
    #[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
    pub PrivateGetIDsOfNames: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *const *const super::OLECHAR, u32, super::LCID, *mut super::DISPID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared", feature = "wtypesbase")))]
    PrivateGetIDsOfNames: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub PrivateInvoke: unsafe extern "system" fn(*mut core::ffi::c_void, super::DISPID, *const super::IID, super::LCID, super::WORD, *const super::DISPPARAMS, *mut super::VARIANT, *mut super::EXCEPINFO, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    PrivateInvoke: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsExtension_Impl: windows_core::IUnknownImpl {
    fn Operate(&self, dwcode: super::DWORD, vardata1: &super::VARIANT, vardata2: &super::VARIANT, vardata3: &super::VARIANT) -> windows_core::Result<()>;
    fn PrivateGetIDsOfNames(&self, riid: *const super::IID, rgsznames: *const *const super::OLECHAR, cnames: u32, lcid: super::LCID) -> windows_core::Result<super::DISPID>;
    fn PrivateInvoke(&self, dispidmember: super::DISPID, riid: *const super::IID, lcid: super::LCID, wflags: super::WORD, pdispparams: *const super::DISPPARAMS, pvarresult: *mut super::VARIANT, pexcepinfo: *mut super::EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsExtension_Vtbl {
    pub const fn new<Identity: IADsExtension_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Operate<Identity: IADsExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcode: super::DWORD, vardata1: super::VARIANT, vardata2: super::VARIANT, vardata3: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsExtension_Impl::Operate(this, core::mem::transmute_copy(&dwcode), core::mem::transmute(&vardata1), core::mem::transmute(&vardata2), core::mem::transmute(&vardata3)).into()
            }
        }
        unsafe extern "system" fn PrivateGetIDsOfNames<Identity: IADsExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, rgsznames: *const *const super::OLECHAR, cnames: u32, lcid: super::LCID, rgdispid: *mut super::DISPID) -> windows_core::HRESULT {
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
        unsafe extern "system" fn PrivateInvoke<Identity: IADsExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispidmember: super::DISPID, riid: *const super::IID, lcid: super::LCID, wflags: super::WORD, pdispparams: *const super::DISPPARAMS, pvarresult: *mut super::VARIANT, pexcepinfo: *mut super::EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsExtension {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsFaxNumber, IADsFaxNumber_Vtbl, 0xa910dea9_4680_11d1_a3b4_00c04fb950dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsFaxNumber {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsFaxNumber, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsFaxNumber {
    pub unsafe fn get_TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TelephoneNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_TelephoneNumber(&self, bstrtelephonenumber: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_TelephoneNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtelephonenumber)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Parameters(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Parameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_Parameters(&self, vparameters: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Parameters)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vparameters)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsFaxNumber_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_TelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_TelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Parameters: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_Parameters: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsFaxNumber_Impl: super::IDispatch_Impl {
    fn get_TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_TelephoneNumber(&self, bstrtelephonenumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Parameters(&self) -> windows_core::Result<super::VARIANT>;
    fn put_Parameters(&self, vparameters: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsFaxNumber_Vtbl {
    pub const fn new<Identity: IADsFaxNumber_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_TelephoneNumber<Identity: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFaxNumber_Impl::get_TelephoneNumber(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_TelephoneNumber<Identity: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtelephonenumber: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsFaxNumber_Impl::put_TelephoneNumber(this, core::mem::transmute(&bstrtelephonenumber)).into()
            }
        }
        unsafe extern "system" fn get_Parameters<Identity: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFaxNumber_Impl::get_Parameters(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Parameters<Identity: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vparameters: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsFaxNumber_Impl::put_Parameters(this, core::mem::transmute(&vparameters)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_TelephoneNumber: get_TelephoneNumber::<Identity, OFFSET>,
            put_TelephoneNumber: put_TelephoneNumber::<Identity, OFFSET>,
            get_Parameters: get_Parameters::<Identity, OFFSET>,
            put_Parameters: put_Parameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsFaxNumber as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsFaxNumber {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsFileService, IADsFileService_Vtbl, 0xa89d1900_31ca_11cf_a98a_00aa006bc149);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsFileService {
    type Target = IADsService;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsFileService, windows_core::IUnknown, super::IDispatch, IADs, IADsService);
#[cfg(feature = "oaidl")]
impl IADsFileService {
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Description)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn get_MaxUserCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MaxUserCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MaxUserCount(&self, lnmaxusercount: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MaxUserCount)(windows_core::Interface::as_raw(self), lnmaxusercount) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsFileService_Vtbl {
    pub base__: IADsService_Vtbl,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_MaxUserCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_MaxUserCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsFileService_Impl: IADsService_Impl {
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_MaxUserCount(&self) -> windows_core::Result<i32>;
    fn put_MaxUserCount(&self, lnmaxusercount: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsFileService_Vtbl {
    pub const fn new<Identity: IADsFileService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Description<Identity: IADsFileService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFileService_Impl::get_Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Description<Identity: IADsFileService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsFileService_Impl::put_Description(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn get_MaxUserCount<Identity: IADsFileService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFileService_Impl::get_MaxUserCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MaxUserCount<Identity: IADsFileService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxusercount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsFileService_Impl::put_MaxUserCount(this, core::mem::transmute_copy(&lnmaxusercount)).into()
            }
        }
        Self {
            base__: IADsService_Vtbl::new::<Identity, OFFSET>(),
            get_Description: get_Description::<Identity, OFFSET>,
            put_Description: put_Description::<Identity, OFFSET>,
            get_MaxUserCount: get_MaxUserCount::<Identity, OFFSET>,
            put_MaxUserCount: put_MaxUserCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsFileService as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID || iid == &<IADsService as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsFileService {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsFileServiceOperations, IADsFileServiceOperations_Vtbl, 0xa02ded10_31ca_11cf_a98a_00aa006bc149);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsFileServiceOperations {
    type Target = IADsServiceOperations;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsFileServiceOperations, windows_core::IUnknown, super::IDispatch, IADs, IADsServiceOperations);
#[cfg(feature = "oaidl")]
impl IADsFileServiceOperations {
    pub unsafe fn Sessions(&self) -> windows_core::Result<IADsCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Sessions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Resources(&self) -> windows_core::Result<IADsCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Resources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsFileServiceOperations_Vtbl {
    pub base__: IADsServiceOperations_Vtbl,
    pub Sessions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsFileServiceOperations_Impl: IADsServiceOperations_Impl {
    fn Sessions(&self) -> windows_core::Result<IADsCollection>;
    fn Resources(&self) -> windows_core::Result<IADsCollection>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        iid == &<IADsFileServiceOperations as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID || iid == &<IADsServiceOperations as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsFileServiceOperations {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsFileShare, IADsFileShare_Vtbl, 0xeb6dcaf0_4b83_11cf_a995_00aa006bc149);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsFileShare {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsFileShare, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsFileShare {
    pub unsafe fn get_CurrentUserCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentUserCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Description)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn get_HostComputer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HostComputer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_HostComputer(&self, bstrhostcomputer: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_HostComputer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrhostcomputer)) }
    }
    pub unsafe fn get_Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Path(&self, bstrpath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Path)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpath)) }
    }
    pub unsafe fn get_MaxUserCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MaxUserCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MaxUserCount(&self, lnmaxusercount: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MaxUserCount)(windows_core::Interface::as_raw(self), lnmaxusercount) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsFileShare_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_CurrentUserCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_HostComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_HostComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_MaxUserCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_MaxUserCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsFileShare_Impl: IADs_Impl {
    fn get_CurrentUserCount(&self) -> windows_core::Result<i32>;
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_HostComputer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_HostComputer(&self, bstrhostcomputer: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Path(&self, bstrpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_MaxUserCount(&self) -> windows_core::Result<i32>;
    fn put_MaxUserCount(&self, lnmaxusercount: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsFileShare_Vtbl {
    pub const fn new<Identity: IADsFileShare_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentUserCount<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFileShare_Impl::get_CurrentUserCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Description<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFileShare_Impl::get_Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Description<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsFileShare_Impl::put_Description(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn get_HostComputer<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFileShare_Impl::get_HostComputer(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_HostComputer<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhostcomputer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsFileShare_Impl::put_HostComputer(this, core::mem::transmute(&bstrhostcomputer)).into()
            }
        }
        unsafe extern "system" fn get_Path<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFileShare_Impl::get_Path(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Path<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsFileShare_Impl::put_Path(this, core::mem::transmute(&bstrpath)).into()
            }
        }
        unsafe extern "system" fn get_MaxUserCount<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsFileShare_Impl::get_MaxUserCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MaxUserCount<Identity: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxusercount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsFileShare_Impl::put_MaxUserCount(this, core::mem::transmute_copy(&lnmaxusercount)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentUserCount: get_CurrentUserCount::<Identity, OFFSET>,
            get_Description: get_Description::<Identity, OFFSET>,
            put_Description: put_Description::<Identity, OFFSET>,
            get_HostComputer: get_HostComputer::<Identity, OFFSET>,
            put_HostComputer: put_HostComputer::<Identity, OFFSET>,
            get_Path: get_Path::<Identity, OFFSET>,
            put_Path: put_Path::<Identity, OFFSET>,
            get_MaxUserCount: get_MaxUserCount::<Identity, OFFSET>,
            put_MaxUserCount: put_MaxUserCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsFileShare as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsFileShare {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsGroup, IADsGroup_Vtbl, 0x27636b00_410f_11cf_b1ff_02608c9e7553);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsGroup {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsGroup, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsGroup {
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Description)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn Members(&self) -> windows_core::Result<IADsMembers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Members)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsMember(&self, bstrmember: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL> {
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
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsGroup_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Members: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub IsMember: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsMember: usize,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsGroup_Impl: IADs_Impl {
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Members(&self) -> windows_core::Result<IADsMembers>;
    fn IsMember(&self, bstrmember: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL>;
    fn Add(&self, bstrnewitem: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Remove(&self, bstritemtoberemoved: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsGroup_Vtbl {
    pub const fn new<Identity: IADsGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Description<Identity: IADsGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsGroup_Impl::get_Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Description<Identity: IADsGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsGroup_Impl::put_Description(this, core::mem::transmute(&bstrdescription)).into()
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
        unsafe extern "system" fn IsMember<Identity: IADsGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmember: *mut core::ffi::c_void, bmember: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
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
            get_Description: get_Description::<Identity, OFFSET>,
            put_Description: put_Description::<Identity, OFFSET>,
            Members: Members::<Identity, OFFSET>,
            IsMember: IsMember::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsGroup as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsGroup {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsHold, IADsHold_Vtbl, 0xb3eb3b37_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsHold {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsHold, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsHold {
    pub unsafe fn get_ObjectName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ObjectName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ObjectName(&self, bstrobjectname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ObjectName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrobjectname)) }
    }
    pub unsafe fn get_Amount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Amount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Amount(&self, lnamount: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Amount)(windows_core::Interface::as_raw(self), lnamount) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsHold_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_ObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Amount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Amount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsHold_Impl: super::IDispatch_Impl {
    fn get_ObjectName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ObjectName(&self, bstrobjectname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Amount(&self) -> windows_core::Result<i32>;
    fn put_Amount(&self, lnamount: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsHold_Vtbl {
    pub const fn new<Identity: IADsHold_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ObjectName<Identity: IADsHold_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsHold_Impl::get_ObjectName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ObjectName<Identity: IADsHold_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrobjectname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsHold_Impl::put_ObjectName(this, core::mem::transmute(&bstrobjectname)).into()
            }
        }
        unsafe extern "system" fn get_Amount<Identity: IADsHold_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsHold_Impl::get_Amount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Amount<Identity: IADsHold_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnamount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsHold_Impl::put_Amount(this, core::mem::transmute_copy(&lnamount)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ObjectName: get_ObjectName::<Identity, OFFSET>,
            put_ObjectName: put_ObjectName::<Identity, OFFSET>,
            get_Amount: get_Amount::<Identity, OFFSET>,
            put_Amount: put_Amount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsHold as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsHold {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsLargeInteger, IADsLargeInteger_Vtbl, 0x9068270b_0939_11d1_8be1_00c04fd8d503);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsLargeInteger {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsLargeInteger, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsLargeInteger {
    pub unsafe fn get_HighPart(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HighPart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_HighPart(&self, lnhighpart: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_HighPart)(windows_core::Interface::as_raw(self), lnhighpart) }
    }
    pub unsafe fn get_LowPart(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LowPart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_LowPart(&self, lnlowpart: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LowPart)(windows_core::Interface::as_raw(self), lnlowpart) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsLargeInteger_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_HighPart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_HighPart: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_LowPart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_LowPart: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsLargeInteger_Impl: super::IDispatch_Impl {
    fn get_HighPart(&self) -> windows_core::Result<i32>;
    fn put_HighPart(&self, lnhighpart: i32) -> windows_core::Result<()>;
    fn get_LowPart(&self) -> windows_core::Result<i32>;
    fn put_LowPart(&self, lnlowpart: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsLargeInteger_Vtbl {
    pub const fn new<Identity: IADsLargeInteger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_HighPart<Identity: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsLargeInteger_Impl::get_HighPart(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_HighPart<Identity: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnhighpart: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsLargeInteger_Impl::put_HighPart(this, core::mem::transmute_copy(&lnhighpart)).into()
            }
        }
        unsafe extern "system" fn get_LowPart<Identity: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsLargeInteger_Impl::get_LowPart(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LowPart<Identity: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnlowpart: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsLargeInteger_Impl::put_LowPart(this, core::mem::transmute_copy(&lnlowpart)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_HighPart: get_HighPart::<Identity, OFFSET>,
            put_HighPart: put_HighPart::<Identity, OFFSET>,
            get_LowPart: get_LowPart::<Identity, OFFSET>,
            put_LowPart: put_LowPart::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsLargeInteger as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsLargeInteger {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsLocality, IADsLocality_Vtbl, 0xa05e03a2_effe_11cf_8abc_00c04fd8d503);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsLocality {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsLocality, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsLocality {
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Description)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn get_LocalityName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LocalityName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_LocalityName(&self, bstrlocalityname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LocalityName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlocalityname)) }
    }
    pub unsafe fn get_PostalAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PostalAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_PostalAddress(&self, bstrpostaladdress: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PostalAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpostaladdress)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_SeeAlso(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SeeAlso)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_SeeAlso(&self, vseealso: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SeeAlso)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vseealso)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsLocality_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_LocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_LocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_PostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_SeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_SeeAlso: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_SeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_SeeAlso: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsLocality_Impl: IADs_Impl {
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_LocalityName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_LocalityName(&self, bstrlocalityname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_PostalAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_PostalAddress(&self, bstrpostaladdress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_SeeAlso(&self) -> windows_core::Result<super::VARIANT>;
    fn put_SeeAlso(&self, vseealso: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsLocality_Vtbl {
    pub const fn new<Identity: IADsLocality_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Description<Identity: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsLocality_Impl::get_Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Description<Identity: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsLocality_Impl::put_Description(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn get_LocalityName<Identity: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsLocality_Impl::get_LocalityName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LocalityName<Identity: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlocalityname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsLocality_Impl::put_LocalityName(this, core::mem::transmute(&bstrlocalityname)).into()
            }
        }
        unsafe extern "system" fn get_PostalAddress<Identity: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsLocality_Impl::get_PostalAddress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PostalAddress<Identity: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpostaladdress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsLocality_Impl::put_PostalAddress(this, core::mem::transmute(&bstrpostaladdress)).into()
            }
        }
        unsafe extern "system" fn get_SeeAlso<Identity: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsLocality_Impl::get_SeeAlso(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SeeAlso<Identity: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vseealso: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsLocality_Impl::put_SeeAlso(this, core::mem::transmute(&vseealso)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            get_Description: get_Description::<Identity, OFFSET>,
            put_Description: put_Description::<Identity, OFFSET>,
            get_LocalityName: get_LocalityName::<Identity, OFFSET>,
            put_LocalityName: put_LocalityName::<Identity, OFFSET>,
            get_PostalAddress: get_PostalAddress::<Identity, OFFSET>,
            put_PostalAddress: put_PostalAddress::<Identity, OFFSET>,
            get_SeeAlso: get_SeeAlso::<Identity, OFFSET>,
            put_SeeAlso: put_SeeAlso::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsLocality as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsLocality {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsMembers, IADsMembers_Vtbl, 0x451a0030_72ec_11cf_b03b_00aa006e0975);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsMembers {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsMembers, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsMembers {
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Filter(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Filter)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_Filter(&self, pvfilter: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Filter)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pvfilter)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsMembers_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Filter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Filter: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_Filter: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_Filter: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsMembers_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Filter(&self) -> windows_core::Result<super::VARIANT>;
    fn put_Filter(&self, pvfilter: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsMembers_Vtbl {
    pub const fn new<Identity: IADsMembers_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: IADsMembers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsMembers_Impl::get_Count(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: IADsMembers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsMembers_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        ppenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Filter<Identity: IADsMembers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvfilter: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsMembers_Impl::get_Filter(this) {
                    Ok(ok__) => {
                        pvfilter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Filter<Identity: IADsMembers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvfilter: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsMembers_Impl::put_Filter(this, core::mem::transmute(&pvfilter)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Count: get_Count::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_Filter: get_Filter::<Identity, OFFSET>,
            put_Filter: put_Filter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsMembers as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsMembers {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsNameTranslate, IADsNameTranslate_Vtbl, 0xb1b272a3_3625_11d1_a3a4_00c04fb950dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsNameTranslate {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsNameTranslate, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsNameTranslate {
    pub unsafe fn put_ChaseReferral(&self, lnchasereferral: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ChaseReferral)(windows_core::Interface::as_raw(self), lnchasereferral) }
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
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetEx(&self, lnformattype: i32, pvar: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEx)(windows_core::Interface::as_raw(self), lnformattype, core::mem::transmute_copy(pvar)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetEx(&self, lnformattype: i32) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEx)(windows_core::Interface::as_raw(self), lnformattype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsNameTranslate_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub put_ChaseReferral: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub SetEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    SetEx: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetEx: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsNameTranslate_Impl: super::IDispatch_Impl {
    fn put_ChaseReferral(&self, lnchasereferral: i32) -> windows_core::Result<()>;
    fn Init(&self, lnsettype: i32, bstradspath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitEx(&self, lnsettype: i32, bstradspath: &windows_core::BSTR, bstruserid: &windows_core::BSTR, bstrdomain: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Set(&self, lnsettype: i32, bstradspath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Get(&self, lnformattype: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SetEx(&self, lnformattype: i32, pvar: &super::VARIANT) -> windows_core::Result<()>;
    fn GetEx(&self, lnformattype: i32) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsNameTranslate_Vtbl {
    pub const fn new<Identity: IADsNameTranslate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_ChaseReferral<Identity: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnchasereferral: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsNameTranslate_Impl::put_ChaseReferral(this, core::mem::transmute_copy(&lnchasereferral)).into()
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
        unsafe extern "system" fn SetEx<Identity: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnformattype: i32, pvar: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsNameTranslate_Impl::SetEx(this, core::mem::transmute_copy(&lnformattype), core::mem::transmute(&pvar)).into()
            }
        }
        unsafe extern "system" fn GetEx<Identity: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnformattype: i32, pvar: *mut super::VARIANT) -> windows_core::HRESULT {
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            put_ChaseReferral: put_ChaseReferral::<Identity, OFFSET>,
            Init: Init::<Identity, OFFSET>,
            InitEx: InitEx::<Identity, OFFSET>,
            Set: Set::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            SetEx: SetEx::<Identity, OFFSET>,
            GetEx: GetEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsNameTranslate as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsNameTranslate {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsNamespaces, IADsNamespaces_Vtbl, 0x28b96ba0_b330_11cf_a9ad_00aa006bc149);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsNamespaces {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsNamespaces, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsNamespaces {
    pub unsafe fn get_DefaultContainer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DefaultContainer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_DefaultContainer(&self, bstrdefaultcontainer: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DefaultContainer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdefaultcontainer)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsNamespaces_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_DefaultContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_DefaultContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsNamespaces_Impl: IADs_Impl {
    fn get_DefaultContainer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_DefaultContainer(&self, bstrdefaultcontainer: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsNamespaces_Vtbl {
    pub const fn new<Identity: IADsNamespaces_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_DefaultContainer<Identity: IADsNamespaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsNamespaces_Impl::get_DefaultContainer(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DefaultContainer<Identity: IADsNamespaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdefaultcontainer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsNamespaces_Impl::put_DefaultContainer(this, core::mem::transmute(&bstrdefaultcontainer)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            get_DefaultContainer: get_DefaultContainer::<Identity, OFFSET>,
            put_DefaultContainer: put_DefaultContainer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsNamespaces as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsNamespaces {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsNetAddress, IADsNetAddress_Vtbl, 0xb21a50a9_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsNetAddress {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsNetAddress, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsNetAddress {
    pub unsafe fn get_AddressType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AddressType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_AddressType(&self, lnaddresstype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AddressType)(windows_core::Interface::as_raw(self), lnaddresstype) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Address(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Address)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_Address(&self, vaddress: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Address)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vaddress)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsNetAddress_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_AddressType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_AddressType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Address: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_Address: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_Address: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsNetAddress_Impl: super::IDispatch_Impl {
    fn get_AddressType(&self) -> windows_core::Result<i32>;
    fn put_AddressType(&self, lnaddresstype: i32) -> windows_core::Result<()>;
    fn get_Address(&self) -> windows_core::Result<super::VARIANT>;
    fn put_Address(&self, vaddress: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsNetAddress_Vtbl {
    pub const fn new<Identity: IADsNetAddress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AddressType<Identity: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsNetAddress_Impl::get_AddressType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AddressType<Identity: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnaddresstype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsNetAddress_Impl::put_AddressType(this, core::mem::transmute_copy(&lnaddresstype)).into()
            }
        }
        unsafe extern "system" fn get_Address<Identity: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsNetAddress_Impl::get_Address(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Address<Identity: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vaddress: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsNetAddress_Impl::put_Address(this, core::mem::transmute(&vaddress)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_AddressType: get_AddressType::<Identity, OFFSET>,
            put_AddressType: put_AddressType::<Identity, OFFSET>,
            get_Address: get_Address::<Identity, OFFSET>,
            put_Address: put_Address::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsNetAddress as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsNetAddress {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsO, IADsO_Vtbl, 0xa1cd2dc6_effe_11cf_8abc_00c04fd8d503);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsO {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsO, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsO {
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Description)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn get_LocalityName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LocalityName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_LocalityName(&self, bstrlocalityname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LocalityName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlocalityname)) }
    }
    pub unsafe fn get_PostalAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PostalAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_PostalAddress(&self, bstrpostaladdress: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PostalAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpostaladdress)) }
    }
    pub unsafe fn get_TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TelephoneNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_TelephoneNumber(&self, bstrtelephonenumber: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_TelephoneNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtelephonenumber)) }
    }
    pub unsafe fn get_FaxNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FaxNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_FaxNumber(&self, bstrfaxnumber: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FaxNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfaxnumber)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_SeeAlso(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SeeAlso)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_SeeAlso(&self, vseealso: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SeeAlso)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vseealso)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsO_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_LocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_LocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_PostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_TelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_TelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_FaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_FaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_SeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_SeeAlso: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_SeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_SeeAlso: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsO_Impl: IADs_Impl {
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_LocalityName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_LocalityName(&self, bstrlocalityname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_PostalAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_PostalAddress(&self, bstrpostaladdress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_TelephoneNumber(&self, bstrtelephonenumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_FaxNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_FaxNumber(&self, bstrfaxnumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_SeeAlso(&self) -> windows_core::Result<super::VARIANT>;
    fn put_SeeAlso(&self, vseealso: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsO_Vtbl {
    pub const fn new<Identity: IADsO_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Description<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsO_Impl::get_Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Description<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsO_Impl::put_Description(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn get_LocalityName<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsO_Impl::get_LocalityName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LocalityName<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlocalityname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsO_Impl::put_LocalityName(this, core::mem::transmute(&bstrlocalityname)).into()
            }
        }
        unsafe extern "system" fn get_PostalAddress<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsO_Impl::get_PostalAddress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PostalAddress<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpostaladdress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsO_Impl::put_PostalAddress(this, core::mem::transmute(&bstrpostaladdress)).into()
            }
        }
        unsafe extern "system" fn get_TelephoneNumber<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsO_Impl::get_TelephoneNumber(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_TelephoneNumber<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtelephonenumber: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsO_Impl::put_TelephoneNumber(this, core::mem::transmute(&bstrtelephonenumber)).into()
            }
        }
        unsafe extern "system" fn get_FaxNumber<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsO_Impl::get_FaxNumber(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FaxNumber<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfaxnumber: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsO_Impl::put_FaxNumber(this, core::mem::transmute(&bstrfaxnumber)).into()
            }
        }
        unsafe extern "system" fn get_SeeAlso<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsO_Impl::get_SeeAlso(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SeeAlso<Identity: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vseealso: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsO_Impl::put_SeeAlso(this, core::mem::transmute(&vseealso)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            get_Description: get_Description::<Identity, OFFSET>,
            put_Description: put_Description::<Identity, OFFSET>,
            get_LocalityName: get_LocalityName::<Identity, OFFSET>,
            put_LocalityName: put_LocalityName::<Identity, OFFSET>,
            get_PostalAddress: get_PostalAddress::<Identity, OFFSET>,
            put_PostalAddress: put_PostalAddress::<Identity, OFFSET>,
            get_TelephoneNumber: get_TelephoneNumber::<Identity, OFFSET>,
            put_TelephoneNumber: put_TelephoneNumber::<Identity, OFFSET>,
            get_FaxNumber: get_FaxNumber::<Identity, OFFSET>,
            put_FaxNumber: put_FaxNumber::<Identity, OFFSET>,
            get_SeeAlso: get_SeeAlso::<Identity, OFFSET>,
            put_SeeAlso: put_SeeAlso::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsO as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsO {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsOU, IADsOU_Vtbl, 0xa2f733b8_effe_11cf_8abc_00c04fd8d503);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsOU {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsOU, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsOU {
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Description)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn get_LocalityName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LocalityName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_LocalityName(&self, bstrlocalityname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LocalityName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlocalityname)) }
    }
    pub unsafe fn get_PostalAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PostalAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_PostalAddress(&self, bstrpostaladdress: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PostalAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpostaladdress)) }
    }
    pub unsafe fn get_TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TelephoneNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_TelephoneNumber(&self, bstrtelephonenumber: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_TelephoneNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtelephonenumber)) }
    }
    pub unsafe fn get_FaxNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FaxNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_FaxNumber(&self, bstrfaxnumber: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FaxNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfaxnumber)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_SeeAlso(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SeeAlso)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_SeeAlso(&self, vseealso: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SeeAlso)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vseealso)) }
    }
    pub unsafe fn get_BusinessCategory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BusinessCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_BusinessCategory(&self, bstrbusinesscategory: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BusinessCategory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbusinesscategory)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsOU_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_LocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_LocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_PostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_TelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_TelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_FaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_FaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_SeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_SeeAlso: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_SeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_SeeAlso: usize,
    pub get_BusinessCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_BusinessCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsOU_Impl: IADs_Impl {
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_LocalityName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_LocalityName(&self, bstrlocalityname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_PostalAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_PostalAddress(&self, bstrpostaladdress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_TelephoneNumber(&self, bstrtelephonenumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_FaxNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_FaxNumber(&self, bstrfaxnumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_SeeAlso(&self) -> windows_core::Result<super::VARIANT>;
    fn put_SeeAlso(&self, vseealso: &super::VARIANT) -> windows_core::Result<()>;
    fn get_BusinessCategory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_BusinessCategory(&self, bstrbusinesscategory: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsOU_Vtbl {
    pub const fn new<Identity: IADsOU_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Description<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsOU_Impl::get_Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Description<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsOU_Impl::put_Description(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn get_LocalityName<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsOU_Impl::get_LocalityName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LocalityName<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlocalityname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsOU_Impl::put_LocalityName(this, core::mem::transmute(&bstrlocalityname)).into()
            }
        }
        unsafe extern "system" fn get_PostalAddress<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsOU_Impl::get_PostalAddress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PostalAddress<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpostaladdress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsOU_Impl::put_PostalAddress(this, core::mem::transmute(&bstrpostaladdress)).into()
            }
        }
        unsafe extern "system" fn get_TelephoneNumber<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsOU_Impl::get_TelephoneNumber(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_TelephoneNumber<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtelephonenumber: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsOU_Impl::put_TelephoneNumber(this, core::mem::transmute(&bstrtelephonenumber)).into()
            }
        }
        unsafe extern "system" fn get_FaxNumber<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsOU_Impl::get_FaxNumber(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FaxNumber<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfaxnumber: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsOU_Impl::put_FaxNumber(this, core::mem::transmute(&bstrfaxnumber)).into()
            }
        }
        unsafe extern "system" fn get_SeeAlso<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsOU_Impl::get_SeeAlso(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SeeAlso<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vseealso: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsOU_Impl::put_SeeAlso(this, core::mem::transmute(&vseealso)).into()
            }
        }
        unsafe extern "system" fn get_BusinessCategory<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsOU_Impl::get_BusinessCategory(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BusinessCategory<Identity: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbusinesscategory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsOU_Impl::put_BusinessCategory(this, core::mem::transmute(&bstrbusinesscategory)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            get_Description: get_Description::<Identity, OFFSET>,
            put_Description: put_Description::<Identity, OFFSET>,
            get_LocalityName: get_LocalityName::<Identity, OFFSET>,
            put_LocalityName: put_LocalityName::<Identity, OFFSET>,
            get_PostalAddress: get_PostalAddress::<Identity, OFFSET>,
            put_PostalAddress: put_PostalAddress::<Identity, OFFSET>,
            get_TelephoneNumber: get_TelephoneNumber::<Identity, OFFSET>,
            put_TelephoneNumber: put_TelephoneNumber::<Identity, OFFSET>,
            get_FaxNumber: get_FaxNumber::<Identity, OFFSET>,
            put_FaxNumber: put_FaxNumber::<Identity, OFFSET>,
            get_SeeAlso: get_SeeAlso::<Identity, OFFSET>,
            put_SeeAlso: put_SeeAlso::<Identity, OFFSET>,
            get_BusinessCategory: get_BusinessCategory::<Identity, OFFSET>,
            put_BusinessCategory: put_BusinessCategory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsOU as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsOU {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsObjectOptions, IADsObjectOptions_Vtbl, 0x46f14fda_232b_11d1_a808_00c04fd8d5a8);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsObjectOptions {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsObjectOptions, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsObjectOptions {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetOption(&self, lnoption: i32) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOption)(windows_core::Interface::as_raw(self), lnoption, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetOption(&self, lnoption: i32, vvalue: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOption)(windows_core::Interface::as_raw(self), lnoption, core::mem::transmute_copy(vvalue)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsObjectOptions_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetOption: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetOption: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub SetOption: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    SetOption: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsObjectOptions_Impl: super::IDispatch_Impl {
    fn GetOption(&self, lnoption: i32) -> windows_core::Result<super::VARIANT>;
    fn SetOption(&self, lnoption: i32, vvalue: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsObjectOptions_Vtbl {
    pub const fn new<Identity: IADsObjectOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOption<Identity: IADsObjectOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnoption: i32, pvvalue: *mut super::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetOption<Identity: IADsObjectOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnoption: i32, vvalue: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsObjectOptions_Impl::SetOption(this, core::mem::transmute_copy(&lnoption), core::mem::transmute(&vvalue)).into()
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), GetOption: GetOption::<Identity, OFFSET>, SetOption: SetOption::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsObjectOptions as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsObjectOptions {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsOctetList, IADsOctetList_Vtbl, 0x7b28b80f_4680_11d1_a3b4_00c04fb950dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsOctetList {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsOctetList, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsOctetList {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_OctetList(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OctetList)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_OctetList(&self, voctetlist: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_OctetList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(voctetlist)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsOctetList_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_OctetList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_OctetList: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_OctetList: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_OctetList: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsOctetList_Impl: super::IDispatch_Impl {
    fn get_OctetList(&self) -> windows_core::Result<super::VARIANT>;
    fn put_OctetList(&self, voctetlist: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsOctetList_Vtbl {
    pub const fn new<Identity: IADsOctetList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_OctetList<Identity: IADsOctetList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsOctetList_Impl::get_OctetList(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_OctetList<Identity: IADsOctetList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, voctetlist: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsOctetList_Impl::put_OctetList(this, core::mem::transmute(&voctetlist)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_OctetList: get_OctetList::<Identity, OFFSET>,
            put_OctetList: put_OctetList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsOctetList as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsOctetList {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsOpenDSObject, IADsOpenDSObject_Vtbl, 0xddf2891e_0f9c_11d0_8ad4_00c04fd8d503);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsOpenDSObject {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsOpenDSObject, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsOpenDSObject {
    pub unsafe fn OpenDSObject(&self, lpszdnname: &windows_core::BSTR, lpszusername: &windows_core::BSTR, lpszpassword: &windows_core::BSTR, lnreserved: i32) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenDSObject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lpszdnname), core::mem::transmute_copy(lpszusername), core::mem::transmute_copy(lpszpassword), lnreserved, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsOpenDSObject_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub OpenDSObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsOpenDSObject_Impl: super::IDispatch_Impl {
    fn OpenDSObject(&self, lpszdnname: &windows_core::BSTR, lpszusername: &windows_core::BSTR, lpszpassword: &windows_core::BSTR, lnreserved: i32) -> windows_core::Result<super::IDispatch>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), OpenDSObject: OpenDSObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsOpenDSObject as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsOpenDSObject {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsPath, IADsPath_Vtbl, 0xb287fcd5_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsPath {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsPath, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsPath {
    pub unsafe fn get_Type(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Type(&self, lntype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Type)(windows_core::Interface::as_raw(self), lntype) }
    }
    pub unsafe fn get_VolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_VolumeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_VolumeName(&self, bstrvolumename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_VolumeName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrvolumename)) }
    }
    pub unsafe fn get_Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Path(&self, bstrpath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Path)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpath)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPath_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Type: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_VolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_VolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsPath_Impl: super::IDispatch_Impl {
    fn get_Type(&self) -> windows_core::Result<i32>;
    fn put_Type(&self, lntype: i32) -> windows_core::Result<()>;
    fn get_VolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_VolumeName(&self, bstrvolumename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Path(&self, bstrpath: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsPath_Vtbl {
    pub const fn new<Identity: IADsPath_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Type<Identity: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPath_Impl::get_Type(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Type<Identity: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lntype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPath_Impl::put_Type(this, core::mem::transmute_copy(&lntype)).into()
            }
        }
        unsafe extern "system" fn get_VolumeName<Identity: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPath_Impl::get_VolumeName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_VolumeName<Identity: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrvolumename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPath_Impl::put_VolumeName(this, core::mem::transmute(&bstrvolumename)).into()
            }
        }
        unsafe extern "system" fn get_Path<Identity: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPath_Impl::get_Path(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Path<Identity: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPath_Impl::put_Path(this, core::mem::transmute(&bstrpath)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Type: get_Type::<Identity, OFFSET>,
            put_Type: put_Type::<Identity, OFFSET>,
            get_VolumeName: get_VolumeName::<Identity, OFFSET>,
            put_VolumeName: put_VolumeName::<Identity, OFFSET>,
            get_Path: get_Path::<Identity, OFFSET>,
            put_Path: put_Path::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPath as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsPath {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsPathname, IADsPathname_Vtbl, 0xd592aed4_f420_11d0_a36e_00c04fb950dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsPathname {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsPathname, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
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
    pub unsafe fn CopyPath(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEscapedElement(&self, lnreserved: i32, bstrinstr: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEscapedElement)(windows_core::Interface::as_raw(self), lnreserved, core::mem::transmute_copy(bstrinstr), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_EscapedMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EscapedMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_EscapedMode(&self, lnescapedmode: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_EscapedMode)(windows_core::Interface::as_raw(self), lnescapedmode) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPathname_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetDisplayType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Retrieve: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNumElements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddLeafElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveLeafElement: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEscapedElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_EscapedMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_EscapedMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsPathname_Impl: super::IDispatch_Impl {
    fn Set(&self, bstradspath: &windows_core::BSTR, lnsettype: i32) -> windows_core::Result<()>;
    fn SetDisplayType(&self, lndisplaytype: i32) -> windows_core::Result<()>;
    fn Retrieve(&self, lnformattype: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetNumElements(&self) -> windows_core::Result<i32>;
    fn GetElement(&self, lnelementindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn AddLeafElement(&self, bstrleafelement: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveLeafElement(&self) -> windows_core::Result<()>;
    fn CopyPath(&self) -> windows_core::Result<super::IDispatch>;
    fn GetEscapedElement(&self, lnreserved: i32, bstrinstr: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn get_EscapedMode(&self) -> windows_core::Result<i32>;
    fn put_EscapedMode(&self, lnescapedmode: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn get_EscapedMode<Identity: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPathname_Impl::get_EscapedMode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_EscapedMode<Identity: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnescapedmode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPathname_Impl::put_EscapedMode(this, core::mem::transmute_copy(&lnescapedmode)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Set: Set::<Identity, OFFSET>,
            SetDisplayType: SetDisplayType::<Identity, OFFSET>,
            Retrieve: Retrieve::<Identity, OFFSET>,
            GetNumElements: GetNumElements::<Identity, OFFSET>,
            GetElement: GetElement::<Identity, OFFSET>,
            AddLeafElement: AddLeafElement::<Identity, OFFSET>,
            RemoveLeafElement: RemoveLeafElement::<Identity, OFFSET>,
            CopyPath: CopyPath::<Identity, OFFSET>,
            GetEscapedElement: GetEscapedElement::<Identity, OFFSET>,
            get_EscapedMode: get_EscapedMode::<Identity, OFFSET>,
            put_EscapedMode: put_EscapedMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPathname as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsPathname {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsPostalAddress, IADsPostalAddress_Vtbl, 0x7adecf29_4680_11d1_a3b4_00c04fb950dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsPostalAddress {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsPostalAddress, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsPostalAddress {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_PostalAddress(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PostalAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_PostalAddress(&self, vpostaladdress: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PostalAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vpostaladdress)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPostalAddress_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_PostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_PostalAddress: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_PostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_PostalAddress: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsPostalAddress_Impl: super::IDispatch_Impl {
    fn get_PostalAddress(&self) -> windows_core::Result<super::VARIANT>;
    fn put_PostalAddress(&self, vpostaladdress: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsPostalAddress_Vtbl {
    pub const fn new<Identity: IADsPostalAddress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_PostalAddress<Identity: IADsPostalAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPostalAddress_Impl::get_PostalAddress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PostalAddress<Identity: IADsPostalAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vpostaladdress: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPostalAddress_Impl::put_PostalAddress(this, core::mem::transmute(&vpostaladdress)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_PostalAddress: get_PostalAddress::<Identity, OFFSET>,
            put_PostalAddress: put_PostalAddress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPostalAddress as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsPostalAddress {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsPrintJob, IADsPrintJob_Vtbl, 0x32fb6780_1ed0_11cf_a988_00aa006bc149);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsPrintJob {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsPrintJob, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsPrintJob {
    pub unsafe fn get_HostPrintQueue(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HostPrintQueue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_User(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_User)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_UserPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UserPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_TimeSubmitted(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TimeSubmitted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_TotalPages(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TotalPages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Size(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Description)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn get_Priority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Priority(&self, lnpriority: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Priority)(windows_core::Interface::as_raw(self), lnpriority) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_StartTime(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_StartTime(&self, dastarttime: super::DATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StartTime)(windows_core::Interface::as_raw(self), dastarttime) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_UntilTime(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UntilTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_UntilTime(&self, dauntiltime: super::DATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UntilTime)(windows_core::Interface::as_raw(self), dauntiltime) }
    }
    pub unsafe fn get_Notify(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Notify)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Notify(&self, bstrnotify: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Notify)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnotify)) }
    }
    pub unsafe fn get_NotifyPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NotifyPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_NotifyPath(&self, bstrnotifypath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_NotifyPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnotifypath)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPrintJob_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_HostPrintQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_UserPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_TimeSubmitted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_TimeSubmitted: usize,
    pub get_TotalPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Priority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_StartTime: usize,
    #[cfg(feature = "wtypes")]
    pub put_StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_StartTime: usize,
    #[cfg(feature = "wtypes")]
    pub get_UntilTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_UntilTime: usize,
    #[cfg(feature = "wtypes")]
    pub put_UntilTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_UntilTime: usize,
    pub get_Notify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Notify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_NotifyPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_NotifyPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsPrintJob_Impl: IADs_Impl {
    fn get_HostPrintQueue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_User(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_UserPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_TimeSubmitted(&self) -> windows_core::Result<super::DATE>;
    fn get_TotalPages(&self) -> windows_core::Result<i32>;
    fn get_Size(&self) -> windows_core::Result<i32>;
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Priority(&self) -> windows_core::Result<i32>;
    fn put_Priority(&self, lnpriority: i32) -> windows_core::Result<()>;
    fn get_StartTime(&self) -> windows_core::Result<super::DATE>;
    fn put_StartTime(&self, dastarttime: super::DATE) -> windows_core::Result<()>;
    fn get_UntilTime(&self) -> windows_core::Result<super::DATE>;
    fn put_UntilTime(&self, dauntiltime: super::DATE) -> windows_core::Result<()>;
    fn get_Notify(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Notify(&self, bstrnotify: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_NotifyPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_NotifyPath(&self, bstrnotifypath: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsPrintJob_Vtbl {
    pub const fn new<Identity: IADsPrintJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_HostPrintQueue<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::get_HostPrintQueue(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_User<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::get_User(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_UserPath<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::get_UserPath(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TimeSubmitted<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::get_TimeSubmitted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TotalPages<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::get_TotalPages(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Size<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::get_Size(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Description<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::get_Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Description<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintJob_Impl::put_Description(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn get_Priority<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::get_Priority(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Priority<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnpriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintJob_Impl::put_Priority(this, core::mem::transmute_copy(&lnpriority)).into()
            }
        }
        unsafe extern "system" fn get_StartTime<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::get_StartTime(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StartTime<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dastarttime: super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintJob_Impl::put_StartTime(this, core::mem::transmute_copy(&dastarttime)).into()
            }
        }
        unsafe extern "system" fn get_UntilTime<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::get_UntilTime(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UntilTime<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dauntiltime: super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintJob_Impl::put_UntilTime(this, core::mem::transmute_copy(&dauntiltime)).into()
            }
        }
        unsafe extern "system" fn get_Notify<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::get_Notify(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Notify<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnotify: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintJob_Impl::put_Notify(this, core::mem::transmute(&bstrnotify)).into()
            }
        }
        unsafe extern "system" fn get_NotifyPath<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJob_Impl::get_NotifyPath(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_NotifyPath<Identity: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnotifypath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintJob_Impl::put_NotifyPath(this, core::mem::transmute(&bstrnotifypath)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            get_HostPrintQueue: get_HostPrintQueue::<Identity, OFFSET>,
            get_User: get_User::<Identity, OFFSET>,
            get_UserPath: get_UserPath::<Identity, OFFSET>,
            get_TimeSubmitted: get_TimeSubmitted::<Identity, OFFSET>,
            get_TotalPages: get_TotalPages::<Identity, OFFSET>,
            get_Size: get_Size::<Identity, OFFSET>,
            get_Description: get_Description::<Identity, OFFSET>,
            put_Description: put_Description::<Identity, OFFSET>,
            get_Priority: get_Priority::<Identity, OFFSET>,
            put_Priority: put_Priority::<Identity, OFFSET>,
            get_StartTime: get_StartTime::<Identity, OFFSET>,
            put_StartTime: put_StartTime::<Identity, OFFSET>,
            get_UntilTime: get_UntilTime::<Identity, OFFSET>,
            put_UntilTime: put_UntilTime::<Identity, OFFSET>,
            get_Notify: get_Notify::<Identity, OFFSET>,
            put_Notify: put_Notify::<Identity, OFFSET>,
            get_NotifyPath: get_NotifyPath::<Identity, OFFSET>,
            put_NotifyPath: put_NotifyPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPrintJob as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsPrintJob {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsPrintJobOperations, IADsPrintJobOperations_Vtbl, 0x9a52db30_1ecf_11cf_a988_00aa006bc149);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsPrintJobOperations {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsPrintJobOperations, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsPrintJobOperations {
    pub unsafe fn get_Status(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_TimeElapsed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TimeElapsed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_PagesPrinted(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PagesPrinted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Position(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Position(&self, lnposition: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Position)(windows_core::Interface::as_raw(self), lnposition) }
    }
    pub unsafe fn Pause(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Resume(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPrintJobOperations_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_TimeElapsed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_PagesPrinted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Position: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsPrintJobOperations_Impl: IADs_Impl {
    fn get_Status(&self) -> windows_core::Result<i32>;
    fn get_TimeElapsed(&self) -> windows_core::Result<i32>;
    fn get_PagesPrinted(&self) -> windows_core::Result<i32>;
    fn get_Position(&self) -> windows_core::Result<i32>;
    fn put_Position(&self, lnposition: i32) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsPrintJobOperations_Vtbl {
    pub const fn new<Identity: IADsPrintJobOperations_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Status<Identity: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJobOperations_Impl::get_Status(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TimeElapsed<Identity: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJobOperations_Impl::get_TimeElapsed(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PagesPrinted<Identity: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJobOperations_Impl::get_PagesPrinted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Position<Identity: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintJobOperations_Impl::get_Position(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Position<Identity: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnposition: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintJobOperations_Impl::put_Position(this, core::mem::transmute_copy(&lnposition)).into()
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
            get_Status: get_Status::<Identity, OFFSET>,
            get_TimeElapsed: get_TimeElapsed::<Identity, OFFSET>,
            get_PagesPrinted: get_PagesPrinted::<Identity, OFFSET>,
            get_Position: get_Position::<Identity, OFFSET>,
            put_Position: put_Position::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPrintJobOperations as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsPrintJobOperations {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsPrintQueue, IADsPrintQueue_Vtbl, 0xb15160d0_1226_11cf_a985_00aa006bc149);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsPrintQueue {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsPrintQueue, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsPrintQueue {
    pub unsafe fn get_PrinterPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PrinterPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_PrinterPath(&self, bstrprinterpath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PrinterPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprinterpath)) }
    }
    pub unsafe fn get_Model(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Model)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Model(&self, bstrmodel: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Model)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmodel)) }
    }
    pub unsafe fn get_Datatype(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Datatype)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Datatype(&self, bstrdatatype: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Datatype)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdatatype)) }
    }
    pub unsafe fn get_PrintProcessor(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PrintProcessor)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_PrintProcessor(&self, bstrprintprocessor: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PrintProcessor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprintprocessor)) }
    }
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Description)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn get_Location(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Location)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Location(&self, bstrlocation: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Location)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlocation)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_StartTime(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_StartTime(&self, dastarttime: super::DATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StartTime)(windows_core::Interface::as_raw(self), dastarttime) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_UntilTime(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UntilTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_UntilTime(&self, dauntiltime: super::DATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UntilTime)(windows_core::Interface::as_raw(self), dauntiltime) }
    }
    pub unsafe fn get_DefaultJobPriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DefaultJobPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DefaultJobPriority(&self, lndefaultjobpriority: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DefaultJobPriority)(windows_core::Interface::as_raw(self), lndefaultjobpriority) }
    }
    pub unsafe fn get_Priority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Priority(&self, lnpriority: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Priority)(windows_core::Interface::as_raw(self), lnpriority) }
    }
    pub unsafe fn get_BannerPage(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BannerPage)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_BannerPage(&self, bstrbannerpage: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BannerPage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbannerpage)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_PrintDevices(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PrintDevices)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_PrintDevices(&self, vprintdevices: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PrintDevices)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vprintdevices)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_NetAddresses(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NetAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_NetAddresses(&self, vnetaddresses: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_NetAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vnetaddresses)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPrintQueue_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_PrinterPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_PrinterPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Model: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Model: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Datatype: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Datatype: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PrintProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_PrintProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Location: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Location: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_StartTime: usize,
    #[cfg(feature = "wtypes")]
    pub put_StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_StartTime: usize,
    #[cfg(feature = "wtypes")]
    pub get_UntilTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_UntilTime: usize,
    #[cfg(feature = "wtypes")]
    pub put_UntilTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_UntilTime: usize,
    pub get_DefaultJobPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_DefaultJobPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Priority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_BannerPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_BannerPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_PrintDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_PrintDevices: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_PrintDevices: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_PrintDevices: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_NetAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_NetAddresses: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_NetAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_NetAddresses: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsPrintQueue_Impl: IADs_Impl {
    fn get_PrinterPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_PrinterPath(&self, bstrprinterpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Model(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Model(&self, bstrmodel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Datatype(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Datatype(&self, bstrdatatype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_PrintProcessor(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_PrintProcessor(&self, bstrprintprocessor: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Location(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Location(&self, bstrlocation: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_StartTime(&self) -> windows_core::Result<super::DATE>;
    fn put_StartTime(&self, dastarttime: super::DATE) -> windows_core::Result<()>;
    fn get_UntilTime(&self) -> windows_core::Result<super::DATE>;
    fn put_UntilTime(&self, dauntiltime: super::DATE) -> windows_core::Result<()>;
    fn get_DefaultJobPriority(&self) -> windows_core::Result<i32>;
    fn put_DefaultJobPriority(&self, lndefaultjobpriority: i32) -> windows_core::Result<()>;
    fn get_Priority(&self) -> windows_core::Result<i32>;
    fn put_Priority(&self, lnpriority: i32) -> windows_core::Result<()>;
    fn get_BannerPage(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_BannerPage(&self, bstrbannerpage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_PrintDevices(&self) -> windows_core::Result<super::VARIANT>;
    fn put_PrintDevices(&self, vprintdevices: &super::VARIANT) -> windows_core::Result<()>;
    fn get_NetAddresses(&self) -> windows_core::Result<super::VARIANT>;
    fn put_NetAddresses(&self, vnetaddresses: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsPrintQueue_Vtbl {
    pub const fn new<Identity: IADsPrintQueue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_PrinterPath<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::get_PrinterPath(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PrinterPath<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprinterpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::put_PrinterPath(this, core::mem::transmute(&bstrprinterpath)).into()
            }
        }
        unsafe extern "system" fn get_Model<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::get_Model(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Model<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmodel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::put_Model(this, core::mem::transmute(&bstrmodel)).into()
            }
        }
        unsafe extern "system" fn get_Datatype<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::get_Datatype(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Datatype<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdatatype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::put_Datatype(this, core::mem::transmute(&bstrdatatype)).into()
            }
        }
        unsafe extern "system" fn get_PrintProcessor<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::get_PrintProcessor(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PrintProcessor<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprintprocessor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::put_PrintProcessor(this, core::mem::transmute(&bstrprintprocessor)).into()
            }
        }
        unsafe extern "system" fn get_Description<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::get_Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Description<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::put_Description(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn get_Location<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::get_Location(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Location<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlocation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::put_Location(this, core::mem::transmute(&bstrlocation)).into()
            }
        }
        unsafe extern "system" fn get_StartTime<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::get_StartTime(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StartTime<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dastarttime: super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::put_StartTime(this, core::mem::transmute_copy(&dastarttime)).into()
            }
        }
        unsafe extern "system" fn get_UntilTime<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::get_UntilTime(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UntilTime<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dauntiltime: super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::put_UntilTime(this, core::mem::transmute_copy(&dauntiltime)).into()
            }
        }
        unsafe extern "system" fn get_DefaultJobPriority<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::get_DefaultJobPriority(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DefaultJobPriority<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lndefaultjobpriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::put_DefaultJobPriority(this, core::mem::transmute_copy(&lndefaultjobpriority)).into()
            }
        }
        unsafe extern "system" fn get_Priority<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::get_Priority(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Priority<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnpriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::put_Priority(this, core::mem::transmute_copy(&lnpriority)).into()
            }
        }
        unsafe extern "system" fn get_BannerPage<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::get_BannerPage(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BannerPage<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbannerpage: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::put_BannerPage(this, core::mem::transmute(&bstrbannerpage)).into()
            }
        }
        unsafe extern "system" fn get_PrintDevices<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::get_PrintDevices(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PrintDevices<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vprintdevices: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::put_PrintDevices(this, core::mem::transmute(&vprintdevices)).into()
            }
        }
        unsafe extern "system" fn get_NetAddresses<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueue_Impl::get_NetAddresses(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_NetAddresses<Identity: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vnetaddresses: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPrintQueue_Impl::put_NetAddresses(this, core::mem::transmute(&vnetaddresses)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            get_PrinterPath: get_PrinterPath::<Identity, OFFSET>,
            put_PrinterPath: put_PrinterPath::<Identity, OFFSET>,
            get_Model: get_Model::<Identity, OFFSET>,
            put_Model: put_Model::<Identity, OFFSET>,
            get_Datatype: get_Datatype::<Identity, OFFSET>,
            put_Datatype: put_Datatype::<Identity, OFFSET>,
            get_PrintProcessor: get_PrintProcessor::<Identity, OFFSET>,
            put_PrintProcessor: put_PrintProcessor::<Identity, OFFSET>,
            get_Description: get_Description::<Identity, OFFSET>,
            put_Description: put_Description::<Identity, OFFSET>,
            get_Location: get_Location::<Identity, OFFSET>,
            put_Location: put_Location::<Identity, OFFSET>,
            get_StartTime: get_StartTime::<Identity, OFFSET>,
            put_StartTime: put_StartTime::<Identity, OFFSET>,
            get_UntilTime: get_UntilTime::<Identity, OFFSET>,
            put_UntilTime: put_UntilTime::<Identity, OFFSET>,
            get_DefaultJobPriority: get_DefaultJobPriority::<Identity, OFFSET>,
            put_DefaultJobPriority: put_DefaultJobPriority::<Identity, OFFSET>,
            get_Priority: get_Priority::<Identity, OFFSET>,
            put_Priority: put_Priority::<Identity, OFFSET>,
            get_BannerPage: get_BannerPage::<Identity, OFFSET>,
            put_BannerPage: put_BannerPage::<Identity, OFFSET>,
            get_PrintDevices: get_PrintDevices::<Identity, OFFSET>,
            put_PrintDevices: put_PrintDevices::<Identity, OFFSET>,
            get_NetAddresses: get_NetAddresses::<Identity, OFFSET>,
            put_NetAddresses: put_NetAddresses::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPrintQueue as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsPrintQueue {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsPrintQueueOperations, IADsPrintQueueOperations_Vtbl, 0x124be5c0_156e_11cf_a986_00aa006bc149);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsPrintQueueOperations {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsPrintQueueOperations, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsPrintQueueOperations {
    pub unsafe fn get_Status(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PrintJobs(&self) -> windows_core::Result<IADsCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrintJobs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPrintQueueOperations_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PrintJobs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Purge: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsPrintQueueOperations_Impl: IADs_Impl {
    fn get_Status(&self) -> windows_core::Result<i32>;
    fn PrintJobs(&self) -> windows_core::Result<IADsCollection>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn Purge(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsPrintQueueOperations_Vtbl {
    pub const fn new<Identity: IADsPrintQueueOperations_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Status<Identity: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPrintQueueOperations_Impl::get_Status(this) {
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
            get_Status: get_Status::<Identity, OFFSET>,
            PrintJobs: PrintJobs::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            Purge: Purge::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPrintQueueOperations as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsPrintQueueOperations {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsProperty, IADsProperty_Vtbl, 0xc8f93dd3_4ae0_11cf_9e73_00aa004a5691);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsProperty {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsProperty, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsProperty {
    pub unsafe fn get_OID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_OID(&self, bstroid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_OID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstroid)) }
    }
    pub unsafe fn get_Syntax(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Syntax)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Syntax(&self, bstrsyntax: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Syntax)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsyntax)) }
    }
    pub unsafe fn get_MaxRange(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MaxRange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MaxRange(&self, lnmaxrange: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MaxRange)(windows_core::Interface::as_raw(self), lnmaxrange) }
    }
    pub unsafe fn get_MinRange(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MinRange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MinRange(&self, lnminrange: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MinRange)(windows_core::Interface::as_raw(self), lnminrange) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_MultiValued(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MultiValued)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_MultiValued(&self, fmultivalued: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MultiValued)(windows_core::Interface::as_raw(self), fmultivalued) }
    }
    pub unsafe fn Qualifiers(&self) -> windows_core::Result<IADsCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Qualifiers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsProperty_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_OID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_OID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Syntax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Syntax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_MaxRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_MaxRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_MinRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_MinRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_MultiValued: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_MultiValued: usize,
    #[cfg(feature = "wtypes")]
    pub put_MultiValued: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_MultiValued: usize,
    pub Qualifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsProperty_Impl: IADs_Impl {
    fn get_OID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_OID(&self, bstroid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Syntax(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Syntax(&self, bstrsyntax: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_MaxRange(&self) -> windows_core::Result<i32>;
    fn put_MaxRange(&self, lnmaxrange: i32) -> windows_core::Result<()>;
    fn get_MinRange(&self) -> windows_core::Result<i32>;
    fn put_MinRange(&self, lnminrange: i32) -> windows_core::Result<()>;
    fn get_MultiValued(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_MultiValued(&self, fmultivalued: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Qualifiers(&self) -> windows_core::Result<IADsCollection>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsProperty_Vtbl {
    pub const fn new<Identity: IADsProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_OID<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsProperty_Impl::get_OID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_OID<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsProperty_Impl::put_OID(this, core::mem::transmute(&bstroid)).into()
            }
        }
        unsafe extern "system" fn get_Syntax<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsProperty_Impl::get_Syntax(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Syntax<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsyntax: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsProperty_Impl::put_Syntax(this, core::mem::transmute(&bstrsyntax)).into()
            }
        }
        unsafe extern "system" fn get_MaxRange<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsProperty_Impl::get_MaxRange(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MaxRange<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxrange: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsProperty_Impl::put_MaxRange(this, core::mem::transmute_copy(&lnmaxrange)).into()
            }
        }
        unsafe extern "system" fn get_MinRange<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsProperty_Impl::get_MinRange(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MinRange<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnminrange: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsProperty_Impl::put_MinRange(this, core::mem::transmute_copy(&lnminrange)).into()
            }
        }
        unsafe extern "system" fn get_MultiValued<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsProperty_Impl::get_MultiValued(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MultiValued<Identity: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmultivalued: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsProperty_Impl::put_MultiValued(this, core::mem::transmute_copy(&fmultivalued)).into()
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
            get_OID: get_OID::<Identity, OFFSET>,
            put_OID: put_OID::<Identity, OFFSET>,
            get_Syntax: get_Syntax::<Identity, OFFSET>,
            put_Syntax: put_Syntax::<Identity, OFFSET>,
            get_MaxRange: get_MaxRange::<Identity, OFFSET>,
            put_MaxRange: put_MaxRange::<Identity, OFFSET>,
            get_MinRange: get_MinRange::<Identity, OFFSET>,
            put_MinRange: put_MinRange::<Identity, OFFSET>,
            get_MultiValued: get_MultiValued::<Identity, OFFSET>,
            put_MultiValued: put_MultiValued::<Identity, OFFSET>,
            Qualifiers: Qualifiers::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsProperty as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsProperty {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsPropertyEntry, IADsPropertyEntry_Vtbl, 0x05792c8e_941f_11d0_8529_00c04fd8d503);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsPropertyEntry {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsPropertyEntry, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsPropertyEntry {
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Name(&self, bstrname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Name)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname)) }
    }
    pub unsafe fn get_ADsType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ADsType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ADsType(&self, lnadstype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ADsType)(windows_core::Interface::as_raw(self), lnadstype) }
    }
    pub unsafe fn get_ControlCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ControlCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ControlCode(&self, lncontrolcode: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ControlCode)(windows_core::Interface::as_raw(self), lncontrolcode) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Values(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Values)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_Values(&self, vvalues: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Values)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vvalues)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPropertyEntry_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ADsType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_ADsType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_ControlCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_ControlCode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Values: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Values: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_Values: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_Values: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsPropertyEntry_Impl: super::IDispatch_Impl {
    fn Clear(&self) -> windows_core::Result<()>;
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Name(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ADsType(&self) -> windows_core::Result<i32>;
    fn put_ADsType(&self, lnadstype: i32) -> windows_core::Result<()>;
    fn get_ControlCode(&self) -> windows_core::Result<i32>;
    fn put_ControlCode(&self, lncontrolcode: i32) -> windows_core::Result<()>;
    fn get_Values(&self) -> windows_core::Result<super::VARIANT>;
    fn put_Values(&self, vvalues: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsPropertyEntry_Vtbl {
    pub const fn new<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clear<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyEntry_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn get_Name<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyEntry_Impl::get_Name(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Name<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyEntry_Impl::put_Name(this, core::mem::transmute(&bstrname)).into()
            }
        }
        unsafe extern "system" fn get_ADsType<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyEntry_Impl::get_ADsType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ADsType<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnadstype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyEntry_Impl::put_ADsType(this, core::mem::transmute_copy(&lnadstype)).into()
            }
        }
        unsafe extern "system" fn get_ControlCode<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyEntry_Impl::get_ControlCode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ControlCode<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lncontrolcode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyEntry_Impl::put_ControlCode(this, core::mem::transmute_copy(&lncontrolcode)).into()
            }
        }
        unsafe extern "system" fn get_Values<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyEntry_Impl::get_Values(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Values<Identity: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vvalues: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyEntry_Impl::put_Values(this, core::mem::transmute(&vvalues)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Clear: Clear::<Identity, OFFSET>,
            get_Name: get_Name::<Identity, OFFSET>,
            put_Name: put_Name::<Identity, OFFSET>,
            get_ADsType: get_ADsType::<Identity, OFFSET>,
            put_ADsType: put_ADsType::<Identity, OFFSET>,
            get_ControlCode: get_ControlCode::<Identity, OFFSET>,
            put_ControlCode: put_ControlCode::<Identity, OFFSET>,
            get_Values: get_Values::<Identity, OFFSET>,
            put_Values: put_Values::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPropertyEntry as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsPropertyEntry {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsPropertyList, IADsPropertyList_Vtbl, 0xc6f602b6_8f69_11d0_8528_00c04fd8d503);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsPropertyList {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsPropertyList, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsPropertyList {
    pub unsafe fn get_PropertyCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PropertyCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Next(&self) -> windows_core::Result<super::VARIANT> {
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
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Item(&self, varindex: &super::VARIANT) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetPropertyItem(&self, bstrname: &windows_core::BSTR, lnadstype: super::LONG) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), lnadstype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn PutPropertyItem(&self, vardata: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutPropertyItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vardata)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ResetPropertyItem(&self, varentry: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetPropertyItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varentry)) }
    }
    pub unsafe fn PurgePropertyList(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PurgePropertyList)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPropertyList_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_PropertyCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Item: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetPropertyItem: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub PutPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    PutPropertyItem: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub ResetPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    ResetPropertyItem: usize,
    pub PurgePropertyList: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsPropertyList_Impl: super::IDispatch_Impl {
    fn get_PropertyCount(&self) -> windows_core::Result<i32>;
    fn Next(&self) -> windows_core::Result<super::VARIANT>;
    fn Skip(&self, celements: i32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Item(&self, varindex: &super::VARIANT) -> windows_core::Result<super::VARIANT>;
    fn GetPropertyItem(&self, bstrname: &windows_core::BSTR, lnadstype: super::LONG) -> windows_core::Result<super::VARIANT>;
    fn PutPropertyItem(&self, vardata: &super::VARIANT) -> windows_core::Result<()>;
    fn ResetPropertyItem(&self, varentry: &super::VARIANT) -> windows_core::Result<()>;
    fn PurgePropertyList(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsPropertyList_Vtbl {
    pub const fn new<Identity: IADsPropertyList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_PropertyCount<Identity: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyList_Impl::get_PropertyCount(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut super::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Item<Identity: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::VARIANT, pvariant: *mut super::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetPropertyItem<Identity: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, lnadstype: super::LONG, pvariant: *mut super::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn PutPropertyItem<Identity: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vardata: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyList_Impl::PutPropertyItem(this, core::mem::transmute(&vardata)).into()
            }
        }
        unsafe extern "system" fn ResetPropertyItem<Identity: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varentry: super::VARIANT) -> windows_core::HRESULT {
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_PropertyCount: get_PropertyCount::<Identity, OFFSET>,
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
        iid == &<IADsPropertyList as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsPropertyList {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsPropertyValue, IADsPropertyValue_Vtbl, 0x79fa9ad0_a97c_11d0_8534_00c04fd8d503);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsPropertyValue {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsPropertyValue, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsPropertyValue {
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn get_ADsType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ADsType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ADsType(&self, lnadstype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ADsType)(windows_core::Interface::as_raw(self), lnadstype) }
    }
    pub unsafe fn get_DNString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DNString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_DNString(&self, bstrdnstring: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DNString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdnstring)) }
    }
    pub unsafe fn get_CaseExactString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CaseExactString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_CaseExactString(&self, bstrcaseexactstring: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_CaseExactString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcaseexactstring)) }
    }
    pub unsafe fn get_CaseIgnoreString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CaseIgnoreString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_CaseIgnoreString(&self, bstrcaseignorestring: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_CaseIgnoreString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcaseignorestring)) }
    }
    pub unsafe fn get_PrintableString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PrintableString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_PrintableString(&self, bstrprintablestring: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PrintableString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprintablestring)) }
    }
    pub unsafe fn get_NumericString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NumericString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_NumericString(&self, bstrnumericstring: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_NumericString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnumericstring)) }
    }
    pub unsafe fn get_Boolean(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Boolean)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Boolean(&self, lnboolean: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Boolean)(windows_core::Interface::as_raw(self), lnboolean) }
    }
    pub unsafe fn get_Integer(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Integer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Integer(&self, lninteger: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Integer)(windows_core::Interface::as_raw(self), lninteger) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_OctetString(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OctetString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_OctetString(&self, voctetstring: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_OctetString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(voctetstring)) }
    }
    pub unsafe fn get_SecurityDescriptor(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SecurityDescriptor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_SecurityDescriptor<P0>(&self, psecuritydescriptor: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_SecurityDescriptor)(windows_core::Interface::as_raw(self), psecuritydescriptor.param().abi()) }
    }
    pub unsafe fn get_LargeInteger(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LargeInteger)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_LargeInteger<P0>(&self, plargeinteger: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_LargeInteger)(windows_core::Interface::as_raw(self), plargeinteger.param().abi()) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_UTCTime(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UTCTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_UTCTime(&self, dautctime: super::DATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UTCTime)(windows_core::Interface::as_raw(self), dautctime) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPropertyValue_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ADsType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_ADsType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_DNString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_DNString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CaseExactString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_CaseExactString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CaseIgnoreString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_CaseIgnoreString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PrintableString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_PrintableString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_NumericString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_NumericString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Boolean: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Boolean: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Integer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Integer: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_OctetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_OctetString: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_OctetString: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_OctetString: usize,
    pub get_SecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_SecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_LargeInteger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_LargeInteger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_UTCTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_UTCTime: usize,
    #[cfg(feature = "wtypes")]
    pub put_UTCTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_UTCTime: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsPropertyValue_Impl: super::IDispatch_Impl {
    fn Clear(&self) -> windows_core::Result<()>;
    fn get_ADsType(&self) -> windows_core::Result<i32>;
    fn put_ADsType(&self, lnadstype: i32) -> windows_core::Result<()>;
    fn get_DNString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_DNString(&self, bstrdnstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_CaseExactString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_CaseExactString(&self, bstrcaseexactstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_CaseIgnoreString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_CaseIgnoreString(&self, bstrcaseignorestring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_PrintableString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_PrintableString(&self, bstrprintablestring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_NumericString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_NumericString(&self, bstrnumericstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Boolean(&self) -> windows_core::Result<i32>;
    fn put_Boolean(&self, lnboolean: i32) -> windows_core::Result<()>;
    fn get_Integer(&self) -> windows_core::Result<i32>;
    fn put_Integer(&self, lninteger: i32) -> windows_core::Result<()>;
    fn get_OctetString(&self) -> windows_core::Result<super::VARIANT>;
    fn put_OctetString(&self, voctetstring: &super::VARIANT) -> windows_core::Result<()>;
    fn get_SecurityDescriptor(&self) -> windows_core::Result<super::IDispatch>;
    fn put_SecurityDescriptor(&self, psecuritydescriptor: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn get_LargeInteger(&self) -> windows_core::Result<super::IDispatch>;
    fn put_LargeInteger(&self, plargeinteger: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn get_UTCTime(&self) -> windows_core::Result<super::DATE>;
    fn put_UTCTime(&self, dautctime: super::DATE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsPropertyValue_Vtbl {
    pub const fn new<Identity: IADsPropertyValue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clear<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn get_ADsType<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::get_ADsType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ADsType<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnadstype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::put_ADsType(this, core::mem::transmute_copy(&lnadstype)).into()
            }
        }
        unsafe extern "system" fn get_DNString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::get_DNString(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DNString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdnstring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::put_DNString(this, core::mem::transmute(&bstrdnstring)).into()
            }
        }
        unsafe extern "system" fn get_CaseExactString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::get_CaseExactString(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_CaseExactString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcaseexactstring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::put_CaseExactString(this, core::mem::transmute(&bstrcaseexactstring)).into()
            }
        }
        unsafe extern "system" fn get_CaseIgnoreString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::get_CaseIgnoreString(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_CaseIgnoreString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcaseignorestring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::put_CaseIgnoreString(this, core::mem::transmute(&bstrcaseignorestring)).into()
            }
        }
        unsafe extern "system" fn get_PrintableString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::get_PrintableString(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PrintableString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprintablestring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::put_PrintableString(this, core::mem::transmute(&bstrprintablestring)).into()
            }
        }
        unsafe extern "system" fn get_NumericString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::get_NumericString(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_NumericString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnumericstring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::put_NumericString(this, core::mem::transmute(&bstrnumericstring)).into()
            }
        }
        unsafe extern "system" fn get_Boolean<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::get_Boolean(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Boolean<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnboolean: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::put_Boolean(this, core::mem::transmute_copy(&lnboolean)).into()
            }
        }
        unsafe extern "system" fn get_Integer<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::get_Integer(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Integer<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lninteger: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::put_Integer(this, core::mem::transmute_copy(&lninteger)).into()
            }
        }
        unsafe extern "system" fn get_OctetString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::get_OctetString(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_OctetString<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, voctetstring: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::put_OctetString(this, core::mem::transmute(&voctetstring)).into()
            }
        }
        unsafe extern "system" fn get_SecurityDescriptor<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::get_SecurityDescriptor(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SecurityDescriptor<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psecuritydescriptor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::put_SecurityDescriptor(this, core::mem::transmute_copy(&psecuritydescriptor)).into()
            }
        }
        unsafe extern "system" fn get_LargeInteger<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::get_LargeInteger(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LargeInteger<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plargeinteger: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::put_LargeInteger(this, core::mem::transmute_copy(&plargeinteger)).into()
            }
        }
        unsafe extern "system" fn get_UTCTime<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsPropertyValue_Impl::get_UTCTime(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UTCTime<Identity: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dautctime: super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue_Impl::put_UTCTime(this, core::mem::transmute_copy(&dautctime)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Clear: Clear::<Identity, OFFSET>,
            get_ADsType: get_ADsType::<Identity, OFFSET>,
            put_ADsType: put_ADsType::<Identity, OFFSET>,
            get_DNString: get_DNString::<Identity, OFFSET>,
            put_DNString: put_DNString::<Identity, OFFSET>,
            get_CaseExactString: get_CaseExactString::<Identity, OFFSET>,
            put_CaseExactString: put_CaseExactString::<Identity, OFFSET>,
            get_CaseIgnoreString: get_CaseIgnoreString::<Identity, OFFSET>,
            put_CaseIgnoreString: put_CaseIgnoreString::<Identity, OFFSET>,
            get_PrintableString: get_PrintableString::<Identity, OFFSET>,
            put_PrintableString: put_PrintableString::<Identity, OFFSET>,
            get_NumericString: get_NumericString::<Identity, OFFSET>,
            put_NumericString: put_NumericString::<Identity, OFFSET>,
            get_Boolean: get_Boolean::<Identity, OFFSET>,
            put_Boolean: put_Boolean::<Identity, OFFSET>,
            get_Integer: get_Integer::<Identity, OFFSET>,
            put_Integer: put_Integer::<Identity, OFFSET>,
            get_OctetString: get_OctetString::<Identity, OFFSET>,
            put_OctetString: put_OctetString::<Identity, OFFSET>,
            get_SecurityDescriptor: get_SecurityDescriptor::<Identity, OFFSET>,
            put_SecurityDescriptor: put_SecurityDescriptor::<Identity, OFFSET>,
            get_LargeInteger: get_LargeInteger::<Identity, OFFSET>,
            put_LargeInteger: put_LargeInteger::<Identity, OFFSET>,
            get_UTCTime: get_UTCTime::<Identity, OFFSET>,
            put_UTCTime: put_UTCTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPropertyValue as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsPropertyValue {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsPropertyValue2, IADsPropertyValue2_Vtbl, 0x306e831c_5bc7_11d1_a3b8_00c04fb950dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsPropertyValue2 {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsPropertyValue2, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsPropertyValue2 {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetObjectProperty(&self, lnadstype: *mut i32) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectProperty)(windows_core::Interface::as_raw(self), lnadstype as _, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn PutObjectProperty(&self, lnadstype: i32, vprop: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutObjectProperty)(windows_core::Interface::as_raw(self), lnadstype, core::mem::transmute_copy(vprop)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPropertyValue2_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetObjectProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetObjectProperty: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub PutObjectProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    PutObjectProperty: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsPropertyValue2_Impl: super::IDispatch_Impl {
    fn GetObjectProperty(&self, lnadstype: *mut i32) -> windows_core::Result<super::VARIANT>;
    fn PutObjectProperty(&self, lnadstype: i32, vprop: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsPropertyValue2_Vtbl {
    pub const fn new<Identity: IADsPropertyValue2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetObjectProperty<Identity: IADsPropertyValue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnadstype: *mut i32, pvprop: *mut super::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn PutObjectProperty<Identity: IADsPropertyValue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnadstype: i32, vprop: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsPropertyValue2_Impl::PutObjectProperty(this, core::mem::transmute_copy(&lnadstype), core::mem::transmute(&vprop)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetObjectProperty: GetObjectProperty::<Identity, OFFSET>,
            PutObjectProperty: PutObjectProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPropertyValue2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsPropertyValue2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsReplicaPointer, IADsReplicaPointer_Vtbl, 0xf60fb803_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsReplicaPointer {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsReplicaPointer, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsReplicaPointer {
    pub unsafe fn get_ServerName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ServerName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ServerName(&self, bstrservername: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ServerName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservername)) }
    }
    pub unsafe fn get_ReplicaType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ReplicaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ReplicaType(&self, lnreplicatype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ReplicaType)(windows_core::Interface::as_raw(self), lnreplicatype) }
    }
    pub unsafe fn get_ReplicaNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ReplicaNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ReplicaNumber(&self, lnreplicanumber: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ReplicaNumber)(windows_core::Interface::as_raw(self), lnreplicanumber) }
    }
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Count(&self, lncount: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Count)(windows_core::Interface::as_raw(self), lncount) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_ReplicaAddressHints(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ReplicaAddressHints)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_ReplicaAddressHints(&self, vreplicaaddresshints: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ReplicaAddressHints)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vreplicaaddresshints)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsReplicaPointer_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_ServerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ServerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ReplicaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_ReplicaType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_ReplicaNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_ReplicaNumber: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Count: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_ReplicaAddressHints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_ReplicaAddressHints: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_ReplicaAddressHints: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_ReplicaAddressHints: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsReplicaPointer_Impl: super::IDispatch_Impl {
    fn get_ServerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ServerName(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ReplicaType(&self) -> windows_core::Result<i32>;
    fn put_ReplicaType(&self, lnreplicatype: i32) -> windows_core::Result<()>;
    fn get_ReplicaNumber(&self) -> windows_core::Result<i32>;
    fn put_ReplicaNumber(&self, lnreplicanumber: i32) -> windows_core::Result<()>;
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn put_Count(&self, lncount: i32) -> windows_core::Result<()>;
    fn get_ReplicaAddressHints(&self) -> windows_core::Result<super::VARIANT>;
    fn put_ReplicaAddressHints(&self, vreplicaaddresshints: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsReplicaPointer_Vtbl {
    pub const fn new<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ServerName<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsReplicaPointer_Impl::get_ServerName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ServerName<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsReplicaPointer_Impl::put_ServerName(this, core::mem::transmute(&bstrservername)).into()
            }
        }
        unsafe extern "system" fn get_ReplicaType<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsReplicaPointer_Impl::get_ReplicaType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ReplicaType<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnreplicatype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsReplicaPointer_Impl::put_ReplicaType(this, core::mem::transmute_copy(&lnreplicatype)).into()
            }
        }
        unsafe extern "system" fn get_ReplicaNumber<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsReplicaPointer_Impl::get_ReplicaNumber(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ReplicaNumber<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnreplicanumber: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsReplicaPointer_Impl::put_ReplicaNumber(this, core::mem::transmute_copy(&lnreplicanumber)).into()
            }
        }
        unsafe extern "system" fn get_Count<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsReplicaPointer_Impl::get_Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Count<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lncount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsReplicaPointer_Impl::put_Count(this, core::mem::transmute_copy(&lncount)).into()
            }
        }
        unsafe extern "system" fn get_ReplicaAddressHints<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsReplicaPointer_Impl::get_ReplicaAddressHints(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ReplicaAddressHints<Identity: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vreplicaaddresshints: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsReplicaPointer_Impl::put_ReplicaAddressHints(this, core::mem::transmute(&vreplicaaddresshints)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ServerName: get_ServerName::<Identity, OFFSET>,
            put_ServerName: put_ServerName::<Identity, OFFSET>,
            get_ReplicaType: get_ReplicaType::<Identity, OFFSET>,
            put_ReplicaType: put_ReplicaType::<Identity, OFFSET>,
            get_ReplicaNumber: get_ReplicaNumber::<Identity, OFFSET>,
            put_ReplicaNumber: put_ReplicaNumber::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
            put_Count: put_Count::<Identity, OFFSET>,
            get_ReplicaAddressHints: get_ReplicaAddressHints::<Identity, OFFSET>,
            put_ReplicaAddressHints: put_ReplicaAddressHints::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsReplicaPointer as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsReplicaPointer {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsResource, IADsResource_Vtbl, 0x34a05b20_4aab_11cf_ae2c_00aa006ebfb9);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsResource {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsResource, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsResource {
    pub unsafe fn get_User(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_User)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_UserPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UserPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_LockCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LockCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsResource_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_UserPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_LockCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsResource_Impl: IADs_Impl {
    fn get_User(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_UserPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_LockCount(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsResource_Vtbl {
    pub const fn new<Identity: IADsResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_User<Identity: IADsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsResource_Impl::get_User(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_UserPath<Identity: IADsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsResource_Impl::get_UserPath(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Path<Identity: IADsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsResource_Impl::get_Path(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LockCount<Identity: IADsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsResource_Impl::get_LockCount(this) {
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
            get_User: get_User::<Identity, OFFSET>,
            get_UserPath: get_UserPath::<Identity, OFFSET>,
            get_Path: get_Path::<Identity, OFFSET>,
            get_LockCount: get_LockCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsResource as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsResource {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsSecurityDescriptor, IADsSecurityDescriptor_Vtbl, 0xb8c787ca_9bdd_11d0_852c_00c04fd8d503);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsSecurityDescriptor {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsSecurityDescriptor, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsSecurityDescriptor {
    pub unsafe fn get_Revision(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Revision)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Revision(&self, lnrevision: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Revision)(windows_core::Interface::as_raw(self), lnrevision) }
    }
    pub unsafe fn get_Control(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Control)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Control(&self, lncontrol: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Control)(windows_core::Interface::as_raw(self), lncontrol) }
    }
    pub unsafe fn get_Owner(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Owner)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Owner(&self, bstrowner: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Owner)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrowner)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_OwnerDefaulted(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OwnerDefaulted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_OwnerDefaulted(&self, fownerdefaulted: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_OwnerDefaulted)(windows_core::Interface::as_raw(self), fownerdefaulted) }
    }
    pub unsafe fn get_Group(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Group)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Group(&self, bstrgroup: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Group)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgroup)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_GroupDefaulted(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_GroupDefaulted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_GroupDefaulted(&self, fgroupdefaulted: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_GroupDefaulted)(windows_core::Interface::as_raw(self), fgroupdefaulted) }
    }
    pub unsafe fn get_DiscretionaryAcl(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DiscretionaryAcl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_DiscretionaryAcl<P0>(&self, pdiscretionaryacl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_DiscretionaryAcl)(windows_core::Interface::as_raw(self), pdiscretionaryacl.param().abi()) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_DaclDefaulted(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DaclDefaulted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_DaclDefaulted(&self, fdacldefaulted: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DaclDefaulted)(windows_core::Interface::as_raw(self), fdacldefaulted) }
    }
    pub unsafe fn get_SystemAcl(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SystemAcl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_SystemAcl<P0>(&self, psystemacl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_SystemAcl)(windows_core::Interface::as_raw(self), psystemacl.param().abi()) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_SaclDefaulted(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SaclDefaulted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_SaclDefaulted(&self, fsacldefaulted: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SaclDefaulted)(windows_core::Interface::as_raw(self), fsacldefaulted) }
    }
    pub unsafe fn CopySecurityDescriptor(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopySecurityDescriptor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsSecurityDescriptor_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Revision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Revision: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Control: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Control: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Owner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Owner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_OwnerDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_OwnerDefaulted: usize,
    #[cfg(feature = "wtypes")]
    pub put_OwnerDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_OwnerDefaulted: usize,
    pub get_Group: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Group: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_GroupDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_GroupDefaulted: usize,
    #[cfg(feature = "wtypes")]
    pub put_GroupDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_GroupDefaulted: usize,
    pub get_DiscretionaryAcl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_DiscretionaryAcl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_DaclDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_DaclDefaulted: usize,
    #[cfg(feature = "wtypes")]
    pub put_DaclDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_DaclDefaulted: usize,
    pub get_SystemAcl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_SystemAcl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_SaclDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_SaclDefaulted: usize,
    #[cfg(feature = "wtypes")]
    pub put_SaclDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_SaclDefaulted: usize,
    pub CopySecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsSecurityDescriptor_Impl: super::IDispatch_Impl {
    fn get_Revision(&self) -> windows_core::Result<i32>;
    fn put_Revision(&self, lnrevision: i32) -> windows_core::Result<()>;
    fn get_Control(&self) -> windows_core::Result<i32>;
    fn put_Control(&self, lncontrol: i32) -> windows_core::Result<()>;
    fn get_Owner(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Owner(&self, bstrowner: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_OwnerDefaulted(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_OwnerDefaulted(&self, fownerdefaulted: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Group(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Group(&self, bstrgroup: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_GroupDefaulted(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_GroupDefaulted(&self, fgroupdefaulted: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_DiscretionaryAcl(&self) -> windows_core::Result<super::IDispatch>;
    fn put_DiscretionaryAcl(&self, pdiscretionaryacl: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn get_DaclDefaulted(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_DaclDefaulted(&self, fdacldefaulted: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_SystemAcl(&self) -> windows_core::Result<super::IDispatch>;
    fn put_SystemAcl(&self, psystemacl: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn get_SaclDefaulted(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_SaclDefaulted(&self, fsacldefaulted: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CopySecurityDescriptor(&self) -> windows_core::Result<super::IDispatch>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsSecurityDescriptor_Vtbl {
    pub const fn new<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Revision<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::get_Revision(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Revision<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnrevision: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::put_Revision(this, core::mem::transmute_copy(&lnrevision)).into()
            }
        }
        unsafe extern "system" fn get_Control<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::get_Control(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Control<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lncontrol: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::put_Control(this, core::mem::transmute_copy(&lncontrol)).into()
            }
        }
        unsafe extern "system" fn get_Owner<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::get_Owner(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Owner<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrowner: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::put_Owner(this, core::mem::transmute(&bstrowner)).into()
            }
        }
        unsafe extern "system" fn get_OwnerDefaulted<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::get_OwnerDefaulted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_OwnerDefaulted<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fownerdefaulted: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::put_OwnerDefaulted(this, core::mem::transmute_copy(&fownerdefaulted)).into()
            }
        }
        unsafe extern "system" fn get_Group<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::get_Group(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Group<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroup: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::put_Group(this, core::mem::transmute(&bstrgroup)).into()
            }
        }
        unsafe extern "system" fn get_GroupDefaulted<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::get_GroupDefaulted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_GroupDefaulted<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fgroupdefaulted: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::put_GroupDefaulted(this, core::mem::transmute_copy(&fgroupdefaulted)).into()
            }
        }
        unsafe extern "system" fn get_DiscretionaryAcl<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::get_DiscretionaryAcl(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DiscretionaryAcl<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdiscretionaryacl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::put_DiscretionaryAcl(this, core::mem::transmute_copy(&pdiscretionaryacl)).into()
            }
        }
        unsafe extern "system" fn get_DaclDefaulted<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::get_DaclDefaulted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DaclDefaulted<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdacldefaulted: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::put_DaclDefaulted(this, core::mem::transmute_copy(&fdacldefaulted)).into()
            }
        }
        unsafe extern "system" fn get_SystemAcl<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::get_SystemAcl(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SystemAcl<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psystemacl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::put_SystemAcl(this, core::mem::transmute_copy(&psystemacl)).into()
            }
        }
        unsafe extern "system" fn get_SaclDefaulted<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityDescriptor_Impl::get_SaclDefaulted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SaclDefaulted<Identity: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsacldefaulted: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityDescriptor_Impl::put_SaclDefaulted(this, core::mem::transmute_copy(&fsacldefaulted)).into()
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Revision: get_Revision::<Identity, OFFSET>,
            put_Revision: put_Revision::<Identity, OFFSET>,
            get_Control: get_Control::<Identity, OFFSET>,
            put_Control: put_Control::<Identity, OFFSET>,
            get_Owner: get_Owner::<Identity, OFFSET>,
            put_Owner: put_Owner::<Identity, OFFSET>,
            get_OwnerDefaulted: get_OwnerDefaulted::<Identity, OFFSET>,
            put_OwnerDefaulted: put_OwnerDefaulted::<Identity, OFFSET>,
            get_Group: get_Group::<Identity, OFFSET>,
            put_Group: put_Group::<Identity, OFFSET>,
            get_GroupDefaulted: get_GroupDefaulted::<Identity, OFFSET>,
            put_GroupDefaulted: put_GroupDefaulted::<Identity, OFFSET>,
            get_DiscretionaryAcl: get_DiscretionaryAcl::<Identity, OFFSET>,
            put_DiscretionaryAcl: put_DiscretionaryAcl::<Identity, OFFSET>,
            get_DaclDefaulted: get_DaclDefaulted::<Identity, OFFSET>,
            put_DaclDefaulted: put_DaclDefaulted::<Identity, OFFSET>,
            get_SystemAcl: get_SystemAcl::<Identity, OFFSET>,
            put_SystemAcl: put_SystemAcl::<Identity, OFFSET>,
            get_SaclDefaulted: get_SaclDefaulted::<Identity, OFFSET>,
            put_SaclDefaulted: put_SaclDefaulted::<Identity, OFFSET>,
            CopySecurityDescriptor: CopySecurityDescriptor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsSecurityDescriptor as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsSecurityDescriptor {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsSecurityUtility, IADsSecurityUtility_Vtbl, 0xa63251b2_5f21_474b_ab52_4a8efad10895);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsSecurityUtility {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsSecurityUtility, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsSecurityUtility {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetSecurityDescriptor(&self, varpath: &super::VARIANT, lpathformat: i32, lformat: i32) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurityDescriptor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varpath), lpathformat, lformat, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetSecurityDescriptor(&self, varpath: &super::VARIANT, lpathformat: i32, vardata: &super::VARIANT, ldataformat: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSecurityDescriptor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varpath), lpathformat, core::mem::transmute_copy(vardata), ldataformat) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ConvertSecurityDescriptor(&self, varsd: &super::VARIANT, ldataformat: i32, loutformat: i32) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConvertSecurityDescriptor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varsd), ldataformat, loutformat, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_SecurityMask(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SecurityMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_SecurityMask(&self, lnsecuritymask: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SecurityMask)(windows_core::Interface::as_raw(self), lnsecuritymask) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsSecurityUtility_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, i32, i32, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetSecurityDescriptor: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub SetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, i32, super::VARIANT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    SetSecurityDescriptor: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub ConvertSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, i32, i32, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    ConvertSecurityDescriptor: usize,
    pub get_SecurityMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_SecurityMask: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsSecurityUtility_Impl: super::IDispatch_Impl {
    fn GetSecurityDescriptor(&self, varpath: &super::VARIANT, lpathformat: i32, lformat: i32) -> windows_core::Result<super::VARIANT>;
    fn SetSecurityDescriptor(&self, varpath: &super::VARIANT, lpathformat: i32, vardata: &super::VARIANT, ldataformat: i32) -> windows_core::Result<()>;
    fn ConvertSecurityDescriptor(&self, varsd: &super::VARIANT, ldataformat: i32, loutformat: i32) -> windows_core::Result<super::VARIANT>;
    fn get_SecurityMask(&self) -> windows_core::Result<i32>;
    fn put_SecurityMask(&self, lnsecuritymask: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsSecurityUtility_Vtbl {
    pub const fn new<Identity: IADsSecurityUtility_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSecurityDescriptor<Identity: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varpath: super::VARIANT, lpathformat: i32, lformat: i32, pvariant: *mut super::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetSecurityDescriptor<Identity: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varpath: super::VARIANT, lpathformat: i32, vardata: super::VARIANT, ldataformat: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityUtility_Impl::SetSecurityDescriptor(this, core::mem::transmute(&varpath), core::mem::transmute_copy(&lpathformat), core::mem::transmute(&vardata), core::mem::transmute_copy(&ldataformat)).into()
            }
        }
        unsafe extern "system" fn ConvertSecurityDescriptor<Identity: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsd: super::VARIANT, ldataformat: i32, loutformat: i32, presult: *mut super::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn get_SecurityMask<Identity: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSecurityUtility_Impl::get_SecurityMask(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SecurityMask<Identity: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnsecuritymask: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSecurityUtility_Impl::put_SecurityMask(this, core::mem::transmute_copy(&lnsecuritymask)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, OFFSET>,
            ConvertSecurityDescriptor: ConvertSecurityDescriptor::<Identity, OFFSET>,
            get_SecurityMask: get_SecurityMask::<Identity, OFFSET>,
            put_SecurityMask: put_SecurityMask::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsSecurityUtility as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsSecurityUtility {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsService, IADsService_Vtbl, 0x68af66e0_31ca_11cf_a98a_00aa006bc149);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsService {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsService, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsService {
    pub unsafe fn get_HostComputer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HostComputer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_HostComputer(&self, bstrhostcomputer: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_HostComputer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrhostcomputer)) }
    }
    pub unsafe fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_DisplayName(&self, bstrdisplayname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdisplayname)) }
    }
    pub unsafe fn get_Version(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Version)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Version(&self, bstrversion: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Version)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrversion)) }
    }
    pub unsafe fn get_ServiceType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ServiceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ServiceType(&self, lnservicetype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ServiceType)(windows_core::Interface::as_raw(self), lnservicetype) }
    }
    pub unsafe fn get_StartType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_StartType(&self, lnstarttype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StartType)(windows_core::Interface::as_raw(self), lnstarttype) }
    }
    pub unsafe fn get_Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Path(&self, bstrpath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Path)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpath)) }
    }
    pub unsafe fn get_StartupParameters(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartupParameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_StartupParameters(&self, bstrstartupparameters: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StartupParameters)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrstartupparameters)) }
    }
    pub unsafe fn get_ErrorControl(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ErrorControl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ErrorControl(&self, lnerrorcontrol: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ErrorControl)(windows_core::Interface::as_raw(self), lnerrorcontrol) }
    }
    pub unsafe fn get_LoadOrderGroup(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LoadOrderGroup)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_LoadOrderGroup(&self, bstrloadordergroup: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LoadOrderGroup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrloadordergroup)) }
    }
    pub unsafe fn get_ServiceAccountName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ServiceAccountName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ServiceAccountName(&self, bstrserviceaccountname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ServiceAccountName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrserviceaccountname)) }
    }
    pub unsafe fn get_ServiceAccountPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ServiceAccountPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ServiceAccountPath(&self, bstrserviceaccountpath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ServiceAccountPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrserviceaccountpath)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Dependencies(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Dependencies)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_Dependencies(&self, vdependencies: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Dependencies)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vdependencies)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsService_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_HostComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_HostComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ServiceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_ServiceType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_StartType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_StartType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_StartupParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_StartupParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ErrorControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_ErrorControl: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_LoadOrderGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_LoadOrderGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ServiceAccountName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ServiceAccountName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ServiceAccountPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ServiceAccountPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Dependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Dependencies: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_Dependencies: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_Dependencies: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsService_Impl: IADs_Impl {
    fn get_HostComputer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_HostComputer(&self, bstrhostcomputer: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_DisplayName(&self, bstrdisplayname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Version(&self, bstrversion: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ServiceType(&self) -> windows_core::Result<i32>;
    fn put_ServiceType(&self, lnservicetype: i32) -> windows_core::Result<()>;
    fn get_StartType(&self) -> windows_core::Result<i32>;
    fn put_StartType(&self, lnstarttype: i32) -> windows_core::Result<()>;
    fn get_Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Path(&self, bstrpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_StartupParameters(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_StartupParameters(&self, bstrstartupparameters: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ErrorControl(&self) -> windows_core::Result<i32>;
    fn put_ErrorControl(&self, lnerrorcontrol: i32) -> windows_core::Result<()>;
    fn get_LoadOrderGroup(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_LoadOrderGroup(&self, bstrloadordergroup: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ServiceAccountName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ServiceAccountName(&self, bstrserviceaccountname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ServiceAccountPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ServiceAccountPath(&self, bstrserviceaccountpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Dependencies(&self) -> windows_core::Result<super::VARIANT>;
    fn put_Dependencies(&self, vdependencies: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsService_Vtbl {
    pub const fn new<Identity: IADsService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_HostComputer<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::get_HostComputer(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_HostComputer<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhostcomputer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::put_HostComputer(this, core::mem::transmute(&bstrhostcomputer)).into()
            }
        }
        unsafe extern "system" fn get_DisplayName<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::get_DisplayName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DisplayName<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdisplayname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::put_DisplayName(this, core::mem::transmute(&bstrdisplayname)).into()
            }
        }
        unsafe extern "system" fn get_Version<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::get_Version(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Version<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrversion: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::put_Version(this, core::mem::transmute(&bstrversion)).into()
            }
        }
        unsafe extern "system" fn get_ServiceType<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::get_ServiceType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ServiceType<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnservicetype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::put_ServiceType(this, core::mem::transmute_copy(&lnservicetype)).into()
            }
        }
        unsafe extern "system" fn get_StartType<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::get_StartType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StartType<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnstarttype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::put_StartType(this, core::mem::transmute_copy(&lnstarttype)).into()
            }
        }
        unsafe extern "system" fn get_Path<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::get_Path(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Path<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::put_Path(this, core::mem::transmute(&bstrpath)).into()
            }
        }
        unsafe extern "system" fn get_StartupParameters<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::get_StartupParameters(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StartupParameters<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrstartupparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::put_StartupParameters(this, core::mem::transmute(&bstrstartupparameters)).into()
            }
        }
        unsafe extern "system" fn get_ErrorControl<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::get_ErrorControl(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ErrorControl<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnerrorcontrol: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::put_ErrorControl(this, core::mem::transmute_copy(&lnerrorcontrol)).into()
            }
        }
        unsafe extern "system" fn get_LoadOrderGroup<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::get_LoadOrderGroup(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LoadOrderGroup<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrloadordergroup: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::put_LoadOrderGroup(this, core::mem::transmute(&bstrloadordergroup)).into()
            }
        }
        unsafe extern "system" fn get_ServiceAccountName<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::get_ServiceAccountName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ServiceAccountName<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrserviceaccountname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::put_ServiceAccountName(this, core::mem::transmute(&bstrserviceaccountname)).into()
            }
        }
        unsafe extern "system" fn get_ServiceAccountPath<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::get_ServiceAccountPath(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ServiceAccountPath<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrserviceaccountpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::put_ServiceAccountPath(this, core::mem::transmute(&bstrserviceaccountpath)).into()
            }
        }
        unsafe extern "system" fn get_Dependencies<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsService_Impl::get_Dependencies(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Dependencies<Identity: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vdependencies: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsService_Impl::put_Dependencies(this, core::mem::transmute(&vdependencies)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            get_HostComputer: get_HostComputer::<Identity, OFFSET>,
            put_HostComputer: put_HostComputer::<Identity, OFFSET>,
            get_DisplayName: get_DisplayName::<Identity, OFFSET>,
            put_DisplayName: put_DisplayName::<Identity, OFFSET>,
            get_Version: get_Version::<Identity, OFFSET>,
            put_Version: put_Version::<Identity, OFFSET>,
            get_ServiceType: get_ServiceType::<Identity, OFFSET>,
            put_ServiceType: put_ServiceType::<Identity, OFFSET>,
            get_StartType: get_StartType::<Identity, OFFSET>,
            put_StartType: put_StartType::<Identity, OFFSET>,
            get_Path: get_Path::<Identity, OFFSET>,
            put_Path: put_Path::<Identity, OFFSET>,
            get_StartupParameters: get_StartupParameters::<Identity, OFFSET>,
            put_StartupParameters: put_StartupParameters::<Identity, OFFSET>,
            get_ErrorControl: get_ErrorControl::<Identity, OFFSET>,
            put_ErrorControl: put_ErrorControl::<Identity, OFFSET>,
            get_LoadOrderGroup: get_LoadOrderGroup::<Identity, OFFSET>,
            put_LoadOrderGroup: put_LoadOrderGroup::<Identity, OFFSET>,
            get_ServiceAccountName: get_ServiceAccountName::<Identity, OFFSET>,
            put_ServiceAccountName: put_ServiceAccountName::<Identity, OFFSET>,
            get_ServiceAccountPath: get_ServiceAccountPath::<Identity, OFFSET>,
            put_ServiceAccountPath: put_ServiceAccountPath::<Identity, OFFSET>,
            get_Dependencies: get_Dependencies::<Identity, OFFSET>,
            put_Dependencies: put_Dependencies::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsService as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsService {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsServiceOperations, IADsServiceOperations_Vtbl, 0x5d7b33f0_31ca_11cf_a98a_00aa006bc149);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsServiceOperations {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsServiceOperations, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsServiceOperations {
    pub unsafe fn get_Status(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsServiceOperations_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Continue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsServiceOperations_Impl: IADs_Impl {
    fn get_Status(&self) -> windows_core::Result<i32>;
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Continue(&self) -> windows_core::Result<()>;
    fn SetPassword(&self, bstrnewpassword: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsServiceOperations_Vtbl {
    pub const fn new<Identity: IADsServiceOperations_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Status<Identity: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsServiceOperations_Impl::get_Status(this) {
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
            get_Status: get_Status::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Continue: Continue::<Identity, OFFSET>,
            SetPassword: SetPassword::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsServiceOperations as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsServiceOperations {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsSession, IADsSession_Vtbl, 0x398b7da0_4aab_11cf_ae2c_00aa006ebfb9);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsSession {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsSession, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsSession {
    pub unsafe fn get_User(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_User)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_UserPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UserPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Computer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Computer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_ComputerPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ComputerPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_ConnectTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ConnectTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_IdleTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IdleTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsSession_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_UserPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Computer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ComputerPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ConnectTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_IdleTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsSession_Impl: IADs_Impl {
    fn get_User(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_UserPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Computer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_ComputerPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_ConnectTime(&self) -> windows_core::Result<i32>;
    fn get_IdleTime(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsSession_Vtbl {
    pub const fn new<Identity: IADsSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_User<Identity: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSession_Impl::get_User(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_UserPath<Identity: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSession_Impl::get_UserPath(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Computer<Identity: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSession_Impl::get_Computer(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ComputerPath<Identity: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSession_Impl::get_ComputerPath(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ConnectTime<Identity: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSession_Impl::get_ConnectTime(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IdleTime<Identity: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSession_Impl::get_IdleTime(this) {
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
            get_User: get_User::<Identity, OFFSET>,
            get_UserPath: get_UserPath::<Identity, OFFSET>,
            get_Computer: get_Computer::<Identity, OFFSET>,
            get_ComputerPath: get_ComputerPath::<Identity, OFFSET>,
            get_ConnectTime: get_ConnectTime::<Identity, OFFSET>,
            get_IdleTime: get_IdleTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsSession as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsSession {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsSyntax, IADsSyntax_Vtbl, 0xc8f93dd2_4ae0_11cf_9e73_00aa004a5691);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsSyntax {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsSyntax, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsSyntax {
    pub unsafe fn get_OleAutoDataType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OleAutoDataType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_OleAutoDataType(&self, lnoleautodatatype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_OleAutoDataType)(windows_core::Interface::as_raw(self), lnoleautodatatype) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsSyntax_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_OleAutoDataType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_OleAutoDataType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsSyntax_Impl: IADs_Impl {
    fn get_OleAutoDataType(&self) -> windows_core::Result<i32>;
    fn put_OleAutoDataType(&self, lnoleautodatatype: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsSyntax_Vtbl {
    pub const fn new<Identity: IADsSyntax_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_OleAutoDataType<Identity: IADsSyntax_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsSyntax_Impl::get_OleAutoDataType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_OleAutoDataType<Identity: IADsSyntax_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnoleautodatatype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsSyntax_Impl::put_OleAutoDataType(this, core::mem::transmute_copy(&lnoleautodatatype)).into()
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, OFFSET>(),
            get_OleAutoDataType: get_OleAutoDataType::<Identity, OFFSET>,
            put_OleAutoDataType: put_OleAutoDataType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsSyntax as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsSyntax {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsTimestamp, IADsTimestamp_Vtbl, 0xb2f5a901_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsTimestamp {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsTimestamp, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsTimestamp {
    pub unsafe fn get_WholeSeconds(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WholeSeconds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_WholeSeconds(&self, lnwholeseconds: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_WholeSeconds)(windows_core::Interface::as_raw(self), lnwholeseconds) }
    }
    pub unsafe fn get_EventID(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EventID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_EventID(&self, lneventid: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_EventID)(windows_core::Interface::as_raw(self), lneventid) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsTimestamp_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_WholeSeconds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_WholeSeconds: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_EventID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_EventID: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsTimestamp_Impl: super::IDispatch_Impl {
    fn get_WholeSeconds(&self) -> windows_core::Result<i32>;
    fn put_WholeSeconds(&self, lnwholeseconds: i32) -> windows_core::Result<()>;
    fn get_EventID(&self) -> windows_core::Result<i32>;
    fn put_EventID(&self, lneventid: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsTimestamp_Vtbl {
    pub const fn new<Identity: IADsTimestamp_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_WholeSeconds<Identity: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTimestamp_Impl::get_WholeSeconds(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_WholeSeconds<Identity: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnwholeseconds: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTimestamp_Impl::put_WholeSeconds(this, core::mem::transmute_copy(&lnwholeseconds)).into()
            }
        }
        unsafe extern "system" fn get_EventID<Identity: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTimestamp_Impl::get_EventID(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_EventID<Identity: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lneventid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTimestamp_Impl::put_EventID(this, core::mem::transmute_copy(&lneventid)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_WholeSeconds: get_WholeSeconds::<Identity, OFFSET>,
            put_WholeSeconds: put_WholeSeconds::<Identity, OFFSET>,
            get_EventID: get_EventID::<Identity, OFFSET>,
            put_EventID: put_EventID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsTimestamp as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsTimestamp {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsTypedName, IADsTypedName_Vtbl, 0xb371a349_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsTypedName {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsTypedName, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsTypedName {
    pub unsafe fn get_ObjectName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ObjectName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ObjectName(&self, bstrobjectname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ObjectName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrobjectname)) }
    }
    pub unsafe fn get_Level(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Level)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Level(&self, lnlevel: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Level)(windows_core::Interface::as_raw(self), lnlevel) }
    }
    pub unsafe fn get_Interval(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Interval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Interval(&self, lninterval: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Interval)(windows_core::Interface::as_raw(self), lninterval) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsTypedName_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_ObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Level: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Level: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Interval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Interval: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsTypedName_Impl: super::IDispatch_Impl {
    fn get_ObjectName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ObjectName(&self, bstrobjectname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Level(&self) -> windows_core::Result<i32>;
    fn put_Level(&self, lnlevel: i32) -> windows_core::Result<()>;
    fn get_Interval(&self) -> windows_core::Result<i32>;
    fn put_Interval(&self, lninterval: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsTypedName_Vtbl {
    pub const fn new<Identity: IADsTypedName_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ObjectName<Identity: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTypedName_Impl::get_ObjectName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ObjectName<Identity: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrobjectname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTypedName_Impl::put_ObjectName(this, core::mem::transmute(&bstrobjectname)).into()
            }
        }
        unsafe extern "system" fn get_Level<Identity: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTypedName_Impl::get_Level(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Level<Identity: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnlevel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTypedName_Impl::put_Level(this, core::mem::transmute_copy(&lnlevel)).into()
            }
        }
        unsafe extern "system" fn get_Interval<Identity: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTypedName_Impl::get_Interval(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Interval<Identity: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lninterval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTypedName_Impl::put_Interval(this, core::mem::transmute_copy(&lninterval)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ObjectName: get_ObjectName::<Identity, OFFSET>,
            put_ObjectName: put_ObjectName::<Identity, OFFSET>,
            get_Level: get_Level::<Identity, OFFSET>,
            put_Level: put_Level::<Identity, OFFSET>,
            get_Interval: get_Interval::<Identity, OFFSET>,
            put_Interval: put_Interval::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsTypedName as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsTypedName {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsUser, IADsUser_Vtbl, 0x3e37e320_17e2_11cf_abc4_02608c9e7553);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsUser {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsUser, windows_core::IUnknown, super::IDispatch, IADs);
#[cfg(feature = "oaidl")]
impl IADsUser {
    pub unsafe fn get_BadLoginAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BadLoginAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_BadLoginCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BadLoginCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_LastLogin(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastLogin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_LastLogoff(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastLogoff)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_LastFailedLogin(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastFailedLogin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_PasswordLastChanged(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PasswordLastChanged)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Description)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)) }
    }
    pub unsafe fn get_Division(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Division)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Division(&self, bstrdivision: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Division)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdivision)) }
    }
    pub unsafe fn get_Department(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Department)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Department(&self, bstrdepartment: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Department)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdepartment)) }
    }
    pub unsafe fn get_EmployeeID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EmployeeID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_EmployeeID(&self, bstremployeeid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_EmployeeID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstremployeeid)) }
    }
    pub unsafe fn get_FullName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FullName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_FullName(&self, bstrfullname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FullName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfullname)) }
    }
    pub unsafe fn get_FirstName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FirstName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_FirstName(&self, bstrfirstname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FirstName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfirstname)) }
    }
    pub unsafe fn get_LastName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_LastName(&self, bstrlastname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LastName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlastname)) }
    }
    pub unsafe fn get_OtherName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OtherName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_OtherName(&self, bstrothername: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_OtherName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrothername)) }
    }
    pub unsafe fn get_NamePrefix(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NamePrefix)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_NamePrefix(&self, bstrnameprefix: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_NamePrefix)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnameprefix)) }
    }
    pub unsafe fn get_NameSuffix(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NameSuffix)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_NameSuffix(&self, bstrnamesuffix: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_NameSuffix)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnamesuffix)) }
    }
    pub unsafe fn get_Title(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Title)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Title(&self, bstrtitle: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Title)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtitle)) }
    }
    pub unsafe fn get_Manager(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Manager)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Manager(&self, bstrmanager: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Manager)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmanager)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_TelephoneHome(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TelephoneHome)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_TelephoneHome(&self, vtelephonehome: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_TelephoneHome)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vtelephonehome)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_TelephoneMobile(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TelephoneMobile)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_TelephoneMobile(&self, vtelephonemobile: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_TelephoneMobile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vtelephonemobile)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_TelephoneNumber(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TelephoneNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_TelephoneNumber(&self, vtelephonenumber: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_TelephoneNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vtelephonenumber)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_TelephonePager(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TelephonePager)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_TelephonePager(&self, vtelephonepager: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_TelephonePager)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vtelephonepager)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_FaxNumber(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FaxNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_FaxNumber(&self, vfaxnumber: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FaxNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vfaxnumber)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_OfficeLocations(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OfficeLocations)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_OfficeLocations(&self, vofficelocations: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_OfficeLocations)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vofficelocations)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_PostalAddresses(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PostalAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_PostalAddresses(&self, vpostaladdresses: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PostalAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vpostaladdresses)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_PostalCodes(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PostalCodes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_PostalCodes(&self, vpostalcodes: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PostalCodes)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vpostalcodes)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_SeeAlso(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SeeAlso)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_SeeAlso(&self, vseealso: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SeeAlso)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vseealso)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AccountDisabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AccountDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AccountDisabled(&self, faccountdisabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AccountDisabled)(windows_core::Interface::as_raw(self), faccountdisabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AccountExpirationDate(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AccountExpirationDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AccountExpirationDate(&self, daaccountexpirationdate: super::DATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AccountExpirationDate)(windows_core::Interface::as_raw(self), daaccountexpirationdate) }
    }
    pub unsafe fn get_GraceLoginsAllowed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_GraceLoginsAllowed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_GraceLoginsAllowed(&self, lngraceloginsallowed: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_GraceLoginsAllowed)(windows_core::Interface::as_raw(self), lngraceloginsallowed) }
    }
    pub unsafe fn get_GraceLoginsRemaining(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_GraceLoginsRemaining)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_GraceLoginsRemaining(&self, lngraceloginsremaining: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_GraceLoginsRemaining)(windows_core::Interface::as_raw(self), lngraceloginsremaining) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsAccountLocked(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsAccountLocked)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_IsAccountLocked(&self, fisaccountlocked: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IsAccountLocked)(windows_core::Interface::as_raw(self), fisaccountlocked) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_LoginHours(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LoginHours)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_LoginHours(&self, vloginhours: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LoginHours)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vloginhours)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_LoginWorkstations(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LoginWorkstations)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_LoginWorkstations(&self, vloginworkstations: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LoginWorkstations)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vloginworkstations)) }
    }
    pub unsafe fn get_MaxLogins(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MaxLogins)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MaxLogins(&self, lnmaxlogins: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MaxLogins)(windows_core::Interface::as_raw(self), lnmaxlogins) }
    }
    pub unsafe fn get_MaxStorage(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MaxStorage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MaxStorage(&self, lnmaxstorage: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MaxStorage)(windows_core::Interface::as_raw(self), lnmaxstorage) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_PasswordExpirationDate(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PasswordExpirationDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_PasswordExpirationDate(&self, dapasswordexpirationdate: super::DATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PasswordExpirationDate)(windows_core::Interface::as_raw(self), dapasswordexpirationdate) }
    }
    pub unsafe fn get_PasswordMinimumLength(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PasswordMinimumLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_PasswordMinimumLength(&self, lnpasswordminimumlength: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PasswordMinimumLength)(windows_core::Interface::as_raw(self), lnpasswordminimumlength) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_PasswordRequired(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PasswordRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_PasswordRequired(&self, fpasswordrequired: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PasswordRequired)(windows_core::Interface::as_raw(self), fpasswordrequired) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_RequireUniquePassword(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RequireUniquePassword)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_RequireUniquePassword(&self, frequireuniquepassword: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RequireUniquePassword)(windows_core::Interface::as_raw(self), frequireuniquepassword) }
    }
    pub unsafe fn get_EmailAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EmailAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_EmailAddress(&self, bstremailaddress: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_EmailAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstremailaddress)) }
    }
    pub unsafe fn get_HomeDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HomeDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_HomeDirectory(&self, bstrhomedirectory: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_HomeDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrhomedirectory)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Languages(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Languages)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_Languages(&self, vlanguages: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Languages)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vlanguages)) }
    }
    pub unsafe fn get_Profile(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Profile)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Profile(&self, bstrprofile: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Profile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprofile)) }
    }
    pub unsafe fn get_LoginScript(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LoginScript)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_LoginScript(&self, bstrloginscript: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LoginScript)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrloginscript)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Picture(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Picture)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_Picture(&self, vpicture: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Picture)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vpicture)) }
    }
    pub unsafe fn get_HomePage(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HomePage)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_HomePage(&self, bstrhomepage: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_HomePage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrhomepage)) }
    }
    pub unsafe fn Groups(&self) -> windows_core::Result<IADsMembers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Groups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPassword(&self, newpassword: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPassword)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newpassword)) }
    }
    pub unsafe fn ChangePassword(&self, bstroldpassword: &windows_core::BSTR, bstrnewpassword: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ChangePassword)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstroldpassword), core::mem::transmute_copy(bstrnewpassword)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsUser_Vtbl {
    pub base__: IADs_Vtbl,
    pub get_BadLoginAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_BadLoginCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_LastLogin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_LastLogin: usize,
    #[cfg(feature = "wtypes")]
    pub get_LastLogoff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_LastLogoff: usize,
    #[cfg(feature = "wtypes")]
    pub get_LastFailedLogin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_LastFailedLogin: usize,
    #[cfg(feature = "wtypes")]
    pub get_PasswordLastChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_PasswordLastChanged: usize,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Division: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Division: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Department: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Department: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_EmployeeID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_EmployeeID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_FullName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_FullName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_FirstName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_FirstName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_LastName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_LastName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_OtherName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_OtherName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_NamePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_NamePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_NameSuffix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_NameSuffix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Manager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Manager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_TelephoneHome: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_TelephoneHome: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_TelephoneHome: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_TelephoneHome: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_TelephoneMobile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_TelephoneMobile: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_TelephoneMobile: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_TelephoneMobile: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_TelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_TelephoneNumber: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_TelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_TelephoneNumber: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_TelephonePager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_TelephonePager: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_TelephonePager: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_TelephonePager: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_FaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_FaxNumber: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_FaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_FaxNumber: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_OfficeLocations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_OfficeLocations: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_OfficeLocations: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_OfficeLocations: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_PostalAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_PostalAddresses: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_PostalAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_PostalAddresses: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_PostalCodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_PostalCodes: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_PostalCodes: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_PostalCodes: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_SeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_SeeAlso: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_SeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_SeeAlso: usize,
    #[cfg(feature = "wtypes")]
    pub get_AccountDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AccountDisabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_AccountDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AccountDisabled: usize,
    #[cfg(feature = "wtypes")]
    pub get_AccountExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AccountExpirationDate: usize,
    #[cfg(feature = "wtypes")]
    pub put_AccountExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AccountExpirationDate: usize,
    pub get_GraceLoginsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_GraceLoginsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_GraceLoginsRemaining: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_GraceLoginsRemaining: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IsAccountLocked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsAccountLocked: usize,
    #[cfg(feature = "wtypes")]
    pub put_IsAccountLocked: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_IsAccountLocked: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_LoginHours: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_LoginHours: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_LoginHours: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_LoginHours: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_LoginWorkstations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_LoginWorkstations: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_LoginWorkstations: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_LoginWorkstations: usize,
    pub get_MaxLogins: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_MaxLogins: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_MaxStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_MaxStorage: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_PasswordExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_PasswordExpirationDate: usize,
    #[cfg(feature = "wtypes")]
    pub put_PasswordExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_PasswordExpirationDate: usize,
    pub get_PasswordMinimumLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_PasswordMinimumLength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_PasswordRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_PasswordRequired: usize,
    #[cfg(feature = "wtypes")]
    pub put_PasswordRequired: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_PasswordRequired: usize,
    #[cfg(feature = "wtypes")]
    pub get_RequireUniquePassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_RequireUniquePassword: usize,
    #[cfg(feature = "wtypes")]
    pub put_RequireUniquePassword: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_RequireUniquePassword: usize,
    pub get_EmailAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_EmailAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_HomeDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_HomeDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Languages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Languages: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_Languages: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_Languages: usize,
    pub get_Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_LoginScript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_LoginScript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Picture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Picture: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_Picture: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_Picture: usize,
    pub get_HomePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_HomePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Groups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChangePassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsUser_Impl: IADs_Impl {
    fn get_BadLoginAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_BadLoginCount(&self) -> windows_core::Result<i32>;
    fn get_LastLogin(&self) -> windows_core::Result<super::DATE>;
    fn get_LastLogoff(&self) -> windows_core::Result<super::DATE>;
    fn get_LastFailedLogin(&self) -> windows_core::Result<super::DATE>;
    fn get_PasswordLastChanged(&self) -> windows_core::Result<super::DATE>;
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Description(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Division(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Division(&self, bstrdivision: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Department(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Department(&self, bstrdepartment: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_EmployeeID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_EmployeeID(&self, bstremployeeid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_FullName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_FullName(&self, bstrfullname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_FirstName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_FirstName(&self, bstrfirstname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_LastName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_LastName(&self, bstrlastname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_OtherName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_OtherName(&self, bstrothername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_NamePrefix(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_NamePrefix(&self, bstrnameprefix: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_NameSuffix(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_NameSuffix(&self, bstrnamesuffix: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Title(&self, bstrtitle: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Manager(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Manager(&self, bstrmanager: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_TelephoneHome(&self) -> windows_core::Result<super::VARIANT>;
    fn put_TelephoneHome(&self, vtelephonehome: &super::VARIANT) -> windows_core::Result<()>;
    fn get_TelephoneMobile(&self) -> windows_core::Result<super::VARIANT>;
    fn put_TelephoneMobile(&self, vtelephonemobile: &super::VARIANT) -> windows_core::Result<()>;
    fn get_TelephoneNumber(&self) -> windows_core::Result<super::VARIANT>;
    fn put_TelephoneNumber(&self, vtelephonenumber: &super::VARIANT) -> windows_core::Result<()>;
    fn get_TelephonePager(&self) -> windows_core::Result<super::VARIANT>;
    fn put_TelephonePager(&self, vtelephonepager: &super::VARIANT) -> windows_core::Result<()>;
    fn get_FaxNumber(&self) -> windows_core::Result<super::VARIANT>;
    fn put_FaxNumber(&self, vfaxnumber: &super::VARIANT) -> windows_core::Result<()>;
    fn get_OfficeLocations(&self) -> windows_core::Result<super::VARIANT>;
    fn put_OfficeLocations(&self, vofficelocations: &super::VARIANT) -> windows_core::Result<()>;
    fn get_PostalAddresses(&self) -> windows_core::Result<super::VARIANT>;
    fn put_PostalAddresses(&self, vpostaladdresses: &super::VARIANT) -> windows_core::Result<()>;
    fn get_PostalCodes(&self) -> windows_core::Result<super::VARIANT>;
    fn put_PostalCodes(&self, vpostalcodes: &super::VARIANT) -> windows_core::Result<()>;
    fn get_SeeAlso(&self) -> windows_core::Result<super::VARIANT>;
    fn put_SeeAlso(&self, vseealso: &super::VARIANT) -> windows_core::Result<()>;
    fn get_AccountDisabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AccountDisabled(&self, faccountdisabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_AccountExpirationDate(&self) -> windows_core::Result<super::DATE>;
    fn put_AccountExpirationDate(&self, daaccountexpirationdate: super::DATE) -> windows_core::Result<()>;
    fn get_GraceLoginsAllowed(&self) -> windows_core::Result<i32>;
    fn put_GraceLoginsAllowed(&self, lngraceloginsallowed: i32) -> windows_core::Result<()>;
    fn get_GraceLoginsRemaining(&self) -> windows_core::Result<i32>;
    fn put_GraceLoginsRemaining(&self, lngraceloginsremaining: i32) -> windows_core::Result<()>;
    fn get_IsAccountLocked(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_IsAccountLocked(&self, fisaccountlocked: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_LoginHours(&self) -> windows_core::Result<super::VARIANT>;
    fn put_LoginHours(&self, vloginhours: &super::VARIANT) -> windows_core::Result<()>;
    fn get_LoginWorkstations(&self) -> windows_core::Result<super::VARIANT>;
    fn put_LoginWorkstations(&self, vloginworkstations: &super::VARIANT) -> windows_core::Result<()>;
    fn get_MaxLogins(&self) -> windows_core::Result<i32>;
    fn put_MaxLogins(&self, lnmaxlogins: i32) -> windows_core::Result<()>;
    fn get_MaxStorage(&self) -> windows_core::Result<i32>;
    fn put_MaxStorage(&self, lnmaxstorage: i32) -> windows_core::Result<()>;
    fn get_PasswordExpirationDate(&self) -> windows_core::Result<super::DATE>;
    fn put_PasswordExpirationDate(&self, dapasswordexpirationdate: super::DATE) -> windows_core::Result<()>;
    fn get_PasswordMinimumLength(&self) -> windows_core::Result<i32>;
    fn put_PasswordMinimumLength(&self, lnpasswordminimumlength: i32) -> windows_core::Result<()>;
    fn get_PasswordRequired(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_PasswordRequired(&self, fpasswordrequired: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_RequireUniquePassword(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_RequireUniquePassword(&self, frequireuniquepassword: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_EmailAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_EmailAddress(&self, bstremailaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_HomeDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_HomeDirectory(&self, bstrhomedirectory: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Languages(&self) -> windows_core::Result<super::VARIANT>;
    fn put_Languages(&self, vlanguages: &super::VARIANT) -> windows_core::Result<()>;
    fn get_Profile(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Profile(&self, bstrprofile: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_LoginScript(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_LoginScript(&self, bstrloginscript: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Picture(&self) -> windows_core::Result<super::VARIANT>;
    fn put_Picture(&self, vpicture: &super::VARIANT) -> windows_core::Result<()>;
    fn get_HomePage(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_HomePage(&self, bstrhomepage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Groups(&self) -> windows_core::Result<IADsMembers>;
    fn SetPassword(&self, newpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ChangePassword(&self, bstroldpassword: &windows_core::BSTR, bstrnewpassword: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsUser_Vtbl {
    pub const fn new<Identity: IADsUser_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_BadLoginAddress<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_BadLoginAddress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_BadLoginCount<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_BadLoginCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastLogin<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_LastLogin(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastLogoff<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_LastLogoff(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastFailedLogin<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_LastFailedLogin(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PasswordLastChanged<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_PasswordLastChanged(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Description<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Description<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_Description(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn get_Division<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_Division(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Division<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdivision: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_Division(this, core::mem::transmute(&bstrdivision)).into()
            }
        }
        unsafe extern "system" fn get_Department<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_Department(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Department<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdepartment: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_Department(this, core::mem::transmute(&bstrdepartment)).into()
            }
        }
        unsafe extern "system" fn get_EmployeeID<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_EmployeeID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_EmployeeID<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstremployeeid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_EmployeeID(this, core::mem::transmute(&bstremployeeid)).into()
            }
        }
        unsafe extern "system" fn get_FullName<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_FullName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FullName<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfullname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_FullName(this, core::mem::transmute(&bstrfullname)).into()
            }
        }
        unsafe extern "system" fn get_FirstName<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_FirstName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FirstName<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfirstname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_FirstName(this, core::mem::transmute(&bstrfirstname)).into()
            }
        }
        unsafe extern "system" fn get_LastName<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_LastName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LastName<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlastname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_LastName(this, core::mem::transmute(&bstrlastname)).into()
            }
        }
        unsafe extern "system" fn get_OtherName<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_OtherName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_OtherName<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrothername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_OtherName(this, core::mem::transmute(&bstrothername)).into()
            }
        }
        unsafe extern "system" fn get_NamePrefix<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_NamePrefix(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_NamePrefix<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnameprefix: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_NamePrefix(this, core::mem::transmute(&bstrnameprefix)).into()
            }
        }
        unsafe extern "system" fn get_NameSuffix<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_NameSuffix(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_NameSuffix<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnamesuffix: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_NameSuffix(this, core::mem::transmute(&bstrnamesuffix)).into()
            }
        }
        unsafe extern "system" fn get_Title<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_Title(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Title<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtitle: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_Title(this, core::mem::transmute(&bstrtitle)).into()
            }
        }
        unsafe extern "system" fn get_Manager<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_Manager(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Manager<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmanager: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_Manager(this, core::mem::transmute(&bstrmanager)).into()
            }
        }
        unsafe extern "system" fn get_TelephoneHome<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_TelephoneHome(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_TelephoneHome<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtelephonehome: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_TelephoneHome(this, core::mem::transmute(&vtelephonehome)).into()
            }
        }
        unsafe extern "system" fn get_TelephoneMobile<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_TelephoneMobile(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_TelephoneMobile<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtelephonemobile: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_TelephoneMobile(this, core::mem::transmute(&vtelephonemobile)).into()
            }
        }
        unsafe extern "system" fn get_TelephoneNumber<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_TelephoneNumber(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_TelephoneNumber<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtelephonenumber: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_TelephoneNumber(this, core::mem::transmute(&vtelephonenumber)).into()
            }
        }
        unsafe extern "system" fn get_TelephonePager<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_TelephonePager(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_TelephonePager<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtelephonepager: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_TelephonePager(this, core::mem::transmute(&vtelephonepager)).into()
            }
        }
        unsafe extern "system" fn get_FaxNumber<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_FaxNumber(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FaxNumber<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vfaxnumber: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_FaxNumber(this, core::mem::transmute(&vfaxnumber)).into()
            }
        }
        unsafe extern "system" fn get_OfficeLocations<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_OfficeLocations(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_OfficeLocations<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vofficelocations: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_OfficeLocations(this, core::mem::transmute(&vofficelocations)).into()
            }
        }
        unsafe extern "system" fn get_PostalAddresses<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_PostalAddresses(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PostalAddresses<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vpostaladdresses: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_PostalAddresses(this, core::mem::transmute(&vpostaladdresses)).into()
            }
        }
        unsafe extern "system" fn get_PostalCodes<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_PostalCodes(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PostalCodes<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vpostalcodes: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_PostalCodes(this, core::mem::transmute(&vpostalcodes)).into()
            }
        }
        unsafe extern "system" fn get_SeeAlso<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_SeeAlso(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SeeAlso<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vseealso: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_SeeAlso(this, core::mem::transmute(&vseealso)).into()
            }
        }
        unsafe extern "system" fn get_AccountDisabled<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_AccountDisabled(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AccountDisabled<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, faccountdisabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_AccountDisabled(this, core::mem::transmute_copy(&faccountdisabled)).into()
            }
        }
        unsafe extern "system" fn get_AccountExpirationDate<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_AccountExpirationDate(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AccountExpirationDate<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, daaccountexpirationdate: super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_AccountExpirationDate(this, core::mem::transmute_copy(&daaccountexpirationdate)).into()
            }
        }
        unsafe extern "system" fn get_GraceLoginsAllowed<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_GraceLoginsAllowed(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_GraceLoginsAllowed<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lngraceloginsallowed: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_GraceLoginsAllowed(this, core::mem::transmute_copy(&lngraceloginsallowed)).into()
            }
        }
        unsafe extern "system" fn get_GraceLoginsRemaining<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_GraceLoginsRemaining(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_GraceLoginsRemaining<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lngraceloginsremaining: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_GraceLoginsRemaining(this, core::mem::transmute_copy(&lngraceloginsremaining)).into()
            }
        }
        unsafe extern "system" fn get_IsAccountLocked<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_IsAccountLocked(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IsAccountLocked<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fisaccountlocked: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_IsAccountLocked(this, core::mem::transmute_copy(&fisaccountlocked)).into()
            }
        }
        unsafe extern "system" fn get_LoginHours<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_LoginHours(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LoginHours<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vloginhours: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_LoginHours(this, core::mem::transmute(&vloginhours)).into()
            }
        }
        unsafe extern "system" fn get_LoginWorkstations<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_LoginWorkstations(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LoginWorkstations<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vloginworkstations: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_LoginWorkstations(this, core::mem::transmute(&vloginworkstations)).into()
            }
        }
        unsafe extern "system" fn get_MaxLogins<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_MaxLogins(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MaxLogins<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxlogins: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_MaxLogins(this, core::mem::transmute_copy(&lnmaxlogins)).into()
            }
        }
        unsafe extern "system" fn get_MaxStorage<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_MaxStorage(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MaxStorage<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxstorage: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_MaxStorage(this, core::mem::transmute_copy(&lnmaxstorage)).into()
            }
        }
        unsafe extern "system" fn get_PasswordExpirationDate<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_PasswordExpirationDate(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PasswordExpirationDate<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dapasswordexpirationdate: super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_PasswordExpirationDate(this, core::mem::transmute_copy(&dapasswordexpirationdate)).into()
            }
        }
        unsafe extern "system" fn get_PasswordMinimumLength<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_PasswordMinimumLength(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PasswordMinimumLength<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnpasswordminimumlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_PasswordMinimumLength(this, core::mem::transmute_copy(&lnpasswordminimumlength)).into()
            }
        }
        unsafe extern "system" fn get_PasswordRequired<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_PasswordRequired(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PasswordRequired<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fpasswordrequired: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_PasswordRequired(this, core::mem::transmute_copy(&fpasswordrequired)).into()
            }
        }
        unsafe extern "system" fn get_RequireUniquePassword<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_RequireUniquePassword(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RequireUniquePassword<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frequireuniquepassword: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_RequireUniquePassword(this, core::mem::transmute_copy(&frequireuniquepassword)).into()
            }
        }
        unsafe extern "system" fn get_EmailAddress<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_EmailAddress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_EmailAddress<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstremailaddress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_EmailAddress(this, core::mem::transmute(&bstremailaddress)).into()
            }
        }
        unsafe extern "system" fn get_HomeDirectory<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_HomeDirectory(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_HomeDirectory<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhomedirectory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_HomeDirectory(this, core::mem::transmute(&bstrhomedirectory)).into()
            }
        }
        unsafe extern "system" fn get_Languages<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_Languages(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Languages<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vlanguages: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_Languages(this, core::mem::transmute(&vlanguages)).into()
            }
        }
        unsafe extern "system" fn get_Profile<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_Profile(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Profile<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_Profile(this, core::mem::transmute(&bstrprofile)).into()
            }
        }
        unsafe extern "system" fn get_LoginScript<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_LoginScript(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LoginScript<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrloginscript: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_LoginScript(this, core::mem::transmute(&bstrloginscript)).into()
            }
        }
        unsafe extern "system" fn get_Picture<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_Picture(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Picture<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vpicture: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_Picture(this, core::mem::transmute(&vpicture)).into()
            }
        }
        unsafe extern "system" fn get_HomePage<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsUser_Impl::get_HomePage(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_HomePage<Identity: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhomepage: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsUser_Impl::put_HomePage(this, core::mem::transmute(&bstrhomepage)).into()
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
            get_BadLoginAddress: get_BadLoginAddress::<Identity, OFFSET>,
            get_BadLoginCount: get_BadLoginCount::<Identity, OFFSET>,
            get_LastLogin: get_LastLogin::<Identity, OFFSET>,
            get_LastLogoff: get_LastLogoff::<Identity, OFFSET>,
            get_LastFailedLogin: get_LastFailedLogin::<Identity, OFFSET>,
            get_PasswordLastChanged: get_PasswordLastChanged::<Identity, OFFSET>,
            get_Description: get_Description::<Identity, OFFSET>,
            put_Description: put_Description::<Identity, OFFSET>,
            get_Division: get_Division::<Identity, OFFSET>,
            put_Division: put_Division::<Identity, OFFSET>,
            get_Department: get_Department::<Identity, OFFSET>,
            put_Department: put_Department::<Identity, OFFSET>,
            get_EmployeeID: get_EmployeeID::<Identity, OFFSET>,
            put_EmployeeID: put_EmployeeID::<Identity, OFFSET>,
            get_FullName: get_FullName::<Identity, OFFSET>,
            put_FullName: put_FullName::<Identity, OFFSET>,
            get_FirstName: get_FirstName::<Identity, OFFSET>,
            put_FirstName: put_FirstName::<Identity, OFFSET>,
            get_LastName: get_LastName::<Identity, OFFSET>,
            put_LastName: put_LastName::<Identity, OFFSET>,
            get_OtherName: get_OtherName::<Identity, OFFSET>,
            put_OtherName: put_OtherName::<Identity, OFFSET>,
            get_NamePrefix: get_NamePrefix::<Identity, OFFSET>,
            put_NamePrefix: put_NamePrefix::<Identity, OFFSET>,
            get_NameSuffix: get_NameSuffix::<Identity, OFFSET>,
            put_NameSuffix: put_NameSuffix::<Identity, OFFSET>,
            get_Title: get_Title::<Identity, OFFSET>,
            put_Title: put_Title::<Identity, OFFSET>,
            get_Manager: get_Manager::<Identity, OFFSET>,
            put_Manager: put_Manager::<Identity, OFFSET>,
            get_TelephoneHome: get_TelephoneHome::<Identity, OFFSET>,
            put_TelephoneHome: put_TelephoneHome::<Identity, OFFSET>,
            get_TelephoneMobile: get_TelephoneMobile::<Identity, OFFSET>,
            put_TelephoneMobile: put_TelephoneMobile::<Identity, OFFSET>,
            get_TelephoneNumber: get_TelephoneNumber::<Identity, OFFSET>,
            put_TelephoneNumber: put_TelephoneNumber::<Identity, OFFSET>,
            get_TelephonePager: get_TelephonePager::<Identity, OFFSET>,
            put_TelephonePager: put_TelephonePager::<Identity, OFFSET>,
            get_FaxNumber: get_FaxNumber::<Identity, OFFSET>,
            put_FaxNumber: put_FaxNumber::<Identity, OFFSET>,
            get_OfficeLocations: get_OfficeLocations::<Identity, OFFSET>,
            put_OfficeLocations: put_OfficeLocations::<Identity, OFFSET>,
            get_PostalAddresses: get_PostalAddresses::<Identity, OFFSET>,
            put_PostalAddresses: put_PostalAddresses::<Identity, OFFSET>,
            get_PostalCodes: get_PostalCodes::<Identity, OFFSET>,
            put_PostalCodes: put_PostalCodes::<Identity, OFFSET>,
            get_SeeAlso: get_SeeAlso::<Identity, OFFSET>,
            put_SeeAlso: put_SeeAlso::<Identity, OFFSET>,
            get_AccountDisabled: get_AccountDisabled::<Identity, OFFSET>,
            put_AccountDisabled: put_AccountDisabled::<Identity, OFFSET>,
            get_AccountExpirationDate: get_AccountExpirationDate::<Identity, OFFSET>,
            put_AccountExpirationDate: put_AccountExpirationDate::<Identity, OFFSET>,
            get_GraceLoginsAllowed: get_GraceLoginsAllowed::<Identity, OFFSET>,
            put_GraceLoginsAllowed: put_GraceLoginsAllowed::<Identity, OFFSET>,
            get_GraceLoginsRemaining: get_GraceLoginsRemaining::<Identity, OFFSET>,
            put_GraceLoginsRemaining: put_GraceLoginsRemaining::<Identity, OFFSET>,
            get_IsAccountLocked: get_IsAccountLocked::<Identity, OFFSET>,
            put_IsAccountLocked: put_IsAccountLocked::<Identity, OFFSET>,
            get_LoginHours: get_LoginHours::<Identity, OFFSET>,
            put_LoginHours: put_LoginHours::<Identity, OFFSET>,
            get_LoginWorkstations: get_LoginWorkstations::<Identity, OFFSET>,
            put_LoginWorkstations: put_LoginWorkstations::<Identity, OFFSET>,
            get_MaxLogins: get_MaxLogins::<Identity, OFFSET>,
            put_MaxLogins: put_MaxLogins::<Identity, OFFSET>,
            get_MaxStorage: get_MaxStorage::<Identity, OFFSET>,
            put_MaxStorage: put_MaxStorage::<Identity, OFFSET>,
            get_PasswordExpirationDate: get_PasswordExpirationDate::<Identity, OFFSET>,
            put_PasswordExpirationDate: put_PasswordExpirationDate::<Identity, OFFSET>,
            get_PasswordMinimumLength: get_PasswordMinimumLength::<Identity, OFFSET>,
            put_PasswordMinimumLength: put_PasswordMinimumLength::<Identity, OFFSET>,
            get_PasswordRequired: get_PasswordRequired::<Identity, OFFSET>,
            put_PasswordRequired: put_PasswordRequired::<Identity, OFFSET>,
            get_RequireUniquePassword: get_RequireUniquePassword::<Identity, OFFSET>,
            put_RequireUniquePassword: put_RequireUniquePassword::<Identity, OFFSET>,
            get_EmailAddress: get_EmailAddress::<Identity, OFFSET>,
            put_EmailAddress: put_EmailAddress::<Identity, OFFSET>,
            get_HomeDirectory: get_HomeDirectory::<Identity, OFFSET>,
            put_HomeDirectory: put_HomeDirectory::<Identity, OFFSET>,
            get_Languages: get_Languages::<Identity, OFFSET>,
            put_Languages: put_Languages::<Identity, OFFSET>,
            get_Profile: get_Profile::<Identity, OFFSET>,
            put_Profile: put_Profile::<Identity, OFFSET>,
            get_LoginScript: get_LoginScript::<Identity, OFFSET>,
            put_LoginScript: put_LoginScript::<Identity, OFFSET>,
            get_Picture: get_Picture::<Identity, OFFSET>,
            put_Picture: put_Picture::<Identity, OFFSET>,
            get_HomePage: get_HomePage::<Identity, OFFSET>,
            put_HomePage: put_HomePage::<Identity, OFFSET>,
            Groups: Groups::<Identity, OFFSET>,
            SetPassword: SetPassword::<Identity, OFFSET>,
            ChangePassword: ChangePassword::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsUser as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IADsUser {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IADsWinNTSystemInfo, IADsWinNTSystemInfo_Vtbl, 0x6c6d65dc_afd1_11d2_9cb9_0000f87a369e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IADsWinNTSystemInfo {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IADsWinNTSystemInfo, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IADsWinNTSystemInfo {
    pub unsafe fn get_UserName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UserName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_ComputerName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ComputerName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_DomainName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DomainName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_PDC(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsWinNTSystemInfo_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ComputerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DomainName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IADsWinNTSystemInfo_Impl: super::IDispatch_Impl {
    fn get_UserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_ComputerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DomainName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_PDC(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IADsWinNTSystemInfo_Vtbl {
    pub const fn new<Identity: IADsWinNTSystemInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_UserName<Identity: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsWinNTSystemInfo_Impl::get_UserName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ComputerName<Identity: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsWinNTSystemInfo_Impl::get_ComputerName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DomainName<Identity: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsWinNTSystemInfo_Impl::get_DomainName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PDC<Identity: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsWinNTSystemInfo_Impl::get_PDC(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_UserName: get_UserName::<Identity, OFFSET>,
            get_ComputerName: get_ComputerName::<Identity, OFFSET>,
            get_DomainName: get_DomainName::<Identity, OFFSET>,
            get_PDC: get_PDC::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsWinNTSystemInfo as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetObjectAttributes(&self, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: super::DWORD, ppattributeentries: *mut PADS_ATTR_INFO, pdwnumattributesreturned: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetObjectAttributes)(windows_core::Interface::as_raw(self), pattributenames, dwnumberattributes, ppattributeentries as _, pdwnumattributesreturned as _) }
    }
    #[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
    pub unsafe fn SetObjectAttributes(&self, pattributeentries: PADS_ATTR_INFO, dwnumattributes: super::DWORD) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetObjectAttributes)(windows_core::Interface::as_raw(self), pattributeentries, dwnumattributes, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwinbase", feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn CreateDSObject<P0>(&self, pszrdnname: P0, pattributeentries: PADS_ATTR_INFO, dwnumattributes: super::DWORD) -> windows_core::Result<super::IDispatch>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDSObject)(windows_core::Interface::as_raw(self), pszrdnname.param().abi(), pattributeentries, dwnumattributes, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    #[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
    pub GetObjectAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, super::DWORD, *mut PADS_ATTR_INFO, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "shared", feature = "winnt")))]
    GetObjectAttributes: usize,
    #[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
    pub SetObjectAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, PADS_ATTR_INFO, super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "shared", feature = "winnt")))]
    SetObjectAttributes: usize,
    #[cfg(all(feature = "minwinbase", feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub CreateDSObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, PADS_ATTR_INFO, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "oaidl", feature = "shared", feature = "winnt")))]
    CreateDSObject: usize,
    pub DeleteDSObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwinbase", feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IDirectoryObject_Impl: windows_core::IUnknownImpl {
    fn GetObjectInformation(&self) -> windows_core::Result<PADS_OBJECT_INFO>;
    fn GetObjectAttributes(&self, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: super::DWORD, ppattributeentries: *mut PADS_ATTR_INFO, pdwnumattributesreturned: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetObjectAttributes(&self, pattributeentries: PADS_ATTR_INFO, dwnumattributes: super::DWORD) -> windows_core::Result<super::DWORD>;
    fn CreateDSObject(&self, pszrdnname: &windows_core::PCWSTR, pattributeentries: PADS_ATTR_INFO, dwnumattributes: super::DWORD) -> windows_core::Result<super::IDispatch>;
    fn DeleteDSObject(&self, pszrdnname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwinbase", feature = "oaidl", feature = "shared", feature = "winnt"))]
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
        unsafe extern "system" fn GetObjectAttributes<Identity: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: super::DWORD, ppattributeentries: *mut PADS_ATTR_INFO, pdwnumattributesreturned: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectoryObject_Impl::GetObjectAttributes(this, core::mem::transmute_copy(&pattributenames), core::mem::transmute_copy(&dwnumberattributes), core::mem::transmute_copy(&ppattributeentries), core::mem::transmute_copy(&pdwnumattributesreturned)).into()
            }
        }
        unsafe extern "system" fn SetObjectAttributes<Identity: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributeentries: PADS_ATTR_INFO, dwnumattributes: super::DWORD, pdwnumattributesmodified: *mut super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn CreateDSObject<Identity: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrdnname: windows_core::PCWSTR, pattributeentries: PADS_ATTR_INFO, dwnumattributes: super::DWORD, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwinbase", feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IDirectoryObject {}
windows_core::imp::define_interface!(IDirectorySchemaMgmt, IDirectorySchemaMgmt_Vtbl, 0x75db3b9c_a4d8_11d0_a79c_00c04fd8d5a8);
windows_core::imp::interface_hierarchy!(IDirectorySchemaMgmt, windows_core::IUnknown);
impl IDirectorySchemaMgmt {
    #[cfg(feature = "shared")]
    pub unsafe fn EnumAttributes(&self, ppszattrnames: *const windows_core::PCWSTR, dwnumattributes: super::DWORD, ppattrdefinition: *const PADS_ATTR_DEF, pdwnumattributes: *const super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumAttributes)(windows_core::Interface::as_raw(self), ppszattrnames, dwnumattributes, ppattrdefinition, pdwnumattributes) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CreateAttributeDefinition<P0>(&self, pszattributename: P0, pattributedefinition: PADS_ATTR_DEF) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateAttributeDefinition)(windows_core::Interface::as_raw(self), pszattributename.param().abi(), pattributedefinition) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn WriteAttributeDefinition<P0>(&self, pszattributename: P0, pattributedefinition: PADS_ATTR_DEF) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteAttributeDefinition)(windows_core::Interface::as_raw(self), pszattributename.param().abi(), pattributedefinition) }
    }
    pub unsafe fn DeleteAttributeDefinition<P0>(&self, pszattributename: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteAttributeDefinition)(windows_core::Interface::as_raw(self), pszattributename.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn EnumClasses(&self, ppszclassnames: *const windows_core::PCWSTR, dwnumclasses: super::DWORD, ppclassdefinition: *const PADS_CLASS_DEF, pdwnumclasses: *const super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumClasses)(windows_core::Interface::as_raw(self), ppszclassnames, dwnumclasses, ppclassdefinition, pdwnumclasses) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn WriteClassDefinition<P0>(&self, pszclassname: P0, pclassdefinition: PADS_CLASS_DEF) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteClassDefinition)(windows_core::Interface::as_raw(self), pszclassname.param().abi(), pclassdefinition) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CreateClassDefinition<P0>(&self, pszclassname: P0, pclassdefinition: PADS_CLASS_DEF) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateClassDefinition)(windows_core::Interface::as_raw(self), pszclassname.param().abi(), pclassdefinition) }
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
    #[cfg(feature = "shared")]
    pub EnumAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, super::DWORD, *const PADS_ATTR_DEF, *const super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EnumAttributes: usize,
    #[cfg(feature = "shared")]
    pub CreateAttributeDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, PADS_ATTR_DEF) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CreateAttributeDefinition: usize,
    #[cfg(feature = "shared")]
    pub WriteAttributeDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, PADS_ATTR_DEF) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    WriteAttributeDefinition: usize,
    pub DeleteAttributeDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub EnumClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, super::DWORD, *const PADS_CLASS_DEF, *const super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EnumClasses: usize,
    #[cfg(feature = "shared")]
    pub WriteClassDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, PADS_CLASS_DEF) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    WriteClassDefinition: usize,
    #[cfg(feature = "shared")]
    pub CreateClassDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, PADS_CLASS_DEF) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CreateClassDefinition: usize,
    pub DeleteClassDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IDirectorySchemaMgmt_Impl: windows_core::IUnknownImpl {
    fn EnumAttributes(&self, ppszattrnames: *const windows_core::PCWSTR, dwnumattributes: super::DWORD, ppattrdefinition: *const PADS_ATTR_DEF, pdwnumattributes: *const super::DWORD) -> windows_core::Result<()>;
    fn CreateAttributeDefinition(&self, pszattributename: &windows_core::PCWSTR, pattributedefinition: PADS_ATTR_DEF) -> windows_core::Result<()>;
    fn WriteAttributeDefinition(&self, pszattributename: &windows_core::PCWSTR, pattributedefinition: PADS_ATTR_DEF) -> windows_core::Result<()>;
    fn DeleteAttributeDefinition(&self, pszattributename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumClasses(&self, ppszclassnames: *const windows_core::PCWSTR, dwnumclasses: super::DWORD, ppclassdefinition: *const PADS_CLASS_DEF, pdwnumclasses: *const super::DWORD) -> windows_core::Result<()>;
    fn WriteClassDefinition(&self, pszclassname: &windows_core::PCWSTR, pclassdefinition: PADS_CLASS_DEF) -> windows_core::Result<()>;
    fn CreateClassDefinition(&self, pszclassname: &windows_core::PCWSTR, pclassdefinition: PADS_CLASS_DEF) -> windows_core::Result<()>;
    fn DeleteClassDefinition(&self, pszclassname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDirectorySchemaMgmt_Vtbl {
    pub const fn new<Identity: IDirectorySchemaMgmt_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumAttributes<Identity: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszattrnames: *const windows_core::PCWSTR, dwnumattributes: super::DWORD, ppattrdefinition: *const PADS_ATTR_DEF, pdwnumattributes: *const super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySchemaMgmt_Impl::EnumAttributes(this, core::mem::transmute_copy(&ppszattrnames), core::mem::transmute_copy(&dwnumattributes), core::mem::transmute_copy(&ppattrdefinition), core::mem::transmute_copy(&pdwnumattributes)).into()
            }
        }
        unsafe extern "system" fn CreateAttributeDefinition<Identity: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszattributename: windows_core::PCWSTR, pattributedefinition: PADS_ATTR_DEF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySchemaMgmt_Impl::CreateAttributeDefinition(this, core::mem::transmute(&pszattributename), core::mem::transmute_copy(&pattributedefinition)).into()
            }
        }
        unsafe extern "system" fn WriteAttributeDefinition<Identity: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszattributename: windows_core::PCWSTR, pattributedefinition: PADS_ATTR_DEF) -> windows_core::HRESULT {
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
        unsafe extern "system" fn EnumClasses<Identity: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszclassnames: *const windows_core::PCWSTR, dwnumclasses: super::DWORD, ppclassdefinition: *const PADS_CLASS_DEF, pdwnumclasses: *const super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySchemaMgmt_Impl::EnumClasses(this, core::mem::transmute_copy(&ppszclassnames), core::mem::transmute_copy(&dwnumclasses), core::mem::transmute_copy(&ppclassdefinition), core::mem::transmute_copy(&pdwnumclasses)).into()
            }
        }
        unsafe extern "system" fn WriteClassDefinition<Identity: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszclassname: windows_core::PCWSTR, pclassdefinition: PADS_CLASS_DEF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySchemaMgmt_Impl::WriteClassDefinition(this, core::mem::transmute(&pszclassname), core::mem::transmute_copy(&pclassdefinition)).into()
            }
        }
        unsafe extern "system" fn CreateClassDefinition<Identity: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszclassname: windows_core::PCWSTR, pclassdefinition: PADS_CLASS_DEF) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDirectorySchemaMgmt {}
windows_core::imp::define_interface!(IDirectorySearch, IDirectorySearch_Vtbl, 0x109ba8ec_92f0_11d0_a790_00c04fd8d5a8);
windows_core::imp::interface_hierarchy!(IDirectorySearch, windows_core::IUnknown);
impl IDirectorySearch {
    #[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
    pub unsafe fn SetSearchPreference(&self, psearchprefs: PADS_SEARCHPREF_INFO, dwnumprefs: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSearchPreference)(windows_core::Interface::as_raw(self), psearchprefs, dwnumprefs) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ExecuteSearch<P0>(&self, pszsearchfilter: P0, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: super::DWORD, phsearchresult: PADS_SEARCH_HANDLE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecuteSearch)(windows_core::Interface::as_raw(self), pszsearchfilter.param().abi(), pattributenames, dwnumberattributes, phsearchresult as _) }
    }
    pub unsafe fn AbandonSearch(&self, phsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AbandonSearch)(windows_core::Interface::as_raw(self), phsearchresult) }
    }
    pub unsafe fn GetFirstRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFirstRow)(windows_core::Interface::as_raw(self), hsearchresult) }
    }
    pub unsafe fn GetNextRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNextRow)(windows_core::Interface::as_raw(self), hsearchresult) }
    }
    pub unsafe fn GetPreviousRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPreviousRow)(windows_core::Interface::as_raw(self), hsearchresult) }
    }
    pub unsafe fn GetNextColumnName(&self, hsearchhandle: ADS_SEARCH_HANDLE) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextColumnName)(windows_core::Interface::as_raw(self), hsearchhandle, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetColumn<P1>(&self, hsearchresult: ADS_SEARCH_HANDLE, szcolumnname: P1, psearchcolumn: PADS_SEARCH_COLUMN) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetColumn)(windows_core::Interface::as_raw(self), hsearchresult, szcolumnname.param().abi(), psearchcolumn as _) }
    }
    #[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
    pub unsafe fn FreeColumn(&self, psearchcolumn: PADS_SEARCH_COLUMN) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeColumn)(windows_core::Interface::as_raw(self), psearchcolumn) }
    }
    pub unsafe fn CloseSearchHandle(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CloseSearchHandle)(windows_core::Interface::as_raw(self), hsearchresult) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectorySearch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
    pub SetSearchPreference: unsafe extern "system" fn(*mut core::ffi::c_void, PADS_SEARCHPREF_INFO, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "shared", feature = "winnt")))]
    SetSearchPreference: usize,
    #[cfg(feature = "shared")]
    pub ExecuteSearch: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::PCWSTR, super::DWORD, PADS_SEARCH_HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ExecuteSearch: usize,
    pub AbandonSearch: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE) -> windows_core::HRESULT,
    pub GetFirstRow: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE) -> windows_core::HRESULT,
    pub GetNextRow: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE) -> windows_core::HRESULT,
    pub GetPreviousRow: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE) -> windows_core::HRESULT,
    pub GetNextColumnName: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
    pub GetColumn: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE, windows_core::PCWSTR, PADS_SEARCH_COLUMN) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "shared", feature = "winnt")))]
    GetColumn: usize,
    #[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
    pub FreeColumn: unsafe extern "system" fn(*mut core::ffi::c_void, PADS_SEARCH_COLUMN) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "shared", feature = "winnt")))]
    FreeColumn: usize,
    pub CloseSearchHandle: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
pub trait IDirectorySearch_Impl: windows_core::IUnknownImpl {
    fn SetSearchPreference(&self, psearchprefs: PADS_SEARCHPREF_INFO, dwnumprefs: super::DWORD) -> windows_core::Result<()>;
    fn ExecuteSearch(&self, pszsearchfilter: &windows_core::PCWSTR, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: super::DWORD, phsearchresult: PADS_SEARCH_HANDLE) -> windows_core::Result<()>;
    fn AbandonSearch(&self, phsearchresult: ADS_SEARCH_HANDLE) -> windows_core::Result<()>;
    fn GetFirstRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::Result<()>;
    fn GetNextRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::Result<()>;
    fn GetPreviousRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::Result<()>;
    fn GetNextColumnName(&self, hsearchhandle: ADS_SEARCH_HANDLE) -> windows_core::Result<windows_core::PWSTR>;
    fn GetColumn(&self, hsearchresult: ADS_SEARCH_HANDLE, szcolumnname: &windows_core::PCWSTR, psearchcolumn: PADS_SEARCH_COLUMN) -> windows_core::Result<()>;
    fn FreeColumn(&self, psearchcolumn: PADS_SEARCH_COLUMN) -> windows_core::Result<()>;
    fn CloseSearchHandle(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
impl IDirectorySearch_Vtbl {
    pub const fn new<Identity: IDirectorySearch_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSearchPreference<Identity: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psearchprefs: PADS_SEARCHPREF_INFO, dwnumprefs: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySearch_Impl::SetSearchPreference(this, core::mem::transmute_copy(&psearchprefs), core::mem::transmute_copy(&dwnumprefs)).into()
            }
        }
        unsafe extern "system" fn ExecuteSearch<Identity: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsearchfilter: windows_core::PCWSTR, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: super::DWORD, phsearchresult: PADS_SEARCH_HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySearch_Impl::ExecuteSearch(this, core::mem::transmute(&pszsearchfilter), core::mem::transmute_copy(&pattributenames), core::mem::transmute_copy(&dwnumberattributes), core::mem::transmute_copy(&phsearchresult)).into()
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
        unsafe extern "system" fn GetColumn<Identity: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE, szcolumnname: windows_core::PCWSTR, psearchcolumn: PADS_SEARCH_COLUMN) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectorySearch_Impl::GetColumn(this, core::mem::transmute_copy(&hsearchresult), core::mem::transmute(&szcolumnname), core::mem::transmute_copy(&psearchcolumn)).into()
            }
        }
        unsafe extern "system" fn FreeColumn<Identity: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psearchcolumn: PADS_SEARCH_COLUMN) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IDirectorySearch {}
windows_core::imp::define_interface!(IPrivateDispatch, IPrivateDispatch_Vtbl, 0x86ab4bbe_65f6_11d1_8c13_00c04fd8d503);
windows_core::imp::interface_hierarchy!(IPrivateDispatch, windows_core::IUnknown);
impl IPrivateDispatch {
    pub unsafe fn ADSIInitializeDispatchManager(&self, dwextensionid: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ADSIInitializeDispatchManager)(windows_core::Interface::as_raw(self), dwextensionid) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ADSIGetTypeInfoCount(&self) -> windows_core::Result<super::UINT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ADSIGetTypeInfoCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared"))]
    pub unsafe fn ADSIGetTypeInfo(&self, itinfo: super::UINT, lcid: super::LCID) -> windows_core::Result<super::ITypeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ADSIGetTypeInfo)(windows_core::Interface::as_raw(self), itinfo, lcid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn ADSIGetIDsOfNames(&self, riid: *const super::IID, rgsznames: *const *const super::OLECHAR, cnames: super::UINT, lcid: super::LCID) -> windows_core::Result<super::DISPID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ADSIGetIDsOfNames)(windows_core::Interface::as_raw(self), riid, rgsznames, cnames, lcid, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ADSIInvoke(&self, dispidmember: super::DISPID, riid: *const super::IID, lcid: super::LCID, wflags: super::WORD, pdispparams: *const super::DISPPARAMS, pvarresult: *mut super::VARIANT, pexcepinfo: *mut super::EXCEPINFO, puargerr: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ADSIInvoke)(windows_core::Interface::as_raw(self), dispidmember, riid, lcid, wflags, pdispparams, pvarresult, pexcepinfo, puargerr as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrivateDispatch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ADSIInitializeDispatchManager: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub ADSIGetTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ADSIGetTypeInfoCount: usize,
    #[cfg(all(feature = "oaidl", feature = "shared"))]
    pub ADSIGetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::LCID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared")))]
    ADSIGetTypeInfo: usize,
    #[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
    pub ADSIGetIDsOfNames: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *const *const super::OLECHAR, super::UINT, super::LCID, *mut super::DISPID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared", feature = "wtypesbase")))]
    ADSIGetIDsOfNames: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub ADSIInvoke: unsafe extern "system" fn(*mut core::ffi::c_void, super::DISPID, *const super::IID, super::LCID, super::WORD, *const super::DISPPARAMS, *mut super::VARIANT, *mut super::EXCEPINFO, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    ADSIInvoke: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPrivateDispatch_Impl: windows_core::IUnknownImpl {
    fn ADSIInitializeDispatchManager(&self, dwextensionid: i32) -> windows_core::Result<()>;
    fn ADSIGetTypeInfoCount(&self) -> windows_core::Result<super::UINT>;
    fn ADSIGetTypeInfo(&self, itinfo: super::UINT, lcid: super::LCID) -> windows_core::Result<super::ITypeInfo>;
    fn ADSIGetIDsOfNames(&self, riid: *const super::IID, rgsznames: *const *const super::OLECHAR, cnames: super::UINT, lcid: super::LCID) -> windows_core::Result<super::DISPID>;
    fn ADSIInvoke(&self, dispidmember: super::DISPID, riid: *const super::IID, lcid: super::LCID, wflags: super::WORD, pdispparams: *const super::DISPPARAMS, pvarresult: *mut super::VARIANT, pexcepinfo: *mut super::EXCEPINFO, puargerr: *mut super::UINT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPrivateDispatch_Vtbl {
    pub const fn new<Identity: IPrivateDispatch_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ADSIInitializeDispatchManager<Identity: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwextensionid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrivateDispatch_Impl::ADSIInitializeDispatchManager(this, core::mem::transmute_copy(&dwextensionid)).into()
            }
        }
        unsafe extern "system" fn ADSIGetTypeInfoCount<Identity: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctinfo: *mut super::UINT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn ADSIGetTypeInfo<Identity: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itinfo: super::UINT, lcid: super::LCID, pptinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn ADSIGetIDsOfNames<Identity: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, rgsznames: *const *const super::OLECHAR, cnames: super::UINT, lcid: super::LCID, rgdispid: *mut super::DISPID) -> windows_core::HRESULT {
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
        unsafe extern "system" fn ADSIInvoke<Identity: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispidmember: super::DISPID, riid: *const super::IID, lcid: super::LCID, wflags: super::WORD, pdispparams: *const super::DISPPARAMS, pvarresult: *mut super::VARIANT, pexcepinfo: *mut super::EXCEPINFO, puargerr: *mut super::UINT) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
pub type LPADSVALUE = *mut ADSVALUE;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
pub type LPADS_SEARCHPREF_INFO = *mut ADS_SEARCHPREF_INFO;
pub type LPNDS_BOOLEAN = *mut u32;
pub const LargeInteger: windows_core::GUID = windows_core::GUID::from_u128(0x927971f5_0939_11d1_8be1_00c04fd8d503);
pub const NameTranslate: windows_core::GUID = windows_core::GUID::from_u128(0x274fae1f_3626_11d1_a3a4_00c04fb950dc);
pub const NetAddress: windows_core::GUID = windows_core::GUID::from_u128(0xb0b71247_4080_11d1_a3ac_00c04fb950dc);
pub const OctetList: windows_core::GUID = windows_core::GUID::from_u128(0x1241400f_4680_11d1_a3b4_00c04fb950dc);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
pub type PADSVALUE = *mut ADSVALUE;
#[cfg(feature = "shared")]
pub type PADS_ATTR_DEF = *mut ADS_ATTR_DEF;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
pub type PADS_ATTR_INFO = *mut ADS_ATTR_INFO;
#[cfg(feature = "shared")]
pub type PADS_BACKLINK = *mut ADS_BACKLINK;
pub type PADS_CASEIGNORE_LIST = *mut ADS_CASEIGNORE_LIST;
pub type PADS_CASE_EXACT_STRING = *mut *mut u16;
pub type PADS_CASE_IGNORE_STRING = *mut *mut u16;
#[cfg(feature = "shared")]
pub type PADS_CLASS_DEF = *mut ADS_CLASS_DEF;
pub type PADS_DN_STRING = *mut *mut u16;
#[cfg(feature = "shared")]
pub type PADS_DN_WITH_BINARY = *mut ADS_DN_WITH_BINARY;
pub type PADS_DN_WITH_STRING = *mut ADS_DN_WITH_STRING;
#[cfg(feature = "shared")]
pub type PADS_EMAIL = *mut ADS_EMAIL;
#[cfg(feature = "shared")]
pub type PADS_FAXNUMBER = *mut ADS_FAXNUMBER;
#[cfg(feature = "shared")]
pub type PADS_HOLD = *mut ADS_HOLD;
pub type PADS_INTEGER = *mut u32;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PADS_LARGE_INTEGER = *mut super::LARGE_INTEGER;
#[cfg(feature = "shared")]
pub type PADS_NETADDRESS = *mut ADS_NETADDRESS;
#[cfg(feature = "shared")]
pub type PADS_NT_SECURITY_DESCRIPTOR = *mut ADS_NT_SECURITY_DESCRIPTOR;
pub type PADS_NUMERIC_STRING = *mut *mut u16;
pub type PADS_OBJECT_CLASS = *mut *mut u16;
pub type PADS_OBJECT_INFO = *mut ADS_OBJECT_INFO;
#[cfg(feature = "shared")]
pub type PADS_OCTET_LIST = *mut ADS_OCTET_LIST;
#[cfg(feature = "shared")]
pub type PADS_OCTET_STRING = *mut ADS_OCTET_STRING;
#[cfg(feature = "shared")]
pub type PADS_PATH = *mut ADS_PATH;
pub type PADS_POSTALADDRESS = *mut ADS_POSTALADDRESS;
pub type PADS_PRINTABLE_STRING = *mut *mut u16;
#[cfg(feature = "shared")]
pub type PADS_PROV_SPECIFIC = *mut ADS_PROV_SPECIFIC;
#[cfg(feature = "shared")]
pub type PADS_REPLICAPOINTER = *mut ADS_REPLICAPOINTER;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
pub type PADS_SEARCHPREF_INFO = *mut ADS_SEARCHPREF_INFO;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
pub type PADS_SEARCH_COLUMN = *mut ADS_SEARCH_COLUMN;
pub type PADS_SEARCH_HANDLE = *mut *mut core::ffi::c_void;
#[cfg(feature = "shared")]
pub type PADS_SORTKEY = *mut ADS_SORTKEY;
pub type PADS_STATUS = *mut ADS_STATUSENUM;
#[cfg(feature = "shared")]
pub type PADS_TIMESTAMP = *mut ADS_TIMESTAMP;
#[cfg(feature = "shared")]
pub type PADS_TYPEDNAME = *mut ADS_TYPEDNAME;
#[cfg(all(feature = "minwinbase", feature = "shared"))]
pub type PADS_UTC_TIME = *mut super::SYSTEMTIME;
#[cfg(feature = "shared")]
pub type PADS_VLV = *mut ADS_VLV;
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
