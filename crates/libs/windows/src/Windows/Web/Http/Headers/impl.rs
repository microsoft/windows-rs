#[cfg(feature = "implement_exclusive")]
pub trait IHttpCacheDirectiveHeaderValueCollectionImpl: Sized {
    fn MaxAge(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetMaxAge(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn MaxStale(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetMaxStale(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn MinFresh(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetMinFresh(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn SharedMaxAge(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetSharedMaxAge(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpCacheDirectiveHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpCacheDirectiveHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpCacheDirectiveHeaderValueCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpCacheDirectiveHeaderValueCollectionImpl, const OFFSET: isize>() -> IHttpCacheDirectiveHeaderValueCollectionVtbl {
        unsafe extern "system" fn MaxAge<Impl: IHttpCacheDirectiveHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxAge<Impl: IHttpCacheDirectiveHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxAge(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxStale<Impl: IHttpCacheDirectiveHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxStale<Impl: IHttpCacheDirectiveHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxStale(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MinFresh<Impl: IHttpCacheDirectiveHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMinFresh<Impl: IHttpCacheDirectiveHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinFresh(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SharedMaxAge<Impl: IHttpCacheDirectiveHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSharedMaxAge<Impl: IHttpCacheDirectiveHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSharedMaxAge(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ParseAdd<Impl: IHttpCacheDirectiveHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpCacheDirectiveHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHttpCacheDirectiveHeaderValueCollection>,
            ::windows::core::GetTrustLevel,
            MaxAge::<Impl, OFFSET>,
            SetMaxAge::<Impl, OFFSET>,
            MaxStale::<Impl, OFFSET>,
            SetMaxStale::<Impl, OFFSET>,
            MinFresh::<Impl, OFFSET>,
            SetMinFresh::<Impl, OFFSET>,
            SharedMaxAge::<Impl, OFFSET>,
            SetSharedMaxAge::<Impl, OFFSET>,
            ParseAdd::<Impl, OFFSET>,
            TryParseAdd::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpChallengeHeaderValueImpl: Sized {
    fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
    fn Scheme(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpChallengeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpChallengeHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpChallengeHeaderValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpChallengeHeaderValueImpl, const OFFSET: isize>() -> IHttpChallengeHeaderValueVtbl {
        unsafe extern "system" fn Parameters<Impl: IHttpChallengeHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Scheme<Impl: IHttpChallengeHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Token<Impl: IHttpChallengeHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpChallengeHeaderValue>, ::windows::core::GetTrustLevel, Parameters::<Impl, OFFSET>, Scheme::<Impl, OFFSET>, Token::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpChallengeHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpChallengeHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpChallengeHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpChallengeHeaderValueCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpChallengeHeaderValueCollectionImpl, const OFFSET: isize>() -> IHttpChallengeHeaderValueCollectionVtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpChallengeHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpChallengeHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpChallengeHeaderValueCollection>, ::windows::core::GetTrustLevel, ParseAdd::<Impl, OFFSET>, TryParseAdd::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpChallengeHeaderValueFactoryImpl: Sized {
    fn CreateFromScheme(&self, scheme: &::windows::core::HSTRING) -> ::windows::core::Result<HttpChallengeHeaderValue>;
    fn CreateFromSchemeWithToken(&self, scheme: &::windows::core::HSTRING, token: &::windows::core::HSTRING) -> ::windows::core::Result<HttpChallengeHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpChallengeHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpChallengeHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpChallengeHeaderValueFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpChallengeHeaderValueFactoryImpl, const OFFSET: isize>() -> IHttpChallengeHeaderValueFactoryVtbl {
        unsafe extern "system" fn CreateFromScheme<Impl: IHttpChallengeHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromSchemeWithToken<Impl: IHttpChallengeHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpChallengeHeaderValueFactory>, ::windows::core::GetTrustLevel, CreateFromScheme::<Impl, OFFSET>, CreateFromSchemeWithToken::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpChallengeHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpChallengeHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, challengeheadervalue: &mut ::core::option::Option<HttpChallengeHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpChallengeHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpChallengeHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpChallengeHeaderValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpChallengeHeaderValueStaticsImpl, const OFFSET: isize>() -> IHttpChallengeHeaderValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IHttpChallengeHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IHttpChallengeHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, challengeheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpChallengeHeaderValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, OFFSET>, TryParse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpConnectionOptionHeaderValueImpl: Sized {
    fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpConnectionOptionHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpConnectionOptionHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpConnectionOptionHeaderValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpConnectionOptionHeaderValueImpl, const OFFSET: isize>() -> IHttpConnectionOptionHeaderValueVtbl {
        unsafe extern "system" fn Token<Impl: IHttpConnectionOptionHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpConnectionOptionHeaderValue>, ::windows::core::GetTrustLevel, Token::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpConnectionOptionHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpConnectionOptionHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpConnectionOptionHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpConnectionOptionHeaderValueCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpConnectionOptionHeaderValueCollectionImpl, const OFFSET: isize>() -> IHttpConnectionOptionHeaderValueCollectionVtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpConnectionOptionHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpConnectionOptionHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpConnectionOptionHeaderValueCollection>, ::windows::core::GetTrustLevel, ParseAdd::<Impl, OFFSET>, TryParseAdd::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpConnectionOptionHeaderValueFactoryImpl: Sized {
    fn Create(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<HttpConnectionOptionHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpConnectionOptionHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpConnectionOptionHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpConnectionOptionHeaderValueFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpConnectionOptionHeaderValueFactoryImpl, const OFFSET: isize>() -> IHttpConnectionOptionHeaderValueFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IHttpConnectionOptionHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpConnectionOptionHeaderValueFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpConnectionOptionHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpConnectionOptionHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, connectionoptionheadervalue: &mut ::core::option::Option<HttpConnectionOptionHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpConnectionOptionHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpConnectionOptionHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpConnectionOptionHeaderValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpConnectionOptionHeaderValueStaticsImpl, const OFFSET: isize>() -> IHttpConnectionOptionHeaderValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IHttpConnectionOptionHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IHttpConnectionOptionHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, connectionoptionheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpConnectionOptionHeaderValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, OFFSET>, TryParse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingHeaderValueImpl: Sized {
    fn ContentCoding(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentCodingHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentCodingHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentCodingHeaderValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentCodingHeaderValueImpl, const OFFSET: isize>() -> IHttpContentCodingHeaderValueVtbl {
        unsafe extern "system" fn ContentCoding<Impl: IHttpContentCodingHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpContentCodingHeaderValue>, ::windows::core::GetTrustLevel, ContentCoding::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentCodingHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentCodingHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentCodingHeaderValueCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentCodingHeaderValueCollectionImpl, const OFFSET: isize>() -> IHttpContentCodingHeaderValueCollectionVtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpContentCodingHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpContentCodingHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpContentCodingHeaderValueCollection>, ::windows::core::GetTrustLevel, ParseAdd::<Impl, OFFSET>, TryParseAdd::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingHeaderValueFactoryImpl: Sized {
    fn Create(&self, contentcoding: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentCodingHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentCodingHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentCodingHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentCodingHeaderValueFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentCodingHeaderValueFactoryImpl, const OFFSET: isize>() -> IHttpContentCodingHeaderValueFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IHttpContentCodingHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentcoding: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpContentCodingHeaderValueFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentCodingHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, contentcodingheadervalue: &mut ::core::option::Option<HttpContentCodingHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentCodingHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentCodingHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentCodingHeaderValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentCodingHeaderValueStaticsImpl, const OFFSET: isize>() -> IHttpContentCodingHeaderValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IHttpContentCodingHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IHttpContentCodingHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contentcodingheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpContentCodingHeaderValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, OFFSET>, TryParse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingWithQualityHeaderValueImpl: Sized {
    fn ContentCoding(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Quality(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentCodingWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentCodingWithQualityHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentCodingWithQualityHeaderValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentCodingWithQualityHeaderValueImpl, const OFFSET: isize>() -> IHttpContentCodingWithQualityHeaderValueVtbl {
        unsafe extern "system" fn ContentCoding<Impl: IHttpContentCodingWithQualityHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Quality<Impl: IHttpContentCodingWithQualityHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpContentCodingWithQualityHeaderValue>, ::windows::core::GetTrustLevel, ContentCoding::<Impl, OFFSET>, Quality::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingWithQualityHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentCodingWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentCodingWithQualityHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentCodingWithQualityHeaderValueCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentCodingWithQualityHeaderValueCollectionImpl, const OFFSET: isize>() -> IHttpContentCodingWithQualityHeaderValueCollectionVtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpContentCodingWithQualityHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpContentCodingWithQualityHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpContentCodingWithQualityHeaderValueCollection>, ::windows::core::GetTrustLevel, ParseAdd::<Impl, OFFSET>, TryParseAdd::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingWithQualityHeaderValueFactoryImpl: Sized {
    fn CreateFromValue(&self, contentcoding: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValue>;
    fn CreateFromValueWithQuality(&self, contentcoding: &::windows::core::HSTRING, quality: f64) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentCodingWithQualityHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentCodingWithQualityHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentCodingWithQualityHeaderValueFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentCodingWithQualityHeaderValueFactoryImpl, const OFFSET: isize>() -> IHttpContentCodingWithQualityHeaderValueFactoryVtbl {
        unsafe extern "system" fn CreateFromValue<Impl: IHttpContentCodingWithQualityHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentcoding: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromValueWithQuality<Impl: IHttpContentCodingWithQualityHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentcoding: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, quality: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpContentCodingWithQualityHeaderValueFactory>, ::windows::core::GetTrustLevel, CreateFromValue::<Impl, OFFSET>, CreateFromValueWithQuality::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingWithQualityHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, contentcodingwithqualityheadervalue: &mut ::core::option::Option<HttpContentCodingWithQualityHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentCodingWithQualityHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentCodingWithQualityHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentCodingWithQualityHeaderValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentCodingWithQualityHeaderValueStaticsImpl, const OFFSET: isize>() -> IHttpContentCodingWithQualityHeaderValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IHttpContentCodingWithQualityHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IHttpContentCodingWithQualityHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contentcodingwithqualityheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpContentCodingWithQualityHeaderValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, OFFSET>, TryParse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentDispositionHeaderValueImpl: Sized {
    fn DispositionType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDispositionType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FileNameStar(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFileNameStar(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
    fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn SetSize(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u64>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentDispositionHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentDispositionHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentDispositionHeaderValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentDispositionHeaderValueImpl, const OFFSET: isize>() -> IHttpContentDispositionHeaderValueVtbl {
        unsafe extern "system" fn DispositionType<Impl: IHttpContentDispositionHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDispositionType<Impl: IHttpContentDispositionHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDispositionType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FileName<Impl: IHttpContentDispositionHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFileName<Impl: IHttpContentDispositionHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FileNameStar<Impl: IHttpContentDispositionHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFileNameStar<Impl: IHttpContentDispositionHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileNameStar(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Name<Impl: IHttpContentDispositionHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetName<Impl: IHttpContentDispositionHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Parameters<Impl: IHttpContentDispositionHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Size<Impl: IHttpContentDispositionHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSize<Impl: IHttpContentDispositionHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(&*(&value as *const <super::super::super::Foundation::IReference<u64> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHttpContentDispositionHeaderValue>,
            ::windows::core::GetTrustLevel,
            DispositionType::<Impl, OFFSET>,
            SetDispositionType::<Impl, OFFSET>,
            FileName::<Impl, OFFSET>,
            SetFileName::<Impl, OFFSET>,
            FileNameStar::<Impl, OFFSET>,
            SetFileNameStar::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            Parameters::<Impl, OFFSET>,
            Size::<Impl, OFFSET>,
            SetSize::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentDispositionHeaderValueFactoryImpl: Sized {
    fn Create(&self, dispositiontype: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentDispositionHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentDispositionHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentDispositionHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentDispositionHeaderValueFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentDispositionHeaderValueFactoryImpl, const OFFSET: isize>() -> IHttpContentDispositionHeaderValueFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IHttpContentDispositionHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispositiontype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpContentDispositionHeaderValueFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentDispositionHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentDispositionHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, contentdispositionheadervalue: &mut ::core::option::Option<HttpContentDispositionHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentDispositionHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentDispositionHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentDispositionHeaderValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentDispositionHeaderValueStaticsImpl, const OFFSET: isize>() -> IHttpContentDispositionHeaderValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IHttpContentDispositionHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IHttpContentDispositionHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contentdispositionheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpContentDispositionHeaderValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, OFFSET>, TryParse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentHeaderCollectionImpl: Sized {
    fn ContentDisposition(&self) -> ::windows::core::Result<HttpContentDispositionHeaderValue>;
    fn SetContentDisposition(&self, value: &::core::option::Option<HttpContentDispositionHeaderValue>) -> ::windows::core::Result<()>;
    fn ContentEncoding(&self) -> ::windows::core::Result<HttpContentCodingHeaderValueCollection>;
    fn ContentLanguage(&self) -> ::windows::core::Result<HttpLanguageHeaderValueCollection>;
    fn ContentLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn SetContentLength(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u64>>) -> ::windows::core::Result<()>;
    fn ContentLocation(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetContentLocation(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ContentMD5(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetContentMD5(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn ContentRange(&self) -> ::windows::core::Result<HttpContentRangeHeaderValue>;
    fn SetContentRange(&self, value: &::core::option::Option<HttpContentRangeHeaderValue>) -> ::windows::core::Result<()>;
    fn ContentType(&self) -> ::windows::core::Result<HttpMediaTypeHeaderValue>;
    fn SetContentType(&self, value: &::core::option::Option<HttpMediaTypeHeaderValue>) -> ::windows::core::Result<()>;
    fn Expires(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetExpires(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn LastModified(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetLastModified(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Append(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryAppendWithoutValidation(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentHeaderCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentHeaderCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>() -> IHttpContentHeaderCollectionVtbl {
        unsafe extern "system" fn ContentDisposition<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentDisposition<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentDisposition(&*(&value as *const <HttpContentDispositionHeaderValue as ::windows::core::Abi>::Abi as *const <HttpContentDispositionHeaderValue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentEncoding<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentLanguage<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentLength<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentLength<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentLength(&*(&value as *const <super::super::super::Foundation::IReference<u64> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentLocation<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentLocation<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentLocation(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentMD5<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentMD5<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentMD5(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentRange<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentRange<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentRange(&*(&value as *const <HttpContentRangeHeaderValue as ::windows::core::Abi>::Abi as *const <HttpContentRangeHeaderValue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentType<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentType<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentType(&*(&value as *const <HttpMediaTypeHeaderValue as ::windows::core::Abi>::Abi as *const <HttpMediaTypeHeaderValue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Expires<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExpires<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpires(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LastModified<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLastModified<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastModified(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Append<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryAppendWithoutValidation<Impl: IHttpContentHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHttpContentHeaderCollection>,
            ::windows::core::GetTrustLevel,
            ContentDisposition::<Impl, OFFSET>,
            SetContentDisposition::<Impl, OFFSET>,
            ContentEncoding::<Impl, OFFSET>,
            ContentLanguage::<Impl, OFFSET>,
            ContentLength::<Impl, OFFSET>,
            SetContentLength::<Impl, OFFSET>,
            ContentLocation::<Impl, OFFSET>,
            SetContentLocation::<Impl, OFFSET>,
            ContentMD5::<Impl, OFFSET>,
            SetContentMD5::<Impl, OFFSET>,
            ContentRange::<Impl, OFFSET>,
            SetContentRange::<Impl, OFFSET>,
            ContentType::<Impl, OFFSET>,
            SetContentType::<Impl, OFFSET>,
            Expires::<Impl, OFFSET>,
            SetExpires::<Impl, OFFSET>,
            LastModified::<Impl, OFFSET>,
            SetLastModified::<Impl, OFFSET>,
            Append::<Impl, OFFSET>,
            TryAppendWithoutValidation::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentRangeHeaderValueImpl: Sized {
    fn FirstBytePosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn LastBytePosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn Length(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn Unit(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUnit(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentRangeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentRangeHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentRangeHeaderValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentRangeHeaderValueImpl, const OFFSET: isize>() -> IHttpContentRangeHeaderValueVtbl {
        unsafe extern "system" fn FirstBytePosition<Impl: IHttpContentRangeHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LastBytePosition<Impl: IHttpContentRangeHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Length<Impl: IHttpContentRangeHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Unit<Impl: IHttpContentRangeHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetUnit<Impl: IHttpContentRangeHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnit(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpContentRangeHeaderValue>, ::windows::core::GetTrustLevel, FirstBytePosition::<Impl, OFFSET>, LastBytePosition::<Impl, OFFSET>, Length::<Impl, OFFSET>, Unit::<Impl, OFFSET>, SetUnit::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentRangeHeaderValueFactoryImpl: Sized {
    fn CreateFromLength(&self, length: u64) -> ::windows::core::Result<HttpContentRangeHeaderValue>;
    fn CreateFromRange(&self, from: u64, to: u64) -> ::windows::core::Result<HttpContentRangeHeaderValue>;
    fn CreateFromRangeWithLength(&self, from: u64, to: u64, length: u64) -> ::windows::core::Result<HttpContentRangeHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentRangeHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentRangeHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentRangeHeaderValueFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentRangeHeaderValueFactoryImpl, const OFFSET: isize>() -> IHttpContentRangeHeaderValueFactoryVtbl {
        unsafe extern "system" fn CreateFromLength<Impl: IHttpContentRangeHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromRange<Impl: IHttpContentRangeHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, from: u64, to: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromRangeWithLength<Impl: IHttpContentRangeHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, from: u64, to: u64, length: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpContentRangeHeaderValueFactory>, ::windows::core::GetTrustLevel, CreateFromLength::<Impl, OFFSET>, CreateFromRange::<Impl, OFFSET>, CreateFromRangeWithLength::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentRangeHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentRangeHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, contentrangeheadervalue: &mut ::core::option::Option<HttpContentRangeHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpContentRangeHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpContentRangeHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpContentRangeHeaderValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContentRangeHeaderValueStaticsImpl, const OFFSET: isize>() -> IHttpContentRangeHeaderValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IHttpContentRangeHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IHttpContentRangeHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contentrangeheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpContentRangeHeaderValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, OFFSET>, TryParse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCookiePairHeaderValueImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpCookiePairHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpCookiePairHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpCookiePairHeaderValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpCookiePairHeaderValueImpl, const OFFSET: isize>() -> IHttpCookiePairHeaderValueVtbl {
        unsafe extern "system" fn Name<Impl: IHttpCookiePairHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IHttpCookiePairHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: IHttpCookiePairHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpCookiePairHeaderValue>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCookiePairHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpCookiePairHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpCookiePairHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpCookiePairHeaderValueCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpCookiePairHeaderValueCollectionImpl, const OFFSET: isize>() -> IHttpCookiePairHeaderValueCollectionVtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpCookiePairHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpCookiePairHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpCookiePairHeaderValueCollection>, ::windows::core::GetTrustLevel, ParseAdd::<Impl, OFFSET>, TryParseAdd::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCookiePairHeaderValueFactoryImpl: Sized {
    fn CreateFromName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCookiePairHeaderValue>;
    fn CreateFromNameWithValue(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCookiePairHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpCookiePairHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpCookiePairHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpCookiePairHeaderValueFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpCookiePairHeaderValueFactoryImpl, const OFFSET: isize>() -> IHttpCookiePairHeaderValueFactoryVtbl {
        unsafe extern "system" fn CreateFromName<Impl: IHttpCookiePairHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromNameWithValue<Impl: IHttpCookiePairHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpCookiePairHeaderValueFactory>, ::windows::core::GetTrustLevel, CreateFromName::<Impl, OFFSET>, CreateFromNameWithValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCookiePairHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCookiePairHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, cookiepairheadervalue: &mut ::core::option::Option<HttpCookiePairHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpCookiePairHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpCookiePairHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpCookiePairHeaderValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpCookiePairHeaderValueStaticsImpl, const OFFSET: isize>() -> IHttpCookiePairHeaderValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IHttpCookiePairHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IHttpCookiePairHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, cookiepairheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpCookiePairHeaderValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, OFFSET>, TryParse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCredentialsHeaderValueImpl: Sized {
    fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
    fn Scheme(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpCredentialsHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpCredentialsHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpCredentialsHeaderValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpCredentialsHeaderValueImpl, const OFFSET: isize>() -> IHttpCredentialsHeaderValueVtbl {
        unsafe extern "system" fn Parameters<Impl: IHttpCredentialsHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Scheme<Impl: IHttpCredentialsHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Token<Impl: IHttpCredentialsHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpCredentialsHeaderValue>, ::windows::core::GetTrustLevel, Parameters::<Impl, OFFSET>, Scheme::<Impl, OFFSET>, Token::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCredentialsHeaderValueFactoryImpl: Sized {
    fn CreateFromScheme(&self, scheme: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCredentialsHeaderValue>;
    fn CreateFromSchemeWithToken(&self, scheme: &::windows::core::HSTRING, token: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCredentialsHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpCredentialsHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpCredentialsHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpCredentialsHeaderValueFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpCredentialsHeaderValueFactoryImpl, const OFFSET: isize>() -> IHttpCredentialsHeaderValueFactoryVtbl {
        unsafe extern "system" fn CreateFromScheme<Impl: IHttpCredentialsHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromSchemeWithToken<Impl: IHttpCredentialsHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpCredentialsHeaderValueFactory>, ::windows::core::GetTrustLevel, CreateFromScheme::<Impl, OFFSET>, CreateFromSchemeWithToken::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCredentialsHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCredentialsHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, credentialsheadervalue: &mut ::core::option::Option<HttpCredentialsHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpCredentialsHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpCredentialsHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpCredentialsHeaderValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpCredentialsHeaderValueStaticsImpl, const OFFSET: isize>() -> IHttpCredentialsHeaderValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IHttpCredentialsHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IHttpCredentialsHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, credentialsheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpCredentialsHeaderValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, OFFSET>, TryParse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpDateOrDeltaHeaderValueImpl: Sized {
    fn Date(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn Delta(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpDateOrDeltaHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpDateOrDeltaHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpDateOrDeltaHeaderValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpDateOrDeltaHeaderValueImpl, const OFFSET: isize>() -> IHttpDateOrDeltaHeaderValueVtbl {
        unsafe extern "system" fn Date<Impl: IHttpDateOrDeltaHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Delta<Impl: IHttpDateOrDeltaHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpDateOrDeltaHeaderValue>, ::windows::core::GetTrustLevel, Date::<Impl, OFFSET>, Delta::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpDateOrDeltaHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpDateOrDeltaHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, dateordeltaheadervalue: &mut ::core::option::Option<HttpDateOrDeltaHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpDateOrDeltaHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpDateOrDeltaHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpDateOrDeltaHeaderValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpDateOrDeltaHeaderValueStaticsImpl, const OFFSET: isize>() -> IHttpDateOrDeltaHeaderValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IHttpDateOrDeltaHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IHttpDateOrDeltaHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, dateordeltaheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpDateOrDeltaHeaderValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, OFFSET>, TryParse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpExpectationHeaderValueImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpExpectationHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpExpectationHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpExpectationHeaderValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpExpectationHeaderValueImpl, const OFFSET: isize>() -> IHttpExpectationHeaderValueVtbl {
        unsafe extern "system" fn Name<Impl: IHttpExpectationHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IHttpExpectationHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: IHttpExpectationHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Parameters<Impl: IHttpExpectationHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpExpectationHeaderValue>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, Parameters::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpExpectationHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpExpectationHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpExpectationHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpExpectationHeaderValueCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpExpectationHeaderValueCollectionImpl, const OFFSET: isize>() -> IHttpExpectationHeaderValueCollectionVtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpExpectationHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpExpectationHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpExpectationHeaderValueCollection>, ::windows::core::GetTrustLevel, ParseAdd::<Impl, OFFSET>, TryParseAdd::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpExpectationHeaderValueFactoryImpl: Sized {
    fn CreateFromName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<HttpExpectationHeaderValue>;
    fn CreateFromNameWithValue(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<HttpExpectationHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpExpectationHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpExpectationHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpExpectationHeaderValueFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpExpectationHeaderValueFactoryImpl, const OFFSET: isize>() -> IHttpExpectationHeaderValueFactoryVtbl {
        unsafe extern "system" fn CreateFromName<Impl: IHttpExpectationHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromNameWithValue<Impl: IHttpExpectationHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpExpectationHeaderValueFactory>, ::windows::core::GetTrustLevel, CreateFromName::<Impl, OFFSET>, CreateFromNameWithValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpExpectationHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpExpectationHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, expectationheadervalue: &mut ::core::option::Option<HttpExpectationHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpExpectationHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpExpectationHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpExpectationHeaderValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpExpectationHeaderValueStaticsImpl, const OFFSET: isize>() -> IHttpExpectationHeaderValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IHttpExpectationHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IHttpExpectationHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, expectationheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpExpectationHeaderValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, OFFSET>, TryParse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpLanguageHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpLanguageHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpLanguageHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpLanguageHeaderValueCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpLanguageHeaderValueCollectionImpl, const OFFSET: isize>() -> IHttpLanguageHeaderValueCollectionVtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpLanguageHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpLanguageHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpLanguageHeaderValueCollection>, ::windows::core::GetTrustLevel, ParseAdd::<Impl, OFFSET>, TryParseAdd::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpLanguageRangeWithQualityHeaderValueImpl: Sized {
    fn LanguageRange(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Quality(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpLanguageRangeWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpLanguageRangeWithQualityHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpLanguageRangeWithQualityHeaderValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpLanguageRangeWithQualityHeaderValueImpl, const OFFSET: isize>() -> IHttpLanguageRangeWithQualityHeaderValueVtbl {
        unsafe extern "system" fn LanguageRange<Impl: IHttpLanguageRangeWithQualityHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Quality<Impl: IHttpLanguageRangeWithQualityHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpLanguageRangeWithQualityHeaderValue>, ::windows::core::GetTrustLevel, LanguageRange::<Impl, OFFSET>, Quality::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpLanguageRangeWithQualityHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpLanguageRangeWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpLanguageRangeWithQualityHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpLanguageRangeWithQualityHeaderValueCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpLanguageRangeWithQualityHeaderValueCollectionImpl, const OFFSET: isize>() -> IHttpLanguageRangeWithQualityHeaderValueCollectionVtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpLanguageRangeWithQualityHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpLanguageRangeWithQualityHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpLanguageRangeWithQualityHeaderValueCollection>, ::windows::core::GetTrustLevel, ParseAdd::<Impl, OFFSET>, TryParseAdd::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpLanguageRangeWithQualityHeaderValueFactoryImpl: Sized {
    fn CreateFromLanguageRange(&self, languagerange: &::windows::core::HSTRING) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValue>;
    fn CreateFromLanguageRangeWithQuality(&self, languagerange: &::windows::core::HSTRING, quality: f64) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpLanguageRangeWithQualityHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpLanguageRangeWithQualityHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpLanguageRangeWithQualityHeaderValueFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpLanguageRangeWithQualityHeaderValueFactoryImpl, const OFFSET: isize>() -> IHttpLanguageRangeWithQualityHeaderValueFactoryVtbl {
        unsafe extern "system" fn CreateFromLanguageRange<Impl: IHttpLanguageRangeWithQualityHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagerange: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromLanguageRangeWithQuality<Impl: IHttpLanguageRangeWithQualityHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagerange: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, quality: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpLanguageRangeWithQualityHeaderValueFactory>, ::windows::core::GetTrustLevel, CreateFromLanguageRange::<Impl, OFFSET>, CreateFromLanguageRangeWithQuality::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpLanguageRangeWithQualityHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, languagerangewithqualityheadervalue: &mut ::core::option::Option<HttpLanguageRangeWithQualityHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpLanguageRangeWithQualityHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpLanguageRangeWithQualityHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpLanguageRangeWithQualityHeaderValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpLanguageRangeWithQualityHeaderValueStaticsImpl, const OFFSET: isize>() -> IHttpLanguageRangeWithQualityHeaderValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IHttpLanguageRangeWithQualityHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IHttpLanguageRangeWithQualityHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, languagerangewithqualityheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpLanguageRangeWithQualityHeaderValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, OFFSET>, TryParse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeHeaderValueImpl: Sized {
    fn CharSet(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCharSet(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MediaType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMediaType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpMediaTypeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpMediaTypeHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpMediaTypeHeaderValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMediaTypeHeaderValueImpl, const OFFSET: isize>() -> IHttpMediaTypeHeaderValueVtbl {
        unsafe extern "system" fn CharSet<Impl: IHttpMediaTypeHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCharSet<Impl: IHttpMediaTypeHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCharSet(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaType<Impl: IHttpMediaTypeHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMediaType<Impl: IHttpMediaTypeHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Parameters<Impl: IHttpMediaTypeHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpMediaTypeHeaderValue>, ::windows::core::GetTrustLevel, CharSet::<Impl, OFFSET>, SetCharSet::<Impl, OFFSET>, MediaType::<Impl, OFFSET>, SetMediaType::<Impl, OFFSET>, Parameters::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeHeaderValueFactoryImpl: Sized {
    fn Create(&self, mediatype: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMediaTypeHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpMediaTypeHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpMediaTypeHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpMediaTypeHeaderValueFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMediaTypeHeaderValueFactoryImpl, const OFFSET: isize>() -> IHttpMediaTypeHeaderValueFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IHttpMediaTypeHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpMediaTypeHeaderValueFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMediaTypeHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, mediatypeheadervalue: &mut ::core::option::Option<HttpMediaTypeHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpMediaTypeHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpMediaTypeHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpMediaTypeHeaderValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMediaTypeHeaderValueStaticsImpl, const OFFSET: isize>() -> IHttpMediaTypeHeaderValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IHttpMediaTypeHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IHttpMediaTypeHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mediatypeheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpMediaTypeHeaderValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, OFFSET>, TryParse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeWithQualityHeaderValueImpl: Sized {
    fn CharSet(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCharSet(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MediaType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMediaType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
    fn Quality(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
    fn SetQuality(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpMediaTypeWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpMediaTypeWithQualityHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpMediaTypeWithQualityHeaderValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMediaTypeWithQualityHeaderValueImpl, const OFFSET: isize>() -> IHttpMediaTypeWithQualityHeaderValueVtbl {
        unsafe extern "system" fn CharSet<Impl: IHttpMediaTypeWithQualityHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCharSet<Impl: IHttpMediaTypeWithQualityHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCharSet(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaType<Impl: IHttpMediaTypeWithQualityHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMediaType<Impl: IHttpMediaTypeWithQualityHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Parameters<Impl: IHttpMediaTypeWithQualityHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Quality<Impl: IHttpMediaTypeWithQualityHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetQuality<Impl: IHttpMediaTypeWithQualityHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuality(&*(&value as *const <super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpMediaTypeWithQualityHeaderValue>, ::windows::core::GetTrustLevel, CharSet::<Impl, OFFSET>, SetCharSet::<Impl, OFFSET>, MediaType::<Impl, OFFSET>, SetMediaType::<Impl, OFFSET>, Parameters::<Impl, OFFSET>, Quality::<Impl, OFFSET>, SetQuality::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeWithQualityHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpMediaTypeWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpMediaTypeWithQualityHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpMediaTypeWithQualityHeaderValueCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMediaTypeWithQualityHeaderValueCollectionImpl, const OFFSET: isize>() -> IHttpMediaTypeWithQualityHeaderValueCollectionVtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpMediaTypeWithQualityHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpMediaTypeWithQualityHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpMediaTypeWithQualityHeaderValueCollection>, ::windows::core::GetTrustLevel, ParseAdd::<Impl, OFFSET>, TryParseAdd::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeWithQualityHeaderValueFactoryImpl: Sized {
    fn CreateFromMediaType(&self, mediatype: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValue>;
    fn CreateFromMediaTypeWithQuality(&self, mediatype: &::windows::core::HSTRING, quality: f64) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpMediaTypeWithQualityHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpMediaTypeWithQualityHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpMediaTypeWithQualityHeaderValueFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMediaTypeWithQualityHeaderValueFactoryImpl, const OFFSET: isize>() -> IHttpMediaTypeWithQualityHeaderValueFactoryVtbl {
        unsafe extern "system" fn CreateFromMediaType<Impl: IHttpMediaTypeWithQualityHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromMediaTypeWithQuality<Impl: IHttpMediaTypeWithQualityHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, quality: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpMediaTypeWithQualityHeaderValueFactory>, ::windows::core::GetTrustLevel, CreateFromMediaType::<Impl, OFFSET>, CreateFromMediaTypeWithQuality::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeWithQualityHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, mediatypewithqualityheadervalue: &mut ::core::option::Option<HttpMediaTypeWithQualityHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpMediaTypeWithQualityHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpMediaTypeWithQualityHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpMediaTypeWithQualityHeaderValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMediaTypeWithQualityHeaderValueStaticsImpl, const OFFSET: isize>() -> IHttpMediaTypeWithQualityHeaderValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IHttpMediaTypeWithQualityHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IHttpMediaTypeWithQualityHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mediatypewithqualityheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpMediaTypeWithQualityHeaderValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, OFFSET>, TryParse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMethodHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpMethodHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpMethodHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpMethodHeaderValueCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpMethodHeaderValueCollectionImpl, const OFFSET: isize>() -> IHttpMethodHeaderValueCollectionVtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpMethodHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpMethodHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpMethodHeaderValueCollection>, ::windows::core::GetTrustLevel, ParseAdd::<Impl, OFFSET>, TryParseAdd::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpNameValueHeaderValueImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpNameValueHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpNameValueHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpNameValueHeaderValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpNameValueHeaderValueImpl, const OFFSET: isize>() -> IHttpNameValueHeaderValueVtbl {
        unsafe extern "system" fn Name<Impl: IHttpNameValueHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IHttpNameValueHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: IHttpNameValueHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpNameValueHeaderValue>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpNameValueHeaderValueFactoryImpl: Sized {
    fn CreateFromName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<HttpNameValueHeaderValue>;
    fn CreateFromNameWithValue(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<HttpNameValueHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpNameValueHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpNameValueHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpNameValueHeaderValueFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpNameValueHeaderValueFactoryImpl, const OFFSET: isize>() -> IHttpNameValueHeaderValueFactoryVtbl {
        unsafe extern "system" fn CreateFromName<Impl: IHttpNameValueHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromNameWithValue<Impl: IHttpNameValueHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpNameValueHeaderValueFactory>, ::windows::core::GetTrustLevel, CreateFromName::<Impl, OFFSET>, CreateFromNameWithValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpNameValueHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpNameValueHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, namevalueheadervalue: &mut ::core::option::Option<HttpNameValueHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpNameValueHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpNameValueHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpNameValueHeaderValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpNameValueHeaderValueStaticsImpl, const OFFSET: isize>() -> IHttpNameValueHeaderValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IHttpNameValueHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IHttpNameValueHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, namevalueheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpNameValueHeaderValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, OFFSET>, TryParse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductHeaderValueImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Version(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpProductHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpProductHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpProductHeaderValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpProductHeaderValueImpl, const OFFSET: isize>() -> IHttpProductHeaderValueVtbl {
        unsafe extern "system" fn Name<Impl: IHttpProductHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Version<Impl: IHttpProductHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpProductHeaderValue>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, Version::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductHeaderValueFactoryImpl: Sized {
    fn CreateFromName(&self, productname: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductHeaderValue>;
    fn CreateFromNameWithVersion(&self, productname: &::windows::core::HSTRING, productversion: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpProductHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpProductHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpProductHeaderValueFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpProductHeaderValueFactoryImpl, const OFFSET: isize>() -> IHttpProductHeaderValueFactoryVtbl {
        unsafe extern "system" fn CreateFromName<Impl: IHttpProductHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromNameWithVersion<Impl: IHttpProductHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, productversion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpProductHeaderValueFactory>, ::windows::core::GetTrustLevel, CreateFromName::<Impl, OFFSET>, CreateFromNameWithVersion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, productheadervalue: &mut ::core::option::Option<HttpProductHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpProductHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpProductHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpProductHeaderValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpProductHeaderValueStaticsImpl, const OFFSET: isize>() -> IHttpProductHeaderValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IHttpProductHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IHttpProductHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, productheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpProductHeaderValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, OFFSET>, TryParse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductInfoHeaderValueImpl: Sized {
    fn Product(&self) -> ::windows::core::Result<HttpProductHeaderValue>;
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpProductInfoHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpProductInfoHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpProductInfoHeaderValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpProductInfoHeaderValueImpl, const OFFSET: isize>() -> IHttpProductInfoHeaderValueVtbl {
        unsafe extern "system" fn Product<Impl: IHttpProductInfoHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Comment<Impl: IHttpProductInfoHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpProductInfoHeaderValue>, ::windows::core::GetTrustLevel, Product::<Impl, OFFSET>, Comment::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductInfoHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpProductInfoHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpProductInfoHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpProductInfoHeaderValueCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpProductInfoHeaderValueCollectionImpl, const OFFSET: isize>() -> IHttpProductInfoHeaderValueCollectionVtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpProductInfoHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpProductInfoHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpProductInfoHeaderValueCollection>, ::windows::core::GetTrustLevel, ParseAdd::<Impl, OFFSET>, TryParseAdd::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductInfoHeaderValueFactoryImpl: Sized {
    fn CreateFromComment(&self, productcomment: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductInfoHeaderValue>;
    fn CreateFromNameWithVersion(&self, productname: &::windows::core::HSTRING, productversion: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductInfoHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpProductInfoHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpProductInfoHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpProductInfoHeaderValueFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpProductInfoHeaderValueFactoryImpl, const OFFSET: isize>() -> IHttpProductInfoHeaderValueFactoryVtbl {
        unsafe extern "system" fn CreateFromComment<Impl: IHttpProductInfoHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productcomment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromNameWithVersion<Impl: IHttpProductInfoHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, productversion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpProductInfoHeaderValueFactory>, ::windows::core::GetTrustLevel, CreateFromComment::<Impl, OFFSET>, CreateFromNameWithVersion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductInfoHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductInfoHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, productinfoheadervalue: &mut ::core::option::Option<HttpProductInfoHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpProductInfoHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpProductInfoHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpProductInfoHeaderValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpProductInfoHeaderValueStaticsImpl, const OFFSET: isize>() -> IHttpProductInfoHeaderValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IHttpProductInfoHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IHttpProductInfoHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, productinfoheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpProductInfoHeaderValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, OFFSET>, TryParse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpRequestHeaderCollectionImpl: Sized {
    fn Accept(&self) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValueCollection>;
    fn AcceptEncoding(&self) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValueCollection>;
    fn AcceptLanguage(&self) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValueCollection>;
    fn Authorization(&self) -> ::windows::core::Result<HttpCredentialsHeaderValue>;
    fn SetAuthorization(&self, value: &::core::option::Option<HttpCredentialsHeaderValue>) -> ::windows::core::Result<()>;
    fn CacheControl(&self) -> ::windows::core::Result<HttpCacheDirectiveHeaderValueCollection>;
    fn Connection(&self) -> ::windows::core::Result<HttpConnectionOptionHeaderValueCollection>;
    fn Cookie(&self) -> ::windows::core::Result<HttpCookiePairHeaderValueCollection>;
    fn Date(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetDate(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Expect(&self) -> ::windows::core::Result<HttpExpectationHeaderValueCollection>;
    fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFrom(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Host(&self) -> ::windows::core::Result<super::super::super::Networking::HostName>;
    fn SetHost(&self, value: &::core::option::Option<super::super::super::Networking::HostName>) -> ::windows::core::Result<()>;
    fn IfModifiedSince(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetIfModifiedSince(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn IfUnmodifiedSince(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetIfUnmodifiedSince(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn MaxForwards(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetMaxForwards(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn ProxyAuthorization(&self) -> ::windows::core::Result<HttpCredentialsHeaderValue>;
    fn SetProxyAuthorization(&self, value: &::core::option::Option<HttpCredentialsHeaderValue>) -> ::windows::core::Result<()>;
    fn Referer(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetReferer(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn TransferEncoding(&self) -> ::windows::core::Result<HttpTransferCodingHeaderValueCollection>;
    fn UserAgent(&self) -> ::windows::core::Result<HttpProductInfoHeaderValueCollection>;
    fn Append(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryAppendWithoutValidation(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpRequestHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpRequestHeaderCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpRequestHeaderCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>() -> IHttpRequestHeaderCollectionVtbl {
        unsafe extern "system" fn Accept<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AcceptEncoding<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AcceptLanguage<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Authorization<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAuthorization<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthorization(&*(&value as *const <HttpCredentialsHeaderValue as ::windows::core::Abi>::Abi as *const <HttpCredentialsHeaderValue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CacheControl<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Connection<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Cookie<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Date<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDate<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDate(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Expect<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn From<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFrom<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrom(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Host<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHost<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHost(&*(&value as *const <super::super::super::Networking::HostName as ::windows::core::Abi>::Abi as *const <super::super::super::Networking::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IfModifiedSince<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIfModifiedSince<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIfModifiedSince(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IfUnmodifiedSince<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIfUnmodifiedSince<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIfUnmodifiedSince(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxForwards<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxForwards<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxForwards(&*(&value as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProxyAuthorization<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetProxyAuthorization<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxyAuthorization(&*(&value as *const <HttpCredentialsHeaderValue as ::windows::core::Abi>::Abi as *const <HttpCredentialsHeaderValue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Referer<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReferer<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReferer(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransferEncoding<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UserAgent<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Append<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryAppendWithoutValidation<Impl: IHttpRequestHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHttpRequestHeaderCollection>,
            ::windows::core::GetTrustLevel,
            Accept::<Impl, OFFSET>,
            AcceptEncoding::<Impl, OFFSET>,
            AcceptLanguage::<Impl, OFFSET>,
            Authorization::<Impl, OFFSET>,
            SetAuthorization::<Impl, OFFSET>,
            CacheControl::<Impl, OFFSET>,
            Connection::<Impl, OFFSET>,
            Cookie::<Impl, OFFSET>,
            Date::<Impl, OFFSET>,
            SetDate::<Impl, OFFSET>,
            Expect::<Impl, OFFSET>,
            From::<Impl, OFFSET>,
            SetFrom::<Impl, OFFSET>,
            Host::<Impl, OFFSET>,
            SetHost::<Impl, OFFSET>,
            IfModifiedSince::<Impl, OFFSET>,
            SetIfModifiedSince::<Impl, OFFSET>,
            IfUnmodifiedSince::<Impl, OFFSET>,
            SetIfUnmodifiedSince::<Impl, OFFSET>,
            MaxForwards::<Impl, OFFSET>,
            SetMaxForwards::<Impl, OFFSET>,
            ProxyAuthorization::<Impl, OFFSET>,
            SetProxyAuthorization::<Impl, OFFSET>,
            Referer::<Impl, OFFSET>,
            SetReferer::<Impl, OFFSET>,
            TransferEncoding::<Impl, OFFSET>,
            UserAgent::<Impl, OFFSET>,
            Append::<Impl, OFFSET>,
            TryAppendWithoutValidation::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpResponseHeaderCollectionImpl: Sized {
    fn Age(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetAge(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn Allow(&self) -> ::windows::core::Result<HttpMethodHeaderValueCollection>;
    fn CacheControl(&self) -> ::windows::core::Result<HttpCacheDirectiveHeaderValueCollection>;
    fn Connection(&self) -> ::windows::core::Result<HttpConnectionOptionHeaderValueCollection>;
    fn Date(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetDate(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetLocation(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ProxyAuthenticate(&self) -> ::windows::core::Result<HttpChallengeHeaderValueCollection>;
    fn RetryAfter(&self) -> ::windows::core::Result<HttpDateOrDeltaHeaderValue>;
    fn SetRetryAfter(&self, value: &::core::option::Option<HttpDateOrDeltaHeaderValue>) -> ::windows::core::Result<()>;
    fn TransferEncoding(&self) -> ::windows::core::Result<HttpTransferCodingHeaderValueCollection>;
    fn WwwAuthenticate(&self) -> ::windows::core::Result<HttpChallengeHeaderValueCollection>;
    fn Append(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryAppendWithoutValidation(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpResponseHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpResponseHeaderCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpResponseHeaderCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpResponseHeaderCollectionImpl, const OFFSET: isize>() -> IHttpResponseHeaderCollectionVtbl {
        unsafe extern "system" fn Age<Impl: IHttpResponseHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAge<Impl: IHttpResponseHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAge(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Allow<Impl: IHttpResponseHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CacheControl<Impl: IHttpResponseHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Connection<Impl: IHttpResponseHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Date<Impl: IHttpResponseHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDate<Impl: IHttpResponseHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDate(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Location<Impl: IHttpResponseHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLocation<Impl: IHttpResponseHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProxyAuthenticate<Impl: IHttpResponseHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RetryAfter<Impl: IHttpResponseHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRetryAfter<Impl: IHttpResponseHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRetryAfter(&*(&value as *const <HttpDateOrDeltaHeaderValue as ::windows::core::Abi>::Abi as *const <HttpDateOrDeltaHeaderValue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransferEncoding<Impl: IHttpResponseHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WwwAuthenticate<Impl: IHttpResponseHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Append<Impl: IHttpResponseHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryAppendWithoutValidation<Impl: IHttpResponseHeaderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHttpResponseHeaderCollection>,
            ::windows::core::GetTrustLevel,
            Age::<Impl, OFFSET>,
            SetAge::<Impl, OFFSET>,
            Allow::<Impl, OFFSET>,
            CacheControl::<Impl, OFFSET>,
            Connection::<Impl, OFFSET>,
            Date::<Impl, OFFSET>,
            SetDate::<Impl, OFFSET>,
            Location::<Impl, OFFSET>,
            SetLocation::<Impl, OFFSET>,
            ProxyAuthenticate::<Impl, OFFSET>,
            RetryAfter::<Impl, OFFSET>,
            SetRetryAfter::<Impl, OFFSET>,
            TransferEncoding::<Impl, OFFSET>,
            WwwAuthenticate::<Impl, OFFSET>,
            Append::<Impl, OFFSET>,
            TryAppendWithoutValidation::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpTransferCodingHeaderValueImpl: Sized {
    fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpTransferCodingHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpTransferCodingHeaderValue";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpTransferCodingHeaderValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpTransferCodingHeaderValueImpl, const OFFSET: isize>() -> IHttpTransferCodingHeaderValueVtbl {
        unsafe extern "system" fn Parameters<Impl: IHttpTransferCodingHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IHttpTransferCodingHeaderValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpTransferCodingHeaderValue>, ::windows::core::GetTrustLevel, Parameters::<Impl, OFFSET>, Value::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpTransferCodingHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpTransferCodingHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpTransferCodingHeaderValueCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpTransferCodingHeaderValueCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpTransferCodingHeaderValueCollectionImpl, const OFFSET: isize>() -> IHttpTransferCodingHeaderValueCollectionVtbl {
        unsafe extern "system" fn ParseAdd<Impl: IHttpTransferCodingHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseAdd(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryParseAdd<Impl: IHttpTransferCodingHeaderValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpTransferCodingHeaderValueCollection>, ::windows::core::GetTrustLevel, ParseAdd::<Impl, OFFSET>, TryParseAdd::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpTransferCodingHeaderValueFactoryImpl: Sized {
    fn Create(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpTransferCodingHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpTransferCodingHeaderValueFactory {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpTransferCodingHeaderValueFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpTransferCodingHeaderValueFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpTransferCodingHeaderValueFactoryImpl, const OFFSET: isize>() -> IHttpTransferCodingHeaderValueFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IHttpTransferCodingHeaderValueFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpTransferCodingHeaderValueFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpTransferCodingHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpTransferCodingHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, transfercodingheadervalue: &mut ::core::option::Option<HttpTransferCodingHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHttpTransferCodingHeaderValueStatics {
    const NAME: &'static str = "Windows.Web.Http.Headers.IHttpTransferCodingHeaderValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHttpTransferCodingHeaderValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpTransferCodingHeaderValueStaticsImpl, const OFFSET: isize>() -> IHttpTransferCodingHeaderValueStaticsVtbl {
        unsafe extern "system" fn Parse<Impl: IHttpTransferCodingHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParse<Impl: IHttpTransferCodingHeaderValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, transfercodingheadervalue: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpTransferCodingHeaderValueStatics>, ::windows::core::GetTrustLevel, Parse::<Impl, OFFSET>, TryParse::<Impl, OFFSET>)
    }
}
