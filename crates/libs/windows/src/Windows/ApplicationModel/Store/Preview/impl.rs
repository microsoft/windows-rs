#[cfg(feature = "implement_exclusive")]
pub trait IDeliveryOptimizationSettingsImpl: Sized {
    fn DownloadMode(&self) -> ::windows::core::Result<DeliveryOptimizationDownloadMode>;
    fn DownloadModeSource(&self) -> ::windows::core::Result<DeliveryOptimizationDownloadModeSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeliveryOptimizationSettings {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IDeliveryOptimizationSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IDeliveryOptimizationSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeliveryOptimizationSettingsImpl, const OFFSET: isize>() -> IDeliveryOptimizationSettingsVtbl {
        unsafe extern "system" fn DownloadMode<Impl: IDeliveryOptimizationSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeliveryOptimizationDownloadMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadModeSource<Impl: IDeliveryOptimizationSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeliveryOptimizationDownloadModeSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadModeSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeliveryOptimizationSettings>, ::windows::core::GetTrustLevel, DownloadMode::<Impl, OFFSET>, DownloadModeSource::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeliveryOptimizationSettingsStaticsImpl: Sized {
    fn GetCurrentSettings(&self) -> ::windows::core::Result<DeliveryOptimizationSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeliveryOptimizationSettingsStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IDeliveryOptimizationSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDeliveryOptimizationSettingsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeliveryOptimizationSettingsStaticsImpl, const OFFSET: isize>() -> IDeliveryOptimizationSettingsStaticsVtbl {
        unsafe extern "system" fn GetCurrentSettings<Impl: IDeliveryOptimizationSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeliveryOptimizationSettingsStatics>, ::windows::core::GetTrustLevel, GetCurrentSettings::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreConfigurationStaticsImpl: Sized {
    fn SetSystemConfiguration(&self, cataloghardwaremanufacturerid: &::windows::core::HSTRING, catalogstorecontentmodifierid: &::windows::core::HSTRING, systemconfigurationexpiration: &super::super::super::Foundation::DateTime, cataloghardwaredescriptor: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetMobileOperatorConfiguration(&self, mobileoperatorid: &::windows::core::HSTRING, appdownloadlimitinmegabytes: u32, updatedownloadlimitinmegabytes: u32) -> ::windows::core::Result<()>;
    fn SetStoreWebAccountId(&self, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsStoreWebAccountId(&self, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn HardwareManufacturerInfo(&self) -> ::windows::core::Result<StoreHardwareManufacturerInfo>;
    fn FilterUnsupportedSystemFeaturesAsync(&self, systemfeatures: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<StoreSystemFeature>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StoreSystemFeature>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreConfigurationStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreConfigurationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreConfigurationStaticsImpl, const OFFSET: isize>() -> IStoreConfigurationStaticsVtbl {
        unsafe extern "system" fn SetSystemConfiguration<Impl: IStoreConfigurationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cataloghardwaremanufacturerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogstorecontentmodifierid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, systemconfigurationexpiration: super::super::super::Foundation::DateTime, cataloghardwaredescriptor: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetSystemConfiguration(
                    &*(&cataloghardwaremanufacturerid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&catalogstorecontentmodifierid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&systemconfigurationexpiration as *const <super::super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                    &*(&cataloghardwaredescriptor as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetMobileOperatorConfiguration<Impl: IStoreConfigurationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mobileoperatorid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appdownloadlimitinmegabytes: u32, updatedownloadlimitinmegabytes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMobileOperatorConfiguration(&*(&mobileoperatorid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), appdownloadlimitinmegabytes, updatedownloadlimitinmegabytes).into()
        }
        unsafe extern "system" fn SetStoreWebAccountId<Impl: IStoreConfigurationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStoreWebAccountId(&*(&webaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsStoreWebAccountId<Impl: IStoreConfigurationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStoreWebAccountId(&*(&webaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareManufacturerInfo<Impl: IStoreConfigurationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareManufacturerInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FilterUnsupportedSystemFeaturesAsync<Impl: IStoreConfigurationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, systemfeatures: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FilterUnsupportedSystemFeaturesAsync(&*(&systemfeatures as *const <super::super::super::Foundation::Collections::IIterable<StoreSystemFeature> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<StoreSystemFeature> as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IStoreConfigurationStatics>,
            ::windows::core::GetTrustLevel,
            SetSystemConfiguration::<Impl, OFFSET>,
            SetMobileOperatorConfiguration::<Impl, OFFSET>,
            SetStoreWebAccountId::<Impl, OFFSET>,
            IsStoreWebAccountId::<Impl, OFFSET>,
            HardwareManufacturerInfo::<Impl, OFFSET>,
            FilterUnsupportedSystemFeaturesAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreConfigurationStatics2Impl: Sized {
    fn PurchasePromptingPolicy(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetPurchasePromptingPolicy(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreConfigurationStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreConfigurationStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreConfigurationStatics2Impl, const OFFSET: isize>() -> IStoreConfigurationStatics2Vtbl {
        unsafe extern "system" fn PurchasePromptingPolicy<Impl: IStoreConfigurationStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PurchasePromptingPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPurchasePromptingPolicy<Impl: IStoreConfigurationStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPurchasePromptingPolicy(&*(&value as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreConfigurationStatics2>, ::windows::core::GetTrustLevel, PurchasePromptingPolicy::<Impl, OFFSET>, SetPurchasePromptingPolicy::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreConfigurationStatics3Impl: Sized {
    fn HasStoreWebAccount(&self) -> ::windows::core::Result<bool>;
    fn HasStoreWebAccountForUser(&self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<bool>;
    fn GetStoreLogDataAsync(&self, options: StoreLogOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamReference>>;
    fn SetStoreWebAccountIdForUser(&self, user: &::core::option::Option<super::super::super::System::User>, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsStoreWebAccountIdForUser(&self, user: &::core::option::Option<super::super::super::System::User>, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn GetPurchasePromptingPolicyForUser(&self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetPurchasePromptingPolicyForUser(&self, user: &::core::option::Option<super::super::super::System::User>, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreConfigurationStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreConfigurationStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreConfigurationStatics3Impl, const OFFSET: isize>() -> IStoreConfigurationStatics3Vtbl {
        unsafe extern "system" fn HasStoreWebAccount<Impl: IStoreConfigurationStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasStoreWebAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasStoreWebAccountForUser<Impl: IStoreConfigurationStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasStoreWebAccountForUser(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoreLogDataAsync<Impl: IStoreConfigurationStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: StoreLogOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoreLogDataAsync(options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoreWebAccountIdForUser<Impl: IStoreConfigurationStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStoreWebAccountIdForUser(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&webaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsStoreWebAccountIdForUser<Impl: IStoreConfigurationStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStoreWebAccountIdForUser(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&webaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPurchasePromptingPolicyForUser<Impl: IStoreConfigurationStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPurchasePromptingPolicyForUser(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPurchasePromptingPolicyForUser<Impl: IStoreConfigurationStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPurchasePromptingPolicyForUser(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStoreConfigurationStatics3>,
            ::windows::core::GetTrustLevel,
            HasStoreWebAccount::<Impl, OFFSET>,
            HasStoreWebAccountForUser::<Impl, OFFSET>,
            GetStoreLogDataAsync::<Impl, OFFSET>,
            SetStoreWebAccountIdForUser::<Impl, OFFSET>,
            IsStoreWebAccountIdForUser::<Impl, OFFSET>,
            GetPurchasePromptingPolicyForUser::<Impl, OFFSET>,
            SetPurchasePromptingPolicyForUser::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreConfigurationStatics4Impl: Sized {
    fn GetStoreWebAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetStoreWebAccountIdForUser(&self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEnterpriseStoreWebAccountId(&self, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetEnterpriseStoreWebAccountIdForUser(&self, user: &::core::option::Option<super::super::super::System::User>, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetEnterpriseStoreWebAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetEnterpriseStoreWebAccountIdForUser(&self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShouldRestrictToEnterpriseStoreOnly(&self) -> ::windows::core::Result<bool>;
    fn ShouldRestrictToEnterpriseStoreOnlyForUser(&self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreConfigurationStatics4 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreConfigurationStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreConfigurationStatics4Impl, const OFFSET: isize>() -> IStoreConfigurationStatics4Vtbl {
        unsafe extern "system" fn GetStoreWebAccountId<Impl: IStoreConfigurationStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoreWebAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoreWebAccountIdForUser<Impl: IStoreConfigurationStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoreWebAccountIdForUser(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnterpriseStoreWebAccountId<Impl: IStoreConfigurationStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnterpriseStoreWebAccountId(&*(&webaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetEnterpriseStoreWebAccountIdForUser<Impl: IStoreConfigurationStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnterpriseStoreWebAccountIdForUser(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&webaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetEnterpriseStoreWebAccountId<Impl: IStoreConfigurationStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnterpriseStoreWebAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnterpriseStoreWebAccountIdForUser<Impl: IStoreConfigurationStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnterpriseStoreWebAccountIdForUser(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldRestrictToEnterpriseStoreOnly<Impl: IStoreConfigurationStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldRestrictToEnterpriseStoreOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldRestrictToEnterpriseStoreOnlyForUser<Impl: IStoreConfigurationStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldRestrictToEnterpriseStoreOnlyForUser(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IStoreConfigurationStatics4>,
            ::windows::core::GetTrustLevel,
            GetStoreWebAccountId::<Impl, OFFSET>,
            GetStoreWebAccountIdForUser::<Impl, OFFSET>,
            SetEnterpriseStoreWebAccountId::<Impl, OFFSET>,
            SetEnterpriseStoreWebAccountIdForUser::<Impl, OFFSET>,
            GetEnterpriseStoreWebAccountId::<Impl, OFFSET>,
            GetEnterpriseStoreWebAccountIdForUser::<Impl, OFFSET>,
            ShouldRestrictToEnterpriseStoreOnly::<Impl, OFFSET>,
            ShouldRestrictToEnterpriseStoreOnlyForUser::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreConfigurationStatics5Impl: Sized {
    fn IsPinToDesktopSupported(&self) -> ::windows::core::Result<bool>;
    fn IsPinToTaskbarSupported(&self) -> ::windows::core::Result<bool>;
    fn IsPinToStartSupported(&self) -> ::windows::core::Result<bool>;
    fn PinToDesktop(&self, apppackagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PinToDesktopForUser(&self, user: &::core::option::Option<super::super::super::System::User>, apppackagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreConfigurationStatics5 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreConfigurationStatics5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreConfigurationStatics5Impl, const OFFSET: isize>() -> IStoreConfigurationStatics5Vtbl {
        unsafe extern "system" fn IsPinToDesktopSupported<Impl: IStoreConfigurationStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPinToDesktopSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPinToTaskbarSupported<Impl: IStoreConfigurationStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPinToTaskbarSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPinToStartSupported<Impl: IStoreConfigurationStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPinToStartSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinToDesktop<Impl: IStoreConfigurationStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PinToDesktop(&*(&apppackagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PinToDesktopForUser<Impl: IStoreConfigurationStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PinToDesktopForUser(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&apppackagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreConfigurationStatics5>, ::windows::core::GetTrustLevel, IsPinToDesktopSupported::<Impl, OFFSET>, IsPinToTaskbarSupported::<Impl, OFFSET>, IsPinToStartSupported::<Impl, OFFSET>, PinToDesktop::<Impl, OFFSET>, PinToDesktopForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreHardwareManufacturerInfoImpl: Sized {
    fn HardwareManufacturerId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StoreContentModifierId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ManufacturerName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreHardwareManufacturerInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStoreHardwareManufacturerInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreHardwareManufacturerInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreHardwareManufacturerInfoImpl, const OFFSET: isize>() -> IStoreHardwareManufacturerInfoVtbl {
        unsafe extern "system" fn HardwareManufacturerId<Impl: IStoreHardwareManufacturerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareManufacturerId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StoreContentModifierId<Impl: IStoreHardwareManufacturerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StoreContentModifierId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelName<Impl: IStoreHardwareManufacturerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerName<Impl: IStoreHardwareManufacturerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManufacturerName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStoreHardwareManufacturerInfo>, ::windows::core::GetTrustLevel, HardwareManufacturerId::<Impl, OFFSET>, StoreContentModifierId::<Impl, OFFSET>, ModelName::<Impl, OFFSET>, ManufacturerName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePreviewImpl: Sized {
    fn RequestProductPurchaseByProductIdAndSkuIdAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<StorePreviewPurchaseResults>>;
    fn LoadAddOnProductInfosAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StorePreviewProductInfo>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorePreview {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStorePreview";
}
#[cfg(feature = "implement_exclusive")]
impl IStorePreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorePreviewImpl, const OFFSET: isize>() -> IStorePreviewVtbl {
        unsafe extern "system" fn RequestProductPurchaseByProductIdAndSkuIdAsync<Impl: IStorePreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestProductPurchaseByProductIdAndSkuIdAsync(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&skuid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadAddOnProductInfosAsync<Impl: IStorePreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadAddOnProductInfosAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorePreview>, ::windows::core::GetTrustLevel, RequestProductPurchaseByProductIdAndSkuIdAsync::<Impl, OFFSET>, LoadAddOnProductInfosAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePreviewProductInfoImpl: Sized {
    fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProductType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SkuInfoList(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<StorePreviewSkuInfo>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorePreviewProductInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStorePreviewProductInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IStorePreviewProductInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorePreviewProductInfoImpl, const OFFSET: isize>() -> IStorePreviewProductInfoVtbl {
        unsafe extern "system" fn ProductId<Impl: IStorePreviewProductInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProductType<Impl: IStorePreviewProductInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Title<Impl: IStorePreviewProductInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Description<Impl: IStorePreviewProductInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SkuInfoList<Impl: IStorePreviewProductInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SkuInfoList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorePreviewProductInfo>, ::windows::core::GetTrustLevel, ProductId::<Impl, OFFSET>, ProductType::<Impl, OFFSET>, Title::<Impl, OFFSET>, Description::<Impl, OFFSET>, SkuInfoList::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePreviewPurchaseResultsImpl: Sized {
    fn ProductPurchaseStatus(&self) -> ::windows::core::Result<StorePreviewProductPurchaseStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorePreviewPurchaseResults {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStorePreviewPurchaseResults";
}
#[cfg(feature = "implement_exclusive")]
impl IStorePreviewPurchaseResultsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorePreviewPurchaseResultsImpl, const OFFSET: isize>() -> IStorePreviewPurchaseResultsVtbl {
        unsafe extern "system" fn ProductPurchaseStatus<Impl: IStorePreviewPurchaseResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StorePreviewProductPurchaseStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductPurchaseStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorePreviewPurchaseResults>, ::windows::core::GetTrustLevel, ProductPurchaseStatus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePreviewSkuInfoImpl: Sized {
    fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SkuId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SkuType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CustomDeveloperData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CurrencyCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormattedListPrice(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExtendedData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorePreviewSkuInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStorePreviewSkuInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IStorePreviewSkuInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorePreviewSkuInfoImpl, const OFFSET: isize>() -> IStorePreviewSkuInfoVtbl {
        unsafe extern "system" fn ProductId<Impl: IStorePreviewSkuInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SkuId<Impl: IStorePreviewSkuInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SkuId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SkuType<Impl: IStorePreviewSkuInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SkuType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IStorePreviewSkuInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Description<Impl: IStorePreviewSkuInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CustomDeveloperData<Impl: IStorePreviewSkuInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrencyCode<Impl: IStorePreviewSkuInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FormattedListPrice<Impl: IStorePreviewSkuInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormattedListPrice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedData<Impl: IStorePreviewSkuInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedData() {
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
            ::windows::core::GetRuntimeClassName::<IStorePreviewSkuInfo>,
            ::windows::core::GetTrustLevel,
            ProductId::<Impl, OFFSET>,
            SkuId::<Impl, OFFSET>,
            SkuType::<Impl, OFFSET>,
            Title::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            CustomDeveloperData::<Impl, OFFSET>,
            CurrencyCode::<Impl, OFFSET>,
            FormattedListPrice::<Impl, OFFSET>,
            ExtendedData::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAuthenticationCoreManagerHelperImpl: Sized {
    fn RequestTokenWithUIElementHostingAsync(&self, request: &::core::option::Option<super::super::super::Security::Authentication::Web::Core::WebTokenRequest>, uielement: &::core::option::Option<super::super::super::UI::Xaml::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>>;
    fn RequestTokenWithUIElementHostingAndWebAccountAsync(&self, request: &::core::option::Option<super::super::super::Security::Authentication::Web::Core::WebTokenRequest>, webaccount: &::core::option::Option<super::super::super::Security::Credentials::WebAccount>, uielement: &::core::option::Option<super::super::super::UI::Xaml::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAuthenticationCoreManagerHelper {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IWebAuthenticationCoreManagerHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAuthenticationCoreManagerHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationCoreManagerHelperImpl, const OFFSET: isize>() -> IWebAuthenticationCoreManagerHelperVtbl {
        unsafe extern "system" fn RequestTokenWithUIElementHostingAsync<Impl: IWebAuthenticationCoreManagerHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, uielement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestTokenWithUIElementHostingAsync(&*(&request as *const <super::super::super::Security::Authentication::Web::Core::WebTokenRequest as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Authentication::Web::Core::WebTokenRequest as ::windows::core::DefaultType>::DefaultType), &*(&uielement as *const <super::super::super::UI::Xaml::UIElement as ::windows::core::Abi>::Abi as *const <super::super::super::UI::Xaml::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestTokenWithUIElementHostingAndWebAccountAsync<Impl: IWebAuthenticationCoreManagerHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, webaccount: ::windows::core::RawPtr, uielement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestTokenWithUIElementHostingAndWebAccountAsync(
                &*(&request as *const <super::super::super::Security::Authentication::Web::Core::WebTokenRequest as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Authentication::Web::Core::WebTokenRequest as ::windows::core::DefaultType>::DefaultType),
                &*(&webaccount as *const <super::super::super::Security::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType),
                &*(&uielement as *const <super::super::super::UI::Xaml::UIElement as ::windows::core::Abi>::Abi as *const <super::super::super::UI::Xaml::UIElement as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebAuthenticationCoreManagerHelper>, ::windows::core::GetTrustLevel, RequestTokenWithUIElementHostingAsync::<Impl, OFFSET>, RequestTokenWithUIElementHostingAndWebAccountAsync::<Impl, OFFSET>)
    }
}
