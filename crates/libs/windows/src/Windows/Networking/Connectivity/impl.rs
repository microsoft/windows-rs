#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IAttributedNetworkUsageImpl: Sized {
    fn BytesSent(&mut self) -> ::windows::core::Result<u64>;
    fn BytesReceived(&mut self) -> ::windows::core::Result<u64>;
    fn AttributionId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AttributionName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AttributionThumbnail(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAttributedNetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.IAttributedNetworkUsage";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IAttributedNetworkUsageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAttributedNetworkUsageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAttributedNetworkUsageVtbl {
        unsafe extern "system" fn BytesSent<Impl: IAttributedNetworkUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesSent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesReceived<Impl: IAttributedNetworkUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesReceived() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttributionId<Impl: IAttributedNetworkUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttributionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttributionName<Impl: IAttributedNetworkUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttributionName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttributionThumbnail<Impl: IAttributedNetworkUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttributionThumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAttributedNetworkUsage, BASE_OFFSET>(),
            BytesSent: BytesSent::<Impl, IMPL_OFFSET>,
            BytesReceived: BytesReceived::<Impl, IMPL_OFFSET>,
            AttributionId: AttributionId::<Impl, IMPL_OFFSET>,
            AttributionName: AttributionName::<Impl, IMPL_OFFSET>,
            AttributionThumbnail: AttributionThumbnail::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAttributedNetworkUsage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICellularApnContextImpl: Sized {
    fn ProviderId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetProviderId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AccessPointName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAccessPointName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUserName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Password(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPassword(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsCompressionEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsCompressionEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AuthenticationType(&mut self) -> ::windows::core::Result<CellularApnAuthenticationType>;
    fn SetAuthenticationType(&mut self, value: CellularApnAuthenticationType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICellularApnContext {
    const NAME: &'static str = "Windows.Networking.Connectivity.ICellularApnContext";
}
#[cfg(feature = "implement_exclusive")]
impl ICellularApnContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICellularApnContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICellularApnContextVtbl {
        unsafe extern "system" fn ProviderId<Impl: ICellularApnContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderId<Impl: ICellularApnContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccessPointName<Impl: ICellularApnContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessPointName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessPointName<Impl: ICellularApnContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessPointName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserName<Impl: ICellularApnContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserName<Impl: ICellularApnContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Password<Impl: ICellularApnContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Password() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPassword<Impl: ICellularApnContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPassword(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsCompressionEnabled<Impl: ICellularApnContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCompressionEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCompressionEnabled<Impl: ICellularApnContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCompressionEnabled(value).into()
        }
        unsafe extern "system" fn AuthenticationType<Impl: ICellularApnContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CellularApnAuthenticationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationType<Impl: ICellularApnContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CellularApnAuthenticationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationType(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICellularApnContext, BASE_OFFSET>(),
            ProviderId: ProviderId::<Impl, IMPL_OFFSET>,
            SetProviderId: SetProviderId::<Impl, IMPL_OFFSET>,
            AccessPointName: AccessPointName::<Impl, IMPL_OFFSET>,
            SetAccessPointName: SetAccessPointName::<Impl, IMPL_OFFSET>,
            UserName: UserName::<Impl, IMPL_OFFSET>,
            SetUserName: SetUserName::<Impl, IMPL_OFFSET>,
            Password: Password::<Impl, IMPL_OFFSET>,
            SetPassword: SetPassword::<Impl, IMPL_OFFSET>,
            IsCompressionEnabled: IsCompressionEnabled::<Impl, IMPL_OFFSET>,
            SetIsCompressionEnabled: SetIsCompressionEnabled::<Impl, IMPL_OFFSET>,
            AuthenticationType: AuthenticationType::<Impl, IMPL_OFFSET>,
            SetAuthenticationType: SetAuthenticationType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICellularApnContext as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICellularApnContext2Impl: Sized {
    fn ProfileName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetProfileName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICellularApnContext2 {
    const NAME: &'static str = "Windows.Networking.Connectivity.ICellularApnContext2";
}
#[cfg(feature = "implement_exclusive")]
impl ICellularApnContext2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICellularApnContext2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICellularApnContext2Vtbl {
        unsafe extern "system" fn ProfileName<Impl: ICellularApnContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProfileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProfileName<Impl: ICellularApnContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProfileName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICellularApnContext2, BASE_OFFSET>(),
            ProfileName: ProfileName::<Impl, IMPL_OFFSET>,
            SetProfileName: SetProfileName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICellularApnContext2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionCostImpl: Sized {
    fn NetworkCostType(&mut self) -> ::windows::core::Result<NetworkCostType>;
    fn Roaming(&mut self) -> ::windows::core::Result<bool>;
    fn OverDataLimit(&mut self) -> ::windows::core::Result<bool>;
    fn ApproachingDataLimit(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectionCost {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionCost";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectionCostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionCostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectionCostVtbl {
        unsafe extern "system" fn NetworkCostType<Impl: IConnectionCostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NetworkCostType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkCostType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Roaming<Impl: IConnectionCostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Roaming() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverDataLimit<Impl: IConnectionCostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverDataLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApproachingDataLimit<Impl: IConnectionCostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApproachingDataLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectionCost, BASE_OFFSET>(),
            NetworkCostType: NetworkCostType::<Impl, IMPL_OFFSET>,
            Roaming: Roaming::<Impl, IMPL_OFFSET>,
            OverDataLimit: OverDataLimit::<Impl, IMPL_OFFSET>,
            ApproachingDataLimit: ApproachingDataLimit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionCost as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionCost2Impl: Sized {
    fn BackgroundDataUsageRestricted(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectionCost2 {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionCost2";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectionCost2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionCost2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectionCost2Vtbl {
        unsafe extern "system" fn BackgroundDataUsageRestricted<Impl: IConnectionCost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundDataUsageRestricted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectionCost2, BASE_OFFSET>(),
            BackgroundDataUsageRestricted: BackgroundDataUsageRestricted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionCost2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IConnectionProfileImpl: Sized {
    fn ProfileName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNetworkConnectivityLevel(&mut self) -> ::windows::core::Result<NetworkConnectivityLevel>;
    fn GetNetworkNames(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn GetConnectionCost(&mut self) -> ::windows::core::Result<ConnectionCost>;
    fn GetDataPlanStatus(&mut self) -> ::windows::core::Result<DataPlanStatus>;
    fn NetworkAdapter(&mut self) -> ::windows::core::Result<NetworkAdapter>;
    fn GetLocalUsage(&mut self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime) -> ::windows::core::Result<DataUsage>;
    fn GetLocalUsagePerRoamingStates(&mut self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime, states: RoamingStates) -> ::windows::core::Result<DataUsage>;
    fn NetworkSecuritySettings(&mut self) -> ::windows::core::Result<NetworkSecuritySettings>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectionProfile {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionProfile";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IConnectionProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectionProfileVtbl {
        unsafe extern "system" fn ProfileName<Impl: IConnectionProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProfileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkConnectivityLevel<Impl: IConnectionProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NetworkConnectivityLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworkConnectivityLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkNames<Impl: IConnectionProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworkNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionCost<Impl: IConnectionProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionCost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataPlanStatus<Impl: IConnectionProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataPlanStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkAdapter<Impl: IConnectionProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAdapter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalUsage<Impl: IConnectionProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalUsage(&*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), &*(&endtime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalUsagePerRoamingStates<Impl: IConnectionProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: RoamingStates, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalUsagePerRoamingStates(&*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), &*(&endtime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), states) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkSecuritySettings<Impl: IConnectionProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkSecuritySettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectionProfile, BASE_OFFSET>(),
            ProfileName: ProfileName::<Impl, IMPL_OFFSET>,
            GetNetworkConnectivityLevel: GetNetworkConnectivityLevel::<Impl, IMPL_OFFSET>,
            GetNetworkNames: GetNetworkNames::<Impl, IMPL_OFFSET>,
            GetConnectionCost: GetConnectionCost::<Impl, IMPL_OFFSET>,
            GetDataPlanStatus: GetDataPlanStatus::<Impl, IMPL_OFFSET>,
            NetworkAdapter: NetworkAdapter::<Impl, IMPL_OFFSET>,
            GetLocalUsage: GetLocalUsage::<Impl, IMPL_OFFSET>,
            GetLocalUsagePerRoamingStates: GetLocalUsagePerRoamingStates::<Impl, IMPL_OFFSET>,
            NetworkSecuritySettings: NetworkSecuritySettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionProfile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IConnectionProfile2Impl: Sized {
    fn IsWwanConnectionProfile(&mut self) -> ::windows::core::Result<bool>;
    fn IsWlanConnectionProfile(&mut self) -> ::windows::core::Result<bool>;
    fn WwanConnectionProfileDetails(&mut self) -> ::windows::core::Result<WwanConnectionProfileDetails>;
    fn WlanConnectionProfileDetails(&mut self) -> ::windows::core::Result<WlanConnectionProfileDetails>;
    fn ServiceProviderGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<::windows::core::GUID>>;
    fn GetSignalBars(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u8>>;
    fn GetDomainConnectivityLevel(&mut self) -> ::windows::core::Result<DomainConnectivityLevel>;
    fn GetNetworkUsageAsync(&mut self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime, granularity: DataUsageGranularity, states: &NetworkUsageStates) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<NetworkUsage>>>;
    fn GetConnectivityIntervalsAsync(&mut self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime, states: &NetworkUsageStates) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectivityInterval>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectionProfile2 {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionProfile2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IConnectionProfile2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionProfile2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectionProfile2Vtbl {
        unsafe extern "system" fn IsWwanConnectionProfile<Impl: IConnectionProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWwanConnectionProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWlanConnectionProfile<Impl: IConnectionProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWlanConnectionProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WwanConnectionProfileDetails<Impl: IConnectionProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WwanConnectionProfileDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WlanConnectionProfileDetails<Impl: IConnectionProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WlanConnectionProfileDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceProviderGuid<Impl: IConnectionProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceProviderGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignalBars<Impl: IConnectionProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignalBars() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomainConnectivityLevel<Impl: IConnectionProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DomainConnectivityLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDomainConnectivityLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkUsageAsync<Impl: IConnectionProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, granularity: DataUsageGranularity, states: NetworkUsageStates, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworkUsageAsync(
                &*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                &*(&endtime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                granularity,
                &*(&states as *const <NetworkUsageStates as ::windows::core::Abi>::Abi as *const <NetworkUsageStates as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectivityIntervalsAsync<Impl: IConnectionProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectivityIntervalsAsync(
                &*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                &*(&endtime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                &*(&states as *const <NetworkUsageStates as ::windows::core::Abi>::Abi as *const <NetworkUsageStates as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectionProfile2, BASE_OFFSET>(),
            IsWwanConnectionProfile: IsWwanConnectionProfile::<Impl, IMPL_OFFSET>,
            IsWlanConnectionProfile: IsWlanConnectionProfile::<Impl, IMPL_OFFSET>,
            WwanConnectionProfileDetails: WwanConnectionProfileDetails::<Impl, IMPL_OFFSET>,
            WlanConnectionProfileDetails: WlanConnectionProfileDetails::<Impl, IMPL_OFFSET>,
            ServiceProviderGuid: ServiceProviderGuid::<Impl, IMPL_OFFSET>,
            GetSignalBars: GetSignalBars::<Impl, IMPL_OFFSET>,
            GetDomainConnectivityLevel: GetDomainConnectivityLevel::<Impl, IMPL_OFFSET>,
            GetNetworkUsageAsync: GetNetworkUsageAsync::<Impl, IMPL_OFFSET>,
            GetConnectivityIntervalsAsync: GetConnectivityIntervalsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionProfile2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IConnectionProfile3Impl: Sized {
    fn GetAttributedNetworkUsageAsync(&mut self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime, states: &NetworkUsageStates) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AttributedNetworkUsage>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectionProfile3 {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionProfile3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IConnectionProfile3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionProfile3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectionProfile3Vtbl {
        unsafe extern "system" fn GetAttributedNetworkUsageAsync<Impl: IConnectionProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributedNetworkUsageAsync(
                &*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                &*(&endtime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                &*(&states as *const <NetworkUsageStates as ::windows::core::Abi>::Abi as *const <NetworkUsageStates as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectionProfile3, BASE_OFFSET>(),
            GetAttributedNetworkUsageAsync: GetAttributedNetworkUsageAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionProfile3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IConnectionProfile4Impl: Sized {
    fn GetProviderNetworkUsageAsync(&mut self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime, states: &NetworkUsageStates) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ProviderNetworkUsage>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectionProfile4 {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionProfile4";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IConnectionProfile4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionProfile4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectionProfile4Vtbl {
        unsafe extern "system" fn GetProviderNetworkUsageAsync<Impl: IConnectionProfile4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderNetworkUsageAsync(
                &*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                &*(&endtime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                &*(&states as *const <NetworkUsageStates as ::windows::core::Abi>::Abi as *const <NetworkUsageStates as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectionProfile4, BASE_OFFSET>(),
            GetProviderNetworkUsageAsync: GetProviderNetworkUsageAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionProfile4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IConnectionProfile5Impl: Sized {
    fn CanDelete(&mut self) -> ::windows::core::Result<bool>;
    fn TryDeleteAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConnectionProfileDeleteStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectionProfile5 {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionProfile5";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IConnectionProfile5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionProfile5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectionProfile5Vtbl {
        unsafe extern "system" fn CanDelete<Impl: IConnectionProfile5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanDelete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDeleteAsync<Impl: IConnectionProfile5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryDeleteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectionProfile5, BASE_OFFSET>(),
            CanDelete: CanDelete::<Impl, IMPL_OFFSET>,
            TryDeleteAsync: TryDeleteAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionProfile5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IConnectionProfileFilterImpl: Sized {
    fn SetIsConnected(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsConnected(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsWwanConnectionProfile(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsWwanConnectionProfile(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsWlanConnectionProfile(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsWlanConnectionProfile(&mut self) -> ::windows::core::Result<bool>;
    fn SetNetworkCostType(&mut self, value: NetworkCostType) -> ::windows::core::Result<()>;
    fn NetworkCostType(&mut self) -> ::windows::core::Result<NetworkCostType>;
    fn SetServiceProviderGuid(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<::windows::core::GUID>>) -> ::windows::core::Result<()>;
    fn ServiceProviderGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<::windows::core::GUID>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectionProfileFilter {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionProfileFilter";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IConnectionProfileFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionProfileFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectionProfileFilterVtbl {
        unsafe extern "system" fn SetIsConnected<Impl: IConnectionProfileFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsConnected(value).into()
        }
        unsafe extern "system" fn IsConnected<Impl: IConnectionProfileFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsWwanConnectionProfile<Impl: IConnectionProfileFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsWwanConnectionProfile(value).into()
        }
        unsafe extern "system" fn IsWwanConnectionProfile<Impl: IConnectionProfileFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWwanConnectionProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsWlanConnectionProfile<Impl: IConnectionProfileFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsWlanConnectionProfile(value).into()
        }
        unsafe extern "system" fn IsWlanConnectionProfile<Impl: IConnectionProfileFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWlanConnectionProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkCostType<Impl: IConnectionProfileFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: NetworkCostType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetworkCostType(value).into()
        }
        unsafe extern "system" fn NetworkCostType<Impl: IConnectionProfileFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NetworkCostType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkCostType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceProviderGuid<Impl: IConnectionProfileFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceProviderGuid(&*(&value as *const <super::super::Foundation::IReference<::windows::core::GUID> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<::windows::core::GUID> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ServiceProviderGuid<Impl: IConnectionProfileFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceProviderGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectionProfileFilter, BASE_OFFSET>(),
            SetIsConnected: SetIsConnected::<Impl, IMPL_OFFSET>,
            IsConnected: IsConnected::<Impl, IMPL_OFFSET>,
            SetIsWwanConnectionProfile: SetIsWwanConnectionProfile::<Impl, IMPL_OFFSET>,
            IsWwanConnectionProfile: IsWwanConnectionProfile::<Impl, IMPL_OFFSET>,
            SetIsWlanConnectionProfile: SetIsWlanConnectionProfile::<Impl, IMPL_OFFSET>,
            IsWlanConnectionProfile: IsWlanConnectionProfile::<Impl, IMPL_OFFSET>,
            SetNetworkCostType: SetNetworkCostType::<Impl, IMPL_OFFSET>,
            NetworkCostType: NetworkCostType::<Impl, IMPL_OFFSET>,
            SetServiceProviderGuid: SetServiceProviderGuid::<Impl, IMPL_OFFSET>,
            ServiceProviderGuid: ServiceProviderGuid::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionProfileFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IConnectionProfileFilter2Impl: Sized {
    fn SetIsRoaming(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn IsRoaming(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
    fn SetIsOverDataLimit(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn IsOverDataLimit(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
    fn SetIsBackgroundDataUsageRestricted(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn IsBackgroundDataUsageRestricted(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
    fn RawData(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectionProfileFilter2 {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionProfileFilter2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IConnectionProfileFilter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionProfileFilter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectionProfileFilter2Vtbl {
        unsafe extern "system" fn SetIsRoaming<Impl: IConnectionProfileFilter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRoaming(&*(&value as *const <super::super::Foundation::IReference<bool> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<bool> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsRoaming<Impl: IConnectionProfileFilter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRoaming() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOverDataLimit<Impl: IConnectionProfileFilter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsOverDataLimit(&*(&value as *const <super::super::Foundation::IReference<bool> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<bool> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsOverDataLimit<Impl: IConnectionProfileFilter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOverDataLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsBackgroundDataUsageRestricted<Impl: IConnectionProfileFilter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsBackgroundDataUsageRestricted(&*(&value as *const <super::super::Foundation::IReference<bool> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<bool> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsBackgroundDataUsageRestricted<Impl: IConnectionProfileFilter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBackgroundDataUsageRestricted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawData<Impl: IConnectionProfileFilter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectionProfileFilter2, BASE_OFFSET>(),
            SetIsRoaming: SetIsRoaming::<Impl, IMPL_OFFSET>,
            IsRoaming: IsRoaming::<Impl, IMPL_OFFSET>,
            SetIsOverDataLimit: SetIsOverDataLimit::<Impl, IMPL_OFFSET>,
            IsOverDataLimit: IsOverDataLimit::<Impl, IMPL_OFFSET>,
            SetIsBackgroundDataUsageRestricted: SetIsBackgroundDataUsageRestricted::<Impl, IMPL_OFFSET>,
            IsBackgroundDataUsageRestricted: IsBackgroundDataUsageRestricted::<Impl, IMPL_OFFSET>,
            RawData: RawData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionProfileFilter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IConnectionProfileFilter3Impl: Sized {
    fn SetPurposeGuid(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<::windows::core::GUID>>) -> ::windows::core::Result<()>;
    fn PurposeGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<::windows::core::GUID>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectionProfileFilter3 {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionProfileFilter3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IConnectionProfileFilter3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionProfileFilter3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectionProfileFilter3Vtbl {
        unsafe extern "system" fn SetPurposeGuid<Impl: IConnectionProfileFilter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPurposeGuid(&*(&value as *const <super::super::Foundation::IReference<::windows::core::GUID> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<::windows::core::GUID> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PurposeGuid<Impl: IConnectionProfileFilter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PurposeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectionProfileFilter3, BASE_OFFSET>(),
            SetPurposeGuid: SetPurposeGuid::<Impl, IMPL_OFFSET>,
            PurposeGuid: PurposeGuid::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionProfileFilter3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IConnectionSessionImpl: Sized + IClosableImpl {
    fn ConnectionProfile(&mut self) -> ::windows::core::Result<ConnectionProfile>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectionSession {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionSession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IConnectionSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectionSessionVtbl {
        unsafe extern "system" fn ConnectionProfile<Impl: IConnectionSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectionSession, BASE_OFFSET>(),
            ConnectionProfile: ConnectionProfile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IConnectivityIntervalImpl: Sized {
    fn StartTime(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn ConnectionDuration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectivityInterval {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectivityInterval";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IConnectivityIntervalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectivityIntervalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectivityIntervalVtbl {
        unsafe extern "system" fn StartTime<Impl: IConnectivityIntervalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionDuration<Impl: IConnectivityIntervalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectivityInterval, BASE_OFFSET>(),
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            ConnectionDuration: ConnectionDuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectivityInterval as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IConnectivityManagerStaticsImpl: Sized {
    fn AcquireConnectionAsync(&mut self, cellularapncontext: &::core::option::Option<CellularApnContext>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConnectionSession>>;
    fn AddHttpRoutePolicy(&mut self, routepolicy: &::core::option::Option<RoutePolicy>) -> ::windows::core::Result<()>;
    fn RemoveHttpRoutePolicy(&mut self, routepolicy: &::core::option::Option<RoutePolicy>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectivityManagerStatics {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectivityManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IConnectivityManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectivityManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectivityManagerStaticsVtbl {
        unsafe extern "system" fn AcquireConnectionAsync<Impl: IConnectivityManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cellularapncontext: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireConnectionAsync(&*(&cellularapncontext as *const <CellularApnContext as ::windows::core::Abi>::Abi as *const <CellularApnContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddHttpRoutePolicy<Impl: IConnectivityManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, routepolicy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddHttpRoutePolicy(&*(&routepolicy as *const <RoutePolicy as ::windows::core::Abi>::Abi as *const <RoutePolicy as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveHttpRoutePolicy<Impl: IConnectivityManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, routepolicy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHttpRoutePolicy(&*(&routepolicy as *const <RoutePolicy as ::windows::core::Abi>::Abi as *const <RoutePolicy as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectivityManagerStatics, BASE_OFFSET>(),
            AcquireConnectionAsync: AcquireConnectionAsync::<Impl, IMPL_OFFSET>,
            AddHttpRoutePolicy: AddHttpRoutePolicy::<Impl, IMPL_OFFSET>,
            RemoveHttpRoutePolicy: RemoveHttpRoutePolicy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectivityManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDataPlanStatusImpl: Sized {
    fn DataPlanUsage(&mut self) -> ::windows::core::Result<DataPlanUsage>;
    fn DataLimitInMegabytes(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn InboundBitsPerSecond(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u64>>;
    fn OutboundBitsPerSecond(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u64>>;
    fn NextBillingCycle(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn MaxTransferSizeInMegabytes(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataPlanStatus {
    const NAME: &'static str = "Windows.Networking.Connectivity.IDataPlanStatus";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDataPlanStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPlanStatusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPlanStatusVtbl {
        unsafe extern "system" fn DataPlanUsage<Impl: IDataPlanStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataPlanUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataLimitInMegabytes<Impl: IDataPlanStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataLimitInMegabytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InboundBitsPerSecond<Impl: IDataPlanStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InboundBitsPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutboundBitsPerSecond<Impl: IDataPlanStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutboundBitsPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextBillingCycle<Impl: IDataPlanStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextBillingCycle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxTransferSizeInMegabytes<Impl: IDataPlanStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTransferSizeInMegabytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPlanStatus, BASE_OFFSET>(),
            DataPlanUsage: DataPlanUsage::<Impl, IMPL_OFFSET>,
            DataLimitInMegabytes: DataLimitInMegabytes::<Impl, IMPL_OFFSET>,
            InboundBitsPerSecond: InboundBitsPerSecond::<Impl, IMPL_OFFSET>,
            OutboundBitsPerSecond: OutboundBitsPerSecond::<Impl, IMPL_OFFSET>,
            NextBillingCycle: NextBillingCycle::<Impl, IMPL_OFFSET>,
            MaxTransferSizeInMegabytes: MaxTransferSizeInMegabytes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPlanStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDataPlanUsageImpl: Sized {
    fn MegabytesUsed(&mut self) -> ::windows::core::Result<u32>;
    fn LastSyncTime(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataPlanUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.IDataPlanUsage";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDataPlanUsageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPlanUsageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataPlanUsageVtbl {
        unsafe extern "system" fn MegabytesUsed<Impl: IDataPlanUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MegabytesUsed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastSyncTime<Impl: IDataPlanUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastSyncTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataPlanUsage, BASE_OFFSET>(),
            MegabytesUsed: MegabytesUsed::<Impl, IMPL_OFFSET>,
            LastSyncTime: LastSyncTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataPlanUsage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IDataUsageImpl: Sized {
    fn BytesSent(&mut self) -> ::windows::core::Result<u64>;
    fn BytesReceived(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.IDataUsage";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IDataUsageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataUsageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataUsageVtbl {
        unsafe extern "system" fn BytesSent<Impl: IDataUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesSent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesReceived<Impl: IDataUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesReceived() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataUsage, BASE_OFFSET>(),
            BytesSent: BytesSent::<Impl, IMPL_OFFSET>,
            BytesReceived: BytesReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataUsage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IIPInformationImpl: Sized {
    fn NetworkAdapter(&mut self) -> ::windows::core::Result<NetworkAdapter>;
    fn PrefixLength(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u8>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IIPInformation {
    const NAME: &'static str = "Windows.Networking.Connectivity.IIPInformation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IIPInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIPInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIPInformationVtbl {
        unsafe extern "system" fn NetworkAdapter<Impl: IIPInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAdapter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrefixLength<Impl: IIPInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrefixLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIPInformation, BASE_OFFSET>(),
            NetworkAdapter: NetworkAdapter::<Impl, IMPL_OFFSET>,
            PrefixLength: PrefixLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIPInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanIdentifierImpl: Sized {
    fn InfrastructureId(&mut self) -> ::windows::core::Result<LanIdentifierData>;
    fn PortId(&mut self) -> ::windows::core::Result<LanIdentifierData>;
    fn NetworkAdapterId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILanIdentifier {
    const NAME: &'static str = "Windows.Networking.Connectivity.ILanIdentifier";
}
#[cfg(feature = "implement_exclusive")]
impl ILanIdentifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILanIdentifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILanIdentifierVtbl {
        unsafe extern "system" fn InfrastructureId<Impl: ILanIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InfrastructureId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PortId<Impl: ILanIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PortId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkAdapterId<Impl: ILanIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAdapterId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILanIdentifier, BASE_OFFSET>(),
            InfrastructureId: InfrastructureId::<Impl, IMPL_OFFSET>,
            PortId: PortId::<Impl, IMPL_OFFSET>,
            NetworkAdapterId: NetworkAdapterId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILanIdentifier as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ILanIdentifierDataImpl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<u32>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u8>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILanIdentifierData {
    const NAME: &'static str = "Windows.Networking.Connectivity.ILanIdentifierData";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILanIdentifierDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILanIdentifierDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILanIdentifierDataVtbl {
        unsafe extern "system" fn Type<Impl: ILanIdentifierDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: ILanIdentifierDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILanIdentifierData, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILanIdentifierData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait INetworkAdapterImpl: Sized {
    fn OutboundMaxBitsPerSecond(&mut self) -> ::windows::core::Result<u64>;
    fn InboundMaxBitsPerSecond(&mut self) -> ::windows::core::Result<u64>;
    fn IanaInterfaceType(&mut self) -> ::windows::core::Result<u32>;
    fn NetworkItem(&mut self) -> ::windows::core::Result<NetworkItem>;
    fn NetworkAdapterId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetConnectedProfileAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConnectionProfile>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkAdapter {
    const NAME: &'static str = "Windows.Networking.Connectivity.INetworkAdapter";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl INetworkAdapterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkAdapterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkAdapterVtbl {
        unsafe extern "system" fn OutboundMaxBitsPerSecond<Impl: INetworkAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutboundMaxBitsPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InboundMaxBitsPerSecond<Impl: INetworkAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InboundMaxBitsPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IanaInterfaceType<Impl: INetworkAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IanaInterfaceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkItem<Impl: INetworkAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkAdapterId<Impl: INetworkAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAdapterId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectedProfileAsync<Impl: INetworkAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectedProfileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkAdapter, BASE_OFFSET>(),
            OutboundMaxBitsPerSecond: OutboundMaxBitsPerSecond::<Impl, IMPL_OFFSET>,
            InboundMaxBitsPerSecond: InboundMaxBitsPerSecond::<Impl, IMPL_OFFSET>,
            IanaInterfaceType: IanaInterfaceType::<Impl, IMPL_OFFSET>,
            NetworkItem: NetworkItem::<Impl, IMPL_OFFSET>,
            NetworkAdapterId: NetworkAdapterId::<Impl, IMPL_OFFSET>,
            GetConnectedProfileAsync: GetConnectedProfileAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkAdapter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait INetworkInformationStaticsImpl: Sized {
    fn GetConnectionProfiles(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ConnectionProfile>>;
    fn GetInternetConnectionProfile(&mut self) -> ::windows::core::Result<ConnectionProfile>;
    fn GetLanIdentifiers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<LanIdentifier>>;
    fn GetHostNames(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>>;
    fn GetProxyConfigurationAsync(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProxyConfiguration>>;
    fn GetSortedEndpointPairs(&mut self, destinationlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::EndpointPair>>, sortoptions: super::HostNameSortOptions) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>;
    fn NetworkStatusChanged(&mut self, networkstatushandler: &::core::option::Option<NetworkStatusChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNetworkStatusChanged(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkInformationStatics {
    const NAME: &'static str = "Windows.Networking.Connectivity.INetworkInformationStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl INetworkInformationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkInformationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkInformationStaticsVtbl {
        unsafe extern "system" fn GetConnectionProfiles<Impl: INetworkInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInternetConnectionProfile<Impl: INetworkInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInternetConnectionProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanIdentifiers<Impl: INetworkInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanIdentifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHostNames<Impl: INetworkInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHostNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProxyConfigurationAsync<Impl: INetworkInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProxyConfigurationAsync(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSortedEndpointPairs<Impl: INetworkInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationlist: ::windows::core::RawPtr, sortoptions: super::HostNameSortOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSortedEndpointPairs(&*(&destinationlist as *const <super::super::Foundation::Collections::IIterable<super::EndpointPair> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::EndpointPair> as ::windows::core::DefaultType>::DefaultType), sortoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkStatusChanged<Impl: INetworkInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkstatushandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkStatusChanged(&*(&networkstatushandler as *const <NetworkStatusChangedEventHandler as ::windows::core::Abi>::Abi as *const <NetworkStatusChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNetworkStatusChanged<Impl: INetworkInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNetworkStatusChanged(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkInformationStatics, BASE_OFFSET>(),
            GetConnectionProfiles: GetConnectionProfiles::<Impl, IMPL_OFFSET>,
            GetInternetConnectionProfile: GetInternetConnectionProfile::<Impl, IMPL_OFFSET>,
            GetLanIdentifiers: GetLanIdentifiers::<Impl, IMPL_OFFSET>,
            GetHostNames: GetHostNames::<Impl, IMPL_OFFSET>,
            GetProxyConfigurationAsync: GetProxyConfigurationAsync::<Impl, IMPL_OFFSET>,
            GetSortedEndpointPairs: GetSortedEndpointPairs::<Impl, IMPL_OFFSET>,
            NetworkStatusChanged: NetworkStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveNetworkStatusChanged: RemoveNetworkStatusChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkInformationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait INetworkInformationStatics2Impl: Sized {
    fn FindConnectionProfilesAsync(&mut self, pprofilefilter: &::core::option::Option<ConnectionProfileFilter>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectionProfile>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkInformationStatics2 {
    const NAME: &'static str = "Windows.Networking.Connectivity.INetworkInformationStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl INetworkInformationStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkInformationStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkInformationStatics2Vtbl {
        unsafe extern "system" fn FindConnectionProfilesAsync<Impl: INetworkInformationStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofilefilter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindConnectionProfilesAsync(&*(&pprofilefilter as *const <ConnectionProfileFilter as ::windows::core::Abi>::Abi as *const <ConnectionProfileFilter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkInformationStatics2, BASE_OFFSET>(),
            FindConnectionProfilesAsync: FindConnectionProfilesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkInformationStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkItemImpl: Sized {
    fn NetworkId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetNetworkTypes(&mut self) -> ::windows::core::Result<NetworkTypes>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkItem {
    const NAME: &'static str = "Windows.Networking.Connectivity.INetworkItem";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkItemVtbl {
        unsafe extern "system" fn NetworkId<Impl: INetworkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkTypes<Impl: INetworkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NetworkTypes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworkTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkItem, BASE_OFFSET>(),
            NetworkId: NetworkId::<Impl, IMPL_OFFSET>,
            GetNetworkTypes: GetNetworkTypes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkSecuritySettingsImpl: Sized {
    fn NetworkAuthenticationType(&mut self) -> ::windows::core::Result<NetworkAuthenticationType>;
    fn NetworkEncryptionType(&mut self) -> ::windows::core::Result<NetworkEncryptionType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkSecuritySettings {
    const NAME: &'static str = "Windows.Networking.Connectivity.INetworkSecuritySettings";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkSecuritySettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkSecuritySettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkSecuritySettingsVtbl {
        unsafe extern "system" fn NetworkAuthenticationType<Impl: INetworkSecuritySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NetworkAuthenticationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAuthenticationType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkEncryptionType<Impl: INetworkSecuritySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NetworkEncryptionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkEncryptionType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkSecuritySettings, BASE_OFFSET>(),
            NetworkAuthenticationType: NetworkAuthenticationType::<Impl, IMPL_OFFSET>,
            NetworkEncryptionType: NetworkEncryptionType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkSecuritySettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkStateChangeEventDetailsImpl: Sized {
    fn HasNewInternetConnectionProfile(&mut self) -> ::windows::core::Result<bool>;
    fn HasNewConnectionCost(&mut self) -> ::windows::core::Result<bool>;
    fn HasNewNetworkConnectivityLevel(&mut self) -> ::windows::core::Result<bool>;
    fn HasNewDomainConnectivityLevel(&mut self) -> ::windows::core::Result<bool>;
    fn HasNewHostNameList(&mut self) -> ::windows::core::Result<bool>;
    fn HasNewWwanRegistrationState(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkStateChangeEventDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.INetworkStateChangeEventDetails";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkStateChangeEventDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkStateChangeEventDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkStateChangeEventDetailsVtbl {
        unsafe extern "system" fn HasNewInternetConnectionProfile<Impl: INetworkStateChangeEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasNewInternetConnectionProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasNewConnectionCost<Impl: INetworkStateChangeEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasNewConnectionCost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasNewNetworkConnectivityLevel<Impl: INetworkStateChangeEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasNewNetworkConnectivityLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasNewDomainConnectivityLevel<Impl: INetworkStateChangeEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasNewDomainConnectivityLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasNewHostNameList<Impl: INetworkStateChangeEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasNewHostNameList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasNewWwanRegistrationState<Impl: INetworkStateChangeEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasNewWwanRegistrationState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkStateChangeEventDetails, BASE_OFFSET>(),
            HasNewInternetConnectionProfile: HasNewInternetConnectionProfile::<Impl, IMPL_OFFSET>,
            HasNewConnectionCost: HasNewConnectionCost::<Impl, IMPL_OFFSET>,
            HasNewNetworkConnectivityLevel: HasNewNetworkConnectivityLevel::<Impl, IMPL_OFFSET>,
            HasNewDomainConnectivityLevel: HasNewDomainConnectivityLevel::<Impl, IMPL_OFFSET>,
            HasNewHostNameList: HasNewHostNameList::<Impl, IMPL_OFFSET>,
            HasNewWwanRegistrationState: HasNewWwanRegistrationState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkStateChangeEventDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkStateChangeEventDetails2Impl: Sized {
    fn HasNewTetheringOperationalState(&mut self) -> ::windows::core::Result<bool>;
    fn HasNewTetheringClientCount(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkStateChangeEventDetails2 {
    const NAME: &'static str = "Windows.Networking.Connectivity.INetworkStateChangeEventDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkStateChangeEventDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkStateChangeEventDetails2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkStateChangeEventDetails2Vtbl {
        unsafe extern "system" fn HasNewTetheringOperationalState<Impl: INetworkStateChangeEventDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasNewTetheringOperationalState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasNewTetheringClientCount<Impl: INetworkStateChangeEventDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasNewTetheringClientCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkStateChangeEventDetails2, BASE_OFFSET>(),
            HasNewTetheringOperationalState: HasNewTetheringOperationalState::<Impl, IMPL_OFFSET>,
            HasNewTetheringClientCount: HasNewTetheringClientCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkStateChangeEventDetails2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait INetworkUsageImpl: Sized {
    fn BytesSent(&mut self) -> ::windows::core::Result<u64>;
    fn BytesReceived(&mut self) -> ::windows::core::Result<u64>;
    fn ConnectionDuration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.INetworkUsage";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl INetworkUsageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkUsageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkUsageVtbl {
        unsafe extern "system" fn BytesSent<Impl: INetworkUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesSent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesReceived<Impl: INetworkUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesReceived() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionDuration<Impl: INetworkUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkUsage, BASE_OFFSET>(),
            BytesSent: BytesSent::<Impl, IMPL_OFFSET>,
            BytesReceived: BytesReceived::<Impl, IMPL_OFFSET>,
            ConnectionDuration: ConnectionDuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkUsage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProviderNetworkUsageImpl: Sized {
    fn BytesSent(&mut self) -> ::windows::core::Result<u64>;
    fn BytesReceived(&mut self) -> ::windows::core::Result<u64>;
    fn ProviderId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProviderNetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.IProviderNetworkUsage";
}
#[cfg(feature = "implement_exclusive")]
impl IProviderNetworkUsageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderNetworkUsageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProviderNetworkUsageVtbl {
        unsafe extern "system" fn BytesSent<Impl: IProviderNetworkUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesSent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesReceived<Impl: IProviderNetworkUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesReceived() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderId<Impl: IProviderNetworkUsageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProviderNetworkUsage, BASE_OFFSET>(),
            BytesSent: BytesSent::<Impl, IMPL_OFFSET>,
            BytesReceived: BytesReceived::<Impl, IMPL_OFFSET>,
            ProviderId: ProviderId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderNetworkUsage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IProxyConfigurationImpl: Sized {
    fn ProxyUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>>;
    fn CanConnectDirectly(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProxyConfiguration {
    const NAME: &'static str = "Windows.Networking.Connectivity.IProxyConfiguration";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IProxyConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProxyConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProxyConfigurationVtbl {
        unsafe extern "system" fn ProxyUris<Impl: IProxyConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyUris() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanConnectDirectly<Impl: IProxyConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanConnectDirectly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProxyConfiguration, BASE_OFFSET>(),
            ProxyUris: ProxyUris::<Impl, IMPL_OFFSET>,
            CanConnectDirectly: CanConnectDirectly::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProxyConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRoutePolicyImpl: Sized {
    fn ConnectionProfile(&mut self) -> ::windows::core::Result<ConnectionProfile>;
    fn HostName(&mut self) -> ::windows::core::Result<super::HostName>;
    fn HostNameType(&mut self) -> ::windows::core::Result<super::DomainNameType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRoutePolicy {
    const NAME: &'static str = "Windows.Networking.Connectivity.IRoutePolicy";
}
#[cfg(feature = "implement_exclusive")]
impl IRoutePolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRoutePolicyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRoutePolicyVtbl {
        unsafe extern "system" fn ConnectionProfile<Impl: IRoutePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HostName<Impl: IRoutePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HostName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HostNameType<Impl: IRoutePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::DomainNameType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HostNameType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRoutePolicy, BASE_OFFSET>(),
            ConnectionProfile: ConnectionProfile::<Impl, IMPL_OFFSET>,
            HostName: HostName::<Impl, IMPL_OFFSET>,
            HostNameType: HostNameType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRoutePolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRoutePolicyFactoryImpl: Sized {
    fn CreateRoutePolicy(&mut self, connectionprofile: &::core::option::Option<ConnectionProfile>, hostname: &::core::option::Option<super::HostName>, r#type: super::DomainNameType) -> ::windows::core::Result<RoutePolicy>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRoutePolicyFactory {
    const NAME: &'static str = "Windows.Networking.Connectivity.IRoutePolicyFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRoutePolicyFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRoutePolicyFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRoutePolicyFactoryVtbl {
        unsafe extern "system" fn CreateRoutePolicy<Impl: IRoutePolicyFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionprofile: ::windows::core::RawPtr, hostname: ::windows::core::RawPtr, r#type: super::DomainNameType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRoutePolicy(&*(&connectionprofile as *const <ConnectionProfile as ::windows::core::Abi>::Abi as *const <ConnectionProfile as ::windows::core::DefaultType>::DefaultType), &*(&hostname as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType), r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRoutePolicyFactory, BASE_OFFSET>(),
            CreateRoutePolicy: CreateRoutePolicy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRoutePolicyFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWlanConnectionProfileDetailsImpl: Sized {
    fn GetConnectedSsid(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWlanConnectionProfileDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.IWlanConnectionProfileDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IWlanConnectionProfileDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWlanConnectionProfileDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWlanConnectionProfileDetailsVtbl {
        unsafe extern "system" fn GetConnectedSsid<Impl: IWlanConnectionProfileDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectedSsid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWlanConnectionProfileDetails, BASE_OFFSET>(),
            GetConnectedSsid: GetConnectedSsid::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWlanConnectionProfileDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWwanConnectionProfileDetailsImpl: Sized {
    fn HomeProviderId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AccessPointName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNetworkRegistrationState(&mut self) -> ::windows::core::Result<WwanNetworkRegistrationState>;
    fn GetCurrentDataClass(&mut self) -> ::windows::core::Result<WwanDataClass>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWwanConnectionProfileDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.IWwanConnectionProfileDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IWwanConnectionProfileDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWwanConnectionProfileDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWwanConnectionProfileDetailsVtbl {
        unsafe extern "system" fn HomeProviderId<Impl: IWwanConnectionProfileDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HomeProviderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessPointName<Impl: IWwanConnectionProfileDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessPointName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkRegistrationState<Impl: IWwanConnectionProfileDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WwanNetworkRegistrationState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworkRegistrationState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentDataClass<Impl: IWwanConnectionProfileDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WwanDataClass) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentDataClass() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWwanConnectionProfileDetails, BASE_OFFSET>(),
            HomeProviderId: HomeProviderId::<Impl, IMPL_OFFSET>,
            AccessPointName: AccessPointName::<Impl, IMPL_OFFSET>,
            GetNetworkRegistrationState: GetNetworkRegistrationState::<Impl, IMPL_OFFSET>,
            GetCurrentDataClass: GetCurrentDataClass::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWwanConnectionProfileDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWwanConnectionProfileDetails2Impl: Sized {
    fn IPKind(&mut self) -> ::windows::core::Result<WwanNetworkIPKind>;
    fn PurposeGuids(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::GUID>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWwanConnectionProfileDetails2 {
    const NAME: &'static str = "Windows.Networking.Connectivity.IWwanConnectionProfileDetails2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWwanConnectionProfileDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWwanConnectionProfileDetails2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWwanConnectionProfileDetails2Vtbl {
        unsafe extern "system" fn IPKind<Impl: IWwanConnectionProfileDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WwanNetworkIPKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IPKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PurposeGuids<Impl: IWwanConnectionProfileDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PurposeGuids() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWwanConnectionProfileDetails2, BASE_OFFSET>(),
            IPKind: IPKind::<Impl, IMPL_OFFSET>,
            PurposeGuids: PurposeGuids::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWwanConnectionProfileDetails2 as ::windows::core::Interface>::IID
    }
}
