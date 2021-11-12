#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
pub const CEventClass: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3451832768,
    data2: 31336,
    data3: 4561,
    data4: [136, 249, 0, 128, 199, 215, 113, 191],
};
pub const CEventPublisher: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2878621216,
    data2: 31174,
    data3: 4561,
    data4: [136, 249, 0, 128, 199, 215, 113, 191],
};
pub const CEventSubscription: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1967319392,
    data2: 31175,
    data3: 4561,
    data4: [136, 249, 0, 128, 199, 215, 113, 191],
};
pub const CEventSystem: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1309997986, data2: 11810, data3: 4561, data4: [153, 100, 0, 192, 79, 187, 179, 69] };
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct COMEVENTSYSCHANGEINFO {
    pub cbSize: u32,
    pub changeType: EOC_ChangeType,
    pub objectId: super::super::super::Foundation::BSTR,
    pub partitionId: super::super::super::Foundation::BSTR,
    pub applicationId: super::super::super::Foundation::BSTR,
    pub reserved: [::windows_sys::core::GUID; 10],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMEVENTSYSCHANGEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMEVENTSYSCHANGEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EOC_ChangeType(pub i32);
pub const EOC_NewObject: EOC_ChangeType = EOC_ChangeType(0i32);
pub const EOC_ModifiedObject: EOC_ChangeType = EOC_ChangeType(1i32);
pub const EOC_DeletedObject: EOC_ChangeType = EOC_ChangeType(2i32);
impl ::core::marker::Copy for EOC_ChangeType {}
impl ::core::clone::Clone for EOC_ChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
pub const EventObjectChange: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3495317504, data2: 40436, data3: 4561, data4: [162, 129, 0, 192, 79, 202, 10, 167] };
pub const EventObjectChange2: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3137845965,
    data2: 52566,
    data3: 20067,
    data4: [168, 255, 203, 240, 53, 95, 185, 244],
};
#[repr(transparent)]
pub struct IDontSupportEventSubscription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumEventObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEventClass(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEventClass2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEventControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEventObjectChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEventObjectChange2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEventObjectCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEventProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEventPublisher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEventSubscription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEventSystem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFiringControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultiInterfaceEventControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultiInterfacePublisherFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPublisherFilter(pub *mut ::core::ffi::c_void);
