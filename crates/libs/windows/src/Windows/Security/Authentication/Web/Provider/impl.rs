#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountClientViewImpl: Sized {
    fn ApplicationCallbackUri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn Type(&self) -> ::windows::core::Result<WebAccountClientViewType>;
    fn AccountPairwiseId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountClientView {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountClientView";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountClientViewVtbl {
    pub const fn new<Impl: IWebAccountClientViewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountClientViewVtbl {
        unsafe extern "system" fn ApplicationCallbackUri<Impl: IWebAccountClientViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplicationCallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IWebAccountClientViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut WebAccountClientViewType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccountPairwiseId<Impl: IWebAccountClientViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccountPairwiseId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountClientView>, base.5, ApplicationCallbackUri::<Impl, OFFSET>, Type::<Impl, OFFSET>, AccountPairwiseId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountClientViewFactoryImpl: Sized {
    fn Create(&self, viewtype: WebAccountClientViewType, applicationcallbackuri: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<WebAccountClientView>;
    fn CreateWithPairwiseId(&self, viewtype: WebAccountClientViewType, applicationcallbackuri: &::core::option::Option<super::super::super::super::Foundation::Uri>, accountpairwiseid: &::windows::core::HSTRING) -> ::windows::core::Result<WebAccountClientView>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountClientViewFactory {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountClientViewFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountClientViewFactoryVtbl {
    pub const fn new<Impl: IWebAccountClientViewFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountClientViewFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IWebAccountClientViewFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewtype: WebAccountClientViewType, applicationcallbackuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(viewtype, &*(&applicationcallbackuri as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithPairwiseId<Impl: IWebAccountClientViewFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewtype: WebAccountClientViewType, applicationcallbackuri: ::windows::core::RawPtr, accountpairwiseid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithPairwiseId(viewtype, &*(&applicationcallbackuri as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&accountpairwiseid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountClientViewFactory>, base.5, Create::<Impl, OFFSET>, CreateWithPairwiseId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountManagerStaticsImpl: Sized {
    fn UpdateWebAccountPropertiesAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, webaccountusername: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn AddWebAccountAsync(&self, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn DeleteWebAccountAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn FindAllProviderWebAccountsAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>>;
    fn PushCookiesAsync(&self, uri: &::core::option::Option<super::super::super::super::Foundation::Uri>, cookies: &::core::option::Option<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Web::Http::HttpCookie>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn SetViewAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, view: &::core::option::Option<WebAccountClientView>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn ClearViewAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, applicationcallbackuri: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn GetViewsAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<WebAccountClientView>>>;
    fn SetWebAccountPictureAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, webaccountpicture: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn ClearWebAccountPictureAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountManagerStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountManagerStaticsVtbl {
    pub const fn new<Impl: IWebAccountManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountManagerStaticsVtbl {
        unsafe extern "system" fn UpdateWebAccountPropertiesAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, additionalproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateWebAccountPropertiesAsync(
                &*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType),
                &*(&webaccountusername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&additionalproperties as *const <super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWebAccountAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddWebAccountAsync(
                &*(&webaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&webaccountusername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&props as *const <super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteWebAccountAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteWebAccountAsync(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllProviderWebAccountsAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAllProviderWebAccountsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushCookiesAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, cookies: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PushCookiesAsync(
                &*(&uri as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&cookies as *const <super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Web::Http::HttpCookie> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Web::Http::HttpCookie> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, view: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetViewAsync(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType), &*(&view as *const <WebAccountClientView as ::windows::core::Abi>::Abi as *const <WebAccountClientView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearViewAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, applicationcallbackuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClearViewAsync(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType), &*(&applicationcallbackuri as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewsAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetViewsAsync(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWebAccountPictureAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, webaccountpicture: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetWebAccountPictureAsync(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType), &*(&webaccountpicture as *const <super::super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearWebAccountPictureAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClearWebAccountPictureAsync(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IWebAccountManagerStatics>,
            base.5,
            UpdateWebAccountPropertiesAsync::<Impl, OFFSET>,
            AddWebAccountAsync::<Impl, OFFSET>,
            DeleteWebAccountAsync::<Impl, OFFSET>,
            FindAllProviderWebAccountsAsync::<Impl, OFFSET>,
            PushCookiesAsync::<Impl, OFFSET>,
            SetViewAsync::<Impl, OFFSET>,
            ClearViewAsync::<Impl, OFFSET>,
            GetViewsAsync::<Impl, OFFSET>,
            SetWebAccountPictureAsync::<Impl, OFFSET>,
            ClearWebAccountPictureAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountManagerStatics2Impl: Sized {
    fn PullCookiesAsync(&self, uristring: &::windows::core::HSTRING, callerpfn: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountManagerStatics2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountManagerStatics2Vtbl {
    pub const fn new<Impl: IWebAccountManagerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountManagerStatics2Vtbl {
        unsafe extern "system" fn PullCookiesAsync<Impl: IWebAccountManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uristring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, callerpfn: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PullCookiesAsync(&*(&uristring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&callerpfn as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountManagerStatics2>, base.5, PullCookiesAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountManagerStatics3Impl: Sized {
    fn FindAllProviderWebAccountsForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>>;
    fn AddWebAccountForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn AddWebAccountWithScopeForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>, scope: WebAccountScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn AddWebAccountWithScopeAndMapForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>, scope: WebAccountScope, peruserwebaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountManagerStatics3 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountManagerStatics3Vtbl {
    pub const fn new<Impl: IWebAccountManagerStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountManagerStatics3Vtbl {
        unsafe extern "system" fn FindAllProviderWebAccountsForUserAsync<Impl: IWebAccountManagerStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAllProviderWebAccountsForUserAsync(&*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWebAccountForUserAsync<Impl: IWebAccountManagerStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddWebAccountForUserAsync(
                &*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType),
                &*(&webaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&webaccountusername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&props as *const <super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWebAccountWithScopeForUserAsync<Impl: IWebAccountManagerStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, scope: WebAccountScope, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddWebAccountWithScopeForUserAsync(
                &*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType),
                &*(&webaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&webaccountusername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&props as *const <super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                scope,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWebAccountWithScopeAndMapForUserAsync<Impl: IWebAccountManagerStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, scope: WebAccountScope, peruserwebaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddWebAccountWithScopeAndMapForUserAsync(
                &*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType),
                &*(&webaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&webaccountusername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&props as *const <super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                scope,
                &*(&peruserwebaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountManagerStatics3>, base.5, FindAllProviderWebAccountsForUserAsync::<Impl, OFFSET>, AddWebAccountForUserAsync::<Impl, OFFSET>, AddWebAccountWithScopeForUserAsync::<Impl, OFFSET>, AddWebAccountWithScopeAndMapForUserAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountManagerStatics4Impl: Sized {
    fn InvalidateAppCacheForAllAccountsAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn InvalidateAppCacheForAccountAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountManagerStatics4 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountManagerStatics4Vtbl {
    pub const fn new<Impl: IWebAccountManagerStatics4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountManagerStatics4Vtbl {
        unsafe extern "system" fn InvalidateAppCacheForAllAccountsAsync<Impl: IWebAccountManagerStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InvalidateAppCacheForAllAccountsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidateAppCacheForAccountAsync<Impl: IWebAccountManagerStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InvalidateAppCacheForAccountAsync(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountManagerStatics4>, base.5, InvalidateAppCacheForAllAccountsAsync::<Impl, OFFSET>, InvalidateAppCacheForAccountAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountMapManagerStaticsImpl: Sized {
    fn AddWebAccountWithScopeAndMapAsync(&self, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>, scope: WebAccountScope, peruserwebaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn SetPerAppToPerUserAccountAsync(&self, perappaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, peruserwebaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn GetPerUserFromPerAppAccountAsync(&self, perappaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn ClearPerUserFromPerAppAccountAsync(&self, perappaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountMapManagerStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountMapManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountMapManagerStaticsVtbl {
    pub const fn new<Impl: IWebAccountMapManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountMapManagerStaticsVtbl {
        unsafe extern "system" fn AddWebAccountWithScopeAndMapAsync<Impl: IWebAccountMapManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, scope: WebAccountScope, peruserwebaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddWebAccountWithScopeAndMapAsync(
                &*(&webaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&webaccountusername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&props as *const <super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                scope,
                &*(&peruserwebaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPerAppToPerUserAccountAsync<Impl: IWebAccountMapManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, perappaccount: ::windows::core::RawPtr, peruserwebaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPerAppToPerUserAccountAsync(&*(&perappaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType), &*(&peruserwebaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPerUserFromPerAppAccountAsync<Impl: IWebAccountMapManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, perappaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPerUserFromPerAppAccountAsync(&*(&perappaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearPerUserFromPerAppAccountAsync<Impl: IWebAccountMapManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, perappaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClearPerUserFromPerAppAccountAsync(&*(&perappaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountMapManagerStatics>, base.5, AddWebAccountWithScopeAndMapAsync::<Impl, OFFSET>, SetPerAppToPerUserAccountAsync::<Impl, OFFSET>, GetPerUserFromPerAppAccountAsync::<Impl, OFFSET>, ClearPerUserFromPerAppAccountAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderAddAccountOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn ReportCompleted(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountProviderAddAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderAddAccountOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountProviderAddAccountOperationVtbl {
    pub const fn new<Impl: IWebAccountProviderAddAccountOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProviderAddAccountOperationVtbl {
        unsafe extern "system" fn ReportCompleted<Impl: IWebAccountProviderAddAccountOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReportCompleted().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProviderAddAccountOperation>, base.5, ReportCompleted::<Impl, OFFSET>)
    }
}
pub trait IWebAccountProviderBaseReportOperationImpl: Sized {
    fn ReportCompleted(&self) -> ::windows::core::Result<()>;
    fn ReportError(&self, value: &::core::option::Option<super::Core::WebProviderError>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWebAccountProviderBaseReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation";
}
impl IWebAccountProviderBaseReportOperationVtbl {
    pub const fn new<Impl: IWebAccountProviderBaseReportOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProviderBaseReportOperationVtbl {
        unsafe extern "system" fn ReportCompleted<Impl: IWebAccountProviderBaseReportOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReportCompleted().into()
        }
        unsafe extern "system" fn ReportError<Impl: IWebAccountProviderBaseReportOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReportError(&*(&value as *const <super::Core::WebProviderError as ::windows::core::Abi>::Abi as *const <super::Core::WebProviderError as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProviderBaseReportOperation>, base.5, ReportCompleted::<Impl, OFFSET>, ReportError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderDeleteAccountOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn WebAccount(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountProviderDeleteAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderDeleteAccountOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountProviderDeleteAccountOperationVtbl {
    pub const fn new<Impl: IWebAccountProviderDeleteAccountOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProviderDeleteAccountOperationVtbl {
        unsafe extern "system" fn WebAccount<Impl: IWebAccountProviderDeleteAccountOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WebAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProviderDeleteAccountOperation>, base.5, WebAccount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderManageAccountOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn WebAccount(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
    fn ReportCompleted(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountProviderManageAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderManageAccountOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountProviderManageAccountOperationVtbl {
    pub const fn new<Impl: IWebAccountProviderManageAccountOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProviderManageAccountOperationVtbl {
        unsafe extern "system" fn WebAccount<Impl: IWebAccountProviderManageAccountOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WebAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompleted<Impl: IWebAccountProviderManageAccountOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReportCompleted().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProviderManageAccountOperation>, base.5, WebAccount::<Impl, OFFSET>, ReportCompleted::<Impl, OFFSET>)
    }
}
pub trait IWebAccountProviderOperationImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<WebAccountProviderOperationKind>;
}
impl ::windows::core::RuntimeName for IWebAccountProviderOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation";
}
impl IWebAccountProviderOperationVtbl {
    pub const fn new<Impl: IWebAccountProviderOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProviderOperationVtbl {
        unsafe extern "system" fn Kind<Impl: IWebAccountProviderOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut WebAccountProviderOperationKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProviderOperation>, base.5, Kind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderRetrieveCookiesOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn Context(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn Cookies(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::super::super::Web::Http::HttpCookie>>;
    fn SetUri(&self, uri: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Uri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn ApplicationCallbackUri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountProviderRetrieveCookiesOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderRetrieveCookiesOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountProviderRetrieveCookiesOperationVtbl {
    pub const fn new<Impl: IWebAccountProviderRetrieveCookiesOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProviderRetrieveCookiesOperationVtbl {
        unsafe extern "system" fn Context<Impl: IWebAccountProviderRetrieveCookiesOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Context() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookies<Impl: IWebAccountProviderRetrieveCookiesOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cookies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUri<Impl: IWebAccountProviderRetrieveCookiesOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&uri as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Uri<Impl: IWebAccountProviderRetrieveCookiesOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationCallbackUri<Impl: IWebAccountProviderRetrieveCookiesOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplicationCallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProviderRetrieveCookiesOperation>, base.5, Context::<Impl, OFFSET>, Cookies::<Impl, OFFSET>, SetUri::<Impl, OFFSET>, Uri::<Impl, OFFSET>, ApplicationCallbackUri::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderSignOutAccountOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn WebAccount(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
    fn ApplicationCallbackUri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn ClientId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountProviderSignOutAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderSignOutAccountOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountProviderSignOutAccountOperationVtbl {
    pub const fn new<Impl: IWebAccountProviderSignOutAccountOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProviderSignOutAccountOperationVtbl {
        unsafe extern "system" fn WebAccount<Impl: IWebAccountProviderSignOutAccountOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WebAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationCallbackUri<Impl: IWebAccountProviderSignOutAccountOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplicationCallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientId<Impl: IWebAccountProviderSignOutAccountOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClientId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProviderSignOutAccountOperation>, base.5, WebAccount::<Impl, OFFSET>, ApplicationCallbackUri::<Impl, OFFSET>, ClientId::<Impl, OFFSET>)
    }
}
pub trait IWebAccountProviderSilentReportOperationImpl: Sized + IWebAccountProviderBaseReportOperationImpl {
    fn ReportUserInteractionRequired(&self) -> ::windows::core::Result<()>;
    fn ReportUserInteractionRequiredWithError(&self, value: &::core::option::Option<super::Core::WebProviderError>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWebAccountProviderSilentReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderSilentReportOperation";
}
impl IWebAccountProviderSilentReportOperationVtbl {
    pub const fn new<Impl: IWebAccountProviderSilentReportOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProviderSilentReportOperationVtbl {
        unsafe extern "system" fn ReportUserInteractionRequired<Impl: IWebAccountProviderSilentReportOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReportUserInteractionRequired().into()
        }
        unsafe extern "system" fn ReportUserInteractionRequiredWithError<Impl: IWebAccountProviderSilentReportOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReportUserInteractionRequiredWithError(&*(&value as *const <super::Core::WebProviderError as ::windows::core::Abi>::Abi as *const <super::Core::WebProviderError as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProviderSilentReportOperation>, base.5, ReportUserInteractionRequired::<Impl, OFFSET>, ReportUserInteractionRequiredWithError::<Impl, OFFSET>)
    }
}
pub trait IWebAccountProviderTokenObjectsImpl: Sized {
    fn Operation(&self) -> ::windows::core::Result<IWebAccountProviderOperation>;
}
impl ::windows::core::RuntimeName for IWebAccountProviderTokenObjects {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects";
}
impl IWebAccountProviderTokenObjectsVtbl {
    pub const fn new<Impl: IWebAccountProviderTokenObjectsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProviderTokenObjectsVtbl {
        unsafe extern "system" fn Operation<Impl: IWebAccountProviderTokenObjectsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Operation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProviderTokenObjects>, base.5, Operation::<Impl, OFFSET>)
    }
}
pub trait IWebAccountProviderTokenObjects2Impl: Sized + IWebAccountProviderTokenObjectsImpl {
    fn User(&self) -> ::windows::core::Result<super::super::super::super::System::User>;
}
impl ::windows::core::RuntimeName for IWebAccountProviderTokenObjects2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects2";
}
impl IWebAccountProviderTokenObjects2Vtbl {
    pub const fn new<Impl: IWebAccountProviderTokenObjects2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProviderTokenObjects2Vtbl {
        unsafe extern "system" fn User<Impl: IWebAccountProviderTokenObjects2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProviderTokenObjects2>, base.5, User::<Impl, OFFSET>)
    }
}
pub trait IWebAccountProviderTokenOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn ProviderRequest(&self) -> ::windows::core::Result<WebProviderTokenRequest>;
    fn ProviderResponses(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>>;
    fn SetCacheExpirationTime(&self, value: &super::super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn CacheExpirationTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime>;
}
impl ::windows::core::RuntimeName for IWebAccountProviderTokenOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenOperation";
}
impl IWebAccountProviderTokenOperationVtbl {
    pub const fn new<Impl: IWebAccountProviderTokenOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProviderTokenOperationVtbl {
        unsafe extern "system" fn ProviderRequest<Impl: IWebAccountProviderTokenOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProviderRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderResponses<Impl: IWebAccountProviderTokenOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProviderResponses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCacheExpirationTime<Impl: IWebAccountProviderTokenOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCacheExpirationTime(&*(&value as *const <super::super::super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CacheExpirationTime<Impl: IWebAccountProviderTokenOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CacheExpirationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProviderTokenOperation>, base.5, ProviderRequest::<Impl, OFFSET>, ProviderResponses::<Impl, OFFSET>, SetCacheExpirationTime::<Impl, OFFSET>, CacheExpirationTime::<Impl, OFFSET>)
    }
}
pub trait IWebAccountProviderUIReportOperationImpl: Sized + IWebAccountProviderBaseReportOperationImpl {
    fn ReportUserCanceled(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWebAccountProviderUIReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderUIReportOperation";
}
impl IWebAccountProviderUIReportOperationVtbl {
    pub const fn new<Impl: IWebAccountProviderUIReportOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProviderUIReportOperationVtbl {
        unsafe extern "system" fn ReportUserCanceled<Impl: IWebAccountProviderUIReportOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReportUserCanceled().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProviderUIReportOperation>, base.5, ReportUserCanceled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountScopeManagerStaticsImpl: Sized {
    fn AddWebAccountWithScopeAsync(&self, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>, scope: WebAccountScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn SetScopeAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, scope: WebAccountScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn GetScope(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<WebAccountScope>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountScopeManagerStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountScopeManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountScopeManagerStaticsVtbl {
    pub const fn new<Impl: IWebAccountScopeManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountScopeManagerStaticsVtbl {
        unsafe extern "system" fn AddWebAccountWithScopeAsync<Impl: IWebAccountScopeManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, scope: WebAccountScope, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddWebAccountWithScopeAsync(
                &*(&webaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&webaccountusername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&props as *const <super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                scope,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScopeAsync<Impl: IWebAccountScopeManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, scope: WebAccountScope, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetScopeAsync(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType), scope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScope<Impl: IWebAccountScopeManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut WebAccountScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetScope(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountScopeManagerStatics>, base.5, AddWebAccountWithScopeAsync::<Impl, OFFSET>, SetScopeAsync::<Impl, OFFSET>, GetScope::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebProviderTokenRequestImpl: Sized {
    fn ClientRequest(&self) -> ::windows::core::Result<super::Core::WebTokenRequest>;
    fn WebAccounts(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>;
    fn WebAccountSelectionOptions(&self) -> ::windows::core::Result<WebAccountSelectionOptions>;
    fn ApplicationCallbackUri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn GetApplicationTokenBindingKeyAsync(&self, keytype: super::TokenBindingKeyType, target: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Cryptography::Core::CryptographicKey>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebProviderTokenRequest {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IWebProviderTokenRequestVtbl {
    pub const fn new<Impl: IWebProviderTokenRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebProviderTokenRequestVtbl {
        unsafe extern "system" fn ClientRequest<Impl: IWebProviderTokenRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClientRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebAccounts<Impl: IWebProviderTokenRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WebAccounts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebAccountSelectionOptions<Impl: IWebProviderTokenRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut WebAccountSelectionOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WebAccountSelectionOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationCallbackUri<Impl: IWebProviderTokenRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplicationCallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationTokenBindingKeyAsync<Impl: IWebProviderTokenRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, keytype: super::TokenBindingKeyType, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetApplicationTokenBindingKeyAsync(keytype, &*(&target as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebProviderTokenRequest>, base.5, ClientRequest::<Impl, OFFSET>, WebAccounts::<Impl, OFFSET>, WebAccountSelectionOptions::<Impl, OFFSET>, ApplicationCallbackUri::<Impl, OFFSET>, GetApplicationTokenBindingKeyAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebProviderTokenRequest2Impl: Sized {
    fn GetApplicationTokenBindingKeyIdAsync(&self, keytype: super::TokenBindingKeyType, target: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebProviderTokenRequest2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest2";
}
#[cfg(feature = "implement_exclusive")]
impl IWebProviderTokenRequest2Vtbl {
    pub const fn new<Impl: IWebProviderTokenRequest2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebProviderTokenRequest2Vtbl {
        unsafe extern "system" fn GetApplicationTokenBindingKeyIdAsync<Impl: IWebProviderTokenRequest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, keytype: super::TokenBindingKeyType, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetApplicationTokenBindingKeyIdAsync(keytype, &*(&target as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebProviderTokenRequest2>, base.5, GetApplicationTokenBindingKeyIdAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebProviderTokenRequest3Impl: Sized {
    fn ApplicationPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ApplicationProcessName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CheckApplicationForCapabilityAsync(&self, capabilityname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebProviderTokenRequest3 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest3";
}
#[cfg(feature = "implement_exclusive")]
impl IWebProviderTokenRequest3Vtbl {
    pub const fn new<Impl: IWebProviderTokenRequest3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebProviderTokenRequest3Vtbl {
        unsafe extern "system" fn ApplicationPackageFamilyName<Impl: IWebProviderTokenRequest3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplicationPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationProcessName<Impl: IWebProviderTokenRequest3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplicationProcessName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckApplicationForCapabilityAsync<Impl: IWebProviderTokenRequest3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, capabilityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckApplicationForCapabilityAsync(&*(&capabilityname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebProviderTokenRequest3>, base.5, ApplicationPackageFamilyName::<Impl, OFFSET>, ApplicationProcessName::<Impl, OFFSET>, CheckApplicationForCapabilityAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebProviderTokenResponseImpl: Sized {
    fn ClientResponse(&self) -> ::windows::core::Result<super::Core::WebTokenResponse>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebProviderTokenResponse {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponse";
}
#[cfg(feature = "implement_exclusive")]
impl IWebProviderTokenResponseVtbl {
    pub const fn new<Impl: IWebProviderTokenResponseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebProviderTokenResponseVtbl {
        unsafe extern "system" fn ClientResponse<Impl: IWebProviderTokenResponseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClientResponse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebProviderTokenResponse>, base.5, ClientResponse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebProviderTokenResponseFactoryImpl: Sized {
    fn Create(&self, webtokenresponse: &::core::option::Option<super::Core::WebTokenResponse>) -> ::windows::core::Result<WebProviderTokenResponse>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebProviderTokenResponseFactory {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWebProviderTokenResponseFactoryVtbl {
    pub const fn new<Impl: IWebProviderTokenResponseFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebProviderTokenResponseFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IWebProviderTokenResponseFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webtokenresponse: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&webtokenresponse as *const <super::Core::WebTokenResponse as ::windows::core::Abi>::Abi as *const <super::Core::WebTokenResponse as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebProviderTokenResponseFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
