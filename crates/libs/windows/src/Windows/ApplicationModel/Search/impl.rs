#[cfg(feature = "deprecated")]
pub trait ISearchPaneQueryChangedEventArgs_Impl: Sized {
    fn QueryText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LinguisticDetails(&mut self) -> ::windows::core::Result<SearchPaneQueryLinguisticDetails>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for ISearchPaneQueryChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchPaneQueryChangedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ISearchPaneQueryChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchPaneQueryChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISearchPaneQueryChangedEventArgs_Vtbl {
        unsafe extern "system" fn QueryText<Impl: ISearchPaneQueryChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: ISearchPaneQueryChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LinguisticDetails<Impl: ISearchPaneQueryChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LinguisticDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISearchPaneQueryChangedEventArgs, BASE_OFFSET>(),
            QueryText: QueryText::<Impl, IMPL_OFFSET>,
            Language: Language::<Impl, IMPL_OFFSET>,
            LinguisticDetails: LinguisticDetails::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchPaneQueryChangedEventArgs as ::windows::core::Interface>::IID
    }
}
