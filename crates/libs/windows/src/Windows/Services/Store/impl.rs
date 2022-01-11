#[cfg(feature = "implement_exclusive")]
pub trait IStoreAcquireLicenseResultImpl: Sized {
    fn StorePackageLicense(&self) -> ::windows::core::Result<StorePackageLicense>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreAcquireLicenseResult {
    const NAME: &'static str = "Windows.Services.Store.IStoreAcquireLicenseResult";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreAcquireLicenseResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreAcquireLicenseResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreAcquireLicenseResultVtbl {
        unsafe extern "system" fn StorePackageLicense<Impl: IStoreAcquireLicenseResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StorePackageLicense() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IStoreAcquireLicenseResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreAcquireLicenseResult>, ::windows::core::GetTrustLevel, StorePackageLicense::<Impl, IMPL_OFFSET>, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreAcquireLicenseResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStoreAppLicenseImpl: Sized {
    fn SkuStoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn IsTrial(&self) -> ::windows::core::Result<bool>;
    fn ExpirationDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AddOnLicenses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, StoreLicense>>;
    fn TrialTimeRemaining(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn IsTrialOwnedByThisUser(&self) -> ::windows::core::Result<bool>;
    fn TrialUniqueId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreAppLicense {
    const NAME: &'static str = "Windows.Services.Store.IStoreAppLicense";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStoreAppLicenseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreAppLicenseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreAppLicenseVtbl {
        unsafe extern "system" fn SkuStoreId<Impl: IStoreAppLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SkuStoreId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsActive<Impl: IStoreAppLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsTrial<Impl: IStoreAppLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpirationDate<Impl: IStoreAppLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedJsonData<Impl: IStoreAppLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedJsonData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOnLicenses<Impl: IStoreAppLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddOnLicenses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrialTimeRemaining<Impl: IStoreAppLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrialTimeRemaining() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTrialOwnedByThisUser<Impl: IStoreAppLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTrialOwnedByThisUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrialUniqueId<Impl: IStoreAppLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrialUniqueId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStoreAppLicense>,
            ::windows::core::GetTrustLevel,
            SkuStoreId::<Impl, IMPL_OFFSET>,
            IsActive::<Impl, IMPL_OFFSET>,
            IsTrial::<Impl, IMPL_OFFSET>,
            ExpirationDate::<Impl, IMPL_OFFSET>,
            ExtendedJsonData::<Impl, IMPL_OFFSET>,
            AddOnLicenses::<Impl, IMPL_OFFSET>,
            TrialTimeRemaining::<Impl, IMPL_OFFSET>,
            IsTrialOwnedByThisUser::<Impl, IMPL_OFFSET>,
            TrialUniqueId::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreAppLicense as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreAppLicense2Impl: Sized {
    fn IsDiscLicense(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreAppLicense2 {
    const NAME: &'static str = "Windows.Services.Store.IStoreAppLicense2";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreAppLicense2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreAppLicense2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreAppLicense2Vtbl {
        unsafe extern "system" fn IsDiscLicense<Impl: IStoreAppLicense2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDiscLicense() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreAppLicense2>, ::windows::core::GetTrustLevel, IsDiscLicense::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreAppLicense2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStoreAvailabilityImpl: Sized {
    fn StoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EndDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Price(&self) -> ::windows::core::Result<StorePrice>;
    fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RequestPurchaseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>;
    fn RequestPurchaseWithPurchasePropertiesAsync(&self, storepurchaseproperties: &::core::option::Option<StorePurchaseProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreAvailability {
    const NAME: &'static str = "Windows.Services.Store.IStoreAvailability";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStoreAvailabilityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreAvailabilityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreAvailabilityVtbl {
        unsafe extern "system" fn StoreId<Impl: IStoreAvailabilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StoreId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDate<Impl: IStoreAvailabilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Price<Impl: IStoreAvailabilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Price() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedJsonData<Impl: IStoreAvailabilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedJsonData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPurchaseAsync<Impl: IStoreAvailabilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPurchaseAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPurchaseWithPurchasePropertiesAsync<Impl: IStoreAvailabilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storepurchaseproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPurchaseWithPurchasePropertiesAsync(&*(&storepurchaseproperties as *const <StorePurchaseProperties as ::windows::core::Abi>::Abi as *const <StorePurchaseProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStoreAvailability>,
            ::windows::core::GetTrustLevel,
            StoreId::<Impl, IMPL_OFFSET>,
            EndDate::<Impl, IMPL_OFFSET>,
            Price::<Impl, IMPL_OFFSET>,
            ExtendedJsonData::<Impl, IMPL_OFFSET>,
            RequestPurchaseAsync::<Impl, IMPL_OFFSET>,
            RequestPurchaseWithPurchasePropertiesAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreAvailability as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreCanAcquireLicenseResultImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn LicensableSku(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Status(&self) -> ::windows::core::Result<StoreCanLicenseStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreCanAcquireLicenseResult {
    const NAME: &'static str = "Windows.Services.Store.IStoreCanAcquireLicenseResult";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreCanAcquireLicenseResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreCanAcquireLicenseResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreCanAcquireLicenseResultVtbl {
        unsafe extern "system" fn ExtendedError<Impl: IStoreCanAcquireLicenseResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LicensableSku<Impl: IStoreCanAcquireLicenseResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LicensableSku() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IStoreCanAcquireLicenseResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StoreCanLicenseStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreCanAcquireLicenseResult>, ::windows::core::GetTrustLevel, ExtendedError::<Impl, IMPL_OFFSET>, LicensableSku::<Impl, IMPL_OFFSET>, Status::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreCanAcquireLicenseResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStoreCollectionDataImpl: Sized {
    fn IsTrial(&self) -> ::windows::core::Result<bool>;
    fn CampaignId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeveloperOfferId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AcquiredDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn StartDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn EndDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn TrialTimeRemaining(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreCollectionData {
    const NAME: &'static str = "Windows.Services.Store.IStoreCollectionData";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStoreCollectionDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreCollectionDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreCollectionDataVtbl {
        unsafe extern "system" fn IsTrial<Impl: IStoreCollectionDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CampaignId<Impl: IStoreCollectionDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CampaignId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeveloperOfferId<Impl: IStoreCollectionDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeveloperOfferId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquiredDate<Impl: IStoreCollectionDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquiredDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartDate<Impl: IStoreCollectionDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDate<Impl: IStoreCollectionDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrialTimeRemaining<Impl: IStoreCollectionDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrialTimeRemaining() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedJsonData<Impl: IStoreCollectionDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedJsonData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStoreCollectionData>,
            ::windows::core::GetTrustLevel,
            IsTrial::<Impl, IMPL_OFFSET>,
            CampaignId::<Impl, IMPL_OFFSET>,
            DeveloperOfferId::<Impl, IMPL_OFFSET>,
            AcquiredDate::<Impl, IMPL_OFFSET>,
            StartDate::<Impl, IMPL_OFFSET>,
            EndDate::<Impl, IMPL_OFFSET>,
            TrialTimeRemaining::<Impl, IMPL_OFFSET>,
            ExtendedJsonData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreCollectionData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreConsumableResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<StoreConsumableStatus>;
    fn TrackingId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BalanceRemaining(&self) -> ::windows::core::Result<u32>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreConsumableResult {
    const NAME: &'static str = "Windows.Services.Store.IStoreConsumableResult";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreConsumableResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreConsumableResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreConsumableResultVtbl {
        unsafe extern "system" fn Status<Impl: IStoreConsumableResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StoreConsumableStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrackingId<Impl: IStoreConsumableResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackingId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BalanceRemaining<Impl: IStoreConsumableResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BalanceRemaining() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IStoreConsumableResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreConsumableResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, TrackingId::<Impl, IMPL_OFFSET>, BalanceRemaining::<Impl, IMPL_OFFSET>, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreConsumableResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
pub trait IStoreContextImpl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
    fn OfflineLicensesChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StoreContext, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOfflineLicensesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetCustomerPurchaseIdAsync(&self, serviceticket: &::windows::core::HSTRING, publisheruserid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetCustomerCollectionsIdAsync(&self, serviceticket: &::windows::core::HSTRING, publisheruserid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetAppLicenseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreAppLicense>>;
    fn GetStoreProductForCurrentAppAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductResult>>;
    fn GetStoreProductsAsync(&self, productkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, storeids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>>;
    fn GetAssociatedStoreProductsAsync(&self, productkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>>;
    fn GetAssociatedStoreProductsWithPagingAsync(&self, productkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, maxitemstoretrieveperpage: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductPagedQueryResult>>;
    fn GetUserCollectionAsync(&self, productkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>>;
    fn GetUserCollectionWithPagingAsync(&self, productkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, maxitemstoretrieveperpage: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductPagedQueryResult>>;
    fn ReportConsumableFulfillmentAsync(&self, productstoreid: &::windows::core::HSTRING, quantity: u32, trackingid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreConsumableResult>>;
    fn GetConsumableBalanceRemainingAsync(&self, productstoreid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreConsumableResult>>;
    fn AcquireStoreLicenseForOptionalPackageAsync(&self, optionalpackage: &::core::option::Option<super::super::ApplicationModel::Package>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreAcquireLicenseResult>>;
    fn RequestPurchaseAsync(&self, storeid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>;
    fn RequestPurchaseWithPurchasePropertiesAsync(&self, storeid: &::windows::core::HSTRING, storepurchaseproperties: &::core::option::Option<StorePurchaseProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>;
    fn GetAppAndOptionalStorePackageUpdatesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StorePackageUpdate>>>;
    fn RequestDownloadStorePackageUpdatesAsync(&self, storepackageupdates: &::core::option::Option<super::super::Foundation::Collections::IIterable<StorePackageUpdate>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>;
    fn RequestDownloadAndInstallStorePackageUpdatesAsync(&self, storepackageupdates: &::core::option::Option<super::super::Foundation::Collections::IIterable<StorePackageUpdate>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>;
    fn RequestDownloadAndInstallStorePackagesAsync(&self, storeids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreContext {
    const NAME: &'static str = "Windows.Services.Store.IStoreContext";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl IStoreContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreContextVtbl {
        unsafe extern "system" fn User<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OfflineLicensesChanged<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OfflineLicensesChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<StoreContext, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<StoreContext, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOfflineLicensesChanged<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOfflineLicensesChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetCustomerPurchaseIdAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceticket: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, publisheruserid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCustomerCollectionsIdAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceticket: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, publisheruserid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAppLicenseAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppLicenseAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoreProductForCurrentAppAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoreProductForCurrentAppAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoreProductsAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productkinds: ::windows::core::RawPtr, storeids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoreProductsAsync(
                &*(&productkinds as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&storeids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAssociatedStoreProductsAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productkinds: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAssociatedStoreProductsAsync(&*(&productkinds as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAssociatedStoreProductsWithPagingAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productkinds: ::windows::core::RawPtr, maxitemstoretrieveperpage: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAssociatedStoreProductsWithPagingAsync(&*(&productkinds as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), maxitemstoretrieveperpage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserCollectionAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productkinds: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserCollectionAsync(&*(&productkinds as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserCollectionWithPagingAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productkinds: ::windows::core::RawPtr, maxitemstoretrieveperpage: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserCollectionWithPagingAsync(&*(&productkinds as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), maxitemstoretrieveperpage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportConsumableFulfillmentAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productstoreid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, quantity: u32, trackingid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportConsumableFulfillmentAsync(&*(&productstoreid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), quantity, &*(&trackingid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConsumableBalanceRemainingAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productstoreid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConsumableBalanceRemainingAsync(&*(&productstoreid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireStoreLicenseForOptionalPackageAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionalpackage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireStoreLicenseForOptionalPackageAsync(&*(&optionalpackage as *const <super::super::ApplicationModel::Package as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Package as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPurchaseAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPurchaseAsync(&*(&storeid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPurchaseWithPurchasePropertiesAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, storepurchaseproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPurchaseWithPurchasePropertiesAsync(&*(&storeid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&storepurchaseproperties as *const <StorePurchaseProperties as ::windows::core::Abi>::Abi as *const <StorePurchaseProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppAndOptionalStorePackageUpdatesAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppAndOptionalStorePackageUpdatesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestDownloadStorePackageUpdatesAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storepackageupdates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestDownloadStorePackageUpdatesAsync(&*(&storepackageupdates as *const <super::super::Foundation::Collections::IIterable<StorePackageUpdate> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<StorePackageUpdate> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestDownloadAndInstallStorePackageUpdatesAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storepackageupdates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestDownloadAndInstallStorePackageUpdatesAsync(&*(&storepackageupdates as *const <super::super::Foundation::Collections::IIterable<StorePackageUpdate> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<StorePackageUpdate> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestDownloadAndInstallStorePackagesAsync<Impl: IStoreContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storeids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestDownloadAndInstallStorePackagesAsync(&*(&storeids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStoreContext>,
            ::windows::core::GetTrustLevel,
            User::<Impl, IMPL_OFFSET>,
            OfflineLicensesChanged::<Impl, IMPL_OFFSET>,
            RemoveOfflineLicensesChanged::<Impl, IMPL_OFFSET>,
            GetCustomerPurchaseIdAsync::<Impl, IMPL_OFFSET>,
            GetCustomerCollectionsIdAsync::<Impl, IMPL_OFFSET>,
            GetAppLicenseAsync::<Impl, IMPL_OFFSET>,
            GetStoreProductForCurrentAppAsync::<Impl, IMPL_OFFSET>,
            GetStoreProductsAsync::<Impl, IMPL_OFFSET>,
            GetAssociatedStoreProductsAsync::<Impl, IMPL_OFFSET>,
            GetAssociatedStoreProductsWithPagingAsync::<Impl, IMPL_OFFSET>,
            GetUserCollectionAsync::<Impl, IMPL_OFFSET>,
            GetUserCollectionWithPagingAsync::<Impl, IMPL_OFFSET>,
            ReportConsumableFulfillmentAsync::<Impl, IMPL_OFFSET>,
            GetConsumableBalanceRemainingAsync::<Impl, IMPL_OFFSET>,
            AcquireStoreLicenseForOptionalPackageAsync::<Impl, IMPL_OFFSET>,
            RequestPurchaseAsync::<Impl, IMPL_OFFSET>,
            RequestPurchaseWithPurchasePropertiesAsync::<Impl, IMPL_OFFSET>,
            GetAppAndOptionalStorePackageUpdatesAsync::<Impl, IMPL_OFFSET>,
            RequestDownloadStorePackageUpdatesAsync::<Impl, IMPL_OFFSET>,
            RequestDownloadAndInstallStorePackageUpdatesAsync::<Impl, IMPL_OFFSET>,
            RequestDownloadAndInstallStorePackagesAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStoreContext2Impl: Sized {
    fn FindStoreProductForPackageAsync(&self, productkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, package: &::core::option::Option<super::super::ApplicationModel::Package>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductResult>>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreContext2 {
    const NAME: &'static str = "Windows.Services.Store.IStoreContext2";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStoreContext2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreContext2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreContext2Vtbl {
        unsafe extern "system" fn FindStoreProductForPackageAsync<Impl: IStoreContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productkinds: ::windows::core::RawPtr, package: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindStoreProductForPackageAsync(&*(&productkinds as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), &*(&package as *const <super::super::ApplicationModel::Package as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Package as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreContext2>, ::windows::core::GetTrustLevel, FindStoreProductForPackageAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreContext2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStoreContext3Impl: Sized {
    fn CanSilentlyDownloadStorePackageUpdates(&self) -> ::windows::core::Result<bool>;
    fn TrySilentDownloadStorePackageUpdatesAsync(&self, storepackageupdates: &::core::option::Option<super::super::Foundation::Collections::IIterable<StorePackageUpdate>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>;
    fn TrySilentDownloadAndInstallStorePackageUpdatesAsync(&self, storepackageupdates: &::core::option::Option<super::super::Foundation::Collections::IIterable<StorePackageUpdate>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>;
    fn CanAcquireStoreLicenseForOptionalPackageAsync(&self, optionalpackage: &::core::option::Option<super::super::ApplicationModel::Package>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreCanAcquireLicenseResult>>;
    fn CanAcquireStoreLicenseAsync(&self, productstoreid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreCanAcquireLicenseResult>>;
    fn GetStoreProductsWithOptionsAsync(&self, productkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, storeids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, storeproductoptions: &::core::option::Option<StoreProductOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductQueryResult>>;
    fn GetAssociatedStoreQueueItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>>;
    fn GetStoreQueueItemsAsync(&self, storeids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>>;
    fn RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync(&self, storeids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, storepackageinstalloptions: &::core::option::Option<StorePackageInstallOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>;
    fn DownloadAndInstallStorePackagesAsync(&self, storeids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>;
    fn RequestUninstallStorePackageAsync(&self, package: &::core::option::Option<super::super::ApplicationModel::Package>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>>;
    fn RequestUninstallStorePackageByStoreIdAsync(&self, storeid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>>;
    fn UninstallStorePackageAsync(&self, package: &::core::option::Option<super::super::ApplicationModel::Package>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>>;
    fn UninstallStorePackageByStoreIdAsync(&self, storeid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreUninstallStorePackageResult>>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreContext3 {
    const NAME: &'static str = "Windows.Services.Store.IStoreContext3";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStoreContext3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreContext3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreContext3Vtbl {
        unsafe extern "system" fn CanSilentlyDownloadStorePackageUpdates<Impl: IStoreContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanSilentlyDownloadStorePackageUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySilentDownloadStorePackageUpdatesAsync<Impl: IStoreContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storepackageupdates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySilentDownloadStorePackageUpdatesAsync(&*(&storepackageupdates as *const <super::super::Foundation::Collections::IIterable<StorePackageUpdate> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<StorePackageUpdate> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySilentDownloadAndInstallStorePackageUpdatesAsync<Impl: IStoreContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storepackageupdates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySilentDownloadAndInstallStorePackageUpdatesAsync(&*(&storepackageupdates as *const <super::super::Foundation::Collections::IIterable<StorePackageUpdate> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<StorePackageUpdate> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanAcquireStoreLicenseForOptionalPackageAsync<Impl: IStoreContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionalpackage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanAcquireStoreLicenseForOptionalPackageAsync(&*(&optionalpackage as *const <super::super::ApplicationModel::Package as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Package as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanAcquireStoreLicenseAsync<Impl: IStoreContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productstoreid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanAcquireStoreLicenseAsync(&*(&productstoreid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoreProductsWithOptionsAsync<Impl: IStoreContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productkinds: ::windows::core::RawPtr, storeids: ::windows::core::RawPtr, storeproductoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoreProductsWithOptionsAsync(
                &*(&productkinds as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&storeids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&storeproductoptions as *const <StoreProductOptions as ::windows::core::Abi>::Abi as *const <StoreProductOptions as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAssociatedStoreQueueItemsAsync<Impl: IStoreContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAssociatedStoreQueueItemsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoreQueueItemsAsync<Impl: IStoreContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storeids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoreQueueItemsAsync(&*(&storeids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync<Impl: IStoreContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storeids: ::windows::core::RawPtr, storepackageinstalloptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync(&*(&storeids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), &*(&storepackageinstalloptions as *const <StorePackageInstallOptions as ::windows::core::Abi>::Abi as *const <StorePackageInstallOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadAndInstallStorePackagesAsync<Impl: IStoreContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storeids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadAndInstallStorePackagesAsync(&*(&storeids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestUninstallStorePackageAsync<Impl: IStoreContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestUninstallStorePackageAsync(&*(&package as *const <super::super::ApplicationModel::Package as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Package as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestUninstallStorePackageByStoreIdAsync<Impl: IStoreContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestUninstallStorePackageByStoreIdAsync(&*(&storeid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallStorePackageAsync<Impl: IStoreContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UninstallStorePackageAsync(&*(&package as *const <super::super::ApplicationModel::Package as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Package as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallStorePackageByStoreIdAsync<Impl: IStoreContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UninstallStorePackageByStoreIdAsync(&*(&storeid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStoreContext3>,
            ::windows::core::GetTrustLevel,
            CanSilentlyDownloadStorePackageUpdates::<Impl, IMPL_OFFSET>,
            TrySilentDownloadStorePackageUpdatesAsync::<Impl, IMPL_OFFSET>,
            TrySilentDownloadAndInstallStorePackageUpdatesAsync::<Impl, IMPL_OFFSET>,
            CanAcquireStoreLicenseForOptionalPackageAsync::<Impl, IMPL_OFFSET>,
            CanAcquireStoreLicenseAsync::<Impl, IMPL_OFFSET>,
            GetStoreProductsWithOptionsAsync::<Impl, IMPL_OFFSET>,
            GetAssociatedStoreQueueItemsAsync::<Impl, IMPL_OFFSET>,
            GetStoreQueueItemsAsync::<Impl, IMPL_OFFSET>,
            RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync::<Impl, IMPL_OFFSET>,
            DownloadAndInstallStorePackagesAsync::<Impl, IMPL_OFFSET>,
            RequestUninstallStorePackageAsync::<Impl, IMPL_OFFSET>,
            RequestUninstallStorePackageByStoreIdAsync::<Impl, IMPL_OFFSET>,
            UninstallStorePackageAsync::<Impl, IMPL_OFFSET>,
            UninstallStorePackageByStoreIdAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreContext3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStoreContext4Impl: Sized {
    fn RequestRateAndReviewAppAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreRateAndReviewResult>>;
    fn SetInstallOrderForAssociatedStoreQueueItemsAsync(&self, items: &::core::option::Option<super::super::Foundation::Collections::IIterable<StoreQueueItem>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreContext4 {
    const NAME: &'static str = "Windows.Services.Store.IStoreContext4";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStoreContext4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreContext4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreContext4Vtbl {
        unsafe extern "system" fn RequestRateAndReviewAppAsync<Impl: IStoreContext4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestRateAndReviewAppAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInstallOrderForAssociatedStoreQueueItemsAsync<Impl: IStoreContext4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, items: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInstallOrderForAssociatedStoreQueueItemsAsync(&*(&items as *const <super::super::Foundation::Collections::IIterable<StoreQueueItem> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<StoreQueueItem> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreContext4>, ::windows::core::GetTrustLevel, RequestRateAndReviewAppAsync::<Impl, IMPL_OFFSET>, SetInstallOrderForAssociatedStoreQueueItemsAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreContext4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IStoreContextStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<StoreContext>;
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<StoreContext>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreContextStatics {
    const NAME: &'static str = "Windows.Services.Store.IStoreContextStatics";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IStoreContextStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreContextStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreContextStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IStoreContextStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForUser<Impl: IStoreContextStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreContextStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>, GetForUser::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreContextStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStoreImageImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn ImagePurposeTag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
    fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreImage {
    const NAME: &'static str = "Windows.Services.Store.IStoreImage";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStoreImageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreImageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreImageVtbl {
        unsafe extern "system" fn Uri<Impl: IStoreImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImagePurposeTag<Impl: IStoreImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImagePurposeTag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IStoreImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IStoreImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Caption<Impl: IStoreImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Caption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreImage>, ::windows::core::GetTrustLevel, Uri::<Impl, IMPL_OFFSET>, ImagePurposeTag::<Impl, IMPL_OFFSET>, Width::<Impl, IMPL_OFFSET>, Height::<Impl, IMPL_OFFSET>, Caption::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreImage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStoreLicenseImpl: Sized {
    fn SkuStoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn ExpirationDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InAppOfferToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreLicense {
    const NAME: &'static str = "Windows.Services.Store.IStoreLicense";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStoreLicenseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreLicenseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreLicenseVtbl {
        unsafe extern "system" fn SkuStoreId<Impl: IStoreLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SkuStoreId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsActive<Impl: IStoreLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpirationDate<Impl: IStoreLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedJsonData<Impl: IStoreLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedJsonData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InAppOfferToken<Impl: IStoreLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InAppOfferToken() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreLicense>, ::windows::core::GetTrustLevel, SkuStoreId::<Impl, IMPL_OFFSET>, IsActive::<Impl, IMPL_OFFSET>, ExpirationDate::<Impl, IMPL_OFFSET>, ExtendedJsonData::<Impl, IMPL_OFFSET>, InAppOfferToken::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreLicense as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePackageInstallOptionsImpl: Sized {
    fn AllowForcedAppRestart(&self) -> ::windows::core::Result<bool>;
    fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorePackageInstallOptions {
    const NAME: &'static str = "Windows.Services.Store.IStorePackageInstallOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IStorePackageInstallOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorePackageInstallOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorePackageInstallOptionsVtbl {
        unsafe extern "system" fn AllowForcedAppRestart<Impl: IStorePackageInstallOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowForcedAppRestart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowForcedAppRestart<Impl: IStorePackageInstallOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowForcedAppRestart(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorePackageInstallOptions>, ::windows::core::GetTrustLevel, AllowForcedAppRestart::<Impl, IMPL_OFFSET>, SetAllowForcedAppRestart::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorePackageInstallOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStorePackageLicenseImpl: Sized + IClosableImpl {
    fn LicenseLost(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StorePackageLicense, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLicenseLost(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Package(&self) -> ::windows::core::Result<super::super::ApplicationModel::Package>;
    fn IsValid(&self) -> ::windows::core::Result<bool>;
    fn ReleaseLicense(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorePackageLicense {
    const NAME: &'static str = "Windows.Services.Store.IStorePackageLicense";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
impl IStorePackageLicenseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorePackageLicenseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorePackageLicenseVtbl {
        unsafe extern "system" fn LicenseLost<Impl: IStorePackageLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LicenseLost(&*(&handler as *const <super::super::Foundation::TypedEventHandler<StorePackageLicense, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<StorePackageLicense, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLicenseLost<Impl: IStorePackageLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLicenseLost(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Package<Impl: IStorePackageLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Package() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsValid<Impl: IStorePackageLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsValid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseLicense<Impl: IStorePackageLicenseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseLicense().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorePackageLicense>, ::windows::core::GetTrustLevel, LicenseLost::<Impl, IMPL_OFFSET>, RemoveLicenseLost::<Impl, IMPL_OFFSET>, Package::<Impl, IMPL_OFFSET>, IsValid::<Impl, IMPL_OFFSET>, ReleaseLicense::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorePackageLicense as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "implement_exclusive"))]
pub trait IStorePackageUpdateImpl: Sized {
    fn Package(&self) -> ::windows::core::Result<super::super::ApplicationModel::Package>;
    fn Mandatory(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "ApplicationModel", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorePackageUpdate {
    const NAME: &'static str = "Windows.Services.Store.IStorePackageUpdate";
}
#[cfg(all(feature = "ApplicationModel", feature = "implement_exclusive"))]
impl IStorePackageUpdateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorePackageUpdateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorePackageUpdateVtbl {
        unsafe extern "system" fn Package<Impl: IStorePackageUpdateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Package() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mandatory<Impl: IStorePackageUpdateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mandatory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorePackageUpdate>, ::windows::core::GetTrustLevel, Package::<Impl, IMPL_OFFSET>, Mandatory::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorePackageUpdate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStorePackageUpdateResultImpl: Sized {
    fn OverallState(&self) -> ::windows::core::Result<StorePackageUpdateState>;
    fn StorePackageUpdateStatuses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StorePackageUpdateStatus>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorePackageUpdateResult {
    const NAME: &'static str = "Windows.Services.Store.IStorePackageUpdateResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStorePackageUpdateResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorePackageUpdateResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorePackageUpdateResultVtbl {
        unsafe extern "system" fn OverallState<Impl: IStorePackageUpdateResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StorePackageUpdateState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverallState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StorePackageUpdateStatuses<Impl: IStorePackageUpdateResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StorePackageUpdateStatuses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorePackageUpdateResult>, ::windows::core::GetTrustLevel, OverallState::<Impl, IMPL_OFFSET>, StorePackageUpdateStatuses::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorePackageUpdateResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStorePackageUpdateResult2Impl: Sized {
    fn StoreQueueItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreQueueItem>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorePackageUpdateResult2 {
    const NAME: &'static str = "Windows.Services.Store.IStorePackageUpdateResult2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStorePackageUpdateResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorePackageUpdateResult2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorePackageUpdateResult2Vtbl {
        unsafe extern "system" fn StoreQueueItems<Impl: IStorePackageUpdateResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StoreQueueItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorePackageUpdateResult2>, ::windows::core::GetTrustLevel, StoreQueueItems::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorePackageUpdateResult2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStorePriceImpl: Sized {
    fn FormattedBasePrice(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormattedPrice(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsOnSale(&self) -> ::windows::core::Result<bool>;
    fn SaleEndDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn CurrencyCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormattedRecurrencePrice(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorePrice {
    const NAME: &'static str = "Windows.Services.Store.IStorePrice";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStorePriceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorePriceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorePriceVtbl {
        unsafe extern "system" fn FormattedBasePrice<Impl: IStorePriceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FormattedPrice<Impl: IStorePriceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsOnSale<Impl: IStorePriceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SaleEndDate<Impl: IStorePriceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrencyCode<Impl: IStorePriceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FormattedRecurrencePrice<Impl: IStorePriceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormattedRecurrencePrice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStorePrice>,
            ::windows::core::GetTrustLevel,
            FormattedBasePrice::<Impl, IMPL_OFFSET>,
            FormattedPrice::<Impl, IMPL_OFFSET>,
            IsOnSale::<Impl, IMPL_OFFSET>,
            SaleEndDate::<Impl, IMPL_OFFSET>,
            CurrencyCode::<Impl, IMPL_OFFSET>,
            FormattedRecurrencePrice::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorePrice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStoreProductImpl: Sized {
    fn StoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProductKind(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasDigitalDownload(&self) -> ::windows::core::Result<bool>;
    fn Keywords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Images(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreImage>>;
    fn Videos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreVideo>>;
    fn Skus(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreSku>>;
    fn IsInUserCollection(&self) -> ::windows::core::Result<bool>;
    fn Price(&self) -> ::windows::core::Result<StorePrice>;
    fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LinkUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn GetIsAnySkuInstalledAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestPurchaseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>;
    fn RequestPurchaseWithPurchasePropertiesAsync(&self, storepurchaseproperties: &::core::option::Option<StorePurchaseProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>;
    fn InAppOfferToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreProduct {
    const NAME: &'static str = "Windows.Services.Store.IStoreProduct";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStoreProductVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreProductImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreProductVtbl {
        unsafe extern "system" fn StoreId<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StoreId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProductKind<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasDigitalDownload<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasDigitalDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Keywords<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Images<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Images() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Videos<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Videos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skus<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInUserCollection<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInUserCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Price<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Price() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedJsonData<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedJsonData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LinkUri<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIsAnySkuInstalledAsync<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsAnySkuInstalledAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPurchaseAsync<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPurchaseAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPurchaseWithPurchasePropertiesAsync<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storepurchaseproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPurchaseWithPurchasePropertiesAsync(&*(&storepurchaseproperties as *const <StorePurchaseProperties as ::windows::core::Abi>::Abi as *const <StorePurchaseProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InAppOfferToken<Impl: IStoreProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InAppOfferToken() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStoreProduct>,
            ::windows::core::GetTrustLevel,
            StoreId::<Impl, IMPL_OFFSET>,
            Language::<Impl, IMPL_OFFSET>,
            Title::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            ProductKind::<Impl, IMPL_OFFSET>,
            HasDigitalDownload::<Impl, IMPL_OFFSET>,
            Keywords::<Impl, IMPL_OFFSET>,
            Images::<Impl, IMPL_OFFSET>,
            Videos::<Impl, IMPL_OFFSET>,
            Skus::<Impl, IMPL_OFFSET>,
            IsInUserCollection::<Impl, IMPL_OFFSET>,
            Price::<Impl, IMPL_OFFSET>,
            ExtendedJsonData::<Impl, IMPL_OFFSET>,
            LinkUri::<Impl, IMPL_OFFSET>,
            GetIsAnySkuInstalledAsync::<Impl, IMPL_OFFSET>,
            RequestPurchaseAsync::<Impl, IMPL_OFFSET>,
            RequestPurchaseWithPurchasePropertiesAsync::<Impl, IMPL_OFFSET>,
            InAppOfferToken::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreProduct as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStoreProductOptionsImpl: Sized {
    fn ActionFilters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreProductOptions {
    const NAME: &'static str = "Windows.Services.Store.IStoreProductOptions";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStoreProductOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreProductOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreProductOptionsVtbl {
        unsafe extern "system" fn ActionFilters<Impl: IStoreProductOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActionFilters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreProductOptions>, ::windows::core::GetTrustLevel, ActionFilters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreProductOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStoreProductPagedQueryResultImpl: Sized {
    fn Products(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, StoreProduct>>;
    fn HasMoreResults(&self) -> ::windows::core::Result<bool>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetNextAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreProductPagedQueryResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreProductPagedQueryResult {
    const NAME: &'static str = "Windows.Services.Store.IStoreProductPagedQueryResult";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStoreProductPagedQueryResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreProductPagedQueryResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreProductPagedQueryResultVtbl {
        unsafe extern "system" fn Products<Impl: IStoreProductPagedQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Products() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasMoreResults<Impl: IStoreProductPagedQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasMoreResults() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IStoreProductPagedQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextAsync<Impl: IStoreProductPagedQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreProductPagedQueryResult>, ::windows::core::GetTrustLevel, Products::<Impl, IMPL_OFFSET>, HasMoreResults::<Impl, IMPL_OFFSET>, ExtendedError::<Impl, IMPL_OFFSET>, GetNextAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreProductPagedQueryResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStoreProductQueryResultImpl: Sized {
    fn Products(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, StoreProduct>>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreProductQueryResult {
    const NAME: &'static str = "Windows.Services.Store.IStoreProductQueryResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStoreProductQueryResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreProductQueryResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreProductQueryResultVtbl {
        unsafe extern "system" fn Products<Impl: IStoreProductQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Products() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IStoreProductQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreProductQueryResult>, ::windows::core::GetTrustLevel, Products::<Impl, IMPL_OFFSET>, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreProductQueryResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreProductResultImpl: Sized {
    fn Product(&self) -> ::windows::core::Result<StoreProduct>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreProductResult {
    const NAME: &'static str = "Windows.Services.Store.IStoreProductResult";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreProductResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreProductResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreProductResultVtbl {
        unsafe extern "system" fn Product<Impl: IStoreProductResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Product() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IStoreProductResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreProductResult>, ::windows::core::GetTrustLevel, Product::<Impl, IMPL_OFFSET>, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreProductResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePurchasePropertiesImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExtendedJsonData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorePurchaseProperties {
    const NAME: &'static str = "Windows.Services.Store.IStorePurchaseProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IStorePurchasePropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorePurchasePropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorePurchasePropertiesVtbl {
        unsafe extern "system" fn Name<Impl: IStorePurchasePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetName<Impl: IStorePurchasePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExtendedJsonData<Impl: IStorePurchasePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedJsonData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtendedJsonData<Impl: IStorePurchasePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtendedJsonData(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorePurchaseProperties>, ::windows::core::GetTrustLevel, Name::<Impl, IMPL_OFFSET>, SetName::<Impl, IMPL_OFFSET>, ExtendedJsonData::<Impl, IMPL_OFFSET>, SetExtendedJsonData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorePurchaseProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePurchasePropertiesFactoryImpl: Sized {
    fn Create(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<StorePurchaseProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorePurchasePropertiesFactory {
    const NAME: &'static str = "Windows.Services.Store.IStorePurchasePropertiesFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IStorePurchasePropertiesFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorePurchasePropertiesFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorePurchasePropertiesFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IStorePurchasePropertiesFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorePurchasePropertiesFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorePurchasePropertiesFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePurchaseResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<StorePurchaseStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorePurchaseResult {
    const NAME: &'static str = "Windows.Services.Store.IStorePurchaseResult";
}
#[cfg(feature = "implement_exclusive")]
impl IStorePurchaseResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorePurchaseResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorePurchaseResultVtbl {
        unsafe extern "system" fn Status<Impl: IStorePurchaseResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StorePurchaseStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedError<Impl: IStorePurchaseResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorePurchaseResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorePurchaseResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStoreQueueItemImpl: Sized {
    fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InstallKind(&self) -> ::windows::core::Result<StoreQueueItemKind>;
    fn GetCurrentStatus(&self) -> ::windows::core::Result<StoreQueueItemStatus>;
    fn Completed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StoreQueueItem, StoreQueueItemCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StoreQueueItem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreQueueItem {
    const NAME: &'static str = "Windows.Services.Store.IStoreQueueItem";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStoreQueueItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreQueueItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreQueueItemVtbl {
        unsafe extern "system" fn ProductId<Impl: IStoreQueueItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PackageFamilyName<Impl: IStoreQueueItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallKind<Impl: IStoreQueueItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StoreQueueItemKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentStatus<Impl: IStoreQueueItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Completed<Impl: IStoreQueueItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<StoreQueueItem, StoreQueueItemCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<StoreQueueItem, StoreQueueItemCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Impl: IStoreQueueItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StatusChanged<Impl: IStoreQueueItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<StoreQueueItem, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<StoreQueueItem, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusChanged<Impl: IStoreQueueItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStoreQueueItem>,
            ::windows::core::GetTrustLevel,
            ProductId::<Impl, IMPL_OFFSET>,
            PackageFamilyName::<Impl, IMPL_OFFSET>,
            InstallKind::<Impl, IMPL_OFFSET>,
            GetCurrentStatus::<Impl, IMPL_OFFSET>,
            Completed::<Impl, IMPL_OFFSET>,
            RemoveCompleted::<Impl, IMPL_OFFSET>,
            StatusChanged::<Impl, IMPL_OFFSET>,
            RemoveStatusChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreQueueItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStoreQueueItem2Impl: Sized {
    fn CancelInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn PauseInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ResumeInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreQueueItem2 {
    const NAME: &'static str = "Windows.Services.Store.IStoreQueueItem2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStoreQueueItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreQueueItem2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreQueueItem2Vtbl {
        unsafe extern "system" fn CancelInstallAsync<Impl: IStoreQueueItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelInstallAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PauseInstallAsync<Impl: IStoreQueueItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PauseInstallAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResumeInstallAsync<Impl: IStoreQueueItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResumeInstallAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreQueueItem2>, ::windows::core::GetTrustLevel, CancelInstallAsync::<Impl, IMPL_OFFSET>, PauseInstallAsync::<Impl, IMPL_OFFSET>, ResumeInstallAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreQueueItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreQueueItemCompletedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<StoreQueueItemStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreQueueItemCompletedEventArgs {
    const NAME: &'static str = "Windows.Services.Store.IStoreQueueItemCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreQueueItemCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreQueueItemCompletedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreQueueItemCompletedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IStoreQueueItemCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreQueueItemCompletedEventArgs>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreQueueItemCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreQueueItemStatusImpl: Sized {
    fn PackageInstallState(&self) -> ::windows::core::Result<StoreQueueItemState>;
    fn PackageInstallExtendedState(&self) -> ::windows::core::Result<StoreQueueItemExtendedState>;
    fn UpdateStatus(&self) -> ::windows::core::Result<StorePackageUpdateStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreQueueItemStatus {
    const NAME: &'static str = "Windows.Services.Store.IStoreQueueItemStatus";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreQueueItemStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreQueueItemStatusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreQueueItemStatusVtbl {
        unsafe extern "system" fn PackageInstallState<Impl: IStoreQueueItemStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StoreQueueItemState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageInstallState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PackageInstallExtendedState<Impl: IStoreQueueItemStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StoreQueueItemExtendedState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageInstallExtendedState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateStatus<Impl: IStoreQueueItemStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<StorePackageUpdateStatus>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IStoreQueueItemStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreQueueItemStatus>, ::windows::core::GetTrustLevel, PackageInstallState::<Impl, IMPL_OFFSET>, PackageInstallExtendedState::<Impl, IMPL_OFFSET>, UpdateStatus::<Impl, IMPL_OFFSET>, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreQueueItemStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreRateAndReviewResultImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WasUpdated(&self) -> ::windows::core::Result<bool>;
    fn Status(&self) -> ::windows::core::Result<StoreRateAndReviewStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreRateAndReviewResult {
    const NAME: &'static str = "Windows.Services.Store.IStoreRateAndReviewResult";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreRateAndReviewResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreRateAndReviewResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreRateAndReviewResultVtbl {
        unsafe extern "system" fn ExtendedError<Impl: IStoreRateAndReviewResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedJsonData<Impl: IStoreRateAndReviewResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedJsonData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WasUpdated<Impl: IStoreRateAndReviewResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WasUpdated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IStoreRateAndReviewResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StoreRateAndReviewStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreRateAndReviewResult>, ::windows::core::GetTrustLevel, ExtendedError::<Impl, IMPL_OFFSET>, ExtendedJsonData::<Impl, IMPL_OFFSET>, WasUpdated::<Impl, IMPL_OFFSET>, Status::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreRateAndReviewResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStoreRequestHelperStaticsImpl: Sized {
    fn SendRequestAsync(&self, context: &::core::option::Option<StoreContext>, requestkind: u32, parametersasjson: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StoreSendRequestResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreRequestHelperStatics {
    const NAME: &'static str = "Windows.Services.Store.IStoreRequestHelperStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStoreRequestHelperStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreRequestHelperStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreRequestHelperStaticsVtbl {
        unsafe extern "system" fn SendRequestAsync<Impl: IStoreRequestHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, requestkind: u32, parametersasjson: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendRequestAsync(&*(&context as *const <StoreContext as ::windows::core::Abi>::Abi as *const <StoreContext as ::windows::core::DefaultType>::DefaultType), requestkind, &*(&parametersasjson as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreRequestHelperStatics>, ::windows::core::GetTrustLevel, SendRequestAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreRequestHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreSendRequestResultImpl: Sized {
    fn Response(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreSendRequestResult {
    const NAME: &'static str = "Windows.Services.Store.IStoreSendRequestResult";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreSendRequestResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreSendRequestResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreSendRequestResultVtbl {
        unsafe extern "system" fn Response<Impl: IStoreSendRequestResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Response() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IStoreSendRequestResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreSendRequestResult>, ::windows::core::GetTrustLevel, Response::<Impl, IMPL_OFFSET>, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreSendRequestResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Web_Http", feature = "implement_exclusive"))]
pub trait IStoreSendRequestResult2Impl: Sized {
    fn HttpStatusCode(&self) -> ::windows::core::Result<super::super::Web::Http::HttpStatusCode>;
}
#[cfg(all(feature = "Web_Http", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreSendRequestResult2 {
    const NAME: &'static str = "Windows.Services.Store.IStoreSendRequestResult2";
}
#[cfg(all(feature = "Web_Http", feature = "implement_exclusive"))]
impl IStoreSendRequestResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreSendRequestResult2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreSendRequestResult2Vtbl {
        unsafe extern "system" fn HttpStatusCode<Impl: IStoreSendRequestResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Web::Http::HttpStatusCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HttpStatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreSendRequestResult2>, ::windows::core::GetTrustLevel, HttpStatusCode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreSendRequestResult2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStoreSkuImpl: Sized {
    fn StoreId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsTrial(&self) -> ::windows::core::Result<bool>;
    fn CustomDeveloperData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Images(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreImage>>;
    fn Videos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreVideo>>;
    fn Availabilities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StoreAvailability>>;
    fn Price(&self) -> ::windows::core::Result<StorePrice>;
    fn ExtendedJsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsInUserCollection(&self) -> ::windows::core::Result<bool>;
    fn BundledSkus(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn CollectionData(&self) -> ::windows::core::Result<StoreCollectionData>;
    fn GetIsInstalledAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestPurchaseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>;
    fn RequestPurchaseWithPurchasePropertiesAsync(&self, storepurchaseproperties: &::core::option::Option<StorePurchaseProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StorePurchaseResult>>;
    fn IsSubscription(&self) -> ::windows::core::Result<bool>;
    fn SubscriptionInfo(&self) -> ::windows::core::Result<StoreSubscriptionInfo>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreSku {
    const NAME: &'static str = "Windows.Services.Store.IStoreSku";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStoreSkuVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreSkuImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreSkuVtbl {
        unsafe extern "system" fn StoreId<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StoreId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsTrial<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CustomDeveloperData<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomDeveloperData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Images<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Images() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Videos<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Videos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Availabilities<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Availabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Price<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Price() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedJsonData<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedJsonData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInUserCollection<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInUserCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BundledSkus<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BundledSkus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionData<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectionData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsInstalledAsync<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsInstalledAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPurchaseAsync<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPurchaseAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPurchaseWithPurchasePropertiesAsync<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storepurchaseproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPurchaseWithPurchasePropertiesAsync(&*(&storepurchaseproperties as *const <StorePurchaseProperties as ::windows::core::Abi>::Abi as *const <StorePurchaseProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSubscription<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSubscription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubscriptionInfo<Impl: IStoreSkuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscriptionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStoreSku>,
            ::windows::core::GetTrustLevel,
            StoreId::<Impl, IMPL_OFFSET>,
            Language::<Impl, IMPL_OFFSET>,
            Title::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            IsTrial::<Impl, IMPL_OFFSET>,
            CustomDeveloperData::<Impl, IMPL_OFFSET>,
            Images::<Impl, IMPL_OFFSET>,
            Videos::<Impl, IMPL_OFFSET>,
            Availabilities::<Impl, IMPL_OFFSET>,
            Price::<Impl, IMPL_OFFSET>,
            ExtendedJsonData::<Impl, IMPL_OFFSET>,
            IsInUserCollection::<Impl, IMPL_OFFSET>,
            BundledSkus::<Impl, IMPL_OFFSET>,
            CollectionData::<Impl, IMPL_OFFSET>,
            GetIsInstalledAsync::<Impl, IMPL_OFFSET>,
            RequestPurchaseAsync::<Impl, IMPL_OFFSET>,
            RequestPurchaseWithPurchasePropertiesAsync::<Impl, IMPL_OFFSET>,
            IsSubscription::<Impl, IMPL_OFFSET>,
            SubscriptionInfo::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreSku as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreSubscriptionInfoImpl: Sized {
    fn BillingPeriod(&self) -> ::windows::core::Result<u32>;
    fn BillingPeriodUnit(&self) -> ::windows::core::Result<StoreDurationUnit>;
    fn HasTrialPeriod(&self) -> ::windows::core::Result<bool>;
    fn TrialPeriod(&self) -> ::windows::core::Result<u32>;
    fn TrialPeriodUnit(&self) -> ::windows::core::Result<StoreDurationUnit>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreSubscriptionInfo {
    const NAME: &'static str = "Windows.Services.Store.IStoreSubscriptionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreSubscriptionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreSubscriptionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreSubscriptionInfoVtbl {
        unsafe extern "system" fn BillingPeriod<Impl: IStoreSubscriptionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BillingPeriod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BillingPeriodUnit<Impl: IStoreSubscriptionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StoreDurationUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BillingPeriodUnit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasTrialPeriod<Impl: IStoreSubscriptionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasTrialPeriod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrialPeriod<Impl: IStoreSubscriptionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrialPeriod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrialPeriodUnit<Impl: IStoreSubscriptionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StoreDurationUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrialPeriodUnit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreSubscriptionInfo>, ::windows::core::GetTrustLevel, BillingPeriod::<Impl, IMPL_OFFSET>, BillingPeriodUnit::<Impl, IMPL_OFFSET>, HasTrialPeriod::<Impl, IMPL_OFFSET>, TrialPeriod::<Impl, IMPL_OFFSET>, TrialPeriodUnit::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreSubscriptionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreUninstallStorePackageResultImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Status(&self) -> ::windows::core::Result<StoreUninstallStorePackageStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreUninstallStorePackageResult {
    const NAME: &'static str = "Windows.Services.Store.IStoreUninstallStorePackageResult";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreUninstallStorePackageResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreUninstallStorePackageResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreUninstallStorePackageResultVtbl {
        unsafe extern "system" fn ExtendedError<Impl: IStoreUninstallStorePackageResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IStoreUninstallStorePackageResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StoreUninstallStorePackageStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreUninstallStorePackageResult>, ::windows::core::GetTrustLevel, ExtendedError::<Impl, IMPL_OFFSET>, Status::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreUninstallStorePackageResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStoreVideoImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn VideoPurposeTag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
    fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PreviewImage(&self) -> ::windows::core::Result<StoreImage>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreVideo {
    const NAME: &'static str = "Windows.Services.Store.IStoreVideo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStoreVideoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreVideoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreVideoVtbl {
        unsafe extern "system" fn Uri<Impl: IStoreVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoPurposeTag<Impl: IStoreVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoPurposeTag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IStoreVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IStoreVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Caption<Impl: IStoreVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Caption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreviewImage<Impl: IStoreVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviewImage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreVideo>, ::windows::core::GetTrustLevel, Uri::<Impl, IMPL_OFFSET>, VideoPurposeTag::<Impl, IMPL_OFFSET>, Width::<Impl, IMPL_OFFSET>, Height::<Impl, IMPL_OFFSET>, Caption::<Impl, IMPL_OFFSET>, PreviewImage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreVideo as ::windows::core::Interface>::IID
    }
}
