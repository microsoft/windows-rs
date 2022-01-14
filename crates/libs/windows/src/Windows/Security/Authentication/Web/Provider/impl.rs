#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebAccountClientView_Impl: Sized {
    fn ApplicationCallbackUri(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn Type(&mut self) -> ::windows::core::Result<WebAccountClientViewType>;
    fn AccountPairwiseId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountClientView {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountClientView";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebAccountClientView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountClientView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountClientView_Vtbl {
        unsafe extern "system" fn ApplicationCallbackUri<Impl: IWebAccountClientView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Type<Impl: IWebAccountClientView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebAccountClientViewType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AccountPairwiseId<Impl: IWebAccountClientView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IWebAccountClientViewFactory_Impl: Sized {
    fn Create(&mut self, viewtype: WebAccountClientViewType, applicationcallbackuri: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<WebAccountClientView>;
    fn CreateWithPairwiseId(&mut self, viewtype: WebAccountClientViewType, applicationcallbackuri: &::core::option::Option<super::super::super::super::Foundation::Uri>, accountpairwiseid: &::windows::core::HSTRING) -> ::windows::core::Result<WebAccountClientView>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountClientViewFactory {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountClientViewFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebAccountClientViewFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountClientViewFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountClientViewFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IWebAccountClientViewFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewtype: WebAccountClientViewType, applicationcallbackuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithPairwiseId<Impl: IWebAccountClientViewFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewtype: WebAccountClientViewType, applicationcallbackuri: ::windows::core::RawPtr, accountpairwiseid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWebAccountManagerStatics_Impl: Sized {
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
impl IWebAccountManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountManagerStatics_Vtbl {
        unsafe extern "system" fn UpdateWebAccountPropertiesAsync<Impl: IWebAccountManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, additionalproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddWebAccountAsync<Impl: IWebAccountManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeleteWebAccountAsync<Impl: IWebAccountManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindAllProviderWebAccountsAsync<Impl: IWebAccountManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PushCookiesAsync<Impl: IWebAccountManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, cookies: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetViewAsync<Impl: IWebAccountManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, view: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClearViewAsync<Impl: IWebAccountManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, applicationcallbackuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetViewsAsync<Impl: IWebAccountManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetWebAccountPictureAsync<Impl: IWebAccountManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, webaccountpicture: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClearWebAccountPictureAsync<Impl: IWebAccountManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWebAccountManagerStatics2_Impl: Sized {
    fn PullCookiesAsync(&mut self, uristring: &::windows::core::HSTRING, callerpfn: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountManagerStatics2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebAccountManagerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountManagerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountManagerStatics2_Vtbl {
        unsafe extern "system" fn PullCookiesAsync<Impl: IWebAccountManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uristring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, callerpfn: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWebAccountManagerStatics3_Impl: Sized {
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
impl IWebAccountManagerStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountManagerStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountManagerStatics3_Vtbl {
        unsafe extern "system" fn FindAllProviderWebAccountsForUserAsync<Impl: IWebAccountManagerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddWebAccountForUserAsync<Impl: IWebAccountManagerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddWebAccountWithScopeForUserAsync<Impl: IWebAccountManagerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, scope: WebAccountScope, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddWebAccountWithScopeAndMapForUserAsync<Impl: IWebAccountManagerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, scope: WebAccountScope, peruserwebaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWebAccountManagerStatics4_Impl: Sized {
    fn InvalidateAppCacheForAllAccountsAsync(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn InvalidateAppCacheForAccountAsync(&mut self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountManagerStatics4 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics4";
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAccountManagerStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountManagerStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountManagerStatics4_Vtbl {
        unsafe extern "system" fn InvalidateAppCacheForAllAccountsAsync<Impl: IWebAccountManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InvalidateAppCacheForAccountAsync<Impl: IWebAccountManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWebAccountMapManagerStatics_Impl: Sized {
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
impl IWebAccountMapManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountMapManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountMapManagerStatics_Vtbl {
        unsafe extern "system" fn AddWebAccountWithScopeAndMapAsync<Impl: IWebAccountMapManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, scope: WebAccountScope, peruserwebaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPerAppToPerUserAccountAsync<Impl: IWebAccountMapManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perappaccount: ::windows::core::RawPtr, peruserwebaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPerUserFromPerAppAccountAsync<Impl: IWebAccountMapManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perappaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClearPerUserFromPerAppAccountAsync<Impl: IWebAccountMapManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perappaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWebAccountProviderAddAccountOperation_Impl: Sized + IWebAccountProviderOperation_Impl {
    fn ReportCompleted(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountProviderAddAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderAddAccountOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountProviderAddAccountOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderAddAccountOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderAddAccountOperation_Vtbl {
        unsafe extern "system" fn ReportCompleted<Impl: IWebAccountProviderAddAccountOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IWebAccountProviderBaseReportOperation_Impl: Sized {
    fn ReportCompleted(&mut self) -> ::windows::core::Result<()>;
    fn ReportError(&mut self, value: &::core::option::Option<super::Core::WebProviderError>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl ::windows::core::RuntimeName for IWebAccountProviderBaseReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation";
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl IWebAccountProviderBaseReportOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderBaseReportOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderBaseReportOperation_Vtbl {
        unsafe extern "system" fn ReportCompleted<Impl: IWebAccountProviderBaseReportOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCompleted().into()
        }
        unsafe extern "system" fn ReportError<Impl: IWebAccountProviderBaseReportOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWebAccountProviderDeleteAccountOperation_Impl: Sized + IWebAccountProviderOperation_Impl {
    fn WebAccount(&mut self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountProviderDeleteAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderDeleteAccountOperation";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAccountProviderDeleteAccountOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderDeleteAccountOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderDeleteAccountOperation_Vtbl {
        unsafe extern "system" fn WebAccount<Impl: IWebAccountProviderDeleteAccountOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWebAccountProviderManageAccountOperation_Impl: Sized + IWebAccountProviderOperation_Impl {
    fn WebAccount(&mut self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
    fn ReportCompleted(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountProviderManageAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderManageAccountOperation";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAccountProviderManageAccountOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderManageAccountOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderManageAccountOperation_Vtbl {
        unsafe extern "system" fn WebAccount<Impl: IWebAccountProviderManageAccountOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReportCompleted<Impl: IWebAccountProviderManageAccountOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IWebAccountProviderOperation_Impl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<WebAccountProviderOperationKind>;
}
impl ::windows::core::RuntimeName for IWebAccountProviderOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation";
}
impl IWebAccountProviderOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderOperation_Vtbl {
        unsafe extern "system" fn Kind<Impl: IWebAccountProviderOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebAccountProviderOperationKind) -> ::windows::core::HRESULT {
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
pub trait IWebAccountProviderRetrieveCookiesOperation_Impl: Sized + IWebAccountProviderOperation_Impl {
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
impl IWebAccountProviderRetrieveCookiesOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderRetrieveCookiesOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderRetrieveCookiesOperation_Vtbl {
        unsafe extern "system" fn Context<Impl: IWebAccountProviderRetrieveCookiesOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Cookies<Impl: IWebAccountProviderRetrieveCookiesOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetUri<Impl: IWebAccountProviderRetrieveCookiesOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&uri as *const <super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Uri<Impl: IWebAccountProviderRetrieveCookiesOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ApplicationCallbackUri<Impl: IWebAccountProviderRetrieveCookiesOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWebAccountProviderSignOutAccountOperation_Impl: Sized + IWebAccountProviderOperation_Impl {
    fn WebAccount(&mut self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
    fn ApplicationCallbackUri(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn ClientId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountProviderSignOutAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderSignOutAccountOperation";
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAccountProviderSignOutAccountOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderSignOutAccountOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderSignOutAccountOperation_Vtbl {
        unsafe extern "system" fn WebAccount<Impl: IWebAccountProviderSignOutAccountOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ApplicationCallbackUri<Impl: IWebAccountProviderSignOutAccountOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClientId<Impl: IWebAccountProviderSignOutAccountOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IWebAccountProviderSilentReportOperation_Impl: Sized + IWebAccountProviderBaseReportOperation_Impl {
    fn ReportUserInteractionRequired(&mut self) -> ::windows::core::Result<()>;
    fn ReportUserInteractionRequiredWithError(&mut self, value: &::core::option::Option<super::Core::WebProviderError>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl ::windows::core::RuntimeName for IWebAccountProviderSilentReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderSilentReportOperation";
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl IWebAccountProviderSilentReportOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderSilentReportOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderSilentReportOperation_Vtbl {
        unsafe extern "system" fn ReportUserInteractionRequired<Impl: IWebAccountProviderSilentReportOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportUserInteractionRequired().into()
        }
        unsafe extern "system" fn ReportUserInteractionRequiredWithError<Impl: IWebAccountProviderSilentReportOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWebAccountProviderTokenObjects_Impl: Sized {
    fn Operation(&mut self) -> ::windows::core::Result<IWebAccountProviderOperation>;
}
impl ::windows::core::RuntimeName for IWebAccountProviderTokenObjects {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects";
}
impl IWebAccountProviderTokenObjects_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderTokenObjects_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderTokenObjects_Vtbl {
        unsafe extern "system" fn Operation<Impl: IWebAccountProviderTokenObjects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWebAccountProviderTokenObjects2_Impl: Sized + IWebAccountProviderTokenObjects_Impl {
    fn User(&mut self) -> ::windows::core::Result<super::super::super::super::System::User>;
}
#[cfg(feature = "System")]
impl ::windows::core::RuntimeName for IWebAccountProviderTokenObjects2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects2";
}
#[cfg(feature = "System")]
impl IWebAccountProviderTokenObjects2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderTokenObjects2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderTokenObjects2_Vtbl {
        unsafe extern "system" fn User<Impl: IWebAccountProviderTokenObjects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWebAccountProviderTokenOperation_Impl: Sized + IWebAccountProviderOperation_Impl {
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
impl IWebAccountProviderTokenOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderTokenOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderTokenOperation_Vtbl {
        unsafe extern "system" fn ProviderRequest<Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderResponses<Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCacheExpirationTime<Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCacheExpirationTime(&*(&value as *const <super::super::super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CacheExpirationTime<Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
pub trait IWebAccountProviderUIReportOperation_Impl: Sized + IWebAccountProviderBaseReportOperation_Impl {
    fn ReportUserCanceled(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl ::windows::core::RuntimeName for IWebAccountProviderUIReportOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountProviderUIReportOperation";
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl IWebAccountProviderUIReportOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderUIReportOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderUIReportOperation_Vtbl {
        unsafe extern "system" fn ReportUserCanceled<Impl: IWebAccountProviderUIReportOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IWebAccountScopeManagerStatics_Impl: Sized {
    fn AddWebAccountWithScopeAsync(&mut self, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>, scope: WebAccountScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn SetScopeAsync(&mut self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, scope: WebAccountScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn GetScope(&mut self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<WebAccountScope>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountScopeManagerStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebAccountScopeManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAccountScopeManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountScopeManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountScopeManagerStatics_Vtbl {
        unsafe extern "system" fn AddWebAccountWithScopeAsync<Impl: IWebAccountScopeManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, scope: WebAccountScope, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScopeAsync<Impl: IWebAccountScopeManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, scope: WebAccountScope, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetScope<Impl: IWebAccountScopeManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut WebAccountScope) -> ::windows::core::HRESULT {
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
pub trait IWebProviderTokenRequest_Impl: Sized {
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
impl IWebProviderTokenRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebProviderTokenRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebProviderTokenRequest_Vtbl {
        unsafe extern "system" fn ClientRequest<Impl: IWebProviderTokenRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WebAccounts<Impl: IWebProviderTokenRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WebAccountSelectionOptions<Impl: IWebProviderTokenRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebAccountSelectionOptions) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ApplicationCallbackUri<Impl: IWebProviderTokenRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetApplicationTokenBindingKeyAsync<Impl: IWebProviderTokenRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keytype: super::TokenBindingKeyType, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWebProviderTokenRequest2_Impl: Sized {
    fn GetApplicationTokenBindingKeyIdAsync(&mut self, keytype: super::TokenBindingKeyType, target: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebProviderTokenRequest2 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IWebProviderTokenRequest2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebProviderTokenRequest2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebProviderTokenRequest2_Vtbl {
        unsafe extern "system" fn GetApplicationTokenBindingKeyIdAsync<Impl: IWebProviderTokenRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keytype: super::TokenBindingKeyType, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWebProviderTokenRequest3_Impl: Sized {
    fn ApplicationPackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ApplicationProcessName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CheckApplicationForCapabilityAsync(&mut self, capabilityname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebProviderTokenRequest3 {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebProviderTokenRequest3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebProviderTokenRequest3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebProviderTokenRequest3_Vtbl {
        unsafe extern "system" fn ApplicationPackageFamilyName<Impl: IWebProviderTokenRequest3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ApplicationProcessName<Impl: IWebProviderTokenRequest3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckApplicationForCapabilityAsync<Impl: IWebProviderTokenRequest3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWebProviderTokenResponse_Impl: Sized {
    fn ClientResponse(&mut self) -> ::windows::core::Result<super::Core::WebTokenResponse>;
}
#[cfg(all(feature = "Security_Authentication_Web_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebProviderTokenResponse {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponse";
}
#[cfg(all(feature = "Security_Authentication_Web_Core", feature = "implement_exclusive"))]
impl IWebProviderTokenResponse_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebProviderTokenResponse_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebProviderTokenResponse_Vtbl {
        unsafe extern "system" fn ClientResponse<Impl: IWebProviderTokenResponse_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWebProviderTokenResponseFactory_Impl: Sized {
    fn Create(&mut self, webtokenresponse: &::core::option::Option<super::Core::WebTokenResponse>) -> ::windows::core::Result<WebProviderTokenResponse>;
}
#[cfg(all(feature = "Security_Authentication_Web_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebProviderTokenResponseFactory {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponseFactory";
}
#[cfg(all(feature = "Security_Authentication_Web_Core", feature = "implement_exclusive"))]
impl IWebProviderTokenResponseFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebProviderTokenResponseFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebProviderTokenResponseFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IWebProviderTokenResponseFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webtokenresponse: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
