#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPhoneCallBlockedTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallBlockedTriggerDetails {}
impl ::core::clone::Clone for IPhoneCallBlockedTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneCallOriginDataRequestTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneCallOriginDataRequestTriggerDetails {}
impl ::core::clone::Clone for IPhoneCallOriginDataRequestTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneIncomingCallDismissedTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneIncomingCallDismissedTriggerDetails {}
impl ::core::clone::Clone for IPhoneIncomingCallDismissedTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneIncomingCallNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneIncomingCallNotificationTriggerDetails {}
impl ::core::clone::Clone for IPhoneIncomingCallNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneLineChangedTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneLineChangedTriggerDetails {}
impl ::core::clone::Clone for IPhoneLineChangedTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneNewVoicemailMessageTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneNewVoicemailMessageTriggerDetails {}
impl ::core::clone::Clone for IPhoneNewVoicemailMessageTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallBlockedReason(pub i32);
impl PhoneCallBlockedReason {
    pub const InCallBlockingList: Self = Self(0i32);
    pub const PrivateNumber: Self = Self(1i32);
    pub const UnknownNumber: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallBlockedReason {}
impl ::core::clone::Clone for PhoneCallBlockedReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallBlockedTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneCallBlockedTriggerDetails {}
impl ::core::clone::Clone for PhoneCallBlockedTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneCallOriginDataRequestTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneCallOriginDataRequestTriggerDetails {}
impl ::core::clone::Clone for PhoneCallOriginDataRequestTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneIncomingCallDismissedReason(pub i32);
impl PhoneIncomingCallDismissedReason {
    pub const Unknown: Self = Self(0i32);
    pub const CallRejected: Self = Self(1i32);
    pub const TextReply: Self = Self(2i32);
    pub const ConnectionLost: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneIncomingCallDismissedReason {}
impl ::core::clone::Clone for PhoneIncomingCallDismissedReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneIncomingCallDismissedTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneIncomingCallDismissedTriggerDetails {}
impl ::core::clone::Clone for PhoneIncomingCallDismissedTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneIncomingCallNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneIncomingCallNotificationTriggerDetails {}
impl ::core::clone::Clone for PhoneIncomingCallNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneLineChangeKind(pub i32);
impl PhoneLineChangeKind {
    pub const Added: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
    pub const PropertiesChanged: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneLineChangeKind {}
impl ::core::clone::Clone for PhoneLineChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneLineChangedTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneLineChangedTriggerDetails {}
impl ::core::clone::Clone for PhoneLineChangedTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneLineProperties(pub u32);
impl PhoneLineProperties {
    pub const None: Self = Self(0u32);
    pub const BrandingOptions: Self = Self(1u32);
    pub const CanDial: Self = Self(2u32);
    pub const CellularDetails: Self = Self(4u32);
    pub const DisplayColor: Self = Self(8u32);
    pub const DisplayName: Self = Self(16u32);
    pub const NetworkName: Self = Self(32u32);
    pub const NetworkState: Self = Self(64u32);
    pub const Transport: Self = Self(128u32);
    pub const Voicemail: Self = Self(256u32);
}
impl ::core::marker::Copy for PhoneLineProperties {}
impl ::core::clone::Clone for PhoneLineProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneNewVoicemailMessageTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneNewVoicemailMessageTriggerDetails {}
impl ::core::clone::Clone for PhoneNewVoicemailMessageTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneTriggerType(pub i32);
impl PhoneTriggerType {
    pub const NewVoicemailMessage: Self = Self(0i32);
    pub const CallHistoryChanged: Self = Self(1i32);
    pub const LineChanged: Self = Self(2i32);
    pub const AirplaneModeDisabledForEmergencyCall: Self = Self(3i32);
    pub const CallOriginDataRequest: Self = Self(4i32);
    pub const CallBlocked: Self = Self(5i32);
    pub const IncomingCallDismissed: Self = Self(6i32);
    pub const IncomingCallNotification: Self = Self(7i32);
}
impl ::core::marker::Copy for PhoneTriggerType {}
impl ::core::clone::Clone for PhoneTriggerType {
    fn clone(&self) -> Self {
        *self
    }
}
