#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct DataClasses(i32);
#[repr(transparent)]
pub struct ESim(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ESimAuthenticationPreference(i32);
#[repr(transparent)]
pub struct ESimDiscoverEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimDiscoverResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ESimDiscoverResultKind(i32);
#[repr(transparent)]
pub struct ESimDownloadProfileMetadataResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimOperationResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ESimOperationStatus(i32);
#[repr(transparent)]
pub struct ESimPolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimProfile(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ESimProfileClass(i32);
#[repr(C)]
pub struct ESimProfileInstallProgress(i32);
#[repr(transparent)]
pub struct ESimProfileMetadata(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ESimProfileMetadataState(i32);
#[repr(transparent)]
pub struct ESimProfilePolicy(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ESimProfileState(i32);
#[repr(transparent)]
pub struct ESimRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimServiceInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ESimState(i32);
#[repr(transparent)]
pub struct ESimUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimWatcher(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ESimWatcherStatus(i32);
#[repr(transparent)]
pub struct FdnAccessManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HotspotAuthenticationContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HotspotAuthenticationEventDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct HotspotAuthenticationResponseCode(i32);
#[repr(transparent)]
pub struct HotspotCredentialsAuthenticationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESim(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESim2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESimAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESimDiscoverEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESimDiscoverResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESimDownloadProfileMetadataResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESimManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESimOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESimPolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESimProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESimProfileMetadata(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESimProfilePolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESimRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESimServiceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESimUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESimWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFdnAccessManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHotspotAuthenticationContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHotspotAuthenticationContext2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHotspotAuthenticationContextStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHotspotAuthenticationEventDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHotspotCredentialsAuthenticationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownCSimFilePathsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownRuimFilePathsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownSimFilePathsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownUSimFilePathsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandAccount(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandAccount2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandAccount3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandAccountEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandAccountStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandAccountUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandAccountWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandAntennaSar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandAntennaSarFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandCellCdma(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandCellGsm(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandCellLte(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandCellNR(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandCellTdscdma(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandCellUmts(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandCellsInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandCellsInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandCurrentSlotIndexChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandDeviceInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandDeviceInformation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandDeviceInformation3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandDeviceInformation4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandDeviceService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceCommandResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceCommandSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceDataSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandModem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandModem2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandModem3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandModemConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandModemConfiguration2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandModemIsolation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandModemIsolationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandModemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandNetwork(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandNetwork2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandNetwork3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandNetworkRegistrationStateChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandPco(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandPcoDataChangeTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandPin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandPinLockStateChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandPinLockStateChangeTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandPinManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandPinOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandRadioStateChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandRadioStateChangeTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandSarManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandSlotInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandSlotInfoChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandSlotManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandTransmissionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandUicc(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandUiccApp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandUiccAppReadRecordResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandUiccAppRecordDetailsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMobileBroadbandUiccAppsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkOperatorDataUsageTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkOperatorNotificationEventDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkOperatorTetheringAccessPointConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkOperatorTetheringAccessPointConfiguration2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkOperatorTetheringClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkOperatorTetheringClientManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkOperatorTetheringEntitlementCheck(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkOperatorTetheringManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkOperatorTetheringManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkOperatorTetheringManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkOperatorTetheringManagerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkOperatorTetheringManagerStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkOperatorTetheringOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProvisionFromXmlDocumentResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProvisionedProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProvisioningAgent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProvisioningAgentStaticMethods(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITetheringEntitlementCheckTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUssdMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUssdMessageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUssdReply(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUssdSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUssdSessionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KnownCSimFilePaths(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KnownRuimFilePaths(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KnownSimFilePaths(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KnownUSimFilePaths(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct LegacyNetworkOperatorsContract(i32);
#[repr(transparent)]
pub struct MobileBroadbandAccount(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandAccountEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandAccountUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandAccountWatcher(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MobileBroadbandAccountWatcherStatus(i32);
#[repr(transparent)]
pub struct MobileBroadbandAntennaSar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandCellCdma(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandCellGsm(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandCellLte(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandCellNR(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandCellTdscdma(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandCellUmts(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandCellsInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandCurrentSlotIndexChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandDeviceInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandDeviceService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceCommandResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceCommandSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceDataSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MobileBroadbandDeviceType(i32);
#[repr(transparent)]
pub struct MobileBroadbandModem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandModemConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandModemIsolation(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MobileBroadbandModemStatus(i32);
#[repr(transparent)]
pub struct MobileBroadbandNetwork(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandNetworkRegistrationStateChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandNetworkRegistrationStateChangeTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandPco(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandPcoDataChangeTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandPin(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MobileBroadbandPinFormat(i32);
#[repr(C)]
pub struct MobileBroadbandPinLockState(i32);
#[repr(transparent)]
pub struct MobileBroadbandPinLockStateChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandPinLockStateChangeTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandPinManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandPinOperationResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MobileBroadbandPinType(i32);
#[repr(C)]
pub struct MobileBroadbandRadioState(i32);
#[repr(transparent)]
pub struct MobileBroadbandRadioStateChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandRadioStateChangeTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandSarManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandSlotInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandSlotInfoChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandSlotManager(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MobileBroadbandSlotState(i32);
#[repr(transparent)]
pub struct MobileBroadbandTransmissionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandUicc(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandUiccApp(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MobileBroadbandUiccAppOperationStatus(i32);
#[repr(transparent)]
pub struct MobileBroadbandUiccAppReadRecordResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandUiccAppRecordDetailsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandUiccAppsResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct NetworkDeviceStatus(i32);
#[repr(C)]
pub struct NetworkOperatorDataUsageNotificationKind(i32);
#[repr(transparent)]
pub struct NetworkOperatorDataUsageTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct NetworkOperatorEventMessageType(i32);
#[repr(transparent)]
pub struct NetworkOperatorNotificationEventDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkOperatorTetheringAccessPointConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkOperatorTetheringClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkOperatorTetheringManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkOperatorTetheringOperationResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct NetworkOperatorsFdnContract(i32);
#[repr(C)]
pub struct NetworkRegistrationState(i32);
#[repr(C)]
pub struct ProfileMediaType(i32);
#[cfg(feature = "Foundation")]
#[repr(C)]
pub struct ProfileUsage(i32);
#[repr(transparent)]
pub struct ProvisionFromXmlDocumentResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProvisionedProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProvisioningAgent(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct TetheringCapability(i32);
#[repr(transparent)]
pub struct TetheringEntitlementCheckTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct TetheringOperationStatus(i32);
#[repr(C)]
pub struct TetheringOperationalState(i32);
#[repr(C)]
pub struct TetheringWiFiBand(i32);
#[repr(C)]
pub struct UiccAccessCondition(i32);
#[repr(C)]
pub struct UiccAppKind(i32);
#[repr(C)]
pub struct UiccAppRecordKind(i32);
#[repr(transparent)]
pub struct UssdMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UssdReply(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UssdResultCode(i32);
#[repr(transparent)]
pub struct UssdSession(pub *mut ::core::ffi::c_void);
