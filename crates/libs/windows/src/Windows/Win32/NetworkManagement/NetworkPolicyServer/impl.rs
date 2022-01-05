#[cfg(feature = "Win32_System_Com")]
pub trait ISdoImpl: Sized + IDispatchImpl {
    fn GetPropertyInfo();
    fn GetProperty();
    fn PutProperty();
    fn ResetProperty();
    fn Apply();
    fn Restore();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISdoCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Add();
    fn Remove();
    fn RemoveAll();
    fn Reload();
    fn IsNameUnique();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISdoDictionaryOldImpl: Sized + IDispatchImpl {
    fn EnumAttributes();
    fn GetAttributeInfo();
    fn EnumAttributeValues();
    fn CreateAttribute();
    fn GetAttributeID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISdoMachineImpl: Sized + IDispatchImpl {
    fn Attach();
    fn GetDictionarySDO();
    fn GetServiceSDO();
    fn GetUserSDO();
    fn GetOSType();
    fn GetDomainType();
    fn IsDirectoryAvailable();
    fn GetAttachedComputer();
    fn GetSDOSchema();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISdoMachine2Impl: Sized + ISdoMachineImpl + IDispatchImpl {
    fn GetTemplatesSDO();
    fn EnableTemplates();
    fn SyncConfigAgainstTemplates();
    fn ImportRemoteTemplates();
    fn Reload();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISdoServiceControlImpl: Sized + IDispatchImpl {
    fn StartService();
    fn StopService();
    fn GetServiceStatus();
    fn ResetService();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITemplateSdoImpl: Sized + ISdoImpl + IDispatchImpl {
    fn AddToCollection();
    fn AddToSdo();
    fn AddToSdoAsProperty();
}
