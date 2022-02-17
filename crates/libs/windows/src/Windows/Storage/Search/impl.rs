#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
pub trait IIndexableContent_Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn Stream(&self) -> ::windows::core::Result<super::Streams::IRandomAccessStream>;
    fn SetStream(&self, value: &::core::option::Option<super::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn StreamContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetStreamContentType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IIndexableContent {
    const NAME: &'static str = "Windows.Storage.Search.IIndexableContent";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl IIndexableContent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIndexableContent_Impl, const OFFSET: isize>() -> IIndexableContent_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Identity: ::windows::core::IUnknownImpl, Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetId(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Identity: ::windows::core::IUnknownImpl, Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Stream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStream<Identity: ::windows::core::IUnknownImpl, Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStream(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn StreamContentType<Identity: ::windows::core::IUnknownImpl, Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StreamContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamContentType<Identity: ::windows::core::IUnknownImpl, Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStreamContentType(::core::mem::transmute(&value)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIndexableContent, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Stream: Stream::<Identity, Impl, OFFSET>,
            SetStream: SetStream::<Identity, Impl, OFFSET>,
            StreamContentType: StreamContentType::<Identity, Impl, OFFSET>,
            SetStreamContentType: SetStreamContentType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIndexableContent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IStorageFolderQueryOperations_Impl: Sized {
    fn GetIndexedStateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IndexedState>>;
    fn CreateFileQueryOverloadDefault(&self) -> ::windows::core::Result<StorageFileQueryResult>;
    fn CreateFileQuery(&self, query: CommonFileQuery) -> ::windows::core::Result<StorageFileQueryResult>;
    fn CreateFileQueryWithOptions(&self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<StorageFileQueryResult>;
    fn CreateFolderQueryOverloadDefault(&self) -> ::windows::core::Result<StorageFolderQueryResult>;
    fn CreateFolderQuery(&self, query: CommonFolderQuery) -> ::windows::core::Result<StorageFolderQueryResult>;
    fn CreateFolderQueryWithOptions(&self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<StorageFolderQueryResult>;
    fn CreateItemQuery(&self) -> ::windows::core::Result<StorageItemQueryResult>;
    fn CreateItemQueryWithOptions(&self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<StorageItemQueryResult>;
    fn GetFilesAsync(&self, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: CommonFileQuery) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFoldersAsync(&self, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: CommonFolderQuery) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>;
    fn AreQueryOptionsSupported(&self, queryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<bool>;
    fn IsCommonFolderQuerySupported(&self, query: CommonFolderQuery) -> ::windows::core::Result<bool>;
    fn IsCommonFileQuerySupported(&self, query: CommonFileQuery) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IStorageFolderQueryOperations {
    const NAME: &'static str = "Windows.Storage.Search.IStorageFolderQueryOperations";
}
#[cfg(feature = "Foundation_Collections")]
impl IStorageFolderQueryOperations_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>() -> IStorageFolderQueryOperations_Vtbl {
        unsafe extern "system" fn GetIndexedStateAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIndexedStateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileQueryOverloadDefault<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFileQueryOverloadDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileQuery<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFileQuery(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileQueryWithOptions<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFileQueryWithOptions(::core::mem::transmute(&queryoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderQueryOverloadDefault<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFolderQueryOverloadDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderQuery<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFolderQuery(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderQueryWithOptions<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFolderQueryWithOptions(::core::mem::transmute(&queryoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItemQuery<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateItemQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItemQueryWithOptions<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateItemQueryWithOptions(::core::mem::transmute(&queryoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilesAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFilesAsync(query, startindex, maxitemstoretrieve) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilesAsyncOverloadDefaultStartAndCount<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFilesAsyncOverloadDefaultStartAndCount(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFoldersAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFoldersAsync(query, startindex, maxitemstoretrieve) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFoldersAsyncOverloadDefaultStartAndCount<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFoldersAsyncOverloadDefaultStartAndCount(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItemsAsync(startindex, maxitemstoretrieve) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreQueryOptionsSupported<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AreQueryOptionsSupported(::core::mem::transmute(&queryoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCommonFolderQuerySupported<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsCommonFolderQuerySupported(query) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCommonFileQuerySupported<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFolderQueryOperations, OFFSET>(),
            GetIndexedStateAsync: GetIndexedStateAsync::<Identity, Impl, OFFSET>,
            CreateFileQueryOverloadDefault: CreateFileQueryOverloadDefault::<Identity, Impl, OFFSET>,
            CreateFileQuery: CreateFileQuery::<Identity, Impl, OFFSET>,
            CreateFileQueryWithOptions: CreateFileQueryWithOptions::<Identity, Impl, OFFSET>,
            CreateFolderQueryOverloadDefault: CreateFolderQueryOverloadDefault::<Identity, Impl, OFFSET>,
            CreateFolderQuery: CreateFolderQuery::<Identity, Impl, OFFSET>,
            CreateFolderQueryWithOptions: CreateFolderQueryWithOptions::<Identity, Impl, OFFSET>,
            CreateItemQuery: CreateItemQuery::<Identity, Impl, OFFSET>,
            CreateItemQueryWithOptions: CreateItemQueryWithOptions::<Identity, Impl, OFFSET>,
            GetFilesAsync: GetFilesAsync::<Identity, Impl, OFFSET>,
            GetFilesAsyncOverloadDefaultStartAndCount: GetFilesAsyncOverloadDefaultStartAndCount::<Identity, Impl, OFFSET>,
            GetFoldersAsync: GetFoldersAsync::<Identity, Impl, OFFSET>,
            GetFoldersAsyncOverloadDefaultStartAndCount: GetFoldersAsyncOverloadDefaultStartAndCount::<Identity, Impl, OFFSET>,
            GetItemsAsync: GetItemsAsync::<Identity, Impl, OFFSET>,
            AreQueryOptionsSupported: AreQueryOptionsSupported::<Identity, Impl, OFFSET>,
            IsCommonFolderQuerySupported: IsCommonFolderQuerySupported::<Identity, Impl, OFFSET>,
            IsCommonFileQuerySupported: IsCommonFileQuerySupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFolderQueryOperations as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IStorageQueryResultBase_Impl: Sized {
    fn GetItemCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn Folder(&self) -> ::windows::core::Result<super::StorageFolder>;
    fn ContentsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentsChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn OptionsChanged(&self, changedhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOptionsChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindStartIndexAsync(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetCurrentQueryOptions(&self) -> ::windows::core::Result<QueryOptions>;
    fn ApplyNewQueryOptions(&self, newqueryoptions: &::core::option::Option<QueryOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IStorageQueryResultBase {
    const NAME: &'static str = "Windows.Storage.Search.IStorageQueryResultBase";
}
#[cfg(feature = "Foundation")]
impl IStorageQueryResultBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>() -> IStorageQueryResultBase_Vtbl {
        unsafe extern "system" fn GetItemCountAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItemCountAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Folder<Identity: ::windows::core::IUnknownImpl, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Folder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentsChanged<Identity: ::windows::core::IUnknownImpl, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContentsChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContentsChanged<Identity: ::windows::core::IUnknownImpl, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveContentsChanged(::core::mem::transmute(&eventcookie)).into()
        }
        unsafe extern "system" fn OptionsChanged<Identity: ::windows::core::IUnknownImpl, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OptionsChanged(::core::mem::transmute(&changedhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOptionsChanged<Identity: ::windows::core::IUnknownImpl, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveOptionsChanged(::core::mem::transmute(&eventcookie)).into()
        }
        unsafe extern "system" fn FindStartIndexAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindStartIndexAsync(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentQueryOptions<Identity: ::windows::core::IUnknownImpl, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentQueryOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyNewQueryOptions<Identity: ::windows::core::IUnknownImpl, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newqueryoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ApplyNewQueryOptions(::core::mem::transmute(&newqueryoptions)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageQueryResultBase, OFFSET>(),
            GetItemCountAsync: GetItemCountAsync::<Identity, Impl, OFFSET>,
            Folder: Folder::<Identity, Impl, OFFSET>,
            ContentsChanged: ContentsChanged::<Identity, Impl, OFFSET>,
            RemoveContentsChanged: RemoveContentsChanged::<Identity, Impl, OFFSET>,
            OptionsChanged: OptionsChanged::<Identity, Impl, OFFSET>,
            RemoveOptionsChanged: RemoveOptionsChanged::<Identity, Impl, OFFSET>,
            FindStartIndexAsync: FindStartIndexAsync::<Identity, Impl, OFFSET>,
            GetCurrentQueryOptions: GetCurrentQueryOptions::<Identity, Impl, OFFSET>,
            ApplyNewQueryOptions: ApplyNewQueryOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageQueryResultBase as ::windows::core::Interface>::IID
    }
}
