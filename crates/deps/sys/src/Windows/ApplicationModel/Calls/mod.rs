#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_Calls_Background")]
pub mod Background;
#[cfg(feature = "ApplicationModel_Calls_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CallAnswerEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CallRejectEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CallStateChangeEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CallsPhoneContract(i32);
#[repr(C)]
pub struct CallsVoipContract(i32);
#[repr(transparent)]
pub struct CellularDtmfMode(pub i32);
impl CellularDtmfMode {
    pub const Continuous: CellularDtmfMode = CellularDtmfMode(0i32);
    pub const Burst: CellularDtmfMode = CellularDtmfMode(1i32);
}
#[repr(transparent)]
pub struct DtmfKey(pub i32);
impl DtmfKey {
    pub const D0: DtmfKey = DtmfKey(0i32);
    pub const D1: DtmfKey = DtmfKey(1i32);
    pub const D2: DtmfKey = DtmfKey(2i32);
    pub const D3: DtmfKey = DtmfKey(3i32);
    pub const D4: DtmfKey = DtmfKey(4i32);
    pub const D5: DtmfKey = DtmfKey(5i32);
    pub const D6: DtmfKey = DtmfKey(6i32);
    pub const D7: DtmfKey = DtmfKey(7i32);
    pub const D8: DtmfKey = DtmfKey(8i32);
    pub const D9: DtmfKey = DtmfKey(9i32);
    pub const Star: DtmfKey = DtmfKey(10i32);
    pub const Pound: DtmfKey = DtmfKey(11i32);
}
#[repr(transparent)]
pub struct DtmfToneAudioPlayback(pub i32);
impl DtmfToneAudioPlayback {
    pub const Play: DtmfToneAudioPlayback = DtmfToneAudioPlayback(0i32);
    pub const DoNotPlay: DtmfToneAudioPlayback = DtmfToneAudioPlayback(1i32);
}
#[repr(transparent)]
pub struct ICallAnswerEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICallRejectEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICallStateChangeEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILockScreenCallEndCallDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILockScreenCallEndRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILockScreenCallUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMuteChangeEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCall(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallBlockingStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallHistoryEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallHistoryEntryAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallHistoryEntryAddressFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallHistoryEntryQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallHistoryEntryReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallHistoryManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallHistoryManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallHistoryManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallHistoryStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallVideoCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallVideoCapabilitiesManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneDialOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneLine(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneLine2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneLine3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneLineCellularDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneLineConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneLineDialResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneLineStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneLineTransportDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneLineTransportDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneLineTransportDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneLineWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneLineWatcherEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneVoicemail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoipCallCoordinator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoipCallCoordinator2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoipCallCoordinator3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoipCallCoordinator4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoipCallCoordinatorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoipPhoneCall(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoipPhoneCall2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoipPhoneCall3(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct LockScreenCallContract(i32);
#[repr(transparent)]
pub struct LockScreenCallEndCallDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LockScreenCallEndRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LockScreenCallUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MuteChangeEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneAudioRoutingEndpoint(pub i32);
impl PhoneAudioRoutingEndpoint {
    pub const Default: PhoneAudioRoutingEndpoint = PhoneAudioRoutingEndpoint(0i32);
    pub const Bluetooth: PhoneAudioRoutingEndpoint = PhoneAudioRoutingEndpoint(1i32);
    pub const Speakerphone: PhoneAudioRoutingEndpoint = PhoneAudioRoutingEndpoint(2i32);
}
#[repr(transparent)]
pub struct PhoneCall(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallAudioDevice(pub i32);
impl PhoneCallAudioDevice {
    pub const Unknown: PhoneCallAudioDevice = PhoneCallAudioDevice(0i32);
    pub const LocalDevice: PhoneCallAudioDevice = PhoneCallAudioDevice(1i32);
    pub const RemoteDevice: PhoneCallAudioDevice = PhoneCallAudioDevice(2i32);
}
#[repr(transparent)]
pub struct PhoneCallDirection(pub i32);
impl PhoneCallDirection {
    pub const Unknown: PhoneCallDirection = PhoneCallDirection(0i32);
    pub const Incoming: PhoneCallDirection = PhoneCallDirection(1i32);
    pub const Outgoing: PhoneCallDirection = PhoneCallDirection(2i32);
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallHistoryEntryAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallHistoryEntryMedia(pub i32);
impl PhoneCallHistoryEntryMedia {
    pub const Audio: PhoneCallHistoryEntryMedia = PhoneCallHistoryEntryMedia(0i32);
    pub const Video: PhoneCallHistoryEntryMedia = PhoneCallHistoryEntryMedia(1i32);
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntryOtherAppReadAccess(pub i32);
impl PhoneCallHistoryEntryOtherAppReadAccess {
    pub const Full: PhoneCallHistoryEntryOtherAppReadAccess = PhoneCallHistoryEntryOtherAppReadAccess(0i32);
    pub const SystemOnly: PhoneCallHistoryEntryOtherAppReadAccess = PhoneCallHistoryEntryOtherAppReadAccess(1i32);
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntryQueryDesiredMedia(pub u32);
impl PhoneCallHistoryEntryQueryDesiredMedia {
    pub const None: PhoneCallHistoryEntryQueryDesiredMedia = PhoneCallHistoryEntryQueryDesiredMedia(0u32);
    pub const Audio: PhoneCallHistoryEntryQueryDesiredMedia = PhoneCallHistoryEntryQueryDesiredMedia(1u32);
    pub const Video: PhoneCallHistoryEntryQueryDesiredMedia = PhoneCallHistoryEntryQueryDesiredMedia(2u32);
    pub const All: PhoneCallHistoryEntryQueryDesiredMedia = PhoneCallHistoryEntryQueryDesiredMedia(4294967295u32);
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntryQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallHistoryEntryRawAddressKind(pub i32);
impl PhoneCallHistoryEntryRawAddressKind {
    pub const PhoneNumber: PhoneCallHistoryEntryRawAddressKind = PhoneCallHistoryEntryRawAddressKind(0i32);
    pub const Custom: PhoneCallHistoryEntryRawAddressKind = PhoneCallHistoryEntryRawAddressKind(1i32);
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntryReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallHistoryManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallHistorySourceIdKind(pub i32);
impl PhoneCallHistorySourceIdKind {
    pub const CellularPhoneLineId: PhoneCallHistorySourceIdKind = PhoneCallHistorySourceIdKind(0i32);
    pub const PackageFamilyName: PhoneCallHistorySourceIdKind = PhoneCallHistorySourceIdKind(1i32);
}
#[repr(transparent)]
pub struct PhoneCallHistoryStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallHistoryStoreAccessType(pub i32);
impl PhoneCallHistoryStoreAccessType {
    pub const AppEntriesReadWrite: PhoneCallHistoryStoreAccessType = PhoneCallHistoryStoreAccessType(0i32);
    pub const AllEntriesLimitedReadWrite: PhoneCallHistoryStoreAccessType = PhoneCallHistoryStoreAccessType(1i32);
    pub const AllEntriesReadWrite: PhoneCallHistoryStoreAccessType = PhoneCallHistoryStoreAccessType(2i32);
}
#[repr(transparent)]
pub struct PhoneCallInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallMedia(pub i32);
impl PhoneCallMedia {
    pub const Audio: PhoneCallMedia = PhoneCallMedia(0i32);
    pub const AudioAndVideo: PhoneCallMedia = PhoneCallMedia(1i32);
    pub const AudioAndRealTimeText: PhoneCallMedia = PhoneCallMedia(2i32);
}
#[repr(transparent)]
pub struct PhoneCallOperationStatus(pub i32);
impl PhoneCallOperationStatus {
    pub const Succeeded: PhoneCallOperationStatus = PhoneCallOperationStatus(0i32);
    pub const OtherFailure: PhoneCallOperationStatus = PhoneCallOperationStatus(1i32);
    pub const TimedOut: PhoneCallOperationStatus = PhoneCallOperationStatus(2i32);
    pub const ConnectionLost: PhoneCallOperationStatus = PhoneCallOperationStatus(3i32);
    pub const InvalidCallState: PhoneCallOperationStatus = PhoneCallOperationStatus(4i32);
}
#[repr(transparent)]
pub struct PhoneCallStatus(pub i32);
impl PhoneCallStatus {
    pub const Lost: PhoneCallStatus = PhoneCallStatus(0i32);
    pub const Incoming: PhoneCallStatus = PhoneCallStatus(1i32);
    pub const Dialing: PhoneCallStatus = PhoneCallStatus(2i32);
    pub const Talking: PhoneCallStatus = PhoneCallStatus(3i32);
    pub const Held: PhoneCallStatus = PhoneCallStatus(4i32);
    pub const Ended: PhoneCallStatus = PhoneCallStatus(5i32);
}
#[repr(transparent)]
pub struct PhoneCallStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallVideoCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneDialOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneLine(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneLineCellularDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneLineConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneLineDialResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneLineNetworkOperatorDisplayTextLocation(pub i32);
impl PhoneLineNetworkOperatorDisplayTextLocation {
    pub const Default: PhoneLineNetworkOperatorDisplayTextLocation = PhoneLineNetworkOperatorDisplayTextLocation(0i32);
    pub const Tile: PhoneLineNetworkOperatorDisplayTextLocation = PhoneLineNetworkOperatorDisplayTextLocation(1i32);
    pub const Dialer: PhoneLineNetworkOperatorDisplayTextLocation = PhoneLineNetworkOperatorDisplayTextLocation(2i32);
    pub const InCallUI: PhoneLineNetworkOperatorDisplayTextLocation = PhoneLineNetworkOperatorDisplayTextLocation(3i32);
}
#[repr(transparent)]
pub struct PhoneLineOperationStatus(pub i32);
impl PhoneLineOperationStatus {
    pub const Succeeded: PhoneLineOperationStatus = PhoneLineOperationStatus(0i32);
    pub const OtherFailure: PhoneLineOperationStatus = PhoneLineOperationStatus(1i32);
    pub const TimedOut: PhoneLineOperationStatus = PhoneLineOperationStatus(2i32);
    pub const ConnectionLost: PhoneLineOperationStatus = PhoneLineOperationStatus(3i32);
    pub const InvalidCallState: PhoneLineOperationStatus = PhoneLineOperationStatus(4i32);
}
#[repr(transparent)]
pub struct PhoneLineTransport(pub i32);
impl PhoneLineTransport {
    pub const Cellular: PhoneLineTransport = PhoneLineTransport(0i32);
    pub const VoipApp: PhoneLineTransport = PhoneLineTransport(1i32);
    pub const Bluetooth: PhoneLineTransport = PhoneLineTransport(2i32);
}
#[repr(transparent)]
pub struct PhoneLineTransportDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneLineWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneLineWatcherEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneLineWatcherStatus(pub i32);
impl PhoneLineWatcherStatus {
    pub const Created: PhoneLineWatcherStatus = PhoneLineWatcherStatus(0i32);
    pub const Started: PhoneLineWatcherStatus = PhoneLineWatcherStatus(1i32);
    pub const EnumerationCompleted: PhoneLineWatcherStatus = PhoneLineWatcherStatus(2i32);
    pub const Stopped: PhoneLineWatcherStatus = PhoneLineWatcherStatus(3i32);
}
#[repr(transparent)]
pub struct PhoneNetworkState(pub i32);
impl PhoneNetworkState {
    pub const Unknown: PhoneNetworkState = PhoneNetworkState(0i32);
    pub const NoSignal: PhoneNetworkState = PhoneNetworkState(1i32);
    pub const Deregistered: PhoneNetworkState = PhoneNetworkState(2i32);
    pub const Denied: PhoneNetworkState = PhoneNetworkState(3i32);
    pub const Searching: PhoneNetworkState = PhoneNetworkState(4i32);
    pub const Home: PhoneNetworkState = PhoneNetworkState(5i32);
    pub const RoamingInternational: PhoneNetworkState = PhoneNetworkState(6i32);
    pub const RoamingDomestic: PhoneNetworkState = PhoneNetworkState(7i32);
}
#[repr(transparent)]
pub struct PhoneSimState(pub i32);
impl PhoneSimState {
    pub const Unknown: PhoneSimState = PhoneSimState(0i32);
    pub const PinNotRequired: PhoneSimState = PhoneSimState(1i32);
    pub const PinUnlocked: PhoneSimState = PhoneSimState(2i32);
    pub const PinLocked: PhoneSimState = PhoneSimState(3i32);
    pub const PukLocked: PhoneSimState = PhoneSimState(4i32);
    pub const NotInserted: PhoneSimState = PhoneSimState(5i32);
    pub const Invalid: PhoneSimState = PhoneSimState(6i32);
    pub const Disabled: PhoneSimState = PhoneSimState(7i32);
}
#[repr(transparent)]
pub struct PhoneVoicemail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneVoicemailType(pub i32);
impl PhoneVoicemailType {
    pub const None: PhoneVoicemailType = PhoneVoicemailType(0i32);
    pub const Traditional: PhoneVoicemailType = PhoneVoicemailType(1i32);
    pub const Visual: PhoneVoicemailType = PhoneVoicemailType(2i32);
}
#[repr(transparent)]
pub struct TransportDeviceAudioRoutingStatus(pub i32);
impl TransportDeviceAudioRoutingStatus {
    pub const Unknown: TransportDeviceAudioRoutingStatus = TransportDeviceAudioRoutingStatus(0i32);
    pub const CanRouteToLocalDevice: TransportDeviceAudioRoutingStatus = TransportDeviceAudioRoutingStatus(1i32);
    pub const CannotRouteToLocalDevice: TransportDeviceAudioRoutingStatus = TransportDeviceAudioRoutingStatus(2i32);
}
#[repr(transparent)]
pub struct VoipCallCoordinator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoipPhoneCall(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoipPhoneCallMedia(pub u32);
impl VoipPhoneCallMedia {
    pub const None: VoipPhoneCallMedia = VoipPhoneCallMedia(0u32);
    pub const Audio: VoipPhoneCallMedia = VoipPhoneCallMedia(1u32);
    pub const Video: VoipPhoneCallMedia = VoipPhoneCallMedia(2u32);
}
#[repr(transparent)]
pub struct VoipPhoneCallRejectReason(pub i32);
impl VoipPhoneCallRejectReason {
    pub const UserIgnored: VoipPhoneCallRejectReason = VoipPhoneCallRejectReason(0i32);
    pub const TimedOut: VoipPhoneCallRejectReason = VoipPhoneCallRejectReason(1i32);
    pub const OtherIncomingCall: VoipPhoneCallRejectReason = VoipPhoneCallRejectReason(2i32);
    pub const EmergencyCallExists: VoipPhoneCallRejectReason = VoipPhoneCallRejectReason(3i32);
    pub const InvalidCallState: VoipPhoneCallRejectReason = VoipPhoneCallRejectReason(4i32);
}
#[repr(transparent)]
pub struct VoipPhoneCallResourceReservationStatus(pub i32);
impl VoipPhoneCallResourceReservationStatus {
    pub const Success: VoipPhoneCallResourceReservationStatus = VoipPhoneCallResourceReservationStatus(0i32);
    pub const ResourcesNotAvailable: VoipPhoneCallResourceReservationStatus = VoipPhoneCallResourceReservationStatus(1i32);
}
#[repr(transparent)]
pub struct VoipPhoneCallState(pub i32);
impl VoipPhoneCallState {
    pub const Ended: VoipPhoneCallState = VoipPhoneCallState(0i32);
    pub const Held: VoipPhoneCallState = VoipPhoneCallState(1i32);
    pub const Active: VoipPhoneCallState = VoipPhoneCallState(2i32);
    pub const Incoming: VoipPhoneCallState = VoipPhoneCallState(3i32);
    pub const Outgoing: VoipPhoneCallState = VoipPhoneCallState(4i32);
}
