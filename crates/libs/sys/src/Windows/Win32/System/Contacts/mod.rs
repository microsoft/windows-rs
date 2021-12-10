#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_System_Contacts'*"]
pub const CGD_ARRAY_NODE: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Contacts'*"]
pub const CGD_BINARY_PROPERTY: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Contacts'*"]
pub const CGD_DATE_PROPERTY: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Contacts'*"]
pub const CGD_DEFAULT: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Contacts'*"]
pub const CGD_STRING_PROPERTY: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Contacts'*"]
pub const CGD_UNKNOWN_PROPERTY: u32 = 0u32;
pub const CLSID_ContactAggregationManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2529734037, data2: 49561, data3: 17630, data4: [179, 78, 172, 51, 196, 66, 223, 57] };
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Contacts'*"]
pub struct CONTACT_AGGREGATION_BLOB {
    pub dwCount: u32,
    pub lpb: *mut u8,
}
impl ::core::marker::Copy for CONTACT_AGGREGATION_BLOB {}
impl ::core::clone::Clone for CONTACT_AGGREGATION_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_Contacts'*"]
pub type CONTACT_AGGREGATION_COLLECTION_OPTIONS = i32;
#[doc = "*Required features: 'Win32_System_Contacts'*"]
pub const CACO_DEFAULT: CONTACT_AGGREGATION_COLLECTION_OPTIONS = 0i32;
#[doc = "*Required features: 'Win32_System_Contacts'*"]
pub const CACO_INCLUDE_EXTERNAL: CONTACT_AGGREGATION_COLLECTION_OPTIONS = 1i32;
#[doc = "*Required features: 'Win32_System_Contacts'*"]
pub const CACO_EXTERNAL_ONLY: CONTACT_AGGREGATION_COLLECTION_OPTIONS = 2i32;
#[doc = "*Required features: 'Win32_System_Contacts'*"]
pub type CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS = i32;
#[doc = "*Required features: 'Win32_System_Contacts'*"]
pub const CA_CREATE_LOCAL: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS = 0i32;
#[doc = "*Required features: 'Win32_System_Contacts'*"]
pub const CA_CREATE_EXTERNAL: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS = 1i32;
pub const Contact: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1639352328, data2: 36590, data3: 20433, data4: [172, 184, 61, 128, 76, 141, 176, 86] };
pub const ContactManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1902495915, data2: 44936, data3: 17085, data4: [134, 253, 83, 16, 180, 40, 90, 2] };
pub type IContact = *mut ::core::ffi::c_void;
pub type IContactAggregationAggregate = *mut ::core::ffi::c_void;
pub type IContactAggregationAggregateCollection = *mut ::core::ffi::c_void;
pub type IContactAggregationContact = *mut ::core::ffi::c_void;
pub type IContactAggregationContactCollection = *mut ::core::ffi::c_void;
pub type IContactAggregationGroup = *mut ::core::ffi::c_void;
pub type IContactAggregationGroupCollection = *mut ::core::ffi::c_void;
pub type IContactAggregationLink = *mut ::core::ffi::c_void;
pub type IContactAggregationLinkCollection = *mut ::core::ffi::c_void;
pub type IContactAggregationManager = *mut ::core::ffi::c_void;
pub type IContactAggregationServerPerson = *mut ::core::ffi::c_void;
pub type IContactAggregationServerPersonCollection = *mut ::core::ffi::c_void;
pub type IContactCollection = *mut ::core::ffi::c_void;
pub type IContactManager = *mut ::core::ffi::c_void;
pub type IContactProperties = *mut ::core::ffi::c_void;
pub type IContactPropertyCollection = *mut ::core::ffi::c_void;
