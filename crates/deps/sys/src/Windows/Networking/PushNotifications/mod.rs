#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPushNotificationChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPushNotificationChannelManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPushNotificationChannelManagerForUser2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPushNotificationChannelsRevokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPushNotificationReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRawNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRawNotification2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRawNotification3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PushNotificationChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PushNotificationChannelManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PushNotificationChannelManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PushNotificationChannelsRevokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PushNotificationReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PushNotificationType(i32);
#[repr(transparent)]
pub struct RawNotification(pub *mut ::core::ffi::c_void);
