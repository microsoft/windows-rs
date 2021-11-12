#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPushNotificationChannel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPushNotificationChannel {}
impl ::core::clone::Clone for IPushNotificationChannel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPushNotificationChannelManagerForUser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPushNotificationChannelManagerForUser {}
impl ::core::clone::Clone for IPushNotificationChannelManagerForUser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPushNotificationChannelManagerForUser2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPushNotificationChannelManagerForUser2 {}
impl ::core::clone::Clone for IPushNotificationChannelManagerForUser2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPushNotificationChannelManagerStatics {}
impl ::core::clone::Clone for IPushNotificationChannelManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPushNotificationChannelManagerStatics2 {}
impl ::core::clone::Clone for IPushNotificationChannelManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPushNotificationChannelManagerStatics3 {}
impl ::core::clone::Clone for IPushNotificationChannelManagerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPushNotificationChannelManagerStatics4 {}
impl ::core::clone::Clone for IPushNotificationChannelManagerStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPushNotificationChannelsRevokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPushNotificationChannelsRevokedEventArgs {}
impl ::core::clone::Clone for IPushNotificationChannelsRevokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPushNotificationReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPushNotificationReceivedEventArgs {}
impl ::core::clone::Clone for IPushNotificationReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRawNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRawNotification {}
impl ::core::clone::Clone for IRawNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRawNotification2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRawNotification2 {}
impl ::core::clone::Clone for IRawNotification2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRawNotification3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRawNotification3 {}
impl ::core::clone::Clone for IRawNotification3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PushNotificationChannel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PushNotificationChannel {}
impl ::core::clone::Clone for PushNotificationChannel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PushNotificationChannelManagerForUser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PushNotificationChannelManagerForUser {}
impl ::core::clone::Clone for PushNotificationChannelManagerForUser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PushNotificationChannelsRevokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PushNotificationChannelsRevokedEventArgs {}
impl ::core::clone::Clone for PushNotificationChannelsRevokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PushNotificationReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PushNotificationReceivedEventArgs {}
impl ::core::clone::Clone for PushNotificationReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PushNotificationType(pub i32);
impl PushNotificationType {
    pub const Toast: Self = Self(0i32);
    pub const Tile: Self = Self(1i32);
    pub const Badge: Self = Self(2i32);
    pub const Raw: Self = Self(3i32);
    pub const TileFlyout: Self = Self(4i32);
}
impl ::core::marker::Copy for PushNotificationType {}
impl ::core::clone::Clone for PushNotificationType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RawNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RawNotification {}
impl ::core::clone::Clone for RawNotification {
    fn clone(&self) -> Self {
        *self
    }
}
