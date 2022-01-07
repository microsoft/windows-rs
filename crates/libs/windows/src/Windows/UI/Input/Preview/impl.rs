#[cfg(feature = "implement_exclusive")]
pub trait IInputActivationListenerPreviewStaticsImpl: Sized {
    fn CreateForApplicationWindow(&self, window: &::core::option::Option<super::super::WindowManagement::AppWindow>) -> ::windows::core::Result<super::InputActivationListener>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputActivationListenerPreviewStatics {
    const NAME: &'static str = "Windows.UI.Input.Preview.IInputActivationListenerPreviewStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInputActivationListenerPreviewStaticsVtbl {
    pub const fn new<Impl: IInputActivationListenerPreviewStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInputActivationListenerPreviewStaticsVtbl {
        unsafe extern "system" fn CreateForApplicationWindow<Impl: IInputActivationListenerPreviewStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForApplicationWindow(&*(&window as *const <super::super::WindowManagement::AppWindow as ::windows::core::Abi>::Abi as *const <super::super::WindowManagement::AppWindow as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInputActivationListenerPreviewStatics>, base.5, CreateForApplicationWindow::<Impl, OFFSET>)
    }
}
