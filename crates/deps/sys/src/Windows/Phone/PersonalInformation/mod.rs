#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Phone_PersonalInformation_Provisioning")]
pub mod Provisioning;
#[link(name = "windows")]
extern "system" {}
pub struct ContactAddress(i32);
pub struct ContactChangeRecord(i32);
pub struct ContactChangeType(i32);
pub struct ContactInformation(i32);
pub struct ContactQueryOptions(i32);
pub struct ContactQueryResult(i32);
pub struct ContactQueryResultOrdering(i32);
pub struct ContactStore(i32);
pub struct ContactStoreApplicationAccessMode(i32);
pub struct ContactStoreSystemAccessMode(i32);
pub struct IContactAddress(pub *mut ::core::ffi::c_void);
pub struct IContactChangeRecord(pub *mut ::core::ffi::c_void);
pub struct IContactInformation(pub *mut ::core::ffi::c_void);
pub struct IContactInformation2(pub *mut ::core::ffi::c_void);
pub struct IContactInformationStatics(pub *mut ::core::ffi::c_void);
pub struct IContactQueryOptions(pub *mut ::core::ffi::c_void);
pub struct IContactQueryResult(pub *mut ::core::ffi::c_void);
pub struct IContactStore(pub *mut ::core::ffi::c_void);
pub struct IContactStore2(pub *mut ::core::ffi::c_void);
pub struct IContactStoreStatics(pub *mut ::core::ffi::c_void);
pub struct IKnownContactPropertiesStatics(pub *mut ::core::ffi::c_void);
pub struct IStoredContact(pub *mut ::core::ffi::c_void);
pub struct IStoredContactFactory(pub *mut ::core::ffi::c_void);
pub struct KnownContactProperties(i32);
pub struct StoredContact(i32);
pub struct VCardFormat(i32);
