pub trait IDontSupportEventSubscriptionImpl: Sized {}
pub trait IEnumEventObjectImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEventClassImpl: Sized + IDispatchImpl {
    fn EventClassID();
    fn SetEventClassID();
    fn EventClassName();
    fn SetEventClassName();
    fn OwnerSID();
    fn SetOwnerSID();
    fn FiringInterfaceID();
    fn SetFiringInterfaceID();
    fn Description();
    fn SetDescription();
    fn CustomConfigCLSID();
    fn SetCustomConfigCLSID();
    fn TypeLib();
    fn SetTypeLib();
}
pub trait IEventClass2Impl: Sized + IEventClassImpl + IDispatchImpl {
    fn PublisherID();
    fn SetPublisherID();
    fn MultiInterfacePublisherFilterCLSID();
    fn SetMultiInterfacePublisherFilterCLSID();
    fn AllowInprocActivation();
    fn SetAllowInprocActivation();
    fn FireInParallel();
    fn SetFireInParallel();
}
pub trait IEventControlImpl: Sized + IDispatchImpl {
    fn SetPublisherFilter();
    fn AllowInprocActivation();
    fn SetAllowInprocActivation();
    fn GetSubscriptions();
    fn SetDefaultQuery();
}
pub trait IEventObjectChangeImpl: Sized {
    fn ChangedSubscription();
    fn ChangedEventClass();
    fn ChangedPublisher();
}
pub trait IEventObjectChange2Impl: Sized {
    fn ChangedSubscription();
    fn ChangedEventClass();
}
pub trait IEventObjectCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn NewEnum();
    fn Count();
    fn Add();
    fn Remove();
}
pub trait IEventPropertyImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Value();
    fn SetValue();
}
pub trait IEventPublisherImpl: Sized + IDispatchImpl {
    fn PublisherID();
    fn SetPublisherID();
    fn PublisherName();
    fn SetPublisherName();
    fn PublisherType();
    fn SetPublisherType();
    fn OwnerSID();
    fn SetOwnerSID();
    fn Description();
    fn SetDescription();
    fn GetDefaultProperty();
    fn PutDefaultProperty();
    fn RemoveDefaultProperty();
    fn GetDefaultPropertyCollection();
}
pub trait IEventSubscriptionImpl: Sized + IDispatchImpl {
    fn SubscriptionID();
    fn SetSubscriptionID();
    fn SubscriptionName();
    fn SetSubscriptionName();
    fn PublisherID();
    fn SetPublisherID();
    fn EventClassID();
    fn SetEventClassID();
    fn MethodName();
    fn SetMethodName();
    fn SubscriberCLSID();
    fn SetSubscriberCLSID();
    fn SubscriberInterface();
    fn SetSubscriberInterface();
    fn PerUser();
    fn SetPerUser();
    fn OwnerSID();
    fn SetOwnerSID();
    fn Enabled();
    fn SetEnabled();
    fn Description();
    fn SetDescription();
    fn MachineName();
    fn SetMachineName();
    fn GetPublisherProperty();
    fn PutPublisherProperty();
    fn RemovePublisherProperty();
    fn GetPublisherPropertyCollection();
    fn GetSubscriberProperty();
    fn PutSubscriberProperty();
    fn RemoveSubscriberProperty();
    fn GetSubscriberPropertyCollection();
    fn InterfaceID();
    fn SetInterfaceID();
}
pub trait IEventSystemImpl: Sized + IDispatchImpl {
    fn Query();
    fn Store();
    fn Remove();
    fn EventObjectChangeEventClassID();
    fn QueryS();
    fn RemoveS();
}
pub trait IFiringControlImpl: Sized + IDispatchImpl {
    fn FireSubscription();
}
pub trait IMultiInterfaceEventControlImpl: Sized {
    fn SetMultiInterfacePublisherFilter();
    fn GetSubscriptions();
    fn SetDefaultQuery();
    fn AllowInprocActivation();
    fn SetAllowInprocActivation();
    fn FireInParallel();
    fn SetFireInParallel();
}
pub trait IMultiInterfacePublisherFilterImpl: Sized {
    fn Initialize();
    fn PrepareToFire();
}
pub trait IPublisherFilterImpl: Sized {
    fn Initialize();
    fn PrepareToFire();
}
