#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHttpCacheDirectiveHeaderValueCollection_Impl: Sized {
    fn MaxAge(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetMaxAge(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn MaxStale(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetMaxStale(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn MinFresh(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetMinFresh(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn SharedMaxAge(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetSharedMaxAge(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn ParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpCacheDirectiveHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpCacheDirectiveHeaderValueCollection";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHttpCacheDirectiveHeaderValueCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpCacheDirectiveHeaderValueCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpCacheDirectiveHeaderValueCollection_Vtbl {
        unsafe extern "system" fn MaxAge<Impl: IHttpCacheDirectiveHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxAge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxAge<Impl: IHttpCacheDirectiveHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxAge(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxStale<Impl: IHttpCacheDirectiveHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxStale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxStale<Impl: IHttpCacheDirectiveHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxStale(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MinFresh<Impl: IHttpCacheDirectiveHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinFresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinFresh<Impl: IHttpCacheDirectiveHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinFresh(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SharedMaxAge<Impl: IHttpCacheDirectiveHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SharedMaxAge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSharedMaxAge<Impl: IHttpCacheDirectiveHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSharedMaxAge(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ParseAdd<Impl: IHttpCacheDirectiveHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpCacheDirectiveHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpCacheDirectiveHeaderValueCollection, BASE_OFFSET>(),
            MaxAge: MaxAge::<Impl, IMPL_OFFSET>,
            SetMaxAge: SetMaxAge::<Impl, IMPL_OFFSET>,
            MaxStale: MaxStale::<Impl, IMPL_OFFSET>,
            SetMaxStale: SetMaxStale::<Impl, IMPL_OFFSET>,
            MinFresh: MinFresh::<Impl, IMPL_OFFSET>,
            SetMinFresh: SetMinFresh::<Impl, IMPL_OFFSET>,
            SharedMaxAge: SharedMaxAge::<Impl, IMPL_OFFSET>,
            SetSharedMaxAge: SetSharedMaxAge::<Impl, IMPL_OFFSET>,
            ParseAdd: ParseAdd::<Impl, IMPL_OFFSET>,
            TryParseAdd: TryParseAdd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpCacheDirectiveHeaderValueCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHttpChallengeHeaderValue_Impl: Sized {
    fn Parameters(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
    fn Scheme(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Token(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpChallengeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpChallengeHeaderValue";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHttpChallengeHeaderValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpChallengeHeaderValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpChallengeHeaderValue_Vtbl {
        unsafe extern "system" fn Parameters<Impl: IHttpChallengeHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scheme<Impl: IHttpChallengeHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scheme() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Token<Impl: IHttpChallengeHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Token() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpChallengeHeaderValue, BASE_OFFSET>(),
            Parameters: Parameters::<Impl, IMPL_OFFSET>,
            Scheme: Scheme::<Impl, IMPL_OFFSET>,
            Token: Token::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpChallengeHeaderValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpChallengeHeaderValueCollection_Impl: Sized {
    fn ParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpChallengeHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpChallengeHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpChallengeHeaderValueCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpChallengeHeaderValueCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpChallengeHeaderValueCollection_Vtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpChallengeHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpChallengeHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpChallengeHeaderValueCollection, BASE_OFFSET>(),
            ParseAdd: ParseAdd::<Impl, IMPL_OFFSET>,
            TryParseAdd: TryParseAdd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpChallengeHeaderValueCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpChallengeHeaderValueFactory_Impl: Sized {
    fn CreateFromScheme(&mut self, scheme: &::windows::core::HSTRING) -> ::windows::core::Result<HttpChallengeHeaderValue>;
    fn CreateFromSchemeWithToken(&mut self, scheme: &::windows::core::HSTRING, token: &::windows::core::HSTRING) -> ::windows::core::Result<HttpChallengeHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpChallengeHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpChallengeHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpChallengeHeaderValueFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpChallengeHeaderValueFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpChallengeHeaderValueFactory_Vtbl {
        unsafe extern "system" fn CreateFromScheme<Impl: IHttpChallengeHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromScheme(&*(&scheme as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromSchemeWithToken<Impl: IHttpChallengeHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromSchemeWithToken(&*(&scheme as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpChallengeHeaderValueFactory, BASE_OFFSET>(),
            CreateFromScheme: CreateFromScheme::<Impl, IMPL_OFFSET>,
            CreateFromSchemeWithToken: CreateFromSchemeWithToken::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpChallengeHeaderValueFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpChallengeHeaderValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpChallengeHeaderValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, challengeheadervalue: &mut ::core::option::Option<HttpChallengeHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpChallengeHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpChallengeHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpChallengeHeaderValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpChallengeHeaderValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpChallengeHeaderValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IHttpChallengeHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IHttpChallengeHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, challengeheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&challengeheadervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpChallengeHeaderValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpChallengeHeaderValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpConnectionOptionHeaderValue_Impl: Sized {
    fn Token(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpConnectionOptionHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpConnectionOptionHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpConnectionOptionHeaderValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpConnectionOptionHeaderValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpConnectionOptionHeaderValue_Vtbl {
        unsafe extern "system" fn Token<Impl: IHttpConnectionOptionHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Token() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpConnectionOptionHeaderValue, BASE_OFFSET>(), Token: Token::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpConnectionOptionHeaderValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpConnectionOptionHeaderValueCollection_Impl: Sized {
    fn ParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpConnectionOptionHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpConnectionOptionHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpConnectionOptionHeaderValueCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpConnectionOptionHeaderValueCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpConnectionOptionHeaderValueCollection_Vtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpConnectionOptionHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpConnectionOptionHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpConnectionOptionHeaderValueCollection, BASE_OFFSET>(),
            ParseAdd: ParseAdd::<Impl, IMPL_OFFSET>,
            TryParseAdd: TryParseAdd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpConnectionOptionHeaderValueCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpConnectionOptionHeaderValueFactory_Impl: Sized {
    fn Create(&mut self, token: &::windows::core::HSTRING) -> ::windows::core::Result<HttpConnectionOptionHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpConnectionOptionHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpConnectionOptionHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpConnectionOptionHeaderValueFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpConnectionOptionHeaderValueFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpConnectionOptionHeaderValueFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IHttpConnectionOptionHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpConnectionOptionHeaderValueFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpConnectionOptionHeaderValueFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpConnectionOptionHeaderValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpConnectionOptionHeaderValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, connectionoptionheadervalue: &mut ::core::option::Option<HttpConnectionOptionHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpConnectionOptionHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpConnectionOptionHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpConnectionOptionHeaderValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpConnectionOptionHeaderValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpConnectionOptionHeaderValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IHttpConnectionOptionHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IHttpConnectionOptionHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, connectionoptionheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&connectionoptionheadervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpConnectionOptionHeaderValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpConnectionOptionHeaderValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingHeaderValue_Impl: Sized {
    fn ContentCoding(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentCodingHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentCodingHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentCodingHeaderValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentCodingHeaderValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpContentCodingHeaderValue_Vtbl {
        unsafe extern "system" fn ContentCoding<Impl: IHttpContentCodingHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentCoding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpContentCodingHeaderValue, BASE_OFFSET>(),
            ContentCoding: ContentCoding::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpContentCodingHeaderValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingHeaderValueCollection_Impl: Sized {
    fn ParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentCodingHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentCodingHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentCodingHeaderValueCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentCodingHeaderValueCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpContentCodingHeaderValueCollection_Vtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpContentCodingHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpContentCodingHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpContentCodingHeaderValueCollection, BASE_OFFSET>(),
            ParseAdd: ParseAdd::<Impl, IMPL_OFFSET>,
            TryParseAdd: TryParseAdd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpContentCodingHeaderValueCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingHeaderValueFactory_Impl: Sized {
    fn Create(&mut self, contentcoding: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentCodingHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentCodingHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentCodingHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentCodingHeaderValueFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentCodingHeaderValueFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpContentCodingHeaderValueFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IHttpContentCodingHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentcoding: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&contentcoding as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpContentCodingHeaderValueFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpContentCodingHeaderValueFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingHeaderValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentCodingHeaderValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, contentcodingheadervalue: &mut ::core::option::Option<HttpContentCodingHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentCodingHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentCodingHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentCodingHeaderValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentCodingHeaderValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpContentCodingHeaderValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IHttpContentCodingHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IHttpContentCodingHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contentcodingheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&contentcodingheadervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpContentCodingHeaderValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpContentCodingHeaderValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHttpContentCodingWithQualityHeaderValue_Impl: Sized {
    fn ContentCoding(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Quality(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpContentCodingWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentCodingWithQualityHeaderValue";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHttpContentCodingWithQualityHeaderValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentCodingWithQualityHeaderValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpContentCodingWithQualityHeaderValue_Vtbl {
        unsafe extern "system" fn ContentCoding<Impl: IHttpContentCodingWithQualityHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentCoding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Quality<Impl: IHttpContentCodingWithQualityHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Quality() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpContentCodingWithQualityHeaderValue, BASE_OFFSET>(),
            ContentCoding: ContentCoding::<Impl, IMPL_OFFSET>,
            Quality: Quality::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpContentCodingWithQualityHeaderValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingWithQualityHeaderValueCollection_Impl: Sized {
    fn ParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentCodingWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentCodingWithQualityHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentCodingWithQualityHeaderValueCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentCodingWithQualityHeaderValueCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpContentCodingWithQualityHeaderValueCollection_Vtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpContentCodingWithQualityHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpContentCodingWithQualityHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpContentCodingWithQualityHeaderValueCollection, BASE_OFFSET>(),
            ParseAdd: ParseAdd::<Impl, IMPL_OFFSET>,
            TryParseAdd: TryParseAdd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpContentCodingWithQualityHeaderValueCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingWithQualityHeaderValueFactory_Impl: Sized {
    fn CreateFromValue(&mut self, contentcoding: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValue>;
    fn CreateFromValueWithQuality(&mut self, contentcoding: &::windows::core::HSTRING, quality: f64) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentCodingWithQualityHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentCodingWithQualityHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentCodingWithQualityHeaderValueFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentCodingWithQualityHeaderValueFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpContentCodingWithQualityHeaderValueFactory_Vtbl {
        unsafe extern "system" fn CreateFromValue<Impl: IHttpContentCodingWithQualityHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentcoding: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromValue(&*(&contentcoding as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromValueWithQuality<Impl: IHttpContentCodingWithQualityHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentcoding: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, quality: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromValueWithQuality(&*(&contentcoding as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpContentCodingWithQualityHeaderValueFactory, BASE_OFFSET>(),
            CreateFromValue: CreateFromValue::<Impl, IMPL_OFFSET>,
            CreateFromValueWithQuality: CreateFromValueWithQuality::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpContentCodingWithQualityHeaderValueFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingWithQualityHeaderValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, contentcodingwithqualityheadervalue: &mut ::core::option::Option<HttpContentCodingWithQualityHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentCodingWithQualityHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentCodingWithQualityHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentCodingWithQualityHeaderValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentCodingWithQualityHeaderValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpContentCodingWithQualityHeaderValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IHttpContentCodingWithQualityHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IHttpContentCodingWithQualityHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contentcodingwithqualityheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&contentcodingwithqualityheadervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpContentCodingWithQualityHeaderValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpContentCodingWithQualityHeaderValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHttpContentDispositionHeaderValue_Impl: Sized {
    fn DispositionType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDispositionType(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FileName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFileName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FileNameStar(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFileNameStar(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Parameters(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
    fn Size(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn SetSize(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<u64>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpContentDispositionHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentDispositionHeaderValue";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHttpContentDispositionHeaderValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentDispositionHeaderValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpContentDispositionHeaderValue_Vtbl {
        unsafe extern "system" fn DispositionType<Impl: IHttpContentDispositionHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DispositionType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDispositionType<Impl: IHttpContentDispositionHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDispositionType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FileName<Impl: IHttpContentDispositionHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileName<Impl: IHttpContentDispositionHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FileNameStar<Impl: IHttpContentDispositionHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileNameStar() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileNameStar<Impl: IHttpContentDispositionHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileNameStar(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Name<Impl: IHttpContentDispositionHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IHttpContentDispositionHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Parameters<Impl: IHttpContentDispositionHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IHttpContentDispositionHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: IHttpContentDispositionHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(&*(&value as *const <super::super::super::Foundation::IReference<u64> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpContentDispositionHeaderValue, BASE_OFFSET>(),
            DispositionType: DispositionType::<Impl, IMPL_OFFSET>,
            SetDispositionType: SetDispositionType::<Impl, IMPL_OFFSET>,
            FileName: FileName::<Impl, IMPL_OFFSET>,
            SetFileName: SetFileName::<Impl, IMPL_OFFSET>,
            FileNameStar: FileNameStar::<Impl, IMPL_OFFSET>,
            SetFileNameStar: SetFileNameStar::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Parameters: Parameters::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpContentDispositionHeaderValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentDispositionHeaderValueFactory_Impl: Sized {
    fn Create(&mut self, dispositiontype: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentDispositionHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentDispositionHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentDispositionHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentDispositionHeaderValueFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentDispositionHeaderValueFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpContentDispositionHeaderValueFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IHttpContentDispositionHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispositiontype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&dispositiontype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpContentDispositionHeaderValueFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpContentDispositionHeaderValueFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentDispositionHeaderValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentDispositionHeaderValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, contentdispositionheadervalue: &mut ::core::option::Option<HttpContentDispositionHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentDispositionHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentDispositionHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentDispositionHeaderValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentDispositionHeaderValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpContentDispositionHeaderValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IHttpContentDispositionHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IHttpContentDispositionHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contentdispositionheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&contentdispositionheadervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpContentDispositionHeaderValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpContentDispositionHeaderValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IHttpContentHeaderCollection_Impl: Sized {
    fn ContentDisposition(&mut self) -> ::windows::core::Result<HttpContentDispositionHeaderValue>;
    fn SetContentDisposition(&mut self, value: &::core::option::Option<HttpContentDispositionHeaderValue>) -> ::windows::core::Result<()>;
    fn ContentEncoding(&mut self) -> ::windows::core::Result<HttpContentCodingHeaderValueCollection>;
    fn ContentLanguage(&mut self) -> ::windows::core::Result<HttpLanguageHeaderValueCollection>;
    fn ContentLength(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn SetContentLength(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<u64>>) -> ::windows::core::Result<()>;
    fn ContentLocation(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetContentLocation(&mut self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ContentMD5(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetContentMD5(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn ContentRange(&mut self) -> ::windows::core::Result<HttpContentRangeHeaderValue>;
    fn SetContentRange(&mut self, value: &::core::option::Option<HttpContentRangeHeaderValue>) -> ::windows::core::Result<()>;
    fn ContentType(&mut self) -> ::windows::core::Result<HttpMediaTypeHeaderValue>;
    fn SetContentType(&mut self, value: &::core::option::Option<HttpMediaTypeHeaderValue>) -> ::windows::core::Result<()>;
    fn Expires(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetExpires(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn LastModified(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetLastModified(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Append(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryAppendWithoutValidation(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpContentHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentHeaderCollection";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IHttpContentHeaderCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentHeaderCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpContentHeaderCollection_Vtbl {
        unsafe extern "system" fn ContentDisposition<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentDisposition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentDisposition<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentDisposition(&*(&value as *const <HttpContentDispositionHeaderValue as ::windows::core::Abi>::Abi as *const <HttpContentDispositionHeaderValue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentEncoding<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentEncoding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentLanguage<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentLength<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentLength<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentLength(&*(&value as *const <super::super::super::Foundation::IReference<u64> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentLocation<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentLocation<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentLocation(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentMD5<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentMD5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentMD5<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentMD5(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentRange<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentRange<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentRange(&*(&value as *const <HttpContentRangeHeaderValue as ::windows::core::Abi>::Abi as *const <HttpContentRangeHeaderValue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentType<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentType<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentType(&*(&value as *const <HttpMediaTypeHeaderValue as ::windows::core::Abi>::Abi as *const <HttpMediaTypeHeaderValue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Expires<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Expires() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpires<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpires(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LastModified<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastModified() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastModified<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastModified(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Append<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryAppendWithoutValidation<Impl: IHttpContentHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryAppendWithoutValidation(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpContentHeaderCollection, BASE_OFFSET>(),
            ContentDisposition: ContentDisposition::<Impl, IMPL_OFFSET>,
            SetContentDisposition: SetContentDisposition::<Impl, IMPL_OFFSET>,
            ContentEncoding: ContentEncoding::<Impl, IMPL_OFFSET>,
            ContentLanguage: ContentLanguage::<Impl, IMPL_OFFSET>,
            ContentLength: ContentLength::<Impl, IMPL_OFFSET>,
            SetContentLength: SetContentLength::<Impl, IMPL_OFFSET>,
            ContentLocation: ContentLocation::<Impl, IMPL_OFFSET>,
            SetContentLocation: SetContentLocation::<Impl, IMPL_OFFSET>,
            ContentMD5: ContentMD5::<Impl, IMPL_OFFSET>,
            SetContentMD5: SetContentMD5::<Impl, IMPL_OFFSET>,
            ContentRange: ContentRange::<Impl, IMPL_OFFSET>,
            SetContentRange: SetContentRange::<Impl, IMPL_OFFSET>,
            ContentType: ContentType::<Impl, IMPL_OFFSET>,
            SetContentType: SetContentType::<Impl, IMPL_OFFSET>,
            Expires: Expires::<Impl, IMPL_OFFSET>,
            SetExpires: SetExpires::<Impl, IMPL_OFFSET>,
            LastModified: LastModified::<Impl, IMPL_OFFSET>,
            SetLastModified: SetLastModified::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            TryAppendWithoutValidation: TryAppendWithoutValidation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpContentHeaderCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHttpContentRangeHeaderValue_Impl: Sized {
    fn FirstBytePosition(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn LastBytePosition(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn Length(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn Unit(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUnit(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpContentRangeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentRangeHeaderValue";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHttpContentRangeHeaderValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentRangeHeaderValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpContentRangeHeaderValue_Vtbl {
        unsafe extern "system" fn FirstBytePosition<Impl: IHttpContentRangeHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstBytePosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBytePosition<Impl: IHttpContentRangeHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastBytePosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: IHttpContentRangeHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unit<Impl: IHttpContentRangeHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnit<Impl: IHttpContentRangeHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnit(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpContentRangeHeaderValue, BASE_OFFSET>(),
            FirstBytePosition: FirstBytePosition::<Impl, IMPL_OFFSET>,
            LastBytePosition: LastBytePosition::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
            Unit: Unit::<Impl, IMPL_OFFSET>,
            SetUnit: SetUnit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpContentRangeHeaderValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentRangeHeaderValueFactory_Impl: Sized {
    fn CreateFromLength(&mut self, length: u64) -> ::windows::core::Result<HttpContentRangeHeaderValue>;
    fn CreateFromRange(&mut self, from: u64, to: u64) -> ::windows::core::Result<HttpContentRangeHeaderValue>;
    fn CreateFromRangeWithLength(&mut self, from: u64, to: u64, length: u64) -> ::windows::core::Result<HttpContentRangeHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentRangeHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentRangeHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentRangeHeaderValueFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentRangeHeaderValueFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpContentRangeHeaderValueFactory_Vtbl {
        unsafe extern "system" fn CreateFromLength<Impl: IHttpContentRangeHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromLength(length) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromRange<Impl: IHttpContentRangeHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, from: u64, to: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromRange(from, to) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromRangeWithLength<Impl: IHttpContentRangeHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, from: u64, to: u64, length: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromRangeWithLength(from, to, length) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpContentRangeHeaderValueFactory, BASE_OFFSET>(),
            CreateFromLength: CreateFromLength::<Impl, IMPL_OFFSET>,
            CreateFromRange: CreateFromRange::<Impl, IMPL_OFFSET>,
            CreateFromRangeWithLength: CreateFromRangeWithLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpContentRangeHeaderValueFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentRangeHeaderValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentRangeHeaderValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, contentrangeheadervalue: &mut ::core::option::Option<HttpContentRangeHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentRangeHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentRangeHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentRangeHeaderValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentRangeHeaderValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpContentRangeHeaderValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IHttpContentRangeHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IHttpContentRangeHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contentrangeheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&contentrangeheadervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpContentRangeHeaderValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpContentRangeHeaderValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCookiePairHeaderValue_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpCookiePairHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpCookiePairHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpCookiePairHeaderValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpCookiePairHeaderValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpCookiePairHeaderValue_Vtbl {
        unsafe extern "system" fn Name<Impl: IHttpCookiePairHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IHttpCookiePairHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IHttpCookiePairHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpCookiePairHeaderValue, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpCookiePairHeaderValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCookiePairHeaderValueCollection_Impl: Sized {
    fn ParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpCookiePairHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpCookiePairHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpCookiePairHeaderValueCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpCookiePairHeaderValueCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpCookiePairHeaderValueCollection_Vtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpCookiePairHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpCookiePairHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpCookiePairHeaderValueCollection, BASE_OFFSET>(),
            ParseAdd: ParseAdd::<Impl, IMPL_OFFSET>,
            TryParseAdd: TryParseAdd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpCookiePairHeaderValueCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCookiePairHeaderValueFactory_Impl: Sized {
    fn CreateFromName(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCookiePairHeaderValue>;
    fn CreateFromNameWithValue(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCookiePairHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpCookiePairHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpCookiePairHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpCookiePairHeaderValueFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpCookiePairHeaderValueFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpCookiePairHeaderValueFactory_Vtbl {
        unsafe extern "system" fn CreateFromName<Impl: IHttpCookiePairHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromNameWithValue<Impl: IHttpCookiePairHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromNameWithValue(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpCookiePairHeaderValueFactory, BASE_OFFSET>(),
            CreateFromName: CreateFromName::<Impl, IMPL_OFFSET>,
            CreateFromNameWithValue: CreateFromNameWithValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpCookiePairHeaderValueFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCookiePairHeaderValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCookiePairHeaderValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, cookiepairheadervalue: &mut ::core::option::Option<HttpCookiePairHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpCookiePairHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpCookiePairHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpCookiePairHeaderValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpCookiePairHeaderValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpCookiePairHeaderValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IHttpCookiePairHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IHttpCookiePairHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, cookiepairheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&cookiepairheadervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpCookiePairHeaderValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpCookiePairHeaderValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHttpCredentialsHeaderValue_Impl: Sized {
    fn Parameters(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
    fn Scheme(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Token(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpCredentialsHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpCredentialsHeaderValue";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHttpCredentialsHeaderValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpCredentialsHeaderValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpCredentialsHeaderValue_Vtbl {
        unsafe extern "system" fn Parameters<Impl: IHttpCredentialsHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scheme<Impl: IHttpCredentialsHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scheme() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Token<Impl: IHttpCredentialsHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Token() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpCredentialsHeaderValue, BASE_OFFSET>(),
            Parameters: Parameters::<Impl, IMPL_OFFSET>,
            Scheme: Scheme::<Impl, IMPL_OFFSET>,
            Token: Token::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpCredentialsHeaderValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCredentialsHeaderValueFactory_Impl: Sized {
    fn CreateFromScheme(&mut self, scheme: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCredentialsHeaderValue>;
    fn CreateFromSchemeWithToken(&mut self, scheme: &::windows::core::HSTRING, token: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCredentialsHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpCredentialsHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpCredentialsHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpCredentialsHeaderValueFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpCredentialsHeaderValueFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpCredentialsHeaderValueFactory_Vtbl {
        unsafe extern "system" fn CreateFromScheme<Impl: IHttpCredentialsHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromScheme(&*(&scheme as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromSchemeWithToken<Impl: IHttpCredentialsHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromSchemeWithToken(&*(&scheme as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpCredentialsHeaderValueFactory, BASE_OFFSET>(),
            CreateFromScheme: CreateFromScheme::<Impl, IMPL_OFFSET>,
            CreateFromSchemeWithToken: CreateFromSchemeWithToken::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpCredentialsHeaderValueFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCredentialsHeaderValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCredentialsHeaderValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, credentialsheadervalue: &mut ::core::option::Option<HttpCredentialsHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpCredentialsHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpCredentialsHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpCredentialsHeaderValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpCredentialsHeaderValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpCredentialsHeaderValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IHttpCredentialsHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IHttpCredentialsHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, credentialsheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&credentialsheadervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpCredentialsHeaderValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpCredentialsHeaderValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHttpDateOrDeltaHeaderValue_Impl: Sized {
    fn Date(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn Delta(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpDateOrDeltaHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpDateOrDeltaHeaderValue";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHttpDateOrDeltaHeaderValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpDateOrDeltaHeaderValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpDateOrDeltaHeaderValue_Vtbl {
        unsafe extern "system" fn Date<Impl: IHttpDateOrDeltaHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Date() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delta<Impl: IHttpDateOrDeltaHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpDateOrDeltaHeaderValue, BASE_OFFSET>(),
            Date: Date::<Impl, IMPL_OFFSET>,
            Delta: Delta::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpDateOrDeltaHeaderValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpDateOrDeltaHeaderValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpDateOrDeltaHeaderValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, dateordeltaheadervalue: &mut ::core::option::Option<HttpDateOrDeltaHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpDateOrDeltaHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpDateOrDeltaHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpDateOrDeltaHeaderValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpDateOrDeltaHeaderValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpDateOrDeltaHeaderValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IHttpDateOrDeltaHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IHttpDateOrDeltaHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, dateordeltaheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&dateordeltaheadervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpDateOrDeltaHeaderValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpDateOrDeltaHeaderValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHttpExpectationHeaderValue_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Parameters(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpExpectationHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpExpectationHeaderValue";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHttpExpectationHeaderValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpExpectationHeaderValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpExpectationHeaderValue_Vtbl {
        unsafe extern "system" fn Name<Impl: IHttpExpectationHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IHttpExpectationHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IHttpExpectationHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Parameters<Impl: IHttpExpectationHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpExpectationHeaderValue, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Parameters: Parameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpExpectationHeaderValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpExpectationHeaderValueCollection_Impl: Sized {
    fn ParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpExpectationHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpExpectationHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpExpectationHeaderValueCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpExpectationHeaderValueCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpExpectationHeaderValueCollection_Vtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpExpectationHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpExpectationHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpExpectationHeaderValueCollection, BASE_OFFSET>(),
            ParseAdd: ParseAdd::<Impl, IMPL_OFFSET>,
            TryParseAdd: TryParseAdd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpExpectationHeaderValueCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpExpectationHeaderValueFactory_Impl: Sized {
    fn CreateFromName(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<HttpExpectationHeaderValue>;
    fn CreateFromNameWithValue(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<HttpExpectationHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpExpectationHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpExpectationHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpExpectationHeaderValueFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpExpectationHeaderValueFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpExpectationHeaderValueFactory_Vtbl {
        unsafe extern "system" fn CreateFromName<Impl: IHttpExpectationHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromNameWithValue<Impl: IHttpExpectationHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromNameWithValue(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpExpectationHeaderValueFactory, BASE_OFFSET>(),
            CreateFromName: CreateFromName::<Impl, IMPL_OFFSET>,
            CreateFromNameWithValue: CreateFromNameWithValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpExpectationHeaderValueFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpExpectationHeaderValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpExpectationHeaderValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, expectationheadervalue: &mut ::core::option::Option<HttpExpectationHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpExpectationHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpExpectationHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpExpectationHeaderValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpExpectationHeaderValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpExpectationHeaderValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IHttpExpectationHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IHttpExpectationHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, expectationheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&expectationheadervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpExpectationHeaderValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpExpectationHeaderValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpLanguageHeaderValueCollection_Impl: Sized {
    fn ParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpLanguageHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpLanguageHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpLanguageHeaderValueCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpLanguageHeaderValueCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpLanguageHeaderValueCollection_Vtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpLanguageHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpLanguageHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpLanguageHeaderValueCollection, BASE_OFFSET>(),
            ParseAdd: ParseAdd::<Impl, IMPL_OFFSET>,
            TryParseAdd: TryParseAdd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpLanguageHeaderValueCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHttpLanguageRangeWithQualityHeaderValue_Impl: Sized {
    fn LanguageRange(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Quality(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpLanguageRangeWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpLanguageRangeWithQualityHeaderValue";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHttpLanguageRangeWithQualityHeaderValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpLanguageRangeWithQualityHeaderValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpLanguageRangeWithQualityHeaderValue_Vtbl {
        unsafe extern "system" fn LanguageRange<Impl: IHttpLanguageRangeWithQualityHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Quality<Impl: IHttpLanguageRangeWithQualityHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Quality() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpLanguageRangeWithQualityHeaderValue, BASE_OFFSET>(),
            LanguageRange: LanguageRange::<Impl, IMPL_OFFSET>,
            Quality: Quality::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpLanguageRangeWithQualityHeaderValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpLanguageRangeWithQualityHeaderValueCollection_Impl: Sized {
    fn ParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpLanguageRangeWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpLanguageRangeWithQualityHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpLanguageRangeWithQualityHeaderValueCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpLanguageRangeWithQualityHeaderValueCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpLanguageRangeWithQualityHeaderValueCollection_Vtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpLanguageRangeWithQualityHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpLanguageRangeWithQualityHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpLanguageRangeWithQualityHeaderValueCollection, BASE_OFFSET>(),
            ParseAdd: ParseAdd::<Impl, IMPL_OFFSET>,
            TryParseAdd: TryParseAdd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpLanguageRangeWithQualityHeaderValueCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpLanguageRangeWithQualityHeaderValueFactory_Impl: Sized {
    fn CreateFromLanguageRange(&mut self, languagerange: &::windows::core::HSTRING) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValue>;
    fn CreateFromLanguageRangeWithQuality(&mut self, languagerange: &::windows::core::HSTRING, quality: f64) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpLanguageRangeWithQualityHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpLanguageRangeWithQualityHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpLanguageRangeWithQualityHeaderValueFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpLanguageRangeWithQualityHeaderValueFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpLanguageRangeWithQualityHeaderValueFactory_Vtbl {
        unsafe extern "system" fn CreateFromLanguageRange<Impl: IHttpLanguageRangeWithQualityHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagerange: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromLanguageRange(&*(&languagerange as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromLanguageRangeWithQuality<Impl: IHttpLanguageRangeWithQualityHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagerange: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, quality: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromLanguageRangeWithQuality(&*(&languagerange as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpLanguageRangeWithQualityHeaderValueFactory, BASE_OFFSET>(),
            CreateFromLanguageRange: CreateFromLanguageRange::<Impl, IMPL_OFFSET>,
            CreateFromLanguageRangeWithQuality: CreateFromLanguageRangeWithQuality::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpLanguageRangeWithQualityHeaderValueFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpLanguageRangeWithQualityHeaderValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, languagerangewithqualityheadervalue: &mut ::core::option::Option<HttpLanguageRangeWithQualityHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpLanguageRangeWithQualityHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpLanguageRangeWithQualityHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpLanguageRangeWithQualityHeaderValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpLanguageRangeWithQualityHeaderValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpLanguageRangeWithQualityHeaderValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IHttpLanguageRangeWithQualityHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IHttpLanguageRangeWithQualityHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, languagerangewithqualityheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&languagerangewithqualityheadervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpLanguageRangeWithQualityHeaderValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpLanguageRangeWithQualityHeaderValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHttpMediaTypeHeaderValue_Impl: Sized {
    fn CharSet(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCharSet(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MediaType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMediaType(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Parameters(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpMediaTypeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpMediaTypeHeaderValue";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHttpMediaTypeHeaderValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMediaTypeHeaderValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpMediaTypeHeaderValue_Vtbl {
        unsafe extern "system" fn CharSet<Impl: IHttpMediaTypeHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharSet<Impl: IHttpMediaTypeHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCharSet(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaType<Impl: IHttpMediaTypeHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaType<Impl: IHttpMediaTypeHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Parameters<Impl: IHttpMediaTypeHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpMediaTypeHeaderValue, BASE_OFFSET>(),
            CharSet: CharSet::<Impl, IMPL_OFFSET>,
            SetCharSet: SetCharSet::<Impl, IMPL_OFFSET>,
            MediaType: MediaType::<Impl, IMPL_OFFSET>,
            SetMediaType: SetMediaType::<Impl, IMPL_OFFSET>,
            Parameters: Parameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpMediaTypeHeaderValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeHeaderValueFactory_Impl: Sized {
    fn Create(&mut self, mediatype: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMediaTypeHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpMediaTypeHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpMediaTypeHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpMediaTypeHeaderValueFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMediaTypeHeaderValueFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpMediaTypeHeaderValueFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IHttpMediaTypeHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&mediatype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpMediaTypeHeaderValueFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpMediaTypeHeaderValueFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeHeaderValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMediaTypeHeaderValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, mediatypeheadervalue: &mut ::core::option::Option<HttpMediaTypeHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpMediaTypeHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpMediaTypeHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpMediaTypeHeaderValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMediaTypeHeaderValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpMediaTypeHeaderValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IHttpMediaTypeHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IHttpMediaTypeHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mediatypeheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&mediatypeheadervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpMediaTypeHeaderValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpMediaTypeHeaderValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHttpMediaTypeWithQualityHeaderValue_Impl: Sized {
    fn CharSet(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCharSet(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MediaType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMediaType(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Parameters(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
    fn Quality(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
    fn SetQuality(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpMediaTypeWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpMediaTypeWithQualityHeaderValue";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHttpMediaTypeWithQualityHeaderValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMediaTypeWithQualityHeaderValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpMediaTypeWithQualityHeaderValue_Vtbl {
        unsafe extern "system" fn CharSet<Impl: IHttpMediaTypeWithQualityHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharSet<Impl: IHttpMediaTypeWithQualityHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCharSet(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaType<Impl: IHttpMediaTypeWithQualityHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaType<Impl: IHttpMediaTypeWithQualityHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Parameters<Impl: IHttpMediaTypeWithQualityHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Quality<Impl: IHttpMediaTypeWithQualityHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Quality() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuality<Impl: IHttpMediaTypeWithQualityHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuality(&*(&value as *const <super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpMediaTypeWithQualityHeaderValue, BASE_OFFSET>(),
            CharSet: CharSet::<Impl, IMPL_OFFSET>,
            SetCharSet: SetCharSet::<Impl, IMPL_OFFSET>,
            MediaType: MediaType::<Impl, IMPL_OFFSET>,
            SetMediaType: SetMediaType::<Impl, IMPL_OFFSET>,
            Parameters: Parameters::<Impl, IMPL_OFFSET>,
            Quality: Quality::<Impl, IMPL_OFFSET>,
            SetQuality: SetQuality::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpMediaTypeWithQualityHeaderValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeWithQualityHeaderValueCollection_Impl: Sized {
    fn ParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpMediaTypeWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpMediaTypeWithQualityHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpMediaTypeWithQualityHeaderValueCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMediaTypeWithQualityHeaderValueCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpMediaTypeWithQualityHeaderValueCollection_Vtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpMediaTypeWithQualityHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpMediaTypeWithQualityHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpMediaTypeWithQualityHeaderValueCollection, BASE_OFFSET>(),
            ParseAdd: ParseAdd::<Impl, IMPL_OFFSET>,
            TryParseAdd: TryParseAdd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpMediaTypeWithQualityHeaderValueCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeWithQualityHeaderValueFactory_Impl: Sized {
    fn CreateFromMediaType(&mut self, mediatype: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValue>;
    fn CreateFromMediaTypeWithQuality(&mut self, mediatype: &::windows::core::HSTRING, quality: f64) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpMediaTypeWithQualityHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpMediaTypeWithQualityHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpMediaTypeWithQualityHeaderValueFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMediaTypeWithQualityHeaderValueFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpMediaTypeWithQualityHeaderValueFactory_Vtbl {
        unsafe extern "system" fn CreateFromMediaType<Impl: IHttpMediaTypeWithQualityHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromMediaType(&*(&mediatype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromMediaTypeWithQuality<Impl: IHttpMediaTypeWithQualityHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, quality: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromMediaTypeWithQuality(&*(&mediatype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpMediaTypeWithQualityHeaderValueFactory, BASE_OFFSET>(),
            CreateFromMediaType: CreateFromMediaType::<Impl, IMPL_OFFSET>,
            CreateFromMediaTypeWithQuality: CreateFromMediaTypeWithQuality::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpMediaTypeWithQualityHeaderValueFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeWithQualityHeaderValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, mediatypewithqualityheadervalue: &mut ::core::option::Option<HttpMediaTypeWithQualityHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpMediaTypeWithQualityHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpMediaTypeWithQualityHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpMediaTypeWithQualityHeaderValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMediaTypeWithQualityHeaderValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpMediaTypeWithQualityHeaderValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IHttpMediaTypeWithQualityHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IHttpMediaTypeWithQualityHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mediatypewithqualityheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&mediatypewithqualityheadervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpMediaTypeWithQualityHeaderValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpMediaTypeWithQualityHeaderValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMethodHeaderValueCollection_Impl: Sized {
    fn ParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpMethodHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpMethodHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpMethodHeaderValueCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMethodHeaderValueCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpMethodHeaderValueCollection_Vtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpMethodHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpMethodHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpMethodHeaderValueCollection, BASE_OFFSET>(),
            ParseAdd: ParseAdd::<Impl, IMPL_OFFSET>,
            TryParseAdd: TryParseAdd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpMethodHeaderValueCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpNameValueHeaderValue_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpNameValueHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpNameValueHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpNameValueHeaderValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpNameValueHeaderValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpNameValueHeaderValue_Vtbl {
        unsafe extern "system" fn Name<Impl: IHttpNameValueHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IHttpNameValueHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IHttpNameValueHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpNameValueHeaderValue, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpNameValueHeaderValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpNameValueHeaderValueFactory_Impl: Sized {
    fn CreateFromName(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<HttpNameValueHeaderValue>;
    fn CreateFromNameWithValue(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<HttpNameValueHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpNameValueHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpNameValueHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpNameValueHeaderValueFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpNameValueHeaderValueFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpNameValueHeaderValueFactory_Vtbl {
        unsafe extern "system" fn CreateFromName<Impl: IHttpNameValueHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromNameWithValue<Impl: IHttpNameValueHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromNameWithValue(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpNameValueHeaderValueFactory, BASE_OFFSET>(),
            CreateFromName: CreateFromName::<Impl, IMPL_OFFSET>,
            CreateFromNameWithValue: CreateFromNameWithValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpNameValueHeaderValueFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpNameValueHeaderValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpNameValueHeaderValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, namevalueheadervalue: &mut ::core::option::Option<HttpNameValueHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpNameValueHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpNameValueHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpNameValueHeaderValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpNameValueHeaderValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpNameValueHeaderValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IHttpNameValueHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IHttpNameValueHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, namevalueheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&namevalueheadervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpNameValueHeaderValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpNameValueHeaderValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductHeaderValue_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Version(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpProductHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpProductHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpProductHeaderValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpProductHeaderValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpProductHeaderValue_Vtbl {
        unsafe extern "system" fn Name<Impl: IHttpProductHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: IHttpProductHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpProductHeaderValue, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpProductHeaderValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductHeaderValueFactory_Impl: Sized {
    fn CreateFromName(&mut self, productname: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductHeaderValue>;
    fn CreateFromNameWithVersion(&mut self, productname: &::windows::core::HSTRING, productversion: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpProductHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpProductHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpProductHeaderValueFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpProductHeaderValueFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpProductHeaderValueFactory_Vtbl {
        unsafe extern "system" fn CreateFromName<Impl: IHttpProductHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromName(&*(&productname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromNameWithVersion<Impl: IHttpProductHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, productversion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromNameWithVersion(&*(&productname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&productversion as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpProductHeaderValueFactory, BASE_OFFSET>(),
            CreateFromName: CreateFromName::<Impl, IMPL_OFFSET>,
            CreateFromNameWithVersion: CreateFromNameWithVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpProductHeaderValueFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductHeaderValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductHeaderValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, productheadervalue: &mut ::core::option::Option<HttpProductHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpProductHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpProductHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpProductHeaderValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpProductHeaderValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpProductHeaderValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IHttpProductHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IHttpProductHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, productheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&productheadervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpProductHeaderValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpProductHeaderValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductInfoHeaderValue_Impl: Sized {
    fn Product(&mut self) -> ::windows::core::Result<HttpProductHeaderValue>;
    fn Comment(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpProductInfoHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpProductInfoHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpProductInfoHeaderValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpProductInfoHeaderValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpProductInfoHeaderValue_Vtbl {
        unsafe extern "system" fn Product<Impl: IHttpProductInfoHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Product() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Impl: IHttpProductInfoHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Comment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpProductInfoHeaderValue, BASE_OFFSET>(),
            Product: Product::<Impl, IMPL_OFFSET>,
            Comment: Comment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpProductInfoHeaderValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductInfoHeaderValueCollection_Impl: Sized {
    fn ParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpProductInfoHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpProductInfoHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpProductInfoHeaderValueCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpProductInfoHeaderValueCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpProductInfoHeaderValueCollection_Vtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpProductInfoHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpProductInfoHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpProductInfoHeaderValueCollection, BASE_OFFSET>(),
            ParseAdd: ParseAdd::<Impl, IMPL_OFFSET>,
            TryParseAdd: TryParseAdd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpProductInfoHeaderValueCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductInfoHeaderValueFactory_Impl: Sized {
    fn CreateFromComment(&mut self, productcomment: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductInfoHeaderValue>;
    fn CreateFromNameWithVersion(&mut self, productname: &::windows::core::HSTRING, productversion: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductInfoHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpProductInfoHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpProductInfoHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpProductInfoHeaderValueFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpProductInfoHeaderValueFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpProductInfoHeaderValueFactory_Vtbl {
        unsafe extern "system" fn CreateFromComment<Impl: IHttpProductInfoHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productcomment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromComment(&*(&productcomment as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromNameWithVersion<Impl: IHttpProductInfoHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, productversion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromNameWithVersion(&*(&productname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&productversion as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpProductInfoHeaderValueFactory, BASE_OFFSET>(),
            CreateFromComment: CreateFromComment::<Impl, IMPL_OFFSET>,
            CreateFromNameWithVersion: CreateFromNameWithVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpProductInfoHeaderValueFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductInfoHeaderValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductInfoHeaderValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, productinfoheadervalue: &mut ::core::option::Option<HttpProductInfoHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpProductInfoHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpProductInfoHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpProductInfoHeaderValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpProductInfoHeaderValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpProductInfoHeaderValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IHttpProductInfoHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IHttpProductInfoHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, productinfoheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&productinfoheadervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpProductInfoHeaderValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpProductInfoHeaderValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Networking", feature = "implement_exclusive"))]
pub trait IHttpRequestHeaderCollection_Impl: Sized {
    fn Accept(&mut self) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValueCollection>;
    fn AcceptEncoding(&mut self) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValueCollection>;
    fn AcceptLanguage(&mut self) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValueCollection>;
    fn Authorization(&mut self) -> ::windows::core::Result<HttpCredentialsHeaderValue>;
    fn SetAuthorization(&mut self, value: &::core::option::Option<HttpCredentialsHeaderValue>) -> ::windows::core::Result<()>;
    fn CacheControl(&mut self) -> ::windows::core::Result<HttpCacheDirectiveHeaderValueCollection>;
    fn Connection(&mut self) -> ::windows::core::Result<HttpConnectionOptionHeaderValueCollection>;
    fn Cookie(&mut self) -> ::windows::core::Result<HttpCookiePairHeaderValueCollection>;
    fn Date(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetDate(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Expect(&mut self) -> ::windows::core::Result<HttpExpectationHeaderValueCollection>;
    fn From(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFrom(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Host(&mut self) -> ::windows::core::Result<super::super::super::Networking::HostName>;
    fn SetHost(&mut self, value: &::core::option::Option<super::super::super::Networking::HostName>) -> ::windows::core::Result<()>;
    fn IfModifiedSince(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetIfModifiedSince(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn IfUnmodifiedSince(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetIfUnmodifiedSince(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn MaxForwards(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetMaxForwards(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn ProxyAuthorization(&mut self) -> ::windows::core::Result<HttpCredentialsHeaderValue>;
    fn SetProxyAuthorization(&mut self, value: &::core::option::Option<HttpCredentialsHeaderValue>) -> ::windows::core::Result<()>;
    fn Referer(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetReferer(&mut self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn TransferEncoding(&mut self) -> ::windows::core::Result<HttpTransferCodingHeaderValueCollection>;
    fn UserAgent(&mut self) -> ::windows::core::Result<HttpProductInfoHeaderValueCollection>;
    fn Append(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryAppendWithoutValidation(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Networking", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpRequestHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpRequestHeaderCollection";
}
#[cfg(all(feature = "Foundation", feature = "Networking", feature = "implement_exclusive"))]
impl IHttpRequestHeaderCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpRequestHeaderCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpRequestHeaderCollection_Vtbl {
        unsafe extern "system" fn Accept<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Accept() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptEncoding<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceptEncoding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptLanguage<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceptLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authorization<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authorization() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthorization<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthorization(&*(&value as *const <HttpCredentialsHeaderValue as ::windows::core::Abi>::Abi as *const <HttpCredentialsHeaderValue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CacheControl<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CacheControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connection<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookie<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cookie() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Date<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Date() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDate<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDate(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Expect<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Expect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn From<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).From() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrom<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrom(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Host<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Host() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHost<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHost(&*(&value as *const <super::super::super::Networking::HostName as ::windows::core::Abi>::Abi as *const <super::super::super::Networking::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IfModifiedSince<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IfModifiedSince() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIfModifiedSince<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIfModifiedSince(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IfUnmodifiedSince<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IfUnmodifiedSince() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIfUnmodifiedSince<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIfUnmodifiedSince(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxForwards<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxForwards() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxForwards<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxForwards(&*(&value as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProxyAuthorization<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyAuthorization() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyAuthorization<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxyAuthorization(&*(&value as *const <HttpCredentialsHeaderValue as ::windows::core::Abi>::Abi as *const <HttpCredentialsHeaderValue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Referer<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Referer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReferer<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReferer(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransferEncoding<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferEncoding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserAgent<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserAgent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryAppendWithoutValidation<Impl: IHttpRequestHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryAppendWithoutValidation(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpRequestHeaderCollection, BASE_OFFSET>(),
            Accept: Accept::<Impl, IMPL_OFFSET>,
            AcceptEncoding: AcceptEncoding::<Impl, IMPL_OFFSET>,
            AcceptLanguage: AcceptLanguage::<Impl, IMPL_OFFSET>,
            Authorization: Authorization::<Impl, IMPL_OFFSET>,
            SetAuthorization: SetAuthorization::<Impl, IMPL_OFFSET>,
            CacheControl: CacheControl::<Impl, IMPL_OFFSET>,
            Connection: Connection::<Impl, IMPL_OFFSET>,
            Cookie: Cookie::<Impl, IMPL_OFFSET>,
            Date: Date::<Impl, IMPL_OFFSET>,
            SetDate: SetDate::<Impl, IMPL_OFFSET>,
            Expect: Expect::<Impl, IMPL_OFFSET>,
            From: From::<Impl, IMPL_OFFSET>,
            SetFrom: SetFrom::<Impl, IMPL_OFFSET>,
            Host: Host::<Impl, IMPL_OFFSET>,
            SetHost: SetHost::<Impl, IMPL_OFFSET>,
            IfModifiedSince: IfModifiedSince::<Impl, IMPL_OFFSET>,
            SetIfModifiedSince: SetIfModifiedSince::<Impl, IMPL_OFFSET>,
            IfUnmodifiedSince: IfUnmodifiedSince::<Impl, IMPL_OFFSET>,
            SetIfUnmodifiedSince: SetIfUnmodifiedSince::<Impl, IMPL_OFFSET>,
            MaxForwards: MaxForwards::<Impl, IMPL_OFFSET>,
            SetMaxForwards: SetMaxForwards::<Impl, IMPL_OFFSET>,
            ProxyAuthorization: ProxyAuthorization::<Impl, IMPL_OFFSET>,
            SetProxyAuthorization: SetProxyAuthorization::<Impl, IMPL_OFFSET>,
            Referer: Referer::<Impl, IMPL_OFFSET>,
            SetReferer: SetReferer::<Impl, IMPL_OFFSET>,
            TransferEncoding: TransferEncoding::<Impl, IMPL_OFFSET>,
            UserAgent: UserAgent::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            TryAppendWithoutValidation: TryAppendWithoutValidation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpRequestHeaderCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHttpResponseHeaderCollection_Impl: Sized {
    fn Age(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetAge(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn Allow(&mut self) -> ::windows::core::Result<HttpMethodHeaderValueCollection>;
    fn CacheControl(&mut self) -> ::windows::core::Result<HttpCacheDirectiveHeaderValueCollection>;
    fn Connection(&mut self) -> ::windows::core::Result<HttpConnectionOptionHeaderValueCollection>;
    fn Date(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetDate(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetLocation(&mut self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ProxyAuthenticate(&mut self) -> ::windows::core::Result<HttpChallengeHeaderValueCollection>;
    fn RetryAfter(&mut self) -> ::windows::core::Result<HttpDateOrDeltaHeaderValue>;
    fn SetRetryAfter(&mut self, value: &::core::option::Option<HttpDateOrDeltaHeaderValue>) -> ::windows::core::Result<()>;
    fn TransferEncoding(&mut self) -> ::windows::core::Result<HttpTransferCodingHeaderValueCollection>;
    fn WwwAuthenticate(&mut self) -> ::windows::core::Result<HttpChallengeHeaderValueCollection>;
    fn Append(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryAppendWithoutValidation(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpResponseHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpResponseHeaderCollection";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHttpResponseHeaderCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpResponseHeaderCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpResponseHeaderCollection_Vtbl {
        unsafe extern "system" fn Age<Impl: IHttpResponseHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Age() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAge<Impl: IHttpResponseHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAge(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Allow<Impl: IHttpResponseHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Allow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CacheControl<Impl: IHttpResponseHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CacheControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connection<Impl: IHttpResponseHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Date<Impl: IHttpResponseHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Date() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDate<Impl: IHttpResponseHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDate(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Location<Impl: IHttpResponseHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Impl: IHttpResponseHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProxyAuthenticate<Impl: IHttpResponseHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyAuthenticate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetryAfter<Impl: IHttpResponseHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetryAfter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetryAfter<Impl: IHttpResponseHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRetryAfter(&*(&value as *const <HttpDateOrDeltaHeaderValue as ::windows::core::Abi>::Abi as *const <HttpDateOrDeltaHeaderValue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransferEncoding<Impl: IHttpResponseHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferEncoding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WwwAuthenticate<Impl: IHttpResponseHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WwwAuthenticate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IHttpResponseHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryAppendWithoutValidation<Impl: IHttpResponseHeaderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryAppendWithoutValidation(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpResponseHeaderCollection, BASE_OFFSET>(),
            Age: Age::<Impl, IMPL_OFFSET>,
            SetAge: SetAge::<Impl, IMPL_OFFSET>,
            Allow: Allow::<Impl, IMPL_OFFSET>,
            CacheControl: CacheControl::<Impl, IMPL_OFFSET>,
            Connection: Connection::<Impl, IMPL_OFFSET>,
            Date: Date::<Impl, IMPL_OFFSET>,
            SetDate: SetDate::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            SetLocation: SetLocation::<Impl, IMPL_OFFSET>,
            ProxyAuthenticate: ProxyAuthenticate::<Impl, IMPL_OFFSET>,
            RetryAfter: RetryAfter::<Impl, IMPL_OFFSET>,
            SetRetryAfter: SetRetryAfter::<Impl, IMPL_OFFSET>,
            TransferEncoding: TransferEncoding::<Impl, IMPL_OFFSET>,
            WwwAuthenticate: WwwAuthenticate::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            TryAppendWithoutValidation: TryAppendWithoutValidation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpResponseHeaderCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHttpTransferCodingHeaderValue_Impl: Sized {
    fn Parameters(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHttpTransferCodingHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpTransferCodingHeaderValue";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHttpTransferCodingHeaderValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpTransferCodingHeaderValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpTransferCodingHeaderValue_Vtbl {
        unsafe extern "system" fn Parameters<Impl: IHttpTransferCodingHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IHttpTransferCodingHeaderValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpTransferCodingHeaderValue, BASE_OFFSET>(),
            Parameters: Parameters::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpTransferCodingHeaderValue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpTransferCodingHeaderValueCollection_Impl: Sized {
    fn ParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpTransferCodingHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpTransferCodingHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpTransferCodingHeaderValueCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpTransferCodingHeaderValueCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpTransferCodingHeaderValueCollection_Vtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpTransferCodingHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpTransferCodingHeaderValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpTransferCodingHeaderValueCollection, BASE_OFFSET>(),
            ParseAdd: ParseAdd::<Impl, IMPL_OFFSET>,
            TryParseAdd: TryParseAdd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpTransferCodingHeaderValueCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpTransferCodingHeaderValueFactory_Impl: Sized {
    fn Create(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpTransferCodingHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpTransferCodingHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpTransferCodingHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpTransferCodingHeaderValueFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpTransferCodingHeaderValueFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpTransferCodingHeaderValueFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IHttpTransferCodingHeaderValueFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpTransferCodingHeaderValueFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpTransferCodingHeaderValueFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpTransferCodingHeaderValueStatics_Impl: Sized {
    fn Parse(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpTransferCodingHeaderValue>;
    fn TryParse(&mut self, input: &::windows::core::HSTRING, transfercodingheadervalue: &mut ::core::option::Option<HttpTransferCodingHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpTransferCodingHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpTransferCodingHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpTransferCodingHeaderValueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpTransferCodingHeaderValueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpTransferCodingHeaderValueStatics_Vtbl {
        unsafe extern "system" fn Parse<Impl: IHttpTransferCodingHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParse<Impl: IHttpTransferCodingHeaderValueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, transfercodingheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&transfercodingheadervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpTransferCodingHeaderValueStatics, BASE_OFFSET>(),
            Parse: Parse::<Impl, IMPL_OFFSET>,
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpTransferCodingHeaderValueStatics as ::windows::core::Interface>::IID
    }
}
