pub trait IFunctionDiscoveryImpl: Sized {
    fn GetInstanceCollection();
    fn GetInstance();
    fn CreateInstanceCollectionQuery();
    fn CreateInstanceQuery();
    fn AddInstance();
    fn RemoveInstance();
}
pub trait IFunctionDiscoveryNotificationImpl: Sized {
    fn OnUpdate();
    fn OnError();
    fn OnEvent();
}
pub trait IFunctionDiscoveryProviderImpl: Sized {
    fn Initialize();
    fn Query();
    fn EndQuery();
    fn InstancePropertyStoreValidateAccess();
    fn InstancePropertyStoreOpen();
    fn InstancePropertyStoreFlush();
    fn InstanceQueryService();
    fn InstanceReleased();
}
pub trait IFunctionDiscoveryProviderFactoryImpl: Sized {
    fn CreatePropertyStore();
    fn CreateInstance();
    fn CreateFunctionInstanceCollection();
}
pub trait IFunctionDiscoveryProviderQueryImpl: Sized {
    fn IsInstanceQuery();
    fn IsSubcategoryQuery();
    fn GetQueryConstraints();
    fn GetPropertyConstraints();
}
pub trait IFunctionDiscoveryServiceProviderImpl: Sized {
    fn Initialize();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionInstanceImpl: Sized + IServiceProviderImpl {
    fn GetID();
    fn GetProviderInstanceID();
    fn OpenPropertyStore();
    fn GetCategory();
}
pub trait IFunctionInstanceCollectionImpl: Sized {
    fn GetCount();
    fn Get();
    fn Item();
    fn Add();
    fn Remove();
    fn Delete();
    fn DeleteAll();
}
pub trait IFunctionInstanceCollectionQueryImpl: Sized {
    fn AddQueryConstraint();
    fn AddPropertyConstraint();
    fn Execute();
}
pub trait IFunctionInstanceQueryImpl: Sized {
    fn Execute();
}
pub trait IPNPXAssociationImpl: Sized {
    fn Associate();
    fn Unassociate();
    fn Delete();
}
pub trait IPNPXDeviceAssociationImpl: Sized {
    fn Associate();
    fn Unassociate();
    fn Delete();
}
pub trait IPropertyStoreCollectionImpl: Sized {
    fn GetCount();
    fn Get();
    fn Item();
    fn Add();
    fn Remove();
    fn Delete();
    fn DeleteAll();
}
pub trait IProviderPropertiesImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn GetValue();
    fn SetValue();
}
pub trait IProviderPropertyConstraintCollectionImpl: Sized {
    fn GetCount();
    fn Get();
    fn Item();
    fn Next();
    fn Skip();
    fn Reset();
}
pub trait IProviderPublishingImpl: Sized {
    fn CreateInstance();
    fn RemoveInstance();
}
pub trait IProviderQueryConstraintCollectionImpl: Sized {
    fn GetCount();
    fn Get();
    fn Item();
    fn Next();
    fn Skip();
    fn Reset();
}
