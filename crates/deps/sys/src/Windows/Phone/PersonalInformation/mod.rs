#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Phone_PersonalInformation_Provisioning")]
pub mod Provisioning;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ContactAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactChangeRecord(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ContactChangeType(i32);
#[repr(transparent)]
pub struct ContactInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactQueryResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ContactQueryResultOrdering(i32);
#[repr(transparent)]
pub struct ContactStore(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ContactStoreApplicationAccessMode(i32);
#[repr(C)]
pub struct ContactStoreSystemAccessMode(i32);
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
pub struct KnownContactProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoredContact(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct VCardFormat(i32);
