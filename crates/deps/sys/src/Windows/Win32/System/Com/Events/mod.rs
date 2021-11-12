#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CEventClass(i32);
pub struct CEventPublisher(i32);
pub struct CEventSubscription(i32);
pub struct CEventSystem(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct COMEVENTSYSCHANGEINFO(i32);
pub struct EOC_ChangeType(i32);
pub struct EventObjectChange(i32);
pub struct EventObjectChange2(i32);
pub struct IDontSupportEventSubscription(pub *mut ::core::ffi::c_void);
pub struct IEnumEventObject(pub *mut ::core::ffi::c_void);
pub struct IEventClass(pub *mut ::core::ffi::c_void);
pub struct IEventClass2(pub *mut ::core::ffi::c_void);
pub struct IEventControl(pub *mut ::core::ffi::c_void);
pub struct IEventObjectChange(pub *mut ::core::ffi::c_void);
pub struct IEventObjectChange2(pub *mut ::core::ffi::c_void);
pub struct IEventObjectCollection(pub *mut ::core::ffi::c_void);
pub struct IEventProperty(pub *mut ::core::ffi::c_void);
pub struct IEventPublisher(pub *mut ::core::ffi::c_void);
pub struct IEventSubscription(pub *mut ::core::ffi::c_void);
pub struct IEventSystem(pub *mut ::core::ffi::c_void);
pub struct IFiringControl(pub *mut ::core::ffi::c_void);
pub struct IMultiInterfaceEventControl(pub *mut ::core::ffi::c_void);
pub struct IMultiInterfacePublisherFilter(pub *mut ::core::ffi::c_void);
pub struct IPublisherFilter(pub *mut ::core::ffi::c_void);
