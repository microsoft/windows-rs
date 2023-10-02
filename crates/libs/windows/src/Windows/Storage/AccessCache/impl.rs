#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IStorageItemAccessList_Impl: Sized {
    fn AddOverloadDefaultMetadata(&self, file: ::core::option::Option<&super::IStorageItem>) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Add(&self, file: ::core::option::Option<&super::IStorageItem>, metadata: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn AddOrReplaceOverloadDefaultMetadata(&self, token: &::windows_core::HSTRING, file: ::core::option::Option<&super::IStorageItem>) -> ::windows_core::Result<()>;
    fn AddOrReplace(&self, token: &::windows_core::HSTRING, file: ::core::option::Option<&super::IStorageItem>, metadata: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn GetItemAsync(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>>;
    fn GetFileAsync(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
    fn GetFolderAsync(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>>;
    fn GetItemWithOptionsAsync(&self, token: &::windows_core::HSTRING, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>>;
    fn GetFileWithOptionsAsync(&self, token: &::windows_core::HSTRING, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
    fn GetFolderWithOptionsAsync(&self, token: &::windows_core::HSTRING, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>>;
    fn Remove(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn ContainsItem(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<bool>;
    fn Clear(&self) -> ::windows_core::Result<()>;
    fn CheckAccess(&self, file: ::core::option::Option<&super::IStorageItem>) -> ::windows_core::Result<bool>;
    fn Entries(&self) -> ::windows_core::Result<AccessListEntryView>;
    fn MaximumItemsAllowed(&self) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for IStorageItemAccessList {
    const NAME: &'static str = "Windows.Storage.AccessCache.IStorageItemAccessList";
}
#[cfg(feature = "Foundation_Collections")]
impl IStorageItemAccessList_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>() -> IStorageItemAccessList_Vtbl {
        unsafe extern "system" fn AddOverloadDefaultMetadata<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddOverloadDefaultMetadata(::windows_core::from_raw_borrowed(&file)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, metadata: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add(::windows_core::from_raw_borrowed(&file), ::core::mem::transmute(&metadata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOrReplaceOverloadDefaultMetadata<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, file: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddOrReplaceOverloadDefaultMetadata(::core::mem::transmute(&token), ::windows_core::from_raw_borrowed(&file)).into()
        }
        unsafe extern "system" fn AddOrReplace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, file: *mut ::core::ffi::c_void, metadata: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddOrReplace(::core::mem::transmute(&token), ::windows_core::from_raw_borrowed(&file), ::core::mem::transmute(&metadata)).into()
        }
        unsafe extern "system" fn GetItemAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemAsync(::core::mem::transmute(&token)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileAsync(::core::mem::transmute(&token)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolderAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFolderAsync(::core::mem::transmute(&token)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemWithOptionsAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: AccessCacheOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemWithOptionsAsync(::core::mem::transmute(&token), options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileWithOptionsAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: AccessCacheOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileWithOptionsAsync(::core::mem::transmute(&token), options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolderWithOptionsAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: AccessCacheOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFolderWithOptionsAsync(::core::mem::transmute(&token), options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn ContainsItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContainsItem(::core::mem::transmute(&token)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        unsafe extern "system" fn CheckAccess<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CheckAccess(::windows_core::from_raw_borrowed(&file)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Entries<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Entries() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumItemsAllowed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaximumItemsAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IStorageItemAccessList, OFFSET>(),
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
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IStorageItemAccessList as ::windows_core::ComInterface>::IID
    }
}
