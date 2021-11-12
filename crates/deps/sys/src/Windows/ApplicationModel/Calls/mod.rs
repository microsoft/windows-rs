#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct CellularDtmfMode(i32);
#[repr(C)]
pub struct DtmfKey(i32);
#[repr(C)]
pub struct DtmfToneAudioPlayback(i32);
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
#[repr(C)]
pub struct PhoneAudioRoutingEndpoint(i32);
#[repr(transparent)]
pub struct PhoneCall(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhoneCallAudioDevice(i32);
#[repr(C)]
pub struct PhoneCallDirection(i32);
#[repr(transparent)]
pub struct PhoneCallHistoryEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallHistoryEntryAddress(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhoneCallHistoryEntryMedia(i32);
#[repr(C)]
pub struct PhoneCallHistoryEntryOtherAppReadAccess(i32);
#[repr(C)]
pub struct PhoneCallHistoryEntryQueryDesiredMedia(i32);
#[repr(transparent)]
pub struct PhoneCallHistoryEntryQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhoneCallHistoryEntryRawAddressKind(i32);
#[repr(transparent)]
pub struct PhoneCallHistoryEntryReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallHistoryManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhoneCallHistorySourceIdKind(i32);
#[repr(transparent)]
pub struct PhoneCallHistoryStore(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhoneCallHistoryStoreAccessType(i32);
#[repr(transparent)]
pub struct PhoneCallInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhoneCallMedia(i32);
#[repr(C)]
pub struct PhoneCallOperationStatus(i32);
#[repr(C)]
pub struct PhoneCallStatus(i32);
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
#[repr(C)]
pub struct PhoneLineNetworkOperatorDisplayTextLocation(i32);
#[repr(C)]
pub struct PhoneLineOperationStatus(i32);
#[repr(C)]
pub struct PhoneLineTransport(i32);
#[repr(transparent)]
pub struct PhoneLineTransportDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneLineWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneLineWatcherEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhoneLineWatcherStatus(i32);
#[repr(C)]
pub struct PhoneNetworkState(i32);
#[repr(C)]
pub struct PhoneSimState(i32);
#[repr(transparent)]
pub struct PhoneVoicemail(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhoneVoicemailType(i32);
#[repr(C)]
pub struct TransportDeviceAudioRoutingStatus(i32);
#[repr(transparent)]
pub struct VoipCallCoordinator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoipPhoneCall(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct VoipPhoneCallMedia(i32);
#[repr(C)]
pub struct VoipPhoneCallRejectReason(i32);
#[repr(C)]
pub struct VoipPhoneCallResourceReservationStatus(i32);
#[repr(C)]
pub struct VoipPhoneCallState(i32);
