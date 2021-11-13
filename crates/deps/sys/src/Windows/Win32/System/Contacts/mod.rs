#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const CGD_ARRAY_NODE: u32 = 8u32;
pub const CGD_BINARY_PROPERTY: u32 = 4u32;
pub const CGD_DATE_PROPERTY: u32 = 2u32;
pub const CGD_DEFAULT: u32 = 0u32;
pub const CGD_STRING_PROPERTY: u32 = 1u32;
pub const CGD_UNKNOWN_PROPERTY: u32 = 0u32;
pub const CLSID_ContactAggregationManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2529734037, data2: 49561, data3: 17630, data4: [179, 78, 172, 51, 196, 66, 223, 57] };
#[repr(C)]
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
#[repr(transparent)]
pub struct CONTACT_AGGREGATION_COLLECTION_OPTIONS(pub i32);
pub const CACO_DEFAULT: CONTACT_AGGREGATION_COLLECTION_OPTIONS = CONTACT_AGGREGATION_COLLECTION_OPTIONS(0i32);
pub const CACO_INCLUDE_EXTERNAL: CONTACT_AGGREGATION_COLLECTION_OPTIONS = CONTACT_AGGREGATION_COLLECTION_OPTIONS(1i32);
pub const CACO_EXTERNAL_ONLY: CONTACT_AGGREGATION_COLLECTION_OPTIONS = CONTACT_AGGREGATION_COLLECTION_OPTIONS(2i32);
impl ::core::marker::Copy for CONTACT_AGGREGATION_COLLECTION_OPTIONS {}
impl ::core::clone::Clone for CONTACT_AGGREGATION_COLLECTION_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS(pub i32);
pub const CA_CREATE_LOCAL: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS = CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS(0i32);
pub const CA_CREATE_EXTERNAL: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS = CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS(1i32);
impl ::core::marker::Copy for CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS {}
impl ::core::clone::Clone for CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const Contact: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1639352328,
    data2: 36590,
    data3: 20433,
    data4: [172, 184, 61, 128, 76, 141, 176, 86],
};
pub const ContactManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1902495915, data2: 44936, data3: 17085, data4: [134, 253, 83, 16, 180, 40, 90, 2] };
#[repr(transparent)]
pub struct IContact(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContact {}
impl ::core::clone::Clone for IContact {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAggregationAggregate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactAggregationAggregate {}
impl ::core::clone::Clone for IContactAggregationAggregate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAggregationAggregateCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactAggregationAggregateCollection {}
impl ::core::clone::Clone for IContactAggregationAggregateCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAggregationContact(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactAggregationContact {}
impl ::core::clone::Clone for IContactAggregationContact {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAggregationContactCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactAggregationContactCollection {}
impl ::core::clone::Clone for IContactAggregationContactCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAggregationGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactAggregationGroup {}
impl ::core::clone::Clone for IContactAggregationGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAggregationGroupCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactAggregationGroupCollection {}
impl ::core::clone::Clone for IContactAggregationGroupCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAggregationLink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactAggregationLink {}
impl ::core::clone::Clone for IContactAggregationLink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAggregationLinkCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactAggregationLinkCollection {}
impl ::core::clone::Clone for IContactAggregationLinkCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAggregationManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactAggregationManager {}
impl ::core::clone::Clone for IContactAggregationManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAggregationServerPerson(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactAggregationServerPerson {}
impl ::core::clone::Clone for IContactAggregationServerPerson {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAggregationServerPersonCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactAggregationServerPersonCollection {}
impl ::core::clone::Clone for IContactAggregationServerPersonCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactCollection {}
impl ::core::clone::Clone for IContactCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactManager {}
impl ::core::clone::Clone for IContactManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactProperties {}
impl ::core::clone::Clone for IContactProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactPropertyCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactPropertyCollection {}
impl ::core::clone::Clone for IContactPropertyCollection {
    fn clone(&self) -> Self {
        *self
    }
}
