#[cfg(feature = "implement_exclusive")]
pub trait IWalletItemSystemStoreImpl: Sized {
    fn GetItemsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<super::WalletItem>>>;
    fn DeleteAsync(&self, item: &::core::option::Option<super::WalletItem>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ImportItemAsync(&self, stream: &::core::option::Option<super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::WalletItem>>;
    fn GetAppStatusForItem(&self, item: &::core::option::Option<super::WalletItem>) -> ::windows::core::Result<WalletItemAppAssociation>;
    fn LaunchAppForItemAsync(&self, item: &::core::option::Option<super::WalletItem>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWalletItemSystemStore {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.IWalletItemSystemStore";
}
#[cfg(feature = "implement_exclusive")]
impl IWalletItemSystemStoreVtbl {
    pub const fn new<Impl: IWalletItemSystemStoreImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWalletItemSystemStoreVtbl {
        unsafe extern "system" fn GetItemsAsync<Impl: IWalletItemSystemStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsync<Impl: IWalletItemSystemStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteAsync(&*(&item as *const <super::WalletItem as ::windows::core::Abi>::Abi as *const <super::WalletItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportItemAsync<Impl: IWalletItemSystemStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImportItemAsync(&*(&stream as *const <super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppStatusForItem<Impl: IWalletItemSystemStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut WalletItemAppAssociation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAppStatusForItem(&*(&item as *const <super::WalletItem as ::windows::core::Abi>::Abi as *const <super::WalletItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchAppForItemAsync<Impl: IWalletItemSystemStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchAppForItemAsync(&*(&item as *const <super::WalletItem as ::windows::core::Abi>::Abi as *const <super::WalletItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWalletItemSystemStore>, base.5, GetItemsAsync::<Impl, OFFSET>, DeleteAsync::<Impl, OFFSET>, ImportItemAsync::<Impl, OFFSET>, GetAppStatusForItem::<Impl, OFFSET>, LaunchAppForItemAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletItemSystemStore2Impl: Sized {
    fn ItemsChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WalletItemSystemStore, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemsChanged(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWalletItemSystemStore2 {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.IWalletItemSystemStore2";
}
#[cfg(feature = "implement_exclusive")]
impl IWalletItemSystemStore2Vtbl {
    pub const fn new<Impl: IWalletItemSystemStore2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWalletItemSystemStore2Vtbl {
        unsafe extern "system" fn ItemsChanged<Impl: IWalletItemSystemStore2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemsChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<WalletItemSystemStore, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<WalletItemSystemStore, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItemsChanged<Impl: IWalletItemSystemStore2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveItemsChanged(&*(&cookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWalletItemSystemStore2>, base.5, ItemsChanged::<Impl, OFFSET>, RemoveItemsChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletManagerSystemStaticsImpl: Sized {
    fn RequestStoreAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WalletItemSystemStore>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWalletManagerSystemStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.IWalletManagerSystemStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWalletManagerSystemStaticsVtbl {
    pub const fn new<Impl: IWalletManagerSystemStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWalletManagerSystemStaticsVtbl {
        unsafe extern "system" fn RequestStoreAsync<Impl: IWalletManagerSystemStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestStoreAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWalletManagerSystemStatics>, base.5, RequestStoreAsync::<Impl, OFFSET>)
    }
}
