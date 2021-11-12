#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub const CGD_ARRAY_NODE: u32 = 8u32;
pub const CGD_BINARY_PROPERTY: u32 = 4u32;
pub const CGD_DATE_PROPERTY: u32 = 2u32;
pub const CGD_DEFAULT: u32 = 0u32;
pub const CGD_STRING_PROPERTY: u32 = 1u32;
pub const CGD_UNKNOWN_PROPERTY: u32 = 0u32;
pub const CLSID_ContactAggregationManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2529734037, data2: 49561, data3: 17630, data4: [179, 78, 172, 51, 196, 66, 223, 57] };
#[repr(C)]
pub struct CONTACT_AGGREGATION_BLOB(i32);
#[repr(C)]
pub struct CONTACT_AGGREGATION_COLLECTION_OPTIONS(i32);
#[repr(C)]
pub struct CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS(i32);
#[repr(C)]
pub struct Contact(i32);
#[repr(C)]
pub struct ContactManager(i32);
#[repr(transparent)]
pub struct IContact(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactAggregationAggregate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactAggregationAggregateCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactAggregationContact(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactAggregationContactCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactAggregationGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactAggregationGroupCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactAggregationLink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactAggregationLinkCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactAggregationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactAggregationServerPerson(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactAggregationServerPersonCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactPropertyCollection(pub *mut ::core::ffi::c_void);
