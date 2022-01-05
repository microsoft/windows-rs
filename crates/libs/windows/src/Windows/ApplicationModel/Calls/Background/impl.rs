#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallBlockedTriggerDetailsImpl: Sized {
    fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CallBlockedReason(&self) -> ::windows::core::Result<PhoneCallBlockedReason>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOriginDataRequestTriggerDetailsImpl: Sized {
    fn RequestId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneIncomingCallDismissedTriggerDetailsImpl: Sized {
    fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DismissalTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn TextReplyMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Reason(&self) -> ::windows::core::Result<PhoneIncomingCallDismissedReason>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneIncomingCallNotificationTriggerDetailsImpl: Sized {
    fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CallId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineChangedTriggerDetailsImpl: Sized {
    fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ChangeType(&self) -> ::windows::core::Result<PhoneLineChangeKind>;
    fn HasLinePropertyChanged(&self, lineproperty: PhoneLineProperties) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneNewVoicemailMessageTriggerDetailsImpl: Sized {
    fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn VoicemailCount(&self) -> ::windows::core::Result<i32>;
    fn OperatorMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
