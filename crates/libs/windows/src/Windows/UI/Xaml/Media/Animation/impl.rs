pub trait INavigationTransitionInfoOverrides_Impl: Sized {
    fn GetNavigationStateCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNavigationStateCore(&mut self, navigationstate: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INavigationTransitionInfoOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.INavigationTransitionInfoOverrides";
}
impl INavigationTransitionInfoOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationTransitionInfoOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationTransitionInfoOverrides_Vtbl {
        unsafe extern "system" fn GetNavigationStateCore<Impl: INavigationTransitionInfoOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNavigationStateCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNavigationStateCore<Impl: INavigationTransitionInfoOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, navigationstate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNavigationStateCore(&*(&navigationstate as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INavigationTransitionInfoOverrides, BASE_OFFSET>(),
            GetNavigationStateCore: GetNavigationStateCore::<Impl, IMPL_OFFSET>,
            SetNavigationStateCore: SetNavigationStateCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationTransitionInfoOverrides as ::windows::core::Interface>::IID
    }
}
