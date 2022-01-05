#[cfg(feature = "implement_exclusive")]
pub trait INamedPolicyDataImpl: Sized {
    fn Area(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<NamedPolicyKind>;
    fn IsManaged(&self) -> ::windows::core::Result<bool>;
    fn IsUserPolicy(&self) -> ::windows::core::Result<bool>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
    fn GetBoolean(&self) -> ::windows::core::Result<bool>;
    fn GetBinary(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn GetInt32(&self) -> ::windows::core::Result<i32>;
    fn GetInt64(&self) -> ::windows::core::Result<i64>;
    fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Changed(&self, changedhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<NamedPolicyData, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INamedPolicyStaticsImpl: Sized {
    fn GetPolicyFromPath(&self, area: &::windows::core::HSTRING, name: &::windows::core::HSTRING) -> ::windows::core::Result<NamedPolicyData>;
    fn GetPolicyFromPathForUser(&self, user: &::core::option::Option<super::super::System::User>, area: &::windows::core::HSTRING, name: &::windows::core::HSTRING) -> ::windows::core::Result<NamedPolicyData>;
}
