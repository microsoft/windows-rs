#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct CEventClass(i32);
#[repr(C)]
pub struct CEventPublisher(i32);
#[repr(C)]
pub struct CEventSubscription(i32);
#[repr(C)]
pub struct CEventSystem(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct COMEVENTSYSCHANGEINFO(i32);
#[repr(transparent)]
pub struct EOC_ChangeType(pub i32);
pub const EOC_NewObject: EOC_ChangeType = EOC_ChangeType(0i32);
pub const EOC_ModifiedObject: EOC_ChangeType = EOC_ChangeType(1i32);
pub const EOC_DeletedObject: EOC_ChangeType = EOC_ChangeType(2i32);
#[repr(C)]
pub struct EventObjectChange(i32);
#[repr(C)]
pub struct EventObjectChange2(i32);
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
