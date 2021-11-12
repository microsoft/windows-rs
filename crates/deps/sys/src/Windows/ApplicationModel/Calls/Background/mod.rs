#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CallsBackgroundContract(i32);
pub struct IPhoneCallBlockedTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IPhoneCallOriginDataRequestTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IPhoneIncomingCallDismissedTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IPhoneIncomingCallNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IPhoneLineChangedTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IPhoneNewVoicemailMessageTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct PhoneCallBlockedReason(i32);
pub struct PhoneCallBlockedTriggerDetails(i32);
pub struct PhoneCallOriginDataRequestTriggerDetails(i32);
pub struct PhoneIncomingCallDismissedReason(i32);
pub struct PhoneIncomingCallDismissedTriggerDetails(i32);
pub struct PhoneIncomingCallNotificationTriggerDetails(i32);
pub struct PhoneLineChangeKind(i32);
pub struct PhoneLineChangedTriggerDetails(i32);
pub struct PhoneLineProperties(i32);
pub struct PhoneNewVoicemailMessageTriggerDetails(i32);
pub struct PhoneTriggerType(i32);
