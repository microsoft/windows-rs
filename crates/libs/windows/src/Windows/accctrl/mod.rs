pub const ACCCTRL_DEFAULT_PROVIDERA: windows_core::PCSTR = windows_core::s!("Windows NT Access Provider");
pub const ACCCTRL_DEFAULT_PROVIDERW: windows_core::PCWSTR = windows_core::w!("Windows NT Access Provider");
pub type ACCESS_MODE = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ACCESS_RIGHTS(pub u32);
#[cfg(feature = "winnt")]
pub type ACTRL_ACCESS = ACTRL_ACCESSA;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTRL_ACCESSA {
    pub cEntries: u32,
    pub pPropertyAccessList: PACTRL_PROPERTY_ENTRYA,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTRL_ACCESSW {
    pub cEntries: u32,
    pub pPropertyAccessList: PACTRL_PROPERTY_ENTRYW,
}
pub const ACTRL_ACCESS_ALLOWED: u32 = 1;
pub const ACTRL_ACCESS_DENIED: u32 = 2;
#[cfg(feature = "winnt")]
pub type ACTRL_ACCESS_ENTRY = ACTRL_ACCESS_ENTRYA;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTRL_ACCESS_ENTRYA {
    pub Trustee: TRUSTEE_A,
    pub fAccessFlags: u32,
    pub Access: ACCESS_RIGHTS,
    pub ProvSpecificAccess: ACCESS_RIGHTS,
    pub Inheritance: INHERIT_FLAGS,
    pub lpInheritProperty: windows_core::PSTR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTRL_ACCESS_ENTRYW {
    pub Trustee: TRUSTEE_W,
    pub fAccessFlags: u32,
    pub Access: ACCESS_RIGHTS,
    pub ProvSpecificAccess: ACCESS_RIGHTS,
    pub Inheritance: INHERIT_FLAGS,
    pub lpInheritProperty: windows_core::PWSTR,
}
#[cfg(feature = "winnt")]
pub type ACTRL_ACCESS_ENTRY_LIST = ACTRL_ACCESS_ENTRY_LISTA;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ACTRL_ACCESS_ENTRY_LISTA {
    pub cEntries: u32,
    pub pAccessList: *mut ACTRL_ACCESS_ENTRYA,
}
#[cfg(feature = "winnt")]
impl Default for ACTRL_ACCESS_ENTRY_LISTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ACTRL_ACCESS_ENTRY_LISTW {
    pub cEntries: u32,
    pub pAccessList: *mut ACTRL_ACCESS_ENTRYW,
}
#[cfg(feature = "winnt")]
impl Default for ACTRL_ACCESS_ENTRY_LISTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ACTRL_ACCESS_INFO = ACTRL_ACCESS_INFOA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTRL_ACCESS_INFOA {
    pub fAccessPermission: u32,
    pub lpAccessPermissionName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTRL_ACCESS_INFOW {
    pub fAccessPermission: u32,
    pub lpAccessPermissionName: windows_core::PWSTR,
}
pub const ACTRL_ACCESS_NO_OPTIONS: u32 = 0;
pub const ACTRL_ACCESS_PROTECTED: u32 = 1;
pub const ACTRL_ACCESS_SUPPORTS_OBJECT_ENTRIES: u32 = 1;
#[cfg(feature = "winnt")]
pub type ACTRL_AUDIT = ACTRL_AUDITA;
#[cfg(feature = "winnt")]
pub type ACTRL_AUDITA = ACTRL_ACCESSA;
#[cfg(feature = "winnt")]
pub type ACTRL_AUDITW = ACTRL_ACCESSW;
pub const ACTRL_AUDIT_FAILURE: u32 = 8;
pub const ACTRL_AUDIT_SUCCESS: u32 = 4;
pub const ACTRL_CHANGE_ACCESS: u32 = 536870912;
pub const ACTRL_CHANGE_OWNER: u32 = 1073741824;
pub type ACTRL_CONTROL_INFO = ACTRL_CONTROL_INFOA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTRL_CONTROL_INFOA {
    pub lpControlId: windows_core::PSTR,
    pub lpControlName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTRL_CONTROL_INFOW {
    pub lpControlId: windows_core::PWSTR,
    pub lpControlName: windows_core::PWSTR,
}
pub const ACTRL_DELETE: u32 = 134217728;
pub const ACTRL_DIR_CREATE_CHILD: u32 = 4;
pub const ACTRL_DIR_CREATE_OBJECT: u32 = 2;
pub const ACTRL_DIR_DELETE_CHILD: u32 = 64;
pub const ACTRL_DIR_LIST: u32 = 1;
pub const ACTRL_DIR_TRAVERSE: u32 = 32;
pub const ACTRL_DS_CONTROL_ACCESS: u32 = 256;
pub const ACTRL_DS_CREATE_CHILD: u32 = 1;
pub const ACTRL_DS_DELETE_CHILD: u32 = 2;
pub const ACTRL_DS_DELETE_TREE: u32 = 64;
pub const ACTRL_DS_LIST: u32 = 4;
pub const ACTRL_DS_LIST_OBJECT: u32 = 128;
pub const ACTRL_DS_OPEN: u32 = 0;
pub const ACTRL_DS_READ_PROP: u32 = 16;
pub const ACTRL_DS_SELF: u32 = 8;
pub const ACTRL_DS_WRITE_PROP: u32 = 32;
pub const ACTRL_FILE_APPEND: u32 = 4;
pub const ACTRL_FILE_CREATE_PIPE: u32 = 512;
pub const ACTRL_FILE_EXECUTE: u32 = 32;
pub const ACTRL_FILE_READ: u32 = 1;
pub const ACTRL_FILE_READ_ATTRIB: u32 = 128;
pub const ACTRL_FILE_READ_PROP: u32 = 8;
pub const ACTRL_FILE_WRITE: u32 = 2;
pub const ACTRL_FILE_WRITE_ATTRIB: u32 = 256;
pub const ACTRL_FILE_WRITE_PROP: u32 = 16;
pub const ACTRL_KERNEL_ALERT: u32 = 1024;
pub const ACTRL_KERNEL_CONTROL: u32 = 512;
pub const ACTRL_KERNEL_DIMPERSONATE: u32 = 32768;
pub const ACTRL_KERNEL_DUP_HANDLE: u32 = 32;
pub const ACTRL_KERNEL_GET_CONTEXT: u32 = 2048;
pub const ACTRL_KERNEL_GET_INFO: u32 = 256;
pub const ACTRL_KERNEL_IMPERSONATE: u32 = 16384;
pub const ACTRL_KERNEL_PROCESS: u32 = 64;
pub const ACTRL_KERNEL_SET_CONTEXT: u32 = 4096;
pub const ACTRL_KERNEL_SET_INFO: u32 = 128;
pub const ACTRL_KERNEL_TERMINATE: u32 = 1;
pub const ACTRL_KERNEL_THREAD: u32 = 2;
pub const ACTRL_KERNEL_TOKEN: u32 = 8192;
pub const ACTRL_KERNEL_VM: u32 = 4;
pub const ACTRL_KERNEL_VM_READ: u32 = 8;
pub const ACTRL_KERNEL_VM_WRITE: u32 = 16;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct ACTRL_OVERLAPPED {
    pub Anonymous: ACTRL_OVERLAPPED_0,
    pub Reserved2: u32,
    pub hEvent: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for ACTRL_OVERLAPPED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union ACTRL_OVERLAPPED_0 {
    pub Provider: *mut core::ffi::c_void,
    pub Reserved1: u32,
}
#[cfg(feature = "winnt")]
impl Default for ACTRL_OVERLAPPED_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACTRL_PERM_1: u32 = 1;
pub const ACTRL_PERM_10: u32 = 512;
pub const ACTRL_PERM_11: u32 = 1024;
pub const ACTRL_PERM_12: u32 = 2048;
pub const ACTRL_PERM_13: u32 = 4096;
pub const ACTRL_PERM_14: u32 = 8192;
pub const ACTRL_PERM_15: u32 = 16384;
pub const ACTRL_PERM_16: u32 = 32768;
pub const ACTRL_PERM_17: u32 = 65536;
pub const ACTRL_PERM_18: u32 = 131072;
pub const ACTRL_PERM_19: u32 = 262144;
pub const ACTRL_PERM_2: u32 = 2;
pub const ACTRL_PERM_20: u32 = 524288;
pub const ACTRL_PERM_3: u32 = 4;
pub const ACTRL_PERM_4: u32 = 8;
pub const ACTRL_PERM_5: u32 = 16;
pub const ACTRL_PERM_6: u32 = 32;
pub const ACTRL_PERM_7: u32 = 64;
pub const ACTRL_PERM_8: u32 = 128;
pub const ACTRL_PERM_9: u32 = 256;
pub const ACTRL_PRINT_JADMIN: u32 = 16;
pub const ACTRL_PRINT_PADMIN: u32 = 4;
pub const ACTRL_PRINT_PUSE: u32 = 8;
pub const ACTRL_PRINT_SADMIN: u32 = 1;
pub const ACTRL_PRINT_SLIST: u32 = 2;
#[cfg(feature = "winnt")]
pub type ACTRL_PROPERTY_ENTRY = ACTRL_PROPERTY_ENTRYA;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTRL_PROPERTY_ENTRYA {
    pub lpProperty: windows_core::PSTR,
    pub pAccessEntryList: PACTRL_ACCESS_ENTRY_LISTA,
    pub fListFlags: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTRL_PROPERTY_ENTRYW {
    pub lpProperty: windows_core::PWSTR,
    pub pAccessEntryList: PACTRL_ACCESS_ENTRY_LISTW,
    pub fListFlags: u32,
}
pub const ACTRL_READ_CONTROL: u32 = 268435456;
pub const ACTRL_REG_CREATE_CHILD: u32 = 4;
pub const ACTRL_REG_LINK: u32 = 32;
pub const ACTRL_REG_LIST: u32 = 8;
pub const ACTRL_REG_NOTIFY: u32 = 16;
pub const ACTRL_REG_QUERY: u32 = 1;
pub const ACTRL_REG_SET: u32 = 2;
pub const ACTRL_RESERVED: u32 = 0;
pub const ACTRL_STD_RIGHTS_ALL: u32 = 4160749568;
pub const ACTRL_STD_RIGHT_REQUIRED: u32 = 2013265920;
pub const ACTRL_SVC_GET_INFO: u32 = 1;
pub const ACTRL_SVC_INTERROGATE: u32 = 128;
pub const ACTRL_SVC_LIST: u32 = 8;
pub const ACTRL_SVC_PAUSE: u32 = 64;
pub const ACTRL_SVC_SET_INFO: u32 = 2;
pub const ACTRL_SVC_START: u32 = 16;
pub const ACTRL_SVC_STATUS: u32 = 4;
pub const ACTRL_SVC_STOP: u32 = 32;
pub const ACTRL_SVC_UCONTROL: u32 = 256;
pub const ACTRL_SYNCHRONIZE: u32 = 2147483648;
pub const ACTRL_SYSTEM_ACCESS: u32 = 67108864;
pub const ACTRL_WIN_CLIPBRD: u32 = 1;
pub const ACTRL_WIN_CREATE: u32 = 4;
pub const ACTRL_WIN_EXIT: u32 = 256;
pub const ACTRL_WIN_GLOBAL_ATOMS: u32 = 2;
pub const ACTRL_WIN_LIST: u32 = 16;
pub const ACTRL_WIN_LIST_DESK: u32 = 8;
pub const ACTRL_WIN_READ_ATTRIBS: u32 = 32;
pub const ACTRL_WIN_SCREEN: u32 = 128;
pub const ACTRL_WIN_WRITE_ATTRIBS: u32 = 64;
pub const DENY_ACCESS: ACCESS_MODE = 3;
#[cfg(feature = "winnt")]
pub type EXPLICIT_ACCESS = EXPLICIT_ACCESSA;
#[cfg(feature = "winnt")]
pub type EXPLICIT_ACCESSA = EXPLICIT_ACCESS_A;
#[cfg(feature = "winnt")]
pub type EXPLICIT_ACCESSW = EXPLICIT_ACCESS_W;
#[cfg(feature = "winnt")]
pub type EXPLICIT_ACCESS_ = EXPLICIT_ACCESS_A;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXPLICIT_ACCESS_A {
    pub grfAccessPermissions: u32,
    pub grfAccessMode: ACCESS_MODE,
    pub grfInheritance: u32,
    pub Trustee: TRUSTEE_A,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXPLICIT_ACCESS_W {
    pub grfAccessPermissions: u32,
    pub grfAccessMode: ACCESS_MODE,
    pub grfInheritance: u32,
    pub Trustee: TRUSTEE_W,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FN_OBJECT_MGR_FUNCTS {
    pub Placeholder: u32,
}
pub const GRANT_ACCESS: ACCESS_MODE = 1;
pub const INHERITED_ACCESS_ENTRY: u32 = 16;
pub type INHERITED_FROM = INHERITED_FROMA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INHERITED_FROMA {
    pub GenerationGap: i32,
    pub AncestorName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INHERITED_FROMW {
    pub GenerationGap: i32,
    pub AncestorName: windows_core::PWSTR,
}
pub const INHERITED_GRANDPARENT: u32 = 536870912;
pub const INHERITED_PARENT: u32 = 268435456;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct INHERIT_FLAGS(pub u32);
pub const INHERIT_NO_PROPAGATE: u32 = 4;
pub const INHERIT_ONLY: u32 = 8;
pub type MULTIPLE_TRUSTEE_OPERATION = i32;
pub const NOT_USED_ACCESS: ACCESS_MODE = 0;
pub const NO_INHERITANCE: u32 = 0;
pub const NO_MULTIPLE_TRUSTEE: MULTIPLE_TRUSTEE_OPERATION = 0;
pub type OBJECTS_AND_NAME_ = OBJECTS_AND_NAME_A;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OBJECTS_AND_NAME_A {
    pub ObjectsPresent: u32,
    pub ObjectType: SE_OBJECT_TYPE,
    pub ObjectTypeName: windows_core::PSTR,
    pub InheritedObjectTypeName: windows_core::PSTR,
    pub ptstrName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OBJECTS_AND_NAME_W {
    pub ObjectsPresent: u32,
    pub ObjectType: SE_OBJECT_TYPE,
    pub ObjectTypeName: windows_core::PWSTR,
    pub InheritedObjectTypeName: windows_core::PWSTR,
    pub ptstrName: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OBJECTS_AND_SID {
    pub ObjectsPresent: u32,
    pub ObjectTypeGuid: windows_core::GUID,
    pub InheritedObjectTypeGuid: windows_core::GUID,
    pub pSid: *mut super::winnt::SID,
}
#[cfg(feature = "winnt")]
impl Default for OBJECTS_AND_SID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PACCESS_RIGHTS = *mut u32;
#[cfg(feature = "winnt")]
pub type PACTRL_ACCESS = PACTRL_ACCESSA;
#[cfg(feature = "winnt")]
pub type PACTRL_ACCESSA = *mut ACTRL_ACCESSA;
#[cfg(feature = "winnt")]
pub type PACTRL_ACCESSW = *mut ACTRL_ACCESSW;
#[cfg(feature = "winnt")]
pub type PACTRL_ACCESS_ENTRY = PACTRL_ACCESS_ENTRYA;
#[cfg(feature = "winnt")]
pub type PACTRL_ACCESS_ENTRYA = *mut ACTRL_ACCESS_ENTRYA;
#[cfg(feature = "winnt")]
pub type PACTRL_ACCESS_ENTRYW = *mut ACTRL_ACCESS_ENTRYW;
#[cfg(feature = "winnt")]
pub type PACTRL_ACCESS_ENTRY_LIST = PACTRL_ACCESS_ENTRY_LISTA;
#[cfg(feature = "winnt")]
pub type PACTRL_ACCESS_ENTRY_LISTA = *mut ACTRL_ACCESS_ENTRY_LISTA;
#[cfg(feature = "winnt")]
pub type PACTRL_ACCESS_ENTRY_LISTW = *mut ACTRL_ACCESS_ENTRY_LISTW;
pub type PACTRL_ACCESS_INFO = PACTRL_ACCESS_INFOA;
pub type PACTRL_ACCESS_INFOA = *mut ACTRL_ACCESS_INFOA;
pub type PACTRL_ACCESS_INFOW = *mut ACTRL_ACCESS_INFOW;
#[cfg(feature = "winnt")]
pub type PACTRL_AUDIT = PACTRL_AUDITA;
#[cfg(feature = "winnt")]
pub type PACTRL_AUDITA = *mut ACTRL_ACCESSA;
#[cfg(feature = "winnt")]
pub type PACTRL_AUDITW = *mut ACTRL_ACCESSW;
pub type PACTRL_CONTROL_INFO = PACTRL_CONTROL_INFOA;
pub type PACTRL_CONTROL_INFOA = *mut ACTRL_CONTROL_INFOA;
pub type PACTRL_CONTROL_INFOW = *mut ACTRL_CONTROL_INFOW;
#[cfg(feature = "winnt")]
pub type PACTRL_OVERLAPPED = *mut ACTRL_OVERLAPPED;
#[cfg(feature = "winnt")]
pub type PACTRL_PROPERTY_ENTRY = PACTRL_PROPERTY_ENTRYA;
#[cfg(feature = "winnt")]
pub type PACTRL_PROPERTY_ENTRYA = *mut ACTRL_PROPERTY_ENTRYA;
#[cfg(feature = "winnt")]
pub type PACTRL_PROPERTY_ENTRYW = *mut ACTRL_PROPERTY_ENTRYW;
#[cfg(feature = "winnt")]
pub type PEXPLICIT_ACCESS = PEXPLICIT_ACCESSA;
#[cfg(feature = "winnt")]
pub type PEXPLICIT_ACCESSA = *mut EXPLICIT_ACCESS_A;
#[cfg(feature = "winnt")]
pub type PEXPLICIT_ACCESSW = *mut EXPLICIT_ACCESS_W;
#[cfg(feature = "winnt")]
pub type PEXPLICIT_ACCESS_ = PEXPLICIT_ACCESS_A;
#[cfg(feature = "winnt")]
pub type PEXPLICIT_ACCESS_A = *mut EXPLICIT_ACCESS_A;
#[cfg(feature = "winnt")]
pub type PEXPLICIT_ACCESS_W = *mut EXPLICIT_ACCESS_W;
pub type PFN_OBJECT_MGR_FUNCTS = *mut FN_OBJECT_MGR_FUNCTS;
pub type PINHERITED_FROM = PINHERITED_FROMA;
pub type PINHERITED_FROMA = *mut INHERITED_FROMA;
pub type PINHERITED_FROMW = *mut INHERITED_FROMW;
pub type PINHERIT_FLAGS = *mut u32;
pub type POBJECTS_AND_NAME_ = POBJECTS_AND_NAME_A;
pub type POBJECTS_AND_NAME_A = *mut OBJECTS_AND_NAME_A;
pub type POBJECTS_AND_NAME_W = *mut OBJECTS_AND_NAME_W;
#[cfg(feature = "winnt")]
pub type POBJECTS_AND_SID = *mut OBJECTS_AND_SID;
pub type PPROG_INVOKE_SETTING = *mut PROG_INVOKE_SETTING;
pub type PROG_INVOKE_SETTING = i32;
#[cfg(feature = "winnt")]
pub type PTRUSTEE = PTRUSTEEA;
#[cfg(feature = "winnt")]
pub type PTRUSTEEA = *mut TRUSTEE_A;
#[cfg(feature = "winnt")]
pub type PTRUSTEEW = *mut TRUSTEE_W;
#[cfg(feature = "winnt")]
pub type PTRUSTEE_ = PTRUSTEE_A;
#[cfg(feature = "winnt")]
pub type PTRUSTEE_A = *mut TRUSTEE_A;
pub type PTRUSTEE_ACCESS = PTRUSTEE_ACCESSA;
pub type PTRUSTEE_ACCESSA = *mut TRUSTEE_ACCESSA;
pub type PTRUSTEE_ACCESSW = *mut TRUSTEE_ACCESSW;
#[cfg(feature = "winnt")]
pub type PTRUSTEE_W = *mut TRUSTEE_W;
pub const ProgressCancelOperation: PROG_INVOKE_SETTING = 4;
pub const ProgressInvokeEveryObject: PROG_INVOKE_SETTING = 2;
pub const ProgressInvokeNever: PROG_INVOKE_SETTING = 1;
pub const ProgressInvokeOnError: PROG_INVOKE_SETTING = 3;
pub const ProgressInvokePrePostError: PROG_INVOKE_SETTING = 6;
pub const ProgressRetryOperation: PROG_INVOKE_SETTING = 5;
pub const REVOKE_ACCESS: ACCESS_MODE = 4;
pub const SET_ACCESS: ACCESS_MODE = 2;
pub const SET_AUDIT_FAILURE: ACCESS_MODE = 6;
pub const SET_AUDIT_SUCCESS: ACCESS_MODE = 5;
pub const SE_DS_OBJECT: SE_OBJECT_TYPE = 8;
pub const SE_DS_OBJECT_ALL: SE_OBJECT_TYPE = 9;
pub const SE_FILE_OBJECT: SE_OBJECT_TYPE = 1;
pub const SE_KERNEL_OBJECT: SE_OBJECT_TYPE = 6;
pub const SE_LMSHARE: SE_OBJECT_TYPE = 5;
pub type SE_OBJECT_TYPE = i32;
pub const SE_PRINTER: SE_OBJECT_TYPE = 3;
pub const SE_PROVIDER_DEFINED_OBJECT: SE_OBJECT_TYPE = 10;
pub const SE_REGISTRY_KEY: SE_OBJECT_TYPE = 4;
pub const SE_REGISTRY_WOW64_32KEY: SE_OBJECT_TYPE = 12;
pub const SE_REGISTRY_WOW64_64KEY: SE_OBJECT_TYPE = 13;
pub const SE_SERVICE: SE_OBJECT_TYPE = 2;
pub const SE_UNKNOWN_OBJECT_TYPE: SE_OBJECT_TYPE = 0;
pub const SE_WINDOW_OBJECT: SE_OBJECT_TYPE = 7;
pub const SE_WMIGUID_OBJECT: SE_OBJECT_TYPE = 11;
pub const SUB_CONTAINERS_AND_OBJECTS_INHERIT: u32 = 3;
pub const SUB_CONTAINERS_ONLY_INHERIT: u32 = 2;
pub const SUB_OBJECTS_ONLY_INHERIT: u32 = 1;
pub const TREE_SEC_INFO_RESET: u32 = 2;
pub const TREE_SEC_INFO_RESET_KEEP_EXPLICIT: u32 = 3;
pub const TREE_SEC_INFO_SET: u32 = 1;
#[cfg(feature = "winnt")]
pub type TRUSTEE = TRUSTEEA;
#[cfg(feature = "winnt")]
pub type TRUSTEEA = TRUSTEE_A;
#[cfg(feature = "winnt")]
pub type TRUSTEEW = TRUSTEE_W;
#[cfg(feature = "winnt")]
pub type TRUSTEE_ = TRUSTEE_A;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRUSTEE_A {
    pub pMultipleTrustee: *mut Self,
    pub MultipleTrusteeOperation: MULTIPLE_TRUSTEE_OPERATION,
    pub TrusteeForm: TRUSTEE_FORM,
    pub TrusteeType: TRUSTEE_TYPE,
    pub ptstrName: super::winnt::LPCH,
}
#[cfg(feature = "winnt")]
impl Default for TRUSTEE_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TRUSTEE_ACCESS = TRUSTEE_ACCESSA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRUSTEE_ACCESSA {
    pub lpProperty: windows_core::PSTR,
    pub Access: ACCESS_RIGHTS,
    pub fAccessFlags: u32,
    pub fReturnedAccess: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRUSTEE_ACCESSW {
    pub lpProperty: windows_core::PWSTR,
    pub Access: ACCESS_RIGHTS,
    pub fAccessFlags: u32,
    pub fReturnedAccess: u32,
}
pub const TRUSTEE_ACCESS_ALL: u32 = 4294967295;
pub const TRUSTEE_ACCESS_ALLOWED: u32 = 1;
pub const TRUSTEE_ACCESS_EXPLICIT: u32 = 1;
pub const TRUSTEE_ACCESS_READ: u32 = 2;
pub const TRUSTEE_ACCESS_READ_WRITE: u32 = 6;
pub const TRUSTEE_ACCESS_WRITE: u32 = 4;
pub const TRUSTEE_BAD_FORM: TRUSTEE_FORM = 2;
pub type TRUSTEE_FORM = i32;
pub const TRUSTEE_IS_ALIAS: TRUSTEE_TYPE = 4;
pub const TRUSTEE_IS_COMPUTER: TRUSTEE_TYPE = 8;
pub const TRUSTEE_IS_DELETED: TRUSTEE_TYPE = 6;
pub const TRUSTEE_IS_DOMAIN: TRUSTEE_TYPE = 3;
pub const TRUSTEE_IS_GROUP: TRUSTEE_TYPE = 2;
pub const TRUSTEE_IS_IMPERSONATE: MULTIPLE_TRUSTEE_OPERATION = 1;
pub const TRUSTEE_IS_INVALID: TRUSTEE_TYPE = 7;
pub const TRUSTEE_IS_NAME: TRUSTEE_FORM = 1;
pub const TRUSTEE_IS_OBJECTS_AND_NAME: TRUSTEE_FORM = 4;
pub const TRUSTEE_IS_OBJECTS_AND_SID: TRUSTEE_FORM = 3;
pub const TRUSTEE_IS_SID: TRUSTEE_FORM = 0;
pub const TRUSTEE_IS_UNKNOWN: TRUSTEE_TYPE = 0;
pub const TRUSTEE_IS_USER: TRUSTEE_TYPE = 1;
pub const TRUSTEE_IS_WELL_KNOWN_GROUP: TRUSTEE_TYPE = 5;
pub type TRUSTEE_TYPE = i32;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRUSTEE_W {
    pub pMultipleTrustee: *mut Self,
    pub MultipleTrusteeOperation: MULTIPLE_TRUSTEE_OPERATION,
    pub TrusteeForm: TRUSTEE_FORM,
    pub TrusteeType: TRUSTEE_TYPE,
    pub ptstrName: super::winnt::LPWCH,
}
#[cfg(feature = "winnt")]
impl Default for TRUSTEE_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
