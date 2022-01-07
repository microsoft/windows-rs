#[cfg(feature = "implement_exclusive")]
pub trait IAttributedNetworkUsageImpl: Sized {
    fn BytesSent(&self) -> ::windows::core::Result<u64>;
    fn BytesReceived(&self) -> ::windows::core::Result<u64>;
    fn AttributionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AttributionName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AttributionThumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAttributedNetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.IAttributedNetworkUsage";
}
#[cfg(feature = "implement_exclusive")]
impl IAttributedNetworkUsageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAttributedNetworkUsageImpl, const OFFSET: isize>() -> IAttributedNetworkUsageVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAttributedNetworkUsage>, ::windows::core::GetTrustLevel, BytesSent::<Impl, OFFSET>, BytesReceived::<Impl, OFFSET>, AttributionId::<Impl, OFFSET>, AttributionName::<Impl, OFFSET>, AttributionThumbnail::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICellularApnContextImpl: Sized {
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetProviderId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AccessPointName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAccessPointName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUserName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Password(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPassword(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsCompressionEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsCompressionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn AuthenticationType(&self) -> ::windows::core::Result<CellularApnAuthenticationType>;
    fn SetAuthenticationType(&self, value: CellularApnAuthenticationType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICellularApnContext {
    const NAME: &'static str = "Windows.Networking.Connectivity.ICellularApnContext";
}
#[cfg(feature = "implement_exclusive")]
impl ICellularApnContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICellularApnContextImpl, const OFFSET: isize>() -> ICellularApnContextVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICellularApnContext>,
            ::windows::core::GetTrustLevel,
            ProviderId::<Impl, OFFSET>,
            SetProviderId::<Impl, OFFSET>,
            AccessPointName::<Impl, OFFSET>,
            SetAccessPointName::<Impl, OFFSET>,
            UserName::<Impl, OFFSET>,
            SetUserName::<Impl, OFFSET>,
            Password::<Impl, OFFSET>,
            SetPassword::<Impl, OFFSET>,
            IsCompressionEnabled::<Impl, OFFSET>,
            SetIsCompressionEnabled::<Impl, OFFSET>,
            AuthenticationType::<Impl, OFFSET>,
            SetAuthenticationType::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICellularApnContext2Impl: Sized {
    fn ProfileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetProfileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICellularApnContext2 {
    const NAME: &'static str = "Windows.Networking.Connectivity.ICellularApnContext2";
}
#[cfg(feature = "implement_exclusive")]
impl ICellularApnContext2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICellularApnContext2Impl, const OFFSET: isize>() -> ICellularApnContext2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICellularApnContext2>, ::windows::core::GetTrustLevel, ProfileName::<Impl, OFFSET>, SetProfileName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionCostImpl: Sized {
    fn NetworkCostType(&self) -> ::windows::core::Result<NetworkCostType>;
    fn Roaming(&self) -> ::windows::core::Result<bool>;
    fn OverDataLimit(&self) -> ::windows::core::Result<bool>;
    fn ApproachingDataLimit(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectionCost {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionCost";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectionCostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionCostImpl, const OFFSET: isize>() -> IConnectionCostVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectionCost>, ::windows::core::GetTrustLevel, NetworkCostType::<Impl, OFFSET>, Roaming::<Impl, OFFSET>, OverDataLimit::<Impl, OFFSET>, ApproachingDataLimit::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionCost2Impl: Sized {
    fn BackgroundDataUsageRestricted(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectionCost2 {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionCost2";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectionCost2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionCost2Impl, const OFFSET: isize>() -> IConnectionCost2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectionCost2>, ::windows::core::GetTrustLevel, BackgroundDataUsageRestricted::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionProfileImpl: Sized {
    fn ProfileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNetworkConnectivityLevel(&self) -> ::windows::core::Result<NetworkConnectivityLevel>;
    fn GetNetworkNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn GetConnectionCost(&self) -> ::windows::core::Result<ConnectionCost>;
    fn GetDataPlanStatus(&self) -> ::windows::core::Result<DataPlanStatus>;
    fn NetworkAdapter(&self) -> ::windows::core::Result<NetworkAdapter>;
    fn GetLocalUsage(&self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime) -> ::windows::core::Result<DataUsage>;
    fn GetLocalUsagePerRoamingStates(&self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime, states: RoamingStates) -> ::windows::core::Result<DataUsage>;
    fn NetworkSecuritySettings(&self) -> ::windows::core::Result<NetworkSecuritySettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectionProfile {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionProfile";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectionProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionProfileImpl, const OFFSET: isize>() -> IConnectionProfileVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IConnectionProfile>,
            ::windows::core::GetTrustLevel,
            ProfileName::<Impl, OFFSET>,
            GetNetworkConnectivityLevel::<Impl, OFFSET>,
            GetNetworkNames::<Impl, OFFSET>,
            GetConnectionCost::<Impl, OFFSET>,
            GetDataPlanStatus::<Impl, OFFSET>,
            NetworkAdapter::<Impl, OFFSET>,
            GetLocalUsage::<Impl, OFFSET>,
            GetLocalUsagePerRoamingStates::<Impl, OFFSET>,
            NetworkSecuritySettings::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionProfile2Impl: Sized {
    fn IsWwanConnectionProfile(&self) -> ::windows::core::Result<bool>;
    fn IsWlanConnectionProfile(&self) -> ::windows::core::Result<bool>;
    fn WwanConnectionProfileDetails(&self) -> ::windows::core::Result<WwanConnectionProfileDetails>;
    fn WlanConnectionProfileDetails(&self) -> ::windows::core::Result<WlanConnectionProfileDetails>;
    fn ServiceProviderGuid(&self) -> ::windows::core::Result<super::super::Foundation::IReference<::windows::core::GUID>>;
    fn GetSignalBars(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u8>>;
    fn GetDomainConnectivityLevel(&self) -> ::windows::core::Result<DomainConnectivityLevel>;
    fn GetNetworkUsageAsync(&self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime, granularity: DataUsageGranularity, states: &NetworkUsageStates) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<NetworkUsage>>>;
    fn GetConnectivityIntervalsAsync(&self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime, states: &NetworkUsageStates) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectivityInterval>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectionProfile2 {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionProfile2";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectionProfile2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionProfile2Impl, const OFFSET: isize>() -> IConnectionProfile2Vtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IConnectionProfile2>,
            ::windows::core::GetTrustLevel,
            IsWwanConnectionProfile::<Impl, OFFSET>,
            IsWlanConnectionProfile::<Impl, OFFSET>,
            WwanConnectionProfileDetails::<Impl, OFFSET>,
            WlanConnectionProfileDetails::<Impl, OFFSET>,
            ServiceProviderGuid::<Impl, OFFSET>,
            GetSignalBars::<Impl, OFFSET>,
            GetDomainConnectivityLevel::<Impl, OFFSET>,
            GetNetworkUsageAsync::<Impl, OFFSET>,
            GetConnectivityIntervalsAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionProfile3Impl: Sized {
    fn GetAttributedNetworkUsageAsync(&self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime, states: &NetworkUsageStates) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AttributedNetworkUsage>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectionProfile3 {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionProfile3";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectionProfile3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionProfile3Impl, const OFFSET: isize>() -> IConnectionProfile3Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectionProfile3>, ::windows::core::GetTrustLevel, GetAttributedNetworkUsageAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionProfile4Impl: Sized {
    fn GetProviderNetworkUsageAsync(&self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime, states: &NetworkUsageStates) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ProviderNetworkUsage>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectionProfile4 {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionProfile4";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectionProfile4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionProfile4Impl, const OFFSET: isize>() -> IConnectionProfile4Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectionProfile4>, ::windows::core::GetTrustLevel, GetProviderNetworkUsageAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionProfile5Impl: Sized {
    fn CanDelete(&self) -> ::windows::core::Result<bool>;
    fn TryDeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConnectionProfileDeleteStatus>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectionProfile5 {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionProfile5";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectionProfile5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionProfile5Impl, const OFFSET: isize>() -> IConnectionProfile5Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectionProfile5>, ::windows::core::GetTrustLevel, CanDelete::<Impl, OFFSET>, TryDeleteAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionProfileFilterImpl: Sized {
    fn SetIsConnected(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsConnected(&self) -> ::windows::core::Result<bool>;
    fn SetIsWwanConnectionProfile(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsWwanConnectionProfile(&self) -> ::windows::core::Result<bool>;
    fn SetIsWlanConnectionProfile(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsWlanConnectionProfile(&self) -> ::windows::core::Result<bool>;
    fn SetNetworkCostType(&self, value: NetworkCostType) -> ::windows::core::Result<()>;
    fn NetworkCostType(&self) -> ::windows::core::Result<NetworkCostType>;
    fn SetServiceProviderGuid(&self, value: &::core::option::Option<super::super::Foundation::IReference<::windows::core::GUID>>) -> ::windows::core::Result<()>;
    fn ServiceProviderGuid(&self) -> ::windows::core::Result<super::super::Foundation::IReference<::windows::core::GUID>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectionProfileFilter {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionProfileFilter";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectionProfileFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionProfileFilterImpl, const OFFSET: isize>() -> IConnectionProfileFilterVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IConnectionProfileFilter>,
            ::windows::core::GetTrustLevel,
            SetIsConnected::<Impl, OFFSET>,
            IsConnected::<Impl, OFFSET>,
            SetIsWwanConnectionProfile::<Impl, OFFSET>,
            IsWwanConnectionProfile::<Impl, OFFSET>,
            SetIsWlanConnectionProfile::<Impl, OFFSET>,
            IsWlanConnectionProfile::<Impl, OFFSET>,
            SetNetworkCostType::<Impl, OFFSET>,
            NetworkCostType::<Impl, OFFSET>,
            SetServiceProviderGuid::<Impl, OFFSET>,
            ServiceProviderGuid::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionProfileFilter2Impl: Sized {
    fn SetIsRoaming(&self, value: &::core::option::Option<super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn IsRoaming(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
    fn SetIsOverDataLimit(&self, value: &::core::option::Option<super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn IsOverDataLimit(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
    fn SetIsBackgroundDataUsageRestricted(&self, value: &::core::option::Option<super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn IsBackgroundDataUsageRestricted(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
    fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectionProfileFilter2 {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionProfileFilter2";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectionProfileFilter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionProfileFilter2Impl, const OFFSET: isize>() -> IConnectionProfileFilter2Vtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IConnectionProfileFilter2>,
            ::windows::core::GetTrustLevel,
            SetIsRoaming::<Impl, OFFSET>,
            IsRoaming::<Impl, OFFSET>,
            SetIsOverDataLimit::<Impl, OFFSET>,
            IsOverDataLimit::<Impl, OFFSET>,
            SetIsBackgroundDataUsageRestricted::<Impl, OFFSET>,
            IsBackgroundDataUsageRestricted::<Impl, OFFSET>,
            RawData::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionProfileFilter3Impl: Sized {
    fn SetPurposeGuid(&self, value: &::core::option::Option<super::super::Foundation::IReference<::windows::core::GUID>>) -> ::windows::core::Result<()>;
    fn PurposeGuid(&self) -> ::windows::core::Result<super::super::Foundation::IReference<::windows::core::GUID>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectionProfileFilter3 {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionProfileFilter3";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectionProfileFilter3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionProfileFilter3Impl, const OFFSET: isize>() -> IConnectionProfileFilter3Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectionProfileFilter3>, ::windows::core::GetTrustLevel, SetPurposeGuid::<Impl, OFFSET>, PurposeGuid::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IConnectionSessionImpl: Sized + IClosableImpl {
    fn ConnectionProfile(&self) -> ::windows::core::Result<ConnectionProfile>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConnectionSession {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectionSession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IConnectionSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionSessionImpl, const OFFSET: isize>() -> IConnectionSessionVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectionSession>, ::windows::core::GetTrustLevel, ConnectionProfile::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectivityIntervalImpl: Sized {
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn ConnectionDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectivityInterval {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectivityInterval";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectivityIntervalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectivityIntervalImpl, const OFFSET: isize>() -> IConnectivityIntervalVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectivityInterval>, ::windows::core::GetTrustLevel, StartTime::<Impl, OFFSET>, ConnectionDuration::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectivityManagerStaticsImpl: Sized {
    fn AcquireConnectionAsync(&self, cellularapncontext: &::core::option::Option<CellularApnContext>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConnectionSession>>;
    fn AddHttpRoutePolicy(&self, routepolicy: &::core::option::Option<RoutePolicy>) -> ::windows::core::Result<()>;
    fn RemoveHttpRoutePolicy(&self, routepolicy: &::core::option::Option<RoutePolicy>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectivityManagerStatics {
    const NAME: &'static str = "Windows.Networking.Connectivity.IConnectivityManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectivityManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectivityManagerStaticsImpl, const OFFSET: isize>() -> IConnectivityManagerStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectivityManagerStatics>, ::windows::core::GetTrustLevel, AcquireConnectionAsync::<Impl, OFFSET>, AddHttpRoutePolicy::<Impl, OFFSET>, RemoveHttpRoutePolicy::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPlanStatusImpl: Sized {
    fn DataPlanUsage(&self) -> ::windows::core::Result<DataPlanUsage>;
    fn DataLimitInMegabytes(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn InboundBitsPerSecond(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u64>>;
    fn OutboundBitsPerSecond(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u64>>;
    fn NextBillingCycle(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn MaxTransferSizeInMegabytes(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPlanStatus {
    const NAME: &'static str = "Windows.Networking.Connectivity.IDataPlanStatus";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPlanStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPlanStatusImpl, const OFFSET: isize>() -> IDataPlanStatusVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDataPlanStatus>,
            ::windows::core::GetTrustLevel,
            DataPlanUsage::<Impl, OFFSET>,
            DataLimitInMegabytes::<Impl, OFFSET>,
            InboundBitsPerSecond::<Impl, OFFSET>,
            OutboundBitsPerSecond::<Impl, OFFSET>,
            NextBillingCycle::<Impl, OFFSET>,
            MaxTransferSizeInMegabytes::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPlanUsageImpl: Sized {
    fn MegabytesUsed(&self) -> ::windows::core::Result<u32>;
    fn LastSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataPlanUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.IDataPlanUsage";
}
#[cfg(feature = "implement_exclusive")]
impl IDataPlanUsageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataPlanUsageImpl, const OFFSET: isize>() -> IDataPlanUsageVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataPlanUsage>, ::windows::core::GetTrustLevel, MegabytesUsed::<Impl, OFFSET>, LastSyncTime::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IDataUsageImpl: Sized {
    fn BytesSent(&self) -> ::windows::core::Result<u64>;
    fn BytesReceived(&self) -> ::windows::core::Result<u64>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.IDataUsage";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IDataUsageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataUsageImpl, const OFFSET: isize>() -> IDataUsageVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataUsage>, ::windows::core::GetTrustLevel, BytesSent::<Impl, OFFSET>, BytesReceived::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIPInformationImpl: Sized {
    fn NetworkAdapter(&self) -> ::windows::core::Result<NetworkAdapter>;
    fn PrefixLength(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u8>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIPInformation {
    const NAME: &'static str = "Windows.Networking.Connectivity.IIPInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IIPInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIPInformationImpl, const OFFSET: isize>() -> IIPInformationVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IIPInformation>, ::windows::core::GetTrustLevel, NetworkAdapter::<Impl, OFFSET>, PrefixLength::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanIdentifierImpl: Sized {
    fn InfrastructureId(&self) -> ::windows::core::Result<LanIdentifierData>;
    fn PortId(&self) -> ::windows::core::Result<LanIdentifierData>;
    fn NetworkAdapterId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILanIdentifier {
    const NAME: &'static str = "Windows.Networking.Connectivity.ILanIdentifier";
}
#[cfg(feature = "implement_exclusive")]
impl ILanIdentifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILanIdentifierImpl, const OFFSET: isize>() -> ILanIdentifierVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILanIdentifier>, ::windows::core::GetTrustLevel, InfrastructureId::<Impl, OFFSET>, PortId::<Impl, OFFSET>, NetworkAdapterId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanIdentifierDataImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<u32>;
    fn Value(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u8>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILanIdentifierData {
    const NAME: &'static str = "Windows.Networking.Connectivity.ILanIdentifierData";
}
#[cfg(feature = "implement_exclusive")]
impl ILanIdentifierDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILanIdentifierDataImpl, const OFFSET: isize>() -> ILanIdentifierDataVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILanIdentifierData>, ::windows::core::GetTrustLevel, Type::<Impl, OFFSET>, Value::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkAdapterImpl: Sized {
    fn OutboundMaxBitsPerSecond(&self) -> ::windows::core::Result<u64>;
    fn InboundMaxBitsPerSecond(&self) -> ::windows::core::Result<u64>;
    fn IanaInterfaceType(&self) -> ::windows::core::Result<u32>;
    fn NetworkItem(&self) -> ::windows::core::Result<NetworkItem>;
    fn NetworkAdapterId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetConnectedProfileAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConnectionProfile>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkAdapter {
    const NAME: &'static str = "Windows.Networking.Connectivity.INetworkAdapter";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkAdapterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkAdapterImpl, const OFFSET: isize>() -> INetworkAdapterVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetworkAdapter>, ::windows::core::GetTrustLevel, OutboundMaxBitsPerSecond::<Impl, OFFSET>, InboundMaxBitsPerSecond::<Impl, OFFSET>, IanaInterfaceType::<Impl, OFFSET>, NetworkItem::<Impl, OFFSET>, NetworkAdapterId::<Impl, OFFSET>, GetConnectedProfileAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkInformationStaticsImpl: Sized {
    fn GetConnectionProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ConnectionProfile>>;
    fn GetInternetConnectionProfile(&self) -> ::windows::core::Result<ConnectionProfile>;
    fn GetLanIdentifiers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<LanIdentifier>>;
    fn GetHostNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>>;
    fn GetProxyConfigurationAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProxyConfiguration>>;
    fn GetSortedEndpointPairs(&self, destinationlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::EndpointPair>>, sortoptions: super::HostNameSortOptions) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>;
    fn NetworkStatusChanged(&self, networkstatushandler: &::core::option::Option<NetworkStatusChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNetworkStatusChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkInformationStatics {
    const NAME: &'static str = "Windows.Networking.Connectivity.INetworkInformationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkInformationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkInformationStaticsImpl, const OFFSET: isize>() -> INetworkInformationStaticsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<INetworkInformationStatics>,
            ::windows::core::GetTrustLevel,
            GetConnectionProfiles::<Impl, OFFSET>,
            GetInternetConnectionProfile::<Impl, OFFSET>,
            GetLanIdentifiers::<Impl, OFFSET>,
            GetHostNames::<Impl, OFFSET>,
            GetProxyConfigurationAsync::<Impl, OFFSET>,
            GetSortedEndpointPairs::<Impl, OFFSET>,
            NetworkStatusChanged::<Impl, OFFSET>,
            RemoveNetworkStatusChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkInformationStatics2Impl: Sized {
    fn FindConnectionProfilesAsync(&self, pprofilefilter: &::core::option::Option<ConnectionProfileFilter>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectionProfile>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkInformationStatics2 {
    const NAME: &'static str = "Windows.Networking.Connectivity.INetworkInformationStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkInformationStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkInformationStatics2Impl, const OFFSET: isize>() -> INetworkInformationStatics2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetworkInformationStatics2>, ::windows::core::GetTrustLevel, FindConnectionProfilesAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkItemImpl: Sized {
    fn NetworkId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetNetworkTypes(&self) -> ::windows::core::Result<NetworkTypes>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkItem {
    const NAME: &'static str = "Windows.Networking.Connectivity.INetworkItem";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkItemImpl, const OFFSET: isize>() -> INetworkItemVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetworkItem>, ::windows::core::GetTrustLevel, NetworkId::<Impl, OFFSET>, GetNetworkTypes::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkSecuritySettingsImpl: Sized {
    fn NetworkAuthenticationType(&self) -> ::windows::core::Result<NetworkAuthenticationType>;
    fn NetworkEncryptionType(&self) -> ::windows::core::Result<NetworkEncryptionType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkSecuritySettings {
    const NAME: &'static str = "Windows.Networking.Connectivity.INetworkSecuritySettings";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkSecuritySettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkSecuritySettingsImpl, const OFFSET: isize>() -> INetworkSecuritySettingsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetworkSecuritySettings>, ::windows::core::GetTrustLevel, NetworkAuthenticationType::<Impl, OFFSET>, NetworkEncryptionType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkStateChangeEventDetailsImpl: Sized {
    fn HasNewInternetConnectionProfile(&self) -> ::windows::core::Result<bool>;
    fn HasNewConnectionCost(&self) -> ::windows::core::Result<bool>;
    fn HasNewNetworkConnectivityLevel(&self) -> ::windows::core::Result<bool>;
    fn HasNewDomainConnectivityLevel(&self) -> ::windows::core::Result<bool>;
    fn HasNewHostNameList(&self) -> ::windows::core::Result<bool>;
    fn HasNewWwanRegistrationState(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkStateChangeEventDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.INetworkStateChangeEventDetails";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkStateChangeEventDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkStateChangeEventDetailsImpl, const OFFSET: isize>() -> INetworkStateChangeEventDetailsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<INetworkStateChangeEventDetails>,
            ::windows::core::GetTrustLevel,
            HasNewInternetConnectionProfile::<Impl, OFFSET>,
            HasNewConnectionCost::<Impl, OFFSET>,
            HasNewNetworkConnectivityLevel::<Impl, OFFSET>,
            HasNewDomainConnectivityLevel::<Impl, OFFSET>,
            HasNewHostNameList::<Impl, OFFSET>,
            HasNewWwanRegistrationState::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkStateChangeEventDetails2Impl: Sized {
    fn HasNewTetheringOperationalState(&self) -> ::windows::core::Result<bool>;
    fn HasNewTetheringClientCount(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkStateChangeEventDetails2 {
    const NAME: &'static str = "Windows.Networking.Connectivity.INetworkStateChangeEventDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkStateChangeEventDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkStateChangeEventDetails2Impl, const OFFSET: isize>() -> INetworkStateChangeEventDetails2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetworkStateChangeEventDetails2>, ::windows::core::GetTrustLevel, HasNewTetheringOperationalState::<Impl, OFFSET>, HasNewTetheringClientCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkUsageImpl: Sized {
    fn BytesSent(&self) -> ::windows::core::Result<u64>;
    fn BytesReceived(&self) -> ::windows::core::Result<u64>;
    fn ConnectionDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.INetworkUsage";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkUsageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkUsageImpl, const OFFSET: isize>() -> INetworkUsageVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetworkUsage>, ::windows::core::GetTrustLevel, BytesSent::<Impl, OFFSET>, BytesReceived::<Impl, OFFSET>, ConnectionDuration::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProviderNetworkUsageImpl: Sized {
    fn BytesSent(&self) -> ::windows::core::Result<u64>;
    fn BytesReceived(&self) -> ::windows::core::Result<u64>;
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProviderNetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.IProviderNetworkUsage";
}
#[cfg(feature = "implement_exclusive")]
impl IProviderNetworkUsageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderNetworkUsageImpl, const OFFSET: isize>() -> IProviderNetworkUsageVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProviderNetworkUsage>, ::windows::core::GetTrustLevel, BytesSent::<Impl, OFFSET>, BytesReceived::<Impl, OFFSET>, ProviderId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProxyConfigurationImpl: Sized {
    fn ProxyUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>>;
    fn CanConnectDirectly(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProxyConfiguration {
    const NAME: &'static str = "Windows.Networking.Connectivity.IProxyConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IProxyConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProxyConfigurationImpl, const OFFSET: isize>() -> IProxyConfigurationVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProxyConfiguration>, ::windows::core::GetTrustLevel, ProxyUris::<Impl, OFFSET>, CanConnectDirectly::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRoutePolicyImpl: Sized {
    fn ConnectionProfile(&self) -> ::windows::core::Result<ConnectionProfile>;
    fn HostName(&self) -> ::windows::core::Result<super::HostName>;
    fn HostNameType(&self) -> ::windows::core::Result<super::DomainNameType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRoutePolicy {
    const NAME: &'static str = "Windows.Networking.Connectivity.IRoutePolicy";
}
#[cfg(feature = "implement_exclusive")]
impl IRoutePolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRoutePolicyImpl, const OFFSET: isize>() -> IRoutePolicyVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRoutePolicy>, ::windows::core::GetTrustLevel, ConnectionProfile::<Impl, OFFSET>, HostName::<Impl, OFFSET>, HostNameType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRoutePolicyFactoryImpl: Sized {
    fn CreateRoutePolicy(&self, connectionprofile: &::core::option::Option<ConnectionProfile>, hostname: &::core::option::Option<super::HostName>, r#type: super::DomainNameType) -> ::windows::core::Result<RoutePolicy>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRoutePolicyFactory {
    const NAME: &'static str = "Windows.Networking.Connectivity.IRoutePolicyFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRoutePolicyFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRoutePolicyFactoryImpl, const OFFSET: isize>() -> IRoutePolicyFactoryVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRoutePolicyFactory>, ::windows::core::GetTrustLevel, CreateRoutePolicy::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWlanConnectionProfileDetailsImpl: Sized {
    fn GetConnectedSsid(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWlanConnectionProfileDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.IWlanConnectionProfileDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IWlanConnectionProfileDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWlanConnectionProfileDetailsImpl, const OFFSET: isize>() -> IWlanConnectionProfileDetailsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWlanConnectionProfileDetails>, ::windows::core::GetTrustLevel, GetConnectedSsid::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWwanConnectionProfileDetailsImpl: Sized {
    fn HomeProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AccessPointName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNetworkRegistrationState(&self) -> ::windows::core::Result<WwanNetworkRegistrationState>;
    fn GetCurrentDataClass(&self) -> ::windows::core::Result<WwanDataClass>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWwanConnectionProfileDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.IWwanConnectionProfileDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IWwanConnectionProfileDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWwanConnectionProfileDetailsImpl, const OFFSET: isize>() -> IWwanConnectionProfileDetailsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWwanConnectionProfileDetails>, ::windows::core::GetTrustLevel, HomeProviderId::<Impl, OFFSET>, AccessPointName::<Impl, OFFSET>, GetNetworkRegistrationState::<Impl, OFFSET>, GetCurrentDataClass::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWwanConnectionProfileDetails2Impl: Sized {
    fn IPKind(&self) -> ::windows::core::Result<WwanNetworkIPKind>;
    fn PurposeGuids(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::GUID>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWwanConnectionProfileDetails2 {
    const NAME: &'static str = "Windows.Networking.Connectivity.IWwanConnectionProfileDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl IWwanConnectionProfileDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWwanConnectionProfileDetails2Impl, const OFFSET: isize>() -> IWwanConnectionProfileDetails2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWwanConnectionProfileDetails2>, ::windows::core::GetTrustLevel, IPKind::<Impl, OFFSET>, PurposeGuids::<Impl, OFFSET>)
    }
}
