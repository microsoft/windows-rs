#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[doc = "*Required features: `Win32_System_Contacts`*"]
pub const CGD_ARRAY_NODE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Contacts`*"]
pub const CGD_BINARY_PROPERTY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Contacts`*"]
pub const CGD_DATE_PROPERTY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Contacts`*"]
pub const CGD_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Contacts`*"]
pub const CGD_STRING_PROPERTY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Contacts`*"]
pub const CGD_UNKNOWN_PROPERTY: u32 = 0u32;
pub const CLSID_ContactAggregationManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2529734037, data2: 49561, data3: 17630, data4: [179, 78, 172, 51, 196, 66, 223, 57] };
pub struct CONTACT_AGGREGATION_BLOB(i32);
pub struct CONTACT_AGGREGATION_COLLECTION_OPTIONS(i32);
pub struct CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS(i32);
pub struct Contact(i32);
pub struct ContactManager(i32);
pub struct IContact(i32);
pub struct IContactAggregationAggregate(i32);
pub struct IContactAggregationAggregateCollection(i32);
pub struct IContactAggregationContact(i32);
pub struct IContactAggregationContactCollection(i32);
pub struct IContactAggregationGroup(i32);
pub struct IContactAggregationGroupCollection(i32);
pub struct IContactAggregationLink(i32);
pub struct IContactAggregationLinkCollection(i32);
pub struct IContactAggregationManager(i32);
pub struct IContactAggregationServerPerson(i32);
pub struct IContactAggregationServerPersonCollection(i32);
pub struct IContactCollection(i32);
pub struct IContactManager(i32);
pub struct IContactProperties(i32);
pub struct IContactPropertyCollection(i32);
