#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DataClasses(pub u32);
impl DataClasses {
    pub const None: Self = Self(0u32);
    pub const Gprs: Self = Self(1u32);
    pub const Edge: Self = Self(2u32);
    pub const Umts: Self = Self(4u32);
    pub const Hsdpa: Self = Self(8u32);
    pub const Hsupa: Self = Self(16u32);
    pub const LteAdvanced: Self = Self(32u32);
    pub const NewRadioNonStandalone: Self = Self(64u32);
    pub const NewRadioStandalone: Self = Self(128u32);
    pub const Cdma1xRtt: Self = Self(65536u32);
    pub const Cdma1xEvdo: Self = Self(131072u32);
    pub const Cdma1xEvdoRevA: Self = Self(262144u32);
    pub const Cdma1xEvdv: Self = Self(524288u32);
    pub const Cdma3xRtt: Self = Self(1048576u32);
    pub const Cdma1xEvdoRevB: Self = Self(2097152u32);
    pub const CdmaUmb: Self = Self(4194304u32);
    pub const Custom: Self = Self(2147483648u32);
}
#[repr(transparent)]
pub struct ESim(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimAuthenticationPreference(pub i32);
impl ESimAuthenticationPreference {
    pub const OnEntry: Self = Self(0i32);
    pub const OnAction: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ESimDiscoverEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimDiscoverResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimDiscoverResultKind(pub i32);
impl ESimDiscoverResultKind {
    pub const None: Self = Self(0i32);
    pub const Events: Self = Self(1i32);
    pub const ProfileMetadata: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ESimDownloadProfileMetadataResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimOperationStatus(pub i32);
impl ESimOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const NotAuthorized: Self = Self(1i32);
    pub const NotFound: Self = Self(2i32);
    pub const PolicyViolation: Self = Self(3i32);
    pub const InsufficientSpaceOnCard: Self = Self(4i32);
    pub const ServerFailure: Self = Self(5i32);
    pub const ServerNotReachable: Self = Self(6i32);
    pub const TimeoutWaitingForUserConsent: Self = Self(7i32);
    pub const IncorrectConfirmationCode: Self = Self(8i32);
    pub const ConfirmationCodeMaxRetriesExceeded: Self = Self(9i32);
    pub const CardRemoved: Self = Self(10i32);
    pub const CardBusy: Self = Self(11i32);
    pub const Other: Self = Self(12i32);
    pub const CardGeneralFailure: Self = Self(13i32);
    pub const ConfirmationCodeMissing: Self = Self(14i32);
    pub const InvalidMatchingId: Self = Self(15i32);
    pub const NoEligibleProfileForThisDevice: Self = Self(16i32);
    pub const OperationAborted: Self = Self(17i32);
    pub const EidMismatch: Self = Self(18i32);
    pub const ProfileNotAvailableForNewBinding: Self = Self(19i32);
    pub const ProfileNotReleasedByOperator: Self = Self(20i32);
    pub const OperationProhibitedByProfileClass: Self = Self(21i32);
    pub const ProfileNotPresent: Self = Self(22i32);
    pub const NoCorrespondingRequest: Self = Self(23i32);
    pub const TimeoutWaitingForResponse: Self = Self(24i32);
    pub const IccidAlreadyExists: Self = Self(25i32);
    pub const ProfileProcessingError: Self = Self(26i32);
    pub const ServerNotTrusted: Self = Self(27i32);
    pub const ProfileDownloadMaxRetriesExceeded: Self = Self(28i32);
}
#[repr(transparent)]
pub struct ESimPolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimProfileClass(pub i32);
impl ESimProfileClass {
    pub const Operational: Self = Self(0i32);
    pub const Test: Self = Self(1i32);
    pub const Provisioning: Self = Self(2i32);
}
#[repr(C)]
pub struct ESimProfileInstallProgress(i32);
#[repr(transparent)]
pub struct ESimProfileMetadata(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimProfileMetadataState(pub i32);
impl ESimProfileMetadataState {
    pub const Unknown: Self = Self(0i32);
    pub const WaitingForInstall: Self = Self(1i32);
    pub const Downloading: Self = Self(2i32);
    pub const Installing: Self = Self(3i32);
    pub const Expired: Self = Self(4i32);
    pub const RejectingDownload: Self = Self(5i32);
    pub const NoLongerAvailable: Self = Self(6i32);
    pub const DeniedByPolicy: Self = Self(7i32);
}
#[repr(transparent)]
pub struct ESimProfilePolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimProfileState(pub i32);
impl ESimProfileState {
    pub const Unknown: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
    pub const Deleted: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ESimRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimServiceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimState(pub i32);
impl ESimState {
    pub const Unknown: Self = Self(0i32);
    pub const Idle: Self = Self(1i32);
    pub const Removed: Self = Self(2i32);
    pub const Busy: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ESimUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ESimWatcherStatus(pub i32);
impl ESimWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
}
#[repr(transparent)]
pub struct HotspotAuthenticationContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HotspotAuthenticationEventDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HotspotAuthenticationResponseCode(pub i32);
impl HotspotAuthenticationResponseCode {
    pub const NoError: Self = Self(0i32);
    pub const LoginSucceeded: Self = Self(50i32);
    pub const LoginFailed: Self = Self(100i32);
    pub const RadiusServerError: Self = Self(102i32);
    pub const NetworkAdministratorError: Self = Self(105i32);
    pub const LoginAborted: Self = Self(151i32);
    pub const AccessGatewayInternalError: Self = Self(255i32);
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
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Aborted: Self = Self(4i32);
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
    pub const Unknown: Self = Self(0i32);
    pub const Embedded: Self = Self(1i32);
    pub const Removable: Self = Self(2i32);
    pub const Remote: Self = Self(3i32);
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
    pub const Success: Self = Self(0i32);
    pub const OtherFailure: Self = Self(1i32);
    pub const Busy: Self = Self(2i32);
    pub const NoDeviceSupport: Self = Self(3i32);
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
    pub const Unknown: Self = Self(0i32);
    pub const Numeric: Self = Self(1i32);
    pub const Alphanumeric: Self = Self(2i32);
}
#[repr(transparent)]
pub struct MobileBroadbandPinLockState(pub i32);
impl MobileBroadbandPinLockState {
    pub const Unknown: Self = Self(0i32);
    pub const Unlocked: Self = Self(1i32);
    pub const PinRequired: Self = Self(2i32);
    pub const PinUnblockKeyRequired: Self = Self(3i32);
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
    pub const None: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
    pub const Pin1: Self = Self(2i32);
    pub const Pin2: Self = Self(3i32);
    pub const SimPin: Self = Self(4i32);
    pub const FirstSimPin: Self = Self(5i32);
    pub const NetworkPin: Self = Self(6i32);
    pub const NetworkSubsetPin: Self = Self(7i32);
    pub const ServiceProviderPin: Self = Self(8i32);
    pub const CorporatePin: Self = Self(9i32);
    pub const SubsidyLock: Self = Self(10i32);
}
#[repr(transparent)]
pub struct MobileBroadbandRadioState(pub i32);
impl MobileBroadbandRadioState {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
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
    pub const Unmanaged: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const OffEmpty: Self = Self(2i32);
    pub const Off: Self = Self(3i32);
    pub const Empty: Self = Self(4i32);
    pub const NotReady: Self = Self(5i32);
    pub const Active: Self = Self(6i32);
    pub const Error: Self = Self(7i32);
    pub const ActiveEsim: Self = Self(8i32);
    pub const ActiveEsimNoProfile: Self = Self(9i32);
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
    pub const Success: Self = Self(0i32);
    pub const InvalidUiccFilePath: Self = Self(1i32);
    pub const AccessConditionNotHeld: Self = Self(2i32);
    pub const UiccBusy: Self = Self(3i32);
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
    pub const DeviceNotReady: Self = Self(0i32);
    pub const DeviceReady: Self = Self(1i32);
    pub const SimNotInserted: Self = Self(2i32);
    pub const BadSim: Self = Self(3i32);
    pub const DeviceHardwareFailure: Self = Self(4i32);
    pub const AccountNotActivated: Self = Self(5i32);
    pub const DeviceLocked: Self = Self(6i32);
    pub const DeviceBlocked: Self = Self(7i32);
}
#[repr(transparent)]
pub struct NetworkOperatorDataUsageNotificationKind(pub i32);
impl NetworkOperatorDataUsageNotificationKind {
    pub const DataUsageProgress: Self = Self(0i32);
}
#[repr(transparent)]
pub struct NetworkOperatorDataUsageTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkOperatorEventMessageType(pub i32);
impl NetworkOperatorEventMessageType {
    pub const Gsm: Self = Self(0i32);
    pub const Cdma: Self = Self(1i32);
    pub const Ussd: Self = Self(2i32);
    pub const DataPlanThresholdReached: Self = Self(3i32);
    pub const DataPlanReset: Self = Self(4i32);
    pub const DataPlanDeleted: Self = Self(5i32);
    pub const ProfileConnected: Self = Self(6i32);
    pub const ProfileDisconnected: Self = Self(7i32);
    pub const RegisteredRoaming: Self = Self(8i32);
    pub const RegisteredHome: Self = Self(9i32);
    pub const TetheringEntitlementCheck: Self = Self(10i32);
    pub const TetheringOperationalStateChanged: Self = Self(11i32);
    pub const TetheringNumberOfClientsChanged: Self = Self(12i32);
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
    pub const None: Self = Self(0i32);
    pub const Deregistered: Self = Self(1i32);
    pub const Searching: Self = Self(2i32);
    pub const Home: Self = Self(3i32);
    pub const Roaming: Self = Self(4i32);
    pub const Partner: Self = Self(5i32);
    pub const Denied: Self = Self(6i32);
}
#[repr(transparent)]
pub struct ProfileMediaType(pub i32);
impl ProfileMediaType {
    pub const Wlan: Self = Self(0i32);
    pub const Wwan: Self = Self(1i32);
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
    pub const Enabled: Self = Self(0i32);
    pub const DisabledByGroupPolicy: Self = Self(1i32);
    pub const DisabledByHardwareLimitation: Self = Self(2i32);
    pub const DisabledByOperator: Self = Self(3i32);
    pub const DisabledBySku: Self = Self(4i32);
    pub const DisabledByRequiredAppNotInstalled: Self = Self(5i32);
    pub const DisabledDueToUnknownCause: Self = Self(6i32);
    pub const DisabledBySystemCapability: Self = Self(7i32);
}
#[repr(transparent)]
pub struct TetheringEntitlementCheckTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TetheringOperationStatus(pub i32);
impl TetheringOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const MobileBroadbandDeviceOff: Self = Self(2i32);
    pub const WiFiDeviceOff: Self = Self(3i32);
    pub const EntitlementCheckTimeout: Self = Self(4i32);
    pub const EntitlementCheckFailure: Self = Self(5i32);
    pub const OperationInProgress: Self = Self(6i32);
    pub const BluetoothDeviceOff: Self = Self(7i32);
    pub const NetworkLimitedConnectivity: Self = Self(8i32);
}
#[repr(transparent)]
pub struct TetheringOperationalState(pub i32);
impl TetheringOperationalState {
    pub const Unknown: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const InTransition: Self = Self(3i32);
}
#[repr(transparent)]
pub struct TetheringWiFiBand(pub i32);
impl TetheringWiFiBand {
    pub const Auto: Self = Self(0i32);
    pub const TwoPointFourGigahertz: Self = Self(1i32);
    pub const FiveGigahertz: Self = Self(2i32);
}
#[repr(transparent)]
pub struct UiccAccessCondition(pub i32);
impl UiccAccessCondition {
    pub const AlwaysAllowed: Self = Self(0i32);
    pub const Pin1: Self = Self(1i32);
    pub const Pin2: Self = Self(2i32);
    pub const Pin3: Self = Self(3i32);
    pub const Pin4: Self = Self(4i32);
    pub const Administrative5: Self = Self(5i32);
    pub const Administrative6: Self = Self(6i32);
    pub const NeverAllowed: Self = Self(7i32);
}
#[repr(transparent)]
pub struct UiccAppKind(pub i32);
impl UiccAppKind {
    pub const Unknown: Self = Self(0i32);
    pub const MF: Self = Self(1i32);
    pub const MFSim: Self = Self(2i32);
    pub const MFRuim: Self = Self(3i32);
    pub const USim: Self = Self(4i32);
    pub const CSim: Self = Self(5i32);
    pub const ISim: Self = Self(6i32);
}
#[repr(transparent)]
pub struct UiccAppRecordKind(pub i32);
impl UiccAppRecordKind {
    pub const Unknown: Self = Self(0i32);
    pub const Transparent: Self = Self(1i32);
    pub const RecordOriented: Self = Self(2i32);
}
#[repr(transparent)]
pub struct UssdMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UssdReply(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UssdResultCode(pub i32);
impl UssdResultCode {
    pub const NoActionRequired: Self = Self(0i32);
    pub const ActionRequired: Self = Self(1i32);
    pub const Terminated: Self = Self(2i32);
    pub const OtherLocalClient: Self = Self(3i32);
    pub const OperationNotSupported: Self = Self(4i32);
    pub const NetworkTimeout: Self = Self(5i32);
}
#[repr(transparent)]
pub struct UssdSession(pub *mut ::core::ffi::c_void);
