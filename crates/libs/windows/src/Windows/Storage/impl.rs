#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageFile_Impl: Sized + Streams::IInputStreamReference_Impl + Streams::IRandomAccessStreamReference_Impl + IStorageItem_Impl {
    fn FileType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OpenAsync(&self, accessmode: FileAccessMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>;
    fn OpenTransactedWriteAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>>;
    fn CopyOverloadDefaultNameAndOptions(&self, destinationfolder: &::core::option::Option<IStorageFolder>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyOverloadDefaultOptions(&self, destinationfolder: &::core::option::Option<IStorageFolder>, desirednewname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyOverload(&self, destinationfolder: &::core::option::Option<IStorageFolder>, desirednewname: &::windows::core::HSTRING, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyAndReplaceAsync(&self, filetoreplace: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverloadDefaultNameAndOptions(&self, destinationfolder: &::core::option::Option<IStorageFolder>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverloadDefaultOptions(&self, destinationfolder: &::core::option::Option<IStorageFolder>, desirednewname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverload(&self, destinationfolder: &::core::option::Option<IStorageFolder>, desirednewname: &::windows::core::HSTRING, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn MoveAndReplaceAsync(&self, filetoreplace: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IStorageFile {
    const NAME: &'static str = "Windows.Storage.IStorageFile";
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl IStorageFile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile_Impl, const OFFSET: isize>() -> IStorageFile_Vtbl {
        unsafe extern "system" fn FileType<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FileType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentType<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessmode: FileAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenAsync(accessmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTransactedWriteAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenTransactedWriteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyOverloadDefaultNameAndOptions<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CopyOverloadDefaultNameAndOptions(::core::mem::transmute(&destinationfolder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyOverloadDefaultOptions<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, desirednewname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CopyOverloadDefaultOptions(::core::mem::transmute(&destinationfolder), ::core::mem::transmute(&desirednewname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyOverload<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, desirednewname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: NameCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CopyOverload(::core::mem::transmute(&destinationfolder), ::core::mem::transmute(&desirednewname), option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyAndReplaceAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetoreplace: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CopyAndReplaceAsync(::core::mem::transmute(&filetoreplace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveOverloadDefaultNameAndOptions<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveOverloadDefaultNameAndOptions(::core::mem::transmute(&destinationfolder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveOverloadDefaultOptions<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, desirednewname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveOverloadDefaultOptions(::core::mem::transmute(&destinationfolder), ::core::mem::transmute(&desirednewname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveOverload<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, desirednewname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: NameCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveOverload(::core::mem::transmute(&destinationfolder), ::core::mem::transmute(&desirednewname), option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveAndReplaceAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetoreplace: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveAndReplaceAsync(::core::mem::transmute(&filetoreplace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFile, OFFSET>(),
            FileType: FileType::<Identity, Impl, OFFSET>,
            ContentType: ContentType::<Identity, Impl, OFFSET>,
            OpenAsync: OpenAsync::<Identity, Impl, OFFSET>,
            OpenTransactedWriteAsync: OpenTransactedWriteAsync::<Identity, Impl, OFFSET>,
            CopyOverloadDefaultNameAndOptions: CopyOverloadDefaultNameAndOptions::<Identity, Impl, OFFSET>,
            CopyOverloadDefaultOptions: CopyOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            CopyOverload: CopyOverload::<Identity, Impl, OFFSET>,
            CopyAndReplaceAsync: CopyAndReplaceAsync::<Identity, Impl, OFFSET>,
            MoveOverloadDefaultNameAndOptions: MoveOverloadDefaultNameAndOptions::<Identity, Impl, OFFSET>,
            MoveOverloadDefaultOptions: MoveOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            MoveOverload: MoveOverload::<Identity, Impl, OFFSET>,
            MoveAndReplaceAsync: MoveAndReplaceAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IStorageFile2_Impl: Sized {
    fn OpenWithOptionsAsync(&self, accessmode: FileAccessMode, options: StorageOpenOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>;
    fn OpenTransactedWriteWithOptionsAsync(&self, options: StorageOpenOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IStorageFile2 {
    const NAME: &'static str = "Windows.Storage.IStorageFile2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IStorageFile2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile2_Impl, const OFFSET: isize>() -> IStorageFile2_Vtbl {
        unsafe extern "system" fn OpenWithOptionsAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessmode: FileAccessMode, options: StorageOpenOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenWithOptionsAsync(accessmode, options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTransactedWriteWithOptionsAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: StorageOpenOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenTransactedWriteWithOptionsAsync(options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFile2, OFFSET>(),
            OpenWithOptionsAsync: OpenWithOptionsAsync::<Identity, Impl, OFFSET>,
            OpenTransactedWriteWithOptionsAsync: OpenTransactedWriteWithOptionsAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFile2 as ::windows::core::Interface>::IID
    }
}
pub trait IStorageFilePropertiesWithAvailability_Impl: Sized {
    fn IsAvailable(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IStorageFilePropertiesWithAvailability {
    const NAME: &'static str = "Windows.Storage.IStorageFilePropertiesWithAvailability";
}
impl IStorageFilePropertiesWithAvailability_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFilePropertiesWithAvailability_Impl, const OFFSET: isize>() -> IStorageFilePropertiesWithAvailability_Vtbl {
        unsafe extern "system" fn IsAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFilePropertiesWithAvailability_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFilePropertiesWithAvailability, OFFSET>(),
            IsAvailable: IsAvailable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFilePropertiesWithAvailability as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties"))]
pub trait IStorageFolder_Impl: Sized + IStorageItem_Impl {
    fn CreateFileAsyncOverloadDefaultOptions(&self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFileAsync(&self, desiredname: &::windows::core::HSTRING, options: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderAsyncOverloadDefaultOptions(&self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn CreateFolderAsync(&self, desiredname: &::windows::core::HSTRING, options: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn GetFileAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn GetFolderAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn GetItemAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>>;
    fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>>;
    fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>>;
    fn GetItemsAsyncOverloadDefaultStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties"))]
impl ::windows::core::RuntimeName for IStorageFolder {
    const NAME: &'static str = "Windows.Storage.IStorageFolder";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties"))]
impl IStorageFolder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolder_Impl, const OFFSET: isize>() -> IStorageFolder_Vtbl {
        unsafe extern "system" fn CreateFileAsyncOverloadDefaultOptions<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFileAsyncOverloadDefaultOptions(::core::mem::transmute(&desiredname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFileAsync(::core::mem::transmute(&desiredname), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderAsyncOverloadDefaultOptions<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFolderAsyncOverloadDefaultOptions(::core::mem::transmute(&desiredname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFolderAsync(::core::mem::transmute(&desiredname), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFileAsync(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolderAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFolderAsync(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItemAsync(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilesAsyncOverloadDefaultOptionsStartAndCount<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFilesAsyncOverloadDefaultOptionsStartAndCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFoldersAsyncOverloadDefaultOptionsStartAndCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsAsyncOverloadDefaultStartAndCount<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItemsAsyncOverloadDefaultStartAndCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFolder, OFFSET>(),
            CreateFileAsyncOverloadDefaultOptions: CreateFileAsyncOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            CreateFileAsync: CreateFileAsync::<Identity, Impl, OFFSET>,
            CreateFolderAsyncOverloadDefaultOptions: CreateFolderAsyncOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            CreateFolderAsync: CreateFolderAsync::<Identity, Impl, OFFSET>,
            GetFileAsync: GetFileAsync::<Identity, Impl, OFFSET>,
            GetFolderAsync: GetFolderAsync::<Identity, Impl, OFFSET>,
            GetItemAsync: GetItemAsync::<Identity, Impl, OFFSET>,
            GetFilesAsyncOverloadDefaultOptionsStartAndCount: GetFilesAsyncOverloadDefaultOptionsStartAndCount::<Identity, Impl, OFFSET>,
            GetFoldersAsyncOverloadDefaultOptionsStartAndCount: GetFoldersAsyncOverloadDefaultOptionsStartAndCount::<Identity, Impl, OFFSET>,
            GetItemsAsyncOverloadDefaultStartAndCount: GetItemsAsyncOverloadDefaultStartAndCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFolder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IStorageFolder2_Impl: Sized {
    fn TryGetItemAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IStorageFolder2 {
    const NAME: &'static str = "Windows.Storage.IStorageFolder2";
}
#[cfg(feature = "Foundation")]
impl IStorageFolder2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolder2_Impl, const OFFSET: isize>() -> IStorageFolder2_Vtbl {
        unsafe extern "system" fn TryGetItemAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TryGetItemAsync(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFolder2, OFFSET>(), TryGetItemAsync: TryGetItemAsync::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFolder2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
pub trait IStorageItem_Impl: Sized {
    fn RenameAsyncOverloadDefaultOptions(&self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn RenameAsync(&self, desiredname: &::windows::core::HSTRING, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn DeleteAsync(&self, option: StorageDeleteOption) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Attributes(&self) -> ::windows::core::Result<FileAttributes>;
    fn DateCreated(&self) -> ::windows::core::Result<super::Foundation::DateTime>;
    fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
impl ::windows::core::RuntimeName for IStorageItem {
    const NAME: &'static str = "Windows.Storage.IStorageItem";
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
impl IStorageItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItem_Impl, const OFFSET: isize>() -> IStorageItem_Vtbl {
        unsafe extern "system" fn RenameAsyncOverloadDefaultOptions<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RenameAsyncOverloadDefaultOptions(::core::mem::transmute(&desiredname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenameAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: NameCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RenameAsync(::core::mem::transmute(&desiredname), option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsyncOverloadDefaultOptions<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeleteAsyncOverloadDefaultOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: StorageDeleteOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeleteAsync(option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBasicPropertiesAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBasicPropertiesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FileAttributes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DateCreated<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DateCreated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOfType<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: StorageItemTypes, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOfType(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageItem, OFFSET>(),
            RenameAsyncOverloadDefaultOptions: RenameAsyncOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            RenameAsync: RenameAsync::<Identity, Impl, OFFSET>,
            DeleteAsyncOverloadDefaultOptions: DeleteAsyncOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            DeleteAsync: DeleteAsync::<Identity, Impl, OFFSET>,
            GetBasicPropertiesAsync: GetBasicPropertiesAsync::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            Attributes: Attributes::<Identity, Impl, OFFSET>,
            DateCreated: DateCreated::<Identity, Impl, OFFSET>,
            IsOfType: IsOfType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
pub trait IStorageItem2_Impl: Sized + IStorageItem_Impl {
    fn GetParentAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn IsEqual(&self, item: &::core::option::Option<IStorageItem>) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
impl ::windows::core::RuntimeName for IStorageItem2 {
    const NAME: &'static str = "Windows.Storage.IStorageItem2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
impl IStorageItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItem2_Impl, const OFFSET: isize>() -> IStorageItem2_Vtbl {
        unsafe extern "system" fn GetParentAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetParentAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsEqual(::core::mem::transmute(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageItem2, OFFSET>(),
            GetParentAsync: GetParentAsync::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageItemProperties_Impl: Sized {
    fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FolderRelativeId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<FileProperties::StorageItemContentProperties>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IStorageItemProperties {
    const NAME: &'static str = "Windows.Storage.IStorageItemProperties";
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl IStorageItemProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemProperties_Impl, const OFFSET: isize>() -> IStorageItemProperties_Vtbl {
        unsafe extern "system" fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThumbnailAsyncOverloadDefaultOptions<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetThumbnailAsyncOverloadDefaultOptions(mode, requestedsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThumbnailAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetThumbnailAsync(mode, requestedsize, options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayType<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FolderRelativeId<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FolderRelativeId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageItemProperties, OFFSET>(),
            GetThumbnailAsyncOverloadDefaultSizeDefaultOptions: GetThumbnailAsyncOverloadDefaultSizeDefaultOptions::<Identity, Impl, OFFSET>,
            GetThumbnailAsyncOverloadDefaultOptions: GetThumbnailAsyncOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            GetThumbnailAsync: GetThumbnailAsync::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            DisplayType: DisplayType::<Identity, Impl, OFFSET>,
            FolderRelativeId: FolderRelativeId::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItemProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageItemProperties2_Impl: Sized + IStorageItemProperties_Impl {
    fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetScaledImageAsThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IStorageItemProperties2 {
    const NAME: &'static str = "Windows.Storage.IStorageItemProperties2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl IStorageItemProperties2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemProperties2_Impl, const OFFSET: isize>() -> IStorageItemProperties2_Vtbl {
        unsafe extern "system" fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(mode, requestedsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScaledImageAsThumbnailAsync<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetScaledImageAsThumbnailAsync(mode, requestedsize, options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageItemProperties2, OFFSET>(),
            GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions: GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions::<Identity, Impl, OFFSET>,
            GetScaledImageAsThumbnailAsyncOverloadDefaultOptions: GetScaledImageAsThumbnailAsyncOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            GetScaledImageAsThumbnailAsync: GetScaledImageAsThumbnailAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItemProperties2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageItemPropertiesWithProvider_Impl: Sized + IStorageItemProperties_Impl {
    fn Provider(&self) -> ::windows::core::Result<StorageProvider>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IStorageItemPropertiesWithProvider {
    const NAME: &'static str = "Windows.Storage.IStorageItemPropertiesWithProvider";
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl IStorageItemPropertiesWithProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemPropertiesWithProvider_Impl, const OFFSET: isize>() -> IStorageItemPropertiesWithProvider_Vtbl {
        unsafe extern "system" fn Provider<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemPropertiesWithProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Provider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageItemPropertiesWithProvider, OFFSET>(),
            Provider: Provider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItemPropertiesWithProvider as ::windows::core::Interface>::IID
    }
}
pub trait IStreamedFileDataRequest_Impl: Sized {
    fn FailAndClose(&self, failuremode: StreamedFileFailureMode) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IStreamedFileDataRequest {
    const NAME: &'static str = "Windows.Storage.IStreamedFileDataRequest";
}
impl IStreamedFileDataRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamedFileDataRequest_Impl, const OFFSET: isize>() -> IStreamedFileDataRequest_Vtbl {
        unsafe extern "system" fn FailAndClose<Identity: ::windows::core::IUnknownImpl, Impl: IStreamedFileDataRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, failuremode: StreamedFileFailureMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FailAndClose(failuremode).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamedFileDataRequest, OFFSET>(),
            FailAndClose: FailAndClose::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamedFileDataRequest as ::windows::core::Interface>::IID
    }
}
