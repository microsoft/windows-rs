#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "Storage_Streams", feature = "Web_Syndication", feature = "implement_exclusive"))]
pub trait IAtomPubClientImpl: Sized + ISyndicationClientImpl {
    fn RetrieveServiceDocumentAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<ServiceDocument, super::Syndication::RetrievalProgress>>;
    fn RetrieveMediaResourceAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, super::Syndication::RetrievalProgress>>;
    fn RetrieveResourceAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::RetrievalProgress>>;
    fn CreateResourceAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, description: &::windows::core::HSTRING, item: &::core::option::Option<super::Syndication::SyndicationItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>>;
    fn CreateMediaResourceAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, mediatype: &::windows::core::HSTRING, description: &::windows::core::HSTRING, mediastream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>>;
    fn UpdateMediaResourceAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, mediatype: &::windows::core::HSTRING, mediastream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>;
    fn UpdateResourceAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, item: &::core::option::Option<super::Syndication::SyndicationItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>;
    fn UpdateResourceItemAsync(&self, item: &::core::option::Option<super::Syndication::SyndicationItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>;
    fn DeleteResourceAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>;
    fn DeleteResourceItemAsync(&self, item: &::core::option::Option<super::Syndication::SyndicationItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>;
    fn CancelAsyncOperations(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "Storage_Streams", feature = "Web_Syndication", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAtomPubClient {
    const NAME: &'static str = "Windows.Web.AtomPub.IAtomPubClient";
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "Storage_Streams", feature = "Web_Syndication", feature = "implement_exclusive"))]
impl IAtomPubClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAtomPubClientImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAtomPubClientVtbl {
        unsafe extern "system" fn RetrieveServiceDocumentAsync<Impl: IAtomPubClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveServiceDocumentAsync(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveMediaResourceAsync<Impl: IAtomPubClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveMediaResourceAsync(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveResourceAsync<Impl: IAtomPubClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveResourceAsync(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResourceAsync<Impl: IAtomPubClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, description: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResourceAsync(
                &*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&description as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&item as *const <super::Syndication::SyndicationItem as ::windows::core::Abi>::Abi as *const <super::Syndication::SyndicationItem as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMediaResourceAsync<Impl: IAtomPubClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, mediatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, description: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mediastream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMediaResourceAsync(
                &*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&mediatype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&description as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&mediastream as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateMediaResourceAsync<Impl: IAtomPubClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, mediatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mediastream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateMediaResourceAsync(
                &*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&mediatype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&mediastream as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateResourceAsync<Impl: IAtomPubClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateResourceAsync(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&item as *const <super::Syndication::SyndicationItem as ::windows::core::Abi>::Abi as *const <super::Syndication::SyndicationItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateResourceItemAsync<Impl: IAtomPubClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateResourceItemAsync(&*(&item as *const <super::Syndication::SyndicationItem as ::windows::core::Abi>::Abi as *const <super::Syndication::SyndicationItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteResourceAsync<Impl: IAtomPubClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteResourceAsync(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteResourceItemAsync<Impl: IAtomPubClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteResourceItemAsync(&*(&item as *const <super::Syndication::SyndicationItem as ::windows::core::Abi>::Abi as *const <super::Syndication::SyndicationItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsyncOperations<Impl: IAtomPubClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelAsyncOperations().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAtomPubClient>,
            ::windows::core::GetTrustLevel,
            RetrieveServiceDocumentAsync::<Impl, IMPL_OFFSET>,
            RetrieveMediaResourceAsync::<Impl, IMPL_OFFSET>,
            RetrieveResourceAsync::<Impl, IMPL_OFFSET>,
            CreateResourceAsync::<Impl, IMPL_OFFSET>,
            CreateMediaResourceAsync::<Impl, IMPL_OFFSET>,
            UpdateMediaResourceAsync::<Impl, IMPL_OFFSET>,
            UpdateResourceAsync::<Impl, IMPL_OFFSET>,
            UpdateResourceItemAsync::<Impl, IMPL_OFFSET>,
            DeleteResourceAsync::<Impl, IMPL_OFFSET>,
            DeleteResourceItemAsync::<Impl, IMPL_OFFSET>,
            CancelAsyncOperations::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAtomPubClient as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IAtomPubClientFactoryImpl: Sized {
    fn CreateAtomPubClientWithCredentials(&self, servercredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<AtomPubClient>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAtomPubClientFactory {
    const NAME: &'static str = "Windows.Web.AtomPub.IAtomPubClientFactory";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IAtomPubClientFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAtomPubClientFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAtomPubClientFactoryVtbl {
        unsafe extern "system" fn CreateAtomPubClientWithCredentials<Impl: IAtomPubClientFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servercredential: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAtomPubClientWithCredentials(&*(&servercredential as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAtomPubClientFactory>, ::windows::core::GetTrustLevel, CreateAtomPubClientWithCredentials::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAtomPubClientFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "Foundation_Collections", feature = "Web_Syndication", feature = "implement_exclusive"))]
pub trait IResourceCollectionImpl: Sized + ISyndicationNodeImpl {
    fn Title(&self) -> ::windows::core::Result<super::Syndication::ISyndicationText>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Categories(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::Syndication::SyndicationCategory>>;
    fn Accepts(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "Foundation_Collections", feature = "Web_Syndication", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceCollection {
    const NAME: &'static str = "Windows.Web.AtomPub.IResourceCollection";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "Foundation_Collections", feature = "Web_Syndication", feature = "implement_exclusive"))]
impl IResourceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceCollectionVtbl {
        unsafe extern "system" fn Title<Impl: IResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uri<Impl: IResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Categories<Impl: IResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Categories() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accepts<Impl: IResourceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Accepts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IResourceCollection>, ::windows::core::GetTrustLevel, Title::<Impl, IMPL_OFFSET>, Uri::<Impl, IMPL_OFFSET>, Categories::<Impl, IMPL_OFFSET>, Accepts::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "Foundation_Collections", feature = "Web_Syndication", feature = "implement_exclusive"))]
pub trait IServiceDocumentImpl: Sized + ISyndicationNodeImpl {
    fn Workspaces(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Workspace>>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "Foundation_Collections", feature = "Web_Syndication", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IServiceDocument {
    const NAME: &'static str = "Windows.Web.AtomPub.IServiceDocument";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "Foundation_Collections", feature = "Web_Syndication", feature = "implement_exclusive"))]
impl IServiceDocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceDocumentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceDocumentVtbl {
        unsafe extern "system" fn Workspaces<Impl: IServiceDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Workspaces() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServiceDocument>, ::windows::core::GetTrustLevel, Workspaces::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceDocument as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "Foundation_Collections", feature = "Web_Syndication", feature = "implement_exclusive"))]
pub trait IWorkspaceImpl: Sized + ISyndicationNodeImpl {
    fn Title(&self) -> ::windows::core::Result<super::Syndication::ISyndicationText>;
    fn Collections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ResourceCollection>>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "Foundation_Collections", feature = "Web_Syndication", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWorkspace {
    const NAME: &'static str = "Windows.Web.AtomPub.IWorkspace";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "Foundation_Collections", feature = "Web_Syndication", feature = "implement_exclusive"))]
impl IWorkspaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceVtbl {
        unsafe extern "system" fn Title<Impl: IWorkspaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collections<Impl: IWorkspaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Collections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWorkspace>, ::windows::core::GetTrustLevel, Title::<Impl, IMPL_OFFSET>, Collections::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspace as ::windows::core::Interface>::IID
    }
}
