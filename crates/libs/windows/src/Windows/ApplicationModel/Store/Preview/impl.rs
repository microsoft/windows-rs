#[cfg(feature = "implement_exclusive")]
pub trait IDeliveryOptimizationSettings_Impl: Sized {
    fn DownloadMode(&mut self) -> ::windows::core::Result<DeliveryOptimizationDownloadMode>;
    fn DownloadModeSource(&mut self) -> ::windows::core::Result<DeliveryOptimizationDownloadModeSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeliveryOptimizationSettings {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IDeliveryOptimizationSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IDeliveryOptimizationSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeliveryOptimizationSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeliveryOptimizationSettings_Vtbl {
        unsafe extern "system" fn DownloadMode<Impl: IDeliveryOptimizationSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeliveryOptimizationDownloadMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DownloadModeSource<Impl: IDeliveryOptimizationSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeliveryOptimizationDownloadModeSource) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeliveryOptimizationSettings, BASE_OFFSET>(),
            DownloadMode: DownloadMode::<Impl, IMPL_OFFSET>,
            DownloadModeSource: DownloadModeSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeliveryOptimizationSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeliveryOptimizationSettingsStatics_Impl: Sized {
    fn GetCurrentSettings(&mut self) -> ::windows::core::Result<DeliveryOptimizationSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeliveryOptimizationSettingsStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IDeliveryOptimizationSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDeliveryOptimizationSettingsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeliveryOptimizationSettingsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeliveryOptimizationSettingsStatics_Vtbl {
        unsafe extern "system" fn GetCurrentSettings<Impl: IDeliveryOptimizationSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeliveryOptimizationSettingsStatics, BASE_OFFSET>(),
            GetCurrentSettings: GetCurrentSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeliveryOptimizationSettingsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStoreConfigurationStatics_Impl: Sized {
    fn SetSystemConfiguration(&mut self, cataloghardwaremanufacturerid: &::windows::core::HSTRING, catalogstorecontentmodifierid: &::windows::core::HSTRING, systemconfigurationexpiration: &super::super::super::Foundation::DateTime, cataloghardwaredescriptor: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetMobileOperatorConfiguration(&mut self, mobileoperatorid: &::windows::core::HSTRING, appdownloadlimitinmegabytes: u32, updatedownloadlimitinmegabytes: u32) -> ::windows::core::Result<()>;
    fn SetStoreWebAccountId(&mut self, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsStoreWebAccountId(&mut self, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn HardwareManufacturerInfo(&mut self) -> ::windows::core::Result<StoreHardwareManufacturerInfo>;
    fn FilterUnsupportedSystemFeaturesAsync(&mut self, systemfeatures: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<StoreSystemFeature>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StoreSystemFeature>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreConfigurationStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStoreConfigurationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreConfigurationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreConfigurationStatics_Vtbl {
        unsafe extern "system" fn SetSystemConfiguration<Impl: IStoreConfigurationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cataloghardwaremanufacturerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogstorecontentmodifierid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, systemconfigurationexpiration: super::super::super::Foundation::DateTime, cataloghardwaredescriptor: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMobileOperatorConfiguration<Impl: IStoreConfigurationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mobileoperatorid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appdownloadlimitinmegabytes: u32, updatedownloadlimitinmegabytes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMobileOperatorConfiguration(&*(&mobileoperatorid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), appdownloadlimitinmegabytes, updatedownloadlimitinmegabytes).into()
        }
        unsafe extern "system" fn SetStoreWebAccountId<Impl: IStoreConfigurationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStoreWebAccountId(&*(&webaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsStoreWebAccountId<Impl: IStoreConfigurationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HardwareManufacturerInfo<Impl: IStoreConfigurationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FilterUnsupportedSystemFeaturesAsync<Impl: IStoreConfigurationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, systemfeatures: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStoreConfigurationStatics, BASE_OFFSET>(),
            SetSystemConfiguration: SetSystemConfiguration::<Impl, IMPL_OFFSET>,
            SetMobileOperatorConfiguration: SetMobileOperatorConfiguration::<Impl, IMPL_OFFSET>,
            SetStoreWebAccountId: SetStoreWebAccountId::<Impl, IMPL_OFFSET>,
            IsStoreWebAccountId: IsStoreWebAccountId::<Impl, IMPL_OFFSET>,
            HardwareManufacturerInfo: HardwareManufacturerInfo::<Impl, IMPL_OFFSET>,
            FilterUnsupportedSystemFeaturesAsync: FilterUnsupportedSystemFeaturesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreConfigurationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStoreConfigurationStatics2_Impl: Sized {
    fn PurchasePromptingPolicy(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetPurchasePromptingPolicy(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreConfigurationStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStoreConfigurationStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreConfigurationStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreConfigurationStatics2_Vtbl {
        unsafe extern "system" fn PurchasePromptingPolicy<Impl: IStoreConfigurationStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPurchasePromptingPolicy<Impl: IStoreConfigurationStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPurchasePromptingPolicy(&*(&value as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStoreConfigurationStatics2, BASE_OFFSET>(),
            PurchasePromptingPolicy: PurchasePromptingPolicy::<Impl, IMPL_OFFSET>,
            SetPurchasePromptingPolicy: SetPurchasePromptingPolicy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreConfigurationStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "System", feature = "implement_exclusive"))]
pub trait IStoreConfigurationStatics3_Impl: Sized {
    fn HasStoreWebAccount(&mut self) -> ::windows::core::Result<bool>;
    fn HasStoreWebAccountForUser(&mut self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<bool>;
    fn GetStoreLogDataAsync(&mut self, options: StoreLogOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamReference>>;
    fn SetStoreWebAccountIdForUser(&mut self, user: &::core::option::Option<super::super::super::System::User>, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsStoreWebAccountIdForUser(&mut self, user: &::core::option::Option<super::super::super::System::User>, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn GetPurchasePromptingPolicyForUser(&mut self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetPurchasePromptingPolicyForUser(&mut self, user: &::core::option::Option<super::super::super::System::User>, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreConfigurationStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics3";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "System", feature = "implement_exclusive"))]
impl IStoreConfigurationStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreConfigurationStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreConfigurationStatics3_Vtbl {
        unsafe extern "system" fn HasStoreWebAccount<Impl: IStoreConfigurationStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasStoreWebAccountForUser<Impl: IStoreConfigurationStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetStoreLogDataAsync<Impl: IStoreConfigurationStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: StoreLogOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStoreWebAccountIdForUser<Impl: IStoreConfigurationStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStoreWebAccountIdForUser(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&webaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsStoreWebAccountIdForUser<Impl: IStoreConfigurationStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPurchasePromptingPolicyForUser<Impl: IStoreConfigurationStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPurchasePromptingPolicyForUser<Impl: IStoreConfigurationStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPurchasePromptingPolicyForUser(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStoreConfigurationStatics3, BASE_OFFSET>(),
            HasStoreWebAccount: HasStoreWebAccount::<Impl, IMPL_OFFSET>,
            HasStoreWebAccountForUser: HasStoreWebAccountForUser::<Impl, IMPL_OFFSET>,
            GetStoreLogDataAsync: GetStoreLogDataAsync::<Impl, IMPL_OFFSET>,
            SetStoreWebAccountIdForUser: SetStoreWebAccountIdForUser::<Impl, IMPL_OFFSET>,
            IsStoreWebAccountIdForUser: IsStoreWebAccountIdForUser::<Impl, IMPL_OFFSET>,
            GetPurchasePromptingPolicyForUser: GetPurchasePromptingPolicyForUser::<Impl, IMPL_OFFSET>,
            SetPurchasePromptingPolicyForUser: SetPurchasePromptingPolicyForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreConfigurationStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IStoreConfigurationStatics4_Impl: Sized {
    fn GetStoreWebAccountId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetStoreWebAccountIdForUser(&mut self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEnterpriseStoreWebAccountId(&mut self, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetEnterpriseStoreWebAccountIdForUser(&mut self, user: &::core::option::Option<super::super::super::System::User>, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetEnterpriseStoreWebAccountId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetEnterpriseStoreWebAccountIdForUser(&mut self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShouldRestrictToEnterpriseStoreOnly(&mut self) -> ::windows::core::Result<bool>;
    fn ShouldRestrictToEnterpriseStoreOnlyForUser(&mut self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreConfigurationStatics4 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics4";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IStoreConfigurationStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreConfigurationStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreConfigurationStatics4_Vtbl {
        unsafe extern "system" fn GetStoreWebAccountId<Impl: IStoreConfigurationStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetStoreWebAccountIdForUser<Impl: IStoreConfigurationStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnterpriseStoreWebAccountId<Impl: IStoreConfigurationStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnterpriseStoreWebAccountId(&*(&webaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetEnterpriseStoreWebAccountIdForUser<Impl: IStoreConfigurationStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnterpriseStoreWebAccountIdForUser(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&webaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetEnterpriseStoreWebAccountId<Impl: IStoreConfigurationStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetEnterpriseStoreWebAccountIdForUser<Impl: IStoreConfigurationStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShouldRestrictToEnterpriseStoreOnly<Impl: IStoreConfigurationStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShouldRestrictToEnterpriseStoreOnlyForUser<Impl: IStoreConfigurationStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStoreConfigurationStatics4, BASE_OFFSET>(),
            GetStoreWebAccountId: GetStoreWebAccountId::<Impl, IMPL_OFFSET>,
            GetStoreWebAccountIdForUser: GetStoreWebAccountIdForUser::<Impl, IMPL_OFFSET>,
            SetEnterpriseStoreWebAccountId: SetEnterpriseStoreWebAccountId::<Impl, IMPL_OFFSET>,
            SetEnterpriseStoreWebAccountIdForUser: SetEnterpriseStoreWebAccountIdForUser::<Impl, IMPL_OFFSET>,
            GetEnterpriseStoreWebAccountId: GetEnterpriseStoreWebAccountId::<Impl, IMPL_OFFSET>,
            GetEnterpriseStoreWebAccountIdForUser: GetEnterpriseStoreWebAccountIdForUser::<Impl, IMPL_OFFSET>,
            ShouldRestrictToEnterpriseStoreOnly: ShouldRestrictToEnterpriseStoreOnly::<Impl, IMPL_OFFSET>,
            ShouldRestrictToEnterpriseStoreOnlyForUser: ShouldRestrictToEnterpriseStoreOnlyForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreConfigurationStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IStoreConfigurationStatics5_Impl: Sized {
    fn IsPinToDesktopSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsPinToTaskbarSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsPinToStartSupported(&mut self) -> ::windows::core::Result<bool>;
    fn PinToDesktop(&mut self, apppackagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PinToDesktopForUser(&mut self, user: &::core::option::Option<super::super::super::System::User>, apppackagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStoreConfigurationStatics5 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStoreConfigurationStatics5";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IStoreConfigurationStatics5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreConfigurationStatics5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreConfigurationStatics5_Vtbl {
        unsafe extern "system" fn IsPinToDesktopSupported<Impl: IStoreConfigurationStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPinToTaskbarSupported<Impl: IStoreConfigurationStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPinToStartSupported<Impl: IStoreConfigurationStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PinToDesktop<Impl: IStoreConfigurationStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PinToDesktop(&*(&apppackagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PinToDesktopForUser<Impl: IStoreConfigurationStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PinToDesktopForUser(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&apppackagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStoreConfigurationStatics5, BASE_OFFSET>(),
            IsPinToDesktopSupported: IsPinToDesktopSupported::<Impl, IMPL_OFFSET>,
            IsPinToTaskbarSupported: IsPinToTaskbarSupported::<Impl, IMPL_OFFSET>,
            IsPinToStartSupported: IsPinToStartSupported::<Impl, IMPL_OFFSET>,
            PinToDesktop: PinToDesktop::<Impl, IMPL_OFFSET>,
            PinToDesktopForUser: PinToDesktopForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreConfigurationStatics5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStoreHardwareManufacturerInfo_Impl: Sized {
    fn HardwareManufacturerId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StoreContentModifierId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ManufacturerName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStoreHardwareManufacturerInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStoreHardwareManufacturerInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IStoreHardwareManufacturerInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStoreHardwareManufacturerInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStoreHardwareManufacturerInfo_Vtbl {
        unsafe extern "system" fn HardwareManufacturerId<Impl: IStoreHardwareManufacturerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StoreContentModifierId<Impl: IStoreHardwareManufacturerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ModelName<Impl: IStoreHardwareManufacturerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ManufacturerName<Impl: IStoreHardwareManufacturerInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStoreHardwareManufacturerInfo, BASE_OFFSET>(),
            HardwareManufacturerId: HardwareManufacturerId::<Impl, IMPL_OFFSET>,
            StoreContentModifierId: StoreContentModifierId::<Impl, IMPL_OFFSET>,
            ModelName: ModelName::<Impl, IMPL_OFFSET>,
            ManufacturerName: ManufacturerName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStoreHardwareManufacturerInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStorePreview_Impl: Sized {
    fn RequestProductPurchaseByProductIdAndSkuIdAsync(&mut self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<StorePreviewPurchaseResults>>;
    fn LoadAddOnProductInfosAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StorePreviewProductInfo>>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorePreview {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStorePreview";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStorePreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorePreview_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorePreview_Vtbl {
        unsafe extern "system" fn RequestProductPurchaseByProductIdAndSkuIdAsync<Impl: IStorePreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LoadAddOnProductInfosAsync<Impl: IStorePreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorePreview, BASE_OFFSET>(),
            RequestProductPurchaseByProductIdAndSkuIdAsync: RequestProductPurchaseByProductIdAndSkuIdAsync::<Impl, IMPL_OFFSET>,
            LoadAddOnProductInfosAsync: LoadAddOnProductInfosAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorePreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStorePreviewProductInfo_Impl: Sized {
    fn ProductId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProductType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SkuInfoList(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<StorePreviewSkuInfo>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorePreviewProductInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStorePreviewProductInfo";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStorePreviewProductInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorePreviewProductInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorePreviewProductInfo_Vtbl {
        unsafe extern "system" fn ProductId<Impl: IStorePreviewProductInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProductType<Impl: IStorePreviewProductInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Title<Impl: IStorePreviewProductInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Description<Impl: IStorePreviewProductInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SkuInfoList<Impl: IStorePreviewProductInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorePreviewProductInfo, BASE_OFFSET>(),
            ProductId: ProductId::<Impl, IMPL_OFFSET>,
            ProductType: ProductType::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SkuInfoList: SkuInfoList::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorePreviewProductInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePreviewPurchaseResults_Impl: Sized {
    fn ProductPurchaseStatus(&mut self) -> ::windows::core::Result<StorePreviewProductPurchaseStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorePreviewPurchaseResults {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStorePreviewPurchaseResults";
}
#[cfg(feature = "implement_exclusive")]
impl IStorePreviewPurchaseResults_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorePreviewPurchaseResults_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorePreviewPurchaseResults_Vtbl {
        unsafe extern "system" fn ProductPurchaseStatus<Impl: IStorePreviewPurchaseResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StorePreviewProductPurchaseStatus) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorePreviewPurchaseResults, BASE_OFFSET>(),
            ProductPurchaseStatus: ProductPurchaseStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorePreviewPurchaseResults as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorePreviewSkuInfo_Impl: Sized {
    fn ProductId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SkuId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SkuType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CustomDeveloperData(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CurrencyCode(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormattedListPrice(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExtendedData(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorePreviewSkuInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IStorePreviewSkuInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IStorePreviewSkuInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorePreviewSkuInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorePreviewSkuInfo_Vtbl {
        unsafe extern "system" fn ProductId<Impl: IStorePreviewSkuInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SkuId<Impl: IStorePreviewSkuInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SkuType<Impl: IStorePreviewSkuInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Title<Impl: IStorePreviewSkuInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Description<Impl: IStorePreviewSkuInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CustomDeveloperData<Impl: IStorePreviewSkuInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrencyCode<Impl: IStorePreviewSkuInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FormattedListPrice<Impl: IStorePreviewSkuInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedData<Impl: IStorePreviewSkuInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorePreviewSkuInfo, BASE_OFFSET>(),
            ProductId: ProductId::<Impl, IMPL_OFFSET>,
            SkuId: SkuId::<Impl, IMPL_OFFSET>,
            SkuType: SkuType::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            CustomDeveloperData: CustomDeveloperData::<Impl, IMPL_OFFSET>,
            CurrencyCode: CurrencyCode::<Impl, IMPL_OFFSET>,
            FormattedListPrice: FormattedListPrice::<Impl, IMPL_OFFSET>,
            ExtendedData: ExtendedData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorePreviewSkuInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml", feature = "implement_exclusive"))]
pub trait IWebAuthenticationCoreManagerHelper_Impl: Sized {
    fn RequestTokenWithUIElementHostingAsync(&mut self, request: &::core::option::Option<super::super::super::Security::Authentication::Web::Core::WebTokenRequest>, uielement: &::core::option::Option<super::super::super::UI::Xaml::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>>;
    fn RequestTokenWithUIElementHostingAndWebAccountAsync(&mut self, request: &::core::option::Option<super::super::super::Security::Authentication::Web::Core::WebTokenRequest>, webaccount: &::core::option::Option<super::super::super::Security::Credentials::WebAccount>, uielement: &::core::option::Option<super::super::super::UI::Xaml::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAuthenticationCoreManagerHelper {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.IWebAuthenticationCoreManagerHelper";
}
#[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml", feature = "implement_exclusive"))]
impl IWebAuthenticationCoreManagerHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationCoreManagerHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAuthenticationCoreManagerHelper_Vtbl {
        unsafe extern "system" fn RequestTokenWithUIElementHostingAsync<Impl: IWebAuthenticationCoreManagerHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, uielement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestTokenWithUIElementHostingAndWebAccountAsync<Impl: IWebAuthenticationCoreManagerHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, webaccount: ::windows::core::RawPtr, uielement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAuthenticationCoreManagerHelper, BASE_OFFSET>(),
            RequestTokenWithUIElementHostingAsync: RequestTokenWithUIElementHostingAsync::<Impl, IMPL_OFFSET>,
            RequestTokenWithUIElementHostingAndWebAccountAsync: RequestTokenWithUIElementHostingAndWebAccountAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAuthenticationCoreManagerHelper as ::windows::core::Interface>::IID
    }
}
