#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`, `\"implement\"`*"]
#[cfg(feature = "deprecated")]
pub trait ISearchPaneQueryChangedEventArgs_Impl: Sized {
    fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn LinguisticDetails(&self) -> ::windows_core::Result<SearchPaneQueryLinguisticDetails>;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for ISearchPaneQueryChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchPaneQueryChangedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ISearchPaneQueryChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISearchPaneQueryChangedEventArgs_Impl, const OFFSET: isize>() -> ISearchPaneQueryChangedEventArgs_Vtbl {
        unsafe extern "system" fn QueryText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISearchPaneQueryChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISearchPaneQueryChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Language() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LinguisticDetails<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISearchPaneQueryChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LinguisticDetails() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ISearchPaneQueryChangedEventArgs, OFFSET>(),
            QueryText: QueryText::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
            LinguisticDetails: LinguisticDetails::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISearchPaneQueryChangedEventArgs as ::windows_core::ComInterface>::IID
    }
}
