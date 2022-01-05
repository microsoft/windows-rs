pub trait IUPnPAddressFamilyControlImpl: Sized {
    fn SetAddressFamily();
    fn GetAddressFamily();
}
pub trait IUPnPAsyncResultImpl: Sized {
    fn AsyncOperationComplete();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDescriptionDocumentImpl: Sized + IDispatchImpl {
    fn ReadyState();
    fn Load();
    fn LoadAsync();
    fn LoadResult();
    fn Abort();
    fn RootDevice();
    fn DeviceByUDN();
}
pub trait IUPnPDescriptionDocumentCallbackImpl: Sized {
    fn LoadComplete();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceImpl: Sized + IDispatchImpl {
    fn IsRootDevice();
    fn RootDevice();
    fn ParentDevice();
    fn HasChildren();
    fn Children();
    fn UniqueDeviceName();
    fn FriendlyName();
    fn Type();
    fn PresentationURL();
    fn ManufacturerName();
    fn ManufacturerURL();
    fn ModelName();
    fn ModelNumber();
    fn Description();
    fn ModelURL();
    fn UPC();
    fn SerialNumber();
    fn IconURL();
    fn Services();
}
pub trait IUPnPDeviceControlImpl: Sized {
    fn Initialize();
    fn GetServiceObject();
}
pub trait IUPnPDeviceControlHttpHeadersImpl: Sized {
    fn GetAdditionalResponseHeaders();
}
pub trait IUPnPDeviceDocumentAccessImpl: Sized {
    fn GetDocumentURL();
}
pub trait IUPnPDeviceDocumentAccessExImpl: Sized {
    fn GetDocument();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDeviceFinderImpl: Sized + IDispatchImpl {
    fn FindByType();
    fn CreateAsyncFind();
    fn StartAsyncFind();
    fn CancelAsyncFind();
    fn FindByUDN();
}
pub trait IUPnPDeviceFinderAddCallbackWithInterfaceImpl: Sized {
    fn DeviceAddedWithInterface();
}
pub trait IUPnPDeviceFinderCallbackImpl: Sized {
    fn DeviceAdded();
    fn DeviceRemoved();
    fn SearchComplete();
}
pub trait IUPnPDeviceProviderImpl: Sized {
    fn Start();
    fn Stop();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPDevicesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
}
pub trait IUPnPEventSinkImpl: Sized {
    fn OnStateChanged();
    fn OnStateChangedSafe();
}
pub trait IUPnPEventSourceImpl: Sized {
    fn Advise();
    fn Unadvise();
}
pub trait IUPnPHttpHeaderControlImpl: Sized {
    fn AddRequestHeaders();
}
pub trait IUPnPRegistrarImpl: Sized {
    fn RegisterDevice();
    fn RegisterRunningDevice();
    fn RegisterDeviceProvider();
    fn GetUniqueDeviceName();
    fn UnregisterDevice();
    fn UnregisterDeviceProvider();
}
pub trait IUPnPRemoteEndpointInfoImpl: Sized {
    fn GetDwordValue();
    fn GetStringValue();
    fn GetGuidValue();
}
pub trait IUPnPReregistrarImpl: Sized {
    fn ReregisterDevice();
    fn ReregisterRunningDevice();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPServiceImpl: Sized + IDispatchImpl {
    fn QueryStateVariable();
    fn InvokeAction();
    fn ServiceTypeIdentifier();
    fn AddCallback();
    fn Id();
    fn LastTransportStatus();
}
pub trait IUPnPServiceAsyncImpl: Sized {
    fn BeginInvokeAction();
    fn EndInvokeAction();
    fn BeginQueryStateVariable();
    fn EndQueryStateVariable();
    fn BeginSubscribeToEvents();
    fn EndSubscribeToEvents();
    fn BeginSCPDDownload();
    fn EndSCPDDownload();
    fn CancelAsyncOperation();
}
pub trait IUPnPServiceCallbackImpl: Sized {
    fn StateVariableChanged();
    fn ServiceInstanceDied();
}
pub trait IUPnPServiceDocumentAccessImpl: Sized {
    fn GetDocumentURL();
    fn GetDocument();
}
pub trait IUPnPServiceEnumPropertyImpl: Sized {
    fn SetServiceEnumProperty();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPServicesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
}
