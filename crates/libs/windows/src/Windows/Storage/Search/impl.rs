#[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
pub trait IIndexableContent_Impl: Sized {
    fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>;
    fn Stream(&self) -> ::windows_core::Result<super::Streams::IRandomAccessStream>;
    fn SetStream(&self, value: ::core::option::Option<&super::Streams::IRandomAccessStream>) -> ::windows_core::Result<()>;
    fn StreamContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetStreamContentType(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl ::windows_core::RuntimeName for IIndexableContent {
    const NAME: &'static str = "Windows.Storage.Search.IIndexableContent";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl IIndexableContent_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIndexableContent_Impl, const OFFSET: isize>() -> IIndexableContent_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetId(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Stream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStream(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn StreamContentType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StreamContentType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamContentType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIndexableContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStreamContentType(::core::mem::transmute(&value)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IIndexableContent, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Stream: Stream::<Identity, Impl, OFFSET>,
            SetStream: SetStream::<Identity, Impl, OFFSET>,
            StreamContentType: StreamContentType::<Identity, Impl, OFFSET>,
            SetStreamContentType: SetStreamContentType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IIndexableContent as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation_Collections")]
pub trait IStorageFolderQueryOperations_Impl: Sized {
    fn GetIndexedStateAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<IndexedState>>;
    fn CreateFileQueryOverloadDefault(&self) -> ::windows_core::Result<StorageFileQueryResult>;
    fn CreateFileQuery(&self, query: CommonFileQuery) -> ::windows_core::Result<StorageFileQueryResult>;
    fn CreateFileQueryWithOptions(&self, queryoptions: ::core::option::Option<&QueryOptions>) -> ::windows_core::Result<StorageFileQueryResult>;
    fn CreateFolderQueryOverloadDefault(&self) -> ::windows_core::Result<StorageFolderQueryResult>;
    fn CreateFolderQuery(&self, query: CommonFolderQuery) -> ::windows_core::Result<StorageFolderQueryResult>;
    fn CreateFolderQueryWithOptions(&self, queryoptions: ::core::option::Option<&QueryOptions>) -> ::windows_core::Result<StorageFolderQueryResult>;
    fn CreateItemQuery(&self) -> ::windows_core::Result<StorageItemQueryResult>;
    fn CreateItemQueryWithOptions(&self, queryoptions: ::core::option::Option<&QueryOptions>) -> ::windows_core::Result<StorageItemQueryResult>;
    fn GetFilesAsync(&self, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: CommonFileQuery) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFoldersAsync(&self, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: CommonFolderQuery) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>;
    fn AreQueryOptionsSupported(&self, queryoptions: ::core::option::Option<&QueryOptions>) -> ::windows_core::Result<bool>;
    fn IsCommonFolderQuerySupported(&self, query: CommonFolderQuery) -> ::windows_core::Result<bool>;
    fn IsCommonFileQuerySupported(&self, query: CommonFileQuery) -> ::windows_core::Result<bool>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for IStorageFolderQueryOperations {
    const NAME: &'static str = "Windows.Storage.Search.IStorageFolderQueryOperations";
}
#[cfg(feature = "Foundation_Collections")]
impl IStorageFolderQueryOperations_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>() -> IStorageFolderQueryOperations_Vtbl {
        unsafe extern "system" fn GetIndexedStateAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIndexedStateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileQueryOverloadDefault<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFileQueryOverloadDefault() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileQuery<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFileQuery(query) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileQueryWithOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFileQueryWithOptions(::windows_core::from_raw_borrowed(&queryoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderQueryOverloadDefault<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFolderQueryOverloadDefault() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderQuery<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFolderQuery(query) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderQueryWithOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFolderQueryWithOptions(::windows_core::from_raw_borrowed(&queryoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItemQuery<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateItemQuery() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItemQueryWithOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateItemQueryWithOptions(::windows_core::from_raw_borrowed(&queryoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilesAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFilesAsync(query, startindex, maxitemstoretrieve) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilesAsyncOverloadDefaultStartAndCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFilesAsyncOverloadDefaultStartAndCount(query) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFoldersAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFoldersAsync(query, startindex, maxitemstoretrieve) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFoldersAsyncOverloadDefaultStartAndCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFoldersAsyncOverloadDefaultStartAndCount(query) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemsAsync(startindex, maxitemstoretrieve) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreQueryOptionsSupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryoptions: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AreQueryOptionsSupported(::windows_core::from_raw_borrowed(&queryoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCommonFolderQuerySupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCommonFolderQuerySupported(query) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCommonFileQuerySupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCommonFileQuerySupported(query) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IStorageFolderQueryOperations, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IStorageFolderQueryOperations as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IStorageQueryResultBase_Impl: Sized {
    fn GetItemCountAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn Folder(&self) -> ::windows_core::Result<super::StorageFolder>;
    fn ContentsChanged(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentsChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn OptionsChanged(&self, changedhandler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOptionsChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn FindStartIndexAsync(&self, value: ::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetCurrentQueryOptions(&self) -> ::windows_core::Result<QueryOptions>;
    fn ApplyNewQueryOptions(&self, newqueryoptions: ::core::option::Option<&QueryOptions>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for IStorageQueryResultBase {
    const NAME: &'static str = "Windows.Storage.Search.IStorageQueryResultBase";
}
#[cfg(feature = "Foundation")]
impl IStorageQueryResultBase_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>() -> IStorageQueryResultBase_Vtbl {
        unsafe extern "system" fn GetItemCountAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemCountAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Folder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Folder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentsChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContentsChanged(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContentsChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveContentsChanged(::core::mem::transmute(&eventcookie)).into()
        }
        unsafe extern "system" fn OptionsChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OptionsChanged(::windows_core::from_raw_borrowed(&changedhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOptionsChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveOptionsChanged(::core::mem::transmute(&eventcookie)).into()
        }
        unsafe extern "system" fn FindStartIndexAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindStartIndexAsync(::windows_core::from_raw_borrowed(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentQueryOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentQueryOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyNewQueryOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newqueryoptions: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplyNewQueryOptions(::windows_core::from_raw_borrowed(&newqueryoptions)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IStorageQueryResultBase, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IStorageQueryResultBase as ::windows_core::ComInterface>::IID
    }
}
