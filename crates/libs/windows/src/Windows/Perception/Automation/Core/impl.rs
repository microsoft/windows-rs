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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICorePerceptionAutomationStaticsImpl, const OFFSET: isize>() -> ICorePerceptionAutomationStaticsVtbl {
        unsafe extern "system" fn SetActivationFactoryProvider<Impl: ICorePerceptionAutomationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActivationFactoryProvider(&*(&provider as *const <super::super::super::Foundation::IGetActivationFactory as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IGetActivationFactory as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICorePerceptionAutomationStatics>, ::windows::core::GetTrustLevel, SetActivationFactoryProvider::<Impl, OFFSET>)
    }
}
