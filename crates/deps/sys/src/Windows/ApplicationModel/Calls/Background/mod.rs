#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct CallsBackgroundContract(i32);
#[repr(transparent)]
pub struct IPhoneCallBlockedTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneCallOriginDataRequestTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneIncomingCallDismissedTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneIncomingCallNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneLineChangedTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneNewVoicemailMessageTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallBlockedReason(pub i32);
impl PhoneCallBlockedReason {
    pub const InCallBlockingList: PhoneCallBlockedReason = PhoneCallBlockedReason(0i32);
    pub const PrivateNumber: PhoneCallBlockedReason = PhoneCallBlockedReason(1i32);
    pub const UnknownNumber: PhoneCallBlockedReason = PhoneCallBlockedReason(2i32);
}
#[repr(transparent)]
pub struct PhoneCallBlockedTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallOriginDataRequestTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneIncomingCallDismissedReason(pub i32);
impl PhoneIncomingCallDismissedReason {
    pub const Unknown: PhoneIncomingCallDismissedReason = PhoneIncomingCallDismissedReason(0i32);
    pub const CallRejected: PhoneIncomingCallDismissedReason = PhoneIncomingCallDismissedReason(1i32);
    pub const TextReply: PhoneIncomingCallDismissedReason = PhoneIncomingCallDismissedReason(2i32);
    pub const ConnectionLost: PhoneIncomingCallDismissedReason = PhoneIncomingCallDismissedReason(3i32);
}
#[repr(transparent)]
pub struct PhoneIncomingCallDismissedTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneIncomingCallNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneLineChangeKind(pub i32);
impl PhoneLineChangeKind {
    pub const Added: PhoneLineChangeKind = PhoneLineChangeKind(0i32);
    pub const Removed: PhoneLineChangeKind = PhoneLineChangeKind(1i32);
    pub const PropertiesChanged: PhoneLineChangeKind = PhoneLineChangeKind(2i32);
}
#[repr(transparent)]
pub struct PhoneLineChangedTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneLineProperties(pub u32);
impl PhoneLineProperties {
    pub const None: PhoneLineProperties = PhoneLineProperties(0u32);
    pub const BrandingOptions: PhoneLineProperties = PhoneLineProperties(1u32);
    pub const CanDial: PhoneLineProperties = PhoneLineProperties(2u32);
    pub const CellularDetails: PhoneLineProperties = PhoneLineProperties(4u32);
    pub const DisplayColor: PhoneLineProperties = PhoneLineProperties(8u32);
    pub const DisplayName: PhoneLineProperties = PhoneLineProperties(16u32);
    pub const NetworkName: PhoneLineProperties = PhoneLineProperties(32u32);
    pub const NetworkState: PhoneLineProperties = PhoneLineProperties(64u32);
    pub const Transport: PhoneLineProperties = PhoneLineProperties(128u32);
    pub const Voicemail: PhoneLineProperties = PhoneLineProperties(256u32);
}
#[repr(transparent)]
pub struct PhoneNewVoicemailMessageTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneTriggerType(pub i32);
impl PhoneTriggerType {
    pub const NewVoicemailMessage: PhoneTriggerType = PhoneTriggerType(0i32);
    pub const CallHistoryChanged: PhoneTriggerType = PhoneTriggerType(1i32);
    pub const LineChanged: PhoneTriggerType = PhoneTriggerType(2i32);
    pub const AirplaneModeDisabledForEmergencyCall: PhoneTriggerType = PhoneTriggerType(3i32);
    pub const CallOriginDataRequest: PhoneTriggerType = PhoneTriggerType(4i32);
    pub const CallBlocked: PhoneTriggerType = PhoneTriggerType(5i32);
    pub const IncomingCallDismissed: PhoneTriggerType = PhoneTriggerType(6i32);
    pub const IncomingCallNotification: PhoneTriggerType = PhoneTriggerType(7i32);
}
