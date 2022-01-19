#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
pub trait IIndexableContent_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn Stream(&mut self) -> ::windows::core::Result<super::Streams::IRandomAccessStream>;
    fn SetStream(&mut self, value: &::core::option::Option<super::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn StreamContentType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetStreamContentType(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IIndexableContent {
    const NAME: &'static str = "Windows.Storage.Search.IIndexableContent";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl IIndexableContent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIndexableContent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIndexableContent_Vtbl {
        unsafe extern "system" fn Id<Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Properties<Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStream<Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStream(&*(&value as *const <super::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StreamContentType<Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamContentType<Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreamContentType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIndexableContent, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            SetId: SetId::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Stream: Stream::<Impl, IMPL_OFFSET>,
            SetStream: SetStream::<Impl, IMPL_OFFSET>,
            StreamContentType: StreamContentType::<Impl, IMPL_OFFSET>,
            SetStreamContentType: SetStreamContentType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIndexableContent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
pub trait IStorageFolderQueryOperations_Impl: Sized {
    fn GetIndexedStateAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IndexedState>>;
    fn CreateFileQueryOverloadDefault(&mut self) -> ::windows::core::Result<StorageFileQueryResult>;
    fn CreateFileQuery(&mut self, query: CommonFileQuery) -> ::windows::core::Result<StorageFileQueryResult>;
    fn CreateFileQueryWithOptions(&mut self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<StorageFileQueryResult>;
    fn CreateFolderQueryOverloadDefault(&mut self) -> ::windows::core::Result<StorageFolderQueryResult>;
    fn CreateFolderQuery(&mut self, query: CommonFolderQuery) -> ::windows::core::Result<StorageFolderQueryResult>;
    fn CreateFolderQueryWithOptions(&mut self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<StorageFolderQueryResult>;
    fn CreateItemQuery(&mut self) -> ::windows::core::Result<StorageItemQueryResult>;
    fn CreateItemQueryWithOptions(&mut self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<StorageItemQueryResult>;
    fn GetFilesAsync(&mut self, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFilesAsyncOverloadDefaultStartAndCount(&mut self, query: CommonFileQuery) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFoldersAsync(&mut self, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetFoldersAsyncOverloadDefaultStartAndCount(&mut self, query: CommonFolderQuery) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetItemsAsync(&mut self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>;
    fn AreQueryOptionsSupported(&mut self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<bool>;
    fn IsCommonFolderQuerySupported(&mut self, query: CommonFolderQuery) -> ::windows::core::Result<bool>;
    fn IsCommonFileQuerySupported(&mut self, query: CommonFileQuery) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IStorageFolderQueryOperations {
    const NAME: &'static str = "Windows.Storage.Search.IStorageFolderQueryOperations";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IStorageFolderQueryOperations_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFolderQueryOperations_Vtbl {
        unsafe extern "system" fn GetIndexedStateAsync<Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndexedStateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileQueryOverloadDefault<Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileQueryOverloadDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileQuery<Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileQuery(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileQueryWithOptions<Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileQueryWithOptions(&*(&queryoptions as *const <QueryOptions as ::windows::core::Abi>::Abi as *const <QueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderQueryOverloadDefault<Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFolderQueryOverloadDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderQuery<Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFolderQuery(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderQueryWithOptions<Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFolderQueryWithOptions(&*(&queryoptions as *const <QueryOptions as ::windows::core::Abi>::Abi as *const <QueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItemQuery<Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateItemQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItemQueryWithOptions<Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateItemQueryWithOptions(&*(&queryoptions as *const <QueryOptions as ::windows::core::Abi>::Abi as *const <QueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilesAsync<Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilesAsync(query, startindex, maxitemstoretrieve) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilesAsyncOverloadDefaultStartAndCount<Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilesAsyncOverloadDefaultStartAndCount(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFoldersAsync<Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFoldersAsync(query, startindex, maxitemstoretrieve) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFoldersAsyncOverloadDefaultStartAndCount<Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFoldersAsyncOverloadDefaultStartAndCount(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsAsync<Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemsAsync(startindex, maxitemstoretrieve) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreQueryOptionsSupported<Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreQueryOptionsSupported(&*(&queryoptions as *const <QueryOptions as ::windows::core::Abi>::Abi as *const <QueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCommonFolderQuerySupported<Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCommonFolderQuerySupported(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCommonFileQuerySupported<Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCommonFileQuerySupported(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFolderQueryOperations, BASE_OFFSET>(),
            GetIndexedStateAsync: GetIndexedStateAsync::<Impl, IMPL_OFFSET>,
            CreateFileQueryOverloadDefault: CreateFileQueryOverloadDefault::<Impl, IMPL_OFFSET>,
            CreateFileQuery: CreateFileQuery::<Impl, IMPL_OFFSET>,
            CreateFileQueryWithOptions: CreateFileQueryWithOptions::<Impl, IMPL_OFFSET>,
            CreateFolderQueryOverloadDefault: CreateFolderQueryOverloadDefault::<Impl, IMPL_OFFSET>,
            CreateFolderQuery: CreateFolderQuery::<Impl, IMPL_OFFSET>,
            CreateFolderQueryWithOptions: CreateFolderQueryWithOptions::<Impl, IMPL_OFFSET>,
            CreateItemQuery: CreateItemQuery::<Impl, IMPL_OFFSET>,
            CreateItemQueryWithOptions: CreateItemQueryWithOptions::<Impl, IMPL_OFFSET>,
            GetFilesAsync: GetFilesAsync::<Impl, IMPL_OFFSET>,
            GetFilesAsyncOverloadDefaultStartAndCount: GetFilesAsyncOverloadDefaultStartAndCount::<Impl, IMPL_OFFSET>,
            GetFoldersAsync: GetFoldersAsync::<Impl, IMPL_OFFSET>,
            GetFoldersAsyncOverloadDefaultStartAndCount: GetFoldersAsyncOverloadDefaultStartAndCount::<Impl, IMPL_OFFSET>,
            GetItemsAsync: GetItemsAsync::<Impl, IMPL_OFFSET>,
            AreQueryOptionsSupported: AreQueryOptionsSupported::<Impl, IMPL_OFFSET>,
            IsCommonFolderQuerySupported: IsCommonFolderQuerySupported::<Impl, IMPL_OFFSET>,
            IsCommonFileQuerySupported: IsCommonFileQuerySupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFolderQueryOperations as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IStorageQueryResultBase_Impl: Sized {
    fn GetItemCountAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn Folder(&mut self) -> ::windows::core::Result<super::StorageFolder>;
    fn ContentsChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentsChanged(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn OptionsChanged(&mut self, changedhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOptionsChanged(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindStartIndexAsync(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetCurrentQueryOptions(&mut self) -> ::windows::core::Result<QueryOptions>;
    fn ApplyNewQueryOptions(&mut self, newqueryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IStorageQueryResultBase {
    const NAME: &'static str = "Windows.Storage.Search.IStorageQueryResultBase";
}
#[cfg(feature = "Foundation")]
impl IStorageQueryResultBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageQueryResultBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageQueryResultBase_Vtbl {
        unsafe extern "system" fn GetItemCountAsync<Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemCountAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Folder<Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Folder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentsChanged<Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentsChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContentsChanged<Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveContentsChanged(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OptionsChanged<Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionsChanged(&*(&changedhandler as *const <super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOptionsChanged<Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOptionsChanged(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FindStartIndexAsync<Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindStartIndexAsync(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentQueryOptions<Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentQueryOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyNewQueryOptions<Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newqueryoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyNewQueryOptions(&*(&newqueryoptions as *const <QueryOptions as ::windows::core::Abi>::Abi as *const <QueryOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageQueryResultBase, BASE_OFFSET>(),
            GetItemCountAsync: GetItemCountAsync::<Impl, IMPL_OFFSET>,
            Folder: Folder::<Impl, IMPL_OFFSET>,
            ContentsChanged: ContentsChanged::<Impl, IMPL_OFFSET>,
            RemoveContentsChanged: RemoveContentsChanged::<Impl, IMPL_OFFSET>,
            OptionsChanged: OptionsChanged::<Impl, IMPL_OFFSET>,
            RemoveOptionsChanged: RemoveOptionsChanged::<Impl, IMPL_OFFSET>,
            FindStartIndexAsync: FindStartIndexAsync::<Impl, IMPL_OFFSET>,
            GetCurrentQueryOptions: GetCurrentQueryOptions::<Impl, IMPL_OFFSET>,
            ApplyNewQueryOptions: ApplyNewQueryOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageQueryResultBase as ::windows::core::Interface>::IID
    }
}
