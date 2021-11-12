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
impl ::core::marker::Copy for DataClasses {}
impl ::core::clone::Clone for DataClasses {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESim(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ESim {}
impl ::core::clone::Clone for ESim {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ESimAddedEventArgs {}
impl ::core::clone::Clone for ESimAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimAuthenticationPreference(pub i32);
impl ESimAuthenticationPreference {
    pub const OnEntry: Self = Self(0i32);
    pub const OnAction: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
impl ::core::marker::Copy for ESimAuthenticationPreference {}
impl ::core::clone::Clone for ESimAuthenticationPreference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimDiscoverEvent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ESimDiscoverEvent {}
impl ::core::clone::Clone for ESimDiscoverEvent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimDiscoverResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ESimDiscoverResult {}
impl ::core::clone::Clone for ESimDiscoverResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimDiscoverResultKind(pub i32);
impl ESimDiscoverResultKind {
    pub const None: Self = Self(0i32);
    pub const Events: Self = Self(1i32);
    pub const ProfileMetadata: Self = Self(2i32);
}
impl ::core::marker::Copy for ESimDiscoverResultKind {}
impl ::core::clone::Clone for ESimDiscoverResultKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimDownloadProfileMetadataResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ESimDownloadProfileMetadataResult {}
impl ::core::clone::Clone for ESimDownloadProfileMetadataResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimOperationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ESimOperationResult {}
impl ::core::clone::Clone for ESimOperationResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ESimOperationStatus {}
impl ::core::clone::Clone for ESimOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimPolicy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ESimPolicy {}
impl ::core::clone::Clone for ESimPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimProfile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ESimProfile {}
impl ::core::clone::Clone for ESimProfile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimProfileClass(pub i32);
impl ESimProfileClass {
    pub const Operational: Self = Self(0i32);
    pub const Test: Self = Self(1i32);
    pub const Provisioning: Self = Self(2i32);
}
impl ::core::marker::Copy for ESimProfileClass {}
impl ::core::clone::Clone for ESimProfileClass {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ESimProfileInstallProgress {
    pub TotalSizeInBytes: i32,
    pub InstalledSizeInBytes: i32,
}
impl ::core::marker::Copy for ESimProfileInstallProgress {}
impl ::core::clone::Clone for ESimProfileInstallProgress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimProfileMetadata(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ESimProfileMetadata {}
impl ::core::clone::Clone for ESimProfileMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ESimProfileMetadataState {}
impl ::core::clone::Clone for ESimProfileMetadataState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimProfilePolicy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ESimProfilePolicy {}
impl ::core::clone::Clone for ESimProfilePolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimProfileState(pub i32);
impl ESimProfileState {
    pub const Unknown: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
    pub const Deleted: Self = Self(3i32);
}
impl ::core::marker::Copy for ESimProfileState {}
impl ::core::clone::Clone for ESimProfileState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ESimRemovedEventArgs {}
impl ::core::clone::Clone for ESimRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimServiceInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ESimServiceInfo {}
impl ::core::clone::Clone for ESimServiceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimState(pub i32);
impl ESimState {
    pub const Unknown: Self = Self(0i32);
    pub const Idle: Self = Self(1i32);
    pub const Removed: Self = Self(2i32);
    pub const Busy: Self = Self(3i32);
}
impl ::core::marker::Copy for ESimState {}
impl ::core::clone::Clone for ESimState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ESimUpdatedEventArgs {}
impl ::core::clone::Clone for ESimUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ESimWatcher {}
impl ::core::clone::Clone for ESimWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ESimWatcherStatus(pub i32);
impl ESimWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
}
impl ::core::marker::Copy for ESimWatcherStatus {}
impl ::core::clone::Clone for ESimWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HotspotAuthenticationContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HotspotAuthenticationContext {}
impl ::core::clone::Clone for HotspotAuthenticationContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HotspotAuthenticationEventDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HotspotAuthenticationEventDetails {}
impl ::core::clone::Clone for HotspotAuthenticationEventDetails {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for HotspotAuthenticationResponseCode {}
impl ::core::clone::Clone for HotspotAuthenticationResponseCode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HotspotCredentialsAuthenticationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HotspotCredentialsAuthenticationResult {}
impl ::core::clone::Clone for HotspotCredentialsAuthenticationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IESim(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IESim {}
impl ::core::clone::Clone for IESim {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IESim2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IESim2 {}
impl ::core::clone::Clone for IESim2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IESimAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IESimAddedEventArgs {}
impl ::core::clone::Clone for IESimAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IESimDiscoverEvent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IESimDiscoverEvent {}
impl ::core::clone::Clone for IESimDiscoverEvent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IESimDiscoverResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IESimDiscoverResult {}
impl ::core::clone::Clone for IESimDiscoverResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IESimDownloadProfileMetadataResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IESimDownloadProfileMetadataResult {}
impl ::core::clone::Clone for IESimDownloadProfileMetadataResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IESimManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IESimManagerStatics {}
impl ::core::clone::Clone for IESimManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IESimOperationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IESimOperationResult {}
impl ::core::clone::Clone for IESimOperationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IESimPolicy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IESimPolicy {}
impl ::core::clone::Clone for IESimPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IESimProfile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IESimProfile {}
impl ::core::clone::Clone for IESimProfile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IESimProfileMetadata(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IESimProfileMetadata {}
impl ::core::clone::Clone for IESimProfileMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IESimProfilePolicy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IESimProfilePolicy {}
impl ::core::clone::Clone for IESimProfilePolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IESimRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IESimRemovedEventArgs {}
impl ::core::clone::Clone for IESimRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IESimServiceInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IESimServiceInfo {}
impl ::core::clone::Clone for IESimServiceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IESimUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IESimUpdatedEventArgs {}
impl ::core::clone::Clone for IESimUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IESimWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IESimWatcher {}
impl ::core::clone::Clone for IESimWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFdnAccessManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFdnAccessManagerStatics {}
impl ::core::clone::Clone for IFdnAccessManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHotspotAuthenticationContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHotspotAuthenticationContext {}
impl ::core::clone::Clone for IHotspotAuthenticationContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHotspotAuthenticationContext2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHotspotAuthenticationContext2 {}
impl ::core::clone::Clone for IHotspotAuthenticationContext2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHotspotAuthenticationContextStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHotspotAuthenticationContextStatics {}
impl ::core::clone::Clone for IHotspotAuthenticationContextStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHotspotAuthenticationEventDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHotspotAuthenticationEventDetails {}
impl ::core::clone::Clone for IHotspotAuthenticationEventDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHotspotCredentialsAuthenticationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHotspotCredentialsAuthenticationResult {}
impl ::core::clone::Clone for IHotspotCredentialsAuthenticationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownCSimFilePathsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownCSimFilePathsStatics {}
impl ::core::clone::Clone for IKnownCSimFilePathsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownRuimFilePathsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownRuimFilePathsStatics {}
impl ::core::clone::Clone for IKnownRuimFilePathsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownSimFilePathsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownSimFilePathsStatics {}
impl ::core::clone::Clone for IKnownSimFilePathsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownUSimFilePathsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownUSimFilePathsStatics {}
impl ::core::clone::Clone for IKnownUSimFilePathsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandAccount(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandAccount {}
impl ::core::clone::Clone for IMobileBroadbandAccount {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandAccount2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandAccount2 {}
impl ::core::clone::Clone for IMobileBroadbandAccount2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandAccount3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandAccount3 {}
impl ::core::clone::Clone for IMobileBroadbandAccount3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandAccountEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandAccountEventArgs {}
impl ::core::clone::Clone for IMobileBroadbandAccountEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandAccountStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandAccountStatics {}
impl ::core::clone::Clone for IMobileBroadbandAccountStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandAccountUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandAccountUpdatedEventArgs {}
impl ::core::clone::Clone for IMobileBroadbandAccountUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandAccountWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandAccountWatcher {}
impl ::core::clone::Clone for IMobileBroadbandAccountWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandAntennaSar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandAntennaSar {}
impl ::core::clone::Clone for IMobileBroadbandAntennaSar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandAntennaSarFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandAntennaSarFactory {}
impl ::core::clone::Clone for IMobileBroadbandAntennaSarFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandCellCdma(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandCellCdma {}
impl ::core::clone::Clone for IMobileBroadbandCellCdma {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandCellGsm(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandCellGsm {}
impl ::core::clone::Clone for IMobileBroadbandCellGsm {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandCellLte(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandCellLte {}
impl ::core::clone::Clone for IMobileBroadbandCellLte {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandCellNR(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandCellNR {}
impl ::core::clone::Clone for IMobileBroadbandCellNR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandCellTdscdma(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandCellTdscdma {}
impl ::core::clone::Clone for IMobileBroadbandCellTdscdma {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandCellUmts(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandCellUmts {}
impl ::core::clone::Clone for IMobileBroadbandCellUmts {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandCellsInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandCellsInfo {}
impl ::core::clone::Clone for IMobileBroadbandCellsInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandCellsInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandCellsInfo2 {}
impl ::core::clone::Clone for IMobileBroadbandCellsInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandCurrentSlotIndexChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandCurrentSlotIndexChangedEventArgs {}
impl ::core::clone::Clone for IMobileBroadbandCurrentSlotIndexChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandDeviceInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandDeviceInformation {}
impl ::core::clone::Clone for IMobileBroadbandDeviceInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandDeviceInformation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandDeviceInformation2 {}
impl ::core::clone::Clone for IMobileBroadbandDeviceInformation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandDeviceInformation3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandDeviceInformation3 {}
impl ::core::clone::Clone for IMobileBroadbandDeviceInformation3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandDeviceInformation4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandDeviceInformation4 {}
impl ::core::clone::Clone for IMobileBroadbandDeviceInformation4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandDeviceService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandDeviceService {}
impl ::core::clone::Clone for IMobileBroadbandDeviceService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceCommandResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandDeviceServiceCommandResult {}
impl ::core::clone::Clone for IMobileBroadbandDeviceServiceCommandResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceCommandSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandDeviceServiceCommandSession {}
impl ::core::clone::Clone for IMobileBroadbandDeviceServiceCommandSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandDeviceServiceDataReceivedEventArgs {}
impl ::core::clone::Clone for IMobileBroadbandDeviceServiceDataReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceDataSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandDeviceServiceDataSession {}
impl ::core::clone::Clone for IMobileBroadbandDeviceServiceDataSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandDeviceServiceInformation {}
impl ::core::clone::Clone for IMobileBroadbandDeviceServiceInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandDeviceServiceTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandDeviceServiceTriggerDetails {}
impl ::core::clone::Clone for IMobileBroadbandDeviceServiceTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandModem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandModem {}
impl ::core::clone::Clone for IMobileBroadbandModem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandModem2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandModem2 {}
impl ::core::clone::Clone for IMobileBroadbandModem2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandModem3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandModem3 {}
impl ::core::clone::Clone for IMobileBroadbandModem3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandModemConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandModemConfiguration {}
impl ::core::clone::Clone for IMobileBroadbandModemConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandModemConfiguration2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandModemConfiguration2 {}
impl ::core::clone::Clone for IMobileBroadbandModemConfiguration2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandModemIsolation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandModemIsolation {}
impl ::core::clone::Clone for IMobileBroadbandModemIsolation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandModemIsolationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandModemIsolationFactory {}
impl ::core::clone::Clone for IMobileBroadbandModemIsolationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandModemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandModemStatics {}
impl ::core::clone::Clone for IMobileBroadbandModemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandNetwork(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandNetwork {}
impl ::core::clone::Clone for IMobileBroadbandNetwork {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandNetwork2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandNetwork2 {}
impl ::core::clone::Clone for IMobileBroadbandNetwork2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandNetwork3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandNetwork3 {}
impl ::core::clone::Clone for IMobileBroadbandNetwork3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandNetworkRegistrationStateChange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandNetworkRegistrationStateChange {}
impl ::core::clone::Clone for IMobileBroadbandNetworkRegistrationStateChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails {}
impl ::core::clone::Clone for IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandPco(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandPco {}
impl ::core::clone::Clone for IMobileBroadbandPco {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandPcoDataChangeTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandPcoDataChangeTriggerDetails {}
impl ::core::clone::Clone for IMobileBroadbandPcoDataChangeTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandPin(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandPin {}
impl ::core::clone::Clone for IMobileBroadbandPin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandPinLockStateChange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandPinLockStateChange {}
impl ::core::clone::Clone for IMobileBroadbandPinLockStateChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandPinLockStateChangeTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandPinLockStateChangeTriggerDetails {}
impl ::core::clone::Clone for IMobileBroadbandPinLockStateChangeTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandPinManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandPinManager {}
impl ::core::clone::Clone for IMobileBroadbandPinManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandPinOperationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandPinOperationResult {}
impl ::core::clone::Clone for IMobileBroadbandPinOperationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandRadioStateChange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandRadioStateChange {}
impl ::core::clone::Clone for IMobileBroadbandRadioStateChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandRadioStateChangeTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandRadioStateChangeTriggerDetails {}
impl ::core::clone::Clone for IMobileBroadbandRadioStateChangeTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandSarManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandSarManager {}
impl ::core::clone::Clone for IMobileBroadbandSarManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandSlotInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandSlotInfo {}
impl ::core::clone::Clone for IMobileBroadbandSlotInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandSlotInfoChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandSlotInfoChangedEventArgs {}
impl ::core::clone::Clone for IMobileBroadbandSlotInfoChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandSlotManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandSlotManager {}
impl ::core::clone::Clone for IMobileBroadbandSlotManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandTransmissionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandTransmissionStateChangedEventArgs {}
impl ::core::clone::Clone for IMobileBroadbandTransmissionStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandUicc(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandUicc {}
impl ::core::clone::Clone for IMobileBroadbandUicc {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandUiccApp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandUiccApp {}
impl ::core::clone::Clone for IMobileBroadbandUiccApp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandUiccAppReadRecordResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandUiccAppReadRecordResult {}
impl ::core::clone::Clone for IMobileBroadbandUiccAppReadRecordResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandUiccAppRecordDetailsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandUiccAppRecordDetailsResult {}
impl ::core::clone::Clone for IMobileBroadbandUiccAppRecordDetailsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMobileBroadbandUiccAppsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMobileBroadbandUiccAppsResult {}
impl ::core::clone::Clone for IMobileBroadbandUiccAppsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkOperatorDataUsageTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkOperatorDataUsageTriggerDetails {}
impl ::core::clone::Clone for INetworkOperatorDataUsageTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkOperatorNotificationEventDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkOperatorNotificationEventDetails {}
impl ::core::clone::Clone for INetworkOperatorNotificationEventDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkOperatorTetheringAccessPointConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkOperatorTetheringAccessPointConfiguration {}
impl ::core::clone::Clone for INetworkOperatorTetheringAccessPointConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkOperatorTetheringAccessPointConfiguration2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkOperatorTetheringAccessPointConfiguration2 {}
impl ::core::clone::Clone for INetworkOperatorTetheringAccessPointConfiguration2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkOperatorTetheringClient(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkOperatorTetheringClient {}
impl ::core::clone::Clone for INetworkOperatorTetheringClient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkOperatorTetheringClientManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkOperatorTetheringClientManager {}
impl ::core::clone::Clone for INetworkOperatorTetheringClientManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkOperatorTetheringEntitlementCheck(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkOperatorTetheringEntitlementCheck {}
impl ::core::clone::Clone for INetworkOperatorTetheringEntitlementCheck {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkOperatorTetheringManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkOperatorTetheringManager {}
impl ::core::clone::Clone for INetworkOperatorTetheringManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkOperatorTetheringManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkOperatorTetheringManagerStatics {}
impl ::core::clone::Clone for INetworkOperatorTetheringManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkOperatorTetheringManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkOperatorTetheringManagerStatics2 {}
impl ::core::clone::Clone for INetworkOperatorTetheringManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkOperatorTetheringManagerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkOperatorTetheringManagerStatics3 {}
impl ::core::clone::Clone for INetworkOperatorTetheringManagerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkOperatorTetheringManagerStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkOperatorTetheringManagerStatics4 {}
impl ::core::clone::Clone for INetworkOperatorTetheringManagerStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkOperatorTetheringOperationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkOperatorTetheringOperationResult {}
impl ::core::clone::Clone for INetworkOperatorTetheringOperationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProvisionFromXmlDocumentResults(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProvisionFromXmlDocumentResults {}
impl ::core::clone::Clone for IProvisionFromXmlDocumentResults {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProvisionedProfile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProvisionedProfile {}
impl ::core::clone::Clone for IProvisionedProfile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProvisioningAgent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProvisioningAgent {}
impl ::core::clone::Clone for IProvisioningAgent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProvisioningAgentStaticMethods(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProvisioningAgentStaticMethods {}
impl ::core::clone::Clone for IProvisioningAgentStaticMethods {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITetheringEntitlementCheckTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITetheringEntitlementCheckTriggerDetails {}
impl ::core::clone::Clone for ITetheringEntitlementCheckTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUssdMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUssdMessage {}
impl ::core::clone::Clone for IUssdMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUssdMessageFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUssdMessageFactory {}
impl ::core::clone::Clone for IUssdMessageFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUssdReply(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUssdReply {}
impl ::core::clone::Clone for IUssdReply {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUssdSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUssdSession {}
impl ::core::clone::Clone for IUssdSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUssdSessionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUssdSessionStatics {}
impl ::core::clone::Clone for IUssdSessionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandAccount(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandAccount {}
impl ::core::clone::Clone for MobileBroadbandAccount {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandAccountEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandAccountEventArgs {}
impl ::core::clone::Clone for MobileBroadbandAccountEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandAccountUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandAccountUpdatedEventArgs {}
impl ::core::clone::Clone for MobileBroadbandAccountUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandAccountWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandAccountWatcher {}
impl ::core::clone::Clone for MobileBroadbandAccountWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandAccountWatcherStatus(pub i32);
impl MobileBroadbandAccountWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Aborted: Self = Self(4i32);
}
impl ::core::marker::Copy for MobileBroadbandAccountWatcherStatus {}
impl ::core::clone::Clone for MobileBroadbandAccountWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandAntennaSar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandAntennaSar {}
impl ::core::clone::Clone for MobileBroadbandAntennaSar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandCellCdma(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandCellCdma {}
impl ::core::clone::Clone for MobileBroadbandCellCdma {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandCellGsm(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandCellGsm {}
impl ::core::clone::Clone for MobileBroadbandCellGsm {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandCellLte(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandCellLte {}
impl ::core::clone::Clone for MobileBroadbandCellLte {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandCellNR(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandCellNR {}
impl ::core::clone::Clone for MobileBroadbandCellNR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandCellTdscdma(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandCellTdscdma {}
impl ::core::clone::Clone for MobileBroadbandCellTdscdma {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandCellUmts(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandCellUmts {}
impl ::core::clone::Clone for MobileBroadbandCellUmts {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandCellsInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandCellsInfo {}
impl ::core::clone::Clone for MobileBroadbandCellsInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandCurrentSlotIndexChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandCurrentSlotIndexChangedEventArgs {}
impl ::core::clone::Clone for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandDeviceInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandDeviceInformation {}
impl ::core::clone::Clone for MobileBroadbandDeviceInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandDeviceService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandDeviceService {}
impl ::core::clone::Clone for MobileBroadbandDeviceService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceCommandResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandDeviceServiceCommandResult {}
impl ::core::clone::Clone for MobileBroadbandDeviceServiceCommandResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceCommandSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandDeviceServiceCommandSession {}
impl ::core::clone::Clone for MobileBroadbandDeviceServiceCommandSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandDeviceServiceDataReceivedEventArgs {}
impl ::core::clone::Clone for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceDataSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandDeviceServiceDataSession {}
impl ::core::clone::Clone for MobileBroadbandDeviceServiceDataSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandDeviceServiceInformation {}
impl ::core::clone::Clone for MobileBroadbandDeviceServiceInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandDeviceServiceTriggerDetails {}
impl ::core::clone::Clone for MobileBroadbandDeviceServiceTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandDeviceType(pub i32);
impl MobileBroadbandDeviceType {
    pub const Unknown: Self = Self(0i32);
    pub const Embedded: Self = Self(1i32);
    pub const Removable: Self = Self(2i32);
    pub const Remote: Self = Self(3i32);
}
impl ::core::marker::Copy for MobileBroadbandDeviceType {}
impl ::core::clone::Clone for MobileBroadbandDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandModem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandModem {}
impl ::core::clone::Clone for MobileBroadbandModem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandModemConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandModemConfiguration {}
impl ::core::clone::Clone for MobileBroadbandModemConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandModemIsolation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandModemIsolation {}
impl ::core::clone::Clone for MobileBroadbandModemIsolation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandModemStatus(pub i32);
impl MobileBroadbandModemStatus {
    pub const Success: Self = Self(0i32);
    pub const OtherFailure: Self = Self(1i32);
    pub const Busy: Self = Self(2i32);
    pub const NoDeviceSupport: Self = Self(3i32);
}
impl ::core::marker::Copy for MobileBroadbandModemStatus {}
impl ::core::clone::Clone for MobileBroadbandModemStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandNetwork(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandNetwork {}
impl ::core::clone::Clone for MobileBroadbandNetwork {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandNetworkRegistrationStateChange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandNetworkRegistrationStateChange {}
impl ::core::clone::Clone for MobileBroadbandNetworkRegistrationStateChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandNetworkRegistrationStateChangeTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {}
impl ::core::clone::Clone for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandPco(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandPco {}
impl ::core::clone::Clone for MobileBroadbandPco {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandPcoDataChangeTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandPcoDataChangeTriggerDetails {}
impl ::core::clone::Clone for MobileBroadbandPcoDataChangeTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandPin(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandPin {}
impl ::core::clone::Clone for MobileBroadbandPin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandPinFormat(pub i32);
impl MobileBroadbandPinFormat {
    pub const Unknown: Self = Self(0i32);
    pub const Numeric: Self = Self(1i32);
    pub const Alphanumeric: Self = Self(2i32);
}
impl ::core::marker::Copy for MobileBroadbandPinFormat {}
impl ::core::clone::Clone for MobileBroadbandPinFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandPinLockState(pub i32);
impl MobileBroadbandPinLockState {
    pub const Unknown: Self = Self(0i32);
    pub const Unlocked: Self = Self(1i32);
    pub const PinRequired: Self = Self(2i32);
    pub const PinUnblockKeyRequired: Self = Self(3i32);
}
impl ::core::marker::Copy for MobileBroadbandPinLockState {}
impl ::core::clone::Clone for MobileBroadbandPinLockState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandPinLockStateChange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandPinLockStateChange {}
impl ::core::clone::Clone for MobileBroadbandPinLockStateChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandPinLockStateChangeTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandPinLockStateChangeTriggerDetails {}
impl ::core::clone::Clone for MobileBroadbandPinLockStateChangeTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandPinManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandPinManager {}
impl ::core::clone::Clone for MobileBroadbandPinManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandPinOperationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandPinOperationResult {}
impl ::core::clone::Clone for MobileBroadbandPinOperationResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for MobileBroadbandPinType {}
impl ::core::clone::Clone for MobileBroadbandPinType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandRadioState(pub i32);
impl MobileBroadbandRadioState {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
}
impl ::core::marker::Copy for MobileBroadbandRadioState {}
impl ::core::clone::Clone for MobileBroadbandRadioState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandRadioStateChange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandRadioStateChange {}
impl ::core::clone::Clone for MobileBroadbandRadioStateChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandRadioStateChangeTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandRadioStateChangeTriggerDetails {}
impl ::core::clone::Clone for MobileBroadbandRadioStateChangeTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandSarManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandSarManager {}
impl ::core::clone::Clone for MobileBroadbandSarManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandSlotInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandSlotInfo {}
impl ::core::clone::Clone for MobileBroadbandSlotInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandSlotInfoChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandSlotInfoChangedEventArgs {}
impl ::core::clone::Clone for MobileBroadbandSlotInfoChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandSlotManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandSlotManager {}
impl ::core::clone::Clone for MobileBroadbandSlotManager {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for MobileBroadbandSlotState {}
impl ::core::clone::Clone for MobileBroadbandSlotState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandTransmissionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandTransmissionStateChangedEventArgs {}
impl ::core::clone::Clone for MobileBroadbandTransmissionStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandUicc(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandUicc {}
impl ::core::clone::Clone for MobileBroadbandUicc {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandUiccApp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandUiccApp {}
impl ::core::clone::Clone for MobileBroadbandUiccApp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandUiccAppOperationStatus(pub i32);
impl MobileBroadbandUiccAppOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const InvalidUiccFilePath: Self = Self(1i32);
    pub const AccessConditionNotHeld: Self = Self(2i32);
    pub const UiccBusy: Self = Self(3i32);
}
impl ::core::marker::Copy for MobileBroadbandUiccAppOperationStatus {}
impl ::core::clone::Clone for MobileBroadbandUiccAppOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandUiccAppReadRecordResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandUiccAppReadRecordResult {}
impl ::core::clone::Clone for MobileBroadbandUiccAppReadRecordResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandUiccAppRecordDetailsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandUiccAppRecordDetailsResult {}
impl ::core::clone::Clone for MobileBroadbandUiccAppRecordDetailsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandUiccAppsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandUiccAppsResult {}
impl ::core::clone::Clone for MobileBroadbandUiccAppsResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for NetworkDeviceStatus {}
impl ::core::clone::Clone for NetworkDeviceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NetworkOperatorDataUsageNotificationKind(pub i32);
impl NetworkOperatorDataUsageNotificationKind {
    pub const DataUsageProgress: Self = Self(0i32);
}
impl ::core::marker::Copy for NetworkOperatorDataUsageNotificationKind {}
impl ::core::clone::Clone for NetworkOperatorDataUsageNotificationKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NetworkOperatorDataUsageTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NetworkOperatorDataUsageTriggerDetails {}
impl ::core::clone::Clone for NetworkOperatorDataUsageTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for NetworkOperatorEventMessageType {}
impl ::core::clone::Clone for NetworkOperatorEventMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NetworkOperatorNotificationEventDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NetworkOperatorNotificationEventDetails {}
impl ::core::clone::Clone for NetworkOperatorNotificationEventDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NetworkOperatorTetheringAccessPointConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NetworkOperatorTetheringAccessPointConfiguration {}
impl ::core::clone::Clone for NetworkOperatorTetheringAccessPointConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NetworkOperatorTetheringClient(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NetworkOperatorTetheringClient {}
impl ::core::clone::Clone for NetworkOperatorTetheringClient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NetworkOperatorTetheringManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NetworkOperatorTetheringManager {}
impl ::core::clone::Clone for NetworkOperatorTetheringManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NetworkOperatorTetheringOperationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NetworkOperatorTetheringOperationResult {}
impl ::core::clone::Clone for NetworkOperatorTetheringOperationResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for NetworkRegistrationState {}
impl ::core::clone::Clone for NetworkRegistrationState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProfileMediaType(pub i32);
impl ProfileMediaType {
    pub const Wlan: Self = Self(0i32);
    pub const Wwan: Self = Self(1i32);
}
impl ::core::marker::Copy for ProfileMediaType {}
impl ::core::clone::Clone for ProfileMediaType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct ProfileUsage {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::DateTime,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for ProfileUsage {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for ProfileUsage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProvisionFromXmlDocumentResults(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProvisionFromXmlDocumentResults {}
impl ::core::clone::Clone for ProvisionFromXmlDocumentResults {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProvisionedProfile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProvisionedProfile {}
impl ::core::clone::Clone for ProvisionedProfile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProvisioningAgent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProvisioningAgent {}
impl ::core::clone::Clone for ProvisioningAgent {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for TetheringCapability {}
impl ::core::clone::Clone for TetheringCapability {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TetheringEntitlementCheckTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TetheringEntitlementCheckTriggerDetails {}
impl ::core::clone::Clone for TetheringEntitlementCheckTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for TetheringOperationStatus {}
impl ::core::clone::Clone for TetheringOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TetheringOperationalState(pub i32);
impl TetheringOperationalState {
    pub const Unknown: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const InTransition: Self = Self(3i32);
}
impl ::core::marker::Copy for TetheringOperationalState {}
impl ::core::clone::Clone for TetheringOperationalState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TetheringWiFiBand(pub i32);
impl TetheringWiFiBand {
    pub const Auto: Self = Self(0i32);
    pub const TwoPointFourGigahertz: Self = Self(1i32);
    pub const FiveGigahertz: Self = Self(2i32);
}
impl ::core::marker::Copy for TetheringWiFiBand {}
impl ::core::clone::Clone for TetheringWiFiBand {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for UiccAccessCondition {}
impl ::core::clone::Clone for UiccAccessCondition {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for UiccAppKind {}
impl ::core::clone::Clone for UiccAppKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UiccAppRecordKind(pub i32);
impl UiccAppRecordKind {
    pub const Unknown: Self = Self(0i32);
    pub const Transparent: Self = Self(1i32);
    pub const RecordOriented: Self = Self(2i32);
}
impl ::core::marker::Copy for UiccAppRecordKind {}
impl ::core::clone::Clone for UiccAppRecordKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UssdMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UssdMessage {}
impl ::core::clone::Clone for UssdMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UssdReply(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UssdReply {}
impl ::core::clone::Clone for UssdReply {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for UssdResultCode {}
impl ::core::clone::Clone for UssdResultCode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UssdSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UssdSession {}
impl ::core::clone::Clone for UssdSession {
    fn clone(&self) -> Self {
        *self
    }
}
