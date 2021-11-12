#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DataClasses(pub u32);
impl DataClasses {
    pub const None: DataClasses = DataClasses(0u32);
    pub const Gprs: DataClasses = DataClasses(1u32);
    pub const Edge: DataClasses = DataClasses(2u32);
    pub const Umts: DataClasses = DataClasses(4u32);
    pub const Hsdpa: DataClasses = DataClasses(8u32);
    pub const Hsupa: DataClasses = DataClasses(16u32);
    pub const LteAdvanced: DataClasses = DataClasses(32u32);
    pub const NewRadioNonStandalone: DataClasses = DataClasses(64u32);
    pub const NewRadioStandalone: DataClasses = DataClasses(128u32);
    pub const Cdma1xRtt: DataClasses = DataClasses(65536u32);
    pub const Cdma1xEvdo: DataClasses = DataClasses(131072u32);
    pub const Cdma1xEvdoRevA: DataClasses = DataClasses(262144u32);
    pub const Cdma1xEvdv: DataClasses = DataClasses(524288u32);
    pub const Cdma3xRtt: DataClasses = DataClasses(1048576u32);
    pub const Cdma1xEvdoRevB: DataClasses = DataClasses(2097152u32);
    pub const CdmaUmb: DataClasses = DataClasses(4194304u32);
    pub const Custom: DataClasses = DataClasses(2147483648u32);
}
#[repr(transparent)]
pub struct ESim(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimAuthenticationPreference(pub i32);
impl ESimAuthenticationPreference {
    pub const OnEntry: ESimAuthenticationPreference = ESimAuthenticationPreference(0i32);
    pub const OnAction: ESimAuthenticationPreference = ESimAuthenticationPreference(1i32);
    pub const Never: ESimAuthenticationPreference = ESimAuthenticationPreference(2i32);
}
#[repr(transparent)]
pub struct ESimDiscoverEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimDiscoverResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimDiscoverResultKind(pub i32);
impl ESimDiscoverResultKind {
    pub const None: ESimDiscoverResultKind = ESimDiscoverResultKind(0i32);
    pub const Events: ESimDiscoverResultKind = ESimDiscoverResultKind(1i32);
    pub const ProfileMetadata: ESimDiscoverResultKind = ESimDiscoverResultKind(2i32);
}
#[repr(transparent)]
pub struct ESimDownloadProfileMetadataResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimOperationStatus(pub i32);
impl ESimOperationStatus {
    pub const Success: ESimOperationStatus = ESimOperationStatus(0i32);
    pub const NotAuthorized: ESimOperationStatus = ESimOperationStatus(1i32);
    pub const NotFound: ESimOperationStatus = ESimOperationStatus(2i32);
    pub const PolicyViolation: ESimOperationStatus = ESimOperationStatus(3i32);
    pub const InsufficientSpaceOnCard: ESimOperationStatus = ESimOperationStatus(4i32);
    pub const ServerFailure: ESimOperationStatus = ESimOperationStatus(5i32);
    pub const ServerNotReachable: ESimOperationStatus = ESimOperationStatus(6i32);
    pub const TimeoutWaitingForUserConsent: ESimOperationStatus = ESimOperationStatus(7i32);
    pub const IncorrectConfirmationCode: ESimOperationStatus = ESimOperationStatus(8i32);
    pub const ConfirmationCodeMaxRetriesExceeded: ESimOperationStatus = ESimOperationStatus(9i32);
    pub const CardRemoved: ESimOperationStatus = ESimOperationStatus(10i32);
    pub const CardBusy: ESimOperationStatus = ESimOperationStatus(11i32);
    pub const Other: ESimOperationStatus = ESimOperationStatus(12i32);
    pub const CardGeneralFailure: ESimOperationStatus = ESimOperationStatus(13i32);
    pub const ConfirmationCodeMissing: ESimOperationStatus = ESimOperationStatus(14i32);
    pub const InvalidMatchingId: ESimOperationStatus = ESimOperationStatus(15i32);
    pub const NoEligibleProfileForThisDevice: ESimOperationStatus = ESimOperationStatus(16i32);
    pub const OperationAborted: ESimOperationStatus = ESimOperationStatus(17i32);
    pub const EidMismatch: ESimOperationStatus = ESimOperationStatus(18i32);
    pub const ProfileNotAvailableForNewBinding: ESimOperationStatus = ESimOperationStatus(19i32);
    pub const ProfileNotReleasedByOperator: ESimOperationStatus = ESimOperationStatus(20i32);
    pub const OperationProhibitedByProfileClass: ESimOperationStatus = ESimOperationStatus(21i32);
    pub const ProfileNotPresent: ESimOperationStatus = ESimOperationStatus(22i32);
    pub const NoCorrespondingRequest: ESimOperationStatus = ESimOperationStatus(23i32);
    pub const TimeoutWaitingForResponse: ESimOperationStatus = ESimOperationStatus(24i32);
    pub const IccidAlreadyExists: ESimOperationStatus = ESimOperationStatus(25i32);
    pub const ProfileProcessingError: ESimOperationStatus = ESimOperationStatus(26i32);
    pub const ServerNotTrusted: ESimOperationStatus = ESimOperationStatus(27i32);
    pub const ProfileDownloadMaxRetriesExceeded: ESimOperationStatus = ESimOperationStatus(28i32);
}
#[repr(transparent)]
pub struct ESimPolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimProfileClass(pub i32);
impl ESimProfileClass {
    pub const Operational: ESimProfileClass = ESimProfileClass(0i32);
    pub const Test: ESimProfileClass = ESimProfileClass(1i32);
    pub const Provisioning: ESimProfileClass = ESimProfileClass(2i32);
}
#[repr(C)]
pub struct ESimProfileInstallProgress(i32);
#[repr(transparent)]
pub struct ESimProfileMetadata(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimProfileMetadataState(pub i32);
impl ESimProfileMetadataState {
    pub const Unknown: ESimProfileMetadataState = ESimProfileMetadataState(0i32);
    pub const WaitingForInstall: ESimProfileMetadataState = ESimProfileMetadataState(1i32);
    pub const Downloading: ESimProfileMetadataState = ESimProfileMetadataState(2i32);
    pub const Installing: ESimProfileMetadataState = ESimProfileMetadataState(3i32);
    pub const Expired: ESimProfileMetadataState = ESimProfileMetadataState(4i32);
    pub const RejectingDownload: ESimProfileMetadataState = ESimProfileMetadataState(5i32);
    pub const NoLongerAvailable: ESimProfileMetadataState = ESimProfileMetadataState(6i32);
    pub const DeniedByPolicy: ESimProfileMetadataState = ESimProfileMetadataState(7i32);
}
#[repr(transparent)]
pub struct ESimProfilePolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimProfileState(pub i32);
impl ESimProfileState {
    pub const Unknown: ESimProfileState = ESimProfileState(0i32);
    pub const Disabled: ESimProfileState = ESimProfileState(1i32);
    pub const Enabled: ESimProfileState = ESimProfileState(2i32);
    pub const Deleted: ESimProfileState = ESimProfileState(3i32);
}
#[repr(transparent)]
pub struct ESimRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimServiceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimState(pub i32);
impl ESimState {
    pub const Unknown: ESimState = ESimState(0i32);
    pub const Idle: ESimState = ESimState(1i32);
    pub const Removed: ESimState = ESimState(2i32);
    pub const Busy: ESimState = ESimState(3i32);
}
#[repr(transparent)]
pub struct ESimUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimWatcherStatus(pub i32);
impl ESimWatcherStatus {
    pub const Created: ESimWatcherStatus = ESimWatcherStatus(0i32);
    pub const Started: ESimWatcherStatus = ESimWatcherStatus(1i32);
    pub const EnumerationCompleted: ESimWatcherStatus = ESimWatcherStatus(2i32);
    pub const Stopping: ESimWatcherStatus = ESimWatcherStatus(3i32);
    pub const Stopped: ESimWatcherStatus = ESimWatcherStatus(4i32);
}
#[repr(transparent)]
pub struct HotspotAuthenticationContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HotspotAuthenticationEventDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HotspotAuthenticationResponseCode(pub i32);
impl HotspotAuthenticationResponseCode {
    pub const NoError: HotspotAuthenticationResponseCode = HotspotAuthenticationResponseCode(0i32);
    pub const LoginSucceeded: HotspotAuthenticationResponseCode = HotspotAuthenticationResponseCode(50i32);
    pub const LoginFailed: HotspotAuthenticationResponseCode = HotspotAuthenticationResponseCode(100i32);
    pub const RadiusServerError: HotspotAuthenticationResponseCode = HotspotAuthenticationResponseCode(102i32);
    pub const NetworkAdministratorError: HotspotAuthenticationResponseCode = HotspotAuthenticationResponseCode(105i32);
    pub const LoginAborted: HotspotAuthenticationResponseCode = HotspotAuthenticationResponseCode(151i32);
    pub const AccessGatewayInternalError: HotspotAuthenticationResponseCode = HotspotAuthenticationResponseCode(255i32);
}
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
#[repr(transparent)]
pub struct MobileBroadbandAccountWatcherStatus(pub i32);
impl MobileBroadbandAccountWatcherStatus {
    pub const Created: MobileBroadbandAccountWatcherStatus = MobileBroadbandAccountWatcherStatus(0i32);
    pub const Started: MobileBroadbandAccountWatcherStatus = MobileBroadbandAccountWatcherStatus(1i32);
    pub const EnumerationCompleted: MobileBroadbandAccountWatcherStatus = MobileBroadbandAccountWatcherStatus(2i32);
    pub const Stopped: MobileBroadbandAccountWatcherStatus = MobileBroadbandAccountWatcherStatus(3i32);
    pub const Aborted: MobileBroadbandAccountWatcherStatus = MobileBroadbandAccountWatcherStatus(4i32);
}
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
#[repr(transparent)]
pub struct MobileBroadbandDeviceType(pub i32);
impl MobileBroadbandDeviceType {
    pub const Unknown: MobileBroadbandDeviceType = MobileBroadbandDeviceType(0i32);
    pub const Embedded: MobileBroadbandDeviceType = MobileBroadbandDeviceType(1i32);
    pub const Removable: MobileBroadbandDeviceType = MobileBroadbandDeviceType(2i32);
    pub const Remote: MobileBroadbandDeviceType = MobileBroadbandDeviceType(3i32);
}
#[repr(transparent)]
pub struct MobileBroadbandModem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandModemConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandModemIsolation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandModemStatus(pub i32);
impl MobileBroadbandModemStatus {
    pub const Success: MobileBroadbandModemStatus = MobileBroadbandModemStatus(0i32);
    pub const OtherFailure: MobileBroadbandModemStatus = MobileBroadbandModemStatus(1i32);
    pub const Busy: MobileBroadbandModemStatus = MobileBroadbandModemStatus(2i32);
    pub const NoDeviceSupport: MobileBroadbandModemStatus = MobileBroadbandModemStatus(3i32);
}
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
#[repr(transparent)]
pub struct MobileBroadbandPinFormat(pub i32);
impl MobileBroadbandPinFormat {
    pub const Unknown: MobileBroadbandPinFormat = MobileBroadbandPinFormat(0i32);
    pub const Numeric: MobileBroadbandPinFormat = MobileBroadbandPinFormat(1i32);
    pub const Alphanumeric: MobileBroadbandPinFormat = MobileBroadbandPinFormat(2i32);
}
#[repr(transparent)]
pub struct MobileBroadbandPinLockState(pub i32);
impl MobileBroadbandPinLockState {
    pub const Unknown: MobileBroadbandPinLockState = MobileBroadbandPinLockState(0i32);
    pub const Unlocked: MobileBroadbandPinLockState = MobileBroadbandPinLockState(1i32);
    pub const PinRequired: MobileBroadbandPinLockState = MobileBroadbandPinLockState(2i32);
    pub const PinUnblockKeyRequired: MobileBroadbandPinLockState = MobileBroadbandPinLockState(3i32);
}
#[repr(transparent)]
pub struct MobileBroadbandPinLockStateChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandPinLockStateChangeTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandPinManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandPinOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandPinType(pub i32);
impl MobileBroadbandPinType {
    pub const None: MobileBroadbandPinType = MobileBroadbandPinType(0i32);
    pub const Custom: MobileBroadbandPinType = MobileBroadbandPinType(1i32);
    pub const Pin1: MobileBroadbandPinType = MobileBroadbandPinType(2i32);
    pub const Pin2: MobileBroadbandPinType = MobileBroadbandPinType(3i32);
    pub const SimPin: MobileBroadbandPinType = MobileBroadbandPinType(4i32);
    pub const FirstSimPin: MobileBroadbandPinType = MobileBroadbandPinType(5i32);
    pub const NetworkPin: MobileBroadbandPinType = MobileBroadbandPinType(6i32);
    pub const NetworkSubsetPin: MobileBroadbandPinType = MobileBroadbandPinType(7i32);
    pub const ServiceProviderPin: MobileBroadbandPinType = MobileBroadbandPinType(8i32);
    pub const CorporatePin: MobileBroadbandPinType = MobileBroadbandPinType(9i32);
    pub const SubsidyLock: MobileBroadbandPinType = MobileBroadbandPinType(10i32);
}
#[repr(transparent)]
pub struct MobileBroadbandRadioState(pub i32);
impl MobileBroadbandRadioState {
    pub const Off: MobileBroadbandRadioState = MobileBroadbandRadioState(0i32);
    pub const On: MobileBroadbandRadioState = MobileBroadbandRadioState(1i32);
}
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
#[repr(transparent)]
pub struct MobileBroadbandSlotState(pub i32);
impl MobileBroadbandSlotState {
    pub const Unmanaged: MobileBroadbandSlotState = MobileBroadbandSlotState(0i32);
    pub const Unknown: MobileBroadbandSlotState = MobileBroadbandSlotState(1i32);
    pub const OffEmpty: MobileBroadbandSlotState = MobileBroadbandSlotState(2i32);
    pub const Off: MobileBroadbandSlotState = MobileBroadbandSlotState(3i32);
    pub const Empty: MobileBroadbandSlotState = MobileBroadbandSlotState(4i32);
    pub const NotReady: MobileBroadbandSlotState = MobileBroadbandSlotState(5i32);
    pub const Active: MobileBroadbandSlotState = MobileBroadbandSlotState(6i32);
    pub const Error: MobileBroadbandSlotState = MobileBroadbandSlotState(7i32);
    pub const ActiveEsim: MobileBroadbandSlotState = MobileBroadbandSlotState(8i32);
    pub const ActiveEsimNoProfile: MobileBroadbandSlotState = MobileBroadbandSlotState(9i32);
}
#[repr(transparent)]
pub struct MobileBroadbandTransmissionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandUicc(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandUiccApp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandUiccAppOperationStatus(pub i32);
impl MobileBroadbandUiccAppOperationStatus {
    pub const Success: MobileBroadbandUiccAppOperationStatus = MobileBroadbandUiccAppOperationStatus(0i32);
    pub const InvalidUiccFilePath: MobileBroadbandUiccAppOperationStatus = MobileBroadbandUiccAppOperationStatus(1i32);
    pub const AccessConditionNotHeld: MobileBroadbandUiccAppOperationStatus = MobileBroadbandUiccAppOperationStatus(2i32);
    pub const UiccBusy: MobileBroadbandUiccAppOperationStatus = MobileBroadbandUiccAppOperationStatus(3i32);
}
#[repr(transparent)]
pub struct MobileBroadbandUiccAppReadRecordResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandUiccAppRecordDetailsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandUiccAppsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkDeviceStatus(pub i32);
impl NetworkDeviceStatus {
    pub const DeviceNotReady: NetworkDeviceStatus = NetworkDeviceStatus(0i32);
    pub const DeviceReady: NetworkDeviceStatus = NetworkDeviceStatus(1i32);
    pub const SimNotInserted: NetworkDeviceStatus = NetworkDeviceStatus(2i32);
    pub const BadSim: NetworkDeviceStatus = NetworkDeviceStatus(3i32);
    pub const DeviceHardwareFailure: NetworkDeviceStatus = NetworkDeviceStatus(4i32);
    pub const AccountNotActivated: NetworkDeviceStatus = NetworkDeviceStatus(5i32);
    pub const DeviceLocked: NetworkDeviceStatus = NetworkDeviceStatus(6i32);
    pub const DeviceBlocked: NetworkDeviceStatus = NetworkDeviceStatus(7i32);
}
#[repr(transparent)]
pub struct NetworkOperatorDataUsageNotificationKind(pub i32);
impl NetworkOperatorDataUsageNotificationKind {
    pub const DataUsageProgress: NetworkOperatorDataUsageNotificationKind = NetworkOperatorDataUsageNotificationKind(0i32);
}
#[repr(transparent)]
pub struct NetworkOperatorDataUsageTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkOperatorEventMessageType(pub i32);
impl NetworkOperatorEventMessageType {
    pub const Gsm: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(0i32);
    pub const Cdma: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(1i32);
    pub const Ussd: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(2i32);
    pub const DataPlanThresholdReached: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(3i32);
    pub const DataPlanReset: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(4i32);
    pub const DataPlanDeleted: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(5i32);
    pub const ProfileConnected: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(6i32);
    pub const ProfileDisconnected: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(7i32);
    pub const RegisteredRoaming: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(8i32);
    pub const RegisteredHome: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(9i32);
    pub const TetheringEntitlementCheck: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(10i32);
    pub const TetheringOperationalStateChanged: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(11i32);
    pub const TetheringNumberOfClientsChanged: NetworkOperatorEventMessageType = NetworkOperatorEventMessageType(12i32);
}
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
#[repr(transparent)]
pub struct NetworkRegistrationState(pub i32);
impl NetworkRegistrationState {
    pub const None: NetworkRegistrationState = NetworkRegistrationState(0i32);
    pub const Deregistered: NetworkRegistrationState = NetworkRegistrationState(1i32);
    pub const Searching: NetworkRegistrationState = NetworkRegistrationState(2i32);
    pub const Home: NetworkRegistrationState = NetworkRegistrationState(3i32);
    pub const Roaming: NetworkRegistrationState = NetworkRegistrationState(4i32);
    pub const Partner: NetworkRegistrationState = NetworkRegistrationState(5i32);
    pub const Denied: NetworkRegistrationState = NetworkRegistrationState(6i32);
}
#[repr(transparent)]
pub struct ProfileMediaType(pub i32);
impl ProfileMediaType {
    pub const Wlan: ProfileMediaType = ProfileMediaType(0i32);
    pub const Wwan: ProfileMediaType = ProfileMediaType(1i32);
}
#[cfg(feature = "Foundation")]
#[repr(C)]
pub struct ProfileUsage(i32);
#[repr(transparent)]
pub struct ProvisionFromXmlDocumentResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProvisionedProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProvisioningAgent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TetheringCapability(pub i32);
impl TetheringCapability {
    pub const Enabled: TetheringCapability = TetheringCapability(0i32);
    pub const DisabledByGroupPolicy: TetheringCapability = TetheringCapability(1i32);
    pub const DisabledByHardwareLimitation: TetheringCapability = TetheringCapability(2i32);
    pub const DisabledByOperator: TetheringCapability = TetheringCapability(3i32);
    pub const DisabledBySku: TetheringCapability = TetheringCapability(4i32);
    pub const DisabledByRequiredAppNotInstalled: TetheringCapability = TetheringCapability(5i32);
    pub const DisabledDueToUnknownCause: TetheringCapability = TetheringCapability(6i32);
    pub const DisabledBySystemCapability: TetheringCapability = TetheringCapability(7i32);
}
#[repr(transparent)]
pub struct TetheringEntitlementCheckTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TetheringOperationStatus(pub i32);
impl TetheringOperationStatus {
    pub const Success: TetheringOperationStatus = TetheringOperationStatus(0i32);
    pub const Unknown: TetheringOperationStatus = TetheringOperationStatus(1i32);
    pub const MobileBroadbandDeviceOff: TetheringOperationStatus = TetheringOperationStatus(2i32);
    pub const WiFiDeviceOff: TetheringOperationStatus = TetheringOperationStatus(3i32);
    pub const EntitlementCheckTimeout: TetheringOperationStatus = TetheringOperationStatus(4i32);
    pub const EntitlementCheckFailure: TetheringOperationStatus = TetheringOperationStatus(5i32);
    pub const OperationInProgress: TetheringOperationStatus = TetheringOperationStatus(6i32);
    pub const BluetoothDeviceOff: TetheringOperationStatus = TetheringOperationStatus(7i32);
    pub const NetworkLimitedConnectivity: TetheringOperationStatus = TetheringOperationStatus(8i32);
}
#[repr(transparent)]
pub struct TetheringOperationalState(pub i32);
impl TetheringOperationalState {
    pub const Unknown: TetheringOperationalState = TetheringOperationalState(0i32);
    pub const On: TetheringOperationalState = TetheringOperationalState(1i32);
    pub const Off: TetheringOperationalState = TetheringOperationalState(2i32);
    pub const InTransition: TetheringOperationalState = TetheringOperationalState(3i32);
}
#[repr(transparent)]
pub struct TetheringWiFiBand(pub i32);
impl TetheringWiFiBand {
    pub const Auto: TetheringWiFiBand = TetheringWiFiBand(0i32);
    pub const TwoPointFourGigahertz: TetheringWiFiBand = TetheringWiFiBand(1i32);
    pub const FiveGigahertz: TetheringWiFiBand = TetheringWiFiBand(2i32);
}
#[repr(transparent)]
pub struct UiccAccessCondition(pub i32);
impl UiccAccessCondition {
    pub const AlwaysAllowed: UiccAccessCondition = UiccAccessCondition(0i32);
    pub const Pin1: UiccAccessCondition = UiccAccessCondition(1i32);
    pub const Pin2: UiccAccessCondition = UiccAccessCondition(2i32);
    pub const Pin3: UiccAccessCondition = UiccAccessCondition(3i32);
    pub const Pin4: UiccAccessCondition = UiccAccessCondition(4i32);
    pub const Administrative5: UiccAccessCondition = UiccAccessCondition(5i32);
    pub const Administrative6: UiccAccessCondition = UiccAccessCondition(6i32);
    pub const NeverAllowed: UiccAccessCondition = UiccAccessCondition(7i32);
}
#[repr(transparent)]
pub struct UiccAppKind(pub i32);
impl UiccAppKind {
    pub const Unknown: UiccAppKind = UiccAppKind(0i32);
    pub const MF: UiccAppKind = UiccAppKind(1i32);
    pub const MFSim: UiccAppKind = UiccAppKind(2i32);
    pub const MFRuim: UiccAppKind = UiccAppKind(3i32);
    pub const USim: UiccAppKind = UiccAppKind(4i32);
    pub const CSim: UiccAppKind = UiccAppKind(5i32);
    pub const ISim: UiccAppKind = UiccAppKind(6i32);
}
#[repr(transparent)]
pub struct UiccAppRecordKind(pub i32);
impl UiccAppRecordKind {
    pub const Unknown: UiccAppRecordKind = UiccAppRecordKind(0i32);
    pub const Transparent: UiccAppRecordKind = UiccAppRecordKind(1i32);
    pub const RecordOriented: UiccAppRecordKind = UiccAppRecordKind(2i32);
}
#[repr(transparent)]
pub struct UssdMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UssdReply(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UssdResultCode(pub i32);
impl UssdResultCode {
    pub const NoActionRequired: UssdResultCode = UssdResultCode(0i32);
    pub const ActionRequired: UssdResultCode = UssdResultCode(1i32);
    pub const Terminated: UssdResultCode = UssdResultCode(2i32);
    pub const OtherLocalClient: UssdResultCode = UssdResultCode(3i32);
    pub const OperationNotSupported: UssdResultCode = UssdResultCode(4i32);
    pub const NetworkTimeout: UssdResultCode = UssdResultCode(5i32);
}
#[repr(transparent)]
pub struct UssdSession(pub *mut ::core::ffi::c_void);
