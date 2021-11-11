#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CallsBackgroundContract();
    fn IPhoneCallBlockedTriggerDetails();
    fn IPhoneCallOriginDataRequestTriggerDetails();
    fn IPhoneIncomingCallDismissedTriggerDetails();
    fn IPhoneIncomingCallNotificationTriggerDetails();
    fn IPhoneLineChangedTriggerDetails();
    fn IPhoneNewVoicemailMessageTriggerDetails();
    fn PhoneCallBlockedReason();
    fn PhoneCallBlockedTriggerDetails();
    fn PhoneCallOriginDataRequestTriggerDetails();
    fn PhoneIncomingCallDismissedReason();
    fn PhoneIncomingCallDismissedTriggerDetails();
    fn PhoneIncomingCallNotificationTriggerDetails();
    fn PhoneLineChangeKind();
    fn PhoneLineChangedTriggerDetails();
    fn PhoneLineProperties();
    fn PhoneNewVoicemailMessageTriggerDetails();
    fn PhoneTriggerType();
}
