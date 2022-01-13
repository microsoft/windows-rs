#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IWalletItemSystemStoreImpl: Sized {
    fn GetItemsAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<super::WalletItem>>>;
    fn DeleteAsync(&mut self, item: &::core::option::Option<super::WalletItem>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ImportItemAsync(&mut self, stream: &::core::option::Option<super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::WalletItem>>;
    fn GetAppStatusForItem(&mut self, item: &::core::option::Option<super::WalletItem>) -> ::windows::core::Result<WalletItemAppAssociation>;
    fn LaunchAppForItemAsync(&mut self, item: &::core::option::Option<super::WalletItem>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWalletItemSystemStore {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.IWalletItemSystemStore";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IWalletItemSystemStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletItemSystemStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWalletItemSystemStoreVtbl {
        unsafe extern "system" fn GetItemsAsync<Impl: IWalletItemSystemStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsync<Impl: IWalletItemSystemStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAsync(&*(&item as *const <super::WalletItem as ::windows::core::Abi>::Abi as *const <super::WalletItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportItemAsync<Impl: IWalletItemSystemStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportItemAsync(&*(&stream as *const <super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppStatusForItem<Impl: IWalletItemSystemStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut WalletItemAppAssociation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppStatusForItem(&*(&item as *const <super::WalletItem as ::windows::core::Abi>::Abi as *const <super::WalletItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchAppForItemAsync<Impl: IWalletItemSystemStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchAppForItemAsync(&*(&item as *const <super::WalletItem as ::windows::core::Abi>::Abi as *const <super::WalletItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWalletItemSystemStore, BASE_OFFSET>(),
            GetItemsAsync: GetItemsAsync::<Impl, IMPL_OFFSET>,
            DeleteAsync: DeleteAsync::<Impl, IMPL_OFFSET>,
            ImportItemAsync: ImportItemAsync::<Impl, IMPL_OFFSET>,
            GetAppStatusForItem: GetAppStatusForItem::<Impl, IMPL_OFFSET>,
            LaunchAppForItemAsync: LaunchAppForItemAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletItemSystemStore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWalletItemSystemStore2Impl: Sized {
    fn ItemsChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WalletItemSystemStore, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemsChanged(&mut self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWalletItemSystemStore2 {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.IWalletItemSystemStore2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWalletItemSystemStore2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletItemSystemStore2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWalletItemSystemStore2Vtbl {
        unsafe extern "system" fn ItemsChanged<Impl: IWalletItemSystemStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemsChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<WalletItemSystemStore, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<WalletItemSystemStore, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItemsChanged<Impl: IWalletItemSystemStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItemsChanged(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWalletItemSystemStore2, BASE_OFFSET>(),
            ItemsChanged: ItemsChanged::<Impl, IMPL_OFFSET>,
            RemoveItemsChanged: RemoveItemsChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletItemSystemStore2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWalletManagerSystemStaticsImpl: Sized {
    fn RequestStoreAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WalletItemSystemStore>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWalletManagerSystemStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.IWalletManagerSystemStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWalletManagerSystemStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletManagerSystemStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWalletManagerSystemStaticsVtbl {
        unsafe extern "system" fn RequestStoreAsync<Impl: IWalletManagerSystemStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStoreAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWalletManagerSystemStatics, BASE_OFFSET>(),
            RequestStoreAsync: RequestStoreAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletManagerSystemStatics as ::windows::core::Interface>::IID
    }
}
