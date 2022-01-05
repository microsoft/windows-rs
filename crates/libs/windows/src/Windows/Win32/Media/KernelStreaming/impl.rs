pub trait IKsAggregateControlImpl: Sized {
    fn KsAddAggregate();
    fn KsRemoveAggregate();
}
pub trait IKsControlImpl: Sized {
    fn KsProperty();
    fn KsMethod();
    fn KsEvent();
}
pub trait IKsFormatSupportImpl: Sized {
    fn IsFormatSupported();
    fn GetDevicePreferredFormat();
}
pub trait IKsJackContainerIdImpl: Sized {
    fn GetJackContainerId();
}
pub trait IKsJackDescriptionImpl: Sized {
    fn GetJackCount();
    fn GetJackDescription();
}
pub trait IKsJackDescription2Impl: Sized {
    fn GetJackCount();
    fn GetJackDescription2();
}
pub trait IKsJackSinkInformationImpl: Sized {
    fn GetJackSinkInformation();
}
pub trait IKsPropertySetImpl: Sized {
    fn Set();
    fn Get();
    fn QuerySupported();
}
pub trait IKsTopologyImpl: Sized {
    fn CreateNodeInstance();
}
