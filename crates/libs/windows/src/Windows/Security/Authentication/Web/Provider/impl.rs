#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebAccountClientViewImpl: Sized {
    fn ApplicationCallbackUri(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn Type(&mut self) -> ::windows::core::Result<WebAccountClientViewType>;
    fn AccountPairwiseId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountClientView {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountClientView";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebAccountClientViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountClientViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountClientViewVtbl {
        unsafe extern "system" fn ApplicationCallbackUri<Impl: IWebAccountClientViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationCallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IWebAccountClientViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebAccountClientViewType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccountPairwiseId<Impl: IWebAccountClientViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountPairwiseId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountClientView, BASE_OFFSET>(),
            ApplicationCallbackUri: ApplicationCallbackUri::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            AccountPairwiseId: AccountPairwiseId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountClientView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebAccountClientViewFactoryImpl: Sized {
    fn Create(&mut self, viewtype: WebAccountClientViewType, applicationcallbackuri: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<WebAccountClientView>;
    fn CreateWithPairwiseId(&mut self, viewtype: WebAccountClientViewType, applicationcallbackuri: &::core::option::Option<super::super::super::super::Foundation::Uri>, accountpairwiseid: &::windows::core::HSTRING) -> ::windows::core::Result<WebAccountClientView>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountClientViewFactory {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountClientViewFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebAccountClientViewFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountClientViewFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountClientViewFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IWebAccountClientViewFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewtype: WebAccountClientViewType, applicationcallbackuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(viewtype, &*(&applicationcallbackuri as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithPairwiseId<Impl: IWebAccountClientViewFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewtype: WebAccountClientViewType, applicationcallbackuri: ::windows::core::RawPtr, accountpairwiseid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithPairwiseId(viewtype, &*(&applicationcallbackuri as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&accountpairwiseid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountClientViewFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithPairwiseId: CreateWithPairwiseId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountClientViewFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Storage_Streams", feature = "Web_Http", feature = "implement_exclusive"))]
pub trait IWebAccountManagerStaticsImpl: Sized {
    fn UpdateWebAccountPropertiesAsync(&mut self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, webaccountusername: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn AddWebAccountAsync(&mut self, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn DeleteWebAccountAsync(&mut self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn FindAllProviderWebAccountsAsync(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>>;
    fn PushCookiesAsync(&mut self, uri: &::core::option::Option<super::super::super::super::Foundation::Uri>, cookies: &::core::option::Option<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Web::Http::HttpCookie>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn SetViewAsync(&mut self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, view: &::core::option::Option<WebAccountClientView>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn ClearViewAsync(&mut self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, applicationcallbackuri: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn GetViewsAsync(&mut self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<WebAccountClientView>>>;
    fn SetWebAccountPictureAsync(&mut self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, webaccountpicture: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn ClearWebAccountPictureAsync(&mut self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Storage_Streams", feature = "Web_Http", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountManagerStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Storage_Streams", feature = "Web_Http", feature = "implement_exclusive"))]
impl IWebAccountManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountManagerStaticsVtbl {
        unsafe extern "system" fn UpdateWebAccountPropertiesAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, additionalproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn AddWebAccountAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn DeleteWebAccountAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteWebAccountAsync(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllProviderWebAccountsAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllProviderWebAccountsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushCookiesAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, cookies: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn SetViewAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, view: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetViewAsync(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType), &*(&view as *const <WebAccountClientView as ::windows::core::Abi>::Abi as *const <WebAccountClientView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearViewAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, applicationcallbackuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearViewAsync(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType), &*(&applicationcallbackuri as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewsAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewsAsync(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWebAccountPictureAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, webaccountpicture: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWebAccountPictureAsync(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType), &*(&webaccountpicture as *const <super::super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearWebAccountPictureAsync<Impl: IWebAccountManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearWebAccountPictureAsync(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountManagerStatics, BASE_OFFSET>(),
            UpdateWebAccountPropertiesAsync: UpdateWebAccountPropertiesAsync::<Impl, IMPL_OFFSET>,
            AddWebAccountAsync: AddWebAccountAsync::<Impl, IMPL_OFFSET>,
            DeleteWebAccountAsync: DeleteWebAccountAsync::<Impl, IMPL_OFFSET>,
            FindAllProviderWebAccountsAsync: FindAllProviderWebAccountsAsync::<Impl, IMPL_OFFSET>,
            PushCookiesAsync: PushCookiesAsync::<Impl, IMPL_OFFSET>,
            SetViewAsync: SetViewAsync::<Impl, IMPL_OFFSET>,
            ClearViewAsync: ClearViewAsync::<Impl, IMPL_OFFSET>,
            GetViewsAsync: GetViewsAsync::<Impl, IMPL_OFFSET>,
            SetWebAccountPictureAsync: SetWebAccountPictureAsync::<Impl, IMPL_OFFSET>,
            ClearWebAccountPictureAsync: ClearWebAccountPictureAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebAccountManagerStatics2Impl: Sized {
    fn PullCookiesAsync(&mut self, uristring: &::windows::core::HSTRING, callerpfn: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountManagerStatics2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebAccountManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountManagerStatics2Vtbl {
        unsafe extern "system" fn PullCookiesAsync<Impl: IWebAccountManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uristring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, callerpfn: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PullCookiesAsync(&*(&uristring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&callerpfn as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountManagerStatics2, BASE_OFFSET>(),
            PullCookiesAsync: PullCookiesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System", feature = "implement_exclusive"))]
pub trait IWebAccountManagerStatics3Impl: Sized {
    fn FindAllProviderWebAccountsForUserAsync(&mut self, user: &::core::option::Option<super::super::super::super::System::User>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>>;
    fn AddWebAccountForUserAsync(&mut self, user: &::core::option::Option<super::super::super::super::System::User>, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn AddWebAccountWithScopeForUserAsync(&mut self, user: &::core::option::Option<super::super::super::super::System::User>, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>, scope: WebAccountScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn AddWebAccountWithScopeAndMapForUserAsync(&mut self, user: &::core::option::Option<super::super::super::super::System::User>, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>, scope: WebAccountScope, peruserwebaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountManagerStatics3 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System", feature = "implement_exclusive"))]
impl IWebAccountManagerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountManagerStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountManagerStatics3Vtbl {
        unsafe extern "system" fn FindAllProviderWebAccountsForUserAsync<Impl: IWebAccountManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllProviderWebAccountsForUserAsync(&*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWebAccountForUserAsync<Impl: IWebAccountManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn AddWebAccountWithScopeForUserAsync<Impl: IWebAccountManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, scope: WebAccountScope, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn AddWebAccountWithScopeAndMapForUserAsync<Impl: IWebAccountManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, scope: WebAccountScope, peruserwebaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountManagerStatics3, BASE_OFFSET>(),
            FindAllProviderWebAccountsForUserAsync: FindAllProviderWebAccountsForUserAsync::<Impl, IMPL_OFFSET>,
            AddWebAccountForUserAsync: AddWebAccountForUserAsync::<Impl, IMPL_OFFSET>,
            AddWebAccountWithScopeForUserAsync: AddWebAccountWithScopeForUserAsync::<Impl, IMPL_OFFSET>,
            AddWebAccountWithScopeAndMapForUserAsync: AddWebAccountWithScopeAndMapForUserAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountManagerStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWebAccountManagerStatics4Impl: Sized {
    fn InvalidateAppCacheForAllAccountsAsync(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn InvalidateAppCacheForAccountAsync(&mut self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountManagerStatics4 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics4";
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAccountManagerStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountManagerStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountManagerStatics4Vtbl {
        unsafe extern "system" fn InvalidateAppCacheForAllAccountsAsync<Impl: IWebAccountManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidateAppCacheForAllAccountsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidateAppCacheForAccountAsync<Impl: IWebAccountManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidateAppCacheForAccountAsync(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountManagerStatics4, BASE_OFFSET>(),
            InvalidateAppCacheForAllAccountsAsync: InvalidateAppCacheForAllAccountsAsync::<Impl, IMPL_OFFSET>,
            InvalidateAppCacheForAccountAsync: InvalidateAppCacheForAccountAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountManagerStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWebAccountMapManagerStaticsImpl: Sized {
    fn AddWebAccountWithScopeAndMapAsync(&mut self, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>, scope: WebAccountScope, peruserwebaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn SetPerAppToPerUserAccountAsync(&mut self, perappaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, peruserwebaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn GetPerUserFromPerAppAccountAsync(&mut self, perappaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn ClearPerUserFromPerAppAccountAsync(&mut self, perappaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountMapManagerStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountMapManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAccountMapManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountMapManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountMapManagerStaticsVtbl {
        unsafe extern "system" fn AddWebAccountWithScopeAndMapAsync<Impl: IWebAccountMapManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, scope: WebAccountScope, peruserwebaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn SetPerAppToPerUserAccountAsync<Impl: IWebAccountMapManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perappaccount: ::windows::core::RawPtr, peruserwebaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPerAppToPerUserAccountAsync(&*(&perappaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType), &*(&peruserwebaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPerUserFromPerAppAccountAsync<Impl: IWebAccountMapManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perappaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPerUserFromPerAppAccountAsync(&*(&perappaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearPerUserFromPerAppAccountAsync<Impl: IWebAccountMapManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perappaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearPerUserFromPerAppAccountAsync(&*(&perappaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountMapManagerStatics, BASE_OFFSET>(),
            AddWebAccountWithScopeAndMapAsync: AddWebAccountWithScopeAndMapAsync::<Impl, IMPL_OFFSET>,
            SetPerAppToPerUserAccountAsync: SetPerAppToPerUserAccountAsync::<Impl, IMPL_OFFSET>,
            GetPerUserFromPerAppAccountAsync: GetPerUserFromPerAppAccountAsync::<Impl, IMPL_OFFSET>,
            ClearPerUserFromPerAppAccountAsync: ClearPerUserFromPerAppAccountAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountMapManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderAddAccountOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn ReportCompleted(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountProviderAddAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderAddAccountOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountProviderAddAccountOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderAddAccountOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderAddAccountOperationVtbl {
        unsafe extern "system" fn ReportCompleted<Impl: IWebAccountProviderAddAccountOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCompleted().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderAddAccountOperation, BASE_OFFSET>(),
            ReportCompleted: ReportCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderAddAccountOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Security_Authentication_Web_Core")]
pub trait IWebAccountProviderBaseReportOperationImpl: Sized {
    fn ReportCompleted(&mut self) -> ::windows::core::Result<()>;
    fn ReportError(&mut self, value: &::core::option::Option<super::Core::WebProviderError>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl ::windows::core::RuntimeName for IWebAccountProviderBaseReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation";
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl IWebAccountProviderBaseReportOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderBaseReportOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderBaseReportOperationVtbl {
        unsafe extern "system" fn ReportCompleted<Impl: IWebAccountProviderBaseReportOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCompleted().into()
        }
        unsafe extern "system" fn ReportError<Impl: IWebAccountProviderBaseReportOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportError(&*(&value as *const <super::Core::WebProviderError as ::windows::core::Abi>::Abi as *const <super::Core::WebProviderError as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderBaseReportOperation, BASE_OFFSET>(),
            ReportCompleted: ReportCompleted::<Impl, IMPL_OFFSET>,
            ReportError: ReportError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderBaseReportOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWebAccountProviderDeleteAccountOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn WebAccount(&mut self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountProviderDeleteAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderDeleteAccountOperation";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAccountProviderDeleteAccountOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderDeleteAccountOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderDeleteAccountOperationVtbl {
        unsafe extern "system" fn WebAccount<Impl: IWebAccountProviderDeleteAccountOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderDeleteAccountOperation, BASE_OFFSET>(),
            WebAccount: WebAccount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderDeleteAccountOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWebAccountProviderManageAccountOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn WebAccount(&mut self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
    fn ReportCompleted(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountProviderManageAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderManageAccountOperation";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAccountProviderManageAccountOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderManageAccountOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderManageAccountOperationVtbl {
        unsafe extern "system" fn WebAccount<Impl: IWebAccountProviderManageAccountOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompleted<Impl: IWebAccountProviderManageAccountOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCompleted().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderManageAccountOperation, BASE_OFFSET>(),
            WebAccount: WebAccount::<Impl, IMPL_OFFSET>,
            ReportCompleted: ReportCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderManageAccountOperation as ::windows::core::Interface>::IID
    }
}
pub trait IWebAccountProviderOperationImpl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<WebAccountProviderOperationKind>;
}
impl ::windows::core::RuntimeName for IWebAccountProviderOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation";
}
impl IWebAccountProviderOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderOperationVtbl {
        unsafe extern "system" fn Kind<Impl: IWebAccountProviderOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebAccountProviderOperationKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderOperation, BASE_OFFSET>(), Kind: Kind::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Web_Http", feature = "implement_exclusive"))]
pub trait IWebAccountProviderRetrieveCookiesOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn Context(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn Cookies(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::super::super::Web::Http::HttpCookie>>;
    fn SetUri(&mut self, uri: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Uri(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn ApplicationCallbackUri(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Web_Http", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountProviderRetrieveCookiesOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderRetrieveCookiesOperation";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Web_Http", feature = "implement_exclusive"))]
impl IWebAccountProviderRetrieveCookiesOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderRetrieveCookiesOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderRetrieveCookiesOperationVtbl {
        unsafe extern "system" fn Context<Impl: IWebAccountProviderRetrieveCookiesOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Context() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookies<Impl: IWebAccountProviderRetrieveCookiesOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cookies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUri<Impl: IWebAccountProviderRetrieveCookiesOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&uri as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Uri<Impl: IWebAccountProviderRetrieveCookiesOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationCallbackUri<Impl: IWebAccountProviderRetrieveCookiesOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationCallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderRetrieveCookiesOperation, BASE_OFFSET>(),
            Context: Context::<Impl, IMPL_OFFSET>,
            Cookies: Cookies::<Impl, IMPL_OFFSET>,
            SetUri: SetUri::<Impl, IMPL_OFFSET>,
            Uri: Uri::<Impl, IMPL_OFFSET>,
            ApplicationCallbackUri: ApplicationCallbackUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderRetrieveCookiesOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWebAccountProviderSignOutAccountOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn WebAccount(&mut self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
    fn ApplicationCallbackUri(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn ClientId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountProviderSignOutAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderSignOutAccountOperation";
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAccountProviderSignOutAccountOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderSignOutAccountOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderSignOutAccountOperationVtbl {
        unsafe extern "system" fn WebAccount<Impl: IWebAccountProviderSignOutAccountOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationCallbackUri<Impl: IWebAccountProviderSignOutAccountOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationCallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientId<Impl: IWebAccountProviderSignOutAccountOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderSignOutAccountOperation, BASE_OFFSET>(),
            WebAccount: WebAccount::<Impl, IMPL_OFFSET>,
            ApplicationCallbackUri: ApplicationCallbackUri::<Impl, IMPL_OFFSET>,
            ClientId: ClientId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderSignOutAccountOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Security_Authentication_Web_Core")]
pub trait IWebAccountProviderSilentReportOperationImpl: Sized + IWebAccountProviderBaseReportOperationImpl {
    fn ReportUserInteractionRequired(&mut self) -> ::windows::core::Result<()>;
    fn ReportUserInteractionRequiredWithError(&mut self, value: &::core::option::Option<super::Core::WebProviderError>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl ::windows::core::RuntimeName for IWebAccountProviderSilentReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderSilentReportOperation";
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl IWebAccountProviderSilentReportOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderSilentReportOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderSilentReportOperationVtbl {
        unsafe extern "system" fn ReportUserInteractionRequired<Impl: IWebAccountProviderSilentReportOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportUserInteractionRequired().into()
        }
        unsafe extern "system" fn ReportUserInteractionRequiredWithError<Impl: IWebAccountProviderSilentReportOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportUserInteractionRequiredWithError(&*(&value as *const <super::Core::WebProviderError as ::windows::core::Abi>::Abi as *const <super::Core::WebProviderError as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderSilentReportOperation, BASE_OFFSET>(),
            ReportUserInteractionRequired: ReportUserInteractionRequired::<Impl, IMPL_OFFSET>,
            ReportUserInteractionRequiredWithError: ReportUserInteractionRequiredWithError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderSilentReportOperation as ::windows::core::Interface>::IID
    }
}
pub trait IWebAccountProviderTokenObjectsImpl: Sized {
    fn Operation(&mut self) -> ::windows::core::Result<IWebAccountProviderOperation>;
}
impl ::windows::core::RuntimeName for IWebAccountProviderTokenObjects {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects";
}
impl IWebAccountProviderTokenObjectsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderTokenObjectsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderTokenObjectsVtbl {
        unsafe extern "system" fn Operation<Impl: IWebAccountProviderTokenObjectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Operation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderTokenObjects, BASE_OFFSET>(),
            Operation: Operation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderTokenObjects as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "System")]
pub trait IWebAccountProviderTokenObjects2Impl: Sized + IWebAccountProviderTokenObjectsImpl {
    fn User(&mut self) -> ::windows::core::Result<super::super::super::super::System::User>;
}
#[cfg(feature = "System")]
impl ::windows::core::RuntimeName for IWebAccountProviderTokenObjects2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects2";
}
#[cfg(feature = "System")]
impl IWebAccountProviderTokenObjects2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderTokenObjects2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderTokenObjects2Vtbl {
        unsafe extern "system" fn User<Impl: IWebAccountProviderTokenObjects2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderTokenObjects2, BASE_OFFSET>(), User: User::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderTokenObjects2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
pub trait IWebAccountProviderTokenOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn ProviderRequest(&mut self) -> ::windows::core::Result<WebProviderTokenRequest>;
    fn ProviderResponses(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>>;
    fn SetCacheExpirationTime(&mut self, value: &super::super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn CacheExpirationTime(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IWebAccountProviderTokenOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenOperation";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IWebAccountProviderTokenOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderTokenOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderTokenOperationVtbl {
        unsafe extern "system" fn ProviderRequest<Impl: IWebAccountProviderTokenOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderResponses<Impl: IWebAccountProviderTokenOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderResponses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCacheExpirationTime<Impl: IWebAccountProviderTokenOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCacheExpirationTime(&*(&value as *const <super::super::super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CacheExpirationTime<Impl: IWebAccountProviderTokenOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CacheExpirationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderTokenOperation, BASE_OFFSET>(),
            ProviderRequest: ProviderRequest::<Impl, IMPL_OFFSET>,
            ProviderResponses: ProviderResponses::<Impl, IMPL_OFFSET>,
            SetCacheExpirationTime: SetCacheExpirationTime::<Impl, IMPL_OFFSET>,
            CacheExpirationTime: CacheExpirationTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderTokenOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Security_Authentication_Web_Core")]
pub trait IWebAccountProviderUIReportOperationImpl: Sized + IWebAccountProviderBaseReportOperationImpl {
    fn ReportUserCanceled(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl ::windows::core::RuntimeName for IWebAccountProviderUIReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderUIReportOperation";
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl IWebAccountProviderUIReportOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderUIReportOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderUIReportOperationVtbl {
        unsafe extern "system" fn ReportUserCanceled<Impl: IWebAccountProviderUIReportOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportUserCanceled().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderUIReportOperation, BASE_OFFSET>(),
            ReportUserCanceled: ReportUserCanceled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderUIReportOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWebAccountScopeManagerStaticsImpl: Sized {
    fn AddWebAccountWithScopeAsync(&mut self, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>, scope: WebAccountScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn SetScopeAsync(&mut self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, scope: WebAccountScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn GetScope(&mut self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<WebAccountScope>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountScopeManagerStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountScopeManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAccountScopeManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountScopeManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountScopeManagerStaticsVtbl {
        unsafe extern "system" fn AddWebAccountWithScopeAsync<Impl: IWebAccountScopeManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, scope: WebAccountScope, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn SetScopeAsync<Impl: IWebAccountScopeManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, scope: WebAccountScope, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScopeAsync(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType), scope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScope<Impl: IWebAccountScopeManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut WebAccountScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScope(&*(&webaccount as *const <super::super::super::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountScopeManagerStatics, BASE_OFFSET>(),
            AddWebAccountWithScopeAsync: AddWebAccountWithScopeAsync::<Impl, IMPL_OFFSET>,
            SetScopeAsync: SetScopeAsync::<Impl, IMPL_OFFSET>,
            GetScope: GetScope::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountScopeManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "Security_Cryptography_Core", feature = "implement_exclusive"))]
pub trait IWebProviderTokenRequestImpl: Sized {
    fn ClientRequest(&mut self) -> ::windows::core::Result<super::Core::WebTokenRequest>;
    fn WebAccounts(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>;
    fn WebAccountSelectionOptions(&mut self) -> ::windows::core::Result<WebAccountSelectionOptions>;
    fn ApplicationCallbackUri(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn GetApplicationTokenBindingKeyAsync(&mut self, keytype: super::TokenBindingKeyType, target: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Cryptography::Core::CryptographicKey>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "Security_Cryptography_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebProviderTokenRequest {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "Security_Cryptography_Core", feature = "implement_exclusive"))]
impl IWebProviderTokenRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebProviderTokenRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebProviderTokenRequestVtbl {
        unsafe extern "system" fn ClientRequest<Impl: IWebProviderTokenRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebAccounts<Impl: IWebProviderTokenRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebAccounts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebAccountSelectionOptions<Impl: IWebProviderTokenRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebAccountSelectionOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebAccountSelectionOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationCallbackUri<Impl: IWebProviderTokenRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationCallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationTokenBindingKeyAsync<Impl: IWebProviderTokenRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keytype: super::TokenBindingKeyType, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplicationTokenBindingKeyAsync(keytype, &*(&target as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebProviderTokenRequest, BASE_OFFSET>(),
            ClientRequest: ClientRequest::<Impl, IMPL_OFFSET>,
            WebAccounts: WebAccounts::<Impl, IMPL_OFFSET>,
            WebAccountSelectionOptions: WebAccountSelectionOptions::<Impl, IMPL_OFFSET>,
            ApplicationCallbackUri: ApplicationCallbackUri::<Impl, IMPL_OFFSET>,
            GetApplicationTokenBindingKeyAsync: GetApplicationTokenBindingKeyAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebProviderTokenRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IWebProviderTokenRequest2Impl: Sized {
    fn GetApplicationTokenBindingKeyIdAsync(&mut self, keytype: super::TokenBindingKeyType, target: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebProviderTokenRequest2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IWebProviderTokenRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebProviderTokenRequest2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebProviderTokenRequest2Vtbl {
        unsafe extern "system" fn GetApplicationTokenBindingKeyIdAsync<Impl: IWebProviderTokenRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keytype: super::TokenBindingKeyType, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplicationTokenBindingKeyIdAsync(keytype, &*(&target as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebProviderTokenRequest2, BASE_OFFSET>(),
            GetApplicationTokenBindingKeyIdAsync: GetApplicationTokenBindingKeyIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebProviderTokenRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebProviderTokenRequest3Impl: Sized {
    fn ApplicationPackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ApplicationProcessName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CheckApplicationForCapabilityAsync(&mut self, capabilityname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebProviderTokenRequest3 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebProviderTokenRequest3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebProviderTokenRequest3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebProviderTokenRequest3Vtbl {
        unsafe extern "system" fn ApplicationPackageFamilyName<Impl: IWebProviderTokenRequest3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationProcessName<Impl: IWebProviderTokenRequest3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationProcessName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckApplicationForCapabilityAsync<Impl: IWebProviderTokenRequest3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckApplicationForCapabilityAsync(&*(&capabilityname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebProviderTokenRequest3, BASE_OFFSET>(),
            ApplicationPackageFamilyName: ApplicationPackageFamilyName::<Impl, IMPL_OFFSET>,
            ApplicationProcessName: ApplicationProcessName::<Impl, IMPL_OFFSET>,
            CheckApplicationForCapabilityAsync: CheckApplicationForCapabilityAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebProviderTokenRequest3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Authentication_Web_Core", feature = "implement_exclusive"))]
pub trait IWebProviderTokenResponseImpl: Sized {
    fn ClientResponse(&mut self) -> ::windows::core::Result<super::Core::WebTokenResponse>;
}
#[cfg(all(feature = "Security_Authentication_Web_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebProviderTokenResponse {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponse";
}
#[cfg(all(feature = "Security_Authentication_Web_Core", feature = "implement_exclusive"))]
impl IWebProviderTokenResponseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebProviderTokenResponseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebProviderTokenResponseVtbl {
        unsafe extern "system" fn ClientResponse<Impl: IWebProviderTokenResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientResponse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebProviderTokenResponse, BASE_OFFSET>(),
            ClientResponse: ClientResponse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebProviderTokenResponse as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Authentication_Web_Core", feature = "implement_exclusive"))]
pub trait IWebProviderTokenResponseFactoryImpl: Sized {
    fn Create(&mut self, webtokenresponse: &::core::option::Option<super::Core::WebTokenResponse>) -> ::windows::core::Result<WebProviderTokenResponse>;
}
#[cfg(all(feature = "Security_Authentication_Web_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebProviderTokenResponseFactory {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponseFactory";
}
#[cfg(all(feature = "Security_Authentication_Web_Core", feature = "implement_exclusive"))]
impl IWebProviderTokenResponseFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebProviderTokenResponseFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebProviderTokenResponseFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IWebProviderTokenResponseFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webtokenresponse: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&webtokenresponse as *const <super::Core::WebTokenResponse as ::windows::core::Abi>::Abi as *const <super::Core::WebTokenResponse as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebProviderTokenResponseFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebProviderTokenResponseFactory as ::windows::core::Interface>::IID
    }
}
