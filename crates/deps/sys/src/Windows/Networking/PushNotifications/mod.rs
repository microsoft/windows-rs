#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IPushNotificationChannel(pub *mut ::core::ffi::c_void);
pub struct IPushNotificationChannelManagerForUser(pub *mut ::core::ffi::c_void);
pub struct IPushNotificationChannelManagerForUser2(pub *mut ::core::ffi::c_void);
pub struct IPushNotificationChannelManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IPushNotificationChannelManagerStatics2(pub *mut ::core::ffi::c_void);
pub struct IPushNotificationChannelManagerStatics3(pub *mut ::core::ffi::c_void);
pub struct IPushNotificationChannelManagerStatics4(pub *mut ::core::ffi::c_void);
pub struct IPushNotificationChannelsRevokedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPushNotificationReceivedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IRawNotification(pub *mut ::core::ffi::c_void);
pub struct IRawNotification2(pub *mut ::core::ffi::c_void);
pub struct IRawNotification3(pub *mut ::core::ffi::c_void);
pub struct PushNotificationChannel(i32);
pub struct PushNotificationChannelManager(i32);
pub struct PushNotificationChannelManagerForUser(i32);
pub struct PushNotificationChannelsRevokedEventArgs(i32);
pub struct PushNotificationReceivedEventArgs(i32);
pub struct PushNotificationType(i32);
pub struct RawNotification(i32);
