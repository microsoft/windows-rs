#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IWalletBarcodeImpl: Sized {
    fn Symbology(&mut self) -> ::windows::core::Result<WalletBarcodeSymbology>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetImageAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamReference>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWalletBarcode {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.IWalletBarcode";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IWalletBarcodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletBarcodeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWalletBarcodeVtbl {
        unsafe extern "system" fn Symbology<Impl: IWalletBarcodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WalletBarcodeSymbology) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Symbology() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IWalletBarcodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageAsync<Impl: IWalletBarcodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWalletBarcode, BASE_OFFSET>(),
            Symbology: Symbology::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            GetImageAsync: GetImageAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletBarcode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IWalletBarcodeFactoryImpl: Sized {
    fn CreateWalletBarcode(&mut self, symbology: WalletBarcodeSymbology, value: &::windows::core::HSTRING) -> ::windows::core::Result<WalletBarcode>;
    fn CreateCustomWalletBarcode(&mut self, streamtobarcodeimage: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<WalletBarcode>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWalletBarcodeFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.IWalletBarcodeFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IWalletBarcodeFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletBarcodeFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWalletBarcodeFactoryVtbl {
        unsafe extern "system" fn CreateWalletBarcode<Impl: IWalletBarcodeFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, symbology: WalletBarcodeSymbology, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWalletBarcode(symbology, &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomWalletBarcode<Impl: IWalletBarcodeFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamtobarcodeimage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCustomWalletBarcode(&*(&streamtobarcodeimage as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWalletBarcodeFactory, BASE_OFFSET>(),
            CreateWalletBarcode: CreateWalletBarcode::<Impl, IMPL_OFFSET>,
            CreateCustomWalletBarcode: CreateCustomWalletBarcode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletBarcodeFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "implement_exclusive"))]
pub trait IWalletItemImpl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsAcknowledged(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsAcknowledged(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IssuerDisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIssuerDisplayName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LastUpdated(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetLastUpdated(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Kind(&mut self) -> ::windows::core::Result<WalletItemKind>;
    fn Barcode(&mut self) -> ::windows::core::Result<WalletBarcode>;
    fn SetBarcode(&mut self, value: &::core::option::Option<WalletBarcode>) -> ::windows::core::Result<()>;
    fn ExpirationDate(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetExpirationDate(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Logo159x159(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetLogo159x159(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn Logo336x336(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetLogo336x336(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn Logo99x99(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetLogo99x99(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn DisplayMessage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayMessage(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsDisplayMessageLaunchable(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsDisplayMessageLaunchable(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn LogoText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLogoText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HeaderColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetHeaderColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn BodyColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBodyColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn HeaderFontColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetHeaderFontColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn BodyFontColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBodyFontColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn HeaderBackgroundImage(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetHeaderBackgroundImage(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn BodyBackgroundImage(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetBodyBackgroundImage(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn LogoImage(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetLogoImage(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn PromotionalImage(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetPromotionalImage(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn RelevantDate(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetRelevantDate(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn RelevantDateDisplayMessage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRelevantDateDisplayMessage(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TransactionHistory(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, WalletTransaction>>;
    fn RelevantLocations(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, WalletRelevantLocation>>;
    fn IsMoreTransactionHistoryLaunchable(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsMoreTransactionHistoryLaunchable(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn DisplayProperties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, WalletItemCustomProperty>>;
    fn Verbs(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, WalletVerb>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWalletItem {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.IWalletItem";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "implement_exclusive"))]
impl IWalletItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWalletItemVtbl {
        unsafe extern "system" fn DisplayName<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Id<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAcknowledged<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAcknowledged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAcknowledged<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAcknowledged(value).into()
        }
        unsafe extern "system" fn IssuerDisplayName<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IssuerDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIssuerDisplayName<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIssuerDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LastUpdated<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastUpdated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastUpdated<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastUpdated(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Kind<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WalletItemKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Barcode<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Barcode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBarcode<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBarcode(&*(&value as *const <WalletBarcode as ::windows::core::Abi>::Abi as *const <WalletBarcode as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExpirationDate<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExpirationDate<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpirationDate(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Logo159x159<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Logo159x159() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogo159x159<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogo159x159(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Logo336x336<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Logo336x336() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogo336x336<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogo336x336(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Logo99x99<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Logo99x99() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogo99x99<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogo99x99(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayMessage<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayMessage<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayMessage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsDisplayMessageLaunchable<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDisplayMessageLaunchable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDisplayMessageLaunchable<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDisplayMessageLaunchable(value).into()
        }
        unsafe extern "system" fn LogoText<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogoText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogoText<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogoText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HeaderColor<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeaderColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeaderColor<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeaderColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BodyColor<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BodyColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBodyColor<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBodyColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HeaderFontColor<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeaderFontColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeaderFontColor<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeaderFontColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BodyFontColor<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BodyFontColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBodyFontColor<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBodyFontColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HeaderBackgroundImage<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeaderBackgroundImage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeaderBackgroundImage<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeaderBackgroundImage(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BodyBackgroundImage<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BodyBackgroundImage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBodyBackgroundImage<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBodyBackgroundImage(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LogoImage<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogoImage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogoImage<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogoImage(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PromotionalImage<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PromotionalImage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPromotionalImage<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPromotionalImage(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RelevantDate<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelevantDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelevantDate<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRelevantDate(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RelevantDateDisplayMessage<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelevantDateDisplayMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelevantDateDisplayMessage<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRelevantDateDisplayMessage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransactionHistory<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionHistory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelevantLocations<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelevantLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMoreTransactionHistoryLaunchable<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMoreTransactionHistoryLaunchable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsMoreTransactionHistoryLaunchable<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsMoreTransactionHistoryLaunchable(value).into()
        }
        unsafe extern "system" fn DisplayProperties<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Verbs<Impl: IWalletItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Verbs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWalletItem, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            IsAcknowledged: IsAcknowledged::<Impl, IMPL_OFFSET>,
            SetIsAcknowledged: SetIsAcknowledged::<Impl, IMPL_OFFSET>,
            IssuerDisplayName: IssuerDisplayName::<Impl, IMPL_OFFSET>,
            SetIssuerDisplayName: SetIssuerDisplayName::<Impl, IMPL_OFFSET>,
            LastUpdated: LastUpdated::<Impl, IMPL_OFFSET>,
            SetLastUpdated: SetLastUpdated::<Impl, IMPL_OFFSET>,
            Kind: Kind::<Impl, IMPL_OFFSET>,
            Barcode: Barcode::<Impl, IMPL_OFFSET>,
            SetBarcode: SetBarcode::<Impl, IMPL_OFFSET>,
            ExpirationDate: ExpirationDate::<Impl, IMPL_OFFSET>,
            SetExpirationDate: SetExpirationDate::<Impl, IMPL_OFFSET>,
            Logo159x159: Logo159x159::<Impl, IMPL_OFFSET>,
            SetLogo159x159: SetLogo159x159::<Impl, IMPL_OFFSET>,
            Logo336x336: Logo336x336::<Impl, IMPL_OFFSET>,
            SetLogo336x336: SetLogo336x336::<Impl, IMPL_OFFSET>,
            Logo99x99: Logo99x99::<Impl, IMPL_OFFSET>,
            SetLogo99x99: SetLogo99x99::<Impl, IMPL_OFFSET>,
            DisplayMessage: DisplayMessage::<Impl, IMPL_OFFSET>,
            SetDisplayMessage: SetDisplayMessage::<Impl, IMPL_OFFSET>,
            IsDisplayMessageLaunchable: IsDisplayMessageLaunchable::<Impl, IMPL_OFFSET>,
            SetIsDisplayMessageLaunchable: SetIsDisplayMessageLaunchable::<Impl, IMPL_OFFSET>,
            LogoText: LogoText::<Impl, IMPL_OFFSET>,
            SetLogoText: SetLogoText::<Impl, IMPL_OFFSET>,
            HeaderColor: HeaderColor::<Impl, IMPL_OFFSET>,
            SetHeaderColor: SetHeaderColor::<Impl, IMPL_OFFSET>,
            BodyColor: BodyColor::<Impl, IMPL_OFFSET>,
            SetBodyColor: SetBodyColor::<Impl, IMPL_OFFSET>,
            HeaderFontColor: HeaderFontColor::<Impl, IMPL_OFFSET>,
            SetHeaderFontColor: SetHeaderFontColor::<Impl, IMPL_OFFSET>,
            BodyFontColor: BodyFontColor::<Impl, IMPL_OFFSET>,
            SetBodyFontColor: SetBodyFontColor::<Impl, IMPL_OFFSET>,
            HeaderBackgroundImage: HeaderBackgroundImage::<Impl, IMPL_OFFSET>,
            SetHeaderBackgroundImage: SetHeaderBackgroundImage::<Impl, IMPL_OFFSET>,
            BodyBackgroundImage: BodyBackgroundImage::<Impl, IMPL_OFFSET>,
            SetBodyBackgroundImage: SetBodyBackgroundImage::<Impl, IMPL_OFFSET>,
            LogoImage: LogoImage::<Impl, IMPL_OFFSET>,
            SetLogoImage: SetLogoImage::<Impl, IMPL_OFFSET>,
            PromotionalImage: PromotionalImage::<Impl, IMPL_OFFSET>,
            SetPromotionalImage: SetPromotionalImage::<Impl, IMPL_OFFSET>,
            RelevantDate: RelevantDate::<Impl, IMPL_OFFSET>,
            SetRelevantDate: SetRelevantDate::<Impl, IMPL_OFFSET>,
            RelevantDateDisplayMessage: RelevantDateDisplayMessage::<Impl, IMPL_OFFSET>,
            SetRelevantDateDisplayMessage: SetRelevantDateDisplayMessage::<Impl, IMPL_OFFSET>,
            TransactionHistory: TransactionHistory::<Impl, IMPL_OFFSET>,
            RelevantLocations: RelevantLocations::<Impl, IMPL_OFFSET>,
            IsMoreTransactionHistoryLaunchable: IsMoreTransactionHistoryLaunchable::<Impl, IMPL_OFFSET>,
            SetIsMoreTransactionHistoryLaunchable: SetIsMoreTransactionHistoryLaunchable::<Impl, IMPL_OFFSET>,
            DisplayProperties: DisplayProperties::<Impl, IMPL_OFFSET>,
            Verbs: Verbs::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletItemCustomPropertyImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AutoDetectLinks(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutoDetectLinks(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn DetailViewPosition(&mut self) -> ::windows::core::Result<WalletDetailViewPosition>;
    fn SetDetailViewPosition(&mut self, value: WalletDetailViewPosition) -> ::windows::core::Result<()>;
    fn SummaryViewPosition(&mut self) -> ::windows::core::Result<WalletSummaryViewPosition>;
    fn SetSummaryViewPosition(&mut self, value: WalletSummaryViewPosition) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWalletItemCustomProperty {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.IWalletItemCustomProperty";
}
#[cfg(feature = "implement_exclusive")]
impl IWalletItemCustomPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletItemCustomPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWalletItemCustomPropertyVtbl {
        unsafe extern "system" fn Name<Impl: IWalletItemCustomPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetName<Impl: IWalletItemCustomPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Value<Impl: IWalletItemCustomPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IWalletItemCustomPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AutoDetectLinks<Impl: IWalletItemCustomPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoDetectLinks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDetectLinks<Impl: IWalletItemCustomPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoDetectLinks(value).into()
        }
        unsafe extern "system" fn DetailViewPosition<Impl: IWalletItemCustomPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WalletDetailViewPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetailViewPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDetailViewPosition<Impl: IWalletItemCustomPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: WalletDetailViewPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDetailViewPosition(value).into()
        }
        unsafe extern "system" fn SummaryViewPosition<Impl: IWalletItemCustomPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WalletSummaryViewPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SummaryViewPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSummaryViewPosition<Impl: IWalletItemCustomPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: WalletSummaryViewPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSummaryViewPosition(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWalletItemCustomProperty, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            AutoDetectLinks: AutoDetectLinks::<Impl, IMPL_OFFSET>,
            SetAutoDetectLinks: SetAutoDetectLinks::<Impl, IMPL_OFFSET>,
            DetailViewPosition: DetailViewPosition::<Impl, IMPL_OFFSET>,
            SetDetailViewPosition: SetDetailViewPosition::<Impl, IMPL_OFFSET>,
            SummaryViewPosition: SummaryViewPosition::<Impl, IMPL_OFFSET>,
            SetSummaryViewPosition: SetSummaryViewPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletItemCustomProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletItemCustomPropertyFactoryImpl: Sized {
    fn CreateWalletItemCustomProperty(&mut self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<WalletItemCustomProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWalletItemCustomPropertyFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.IWalletItemCustomPropertyFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWalletItemCustomPropertyFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletItemCustomPropertyFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWalletItemCustomPropertyFactoryVtbl {
        unsafe extern "system" fn CreateWalletItemCustomProperty<Impl: IWalletItemCustomPropertyFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWalletItemCustomProperty(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWalletItemCustomPropertyFactory, BASE_OFFSET>(),
            CreateWalletItemCustomProperty: CreateWalletItemCustomProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletItemCustomPropertyFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletItemFactoryImpl: Sized {
    fn CreateWalletItem(&mut self, kind: WalletItemKind, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<WalletItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWalletItemFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.IWalletItemFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWalletItemFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletItemFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWalletItemFactoryVtbl {
        unsafe extern "system" fn CreateWalletItem<Impl: IWalletItemFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: WalletItemKind, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWalletItem(kind, &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWalletItemFactory, BASE_OFFSET>(),
            CreateWalletItem: CreateWalletItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletItemFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IWalletItemStoreImpl: Sized {
    fn AddAsync(&mut self, id: &::windows::core::HSTRING, item: &::core::option::Option<WalletItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ClearAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetWalletItemAsync(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WalletItem>>;
    fn GetItemsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WalletItem>>>;
    fn GetItemsWithKindAsync(&mut self, kind: WalletItemKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WalletItem>>>;
    fn ImportItemAsync(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WalletItem>>;
    fn DeleteAsync(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowItemAsync(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UpdateAsync(&mut self, item: &::core::option::Option<WalletItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWalletItemStore {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.IWalletItemStore";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IWalletItemStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletItemStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWalletItemStoreVtbl {
        unsafe extern "system" fn AddAsync<Impl: IWalletItemStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&item as *const <WalletItem as ::windows::core::Abi>::Abi as *const <WalletItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearAsync<Impl: IWalletItemStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWalletItemAsync<Impl: IWalletItemStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWalletItemAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsAsync<Impl: IWalletItemStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetItemsWithKindAsync<Impl: IWalletItemStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: WalletItemKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemsWithKindAsync(kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportItemAsync<Impl: IWalletItemStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportItemAsync(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsync<Impl: IWalletItemStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowAsync<Impl: IWalletItemStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowItemAsync<Impl: IWalletItemStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowItemAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAsync<Impl: IWalletItemStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateAsync(&*(&item as *const <WalletItem as ::windows::core::Abi>::Abi as *const <WalletItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWalletItemStore, BASE_OFFSET>(),
            AddAsync: AddAsync::<Impl, IMPL_OFFSET>,
            ClearAsync: ClearAsync::<Impl, IMPL_OFFSET>,
            GetWalletItemAsync: GetWalletItemAsync::<Impl, IMPL_OFFSET>,
            GetItemsAsync: GetItemsAsync::<Impl, IMPL_OFFSET>,
            GetItemsWithKindAsync: GetItemsWithKindAsync::<Impl, IMPL_OFFSET>,
            ImportItemAsync: ImportItemAsync::<Impl, IMPL_OFFSET>,
            DeleteAsync: DeleteAsync::<Impl, IMPL_OFFSET>,
            ShowAsync: ShowAsync::<Impl, IMPL_OFFSET>,
            ShowItemAsync: ShowItemAsync::<Impl, IMPL_OFFSET>,
            UpdateAsync: UpdateAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletItemStore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWalletItemStore2Impl: Sized {
    fn ItemsChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WalletItemStore, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemsChanged(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWalletItemStore2 {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.IWalletItemStore2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWalletItemStore2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletItemStore2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWalletItemStore2Vtbl {
        unsafe extern "system" fn ItemsChanged<Impl: IWalletItemStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemsChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<WalletItemStore, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<WalletItemStore, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItemsChanged<Impl: IWalletItemStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItemsChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWalletItemStore2, BASE_OFFSET>(),
            ItemsChanged: ItemsChanged::<Impl, IMPL_OFFSET>,
            RemoveItemsChanged: RemoveItemsChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletItemStore2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWalletManagerStaticsImpl: Sized {
    fn RequestStoreAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WalletItemStore>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWalletManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.IWalletManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWalletManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWalletManagerStaticsVtbl {
        unsafe extern "system" fn RequestStoreAsync<Impl: IWalletManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWalletManagerStatics, BASE_OFFSET>(),
            RequestStoreAsync: RequestStoreAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
pub trait IWalletRelevantLocationImpl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::Devices::Geolocation::BasicGeoposition>;
    fn SetPosition(&mut self, value: &super::super::Devices::Geolocation::BasicGeoposition) -> ::windows::core::Result<()>;
    fn DisplayMessage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayMessage(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWalletRelevantLocation {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.IWalletRelevantLocation";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IWalletRelevantLocationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletRelevantLocationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWalletRelevantLocationVtbl {
        unsafe extern "system" fn Position<Impl: IWalletRelevantLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Geolocation::BasicGeoposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Impl: IWalletRelevantLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Devices::Geolocation::BasicGeoposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::super::Devices::Geolocation::BasicGeoposition as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::BasicGeoposition as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayMessage<Impl: IWalletRelevantLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayMessage<Impl: IWalletRelevantLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayMessage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWalletRelevantLocation, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
            DisplayMessage: DisplayMessage::<Impl, IMPL_OFFSET>,
            SetDisplayMessage: SetDisplayMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletRelevantLocation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWalletTransactionImpl: Sized {
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayAmount(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayAmount(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IgnoreTimeOfDay(&mut self) -> ::windows::core::Result<bool>;
    fn SetIgnoreTimeOfDay(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn DisplayLocation(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayLocation(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TransactionDate(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetTransactionDate(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn IsLaunchable(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsLaunchable(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWalletTransaction {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.IWalletTransaction";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWalletTransactionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletTransactionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWalletTransactionVtbl {
        unsafe extern "system" fn Description<Impl: IWalletTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDescription<Impl: IWalletTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayAmount<Impl: IWalletTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayAmount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayAmount<Impl: IWalletTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayAmount(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IgnoreTimeOfDay<Impl: IWalletTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IgnoreTimeOfDay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIgnoreTimeOfDay<Impl: IWalletTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIgnoreTimeOfDay(value).into()
        }
        unsafe extern "system" fn DisplayLocation<Impl: IWalletTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayLocation<Impl: IWalletTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayLocation(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransactionDate<Impl: IWalletTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransactionDate<Impl: IWalletTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransactionDate(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsLaunchable<Impl: IWalletTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLaunchable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLaunchable<Impl: IWalletTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsLaunchable(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWalletTransaction, BASE_OFFSET>(),
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            DisplayAmount: DisplayAmount::<Impl, IMPL_OFFSET>,
            SetDisplayAmount: SetDisplayAmount::<Impl, IMPL_OFFSET>,
            IgnoreTimeOfDay: IgnoreTimeOfDay::<Impl, IMPL_OFFSET>,
            SetIgnoreTimeOfDay: SetIgnoreTimeOfDay::<Impl, IMPL_OFFSET>,
            DisplayLocation: DisplayLocation::<Impl, IMPL_OFFSET>,
            SetDisplayLocation: SetDisplayLocation::<Impl, IMPL_OFFSET>,
            TransactionDate: TransactionDate::<Impl, IMPL_OFFSET>,
            SetTransactionDate: SetTransactionDate::<Impl, IMPL_OFFSET>,
            IsLaunchable: IsLaunchable::<Impl, IMPL_OFFSET>,
            SetIsLaunchable: SetIsLaunchable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletTransaction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletVerbImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWalletVerb {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.IWalletVerb";
}
#[cfg(feature = "implement_exclusive")]
impl IWalletVerbVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletVerbImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWalletVerbVtbl {
        unsafe extern "system" fn Name<Impl: IWalletVerbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetName<Impl: IWalletVerbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWalletVerb, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletVerb as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWalletVerbFactoryImpl: Sized {
    fn CreateWalletVerb(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<WalletVerb>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWalletVerbFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.IWalletVerbFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWalletVerbFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWalletVerbFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWalletVerbFactoryVtbl {
        unsafe extern "system" fn CreateWalletVerb<Impl: IWalletVerbFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWalletVerb(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWalletVerbFactory, BASE_OFFSET>(),
            CreateWalletVerb: CreateWalletVerb::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWalletVerbFactory as ::windows::core::Interface>::IID
    }
}
