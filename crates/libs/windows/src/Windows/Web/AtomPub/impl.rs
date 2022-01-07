#[cfg(all(feature = "Web_Syndication", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Web_Syndication", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAtomPubClient {
    const NAME: &'static str = "Windows.Web.AtomPub.IAtomPubClient";
}
#[cfg(all(feature = "Web_Syndication", feature = "implement_exclusive"))]
impl IAtomPubClientVtbl {
    pub const fn new<Impl: IAtomPubClientImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAtomPubClientVtbl {
        unsafe extern "system" fn RetrieveServiceDocumentAsync<Impl: IAtomPubClientImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RetrieveServiceDocumentAsync(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveMediaResourceAsync<Impl: IAtomPubClientImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RetrieveMediaResourceAsync(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveResourceAsync<Impl: IAtomPubClientImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RetrieveResourceAsync(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResourceAsync<Impl: IAtomPubClientImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, description: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn CreateMediaResourceAsync<Impl: IAtomPubClientImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, mediatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, description: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mediastream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn UpdateMediaResourceAsync<Impl: IAtomPubClientImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, mediatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mediastream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn UpdateResourceAsync<Impl: IAtomPubClientImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateResourceAsync(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&item as *const <super::Syndication::SyndicationItem as ::windows::core::Abi>::Abi as *const <super::Syndication::SyndicationItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateResourceItemAsync<Impl: IAtomPubClientImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateResourceItemAsync(&*(&item as *const <super::Syndication::SyndicationItem as ::windows::core::Abi>::Abi as *const <super::Syndication::SyndicationItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteResourceAsync<Impl: IAtomPubClientImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteResourceAsync(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteResourceItemAsync<Impl: IAtomPubClientImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteResourceItemAsync(&*(&item as *const <super::Syndication::SyndicationItem as ::windows::core::Abi>::Abi as *const <super::Syndication::SyndicationItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsyncOperations<Impl: IAtomPubClientImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).CancelAsyncOperations().into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAtomPubClient>,
            base.5,
            RetrieveServiceDocumentAsync::<Impl, OFFSET>,
            RetrieveMediaResourceAsync::<Impl, OFFSET>,
            RetrieveResourceAsync::<Impl, OFFSET>,
            CreateResourceAsync::<Impl, OFFSET>,
            CreateMediaResourceAsync::<Impl, OFFSET>,
            UpdateMediaResourceAsync::<Impl, OFFSET>,
            UpdateResourceAsync::<Impl, OFFSET>,
            UpdateResourceItemAsync::<Impl, OFFSET>,
            DeleteResourceAsync::<Impl, OFFSET>,
            DeleteResourceItemAsync::<Impl, OFFSET>,
            CancelAsyncOperations::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAtomPubClientFactoryImpl: Sized {
    fn CreateAtomPubClientWithCredentials(&self, servercredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<AtomPubClient>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAtomPubClientFactory {
    const NAME: &'static str = "Windows.Web.AtomPub.IAtomPubClientFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAtomPubClientFactoryVtbl {
    pub const fn new<Impl: IAtomPubClientFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAtomPubClientFactoryVtbl {
        unsafe extern "system" fn CreateAtomPubClientWithCredentials<Impl: IAtomPubClientFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, servercredential: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAtomPubClientWithCredentials(&*(&servercredential as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAtomPubClientFactory>, base.5, CreateAtomPubClientWithCredentials::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Web_Syndication", feature = "implement_exclusive"))]
pub trait IResourceCollectionImpl: Sized + ISyndicationNodeImpl {
    fn Title(&self) -> ::windows::core::Result<super::Syndication::ISyndicationText>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Categories(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::Syndication::SyndicationCategory>>;
    fn Accepts(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Web_Syndication", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceCollection {
    const NAME: &'static str = "Windows.Web.AtomPub.IResourceCollection";
}
#[cfg(all(feature = "Web_Syndication", feature = "implement_exclusive"))]
impl IResourceCollectionVtbl {
    pub const fn new<Impl: IResourceCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceCollectionVtbl {
        unsafe extern "system" fn Title<Impl: IResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uri<Impl: IResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Categories<Impl: IResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Categories() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accepts<Impl: IResourceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Accepts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceCollection>, base.5, Title::<Impl, OFFSET>, Uri::<Impl, OFFSET>, Categories::<Impl, OFFSET>, Accepts::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Web_Syndication", feature = "implement_exclusive"))]
pub trait IServiceDocumentImpl: Sized + ISyndicationNodeImpl {
    fn Workspaces(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Workspace>>;
}
#[cfg(all(feature = "Web_Syndication", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IServiceDocument {
    const NAME: &'static str = "Windows.Web.AtomPub.IServiceDocument";
}
#[cfg(all(feature = "Web_Syndication", feature = "implement_exclusive"))]
impl IServiceDocumentVtbl {
    pub const fn new<Impl: IServiceDocumentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IServiceDocumentVtbl {
        unsafe extern "system" fn Workspaces<Impl: IServiceDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Workspaces() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IServiceDocument>, base.5, Workspaces::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Web_Syndication", feature = "implement_exclusive"))]
pub trait IWorkspaceImpl: Sized + ISyndicationNodeImpl {
    fn Title(&self) -> ::windows::core::Result<super::Syndication::ISyndicationText>;
    fn Collections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ResourceCollection>>;
}
#[cfg(all(feature = "Web_Syndication", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWorkspace {
    const NAME: &'static str = "Windows.Web.AtomPub.IWorkspace";
}
#[cfg(all(feature = "Web_Syndication", feature = "implement_exclusive"))]
impl IWorkspaceVtbl {
    pub const fn new<Impl: IWorkspaceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWorkspaceVtbl {
        unsafe extern "system" fn Title<Impl: IWorkspaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collections<Impl: IWorkspaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Collections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWorkspace>, base.5, Title::<Impl, OFFSET>, Collections::<Impl, OFFSET>)
    }
}
