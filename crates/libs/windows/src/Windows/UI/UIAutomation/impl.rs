#[cfg(feature = "implement_exclusive")]
pub trait IAutomationConnectionImpl: Sized {
    fn IsRemoteSystem(&self) -> ::windows::core::Result<bool>;
    fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExecutableFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationConnectionBoundObjectImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<AutomationConnection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementImpl: Sized {
    fn IsRemoteSystem(&self) -> ::windows::core::Result<bool>;
    fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExecutableFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationTextRangeImpl: Sized {}
