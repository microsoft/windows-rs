#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_Calls_Background")]
pub mod Background;
#[cfg(feature = "ApplicationModel_Calls_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CallAnswerEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CallAnswerEventArgs {}
impl ::core::clone::Clone for CallAnswerEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CallRejectEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CallRejectEventArgs {}
impl ::core::clone::Clone for CallRejectEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CallStateChangeEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CallStateChangeEventArgs {}
impl ::core::clone::Clone for CallStateChangeEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CellularDtmfMode(pub i32);
impl CellularDtmfMode {
    pub const Continuous: Self = Self(0i32);
    pub const Burst: Self = Self(1i32);
}
impl ::core::marker::Copy for CellularDtmfMode {}
impl ::core::clone::Clone for CellularDtmfMode {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for DtmfKey {}
impl ::core::clone::Clone for DtmfKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DtmfToneAudioPlayback(pub i32);
impl DtmfToneAudioPlayback {
    pub const Play: Self = Self(0i32);
    pub const DoNotPlay: Self = Self(1i32);
}
impl ::core::marker::Copy for DtmfToneAudioPlayback {}
impl ::core::clone::Clone for DtmfToneAudioPlayback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICallAnswerEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICallAnswerEventArgs {}
impl ::core::clone::Clone for ICallAnswerEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICallRejectEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICallRejectEventArgs {}
impl ::core::clone::Clone for ICallRejectEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICallStateChangeEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICallStateChangeEventArgs {}
impl ::core::clone::Clone for ICallStateChangeEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILockScreenCallEndCallDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILockScreenCallEndCallDeferral {}
impl ::core::clone::Clone for ILockScreenCallEndCallDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILockScreenCallEndRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILockScreenCallEndRequestedEventArgs {}
impl ::core::clone::Clone for ILockScreenCallEndRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILockScreenCallUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILockScreenCallUI {}
impl ::core::clone::Clone for ILockScreenCallUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMuteChangeEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMuteChangeEventArgs {}
impl ::core::clone::Clone for IMuteChangeEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCall(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCall {}
impl ::core::clone::Clone for IPhoneCall {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallBlockingStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallBlockingStatics {}
impl ::core::clone::Clone for IPhoneCallBlockingStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallHistoryEntry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallHistoryEntry {}
impl ::core::clone::Clone for IPhoneCallHistoryEntry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallHistoryEntryAddress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallHistoryEntryAddress {}
impl ::core::clone::Clone for IPhoneCallHistoryEntryAddress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallHistoryEntryAddressFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallHistoryEntryAddressFactory {}
impl ::core::clone::Clone for IPhoneCallHistoryEntryAddressFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallHistoryEntryQueryOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallHistoryEntryQueryOptions {}
impl ::core::clone::Clone for IPhoneCallHistoryEntryQueryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallHistoryEntryReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallHistoryEntryReader {}
impl ::core::clone::Clone for IPhoneCallHistoryEntryReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallHistoryManagerForUser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallHistoryManagerForUser {}
impl ::core::clone::Clone for IPhoneCallHistoryManagerForUser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallHistoryManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallHistoryManagerStatics {}
impl ::core::clone::Clone for IPhoneCallHistoryManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallHistoryManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallHistoryManagerStatics2 {}
impl ::core::clone::Clone for IPhoneCallHistoryManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallHistoryStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallHistoryStore {}
impl ::core::clone::Clone for IPhoneCallHistoryStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallInfo {}
impl ::core::clone::Clone for IPhoneCallInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallManagerStatics {}
impl ::core::clone::Clone for IPhoneCallManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallManagerStatics2 {}
impl ::core::clone::Clone for IPhoneCallManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallStatics {}
impl ::core::clone::Clone for IPhoneCallStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallStore {}
impl ::core::clone::Clone for IPhoneCallStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallVideoCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallVideoCapabilities {}
impl ::core::clone::Clone for IPhoneCallVideoCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallVideoCapabilitiesManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallVideoCapabilitiesManagerStatics {}
impl ::core::clone::Clone for IPhoneCallVideoCapabilitiesManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallsResult {}
impl ::core::clone::Clone for IPhoneCallsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneDialOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneDialOptions {}
impl ::core::clone::Clone for IPhoneDialOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneLine(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneLine {}
impl ::core::clone::Clone for IPhoneLine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneLine2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneLine2 {}
impl ::core::clone::Clone for IPhoneLine2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneLine3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneLine3 {}
impl ::core::clone::Clone for IPhoneLine3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneLineCellularDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneLineCellularDetails {}
impl ::core::clone::Clone for IPhoneLineCellularDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneLineConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneLineConfiguration {}
impl ::core::clone::Clone for IPhoneLineConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneLineDialResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneLineDialResult {}
impl ::core::clone::Clone for IPhoneLineDialResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneLineStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneLineStatics {}
impl ::core::clone::Clone for IPhoneLineStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneLineTransportDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneLineTransportDevice {}
impl ::core::clone::Clone for IPhoneLineTransportDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneLineTransportDevice2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneLineTransportDevice2 {}
impl ::core::clone::Clone for IPhoneLineTransportDevice2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneLineTransportDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneLineTransportDeviceStatics {}
impl ::core::clone::Clone for IPhoneLineTransportDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneLineWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneLineWatcher {}
impl ::core::clone::Clone for IPhoneLineWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneLineWatcherEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneLineWatcherEventArgs {}
impl ::core::clone::Clone for IPhoneLineWatcherEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneVoicemail(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneVoicemail {}
impl ::core::clone::Clone for IPhoneVoicemail {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoipCallCoordinator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoipCallCoordinator {}
impl ::core::clone::Clone for IVoipCallCoordinator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoipCallCoordinator2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoipCallCoordinator2 {}
impl ::core::clone::Clone for IVoipCallCoordinator2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoipCallCoordinator3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoipCallCoordinator3 {}
impl ::core::clone::Clone for IVoipCallCoordinator3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoipCallCoordinator4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoipCallCoordinator4 {}
impl ::core::clone::Clone for IVoipCallCoordinator4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoipCallCoordinatorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoipCallCoordinatorStatics {}
impl ::core::clone::Clone for IVoipCallCoordinatorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoipPhoneCall(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoipPhoneCall {}
impl ::core::clone::Clone for IVoipPhoneCall {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoipPhoneCall2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoipPhoneCall2 {}
impl ::core::clone::Clone for IVoipPhoneCall2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoipPhoneCall3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoipPhoneCall3 {}
impl ::core::clone::Clone for IVoipPhoneCall3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LockScreenCallEndCallDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LockScreenCallEndCallDeferral {}
impl ::core::clone::Clone for LockScreenCallEndCallDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LockScreenCallEndRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LockScreenCallEndRequestedEventArgs {}
impl ::core::clone::Clone for LockScreenCallEndRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LockScreenCallUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LockScreenCallUI {}
impl ::core::clone::Clone for LockScreenCallUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MuteChangeEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MuteChangeEventArgs {}
impl ::core::clone::Clone for MuteChangeEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneAudioRoutingEndpoint(pub i32);
impl PhoneAudioRoutingEndpoint {
    pub const Default: Self = Self(0i32);
    pub const Bluetooth: Self = Self(1i32);
    pub const Speakerphone: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneAudioRoutingEndpoint {}
impl ::core::clone::Clone for PhoneAudioRoutingEndpoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCall(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneCall {}
impl ::core::clone::Clone for PhoneCall {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallAudioDevice(pub i32);
impl PhoneCallAudioDevice {
    pub const Unknown: Self = Self(0i32);
    pub const LocalDevice: Self = Self(1i32);
    pub const RemoteDevice: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallAudioDevice {}
impl ::core::clone::Clone for PhoneCallAudioDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallDirection(pub i32);
impl PhoneCallDirection {
    pub const Unknown: Self = Self(0i32);
    pub const Incoming: Self = Self(1i32);
    pub const Outgoing: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallDirection {}
impl ::core::clone::Clone for PhoneCallDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneCallHistoryEntry {}
impl ::core::clone::Clone for PhoneCallHistoryEntry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntryAddress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneCallHistoryEntryAddress {}
impl ::core::clone::Clone for PhoneCallHistoryEntryAddress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntryMedia(pub i32);
impl PhoneCallHistoryEntryMedia {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallHistoryEntryMedia {}
impl ::core::clone::Clone for PhoneCallHistoryEntryMedia {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntryOtherAppReadAccess(pub i32);
impl PhoneCallHistoryEntryOtherAppReadAccess {
    pub const Full: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallHistoryEntryOtherAppReadAccess {}
impl ::core::clone::Clone for PhoneCallHistoryEntryOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntryQueryDesiredMedia(pub u32);
impl PhoneCallHistoryEntryQueryDesiredMedia {
    pub const None: Self = Self(0u32);
    pub const Audio: Self = Self(1u32);
    pub const Video: Self = Self(2u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for PhoneCallHistoryEntryQueryDesiredMedia {}
impl ::core::clone::Clone for PhoneCallHistoryEntryQueryDesiredMedia {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntryQueryOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneCallHistoryEntryQueryOptions {}
impl ::core::clone::Clone for PhoneCallHistoryEntryQueryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntryRawAddressKind(pub i32);
impl PhoneCallHistoryEntryRawAddressKind {
    pub const PhoneNumber: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallHistoryEntryRawAddressKind {}
impl ::core::clone::Clone for PhoneCallHistoryEntryRawAddressKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallHistoryEntryReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneCallHistoryEntryReader {}
impl ::core::clone::Clone for PhoneCallHistoryEntryReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallHistoryManagerForUser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneCallHistoryManagerForUser {}
impl ::core::clone::Clone for PhoneCallHistoryManagerForUser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallHistorySourceIdKind(pub i32);
impl PhoneCallHistorySourceIdKind {
    pub const CellularPhoneLineId: Self = Self(0i32);
    pub const PackageFamilyName: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallHistorySourceIdKind {}
impl ::core::clone::Clone for PhoneCallHistorySourceIdKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallHistoryStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneCallHistoryStore {}
impl ::core::clone::Clone for PhoneCallHistoryStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallHistoryStoreAccessType(pub i32);
impl PhoneCallHistoryStoreAccessType {
    pub const AppEntriesReadWrite: Self = Self(0i32);
    pub const AllEntriesLimitedReadWrite: Self = Self(1i32);
    pub const AllEntriesReadWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallHistoryStoreAccessType {}
impl ::core::clone::Clone for PhoneCallHistoryStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneCallInfo {}
impl ::core::clone::Clone for PhoneCallInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallMedia(pub i32);
impl PhoneCallMedia {
    pub const Audio: Self = Self(0i32);
    pub const AudioAndVideo: Self = Self(1i32);
    pub const AudioAndRealTimeText: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallMedia {}
impl ::core::clone::Clone for PhoneCallMedia {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for PhoneCallOperationStatus {}
impl ::core::clone::Clone for PhoneCallOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for PhoneCallStatus {}
impl ::core::clone::Clone for PhoneCallStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneCallStore {}
impl ::core::clone::Clone for PhoneCallStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallVideoCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneCallVideoCapabilities {}
impl ::core::clone::Clone for PhoneCallVideoCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneCallsResult {}
impl ::core::clone::Clone for PhoneCallsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneDialOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneDialOptions {}
impl ::core::clone::Clone for PhoneDialOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneLine(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneLine {}
impl ::core::clone::Clone for PhoneLine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneLineCellularDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneLineCellularDetails {}
impl ::core::clone::Clone for PhoneLineCellularDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneLineConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneLineConfiguration {}
impl ::core::clone::Clone for PhoneLineConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneLineDialResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneLineDialResult {}
impl ::core::clone::Clone for PhoneLineDialResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneLineNetworkOperatorDisplayTextLocation(pub i32);
impl PhoneLineNetworkOperatorDisplayTextLocation {
    pub const Default: Self = Self(0i32);
    pub const Tile: Self = Self(1i32);
    pub const Dialer: Self = Self(2i32);
    pub const InCallUI: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneLineNetworkOperatorDisplayTextLocation {}
impl ::core::clone::Clone for PhoneLineNetworkOperatorDisplayTextLocation {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for PhoneLineOperationStatus {}
impl ::core::clone::Clone for PhoneLineOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneLineTransport(pub i32);
impl PhoneLineTransport {
    pub const Cellular: Self = Self(0i32);
    pub const VoipApp: Self = Self(1i32);
    pub const Bluetooth: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneLineTransport {}
impl ::core::clone::Clone for PhoneLineTransport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneLineTransportDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneLineTransportDevice {}
impl ::core::clone::Clone for PhoneLineTransportDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneLineWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneLineWatcher {}
impl ::core::clone::Clone for PhoneLineWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneLineWatcherEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneLineWatcherEventArgs {}
impl ::core::clone::Clone for PhoneLineWatcherEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneLineWatcherStatus(pub i32);
impl PhoneLineWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneLineWatcherStatus {}
impl ::core::clone::Clone for PhoneLineWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for PhoneNetworkState {}
impl ::core::clone::Clone for PhoneNetworkState {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for PhoneSimState {}
impl ::core::clone::Clone for PhoneSimState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneVoicemail(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneVoicemail {}
impl ::core::clone::Clone for PhoneVoicemail {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneVoicemailType(pub i32);
impl PhoneVoicemailType {
    pub const None: Self = Self(0i32);
    pub const Traditional: Self = Self(1i32);
    pub const Visual: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneVoicemailType {}
impl ::core::clone::Clone for PhoneVoicemailType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TransportDeviceAudioRoutingStatus(pub i32);
impl TransportDeviceAudioRoutingStatus {
    pub const Unknown: Self = Self(0i32);
    pub const CanRouteToLocalDevice: Self = Self(1i32);
    pub const CannotRouteToLocalDevice: Self = Self(2i32);
}
impl ::core::marker::Copy for TransportDeviceAudioRoutingStatus {}
impl ::core::clone::Clone for TransportDeviceAudioRoutingStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VoipCallCoordinator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VoipCallCoordinator {}
impl ::core::clone::Clone for VoipCallCoordinator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VoipPhoneCall(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VoipPhoneCall {}
impl ::core::clone::Clone for VoipPhoneCall {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VoipPhoneCallMedia(pub u32);
impl VoipPhoneCallMedia {
    pub const None: Self = Self(0u32);
    pub const Audio: Self = Self(1u32);
    pub const Video: Self = Self(2u32);
}
impl ::core::marker::Copy for VoipPhoneCallMedia {}
impl ::core::clone::Clone for VoipPhoneCallMedia {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for VoipPhoneCallRejectReason {}
impl ::core::clone::Clone for VoipPhoneCallRejectReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VoipPhoneCallResourceReservationStatus(pub i32);
impl VoipPhoneCallResourceReservationStatus {
    pub const Success: Self = Self(0i32);
    pub const ResourcesNotAvailable: Self = Self(1i32);
}
impl ::core::marker::Copy for VoipPhoneCallResourceReservationStatus {}
impl ::core::clone::Clone for VoipPhoneCallResourceReservationStatus {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for VoipPhoneCallState {}
impl ::core::clone::Clone for VoipPhoneCallState {
    fn clone(&self) -> Self {
        *self
    }
}
