#[cfg(feature = "implement_exclusive")]
pub trait ICorePerceptionAutomationStaticsImpl: Sized {
    fn SetActivationFactoryProvider(&self, provider: &::core::option::Option<super::super::super::Foundation::IGetActivationFactory>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICorePerceptionAutomationStatics {
    const NAME: &'static str = "Windows.Perception.Automation.Core.ICorePerceptionAutomationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICorePerceptionAutomationStaticsVtbl {
    pub const fn new<Impl: ICorePerceptionAutomationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICorePerceptionAutomationStaticsVtbl {
        unsafe extern "system" fn SetActivationFactoryProvider<Impl: ICorePerceptionAutomationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetActivationFactoryProvider(&*(&provider as *const <super::super::super::Foundation::IGetActivationFactory as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IGetActivationFactory as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICorePerceptionAutomationStatics>, base.5, SetActivationFactoryProvider::<Impl, OFFSET>)
    }
}
