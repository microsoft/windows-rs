pub trait ISideShowBulkCapabilitiesImpl: Sized + ISideShowCapabilitiesImpl {
    fn GetCapabilities();
}
pub trait ISideShowCapabilitiesImpl: Sized {
    fn GetCapability();
}
pub trait ISideShowCapabilitiesCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
}
pub trait ISideShowContentImpl: Sized {
    fn GetContent();
    fn ContentId();
    fn DifferentiateContent();
}
pub trait ISideShowContentManagerImpl: Sized {
    fn Add();
    fn Remove();
    fn RemoveAll();
    fn SetEventSink();
    fn GetDeviceCapabilities();
}
pub trait ISideShowEventsImpl: Sized {
    fn ContentMissing();
    fn ApplicationEvent();
    fn DeviceAdded();
    fn DeviceRemoved();
}
pub trait ISideShowKeyCollectionImpl: Sized {
    fn Add();
    fn Clear();
    fn GetAt();
    fn GetCount();
    fn RemoveAt();
}
pub trait ISideShowNotificationImpl: Sized {
    fn NotificationId();
    fn SetNotificationId();
    fn Title();
    fn SetTitle();
    fn Message();
    fn SetMessage();
    fn Image();
    fn SetImage();
    fn ExpirationTime();
    fn SetExpirationTime();
}
pub trait ISideShowNotificationManagerImpl: Sized {
    fn Show();
    fn Revoke();
    fn RevokeAll();
}
pub trait ISideShowPropVariantCollectionImpl: Sized {
    fn Add();
    fn Clear();
    fn GetAt();
    fn GetCount();
    fn RemoveAt();
}
pub trait ISideShowSessionImpl: Sized {
    fn RegisterContent();
    fn RegisterNotifications();
}
