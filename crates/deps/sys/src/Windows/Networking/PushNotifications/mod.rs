#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
pub struct PushNotificationChannelManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PushNotificationChannelsRevokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PushNotificationReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PushNotificationType(pub i32);
impl PushNotificationType {
    pub const Toast: Self = Self(0i32);
    pub const Tile: Self = Self(1i32);
    pub const Badge: Self = Self(2i32);
    pub const Raw: Self = Self(3i32);
    pub const TileFlyout: Self = Self(4i32);
}
#[repr(transparent)]
pub struct RawNotification(pub *mut ::core::ffi::c_void);
