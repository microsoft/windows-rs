pub trait IDXCoreAdapterImpl: Sized {
    fn IsValid();
    fn IsAttributeSupported();
    fn IsPropertySupported();
    fn GetProperty();
    fn GetPropertySize();
    fn IsQueryStateSupported();
    fn QueryState();
    fn IsSetStateSupported();
    fn SetState();
    fn GetFactory();
}
pub trait IDXCoreAdapterFactoryImpl: Sized {
    fn CreateAdapterList();
    fn GetAdapterByLuid();
    fn IsNotificationTypeSupported();
    fn RegisterEventNotification();
    fn UnregisterEventNotification();
}
pub trait IDXCoreAdapterListImpl: Sized {
    fn GetAdapter();
    fn GetAdapterCount();
    fn IsStale();
    fn GetFactory();
    fn Sort();
    fn IsAdapterPreferenceSupported();
}
