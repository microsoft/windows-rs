pub const HAO_DELETE: HANDLE_ACCESS_OPTIONS = 65536i32;
pub const HAO_NONE: HANDLE_ACCESS_OPTIONS = 0i32;
pub const HAO_READ: HANDLE_ACCESS_OPTIONS = 1179785i32;
pub const HAO_READ_ATTRIBUTES: HANDLE_ACCESS_OPTIONS = 128i32;
pub const HAO_WRITE: HANDLE_ACCESS_OPTIONS = 1179926i32;
pub const HCO_CREATE_ALWAYS: HANDLE_CREATION_OPTIONS = 2i32;
pub const HCO_CREATE_NEW: HANDLE_CREATION_OPTIONS = 1i32;
pub const HCO_OPEN_ALWAYS: HANDLE_CREATION_OPTIONS = 4i32;
pub const HCO_OPEN_EXISTING: HANDLE_CREATION_OPTIONS = 3i32;
pub const HCO_TRUNCATE_EXISTING: HANDLE_CREATION_OPTIONS = 5i32;
pub const HO_DELETE_ON_CLOSE: HANDLE_OPTIONS = 67108864u32;
pub const HO_NONE: HANDLE_OPTIONS = 0u32;
pub const HO_NO_BUFFERING: HANDLE_OPTIONS = 536870912u32;
pub const HO_OPEN_REQUIRING_OPLOCK: HANDLE_OPTIONS = 262144u32;
pub const HO_OVERLAPPED: HANDLE_OPTIONS = 1073741824u32;
pub const HO_RANDOM_ACCESS: HANDLE_OPTIONS = 268435456u32;
pub const HO_SEQUENTIAL_SCAN: HANDLE_OPTIONS = 134217728u32;
pub const HO_WRITE_THROUGH: HANDLE_OPTIONS = 2147483648u32;
pub const HSO_SHARE_DELETE: HANDLE_SHARING_OPTIONS = 4i32;
pub const HSO_SHARE_NONE: HANDLE_SHARING_OPTIONS = 0i32;
pub const HSO_SHARE_READ: HANDLE_SHARING_OPTIONS = 1i32;
pub const HSO_SHARE_WRITE: HANDLE_SHARING_OPTIONS = 2i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HANDLE_ACCESS_OPTIONS(pub i32);
impl windows_core::TypeKind for HANDLE_ACCESS_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HANDLE_CREATION_OPTIONS(pub i32);
impl windows_core::TypeKind for HANDLE_CREATION_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HANDLE_OPTIONS(pub u32);
impl windows_core::TypeKind for HANDLE_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HANDLE_SHARING_OPTIONS(pub i32);
impl windows_core::TypeKind for HANDLE_SHARING_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
