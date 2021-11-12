#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct INamedPolicyData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INamedPolicyStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NamedPolicyData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NamedPolicyKind(pub i32);
impl NamedPolicyKind {
    pub const Invalid: NamedPolicyKind = NamedPolicyKind(0i32);
    pub const Binary: NamedPolicyKind = NamedPolicyKind(1i32);
    pub const Boolean: NamedPolicyKind = NamedPolicyKind(2i32);
    pub const Int32: NamedPolicyKind = NamedPolicyKind(3i32);
    pub const Int64: NamedPolicyKind = NamedPolicyKind(4i32);
    pub const String: NamedPolicyKind = NamedPolicyKind(5i32);
}
