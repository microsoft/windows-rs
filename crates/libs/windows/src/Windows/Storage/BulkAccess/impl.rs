#[cfg(feature = "implement_exclusive")]
pub trait IFileInformationFactoryImpl: Sized {
    fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>>;
    fn GetItemsAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>>;
    fn GetFilesAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>>;
    fn GetFilesAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>>;
    fn GetFoldersAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>>;
    fn GetFoldersAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>>;
    fn GetVirtualizedItemsVector(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetVirtualizedFilesVector(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetVirtualizedFoldersVector(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileInformationFactory {
    const NAME: &'static str = "Windows.Storage.BulkAccess.IFileInformationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFileInformationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileInformationFactoryImpl, const OFFSET: isize>() -> IFileInformationFactoryVtbl {
        unsafe extern "system" fn GetItemsAsync<Impl: IFileInformationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetItemsAsyncDefaultStartAndCount<Impl: IFileInformationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemsAsyncDefaultStartAndCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilesAsync<Impl: IFileInformationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilesAsync(startindex, maxitemstoretrieve) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilesAsyncDefaultStartAndCount<Impl: IFileInformationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilesAsyncDefaultStartAndCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFoldersAsync<Impl: IFileInformationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFoldersAsync(startindex, maxitemstoretrieve) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFoldersAsyncDefaultStartAndCount<Impl: IFileInformationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFoldersAsyncDefaultStartAndCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVirtualizedItemsVector<Impl: IFileInformationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVirtualizedItemsVector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVirtualizedFilesVector<Impl: IFileInformationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVirtualizedFilesVector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVirtualizedFoldersVector<Impl: IFileInformationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVirtualizedFoldersVector() {
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
            ::windows::core::GetRuntimeClassName::<IFileInformationFactory>,
            ::windows::core::GetTrustLevel,
            GetItemsAsync::<Impl, OFFSET>,
            GetItemsAsyncDefaultStartAndCount::<Impl, OFFSET>,
            GetFilesAsync::<Impl, OFFSET>,
            GetFilesAsyncDefaultStartAndCount::<Impl, OFFSET>,
            GetFoldersAsync::<Impl, OFFSET>,
            GetFoldersAsyncDefaultStartAndCount::<Impl, OFFSET>,
            GetVirtualizedItemsVector::<Impl, OFFSET>,
            GetVirtualizedFilesVector::<Impl, OFFSET>,
            GetVirtualizedFoldersVector::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileInformationFactoryFactoryImpl: Sized {
    fn CreateWithMode(&self, queryresult: &::core::option::Option<super::Search::IStorageQueryResultBase>, mode: super::FileProperties::ThumbnailMode) -> ::windows::core::Result<FileInformationFactory>;
    fn CreateWithModeAndSize(&self, queryresult: &::core::option::Option<super::Search::IStorageQueryResultBase>, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32) -> ::windows::core::Result<FileInformationFactory>;
    fn CreateWithModeAndSizeAndOptions(&self, queryresult: &::core::option::Option<super::Search::IStorageQueryResultBase>, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions) -> ::windows::core::Result<FileInformationFactory>;
    fn CreateWithModeAndSizeAndOptionsAndFlags(&self, queryresult: &::core::option::Option<super::Search::IStorageQueryResultBase>, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, delayload: bool) -> ::windows::core::Result<FileInformationFactory>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileInformationFactoryFactory {
    const NAME: &'static str = "Windows.Storage.BulkAccess.IFileInformationFactoryFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFileInformationFactoryFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileInformationFactoryFactoryImpl, const OFFSET: isize>() -> IFileInformationFactoryFactoryVtbl {
        unsafe extern "system" fn CreateWithMode<Impl: IFileInformationFactoryFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryresult: ::windows::core::RawPtr, mode: super::FileProperties::ThumbnailMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithMode(&*(&queryresult as *const <super::Search::IStorageQueryResultBase as ::windows::core::Abi>::Abi as *const <super::Search::IStorageQueryResultBase as ::windows::core::DefaultType>::DefaultType), mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithModeAndSize<Impl: IFileInformationFactoryFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryresult: ::windows::core::RawPtr, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithModeAndSize(&*(&queryresult as *const <super::Search::IStorageQueryResultBase as ::windows::core::Abi>::Abi as *const <super::Search::IStorageQueryResultBase as ::windows::core::DefaultType>::DefaultType), mode, requestedthumbnailsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithModeAndSizeAndOptions<Impl: IFileInformationFactoryFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryresult: ::windows::core::RawPtr, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithModeAndSizeAndOptions(&*(&queryresult as *const <super::Search::IStorageQueryResultBase as ::windows::core::Abi>::Abi as *const <super::Search::IStorageQueryResultBase as ::windows::core::DefaultType>::DefaultType), mode, requestedthumbnailsize, thumbnailoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithModeAndSizeAndOptionsAndFlags<Impl: IFileInformationFactoryFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryresult: ::windows::core::RawPtr, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, delayload: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithModeAndSizeAndOptionsAndFlags(&*(&queryresult as *const <super::Search::IStorageQueryResultBase as ::windows::core::Abi>::Abi as *const <super::Search::IStorageQueryResultBase as ::windows::core::DefaultType>::DefaultType), mode, requestedthumbnailsize, thumbnailoptions, delayload) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileInformationFactoryFactory>, ::windows::core::GetTrustLevel, CreateWithMode::<Impl, OFFSET>, CreateWithModeAndSize::<Impl, OFFSET>, CreateWithModeAndSizeAndOptions::<Impl, OFFSET>, CreateWithModeAndSizeAndOptionsAndFlags::<Impl, OFFSET>)
    }
}
pub trait IStorageItemInformationImpl: Sized {
    fn MusicProperties(&self) -> ::windows::core::Result<super::FileProperties::MusicProperties>;
    fn VideoProperties(&self) -> ::windows::core::Result<super::FileProperties::VideoProperties>;
    fn ImageProperties(&self) -> ::windows::core::Result<super::FileProperties::ImageProperties>;
    fn DocumentProperties(&self) -> ::windows::core::Result<super::FileProperties::DocumentProperties>;
    fn BasicProperties(&self) -> ::windows::core::Result<super::FileProperties::BasicProperties>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::FileProperties::StorageItemThumbnail>;
    fn ThumbnailUpdated(&self, changedhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveThumbnailUpdated(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PropertiesUpdated(&self, changedhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePropertiesUpdated(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IStorageItemInformation {
    const NAME: &'static str = "Windows.Storage.BulkAccess.IStorageItemInformation";
}
impl IStorageItemInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemInformationImpl, const OFFSET: isize>() -> IStorageItemInformationVtbl {
        unsafe extern "system" fn MusicProperties<Impl: IStorageItemInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MusicProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProperties<Impl: IStorageItemInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageProperties<Impl: IStorageItemInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentProperties<Impl: IStorageItemInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BasicProperties<Impl: IStorageItemInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BasicProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thumbnail<Impl: IStorageItemInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Thumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThumbnailUpdated<Impl: IStorageItemInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThumbnailUpdated(&*(&changedhandler as *const <super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveThumbnailUpdated<Impl: IStorageItemInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveThumbnailUpdated(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PropertiesUpdated<Impl: IStorageItemInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertiesUpdated(&*(&changedhandler as *const <super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePropertiesUpdated<Impl: IStorageItemInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePropertiesUpdated(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStorageItemInformation>,
            ::windows::core::GetTrustLevel,
            MusicProperties::<Impl, OFFSET>,
            VideoProperties::<Impl, OFFSET>,
            ImageProperties::<Impl, OFFSET>,
            DocumentProperties::<Impl, OFFSET>,
            BasicProperties::<Impl, OFFSET>,
            Thumbnail::<Impl, OFFSET>,
            ThumbnailUpdated::<Impl, OFFSET>,
            RemoveThumbnailUpdated::<Impl, OFFSET>,
            PropertiesUpdated::<Impl, OFFSET>,
            RemovePropertiesUpdated::<Impl, OFFSET>,
        )
    }
}
