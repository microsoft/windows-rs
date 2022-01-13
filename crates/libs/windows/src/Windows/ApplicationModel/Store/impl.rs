#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICurrentAppImpl: Sized {
    fn LicenseInformation(&mut self) -> ::windows::core::Result<LicenseInformation>;
    fn LinkUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn AppId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RequestAppPurchaseAsync(&mut self, includereceipt: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn RequestProductPurchaseAsync(&mut self, productid: &::windows::core::HSTRING, includereceipt: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn LoadListingInformationAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>;
    fn GetAppReceiptAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetProductReceiptAsync(&mut self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICurrentApp {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ICurrentApp";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICurrentAppVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICurrentAppImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICurrentAppVtbl {
        unsafe extern "system" fn LicenseInformation<Impl: ICurrentAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LicenseInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LinkUri<Impl: ICurrentAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LinkUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppId<Impl: ICurrentAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAppPurchaseAsync<Impl: ICurrentAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includereceipt: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAppPurchaseAsync(includereceipt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestProductPurchaseAsync<Impl: ICurrentAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, includereceipt: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestProductPurchaseAsync(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), includereceipt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadListingInformationAsync<Impl: ICurrentAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadListingInformationAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppReceiptAsync<Impl: ICurrentAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppReceiptAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProductReceiptAsync<Impl: ICurrentAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProductReceiptAsync(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICurrentApp, BASE_OFFSET>(),
            LicenseInformation: LicenseInformation::<Impl, IMPL_OFFSET>,
            LinkUri: LinkUri::<Impl, IMPL_OFFSET>,
            AppId: AppId::<Impl, IMPL_OFFSET>,
            RequestAppPurchaseAsync: RequestAppPurchaseAsync::<Impl, IMPL_OFFSET>,
            RequestProductPurchaseAsync: RequestProductPurchaseAsync::<Impl, IMPL_OFFSET>,
            LoadListingInformationAsync: LoadListingInformationAsync::<Impl, IMPL_OFFSET>,
            GetAppReceiptAsync: GetAppReceiptAsync::<Impl, IMPL_OFFSET>,
            GetProductReceiptAsync: GetProductReceiptAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICurrentApp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICurrentApp2StaticsImpl: Sized {
    fn GetCustomerPurchaseIdAsync(&mut self, serviceticket: &::windows::core::HSTRING, publisheruserid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetCustomerCollectionsIdAsync(&mut self, serviceticket: &::windows::core::HSTRING, publisheruserid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICurrentApp2Statics {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ICurrentApp2Statics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICurrentApp2StaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICurrentApp2StaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICurrentApp2StaticsVtbl {
        unsafe extern "system" fn GetCustomerPurchaseIdAsync<Impl: ICurrentApp2StaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceticket: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, publisheruserid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomerPurchaseIdAsync(&*(&serviceticket as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&publisheruserid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomerCollectionsIdAsync<Impl: ICurrentApp2StaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceticket: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, publisheruserid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomerCollectionsIdAsync(&*(&serviceticket as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&publisheruserid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICurrentApp2Statics, BASE_OFFSET>(),
            GetCustomerPurchaseIdAsync: GetCustomerPurchaseIdAsync::<Impl, IMPL_OFFSET>,
            GetCustomerCollectionsIdAsync: GetCustomerCollectionsIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICurrentApp2Statics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
pub trait ICurrentAppSimulatorImpl: Sized {
    fn LicenseInformation(&mut self) -> ::windows::core::Result<LicenseInformation>;
    fn LinkUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn AppId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RequestAppPurchaseAsync(&mut self, includereceipt: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn RequestProductPurchaseAsync(&mut self, productid: &::windows::core::HSTRING, includereceipt: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn LoadListingInformationAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>;
    fn GetAppReceiptAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetProductReceiptAsync(&mut self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ReloadSimulatorAsync(&mut self, simulatorsettingsfile: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICurrentAppSimulator {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ICurrentAppSimulator";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ICurrentAppSimulatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICurrentAppSimulatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICurrentAppSimulatorVtbl {
        unsafe extern "system" fn LicenseInformation<Impl: ICurrentAppSimulatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LicenseInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LinkUri<Impl: ICurrentAppSimulatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LinkUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppId<Impl: ICurrentAppSimulatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAppPurchaseAsync<Impl: ICurrentAppSimulatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includereceipt: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAppPurchaseAsync(includereceipt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestProductPurchaseAsync<Impl: ICurrentAppSimulatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, includereceipt: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestProductPurchaseAsync(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), includereceipt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadListingInformationAsync<Impl: ICurrentAppSimulatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadListingInformationAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppReceiptAsync<Impl: ICurrentAppSimulatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppReceiptAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProductReceiptAsync<Impl: ICurrentAppSimulatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProductReceiptAsync(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReloadSimulatorAsync<Impl: ICurrentAppSimulatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, simulatorsettingsfile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReloadSimulatorAsync(&*(&simulatorsettingsfile as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICurrentAppSimulator, BASE_OFFSET>(),
            LicenseInformation: LicenseInformation::<Impl, IMPL_OFFSET>,
            LinkUri: LinkUri::<Impl, IMPL_OFFSET>,
            AppId: AppId::<Impl, IMPL_OFFSET>,
            RequestAppPurchaseAsync: RequestAppPurchaseAsync::<Impl, IMPL_OFFSET>,
            RequestProductPurchaseAsync: RequestProductPurchaseAsync::<Impl, IMPL_OFFSET>,
            LoadListingInformationAsync: LoadListingInformationAsync::<Impl, IMPL_OFFSET>,
            GetAppReceiptAsync: GetAppReceiptAsync::<Impl, IMPL_OFFSET>,
            GetProductReceiptAsync: GetProductReceiptAsync::<Impl, IMPL_OFFSET>,
            ReloadSimulatorAsync: ReloadSimulatorAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICurrentAppSimulator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICurrentAppSimulatorStaticsWithFilteringImpl: Sized {
    fn LoadListingInformationByProductIdsAsync(&mut self, productids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>;
    fn LoadListingInformationByKeywordsAsync(&mut self, keywords: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICurrentAppSimulatorStaticsWithFiltering {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ICurrentAppSimulatorStaticsWithFiltering";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICurrentAppSimulatorStaticsWithFilteringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICurrentAppSimulatorStaticsWithFilteringImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICurrentAppSimulatorStaticsWithFilteringVtbl {
        unsafe extern "system" fn LoadListingInformationByProductIdsAsync<Impl: ICurrentAppSimulatorStaticsWithFilteringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadListingInformationByProductIdsAsync(&*(&productids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadListingInformationByKeywordsAsync<Impl: ICurrentAppSimulatorStaticsWithFilteringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keywords: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadListingInformationByKeywordsAsync(&*(&keywords as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICurrentAppSimulatorStaticsWithFiltering, BASE_OFFSET>(),
            LoadListingInformationByProductIdsAsync: LoadListingInformationByProductIdsAsync::<Impl, IMPL_OFFSET>,
            LoadListingInformationByKeywordsAsync: LoadListingInformationByKeywordsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICurrentAppSimulatorStaticsWithFiltering as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICurrentAppSimulatorWithCampaignIdImpl: Sized {
    fn GetAppPurchaseCampaignIdAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICurrentAppSimulatorWithCampaignId {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ICurrentAppSimulatorWithCampaignId";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICurrentAppSimulatorWithCampaignIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICurrentAppSimulatorWithCampaignIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICurrentAppSimulatorWithCampaignIdVtbl {
        unsafe extern "system" fn GetAppPurchaseCampaignIdAsync<Impl: ICurrentAppSimulatorWithCampaignIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppPurchaseCampaignIdAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICurrentAppSimulatorWithCampaignId, BASE_OFFSET>(),
            GetAppPurchaseCampaignIdAsync: GetAppPurchaseCampaignIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICurrentAppSimulatorWithCampaignId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICurrentAppSimulatorWithConsumablesImpl: Sized {
    fn ReportConsumableFulfillmentAsync(&mut self, productid: &::windows::core::HSTRING, transactionid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FulfillmentResult>>;
    fn RequestProductPurchaseWithResultsAsync(&mut self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PurchaseResults>>;
    fn RequestProductPurchaseWithDisplayPropertiesAsync(&mut self, productid: &::windows::core::HSTRING, offerid: &::windows::core::HSTRING, displayproperties: &::core::option::Option<ProductPurchaseDisplayProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PurchaseResults>>;
    fn GetUnfulfilledConsumablesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UnfulfilledConsumable>>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICurrentAppSimulatorWithConsumables {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ICurrentAppSimulatorWithConsumables";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICurrentAppSimulatorWithConsumablesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICurrentAppSimulatorWithConsumablesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICurrentAppSimulatorWithConsumablesVtbl {
        unsafe extern "system" fn ReportConsumableFulfillmentAsync<Impl: ICurrentAppSimulatorWithConsumablesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, transactionid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportConsumableFulfillmentAsync(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&transactionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestProductPurchaseWithResultsAsync<Impl: ICurrentAppSimulatorWithConsumablesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestProductPurchaseWithResultsAsync(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestProductPurchaseWithDisplayPropertiesAsync<Impl: ICurrentAppSimulatorWithConsumablesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, offerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestProductPurchaseWithDisplayPropertiesAsync(
                &*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&offerid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&displayproperties as *const <ProductPurchaseDisplayProperties as ::windows::core::Abi>::Abi as *const <ProductPurchaseDisplayProperties as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnfulfilledConsumablesAsync<Impl: ICurrentAppSimulatorWithConsumablesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnfulfilledConsumablesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICurrentAppSimulatorWithConsumables, BASE_OFFSET>(),
            ReportConsumableFulfillmentAsync: ReportConsumableFulfillmentAsync::<Impl, IMPL_OFFSET>,
            RequestProductPurchaseWithResultsAsync: RequestProductPurchaseWithResultsAsync::<Impl, IMPL_OFFSET>,
            RequestProductPurchaseWithDisplayPropertiesAsync: RequestProductPurchaseWithDisplayPropertiesAsync::<Impl, IMPL_OFFSET>,
            GetUnfulfilledConsumablesAsync: GetUnfulfilledConsumablesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICurrentAppSimulatorWithConsumables as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICurrentAppStaticsWithFilteringImpl: Sized {
    fn LoadListingInformationByProductIdsAsync(&mut self, productids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>;
    fn LoadListingInformationByKeywordsAsync(&mut self, keywords: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>;
    fn ReportProductFulfillment(&mut self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICurrentAppStaticsWithFiltering {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ICurrentAppStaticsWithFiltering";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICurrentAppStaticsWithFilteringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICurrentAppStaticsWithFilteringImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICurrentAppStaticsWithFilteringVtbl {
        unsafe extern "system" fn LoadListingInformationByProductIdsAsync<Impl: ICurrentAppStaticsWithFilteringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadListingInformationByProductIdsAsync(&*(&productids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadListingInformationByKeywordsAsync<Impl: ICurrentAppStaticsWithFilteringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keywords: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadListingInformationByKeywordsAsync(&*(&keywords as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportProductFulfillment<Impl: ICurrentAppStaticsWithFilteringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportProductFulfillment(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICurrentAppStaticsWithFiltering, BASE_OFFSET>(),
            LoadListingInformationByProductIdsAsync: LoadListingInformationByProductIdsAsync::<Impl, IMPL_OFFSET>,
            LoadListingInformationByKeywordsAsync: LoadListingInformationByKeywordsAsync::<Impl, IMPL_OFFSET>,
            ReportProductFulfillment: ReportProductFulfillment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICurrentAppStaticsWithFiltering as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICurrentAppWithCampaignIdImpl: Sized {
    fn GetAppPurchaseCampaignIdAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICurrentAppWithCampaignId {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ICurrentAppWithCampaignId";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICurrentAppWithCampaignIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICurrentAppWithCampaignIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICurrentAppWithCampaignIdVtbl {
        unsafe extern "system" fn GetAppPurchaseCampaignIdAsync<Impl: ICurrentAppWithCampaignIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppPurchaseCampaignIdAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICurrentAppWithCampaignId, BASE_OFFSET>(),
            GetAppPurchaseCampaignIdAsync: GetAppPurchaseCampaignIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICurrentAppWithCampaignId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICurrentAppWithConsumablesImpl: Sized {
    fn ReportConsumableFulfillmentAsync(&mut self, productid: &::windows::core::HSTRING, transactionid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<FulfillmentResult>>;
    fn RequestProductPurchaseWithResultsAsync(&mut self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PurchaseResults>>;
    fn RequestProductPurchaseWithDisplayPropertiesAsync(&mut self, productid: &::windows::core::HSTRING, offerid: &::windows::core::HSTRING, displayproperties: &::core::option::Option<ProductPurchaseDisplayProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PurchaseResults>>;
    fn GetUnfulfilledConsumablesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UnfulfilledConsumable>>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICurrentAppWithConsumables {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ICurrentAppWithConsumables";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICurrentAppWithConsumablesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICurrentAppWithConsumablesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICurrentAppWithConsumablesVtbl {
        unsafe extern "system" fn ReportConsumableFulfillmentAsync<Impl: ICurrentAppWithConsumablesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, transactionid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportConsumableFulfillmentAsync(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&transactionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestProductPurchaseWithResultsAsync<Impl: ICurrentAppWithConsumablesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestProductPurchaseWithResultsAsync(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestProductPurchaseWithDisplayPropertiesAsync<Impl: ICurrentAppWithConsumablesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, offerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestProductPurchaseWithDisplayPropertiesAsync(
                &*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&offerid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&displayproperties as *const <ProductPurchaseDisplayProperties as ::windows::core::Abi>::Abi as *const <ProductPurchaseDisplayProperties as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnfulfilledConsumablesAsync<Impl: ICurrentAppWithConsumablesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnfulfilledConsumablesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICurrentAppWithConsumables, BASE_OFFSET>(),
            ReportConsumableFulfillmentAsync: ReportConsumableFulfillmentAsync::<Impl, IMPL_OFFSET>,
            RequestProductPurchaseWithResultsAsync: RequestProductPurchaseWithResultsAsync::<Impl, IMPL_OFFSET>,
            RequestProductPurchaseWithDisplayPropertiesAsync: RequestProductPurchaseWithDisplayPropertiesAsync::<Impl, IMPL_OFFSET>,
            GetUnfulfilledConsumablesAsync: GetUnfulfilledConsumablesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICurrentAppWithConsumables as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ILicenseInformationImpl: Sized {
    fn ProductLicenses(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ProductLicense>>;
    fn IsActive(&mut self) -> ::windows::core::Result<bool>;
    fn IsTrial(&mut self) -> ::windows::core::Result<bool>;
    fn ExpirationDate(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn LicenseChanged(&mut self, handler: &::core::option::Option<LicenseChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLicenseChanged(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILicenseInformation {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ILicenseInformation";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILicenseInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILicenseInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILicenseInformationVtbl {
        unsafe extern "system" fn ProductLicenses<Impl: ILicenseInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductLicenses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsActive<Impl: ILicenseInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTrial<Impl: ILicenseInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTrial() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationDate<Impl: ILicenseInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LicenseChanged<Impl: ILicenseInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LicenseChanged(&*(&handler as *const <LicenseChangedEventHandler as ::windows::core::Abi>::Abi as *const <LicenseChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLicenseChanged<Impl: ILicenseInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLicenseChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILicenseInformation, BASE_OFFSET>(),
            ProductLicenses: ProductLicenses::<Impl, IMPL_OFFSET>,
            IsActive: IsActive::<Impl, IMPL_OFFSET>,
            IsTrial: IsTrial::<Impl, IMPL_OFFSET>,
            ExpirationDate: ExpirationDate::<Impl, IMPL_OFFSET>,
            LicenseChanged: LicenseChanged::<Impl, IMPL_OFFSET>,
            RemoveLicenseChanged: RemoveLicenseChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILicenseInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IListingInformationImpl: Sized {
    fn CurrentMarket(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProductListings(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ProductListing>>;
    fn FormattedPrice(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AgeRating(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListingInformation {
    const NAME: &'static str = "Windows.ApplicationModel.Store.IListingInformation";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IListingInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListingInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListingInformationVtbl {
        unsafe extern "system" fn CurrentMarket<Impl: IListingInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentMarket() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IListingInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductListings<Impl: IListingInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductListings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormattedPrice<Impl: IListingInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormattedPrice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IListingInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AgeRating<Impl: IListingInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AgeRating() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListingInformation, BASE_OFFSET>(),
            CurrentMarket: CurrentMarket::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            ProductListings: ProductListings::<Impl, IMPL_OFFSET>,
            FormattedPrice: FormattedPrice::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            AgeRating: AgeRating::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListingInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IListingInformation2Impl: Sized {
    fn FormattedBasePrice(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SaleEndDate(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn IsOnSale(&mut self) -> ::windows::core::Result<bool>;
    fn CurrencyCode(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListingInformation2 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.IListingInformation2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IListingInformation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListingInformation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListingInformation2Vtbl {
        unsafe extern "system" fn FormattedBasePrice<Impl: IListingInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormattedBasePrice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaleEndDate<Impl: IListingInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaleEndDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOnSale<Impl: IListingInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOnSale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrencyCode<Impl: IListingInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrencyCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListingInformation2, BASE_OFFSET>(),
            FormattedBasePrice: FormattedBasePrice::<Impl, IMPL_OFFSET>,
            SaleEndDate: SaleEndDate::<Impl, IMPL_OFFSET>,
            IsOnSale: IsOnSale::<Impl, IMPL_OFFSET>,
            CurrencyCode: CurrencyCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListingInformation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IProductLicenseImpl: Sized {
    fn ProductId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsActive(&mut self) -> ::windows::core::Result<bool>;
    fn ExpirationDate(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProductLicense {
    const NAME: &'static str = "Windows.ApplicationModel.Store.IProductLicense";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProductLicenseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProductLicenseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProductLicenseVtbl {
        unsafe extern "system" fn ProductId<Impl: IProductLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsActive<Impl: IProductLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationDate<Impl: IProductLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProductLicense, BASE_OFFSET>(),
            ProductId: ProductId::<Impl, IMPL_OFFSET>,
            IsActive: IsActive::<Impl, IMPL_OFFSET>,
            ExpirationDate: ExpirationDate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProductLicense as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IProductLicenseWithFulfillmentImpl: Sized + IProductLicenseImpl {
    fn IsConsumable(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProductLicenseWithFulfillment {
    const NAME: &'static str = "Windows.ApplicationModel.Store.IProductLicenseWithFulfillment";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProductLicenseWithFulfillmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProductLicenseWithFulfillmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProductLicenseWithFulfillmentVtbl {
        unsafe extern "system" fn IsConsumable<Impl: IProductLicenseWithFulfillmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConsumable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProductLicenseWithFulfillment, BASE_OFFSET>(),
            IsConsumable: IsConsumable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProductLicenseWithFulfillment as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProductListingImpl: Sized {
    fn ProductId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormattedPrice(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProductListing {
    const NAME: &'static str = "Windows.ApplicationModel.Store.IProductListing";
}
#[cfg(feature = "implement_exclusive")]
impl IProductListingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProductListingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProductListingVtbl {
        unsafe extern "system" fn ProductId<Impl: IProductListingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormattedPrice<Impl: IProductListingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormattedPrice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IProductListingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProductListing, BASE_OFFSET>(),
            ProductId: ProductId::<Impl, IMPL_OFFSET>,
            FormattedPrice: FormattedPrice::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProductListing as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IProductListing2Impl: Sized {
    fn FormattedBasePrice(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SaleEndDate(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn IsOnSale(&mut self) -> ::windows::core::Result<bool>;
    fn CurrencyCode(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProductListing2 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.IProductListing2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProductListing2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProductListing2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProductListing2Vtbl {
        unsafe extern "system" fn FormattedBasePrice<Impl: IProductListing2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormattedBasePrice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaleEndDate<Impl: IProductListing2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaleEndDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOnSale<Impl: IProductListing2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOnSale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrencyCode<Impl: IProductListing2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrencyCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProductListing2, BASE_OFFSET>(),
            FormattedBasePrice: FormattedBasePrice::<Impl, IMPL_OFFSET>,
            SaleEndDate: SaleEndDate::<Impl, IMPL_OFFSET>,
            IsOnSale: IsOnSale::<Impl, IMPL_OFFSET>,
            CurrencyCode: CurrencyCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProductListing2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProductListingWithConsumablesImpl: Sized {
    fn ProductType(&mut self) -> ::windows::core::Result<ProductType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProductListingWithConsumables {
    const NAME: &'static str = "Windows.ApplicationModel.Store.IProductListingWithConsumables";
}
#[cfg(feature = "implement_exclusive")]
impl IProductListingWithConsumablesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProductListingWithConsumablesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProductListingWithConsumablesVtbl {
        unsafe extern "system" fn ProductType<Impl: IProductListingWithConsumablesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ProductType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProductListingWithConsumables, BASE_OFFSET>(),
            ProductType: ProductType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProductListingWithConsumables as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IProductListingWithMetadataImpl: Sized + IProductListingImpl {
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Keywords(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>;
    fn ProductType(&mut self) -> ::windows::core::Result<ProductType>;
    fn Tag(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ImageUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProductListingWithMetadata {
    const NAME: &'static str = "Windows.ApplicationModel.Store.IProductListingWithMetadata";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IProductListingWithMetadataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProductListingWithMetadataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProductListingWithMetadataVtbl {
        unsafe extern "system" fn Description<Impl: IProductListingWithMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Keywords<Impl: IProductListingWithMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Keywords() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductType<Impl: IProductListingWithMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ProductType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tag<Impl: IProductListingWithMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageUri<Impl: IProductListingWithMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProductListingWithMetadata, BASE_OFFSET>(),
            Description: Description::<Impl, IMPL_OFFSET>,
            Keywords: Keywords::<Impl, IMPL_OFFSET>,
            ProductType: ProductType::<Impl, IMPL_OFFSET>,
            Tag: Tag::<Impl, IMPL_OFFSET>,
            ImageUri: ImageUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProductListingWithMetadata as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IProductPurchaseDisplayPropertiesImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Image(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetImage(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProductPurchaseDisplayProperties {
    const NAME: &'static str = "Windows.ApplicationModel.Store.IProductPurchaseDisplayProperties";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProductPurchaseDisplayPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProductPurchaseDisplayPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProductPurchaseDisplayPropertiesVtbl {
        unsafe extern "system" fn Name<Impl: IProductPurchaseDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IProductPurchaseDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IProductPurchaseDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IProductPurchaseDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Image<Impl: IProductPurchaseDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImage<Impl: IProductPurchaseDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImage(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProductPurchaseDisplayProperties, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            Image: Image::<Impl, IMPL_OFFSET>,
            SetImage: SetImage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProductPurchaseDisplayProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProductPurchaseDisplayPropertiesFactoryImpl: Sized {
    fn CreateProductPurchaseDisplayProperties(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<ProductPurchaseDisplayProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProductPurchaseDisplayPropertiesFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Store.IProductPurchaseDisplayPropertiesFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IProductPurchaseDisplayPropertiesFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProductPurchaseDisplayPropertiesFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProductPurchaseDisplayPropertiesFactoryVtbl {
        unsafe extern "system" fn CreateProductPurchaseDisplayProperties<Impl: IProductPurchaseDisplayPropertiesFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProductPurchaseDisplayProperties(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProductPurchaseDisplayPropertiesFactory, BASE_OFFSET>(),
            CreateProductPurchaseDisplayProperties: CreateProductPurchaseDisplayProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProductPurchaseDisplayPropertiesFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPurchaseResultsImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<ProductPurchaseStatus>;
    fn TransactionId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ReceiptXml(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OfferId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPurchaseResults {
    const NAME: &'static str = "Windows.ApplicationModel.Store.IPurchaseResults";
}
#[cfg(feature = "implement_exclusive")]
impl IPurchaseResultsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPurchaseResultsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPurchaseResultsVtbl {
        unsafe extern "system" fn Status<Impl: IPurchaseResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ProductPurchaseStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Impl: IPurchaseResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptXml<Impl: IPurchaseResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiptXml() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OfferId<Impl: IPurchaseResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OfferId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPurchaseResults, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            TransactionId: TransactionId::<Impl, IMPL_OFFSET>,
            ReceiptXml: ReceiptXml::<Impl, IMPL_OFFSET>,
            OfferId: OfferId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPurchaseResults as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnfulfilledConsumableImpl: Sized {
    fn ProductId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TransactionId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn OfferId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUnfulfilledConsumable {
    const NAME: &'static str = "Windows.ApplicationModel.Store.IUnfulfilledConsumable";
}
#[cfg(feature = "implement_exclusive")]
impl IUnfulfilledConsumableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnfulfilledConsumableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnfulfilledConsumableVtbl {
        unsafe extern "system" fn ProductId<Impl: IUnfulfilledConsumableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Impl: IUnfulfilledConsumableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OfferId<Impl: IUnfulfilledConsumableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OfferId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUnfulfilledConsumable, BASE_OFFSET>(),
            ProductId: ProductId::<Impl, IMPL_OFFSET>,
            TransactionId: TransactionId::<Impl, IMPL_OFFSET>,
            OfferId: OfferId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnfulfilledConsumable as ::windows::core::Interface>::IID
    }
}
