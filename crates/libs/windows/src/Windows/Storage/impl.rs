#[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageFile_Impl: Sized + Streams::IInputStreamReference_Impl + Streams::IRandomAccessStreamReference_Impl + IStorageItem_Impl {
    fn FileType(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn OpenAsync(&self, accessmode: FileAccessMode) -> ::windows_core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>;
    fn OpenTransactedWriteAsync(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>>;
    fn CopyOverloadDefaultNameAndOptions(&self, destinationfolder: ::core::option::Option<&IStorageFolder>) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyOverloadDefaultOptions(&self, destinationfolder: ::core::option::Option<&IStorageFolder>, desirednewname: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyOverload(&self, destinationfolder: ::core::option::Option<&IStorageFolder>, desirednewname: &::windows_core::HSTRING, option: NameCollisionOption) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyAndReplaceAsync(&self, filetoreplace: ::core::option::Option<&IStorageFile>) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverloadDefaultNameAndOptions(&self, destinationfolder: ::core::option::Option<&IStorageFolder>) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverloadDefaultOptions(&self, destinationfolder: ::core::option::Option<&IStorageFolder>, desirednewname: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverload(&self, destinationfolder: ::core::option::Option<&IStorageFolder>, desirednewname: &::windows_core::HSTRING, option: NameCollisionOption) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
    fn MoveAndReplaceAsync(&self, filetoreplace: ::core::option::Option<&IStorageFile>) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl ::windows_core::RuntimeName for IStorageFile {
    const NAME: &'static str = "Windows.Storage.IStorageFile";
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl IStorageFile_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: isize>() -> IStorageFile_Vtbl {
        unsafe extern "system" fn FileType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FileType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContentType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessmode: FileAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenAsync(accessmode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTransactedWriteAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenTransactedWriteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyOverloadDefaultNameAndOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CopyOverloadDefaultNameAndOptions(::windows_core::from_raw_borrowed(&destinationfolder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyOverloadDefaultOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: *mut ::core::ffi::c_void, desirednewname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CopyOverloadDefaultOptions(::windows_core::from_raw_borrowed(&destinationfolder), ::core::mem::transmute(&desirednewname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyOverload<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: *mut ::core::ffi::c_void, desirednewname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, option: NameCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CopyOverload(::windows_core::from_raw_borrowed(&destinationfolder), ::core::mem::transmute(&desirednewname), option) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyAndReplaceAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetoreplace: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CopyAndReplaceAsync(::windows_core::from_raw_borrowed(&filetoreplace)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveOverloadDefaultNameAndOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveOverloadDefaultNameAndOptions(::windows_core::from_raw_borrowed(&destinationfolder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveOverloadDefaultOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: *mut ::core::ffi::c_void, desirednewname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveOverloadDefaultOptions(::windows_core::from_raw_borrowed(&destinationfolder), ::core::mem::transmute(&desirednewname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveOverload<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: *mut ::core::ffi::c_void, desirednewname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, option: NameCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveOverload(::windows_core::from_raw_borrowed(&destinationfolder), ::core::mem::transmute(&desirednewname), option) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveAndReplaceAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetoreplace: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveAndReplaceAsync(::windows_core::from_raw_borrowed(&filetoreplace)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IStorageFile, OFFSET>(),
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
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IStorageFile as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IStorageFile2_Impl: Sized {
    fn OpenWithOptionsAsync(&self, accessmode: FileAccessMode, options: StorageOpenOptions) -> ::windows_core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>;
    fn OpenTransactedWriteWithOptionsAsync(&self, options: StorageOpenOptions) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::RuntimeName for IStorageFile2 {
    const NAME: &'static str = "Windows.Storage.IStorageFile2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IStorageFile2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFile2_Impl, const OFFSET: isize>() -> IStorageFile2_Vtbl {
        unsafe extern "system" fn OpenWithOptionsAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessmode: FileAccessMode, options: StorageOpenOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenWithOptionsAsync(accessmode, options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTransactedWriteWithOptionsAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: StorageOpenOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenTransactedWriteWithOptionsAsync(options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IStorageFile2, OFFSET>(),
            OpenWithOptionsAsync: OpenWithOptionsAsync::<Identity, Impl, OFFSET>,
            OpenTransactedWriteWithOptionsAsync: OpenTransactedWriteWithOptionsAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IStorageFile2 as ::windows_core::ComInterface>::IID
    }
}
pub trait IStorageFilePropertiesWithAvailability_Impl: Sized {
    fn IsAvailable(&self) -> ::windows_core::Result<bool>;
}
impl ::windows_core::RuntimeName for IStorageFilePropertiesWithAvailability {
    const NAME: &'static str = "Windows.Storage.IStorageFilePropertiesWithAvailability";
}
impl IStorageFilePropertiesWithAvailability_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFilePropertiesWithAvailability_Impl, const OFFSET: isize>() -> IStorageFilePropertiesWithAvailability_Vtbl {
        unsafe extern "system" fn IsAvailable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFilePropertiesWithAvailability_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IStorageFilePropertiesWithAvailability, OFFSET>(),
            IsAvailable: IsAvailable::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IStorageFilePropertiesWithAvailability as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage_FileProperties\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties"))]
pub trait IStorageFolder_Impl: Sized + IStorageItem_Impl {
    fn CreateFileAsyncOverloadDefaultOptions(&self, desiredname: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFileAsync(&self, desiredname: &::windows_core::HSTRING, options: CreationCollisionOption) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderAsyncOverloadDefaultOptions(&self, desiredname: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn CreateFolderAsync(&self, desiredname: &::windows_core::HSTRING, options: CreationCollisionOption) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn GetFileAsync(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn GetFolderAsync(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn GetItemAsync(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<IStorageItem>>;
    fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>>;
    fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>>;
    fn GetItemsAsyncOverloadDefaultStartAndCount(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties"))]
impl ::windows_core::RuntimeName for IStorageFolder {
    const NAME: &'static str = "Windows.Storage.IStorageFolder";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties"))]
impl IStorageFolder_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: isize>() -> IStorageFolder_Vtbl {
        unsafe extern "system" fn CreateFileAsyncOverloadDefaultOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFileAsyncOverloadDefaultOptions(::core::mem::transmute(&desiredname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: CreationCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFileAsync(::core::mem::transmute(&desiredname), options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderAsyncOverloadDefaultOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFolderAsyncOverloadDefaultOptions(::core::mem::transmute(&desiredname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: CreationCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFolderAsync(::core::mem::transmute(&desiredname), options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileAsync(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolderAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFolderAsync(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemAsync(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilesAsyncOverloadDefaultOptionsStartAndCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFilesAsyncOverloadDefaultOptionsStartAndCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFoldersAsyncOverloadDefaultOptionsStartAndCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsAsyncOverloadDefaultStartAndCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemsAsyncOverloadDefaultStartAndCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IStorageFolder, OFFSET>(),
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
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IStorageFolder as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IStorageFolder2_Impl: Sized {
    fn TryGetItemAsync(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<IStorageItem>>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for IStorageFolder2 {
    const NAME: &'static str = "Windows.Storage.IStorageFolder2";
}
#[cfg(feature = "Foundation")]
impl IStorageFolder2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolder2_Impl, const OFFSET: isize>() -> IStorageFolder2_Vtbl {
        unsafe extern "system" fn TryGetItemAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryGetItemAsync(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IStorageFolder2, OFFSET>(),
            TryGetItemAsync: TryGetItemAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IStorageFolder2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
pub trait IStorageItem_Impl: Sized {
    fn RenameAsyncOverloadDefaultOptions(&self, desiredname: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
    fn RenameAsync(&self, desiredname: &::windows_core::HSTRING, option: NameCollisionOption) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
    fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
    fn DeleteAsync(&self, option: StorageDeleteOption) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
    fn GetBasicPropertiesAsync(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>;
    fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Path(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Attributes(&self) -> ::windows_core::Result<FileAttributes>;
    fn DateCreated(&self) -> ::windows_core::Result<super::Foundation::DateTime>;
    fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows_core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
impl ::windows_core::RuntimeName for IStorageItem {
    const NAME: &'static str = "Windows.Storage.IStorageItem";
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
impl IStorageItem_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: isize>() -> IStorageItem_Vtbl {
        unsafe extern "system" fn RenameAsyncOverloadDefaultOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RenameAsyncOverloadDefaultOptions(::core::mem::transmute(&desiredname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenameAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, option: NameCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RenameAsync(::core::mem::transmute(&desiredname), option) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsyncOverloadDefaultOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeleteAsyncOverloadDefaultOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: StorageDeleteOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeleteAsync(option) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBasicPropertiesAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBasicPropertiesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FileAttributes) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DateCreated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DateCreated() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOfType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: StorageItemTypes, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOfType(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IStorageItem, OFFSET>(),
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
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IStorageItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
pub trait IStorageItem2_Impl: Sized + IStorageItem_Impl {
    fn GetParentAsync(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn IsEqual(&self, item: ::core::option::Option<&IStorageItem>) -> ::windows_core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
impl ::windows_core::RuntimeName for IStorageItem2 {
    const NAME: &'static str = "Windows.Storage.IStorageItem2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
impl IStorageItem2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItem2_Impl, const OFFSET: isize>() -> IStorageItem2_Vtbl {
        unsafe extern "system" fn GetParentAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParentAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEqual(::windows_core::from_raw_borrowed(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IStorageItem2, OFFSET>(),
            GetParentAsync: GetParentAsync::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IStorageItem2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageItemProperties_Impl: Sized {
    fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows_core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows_core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows_core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn DisplayType(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn FolderRelativeId(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Properties(&self) -> ::windows_core::Result<FileProperties::StorageItemContentProperties>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl ::windows_core::RuntimeName for IStorageItemProperties {
    const NAME: &'static str = "Windows.Storage.IStorageItemProperties";
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl IStorageItemProperties_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemProperties_Impl, const OFFSET: isize>() -> IStorageItemProperties_Vtbl {
        unsafe extern "system" fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(mode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThumbnailAsyncOverloadDefaultOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThumbnailAsyncOverloadDefaultOptions(mode, requestedsize) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThumbnailAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThumbnailAsync(mode, requestedsize, options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FolderRelativeId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FolderRelativeId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IStorageItemProperties, OFFSET>(),
            GetThumbnailAsyncOverloadDefaultSizeDefaultOptions: GetThumbnailAsyncOverloadDefaultSizeDefaultOptions::<Identity, Impl, OFFSET>,
            GetThumbnailAsyncOverloadDefaultOptions: GetThumbnailAsyncOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            GetThumbnailAsync: GetThumbnailAsync::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            DisplayType: DisplayType::<Identity, Impl, OFFSET>,
            FolderRelativeId: FolderRelativeId::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IStorageItemProperties as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageItemProperties2_Impl: Sized + IStorageItemProperties_Impl {
    fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows_core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows_core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetScaledImageAsThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows_core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl ::windows_core::RuntimeName for IStorageItemProperties2 {
    const NAME: &'static str = "Windows.Storage.IStorageItemProperties2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl IStorageItemProperties2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemProperties2_Impl, const OFFSET: isize>() -> IStorageItemProperties2_Vtbl {
        unsafe extern "system" fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(mode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(mode, requestedsize) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScaledImageAsThumbnailAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScaledImageAsThumbnailAsync(mode, requestedsize, options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IStorageItemProperties2, OFFSET>(),
            GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions: GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions::<Identity, Impl, OFFSET>,
            GetScaledImageAsThumbnailAsyncOverloadDefaultOptions: GetScaledImageAsThumbnailAsyncOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            GetScaledImageAsThumbnailAsync: GetScaledImageAsThumbnailAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IStorageItemProperties2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageItemPropertiesWithProvider_Impl: Sized + IStorageItemProperties_Impl {
    fn Provider(&self) -> ::windows_core::Result<StorageProvider>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl ::windows_core::RuntimeName for IStorageItemPropertiesWithProvider {
    const NAME: &'static str = "Windows.Storage.IStorageItemPropertiesWithProvider";
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl IStorageItemPropertiesWithProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemPropertiesWithProvider_Impl, const OFFSET: isize>() -> IStorageItemPropertiesWithProvider_Vtbl {
        unsafe extern "system" fn Provider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemPropertiesWithProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Provider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IStorageItemPropertiesWithProvider, OFFSET>(),
            Provider: Provider::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IStorageItemPropertiesWithProvider as ::windows_core::ComInterface>::IID
    }
}
pub trait IStreamedFileDataRequest_Impl: Sized {
    fn FailAndClose(&self, failuremode: StreamedFileFailureMode) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IStreamedFileDataRequest {
    const NAME: &'static str = "Windows.Storage.IStreamedFileDataRequest";
}
impl IStreamedFileDataRequest_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStreamedFileDataRequest_Impl, const OFFSET: isize>() -> IStreamedFileDataRequest_Vtbl {
        unsafe extern "system" fn FailAndClose<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStreamedFileDataRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, failuremode: StreamedFileFailureMode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FailAndClose(failuremode).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IStreamedFileDataRequest, OFFSET>(),
            FailAndClose: FailAndClose::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IStreamedFileDataRequest as ::windows_core::ComInterface>::IID
    }
}
