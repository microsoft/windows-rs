#[cfg(feature = "Foundation_Collections")]
pub trait IStorageItemAccessList_Impl: Sized {
    fn AddOverloadDefaultMetadata(&self, file: &::core::option::Option<super::IStorageItem>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Add(&self, file: &::core::option::Option<super::IStorageItem>, metadata: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AddOrReplaceOverloadDefaultMetadata(&self, token: &::windows::core::HSTRING, file: &::core::option::Option<super::IStorageItem>) -> ::windows::core::Result<()>;
    fn AddOrReplace(&self, token: &::windows::core::HSTRING, file: &::core::option::Option<super::IStorageItem>, metadata: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetItemAsync(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>>;
    fn GetFileAsync(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
    fn GetFolderAsync(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>>;
    fn GetItemWithOptionsAsync(&self, token: &::windows::core::HSTRING, options: AccessCacheOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>>;
    fn GetFileWithOptionsAsync(&self, token: &::windows::core::HSTRING, options: AccessCacheOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
    fn GetFolderWithOptionsAsync(&self, token: &::windows::core::HSTRING, options: AccessCacheOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>>;
    fn Remove(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContainsItem(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn CheckAccess(&self, file: &::core::option::Option<super::IStorageItem>) -> ::windows::core::Result<bool>;
    fn Entries(&self) -> ::windows::core::Result<AccessListEntryView>;
    fn MaximumItemsAllowed(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IStorageItemAccessList {
    const NAME: &'static str = "Windows.Storage.AccessCache.IStorageItemAccessList";
}
#[cfg(feature = "Foundation_Collections")]
impl IStorageItemAccessList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>() -> IStorageItemAccessList_Vtbl {
        unsafe extern "system" fn AddOverloadDefaultMetadata<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddOverloadDefaultMetadata(::core::mem::transmute(&file)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, metadata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add(::core::mem::transmute(&file), ::core::mem::transmute(&metadata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOrReplaceOverloadDefaultMetadata<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, file: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddOrReplaceOverloadDefaultMetadata(::core::mem::transmute(&token), ::core::mem::transmute(&file)).into()
        }
        unsafe extern "system" fn AddOrReplace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, file: *mut ::core::ffi::c_void, metadata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddOrReplace(::core::mem::transmute(&token), ::core::mem::transmute(&file), ::core::mem::transmute(&metadata)).into()
        }
        unsafe extern "system" fn GetItemAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemAsync(::core::mem::transmute(&token)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileAsync(::core::mem::transmute(&token)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolderAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFolderAsync(::core::mem::transmute(&token)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemWithOptionsAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: AccessCacheOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemWithOptionsAsync(::core::mem::transmute(&token), options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileWithOptionsAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: AccessCacheOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileWithOptionsAsync(::core::mem::transmute(&token), options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolderWithOptionsAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: AccessCacheOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFolderWithOptionsAsync(::core::mem::transmute(&token), options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn ContainsItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContainsItem(::core::mem::transmute(&token)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        unsafe extern "system" fn CheckAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CheckAccess(::core::mem::transmute(&file)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Entries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Entries() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumItemsAllowed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaximumItemsAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IStorageItemAccessList, OFFSET>(),
            AddOverloadDefaultMetadata: AddOverloadDefaultMetadata::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            AddOrReplaceOverloadDefaultMetadata: AddOrReplaceOverloadDefaultMetadata::<Identity, Impl, OFFSET>,
            AddOrReplace: AddOrReplace::<Identity, Impl, OFFSET>,
            GetItemAsync: GetItemAsync::<Identity, Impl, OFFSET>,
            GetFileAsync: GetFileAsync::<Identity, Impl, OFFSET>,
            GetFolderAsync: GetFolderAsync::<Identity, Impl, OFFSET>,
            GetItemWithOptionsAsync: GetItemWithOptionsAsync::<Identity, Impl, OFFSET>,
            GetFileWithOptionsAsync: GetFileWithOptionsAsync::<Identity, Impl, OFFSET>,
            GetFolderWithOptionsAsync: GetFolderWithOptionsAsync::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            ContainsItem: ContainsItem::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            CheckAccess: CheckAccess::<Identity, Impl, OFFSET>,
            Entries: Entries::<Identity, Impl, OFFSET>,
            MaximumItemsAllowed: MaximumItemsAllowed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItemAccessList as ::windows::core::Interface>::IID
    }
}
