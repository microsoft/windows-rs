#[cfg(feature = "implement_exclusive")]
pub trait IUserNotificationListenerImpl: Sized {
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UserNotificationListenerAccessStatus>>;
    fn GetAccessStatus(&self) -> ::windows::core::Result<UserNotificationListenerAccessStatus>;
    fn NotificationChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<UserNotificationListener, super::UserNotificationChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNotificationChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetNotificationsAsync(&self, kinds: super::NotificationKinds) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<super::UserNotification>>>;
    fn GetNotification(&self, notificationid: u32) -> ::windows::core::Result<super::UserNotification>;
    fn ClearNotifications(&self) -> ::windows::core::Result<()>;
    fn RemoveNotification(&self, notificationid: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserNotificationListenerStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<UserNotificationListener>;
}
