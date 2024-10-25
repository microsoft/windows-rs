pub const BSF_ALLOWSFW: BROADCAST_SYSTEM_MESSAGE_FLAGS = 128u32;
pub const BSF_FLUSHDISK: BROADCAST_SYSTEM_MESSAGE_FLAGS = 4u32;
pub const BSF_FORCEIFHUNG: BROADCAST_SYSTEM_MESSAGE_FLAGS = 32u32;
pub const BSF_IGNORECURRENTTASK: BROADCAST_SYSTEM_MESSAGE_FLAGS = 2u32;
pub const BSF_LUID: BROADCAST_SYSTEM_MESSAGE_FLAGS = 1024u32;
pub const BSF_NOHANG: BROADCAST_SYSTEM_MESSAGE_FLAGS = 8u32;
pub const BSF_NOTIMEOUTIFNOTHUNG: BROADCAST_SYSTEM_MESSAGE_FLAGS = 64u32;
pub const BSF_POSTMESSAGE: BROADCAST_SYSTEM_MESSAGE_FLAGS = 16u32;
pub const BSF_QUERY: BROADCAST_SYSTEM_MESSAGE_FLAGS = 1u32;
pub const BSF_RETURNHDESK: BROADCAST_SYSTEM_MESSAGE_FLAGS = 512u32;
pub const BSF_SENDNOTIFYMESSAGE: BROADCAST_SYSTEM_MESSAGE_FLAGS = 256u32;
pub const BSM_ALLCOMPONENTS: BROADCAST_SYSTEM_MESSAGE_INFO = 0u32;
pub const BSM_ALLDESKTOPS: BROADCAST_SYSTEM_MESSAGE_INFO = 16u32;
pub const BSM_APPLICATIONS: BROADCAST_SYSTEM_MESSAGE_INFO = 8u32;
pub const DESKTOP_CREATEMENU: DESKTOP_ACCESS_FLAGS = 4u32;
pub const DESKTOP_CREATEWINDOW: DESKTOP_ACCESS_FLAGS = 2u32;
pub const DESKTOP_DELETE: DESKTOP_ACCESS_FLAGS = 65536u32;
pub const DESKTOP_ENUMERATE: DESKTOP_ACCESS_FLAGS = 64u32;
pub const DESKTOP_HOOKCONTROL: DESKTOP_ACCESS_FLAGS = 8u32;
pub const DESKTOP_JOURNALPLAYBACK: DESKTOP_ACCESS_FLAGS = 32u32;
pub const DESKTOP_JOURNALRECORD: DESKTOP_ACCESS_FLAGS = 16u32;
pub const DESKTOP_READOBJECTS: DESKTOP_ACCESS_FLAGS = 1u32;
pub const DESKTOP_READ_CONTROL: DESKTOP_ACCESS_FLAGS = 131072u32;
pub const DESKTOP_SWITCHDESKTOP: DESKTOP_ACCESS_FLAGS = 256u32;
pub const DESKTOP_SYNCHRONIZE: DESKTOP_ACCESS_FLAGS = 1048576u32;
pub const DESKTOP_WRITEOBJECTS: DESKTOP_ACCESS_FLAGS = 128u32;
pub const DESKTOP_WRITE_DAC: DESKTOP_ACCESS_FLAGS = 262144u32;
pub const DESKTOP_WRITE_OWNER: DESKTOP_ACCESS_FLAGS = 524288u32;
pub const DF_ALLOWOTHERACCOUNTHOOK: DESKTOP_CONTROL_FLAGS = 1u32;
pub const UOI_FLAGS: USER_OBJECT_INFORMATION_INDEX = 1i32;
pub const UOI_HEAPSIZE: USER_OBJECT_INFORMATION_INDEX = 5i32;
pub const UOI_IO: USER_OBJECT_INFORMATION_INDEX = 6i32;
pub const UOI_NAME: USER_OBJECT_INFORMATION_INDEX = 2i32;
pub const UOI_TYPE: USER_OBJECT_INFORMATION_INDEX = 3i32;
pub const UOI_USER_SID: USER_OBJECT_INFORMATION_INDEX = 4i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BROADCAST_SYSTEM_MESSAGE_FLAGS(pub u32);
impl windows_core::TypeKind for BROADCAST_SYSTEM_MESSAGE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BROADCAST_SYSTEM_MESSAGE_INFO(pub u32);
impl windows_core::TypeKind for BROADCAST_SYSTEM_MESSAGE_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DESKTOP_ACCESS_FLAGS(pub u32);
impl windows_core::TypeKind for DESKTOP_ACCESS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DESKTOP_CONTROL_FLAGS(pub u32);
impl windows_core::TypeKind for DESKTOP_CONTROL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct USER_OBJECT_INFORMATION_INDEX(pub i32);
impl windows_core::TypeKind for USER_OBJECT_INFORMATION_INDEX {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BSMINFO {
    pub cbSize: u32,
    pub hdesk: HDESK,
    pub hwnd: super::super::Foundation::HWND,
    pub luid: super::super::Foundation::LUID,
}
impl Default for BSMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BSMINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct USEROBJECTFLAGS {
    pub fInherit: super::super::Foundation::BOOL,
    pub fReserved: super::super::Foundation::BOOL,
    pub dwFlags: u32,
}
impl Default for USEROBJECTFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for USEROBJECTFLAGS {
    type TypeKind = windows_core::CloneType;
}
pub type DESKTOPENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
pub type DESKTOPENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
pub type WINSTAENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
pub type WINSTAENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
