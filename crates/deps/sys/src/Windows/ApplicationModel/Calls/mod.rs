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
    pub const Continuous: Self = Self(0i32);
    pub const Burst: Self = Self(1i32);
}
#[repr(transparent)]
pub struct DtmfKey(pub i32);
impl DtmfKey {
    pub const D0: Self = Self(0i32);
    pub const D1: Self = Self(1i32);
    pub const D2: Self = Self(2i32);
    pub const D3: Self = Self(3i32);
    pub const D4: Self = Self(4i32);
    pub const D5: Self = Self(5i32);
    pub const D6: Self = Self(6i32);
    pub const D7: Self = Self(7i32);
    pub const D8: Self = Self(8i32);
    pub const D9: Self = Self(9i32);
    pub const Star: Self = Self(10i32);
    pub const Pound: Self = Self(11i32);
}
#[repr(transparent)]
pub struct DtmfToneAudioPlayback(pub i32);
impl DtmfToneAudioPlayback {
    pub const Play: Self = Self(0i32);
    pub const DoNotPlay: Self = Self(1i32);
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
    pub const Default: Self = Self(0i32);
    pub const Bluetooth: Self = Self(1i32);
    pub const Speakerphone: Self = Self(2i32);
}
#[repr(transparent)]
pub struct PhoneCall(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallAudioDevice(pub i32);
impl PhoneCallAudioDevice {
    pub const Unknown: Self = Self(0i32);
    pub const LocalDevice: Self = Self(1i32);
    pub const RemoteDevice: Self = Self(2i32);
}
#[repr(transparent)]
pub struct PhoneCallDirection(pub i32);
impl PhoneCallDirection {
    pub const Unknown: Self = Self(0i32);
    pub const Incoming: Self = Self(1i32);
    pub const Outgoing: Self = Self(2i32);
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallHistoryEntryAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallHistoryEntryMedia(pub i32);
impl PhoneCallHistoryEntryMedia {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntryOtherAppReadAccess(pub i32);
impl PhoneCallHistoryEntryOtherAppReadAccess {
    pub const Full: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntryQueryDesiredMedia(pub u32);
impl PhoneCallHistoryEntryQueryDesiredMedia {
    pub const None: Self = Self(0u32);
    pub const Audio: Self = Self(1u32);
    pub const Video: Self = Self(2u32);
    pub const All: Self = Self(4294967295u32);
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntryQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallHistoryEntryRawAddressKind(pub i32);
impl PhoneCallHistoryEntryRawAddressKind {
    pub const PhoneNumber: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntryReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallHistoryManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallHistorySourceIdKind(pub i32);
impl PhoneCallHistorySourceIdKind {
    pub const CellularPhoneLineId: Self = Self(0i32);
    pub const PackageFamilyName: Self = Self(1i32);
}
#[repr(transparent)]
pub struct PhoneCallHistoryStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallHistoryStoreAccessType(pub i32);
impl PhoneCallHistoryStoreAccessType {
    pub const AppEntriesReadWrite: Self = Self(0i32);
    pub const AllEntriesLimitedReadWrite: Self = Self(1i32);
    pub const AllEntriesReadWrite: Self = Self(2i32);
}
#[repr(transparent)]
pub struct PhoneCallInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallMedia(pub i32);
impl PhoneCallMedia {
    pub const Audio: Self = Self(0i32);
    pub const AudioAndVideo: Self = Self(1i32);
    pub const AudioAndRealTimeText: Self = Self(2i32);
}
#[repr(transparent)]
pub struct PhoneCallOperationStatus(pub i32);
impl PhoneCallOperationStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const OtherFailure: Self = Self(1i32);
    pub const TimedOut: Self = Self(2i32);
    pub const ConnectionLost: Self = Self(3i32);
    pub const InvalidCallState: Self = Self(4i32);
}
#[repr(transparent)]
pub struct PhoneCallStatus(pub i32);
impl PhoneCallStatus {
    pub const Lost: Self = Self(0i32);
    pub const Incoming: Self = Self(1i32);
    pub const Dialing: Self = Self(2i32);
    pub const Talking: Self = Self(3i32);
    pub const Held: Self = Self(4i32);
    pub const Ended: Self = Self(5i32);
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
    pub const Default: Self = Self(0i32);
    pub const Tile: Self = Self(1i32);
    pub const Dialer: Self = Self(2i32);
    pub const InCallUI: Self = Self(3i32);
}
#[repr(transparent)]
pub struct PhoneLineOperationStatus(pub i32);
impl PhoneLineOperationStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const OtherFailure: Self = Self(1i32);
    pub const TimedOut: Self = Self(2i32);
    pub const ConnectionLost: Self = Self(3i32);
    pub const InvalidCallState: Self = Self(4i32);
}
#[repr(transparent)]
pub struct PhoneLineTransport(pub i32);
impl PhoneLineTransport {
    pub const Cellular: Self = Self(0i32);
    pub const VoipApp: Self = Self(1i32);
    pub const Bluetooth: Self = Self(2i32);
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
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
}
#[repr(transparent)]
pub struct PhoneNetworkState(pub i32);
impl PhoneNetworkState {
    pub const Unknown: Self = Self(0i32);
    pub const NoSignal: Self = Self(1i32);
    pub const Deregistered: Self = Self(2i32);
    pub const Denied: Self = Self(3i32);
    pub const Searching: Self = Self(4i32);
    pub const Home: Self = Self(5i32);
    pub const RoamingInternational: Self = Self(6i32);
    pub const RoamingDomestic: Self = Self(7i32);
}
#[repr(transparent)]
pub struct PhoneSimState(pub i32);
impl PhoneSimState {
    pub const Unknown: Self = Self(0i32);
    pub const PinNotRequired: Self = Self(1i32);
    pub const PinUnlocked: Self = Self(2i32);
    pub const PinLocked: Self = Self(3i32);
    pub const PukLocked: Self = Self(4i32);
    pub const NotInserted: Self = Self(5i32);
    pub const Invalid: Self = Self(6i32);
    pub const Disabled: Self = Self(7i32);
}
#[repr(transparent)]
pub struct PhoneVoicemail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneVoicemailType(pub i32);
impl PhoneVoicemailType {
    pub const None: Self = Self(0i32);
    pub const Traditional: Self = Self(1i32);
    pub const Visual: Self = Self(2i32);
}
#[repr(transparent)]
pub struct TransportDeviceAudioRoutingStatus(pub i32);
impl TransportDeviceAudioRoutingStatus {
    pub const Unknown: Self = Self(0i32);
    pub const CanRouteToLocalDevice: Self = Self(1i32);
    pub const CannotRouteToLocalDevice: Self = Self(2i32);
}
#[repr(transparent)]
pub struct VoipCallCoordinator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoipPhoneCall(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoipPhoneCallMedia(pub u32);
impl VoipPhoneCallMedia {
    pub const None: Self = Self(0u32);
    pub const Audio: Self = Self(1u32);
    pub const Video: Self = Self(2u32);
}
#[repr(transparent)]
pub struct VoipPhoneCallRejectReason(pub i32);
impl VoipPhoneCallRejectReason {
    pub const UserIgnored: Self = Self(0i32);
    pub const TimedOut: Self = Self(1i32);
    pub const OtherIncomingCall: Self = Self(2i32);
    pub const EmergencyCallExists: Self = Self(3i32);
    pub const InvalidCallState: Self = Self(4i32);
}
#[repr(transparent)]
pub struct VoipPhoneCallResourceReservationStatus(pub i32);
impl VoipPhoneCallResourceReservationStatus {
    pub const Success: Self = Self(0i32);
    pub const ResourcesNotAvailable: Self = Self(1i32);
}
#[repr(transparent)]
pub struct VoipPhoneCallState(pub i32);
impl VoipPhoneCallState {
    pub const Ended: Self = Self(0i32);
    pub const Held: Self = Self(1i32);
    pub const Active: Self = Self(2i32);
    pub const Incoming: Self = Self(3i32);
    pub const Outgoing: Self = Self(4i32);
}
