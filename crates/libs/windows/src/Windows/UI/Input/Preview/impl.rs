#[cfg(all(feature = "UI_WindowManagement", feature = "implement_exclusive"))]
pub trait IInputActivationListenerPreviewStaticsImpl: Sized {
    fn CreateForApplicationWindow(&self, window: &::core::option::Option<super::super::WindowManagement::AppWindow>) -> ::windows::core::Result<super::InputActivationListener>;
}
#[cfg(all(feature = "UI_WindowManagement", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInputActivationListenerPreviewStatics {
    const NAME: &'static str = "Windows.UI.Input.Preview.IInputActivationListenerPreviewStatics";
}
#[cfg(all(feature = "UI_WindowManagement", feature = "implement_exclusive"))]
impl IInputActivationListenerPreviewStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputActivationListenerPreviewStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputActivationListenerPreviewStaticsVtbl {
        unsafe extern "system" fn CreateForApplicationWindow<Impl: IInputActivationListenerPreviewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForApplicationWindow(&*(&window as *const <super::super::WindowManagement::AppWindow as ::windows::core::Abi>::Abi as *const <super::super::WindowManagement::AppWindow as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInputActivationListenerPreviewStatics>, ::windows::core::GetTrustLevel, CreateForApplicationWindow::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputActivationListenerPreviewStatics as ::windows::core::Interface>::IID
    }
}
