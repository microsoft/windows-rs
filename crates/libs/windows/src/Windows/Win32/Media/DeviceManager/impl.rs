pub trait IComponentAuthenticateImpl: Sized {
    fn SACAuth();
    fn SACGetProtocols();
}
pub trait IMDSPDeviceImpl: Sized {
    fn GetName();
    fn GetManufacturer();
    fn GetVersion();
    fn GetType();
    fn GetSerialNumber();
    fn GetPowerSource();
    fn GetStatus();
    fn GetDeviceIcon();
    fn EnumStorage();
    fn GetFormatSupport();
    fn SendOpaqueCommand();
}
pub trait IMDSPDevice2Impl: Sized + IMDSPDeviceImpl {
    fn GetStorage();
    fn GetFormatSupport2();
    fn GetSpecifyPropertyPages();
    fn GetCanonicalName();
}
pub trait IMDSPDevice3Impl: Sized + IMDSPDevice2Impl + IMDSPDeviceImpl {
    fn GetProperty();
    fn SetProperty();
    fn GetFormatCapability();
    fn DeviceIoControl();
    fn FindStorage();
}
pub trait IMDSPDeviceControlImpl: Sized {
    fn GetDCStatus();
    fn GetCapabilities();
    fn Play();
    fn Record();
    fn Pause();
    fn Resume();
    fn Stop();
    fn Seek();
}
pub trait IMDSPDirectTransferImpl: Sized {
    fn TransferToDevice();
}
pub trait IMDSPEnumDeviceImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IMDSPEnumStorageImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IMDSPObjectImpl: Sized {
    fn Open();
    fn Read();
    fn Write();
    fn Delete();
    fn Seek();
    fn Rename();
    fn Move();
    fn Close();
}
pub trait IMDSPObject2Impl: Sized + IMDSPObjectImpl {
    fn ReadOnClearChannel();
    fn WriteOnClearChannel();
}
pub trait IMDSPObjectInfoImpl: Sized {
    fn GetPlayLength();
    fn SetPlayLength();
    fn GetPlayOffset();
    fn SetPlayOffset();
    fn GetTotalLength();
    fn GetLastPlayPosition();
    fn GetLongestPlayPosition();
}
pub trait IMDSPRevokedImpl: Sized {
    fn GetRevocationURL();
}
pub trait IMDSPStorageImpl: Sized {
    fn SetAttributes();
    fn GetStorageGlobals();
    fn GetAttributes();
    fn GetName();
    fn GetDate();
    fn GetSize();
    fn GetRights();
    fn CreateStorage();
    fn EnumStorage();
    fn SendOpaqueCommand();
}
pub trait IMDSPStorage2Impl: Sized + IMDSPStorageImpl {
    fn GetStorage();
    fn CreateStorage2();
    fn SetAttributes2();
    fn GetAttributes2();
}
pub trait IMDSPStorage3Impl: Sized + IMDSPStorage2Impl + IMDSPStorageImpl {
    fn GetMetadata();
    fn SetMetadata();
}
pub trait IMDSPStorage4Impl: Sized + IMDSPStorage3Impl + IMDSPStorage2Impl + IMDSPStorageImpl {
    fn SetReferences();
    fn GetReferences();
    fn CreateStorageWithMetadata();
    fn GetSpecifiedMetadata();
    fn FindStorage();
    fn GetParent();
}
pub trait IMDSPStorageGlobalsImpl: Sized {
    fn GetCapabilities();
    fn GetSerialNumber();
    fn GetTotalSize();
    fn GetTotalFree();
    fn GetTotalBad();
    fn GetStatus();
    fn Initialize();
    fn GetDevice();
    fn GetRootStorage();
}
pub trait IMDServiceProviderImpl: Sized {
    fn GetDeviceCount();
    fn EnumDevices();
}
pub trait IMDServiceProvider2Impl: Sized + IMDServiceProviderImpl {
    fn CreateDevice();
}
pub trait IMDServiceProvider3Impl: Sized + IMDServiceProvider2Impl + IMDServiceProviderImpl {
    fn SetDeviceEnumPreference();
}
pub trait ISCPSecureAuthenticateImpl: Sized {
    fn GetSecureQuery();
}
pub trait ISCPSecureAuthenticate2Impl: Sized + ISCPSecureAuthenticateImpl {
    fn GetSCPSession();
}
pub trait ISCPSecureExchangeImpl: Sized {
    fn TransferContainerData();
    fn ObjectData();
    fn TransferComplete();
}
pub trait ISCPSecureExchange2Impl: Sized + ISCPSecureExchangeImpl {
    fn TransferContainerData2();
}
pub trait ISCPSecureExchange3Impl: Sized + ISCPSecureExchange2Impl + ISCPSecureExchangeImpl {
    fn TransferContainerDataOnClearChannel();
    fn GetObjectDataOnClearChannel();
    fn TransferCompleteForDevice();
}
pub trait ISCPSecureQueryImpl: Sized {
    fn GetDataDemands();
    fn ExamineData();
    fn MakeDecision();
    fn GetRights();
}
pub trait ISCPSecureQuery2Impl: Sized + ISCPSecureQueryImpl {
    fn MakeDecision2();
}
pub trait ISCPSecureQuery3Impl: Sized + ISCPSecureQuery2Impl + ISCPSecureQueryImpl {
    fn GetRightsOnClearChannel();
    fn MakeDecisionOnClearChannel();
}
pub trait ISCPSessionImpl: Sized {
    fn BeginSession();
    fn EndSession();
    fn GetSecureQuery();
}
pub trait IWMDMDeviceImpl: Sized {
    fn GetName();
    fn GetManufacturer();
    fn GetVersion();
    fn GetType();
    fn GetSerialNumber();
    fn GetPowerSource();
    fn GetStatus();
    fn GetDeviceIcon();
    fn EnumStorage();
    fn GetFormatSupport();
    fn SendOpaqueCommand();
}
pub trait IWMDMDevice2Impl: Sized + IWMDMDeviceImpl {
    fn GetStorage();
    fn GetFormatSupport2();
    fn GetSpecifyPropertyPages();
    fn GetCanonicalName();
}
pub trait IWMDMDevice3Impl: Sized + IWMDMDevice2Impl + IWMDMDeviceImpl {
    fn GetProperty();
    fn SetProperty();
    fn GetFormatCapability();
    fn DeviceIoControl();
    fn FindStorage();
}
pub trait IWMDMDeviceControlImpl: Sized {
    fn GetStatus();
    fn GetCapabilities();
    fn Play();
    fn Record();
    fn Pause();
    fn Resume();
    fn Stop();
    fn Seek();
}
pub trait IWMDMDeviceSessionImpl: Sized {
    fn BeginSession();
    fn EndSession();
}
pub trait IWMDMEnumDeviceImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IWMDMEnumStorageImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IWMDMLoggerImpl: Sized {
    fn IsEnabled();
    fn Enable();
    fn GetLogFileName();
    fn SetLogFileName();
    fn LogString();
    fn LogDword();
    fn Reset();
    fn GetSizeParams();
    fn SetSizeParams();
}
pub trait IWMDMMetaDataImpl: Sized {
    fn AddItem();
    fn QueryByName();
    fn QueryByIndex();
    fn GetItemCount();
}
pub trait IWMDMNotificationImpl: Sized {
    fn WMDMMessage();
}
pub trait IWMDMObjectInfoImpl: Sized {
    fn GetPlayLength();
    fn SetPlayLength();
    fn GetPlayOffset();
    fn SetPlayOffset();
    fn GetTotalLength();
    fn GetLastPlayPosition();
    fn GetLongestPlayPosition();
}
pub trait IWMDMOperationImpl: Sized {
    fn BeginRead();
    fn BeginWrite();
    fn GetObjectName();
    fn SetObjectName();
    fn GetObjectAttributes();
    fn SetObjectAttributes();
    fn GetObjectTotalSize();
    fn SetObjectTotalSize();
    fn TransferObjectData();
    fn End();
}
pub trait IWMDMOperation2Impl: Sized + IWMDMOperationImpl {
    fn SetObjectAttributes2();
    fn GetObjectAttributes2();
}
pub trait IWMDMOperation3Impl: Sized + IWMDMOperationImpl {
    fn TransferObjectDataOnClearChannel();
}
pub trait IWMDMProgressImpl: Sized {
    fn Begin();
    fn Progress();
    fn End();
}
pub trait IWMDMProgress2Impl: Sized + IWMDMProgressImpl {
    fn End2();
}
pub trait IWMDMProgress3Impl: Sized + IWMDMProgress2Impl + IWMDMProgressImpl {
    fn Begin3();
    fn Progress3();
    fn End3();
}
pub trait IWMDMRevokedImpl: Sized {
    fn GetRevocationURL();
}
pub trait IWMDMStorageImpl: Sized {
    fn SetAttributes();
    fn GetStorageGlobals();
    fn GetAttributes();
    fn GetName();
    fn GetDate();
    fn GetSize();
    fn GetRights();
    fn EnumStorage();
    fn SendOpaqueCommand();
}
pub trait IWMDMStorage2Impl: Sized + IWMDMStorageImpl {
    fn GetStorage();
    fn SetAttributes2();
    fn GetAttributes2();
}
pub trait IWMDMStorage3Impl: Sized + IWMDMStorage2Impl + IWMDMStorageImpl {
    fn GetMetadata();
    fn SetMetadata();
    fn CreateEmptyMetadataObject();
    fn SetEnumPreference();
}
pub trait IWMDMStorage4Impl: Sized + IWMDMStorage3Impl + IWMDMStorage2Impl + IWMDMStorageImpl {
    fn SetReferences();
    fn GetReferences();
    fn GetRightsWithProgress();
    fn GetSpecifiedMetadata();
    fn FindStorage();
    fn GetParent();
}
pub trait IWMDMStorageControlImpl: Sized {
    fn Insert();
    fn Delete();
    fn Rename();
    fn Read();
    fn Move();
}
pub trait IWMDMStorageControl2Impl: Sized + IWMDMStorageControlImpl {
    fn Insert2();
}
pub trait IWMDMStorageControl3Impl: Sized + IWMDMStorageControl2Impl + IWMDMStorageControlImpl {
    fn Insert3();
}
pub trait IWMDMStorageGlobalsImpl: Sized {
    fn GetCapabilities();
    fn GetSerialNumber();
    fn GetTotalSize();
    fn GetTotalFree();
    fn GetTotalBad();
    fn GetStatus();
    fn Initialize();
}
pub trait IWMDeviceManagerImpl: Sized {
    fn GetRevision();
    fn GetDeviceCount();
    fn EnumDevices();
}
pub trait IWMDeviceManager2Impl: Sized + IWMDeviceManagerImpl {
    fn GetDeviceFromCanonicalName();
    fn EnumDevices2();
    fn Reinitialize();
}
pub trait IWMDeviceManager3Impl: Sized + IWMDeviceManager2Impl + IWMDeviceManagerImpl {
    fn SetDeviceEnumPreference();
}
