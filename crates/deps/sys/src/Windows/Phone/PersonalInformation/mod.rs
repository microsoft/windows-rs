#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Phone_PersonalInformation_Provisioning")]
pub mod Provisioning;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ContactAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactChangeRecord(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactChangeType(pub i32);
impl ContactChangeType {
    pub const Created: Self = Self(0i32);
    pub const Modified: Self = Self(1i32);
    pub const Deleted: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactChangeType {}
impl ::core::clone::Clone for ContactChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactQueryResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactQueryResultOrdering(pub i32);
impl ContactQueryResultOrdering {
    pub const SystemDefault: Self = Self(0i32);
    pub const GivenNameFamilyName: Self = Self(1i32);
    pub const FamilyNameGivenName: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactQueryResultOrdering {}
impl ::core::clone::Clone for ContactQueryResultOrdering {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactStoreApplicationAccessMode(pub i32);
impl ContactStoreApplicationAccessMode {
    pub const LimitedReadOnly: Self = Self(0i32);
    pub const ReadOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactStoreApplicationAccessMode {}
impl ::core::clone::Clone for ContactStoreApplicationAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactStoreSystemAccessMode(pub i32);
impl ContactStoreSystemAccessMode {
    pub const ReadOnly: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactStoreSystemAccessMode {}
impl ::core::clone::Clone for ContactStoreSystemAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactChangeRecord(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactInformation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactInformationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactQueryResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactStore2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactStoreStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownContactPropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoredContact(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoredContactFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoredContact(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VCardFormat(pub i32);
impl VCardFormat {
    pub const Version2_1: Self = Self(0i32);
    pub const Version3: Self = Self(1i32);
}
impl ::core::marker::Copy for VCardFormat {}
impl ::core::clone::Clone for VCardFormat {
    fn clone(&self) -> Self {
        *self
    }
}
