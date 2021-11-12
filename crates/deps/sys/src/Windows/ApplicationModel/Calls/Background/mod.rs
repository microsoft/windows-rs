#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct PhoneCallBlockedReason(i32);
#[repr(transparent)]
pub struct PhoneCallBlockedTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneCallOriginDataRequestTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhoneIncomingCallDismissedReason(i32);
#[repr(transparent)]
pub struct PhoneIncomingCallDismissedTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneIncomingCallNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhoneLineChangeKind(i32);
#[repr(transparent)]
pub struct PhoneLineChangedTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhoneLineProperties(i32);
#[repr(transparent)]
pub struct PhoneNewVoicemailMessageTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhoneTriggerType(i32);
