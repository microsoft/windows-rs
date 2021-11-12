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
    pub const Created: ContactChangeType = ContactChangeType(0i32);
    pub const Modified: ContactChangeType = ContactChangeType(1i32);
    pub const Deleted: ContactChangeType = ContactChangeType(2i32);
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
    pub const SystemDefault: ContactQueryResultOrdering = ContactQueryResultOrdering(0i32);
    pub const GivenNameFamilyName: ContactQueryResultOrdering = ContactQueryResultOrdering(1i32);
    pub const FamilyNameGivenName: ContactQueryResultOrdering = ContactQueryResultOrdering(2i32);
}
#[repr(transparent)]
pub struct ContactStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactStoreApplicationAccessMode(pub i32);
impl ContactStoreApplicationAccessMode {
    pub const LimitedReadOnly: ContactStoreApplicationAccessMode = ContactStoreApplicationAccessMode(0i32);
    pub const ReadOnly: ContactStoreApplicationAccessMode = ContactStoreApplicationAccessMode(1i32);
}
#[repr(transparent)]
pub struct ContactStoreSystemAccessMode(pub i32);
impl ContactStoreSystemAccessMode {
    pub const ReadOnly: ContactStoreSystemAccessMode = ContactStoreSystemAccessMode(0i32);
    pub const ReadWrite: ContactStoreSystemAccessMode = ContactStoreSystemAccessMode(1i32);
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
    pub const Version2_1: VCardFormat = VCardFormat(0i32);
    pub const Version3: VCardFormat = VCardFormat(1i32);
}
