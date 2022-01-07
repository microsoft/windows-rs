#[cfg(feature = "implement_exclusive")]
pub trait IItemRemovedEventArgsImpl: Sized {
    fn RemovedEntry(&self) -> ::windows::core::Result<AccessListEntry>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemRemovedEventArgs {
    const NAME: &'static str = "Windows.Storage.AccessCache.IItemRemovedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IItemRemovedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemRemovedEventArgsImpl, const OFFSET: isize>() -> IItemRemovedEventArgsVtbl {
        unsafe extern "system" fn RemovedEntry<Impl: IItemRemovedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<AccessListEntry>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovedEntry() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IItemRemovedEventArgs>, ::windows::core::GetTrustLevel, RemovedEntry::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageApplicationPermissionsStaticsImpl: Sized {
    fn FutureAccessList(&self) -> ::windows::core::Result<StorageItemAccessList>;
    fn MostRecentlyUsedList(&self) -> ::windows::core::Result<StorageItemMostRecentlyUsedList>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageApplicationPermissionsStatics {
    const NAME: &'static str = "Windows.Storage.AccessCache.IStorageApplicationPermissionsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageApplicationPermissionsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageApplicationPermissionsStaticsImpl, const OFFSET: isize>() -> IStorageApplicationPermissionsStaticsVtbl {
        unsafe extern "system" fn FutureAccessList<Impl: IStorageApplicationPermissionsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FutureAccessList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MostRecentlyUsedList<Impl: IStorageApplicationPermissionsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MostRecentlyUsedList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageApplicationPermissionsStatics>, ::windows::core::GetTrustLevel, FutureAccessList::<Impl, OFFSET>, MostRecentlyUsedList::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageApplicationPermissionsStatics2Impl: Sized {
    fn GetFutureAccessListForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<StorageItemAccessList>;
    fn GetMostRecentlyUsedListForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<StorageItemMostRecentlyUsedList>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageApplicationPermissionsStatics2 {
    const NAME: &'static str = "Windows.Storage.AccessCache.IStorageApplicationPermissionsStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageApplicationPermissionsStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageApplicationPermissionsStatics2Impl, const OFFSET: isize>() -> IStorageApplicationPermissionsStatics2Vtbl {
        unsafe extern "system" fn GetFutureAccessListForUser<Impl: IStorageApplicationPermissionsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFutureAccessListForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMostRecentlyUsedListForUser<Impl: IStorageApplicationPermissionsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMostRecentlyUsedListForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageApplicationPermissionsStatics2>, ::windows::core::GetTrustLevel, GetFutureAccessListForUser::<Impl, OFFSET>, GetMostRecentlyUsedListForUser::<Impl, OFFSET>)
    }
}
pub trait IStorageItemAccessListImpl: Sized {
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
impl ::windows::core::RuntimeName for IStorageItemAccessList {
    const NAME: &'static str = "Windows.Storage.AccessCache.IStorageItemAccessList";
}
impl IStorageItemAccessListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemAccessListImpl, const OFFSET: isize>() -> IStorageItemAccessListVtbl {
        unsafe extern "system" fn AddOverloadDefaultMetadata<Impl: IStorageItemAccessListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddOverloadDefaultMetadata(&*(&file as *const <super::IStorageItem as ::windows::core::Abi>::Abi as *const <super::IStorageItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IStorageItemAccessListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, metadata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&file as *const <super::IStorageItem as ::windows::core::Abi>::Abi as *const <super::IStorageItem as ::windows::core::DefaultType>::DefaultType), &*(&metadata as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOrReplaceOverloadDefaultMetadata<Impl: IStorageItemAccessListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, file: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddOrReplaceOverloadDefaultMetadata(&*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&file as *const <super::IStorageItem as ::windows::core::Abi>::Abi as *const <super::IStorageItem as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddOrReplace<Impl: IStorageItemAccessListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, file: ::windows::core::RawPtr, metadata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .AddOrReplace(&*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&file as *const <super::IStorageItem as ::windows::core::Abi>::Abi as *const <super::IStorageItem as ::windows::core::DefaultType>::DefaultType), &*(&metadata as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn GetItemAsync<Impl: IStorageItemAccessListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemAsync(&*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileAsync<Impl: IStorageItemAccessListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileAsync(&*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolderAsync<Impl: IStorageItemAccessListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFolderAsync(&*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemWithOptionsAsync<Impl: IStorageItemAccessListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: AccessCacheOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemWithOptionsAsync(&*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileWithOptionsAsync<Impl: IStorageItemAccessListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: AccessCacheOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileWithOptionsAsync(&*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolderWithOptionsAsync<Impl: IStorageItemAccessListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: AccessCacheOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFolderWithOptionsAsync(&*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IStorageItemAccessListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContainsItem<Impl: IStorageItemAccessListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainsItem(&*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IStorageItemAccessListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn CheckAccess<Impl: IStorageItemAccessListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckAccess(&*(&file as *const <super::IStorageItem as ::windows::core::Abi>::Abi as *const <super::IStorageItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Entries<Impl: IStorageItemAccessListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Entries() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumItemsAllowed<Impl: IStorageItemAccessListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaximumItemsAllowed() {
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
            ::windows::core::GetRuntimeClassName::<IStorageItemAccessList>,
            ::windows::core::GetTrustLevel,
            AddOverloadDefaultMetadata::<Impl, OFFSET>,
            Add::<Impl, OFFSET>,
            AddOrReplaceOverloadDefaultMetadata::<Impl, OFFSET>,
            AddOrReplace::<Impl, OFFSET>,
            GetItemAsync::<Impl, OFFSET>,
            GetFileAsync::<Impl, OFFSET>,
            GetFolderAsync::<Impl, OFFSET>,
            GetItemWithOptionsAsync::<Impl, OFFSET>,
            GetFileWithOptionsAsync::<Impl, OFFSET>,
            GetFolderWithOptionsAsync::<Impl, OFFSET>,
            Remove::<Impl, OFFSET>,
            ContainsItem::<Impl, OFFSET>,
            Clear::<Impl, OFFSET>,
            CheckAccess::<Impl, OFFSET>,
            Entries::<Impl, OFFSET>,
            MaximumItemsAllowed::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageItemMostRecentlyUsedListImpl: Sized + IStorageItemAccessListImpl {
    fn ItemRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StorageItemMostRecentlyUsedList, ItemRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemRemoved(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageItemMostRecentlyUsedList {
    const NAME: &'static str = "Windows.Storage.AccessCache.IStorageItemMostRecentlyUsedList";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageItemMostRecentlyUsedListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemMostRecentlyUsedListImpl, const OFFSET: isize>() -> IStorageItemMostRecentlyUsedListVtbl {
        unsafe extern "system" fn ItemRemoved<Impl: IStorageItemMostRecentlyUsedListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemRemoved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<StorageItemMostRecentlyUsedList, ItemRemovedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<StorageItemMostRecentlyUsedList, ItemRemovedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItemRemoved<Impl: IStorageItemMostRecentlyUsedListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItemRemoved(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageItemMostRecentlyUsedList>, ::windows::core::GetTrustLevel, ItemRemoved::<Impl, OFFSET>, RemoveItemRemoved::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageItemMostRecentlyUsedList2Impl: Sized + IStorageItemAccessListImpl + IStorageItemMostRecentlyUsedListImpl {
    fn AddWithMetadataAndVisibility(&self, file: &::core::option::Option<super::IStorageItem>, metadata: &::windows::core::HSTRING, visibility: RecentStorageItemVisibility) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AddOrReplaceWithMetadataAndVisibility(&self, token: &::windows::core::HSTRING, file: &::core::option::Option<super::IStorageItem>, metadata: &::windows::core::HSTRING, visibility: RecentStorageItemVisibility) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageItemMostRecentlyUsedList2 {
    const NAME: &'static str = "Windows.Storage.AccessCache.IStorageItemMostRecentlyUsedList2";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageItemMostRecentlyUsedList2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemMostRecentlyUsedList2Impl, const OFFSET: isize>() -> IStorageItemMostRecentlyUsedList2Vtbl {
        unsafe extern "system" fn AddWithMetadataAndVisibility<Impl: IStorageItemMostRecentlyUsedList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, metadata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, visibility: RecentStorageItemVisibility, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddWithMetadataAndVisibility(&*(&file as *const <super::IStorageItem as ::windows::core::Abi>::Abi as *const <super::IStorageItem as ::windows::core::DefaultType>::DefaultType), &*(&metadata as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), visibility) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOrReplaceWithMetadataAndVisibility<Impl: IStorageItemMostRecentlyUsedList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, file: ::windows::core::RawPtr, metadata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, visibility: RecentStorageItemVisibility) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .AddOrReplaceWithMetadataAndVisibility(
                    &*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&file as *const <super::IStorageItem as ::windows::core::Abi>::Abi as *const <super::IStorageItem as ::windows::core::DefaultType>::DefaultType),
                    &*(&metadata as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    visibility,
                )
                .into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageItemMostRecentlyUsedList2>, ::windows::core::GetTrustLevel, AddWithMetadataAndVisibility::<Impl, OFFSET>, AddOrReplaceWithMetadataAndVisibility::<Impl, OFFSET>)
    }
}
