#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Phone_PersonalInformation_Provisioning")]
pub mod Provisioning;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ContactAddress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactAddress {}
impl ::core::clone::Clone for ContactAddress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactChangeRecord(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactChangeRecord {}
impl ::core::clone::Clone for ContactChangeRecord {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ContactInformation {}
impl ::core::clone::Clone for ContactInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactQueryOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactQueryOptions {}
impl ::core::clone::Clone for ContactQueryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactQueryResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactQueryResult {}
impl ::core::clone::Clone for ContactQueryResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ContactStore {}
impl ::core::clone::Clone for ContactStore {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for IContactAddress {}
impl ::core::clone::Clone for IContactAddress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactChangeRecord(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactChangeRecord {}
impl ::core::clone::Clone for IContactChangeRecord {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactInformation {}
impl ::core::clone::Clone for IContactInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactInformation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactInformation2 {}
impl ::core::clone::Clone for IContactInformation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactInformationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactInformationStatics {}
impl ::core::clone::Clone for IContactInformationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactQueryOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactQueryOptions {}
impl ::core::clone::Clone for IContactQueryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactQueryResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactQueryResult {}
impl ::core::clone::Clone for IContactQueryResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactStore {}
impl ::core::clone::Clone for IContactStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactStore2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactStore2 {}
impl ::core::clone::Clone for IContactStore2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactStoreStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactStoreStatics {}
impl ::core::clone::Clone for IContactStoreStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownContactPropertiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownContactPropertiesStatics {}
impl ::core::clone::Clone for IKnownContactPropertiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStoredContact(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStoredContact {}
impl ::core::clone::Clone for IStoredContact {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStoredContactFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStoredContactFactory {}
impl ::core::clone::Clone for IStoredContactFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StoredContact(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StoredContact {}
impl ::core::clone::Clone for StoredContact {
    fn clone(&self) -> Self {
        *self
    }
}
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
