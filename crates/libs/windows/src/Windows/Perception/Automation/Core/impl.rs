#[cfg(feature = "implement_exclusive")]
pub trait ICorePerceptionAutomationStaticsImpl: Sized {
    fn SetActivationFactoryProvider(&self, provider: &::core::option::Option<super::super::super::Foundation::IGetActivationFactory>) -> ::windows::core::Result<()>;
}
