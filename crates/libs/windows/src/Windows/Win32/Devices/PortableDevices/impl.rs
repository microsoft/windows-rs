pub trait IConnectionRequestCallbackImpl: Sized {
    fn OnComplete();
}
pub trait IEnumPortableDeviceConnectorsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumPortableDeviceObjectIDsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn Cancel();
}
pub trait IMediaRadioManagerImpl: Sized {
    fn GetRadioInstances();
    fn OnSystemRadioStateChange();
}
pub trait IMediaRadioManagerNotifySinkImpl: Sized {
    fn OnInstanceAdd();
    fn OnInstanceRemove();
    fn OnInstanceRadioChange();
}
pub trait IPortableDeviceImpl: Sized {
    fn Open();
    fn SendCommand();
    fn Content();
    fn Capabilities();
    fn Cancel();
    fn Close();
    fn Advise();
    fn Unadvise();
    fn GetPnPDeviceID();
}
pub trait IPortableDeviceCapabilitiesImpl: Sized {
    fn GetSupportedCommands();
    fn GetCommandOptions();
    fn GetFunctionalCategories();
    fn GetFunctionalObjects();
    fn GetSupportedContentTypes();
    fn GetSupportedFormats();
    fn GetSupportedFormatProperties();
    fn GetFixedPropertyAttributes();
    fn Cancel();
    fn GetSupportedEvents();
    fn GetEventOptions();
}
pub trait IPortableDeviceConnectorImpl: Sized {
    fn Connect();
    fn Disconnect();
    fn Cancel();
    fn GetProperty();
    fn SetProperty();
    fn GetPnPID();
}
pub trait IPortableDeviceContentImpl: Sized {
    fn EnumObjects();
    fn Properties();
    fn Transfer();
    fn CreateObjectWithPropertiesOnly();
    fn CreateObjectWithPropertiesAndData();
    fn Delete();
    fn GetObjectIDsFromPersistentUniqueIDs();
    fn Cancel();
    fn Move();
    fn Copy();
}
pub trait IPortableDeviceContent2Impl: Sized + IPortableDeviceContentImpl {
    fn UpdateObjectWithPropertiesAndData();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceDataStreamImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn GetObjectID();
    fn Cancel();
}
pub trait IPortableDeviceDispatchFactoryImpl: Sized {
    fn GetDeviceDispatch();
}
pub trait IPortableDeviceEventCallbackImpl: Sized {
    fn OnEvent();
}
pub trait IPortableDeviceKeyCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn Add();
    fn Clear();
    fn RemoveAt();
}
pub trait IPortableDeviceManagerImpl: Sized {
    fn GetDevices();
    fn RefreshDeviceList();
    fn GetDeviceFriendlyName();
    fn GetDeviceDescription();
    fn GetDeviceManufacturer();
    fn GetDeviceProperty();
    fn GetPrivateDevices();
}
pub trait IPortableDevicePropVariantCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn Add();
    fn GetType();
    fn ChangeType();
    fn Clear();
    fn RemoveAt();
}
pub trait IPortableDevicePropertiesImpl: Sized {
    fn GetSupportedProperties();
    fn GetPropertyAttributes();
    fn GetValues();
    fn SetValues();
    fn Delete();
    fn Cancel();
}
pub trait IPortableDevicePropertiesBulkImpl: Sized {
    fn QueueGetValuesByObjectList();
    fn QueueGetValuesByObjectFormat();
    fn QueueSetValuesByObjectList();
    fn Start();
    fn Cancel();
}
pub trait IPortableDevicePropertiesBulkCallbackImpl: Sized {
    fn OnStart();
    fn OnProgress();
    fn OnEnd();
}
pub trait IPortableDeviceResourcesImpl: Sized {
    fn GetSupportedResources();
    fn GetResourceAttributes();
    fn GetStream();
    fn Delete();
    fn Cancel();
    fn CreateResource();
}
pub trait IPortableDeviceServiceImpl: Sized {
    fn Open();
    fn Capabilities();
    fn Content();
    fn Methods();
    fn Cancel();
    fn Close();
    fn GetServiceObjectID();
    fn GetPnPServiceID();
    fn Advise();
    fn Unadvise();
    fn SendCommand();
}
pub trait IPortableDeviceServiceActivationImpl: Sized {
    fn OpenAsync();
    fn CancelOpenAsync();
}
pub trait IPortableDeviceServiceCapabilitiesImpl: Sized {
    fn GetSupportedMethods();
    fn GetSupportedMethodsByFormat();
    fn GetMethodAttributes();
    fn GetMethodParameterAttributes();
    fn GetSupportedFormats();
    fn GetFormatAttributes();
    fn GetSupportedFormatProperties();
    fn GetFormatPropertyAttributes();
    fn GetSupportedEvents();
    fn GetEventAttributes();
    fn GetEventParameterAttributes();
    fn GetInheritedServices();
    fn GetFormatRenderingProfiles();
    fn GetSupportedCommands();
    fn GetCommandOptions();
    fn Cancel();
}
pub trait IPortableDeviceServiceManagerImpl: Sized {
    fn GetDeviceServices();
    fn GetDeviceForService();
}
pub trait IPortableDeviceServiceMethodCallbackImpl: Sized {
    fn OnComplete();
}
pub trait IPortableDeviceServiceMethodsImpl: Sized {
    fn Invoke();
    fn InvokeAsync();
    fn Cancel();
}
pub trait IPortableDeviceServiceOpenCallbackImpl: Sized {
    fn OnComplete();
}
pub trait IPortableDeviceUnitsStreamImpl: Sized {
    fn SeekInUnits();
    fn Cancel();
}
pub trait IPortableDeviceValuesImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn SetValue();
    fn GetValue();
    fn SetStringValue();
    fn GetStringValue();
    fn SetUnsignedIntegerValue();
    fn GetUnsignedIntegerValue();
    fn SetSignedIntegerValue();
    fn GetSignedIntegerValue();
    fn SetUnsignedLargeIntegerValue();
    fn GetUnsignedLargeIntegerValue();
    fn SetSignedLargeIntegerValue();
    fn GetSignedLargeIntegerValue();
    fn SetFloatValue();
    fn GetFloatValue();
    fn SetErrorValue();
    fn GetErrorValue();
    fn SetKeyValue();
    fn GetKeyValue();
    fn SetBoolValue();
    fn GetBoolValue();
    fn SetIUnknownValue();
    fn GetIUnknownValue();
    fn SetGuidValue();
    fn GetGuidValue();
    fn SetBufferValue();
    fn GetBufferValue();
    fn SetIPortableDeviceValuesValue();
    fn GetIPortableDeviceValuesValue();
    fn SetIPortableDevicePropVariantCollectionValue();
    fn GetIPortableDevicePropVariantCollectionValue();
    fn SetIPortableDeviceKeyCollectionValue();
    fn GetIPortableDeviceKeyCollectionValue();
    fn SetIPortableDeviceValuesCollectionValue();
    fn GetIPortableDeviceValuesCollectionValue();
    fn RemoveValue();
    fn CopyValuesFromPropertyStore();
    fn CopyValuesToPropertyStore();
    fn Clear();
}
pub trait IPortableDeviceValuesCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn Add();
    fn Clear();
    fn RemoveAt();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceWebControlImpl: Sized + IDispatchImpl {
    fn GetDeviceFromId();
    fn GetDeviceFromIdAsync();
}
pub trait IRadioInstanceImpl: Sized {
    fn GetRadioManagerSignature();
    fn GetInstanceSignature();
    fn GetFriendlyName();
    fn GetRadioState();
    fn SetRadioState();
    fn IsMultiComm();
    fn IsAssociatingDevice();
}
pub trait IRadioInstanceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
}
pub trait IWpdSerializerImpl: Sized {
    fn GetIPortableDeviceValuesFromBuffer();
    fn WriteIPortableDeviceValuesToBuffer();
    fn GetBufferFromIPortableDeviceValues();
    fn GetSerializedSize();
}
