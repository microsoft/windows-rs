#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICorePerceptionAutomationStatics_Impl: Sized {
    fn SetActivationFactoryProvider(&mut self, provider: &::core::option::Option<super::super::super::Foundation::IGetActivationFactory>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICorePerceptionAutomationStatics {
    const NAME: &'static str = "Windows.Perception.Automation.Core.ICorePerceptionAutomationStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICorePerceptionAutomationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICorePerceptionAutomationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICorePerceptionAutomationStatics_Vtbl {
        unsafe extern "system" fn SetActivationFactoryProvider<Impl: ICorePerceptionAutomationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActivationFactoryProvider(&*(&provider as *const <super::super::super::Foundation::IGetActivationFactory as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IGetActivationFactory as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICorePerceptionAutomationStatics, BASE_OFFSET>(),
            SetActivationFactoryProvider: SetActivationFactoryProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorePerceptionAutomationStatics as ::windows::core::Interface>::IID
    }
}
