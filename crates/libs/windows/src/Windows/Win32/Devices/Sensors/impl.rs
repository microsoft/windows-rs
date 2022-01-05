pub trait ILocationPermissionsImpl: Sized {
    fn GetGlobalLocationPermission();
    fn CheckLocationCapability();
}
pub trait ISensorImpl: Sized {
    fn GetID();
    fn GetCategory();
    fn GetType();
    fn GetFriendlyName();
    fn GetProperty();
    fn GetProperties();
    fn GetSupportedDataFields();
    fn SetProperties();
    fn SupportsDataField();
    fn GetState();
    fn GetData();
    fn SupportsEvent();
    fn GetEventInterest();
    fn SetEventInterest();
    fn SetEventSink();
}
pub trait ISensorCollectionImpl: Sized {
    fn GetAt();
    fn GetCount();
    fn Add();
    fn Remove();
    fn RemoveByID();
    fn Clear();
}
pub trait ISensorDataReportImpl: Sized {
    fn GetTimestamp();
    fn GetSensorValue();
    fn GetSensorValues();
}
pub trait ISensorEventsImpl: Sized {
    fn OnStateChanged();
    fn OnDataUpdated();
    fn OnEvent();
    fn OnLeave();
}
pub trait ISensorManagerImpl: Sized {
    fn GetSensorsByCategory();
    fn GetSensorsByType();
    fn GetSensorByID();
    fn SetEventSink();
    fn RequestPermissions();
}
pub trait ISensorManagerEventsImpl: Sized {
    fn OnSensorEnter();
}
