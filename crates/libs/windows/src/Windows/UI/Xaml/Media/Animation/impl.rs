pub trait INavigationTransitionInfoOverrides_Impl: Sized {
    fn GetNavigationStateCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNavigationStateCore(&self, navigationstate: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows::core::RuntimeName for INavigationTransitionInfoOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.INavigationTransitionInfoOverrides";
}
impl INavigationTransitionInfoOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationTransitionInfoOverrides_Impl, const OFFSET: isize>() -> INavigationTransitionInfoOverrides_Vtbl {
        unsafe extern "system" fn GetNavigationStateCore<Identity: ::windows::core::IUnknownImpl, Impl: INavigationTransitionInfoOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNavigationStateCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNavigationStateCore<Identity: ::windows::core::IUnknownImpl, Impl: INavigationTransitionInfoOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, navigationstate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNavigationStateCore(::core::mem::transmute(&navigationstate)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INavigationTransitionInfoOverrides, OFFSET>(),
            GetNavigationStateCore: GetNavigationStateCore::<Identity, Impl, OFFSET>,
            SetNavigationStateCore: SetNavigationStateCore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationTransitionInfoOverrides as ::windows::core::Interface>::IID
    }
}
